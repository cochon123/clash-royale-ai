# Arena speed plan: run the presentation in the browser

Status: **plan only, not started.** Written 2026-10-24 for implementation by
Grok 4.7 agents (T3 `delegate_task`, provider `cursor`, model `grok-4.7`,
options `contextWindow: 256k`, `fastMode: true`).

Goal: Arena battles on <https://calgarypermit.ca/clashroyale/> start in about
2 s and play smoothly at real speed. No multi-second freezes. No texture
downloads during play.

## 1. What is slow (measured 2026-10-24, release `20261024-01-inline-poses`)

Scenario: a watch match, `banana-hog-1` (hog-mirror) vs `banana-1` (log-bait),
45 s, headless Chrome 154 with WebGL2, against the public site. The raw numbers
are in [`baseline-2026-10-24.json`](baseline-2026-10-24.json). Reproduce them with
[`probe_stream.py`](probe_stream.py) (server side) and
[`probe_browser.py`](probe_browser.py) (what the browser shows).

| Measurement | Cold browser | Warm browser |
| --- | ---: | ---: |
| Create to first visible frame | 8.09 s | 7.72 s |
| **Time the picture was frozen** (game time not advancing), out of 38.5 s | **33.8 s** | **35.4 s** |
| Longest single freeze | 19.3 s | 11.4 s |
| Texture HTTP requests during the match | 1,416 | 1,531 |
| Scene packets dropped by the client | 738 | 731 |
| Frames with the pending queue full (blocked on textures) | 715 / 767 | 684 / 731 |

The server itself is fine:
- **The server keeps real time.** The engine ran at 20.01 ticks/s and sent
  scene packets at 20/s. VPS CPU was 60–80% idle.
- **The engine is not the bottleneck at 1×.** One native tick takes 6.5 ms
  (median) and 9 ms (p95) on the VPS.

The browser alternates long freezes with jumps of 12–20 s of game time.

Startup on the VPS (2 vCPU EPYC, `CPUQuota=150%`):
- `native-watch` reset takes **3.8–4.1 s**. Every reset re-reads
  `checkpoint.json` and all 354 MB of region files, and runs external
  `sha256sum` on them, on `libunicorn` and on `libg.decrypted.so`
  (`NativeSession::load`, `engine/native-engine/src/oracle.rs:1614`).
- Policy loading takes 0.2 s per side, and match manifests hash the engine
  binary.
- The server-side presenter (`clash-web-scene`, deployed from `target/debug`)
  uses **780 MB RSS**. The Python process uses 344 MB and `native-watch` uses
  842 MB.

## 2. Root cause

Presentation runs on the server and is delivered pose by pose over the network.

1. The Rust `Painter` runs on the VPS. Every new animation pose it composes
   becomes a PNG file. The browser learns about a pose only when a display list
   references it.
2. The browser fetches each pose as a separate HTTP request (concurrency 4,
   about 40/s at this RTT). A match needs about 35 new poses/s in its first
   minute.
3. `ArenaSceneRenderer.promote()` only shows a frame once **every** texture it
   references is loaded. This is head-of-line blocking: one missing pose freezes
   the whole picture. The pending queue (4 frames) fills, packets are shed, and
   the display jumps forward when the textures arrive.
4. Inline blobs (`MAX_INLINE_*`) only cover poses encoded *since the last
   packet*. The shared server pose cache already holds about 8,146 poses from
   earlier matches. Those are never inlined, so almost everything is fetched.
5. Display lists are 50–100 KB of JSON per packet at 20/s, about 1.5 Mbps after
   gzip. A 50 ms interpolation buffer leaves no headroom for network jitter.

Tuning the transport (bigger inline budget, more concurrency, prefetch) would
reduce the symptoms. The structure would stay the same: thousands of
on-demand images, server CPU spent painting, and a network round trip between
the simulation and every new pose.

## 3. Target architecture

```
VPS (authoritative)                              Browser
-------------------                              -------
native-watch (Unicorn, warm pool, fast reset)    main thread: DOM HUD, input overlay, POST /input
   | JSONL                                          ^ layout, status
Python match loop, 20 ticks/s                       |
   | one Snapshot per tick (~3-15 KB)            Painter worker
   +-- SSE /api/matches/{id}/ticks ------------>   painter.wasm = the same Rust Painter,
       (ordered, complete, ring buffer,            fed from a virtual FS of original assets
        resumable, batched at >1x)                 (fetched once per card, immutable, cached)
static /clashroyale/assets/  -- packs + wasm -->   observe(every tick) -> draw(display_tick)
                                                   -> display list + new pose RGBA (local, synchronous)
                                                   -> WebGL2 compositor on an OffscreenCanvas
                                                      (existing GLPainter, existing interpolation)
```

- **Server.** It runs the engine and the AI and streams the *same* `Snapshot`
  JSON it currently writes to `clash-web-scene`'s stdin, plus the small HUD
  payload. The server no longer runs a presenter on the default path. Explicit
  **Stream view** (JPEG `clash-web-renderer`, started lazily) stays as the
  compatibility fallback.
- **Browser.** It runs the Painter compiled to `wasm32-unknown-unknown`, so the
  pose selection, draw order and layout code is the exact code in
  `engine/native-spectator`. Poses are rasterized locally from original atlases
  that are already in memory. A frame therefore never waits on the network for
  art. The existing WebGL2 compositor, blend modes, shadows and
  `interpolateDraws` are reused unchanged.
- **Jitter buffer.** Network jitter is absorbed *before* the Painter: the display
  clock runs about 150 ms behind the newest received tick (adaptive, 100–300 ms).
  Painter → compositor is local and steady, so the existing 50 ms interpolation
  delay stays.
- **Startup.** The engine is preloaded and resets from a pristine in-memory
  template. The browser fetches packs and initializes the Painter while the
  server configures the battle.

### Rejected

- **Bigger inline budget / prefetch / HTTP tuning.** It treats the symptom and
  keeps 1.5 Mbps of display lists and server painting.
- **Running the engine in the browser.** The checkpoint is 354 MB, and the
  recompiled engine needs fixed guest mappings and has six-deck coverage only.
- **A full GPU port of SC shape drawing.** It is a rewrite of `draw.rs`
  (3,500 lines) and loses parity by construction. Keep CPU pose rasterization
  in WASM; revisit only if phone memory fails (section 8).
- **Swapping to the recompiled engine now.** It is not the measured bottleneck.
  It is phase 5, conditional (section 6).

## 4. Workstreams

Integration branch: `arena-speed`, from `main` *after* the deployed Arena
working-tree changes are committed (see prerequisite P0). Each agent works only
in its own worktree under
`/home/cochon/Documents/mini-project/clash-royale-ai-worktrees/` and commits
only to its own branch. Opus integrates.

| ID | Branch / worktree | Language | Scope |
| --- | --- | --- | --- |
| W1 painter | `arena-speed/wasm-painter` / `arena-speed-wasm` | Rust | Painter and viewer crates build for wasm32; C2 ABI; native↔wasm parity harness |
| W2 assets | `arena-speed/assets` / `arena-speed-assets` | Rust + Python | C1 pack builder, subsetting of `ui`/`effects`, size report, deploy packaging |
| W3 client | `arena-speed/client` / `arena-speed-client` | JS | Painter worker, OffscreenCanvas compositor, jitter clock, loading UI, diagnostics, fallbacks |
| W4 server | `arena-speed/server` / `arena-speed-server` | Python | C3 tick stream, static asset routes, CSP/nginx, engine pool consumer, startup caches, legacy flag |
| W5 engine | `arena-speed/engine` / `arena-speed-engine` | Rust | C4: preload, verify-once, in-memory reset template, `ping` op, equivalence tests |

Phase 1 runs all five in parallel. W3 develops against a fake painter (section
5, C2) and recorded tick JSONL. It does not wait for W1 or W4.

## 5. Interface contracts (fixed: implement exactly so the branches merge)

### C1. Asset packs (W2 produces; W1, W3 and W4 consume)

- Logical model: a **virtual file system** with the same relative paths as the
  bundle `engine/analysis/native-spectator/`, for example `assets.json`,
  `presentation.json`, `display-trees/chr_hog_rider.json`,
  `atlases/chr_hog_rider_0.png` and `fonts/...`. The WASM Painter reads only
  through this VFS (C2), so any file the native Painter reads must be
  addressable by its relative path.
- Builder: `deploy/arena/build_web_assets.py --assets DIR --out DIR` (it may
  shell out to a Rust helper bin in `native-spectator`). The output is:
  - `files/<sha256[:20]>.<ext>`: content-addressed. JSON also gets
    precompressed `.gz` and `.br` siblings. PNG is stored as is.
  - `manifest-<sha256[:12]>.json`:
    `{"v":1,"bundle":"<source bundle fingerprint>","painter_wasm":"files/<hash>.wasm",
    "files":{"<relative path>":{"url":"files/<hash>.<ext>","bytes":N,"sha256":"..."}},
    "always":["<relative path>",...],
    "cards":{"<native card row>":["<relative path>",...]}}`.
    `always` and `cards` come from `scene.always_documents` and
    `scene.card_dependency_documents` (the keys that `draw.rs:142` already
    reads), with each document expanded to its display tree and atlas pages.
    **Neither the deployed bundle nor the local bundle has these keys yet.**
    This is why production `clash-web-scene` loads the whole bundle (780 MB).
    W2 generates both keys into a derived bundle. The index is accepted only if
    every visual fixture gives identical native display lists with the deck
    subset and with the full bundle.
  - `manifest.json`: a short-cache pointer `{"current":"manifest-<hash>.json"}`.
- Subsetting: `ui.json` (45 MB), `effects.json` (18.7 MB) and the `ui_*.png`
  atlases (three 4096² pages) are reduced to the exports and atlas regions the
  Painter can reach for HUD, towers and effects. The schema is unchanged. The
  subset bundle is accepted only if the native Painter produces **identical
  display lists and pose pixels** on the visual fixtures with the full bundle
  and with the subset (`engine/tools/visual_check.py run --assets <subset>
  --expect none` PASS, plus a direct diff tool).
- Size report `deploy/arena/web-assets-report.json`: for each of the six preset
  pairs (and the worst custom pair), cold-download bytes (compressed on the
  wire) and decoded RGBA bytes.

### C2. Painter WASM ABI (W1 produces, W3 consumes)

Raw C ABI. No `wasm-bindgen` CLI is needed, which keeps builds offline. The
crate is `engine/native-spectator`, with feature `wasm` and target
`wasm32-unknown-unknown`, built by `engine/native-spectator/build-wasm.sh`.
Output: `engine/native-spectator/target/wasm32-unknown-unknown/release/clash_painter.wasm`.

All strings and JSON are UTF-8 in linear memory. Return codes: 0 means ok;
nonzero means an error, readable through `painter_last_error()`.

```
painter_alloc(len) -> ptr                       painter_free(ptr, len)
painter_vfs_put(path_ptr, path_len, data_ptr, data_len) -> rc    // bytes copied in
painter_init(cards_json_ptr, len, width=540, height=900) -> rc   // load_for_cards over the VFS
painter_set_lag_ticks(f64)                      // display lag behind newest observed tick (default 3.0)
painter_observe(snapshot_json_ptr, len) -> rc   // EVERY tick, in order (Painter.observe)
painter_step(dt_seconds: f64) -> i32            // SceneDisplayClock.step + draw; 1 = new packet, 0 = none, <0 error
painter_result_ptr() -> ptr ; painter_result_len() -> len   // packet JSON of the last step
painter_texture_ptr(id_ptr, id_len) -> ptr ; painter_texture_len(...) -> len  // RGBA8, w*h*4, valid until next call
painter_texture_release(id_ptr, id_len)         // JS uploaded it; drop the RGBA copy, keep the id known
painter_reset()                                 // new round / resync: clear history, clock and actors
painter_stats_json() -> rc (result buffer)      // draw ms, observe count, cache bytes, wasm memory
painter_last_error() -> rc (result buffer)
```

- The packet JSON is **exactly** today's `clash-web-scene` stdout packet
  (`SCENE_PROTOCOL.md`): `v, sequence, tick, display_tick, match_id, width,
  height, clear, layout, textures, draws, status, playback_speed, evicted`.
  `textures` lists newly created poses. Their pixels come from
  `painter_texture_*`, never from PNG.
- In-memory `TextureCache` keeps today's LRU budget semantics (`evicted` is
  still emitted). `pose_cache.rs` (disk) is disabled under `wasm`. Time comes
  from the `dt` argument or an injected clock, never from `Instant` or
  `SystemTime`. Both panic on `wasm32-unknown-unknown`.
- `minifb`, `turbojpeg`, filesystem access and threads are feature-gated off
  under `wasm`, in `native-ui-viewer` as well as `native-spectator`. Native
  builds and their output must not change: VISUAL_QA `--expect none` PASS on
  the quick suite.
- **Parity harness** (`engine/native-spectator/tests/wasm_parity` plus a Node
  runner): feed the visual-fixture captures through the native Painter and the
  wasm Painter on the same display-tick grid. Draws must be equal; floats are
  compared with relative tolerance 1e-9. Texture ids are expected to be equal.
  If libm differences change a pose, pixels must be within max 2/255 and mean
  0.05/255, and the report says which. Draw p50/p95 ms and peak wasm memory are
  reported.
- **Address gate:** determine whether Painter output depends on the numeric
  value or order of `Entity.address` and `native_focus_entity`. Apply a
  consistent random permutation of addresses over a fixture and diff the
  output. Report the result. W4 remaps addresses only if the output is
  unchanged (C3).
- **Fake painter for W3.** `webapps/arena/static/fake-painter.js` implements the
  same JS-facing module API (C5) by replaying recorded packets with RGBA from
  fixture PNGs. W1 provides the recording script; W3 may write the fake itself.

### C3. Tick stream (W4 produces, W3 consumes)

`GET /api/matches/{id}/ticks` is an SSE stream with gzip, flushed per event,
and owner-checked like today. It replaces `/stream` and `/scene` for browser
mode.

- `meta`: today's `meta()`, plus
  `"protocol":2, "assets":{"manifest":"<base>/assets/manifest-<hash>.json"},
  "cards":[<16 native card rows>]`.
- `ticks` (SSE `id: <match_number>:<last tick>`):
  `{"v":2,"items":[{"snapshot":<Snapshot>,"hud":<payload>}, ...]}`.
  - `snapshot` is exactly the object `match.py` `publish()` currently builds as
    `original` and submits to the presenter: the `BattleFrame` plus
    `runtime_status` and `playback_speed`, with hidden-opponent hands and
    `human` already applied.
  - `hud` is today's `frame` payload *without* `entities`.
- Delivery is **ordered and complete**: every engine tick, exactly once, in
  order, for every subscriber. No latest-only coalescing; the Painter must
  observe every tick.
- Batching: at most one `ticks` event per 33 ms of wall time. At 1× each event
  carries 1–2 items; at 4× about 3.
- Per-match ring buffer of the last 400 ticks. A new subscriber receives the
  ring, then live ticks. `Last-Event-ID` resumes from the ring. A subscriber
  that falls more than 400 ticks behind receives `resync`, then the full ring.
  A new round sends `round` (the client calls `painter_reset`), then ticks.
- `status`, `closed` and `error` behave as today.
- Address remap: only if the W1 gate passes. Then `address`,
  `native_focus_entity` and `native_entity_id` are replaced with opaque
  per-round ids, using the same map everywhere.
- Legacy: `ARENA_SCENE_LEGACY=1` keeps today's `/scene`, `/textures` and
  `/stream` path for A/B testing until phase 4. The default is off.

### C4. Engine host (W5 produces, W4 consumes)

`native-watch`:
- `--preload` loads and verifies the checkpoint before reading stdin, and keeps
  a pristine template: checkpoint JSON, region bytes and allocator/register
  state in memory.
- New op `{"op":"ping"}` returns `{"ready":true,"load_seconds":x,"resets":n}`.
- `reset` verifies hashes **once per process**. Later resets compare file
  identity (device, inode, size, mtime) and skip re-hashing. A new
  `NativeSession` is built from the in-memory template, with no file reads. The
  reply schema is unchanged. Target: reset ≤ 0.3 s locally and ≤ 0.6 s on the
  VPS, down from 1.9 s and 4.0 s.
- **Equivalence:** for 5 seeds × 3 deck pairs × 600 ticks with a fixed action
  script, the reply JSON (excluding `*_seconds` timing fields) of reset #1 in a
  fresh process must be byte-identical to reset #5 in a reused process. The
  first verification failure must still abort, as today.

Python `webapps/arena/engine_pool.py` (written by W4 against this contract):
- Keep one preloaded idle engine, warmed at service start when
  `ARENA_ENGINE_POOL=1`.
- `acquire()` returns a ready engine. `release(engine)` returns a healthy engine
  to the pool, at most 1 idle; otherwise the engine is closed.
- A replacement is spawned in the background, so memory stays within the
  service `MemoryMax=4G`.

### C5. Client modules (W3)

- `static/painter-worker.js`, a dedicated worker:
  1. fetches the manifest and the files for `always` plus the match's 16 cards,
     with progress events;
  2. instantiates the wasm with `WebAssembly.instantiateStreaming`;
  3. fills the VFS, then calls `painter_init`;
  4. consumes the tick stream: `EventSource` in the worker when available,
     otherwise the main thread forwards raw `event.data`;
  5. calls `painter_observe` per item;
  6. drives `painter_step` at 30 Hz;
  7. composites into a transferred `OffscreenCanvas` with WebGL2.
- `static/scene-compositor.js`: `GLPainter`, `SoftwarePainter`,
  `interpolateDraws` and the timeline/interpolation logic moved out of
  `scene-renderer.js` so they run in a worker or on the main thread. Textures
  come from a `textureSource(id) -> {width,height,rgba}` callback, not from
  URLs.
- Fallback: if the worker cannot use OffscreenCanvas WebGL2, it posts
  `{packet, textures:[{id,width,height,buffer}]}` (transferable buffers) to the
  main thread, which composites.
- Adaptive lag: the display lag is the p95 of tick inter-arrival over the last
  10 s plus 50 ms, clamped to 100–300 ms, and passed to
  `painter_set_lag_ticks`. A stall lets the clock run down to the newest tick
  and hold. It must never hold the old frame while ticks are waiting.
- `lobby.js` prefetches the manifest, the wasm and the packs for the two
  selected decks as soon as the decks are chosen, before Play is pressed. Use
  low-priority `fetch` into the HTTP cache, so most of the download overlaps
  with the user choosing.
- `play.js` uses the single `ticks` stream for HUD/status and gets `layout`
  from the worker. The input path (`POST /input`, `input-transport.js`) is
  unchanged. Loading shows engine status and asset progress (bytes / total).
- Diagnostics: `arenaDiagnostics()` keeps its fields and adds `painter`
  (draw ms p50/p95, wasm MB, observe backlog), `lag_ms`, `jitter_ms`,
  `freezes` (count and total ms > 100 ms) and `asset_bytes`.
  `arenaDisplayTrace()` returns `[[wall_ms, shown_native_ms, pending, inflight], ...]`.
  `probe_browser.py` will switch to it.

## 6. Phases

- **P0. Prerequisites** (Opus, needs your OK):
  - commit the deployed, uncommitted Arena changes (`webapps/arena/*` only) to
    `main` as the baseline;
  - create the `arena-speed` branch and the five worktrees with symlinks
    (`.venv`, `engine/analysis`, `models/...`, and `engine/runs/...` for the
    registry checkpoints);
  - accept a VISUAL_QA baseline on `arena-speed`.
- **P1. Build** (W1–W5 in parallel, Grok 4.7). Each agent delivers its branch,
  tests, and a short `RESULTS.md` with measurements against its targets.
- **P2. Integrate** (Opus): merge W5 → W4 → W2 → W1 → W3 into `arena-speed`,
  then:
  - run the full local stack;
  - run the probes locally with network throttling (150 ms RTT, 5 Mbps);
  - run VISUAL_QA `--expect none`;
  - run the wasm parity report;
  - play human matches with mouse and with touch at 390×844;
  - test reset, pause, 4× watch, reconnect mid-match and the Stream view
    fallback.
  - Review rounds with fresh Grok tasks if something fails.
- **P3. Deploy** (Opus, only after your go-ahead):
  - package a new release (with packs and wasm, without `clash-web-scene`);
  - update nginx (the static `assets/` location, `application/wasm`, and
    `script-src 'self' 'wasm-unsafe-eval'` in the CSP);
  - re-run the probes on the public site and keep the previous release for
    rollback.
- **P4. Cleanup:** after a week without problems, delete the legacy server
  scene path (`scene.py` presenter, `/scene`, `/textures`, `/stream`
  coalescing) and update the README, `SCENE_PROTOCOL.md` (v2) and
  `BROWSER_RENDERER_VALIDATION.md`.
- **P5. Conditional: recompiled engine for the six presets.** Do this only if,
  after P3, 4× watch cannot hold ≥ 76 ticks/s on the VPS or reset is still
  > 1 s. It requires the full presentation frame, `validate_target`,
  `resolve_target` and placement parity with `native-watch`. Custom decks stay
  on Unicorn.

## 7. Acceptance (public site, same scenario as the baseline)

| Metric | Baseline | Target |
| --- | ---: | ---: |
| Create to first visible frame, warm cache | 7.7 s | ≤ 2.5 s |
| Create to first visible frame, cold first visit | 8.1 s | ≤ 2.5 s + pack download, with a progress bar |
| Frozen display time (> 100 ms with no game-time advance) | 88–92% | ≤ 1% |
| Asset/texture requests after the first frame | 1,416–1,531 | 0 |
| Shown game time per wall second, every 2 s window, 1× | 0 to 10× (bursts) | 0.95–1.05 |
| Download during play at 1× | 1.48 Mbps + textures | ≤ 200 kbps |
| `native-watch` reset on the VPS | 3.8–4.1 s | ≤ 0.6 s |
| Server RSS (Python + engine + presenter) | 344 + 842 + 780 MB | ≤ 1.3 GB total |
| Card release to unit visible (human) | not measured | ≤ RTT + 250 ms |
| Painter wasm draw p95 | n/a | ≤ 8 ms desktop, ≤ 25 ms at 4× CPU throttle |
| Painter worker memory, 16-card match | n/a | ≤ 450 MB |
| Visual parity | n/a | VISUAL_QA PASS; wasm parity within C2 tolerances |

## 8. Risks and gates

- **Pack size on first visit.** Character atlases are 0.3–8 MB each (PNG). A
  new deck pair may cost 25–40 MB once, then it is cached forever (immutable).
  W2's report decides whether lossless WebP or atlas cropping is worth it.
  Subsetting `ui` and `effects` is mandatory.
- **Worker memory on phones.** Decoded atlases for 16 cards are about 250–350 MB
  RGBA. Mitigations, in order:
  - subset the HUD and effects;
  - decode by priority (own deck, then opponent) with lazy decode as a last
    resort;
  - cap the in-memory pose cache.
  
  If ≤ 450 MB still cannot be met, low-memory devices
  (`navigator.deviceMemory <= 2`) get the Stream view.
- **libm / float differences** between wasm and native can change pose pixels.
  C2 tolerances decide; any mismatch is reported, not hidden.
- **Address exposure.** `SCENE_PROTOCOL.md` says engine addresses must not
  reach clients. The W1 gate decides whether W4 can remap them without visual
  change. If it cannot, Opus asks you before shipping raw addresses.
- **CSP.** WebAssembly needs `'wasm-unsafe-eval'`. Without it the page fails
  closed to the Stream view, never to a blank screen.
- **Ordered delivery** raises server memory per subscriber slightly (a 400-tick
  ring of about 10 KB each, about 4 MB per match). This is acceptable.

## 9. Global constraints for every agent

- Work only in your own worktree and branch. Never write to the main checkout
  `/home/cochon/Documents/mini-project/clash-royale-ai`, to its `engine/runs`,
  or to the VPS. Reading the main checkout is fine.
- **Python:** `/home/cochon/Documents/mini-project/clash-royale-ai/.venv/bin/python`.
- **Cargo:** `--offline` only, with the already-vendored crates. Do not add
  crates that are not in `~/.cargo/registry`.
- **Node:** `/home/cochon/.local/bin/node`.
- **Hardware:** RTX 3050 6 GB, i5-13420H (12 threads), 30 GB RAM, shared with
  other jobs. Keep builds at `-j 6`.
- **Renderer rules:** follow `engine/native-spectator/VISUAL_QA.md` exactly.
  Take a baseline first. Do not edit rules, thresholds, scenarios or fixtures.
  Report the verdict and its limits.
- **Tests:** keep all existing tests passing:
  - `node --test webapps/arena/test_*.cjs`;
  - `python -m unittest` for the Arena modules listed in
    `webapps/arena/README.md`;
  - `cargo test --offline` for the touched crates.
  
  Add tests for every contract item you produce.
- **No deployment, no pushes.** Commit to your branch with clear messages. End
  with `RESULTS.md`: what was done, measurements versus the targets, deviations
  from the contracts, and open problems. Do not claim parity or speed you did
  not measure.
