"""Redraw the held-out winner confidence chart from the committed JSON.

Training already writes ``models/winner_predictor/accuracy_vs_confidence.json``
with two selective-prediction curves (previous model and improved model).
The PNG next to the checkpoint is gitignored, so this script regenerates a
copy that can live in the repo:

    python scripts/plot_winner_confidence.py
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT / "src") not in sys.path:
    sys.path.insert(0, str(ROOT / "src"))

from cr_replay_pipeline.winner_visuals import save_confidence_plot

DEFAULT_INPUT = ROOT / "models" / "winner_predictor" / "accuracy_vs_confidence.json"
DEFAULT_OUTPUT = ROOT / "reports" / "winner_accuracy_vs_confidence.png"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, default=DEFAULT_INPUT)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()
    payload = json.loads(args.input.read_text(encoding="utf-8"))
    baseline = payload.get("baseline") or []
    improved = payload.get("improved") or []
    if not baseline or not improved:
        raise SystemExit(f"{args.input} is missing baseline or improved points")
    for name, curve in (("baseline", baseline), ("improved", improved)):
        for point in curve:
            for key in ("min_confidence", "accuracy", "coverage"):
                if key not in point:
                    raise SystemExit(f"{name} point missing {key}")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    save_confidence_plot(baseline, improved, args.output)
    print(
        f"Wrote {args.output} "
        f"({len(baseline)} baseline points, {len(improved)} improved points)"
    )


if __name__ == "__main__":
    main()
