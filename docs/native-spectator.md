# Original-asset battle spectator

The new `native-spectator` crate combines the existing static Rust engine with
the original SC5 software renderer. It presents a read-only original `Normal`
AI match, with pause, single-step, reset, playback speed and screenshot controls.
It does not submit player input or run a Banana policy.

## Source bindings

The pinned original ELF SHA-256 is
`79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6`.
The initial checkpoint SHA-256 is
`66f9bf75bfde9b5e0c1f4056573079016d41576020018b9e57fca538fd1b840e`.

`backend.rs` calls original harness step RVA `0x11d6398`, using the same
eight-byte result slot as the reference runner. Snapshots read memory directly:

| Value | Recovered layout / accessor evidence |
| --- | --- |
| Entity list | world+0x40 manager; manager+8 pointers, +0x14 count |
| Native row | entity+0x48; native string at row+0x28 |
| Side | entity+0x78; original f94124 |
| X / Y | entity+0x7c / +0x80; f94298 / f942a0 |
| Z | entity+0x74 plus +0x8c; f942a8 |
| Health | component slot 2 only when vtable is 0x1193b660; current +0x10, max +0x14; fcc140 and fc9ea8 |
| Elixir | side+0x2f8 divided by 10000 minus native queued reservations; fa1c84 / f99808; cap through original globals chain |
| Terminal / winner | battle+0x1e0 byte / +0x1b8 signed integer |
| Arena dimensions | battle+0x188 / +0x18c cells ×500 |

Unknown component classes are not interpreted as health. Moving means a native
position changed since the previous snapshot. Active attack state has not been
bound; the renderer does not infer it from the nearby target list. Time is
displayed using the 50-unit timer delta observed in original world components,
with 20 updates per second. This is not a recovered original client clock UI.

## Original artwork and renderer

`tools/spectator_assets.py` decodes selected original SC5 documents, retains
source hashes, extracts the original atlas pixels and fonts, and joins native
rows to original nested logic `FileName` and export fields. Observed native
JSONL captures add exact bindings for spawned units and projectiles.

The original `arena_training:training_area_bg` export renders to 2373×1874 at
origin (-1188,-553). Its playing field is cropped from SC bounds x[-270,270]
and y[0,768]: image rectangle (918,553,540,768). Presentation maps world
x[0,18000], y[0,32000] onto that field, with side 0 at the bottom. This camera
mapping is calibrated from assets and initial tower coordinates, rather than
recovered from the original camera routine. Original location CSV data and
TrainingCamp selection are preserved in `assets.json`.

Character sprites use their source export origin and logic scale. Red/blue
variants come from original exports. Root and child timelines use elapsed
native tick time. Source frames, fitted HUD art and text are cached. Live
elixir labels and player names use recovered text fields and original fonts.
The HUD banners currently use a first-pass arrangement; native client layout,
nine-slice resizing, all health effects, destruction visibility and precise
animation-state transitions are not complete.

The opt-in text renderer and object visibility controls added to
`native-ui-viewer` preserve its default static-render behavior. The previous
56 independent reference cases still match exactly. This establishes agreement
with the corrected asset reference, not pixel parity with the running official
client.

## Actual verification

The spectator advanced 100 original harness ticks, reading snapshots after
every tick. Its saved result was compared with the original Unicorn runner:
**all 12 guest memory regions, 70 registers and allocator contents matched**.
The retained comparison is
`analysis/native-spectator-compare-100.json`. Snapshot reads do not change the
native game for this audited sequence.

A complete capture terminated at native tick 1354, winner side 0:

- 1,352 steps and 1,353 snapshots, including the initial state.
- 4,607 position-change observations and 806 HP-decrease observations.
- Up to 17 simultaneous entities and 28 distinct native row names.
- 25 rows have exact original file/export bindings. GoblinCurse,
  GoblinCurseBase and SkeletonKingGraveyard have no direct file/export in the
  inspected source logic and remain explicit unresolved markers.
- Capture and final PNG rendering took 26.52 seconds in the development build
  on this machine, including loading and per-tick JSON serialization. This is
  not the headless RL throughput benchmark.

Artifacts are `runs/spectator-complete.jsonl`,
`runs/spectator-complete-result.json`, and `runs/spectator-complete.png`.
The six-tower rendering fixture averaged 8.38 ms/frame in the test profile;
that is a warm rendering microbenchmark, not full-window battle throughput.

Reproduce the noninterference comparison with fresh output directories:

```sh
native-spectator/target/debug/clash-native-spectator --ticks 100 \
  --audit-out analysis/spectator-candidate-new
native-engine/target/release/native-oracle \
  --checkpoint analysis/native-pipeline/oracle-export-01/stage01-attempt001/state \
  --world-steps 0 --harness-steps 100 --audit-out analysis/spectator-reference-new
tools/reverse-env/bin/python tools/native_rust_compare.py \
  --reference analysis/spectator-reference-new \
  --candidate analysis/spectator-candidate-new \
  --out analysis/spectator-comparison-new.json
```
