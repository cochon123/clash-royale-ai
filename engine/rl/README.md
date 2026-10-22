# Minimal original-engine RL environment

This version controls the recovered original ARM64 engine through a persistent
Rust subprocess and a Gymnasium Python interface. It uses the prepared native
`Normal` scenario and its fixed decks. Combat, card validation, elixir, deployment,
hand cycling, completion and winner selection execute in the original engine.
The original battle renderer and the Rust combat rewrite are separate work.

## Banana deck specialists

The current **banana-hog-1** is the fresh two-hour run at `runs/banana-hog-1`,
started 21 October 2026 directly from original Banana-1 / policy-bc-v4.4.2.
It uses tower-HP inputs, trainable representation/placement and matched sampled
evaluation. [Current trainer details](../docs/banana-adaptive-training.md).
Earlier Hog checkpoints were deleted at the user's request; the old Hog paths
in the historical commands below no longer exist. Mortar models remain.

The spectator now defaults to the learned timing controller in
`banana_timed_policy.py`, following the two-phone harness in
`../clash-royale-ai/src/cr_replay_pipeline/phone_lab/battle.py`: choose from the
actual hand (including unaffordable cards), anchor the model's delay to the
latest confirmed event, and check native elixir before executing. A planned
card persists while saving, with replanning after confirmed events or hand
changes. Both sides use independent clocks; native tile masks stay enabled.
`--legacy-wait-head` reproduces the former affordable-card/PPO-wait behavior.
This inference controller is separate from the existing PPO wait-head trainer.

For the current matchup:

```sh
../clash-royale-ai/.venv/bin/python -m rl.banana_watch \
  --deck-a hog --checkpoint-a runs/banana-hog-1/models/final.pt \
  --deck-b log-bait --checkpoint-b runs/banana-1/base.pt
```

Each launch picks a fresh random seed and prints it. Pass `--seed NUMBER` to
reproduce a particular match; the chosen seed is also saved in `manifest.json`.

`decisions.jsonl` records predicted delays, planned cards, and waits for elixir.
A 90-second native check of this command deployed three Hogs, played an
original-model Rocket after saving to six elixir, and recorded zero rejections.


`banana_train.py` adds a separate warm-started PPO run using the actual sibling
policy-bc-v4.4.2 checkpoint. `banana-hog-1` uses Hog Rider 2.6;
`banana-mortar-1` uses the saved `pixel9` deck: Mortar, Knight, Archers,
Skeletons, Barbarian Barrel, Arrows, Mini P.E.K.K.A and Rocket.
Each specialist plays a frozen starting policy in mirrored-deck games, with
independently alternating seats during training. A separate spectator can load
both final checkpoints and run one asymmetric Hog-versus-Mortar match without
training either model.

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

## Watch the two trained specialists fight

```sh
/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python -m rl.banana_watch
```

This opens the original-asset desktop spectator with `banana-hog-1` (Hog 2.6)
on the blue side against `banana-mortar-1` (Mortar) on the red side. It loads
each specialist's final checkpoint from the Banana League run, sends both deck
presets to the original engine (the native watch runner accepts a separate
`deck_preset_side1`), and samples each actor from its own causal perspective.
No optimizer or training-run file is written. Space pauses, R restarts, +/-
changes speed, and Esc closes. Initial startup is skipped before the displayed
model clock begins.

Point either side at another saved `banana-native-ppo-v1` checkpoint:

```sh
/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python -m rl.banana_watch \
  --deck-a hog --deck-b mortar \
  --checkpoint-a runs/banana-league-20261019/models/hog/latest.pt \
  --checkpoint-b runs/banana-league-20261019/models/mortar/latest.pt
```

Use `--headless --max-play-ticks 200` for a bounded frame/action capture without
a window. `--deck mortar` keeps the older mirrored-deck frozen-4.4.2 view.

To compare the untrained warm-start actor against the trained Hog specialist,
mirror Hog 2.6 and load the final checkpoint on side 1 only:

```sh
/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python -m rl.banana_watch \
  --deck hog --checkpoint-b runs/banana-league-20261019/models/hog/final.pt
```

The base actor is the frozen 4.4.2 checkpoint the league started from; the
initial `models/hog/initial.pt` has the same actor weights.

## Play against a Banana AI

```sh
cd /home/cochon/Documents/mini-project/clash-rust
../clash-royale-ai/.venv/bin/python -m rl.banana_play --opponent banana-hog-1 --deck hog --ai-deck log-bait
```

You play blue (bottom) with `--deck`; the saved actor plays red with
`--ai-deck`. `--opponent` is `banana-1` (original warm start) or
`banana-hog-1` (`runs/banana-hog-1/models/final.pt`); decks are `hog`,
`log-bait` or `mortar`. The command rebuilds `native-watch` and the spectator
when their sources changed.

- Drag a card from your hand and drop it on the arena to deploy. You can also
  press **1-4** or click a card, then click the arena.
  Right-click or **Esc** cancels the selection.
- **Space** pauses, **R** starts a new match, **Q** or closing the window quits.
  **S** saves `screenshot.png` in the session folder.
- Cursor colours: green is accepted, red is a blocked tile (towers, outside
  the arena), and amber is not enough elixir. Aiming a troop, building or the
  Log past your territory shades the no-deploy zone red, and the cursor jumps to
  where it will land: the edge of your side by the bridge, in the same column.
  A lane whose enemy Princess Tower has fallen is not shaded (its pocket rule
  is not modelled). The engine still validates every play.

Every human play is checked by the original target validator and submitted
through the same native path as the AI's plays. The AI uses the learned-delay
scheduler and native placement masks from `banana_watch`. The opponent's hand is
hidden, and the display trails the engine by 2 ticks (100 ms) instead of the
spectator's 16. Each session writes `runs/banana-play/<time>-<pid>/`. In it,
`actions.jsonl` tags every play `human` or `ai`, and `inputs.jsonl` holds the
raw viewer commands. `--seed N` reproduces a match, and `--greedy` makes the AI
choose its most likely actions instead of sampling. `--headless` reads
`inputs.jsonl` without a window for scripted checks.

## Watch a separate frozen 4.4.2 match

```sh
/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python -m rl.banana_watch --deck mortar
```

This opens the original-asset desktop spectator for a mirrored custom Mortar
deck match at 1× (20 native ticks/second). It uses a separate original-engine
`native-watch` process and CPU inference. Both sides use the frozen original
4.4.2 actor; no learning or training-run file writes occur. Space pauses, R
restarts, +/- changes speed, and Esc closes. Initial startup is skipped before
the displayed model clock begins. See [spectator setup](../native-spectator/README.md).

The driver records provenance, actual native actions, snapshots and state in
`runs/v442-custom-watch/`. A 40-tick diagnostic capture verified native deck
selection, both controllers and accepted card submission before launch. The
separate displayed match is paced in real time, while headless diagnostics
run without pacing. This uses the same partial-observation adapter as training.

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
