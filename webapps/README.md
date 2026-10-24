# Arena and Lab

The public Arena is at **https://calgarypermit.ca/clashroyale/**.
[VPS deployment and operations](../deploy/arena/README.md). The Lab remains local.

Run from `clash-royale-ai/`:

```sh
.venv/bin/python -m webapps
```

- **Arena:** <http://127.0.0.1:8800/> — play against a saved model or watch two
  models. Choose both decks, use six presets, or build and save a custom deck
  from the recovered card catalog. Custom decks are saved in this browser.
- **Lab:** <http://127.0.0.1:8801/> — model inventory and evidence, RL runs and
  curves, live process/service observations, engine validation, datasets,
  reports and documentation. [Lab details and limits](lab/README.md).

Ctrl+C stops both servers. They can also run independently:

```sh
.venv/bin/python -m webapps.arena --port 8800
.venv/bin/python -m webapps.lab --port 8801
```

This machine also runs Arena as a user service. After updating Arena Python
code or rebuilding its renderer, restart it so both use the same frame format:

```sh
systemctl --user restart clash-arena.service
```

To open the websites on a phone on the same trusted LAN, use
`.venv/bin/python -m webapps --host 0.0.0.0`, then open
`http://<this-computer-LAN-IP>:8800/` or `:8801/` on the phone. These local tools
have no sign-in; leave the default loopback binding when LAN access is unneeded.
Layouts and touch controls work in a phone browser. A native/installable phone
app is future work.

## Match graphics and controls

Arena uses **the same original-asset Rust Painter as `rl.banana_play`**:
arena and tower sprites, character animation, projectiles, effects, health
bars, card portraits and wood-panel HUD. In the browser renderer prototype,
the Painter exports compact drawing commands and immutable original sprites.
WebGL2 composites and interpolates them locally. A separate immediate input
overlay uses the original card rectangles and camera geometry. The toolbar
retains an explicit JPEG/MJPEG stream view, created lazily when requested.
[Local prototype setup and limits](arena/README.md).

Select a card by tapping it or pressing 1–4, then tap/click the arena; dragging a
card onto the arena also plays it. Right click or Escape cancels. Space pauses,
R starts another match, Q ends it, and the toolbar provides pause, restart, full screen,
and End. Watch mode shows both hands and offers playback speeds. Human mode
hides the opponent’s hand. Gameplay stays authoritative in the reference
engine; models and checkpoints are never trained or modified by a match.

AI and human plays use the original native placement resolver before submission.
The accepted action's `result.resolved_target` supplies policy history and play
markers; `request` in the action log retains the original aim for diagnostics.
This matters when a troop aimed across the river is moved back to its territory:
the requested aim must never be presented to the policies as its deployment.
Match manifests identify this contract as `native-resolved-target-v1`. Rebuild
`native-watch` and restart Arena together when updating this contract.

The **very fast, recompiled six-deck engine is for training**. Every browser
match uses the original Unicorn reference engine, including custom decks.
An RL specialist may play less well outside its training deck. Presets do not
change the engine used by Arena. Some recovered cards can still encounter
native or visual coverage limits; errors are shown in the match view.

## Inputs and building

The existing project environment needs PyTorch and Pillow. Original extracted
assets, the reference checkpoint and model weights remain local ignored
artifacts under `engine/analysis/`, `engine/runs/`, and `models/`; a code-only
clone does not include them. Build the engine and scene presenter from the repo root (the JPEG binary is optional for the stream fallback):

```sh
cargo build --release --offline --manifest-path engine/native-engine/Cargo.toml --bin native-watch
cargo build --offline --manifest-path engine/native-spectator/Cargo.toml --bin clash-web-scene
TURBOJPEG_STATIC=1 cargo build --offline --manifest-path engine/native-spectator/Cargo.toml --features web-jpeg --bin clash-web-renderer
```

Required original assets: `engine/analysis/native-spectator/assets.json` and
the files it references. [Extraction instructions](../engine/native-spectator/README.md).
The checkpoint is
`engine/analysis/native-pipeline/oracle-export-01/stage01-attempt001/state/`.
Arena’s health endpoint reports missing inputs before starting a match.

Playable models are curated in [models/registry.json](../models/registry.json).
Add an engine-compatible checkpoint, a unique ID, display name and optional
training-deck ID there. Replay-only policies and auxiliary predictors are
automatically discovered by Lab; they need an engine adapter before becoming
playable. Missing checkpoints remain visible but cannot be selected.

## Small API, reusable by a future phone client

| Endpoint | Purpose |
| --- | --- |
| `GET /api/health` | Engine, assets and renderer readiness |
| `GET /api/catalog` | Playable models, six deck presets and recovered cards |
| `GET /api/matches` | Active matches |
| `POST /api/matches` | Start a match with two controllers and decks |
| `GET /api/matches/{id}` | Match status and completed results |
| `GET /api/matches/{id}/stream` | SSE metadata, game state and status |
| `GET /api/matches/{id}/scene` | Original-sprite display lists over SSE |
| `GET /api/matches/{id}/textures/{id}.png` | Cached, owner-scoped original texture |
| `GET /api/matches/{id}/view?after=N` | Next original-asset JPEG; sequence and input layout in response headers |
| `GET /api/matches/{id}/video` | Persistent MJPEG stream of the latest original-asset frames |
| `POST /api/matches/{id}/input` | Human select/aim/play/cancel commands |
| `POST /api/matches/{id}/control` | Pause, reset, watch speed |
| `POST /api/matches/{id}/close` | Stop the match and its engine/renderer |

For example, `POST /api/matches` with:

```json
{"sides":[{"controller":"human","deck":"hog-mirror"},{"controller":"banana-hog-1","deck":"log-bait"}],"seed":77}
```

Side 0 is blue/bottom. Decks may be preset IDs or arrays of eight unique card
IDs from `/api/catalog`. Replace `human` with a registry model ID to watch two
models. Follow the returned ID at `/play.html?id=<id>`.

At most three matches run by default; `--max-matches` changes that limit. Each
owns an engine and renderer process. Unwatched matches close after two minutes;
End releases one immediately. Match manifests, actions, results and viewer
diagnostics are under `engine/runs/arena/`. Placement-mask warming runs in a
separate reference process; it is a cache optimization, not an RL run.

## Checks

```sh
.venv/bin/python -m unittest webapps.arena.test_api webapps.common.test_httpkit webapps.lab.test_lab
node --check webapps/arena/static/lobby.js
node --check webapps/arena/static/play.js
node --check webapps/lab/static/app.js
cargo test --offline --manifest-path engine/native-spectator/Cargo.toml --lib
```
