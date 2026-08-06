from __future__ import annotations

import json
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

NON_SLUG = re.compile(r"[^a-z0-9]+")


def card_slug(name: str | None) -> str:
    return NON_SLUG.sub("-", (name or "").strip().lower()).strip("-")


def active_heroes(cards: Iterable[dict]) -> list[str]:
    heroes = []
    for card in cards:
        icon_urls = card.get("iconUrls") or {}
        if int(card.get("evolutionLevel") or 0) >= 2 and icon_urls.get("heroMedium"):
            heroes.append(card_slug(card.get("name")))
    return [hero for hero in heroes if hero]


def normalize_tag(tag: str | None) -> str:
    return (tag or "").lstrip("#").upper()


def battle_identity(record: dict) -> tuple:
    team = record.get("team") or []
    opponent = record.get("opponent") or []
    players = tuple(
        sorted(
            (normalize_tag(player.get("tag")), player.get("crowns"))
            for player in [*team, *opponent]
        )
    )
    return (
        record.get("battleTime"),
        players,
    )


def normalize_battle(record: dict) -> dict:
    normalized = {
        "battle_time": record.get("battleTime"),
        "type": record.get("type"),
        "game_mode": (record.get("gameMode") or {}).get("name"),
        "team": [],
        "opponent": [],
    }
    for side in ("team", "opponent"):
        for player in record.get(side) or []:
            cards = player.get("cards") or []
            normalized[side].append(
                {
                    "tag": normalize_tag(player.get("tag")),
                    "name": player.get("name"),
                    "crowns": player.get("crowns"),
                    "cards": [
                        {
                            "name": card.get("name"),
                            "id": card.get("id"),
                            "evolution_level": card.get("evolutionLevel", 0),
                            "is_hero": card_slug(card.get("name"))
                            in active_heroes([card]),
                        }
                        for card in cards
                    ],
                    "active_heroes": active_heroes(cards),
                }
            )
    return normalized


@dataclass
class MetadataMatch:
    record: dict | None
    reason: str | None = None


class MetadataIndex:
    def __init__(self, records: Iterable[dict]):
        self.records = list(records)
        self.by_participants: dict[tuple, list[dict]] = {}
        for record in self.records:
            key = self._participant_key_from_metadata(record)
            self.by_participants.setdefault(key, []).append(record)

    @classmethod
    def from_jsonl(cls, path: str | Path) -> "MetadataIndex":
        records = []
        with Path(path).open(encoding="utf-8") as handle:
            for line in handle:
                if line.strip():
                    records.append(json.loads(line))
        return cls(records)

    @staticmethod
    def _participant_key_from_metadata(record: dict) -> tuple:
        return (
            tuple(sorted(player["tag"] for player in record.get("team", []))),
            tuple(sorted(player["tag"] for player in record.get("opponent", []))),
            tuple(player.get("crowns") for player in record.get("team", [])),
            tuple(player.get("crowns") for player in record.get("opponent", [])),
        )

    @staticmethod
    def _participant_key_from_replay(replay) -> tuple:
        return (
            tuple(sorted(replay.participant_tags["team"])),
            tuple(sorted(replay.participant_tags["opponent"])),
            (replay.crowns["team"],),
            (replay.crowns["opponent"],),
        )

    def match(self, replay) -> MetadataMatch:
        exact = self.by_participants.get(self._participant_key_from_replay(replay), [])
        if len(exact) == 1:
            return MetadataMatch(exact[0])
        if not exact:
            # RoyaleAPI can orient a battle opposite to the official battlelog.
            reverse_key = (
                tuple(sorted(replay.participant_tags["opponent"])),
                tuple(sorted(replay.participant_tags["team"])),
                (replay.crowns["opponent"],),
                (replay.crowns["team"],),
            )
            reverse = self.by_participants.get(reverse_key, [])
            if len(reverse) == 1:
                record = dict(reverse[0])
                record["team"], record["opponent"] = record["opponent"], record["team"]
                return MetadataMatch(record)
            if len(reverse) > 1:
                return MetadataMatch(None, "ambiguous_reversed_metadata")
            return MetadataMatch(None, "metadata_not_found")
        return MetadataMatch(None, "ambiguous_metadata")
