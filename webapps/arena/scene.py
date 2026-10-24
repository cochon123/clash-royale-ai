"""Original-asset display lists for local browser compositing.

Rust retains presentation planning, while clients cache sprites and composite
them. JPEG rendering is created lazily only when an explicit fallback requests
an image. Neither the game loop nor a slow client waits for presentation work.
"""
from __future__ import annotations

import base64
from collections import OrderedDict
import json
import os
from pathlib import Path
import queue
import re
import subprocess
import threading

from webapps.common import paths
from webapps.common.httpkit import EventStream, HTTPError, Raw
from .renderer import ASSETS, NativeRenderer, asset_selection_args

BINARY = Path(os.environ.get("ARENA_SCENE_BIN", str(
    paths.ENGINE / "native-spectator/target/debug/clash-web-scene")))
TEXTURE_ID = re.compile(r"[0-9a-f]{16,64}")
MAX_PACKET_BYTES = 4 * 1024 * 1024
# Pose PNGs ride the scene stream that is already open. A separate GET for each
# new pose would take the browser's remaining HTTP/1.1 connections (the match
# stream and the scene stream already hold two of the six), and clicks, aims
# and pause would queue behind image downloads for the whole battle.
MAX_INLINE_TEXTURE_BYTES = 3 * 1024 * 1024
MAX_INLINE_FILE_BYTES = 1536 * 1024
MAX_TEXTURE_BYTES = 256 * 1024 * 1024
MAX_TEXTURES = 8192
MAX_TEXTURE_TOMBSTONES = 8192
MAX_PENDING_SNAPSHOTS = 256
MAX_DRAW_COMMANDS = 10000
MAX_SHADOW_DEPTH = 16


def browser_scene_enabled():
    return os.environ.get("ARENA_BROWSER_SCENE", "1") == "1"


def texture_references(draws):
    """Only textures actually referenced by this owner's display lists."""
    result = set()
    pending = [(draws, 0)]
    commands = 0
    while pending:
        children, depth = pending.pop()
        if not isinstance(children, list) or depth > MAX_SHADOW_DEPTH:
            raise ValueError("invalid scene draw list")
        commands += len(children)
        if commands > MAX_DRAW_COMMANDS:
            raise ValueError("scene draw budget exceeded")
        for draw in children:
            if not isinstance(draw, dict):
                raise ValueError("invalid scene command")
            if "tex" in draw:
                value = draw["tex"]
                if not isinstance(value, str) or not TEXTURE_ID.fullmatch(value):
                    raise ValueError("invalid scene texture identity")
                result.add(value)
            if draw.get("kind") == "shadow":
                pending.append((draw.get("draws", []), depth + 1))
    return result


class NativeSceneRenderer:
    def __init__(self, run_dir, *, cards=None):
        self.cards = tuple(cards) if cards else None
        self.run_dir = Path(run_dir)
        self.run_dir.mkdir(parents=True, exist_ok=True)
        # Poses and encoded textures are shared by every match in this arena.
        # A walk frame rendered in one game is reused by the next.
        shared = Path(os.environ.get("ARENA_SCENE_CACHE", self.run_dir.parent / "scene-cache"))
        self.texture_dir = shared / "textures"
        self.pose_dir = shared / "poses"
        self.texture_dir.mkdir(parents=True, exist_ok=True)
        self.pose_dir.mkdir(parents=True, exist_ok=True)
        self.log = (self.run_dir / "scene.log").open("ab", buffering=0)
        try:
            self.proc = subprocess.Popen(
                [str(BINARY), "--assets", str(ASSETS), "--texture-cache", str(self.texture_dir),
                 "--pose-cache", str(self.pose_dir),
                 "--fps", os.environ.get("ARENA_SCENE_FPS", "30"),
                 *asset_selection_args(self.cards)],
                cwd=paths.ENGINE, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=self.log)
        except Exception:
            self.log.close()
            raise
        self.snapshots = queue.Queue(maxsize=MAX_PENDING_SNAPSHOTS)
        self.condition = threading.Condition()
        self.closed = threading.Event()
        self.error = None
        self.latest = None
        self.last_snapshot = None
        self.texture_ids = set()
        self.texture_sizes = {}
        self.expired_texture_ids = OrderedDict()
        self.texture_bytes = 0
        self.fallback = None
        self.fallback_lock = threading.Lock()
        self.writer = threading.Thread(target=self._write, name="scene-input", daemon=True)
        self.reader = threading.Thread(target=self._read, name="scene-output", daemon=True)
        self.writer.start()
        self.reader.start()

    def submit(self, snapshot):
        if self.error:
            raise RuntimeError(self.error)
        if self.closed.is_set():
            return
        self.last_snapshot = snapshot
        try:
            self.snapshots.put_nowait(snapshot)
        except queue.Full:
            # Painter.observe must see every native tick for attacks, spawns,
            # deaths and effect lifetimes. Dropping snapshots corrupts that
            # history; fail explicitly if a stalled presenter cannot keep up.
            self._fail("input snapshot queue overflow; presentation could not preserve every engine tick")
            raise RuntimeError(self.error or "scene input snapshot queue overflow")
        with self.fallback_lock:
            if self.fallback:
                try:
                    self.fallback.submit(snapshot)
                except RuntimeError:
                    # An explicitly requested compatibility view must not kill
                    # authoritative combat or the healthy browser scene. Its
                    # image/frames endpoints expose the fallback error.
                    pass

    def _fail(self, error):
        if not self.closed.is_set():
            with self.condition:
                self.error = f"Original-asset scene presenter stopped: {error}. See scene.log in this match's run folder."
                self.condition.notify_all()

    def _write(self):
        try:
            while not self.closed.is_set():
                try:
                    snapshot = self.snapshots.get(timeout=.2)
                except queue.Empty:
                    continue
                # Rust observes every input snapshot and independently paces
                # display-list output; presentation history is never drained.
                self.proc.stdin.write(json.dumps(snapshot, separators=(",", ":")).encode() + b"\n")
                self.proc.stdin.flush()
        except (OSError, ValueError) as exc:
            self._fail(exc)

    def _accept(self, packet):
        if not isinstance(packet, dict) or type(packet.get("v")) is not int or packet["v"] != 1:
            raise ValueError("unsupported scene protocol")
        if packet.get("width") != 540 or packet.get("height") != 900:
            raise ValueError("unexpected scene dimensions")
        if type(packet.get("sequence")) is not int or packet["sequence"] < 1:
            raise ValueError("invalid scene sequence")
        if self.latest and packet["sequence"] <= self.latest["sequence"]:
            raise ValueError("scene sequence did not advance")
        references = texture_references(packet.get("draws"))
        eviction_list = packet.get("evicted", [])
        if (not isinstance(eviction_list, list) or len(eviction_list) > MAX_TEXTURES
                or any(not isinstance(name, str) or not TEXTURE_ID.fullmatch(name)
                       for name in eviction_list)):
            raise ValueError("invalid scene texture eviction list")
        evicted = set(eviction_list)
        if evicted & references:
            raise ValueError("current scene references an evicted texture")
        removed = evicted & self.texture_ids
        added = references - self.texture_ids
        try:
            sizes = {name: self._texture_path(name).stat().st_size for name in added}
        except (HTTPError, OSError) as exc:
            raise ValueError("missing scene texture") from exc
        texture_bytes = (self.texture_bytes - sum(self.texture_sizes[name] for name in removed)
                         + sum(sizes.values()))
        if (len(self.texture_ids) - len(removed) + len(added) > MAX_TEXTURES
                or texture_bytes > MAX_TEXTURE_BYTES):
            raise ValueError("scene texture budget exceeded")
        # Public packets contain no native filesystem location; whitelist the
        # protocol keys so a future native diagnostic cannot leak host paths.
        public = {k: packet[k] for k in (
            "v", "sequence", "tick", "display_tick", "match_id", "width", "height",
            "clear", "layout", "textures", "draws", "status", "playback_speed") if k in packet}
        if "status" not in public:
            public["status"] = (self.last_snapshot or {}).get("runtime_status", "running")
        with self.condition:
            for name in sorted(removed):
                del self.texture_sizes[name]
                self.expired_texture_ids.pop(name, None)
                self.expired_texture_ids[name] = None
            self.texture_ids.difference_update(removed)
            self.texture_sizes.update(sizes)
            self.texture_ids.update(added)
            for name in added:
                self.expired_texture_ids.pop(name, None)
            while len(self.expired_texture_ids) > MAX_TEXTURE_TOMBSTONES:
                self.expired_texture_ids.popitem(last=False)
            self.texture_bytes = texture_bytes
            self.latest = public
            self.condition.notify_all()

    def _read(self):
        try:
            while not self.closed.is_set():
                line = self.proc.stdout.readline(MAX_PACKET_BYTES + 1)
                if not line:
                    raise RuntimeError(f"presenter exited ({self.proc.wait()})")
                if len(line) > MAX_PACKET_BYTES or not line.endswith(b"\n"):
                    raise ValueError("scene packet too large or incomplete")
                self._accept(json.loads(line))
        except (OSError, ValueError, KeyError, TypeError, RuntimeError, RecursionError) as exc:
            self._fail(exc)

    def _client_packet(self, packet, *, full):
        """Copy of ``packet`` with pose bytes for this subscriber.

        The first packet carries every pose that frame draws, so joining or
        reloading does not open dozens of texture requests. Later packets carry
        only poses encoded since the previous one. The stored packet is left
        unchanged so another subscriber can make its own first delivery.
        """
        if full:
            wanted = texture_references(packet.get("draws"))
        else:
            wanted = {item["id"] for item in packet.get("textures") or []
                      if isinstance(item, dict) and isinstance(item.get("id"), str)}
        files = []
        for ident in wanted:
            try:
                data = self._texture_path(ident).read_bytes()
            except (HTTPError, OSError):
                continue
            if 0 < len(data) <= MAX_INLINE_FILE_BYTES:
                files.append((len(data), ident, data))
        files.sort()
        blobs = []
        budget = MAX_INLINE_TEXTURE_BYTES
        for size, ident, data in files:
            if size > budget:
                break
            blobs.append({"id": ident, "png": base64.b64encode(data).decode("ascii")})
            budget -= size
        if not blobs:
            return packet
        delivered = dict(packet)
        delivered["blobs"] = blobs
        return delivered

    def events(self):
        last = 0
        primed = False
        while not self.closed.is_set():
            with self.condition:
                changed = self.condition.wait_for(
                    lambda: self.error or self.closed.is_set() or
                    self.latest and self.latest["sequence"] > last, timeout=5)
                if self.closed.is_set():
                    return
                error = self.error
                packet = self.latest
            # Never suspend a generator while holding the presenter lock: a
            # slow/disconnected subscriber must not block failure or shutdown.
            if error:
                yield "error", {"error": error}
                return
            if packet and changed:
                last = packet["sequence"]
                full = not primed
                primed = True
                yield "scene", self._client_packet(packet, full=full)
            else:
                yield "heartbeat", {}

    def texture(self, texture_id):
        with self.condition:
            if (not isinstance(texture_id, str) or not TEXTURE_ID.fullmatch(texture_id)
                    or texture_id not in self.texture_ids):
                raise self._texture_unavailable(texture_id)
        try:
            return self._texture_path(texture_id)
        except HTTPError:
            # A native eviction can race a slow client's file request.
            raise self._texture_unavailable(texture_id) from None

    def _texture_unavailable(self, texture_id):
        with self.condition:
            if isinstance(texture_id, str) and texture_id in self.expired_texture_ids:
                return HTTPError(410, "scene texture has expired; refresh the latest scene")
        return HTTPError(404, "no such scene texture")

    def _texture_path(self, texture_id):
        if not isinstance(texture_id, str) or not TEXTURE_ID.fullmatch(texture_id):
            raise HTTPError(404, "no such scene texture")
        try:
            directory = self.texture_dir.resolve(strict=True)
            path = (directory / (texture_id + ".png")).resolve(strict=True)
            if path.parent != directory or not path.is_file():
                raise HTTPError(404, "no such scene texture")
            return path
        except (OSError, ValueError) as exc:
            raise HTTPError(404, "no such scene texture") from exc

    def _fallback(self):
        with self.fallback_lock:
            if self.closed.is_set():
                raise HTTPError(404, "match has closed")
            if self.fallback is None:
                options = {"cards": self.cards} if getattr(self, "cards", None) else {}
                self.fallback = NativeRenderer(self.run_dir / "stream-fallback", **options)
                if self.last_snapshot:
                    self.fallback.submit(self.last_snapshot)
            return self.fallback

    def image(self, after=0):
        return self._fallback().image(after)

    def frames(self):
        yield from self._fallback().frames()

    def close(self):
        self.closed.set()
        with self.condition:
            self.condition.notify_all()
        if self.proc.poll() is None:
            self.proc.terminate()
            try:
                self.proc.wait(timeout=3)
            except subprocess.TimeoutExpired:
                self.proc.kill()
                self.proc.wait(timeout=3)
        self.writer.join(timeout=1)
        self.reader.join(timeout=1)
        for pipe in (self.proc.stdin, self.proc.stdout):
            if pipe:
                pipe.close()
        with self.fallback_lock:
            if self.fallback:
                self.fallback.close()
        self.log.close()


def register_routes(app, manager):
    def owned(request):
        match = manager.get(request.params["id"])
        if match is None or request.session_id is not None and match.owner != request.session_id:
            raise HTTPError(404, "no such match")
        if match.error:
            raise HTTPError(503, match.error)
        if not isinstance(match.renderer, NativeSceneRenderer):
            raise HTTPError(503, "Preparing original-asset browser scene...")
        return match

    @app.route("GET", "/api/matches/{id}/scene")
    def scene(request):
        match = owned(request)

        def events():
            match.subscribers += 1
            source = None
            try:
                source = iter(match.renderer.events())
                yield from source
            finally:
                try:
                    close = getattr(source, "close", None)
                    if close:
                        close()
                finally:
                    match.subscribers -= 1
                    import time
                    match.last_watched = time.monotonic()
        source = events()
        return EventStream(source, on_close=source.close, gzip=True)

    @app.route("GET", "/api/matches/{id}/textures/{texture}")
    def texture(request):
        match = owned(request)
        name = request.params["texture"]
        if not name.endswith(".png"):
            raise HTTPError(404, "no such scene texture")
        texture_id = name[:-4]
        path = match.renderer.texture(texture_id)
        headers = {"Cache-Control": "private, max-age=31536000, immutable", "ETag": '"' + texture_id + '"'}
        if request.headers.get("If-None-Match") == headers["ETag"]:
            return Raw(b"", "image/png", status=304, headers=headers)
        try:
            return Raw(path.read_bytes(), "image/png", headers=headers)
        except OSError as exc:
            raise match.renderer._texture_unavailable(texture_id) from exc
