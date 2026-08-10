"""Apply the predeclared v7 pilot go/no-go gates to two pilot reports."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


def _read(path: str | Path) -> dict:
    return json.loads(Path(path).read_text(encoding="utf-8"))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--aligned", required=True)
    parser.add_argument("--shuffled", required=True)
    parser.add_argument("--baseline", required=True)
    parser.add_argument("--probe", required=True)
    args = parser.parse_args()
    aligned = _read(args.aligned)
    shuffled = _read(args.shuffled)
    baseline = _read(args.baseline)
    probe = _read(args.probe)
    a = aligned.get("val") or {}
    s = shuffled.get("val") or {}
    b = baseline.get("metrics") or baseline.get("test") or {}
    ablations = probe.get("ablations") or {}
    disabled = ablations.get("arena_memory_disabled") or {}
    normal = ablations.get("oracle_full_opponent_deck") or {}
    gates = {
        "aligned_nll_beats_baseline_0_02": float(a.get("tile_nll", 1e9)) <= float(b.get("tile_nll", 1e9)) - 0.02,
        "aligned_nll_beats_shuffled_0_01": float(a.get("tile_nll", 1e9)) <= float(s.get("tile_nll", 1e9)) - 0.01,
        "aligned_top1_beats_shuffled_0_5pp": float(a.get("tile_class_acc", 0.0)) >= float(s.get("tile_class_acc", 0.0)) + 0.005,
        "adapter_off_removes_half_gain": (
            float(normal.get("tile_top1", 0.0)) - float(disabled.get("tile_top1", 0.0))
            >= 0.5 * max(float(normal.get("tile_top1", 0.0)) - float(b.get("tile_class_acc", 0.0)), 0.0)
        ),
    }
    aligned["promotion_gates"] = gates
    aligned["verdict"] = "rejected" if not all(gates.values()) else "pilot_passed"
    aligned["verdict_reason"] = (
        "The aligned arena-memory adapter did not clear the predeclared shuffled-control thresholds; stop before full training."
        if not all(gates.values())
        else "The cheap aligned-versus-shuffled pilot cleared all predeclared thresholds."
    )
    target = Path(args.aligned)
    target.write_text(json.dumps(aligned, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"verdict": aligned["verdict"], "promotion_gates": gates}, indent=2))


if __name__ == "__main__":
    main()

