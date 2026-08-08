from pathlib import Path

from cr_replay_pipeline.realism_generate import (
    TimingPrior,
    generate_easy_negative,
    generate_hard_negative,
    generate_medium_negative,
)
from cr_replay_pipeline.realism_report import render_realism_report
from cr_replay_pipeline.realism_train import extract_realism_features
from cr_replay_pipeline.winner_dataset import BattleExample
import random


def _sample_battle() -> BattleExample:
    return BattleExample(
        battle_id="battle-realism-1",
        team_deck=(
            "knight",
            "archers",
            "goblins",
            "fireball",
            "cannon",
            "skeletons",
            "the-log",
            "ice-spirit",
        ),
        opponent_deck=(
            "hog-rider",
            "musketeer",
            "ice-golem",
            "cannon",
            "musketeer",
            "skeletons",
            "ice-spirit",
            "fireball",
        ),
        team_wins=1,
        events=(
            {
                "seconds": 10.0,
                "side": "team",
                "event_type": "card_play",
                "card": "knight",
                "x": 4000,
                "y": 8000,
            },
            {
                "seconds": 14.0,
                "side": "opponent",
                "event_type": "card_play",
                "card": "hog-rider",
                "x": 12000,
                "y": 24000,
            },
            {
                "seconds": 18.0,
                "side": "team",
                "event_type": "card_play",
                "card": "fireball",
                "x": 9000,
                "y": 22000,
            },
            {
                "seconds": 22.0,
                "side": "opponent",
                "event_type": "card_play",
                "card": "cannon",
                "x": 8000,
                "y": 20000,
            },
            {
                "seconds": 28.0,
                "side": "team",
                "event_type": "card_play",
                "card": "archers",
                "x": 5000,
                "y": 7000,
            },
            {
                "seconds": 34.0,
                "side": "opponent",
                "event_type": "card_play",
                "card": "musketeer",
                "x": 11000,
                "y": 25000,
            },
            {
                "seconds": 40.0,
                "side": "team",
                "event_type": "card_play",
                "card": "ice-spirit",
                "x": 6000,
                "y": 9000,
            },
            {
                "seconds": 48.0,
                "side": "opponent",
                "event_type": "card_play",
                "card": "ice-golem",
                "x": 10000,
                "y": 23000,
            },
            {
                "seconds": 55.0,
                "side": "team",
                "event_type": "card_play",
                "card": "goblins",
                "x": 7000,
                "y": 10000,
            },
            {
                "seconds": 62.0,
                "side": "opponent",
                "event_type": "card_play",
                "card": "fireball",
                "x": 9000,
                "y": 9000,
            },
            {
                "seconds": 70.0,
                "side": "team",
                "event_type": "card_play",
                "card": "the-log",
                "x": 9000,
                "y": 15000,
            },
            {
                "seconds": 80.0,
                "side": "opponent",
                "event_type": "card_play",
                "card": "skeletons",
                "x": 8500,
                "y": 21000,
            },
        ),
    )


def test_timing_prior_samples_positive_gaps():
    prior = TimingPrior.from_battles([_sample_battle()])
    rng = random.Random(0)
    gap = prior.sample(rng)
    assert 0.05 <= gap <= 30.0


def test_generators_produce_deck_legal_card_plays():
    battle = _sample_battle()
    costs = {card: 3 for card in battle.team_deck + battle.opponent_deck}
    costs["fireball"] = 4
    costs["hog-rider"] = 4
    prior = TimingPrior.from_battles([battle])
    rng = random.Random(1)

    easy = generate_easy_negative(battle, costs, rng, prior)
    medium = generate_medium_negative(battle, costs, random.Random(2), prior)
    hard = generate_hard_negative(battle, costs, random.Random(3), prior)

    for synthetic in (easy, medium, hard):
        assert len(synthetic.events) >= 8
        for event in synthetic.events:
            deck = (
                battle.team_deck
                if event["side"] == "team"
                else battle.opponent_deck
            )
            if event["event_type"] == "card_play":
                assert event["card"] in deck
                if event["side"] == "team":
                    assert event["y"] < 16000
                else:
                    assert event["y"] > 16000


def test_realism_features_are_finite_vector():
    features = extract_realism_features(_sample_battle(), {"knight": 3, "fireball": 4})
    assert features.ndim == 1
    assert features.size >= 40
    assert features.dtype.kind == "f"
    assert not any(value != value for value in features)  # no NaNs


def test_render_realism_report_matches_winner_style(tmp_path: Path) -> None:
    model_dir = Path("models/realism_scorer")
    if not (model_dir / "report.json").exists():
        return

    # Reuse the cached spot-the-fake rounds so the test stays offline and fast.
    cached = Path("reports/realism_spot_the_fake.json")
    if cached.exists():
        (tmp_path / "realism_spot_the_fake.json").write_text(
            cached.read_text(encoding="utf-8"), encoding="utf-8"
        )

    out = render_realism_report(
        model_dir, tmp_path / "realism_scorer_v1.html", build_quiz=False
    )
    text = out.read_text(encoding="utf-8")
    assert "<!doctype html>" in text.lower()
    assert 'class="chart"' in text
    assert "Lessons learned" in text
    assert "Spot the fake" in text
    assert "const DATA =" in text
    assert "matplotlib" not in text.lower()
    assert "<canvas" not in text
