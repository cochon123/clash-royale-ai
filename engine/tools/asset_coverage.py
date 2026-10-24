"""Source-resolved card, effect and document coverage for the spectator bundle.

The walker follows original spell roots and logic link fields. It does not
invent rows by trimming plurals or by fuzzy names. Coverage is source
availability of documents, exports, effects and emitters, not animation parity.
"""
from __future__ import annotations

import csv
import hashlib
import json
import re
import tomllib
import zipfile
from pathlib import Path

try:
    from spectator_presentation import (
        EXTRA_EFFECTS, ROW_LINK_FIELDS, SEED_ROWS, index_logic, pick_row, scalar_fields,
    )
except ImportError:
    from tools.spectator_presentation import (
        EXTRA_EFFECTS, ROW_LINK_FIELDS, SEED_ROWS, index_logic, pick_row, scalar_fields,
    )

SPELL_CSV = ("spells_characters.csv", "spells_buildings.csv", "spells_other.csv")
SPELL_TOML = ("spells_characters.toml", "spells_buildings.toml", "spells_other.toml")
ROOT_FIELDS = (
    "SummonCharacter", "SummonCharacterSecond", "Projectile",
    "CustomFirstProjectile", "AreaEffectObject",
)
ALWAYS_ROWS = ("KingTower", "PrincessTower", "TowerPrincess")
# chr_king is the source-verified king_dummy replacement, not a logic FileName.
ALWAYS_STEMS = (
    "arena_training", "level_bootcamp_arena_dl", "ui", "ui_spectate_hud",
    "effects", "building_tower", "chr_king", "chr_princess",
)
CAPTURE_FOLDERS = ("captures", "all-card-captures", "target-captures", "priority-captures")
EXPORT_FIELDS = (
    ("blue", "BlueExportName"),
    ("red", "RedExportName"),
    ("export", "ExportName"),
    ("blue_top", "BlueTopExportName"),
    ("red_top", "RedTopExportName"),
    ("shadow", "ShadowExportName"),
)
BINDING_FIELDS = {
    "BlueTopExportName", "RedTopExportName", "Scale", "ScaleX", "ScaleY",
    "ShadowX", "ShadowY", "ShadowScaleX", "ShadowScaleY", "ShadowSkew",
    "Projectile", "ProjectileEffect", "HitEffect", "DeathEffect",
    "DamageEffect", "AttackStartEffect", "SpawnEffect", "MoveEffect",
    "DeployBaseAnimExportName", "AttachedCharacter", "AttachedCharacterHeight",
    "use360Frames", "Gravity", "Speed", "Homing", "OnlyEnemies",
    "ProjectileRange", "AoeToAir", "AoeToGround",
}
EFFECT_KEY = re.compile(r"Effect\d*$")
STATUS_BASIS = (
    "source availability of original documents, exports, effects, and emitters; "
    "not animation parity"
)
CARDS_SELECTOR = {
    "argument": "--cards",
    "format": "comma-separated native card names",
    "argv": "one process argument; spaces inside a name are preserved",
    "example": "Pekka,Elixir Collector,BabyDragon",
    "absent_metadata_fallback": (
        "If card_dependency_documents is absent, loaders keep the previous whole-bundle "
        "behavior. That fallback is only for an older bundle. When the key is present it "
        "is authoritative. Unknown extra keys do not trigger the fallback."
    ),
}
TYPED_BASES = {"CHARACTER", "BUILDING", "PROJECTILE", "AEO", "SPELL"}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def parse_cards_argument(value: str | None) -> list[str]:
    """Split one process argument on commas. Internal spaces stay intact."""
    if value is None:
        return []
    return [part.strip() for part in str(value).split(",") if part.strip()]


def output_is_readonly_tree(output: Path, engine: Path) -> bool:
    resolved = output.resolve()
    for name in ("analysis", "runs"):
        root = (engine / name).resolve()
        if resolved == root or root in resolved.parents:
            return True
    return False


def selectable_native_cards(engine: Path) -> list[str]:
    """Policy aliases whose original catalog art file is present. Barbarian Hut and Giant Snowball have no art."""
    aliases = json.loads((engine / "data" / "policy_card_aliases.json").read_text())["cards"]
    catalog = {
        row.get("sc_name"): row
        for row in json.loads((engine / "data" / "card_catalog.json").read_text())
    }
    names = []
    for alias in aliases:
        native = alias.get("internal_id")
        art = ((catalog.get(native) or {}).get("art") or {}).get("path")
        if native and art and (engine / "web" / art).is_file():
            names.append(native)
    return names


def _csv_records(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8-sig") as handle:
        rows = list(csv.reader(handle))
    if len(rows) < 3:
        return []
    names = [name.strip() for name in rows[0]]
    records = []
    for values in rows[2:]:
        if not any(value.strip() for value in values):
            continue
        values += [""] * (len(names) - len(values))
        records.append({key: value.strip() for key, value in zip(names, values) if key})
    return records


def _root(field: str, target: str, source_file: str, digest: str) -> dict:
    return {"field": field, "target": target, "source_file": source_file, "source_sha256": digest}


def load_spell_cards(logic: Path) -> dict[str, dict]:
    """CSV summon/projectile/area roots, plus toml SummonCharactersList where the CSV cell is empty."""
    cards: dict[str, dict] = {}
    for filename in SPELL_CSV:
        path = logic / filename
        if not path.is_file():
            continue
        digest = sha256(path.read_bytes())
        for record in _csv_records(path):
            name = record.get("Name")
            if not name:
                continue
            entry = cards.setdefault(name, {
                "name": name, "spell_file": filename, "spell_sha256": digest,
                "roots": [], "cast_effect": None,
            })
            known = {(root["field"], root["target"]) for root in entry["roots"]}
            for field in ROOT_FIELDS:
                target = record.get(field, "")
                if target and (field, target) not in known:
                    entry["roots"].append(_root(field, target, filename, digest))
                    known.add((field, target))
            effect = record.get("Effect", "")
            if effect and effect.lower() not in {"true", "false"} and any(char.isalpha() for char in effect):
                entry["cast_effect"] = {"name": effect, "source_file": filename, "source_sha256": digest}
    for filename in SPELL_TOML:
        path = logic / filename
        if not path.is_file():
            continue
        digest = sha256(path.read_bytes())
        try:
            data = tomllib.loads(path.read_text(encoding="utf-8"))
        except (OSError, tomllib.TOMLDecodeError, UnicodeDecodeError):
            continue
        for name, fields in data.items():
            if not isinstance(fields, dict):
                continue
            entry = cards.setdefault(name, {
                "name": name, "spell_file": filename, "spell_sha256": digest,
                "roots": [], "cast_effect": None,
            })
            known = {(root["field"], root["target"]) for root in entry["roots"]}
            for field in ROOT_FIELDS:
                target = fields.get(field)
                if isinstance(target, str) and target and (field, target) not in known:
                    entry["roots"].append(_root(field, target, filename, digest))
                    known.add((field, target))
            listed = fields.get("SummonCharactersList")
            if isinstance(listed, list):
                for target in listed:
                    if isinstance(target, str) and target and ("SummonCharactersList", target) not in known:
                        entry["roots"].append(_root("SummonCharactersList", target, filename, digest))
                        known.add(("SummonCharactersList", target))
    return cards


def document_stem(file_name) -> str | None:
    if not isinstance(file_name, str):
        return None
    clean = file_name.split("#", 1)[0].strip()
    if not clean.endswith(".sc"):
        return None
    return Path(clean).stem or None


def apk_document_stems(apk: Path) -> set[str]:
    with zipfile.ZipFile(apk) as archive:
        return {
            Path(name).stem for name in archive.namelist()
            if name.startswith("assets/sc/") and name.endswith(".sc")
        }


def load_vfx(apk: Path):
    with zipfile.ZipFile(apk) as archive:
        effects_raw = archive.read("assets/legacy_vfx/effects.json")
        emitters_raw = archive.read("assets/legacy_vfx/emitters.json")
    return json.loads(effects_raw), json.loads(emitters_raw), effects_raw, emitters_raw


def _cached_pick(index, cache, name):
    if name not in cache:
        cache[name] = pick_row(index, name)
    return cache[name]


def _is_effect_key(key: str) -> bool:
    return bool(EFFECT_KEY.search(key) or key.endswith("EffectName"))


def walk_rows(index, seeds, cache, all_effects) -> tuple[dict, list[str], list[str]]:
    """Return resolved rows, unresolved row names, and names that are effects rather than rows."""
    rows = {}
    unresolved = []
    promoted = []
    seen_unresolved = set()
    seen_promoted = set()
    pending = [seed for seed in seeds if isinstance(seed, str) and seed]
    while pending:
        name = pending.pop()
        if name in rows or name in seen_unresolved or name in seen_promoted:
            continue
        picked = _cached_pick(index, cache, name)
        if picked is None:
            if name in all_effects:
                seen_promoted.add(name)
                promoted.append(name)
                continue
            seen_unresolved.add(name)
            unresolved.append(name)
            continue
        rel, kind, fields = picked
        scalars = scalar_fields(fields)
        rows[name] = {"logic_file": rel, "type": kind, "fields": scalars}
        for link in ROW_LINK_FIELDS:
            target = scalars.get(link)
            if isinstance(target, str) and target:
                pending.append(target)
    return rows, unresolved, promoted


def effect_names(fields: dict, index, cache, all_effects) -> list[str]:
    names = []
    for key, value in fields.items():
        if not isinstance(value, str) or not value or value.lower() in {"true", "false"}:
            continue
        if not _is_effect_key(key):
            continue
        if key in ROW_LINK_FIELDS and _cached_pick(index, cache, value) is not None and value not in all_effects:
            continue
        names.append(value)
    return names


def walk_vfx(index, cache, all_effects, all_emitters, rows, extra_effects=()):
    wanted = [name for name in extra_effects if isinstance(name, str) and name]
    for row in rows.values():
        wanted.extend(effect_names(row["fields"], index, cache, all_effects))
    effects = {}
    missing_effects = []
    seen_missing = set()
    while wanted:
        name = wanted.pop()
        if name in effects or name in seen_missing:
            continue
        effect = all_effects.get(name)
        if not isinstance(effect, dict):
            seen_missing.add(name)
            missing_effects.append(name)
            continue
        effects[name] = effect
        enemy = effect.get("EnemyVersion")
        if isinstance(enemy, str) and enemy:
            wanted.append(enemy)
        for component in effect.get("RenderableComponents") or []:
            if isinstance(component, dict) and component.get("Type") == "Effect":
                nested = component.get("Effect")
                if isinstance(nested, str) and nested:
                    wanted.append(nested)
    emitters = {}
    missing_emitters = []
    seen_emitters = set()
    documents = set()
    export_refs = []
    for effect_name, effect in effects.items():
        for component in effect.get("RenderableComponents") or []:
            if not isinstance(component, dict):
                continue
            export = component.get("ExportName")
            if component.get("Type") == "SWF" and isinstance(export, str) and export:
                stem = document_stem(component.get("FileName", ""))
                if stem:
                    documents.add(stem)
                export_refs.append({
                    "document": stem or "", "export": export, "role": "effect", "owner": effect_name,
                })
            emitter_name = component.get("ParticleEmitterName")
            if not isinstance(emitter_name, str) or not emitter_name or emitter_name in emitters or emitter_name in seen_emitters:
                continue
            emitter = all_emitters.get(emitter_name)
            if not isinstance(emitter, dict):
                seen_emitters.add(emitter_name)
                missing_emitters.append(emitter_name)
                continue
            emitters[emitter_name] = emitter
    for emitter_name, emitter in emitters.items():
        for export, resource in zip(emitter.get("ParticleExportName") or [], emitter.get("ParticleResource") or []):
            stem = document_stem(resource)
            if stem:
                documents.add(stem)
            if isinstance(export, str) and export:
                export_refs.append({
                    "document": stem or "", "export": export, "role": "emitter", "owner": emitter_name,
                })
    for row_name, row in rows.items():
        fields = row["fields"]
        stem = document_stem(fields.get("FileName"))
        if stem:
            documents.add(stem)
        for role, key in EXPORT_FIELDS:
            export = fields.get(key)
            if isinstance(export, str) and export:
                export_refs.append({
                    "document": stem or "", "export": export, "role": role, "owner": row_name,
                })
    return effects, sorted(set(missing_effects)), emitters, sorted(set(missing_emitters)), documents, export_refs


def _close(index, cache, all_effects, all_emitters, seeds, extra_effects=()):
    rows, unresolved, promoted = walk_rows(index, seeds, cache, all_effects)
    effects, missing_effects, emitters, missing_emitters, documents, export_refs = walk_vfx(
        index, cache, all_effects, all_emitters, rows, list(extra_effects) + promoted)
    return {
        "rows": rows,
        "unresolved_rows": sorted(set(unresolved)),
        "effects": effects,
        "missing_effects": missing_effects,
        "emitters": emitters,
        "missing_emitters": missing_emitters,
        "documents": documents,
        "export_refs": export_refs,
    }


def _source_rows(rows: dict) -> list[dict]:
    records = []
    for name in sorted(rows):
        row = rows[name]
        fields = row["fields"]
        red = fields.get("RedExportName")
        records.append({
            "native_row": name,
            "logic_file": row["logic_file"],
            "source_row": f"{row['type']}.{name}" if row["type"] else name,
            "source_file": fields.get("FileName") if isinstance(fields.get("FileName"), str) else None,
            "document": document_stem(fields.get("FileName")),
            "blue_export": fields.get("BlueExportName") or fields.get("ExportName"),
            "red_export": red if isinstance(red, str) and red else None,
            "red_team": "RedExportName" if isinstance(red, str) and red else
                        "source has no RedExportName; the red team uses the blue export",
        })
    return records


def _card_record(name, spell, closed) -> dict:
    return {
        "native_card": name,
        "spell_found": spell is not None,
        "spell_file": None if spell is None else spell["spell_file"],
        "spell_sha256": None if spell is None else spell["spell_sha256"],
        "roots": [] if spell is None else spell["roots"],
        "cast_effect": None if spell is None else spell.get("cast_effect"),
        "note": None if spell is None or spell["roots"] else
                "original spell row has no SummonCharacter, Projectile, CustomFirstProjectile, AreaEffectObject, or SummonCharactersList",
        "source_rows": _source_rows(closed["rows"]),
        "documents": sorted(closed["documents"]),
        "unresolved_rows": closed["unresolved_rows"],
        "effects": sorted(closed["effects"]),
        "missing_effects": closed["missing_effects"],
        "emitters": sorted(closed["emitters"]),
        "missing_emitters": closed["missing_emitters"],
        "export_refs": closed["export_refs"],
    }


def _unresolved_card(name: str) -> dict:
    empty = {
        "rows": {}, "unresolved_rows": [], "effects": {}, "missing_effects": [],
        "emitters": {}, "missing_emitters": [], "documents": set(), "export_refs": [],
    }
    record = _card_record(name, None, empty)
    record["unresolved_rows"] = [name]
    return record


def coverage_status(spell_found, missing_documents, missing_exports, unresolved_rows, missing_effects, missing_emitters) -> str:
    if not spell_found:
        return "unresolved"
    if missing_documents or missing_exports or unresolved_rows or missing_effects or missing_emitters:
        return "partial"
    return "complete"


def _family_count(exports: set[str], prefix: str) -> int:
    return sum(1 for name in exports if name == prefix or name.startswith(prefix + "_"))


def apply_bundle_checks(record: dict, bundle_exports: dict[str, set[str]], doc_hashes: dict, apk_entries: dict) -> dict:
    """Fill document and export presence. Does not invent missing artwork."""
    checked = dict(record)
    document_checks = []
    missing_documents = []
    for stem in record.get("documents") or []:
        present = stem in bundle_exports
        document_checks.append({
            "stem": stem,
            "in_bundle": present,
            "source_sha256": doc_hashes.get(stem),
            "apk_entry": apk_entries.get(stem),
        })
        if not present:
            missing_documents.append(stem)
    export_checks = []
    missing_exports = []
    vfx_total = vfx_present = 0
    for ref in record.get("export_refs") or []:
        document = ref.get("document") or ""
        export = ref.get("export") or ""
        names = bundle_exports.get(document, set())
        exact = bool(document) and export in names
        family = _family_count(names, export) if document else 0
        # Original SC documents store the logic export as a prefix of the
        # directional clips (pekka_run1_1), which is the same family join the
        # animation bindings use.
        present = exact or family > 0
        if ref.get("role") in {"effect", "emitter"}:
            vfx_total += 1
            vfx_present += int(present)
            if not present:
                missing_exports.append(f"{document}:{export}" if document else export)
            continue
        export_checks.append({
            "document": document, "export": export, "role": ref.get("role"),
            "owner": ref.get("owner"), "exact_export_present": exact,
            "present": present, "family_count": family,
        })
        if not present:
            missing_exports.append(f"{document}:{export}" if document else export)
    checked["document_checks"] = document_checks
    checked["missing_documents"] = sorted(missing_documents)
    checked["export_checks"] = sorted(export_checks, key=lambda item: (item["document"], item["export"], item["role"]))
    checked["missing_exports"] = sorted(set(missing_exports))
    checked["vfx_export_count"] = vfx_total
    checked["vfx_exports_present"] = vfx_present
    checked.pop("export_refs", None)
    checked["status"] = coverage_status(
        checked["spell_found"], checked["missing_documents"], checked["missing_exports"],
        checked["unresolved_rows"], checked["missing_effects"], checked["missing_emitters"],
    )
    checked["status_basis"] = STATUS_BASIS
    return checked


def _binding(name: str, row: dict) -> dict:
    fields = row["fields"]
    source_file = fields.get("FileName") if isinstance(fields.get("FileName"), str) else None
    blue = fields.get("BlueExportName") or fields.get("ExportName")
    red = fields.get("RedExportName") or blue
    base = fields.get("Base")
    match_basis = "exact original nested logic row FileName and export fields"
    if isinstance(base, str) and base.split(".", 1)[0] in TYPED_BASES:
        match_basis = f"original typed Base inheritance: {base} ({row['logic_file']})"
    if not (source_file and blue):
        match_basis = "original logic row; this row has no SC document and export"
    return {
        "native_row": name,
        "logic_file": row["logic_file"],
        "source_row": f"{row['type']}.{name}" if row["type"] else name,
        "source_file": source_file,
        "document": document_stem(source_file) if source_file else None,
        "blue_export": blue,
        "red_export": red,
        "scale_percent": fields.get("Scale", 100),
        "source_fields": {key: fields[key] for key in sorted(BINDING_FIELDS) if key in fields},
        "match_basis": match_basis,
    }


def _unresolved_binding(name: str, match_basis: str, roots=None) -> dict:
    binding = {
        "native_row": name,
        "document": None,
        "blue_export": None,
        "red_export": None,
        "match_basis": match_basis,
    }
    if roots is not None:
        binding["roots"] = roots
    return binding


def _alias_binding(name: str, root: dict, picked) -> dict | None:
    if picked is None:
        return None
    rel, kind, fields = picked
    scalars = scalar_fields(fields)
    source_file = scalars.get("FileName") if isinstance(scalars.get("FileName"), str) else None
    blue = scalars.get("BlueExportName") or scalars.get("ExportName")
    if not (source_file and blue):
        return None
    binding = _binding(root["target"], {"logic_file": rel, "type": kind, "fields": scalars})
    binding["native_row"] = name
    binding["alias_of"] = root["target"]
    binding["match_basis"] = (
        f"spell-root alias {root['field']}={root['target']} from {root['source_file']} "
        f"sha256={root['source_sha256']}"
    )
    return binding


def discover_frames(engine: Path, extra: list[Path] | None = None) -> list[Path]:
    found = set()
    runs = engine / "runs"
    if runs.exists():
        found.update(path for path in runs.glob("spectator-*.jsonl"))
        for name in ("v442-custom-watch/frames.jsonl", "v442-watch-smoke/frames.jsonl", "banana-duel-watch/frames.jsonl"):
            found.add(runs / name)
    worktree = engine.parent / ".local-arena"
    for folder in CAPTURE_FOLDERS:
        base = worktree / folder
        if base.is_dir():
            found.update(base.rglob("frames.jsonl"))
    for path in extra or []:
        found.add(Path(path))
    return sorted(path.resolve() for path in found if path.is_file())


def names_from_frames(paths: list[Path]) -> tuple[set[str], set[str]]:
    rows, cards = set(), set()
    for path in paths:
        try:
            stream = path.open(encoding="utf-8")
        except OSError:
            continue
        with stream:
            for line in stream:
                line = line.strip()
                if not line:
                    continue
                try:
                    frame = json.loads(line)
                except json.JSONDecodeError:
                    continue
                for entity in frame.get("entities") or []:
                    if isinstance(entity, dict) and entity.get("row"):
                        rows.add(str(entity["row"]))
                for hand in frame.get("hands") or []:
                    if not isinstance(hand, list):
                        continue
                    for card in hand:
                        if isinstance(card, dict) and card.get("name"):
                            cards.add(str(card["name"]))
    return rows, cards


def _context(logic: Path, apk: Path):
    index, logic_hashes = index_logic(logic)
    effects, emitters, effects_raw, emitters_raw = load_vfx(apk)
    return index, logic_hashes, load_spell_cards(logic), effects, emitters, effects_raw, emitters_raw


def plan_coverage(logic: Path, apk: Path, engine: Path, cards_argument: str = "", frame_paths: list[Path] | None = None) -> dict:
    index, _, spells, all_effects, all_emitters, _, _ = _context(logic, apk)
    requested = parse_cards_argument(cards_argument)
    selected = requested or selectable_native_cards(engine)
    frame_rows, frame_cards = names_from_frames(frame_paths or [])
    cache = {}
    cards = {}
    union_seeds = list(SEED_ROWS) + list(ALWAYS_ROWS)
    for name in selected:
        spell = spells.get(name)
        if spell is None:
            cards[name] = _unresolved_card(name)
            continue
        seeds = [root["target"] for root in spell["roots"]]
        extra = [spell["cast_effect"]["name"]] if spell.get("cast_effect") else []
        cards[name] = _card_record(name, spell, _close(index, cache, all_effects, all_emitters, seeds, extra))
        union_seeds.extend(seeds)
    alias_bindings = []
    unresolved_names = []
    for name in sorted(frame_rows | frame_cards):
        spell = spells.get(name)
        picked = _cached_pick(index, cache, name)
        if picked is not None:
            union_seeds.append(name)
            continue
        if spell is not None:
            union_seeds.extend(root["target"] for root in spell["roots"])
            if name not in frame_rows:
                continue
            visual_roots = []
            for root in spell["roots"]:
                root_pick = _cached_pick(index, cache, root["target"])
                if root_pick is not None:
                    _, _, fields = root_pick
                    scalars = scalar_fields(fields)
                    if isinstance(scalars.get("FileName"), str) and (scalars.get("BlueExportName") or scalars.get("ExportName")):
                        visual_roots.append((root, root_pick))
            if len(visual_roots) == 1:
                binding = _alias_binding(name, visual_roots[0][0], visual_roots[0][1])
                if binding is not None:
                    alias_bindings.append(binding)
                    continue
            unresolved_names.append(_unresolved_binding(
                name,
                "spell alias has no single SC export root; each root is bound under its own row",
                spell["roots"],
            ))
            continue
        if name in frame_rows:
            unresolved_names.append(_unresolved_binding(
                name, "no original nested logic row supplied an SC document and export"))
    always_closed = _close(index, cache, all_effects, all_emitters, list(ALWAYS_ROWS), EXTRA_EFFECTS)
    always_record = _card_record("always", None, always_closed)
    always_record["spell_found"] = True
    always_record["documents"] = sorted(set(ALWAYS_STEMS) | always_closed["documents"])
    always_record["note"] = "arena, HUD, tower, source-verified king_dummy replacement, and shared effect documents required every match"
    union = _close(index, cache, all_effects, all_emitters, union_seeds, EXTRA_EFFECTS)
    documents = set(always_record["documents"]) | union["documents"]
    for card in cards.values():
        documents.update(card["documents"])
    bindings = {name: _binding(name, row) for name, row in union["rows"].items()}
    for binding in alias_bindings + unresolved_names:
        bindings.setdefault(binding["native_row"], binding)
    return {
        "cards": cards,
        "always": always_record,
        "always_documents": always_record["documents"],
        "documents": sorted(documents),
        "bindings": [bindings[name] for name in sorted(bindings)],
        "rows": union["rows"],
        "effects": union["effects"],
        "emitters": union["emitters"],
        "unresolved_rows": union["unresolved_rows"],
        "missing_effects": union["missing_effects"],
        "missing_emitters": union["missing_emitters"],
    }


def finalize_coverage(plan: dict, docs_meta: list[dict]) -> dict:
    bundle_exports = {doc["name"]: set(doc.get("exports") or {}) for doc in docs_meta}
    doc_hashes = {doc["name"]: doc.get("sha256") for doc in docs_meta}
    apk_entries = {doc["name"]: doc.get("original_entry") for doc in docs_meta}
    cards = {
        name: apply_bundle_checks(record, bundle_exports, doc_hashes, apk_entries)
        for name, record in sorted(plan["cards"].items())
    }
    always = apply_bundle_checks(plan["always"], bundle_exports, doc_hashes, apk_entries)
    summary = {
        "cards": len(cards),
        "complete": sum(card["status"] == "complete" for card in cards.values()),
        "partial": sum(card["status"] == "partial" for card in cards.values()),
        "unresolved": sum(card["status"] == "unresolved" for card in cards.values()),
        "documents": len(plan["documents"]),
        "bindings": len(plan["bindings"]),
        "missing_document_cards": sum(bool(card["missing_documents"]) for card in cards.values()),
        "missing_effect_cards": sum(bool(card["missing_effects"]) for card in cards.values()),
        "missing_emitter_cards": sum(bool(card["missing_emitters"]) for card in cards.values()),
        "missing_export_cards": sum(bool(card["missing_exports"]) for card in cards.values()),
        "always_status": always["status"],
    }
    return {
        "bindings": plan["bindings"],
        "card_dependency_documents": {
            name: card["documents"] for name, card in cards.items() if card["spell_found"]
        },
        "always_documents": list(plan["always_documents"]),
        "asset_coverage": {"basis": STATUS_BASIS, "summary": summary, "always": always, "cards": cards},
        "cards_selector": CARDS_SELECTOR,
    }


def build_presentation_graph(logic: Path, apk: Path, engine: Path, cards_argument: str = "", frame_paths: list[Path] | None = None) -> dict:
    """Rows, effects and emitters reachable from selectable cards, captures, towers, and the legacy seeds."""
    index, logic_hashes, spells, all_effects, all_emitters, effects_raw, emitters_raw = _context(logic, apk)
    requested = parse_cards_argument(cards_argument)
    selected = requested or selectable_native_cards(engine)
    frame_rows, frame_cards = names_from_frames(frame_paths or [])
    seeds = list(SEED_ROWS) + list(ALWAYS_ROWS)
    extra = list(EXTRA_EFFECTS)
    for name in list(selected) + sorted(frame_cards | frame_rows):
        spell = spells.get(name)
        if spell is not None:
            seeds.extend(root["target"] for root in spell["roots"])
            if spell.get("cast_effect"):
                extra.append(spell["cast_effect"]["name"])
        elif _cached_pick(index, {}, name) is not None or name in frame_rows:
            seeds.append(name)
    closed = _close(index, {}, all_effects, all_emitters, seeds, extra)
    return {
        "rows": {name: {"logic_file": row["logic_file"], "type": row["type"], "fields": row["fields"]}
                 for name, row in sorted(closed["rows"].items())},
        "effects": dict(sorted(closed["effects"].items())),
        "emitters": dict(sorted(closed["emitters"].items())),
        "unresolved_rows": closed["unresolved_rows"],
        "missing_effects": closed["missing_effects"],
        "missing_emitters": closed["missing_emitters"],
        "logic_hashes": logic_hashes,
        "effects_raw": effects_raw,
        "emitters_raw": emitters_raw,
        "export_refs": closed["export_refs"],
    }


def missing_exports_against_bundle(export_refs, bundle_exports: dict[str, set[str]]) -> list[str]:
    missing = []
    for ref in export_refs:
        document = ref.get("document") or ""
        export = ref.get("export") or ""
        if not export:
            continue
        names = bundle_exports.get(document, set())
        if export in names or _family_count(names, export) > 0:
            continue
        missing.append(f"{document}:{export}" if document else export)
    return sorted(set(missing))
