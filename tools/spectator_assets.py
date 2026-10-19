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
from PIL import Image, ImageDraw, ImageFont

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
    "chr_skeleton_king_dl", "chr_king", "chr_princess", "level_bootcamp_arena_dl", "ui_spectate_hud", "ui",
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
TOWER_ATTACHED_ROWS = {"TowerPrincess"}
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
                                   "source_fields": {key: value for key, value in row.items()
                                                     if key in {"Projectile", "ProjectileEffect", "HitEffect", "DeathEffect",
                                                                "DamageEffect", "AttackStartEffect", "SpawnEffect", "MoveEffect",
                                                                "DeployBaseAnimExportName", "AttachedCharacter",
                                                                "AttachedCharacterHeight", "BlueTopExportName", "RedTopExportName",
                                                                "use360Frames", "Gravity", "Speed", "Homing", "OnlyEnemies",
                                                                "ProjectileRange", "AoeToAir", "AoeToGround"}},
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
                    source_fields = {
                        key: value for key, value in row.items()
                        if key in {
                            "BlueTopExportName", "RedTopExportName", "Scale", "ScaleX", "ScaleY",
                            "ShadowX", "ShadowY", "ShadowScaleX", "ShadowScaleY", "ShadowSkew",
                            "Projectile", "ProjectileEffect", "HitEffect", "DeathEffect",
                            "DamageEffect", "AttackStartEffect", "SpawnEffect", "MoveEffect",
                            "DeployBaseAnimExportName", "AttachedCharacter", "AttachedCharacterHeight",
                            "use360Frames", "Gravity", "Speed", "Homing", "OnlyEnemies",
                            "ProjectileRange", "AoeToAir", "AoeToGround",
                        }
                    }
                    found[name] = {
                        "native_row": name,
                        "logic_file": path.relative_to(logic_dir).as_posix(),
                        "source_row": f"{table_name}.{name}",
                        "source_file": source_file,
                        "document": Path(source_file).stem,
                        "blue_export": blue,
                        "red_export": red,
                        "scale_percent": row.get("Scale", 100),
                        "source_fields": source_fields,
                        "match_basis": "exact original nested logic row FileName and export fields",
                    }
    return [found[name] if name in found else {
        "native_row": name,
        "document": None,
        "blue_export": None,
        "red_export": None,
        "match_basis": "no original nested logic row supplied an SC document and export",
    } for name in sorted(names)]


def card_visual_bindings(card_catalog_path: Path, card_names: set[str], bundle_dir: Path) -> list[dict]:
    """Join live card rows to original image and card-frame exports by source record."""
    catalog = json.loads(card_catalog_path.read_text())
    by_sc_name = {str(card.get("sc_name")): card for card in catalog if card.get("sc_name")}
    by_id = {str(card.get("id")): card for card in catalog if card.get("id")}
    rows = []
    frame_by_rarity = {
        "Common": "card_item_frame", "Rare": "card_item_frame",
        "Epic": "card_item_frame", "Legendary": "card_item_frame_legendary",
        "Champion": "card_item_frame_champion",
    }
    for native_row in sorted(card_names):
        card = by_sc_name.get(native_row)
        if not card:
            # A few server card rows are presentation aliases; only join by known catalog id/name.
            card = next((v for v in catalog if v.get("name") == native_row), None)
        if not card:
            rows.append({"native_card_row": native_row, "catalog_match": None,
                         "match_basis": "no exact SC name or display-name match in recovered card catalog"})
            continue
        art = card.get("art") or {}
        art_rel = art.get("path")
        source_image = ROOT / "web" / art_rel if art_rel else None
        image_sha = sha256(source_image.read_bytes()) if source_image and source_image.is_file() else None
        bundle_image = bundle_dir / "cards" / Path(art_rel).name if image_sha and art_rel else None
        if bundle_image is not None:
            bundle_image.parent.mkdir(parents=True, exist_ok=True)
            bundle_image.write_bytes(source_image.read_bytes())
        rows.append({
            "native_card_row": native_row,
            "card_id": card.get("id"),
            "display_name": card.get("name"),
            "rarity": card.get("rarity"),
            "cost": card.get("elixir"),
            "elixir_cost": card.get("elixir"),
            "portrait": {
                "original_asset_path": art.get("source"),
                "recovered_path": art_rel,
                "file": (Path("cards") / Path(art_rel).name).as_posix() if image_sha else None,
                "source_repository_file": (Path("web") / art_rel).as_posix() if image_sha else None,
                "sha256": image_sha,
                "catalog_sha256": art.get("sha256"),
                "hash_matches_catalog": bool(image_sha and image_sha == art.get("sha256")),
                "original_record": card.get("source"),
            },
            "card_frame_export": frame_by_rarity.get(card.get("rarity")),
            "elixir_icon_export": "card_item_elixir_ingame",
            "portrait_slot_export": "card_item_image",
            "ui_document": "ui",
            "ui": {"document": "ui", "frame_export": frame_by_rarity.get(card.get("rarity")),
                   "elixir_export": "card_item_elixir_ingame", "portrait_slot_export": "card_item_image",
                   "frame_selection": "generic native card_item_frame for Common/Rare/Epic; explicit rarity frame for Legendary/Champion"},
            "match_basis": "exact recovered catalog sc_name and source CSV record",
        })
    return rows


def animation_bindings(document_meta: list[dict], entity_rows: list[dict]) -> dict[str, list[dict]]:
    """Index exact SC exports by observed native row, keeping original names and provenance."""
    docs = {doc["name"]: doc for doc in document_meta}
    result = {}
    for entity in entity_rows:
        doc_name = entity.get("document")
        if not doc_name or doc_name not in docs:
            result[entity["native_row"]] = {"document": doc_name, "exports": [],
                                           "reason": "no selected original SC document resolves this native row"}
            continue
        prefixes = {entity.get("blue_export"), entity.get("red_export")}
        prefixes.discard(None)
        exports = []
        for row in docs[doc_name].get("animation_exports", []):
            name = row["export"]
            if any(name == prefix or name.startswith(prefix + "_") for prefix in prefixes):
                exports.append({key: row[key] for key in
                                ("export", "action_token", "direction_index", "team_variant", "fps", "frames", "labels")})
        result[entity["native_row"]] = {
            "document": doc_name,
            "source_file": entity.get("source_file"),
            "source_row": entity.get("source_row"),
            "blue_export": entity.get("blue_export"),
            "red_export": entity.get("red_export"),
            "exports": exports,
            "match_basis": "exact source FileName/team export prefix joined to parsed original SC export table",
        }
    return result


def write_card_contact_sheet(out: Path, cards: list[dict]) -> dict:
    """Create a labeled verification sheet from recovered original card portrait PNGs."""
    visible = [card for card in cards if (card.get("portrait") or {}).get("file")]
    columns, cell_w, cell_h = 5, 176, 218
    rows = max(1, (len(visible) + columns - 1) // columns)
    sheet = Image.new("RGBA", (columns * cell_w, rows * cell_h), (24, 31, 43, 255))
    draw = ImageDraw.Draw(sheet)
    font = ImageFont.load_default()
    for i, card in enumerate(visible):
        x, y = (i % columns) * cell_w, (i // columns) * cell_h
        source = out / card["portrait"]["file"]
        image = Image.open(source).convert("RGBA")
        image.thumbnail((156, 168), Image.Resampling.LANCZOS)
        sheet.alpha_composite(image, (x + (cell_w - image.width) // 2, y + 5))
        label = f'{card["native_card_row"]} · {card.get("rarity") or "?"} · {card.get("elixir_cost") or "?"} elixir'
        draw.text((x + 6, y + 180), label, fill=(245, 246, 250, 255), font=font)
    target = out / "card-contact-sheet.png"
    target.parent.mkdir(parents=True, exist_ok=True)
    sheet.save(target)
    return {"file": target.relative_to(out).as_posix(), "sha256": sha256(target.read_bytes()),
            "cards_rendered": len(visible), "basis": "recovered original card images at each catalog-declared PNG hash"}


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
    character_rows = observed_character_rows(args.logic, observed_card_names(trace_paths) | FIXED_TRAINING_CARDS | TOWER_ATTACHED_ROWS)
    for binding in character_rows.values():
        if binding["document"] not in args.docs:
            args.docs.append(binding["document"])
    battle_frames = sorted(set((ROOT / "runs").glob("spectator-*.jsonl")) | {
        ROOT / "runs/v442-custom-watch/frames.jsonl",
        ROOT / "runs/v442-watch-smoke/frames.jsonl",
    })
    battle_frames = [path for path in battle_frames if path.is_file()]
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
                              "exports": sc.exports,
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
    observed_card_rows = {row["native_card_row"] for row in deck_metadata["cards_observed_across_traces"]}
    card_visuals = {row["native_card_row"]: row for row in card_visual_bindings(
        ROOT / "data/card_catalog.json", observed_card_rows | FIXED_TRAINING_CARDS, out)}
    card_contact_sheet = write_card_contact_sheet(out, list(card_visuals.values()))
    ui_document = next((doc for doc in docs_meta if doc["name"] == "ui"), None)
    ui_exports = set(ui_document["exports"]) if ui_document else set()
    for card in card_visuals.values():
        for role, export_key in (("card_frame_export", "card_frame_export"),
                                 ("elixir_icon_export", "elixir_icon_export"),
                                 ("portrait_slot_export", "portrait_slot_export")):
            export = card.get(role)
            card[role + "_present_in_source_document"] = bool(export and export in ui_exports)
    entity_animation_rows = list(live_bindings)
    known_animation_rows = {row["native_row"] for row in entity_animation_rows}
    for name, binding in sorted(character_rows.items()):
        if name in known_animation_rows or binding.get("document") not in args.docs:
            continue
        entity_animation_rows.append({
            "native_row": name,
            "document": binding.get("document"),
            "source_file": binding.get("source_file"),
            "source_row": binding.get("source_row"),
            "blue_export": binding.get("blue_export"),
            "red_export": binding.get("red_export"),
            "scale_percent": binding.get("scale_percent"),
            "source_fields": binding.get("source_fields", {}),
            "match_basis": "fixed training card joined to exact original CHARACTER/BUILDING logic row",
        })
    entity_animations = animation_bindings(docs_meta, entity_animation_rows)
    export_index = {}
    for doc in docs_meta:
        for export_name in doc.get("exports", {}):
            export_index.setdefault(export_name, []).append(doc["name"])
    effects_by_native_row = {}
    for row in entity_animation_rows:
        refs = []
        for field, value in sorted((row.get("source_fields") or {}).items()):
            if not isinstance(value, str) or not (field.endswith("Effect") or field.endswith("EffectName")):
                continue
            refs.append({"logic_field": field, "logic_reference": value,
                         "exact_original_exports": export_index.get(value, []),
                         "resolution": "exact export match" if value in export_index else
                                       "effect reference is sourced from original logic; no exact export name in selected SC docs"})
        if refs:
            effects_by_native_row[row["native_row"]] = refs
    tower_visuals = []
    projectile_bindings = []
    tower_tree_path = out / "display-trees" / "building_tower.json"
    tower_doc = json.loads(tower_tree_path.read_text()) if tower_tree_path.is_file() else None
    tower_movies = tower_doc.get("movies", {}) if tower_doc else {}
    for entity in live_bindings:
        row = entity["native_row"]
        fields = entity.get("source_fields", {})
        if row in {"KingTower", "PrincessTower"}:
            component_slots = {}
            if tower_doc:
                for variant, export_name in (("blue", entity.get("blue_export")),
                                             ("red", entity.get("red_export"))):
                    object_id = tower_doc.get("exports", {}).get(export_name)
                    movie = tower_movies.get(str(object_id), {}) if object_id is not None else {}
                    slots = []
                    child_names = movie.get("children_names") or []
                    for index, child_id in enumerate(movie.get("children_ids", [])):
                        name = child_names[index] if index < len(child_names) else ""
                        if not name:
                            continue
                        child_movie = tower_movies.get(str(child_id), {})
                        slots.append({"slot_name": name, "child_id": child_id,
                                      "child_frame_count": len(child_movie.get("frame_element_counts", [])),
                                      "child_total_elements": sum(child_movie.get("frame_element_counts", [])),
                                      "fps": child_movie.get("framerate"),
                                      "source_basis": "named child slot in exact original tower export"})
                    component_slots[variant] = slots
            attached = fields.get("AttachedCharacter")
            attached_binding = entity_animations.get(attached) if attached else None
            tower_visuals.append({
                "native_row": row,
                "document": entity.get("document"),
                "source_file": entity.get("source_file"),
                "source_row": entity.get("source_row"),
                "blue_base_export": entity.get("blue_export"),
                "red_base_export": entity.get("red_export"),
                "blue_top_export": fields.get("BlueTopExportName"),
                "red_top_export": fields.get("RedTopExportName"),
                "scale_percent": entity.get("scale_percent"),
                "attached_character": fields.get("AttachedCharacter"),
                "attached_character_height": fields.get("AttachedCharacterHeight"),
                "attached_character_binding": attached_binding,
                "projectile_row": fields.get("Projectile"),
                "projectile_effect": fields.get("ProjectileEffect"),
                "named_component_slots": component_slots,
                "match_basis": "exact original BUILDING row FileName, base exports, and optional top exports",
            })
        if entity.get("source_row", "").startswith("PROJECTILE."):
            projectile_bindings.append({
                "native_row": row,
                "document": entity.get("document"),
                "source_file": entity.get("source_file"),
                "source_row": entity.get("source_row"),
                "blue_export": entity.get("blue_export"),
                "red_export": entity.get("red_export"),
                "scale_percent": entity.get("scale_percent"),
                "projectile_effect": fields.get("ProjectileEffect"),
                "hit_effect": fields.get("HitEffect"),
                "death_effect": fields.get("DeathEffect"),
                "source_fields": fields,
                "animation_exports": entity_animations.get(row, {}).get("exports", []),
                "match_basis": entity.get("match_basis"),
            })
    king_tower_row = next((row for row in tower_visuals if row["native_row"] == "KingTower"), None)
    king_candidate_doc = next((doc for doc in docs_meta if doc["name"] == "chr_king"), None)
    king_character_candidate = None
    if king_tower_row and king_candidate_doc:
        king_exports = king_candidate_doc.get("exports", {})
        king_character_candidate = {
            "document": "chr_king",
            "source_file": "sc/chr_king.sc",
            "blue_export": "King_blue" if "King_blue" in king_exports else None,
            "red_export": "King_red" if "King_red" in king_exports else None,
            "tower_child_slot": "king_dummy",
            "basis": "native setup locates the original named king_dummy child and replaces it with the side-specific export from chr_king.sc",
            "status": "source-verified gameplay binding",
            "native_source_proof": {
                "function_rva": "0x83f4e4",
                "string_rva": "0x32ae15 (king_dummy)",
                "find_child_call_rva": "0x1242f34",
                "create_export_call_rva": "0x88f3d8",
                "replace_child_call_rva": "0x1242a8c",
                "asset_global_rva": "0xe67364 -> 0x1a5e8e8",
                "behavior": "selects King_blue for side 0 or King_red for side 1, creates that export, replaces king_dummy, then destroys the placeholder",
                "original_tower_slot": {"frame_zero_matrix_index": 65535, "transform": "identity", "replacement_origin": "parent origin"},
                "turret_anchor_source": {"blue_child_index": 2, "blue_offset_xy": [0, -57],
                                         "red_child_index": 4, "red_offset_xy": [0, -33]},
                "basis": "root reverse-engineering verification of original native replacement code and original SC frame-zero slot transforms",
            },
        }
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
                  "animations_by_native_row": entity_animations,
                  "effects_by_native_row": effects_by_native_row,
                  "tower_visuals": tower_visuals,
                  "king_character_candidate": king_character_candidate,
                  "projectile_bindings": projectile_bindings,
                  "card_visuals": card_visuals,
                  "card_contact_sheet": card_contact_sheet,
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
                      "painter_fit": {"x": "world_x / 18000 across fitted field width", "y": "((1 - world_y / 32000) * 768 + 60) / 828 across fitted field height", "blue_side": "lower field"},
                      "original_field_source_viewport": {"x_min": -270, "x_max": 270, "y_min": -60, "y_max": 768},
                      "original_field_source_crop": {"x": 918, "y": 493, "width": 540, "height": 828},
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
