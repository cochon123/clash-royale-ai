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

## Round 2

The round-1 mismatches were two bugs, plus one libm pixel that is inside the C2 tolerance.

`Vfx::clear` did not reset the xorshift or the sprite id counter. `clash-fixture-record` runs the address gate on the session and then records. Each gate pass calls `reset`, so the native recording started with a used RNG and ids already in the thousands (`vfx-1812`, `vfx-2410`). The wasm runner never runs the gate, so it started at id 1 (`vfx-9`, `vfx-12`). The harness compares draws by index and strips ids when the counts differ, which made that look like one sprite rotated by 22.5°. It was a different particle. `clear` now restores the seed `0x9e37_79b9_7f4a_7c15` and `next_id = 1`. A fresh painter does not call `clear`, so this does not change a cold native process.

`texture_id` and the fitted-pose cache key hashed through `DefaultHasher`. `[u8]`, `Vec<u8>`, `str` and `usize` write a length prefix as `usize`: 8 bytes on x86_64, 4 on wasm32. `Width64Hasher` forwards `write_usize` / `write_isize` as `write_u64` / `write_i64`. On this machine `width64_hasher_matches_native_default_hasher` shows the digest equals `DefaultHasher`, so native ids do not move. There is no `HashMap` in the painter or viewer render path; maps are `BTreeMap`.

### Native output

`visual_check.py run --expect none --memory-gb 1.2` — VERDICT **PASS**. Report `.visual-check/runs/20261024-120213-quick/report.html`. 0 new, 0 worsened, 0 fixed. `unexpected_changes` is empty. The baseline still has 21 errors and 2 warnings (same unresolved-art and flicker set as round 1). 9 scenarios, 129 s. Library tests: 49 passed, 3 ignored. VFS test passed. `tests/wasm_parity.rs` passed, including the export list with `painter_set_draw_ms`.

Direct diff of the new native packets against the round-1 recordings (`target/wasm-parity-round1/`): hog-mirror and log-bait packet 0 are equal ignoring `render_ms` (same texture ids, same draw ids). Packet 1 is where the old recording’s effect ids begin (`vfx-1812` / `vfx-2410` versus `vfx-9` / `vfx-12`). Those old ids are the RNG carry. They are not a cold-painter baseline. VISUAL_QA is the cold-painter check, and it matches the accepted baseline.

### Parity

Same harness and window as round 1 (ticks 140–187, 48 snapshots, 68 packets, lag 3, 30 fps). Draws compared at relative tolerance 1e-9.

| Fixture | Draws | Texture ids | Pixels outside tolerance |
| --- | --- | --- | --- |
| hog-mirror | equal, 68 packets | 1 mismatch | 0 |
| log-bait | equal, 68 packets | 1 mismatch | 0 |

The one id mismatch is the same pose on both fixtures, packet 0: native `2adaa24290b4d529126fb9e5a920b9f9`, wasm `a66bdb78b6134e73ff59ca25578f9c69`. Max channel delta 1, mean `1.70e-6`. That is inside max 2 and mean 0.05. It is the arena background resample (Lanczos over `f64`); one channel differs, so the hash differs. The harness remaps that id before comparing draws. No other pose differed. The 22.5° matrix gap and the 25×27 vs 17×18 pose are gone.

Quick-suite captures replayed through W2’s read-only subset bundle (`arena-speed-assets/deploy/arena/derived-bundle`, not modified): pekka-bridge, hog-mirror, golem, lava-loon, log-bait, custom-mortar. All six: draws equal, the same single max-delta-1 texture, 0 pixels outside tolerance. The full local bundle has no `card_dependency_documents`, so those four extra captures were not repeated on it.

### Draw time and memory

`painter_set_draw_ms(f64)` records the host `performance.now()` around `painter_step`. `painter_stats_json` `draw_ms` is that last publishing step (hog 6.14 ms, log-bait 12.87 ms). `render_ms` inside the packet stays 0: the packet is sealed before the host knows the duration. p50/p95 below are every publishing step, measured with nothing else running.

| Load | p50 ms | p90 ms | p95 ms | max ms | Peak wasm | After init | Atlas RGBA at end |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| hog-mirror, full index | 7.94 | 12.47 | 14.97 | 1460 | 409.8 MB | 217.6 MB | 294.4 MB |
| log-bait, full index | 14.85 | 18.94 | 19.94 | 1198 | 414.6 MB | 222.4 MB | 289.9 MB |

The max is the first step, which inflates the atlases it actually samples. Hog’s next steps are 15 ms, 28 ms, then about 5 ms; 7 of 68 draws are over 12 ms, and the p95 is that second step. Log-bait’s first step is 1198 ms and 51 of 68 draws are over 12 ms, so its p95 is the board, not the decode. Plan p95 ≤ 8 ms is still missed on both. Decoding every atlas page up front would also decode pages this window never samples (`ui_3` 21 MB RGBA, `ui_4` 6.4 MB, `chr_hog_rider_0` 10.7 MB, `level_bootcamp_arena_dl_0` 7.5 MB). Wasm linear memory does not shrink, so those pages would raise the high water. They stay as PNG until used.

Where the 410 MB goes, hog-mirror full index (80 files, 149 MB put into the VFS):

- After `painter_init`: 218 MB. Display-tree JSON is parsed and `vfs::take`n (`ui.json` 43 MB, `effects.json` 18 MB, the rest of the trees, `assets.json`, `presentation.json`, the catalog). The parsed documents stay. The byte length does not go back down when the raw JSON is freed.
- End of the window: 294 MB of decoded atlas RGBA, 16.0 MB still in the VFS. The leftover is fonts (2.6 MB), every card portrait the temporary index links (4.6 MB; portraits are decoded into the hand cache and the PNG is kept because a second card can share a file), and the unsampled atlas PNGs above.
- Peak 409.8 MB is the high water while those PNGs are inflated into RGBA, not a set that can be returned to the OS. Round 1 held every PNG and every decoded page at once (636 MB).

W2 subset, same window, VFS limited to that bundle’s `always_documents`, the fixture’s `card_dependency_documents`, fonts, `cards/`, and the catalog. The bundle directory was only read.

| Capture | VFS | p50 ms | p95 ms | Peak wasm | Atlas RGBA |
| --- | ---: | ---: | ---: | ---: | ---: |
| pekka-bridge | 83.0 MB | 3.95 | 6.71 | 195.9 MB | 95.8 MB |
| hog-mirror | 57.6 MB | 7.90 | 16.00 | 172.8 MB | 95.7 MB |
| golem | 57.3 MB | 4.07 | 13.96 | 176.3 MB | 104.7 MB |
| lava-loon | 50.2 MB | 6.24 | 12.62 | 156.8 MB | 88.6 MB |
| log-bait | 56.4 MB | 14.98 | 20.88 | 170.1 MB | 91.2 MB |
| custom-mortar | 68.7 MB | 11.32 | 21.42 | 204.8 MB | 115.9 MB |

The full-index 16-slot window is under the 450 MB cap (409.8 / 414.6). The subset peaks are 157–205 MB. p95 ≤ 8 ms holds for subset pekka-bridge only.

### Address gate

Seed `0xC0FFEE`, 24 snapshots, 32 packets, `render_ms` ignored. Remeasured after the RNG reset. Round 1’s packet-1 draw-count failure was the RNG carry between passes, not the addresses.

On the full index, an order-preserving shift (`address + 65536`, `BTreeMap` order unchanged) matches on hog-mirror and log-bait. A random permutation does not: hog-mirror packet 22 `$.draws[27].m[4]` `152.988` vs `100.980`, log-bait packet 22 `$.draws[100].m[4]` `105.189` vs `129.882`. Applying the same map to entity ids fails the same way. Actor storage is a `BTreeMap` keyed by address, so a permutation that changes sort order changes who consumes the effect RNG. Output still depends on `Entity.address` / `native_focus_entity`. W4 keeps remap off.

On the subset bundle the same gate is independent for pekka-bridge, golem, and lava-loon in this window, and dependent for hog-mirror, log-bait, and custom-mortar (mortar fails at packet 18, a translation). Independence is per board, not a license to remap.

`address_permutation` reduces the LCG with `as usize`. On this x86_64 recorder that is the full 64-bit word. A wasm32 run of the gate would truncate. The gate stays in the native recorder; the cast was not changed. `image_key` is still the in-process pixel-buffer address and is not a packet field.

### Still open

- Draw p95 is above 8 ms on hog-mirror and log-bait. The first wasm step spends about 1.2–1.5 s inflating atlases.
- One arena texture id differs by a single channel. Pixels are inside the C2 libm clause.
- Addresses still change effect positions on hog, log, and mortar. Remap stays off.
- `painter_reset` still does not free decoded atlases. Peak memory is a high-water mark.
