#!/usr/bin/env python3
"""Compare native display lists and pose pixels between two spectator bundles.

One run loads the whole bundle. The other can pass ``--cards`` so
``scene.always_documents`` plus ``scene.card_dependency_documents`` is what the
Painter loads. Pose pixels are the content-addressed textures clash-visual-check
writes. A match means every recorded frame's draws are equal and every texture
file hashes equal.
"""
from __future__ import annotations

import argparse
import gzip
import hashlib
import json
import shutil
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BINARY = ROOT / "engine/native-spectator/target/debug/clash-visual-check"
FIXTURES = ROOT / "engine/native-spectator/visual-fixtures"


def _hash_tree(directory: Path) -> dict[str, str]:
    hashes = {}
    if not directory.is_dir():
        return hashes
    for path in sorted(directory.iterdir()):
        if path.is_file():
            hashes[path.name] = hashlib.sha256(path.read_bytes()).hexdigest()
    return hashes


def _replay(frames: Path, assets: Path, out: Path, scenario: dict, cards: list[str] | None) -> None:
    if out.exists():
        shutil.rmtree(out)
    command = [str(BINARY), "--frames", str(frames), "--assets", str(assets), "--out", str(out),
               "--fps", str(scenario.get("fps", 20)), "--lag", str(scenario.get("lag", 0))]
    if "from_tick" in scenario:
        command += ["--from-tick", str(scenario["from_tick"])]
    if "to_tick" in scenario:
        command += ["--to-tick", str(scenario["to_tick"])]
    if cards:
        command += ["--cards", ",".join(cards)]
    result = subprocess.run(command, capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(result.stderr.strip()[-2000:] or result.stdout[-2000:])


def _compare(left: Path, right: Path) -> dict:
    draws_differ = 0
    frames = 0
    first = None
    with (left / "trace.jsonl").open() as a, (right / "trace.jsonl").open() as b:
        for line_a, line_b in zip(a, b):
            frames += 1
            rec_a, rec_b = json.loads(line_a), json.loads(line_b)
            if rec_a.get("draws") != rec_b.get("draws") or rec_a.get("t") != rec_b.get("t"):
                draws_differ += 1
                if first is None:
                    first = {"t": rec_a.get("t"), "left_draws": len(rec_a.get("draws") or []),
                             "right_draws": len(rec_b.get("draws") or [])}
        extra_left = sum(1 for _ in a)
        extra_right = sum(1 for _ in b)
    textures_left = _hash_tree(left / "textures")
    textures_right = _hash_tree(right / "textures")
    return {
        "frames": frames,
        "draw_frames_differ": draws_differ,
        "extra_left_frames": extra_left,
        "extra_right_frames": extra_right,
        "texture_files_left": len(textures_left),
        "texture_files_right": len(textures_right),
        "textures_equal": textures_left == textures_right,
        "first_draw_difference": first,
        "equal": draws_differ == 0 and extra_left == 0 and extra_right == 0 and textures_left == textures_right,
    }


def _scenarios(suite: str) -> list[dict]:
    catalog = json.loads((FIXTURES / "scenarios.json").read_text())
    by_name = {item["name"]: item for item in catalog["scenarios"]}
    return [by_name[name] for name in catalog["suites"][suite]]


def _cards_for(capture: str) -> list[str]:
    meta = json.loads((FIXTURES / "captures" / f"{capture}.json").read_text())
    names = []
    for deck in meta.get("decks") or []:
        for card in deck:
            if card not in names:
                names.append(card)
    return names


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--full", type=Path, required=True, help="bundle loaded without --cards")
    parser.add_argument("--subset", type=Path, required=True, help="derived bundle")
    parser.add_argument("--suite", default="full", choices=["quick", "full", "ghost"])
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--cards", action="store_true", help="also compare full load vs subset --cards for each fixture")
    args = parser.parse_args()
    if not BINARY.is_file():
        raise SystemExit(f"missing {BINARY}; build clash-visual-check first")
    rows = []
    failures = 0
    scenarios = _scenarios(args.suite)
    with tempfile.TemporaryDirectory(prefix="arena-display-diff-") as temp:
        temp = Path(temp)
        for index, scenario in enumerate(scenarios, start=1):
            capture = FIXTURES / "captures" / f"{scenario['capture']}.jsonl.gz"
            frames = temp / f"{scenario['name']}.jsonl"
            with gzip.open(capture, "rb") as src, frames.open("wb") as dst:
                shutil.copyfileobj(src, dst)
            full_out = temp / f"{scenario['name']}-full"
            subset_out = temp / f"{scenario['name']}-subset"
            _replay(frames, args.full, full_out, scenario, None)
            _replay(frames, args.subset, subset_out, scenario, None)
            compared = _compare(full_out, subset_out)
            row = {"scenario": scenario["name"], "capture": scenario["capture"], "subset_vs_full": compared}
            if args.cards:
                cards = _cards_for(scenario["capture"])
                cards_out = temp / f"{scenario['name']}-cards"
                _replay(frames, args.subset, cards_out, scenario, cards)
                row["cards"] = cards
                row["cards_vs_full"] = _compare(full_out, cards_out)
            ok = row["subset_vs_full"]["equal"] and (not args.cards or row["cards_vs_full"]["equal"])
            failures += not ok
            rows.append(row)
            print(f"{index}/{len(scenarios)} {scenario['name']} {'OK' if ok else 'DIFF'}", flush=True)
            shutil.rmtree(full_out, ignore_errors=True)
            shutil.rmtree(subset_out, ignore_errors=True)
            if args.cards:
                shutil.rmtree(temp / f"{scenario['name']}-cards", ignore_errors=True)
            frames.unlink(missing_ok=True)
    args.out.parent.mkdir(parents=True, exist_ok=True)
    payload = {"suite": args.suite, "failures": failures, "scenarios": rows}
    args.out.write_text(json.dumps(payload, indent=2) + "\n")
    print(json.dumps({"failures": failures, "scenarios": len(rows), "report": str(args.out)}), flush=True)
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
