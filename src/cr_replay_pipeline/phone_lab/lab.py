"""Entry point for the dual-phone browser lab."""

from __future__ import annotations

import time
import webbrowser
from pathlib import Path

from .adb_phone import (
    DEFAULT_PIXEL8,
    DEFAULT_PIXEL9,
    AdbPhone,
    adb_available,
    resolve_pair,
)
from .hand_detect import DEFAULT_YOLO
from .server import build_lab_state, serve_lab

DEFAULT_CALIB_DIR = Path("data/phone_lab/calibrations")


def run_phone_lab(
    *,
    host: str = "127.0.0.1",
    port: int = 8766,
    pixel9: str = DEFAULT_PIXEL9,
    pixel8: str = DEFAULT_PIXEL8,
    calib_dir: str | Path = DEFAULT_CALIB_DIR,
    yolo_model: str | Path = DEFAULT_YOLO,
    card_costs: str | Path = "data/card_costs.json",
    policy_v3: str | Path = "models/policy_bc_v3",
    policy_v4: str | Path = "models/policy_bc_v4",
    policy_v41: str | Path = "models/policy_bc_v4.1",
    policy_v42: str | Path = "models/policy_bc_v4.2_full",
    mirror_tta: bool = False,
    think_steps: int = 0,
    open_browser: bool = True,
) -> dict[str, object]:
    if not adb_available():
        raise RuntimeError("adb not found on PATH — install android-tools-adb")

    d9, d8 = resolve_pair(pixel9, pixel8)
    phone9 = AdbPhone(d9.serial, label=d9.model or "Pixel 9")
    phone8 = AdbPhone(d8.serial, label=d8.model or "Pixel 8")

    state = build_lab_state(
        pixel9=phone9,
        pixel8=phone8,
        calib_dir=Path(calib_dir),
        yolo_model=Path(yolo_model),
        card_costs_path=Path(card_costs),
        policy_v3=Path(policy_v3),
        policy_v4=Path(policy_v4),
        policy_v41=Path(policy_v41),
        policy_v42=Path(policy_v42),
        mirror_tta=mirror_tta,
        think_steps=think_steps,
    )
    # Warm detector once so the first live detect is not a cold GPU start.
    state.detector.warm_up()

    httpd = serve_lab(state, host=host, port=port)
    url = f"http://{host}:{port}/"

    def _say(msg: str) -> None:
        print(msg, flush=True)

    _say("phone-lab ready")
    _say(f"  pixel9: {phone9.serial} ({phone9.label}) {phone9.width}x{phone9.height}")
    _say(f"  pixel8: {phone8.serial} ({phone8.label}) {phone8.width}x{phone8.height}")
    _say(f"  yolo:   {state.detector.status}")
    _say("  stream: scrcpy-server h264 → websocket → WebCodecs (+ touch)")
    _say(f"  open:   {url}")
    _say(f"  mirror: {'enabled' if mirror_tta else 'disabled'}")
    _say(f"  think:  {think_steps} step(s)" if think_steps else "  think:  off")

    if open_browser:
        webbrowser.open(url)

    try:
        while True:
            time.sleep(1.0)
    except KeyboardInterrupt:
        _say("\nstopping phone-lab")
    finally:
        state.stop()
        httpd.shutdown()
        httpd.server_close()

    return {
        "url": url,
        "pixel9": phone9.info_dict(),
        "pixel8": phone8.info_dict(),
        "detector": state.detector.status,
    }
