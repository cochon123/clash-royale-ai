"""Pure pieces of the Hog 2.6 trainer: PFSP, rewards, GAE, state bank, ETA.

Nothing here opens an engine process. The trainer in ``crazy_hog_train`` calls
these helpers and writes the audit files.
"""
from __future__ import annotations

import copy
import random
from collections import Counter, deque
from typing import Any, Mapping, Sequence

import torch

from .banana_hog_retrain import gae_returns, shaped_rewards
from .board_rewards import audit_elixir_trade, shaped_elixir_terms

PINNED_ENGINE_PREFIX = "32b6deb2"
WIN_ROWS = frozenset({
    "Pekka", "BattleRam", "Golem", "LavaHound", "Balloon",
    "GoblinBarrel", "GoblinBarrelSpell", "HogRider",
})
TOWER_ROWS = frozenset({"KingTower", "PrincessTower"})
HOG_CONNECT_RANGE = 4500.0
HOG_CONNECT_TICKS = 160  # 8 seconds at 20 ticks/second
BRIDGE_Y = 16000
RESULT_SCORE = {"W": 1.0, "D": 0.5, "L": 0.0}
_SHARED_STATE_FIELDS = frozenset({"dataset", "vocab", "costs"})

CRAZY_RESUME_FIELDS = (
    "seed", "workers", "eval_seeds", "confirmation_seeds", "match_ticks", "duration_seconds",
    "learning_rate", "critic_learning_rate", "backbone_learning_rate", "board_learning_rate",
    "shaping_coefficient", "trade_coef", "gae_lambda", "card_anchor", "wait_anchor",
    "target_kl", "max_parent_kl", "entropy_coef", "regression_threshold", "epochs",
    "minibatch_size", "async_collection", "max_policy_lag", "async_ready_games", "engine_pool",
    "step_groups", "state_bank_fraction", "max_snapshots", "bank_horizon_seconds",
    "snapshot_every", "snapshot_keep", "pfsp_power", "pfsp_floor", "search", "trainable",
    "native_placement_masks", "allow_unpinned_engine",
)


def assert_engine_pin(digest: str, allow: bool = False) -> None:
    if allow:
        return
    if not str(digest).startswith(PINNED_ENGINE_PREFIX):
        raise RuntimeError(
            f"engine sha256 {digest} is not the pinned PGO binary ({PINNED_ENGINE_PREFIX}…)"
        )


def _json_value(value: Any) -> Any:
    if isinstance(value, bool) or value is None or isinstance(value, (int, float, str)):
        return value
    return str(value)


def validate_crazy_resume(saved: Mapping[str, Any], args: Any, source_hashes: Mapping[str, str]) -> None:
    """Refuse a resume that would silently change the experiment or its source."""
    for name, digest in source_hashes.items():
        key = f"{name}_sha256"
        recorded = saved.get(key)
        if recorded and recorded != digest:
            raise ValueError(f"resume refused: {name} changed")
    stored = saved.get("args") or {}
    for key in CRAZY_RESUME_FIELDS:
        if key not in stored:
            continue
        if stored[key] != _json_value(getattr(args, key)):
            raise ValueError(f"resume cannot change experiment setting {key}")
    if saved.get("format") not in (None, "crazy-hog-v1"):
        raise ValueError("resume manifest is not crazy-hog-v1")


class PFSPSampler:
    """Matchup sampler. Weight is ``floor + (1 - score) ** power``.

    ``score`` is the learner's mean result on that (deck, controller) cell
    (win 1, draw 0.5, loss 0). Hard matchups are sampled more often. A floor
    keeps every legal cell in the rotation. Self-play controllers are restricted
    to the Hog mirror deck.
    """

    def __init__(self, decks: Sequence[Mapping[str, Any]], controllers: Sequence[str], seed: int,
                 power: float = 2.0, floor: float = 0.05, mirror_id: str = "hog-mirror",
                 selfplay_prefix: str = "selfplay-"):
        if power < 0:
            raise ValueError("pfsp power must be non-negative")
        if floor < 0:
            raise ValueError("pfsp floor must be non-negative")
        self.power = float(power)
        self.floor = float(floor)
        self.mirror_id = mirror_id
        self.selfplay_prefix = selfplay_prefix
        self.rng = random.Random(seed)
        self.cells: list[tuple[str, str]] = []
        self.results: dict[tuple[str, str], deque] = {}
        self.seats: Counter = Counter()
        self.retired: dict[tuple[str, str], list] = {}
        for deck in decks:
            if deck.get("holdout"):
                continue
            for controller in controllers:
                self._add_cell(deck["id"], controller)
        self.warm = self.cells[:]
        self.rng.shuffle(self.warm)

    def _add_cell(self, deck_id: str, controller: str) -> None:
        if controller.startswith(self.selfplay_prefix) and deck_id != self.mirror_id:
            return
        cell = (deck_id, controller)
        if cell in self.results:
            return
        self.cells.append(cell)
        self.results[cell] = deque(maxlen=200)

    def add_controller(self, controller: str, deck_ids: Sequence[str]) -> None:
        before = set(self.cells)
        for deck_id in deck_ids:
            self._add_cell(deck_id, controller)
        self.warm.extend(cell for cell in self.cells if cell not in before)

    def remove_controller(self, controller: str) -> None:
        kept = []
        for cell in self.cells:
            if cell[1] == controller:
                self.retired[cell] = list(self.results.pop(cell, ()))
            else:
                kept.append(cell)
        self.cells = kept
        self.warm = [cell for cell in self.warm if cell[1] != controller]

    def score(self, cell: tuple[str, str]) -> float:
        rows = self.results.get(cell) or ()
        if not rows:
            return 0.5
        return sum(RESULT_SCORE[item] for item in rows) / len(rows)

    def weight(self, cell: tuple[str, str]) -> float:
        return self.floor + (1.0 - self.score(cell)) ** self.power

    def probabilities(self) -> list[float]:
        if not self.cells:
            return []
        weights = [self.weight(cell) for cell in self.cells]
        total = sum(weights)
        if total <= 0:
            share = 1.0 / len(self.cells)
            return [share] * len(self.cells)
        return [item / total for item in weights]

    def draw(self) -> dict[str, Any]:
        if not self.cells:
            raise RuntimeError("PFSP sampler has no matchups")
        cell = self.warm.pop() if self.warm else self.rng.choices(self.cells, self.probabilities())[0]
        seat = self.seats[cell] % 2
        self.seats[cell] += 1
        return {"deck_id": cell[0], "controller": cell[1], "seat": seat,
                "seed": self.rng.randrange(1, 1_000_000_000)}

    def record(self, job: Mapping[str, Any], result: str) -> None:
        if result not in RESULT_SCORE:
            return
        cell = (job["deck_id"], job["controller"])
        if cell in self.results:
            self.results[cell].append(result)

    def public_state(self) -> dict[str, Any]:
        probs = self.probabilities()
        rows = []
        for index, cell in enumerate(self.cells):
            history = list(self.results[cell])
            rows.append({
                "deck_id": cell[0], "controller": cell[1], "score": self.score(cell),
                "weight": self.weight(cell), "probability": probs[index] if probs else 0.0,
                "games": len(history), "record": dict(Counter(history)),
            })
        return {"power": self.power, "floor": self.floor, "warm_remaining": len(self.warm),
                "cells": rows, "retired": [list(cell) for cell in self.retired]}

    def state_dict(self) -> dict[str, Any]:
        return {
            "power": self.power, "floor": self.floor, "mirror_id": self.mirror_id,
            "selfplay_prefix": self.selfplay_prefix, "cells": self.cells, "rng": self.rng.getstate(),
            "seats": dict(self.seats), "warm": self.warm,
            "results": {cell: list(rows) for cell, rows in self.results.items()},
            "retired": self.retired,
        }

    @classmethod
    def from_state_dict(cls, state: Mapping[str, Any]) -> "PFSPSampler":
        result = object.__new__(cls)
        result.power = float(state["power"])
        result.floor = float(state["floor"])
        result.mirror_id = state["mirror_id"]
        result.selfplay_prefix = state["selfplay_prefix"]
        result.cells = [tuple(cell) for cell in state["cells"]]
        result.rng = random.Random()
        result.rng.setstate(state["rng"])
        result.seats = Counter()
        for key, value in state["seats"].items():
            result.seats[tuple(key)] = value
        result.warm = [tuple(cell) for cell in state["warm"]]
        result.results = {tuple(cell): deque(rows, maxlen=200) for cell, rows in state["results"].items()}
        result.retired = {tuple(cell): list(rows) for cell, rows in state.get("retired", {}).items()}
        return result


def gae_with_bootstrap(rewards: Sequence[float], values: Sequence[float], lam: float = 0.97,
                       bootstrap: float = 0.0) -> tuple[list[float], list[float]]:
    """GAE(λ). ``bootstrap`` is V(s_T) at truncation and 0 at a true terminal.

    With ``bootstrap == 0`` this matches ``banana_hog_retrain.gae_returns``.
    """
    if len(rewards) != len(values):
        raise ValueError("GAE rewards and values must have the same length")
    if not values:
        return [], []
    if bootstrap == 0.0:
        return gae_returns(list(rewards), list(values), lam)
    advantages = [0.0] * len(values)
    running = 0.0
    for index in reversed(range(len(values))):
        next_value = values[index + 1] if index + 1 < len(values) else float(bootstrap)
        running = rewards[index] + next_value - values[index] + lam * running
        advantages[index] = running
    return advantages, [adv + value for adv, value in zip(advantages, values)]


def open_shaped(potentials: Sequence[float], coefficient: float) -> list[float]:
    """Potential differences that keep the last measured Phi (truncated games)."""
    if len(potentials) < 2:
        raise ValueError("open shaping needs the initial potential and one successor")
    return [float(coefficient) * (float(after) - float(before))
            for before, after in zip(potentials, potentials[1:])]


def _close(left: float, right: float, tol: float = 1e-6) -> bool:
    return abs(float(left) - float(right)) <= tol


def audit_game_rewards(tower_potentials: Sequence[float], trade_potentials: Sequence[float], outcome: float,
                       tower_coef: float, trade_coef: float, *, terminal: bool,
                       tower_terms: Sequence[float] | None = None,
                       trade_terms: Sequence[float] | None = None, tol: float = 1e-6) -> dict[str, Any]:
    """Check tower-HP and elixir-trade shaping. Terminal Phi is 0 only when the game ends.

    A truncated bank episode keeps the last measured potential and adds no
    win/loss outcome. A true terminal adds the outcome once, on the tower term.
    """
    if len(tower_potentials) != len(trade_potentials):
        raise ValueError("tower and elixir trajectories must have the same length")
    if len(tower_potentials) < 2:
        raise ValueError("reward audit needs an initial potential and one successor")
    if terminal:
        expected_tower = shaped_rewards(list(tower_potentials), float(outcome), float(tower_coef))
        expected_trade = shaped_elixir_terms(trade_potentials, float(trade_coef))
        trade_audit = audit_elixir_trade(trade_potentials, float(trade_coef), expected_trade, tol=tol)
        tower_sum = float(sum(expected_tower))
        tower_expected = float(outcome) - float(tower_coef) * float(tower_potentials[0])
        if not _close(tower_sum, tower_expected, tol):
            raise RuntimeError(f"tower potential failed telescoping: sum={tower_sum} expected={tower_expected}")
    else:
        if abs(float(outcome)) > tol:
            raise RuntimeError("a truncated episode cannot carry a terminal outcome")
        expected_tower = open_shaped(tower_potentials, tower_coef)
        expected_trade = open_shaped(trade_potentials, trade_coef)
        tower_sum = float(sum(expected_tower))
        tower_expected = float(tower_coef) * (float(tower_potentials[-1]) - float(tower_potentials[0]))
        trade_sum = float(sum(expected_trade))
        trade_expected = float(trade_coef) * (float(trade_potentials[-1]) - float(trade_potentials[0]))
        if not _close(tower_sum, tower_expected, tol) or not _close(trade_sum, trade_expected, tol):
            raise RuntimeError("truncated potential failed telescoping")
        trade_audit = {"ok": True, "sum_shaped": trade_sum, "expected": trade_expected,
                       "initial_potential": float(trade_potentials[0]), "coefficient": float(trade_coef),
                       "steps": len(expected_trade), "terminal_potential": float(trade_potentials[-1])}
    if tower_terms is not None and (len(tower_terms) != len(expected_tower) or any(
            not _close(a, b, tol) for a, b in zip(tower_terms, expected_tower))):
        raise RuntimeError("shaped tower terms do not match the potential trajectory")
    if trade_terms is not None and (len(trade_terms) != len(expected_trade) or any(
            not _close(a, b, tol) for a, b in zip(trade_terms, expected_trade))):
        raise RuntimeError("shaped elixir terms do not match the potential trajectory")
    return {
        "ok": True,
        "terminal": bool(terminal),
        "outcome": float(outcome) if terminal else 0.0,
        "tower": {"initial_potential": float(tower_potentials[0]), "sum_shaped": float(sum(expected_tower)),
                  "expected": tower_expected, "coefficient": float(tower_coef), "steps": len(expected_tower)},
        "trade": trade_audit,
        "rewards": [tower + trade for tower, trade in zip(expected_tower, expected_trade)],
    }


def learner_half(seat: int, y: float) -> bool:
    """True when a physical ``y`` is on the learner's side of the river."""
    return float(y) < BRIDGE_Y if int(seat) == 0 else float(y) > BRIDGE_Y


def _compact(name: str) -> str:
    return "".join(ch.lower() for ch in str(name) if ch.isalnum())


def hog_in_hand(observation: Mapping[str, Any], seat: int, elixir_min: float = 4.0) -> bool:
    elixir = observation.get("elixir")
    hands = observation.get("hands")
    if not isinstance(elixir, Sequence) or len(elixir) != 2:
        return False
    if float(elixir[seat]) < elixir_min:
        return False
    if not isinstance(hands, Sequence) or len(hands) != 2:
        return False
    for row in hands[seat] or ():
        if not isinstance(row, Mapping):
            continue
        compact = _compact(row.get("name", ""))
        if compact in {"hogrider", "hog"}:
            return True
    return False


def critical_moments(board: Mapping[str, Any], observation: Mapping[str, Any], seat: int, *,
                     seen_cross: set[int], seen_towers: set[int], saw_double: bool,
                     saw_hog_hand: bool, elapsed_seconds: float) -> list[str]:
    """Reasons to snapshot, excluding reasons already taken on this episode."""
    reasons = []
    entities = board.get("entities") or ()
    for entity in entities:
        row = entity.get("row")
        identity = entity.get("id")
        if identity is None:
            continue
        if (row in WIN_ROWS and int(entity.get("side", seat)) != int(seat)
                and learner_half(seat, entity.get("y", BRIDGE_Y)) and identity not in seen_cross):
            reasons.append(f"bridge:{row}:{identity}")
        max_hp = float(entity.get("max_hp") or 0)
        hp = float(entity.get("hp") or 0)
        if row in TOWER_ROWS and max_hp > 0 and hp / max_hp < 0.5 and identity not in seen_towers:
            reasons.append(f"tower:{identity}")
    if elapsed_seconds >= 120.0 and not saw_double:
        reasons.append("double-elixir")
    if hog_in_hand(observation, seat) and not saw_hog_hand:
        reasons.append("hog-hand")
    return reasons


def note_critical(game_seen: dict[str, Any], reasons: Sequence[str]) -> None:
    for reason in reasons:
        if reason.startswith("bridge:"):
            game_seen["seen_cross"].add(int(reason.rsplit(":", 1)[-1]))
        elif reason.startswith("tower:"):
            game_seen["seen_towers"].add(int(reason.split(":", 1)[-1]))
        elif reason == "double-elixir":
            game_seen["saw_double"] = True
        elif reason == "hog-hand":
            game_seen["saw_hog_hand"] = True


def _distance(left: Mapping[str, Any], right: Mapping[str, Any]) -> float:
    return ((float(left["x"]) - float(right["x"])) ** 2 + (float(left["y"]) - float(right["y"])) ** 2) ** 0.5


def enemy_tower_hp(board: Mapping[str, Any], seat: int) -> float:
    total = 0.0
    for entity in board.get("entities") or ():
        if entity.get("row") in TOWER_ROWS and int(entity.get("side", -1)) != int(seat):
            hp = entity.get("hp")
            if isinstance(hp, (int, float)) and hp > 0:
                total += float(hp)
    return total


def update_hog_stats(stats: dict[str, Any], board: Mapping[str, Any], seat: int, tick: int,
                     new_events: Sequence[Mapping[str, Any]], elixir: float | None,
                     step_seconds: float = 1.0) -> dict[str, Any]:
    """Update per-game Hog diagnostics in place and return the same dict.

    A Hog play connects when that Hog is alive inside enemy tower range, or
    when an enemy tower loses HP within 8 seconds of the play.
    """
    learner_side = "team" if int(seat) == 0 else "opponent"
    for event in new_events:
        if event.get("side") != learner_side:
            continue
        card = str(event.get("card") or "")
        stats["plays"] += 1
        stats["card_counts"][card] = stats["card_counts"].get(card, 0) + 1
        if card == "hog-rider":
            stats["hog_plays"] += 1
            stats["pending_hogs"].append({"tick": int(tick), "connected": False})
    if elixir is not None and float(elixir) >= 9.999:
        stats["leak_seconds"] += float(step_seconds)
    hog_in_range = False
    towers = [entity for entity in (board.get("entities") or ())
              if entity.get("row") in TOWER_ROWS and int(entity.get("side", -1)) != int(seat)]
    for entity in board.get("entities") or ():
        hp = entity.get("hp")
        if entity.get("row") != "HogRider" or int(entity.get("side", -1)) != int(seat):
            continue
        if not isinstance(hp, (int, float)) or hp <= 0:
            continue
        if any(_distance(entity, tower) <= HOG_CONNECT_RANGE for tower in towers):
            hog_in_range = True
            break
    current_hp = enemy_tower_hp(board, seat)
    previous_hp = stats.get("enemy_tower_hp")
    damaged = previous_hp is not None and current_hp < float(previous_hp) - 1e-3
    stats["enemy_tower_hp"] = current_hp
    for pending in stats["pending_hogs"]:
        if pending["connected"]:
            continue
        age = int(tick) - int(pending["tick"])
        if age < 0 or age > HOG_CONNECT_TICKS:
            continue
        if hog_in_range or damaged:
            pending["connected"] = True
            stats["hog_connects"] += 1
    return stats


def fresh_hog_stats() -> dict[str, Any]:
    return {"plays": 0, "hog_plays": 0, "hog_connects": 0, "leak_seconds": 0.0,
            "card_counts": {}, "pending_hogs": [], "enemy_tower_hp": None}


def hog_summary(stats: Mapping[str, Any]) -> dict[str, Any]:
    plays = int(stats.get("hog_plays") or 0)
    connects = int(stats.get("hog_connects") or 0)
    return {"hog_plays": plays, "plays": int(stats.get("plays") or 0),
            "hog_connects": connects,
            "connect_rate": (connects / plays) if plays else None,
            "leak_seconds": float(stats.get("leak_seconds") or 0.0),
            "card_counts": dict(stats.get("card_counts") or {})}


def clone_encoder_state(state: Any) -> Any:
    """Copy a causal fold without cloning the shared dataset, vocab, or cost table."""
    cloned = object.__new__(type(state))
    for key, value in state.__dict__.items():
        if key in _SHARED_STATE_FIELDS:
            setattr(cloned, key, value)
        elif isinstance(value, torch.Tensor):
            setattr(cloned, key, value.detach().clone())
        else:
            setattr(cloned, key, copy.deepcopy(value))
    return cloned


def capture_encoder_states(battle_id: str, policies: Sequence[Any]) -> list[tuple]:
    from .banana_fast_features import _CACHE_LOCK, _STREAM_CACHE
    saved = []
    with _CACHE_LOCK:
        for policy_index, policy in enumerate(policies):
            dataset = getattr(policy, "_dataset", None)
            vocab = getattr(policy, "vocab", None)
            costs = getattr(policy, "costs", None)
            if dataset is None or vocab is None or costs is None:
                continue
            for swap in (False, True):
                key = (battle_id, swap, id(dataset), id(vocab), id(costs))
                state = _STREAM_CACHE.get(key)
                if state is not None:
                    saved.append((policy_index, key, clone_encoder_state(state)))
    return saved


def restore_encoder_states(saved: Sequence[tuple]) -> None:
    from .banana_fast_features import _CACHE_LOCK, _STREAM_CACHE
    with _CACHE_LOCK:
        for _policy_index, key, state in saved:
            _STREAM_CACHE[tuple(key)] = clone_encoder_state(state)


def capture_board_caches(episode: Any, policies: Sequence[Any]) -> list[tuple]:
    saved = []
    battle_id = getattr(episode, "battle_id", None)
    for policy_index, policy in enumerate(policies):
        cache = getattr(policy, "_board_cache", None)
        if not isinstance(cache, dict):
            continue
        for key in (("episode", id(episode)), ("battle", battle_id)):
            found = cache.get(key)
            if found is not None and hasattr(found, "clone"):
                saved.append((policy_index, key, found.clone()))
    return saved


def restore_board_caches(episode: Any, policies: Sequence[Any], saved: Sequence[tuple]) -> None:
    battle_id = getattr(episode, "battle_id", None)
    for policy_index, key, tensors in saved:
        policy = policies[policy_index]
        cache = getattr(policy, "_board_cache", None)
        if cache is None:
            continue
        kind = key[0]
        if kind == "episode":
            cache[("episode", id(episode))] = tensors.clone()
        elif kind == "battle" and battle_id is not None:
            cache[("battle", battle_id)] = tensors.clone()


def capture_match_python(episode: Any, policies: Sequence[Any], view: Mapping[str, Any]) -> dict[str, Any]:
    """Snapshot the episode, encoder folds, board caches, and plain game view."""
    return {
        "battle_id": episode.battle_id,
        "decks": tuple(tuple(deck) for deck in episode.decks),
        "events": copy.deepcopy(list(episode.events)),
        "encoders": capture_encoder_states(episode.battle_id, policies),
        "boards": capture_board_caches(episode, policies),
        "view": copy.deepcopy(dict(view)),
    }


def restore_match_python(episode: Any, policies: Sequence[Any], snapshot: Mapping[str, Any]) -> dict[str, Any]:
    """Restore a snapshot into ``episode``. Returns the plain game view.

    Encoder state is copied again so a later continuation cannot rewrite the
    bank's stored fold. Board caches are rebound to this episode object.
    """
    if episode.battle_id != snapshot["battle_id"]:
        raise RuntimeError("python snapshot battle id does not match the episode")
    episode.events[:] = copy.deepcopy(list(snapshot["events"]))
    restore_encoder_states(snapshot["encoders"])
    restore_board_caches(episode, policies, snapshot["boards"])
    return copy.deepcopy(dict(snapshot["view"]))


def eta_seconds(now: float, started: float, deadline: float, status: str) -> int:
    """Wall-clock seconds left in the budget. Finished runs report zero."""
    if status in {"completed", "failed", "stopped"}:
        return 0
    return max(0, int(round(float(deadline) - float(now))))


def budget_percent(now: float, started: float, deadline: float) -> float:
    span = float(deadline) - float(started)
    if span <= 0:
        return 100.0
    return min(100.0, max(0.0, (float(now) - float(started)) / span * 100.0))


def format_eta(seconds: int) -> str:
    seconds = max(0, int(seconds))
    return f"{seconds // 3600:02d}:{seconds // 60 % 60:02d}:{seconds % 60:02d}"
