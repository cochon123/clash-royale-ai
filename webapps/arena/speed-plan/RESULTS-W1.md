# W1 results — wasm painter

Branch `arena-speed/wasm-painter`. Contract C2 from `PLAN.md`.

## What was built

`engine/native-spectator` and `engine/native-ui-viewer` gained a `wasm` feature. Under that feature the painter does not use minifb, turbojpeg, the filesystem, threads, `Instant`, or `SystemTime`. Assets are read from an in-memory VFS (`clash_native_ui_viewer::vfs`) keyed by bundle-relative paths (`assets.json`, `presentation.json`, `display-trees/...`, `atlases/...`, `fonts/...`, `cards/...`, `data/card_catalog.json`). The disk pose cache returns an error. `SceneDisplayClock` takes its lag from `painter_set_lag_ticks` (wasm default 3.0 ticks). Native `SceneDisplayClock::new` still uses `SCENE_LAG_TICKS` (1.0).

`engine/native-spectator/build-wasm.sh` writes

`engine/native-spectator/target/wasm32-unknown-unknown/release/clash_painter.wasm`

The module has no imports. Exports: `memory`, `painter_alloc`, `painter_free`, `painter_vfs_put`, `painter_init`, `painter_set_lag_ticks`, `painter_observe`, `painter_step`, `painter_result_ptr`, `painter_result_len`, `painter_texture_ptr`, `painter_texture_len`, `painter_texture_release`, `painter_reset`, `painter_stats_json`, `painter_last_error`. Built size 1.6 MB. `painter_step` returns 1 when a packet was written, 0 when the display tick did not move, and -1 on error.

The packet object is the `clash-web-scene` object: `v`, `sequence`, `tick`, `display_tick`, `match_id`, `width`, `height`, `clear` `[19, 27, 31, 255]`, `layout`, `textures`, `draws`, `status`, `playback_speed`, `render_ms`, `texture_count`, `texture_bytes`, `evicted`, `unresolved_rows`. Width and height default to 540×900.

`TextureCache::memory` keeps RGBA (`width*height*4`) and the same 8192 / 256 MB / 30 s LRU. `painter_texture_release` drops the RGBA and keeps the id. `encoded_bytes` / `texture_bytes` count those RGBA bytes. `painter_reset` clears history, the clock, and actors. It leaves the texture cache and the VFS in place.

## Native visual QA

Baseline was accepted on the unmodified tree (`20261024-110057-quick`) before these edits. After the wasm feature:

```
visual_check.py run --expect none --memory-gb 1.2
```

VERDICT **PASS**. Report `.visual-check/runs/20261024-112243-quick/report.html`. 0 new, 0 worsened, 0 fixed findings. `unexpected_changes` is empty. The baseline still contains 21 errors and 2 warnings (unresolved art for rows this bundle does not ship, Tombstone missing logic, two flicker warnings). Those persisted. 2708 frames, 9 quick-suite scenarios, 131 s.

Library tests: `cargo test --lib` on `clash-native-spectator` — 48 passed, 3 ignored. `clash-native-ui-viewer` VFS test passed. `tests/wasm_parity.rs` passed (runner syntax and the C2 export list).

## Measurements

Harness: `engine/native-spectator/tests/wasm_parity/run_parity.py`. Both sides use `ExportSession` (in-memory RGBA cache, lag 3, 30 fps). Observe snapshot `i` when `i/20 <= wall`, then `step(1/30)`. Host `performance.now()` around `painter_step` is the draw time. Peak wasm memory is the high-water mark of linear memory (`memory.buffer.byteLength` and `wasm_memory_peak`).

Window: visual-fixture ticks 140–187 (48 snapshots, 68 packets). The full captures are 1600 frames. Each fixture plays one 8-card deck on both sides. The local bundle has no `scene.card_dependency_documents`, so the harness writes a temporary index under `target/wasm-parity/` (it does not modify `engine/analysis`). Always-loaded documents: `arena_training`, `building_tower`, `effects`, `ui`, `ui_spectate_hud`, `chr_king`, `chr_princess`, `level_bootcamp_arena_dl`, plus the card documents listed in the script. VFS payload for that index: 80 files, 156 MB.

| Load | Draw samples | p50 ms | p95 ms | Peak wasm | Peak Node RSS |
| --- | ---: | ---: | ---: | ---: | ---: |
| hog-mirror (latest run) | 68 | 7.72 | 12.37 | 635.8 MB | 713.2 MB |
| log-bait | 68 | 14.92 | 19.99 | 640.6 MB | 709.3 MB |

Four earlier hog-mirror runs of the same window measured p95 between 12.2 ms and 16.4 ms. Peak wasm stayed 635.8 MB. Plan targets are draw p95 ≤ 8 ms and painter memory ≤ 450 MB on a 16-card load. These runs are above both. At the end of the hog run `cache_bytes` was 0 (RGBA had been released) while `wasm_memory_peak` was still 666632192. The retained memory is the decoded bundle, including the three 4096² `ui` atlases, not the pose cache.

## Parity

Packet counts matched (68 and 68). Draws did not.

Hog-mirror, packet 1, display tick 140.666…, draw 3: an unanchored effect sprite. Native matrix `[0.1522, -0.2970, 0.2970, 0.1522, 133.63, 425.74]`, wasm `[0.0270, -0.3322, 0.3322, 0.0270, 128.72, 419.66]`. The angles differ by 22.49°, one 22.5° step. Pixel tolerance (max 2, mean 0.05) does not cover that step. Later packets diverge from there. Log-bait’s first failure is a published pose of 25×27 on native and 17×18 on wasm.

Texture ids: hog-mirror published 78 poses whose ids differed. 63 of those had identical RGBA (max channel delta 0). `texture_id` uses `std::collections::hash_map::DefaultHasher`, which is not the same on wasm32 and x86_64. C2 expected the ids to match.

## Address gate

Seed `0xC0FFEE`. 24 snapshots, 32 packets. `render_ms` ignored. The texture cache was cleared between passes so the diff is the painter output.

| Trial | hog-mirror | log-bait |
| --- | --- | --- |
| Random permutation of `address` and `native_focus_entity` | draw count 120 vs 119 at packet 1 | 148 vs 146 |
| Same addresses plus 65536 (sorted order preserved) | 120 vs 119 | 148 vs 146 |
| Permutation also applied to `native_entity_id` and `native_focus_entity_id` | 120 vs 119 | 148 vs 146 |

Output depends on the numeric value of `Entity.address` / `native_focus_entity`. W4 must not remap.

## Recorder format (`clash-painter-fixture-v1`)

`clash-fixture-record` (and `run_parity.py`, which calls it) writes a directory:

- `meta.json` — `format`, `lag_ticks`, `fps`, `dt_seconds`, `width`, `height`, `cards`, `snapshots`, `packets`, and the observe/step rules.
- `packets.jsonl` — one `clash-web-scene` packet per line, in order. `render_ms` is the native elapsed time.
- `textures.json` — `[{id, width, height, rgba, png}]`.
- `textures/<id>.rgba` — raw RGBA8, row-major, straight alpha, `width*height*4` bytes.
- `textures/<id>.png` — the same pixels.
- `address-gate.json` — present when `--address-gate` is set.

RGBA is copied and then released, in `textures[]` order, before the next step. W3’s fake painter can replay `packets.jsonl` and take pixels from the PNG or the raw RGBA.

## Contract deviations

- `painter_stats_json` reports `draw_ms: null` and `draw_ms_source: "host"`. Wasm packets set `render_ms` to 0. `Instant` panics on `wasm32-unknown-unknown`, and C2 lists no time import. Draw milliseconds are the host timer around `painter_step`.
- In-memory `texture_bytes` is the RGBA size. The disk cache’s `texture_bytes` is the PNG size.
- Texture ids differ across native and wasm for identical pixels because `DefaultHasher` is not portable. Native ids were left unchanged so VISUAL_QA `--expect none` still passes.
- The 16-slot loads used a temporary `always_documents` / `card_dependency_documents` index. Spells point at `effects`. This is not W2’s index.
- The measured window is 48 snapshots starting at tick 140, not a full match.

## Open problems

- Draw p95 and wasm memory are above the plan targets on this window.
- Effect affines diverge by a full rotation step at the same display tick. The pixel tolerance does not accept that.
- Portable texture ids would change every native display list. That needs a native baseline update, which this workstream did not take.
- `painter_reset` does not free decoded atlases. A long session keeps the ~636 MB working set.
