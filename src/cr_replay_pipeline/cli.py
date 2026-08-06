from __future__ import annotations

import argparse
import json
import os
import sys
import textwrap
import time
from datetime import datetime
from pathlib import Path

from .api import discover_battles, normalize_battlelog_directory
from .cleaner import clean_directory
from .frontier import Frontier
from .server import serve


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="cr-replays")
    commands = parser.add_subparsers(dest="command", required=True)

    server = commands.add_parser("serve", help="Run the local extension ingest server")
    server.add_argument("--raw-dir", default="data/raw")
    server.add_argument("--host", default="127.0.0.1")
    server.add_argument("--port", type=int, default=8765)
    server.add_argument("--db", default="data/collector.sqlite3")

    status = commands.add_parser("status", help="Show durable collector status")
    status.add_argument("--db", default="data/collector.sqlite3")
    status.add_argument("--raw-dir", default="data/raw")
    status.add_argument("--json", action="store_true", help="Emit machine-readable JSON")
    status.add_argument("--no-color", action="store_true")

    watch = commands.add_parser("watch", help="Continuously show collector status")
    watch.add_argument("--db", default="data/collector.sqlite3")
    watch.add_argument("--raw-dir", default="data/raw")
    watch.add_argument("--interval", type=float, default=5)
    watch.add_argument("--json", action="store_true", help="Emit one JSON object per line")
    watch.add_argument("--no-color", action="store_true")

    pause = commands.add_parser("pause", help="Pause new player claims")
    pause.add_argument("--db", default="data/collector.sqlite3")
    pause.add_argument("--reason", default="paused by operator")

    resume = commands.add_parser("resume", help="Resume player claims")
    resume.add_argument("--db", default="data/collector.sqlite3")

    seed = commands.add_parser("seed", help="Seed the frontier from tags or replay files")
    seed.add_argument("tags", nargs="*")
    seed.add_argument("--input", help="Existing raw replay directory")
    seed.add_argument("--db", default="data/collector.sqlite3")

    discover = commands.add_parser(
        "discover", help="Collect official leaderboard battlelog metadata"
    )
    discover.add_argument("--output", default="data/metadata/battles.jsonl")
    discover.add_argument("--limit", type=int, default=1000)
    discover.add_argument("--workers", type=int, default=12)

    normalize = commands.add_parser(
        "normalize-battlelogs",
        help="Deduplicate and normalize previously downloaded battlelog JSON files",
    )
    normalize.add_argument("--input", required=True)
    normalize.add_argument("--output", default="data/metadata/battles.jsonl")

    clean = commands.add_parser("clean", help="Parse, label, deduplicate, and filter")
    clean.add_argument("--input", required=True)
    clean.add_argument("--output", default="data/cleaned")
    clean.add_argument("--metadata")
    clean.add_argument("--legacy-roster", choices=["december-2025"])
    clean.add_argument("--audit-only", action="store_true")
    clean.add_argument("--report")

    train_winner = commands.add_parser(
        "train-winner",
        help="Train a GPU model that predicts the battle winner from action prefixes",
    )
    train_winner.add_argument("--input", default="data/raw")
    train_winner.add_argument("--output", default="models/winner_predictor")
    train_winner.add_argument("--card-costs", default="data/card_costs.json")
    train_winner.add_argument("--epochs", type=int, default=40)
    train_winner.add_argument("--batch-size", type=int, default=64)
    train_winner.add_argument("--lr", type=float, default=2e-4)
    train_winner.add_argument("--d-model", type=int, default=160)
    train_winner.add_argument("--num-layers", type=int, default=2)
    train_winner.add_argument("--min-card-plays", type=int, default=12)
    train_winner.add_argument("--seed", type=int, default=42)
    train_winner.add_argument("--device", default=None, help="cuda, cpu, or omit for auto")

    train_winner_hgb = commands.add_parser(
        "train-winner-hgb",
        help=(
            "Train a full-game symmetric tree ensemble "
            "(stronger than the sequence net on current replays)"
        ),
    )
    train_winner_hgb.add_argument("--input", default="data/raw")
    train_winner_hgb.add_argument("--output", default="models/winner_predictor")
    train_winner_hgb.add_argument("--card-costs", default="data/card_costs.json")
    train_winner_hgb.add_argument("--min-card-plays", type=int, default=12)
    train_winner_hgb.add_argument("--seed", type=int, default=42)
    train_winner_hgb.add_argument(
        "--trees",
        "--ensemble-size",
        dest="trees",
        type=int,
        default=100,
        help="Number of Extra Trees (the old --ensemble-size name remains an alias)",
    )

    report_winner = commands.add_parser(
        "report-winner",
        help="Generate HTML training reports for winner prediction models",
    )
    report_winner.add_argument("--model-dir", default="models/winner_predictor")
    report_winner.add_argument("--output-dir", default="reports")
    return parser


def _status(frontier: Frontier, raw_dir: str | None = None) -> dict:
    status = frontier.status()
    if raw_dir is not None:
        status["raw_replays"] = sum(1 for _ in Path(raw_dir).rglob("*.json"))
        players = status["players"]
        actionable = players["queued"] + players["leased"] + players["completed"]
        status["crawl_progress_percent"] = (
            round(100 * players["completed"] / actionable, 1) if actionable else 100.0
        )
    return status


class _Palette:
    def __init__(self, enabled: bool) -> None:
        self.enabled = enabled

    def paint(self, text: str, code: str) -> str:
        return f"\033[{code}m{text}\033[0m" if self.enabled else text

    def green(self, text: str) -> str:
        return self.paint(text, "32")

    def yellow(self, text: str) -> str:
        return self.paint(text, "33")

    def red(self, text: str) -> str:
        return self.paint(text, "31")

    def cyan(self, text: str) -> str:
        return self.paint(text, "36")

    def dim(self, text: str) -> str:
        return self.paint(text, "2")

    def bold(self, text: str) -> str:
        return self.paint(text, "1")


def _palette(no_color: bool = False) -> _Palette:
    return _Palette(sys.stdout.isatty() and not no_color and "NO_COLOR" not in os.environ)


def _state(status: dict) -> tuple[str, str]:
    if status["paused"]:
        return "PAUSED", "yellow"
    if status["players"]["leased"]:
        return "RUN", "green"
    if status["players"]["queued"]:
        return "IDLE", "yellow"
    return "DONE", "cyan"


def _progress_bar(percent: float, width: int = 28) -> str:
    filled = min(width, max(0, round(width * percent / 100)))
    return "█" * filled + "░" * (width - filled)


def _print_dashboard(status: dict, colors: _Palette) -> None:
    players = status["players"]
    rate = status["rate"]
    state, color_name = _state(status)
    width = 58

    def box(text: str = "") -> str:
        return colors.dim("│") + f" {text:<{width - 2}} " + colors.dim("│")

    print(colors.dim("┌" + "─" * width + "┐"))
    title = f"Clash Royale Collector  •  {state}"
    title_line = box(title).replace(state, getattr(colors, color_name)(state), 1)
    print(colors.bold(title_line))
    print(colors.dim("├" + "─" * width + "┤"))
    target = rate.get("recommended_workers", "—")
    print(box(f"Replays  {status.get('raw_replays', '—'):<5}   Players  {players['total']:<5}   Workers {players['leased']} / {target}"))
    print(box(f"Queue    {players['queued']:<5}   Completed {players['completed']:<5}   Manual  {players['manual']}"))
    print(box(f"Pace     {rate['interval_seconds']:.2f}s   Clean     {rate['clean_streak']:<5}   429s    {rate['total_rate_limits']}"))

    progress = float(status.get("crawl_progress_percent", 0))
    print(box())
    print(box(f"{_progress_bar(progress)}  {progress:5.1f}%"))

    if status["paused"]:
        reason = status.get("pause_reason") or "No reason supplied"
        print(box())
        wrapped = textwrap.wrap(reason, width=width - 13) or [reason]
        for index, line in enumerate(wrapped):
            label = "Reason" if index == 0 else ""
            rendered = box(f"{label:<8} {line}")
            if label:
                rendered = rendered.replace(label, colors.yellow(label), 1)
            print(rendered)
    elif status["active"]:
        print(box())
        worker_heading = box("Workers").replace("Workers", colors.bold("Workers"), 1)
        print(worker_heading)
        for worker in status["active"]:
            lease = datetime.fromtimestamp(worker["lease_until"]).strftime("%H:%M:%S")
            worker_line = box(
                f"● {worker['worker_id']:<10} {worker['tag']:<12} lease until {lease}"
            ).replace("●", colors.green("●"), 1)
            print(worker_line)
    elif players["queued"]:
        print(box())
        waiting = box("● Waiting for a Chrome worker to claim the queue")
        print(waiting.replace("●", colors.yellow("●"), 1))

    print(colors.dim("└" + "─" * width + "┘"), flush=True)


_WATCH_COLUMNS = (
    ("TIME", 8, "left"),
    ("STATE", 6, "left"),
    ("RAW", 5, "right"),
    ("TOTAL", 5, "right"),
    ("QUEUE", 5, "right"),
    ("ACT", 5, "right"),
    ("DONE", 5, "right"),
    ("MAN", 3, "right"),
    ("PACE", 5, "right"),
    ("OK", 4, "right"),
    ("429", 3, "right"),
    ("PROG", 6, "right"),
)


def _watch_cells(status: dict) -> list[str]:
    players = status["players"]
    rate = status["rate"]
    state, _ = _state(status)
    return [
        datetime.now().strftime("%H:%M:%S"),
        state,
        str(status.get("raw_replays", "—")),
        str(players["total"]),
        str(players["queued"]),
        f"{players['leased']}/{rate.get('recommended_workers', '—')}",
        str(players["completed"]),
        str(players["manual"]),
        f"{rate['interval_seconds']:.2f}s",
        str(rate["clean_streak"]),
        str(rate["total_rate_limits"]),
        f"{status.get('crawl_progress_percent', 0):.1f}%",
    ]


def _format_watch_cells(cells: list[str]) -> str:
    rendered = []
    for value, (_, width, alignment) in zip(cells, _WATCH_COLUMNS):
        rendered.append(f"{value:<{width}}" if alignment == "left" else f"{value:>{width}}")
    return " ".join(rendered)


def _print_watch_header(colors: _Palette) -> None:
    print(colors.bold(colors.dim(_format_watch_cells([column[0] for column in _WATCH_COLUMNS]))), flush=True)


def _print_watch_row(status: dict, colors: _Palette) -> None:
    cells = _watch_cells(status)
    _, color_name = _state(status)
    line = _format_watch_cells(cells)
    # Color only the state word, preserving the table's fixed widths.
    padded_state = f"{cells[1]:<6}"
    line = line.replace(padded_state, getattr(colors, color_name)(padded_state), 1)
    print(line, flush=True)


def _print_json(status: dict, *, compact: bool = False) -> None:
    print(
        json.dumps(status, ensure_ascii=False, separators=(",", ":") if compact else None, indent=None if compact else 2),
        flush=True,
    )


def main() -> None:
    args = build_parser().parse_args()
    if args.command == "serve":
        serve(args.raw_dir, args.host, args.port, args.db)
    elif args.command == "status":
        status = _status(Frontier(args.db), args.raw_dir)
        if args.json:
            _print_json(status)
        else:
            _print_dashboard(status, _palette(args.no_color))
    elif args.command == "watch":
        frontier = Frontier(args.db)
        colors = _palette(args.no_color)
        if not args.json:
            _print_watch_header(colors)
        try:
            while True:
                status = _status(frontier, args.raw_dir)
                if args.json:
                    _print_json(status, compact=True)
                else:
                    _print_watch_row(status, colors)
                time.sleep(max(0.2, args.interval))
        except KeyboardInterrupt:
            print()
    elif args.command == "pause":
        frontier = Frontier(args.db)
        frontier.pause(args.reason)
        _print_dashboard(_status(frontier, "data/raw"), _palette())
    elif args.command == "resume":
        frontier = Frontier(args.db)
        frontier.resume()
        _print_dashboard(_status(frontier, "data/raw"), _palette())
    elif args.command == "seed":
        frontier = Frontier(args.db)
        result = {"tags_added": frontier.seed(args.tags)}
        if args.input:
            result["bootstrap"] = frontier.bootstrap(args.input)
        result["status"] = frontier.status()
        print(json.dumps(result, indent=2, ensure_ascii=False))
    elif args.command == "discover":
        print(
            json.dumps(
                discover_battles(args.output, args.limit, args.workers),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "normalize-battlelogs":
        print(
            json.dumps(
                normalize_battlelog_directory(args.input, args.output),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "clean":
        print(
            json.dumps(
                clean_directory(
                    args.input,
                    args.output,
                    metadata_path=args.metadata,
                    legacy_roster=args.legacy_roster,
                    audit_only=args.audit_only,
                    report_path=args.report,
                ),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "train-winner":
        from .winner_train import train_winner_model

        print(
            json.dumps(
                train_winner_model(
                    input_dir=args.input,
                    output_dir=args.output,
                    card_costs_path=args.card_costs,
                    epochs=args.epochs,
                    batch_size=args.batch_size,
                    learning_rate=args.lr,
                    d_model=args.d_model,
                    num_layers=args.num_layers,
                    min_card_plays=args.min_card_plays,
                    seed=args.seed,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "train-winner-hgb":
        from .winner_tabular import train_winner_tabular

        print(
            json.dumps(
                train_winner_tabular(
                    input_dir=args.input,
                    output_dir=args.output,
                    card_costs_path=args.card_costs,
                    min_card_plays=args.min_card_plays,
                    seed=args.seed,
                    extra_trees=args.trees,
                ),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "report-winner":
        from .winner_report import render_winner_reports

        paths = render_winner_reports(args.model_dir, args.output_dir)
        print(json.dumps({"reports": [str(path) for path in paths]}, indent=2))


if __name__ == "__main__":
    main()
