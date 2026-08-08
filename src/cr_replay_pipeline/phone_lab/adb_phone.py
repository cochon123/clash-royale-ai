"""Reliable ADB helpers for the dual-phone lab."""

from __future__ import annotations

import io
import re
import struct
import subprocess
import time
from dataclasses import dataclass

from PIL import Image


DEFAULT_PIXEL9 = "4B090DLAQ002ZT"
DEFAULT_PIXEL8 = "41060DLJH000KW"


@dataclass(frozen=True)
class DeviceInfo:
    serial: str
    state: str
    model: str = ""
    product: str = ""
    transport: str = ""

    @property
    def is_ready(self) -> bool:
        return self.state == "device"

    @property
    def is_waydroid(self) -> bool:
        return ":" in self.serial or "waydroid" in self.product.lower()


def _run(
    args: list[str],
    *,
    timeout: float = 12.0,
    check: bool = False,
) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        args,
        capture_output=True,
        timeout=timeout,
        check=check,
    )


def adb_available() -> bool:
    try:
        result = _run(["adb", "version"], timeout=5)
        return result.returncode == 0
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return False


def list_devices(*, include_offline: bool = False) -> list[DeviceInfo]:
    result = _run(["adb", "devices", "-l"], timeout=8)
    if result.returncode != 0:
        return []
    devices: list[DeviceInfo] = []
    for raw in result.stdout.decode("utf-8", errors="replace").splitlines()[1:]:
        line = raw.strip()
        if not line:
            continue
        parts = line.split()
        if len(parts) < 2:
            continue
        serial, state = parts[0], parts[1]
        meta = " ".join(parts[2:])
        model = _meta(meta, "model")
        product = _meta(meta, "product")
        transport = _meta(meta, "transport_id")
        info = DeviceInfo(
            serial=serial,
            state=state,
            model=model.replace("_", " "),
            product=product,
            transport=transport,
        )
        if include_offline or info.is_ready:
            devices.append(info)
    return devices


def _meta(blob: str, key: str) -> str:
    match = re.search(rf"{re.escape(key)}:(\S+)", blob)
    return match.group(1) if match else ""


def physical_devices() -> list[DeviceInfo]:
    return [d for d in list_devices() if d.is_ready and not d.is_waydroid]


def resolve_pair(
    pixel9: str = DEFAULT_PIXEL9,
    pixel8: str = DEFAULT_PIXEL8,
) -> tuple[DeviceInfo, DeviceInfo]:
    ready = {d.serial: d for d in physical_devices()}
    missing = [s for s in (pixel9, pixel8) if s not in ready]
    if missing:
        present = ", ".join(ready) if ready else "(none)"
        raise RuntimeError(
            "Need both phones connected over USB as ADB devices.\n"
            f"  missing: {', '.join(missing)}\n"
            f"  present: {present}\n"
            "  tip: adb devices -l"
        )
    return ready[pixel9], ready[pixel8]


class AdbPhone:
    """One physical phone with fast screencap and taps."""

    def __init__(self, serial: str, label: str = ""):
        self.serial = serial
        self.label = label or serial
        self.width, self.height = self._read_size()

    def _adb(self, *args: str, timeout: float = 12.0) -> subprocess.CompletedProcess[bytes]:
        return _run(["adb", "-s", self.serial, *args], timeout=timeout)

    def _read_size(self) -> tuple[int, int]:
        result = self._adb("shell", "wm", "size", timeout=6)
        text = result.stdout.decode("utf-8", errors="replace")
        # Prefer Override size if present, else Physical size.
        override = re.search(r"Override size:\s*(\d+)x(\d+)", text)
        physical = re.search(r"Physical size:\s*(\d+)x(\d+)", text)
        match = override or physical or re.search(r"(\d+)x(\d+)", text)
        if not match:
            return 1080, 2400
        return int(match.group(1)), int(match.group(2))

    def screencap_png(self, *, timeout: float = 8.0) -> bytes:
        """Return PNG bytes via exec-out (no sdcard pull)."""
        result = self._adb("exec-out", "screencap", "-p", timeout=timeout)
        if result.returncode != 0 or not result.stdout.startswith(b"\x89PNG"):
            err = result.stderr.decode("utf-8", errors="replace")[:200]
            raise RuntimeError(f"screencap failed on {self.serial}: {err or 'bad png'}")
        return result.stdout

    def screencap_image(self, *, timeout: float = 8.0) -> Image.Image:
        """Faster raw RGBA screencap → RGB PIL image (full resolution)."""
        result = self._adb("exec-out", "screencap", timeout=timeout)
        raw = result.stdout
        if result.returncode != 0 or len(raw) < 16:
            err = result.stderr.decode("utf-8", errors="replace")[:200]
            raise RuntimeError(f"raw screencap failed on {self.serial}: {err}")
        w, h, _fmt, _ = struct.unpack_from("<IIII", raw, 0)
        need = 16 + w * h * 4
        if len(raw) < need:
            raise RuntimeError(
                f"raw screencap truncated on {self.serial}: {len(raw)} < {need}"
            )
        rgba = Image.frombytes("RGBA", (w, h), raw[16:need])
        return rgba.convert("RGB")

    def screencap_png_fast(self, *, timeout: float = 8.0) -> bytes:
        """Full-res capture encoded as PNG for detectors that expect PNG bytes."""
        img = self.screencap_image(timeout=timeout)
        buf = io.BytesIO()
        img.save(buf, format="PNG", compress_level=1)
        return buf.getvalue()

    def tap(self, x: int, y: int) -> None:
        x = max(0, min(self.width - 1, int(x)))
        y = max(0, min(self.height - 1, int(y)))
        result = self._adb("shell", "input", "tap", str(x), str(y), timeout=6)
        if result.returncode != 0:
            err = result.stderr.decode("utf-8", errors="replace")
            raise RuntimeError(f"tap failed on {self.serial}: {err}")

    def swipe(
        self, x1: int, y1: int, x2: int, y2: int, duration_ms: int = 160
    ) -> None:
        result = self._adb(
            "shell",
            "input",
            "swipe",
            str(int(x1)),
            str(int(y1)),
            str(int(x2)),
            str(int(y2)),
            str(int(duration_ms)),
            timeout=8,
        )
        if result.returncode != 0:
            err = result.stderr.decode("utf-8", errors="replace")
            raise RuntimeError(f"swipe failed on {self.serial}: {err}")

    def info_dict(self) -> dict[str, object]:
        return {
            "serial": self.serial,
            "label": self.label,
            "width": self.width,
            "height": self.height,
        }


def wait_for_pair(
    pixel9: str,
    pixel8: str,
    *,
    timeout_s: float = 0.0,
    poll_s: float = 1.0,
) -> tuple[DeviceInfo, DeviceInfo]:
    """Resolve immediately, or poll until timeout_s elapses (0 = no wait)."""
    deadline = time.monotonic() + max(0.0, timeout_s)
    while True:
        try:
            return resolve_pair(pixel9, pixel8)
        except RuntimeError:
            if time.monotonic() >= deadline:
                raise
            time.sleep(poll_s)
