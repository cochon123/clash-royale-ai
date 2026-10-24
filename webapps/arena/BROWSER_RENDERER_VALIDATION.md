# Local browser-renderer validation

Initially validated in the isolated `arena/browser-renderer` worktree on
2026-10-22 at <http://127.0.0.1:8810/>. That tested baseline, commit `8b62365`,
was subsequently integrated into local `main` and deployed to
<https://calgarypermit.ca/clashroyale/> at the user's request. Further animation
and coverage work remains in this feature worktree.

The deployed baseline passed a real signed-session match with mouse and mobile
touch deployment, pause/resume, no browser exceptions and no video requests.
The test match was closed afterward. Its first frame took 8.63 seconds; an
8.08-second sample including new pose textures used 1.20 Mbps. These are single
sample observations, not a capacity or frame-rate guarantee. Release
`20261022-04-browser-8b62365` retains the previous release for rollback. Main's
unrelated working changes were preserved during integration.

## Automated checks

- 118 Python checks: Arena requests, sessions/base prefixes, SSE compression,
  disconnect cleanup, video fallback, scene texture privacy/path checks, input
  validation, resolved placement, and Lab status checks.
- 33 JavaScript checks: bounded input transport, original blending/shadows,
  affine transforms, cache lifecycle, pose interpolation, discrete HUD updates,
  pause, and interpolation during fast playback.
- 28 Rust library checks and 3 native pacing checks passed; 3 pre-existing
  art-sheet checks remain ignored.
- Original fixture compared with the GPU compositor: maximum channel difference
  3/255, mean 0.033/255 across the 540×900 image. The fixture contains 66 commands
  and 44 original textures. Native capture replay mean difference 0.000804/255.

## Actual browser flows

Real matches used the original native engine and saved Banana Hog 1 policy.
Checked mouse placement, touch placement at 390×844, pause/resume, new round,
watch playback speed, a natural model-vs-model result, explicit stream view,
returning to browser view, page restoration, and graphics failure recovery.
Also checked a real signed-session ASGI server under `/clashroyale/`: touch drag
placement succeeded at 390×844 without horizontal overflow, all resources kept
the prefix, and a separate client could not read the match’s textures (404).
The default path requested no `/view` or `/video` endpoints and launched no JPEG
renderer. Explicit stream view created its renderer lazily.

A forced initial HTTP503 on `/scene` retried successfully. An injected expired
texture HTTP410 was discarded without a renderer error; a newer pose loaded and
mouse placement still succeeded. A forced lack of
WebGL2 used the original-art Canvas compositor and still accepted mouse play.
The graphics-error overlay remained visible across subsequent game-state events
and offered the explicit stream link. No browser runtime exceptions occurred.

Latest human-match sample:

| Measurement | Observation |
| --- | --- |
| Start click to first original-art frame | 3.12 s |
| 15.27 s network sample, including state, drawing commands and new PNG poses | 1.75 Mbps |
| Composition/display rate at the end of that sample | 25 FPS |
| Card release to acknowledged placement under simulated 150 ms network latency | 0.391 s |
| Input backlog during rapid pointer movement | At most 1 queued command |

Across the browser samples display rates varied roughly 20–45 FPS while cold
poses loaded. The browser was headless Chrome with `--disable-gpu`; these rates are
measurements of that test environment, not a 60 FPS guarantee for user devices.
The full fixture took 2.9 ms with a synchronous GPU completion; the optimized
Canvas fallback took 12.8–14.2 ms warm for that fixture. Those fixture timings do
not include network or authoritative combat.

A longer model-vs-model test covered 260 game seconds, including tower deaths
and 4× playback, with no scene error or browser exception. After output pacing,
10-second network windows used about 2.2–4.0 Mbps during accelerated playback.
Its final client cache held 4070 textures / 63.4 MB decoded, below the 192 MiB
limit. A repeated model-vs-model match with seed2196547421 reached the same natural
result at tick3601 before and after the review fixes: winner0, crowns[1,0],
accepted plays[34,41], no rejected plays.
Historical production MJPEG used about 19 Mbps, but that was a different host
and match; this is not a controlled cross-host speed comparison.

## Review fixes

Grok 4.7 reviewed the implementation using the 256K context setting. The review
led to preserving every input tick before native animation observation, ordered
promotion of complete browser poses, and a bounded native LRU pose cache with a
30-second grace period and current/previous frame pinning. Expired texture URLs
return owner-scoped HTTP410; the browser discards the stale pending pose and
accepts newer scenes without showing partial objects. Missing current assets
still surface an error.

The interpolation buffer deliberately remains 50 milliseconds of wall time.
At 4× playback the output budget is still 30 updates/s, so scaling that wall
buffer into game time keeps it within the retained pose timeline. A focused
regression checks the midpoint between coalesced fast-playback poses.

## Remaining work

A guaranteed 60 FPS result has not been established. Cold pose downloads and
presentation/combat CPU work still affect perceived smoothness.

Combat, AI and animation-pose selection remain on the server. The browser caches
and composites original poses and interpolates their transforms. Newly seen
animations still require PNG downloads; this is not a complete SC timeline port.

Local bottleneck measurements found native reset around 1.9 s, including a
354 MB checkpoint reload/verification. Cold placement-mask queries took 72–81 ms
and can stall a simulation tick, particularly after tower state changes. These
costs remain relevant on a busy VPS.

The compiled six-deck training engine needs a full frame/action adapter before
Arena can use it accurately. It currently lacks the complete presentation frame,
exact browser placement/target operations and resolved play coordinates expected
by this loop. No combat-engine substitution has been made in this prototype.
