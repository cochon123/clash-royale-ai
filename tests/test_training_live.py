import json
from pathlib import Path

from cr_replay_pipeline.training_live import ProgressFeed, snapshot_progress, stride_indices


def _write_jsonl(path: Path, rows: list[dict]) -> None:
    path.write_text(
        "".join(json.dumps(row) + "\n" for row in rows),
        encoding="utf-8",
    )


def test_stride_keeps_first_and_last():
    idx = stride_indices(5000, 200)
    assert idx[0] == 0
    assert idx[-1] == 4999
    assert len(idx) <= 200 or idx[0] == 0
    assert len(idx) >= 2


def test_snapshot_skips_loading_and_validation(tmp_path: Path):
    progress = tmp_path / "progress.jsonl"
    _write_jsonl(
        progress,
        [
            {"phase": "loading", "epoch": 0, "batch": 0, "loss": None},
            {
                "phase": "v8",
                "epoch": 1,
                "batch": 1,
                "batches_total": 100,
                "loss": 8.5,
                "card": 0.2,
                "acc": 0.2,
                "tile_exact": 0.1,
                "tile_near": 0.2,
                "wait": 0.6,
                "model_name": "policy-bc-v8",
                "parameters": 12,
            },
            {
                "phase": "validation",
                "epoch": 1,
                "batch": 100,
                "batches_total": 100,
                "loss": 99.0,
                "card": 0.4,
                "tile_exact": 0.3,
                "tile_near": 0.5,
                "wait": 0.7,
            },
            {
                "phase": "log",
                "message": "12:00:00  saved best checkpoint",
            },
            {
                "phase": "v8",
                "epoch": 1,
                "batch": 50,
                "batches_total": 100,
                "loss": 7.2,
                "card": 0.25,
                "tile_exact": 0.12,
                "tile_near": 0.22,
                "wait": 0.61,
            },
        ],
    )
    snap = snapshot_progress(progress)
    assert snap["series"]["loss"] == [8.5, 7.2]
    assert 99.0 not in snap["series"]["loss"]
    assert snap["series"]["step"][0] == 1
    assert snap["series"]["step"][-1] == 50
    assert snap["n_points"] == 2
    assert any("saved best" in line for line in snap["log"])
    assert snap["val"]["card"] == 0.4


def test_feed_does_not_wipe_progress(tmp_path: Path):
    progress = tmp_path / "progress.jsonl"
    progress.write_text('{"phase":"v8","loss":1.2,"epoch":1,"batch":3,"batches_total":10}\n', encoding="utf-8")
    original = progress.read_text(encoding="utf-8")
    feed = ProgressFeed(progress)
    feed.poll()
    feed.poll()
    assert progress.read_text(encoding="utf-8") == original
    assert feed.loss == [1.2]


def test_feed_resets_when_file_shrinks(tmp_path: Path):
    progress = tmp_path / "progress.jsonl"
    _write_jsonl(
        progress,
        [
            {"phase": "v8", "epoch": 1, "batch": i, "batches_total": 20, "loss": 8.0 - i * 0.01, "card": 0.2}
            for i in range(1, 6)
        ],
    )
    feed = ProgressFeed(progress)
    feed.poll()
    assert len(feed.loss) == 5
    _write_jsonl(
        progress,
        [{"phase": "v8", "epoch": 1, "batch": 1, "batches_total": 20, "loss": 4.0, "card": 0.3}],
    )
    feed.poll()
    assert feed.loss == [4.0]
    assert feed.steps == [1.0]


def test_controlled_suite_progress_drives_browser_dashboard(tmp_path: Path):
    progress = tmp_path / "progress.jsonl"
    _write_jsonl(progress, [{
        "timestamp": "2026-09-07T03:00:00+00:00", "phase": "memory",
        "model_name": "controlled · memory", "epoch": 25, "epochs_total": 36,
        "batch": 10, "batches_total": 100, "loss": 6.2, "card": .51,
        "tile_exact": .13, "samples_done": 2500, "samples_total": 3600,
        "progress_percent": 69.4, "eta_seconds": 420, "elapsed_seconds": 800,
        "parameters": 1234, "device": "cuda", "batch_size": 128,
    }])
    snap = snapshot_progress(progress)
    assert snap["run"]["title"] == "controlled · memory"
    assert snap["run"]["samples_done"] == 2500
    assert snap["run"]["eta_seconds"] == 420
    assert snap["run"]["progress_percent"] == 69.4
    assert snap["series"]["card"] == [.51]


def test_incomplete_last_line_is_held(tmp_path: Path):
    progress = tmp_path / "progress.jsonl"
    progress.write_text('{"phase":"v8","epoch":1,"batch":1,"batches_total":10,"loss":5.0,"card":0.1}', encoding="utf-8")
    feed = ProgressFeed(progress)
    feed.poll()
    assert feed.loss == []
    with progress.open("a", encoding="utf-8") as handle:
        handle.write('\n{"phase":"v8","epoch":1,"batch":2,"batches_total":10,"loss":4.5,"card":0.2}\n')
    feed.poll()
    assert feed.loss == [5.0, 4.5]
