"""Models, preset decks and cards available to the websites.

Cards are the recovered cards the replay-trained policies know (the explicit
native-name to policy-slug alias table) joined with their original card art.
Preset decks are the six training decks of the fast league simulator; any
other eight-card combination still runs in the reference engine.
"""
from __future__ import annotations

import json
from functools import lru_cache

from . import paths

PRESET_LABELS = {"hog-mirror": "Hog 2.6"}


@lru_cache(maxsize=1)
def cards() -> dict[str, dict]:
    """Policy slug -> card description, sorted by elixir then name."""
    aliases = json.loads((paths.ENGINE_DATA / "policy_card_aliases.json").read_text())["cards"]
    catalog = {row["sc_name"]: row for row in json.loads((paths.ENGINE_DATA / "card_catalog.json").read_text())}
    preset_cards = {card for deck in _raw_presets() for card in deck["cards"]}
    result = {}
    for alias in aliases:
        row = catalog.get(alias["internal_id"], {})
        art = (row.get("art") or {}).get("path")
        if not art or not (paths.ENGINE / "web" / art).is_file():
            continue
        result[alias["policy_id"]] = {
            "id": alias["policy_id"],
            "native": alias["internal_id"],
            "name": alias["name"],
            "elixir": alias.get("elixir", row.get("elixir")),
            "rarity": row.get("rarity"),
            "art": "/" + art,
            "in_preset": alias["policy_id"] in preset_cards,
        }
    return dict(sorted(result.items(), key=lambda item: (item[1]["elixir"] or 0, item[1]["name"])))


@lru_cache(maxsize=1)
def native_to_card() -> dict[str, dict]:
    """Native row name (and compact lowercase form) -> card, for live entities."""
    table = {}
    for card in cards().values():
        table[card["native"]] = card
        table["".join(ch.lower() for ch in card["native"] if ch.isalnum())] = card
    return table


def _raw_presets() -> list[dict]:
    return json.loads(paths.PRESET_DECKS.read_text())["decks"]


@lru_cache(maxsize=1)
def presets() -> list[dict]:
    known = cards()
    result = []
    for deck in _raw_presets():
        missing = [card for card in deck["cards"] if card not in known]
        if missing:
            raise ValueError(f"preset {deck['id']} uses cards without art or policy alias: {missing}")
        natives = [known[card]["native"] for card in deck["cards"]]
        if natives != deck["native_cards"]:
            raise ValueError(f"preset {deck['id']} native names disagree with the alias table")
        result.append({
            "id": deck["id"],
            "name": PRESET_LABELS.get(deck["id"], deck["name"]),
            "archetype": deck.get("archetype"),
            "cards": list(deck["cards"]),
            "average_elixir": round(sum(known[c]["elixir"] for c in deck["cards"]) / 8, 1),
        })
    return result


def preset(deck_id: str) -> dict | None:
    return next((deck for deck in presets() if deck["id"] == deck_id), None)


def validate_deck(deck: list[str]) -> list[str]:
    """Return eight unique known card slugs or raise ValueError."""
    if not isinstance(deck, list) or len(deck) != 8:
        raise ValueError("a deck needs exactly 8 cards")
    if any(not isinstance(card, str) for card in deck):
        raise ValueError("every card must be a card slug string")
    if len(set(deck)) != 8:
        raise ValueError("a deck cannot repeat a card")
    unknown = [card for card in deck if card not in cards()]
    if unknown:
        raise ValueError(f"unknown cards: {', '.join(map(str, unknown))}")
    return list(deck)


def registry() -> dict:
    return json.loads(paths.MODEL_REGISTRY.read_text())


def engine_models() -> list[dict]:
    """Registry models with checkpoint availability resolved."""
    result = []
    for model in registry()["engine_models"]:
        model = dict(model)
        checkpoint = model.get("checkpoint")
        model["available"] = checkpoint is None or (paths.REPO / checkpoint).is_file()
        trained = model.get("trained_deck")
        model["trained_deck_name"] = (preset(trained) or {}).get("name") if trained else None
        result.append(model)
    return result


def engine_model(model_id: str) -> dict | None:
    return next((m for m in engine_models() if m["id"] == model_id), None)
