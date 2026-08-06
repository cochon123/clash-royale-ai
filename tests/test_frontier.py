import json
import sqlite3
from pathlib import Path

import pytest

from cr_replay_pipeline.frontier import Frontier, recommended_workers, replay_participants


def test_two_workers_are_enough_to_saturate_the_request_governor():
    assert recommended_workers(5.0) == 2
    assert recommended_workers(3.0) == 2
    assert recommended_workers(0.8) == 2


def test_database_context_closes_connection(tmp_path: Path):
    frontier = Frontier(tmp_path / "collector.sqlite3")
    with frontier._connect() as connection:
        connection.execute("SELECT 1").fetchone()
    with pytest.raises(sqlite3.ProgrammingError, match="closed database"):
        connection.execute("SELECT 1")


def test_frontier_claim_discover_complete_and_retry(tmp_path: Path):
    frontier = Frontier(tmp_path / "collector.sqlite3")
    assert frontier.seed(["#ABC", "ABC", "DEF"]) == 2

    first = frontier.claim("worker-1")
    assert first is not None
    assert first["tag"] in {"ABC", "DEF"}
    assert frontier.complete(first["tag"]) == "completed"

    assert frontier.discover(first["tag"], {first["tag"], "XYZ"}) == 1
    status = frontier.status()
    assert status["players"]["completed"] == 1
    assert status["players"]["queued"] == 2

    second = frontier.claim("worker-2")
    assert second is not None
    assert frontier.complete(second["tag"], retry=True, error="layout") == "queued"


def test_claim_is_idempotent_for_a_worker(tmp_path: Path):
    frontier = Frontier(tmp_path / "collector.sqlite3")
    frontier.seed(["AAA", "BBB"])
    first = frontier.claim("worker-1")
    repeated = frontier.claim("worker-1")
    assert repeated == first
    assert frontier.status()["players"]["leased"] == 1


def test_pause_prevents_claims(tmp_path: Path):
    frontier = Frontier(tmp_path / "collector.sqlite3")
    frontier.seed(["ABC"])
    frontier.pause("challenge")
    assert frontier.claim("worker") is None
    assert frontier.status()["pause_reason"] == "challenge"
    frontier.resume()
    assert frontier.claim("worker")["tag"] == "ABC"


def test_rate_controller_calibrates_and_backs_off(tmp_path: Path):
    frontier = Frontier(
        tmp_path / "collector.sqlite3",
        initial_interval=2.2,
        calibration_window=3,
    )
    for _ in range(3):
        frontier.rate_result(rate_limited=False)
    assert frontier.status()["rate"]["interval_seconds"] == 2.046

    frontier.rate_result(rate_limited=True, retry_after=12)
    rate = frontier.status()["rate"]
    assert rate["interval_seconds"] == 2.558
    assert rate["blocked_for_seconds"] > 11
    assert rate["total_rate_limits"] == 1
    assert rate["recent_rate_limits"] == 1
    assert rate["recent_requests"] == 4
    assert rate["recent_rate_limit_percent"] == 25.0


def test_rate_controller_migrates_an_inflated_legacy_interval(tmp_path: Path):
    path = tmp_path / "collector.sqlite3"
    frontier = Frontier(path)
    with frontier._connect() as db:
        frontier._set(db, "interval_seconds", 9.0)
        frontier._set(db, "rate_controller_version", 2)

    migrated = Frontier(path)
    assert migrated.status()["rate"]["interval_seconds"] == 3.3


def test_bootstrap_marks_sources_done_and_queues_opponents(tmp_path: Path):
    raw = tmp_path / "raw"
    raw.mkdir()
    record = {
        "request_url": (
            "https://royaleapi.com/data/replay?tag=BATTLE&"
            "team_tags=AAA&opponent_tags=BBB"
        ),
        "referrer_url": "https://royaleapi.com/player/AAA/battles?crcollector=1",
        "payload": {"success": True, "html": "<div/>"},
    }
    (raw / "BATTLE.json").write_text(json.dumps(record), encoding="utf-8")

    source, participants = replay_participants(record)
    assert source == "AAA"
    assert participants == {"AAA", "BBB"}

    frontier = Frontier(tmp_path / "collector.sqlite3")
    result = frontier.bootstrap(raw)
    assert result["sources_completed"] == 1
    assert result["new_queued"] == 1
    assert frontier.status()["players"] == {
        "total": 2,
        "queued": 1,
        "leased": 0,
        "completed": 1,
        "manual": 0,
    }
