"""Load a trained policy and predict the next action from an event prefix."""

from __future__ import annotations

import json
import random
from pathlib import Path
from typing import Any

import numpy as np
import torch

from .policy_dataset import GLOBAL_DIM, encode_policy_sample
from .policy_model import PolicyBC, xy_to_zone
from .winner_dataset import BattleExample, CardVocab, load_card_costs


def rollout_decode_settings(cfg: dict[str, Any]) -> dict[str, Any]:
    """Resolve the decoder a checkpoint was evaluated with.

    Older heatmap checkpoints did not persist these fields; v4.4's intended
    deployment decode was tile argmax, while continuous-XY models use expected.
    """
    placement_mode = str(cfg.get("placement_mode", "xy"))
    return {
        "slot_decode": str(cfg.get("rollout_slot_decode", "sample")),
        "temperature": float(cfg.get("rollout_slot_temperature", 0.8)),
        "placement_decode": str(
            cfg.get("rollout_placement_decode")
            or ("argmax" if placement_mode == "heatmap" else "expected")
        ),
        "placement_temperature": float(cfg.get("rollout_placement_temperature") or 1.0),
        "placement_top_k": cfg.get("rollout_placement_top_k"),
        "think_steps": int(cfg.get("eval_think_steps", cfg.get("max_think_steps", 0))),
    }


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
        placement_mode=str(cfg.get("placement_mode", "xy")),
        placement_card_mode=str(cfg.get("placement_card_mode", "soft")),
        arena_memory_channels=int(cfg.get("arena_memory_channels", 0)),
        arena_hidden_channels=int(cfg.get("arena_hidden_channels", 32)),
        arena_memory_version=str(cfg.get("arena_memory_version", "none")),
        arena_gate_bias=float(cfg.get("arena_gate_bias", -2.2)),
        max_think_steps=int(cfg.get("max_think_steps", 0)),
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
    slot_decode: str = "argmax",
    max_context: int = 64,
    threat_dim: int = 0,
    min_context: int | None = None,
    prefer_cards: set[str] | None = None,
    placement_decode: str = "expected",
    placement_temperature: float = 1.0,
    placement_top_k: int | None = None,
    mirror_tta: bool = False,
    think_steps: int | None = None,
    now_seconds: float | None = None,
    rng: random.Random | None = None,
) -> dict[str, Any]:
    """Predict the next action as if ``acting_side`` is about to play.

    ``min_context=0`` allows live kickoff with an empty event history.
    ``prefer_cards`` is a hard execution mask for cards currently observed in
    hand. ``now_seconds`` timestamps the live probe; the current architecture
    remains causal and encodes only confirmed events before that probe.
    ``think_steps`` toggles latent refine compute: ``0`` is off/fast, while
    ``None`` uses the model's trained maximum (the evaluation convention).
    """
    acting_deck = battle.team_deck if acting_side == "team" else battle.opponent_deck
    if not acting_deck:
        raise ValueError("acting deck is empty")

    # Probe with a dummy next event on the requested side so encoding swaps correctly.
    if battle.events:
        last_seconds = float(battle.events[-1]["seconds"])
    else:
        last_seconds = 0.0
    observed_now = last_seconds + 1.0 if now_seconds is None else float(now_seconds)
    observed_now = max(last_seconds + 1e-3, observed_now)
    dummy = {
        "seconds": observed_now,
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

    legal_slots: list[int] | None = None
    if prefer_cards:
        normalized_preferred = {str(name).strip().lower().replace("_", "-") for name in prefer_cards}
        preferred = torch.zeros(8, dtype=torch.bool)
        for i, name in enumerate(acting_deck):
            if name in normalized_preferred:
                preferred[i] = True
        if preferred.any():
            hand_mask = preferred
            legal_slots = preferred.nonzero(as_tuple=False).flatten().tolist()
        else:
            raise ValueError("none of prefer_cards exists in the acting deck")

    resolved_think = (
        int(getattr(model, "max_think_steps", 0))
        if think_steps is None
        else int(think_steps)
    )
    mirror_inputs = None
    if mirror_tta:
        # Evaluate the exact horizontal reflection used during v4.2 training,
        # map its spatial outputs back, then average both probability views.
        # Imports stay local to keep the ordinary one-pass inference path lean.
        from .policy_train import _MirroredBattle
        from .policy_tta import mirror_ensemble_outputs

        mirrored_sample = encode_policy_sample(
            _MirroredBattle(probe),
            len(battle.events),
            vocab,
            costs,
            max_context=max_context,
            threat_dim=threat_dim,
            min_context=min_context,
        )
        if mirrored_sample is None:
            raise RuntimeError("Could not encode mirrored policy prefix")
        (
            mirror_continuous,
            mirror_card_ids,
            mirror_team_deck,
            mirror_opp_deck,
            mirror_global_feat,
            mirror_slot_feats,
            mirror_hand_mask,
            _mirror_slot,
            _mirror_type,
            _mirror_zone,
            _mirror_xy,
            _mirror_timing,
            mirror_length,
        ) = mirrored_sample
        if legal_slots is not None:
            mirror_hand_mask = hand_mask
        mirror_inputs = (
            mirror_continuous, mirror_card_ids, mirror_team_deck, mirror_opp_deck,
            mirror_global_feat, mirror_length, mirror_slot_feats, mirror_hand_mask,
        )

    def forward(placement_slot: int | None = None) -> dict[str, torch.Tensor]:
        placement_slots = (
            torch.tensor([placement_slot], device=device)
            if placement_slot is not None else None
        )
        direct = model(
            continuous.unsqueeze(0).to(device), card_ids.unsqueeze(0).to(device),
            team_deck.unsqueeze(0).to(device), opp_deck.unsqueeze(0).to(device),
            global_feat.unsqueeze(0).to(device), length.unsqueeze(0).to(device),
            slot_feats.unsqueeze(0).to(device), hand_mask.unsqueeze(0).to(device),
            placement_slots=placement_slots, think_steps=resolved_think,
        )
        if mirror_inputs is None:
            return direct
        mc, mi, mt, mo, mg, ml, ms, mh = mirror_inputs
        reflected = model(
            mc.unsqueeze(0).to(device), mi.unsqueeze(0).to(device),
            mt.unsqueeze(0).to(device), mo.unsqueeze(0).to(device),
            mg.unsqueeze(0).to(device), ml.unsqueeze(0).to(device),
            ms.unsqueeze(0).to(device), mh.unsqueeze(0).to(device),
            placement_slots=placement_slots, think_steps=resolved_think,
        )
        return mirror_ensemble_outputs(direct, reflected)

    out = forward()

    if placement_decode not in {"expected", "argmax", "sample"}:
        raise ValueError("placement_decode must be expected, argmax, or sample")
    if slot_decode not in {"argmax", "sample"}:
        raise ValueError("slot_decode must be argmax or sample")

    logits = out["slot_logits"][0] / max(temperature, 1e-3)
    probs = torch.softmax(logits, dim=-1).cpu().numpy()
    selection_probs = probs.copy()
    if legal_slots is not None:
        legal = np.zeros(8, dtype=bool)
        legal[legal_slots] = True
        selection_probs[~legal] = 0.0
        selection_probs /= selection_probs.sum()
    if slot_decode == "sample":
        if rng is None:
            slot = int(torch.multinomial(torch.from_numpy(selection_probs), 1).item())
        else:
            slot = int(rng.choices(range(8), weights=selection_probs.tolist(), k=1)[0])
    else:
        slot = int(selection_probs.argmax())
    card = acting_deck[slot]
    # v4.4.1's placement head is explicitly conditioned on the selected card.
    # The first pass selects the slot; this second pass produces that card's
    # type/zone/tile/timing instead of the soft pre-choice placement.
    if getattr(model, "placement_card_mode", "soft") == "selected":
        out = forward(slot)
    event_type = (
        "ability_activation"
        if int(out["type_logits"][0].argmax().item()) == 1
        else "card_play"
    )
    zone = int(out["zone_logits"][0].argmax().item())
    xy = out["xy"][0].cpu().numpy()
    selected_tile = None
    tile_logits = out.get("tile_logits")
    if tile_logits is not None and placement_decode != "expected":
        tile_probs = torch.softmax(tile_logits[0] / max(placement_temperature, 1e-3), dim=-1)
        if placement_decode == "argmax":
            selected_tile = int(tile_probs.argmax().item())
        else:
            if placement_top_k is not None:
                k = max(1, min(int(placement_top_k), int(tile_probs.numel())))
                values, indices = torch.topk(tile_probs, k)
                if rng is None:
                    chosen = int(torch.multinomial(values, 1).item())
                else:
                    chosen = int(rng.choices(range(k), weights=values.cpu().tolist(), k=1)[0])
                selected_tile = int(indices[chosen].item())
            else:
                if rng is None:
                    selected_tile = int(torch.multinomial(tile_probs, 1).item())
                else:
                    selected_tile = int(rng.choices(range(int(tile_probs.numel())), weights=tile_probs.cpu().tolist(), k=1)[0])
        rows, cols = divmod(selected_tile, 32)
        xy = np.asarray([(cols + 0.5) / 32.0, (rows + 0.5) / 18.0], dtype=np.float32)
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
            {
                "slot": i,
                "card": acting_deck[i],
                "prob": float(selection_probs[i]),
                "raw_prob": float(probs[i]),
                "legal": legal_slots is None or i in legal_slots,
            }
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
        "placement_decode": placement_decode,
        "slot_decode": slot_decode,
        "slot_temperature": float(temperature),
        "placement_temperature": float(placement_temperature),
        "placement_top_k": placement_top_k,
        "mirror_tta": bool(mirror_tta),
        "think_steps": resolved_think,
        "tile": selected_tile,
        "top3": ranked[:3],
        "ranked_slots": ranked,
        "hand_mask": [bool(v) for v in hand_mask.tolist()],
        "observed_now_seconds": observed_now,
    }


def demo_predict_from_raw(
    raw_path: str | Path,
    model_dir: str | Path = "models/policy_bc",
    card_costs_path: str | Path = "data/card_costs.json",
    prefix_events: int = 20,
    acting_side: str = "team",
    device_name: str | None = None,
    think_steps: int | None = None,
    seed: int = 0,
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
    decode = rollout_decode_settings(cfg)
    prediction = predict_next_action(
        model,
        vocab,
        costs,
        battle,
        device,
        acting_side=acting_side,
        max_context=int(cfg.get("max_context", 64)),
        threat_dim=int(cfg.get("threat_dim", 0)),
        temperature=decode["temperature"],
        slot_decode=decode["slot_decode"],
        placement_decode=decode["placement_decode"],
        placement_temperature=decode["placement_temperature"],
        placement_top_k=decode["placement_top_k"],
        think_steps=decode["think_steps"] if think_steps is None else think_steps,
        rng=random.Random(seed),
    )
    return {
        "battle_id": replay.battle_id,
        "prefix_events": len(events),
        "acting_side": acting_side,
        "prediction": prediction,
    }
