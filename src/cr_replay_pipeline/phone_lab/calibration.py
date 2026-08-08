"""Load and scale unified phone calibrations.

Placement taps always go through clicked ``placement_points`` (matplotlib
``phone-lab-calibrate``). Named presets are the six landmarks; arbitrary
board positions use normalized (u, v) mapped through those landmarks.
"""

from __future__ import annotations

import json
from copy import deepcopy
from pathlib import Path
from typing import Any

DEFAULT_CALIB_DIR = Path("data/phone_lab/calibrations")

# Six landmarks from phone-lab-calibrate, in click order conceptually.
REQUIRED_PLACEMENT_KEYS = (
    "bridge_left",
    "bridge_right",
    "bottom_left",
    "bottom_right",
    "enemy_left",
    "enemy_right",
)

# Named presets → normalized arena (u, v):
# u: 0 = left edge, 1 = right edge
# v: 0 = enemy back line, 0.5 = bridge line, 1 = own back line
PRESET_UV: dict[str, tuple[float, float]] = {
    "enemy_left": (0.0, 0.0),
    "enemy_right": (1.0, 0.0),
    "bridge_left": (0.0, 0.5),
    "bridge_right": (1.0, 0.5),
    "bottom_left": (0.0, 1.0),
    "bottom_right": (1.0, 1.0),
}

# Maps UI / TEST names → keys stored in placement_points.
PRESET_ALIASES = {
    "my_corner_left": "bottom_left",
    "my_corner_right": "bottom_right",
    "enemy_corner_left": "enemy_left",
    "enemy_corner_right": "enemy_right",
}


def _as_rect(zone: dict[str, Any]) -> dict[str, int]:
    return {
        "x_min": int(zone["x_min"]),
        "y_min": int(zone["y_min"]),
        "x_max": int(zone["x_max"]),
        "y_max": int(zone["y_max"]),
    }


def rect_center(rect: dict[str, int]) -> tuple[int, int]:
    return (
        (rect["x_min"] + rect["x_max"]) // 2,
        (rect["y_min"] + rect["y_max"]) // 2,
    )


def scale_rect(
    rect: dict[str, int],
    *,
    src_w: int,
    src_h: int,
    dst_w: int,
    dst_h: int,
) -> dict[str, int]:
    sx = dst_w / max(1, src_w)
    sy = dst_h / max(1, src_h)
    return {
        "x_min": int(round(rect["x_min"] * sx)),
        "y_min": int(round(rect["y_min"] * sy)),
        "x_max": int(round(rect["x_max"] * sx)),
        "y_max": int(round(rect["y_max"] * sy)),
    }


def scale_point(
    point: dict[str, Any],
    *,
    src_w: int,
    src_h: int,
    dst_w: int,
    dst_h: int,
) -> dict[str, int]:
    sx = dst_w / max(1, src_w)
    sy = dst_h / max(1, src_h)
    return {
        "x": int(round(int(point["x"]) * sx)),
        "y": int(round(int(point["y"]) * sy)),
    }


def load_raw_calibration(
    serial: str,
    calib_dir: Path | str = DEFAULT_CALIB_DIR,
) -> dict[str, Any]:
    calib_dir = Path(calib_dir)
    candidates = [
        calib_dir / f"{serial}_unified.json",
        calib_dir / "shared_unified.json",
    ]
    for path in candidates:
        if path.is_file():
            data = json.loads(path.read_text(encoding="utf-8"))
            data["_source_path"] = str(path)
            return data
    raise FileNotFoundError(
        f"No calibration for {serial} under {calib_dir} "
        f"(tried {[p.name for p in candidates]})"
    )


def load_scaled_calibration(
    serial: str,
    *,
    width: int,
    height: int,
    calib_dir: Path | str = DEFAULT_CALIB_DIR,
) -> dict[str, Any]:
    raw = load_raw_calibration(serial, calib_dir)
    src_w = int(raw.get("screen_width") or width)
    src_h = int(raw.get("screen_height") or height)
    zones_in = raw.get("zones") or {}
    zones: dict[str, dict[str, int]] = {}
    for name, zone in zones_in.items():
        rect = _as_rect(zone)
        if src_w != width or src_h != height:
            rect = scale_rect(
                rect, src_w=src_w, src_h=src_h, dst_w=width, dst_h=height
            )
        zones[name] = rect

    required = [f"card_slot_{i}" for i in range(4)] + ["blue_arena"]
    missing = [k for k in required if k not in zones]
    if missing:
        raise ValueError(f"Calibration missing zones: {missing}")

    points_in = raw.get("placement_points") or {}
    points: dict[str, dict[str, int]] = {}
    for name, point in points_in.items():
        if src_w != width or src_h != height:
            points[name] = scale_point(
                point, src_w=src_w, src_h=src_h, dst_w=width, dst_h=height
            )
        else:
            points[name] = {"x": int(point["x"]), "y": int(point["y"])}

    out = deepcopy(raw)
    out["device_serial"] = serial
    out["screen_width"] = width
    out["screen_height"] = height
    out["zones"] = zones
    out["placement_points"] = points
    out["scaled_from"] = {"width": src_w, "height": src_h}
    return out


def card_slot_rects(calibration: dict[str, Any]) -> dict[str, dict[str, int]]:
    zones = calibration["zones"]
    return {f"card_slot_{i}": zones[f"card_slot_{i}"] for i in range(4)}


def card_slot_rects_for_size(
    calibration: dict[str, Any],
    *,
    width: int,
    height: int,
) -> list[dict[str, int]]:
    """Scale device-space card slots into a stream / canvas resolution."""
    src_w = int(calibration["screen_width"])
    src_h = int(calibration["screen_height"])
    out: list[dict[str, int]] = []
    for i in range(4):
        rect = calibration["zones"][f"card_slot_{i}"]
        if width == src_w and height == src_h:
            out.append(_as_rect(rect))
        else:
            out.append(
                scale_rect(
                    rect, src_w=src_w, src_h=src_h, dst_w=width, dst_h=height
                )
            )
    return out


def normalize_preset(preset: str) -> str:
    return PRESET_ALIASES.get(preset, preset)


def _xy(point: dict[str, Any]) -> tuple[float, float]:
    return float(point["x"]), float(point["y"])


def _lerp(a: tuple[float, float], b: tuple[float, float], t: float) -> tuple[float, float]:
    return (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t)


def require_placement_points(calibration: dict[str, Any]) -> dict[str, dict[str, int]]:
    """Return clicked landmarks or raise — placements must not use arena fractions."""
    points = calibration.get("placement_points") or {}
    missing = [k for k in REQUIRED_PLACEMENT_KEYS if k not in points]
    if missing:
        serial = calibration.get("device_serial") or "?"
        raise RuntimeError(
            f"Placement calibration incomplete for {serial}; missing {missing}. "
            f"Run: cr-replays phone-lab-calibrate --phone <pixel9|pixel8>"
        )
    return points  # type: ignore[return-value]


def arena_uv_to_pixel(
    calibration: dict[str, Any],
    u: float,
    v: float,
) -> tuple[int, int]:
    """Map normalized arena (u, v) → screen pixels via the 6 clicked landmarks.

    u: 0 left … 1 right
    v: 0 enemy back … 0.5 bridge … 1 own back
    """
    points = require_placement_points(calibration)
    u = max(0.0, min(1.0, float(u)))
    v = max(0.0, min(1.0, float(v)))

    enemy_l = _xy(points["enemy_left"])
    enemy_r = _xy(points["enemy_right"])
    bridge_l = _xy(points["bridge_left"])
    bridge_r = _xy(points["bridge_right"])
    own_l = _xy(points["bottom_left"])
    own_r = _xy(points["bottom_right"])

    if v <= 0.5:
        t = v / 0.5
        left = _lerp(enemy_l, bridge_l, t)
        right = _lerp(enemy_r, bridge_r, t)
    else:
        t = (v - 0.5) / 0.5
        left = _lerp(bridge_l, own_l, t)
        right = _lerp(bridge_r, own_r, t)

    x, y = _lerp(left, right, u)
    return int(round(x)), int(round(y))


def placement_pixel(
    calibration: dict[str, Any],
    preset: str,
) -> tuple[int, int]:
    """Resolve a named placement preset to pixels via clicked calibration only."""
    preset = normalize_preset(preset)
    points = require_placement_points(calibration)
    if preset in points:
        p = points[preset]
        return int(p["x"]), int(p["y"])
    if preset in PRESET_UV:
        return arena_uv_to_pixel(calibration, *PRESET_UV[preset])
    raise KeyError(
        f"Unknown placement preset {preset!r}; "
        f"choose from {sorted(PRESET_UV)} or pass arena (u, v)"
    )


def resolve_placement(
    calibration: dict[str, Any],
    *,
    preset: str | None = None,
    u: float | None = None,
    v: float | None = None,
) -> tuple[int, int]:
    """Single entry point for every place-card tap on a phone."""
    if u is not None and v is not None:
        return arena_uv_to_pixel(calibration, u, v)
    if preset:
        return placement_pixel(calibration, preset)
    raise ValueError("placement requires preset=... or u=... and v=...")


def public_calibration(calibration: dict[str, Any]) -> dict[str, Any]:
    points = calibration.get("placement_points") or {}
    ready = all(k in points for k in REQUIRED_PLACEMENT_KEYS)
    presets: dict[str, Any] = {}
    if ready:
        for name in PRESET_UV:
            presets[name] = {
                "pixel": list(placement_pixel(calibration, name)),
                "uv": list(PRESET_UV[name]),
                "source": "clicked",
            }
    return {
        "device_serial": calibration.get("device_serial"),
        "screen_width": calibration.get("screen_width"),
        "screen_height": calibration.get("screen_height"),
        "scaled_from": calibration.get("scaled_from"),
        "source_path": calibration.get("_source_path"),
        "zones": calibration.get("zones"),
        "placement_points": points,
        "placement_ready": ready,
        "presets": presets,
    }
