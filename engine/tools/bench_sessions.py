#!/usr/bin/env python3
"""Games per minute: one process with 8 sessions versus 8 processes.

Both sides play random positions that placement_mask marks legal. The whole
process tree is meant to be started under one MemoryMax=6G scope so the eight
engines share that cap. This script does not add another systemd-run.
"""
from __future__ import annotations

import argparse
import gzip
import json
import os
import selectors
import subprocess
import threading
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CHECKPOINT = ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"
TRACE = ROOT / "analysis/fast-league-worktree-20261022/development/0000.json.gz"


class Client:
    def __init__(self, binary, sessions, log_path):
        command = [str(binary), "--checkpoint", str(CHECKPOINT)]
        if sessions != 1:
            command.extend(["--sessions", str(sessions)])
        self.proc = subprocess.Popen(
            command, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=log_path.open("ab"), cwd=ROOT, bufsize=0,
        )
        self.selector = selectors.DefaultSelector()
        self.selector.register(self.proc.stdout, selectors.EVENT_READ)
        self.pending = b""
        self.lock = threading.Lock()

    def request(self, obj):
        payload = json.dumps(obj, separators=(",", ":")).encode() + b"\n"
        with self.lock:
            self.proc.stdin.write(payload)
            end = time.monotonic() + 180
            while b"\n" not in self.pending:
                remaining = end - time.monotonic()
                if remaining <= 0 or not self.selector.select(remaining):
                    raise TimeoutError(obj.get("op"))
                data = os.read(self.proc.stdout.fileno(), 1 << 20)
                if not data:
                    raise RuntimeError(f"engine exited {self.proc.poll()}")
                self.pending += data
            line, self.pending = self.pending.split(b"\n", 1)
        reply = json.loads(line)
        if reply.get("ok") is not True:
            raise RuntimeError(str(reply.get("error")))
        return reply["result"]

    def close(self):
        if self.proc.poll() is None:
            try:
                self.request({"op": "close"})
            except Exception:
                self.proc.kill()
            self.proc.wait(timeout=10)
        self.selector.close()


def decks():
    with gzip.open(TRACE, "rt") as handle:
        request = json.load(handle)["records"][0]["request"]
    return request["deck0"], request["deck1"]


def tile_coord(tile, side):
    x = round(((tile % 32) + 0.5) * 18000 / 32)
    y_norm = ((tile // 32) + 0.5) / 18
    y = round((1 - y_norm if side == 1 else y_norm) * 32000)
    return x, y


def legal_action(client, side, slot, session, rng_lock, rng):
    request = {"op": "placement_mask", "side": side, "slot": slot}
    if session is not None:
        request["session"] = session
    mask = client.request(request)["mask"]
    legal = [index for index, allowed in enumerate(mask) if allowed]
    if not legal:
        return None
    with rng_lock:
        tile = legal[rng.randrange(len(legal))]
    x, y = tile_coord(tile, side)
    return {"side": side, "slot": slot, "x": x, "y": y}


def run_sessions(binary, seconds, log_dir):
    """One host, eight sessions. Masks are serial; steps run together."""
    import random
    deck0, deck1 = decks()
    rng = random.Random(55)
    rng_lock = threading.Lock()
    client = Client(binary, 8, log_dir / "sessions.log")
    games = 0
    steps = 0
    mask_calls = 0
    seeds = [10000 + index for index in range(8)]
    started = time.monotonic()
    try:
        for session in range(8):
            client.request({
                "op": "reset", "session": session, "deck0": deck0, "deck1": deck1,
                "seed": seeds[session], "max_ticks": 3600,
            })
        deadline = time.monotonic() + seconds
        while time.monotonic() < deadline:
            items = []
            for session in range(8):
                actions = []
                for side in (0, 1):
                    with rng_lock:
                        play = rng.random() < 0.5
                        slot = rng.randrange(4)
                    if not play:
                        continue
                    action = legal_action(client, side, slot, session, rng_lock, rng)
                    mask_calls += 1
                    if action:
                        actions.append(action)
                items.append({"session": session, "ticks": 20, "actions": actions})
            result = client.request({"op": "step_many", "items": items})
            steps += 8
            resets = []
            for session, reply in enumerate(result["items"]):
                if reply["terminated"] or reply["truncated"]:
                    games += 1
                    seeds[session] += 8
                    resets.append(session)
            for session in resets:
                client.request({
                    "op": "reset", "session": session, "deck0": deck0, "deck1": deck1,
                    "seed": seeds[session], "max_ticks": 3600,
                })
    finally:
        client.close()
    elapsed = time.monotonic() - started
    return {"games": games, "steps": steps, "mask_calls": mask_calls, "seconds": elapsed}


def run_processes(binary, seconds, log_dir):
    """Eight single-session processes, each deciding and stepping on its thread."""
    import random
    deck0, deck1 = decks()
    clients = [Client(binary, 1, log_dir / f"proc-{index}.log") for index in range(8)]
    counts = {"games": 0, "steps": 0, "mask_calls": 0}
    lock = threading.Lock()
    started = time.monotonic()
    deadline = started + seconds

    def worker(index, client):
        rng = random.Random(55 + index)
        rng_lock = threading.Lock()
        seed = 10000 + index
        client.request({
            "op": "reset", "deck0": deck0, "deck1": deck1,
            "seed": seed, "max_ticks": 3600,
        })
        while time.monotonic() < deadline:
            actions = []
            for side in (0, 1):
                if rng.random() >= 0.5:
                    continue
                slot = rng.randrange(4)
                action = legal_action(client, side, slot, None, rng_lock, rng)
                with lock:
                    counts["mask_calls"] += 1
                if action:
                    actions.append(action)
            reply = client.request({"op": "step", "ticks": 20, "actions": actions})
            with lock:
                counts["steps"] += 1
            if reply["terminated"] or reply["truncated"]:
                with lock:
                    counts["games"] += 1
                seed += 8
                client.request({
                    "op": "reset", "deck0": deck0, "deck1": deck1,
                    "seed": seed, "max_ticks": 3600,
                })

    threads = [threading.Thread(target=worker, args=(index, client)) for index, client in enumerate(clients)]
    try:
        for thread in threads:
            thread.start()
        for thread in threads:
            thread.join()
    finally:
        for client in clients:
            client.close()
    counts["seconds"] = time.monotonic() - started
    return counts


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--seconds", type=int, default=75)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    log_dir = args.out.parent / "bench-logs"
    log_dir.mkdir(parents=True, exist_ok=True)
    started = time.monotonic()
    sessions = run_sessions(args.binary, args.seconds, log_dir)
    processes = run_processes(args.binary, args.seconds, log_dir)
    def rate(row):
        return round(row["games"] / row["seconds"] * 60, 2) if row["seconds"] else 0
    payload = {
        "binary": str(args.binary),
        "seconds_each": args.seconds,
        "max_ticks": 3600,
        "note": "8 sessions share one address-space-isolated host. placement_mask calls on that host are serial; the 8 processes call it in parallel. step_many is the concurrent part.",
        "sessions": {**sessions, "games_per_minute": rate(sessions)},
        "processes": {**processes, "games_per_minute": rate(processes)},
        "elapsed_seconds": round(time.monotonic() - started, 3),
    }
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(payload, indent=2) + "\n")
    print(json.dumps(payload, indent=2), flush=True)


if __name__ == "__main__":
    main()
