import json
import threading
from pathlib import Path
from http.server import ThreadingHTTPServer
from urllib.request import Request, urlopen

from cr_replay_pipeline.frontier import Frontier
from cr_replay_pipeline.server import IngestStore, make_handler


def record(tag: str, payload: dict) -> dict:
    return {
        "request_url": f"https://royaleapi.com/data/replay?tag={tag}",
        "referrer_url": "https://royaleapi.com/player/TEST/battles",
        "payload": payload,
    }


def test_rate_limit_is_quarantined_without_entering_raw(tmp_path: Path):
    store = IngestStore(tmp_path / "raw")
    result = store.store(
        record(
            "ABC123",
            {
                "status": 429,
                "error_code": 1015,
                "retry_after": 30,
            },
        )
    )

    assert result["classification"] == "rate_limited"
    assert result["stored"] is False
    assert not (tmp_path / "raw" / "ABC123.json").exists()
    assert (
        tmp_path / "quarantine" / "capture-errors" / "ABC123.json"
    ).exists()
    assert store.health()["rate_limited"] == 1


def test_error_cannot_overwrite_valid_replay(tmp_path: Path):
    store = IngestStore(tmp_path / "raw")
    good = record("ABC123", {"success": True, "html": "<div>replay</div>"})
    store.store(good)
    store.store(record("ABC123", {"status": 429, "error_code": 1015}))

    saved = json.loads((tmp_path / "raw" / "ABC123.json").read_text())
    assert saved["payload"]["success"] is True


def test_first_valid_replay_is_preserved_as_duplicate(tmp_path: Path):
    store = IngestStore(tmp_path / "raw")
    first = record("ABC123", {"success": True, "html": "<div>first</div>"})
    second = record("ABC123", {"success": True, "html": "<div>second</div>"})

    assert store.store(first)["stored"] is True
    duplicate = store.store(second)

    assert duplicate["classification"] == "duplicate"
    saved = json.loads((tmp_path / "raw" / "ABC123.json").read_text())
    assert saved["payload"]["html"] == "<div>first</div>"


def test_http_queue_and_ingest_discovery(tmp_path: Path):
    frontier = Frontier(tmp_path / "collector.sqlite3")
    server = ThreadingHTTPServer(
        ("127.0.0.1", 0), make_handler(tmp_path / "raw", frontier)
    )
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()

    def post(path: str, body: dict) -> dict:
        request = Request(
            f"http://127.0.0.1:{server.server_port}{path}",
            data=json.dumps(body).encode(),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urlopen(request) as response:
            return json.load(response)

    try:
        assert post("/players/seed", {"tags": ["AAA"]})["added"] == 1
        assert post("/jobs/claim", {"worker_id": "test"})["job"]["tag"] == "AAA"
        ingested = post(
            "/replays",
            {
                "request_url": (
                    "https://royaleapi.com/data/replay?tag=GAME&"
                    "team_tags=AAA&opponent_tags=BBB"
                ),
                "referrer_url": "https://royaleapi.com/player/AAA/battles",
                "payload": {"success": True, "html": "<div/>"},
            },
        )
        assert ingested["players_discovered"] == 1
        assert frontier.status()["players"]["queued"] == 1
    finally:
        server.shutdown()
        server.server_close()
        thread.join()
