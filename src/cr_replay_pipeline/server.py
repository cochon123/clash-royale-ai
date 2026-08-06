from __future__ import annotations

import hashlib
import json
import re
import threading
from dataclasses import dataclass, field
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

from .frontier import Frontier, replay_participants

SAFE_NAME = re.compile(r"[^A-Za-z0-9_-]+")
MAX_BODY = 5 * 1024 * 1024


def _battle_id(record: dict) -> str:
    url = record.get("request_url") or ""
    match = re.search(r"[?&]tag=([^&]+)", url)
    raw = match.group(1) if match else ""
    safe = SAFE_NAME.sub("", raw)
    if safe:
        return safe
    digest = hashlib.sha256(
        json.dumps(record, sort_keys=True, ensure_ascii=False).encode("utf-8")
    ).hexdigest()
    return digest[:24]


def _classification(record: dict) -> str:
    payload = record.get("payload")
    if not isinstance(payload, dict):
        return "invalid_payload"
    if payload.get("success") is True and isinstance(payload.get("html"), str):
        return "replay"
    if payload.get("status") == 429 or payload.get("error_code") == 1015:
        return "rate_limited"
    return "upstream_error"


def _atomic_write(target: Path, record: dict) -> None:
    encoded = json.dumps(record, ensure_ascii=False, separators=(",", ":"))
    temporary = target.with_suffix(f"{target.suffix}.tmp")
    temporary.write_text(encoded, encoding="utf-8")
    temporary.replace(target)


@dataclass
class IngestStore:
    raw_dir: Path
    quarantine_dir: Path | None = None
    accepted: int = 0
    duplicates: int = 0
    rejected: int = 0
    rate_limited: int = 0
    _lock: threading.Lock = field(default_factory=threading.Lock, repr=False)

    def __post_init__(self) -> None:
        self.raw_dir = Path(self.raw_dir)
        if self.quarantine_dir is None:
            self.quarantine_dir = (
                self.raw_dir.parent / "quarantine" / "capture-errors"
            )
        else:
            self.quarantine_dir = Path(self.quarantine_dir)

    def store(self, record: dict) -> dict:
        record.setdefault(
            "captured_at", datetime.now(timezone.utc).isoformat(timespec="seconds")
        )
        record.setdefault("schema_version", 1)
        battle_id = _battle_id(record)
        classification = _classification(record)

        with self._lock:
            if classification != "replay":
                self.rejected += 1
                if classification == "rate_limited":
                    self.rate_limited += 1
                self.quarantine_dir.mkdir(parents=True, exist_ok=True)
                target = self.quarantine_dir / f"{battle_id}.json"
                if not target.exists():
                    _atomic_write(target, record)
                return {
                    "ok": True,
                    "stored": False,
                    "classification": classification,
                    "file": target.name,
                }

            self.raw_dir.mkdir(parents=True, exist_ok=True)
            target = self.raw_dir / f"{battle_id}.json"
            if target.exists():
                try:
                    existing = json.loads(target.read_text(encoding="utf-8"))
                except (OSError, json.JSONDecodeError):
                    existing = None
                if isinstance(existing, dict) and _classification(existing) == "replay":
                    self.duplicates += 1
                    return {
                        "ok": True,
                        "stored": False,
                        "classification": "duplicate",
                        "file": target.name,
                    }

            _atomic_write(target, record)
            self.accepted += 1
            return {
                "ok": True,
                "stored": True,
                "classification": "replay",
                "file": target.name,
            }

    def health(self) -> dict:
        with self._lock:
            return {
                "ok": True,
                "accepted": self.accepted,
                "duplicates": self.duplicates,
                "rejected": self.rejected,
                "rate_limited": self.rate_limited,
                "raw_files": sum(1 for _ in self.raw_dir.glob("*.json")),
            }


def make_handler(raw_dir: Path, frontier: Frontier | None = None):
    store = IngestStore(raw_dir)

    class Handler(BaseHTTPRequestHandler):
        def _json(self, payload: dict, status: int = 200):
            encoded = json.dumps(payload).encode("utf-8")
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(encoded)))
            self.send_header("Access-Control-Allow-Origin", "*")
            self.send_header("Access-Control-Allow-Headers", "Content-Type")
            self.send_header("Access-Control-Allow-Methods", "GET,POST,OPTIONS")
            self.end_headers()
            self.wfile.write(encoded)

        def do_OPTIONS(self):
            self._json({}, 204)

        def do_GET(self):
            if self.path not in ("/health", "/status"):
                self._json({"ok": False}, 404)
                return
            result = store.health()
            if frontier is not None:
                result["collector"] = frontier.status()
            self._json(result)

        def _body(self) -> dict | None:
            length = int(self.headers.get("Content-Length") or 0)
            if length <= 0 or length > MAX_BODY:
                return None
            try:
                value = json.loads(self.rfile.read(length))
            except (UnicodeDecodeError, json.JSONDecodeError):
                return None
            return value if isinstance(value, dict) else None

        def do_POST(self):
            body = self._body()
            if body is None:
                self._json({"ok": False, "error": "invalid_body"}, 400)
                return

            if self.path == "/replays":
                result = store.store(body)
                if frontier is not None:
                    classification = result["classification"]
                    if classification == "rate_limited":
                        payload = body.get("payload") or {}
                        frontier.rate_result(
                            rate_limited=True,
                            retry_after=float(payload.get("retry_after") or 30),
                        )
                    elif classification in ("replay", "duplicate"):
                        frontier.rate_result(rate_limited=False)
                        source, participants = replay_participants(body)
                        result["players_discovered"] = frontier.discover(
                            source, participants
                        )
                self._json(result)
                return

            if frontier is None:
                self._json({"ok": False, "error": "frontier_disabled"}, 503)
                return

            if self.path == "/players/seed":
                tags = body.get("tags")
                if not isinstance(tags, list):
                    self._json({"ok": False, "error": "tags_must_be_a_list"}, 400)
                    return
                added = frontier.seed(tags, source=str(body.get("source") or "browser"))
                self._json({"ok": True, "added": added, **frontier.status()["players"]})
                return

            if self.path == "/jobs/claim":
                worker_id = str(body.get("worker_id") or "browser")
                job = frontier.claim(worker_id)
                self._json({"ok": True, "job": job, "paused": frontier.status()["paused"]})
                return

            if self.path == "/jobs/complete":
                tag = str(body.get("tag") or "")
                if not tag:
                    self._json({"ok": False, "error": "missing_tag"}, 400)
                    return
                status = frontier.complete(
                    tag,
                    retry=bool(body.get("retry")),
                    error=str(body.get("error") or "") or None,
                )
                self._json({"ok": True, "status": status})
                return

            if self.path == "/rate/lease":
                self._json(frontier.rate_lease(str(body.get("worker_id") or "browser")))
                return

            if self.path in ("/control", "/problem"):
                action = str(body.get("action") or ("pause" if self.path == "/problem" else ""))
                if action == "pause":
                    frontier.pause(str(body.get("reason") or "browser requires attention"))
                elif action == "resume":
                    frontier.resume()
                else:
                    self._json({"ok": False, "error": "unknown_action"}, 400)
                    return
                self._json({"ok": True, **frontier.status()})
                return

            if self.path != "/replays":
                self._json({"ok": False}, 404)
                return

        def log_message(self, format, *args):
            return

    return Handler


def serve(
    raw_dir: str | Path,
    host: str = "127.0.0.1",
    port: int = 8765,
    db_path: str | Path = "data/collector.sqlite3",
):
    target = Path(raw_dir)
    frontier = Frontier(db_path)
    bootstrap = frontier.bootstrap(target)
    server = ThreadingHTTPServer((host, port), make_handler(target, frontier))
    print(
        f"Listening on http://{host}:{port}; writing replays to {target}; "
        f"frontier={db_path}; bootstrap={bootstrap}",
        flush=True,
    )
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
