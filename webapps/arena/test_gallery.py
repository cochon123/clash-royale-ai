"""Ensure the opt-in fixture viewer cannot expose sibling run/source files."""
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from webapps.arena.gallery import register_routes
from webapps.common.httpkit import App, HTTPError, Request


class GalleryTests(unittest.TestCase):
    def test_disabled_by_default(self):
        app = App("test")
        with patch.dict(os.environ, {}, clear=True):
            register_routes(app, Path("static"))
        self.assertEqual((app.routes, app.mounts), ([], []))

    def test_fixture_mount_blocks_traversal_and_symlink_escape(self):
        with tempfile.TemporaryDirectory() as directory:
            parent = Path(directory)
            root = parent / "fixtures"
            root.mkdir()
            (parent / "private.json").write_text("secret")
            (root / "escape.json").symlink_to(parent / "private.json")
            (root / "frame.json").write_text("{}")
            app = App("test")
            with patch.dict(os.environ, {"ARENA_VISUAL_GALLERY_DIR": str(root)}):
                register_routes(app, parent / "static")
            for suffix in ("../private.json", "%2e%2e/private.json", "escape.json"):
                self.assertIsNone(app.resolve_static("/api/visual-gallery/files/" + suffix))
            self.assertEqual(app.resolve_static("/api/visual-gallery/files/frame.json"), root / "frame.json")
            route = next(fn for _, pattern, fn in app.routes if pattern.fullmatch("/api/visual-gallery/manifest"))
            request = Request("GET", "", {}, {}, {})
            with self.assertRaises(HTTPError) as error:
                route(request)
            self.assertEqual(error.exception.status, 503)
            (root / "manifest.json").write_text(json.dumps({"version": 1, "cases": []}))
            self.assertEqual(route(request), {"version": 1, "cases": []})
            (root / "manifest.json").unlink()
            (root / "manifest.json").symlink_to(parent / "private.json")
            with self.assertRaises(HTTPError) as error:
                route(request)
            self.assertEqual(error.exception.status, 404)
