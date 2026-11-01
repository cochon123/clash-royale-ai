#!/usr/bin/env python3
"""Build the C1 Arena asset pack from a native-spectator bundle.

Writes a derived bundle (scene.always_documents, scene.card_dependency_documents,
and a ui/effects subset) and a content-addressed ``files/`` tree. JSON files get
``.gz`` and ``.br`` siblings. ``manifest.json`` points at ``manifest-<hash>.json``.

``--painter-wasm PATH`` content-addresses that binary and sets ``painter_wasm``.
Without the flag, ``painter_wasm`` is null and no wasm file is published. This
script does not compile the painter.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "engine"))

from tools.web_assets import build  # noqa: E402


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--assets", type=Path, required=True, help="source native-spectator bundle (read only)")
    parser.add_argument("--out", type=Path, required=True, help="content-addressed pack directory")
    parser.add_argument("--derived", type=Path, help="derived bundle directory (default: deploy/arena/derived-bundle)")
    parser.add_argument("--painter-wasm", type=Path, help="optional clash_painter.wasm; omitted leaves painter_wasm null")
    parser.add_argument("--report", type=Path, help="size report (default: deploy/arena/web-assets-report.json)")
    parser.add_argument("--presets", type=Path, help="preset deck catalog")
    args = parser.parse_args()
    derived = args.derived or (ROOT / "deploy/arena/derived-bundle")
    report = args.report or (ROOT / "deploy/arena/web-assets-report.json")
    presets = args.presets or (ROOT / "engine/data/fast_league_decks.json")
    summary = build(args.assets, args.out, derived, ROOT / "engine", args.painter_wasm, report, presets)
    print({
        "out": str(args.out),
        "derived": str(derived),
        "report": str(report),
        "painter_wasm": summary["painter_wasm"],
        "preset_pairs": len(summary["preset_pairs"]),
        "worst_wire_bytes": summary["worst_custom_pair"]["wire_bytes"],
        "ui_json_bytes": [summary["subset"]["ui"]["json_bytes_before"], summary["subset"]["ui"]["json_bytes_after"]],
        "effects_json_bytes": [summary["subset"]["effects"]["json_bytes_before"], summary["subset"]["effects"]["json_bytes_after"]],
    }, flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
