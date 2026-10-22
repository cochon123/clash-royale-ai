#!/usr/bin/env python3
"""Run Banana RL training while sampling its process tree and whole-device GPU."""
from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import time
from collections import defaultdict
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_CHECKPOINT = ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"
DEFAULT_PYTHON = ROOT.parent / "clash-royale-ai/.venv/bin/python"
AVAILABLE_CPUS = len(os.sched_getaffinity(0)) if hasattr(os, "sched_getaffinity") else (os.cpu_count() or 1)


def process_tree(root_pid: int) -> list[int]:
    """Return currently visible descendants, including root_pid."""
    parents: dict[int, int] = {}
    for entry in Path("/proc").iterdir():
        if not entry.name.isdigit():
            continue
        try:
            raw = (entry / "stat").read_text()
            fields = raw[raw.rfind(")") + 2 :].split()
            parents[int(entry.name)] = int(fields[1])
        except (OSError, ValueError, IndexError):
            continue
    found = {root_pid}
    changed = True
    while changed:
        changed = False
        for pid, ppid in parents.items():
            if ppid in found and pid not in found:
                found.add(pid)
                changed = True
    return sorted(found)


def cpu_ticks(pids: list[int]) -> int:
    total = 0
    for pid in pids:
        try:
            raw = Path(f"/proc/{pid}/stat").read_text()
            fields = raw[raw.rfind(")") + 2 :].split()
            total += int(fields[11]) + int(fields[12])  # utime + stime
        except (OSError, ValueError, IndexError):
            pass
    return total


def gpu_sample() -> dict[str, int | None] | None:
    if not shutil.which("nvidia-smi"):
        return None
    try:
        result = subprocess.run(
            ["nvidia-smi", "--query-gpu=utilization.gpu,memory.used,memory.total",
             "--format=csv,noheader,nounits"],
            capture_output=True, text=True, timeout=3, check=True,
        )
        rows = []
        for line in result.stdout.splitlines():
            parts = [part.strip() for part in line.split(",")]
            if len(parts) != 3:
                continue
            rows.append([None if value in ("[Not Supported]", "N/A") else int(value) for value in parts])
        if not rows:
            return None
        # Report the first device; its utilization/memory is whole-device use,
        # including desktop and other services, not attributed to this runner.
        util, used, total = rows[0]
        return {"utilization_percent": util, "memory_used_mib": used, "memory_total_mib": total}
    except (OSError, subprocess.SubprocessError, ValueError):
        return None


def phase_for(run_dir: Path) -> str:
    try:
        state = json.loads((run_dir / "state.json").read_text())
        return "training" if state.get("status") == "running" else "preparing"
    except (OSError, json.JSONDecodeError):
        return "preparing"


def summarize(samples: list[dict[str, Any]]) -> dict[str, Any]:
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for sample in samples:
        grouped[sample["phase"]].append(sample)
    result: dict[str, Any] = {}
    for phase, rows in grouped.items():
        cpu = [row["cpu_cores"] for row in rows if row["cpu_cores"] is not None]
        gpu = [row["gpu"] for row in rows if row["gpu"] is not None]
        utils = [row["utilization_percent"] for row in gpu if row["utilization_percent"] is not None]
        memories = [row["memory_used_mib"] for row in gpu if row["memory_used_mib"] is not None]
        result[phase] = {
            "sample_count": len(rows),
            "cpu_average_cores": sum(cpu) / len(cpu) if cpu else None,
            "cpu_peak_cores": max(cpu) if cpu else None,
            "whole_device_gpu_average_utilization_percent": sum(utils) / len(utils) if utils else None,
            "whole_device_gpu_peak_utilization_percent": max(utils) if utils else None,
            "whole_device_gpu_average_memory_mib": sum(memories) / len(memories) if memories else None,
            "whole_device_gpu_peak_memory_mib": max(memories) if memories else None,
        }
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--seconds", type=float, default=120)
    parser.add_argument("--workers", type=int, default=max(1, min(16, AVAILABLE_CPUS - 2)))
    parser.add_argument("--device", default="cuda")
    parser.add_argument("--python", type=Path, default=DEFAULT_PYTHON)
    parser.add_argument("--checkpoint", type=Path, default=DEFAULT_CHECKPOINT)
    parser.add_argument("--trace-dir", type=Path)
    parser.add_argument("--fallback-bin", type=Path, help="optional original engine with exact-prefix recovery")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--engine-bin", type=Path, help="compiled runner (default: the trainer's default)")
    args = parser.parse_args()
    run_dir = args.run_dir if args.run_dir.is_absolute() else ROOT / args.run_dir
    output = args.output or ROOT / "analysis/cpu-gpu-tuning" / f"{run_dir.name}-compute.json"
    if not args.python.is_file():
        parser.error(f"Python executable does not exist: {args.python}")
    if not args.checkpoint.is_dir():
        parser.error(f"checkpoint directory does not exist: {args.checkpoint}")
    if run_dir.exists() and any(run_dir.iterdir()):
        parser.error("choose an empty run directory; existing artifacts are preserved")
    command = [str(args.python), "-m", "rl.banana_fast_train", "--run-dir", str(run_dir),
               "--checkpoint", str(args.checkpoint), "--duration-seconds", str(args.seconds),
               "--workers", str(args.workers), "--device", args.device,
               "--torch-threads", "2", "--eval-interval", "0"]
    if args.trace_dir:
        command.extend(["--trace-dir", str(args.trace_dir)])
    if args.fallback_bin:
        command.extend(["--fallback-bin", str(args.fallback_bin)])
    if args.engine_bin:
        command.extend(["--engine-bin", str(args.engine_bin.resolve())])

    started = time.monotonic()
    child = subprocess.Popen(command, cwd=ROOT, start_new_session=True)
    samples: list[dict[str, Any]] = []
    ticks_per_second = os.sysconf("SC_CLK_TCK")
    previous_time = time.monotonic()
    previous_ticks = cpu_ticks(process_tree(child.pid))
    interrupted = False
    try:
        while child.poll() is None:
            time.sleep(1)
            now = time.monotonic()
            current_ticks = cpu_ticks(process_tree(child.pid))
            cores = max(0, current_ticks - previous_ticks) / ticks_per_second / max(now - previous_time, 1e-6)
            samples.append({"elapsed_seconds": now - started, "phase": phase_for(run_dir),
                            "cpu_cores": cores, "gpu": gpu_sample()})
            previous_time, previous_ticks = now, current_ticks
    except KeyboardInterrupt:
        interrupted = True
        (run_dir / "STOP").parent.mkdir(parents=True, exist_ok=True)
        (run_dir / "STOP").write_text("compute benchmark interrupted\n")
        while child.poll() is None:
            try:
                time.sleep(1)
            except KeyboardInterrupt:
                continue

    state: dict[str, Any] = {}
    try:
        state = json.loads((run_dir / "state.json").read_text())
    except (OSError, json.JSONDecodeError):
        pass
    elapsed = time.monotonic() - started
    report = {
        "format": "banana-compute-bench-v1", "command": command,
        "run_dir": str(run_dir), "started_unix": time.time() - elapsed,
        "elapsed_wall_seconds": elapsed, "interrupted": interrupted,
        "exit_code": child.returncode, "final_state": state,
        "rates": {
            "trainer_elapsed_seconds": state.get("elapsed_seconds"),
            "decision_steps": state.get("decision_steps"),
            "games": state.get("completed_games"),
            "truncated_games": state.get("truncated_games"),
            "decisions_per_second": (state.get("decision_steps", 0) / state["elapsed_seconds"]
                                      if state.get("elapsed_seconds", 0) else None),
            "games_per_hour": (state.get("completed_games", 0)
                               * 3600 / state["elapsed_seconds"] if state.get("elapsed_seconds", 0) else None),
        },
        "phase_summary": summarize(samples), "samples": samples,
        "gpu_scope": "Whole first NVIDIA device, including desktop and other services; not process-attributed.",
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(report, indent=2, allow_nan=False) + "\n")
    print(f"Compute report: {output}", flush=True)
    return 130 if interrupted else int(child.returncode or 0)


if __name__ == "__main__":
    raise SystemExit(main())
