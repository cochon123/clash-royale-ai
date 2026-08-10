"""Causal next-action policy for Clash Royale behavior cloning."""

from __future__ import annotations

import torch
import torch.nn as nn
import torch.nn.functional as F

from .winner_dataset import CONTINUOUS_DIM, GLOBAL_DIM

SLOT_FEAT_DIM = 8
NUM_ZONES = 12  # 4 y-bands × 3 x-lanes
TILE_ROWS = 18
TILE_COLS = 32
NUM_TILES = TILE_ROWS * TILE_COLS


class ArenaMemoryRasterizer(nn.Module):
    """Turn causal action events into a small, perspective-normalized arena map.

    The dataset stores action history rather than observed unit state.  This
    module deliberately models that limitation: every event contributes a
    decaying, bilinearly splatted footprint and nothing is inferred from the
    target event.  The first twelve channels are dynamic; four fixed geometry
    channels are appended by :meth:`forward`.
    """

    DYNAMIC_CHANNELS = 12
    TOTAL_CHANNELS = 16

    def __init__(self, rows: int = TILE_ROWS, cols: int = TILE_COLS):
        super().__init__()
        self.rows = int(rows)
        self.cols = int(cols)
        col = (torch.arange(self.cols, dtype=torch.float32) + 0.5) / self.cols
        row = (torch.arange(self.rows, dtype=torch.float32) + 0.5) / self.rows
        yy, xx = torch.meshgrid(row, col, indexing="ij")
        geometry = torch.stack(
            [
                xx,
                yy,
                (yy - 0.5).abs() * 2.0,
                ((yy >= 0.42) & (yy <= 0.58)).float(),
            ],
            dim=0,
        )
        self.register_buffer("geometry", geometry, persistent=False)

    @staticmethod
    def _decay(age: torch.Tensor, seconds: float) -> torch.Tensor:
        return torch.exp(-age / float(seconds))

    def forward(
        self,
        continuous: torch.Tensor,
        lengths: torch.Tensor,
        timing: torch.Tensor,
        arena_permutation: torch.Tensor | None = None,
    ) -> torch.Tensor:
        batch, steps, _ = continuous.shape
        device = continuous.device
        valid = torch.arange(steps, device=device).unsqueeze(0) < lengths.unsqueeze(1)
        seconds = continuous[..., 0].float().clamp(0.0, 1.5) * 300.0
        last_index = (lengths - 1).clamp(min=0)
        last_time = seconds.gather(1, last_index.unsqueeze(1)).squeeze(1)
        # timing is detached so the placement memory cannot train the timing
        # head, and the target delay is never used as a feature.
        predicted_delay = torch.expm1(timing.detach().float()).clamp(0.2, 12.0)
        reference_time = last_time + predicted_delay
        age = (reference_time.unsqueeze(1) - seconds).clamp_min(0.0)

        side_friendly = continuous[..., 1] > 0.5
        ability = continuous[..., 10] > 0.5
        spell = continuous[..., 11] > 0.5
        wincon = continuous[..., 12] > 0.5
        nonspell = valid & ~ability & ~spell
        spell = valid & ~ability & spell
        friendly = valid & side_friendly
        enemy = valid & ~side_friendly
        cost = continuous[..., 9].float().clamp_min(0.0)

        values = torch.zeros(
            (batch, self.DYNAMIC_CHANNELS, steps), device=device, dtype=torch.float32
        )
        # Three temporal scales for each side's non-spell action mass.
        for channel, tau, mask in (
            (0, 2.5, friendly & nonspell),
            (1, 7.5, friendly & nonspell),
            (2, 20.0, friendly & nonspell),
            (3, 2.5, enemy & nonspell),
            (4, 7.5, enemy & nonspell),
            (5, 20.0, enemy & nonspell),
        ):
            values[:, channel] = mask.float() * self._decay(age, tau)
        values[:, 6] = (friendly & nonspell).float() * cost * self._decay(age, 7.5)
        values[:, 7] = (enemy & nonspell).float() * cost * self._decay(age, 7.5)
        values[:, 8] = (friendly & wincon).float() * self._decay(age, 12.0)
        values[:, 9] = (enemy & wincon).float() * self._decay(age, 12.0)
        values[:, 10] = (friendly & spell).float() * self._decay(age, 2.0)
        values[:, 11] = (enemy & spell).float() * self._decay(age, 2.0)

        # Map normalized coordinates to cell centers.  Four scatter-add passes
        # implement bilinear splatting without allocating B×T×576 one-hots.
        # Coordinates are normalized arena positions; subtracting half a
        # cell makes a tile center land exactly on its integer raster index.
        gx = continuous[..., 4].float().clamp(0.0, 1.0) * self.cols - 0.5
        gy = continuous[..., 5].float().clamp(0.0, 1.0) * self.rows - 0.5
        x0 = gx.floor().long().clamp(0, self.cols - 1)
        y0 = gy.floor().long().clamp(0, self.rows - 1)
        x1 = (x0 + 1).clamp(0, self.cols - 1)
        y1 = (y0 + 1).clamp(0, self.rows - 1)
        wx = (gx - x0.float()).clamp(0.0, 1.0)
        wy = (gy - y0.float()).clamp(0.0, 1.0)
        memory = torch.zeros(
            (batch, self.DYNAMIC_CHANNELS, self.rows * self.cols),
            device=device,
            dtype=torch.float32,
        )
        for row_idx, col_idx, weight in (
            (y0, x0, (1.0 - wy) * (1.0 - wx)),
            (y0, x1, (1.0 - wy) * wx),
            (y1, x0, wy * (1.0 - wx)),
            (y1, x1, wy * wx),
        ):
            cell = row_idx * self.cols + col_idx
            memory.scatter_add_(
                2,
                cell.unsqueeze(1).expand(-1, self.DYNAMIC_CHANNELS, -1),
                values * weight.unsqueeze(1),
            )
        memory = memory.view(batch, self.DYNAMIC_CHANNELS, self.rows, self.cols)
        if arena_permutation is not None:
            permutation = arena_permutation.to(device=device, dtype=torch.long)
            if permutation.numel() != batch:
                raise ValueError("arena_permutation must contain one index per batch row")
            memory = memory[permutation]
        geometry = self.geometry.to(dtype=memory.dtype).unsqueeze(0).expand(batch, -1, -1, -1)
        return torch.cat([memory, geometry], dim=1)


class ArenaMemoryAdapter(nn.Module):
    """Small card-conditioned residual over the incumbent tile distribution."""

    def __init__(self, place_context_dim: int, hidden_channels: int = 32, gate_bias: float = -2.2):
        super().__init__()
        self.stem = nn.Sequential(
            nn.Conv2d(16, hidden_channels, kernel_size=3, padding=1),
            nn.GroupNorm(8, hidden_channels),
            nn.SiLU(),
        )
        self.block = nn.Sequential(
            nn.Conv2d(hidden_channels, hidden_channels, kernel_size=3, padding=1),
            nn.GroupNorm(8, hidden_channels),
            nn.SiLU(),
            nn.Conv2d(hidden_channels, hidden_channels, kernel_size=3, padding=2, dilation=2),
            nn.GroupNorm(8, hidden_channels),
        )
        self.query = nn.Sequential(
            nn.Linear(place_context_dim, hidden_channels),
            nn.LayerNorm(hidden_channels),
            nn.SiLU(),
        )
        # Keep the warm-started v6.1 distribution exact at step zero.  The
        # zeroed projection still receives gradients from the residual loss.
        self.query_proj = nn.Linear(hidden_channels, hidden_channels)
        self.spatial_residual = nn.Conv2d(hidden_channels, 1, kernel_size=1)
        self.gate = nn.Linear(place_context_dim, 1)
        nn.init.zeros_(self.spatial_residual.weight)
        nn.init.zeros_(self.spatial_residual.bias)
        nn.init.zeros_(self.query_proj.weight)
        nn.init.zeros_(self.query_proj.bias)
        nn.init.constant_(self.gate.bias, float(gate_bias))

    def forward(
        self,
        memory: torch.Tensor,
        place_context: torch.Tensor,
    ) -> tuple[torch.Tensor, torch.Tensor, torch.Tensor]:
        features = self.stem(memory)
        features = F.silu(features + self.block(features))
        query = self.query_proj(self.query(place_context))
        dynamic = (features * query.unsqueeze(-1).unsqueeze(-1)).sum(dim=1)
        residual = dynamic + self.spatial_residual(features).squeeze(1)
        gate = torch.sigmoid(self.gate(place_context))
        return residual.flatten(1), gate, features


class ThinkRefiner(nn.Module):
    """Shared-weight latent refine loop — inference compute scales with K.

    ``think_steps=0`` is a pure identity (feature off / fast path).  Positive
    ``K`` applies the same residual MLP ``K`` times, conditioned on a step
    index embedding so the model can spend extra compute when asked.
    """

    def __init__(self, d_model: int, dropout: float = 0.2, max_steps: int = 8):
        super().__init__()
        self.max_steps = int(max_steps)
        self.step_embed = nn.Embedding(self.max_steps, d_model)
        self.cell = nn.Sequential(
            nn.Linear(d_model, d_model * 4),
            nn.GELU(),
            nn.Dropout(dropout),
            nn.Linear(d_model * 4, d_model),
            nn.Dropout(dropout),
        )
        self.norm = nn.LayerNorm(d_model)

    def forward(self, fused: torch.Tensor, think_steps: int) -> torch.Tensor:
        steps = max(0, min(int(think_steps), self.max_steps))
        if steps <= 0:
            return fused
        hidden = fused
        batch = hidden.size(0)
        for step_index in range(steps):
            index = torch.full(
                (batch,),
                step_index,
                device=hidden.device,
                dtype=torch.long,
            )
            hidden = self.norm(hidden + self.cell(hidden + self.step_embed(index)))
        return hidden


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
        placement_mode: str = "xy",
        arena_memory_channels: int = 0,
        arena_hidden_channels: int = 32,
        arena_memory_version: str = "none",
        arena_gate_bias: float = -2.2,
        max_think_steps: int = 0,
    ):
        super().__init__()
        self.card_embed_dim = card_embed_dim
        self.d_model = d_model
        self.card_conditioned_placement = card_conditioned_placement
        self.placement_mode = str(placement_mode)
        self.arena_memory_channels = int(arena_memory_channels)
        self.arena_hidden_channels = int(arena_hidden_channels)
        self.arena_memory_version = str(arena_memory_version)
        self.arena_gate_bias = float(arena_gate_bias)
        self.max_think_steps = int(max_think_steps)
        if self.placement_mode not in {"xy", "heatmap"}:
            raise ValueError(f"Unknown placement_mode: {self.placement_mode}")
        rows = torch.arange(TILE_ROWS, dtype=torch.float32).repeat_interleave(TILE_COLS)
        cols = torch.arange(TILE_COLS, dtype=torch.float32).repeat(TILE_ROWS)
        self.register_buffer(
            "tile_centers",
            torch.stack([(cols + 0.5) / TILE_COLS, (rows + 0.5) / TILE_ROWS], dim=-1),
            persistent=False,
        )
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
        self.think = (
            ThinkRefiner(d_model, dropout=dropout, max_steps=self.max_think_steps)
            if self.max_think_steps > 0
            else None
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
            if self.placement_mode == "heatmap":
                self.tile_head = nn.Sequential(
                    nn.Linear(place_in + num_zones, d_model),
                    nn.GELU(),
                    nn.Dropout(dropout),
                    nn.Linear(d_model, NUM_TILES),
                )
                if self.arena_memory_channels:
                    if self.arena_memory_channels != ArenaMemoryRasterizer.TOTAL_CHANNELS:
                        raise ValueError(
                            "v7 arena memory requires 16 channels (12 dynamic + 4 geometry)"
                        )
                    self.arena_rasterizer = ArenaMemoryRasterizer()
                    self.arena_adapter = ArenaMemoryAdapter(
                        place_in + num_zones,
                        hidden_channels=self.arena_hidden_channels,
                        gate_bias=self.arena_gate_bias,
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
        arena_permutation: torch.Tensor | None = None,
        disable_arena: bool = False,
        zero_arena_memory: bool = False,
        return_debug: bool = False,
        think_steps: int | None = None,
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
        resolved_think_steps = 0
        if self.think is not None:
            if think_steps is None:
                # Train both the fast path and deeper compute by sampling K.
                if self.training:
                    resolved_think_steps = int(
                        torch.randint(0, self.max_think_steps + 1, (1,)).item()
                    )
                else:
                    resolved_think_steps = 0
            else:
                resolved_think_steps = max(0, int(think_steps))
            fused = self.think(fused, resolved_think_steps)

        fused_exp = fused.unsqueeze(1).expand(-1, 8, -1)
        slot_in = torch.cat([fused_exp, team_embeds, slot_features], dim=-1)
        slot_logits = self.slot_scorer(slot_in).squeeze(-1)
        if hand_mask is not None:
            # Soft mask: keep all legal if none marked, else strongly prefer hand.
            has_hand = hand_mask.any(dim=-1, keepdim=True)
            masked = slot_logits.masked_fill(has_hand & ~hand_mask, -8.0)
            slot_logits = torch.where(has_hand, masked, slot_logits)

        type_logits = self.type_head(fused)
        timing = self.timing_head(fused).squeeze(-1)
        debug: dict[str, torch.Tensor] = {}
        if self.card_conditioned_placement:
            card_ctx = self._placement_card_context(
                team_embeds, slot_logits, target_slots
            )
            place_in = torch.cat([fused, card_ctx], dim=-1)
            zone_logits = self.zone_head(place_in)
            zone_probs = F.softmax(zone_logits, dim=-1)
            if self.placement_mode == "heatmap":
                place_context = torch.cat([place_in, zone_probs], dim=-1)
                base_tile_logits = self.tile_head(place_context)
                tile_logits = base_tile_logits
                if self.arena_memory_channels:
                    memory = self.arena_rasterizer(
                        continuous,
                        lengths,
                        timing,
                        arena_permutation=arena_permutation,
                    )
                    if zero_arena_memory:
                        # Preserve the four static geometry channels while
                        # removing only the twelve dynamic action-history
                        # channels.  This is distinct from disable_arena,
                        # which bypasses the adapter entirely.
                        memory = memory.clone()
                        memory[:, : ArenaMemoryRasterizer.DYNAMIC_CHANNELS] = 0.0
                    if disable_arena:
                        residual = torch.zeros_like(base_tile_logits)
                        gate = torch.zeros(
                            (fused.size(0), 1), device=fused.device, dtype=fused.dtype
                        )
                        features = torch.zeros(
                            fused.size(0),
                            self.arena_hidden_channels,
                            TILE_ROWS,
                            TILE_COLS,
                            device=fused.device,
                            dtype=fused.dtype,
                        )
                    else:
                        residual, gate, features = self.arena_adapter(memory, place_context)
                        tile_logits = base_tile_logits + gate * residual
                    if return_debug:
                        debug.update(
                            {
                                "base_tile_logits": base_tile_logits,
                                "arena_residual_logits": residual,
                                "arena_gate": gate,
                                "arena_memory": memory,
                                "arena_features": features,
                            }
                        )
                tile_probs = F.softmax(tile_logits, dim=-1)
                # Expected coordinate is retained for compatibility with all
                # existing evaluators; inference may sample tile_logits.
                centers = self.tile_centers.to(device=fused.device, dtype=tile_probs.dtype)
                xy = tile_probs @ centers
            else:
                tile_logits = None
                xy = torch.sigmoid(
                    self.xy_head(torch.cat([place_in, zone_probs], dim=-1))
                )
        else:
            zone_logits = self.zone_head(fused)
            zone_probs = F.softmax(zone_logits, dim=-1)
            xy = torch.sigmoid(self.xy_head(torch.cat([fused, zone_probs], dim=-1)))
            tile_logits = None
        outputs = {
            "slot_logits": slot_logits,
            "type_logits": type_logits,
            "zone_logits": zone_logits,
            "tile_logits": tile_logits,
            "xy": xy,
            "timing": timing,
            "slot_probs": F.softmax(slot_logits, dim=-1),
            # Exposed for frozen-trunk probes (card-conditioned placement, etc.).
            "fused": fused,
            "think_steps": fused.new_tensor(resolved_think_steps, dtype=torch.long),
        }
        if return_debug:
            outputs.update(debug)
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
        zone_weight: float = 0.7,
        xy_weight: float = 0.35,
        timing_weight: float = 0.25,
        tile_weight: float = 0.0,
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
        tile_loss = outputs["xy"].new_zeros(())
        if outputs.get("tile_logits") is not None and tile_weight > 0:
            # Same 18×32 grid as the tile centers above.  Clamp edge values so
            # a coordinate of exactly 1.0 remains a legal target.
            tile_x = (xy[:, 0] * TILE_COLS).floor().long().clamp(0, TILE_COLS - 1)
            tile_y = (xy[:, 1] * TILE_ROWS).floor().long().clamp(0, TILE_ROWS - 1)
            tile_targets = tile_y * TILE_COLS + tile_x
            tile_loss = F.cross_entropy(
                outputs["tile_logits"], tile_targets, label_smoothing=0.01
            )
        timing_loss = F.smooth_l1_loss(outputs["timing"], timing)
        total = (
            slot_weight * slot_loss
            + type_weight * type_loss
            + zone_weight * zone_loss
            + xy_weight * xy_loss
            + tile_weight * tile_loss
            + timing_weight * timing_loss
        )
        return {
            "loss": total,
            "slot_loss": slot_loss.detach(),
            "type_loss": type_loss.detach(),
            "zone_loss": zone_loss.detach(),
            "xy_loss": xy_loss.detach(),
            "tile_loss": tile_loss.detach(),
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
