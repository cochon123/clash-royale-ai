"""Lightweight visual elixir-bar measurement for the portrait battle UI."""

from __future__ import annotations

from typing import Any

import numpy as np
from PIL import Image

# The purple bar is stable across both phones and resolutions.  The left edge
# deliberately excludes the elixir-drop icon; the right edge ends inside the
# rounded bar outline.
ELIXIR_BAR_RECT = (0.270, 0.969, 0.974, 0.991)


def elixir_bar_rect(width: int, height: int) -> tuple[int, int, int, int]:
    x0, y0, x1, y1 = ELIXIR_BAR_RECT
    return (
        round(width * x0),
        round(height * y0),
        round(width * x1),
        round(height * y1),
    )


def crop_elixir_bar(image: Image.Image) -> Image.Image:
    return image.crop(elixir_bar_rect(*image.size))


def estimate_elixir_bar(crop: Image.Image) -> dict[str, Any]:
    """Estimate continuous elixir from a tight crop of the purple fill bar.

    Returns confidence zero when the battle bar is not visible (loading/result
    screens).  A low estimate is intentionally conservative: delaying a play is
    much safer for dataset integrity than issuing an unaffordable action.
    """
    rgb = np.asarray(crop.convert("RGB"), dtype=np.float32)
    if rgb.ndim != 3 or rgb.shape[0] < 4 or rgb.shape[1] < 20:
        return {"value": None, "confidence": 0.0, "reason": "bad_crop"}

    red, green, blue = rgb[..., 0], rgb[..., 1], rgb[..., 2]
    purple = (red > 125) & (blue > 115) & (red > green * 1.30) & (blue > green * 1.30)
    purple_by_column = purple.mean(axis=0)
    filled_columns = purple_by_column > 0.10

    # A visible empty bar is dark blue; this distinguishes zero elixir from a
    # result/loading screen where the entire control is absent.
    blue_bar = (blue > 42) & (blue > red * 1.08) & (blue > green * 1.03)
    bar_visibility = float(blue_bar.mean())

    if not bool(filled_columns.any()):
        if bar_visibility < 0.12:
            return {"value": None, "confidence": 0.0, "reason": "bar_not_visible"}
        return {
            "value": 0.0,
            "confidence": round(min(0.9, 0.55 + bar_visibility * 0.35), 4),
            "reason": "empty_bar",
        }

    edge = int(np.flatnonzero(filled_columns)[-1])
    value = 10.0 * (edge + 1) / max(1, rgb.shape[1])
    continuity = float(filled_columns[: edge + 1].mean())
    confidence = min(1.0, 0.55 + 0.45 * continuity)
    return {
        "value": round(max(0.0, min(10.0, value)), 3),
        "confidence": round(confidence, 4),
        "reason": "purple_fill",
        "continuity": round(continuity, 4),
    }
