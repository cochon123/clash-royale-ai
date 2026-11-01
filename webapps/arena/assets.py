"""Static C1 asset packs for local Arena, matching the intended nginx location.

Content-addressed files are immutable. ``manifest.json`` is the short-cache
pointer. JSON siblings ``.br`` and ``.gz`` are selected from ``Accept-Encoding``
(brotli, then gzip). Paths cannot escape ``ARENA_WEB_ASSETS_DIR``.
"""
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import re
from urllib.parse import unquote

from webapps.common.httpkit import HTTPError, Raw


HASHED_FILE = re.compile(r"[0-9a-f]{20}\.[A-Za-z0-9]+")
HASHED_MANIFEST = re.compile(r"manifest-[0-9a-f]{12}\.json")
CONTENT_TYPES = {
    ".wasm": "application/wasm",
    ".json": "application/json",
    ".png": "image/png",
    ".webp": "image/webp",
    ".js": "text/javascript; charset=utf-8",
    ".css": "text/css; charset=utf-8",
    ".svg": "image/svg+xml",
    ".html": "text/html; charset=utf-8",
}


def assets_root() -> Path | None:
    raw = os.environ.get("ARENA_WEB_ASSETS_DIR")
    if not raw:
        return None
    return Path(raw).expanduser().resolve()


def manifest_public_path() -> str | None:
    """URL path of the current content-addressed manifest, or None."""
    root = assets_root()
    if root is None:
        return None
    try:
        pointer = json.loads((root / "manifest.json").read_text())
    except (OSError, ValueError):
        return None
    current = pointer.get("current") if isinstance(pointer, dict) else None
    if not isinstance(current, str) or safe_asset_file(root, current) is None:
        return None
    base = os.environ.get("ARENA_BASE_PATH", "").rstrip("/")
    return f"{base}/assets/{current}"


def cache_control(relative: str) -> str:
    name = Path(relative).name
    if name == "manifest.json":
        return "public, max-age=60"
    if HASHED_MANIFEST.fullmatch(name) or (relative.startswith("files/") and HASHED_FILE.fullmatch(name)):
        return "public, max-age=31536000, immutable"
    return "public, max-age=300"


def content_type(path: Path) -> str:
    return CONTENT_TYPES.get(path.suffix.lower(), "application/octet-stream")


def encoding_rank(accept_encoding: str) -> list[str]:
    """Preferred encodings with q > 0, brotli before gzip. Absent header is identity."""
    if not accept_encoding:
        return ["identity"]
    qualities: dict[str, float] = {}
    for item in accept_encoding.split(","):
        coding, *parameters = item.strip().lower().split(";")
        coding = coding.strip()
        if coding not in ("br", "gzip", "identity", "*"):
            continue
        quality = 1.0
        weights = [part.partition("=")[2].strip() for part in parameters if part.partition("=")[0].strip() == "q"]
        if weights:
            quality = float(weights[0]) if len(weights) == 1 and re.fullmatch(
                r"(?:0(?:\.[0-9]{0,3})?|1(?:\.0{0,3})?)", weights[0]) else 0.0
        qualities[coding] = min(qualities.get(coding, 1.0), quality)
    wildcard = qualities.get("*")

    def quality_of(name: str) -> float:
        if name in qualities:
            return qualities[name]
        if wildcard is not None:
            return wildcard
        return 1.0 if name == "identity" else 0.0

    ranked = [name for name in ("br", "gzip", "identity") if quality_of(name) > 0]
    return ranked or ["identity"]


def header_value(headers, name: str) -> str:
    if headers is None:
        return ""
    getter = getattr(headers, "get", None)
    if getter is None:
        return ""
    for key in (name, name.lower()):
        value = getter(key)
        if value:
            return value
    return ""


def safe_asset_file(root: Path, relative: str) -> Path | None:
    """Resolved file inside ``root``, or None when the path is unsafe or missing."""
    text = unquote(relative)
    if not text or "\x00" in text or "\\" in text or text.startswith("/"):
        return None
    parts = Path(text).parts
    if not parts or any(part in ("", ".", "..") for part in parts):
        return None
    try:
        root_resolved = root.resolve(strict=True)
        resolved = root_resolved.joinpath(*parts).resolve(strict=True)
        resolved.relative_to(root_resolved)
    except (OSError, ValueError):
        return None
    if not resolved.is_file():
        return None
    return resolved


def resolve_web_asset(relative: str, headers) -> Raw:
    root = assets_root()
    if root is None:
        raise HTTPError(404, "web assets are not configured")
    logical = safe_asset_file(root, relative)
    if logical is None:
        raise HTTPError(404, "no such asset")
    encoding = "identity"
    body_path = logical
    for choice in encoding_rank(header_value(headers, "Accept-Encoding")):
        if choice == "identity":
            break
        sibling = safe_asset_file(root, relative + "." + ("br" if choice == "br" else "gz"))
        if sibling is not None:
            encoding = choice
            body_path = sibling
            break
    try:
        body = body_path.read_bytes()
    except OSError as exc:
        raise HTTPError(404, "no such asset") from exc
    etag = _etag(relative, logical, encoding)
    response_headers = {
        "Cache-Control": cache_control(relative),
        "ETag": etag,
        "Vary": "Accept-Encoding",
        "X-Content-Type-Options": "nosniff",
    }
    if encoding != "identity":
        response_headers["Content-Encoding"] = encoding
    if header_value(headers, "If-None-Match") == etag:
        return Raw(b"", content_type(logical), status=304, headers=response_headers)
    return Raw(body, content_type(logical), headers=response_headers)


def _etag(relative: str, logical: Path, encoding: str) -> str:
    name = Path(relative).name
    stem = name.split(".", 1)[0]
    if re.fullmatch(r"[0-9a-f]{12,64}", stem):
        token = stem
    else:
        stat = logical.stat()
        token = hashlib.sha256(f"{stat.st_mtime_ns}:{stat.st_size}:{relative}".encode()).hexdigest()[:20]
    if encoding != "identity":
        token = f"{token}-{encoding}"
    return f'"{token}"'


def register_routes(app):
    @app.route("GET", "/assets/{path:path}")
    def web_asset(request):
        return resolve_web_asset(request.params["path"], request.headers)

    @app.route("HEAD", "/assets/{path:path}")
    def web_asset_head(request):
        return resolve_web_asset(request.params["path"], request.headers)
