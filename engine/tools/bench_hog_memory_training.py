#!/usr/bin/env python3
"""Paired training memory trials from a frozen latest checkpoint.

The production run must be stopped first. Each trial is a disposable resume
clone, runs sequentially in a capped service, and never writes production
models. Baseline and jemalloc-4 use identical workers, weights, RNG and sampler.
"""
from __future__ import annotations

import argparse
import hashlib
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
JEMALLOC = "narenas:2,dirty_decay_ms:0,muzzy_decay_ms:0,background_thread:false"


def properties(unit):
    result = subprocess.run(["systemctl", "--user", "show", unit, "-p", "ActiveState",
                             "-p", "ControlGroup", "-p", "ExecMainStatus", "-p", "Result"],
                            capture_output=True, text=True, check=False)
    return dict(line.split("=", 1) for line in result.stdout.splitlines() if "=" in line)


def read(path):
    try:
        return json.loads(path.read_text())
    except (OSError, ValueError):
        return {}


def trial(source, out, name, waves):
    workers = 6 if name.endswith("6") else 4
    tuned = name.startswith("jemalloc")
    lean = name.startswith("lean")
    maximum = 7 if workers == 6 else 6
    target = out / name
    if target.exists():
        raise RuntimeError(f"trial directory already exists: {target}")
    (target / "models").mkdir(parents=True)
    manifest = read(source / "manifest.json")
    baseline = read(source / "state.json")["counters"]
    manifest["args"]["workers"] = workers
    manifest["args"]["run_dir"] = str(target)
    if lean:
        binary = ROOT / "analysis-local/lean-host/rust_league_rl"
        manifest["engine"]["path"] = str(binary)
        manifest["engine"]["sha256"] = hashlib.sha256(binary.read_bytes()).hexdigest()
        manifest["args"]["engine"] = str(binary)
        manifest["args"]["allow_unpinned_engine"] = True
    (target / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    for file in ("latest.pt", "best.pt"):
        shutil.copy2(source / "models" / file, target / "models" / file)
    if (source / "models/snapshots").exists():
        shutil.copytree(source / "models/snapshots", target / "models/snapshots")
    (target / "notes.md").write_text(f"# Disposable {name} memory trial\n\nResume clone, not the production model.\n")
    unit = "hog-memory-training-" + name
    command = ["systemd-run", "--user", f"--unit={unit}", "--collect", "--service-type=exec",
               "-p", f"WorkingDirectory={ROOT}", "-p", f"MemoryHigh={maximum * 1024 - 512}M",
               "-p", f"MemoryMax={maximum}G", "-p", "MemorySwapMax=0", "-p", "CPUQuota=400%",
               "-p", "Nice=10", "-p", "OOMPolicy=kill", "-p", "KillMode=mixed",
               "-p", "TimeoutStopSec=30", "-p", "RuntimeMaxSec=300", "-p", "Restart=no",
               "--setenv=PYTHONPATH=.:rl", "--setenv=OMP_NUM_THREADS=1",
               "--setenv=MKL_NUM_THREADS=1", "--setenv=OPENBLAS_NUM_THREADS=1"]
    if tuned:
        command += ["--setenv=LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so.2",
                    f"--setenv=MALLOC_CONF={JEMALLOC + ',dss:primary,retain:false' if 'dss' in name else JEMALLOC}"]
    if lean:
        command.append("--setenv=CR_TRIM_CHECKPOINT_JSON=1")
    command += [str(ROOT.parent / ".venv/bin/python"), "-m", "rl.crazy_hog_train",
                "--run-dir", str(target), "--name", manifest["name"], "--resume",
                "--duration-seconds", "10800", "--workers", str(workers), "--torch-threads", "1",
                "--minibatch-size", "32", "--max-snapshots", "2", "--state-bank-fraction", "0.15",
                "--snapshot-every", "10", "--snapshot-keep", "2", "--search", "none", "--sync-waves",
                "--max-waves", str(int(baseline["updates"]) + waves)]
    if lean:
        command += ["--engine", str(binary), "--allow-unpinned-engine"]
    begin = time.monotonic()
    subprocess.run(command, check=True, capture_output=True, text=True)
    peak = 0
    events = {}
    iterations = 0
    while time.monotonic() - begin < 340:
        unit_state = properties(unit)
        cg = Path("/sys/fs/cgroup") / unit_state.get("ControlGroup", "").lstrip("/")
        try:
            peak = max(peak, int((cg / "memory.peak").read_text()))
            events = dict(line.split() for line in (cg / "memory.events").read_text().splitlines())
        except (OSError, ValueError):
            pass
        state = read(target / "state.json")
        counters = state.get("counters", {})
        updates = max(0, int(counters.get("updates", 0)) - int(baseline["updates"]))
        elapsed = time.monotonic() - begin
        if iterations % 5 == 0:
            print(json.dumps({"case": name, "updates_done": updates, "updates_total": waves,
                              "eta_seconds": round(elapsed / updates * (waves - updates), 1) if updates else None,
                              "memory_peak_gib": round(peak / 2**30, 2), "phase": state.get("phase")}), flush=True)
        if unit_state.get("ActiveState") not in {"active", "activating", "deactivating"}:
            break
        iterations += 1
        time.sleep(2)
    else:
        subprocess.run(["systemctl", "--user", "stop", unit], check=False)
        raise TimeoutError(name)
    state = read(target / "state.json")
    if state.get("status") != "completed":
        raise RuntimeError(f"{name} failed: {state.get('phase')}; {events}")
    elapsed = time.monotonic() - begin
    counters = state["counters"]
    games = int(counters.get("training_games", 0)) - int(baseline.get("training_games", 0))
    banks = int(counters.get("bank_episodes", 0)) - int(baseline.get("bank_episodes", 0))
    decisions = int(counters.get("decisions", 0)) - int(baseline.get("decisions", 0))
    updates = int(counters.get("updates", 0)) - int(baseline.get("updates", 0))
    if updates != waves:
        raise RuntimeError(f"{name}: expected {waves} updates, got {updates}")
    voids = int(counters.get("voids", 0)) - int(baseline.get("voids", 0))
    audits = [json.loads(line) for line in (target / "reward-audit.jsonl").read_text().splitlines()]
    if voids or any(not row.get("ok") for row in audits):
        raise RuntimeError(f"{name}: invalid trial: {voids} new VOID games or a failed reward audit")
    result = {"case": name, "workers": workers, "jemalloc": tuned, "lean_checkpoint": lean, "elapsed_seconds": elapsed,
              "full_training_games": games, "bank_episodes": banks, "updates": updates,
              "training_episodes_per_minute": (games + banks) * 60 / elapsed,
              "decisions_per_second": decisions / elapsed, "memory_peak_bytes": peak,
              "reward_audits_passed": len(audits), "voids": voids,
              "memory_events": events, "at": datetime.now(timezone.utc).isoformat()}
    (target / "benchmark.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result), flush=True)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--waves", type=int, default=8)
    parser.add_argument("--cases", nargs="+", choices=("baseline4", "jemalloc4", "jemalloc6", "jemalloc_dss4", "jemalloc_dss6", "lean4", "lean6"),
                        default=("baseline4", "jemalloc4", "jemalloc6"))
    args = parser.parse_args()
    if properties("banana-hog-3.service").get("ActiveState") == "active":
        raise RuntimeError("Stop production training before memory trials")
    args.out.mkdir(parents=True, exist_ok=True)
    results = []
    for case in args.cases:
        result = trial(args.source, args.out, case, args.waves)
        results.append(result)
        (args.out / "results.json").write_text(json.dumps(results, indent=2) + "\n")


if __name__ == "__main__":
    main()
