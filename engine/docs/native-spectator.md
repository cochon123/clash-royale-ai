# Original-asset battle spectator

The new `native-spectator` crate combines the existing static Rust engine with
the original SC5 software renderer. It presents a read-only original `Normal`
AI match, with pause, single-step, reset, playback speed and screenshot controls.
It does not submit player input. External-frame mode also visualizes frozen
v4.4.2 policies driven by `rl.banana_watch`, in a separate original-engine
session independent of the ongoing Banana training run.

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
| Entity identity | entity+8; used by original removal/target routines |
| Focus target | guarded component-slot-0 vtable 0x1193b570; component+0x10; original fc7e20 |
| Z | entity+0x74 plus +0x8c; f942a8 |
| Health | component slot 2 only when vtable is 0x1193b660; current +0x10, max +0x14; fcc140 and fc9ea8 |
| Elixir | side+0x2f8 divided by 10000 minus native queued reservations; fa1c84 / f99808; cap through original globals chain |
| Terminal / winner | battle+0x1e0 byte / +0x1b8 signed integer |
| Arena dimensions | battle+0x188 / +0x18c cells ×500 |

Unknown component classes are not interpreted as health. Moving means a native
position changed since the previous snapshot; exact X/Y deltas and first-seen
ticks are included by `native-watch`. First-seen is an observer timestamp, not
a recovered native birth field. Active attack state has not been bound: the
snapshot's `attacking` remains false. The renderer uses explicitly observational
attack cues: native projectile birth near its shooter, or target HP loss when a
stationary actor's verified native focus points to that target. These visual
cues do not affect the engine, damage, policy inputs or training rewards. Time is
displayed using the 50-unit timer delta observed in original world components,
with 20 updates per second. This is not a recovered original client clock UI.

## Original artwork and renderer

`tools/spectator_assets.py` decodes selected original SC5 documents, retains
source hashes, extracts the original atlas pixels and fonts, and joins native
rows to original nested logic `FileName` and export fields. Observed native
JSONL captures add exact bindings for spawned units and projectiles.

Hog 2.6 and Classic Log Bait artwork includes Princess, both Goblin Gang
units, Barrel Goblins, Goblin Barrel flight and Inferno Tower, plus their hand
portraits. Spawned `EXT` rows inherit the original typed `Base` before applying
their overrides (BarrelGoblin inherits Goblin art and keeps its own load time).
To refresh this bundle without re-decoding unchanged source documents:

```sh
.venv-ui/bin/python tools/spectator_assets.py --reuse-existing --frames runs/banana-duel-watch/frames.jsonl
.venv-ui/bin/python tools/spectator_presentation.py
```

Inferno Tower uses the original animated beam exports, stretched toward its
live, in-range native focus. Warm-up, damage-stage timing and attachment points
are presentation estimates from engagement time, original logic and sprite
bounds. They do not change gameplay or model decisions.

The original `arena_training:training_area_bg` export renders to 2373×1874 at
origin (-1188,-553). Its playing field is cropped from SC bounds x[-270,270]
and y[-60,768]: image rectangle (918,493,540,828), including headroom for the
red King. Presentation maps world x[0,18000] across the field width and world Y
using `((1-y/32000)*768+60)/828` across its height, with side 0 at the bottom.
Native elevation is projected with `z*0.03` source pixels. This camera
mapping is calibrated from assets and initial tower coordinates, rather than
recovered from the original camera routine. Original location CSV data and
TrainingCamp selection are preserved in `assets.json`.

Character sprites use their source export origin and logic scale. Red/blue
variants come from original exports. Motion and resolved native focus provide
a presentation heading; nine-view and five-view banks are mapped separately,
with mirrored poses for the opposite half-turn. Source animation timing uses
native ticks; frames are drawn at fractional ticks between snapshots (see
"Interpolated presentation" below). Attack clips align their authored
`action_frame` to the observed impact/projectile event; the display lag makes
each observed impact known before its wind-up starts.

King reconstruction follows original routine RVA 0x83f4e4: it selects
`chr_king:King_blue` / `King_red` and replaces the empty `king_dummy` child.
The source team-specific child order is retained, with turret object 238 at
(0,-57) for blue and (0,-33) for red. Its 36 frames encode 18 azimuth views,
each held twice; they are not played as a spinning animation. Princess towers
use their original base/top exports plus `TowerPrincess` from `chr_princess`,
at the source attached-character height of 2200 native units.
Observed King damage or allied Princess destruction triggers the original
King/tower `start`→`stop` sequence, then holds the last pose. The trigger is a
presentation inference from native health, not a recovered activation flag.

Deployment applies the original `DeployBaseAnimExportName` movie's root matrix
and color filter to the unit art, ending at the authored clip boundary.
Projectile exports and logic fields distinguish authored view banks from timed
movies; native X/Y/Z drive flight placement. Arrows and Rocket use audited
neutral poses rotated along observed motion; these banks are not played by age
or treated as world azimuth. The original client depth/pitch-to-frame mapping
is unrecovered. Barbarian Barrel and Log advance their authored timelines; their
art is authored rolling east and is turned with the path so the roller stays
perpendicular to it. Rotation follows projected screen velocity, including the
native arc.

The watch bridge publishes each side's real four-slot hand. The spectator uses
hash-checked original PNG portraits, original `ui.sc` card frame and elixir
icon, native costs and live elixir. Both hands are visible simultaneously in
a spectator layout on the original `ui_spectate_hud` red/blue `panel_back` and
`panel` planks. Its elixir bar is assembled from the original `bar_bg`,
`bar_body` (stretched to the elixir fraction), `bar_end`, nine `d1` dividers
and `elixirBarLeft`, as the client places those pieces in code. Old captures
without hands explicitly say they were not recorded. Original fonts render labels. Source sprites, fitted art and text
are cached.

The HUD arrangement, camera and event-to-animation mapping remain presentation
approximations. Exact original client layout, nine-slice resizing and the
client's own animation-state transitions remain unrecovered.

## Interpolated presentation

`tools/spectator_presentation.py` writes `presentation.json` beside the asset
bundle. It joins every native row, transitively through `Projectile`,
`SpawnProjectile` and area-effect links, to its original logic row in
`csv_logic` (attack/load times, range, collision radius, projectile export and
shadow, shadow shear, effect names, health-bar size). It copies the referenced
entries of the APK's `assets/legacy_vfx/effects.json` and `emitters.json`,
including nested effects and enemy variants, and checks every referenced SC
export against the bundled display trees. Unresolved rows, effects, emitters
and exports are listed in the file; nothing is synthesized.

The renderer observes every snapshot and draws any fractional tick between the
two that bracket the display clock. The clock trails the newest snapshot by 16
ticks with soft jitter correction. Per-entity presentation state is kept by
time (walk intervals, impacts, damage filters), because it is observed ahead
of display:

- Walking survives up to three ticks without displacement; observed native
  units pause for single ticks while walking.
- Engagement means two still ticks, a native focus target, and center distance
  within original `Range` plus both collision radii (+400). Engaged units cycle
  `HitSpeed` after `LoadTime`; projectile births and melee HP loss
  re-synchronize the cycle. Clips longer than `HitSpeed` are compressed.
- Removed projectiles continue one observed tick to their impact point, where
  `HitEffect`/`DeathEffect` start; new projectiles leave their attributed
  shooter during the preceding interval.
- HP loss applies `filter_damage_{up,down,left,right}` from the original effects
  document, pushed away from the attributed attacker.
- Legacy effects interpret emitter fields as: milliseconds for lives, delays and
  intervals; SC pixels and pixels per second for positions and speeds;
  `ParticleCount` as the live-particle budget; `Inertia`/10 as exponential
  damping per second; negative gravity downward. Particle heights are drawn at
  0.65 of their simulated value. Effect `Scale` values of 10 or less (muzzle
  flashes use 1) are multipliers, larger values percentages. Exports whose root
  children all use SC blend 8 or 4 are composited additively or as screen,
  since rendering them alone leaves premultiplied color.
- Unit shadows are the sprite silhouette scaled by `ShadowScaleX/Y`, sheared by
  `ShadowSkew` and offset by `ShadowX/Y`, darkened once with TrainingCamp
  `ShadowB=100`, `ShadowA=60` (drawn at 0.5 without the client's blur).
- Health bars are the original `ui.sc` `hp_{player,enemy}_{small,medium,high}`
  and `hp_*_tower_withNumber` movies; the `bar` child's frame is the depleted
  percentage (0 full, 99 empty). Own full-health troops hide their bar, as in
  `health_bars.csv`; card levels are not exposed by the engine and are left blank.

These are presentation interpretations of original data, not recovered client
code; they never affect the engine, policy inputs or rewards.

The opt-in text renderer and object visibility controls added to
`native-ui-viewer` preserve its default static-render behavior. The previous
56 independent reference cases still match exactly, including after the opt-in
root transform/color controls added for deployment and projectiles (retained
at `analysis/native-ui/visual-update-verification`). This establishes agreement
with the corrected asset reference, not pixel parity with the running official
client.

## Actual verification

The visual update was checked against 652 snapshots from a fresh original-native
v4.4.2 custom-deck match (ticks 100–750; 32.5 seconds of gameplay, not a complete
match). Both four-slot hands and all displayed native costs were validated in
every frame; blue changed hand six times and red seven times. All 28 bundled
portraits matched their original hashes. Rendering the complete history left
no unresolved visual rows. Retained inputs/checks are under
`runs/v442-visual-capture/`, with the preview at
`runs/native-ui-improvement-preview.png`. Nine Rust tests pass; two optional
contact-sheet writers are ignored in the normal test run.

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
