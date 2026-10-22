#!/usr/bin/env python3
"""Read-only local dashboard server for banana-hog-2 training telemetry."""
from __future__ import annotations

import argparse
import json
import os
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, urlsplit

ROOT = Path(__file__).resolve().parents[1]
HTML_PATH = Path(__file__).with_name("hog2_dashboard.html")
MAX_STATE_BYTES = 8 * 1024 * 1024
MAX_MATCH_LINE_BYTES = 256 * 1024
MAX_MATCH_RESPONSE = 500


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds").replace("+00:00", "Z")


def read_state(run_dir: Path) -> tuple[dict, str | None]:
    path = run_dir / "state.json"
    try:
        if path.stat().st_size > MAX_STATE_BYTES:
            return {}, "state.json exceeds the 8 MiB safety limit"
        with path.open("r", encoding="utf-8") as f:
            value = json.load(f)
        if not isinstance(value, dict):
            return {}, "state.json must contain a JSON object"
        # Optional run manifest supplements the state without changing the
        # existing state.json schema. The path is fixed under --run-dir.
        manifest_path = run_dir / "manifest.json"
        if manifest_path.is_file():
            try:
                if manifest_path.stat().st_size <= 2 * 1024 * 1024:
                    with manifest_path.open("r", encoding="utf-8") as f:
                        manifest = json.load(f)
                    if isinstance(manifest, dict):
                        embedded = value.get("manifest")
                        if isinstance(embedded, dict):
                            manifest.update(embedded)
                        value["manifest"] = manifest
            except (OSError, UnicodeError, json.JSONDecodeError):
                pass
        operator_path = run_dir / "operator-notes.json"
        if operator_path.is_file():
            try:
                if operator_path.stat().st_size <= 16 * 1024:
                    with operator_path.open("r", encoding="utf-8") as f:
                        operator_data = json.load(f)
                    if isinstance(operator_data, dict) and isinstance(operator_data.get("notes"), list):
                        existing = value.get("operator_notes", [])
                        if not isinstance(existing, list):
                            existing = []
                        value["operator_notes"] = existing + [
                            note for note in operator_data["notes"] if isinstance(note, str)
                        ]
            except (OSError, UnicodeError, json.JSONDecodeError):
                pass
        value["server_now"] = utc_now()
        return value, None
    except FileNotFoundError:
        return {}, "Waiting for state.json"
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        return {}, f"Cannot read state.json: {type(exc).__name__}"


def read_matches(run_dir: Path, after: str | None, limit: int) -> list[dict]:
    """Read only the fixed matches.jsonl file; never follow client-supplied paths."""
    path = run_dir / "matches.jsonl"
    if not path.is_file():
        return []
    result = []
    try:
        with path.open("rb") as f:
            # Tail bounded file reads; avoid loading an unbounded match log.
            f.seek(0, os.SEEK_END)
            end = f.tell()
            start = max(0, end - 4 * 1024 * 1024)
            f.seek(start)
            data = f.read()
        lines = data.splitlines()
        if start and lines:
            lines = lines[1:]
        for raw in lines[-MAX_MATCH_RESPONSE * 3:]:
            if len(raw) > MAX_MATCH_LINE_BYTES:
                continue
            try:
                row = json.loads(raw)
            except (json.JSONDecodeError, UnicodeDecodeError):
                continue
            if not isinstance(row, dict):
                continue
            ident = str(row.get("id", ""))
            if after is not None and ident <= after:
                continue
            result.append(row)
        return result[-limit:]
    except OSError:
        return []


class Handler(BaseHTTPRequestHandler):
    server_version = "TrainingDashboard/1.1"
    sys_version = ""

    @property
    def run_dir(self) -> Path:
        return self.server.run_dir  # type: ignore[attr-defined]

    def send_bytes(self, body: bytes, content_type: str, status: int = 200) -> None:
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.send_header("X-Content-Type-Options", "nosniff")
        self.end_headers()
        self.wfile.write(body)

    def send_json(self, value: object, status: int = 200) -> None:
        self.send_bytes(json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode(), "application/json; charset=utf-8", status)

    def do_GET(self) -> None:
        parsed = urlsplit(self.path)
        path = parsed.path
        if path == "/" or path == "/index.html":
            try:
                self.send_bytes(HTML_PATH.read_bytes(), "text/html; charset=utf-8")
            except OSError:
                self.send_json({"error": "dashboard file unavailable"}, 500)
        elif path in ("/api/state", "/api/snapshot"):
            state, error = read_state(self.run_dir)
            if error:
                state["read_error"] = error
            self.send_json(state)
        elif path == "/api/health":
            state, error = read_state(self.run_dir)
            self.send_json({"ok": error is None, "state_file": (self.run_dir / "state.json").is_file(), "server_now": utc_now(), "status": state.get("status"), "error": error})
        elif path == "/api/matches":
            q = parse_qs(parsed.query)
            after = q.get("after", [None])[0]
            try:
                limit = max(1, min(MAX_MATCH_RESPONSE, int(q.get("limit", ["100"])[0])))
            except ValueError:
                limit = 100
            self.send_json({"matches": read_matches(self.run_dir, after, limit), "server_now": utc_now()})
        elif path == "/advice":
            run_advice = self.run_dir / "advice.md"
            if run_advice.is_file():
                try:
                    self.send_bytes(run_advice.read_bytes(), "text/markdown; charset=utf-8")
                except OSError:
                    self.send_json({"error": "advice unavailable"}, 404)
                return
            state, _ = read_state(self.run_dir)
            run_name = state.get("name")
            # The repository advice is specific to banana-hog-2; never present it
            # as advice for a different experiment.
            if run_name != "banana-hog-2":
                self.send_json({"error": "run-specific advice unavailable"}, 404)
                return
            source = ROOT / "analysis/hog2/advice/opus-5-5.json"
            try:
                with source.open(encoding="utf-8") as f:
                    data = json.load(f)
                self.send_bytes(str(data.get("result", "Advice unavailable")).encode("utf-8"),
                                "text/markdown; charset=utf-8")
            except (OSError, json.JSONDecodeError):
                self.send_json({"error": "advice unavailable"}, 404)
        else:
            self.send_json({"error": "not found"}, 404)

    def log_message(self, fmt: str, *args: object) -> None:
        # Keep logs local and avoid echoing request query strings or environment details.
        print(f"[{utc_now()}] {self.address_string()} {args[0] if args else ''}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", type=Path, required=True, help="Directory containing state.json and matches.jsonl")
    parser.add_argument("--host", default="127.0.0.1", help="Bind address (default: localhost)")
    parser.add_argument("--port", type=int, default=8782)
    args = parser.parse_args()
    run_dir = args.run_dir.expanduser().resolve()
    if not run_dir.is_dir():
        parser.error(f"run directory does not exist: {run_dir}")
    server = ThreadingHTTPServer((args.host, args.port), Handler)
    server.run_dir = run_dir  # type: ignore[attr-defined]
    print(f"banana-hog-2 dashboard at http://{args.host}:{args.port}/ (read-only; run dir: {run_dir})", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
