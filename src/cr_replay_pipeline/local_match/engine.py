"""Legality harness for a local match.

This is the control-path substitute used when Supercell's battle server is not
running. It enforces the public rules the policies already assume: 8-card
cycle, 4-card hand, elixir curve, and which half of the 32×18 tile grid a
card may occupy. It does not simulate troops, targeting, or crowns.

Actions are the same facts a reversed client would submit: card id, tile, and
match time.
"""

from __future__ import annotations

import threading
from dataclasses import dataclass, field

from .catalog import Card, Catalog

PROTOCOL = "cr-local-match/1"
ARENA_COLS = 32
ARENA_ROWS = 18
ARENA_X = 18000
ARENA_Y = 32000
HAND_SIZE = 4
ELIXIR_CAP = 10.0
STARTING_ELIXIR = 5.0
ABILITY_COST = 1

REAL_ENGINE_BLOCKER = (
    "REA (https://github.com/morluto/rea) inspects a caller-supplied APK with "
    "JADX and can pull that APK off a device you control. It does not download "
    "Clash Royale and it does not run Supercell's battle server. This process "
    "refuses Supercell and every other non-loopback host. Without a local "
    "client engine on 127.0.0.1, matches open directly in battle on this "
    "control harness: real card ids, legal tiles, elixir, and hand cycle. "
    "Troop combat and crowns are not resolved."
)


class IllegalAction(ValueError):
    pass


def elixir_rate(seconds: float) -> float:
    """Elixir per second. Matches the training clock: 2.8s / 1.4s / 0.9s."""
    if seconds >= 240:
        return 1.0 / 0.9
    if seconds >= 120:
        return 1.0 / 1.4
    return 1.0 / 2.8


def add_elixir(elixir: float, t0: float, t1: float, cap: float = ELIXIR_CAP) -> float:
    if t1 < t0 - 1e-9:
        raise IllegalAction("match clock went backwards")
    value = float(elixir)
    t = float(t0)
    while t < t1 - 1e-12 and value < cap - 1e-9:
        boundary = t1
        for mark in (120.0, 240.0):
            if t < mark < boundary:
                boundary = mark
        rate = elixir_rate(t)
        dt = boundary - t
        room = cap - value
        if room <= rate * dt + 1e-12:
            return cap
        value += rate * dt
        t = boundary
    return min(cap, value)


def row_bounds(side: str, card: Card) -> tuple[int, int]:
    if side not in ("red", "blue"):
        raise IllegalAction(f"side must be red or blue, got {side!r}")
    if card.anywhere:
        return 0, ARENA_ROWS - 1
    if card.type == "Building":
        return (0, 7) if side == "red" else (10, ARENA_ROWS - 1)
    return (0, 8) if side == "red" else (9, ARENA_ROWS - 1)


def tile_center(col: int, row: int) -> tuple[int, int]:
    x = int(round((col + 0.5) / ARENA_COLS * ARENA_X))
    y = int(round((row + 0.5) / ARENA_ROWS * ARENA_Y))
    return x, y


def tile_from_raw(x: float, y: float) -> tuple[int, int]:
    col = int(x / ARENA_X * ARENA_COLS)
    row = int(y / ARENA_Y * ARENA_ROWS)
    return max(0, min(ARENA_COLS - 1, col)), max(0, min(ARENA_ROWS - 1, row))


def clamp_tile(side: str, card: Card, col: int, row: int) -> tuple[int, int]:
    lo, hi = row_bounds(side, card)
    return (
        max(0, min(ARENA_COLS - 1, int(col))),
        max(lo, min(hi, int(row))),
    )


@dataclass
class SideState:
    deck: list[Card]
    queue: list[Card]
    elixir: float = STARTING_ELIXIR
    last_cost: int | None = None
    played: set[str] = field(default_factory=set)

    @property
    def hand(self) -> list[Card]:
        return self.queue[:HAND_SIZE]

    def cost_of(self, card: Card) -> int:
        if card.key != "mirror":
            return card.elixir
        if self.last_cost is None:
            raise IllegalAction("mirror needs a previous card")
        return min(10, self.last_cost + 1)

    def play(self, card: Card) -> None:
        try:
            index = self.queue.index(card)
        except ValueError as exc:
            raise IllegalAction(f"{card.key} is not in the cycle") from exc
        if index >= HAND_SIZE:
            raise IllegalAction(f"{card.key} is not in hand")
        cost = self.cost_of(card)
        have = round(self.elixir, 4)
        if have + 1e-9 < cost:
            raise IllegalAction(f"need {cost} elixir for {card.key}, have {have}")
        self.elixir = round(have - cost, 4)
        self.last_cost = cost
        self.queue.pop(index)
        self.queue.append(card)
        self.played.add(card.key)


class Harness:
    """In-process match. `dispatch` is the loopback protocol body."""

    def __init__(self, catalog: Catalog, seed: int = 0):
        import random

        self.catalog = catalog
        self.seed = int(seed)
        self._rng = random.Random(self.seed)
        self._lock = threading.Lock()
        self.started = False
        self.t = 0.0
        self.max_seconds = 180.0
        self.sides: dict[str, SideState] = {}
        self.events: list[dict] = []
        self.phase = "idle"

    def hello(self) -> dict:
        return {
            "ok": True,
            "protocol": PROTOCOL,
            "engine": "control-harness",
            "real_engine": False,
            "combat": False,
            "phase": self.phase,
            "blocker": REAL_ENGINE_BLOCKER,
        }

    def dispatch(self, payload: dict) -> dict:
        op = str(payload.get("op") or "")
        if op == "hello":
            return self.hello()
        if op == "start":
            return self.start(
                red=list(payload.get("red") or []),
                blue=list(payload.get("blue") or []),
                seed=payload.get("seed", self.seed),
                max_seconds=float(payload.get("max_seconds") or 180),
            )
        if op == "advance":
            self.advance_to(float(payload["seconds"]))
            return self.snapshot()
        if op == "act":
            event = self.act(
                side=str(payload["side"]),
                card_key=str(payload["card"]),
                tile_col=int(payload.get("tile_col", 0)),
                tile_row=int(payload.get("tile_row", 0)),
                event_type=str(payload.get("event_type") or "card_play"),
            )
            body = self.snapshot()
            body["event"] = event
            return body
        if op == "state":
            return self.snapshot()
        raise IllegalAction(f"unknown op {op!r}")

    def start(
        self,
        red: list[str],
        blue: list[str],
        seed: int | None = None,
        max_seconds: float = 180.0,
    ) -> dict:
        import random

        with self._lock:
            if seed is not None:
                self.seed = int(seed)
                self._rng = random.Random(self.seed)
            red_cards = self.catalog.validate_deck(red)
            blue_cards = self.catalog.validate_deck(blue)
            self.sides = {
                "red": self._deal(red_cards),
                "blue": self._deal(blue_cards),
            }
            self.events = []
            self.t = 0.0
            self.max_seconds = float(max_seconds)
            self.started = True
            self.phase = "battle"
            body = self._snapshot_unlocked()
            body["opened_in"] = "battle"
            body["menu_steps"] = 0
            return body

    def _deal(self, deck: list[Card]) -> SideState:
        front = [card for card in deck if not card.not_in_starting_hand]
        back = [card for card in deck if card.not_in_starting_hand]
        self._rng.shuffle(front)
        self._rng.shuffle(back)
        return SideState(deck=list(deck), queue=front + back)

    def advance_to(self, seconds: float) -> None:
        with self._lock:
            self._advance_unlocked(float(seconds))

    def _advance_unlocked(self, seconds: float) -> None:
        if not self.started:
            raise IllegalAction("match has not started")
        if seconds < self.t - 1e-9:
            raise IllegalAction("match clock went backwards")
        for side in self.sides.values():
            side.elixir = add_elixir(side.elixir, self.t, seconds)
        self.t = seconds
        if self.t >= self.max_seconds - 1e-9:
            self.phase = "regulation_ended"

    def act(
        self,
        side: str,
        card_key: str,
        tile_col: int,
        tile_row: int,
        event_type: str = "card_play",
    ) -> dict:
        with self._lock:
            if not self.started or self.phase != "battle":
                raise IllegalAction(f"cannot act while phase is {self.phase}")
            if side not in self.sides:
                raise IllegalAction(f"unknown side {side!r}")
            state = self.sides[side]
            card = self.catalog.resolve(card_key)
            if event_type == "ability_activation":
                return self._ability_unlocked(side, state, card)
            if event_type != "card_play":
                raise IllegalAction(f"unsupported event_type {event_type!r}")
            lo, hi = row_bounds(side, card)
            if not (0 <= int(tile_col) < ARENA_COLS and lo <= int(tile_row) <= hi):
                raise IllegalAction(
                    f"{card.key} cannot be placed at tile {tile_col},{tile_row} "
                    f"for {side} (rows {lo}-{hi})"
                )
            state.play(card)
            x, y = tile_center(int(tile_col), int(tile_row))
            event = {
                "seconds": round(self.t, 4),
                "side": side,
                "event_type": "card_play",
                "card": card.key,
                "card_id": card.card_id,
                "sc_key": card.sc_key,
                "tile_col": int(tile_col),
                "tile_row": int(tile_row),
                "x": x,
                "y": y,
                "elixir_after": round(state.elixir, 4),
            }
            self.events.append(event)
            return event

    def _ability_unlocked(self, side: str, state: SideState, card: Card) -> dict:
        if not card.champion:
            raise IllegalAction(f"{card.key} has no champion ability")
        if card.key not in state.played:
            raise IllegalAction(f"{card.key} must be in play before its ability")
        have = round(state.elixir, 4)
        if have + 1e-9 < ABILITY_COST:
            raise IllegalAction(f"need 1 elixir for the ability, have {have}")
        state.elixir = round(have - ABILITY_COST, 4)
        event = {
            "seconds": round(self.t, 4),
            "side": side,
            "event_type": "ability_activation",
            "card": card.key,
            "card_id": card.card_id,
            "sc_key": card.sc_key,
            "tile_col": None,
            "tile_row": None,
            "x": 9000,
            "y": 16000,
            "elixir_after": round(state.elixir, 4),
        }
        self.events.append(event)
        return event

    def seconds_until_playable(self, side: str) -> float:
        with self._lock:
            state = self.sides[side]
            costs = []
            for card in state.hand:
                try:
                    costs.append(state.cost_of(card))
                except IllegalAction:
                    continue
            if not costs:
                return 1.0
            cheapest = min(costs)
            if state.elixir + 1e-6 >= cheapest:
                return 0.25
            rate = elixir_rate(self.t)
            return max(0.25, (cheapest - state.elixir) / rate)

    def snapshot(self) -> dict:
        with self._lock:
            return self._snapshot_unlocked()

    def _snapshot_unlocked(self) -> dict:
        return {
            "ok": True,
            "protocol": PROTOCOL,
            "engine": "control-harness",
            "real_engine": False,
            "combat": False,
            "phase": self.phase,
            "seconds": round(self.t, 4),
            "max_seconds": self.max_seconds,
            "seed": self.seed,
            "work": len(self.events),
            "opened_in": "battle" if self.started else None,
            "menu_steps": 0,
            "blocker": REAL_ENGINE_BLOCKER,
            "result": (
                None
                if self.phase == "battle"
                else "regulation ended; crowns are not resolved without the native engine"
            ),
            "sides": {
                name: self._side_public(name, state)
                for name, state in self.sides.items()
            },
            "events": list(self.events),
        }

    def _side_public(self, name: str, state: SideState) -> dict:
        costs: dict[str, int | None] = {}
        bounds: dict[str, list[int]] = {}
        for card in state.hand:
            try:
                costs[card.key] = state.cost_of(card)
            except IllegalAction:
                costs[card.key] = None
            lo, hi = row_bounds(name, card)
            bounds[card.key] = [lo, hi]
        return {
            "deck": [card.key for card in state.deck],
            "deck_ids": [card.card_id for card in state.deck],
            "hand": [card.key for card in state.hand],
            "queue": [card.key for card in state.queue],
            "elixir": round(state.elixir, 4),
            "costs": costs,
            "card_elixir": {card.key: card.elixir for card in state.deck},
            "bounds": bounds,
        }
