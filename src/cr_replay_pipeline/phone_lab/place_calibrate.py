"""Matplotlib click calibration for placement landmarks."""

from __future__ import annotations

import json
import os
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import numpy as np
from PIL import Image

from .adb_phone import (
    DEFAULT_PIXEL8,
    DEFAULT_PIXEL9,
    AdbPhone,
    physical_devices,
    resolve_pair,
)
from .calibration import DEFAULT_CALIB_DIR

_INTERACTIVE_BACKENDS = ("TkAgg", "QtAgg", "Qt5Agg", "GTK3Agg", "GTK4Agg")


def _require_interactive_pyplot():
    """Force a GUI matplotlib backend before pyplot is imported."""
    import matplotlib

    current = matplotlib.get_backend().lower()
    if current != "agg" and "inline" not in current:
        import matplotlib.pyplot as plt
        from matplotlib.patches import Circle

        return plt, Circle

    # Prefer explicit GUI backends over the default Agg (headless / CI).
    preferred = os.environ.get("MPLBACKEND", "").strip()
    candidates = ([preferred] if preferred else []) + list(_INTERACTIVE_BACKENDS)
    errors: list[str] = []
    for name in candidates:
        if not name or name.lower() == "agg":
            continue
        try:
            matplotlib.use(name, force=True)
            import matplotlib.pyplot as plt
            from matplotlib.patches import Circle

            # Touch a figure so import-time binding failures surface now.
            fig = plt.figure()
            plt.close(fig)
            return plt, Circle
        except Exception as exc:  # noqa: BLE001 — probe each backend
            errors.append(f"{name}: {exc}")

    detail = "\n".join(f"  - {e}" for e in errors) or "  (no candidates tried)"
    raise RuntimeError(
        "No interactive matplotlib backend available (got Agg). "
        "Install a GUI binding, e.g. system tk + Pillow with ImageTk, then retry.\n"
        f"Tried:\n{detail}\n"
        "Tip: `uv pip install --force-reinstall pillow` often fixes missing ImageTk."
    )

# Click order shown to the user.
PLACE_STEPS: list[tuple[str, str]] = [
    ("bridge_left", "BRIDGE LEFT — own side of the left bridge, far left edge"),
    ("bridge_right", "BRIDGE RIGHT — own side of the right bridge, far right edge"),
    ("bottom_left", "MY CORNER LEFT — extreme bottom-left of your playable floor"),
    ("bottom_right", "MY CORNER RIGHT — extreme bottom-right of your playable floor"),
    ("enemy_left", "ENEMY CORNER LEFT — extreme top-left of enemy playable floor"),
    ("enemy_right", "ENEMY CORNER RIGHT — extreme top-right of enemy playable floor"),
]

PHONE_ALIASES = {
    "pixel9": DEFAULT_PIXEL9,
    "p9": DEFAULT_PIXEL9,
    "9": DEFAULT_PIXEL9,
    "pixel8": DEFAULT_PIXEL8,
    "p8": DEFAULT_PIXEL8,
    "8": DEFAULT_PIXEL8,
}


def resolve_serial(phone: str) -> str:
    key = phone.strip().lower()
    if key in PHONE_ALIASES:
        return PHONE_ALIASES[key]
    # Allow raw serials; verify connected.
    ready = {d.serial for d in physical_devices()}
    if phone in ready:
        return phone
    raise RuntimeError(
        f"Unknown phone {phone!r}. Use pixel9 / pixel8 or a connected serial.\n"
        f"Connected: {', '.join(sorted(ready)) or '(none)'}"
    )


def _calib_path(serial: str, calib_dir: Path) -> Path:
    path = calib_dir / f"{serial}_unified.json"
    if path.is_file():
        return path
    shared = calib_dir / "shared_unified.json"
    if shared.is_file():
        return shared
    return path


def run_place_calibrate(
    *,
    phone: str = "pixel9",
    calib_dir: str | Path = DEFAULT_CALIB_DIR,
) -> dict[str, Any]:
    """Open a screenshot and collect 6 placement clicks; save into unified JSON."""
    plt, Circle = _require_interactive_pyplot()
    calib_dir = Path(calib_dir)
    calib_dir.mkdir(parents=True, exist_ok=True)
    serial = resolve_serial(phone)
    device = AdbPhone(serial)
    png = device.screencap_png_fast()
    image = Image.open(__import__("io").BytesIO(png)).convert("RGB")
    rgb = np.asarray(image)

    points: dict[str, dict[str, int]] = {}
    step = {"i": 0}
    artists: list[Any] = []

    fig, ax = plt.subplots(figsize=(7.2, 14))
    ax.imshow(rgb)
    ax.set_axis_off()
    title = ax.set_title(PLACE_STEPS[0][1], fontsize=11, pad=8)
    fig.suptitle(
        f"Placement calibrate · {serial} · click points in order · right-click = undo",
        fontsize=10,
    )
    plt.tight_layout()

    def redraw_title() -> None:
        i = step["i"]
        if i >= len(PLACE_STEPS):
            title.set_text("Done — close the window to save")
        else:
            title.set_text(f"[{i + 1}/{len(PLACE_STEPS)}] {PLACE_STEPS[i][1]}")
        fig.canvas.draw_idle()

    def on_click(event) -> None:
        if event.inaxes != ax or event.xdata is None or event.ydata is None:
            return
        if event.button == 3:  # right-click undo
            if step["i"] <= 0 or not artists:
                return
            last_key = PLACE_STEPS[step["i"] - 1][0]
            points.pop(last_key, None)
            step["i"] -= 1
            for art in artists.pop():
                art.remove()
            redraw_title()
            return
        if event.button != 1:
            return
        if step["i"] >= len(PLACE_STEPS):
            return
        key, _ = PLACE_STEPS[step["i"]]
        x, y = int(round(event.xdata)), int(round(event.ydata))
        points[key] = {"x": x, "y": y}
        colors = {
            "bridge_left": "#1f9d55",
            "bridge_right": "#1f9d55",
            "bottom_left": "#2563eb",
            "bottom_right": "#2563eb",
            "enemy_left": "#dc2626",
            "enemy_right": "#dc2626",
        }
        dot = Circle((x, y), radius=14, fill=False, ec=colors.get(key, "yellow"), lw=2)
        ax.add_patch(dot)
        label = ax.text(
            x + 18,
            y - 18,
            key,
            color=colors.get(key, "yellow"),
            fontsize=9,
            weight="bold",
            bbox={"facecolor": "black", "alpha": 0.45, "pad": 2},
        )
        artists.append((dot, label))
        step["i"] += 1
        redraw_title()
        fig.canvas.draw_idle()
        if step["i"] >= len(PLACE_STEPS):
            print("All 6 points set. Close the matplotlib window to save.", flush=True)

    cid = fig.canvas.mpl_connect("button_press_event", on_click)
    backend = plt.get_backend()
    print(
        f"Calibrating {serial} ({device.width}x{device.height}) · backend={backend}",
        flush=True,
    )
    for i, (key, prompt) in enumerate(PLACE_STEPS, 1):
        print(f"  {i}. {key}: {prompt}", flush=True)
    # block=True is required; Agg would no-op and leave points empty.
    plt.show(block=True)
    fig.canvas.mpl_disconnect(cid)

    if len(points) < len(PLACE_STEPS):
        missing = [k for k, _ in PLACE_STEPS if k not in points]
        raise RuntimeError(f"Calibration incomplete; missing: {missing}")

    path = _calib_path(serial, calib_dir)
    if path.is_file():
        data = json.loads(path.read_text(encoding="utf-8"))
    else:
        data = {
            "device_serial": serial,
            "screen_width": device.width,
            "screen_height": device.height,
            "zones": {},
        }

    data["device_serial"] = serial
    data["screen_width"] = device.width
    data["screen_height"] = device.height
    data["placement_points"] = points
    data["placement_calibrated_at"] = datetime.now(timezone.utc).strftime(
        "%Y-%m-%dT%H:%M:%SZ"
    )

    # Refresh arena boxes from the clicked corners when possible.
    zones = data.setdefault("zones", {})
    own = [points["bottom_left"], points["bottom_right"], points["bridge_left"], points["bridge_right"]]
    enemy = [points["enemy_left"], points["enemy_right"], points["bridge_left"], points["bridge_right"]]
    zones["blue_arena"] = {
        "x_min": min(p["x"] for p in own),
        "y_min": min(p["y"] for p in own),
        "x_max": max(p["x"] for p in own),
        "y_max": max(p["y"] for p in own),
    }
    zones["red_arena"] = {
        "x_min": min(p["x"] for p in enemy),
        "y_min": min(p["y"] for p in enemy),
        "x_max": max(p["x"] for p in enemy),
        "y_max": max(p["y"] for p in enemy),
    }

    out_path = calib_dir / f"{serial}_unified.json"
    out_path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
    print(f"Saved placement points → {out_path}", flush=True)
    for key, _ in PLACE_STEPS:
        p = points[key]
        print(f"  {key}: ({p['x']}, {p['y']})", flush=True)
    return {"path": str(out_path), "serial": serial, "placement_points": points}


def run_place_calibrate_both(
    *,
    calib_dir: str | Path = DEFAULT_CALIB_DIR,
    pixel9: str = DEFAULT_PIXEL9,
    pixel8: str = DEFAULT_PIXEL8,
) -> dict[str, Any]:
    resolve_pair(pixel9, pixel8)
    results = {}
    for label, serial in (("pixel9", pixel9), ("pixel8", pixel8)):
        print(f"\n=== {label} ({serial}) ===", flush=True)
        results[label] = run_place_calibrate(phone=serial, calib_dir=calib_dir)
    return results
