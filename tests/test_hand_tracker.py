"""Unit tests for exact CR cycle hand reconstruction."""

from cr_replay_pipeline.hand_tracker import (
    consistent_initial_indices,
    hand_posteriors_causal,
    hand_posteriors_smoothed,
    track_side,
)
from cr_replay_pipeline.winner_dataset import BattleExample
import numpy as np


def test_oldest_cycle_recovers_hand_after_full_rotation():
    plays = [0, 1, 2, 3, 4, 5, 6, 7]
    posts, n = hand_posteriors_smoothed(plays)
    assert n > 0
    # After observing a full oldest-only cycle, opening hand of the true
    # rotation becomes certain at the wrap-around (before 2nd play of 0 is
    # after 8 plays — check posterior before play index 8 on extended seq).
    posts2, _ = hand_posteriors_smoothed(plays * 2)
    # Before the 9th play (second 0), hand must be {0,1,2,3}.
    assert posts2[8, 0] >= 0.99
    assert posts2[8, 1] >= 0.99
    assert posts2[8, 2] >= 0.99
    assert posts2[8, 3] >= 0.99
    assert posts2[8, 4] <= 0.01


def test_first_played_card_has_posterior_one():
    plays = [3, 1, 0, 2, 5, 4, 7, 6]
    posts, n = hand_posteriors_smoothed(plays)
    assert n > 0
    assert posts[0, 3] == 1.0


def test_inconsistent_sequence_untrackable():
    # Impossible: play 0 then immediately play 0 again before it can cycle
    # back — actually 0 goes to back and needs 4 more plays to return.
    # After playing 0, hand has 3 others + next. Playing 0 again too soon
    # with only 3 intervening from a fixed set can be impossible.
    # Construct clearly impossible: play card 0 twice with zero other plays.
    assert consistent_initial_indices([0, 0]).size == 0
    assert hand_posteriors_smoothed([0, 0]) is None


def test_causal_opening_is_uniform_smoothed_pins_first_play():
    plays = [0, 1, 2, 3, 4, 5, 6, 7, 0, 1]
    causal, n_alive = hand_posteriors_causal(plays)
    smoothed, _n = hand_posteriors_smoothed(plays)
    assert causal is not None and smoothed is not None
    assert n_alive[0] == 40320
    # Causal has no observations yet → each card in hand w/ prob 1/2.
    assert abs(causal[0, 0] - 0.5) < 1e-9
    # Smoothed conditions on the full sequence → first played card is certain.
    assert smoothed[0, 0] == 1.0


def test_track_side_on_synthetic_battle():
    deck = tuple(f"c{i}" for i in range(8))
    opp = tuple(f"o{i}" for i in range(8))
    events = []
    seconds = 1.0
    # Play team cards in deck order (cycle-friendly).
    for i in range(12):
        events.append(
            {
                "seconds": seconds,
                "side": "team",
                "event_type": "card_play",
                "card": deck[i % 8],
                "x": 9000,
                "y": 8000,
            }
        )
        seconds += 2.0
        events.append(
            {
                "seconds": seconds,
                "side": "opponent",
                "event_type": "card_play",
                "card": opp[i % 8],
                "x": 9000,
                "y": 24000,
            }
        )
        seconds += 2.0
    battle = BattleExample(
        battle_id="hand-test",
        team_deck=deck,
        opponent_deck=opp,
        team_wins=True,
        events=tuple(events),
    )
    track = track_side(battle, "team")
    assert track.trackable
    assert track.n_consistent > 0
    assert track.smoothed.shape[0] == 12
    mask, source = track.mask_at(track.event_indices[8], threshold=0.5, smoothed=False)
    assert source == "exact"
    assert mask.dtype == bool
    assert int(mask.sum()) >= 1
