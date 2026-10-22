# Local browser renderer prototype

Branch `arena/browser-renderer` keeps the authoritative original combat engine and
saved policies on the server. The browser composites the original artwork with
WebGL2 and interpolates stable entity transforms at the display refresh rate.
The existing Painter still selects animation poses, effects and drawing order on
the server; this is a presentation adapter, not a complete client animation port.

Run from this worktree, leaving the main checkout's services alone:

```sh
cargo build --offline --manifest-path engine/native-spectator/Cargo.toml --bin clash-web-scene
ARENA_RUNS_DIR="$PWD/.local-arena/runs" CLASH_ENGINE_ROOT="$PWD/engine" \
  PYTHONPATH="$PWD/src" ARENA_WARM_PLACEMENTS=0 \
  .venv/bin/python -m webapps.arena --port 8810
```

Open <http://127.0.0.1:8810/>. The engine checkpoint, recovered graphics and
model weights are required, as in the existing Arena. This isolated worktree
shares those ignored inputs with the main checkout. New match outputs go only
to `.local-arena/runs`; build targets are local to this checkout.

`ARENA_WARM_PLACEMENTS=0` avoids competing with an initial match while testing.
Placement masks still use the existing persistent cache and are computed on a
miss. The six-deck compiled training engine is not wired into Arena yet: it
needs the complete original frame, target validation and placement protocol
before it can preserve gameplay and presentation.

The default path uses `/scene` SSE display lists and immutable per-match PNG
textures. Both SSE feeds negotiate gzip and flush every event immediately.
Presentation and state updates are bounded at 30 per wall-clock second during
fast watch playback; combat still advances at the chosen speed.
`ARENA_SCENE_FPS` can adjust the presentation limit (1–60). Browser drawing uses
the display refresh rate independently. It never starts the JPEG renderer or requests `/view` or `/video`.
The toolbar's **Stream view** explicitly opens the existing image transport for
comparison or compatibility. That path starts a separate JPEG process lazily.
Set `ARENA_BROWSER_SCENE=0` to use the old transport for an entire local server.
Canvas software compositing is available when WebGL2 cannot be created, but is
considerably slower. Missing assets and graphics failures offer the stream view.

Input selection and aiming appear immediately in a separate browser overlay.
The transport keeps one request in flight and one latest unsent aim. A play
carries its own card identity and coordinates; the server validates current
hand, elixir and placement rules atomically. Reset drops stale queued commands.

`arenaDiagnostics()` in the browser console exposes rendering rate, composition
time, cache size, downloaded texture bytes, pending poses and input queue counts.
The simulation stays at 20 ticks/s. Display rate depends on the device, scene,
network and the time needed to download newly encountered animation poses.
The initial asset cache and cold effects may still produce visible delays.

Protocol details: [SCENE_PROTOCOL.md](SCENE_PROTOCOL.md).
Measured results and remaining limits: [BROWSER_RENDERER_VALIDATION.md](BROWSER_RENDERER_VALIDATION.md). No production deployment
has been made from this prototype.

Focused checks:

```sh
node --test webapps/arena/test_input_transport.cjs webapps/arena/test_scene_renderer.cjs
.venv/bin/python -m unittest webapps.arena.test_api webapps.arena.test_asgi \
  webapps.arena.test_video webapps.arena.test_scene engine.rl.test_browser_input \
  engine.rl.test_banana_human engine.rl.test_banana_placement
cargo test --offline --manifest-path engine/native-spectator/Cargo.toml --lib --bin clash-web-scene
```
