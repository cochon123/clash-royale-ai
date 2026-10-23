"""Board, snapshot, and batched-step helpers for the league engine.

These call the same JSONL ``request`` method as ``NativeClient`` in
``banana_hog_league``. ``open_scoped`` starts the real binary under a 6G
memory cap with no swap.
"""
from __future__ import annotations

import subprocess
from pathlib import Path

from .banana_hog_league import NativeClient, ROOT


def _with_session(request, session):
    if session is not None:
        request = dict(request)
        request["session"] = session
    return request


def scoped_command(binary, checkpoint, *, sessions=1, max_snapshots=8):
    command = [
        "systemd-run", "--user", "--scope", "--quiet",
        "-p", "MemoryMax=6G", "-p", "MemorySwapMax=0",
        "--",
        str(binary), "--checkpoint", str(checkpoint),
        "--max-snapshots", str(max_snapshots),
    ]
    if sessions != 1:
        command.extend(["--sessions", str(sessions)])
    return command


def open_scoped(binary, checkpoint, log_path, *, sessions=1, max_snapshots=8):
    """Launch ``rust_league_rl`` with the league client's JSONL protocol."""
    return NativeClient(
        binary, checkpoint, Path(log_path),
        command=scoped_command(binary, checkpoint, sessions=sessions, max_snapshots=max_snapshots),
    )


def reset(client, deck0, deck1, *, seed=1, max_ticks=7200, reward_state=False, board=False, session=None):
    request = {
        "op": "reset", "deck0": list(deck0), "deck1": list(deck1),
        "seed": seed, "max_ticks": max_ticks,
    }
    if reward_state:
        request["reward_state"] = True
    if board:
        request["board"] = True
    return client.request(_with_session(request, session))


def observe(client, session=None):
    return client.request(_with_session({"op": "observe"}, session))


def step(client, ticks, actions, session=None):
    return client.request(_with_session({"op": "step", "ticks": ticks, "actions": actions}, session))


def placement_mask(client, side, slot, session=None):
    return client.request(_with_session({"op": "placement_mask", "side": side, "slot": slot}, session))


def snapshot(client, session=None):
    return client.request(_with_session({"op": "snapshot"}, session))


def restore(client, snapshot_id, session=None):
    return client.request(_with_session({"op": "restore", "snapshot_id": snapshot_id}, session))


def drop_snapshot(client, snapshot_id, session=None):
    return client.request(_with_session({"op": "drop_snapshot", "snapshot_id": snapshot_id}, session))


def save_snapshot(client, snapshot_id, path, session=None):
    return client.request(_with_session(
        {"op": "save_snapshot", "snapshot_id": snapshot_id, "path": str(path)}, session))


def load_snapshot(client, path, session=None):
    return client.request(_with_session({"op": "load_snapshot", "path": str(path)}, session))


def step_many(client, items):
    return client.request({"op": "step_many", "items": items})


def audit(client, out, session=None):
    return client.request(_with_session({"op": "audit", "out": str(out)}, session))


def close(client):
    try:
        client.request({"op": "close"}, timeout=10)
    except Exception:
        pass
    client.close()


def run_scoped(command):
    """Run one short engine-side command inside the same memory scope."""
    return subprocess.run(
        ["systemd-run", "--user", "--scope", "--quiet", "-p", "MemoryMax=6G", "-p", "MemorySwapMax=0", "--", *command],
        cwd=ROOT, check=True,
    )
