"""Tiny standard-library HTTP toolkit: JSON routes, static mounts and SSE.

Both websites use only the Python standard library for serving, so they run in
the repository's existing environment without extra packages. Every API route
returns JSON, which keeps the server reusable by a future phone app.
"""
from __future__ import annotations

import json
import mimetypes
import re
import sys
import traceback
import zlib
from dataclasses import dataclass, field
from http import HTTPStatus
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any, Callable, Iterable
from urllib.parse import parse_qs, unquote, urlparse

mimetypes.add_type("text/javascript", ".js")
mimetypes.add_type("text/markdown; charset=utf-8", ".md")
MAX_BODY_BYTES = 1024 * 1024


class HTTPError(Exception):
    def __init__(self, status: int, message: str):
        super().__init__(message)
        self.status = status
        self.message = message


@dataclass
class Request:
    method: str
    path: str
    query: dict[str, str]
    params: dict[str, str]
    headers: Any
    raw_body: bytes = b""
    session_id: str | None = None

    def json(self) -> Any:
        if not self.raw_body:
            return {}
        def reject_constant(value):
            raise ValueError(f"{value} is not a JSON number")
        try:
            return json.loads(self.raw_body, parse_constant=reject_constant)
        except (ValueError, RecursionError) as exc:
            raise HTTPError(400, f"invalid JSON body: {exc}") from exc


@dataclass
class EventStream:
    """Return from a route to stream server-sent events. ``events`` yields
    (event_name_or_None, json_payload) pairs until the client disconnects.
    ``gzip=True`` negotiates compression, flushing after each event."""
    events: Iterable[tuple[str | None, Any]]
    on_close: Callable[[], None] | None = None
    gzip: bool = False


def accepts_gzip(accept_encoding: str) -> bool:
    """Negotiate gzip without treating an explicit ``gzip;q=0`` as support.

    An absent header keeps the original identity response. An explicit gzip
    preference overrides wildcard, including its refusal or malformed weight.
    Conflicting duplicate weights conservatively retain the lowest quality.
    """
    qualities = {}
    for item in accept_encoding.split(','):
        coding, *parameters = item.strip().lower().split(';')
        coding = coding.strip()
        if coding not in ('gzip', '*'):
            continue
        quality = 1.
        weights = [part.partition('=')[2].strip() for part in parameters
                   if part.partition('=')[0].strip() == 'q']
        if weights:
            quality = float(weights[0]) if len(weights) == 1 and re.fullmatch(
                r'(?:0(?:\.[0-9]{0,3})?|1(?:\.0{0,3})?)', weights[0]) else 0.
        qualities[coding] = min(qualities.get(coding, 1.), quality)
    return qualities.get('gzip', qualities.get('*', 0.)) > 0


def _event_fields(item):
    """Accept ``(event, payload)`` or ``(event, payload, id)``."""
    if len(item) == 3:
        event, payload, event_id = item
    elif len(item) == 2:
        event, payload = item
        event_id = None
    else:
        raise ValueError("SSE events are (name, payload) or (name, payload, id)")
    if event_id is not None and (not isinstance(event_id, str) or any(char in event_id for char in "\r\n")):
        raise ValueError("SSE id cannot contain a newline")
    return event, payload, event_id


def event_stream_chunks(events, *, gzip=False):
    """Encode SSE and optionally flush gzip after every individual event.

    The flush emits a complete decodable event without waiting for a second
    native tick. Closing the iterator on disconnect also closes its source.
    A third tuple field is the SSE ``id:`` line, used by the tick stream so a
    browser can resume with ``Last-Event-ID``.
    """
    source = iter(events)
    compressor = zlib.compressobj(level=3, wbits=16 + zlib.MAX_WBITS) if gzip else None
    try:
        for item in source:
            event, payload, event_id = _event_fields(item)
            chunk = ((f'id: {event_id}\n' if event_id is not None else '') +
                     (f'event: {event}\n' if event else '') + 'data: ' +
                     json.dumps(payload, separators=(',', ':'), default=str) + '\n\n').encode()
            if compressor:
                chunk = compressor.compress(chunk) + compressor.flush(zlib.Z_SYNC_FLUSH)
            yield chunk
        if compressor:
            yield compressor.flush(zlib.Z_FINISH)
    finally:
        close = getattr(source, 'close', None)
        if close:
            close()


@dataclass
class ByteStream:
    """Return from a route to stream raw chunks (e.g. multipart video).
    ``chunks`` yields bytes until the client disconnects."""
    chunks: Iterable[bytes]
    content_type: str
    on_close: Callable[[], None] | None = None


@dataclass
class Raw:
    body: bytes
    content_type: str = "text/plain; charset=utf-8"
    status: int = 200
    headers: dict[str, str] = field(default_factory=dict)


class App:
    def __init__(self, name: str):
        self.name = name
        self.routes: list[tuple[str, re.Pattern, Callable[[Request], Any]]] = []
        self.mounts: list[tuple[str, Path]] = []

    def route(self, method: str, pattern: str):
        """Register ``pattern`` such as ``/api/matches/{id}``.

        ``{name}`` is one path segment. ``{name:path}`` captures the rest of
        the path, including slashes, for static asset trees.
        """
        def replace(match):
            name, kind = match.group(1), match.group(2)
            body = ".+" if kind == "path" else "[^/]+"
            return rf"(?P<{name}>{body})"

        regex = re.compile("^" + re.sub(r"\{(\w+)(?::(path))?\}", replace, pattern) + "$")

        def decorate(function):
            self.routes.append((method.upper(), regex, function))
            return function
        return decorate

    def mount(self, prefix: str, directory: Path):
        """Serve files below ``directory`` at ``prefix`` (longest prefix wins)."""
        self.mounts.append((prefix.rstrip("/") + "/", Path(directory).resolve()))
        self.mounts.sort(key=lambda item: -len(item[0]))

    def resolve_static(self, path: str) -> Path | None:
        for prefix, directory in self.mounts:
            if path == prefix.rstrip("/") or path.startswith(prefix):
                relative = unquote(path[len(prefix):]) or "index.html"
                candidate = (directory / relative).resolve()
                if candidate.is_dir():
                    candidate = candidate / "index.html"
                if directory in candidate.parents and candidate.is_file():
                    return candidate
        return None

    def serve(self, host: str, port: int):
        app = self

        class Handler(BaseHTTPRequestHandler):
            server_version = f"{app.name}/1"
            protocol_version = "HTTP/1.1"
            # Tiny texture bodies and SSE chunks must not wait for delayed ACKs.
            disable_nagle_algorithm = True

            def log_message(self, fmt, *args):  # keep the console readable
                if not self.path.startswith(("/api/", "/static/", "/cards/")) or "error" in fmt:
                    sys.stderr.write(f"[{app.name}] {self.address_string()} {fmt % args}\n")

            def _dispatch(self, method: str):
                try:
                    url = urlparse(self.path)
                    query = {k: v[-1] for k, v in parse_qs(url.query).items()}
                    lengths = self.headers.get_all("Content-Length", [])
                    if (len(lengths) > 1 or (lengths and not re.fullmatch(r"[0-9]+", lengths[0]))
                            or self.headers.get("Transfer-Encoding")):
                        self.close_connection = True
                        raise HTTPError(400, "a single non-negative Content-Length is required; Transfer-Encoding is unsupported")
                    try:
                        length = int(lengths[0]) if lengths else 0
                    except ValueError as exc:
                        self.close_connection = True
                        raise HTTPError(400, "invalid Content-Length") from exc
                    if length > MAX_BODY_BYTES:
                        self.close_connection = True
                        raise HTTPError(413, f"request body exceeds {MAX_BODY_BYTES} bytes")
                    body = self.rfile.read(length) if length else b""
                    if len(body) != length:
                        self.close_connection = True
                        raise HTTPError(400, "request body is shorter than Content-Length")
                    for route_method, regex, function in app.routes:
                        match = regex.match(url.path)
                        if match and route_method == method:
                            request = Request(method, url.path, query, match.groupdict(), self.headers, body)
                            return self._respond(function(request), head=method == "HEAD")
                    if method in ("GET", "HEAD"):
                        static = app.resolve_static(url.path)
                        if static is not None:
                            return self._file(static, head=method == "HEAD")
                    raise HTTPError(404, f"not found: {url.path}")
                except HTTPError as exc:
                    self._json({"error": exc.message}, exc.status)
                except (BrokenPipeError, ConnectionResetError):
                    pass
                except Exception as exc:  # report, never crash the server
                    traceback.print_exc()
                    try:
                        self._json({"error": f"{type(exc).__name__}: {exc}"}, 500)
                    except OSError:
                        pass

            def do_GET(self):
                self._dispatch("GET")

            def do_HEAD(self):
                self._dispatch("HEAD")

            def do_POST(self):
                self._dispatch("POST")

            def do_DELETE(self):
                self._dispatch("DELETE")

            def _respond(self, result, *, head=False):
                if isinstance(result, EventStream):
                    return self._stream(result)
                if isinstance(result, ByteStream):
                    return self._stream_bytes(result)
                if isinstance(result, Raw):
                    return self._send(result.body, result.content_type, result.status, result.headers, head=head)
                if isinstance(result, Path):
                    return self._file(result)
                status = 200
                if isinstance(result, tuple):
                    status, result = result
                self._json(result, status)

            def _json(self, value, status=200):
                body = json.dumps(value, separators=(",", ":"), default=str).encode()
                self._send(body, "application/json", status)

            def _send(self, body: bytes, content_type: str, status=200, headers=None, head=False):
                self.send_response(status)
                self.send_header("Content-Type", content_type)
                self.send_header("Content-Length", str(len(body)))
                if not any(key.lower() == 'cache-control' for key in (headers or {})):
                    self.send_header("Cache-Control", "no-store")
                for key, value in (headers or {}).items():
                    self.send_header(key, value)
                self.end_headers()
                if not head:
                    self.wfile.write(body)

            def _file(self, path: Path, head=False):
                content_type = mimetypes.guess_type(path.name)[0] or "application/octet-stream"
                if content_type.startswith("text/") and "charset" not in content_type:
                    content_type += "; charset=utf-8"
                self._send(path.read_bytes(), content_type, head=head)

            def _stream(self, stream: EventStream):
                compress = stream.gzip and accepts_gzip(','.join(self.headers.get_all('Accept-Encoding', [])))
                self.send_response(200)
                self.send_header("Content-Type", "text/event-stream")
                self.send_header("Cache-Control", "no-store")
                self.send_header("Connection", "close")
                self.send_header("X-Accel-Buffering", "no")
                if stream.gzip:
                    self.send_header('Vary', 'Accept-Encoding')
                if compress:
                    self.send_header('Content-Encoding', 'gzip')
                self.end_headers()
                self.close_connection = True
                chunks = event_stream_chunks(stream.events, gzip=compress)
                try:
                    for chunk in chunks:
                        self.wfile.write(chunk)
                        self.wfile.flush()
                except (BrokenPipeError, ConnectionResetError, TimeoutError):
                    pass
                finally:
                    try:
                        chunks.close()
                    finally:
                        if stream.on_close:
                            stream.on_close()

            def _stream_bytes(self, stream: ByteStream):
                self.send_response(200)
                self.send_header("Content-Type", stream.content_type)
                self.send_header("Cache-Control", "no-store")
                self.send_header("Connection", "close")
                self.send_header("X-Accel-Buffering", "no")
                self.end_headers()
                self.close_connection = True
                chunks = iter(stream.chunks)
                try:
                    for chunk in chunks:
                        self.wfile.write(chunk)
                        self.wfile.flush()
                except (BrokenPipeError, ConnectionResetError, TimeoutError):
                    pass
                finally:
                    try:
                        close = getattr(chunks, "close", None)
                        if close:
                            close()
                    finally:
                        if stream.on_close:
                            stream.on_close()

        server = ThreadingHTTPServer((host, port), Handler)
        server.daemon_threads = True
        print(f"{self.name}: http://{host if host not in ('0.0.0.0', '') else '127.0.0.1'}:{port}/", flush=True)
        try:
            server.serve_forever()
        except KeyboardInterrupt:
            pass
        finally:
            server.server_close()
