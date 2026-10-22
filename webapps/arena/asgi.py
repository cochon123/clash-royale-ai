"""Production Arena adapter: Uvicorn, signed sessions and same-origin writes.

Run one worker: the reference engines and match manager belong to this process.
"""
from __future__ import annotations

from contextlib import asynccontextmanager
from email.message import Message
import json
import mimetypes
import os
from pathlib import Path
import re
import secrets
import time
from urllib.parse import urlsplit

import anyio
from starlette.applications import Starlette
from starlette.concurrency import run_in_threadpool
from starlette.middleware.sessions import SessionMiddleware
from starlette.middleware.trustedhost import TrustedHostMiddleware
from starlette.requests import Request as WebRequest
from starlette.responses import FileResponse, JSONResponse, Response, StreamingResponse
from starlette.routing import Mount, Route

from webapps.common.httpkit import ByteStream, EventStream, HTTPError, MAX_BODY_BYTES, Raw, Request
from .server import app as router, manager, warmup


class ClosingStreamingResponse(StreamingResponse):
    """Release the synchronous source even when the socket disconnects."""

    def __init__(self, content, *, on_close=None, **kwargs):
        self.source = iter(content)
        self.on_close = on_close
        super().__init__(self.source, **kwargs)

    async def __call__(self, scope, receive, send):
        try:
            await super().__call__(scope, receive, send)
        finally:
            # A cancelled response must still release its renderer subscriber.
            # Threadpool iteration has finished before we close its generator.
            with anyio.CancelScope(shield=True):
                try:
                    close = getattr(self.source, "close", None)
                    if close:
                        await run_in_threadpool(close)
                finally:
                    if self.on_close:
                        await run_in_threadpool(self.on_close)


def create_app(*, secret=None, secure=True, base_path="", allowed_hosts=None, max_matches=1, start_warmup=True):
    secret = secret or os.environ.get("ARENA_SESSION_SECRET")
    if not secret or len(secret) < 32:
        raise RuntimeError("ARENA_SESSION_SECRET must contain at least 32 characters")
    prefix = base_path.rstrip("/")
    if prefix and not re.fullmatch(r"(?:/[a-zA-Z0-9_-]+)+", prefix):
        raise ValueError("invalid Arena base path")
    manager.max_matches = max_matches
    starts = {}  # bounded, in-process admission throttling by signed browser ID

    @asynccontextmanager
    async def lifespan(_app):
        if start_warmup:
            warmup()
        try:
            yield
        finally:
            manager.close_all()

    def result_response(value, *, method):
        if isinstance(value, EventStream):
            # A synchronous iterator is scheduled by Starlette outside the
            # event loop; waiting for a native tick never blocks other clients.
            def iterator():
                try:
                    for event, payload in value.events:
                        yield ((f"event: {event}\n" if event else "") + "data: " +
                               json.dumps(payload, separators=(",", ":"), default=str) + "\n\n").encode()
                finally:
                    close = getattr(value.events, "close", None)
                    if close:
                        close()
            return ClosingStreamingResponse(iterator(), on_close=value.on_close, media_type="text/event-stream",
                                     headers={"X-Accel-Buffering": "no", "Cache-Control": "no-store"})
        if isinstance(value, ByteStream):
            return ClosingStreamingResponse(value.chunks, on_close=value.on_close, media_type=value.content_type,
                                     headers={"X-Accel-Buffering": "no", "Cache-Control": "no-store"})
        if isinstance(value, Raw):
            return Response(value.body if method != "HEAD" else b"", status_code=value.status,
                            media_type=value.content_type, headers=value.headers)
        if isinstance(value, Path):
            if value.suffix == ".html":
                html = value.read_text()
                if prefix:
                    html = re.sub(r'((?:href|src)=["\'])/', lambda m: m[1] + prefix + "/", html)
                return Response(html, media_type="text/html")
            return FileResponse(value, media_type=mimetypes.guess_type(value.name)[0])
        status, value = value if isinstance(value, tuple) else (200, value)
        return JSONResponse(value, status_code=status)

    async def dispatch(web: WebRequest):
        sid = web.session.get("arena_id")
        if not isinstance(sid, str) or not re.fullmatch(r"[A-Za-z0-9_-]{32}", sid):
            sid = secrets.token_urlsafe(24)
            web.session["arena_id"] = sid
        path = "/" + web.path_params.get("path", "")
        try:
            if web.method not in ("GET", "HEAD"):
                origin = web.headers.get("origin")
                if origin and urlsplit(origin).netloc.lower() != web.headers.get("host", "").lower():
                    raise HTTPError(403, "use Arena from its own website")
            if web.method == "POST" and path == "/api/matches":
                now = time.monotonic()
                previous = starts.get(sid, 0)
                if now - previous < 3:
                    raise HTTPError(429, "Please wait a few seconds before starting another match.")
                starts[sid] = now
                if len(starts) > 5000:
                    starts.clear()
                    starts[sid] = now
            body = bytearray()
            async for chunk in web.stream():
                body.extend(chunk)
                if len(body) > MAX_BODY_BYTES:
                    raise HTTPError(413, "request body too large")
            headers = Message()
            for key, value in web.headers.items():
                headers[key] = value
            for method, regex, function in router.routes:
                match = regex.match(path)
                if method == web.method and match:
                    request = Request(web.method, path, dict(web.query_params), match.groupdict(),
                                      headers, bytes(body), session_id=sid)
                    value = await run_in_threadpool(function, request)
                    response = result_response(value, method=web.method)
                    break
            else:
                static = router.resolve_static(path) if web.method in ("GET", "HEAD") else None
                if static is None:
                    raise HTTPError(404, "not found")
                response = result_response(static, method=web.method)
            response.headers.setdefault("Cache-Control", "no-store")
            response.headers["X-Content-Type-Options"] = "nosniff"
            return response
        except HTTPError as error:
            return JSONResponse({"error": error.message}, status_code=error.status,
                                headers={"Cache-Control": "no-store"})

    core = Starlette(routes=[Route("/{path:path}", dispatch, methods=["GET", "HEAD", "POST", "DELETE"])])
    application = Starlette(routes=[Mount(prefix, core)] if prefix else core.routes, lifespan=lifespan)
    application.add_middleware(SessionMiddleware, secret_key=secret, session_cookie="arena_session",
                               same_site="lax", https_only=secure, max_age=7 * 86400,
                               path=prefix + "/" if prefix else "/")
    if allowed_hosts:
        application.add_middleware(TrustedHostMiddleware, allowed_hosts=allowed_hosts)
    return application


def application():
    """Uvicorn --factory entry point, configured by the service environment."""
    return create_app(secure=os.environ.get("ARENA_SECURE_COOKIES", "1") == "1",
                      base_path=os.environ.get("ARENA_BASE_PATH", ""),
                      allowed_hosts=os.environ.get("ARENA_ALLOWED_HOSTS", "localhost").split(","),
                      max_matches=int(os.environ.get("ARENA_MAX_MATCHES", "1")),
                      start_warmup=os.environ.get("ARENA_WARMUP", "1") == "1")
