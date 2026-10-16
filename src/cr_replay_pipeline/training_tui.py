"""Live training dashboard: four charts, run stats, log, and hardware bars."""

from __future__ import annotations

import json
import math
import os
import shutil
import subprocess
import time
from collections import deque
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable

try:
    from rich.console import Console, Group
    from rich.layout import Layout
    from rich.live import Live
    from rich.panel import Panel
    from rich.table import Table
    from rich.text import Text
    from rich import box

    HAS_RICH = True
except ImportError:  # pragma: no cover - optional visual dependency
    HAS_RICH = False
    Console = Live = object  # type: ignore[misc, assignment]


BLUE = "#638bc4"
GREEN = "#3dcc8a"
ORANGE = "#f29665"
VIOLET = "#9b7ed9"
MUTED = "#8aa3b3"
INK = "#d7e4ea"

WAIT_HIT_SECONDS = 1.5
LOG_LINES = 40


def _fmt_seconds(value: float) -> str:
    value = max(0.0, float(value))
    minutes, seconds = divmod(int(value), 60)
    hours, minutes = divmod(minutes, 60)
    if hours:
        return f"{hours}h{minutes:02d}m"
    return f"{minutes:02d}m{seconds:02d}s"


def _fmt_bytes(mebibytes: float) -> str:
    if mebibytes >= 1024:
        return f"{mebibytes / 1024:.1f}G"
    return f"{mebibytes:.0f}M"


def _pct(value: float | None) -> str:
    if value is None:
        return "—"
    return f"{100.0 * value:5.2f}%"


def _num(value: float | None, digits: int = 3) -> str:
    if value is None:
        return "—"
    return f"{value:.{digits}f}"


class HardwareSampler:
    """CPU / RAM / GPU snapshots, capped at about 1 Hz."""

    def __init__(self) -> None:
        self._cpu_idle = 0.0
        self._cpu_total = 0.0
        self._last_cpu = 0.0
        self._cached_gpu_name = "gpu"
        self._last_sample = 0.0
        self._cached: dict[str, float] | None = None
        self._gpu_name()
        self._cached = self._sample_now()
        self._last_sample = time.time()

    def _cpu_times(self) -> tuple[float, float]:
        try:
            with open("/proc/stat", encoding="utf-8") as handle:
                parts = handle.readline().split()
            values = [float(item) for item in parts[1:8]]
            idle = values[3] + (values[4] if len(values) > 4 else 0.0)
            return idle, sum(values)
        except OSError:
            load = os.getloadavg()[0]
            cores = os.cpu_count() or 1
            return 0.0, max(load / cores, 1e-6)

    def _ram(self) -> tuple[float, float]:
        try:
            info: dict[str, float] = {}
            with open("/proc/meminfo", encoding="utf-8") as handle:
                for line in handle:
                    key, raw, *_ = line.replace(":", " ").split()
                    info[key] = float(raw)
            total = info.get("MemTotal", 1.0) / 1024.0
            available = info.get("MemAvailable", info.get("MemFree", 0.0)) / 1024.0
            return max(total - available, 0.0), total
        except OSError:
            return 0.0, 1.0

    def _gpu_name(self) -> str:
        try:
            self._cached_gpu_name = subprocess.check_output(
                ("nvidia-smi", "--query-gpu=name", "--format=csv,noheader"),
                text=True,
                timeout=0.2,
            ).strip() or "gpu"
        except (OSError, subprocess.SubprocessError):
            self._cached_gpu_name = "gpu"
        return self._cached_gpu_name

    def _gpu(self) -> dict[str, float]:
        query = (
            "nvidia-smi",
            "--query-gpu=utilization.gpu,memory.used,memory.total,temperature.gpu",
            "--format=csv,noheader,nounits",
        )
        try:
            raw = subprocess.check_output(query, text=True, timeout=0.2)
            util, used, total, temp = [float(item.strip()) for item in raw.split(",")[:4]]
            return {
                "gpu_util": util / 100.0,
                "vram_used_mb": used,
                "vram_total_mb": max(total, 1.0),
                "gpu_temp": temp,
                "gpu_name": self._cached_gpu_name,
            }
        except (OSError, subprocess.SubprocessError, ValueError, IndexError):
            return {
                "gpu_util": 0.0,
                "vram_used_mb": 0.0,
                "vram_total_mb": 1.0,
                "gpu_temp": 0.0,
                "gpu_name": self._cached_gpu_name,
            }

    def _sample_now(self) -> dict[str, float]:
        idle, total = self._cpu_times()
        idle_d = idle - self._cpu_idle
        total_d = total - self._cpu_total
        if total_d > 0:
            self._last_cpu = 1.0 - idle_d / total_d
        self._cpu_idle, self._cpu_total = idle, total
        ram_used, ram_total = self._ram()
        gpu = self._gpu()
        return {
            "cpu": min(max(self._last_cpu, 0.0), 1.0),
            "ram_used_mb": ram_used,
            "ram_total_mb": ram_total,
            "ram": min(ram_used / max(ram_total, 1.0), 1.0),
            **gpu,
            "vram": min(gpu["vram_used_mb"] / max(gpu["vram_total_mb"], 1.0), 1.0),
        }

    def sample(self) -> dict[str, float]:
        now = time.time()
        if self._cached is not None and now - self._last_sample < 1.0:
            return self._cached
        self._cached = self._sample_now()
        self._last_sample = now
        return self._cached


def _bresenham(x0: int, y0: int, x1: int, y1: int) -> Iterable[tuple[int, int]]:
    dx = abs(x1 - x0)
    dy = abs(y1 - y0)
    sx = 1 if x0 < x1 else -1
    sy = 1 if y0 < y1 else -1
    err = dx - dy
    x, y = x0, y0
    while True:
        yield x, y
        if x == x1 and y == y1:
            return
        twice = err * 2
        if twice > -dy:
            err -= dy
            x += sx
        if twice < dx:
            err += dx
            y += sy


def _nice_ticks(lo: float, hi: float, count: int = 5) -> list[float]:
    if not math.isfinite(lo) or not math.isfinite(hi):
        return [0.0, 1.0]
    if hi <= lo:
        hi = lo + 1.0
    raw = (hi - lo) / max(count - 1, 1)
    magnitude = 10 ** math.floor(math.log10(raw or 1.0))
    nice = magnitude * 10
    for step in (1.0, 2.0, 2.5, 5.0, 10.0):
        candidate = step * magnitude
        if candidate >= raw * 0.75:
            nice = candidate
            break
    start = math.floor(lo / nice) * nice
    ticks: list[float] = []
    value = start
    for _ in range(24):
        if value >= lo - nice * 1e-6 and value <= hi + nice * 1e-6:
            ticks.append(value)
        value += nice
        if value > hi + nice:
            break
    return ticks or [lo, hi]


def _fmt_tick(value: float, kind: str, width: int = 6) -> str:
    if kind == "pct":
        text = f"{100.0 * value:5.0f}%"
    elif abs(value) >= 100:
        text = f"{value:.1f}"
    else:
        text = f"{value:.2f}"
    return text.rjust(width)[:width]


def _fmt_step(value: float) -> str:
    if value >= 1_000_000:
        return f"{value / 1_000_000:.1f}M"
    if value >= 10_000:
        return f"{value / 1000:.0f}k"
    if value >= 1000:
        return f"{value / 1000:.1f}k"
    return str(int(value))


def _downsample(values: list[float], n: int) -> list[float]:
    if n < 2 or len(values) <= n:
        return list(values)
    last = len(values) - 1
    return [values[int(round(i * last / (n - 1)))] for i in range(n)]


def _block_lines(
    series: list[tuple[list[float], str]],
    width: int,
    height: int,
    y_range: tuple[float, float],
    grid_rows: set[int] | None = None,
) -> list[Text]:
    """Connected polylines using 1-cell ▀▄█ glyphs so terminals do not wrap."""
    width = max(8, width)
    height = max(4, height)
    lo, hi = y_range
    span = (hi - lo) or 1.0
    pix_h = height * 2
    top: list[list[str | None]] = [[None] * width for _ in range(height)]
    bot: list[list[str | None]] = [[None] * width for _ in range(height)]
    for points, color in series:
        if not points:
            continue
        mapped: list[tuple[int, int]] = []
        last = max(len(points) - 1, 1)
        for index, value in enumerate(points):
            x = int(round(index * (width - 1) / last)) if len(points) > 1 else 0
            y = int(round((1.0 - (float(value) - lo) / span) * (pix_h - 1)))
            mapped.append((min(max(x, 0), width - 1), min(max(y, 0), pix_h - 1)))
        pairs = zip(mapped, mapped[1:]) if len(mapped) > 1 else ((mapped[0], mapped[0]),)
        for (x0, y0), (x1, y1) in pairs:
            for x, y in _bresenham(x0, y0, x1, y1):
                row, half = divmod(y, 2)
                if row >= height:
                    continue
                if half == 0:
                    top[row][x] = color
                else:
                    bot[row][x] = color
    grid = grid_rows or set()
    rows: list[Text] = []
    for row in range(height):
        line = Text()
        for col in range(width):
            upper, lower = top[row][col], bot[row][col]
            if upper and lower:
                line.append("█", style=upper)
            elif upper:
                line.append("▀", style=upper)
            elif lower:
                line.append("▄", style=lower)
            elif row in grid and col % 4 == 0:
                line.append("·", style=MUTED)
            else:
                line.append(" ")
        rows.append(line)
    return rows


def _axis_chart(
    series: list[tuple[list[float], str]],
    *,
    width: int,
    height: int,
    kind: str = "loss",
    x_values: list[float] | None = None,
    y_range: tuple[float, float] | None = None,
) -> Text:
    """Single-Y plot. No series names in the pane — ticks and color only."""
    width = max(24, width)
    height = max(8, height)
    left_w = 6
    plot_cols = max(10, width - left_w - 2)
    plot_rows = max(5, height - 2)

    prepared: list[tuple[list[float], str]] = []
    for values, color in series:
        points = _downsample([float(v) for v in values], plot_cols)
        prepared.append((points, color))

    if kind == "pct":
        lo, hi = 0.0, 1.0
    elif y_range is not None:
        lo, hi = y_range
    else:
        pooled = [v for points, _color in prepared for v in points]
        if len(pooled) < 2:
            lo, hi = 0.0, 1.0
        else:
            lo, hi = min(pooled), max(pooled)
            pad = ((hi - lo) or abs(hi) or 1.0) * 0.08
            lo, hi = lo - pad, hi + pad
    if hi <= lo:
        hi = lo + 1.0

    tick_vals = [0.0, 0.25, 0.5, 0.75, 1.0] if kind == "pct" else _nice_ticks(lo, hi, 5)
    ticks: dict[int, float] = {}
    for tick in tick_vals:
        frac = 0.0 if hi == lo else (hi - tick) / (hi - lo)
        row = int(round(frac * (plot_rows - 1)))
        ticks[min(max(row, 0), plot_rows - 1)] = tick
    ticks[0] = hi
    ticks[plot_rows - 1] = lo

    has_line = any(len(points) >= 1 for points, _color in prepared)
    if has_line:
        plot_rows_text = _block_lines(
            prepared, plot_cols, plot_rows, (lo, hi), grid_rows=set(ticks)
        )
    else:
        plot_rows_text = [
            Text("".join("·" if (row in ticks and col % 4 == 0) else " " for col in range(plot_cols)), style=MUTED)
            for row in range(plot_rows)
        ]
    if len(plot_rows_text) < plot_rows:
        plot_rows_text.extend(Text(" " * plot_cols) for _ in range(plot_rows - len(plot_rows_text)))

    body = Text()
    for row_i, plot_line in enumerate(plot_rows_text[:plot_rows]):
        if row_i in ticks:
            body.append(_fmt_tick(ticks[row_i], kind), style=MUTED)
            body.append("┤", style=MUTED)
        else:
            body.append(" " * left_w, style=MUTED)
            body.append("│", style=MUTED)
        body.append_text(plot_line)
        body.append("│", style=MUTED)
        body.append("\n")

    tick_cols = [0, plot_cols // 4, plot_cols // 2, (plot_cols * 3) // 4, plot_cols - 1]
    axis = Text(" " * left_w, style=MUTED)
    axis.append("└", style=MUTED)
    chars = ["─"] * plot_cols
    for col in tick_cols:
        chars[col] = "┬"
    axis.append("".join(chars), style=MUTED)
    axis.append("┘", style=MUTED)
    body.append_text(axis)
    body.append("\n")

    xs = x_values if x_values and len(x_values) >= 2 else list(range(max(len(prepared[0][0]) if prepared else 2, 2)))
    x_lo, x_hi = 0.0, float(xs[-1])
    labels = Text(" " * (left_w + 1), style=MUTED)
    placed = 0
    for col in tick_cols:
        frac = col / max(plot_cols - 1, 1)
        label = _fmt_step(x_lo + frac * (x_hi - x_lo))
        target = col - len(label) // 2
        gap = max(target - placed, 0)
        labels.append(" " * gap + label, style=MUTED)
        placed += gap + len(label)
    labels.append(" " * max(plot_cols - placed, 0), style=MUTED)
    body.append_text(labels)
    return body


def _meter(label: str, fraction: float, extra: str, color: str = ORANGE, width: int = 18) -> Text:
    fraction = min(max(fraction, 0.0), 1.0)
    filled = int(round(fraction * width))
    bar = "█" * filled + "░" * (width - filled)
    text = Text()
    text.append(f"{label:<4} ", style=ORANGE)
    text.append(bar, style=color)
    text.append(f" {extra}", style=INK)
    return text


def _knob_bar(fraction: float, width: int = 18) -> Text:
    fraction = min(max(fraction, 0.0), 1.0)
    pos = int(round(fraction * (width - 1)))
    chars = ["─"] * width
    chars[0] = "●"
    chars[width // 2] = "●"
    chars[-1] = "●"
    chars[pos] = "◆"
    text = Text("temp ", style=ORANGE)
    text.append("".join(chars), style=ORANGE)
    return text


def _want_trainer_tui(flag: bool) -> bool:
    env = os.environ.get("CR_TRAINING_TUI", "").strip().lower()
    if env in {"0", "false", "no"}:
        return False
    if env in {"1", "true", "yes"}:
        return bool(flag and HAS_RICH)
    return False


class TrainingDashboard:
    """Trainer writes JSONL. The watcher owns the Live display."""

    _active: TrainingDashboard | None = None

    def __init__(
        self,
        *,
        title: str,
        progress_path: Path,
        log_path: Path,
        epochs: int,
        batches_total: int,
        samples_total: int,
        batch_size: int,
        device: str,
        parameters: int,
        use_tui: bool = True,
        start_live: bool = True,
        persist: bool = True,
        reset: bool = True,
        screen: bool = False,
    ) -> None:
        self.title = title
        self.progress_path = Path(progress_path)
        self.log_path = Path(log_path)
        self.epochs = int(epochs)
        self.batches_total = max(int(batches_total), 1)
        self.samples_total = max(int(samples_total), 1)
        self.batch_size = int(batch_size)
        self.device = device
        self.parameters = int(parameters)
        self.persist = bool(persist)
        self.use_tui = bool(use_tui and HAS_RICH and self._tty())
        self.started = time.time()
        self.losses: list[float] = []
        self.cards: list[float] = []
        self.tiles_exact: list[float] = []
        self.tiles_near: list[float] = []
        self.waits: list[float] = []
        self.steps: list[float] = []
        self.events: deque[str] = deque(maxlen=LOG_LINES)
        self.live_status: str | None = None
        self.peak_vram = 0.0
        self.loss_ymax = 1.0
        self.hardware: HardwareSampler | None = None
        if getattr(TrainingDashboard, "_active", None) is not None:
            TrainingDashboard._active.close()
        TrainingDashboard._active = self
        self.state: dict[str, Any] = {
            "phase": "starting",
            "epoch": 0,
            "batch": 0,
            "loss": None,
            "acc": None,
            "card": None,
            "tile_exact": None,
            "tile_near": None,
            "wait": None,
            "wait_mae": None,
            "val_acc": None,
            "clock": None,
            "lr": None,
            "eta_seconds": 0.0,
            "progress_percent": 0.0,
            "samples_done": 0,
            "elapsed_seconds": 0.0,
        }
        if self.persist:
            self.progress_path.parent.mkdir(parents=True, exist_ok=True)
            self.log_path.parent.mkdir(parents=True, exist_ok=True)
            if reset:
                self.progress_path.write_text("", encoding="utf-8")
                self.log_path.write_text(
                    f"# {title}  device={device}  params={parameters:,}\n", encoding="utf-8"
                )
            else:
                if not self.progress_path.exists():
                    self.progress_path.write_text("", encoding="utf-8")
                if not self.log_path.exists():
                    self.log_path.write_text(
                        f"# {title}  device={device}  params={parameters:,}\n",
                        encoding="utf-8",
                    )
        self.console = (
            Console(highlight=False, color_system="truecolor", force_terminal=self.use_tui)
            if HAS_RICH
            else None
        )
        self._live: Any = None
        self._screen = bool(screen)
        if self.use_tui and start_live:
            self.start_live()

    def start_live(self) -> None:
        if not self.use_tui or self._live is not None or not HAS_RICH:
            return
        self._live = Live(
            self._render(),
            console=self.console,
            refresh_per_second=1,
            screen=self._screen,
            transient=False,
        )
        self._live.start()
        self._refresh(hardware=True)

    @staticmethod
    def _tty() -> bool:
        isatty = getattr(os, "isatty", None)
        if isatty is None:
            return False
        return bool(isatty(1) or isatty(2))

    def log(self, message: str) -> None:
        stamp = datetime.now().strftime("%H:%M:%S")
        line = f"{stamp}  {message}"
        self.events.append(line)
        if self.persist:
            with self.log_path.open("a", encoding="utf-8") as handle:
                handle.write(line + "\n")
            with self.progress_path.open("a", encoding="utf-8") as handle:
                handle.write(
                    json.dumps(
                        {
                            "phase": "log",
                            "message": line,
                            "timestamp": datetime.now(timezone.utc).isoformat(),
                        }
                    )
                    + "\n"
                )
        if not self.use_tui:
            print(line, flush=True)
        self._refresh()

    def _note_train_point(self, row: dict[str, Any]) -> None:
        phase = str(row.get("phase") or "")
        if phase in {"loading", "validation", "log"}:
            return
        if row.get("loss") is not None:
            loss = float(row["loss"])
            self.losses.append(loss)
            self.loss_ymax = max(self.loss_ymax, loss * 1.08)
            epoch = int(row.get("epoch") or 0)
            batch = int(row.get("batch") or 0)
            total = int(row.get("batches_total") or self.batches_total or 1)
            self.steps.append(max((epoch - 1) * total + batch, len(self.steps) + 1))
        card = row.get("card", row.get("acc"))
        if card is not None:
            self.cards.append(float(card))
        if row.get("tile_exact") is not None:
            self.tiles_exact.append(float(row["tile_exact"]))
        if row.get("tile_near") is not None:
            self.tiles_near.append(float(row["tile_near"]))
        if row.get("wait") is not None:
            self.waits.append(float(row["wait"]))

    def update(self, **fields: Any) -> None:
        self.state.update(fields)
        self._note_train_point(fields)
        if self.persist:
            row = {
                "timestamp": datetime.now(timezone.utc).isoformat(),
                **{key: value for key, value in self.state.items() if key != "hardware"},
                "elapsed_seconds": self.state.get("elapsed_seconds")
                or (time.time() - self.started),
            }
            with self.progress_path.open("a", encoding="utf-8") as handle:
                handle.write(json.dumps(row) + "\n")
        self._refresh()

    def ingest(self, row: dict[str, Any], *, announce: bool = False, refresh: bool = True) -> None:
        self.state.update(row)
        if row.get("batches_total"):
            self.batches_total = max(int(row["batches_total"]), 1)
        if row.get("samples_total"):
            self.samples_total = max(int(row["samples_total"]), 1)
        if row.get("epochs_total"):
            self.epochs = max(int(row["epochs_total"]), 1)
        if row.get("parameters"):
            self.parameters = int(row["parameters"])
        if row.get("batch_size"):
            self.batch_size = int(row["batch_size"])
        if row.get("device"):
            self.device = str(row["device"])
        if row.get("model_name"):
            self.title = str(row["model_name"])
        self._note_train_point(row)
        phase = str(row.get("phase") or "")
        if phase == "log":
            message = row.get("message")
            if message:
                self.events.append(str(message))
        elif phase == "validation":
            self.events.append(
                f"{datetime.now().strftime('%H:%M:%S')}  val  "
                f"ep {row.get('epoch')}  card={_pct(row.get('card') or row.get('acc'))}  "
                f"tile={_pct(row.get('tile_exact'))}  wait={_pct(row.get('wait'))}"
            )
        elif row.get("loss") is not None:
            self.live_status = (
                f"{datetime.now().strftime('%H:%M:%S')}  "
                f"ep {row.get('epoch')}  {row.get('batch')}/{self.batches_total}  "
                f"obj={_num(row.get('loss'))}  card={_pct(row.get('card') or row.get('acc'))}"
            )
        if announce:
            self.log(
                f"epoch {row.get('epoch')} batch {row.get('batch')} "
                f"obj={_num(row.get('loss'))} card={_pct(row.get('card') or row.get('acc'))}"
            )
        elif refresh:
            self._refresh()

    def close(self) -> None:
        if self._live is not None:
            self._refresh()
            self._live.stop()
            self._live = None
        if getattr(TrainingDashboard, "_active", None) is self:
            TrainingDashboard._active = None

    def _sampler(self) -> HardwareSampler:
        if self.hardware is None:
            self.hardware = HardwareSampler()
        return self.hardware

    def _refresh(self, hardware: bool = False) -> None:
        if self._live is None:
            return
        sampled = self._sampler().sample()
        self.peak_vram = max(self.peak_vram, float(sampled.get("vram_used_mb") or 0.0))
        sampled["peak_vram_mb"] = self.peak_vram
        self.state["hardware"] = sampled
        try:
            self._live.update(self._render())
        except Exception:
            pass

    def _elapsed(self) -> float:
        return float(self.state.get("elapsed_seconds") or (time.time() - self.started))

    def _throughput(self) -> tuple[float, float]:
        elapsed = max(self._elapsed(), 1e-6)
        samples = float(self.state.get("samples_done") or 0.0)
        work = (int(self.state.get("epoch") or 0) - 1) * self.batches_total + int(
            self.state.get("batch") or 0
        )
        return samples / elapsed, max(work, 0) / elapsed

    def _term(self) -> tuple[int, int]:
        if self.console is not None:
            return int(self.console.size.width), int(self.console.size.height)
        size = shutil.get_terminal_size((160, 42))
        return size.columns, size.lines

    def _header(self) -> Group:
        s = self.state
        progress = float(s.get("progress_percent") or 0.0) / 100.0
        eta = _fmt_seconds(s.get("eta_seconds") or 0.0)
        elapsed = _fmt_seconds(self._elapsed())
        width = max(18, min(36, self._term()[0] // 5))
        filled = int(round(min(max(progress, 0.0), 1.0) * width))
        bar = "█" * filled + "░" * (width - filled)
        card = s.get("card") if s.get("card") is not None else s.get("acc")
        metrics = Table.grid(expand=True, padding=(0, 2))
        metrics.add_column(ratio=2)
        metrics.add_column(ratio=2)
        metrics.add_column(ratio=3)
        metrics.add_column(ratio=2)
        metrics.add_row(
            Text.from_markup(f"[bold {BLUE}]loss[/] [{BLUE}]{_num(s.get('loss'))}[/]"),
            Text.from_markup(f"[bold {GREEN}]card[/] [{GREEN}]{_pct(card).strip()}[/]"),
            Text.from_markup(
                f"[bold {GREEN}]tile[/] [{GREEN}]{_pct(s.get('tile_exact')).strip()}[/]  "
                f"[{VIOLET}]{_pct(s.get('tile_near')).strip()}[/]"
            ),
            Text.from_markup(f"[bold {ORANGE}]wait[/] [{ORANGE}]{_pct(s.get('wait')).strip()}[/]"),
        )
        clock = Text.from_markup(
            f"[bold {ORANGE}]time[/] [{ORANGE}]{bar}[/]  "
            f"[dim]{elapsed}  eta {eta}  {progress * 100:4.1f}%[/]"
        )
        return Group(metrics, clock)

    def _plot_size(self, pane_w: int, pane_h: int) -> tuple[int, int]:
        return max(22, pane_w - 4), max(7, pane_h - 4)

    def _mini_chart(
        self,
        series: list[tuple[list[float], str]],
        *,
        kind: str,
        pane_w: int,
        pane_h: int,
        y_range: tuple[float, float] | None = None,
    ) -> Text:
        width, height = self._plot_size(pane_w, pane_h)
        xs = list(self.steps) if self.steps else None
        return _axis_chart(
            series,
            width=width,
            height=height,
            kind=kind,
            x_values=xs,
            y_range=y_range,
        )

    def _obj_panel(self, pane_w: int, pane_h: int) -> Panel:
        losses = list(self.losses)
        y_hi = max(self.loss_ymax, max(losses) * 1.08 if losses else 1.0)
        chart = self._mini_chart(
            [(losses, BLUE)],
            kind="loss",
            pane_w=pane_w,
            pane_h=pane_h,
            y_range=(0.0, y_hi),
        )
        return Panel(chart, title="[bold]obj[/]", border_style=BLUE, box=box.ROUNDED)

    def _tile_panel(self, pane_w: int, pane_h: int) -> Panel:
        exact = list(self.tiles_exact)
        near = list(self.tiles_near)
        extra = ""
        if exact and near:
            extra = f"  [{GREEN}]{_pct(exact[-1]).strip()}[/]  [{VIOLET}]{_pct(near[-1]).strip()}[/]"
        chart = self._mini_chart(
            [(exact, GREEN), (near, VIOLET)],
            kind="pct",
            pane_w=pane_w,
            pane_h=pane_h,
        )
        return Panel(
            chart,
            title=f"[bold]tile[/]{extra}",
            border_style=GREEN,
            box=box.ROUNDED,
        )

    def _wait_panel(self, pane_w: int, pane_h: int) -> Panel:
        waits = list(self.waits)
        extra = f"  [{ORANGE}]{_pct(waits[-1]).strip()}[/]" if waits else ""
        chart = self._mini_chart(
            [(waits, ORANGE)],
            kind="pct",
            pane_w=pane_w,
            pane_h=pane_h,
        )
        return Panel(
            chart,
            title=f"[bold]wait[/]{extra}",
            border_style=ORANGE,
            box=box.ROUNDED,
        )

    def _card_panel(self, pane_w: int, pane_h: int) -> Panel:
        cards = list(self.cards)
        extra = f"  [{GREEN}]{_pct(cards[-1]).strip()}[/]" if cards else ""
        chart = self._mini_chart(
            [(cards, GREEN)],
            kind="pct",
            pane_w=pane_w,
            pane_h=pane_h,
        )
        return Panel(
            chart,
            title=f"[bold]card[/]{extra}",
            border_style=GREEN,
            box=box.ROUNDED,
        )

    def _stats_panel(self) -> Panel:
        s = self.state
        sps, bps = self._throughput()
        epoch = int(s.get("epoch") or 0)
        batch = int(s.get("batch") or 0)
        epoch_frac = batch / max(self.batches_total, 1)
        epoch_bar = "█" * int(epoch_frac * 10) + "░" * (10 - int(epoch_frac * 10))
        rows = [
            ("ep", f"{epoch}/{self.epochs}  {epoch_bar}"),
            ("step", f"{batch}/{self.batches_total}"),
            ("done", f"{int(s.get('samples_done') or 0):,}/{self.samples_total:,}"),
            ("rate", f"{sps:,.0f}/s  {bps:.1f} it/s"),
            ("lr", f"{s.get('lr'):.2e}" if s.get("lr") is not None else "—"),
            ("Δt", f"{_num(s.get('wait_mae') or s.get('timing'))}s"),
            ("clk", _num(s.get("clock"))),
            ("val", _pct(s.get("val_acc"))),
        ]
        body = Text()
        for label, value in rows:
            body.append(f"{label:<5}", style=MUTED)
            body.append(f"{value}\n", style=INK)
        return Panel(body, title="[bold]run[/]", border_style=BLUE, box=box.ROUNDED)

    def _hardware_panel(self) -> Panel:
        hw = self.state.get("hardware") or self._sampler().sample()
        group = Group(
            Text(str(hw.get("gpu_name") or "gpu"), style=INK),
            _meter("cpu", hw.get("cpu", 0.0), f"{100*hw.get('cpu', 0):.0f}%"),
            _meter("gpu", hw.get("gpu_util", 0.0), f"{100*hw.get('gpu_util', 0):.0f}%"),
            _meter(
                "ram",
                hw.get("ram", 0.0),
                f"{_fmt_bytes(hw.get('ram_used_mb', 0))}/{_fmt_bytes(hw.get('ram_total_mb', 1))}",
            ),
            _meter(
                "vram",
                hw.get("vram", 0.0),
                f"{_fmt_bytes(hw.get('vram_used_mb', 0))}/{_fmt_bytes(hw.get('vram_total_mb', 1))}",
            ),
            _knob_bar(min(hw.get("gpu_temp", 0.0) / 90.0, 1.0)),
            Text(f"     {hw.get('gpu_temp', 0):.0f}°C", style=MUTED),
        )
        return Panel(group, title=f"[bold {ORANGE}]hw[/]", border_style=ORANGE, box=box.ROUNDED)

    def _log_panel(self) -> Panel:
        lines = list(self.events)[-12:]
        if self.live_status:
            lines.append(self.live_status)
        if not lines:
            body = Text("waiting…", style=MUTED)
        else:
            body = Text()
            for line in lines:
                style = INK
                lower = line.lower()
                if "val" in lower or "best" in lower or "saved" in lower:
                    style = ORANGE
                elif "epoch" in lower:
                    style = BLUE
                body.append(line + "\n", style=style)
        return Panel(body, title="[bold]log[/]", border_style="grey37", box=box.ROUNDED)

    def _render(self) -> Layout:
        cols, lines = self._term()
        header_h = 4
        top_h = max(10, int(round((lines - header_h) * 19 / 39)))
        layout = Layout()
        layout.split_column(
            Layout(name="header", size=header_h),
            Layout(name="top", size=top_h),
            Layout(name="bottom"),
        )
        layout["top"].split_row(
            Layout(name="obj", ratio=1),
            Layout(name="tile", ratio=1),
            Layout(name="wait", ratio=1),
        )
        third = max(24, cols // 3)
        layout["bottom"].split_row(
            Layout(name="card", size=third),
            Layout(name="hw", size=max(28, int(cols * 42 / 160))),
            Layout(name="run", size=max(24, int(cols * 32 / 160))),
            Layout(name="log"),
        )
        pane_w = max(24, cols // 3)
        pane_h_top = max(8, top_h)
        pane_h_bot = max(8, lines - header_h - top_h)
        header = Panel(
            self._header(),
            title=f"[bold {ORANGE}]{self.title}[/]",
            border_style=ORANGE,
            box=box.ROUNDED,
        )
        layout["header"].update(header)
        layout["obj"].update(self._obj_panel(pane_w, pane_h_top))
        layout["tile"].update(self._tile_panel(pane_w, pane_h_top))
        layout["wait"].update(self._wait_panel(pane_w, pane_h_top))
        layout["card"].update(self._card_panel(pane_w, pane_h_bot))
        layout["hw"].update(self._hardware_panel())
        layout["run"].update(self._stats_panel())
        layout["log"].update(self._log_panel())
        return layout


def watch_progress(progress_path: str | Path, interval: float = 2.0) -> None:
    """Read-only TUI over an already-running trainer JSONL file."""
    target = Path(progress_path)
    dash: TrainingDashboard | None = None
    offset = 0
    last_draw = 0.0
    poll = max(float(interval), 1.0)
    try:
        while True:
            drew = False
            new_rows: list[dict[str, Any]] = []
            if target.exists():
                size = target.stat().st_size
                if size < offset:
                    offset = 0
                    if dash is not None:
                        dash.close()
                        dash = None
                with target.open("r", encoding="utf-8") as handle:
                    handle.seek(offset)
                    chunk = handle.read()
                    offset = handle.tell()
                for line in chunk.splitlines():
                    if not line.strip():
                        continue
                    try:
                        new_rows.append(json.loads(line))
                    except json.JSONDecodeError:
                        continue
                if new_rows:
                    if dash is None:
                        seed = next(
                            (row for row in reversed(new_rows) if row.get("parameters")),
                            new_rows[-1],
                        )
                        dash = TrainingDashboard(
                            title=str(seed.get("model_name") or seed.get("phase") or "policy"),
                            progress_path=target,
                            log_path=target.with_suffix(".watch.log"),
                            epochs=int(seed.get("epochs_total") or 1),
                            batches_total=int(seed.get("batches_total") or 1),
                            samples_total=int(seed.get("samples_total") or 1),
                            batch_size=int(seed.get("batch_size") or 1),
                            device=str(seed.get("device") or "cuda"),
                            parameters=int(seed.get("parameters") or 0),
                            persist=False,
                            screen=True,
                            start_live=True,
                        )
                        for row in new_rows[:-1]:
                            dash.ingest(row, announce=False, refresh=False)
                        dash.ingest(new_rows[-1], announce=False, refresh=True)
                    else:
                        for row in new_rows[:-1]:
                            dash.ingest(row, announce=False, refresh=False)
                        dash.ingest(new_rows[-1], announce=False, refresh=True)
                    last_draw = time.time()
                    drew = True
                    if not dash.use_tui:
                        row = new_rows[-1]
                        print(
                            f"progress {row.get('phase')} epoch={row.get('epoch')} "
                            f"batch={row.get('batch')} work={float(row.get('progress_percent') or 0):.1f}% "
                            f"ETA={_fmt_seconds(float(row.get('eta_seconds') or 0))} "
                            f"obj={_num(row.get('loss'))} card={_pct(row.get('card') or row.get('acc'))}",
                            flush=True,
                        )
            if dash is not None and not drew and time.time() - last_draw >= 1.0:
                dash._refresh()
                last_draw = time.time()
            time.sleep(poll)
    finally:
        if dash is not None:
            dash.close()
