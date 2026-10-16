"""Matched GRU policy, optional clock, and optional gated memory residual.

All variants share exactly the same action trunk/heads. Memory is an added
residual, not a simultaneous replacement of trunk, losses, and decoder.
"""

import torch
from torch import nn
from .policy_model import PolicyBC
from .feature_schema import CONTINUOUS_INDEX, GLOBAL_INDEX
from .belief_policy import TimeTransition
from .event_clock import positive_rates

VARIANTS = ("baseline", "clock", "memory", "shuffled", "frozen_time")


class ControlledPolicy(nn.Module):
    def __init__(self, vocab_size, variant="baseline", d_model=160, memory_tokens=8):
        super().__init__()
        if variant not in VARIANTS:
            raise ValueError(variant)
        self.variant = variant
        self.base = PolicyBC(
            vocab_size=vocab_size,
            global_dim=62,
            d_model=d_model,
            num_layers=2,
            dropout=0.2,
            card_conditioned_placement=True,
            placement_mode="heatmap",
            placement_card_mode="selected",
        )
        self.clock_head = nn.Linear(d_model, 2) if variant != "baseline" else None
        if variant in ("memory", "shuffled", "frozen_time"):
            self.memory = nn.Parameter(torch.randn(memory_tokens, d_model) * 0.02)
            self.event_embed = nn.Embedding(vocab_size, 32)
            self.event_proj = nn.Linear(60, d_model)
            self.update_gate = nn.Linear(2 * d_model, d_model)
            self.transition = TimeTransition(d_model)
            self.readout = nn.Linear(d_model, d_model)
            self.residual_gate = nn.Parameter(torch.tensor(-2.0))

    def memory_residual(self, c, ids, lengths, globals_):
        if not hasattr(self, "memory"):
            return None
        if self.variant == "shuffled":
            order = torch.arange(len(c), device=c.device).roll(1)
            c, ids, lengths, globals_ = (
                c[order],
                ids[order],
                lengths[order],
                globals_[order],
            )
        state = self.memory.unsqueeze(0).expand(len(c), -1, -1)
        events = torch.tanh(self.event_proj(torch.cat([c, self.event_embed(ids)], -1)))
        previous = c[:, 0, CONTINUOUS_INDEX["seconds"]] * 300
        for step in range(c.size(1)):
            valid = (step < lengths) & (ids[:, step] != 0)
            now = c[:, step, CONTINUOUS_INDEX["seconds"]] * 300
            dt = (now - previous).clamp_min(0)
            if self.variant == "frozen_time":
                dt = torch.zeros_like(dt)
            advanced = self.transition(state, dt)
            event = events[:, step : step + 1].expand_as(state)
            gate = torch.sigmoid(self.update_gate(torch.cat([advanced, event], -1)))
            updated = advanced + gate * (event - advanced)
            state = torch.where(valid[:, None, None], updated, state)
            previous = torch.where(valid, now, previous)
        dt = (globals_[:, GLOBAL_INDEX["now"]] * 300 - previous).clamp_min(0)
        if self.variant == "frozen_time":
            dt = torch.zeros_like(dt)
        state = self.transition(state, dt)
        return torch.sigmoid(self.residual_gate) * self.readout(state.mean(1))

    def forward(self, batch, teacher=False):
        c, ids, td, od, g, sf, hm, slots, types, zones, xy, timing, lengths, weights = (
            batch
        )
        residual = self.memory_residual(c, ids, lengths, g)
        out = self.base(
            c,
            ids,
            td,
            od,
            g,
            lengths,
            sf,
            hm,
            placement_slots=slots if teacher else None,
            fused_residual=residual,
        )
        if self.clock_head is not None:
            out["clock_rates"] = positive_rates(self.clock_head(out["fused"]).float())
        return out

    @torch.inference_mode()
    def schedule(self, batch):
        """Recompute from the latest observation; cancel on any new event.

        Returns the next-event median and probability it is our event. The
        caller waits for new evidence when the opponent is more probable.
        This is an offline policy interface, not a phone command.
        """
        if self.clock_head is None:
            raise ValueError("Baseline has no event clock")
        out = self(batch)
        rates = out["clock_rates"]
        own = rates[:, 0] / rates.sum(-1)
        return {
            "delay_seconds": 0.69314718056 / rates.sum(-1),
            "own_next_probability": own,
            "wait_for_opponent": own < 0.5,
            "slot": out["slot_logits"].argmax(-1),
        }


def load_controlled_policy(path, device="cpu"):
    from .causal_observation import OBSERVATION_VERSION
    from .winner_dataset import CardVocab

    artifact = torch.load(path, map_location=device, weights_only=False)
    if artifact["contract"].get("observation") != OBSERVATION_VERSION:
        raise ValueError("Incompatible observation contract")
    config = artifact["config"]
    model = ControlledPolicy(
        config["vocab_size"], config["variant"], config["d_model"]
    ).to(device)
    model.load_state_dict(artifact["model_state"])
    model.eval()
    return model, CardVocab.from_dict(artifact["vocab"])


@torch.inference_mode()
def predict_observation(model, observation, vocab, costs):
    """Shared deployed observation encoding; offline proposals only, no taps."""
    from .controlled_dataset import tensor_row
    from .policy_dataset import collate_policy_batch
    from .policy_model import TILE_ROWS, TILE_COLS

    device = next(model.parameters()).device
    batch = tuple(
        t.to(device)
        for t in collate_policy_batch([tensor_row(observation.encode(vocab, costs))])
    )
    out = model(batch)
    slot = int(out["slot_logits"][0].argmax())
    tile = int(out["tile_logits"][0].argmax())
    b = observation.battle
    deck = b.team_deck if observation.agent_side == "team" else b.opponent_deck
    result = {
        "card": deck[slot],
        "event_type": "card_play",
        "slot": slot,
        "x": (tile % TILE_COLS + 0.5) / TILE_COLS,
        "y": (tile // TILE_COLS + 0.5) / TILE_ROWS,
        "observation_seconds": observation.now,
        "requery_after_seconds": 0.25,
        "coordinate_frame": "agent-relative normalized",
        "ability_actions_supported": False,
    }
    if model.clock_head is not None:
        rates = out["clock_rates"][0]
        own = float(rates[0] / rates.sum())
        result.update(
            delay_seconds=float(0.69314718056 / rates.sum()),
            own_next_probability=own,
            wait_for_opponent=own < 0.5,
            scheduler="joint_clock",
        )
    else:
        result.update(
            delay_seconds=max(0.0, float(out["timing"][0].expm1())),
            scheduler="baseline_timing",
        )
    return result
