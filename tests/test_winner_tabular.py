import numpy as np

from cr_replay_pipeline.winner_dataset import BattleExample
from cr_replay_pipeline.winner_tabular import swap_battle_perspective
from cr_replay_pipeline.winner_visuals import (
    area_under_risk_coverage,
    confidence_curve,
    selective_curve,
)


def test_swap_battle_perspective_is_reversible():
    battle = BattleExample(
        battle_id="battle-1",
        team_deck=("archers",) * 8,
        opponent_deck=("knight",) * 8,
        team_wins=1,
        events=(
            {
                "seconds": 12.0,
                "side": "team",
                "event_type": "card_play",
                "card": "archers",
                "x": 3500,
                "y": 9500,
            },
            {
                "seconds": 15.0,
                "side": "opponent",
                "event_type": "card_play",
                "card": "knight",
                "x": 14500,
                "y": 22500,
            },
        ),
    )

    swapped = swap_battle_perspective(battle)
    assert swapped.team_deck == battle.opponent_deck
    assert swapped.opponent_deck == battle.team_deck
    assert swapped.team_wins == 0
    assert swapped.events[0]["side"] == "opponent"
    assert swapped.events[0]["x"] == 14500
    assert swapped.events[0]["y"] == 22500
    assert swap_battle_perspective(swapped) == battle


def test_confidence_curve_reports_accuracy_and_coverage():
    labels = np.asarray([1, 0, 1, 0])
    probabilities = np.asarray([0.9, 0.1, 0.55, 0.8])

    curve = confidence_curve(labels, probabilities, thresholds=[0.0, 0.2, 0.4])

    assert curve[0] == {
        "min_confidence": 0.0,
        "accuracy": 0.75,
        "coverage": 1.0,
        "n": 4,
    }
    assert curve[1]["accuracy"] == 2 / 3
    assert curve[1]["coverage"] == 0.75
    assert curve[1]["n"] == 3
    assert curve[2]["accuracy"] == 1.0
    assert curve[2]["coverage"] == 0.5
    assert curve[2]["n"] == 2


def test_selective_curve_accepts_calibrated_correctness_confidence():
    correct = np.asarray([1, 0, 1, 1])
    confidence = np.asarray([0.95, 0.55, 0.80, 0.90])

    curve = selective_curve(correct, confidence, thresholds=[0.5, 0.85, 0.95])

    assert curve[0]["accuracy"] == 0.75
    assert curve[0]["coverage"] == 1.0
    assert curve[1]["accuracy"] == 1.0
    assert curve[1]["coverage"] == 0.5
    assert curve[2]["accuracy"] == 1.0
    assert curve[2]["coverage"] == 0.25


def test_area_under_risk_coverage_rewards_ranking_correct_predictions_first():
    correct = np.asarray([1, 1, 0, 0])
    good_ranking = np.asarray([0.9, 0.8, 0.7, 0.6])
    bad_ranking = good_ranking[::-1]

    assert area_under_risk_coverage(correct, good_ranking) < area_under_risk_coverage(
        correct, bad_ranking
    )
