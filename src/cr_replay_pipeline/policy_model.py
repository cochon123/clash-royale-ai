"""Causal next-action policy for Clash Royale behavior cloning."""

from __future__ import annotations

import torch
import torch.nn as nn
import torch.nn.functional as F

from .winner_dataset import CONTINUOUS_DIM, GLOBAL_DIM

SLOT_FEAT_DIM = 8
NUM_ZONES = 12  # 4 y-bands × 3 x-lanes


class DeckMatchupEncoder(nn.Module):
    def __init__(self, embed_dim: int, d_model: int, dropout: float):
        super().__init__()
        self.card_proj = nn.Sequential(
            nn.Linear(embed_dim, d_model),
            nn.LayerNorm(d_model),
            nn.GELU(),
            nn.Dropout(dropout),
        )
        self.out = nn.Sequential(
            nn.Linear(d_model * 4, d_model),
            nn.LayerNorm(d_model),
            nn.GELU(),
            nn.Dropout(dropout),
        )

    def forward(
        self,
        team_embeds: torch.Tensor,
        opp_embeds: torch.Tensor,
        team_ids: torch.Tensor,
        opp_ids: torch.Tensor,
    ) -> torch.Tensor:
        team_mask = (team_ids != 0).unsqueeze(-1).float()
        opp_mask = (opp_ids != 0).unsqueeze(-1).float()
        team = self.card_proj(team_embeds)
        opp = self.card_proj(opp_embeds)
        team_pool = (team * team_mask).sum(dim=1) / team_mask.sum(dim=1).clamp_min(1.0)
        opp_pool = (opp * opp_mask).sum(dim=1) / opp_mask.sum(dim=1).clamp_min(1.0)
        return self.out(
            torch.cat(
                [team_pool, opp_pool, team_pool - opp_pool, team_pool * opp_pool],
                dim=-1,
            )
        )


class PolicyBC(nn.Module):
    """Predict next deck-slot / type / zone / xy / timing from causal history."""

    def __init__(
        self,
        vocab_size: int,
        continuous_dim: int = CONTINUOUS_DIM,
        global_dim: int = GLOBAL_DIM,
        card_embed_dim: int = 48,
        d_model: int = 160,
        num_layers: int = 2,
        dropout: float = 0.2,
        slot_feat_dim: int = SLOT_FEAT_DIM,
        num_zones: int = NUM_ZONES,
        card_conditioned_placement: bool = False,
    ):
        super().__init__()
        self.card_embed_dim = card_embed_dim
        self.d_model = d_model
        self.card_conditioned_placement = card_conditioned_placement
        self.card_embedding = nn.Embedding(vocab_size, card_embed_dim, padding_idx=0)
        self.input_proj = nn.Sequential(
            nn.Linear(card_embed_dim + continuous_dim, d_model),
            nn.LayerNorm(d_model),
            nn.GELU(),
            nn.Dropout(dropout),
        )
        self.encoder = nn.GRU(
            input_size=d_model,
            hidden_size=d_model,
            num_layers=num_layers,
            batch_first=True,
            bidirectional=False,
            dropout=dropout if num_layers > 1 else 0.0,
        )
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
        # Per-card slot scorer: fused state + card embed + cycle features.
        self.slot_scorer = nn.Sequential(
            nn.Linear(d_model + card_embed_dim + slot_feat_dim, d_model),
            nn.GELU(),
            nn.Dropout(dropout),
            nn.Linear(d_model, 1),
        )
        self.type_head = nn.Linear(d_model, 2)
        place_in = d_model + (card_embed_dim if card_conditioned_placement else 0)
        if card_conditioned_placement:
            # MLP placement heads (matches the offline probe that cleared 52% oracle zone).
            self.zone_head = nn.Sequential(
                nn.Linear(place_in, d_model),
                nn.GELU(),
                nn.Dropout(dropout),
                nn.Linear(d_model, num_zones),
            )
            self.xy_head = nn.Sequential(
                nn.Linear(place_in + num_zones, d_model),
                nn.GELU(),
                nn.Dropout(dropout),
                nn.Linear(d_model, 2),
            )
        else:
            self.zone_head = nn.Linear(d_model, num_zones)
            self.xy_head = nn.Linear(d_model + num_zones, 2)
        self.timing_head = nn.Linear(d_model, 1)

    def _last_hidden(
        self, sequence: torch.Tensor, lengths: torch.Tensor
    ) -> torch.Tensor:
        index = (lengths - 1).clamp(min=0)
        batch = torch.arange(sequence.size(0), device=sequence.device)
        return sequence[batch, index]

    def _placement_card_context(
        self,
        team_embeds: torch.Tensor,
        slot_logits: torch.Tensor,
        target_slots: torch.Tensor | None,
    ) -> torch.Tensor:
        """Card embedding used to condition zone/XY.

        Train: mix teacher-forced GT card with soft slot-weighted cards so the
        placement head stays useful under the model's own slot distribution
        (the probe showed oracle helps but e2e-on-frozen-argmax did not).
        Eval: soft mixture (≈ argmax when confident).
        """
        probs = F.softmax(slot_logits, dim=-1)
        soft = (probs.unsqueeze(-1) * team_embeds).sum(dim=1)
        if target_slots is None or not self.training:
            return soft
        batch = torch.arange(team_embeds.size(0), device=team_embeds.device)
        teacher = team_embeds[batch, target_slots]
        # 70% teacher / 30% soft keeps oracle-quality gradients while exposing
        # the placement head to slot uncertainty.
        return 0.7 * teacher + 0.3 * soft

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
    ) -> dict[str, torch.Tensor]:
        embeds = self.card_embedding(card_ids)
        tokens = self.input_proj(torch.cat([embeds, continuous], dim=-1))
        packed = nn.utils.rnn.pack_padded_sequence(
            tokens,
            lengths.cpu().clamp(min=1),
            batch_first=True,
            enforce_sorted=False,
        )
        encoded, _ = self.encoder(packed)
        encoded, _ = nn.utils.rnn.pad_packed_sequence(encoded, batch_first=True)
        last = self._last_hidden(encoded, lengths)

        team_embeds = self.card_embedding(team_deck)
        opp_embeds = self.card_embedding(opponent_deck)
        deck_feat = self.deck_encoder(team_embeds, opp_embeds, team_deck, opponent_deck)
        global_feat = self.global_encoder(globals_)
        fused = self.fuse(torch.cat([last, deck_feat, global_feat], dim=-1))

        fused_exp = fused.unsqueeze(1).expand(-1, 8, -1)
        slot_in = torch.cat([fused_exp, team_embeds, slot_features], dim=-1)
        slot_logits = self.slot_scorer(slot_in).squeeze(-1)
        if hand_mask is not None:
            # Soft mask: keep all legal if none marked, else strongly prefer hand.
            has_hand = hand_mask.any(dim=-1, keepdim=True)
            masked = slot_logits.masked_fill(has_hand & ~hand_mask, -8.0)
            slot_logits = torch.where(has_hand, masked, slot_logits)

        type_logits = self.type_head(fused)
        if self.card_conditioned_placement:
            card_ctx = self._placement_card_context(
                team_embeds, slot_logits, target_slots
            )
            place_in = torch.cat([fused, card_ctx], dim=-1)
            zone_logits = self.zone_head(place_in)
            zone_probs = F.softmax(zone_logits, dim=-1)
            xy = torch.sigmoid(
                self.xy_head(torch.cat([place_in, zone_probs], dim=-1))
            )
        else:
            zone_logits = self.zone_head(fused)
            zone_probs = F.softmax(zone_logits, dim=-1)
            xy = torch.sigmoid(self.xy_head(torch.cat([fused, zone_probs], dim=-1)))
        timing = self.timing_head(fused).squeeze(-1)
        return {
            "slot_logits": slot_logits,
            "type_logits": type_logits,
            "zone_logits": zone_logits,
            "xy": xy,
            "timing": timing,
            "slot_probs": F.softmax(slot_logits, dim=-1),
            # Exposed for frozen-trunk probes (card-conditioned placement, etc.).
            "fused": fused,
        }

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
        zone_weight: float = 0.7,
        xy_weight: float = 0.35,
        timing_weight: float = 0.25,
    ) -> dict[str, torch.Tensor]:
        slot_raw = F.cross_entropy(
            outputs["slot_logits"],
            slots,
            label_smoothing=0.02,
            reduction="none",
        )
        if sample_weights is not None:
            weights = sample_weights / sample_weights.mean().clamp_min(1e-6)
            slot_loss = (slot_raw * weights).mean()
        else:
            slot_loss = slot_raw.mean()
        type_loss = F.cross_entropy(outputs["type_logits"], types, label_smoothing=0.02)
        zone_loss = F.cross_entropy(outputs["zone_logits"], zones, label_smoothing=0.02)
        xy_loss = F.smooth_l1_loss(outputs["xy"], xy)
        timing_loss = F.smooth_l1_loss(outputs["timing"], timing)
        total = (
            slot_weight * slot_loss
            + type_weight * type_loss
            + zone_weight * zone_loss
            + xy_weight * xy_loss
            + timing_weight * timing_loss
        )
        return {
            "loss": total,
            "slot_loss": slot_loss.detach(),
            "type_loss": type_loss.detach(),
            "zone_loss": zone_loss.detach(),
            "xy_loss": xy_loss.detach(),
            "timing_loss": timing_loss.detach(),
        }


def xy_to_zone(x: float, y: float) -> int:
    if x < 0.4:
        x_bin = 0
    elif x < 0.6:
        x_bin = 1
    else:
        x_bin = 2
    if y < 0.25:
        y_bin = 0
    elif y < 0.45:
        y_bin = 1
    elif y < 0.55:
        y_bin = 2
    else:
        y_bin = 3
    return y_bin * 3 + x_bin
