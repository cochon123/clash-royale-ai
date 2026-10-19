# Original-engine desktop spectator

This viewer runs the recovered `Normal` scenario through the static Rust
recompilation and reads presentation snapshots without calling native getters.
It draws original SC5 arena, character, tower and spectator HUD assets.
The current controllers are the original scenario bots. It is not connected
to the Banana policies yet, and has no human card controls.

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
