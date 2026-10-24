"""Arena website: pick a model and decks, then play or watch in the browser.

Control routes return JSON; original sprites are composited in the browser.
The JPEG spectator remains an explicit compatibility view.
A future phone app can reuse the same endpoints:

GET  /api/health                 server and engine readiness
GET  /api/catalog                playable models, preset decks and cards
GET  /api/matches                active matches
POST /api/matches                start one: {"mode": "human"|"watch", "sides": [...], "seed"?}
GET  /api/matches/{id}           summary of one match
GET  /api/matches/{id}/stream    server-sent events: meta, rows, frame, status, closed
GET  /api/matches/{id}/ticks     protocol-2 tick SSE: meta, ticks, round, resync, status, closed, error
GET  /api/matches/{id}/scene     legacy original-sprite display-list SSE (ARENA_SCENE_LEGACY=1)
GET  /api/matches/{id}/textures/{id}.png   immutable original sprite
GET  /api/matches/{id}/view      rendered original spectator frame (JPEG)
GET  /api/matches/{id}/video     multipart/x-mixed-replace JPEG stream of the match
POST /api/matches/{id}/input     human command: select/aim/play/cancel/pause/reset
POST /api/matches/{id}/control   watch controls: pause/reset/speed
POST /api/matches/{id}/close     end the match and free its engine
"""
from __future__ import annotations

import math
import os
import secrets
import threading
import time

from webapps.common import catalog, paths
from webapps.common.httpkit import App, ByteStream, EventStream, HTTPError

from .assets import register_routes as register_asset_routes
from .match import MatchManager
from .placement import Warmer
from .gallery import register_routes as register_gallery_routes
from .renderer import ASSETS, BINARY, register_routes
from .scene import BINARY as SCENE_BINARY, browser_scene_enabled, register_routes as register_scene_routes, scene_legacy_enabled

HERE = paths.REPO / "webapps" / "arena"
app = App("arena")
manager = MatchManager()
warmer = Warmer()
app.mount("/cards", paths.CARD_ART)
app.mount("/", HERE / "static")
register_gallery_routes(app, HERE / "static")
register_asset_routes(app)
_warmup_lock = threading.Lock()
_warming = False


def _warm_imports():
    """Import torch and the engine adapters once, so the first match starts faster."""
    try:
        from . import match  # noqa: F401
        import rl.banana_watch  # noqa: F401
    except Exception as exc:  # reported again, with context, when a match starts
        print(f"arena: background import failed: {exc}", flush=True)


def warmup():
    """Start optional model imports and placement warming when serving the app."""
    global _warming
    with _warmup_lock:
        if _warming:
            return
        _warming = True
        if os.environ.get("ARENA_WARM_PLACEMENTS", "1") == "1":
            for preset in catalog.presets():
                warmer.enqueue(preset["cards"], preset["cards"])
        if os.environ.get("ARENA_ENGINE_POOL", "0") == "1":
            from .engine_pool import get_pool
            get_pool().warm()
        threading.Thread(target=_warm_imports, daemon=True).start()


def _checkpoint_available():
    return (paths.ENGINE_CHECKPOINT / "checkpoint.json").is_file()


def _renderer_binary():
    """Binary required before a match starts.

    The default tick path still requires the JPEG renderer so an explicit
    Stream view can start later. It does not start that process at create time.
    The scene presenter is required only when legacy mode is on.
    """
    if scene_legacy_enabled() and browser_scene_enabled():
        return SCENE_BINARY
    return BINARY


def _renderer_assets():
    return ASSETS / "assets.json"


def _object_body(request):
    body = request.json()
    if not isinstance(body, dict):
        raise HTTPError(400, "JSON body must be an object")
    return body


@app.route("GET", "/api/health")
def health(_request):
    return {"ok": True, "engine_built": paths.NATIVE_WATCH.is_file(),
            "engine_checkpoint": _checkpoint_available(),
            "renderer_built": _renderer_binary().is_file(),
            "scene_built": SCENE_BINARY.is_file(), "stream_renderer_built": BINARY.is_file(),
            "renderer_assets": _renderer_assets().is_file(),
            "tick_stream": True, "scene_legacy": scene_legacy_enabled(),
            "matches": len(manager.list()), "max_matches": manager.max_matches,
            "placement_warmer": warmer.status}


@app.route("GET", "/api/catalog")
def get_catalog(_request):
    return {"models": catalog.engine_models(), "presets": catalog.presets(),
            "public": os.environ.get("ARENA_PUBLIC", "0") == "1",
            "cards": list(catalog.cards().values()),
            "rendering": {
                "default": "ticks" if not scene_legacy_enabled() else ("browser" if browser_scene_enabled() else "stream"),
                "scene_available": SCENE_BINARY.is_file(), "stream_available": BINARY.is_file()},
            "notes": {
                "presets": "These six presets are supported by the fast training engine. "
                           "Browser matches use the original engine.",
                "custom": "Custom 8-card decks run in the original engine. Specialist models may play "
                          "less well outside their training deck.",
            }}


def _side(spec: dict, index: int, human_allowed: bool) -> dict:
    if not isinstance(spec, dict):
        raise HTTPError(400, f"side {index} must be an object")
    controller = spec.get("controller")
    if not isinstance(controller, str):
        raise HTTPError(400, f"side {index} controller must be a model id or human")
    if controller == "human":
        if not human_allowed:
            raise HTTPError(400, "only the first (bottom) side can be human")
        label = spec.get("label")
        if label is not None and not isinstance(label, str):
            raise HTTPError(400, f"side {index} label must be a string")
        label = label or "You"
    else:
        model = catalog.engine_model(controller)
        if model is None:
            raise HTTPError(400, f"unknown model {controller!r}")
        if not model["available"]:
            raise HTTPError(400, f"{model['name']} checkpoint is missing: {model['checkpoint']}")
        label = model["name"]
    deck = spec.get("deck")
    if isinstance(deck, str):
        preset = catalog.preset(deck)
        if preset is None:
            raise HTTPError(400, f"unknown preset deck {deck!r}")
        deck = preset["cards"]
    try:
        deck = catalog.validate_deck(deck)
    except ValueError as exc:
        raise HTTPError(400, f"side {index}: {exc}") from exc
    return {"controller": controller, "deck": deck, "label": label}


@app.route("POST", "/api/matches")
def create_match(request):
    body = _object_body(request)
    sides = body.get("sides")
    if not isinstance(sides, list) or len(sides) != 2:
        raise HTTPError(400, "sides must list the bottom (blue) side then the top (red) side")
    config = {"sides": [_side(sides[0], 0, True), _side(sides[1], 1, False)],
              "owner": request.session_id}
    mode = body.get("mode")
    if mode is not None:
        if mode not in ("human", "watch"):
            raise HTTPError(400, "mode must be human or watch")
        if (mode == "human") != (config["sides"][0]["controller"] == "human"):
            raise HTTPError(400, "mode disagrees with the bottom side controller")
    seed = body.get("seed")
    if seed in (None, ""):
        seed = secrets.randbits(32)
    try:
        if type(seed) is not int and not (isinstance(seed, str) and seed.isascii() and seed.isdecimal()):
            raise ValueError("not an integer")
        seed = int(seed)
    except (TypeError, ValueError) as exc:
        raise HTTPError(400, "seed must be an integer") from exc
    if not 0 <= seed < 2**32:
        raise HTTPError(400, "seed must be in 0..4294967295")
    config["seed"] = seed
    if not paths.NATIVE_WATCH.is_file():
        raise HTTPError(503, "the engine is not built yet; run: cargo build --release --offline "
                             "--manifest-path engine/native-engine/Cargo.toml --bin native-watch")
    if not _checkpoint_available():
        raise HTTPError(503, f"the original engine checkpoint is missing: {paths.relative(paths.ENGINE_CHECKPOINT)}")
    if not _renderer_binary().is_file():
        legacy_scene = scene_legacy_enabled() and browser_scene_enabled()
        build = ("cargo build --offline --manifest-path engine/native-spectator/Cargo.toml --bin clash-web-scene"
                 if legacy_scene else
                 "TURBOJPEG_STATIC=1 cargo build --offline --manifest-path engine/native-spectator/Cargo.toml --features web-jpeg --bin clash-web-renderer")
        raise HTTPError(503, f"the original renderer is not built yet; run: {build}")
    if not _renderer_assets().is_file():
        raise HTTPError(503, f"the original renderer assets are missing: {paths.relative(_renderer_assets())}")
    if request.session_id is not None and manager.list(owner=request.session_id):
        raise HTTPError(429, "you already have a match running; end it first")
    if os.environ.get("ARENA_WARM_PLACEMENTS", "1") == "1":
        warmer.enqueue(config["sides"][0]["deck"], config["sides"][1]["deck"])
    try:
        match = manager.create(config)
    except RuntimeError as exc:
        raise HTTPError(429, str(exc)) from exc
    return 201, match.summary()


@app.route("GET", "/api/matches")
def list_matches(request):
    return {"matches": manager.list(owner=request.session_id)}


def _match(request):
    match = manager.get(request.params["id"])
    if match is None or (request.session_id is not None and match.owner != request.session_id):
        raise HTTPError(404, "no such match (it may have ended after being idle)")
    return match


@app.route("GET", "/api/matches/{id}")
def get_match(request):
    return _match(request).summary()


@app.route("GET", "/api/matches/{id}/stream")
def stream(request):
    return EventStream(_match(request).events(), gzip=True)


def _last_event_id(request):
    headers = request.headers
    getter = getattr(headers, "get", None)
    if getter is None:
        return None
    for key in ("Last-Event-ID", "last-event-id"):
        value = getter(key)
        if value:
            return value
    return None


@app.route("GET", "/api/matches/{id}/ticks")
def ticks(request):
    """Ordered protocol-2 snapshots. Gzip is negotiated and flushed per event."""
    match = _match(request)

    def events():
        match.subscribers += 1
        source = None
        try:
            source = iter(match.tick_stream(_last_event_id(request)))
            yield from source
        finally:
            try:
                close = getattr(source, "close", None)
                if close:
                    close()
            finally:
                match.subscribers -= 1
                match.last_watched = time.monotonic()

    source = events()
    return EventStream(source, on_close=source.close, gzip=True)


@app.route("GET", "/api/matches/{id}/video")
def match_video(request):
    """MJPEG stream: one persistent connection carries every new frame, so
    frame rate is not capped by per-frame request round trips."""
    match = _match(request)
    if match.error:
        raise HTTPError(503, match.error)
    if match.renderer is None:
        raise HTTPError(503, match.message)
    boundary = "arenaframe"

    def chunks():
        match.subscribers += 1
        frames = iter(match.renderer.frames())
        try:
            for header, frame in frames:
                yield (f"--{boundary}\r\nContent-Type: image/jpeg\r\n"
                       f"Content-Length: {len(frame)}\r\n"
                       f"X-Arena-Sequence: {header['sequence']}\r\n\r\n").encode() + frame + b"\r\n"
        finally:
            try:
                close = getattr(frames, "close", None)
                if close:
                    close()
            finally:
                match.subscribers -= 1
                match.last_watched = time.monotonic()

    return ByteStream(chunks(), f"multipart/x-mixed-replace; boundary={boundary}")


@app.route("POST", "/api/matches/{id}/input")
def match_input(request):
    match = _match(request)
    if not match.human:
        raise HTTPError(400, "this match has no human player")
    command = _object_body(request)
    if command.get("op") not in ("select", "aim", "play", "cancel", "pause", "reset"):
        raise HTTPError(400, "unknown input op")
    match.submit_input(command)
    return {"ok": True}


@app.route("POST", "/api/matches/{id}/control")
def match_control(request):
    match = _match(request)
    command = _object_body(request)
    if command.get("op") not in ("pause", "reset", "speed", "ready"):
        raise HTTPError(400, "control op must be pause, reset, speed or ready")
    if command["op"] == "speed" and match.human:
        raise HTTPError(400, "speed is only adjustable when watching two models")
    if command["op"] == "speed":
        value = command.get("value", 1)
        if type(value) not in (int, float) or (isinstance(value, float) and not math.isfinite(value)):
            raise HTTPError(400, "speed value must be a finite number")
        command["value"] = max(0.25, min(8.0, value))
    if command["op"] == "pause" and command.get("value") is not None and type(command["value"]) is not bool:
        raise HTTPError(400, "pause value must be a boolean")
    match.control(command)
    return {"ok": True}


@app.route("POST", "/api/matches/{id}/close")
def close_match(request):
    return {"ok": manager.close(_match(request).id)}


register_routes(app, manager)
register_scene_routes(app, manager)
