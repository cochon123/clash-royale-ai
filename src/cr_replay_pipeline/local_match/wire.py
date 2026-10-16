"""Loopback HTTP protocol. Non-local hosts are refused before any socket."""

from __future__ import annotations

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.error import HTTPError
from urllib.parse import urlparse
from urllib.request import Request, urlopen

from .catalog import Catalog
from .engine import PROTOCOL, Harness, IllegalAction

LOOPBACK_HOSTS = frozenset({"127.0.0.1", "localhost", "::1"})


class RemoteHostRefused(RuntimeError):
    pass


def assert_loopback(url: str) -> str:
    parsed = urlparse(url)
    host = (parsed.hostname or "").lower()
    if parsed.scheme not in {"http", "https"} or host not in LOOPBACK_HOSTS:
        raise RemoteHostRefused(
            f"Refusing non-local endpoint {url!r}. This harness never contacts "
            "Supercell or any other remote host."
        )
    return url


class ProtocolClient:
    def __init__(self, base_url: str, timeout: float = 10.0):
        self.base_url = assert_loopback(base_url).rstrip("/")
        self.timeout = timeout

    def call(self, payload: dict) -> dict:
        # Re-check on every call so a later redirect or edited URL cannot leave.
        assert_loopback(self.base_url)
        encoded = json.dumps(payload).encode("utf-8")
        request = Request(
            self.base_url + "/v1",
            data=encoded,
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        try:
            with urlopen(request, timeout=self.timeout) as response:
                body = json.loads(response.read().decode("utf-8"))
        except HTTPError as exc:
            raw = exc.read().decode("utf-8", errors="replace")
            try:
                body = json.loads(raw)
            except json.JSONDecodeError:
                body = {"ok": False, "error": raw or exc.reason}
            raise IllegalAction(str(body.get("error") or exc.reason)) from exc
        if isinstance(body, dict) and body.get("ok") is False:
            raise IllegalAction(str(body.get("error") or "protocol error"))
        return body

    def hello(self) -> dict:
        return self.call({"op": "hello"})

    def start(self, red: list[str], blue: list[str], seed: int, max_seconds: float) -> dict:
        return self.call(
            {
                "op": "start",
                "red": red,
                "blue": blue,
                "seed": seed,
                "max_seconds": max_seconds,
            }
        )

    def advance(self, seconds: float) -> dict:
        return self.call({"op": "advance", "seconds": seconds})

    def act(self, side: str, decision) -> dict:
        return self.call(
            {
                "op": "act",
                "side": side,
                "card": decision.card,
                "tile_col": decision.tile_col,
                "tile_row": decision.tile_row,
                "event_type": decision.event_type,
            }
        )

    def state(self) -> dict:
        return self.call({"op": "state"})


def _handler_class():
    class Handler(BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"

        def log_message(self, fmt: str, *args) -> None:
            return

        def _send(self, payload: dict, status: int = 200) -> None:
            encoded = json.dumps(payload).encode("utf-8")
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(encoded)))
            self.end_headers()
            self.wfile.write(encoded)

        def do_GET(self) -> None:
            if self.path in ("/health", "/v1/health"):
                self._send(self.server.harness.hello())
                return
            self._send({"ok": False, "error": "not found"}, 404)

        def do_POST(self) -> None:
            if self.path != "/v1":
                self._send({"ok": False, "error": "not found"}, 404)
                return
            length = int(self.headers.get("Content-Length", "0") or 0)
            if length > 1_000_000:
                self._send({"ok": False, "error": "body too large"}, 413)
                return
            try:
                payload = json.loads(self.rfile.read(length).decode("utf-8"))
            except (UnicodeError, json.JSONDecodeError):
                self._send({"ok": False, "error": "invalid json"}, 400)
                return
            try:
                result = self.server.harness.dispatch(payload)
            except (IllegalAction, ValueError, KeyError) as exc:
                self._send({"ok": False, "error": str(exc)}, 400)
                return
            self._send(result)

    return Handler


class ProtocolServer:
    def __init__(self, catalog: Catalog, seed: int = 0):
        self.harness = Harness(catalog, seed=seed)
        self.httpd: ThreadingHTTPServer | None = None
        self.thread: threading.Thread | None = None
        self.url = ""

    def start(self) -> str:
        handler = _handler_class()

        class _Bound(ThreadingHTTPServer):
            def __init__(self, harness: Harness):
                self.harness = harness
                super().__init__(("127.0.0.1", 0), handler)

        self.httpd = _Bound(self.harness)
        self.thread = threading.Thread(target=self.httpd.serve_forever, daemon=True)
        self.thread.start()
        port = self.httpd.server_address[1]
        self.url = f"http://127.0.0.1:{port}"
        return self.url

    def stop(self) -> None:
        if self.httpd is not None:
            self.httpd.shutdown()
            self.httpd.server_close()
            self.httpd = None


def protocol_banner(hello: dict) -> str:
    engine = hello.get("engine")
    if hello.get("real_engine"):
        return f"Local engine {engine} speaking {PROTOCOL}."
    return (
        f"Transport is loopback {PROTOCOL}. Resolver is {engine}. "
        + str(hello.get("blocker") or "")
    )
