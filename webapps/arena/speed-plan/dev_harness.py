#!/usr/bin/env python3
"""Replay a synthetic C3 tick stream for the painter worker.

The harness is local only. It does not talk to the engine or the public site.
"""
from __future__ import annotations

import argparse
import base64
import json
import struct
import threading
import time
import zlib
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, urlparse

STATIC = Path(__file__).resolve().parents[1] / "static"
TICKS = 20


def png(width: int, height: int, rgba: bytes) -> bytes:
    raw = bytearray()
    stride = width * 4
    for y in range(height):
        raw.append(0)
        raw.extend(rgba[y * stride:(y + 1) * stride])

    def chunk(kind: bytes, data: bytes) -> bytes:
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)

    ihdr = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", ihdr) + chunk(b"IDAT", zlib.compress(bytes(raw))) + chunk(b"IEND", b"")


def build_recording(ticks: int) -> dict:
    pose = png(4, 4, bytes([220, 96, 48, 255]) * 16)
    frames = []
    for tick in range(ticks):
        x = 80 + (tick % 120) * 3
        frames.append({
            "tick": tick,
            "packet": {
                "v": 1, "sequence": tick + 1, "tick": tick, "display_tick": tick, "match_id": 1,
                "width": 540, "height": 900, "clear": [19, 27, 31, 255], "status": "running", "playback_speed": 1,
                "layout": {"field": [43, 125, 454, 648], "world": [18000, 32000], "cards": [[20, 760, 110, 130], [140, 760, 110, 130], [260, 760, 110, 130], [380, 760, 110, 130]]},
                "draws": [{"id": "entity-1-part-0", "kind": "sprite", "tex": "ab", "m": [12, 0, 0, 12, x, 430], "alpha": 1, "blend": "normal", "anchor": [x + 24, 454]}],
                "textures": [],
            },
        })
    return {"width": 540, "height": 900, "frames": frames, "textures": {"ab": {"png_b64": base64.b64encode(pose).decode()}}}


def build_items(ticks: int) -> list[dict]:
    items = []
    for tick in range(ticks):
        items.append({
            "snapshot": {"tick": tick, "runtime_status": "running", "playback_speed": 1, "match_id": 1, "entities": []},
            "hud": {
                "status": "running", "message": "Engine ready", "match_number": 1, "tick": tick,
                "elapsed_ms": tick * 50, "speed": 1, "hands": [[], []], "towers": [], "crowns": [0, 0], "loading": False,
            },
        })
    return items


def meta() -> dict:
    return {
        "id": "demo", "mode": "watch", "labels": ["Hog", "Bait"], "protocol": 2,
        "assets": {"manifest": "/assets/manifest.json"},
        "cards": ["HogRider", "Fireball"],
        "renderer": {"kind": "browser-painter", "width": 540, "height": 900},
        "ticks_per_second": TICKS,
    }


MANIFEST = {
    "v": 1,
    "bundle": "dev-harness",
    "painter_wasm": "files/painter.wasm",
    "files": {
        "presentation.json": {"url": "files/presentation.json", "bytes": 18, "sha256": "dev"},
        "cards/hog.json": {"url": "files/hog.json", "bytes": 12, "sha256": "dev"},
        "cards/fireball.json": {"url": "files/fireball.json", "bytes": 14, "sha256": "dev"},
    },
    "always": ["presentation.json"],
    "cards": {"HogRider": ["cards/hog.json"], "Fireball": ["cards/fireball.json"]},
}
FILES = {
    "/assets/manifest.json": {"current": "manifest-demo.json"},
    "/assets/manifest-demo.json": MANIFEST,
    "/assets/files/presentation.json": b'{"dev":true}\n',
    "/assets/files/hog.json": b'{"card":1}\n',
    "/assets/files/fireball.json": b'{"card":2}\n',
    "/assets/files/painter.wasm": b"\0asmdev",
}


class Handler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"
    recording: dict = {}
    items: list = []
    jitter_s: float = 0.0
    tick_count: int = 240

    def log_message(self, fmt: str, *args) -> None:
        return

    def _send(self, status: int, body: bytes, content_type: str, extra: dict | None = None) -> None:
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "public, max-age=31536000, immutable" if "/assets/files/" in self.path else "no-store")
        self.send_header("Access-Control-Allow-Origin", "*")
        for key, value in (extra or {}).items():
            self.send_header(key, value)
        self.end_headers()
        if self.command != "HEAD":
            self.wfile.write(body)

    def do_HEAD(self) -> None:
        self.do_GET()

    def do_POST(self) -> None:
        path = urlparse(self.path).path
        length = int(self.headers.get("Content-Length") or 0)
        if length:
            self.rfile.read(length)
        if path == "/api/matches":
            self._send(200, b'{"id":"demo"}', "application/json")
            return
        if path.startswith("/api/matches/demo/"):
            self._send(200, b'{"accepted":true}', "application/json")
            return
        self._send(404, b'{"error":"not found"}', "application/json")

    def do_GET(self) -> None:
        path = urlparse(self.path).path
        if path == "/api/matches/demo":
            self._send(200, b'{"id":"demo","status":"running"}', "application/json")
            return
        if path == "/api/matches/demo/ticks":
            self._ticks()
            return
        if path == "/fixtures/recording.json":
            body = json.dumps(self.recording).encode()
            self._send(200, body, "application/json")
            return
        if path in FILES:
            body = FILES[path]
            raw = body if isinstance(body, bytes) else json.dumps(body).encode()
            kind = "application/json" if not path.endswith(".wasm") else "application/wasm"
            self._send(200, raw, kind)
            return
        rel = path.lstrip("/") or "index.html"
        file = (STATIC / rel).resolve()
        if not str(file).startswith(str(STATIC)) or not file.is_file():
            self._send(404, b'{"error":"not found"}', "application/json")
            return
        kind = "text/html" if file.suffix == ".html" else "text/javascript" if file.suffix == ".js" else "text/css" if file.suffix == ".css" else "application/octet-stream"
        self._send(200, file.read_bytes(), kind)

    def _ticks(self) -> None:
        self.send_response(200)
        self.send_header("Content-Type", "text/event-stream")
        self.send_header("Cache-Control", "no-store")
        self.send_header("X-Accel-Buffering", "no")
        self.end_headers()
        try:
            self._write_event("meta", meta(), "demo:0")
            self._write_event("status", {"status": "running", "message": "Engine ready"}, "demo:0")
            cursor = 0
            while cursor < len(self.items):
                batch = self.items[cursor:cursor + 1]
                cursor += len(batch)
                last = batch[-1]["snapshot"]["tick"]
                self._write_event("ticks", {"v": 2, "items": batch}, f"1:{last}")
                delay = 1 / TICKS
                if self.jitter_s:
                    delay += (zlib.adler32(str(last).encode()) % 1000) / 1000 * self.jitter_s
                time.sleep(delay)
        except (BrokenPipeError, ConnectionResetError):
            return

    def _write_event(self, name: str, payload: dict, event_id: str) -> None:
        data = json.dumps(payload, separators=(",", ":")).encode()
        self.wfile.write(f"id: {event_id}\nevent: {name}\ndata: ".encode() + data + b"\n\n")
        self.wfile.flush()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, default=8765)
    parser.add_argument("--ticks", type=int, default=240)
    parser.add_argument("--jitter-ms", type=float, default=40)
    args = parser.parse_args()
    Handler.recording = build_recording(args.ticks)
    Handler.items = build_items(args.ticks)
    Handler.jitter_s = args.jitter_ms / 1000
    Handler.tick_count = args.ticks
    server = ThreadingHTTPServer((args.host, args.port), Handler)
    print(f"harness http://{args.host}:{args.port}/play.html?id=demo&painter=fake&recording=/fixtures/recording.json ticks={args.ticks} jitter_ms={args.jitter_ms}", flush=True)
    server.serve_forever()


if __name__ == "__main__":
    main()
