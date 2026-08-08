"""Minimal RFC6455 helpers for binary video fan-out (stdlib only)."""

from __future__ import annotations

import base64
import hashlib
import struct
from http.server import BaseHTTPRequestHandler


WS_GUID = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"
OP_CONTINUATION = 0x0
OP_TEXT = 0x1
OP_BINARY = 0x2
OP_CLOSE = 0x8
OP_PING = 0x9
OP_PONG = 0xA


def accept_key(sec_web_socket_key: str) -> str:
    digest = hashlib.sha1((sec_web_socket_key + WS_GUID).encode("ascii")).digest()
    return base64.b64encode(digest).decode("ascii")


def handshake(handler: BaseHTTPRequestHandler) -> bool:
    key = handler.headers.get("Sec-WebSocket-Key")
    if not key:
        handler.send_error(400, "missing Sec-WebSocket-Key")
        return False
    handler.send_response(101, "Switching Protocols")
    handler.send_header("Upgrade", "websocket")
    handler.send_header("Connection", "Upgrade")
    handler.send_header("Sec-WebSocket-Accept", accept_key(key))
    handler.end_headers()
    return True


def _write_frame(wfile, *, opcode: int, payload: bytes) -> None:
    length = len(payload)
    header = bytearray()
    header.append(0x80 | (opcode & 0x0F))
    if length < 126:
        header.append(length)
    elif length < (1 << 16):
        header.append(126)
        header.extend(struct.pack(">H", length))
    else:
        header.append(127)
        header.extend(struct.pack(">Q", length))
    wfile.write(header)
    wfile.write(payload)
    wfile.flush()


def send_text(wfile, text: str) -> None:
    _write_frame(wfile, opcode=OP_TEXT, payload=text.encode("utf-8"))


def send_binary(wfile, payload: bytes) -> None:
    _write_frame(wfile, opcode=OP_BINARY, payload=payload)


def send_pong(wfile, payload: bytes = b"") -> None:
    _write_frame(wfile, opcode=OP_PONG, payload=payload)


def send_close(wfile, code: int = 1000) -> None:
    _write_frame(wfile, opcode=OP_CLOSE, payload=struct.pack(">H", code))


def read_frame(rfile) -> tuple[int, bytes] | None:
    hdr = rfile.read(2)
    if not hdr or len(hdr) < 2:
        return None
    b0, b1 = hdr[0], hdr[1]
    opcode = b0 & 0x0F
    masked = bool(b1 & 0x80)
    length = b1 & 0x7F
    if length == 126:
        ext = rfile.read(2)
        if len(ext) < 2:
            return None
        length = struct.unpack(">H", ext)[0]
    elif length == 127:
        ext = rfile.read(8)
        if len(ext) < 8:
            return None
        length = struct.unpack(">Q", ext)[0]
    mask = b""
    if masked:
        mask = rfile.read(4)
        if len(mask) < 4:
            return None
    payload = rfile.read(length) if length else b""
    if length and len(payload) < length:
        return None
    if masked and payload:
        payload = bytes(b ^ mask[i % 4] for i, b in enumerate(payload))
    return opcode, payload
