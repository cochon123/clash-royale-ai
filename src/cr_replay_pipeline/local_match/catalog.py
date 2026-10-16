"""Card roster used by the local match control plane.

Identities come from the repo's existing card table (key, Supercell id,
elixir, type). Flavor text is not copied into API responses.
"""

from __future__ import annotations

import json
import re
from dataclasses import dataclass
from pathlib import Path

NOT_IN_STARTING_HAND = frozenset({"mirror", "elixir-collector"})
# Spells are anywhere. These two non-spells may also be placed on either half.
ANYWHERE_KEYS = frozenset({"miner", "goblin-drill"})
_ALIAS_RE = re.compile(r"-+")


def repo_root() -> Path:
    return Path(__file__).resolve().parents[3]


def default_cards_path() -> Path:
    return repo_root() / "data" / "v9_card_stats" / "cards.json"


def alias_key(name: str) -> str:
    text = str(name).strip().lower().replace("_", "-").replace(" ", "-").replace(".", "")
    return _ALIAS_RE.sub("-", text).strip("-")


@dataclass(frozen=True)
class Card:
    key: str
    name: str
    sc_key: str
    elixir: int
    type: str
    rarity: str
    card_id: int

    @property
    def anywhere(self) -> bool:
        return self.type == "Spell" or self.key in ANYWHERE_KEYS

    @property
    def champion(self) -> bool:
        return self.rarity == "Champion"

    @property
    def not_in_starting_hand(self) -> bool:
        return self.key in NOT_IN_STARTING_HAND

    def public(self) -> dict:
        return {
            "key": self.key,
            "name": self.name,
            "sc_key": self.sc_key,
            "elixir": self.elixir,
            "type": self.type,
            "rarity": self.rarity,
            "id": self.card_id,
        }


class Catalog:
    def __init__(self, cards: list[Card]):
        self.cards = cards
        self.by_key = {card.key: card for card in cards}
        self.by_id = {card.card_id: card for card in cards}
        self.aliases: dict[str, Card] = {}
        for card in cards:
            self.aliases[card.key] = card
            self.aliases.setdefault(alias_key(card.name), card)

    def __len__(self) -> int:
        return len(self.cards)

    def resolve(self, name: str) -> Card:
        key = alias_key(name)
        card = self.by_key.get(key) or self.aliases.get(key)
        if card is None:
            raise ValueError(f"unknown card: {name}")
        return card

    def validate_deck(self, names: list[str]) -> list[Card]:
        if len(names) != 8:
            raise ValueError(f"deck must have 8 cards, got {len(names)}")
        cards = [self.resolve(name) for name in names]
        keys = [card.key for card in cards]
        if len(set(keys)) != 8:
            raise ValueError("deck cards must be unique")
        champions = [card.key for card in cards if card.champion]
        if len(champions) > 1:
            raise ValueError(f"at most one champion, got {champions}")
        return cards


def load_catalog(path: str | Path | None = None) -> Catalog:
    source = Path(path) if path else default_cards_path()
    raw = json.loads(source.read_text(encoding="utf-8"))
    if not isinstance(raw, list):
        raise ValueError(f"{source} is not a card list")
    chosen: dict[str, Card] = {}
    for row in raw:
        if not isinstance(row, dict):
            continue
        if row.get("is_evolved"):
            continue
        key = str(row.get("key") or "").strip()
        if not key or key in chosen:
            continue
        card_id = row.get("id")
        if not isinstance(card_id, int):
            continue
        chosen[key] = Card(
            key=key,
            name=str(row.get("name") or key),
            sc_key=str(row.get("sc_key") or ""),
            elixir=int(row.get("elixir") or 0),
            type=str(row.get("type") or "Troop"),
            rarity=str(row.get("rarity") or ""),
            card_id=card_id,
        )
    cards = sorted(chosen.values(), key=lambda card: card.name.lower())
    if len(cards) < 50:
        raise ValueError(f"card table {source} has only {len(cards)} playable cards")
    return Catalog(cards)
