"""Batched inference/PPO adapter for the unchanged original BananaPolicy."""
from __future__ import annotations

from collections.abc import Mapping, Sequence
from contextlib import contextmanager
from typing import Any

import torch
from torch import Tensor
from torch.distributions import Bernoulli, Categorical
from torch.nn import functional as F

try:
    from .banana_policy import BananaEpisode, BananaPolicy, SampledAction
    from .banana_fast_features import encode_online_sample
except ImportError:
    from banana_policy import BananaEpisode, BananaPolicy, SampledAction
    from banana_fast_features import encode_online_sample


class FastBananaPolicy(BananaPolicy):
    """Batch independent native observations while preserving original semantics.

    ``act_batch`` takes ``(episode, observation, side, now_seconds)`` requests
    for one policy instance. Every request uses the authoritative native hand,
    elixir, selected-card conditioning, and PPO placement candidate support from
    :class:`BananaPolicy`. Sequence histories are right-padded only for the
    model call; their true lengths remain unchanged.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        # Dense unidirectional GRU states at lengths-1 cannot depend on later
        # right padding. This avoids a CUDA -> CPU lengths copy in pack().
        self.model.padded_encoder = True

    def _encode_policy_sample(self, *args, **kwargs):
        return encode_online_sample(self._dataset, *args, **kwargs)

    @contextmanager
    def _ppo_backend(self):
        encoder = self.model.encoder
        if self.device.type != "cuda" or not torch.backends.cudnn.is_available():
            with torch.backends.cudnn.flags(enabled=False):
                yield
            return
        # cuDNN only retains GRU backward workspace in training mode. Change
        # this child alone and explicitly disable inter-layer dropout, which
        # is already disabled by the rollout's eval mode. All other dropout
        # modules stay in eval. Restore both attributes even on failure.
        training, dropout = encoder.training, encoder.dropout
        try:
            encoder.training, encoder.dropout = True, 0.0
            with torch.backends.cudnn.flags(enabled=True):
                yield
        finally:
            encoder.training, encoder.dropout = training, dropout

    def _placement_from_context(self, first, caches, indices, slots):
        """Reuse the card-independent trunk; retain full-model compatibility.

        The selected-card 4.4.2 architecture applies placement_slots only after
        fused/slot scoring. Eval disables dropout, so sharing this graph also
        preserves PPO's gradients. Other architectures use the full path.
        """
        model = self.model
        subset = [caches[i] for i in indices]
        if (model.training or not model.card_conditioned_placement
                or model.placement_mode != "heatmap"
                or model.placement_card_mode != "selected"
                or getattr(model, "arena_memory_channels", 0)):
            return self._forward_batch(subset, slots)
        ix = torch.tensor(indices, dtype=torch.long, device=self.device)
        decks = torch.stack([cache["inputs"][2] for cache in subset]).to(self.device)
        card_ids = decks.gather(1, slots[:, None]).squeeze(1)
        place_in = torch.cat((first["fused"][ix], model.card_embedding(card_ids)), dim=-1)
        zone_probs = F.softmax(model.zone_head(place_in), dim=-1)
        return {"tile_logits": model.tile_head(torch.cat((place_in, zone_probs), dim=-1))}

    def _tile_batch(self, logits, caches):
        """One categorical per row, including variable masks/support sizes."""
        candidates, counts = [], []
        for cache in caches:
            fixed = cache.get("tile_candidates")
            mask = cache["tile_mask"]
            row = (fixed.long() if fixed is not None else
                   mask.nonzero(as_tuple=False).flatten() if mask is not None else
                   torch.arange(logits.shape[1]))
            if not row.numel():
                raise ValueError("legal tile mask contains no legal locations")
            candidates.append(row)
            counts.append(len(row))
        width = max(counts)
        indices = torch.stack([F.pad(row, (0, width - len(row))) for row in candidates]).to(self.device)
        valid = torch.arange(width, device=self.device)[None, :] < torch.tensor(counts, device=self.device)[:, None]
        temperatures = logits.new_tensor([max(float(c["placement_temperature"]), 1e-4) for c in caches])
        scores = (logits / temperatures[:, None]).gather(1, indices)
        scores = scores.masked_fill(~valid, torch.finfo(scores.dtype).min)
        fixed_rows = [c.get("tile_candidates") is not None for c in caches]
        if not all(fixed_rows):
            if any(fixed_rows):
                raise ValueError("cannot mix fresh and fixed placement supports")
            sizes = [max(1, min(int(c["placement_top_k"]), n)) for c, n in zip(caches, counts)]
            scores, local = torch.topk(scores, max(sizes), dim=1)
            indices = indices.gather(1, local)
            valid = torch.arange(max(sizes), device=self.device)[None, :] < torch.tensor(sizes, device=self.device)[:, None]
            scores = scores.masked_fill(~valid, torch.finfo(scores.dtype).min)
            counts = sizes
        # Inputs/supports were validated above. Distribution validation would
        # otherwise synchronize CUDA for each tiny categorical.
        return Categorical(logits=scores, validate_args=False), indices, counts

    @staticmethod
    def _cache_request(
        policy: "FastBananaPolicy",
        episode: BananaEpisode,
        observation: Mapping[str, Any],
        side: int,
        now_seconds: float,
        temperature: float,
        placement_temperature: float,
        placement_top_k: int,
        legal_tile_mask: Tensor | None,
        *, require_affordable: bool = True,
    ) -> dict[str, Any]:
        hands = observation.get("hands")
        elixirs = observation.get("elixir")
        if (
            not isinstance(hands, Sequence)
            or len(hands) != 2
            or not isinstance(elixirs, Sequence)
            or len(elixirs) != 2
        ):
            raise ValueError("expected native observation with hands[2][4] and elixir[2]")
        inputs, legal, mask = policy._inputs(
            episode,
            side,
            now_seconds,
            hands[side],
            float(elixirs[side]),
            float(elixirs[1 - side]),
            require_affordable=require_affordable,
        )
        return {
            "inputs": tuple(
                t.detach().cpu().clone()
                for t in (
                    inputs[0],
                    inputs[1],
                    inputs[2],
                    inputs[3],
                    inputs[4],
                    inputs[12],
                    inputs[5],
                    mask,
                )
            ),
            "legal": tuple(legal),
            "tile_mask": None
            if legal_tile_mask is None
            else legal_tile_mask.detach().cpu().bool().flatten().clone(),
            "temperature": float(temperature),
            "placement_temperature": float(placement_temperature),
            "placement_top_k": int(placement_top_k),
            "think_steps": policy.think_steps,
            "can_play": bool(legal),
        }

    def _forward_batch(
        self,
        caches: Sequence[Mapping[str, Any]],
        placement_slots: Tensor | None = None,
    ) -> dict[str, Tensor]:
        if not caches:
            raise ValueError("cannot forward an empty batch")
        inputs = [cache["inputs"] for cache in caches]
        max_steps = max(int(row[0].shape[0]) for row in inputs)
        continuous = torch.stack(
            [F.pad(row[0], (0, 0, 0, max_steps - row[0].shape[0])) for row in inputs]
        )
        card_ids = torch.stack(
            [F.pad(row[1], (0, max_steps - row[1].shape[0])) for row in inputs]
        )
        batched = [
            continuous,
            card_ids,
            *(torch.stack([row[i] for row in inputs]) for i in (2, 3, 4, 5, 6)),
        ]
        hand_mask = torch.stack([row[7] for row in inputs])
        kwargs: dict[str, Any] = {
            "hand_mask": hand_mask.to(self.device),
            "think_steps": int(caches[0]["think_steps"]),
        }
        if any(int(cache["think_steps"]) != kwargs["think_steps"] for cache in caches):
            raise ValueError("batch rows use different think_steps")
        if placement_slots is not None:
            kwargs["placement_slots"] = placement_slots.to(self.device, dtype=torch.long)
        return self.model(
            *(tensor.to(self.device) for tensor in batched),
            **kwargs,
        )

    @staticmethod
    def _sample_with_uniform(distribution, caches, component):
        """Optional per-game random streams, independent of worker scheduling."""
        if not all("sample_uniforms" in cache for cache in caches):
            return distribution.sample()
        probs = distribution.probs
        uniform = probs.new_tensor([cache["sample_uniforms"][component] for cache in caches])
        if isinstance(distribution, Bernoulli):
            return (uniform < probs).to(probs.dtype)
        cdf = probs.cumsum(-1)
        cutoff = uniform.clamp_max(1 - torch.finfo(probs.dtype).eps) * cdf[:, -1]
        return (cutoff[:, None] >= cdf).sum(-1).clamp_max(probs.shape[-1] - 1)

    def act(self, episode, observation, side, now_seconds, temperature=1.,
            placement_temperature=.8, placement_top_k=5, deterministic=False,
            legal_tile_mask=None):
        """Use the same fast path for the inherited trainer's periodic evaluations."""
        return self.act_batch(
            [(episode, observation, side, now_seconds)], temperature=temperature,
            placement_temperature=placement_temperature, placement_top_k=placement_top_k,
            deterministic=deterministic, legal_tile_masks=[legal_tile_mask],
        )[0]

    def act_batch(
        self,
        requests: Sequence[tuple[BananaEpisode, Mapping[str, Any], int, float]],
        *,
        temperature: float = 1.0,
        placement_temperature: float = 0.8,
        placement_top_k: int = 5,
        deterministic: bool = False,
        legal_tile_masks: Sequence[Tensor | None] | None = None,
        placement_mask_provider=None,
    ) -> list[SampledAction]:
        """Sample a batch of independent observations with two model passes.

        One shared policy/device and shared decode parameters are used for the
        batch. ``legal_tile_masks`` is optional and aligned with ``requests``.
        Returned actions preserve request order and have the same cache layout
        consumed by the existing PPO trainer.
        """
        if not requests:
            return []
        if legal_tile_masks is None:
            legal_tile_masks = [None] * len(requests)
        if len(legal_tile_masks) != len(requests):
            raise ValueError("legal_tile_masks must match requests length")
        caches = [
            self._cache_request(
                self,
                episode,
                observation,
                side,
                now_seconds,
                temperature,
                placement_temperature,
                placement_top_k,
                legal_tile_masks[i],
            )
            for i, (episode, observation, side, now_seconds) in enumerate(requests)
        ]
        sides = [int(request[2]) for request in requests]
        with torch.no_grad():
            first = self._forward_batch(caches)
            wait_dist = Bernoulli(logits=self.wait_head(first["fused"]).squeeze(-1), validate_args=False)
            values = self.critic(first["fused"]).squeeze(-1)
            can_play = torch.tensor([c["can_play"] for c in caches], device=self.device)
            waits = wait_dist.probs >= 0.5 if deterministic else self._sample_with_uniform(wait_dist, caches, 0).bool()
            waits = waits | ~can_play
            logp = torch.where(can_play, wait_dist.log_prob(waits.float()), 0.)
            entropy = torch.where(can_play, wait_dist.entropy(), 0.)
            # Transfer each decision vector once, rather than synchronizing for
            # every action's wait/slot/tile/x/y scalar.
            wait_rows = waits.cpu().tolist()
            pending = [i for i, wait in enumerate(wait_rows) if not wait]
            placements = {}
            if pending:
                subset = [caches[i] for i in pending]
                ix = torch.tensor(pending, device=self.device)
                mask = torch.stack([c["inputs"][7] for c in subset]).to(self.device)
                slot_logits = first["slot_logits"][ix] / max(float(temperature), 1e-4)
                slot_dist = Categorical(logits=slot_logits.masked_fill(~mask, torch.finfo(slot_logits.dtype).min),
                                        validate_args=False)
                slots = slot_dist.logits.argmax(-1) if deterministic else self._sample_with_uniform(slot_dist, subset, 1)
                if placement_mask_provider is not None:
                    for i, slot in zip(pending, slots.cpu().tolist()):
                        native_slot = next(row[0] for row in caches[i]["legal"] if row[1] == slot)
                        mask = torch.as_tensor(placement_mask_provider(requests[i], native_slot), dtype=torch.bool).flatten().cpu()
                        if mask.numel() != 576 or not mask.any():
                            raise ValueError("native placement mask must contain 576 tiles and a legal target")
                        supplied = caches[i]["tile_mask"]
                        caches[i]["tile_mask"] = mask if supplied is None else mask & supplied
                second = self._placement_from_context(first, caches, pending, slots)
                tile_dist, indices, counts = self._tile_batch(second["tile_logits"], subset)
                choices = tile_dist.logits.argmax(-1) if deterministic else self._sample_with_uniform(tile_dist, subset, 2)
                tiles = indices.gather(1, choices[:, None]).squeeze(1)
                xy = torch.stack(((tiles % 32 + .5) / 32, (tiles // 32 + .5) / 18), dim=1)
                if second.get("offset") is not None:
                    xy = (xy + second["offset"].detach() * xy.new_tensor([.5 / 32, .5 / 18])).clamp(0., 1.)
                red = torch.tensor([sides[i] == 1 for i in pending], device=self.device)
                xy[:, 1] = torch.where(red, 1. - xy[:, 1], xy[:, 1])
                coords = torch.round(xy * xy.new_tensor([18000., 32000.])).clamp_min(0)
                coords = torch.minimum(coords, coords.new_tensor([18000., 32000.])).long()
                logp[ix] += slot_dist.log_prob(slots) + tile_dist.log_prob(choices)
                entropy[ix] += slot_dist.entropy() + tile_dist.entropy()
                decoded = torch.cat((slots[:, None], tiles[:, None], coords, indices), dim=1).cpu()
                for row, i, count in zip(decoded, pending, counts):
                    slot, tile, x, y = row[:4].tolist()
                    caches[i]["tile_candidates"] = row[4:4 + count].clone()
                    placements[i] = slot, tile, x, y
            statistics = torch.stack((logp, entropy, values), dim=1).cpu()
        results = []
        for i, cache in enumerate(caches):
            legal_slots = tuple(sorted({row[1] for row in cache["legal"]}))
            lp, ent, val = statistics[i]
            if wait_rows[i]:
                results.append(SampledAction(sides[i], -1, -1, "", 0, 0, -1, lp, ent, val,
                                             legal_slots, True, cache))
            else:
                slot, tile, x, y = placements[i]
                native_slot, _, card = next(row for row in cache["legal"] if row[1] == slot)
                results.append(SampledAction(sides[i], native_slot, slot, card, x, y, tile,
                                             lp, ent, val, legal_slots, False, cache))
        return results

    def evaluate_actions(self, actions: Sequence[SampledAction]) -> tuple[Tensor, Tensor, Tensor]:
        """Vectorized PPO likelihoods; placement reuses the differentiable trunk."""
        if not actions:
            empty = torch.empty(0, device=self.device)
            return empty, empty, empty
        caches = [action.input_cache for action in actions]
        with self._ppo_backend():
            first = self._forward_batch(caches)
            wait_dist = Bernoulli(logits=self.wait_head(first["fused"]).squeeze(-1), validate_args=False)
            values = self.critic(first["fused"]).squeeze(-1)
            can_play = torch.tensor([c["can_play"] for c in caches], device=self.device)
            waits = torch.tensor([a.wait for a in actions], dtype=torch.float32, device=self.device)
            logp = torch.where(can_play, wait_dist.log_prob(waits), 0.)
            entropy = torch.where(can_play, wait_dist.entropy(), 0.)
            pending = [i for i, action in enumerate(actions) if not action.wait]
            if pending:
                subset = [caches[i] for i in pending]
                ix = torch.tensor(pending, device=self.device)
                slots = torch.tensor([actions[i].deck_slot for i in pending], device=self.device)
                mask = torch.stack([c["inputs"][7] for c in subset]).to(self.device)
                temperatures = values.new_tensor([max(float(c["temperature"]), 1e-4) for c in subset])
                slot_logits = first["slot_logits"][ix] / temperatures[:, None]
                slot_dist = Categorical(logits=slot_logits.masked_fill(~mask, torch.finfo(slot_logits.dtype).min),
                                        validate_args=False)
                second = self._placement_from_context(first, caches, pending, slots)
                # Rollout caches always contain fixed supports. Keep the base
                # behavior for externally created legacy actions without one.
                if all(c.get("tile_candidates") is not None for c in subset):
                    choices = []
                    for i, cache in zip(pending, subset):
                        support = cache["tile_candidates"].tolist()
                        if support.count(actions[i].tile) != 1:
                            raise ValueError(f"stored tile {actions[i].tile} is not in this action's legal top-k set")
                        choices.append(support.index(actions[i].tile))
                    tiles, _, _ = self._tile_batch(second["tile_logits"], subset)
                    choices = torch.tensor(choices, device=self.device)
                    tile_logp, tile_entropy = tiles.log_prob(choices), tiles.entropy()
                else:
                    tile_logp, tile_entropy = [], []
                    for row, i in enumerate(pending):
                        dist, indices = self._tile_distribution(second["tile_logits"][row], caches[i])
                        choice = (indices == actions[i].tile).nonzero(as_tuple=False).flatten()
                        if choice.numel() != 1:
                            raise ValueError(f"stored tile {actions[i].tile} is not in this action's legal top-k set")
                        tile_logp.append(dist.log_prob(choice[0]))
                        tile_entropy.append(dist.entropy())
                    tile_logp, tile_entropy = torch.stack(tile_logp), torch.stack(tile_entropy)
                logp = logp.index_add(0, ix, slot_dist.log_prob(slots) + tile_logp)
                entropy = entropy.index_add(0, ix, slot_dist.entropy() + tile_entropy)
        return logp, entropy, values
