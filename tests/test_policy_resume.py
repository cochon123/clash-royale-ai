from pathlib import Path

import torch
from torch import nn
from torch.optim import AdamW
from torch.optim.lr_scheduler import CosineAnnealingLR

from cr_replay_pipeline.policy_train import (
    clip_history_to_checkpoint,
    cpu_state_dict,
    eager_module,
    load_resume_checkpoint,
    resume_start_epoch,
)
from cr_replay_pipeline.training_tui import TrainingDashboard


def test_resume_start_epoch_uses_checkpoint_not_stale_history():
    checkpoint = {"epoch": 6}
    history = [{"epoch": 1}, {"epoch": 6}, {"epoch": 7}]
    assert resume_start_epoch(checkpoint, history) == 7
    assert [row["epoch"] for row in clip_history_to_checkpoint(history, checkpoint)] == [1, 6]


def test_resume_start_epoch_falls_back_to_history():
    assert resume_start_epoch({}, [{"epoch": 4}]) == 5
    assert resume_start_epoch({}, []) == 1


def test_eager_module_unwraps_orig_mod():
    inner = nn.Linear(3, 2)
    wrapper = nn.Identity()
    wrapper._orig_mod = inner
    assert eager_module(wrapper) is inner
    assert eager_module(inner) is inner


def test_cpu_state_dict_clones_detached_cpu_weights():
    model = nn.Linear(4, 3)
    with torch.no_grad():
        model.weight.fill_(1.5)
    state = cpu_state_dict(model)
    assert state["weight"].device.type == "cpu"
    assert state["weight"].is_contiguous()
    model.weight.data.fill_(0.0)
    assert torch.allclose(state["weight"], torch.full((3, 4), 1.5))


def test_load_resume_prefers_last_checkpoint(tmp_path: Path):
    vocab = {"name_to_id": {"a": 1}, "id_to_name": {"1": "a"}}
    torch.save(
        {"epoch": 6, "best_val_loss": 7.5, "model_state": {}, "vocab": vocab},
        tmp_path / "best_model.pt",
    )
    torch.save(
        {"epoch": 7, "best_val_loss": 7.4, "model_state": {}, "vocab": vocab},
        tmp_path / "last.pt",
    )
    (tmp_path / "training_stages.json").write_text(
        '[{"epoch": 6}, {"epoch": 7}]',
        encoding="utf-8",
    )
    checkpoint, history, start_epoch = load_resume_checkpoint(tmp_path)
    assert checkpoint["epoch"] == 7
    assert start_epoch == 8
    assert [row["epoch"] for row in history] == [6, 7]


def test_cosine_lr_after_six_epochs_matches_hung_run():
    param = nn.Parameter(torch.zeros(1))
    optimizer = AdamW([param], lr=2e-4)
    scheduler = CosineAnnealingLR(optimizer, T_max=20)
    for _ in range(6):
        scheduler.step()
    assert abs(optimizer.param_groups[0]["lr"] - 0.00015877852522924732) < 1e-12


def test_dashboard_resume_appends_without_wiping(tmp_path: Path):
    progress = tmp_path / "progress.jsonl"
    log_path = tmp_path / "run.log"
    progress.write_text('{"phase":"v8","epoch":7}\n', encoding="utf-8")
    log_path.write_text("old line\n", encoding="utf-8")
    dash = TrainingDashboard(
        title="resume",
        progress_path=progress,
        log_path=log_path,
        epochs=20,
        batches_total=10,
        samples_total=100,
        batch_size=8,
        device="cpu",
        parameters=12,
        use_tui=False,
        start_live=False,
        persist=True,
        reset=False,
    )
    dash.log("continuing")
    dash.close()
    text = progress.read_text(encoding="utf-8")
    assert '{"phase":"v8","epoch":7}' in text
    assert "continuing" in text
    assert "old line" in log_path.read_text(encoding="utf-8")
    assert "continuing" in log_path.read_text(encoding="utf-8")
