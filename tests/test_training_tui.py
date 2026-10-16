from pathlib import Path

import torch

from cr_replay_pipeline.policy_model import NUM_TILES, TILE_COLS
from cr_replay_pipeline.policy_train import batch_play_metrics
from cr_replay_pipeline.training_tui import (
    BLUE,
    GREEN,
    VIOLET,
    TrainingDashboard,
    _axis_chart,
    _nice_ticks,
    _want_trainer_tui,
)


def test_nice_ticks_cover_the_range():
    ticks = _nice_ticks(7.2, 8.9, 5)
    assert len(ticks) >= 3
    assert ticks[0] <= ticks[-1]
    assert ticks[-1] >= 8.5
    gaps = [b - a for a, b in zip(ticks, ticks[1:])]
    assert gaps and max(gaps) - min(gaps) < 1e-6


def test_axis_chart_has_graduations_without_loss_acc_words():
    loss = [8.8 - 0.02 * i + (0.1 if i % 7 == 0 else 0.0) for i in range(80)]
    chart = _axis_chart(
        [(loss, BLUE)],
        width=72,
        height=18,
        kind="loss",
        x_values=list(range(1, 81)),
        y_range=(0.0, 10.0),
    )
    plain = chart.plain
    assert "│" in plain
    assert "└" in plain
    assert "┬" in plain
    assert "┤" in plain
    assert "loss" not in plain.lower()
    assert "acc" not in plain.lower()
    lines = [line for line in plain.splitlines() if line.strip()]
    assert len(lines) >= 16
    widths = {len(line.rstrip()) for line in lines[:-1]}
    assert max(widths) >= 60


def test_axis_chart_x_scale_starts_at_zero():
    loss = [8.0 - 0.01 * i for i in range(80)]
    chart = _axis_chart(
        [(loss, BLUE)],
        width=72,
        height=14,
        kind="loss",
        x_values=list(range(515, 595)),
        y_range=(0.0, 10.0),
    )
    labels = chart.plain.splitlines()[-1]
    assert labels.strip().startswith("0") or " 0" in labels[:12]


def test_tile_chart_shares_percent_axis_for_exact_and_near():
    exact = [0.08 + 0.001 * i for i in range(40)]
    near = [0.18 + 0.002 * i for i in range(40)]
    chart = _axis_chart(
        [(exact, GREEN), (near, VIOLET)],
        width=48,
        height=12,
        kind="pct",
    )
    plain = chart.plain
    assert "%" in plain
    assert "0%" in plain
    assert "100" in plain
    assert any(glyph in plain for glyph in "█▀▄")
    assert "loss" not in plain.lower()
    assert "acc" not in plain.lower()
    assert "tile" not in plain.lower()


def test_pct_chart_draws_values_below_fifty_percent():
    low = [0.09 + 0.002 * i for i in range(40)]
    chart = _axis_chart([(low, GREEN)], width=48, height=16, kind="pct")
    plain = chart.plain
    assert "0%" in plain
    assert "  50%" in plain or "50%" in plain
    assert any(glyph in plain for glyph in "█▀▄")


def test_dashboard_keeps_the_full_run_not_a_window(tmp_path: Path):
    dash = TrainingDashboard(
        title="watch",
        progress_path=tmp_path / "progress.jsonl",
        log_path=tmp_path / "watch.log",
        epochs=20,
        batches_total=2477,
        samples_total=100,
        batch_size=8,
        device="cpu",
        parameters=12,
        use_tui=False,
        start_live=False,
        persist=False,
    )
    for index in range(1, 901):
        dash.ingest(
            {
                "phase": "v8",
                "epoch": 1,
                "batch": index,
                "batches_total": 2477,
                "loss": 8.0,
                "card": 0.2,
                "tile_exact": 0.1,
                "tile_near": 0.2,
                "wait": 0.6,
            },
            refresh=False,
        )
    assert len(dash.losses) == 900
    assert dash.steps[0] == 1
    assert dash.steps[-1] == 900
    dash.close()


def test_watch_dashboard_does_not_wipe_progress(tmp_path: Path):
    progress = tmp_path / "progress.jsonl"
    progress.write_text('{"phase":"v8","loss":1.2,"parameters":12}\n', encoding="utf-8")
    dash = TrainingDashboard(
        title="watch",
        progress_path=progress,
        log_path=tmp_path / "watch.log",
        epochs=20,
        batches_total=10,
        samples_total=100,
        batch_size=8,
        device="cpu",
        parameters=12,
        use_tui=False,
        start_live=False,
        persist=False,
    )
    assert "1.2" in progress.read_text(encoding="utf-8")
    assert not (tmp_path / "watch.log").exists()
    dash.close()


def test_trainer_tui_stays_off_unless_opted_in(monkeypatch):
    monkeypatch.delenv("CR_TRAINING_TUI", raising=False)
    assert _want_trainer_tui(True) is False
    monkeypatch.setenv("CR_TRAINING_TUI", "1")
    assert _want_trainer_tui(False) is False


def test_batch_play_metrics_exact_near_and_wait():
    batch = 4
    xy = torch.tensor([[0.01, 0.01]] * batch)
    slots = torch.zeros(batch, dtype=torch.long)
    slot_logits = torch.zeros(batch, 8)
    slot_logits[:, 0] = 4.0
    tile_logits = torch.zeros(batch, NUM_TILES)
    tile_logits[0, 0] = 4.0
    tile_logits[1, 1] = 4.0
    tile_logits[2, TILE_COLS] = 4.0
    tile_logits[3, 2] = 4.0
    true_dt = torch.log1p(torch.tensor([1.0, 1.0, 1.0, 1.0]))
    pred_dt = torch.log1p(torch.tensor([1.0, 1.4, 3.0, 1.0]))
    metrics = batch_play_metrics(
        {"slot_logits": slot_logits, "tile_logits": tile_logits, "timing": pred_dt},
        slots,
        xy,
        true_dt,
    )
    assert metrics["card"] == 1.0
    assert metrics["tile_exact"] == 0.25
    assert metrics["tile_near"] == 0.75
    assert metrics["wait"] == 0.75
