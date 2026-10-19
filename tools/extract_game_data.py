#!/usr/bin/env python3
"""Recover packaged Clash Royale configuration; Python 3.11+, no dependencies.

The shipped files use LZMA-alone with a four-byte uncompressed-size field in
place of the standard eight-byte field. Nothing here executes the Android app.
Decoded resources stay in analysis/decoded; small, provenance-bearing JSON
exports in data/ support the independent offline Rust reconstruction.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import io
import json
import lzma
from pathlib import Path, PurePosixPath
import re
import tomllib
import zipfile


ROOT = Path(__file__).resolve().parents[1]
LOGIC = "assets/csv_logic/"
CARD_TABLES = ("spells_characters", "spells_buildings", "spells_other",
               "spells_evolved", "spells_hero_form")
SELECTION = (
    ("knight", "Knight", "Knight", "troop"),
    ("archers", "Archers", "Archer", "troop"),
    ("goblins", "Goblins", "Goblins", "troop"),
    ("giant", "Giant", "Giant", "troop"),
    ("minions", "Minions", "Minions", "troop"),
    ("skeletons", "Skeletons", "Skeletons", "troop"),
    ("musketeer", "Musketeer", "Musketeer", "troop"),
    ("mini_pekka", "Mini P.E.K.K.A", "MiniPekka", "troop"),
    ("spear_goblins", "Spear Goblins", "SpearGoblins", "troop"),
    ("valkyrie", "Valkyrie", "Valkyrie", "troop"),
    ("cannon", "Cannon", "Cannon", "building"),
    ("fireball", "Fireball", "Fireball", "spell"),
)
ALIASES = {
    "Archer": "archers", "MiniPekka": "mini_pekka", "IceSpirits": "ice_spirit",
    "FireSpirits": "fire_spirit", "IceGolemite": "ice_golem", "Log": "the_log",
    "SkeletonWarriors": "guards", "ZapMachine": "sparky", "RageBarbarian": "lumberjack",
    "BlowdartGoblin": "dart_goblin", "AngryBarbarians": "elite_barbarians",
    "AxeMan": "executioner", "Assassin": "bandit", "DarkWitch": "night_witch",
    "Ghost": "royal_ghost", "MiniSparkys": "zappies", "MovingCannon": "cannon_cart",
    "SkeletonBalloon": "skeleton_barrel", "DartBarrell": "flying_machine",
    "Wallbreakers": "wall_breakers", "EliteArcher": "magic_archer",
    "SuperEliteArcher": "super_magic_archer", "SuperHogRider": "santa_hog_rider",
    "SuperIceGolemite": "super_ice_golem", "SuperArcher": "super_archers",
    "SuperHogRiderTerry": "terry", "WitchMother": "mother_witch", "PrinceBuff": "raging_prince",
    "Xbow": "x_bow", "FirespiritHut": "furnace", "GoblinPartyHut": "party_hut",
    "BarbLog": "barbarian_barrel", "Heal": "heal_spirit", "Snowball": "giant_snowball",
    "GoblinPartyRocket": "party_rocket",
}


def canonical_id(name: str) -> str:
    return ALIASES.get(name, re.sub(r"(?<!^)(?=[A-Z])", "_", name).replace(" ", "_").lower())


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def decode(data: bytes) -> tuple[bytes, str]:
    # Recognize readable resources as well, without swallowing corrupt LZMA.
    try:
        text = data.decode("utf-8-sig")
        if all(ord(c) >= 32 or c in "\r\n\t" for c in text):
            return data, "plain-utf8"
    except UnicodeDecodeError:
        pass
    if len(data) < 9:
        raise ValueError("compressed resource is shorter than its header")
    size = int.from_bytes(data[5:9], "little")
    if size > 128 * 1024 * 1024:
        raise ValueError(f"unreasonable declared decompressed size: {size}")
    decoded = lzma.decompress(data[:9] + b"\0" * 4 + data[9:],
                              format=lzma.FORMAT_ALONE, memlimit=256 * 1024 * 1024)
    if len(decoded) != size:
        raise ValueError(f"size mismatch: declared {size}, decoded {len(decoded)}")
    decoded.decode("utf-8-sig")  # Validate text before writing.
    return decoded, "supercell-lzma-4-byte-size"


def cell(value: str, kind: str):
    if not value:
        return None
    kind = kind.lower().removesuffix("array")
    if kind == "int":
        return int(value)
    if kind == "float":
        return float(value)
    if kind == "boolean":
        if value.lower() not in ("true", "false"):
            raise ValueError(f"invalid boolean: {value!r}")
        return value.lower() == "true"
    return value


def read_csv(data: bytes, member: str) -> dict:
    """Retain blank-name continuation rows rather than misreading them as cards."""
    rows = list(csv.reader(io.StringIO(data.decode("utf-8-sig"))))
    columns, types = rows[:2]
    records = []
    for line, row in enumerate(rows[2:], 3):
        if len(row) > len(columns) and any(row[len(columns):]):
            raise ValueError(f"{member}:{line}: extra populated CSV columns")
        values = {key: cell(value, kind) for key, kind, value in
                  zip(columns, types, row) if value}
        if not values:
            continue
        if row[0]:
            records.append({"name": row[0], "line": line,
                            "fields": values, "continuations": []})
        elif records:
            records[-1]["continuations"].append({"line": line, "fields": values})
        else:
            raise ValueError(f"{member}:{line}: continuation without record")
    return {"source": member, "columns": dict(zip(columns, types)), "records": records}


def write_json(path: Path, value) -> None:
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False, sort_keys=True) + "\n")


def recover(apk: Path, decoded_dir: Path, out_dir: Path) -> dict:
    archive_bytes = apk.read_bytes()
    resources = {}
    manifest = {}
    with zipfile.ZipFile(apk) as archive:
        for info in sorted(archive.infolist(), key=lambda i: i.filename):
            if not info.filename.endswith((".csv", ".toml")):
                continue
            member = PurePosixPath(info.filename)
            if member.is_absolute() or ".." in member.parts:
                raise ValueError(f"unsafe archive member {info.filename!r}")
            raw = archive.read(info)
            text, compression = decode(raw)
            dest = decoded_dir.joinpath(*member.parts)
            dest.parent.mkdir(parents=True, exist_ok=True)
            dest.write_bytes(text)
            resources[info.filename] = text
            manifest[info.filename] = {"compressed_bytes": len(raw),
                                       "decoded_bytes": len(text),
                                       "packaged_sha256": sha256(raw),
                                       "decoded_sha256": sha256(text),
                                       "encoding": compression}

    tables = {name: read_csv(resources[LOGIC + name + ".csv"], LOGIC + name + ".csv")
              for name in CARD_TABLES + ("globals", "battle_timelines", "game_modes", "rarities")}
    # Keep each definition separate and ordered by member. Do not pretend that
    # native configuration overlay/inheritance semantics have been recovered.
    filters_member = LOGIC + "game_object_filters.toml"
    filters = tomllib.loads(resources[filters_member].decode("utf-8-sig"))
    categories = {name: {} for name in ("CHARACTER", "BUILDING", "PROJECTILE", "AEO", "BUFF", "EXT")}
    toml_files = []
    for member, raw in resources.items():
        if not member.startswith(LOGIC) or not member.endswith(".toml"):
            continue
        if "/characters/" not in member and member not in (
            LOGIC + "buildings.toml", LOGIC + "projectiles.toml",
            LOGIC + "area_effect_objects.toml"):
            continue
        parsed = tomllib.loads(raw.decode("utf-8-sig"))
        toml_files.append({"source": member, "data": parsed})
        for category in categories:
            for name, fields in parsed.get(category, {}).items():
                categories[category].setdefault(name, []).append({"source": member, "fields": fields})

    def entry(category: str, name: str, seen: tuple = ()) -> dict:
        if name not in categories[category] and name in categories["EXT"]:
            if name in seen:
                raise ValueError(f"cyclic extension {name}")
            extensions = categories["EXT"][name]
            if len(extensions) != 1:
                raise ValueError(f"ambiguous extension {name}")
            extension = extensions[0]
            base_category, base_name = extension["fields"]["Base"].split(".", 1)
            if base_category != category:
                raise ValueError(f"extension {name} belongs to {base_category}, not {category}")
            base = entry(category, base_name, (*seen, name))
            merged = dict(base["fields"])
            merged.update({k: v for k, v in extension["fields"].items() if k != "Base"})
            merged["Name"] = name
            return {"source": extension["source"], "fields": merged,
                    "inherited_sources": base.get("inherited_sources", []) +
                    [{"member": base["source"], "record": base_name}]}
        matches = categories[category][name]
        if len(matches) != 1:
            raise ValueError(f"ambiguous selected {category}.{name}: {len(matches)} definitions")
        return matches[0]

    def source(member: str, record: str, line: int | None = None) -> dict:
        result = {"member": member, "record": record,
                  "packaged_sha256": manifest[member]["packaged_sha256"],
                  "decoded_sha256": manifest[member]["decoded_sha256"]}
        if line is not None:
            result["line"] = line
        return result

    def playable(card_id: str, name: str, kind: str, definition: dict,
                 card_fields: dict | None = None, sources: list | None = None) -> dict:
        fields = definition["fields"]
        card_fields = card_fields or {}
        sources = list(sources or [])
        sources.append(source(definition["source"], fields["Name"]))
        for inherited in definition.get("inherited_sources", []):
            sources.append(source(inherited["member"], inherited["record"]))
        projectile_name = fields.get("CustomFirstProjectile") or fields.get("Projectile")
        projectile = entry("PROJECTILE", projectile_name) if projectile_name else None
        if projectile:
            sources.append(source(projectile["source"], projectile["fields"]["Name"]))
        attack = projectile["fields"] if projectile else fields
        limitations = []
        for _ in range(4):
            if "Damage" in attack:
                break
            if isinstance(attack.get("DamagePerSecond"), int):
                attack = dict(attack, Damage=attack["DamagePerSecond"])
                limitations.append("Damage-per-second field flattened to one immediate second of damage; timed application omitted.")
                break
            chained = attack.get("SpawnProjectile")
            if chained:
                projectile = entry("PROJECTILE", chained)
                sources.append(source(projectile["source"], projectile["fields"]["Name"]))
                attack = projectile["fields"]
                limitations.append("Chained projectile flattened to one area hit; travel path and pushback omitted.")
            elif attack.get("Buff") and attack["Buff"] in categories["BUFF"]:
                buff = entry("BUFF", attack["Buff"])
                sources.append(source(buff["source"], buff["fields"]["Name"]))
                attack = buff["fields"]
                limitations.append("Damage-over-time/buff represented by one stored damage tick; duration and buff behavior omitted.")
            else:
                raise ValueError("No directly recoverable damage for simplified combat")
        damage = attack["Damage"]
        if isinstance(damage, dict):
            if "BaseDamage" not in damage:
                raise ValueError("Damage expression has no literal BaseDamage")
            damage = damage["BaseDamage"]
        if not isinstance(damage, int):
            raise ValueError("Damage is not a literal integer")
        result = {
            "id": card_id, "name": name, "kind": kind,
            "elixir": card_fields.get("ManaCost", 0),
            "hp": fields.get("Hitpoints", 0), "damage": damage,
            "attack_interval_ms": fields.get("HitSpeed", 0),
            "range": fields.get("Range", 0),
            "speed": fields.get("Speed", 0) * 1000 // 80,
            "count": card_fields.get("SummonNumber", 1),
            "target": ("buildings" if fields.get("TargetOnlyBuildings") else
                       "air_ground" if fields.get("AttacksAir") or fields.get("AoeToAir") else "ground"),
            "flying": fields.get("FlyingHeight", 0) > 0,
            "radius": fields.get("CollisionRadius", 0),
            "splash_radius": (attack.get("Radius") or fields.get("AreaDamageRadius") or
                              (fields.get("Radius", 0) if kind == "spell" else 0)),
            "lifetime_ms": fields.get("LifeTime", 0),
            "deploy_ms": fields.get("DeployTime", 0),
            "sources": sources,
            "raw_speed": fields.get("Speed"),
            "raw_record": fields["Name"],
            "reconstruction_defaults": {},
            "can_deploy_enemy_side": card_fields.get("CanDeployOnEnemySide", False) or kind == "spell",
            "kamikaze": fields.get("Kamikaze", False),
            "limitations": limitations,
        }
        filter_name = fields.get("Filter")
        if (kind == "spell" and filter_name in filters and
                filters[filter_name].get("FilterDescriptionTID") == "TID_TARGETS_AIR_AND_GROUND"):
            result["target"] = "air_ground"
            result["sources"].append(source(filters_member, filter_name))
        unsupported_fields = {
            "DeathSpawnCharacter": "Death-spawn units omitted.", "SpawnCharacter": "Periodic summoned units omitted.",
            "OnDeathAction": "Scripted death actions omitted.", "OnStartingAction": "Scripted starting actions omitted.",
            "OnAttackAction": "Scripted attack actions omitted.", "JumpEnabled": "Jump/pathfinding behavior simplified.",
            "VariableDamage2": "Damage ramp omitted; uses stored initial damage.",
            "ShieldHitpoints": "Shield layer omitted.", "ChargeRange": "Charge behavior omitted.",
            "MorphCharacter": "Morph behavior omitted.", "Ability": "Active champion ability omitted.",
            "DashDamage": "Dash behavior omitted.", "BuffOnDamage": "On-hit buff omitted.",
            "StartingBuff": "Starting buff omitted.", "HidesWhenNotAttacking": "Hiding behavior omitted.",
            "MultipleProjectiles": "Multiple projectile pattern simplified to a single damage hit.",
            "CustomFirstProjectile": "Projectile patterns simplified to primary damaging projectile.",
            "DeathAreaEffect": "Death area effects omitted.",
            "AreaEffectOnHit": "On-hit area effect omitted.",
            "DeathDamage": "Death explosion omitted.",
        }
        for key, explanation in unsupported_fields.items():
            if key == "OnStartingAction" and fields["Name"] == "KingTower":
                continue  # The local engine explicitly implements its activation wait.
            if fields.get(key):
                limitations.append(explanation)
        if attack.get("TargetBuff") or attack.get("Buff"):
            limitations.append("Projectile/on-hit buff and stun effects omitted.")
        if attack.get("Pushback"):
            limitations.append("Attack knockback omitted.")
        if card_fields.get("ProjectileWaves"):
            limitations.append("Projectile waves represented by one stored projectile damage hit.")
        if card_fields.get("SummonCharacterSecond"):
            limitations.append("Mixed summon group approximated with the primary unit type and total stored count.")
            result["count"] += card_fields.get("SummonCharacterSecondCount", 0)
        if kind == "spell":
            limitations.append("Projectile travel, knockback, stun and timed pulses are simplified to immediate area damage.")
        result["support_status"] = "approximate" if limitations else "supported"
        defaults = result["reconstruction_defaults"]
        defaults["speed"] = "raw Speed * 1000 / 80; reconstruction assumption, native timebase unverified"
        for output, raw_name in (("hp", "Hitpoints"), ("attack_interval_ms", "HitSpeed"),
                                 ("range", "Range"), ("radius", "CollisionRadius"),
                                 ("lifetime_ms", "LifeTime"), ("deploy_ms", "DeployTime")):
            if raw_name not in fields:
                defaults[output] = "0 when field absent; not a recovered engine default"
        if "ManaCost" not in card_fields:
            defaults["elixir"] = "0 for non-playable crown towers"
        if "SummonNumber" not in card_fields:
            defaults["count"] = "1 when field absent; reconstruction default"
        if "FlyingHeight" not in fields:
            defaults["flying"] = "false when FlyingHeight absent"
        if result["target"] == "ground" and "AttacksAir" not in fields and "AoeToAir" not in fields and not fields.get("TargetOnlyBuildings"):
            defaults["target"] = "ground when AttacksAir absent"
        if not result["splash_radius"]:
            defaults["splash_radius"] = "0 when area radius absent"
        if "CrownTowerDamagePercent" in attack:
            result["crown_tower_damage_percent_raw"] = attack["CrownTowerDamagePercent"]
        if isinstance(attack.get("Damage"), dict) and "TowerDamage" in attack["Damage"]:
            result["crown_tower_damage"] = attack["Damage"]["TowerDamage"]
        return result

    selected = []
    catalog = []
    for card_id, display, raw_name, kind in SELECTION:
        table = tables["spells_buildings" if kind == "building" else
                       "spells_other" if kind == "spell" else "spells_characters"]
        row = next(r for r in table["records"] if r["name"] == raw_name)
        fields = row["fields"]
        category = "PROJECTILE" if kind == "spell" else "BUILDING" if kind == "building" else "CHARACTER"
        raw_entity = fields["Projectile"] if kind == "spell" else fields["SummonCharacter"]
        selected.append(playable(card_id, display, kind, entry(category, raw_entity), fields,
                                 [source(table["source"], raw_name, row["line"])]))
        selected[-1]["sc_name"] = raw_name
    by_sc_name = {c["sc_name"]: c for c in selected}
    # Broader card selection can use raw basic combat while exposing every
    # omitted special mechanic. Entries without literal damage stay unsupported.
    for table_name in CARD_TABLES[:3]:
        table = tables[table_name]
        overlay_name = LOGIC + table_name + ".toml"
        overlays = tomllib.loads(resources[overlay_name].decode("utf-8-sig"))
        for row in table["records"]:
            sc_name = row["name"]
            fields = dict(row["fields"])
            fields.update(overlays.get(sc_name, {}))
            cid = canonical_id(sc_name)
            card = by_sc_name.get(sc_name)
            error = None
            if card is None and not fields.get("NotInUse") and not fields.get("NotVisible"):
                try:
                    kind = "building" if table_name == "spells_buildings" else "troop"
                    category = "BUILDING" if kind == "building" else "CHARACTER"
                    entity_name = fields.get("SummonCharacter")
                    if not entity_name and fields.get("SummonCharactersList"):
                        entity_name = fields["SummonCharactersList"][0]
                    if table_name == "spells_other" and not entity_name:
                        category, kind = ("PROJECTILE", "spell") if fields.get("Projectile") else ("AEO", "spell")
                        entity_name = fields.get("Projectile") or fields.get("AreaEffectObject")
                    if sc_name == "GoblinBarrel":
                        barrel = entry("PROJECTILE", fields["Projectile"])
                        entity_name = barrel["fields"]["SpawnCharacter"]
                        fields["SummonNumber"] = barrel["fields"]["SpawnCharacterCount"]
                        category, kind = "CHARACTER", "troop"
                    definition = entry(category, entity_name)
                    card = playable(cid, cid.replace("_", " ").title(), kind, definition, fields,
                                    [source(table["source"], sc_name, row["line"])])
                    if sc_name == "GoblinBarrel":
                        card["limitations"].append("Barrel projectile travel and deployment delay omitted; directly summons recovered goblins.")
                        card["support_status"] = "approximate"
                    if "SummonCharactersList" in fields:
                        card["limitations"].append("Custom summon list approximated using first listed unit type.")
                        card["support_status"] = "approximate"
                    if kind != "spell" and card["hp"] <= 0:
                        raise ValueError("No recoverable positive Hitpoints")
                    if "ManaCost" not in fields or not 1 <= fields["ManaCost"] <= 10:
                        raise ValueError("Missing or unsupported elixir cost")
                    card["sc_name"] = sc_name
                    selected.append(card)
                    by_sc_name[sc_name] = card
                except (KeyError, ValueError) as exc:
                    card = None
                    error = str(exc)
            elif card is None:
                error = "Stored NotInUse or NotVisible flag is set"
            catalog.append({"id": cid, "name": card["name"] if card else cid.replace("_", " ").title(),
                            "sc_name": sc_name, "elixir": fields.get("ManaCost"),
                            "rarity": fields.get("Rarity"), "source": source(table["source"], sc_name, row["line"]),
                            "support_status": card["support_status"] if card else "unsupported",
                            "limitations": card["limitations"] if card else [error or "No supported simplified combat representation"],
                            "playable": card is not None, "art_source": fields.get("HighresImageFilename")})
    towers = [playable(card_id, display, "building", entry("BUILDING", raw_name))
              for card_id, display, raw_name in (("princess_tower", "Princess Tower", "PrincessTower"),
                                                  ("king_tower", "King Tower", "KingTower"))]
    out_dir.mkdir(parents=True, exist_ok=True)
    # Card pictures are genuine PNGs already present in the supplied APK.
    # Copy bytes unchanged; never interpret a .sc file as a conventional image.
    art_dir = ROOT / "web/cards"
    art_dir.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(apk) as archive:
        for card in catalog:
            member = "assets/" + (card["art_source"] or "")
            if member not in archive.namelist():
                continue
            raw = archive.read(member)
            if not raw.startswith(b"\x89PNG\r\n\x1a\n"):
                continue
            art_path = art_dir / (card["id"] + ".png")
            art_path.write_bytes(raw)
            card["art"] = {"path": "cards/" + art_path.name, "source": member, "sha256": sha256(raw)}
            if card["sc_name"] in by_sc_name:
                by_sc_name[card["sc_name"]]["art"] = card["art"]
    write_json(out_dir / "cards.json", selected)
    write_json(out_dir / "towers.json", towers)
    write_json(out_dir / "card_catalog.json", catalog)
    write_json(out_dir / "recovered_catalog.json", {
        "schema_version": 1, "card_tables": {name: tables[name] for name in CARD_TABLES},
        "definitions": categories, "toml_documents": toml_files,
        "notes": ["Definitions are raw stored baselines, not displayed card-level statistics.",
                  "Blank CSV cells are absent; blank-name rows are continuations of the preceding record.",
                  "Duplicate definitions and TOML documents are retained, not arbitrarily merged.",
                  "Catalog includes unused, hidden, event, evolution and hero variants; presence does not imply availability."]})
    write_json(out_dir / "rules.json", {
        "schema_version": 1,
        "globals": tables["globals"], "battle_timelines": tables["battle_timelines"],
        "game_modes": tables["game_modes"], "rarities": tables["rarities"],
        "reconstruction_assumptions": [
            "Coordinate conversion 1000 raw distance units per tile is unverified in native code.",
            "Speed conversion raw*1000/80 milli-tiles per second is a reconstruction assumption.",
            "HitSpeed, DeployTime, LifeTime are interpreted as milliseconds from resource field context.",
            "CrownTowerDamagePercent=-77 is interpreted as a 77% reduction (23% retained); native formula unverified.",
            "No rarity/display-level scaling is applied; these are the raw configuration baselines."],
        "recommended_deck": ["knight", "archers", "giant", "minions", "skeletons", "musketeer", "mini_pekka", "fireball"]})
    try:
        archive_path = str(apk.resolve().relative_to(ROOT))
    except ValueError:
        archive_path = str(apk.resolve())
    provenance = {"schema_version": 1,
                  "archive": {"path": archive_path, "sha256": sha256(archive_bytes), "bytes": len(archive_bytes)},
                  "decoder": "LZMA-alone: keep 5 property bytes and 4-byte size; insert four zero bytes before stream",
                  "resource_count": len(resources), "members": manifest,
                  "exports": {"playable_cards": len(selected), "crown_towers": len(towers),
                              "card_records_by_table": {n: len(tables[n]["records"]) for n in CARD_TABLES},
                              "definition_counts": {c: len(d) for c, d in categories.items()}},
                  "coverage": "Every .csv/.toml archive member decoded. All character TOML documents plus shared buildings, projectiles and area-effect documents catalogued. Other tables remain readable under analysis/decoded."}
    write_json(out_dir / "provenance.json", provenance)
    return {"decoded_resources": len(resources), **provenance["exports"]}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("apk", nargs="?", type=Path, default=ROOT / "analysis/apks/install_time_asset_pack.apk")
    parser.add_argument("--decoded-dir", type=Path, default=ROOT / "analysis/decoded")
    parser.add_argument("--out-dir", type=Path, default=ROOT / "data")
    args = parser.parse_args()
    print(json.dumps(recover(args.apk, args.decoded_dir, args.out_dir), indent=2))


if __name__ == "__main__":
    main()
