# Fast original-engine training

The compiled deck runner executes recovered original ARM routines through the
existing static Rust translation, with no Unicorn fallback. It supports the
mirrored Hog 2.6 and custom Mortar presets used by the Banana specialists.
`rust_deck_rl` restores private checkpoint pages and the allocator journal,
then calls the original deck/input/harness constructors with the requested
native seed. It verifies both eight-card decks before accepting an episode.
Unsupported code paths fail explicitly. Asymmetric decks currently require the
original native watch/setup runner; the compiled runner rejects that request.
The isolated `deck_reference` recovery tool also accepts mirrored games only.

The trainer prefers the profile-guided runner at `rust-engine/target/pgo/rust_deck_rl`
(about 2.8× faster engine replay, +47% training decisions/second; see the
[league simulator notes](fast-league-simulator.md#profile-guided-build)).

The separate `rl.banana_fast_train` runner owns one engine subprocess per worker.
It chooses the available logical CPU count minus two, capped at sixteen
(ten engine workers on this machine). Workers collect complete-game waves. Actor weights stay fixed
until every game in the wave has ended, then PPO updates the specialists using
only original terminal rewards (+1/-1/0). Deadline/stop truncations do not receive
an invented reward. Each specialist faces the frozen starting 4.4.2 actor and
alternates physical sides independently. Hog and Mortar do not face each other
in this training mode.

Policy inference groups independent observations by actor. After the shared
baseline and first specialist are sampled, their CPU games start while the
next specialist uses the GPU. Initial engine preflight also runs across workers
in parallel. Each process has one outstanding request; updates still wait for
every game in the wave.

The online feature encoder builds the confirmed prefix once, including its
exact observation-time global features. The reference encoder built it twice
for an online decision. All thirteen output tensors are checked with exact
equality against the reference at both perspectives, different history lengths,
quiet intervals, and elixir multiplier boundaries.

The policy uses a dense unidirectional GRU, gathering the state at each true
sequence length. The selected-card placement head reuses the first pass's
context. Wait, card and placement distributions and coordinate decoding are
vectorized; rollout metadata moves back to the CPU in batches. PPO retains
the original legal hand/elixir masks and fixed sampled placement support.
CUDA backward uses cuDNN with only the GRU child in training mode and its
inter-layer dropout set to zero; all other modules stay in eval. The GRU state
is restored after the call, including error paths. This preserves the eval
policy math while retaining cuDNN's backward workspace.
Original policy/trainer implementations remain available. Tests compare action
outputs, log-probabilities, entropy, values and parameter gradients against the
original loop; floating-point batching is checked with tolerances. Engine
state comparisons use exact equality.

The first fresh-deck validation records four complete native games (two seeds
for each deck), 775 exact protocol replies, and equal final memory/register/
allocator state. Additional actual model-action traces cover six independent
worker games, PPO updates between collection waves, and their preflight resets. Retained evidence is under
`analysis/fast-decks/`; the small checked-in validation summary identifies the
specific final executable and report hashes.

## CPU/GPU tuning results

| Trial | Actual elapsed | Decisions | Complete games | Decisions/second |
| --- | ---: | ---: | ---: | ---: |
| Previous runner, 3 workers | 60.07 s | 3,226 | 3 | 53.71 |
| Policy/CPU improvements, 8 workers | 60.53 s | 8,366 | 10 | 138.22 |
| Final build, 10 workers, battery power | 120.89 s | 9,814 | 11 | 81.18 |

The eight-worker sample observed 2.57 times the previous decision rate. Power
state was not recorded for those earlier samples. During the final trial the
AC adapter was offline, the profile was balanced, and sampled CPU clocks were
about 1.1–1.3 GHz. This is not a controlled hardware comparison and does not
establish that eight workers outperform ten. Both successful updated runs are
retained; failed coverage-discovery trials are excluded from these rates.

The final two-minute run completed without engine errors or reference recovery.
Nine unfinished games at its deadline received no training outcome. It averaged
about 3.47 CPU cores in the process-tree samples (5.60 peak). Whole-device GPU
utilization averaged 3.51% with a 42% peak; CPU preparation/simulation and the
power state leave the small neural network waiting. The earlier eight-worker
sample averaged 12.20% whole-device GPU utilization with a 63% peak.

Artifacts are `runs/perf-cpu-gpu-before-20261019/`,
`runs/perf-cpu-gpu-eight-fixed-20261019/`, and
`runs/perf-cpu-gpu-ten-final-20261019/`. Resource reports and the recorded
power-state observation are in `analysis/cpu-gpu-tuning/`.
The checked-in [summary](../rust-engine/compute-validation.json) pins the final
binary and validation reports. Earlier eight-worker timing used the preceding
binary before the final Boundary geometry coverage extension.

## CPU/GPU tuning validation

The updated runner has 21 policy, feature, scheduling and recovery tests.
CPU and CUDA checks cover deterministic actions, stochastic rollout likelihoods,
wait/play mixtures, variable legal placement sets, fixed supports after a
parameter change, and PPO parameter gradients. All thirteen encoded features
match the original tensors exactly; neural batching/backend comparisons use
FP32 tolerances.

A longer tuning trial exposed an untranslated branch in the original C++
hash-table sizing helper `std::__ndk1::__next_prime(unsigned long)`. The generator
now accepts original runtime symbols with verified ELF hashes, executable
segment bounds, and byte equality against the retained checkpoint. The full
1,824-byte symbol is translated, including the larger-input branch. This adds
435 instructions. Another longer trial reached the original Boundary geometry
routine and its radius helper; the exact failing request was replayed in the
reference, and both full FDEs were added. The generation now contains 1,910
bodies/runtime ranges and 103,754 instructions, with zero unsupported encodings
in the set.

The helper matches the original ARM code for 606 inputs, including every X
register, SP, PC and NZCV. The rebuilt game binary also matches 775 responses
from the existing four complete games and 305 responses from an additional
native Mortar replay, including the request that reached the missing geometry
routine. These checks are recorded under `analysis/cpu-gpu-tuning/`.
The failed intermediate tuning run is retained there and excluded from
successful throughput measurements.

Reproduce the function check with the installed analysis environment:

```sh
cargo build --release --offline --manifest-path rust-engine/Cargo.toml --example next_prime_probe
tools/reverse-env/bin/python tools/validate_next_prime.py
PYTHONPATH=rl ../.venv/bin/python \
  -m unittest rl.test_banana_fast_features rl.test_banana_fast_policy \
  rl.test_banana_fast_train rl.test_banana_engine
```

The CPU engines run recovered game code; CUDA handles batched neural inference
and PPO. GPU utilization need not reach 100%: input encoding, CPU simulation,
and collection-wave tails still limit the rate. The benchmark wrapper samples
the trainer process tree's CPU consumption and whole-device NVIDIA utilization
and memory. The latter includes desktop/background services and is not
attributed solely to the learner.

```sh
python3 tools/banana_compute_bench.py --run-dir runs/new-compute-benchmark \
  --seconds 120 --workers 10 \
  --fallback-bin analysis/fast-decks/reference-target/release/deck_reference
```

The JSON report includes the exact command, final run state, decisions per
second, completed games per hour, and preparing/training resource samples.
Completed-game rates exclude deadline truncations.

## Earlier compiled-backend benchmark

One-minute-budget trials on this machine used the same starting 4.4.2 model,
CUDA, two Torch CPU threads, two PPO epochs, a 32-action minibatch and no
periodic evaluation. Timing starts after preflight and includes resets, model
inference, native simulation, PPO and checkpoint writes. Final updates may
finish after the budget; rates use actual elapsed time.

| Runner | Actual elapsed | Complete games | Decisions | Decisions/second |
| --- | ---: | ---: | ---: | ---: |
| Original reference, sequential policy/training | 61.10 s | 2 | 928 | 15.19 |
| Compiled Rust, 3 workers, batched policy/training | 64.69 s | 6 | 3374 | 52.16 |

This is **3.43× decision throughput** and approximately
2.83× completed-game throughput in these short trials.
The compiled trial made four PPO updates, used no reference recovery, and had
no truncations or engine errors. The reference trial completed two updates
and excluded one deadline-truncated game from its completion count.

Artifacts: `runs/perf-reference-serial-20261019/` and
`runs/perf-fast-verified-20261019/`. Desktop load and short trial length limit
precision; batching changes the order of policy RNG draws and PPO collection,
so these are throughput measurements, not identical training trajectories or
evidence that the resulting policy is stronger. Engine parity is checked
separately by replaying identical seeds/actions.

A separate engine-only replay used the exact same 775 requests from four
complete Hog/Mortar games. The original runtime took 40.13s and compiled Rust
took 19.89s (**2.02×**), including first load and repeated resets.
Coverage profiling, state-audit writes, rendering and neural inference were
excluded from this replay timing.

## Run

From the repository root:

```sh
cargo build --release --offline --manifest-path rust-engine/Cargo.toml --bin rust_deck_rl
../.venv/bin/python -m rl.banana_fast_train \
  --run-dir runs/banana-fast-new-run \
  --checkpoint analysis/native-pipeline/oracle-export-01/stage01-attempt001/state \
  --workers 10 --duration-seconds 10800 --device cuda --torch-threads 2
```

Use an empty run directory. Existing training artifacts are rejected rather
than overwritten. This command starts a new run from the original 4.4.2 actor;
it does not resume or replace the completed three-hour Banana run. `--workers`
accepts 1–64; `--eval-interval 0` disables periodic baseline evaluations for a
throughput benchmark. `--preflight-only` verifies native setup without starting
the training clock. `--trace-dir NEWDIR` optionally records exact per-worker
protocol requests/replies for source replays.

To view this run with the existing HTML dashboard, use a separate port:

```sh
python3 -m rl.banana_live --run-dir runs/banana-fast-new-run --port 8781
```

The output preserves the existing dashboard/checkpoint format: `state.json`,
`progress.jsonl`, `run-manifest.json`, and `models/{hog,mortar}/`. The manifest
pins the executable, initial model and source files. State includes measured
reset, inference, remaining native-step wait and update wall times, along with
worker/device settings and peak Torch GPU allocation. Inference timing includes
overlapping CPU game execution; native-step timing measures only the subsequent
wait. These timings are not independent stage costs. `STOP` saves completed work
and exits. Checkpoints are written atomically. Timing includes model inference,
PPO, observations and IPC; it is distinct from the raw engine replay benchmark.

For a longer run, `--fallback-bin analysis/fast-decks/reference-target/release/deck_reference`
optionally recovers an episode on a compiled coverage error. It discards the
failed compiled process and replays the same seed and exact action/request
prefix in the original runtime. Every previous semantic reply must agree
before the failed request is retried. A disagreement stops the run. The
reference finishes that episode; the next reset tries compiled execution again.
State reports recovery counts and reasons, and the manifest pins both binaries.
Strict compiled mode remains the default and was used for the speed benchmark.
The compiled Rust crate itself still has no emulator fallback.

## Validation and reproduction

The isolated reference binary is built in a separate target directory. Building
it never replaces the existing training or spectator executables:

```sh
cargo build --release --offline --manifest-path native-engine/Cargo.toml \
  --target-dir analysis/fast-decks/reference-target --bin deck_reference
python3 tools/native_deck_validation.py --record \
  --binary analysis/fast-decks/reference-target/release/deck_reference \
  --output analysis/fast-decks/new-reference.json --seeds 1 2 --audit \
  --profile-dir analysis/fast-decks/new-profiles
```

Reference block profiles guide coverage, not runtime speed claims. Merge them
with the retained original coverage and regenerate using
`tools/native_rust_merge_profiles.py` and `tools/native_rust_codegen.py`.
The generated instruction set reports unsupported encodings explicitly.

```sh
python3 tools/native_deck_validation.py \
  --trace analysis/fast-decks/new-reference.json \
  --binary rust-engine/target/release/rust_deck_rl \
  --output analysis/fast-decks/new-replay.json --audit
PYTHONPATH=rl ../.venv/bin/python \
  -m unittest rl/test_banana_fast_policy.py
```

Use `tools/native_rust_compare.py` to compare each final reference/candidate
state audit. Protocol replay excludes only timing fields. Full state audits
compare all retained memory regions, all 70 architectural registers and the
allocator. Agreement establishes the recorded paths, not general game/platform
coverage. Position, damage, update rate, card rules and native RNG are not
approximated to obtain speed.
