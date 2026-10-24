# W3 results — client painter path

Branch `arena-speed/client`. Local only: headless Chrome against `speed-plan/dev_harness.py`. These numbers are not the public site, and they are not a wasm Painter.

## What landed

Contract C5, against a fake C2 painter and a synthetic C3 tick stream:

- `static/scene-compositor.js` holds `GLPainter`, `SoftwarePainter`, `interpolateDraws`, and the timeline. Textures come from `textureSource(id) -> {width, height, rgba}`. A pose is uploaded and painted in the same turn. `pending_frames` stays 0. `scene-renderer.js` still serves the gallery and the legacy `/stream` path, and re-exports the moved painters.
- `static/painter-worker.js` loads the C1 manifest subset, observes every tick before it steps, steps at 30 Hz, and paints on a transferred `OffscreenCanvas` (WebGL2). If that probe fails, it posts transferable RGBA to a main-thread compositor. The canvas is transferred only after protocol 2, so a legacy match can still use the visible canvas.
- Adaptive lag is the p95 inter-arrival over 10 s plus 50 ms, clamped to 100–300 ms, passed through `painter_set_lag_ticks`. A stall sets the applied lag to 0. Restoring lag cannot rewind the tick already shown.
- `play.js` uses one `ticks` stream. Layout comes from the worker. `POST /input` and `input-transport.js` are unchanged. The loading overlay shows engine status and asset bytes. `lobby.js` prefetches the manifest, wasm, and packs for the two selected decks with a low-priority fetch.
- `arenaDiagnostics()` keeps the old fields and adds `painter`, `lag_ms`, `jitter_ms`, `freezes`, and `asset_bytes`. `arenaDisplayTrace()` returns `[[wall_ms, shown_native_ms, pending, inflight], ...]`. `probe_browser.py` uses that trace when it exists and keeps the old scene-renderer hook otherwise.
- `static/fake-painter.js` is the C2 stand-in. `speed-plan/dev_harness.py` replays synthetic tick JSONL as C3 SSE.

Node: `node --test webapps/arena/test_*.cjs` — 59 passed, including the existing scene-renderer and input-transport assertions. Python arena tests and `cargo test` were not re-run; those modules were not changed. Native VISUAL_QA was not re-run: no native renderer, fixture, rule, or threshold changed.

## Measured (headless Chrome 154, 150 ms latency, 5 Mbps)

Harness: `http://127.0.0.1:8877/play.html?id=demo&painter=fake&recording=/fixtures/recording.json`. One 4×4 texture. Probe: `speed-plan/probe_local.py`. Raw reports: `probe-local.json` (40 ms extra jitter), `probe-steady.json` (0 jitter), `probe-cpu4.json` (0 jitter, CPU rate 4).

The last `game_per_wall` bucket in each run is a partial window (the span is not a multiple of 2 s). It is listed, and it is not compared to the 0.95–1.05 band.

| | 40 ms added jitter, CPU 1, 14 s | 0 jitter, CPU 1, 14 s | 0 jitter, CPU 4, 10 s |
| --- | ---: | ---: | ---: |
| First frame | 1.177 s | 1.173 s | 1.214 s |
| Requests after ready | 0 of 9 | 0 of 9 | 0 of 9 |
| Freeze ratio (>100 ms, shown time stuck) | 0 (0 ms / 16.897 s) | 0 (0 ms / 16.893 s) | 0 (0 ms / 12.874 s) |
| Trace cadence p50 / p95 | 33 / 66 ms | 33 / 35 ms | 33 / 35 ms |
| Complete 2 s windows, game s per wall s | 0.683–0.725 | 0.952, 0.988, 0.969, 0.988, 0.985, 0.970, 0.988, 0.969 | 0.939, 0.990, 0.969, 0.988, 0.984, 0.970 |
| Partial tail window | 0.328 | 0.444 | 0.429 |
| `lag_ms` / `jitter_ms` | 138 / 88 | 101.1 / 51.1 | 102.1 / 52.1 |
| `painter.draw_ms` p50 / p95 | 0.2 / 0.3 | 0.2 / 0.3 | 0.2 / 0.3 |
| `observe_backlog` | 0 | 0 | 0 |
| Trace pending max | 5 | 4 | 4 |
| `asset_bytes` | 44 | 44 | 44 |
| `wasm_mb` | 0 | 0 | 0 |

The 40 ms run is slower than 1× because the harness sleeps `1/20 + (adler32(tick) % 1000) / 1000 * 0.040` seconds. That adds 0–40 ms on top of the 50 ms tick, so the producer itself is about 0.7×. The picture still advanced every sample; it did not freeze and then jump.

On the steady 20 Hz stream, every complete 2 s window was inside 0.95–1.05. Under 4× CPU throttle the first complete window was 0.939 and the later ones were inside the band. Draw samples stayed at 0.3 ms p95 because this is a fake step plus one 4×4 upload, not rasterization of real atlases.

## Targets from PLAN §7

| Metric | Target | This workstream |
| --- | --- | --- |
| Warm create → first frame | ≤ 2.5 s | 1.17 s here, on a 44-byte fake pack. Not a public-site or real-pack measurement. |
| Cold create → first frame | ≤ 2.5 s + pack download, with progress | Progress UI is in place. Real pack download was not measured. |
| Frozen display | ≤ 1% (baseline 88–92%) | 0% on these three local runs. |
| Asset requests after first frame | 0 (baseline 1,416–1,531) | 0 on these runs. |
| Game time per wall second, every full 2 s window, 1× | 0.95–1.05 | Met on the steady local stream. Not met when the harness itself emits slower than 20 Hz. |
| Download during play | ≤ 200 kbps | Not metered. No new request opened after ready; the tick SSE was already open. |
| Painter wasm draw p95 | ≤ 8 ms desktop, ≤ 25 ms at 4× CPU | Not measured. Fake step p95 was 0.3 ms at both CPU rates. |
| Painter worker memory | ≤ 450 MB | Not measured. |
| Reset, server RSS, visual / wasm parity | other workstreams | Not run. |

## Contract deviations

- `SceneDisplayClock` starts at `latest - lag` when the first step already has a backlog wider than the lag. The Rust clock in `engine/native-spectator/src/clock.rs` starts at the oldest tick. A burst observed while assets load would otherwise stay on screen for the whole backlog. `painter_set_lag_ticks` still receives the adaptive lag, and restoring lag does not rewind.
- `arenaDiagnostics().painter` also includes `display_tick` and `error`.
- The dev harness does not gzip the tick stream. C3 compression belongs to W4.
- The worker asks `painter.texture` for any draw id the compositor does not already hold, including when `packet.textures` omitted it. C2 still lists new textures on the packet; this is a second chance before the pose is rejected.

## Open problems

- W1's real wasm module is not wired up. `instantiateStreaming` is implemented and untested against that module.
- If W1's clock starts at the oldest tick, a full 400-tick ring can leave a delay this client will not snap except by the lag already passed to `painter_set_lag_ticks`.
- Worker RSS, wasm draw time, phone memory, and VISUAL_QA / wasm pixel parity are not known.
- The legacy `?view=stream` path was not opened in a browser against a real arena server in this session. Its node tests still pass.
- Which side owned `EventSource` during the probes (worker vs main-thread forward) was not recorded. Ticks did arrive, and observe ran before paint.
