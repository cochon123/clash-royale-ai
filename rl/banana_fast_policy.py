"""Batched inference/PPO adapter for the unchanged original BananaPolicy."""
from __future__ import annotations

from collections.abc import Mapping, Sequence
from typing import Any

import torch
from torch import Tensor
from torch.distributions import Bernoulli, Categorical
from torch.nn import functional as F

try:
    from .banana_policy import BananaEpisode, BananaPolicy, SampledAction
except ImportError:
    from banana_policy import BananaEpisode, BananaPolicy, SampledAction


class FastBananaPolicy(BananaPolicy):
    """Batch independent native observations while preserving original semantics.

    ``act_batch`` takes ``(episode, observation, side, now_seconds)`` requests
    for one policy instance. Every request uses the authoritative native hand,
    elixir, selected-card conditioning, and PPO placement candidate support from
    :class:`BananaPolicy`. Sequence histories are right-padded only for the
    model call; their true lengths remain unchanged.
    """

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

    def act_batch(
        self,
        requests: Sequence[tuple[BananaEpisode, Mapping[str, Any], int, float]],
        *,
        temperature: float = 1.0,
        placement_temperature: float = 0.8,
        placement_top_k: int = 5,
        deterministic: bool = False,
        legal_tile_masks: Sequence[Tensor | None] | None = None,
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
            wait_logits = self.wait_head(first["fused"]).squeeze(-1)
            value_rows = self.critic(first["fused"]).squeeze(-1)
            pending: list[tuple[int, Tensor, Tensor, Tensor, int, int, str]] = []
            results: list[SampledAction | None] = [None] * len(caches)
            for i, cache in enumerate(caches):
                wait_dist = Bernoulli(logits=wait_logits[i])
                legal = cache["legal"]
                legal_deck_slots = tuple(sorted({row[1] for row in legal}))
                value = value_rows[i].detach()
                if not cache["can_play"]:
                    wait_choice = True
                    log_prob = first["fused"][i].sum() * 0.0
                    entropy = log_prob
                else:
                    wait_choice = (
                        bool(wait_dist.sample().item())
                        if not deterministic
                        else bool(wait_dist.probs.item() >= 0.5)
                    )
                    wait_value = torch.tensor(float(wait_choice), device=self.device)
                    log_prob = wait_dist.log_prob(wait_value)
                    entropy = wait_dist.entropy()
                if wait_choice:
                    results[i] = SampledAction(
                        sides[i], -1, -1, "", 0, 0, -1,
                        log_prob.detach(), entropy.detach(), value,
                        legal_deck_slots, True, cache,
                    )
                    continue
                slot_logits = first["slot_logits"][i] / max(float(temperature), 1e-4)
                mask = cache["inputs"][7].to(self.device)
                slot_dist = Categorical(
                    logits=slot_logits.masked_fill(~mask, torch.finfo(slot_logits.dtype).min)
                )
                slot_tensor = slot_logits.argmax() if deterministic else slot_dist.sample()
                deck_slot = int(slot_tensor.item())
                native_slot, _, card = next(row for row in legal if row[1] == deck_slot)
                log_prob = log_prob + slot_dist.log_prob(slot_tensor)
                entropy = entropy + slot_dist.entropy()
                pending.append((i, slot_tensor, log_prob, entropy, native_slot, deck_slot, card))

            if pending:
                subset = [caches[row[0]] for row in pending]
                selected_slots = torch.stack([row[1] for row in pending]).long()
                second = self._forward_batch(subset, selected_slots)
                for batch_index, row in enumerate(pending):
                    i, _slot_tensor, log_prob, entropy, native_slot, deck_slot, card = row
                    cache = caches[i]
                    tile_dist, indices = self._tile_distribution(
                        second["tile_logits"][batch_index], cache
                    )
                    cache["tile_candidates"] = indices.detach().cpu().clone()
                    choice = tile_dist.logits.argmax() if deterministic else tile_dist.sample()
                    tile = int(indices[choice].item())
                    xy = torch.tensor(
                        [((tile % 32) + 0.5) / 32, ((tile // 32) + 0.5) / 18],
                        device=self.device,
                    )
                    offset = second.get("offset")
                    if offset is not None:
                        xy = (
                            xy + offset[batch_index].detach()
                            * xy.new_tensor([0.5 / 32, 0.5 / 18])
                        ).clamp(0.0, 1.0)
                    if sides[i] == 1:
                        xy = torch.stack((xy[0], 1.0 - xy[1]))
                    x = int(torch.round(xy[0] * 18000).clamp(0, 18000).item())
                    y = int(torch.round(xy[1] * 32000).clamp(0, 32000).item())
                    log_prob = log_prob + tile_dist.log_prob(choice)
                    entropy = entropy + tile_dist.entropy()
                    results[i] = SampledAction(
                        sides[i], native_slot, deck_slot, card, x, y, tile,
                        log_prob.detach(), entropy.detach(), value_rows[i].detach(),
                        tuple(sorted({d for _, d, _ in cache["legal"]})), False, cache,
                    )
            if any(result is None for result in results):
                raise RuntimeError("internal error: batch action was not filled")
            return [result for result in results if result is not None]

    def evaluate_actions(self, actions: Sequence[SampledAction]) -> tuple[Tensor, Tensor, Tensor]:
        """Batch PPO likelihood/value recomputation, including placement conditioning."""
        if not actions:
            empty = torch.empty(0, device=self.device)
            return empty, empty, empty
        caches = [action.input_cache for action in actions]
        log_probs: list[Tensor | None] = [None] * len(actions)
        entropies: list[Tensor | None] = [None] * len(actions)
        with torch.backends.cudnn.flags(enabled=False):
            first = self._forward_batch(caches)
            wait_logits = self.wait_head(first["fused"]).squeeze(-1)
            values = self.critic(first["fused"]).squeeze(-1)
            pending: list[int] = []
            for i, action in enumerate(actions):
                cache = caches[i]
                wait_dist = Bernoulli(logits=wait_logits[i])
                wait_value = torch.tensor(float(action.wait), device=self.device)
                can_play = bool(cache["can_play"])
                if can_play:
                    log_prob = wait_dist.log_prob(wait_value)
                    entropy = wait_dist.entropy()
                else:
                    log_prob = first["fused"][i].sum() * 0.0
                    entropy = log_prob
                if not action.wait:
                    slot_logits = first["slot_logits"][i] / max(float(cache["temperature"]), 1e-4)
                    mask = cache["inputs"][7].to(self.device)
                    slot_dist = Categorical(
                        logits=slot_logits.masked_fill(~mask, torch.finfo(slot_logits.dtype).min)
                    )
                    slot_tensor = torch.tensor(action.deck_slot, device=self.device)
                    log_prob = log_prob + slot_dist.log_prob(slot_tensor)
                    entropy = entropy + slot_dist.entropy()
                    pending.append(i)
                log_probs[i] = log_prob
                entropies[i] = entropy

            if pending:
                second = self._forward_batch(
                    [caches[i] for i in pending],
                    torch.tensor([actions[i].deck_slot for i in pending], device=self.device),
                )
                for batch_index, i in enumerate(pending):
                    cache = caches[i]
                    action = actions[i]
                    tile_dist, indices = self._tile_distribution(
                        second["tile_logits"][batch_index], cache
                    )
                    choices = (indices == action.tile).nonzero(as_tuple=False).flatten()
                    if choices.numel() != 1:
                        raise ValueError(
                            f"stored tile {action.tile} is not in this action's legal top-k set"
                        )
                    choice = choices[0]
                    log_probs[i] = log_probs[i] + tile_dist.log_prob(choice)
                    entropies[i] = entropies[i] + tile_dist.entropy()
        return (
            torch.stack([value for value in log_probs if value is not None]),
            torch.stack([value for value in entropies if value is not None]),
            values,
        )
