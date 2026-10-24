#!/usr/bin/env python3
"""Native-vs-wasm painter parity for one visual fixture.

The local bundle has no scene.card_dependency_documents. This script writes a
temporary index (outside the shared bundle) that adds those keys, then records
packets with clash-fixture-record and replays them through clash_painter.wasm.

Output of the recorder (clash-painter-fixture-v1) is what W3's fake painter
replays. See the docstring in record() and RESULTS-W1.md.
"""

from __future__ import annotations

import argparse
import gzip
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
SPECTATOR = ROOT / "engine" / "native-spectator"
BUNDLE = ROOT / "engine" / "analysis" / "native-spectator"
CATALOG = ROOT / "engine" / "data" / "card_catalog.json"
NODE = Path("/home/cochon/.local/bin/node")
RECORDER = SPECTATOR / "target" / "debug" / "clash-fixture-record"
WASM = SPECTATOR / "target" / "wasm32-unknown-unknown" / "release" / "clash_painter.wasm"

# Temporary stand-in for the index W2 produces. Spells in this bundle live in
# `effects`; there is no separate fireball/log/rocket document.
ALWAYS = [
    "arena_training",
    "building_tower",
    "effects",
    "ui",
    "ui_spectate_hud",
    "chr_king",
    "chr_princess",
    "level_bootcamp_arena_dl",
]
CARD_DOCS = {
    "HogRider": ["chr_hog_rider"],
    "Musketeer": ["chr_musketeer"],
    "IceGolemite": ["chr_snowman"],
    "Skeletons": ["chr_skeleton"],
    "IceSpirits": ["chr_ice_spirits"],
    "Cannon": ["building_basic_cannon"],
    "Fireball": ["effects"],
    "Log": ["effects"],
    "GoblinBarrel": ["spell_goblin_barrel", "chr_goblin"],
    "Princess": ["chr_princess"],
    "Knight": ["chr_knight"],
    "GoblinGang": ["chr_goblin", "chr_goblin_archer"],
    "InfernoTower": ["building_inferno_tower"],
    "Rocket": ["effects"],
}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--capture", default="hog-mirror")
    parser.add_argument("--from-tick", type=int, default=140)
    parser.add_argument("--limit", type=int, default=48)
    parser.add_argument("--gate-frames", type=int, default=24)
    parser.add_argument("--lag", type=float, default=3.0)
    parser.add_argument("--fps", type=float, default=30.0)
    parser.add_argument("--out", type=Path, default=None)
    parser.add_argument("--skip-wasm", action="store_true")
    args = parser.parse_args()
    out = args.out or (SPECTATOR / "target" / "wasm-parity" / args.capture)
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    cards = cards_for(args.capture)
    missing = [card for card in cards if card not in CARD_DOCS]
    if missing:
        raise SystemExit(f"no temporary dependency documents for {missing}")
    index = out / "card-index"
    vfs = prepare_index(index, cards)
    frames = out / "frames.jsonl"
    extract_frames(args.capture, frames, args.from_tick, args.limit)
    if not RECORDER.is_file():
        raise SystemExit(f"missing {RECORDER}; cargo build --bin clash-fixture-record")
    record = subprocess.run(
        [
            str(RECORDER),
            "--assets",
            str(index),
            "--frames",
            str(frames),
            "--out",
            str(out / "native"),
            "--cards",
            ",".join(cards),
            "--lag",
            str(args.lag),
            "--fps",
            str(args.fps),
            "--address-gate",
            "--gate-frames",
            str(args.gate_frames),
        ],
        check=False,
        text=True,
    )
    if record.returncode != 0:
        return record.returncode
    report = {
        "capture": args.capture,
        "cards": cards,
        "from_tick": args.from_tick,
        "limit": args.limit,
        "index": "temporary; scene.always_documents and scene.card_dependency_documents were injected locally",
        "address_gate": json.loads((out / "native" / "address-gate.json").read_text()),
    }
    if not args.skip_wasm:
        if not WASM.is_file():
            raise SystemExit(f"missing {WASM}; run engine/native-spectator/build-wasm.sh")
        cards_path = out / "cards.json"
        cards_path.write_text(json.dumps(cards))
        wasm_report = out / "wasm-report.json"
        node = subprocess.run(
            [
                str(NODE),
                str(SPECTATOR / "tests" / "wasm_parity" / "runner.mjs"),
                "--wasm",
                str(WASM),
                "--frames",
                str(frames),
                "--vfs",
                str(vfs),
                "--native-dir",
                str(out / "native"),
                "--cards",
                str(cards_path),
                "--out",
                str(wasm_report),
                "--lag",
                str(args.lag),
                "--fps",
                str(args.fps),
            ],
            check=False,
        )
        if wasm_report.is_file():
            report["wasm"] = json.loads(wasm_report.read_text())
        (out / "report.json").write_text(json.dumps(report, indent=2))
        print(json.dumps({k: report[k] for k in ("capture", "address_gate") if k in report}, indent=2)[:2000])
        return node.returncode
    (out / "report.json").write_text(json.dumps(report, indent=2))
    return 0


def cards_for(capture: str) -> list[str]:
    sidecar = SPECTATOR / "visual-fixtures" / "captures" / f"{capture}.json"
    if sidecar.is_file():
        meta = json.loads(sidecar.read_text())
        names: list[str] = []
        for deck in meta.get("decks", []):
            for card in deck:
                if card not in names:
                    names.append(card)
        if names:
            return names
    raise SystemExit(f"no decks in {sidecar}")


def prepare_index(dest: Path, cards: list[str]) -> Path:
    assets = json.loads((BUNDLE / "assets.json").read_text())
    selected = set(ALWAYS)
    deps = {}
    for card in cards:
        docs = CARD_DOCS[card]
        deps[card] = docs
        selected.update(docs)
    assets["scene"]["always_documents"] = ALWAYS
    assets["scene"]["card_dependency_documents"] = deps
    dest.mkdir(parents=True)
    (dest / "assets.json").write_text(json.dumps(assets))
    link(BUNDLE / "presentation.json", dest / "presentation.json")
    link(BUNDLE / "fonts", dest / "fonts")
    link(BUNDLE / "cards", dest / "cards")
    (dest / "display-trees").mkdir()
    (dest / "atlases").mkdir()
    by_name = {doc["name"]: doc for doc in assets["documents"]}
    missing = sorted(name for name in selected if name not in by_name)
    if missing:
        raise SystemExit(f"bundle is missing documents {missing}")
    for name in sorted(selected):
        link(BUNDLE / "display-trees" / f"{name}.json", dest / "display-trees" / f"{name}.json")
        for atlas in by_name[name]["atlases"]:
            source = BUNDLE / atlas["file"]
            link(source, dest / "atlases" / source.name)
    files = []
    # os.walk follows the fonts/ and cards/ directory symlinks. Path.rglob does not.
    for dirpath, _dirnames, filenames in os.walk(dest, followlinks=True):
        for name in filenames:
            path = Path(dirpath) / name
            if not path.is_file():
                continue
            files.append(
                {
                    "path": path.relative_to(dest).as_posix(),
                    "source": str(path.resolve()),
                }
            )
    files.sort(key=lambda item: item["path"])
    if CATALOG.is_file():
        files.append({"path": "data/card_catalog.json", "source": str(CATALOG.resolve())})
    names = {item["path"] for item in files}
    if "fonts/supercell-magic_0.ttf" not in names:
        raise SystemExit("vfs manifest is missing fonts/supercell-magic_0.ttf")
    manifest = dest.parent / "vfs.json"
    manifest.write_text(json.dumps(files))
    print(f"vfs {len(files)} files", flush=True)
    return manifest


def link(source: Path, dest: Path) -> None:
    if dest.exists() or dest.is_symlink():
        return
    dest.symlink_to(source)


def extract_frames(capture: str, dest: Path, from_tick: int, limit: int) -> None:
    captures = SPECTATOR / "visual-fixtures" / "captures"
    gzipped = captures / f"{capture}.jsonl.gz"
    plain = captures / f"{capture}.jsonl"
    if gzipped.is_file():
        handle = gzip.open(gzipped, "rt")
    elif plain.is_file():
        handle = plain.open()
    else:
        raise SystemExit(f"no capture for {capture}")
    kept = 0
    with handle, dest.open("w") as out:
        for line in handle:
            if not line.strip():
                continue
            tick = json.loads(line)["tick"]
            if tick < from_tick:
                continue
            out.write(line if line.endswith("\n") else line + "\n")
            kept += 1
            if kept >= limit:
                break
    if kept == 0:
        raise SystemExit(f"no frames at or after tick {from_tick}")


if __name__ == "__main__":
    sys.exit(main())
