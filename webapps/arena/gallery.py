"""Opt-in, offline original-art inspection fixtures for renderer development.

The fixture directory contains only generated scenes, PNGs, and a manifest;
it is separate from match output and source assets. Nothing is mounted unless
the operator explicitly sets ARENA_VISUAL_GALLERY_DIR.
"""
from __future__ import annotations

import json
import os
from pathlib import Path

from webapps.common.httpkit import HTTPError


def register_routes(app, static_dir: Path):
    configured = os.environ.get("ARENA_VISUAL_GALLERY_DIR")
    if not configured:
        return
    root = Path(configured).expanduser().resolve()
    app.mount("/api/visual-gallery/files", root)

    @app.route("GET", "/visual-gallery")
    def page(_request):
        return static_dir / "gallery.html"

    @app.route("GET", "/api/visual-gallery/manifest")
    def manifest(_request):
        try:
            source = root / "manifest.json"
            # The static mount applies the same containment rule to all files.
            if source.resolve().parent != root:
                raise HTTPError(404, "visual gallery manifest is unavailable")
            data = json.loads(source.read_text())
        except (OSError, ValueError):
            raise HTTPError(503, "visual gallery fixtures have not been generated") from None
        if not isinstance(data, dict) or data.get("version") != 1 or not isinstance(data.get("cases"), list):
            raise HTTPError(503, "visual gallery manifest is invalid")
        return data
