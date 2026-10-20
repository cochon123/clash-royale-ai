"""Exact one-pass online feature encoding for native Banana rollouts."""
from __future__ import annotations

import math
from dataclasses import replace
from typing import Any

import numpy as np
import torch


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
