"""Batched inference/PPO adapter for the unchanged original BananaPolicy."""
from __future__ import annotations

from collections.abc import Mapping, Sequence
from contextlib import contextmanager
from typing import Any

import torch
from torch import Tensor
from torch.distributions import Bernoulli, Categorical
from torch.nn import functional as F
from torch.nn.utils.rnn import pad_sequence

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
        decks = first["_deck_ids"][ix]
        card_ids = decks.gather(1, slots[:, None]).squeeze(1)
        place_in = torch.cat((first["fused"][ix], model.card_embedding(card_ids)), dim=-1)
        zone_probs = F.softmax(model.zone_head(place_in), dim=-1)
        return {"tile_logits": model.tile_head(torch.cat((place_in, zone_probs), dim=-1))}

    def _tile_batch(self, logits, caches):
        """Rank fresh supports by score then tile ID; retain stored PPO order."""
        # The common unmasked rollout needs no CPU candidate tensors, padding,
        # host-to-device copies, or gather of the entire 576-tile grid.
        if all(c.get("tile_candidates") is None and c["tile_mask"] is None for c in caches):
            sizes = [max(1, min(int(c["placement_top_k"]), logits.shape[1])) for c in caches]
            temperatures = logits.new_tensor([max(float(c["placement_temperature"]), 1e-4) for c in caches])
            scaled = logits / temperatures[:, None]
            # Stable descending sorting breaks ties by ascending tile ID. In
            # contrast, topk's tie order changes when other rows widen padding.
            indices = torch.argsort(scaled, dim=1, descending=True, stable=True)[:, :max(sizes)]
            scores = scaled.gather(1, indices)
            if min(sizes) != max(sizes):
                valid = torch.arange(max(sizes), device=self.device)[None, :] < torch.tensor(sizes, device=self.device)[:, None]
                scores = scores.masked_fill(~valid, torch.finfo(scores.dtype).min)
            return Categorical(logits=scores, validate_args=False), indices, sizes
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
        indices = torch.stack([F.pad(row, (0, width - len(row))) for row in candidates]).to(self.device, non_blocking=True)
        valid = torch.arange(width, device=self.device)[None, :] < torch.tensor(counts, device=self.device)[:, None]
        temperatures = logits.new_tensor([max(float(c["placement_temperature"]), 1e-4) for c in caches])
        scores = (logits / temperatures[:, None]).gather(1, indices)
        scores = scores.masked_fill(~valid, torch.finfo(scores.dtype).min)
        fixed_rows = [c.get("tile_candidates") is not None for c in caches]
        if not all(fixed_rows):
            if any(fixed_rows):
                raise ValueError("cannot mix fresh and fixed placement supports")
            sizes = [max(1, min(int(c["placement_top_k"]), n)) for c, n in zip(caches, counts)]
            # Fresh candidate rows come from nonzero/arange in tile ID order.
            local = torch.argsort(scores, dim=1, descending=True, stable=True)[:, :max(sizes)]
            scores = scores.gather(1, local)
            indices = indices.gather(1, local)
            valid = torch.arange(max(sizes), device=self.device)[None, :] < torch.tensor(sizes, device=self.device)[:, None]
            scores = scores.masked_fill(~valid, torch.finfo(scores.dtype).min)
            counts = sizes
        # Inputs/supports were validated above. Distribution validation would
        # otherwise synchronize CUDA for each tiny categorical.
        return Categorical(logits=scores, validate_args=False), indices, counts

    def _tile_distribution(self, logits: Tensor, cached: Mapping[str, Any]):
        # Legacy single-action PPO recomputation must use the same canonical
        # fresh support as batched rollout; fixed supports remain untouched.
        distribution, indices, counts = self._tile_batch(logits.unsqueeze(0), [cached])
        return (Categorical(logits=distribution.logits[0, :counts[0]], validate_args=False),
                indices[0, :counts[0]])

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
        continuous = pad_sequence([row[0] for row in inputs], batch_first=True)
        card_ids = pad_sequence([row[1] for row in inputs], batch_first=True)
        batched = [
            continuous,
            card_ids,
            *(torch.stack([row[i] for row in inputs]) for i in (2, 3, 4, 5, 6)),
        ]
        hand_mask = torch.stack([row[7] for row in inputs])
        kwargs: dict[str, Any] = {
            "hand_mask": hand_mask.to(self.device, non_blocking=True),
            "think_steps": int(caches[0]["think_steps"]),
        }
        if any(int(cache["think_steps"]) != kwargs["think_steps"] for cache in caches):
            raise ValueError("batch rows use different think_steps")
        if placement_slots is not None:
            kwargs["placement_slots"] = placement_slots.to(self.device, dtype=torch.long, non_blocking=True)
        # Queue all tiny input copies on the current stream before inference;
        # blocking .to() synchronizes once per input even for pageable tensors.
        batched = [tensor.to(self.device, non_blocking=True) for tensor in batched]
        result = self.model(*batched, **kwargs)
        # Placement and slot decoding reuse tensors already sent to the model.
        result["_deck_ids"] = batched[2]
        result["_hand_mask"] = kwargs["hand_mask"]
        return result

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

    def _select_batch(self, distribution, caches, component, deterministic, support_counts=None):
        """Select independently per row without drawing RNG for greedy rows."""
        if deterministic is False or (not isinstance(deterministic, bool) and not any(deterministic)):
            return self._sample_with_uniform(distribution, caches, component)
        greedy = (distribution.probs >= .5 if isinstance(distribution, Bernoulli)
                  else distribution.logits.argmax(-1))
        if isinstance(deterministic, bool):
            return greedy if deterministic else self._sample_with_uniform(distribution, caches, component)
        if all(deterministic):
            return greedy
        if all("sample_uniforms" in cache for cache in caches):
            # Per-game uniforms consume no global RNG. Keep the vectorized
            # path; zero-probability padding cannot change the sampled support.
            sampled = self._sample_with_uniform(distribution, caches, component)
            return torch.where(torch.tensor(deterministic, device=self.device), greedy, sampled)
        # The original FastBananaPolicy uses torch's global RNG. Restrict draws
        # to sampled rows AND their support width: multinomial's RNG use also
        # depends on category count, even for zero-probability padded entries.
        rows = [i for i, flag in enumerate(deterministic) if not flag]
        ix = torch.tensor(rows, device=self.device)
        if isinstance(distribution, Bernoulli):
            selected = Bernoulli(logits=distribution.logits[ix], validate_args=False)
        else:
            width = (max(support_counts[i] for i in rows) if support_counts is not None
                     else distribution.logits.shape[-1])
            selected = Categorical(logits=distribution.logits[ix, :width], validate_args=False)
        result = greedy.to(distribution.probs.dtype) if isinstance(distribution, Bernoulli) else greedy.clone()
        result[ix] = self._sample_with_uniform(selected, [caches[i] for i in rows], component)
        return result

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
        deterministic: bool | Sequence[bool] = False,
        legal_tile_masks: Sequence[Tensor | None] | None = None,
        placement_mask_provider=None,
    ) -> list[SampledAction]:
        """Sample a batch of independent observations with two model passes.

        One shared policy/device and shared decode parameters are used for the
        batch. ``legal_tile_masks`` is optional and aligned with ``requests``.
        ``deterministic`` is a bool or one bool per request. A placement provider
        may expose ``batch([(request, native_slot), ...])``; otherwise it is
        called once per playing row as ``provider(request, native_slot)``.
        Returned actions preserve request order and PPO's existing cache layout.
        """
        if not requests:
            return []
        if not isinstance(deterministic, bool):
            deterministic = tuple(bool(flag) for flag in deterministic)
            if len(deterministic) != len(requests):
                raise ValueError("deterministic must match requests length")
            if all(deterministic) or not any(deterministic):
                deterministic = all(deterministic)
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
            waits = self._select_batch(wait_dist, caches, 0, deterministic).bool()
            waits = waits | ~can_play
            logp = torch.where(can_play, wait_dist.log_prob(waits.float()), 0.)
            entropy = torch.where(can_play, wait_dist.entropy(), 0.)
            # Native masks need both wait decisions and chosen native slots on
            # the host. With per-game uniforms (or greedy selection), decoding
            # slots for every row is independent of which rows wait: transfer
            # the two decision vectors together instead of synchronizing twice.
            joint_slots = (placement_mask_provider is not None
                           and any(c["can_play"] for c in caches)
                           and (deterministic is True or all("sample_uniforms" in c for c in caches)))
            if joint_slots:
                slot_logits = first["slot_logits"] / max(float(temperature), 1e-4)
                slot_dist = Categorical(logits=slot_logits.masked_fill(
                    ~first["_hand_mask"], torch.finfo(slot_logits.dtype).min), validate_args=False)
                all_slots = self._select_batch(slot_dist, caches, 1, deterministic)
                decisions = torch.stack((waits.long(), all_slots), dim=1).cpu().tolist()
                wait_rows = [bool(row[0]) for row in decisions]
            else:
                wait_rows = waits.cpu().tolist()
            # Transfer each decision vector once, rather than synchronizing for
            # every action's wait/slot/tile/x/y scalar.
            pending = [i for i, wait in enumerate(wait_rows) if not wait]
            placements = {}
            if pending:
                subset = [caches[i] for i in pending]
                ix = torch.tensor(pending, device=self.device)
                row_modes = deterministic if isinstance(deterministic, bool) else tuple(deterministic[i] for i in pending)
                if joint_slots:
                    slots = all_slots[ix]
                    slot_logp = slot_dist.log_prob(all_slots)[ix]
                    slot_entropy = slot_dist.entropy()[ix]
                else:
                    mask = first["_hand_mask"][ix]
                    slot_logits = first["slot_logits"][ix] / max(float(temperature), 1e-4)
                    slot_dist = Categorical(logits=slot_logits.masked_fill(~mask, torch.finfo(slot_logits.dtype).min),
                                            validate_args=False)
                    slots = self._select_batch(slot_dist, subset, 1, row_modes)
                    slot_logp, slot_entropy = slot_dist.log_prob(slots), slot_dist.entropy()
                if placement_mask_provider is not None:
                    host_slots = [decisions[i][1] for i in pending] if joint_slots else slots.cpu().tolist()
                    pairs = [(requests[i], next(row[0] for row in caches[i]["legal"] if row[1] == slot))
                             for i, slot in zip(pending, host_slots)]
                    batch_provider = getattr(placement_mask_provider, "batch", None)
                    masks = (batch_provider(pairs) if batch_provider is not None else
                             [placement_mask_provider(request, slot) for request, slot in pairs])
                    if len(masks) != len(pending):
                        raise ValueError("native placement masks must match playing rows")
                    for i, native_mask in zip(pending, masks):
                        mask = torch.as_tensor(native_mask, dtype=torch.bool).flatten().cpu()
                        if mask.numel() != 576 or not mask.any():
                            raise ValueError("native placement mask must contain 576 tiles and a legal target")
                        supplied = caches[i]["tile_mask"]
                        caches[i]["tile_mask"] = mask if supplied is None else mask & supplied
                second = self._placement_from_context(first, caches, pending, slots)
                tile_dist, indices, counts = self._tile_batch(second["tile_logits"], subset)
                choices = self._select_batch(tile_dist, subset, 2, row_modes, counts)
                tiles = indices.gather(1, choices[:, None]).squeeze(1)
                xy = torch.stack(((tiles % 32 + .5) / 32, (tiles // 32 + .5) / 18), dim=1)
                if second.get("offset") is not None:
                    xy = (xy + second["offset"].detach() * xy.new_tensor([.5 / 32, .5 / 18])).clamp(0., 1.)
                red = torch.tensor([sides[i] == 1 for i in pending], device=self.device)
                xy[:, 1] = torch.where(red, 1. - xy[:, 1], xy[:, 1])
                coords = torch.round(xy * xy.new_tensor([18000., 32000.])).clamp_min(0)
                coords = torch.minimum(coords, coords.new_tensor([18000., 32000.])).long()
                logp[ix] += slot_logp + tile_dist.log_prob(choices)
                entropy[ix] += slot_entropy + tile_dist.entropy()
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
                mask = first["_hand_mask"][ix]
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
