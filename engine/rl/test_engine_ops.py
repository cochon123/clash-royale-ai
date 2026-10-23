"""Exactness checks for board export, snapshots, and step_many.

These tests launch the real ``rust_league_rl`` binary. They are not a
substitute for the league regression.
"""
from __future__ import annotations

import gzip
import json
import time
from pathlib import Path

import pytest

from .engine_ops import (
    audit, close, load_snapshot, observe, open_scoped, reset, restore,
    save_snapshot, snapshot, step, step_many,
)

ROOT = Path(__file__).resolve().parents[1]
BINARY = ROOT / "rust-engine/target/release/rust_league_rl"
CHECKPOINT = ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"
TRACE = ROOT / "analysis/fast-league-worktree-20261022/development/0000.json.gz"
TIMING = {"reset_seconds", "step_seconds", "query_seconds"}


def semantic(value):
    if isinstance(value, dict):
        return {key: semantic(item) for key, item in value.items() if key not in TIMING}
    if isinstance(value, list):
        return [semantic(item) for item in value]
    return value


def trace_records():
    with gzip.open(TRACE, "rt") as handle:
        return json.load(handle)["records"]


def open_engine(tmp_path, name, **kwargs):
    if not BINARY.exists():
        pytest.fail(f"missing release binary {BINARY}")
    return open_scoped(BINARY, CHECKPOINT, tmp_path / name, **kwargs)


def audit_signature(path: Path):
    meta = json.loads((path / "audit.json").read_text())
    allocator = json.loads((path / "allocator.json").read_text())
    return {
        "regions": [(row["file"], row["sha256"], row["size"]) for row in meta["regions"]],
        "registers": meta["registers"],
        "ticks": meta["ticks"],
        "imports_called": meta["imports_called"],
        "allocator": allocator,
    }


def board_static(board):
    """Identity and pose, without the motion that an extra observe clears."""
    return [
        {key: entity[key] for key in (
            "id", "row", "side", "x", "y", "hp", "max_hp", "first_seen_tick")}
        for entity in board["entities"]
    ]


def test_default_path_matches_development_prefix(tmp_path):
    records = trace_records()[:6]
    client = open_engine(tmp_path, "default.log")
    try:
        for index, record in enumerate(records):
            actual = client.request(record["request"])
            assert semantic(actual) == record["expected"], index
            assert set(actual) - TIMING == set(record["expected"])
    finally:
        close(client)


def test_board_exports_entities_and_keeps_tower_hp(tmp_path):
    records = trace_records()
    reset_request = records[0]["request"]
    step_request = next(record["request"] for record in records if record["request"]["op"] == "step")
    plain = open_engine(tmp_path, "plain.log")
    try:
        without = reset(plain, reset_request["deck0"], reset_request["deck1"],
                        seed=reset_request["seed"], max_ticks=400, reward_state=True, board=False)
        without_step = step(plain, step_request["ticks"], step_request["actions"])
    finally:
        close(plain)
    assert "board" not in without and "board" not in without_step
    boarded = open_engine(tmp_path, "board.log")
    try:
        with_board = reset(boarded, reset_request["deck0"], reset_request["deck1"],
                           seed=reset_request["seed"], max_ticks=400, reward_state=True, board=True)
        boarded_step = step(boarded, step_request["ticks"], step_request["actions"])
    finally:
        close(boarded)
    for left, right in ((without, with_board), (without_step, boarded_step)):
        assert semantic(left["observation"]) == semantic(right["observation"])
        assert left["reward_state"] == right["reward_state"]
    board = boarded_step["board"]
    assert board["tick"] == boarded_step["observation"]["tick"]
    assert board["entities"]
    towers = [entity for entity in board["entities"] if entity["row"] in ("KingTower", "PrincessTower")]
    assert len(towers) == 6
    by_id = {(tower["side"], tower["id"]): tower for tower in towers}
    for tower in boarded_step["reward_state"]["towers"]:
        entity = by_id[(tower["side"], tower["id"])]
        assert entity["hp"] == tower["hp"]
        assert entity["max_hp"] == tower["max_hp"]
        assert entity["row"] == tower["row"]
    rows = {entity["row"] for entity in board["entities"]}
    assert {"KingTower", "PrincessTower"} <= rows


def test_snapshot_restore_and_reload_match(tmp_path):
    records = trace_records()
    steps = [record["request"] for record in records if record["request"]["op"] == "step"]
    client = open_engine(tmp_path, "snap.log", max_snapshots=4)
    measurements = {}
    try:
        reset_request = records[0]["request"]
        reset(client, reset_request["deck0"], reset_request["deck1"],
              seed=reset_request["seed"], max_ticks=220, reward_state=True, board=True)
        played = step(client, steps[0]["ticks"], steps[0]["actions"])
        started = time.perf_counter()
        image = snapshot(client)
        measurements["snapshot_seconds"] = time.perf_counter() - started
        measurements["snapshot_bytes"] = image["bytes"]
        path = tmp_path / "branch.snap"
        saved = save_snapshot(client, image["snapshot_id"], path)
        measurements["file_bytes"] = saved["file_bytes"]
        measurements["file_size"] = path.stat().st_size
        suffix = steps[1:7]
        continuation = []
        for request in suffix:
            reply = step(client, request["ticks"], request["actions"])
            continuation.append(semantic(reply))
            if reply["terminated"] or reply["truncated"]:
                break
        assert continuation, "the branched game produced no further steps"
        audit(client, tmp_path / "audit-a")
        signature = audit_signature(tmp_path / "audit-a")
        started = time.perf_counter()
        restored = restore(client, image["snapshot_id"])
        measurements["restore_seconds"] = time.perf_counter() - started
        assert semantic(restored["observation"]) == semantic(played["observation"])
        assert restored["reward_state"] == played["reward_state"]
        assert board_static(restored["board"]) == board_static(played["board"])
        replay = []
        for request in suffix[:len(continuation)]:
            reply = step(client, request["ticks"], request["actions"])
            replay.append(semantic(reply))
            if reply["terminated"] or reply["truncated"]:
                break
        assert replay == continuation
        audit(client, tmp_path / "audit-b")
        assert audit_signature(tmp_path / "audit-b") == signature
        for _ in range(6):
            try:
                snapshot(client)
            except RuntimeError as error:
                assert "snapshot limit" in str(error)
                break
        else:
            pytest.fail("snapshot limit was not enforced")
    finally:
        close(client)

    reloaded = open_engine(tmp_path, "reload.log", max_snapshots=4)
    try:
        loaded = load_snapshot(reloaded, path)
        assert loaded["tick"] == image["tick"]
        assert loaded["bytes"] == image["bytes"]
        observed = observe(reloaded)
        assert semantic(observed["observation"]) == semantic(played["observation"])
        assert board_static(observed["board"]) == board_static(played["board"])
        replay = []
        for request in suffix[:len(continuation)]:
            reply = step(reloaded, request["ticks"], request["actions"])
            replay.append(semantic(reply))
            if reply["terminated"] or reply["truncated"]:
                break
        assert replay == continuation
        audit(reloaded, tmp_path / "audit-c")
        assert audit_signature(tmp_path / "audit-c") == signature
    finally:
        close(reloaded)
    out = ROOT / "analysis-local/snapshot-measurements.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(measurements, indent=2) + "\n")
    print(json.dumps(measurements))


def test_step_many_matches_single_sessions(tmp_path):
    records = trace_records()
    reset_request = records[0]["request"]
    steps = [record["request"] for record in records if record["request"]["op"] == "step"][:3]
    multi = open_engine(tmp_path, "multi.log", sessions=2, max_snapshots=2)
    try:
        for session in (0, 1):
            reset(multi, reset_request["deck0"], reset_request["deck1"],
                  seed=reset_request["seed"] + session, max_ticks=400,
                  reward_state=True, board=True, session=session)
        batched = []
        for request in steps:
            batched.append(step_many(multi, [
                {"session": 0, "ticks": request["ticks"], "actions": request["actions"]},
                {"session": 1, "ticks": request["ticks"], "actions": request["actions"]},
            ]))
    finally:
        close(multi)
    singles = []
    for session in (0, 1):
        client = open_engine(tmp_path, f"single-{session}.log")
        try:
            reset(client, reset_request["deck0"], reset_request["deck1"],
                  seed=reset_request["seed"] + session, max_ticks=400,
                  reward_state=True, board=True)
            singles.append([semantic(step(client, request["ticks"], request["actions"])) for request in steps])
        finally:
            close(client)
    for index, batch in enumerate(batched):
        assert len(batch["items"]) == 2
        assert semantic(batch["items"][0]) == singles[0][index]
        assert semantic(batch["items"][1]) == singles[1][index]
