# Minimal original-engine RL environment

This version controls the recovered original ARM64 engine through a persistent
Rust subprocess and a Gymnasium Python interface. It uses the prepared native
`Normal` scenario and its fixed decks. Combat, card validation, elixir, deployment,
hand cycling, completion and winner selection execute in the original engine.
The original battle renderer and the Rust combat rewrite are separate work.

## Banana deck specialists

`banana_train.py` adds a separate warm-started PPO run using the actual sibling
policy-bc-v4.4.2 checkpoint. `banana-hog-1` uses Hog Rider 2.6;
`banana-mortar-1` uses the saved `pixel9` deck: Mortar, Knight, Archers,
Skeletons, Barbarian Barrel, Arrows, Mini P.E.K.K.A and Rocket.
Each specialist plays a frozen starting policy in mirrored-deck games, with
independently alternating seats. They do not play Hog against Mortar directly.

This specialized runner uses **the original ARM64 runtime through Unicorn**.
It builds a fresh native harness with exact source deck rows and a native seed;
the older fixed-deck static Rust backend is unchanged. The actor sees current
hands, elixir and accepted play history, without battlefield positions or HP.
Native placement rejection remains authoritative. These limits matter when
interpreting any apparent win-rate improvement.

The first real run started **2026-10-19 at 13:09:05 America/Edmonton**, with a
shared 10,800-second budget and deadline **16:09:05**. Both specialists completed
native matches, made PPO updates and saved current checkpoints during initial
supervision. The initial actor weights exactly match the source checkpoint;
48 actor tensors changed after the first Hog update, with finite values.
These are initial plumbing checks, not a statistically meaningful strength result.

Live dashboard: <http://127.0.0.1:8780>. Its `/report` page explains provenance,
compute, objectives, metrics, observation limitations and saved artifacts.
Run-wide curves are reconstructed from genuine completed-game log events.

Stream work done and remaining time:

```sh
python3 -m rl.banana_monitor
```

The run and dashboard are durable local systemd user services:

```sh
systemctl --user status clash-banana-train.service
systemctl --user status clash-banana-dashboard.service
```

Closing the browser does not stop training. The dashboard's Stop & save button
creates `runs/banana-league-20261019/STOP`; the trainer saves and reports stopped
status. Initial, latest and final checkpoints live under `models/hog/` and
`models/mortar/` inside that run folder. `run-manifest.json` pins the actual
trainer, adapter, native runner and base-weight hashes. `progress.jsonl` retains
all game/update events. No fake-protocol smoke results enter this live folder.

To reproduce in a **fresh run directory** using the existing CUDA environment:

```sh
cargo build --release --offline --manifest-path native-engine/Cargo.toml --bin deck-setup-probe
/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python -m rl.banana_train \
  --run-dir runs/banana-new-run \
  --engine-bin native-engine/target/release/deck-setup-probe \
  --checkpoint analysis/native-pipeline/oracle-export-01/stage01-attempt001/state \
  --duration-seconds 10800 --seed 41019 --device cuda --torch-threads 3
```

The preflight checks both exact decks and accepted native deployments on both
sides. Fresh games advance through 100 original startup ticks before actions;
the displayed model clock begins afterward. Reward uses only native terminal
outcomes (+1 / −1 / 0). Deadline truncations receive no invented outcome.
The actor update preserves reward sign instead of centering the identical
returns of a single episode. PPO reevaluation uses the rollout's fixed placement
candidate support. Initial/periodic/final checkpoints are saved atomically.

The original minimal Gym environment described below remains available.

From the project root, using the existing local reverse-engineering environment:

```sh
uv pip install --python tools/reverse-env/bin/python -r rl/requirements.txt
cargo build --manifest-path native-engine/Cargo.toml --release --offline --bin native-rl
tools/reverse-env/bin/python -m rl.smoke_test
tools/reverse-env/bin/python -m rl.benchmark --seconds 60 --out analysis/native-rl/benchmark-60s.json
```

The existing original-engine checkpoint, native libraries and captured resources
are required. They are validated by their recorded hashes. Reset loads that
checkpoint into a fresh emulator; steps reuse that instance. Unsupported native
imports or execution failures produce errors and require reset.

## Use from Python

The default executable remains the preserved Unicorn reference. To select the
experimental [static Rust backend](../rust-engine/README.md), build it and pass
its path; the action/observation protocol is the same:

```sh
cargo build --manifest-path rust-engine/Cargo.toml --release --offline
tools/reverse-env/bin/python -m rl.benchmark --seconds 60 \
  --binary rust-engine/target/release/rust_rl \
  --out analysis/native-port/benchmark-fast-60s.json
```

```python
from pathlib import Path
from rl import NativeClashEnv

env = NativeClashEnv(binary=Path("rust-engine/target/release/rust_rl"))
```

Both backends currently use the same fixed scenario/decks and partial
observations. The Rust backend has limited translated-path coverage and guarded
FP support; its README records validation and remaining limits.

```python
from rl import NativeClashEnv

with NativeClashEnv(ticks_per_step=20) as env:
    observation, info = env.reset(seed=1)
    finished = False
    while not finished:
        action = env.action_space.sample()  # replace with your policy
        observation, reward, terminated, truncated, info = env.step(action)
        finished = terminated or truncated
```

The agent controls side 0. The default side-1 opponent chooses an affordable
hand card and random position in Python, then submits it to the original engine.
`opponent="wait"` disables those opponent card plays. The original built-in bot
decision loop is not called during these RL transitions.

The `MultiDiscrete([5,36,64])` action is `[card, x, y]`. Card 0 waits; 1–4 choose
current hand slots 0–3. A position maps to native coordinates
`(500*x+250, 500*y+250)`, sampling the ranges used by the native bot's original
width/height getters. This is a coarse action discretization. The native engine
decides whether each play is legal; rejected plays remain in `info["actions"]`.

Observations are deliberately partial:

| Field | Shape | Meaning |
| --- | --- | --- |
| `tick` | `(1,)` | Original world counter |
| `elixir` | `(2,)` | Both sides' current native elixir |
| `card_ids`, `card_costs` | `(2,4)` | Current hand identities and costs |
| `card_available` | `(2,4)` | A native card exists in that hand slot; affordability is separate |
| `entity_count` | `(1,)` | Current native entity count |

Unit positions, unit health, tower health and images are not included yet. This
is suitable for testing an RL loop and throughput, with limited information for
learning combat. Native card names and outcome fields are available in
`env.raw_observation` and transition info.

Reward is **+1 for side-0 victory, -1 for defeat, 0 for a draw**. Reward is zero
before completion. The original completion flag and signed winner field are
documented in [native-rl-outcome.md](../docs/native-rl-outcome.md). The default
7,200-tick episode cap reports `truncated=True`, with zero reward, if native
completion has not happened; it does not claim a finished game.

`reset(seed=...)` seeds Python opponent sampling. The engine itself resets to
the same verified scenario, RNG state and decks. This first version does not
randomize native decks or support arbitrary engine seeds.

## Measured throughput

The experimental [Rust backend](../rust-engine/README.md#measured-rl-throughput)
now measures **29.94 games/minute**, versus **10.97** in a fresh paired slow run
(2.73×). Both completed without errors or truncations. The historical reference
measurement below is retained for comparison.

The [one-minute benchmark](../analysis/native-rl/benchmark-60s.json) completed
**12 native games in 61.476 seconds: 11.71 games per minute**. All 12 reached
native completion; there were zero truncated episodes or native errors. A
thirteenth episode was reset near the deadline and was not counted. The run
processed 25,621 native ticks and accepted 410 of 575 card attempts. Snapshot
reloads took approximately 1.7 seconds each.

This measurement uses one process on an Intel Core i5-13420H under the desktop's
current load. The [environment record](../analysis/native-rl/benchmark-environment.json)
retains platform and tested code/binary hashes. It includes snapshot reset,
observations, actions, IPC and the original simulation. Initial Rust compilation
and Python import time are excluded. It uses two affordable-random Python
policies, the fixed native scenario/decks, and no renderer or policy training.
It does not predict throughput with different decks, longer battles or a neural
network policy.

The benchmark counts only original engine completions. Truncated and unfinished
episodes are reported separately; native errors stop the measurement. A reset
or native step may finish after the requested duration, so throughput uses the
actual elapsed time. Re-running with the same `--out` path replaces the report;
choose a new path to retain multiple measurements.

The smoke test checks real native card identities, snapshot reset, tick
progression, truncation semantics, Gymnasium compliance and a complete
Python-controlled native match with its win/loss reward.
