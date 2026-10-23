#!/usr/bin/env python3
"""Map presentation row names to catalog cards.

Reads observed row names from a JSON list (or collects them by replaying
league traces with board export) and writes engine/data/board_rows.json.
Catalog gaps are marked uncertain instead of being guessed.
"""
from __future__ import annotations

import argparse
import gzip
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def load_json(path: Path):
    return json.loads(path.read_text())


def fields_of(entry):
    if isinstance(entry, list):
        entry = entry[0]
    return entry.get("fields", entry)


def build_index(catalog):
    cards = []
    for table in ("spells_characters", "spells_buildings", "spells_other"):
        for record in catalog["card_tables"][table]["records"]:
            fields = record["fields"]
            if fields.get("NotInUse") or fields.get("NotVisible"):
                continue
            cards.append({
                "card": record["name"],
                "elixir": fields.get("ManaCost"),
                "summon": fields.get("SummonCharacter"),
                "count": fields.get("SummonNumber"),
                "count_explicit": "SummonNumber" in fields,
                "second": fields.get("SummonCharacterSecond"),
                "second_count": fields.get("SummonCharacterSecondCount"),
                "projectile": fields.get("Projectile"),
                "area": fields.get("AreaEffectObject"),
                "spell_type": fields.get("TypeOfSpell"),
                "table": table,
            })
    characters = {
        name: fields_of(row) for name, row in catalog["definitions"]["CHARACTER"].items()
    }
    buildings = {
        name: fields_of(row) for name, row in catalog["definitions"]["BUILDING"].items()
    }
    projectiles = {
        name: fields_of(row) for name, row in catalog["definitions"]["PROJECTILE"].items()
    }
    aeos = {name: fields_of(row) for name, row in catalog["definitions"]["AEO"].items()}
    return cards, characters, buildings, projectiles, aeos


def _chain_reaches(projectiles, start, target):
    seen = set()
    current = start
    while current and current not in seen:
        if current == target:
            return True
        seen.add(current)
        current = projectiles.get(current, {}).get("SpawnProjectile")
    return False


def _direct_cards(cards, character):
    found = []
    for card in cards:
        if card["summon"] == character:
            found.append((card, "SummonCharacter", card["count"], card["count_explicit"]))
        if card["second"] == character:
            found.append((card, "SummonCharacterSecond", card["second_count"], card["second_count"] is not None))
    return found


def describe(row, cards, characters, buildings, projectiles, aeos):
    notes = []
    links = []
    kind = "unknown"
    entities = {**characters, **buildings}

    def add(card, why, units, count_known, role):
        links.append({
            "card": card["card"], "elixir": card["elixir"], "why": why,
            "units_per_card": units if count_known else None, "role": role,
            "count_known": count_known,
        })

    if row in ("KingTower", "PrincessTower"):
        projectile = buildings.get(row, {}).get("Projectile")
        note = "Arena tower placed by the match, not by a card."
        if projectile:
            note += f" Attacks with {projectile}."
        return {
            "row": row, "kind": "tower", "source_card": None, "source_cards": [],
            "elixir": None, "units_per_card": None, "uncertain": False, "notes": note,
        }

    for card, why, units, known in _direct_cards(cards, row):
        kind = "building" if card["table"] == "spells_buildings" else "troop"
        if not known:
            units = 1
            notes.append(f"{card['card']} omits SummonNumber; catalog reconstruction uses 1")
        add(card, why, units, True, "deploy")
        if not known:
            links[-1]["count_known"] = False

    for parent, fields in entities.items():
        if fields.get("DeathSpawnCharacter") != row:
            continue
        kind = "troop"
        count = fields.get("DeathSpawnCount")
        known = "DeathSpawnCount" in fields
        parents = _direct_cards(cards, parent)
        if not parents:
            notes.append(f"death spawn of {parent}, which is not a direct card summon")
        for card, _why, _units, _known in parents:
            shown = count if known else "unknown"
            add(card, f"DeathSpawnCharacter of {parent} x{shown}", count, known, "death")
            if not known:
                notes.append(f"DeathSpawnCount absent on {parent}; not defaulting it")

    for projectile_name, fields in projectiles.items():
        if fields.get("SpawnCharacter") != row:
            continue
        kind = "troop"
        count = fields.get("SpawnCharacterCount")
        known = "SpawnCharacterCount" in fields
        owners = [card for card in cards if card["projectile"] and _chain_reaches(projectiles, card["projectile"], projectile_name)]
        if not owners:
            notes.append(f"projectile {projectile_name} spawns this row but no card Projectile field reaches it")
        for card in owners:
            add(card, f"projectile {projectile_name} SpawnCharacter", count, known, "projectile_deploy")
        if not known:
            notes.append(f"SpawnCharacterCount absent on {projectile_name}; not defaulting it")

    for parent, fields in entities.items():
        if fields.get("SpawnCharacter") != row:
            continue
        number = fields.get("SpawnNumber")
        notes.append(f"{parent} periodically spawns {number if number is not None else '?'} {row}")
        for card, _why, _units, _known in _direct_cards(cards, parent):
            add(card, f"periodic SpawnCharacter of {parent}", number, number is not None, "periodic")

    same = [card for card in cards if card["card"] == row and not card["summon"] and (row in characters or row in buildings)]
    for card in same:
        kind = "troop" if row in characters else "building"
        add(card, "card name matches the row and SummonCharacter is absent", None, False, "inferred_summon")
        notes.append(f"{card['card']} has no SummonCharacter; the matching character exists, so the unit count is not in the card table")

    if row in projectiles:
        kind = "projectile"
        fields = projectiles[row]
        if fields.get("SpawnCharacter"):
            count = fields.get("SpawnCharacterCount")
            notes.append(f"this projectile spawns {count if count is not None else '?'} {fields['SpawnCharacter']}")
        if fields.get("SpawnProjectile"):
            notes.append(f"chains to projectile {fields['SpawnProjectile']}")
        owners = [card for card in cards if card["projectile"] and _chain_reaches(projectiles, card["projectile"], row)]
        for name, entity in entities.items():
            if entity.get("Projectile") == row or entity.get("CustomFirstProjectile") == row:
                notes.append(f"fired by {name}")
                for card, _why, _units, _known in _direct_cards(cards, name):
                    owners.append(card)
                if name in ("KingTower", "PrincessTower"):
                    notes.append(f"{row} belongs to the {name}, not to a card")
        for card in owners:
            add(card, f"projectile owned by {card['card']}", None, False, "projectile")
        if not owners and not any("belongs to" in note for note in notes):
            notes.append("no recovered card or tower Projectile field points at this row")

    if row in aeos:
        kind = "area_effect"
        referenced = []
        for name, entity in entities.items():
            hits = [key for key, value in entity.items() if value == row]
            if hits:
                referenced.append(f"{name}.{','.join(hits)}")
        for card in cards:
            if card["area"] == row:
                referenced.append(f"card {card['card']}.AreaEffectObject")
                add(card, "card AreaEffectObject", None, False, "effect")
        for name, fields in aeos.items():
            if name == row:
                continue
            hits = [key for key, value in fields.items() if value == row]
            if hits:
                referenced.append(f"AEO {name}.{hits[0]}")
        if referenced:
            notes.append("area effect referenced by " + "; ".join(referenced))
        else:
            notes.append("area-effect row with no recovered field pointing at this name")
        for name, entity in entities.items():
            hits = [key for key, value in entity.items() if value == row]
            if not hits:
                continue
            for card, _why, _units, _known in _direct_cards(cards, name):
                add(card, f"{name} {hits[0]}", None, False, "effect")

    if kind == "unknown" and row in entities:
        kind = "building" if row in buildings else "troop"

    for link in links:
        if link["role"] not in {"deploy", "projectile_deploy"}:
            notes.append(link["why"])
    cards_named = sorted({link["card"] for link in links})
    source_card = cards_named[0] if len(cards_named) == 1 else None
    elixir_values = {link["elixir"] for link in links}
    elixir = next(iter(elixir_values)) if len(cards_named) == 1 and len(elixir_values) == 1 else None
    uncertain = False
    units = None
    if kind in {"projectile", "area_effect"}:
        if len(cards_named) == 1:
            notes.append("not a deployed unit; units_per_card does not apply")
        elif any("belongs to the" in note for note in notes) and not cards_named:
            elixir = None
        else:
            uncertain = True
            if len(cards_named) > 1:
                notes.append("more than one catalog card can produce this row")
            elif not notes:
                notes.append("no single card or tower owns this row")
    elif kind == "tower":
        uncertain = False
    else:
        deploy = [link for link in links if link["role"] in {"deploy", "projectile_deploy"}]
        other = [link for link in links if link["role"] not in {"deploy", "projectile_deploy"}]
        if len(cards_named) != 1 or other or not links:
            uncertain = True
        if len(cards_named) > 1:
            notes.append("more than one catalog card can produce this row")
        if not links:
            notes.append("no summon, death-spawn, projectile, or area-effect link in the recovered catalog")
        known = [link["units_per_card"] for link in deploy if link["count_known"]]
        unknown = [link for link in deploy if not link["count_known"]]
        if len(set(known)) > 1:
            notes.append("direct deploys do not agree on units per card")
        if len(set(known)) == 1 and not unknown and not other and len(cards_named) == 1:
            units = known[0]
        elif len(deploy) == 1 and not other and len(cards_named) == 1 and not deploy[0]["count_known"]:
            units = 1
            uncertain = True
        elif not deploy and other and len({link["units_per_card"] for link in other if link["role"] == "death" and link["count_known"]}) == 1 and all(link["role"] == "death" and link["count_known"] for link in other):
            units = next(link["units_per_card"] for link in other if link["role"] == "death")
            uncertain = True
    if not notes and links:
        notes.append(links[0]["why"])
    return {
        "row": row,
        "kind": kind,
        "source_card": source_card,
        "source_cards": cards_named,
        "elixir": elixir,
        "units_per_card": units,
        "uncertain": uncertain,
        "notes": "; ".join(dict.fromkeys(notes)),
    }


def collect_from_traces(binary, traces, limit, workers):
    """Replay trace actions with board export and return the row names seen."""
    import os
    import selectors
    import subprocess
    import time
    from concurrent.futures import ThreadPoolExecutor

    checkpoint = ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"

    class Engine:
        def __init__(self):
            self.proc = subprocess.Popen(
                ["systemd-run", "--user", "--scope", "--quiet",
                 "-p", "MemoryMax=6G", "-p", "MemorySwapMax=0", "--",
                 str(binary), "--checkpoint", str(checkpoint)],
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                cwd=ROOT, bufsize=0,
            )
            self.selector = selectors.DefaultSelector()
            self.selector.register(self.proc.stdout, selectors.EVENT_READ)
            self.pending = b""

        def request(self, obj):
            self.proc.stdin.write(json.dumps(obj, separators=(",", ":")).encode() + b"\n")
            end = time.monotonic() + 180
            while b"\n" not in self.pending:
                remaining = end - time.monotonic()
                if remaining <= 0 or not self.selector.select(remaining):
                    raise TimeoutError("board collection timed out")
                data = os.read(self.proc.stdout.fileno(), 1 << 20)
                if not data:
                    raise RuntimeError(f"engine exited {self.proc.poll()}")
                self.pending += data
            line, self.pending = self.pending.split(b"\n", 1)
            reply = json.loads(line)
            if reply.get("ok") is not True:
                raise RuntimeError(str(reply.get("error")))
            return reply["result"]

        def close(self):
            if self.proc.poll() is None:
                try:
                    self.request({"op": "close"})
                except Exception:
                    self.proc.kill()
                self.proc.wait(timeout=10)
            self.selector.close()

    def one(path):
        with gzip.open(path, "rt") as handle:
            trace = json.load(handle)
        rows = set()
        engine = Engine()
        try:
            for item in trace["records"]:
                request = dict(item["request"])
                if request.get("op") == "reset":
                    request["board"] = True
                if request.get("op") not in {"reset", "step", "observe"}:
                    continue
                result = engine.request(request)
                board = result.get("board") or {}
                for entity in board.get("entities") or []:
                    rows.add(entity["row"])
                if result.get("terminated") or result.get("truncated"):
                    break
        finally:
            engine.close()
        return sorted(rows)

    paths = sorted(Path(traces).glob("*.json.gz"))
    if limit:
        paths = paths[:limit]
    found = set()
    with ThreadPoolExecutor(max_workers=workers) as pool:
        for rows in pool.map(one, paths):
            found.update(rows)
            print(json.dumps({"games_rows": len(found), "latest": rows[:8]}), flush=True)
    return sorted(found)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--observed", type=Path, help="JSON list of row names")
    parser.add_argument("--traces", type=Path)
    parser.add_argument("--binary", type=Path)
    parser.add_argument("--limit", type=int, default=0)
    parser.add_argument("--workers", type=int, default=2)
    parser.add_argument("--out", type=Path, default=ROOT / "data/board_rows.json")
    parser.add_argument("--label", default="")
    args = parser.parse_args()
    catalog = load_json(ROOT / "data/recovered_catalog.json")
    index = build_index(catalog)
    if args.observed:
        rows = load_json(args.observed)
    elif args.traces and args.binary:
        rows = collect_from_traces(args.binary, args.traces, args.limit, args.workers)
    else:
        raise SystemExit("pass --observed or both --traces and --binary")
    described = [describe(row, *index) for row in rows]
    # Tower rows are certain even though source_card is null.
    for row in described:
        if row["kind"] == "tower":
            row["uncertain"] = False
    payload = {
        "description": "Presentation row names seen in real league games, joined to the recovered card catalog.",
        "label": args.label,
        "row_count": len(described),
        "uncertain_count": sum(row["uncertain"] for row in described),
        "rows": described,
    }
    args.out.write_text(json.dumps(payload, indent=2) + "\n")
    print(json.dumps({"wrote": str(args.out), "rows": payload["row_count"], "uncertain": payload["uncertain_count"]}))


if __name__ == "__main__":
    main()
