# Fast original-engine training

The compiled deck runner executes recovered original ARM routines through the
existing static Rust translation, with no Unicorn fallback. It supports the
mirrored Hog 2.6 and custom Mortar presets used by the Banana specialists.
`rust_deck_rl` restores private checkpoint pages and the allocator journal,
then calls the original deck/input/harness constructors with the requested
native seed. It verifies both eight-card decks before accepting an episode.
Unsupported code paths fail explicitly. Asymmetric decks currently require the
original reference runner; the compiled runner rejects that request.

The separate `rl.banana_fast_train` runner owns one engine subprocess per worker.
Its default three workers collect complete-game waves. Actor weights stay fixed
until every game in the wave has ended, then PPO updates the specialists using
only original terminal rewards (+1/-1/0). Deadline/stop truncations do not receive
an invented reward. Each specialist faces the frozen starting 4.4.2 actor and
alternates physical sides independently. Hog and Mortar do not face each other
in this training mode.

Policy inference groups independent observations by actor. PPO recomputation
batches the first forward pass and the card-conditioned placement pass while
preserving legal hand/elixir masks and the sampled placement-candidate support.
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

## Measured training throughput

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
/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python -m rl.banana_fast_train \
  --run-dir runs/banana-fast-new-run \
  --checkpoint analysis/native-pipeline/oracle-export-01/stage01-attempt001/state \
  --workers 3 --duration-seconds 10800 --device cuda --torch-threads 2
```

Use an empty run directory. Existing training artifacts are rejected rather
than overwritten. This command starts a new run from the original 4.4.2 actor;
it does not resume or replace the completed three-hour Banana run. `--workers`
accepts 1–4; `--eval-interval 0` disables periodic baseline evaluations for a
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
reset, inference, native-step and update wall times. `STOP` saves completed work
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
PYTHONPATH=rl /home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python \
  -m unittest rl/test_banana_fast_policy.py
```

Use `tools/native_rust_compare.py` to compare each final reference/candidate
state audit. Protocol replay excludes only timing fields. Full state audits
compare all retained memory regions, all 70 architectural registers and the
allocator. Agreement establishes the recorded paths, not general game/platform
coverage. Position, damage, update rate, card rules and native RNG are not
approximated to obtain speed.
