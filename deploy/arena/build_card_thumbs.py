#!/usr/bin/env python3
"""Lobby card thumbnails: engine/web/cards/*.png -> webapps/arena/static/card-thumbs/*.webp.

The lobby shows card art about 80 CSS pixels wide. The originals are 960 px
PNGs of about 140 KB each, and the lobby drew up to 137 of them on every visit.
240 px covers a 3x phone screen.

    python deploy/arena/build_card_thumbs.py [--width 240] [--quality 82]
"""
from __future__ import annotations

import argparse
from pathlib import Path

from PIL import Image

REPO = Path(__file__).resolve().parents[2]
SOURCE = REPO / "engine" / "web" / "cards"
OUT = REPO / "webapps" / "arena" / "static" / "card-thumbs"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--width", type=int, default=240)
    parser.add_argument("--quality", type=int, default=82)
    args = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    before = after = 0
    for source in sorted(SOURCE.glob("*.png")):
        image = Image.open(source).convert("RGBA")
        height = round(image.height * args.width / image.width)
        thumb = image.resize((args.width, height), Image.LANCZOS)
        target = OUT / (source.stem + ".webp")
        thumb.save(target, "WEBP", quality=args.quality, method=6)
        before += source.stat().st_size
        after += target.stat().st_size
    print(f"{len(list(OUT.glob('*.webp')))} thumbnails, {before / 1e6:.1f} MB -> {after / 1e6:.2f} MB in {OUT.relative_to(REPO)}")


if __name__ == "__main__":
    main()
