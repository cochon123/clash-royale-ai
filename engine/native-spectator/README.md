# Original-engine desktop spectator

This viewer runs the recovered `Normal` scenario through the static Rust
recompilation and reads presentation snapshots without calling native getters.
It draws original SC5 arena, character, tower and spectator HUD assets.
The default controllers are the original scenario bots. External-frame follow
mode also displays a separate match controlled by saved Banana specialists or
frozen v4.4.2 actors. With `--input-file` (used by `rl.banana_play`), follow
mode also accepts human card controls. It appends select, aim, play and cancel
commands to a JSONL queue, and the Python driver validates them through the
original engine. See `rl/README.md`, "Play against a Banana AI".

Watch the two trained specialists, `banana-hog-1` against `banana-mortar-1`,
with their own native decks at 1×:

```sh
cargo build --release --offline --manifest-path native-engine/Cargo.toml --bin native-watch
cargo build --offline --manifest-path native-spectator/Cargo.toml
/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python -m rl.banana_watch
```

The driver runs on CPU independently of training, verifies each side's native
deck, waits until the spectator window exists, then advances 20 native ticks per
second. Each side samples its own checkpoint; there is no optimizer or
checkpoint write. Space pauses, R restarts, +/- changes speed, and Escape closes
the separate engine and viewer. The final frame remains visible until restart
or close. `--deck mortar` keeps the older mirrored-deck frozen-4.4.2 match, and
`--checkpoint-a` / `--checkpoint-b` select other saved actors.

`rl.banana_watch --headless --max-play-ticks 40 --run-dir runs/watch-check`
performs a bounded frame/action capture without a window or real-time pacing.
It does not claim a completed match.

The native viewer can follow any compatible atomic snapshot file using
`--follow FILE --control-file COMMAND.json --ready-file READY --title NAME`.
This mode never creates another engine session. Metadata reports controller
names, status, errors and speed in the title.

From the project root, open the desktop viewer:

```sh
bash native-spectator/run.sh
```

Space pauses, Right advances one tick, R restarts, +/- changes speed, S saves
`runs/native-spectator.png`, and Escape closes. The engine runs on a separate
thread and forwards every snapshot; the window draws at 60 FPS.
The default window is 540×900. Use `--width` / `--height` to change it.

The watched-policy view shows both actual four-card hands, original card
portraits and elixir icons, native costs, affordability and live elixir, on the
original `ui_spectate_hud` wood panels with its elixir bar.
Characters use the original team variants and directional walk/idle/attack
clips. Towers include the recovered King, cannon and attached Princess layers.
Deployment uses the original scale, translation and color filter. Projectile
view banks are selected separately from timed animations.

### Smooth playback and original effects

Both window modes draw interpolated frames between the 20 Hz native
snapshots: positions, elevation, projectile flight and animation clocks advance
continuously. The display trails the newest snapshot by 16 ticks (0.8 s), so
every observed impact is known before its authored wind-up starts.

- Walk cycles do not restart on the single-tick pauses some units make while
  walking (Ice Golemite holds still every 11 ticks). Facing uses the displayed
  motion with angular hysteresis, or the native focus target when stationary.
- Attack clips align their authored `action_frame` with each observed impact:
  a projectile birth attributed to the nearest shooter whose original
  `Projectile` field names it, or target HP loss for melee units. While a unit
  is engaged (still, native focus target within original `Range`), the original
  `HitSpeed`/`LoadTime` cadence predicts swings between observations.
- Every projectile and spell missile is drawn from its original logic export,
  including rows absent from the observed bindings (Musketeer shots,
  cannonballs, Fireball, Log, Ice Spirits), with its original ground shadow.
  Rolling Log/Barrel art stays perpendicular to its path.
- Original legacy effects play for impacts (`HitEffect`), deaths
  (`DeathEffect`, tower explosions), deployment dust (`SpawnEffect`), muzzle
  flashes (`ProjectileEffect`), melee hits (`DamageEffect`), trails and area
  effects, using the original particle emitters and SC blend modes.
- Hits apply the original directional `filter_damage_*` flash; units cast
  `ShadowScale/Skew/X/Y` silhouettes in the TrainingCamp shadow color; health
  bars are the original `ui.sc` bars (own troops at full health hide them, as
  in `health_bars.csv`).

These inputs come from `analysis/native-spectator/presentation.json`. Create it
once (it reads the local APK and decoded logic; output stays under `analysis/`):

```sh
python3 tools/spectator_presentation.py
```

Without it the viewer still runs, without logic-timed attacks or effects.
Render an interpolated PNG sequence from a capture for review:

```sh
native-spectator/target/debug/clash-native-spectator \
  --follow runs/banana-duel-watch/frames.jsonl --render-sequence runs/sequence-new \
  --from-tick 2850 --to-tick 2900 --fps 30
```

Render a captured live frame without opening a window:

```sh
native-spectator/target/debug/clash-native-spectator \
  --follow runs/v442-visual-capture/frames.jsonl --render runs/watch-preview.png
```

JSONL playback processes the preceding frames so movement, impact cues and
animation clocks have their observed history. A single JSON snapshot is also
supported, with less presentation history.

The debug profile compiles the viewer and its engine dependency at
optimization level 3, so `run.sh` and `rl.banana_watch` composite 60 FPS frames.

Capture a real full match without a window:

```sh
native-spectator/target/debug/clash-native-spectator --run-to-end \
  --record runs/spectator-complete.jsonl --render runs/spectator-complete.png
```

`--ticks N` runs a bounded capture. `--audit-out NEWDIR` saves guest memory,
registers and allocator data for an exact reference comparison.
`--checkpoint DIR` and `--assets DIR` override the pinned inputs.

The original extracted inputs remain local under `analysis/`. If the spectator
bundle is missing, use `.venv-ui/bin/python tools/spectator_assets.py` after
the original UI extraction setup. See [source bindings and verification](../docs/native-spectator.md).

The camera mapping and HUD layout are presentation approximations calibrated
against original assets. Combat and positions come from original game routines.
Facing is derived from native motion and verified focus targets. Attack and
King activation visuals use observed native damage/projectile events; the
original client animation-state stream has not been recovered. The particle
integration interprets the original emitter fields (times in ms, SC-pixel
distances, `ParticleCount` as live budget, `Inertia` as damping); the client's
own particle code is not recovered. Camera behavior and exact native UI layout
remain unfinished. Missing healthy unit visuals appear as explicit markers.
