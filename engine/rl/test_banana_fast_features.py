from __future__ import annotations

import math
import random
import sys
import unittest
from dataclasses import replace
from pathlib import Path
from types import SimpleNamespace
from typing import Any
from unittest.mock import patch

import numpy as np
import torch

ROOT = Path(__file__).resolve().parents[1]
SIBLING_SRC = ROOT.parent / "src"
if str(SIBLING_SRC) not in sys.path:
    sys.path.insert(0, str(SIBLING_SRC))

from cr_replay_pipeline import policy_dataset, policy_infer, winner_dataset  # noqa: E402
from rl.banana_fast_features import encode_online_sample, _STREAM_CACHE, _CACHE_LOCK  # noqa: E402



# Frozen baseline (main 8b62365): this deliberately does not call the new fold.
from cr_replay_pipeline.policy_dataset import (
    DEFAULT_MIN_CONTEXT, SPELL_CARDS, WIN_CONDITIONISH, DEFAULT_ELIXIR_COST,
    _deck_stats, _deck_ids, _elixir_gain_rate, _event_cost, _window_sum,
    deck_slot_for_card, prefix_observation_seconds, SLOT_FEAT_DIM, BattleExample, CardVocab,
)

def _frozen_causal_stream(
    battle: BattleExample,
    vocab: CardVocab,
    costs: dict[str, int],
    swap_sides: bool = False,
    min_events: int | None = None,
    observation_seconds: float | None = None,
) -> tuple[torch.Tensor, torch.Tensor, torch.Tensor, torch.Tensor, torch.Tensor] | None:
    """Encode a full battle once with causal (sliceable) features.

    Unlike winner ``_encode_prefix``, every row is valid as a prefix end: no
    feature depends on future events. Globals are snapshotted after each event
    so ``globals[i]`` matches prefix ``events[:i+1]``.
    """
    events = battle.events
    need = DEFAULT_MIN_CONTEXT + 1 if min_events is None else max(1, int(min_events))
    if len(events) < need:
        return None

    team_elixir = 5.0
    opponent_elixir = 5.0
    team_spent = 0.0
    opponent_spent = 0.0
    team_plays = 0.0
    opponent_plays = 0.0
    team_deep = 0.0
    opponent_deep = 0.0
    team_left = 0.0
    opponent_left = 0.0
    team_bridge = 0.0
    opponent_bridge = 0.0
    team_y_sum = 0.0
    opponent_y_sum = 0.0
    team_abilities = 0.0
    opponent_abilities = 0.0
    team_spells = 0.0
    opponent_spells = 0.0
    team_wincons = 0.0
    opponent_wincons = 0.0
    team_double_spent = 0.0
    opponent_double_spent = 0.0
    team_triple_spent = 0.0
    opponent_triple_spent = 0.0
    team_unique: set[str] = set()
    opponent_unique: set[str] = set()
    team_cycle: list[str] = []
    opponent_cycle: list[str] = []
    team_spend_times: list[float] = []
    team_spend_vals: list[float] = []
    opp_spend_times: list[float] = []
    opp_spend_vals: list[float] = []
    team_deep_times: list[float] = []
    opp_deep_times: list[float] = []
    team_deep_streak = 0
    opp_deep_streak = 0
    team_max_deep_streak = 0
    opp_max_deep_streak = 0
    last_time = 0.0
    last_team_time = 0.0
    last_opp_time = 0.0
    streak = 0
    last_side: str | None = None
    continuous_rows: list[list[float]] = []
    card_id_rows: list[int] = []
    global_rows: list[list[float]] = []

    if swap_sides:
        team_deck_cards = battle.opponent_deck
        opp_deck_cards = battle.team_deck
    else:
        team_deck_cards = battle.team_deck
        opp_deck_cards = battle.opponent_deck
    team_avg, team_std, team_spell_frac = _deck_stats(team_deck_cards, costs)
    opp_avg, opp_std, opp_spell_frac = _deck_stats(opp_deck_cards, costs)
    shared = len(set(team_deck_cards) & set(opp_deck_cards)) / 8.0
    del team_std, opp_std

    for event in events:
        seconds = float(event["seconds"])
        gain = (seconds - last_time) / _elixir_gain_rate(seconds)
        team_elixir = min(10.0, team_elixir + gain)
        opponent_elixir = min(10.0, opponent_elixir + gain)

        side = event["side"]
        if swap_sides:
            side = "opponent" if side == "team" else "team"

        card = event["card"]
        cost = _event_cost(card, event["event_type"], costs)
        position_known = bool(event.get("position_known", True))
        x = float(event["x"]) / 18000.0 if position_known else 0.5
        y = float(event["y"]) / 32000.0 if position_known else 0.5
        if swap_sides:
            y = 1.0 - y

        is_ability = 1.0 if event["event_type"] == "ability_activation" else 0.0
        is_spell = 1.0 if card in SPELL_CARDS else 0.0
        is_wincon = 1.0 if card in WIN_CONDITIONISH else 0.0
        is_deep = 0.0
        is_bridge = 1.0 if position_known and 0.42 <= y <= 0.58 else 0.0
        is_left = 1.0 if position_known and x < 0.5 else 0.0
        is_double = 1.0 if seconds >= 120 else 0.0
        is_triple = 1.0 if seconds >= 240 else 0.0

        if side == "team":
            team_elixir = max(0.0, team_elixir - cost)
            team_spent += cost
            team_plays += 1.0
            team_y_sum += y
            team_abilities += is_ability
            team_spells += is_spell
            team_wincons += is_wincon
            if is_double:
                team_double_spent += cost
            if is_triple:
                team_triple_spent += cost
            if card and is_ability < 0.5:
                team_unique.add(card)
                team_cycle.append(card)
                if len(team_cycle) > 8:
                    team_cycle = team_cycle[-8:]
            team_spend_times.append(seconds)
            team_spend_vals.append(cost)
            side_flag = 1.0
            if y > 0.55:
                team_deep += 1.0
                is_deep = 1.0
                team_deep_times.append(seconds)
                team_deep_streak += 1
                team_max_deep_streak = max(team_max_deep_streak, team_deep_streak)
            else:
                team_deep_streak = 0
            if is_left:
                team_left += 1.0
            if is_bridge:
                team_bridge += 1.0
            dt_same = min((seconds - last_team_time) / 30.0, 2.0) if last_team_time else 0.0
            last_team_time = seconds
            cycle_avg = (
                float(np.mean([costs.get(c, DEFAULT_ELIXIR_COST) for c in team_cycle[-4:]]))
                / 10.0
                if team_cycle
                else 0.4
            )
        else:
            opponent_elixir = max(0.0, opponent_elixir - cost)
            opponent_spent += cost
            opponent_plays += 1.0
            opponent_y_sum += y
            opponent_abilities += is_ability
            opponent_spells += is_spell
            opponent_wincons += is_wincon
            if is_double:
                opponent_double_spent += cost
            if is_triple:
                opponent_triple_spent += cost
            if card and is_ability < 0.5:
                opponent_unique.add(card)
                opponent_cycle.append(card)
                if len(opponent_cycle) > 8:
                    opponent_cycle = opponent_cycle[-8:]
            opp_spend_times.append(seconds)
            opp_spend_vals.append(cost)
            side_flag = 0.0
            if y < 0.45:
                opponent_deep += 1.0
                is_deep = 1.0
                opp_deep_times.append(seconds)
                opp_deep_streak += 1
                opp_max_deep_streak = max(opp_max_deep_streak, opp_deep_streak)
            else:
                opp_deep_streak = 0
            if is_left:
                opponent_left += 1.0
            if is_bridge:
                opponent_bridge += 1.0
            dt_same = min((seconds - last_opp_time) / 30.0, 2.0) if last_opp_time else 0.0
            last_opp_time = seconds
            cycle_avg = (
                float(
                    np.mean(
                        [costs.get(c, DEFAULT_ELIXIR_COST) for c in opponent_cycle[-4:]]
                    )
                )
                / 10.0
                if opponent_cycle
                else 0.4
            )

        if side == last_side:
            streak += 1
        else:
            streak = 1
            last_side = side

        recent_team_30 = _window_sum(team_spend_times, team_spend_vals, seconds, 30.0)
        recent_opp_30 = _window_sum(opp_spend_times, opp_spend_vals, seconds, 30.0)
        play_scale = max(team_plays + opponent_plays, 1.0)
        # Causal: no "time remaining in battle" — that leaked the match length.
        continuous_rows.append(
            [
                min(seconds / 300.0, 1.5),
                side_flag,
                is_double,
                is_triple,
                x,
                y,
                team_elixir / 10.0,
                opponent_elixir / 10.0,
                (team_elixir - opponent_elixir) / 10.0,
                cost / 10.0,
                is_ability,
                is_spell,
                is_wincon,
                team_spent / 40.0,
                opponent_spent / 40.0,
                (team_spent - opponent_spent) / 40.0,
                (team_plays - opponent_plays) / play_scale,
                (team_deep - opponent_deep) / play_scale,
                is_deep,
                is_bridge,
                min(streak / 6.0, 1.0),
                dt_same,
                recent_team_30 / 20.0,
                recent_opp_30 / 20.0,
                (recent_team_30 - recent_opp_30) / 20.0,
                cycle_avg,
                team_elixir / 10.0 if side_flag > 0.5 else opponent_elixir / 10.0,
                0.0,
            ]
        )
        card_id_rows.append(vocab.encode(card))
        last_time = seconds

        team_mean_y = team_y_sum / max(team_plays, 1.0)
        opp_mean_y = opponent_y_sum / max(opponent_plays, 1.0)
        now = seconds
        if observation_seconds is not None and event is events[-1]:
            now = float(observation_seconds)
            if now < seconds:
                raise ValueError("Observation precedes the confirmed prefix")
            # Integrate across multiplier boundaries rather than applying the
            # ending multiplier to the whole quiet interval.
            gain = sum(max(0.0, min(now, end) - max(seconds, start)) / period
                       for start, end, period in ((0., 120., 2.8), (120., 240., 1.4), (240., float('inf'), .9)))
            team_elixir = min(10., team_elixir + gain)
            opponent_elixir = min(10., opponent_elixir + gain)
        team_15 = _window_sum(team_spend_times, team_spend_vals, now, 15.0)
        opp_15 = _window_sum(opp_spend_times, opp_spend_vals, now, 15.0)
        team_30 = _window_sum(team_spend_times, team_spend_vals, now, 30.0)
        opp_30 = _window_sum(opp_spend_times, opp_spend_vals, now, 30.0)
        team_60 = _window_sum(team_spend_times, team_spend_vals, now, 60.0)
        opp_60 = _window_sum(opp_spend_times, opp_spend_vals, now, 60.0)
        team_deep_30 = sum(1 for t in team_deep_times if t >= now - 30.0)
        opp_deep_30 = sum(1 for t in opp_deep_times if t >= now - 30.0)
        team_deep_60 = sum(1 for t in team_deep_times if t >= now - 60.0)
        opp_deep_60 = sum(1 for t in opp_deep_times if t >= now - 60.0)
        last_team_deep = team_deep_times[-1] if team_deep_times else -1.0
        last_opp_deep = opp_deep_times[-1] if opp_deep_times else -1.0
        if last_team_deep < 0 and last_opp_deep < 0:
            last_deep_side = 0.5
        elif last_team_deep >= last_opp_deep:
            last_deep_side = 1.0
        else:
            last_deep_side = 0.0
        team_cycle_avg = (
            float(np.mean([costs.get(c, DEFAULT_ELIXIR_COST) for c in team_cycle[-4:]]))
            / 10.0
            if team_cycle
            else 0.4
        )
        opp_cycle_avg = (
            float(np.mean([costs.get(c, DEFAULT_ELIXIR_COST) for c in opponent_cycle[-4:]]))
            / 10.0
            if opponent_cycle
            else 0.4
        )
        global_rows.append(
            [
                team_spent / 40.0,
                opponent_spent / 40.0,
                (team_spent - opponent_spent) / 40.0,
                team_plays / 40.0,
                opponent_plays / 40.0,
                (team_plays - opponent_plays) / play_scale,
                team_deep / play_scale,
                opponent_deep / play_scale,
                (team_deep - opponent_deep) / play_scale,
                team_left / max(team_plays, 1.0),
                opponent_left / max(opponent_plays, 1.0),
                team_bridge / play_scale,
                opponent_bridge / play_scale,
                team_mean_y,
                opp_mean_y,
                team_mean_y - opp_mean_y,
                team_abilities / 8.0,
                opponent_abilities / 8.0,
                team_elixir / 10.0,
                opponent_elixir / 10.0,
                (team_elixir - opponent_elixir) / 10.0,
                min(now / 300.0, 1.5),
                (team_15 - opp_15) / 15.0,
                (team_30 - opp_30) / 20.0,
                (team_60 - opp_60) / 30.0,
                team_15 / 15.0,
                opp_15 / 15.0,
                team_30 / 20.0,
                opp_30 / 20.0,
                (team_deep_30 - opp_deep_30) / 4.0,
                (team_deep_60 - opp_deep_60) / 6.0,
                last_deep_side,
                min((now - last_team_deep) / 60.0, 2.0) if last_team_deep >= 0 else 2.0,
                min((now - last_opp_deep) / 60.0, 2.0) if last_opp_deep >= 0 else 2.0,
                (team_double_spent - opponent_double_spent) / 30.0,
                (team_triple_spent - opponent_triple_spent) / 20.0,
                (team_spells - opponent_spells) / 8.0,
                (team_wincons - opponent_wincons) / 8.0,
                team_cycle_avg,
                opp_cycle_avg,
                team_cycle_avg - opp_cycle_avg,
                len(team_unique) / 8.0,
                len(opponent_unique) / 8.0,
                min(team_max_deep_streak / 5.0, 1.0),
                min(opp_max_deep_streak / 5.0, 1.0),
                (team_avg - opp_avg) / 10.0,
                (team_spell_frac - opp_spell_frac),
                shared,
            ]
        )

    team_deck = _deck_ids(team_deck_cards, vocab)
    opponent_deck = _deck_ids(opp_deck_cards, vocab)
    return (
        torch.tensor(continuous_rows, dtype=torch.float32),
        torch.tensor(card_id_rows, dtype=torch.long),
        team_deck,
        opponent_deck,
        torch.tensor(global_rows, dtype=torch.float32),
    )

def frozen_encode_online_sample(
    dataset: Any,
    battle: Any,
    event_index: int,
    vocab: Any,
    costs: dict[str, int],
    max_context: int = 64,
    threat_dim: int = 0,
    min_context: int | None = None,
    observation_seconds: float | None = None,
    reaction_seconds: float = 5.0,
) -> tuple[torch.Tensor, ...] | None:
    """Return the same 13 tensors as ``encode_policy_sample`` with one stream pass.

    Kickoff, invalid-clock, and rejected-prefix cases delegate to the reference
    encoder. For ordinary prefixes, the target is omitted from the stream and
    the final global snapshot is encoded at the observation clock directly.
    """
    fallback = dataset.encode_policy_sample
    need = dataset.DEFAULT_MIN_CONTEXT if min_context is None else max(0, int(min_context))
    if event_index < need or event_index < 1 or event_index >= len(battle.events):
        return fallback(
            battle, event_index, vocab, costs, max_context=max_context,
            threat_dim=threat_dim, reaction_seconds=reaction_seconds,
            min_context=min_context, observation_seconds=observation_seconds,
        )
    try:
        observed_now = dataset.prefix_observation_seconds(battle, event_index, observation_seconds)
    except (TypeError, ValueError, OverflowError):
        observed_now = float("nan")
    if not math.isfinite(observed_now):
        return fallback(
            battle, event_index, vocab, costs, max_context=max_context,
            threat_dim=threat_dim, reaction_seconds=reaction_seconds,
            min_context=min_context, observation_seconds=observation_seconds,
        )

    target = battle.events[event_index]
    swap = target["side"] == "opponent"
    prefix = replace(battle, events=battle.events[:event_index])
    stream = dataset._encode_causal_stream(
        prefix, vocab, costs, swap_sides=swap, min_events=1,
        observation_seconds=observed_now,
    )
    if stream is None:
        return fallback(
            battle, event_index, vocab, costs, max_context=max_context,
            threat_dim=threat_dim, reaction_seconds=reaction_seconds,
            min_context=min_context, observation_seconds=observation_seconds,
        )
    continuous, card_ids, team_deck, opponent_deck, globals_all = stream
    if continuous.size(0) > max_context:
        continuous, card_ids = continuous[-max_context:], card_ids[-max_context:]
    global_feat = globals_all[-1]

    if threat_dim > 0:
        threat, _ = dataset.recent_opponent_threat(
            battle, event_index, target["side"], costs,
            max_age=reaction_seconds, observation_seconds=observed_now,
        )
        if threat_dim != dataset.THREAT_DIM:
            threat = (threat[:threat_dim] if threat.numel() >= threat_dim
                      else torch.nn.functional.pad(threat, (0, threat_dim - threat.numel())))
        global_feat = torch.cat((global_feat, threat), dim=-1)

    acting_deck = battle.opponent_deck if swap else battle.team_deck
    slot = dataset.deck_slot_for_card(acting_deck, target["card"])
    if slot is None:
        return None
    nx, ny = dataset._normalize_xy(int(target["x"]), int(target["y"]), swap_sides=swap)
    prev_seconds = float(battle.events[event_index - 1]["seconds"])
    delta = max(0.0, float(target["seconds"]) - prev_seconds)
    event_type = 1 if target["event_type"] == "ability_activation" else 0
    slot_feats, hand_mask = dataset.acting_cycle_features(
        battle, event_index, costs, swap, observation_seconds=observed_now,
    )
    from cr_replay_pipeline.policy_model import xy_to_zone

    return (
        continuous, card_ids, team_deck, opponent_deck, global_feat,
        slot_feats, hand_mask, torch.tensor(slot, dtype=torch.long),
        torch.tensor(event_type, dtype=torch.long), torch.tensor(xy_to_zone(nx, ny), dtype=torch.long),
        torch.tensor([nx, ny], dtype=torch.float32),
        torch.tensor(float(np.log1p(delta)), dtype=torch.float32),
        torch.tensor(continuous.size(0), dtype=torch.long),
    )


def _frozen_acting_cycle_features(
    battle: BattleExample,
    event_index: int,
    costs: dict[str, int],
    swap_sides: bool,
    observation_seconds: float | None = None,
) -> tuple[torch.Tensor, torch.Tensor]:
    """Per-deck-slot cycle features + soft hand mask for the acting side.

    Hand heuristic: 4 cards with the oldest last-play time (never-played count
    as oldest). This is imperfect at kickoff but becomes informative mid-game
    without inventing a starting shuffle.
    """
    acting_deck = battle.opponent_deck if swap_sides else battle.team_deck
    # Prefix uses original sides; swap selects which physical side is acting.
    raw_side = "opponent" if swap_sides else "team"

    last_play = [-1.0] * 8
    play_count = [0.0] * 8
    now = prefix_observation_seconds(battle, event_index, observation_seconds)
    for event in battle.events[:event_index]:
        if event["side"] != raw_side or event["event_type"] != "card_play":
            continue
        slot = deck_slot_for_card(acting_deck, event["card"])
        if slot is None:
            continue
        last_play[slot] = float(event["seconds"])
        play_count[slot] += 1.0

    waits: list[tuple[float, int]] = []
    feats = torch.zeros(8, SLOT_FEAT_DIM, dtype=torch.float32)
    for slot, card in enumerate(acting_deck):
        cost = float(costs.get(card, DEFAULT_ELIXIR_COST)) / 10.0
        never = 1.0 if last_play[slot] < 0 else 0.0
        dt = 2.0 if never else min((now - last_play[slot]) / 60.0, 2.0)
        waits.append((-1.0 if never else last_play[slot], slot))
        feats[slot, 0] = cost
        feats[slot, 1] = min(play_count[slot] / 10.0, 1.5)
        feats[slot, 2] = dt
        feats[slot, 3] = never
        feats[slot, 4] = float(slot) / 7.0
        feats[slot, 5] = 1.0 if cost <= 0.3 else 0.0  # cheap cycle card hint
        feats[slot, 6] = 1.0 if card in SPELL_CARDS else 0.0
        feats[slot, 7] = 1.0 if card in WIN_CONDITIONISH else 0.0

    # Oldest last-play first (never-played first).
    waits.sort(key=lambda item: (item[0], item[1]))
    hand_mask = torch.zeros(8, dtype=torch.bool)
    for _, slot in waits[:4]:
        hand_mask[slot] = True
        feats[slot, 3] = max(feats[slot, 3], 0.0)
    # Feature 3 already never; overwrite a wait-rank channel into col 4 was slot
    # index — also stamp approx-in-hand into never's neighbor by using col via mask.
    # Put hand indicator into a free interpretation: reuse col3 never OR add via mask only.
    for rank, (_, slot) in enumerate(waits):
        feats[slot, 4] = rank / 7.0

    # Confidence: once ≥4 plays by this side, hand heuristic is usable.
    side_plays = sum(
        1
        for event in battle.events[:event_index]
        if event["side"] == raw_side and event["event_type"] == "card_play"
    )
    if side_plays < 4:
        hand_mask[:] = False
    return feats, hand_mask

def frozen_dataset():
    return SimpleNamespace(**{**vars(policy_dataset), "_encode_causal_stream": _frozen_causal_stream,
                              "acting_cycle_features": _frozen_acting_cycle_features})


HOG = ("hog-rider", "musketeer", "ice-golem", "skeletons", "ice-spirit", "cannon", "fireball", "the-log")
MORTAR = ("mortar", "knight", "archers", "skeletons", "barbarian-barrel", "arrows", "mini-pekka", "rocket")


class FastFeatureParity(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        torch.set_num_threads(2)
        _, cls.vocab, _, _ = policy_infer.load_policy(
            ROOT.parent / "models" / "policy_bc_v4.4.2", "cpu"
        )
        cls.costs = winner_dataset.load_card_costs(
            ROOT.parent / "data" / "card_costs.json"
        )

    @staticmethod
    def battle(history_len: int) -> winner_dataset.BattleExample:
        rows = []
        for i in range(history_len + 1):
            side = "team" if i % 2 == 0 else "opponent"
            deck = HOG if side == "team" else MORTAR
            # Crosses double/triple-elixir boundaries in the longer prefixes.
            seconds = i * 2.15
            rows.append({"seconds": seconds, "side": side,
                         "event_type": "ability_activation" if i % 17 == 3 else "card_play",
                         "card": deck[(i * 3) % 8], "x": (i * 997) % 18000,
                         "y": (i * 1237) % 32000})
        return winner_dataset.BattleExample("feature-parity", HOG, MORTAR, 1, tuple(rows))

    def assert_equal_sample(self, battle, index, clock):
        expected = policy_dataset.encode_policy_sample(
            battle, index, self.vocab, self.costs, max_context=64,
            threat_dim=policy_dataset.THREAT_DIM, min_context=0,
            observation_seconds=clock,
        )
        calls = 0
        original = policy_dataset._encode_causal_stream

        def counted(*args, **kwargs):
            nonlocal calls
            calls += 1
            return original(*args, **kwargs)

        with patch.object(policy_dataset, "_encode_causal_stream", side_effect=counted):
            actual = encode_online_sample(
                policy_dataset, battle, index, self.vocab, self.costs,
                max_context=64, threat_dim=policy_dataset.THREAT_DIM,
                min_context=0, observation_seconds=clock,
            )
        self.assertEqual(len(expected), 13)
        self.assertEqual(len(actual), 13)
        for position, (left, right) in enumerate(zip(expected, actual)):
            self.assertTrue(torch.equal(left, right), f"tensor {position} differs at index={index}, clock={clock}")
        if index > 0 and clock is not None:
            self.assertEqual(calls, 0, "online path must not refold the reference stream")

    def test_lengths_sides_clocks_and_context_truncation(self):
        for history_len in (0, 1, 4, 65, 130):
            battle = self.battle(history_len)
            index = history_len
            if history_len == 0:
                self.assert_equal_sample(battle, index, 0.0)
                continue
            latest = battle.events[index - 1]["seconds"]
            # Both physical perspectives, exact event clocks, quiet clocks,
            # plus explicit double/triple-elixir observation times.
            for clock in (latest, latest + 13.0, max(latest, 120.0), max(latest, 240.0)):
                self.assert_equal_sample(battle, index, clock)

    def test_inferred_clock_matches_reference(self):
        battle = self.battle(4)
        self.assert_equal_sample(battle, 4, None)


class IncrementalFeatureParity(FastFeatureParity):
    """Exercise fresh BattleExamples wrapping the same append-only dicts."""
    sample_checks = 0
    tensor_checks = 0

    @classmethod
    def setUpClass(cls):
        super().setUpClass()
        cls.reference = frozen_dataset()

    def setUp(self):
        with _CACHE_LOCK:
            _STREAM_CACHE.clear()

    @classmethod
    def tearDownClass(cls):
        print(f"Frozen encoder parity: {cls.sample_checks} sample pairs, "
              f"{cls.tensor_checks} tensor pairs checked")

    def compare(self, battle, index, clock=None, **kwargs):
        opts = dict(max_context=64, threat_dim=policy_dataset.THREAT_DIM,
                    min_context=0, observation_seconds=clock)
        opts.update(kwargs)
        expected = frozen_encode_online_sample(
            self.reference, battle, index, self.vocab, self.costs, **opts)
        actual = encode_online_sample(
            policy_dataset, battle, index, self.vocab, self.costs, **opts)
        type(self).sample_checks += 1
        self.assertEqual(expected is None, actual is None)
        if expected is None:
            return
        self.assertEqual(len(expected), 13)
        self.assertEqual(len(actual), 13)
        for position, (left, right) in enumerate(zip(expected, actual)):
            type(self).tensor_checks += 1
            self.assertEqual(left.dtype, right.dtype)
            self.assertEqual(left.shape, right.shape)
            if left.is_floating_point() and torch.isnan(left).any():
                self.assertTrue(torch.equal(torch.isnan(left), torch.isnan(right)))
                # torch.equal treats matching NaNs as unequal; compare their
                # actual bits too, including NaN payloads and signed zero.
                equal = torch.equal(left.contiguous().view(torch.uint8),
                                    right.contiguous().view(torch.uint8))
            else:
                equal = torch.equal(left, right)
            self.assertTrue(equal, f"tensor {position} differs: index={index}, "
                            f"clock={clock}, options={kwargs}")
        return actual

    @staticmethod
    def probe(events, side, clock, ident="incremental", decks=(HOG, MORTAR)):
        deck = decks[0] if side == "team" else decks[1]
        dummy = dict(seconds=clock + 0.25, side=side, event_type="card_play",
                     card=deck[0], x=4000, y=25000)
        return winner_dataset.BattleExample(ident, *decks, 1, tuple(events) + (dummy,))

    def test_random_append_interleaved_perspectives(self):
        rng = random.Random(718213)
        for trial in range(3):
            events = []
            seconds = 0.0
            # Long games cross both elixir boundaries and the context limit.
            for index in range(181):
                if index:
                    seconds += rng.choice((0., .01, .9, 1.4, 2.8, 7., 18.))
                    if index in (35, 36, 37, 70, 71, 72):
                        seconds = {35:119.999999, 36:120., 37:120.000001,
                                   70:239.999999, 71:240., 72:240.000001}[index]
                    side = rng.choice(("team", "opponent"))
                    deck = HOG if side == "team" else MORTAR
                    events.append(dict(seconds=seconds, side=side,
                        event_type=rng.choice(("card_play", "card_play", "ability_activation")),
                        card=rng.choice(deck), x=rng.choice((0, 9000, 18000, rng.randrange(18001))),
                        y=rng.choice((0, 13440, 14400, 17600, 18560, 32000, rng.randrange(32001))),
                        position_known=rng.choice((True, True, False))))
                # Both sides share the actual event dicts. Repeated observations
                # cannot commit quiet-interval gains into the cached fold.
                for side in ("team", "opponent"):
                    for quiet in (0., 13., 600.):
                        now = seconds + quiet
                        self.compare(self.probe(events, side, now, f"random-{trial}"),
                                     index, now)

    def test_rebuilds_rewind_and_mutable_inputs(self):
        events = [dict(row) for row in self.battle(96).events[:-1]]
        for side in ("team", "opponent"):
            def check(rows, **kw):
                now = max(float(row["seconds"]) for row in rows) + 12.
                return self.compare(self.probe(rows, side, now, **kw), len(rows), now)
            check(events)
            check(events[:31])  # rewind
            check(events)
            # Reused ID, same length, same last event, changed early history.
            changed = [dict(row) for row in events]
            changed[3]["x"] += 999
            check(changed)
            changed[0]["seconds"] += .01  # edit an existing dict in place
            check(changed)
            check([dict(row) for row in changed])  # equal copies are valid
            check(events, decks=(tuple(reversed(HOG)), MORTAR))
            check(events)
            # Cost and vocab identities are normally immutable. Edits must
            # invalidate too; restore shared test fixtures even on failure.
            old_cost = self.costs["hog-rider"]
            old_id = self.vocab.name_to_id["hog-rider"]
            try:
                self.costs["hog-rider"] = old_cost + 1
                check(events)
                self.vocab.name_to_id["hog-rider"] = old_id + 1
                check(events)
            finally:
                self.costs["hog-rider"] = old_cost
                self.vocab.name_to_id["hog-rider"] = old_id
            check(events)

    def test_observation_boundaries_context_and_threat_sizes(self):
        for end in (119.999999, 120., 120.000001, 239.999999, 240., 240.000001):
            events = [dict(row) for row in self.battle(6).events[:-1]]
            events[-1]["seconds"] = end
            for side in ("team", "opponent"):
                for now in (end, end + .001, end + 30., 1000.):
                    battle = self.probe(events, side, now)
                    for context, threat in ((0, 0), (1, 4), (4, 14), (64, 20), (-2, 0), (-64, 14)):
                        self.compare(battle, len(events), now,
                                     max_context=context, threat_dim=threat)

    def test_fallbacks_none_and_invalid_clock(self):
        battle = self.battle(4)
        for index, clock, minimum in ((0, 0., 0), (2, None, 8), (5, 100., 0),
                                      (4, float("nan"), 0), (4, float("inf"), 0),
                                      (4, float("-inf"), 0)):
            try:
                expected = frozen_encode_online_sample(self.reference, battle, index,
                    self.vocab, self.costs, min_context=minimum, observation_seconds=clock)
            except Exception as exc:
                with self.assertRaises(type(exc)):
                    encode_online_sample(policy_dataset, battle, index, self.vocab,
                        self.costs, min_context=minimum, observation_seconds=clock)
            else:
                self.compare(battle, index, clock, min_context=minimum, threat_dim=0)
        events = list(battle.events[:-1])
        for side in ("team", "opponent"):
            target = self.probe(events, side, 100.)
            target.events[-1]["card"] = "absent-card"
            self.compare(target, len(events), 100.)
        # Errors during snapshot/feed evict partially used states.
        with self.assertRaisesRegex(ValueError, "Observation precedes"):
            encode_online_sample(policy_dataset, battle, 4, self.vocab, self.costs,
                                 min_context=0, observation_seconds=0.)
        self.compare(battle, 4, 100.)

    def test_nan_positions_and_fractional_costs(self):
        events = [dict(row) for row in self.battle(70).events[:-1]]
        events[3]["x"] = float("nan")
        events[5]["y"] = float("nan")
        old = self.costs["hog-rider"]
        try:
            self.costs["hog-rider"] = 3.123456789
            for side in ("team", "opponent"):
                for n in (8, 32, 70):
                    self.compare(self.probe(events[:n], side, 400.), n, 400.)
        finally:
            self.costs["hog-rider"] = old

    def test_cache_bound_and_output_mutation(self):
        from rl import banana_fast_features as fast
        events = list(self.battle(8).events[:-1])
        for i in range(9):
            with patch.object(fast, "_CACHE_LIMIT", 4):
                self.compare(self.probe(events, "team", 40., f"bound-{i}"), 8, 40.)
                self.assertLessEqual(len(_STREAM_CACHE), 4)
        battle = self.probe(events, "team", 40.)
        result = self.compare(battle, 8, 40.)
        for tensor in result:
            tensor.zero_()
        self.compare(battle, 8, 40.)

    def test_incremental_fold_and_one_snapshot_per_call(self):
        from rl.banana_incremental_features import CausalState
        calls = dict(feed=0, global_row=0)
        feed, snapshot = CausalState.feed, CausalState.global_row
        def counted_feed(state, event):
            calls["feed"] += 1
            return feed(state, event)
        def counted_snapshot(state, now):
            calls["global_row"] += 1
            return snapshot(state, now)
        rows = list(self.battle(96).events[:-1])
        with patch.object(CausalState, "feed", counted_feed), \
             patch.object(CausalState, "global_row", counted_snapshot):
            for n in range(1, 97):
                self.compare(self.probe(rows[:n], "team", 400.), n, 400.)
                self.compare(self.probe(rows[:n], "team", 500.), n, 500.)
        self.assertEqual(calls, {"feed": 96, "global_row": 192})

    def test_aliased_last_event_and_stream_none_fallback(self):
        events = list(self.battle(8).events[:-1])
        events.append(events[3])
        battle = self.probe(events, "team", 400.)
        self.compare(battle, len(events), 400.)
        sentinel = object()
        with patch.object(policy_dataset, "_encode_causal_stream", return_value=None), \
             patch.object(policy_dataset, "encode_policy_sample", return_value=sentinel) as fallback:
            actual = encode_online_sample(policy_dataset, battle, len(events),
                self.vocab, self.costs, min_context=0, observation_seconds=400.)
        self.assertIs(actual, sentinel)
        fallback.assert_called_once()

    def test_dataset_vocab_and_cost_identities(self):
        events = list(self.battle(12).events[:-1])
        battle = self.probe(events, "team", 100.)
        self.compare(battle, 12, 100.)
        alternate_vocab = winner_dataset.CardVocab(list(reversed(HOG)))
        alternate_costs = dict(self.costs, **{"hog-rider": 6})
        for dataset, vocab, costs in ((policy_dataset, alternate_vocab, self.costs),
                                      (policy_dataset, self.vocab, alternate_costs),
                                      (frozen_dataset(), self.vocab, self.costs)):
            left = frozen_encode_online_sample(self.reference, battle, 12, vocab, costs,
                                                min_context=0, observation_seconds=100.)
            right = encode_online_sample(dataset, battle, 12, vocab, costs,
                                         min_context=0, observation_seconds=100.)
            type(self).sample_checks += 1
            for a, b in zip(left, right):
                type(self).tensor_checks += 1
                self.assertTrue(torch.equal(a, b))
        self.assertEqual(len(_STREAM_CACHE), 4)


if __name__ == "__main__":
    unittest.main()
