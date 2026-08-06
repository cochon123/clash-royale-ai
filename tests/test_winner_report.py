from pathlib import Path

from cr_replay_pipeline.winner_report import render_winner_reports


def test_render_winner_reports(tmp_path: Path) -> None:
    model_dir = Path("models/winner_predictor")
    if not (model_dir / "hgb_report.json").exists():
        return

    output_dir = tmp_path / "reports"
    paths = render_winner_reports(model_dir, output_dir)
    assert paths
    for path in paths:
        text = path.read_text(encoding="utf-8")
        assert "<!doctype html>" in text.lower()
        assert "Lessons learned" in text
