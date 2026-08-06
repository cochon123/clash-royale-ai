from pathlib import Path

from cr_replay_pipeline.cleaner import _label_abilities
from cr_replay_pipeline.parser import parse_replay

FIXTURE = Path(__file__).parent / "fixtures" / "replay.json"


def test_parser_preserves_ability_events_and_ticks():
    replay = parse_replay(FIXTURE)
    assert replay.battle_id == "TEST123"
    assert len(replay.decks["team"]) == 8
    abilities = [
        event for event in replay.events if event.event_type == "ability_activation"
    ]
    assert [(event.ticks, event.side) for event in abilities] == [
        (200, "team"),
        (400, "opponent"),
    ]
    assert abilities[0].ability_card is None
    assert abilities[1].ability_card == "skeleton-king"
    assert abilities[1].attribution == "direct_icon"
    assert abilities[0].seconds == 10.0


def test_single_historical_candidate_labels_empty_ability():
    replay = parse_replay(FIXTURE)
    reasons = _label_abilities(replay, metadata=None, roster={"knight", "giant"})
    assert reasons == []
    team_ability = next(
        event
        for event in replay.events
        if event.event_type == "ability_activation" and event.side == "team"
    )
    assert team_ability.ability_card == "knight"
    assert team_ability.attribution == "single_historical_candidate"


def test_ambiguous_historical_candidates_are_rejected():
    replay = parse_replay(FIXTURE)
    reasons = _label_abilities(
        replay, metadata=None, roster={"knight", "goblins", "giant"}
    )
    assert "team_has_multiple_hero_candidates" in reasons
    assert "unresolved_ability_team" in reasons

