"""Board-aware Hog policy (C5).

``BoardHogPolicy`` is an ``AdaptiveHogPolicy`` that loads hog-2 ``final.pt``
and adds three zero-initialized branches:

- a 64-d entity-set encoder, projected into the context used for card logits,
  waiting and placement;
- a raster CNN whose last layer is a per-tile residual on placement logits;
- a privileged critic residual. The loaded hog-2 value head is not
  reinitialized. Only an extra linear, also zero at init, mixes the board
  embedding into the critic input. Actor card, wait and placement context do
  not receive that critic residual.

Because every new output layer is zero, a non-empty board does not change
actions, log-probabilities, entropy or values relative to
``AdaptiveHogPolicy`` at initialization. The critic matches because its base
weights are the checkpoint's value head, not a fresh head.

Call ``observe_board(game_key, board_json, perspective)`` before
``act_batch``. The acting side and the cached perspective must agree.
``observation["board"]`` overrides the cache for that decision. The tensors
copied into ``SampledAction.input_cache["board"]`` are the ones PPO
recomputes, so a later ``observe_board`` does not rewrite a stored rollout.
"""
from __future__ import annotations

from pathlib import Path
from typing import Any, Mapping, Sequence

import torch
from torch import Tensor, nn

from .banana_hog_adaptive import AdaptiveHogPolicy
from .banana_policy import BananaEpisode
from .board_features import (
    FEATURE_NAMES,
    HASH_BUCKETS,
    NUM_TILES,
    RASTER_CHANNELS,
    BoardTensors,
    RowTable,
    RowVocab,
    collate_boards,
    encode_board,
    load_row_table,
    vocab_for_table,
)

DEFAULT_HOG2_WEIGHTS = Path(
    "/home/cochon/Documents/mini-project/clash-royale-ai/engine/runs/banana-hog-2/models/final.pt"
)
BOARD_DIM = 64
CHECKPOINT_FORMAT = "board-hog-policy-v1"


class EntitySetEncoder(nn.Module):
    """One transformer layer over at most 64 entities, pooled at a CLS token."""

    def __init__(self, vocab_size: int, feature_dim: int, d_model: int = BOARD_DIM, nhead: int = 4):
        super().__init__()
        self.d_model = d_model
        self.embed = nn.Embedding(vocab_size, d_model, padding_idx=0)
        self.feat = nn.Linear(feature_dim, d_model)
        self.cls = nn.Parameter(torch.empty(1, 1, d_model))
        nn.init.normal_(self.cls, std=0.02)
        layer = nn.TransformerEncoderLayer(
            d_model=d_model,
            nhead=nhead,
            dim_feedforward=d_model * 2,
            dropout=0.0,
            activation="gelu",
            batch_first=True,
            norm_first=True,
        )
        self.encoder = nn.TransformerEncoder(layer, num_layers=1, enable_nested_tensor=False)
        self.norm = nn.LayerNorm(d_model)

    def forward(self, type_ids: Tensor, features: Tensor, mask: Tensor) -> Tensor:
        tokens = self.embed(type_ids) + self.feat(features)
        tokens = tokens * mask.unsqueeze(-1).to(dtype=tokens.dtype)
        batch = type_ids.shape[0]
        cls = self.cls.expand(batch, -1, -1)
        tokens = torch.cat((cls, tokens), dim=1)
        keep = torch.cat((mask.new_ones(batch, 1), mask), dim=1)
        encoded = self.encoder(tokens, src_key_padding_mask=~keep)
        return self.norm(encoded[:, 0])


class RasterPlacementCNN(nn.Module):
    """Per-tile placement residual. The last convolution is zero at init."""

    def __init__(self, channels: int):
        super().__init__()
        self.net = nn.Sequential(
            nn.Conv2d(channels, 16, kernel_size=3, padding=1),
            nn.GELU(),
            nn.Conv2d(16, 16, kernel_size=3, padding=1),
            nn.GELU(),
            nn.Conv2d(16, 1, kernel_size=1),
        )
        nn.init.zeros_(self.net[-1].weight)
        nn.init.zeros_(self.net[-1].bias)

    def forward(self, raster: Tensor) -> Tensor:
        return self.net(raster).squeeze(1).reshape(raster.shape[0], NUM_TILES)


class PrivilegedCritic(nn.Module):
    """Hog-2 value head plus a board residual that the actor does not see."""

    def __init__(self, base: nn.Module, board_dim: int, d_model: int):
        super().__init__()
        self.base = base
        self.proj = nn.Linear(board_dim, d_model)
        nn.init.zeros_(self.proj.weight)
        nn.init.zeros_(self.proj.bias)
        self._embed: Tensor | None = None

    def set_embed(self, embed: Tensor | None) -> None:
        self._embed = embed

    def forward(self, fused: Tensor) -> Tensor:
        if self._embed is None:
            return self.base(fused)
        if self._embed.shape[0] != fused.shape[0]:
            raise RuntimeError(
                f"privileged critic batch {fused.shape[0]} does not match board embedding {self._embed.shape[0]}"
            )
        return self.base(fused + self.proj(self._embed))


def _cache_key(game_key: Any) -> tuple:
    if isinstance(game_key, BananaEpisode):
        return ("episode", id(game_key))
    return ("key", game_key)


class BoardHogPolicy(AdaptiveHogPolicy):
    def __init__(
        self,
        *args,
        weights: str | Path | None = None,
        trainable: str = "all",
        rows_path: str | Path | None = None,
        row_table: RowTable | None = None,
        row_vocab: RowVocab | None = None,
        **kwargs,
    ):
        if weights is None:
            weights = DEFAULT_HOG2_WEIGHTS
        weights = Path(weights)
        if not weights.is_file():
            raise FileNotFoundError(f"hog-2 checkpoint is missing: {weights}")
        super().__init__(*args, weights=weights, trainable=trainable, **kwargs)
        # ``self.vocab`` is the card vocabulary owned by BananaPolicy. The row
        # vocabulary is a separate table and must not replace it.
        self.row_table = row_table if row_table is not None else load_row_table(rows_path)
        self.row_vocab = row_vocab if row_vocab is not None else vocab_for_table(self.row_table, HASH_BUCKETS)
        d_model = int(getattr(self.model, "d_model", self.config.get("d_model", 160)))
        self.board_dim = BOARD_DIM
        self.board_encoder = EntitySetEncoder(self.row_vocab.size, len(FEATURE_NAMES), BOARD_DIM).to(self.device)
        self.context_residual = nn.Linear(BOARD_DIM, 8 + d_model).to(self.device)
        self.raster = RasterPlacementCNN(len(RASTER_CHANNELS)).to(self.device)
        nn.init.zeros_(self.context_residual.weight)
        nn.init.zeros_(self.context_residual.bias)
        base_critic = self.critic
        self.critic = PrivilegedCritic(base_critic, BOARD_DIM, d_model).to(self.device)
        self._board_cache: dict[tuple, BoardTensors] = {}
        # Placement-prior from a Hog-connect value head. The head's scale is
        # zero at init, and the branch stays off until a caller enables it.
        self.connect_prior = None
        self.connect_prior_enabled = False
        self.eval()

    def board_parameters(self) -> list[nn.Parameter]:
        """Actor-side board branches: set encoder, context residual and raster.

        The privileged projection ``critic.proj`` is intentionally absent. It is
        already included in ``critic.parameters()``, and Adam rejects a parameter
        that appears in two groups. Add the returned list as its own group.
        """
        return [
            parameter
            for parameter in (
                *self.board_encoder.parameters(),
                *self.context_residual.parameters(),
                *self.raster.parameters(),
            )
        ]

    def parameter_report(self) -> dict[str, int]:
        def count(module: nn.Module) -> int:
            return sum(parameter.numel() for parameter in module.parameters())

        board = {
            "board_encoder": count(self.board_encoder),
            "context_residual": count(self.context_residual),
            "raster": count(self.raster),
            "critic_board": count(self.critic.proj),
        }
        board["board_total"] = sum(board.values())
        board["base_actor"] = count(self.model) + count(self.wait_head)
        board["base_critic"] = count(self.critic.base)
        return board

    def observe_board(self, game_key: Any, board_json: Mapping[str, Any], perspective: int) -> BoardTensors:
        """Cache one match's latest board in the learner's perspective.

        ``game_key`` is the ``BananaEpisode`` for that match, or any other
        hashable id. An episode is stored by object identity and by
        ``battle_id``. A string id is also stored as a battle id so a later
        request whose episode uses that ``battle_id`` can find it.
        """
        encoded = encode_board(board_json, int(perspective), self.row_table, self.row_vocab)
        self._board_cache[_cache_key(game_key)] = encoded
        battle_id = getattr(game_key, "battle_id", None)
        if battle_id is not None:
            self._board_cache[("battle", battle_id)] = encoded
        elif isinstance(game_key, str):
            self._board_cache[("battle", game_key)] = encoded
        return encoded

    def clear_boards(self) -> None:
        self._board_cache.clear()

    def set_connect_prior(self, prior, *, enabled: bool = False) -> None:
        """Attach a Hog-connect placement prior.

        ``enabled`` defaults to false, so attaching a head does not change
        actions. The prior's own scale is zero-initialized; enabling it still
        leaves tile logits unchanged until that scale moves.
        """
        if prior is not None:
            prior.to(self.device)
        self.connect_prior = prior
        self.connect_prior_enabled = bool(enabled) and prior is not None

    def _lookup_board(self, episode: BananaEpisode, observation: Mapping[str, Any], side: int) -> BoardTensors:
        if observation.get("board") is not None:
            perspective = int(observation["perspective"]) if "perspective" in observation else int(side)
            return encode_board(observation["board"], perspective, self.row_table, self.row_vocab)
        found = self._board_cache.get(("episode", id(episode)))
        if found is None and getattr(episode, "battle_id", None) is not None:
            found = self._board_cache.get(("battle", episode.battle_id))
        if found is None:
            raise ValueError("board-aware policy requires observe_board(...) or observation['board']")
        if found.perspective != int(side):
            raise ValueError(
                f"cached board perspective {found.perspective} does not match acting side {int(side)}"
            )
        return found

    @staticmethod
    def _cache_request(policy, episode, observation, side, now_seconds, *args, **kwargs):
        cache = AdaptiveHogPolicy._cache_request(
            policy, episode, observation, side, now_seconds, *args, **kwargs
        )
        cache["board"] = policy._lookup_board(episode, observation, side).clone()
        return cache

    def _forward_batch(self, caches: Sequence[Mapping[str, Any]], placement_slots: Tensor | None = None):
        if any("board" not in cache for cache in caches):
            raise ValueError("board cache is missing; call observe_board or pass observation['board']")
        result = super()._forward_batch(caches, placement_slots)
        batch = collate_boards([cache["board"] for cache in caches]).to(self.device)
        embed = self.board_encoder(batch.type_ids, batch.features, batch.mask)
        residual = self.context_residual(embed)
        slot_width = result["slot_logits"].shape[-1]
        if residual.shape[-1] != slot_width + result["fused"].shape[-1]:
            raise RuntimeError("board context residual does not match slot and fused widths")
        result["slot_logits"] = result["slot_logits"] + residual[:, :slot_width]
        result["fused"] = result["fused"] + residual[:, slot_width:]
        result["board_embed"] = embed
        self.critic.set_embed(embed)
        return result

    def _tile_residual(self, caches: Sequence[Mapping[str, Any]], indices: Sequence[int]) -> Tensor:
        batch = collate_boards([caches[index]["board"] for index in indices]).to(self.device)
        return self.raster(batch.raster)

    def _placement_from_context(self, first, caches, indices, slots):
        # A full-model placement pass forwards the subset and would replace the
        # critic's batch embedding. Restore the embedding from the action pass.
        saved = self.critic._embed
        try:
            second = super()._placement_from_context(first, caches, indices, slots)
        finally:
            self.critic.set_embed(saved)
        if second.get("tile_logits") is None:
            raise RuntimeError("board placement residual requires heatmap tile logits")
        residual = self._tile_residual(caches, indices)
        if residual.shape != second["tile_logits"].shape:
            raise RuntimeError(
                f"placement residual shape {tuple(residual.shape)} != tile logits {tuple(second['tile_logits'].shape)}"
            )
        second = dict(second)
        second["tile_logits"] = second["tile_logits"] + residual
        if self.connect_prior_enabled and self.connect_prior is not None:
            batch = collate_boards([caches[index]["board"] for index in indices]).to(self.device)
            extra = self.connect_prior.residual(batch.raster)
            if extra.shape != second["tile_logits"].shape:
                raise RuntimeError(
                    f"connect prior shape {tuple(extra.shape)} != tile logits {tuple(second['tile_logits'].shape)}"
                )
            second["tile_logits"] = second["tile_logits"] + extra
        return second

    def checkpoint_payload(self) -> dict[str, Any]:
        return {
            "format": CHECKPOINT_FORMAT,
            "model_state": self.model.state_dict(),
            "wait_state": self.wait_head.state_dict(),
            "critic_state": self.critic.base.state_dict(),
            "board_state": {
                "encoder": self.board_encoder.state_dict(),
                "context_residual": self.context_residual.state_dict(),
                "raster": self.raster.state_dict(),
                "critic_proj": self.critic.proj.state_dict(),
                "vocab": self.row_vocab.to_dict(),
            },
            "rows_source": self.row_table.source,
        }

    def save_checkpoint(self, path: str | Path) -> None:
        path = Path(path)
        path.parent.mkdir(parents=True, exist_ok=True)
        torch.save(self.checkpoint_payload(), path)

    def load_checkpoint(self, path: str | Path) -> "BoardHogPolicy":
        payload = torch.load(Path(path), map_location=self.device, weights_only=False)
        self._load_payload(payload)
        return self

    def _load_payload(self, payload: Mapping[str, Any]) -> None:
        if payload.get("format") not in (CHECKPOINT_FORMAT, "crazy-hog-ppo-v1"):
            raise ValueError(f"expected {CHECKPOINT_FORMAT}, got {payload.get('format')}")
        vocab = RowVocab.from_dict(payload["board_state"]["vocab"])
        if vocab.names != self.row_vocab.names or vocab.hash_buckets != self.row_vocab.hash_buckets:
            raise ValueError("board checkpoint vocab does not match this policy")
        self.model.load_state_dict(payload["model_state"])
        self.wait_head.load_state_dict(payload["wait_state"])
        if payload.get("format") == "crazy-hog-ppo-v1":
            self.critic.load_state_dict(payload["critic_state"])
        else:
            self.critic.base.load_state_dict(payload["critic_state"])
        board = payload["board_state"]
        self.board_encoder.load_state_dict(board["encoder"])
        self.context_residual.load_state_dict(board["context_residual"])
        self.raster.load_state_dict(board["raster"])
        self.critic.proj.load_state_dict(board["critic_proj"])
        self.critic.set_embed(None)

    @classmethod
    def from_checkpoint(cls, path: str | Path, **kwargs) -> "BoardHogPolicy":
        payload = torch.load(Path(path), map_location="cpu", weights_only=False)
        if payload.get("format") not in (CHECKPOINT_FORMAT, "crazy-hog-ppo-v1"):
            raise ValueError(f"expected {CHECKPOINT_FORMAT}, got {payload.get('format')}")
        vocab = RowVocab.from_dict(payload["board_state"]["vocab"])
        kwargs.pop("row_vocab", None)
        policy = cls(row_vocab=vocab, **kwargs)
        map_payload = payload
        if policy.device.type != "cpu":
            map_payload = torch.load(Path(path), map_location=policy.device, weights_only=False)
        policy._load_payload(map_payload)
        return policy
