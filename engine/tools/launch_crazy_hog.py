#!/usr/bin/env python3
"""Launch one Hog training job with aggregate desktop resource limits."""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("run_dir", type=Path)
    parser.add_argument("duration", type=int, nargs="?", default=10800)
    parser.add_argument("--name", default="banana-hog-3")
    parser.add_argument("--unit", default="banana-hog-3")
    parser.add_argument("--workers", type=int, choices=(1, 2, 3, 4, 5, 6), default=2)
    parser.add_argument("--engine", type=Path, help="Explicit validated engine artifact")
    parser.add_argument("--trim-checkpoint-json", action="store_true", help="Release duplicate host checkpoint allocation metadata")
    parser.add_argument("--memory-gib", type=int, choices=range(4, 9), help="Aggregate RAM cap; default scales with workers")
    parser.add_argument("--cpu-cores", type=int, choices=range(1, 7), help="Aggregate CPU quota; default scales with workers")
    parser.add_argument("--resume", action="store_true")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()
    if args.duration <= 0:
        parser.error("duration must be positive")
    if not args.unit or any(c not in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_.-" for c in args.unit):
        parser.error("unit must contain only letters, digits, dots, underscores, or hyphens")
    run = args.run_dir.resolve()
    if not args.resume and run.exists() and any(run.iterdir()):
        parser.error("run directory is not empty; use a fresh path or --resume")
    python = ROOT.parent / ".venv/bin/python"
    if not python.exists():
        parser.error(f"missing Python environment: {python}")
    # This budget covers the trainer AND every engine child in one cgroup.
    # Do not wrap individual engine children in independent systemd scopes.
    memory_gib = args.memory_gib or max(4, args.workers + 2)
    memory_high_mib = memory_gib * 1024 - 512
    cpu_cores = args.cpu_cores or max(2, args.workers)
    runtime_seconds = args.duration
    if args.resume:
        manifest = run / "manifest.json"
        if manifest.is_file():
            deadline = json.loads(manifest.read_text()).get("deadline")
            if deadline:
                runtime_seconds = int((datetime.fromisoformat(deadline.replace("Z", "+00:00")) - datetime.now(timezone.utc)).total_seconds())
                if runtime_seconds <= 0:
                    parser.error("the saved run deadline has passed")
    command = [
        "systemd-run", "--user", "--collect", f"--unit={args.unit}",
        "--service-type=exec",
        "-p", f"WorkingDirectory={ROOT}",
        "-p", f"MemoryHigh={memory_high_mib}M", "-p", f"MemoryMax={memory_gib}G", "-p", "MemorySwapMax=0",
        "-p", f"CPUQuota={cpu_cores * 100}%", "-p", "Nice=10", "-p", "OOMPolicy=kill",
        "-p", "KillMode=mixed", "-p", "TimeoutStopSec=60",
        "-p", f"RuntimeMaxSec={runtime_seconds + 180}", "-p", "Restart=no",
        "--setenv=PYTHONPATH=.:rl", "--setenv=OMP_NUM_THREADS=1",
        "--setenv=MKL_NUM_THREADS=1", "--setenv=OPENBLAS_NUM_THREADS=1",
        "flock", "--nonblock", "--no-fork",
        f"/run/user/{os.getuid()}/clash-hog-compute.lock",
        str(python), "-m", "rl.crazy_hog_train", "--run-dir", str(run),
        "--name", args.name, "--duration-seconds", str(args.duration),
        "--workers", str(args.workers), "--torch-threads", "1",
        "--minibatch-size", "32", "--max-snapshots", "2",
        "--state-bank-fraction", "0.15", "--snapshot-every", "10",
        "--snapshot-keep", "2", "--search", "none", "--sync-waves",
    ]
    if args.resume:
        command.append("--resume")
    if args.engine:
        if not args.engine.is_file():
            parser.error(f"missing engine: {args.engine}")
        command += ["--engine", str(args.engine.resolve()), "--allow-unpinned-engine"]
    if args.trim_checkpoint_json:
        if not args.engine:
            parser.error("--trim-checkpoint-json requires an explicit validated --engine")
        command.insert(command.index("flock"), "--setenv=CR_TRIM_CHECKPOINT_JSON=1")
    if args.dry_run:
        print(json.dumps(command, indent=2))
        return
    available = 0
    for line in Path("/proc/meminfo").read_text().splitlines():
        if line.startswith("MemAvailable:"):
            available = int(line.split()[1]) * 1024
    if available < (memory_gib + 2) * 2**30:
        parser.error(f"only {available / 2**30:.1f} GiB available; require {memory_gib + 2} GiB before starting a {memory_gib} GiB job")
    subprocess.run(command, check=True)
    print(f"Service: {args.unit}.service; RAM ≤{memory_gib} GiB, no swap, CPU ≤{cpu_cores} cores, workers={args.workers}")
    print(f"Monitor: cd {ROOT} && {python} -m rl.crazy_hog_monitor --run-dir {run}")
    print(f"Stop gracefully: touch {run / 'STOP'}")


if __name__ == "__main__":
    main()
