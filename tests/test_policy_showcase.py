import json
from pathlib import Path

from cr_replay_pipeline.policy_showcase import ZONE_NAMES, _entropy, _zone_grid, pretty_card
from cr_replay_pipeline.policy_showcase_report import (
    _cell_pairs,
    _metric_rows,
    render_policy_showcase_report,
)
from collections import Counter


def test_zone_grid_and_entropy():
    counts = Counter({0: 3, 5: 1})
    grid = _zone_grid(counts, 4)
    assert len(grid) == len(ZONE_NAMES)
    assert grid[0] == 0.75 and grid[5] == 0.25
    assert abs(sum(grid) - 1.0) < 1e-9
    assert _zone_grid(Counter(), 0) == [0.0] * 12
    flat = [1 / 12] * 12
    assert abs(_entropy(flat) - 3.5849625) < 1e-4
    assert _entropy([1.0] + [0.0] * 11) == 0.0


def test_pretty_card():
    assert pretty_card("hog-rider") == "Hog Rider"


def test_metric_rows_prefer_same_sample_numbers():
    overall = {
        "n": 100,
        "old_slot_acc": 0.5,
        "new_slot_acc": 0.55,
        "old_slot_top3": 0.9,
        "new_slot_top3": 0.92,
        "old_zone_acc": 0.4,
        "new_zone_acc": 0.44,
        "old_tile_acc": 0.02,
        "new_tile_acc": 0.05,
        "old_xy_mae": 5600.0,
        "new_xy_mae": 5200.0,
    }
    rows = _metric_rows(
        overall,
        {"test": {"timing_mae": 1.54}, "rollouts": {"mean_score_policy": 0.88}},
        {"test": {"timing_mae": 1.57}, "rollouts": {"mean_score_policy": 0.79}},
        {"overall": {"top1": 0.615, "n": 7603}},
        {"overall": {"top1": 0.607, "n": 4767}},
    )
    by_label = {r["label"]: r for r in rows}
    # same-sample rows must come from the rescoring, not the archived runs
    assert by_label["Which card (slot top-1)"]["new"] == 0.55
    assert by_label["Which card (slot top-1)"]["fair"] is True
    assert by_label["Placement error"]["higher"] is False
    assert by_label["Real defense windows"]["fair"] is False
    assert [r["fair"] for r in rows][:5] == [True] * 5


def test_cell_pairs_sorted_by_gain():
    new_support = {
        "cells": [
            {"threat": "hog-rider", "answer": "cannon", "model_top1_answer_given_in_hand": 0.6},
            {"threat": "graveyard", "answer": "poison", "model_top1_answer_given_in_hand": 0.4},
        ]
    }
    old_support = {
        "cells": [
            {"threat": "hog-rider", "answer": "cannon", "model_top1_answer_given_in_hand": 0.68},
            {"threat": "graveyard", "answer": "poison", "model_top1_answer_given_in_hand": 0.26},
        ]
    }
    pairs = _cell_pairs(new_support, old_support)
    assert [p["answer"] for p in pairs] == ["poison", "cannon"]
    assert pairs[0]["label"] == "graveyard → poison"


def _fake_showcase() -> dict:
    grid = [1 / 12] * 12
    card = {
        "card": "hog-rider",
        "label": "Hog Rider",
        "n": 12,
        "cost": 4,
        "is_wincon": True,
        "human_grid": grid,
        "new_grid": grid,
        "old_grid": grid,
        "new_zone_acc": 0.5,
        "old_zone_acc": 0.4,
        "scatter": [{"hx": 0.5, "hy": 0.3, "nx": 0.52, "ny": 0.31, "ox": 0.7, "oy": 0.8}],
    }
    return {
        "created_at": "2026-08-08T00:00:00Z",
        "new_model": {"version": "4"},
        "old_model": {"version": "3"},
        "compute": {"device": "cpu", "battles_scored": 3, "actions_scored": 12, "seconds": 1.0},
        "zone_names": list(ZONE_NAMES),
        "overall": {
            "n": 12,
            "human_grid": grid,
            "new_grid": grid,
            "old_grid": grid,
            "new_zone_acc": 0.44,
            "old_zone_acc": 0.41,
            "new_slot_acc": 0.55,
            "old_slot_acc": 0.52,
            "new_slot_top3": 0.94,
            "old_slot_top3": 0.93,
            "new_xy_mae": 5200.0,
            "old_xy_mae": 5600.0,
            "new_tile_acc": 0.05,
            "old_tile_acc": 0.02,
            "zone_entropy_human": 3.4,
            "zone_entropy_new": 3.2,
            "zone_entropy_old": 3.1,
        },
        "cards": [card],
        "fix_gallery": [
            {
                "battle_id": "ABC123",
                "card": "hog-rider",
                "label": "Hog Rider",
                "seconds": 42.0,
                "true_zone": 4,
                "old_zone": 9,
                "new_zone": 4,
                "true_xy": [0.5, 0.3],
                "new_xy": [0.51, 0.31],
                "old_xy": [0.8, 0.9],
                "is_reaction": True,
            }
        ],
        "fix_counts": {"fixed": 7, "regressed": 3, "net": 4},
        "scenarios": [
            {
                "battle_id": "ABC1234567",
                "seconds": 60.0,
                "threat": "hog-rider",
                "threat_label": "Hog Rider",
                "threat_delay": 1.5,
                "threat_xy": [0.3, 0.8],
                "human_xy": [0.5, 0.3],
                "new_xy": [0.52, 0.32],
                "hand": [
                    {"slot": i, "card": f"card-{i}", "label": f"Card {i}", "cost": 3}
                    for i in range(4)
                ],
                "human_slot": 1,
                "human_card": "card-1",
                "new_slot": 1,
                "old_slot": 2,
                "new_slot_p": 0.42,
                "human_zone": 4,
                "new_zone": 4,
            }
        ],
        "splits": {
            "reaction": {"n": 5, "new_slot_acc": 0.6, "old_slot_acc": 0.57,
                         "new_zone_acc": 0.55, "old_zone_acc": 0.51},
            "non_reaction": {"n": 7, "new_slot_acc": 0.53, "old_slot_acc": 0.51,
                             "new_zone_acc": 0.41, "old_zone_acc": 0.37},
        },
    }


def test_render_showcase_report(tmp_path: Path):
    show = tmp_path / "showcase.json"
    show.write_text(json.dumps(_fake_showcase()), encoding="utf-8")
    model_dir = tmp_path / "policy_bc_v4"
    old_dir = tmp_path / "policy_bc_v3"
    for path, timing in ((model_dir, 1.54), (old_dir, 1.57)):
        path.mkdir()
        (path / "report.json").write_text(
            json.dumps(
                {
                    "test": {"timing_mae": timing},
                    "rollouts": {"mean_score_policy": 0.88},
                    "history": [
                        {"epoch": 1, "val_slot_top1": 0.5, "val_zone_acc": 0.4},
                        {"epoch": 2, "val_slot_top1": 0.52, "val_zone_acc": 0.42},
                    ],
                    "lessons": ["placement needs the card identity"],
                }
            ),
            encoding="utf-8",
        )
    out = render_policy_showcase_report(
        showcase_path=show,
        model_dir=model_dir,
        old_model_dir=old_dir,
        slice_path=tmp_path / "missing_slice_v4.json",
        support_path=tmp_path / "missing_support_v4.json",
        output_path=tmp_path / "showcase.html",
    )
    body = out.read_text(encoding="utf-8")
    assert out.exists()
    # the interactive scaffolding and its data payload both need to be present
    for marker in ("id=\"arena\"", "id=\"quiz\"", "id=\"race\"", "Hog Rider", "const DATA ="):
        assert marker in body
    assert "matplotlib" not in body
