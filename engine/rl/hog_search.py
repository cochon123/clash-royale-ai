"""Snapshot lookahead and expert iteration (C7).

``ExpertIteration.search`` ranks the learner's legal (card, tile) pairs, rolls
each candidate forward on the slot's own engine session, and restores that
session before returning. The engine reply and audit after ``search`` match
the path that never searched. ``aux_loss`` is KL(target || policy) on the
searched choices, recomputed from the stored policy inputs.
"""
from __future__ import annotations

import time
from dataclasses import dataclass, field
from typing import Any, Callable, Mapping, Sequence

import torch
from torch import Tensor
from torch.nn import functional as F

from .banana_hog_retrain import tower_potential
from .board_rewards import elixir_trade_potential

TOWER_ROWS = ("KingTower", "PrincessTower")
RIVER_Y = 16000
IGNORED_KINDS = frozenset({"tower", "projectile", "area_effect"})
HOG_COMPACT = frozenset({"hogrider"})


@dataclass
class SearchConfig:
    candidates: int = 6
    horizon_ticks: int = 120
    search_fraction: float = 0.05
    temperature: float = 0.5
    aux_coef: float = 0.5
    # Leaf value added to Q. Off: Q is only the potential delta.
    critic_bootstrap: bool = False

    def __post_init__(self) -> None:
        if self.candidates < 1:
            raise ValueError("candidates must be positive")
        if self.horizon_ticks < 1:
            raise ValueError("horizon_ticks must be positive")
        if not 0.0 <= float(self.search_fraction) <= 1.0:
            raise ValueError("search_fraction must be in [0, 1]")
        if self.temperature <= 0:
            raise ValueError("temperature must be positive")


@dataclass
class SearchCandidate:
    """One action inside a search, in the same order as ``SearchTarget.probabilities``."""

    wait: bool
    card: str | None = None
    tile: int | None = None
    native_slot: int | None = None
    deck_slot: int | None = None
    x: int | None = None
    y: int | None = None
    q: float = 0.0
    tile_mask: Tensor | None = field(default=None, repr=False)

    def native_action(self, side: int) -> dict[str, int] | None:
        if self.wait:
            return None
        return {"side": int(side), "slot": int(self.native_slot), "x": int(self.x), "y": int(self.y)}


@dataclass
class SearchTarget:
    side: int
    candidates: tuple[SearchCandidate, ...]
    probabilities: Tensor
    q_values: tuple[float, ...]
    cache: dict[str, Any]
    chosen: int
    elapsed_ms: float
    root_potential: float


def tile_native(tile: int, side: int) -> tuple[int, int]:
    """Native arena point at the center of a placement tile. Side 1 mirrors Y."""
    col = int(tile) % 32
    row = int(tile) // 32
    x = int(round((col + 0.5) * 18000.0 / 32.0))
    y_norm = (row + 0.5) / 18.0
    if int(side) == 1:
        y_norm = 1.0 - y_norm
    y = int(round(y_norm * 32000.0))
    return x, y


def tower_state_from_board(board: Mapping[str, Any]) -> dict[str, Any]:
    towers = []
    for entity in board.get("entities", []):
        if entity.get("row") not in TOWER_ROWS:
            continue
        towers.append({
            "id": entity["id"],
            "side": entity["side"],
            "row": entity["row"],
            "hp": entity["hp"],
            "max_hp": entity["max_hp"],
        })
    return {"towers": towers}


def combined_potential(board: Mapping[str, Any], elixir: Sequence[float], side: int, row_table) -> float:
    """Tower-HP potential plus elixir-trade potential, from a C1 board."""
    tower = tower_potential(tower_state_from_board(board), side)
    trade = elixir_trade_potential(
        board, side, float(elixir[side]), float(elixir[1 - side]), row_table,
    )
    return float(tower) + float(trade.potential)


def q_delta(
    before_board: Mapping[str, Any],
    before_elixir: Sequence[float],
    after_board: Mapping[str, Any],
    after_elixir: Sequence[float],
    side: int,
    row_table,
) -> float:
    """Change in tower-HP potential + elixir-trade potential over a horizon."""
    return combined_potential(after_board, after_elixir, side, row_table) - combined_potential(
        before_board, before_elixir, side, row_table,
    )


def _compact(name: str) -> str:
    return "".join(ch.lower() for ch in str(name) if ch.isalnum())


def _critic_pair(policy):
    critic = getattr(policy, "critic", None)
    return critic, getattr(critic, "_embed", None)


def _restore_embed(critic, embed) -> None:
    if critic is not None and hasattr(critic, "set_embed"):
        critic.set_embed(embed)


def candidate_log_probs(policy, cache: Mapping[str, Any], candidates: Sequence[SearchCandidate]) -> Tensor:
    """Differentiable log π(c) for each searched choice, before renormalizing.

    Wait scores log P(wait). A card scores log P(play) + log P(slot) + log P(tile),
    with the temperatures and the legal tile mask stored on that candidate.
    """
    if not candidates:
        return torch.empty(0, device=policy.device)
    critic, saved = _critic_pair(policy)
    try:
        first = policy._forward_batch([cache])
        if not cache.get("can_play", True):
            zeros = first["fused"].reshape(-1)[0] * 0.0
            return torch.stack([zeros for _ in candidates])
        wait_logit = policy.wait_head(first["fused"]).reshape(-1)[0]
        log_wait = F.logsigmoid(wait_logit)
        log_play = F.logsigmoid(-wait_logit)
        temperature = max(float(cache["temperature"]), 1e-4)
        placement_temperature = max(float(cache["placement_temperature"]), 1e-4)
        slot_logits = first["slot_logits"][0] / temperature
        slot_logits = slot_logits.masked_fill(~first["_hand_mask"][0], torch.finfo(slot_logits.dtype).min)
        log_slot = F.log_softmax(slot_logits, dim=-1)
        log_tiles: dict[int, Tensor] = {}
        for candidate in candidates:
            if candidate.wait or candidate.deck_slot in log_tiles:
                continue
            if candidate.tile_mask is None:
                raise ValueError("a played search candidate needs its legal tile mask")
            copied = dict(cache)
            copied["tile_mask"] = candidate.tile_mask
            slots = torch.tensor([int(candidate.deck_slot)], dtype=torch.long, device=policy.device)
            second = policy._placement_from_context(first, [copied], [0], slots)
            logits = second["tile_logits"][0] / placement_temperature
            mask = candidate.tile_mask.to(device=logits.device, dtype=torch.bool)
            if mask.numel() != logits.numel():
                raise ValueError("tile mask does not match the placement head")
            logits = logits.masked_fill(~mask, torch.finfo(logits.dtype).min)
            log_tiles[int(candidate.deck_slot)] = F.log_softmax(logits, dim=-1)
        scores = []
        for candidate in candidates:
            if candidate.wait:
                scores.append(log_wait)
            else:
                scores.append(
                    log_play
                    + log_slot[int(candidate.deck_slot)]
                    + log_tiles[int(candidate.deck_slot)][int(candidate.tile)]
                )
        return torch.stack(scores)
    finally:
        _restore_embed(critic, saved)


class ExpertIteration:
    def __init__(self, config: SearchConfig, row_table):
        self.config = config
        self.row_table = row_table
        self.decisions = 0
        self.searches = 0

    def key_moment(self, observation: Mapping[str, Any], board: Mapping[str, Any], side: int) -> bool:
        return (
            self._hog_ready(observation, side)
            or self._enemy_crossed(board, side)
            or self._tower_in_danger(board, side)
        )

    def should_search(self, observation: Mapping[str, Any], board: Mapping[str, Any], side: int, rng) -> bool:
        """True only at a key moment, and never above ``search_fraction`` of decisions."""
        self.decisions += 1
        fraction = float(self.config.search_fraction)
        if fraction <= 0.0 or not self.key_moment(observation, board, side):
            return False
        if (self.searches + 1) > fraction * self.decisions + 1e-9:
            return False
        if rng.random() >= min(1.0, fraction):
            return False
        self.searches += 1
        return True

    def search(
        self,
        pool,
        slot: int,
        policy,
        episode,
        side: int,
        observation: Mapping[str, Any],
        board: Mapping[str, Any],
        opponent_act: Callable[..., Any] | Mapping[str, Any] | None = None,
    ) -> SearchTarget | None:
        """Look ahead on this slot and leave the engine on the pre-search state.

        ``opponent_act=None`` submits no opponent action. A dict is submitted
        once, at the start of the horizon. A callable is invoked as
        ``opponent_act(observation, board)`` and may return a dict or None.
        """
        started = time.perf_counter()
        events = list(episode.events)
        board_cache = getattr(policy, "_board_cache", None)
        board_items = () if not board_cache else tuple(board_cache.items())
        side = int(side)
        view = dict(observation)
        view["board"] = board
        view["perspective"] = side
        if "tower_state" not in view:
            view["tower_state"] = tower_state_from_board(board)
        now = float(view.get("tick", 0)) / 20.0
        cache = policy._cache_request(
            policy, episode, view, side, now, 1.0, 0.8, 576, None,
        )
        root = pool.call(slot, {"op": "snapshot"})
        snapshot_id = int(root["snapshot_id"])
        try:
            plays = self._legal_plays(pool, slot, side, snapshot_id, cache)
            ranked = self._top_plays(policy, cache, plays)
            wait = SearchCandidate(wait=True)
            chosen = tuple(ranked) + (wait,)
            before_elixir = view["elixir"]
            root_potential = combined_potential(board, before_elixir, side, self.row_table)
            opponent = self._opponent_action(opponent_act, view, board)
            q_values = []
            for candidate in chosen:
                q_values.append(self._rollout(
                    pool, slot, policy, episode, side, snapshot_id, candidate, opponent,
                    board, before_elixir, now,
                ))
            for candidate, q in zip(chosen, q_values):
                candidate.q = float(q)
            temperature = max(float(self.config.temperature), 1e-4)
            scores = torch.tensor(q_values, dtype=torch.float32)
            probabilities = torch.softmax(scores / temperature, dim=0).detach().cpu()
            target = SearchTarget(
                side=side,
                candidates=chosen,
                probabilities=probabilities,
                q_values=tuple(float(q) for q in q_values),
                cache=cache,
                chosen=int(torch.argmax(probabilities).item()),
                elapsed_ms=(time.perf_counter() - started) * 1000.0,
                root_potential=float(root_potential),
            )
        finally:
            try:
                pool.call(slot, {"op": "restore", "snapshot_id": snapshot_id})
            finally:
                pool.call(slot, {"op": "drop_snapshot", "snapshot_id": snapshot_id})
        if list(episode.events) != events:
            raise RuntimeError("search mutated the episode history")
        after_cache = getattr(policy, "_board_cache", None)
        after_items = () if not after_cache else tuple(after_cache.items())
        if len(after_items) != len(board_items) or any(
            key != other or value is not same
            for (key, value), (other, same) in zip(board_items, after_items)
        ):
            raise RuntimeError("search mutated the policy board cache")
        return target

    def aux_loss(self, policy, targets: list[SearchTarget]) -> Tensor:
        """Mean KL(target || policy) over searched choices. Empty input is 0."""
        if not targets:
            return torch.zeros((), device=policy.device)
        losses = []
        for target in targets:
            log_pi = candidate_log_probs(policy, target.cache, target.candidates)
            log_policy = log_pi - torch.logsumexp(log_pi, dim=0)
            probs = target.probabilities.to(device=log_policy.device, dtype=log_policy.dtype)
            probs = probs.clamp_min(0)
            probs = probs / probs.sum().clamp_min(1e-12)
            log_target = probs.clamp_min(1e-12).log()
            losses.append((probs * (log_target - log_policy)).sum())
        return torch.stack(losses).mean()

    def _legal_plays(self, pool, slot, side, snapshot_id, cache) -> list[SearchCandidate]:
        plays = []
        for native_slot, deck_slot, card in cache["legal"]:
            pool.call(slot, {"op": "restore", "snapshot_id": snapshot_id})
            reply = pool.call(slot, {
                "op": "placement_mask", "side": side, "slot": int(native_slot),
            })
            mask = torch.as_tensor(reply["mask"], dtype=torch.bool).flatten().cpu()
            if mask.numel() != 576:
                raise ValueError("native placement mask must contain 576 tiles")
            for tile in mask.nonzero(as_tuple=False).flatten().tolist():
                x, y = tile_native(int(tile), side)
                plays.append(SearchCandidate(
                    wait=False, card=str(card), tile=int(tile),
                    native_slot=int(native_slot), deck_slot=int(deck_slot),
                    x=x, y=y, tile_mask=mask,
                ))
        return plays

    def _top_plays(self, policy, cache, plays: list[SearchCandidate]) -> list[SearchCandidate]:
        if not plays:
            return []
        with torch.no_grad():
            scores = candidate_log_probs(policy, cache, plays).detach().cpu()
        order = sorted(
            range(len(plays)),
            key=lambda index: (-float(scores[index]), int(plays[index].tile), str(plays[index].card)),
        )
        return [plays[index] for index in order[: int(self.config.candidates)]]

    def _rollout(
        self, pool, slot, policy, episode, side, snapshot_id, candidate, opponent,
        board, before_elixir, now,
    ) -> float:
        pool.call(slot, {"op": "restore", "snapshot_id": snapshot_id})
        actions = []
        native = candidate.native_action(side)
        if native is not None:
            actions.append(native)
        if opponent is not None:
            actions.append(dict(opponent))
        reply = pool.call(slot, {
            "op": "step", "ticks": int(self.config.horizon_ticks), "actions": actions,
        })
        after_board = reply.get("board")
        if not isinstance(after_board, Mapping):
            raise RuntimeError("search horizon reply has no board; reset with board=true")
        after_elixir = reply["observation"]["elixir"]
        q = q_delta(board, before_elixir, after_board, after_elixir, side, self.row_table)
        if self.config.critic_bootstrap:
            q += self._leaf_value(policy, episode, reply, side, now)
        return float(q)

    def _leaf_value(self, policy, episode, reply, side, now) -> float:
        leaf = dict(reply["observation"])
        leaf["board"] = reply["board"]
        leaf["perspective"] = side
        leaf["tower_state"] = reply.get("reward_state") or tower_state_from_board(reply["board"])
        critic, saved = _critic_pair(policy)
        try:
            cache = policy._cache_request(policy, episode, leaf, side, now + self.config.horizon_ticks / 20.0, 1.0, 0.8, 1, None)
            first = policy._forward_batch([cache])
            value = policy.critic(first["fused"]).reshape(-1)[0]
            return float(value.detach())
        finally:
            _restore_embed(critic, saved)

    def _hog_ready(self, observation: Mapping[str, Any], side: int) -> bool:
        elixir = float(observation["elixir"][side])
        for card in observation["hands"][side]:
            if not card.get("available", True):
                continue
            if _compact(card.get("name", "")) not in HOG_COMPACT:
                continue
            if float(card.get("cost", 4)) <= elixir + 1e-6:
                return True
        return False

    def _enemy_crossed(self, board: Mapping[str, Any], side: int) -> bool:
        for entity in board.get("entities", []):
            if int(entity.get("side", side)) == side or self._ignored(entity):
                continue
            y = int(entity["y"])
            if side == 0 and y < RIVER_Y:
                return True
            if side == 1 and y > RIVER_Y:
                return True
        return False

    def _tower_in_danger(self, board: Mapping[str, Any], side: int) -> bool:
        towers = [
            entity for entity in board.get("entities", [])
            if entity.get("row") in TOWER_ROWS and int(entity.get("side", -1)) == side
        ]
        for tower in towers:
            maximum = float(tower["max_hp"])
            if maximum <= 0:
                continue
            fraction = float(tower["hp"]) / maximum
            if tower["row"] == "KingTower" and fraction < 0.8:
                return True
            if tower["row"] == "PrincessTower" and fraction < 0.5:
                return True
        for entity in board.get("entities", []):
            if int(entity.get("side", side)) == side or self._ignored(entity):
                continue
            for tower in towers:
                dx = float(entity["x"]) - float(tower["x"])
                dy = float(entity["y"]) - float(tower["y"])
                if dx * dx + dy * dy <= 4500.0 * 4500.0:
                    return True
        return False

    def _ignored(self, entity: Mapping[str, Any]) -> bool:
        row = str(entity.get("row", ""))
        if row in TOWER_ROWS:
            return True
        table = self.row_table
        if table is None:
            return False
        kind = getattr(table.lookup(row), "kind", "unit")
        return kind in IGNORED_KINDS

    @staticmethod
    def _opponent_action(opponent_act, observation, board):
        if opponent_act is None:
            return None
        if callable(opponent_act):
            action = opponent_act(observation, board)
        else:
            action = opponent_act
        if action is None:
            return None
        if not isinstance(action, Mapping):
            raise TypeError("opponent_act must return an action dict or None")
        return dict(action)
