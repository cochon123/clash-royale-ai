"""Behavior-cloning dataset: action history -> next action (deck-slot constrained)."""

from __future__ import annotations

import random
from typing import Any

import numpy as np
import torch
from torch.nn.utils.rnn import pad_sequence
from torch.utils.data import DataLoader, Dataset

from .winner_dataset import (
    CONTINUOUS_DIM,
    DEFAULT_ELIXIR_COST,
    GLOBAL_DIM,
    SPELL_CARDS,
    WIN_CONDITIONISH,
    BattleExample,
    CardVocab,
    _deck_ids,
    _deck_stats,
    _elixir_gain_rate,
    _event_cost,
    _window_sum,
    build_vocab,
    collect_battles,
    load_card_costs,
    split_battles,
    summarize_split,
)

__all__ = [
    "CONTINUOUS_DIM",
    "GLOBAL_DIM",
    "THREAT_DIM",
    "PolicyActionDataset",
    "build_vocab",
    "collect_battles",
    "collate_policy_batch",
    "create_policy_dataloaders",
    "load_card_costs",
    "split_battles",
    "summarize_split",
    "deck_slot_for_card",
    "encode_policy_sample",
    "recent_opponent_threat",
]

DEFAULT_MAX_CONTEXT = 64
DEFAULT_MIN_CONTEXT = 8
TILE_UNITS = 1000.0
SLOT_FEAT_DIM = 8

# Focus threats for v3 conditioning (aligned with defense probes).
THREAT_FOCUS = ("hog-rider", "balloon", "graveyard", "golem")
# 4 threats × (active, age) + any_focus + focus_cost + any_wincon + wincon_age +
# threat_x + threat_y = 14
THREAT_DIM = 14
DEFAULT_REACTION_SECONDS = 5.0


def deck_slot_for_card(deck: tuple[str, ...], card: str) -> int | None:
    try:
        return deck.index(card)
    except ValueError:
        return None


def _normalize_xy(x: int, y: int, swap_sides: bool) -> tuple[float, float]:
    nx = float(x) / 18000.0
    ny = float(y) / 32000.0
    if swap_sides:
        ny = 1.0 - ny
    return nx, ny


def recent_opponent_threat(
    battle: BattleExample,
    event_index: int,
    acting_side: str,
    costs: dict[str, int],
    max_age: float = DEFAULT_REACTION_SECONDS,
) -> tuple[torch.Tensor, bool]:
    """Threat conditioning features from the opponent's recent plays.

    Returns (threat_feat[THREAT_DIM], is_reaction) where is_reaction means a
    focus/wincon threat was played by the opponent within ``max_age`` seconds.
    """
    now = float(battle.events[event_index]["seconds"]) if event_index < len(battle.events) else 0.0
    # Prefix-only: look at events before the prediction index.
    prefix = battle.events[:event_index]
    opponent = "opponent" if acting_side == "team" else "team"

    last_by_focus: dict[str, dict[str, Any]] = {}
    last_wincon: dict[str, Any] | None = None
    for event in prefix:
        if event["side"] != opponent or event["event_type"] != "card_play":
            continue
        age = now - float(event["seconds"])
        if age < 0 or age > max_age:
            continue
        card = event["card"]
        if card in THREAT_FOCUS:
            prev = last_by_focus.get(card)
            if prev is None or float(event["seconds"]) >= float(prev["seconds"]):
                last_by_focus[card] = event
        if card in WIN_CONDITIONISH:
            if last_wincon is None or float(event["seconds"]) >= float(
                last_wincon["seconds"]
            ):
                last_wincon = event

    feats = [0.0] * THREAT_DIM
    any_focus = False
    focus_cost = 0.0
    latest_focus: dict[str, Any] | None = None
    for i, card in enumerate(THREAT_FOCUS):
        event = last_by_focus.get(card)
        if event is None:
            continue
        any_focus = True
        age = (now - float(event["seconds"])) / max_age
        feats[i * 2] = 1.0
        feats[i * 2 + 1] = min(max(age, 0.0), 1.0)
        cost = float(costs.get(card, DEFAULT_ELIXIR_COST)) / 10.0
        if latest_focus is None or float(event["seconds"]) >= float(
            latest_focus["seconds"]
        ):
            latest_focus = event
            focus_cost = cost

    feats[8] = 1.0 if any_focus else 0.0
    feats[9] = focus_cost
    if last_wincon is not None:
        feats[10] = 1.0
        feats[11] = min(
            max((now - float(last_wincon["seconds"])) / max_age, 0.0), 1.0
        )
        # Placement of most recent wincon threat (normalized, acting-side view).
        nx, ny = _normalize_xy(
            int(last_wincon["x"]),
            int(last_wincon["y"]),
            swap_sides=(acting_side == "opponent"),
        )
        feats[12] = nx
        feats[13] = ny
    elif latest_focus is not None:
        nx, ny = _normalize_xy(
            int(latest_focus["x"]),
            int(latest_focus["y"]),
            swap_sides=(acting_side == "opponent"),
        )
        feats[12] = nx
        feats[13] = ny

    is_reaction = any_focus or last_wincon is not None
    return torch.tensor(feats, dtype=torch.float32), is_reaction


def acting_cycle_features(
    battle: BattleExample,
    event_index: int,
    costs: dict[str, int],
    swap_sides: bool,
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
    now = float(battle.events[event_index - 1]["seconds"]) if event_index else 0.0
    for event in battle.events[:event_index]:
        if event["side"] != raw_side or event["event_type"] != "card_play":
            continue
        slot = deck_slot_for_card(acting_deck, event["card"])
        if slot is None:
            continue
        last_play[slot] = float(event["seconds"])
        play_count[slot] += 1.0
        now = float(event["seconds"])

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


def _encode_causal_stream(
    battle: BattleExample,
    vocab: CardVocab,
    costs: dict[str, int],
    swap_sides: bool = False,
    min_events: int | None = None,
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
        x = float(event["x"]) / 18000.0
        y = float(event["y"]) / 32000.0
        if swap_sides:
            y = 1.0 - y

        is_ability = 1.0 if event["event_type"] == "ability_activation" else 0.0
        is_spell = 1.0 if card in SPELL_CARDS else 0.0
        is_wincon = 1.0 if card in WIN_CONDITIONISH else 0.0
        is_deep = 0.0
        is_bridge = 1.0 if 0.42 <= y <= 0.58 else 0.0
        is_left = 1.0 if x < 0.5 else 0.0
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


def encode_policy_sample(
    battle: BattleExample,
    event_index: int,
    vocab: CardVocab,
    costs: dict[str, int],
    max_context: int = DEFAULT_MAX_CONTEXT,
    threat_dim: int = 0,
    reaction_seconds: float = DEFAULT_REACTION_SECONDS,
    min_context: int | None = None,
) -> tuple[torch.Tensor, ...] | None:
    """Encode prefix events[:event_index] predicting events[event_index]."""
    from .policy_model import xy_to_zone

    need = DEFAULT_MIN_CONTEXT if min_context is None else max(0, int(min_context))
    if event_index < need or event_index >= len(battle.events):
        return None

    target = battle.events[event_index]
    swap = target["side"] == "opponent"
    stream_min = 1 if need == 0 else need + 1
    stream = _encode_causal_stream(
        battle, vocab, costs, swap_sides=swap, min_events=stream_min
    )
    if stream is None:
        return None
    continuous, card_ids, team_deck, opponent_deck, globals_all = stream
    continuous = continuous[:event_index]
    card_ids = card_ids[:event_index]
    if event_index == 0:
        # Kickoff: no prior plays — pad a single empty token + starting globals.
        continuous = torch.zeros(1, CONTINUOUS_DIM, dtype=torch.float32)
        card_ids = torch.zeros(1, dtype=torch.long)
        global_feat = torch.zeros(GLOBAL_DIM, dtype=torch.float32)
        # Indices 18/19 are team/opp elixir (see causal stream).
        global_feat[18] = 0.5
        global_feat[19] = 0.5
    else:
        global_feat = globals_all[event_index - 1]
    if continuous.size(0) > max_context:
        continuous = continuous[-max_context:]
        card_ids = card_ids[-max_context:]

    acting_deck = battle.opponent_deck if swap else battle.team_deck
    slot = deck_slot_for_card(acting_deck, target["card"])
    if slot is None:
        return None

    if threat_dim > 0:
        threat_feat, _is_reaction = recent_opponent_threat(
            battle,
            event_index,
            acting_side=target["side"],
            costs=costs,
            max_age=reaction_seconds,
        )
        if threat_dim != THREAT_DIM:
            # Allow trunc/pad if config asks for a different size.
            if threat_feat.numel() >= threat_dim:
                threat_feat = threat_feat[:threat_dim]
            else:
                threat_feat = torch.nn.functional.pad(
                    threat_feat, (0, threat_dim - threat_feat.numel())
                )
        global_feat = torch.cat([global_feat, threat_feat], dim=-1)

    nx, ny = _normalize_xy(int(target["x"]), int(target["y"]), swap_sides=swap)
    if event_index == 0:
        prev_seconds = 0.0
    else:
        prev_seconds = float(battle.events[event_index - 1]["seconds"])
    delta = max(0.0, float(target["seconds"]) - prev_seconds)
    event_type = 1 if target["event_type"] == "ability_activation" else 0
    slot_feats, hand_mask = acting_cycle_features(battle, event_index, costs, swap)
    zone = xy_to_zone(nx, ny)

    return (
        continuous,
        card_ids,
        team_deck,
        opponent_deck,
        global_feat,
        slot_feats,
        hand_mask,
        torch.tensor(slot, dtype=torch.long),
        torch.tensor(event_type, dtype=torch.long),
        torch.tensor(zone, dtype=torch.long),
        torch.tensor([nx, ny], dtype=torch.float32),
        torch.tensor(float(np.log1p(delta)), dtype=torch.float32),
        torch.tensor(continuous.size(0), dtype=torch.long),
    )


class PolicyActionDataset(Dataset):
    """Stores one causal stream per battle perspective + a compact sample index.

    Peak RAM is roughly O(battles * events), not O(samples * context).
    """

    def __init__(
        self,
        battles: list[BattleExample],
        vocab: CardVocab,
        costs: dict[str, int],
        max_context: int = DEFAULT_MAX_CONTEXT,
        max_samples_per_battle: int | None = 32,
        stride: int = 2,
        seed: int = 42,
        threat_dim: int = 0,
        reaction_seconds: float = DEFAULT_REACTION_SECONDS,
        reaction_weight: float = 1.0,
        prefer_reactions: bool = False,
        reaction_repeats: int = 1,
    ):
        self.max_context = max_context
        self.battles = battles
        self.costs = costs
        self.threat_dim = threat_dim
        self.reaction_seconds = reaction_seconds
        self.reaction_weight = reaction_weight
        self.index: list[tuple[int, int]] = []
        self._streams: dict[
            tuple[int, bool],
            tuple[torch.Tensor, torch.Tensor, torch.Tensor, torch.Tensor, torch.Tensor],
        ] = {}
        rng = random.Random(seed)

        for battle_i, battle in enumerate(battles):
            candidates = list(range(DEFAULT_MIN_CONTEXT, len(battle.events), stride))
            if not candidates:
                continue

            # Tag reaction windows before subsampling so they are not starved.
            reaction: list[int] = []
            other: list[int] = []
            for event_index in candidates:
                target = battle.events[event_index]
                acting = (
                    battle.opponent_deck
                    if target["side"] == "opponent"
                    else battle.team_deck
                )
                if deck_slot_for_card(acting, target["card"]) is None:
                    continue
                if prefer_reactions:
                    _feat, is_react = recent_opponent_threat(
                        battle,
                        event_index,
                        acting_side=target["side"],
                        costs=costs,
                        max_age=reaction_seconds,
                    )
                    if is_react:
                        reaction.append(event_index)
                        continue
                other.append(event_index)

            if prefer_reactions and max_samples_per_battle is not None:
                kept = list(reaction)
                remaining = max(0, max_samples_per_battle - len(kept))
                if remaining and len(other) > remaining:
                    kept.extend(sorted(rng.sample(other, remaining)))
                else:
                    kept.extend(other[:remaining] if remaining else [])
                    if not remaining and not kept:
                        kept = other[:max_samples_per_battle]
                # If still under budget, pad with non-reactions already unused.
                if len(kept) < max_samples_per_battle and other:
                    unused = [i for i in other if i not in set(kept)]
                    need = max_samples_per_battle - len(kept)
                    if unused:
                        kept.extend(
                            sorted(rng.sample(unused, min(need, len(unused))))
                        )
                indices = sorted(set(kept))
            else:
                indices = reaction + other
                if (
                    max_samples_per_battle is not None
                    and len(indices) > max_samples_per_battle
                ):
                    indices = sorted(rng.sample(indices, max_samples_per_battle))

            kept = indices
            if not kept:
                continue

            for swap in {battle.events[i]["side"] == "opponent" for i in kept}:
                stream = _encode_causal_stream(battle, vocab, costs, swap_sides=swap)
                if stream is not None:
                    self._streams[(battle_i, swap)] = stream

            for event_index in kept:
                swap = battle.events[event_index]["side"] == "opponent"
                if (battle_i, swap) not in self._streams:
                    continue
                repeats = 1
                if prefer_reactions and reaction_repeats > 1:
                    target = battle.events[event_index]
                    _feat, is_react = recent_opponent_threat(
                        battle,
                        event_index,
                        acting_side=target["side"],
                        costs=costs,
                        max_age=reaction_seconds,
                    )
                    if is_react:
                        repeats = reaction_repeats
                for _ in range(repeats):
                    self.index.append((battle_i, event_index))

    def __len__(self) -> int:
        return len(self.index)

    def __getitem__(self, index: int):
        from .policy_model import xy_to_zone

        battle_i, event_index = self.index[index]
        battle = self.battles[battle_i]
        target = battle.events[event_index]
        swap = target["side"] == "opponent"
        continuous, card_ids, team_deck, opponent_deck, globals_all = self._streams[
            (battle_i, swap)
        ]
        continuous = continuous[:event_index]
        card_ids = card_ids[:event_index]
        global_feat = globals_all[event_index - 1]
        if continuous.size(0) > self.max_context:
            continuous = continuous[-self.max_context :]
            card_ids = card_ids[-self.max_context :]

        weight = 1.0
        if self.threat_dim > 0 or self.reaction_weight != 1.0:
            threat_feat, is_react = recent_opponent_threat(
                battle,
                event_index,
                acting_side=target["side"],
                costs=self.costs,
                max_age=self.reaction_seconds,
            )
            if self.threat_dim > 0:
                if threat_feat.numel() >= self.threat_dim:
                    threat_feat = threat_feat[: self.threat_dim]
                else:
                    threat_feat = torch.nn.functional.pad(
                        threat_feat, (0, self.threat_dim - threat_feat.numel())
                    )
                global_feat = torch.cat([global_feat, threat_feat], dim=-1)
            if is_react:
                weight = float(self.reaction_weight)

        acting_deck = battle.opponent_deck if swap else battle.team_deck
        slot = deck_slot_for_card(acting_deck, target["card"])
        assert slot is not None
        nx, ny = _normalize_xy(int(target["x"]), int(target["y"]), swap_sides=swap)
        prev_seconds = float(battle.events[event_index - 1]["seconds"])
        delta = max(0.0, float(target["seconds"]) - prev_seconds)
        event_type = 1 if target["event_type"] == "ability_activation" else 0
        slot_feats, hand_mask = acting_cycle_features(
            battle, event_index, self.costs, swap
        )
        zone = xy_to_zone(nx, ny)

        return (
            continuous,
            card_ids,
            team_deck,
            opponent_deck,
            global_feat,
            slot_feats,
            hand_mask,
            torch.tensor(slot, dtype=torch.long),
            torch.tensor(event_type, dtype=torch.long),
            torch.tensor(zone, dtype=torch.long),
            torch.tensor([nx, ny], dtype=torch.float32),
            torch.tensor(float(np.log1p(delta)), dtype=torch.float32),
            int(continuous.size(0)),
            torch.tensor(weight, dtype=torch.float32),
        )


def collate_policy_batch(batch):
    (
        sequences,
        card_ids,
        team_decks,
        opponent_decks,
        globals_,
        slot_feats,
        hand_masks,
        slots,
        types,
        zones,
        xy,
        timing,
        lengths,
        weights,
    ) = zip(*batch)
    return (
        pad_sequence(sequences, batch_first=True, padding_value=0.0),
        pad_sequence(card_ids, batch_first=True, padding_value=0),
        torch.stack(team_decks),
        torch.stack(opponent_decks),
        torch.stack(globals_),
        torch.stack(slot_feats),
        torch.stack(hand_masks),
        torch.stack(slots),
        torch.stack(types),
        torch.stack(zones),
        torch.stack(xy),
        torch.stack(timing),
        torch.tensor(lengths, dtype=torch.long),
        torch.stack(weights),
    )


def create_policy_dataloaders(
    train_battles: list[BattleExample],
    val_battles: list[BattleExample],
    test_battles: list[BattleExample],
    vocab: CardVocab,
    costs: dict[str, int],
    batch_size: int = 256,
    max_context: int = DEFAULT_MAX_CONTEXT,
    max_samples_per_battle: int | None = 40,
    threat_dim: int = 0,
    reaction_seconds: float = DEFAULT_REACTION_SECONDS,
    reaction_weight: float = 1.0,
    reaction_repeats: int = 1,
) -> tuple[DataLoader, DataLoader, DataLoader]:
    train_ds = PolicyActionDataset(
        train_battles,
        vocab,
        costs,
        max_context=max_context,
        max_samples_per_battle=max_samples_per_battle,
        stride=2,
        seed=11,
        threat_dim=threat_dim,
        reaction_seconds=reaction_seconds,
        reaction_weight=reaction_weight,
        prefer_reactions=threat_dim > 0 or reaction_weight > 1.0 or reaction_repeats > 1,
        reaction_repeats=reaction_repeats,
    )
    val_ds = PolicyActionDataset(
        val_battles,
        vocab,
        costs,
        max_context=max_context,
        max_samples_per_battle=min(24, max_samples_per_battle or 24),
        stride=3,
        seed=22,
        threat_dim=threat_dim,
        reaction_seconds=reaction_seconds,
        reaction_weight=1.0,
        prefer_reactions=False,
        reaction_repeats=1,
    )
    test_ds = PolicyActionDataset(
        test_battles,
        vocab,
        costs,
        max_context=max_context,
        max_samples_per_battle=min(24, max_samples_per_battle or 24),
        stride=3,
        seed=33,
        threat_dim=threat_dim,
        reaction_seconds=reaction_seconds,
        reaction_weight=1.0,
        prefer_reactions=False,
        reaction_repeats=1,
    )
    return (
        DataLoader(
            train_ds,
            batch_size=batch_size,
            shuffle=True,
            collate_fn=collate_policy_batch,
            num_workers=0,
        ),
        DataLoader(
            val_ds,
            batch_size=batch_size,
            shuffle=False,
            collate_fn=collate_policy_batch,
            num_workers=0,
        ),
        DataLoader(
            test_ds,
            batch_size=batch_size,
            shuffle=False,
            collate_fn=collate_policy_batch,
            num_workers=0,
        ),
    )


def baseline_frequency_slot(battles: list[BattleExample]) -> dict[str, Any]:
    correct = 0
    top3 = 0
    total = 0
    for battle in battles:
        for side, deck in (("team", battle.team_deck), ("opponent", battle.opponent_deck)):
            counts = {card: 0 for card in deck}
            side_events = [
                event
                for event in battle.events
                if event["side"] == side and event["event_type"] == "card_play"
            ]
            for event in side_events:
                if not counts:
                    continue
                ranked = sorted(counts.keys(), key=lambda c: (-counts[c], deck.index(c)))
                pred = ranked[0]
                total += 1
                if pred == event["card"]:
                    correct += 1
                if event["card"] in ranked[:3]:
                    top3 += 1
                if event["card"] in counts:
                    counts[event["card"]] += 1
    return {
        "name": "online_frequency",
        "slot_top1": correct / max(total, 1),
        "slot_top3": top3 / max(total, 1),
        "n": total,
    }


def baseline_cycle_slot(battles: list[BattleExample]) -> dict[str, Any]:
    correct = 0
    top3 = 0
    total = 0
    for battle in battles:
        for side, deck in (("team", battle.team_deck), ("opponent", battle.opponent_deck)):
            seen: list[str] = []
            hand: list[str] = []
            queue: list[str] = []
            counts = {card: 0 for card in deck}
            for event in battle.events:
                if event["side"] != side or event["event_type"] != "card_play":
                    continue
                card = event["card"]
                if len(hand) < 4:
                    ranked = sorted(
                        counts.keys(), key=lambda c: (-counts[c], deck.index(c))
                    )
                    preds = ranked[:3]
                else:
                    preds = list(hand[:3]) if len(hand) >= 3 else list(hand)
                total += 1
                if preds and preds[0] == card:
                    correct += 1
                if card in preds:
                    top3 += 1
                if card in counts:
                    counts[card] += 1
                if card not in seen and len(seen) < 8:
                    seen.append(card)
                if len(hand) < 4:
                    if card in deck and card not in hand and card not in queue:
                        remaining = [c for c in deck if c not in seen]
                        if len(seen) == 4 and not hand:
                            hand = list(seen)
                            queue = remaining[:4]
                    continue
                if card in hand:
                    hand.remove(card)
                    if queue:
                        hand.append(queue.pop(0))
                    queue.append(card)
                elif len(hand) + len(queue) < 8:
                    if not hand:
                        hand = list(seen[:4])
                        queue = [c for c in deck if c not in hand][:4]
                    if card in hand:
                        hand.remove(card)
                        if queue:
                            hand.append(queue.pop(0))
                        queue.append(card)
    return {
        "name": "approx_cycle",
        "slot_top1": correct / max(total, 1),
        "slot_top3": top3 / max(total, 1),
        "n": total,
    }
