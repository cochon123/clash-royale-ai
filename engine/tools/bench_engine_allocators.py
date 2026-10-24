#!/usr/bin/env python3
"""Compare allocator configurations on identical saved native-engine traces.

Run under one aggregate systemd memory/CPU cgroup with training paused.
Uses the unchanged pinned PGO binary. Every saved semantic reply must match;
additional snapshot/restore probes must leave subsequent replies unchanged.
"""
from __future__ import annotations

import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import selectors
import re
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
TIMING = {"reset_seconds", "step_seconds", "query_seconds"}
CASES = {
    "baseline": {},
    "glibc_tuned": {"MALLOC_ARENA_MAX": "2", "MALLOC_TRIM_THRESHOLD_": "65536", "MALLOC_MMAP_THRESHOLD_": "65536"},
    "jemalloc": {"LD_PRELOAD": "/usr/lib/x86_64-linux-gnu/libjemalloc.so.2", "MALLOC_CONF": "narenas:2,dirty_decay_ms:0,muzzy_decay_ms:0,background_thread:false"},
    "jemalloc_balanced": {"LD_PRELOAD": "/usr/lib/x86_64-linux-gnu/libjemalloc.so.2", "MALLOC_CONF": "narenas:2,dirty_decay_ms:1000,muzzy_decay_ms:1000,background_thread:true"},
    "jemalloc_dss": {"LD_PRELOAD": "/usr/lib/x86_64-linux-gnu/libjemalloc.so.2", "MALLOC_CONF": "narenas:2,dirty_decay_ms:0,muzzy_decay_ms:0,background_thread:false,dss:primary,retain:false"},
    "lean_checkpoint": {"CR_TRIM_CHECKPOINT_JSON": "1"},
}


def semantic(value):
    if isinstance(value, dict):
        return {key: semantic(item) for key, item in value.items() if key not in TIMING}
    if isinstance(value, list):
        return [semantic(item) for item in value]
    return value


def memory(pid):
    out = {}
    for name in ("status", "smaps_rollup"):
        for line in Path(f"/proc/{pid}/{name}").read_text().splitlines():
            if ":" in line:
                key, val = line.split(":", 1)
                if key in {"Rss", "Pss", "Pss_Anon", "VmHWM"}:
                    out[key + "_bytes"] = int(val.split()[0]) * 1024
    return out


class Engine:
    def __init__(self, binary, env, out):
        self.err = (out / "stderr.log").open("ab")
        self.proc = subprocess.Popen(
            [str(binary), "--checkpoint",
             str(ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"),
             "--max-snapshots", "2"], env=env, cwd=ROOT,
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=self.err, bufsize=0)
        self.sel = selectors.DefaultSelector()
        self.sel.register(self.proc.stdout, selectors.EVENT_READ)
        self.pending = b""

    def request(self, request):
        self.proc.stdin.write(json.dumps(request, separators=(",", ":")).encode() + b"\n")
        end = time.monotonic() + 90
        while b"\n" not in self.pending:
            wait = end - time.monotonic()
            if wait <= 0 or not self.sel.select(wait):
                raise TimeoutError("engine request timed out")
            chunk = os.read(self.proc.stdout.fileno(), 1 << 20)
            if not chunk:
                raise RuntimeError(f"engine exited: {self.proc.poll()}")
            self.pending += chunk
        line, self.pending = self.pending.split(b"\n", 1)
        reply = json.loads(line)
        if not reply.get("ok"):
            raise RuntimeError(str(reply))
        return reply["result"]

    def close(self):
        if self.proc.poll() is None:
            self.proc.terminate()
            try:
                self.proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.proc.kill()
                self.proc.wait(timeout=5)
        self.sel.close()
        self.proc.stdin.close()
        self.proc.stdout.close()
        self.err.close()


def snapshot_digest(path):
    digest, semantic_digest = hashlib.sha256(), hashlib.sha256()
    with path.open("rb") as f:
        prefix = f.read(16)
        header = f.read(int.from_bytes(prefix[8:], "little"))
        normalized, count = re.subn(rb'"binary_sha256":"[0-9a-f]{64}"',
                                    b'"binary_sha256":"' + b'0' * 64 + b'"', header)
        if count != 1:
            raise ValueError("snapshot header does not contain one executable hash")
        digest.update(prefix + header)
        semantic_digest.update(prefix + normalized)
        for block in iter(lambda: f.read(1024 * 1024), b""):
            digest.update(block)
            semantic_digest.update(block)
    return digest.hexdigest(), semantic_digest.hexdigest()


def run_case(name, traces, out, binary):
    out.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ)
    for key in {key for values in CASES.values() for key in values}:
        env.pop(key, None)
    env.update(CASES[name])
    engine = Engine(binary, env, out)
    started = time.monotonic()
    snapshots, samples, captures, restores = [], [], [], []
    matched = 0
    replies = 0
    try:
        for index, path in enumerate(traces):
            with gzip.open(path, "rt") as f:
                trace = json.load(f)
            for offset, record in enumerate(trace["records"]):
                result = engine.request(record["request"])
                if semantic(result) != record["expected"]:
                    raise AssertionError(f"{name}: mismatch trace={path.name} reply={offset}")
                replies += 1
                if offset in (6, 40):
                    if len(snapshots) >= 2:
                        engine.request({"op": "drop_snapshot", "snapshot_id": snapshots.pop(0)})
                    begin = time.monotonic()
                    image = engine.request({"op": "snapshot"})
                    captures.append(time.monotonic() - begin)
                    snapshots.append(image["snapshot_id"])
                    observation = engine.request({"op": "observe"})
                    begin = time.monotonic()
                    restored = engine.request({"op": "restore", "snapshot_id": image["snapshot_id"]})
                    restores.append(time.monotonic() - begin)
                    if semantic(observation) != semantic(restored):
                        raise AssertionError(f"{name}: restore differs in {path.name}")
                    samples.append(memory(engine.proc.pid))
            matched += 1
            samples.append(memory(engine.proc.pid))
            elapsed = time.monotonic() - started
            print(json.dumps({"case": name, "done_games": matched, "total_games": len(traces),
                              "eta_seconds": round(elapsed / matched * (len(traces) - matched), 1),
                              "pss_mib": round(samples[-1]["Pss_bytes"] / 2**20, 1)}), flush=True)
        elapsed = time.monotonic() - started
        result = {"case": name, "environment": CASES[name], "games": matched, "replies_matched": replies,
                  "snapshot_restores_matched": len(restores), "elapsed_seconds": elapsed,
                  "games_per_minute": matched * 60 / elapsed,
                  "mean_capture_seconds": sum(captures) / len(captures),
                  "mean_restore_seconds": sum(restores) / len(restores),
                  "steady_pss_bytes": samples[-1]["Pss_bytes"],
                  "max_sampled_pss_bytes": max(row["Pss_bytes"] for row in samples),
                  "peak_rss_bytes": memory(engine.proc.pid)["VmHWM_bytes"], "samples": samples}
        image_path = out / "exact-state.crsnap"
        engine.request({"op": "save_snapshot", "snapshot_id": snapshots[-1], "path": str(image_path.resolve())})
        result["snapshot_file_sha256"], result["snapshot_semantic_sha256"] = snapshot_digest(image_path)
        (out / "result.json").write_text(json.dumps(result, indent=2) + "\n")
        return result
    finally:
        engine.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--games", type=int, default=12)
    parser.add_argument("--binary", type=Path, default=ROOT / "rust-engine/target/pgo/rust_league_rl")
    parser.add_argument("--cases", nargs="+", choices=tuple(CASES), default=list(CASES))
    args = parser.parse_args()
    traces = sorted((ROOT / "analysis/fast-league-worktree-20261022/development").glob("[0-9]*.json.gz"))[:args.games]
    if len(traces) != args.games:
        raise ValueError(f"requested {args.games} traces, found {len(traces)}")
    results = []
    for name in args.cases:
        result = run_case(name, traces, args.out / name, args.binary.resolve())
        if results and result["snapshot_file_sha256"] != results[0]["snapshot_file_sha256"]:
            raise AssertionError("Complete saved engine state differs across allocator cases")
        results.append(result)
        (args.out / "results.json").write_text(json.dumps(results, indent=2) + "\n")
        print(json.dumps({key: value for key, value in result.items() if key != "samples"}), flush=True)


if __name__ == "__main__":
    main()
