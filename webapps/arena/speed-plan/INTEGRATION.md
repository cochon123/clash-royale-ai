# P2 integration results (2026-10-24, local)

Branch `arena-speed/integration`: merges of W5 (8f628d0), W4 (b8219ec), W3 (0242411), W2 (d700b8a), W1 (W1 round 2, 66a78ed).
All Arena Python tests (test_ticks, test_assets, test_engine_pool, test_api, test_asgi, test_video, test_scene) and node tests
(compositor 5, input transport 10, jitter clock 8, painter worker 7, scene renderer 29) pass after the merges.

Local stack: `python -m webapps.arena --port 8840` with ARENA_ENGINE_POOL=1, ARENA_WEB_ASSETS_DIR=.local-arena/web-assets
(built by `deploy/arena/build_web_assets.py --painter-wasm ...`, 171 MB on disk), W5 native-watch, headless Chrome 154, loopback network.
Scenario as the baseline: watch banana-hog-1 (hog-mirror) vs banana-1 (log-bait).

| Metric | Public baseline | Local integration |
| --- | ---: | ---: |
| Create to first tick (server, real models, warm pool) | 6.85 s | 0.35-0.47 s |
| Create to first visible frame, cold browser (52 MB packs over loopback) | 8.09 s | 2.12 s |
| Create to first visible frame, warm browser | 7.72 s | 1.12-1.13 s |
| Frozen display (>100 ms) | 33.8-35.4 s of 38.5 s | 0 s of 43.9 s |
| Shown game time per wall time (4 s trace windows) | bursts 0-10x | 0.996 |
| Display lag behind newest tick | n/a | 169-208 ms |
| Tick stream wire bandwidth | 1.48 Mbps + textures | 45 kbps |
| Server tick rate | 20.01 | 20.00 |
| Human mode, 30 play attempts over 30 s | not measured | 7 accepted, 0 rejected, 0 freezes |
| Painter wasm draw p50 / p95 in Chrome worker | n/a | 8.3-10.2 / 15.8-20.4 ms |

Notes and open items
- The per-2 s bucket line of `probe_browser.py analyze()` under-reports pace by ~4%: it takes first/last samples inside
  each bucket and drops the gap between buckets. The trace slope is the correct measure.
- Draw p95 is above the 8 ms plan target; at a 30 Hz worker step it does not cause freezes. Not optimized yet.
- `painter.wasm_mb` diagnostic reports 0 (not wired).
- Worst custom deck pair: 106 MB download / 496 MB decoded atlases (over the 450 MB gate); presets are 25-60 MB / 126-297 MB.
- Address remap stays off (W1 gate: actor BTreeMap ordering depends on addresses).
- Local server needs ARENA_RENDERER_BIN pointing at a clash-web-renderer build; create_match requires it so Stream view stays available.
- Not measured yet: the VPS (reset time, CPU, RSS), real WAN latency, phones.

# P3 deployment (2026-10-24 21:36 UTC)

Release `/opt/clash-arena/releases/20261024-02-arena-speed` (archive sha256
`0030c7f180df38b84a4cc71cebfeeacbcb7892b2e9267d35b3dce67b197a1830`, 1,257 files, all verified).
Previous release `20261024-01-inline-poses` kept; backups `/etc/nginx/snippets/clash-arena.conf.bak-20261024-01`
and `/etc/clash-arena.env.bak-20261024-01`. Rollback: relink `/opt/clash-arena/current`, restore both files,
restart `clash-arena`, reload nginx.

Changes on the VPS: env `ARENA_ENGINE_POOL=1`, `ARENA_WEB_ASSETS_DIR=...`, `ARENA_WARMUP=1` (preloads one engine,
588 MB). Web assets copied world-readable to `/var/www/clash-arena-assets/next` (`current` symlink), served by
nginx under `/clashroyale/assets/` (immutable, `application/wasm`). CSP adds `'wasm-unsafe-eval'`.

Added after P2 at the user's request: the painter decodes every atlas page on the loading screen
(`painter_prewarm`), and the server holds the first round at its start tick until a viewer posts
`control {"op":"ready"}` (90 s fallback). Art download and unpacking therefore happen before the battle starts.

Public-site probes (headless Chrome on the laptop, which was under load average ~30 from other jobs):

| Run | Loading screen until first frame | Frozen display after start | Notes |
| --- | ---: | ---: | --- |
| Watch, first visit (52 MB download) | 9.0 s | 0 s of 37.9 s | 852 ticks at the end, about 19.8 ticks/s shown |
| Watch, returning visit | about 2.5 s | 0.5 s (one 494 ms hitch) of 37.8 s | |
| Human, returning visit | 2.48 s | 0.5 s (one 452 ms hitch) of 28.3 s | 5 human and 6 model plays accepted, 0 rejected |

Service memory with a match running: 1.29 GB (was about 2 GB). VPS load average 0.3-0.5.
The single ~0.5 s hitches were not investigated; the client laptop was heavily loaded during these runs.
