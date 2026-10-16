"""Browser dashboard over a running trainer JSONL file."""

from __future__ import annotations

import json
import math
import threading
import time
import webbrowser
from collections import deque
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any
from urllib.parse import urlparse

from .report_kit import FONT_LINKS, favicon_link, kit_script, kit_styles, shared_styles
from .training_tui import LOG_LINES, HardwareSampler, _num, _pct

SKIP_CURVE_PHASES = frozenset({"loading", "validation", "log"})
SERIES_LIMIT = 1800
TEMPLATE_PATH = Path(__file__).resolve().parent / "assets" / "training_live.html"


def stride_indices(length: int, limit: int) -> list[int]:
    if length <= limit or limit < 2:
        return list(range(length))
    last = length - 1
    out: list[int] = []
    seen: set[int] = set()
    for i in range(limit):
        index = int(round(i * last / (limit - 1)))
        if index not in seen:
            seen.add(index)
            out.append(index)
    if out[0] != 0:
        out.insert(0, 0)
    if out[-1] != last:
        out.append(last)
    return out


def _finite(value: Any) -> float | None:
    if value is None:
        return None
    try:
        number = float(value)
    except (TypeError, ValueError):
        return None
    return number if math.isfinite(number) else None


def _clean(value: Any) -> Any:
    if isinstance(value, float) and not math.isfinite(value):
        return None
    if isinstance(value, dict):
        return {str(key): _clean(item) for key, item in value.items()}
    if isinstance(value, (list, tuple)):
        return [_clean(item) for item in value]
    return value


def _step(row: dict[str, Any], batches_total: int) -> int:
    epoch = int(row.get("epoch") or 0)
    batch = int(row.get("batch") or 0)
    total = int(row.get("batches_total") or batches_total or 1)
    return max((epoch - 1) * total + batch, 1)


def _tail_lines(path: Path, limit: int = LOG_LINES) -> list[str]:
    if not path.exists():
        return []
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return []
    lines = [line.rstrip() for line in text.splitlines() if line.strip()]
    return lines[-limit:]


class ProgressFeed:
    """Incremental reader. Never writes the JSONL file."""

    def __init__(self, progress_path: str | Path, log_path: str | Path | None = None) -> None:
        self.path = Path(progress_path)
        self.log_path = Path(log_path) if log_path else None
        self._lock = threading.Lock()
        self.offset = 0
        self.partial = ""
        self.n_raw = 0
        self.latest: dict[str, Any] = {}
        self.val: dict[str, Any] = {}
        self.events: deque[str] = deque(maxlen=LOG_LINES)
        self.steps: list[float] = []
        self.loss: list[float] = []
        self.card: list[float] = []
        self.tile_exact: list[float] = []
        self.tile_near: list[float] = []
        self.wait: list[float] = []
        self.batches_total = 1
        self._reset_series()

    def _reset_series(self) -> None:
        self.offset = 0
        self.partial = ""
        self.n_raw = 0
        self.latest = {}
        self.val = {}
        self.events.clear()
        self.steps.clear()
        self.loss.clear()
        self.card.clear()
        self.tile_exact.clear()
        self.tile_near.clear()
        self.wait.clear()
        self.batches_total = 1

    def poll(self) -> None:
        with self._lock:
            self._poll_unlocked()

    def _poll_unlocked(self) -> None:
        if not self.path.exists():
            if self.offset or self.steps:
                self._reset_series()
            return
        size = self.path.stat().st_size
        if size < self.offset:
            self._reset_series()
        with self.path.open("r", encoding="utf-8") as handle:
            handle.seek(self.offset)
            chunk = handle.read()
            self.offset = handle.tell()
        text = self.partial + chunk
        lines = text.split("\n")
        self.partial = lines[-1]
        for line in lines[:-1]:
            if not line.strip():
                continue
            try:
                row = json.loads(line)
            except json.JSONDecodeError:
                continue
            if isinstance(row, dict):
                self.ingest(row)

    def ingest(self, row: dict[str, Any]) -> None:
        self.n_raw += 1
        self.latest.update(row)
        if row.get("batches_total"):
            self.batches_total = max(int(row["batches_total"]), 1)
        phase = str(row.get("phase") or "")
        if phase == "log":
            message = row.get("message")
            if message:
                self.events.append(str(message))
            return
        if phase == "validation":
            self.val = dict(row)
            self.events.append(
                f"{_stamp(row)}  val  ep {row.get('epoch')}  "
                f"card={_pct(row.get('card') or row.get('acc'))}  "
                f"tile={_pct(row.get('tile_exact'))}  wait={_pct(row.get('wait'))}"
            )
            return
        if phase in SKIP_CURVE_PHASES or row.get("loss") is None:
            return
        loss = _finite(row.get("loss"))
        if loss is None:
            return
        self.steps.append(float(_step(row, self.batches_total)))
        self.loss.append(loss)
        self.card.append(_finite(row.get("card", row.get("acc"))) or 0.0)
        self.tile_exact.append(_finite(row.get("tile_exact")) or 0.0)
        self.tile_near.append(_finite(row.get("tile_near")) or 0.0)
        self.wait.append(_finite(row.get("wait")) or 0.0)

    def snapshot(self, *, hardware: dict[str, Any] | None = None) -> dict[str, Any]:
        with self._lock:
            latest = _clean(dict(self.latest))
            val = _clean(dict(self.val))
            n_raw = self.n_raw
            n_points = len(self.steps)
            idx = stride_indices(n_points, SERIES_LIMIT)
            series = {
                "step": [self.steps[i] for i in idx],
                "loss": [self.loss[i] for i in idx],
                "card": [self.card[i] for i in idx],
                "tile_exact": [self.tile_exact[i] for i in idx],
                "tile_near": [self.tile_near[i] for i in idx],
                "wait": [self.wait[i] for i in idx],
            }
            events = list(self.events)
            batches_total = self.batches_total
            exists = self.path.exists()
        run = _run_stats(latest, batches_total)
        log = events
        if self.log_path is not None:
            tailed = _tail_lines(self.log_path)
            if tailed:
                log = tailed
        age = _age_seconds(latest.get("timestamp"))
        live = None
        if latest.get("loss") is not None and str(latest.get("phase") or "") not in SKIP_CURVE_PHASES:
            live = (
                f"{datetime.now().strftime('%H:%M:%S')}  "
                f"ep {latest.get('epoch')}  {latest.get('batch')}/{run['batches_total']}  "
                f"obj={_num(latest.get('loss'))}  card={_pct(latest.get('card') or latest.get('acc'))}"
            )
        return {
            "ok": exists,
            "waiting": n_points == 0,
            "path": str(self.path),
            "age_seconds": age,
            "n_raw": n_raw,
            "n_points": n_points,
            "latest": latest,
            "val": val,
            "run": run,
            "series": series,
            "hardware": _clean(hardware or {}),
            "log": log,
            "live": live,
        }


def _stamp(row: dict[str, Any]) -> str:
    raw = str(row.get("timestamp") or "")
    if "T" in raw and len(raw) >= 19:
        return raw[11:19]
    return datetime.now().strftime("%H:%M:%S")


def _age_seconds(timestamp: Any) -> float | None:
    if not timestamp:
        return None
    text = str(timestamp).replace("Z", "+00:00")
    try:
        stamped = datetime.fromisoformat(text)
    except ValueError:
        return None
    if stamped.tzinfo is None:
        stamped = stamped.replace(tzinfo=timezone.utc)
    return max(0.0, time.time() - stamped.timestamp())


def _run_stats(latest: dict[str, Any], batches_total: int) -> dict[str, Any]:
    total = int(latest.get("batches_total") or batches_total or 1)
    epoch = int(latest.get("epoch") or 0)
    batch = int(latest.get("batch") or 0)
    elapsed = float(latest.get("elapsed_seconds") or 0.0)
    samples = float(latest.get("samples_done") or 0.0)
    work = max((epoch - 1) * total + batch, 0)
    return {
        "title": latest.get("model_name") or latest.get("phase") or "policy",
        "epoch": epoch,
        "epochs": int(latest.get("epochs_total") or 0),
        "batch": batch,
        "batches_total": total,
        "samples_done": int(samples),
        "samples_total": int(latest.get("samples_total") or 0),
        "eta_seconds": _finite(latest.get("eta_seconds")) or 0.0,
        "elapsed_seconds": elapsed,
        "progress_percent": _finite(latest.get("progress_percent")) or 0.0,
        "parameters": int(latest.get("parameters") or 0),
        "device": latest.get("device") or "cuda",
        "batch_size": int(latest.get("batch_size") or 0),
        "lr": _finite(latest.get("lr")),
        "its": work / max(elapsed, 1e-6),
        "samples_per_sec": samples / max(elapsed, 1e-6),
    }


def render_dashboard_html(*, interval: float) -> str:
    template = TEMPLATE_PATH.read_text(encoding="utf-8")
    return (
        template.replace("@@FAVICON@@", favicon_link())
        .replace("@@FONTS@@", FONT_LINKS)
        .replace("@@SHARED_STYLES@@", shared_styles() + kit_styles())
        .replace("@@KIT@@", kit_script())
        .replace("@@INTERVAL@@", str(int(max(interval, 0.5) * 1000)))
    )


def snapshot_progress(
    progress_path: str | Path,
    log_path: str | Path | None = None,
    hardware: dict[str, Any] | None = None,
) -> dict[str, Any]:
    feed = ProgressFeed(progress_path, log_path=log_path)
    feed.poll()
    return feed.snapshot(hardware=hardware)


def _guess_log_path(progress_path: Path, explicit: str | Path | None) -> Path | None:
    if explicit is not None:
        return Path(explicit)
    candidates = [
        Path("logs/policy_bc_v8.log"),
        progress_path.with_name("training.log"),
        Path("logs") / f"{progress_path.parent.name}.log",
    ]
    for candidate in candidates:
        if candidate.exists():
            return candidate
    return candidates[0]


class _DashboardHandler(BaseHTTPRequestHandler):
    feed: ProgressFeed
    sampler: HardwareSampler
    html: bytes
    interval: float

    def log_message(self, format: str, *args: Any) -> None:  # noqa: A003
        path = str(args[0]) if args else ""
        if "/api/state" in path:
            return
        super().log_message(format, *args)

    def do_GET(self) -> None:  # noqa: N802
        parsed = urlparse(self.path)
        if parsed.path in {"/", "/index.html"}:
            self._send(200, self.html, "text/html; charset=utf-8")
            return
        if parsed.path == "/api/state":
            self.feed.poll()
            payload = self.feed.snapshot(hardware=self.sampler.sample())
            body = json.dumps(payload, default=str).encode("utf-8")
            self._send(200, body, "application/json; charset=utf-8")
            return
        if parsed.path == "/health":
            self._send(200, b'{"ok":true}', "application/json; charset=utf-8")
            return
        self._send(404, b"not found", "text/plain; charset=utf-8")

    def _send(self, code: int, body: bytes, content_type: str) -> None:
        self.send_response(code)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)


def serve_training_dashboard(
    progress_path: str | Path,
    *,
    host: str = "127.0.0.1",
    port: int = 8766,
    interval: float = 2.0,
    log_path: str | Path | None = None,
    open_browser: bool = True,
) -> None:
    target = Path(progress_path)
    log_target = _guess_log_path(target, log_path)
    html = render_dashboard_html(interval=interval).encode("utf-8")
    feed = ProgressFeed(target, log_path=log_target)
    sampler = HardwareSampler()

    class Handler(_DashboardHandler):
        pass

    Handler.feed = feed
    Handler.sampler = sampler
    Handler.html = html
    Handler.interval = interval

    httpd = None
    bound = port
    last_error: OSError | None = None
    for candidate in range(port, port + 8):
        try:
            httpd = ThreadingHTTPServer((host, candidate), Handler)
            bound = candidate
            break
        except OSError as exc:
            last_error = exc
    if httpd is None:
        raise RuntimeError(f"could not bind {host}:{port}: {last_error}") from last_error

    url = f"http://{host}:{bound}"
    print(f"Astra live dashboard", flush=True)
    print(f"  open      {url}", flush=True)
    print(f"  progress  {target}", flush=True)
    if log_target is not None:
        print(f"  log       {log_target}", flush=True)
    print(f"  poll      {interval:.1f}s   (Ctrl+C to stop the viewer, not training)", flush=True)
    if open_browser:
        try:
            webbrowser.open(url)
        except Exception:
            pass
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print(flush=True)
    finally:
        httpd.server_close()
