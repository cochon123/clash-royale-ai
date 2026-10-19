#!/usr/bin/env python3
"""Extract original SC5 scene assets for the headless-engine spectator view.

This tool only unpacks the supplied APK, parses its original SC5 documents,
decodes their SCTX atlases, and writes provenance/binding metadata. It does not
create or modify artwork. The resulting directory uses native-ui-viewer's
display-trees/ and atlases/ input layout.
"""
from __future__ import annotations

import argparse
import csv
import dataclasses
import hashlib
import json
import re
import struct
import sys
import tomllib
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
from ui_recover import corrected_parser, decode_sctx  # noqa: E402
from sc5_parser._schemas.sc.flash.SC2.FileDescriptor import FileDescriptor  # noqa: E402

DEFAULT_APK = ROOT / "analysis/apks/install_time_asset_pack.apk"
DEFAULT_LOGIC = ROOT / "analysis/decoded/assets/csv_logic"
DEFAULT_LOCATIONS = ROOT / "analysis/decoded/assets/locations"
DEFAULT_OUTPUT = ROOT / "analysis/native-spectator"
DEFAULT_DOCS = [
    "arena_training", "building_tower", "building_mortar", "building_tombstone",
    "building_goblin_hut", "effects", "chr_knight", "chr_musketeer", "chr_skeleton",
    "chr_berserker", "chr_movingcannon", "chr_minion", "chr_ghost_dl",
    "chr_skeleton_king_dl", "level_bootcamp_arena_dl", "ui_spectate_hud",
]
CARD_ASSET_ALIASES = {
    "Mortar": "building_mortar", "Tombstone": "building_tombstone",
    "GoblinPartyHut": "building_goblin_hut", "Berserker": "chr_berserker",
    "MovingCannon": "chr_movingcannon", "Ghost": "chr_ghost_dl",
    "SkeletonKing": "chr_skeleton_king_dl", "MinionHorde": "chr_minion",
    "BarbLog": "effects", "GoblinCurse": "effects",
}
CARD_ASSET_PREFIXES = {
    "Mortar": ["building_mortar1", "building_mortar1_enemy"],
    "Tombstone": ["building_tombstone_blue", "building_tombstone_red"],
    "GoblinPartyHut": ["building_goblinHut_player", "building_goblinHut_enemy"],
}
CARD_ASSET_EXPORTS = {
    "BarbLog": ["log_projectile", "log_projectile_shadow", "log_roll_projectile", "log_roll_projectile_red"],
    "GoblinCurse": ["goblin_curse_ground", "goblin_curse_wave_export", "goblin_curse_bottle_drop",
                    "goblin_curse_dmg_amplification_spark"],
}
FIXED_TRAINING_CARDS = {
    "HogRider", "Musketeer", "IceGolemite", "Skeletons", "IceSpirits", "Cannon",
    "Fireball", "Log", "Mortar", "Knight", "Archer", "BarbLog", "Arrows",
    "MiniPekka", "Rocket",
}
PARSER_COMMIT = "9108080256a0df6cd772d81afdbc26047d79d78d"


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def write_json(path: Path, data) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    rows = list(csv.reader(path.open(newline="", encoding="utf-8-sig")))
    if len(rows) < 3:
        return [], []
    names = [name.strip() for name in rows[0]]
    records = []
    for values in rows[2:]:
        if not any(value.strip() for value in values):
            continue
        values += [""] * (len(names) - len(values))
        records.append({k: v.strip() for k, v in zip(names, values) if k})
    return names, records


def read_toml(path: Path) -> tuple[str, dict]:
    raw = path.read_bytes()
    return sha256(raw), tomllib.loads(raw.decode("utf-8"))


def training_objects(path: Path, decos: dict[str, dict]) -> tuple[dict, list[dict]]:
    rows = list(csv.reader(path.open(newline="", encoding="utf-8-sig")))
    bg = next((r for r in rows if len(r) > 2 and r[1].strip() == "sc/arena_training.sc"), None)
    background = None if not bg else {"file": bg[1].strip(), "document": "arena_training", "export": bg[2].strip()}
    objects = []
    current = None
    in_objects = False
    for row in rows:
        row += [""] * max(0, 4 - len(row))
        if row[0].strip() == "Objects":
            in_objects = True
            continue
        if not in_objects:
            continue
        name = row[1].strip()
        if name:
            if current:
                objects.append(current)
            current = {"native_location_row": name, "placements": []}
            continue
        if current and row[2].strip().lstrip("-+").isdigit():
            x = int(row[2].strip())
            y_text = row[3].strip()
            current["placements"].append({"x": x, "y": int(y_text) if y_text.lstrip("-+").isdigit() else None})
    if current:
        objects.append(current)
    for obj in objects:
        deco = decos.get(obj["native_location_row"])
        if deco:
            filename = Path(deco.get("FileName", "")).stem
            obj["asset"] = {
                "logic_file": "decos.csv",
                "document": filename or None,
                "export": deco.get("ExportName") or None,
                "scale_x_percent": int(deco["ShadowScaleX"]) if deco.get("ShadowScaleX", "").isdigit() else None,
                "scale_y_percent": int(deco["ShadowScaleY"]) if deco.get("ShadowScaleY", "").isdigit() else None,
                "origin_offset_x": int(deco["ShadowX"]) if deco.get("ShadowX", "").lstrip("-+").isdigit() else None,
                "origin_offset_y": int(deco["ShadowY"]) if deco.get("ShadowY", "").lstrip("-+").isdigit() else None,
                "layer": deco.get("Layer"),
            }
        elif obj["native_location_row"].startswith("foldable"):
            obj["asset"] = {"logic_file": "locations/training_arena.csv",
                            "document": "level_bootcamp_arena_dl",
                            "export": obj["native_location_row"],
                            "match_basis": "exact original location object/export name"}
        else:
            obj["asset"] = None
    return background or {}, objects


def animation_rows(sc) -> tuple[list[dict], dict]:
    movies = {str(k): v for k, v in sc.movie_clip_data.items()}
    rows = []
    for export, object_id in sorted(sc.exports.items()):
        movie = sc.movie_clip_data.get(object_id)
        if movie is None:
            continue
        labels = [{"frame": i, "name": label} for i, label in enumerate(movie.frame_labels) if label]
        direction = re.search(r"(?:_|-)([1-9])$", export)
        faction = "blue" if re.search(r"(?:^|_)(?:blue)(?:_|$)", export, re.I) else "red" if re.search(r"(?:^|_)(?:enemy|red)(?:_|$)", export, re.I) else None
        action = next((a for a in ("attack", "run", "idle", "walk", "deploy") if a in export.lower()), None)
        rows.append({"export": export, "object_id": object_id,
                     "frames": len(movie.frame_element_counts), "fps": movie.framerate,
                     "labels": labels, "direction_index": int(direction.group(1)) if direction else None,
                     "team_variant": faction, "action_token": action})
    return rows, movies


def direct_logic_bindings(logic_dir: Path, included_docs: set[str]) -> tuple[list[dict], dict]:
    sources = {}
    bindings = []
    paths = [logic_dir / filename for filename in ("characters_base.toml", "buildings.toml", "projectiles.toml", "spells_characters.toml")]
    paths.extend(sorted((logic_dir / "characters").rglob("*.toml")))
    for path in paths:
        filename = path.relative_to(logic_dir).as_posix()
        if path.exists():
            digest, data = read_toml(path)
            sources[filename] = digest
        else:
            data = {}
        for row_name, row in data.items():
            if not isinstance(row, dict) or not row.get("FileName"):
                continue
            doc = Path(row["FileName"]).stem
            if doc not in included_docs:
                continue
            exports = {key: row[key] for key in row if key.endswith("ExportName") or key.endswith("TopExportName")}
            bindings.append({"logic_file": filename, "row": row_name, "document": doc,
                             "source_file": row["FileName"], "exports": exports,
                             "scale_percent": row.get("Scale"),
                             "projectile": row.get("Projectile"),
                             "projectile_effect": row.get("ProjectileEffect"),
                             "spawn_character": row.get("SpawnCharacter")})
    return bindings, sources


def observed_character_rows(logic_dir: Path, names: set[str]) -> dict[str, dict]:
    """Resolve observed card rows through original nested CHARACTER tables."""
    found = {}
    for path in sorted((logic_dir / "characters").rglob("*.toml")):
        try:
            _, data = read_toml(path)
        except (OSError, tomllib.TOMLDecodeError):
            continue
        for table_name, tables in data.items():
            if not isinstance(tables, dict):
                continue
            for name in names:
                if name in found:
                    continue
                row = tables.get(name)
                matched_name = name
                if not isinstance(row, dict) and name.endswith("s"):
                    matched_name = name[:-1]
                    row = tables.get(matched_name)
                if isinstance(row, dict) and row.get("FileName"):
                    found[name] = {"logic_file": path.relative_to(logic_dir).as_posix(),
                                   "source_file": row["FileName"],
                                   "document": Path(row["FileName"]).stem,
                                   "blue_export": row.get("BlueExportName"),
                                   "red_export": row.get("RedExportName"),
                                   "scale_percent": row.get("Scale"),
                                   "source_row": f"{table_name}.{matched_name}"}
    return found


def observed_card_names(paths: list[Path]) -> set[str]:
    names = set()
    for path in paths:
        if not path.exists():
            continue
        data = json.loads(path.read_text())
        for item in data.get("requests", []):
            obs = item.get("expected", {}).get("observation", {})
            for hand in obs.get("hands", [])[:2]:
                for card in hand:
                    if card.get("name"):
                        names.add(card["name"])
    return names


def observed_entity_names(paths: list[Path]) -> set[str]:
    """Collect actual entity row names from recorded native battle frames."""
    names = set()
    for path in paths:
        if not path.exists():
            continue
        # Full spectator captures are JSONL frames; RL traces remain JSON
        # action records and intentionally do not supply guessed live entities.
        try:
            with path.open() as stream:
                for line in stream:
                    try:
                        frame = json.loads(line)
                    except json.JSONDecodeError:
                        continue
                    for entity in frame.get("entities", []):
                        if isinstance(entity, dict) and entity.get("row"):
                            names.add(str(entity["row"]))
        except OSError:
            continue
    return names


def observed_entity_bindings(logic_dir: Path, names: set[str]) -> list[dict]:
    """Resolve live rows only when the original logic row supplies an SC file/export."""
    paths = [logic_dir / filename for filename in ("characters_base.toml", "buildings.toml", "projectiles.toml", "spells_characters.toml")]
    paths.extend(sorted((logic_dir / "characters").rglob("*.toml")))
    found = {}
    for path in paths:
        try:
            _, data = read_toml(path)
        except (OSError, tomllib.TOMLDecodeError):
            continue
        for table_name, rows in data.items():
            if not isinstance(rows, dict):
                continue
            for name in names - found.keys():
                row = rows.get(name)
                if not isinstance(row, dict):
                    continue
                source_file = row.get("FileName")
                blue = row.get("BlueExportName") or row.get("ExportName")
                red = row.get("RedExportName") or row.get("BlueExportName") or row.get("ExportName")
                if source_file and blue:
                    found[name] = {
                        "native_row": name,
                        "logic_file": path.relative_to(logic_dir).as_posix(),
                        "source_row": f"{table_name}.{name}",
                        "source_file": source_file,
                        "document": Path(source_file).stem,
                        "blue_export": blue,
                        "red_export": red,
                        "scale_percent": row.get("Scale", 100),
                        "match_basis": "exact original nested logic row FileName and export fields",
                    }
    return [found[name] if name in found else {
        "native_row": name,
        "document": None,
        "blue_export": None,
        "red_export": None,
        "match_basis": "no original nested logic row supplied an SC document and export",
    } for name in sorted(names)]


def observed_decks(paths: list[Path], included_docs: set[str], character_rows: dict[str, dict] | None = None) -> tuple[dict, list[dict]]:
    character_rows = character_rows or {}
    decks: dict[int, dict[tuple[int, str], None]] = {0: {}, 1: {}}
    hashes = []
    for path in paths:
        if not path.exists():
            continue
        data = json.loads(path.read_text())
        hashes.append({"file": path.relative_to(ROOT).as_posix(), "sha256": sha256(path.read_bytes())})
        for item in data.get("requests", []):
            obs = item.get("expected", {}).get("observation", {})
            hands = obs.get("hands", [])
            for side, hand in enumerate(hands[:2]):
                for card in hand:
                    name = card.get("name")
                    card_id = card.get("card_id")
                    if name and isinstance(card_id, int):
                        decks[side][(card_id, name)] = None
    rows = []
    for side, cards in decks.items():
        for card_id, name in sorted(cards):
            doc = CARD_ASSET_ALIASES.get(name) or character_rows.get(name, {}).get("document")
            source_row = character_rows.get(name)
            prefixes = CARD_ASSET_PREFIXES.get(name, [])
            if source_row:
                prefixes = [v for v in (source_row.get("blue_export"), source_row.get("red_export")) if v]
            rows.append({"side": side, "card_id": card_id, "native_card_row": name,
                         "document": doc if doc in included_docs else None,
                         "export_prefixes": prefixes,
                         "export_candidates": CARD_ASSET_EXPORTS.get(name, []),
                         "character_logic_binding": source_row,
                         "asset_match_basis": ("nested original CHARACTER row with FileName and team exports" if source_row else "repository card-to-SC asset alias")
                         if doc in included_docs else "no original SC5 document selected by a direct source row or repository alias"})
    return {"cards_observed_across_traces": rows, "trace_sources": hashes}, hashes


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--apk", type=Path, default=DEFAULT_APK)
    ap.add_argument("--logic", type=Path, default=DEFAULT_LOGIC)
    ap.add_argument("--locations", type=Path, default=DEFAULT_LOCATIONS)
    ap.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    ap.add_argument("--docs", nargs="+", default=DEFAULT_DOCS, help="SC document stems to extract")
    args = ap.parse_args()
    apk = args.apk.resolve()
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=True)
    trace_paths = sorted((ROOT / "analysis/native-port").glob("rl-trace-seed[0-9]*.json"))
    character_rows = observed_character_rows(args.logic, observed_card_names(trace_paths) | FIXED_TRAINING_CARDS)
    for binding in character_rows.values():
        if binding["document"] not in args.docs:
            args.docs.append(binding["document"])
    battle_frames = sorted((ROOT / "runs").glob("spectator-*.jsonl"))
    live_names = observed_entity_names(battle_frames)
    live_bindings = observed_entity_bindings(args.logic, live_names)
    for binding in live_bindings:
        doc = binding.get("document")
        if doc and doc not in args.docs:
            args.docs.append(doc)
    sc_parser = corrected_parser()
    apk_bytes = apk.read_bytes()
    apk_hash = sha256(apk_bytes)
    bindings = []
    input_hashes = {}
    with zipfile.ZipFile(apk) as archive:
        names = archive.namelist()
        by_basename = {Path(n).name: n for n in names if n.startswith("assets/sc/")}
        docs_meta = []
        for stem in args.docs:
            sc_name = f"{stem}.sc"
            entry = by_basename.get(sc_name)
            if not entry:
                raise SystemExit(f"Original SC document missing from APK: {sc_name}")
            source_path = out / "source" / sc_name
            source_path.parent.mkdir(parents=True, exist_ok=True)
            source_path.write_bytes(archive.read(entry))
            source_hash = sha256(source_path.read_bytes())
            input_hashes[entry] = source_hash
            raw_sc = source_path.read_bytes()
            fd_size = struct.unpack_from("<I", raw_sc, 6)[0]
            descriptor = FileDescriptor.GetRootAs(raw_sc[10 : 10 + fd_size], 0)
            sc = sc_parser(source_path)
            texture_meta = []
            for texture in sc.textures:
                atlas_name = texture["external"]
                atlas_entry = by_basename.get(atlas_name)
                if not atlas_entry:
                    raise SystemExit(f"Original atlas missing from APK: {atlas_name} (from {sc_name})")
                atlas_data = archive.read(atlas_entry)
                input_hashes[atlas_entry] = sha256(atlas_data)
                atlas_source = out / "source" / atlas_name
                atlas_source.write_bytes(atlas_data)
                image, meta = decode_sctx(atlas_source)
                target = out / "atlases" / f"{Path(atlas_name).stem}.png"
                target.parent.mkdir(parents=True, exist_ok=True)
                image.save(target)
                texture_meta.append({**meta, "file": target.relative_to(out).as_posix(), "original_file": atlas_name})
            doc = {"source": f"source/{sc_name}", "sha256": source_hash, "exports": sc.exports,
                   "movies": {str(k): dataclasses.asdict(v) for k, v in sc.movie_clip_data.items()},
                   "text_fields": {str(k): dataclasses.asdict(v) for k, v in sc.text_fields.items()},
                   "shapes": sc.shapes, "vertices": sc.vertices, "modifiers": sc.modifiers,
                   "matrix_banks": [[dataclasses.asdict(m) for m in b] for b in sc.matrix_banks],
                   "color_banks": [[dataclasses.asdict(c) for c in b] for b in sc.color_banks],
                   "frame_elements": sc._frame_elements.tolist(), "textures": sc.textures}
            write_json(out / "display-trees" / f"{stem}.json", doc)
            animations, _ = animation_rows(sc)
            docs_meta.append({"name": stem, "original_entry": entry, "sha256": source_hash,
                              "counts": {"exports": len(sc.exports), "movies": len(sc.movie_clip_data),
                                         "shapes": len(sc.shapes), "vertices": len(sc.vertices),
                                         "matrix_banks": [len(x) for x in sc.matrix_banks],
                                         "color_banks": [len(x) for x in sc.color_banks]},
                              "external_matrix_bank_size": descriptor.ExternalMatrixBankSize() or 0,
                              "atlases": texture_meta, "animation_exports": animations})

        # Original bundled fonts are copied byte-for-byte for the later text pass.
        font_entries = [n for n in names if n.startswith("assets/font/") and n.lower().endswith((".ttf", ".otf"))]
        for entry in font_entries:
            target = out / "fonts" / Path(entry).name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(archive.read(entry))
            input_hashes[entry] = sha256(target.read_bytes())

    bindings, logic_hashes = direct_logic_bindings(args.logic, set(args.docs))
    decos_path = args.logic / "decos.csv"
    _, decos_rows = read_csv(decos_path)
    decos = {row["Name"]: row for row in decos_rows if row.get("Name")}
    location_path = args.locations / "training_arena.csv"
    background, placements = training_objects(location_path, decos)
    location_sources = {}
    for path in [args.logic / "testing.csv", args.logic / "locations.csv", args.logic / "globals.csv", args.logic / "buildings.toml",
                 args.logic / "characters_base.toml", args.logic / "projectiles.toml", decos_path, location_path]:
        if path.exists():
            location_sources[path.relative_to(ROOT).as_posix() if path.is_relative_to(ROOT) else path.name] = sha256(path.read_bytes())
    for name, digest in logic_hashes.items():
        source_path = args.logic / name
        key = source_path.relative_to(ROOT).as_posix() if source_path.is_relative_to(ROOT) else name
        location_sources[key] = digest
    _, test_rows = read_csv(args.logic / "testing.csv")
    normal_test = next((row for row in test_rows if row.get("Name") == "Normal"), None)
    _, location_rows = read_csv(args.logic / "locations.csv")
    training_location = {row["Name"]: row for row in location_rows if row.get("Name") in ("TrainingCamp", "FirstMatch")}
    _, global_rows = read_csv(args.logic / "globals.csv")
    starting_arena = next((row for row in global_rows if row.get("Name") == "STARTING_ARENA"), None)
    deck_metadata, trace_hashes = observed_decks(trace_paths, set(args.docs), character_rows)
    bg_render_path = out / "training-area-bg-render.json"
    bg_render = json.loads(bg_render_path.read_text()) if bg_render_path.exists() else None
    manifest = {
        "format": "native-spectator-sc5-assets-v1",
        "input_apk": str(apk.relative_to(ROOT) if apk.is_relative_to(ROOT) else apk),
        "input_apk_sha256": apk_hash,
        "parser_commit": PARSER_COMMIT,
        "parser_corrections": ["SC string references use verified zero-based indices", "SCTX ASTC 6x6 support from existing decoder"],
        "scene": {"normal_testing_row": normal_test, "training_location_rows": training_location,
                  "starting_arena_global": starting_arena,
                  "location_source": location_path.relative_to(ROOT).as_posix() if location_path.is_relative_to(ROOT) else str(location_path),
                  "location_source_sha256": sha256(location_path.read_bytes()),
                  "background": background, "objects": placements,
                  "observed_decks": deck_metadata,
                  "observed_entity_bindings": live_bindings,
                  "observed_frame_sources": [
                      {"file": path.relative_to(ROOT).as_posix(), "sha256": sha256(path.read_bytes())}
                      for path in battle_frames
                  ],
                  "fixed_training_deck_assets": [
                      {"native_card_row": name, **binding}
                      for name, binding in sorted(character_rows.items())
                      if name in FIXED_TRAINING_CARDS and binding["document"] in args.docs
                  ],
                  "camera_calibration": {
                      "status": "asset-calibrated presentation approximation; original camera routine has not been recovered",
                      "field_world_bounds": {"x_min": 0, "x_max": 18000, "y_min": 0, "y_max": 32000},
                      "observed_native_projection": {"sc_x_approx": "(world_x - 9000) * 0.03", "sc_y_approx": "(32000 - world_y) * 0.024"},
                      "painter_fit": {"x": "world_x / 18000 across fitted field width", "y": "(32000 - world_y) / 32000 across fitted field height", "blue_side": "lower field"},
                      "basis": "training_area_bg rendered bounds and native tower coordinates; this mapping is not claimed to reproduce the game's exact camera transform",
                  },
                  "background_render_reference": {
                      "file": bg_render_path.relative_to(out).as_posix(),
                      "image": "training-area-bg.png",
                      "render": bg_render.get("render") if bg_render else None,
                      "original_sc_sha256": bg_render.get("original_sc_sha256") if bg_render else None,
                  },
                  "coordinate_note": "Original location CSV coordinates are preserved. Painter camera mapping is an asset-calibrated approximation recorded in camera_calibration, not source-verified game camera behavior."},
        "bindings": bindings,
        "documents": docs_meta,
        "logic_source_hashes": location_sources,
        "extracted_source_hashes": input_hashes,
        "trace_source_hashes": trace_hashes,
        "fonts": [{"file": Path(n).name, "sha256": input_hashes[n]} for n in font_entries],
        "renderer_input_layout": {"display_trees": "display-trees/", "atlases": "atlases/", "fonts": "fonts/"},
        "limits": ["Only listed original SC5 documents are extracted.",
                   "The source SC5 documents carry no external matrix banks; compressed external-matrix data is not used by this selection.",
                   "Native gameplay state, model selection, projection, and animation binding are supplied by the engine/session; this bundle infers none of them."],
    }
    write_json(out / "assets.json", manifest)
    print(json.dumps({"output": str(out), "apk_sha256": apk_hash,
                      "documents": [{"name": d["name"], "exports": d["counts"]["exports"],
                                     "movies": d["counts"]["movies"], "atlases": len(d["atlases"])} for d in docs_meta],
                      "placements": len(placements), "bindings": len(bindings),
                      "observed_live_rows": len(live_bindings), "fonts": len(font_entries)}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
