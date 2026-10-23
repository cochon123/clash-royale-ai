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
