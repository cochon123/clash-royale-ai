#!/usr/bin/env python3
"""Replay saved league traces without writing into the trace directory.

Results go to --out. Each worker is a real engine under a 6G scope with no
swap. Workers default to 2.
"""
from __future__ import annotations

import argparse
import gzip
import json
import os
from pathlib import Path
import selectors
import subprocess
import time
import concurrent.futures as futures

from league_validation import replay_game, sha

ROOT = Path(__file__).resolve().parents[1]
CHECKPOINT = ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"


class ScopedEngine:
    def __init__(self, binary, log):
        self.err = Path(log).open("ab")
        self.proc = subprocess.Popen(
            ["systemd-run", "--user", "--scope", "--quiet",
             "-p", "MemoryMax=6G", "-p", "MemorySwapMax=0", "--",
             str(Path(binary).resolve()), "--checkpoint", str(CHECKPOINT)],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=self.err,
            bufsize=0, cwd=ROOT,
        )
        self.selector = selectors.DefaultSelector()
        self.selector.register(self.proc.stdout, selectors.EVENT_READ)
        self.pending = b""

    def request(self, obj):
        self.proc.stdin.write(json.dumps(obj, separators=(",", ":")).encode() + b"\n")
        end = time.monotonic() + 180
        while b"\n" not in self.pending:
            remaining = end - time.monotonic()
            if remaining <= 0 or not self.selector.select(remaining):
                raise TimeoutError("engine request timeout")
            data = os.read(self.proc.stdout.fileno(), 1 << 20)
            if not data:
                raise RuntimeError(f"engine exited: {self.proc.poll()}")
            self.pending += data
        line, self.pending = self.pending.split(b"\n", 1)
        reply = json.loads(line)
        if reply.get("ok") is not True:
            raise RuntimeError(str(reply.get("error", reply)))
        return reply["result"]

    def close(self):
        if self.proc.poll() is None:
            try:
                self.request({"op": "close"})
                self.proc.wait(timeout=20)
            except Exception:
                self.proc.kill()
                self.proc.wait(timeout=10)
        self.selector.close()
        self.err.close()


def replay_worker(binary, traces, jobs, out, worker):
    engine = ScopedEngine(binary, out / f"candidate-{worker}.log")
    results = []
    try:
        for job in jobs:
            path = traces / f"{job['id']:04d}.json.gz"
            started = time.monotonic()
            with gzip.open(path, "rt") as handle:
                trace = json.load(handle)
            result = {"id": job["id"], "trace_sha256": sha(path)}
            result.update(replay_game(trace, engine))
            result["total_seconds"] = time.monotonic() - started
            if not result["matched"]:
                engine.close()
                engine = ScopedEngine(binary, out / f"candidate-{worker}.log")
            results.append(result)
            print(json.dumps({k: result[k] for k in ("id", "matched", "compared", "total_seconds")}), flush=True)
    finally:
        engine.close()
    return results


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--traces", type=Path, required=True)
    parser.add_argument("--candidate", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--workers", type=int, default=2)
    args = parser.parse_args()
    if args.workers < 1 or args.workers > 2:
        raise SystemExit("workers must be 1 or 2")
    args.out.mkdir(parents=True, exist_ok=True)
    manifest = json.loads((args.traces / "manifest.json").read_text())
    jobs = manifest["jobs"]
    started = time.monotonic()
    results = []
    with futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
        tasks = [
            pool.submit(replay_worker, args.candidate, args.traces, jobs[i::args.workers], args.out, i)
            for i in range(args.workers)
        ]
        for task in futures.as_completed(tasks):
            results.extend(task.result())
    summary = {
        "split": manifest.get("split"),
        "traces": str(args.traces),
        "candidate_sha256": sha(args.candidate),
        "total_games": len(jobs),
        "fully_matched": sum(bool(row.get("matched")) for row in results),
        "requests_compared": sum(row.get("compared", 0) for row in results),
        "elapsed_seconds": time.monotonic() - started,
        "failures": [row for row in results if not row.get("matched")],
    }
    (args.out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps({k: summary[k] for k in summary if k != "failures"}, indent=2), flush=True)
    return 0 if summary["fully_matched"] == len(jobs) else 1


if __name__ == "__main__":
    raise SystemExit(main())
