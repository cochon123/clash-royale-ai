"""Generate legal-but-random Clash Royale battles for realism scoring."""

from __future__ import annotations

import random
from dataclasses import dataclass
from typing import Any, Literal

from .winner_dataset import (
    ABILITY_ELIXIR_COST,
    DEFAULT_ELIXIR_COST,
    SPELL_CARDS,
    BattleExample,
    _elixir_gain_rate,
)

NegativeTier = Literal["easy", "medium", "hard"]
TIERS: tuple[NegativeTier, ...] = ("easy", "medium", "hard")

# Approximate RoyaleAPI placement bounds observed in the corpus.
X_MIN, X_MAX = 3000, 15000
Y_MIN, Y_MAX = 500, 31500
Y_MID = 16000


@dataclass(frozen=True)
class TimingPrior:
    """Empirical inter-play gap distribution estimated from real battles."""

    gaps: tuple[float, ...]

    @classmethod
    def from_battles(cls, battles: list[BattleExample], max_gaps: int = 200_000) -> TimingPrior:
        gaps: list[float] = []
        for battle in battles:
            times = [float(event["seconds"]) for event in battle.events]
            for previous, current in zip(times, times[1:]):
                delta = current - previous
                if 0.05 <= delta <= 30.0:
                    gaps.append(delta)
                if len(gaps) >= max_gaps:
                    return cls(tuple(gaps))
        if not gaps:
            gaps = [1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0]
        return cls(tuple(gaps))

    def sample(self, rng: random.Random) -> float:
        return float(rng.choice(self.gaps))


def _card_cost(card: str, event_type: str, costs: dict[str, int]) -> float:
    if event_type == "ability_activation":
        return float(ABILITY_ELIXIR_COST)
    return float(costs.get(card, DEFAULT_ELIXIR_COST))


def _random_position(side: str, rng: random.Random) -> tuple[int, int]:
    x = rng.randint(X_MIN, X_MAX)
    if side == "team":
        y = rng.randint(Y_MIN, Y_MID - 500)
    else:
        y = rng.randint(Y_MID + 500, Y_MAX)
    return x, y


def _legal_position_for_side(side: str, x: int, y: int, rng: random.Random) -> tuple[int, int]:
    """Keep x, remap y into the legal half for the side."""
    x = min(X_MAX, max(X_MIN, int(x)))
    if side == "team":
        if y >= Y_MID:
            y = rng.randint(Y_MIN, Y_MID - 500)
    else:
        if y <= Y_MID:
            y = rng.randint(Y_MID + 500, Y_MAX)
    return x, y


class _CycleHand:
    """Approximate 4-card hand + 4-card queue cycle."""

    def __init__(self, deck: tuple[str, ...], rng: random.Random):
        cards = list(deck)
        rng.shuffle(cards)
        self.hand = cards[:4]
        self.queue = cards[4:]

    def affordable(self, costs: dict[str, int], elixir: float) -> list[str]:
        return [
            card
            for card in self.hand
            if float(costs.get(card, DEFAULT_ELIXIR_COST)) <= elixir + 1e-9
        ]

    def play(self, card: str) -> None:
        index = self.hand.index(card)
        self.hand.pop(index)
        if self.queue:
            self.hand.append(self.queue.pop(0))
        self.queue.append(card)


def _estimate_target_duration(battle: BattleExample, rng: random.Random) -> float:
    base = float(battle.events[-1]["seconds"]) if battle.events else 180.0
    return float(min(330.0, max(90.0, base * rng.uniform(0.85, 1.15))))


def _estimate_target_plays(battle: BattleExample, rng: random.Random) -> int:
    plays = sum(1 for event in battle.events if event["event_type"] == "card_play")
    return int(max(12, round(plays * rng.uniform(0.8, 1.2))))


def generate_easy_negative(
    battle: BattleExample,
    costs: dict[str, int],
    rng: random.Random,
    timing: TimingPrior,
) -> BattleExample:
    """Random affordable cards from each deck; no cycle discipline."""
    duration = _estimate_target_duration(battle, rng)
    target_plays = _estimate_target_plays(battle, rng)
    team_elixir = 5.0
    opp_elixir = 5.0
    seconds = 0.0
    events: list[dict[str, Any]] = []
    decks = {"team": battle.team_deck, "opponent": battle.opponent_deck}

    while len(events) < target_plays and seconds < duration:
        gap = timing.sample(rng) * rng.uniform(0.6, 1.8)
        seconds = min(duration, seconds + max(0.2, gap))
        gain = gap / _elixir_gain_rate(seconds)
        team_elixir = min(10.0, team_elixir + gain)
        opp_elixir = min(10.0, opp_elixir + gain)

        side = "team" if rng.random() < 0.5 else "opponent"
        elixir = team_elixir if side == "team" else opp_elixir
        candidates = [
            card
            for card in decks[side]
            if float(costs.get(card, DEFAULT_ELIXIR_COST)) <= elixir + 1e-9
        ]
        if not candidates:
            continue
        card = rng.choice(candidates)
        cost = float(costs.get(card, DEFAULT_ELIXIR_COST))
        x, y = _random_position(side, rng)
        if side == "team":
            team_elixir = max(0.0, team_elixir - cost)
        else:
            opp_elixir = max(0.0, opp_elixir - cost)
        events.append(
            {
                "seconds": round(seconds, 2),
                "side": side,
                "event_type": "card_play",
                "card": card,
                "x": x,
                "y": y,
            }
        )

    if len(events) < 8:
        # Extremely short synthetic — pad with cheap legal plays near the end.
        for index in range(8 - len(events)):
            side = "team" if index % 2 == 0 else "opponent"
            card = min(
                decks[side],
                key=lambda name: costs.get(name, DEFAULT_ELIXIR_COST),
            )
            x, y = _random_position(side, rng)
            events.append(
                {
                    "seconds": round(min(duration, 20.0 + index * 3.0), 2),
                    "side": side,
                    "event_type": "card_play",
                    "card": card,
                    "x": x,
                    "y": y,
                }
            )
        events.sort(key=lambda item: item["seconds"])

    return BattleExample(
        battle_id=f"{battle.battle_id}::easy::{rng.randrange(1 << 14)}",
        team_deck=battle.team_deck,
        opponent_deck=battle.opponent_deck,
        team_wins=battle.team_wins,
        events=tuple(events),
    )


def generate_medium_negative(
    battle: BattleExample,
    costs: dict[str, int],
    rng: random.Random,
    timing: TimingPrior,
) -> BattleExample:
    """Cycle-aware random plays with empirical gaps and mild side alternation."""
    duration = _estimate_target_duration(battle, rng)
    target_plays = _estimate_target_plays(battle, rng)
    team_elixir = 5.0
    opp_elixir = 5.0
    seconds = 0.0
    events: list[dict[str, Any]] = []
    hands = {
        "team": _CycleHand(battle.team_deck, rng),
        "opponent": _CycleHand(battle.opponent_deck, rng),
    }
    last_side: str | None = None

    while len(events) < target_plays and seconds < duration:
        gap = timing.sample(rng)
        # Humans rarely dump every 0.2s for long stretches; keep medium gaps grounded.
        gap = float(min(18.0, max(0.35, gap)))
        seconds = min(duration, seconds + gap)
        gain = gap / _elixir_gain_rate(seconds)
        team_elixir = min(10.0, team_elixir + gain)
        opp_elixir = min(10.0, opp_elixir + gain)

        # Soft preference to alternate, still random when one side cannot afford.
        if last_side is None:
            side_order = ["team", "opponent"]
            rng.shuffle(side_order)
        elif rng.random() < 0.65:
            side_order = ["opponent" if last_side == "team" else "team", last_side]
        else:
            side_order = [last_side, "opponent" if last_side == "team" else "team"]

        played = False
        for side in side_order:
            elixir = team_elixir if side == "team" else opp_elixir
            candidates = hands[side].affordable(costs, elixir)
            if not candidates:
                continue
            # Mild spell bias when elixir is high, otherwise uniform.
            if elixir >= 7 and rng.random() < 0.35:
                spells = [card for card in candidates if card in SPELL_CARDS]
                card = rng.choice(spells or candidates)
            else:
                card = rng.choice(candidates)
            cost = float(costs.get(card, DEFAULT_ELIXIR_COST))
            x, y = _random_position(side, rng)
            # Medium: prefer own backline a bit more often than pure random.
            if rng.random() < 0.45:
                if side == "team":
                    y = rng.randint(Y_MIN, 11000)
                else:
                    y = rng.randint(21000, Y_MAX)
            if side == "team":
                team_elixir = max(0.0, team_elixir - cost)
            else:
                opp_elixir = max(0.0, opp_elixir - cost)
            hands[side].play(card)
            events.append(
                {
                    "seconds": round(seconds, 2),
                    "side": side,
                    "event_type": "card_play",
                    "card": card,
                    "x": x,
                    "y": y,
                }
            )
            last_side = side
            played = True
            break
        if not played:
            continue

    if len(events) < 8:
        return generate_easy_negative(battle, costs, rng, timing)

    return BattleExample(
        battle_id=f"{battle.battle_id}::medium::{rng.randrange(1 << 14)}",
        team_deck=battle.team_deck,
        opponent_deck=battle.opponent_deck,
        team_wins=battle.team_wins,
        events=tuple(events),
    )


def generate_hard_negative(
    battle: BattleExample,
    costs: dict[str, int],
    rng: random.Random,
    timing: TimingPrior | None = None,
) -> BattleExample:
    """Perturb a real battle while preserving coarse legal structure.

    Modes:
    - position: keep cards/times/sides, scramble placements in the legal half
    - remap: keep times/sides/positions, permute which deck card is played
    - jitter: keep cards/sides, jitter times and scramble positions
    """
    del timing  # unused; kept for a uniform generator signature
    mode = rng.choice(["position", "remap", "jitter"])
    team_map = dict(zip(battle.team_deck, rng.sample(list(battle.team_deck), k=8)))
    opp_map = dict(
        zip(battle.opponent_deck, rng.sample(list(battle.opponent_deck), k=8))
    )

    events: list[dict[str, Any]] = []
    time_scale = rng.uniform(0.9, 1.1) if mode == "jitter" else 1.0
    for event in battle.events:
        item = dict(event)
        side = item["side"]
        if mode in {"position", "jitter"} and item["event_type"] == "card_play":
            x, y = _legal_position_for_side(side, int(item["x"]), int(item["y"]), rng)
            if mode == "position":
                x, y = _random_position(side, rng)
            else:
                x = min(X_MAX, max(X_MIN, int(x + rng.randint(-2500, 2500))))
                y = int(y + rng.randint(-3000, 3000))
                x, y = _legal_position_for_side(side, x, y, rng)
            item["x"] = x
            item["y"] = y
        if mode == "remap" and item["event_type"] == "card_play":
            mapping = team_map if side == "team" else opp_map
            item["card"] = mapping.get(item["card"], item["card"])
        if mode == "jitter":
            item["seconds"] = round(
                max(0.5, float(item["seconds"]) * time_scale + rng.uniform(-2.5, 2.5)),
                2,
            )
        events.append(item)

    events.sort(key=lambda item: (item["seconds"], item["side"], item["card"]))
    # Drop impossible elixir sequences created by aggressive remaps by filtering
    # plays that would require more elixir than available under a soft sim.
    filtered = _soft_elixir_filter(events, battle, costs)
    if len(filtered) < max(8, len(battle.events) // 3):
        filtered = events

    return BattleExample(
        battle_id=f"{battle.battle_id}::hard::{mode}::{rng.randrange(1 << 14)}",
        team_deck=battle.team_deck,
        opponent_deck=battle.opponent_deck,
        team_wins=battle.team_wins,
        events=tuple(filtered),
    )


def _soft_elixir_filter(
    events: list[dict[str, Any]],
    battle: BattleExample,
    costs: dict[str, int],
) -> list[dict[str, Any]]:
    team_elixir = 5.0
    opp_elixir = 5.0
    last = 0.0
    kept: list[dict[str, Any]] = []
    for event in events:
        seconds = float(event["seconds"])
        if seconds < last:
            continue
        gain = (seconds - last) / _elixir_gain_rate(seconds)
        team_elixir = min(10.0, team_elixir + gain)
        opp_elixir = min(10.0, opp_elixir + gain)
        cost = _card_cost(event["card"], event["event_type"], costs)
        if event["side"] == "team":
            if cost > team_elixir + 0.75:
                last = seconds
                continue
            team_elixir = max(0.0, team_elixir - cost)
        else:
            if cost > opp_elixir + 0.75:
                last = seconds
                continue
            opp_elixir = max(0.0, opp_elixir - cost)
        kept.append(event)
        last = seconds
    return kept


GENERATORS = {
    "easy": generate_easy_negative,
    "medium": generate_medium_negative,
    "hard": generate_hard_negative,
}


def generate_negatives_for_battles(
    battles: list[BattleExample],
    costs: dict[str, int],
    timing: TimingPrior,
    seed: int,
    tiers: tuple[NegativeTier, ...] = TIERS,
    per_tier: int = 1,
) -> list[tuple[BattleExample, NegativeTier]]:
    """Create synthetic negatives paired to real battles (no cross-split leakage)."""
    out: list[tuple[BattleExample, NegativeTier]] = []
    for battle_index, battle in enumerate(battles):
        for tier in tiers:
            for copy_index in range(per_tier):
                rng = random.Random(
                    seed
                    + 1_000_003 * battle_index
                    + 97 * TIERS.index(tier)
                    + copy_index
                )
                synthetic = GENERATORS[tier](battle, costs, rng, timing)
                out.append((synthetic, tier))
    return out


def realism_label_name(is_real: bool, tier: NegativeTier | None = None) -> str:
    if is_real:
        return "real"
    return tier or "synthetic"
