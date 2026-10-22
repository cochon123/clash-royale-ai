#!/usr/bin/env python3
"""Recover original Clash SC5 display trees, atlas textures and sample UI frames.

Run with the project-local environment installed by ui_setup.sh. This decodes
supplied assets; it does not emulate or launch the original native client.
"""
from __future__ import annotations

import argparse
import dataclasses
import hashlib
import importlib.metadata
import inspect
import json
import lzma
import math
from pathlib import Path
import re
import sqlite3
import struct
import zipfile

from PIL import Image, ImageDraw
import sc5_parser.parser as parser_module
import texture2ddecoder
import zstandard


PARSER_COMMIT = "9108080256a0df6cd772d81afdbc26047d79d78d"
ASSETS = ["ui", "ui_ability_button", "ui_deck_builder", "ui_spectate_hud", "ui_battle_end", "ui_arena"]


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def corrected_parser():
    """Correct the upstream's 1-based references using the primary C++ reader.

    SupercellFlash/flash/objects/ExportName.cpp uses strings_vector->Get(ref),
    likewise TextField.cpp and MovieClip.cpp. Our supplied files also have an
    explicit empty string at index zero. Keep the installed package untouched.
    """
    source = inspect.getsource(parser_module)
    source, lookups = re.subn(r"self\.strings\[(\w+) - 1\]", r"self.strings[\1]", source)
    source, bounds = re.subn(r"0 < (\w+) <= len\(self\.strings\)", r"0 <= \1 < len(self.strings)", source)
    if (lookups, bounds) != (6, 6):
        raise RuntimeError(f"Pinned parser changed: expected 6 reference repairs, got {lookups}/{bounds}")
    exec(compile(source, parser_module.__file__, "exec"), parser_module.__dict__)
    return parser_module.SC5File


def decode_sctx(path: Path):
    """Decode the supplied SCTX container, including its ASTC 6x6 format.

    ASTC enum mapping: sc-workshop/SupercellTexture, ScPixel.hpp. Texture
    blocks are decoded by texture2ddecoder, not a reconstructed image model.
    """
    data = path.read_bytes()
    streaming_size = struct.unpack_from("<I", data, 0)[0]
    header_size = struct.unpack_from("<I", data, 4)[0]
    pixel_type, width, height = struct.unpack_from("<IHH", data, 8 + header_size)
    data_offset = 4 + streaming_size
    data_size = struct.unpack_from("<I", data, data_offset)[0]
    compressed_offset = data_offset + 4 + data_size
    raw = zstandard.ZstdDecompressor().decompress(data[compressed_offset:], max_output_size=width * height * 8)
    blocks = {204: (4, 4), 205: (5, 4), 206: (5, 5), 207: (6, 5), 208: (6, 6), 210: (8, 5), 211: (8, 6), 212: (8, 8)}
    if pixel_type == 70:
        pixels = raw
    elif pixel_type in blocks:
        bx, by = blocks[pixel_type]
        expected = ((width + bx - 1) // bx) * ((height + by - 1) // by) * 16
        if len(raw) != expected:
            raise ValueError(f"Unexpected ASTC payload size in {path}: {len(raw)} != {expected}")
        pixels = texture2ddecoder.decode_astc(raw, width, height, bx, by)
    else:
        raise ValueError(f"Unimplemented SCTX pixel type {pixel_type} in {path}")
    image = Image.frombytes("RGBA", (width, height), pixels, "raw", "BGRA")
    return image, {"pixel_type": pixel_type, "width": width, "height": height, "decompressed_bytes": len(raw), "source_sha256": sha256(data)}


def write_json(path: Path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n")


def recover_database(db: Path, output: Path):
    connection = sqlite3.connect(f"file:{db}?mode=ro", uri=True)
    rows = connection.execute("""select a.path,m.assetType,a.format,a.data,a.extref
        from asset a join meta m using(path)
        where m.assetType in ('AssetUISchema','AssetUITimeline') order by a.path""")
    index = []
    for asset_path, asset_type, fmt, data, external in rows:
        row = {"path": asset_path, "type": asset_type, "format": fmt, "external": external}
        if data is not None:
            data = bytes(data)
            row.update(bytes=len(data), sha256=sha256(data))
            try:
                parsed = json.loads(data)
                target = output / "schemas" / f"{asset_path}.json"
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(data)  # Preserve the exact original JSON bytes.
                row["decoded"] = True
            except (UnicodeDecodeError, json.JSONDecodeError):
                row["decoded"] = False
        index.append(row)
    write_json(output / "ui-schema-index.json", index)
    return index


def render_samples(name, sc, textures, output):
    selected = {
        "ui": ["HUD_player", "HUD_topLeft", "HUD_topRight", "HUD_topRight_double_elixir", "UI_menu_background", "ability_ingame_button", "hp_bar_player", "hp_bar_enemy"],
        "ui_deck_builder": ["deck_builder_top_tabs", "deck_builder_bottom", "card_slot", "button_deck_wizard", "card_evolution_slot"],
        "ui_spectate_hud": ["spectate_hud_1v1_player", "spectate_hud_1v1_enemy", "panel_replay"],
        "ui_ability_button": ["ability_ingame_button"],
        "ui_battle_end": [],
        "ui_arena": [],
    }[name]
    results = []
    for export in selected:
        if export not in sc.exports:
            results.append({"export": export, "status": "not present"})
            continue
        frame = 32 if name == "ui_ability_button" else 0
        target = output / "rendered" / name / f"{export}.png"
        target.parent.mkdir(parents=True, exist_ok=True)
        try:
            image = sc.extract_sprite(export, textures, str(target), frame_index=frame)
            results.append({"export": export, "frame": frame, "status": "rendered" if image else "no visible shapes", "size": image.size if image else None})
        except Exception as error:
            results.append({"export": export, "status": "renderer error", "error": str(error)})
    return results


def render_ability_animation(sc, textures, output):
    """Render the root timeline at the actual encoded frame rate.

    Descendant movie clips stay in their default state. Game code drives
    their named state selectors independently; no inferred transitions here.
    """
    name = "ability_ingame_button"
    mc = sc.movie_clip_data[sc.exports[name]]
    frames = [sc.extract_sprite_with_offset(name, textures, frame_index=i) for i in range(len(mc.frame_element_counts))]
    visible = [f for f in frames if f is not None]
    left = math.floor(min(f[1] for f in visible))
    top = math.floor(min(f[2] for f in visible))
    right = math.ceil(max(f[1] + f[0].width for f in visible))
    bottom = math.ceil(max(f[2] + f[0].height for f in visible))
    aligned = []
    for f in frames:
        canvas = Image.new("RGBA", (right - left, bottom - top))
        if f:
            canvas.alpha_composite(f[0], (round(f[1]) - left, round(f[2]) - top))
        aligned.append(canvas)
    target = output / "rendered" / "ui_ability_button" / "ability_ingame_button.webp"
    aligned[0].save(target, save_all=True, append_images=aligned[1:], duration=round(1000 / mc.framerate), lossless=True, loop=0)
    return {"path": str(target), "frames": len(frames), "framerate": mc.framerate, "bounds": [left, top, right, bottom], "labels": {i: label for i, label in enumerate(mc.frame_labels) if label}, "descendants": "default frame only"}


def make_preview(output):
    chosen = [
        ("Player hand / elixir display", "ui/HUD_player.png"),
        ("Spectator HUD", "ui_spectate_hud/spectate_hud_1v1_player.png"),
        ("Replay panel", "ui_spectate_hud/panel_replay.png"),
        ("Deck tabs", "ui_deck_builder/deck_builder_top_tabs.png"),
        ("Card slot", "ui_deck_builder/card_slot.png"),
        ("Ability button", "ui_ability_button/ability_ingame_button.png"),
    ]
    canvas = Image.new("RGB", (1100, 1000), "#141e2d")
    draw = ImageDraw.Draw(canvas)
    draw.text((32, 20), "Original Clash Royale assets decoded from the supplied APK", fill="white", font_size=24)
    draw.text((32, 55), "Static display objects: runtime text, card portraits and game-state bindings are absent.", fill="#b2c3d5", font_size=17)
    positions = [(28, 100, 1030, 225), (28, 360, 1030, 210), (28, 610, 650, 150), (28, 810, 650, 100), (730, 650, 170, 260), (930, 650, 140, 180)]
    for (title, rel), (x, y, width, height) in zip(chosen, positions):
        image = Image.open(output / "rendered" / rel).convert("RGBA")
        image.thumbnail((width, height))
        draw.text((x, y - 25), title, fill="#b2c3d5", font_size=17)
        canvas.paste(image, (x + (width - image.width) // 2, y), image)
    canvas.save(output / "recovered-controls.png")


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--apk", type=Path, default=Path("analysis/apks/install_time_asset_pack.apk"))
    ap.add_argument("--output", type=Path, default=Path("analysis/native-ui"))
    ap.add_argument("--assets", nargs="+", default=ASSETS)
    args = ap.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    sources = args.output / "source"
    sources.mkdir(exist_ok=True)
    SC5File = corrected_parser()
    manifest = {"input": str(args.apk), "input_sha256": sha256(args.apk.read_bytes()), "parser_commit": PARSER_COMMIT,
                "packages": {n: importlib.metadata.version(n) for n in ["sc5-parser", "flatbuffers", "numpy", "pillow", "texture2ddecoder", "zstandard"]},
                "corrections": ["zero-based SC string references", "ASTC 6x6 pixel type 208"], "assets": []}
    with zipfile.ZipFile(args.apk) as archive:
        for entry in archive.namelist():
            filename = Path(entry).name
            if any(filename == f"{n}.sc" or (filename.startswith(n + "_") and filename[len(n) + 1:].split(".")[0].isdigit() and filename.endswith(".sctx")) for n in args.assets):
                (sources / filename).write_bytes(archive.read(entry))
            if entry.startswith("assets/font/") and entry.endswith((".ttf", ".otf")):
                target = args.output / "fonts" / filename
                target.parent.mkdir(exist_ok=True)
                target.write_bytes(archive.read(entry))
        db = args.output / "assets.scdb"
        if not db.exists():
            db.write_bytes(archive.read("assets/assets.scdb"))
        manifest["database_sha256"] = sha256(db.read_bytes())
        schema_index = recover_database(db, args.output)
        manifest["ui_schema_counts"] = {t: sum(row["type"] == t for row in schema_index) for t in ["AssetUISchema", "AssetUITimeline"]}
    for name in args.assets:
        print(f"Decoding {name}...", flush=True)
        path = sources / f"{name}.sc"
        sc = SC5File(path)
        textures, texture_metadata = [], []
        for texture in sc.textures:
            texture_path = sources / texture["external"]
            image, metadata = decode_sctx(texture_path)
            target = args.output / "atlases" / (texture_path.stem + ".png")
            target.parent.mkdir(parents=True, exist_ok=True)
            image.save(target)
            textures.append(image)
            texture_metadata.append(metadata)
        document = {"source": str(path), "sha256": sha256(path.read_bytes()), "exports": sc.exports,
                    "movies": {k: dataclasses.asdict(v) for k, v in sc.movie_clip_data.items()},
                    "text_fields": {k: dataclasses.asdict(v) for k, v in sc.text_fields.items()},
                    "shapes": sc.shapes, "vertices": sc.vertices,
                    "modifiers": sc.modifiers,
                    "matrix_banks": [[dataclasses.asdict(m) for m in b] for b in sc.matrix_banks],
                    "color_banks": [[dataclasses.asdict(c) for c in b] for b in sc.color_banks],
                    "frame_elements": sc._frame_elements.tolist(), "textures": sc.textures}
        write_json(args.output / "display-trees" / f"{name}.json", document)
        result = {"name": name, "sha256": document["sha256"], "shapes": len(sc.shapes), "movies": len(sc.movie_clip_data), "exports": len(sc.exports),
                  "text_fields": len(sc.text_fields), "textures": texture_metadata, "renders": render_samples(name, sc, textures, args.output)}
        if name == "ui_ability_button":
            result["animation"] = render_ability_animation(sc, textures, args.output)
        manifest["assets"].append(result)
        write_json(args.output / "recovery-manifest.json", manifest)
    if set(["ui", "ui_deck_builder", "ui_spectate_hud", "ui_ability_button"]).issubset(args.assets):
        make_preview(args.output)
    print(json.dumps({"output": str(args.output), "ui_schema_counts": manifest["ui_schema_counts"], "assets": [{k: a[k] for k in ["name", "shapes", "movies", "exports"]} for a in manifest["assets"]]}, indent=2))


if __name__ == "__main__":
    main()
