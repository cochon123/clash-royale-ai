# Original-engine desktop spectator

This viewer runs the recovered `Normal` scenario through the static Rust
recompilation and reads presentation snapshots without calling native getters.
It draws original SC5 arena, character, tower and spectator HUD assets.
The default controllers are the original scenario bots. External-frame follow
mode also displays a separate match controlled by frozen v4.4.2 actors. It has
no human card controls; trained Banana checkpoint selection is not wired yet.

Watch frozen v4.4.2 against itself using the custom Mortar deck at 1×:

```sh
cargo build --release --offline --manifest-path native-engine/Cargo.toml --bin native-watch
cargo build --offline --manifest-path native-spectator/Cargo.toml
/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python -m rl.banana_watch
```

The driver runs on CPU independently of training, verifies both native decks,
waits until the spectator window exists, then advances 20 native ticks per
second. Both sides sample frozen actor weights; there is no optimizer or
checkpoint write. Space pauses, R restarts, +/- changes speed, and Escape closes
the separate engine and viewer. The final frame remains visible until restart
or close. Use `--deck hog` for a mirrored Hog 2.6 match.

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
thread; drawing can skip old snapshots without changing game updates.
The default window is 540×900. Use `--width` / `--height` to change it.

The debug profile deliberately compiles the engine dependency at optimization
level 3. The rest of the viewer uses level 1, avoiding another expensive build
of the generated engine during development.

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
Original client camera behavior, attack-state selection, all effects and native
UI layout parity remain unfinished. Missing visuals appear as explicit markers.
