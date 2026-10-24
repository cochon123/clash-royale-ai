# W4 results — Arena tick stream

Branch `arena-speed/server`. Python side of contracts C3 and C4, plus the local static routes for C1.

## What landed

- `GET /api/matches/{id}/ticks` is an owner-checked, gzip SSE stream. `meta` is today's meta plus `protocol: 2`, the assets manifest URL, and the 16 native card rows. Each `ticks` event is `{"v":2,"items":[{"snapshot","hud"}]}` with SSE id `match_number:tick`. `snapshot` is `publish()`'s `original`. `hud` is that frame payload without `entities`.
- Delivery is ordered and complete for every subscriber. A ring keeps the last 400 ticks. A new subscriber gets the ring, then live ticks. `Last-Event-ID` resumes after the last record with that id. A cursor that has fallen out of the ring gets `resync`, then the ring. A new round emits `round` (no leading `round` for a subscriber whose painter has not seen a tick yet). `status`, `closed`, and `error` follow the existing stream. At most one `ticks` event goes out per 33 ms of wall time. The first event can carry the whole queued ring; the 33 ms cap is on event rate, not on items per event.
- The default match path builds a `LazyStreamRenderer`. It stores snapshots and starts `clash-web-renderer` only when `/view` or `/video` asks for an image. `ARENA_SCENE_LEGACY=1` restores today's scene presenter (`ARENA_BROWSER_SCENE=1`) or the eager JPEG renderer (`ARENA_BROWSER_SCENE=0`). With the flag off, `/scene` and `/textures` answer 503 and say the tick stream replaced the presenter.
- `GET` and `HEAD /assets/{path}` serve `ARENA_WEB_ASSETS_DIR`. Content-addressed files and hashed manifests are `public, max-age=31536000, immutable`. `manifest.json` is `public, max-age=60`. MIME includes `application/wasm`. A sibling `.br` or `.gz` is chosen from `Accept-Encoding` (`br`, then `gzip`, then identity), with `Vary: Accept-Encoding` and `If-None-Match` → 304. Paths that escape the root, including symlinks, are 404.
- `ARENA_REMAP_ADDRESSES` defaults to off. When it is `1`, streamed snapshots get opaque per-round ids. The JPEG/scene presenter still receives the raw addresses.
- `webapps/arena/engine_pool.py` keeps one idle engine when `ARENA_ENGINE_POOL=1` (default off) and warms it from `server.warmup`. `--preload` and `{"op":"ping"}` are used only when `native-watch --help` advertises both. This machine's `native-watch` has neither, so the pool spawns the same command as today's `JsonlEngine` and does not ping. `policy_cache.py` loads each policy file once per process and still builds a fresh module and RNG seed per match. The engine sha256 is hashed once per path per process.

## Measurements (this machine, 2026-10-24)

Script: `webapps/arena/speed-plan/measure_first_tick.py`. Worktree runs dir `.local-arena/runs`. `ARENA_ENGINE_POOL=0`. Controllers `random` vs `random` (hog-mirror vs the next preset), seed 1, so no policy checkpoint is loaded. `_load` still imports torch, and that cost is inside the create-to-first-tick number. The first published tick is 100, because the loop publishes after the 100-tick startup step. `native-watch` is the main checkout release binary. Each figure is one run.

| Path | Create → first tick | RSS at that tick | RSS 5 s later |
| --- | ---: | --- | --- |
| Default (`ARENA_SCENE_LEGACY=0`, `LazyStreamRenderer`) | 3.564 s | walker missed the engine process | walker missed it |
| Default, repeat | 4.319 s | python 516896 KiB, native-watch 841304 KiB | walker missed the engine |
| Default, repeat with the fixed walker | 3.623 s | python 517008 KiB, native-watch 841336 KiB | python 518572 KiB, native-watch 841336 KiB |
| Legacy (`ARENA_SCENE_LEGACY=1`, `NativeSceneRenderer`) | 3.639 s | python 519292 KiB, native-watch 841348 KiB, clash-web-scene 570260 KiB | python 520536 KiB, native-watch 841348 KiB, clash-web-scene 863176 KiB |

The legacy row used `clash-web-scene` from the `browser-renderer` worktree (`--help` accepts `--pose-cache`). The main checkout binary exits with `unknown argument --pose-cache`, and that attempt published no tick.

Default-path Python + engine at the clean sample is 517008 + 841336 = 1,358,344 KiB. The legacy presenter adds 570260 KiB at the first tick and 863176 KiB five seconds later. The default samples and the legacy sample sit in the same 3.6–4.3 s band, so starting the presenter does not move create-to-first-tick on this machine. The time is the engine process, its checkpoint load, reset, and the startup step.

In-process `TickLog` only (20 empty ticks, `batch_seconds=0`, no engine): p50 0.014 ms, p95 0.086 ms, max 0.476 ms from `publish` to the subscriber.

### Against the plan targets

| Plan target | This measurement |
| --- | --- |
| Create to first visible frame ≤ 2.5 s (public site, warm browser) | Not measured. Local create → first tick is 3.564 s, 4.319 s, and 3.623 s on the default path, and 3.639 s with the legacy presenter. |
| `native-watch` reset ≤ 0.3 s locally | Not isolated. The 3.6–4.3 s includes process start, checkpoint load, reset, and 100 startup ticks. W5's `--preload` / `ping` are absent from this binary. |
| Server RSS Python + engine + presenter ≤ 1.3 GB | Default path has no presenter. Python + engine was 1,358,344 KiB at the clean sample. Legacy adds the scene process above. |

## Tests

`python -m unittest` of `webapps.arena.test_ticks`, `test_assets`, `test_engine_pool`, `test_api`, `test_asgi`, `test_video`, `test_scene`, `webapps.common.test_stream_compression`, `engine.rl.test_browser_input`, `engine.rl.test_banana_human`, `engine.rl.test_banana_placement`: 120 tests, 1 skipped, OK.

`node --test webapps/arena/test_input_transport.cjs webapps/arena/test_scene_renderer.cjs`: 39 passed.

Covered: two subscribers including a slow one, resume, resync, a live gap, a new round, 4× batching, hidden opponent hand, remap on/off and round reset, pool acquire/release and the no-ping degrade, legacy flag, lazy JPEG start, asset traversal, brotli, wasm, HEAD, and 304.

## Contract deviations

- Address remap uses two tables that share one counter. `address` and `native_focus_entity` share one. `native_entity_id` and `native_focus_entity_id` share the other. One integer table would alias a small entity id with an equal guest pointer. C3 said "the same map everywhere"; the two namespaces stay consistent for the comparisons the painter actually does. The flag stays off until W1's gate.
- A second publish of the same tick is appended. Resume skips to the record after the last one with that SSE id, so earlier HUD copies of that tick are not replayed.
- nginx and the CSP `wasm-unsafe-eval` change are W2's. This process serves `/assets/` itself and does not send a CSP.
- `create_match` still requires a renderer binary on disk so the existing missing-binary check stays. The default path requires the JPEG binary and does not spawn it.

## Open

- W1 has not published the address gate, so `ARENA_REMAP_ADDRESSES` stays off.
- This `native-watch` has no `--preload` and no `ping`. The pool degrades to a plain spawn. Reset time is unchanged.
- Create-to-first-tick is still the engine startup. Removing the presenter does not shorten it.
- The main checkout `clash-web-scene` does not accept `--pose-cache`, which this branch's legacy presenter passes.
