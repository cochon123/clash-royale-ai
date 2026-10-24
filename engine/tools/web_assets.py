"""Derived spectator bundle and content-addressed Arena asset packs (contract C1).

The source bundle under ``engine/analysis`` is only read. ``always_documents`` and
``card_dependency_documents`` are written into a derived copy, ``ui`` and
``effects`` are reduced to the exports the Painter can request, and
``deploy/arena/build_web_assets.py`` publishes that copy as immutable files.

``painter_wasm`` in the manifest is ``files/<hash>.wasm`` when ``--painter-wasm``
points at a file. When the flag is omitted the field is JSON null and no wasm
file is written. W1's binary is not built here.
"""
from __future__ import annotations

import copy
import ctypes
import datetime as dt
import gzip
import hashlib
import json
import math
import os
import shutil
from pathlib import Path

import numpy as np
from PIL import Image

from tools.asset_coverage import (
    finalize_coverage, output_is_readonly_tree, plan_coverage, selectable_native_cards,
)
from tools.spectator_presentation import DEFAULT_APK, DEFAULT_LOGIC

HP_EXPORTS = (
    "hp_player_small", "hp_enemy_small",
    "hp_player_medium", "hp_enemy_medium",
    "hp_player_high", "hp_enemy_high",
    "hp_player_tower_withNumber", "hp_enemy_tower_withNumber",
)
UI_FIXED_EXPORTS = ("card_item_frame", "card_item_elixir_ingame", "card_item_image")
DAMAGE_FILTERS = (
    "filter_damage_up", "filter_damage_down", "filter_damage_left", "filter_damage_right",
)
SCHEMA_KEYS = (
    "color_banks", "exports", "frame_elements", "matrix_banks", "modifiers", "movies",
    "sha256", "shapes", "source", "text_fields", "textures",
)
IDENTITY = 65535
MAX_ATLAS = 4096
# int(u + 0.5) is not translation-invariant for coordinates just outside the
# texture, where the rasterizer clamps. One replicated edge pixel absorbs that.
EDGE_PAD = 1


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def brotli_compress(data: bytes, quality: int = 11) -> bytes:
    """Lossless brotli via the system libbrotlienc. Quality 11 matches a small static file."""
    lib = ctypes.CDLL("libbrotlienc.so.1")
    lib.BrotliEncoderMaxCompressedSize.argtypes = [ctypes.c_size_t]
    lib.BrotliEncoderMaxCompressedSize.restype = ctypes.c_size_t
    lib.BrotliEncoderCompress.argtypes = [
        ctypes.c_int, ctypes.c_int, ctypes.c_int,
        ctypes.c_size_t, ctypes.c_void_p,
        ctypes.POINTER(ctypes.c_size_t), ctypes.c_void_p,
    ]
    lib.BrotliEncoderCompress.restype = ctypes.c_int
    cap = lib.BrotliEncoderMaxCompressedSize(len(data)) or (len(data) + 1024)
    out = ctypes.create_string_buffer(cap)
    size = ctypes.c_size_t(cap)
    # mode 1 is the text encoder. lgwin 22 is the library default window.
    ok = lib.BrotliEncoderCompress(quality, 22, 1, len(data), data, ctypes.byref(size), out)
    if not ok:
        raise RuntimeError("brotli compress failed")
    return out.raw[:size.value]


def brotli_decompress(data: bytes) -> bytes:
    lib = ctypes.CDLL("libbrotlidec.so.1")
    lib.BrotliDecoderDecompress.argtypes = [
        ctypes.c_size_t, ctypes.c_void_p, ctypes.POINTER(ctypes.c_size_t), ctypes.c_void_p,
    ]
    lib.BrotliDecoderDecompress.restype = ctypes.c_int
    cap = max(len(data) * 8, 64)
    for _ in range(8):
        out = ctypes.create_string_buffer(cap)
        size = ctypes.c_size_t(cap)
        status = lib.BrotliDecoderDecompress(len(data), data, ctypes.byref(size), out)
        if status == 1:
            return out.raw[:size.value]
        cap *= 4
    raise RuntimeError(f"brotli decompress failed ({status})")


def refuse_readonly(path: Path, engine: Path) -> None:
    if output_is_readonly_tree(path, engine):
        raise RuntimeError(f"refusing to write inside the shared engine tree: {path}")


def _scene_exports(node, document: str, found: set[str]) -> None:
    """Collect export names stored next to ``{"document": document}`` in assets.json."""
    if isinstance(node, dict):
        if node.get("document") == document:
            for key in ("export", "blue_export", "red_export"):
                value = node.get(key)
                if isinstance(value, str) and value:
                    found.add(value)
        for value in node.values():
            _scene_exports(value, document, found)
    elif isinstance(node, list):
        for value in node:
            _scene_exports(value, document, found)


def ui_seed_exports(assets: dict) -> set[str]:
    """Export names draw.rs and hud.rs request from ``ui`` by exact string."""
    names = set(HP_EXPORTS) | set(UI_FIXED_EXPORTS)
    visuals = assets.get("scene", {}).get("card_visuals") or {}
    rows = visuals.values() if isinstance(visuals, dict) else visuals
    for row in rows:
        if not isinstance(row, dict):
            continue
        for key in ("card_frame_export", "elixir_icon_export", "portrait_slot_export"):
            if isinstance(row.get(key), str):
                names.add(row[key])
        ui = row.get("ui") or {}
        for key in ("frame_export", "elixir_export"):
            if isinstance(ui.get(key), str):
                names.add(ui[key])
    _scene_exports(assets.get("scene") or {}, "ui", names)
    return names


def effects_seed_exports(presentation: dict, assets: dict | None = None) -> set[str]:
    """Exact ``effects`` export names the Painter resolves from presentation and the scene index."""
    names = set(DAMAGE_FILTERS)
    for row in (presentation.get("rows") or {}).values():
        fields = row.get("fields") or {}
        file_stem = Path(str(fields.get("FileName") or "")).stem
        for key, value in fields.items():
            if not isinstance(value, str) or not value:
                continue
            if key == "DeployBaseAnimExportName" or (file_stem == "effects" and "Export" in key):
                names.add(value)
    for effect in (presentation.get("effects") or {}).values():
        for component in effect.get("RenderableComponents") or []:
            file_name = component.get("FileName") or ""
            export = component.get("ExportName")
            if export and Path(file_name).stem == "effects":
                names.add(export)
    for emitter in (presentation.get("emitters") or {}).values():
        resources = emitter.get("ParticleResource") or []
        exports = emitter.get("ParticleExportName") or []
        for resource, export in zip(resources, exports):
            if export and Path(str(resource)).stem == "effects":
                names.add(export)
    if assets is not None:
        _scene_exports(assets.get("scene") or {}, "effects", names)
    return names


def reachable_ids(doc: dict, seeds: set[str]) -> set[int]:
    movies = {int(key): movie for key, movie in doc["movies"].items()}
    stack = [doc["exports"][name] for name in seeds if name in doc["exports"]]
    seen: set[int] = set()
    while stack:
        ident = int(stack.pop())
        if ident in seen:
            continue
        seen.add(ident)
        movie = movies.get(ident)
        if movie:
            stack.extend(int(child) for child in movie["children_ids"])
    return seen


def _used_bank_slots(doc: dict, movies: list[dict]) -> tuple[dict[int, set[int]], dict[int, set[int]]]:
    matrices: dict[int, set[int]] = {}
    colors: dict[int, set[int]] = {}
    elements = doc["frame_elements"]
    for movie in movies:
        if movie["frame_elements_offset"] == IDENTITY or movie["frame_elements_offset"] == 2**32 - 1:
            continue
        bank = int(movie["matrix_bank_index"])
        start = int(movie["frame_elements_offset"])
        count = sum(movie["frame_element_counts"]) * 3
        chunk = elements[start:start + count]
        for index in range(0, len(chunk) - 2, 3):
            matrix = int(chunk[index + 1])
            color = int(chunk[index + 2])
            if matrix != IDENTITY:
                matrices.setdefault(bank, set()).add(matrix)
            if color != IDENTITY:
                colors.setdefault(bank, set()).add(color)
    return matrices, colors


def subset_document(doc: dict, seeds: set[str]) -> dict:
    """Drop exports the Painter does not name. Field names stay the Rust schema."""
    seen = reachable_ids(doc, seeds)
    movies = []
    kept_movies = {}
    for key, movie in doc["movies"].items():
        if int(key) not in seen:
            continue
        movie = copy.copy(movie)
        movies.append(movie)
        kept_movies[key] = movie
    matrix_slots, color_slots = _used_bank_slots(doc, movies)
    banks_used = sorted(set(matrix_slots) | set(color_slots) | {int(m["matrix_bank_index"]) for m in movies})
    bank_remap = {old: new for new, old in enumerate(banks_used)}
    matrix_remap = {}
    color_remap = {}
    new_matrices = []
    new_colors = []
    for old in banks_used:
        source_m = doc["matrix_banks"][old] if old < len(doc["matrix_banks"]) else []
        source_c = doc["color_banks"][old] if old < len(doc["color_banks"]) else []
        order_m = sorted(slot for slot in matrix_slots.get(old, ()) if slot < len(source_m))
        order_c = sorted(slot for slot in color_slots.get(old, ()) if slot < len(source_c))
        matrix_remap[old] = {slot: index for index, slot in enumerate(order_m)}
        color_remap[old] = {slot: index for index, slot in enumerate(order_c)}
        new_matrices.append([source_m[slot] for slot in order_m])
        new_colors.append([source_c[slot] for slot in order_c])
    elements = doc["frame_elements"]
    new_elements: list[int] = []
    for movie in movies:
        movie["matrix_bank_index"] = bank_remap[int(movie["matrix_bank_index"])]
        if movie["frame_elements_offset"] in (IDENTITY, 2**32 - 1):
            continue
        old_bank = banks_used[movie["matrix_bank_index"]]
        start = int(movie["frame_elements_offset"])
        count = sum(movie["frame_element_counts"]) * 3
        chunk = [int(value) for value in elements[start:start + count]]
        mapped = []
        for index in range(0, len(chunk) - 2, 3):
            child, matrix, color = chunk[index], chunk[index + 1], chunk[index + 2]
            if matrix != IDENTITY:
                matrix = matrix_remap[old_bank].get(matrix, IDENTITY)
            if color != IDENTITY:
                color = color_remap[old_bank].get(color, IDENTITY)
            mapped.extend((child, matrix, color))
        movie["frame_elements_offset"] = len(new_elements)
        new_elements.extend(mapped)
    subset = {
        "source": doc["source"],
        "sha256": doc["sha256"],
        "exports": {name: ident for name, ident in doc["exports"].items() if name in seeds and name in doc["exports"]},
        "movies": kept_movies,
        "shapes": [shape for shape in doc["shapes"] if int(shape["id"]) in seen],
        "text_fields": {key: value for key, value in doc["text_fields"].items() if int(key) in seen},
        "modifiers": {key: value for key, value in doc["modifiers"].items() if int(key) in seen},
        "matrix_banks": new_matrices,
        "color_banks": new_colors,
        "frame_elements": new_elements,
        "textures": copy.deepcopy(doc["textures"]),
        "vertices": [],
    }
    for key in SCHEMA_KEYS:
        if key not in subset:
            raise RuntimeError(f"subset dropped schema field {key}")
    return subset


def rasterize_command(vertices: list[list[float]], atlas: np.ndarray) -> np.ndarray:
    """CPU pose raster used to check a cropped atlas. Ports native-ui-viewer rasterize."""
    if len(vertices) < 3:
        return np.zeros((1, 1, 4), np.uint8)
    height_a, width_a = atlas.shape[:2]
    mapped = []
    for vertex in vertices:
        mapped.append([
            float(vertex[0]), float(vertex[1]),
            float(vertex[2]) / 65535.0 * width_a,
            float(vertex[3]) / 65535.0 * height_a,
        ])
    xmin = min(v[0] for v in mapped)
    ymin = min(v[1] for v in mapped)
    xmax = max(v[0] for v in mapped)
    ymax = max(v[1] for v in mapped)
    width = max(1, int(xmax - xmin + 1.5))
    height = max(1, int(ymax - ymin + 1.5))
    if width > 4096 or height > 4096:
        raise RuntimeError(f"shape exceeds raster bounds {width}x{height}")
    image = np.zeros((height, width, 4), np.uint8)
    filled = np.zeros(width * height, np.bool_)
    for index in range(len(mapped) - 2):
        if index % 2 == 0:
            tri = [mapped[index], mapped[index + 1], mapped[index + 2]]
        else:
            tri = [mapped[index], mapped[index + 2], mapped[index + 1]]
        shifted = [[v[0] - xmin, v[1] - ymin, v[2], v[3]] for v in tri]
        denominator = (shifted[1][1] - shifted[2][1]) * (shifted[0][0] - shifted[2][0]) + (
            shifted[2][0] - shifted[1][0]) * (shifted[0][1] - shifted[2][1])
        if abs(denominator) < 1e-6:
            continue
        if denominator < 0:
            shifted[1], shifted[2] = shifted[2], shifted[1]
            denominator = -denominator
        v0, v1, v2 = shifted
        inverse = 1.0 / denominator
        min_y = max(0, int(min(v0[1], v1[1], v2[1])))
        max_y = min(height - 1, int(max(v0[1], v1[1], v2[1]) + 0.5))
        for py in range(min_y, max_y + 1):
            intersections = []
            y = float(py)
            for start, end in ((v0, v1), (v1, v2), (v2, v0)):
                if start[1] != end[1] and ((start[1] <= y < end[1]) or (end[1] <= y < start[1])):
                    intersections.append(start[0] + (y - start[1]) / (end[1] - start[1]) * (end[0] - start[0]))
            if len(intersections) < 2:
                continue
            min_x = max(0, int(min(intersections)))
            max_x = min(width - 1, int(max(intersections) + 0.5))
            for px in range(min_x, max_x + 1):
                w0 = ((v1[1] - v2[1]) * (px - v2[0]) + (v2[0] - v1[0]) * (py - v2[1])) * inverse
                w1 = ((v2[1] - v0[1]) * (px - v2[0]) + (v0[0] - v2[0]) * (py - v2[1])) * inverse
                w2 = 1.0 - w0 - w1
                offset = py * width + px
                weights = [w0, w1, w2]
                if any(weight < -0.01 for weight in weights):
                    if filled[offset]:
                        continue
                    weights = [max(0.0, weight) for weight in weights]
                    total = sum(weights)
                    if total < 1e-6:
                        continue
                    weights = [weight / total for weight in weights]
                else:
                    filled[offset] = True
                u = weights[0] * v0[2] + weights[1] * v1[2] + weights[2] * v2[2]
                v = weights[0] * v0[3] + weights[1] * v1[3] + weights[2] * v2[3]
                tx = min(width_a - 1, max(0, int(u + 0.5)))
                ty = min(height_a - 1, max(0, int(v + 0.5)))
                image[py, px] = atlas[ty, tx]
    return image


def _pixel_bounds(vertices: list[list[float]], width: int, height: int) -> tuple[int, int, int, int] | None:
    if len(vertices) < 3:
        return None
    us = [float(v[2]) / 65535.0 * width for v in vertices]
    vs = [float(v[3]) / 65535.0 * height for v in vertices]
    x0 = max(0, math.floor(min(us)) - 1)
    y0 = max(0, math.floor(min(vs)) - 1)
    x1 = min(width - 1, math.floor(max(us) + 0.5) + 1)
    y1 = min(height - 1, math.floor(max(vs) + 0.5) + 1)
    if x1 < x0 or y1 < y0:
        return None
    return x0, y0, x1, y1


def repack_atlases(doc: dict, images: dict[int, np.ndarray]) -> tuple[dict, dict[int, np.ndarray], bool]:
    """Pack each used UV rectangle into a smaller page and rewrite normalized UVs.

    Returns the original images unchanged when the pack is not strictly smaller
    or when a re-rasterized command differs.
    """
    commands = []
    for shape in doc["shapes"]:
        for command in shape["commands"]:
            commands.append(command)
    placed: dict[int, list[tuple[tuple[int, int, int, int], int, int, int, int]]] = {}
    cursor: dict[int, list[int]] = {}
    seen: dict[tuple, tuple[int, int]] = {}
    for index in images:
        cursor[index] = [0, 0, 0, 0, 0]  # x, y, row_h, used_w, used_h
        placed[index] = []
    rect_of = []
    for command in commands:
        image = images[command["texture_index"]]
        bounds = _pixel_bounds(command["vertices"], image.shape[1], image.shape[0])
        if bounds is None:
            rect_of.append(None)
            continue
        key = (command["texture_index"], bounds)
        if key in seen:
            dest_x, dest_y = seen[key]
        else:
            width = bounds[2] - bounds[0] + 1
            height = bounds[3] - bounds[1] + 1
            foot_w = width + 2 * EDGE_PAD
            foot_h = height + 2 * EDGE_PAD
            x, y, row_h, used_w, used_h = cursor[command["texture_index"]]
            if x > 0 and x + foot_w > MAX_ATLAS:
                y += row_h + 1
                x = 0
                row_h = 0
            if y + foot_h > MAX_ATLAS or foot_w > MAX_ATLAS or foot_h > MAX_ATLAS:
                print(f"repack: command exceeds {MAX_ATLAS} ({foot_w}x{foot_h} at {x},{y})", flush=True)
                return doc, images, False
            dest_x, dest_y = x + EDGE_PAD, y + EDGE_PAD
            x = x + foot_w + 1
            row_h = max(row_h, foot_h)
            used_w = max(used_w, dest_x + width + EDGE_PAD)
            used_h = max(used_h, dest_y + height + EDGE_PAD)
            cursor[command["texture_index"]] = [x, y, row_h, used_w, used_h]
            placed[command["texture_index"]].append((bounds, dest_x, dest_y, width, height))
            seen[key] = (dest_x, dest_y)
        rect_of.append((bounds, dest_x, dest_y))
    rewritten = copy.deepcopy(doc)
    new_images = {}
    flat_commands = []
    for shape in rewritten["shapes"]:
        flat_commands.extend(shape["commands"])
    for index, image in images.items():
        used_w = max(1, cursor[index][3])
        used_h = max(1, cursor[index][4])
        if not placed[index]:
            canvas = np.zeros((1, 1, 4), np.uint8)
        else:
            canvas = np.zeros((used_h, used_w, 4), np.uint8)
            for bounds, dest_x, dest_y, width, height in placed[index]:
                src = image[bounds[1]:bounds[3] + 1, bounds[0]:bounds[2] + 1]
                padded = np.pad(src, ((EDGE_PAD, EDGE_PAD), (EDGE_PAD, EDGE_PAD), (0, 0)), mode="edge")
                origin_y = dest_y - EDGE_PAD
                origin_x = dest_x - EDGE_PAD
                canvas[origin_y:origin_y + padded.shape[0], origin_x:origin_x + padded.shape[1]] = padded
        new_images[index] = canvas
        texture = rewritten["textures"][index]
        texture["width"] = int(canvas.shape[1])
        texture["height"] = int(canvas.shape[0])
    for command, rect in zip(flat_commands, rect_of):
        if rect is None:
            continue
        bounds, dest_x, dest_y = rect
        old = images[command["texture_index"]]
        new = new_images[command["texture_index"]]
        old_w, old_h = old.shape[1], old.shape[0]
        new_w, new_h = new.shape[1], new.shape[0]
        src_x0, src_y0 = bounds[0], bounds[1]
        updated = []
        for vertex in command["vertices"]:
            old_u = float(vertex[2]) / 65535.0 * old_w
            old_v = float(vertex[3]) / 65535.0 * old_h
            new_u = (old_u - src_x0 + dest_x) / new_w * 65535.0
            new_v = (old_v - src_y0 + dest_y) / new_h * 65535.0
            updated.append([vertex[0], vertex[1], new_u, new_v])
        command["vertices"] = updated
    before = sum(image.shape[0] * image.shape[1] * 4 for image in images.values())
    after = sum(image.shape[0] * image.shape[1] * 4 for image in new_images.values())
    print(f"repack: RGBA {before} -> {after}", flush=True)
    if after >= before:
        return doc, images, False
    old_commands = [command for shape in doc["shapes"] for command in shape["commands"]]
    new_commands = [command for shape in rewritten["shapes"] for command in shape["commands"]]
    for nth, (old_command, new_command) in enumerate(zip(old_commands, new_commands)):
        before_image = rasterize_command(old_command["vertices"], images[old_command["texture_index"]])
        after_image = rasterize_command(new_command["vertices"], new_images[new_command["texture_index"]])
        if before_image.shape != after_image.shape or not np.array_equal(before_image, after_image):
            print(
                f"repack: raster mismatch on command {nth} texture {old_command['texture_index']}",
                flush=True,
            )
            return doc, images, False
    return rewritten, new_images, True


def _load_png(path: Path) -> np.ndarray:
    with Image.open(path) as image:
        return np.asarray(image.convert("RGBA"))


def _save_png(path: Path, image: np.ndarray) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    Image.fromarray(image, "RGBA").save(path, compress_level=9)


def atlas_path(root: Path, external: str) -> Path:
    return root / "atlases" / f"{Path(external).stem}.png"


def subset_tree_files(source: Path, dest: Path, doc_name: str, seeds: set[str]) -> dict:
    """Rewrite one display tree and its atlas pages in the derived bundle."""
    original = json.loads((source / "display-trees" / f"{doc_name}.json").read_text())
    before_bytes = (source / "display-trees" / f"{doc_name}.json").stat().st_size
    subset = subset_document(original, seeds)
    images = {}
    for index, texture in enumerate(original["textures"]):
        images[index] = _load_png(atlas_path(source, texture["external"]))
    before_rgba = {Path(texture["external"]).stem: int(images[i].shape[0] * images[i].shape[1] * 4)
                   for i, texture in enumerate(original["textures"])}
    packed, packed_images, cropped = repack_atlases(subset, images)
    target = dest / "display-trees" / f"{doc_name}.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(packed, separators=(",", ":")))
    for index, texture in enumerate(packed["textures"]):
        _save_png(atlas_path(dest, texture["external"]), packed_images[index])
    after_rgba = {Path(texture["external"]).stem: int(packed_images[i].shape[0] * packed_images[i].shape[1] * 4)
                  for i, texture in enumerate(packed["textures"])}
    return {
        "document": doc_name,
        "json_bytes_before": before_bytes,
        "json_bytes_after": target.stat().st_size,
        "exports_before": len(original["exports"]),
        "exports_after": len(packed["exports"]),
        "exports_missing": sorted(name for name in seeds if name not in original["exports"]),
        "atlas_cropped": cropped,
        "rgba_before": before_rgba,
        "rgba_after": after_rgba,
    }


def _link_or_copy(source: Path, dest: Path) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    if dest.exists() or dest.is_symlink():
        dest.unlink()
    try:
        os.link(source, dest)
    except OSError:
        shutil.copy2(source, dest)


def _coverage(engine: Path, assets: dict) -> dict:
    meta = []
    for doc in assets["documents"]:
        exports = doc.get("exports") or {}
        names = set(exports) if isinstance(exports, dict) else set(exports)
        meta.append({
            "name": doc["name"],
            "exports": names,
            "sha256": doc.get("sha256"),
            "original_entry": doc.get("original_entry"),
        })
    plan = plan_coverage(DEFAULT_LOGIC, DEFAULT_APK, engine, "", [])
    return finalize_coverage(plan, meta)


def _document_atlases(assets: dict, name: str) -> set[str]:
    record = next((doc for doc in assets["documents"] if doc["name"] == name), None)
    found = set()
    if not record:
        return found
    for atlas in record.get("atlases") or []:
        if isinstance(atlas, dict) and atlas.get("file"):
            found.add(atlas["file"])
    return found


def _present_documents(assets: dict, names: list[str]) -> list[str]:
    present = {doc["name"] for doc in assets["documents"]}
    return [name for name in names if name in present]


def derive_bundle(source: Path, dest: Path, engine: Path) -> dict:
    """Hardlink a bundle and write the scene index plus the ui/effects subset."""
    refuse_readonly(dest, engine)
    if dest.resolve() == source.resolve():
        raise RuntimeError("refusing to modify the source bundle")
    assets_path = source / "assets.json"
    assets = json.loads(assets_path.read_text())
    presentation = json.loads((source / "presentation.json").read_text())
    fingerprint = sha256(assets_path.read_bytes())
    print("coverage: walking logic and the apk", flush=True)
    coverage = _coverage(engine, assets)
    print(f"coverage: {len(coverage['card_dependency_documents'])} cards, "
          f"{len(coverage['always_documents'])} always documents", flush=True)
    present = {doc["name"] for doc in assets["documents"]}
    omitted = sorted(set(coverage["always_documents"]) - present)
    always = _present_documents(assets, list(coverage["always_documents"]))
    first = min(present)
    deviations = []
    if first not in always:
        always.append(first)
        deviations.append(
            f"{first} was added to always_documents because the Painter uses the alphabetically "
            "first loaded document as its font scene"
        )
    always = sorted(set(always))
    if omitted:
        deviations.append("always_documents names absent from this bundle were omitted: " + ", ".join(omitted))
    cards = {}
    omitted_cards = {}
    for name, docs in coverage["card_dependency_documents"].items():
        kept = _present_documents(assets, list(docs))
        missing = [doc for doc in docs if doc not in present]
        if missing:
            omitted_cards[name] = missing
        cards[name] = kept
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    replaced = {"assets.json", "display-trees/ui.json", "display-trees/effects.json"}
    replaced.update(atlas for doc_name in ("ui", "effects") for atlas in _document_atlases(assets, doc_name))
    for path in source.rglob("*"):
        if not path.is_file():
            continue
        relative = path.relative_to(source).as_posix()
        if relative in replaced:
            continue
        _link_or_copy(path, dest / relative)
    print("subset: ui", flush=True)
    ui_stats = subset_tree_files(source, dest, "ui", ui_seed_exports(assets))
    print("subset: effects", flush=True)
    effect_stats = subset_tree_files(source, dest, "effects", effects_seed_exports(presentation, assets))
    print({"ui_json": [ui_stats["json_bytes_before"], ui_stats["json_bytes_after"]],
           "effects_json": [effect_stats["json_bytes_before"], effect_stats["json_bytes_after"]],
           "ui_cropped": ui_stats["atlas_cropped"], "effects_cropped": effect_stats["atlas_cropped"]}, flush=True)
    _refresh_document_meta(assets, dest, "ui")
    _refresh_document_meta(assets, dest, "effects")
    assets.setdefault("scene", {})
    assets["scene"]["always_documents"] = always
    assets["scene"]["card_dependency_documents"] = cards
    portraits = _install_portraits(dest, engine, assets)
    (dest / "assets.json").write_text(json.dumps(assets, separators=(",", ":")))
    (dest / "portrait-map.json").write_text(json.dumps(portraits["paths"], sort_keys=True))
    return {
        "bundle_fingerprint": fingerprint,
        "always_documents": always,
        "cards": len(cards),
        "omitted_always_documents": omitted,
        "omitted_card_documents": omitted_cards,
        "deviations": deviations,
        "subset": {"ui": ui_stats, "effects": effect_stats},
        "portraits": portraits["copied"],
        "font_document": first,
    }


def _refresh_document_meta(assets: dict, dest: Path, name: str) -> None:
    doc = json.loads((dest / "display-trees" / f"{name}.json").read_text())
    record = next(item for item in assets["documents"] if item["name"] == name)
    record["exports"] = doc["exports"]
    record["atlases"] = []
    for texture in doc["textures"]:
        file_name = f"atlases/{Path(texture['external']).stem}.png"
        record["atlases"].append({
            "file": file_name,
            "width": texture["width"],
            "height": texture["height"],
            "decompressed_bytes": int(texture["width"]) * int(texture["height"]) * 4,
            "original_file": texture["external"],
        })
    if isinstance(record.get("counts"), dict):
        record["counts"]["exports"] = len(doc["exports"])
        record["counts"]["movies"] = len(doc["movies"])


def _install_portraits(dest: Path, engine: Path, assets: dict) -> dict:
    """Copy catalog portraits to the relative path the HUD opens on the bundle root."""
    catalog_path = engine / "data" / "card_catalog.json"
    wanted = {}
    visuals = assets.get("scene", {}).get("card_visuals") or {}
    rows = visuals.values() if isinstance(visuals, dict) else []
    for row in rows:
        native = row.get("native_card_row")
        portrait = (row.get("portrait") or {}).get("file")
        if native and isinstance(portrait, str):
            wanted[native] = portrait
    if catalog_path.is_file():
        for row in json.loads(catalog_path.read_text()):
            art = (row.get("art") or {}).get("path")
            native = row.get("sc_name")
            if native and art:
                wanted.setdefault(native, art)
    web = engine / "web"
    paths = {}
    for native, relative in wanted.items():
        source = web / relative
        if not source.is_file():
            continue
        _link_or_copy(source, dest / relative)
        paths[native] = relative
    return {"copied": len(paths), "paths": paths}


def document_files(assets: dict, name: str) -> list[str]:
    files = [f"display-trees/{name}.json"]
    record = next((doc for doc in assets["documents"] if doc["name"] == name), None)
    if record:
        for atlas in record.get("atlases") or []:
            if isinstance(atlas, dict) and atlas.get("file"):
                files.append(atlas["file"])
    return files


def manifest_lists(derived: Path) -> tuple[list[str], dict[str, list[str]], dict]:
    assets = json.loads((derived / "assets.json").read_text())
    scene = assets["scene"]
    always_docs = list(scene["always_documents"])
    card_docs = {name: list(docs) for name, docs in scene["card_dependency_documents"].items()}
    always = []
    for name in ("assets.json", "presentation.json"):
        always.append(name)
    font_dir = derived / "fonts"
    if font_dir.is_dir():
        always.extend(sorted(path.relative_to(derived).as_posix() for path in font_dir.iterdir() if path.is_file()))
    for name in always_docs:
        always.extend(document_files(assets, name))
    map_path = derived / "portrait-map.json"
    portraits = json.loads(map_path.read_text()) if map_path.is_file() else {}
    cards = {}
    for name, docs in card_docs.items():
        paths = []
        for doc in docs:
            paths.extend(document_files(assets, doc))
        portrait = portraits.get(name)
        if portrait:
            paths.append(portrait)
        cards[name] = sorted(set(paths))
    return sorted(set(always)), cards, assets


def _store_file(out: Path, data: bytes, ext: str, cache: dict[str, str]) -> tuple[str, int, str]:
    digest = sha256(data)
    name = f"{digest[:20]}{ext}"
    folder = out / "files"
    folder.mkdir(parents=True, exist_ok=True)
    if name not in cache:
        (folder / name).write_bytes(data)
        if ext == ".json":
            gz = gzip.compress(data, compresslevel=9)
            (folder / f"{name}.gz").write_bytes(gz)
            (folder / f"{name}.br").write_bytes(brotli_compress(data))
        cache[name] = digest
    return f"files/{name}", len(data), digest


def pack_directory(derived: Path, out: Path, bundle_fingerprint: str, painter_wasm: Path | None) -> dict:
    """Write the C1 content-addressed tree. JSON gets ``.gz`` and ``.br`` siblings."""
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    always, cards, _assets = manifest_lists(derived)
    needed = set(always)
    for paths in cards.values():
        needed.update(paths)
    cache: dict[str, str] = {}
    files = {}
    print(f"pack: {len(needed)} files", flush=True)
    for relative in sorted(needed):
        path = derived / relative
        if not path.is_file():
            raise FileNotFoundError(f"derived bundle is missing {relative}")
        ext = path.suffix.lower() or ".bin"
        url, size, digest = _store_file(out, path.read_bytes(), ext, cache)
        files[relative] = {"url": url, "bytes": size, "sha256": digest}
    wasm_url = None
    if painter_wasm is not None:
        if not painter_wasm.is_file():
            raise FileNotFoundError(painter_wasm)
        wasm_url, _size, _digest = _store_file(out, painter_wasm.read_bytes(), ".wasm", cache)
    manifest = {
        "v": 1,
        "bundle": bundle_fingerprint,
        "painter_wasm": wasm_url,
        "files": files,
        "always": always,
        "cards": {name: cards[name] for name in sorted(cards)},
    }
    body = json.dumps(manifest, sort_keys=True, separators=(",", ":")).encode() + b"\n"
    digest = sha256(body)
    manifest_name = f"manifest-{digest[:12]}.json"
    (out / manifest_name).write_bytes(body)
    (out / f"{manifest_name}.gz").write_bytes(gzip.compress(body, compresslevel=9))
    (out / f"{manifest_name}.br").write_bytes(brotli_compress(body))
    pointer = json.dumps({"current": manifest_name}, sort_keys=True, separators=(",", ":")).encode() + b"\n"
    (out / "manifest.json").write_bytes(pointer)
    (out / "manifest.json.gz").write_bytes(gzip.compress(pointer, compresslevel=9))
    (out / "manifest.json.br").write_bytes(brotli_compress(pointer))
    return json.loads(body)


def png_rgba_bytes(path: Path) -> int:
    with path.open("rb") as handle:
        if handle.read(8) != b"\x89PNG\r\n\x1a\n":
            raise ValueError(f"not a png: {path}")
        handle.read(4)
        if handle.read(4) != b"IHDR":
            raise ValueError(f"png missing IHDR: {path}")
        width, height = int.from_bytes(handle.read(4), "big"), int.from_bytes(handle.read(4), "big")
    return width * height * 4


def _wire_bytes(out: Path, record: dict, prefer_brotli: bool) -> int:
    url = record["url"]
    path = out / url
    if path.suffix == ".json":
        sibling = path.with_name(path.name + (".br" if prefer_brotli else ".gz"))
        return sibling.stat().st_size
    return record["bytes"]


def _cost_tables(out: Path, manifest: dict) -> dict:
    gzip_wire, brotli_wire, rgba = {}, {}, {}
    for relative, record in manifest["files"].items():
        gzip_wire[relative] = _wire_bytes(out, record, False)
        brotli_wire[relative] = _wire_bytes(out, record, True)
        rgba[relative] = png_rgba_bytes(out / record["url"]) if relative.endswith(".png") else 0
    pointer = out / "manifest.json"
    hashed = out / json.loads(pointer.read_text())["current"]
    overhead = {
        False: pointer.with_name(pointer.name + ".gz").stat().st_size + hashed.with_name(hashed.name + ".gz").stat().st_size,
        True: pointer.with_name(pointer.name + ".br").stat().st_size + hashed.with_name(hashed.name + ".br").stat().st_size,
    }
    wasm = out / manifest["painter_wasm"] if manifest.get("painter_wasm") else None
    return {"gzip": gzip_wire, "brotli": brotli_wire, "rgba": rgba, "overhead": overhead,
            "wasm": wasm.stat().st_size if wasm else 0}


def _pair_cost(manifest: dict, tables: dict, native_cards: list[str], prefer_brotli: bool) -> dict:
    paths = set(manifest["always"])
    missing = [card for card in native_cards if card not in manifest["cards"]]
    for card in native_cards:
        paths.update(manifest["cards"].get(card, ()))
    wire_table = tables["brotli"] if prefer_brotli else tables["gzip"]
    wire = sum(wire_table[relative] for relative in paths) + tables["overhead"][prefer_brotli] + tables["wasm"]
    rgba = sum(tables["rgba"][relative] for relative in paths)
    return {"wire_bytes": wire, "decoded_rgba_bytes": rgba, "files": len(paths), "missing_cards": missing}


def _worst_custom_pair(manifest: dict, tables: dict) -> dict:
    cards = list(manifest["cards"])
    selected: list[str] = []
    remaining = set(cards)

    def cost(names: list[str]) -> int:
        return _pair_cost(manifest, tables, names, False)["wire_bytes"]

    base = cost([])
    while len(selected) < 16 and remaining:
        best = max(remaining, key=lambda name: cost(selected + [name]) - base)
        selected.append(best)
        remaining.remove(best)
        base = cost(selected)
    improved = True
    while improved:
        improved = False
        current = cost(selected)
        for index, name in enumerate(list(selected)):
            for candidate in list(remaining):
                trial = list(selected)
                trial[index] = candidate
                trial_cost = cost(trial)
                if trial_cost > current:
                    remaining.add(name)
                    remaining.remove(candidate)
                    selected = trial
                    current = trial_cost
                    improved = True
                    break
            if improved:
                break
    left, right = selected[:8], selected[8:16]
    measured = _pair_cost(manifest, tables, selected, False)
    brotli = _pair_cost(manifest, tables, selected, True)
    return {
        "native_cards": [left, right],
        "unique_cards": selected,
        "wire_bytes": measured["wire_bytes"],
        "wire_bytes_if_brotli": brotli["wire_bytes"],
        "decoded_rgba_bytes": measured["decoded_rgba_bytes"],
        "files": measured["files"],
        "search": "greedy marginal gzip wire bytes, then pairwise swap to a local maximum among selectable cards",
    }


def measure_webp(out: Path, manifest: dict) -> dict:
    """Lossless WebP is measured and not shipped. C1 stores PNG bytes as they are."""
    png_bytes = 0
    webp_bytes = 0
    counted = 0
    for record in manifest["files"].values():
        if not record["url"].endswith(".png"):
            continue
        path = out / record["url"]
        png_bytes += path.stat().st_size
        print(f"webp: {path.name} {path.stat().st_size}", flush=True)
        with Image.open(path) as image:
            import io
            buffer = io.BytesIO()
            image.save(buffer, format="WEBP", lossless=True, quality=100, method=4)
            webp_bytes += buffer.tell()
        counted += 1
    return {
        "lossless": True,
        "pillow_method": 4,
        "png_files": counted,
        "png_bytes": png_bytes,
        "webp_bytes": webp_bytes,
        "shipped_format": "png",
        "note": "Lossless WebP was measured only. The pack stores PNG as C1 requires.",
    }


def size_report(derived: Path, out: Path, manifest: dict, derive_info: dict, presets_path: Path) -> dict:
    presets = json.loads(presets_path.read_text())["decks"]
    tables = _cost_tables(out, manifest)
    pairs = []
    for index, left in enumerate(presets):
        for right in presets[index:]:
            natives = list(dict.fromkeys(list(left["native_cards"]) + list(right["native_cards"])))
            gzip_cost = _pair_cost(manifest, tables, natives, False)
            brotli_cost = _pair_cost(manifest, tables, natives, True)
            pairs.append({
                "decks": [left["id"], right["id"]],
                "native_cards": [list(left["native_cards"]), list(right["native_cards"])],
                "wire_bytes": gzip_cost["wire_bytes"],
                "wire_bytes_if_brotli": brotli_cost["wire_bytes"],
                "decoded_rgba_bytes": gzip_cost["decoded_rgba_bytes"],
                "files": gzip_cost["files"],
                "missing_cards": gzip_cost["missing_cards"],
            })
    return {
        "generated_at": dt.datetime.now(dt.timezone.utc).isoformat(),
        "bundle_fingerprint": derive_info["bundle_fingerprint"],
        "painter_wasm": manifest["painter_wasm"],
        "painter_wasm_note": (
            "painter_wasm is null unless build_web_assets.py is given --painter-wasm PATH. "
            "That file is content-addressed to files/<sha256[:20]>.wasm. W1's wasm binary is not built by W2."
        ),
        "wire_encoding": "gzip siblings for JSON (the nginx gzip_static location). wire_bytes_if_brotli uses the .br siblings.",
        "subset": derive_info["subset"],
        "index": {
            "always_documents": derive_info["always_documents"],
            "cards": derive_info["cards"],
            "omitted_always_documents": derive_info["omitted_always_documents"],
            "font_document": derive_info["font_document"],
            "portraits_copied": derive_info["portraits"],
        },
        "deviations": derive_info["deviations"],
        "preset_pairs": pairs,
        "worst_custom_pair": _worst_custom_pair(manifest, tables),
        "webp": measure_webp(out, manifest),
        "selectable_cards": len(selectable_native_cards(presets_path.parent.parent)),
    }


def build(assets: Path, out: Path, derived: Path, engine: Path, painter_wasm: Path | None, report_path: Path, presets_path: Path) -> dict:
    refuse_readonly(out, engine)
    refuse_readonly(derived, engine)
    refuse_readonly(report_path, engine)
    info = derive_bundle(assets, derived, engine)
    manifest = pack_directory(derived, out, info["bundle_fingerprint"], painter_wasm)
    print("report: preset pairs, worst custom pair, lossless webp", flush=True)
    report = size_report(derived, out, manifest, info, presets_path)
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(json.dumps(report, indent=2) + "\n")
    return report
