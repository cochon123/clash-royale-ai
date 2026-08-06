from __future__ import annotations

from dataclasses import asdict, dataclass, field
from typing import Any


@dataclass
class ReplayEvent:
    ticks: int
    seconds: float
    side: str
    event_type: str
    card: str | None = None
    x: int | None = None
    y: int | None = None
    ability_card: str | None = None
    attribution: str | None = None
    ability_icon: str | None = None

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class ParsedReplay:
    battle_id: str
    source_file: str
    request_url: str | None
    participant_tags: dict[str, list[str]]
    crowns: dict[str, int | None]
    decks: dict[str, list[str]]
    events: list[ReplayEvent]
    content_hash: str
    warnings: list[str] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        data = asdict(self)
        data["events"] = [event.to_dict() for event in self.events]
        return data

