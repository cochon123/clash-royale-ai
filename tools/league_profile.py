#!/usr/bin/env python3
"""Collect original-engine basic-block profiles by replaying recorded traces.

Coverage discovery only: requests are the recorded requests of a development
trace, so the reference executes exactly the path it executed while recording.
The resulting profile is a generation input for tools/native_rust_codegen.py.
"""
from __future__ import annotations
import argparse
import concurrent.futures as futures
import gzip
import json
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))

from league_validation import CHECKPOINT, Engine, ROOT


def profile_trace(path, reference_binary, out_dir):
    trace = json.load(gzip.open(path, "rt"))
    worker = f"{path.stem}-{path.stat().st_mtime_ns}"
    engine = Engine(reference_binary, out_dir / f"profile-{path.stem}.log")
    try:
        records = trace["records"]
        engine.request(records[0]["request"])  # reset
        engine.request({"op": "profile_start"})
        for item in records[1:]:
            engine.request(item["request"])
        profile = engine.request({"op": "profile_finish"})
    finally:
        engine.close()
    profile["trace"] = str(path)
    profile["trace_sha256"] = __import__("hashlib").sha256(path.read_bytes()).hexdigest()
    target = out_dir / f"{path.stem}.json"
    target.write_text(json.dumps(profile, separators=(",", ":")) + "\n")
    return {"trace": path.name, "blocks": len(profile["blocks"]), "game": trace["job"]["id"]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reference", type=Path, required=True)
    parser.add_argument("--traces", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--workers", type=int, default=6)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    paths = sorted(args.traces.glob("[0-9][0-9][0-9][0-9].json.gz"))
    if not paths:
        raise SystemExit(f"no traces under {args.traces}")
    results = []
    with futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
        jobs = {pool.submit(profile_trace, path, args.reference, args.out): path for path in paths}
        for job in futures.as_completed(jobs):
            result = job.result()
            results.append(result)
            print(json.dumps(result), flush=True)
    print(json.dumps({"traces": len(results), "out": str(args.out)}), flush=True)


if __name__ == "__main__":
    main()
