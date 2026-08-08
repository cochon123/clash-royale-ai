"""Load a trained policy and predict the next action from an event prefix."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import numpy as np
import torch

from .policy_dataset import GLOBAL_DIM, encode_policy_sample
from .policy_model import PolicyBC, xy_to_zone
from .winner_dataset import BattleExample, CardVocab, load_card_costs


def load_policy(
    model_dir: str | Path = "models/policy_bc",
    device_name: str | None = None,
) -> tuple[PolicyBC, CardVocab, dict[str, Any], torch.device]:
    model_dir = Path(model_dir)
    ckpt = torch.load(model_dir / "best_model.pt", map_location="cpu", weights_only=False)
    vocab = CardVocab.from_dict(ckpt["vocab"])
    cfg = ckpt.get("config", {})
    device = torch.device(
        device_name
        if device_name
        else ("cuda" if torch.cuda.is_available() else "cpu")
    )
    threat_dim = int(cfg.get("threat_dim", 0))
    global_dim = int(cfg.get("global_dim", GLOBAL_DIM + threat_dim))
    model = PolicyBC(
        vocab_size=vocab.vocab_size,
        global_dim=global_dim,
        d_model=int(cfg.get("d_model", 160)),
        num_layers=int(cfg.get("num_layers", 2)),
        dropout=float(cfg.get("dropout", 0.2)),
        card_conditioned_placement=bool(cfg.get("card_conditioned_placement", False)),
    )
    model.load_state_dict(ckpt["model_state"])
    model.to(device)
    model.eval()
    return model, vocab, cfg, device


@torch.no_grad()
def predict_next_action(
    model: PolicyBC,
    vocab: CardVocab,
    costs: dict[str, int],
    battle: BattleExample,
    device: torch.device,
    acting_side: str = "team",
    temperature: float = 1.0,
    max_context: int = 64,
    threat_dim: int = 0,
    min_context: int | None = None,
    prefer_cards: set[str] | None = None,
) -> dict[str, Any]:
    """Predict the next action as if ``acting_side`` is about to play.

    ``min_context=0`` allows live kickoff with an empty event history.
    ``prefer_cards`` (normalized names) biases argmax toward cards currently
    observed in hand (e.g. YOLO detections).
    """
    acting_deck = battle.team_deck if acting_side == "team" else battle.opponent_deck
    if not acting_deck:
        raise ValueError("acting deck is empty")

    # Probe with a dummy next event on the requested side so encoding swaps correctly.
    if battle.events:
        last_seconds = float(battle.events[-1]["seconds"])
    else:
        last_seconds = 0.0
    dummy = {
        "seconds": last_seconds + 1.0,
        "side": acting_side,
        "event_type": "card_play",
        "card": acting_deck[0],
        "x": 9000,
        "y": 8000 if acting_side == "team" else 24000,
    }
    probe = BattleExample(
        battle_id=battle.battle_id,
        team_deck=battle.team_deck,
        opponent_deck=battle.opponent_deck,
        team_wins=battle.team_wins,
        events=tuple(battle.events) + (dummy,),
    )
    sample = encode_policy_sample(
        probe,
        len(battle.events),
        vocab,
        costs,
        max_context=max_context,
        threat_dim=threat_dim,
        min_context=min_context,
    )
    if sample is None:
        raise RuntimeError("Could not encode policy prefix")

    (
        continuous,
        card_ids,
        team_deck,
        opp_deck,
        global_feat,
        slot_feats,
        hand_mask,
        _slot,
        _type,
        _zone,
        _xy,
        _timing,
        length,
    ) = sample

    if prefer_cards:
        preferred = torch.zeros(8, dtype=torch.bool)
        for i, name in enumerate(acting_deck):
            if name in prefer_cards:
                preferred[i] = True
        if preferred.any():
            hand_mask = preferred

    out = model(
        continuous.unsqueeze(0).to(device),
        card_ids.unsqueeze(0).to(device),
        team_deck.unsqueeze(0).to(device),
        opp_deck.unsqueeze(0).to(device),
        global_feat.unsqueeze(0).to(device),
        length.unsqueeze(0).to(device),
        slot_feats.unsqueeze(0).to(device),
        hand_mask.unsqueeze(0).to(device),
    )

    logits = out["slot_logits"][0] / max(temperature, 1e-3)
    probs = torch.softmax(logits, dim=-1).cpu().numpy()
    slot = int(probs.argmax())
    card = acting_deck[slot]
    event_type = (
        "ability_activation"
        if int(out["type_logits"][0].argmax().item()) == 1
        else "card_play"
    )
    zone = int(out["zone_logits"][0].argmax().item())
    xy = out["xy"][0].cpu().numpy()
    x_norm, y_norm = float(xy[0]), float(xy[1])
    if acting_side == "opponent":
        y_norm = 1.0 - y_norm
    x = int(np.clip(x_norm * 18000.0, 3000, 15000))
    y = int(np.clip(y_norm * 32000.0, 500, 31500))
    if event_type == "ability_activation":
        x, y = 9000, 16000
    dt = float(np.clip(np.expm1(out["timing"][0].item()), 0.2, 12.0))

    ranked = sorted(
        [
            {"slot": i, "card": acting_deck[i], "prob": float(probs[i])}
            for i in range(8)
        ],
        key=lambda row: -row["prob"],
    )

    return {
        "card": card,
        "slot": slot,
        "event_type": event_type,
        "x": x,
        "y": y,
        "zone": zone,
        "zone_from_xy": xy_to_zone(float(xy[0]), float(xy[1])),
        "delay_seconds": dt,
        "top3": ranked[:3],
        "ranked_slots": ranked,
        "hand_mask": [bool(v) for v in hand_mask.tolist()],
    }


def demo_predict_from_raw(
    raw_path: str | Path,
    model_dir: str | Path = "models/policy_bc",
    card_costs_path: str | Path = "data/card_costs.json",
    prefix_events: int = 20,
    acting_side: str = "team",
    device_name: str | None = None,
) -> dict[str, Any]:
    from .parser import parse_replay

    model, vocab, cfg, device = load_policy(model_dir, device_name=device_name)
    costs = load_card_costs(card_costs_path)
    replay = parse_replay(raw_path)
    events = []
    for event in replay.events:
        if event.event_type == "card_play" and event.card and event.x is not None:
            events.append(
                {
                    "seconds": float(event.seconds),
                    "side": event.side,
                    "event_type": "card_play",
                    "card": event.card,
                    "x": int(event.x),
                    "y": int(event.y),
                }
            )
        elif event.event_type == "ability_activation":
            events.append(
                {
                    "seconds": float(event.seconds),
                    "side": event.side,
                    "event_type": "ability_activation",
                    "card": event.ability_card or event.card or "_ability",
                    "x": 9000,
                    "y": 16000,
                }
            )
    events = events[: max(8, prefix_events)]
    battle = BattleExample(
        battle_id=replay.battle_id,
        team_deck=tuple(replay.decks.get("team") or ()),
        opponent_deck=tuple(replay.decks.get("opponent") or ()),
        team_wins=1,
        events=tuple(events),
    )
    prediction = predict_next_action(
        model,
        vocab,
        costs,
        battle,
        device,
        acting_side=acting_side,
        max_context=int(cfg.get("max_context", 64)),
        threat_dim=int(cfg.get("threat_dim", 0)),
    )
    return {
        "battle_id": replay.battle_id,
        "prefix_events": len(events),
        "acting_side": acting_side,
        "prediction": prediction,
    }
