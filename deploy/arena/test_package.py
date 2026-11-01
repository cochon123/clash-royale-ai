"""Packaging and nginx checks for the browser asset pack."""
import importlib.util
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
_SPEC = importlib.util.spec_from_file_location("arena_package", Path(__file__).resolve().parent / "package.py")
package = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(package)


class PackageTests(unittest.TestCase):
    def test_scene_presenter_is_not_required(self):
        self.assertNotIn("engine/native-spectator/target/debug/clash-web-scene", package.REQUIRED_PATHS)
        self.assertIn("engine/native-spectator/target/debug/clash-web-renderer", package.REQUIRED_PATHS)

    def test_web_assets_are_included_and_wasm_is_optional(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            directory = root / "deploy/arena/web-assets"
            directory.mkdir(parents=True)
            (directory / "manifest.json").write_text('{"current":"manifest-abc.json"}\n')
            (directory / "files").mkdir()
            (directory / "files" / "abc.png").write_bytes(b"png")
            files = package.web_asset_files(root)
            self.assertEqual([path.name for path in files], ["abc.png", "manifest.json"])
            (directory / "files" / "painter.wasm").write_bytes(b"\0asm")
            names = {path.name for path in package.web_asset_files(root)}
            self.assertIn("painter.wasm", names)
            (directory / "manifest.json").unlink()
            with self.assertRaises(FileNotFoundError):
                package.web_asset_files(root)


class NginxTests(unittest.TestCase):
    def test_static_assets_and_wasm_csp(self):
        text = (ROOT / "deploy/arena/nginx-location.conf").read_text()
        self.assertIn("location ^~ /clashroyale/assets/", text)
        self.assertIn("location = /clashroyale/assets/manifest.json", text)
        self.assertIn("gzip_static on;", text)
        self.assertIn("application/wasm wasm;", text)
        self.assertIn('max-age=31536000, immutable', text)
        self.assertIn('max-age=60', text)
        self.assertIn("script-src 'self' 'wasm-unsafe-eval'", text)


if __name__ == "__main__":
    unittest.main()
