"""Persistent original-asset spectator, with an image transport for browsers.

The Rust process uses the desktop spectator's Painter, animations and input
geometry, and encodes JPEG frames itself; this side only moves compressed
bytes. Rendering never blocks the engine tick or changes its state.
"""
from __future__ import annotations

import json
import os
import queue
import subprocess
import threading
from pathlib import Path

from webapps.common import paths
from webapps.common.httpkit import HTTPError, Raw

BINARY = Path(os.environ["ARENA_RENDERER_BIN"]) if os.environ.get("ARENA_RENDERER_BIN") else paths.ENGINE / "native-spectator/target/debug/clash-web-renderer"
# A worktree can recover additional original art into its own bundle without
# writing through an analysis/ symlink shared with another checkout.
ASSETS = Path(os.environ.get("ARENA_ASSETS_DIR", str(paths.ENGINE / "analysis/native-spectator"))).expanduser().resolve()
JPEG_QUALITY = int(os.environ.get("ARENA_JPEG_QUALITY", "88"))


def asset_selection_args(cards):
    """Use the dependency-complete deck subset when the bundle supports it.

    Older bundles have no dependency index and retain their existing loading
    behavior. Rust validates all requested cards and transitive documents.
    Names come from the private match configuration, including all eight
    cards on each side rather than just the currently visible hand.
    """
    if not cards:
        return []
    manifest = json.loads((ASSETS / "assets.json").read_text())
    if not isinstance(manifest.get("scene", {}).get("card_dependency_documents"), dict):
        return []
    return ["--cards", ",".join(dict.fromkeys(cards))]


def spectator_hands(hands, *, hide_opponent=False):
    """Empty/cycling native slots carry null IDs and costs, unlike Rust's HUD.

    Keep their slot and unavailable flag, with neutral values for the absent
    card. This also keeps an opponent's hand out of human-view snapshots.
    """
    return [[dict(row, slot=int(row.get("slot") or 0),
                  card_id=int(row.get("card_id") or 0), cost=int(row.get("cost") or 0),
                  name=row.get("name") or "") for row in hand]
            if not (hide_opponent and side == 1) else []
            for side, hand in enumerate(hands)]


class NativeRenderer:
    def __init__(self, run_dir, *, cards=None):
        run_dir.mkdir(parents=True, exist_ok=True)
        selection = asset_selection_args(cards)
        self.log = (run_dir / "viewer.log").open("ab", buffering=0)
        try:
            self.proc = subprocess.Popen([str(BINARY), "--assets", str(ASSETS),
                                          "--fps", os.environ.get("ARENA_RENDER_FPS", "30"),
                                          "--jpeg-quality", str(JPEG_QUALITY), *selection], cwd=paths.ENGINE,
                                         stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=self.log)
        except Exception:
            self.log.close()
            raise
        self.snapshots = queue.Queue(maxsize=2)
        self.condition = threading.Condition()
        self.closed = threading.Event()
        self.error = None
        self.latest = None
        self.writer = threading.Thread(target=self._write, name="spectator-input", daemon=True)
        self.reader = threading.Thread(target=self._read, name="spectator-output", daemon=True)
        self.writer.start()
        self.reader.start()

    def submit(self, frame):
        if self.error:
            raise RuntimeError(self.error)
        if self.closed.is_set():
            return
        try:
            self.snapshots.put_nowait(frame)
        except queue.Full:
            # Bound memory during a stalled renderer, keeping the newest frame.
            try:
                self.snapshots.get_nowait()
            except queue.Empty:
                pass
            self.snapshots.put_nowait(frame)

    def _fail(self, error):
        if not self.closed.is_set():
            with self.condition:
                self.error = f"Original-asset viewer stopped: {error}. See viewer.log in this match's run folder."
                self.condition.notify_all()

    def _write(self):
        try:
            while not self.closed.is_set():
                try:
                    frame = self.snapshots.get(timeout=0.2)
                except queue.Empty:
                    continue
                self.proc.stdin.write(json.dumps(frame, separators=(",", ":")).encode() + b"\n")
                self.proc.stdin.flush()
        except (OSError, ValueError) as exc:
            self._fail(exc)

    def _read(self):
        try:
            while not self.closed.is_set():
                line = self.proc.stdout.readline(65536)
                if not line:
                    raise RuntimeError(f"renderer exited ({self.proc.wait()})")
                header = json.loads(line)
                length = header["bytes"]
                if type(length) is not int or not 0 < length <= 16 * 1024 * 1024:
                    raise ValueError("invalid image length")
                payload = self.proc.stdout.read(length)
                if len(payload) != length:
                    raise ValueError("incomplete frame")
                if header.get("format") != "jpeg":
                    raise ValueError("renderer must emit JPEG frames; deploy the matching binary")
                with self.condition:
                    self.latest = (header, payload)
                    self.condition.notify_all()
        except (OSError, ValueError, KeyError, RuntimeError) as exc:
            self._fail(exc)

    def image(self, after=0):
        with self.condition:
            self.condition.wait_for(lambda: self.error or self.closed.is_set()
                                    or self.latest and self.latest[0]["sequence"] > after, timeout=5)
            if self.error:
                raise HTTPError(503, self.error)
            if self.latest is None:
                raise HTTPError(503, "Preparing the original-asset viewer...")
            return self.latest

    def frames(self):
        """Yield the newest JPEG; repeat it on idle to keep proxies connected."""
        last = 0
        while True:
            with self.condition:
                self.condition.wait_for(lambda: self.error or self.closed.is_set()
                                        or self.latest and self.latest[0]["sequence"] > last, timeout=5)
                if self.error:
                    raise HTTPError(503, self.error)
                if self.closed.is_set() or self.latest is None:
                    return
                header, frame = self.latest
            last = header["sequence"]
            yield header, frame

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
        self.log.close()


class LazyStreamRenderer:
    """JPEG spectator started only when an explicit Stream view asks for pixels.

    ``submit`` remembers the latest snapshot and forwards it once the process
    exists. The default tick path never constructs ``NativeRenderer``.
    """

    def __init__(self, run_dir, *, cards=None):
        self.run_dir = Path(run_dir)
        self.cards = tuple(cards) if cards else None
        self.lock = threading.Lock()
        self.inner = None
        self.last = None
        self.closed = False

    def submit(self, snapshot):
        with self.lock:
            if self.closed:
                return
            self.last = snapshot
            inner = self.inner
        if inner is not None:
            inner.submit(snapshot)

    def _start(self):
        with self.lock:
            if self.closed:
                raise HTTPError(404, "match has closed")
            if self.inner is None:
                options = {"cards": self.cards} if self.cards else {}
                self.inner = NativeRenderer(self.run_dir / "stream-view", **options)
                if self.last is not None:
                    self.inner.submit(self.last)
            return self.inner

    def image(self, after=0):
        return self._start().image(after)

    def frames(self):
        yield from self._start().frames()

    def close(self):
        with self.lock:
            self.closed = True
            inner = self.inner
        if inner is not None:
            inner.close()


def register_routes(app, manager):
    @app.route("GET", "/api/matches/{id}/view")
    def view(request):
        match = manager.get(request.params["id"])
        if match is None or (request.session_id is not None and match.owner != request.session_id):
            raise HTTPError(404, "no such match")
        if match.error:
            raise HTTPError(503, match.error)
        if match.renderer is None:
            raise HTTPError(503, match.message)
        try:
            after = int(request.query.get("after", "0"))
        except ValueError as exc:
            raise HTTPError(400, "after must be a sequence number") from exc
        header, pixels = match.renderer.image(after)
        return Raw(pixels, "image/jpeg", headers={
            "X-Arena-Sequence": str(header["sequence"]),
            "X-Arena-Match": str(header["match_id"]),
            "X-Arena-Tick": str(header["tick"]),
            "X-Arena-Layout": json.dumps(header["layout"], separators=(",", ":")),
        })
