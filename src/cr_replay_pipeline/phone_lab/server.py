"""Local HTTP lab: dual low-latency H.264 WebSocket streams + calibration APIs."""

from __future__ import annotations

import base64
import io
import json
import struct
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any
from urllib.parse import urlparse

from PIL import Image

from .adb_phone import AdbPhone
from .battle import (
    DEFAULT_CONTROLLERS,
    DEFAULT_POLICY_DIRS,
    BattleConfig,
    BattleRunner,
    cards_payload,
)
from .calibration import (
    card_slot_rects,
    card_slot_rects_for_size,
    load_scaled_calibration,
    resolve_placement,
    public_calibration,
    rect_center,
)
from .hand_detect import HandDetector
from .stream_source import (
    ACTION_DOWN,
    ACTION_MOVE,
    ACTION_UP,
    PacketHub,
    ScrcpyStream,
    VideoPacket,
)
from . import websocket_util as ws

_TOUCH_ACTIONS = {
    "down": ACTION_DOWN,
    "move": ACTION_MOVE,
    "up": ACTION_UP,
    "0": ACTION_DOWN,
    "1": ACTION_UP,
    "2": ACTION_MOVE,
}

UI_PATH = Path(__file__).with_name("ui.html")

# Binary frame: [u8 flags][payload]
FLAG_CONFIG = 0x01
FLAG_KEY = 0x02


class LabState:
    def __init__(
        self,
        phones: dict[str, AdbPhone],
        calibrations: dict[str, dict[str, Any]],
        detector: HandDetector,
        streams: dict[str, ScrcpyStream],
        *,
        card_costs_path: Path = Path("data/card_costs.json"),
        policy_dirs: dict[str, Path] | None = None,
        mirror_tta: bool = False,
        think_steps: int | None = None,
    ):
        self.phones = phones
        self.calibrations = calibrations
        self.detector = detector
        self.streams = streams
        self.lock = threading.Lock()
        self.card_costs_path = Path(card_costs_path)
        self.mirror_tta = bool(mirror_tta)
        self.think_steps = None if think_steps is None else int(think_steps)
        self.policy_dirs = {
            k: Path(v) for k, v in (policy_dirs or DEFAULT_POLICY_DIRS).items()
        }
        self._detected_hands: dict[str, list[dict[str, Any]]] = {}
        self._detected_hands_at: dict[str, float] = {}
        self.battle = BattleRunner(
            detect_hand=self._battle_detect_hand,
            execute_action=self._battle_execute,
        )

    def start(self) -> None:
        # Stagger USB/scrcpy startups — launching both at once often stalls one phone.
        for i, stream in enumerate(self.streams.values()):
            if i:
                time.sleep(0.7)
            stream.start()

    def stop(self) -> None:
        self.battle.stop()
        for stream in self.streams.values():
            stream.stop()

    def _battle_detect_hand(self, key: str) -> list[dict[str, Any]]:
        # The browser decodes the already-running scrcpy stream and posts four
        # tiny slot crops. Give that low-latency feed a moment to publish a frame;
        # retain full ADB screencap as a headless fallback.
        deadline = time.monotonic() + 0.45
        while time.monotonic() < deadline:
            with self.lock:
                slots = self._detected_hands.get(key)
                age = time.monotonic() - self._detected_hands_at.get(key, 0.0)
                if slots and age <= 1.10:
                    return [dict(row) for row in slots]
            time.sleep(0.025)
        return self.detect_phone(key)["slots"]

    def _battle_execute(
        self, phone_key: str, slot: int, u: float, v: float
    ) -> dict[str, Any]:
        result = self.test_action(phone_key, slot, u=u, v=v)
        # Require a crop captured after the tap for deployment confirmation.
        with self.lock:
            self._detected_hands_at[phone_key] = 0.0
        return result

    def _cache_detected_hand(self, key: str, slots: list[dict[str, Any]]) -> None:
        with self.lock:
            self._detected_hands[key] = [dict(row) for row in slots]
            self._detected_hands_at[key] = time.monotonic()

    def capture_png(self, key: str) -> bytes:
        return self.phones[key].screencap_png_fast()

    def stream_slots(self, key: str) -> list[dict[str, int]] | None:
        stream = self.streams[key]
        if stream.width <= 0 or stream.height <= 0:
            return None
        return card_slot_rects_for_size(
            self.calibrations[key],
            width=stream.width,
            height=stream.height,
        )

    @staticmethod
    def _public_slots(slots: list[dict[str, Any]]) -> list[dict[str, Any]]:
        public_slots = []
        for slot in slots:
            crop = slot.get("crop_jpeg") or b""
            public_slots.append(
                {
                    "slot": slot["slot"],
                    "card_name": slot["card_name"],
                    "confidence": slot["confidence"],
                    "error": slot.get("error"),
                    "rect": slot.get("rect"),
                    "crop_data_url": (
                        "data:image/jpeg;base64,"
                        + base64.b64encode(crop).decode("ascii")
                        if crop
                        else ""
                    ),
                }
            )
        return public_slots

    @staticmethod
    def _assume_unknown_musketeer_pixel8(
        key: str, slots: list[dict[str, Any]]
    ) -> list[dict[str, Any]]:
        """Compatibility hook that deliberately does not invent card labels.

        The previous Pixel 8 special case turned any single unreadable crop
        into Musketeer. That could make the policy tap a different real card.
        Unknown visual state must remain unknown until the detector reads it.
        """
        del key
        return slots

    def detect_phone(self, key: str, *, png: bytes | None = None) -> dict[str, Any]:
        t0 = time.perf_counter()
        png = png or self.capture_png(key)
        slots = self.detector.detect(png, card_slot_rects(self.calibrations[key]))
        slots = self._assume_unknown_musketeer_pixel8(key, slots)
        public_slots = self._public_slots(slots)
        self._cache_detected_hand(key, public_slots)
        ms = int((time.perf_counter() - t0) * 1000)
        return {
            "phone": key,
            "slots": public_slots,
            "ms": ms,
            "source": "screencap",
        }

    def detect_phone_crops(
        self, key: str, crop_data_urls: list[str]
    ) -> dict[str, Any]:
        if key not in self.phones:
            raise KeyError(f"unknown phone {key}")
        if len(crop_data_urls) != 4:
            raise ValueError("crops must be a list of 4 data URLs")
        images: list[Image.Image] = []
        for raw in crop_data_urls:
            if "," in raw:
                raw = raw.split(",", 1)[1]
            blob = base64.b64decode(raw)
            images.append(Image.open(io.BytesIO(blob)).convert("RGB"))
        stream_rects = self.stream_slots(key)
        rects = stream_rects if stream_rects else [None] * 4
        t0 = time.perf_counter()
        slots = self.detector.detect_crop_images(images, rects=rects)
        slots = self._assume_unknown_musketeer_pixel8(key, slots)
        public_slots = self._public_slots(slots)
        self._cache_detected_hand(key, public_slots)
        ms = int((time.perf_counter() - t0) * 1000)
        return {
            "phone": key,
            "slots": public_slots,
            "ms": ms,
            "source": "stream-crops",
        }

    def test_action(
        self,
        phone_key: str,
        slot: int,
        placement: str | None = None,
        *,
        u: float | None = None,
        v: float | None = None,
    ) -> dict[str, Any]:
        if phone_key not in self.phones:
            raise KeyError(f"unknown phone {phone_key}")
        if slot not in (0, 1, 2, 3):
            raise ValueError("slot must be 0..3")
        phone = self.phones[phone_key]
        calib = self.calibrations[phone_key]
        card_rect = calib["zones"][f"card_slot_{slot}"]
        card_xy = rect_center(card_rect)
        # Every place tap goes through clicked placement_points (or uv map).
        place_xy = resolve_placement(
            calib,
            preset=placement,
            u=u,
            v=v,
        )
        stream = self.streams[phone_key]
        transport = "adb"
        with self.lock:
            if stream.control_ready and stream.width > 0 and stream.height > 0:
                transport = "scrcpy"

                def stream_tap(point: tuple[int, int]) -> None:
                    x = round(point[0] * stream.width / max(phone.width, 1))
                    y = round(point[1] * stream.height / max(phone.height, 1))
                    stream.inject_touch(ACTION_DOWN, x, y, pressure=1.0)
                    stream.inject_touch(ACTION_UP, x, y, pressure=0.0)

                stream_tap(card_xy)
                time.sleep(0.12)
                stream_tap(place_xy)
            else:
                phone.tap(*card_xy)
                time.sleep(0.12)
                phone.tap(*place_xy)
        # Hand refresh is done client-side from the live frame (~1500ms later).
        return {
            "phone": phone_key,
            "slot": slot,
            "placement": placement,
            "u": u,
            "v": v,
            "card_tap": list(card_xy),
            "place_tap": list(place_xy),
            "transport": transport,
        }


def _json_response(handler: BaseHTTPRequestHandler, code: int, payload: dict) -> None:
    body = json.dumps(payload).encode("utf-8")
    handler.send_response(code)
    handler.send_header("Content-Type", "application/json")
    handler.send_header("Content-Length", str(len(body)))
    handler.send_header("Cache-Control", "no-store")
    handler.end_headers()
    handler.wfile.write(body)


def _pack_video(packet: VideoPacket) -> bytes:
    flags = 0
    if packet.is_config:
        flags |= FLAG_CONFIG
    if packet.is_keyframe:
        flags |= FLAG_KEY
    return struct.pack("B", flags) + packet.data


def make_handler(state: LabState):
    class Handler(BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"

        def log_message(self, fmt: str, *args) -> None:
            if self.path.startswith("/ws/"):
                return
            super().log_message(fmt, *args)

        def do_GET(self) -> None:  # noqa: N802
            path = urlparse(self.path).path
            if path in {"/", "/index.html"}:
                html = UI_PATH.read_bytes()
                self.send_response(200)
                self.send_header("Content-Type", "text/html; charset=utf-8")
                self.send_header("Content-Length", str(len(html)))
                self.end_headers()
                self.wfile.write(html)
                return
            if path == "/api/status":
                _json_response(
                    self,
                    200,
                    {
                        "phones": {
                            k: {
                                **v.info_dict(),
                                "stream": state.streams[k].stats(),
                                "stream_slots": state.stream_slots(k),
                            }
                            for k, v in state.phones.items()
                        },
                        "calibrations": {
                            k: public_calibration(v)
                            for k, v in state.calibrations.items()
                        },
                        "detector": state.detector.status,
                        "transport": "scrcpy-h264-websocket",
                        "battle": state.battle.status(),
                        "default_controllers": dict(DEFAULT_CONTROLLERS),
                        "mirror_tta": state.mirror_tta,
                        "think_steps": state.think_steps,
                    },
                )
                return
            if path == "/api/cards":
                _json_response(self, 200, cards_payload(state.card_costs_path))
                return
            if path == "/api/battle/status":
                _json_response(self, 200, state.battle.status())
                return
            if path.startswith("/ws/"):
                key = path.rsplit("/", 1)[-1]
                if key not in state.streams:
                    self.send_error(404, "unknown phone")
                    return
                self._video_ws(key)
                return
            self.send_error(404, "not found")

        def do_POST(self) -> None:  # noqa: N802
            path = urlparse(self.path).path
            length = int(self.headers.get("Content-Length") or 0)
            raw = self.rfile.read(length) if length else b"{}"
            try:
                payload = json.loads(raw.decode("utf-8") or "{}")
            except json.JSONDecodeError:
                _json_response(self, 400, {"error": "invalid json"})
                return
            try:
                if path == "/api/detect":
                    phone = payload.get("phone")
                    if phone:
                        out = {phone: state.detect_phone(phone)}
                    else:
                        out = {
                            "pixel9": state.detect_phone("pixel9"),
                            "pixel8": state.detect_phone("pixel8"),
                        }
                    _json_response(self, 200, out)
                    return
                if path == "/api/detect-crops":
                    phone = str(payload.get("phone") or "pixel9")
                    crops = payload.get("crops") or []
                    if not isinstance(crops, list):
                        raise ValueError("crops must be a list")
                    out = state.detect_phone_crops(phone, [str(c) for c in crops])
                    _json_response(self, 200, out)
                    return
                if path == "/api/test-action":
                    u = payload.get("u")
                    v = payload.get("v")
                    result = state.test_action(
                        str(payload.get("phone") or "pixel9"),
                        int(payload.get("slot", 0)),
                        (
                            None
                            if u is not None and v is not None
                            else str(payload.get("placement") or "bottom_left")
                        ),
                        u=float(u) if u is not None else None,
                        v=float(v) if v is not None else None,
                    )
                    _json_response(self, 200, result)
                    return
                if path == "/api/tap":
                    # Raw debug tap only — card placement must use /api/test-action
                    # (resolve_placement → clicked landmarks).
                    key = str(payload.get("phone") or "pixel9")
                    x, y = int(payload["x"]), int(payload["y"])
                    state.phones[key].tap(x, y)
                    _json_response(
                        self, 200, {"ok": True, "phone": key, "x": x, "y": y}
                    )
                    return
                if path == "/api/battle/start":
                    pixel8 = payload.get("pixel8") or {}
                    pixel9 = payload.get("pixel9") or {}
                    timeout = float(payload.get("timeout_s") or 300.0)
                    cfg = BattleConfig(
                        decks={
                            "pixel8": list(pixel8.get("deck") or []),
                            "pixel9": list(pixel9.get("deck") or []),
                        },
                        controllers={
                            "pixel8": str(
                                pixel8.get("controller")
                                or DEFAULT_CONTROLLERS["pixel8"]
                            ),
                            "pixel9": str(
                                pixel9.get("controller")
                                or DEFAULT_CONTROLLERS["pixel9"]
                            ),
                        },
                        timeout_s=timeout,
                        card_costs_path=state.card_costs_path,
                        policy_dirs=state.policy_dirs,
                        mirror_tta=state.mirror_tta,
                        think_steps=state.think_steps,
                    )
                    out = state.battle.start(cfg)
                    _json_response(self, 200, out)
                    return
                if path == "/api/battle/stop":
                    state.battle.stop()
                    _json_response(self, 200, state.battle.status())
                    return
            except Exception as exc:  # noqa: BLE001
                _json_response(self, 500, {"error": str(exc)})
                return
            _json_response(self, 404, {"error": "not found"})

        def _video_ws(self, key: str) -> None:
            if not ws.handshake(self):
                return
            stream = state.streams[key]
            hub = PacketHub(maxsize=12)

            def on_packet(packet: VideoPacket) -> None:
                hub.push(packet)

            # Serialize socket writes (reader thread answers pings).
            write_lock = threading.Lock()
            sid = stream.subscribe(on_packet)
            try:
                # Wait briefly for codec/size from an active session.
                deadline = time.time() + 4.0
                while time.time() < deadline and (stream.width <= 0 or stream.height <= 0):
                    if stream.stats().get("error"):
                        break
                    time.sleep(0.05)
                hello = {
                    "type": "hello",
                    "phone": key,
                    "codec": stream.codec,
                    "width": stream.width,
                    "height": stream.height,
                    "serial": stream.serial,
                    "control": stream.control_ready,
                }
                with write_lock:
                    ws.send_text(self.wfile, json.dumps(hello))
                closed = threading.Event()

                def _handle_client_text(raw: str) -> None:
                    try:
                        msg = json.loads(raw)
                    except json.JSONDecodeError:
                        return
                    kind = msg.get("type")
                    if kind == "touch":
                        action = _TOUCH_ACTIONS.get(str(msg.get("action", "")).lower())
                        if action is None:
                            return
                        stream.inject_touch(
                            action,
                            int(msg["x"]),
                            int(msg["y"]),
                            pressure=msg.get("pressure"),
                        )
                    elif kind == "back":
                        stream.inject_back(ACTION_DOWN)

                def _reader() -> None:
                    try:
                        while not closed.is_set():
                            frame = ws.read_frame(self.rfile)
                            if frame is None:
                                break
                            opcode, payload = frame
                            if opcode == ws.OP_CLOSE:
                                break
                            if opcode == ws.OP_PING:
                                with write_lock:
                                    ws.send_pong(self.wfile, payload)
                            elif opcode == ws.OP_TEXT:
                                try:
                                    _handle_client_text(payload.decode("utf-8"))
                                except Exception:
                                    pass
                    except Exception:
                        pass
                    finally:
                        closed.set()
                        hub.close()

                threading.Thread(
                    target=_reader, name=f"ws-reader-{key}", daemon=True
                ).start()

                while not closed.is_set():
                    packet = hub.pop(timeout=1.0)
                    if packet is None:
                        if closed.is_set():
                            break
                        continue
                    try:
                        with write_lock:
                            ws.send_binary(self.wfile, _pack_video(packet))
                    except (BrokenPipeError, ConnectionResetError, OSError):
                        break
            finally:
                stream.unsubscribe(sid)
                hub.close()
                try:
                    with write_lock:
                        ws.send_close(self.wfile)
                except Exception:
                    pass

    return Handler


def build_lab_state(
    *,
    pixel9: AdbPhone,
    pixel8: AdbPhone,
    calib_dir: Path,
    yolo_model: Path,
    card_costs_path: Path = Path("data/card_costs.json"),
    policy_v3: Path = Path("models/policy_bc_v3"),
    policy_v4: Path = Path("models/policy_bc_v4"),
    policy_v41: Path = Path("models/policy_bc_v4.1"),
    policy_v42: Path = Path("models/policy_bc_v4.2_full"),
    policy_v43: Path = Path("models/policy_bc_v4.3"),
    policy_v44: Path = Path("models/policy_bc_v4.4"),
    policy_v441: Path = Path("models/policy_bc_v4.4.1"),
    mirror_tta: bool = False,
    think_steps: int | None = None,
) -> LabState:
    phones = {"pixel9": pixel9, "pixel8": pixel8}
    calibrations = {
        key: load_scaled_calibration(
            phone.serial,
            width=phone.width,
            height=phone.height,
            calib_dir=calib_dir,
        )
        for key, phone in phones.items()
    }
    streams = {
        key: ScrcpyStream(
            phone.serial,
            max_size=960,
            bit_rate=8_000_000,
            max_fps=60,
        )
        for key, phone in phones.items()
    }
    detector = HandDetector(yolo_model)
    return LabState(
        phones,
        calibrations,
        detector,
        streams,
        card_costs_path=card_costs_path,
        policy_dirs={
            "policy_bc_v3": Path(policy_v3),
            "policy_bc_v4": Path(policy_v4),
            "policy_bc_v4.1": Path(policy_v41),
            "policy_bc_v4.2": Path(policy_v42),
            "policy_bc_v4.3": Path(policy_v43),
            "policy_bc_v4.4": Path(policy_v44),
            "policy_bc_v4.4.1": Path(policy_v441),
        },
        mirror_tta=mirror_tta,
        think_steps=think_steps,
    )


def serve_lab(
    state: LabState,
    *,
    host: str = "127.0.0.1",
    port: int = 8766,
) -> ThreadingHTTPServer:
    state.start()
    handler = make_handler(state)
    httpd = ThreadingHTTPServer((host, port), handler)
    thread = threading.Thread(target=httpd.serve_forever, daemon=True)
    thread.start()
    return httpd
