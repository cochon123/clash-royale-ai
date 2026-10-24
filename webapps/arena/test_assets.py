"""C1 asset routes: cache, MIME, precompression and path confinement."""
from __future__ import annotations

import gzip
import hashlib
import json
import os
import tempfile
import threading
import unittest
from email.message import Message
from http.client import HTTPConnection
from http.server import ThreadingHTTPServer
from pathlib import Path
from unittest.mock import patch

from webapps.arena.assets import cache_control, content_type, encoding_rank, resolve_web_asset
from webapps.arena.server import app
from webapps.common.httpkit import HTTPError, Request


HASH = "0123456789abcdef0123"
MANIFEST_NAME = "manifest-abc123def456.json"


def headers_of(**values):
    message = Message()
    for key, value in values.items():
        message[key] = value
    return message


class AssetNegotiationTests(unittest.TestCase):
    def test_encoding_preference_and_cache_rules(self):
        self.assertEqual(encoding_rank(""), ["identity"])
        self.assertEqual(encoding_rank("gzip"), ["gzip", "identity"])
        self.assertEqual(encoding_rank("br, gzip"), ["br", "gzip", "identity"])
        self.assertEqual(encoding_rank("gzip;q=1, br;q=0"), ["gzip", "identity"])
        self.assertEqual(encoding_rank("identity;q=0, br;q=0, gzip;q=0"), ["identity"])
        self.assertEqual(cache_control("manifest.json"), "public, max-age=60")
        self.assertIn("immutable", cache_control(f"files/{HASH}.json"))
        self.assertIn("immutable", cache_control(MANIFEST_NAME))
        self.assertEqual(content_type(Path("painter.wasm")), "application/wasm")

    def test_missing_root_and_traversal_and_precompressed_siblings(self):
        with patch.dict(os.environ, {}, clear=False):
            os.environ.pop("ARENA_WEB_ASSETS_DIR", None)
            with self.assertRaises(HTTPError) as missing:
                resolve_web_asset("manifest.json", headers_of())
            self.assertEqual(missing.exception.status, 404)

        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        root = Path(temp.name) / "assets"
        files = root / "files"
        files.mkdir(parents=True)
        secret = Path(temp.name) / "secret.txt"
        secret.write_text("nope")
        logical = files / f"{HASH}.json"
        payload = b'{"card":"hog"}'
        logical.write_bytes(payload)
        (files / f"{HASH}.json.gz").write_bytes(gzip.compress(payload))
        (files / f"{HASH}.json.br").write_bytes(b"brotli-bytes")
        wasm = files / f"{HASH}.wasm"
        wasm.write_bytes(b"\0asm")
        pointer = {"v": 1, "current": MANIFEST_NAME}
        (root / "manifest.json").write_text(json.dumps(pointer))
        (root / MANIFEST_NAME).write_bytes(b"{}")
        (files / "escape").symlink_to(secret)

        with patch.dict(os.environ, {"ARENA_WEB_ASSETS_DIR": str(root), "ARENA_BASE_PATH": "/clashroyale"}):
            from webapps.arena.assets import manifest_public_path
            self.assertEqual(manifest_public_path(), f"/clashroyale/assets/{MANIFEST_NAME}")
            for relative in ("../secret.txt", "files/../../secret.txt", "files/%2e%2e/%2e%2e/secret.txt",
                             "files/escape", "files/missing.json", "files/%00.json"):
                with self.subTest(relative=relative):
                    with self.assertRaises(HTTPError) as caught:
                        resolve_web_asset(relative, headers_of())
                    self.assertEqual(caught.exception.status, 404)
            identity = resolve_web_asset(f"files/{HASH}.json", headers_of(**{"Accept-Encoding": "identity"}))
            self.assertEqual(identity.body, payload)
            self.assertNotIn("Content-Encoding", identity.headers)
            brotli = resolve_web_asset(f"files/{HASH}.json", headers_of(**{"Accept-Encoding": "br, gzip"}))
            self.assertEqual(brotli.body, b"brotli-bytes")
            self.assertEqual(brotli.headers["Content-Encoding"], "br")
            self.assertEqual(brotli.content_type, "application/json")
            self.assertIn("immutable", brotli.headers["Cache-Control"])
            gzip_body = resolve_web_asset(f"files/{HASH}.json", headers_of(**{"Accept-Encoding": "gzip"}))
            self.assertEqual(gzip.decompress(gzip_body.body), payload)
            self.assertEqual(gzip_body.headers["Content-Encoding"], "gzip")
            wasm_body = resolve_web_asset(f"files/{HASH}.wasm", headers_of(**{"Accept-Encoding": "br, gzip"}))
            self.assertEqual(wasm_body.content_type, "application/wasm")
            self.assertEqual(wasm_body.body, b"\0asm")
            manifest = resolve_web_asset("manifest.json", headers_of())
            self.assertEqual(manifest.headers["Cache-Control"], "public, max-age=60")
            cached = resolve_web_asset(f"files/{HASH}.json", headers_of(**{
                "Accept-Encoding": "identity", "If-None-Match": identity.headers["ETag"]}))
            self.assertEqual((cached.status, cached.body), (304, b""))

    def test_http_serves_wasm_brotli_and_head_on_a_local_port(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        root = Path(temp.name)
        (root / "files").mkdir()
        wasm = root / "files" / f"{HASH}.wasm"
        wasm.write_bytes(b"\0asm-live")
        (root / "files" / f"{HASH}.wasm.br").write_bytes(b"br-wasm")
        ready = threading.Event()
        environ = patch.dict(os.environ, {"ARENA_WEB_ASSETS_DIR": str(root)})
        environ.start()
        self.addCleanup(environ.stop)

        def factory(address, handler):
            server = ThreadingHTTPServer(address, handler)
            self.server = server
            ready.set()
            return server

        with patch("webapps.common.httpkit.ThreadingHTTPServer", side_effect=factory):
            thread = threading.Thread(target=app.serve, args=("127.0.0.1", 8832), daemon=True)
            thread.start()
            self.assertTrue(ready.wait(5))
        self.addCleanup(self.server.shutdown)
        self.addCleanup(thread.join, 5)
        connection = HTTPConnection("127.0.0.1", 8832, timeout=3)
        self.addCleanup(connection.close)
        connection.request("GET", f"/assets/files/{HASH}.wasm", headers={"Accept-Encoding": "br"})
        response = connection.getresponse()
        self.assertEqual(response.status, 200)
        self.assertEqual(response.getheader("Content-Type"), "application/wasm")
        self.assertEqual(response.getheader("Content-Encoding"), "br")
        self.assertIn("immutable", response.getheader("Cache-Control"))
        self.assertEqual(response.read(), b"br-wasm")
        connection.request("HEAD", f"/assets/files/{HASH}.wasm", headers={"Accept-Encoding": "identity"})
        head = connection.getresponse()
        self.assertEqual(head.status, 200)
        self.assertEqual(head.read(), b"")
        self.assertEqual(int(head.getheader("Content-Length")), len(b"\0asm-live"))
        digest = hashlib.sha256(b"\0asm-live").hexdigest()
        self.assertNotEqual(digest, "")
        connection.request("GET", "/assets/../secret")
        missing = connection.getresponse()
        self.assertEqual(missing.status, 404)
        missing.read()


class RoutePatternTests(unittest.TestCase):
    def test_asset_route_captures_nested_paths(self):
        pattern = next(regex for method, regex, _function in app.routes
                       if method == "GET" and regex.match("/assets/files/a.json"))
        self.assertEqual(pattern.match("/assets/files/a.json").group("path"), "files/a.json")
        self.assertIsNone(pattern.match("/assets"))


if __name__ == "__main__":
    unittest.main()
