#!/usr/bin/env python3
"""Join native spectator rows to original logic, effect and particle data.

The spectator bundle written by ``tools/spectator_assets.py`` binds native rows
to SC documents. This companion writes ``presentation.json`` beside it with the
original fields the viewer needs for presentation timing and visual effects:

* logic rows (attack/load times, projectile exports, shadows, effect names),
  resolved transitively from the native rows through Projectile/Spawn fields;
* the original ``legacy_vfx/effects.json`` entries those rows reference,
  including nested effects and enemy variants;
* the ``legacy_vfx/emitters.json`` particle emitters used by those effects.

Every SC export referenced by an effect or emitter is checked against the
bundled display trees. Missing documents/exports are listed, never invented.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import tomllib
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_APK = ROOT / "analysis/apks/install_time_asset_pack.apk"
DEFAULT_LOGIC = ROOT / "analysis/decoded/assets/csv_logic"
DEFAULT_BUNDLE = ROOT / "analysis/native-spectator"

# Native rows observed in recorded original-engine matches, plus the attached
# tower character. Rows reachable from these through logic links are added.
SEED_ROWS = [
    "Archer", "ArcherArrow", "ArrowsSpell", "BarbLogProjectile", "BarbLogProjectileRolling",
    "Barbarian", "Berserker", "Cannon", "DarkPrince", "ElectroGiant", "FireballSpell",
    "FreezeIceGolemite", "GoblinCurse", "GoblinCurseBase", "GoblinPartyHut", "HogRider",
    "IceGolemite", "IceGolemiteDeathArea", "IceSpirits", "IceSpiritsProjectile",
    "KingProjectile", "KingTower", "Knight", "LogProjectile", "LogProjectileRolling",
    "MiniPekka", "Minion", "MinionSpit", "Mortar", "MortarProjectile", "Musketeer",
    "MusketeerProjectile", "PrincessTower", "Ram", "RamRider", "RamRiderBola", "RocketSpell",
    "RoyalGiant", "RoyalGiantProjectile", "Skeleton", "SkeletonKing", "SkeletonKingGraveyard",
    "SkeletonKingSkeleton", "TowerCannonball", "TowerPrincessProjectile", "Wallbreaker",
    "WallbreakerProjectile", "TowerPrincess",
    "Princess", "PrincessProjectile", "PrincessProjectileDeco", "InfernoTower",
    "GoblinBarrel", "GoblinBarrelSpell", "BarrelGoblin", "GoblinGang", "Goblin_Stab",
    "SpearGoblin", "SpearGoblinProjectile",
]
ROW_LINK_FIELDS = (
    "Projectile", "SpawnProjectile", "SpawnCharacter", "DeathAreaEffect", "AreaEffect",
    "SpawnAreaObject", "DeathSpawnCharacter", "ProjectileSpecial",
)
# Effects the client plays for default towers. They are named by the tower
# rows' DeathEffect fields or by the default tower skin; listed explicitly so
# the presentation file is complete without a skin table.
EXTRA_EFFECTS = [
    "Damage", "Death_player", "King_player_die", "King_enemy_die", "TowerExplode",
    "Default_King_Tower_Die", "Default_Princess_Tower_Die", "crown_tower_death1",
    "crown_tower_death1_red", "crown_tower_death2", "crown_tower_death2_red",
]
KEEP_TYPES = {"CHARACTER", "BUILDING", "PROJECTILE", "AEO", "SPELL", None}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def scalar_fields(fields: dict) -> dict:
    return {k: v for k, v in fields.items() if isinstance(v, (str, int, float, bool))}


def index_logic(logic: Path) -> tuple[dict, dict]:
    index: dict[str, list[tuple[str, str | None, dict]]] = {}
    hashes = {}
    files = sorted(logic.glob("*.toml")) + sorted((logic / "characters").rglob("*.toml"))
    for path in files:
        raw = path.read_bytes()
        try:
            data = tomllib.loads(raw.decode("utf-8"))
        except (tomllib.TOMLDecodeError, UnicodeDecodeError):
            continue
        rel = path.relative_to(logic).as_posix()
        hashes[rel] = sha256(raw)
        for key, value in data.items():
            if not isinstance(value, dict):
                continue
            nested = value and key.isupper() and all(isinstance(v, dict) for v in value.values())
            if nested:
                for name, fields in value.items():
                    index.setdefault(name, []).append((rel, key, fields))
            else:
                index.setdefault(key, []).append((rel, None, value))
    return index, hashes


def pick_row(index: dict, name: str, seen=frozenset()):
    """Prefer per-card character files, then typed base tables."""
    candidates = [c for c in index.get(name, []) if c[1] in KEEP_TYPES and scalar_fields(c[2])]
    if not candidates:
        if name in seen:
            return None
        for rel, kind, fields in index.get(name, []):
            if kind != "EXT" or not isinstance(fields.get("Base"), str):
                continue
            base_kind, _, base_name = fields["Base"].partition(".")
            base_index = dict(index)
            base_index[base_name] = [row for row in index.get(base_name, []) if row[1] in (base_kind, "EXT")]
            parent = pick_row(base_index, base_name, seen | {name})
            if parent is not None:
                return rel, parent[1], {**parent[2], **fields}
        return None
    candidates.sort(key=lambda c: (
        not c[0].startswith("characters/"),
        "/hero_form/" in c[0] or "_ev" in c[0] or "evo" in c[0],
        c[1] is None,
        -len(c[2]),
    ))
    return candidates[0]


def bundle_exports(bundle: Path) -> dict[str, set[str]]:
    exports = {}
    for tree in sorted((bundle / "display-trees").glob("*.json")):
        exports[tree.stem] = set(json.loads(tree.read_text())["exports"])
    return exports


def doc_of(file_name: str) -> str:
    return Path(file_name).stem


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--apk", type=Path, default=DEFAULT_APK)
    ap.add_argument("--logic", type=Path, default=DEFAULT_LOGIC)
    ap.add_argument("--bundle", type=Path, default=DEFAULT_BUNDLE)
    ap.add_argument("--out", type=Path, help="defaults to BUNDLE/presentation.json")
    args = ap.parse_args()
    out = args.out or args.bundle / "presentation.json"

    with zipfile.ZipFile(args.apk) as apk:
        effects_raw = apk.read("assets/legacy_vfx/effects.json")
        emitters_raw = apk.read("assets/legacy_vfx/emitters.json")
    all_effects = json.loads(effects_raw)
    all_emitters = json.loads(emitters_raw)
    index, logic_hashes = index_logic(args.logic)
    exports = bundle_exports(args.bundle)

    rows: dict[str, dict] = {}
    pending = list(SEED_ROWS)
    unresolved_rows = []
    while pending:
        name = pending.pop()
        if name in rows or name in unresolved_rows:
            continue
        picked = pick_row(index, name)
        if picked is None:
            unresolved_rows.append(name)
            continue
        rel, kind, fields = picked
        scalars = scalar_fields(fields)
        rows[name] = {"logic_file": rel, "type": kind, "fields": scalars}
        for link in ROW_LINK_FIELDS:
            target = scalars.get(link)
            if isinstance(target, str) and target not in rows:
                pending.append(target)

    wanted_effects = list(EXTRA_EFFECTS)
    for row in rows.values():
        for key, value in row["fields"].items():
            if isinstance(value, str) and (re.search(r"Effect\d*$", key) or key.endswith("EffectName")):
                wanted_effects.append(value)
    effects: dict[str, dict] = {}
    missing_effects = []
    while wanted_effects:
        name = wanted_effects.pop()
        if name in effects or name in missing_effects:
            continue
        effect = all_effects.get(name)
        if effect is None:
            missing_effects.append(name)
            continue
        effects[name] = effect
        for linked in ("EnemyVersion",):
            if isinstance(effect.get(linked), str):
                wanted_effects.append(effect[linked])
        for component in effect.get("RenderableComponents", []):
            if component.get("Type") == "Effect" and isinstance(component.get("Effect"), str):
                wanted_effects.append(component["Effect"])

    emitters = {}
    missing_emitters = []
    missing_exports = set()
    for effect in effects.values():
        for component in effect.get("RenderableComponents", []):
            if component.get("Type") == "SWF" and component.get("ExportName"):
                doc = doc_of(component.get("FileName", ""))
                if component["ExportName"] not in exports.get(doc, set()):
                    missing_exports.add(f"{doc}:{component['ExportName']}")
            name = component.get("ParticleEmitterName")
            if not name or name in emitters:
                continue
            emitter = all_emitters.get(name)
            if emitter is None:
                missing_emitters.append(name)
                continue
            emitters[name] = emitter
            for export, resource in zip(emitter.get("ParticleExportName", []),
                                        emitter.get("ParticleResource", [])):
                doc = doc_of(resource)
                if export not in exports.get(doc, set()):
                    missing_exports.add(f"{doc}:{export}")
    for row in rows.values():
        fields = row["fields"]
        file_name = fields.get("FileName")
        if not isinstance(file_name, str):
            continue
        doc = doc_of(file_name)
        for key in ("ExportName", "RedExportName", "ShadowExportName"):
            export = fields.get(key)
            if isinstance(export, str) and export not in exports.get(doc, set()):
                missing_exports.add(f"{doc}:{export}")

    result = {
        "format": "clash-native-spectator-presentation-v1",
        "basis": "Original logic rows and legacy_vfx effect/emitter definitions; exports checked against bundled display trees.",
        "sources": {
            "apk": args.apk.name,
            "apk_sha256": sha256(args.apk.read_bytes()),
            "effects_json_sha256": sha256(effects_raw),
            "emitters_json_sha256": sha256(emitters_raw),
            "logic_files": {row["logic_file"]: logic_hashes.get(row["logic_file"]) for row in rows.values()},
        },
        "rows": dict(sorted(rows.items())),
        "effects": dict(sorted(effects.items())),
        "emitters": dict(sorted(emitters.items())),
        "unresolved": {
            "rows": sorted(unresolved_rows),
            "effects": sorted(missing_effects),
            "emitters": sorted(missing_emitters),
            "exports_not_in_bundle": sorted(missing_exports),
        },
    }
    out.write_text(json.dumps(result, indent=1, sort_keys=False) + "\n")
    print(json.dumps({
        "written": str(out), "rows": len(rows), "effects": len(effects), "emitters": len(emitters),
        "unresolved_rows": sorted(unresolved_rows), "missing_effects": len(missing_effects),
        "missing_emitters": len(missing_emitters), "exports_not_in_bundle": len(missing_exports),
    }, indent=1))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
