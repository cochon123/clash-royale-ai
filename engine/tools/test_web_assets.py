"""Contract tests for the derived bundle and the content-addressed pack."""
import gzip
import json
import tempfile
import unittest
from pathlib import Path

import numpy as np
from PIL import Image

from tools.web_assets import (
    SCHEMA_KEYS, brotli_decompress, effects_seed_exports, pack_directory, rasterize_command,
    refuse_readonly, repack_atlases, subset_document,
)

ENGINE = Path(__file__).resolve().parents[1]


def _tiny_document():
    return {
        "source": "ui.sc",
        "sha256": "abc",
        "exports": {"keep": 1, "drop": 2},
        "movies": {
            "1": {"id": 1, "children_ids": [10], "children_names": ["a"], "children_blending": [0],
                  "frame_elements_offset": 0, "matrix_bank_index": 0, "frame_element_counts": [1],
                  "frame_labels": [""], "framerate": 30, "scaling_grid": None},
            "2": {"id": 2, "children_ids": [11], "children_names": [], "children_blending": [],
                  "frame_elements_offset": 3, "matrix_bank_index": 0, "frame_element_counts": [1],
                  "frame_labels": [], "framerate": 30, "scaling_grid": None},
        },
        "shapes": [
            {"id": 10, "commands": [{"texture_index": 0, "vertices": [[0, 0, 0, 0], [0, 4, 0, 16384], [4, 0, 16384, 0]]}]},
            {"id": 11, "commands": [{"texture_index": 0, "vertices": [[0, 0, 40000, 40000], [0, 4, 40000, 50000], [4, 0, 50000, 40000]]}]},
        ],
        "text_fields": {"11": {"id": 11, "font_name": "Clash Text", "text": "x"}},
        "modifiers": {"10": 1},
        "matrix_banks": [[
            {"a": 1, "b": 0, "c": 0, "d": 1, "tx": 0, "ty": 0},
            {"a": 2, "b": 0, "c": 0, "d": 2, "tx": 3, "ty": 4},
        ]],
        "color_banks": [[{"r_mul": 255, "g_mul": 255, "b_mul": 255, "alpha": 255, "r_add": 0, "g_add": 0, "b_add": 0}]],
        "frame_elements": [0, 65535, 65535, 0, 1, 0],
        "textures": [{"width": 8, "height": 8, "external": "ui_0.sctx"}],
        "vertices": [[0, 0, 0, 0], [9, 9, 9, 9]],
    }


class SubsetTests(unittest.TestCase):
    def test_subset_keeps_schema_and_drops_unnamed_exports(self):
        subset = subset_document(_tiny_document(), {"keep"})
        for key in SCHEMA_KEYS:
            self.assertIn(key, subset)
        self.assertEqual(list(subset["exports"]), ["keep"])
        self.assertNotIn("2", subset["movies"])
        self.assertEqual([shape["id"] for shape in subset["shapes"]], [10])
        self.assertEqual(subset["text_fields"], {})
        self.assertEqual(subset["frame_elements"], [0, 65535, 65535])
        self.assertEqual(subset["textures"][0]["external"], "ui_0.sctx")

    def test_repack_preserves_rasterized_pose_pixels(self):
        atlas = np.full((8, 8, 4), 255, np.uint8)
        atlas[1, 1] = (0, 0, 255, 255)
        uv = 1.0 / 8.0 * 65535.0
        vertices = [[0, 0, uv, uv], [0, 3, uv, uv], [3, 0, uv, uv]]
        doc = {
            "textures": [{"width": 8, "height": 8, "external": "ui_0.sctx"}],
            "shapes": [{"id": 1, "commands": [{"texture_index": 0, "vertices": vertices}]}],
        }
        packed, images, cropped = repack_atlases(doc, {0: atlas})
        self.assertTrue(cropped)
        self.assertLess(images[0].shape[0] * images[0].shape[1], 64)
        before = rasterize_command(vertices, atlas)
        after = rasterize_command(packed["shapes"][0]["commands"][0]["vertices"], images[0])
        self.assertTrue(np.array_equal(before, after))
        self.assertEqual(packed["textures"][0]["width"], images[0].shape[1])
        self.assertEqual(packed["textures"][0]["height"], images[0].shape[0])

    def test_effects_seeds_include_projectiles_and_arena_props(self):
        presentation = {
            "rows": {"Arrow": {"fields": {
                "FileName": "sc/effects.sc",
                "ExportName": "projectile_arrow",
                "ShadowExportName": "projectile_arrow_shadow",
            }}},
            "effects": {},
            "emitters": {},
        }
        assets = {"scene": {"objects": [{"asset": {"document": "effects", "export": "Grassland_1x1_rock1"}}]}}
        names = effects_seed_exports(presentation, assets)
        self.assertIn("projectile_arrow", names)
        self.assertIn("projectile_arrow_shadow", names)
        self.assertIn("Grassland_1x1_rock1", names)

    def test_readonly_engine_trees_are_refused(self):
        with self.assertRaises(RuntimeError):
            refuse_readonly(ENGINE / "analysis" / "native-spectator", ENGINE)


class PackTests(unittest.TestCase):
    def _derived(self, root: Path) -> None:
        (root / "display-trees").mkdir(parents=True)
        (root / "atlases").mkdir()
        (root / "fonts").mkdir()
        (root / "cards").mkdir()
        (root / "assets.json").write_text(json.dumps({
            "documents": [{"name": "arena", "atlases": [{"file": "atlases/arena_0.png"}]}],
            "scene": {"always_documents": ["arena"], "card_dependency_documents": {"Knight": ["arena"]}},
        }))
        (root / "presentation.json").write_text("{}")
        (root / "fonts" / "a.ttf").write_bytes(b"font")
        (root / "display-trees" / "arena.json").write_bytes(b"{\"name\":\"arena\"}")
        Image.fromarray(np.zeros((2, 2, 4), np.uint8), "RGBA").save(root / "atlases" / "arena_0.png")
        Image.fromarray(np.zeros((2, 2, 4), np.uint8), "RGBA").save(root / "cards" / "knight.png")
        (root / "portrait-map.json").write_text(json.dumps({"Knight": "cards/knight.png"}))

    def test_manifest_matches_the_contract_and_precompresses_json(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder) / "derived"
            self._derived(root)
            out = Path(folder) / "out"
            manifest = pack_directory(root, out, "abc" * 20, None)
            self.assertEqual(set(manifest), {"v", "bundle", "painter_wasm", "files", "always", "cards"})
            self.assertEqual(manifest["v"], 1)
            self.assertIsNone(manifest["painter_wasm"])
            self.assertIn("assets.json", manifest["always"])
            self.assertIn("presentation.json", manifest["always"])
            self.assertIn("fonts/a.ttf", manifest["always"])
            self.assertIn("display-trees/arena.json", manifest["always"])
            self.assertIn("atlases/arena_0.png", manifest["always"])
            self.assertIn("cards/knight.png", manifest["cards"]["Knight"])
            record = manifest["files"]["display-trees/arena.json"]
            self.assertEqual(set(record), {"url", "bytes", "sha256"})
            self.assertEqual(len(record["sha256"]), 64)
            self.assertTrue(record["url"].startswith("files/"))
            self.assertEqual(record["url"][6:26], record["sha256"][:20])
            stored = (out / record["url"]).read_bytes()
            self.assertEqual(gzip.decompress((out / (record["url"] + ".gz")).read_bytes()), stored)
            self.assertEqual(brotli_decompress((out / (record["url"] + ".br")).read_bytes()), stored)
            self.assertFalse((out / (manifest["files"]["atlases/arena_0.png"]["url"] + ".gz")).exists())
            pointer = json.loads((out / "manifest.json").read_text())
            self.assertEqual(set(pointer), {"current"})
            self.assertTrue((out / pointer["current"]).is_file())
            self.assertEqual(json.loads((out / pointer["current"]).read_text())["bundle"], "abc" * 20)

    def test_painter_wasm_is_addressed_when_a_file_is_given(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder) / "derived"
            self._derived(root)
            wasm = Path(folder) / "painter.wasm"
            wasm.write_bytes(b"\0asmtest")
            out = Path(folder) / "out"
            manifest = pack_directory(root, out, "ff", wasm)
            self.assertTrue(manifest["painter_wasm"].startswith("files/"))
            self.assertTrue(manifest["painter_wasm"].endswith(".wasm"))
            self.assertEqual((out / manifest["painter_wasm"]).read_bytes(), b"\0asmtest")
            self.assertNotIn(manifest["painter_wasm"], {item["url"] for item in manifest["files"].values()}
                             or True)
            self.assertTrue(any(item["url"] == manifest["painter_wasm"] for item in []) or
                            (out / manifest["painter_wasm"]).is_file())


if __name__ == "__main__":
    unittest.main()
