"""Exact incremental online feature encoding for native Banana rollouts."""
from __future__ import annotations

import math
from collections import OrderedDict
from dataclasses import replace
from threading import RLock
from typing import Any

import numpy as np
import torch

try:
    from .banana_incremental_features import CausalState
except ImportError:  # Direct imports with engine/rl on PYTHONPATH.
    from banana_incremental_features import CausalState

# Bound retained games. Prefix/deck/config snapshots also detect id reuse,
# non-prefix input, and edits to caller-owned dictionaries. Prefix comparison
# is a cheap O(n) guard; all feature arithmetic folds only newly appended rows.
_CACHE_LIMIT = 128
_STREAM_CACHE: OrderedDict[tuple, CausalState] = OrderedDict()
_CACHE_LOCK = RLock()


def _incremental_stream(dataset, battle, event_index, vocab, costs, swap, now, max_context):
    # The reference tests `event is events[-1]`, rather than an index. If a
    # caller repeats the final dict object earlier in the prefix, it applies
    # the observation gain there too. Preserve that unusual input verbatim.
    last = battle.events[event_index - 1]
    if any(battle.events[index] is last for index in range(event_index - 1)):
        stream = dataset._encode_causal_stream(
            replace(battle, events=battle.events[:event_index]), vocab, costs,
            swap_sides=swap, min_events=1, observation_seconds=now,
        )
        if stream is None:
            return None
        continuous, card_ids, team_deck, opponent_deck, globals_all = stream
        if continuous.size(0) > max_context:
            continuous, card_ids = continuous[-max_context:], card_ids[-max_context:]
        cycle = dataset.acting_cycle_features(
            battle, event_index, costs, swap, observation_seconds=now,
        )
        return continuous, card_ids, team_deck, opponent_deck, globals_all[-1], cycle
    key = (battle.battle_id, swap, id(dataset), id(vocab), id(costs))
    with _CACHE_LOCK:
        state = _STREAM_CACHE.get(key)
        if (state is None or state.decks != (battle.team_deck, battle.opponent_deck)
                or len(state.events) > event_index
                or state.cost_values != costs
                or state.vocab_values != vocab.name_to_id or state.vocab_unk != vocab.unk_id
                or any(event != snapshot for event, snapshot in
                       zip(battle.events, state.event_values))):
            state = CausalState(dataset, battle, vocab, costs, swap)
            _STREAM_CACHE[key] = state
        _STREAM_CACHE.move_to_end(key)
        while len(_STREAM_CACHE) > _CACHE_LIMIT:
            _STREAM_CACHE.popitem(last=False)
        try:
            for index in range(len(state.events), event_index):
                state.feed(battle.events[index])
            # Slicing follows the old `if size > max_context` semantics even
            # for zero/negative context lengths; create no discarded tensors.
            rows, ids = state.continuous_rows, state.card_id_rows
            if len(rows) > max_context:
                rows, ids = rows[-max_context:], ids[-max_context:]
            continuous = (torch.tensor(rows, dtype=torch.float32) if rows else
                          torch.empty((0, dataset.CONTINUOUS_DIM), dtype=torch.float32))
            card_ids = torch.tensor(ids, dtype=torch.long)
            global_feat = state.global_row(now)
            cycle = state.cycle_features(now)
            # Cached deck tensors must not be exposed to caller mutation.
            return (continuous, card_ids, state.team_deck.clone(),
                    state.opponent_deck.clone(), global_feat, cycle)
        except BaseException:
            # A failed feed must never leave a partially updated fold reusable.
            _STREAM_CACHE.pop(key, None)
            raise


def encode_online_sample(
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
    """Return the same 13 tensors as the original one-pass online encoder.

    Kickoff, invalid-clock, and rejected-prefix cases delegate to the reference
    encoder. For ordinary prefixes, only newly confirmed events are folded and
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
    stream = _incremental_stream(
        dataset, battle, event_index, vocab, costs, swap, observed_now, max_context,
    )
    if stream is None:
        return fallback(
            battle, event_index, vocab, costs, max_context=max_context,
            threat_dim=threat_dim, reaction_seconds=reaction_seconds,
            min_context=min_context, observation_seconds=observation_seconds,
        )
    continuous, card_ids, team_deck, opponent_deck, global_feat, cycle = stream

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
    slot_feats, hand_mask = cycle
    from cr_replay_pipeline.policy_model import xy_to_zone

    return (
        continuous, card_ids, team_deck, opponent_deck, global_feat,
        slot_feats, hand_mask, torch.tensor(slot, dtype=torch.long),
        torch.tensor(event_type, dtype=torch.long), torch.tensor(xy_to_zone(nx, ny), dtype=torch.long),
        torch.tensor([nx, ny], dtype=torch.float32),
        torch.tensor(float(np.log1p(delta)), dtype=torch.float32),
        torch.tensor(continuous.size(0), dtype=torch.long),
    )
