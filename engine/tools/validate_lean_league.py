#!/usr/bin/env python3
"""Replay the complete pinned regression directly under one outer cgroup.

Unlike replay_saved_league's independent worker scopes, every engine spawned
here inherits the caller's aggregate memory/CPU cap. No game generation or
regeneration is performed.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
import gzip
import json
import os
from pathlib import Path
import time

from league_validation import Engine, replay_game, sha

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--workers", type=int, choices=(1, 2, 4), default=2)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    base = ROOT / "analysis/fast-league-worktree-20261022"
    jobs = []
    for split in ("development", "holdout-final", "holdout-cycle"):
        directory = base / split
        manifest = json.loads((directory / "manifest.json").read_text())
        jobs += [(split, directory / f"{job['id']:04d}.json.gz") for job in manifest["jobs"]]
    started = time.monotonic()

    def worker(index):
        results = []
        engine = Engine(args.binary, args.out / f"worker-{index}.log")
        try:
            for split, path in jobs[index::args.workers]:
                with gzip.open(path, "rt") as f:
                    trace = json.load(f)
                result = replay_game(trace, engine)
                result.update(split=split, trace=path.name)
                results.append(result)
                (args.out / f"worker-{index}.json").write_text(json.dumps(results, indent=2) + "\n")
                if not result["matched"]:
                    raise AssertionError(str(result))
                if len(results) % 10 == 0:
                    elapsed = time.monotonic() - started
                    total = len(jobs[index::args.workers])
                    print(json.dumps({"worker": index, "done": len(results), "total": total,
                                      "eta_seconds": round(elapsed / len(results) * (total - len(results)), 1)}), flush=True)
        finally:
            engine.close()
        return results

    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        all_results = [result for rows in pool.map(worker, range(args.workers)) for result in rows]
    summary = {"binary_sha256": sha(args.binary), "environment": {key: os.getenv(key) for key in ("CR_TRIM_CHECKPOINT_JSON", "LD_PRELOAD", "MALLOC_CONF")},
               "games": len(all_results), "matched": sum(result["matched"] for result in all_results),
               "replies": sum(result["compared"] for result in all_results), "elapsed_seconds": time.monotonic() - started,
               "splits": {split: sum(result["matched"] for result in all_results if result["split"] == split)
                          for split in ("development", "holdout-final", "holdout-cycle")}}
    (args.out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary), flush=True)


if __name__ == "__main__":
    main()
