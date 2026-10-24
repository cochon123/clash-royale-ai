"""Train one banana-{deck}-1 specialist per supported deck until 06:00.

Hog 2.6 already has banana-hog-1, so this night skips hog-mirror. Each other
deck in the opponent pool gets the same tower-aware PPO recipe: warm start
from banana-1, original-engine opponents, eight workers, two step groups.
Wall-clock is split evenly across the decks that are still unfinished.
"""
from __future__ import annotations

import argparse
import datetime as dt
import json
import os
import signal
import subprocess
import sys
import time
from pathlib import Path
from zoneinfo import ZoneInfo

ROOT = Path(__file__).resolve().parents[1]
MAIN_RUNS = Path("/home/cochon/Documents/mini-project/clash-royale-ai/engine/runs")
NIGHT_DIR = MAIN_RUNS / "banana-deck-night-20261022"
POOL = ROOT / "data/hog2_opponent_pool.json"
PARENT = MAIN_RUNS / "banana-1/base.pt"
ENGINE = MAIN_RUNS / "banana-hog-overnight-20261022/bin/league_reference"
CHECKPOINT = Path("/home/cochon/Documents/mini-project/clash-royale-ai/engine/analysis/native-pipeline/oracle-export-01/stage01-attempt001/state")
ZONE = ZoneInfo("America/Edmonton")
SKIP_DECKS = {"hog-mirror"}
WORKERS = 8
STEP_GROUPS = 2
DASHBOARD_PORT = 8784


def model_name(deck_id: str) -> str:
    return f"banana-{deck_id}-1"


def training_decks(pool: dict | None = None) -> list[str]:
    payload = pool if pool is not None else json.loads(POOL.read_text())
    return [deck["id"] for deck in payload["decks"] if deck["id"] not in SKIP_DECKS]


def edmonton_morning(now: dt.datetime | None = None) -> dt.datetime:
    local = (now or dt.datetime.now(ZONE)).astimezone(ZONE)
    target = local.replace(hour=6, minute=0, second=0, microsecond=0)
    if target <= local:
        target += dt.timedelta(days=1)
    return target


def memory_available_gib() -> float | None:
    try:
        for line in Path("/proc/meminfo").read_text().splitlines():
            if line.startswith("MemAvailable:"):
                return int(line.split()[1]) / 1024 / 1024
    except OSError:
        return None
    return None


def read_json(path: Path) -> dict | None:
    try:
        value = json.loads(path.read_text())
    except (OSError, json.JSONDecodeError):
        return None
    return value if isinstance(value, dict) else None


def eta_text(seconds: float) -> str:
    seconds = max(0, int(seconds))
    return f"{seconds // 3600}h{(seconds % 3600) // 60:02d}m"


def run_dir_for(deck_id: str) -> Path:
    return MAIN_RUNS / model_name(deck_id)


def snapshot(decks: list[str], deadline: dt.datetime, current: str | None) -> dict:
    finished = [deck for deck in decks if (run_dir_for(deck) / "models/final.pt").exists()]
    state = read_json(run_dir_for(current) / "state.json") if current else None
    counters = (state or {}).get("counters") or {}
    updates = int(counters.get("updates") or 0)
    games = int(counters.get("games") or 0)
    phase = (state or {}).get("phase") or "starting"
    status = (state or {}).get("status") or ("idle" if current is None else "starting")
    remaining = max(0, deadline.timestamp() - time.time())
    work = f"{len(finished)}/{len(decks)} decks"
    if current:
        work += f" · {current} update {updates} · {games} games"
    return {
        "deadline": deadline.isoformat(),
        "deadline_local": deadline.strftime("%Y-%m-%d %H:%M America/Edmonton"),
        "decks_total": len(decks),
        "decks_finished": len(finished),
        "finished": [model_name(deck) for deck in finished],
        "current_deck": current,
        "current_model": model_name(current) if current else None,
        "updates": updates,
        "games": games,
        "training_games": int(counters.get("training_games") or 0),
        "phase": phase,
        "status": status,
        "work_done": work,
        "eta_seconds": int(remaining),
        "eta": eta_text(remaining),
        "memory_available_gib": memory_available_gib(),
        "updated_at": dt.datetime.now(ZONE).isoformat(),
    }


def publish(progress: dict) -> None:
    NIGHT_DIR.mkdir(parents=True, exist_ok=True)
    target = NIGHT_DIR / "progress.json"
    temporary = target.with_suffix(".json.tmp")
    temporary.write_text(json.dumps(progress, indent=2) + "\n")
    os.replace(temporary, target)


def watch(interval: float) -> None:
    print(f"{'TIME':<10} {'WORK_DONE':<62} {'ETA':<8} {'RAM_AVAIL':<12} STATUS", flush=True)
    while True:
        progress = read_json(NIGHT_DIR / "progress.json") or {}
        ram = progress.get("memory_available_gib")
        ram_text = f"{ram:.1f} GiB" if isinstance(ram, (int, float)) else "—"
        print(
            f"{dt.datetime.now(ZONE):%H:%M:%S}  "
            f"{str(progress.get('work_done') or 'waiting for the night supervisor'):<62} "
            f"{str(progress.get('eta') or '—'):<8} {ram_text:<12} "
            f"{progress.get('status') or '—'} · {progress.get('phase') or '—'}",
            flush=True,
        )
        if progress.get("status") in {"completed", "stopped"} and progress.get("current_deck") is None:
            return
        time.sleep(interval)


def launch_command(deck_id: str, deadline: dt.datetime, duration: float, seed: int, resume: bool) -> list[str]:
    # Resume locks duration, seed, and learner deck. Reuse the original
    # command and only move the absolute deadline forward or back.
    saved = read_json(run_dir_for(deck_id) / "launch.json") if resume else None
    if saved and isinstance(saved.get("command"), list):
        command = [str(part) for part in saved["command"] if part != "--resume"]
        if "--deadline" in command:
            command[command.index("--deadline") + 1] = deadline.isoformat()
        command.append("--resume")
        return command
    return [
        sys.executable, "-u", "-m", "rl.banana_hog_adaptive",
        "--run-dir", str(run_dir_for(deck_id)),
        "--name", model_name(deck_id),
        "--learner-deck", deck_id,
        "--parent", str(PARENT),
        "--parent-name", "banana-1",
        "--opponents", "original",
        "--engine", str(ENGINE),
        "--checkpoint", str(CHECKPOINT),
        "--duration-seconds", f"{duration:.3f}",
        "--deadline", deadline.isoformat(),
        "--workers", str(WORKERS),
        "--step-groups", str(STEP_GROUPS),
        "--device", "cuda",
        "--eval-every", "2700",
        "--eval-seeds", "1",
        "--confirmation-seeds", "1",
        "--seed", str(seed),
    ]


def start_dashboard(deck_id: str) -> subprocess.Popen | None:
    run_dir = run_dir_for(deck_id)
    if not run_dir.is_dir():
        return None
    try:
        return subprocess.Popen(
            [sys.executable, "-u", "-m", "rl.hog2_live", "--run-dir", str(run_dir),
             "--host", "127.0.0.1", "--port", str(DASHBOARD_PORT)],
            cwd=ROOT,
        )
    except OSError as exc:
        print(f"dashboard did not start: {exc}", flush=True)
        return None


def stop_process(proc: subprocess.Popen | None) -> None:
    if proc is None or proc.poll() is not None:
        return
    proc.send_signal(signal.SIGTERM)
    try:
        proc.wait(timeout=20)
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.wait(timeout=10)


def train_one(deck_id: str, deadline: dt.datetime, index: int, stop: list[bool],
              decks: list[str], night_deadline: dt.datetime) -> str:
    """Return finished, failed, or stopped."""
    run_dir = run_dir_for(deck_id)
    run_dir.mkdir(parents=True, exist_ok=True)
    resume = (run_dir / "models/latest.pt").exists()
    duration = max(1.0, deadline.timestamp() - time.time())
    command = launch_command(deck_id, deadline, duration, 20261023 + index, resume)
    (run_dir / "launch.json").write_text(json.dumps({
        "name": model_name(deck_id), "deck": deck_id, "command": command,
        "deadline": deadline.isoformat(), "started_at": dt.datetime.now(ZONE).isoformat(),
    }, indent=2) + "\n")
    print(" ".join(command), flush=True)
    child = subprocess.Popen(command, cwd=ROOT)
    dashboard = start_dashboard(deck_id)
    handed_off = False
    try:
        while child.poll() is None:
            publish(snapshot(decks, night_deadline, deck_id))
            if stop[0] or (NIGHT_DIR / "STOP").exists():
                stop[0] = True
                child.send_signal(signal.SIGTERM)
                break
            state = read_json(run_dir / "state.json") or {}
            phase = str(state.get("phase") or "")
            if (run_dir / "models/final.pt").exists() and "waiting for" in phase:
                handed_off = True
                child.send_signal(signal.SIGTERM)
                break
            time.sleep(5)
        try:
            code = child.wait(timeout=120)
        except subprocess.TimeoutExpired:
            child.kill()
            code = child.wait(timeout=20)
    finally:
        stop_process(dashboard)
    if stop[0]:
        return "stopped"
    if (run_dir / "models/final.pt").exists() and (code == 0 or handed_off):
        return "finished"
    print(f"{model_name(deck_id)} exited {code} without a confirmed handoff", flush=True)
    return "failed"


def run_night() -> None:
    for path in (PARENT, ENGINE, POOL, CHECKPOINT):
        if not path.exists():
            raise SystemExit(f"missing training input: {path}")
    decks = training_decks()
    deadline = edmonton_morning()
    NIGHT_DIR.mkdir(parents=True, exist_ok=True)
    stop = [False]

    def request_stop(_signum, _frame):
        stop[0] = True

    signal.signal(signal.SIGTERM, request_stop)
    signal.signal(signal.SIGINT, request_stop)
    attempts: dict[str, int] = {}
    failed: list[str] = []
    print(f"Night ends {deadline.isoformat()} · {len(decks)} decks · workers {WORKERS} · step-groups {STEP_GROUPS}", flush=True)
    while not stop[0] and time.time() < deadline.timestamp() - 60:
        pending = [deck for deck in decks if deck not in failed and not (run_dir_for(deck) / "models/final.pt").exists()]
        if not pending:
            break
        if (NIGHT_DIR / "STOP").exists():
            break
        deck = pending[0]
        share = max(1.0, (deadline.timestamp() - time.time()) / len(pending))
        slice_end = dt.datetime.fromtimestamp(time.time() + share, dt.timezone.utc).astimezone(ZONE)
        publish(snapshot(decks, deadline, deck))
        outcome = train_one(deck, slice_end, decks.index(deck), stop, decks, deadline)
        publish(snapshot(decks, deadline, deck if outcome == "failed" else None))
        if outcome == "stopped":
            break
        if outcome == "failed":
            attempts[deck] = attempts.get(deck, 0) + 1
            if attempts[deck] >= 2:
                failed.append(deck)
                print(f"skipping {model_name(deck)} after {attempts[deck]} failures", flush=True)
    final = snapshot(decks, deadline, None)
    final["status"] = "stopped" if stop[0] or (NIGHT_DIR / "STOP").exists() else "completed"
    final["phase"] = "night finished"
    final["failed"] = [model_name(deck) for deck in failed]
    publish(final)
    print(final["work_done"] + " · " + final["status"], flush=True)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--watch", action="store_true", help="Print work done and ETA until the night ends")
    parser.add_argument("--interval", type=float, default=5)
    args = parser.parse_args()
    if args.watch:
        watch(args.interval)
        return
    run_night()


if __name__ == "__main__":
    main()
