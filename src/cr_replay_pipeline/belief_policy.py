"""Astra v8 Stage A: time-aware belief memory + joint event clock.

Replaces the GRU trunk with a compact latent memory that attends to a causal
event stream, mixes friendly/enemy evidence, and evolves with elapsed time.
Action heads stay the compact selected-card heatmap prior; placement adds a
within-tile offset. Board observations are Stage B and are not used here.
"""

from __future__ import annotations

import math
from typing import Any

import torch
import torch.nn as nn
import torch.nn.functional as F

from .event_clock import (
    CLOCK_HORIZONS,
    activity_targets,
    competing_risk_nll,
    positive_rates,
)
from .feature_schema import CONTINUOUS_INDEX, GLOBAL_INDEX
from .policy_model import (
    NUM_TILES,
    NUM_ZONES,
    SLOT_FEAT_DIM,
    TILE_COLS,
    TILE_ROWS,
    DeckMatchupEncoder,
    PolicyBC,
)
from .winner_dataset import CONTINUOUS_DIM, GLOBAL_DIM


def fourier_freqs(dim: int, max_period: float = 64.0) -> torch.Tensor:
    """Log-spaced frequencies for ``fourier_encode`` (even ``dim`` uses ``dim // 2``)."""
    half = dim // 2
    return torch.exp(torch.linspace(0.0, math.log(max_period), half))


def fourier_encode(
    values: torch.Tensor,
    dim: int,
    max_period: float = 64.0,
    freqs: torch.Tensor | None = None,
) -> torch.Tensor:
    """Map seconds to a fixed sinusoidal feature of even ``dim``."""
    if freqs is None:
        freqs = fourier_freqs(dim, max_period).to(device=values.device, dtype=values.dtype)
    else:
        freqs = freqs.to(device=values.device, dtype=values.dtype)
    angles = values.unsqueeze(-1) * freqs
    encoded = torch.cat([angles.sin(), angles.cos()], dim=-1)
    if dim % 2:
        encoded = F.pad(encoded, (0, 1))
    return encoded


class FeedForward(nn.Module):
    def __init__(self, d_model: int, d_ff: int, dropout: float):
        super().__init__()
        self.net = nn.Sequential(
            nn.Linear(d_model, d_ff),
            nn.GELU(),
            nn.Dropout(dropout),
            nn.Linear(d_ff, d_model),
            nn.Dropout(dropout),
        )

    def forward(self, hidden: torch.Tensor) -> torch.Tensor:
        return self.net(hidden)


class InteractionBlock(nn.Module):
    """Self-attention over memory tokens so friendly and enemy evidence mix."""

    def __init__(self, d_model: int, n_heads: int, d_ff: int, dropout: float):
        super().__init__()
        self.norm_attn = nn.LayerNorm(d_model)
        self.attn = nn.MultiheadAttention(
            d_model, n_heads, dropout=dropout, batch_first=True
        )
        self.norm_ff = nn.LayerNorm(d_model)
        self.ff = FeedForward(d_model, d_ff, dropout)

    def forward(self, memory: torch.Tensor) -> torch.Tensor:
        query = self.norm_attn(memory)
        mixed, _ = self.attn(query, query, query, need_weights=False)
        memory = memory + mixed
        return memory + self.ff(self.norm_ff(memory))


class TimeTransition(nn.Module):
    """Gated residual that advances memory while nobody plays."""

    def __init__(self, d_model: int, time_dim: int = 16, legacy: bool = False):
        super().__init__()
        self.legacy = legacy
        self.time_dim = time_dim
        if not legacy:
            self.equilibrium = nn.Parameter(torch.zeros(d_model))
            self.log_rates = nn.Parameter(torch.full((d_model,), -3.0))
            return
        self.gate = nn.Linear(d_model + time_dim, d_model)
        self.delta = nn.Linear(d_model + time_dim, d_model)
        self.register_buffer("freqs", fourier_freqs(time_dim, 32.0), persistent=False)

    def forward(self, memory: torch.Tensor, delay_seconds: torch.Tensor) -> torch.Tensor:
        if not self.legacy:
            # Autonomous exponential relaxation: identity at zero, stable for
            # long waits, and composition invariant between event updates.
            baseline = torch.tanh(self.equilibrium)
            rates = F.softplus(self.log_rates)
            decay = torch.exp(-delay_seconds.clamp_min(0)[:, None, None] * rates)
            moved = baseline + (memory - baseline) * decay
            return torch.where(delay_seconds[:, None, None] == 0, memory, moved)
        stamp = fourier_encode(
            delay_seconds.clamp_min(0.0), self.time_dim, max_period=32.0, freqs=self.freqs
        )
        stamp = stamp.unsqueeze(1).expand(-1, memory.size(1), -1)
        cond = torch.cat([memory, stamp], dim=-1)
        gate = torch.sigmoid(self.gate(cond))
        return memory + gate * torch.tanh(self.delta(cond))


class CausalBeliefPolicy(nn.Module):
    """Predict next action from a revisable match memory, plus a joint clock."""

    def __init__(
        self,
        vocab_size: int,
        continuous_dim: int = CONTINUOUS_DIM,
        global_dim: int = GLOBAL_DIM,
        card_embed_dim: int = 64,
        d_model: int = 128,
        num_layers: int = 4,
        dropout: float = 0.1,
        slot_feat_dim: int = SLOT_FEAT_DIM,
        num_zones: int = NUM_ZONES,
        memory_tokens: int = 24,
        memory_heads: int = 4,
        interaction_blocks: int = 2,
        time_dim: int = 16,
        card_conditioned_placement: bool = True,
        placement_mode: str = "heatmap",
        placement_card_mode: str = "selected",
        event_clock: bool = True,
        memory_revision: int = 2,
    ):
        super().__init__()
        if memory_tokens < 2:
            raise ValueError("memory_tokens must be at least 2")
        if placement_mode != "heatmap":
            raise ValueError("v8 Stage A uses heatmap placement")
        if placement_card_mode != "selected":
            raise ValueError("v8 Stage A uses selected-card placement")
        self.card_embed_dim = card_embed_dim
        self.d_model = d_model
        self.continuous_dim = continuous_dim
        self.memory_tokens = int(memory_tokens)
        self.memory_heads = int(memory_heads)
        self.interaction_blocks = int(interaction_blocks)
        self.time_dim = int(time_dim)
        self.card_conditioned_placement = True
        self.placement_mode = "heatmap"
        self.placement_card_mode = "selected"
        self.event_clock_enabled = bool(event_clock)
        self.memory_revision = memory_revision
        self.arena_memory_channels = 0
        self.max_think_steps = 0
        d_ff = d_model * 8

        rows = torch.arange(TILE_ROWS, dtype=torch.float32).repeat_interleave(TILE_COLS)
        cols = torch.arange(TILE_COLS, dtype=torch.float32).repeat(TILE_ROWS)
        self.register_buffer(
            "tile_centers",
            torch.stack([(cols + 0.5) / TILE_COLS, (rows + 0.5) / TILE_ROWS], dim=-1),
            persistent=False,
        )
        self.card_embedding = nn.Embedding(vocab_size, card_embed_dim, padding_idx=0)
        event_in = card_embed_dim + continuous_dim + 2 * time_dim
        self.event_proj = nn.Sequential(
            nn.Linear(event_in, d_model),
            nn.LayerNorm(d_model),
            nn.GELU(),
            nn.Dropout(dropout),
        )
        encoder_layer = nn.TransformerEncoderLayer(
            d_model,
            nhead=memory_heads,
            dim_feedforward=d_ff,
            dropout=dropout,
            activation="gelu",
            batch_first=True,
            norm_first=True,
        )
        self.event_encoder = nn.TransformerEncoder(
            encoder_layer, num_layers=max(int(num_layers), 1), enable_nested_tensor=False
        )
        self.memory = nn.Parameter(torch.randn(self.memory_tokens, d_model) * 0.02)
        self.side_embed = nn.Embedding(2, d_model)
        self.cross_attn = nn.MultiheadAttention(
            d_model, memory_heads, dropout=dropout, batch_first=True
        )
        self.cross_norm = nn.LayerNorm(d_model)
        self.blocks = nn.ModuleList(
            [
                InteractionBlock(d_model, memory_heads, d_ff, dropout)
                for _ in range(self.interaction_blocks)
            ]
        )
        self.time_transition = TimeTransition(d_model, time_dim=time_dim, legacy=memory_revision == 1)
        if memory_revision >= 2:
            self.event_gate = nn.Linear(2 * d_model, d_model)
            self.event_value = nn.Linear(d_model, d_model)
        self.pool_query = nn.Linear(d_model, 1)
        self.deck_encoder = DeckMatchupEncoder(card_embed_dim, d_model, dropout)
        self.global_encoder = nn.Sequential(
            nn.Linear(global_dim, d_model),
            nn.LayerNorm(d_model),
            nn.GELU(),
            nn.Dropout(dropout),
        )
        self.fuse = nn.Sequential(
            nn.Linear(d_model * 3, d_model),
            nn.LayerNorm(d_model),
            nn.GELU(),
            nn.Dropout(dropout),
        )
        self.slot_scorer = nn.Sequential(
            nn.Linear(d_model + card_embed_dim + slot_feat_dim, d_model),
            nn.GELU(),
            nn.Dropout(dropout),
            nn.Linear(d_model, 1),
        )
        self.type_head = nn.Linear(d_model, 2)
        place_in = d_model + card_embed_dim
        self.zone_head = nn.Sequential(
            nn.Linear(place_in, d_model),
            nn.GELU(),
            nn.Dropout(dropout),
            nn.Linear(d_model, num_zones),
        )
        self.tile_head = nn.Sequential(
            nn.Linear(place_in + num_zones, d_model),
            nn.GELU(),
            nn.Dropout(dropout),
            nn.Linear(d_model, NUM_TILES),
        )
        self.offset_head = nn.Linear(place_in + num_zones, 2)
        self.timing_head = nn.Linear(d_model, 1)
        self.clock_head = nn.Linear(d_model, 2)
        self.activity_head = nn.Linear(d_model, len(CLOCK_HORIZONS))
        self.register_buffer("stamp_freqs", fourier_freqs(time_dim, 300.0), persistent=False)
        self.register_buffer("delta_freqs", fourier_freqs(time_dim, 32.0), persistent=False)

    @staticmethod
    def _event_times(continuous: torch.Tensor) -> tuple[torch.Tensor, torch.Tensor]:
        seconds = continuous[..., CONTINUOUS_INDEX["seconds"]].float().clamp(0.0, 1.5) * 300.0
        delta = torch.zeros_like(seconds)
        delta[:, 1:] = (seconds[:, 1:] - seconds[:, :-1]).clamp_min(0.0)
        return seconds, delta

    def _encode_events(
        self,
        continuous: torch.Tensor,
        card_ids: torch.Tensor,
        lengths: torch.Tensor,
    ) -> tuple[torch.Tensor, torch.Tensor, torch.Tensor]:
        batch, steps, _ = continuous.shape
        embeds = self.card_embedding(card_ids)
        seconds, delta = self._event_times(continuous)
        time_feat = fourier_encode(
            seconds, self.time_dim, max_period=300.0, freqs=self.stamp_freqs
        )
        delta_feat = fourier_encode(
            delta, self.time_dim, max_period=32.0, freqs=self.delta_freqs
        )
        tokens = self.event_proj(
            torch.cat([embeds, continuous, time_feat, delta_feat], dim=-1)
        )
        padding = torch.arange(steps, device=continuous.device).unsqueeze(0) >= lengths.unsqueeze(1)
        causal = torch.triu(
            torch.ones(steps, steps, device=continuous.device, dtype=torch.bool),
            diagonal=1,
        )
        encoded = self.event_encoder(
            tokens,
            mask=causal,
            src_key_padding_mask=padding,
            is_causal=True,
        )
        encoded = encoded.masked_fill(padding.unsqueeze(-1), 0.0)
        return encoded, padding, seconds

    def _update_memory(
        self,
        events: torch.Tensor,
        padding: torch.Tensor,
        continuous: torch.Tensor,
        permutation: torch.Tensor | None,
    ) -> torch.Tensor:
        batch, steps, _ = events.shape
        memory = self.memory.unsqueeze(0).expand(batch, -1, -1).clone()
        token_side = torch.arange(self.memory_tokens, device=events.device) >= (
            self.memory_tokens // 2
        )
        memory = memory + self.side_embed(token_side.long()).unsqueeze(0)
        if permutation is not None:
            events = events[permutation]
            padding = padding[permutation]
            continuous = continuous[permutation]
        friendly = continuous[..., CONTINUOUS_INDEX["side_flag"]] > 0.5
        _seconds, delta = self._event_times(continuous)
        n_friend = self.memory_tokens // 2
        n_enemy = self.memory_tokens - n_friend
        for step in range(steps):
            alive = ~padding[:, step]
            delay = torch.where(alive, delta[:, step], torch.zeros_like(delta[:, step]))
            advanced = self.time_transition(memory, delay)
            memory = torch.where(alive.view(batch, 1, 1), advanced, memory)
            event = events[:, step : step + 1]
            is_friend = friendly[:, step] & alive
            is_enemy = (~friendly[:, step]) & alive
            query = self.cross_norm(memory)
            if self.memory_revision >= 2:
                expanded = event.expand(-1, self.memory_tokens, -1)
                select = torch.sigmoid(self.event_gate(torch.cat([query, expanded], dim=-1)))
                attended = select * torch.tanh(self.event_value(expanded))
            else:
                attended, _ = self.cross_attn(query, event, event, need_weights=False)
            gate = torch.cat(
                [
                    is_friend.to(dtype=attended.dtype).view(batch, 1, 1).expand(-1, n_friend, 1),
                    is_enemy.to(dtype=attended.dtype).view(batch, 1, 1).expand(-1, n_enemy, 1),
                ],
                dim=1,
            )
            memory = memory + attended * gate
        for block in self.blocks:
            memory = block(memory)
        return memory

    def _query_delay(
        self, seconds: torch.Tensor, lengths: torch.Tensor, globals_: torch.Tensor
    ) -> torch.Tensor:
        last_index = (lengths - 1).clamp(min=0)
        last_time = seconds.gather(1, last_index.unsqueeze(1)).squeeze(1)
        if globals_.size(-1) > GLOBAL_INDEX["now"]:
            observed = globals_[..., GLOBAL_INDEX["now"]].float().clamp(0.0, 1.5) * 300.0
        else:
            observed = last_time
        return (observed - last_time).clamp_min(0.0)

    def _placement_card_context(
        self,
        team_embeds: torch.Tensor,
        slot_logits: torch.Tensor,
        target_slots: torch.Tensor | None,
        placement_slots: torch.Tensor | None,
    ) -> torch.Tensor:
        selected = placement_slots
        if selected is None:
            if self.training and target_slots is not None:
                selected = target_slots
            else:
                selected = slot_logits.argmax(dim=-1)
        batch = torch.arange(team_embeds.size(0), device=team_embeds.device)
        return team_embeds[batch, selected]

    def forward(
        self,
        continuous: torch.Tensor,
        card_ids: torch.Tensor,
        team_deck: torch.Tensor,
        opponent_deck: torch.Tensor,
        globals_: torch.Tensor,
        lengths: torch.Tensor,
        slot_features: torch.Tensor,
        hand_mask: torch.Tensor | None = None,
        target_slots: torch.Tensor | None = None,
        placement_slots: torch.Tensor | None = None,
        arena_permutation: torch.Tensor | None = None,
        disable_arena: bool = False,
        zero_arena_memory: bool = False,
        return_debug: bool = False,
        think_steps: int | None = None,
    ) -> dict[str, torch.Tensor]:
        del think_steps
        encoded, padding, seconds = self._encode_events(continuous, card_ids, lengths)
        memory = self._update_memory(encoded, padding, continuous, arena_permutation)
        query_dt = self._query_delay(seconds, lengths, globals_)
        memory = self.time_transition(memory, query_dt)
        if disable_arena or zero_arena_memory:
            memory = torch.zeros_like(memory)
        pool_logits = self.pool_query(memory).squeeze(-1)
        pooled = (memory * torch.softmax(pool_logits, dim=-1).unsqueeze(-1)).sum(dim=1)

        team_embeds = self.card_embedding(team_deck)
        opp_embeds = self.card_embedding(opponent_deck)
        deck_feat = self.deck_encoder(team_embeds, opp_embeds, team_deck, opponent_deck)
        global_feat = self.global_encoder(globals_)
        fused = self.fuse(torch.cat([pooled, deck_feat, global_feat], dim=-1))

        fused_exp = fused.unsqueeze(1).expand(-1, 8, -1)
        slot_logits = self.slot_scorer(
            torch.cat([fused_exp, team_embeds, slot_features], dim=-1)
        ).squeeze(-1)
        if hand_mask is not None:
            has_hand = hand_mask.any(dim=-1, keepdim=True)
            masked = slot_logits.masked_fill(has_hand & ~hand_mask, -8.0)
            slot_logits = torch.where(has_hand, masked, slot_logits)

        card_ctx = self._placement_card_context(
            team_embeds, slot_logits, target_slots, placement_slots
        )
        place_in = torch.cat([fused, card_ctx], dim=-1)
        zone_logits = self.zone_head(place_in)
        zone_probs = F.softmax(zone_logits, dim=-1)
        place_context = torch.cat([place_in, zone_probs], dim=-1)
        with torch.amp.autocast(fused.device.type, enabled=False):
            tile_logits = self.tile_head(place_context.float())
            tile_probs = F.softmax(tile_logits, dim=-1)
            centers = self.tile_centers.to(device=fused.device, dtype=torch.float32)
            offset = torch.tanh(self.offset_head(place_context.float()))
            half_tile = centers.new_tensor([0.5 / TILE_COLS, 0.5 / TILE_ROWS])
            xy = tile_probs @ centers + offset * half_tile
            fused_f = fused.float()
            timing = self.timing_head(fused_f).squeeze(-1)
            clock_rates = positive_rates(self.clock_head(fused_f))
            activity_logits = self.activity_head(fused_f)
        outputs = {
            "slot_logits": slot_logits,
            "type_logits": self.type_head(fused),
            "zone_logits": zone_logits,
            "tile_logits": tile_logits,
            "xy": xy.clamp(0.0, 1.0),
            "timing": timing,
            "offset": offset,
            "clock_rates": clock_rates,
            "activity_logits": activity_logits,
            "slot_probs": F.softmax(slot_logits, dim=-1),
            "fused": fused,
            "memory": memory,
            "query_dt": query_dt,
            "think_steps": fused.new_zeros((), dtype=torch.long),
        }
        if return_debug:
            outputs["event_encoded"] = encoded
        return outputs

    def loss(
        self,
        outputs: dict[str, torch.Tensor],
        slots: torch.Tensor,
        types: torch.Tensor,
        zones: torch.Tensor,
        xy: torch.Tensor,
        timing: torch.Tensor,
        sample_weights: torch.Tensor | None = None,
        slot_weight: float = 1.4,
        type_weight: float = 0.25,
        zone_weight: float = 1.1,
        xy_weight: float = 0.0,
        timing_weight: float = 0.25,
        tile_weight: float = 0.55,
        clock_weight: float = 0.35,
        activity_weight: float = 0.15,
        offset_weight: float = 0.12,
        **_extra: Any,
    ) -> dict[str, torch.Tensor]:
        slot_raw = F.cross_entropy(
            outputs["slot_logits"].float(), slots, label_smoothing=0.02, reduction="none"
        )
        if sample_weights is not None:
            weights = sample_weights / sample_weights.mean().clamp_min(1e-6)
            slot_loss = (slot_raw * weights).mean()
        else:
            slot_loss = slot_raw.mean()
        placement_mask = (types == 0).float()
        def placement_mean(raw):
            while raw.ndim > 1:
                raw = raw.mean(-1)
            return (raw * placement_mask).sum() / placement_mask.sum().clamp_min(1)
        type_loss = F.cross_entropy(outputs["type_logits"].float(), types, label_smoothing=0.02)
        zone_loss = placement_mean(F.cross_entropy(outputs["zone_logits"].float(), zones, label_smoothing=0.02, reduction="none"))
        xy_loss = placement_mean(F.smooth_l1_loss(outputs["xy"].float(), xy.float(), reduction="none"))
        tile_x = (xy[:, 0] * TILE_COLS).floor().long().clamp(0, TILE_COLS - 1)
        tile_y = (xy[:, 1] * TILE_ROWS).floor().long().clamp(0, TILE_ROWS - 1)
        tile_targets = tile_y * TILE_COLS + tile_x
        tile_loss = placement_mean(F.cross_entropy(
            outputs["tile_logits"].float(), tile_targets, label_smoothing=0.01, reduction="none"
        ))
        timing_loss = F.smooth_l1_loss(outputs["timing"].float(), timing.float())
        delay = torch.expm1(timing.float()).clamp_min(0.0)
        # Legacy action-conditioned batches do not contain next-actor labels.
        # Train the joint clock only through the fixed-perspective clock dataset.
        clock_loss = delay.new_zeros(())
        if self.event_clock_enabled and self.memory_revision == 1:
            clock_loss = competing_risk_nll(outputs["clock_rates"].float(), delay).mean()
        activity_loss = F.binary_cross_entropy_with_logits(
            outputs["activity_logits"].float(), activity_targets(delay)
        )
        centers = self.tile_centers.to(device=xy.device, dtype=xy.dtype)
        half_tile = xy.new_tensor([0.5 / TILE_COLS, 0.5 / TILE_ROWS])
        offset_xy = centers[tile_targets] + outputs["offset"].float() * half_tile.float()
        offset_loss = placement_mean(F.smooth_l1_loss(offset_xy, xy.float(), reduction="none"))
        total = (
            slot_weight * slot_loss
            + type_weight * type_loss
            + zone_weight * zone_loss
            + xy_weight * xy_loss
            + tile_weight * tile_loss
            + timing_weight * timing_loss
            + clock_weight * clock_loss
            + activity_weight * activity_loss
            + offset_weight * offset_loss
        )
        return {
            "loss": total,
            "slot_loss": slot_loss.detach(),
            "type_loss": type_loss.detach(),
            "zone_loss": zone_loss.detach(),
            "xy_loss": xy_loss.detach(),
            "tile_loss": tile_loss.detach(),
            "timing_loss": timing_loss.detach(),
            "clock_loss": clock_loss.detach(),
            "activity_loss": activity_loss.detach(),
            "offset_loss": offset_loss.detach(),
        }


def build_policy_model(
    vocab_size: int,
    cfg: dict[str, Any],
    *,
    global_dim: int | None = None,
) -> nn.Module:
    """Construct the architecture recorded in a checkpoint or training spec."""
    threat_dim = int(cfg.get("threat_dim", 0))
    resolved_global = int(cfg.get("global_dim", global_dim or (GLOBAL_DIM + threat_dim)))
    memory_tokens = int(cfg.get("memory_tokens", 0) or 0)
    common = dict(
        vocab_size=vocab_size,
        global_dim=resolved_global,
        d_model=int(cfg.get("d_model", 160)),
        num_layers=int(cfg.get("num_layers", 2)),
        dropout=float(cfg.get("dropout", 0.2)),
        card_conditioned_placement=bool(cfg.get("card_conditioned_placement", False)),
        placement_mode=str(cfg.get("placement_mode", "xy")),
        placement_card_mode=str(cfg.get("placement_card_mode", "soft")),
    )
    if memory_tokens > 0:
        common["dropout"] = float(cfg.get("dropout", 0.1))
        common["d_model"] = int(cfg.get("d_model", 128))
        common["num_layers"] = int(cfg.get("num_layers", 4))
        common["card_embed_dim"] = int(cfg.get("card_embed_dim", 64))
        return CausalBeliefPolicy(
            **common,
            memory_tokens=memory_tokens,
            memory_heads=int(cfg.get("memory_heads", 4)),
            interaction_blocks=int(cfg.get("interaction_blocks", 2)),
            event_clock=bool(cfg.get("event_clock", True)),
            memory_revision=int(cfg.get("memory_revision", 1)),
        )
    return PolicyBC(
        **common,
        arena_memory_channels=int(cfg.get("arena_memory_channels", 0)),
        arena_hidden_channels=int(cfg.get("arena_hidden_channels", 32)),
        arena_memory_version=str(cfg.get("arena_memory_version", "none")),
        arena_gate_bias=float(cfg.get("arena_gate_bias", -2.2)),
        max_think_steps=int(cfg.get("max_think_steps", 0)),
    )
