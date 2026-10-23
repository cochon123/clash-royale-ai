# RL pipeline throughput

The `rl-speed/integration` work combines three trainer optimizations:

- Incremental causal feature encoding, cached per match and perspective. Only
  appended events are folded; input changes invalidate the cache. Reference
  encoder comparisons require exact tensor equality.
- Worker groups overlap one group's native engine steps with another group's
  inference. The complete wave still finishes before PPO updates its policy.
- Frozen opponent controllers sharing a policy use one batched forward pass.
  Native placement masks are requested concurrently across distinct clients;
  requests sharing a client are serialized.

`--step-groups 2` enables grouped overlap in league and retraining entry points,
including the adaptive/overnight trainers. `--step-groups 1` selects the previous
lockstep scheduler while retaining the encoder and inference optimizations.
Neither setting changes the engine binary or the combat simulation.

## Sampling and reproducibility

Fresh placement supports rank tiles by score, breaking equal scores by ascending
tile ID. This gives a canonical tie order independent of batch padding. Existing
stored candidate order is retained when computing PPO likelihoods. This explicit
tie rule can differ from historical `torch.topk` behavior on exactly tied scores.

Adaptive policies use per-game uniforms, so scheduling does not consume another
game's random stream. Mixed greedy/sampled inference excludes greedy rows from
global RNG draws and trims placement distributions to the sampled subset's
support width. Arbitrarily repartitioning global-RNG sampled calls is not a
reproducibility guarantee.

Different GPU batch shapes can produce small floating-point differences. Also,
completion order can change transition order and PPO minibatch composition, so
multi-update training trajectories need not match the old trainer exactly.
Encoder equality and frozen-policy sampling tests are separate from throughput
measurements; speed does not establish improved playing strength.

Existing source-hash resume checks remain in force. Use a fresh run directory for
the changed trainer rather than bypassing a failed source guard.

## Measurement

The benchmark uses the same native binary, initial weights, deck pool, checkpoint
and seed for each matched configuration. It measures full collection/PPO waves,
including reset and saving, after league construction. Configurations run
sequentially with reversed order on the second seed. A three-wave training run
also measures later, potentially different trajectories; first-wave comparisons
provide a closer workload match.

`act_batch` timing includes encoding, transfers, mask handling and CPU decoding;
it is not pure GPU time. Concurrent per-engine latencies cannot be added to wall
time. Outcome comparisons pair games by policy version and seed, independent of
completion order.

The accompanying HTML report and raw benchmark evidence record the measured
configuration, machine, spread across seeds, outcome agreement and limitations.

See the [22 October 2026 benchmark report](../../reports/rl_pipeline_speed_20261022.html)
and [validation evidence](../../reports/rl_pipeline_speed_20261022/validation.json).
The final measurements use eight workers in an 8 GiB memory-capped transient
service with swap disabled. The supervisor stops its benchmark if host available
memory falls below 6 GiB; each new run requires at least 12 GiB available.
The earlier incomplete 12/16-worker measurements remain supplemental evidence.

## Continuous collection and the engine pool

Two opt-in paths sit on the overnight and league trainers. Defaults are unchanged:
wave barriers stay, and each worker keeps its own `rust_league_rl` process.

| Flag | Default | Meaning |
|---|---|---|
| `--async-collection` | off | A worker resets as soon as its game ends. PPO runs when enough finished games are buffered. |
| `--max-policy-lag` | 2 | Drop a finished game from the update when `version - min(behavior_version) >` this. The match row and reward audit stay. |
| `--async-ready-games` | 0 | Completed non-stale games per PPO update. 0 uses the worker count. |
| `--engine-pool` | `thread` | `thread` is one process per worker. `session` is one `rust_league_rl --sessions N` process and C3 `step_many`. |

`SampledAction.behavior_version` is the learner version that sampled that decision.
PPO still uses the stored log-prob. Evaluation banks still call `play()` with fixed
weights. Resume refuses a change to these flags. `--engine-pool session` needs the
multi-session binary; the PGO `banana-hog-2` engine does not implement it, and no
real-engine session run was made. `rl/engine_pool.py` covers both pools.
`ThreadPoolEngine.call` talks to the client directly so a mask or reset already
running on the pool cannot deadlock. `MultiSessionEngine` turns a contiguous run of
`step` requests into one `step_many` and sends every other op with `session` set.
`serve_c3` / `open_fake_c3` are the in-process host the session tests use.

`fused_act_round()` wraps one scheduling round. Learner and opponent trunks run
before the first host sync. Sampling order stays queue order. Stored log-probs,
values, and tile candidates are copied out of `inference_mode` with `_cpu_fresh`
so PPO sees ordinary fp32 tensors. Host-to-device copies of staged batches use
pinned memory when the policy is on CUDA. Mixed precision is not enabled: rollout
versus PPO log-prob checks are tighter than autocast noise.

On the session host, non-step ops share one pipe with `step_many`, so a mask cannot
overlap a step in that process. The default thread pool still overlaps them across
workers.

### What the 2-worker pilot actually showed

The phase split below is the instrumented trainer from before this change, one
wave, two workers, seed 777, parent `banana-hog-2/models/final.pt`, pool
`engine/data/fast_league_decks.json`, PGO `rust_league_rl`. cProfile and the phase
wrappers make it slower than a clean run. Wall time is `run()`, 7.19 s, of which
play is 6.38 s. 2 games, 848 decisions, 117.9 decisions/s over `run()` (133.0
during play), 16.7 games/min. Engine-busy utilization, the mean fraction of play
wall a worker spent inside a native request, was 0.452.

Play phases, nested, not a partition of wall time: `act_batch` 3.99 s (848 calls,
848 rows), forward 2.27 s, encode 0.39 s, step wait 0.60 s, PPO update 0.66 s.
Engine sums, also not wall time: steps 2.54 s over 426 calls, resets 3.14 s over
2 calls, masks 0.08 s over 259 calls. With two workers and `--step-groups 2` each
group is one game, so every forward is a single row. Encoding is a few percent of
play. The wave tail is real but smaller than inference: the two games took 177 and
249 steps.

Clean runs use `engine/runs/w3-profile/bench_once.py` under
`systemd-run --user --scope -p MemoryMax=6G -p MemorySwapMax=0 -p RuntimeMaxSec=600`,
working directory `engine`, two workers, four waves, seed 777, same parent, pool,
and binary. Baseline is the default flags after the fused forward and thread pool.
Async adds `--async-collection --async-ready-games 2`. The same baseline workload
(3398 decisions, 8 games) measured 157.9, 98.1, and 100.7 decisions/s as other
processes came and went. Compare runs that were started back to back.

The shipped async scheduler keeps step groups, so inference still overlaps the
other worker's engine step, and it does not wait for a sibling game before
resetting a finished worker. Against the baseline started immediately before it:
100.7 decisions/s and 14.23 games/min versus 100.8 decisions/s and 13.49 games/min.
Async collected 3586 decisions in 35.58 s; baseline collected 3398 in 33.73 s.
Engine-busy utilization rose from 0.361 to 0.416. Inference remained most of the
wall (`infer_s` 23.0 s of 33.7 s baseline, 24.9 s of 35.6 s async). An earlier
async loop that forwarded both workers in one lockstep batch, and one that waited
15 ms to gather the sibling, were both slower on decisions/s and are not the
shipped loop.

At two workers this does not raise decisions/s. The barrier idle is smaller than
the single-row forwards. Async still matters for the contract: games do not sit
idle until the slowest sibling ends, decisions keep the behavior version that
sampled them, and a game more than `--max-policy-lag` versions old stays in the
logs but leaves the PPO batch.

### Projection for 8 and 12 workers

These are not measurements. The earlier eight-worker trainer, before this change,
was about 280 decisions/s and 36 games/min. The two-worker pilot did not move
decisions/s, so the projected eight-worker rate stays near that figure until
someone runs eight workers. An optimistic reading of the old ~70% wave-barrier
utilization would be about 400 decisions/s and about 51 games/min if the barrier
were the only idle and it disappeared entirely. This pilot does not support that
multiplier. Twelve workers would add more singleton forwards; expect sublinear
scaling. The engine ceiling of about 60–80 games/min per core is a ceiling. Two
workers here finished about 14–22 games/min, several times below 120–160.

Commands, sequential, from `engine`:

```
systemd-run --user --scope -p MemoryMax=6G -p MemorySwapMax=0 -p RuntimeMaxSec=600 \
  --working-directory=$PWD --setenv=PYTHONDONTWRITEBYTECODE=1 --setenv=PYTHONUNBUFFERED=1 -- \
  /home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python \
  runs/w3-profile/bench_once.py baseline-w2-s777-w4c 4 777

systemd-run --user --scope -p MemoryMax=6G -p MemorySwapMax=0 -p RuntimeMaxSec=600 \
  --working-directory=$PWD --setenv=PYTHONDONTWRITEBYTECODE=1 --setenv=PYTHONUNBUFFERED=1 -- \
  /home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python \
  runs/w3-profile/bench_once.py async-w2-s777-w4c 4 777 \
  --async-collection --async-ready-games 2
```

`runs/` is gitignored. The JSON reports stay in `runs/w3-profile/`.
