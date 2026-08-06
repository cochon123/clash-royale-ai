from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path
from urllib.parse import parse_qs, urlparse

from lxml import html as lxml_html

from .models import ParsedReplay, ReplayEvent

TICKS_PER_SECOND = 20.0
ABILITY_SLUG = re.compile(r"/ability-[^/]*?/?ability-([a-z0-9-]+)\.png", re.I)
SAFE_ID = re.compile(r"[^A-Za-z0-9_-]+")


class ReplayParseError(ValueError):
    pass


def _class_xpath(name: str) -> str:
    return (
        "contains(concat(' ', normalize-space(@class), ' '), "
        f"' {name} ')"
    )


def _side(value: str | None) -> str | None:
    mapping = {"t": "team", "blue": "team", "o": "opponent", "red": "opponent"}
    return mapping.get((value or "").lower())


def _int_or_none(value: str | None) -> int | None:
    if value in (None, "", "None", "null"):
        return None
    try:
        return int(float(value))
    except (TypeError, ValueError):
        return None


def _extract_payload(path: Path) -> tuple[str, str | None]:
    text = path.read_text(encoding="utf-8", errors="replace")
    try:
        value = json.loads(text)
    except json.JSONDecodeError as exc:
        if "Just a moment..." in text or "challenge-platform" in text:
            raise ReplayParseError("cloudflare_challenge") from exc
        if "<html" in text.lower() and "data-tag=" in text:
            return text, None
        raise ReplayParseError(f"invalid_json:{exc.msg}") from exc

    request_url = None
    if isinstance(value, dict):
        request_url = value.get("request_url") or value.get("url")
        payload = value.get("payload", value)
        if isinstance(payload, dict):
            page = payload.get("html")
        else:
            page = None
        if page is None:
            page = value.get("html")
    else:
        page = None

    if not isinstance(page, str) or not page.strip():
        raise ReplayParseError("missing_html")
    if "Just a moment..." in page or "challenge-platform" in page:
        raise ReplayParseError("cloudflare_challenge")
    return page, request_url


def _query_from_permalink(root) -> dict[str, list[str]]:
    links = root.xpath("//a[contains(@href, '/replay?')]/@href")
    for href in links:
        query = parse_qs(urlparse(href).query)
        if query.get("tag"):
            return query
    return {}


def _battle_id(root, query: dict[str, list[str]], path: Path) -> str:
    tags = root.xpath("//*[@data-tag]/@data-tag")
    raw = tags[0] if tags else (query.get("tag") or [path.stem])[0]
    value = SAFE_ID.sub("", raw)
    if not value:
        raise ReplayParseError("missing_battle_id")
    return value


def _extract_decks(root) -> dict[str, list[str]]:
    decks = {"team": [], "opponent": []}
    stats = root.xpath(f"//*[{_class_xpath('stats')}]")
    if not stats:
        return decks
    for image in stats[0].xpath(
        f".//*[{_class_xpath('replay_stats')}]//img[@data-card and @data-s]"
    ):
        side = _side(image.get("data-s"))
        card = image.get("data-card")
        if side and card and card not in decks[side]:
            decks[side].append(card)
    return decks


def _ability_timeline(root) -> list[dict]:
    rows = []
    for image in root.xpath(
        f"//img[{_class_xpath('replay_card')} and @data-t and @data-ability='1']"
    ):
        ticks = _int_or_none(image.get("data-t"))
        side = _side(image.get("data-s"))
        if ticks is None or side is None:
            continue
        src = image.get("src") or ""
        match = ABILITY_SLUG.search(src)
        rows.append(
            {
                "ticks": ticks,
                "side": side,
                "ability_card": match.group(1) if match else None,
                "src": src or None,
            }
        )
    return rows


def _closest_ability(
    abilities: list[dict], ticks: int, side: str, used: set[int]
) -> dict | None:
    candidates = [
        (abs(row["ticks"] - ticks), index, row)
        for index, row in enumerate(abilities)
        if index not in used and row["side"] == side and abs(row["ticks"] - ticks) <= 2
    ]
    if not candidates:
        return None
    _, index, row = min(candidates, key=lambda value: value[0])
    used.add(index)
    return row


def parse_replay(path: str | Path) -> ParsedReplay:
    source = Path(path)
    page, request_url = _extract_payload(source)
    try:
        root = lxml_html.fromstring(page)
    except (TypeError, ValueError) as exc:
        raise ReplayParseError("invalid_html") from exc

    query = _query_from_permalink(root)
    battle_id = _battle_id(root, query, source)
    timeline = _ability_timeline(root)
    used_abilities: set[int] = set()
    events: list[ReplayEvent] = []

    markers = root.xpath("//*[@data-x and @data-y and @data-c and @data-t and @data-s]")
    for marker in markers:
        ticks = _int_or_none(marker.get("data-t"))
        side = _side(marker.get("data-s"))
        card = marker.get("data-c")
        if ticks is None or side is None or not card:
            continue
        x = _int_or_none(marker.get("data-x"))
        y = _int_or_none(marker.get("data-y"))
        is_ability = card == "_invalid" and x is None and y is None
        if is_ability:
            ability = _closest_ability(timeline, ticks, side, used_abilities)
            events.append(
                ReplayEvent(
                    ticks=ticks,
                    seconds=round(ticks / TICKS_PER_SECOND, 3),
                    side=side,
                    event_type="ability_activation",
                    ability_card=ability["ability_card"] if ability else None,
                    attribution="direct_icon"
                    if ability and ability["ability_card"]
                    else None,
                    ability_icon=ability["src"] if ability else None,
                )
            )
        elif x is not None and y is not None and card != "_invalid":
            events.append(
                ReplayEvent(
                    ticks=ticks,
                    seconds=round(ticks / TICKS_PER_SECOND, 3),
                    side=side,
                    event_type="card_play",
                    card=card,
                    x=x,
                    y=y,
                )
            )

    # Some payloads expose an ability on the timeline but omit its map marker.
    for index, ability in enumerate(timeline):
        if index in used_abilities:
            continue
        events.append(
            ReplayEvent(
                ticks=ability["ticks"],
                seconds=round(ability["ticks"] / TICKS_PER_SECOND, 3),
                side=ability["side"],
                event_type="ability_activation",
                ability_card=ability["ability_card"],
                attribution="direct_icon" if ability["ability_card"] else None,
                ability_icon=ability["src"],
            )
        )

    events.sort(key=lambda event: (event.ticks, event.side, event.event_type))
    participant_tags = {
        "team": (query.get("team_tags") or [""])[0].split(","),
        "opponent": (query.get("opponent_tags") or [""])[0].split(","),
    }
    participant_tags = {
        side: [tag.lstrip("#") for tag in tags if tag]
        for side, tags in participant_tags.items()
    }
    crowns = {
        "team": _int_or_none((query.get("team_crowns") or [None])[0]),
        "opponent": _int_or_none((query.get("opponent_crowns") or [None])[0]),
    }
    digest = hashlib.sha256(page.encode("utf-8")).hexdigest()
    warnings = []
    if not timeline and any(event.event_type == "ability_activation" for event in events):
        warnings.append("ability_marker_without_timeline")
    if len(_extract_decks(root)["team"]) != 8 or len(_extract_decks(root)["opponent"]) != 8:
        warnings.append("incomplete_deck")

    return ParsedReplay(
        battle_id=battle_id,
        source_file=str(source),
        request_url=request_url,
        participant_tags=participant_tags,
        crowns=crowns,
        decks=_extract_decks(root),
        events=events,
        content_hash=digest,
        warnings=warnings,
    )
