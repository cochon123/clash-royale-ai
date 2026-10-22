#!/usr/bin/env python3
"""Compare warm independent engine fleets on the same complete game replays."""
import argparse
import concurrent.futures
import gzip
import json
import os
from pathlib import Path
import platform
import queue
import time

from league_validation import Engine, replay_game, sha


def measure(binary, traces, args, label):
    engines = []
    tasks = queue.Queue()
    for repeat in range(args.repeats):
        for trace in traces:
            tasks.put((repeat, trace))
    try:
        for index in range(args.workers):
            engine = Engine(binary, args.out / f"{label}-{index}.log")
            engines.append(engine)
            if args.cpus:
                os.sched_setaffinity(engine.proc.pid, {args.cpus[index]})
            engine.request(traces[0]["records"][0]["request"])

        def work(index):
            rows = []
            while True:
                try:
                    repeat, trace = tasks.get_nowait()
                except queue.Empty:
                    return rows
                result = replay_game(trace, engines[index])
                if not result["matched"]:
                    raise RuntimeError(f"{label} game {trace['job']['id']} differs: {result}")
                rows.append(dict(worker=index, sample=repeat, game=trace["job"]["id"],
                                 seconds=result["seconds"], requests=result["compared"],
                                 ticks=trace["terminal_tick"]))

        with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
            start = time.perf_counter()
            jobs = [pool.submit(work, index) for index in range(args.workers)]
            rows = [row for job in jobs for row in job.result()]
            elapsed = time.perf_counter()-start
        return dict(games=len(rows), wall_seconds=elapsed, games_per_minute=60*len(rows)/elapsed,
                    requests=sum(r["requests"] for r in rows), rows=rows)
    finally:
        for engine in engines:
            engine.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reference", type=Path, required=True)
    parser.add_argument("--candidate", type=Path, required=True)
    parser.add_argument("--traces", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--workers", type=int, default=6)
    parser.add_argument("--repeats", type=int, default=6)
    parser.add_argument("--rounds", type=int, default=2)
    parser.add_argument("--ids", type=int, nargs="+", default=[5, 11, 17, 23, 29, 30])
    parser.add_argument("--cpus", type=int, nargs="+", help="one logical CPU per worker; omit to use the scheduler")
    args = parser.parse_args()
    if min(args.workers, args.repeats, args.rounds) < 1:
        parser.error("workers, repeats and rounds must be positive")
    if args.cpus and (len(args.cpus) != args.workers or len(set(args.cpus)) != args.workers):
        parser.error("--cpus needs one distinct logical CPU per worker")
    args.out.mkdir(parents=True, exist_ok=True)
    traces = []
    for game in args.ids:
        with gzip.open(args.traces / f"{game:04d}.json.gz", "rt") as handle:
            traces.append(json.load(handle))
    hashes = {name: sha(getattr(args, name)) for name in ("reference", "candidate")}
    measurements = []
    for repeat in range(args.rounds):
        for name in (("reference", "candidate") if repeat % 2 == 0 else ("candidate", "reference")):
            result = dict(engine=name, round=repeat, **measure(getattr(args, name), traces, args, f"{name}-{repeat}"))
            measurements.append(result)
            print(json.dumps({k: v for k, v in result.items() if k != "rows"}), flush=True)
    assert hashes == {name: sha(getattr(args, name)) for name in hashes}, "binary changed"
    summaries = {}
    for name in hashes:
        selected = [r for r in measurements if r["engine"] == name]
        games, elapsed = sum(r["games"] for r in selected), sum(r["wall_seconds"] for r in selected)
        summaries[name] = dict(games=games, wall_seconds=elapsed, games_per_minute=60*games/elapsed)
    report = dict(binary_sha256=hashes, workers=args.workers, engine_cpus=args.cpus,
                  hardware=platform.uname()._asdict(), rounds=args.rounds, repeats=args.repeats,
                  trace_ids=args.ids, trace_sha256={str(i): sha(args.traces / f"{i:04d}.json.gz") for i in args.ids},
                  summaries=summaries, speedup=summaries["candidate"]["games_per_minute"]/summaries["reference"]["games_per_minute"],
                  measurements=measurements,
                  scope="Complete exact replays in warm persistent processes. Includes reset, legal masks, actions, observations, HP, JSON IPC and work scheduling; excludes startup, warm reset, shutdown and neural inference. Fleets run sequentially, alternating order by round.")
    (args.out / "report.json").write_text(json.dumps(report, indent=2)+"\n")
    print(json.dumps({k: v for k, v in report.items() if k != "measurements"}, indent=2))


if __name__ == "__main__":
    main()
