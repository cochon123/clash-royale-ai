"""C7 search stub used until ``rl/hog_search.py`` lands.

The real module is owned by the search workstream. This stub has the same
surface: ``SearchConfig``, ``SearchTarget``, and ``ExpertIteration``. ``search``
may snapshot and restore the slot, then returns the engine to its pre-search
state. It does not append episode events and does not write policy caches.
``aux_loss`` is KL(target || policy) on the wait decision, recomputed from the
stored rollout cache, and it is differentiable through the policy.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Sequence

import torch
from torch.nn import functional as F


@dataclass
class SearchConfig:
    candidates: int = 6
    horizon_ticks: int = 120
    search_fraction: float = 0.05
    temperature: float = 0.5
    aux_coef: float = 0.5


@dataclass
class SearchTarget:
    side: int
    wait_prob: float
    cache: dict | None = None
    card_index: int | None = None
    note: str = "stub"
    extras: dict = field(default_factory=dict)


class ExpertIteration:
    def __init__(self, config: SearchConfig, row_table: Any):
        self.config = config
        self.row_table = row_table
        self.searches = 0

    def should_search(self, observation: dict, board: dict, side: int, rng) -> bool:
        if self.config.search_fraction <= 0:
            return False
        if not isinstance(board, dict) or "entities" not in board:
            return False
        return float(rng.random()) < float(self.config.search_fraction)

    def search(self, pool, slot: int, policy, episode, side: int,
               observation: dict, board: dict, opponent_act=None) -> SearchTarget | None:
        """Leave the engine, the episode, and the policy caches unchanged."""
        del policy, observation, opponent_act
        events_before = len(getattr(episode, "events", ()))
        snap_id = None
        if pool is not None:
            taken = pool.call(slot, {"op": "snapshot"})
            snap_id = taken["snapshot_id"]
            pool.call(slot, {"op": "restore", "snapshot_id": snap_id})
            pool.call(slot, {"op": "drop_snapshot", "snapshot_id": snap_id})
        if len(episode.events) != events_before:
            raise RuntimeError("search stub mutated the episode")
        self.searches += 1
        # A fixed target. The trainer attaches the rollout cache after return
        # when the stub did not need to encode anything.
        return SearchTarget(side=int(side), wait_prob=0.25, cache=None, note="stub",
                            extras={"snapshot_id": snap_id, "candidates": self.config.candidates})

    def aux_loss(self, policy, targets: Sequence[SearchTarget]) -> torch.Tensor:
        usable = [target for target in targets if target is not None and target.cache is not None]
        if not usable:
            return policy.wait_head.weight.sum() * 0
        current = policy._forward_batch([target.cache for target in usable])
        wait_logit = policy.wait_head(current["fused"]).squeeze(-1)
        target = wait_logit.new_tensor([float(item.wait_prob) for item in usable]).clamp(1e-4, 1 - 1e-4)
        log_q = F.logsigmoid(wait_logit)
        log_q_neg = F.logsigmoid(-wait_logit)
        kl = target * (torch.log(target) - log_q) + (1 - target) * (torch.log(1 - target) - log_q_neg)
        return self.config.aux_coef * kl.mean()
