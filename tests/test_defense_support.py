"""Unit tests for defense support-audit decision rules."""

from cr_replay_pipeline.defense_support_audit import _decide_cell
from cr_replay_pipeline.defense_support_report import render_defense_support_report


def test_decide_unsupported_low_n():
    row = {
        "n_threat_answer_in_deck": 50,
        "n_threat_answer_in_hand": 40,
        "human_use_rate_given_in_deck": 0.5,
        "human_use_rate_given_in_hand": 0.5,
        "n_human_chose_answer_in_hand": 10,
        "model_top1_when_human_chose_answer": 0.1,
        "model_top1_answer_given_in_hand": 0.1,
        "model_beats_cheap_when_human_chose": 0.2,
    }
    d = _decide_cell(row)
    assert d["status"] == "unsupported"


def test_decide_unsupported_low_human_rate():
    row = {
        "n_threat_answer_in_deck": 500,
        "n_threat_answer_in_hand": 400,
        "human_use_rate_given_in_deck": 0.05,
        "human_use_rate_given_in_hand": 0.08,
        "n_human_chose_answer_in_hand": 20,
        "model_top1_when_human_chose_answer": 0.9,
        "model_top1_answer_given_in_hand": 0.9,
        "model_beats_cheap_when_human_chose": 0.9,
    }
    d = _decide_cell(row)
    assert d["status"] == "unsupported"


def test_decide_supported_but_model_fails():
    row = {
        "n_threat_answer_in_deck": 500,
        "n_threat_answer_in_hand": 400,
        "human_use_rate_given_in_deck": 0.4,
        "human_use_rate_given_in_hand": 0.45,
        "n_human_chose_answer_in_hand": 80,
        "model_top1_when_human_chose_answer": 0.2,
        "model_top1_answer_given_in_hand": 0.15,
        "model_beats_cheap_when_human_chose": 0.3,
    }
    d = _decide_cell(row)
    assert d["status"] == "supported_but_model_fails"


def test_decide_supported_and_ok():
    row = {
        "n_threat_answer_in_deck": 500,
        "n_threat_answer_in_hand": 400,
        "human_use_rate_given_in_deck": 0.4,
        "human_use_rate_given_in_hand": 0.45,
        "n_human_chose_answer_in_hand": 80,
        "model_top1_when_human_chose_answer": 0.6,
        "model_top1_answer_given_in_hand": 0.55,
        "model_beats_cheap_when_human_chose": 0.8,
    }
    d = _decide_cell(row)
    assert d["status"] == "supported_and_ok"
    assert d["sol_natural_counterfactual"]["sol_pass"] is True


def test_render_support_report(tmp_path):
    audit = {
        "created_at": "2026-01-01T00:00:00Z",
        "model_name": "policy-bc-v2",
        "setup": {
            "hand_heuristic": "test",
            "max_response_seconds": 5.0,
            "thresholds": {
                "unsupported_if_n_lt": 200,
                "unsupported_if_human_rate_lt": 0.15,
                "model_fail_if_top1_when_human_lt": 0.35,
                "sol_min_examples": 50,
                "sol_top1": 0.5,
                "sol_beats_cheap": 0.7,
            },
        },
        "summary": {
            "cells": 1,
            "status_counts": {"unsupported": 1},
            "failing_cells": 1,
            "control_cells": 0,
            "recommend_v3": False,
            "next_step": "drop_unsupported_gates_continue_offline",
        },
        "cells": [
            {
                "threat": "graveyard",
                "answer": "poison",
                "role": "failing",
                "n_threat_answer_in_deck": 10,
                "n_threat_answer_in_hand": 5,
                "human_use_rate_given_in_deck": 0.1,
                "human_use_rate_given_in_hand": 0.2,
                "n_scored_when_human_chose_test": 0,
                "model_top1_when_human_chose_answer": None,
                "model_beats_cheap_when_human_chose": None,
                "decision": {
                    "status": "unsupported",
                    "action": "Drop from gates.",
                },
            }
        ],
        "verdict": "Test verdict",
        "lessons": ["Lesson one"],
    }
    audit_path = tmp_path / "audit.json"
    audit_path.write_text(__import__("json").dumps(audit), encoding="utf-8")
    out = tmp_path / "report.html"
    path = render_defense_support_report(audit_path=audit_path, output_path=out)
    html = path.read_text(encoding="utf-8")
    assert "graveyard" in html
    assert "unsupported" in html
    assert "Test verdict" in html
