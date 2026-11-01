# Arena smoothness pass (2026-11-01)

Players said live battles on <https://calgarypermit.ca/clashroyale/> felt choppy and slow.
The deployed release is `20261024-02-arena-speed` (this branch at fc9c3ab plus the
uncommitted texture-cache fix in `scene.rs` / `scene-compositor.js`).

Measured with [`probe_smooth.py`](probe_smooth.py) against the live server. It runs a
headed Chrome on the laptop's GPU and instruments the painter worker and the compositor
before their first line runs. With `--wasm` and `--static`, the browser gets local client
files while the server stays the live one. Raw summaries are in
[`probe-smooth-2026-11-01.json`](probe-smooth-2026-11-01.json).

## What was wrong

1. **The painter step starved the display.** One worker both stepped the wasm Painter
   (30 Hz `setInterval`) and drew the canvas. A step took 16–18 ms at p50 and 28–37 ms at
   p95, then 4–12 ms of texture uploads. In busy fights the step timer ran back to back,
   and no `requestAnimationFrame` ran for up to 850 ms. 12–28% of display frames missed
   vsync. On a phone-class CPU the longest gap was 4.1 s.
2. **SipHash was the largest single cost** (27% of worker busy time). Every sprite on
   every frame (about 150 draws) was hashed twice, in full, to find its texture id.
3. **Shown time was placed by arrival.** The compositor anchored each painter frame at the
   time it arrived, so a slow step moved the picture. 22–30% of frames were more than
   4 ms off the wall clock.
4. **The display clock froze and never caught up.** It stopped hard at its lag target when
   a tick was late, then played on at 1x. Every stall added delay for the rest of the
   match. One cold run showed 24.6 s of game in 28.1 s (0.87x). In human mode that delay
   is input latency.
5. **Loading.** In real sessions from the nginx log, first visits took 25–38 s and two
   phones never got in. A first visit downloads 40 MB of packs. The lobby prefetch never
   read its response bodies, so it left partial cache entries that were downloaded again
   (206s in the log).
6. **The lobby.** It drew about 120 full-size card PNGs (960 px, about 140 KB each) on
   every visit with `Cache-Control: no-store`. Python served them. A burst past uvicorn's
   `--limit-concurrency 64` returned 503s (78 in one second on Nov 1).
7. **"End it first".** Leaving a battle does not end it, and the server allows one match
   per browser. One player got 22 refusals (429) in 4 minutes.

Not a server problem: on the VPS, through nginx, ticks arrive every 50 ms
(p95 55 ms, max 98 ms, 20.02/s). Over the internet to the laptop the p95 gap was 166 ms and
the max 478 ms. TCP showed RTT 129±90 ms (min 55) and almost no retransmits. That is
path jitter, so the adaptive 200–250 ms lag is needed. What changed is how the
client absorbs it.

## What changed

Painter (Rust, `engine/native-spectator`, `engine/native-ui-viewer`):
- `TextureCache::memo_id`: a buffer whose address, size and XXH64 digest are unchanged
  keeps its id, so SipHash runs only for new content. Ids are byte-identical to before.
- Batch eviction: once the cache is full, it evicts down to 15/16 of the budget, so the
  full sort no longer runs for every new pose.
- `SceneDisplayClock` rate control:
  - up to 25% slower when the display is closer to the newest tick than its lag;
  - up to 10% faster when it is up to 200 ms behind;
  - up to 2x after a longer stall, and a jump past 5 s.
  It never passes one tick before the newest snapshot and never moves back.
- Parsed fonts are cached. Each new string re-parsed its font, 2 MB for SGB.
- `build-wasm.sh`: fat LTO, one codegen unit.

Client (`webapps/arena/static`):
- `compositor-worker.js`: the canvas has its own worker. The painter worker sends frames over a
  `MessagePort` with transferred pixels, so painting is never blocked by a painter step.
- Compositor:
  - draws only on display frames;
  - places frames by the painter's step time;
  - adapts its delay to the measured p95;
  - runs its own clock, which leans at most 10% toward the newest frame and never moves back;
  - keeps frames by time window instead of the last three;
  - checks `gl.getError()` once per frame instead of once per pose;
  - releases poses evicted while still on screen once their frame leaves.
- The main-thread fallback now works with the real wasm: after the first frame, reused
  poses were reported missing.
- Boot compiles the wasm while packs download and loads each pack as it arrives.
- `jitter-clock.js` mirrors the new clock and no longer collapses the lag when the stream is
  quiet.
- Lobby:
  - the prefetch reads its bodies, 3 low-priority streams, painter and always-files first;
  - it shows 240 px WebP thumbnails (`deploy/arena/build_card_thumbs.py`, 19.4 MB to 1.9 MB);
  - starting a match first ends this browser's previous one.
- Human mode: a ring at the landing tile of an accepted play. It bridges the
  display lag before the unit appears.

Server and deploy:
- Static files: `no-cache` with ETag and 304, instead of `no-store`. Card art and thumbnails
  get `max-age=86400`.
- Catalog: cards carry a `thumb` path.
- `clash-arena.service`: `--limit-concurrency 256`.

## Results (live server, laptop GPU, 30 s watch matches)

| | Deployed | New |
| --- | ---: | ---: |
| Display frame interval p95 | 40–71 ms | 17.0 ms |
| Longest display gap | 1.35–2.84 s | 0.11 s |
| Frames that missed vsync | 12–28% | 0.2–0.7% |
| Shown-time error p95 | 9.8–13.8 ms | 1.9–2.2 ms |
| Frames off by > 4 ms | 22–30% | 0.8–3.1% |
| Painter step p50 / p95 | 16–18 / 28–37 ms | 7–10 / 12–24 ms |
| Painter worker busy | 64–78% (shared with drawing) | 30–42% (drawing is separate) |
| Pace (game s per wall s) | 0.87–1.00 | 1.00 |
| Assets complete to first frame (cold) | 2.5 s | 0.9 s |

Phone-class CPU (`--slow-cpu 2 --slow-quota 80`: the whole browser on two E-cores at 80%):

| | Deployed | New (two runs) |
| --- | ---: | ---: |
| Display frame interval p95 | 112 ms | 31–33 ms |
| Longest display gap | 4.09 s | 73–89 ms |
| Frames that missed vsync | 24% | 5.8–6.9% |
| Frames off by > 8 ms | 12.7% | 2.3–7.9% |

DevTools CPU throttling does not reach dedicated workers: their step times did not move
at "4x". Use `--slow-cpu` instead.

Human mode, rematch, and visuals:
- Human mode: 14 inputs sent, 14 completed, 0 failed.
- Rematch mid-match: the new round restarts at 5.0 s game time and plays on with no errors.
- `visual_check.py run --expect none` against a baseline of the unmodified painter:
  PASS, 0 new, 0 worsened, 0 fixed, 23 persisting findings, no changed frames in any
  scenario.

Tests:
- `cargo test --lib`: native-spectator 54, native-ui-viewer 1.
- `node --test webapps/arena/test_*.cjs`: 68.
- Arena pytest: 92.
- `python -m unittest webapps.arena.test_asgi`: 6. The ASGI tests are skipped under pytest
  because of the relative import.

## Not done

- **WebP atlases** would cut the always-loaded PNGs by about 29% (24.4 MB to 17.3 MB on the
  six largest). The wasm needs the `image` crate's WebP decoder, which is not in the offline
  cargo cache.
- **Atlas decoding during download.** `painter_prewarm` still decodes every atlas after the
  last pack arrives: 0.5–1.5 s on the laptop, more on phones.
- **SIMD wasm** was about 8% faster (step p50 10.9 to 10.0 ms). It needs a second build and
  feature detection, because browsers without wasm SIMD would fail to load the painter.
- **Server tick stall.** One cold run saw a 3.6 s gap in ticks. Ticks over the VPS's own
  nginx were clean, so it was not reproduced.
