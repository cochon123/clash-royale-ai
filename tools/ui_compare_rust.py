#!/usr/bin/env python3
"""Compare Rust SC5 playback with the independent original-asset Python reader.

This verifies the Rust asset port against an independent raster reference.
It does not establish parity with a running official Clash Royale client.
Requires the isolated environment installed by tools/ui_setup.sh.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

import numpy as np
from PIL import Image

from ui_recover import corrected_parser


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=Path("native-ui-viewer/target/release/clash-native-ui-viewer"))
    parser.add_argument("--assets", type=Path, default=Path("analysis/native-ui"))
    parser.add_argument("--output", type=Path, default=Path("analysis/native-ui/rust-verification"))
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    SC5File = corrected_parser()
    cache = {}

    def reference(document):
        if document not in cache:
            sc = SC5File(args.assets / "source" / f"{document}.sc")
            textures = [Image.open(args.assets / "atlases" / (Path(t["external"]).stem + ".png")).convert("RGBA") for t in sc.textures]
            cache[document] = sc, textures
        return cache[document]

    cases = [("ui", "HUD_player", 0), ("ui_spectate_hud", "spectate_hud_1v1_player", 0),
             ("ui", "healthbar_player_shield_champion_anim", 0), ("ui", "panel_ingame_blue", 0)]
    sc, _ = reference("ui_ability_button")
    count = len(sc.movie_clip_data[sc.exports["ability_ingame_button"]].frame_element_counts)
    cases += [("ui_ability_button", "ability_ingame_button", frame) for frame in range(count)]
    results = []
    for document, export, frame in cases:
        sc, textures = reference(document)
        expected = sc.extract_sprite_with_offset(export, textures, frame_index=frame)
        case = args.output / document / export / f"frame_{frame:03d}"
        case.parent.mkdir(parents=True, exist_ok=True)
        output = case.with_suffix(".rust.png")
        metadata = case.with_suffix(".json")
        subprocess.run([str(args.binary.resolve()), "--assets", str(args.assets.resolve()), "--document", document,
                        "--export", export, "--frame", str(frame), "--render", str(output), "--metadata", str(metadata)], check=True, capture_output=True)
        report = json.loads(metadata.read_text())
        row = {"document": document, "export": export, "frame": frame, "original_sc_sha256": report["original_sc_sha256"], "stats": report["render"]["stats"] if report["render"] else None}
        if expected is None:
            row["matches"] = report["render"] is None
            row["empty"] = True
        else:
            image, x, y = expected
            image.save(case.with_suffix(".python.png"))
            a = np.array(image)
            b = np.array(Image.open(output).convert("RGBA"))
            row["python_size"] = image.size
            row["rust_size"] = [b.shape[1], b.shape[0]]
            row["origin_matches"] = report["render"] is not None and [x, y] == [report["render"]["origin_x"], report["render"]["origin_y"]]
            row["matches"] = a.shape == b.shape and bool(np.array_equal(a, b)) and row["origin_matches"]
            if a.shape == b.shape:
                difference = np.abs(a.astype(np.int16) - b.astype(np.int16))
                row["maximum_channel_difference"] = int(difference.max())
                row["mean_channel_difference"] = float(difference.mean())
                row["identical_pixel_percent"] = float(100 * np.all(a == b, axis=-1).mean())
        results.append(row)
    summary = {"kind": "Rust port vs independent Python original-asset parser/rasterizer; not original-client parity",
               "rust_binary_sha256": hashlib.sha256(args.binary.read_bytes()).hexdigest(),
               "cases": len(results), "exact_matches": sum(r["matches"] for r in results), "results": results}
    (args.output / "comparison.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps({k: summary[k] for k in ["kind", "cases", "exact_matches"]}, indent=2))
    if summary["exact_matches"] != summary["cases"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
