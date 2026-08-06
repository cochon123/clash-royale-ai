from __future__ import annotations

import torch
import torch.nn as nn
import torch.nn.functional as F

from .winner_dataset import CONTINUOUS_DIM, GLOBAL_DIM


class AttentionPool(nn.Module):
    def __init__(self, d_model: int, dropout: float):
        super().__init__()
        self.score = nn.Sequential(
            nn.Linear(d_model, d_model // 2),
            nn.Tanh(),
            nn.Dropout(dropout),
            nn.Linear(d_model // 2, 1),
        )

    def forward(self, sequence: torch.Tensor, mask: torch.Tensor) -> torch.Tensor:
        # sequence: [B, T, D], mask: [B, T] True for valid tokens
        logits = self.score(sequence).squeeze(-1)
        logits = logits.masked_fill(~mask, -1e4)
        weights = torch.softmax(logits, dim=-1)
        return torch.bmm(weights.unsqueeze(1), sequence).squeeze(1)


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


class WinnerPredictor(nn.Module):
    """Late-tempo win predictor: globals backbone + sequence/deck residuals."""

    def __init__(
        self,
        vocab_size: int,
        continuous_dim: int = CONTINUOUS_DIM,
        global_dim: int = GLOBAL_DIM,
        card_embed_dim: int = 48,
        d_model: int = 160,
        num_layers: int = 2,
        dropout: float = 0.3,
        max_seq_len: int = 256,
    ):
        super().__init__()
        del max_seq_len
        self.card_embedding = nn.Embedding(vocab_size, card_embed_dim, padding_idx=0)
        self.input_proj = nn.Sequential(
            nn.Linear(card_embed_dim + continuous_dim, d_model),
            nn.LayerNorm(d_model),
            nn.GELU(),
            nn.Dropout(dropout),
        )
        self.play_gru = nn.GRU(
            input_size=d_model,
            hidden_size=d_model // 2,
            num_layers=num_layers,
            batch_first=True,
            bidirectional=True,
            dropout=dropout if num_layers > 1 else 0.0,
        )
        self.play_pool = AttentionPool(d_model, dropout)
        self.play_proj = nn.Sequential(
            nn.Linear(d_model, d_model),
            nn.LayerNorm(d_model),
            nn.GELU(),
            nn.Dropout(dropout),
        )

        self.deck_encoder = DeckMatchupEncoder(card_embed_dim, d_model, dropout)

        self.global_encoder = nn.Sequential(
            nn.Linear(global_dim, d_model * 2),
            nn.LayerNorm(d_model * 2),
            nn.GELU(),
            nn.Dropout(dropout),
            nn.Linear(d_model * 2, d_model),
            nn.LayerNorm(d_model),
            nn.GELU(),
            nn.Dropout(dropout),
        )
        self.global_logits = nn.Linear(d_model, 2)

        self.fusion = nn.Sequential(
            nn.Linear(d_model * 3, d_model),
            nn.LayerNorm(d_model),
            nn.GELU(),
            nn.Dropout(dropout),
            nn.Linear(d_model, d_model // 2),
            nn.GELU(),
            nn.Dropout(dropout),
            nn.Linear(d_model // 2, 2),
        )
        # Start with globals dominant; learn how much sequence/deck to trust.
        self.residual_scale = nn.Parameter(torch.tensor(0.35))
        self._init_weights()

    def _init_weights(self) -> None:
        for module in self.modules():
            if isinstance(module, nn.Linear):
                nn.init.xavier_uniform_(module.weight)
                if module.bias is not None:
                    nn.init.zeros_(module.bias)
            elif isinstance(module, nn.Embedding):
                nn.init.normal_(module.weight, mean=0.0, std=0.02)
                if module.padding_idx is not None:
                    with torch.no_grad():
                        module.weight[module.padding_idx].fill_(0.0)
            elif isinstance(module, nn.GRU):
                for name, param in module.named_parameters():
                    if "weight_hh" in name:
                        nn.init.orthogonal_(param)
                    elif "weight_ih" in name:
                        nn.init.xavier_uniform_(param)
                    elif "bias" in name:
                        nn.init.zeros_(param)

    def forward(
        self,
        continuous_features: torch.Tensor,
        card_ids: torch.Tensor,
        team_deck: torch.Tensor,
        opponent_deck: torch.Tensor,
        global_features: torch.Tensor,
        lengths: torch.Tensor,
    ) -> dict[str, torch.Tensor]:
        global_ctx = self.global_encoder(global_features)
        base_logits = self.global_logits(global_ctx)

        card_emb = self.card_embedding(card_ids)
        play_in = self.input_proj(torch.cat([card_emb, continuous_features], dim=-1))
        packed = nn.utils.rnn.pack_padded_sequence(
            play_in, lengths.cpu(), batch_first=True, enforce_sorted=False
        )
        packed_out, _ = self.play_gru(packed)
        play_seq, _ = nn.utils.rnn.pad_packed_sequence(
            packed_out, batch_first=True, total_length=continuous_features.size(1)
        )
        idx = torch.arange(play_seq.size(1), device=play_seq.device).unsqueeze(0)
        mask = idx < lengths.unsqueeze(1)
        play_vec = self.play_proj(self.play_pool(play_seq, mask))

        team_embeds = self.card_embedding(team_deck)
        opp_embeds = self.card_embedding(opponent_deck)
        deck_ctx = self.deck_encoder(team_embeds, opp_embeds, team_deck, opponent_deck)

        residual_logits = self.fusion(
            torch.cat([global_ctx, play_vec, deck_ctx], dim=-1)
        )
        scale = self.residual_scale.clamp(0.0, 1.5)
        logits = base_logits + scale * residual_logits
        probs = F.softmax(logits, dim=-1)
        return {
            "logits": logits,
            "global_logits": base_logits,
            "probs": probs,
            "winner_pred": logits.argmax(dim=-1),
            "team_win_prob": probs[:, 1],
        }
