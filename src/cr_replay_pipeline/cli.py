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

    train_realism = commands.add_parser(
        "train-realism",
        help=(
            "Train a realism scorer that separates real battles from "
            "legal-but-random synthetic ones"
        ),
    )
    train_realism.add_argument("--input", default="data/raw")
    train_realism.add_argument("--output", default="models/realism_scorer")
    train_realism.add_argument("--card-costs", default="data/card_costs.json")
    train_realism.add_argument("--min-card-plays", type=int, default=12)
    train_realism.add_argument("--seed", type=int, default=42)
    train_realism.add_argument("--trees", type=int, default=120)
    train_realism.add_argument(
        "--per-tier",
        type=int,
        default=1,
        help="How many synthetic battles to generate per real battle per tier",
    )

    report_realism = commands.add_parser(
        "report-realism",
        help="Generate an HTML training report for the realism scorer",
    )
    report_realism.add_argument("--model-dir", default="models/realism_scorer")
    report_realism.add_argument("--output-dir", default="reports")

    train_policy = commands.add_parser(
        "train-policy",
        help=(
            "Train a behavior-cloning next-action policy "
            "(card slot, placement, timing) from replay history"
        ),
    )
    train_policy.add_argument("--input", default="data/raw")
    train_policy.add_argument("--output", default="models/policy_bc")
    train_policy.add_argument("--card-costs", default="data/card_costs.json")
    train_policy.add_argument(
        "--realism-model-dir",
        default="models/realism_scorer",
        help="Optional realism scorer for offline rollout scoring",
    )
    train_policy.add_argument("--epochs", type=int, default=25)
    train_policy.add_argument("--batch-size", type=int, default=256)
    train_policy.add_argument("--lr", type=float, default=2e-4)
    train_policy.add_argument("--d-model", type=int, default=160)
    train_policy.add_argument("--num-layers", type=int, default=2)
    train_policy.add_argument("--min-card-plays", type=int, default=12)
    train_policy.add_argument("--seed", type=int, default=42)
    train_policy.add_argument("--device", default=None, help="cuda, cpu, or omit for auto")
    train_policy.add_argument(
        "--version",
        default="2",
        choices=["2", "3", "4"],
        help=(
            "2=cycle features; 3=threat+reaction; "
            "4=v3 + jointly trained card-conditioned placement"
        ),
    )
    train_policy.add_argument(
        "--reaction-weight",
        type=float,
        default=3.0,
        help="Loss weight for post-threat reaction samples (v3)",
    )
    train_policy.add_argument(
        "--reaction-repeats",
        type=int,
        default=2,
        help="Extra index repeats for reaction samples (v3)",
    )

    showcase = commands.add_parser(
        "showcase-policy",
        help=(
            "Score two policy checkpoints on held-out actions and emit "
            "interactive-report data (arena heatmaps, fixes, scenarios)"
        ),
    )
    showcase.add_argument("--input", default="data/raw")
    showcase.add_argument("--new-policy-dir", default="models/policy_bc_v4")
    showcase.add_argument("--old-policy-dir", default="models/policy_bc_v3")
    showcase.add_argument("--card-costs", default="data/card_costs.json")
    showcase.add_argument("--output", default="reports/policy_showcase_v4.json")
    showcase.add_argument("--max-battles", type=int, default=700)
    showcase.add_argument("--max-samples-per-battle", type=int, default=10)
    showcase.add_argument("--top-cards", type=int, default=12)
    showcase.add_argument("--seed", type=int, default=42)
    showcase.add_argument("--device", default=None)

    report_showcase = commands.add_parser(
        "report-showcase",
        help="Render the interactive policy showcase HTML report",
    )
    report_showcase.add_argument(
        "--showcase-json", default="reports/policy_showcase_v4.json"
    )
    report_showcase.add_argument("--model-dir", default="models/policy_bc_v4")
    report_showcase.add_argument("--old-model-dir", default="models/policy_bc_v3")
    report_showcase.add_argument(
        "--slice-json", default="reports/defense_slice_eval_v4.json"
    )
    report_showcase.add_argument(
        "--support-json", default="reports/defense_support_audit_v4.json"
    )
    report_showcase.add_argument("--output", default="reports/policy_bc_v4_showcase.html")

    report_policy = commands.add_parser(
        "report-policy",
        help="Generate an HTML training report for the behavior-cloning policy",
    )
    report_policy.add_argument("--model-dir", default="models/policy_bc")
    report_policy.add_argument("--output-dir", default="reports")
    report_policy.add_argument(
        "--output-name",
        default=None,
        help="Optional HTML filename (default derived from model_name)",
    )

    predict_policy = commands.add_parser(
        "predict-policy",
        help="Offline next-action prediction from a raw replay prefix (no live play)",
    )
    predict_policy.add_argument("replay", help="Path to a raw RoyaleAPI replay JSON")
    predict_policy.add_argument("--model-dir", default="models/policy_bc")
    predict_policy.add_argument("--card-costs", default="data/card_costs.json")
    predict_policy.add_argument("--prefix-events", type=int, default=20)
    predict_policy.add_argument("--side", choices=["team", "opponent"], default="team")
    predict_policy.add_argument("--device", default=None)

    matchup = commands.add_parser(
        "eval-matchups",
        help=(
            "Mine strong deck matchups from data, run policy-vs-policy games, "
            "and compare win rates via the winner model"
        ),
    )
    matchup.add_argument("--input", default="data/raw")
    matchup.add_argument("--policy-dir", default="models/policy_bc")
    matchup.add_argument("--winner-dir", default="models/winner_predictor")
    matchup.add_argument("--card-costs", default="data/card_costs.json")
    matchup.add_argument("--output", default="reports/matchup_eval.json")
    matchup.add_argument("--games", type=int, default=48)
    matchup.add_argument("--top-k", type=int, default=6)
    matchup.add_argument("--min-n", type=int, default=60)
    matchup.add_argument("--seed", type=int, default=42)
    matchup.add_argument("--device", default=None)

    defense = commands.add_parser(
        "eval-defense",
        help=(
            "Probe defensive card choice: opponent drops a threat, hand has "
            "1 strong answer + 3 weak cards"
        ),
    )
    defense.add_argument("--policy-dir", default="models/policy_bc")
    defense.add_argument("--card-costs", default="data/card_costs.json")
    defense.add_argument("--output", default="reports/defense_eval.json")
    defense.add_argument("--trials", type=int, default=64)
    defense.add_argument("--seed", type=int, default=42)
    defense.add_argument("--device", default=None)

    defense_slice = commands.add_parser(
        "eval-defense-slice",
        help=(
            "Score policy on real held-out reaction windows after hog/balloon/"
            "graveyard/golem (no forced hand)"
        ),
    )
    defense_slice.add_argument("--input", default="data/raw")
    defense_slice.add_argument("--policy-dir", default="models/policy_bc")
    defense_slice.add_argument("--card-costs", default="data/card_costs.json")
    defense_slice.add_argument("--output", default="reports/defense_slice_eval.json")
    defense_slice.add_argument("--max-response-seconds", type=float, default=5.0)
    defense_slice.add_argument("--seed", type=int, default=42)
    defense_slice.add_argument("--device", default=None)

    report_defense_slice = commands.add_parser(
        "report-defense-slice",
        help="HTML report for real defense-slice + fair counterfactual probes",
    )
    report_defense_slice.add_argument(
        "--slice-json", default="reports/defense_slice_eval.json"
    )
    report_defense_slice.add_argument(
        "--fair-json", default="reports/defense_eval_fair.json"
    )
    report_defense_slice.add_argument(
        "--output", default="reports/defense_slice_v1.html"
    )

    support_audit = commands.add_parser(
        "audit-defense-support",
        help=(
            "Data-support + natural-hand counterfactual audit for defense "
            "probe cells (GY→poison, hog→tornado, controls)"
        ),
    )
    support_audit.add_argument("--input", default="data/raw")
    support_audit.add_argument("--policy-dir", default="models/policy_bc")
    support_audit.add_argument("--card-costs", default="data/card_costs.json")
    support_audit.add_argument(
        "--output", default="reports/defense_support_audit.json"
    )
    support_audit.add_argument("--max-response-seconds", type=float, default=5.0)
    support_audit.add_argument("--seed", type=int, default=42)
    support_audit.add_argument("--device", default=None)
    support_audit.add_argument(
        "--no-score-model",
        action="store_true",
        help="Coverage-only audit (skip policy scoring on test)",
    )

    report_support = commands.add_parser(
        "report-defense-support",
        help="HTML report for defense support / natural-hand audit",
    )
    report_support.add_argument(
        "--audit-json", default="reports/defense_support_audit.json"
    )
    report_support.add_argument(
        "--output", default="reports/defense_support_audit_v1.html"
    )

    placement = commands.add_parser(
        "placement-probe",
        help=(
            "Experiment B: card-conditioned placement probe + per-card "
            "lookup control (offline; freezes policy_bc_v3 trunk)"
        ),
    )
    placement.add_argument("--input", default="data/raw")
    placement.add_argument("--policy-dir", default="models/policy_bc_v3")
    placement.add_argument("--card-costs", default="data/card_costs.json")
    placement.add_argument(
        "--output-json", default="reports/placement_probe_v1.json"
    )
    placement.add_argument(
        "--output-html", default="reports/placement_probe_v1.html"
    )
    placement.add_argument("--epochs", type=int, default=10)
    placement.add_argument("--batch-size", type=int, default=256)
    placement.add_argument("--probe-batch-size", type=int, default=512)
    placement.add_argument("--lr", type=float, default=1e-3)
    placement.add_argument("--hidden", type=int, default=256)
    placement.add_argument("--min-card-plays", type=int, default=12)
    placement.add_argument("--seed", type=int, default=42)
    placement.add_argument("--device", default=None)

    phone_lab = commands.add_parser(
        "phone-lab",
        help=(
            "Open a browser lab for dual-phone calibration and live battle: "
            "streams, hand detection, test taps, and policy-driven AI vs AI"
        ),
    )
    phone_lab.add_argument("--host", default="127.0.0.1")
    phone_lab.add_argument("--port", type=int, default=8766)
    phone_lab.add_argument(
        "--pixel9",
        default="4B090DLAQ002ZT",
        help="ADB serial for Pixel 9",
    )
    phone_lab.add_argument(
        "--pixel8",
        default="41060DLJH000KW",
        help="ADB serial for Pixel 8",
    )
    phone_lab.add_argument(
        "--calib-dir", default="data/phone_lab/calibrations"
    )
    phone_lab.add_argument(
        "--yolo-model",
        default="/home/cochon/Documents/ClashRoyaleAI/models/yolo/card_detector.pt",
    )
    phone_lab.add_argument(
        "--card-costs", default="data/card_costs.json"
    )
    phone_lab.add_argument(
        "--policy-v3", default="models/policy_bc_v3"
    )
    phone_lab.add_argument(
        "--policy-v4", default="models/policy_bc_v4"
    )
    phone_lab.add_argument(
        "--no-open",
        action="store_true",
        help="Do not open a browser tab automatically",
    )

    place_cal = commands.add_parser(
        "phone-lab-calibrate",
        help=(
            "Matplotlib click calibration: set bridge left/right, my corners, "
            "and enemy corners on a live phone screenshot"
        ),
    )
    place_cal.add_argument(
        "--phone",
        default="pixel9",
        help="pixel9, pixel8, both, or an ADB serial",
    )
    place_cal.add_argument(
        "--calib-dir", default="data/phone_lab/calibrations"
    )
    place_cal.add_argument(
        "--pixel9", default="4B090DLAQ002ZT", help="Used when --phone both"
    )
    place_cal.add_argument(
        "--pixel8", default="41060DLJH000KW", help="Used when --phone both"
    )

    audit_hands = commands.add_parser(
        "audit-hands",
        help=(
            "Experiment A: exact CR cycle hand reconstruction vs oldest-four "
            "heuristic; rescore frozen policy masks (offline)"
        ),
    )
    audit_hands.add_argument("--input", default="data/raw")
    audit_hands.add_argument("--policy-dir", default="models/policy_bc_v3")
    audit_hands.add_argument("--card-costs", default="data/card_costs.json")
    audit_hands.add_argument(
        "--output-json", default="reports/hand_audit_v1.json"
    )
    audit_hands.add_argument(
        "--output-html", default="reports/hand_audit_v1.html"
    )
    audit_hands.add_argument("--seed", type=int, default=42)
    audit_hands.add_argument(
        "--device", default="cpu", help="cpu recommended for this audit"
    )
    audit_hands.add_argument(
        "--max-quality-battles",
        type=int,
        default=None,
        help="Optional cap on battles for heuristic-quality pass",
    )
    audit_hands.add_argument(
        "--cache",
        default=None,
        help="Optional battles cache pickle path",
    )
    audit_hands.add_argument(
        "--no-stale-cache",
        action="store_true",
        help="Refuse stale winner_battles_cache.pkl (rebuild if file_count drifts)",
    )
    audit_hands.add_argument(
        "--quality-workers",
        type=int,
        default=4,
        help="Process workers for heuristic-quality pass",
    )
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
    elif args.command == "train-realism":
        from .realism_train import train_realism_scorer

        print(
            json.dumps(
                train_realism_scorer(
                    input_dir=args.input,
                    output_dir=args.output,
                    card_costs_path=args.card_costs,
                    min_card_plays=args.min_card_plays,
                    seed=args.seed,
                    trees=args.trees,
                    per_tier=args.per_tier,
                ),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "report-realism":
        from .realism_report import render_realism_report

        path = render_realism_report(
            args.model_dir,
            Path(args.output_dir) / "realism_scorer_v1.html",
        )
        print(json.dumps({"report": str(path)}, indent=2))
    elif args.command == "train-policy":
        from .policy_train import train_policy_model

        output = args.output
        if args.version == "3" and output == "models/policy_bc":
            output = "models/policy_bc_v3"
        elif args.version == "4" and output == "models/policy_bc":
            output = "models/policy_bc_v4"
        print(
            json.dumps(
                train_policy_model(
                    input_dir=args.input,
                    output_dir=output,
                    card_costs_path=args.card_costs,
                    realism_model_dir=args.realism_model_dir,
                    epochs=args.epochs,
                    batch_size=args.batch_size,
                    learning_rate=args.lr,
                    d_model=args.d_model,
                    num_layers=args.num_layers,
                    min_card_plays=args.min_card_plays,
                    seed=args.seed,
                    device_name=args.device,
                    version=args.version,
                    reaction_weight=args.reaction_weight,
                    reaction_repeats=args.reaction_repeats,
                ),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "showcase-policy":
        from .policy_showcase import build_policy_showcase

        build_policy_showcase(
            input_dir=args.input,
            new_policy_dir=args.new_policy_dir,
            old_policy_dir=args.old_policy_dir,
            card_costs_path=args.card_costs,
            output_path=args.output,
            max_battles=args.max_battles,
            max_samples_per_battle=args.max_samples_per_battle,
            top_cards=args.top_cards,
            seed=args.seed,
            device_name=args.device,
        )
    elif args.command == "report-showcase":
        from .policy_showcase_report import render_policy_showcase_report

        path = render_policy_showcase_report(
            showcase_path=args.showcase_json,
            model_dir=args.model_dir,
            old_model_dir=args.old_model_dir,
            slice_path=args.slice_json,
            support_path=args.support_json,
            output_path=args.output,
        )
        print(json.dumps({"report": str(path)}, indent=2))
    elif args.command == "report-policy":
        from .policy_report import render_policy_report

        name = args.output_name
        if name is None:
            md = str(args.model_dir)
            if "v4" in md:
                name = "policy_bc_v4.html"
            elif "v3" in md:
                name = "policy_bc_v3.html"
            else:
                name = "policy_bc_v2.html"
        path = render_policy_report(
            args.model_dir,
            Path(args.output_dir) / name,
        )
        print(json.dumps({"report": str(path)}, indent=2))
    elif args.command == "predict-policy":
        from .policy_infer import demo_predict_from_raw

        print(
            json.dumps(
                demo_predict_from_raw(
                    args.replay,
                    model_dir=args.model_dir,
                    card_costs_path=args.card_costs,
                    prefix_events=args.prefix_events,
                    acting_side=args.side,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
            )
        )
    elif args.command == "eval-matchups":
        from .matchup_eval import evaluate_matchups

        print(
            json.dumps(
                evaluate_matchups(
                    input_dir=args.input,
                    policy_dir=args.policy_dir,
                    winner_dir=args.winner_dir,
                    card_costs_path=args.card_costs,
                    output_path=args.output,
                    games_per_matchup=args.games,
                    top_k=args.top_k,
                    min_n=args.min_n,
                    seed=args.seed,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "eval-defense":
        from .defense_eval import evaluate_defense

        print(
            json.dumps(
                evaluate_defense(
                    policy_dir=args.policy_dir,
                    card_costs_path=args.card_costs,
                    output_path=args.output,
                    trials_per_scenario=args.trials,
                    seed=args.seed,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "eval-defense-slice":
        from .defense_slice_eval import evaluate_defense_slice

        print(
            json.dumps(
                evaluate_defense_slice(
                    input_dir=args.input,
                    policy_dir=args.policy_dir,
                    card_costs_path=args.card_costs,
                    output_path=args.output,
                    max_response_seconds=args.max_response_seconds,
                    seed=args.seed,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "report-defense-slice":
        from .defense_slice_report import render_defense_slice_report

        path = render_defense_slice_report(
            slice_path=args.slice_json,
            fair_probe_path=args.fair_json,
            output_path=args.output,
        )
        print(json.dumps({"report": str(path)}, indent=2))
    elif args.command == "audit-defense-support":
        from .defense_support_audit import audit_defense_support

        print(
            json.dumps(
                audit_defense_support(
                    input_dir=args.input,
                    policy_dir=args.policy_dir,
                    card_costs_path=args.card_costs,
                    output_path=args.output,
                    max_response_seconds=args.max_response_seconds,
                    seed=args.seed,
                    device_name=args.device,
                    score_model=not args.no_score_model,
                ),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "report-defense-support":
        from .defense_support_report import render_defense_support_report

        path = render_defense_support_report(
            audit_path=args.audit_json,
            output_path=args.output,
        )
        print(json.dumps({"report": str(path)}, indent=2))
    elif args.command == "placement-probe":
        from .placement_probe import run_placement_probe

        print(
            json.dumps(
                run_placement_probe(
                    input_dir=args.input,
                    policy_dir=args.policy_dir,
                    card_costs_path=args.card_costs,
                    output_json=args.output_json,
                    output_html=args.output_html,
                    epochs=args.epochs,
                    batch_size=args.batch_size,
                    probe_batch_size=args.probe_batch_size,
                    lr=args.lr,
                    hidden=args.hidden,
                    min_card_plays=args.min_card_plays,
                    seed=args.seed,
                    device_name=args.device,
                ),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "audit-hands":
        from .hand_audit import run_hand_audit

        print(
            json.dumps(
                run_hand_audit(
                    input_dir=args.input,
                    policy_dir=args.policy_dir,
                    card_costs_path=args.card_costs,
                    output_json=args.output_json,
                    output_html=args.output_html,
                    seed=args.seed,
                    device_name=args.device,
                    max_quality_battles=args.max_quality_battles,
                    allow_stale_cache=not args.no_stale_cache,
                    cache_path=args.cache,
                    quality_workers=args.quality_workers,
                ).get("verdict"),
                indent=2,
                ensure_ascii=False,
                default=str,
            )
        )
    elif args.command == "phone-lab":
        from .phone_lab import run_phone_lab

        try:
            run_phone_lab(
                host=args.host,
                port=args.port,
                pixel9=args.pixel9,
                pixel8=args.pixel8,
                calib_dir=args.calib_dir,
                yolo_model=args.yolo_model,
                card_costs=args.card_costs,
                policy_v3=args.policy_v3,
                policy_v4=args.policy_v4,
                open_browser=not args.no_open,
            )
        except RuntimeError as exc:
            print(f"phone-lab error: {exc}", file=sys.stderr)
            raise SystemExit(1) from exc
    elif args.command == "phone-lab-calibrate":
        from .phone_lab.place_calibrate import (
            run_place_calibrate,
            run_place_calibrate_both,
        )

        try:
            if args.phone.strip().lower() == "both":
                result = run_place_calibrate_both(
                    calib_dir=args.calib_dir,
                    pixel9=args.pixel9,
                    pixel8=args.pixel8,
                )
            else:
                result = run_place_calibrate(
                    phone=args.phone,
                    calib_dir=args.calib_dir,
                )
            print(json.dumps(result, indent=2, ensure_ascii=False, default=str))
        except (RuntimeError, FileNotFoundError) as exc:
            print(f"phone-lab-calibrate error: {exc}", file=sys.stderr)
            raise SystemExit(1) from exc


if __name__ == "__main__":
    main()
