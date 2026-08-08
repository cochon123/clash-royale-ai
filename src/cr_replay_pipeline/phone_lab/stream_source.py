"""Low-latency phone video via scrcpy-server H.264 socket (no ffmpeg/JPEG)."""

from __future__ import annotations

import os
import random
import socket
import struct
import subprocess
import threading
import time
from collections import deque
from dataclasses import dataclass
from pathlib import Path
from typing import Callable


CODEC_H264 = 0x68323634
PACKET_FLAG_CONFIG = 1 << 63
PACKET_FLAG_KEY = 1 << 62
DEVICE_NAME_LEN = 64

# scrcpy control messages (v3.3.x)
CTRL_INJECT_TOUCH = 2
CTRL_BACK_OR_SCREEN_ON = 4
ACTION_DOWN = 0
ACTION_UP = 1
ACTION_MOVE = 2
POINTER_ID_FINGER = (1 << 64) - 2  # uint64(-2)

DEFAULT_SERVER_PATHS = (
    Path("/usr/share/scrcpy/scrcpy-server"),
    Path("/usr/local/share/scrcpy/scrcpy-server"),
)


def pack_touch_event(
    action: int,
    x: int,
    y: int,
    video_w: int,
    video_h: int,
    *,
    pressure: float = 1.0,
) -> bytes:
    """Serialize SC_CONTROL_MSG_TYPE_INJECT_TOUCH_EVENT (32 bytes)."""
    buf = bytearray(32)
    buf[0] = CTRL_INJECT_TOUCH
    buf[1] = action & 0xFF
    struct.pack_into(">Q", buf, 2, POINTER_ID_FINGER)
    struct.pack_into(">ii", buf, 10, int(x), int(y))
    struct.pack_into(">HH", buf, 18, int(video_w) & 0xFFFF, int(video_h) & 0xFFFF)
    u16p = int(max(0.0, min(1.0, pressure)) * 0xFFFF)
    struct.pack_into(">H", buf, 22, u16p)
    struct.pack_into(">II", buf, 24, 0, 0)  # action_button, buttons
    return bytes(buf)


def pack_back_or_screen_on(action: int) -> bytes:
    return bytes((CTRL_BACK_OR_SCREEN_ON, action & 0xFF))


def find_scrcpy_server() -> Path:
    env = os.environ.get("SCRCPY_SERVER_PATH")
    if env:
        path = Path(env)
        if path.is_file():
            return path
    for path in DEFAULT_SERVER_PATHS:
        if path.is_file():
            return path
    raise FileNotFoundError(
        "scrcpy-server not found (set SCRCPY_SERVER_PATH or install scrcpy)"
    )


def scrcpy_version() -> str:
    result = subprocess.run(
        ["scrcpy", "--version"],
        capture_output=True,
        text=True,
        timeout=5,
        check=False,
    )
    line = (result.stdout or result.stderr or "").splitlines()[0]
    # "scrcpy 3.3.4 <https://...>"
    parts = line.split()
    if len(parts) >= 2 and parts[0].lower() == "scrcpy":
        return parts[1]
    raise RuntimeError(f"could not parse scrcpy version from: {line!r}")


def _adb(serial: str, *args: str, timeout: float = 12.0) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        ["adb", "-s", serial, *args],
        capture_output=True,
        timeout=timeout,
        check=False,
    )


def _read_exact(sock: socket.socket, n: int) -> bytes:
    buf = bytearray()
    while len(buf) < n:
        chunk = sock.recv(n - len(buf))
        if not chunk:
            raise EOFError(f"socket closed while reading {n} bytes")
        buf.extend(chunk)
    return bytes(buf)


@dataclass(frozen=True)
class VideoPacket:
    data: bytes
    is_config: bool
    is_keyframe: bool
    pts: int


Subscriber = Callable[[VideoPacket], None]


class ScrcpyStream:
    """Background scrcpy-server mirror; fans out H.264 access units to subscribers."""

    def __init__(
        self,
        serial: str,
        *,
        max_size: int = 960,
        bit_rate: int = 8_000_000,
        max_fps: int = 60,
        server_path: Path | None = None,
        version: str | None = None,
    ):
        self.serial = serial
        self.max_size = max_size
        self.bit_rate = bit_rate
        self.max_fps = max_fps
        self.server_path = Path(server_path) if server_path else find_scrcpy_server()
        self.version = version or scrcpy_version()

        self._lock = threading.Lock()
        self._stop = threading.Event()
        self._thread: threading.Thread | None = None
        self._shell: subprocess.Popen[bytes] | None = None
        self._sock: socket.socket | None = None
        self._control_sock: socket.socket | None = None
        self._control_lock = threading.Lock()
        self._control_reader: threading.Thread | None = None
        self._port: int | None = None
        self._scid_hex: str | None = None
        self._error: str | None = None
        self.control_ready = False

        self.width = 0
        self.height = 0
        self.codec = "h264"
        self._frame_id = 0
        self._last_ts = 0.0
        self._fps_ema = 0.0
        self._fps_window_t0 = 0.0
        self._fps_window_n = 0
        self._config: bytes | None = None
        self._subscribers: list[tuple[int, Subscriber]] = []
        self._sub_id = 0

    def subscribe(self, callback: Subscriber) -> int:
        with self._lock:
            self._sub_id += 1
            sid = self._sub_id
            self._subscribers.append((sid, callback))
            cfg = self._config
        # Late joiners miss the one-time SPS/PPS packet; replay it immediately.
        if cfg:
            callback(
                VideoPacket(
                    data=cfg, is_config=True, is_keyframe=False, pts=0
                )
            )
        return sid

    def unsubscribe(self, sid: int) -> None:
        with self._lock:
            self._subscribers = [(i, cb) for i, cb in self._subscribers if i != sid]

    def stats(self) -> dict[str, object]:
        with self._lock:
            age_ms = (
                int((time.time() - self._last_ts) * 1000) if self._last_ts else None
            )
            return {
                "serial": self.serial,
                "frames": self._frame_id,
                "fps": round(self._fps_ema, 1),
                "age_ms": age_ms,
                "width": self.width,
                "height": self.height,
                "codec": self.codec,
                "running": bool(self._thread and self._thread.is_alive()),
                "control": self.control_ready,
                "error": self._error,
                "transport": "scrcpy-h264",
            }

    def inject_touch(
        self,
        action: int,
        x: int,
        y: int,
        *,
        pressure: float | None = None,
    ) -> None:
        if not self.control_ready or self.width <= 0 or self.height <= 0:
            raise RuntimeError("control channel not ready")
        x = max(0, min(self.width - 1, int(x)))
        y = max(0, min(self.height - 1, int(y)))
        if pressure is None:
            pressure = 0.0 if action == ACTION_UP else 1.0
        self._control_send(
            pack_touch_event(
                action, x, y, self.width, self.height, pressure=pressure
            )
        )

    def inject_back(self, action: int = ACTION_DOWN) -> None:
        if not self.control_ready:
            raise RuntimeError("control channel not ready")
        self._control_send(pack_back_or_screen_on(action))
        if action == ACTION_DOWN:
            self._control_send(pack_back_or_screen_on(ACTION_UP))

    def _control_send(self, payload: bytes) -> None:
        sock = self._control_sock
        if sock is None:
            raise RuntimeError("control socket closed")
        with self._control_lock:
            sock.sendall(payload)

    def start(self) -> None:
        if self._thread and self._thread.is_alive():
            return
        self._stop.clear()
        self._thread = threading.Thread(
            target=self._run, name=f"scrcpy-h264-{self.serial}", daemon=True
        )
        self._thread.start()

    def stop(self) -> None:
        self._stop.set()
        self._close_io()
        if self._thread:
            self._thread.join(timeout=2.5)
        self._cleanup_device()

    def _close_io(self) -> None:
        self.control_ready = False
        for attr in ("_sock", "_control_sock"):
            sock = getattr(self, attr)
            setattr(self, attr, None)
            if sock is not None:
                try:
                    sock.shutdown(socket.SHUT_RDWR)
                except OSError:
                    pass
                try:
                    sock.close()
                except OSError:
                    pass
        shell = self._shell
        self._shell = None
        if shell is not None and shell.poll() is None:
            try:
                shell.terminate()
            except Exception:
                pass
            try:
                shell.wait(timeout=1.0)
            except Exception:
                try:
                    shell.kill()
                except Exception:
                    pass
        if self._port is not None:
            _adb(self.serial, "forward", "--remove", f"tcp:{self._port}")
            self._port = None

    def _cleanup_device(self) -> None:
        # Best-effort: stop orphaned servers for this serial.
        _adb(self.serial, "shell", "pkill", "-f", "com.genymobile.scrcpy.Server")

    def _run(self) -> None:
        while not self._stop.is_set():
            try:
                self._session()
            except Exception as exc:  # noqa: BLE001
                self._error = f"{type(exc).__name__}: {exc}"
            self._close_io()
            if self._stop.wait(0.5):
                break

    def _session(self) -> None:
        self._error = None
        scid = random.randint(1, 0x7FFFFFFF)
        self._scid_hex = f"{scid:08x}"
        socket_name = f"scrcpy_{self._scid_hex}"

        push = _adb(
            self.serial,
            "push",
            str(self.server_path),
            "/data/local/tmp/scrcpy-server.jar",
            timeout=20,
        )
        if push.returncode != 0:
            raise RuntimeError(
                push.stderr.decode("utf-8", "replace") or "adb push failed"
            )

        # Bind an ephemeral local port, then ask adb to forward it.
        probe = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        probe.bind(("127.0.0.1", 0))
        port = probe.getsockname()[1]
        probe.close()
        self._port = port

        fwd = _adb(
            self.serial,
            "forward",
            f"tcp:{port}",
            f"localabstract:{socket_name}",
        )
        if fwd.returncode != 0:
            raise RuntimeError(
                fwd.stderr.decode("utf-8", "replace") or "adb forward failed"
            )

        cmd = (
            f"CLASSPATH=/data/local/tmp/scrcpy-server.jar app_process / "
            f"com.genymobile.scrcpy.Server {self.version} "
            f"scid={self._scid_hex} log_level=info tunnel_forward=true "
            f"audio=false control=true cleanup=false "
            f"max_size={int(self.max_size)} max_fps={int(self.max_fps)} "
            f"video_bit_rate={int(self.bit_rate)} "
            # Frequent IDRs so browser clients can join without a long black screen.
            f"video_codec_options=i-frame-interval:int=1"
        )
        self._shell = subprocess.Popen(
            ["adb", "-s", self.serial, "shell", cmd],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            start_new_session=True,
        )

        if not self._wait_socket(socket_name, timeout=4.0):
            raise RuntimeError("scrcpy-server did not open video socket")

        # With control=true the server accepts video, then blocks until control
        # connects, then sends device meta.
        sock = socket.create_connection(("127.0.0.1", port), timeout=3.0)
        sock.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
        sock.settimeout(5.0)
        self._sock = sock
        _read_exact(sock, 1)  # dummy byte (forward tunnel)

        control = socket.create_connection(("127.0.0.1", port), timeout=3.0)
        control.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
        control.settimeout(2.0)
        self._control_sock = control
        self.control_ready = True
        self._control_reader = threading.Thread(
            target=self._drain_control,
            name=f"scrcpy-ctrl-{self.serial}",
            daemon=True,
        )
        self._control_reader.start()

        _ = _read_exact(sock, DEVICE_NAME_LEN)
        codec_id = struct.unpack(">I", _read_exact(sock, 4))[0]
        if codec_id != CODEC_H264:
            raise RuntimeError(f"unsupported codec id 0x{codec_id:08x}")
        width, height = struct.unpack(">II", _read_exact(sock, 8))
        with self._lock:
            self.width = width
            self.height = height
            self.codec = "h264"

        sock.settimeout(2.0)
        self._fps_window_t0 = time.perf_counter()
        self._fps_window_n = 0

        while not self._stop.is_set():
            try:
                header = _read_exact(sock, 12)
            except (TimeoutError, socket.timeout):
                if self._shell and self._shell.poll() is not None:
                    raise RuntimeError("scrcpy-server exited")
                continue
            pts_flags, size = struct.unpack(">QI", header)
            if size <= 0 or size > 8_000_000:
                raise RuntimeError(f"invalid packet size {size}")
            payload = _read_exact(sock, size)
            is_config = bool(pts_flags & PACKET_FLAG_CONFIG)
            is_key = bool(pts_flags & PACKET_FLAG_KEY)
            pts = pts_flags & (PACKET_FLAG_KEY - 1)
            packet = VideoPacket(
                data=payload,
                is_config=is_config,
                is_keyframe=is_key,
                pts=pts,
            )
            now = time.perf_counter()
            with self._lock:
                if is_config:
                    self._config = payload
                else:
                    self._frame_id += 1
                    self._last_ts = time.time()
                    self._fps_window_n += 1
                    elapsed = now - self._fps_window_t0
                    if elapsed >= 0.5:
                        self._fps_ema = self._fps_window_n / elapsed
                        self._fps_window_t0 = now
                        self._fps_window_n = 0
                subs = list(self._subscribers)
            for _, callback in subs:
                try:
                    callback(packet)
                except Exception:
                    pass

    def _drain_control(self) -> None:
        """Read device→client control messages so the socket never backs up."""
        sock = self._control_sock
        if sock is None:
            return
        try:
            while not self._stop.is_set():
                try:
                    chunk = sock.recv(4096)
                except (TimeoutError, socket.timeout):
                    continue
                if not chunk:
                    break
        except OSError:
            pass
        finally:
            self.control_ready = False

    def _wait_socket(self, socket_name: str, timeout: float) -> bool:
        deadline = time.time() + timeout
        needle = f"@{socket_name}"
        while time.time() < deadline and not self._stop.is_set():
            if self._shell and self._shell.poll() is not None:
                return False
            result = _adb(
                self.serial,
                "shell",
                f"grep -F {needle} /proc/net/unix",
                timeout=3.0,
            )
            if result.returncode == 0 and needle.encode() in result.stdout:
                return True
            time.sleep(0.08)
        return False


class PacketHub:
    """Per-viewer queue that drops to the next keyframe under backpressure."""

    def __init__(self, maxsize: int = 8):
        self._q: deque[VideoPacket] = deque()
        self._maxsize = maxsize
        self._lock = threading.Lock()
        self._cond = threading.Condition(self._lock)
        self._closed = False
        self._config: bytes | None = None
        self._need_key = True

    def close(self) -> None:
        with self._cond:
            self._closed = True
            self._cond.notify_all()

    def push(self, packet: VideoPacket) -> None:
        with self._cond:
            if self._closed:
                return
            if packet.is_config:
                self._config = packet.data
                # Replace any prior config still sitting at the front.
                if self._q and self._q[0].is_config:
                    self._q[0] = packet
                else:
                    self._q.appendleft(packet)
            else:
                if self._need_key and not packet.is_keyframe:
                    self._cond.notify()
                    return
                if packet.is_keyframe:
                    self._need_key = False
                    # A new IDR invalidates older queued media frames.
                    cfg = self._q[0] if self._q and self._q[0].is_config else None
                    self._q.clear()
                    if cfg is not None:
                        self._q.append(cfg)
                self._q.append(packet)
                # Count only media frames toward backpressure.
                media = sum(1 for p in self._q if not p.is_config)
                if media > self._maxsize:
                    cfg = self._q[0] if self._q and self._q[0].is_config else None
                    self._q.clear()
                    if cfg is not None:
                        self._q.append(cfg)
                    self._need_key = True
            self._cond.notify()

    def pop(self, timeout: float = 1.0) -> VideoPacket | None:
        with self._cond:
            ok = self._cond.wait_for(
                lambda: self._closed or bool(self._q), timeout=timeout
            )
            if not ok or self._closed:
                return None
            return self._q.popleft()
