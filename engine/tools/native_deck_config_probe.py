#!/usr/bin/env python3
"""Prepare a source-bound LogicTestingData deck override for original-native runs.

This writes a derived testing.csv and an exact-path read-only VFS manifest. The
native engine still parses the CSV and constructs every card/deck object. The
override must be supplied before the testing table is loaded; it cannot be
applied to an already-postloaded checkpoint.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_TABLES = ROOT / "analysis/decoded/assets/csv_logic"
SOURCE_SHA256 = "79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6"

# Card display names mapped to the exact native LogicData row names in this
# pinned game's CSV tables.
DECKS = {
    "hog26": {
        "scenario": "Normal",
        "cards": [
            "HogRider", "Musketeer", "IceGolemite", "Skeletons",
            "IceSpirits", "Cannon", "Fireball", "Log",
        ],
        "labels": [
            "Hog Rider", "Musketeer", "Ice Golem", "Skeletons",
            "Ice Spirit", "Cannon", "Fireball", "The Log",
        ],
    },
    "pixel9_mortar": {
        "scenario": "Normal",
        "cards": [
            "Mortar", "Knight", "Archer", "Skeletons",
            "BarbLog", "Arrows", "MiniPekka", "Rocket",
        ],
        "labels": [
            "Mortar", "Knight", "Archers", "Skeletons",
            "Barbarian Barrel", "Arrows", "Mini P.E.K.K.A", "Rocket",
        ],
    },
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def card_rows(tables_dir: Path) -> set[str]:
    names: set[str] = set()
    for filename in (
        "spells_characters.csv",
        "spells_buildings.csv",
        "spells_other.csv",
    ):
        path = tables_dir / filename
        with path.open("r", encoding="utf-8-sig", newline="") as stream:
            rows = csv.reader(stream)
            next(rows, None)
            next(rows, None)
            names.update(row[0] for row in rows if row and row[0])
    return names


def build(args: argparse.Namespace) -> dict:
    deck = DECKS[args.deck]
    source_path = args.testing_csv.resolve()
    source_bytes = source_path.read_bytes()
    source_hash = sha256(source_bytes)
    rows = list(csv.reader(source_bytes.decode("utf-8-sig").splitlines()))
    if len(rows) < 2 or rows[0] != [
        "Name", "IsEnabled", "TestType", "Flags", "AlwaysSpells",
        "NeverSpells", "GameMode", "UseEvolution", "UseHeroForm",
    ]:
        raise ValueError(f"Unexpected LogicTestingData schema in {source_path}")
    normal = next((row for row in rows[2:] if row and row[0] == deck["scenario"]), None)
    if normal is None:
        raise ValueError("The pinned testing table has no Normal scenario row")

    available = card_rows(args.tables_dir.resolve())
    missing = [name for name in deck["cards"] if name not in available]
    if missing:
        raise ValueError("Native card rows not found: " + ", ".join(missing))

    # Override the existing row used by the stock harness, preserving its
    # native scenario type and flags. Its normal deck constructor consumes
    # AlwaysSpells independently for both sides.
    normal[1] = "true"
    normal[4] = ";".join(deck["cards"])
    csv_text = "\n".join(",".join('"' + cell.replace('"', '""') + '"' for cell in row)
                        for row in rows) + "\n"
    output = args.out_dir.resolve()
    output.mkdir(parents=True, exist_ok=False)
    testing_out = output / "testing.csv"
    testing_bytes = csv_text.encode("utf-8")
    testing_out.write_bytes(testing_bytes)
    manifest = {
        "csv_logic/testing.csv": {
            "path": str(testing_out),
            "sha256": sha256(testing_bytes),
        }
    }
    manifest_path = output / "virtual-files.json"
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    metadata = {
        "source_elf_sha256": SOURCE_SHA256,
        "source_testing_csv": str(source_path),
        "source_testing_csv_sha256": source_hash,
        "derived_testing_csv": str(testing_out),
        "derived_testing_csv_sha256": sha256(testing_bytes),
        "virtual_files_manifest": str(manifest_path),
        "scenario": deck["scenario"],
        "native_card_rows": deck["cards"],
        "display_labels": deck["labels"],
        "construction_contract": {
            "table_id": 121,
            "field": "AlwaysSpells",
            "source_factory_rva": "0x11d7250",
            "deck_builder_rva": "0xfea548",
            "battle_slot_setter_rva": "0xd3bd58",
        "scope": "The existing Normal scenario is selected by the stock harness. AlwaysSpells is consumed for each side, so this prepares the same exact eight-card deck for both sides.",
        },
        "limitations": [
            "This prepares inputs only; original-native execution must confirm the loaded row and live deck.",
            "Use the VFS manifest before testing.csv is parsed; a postloaded context checkpoint has already consumed the original table.",
            "Per-side asymmetric decks are supported at runtime by the native-watch/deck-setup-probe `deck_preset_side1` field; this CSV override still prepares a single mirrored AlwaysSpells vector.",
        ],
    }
    metadata_path = output / "deck-config.json"
    metadata_path.write_text(json.dumps(metadata, indent=2) + "\n", encoding="utf-8")
    return metadata


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--deck", choices=sorted(DECKS), required=True)
    parser.add_argument("--out-dir", type=Path, required=True,
                        help="New output directory for derived CSV, VFS manifest and provenance")
    parser.add_argument("--testing-csv", type=Path,
                        default=DEFAULT_TABLES / "testing.csv")
    parser.add_argument("--tables-dir", type=Path, default=DEFAULT_TABLES,
                        help="Directory containing the pinned spells_*.csv source rows")
    args = parser.parse_args()
    result = build(args)
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
