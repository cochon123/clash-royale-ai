from __future__ import annotations

import json
from collections import Counter
from pathlib import Path

from .metadata import MetadataIndex, card_slug
from .parser import ReplayParseError, parse_replay

LEGACY_ROSTERS = {
    "december-2025": {"mini-pekka", "musketeer", "giant", "knight"},
}


def _slug(name: str) -> str:
    return card_slug(name)


def _metadata_heroes(record: dict, side: str) -> list[str]:
    players = record.get(side) or []
    if len(players) != 1:
        return []
    return [_slug(name) for name in players[0].get("active_heroes") or []]


def _label_abilities(replay, metadata: dict | None, roster: set[str] | None) -> list[str]:
    reasons = []
    candidates = {"team": [], "opponent": []}
    method = None
    if metadata:
        for side in candidates:
            candidates[side] = _metadata_heroes(metadata, side)
        method = "single_active_hero"
    elif roster:
        for side in candidates:
            candidates[side] = [
                card for card in replay.decks[side] if _slug(card) in roster
            ]
        method = "single_historical_candidate"

    for side, heroes in candidates.items():
        if len(heroes) > 1:
            reasons.append(f"{side}_has_multiple_hero_candidates")

    for event in replay.events:
        if event.event_type != "ability_activation" or event.ability_card:
            continue
        heroes = candidates[event.side]
        if len(heroes) == 1:
            event.ability_card = heroes[0]
            event.attribution = method
        else:
            reasons.append(f"unresolved_ability_{event.side}")
    return sorted(set(reasons))


def _quality_reasons(replay) -> list[str]:
    reasons = []
    plays = [event for event in replay.events if event.event_type == "card_play"]
    sides = {event.side for event in plays}
    if len(plays) < 10:
        reasons.append("too_few_card_plays")
    if sides != {"team", "opponent"}:
        reasons.append("missing_side_actions")
    for side, deck in replay.decks.items():
        if len(deck) != 8:
            reasons.append(f"{side}_deck_size_{len(deck)}")
    if any(
        event.event_type == "ability_activation" and not event.ability_card
        for event in replay.events
    ):
        reasons.append("unresolved_ability")
    if any(
        event.event_type == "card_play"
        and (
            event.x is None
            or event.y is None
            or not 0 <= event.x < 18000
            or not 0 <= event.y < 32000
        )
        for event in replay.events
    ):
        reasons.append("invalid_coordinates")
    return sorted(set(reasons))


def clean_directory(
    input_dir: str | Path,
    output_dir: str | Path,
    metadata_path: str | Path | None = None,
    legacy_roster: str | None = None,
    audit_only: bool = False,
    report_path: str | Path | None = None,
) -> dict:
    source = Path(input_dir)
    files = sorted(source.rglob("*.json"))
    metadata = MetadataIndex.from_jsonl(metadata_path) if metadata_path else None
    roster = LEGACY_ROSTERS.get(legacy_roster) if legacy_roster else None
    if legacy_roster and roster is None:
        raise ValueError(f"Unknown legacy roster: {legacy_roster}")

    stats = Counter(total_files=len(files))
    rejection_counts = Counter()
    attribution_counts = Counter()
    accepted_attribution_counts = Counter()
    accepted_event_counts = Counter()
    accepted = []
    quarantine = []
    seen_battles = set()
    seen_hashes = set()

    for path in files:
        try:
            replay = parse_replay(path)
        except (ReplayParseError, OSError) as exc:
            reason = str(exc)
            stats["parse_errors"] += 1
            rejection_counts[reason] += 1
            quarantine.append({"source_file": str(path), "reasons": [reason]})
            continue

        if replay.battle_id in seen_battles or replay.content_hash in seen_hashes:
            stats["duplicates"] += 1
            continue
        seen_battles.add(replay.battle_id)
        seen_hashes.add(replay.content_hash)

        matched = metadata.match(replay) if metadata else None
        record = matched.record if matched else None
        if matched and matched.reason:
            stats[matched.reason] += 1
        reasons = _label_abilities(replay, record, roster)
        reasons.extend(_quality_reasons(replay))
        reasons = sorted(set(reasons))
        for event in replay.events:
            if event.event_type == "ability_activation":
                attribution_counts[event.attribution or "unresolved"] += 1

        if reasons:
            stats["rejected_unique"] += 1
            rejection_counts.update(reasons)
            quarantine.append(
                {
                    "battle_id": replay.battle_id,
                    "source_file": replay.source_file,
                    "reasons": reasons,
                }
            )
            continue
        stats["accepted_unique"] += 1
        for event in replay.events:
            accepted_event_counts[event.event_type] += 1
            if event.event_type == "ability_activation":
                accepted_attribution_counts[event.attribution or "unresolved"] += 1
        accepted.append(replay)

    manifest = {
        **dict(stats),
        "unique_parsed": len(seen_battles),
        "rejection_counts": dict(rejection_counts.most_common()),
        "ability_attribution_counts": dict(attribution_counts.most_common()),
        "accepted_ability_attribution_counts": dict(
            accepted_attribution_counts.most_common()
        ),
        "accepted_event_counts": dict(accepted_event_counts.most_common()),
        "legacy_roster": legacy_roster,
        "metadata_path": str(metadata_path) if metadata_path else None,
    }

    if not audit_only:
        destination = Path(output_dir)
        destination.mkdir(parents=True, exist_ok=True)
        with (destination / "matches.jsonl").open("w", encoding="utf-8") as matches:
            for replay in accepted:
                row = replay.to_dict()
                row.pop("events")
                matches.write(json.dumps(row, ensure_ascii=False, separators=(",", ":")))
                matches.write("\n")
        with (destination / "events.jsonl").open("w", encoding="utf-8") as events:
            for replay in accepted:
                for sequence, event in enumerate(replay.events):
                    row = {
                        "battle_id": replay.battle_id,
                        "sequence": sequence,
                        **event.to_dict(),
                    }
                    events.write(json.dumps(row, ensure_ascii=False, separators=(",", ":")))
                    events.write("\n")
        (destination / "quarantine.jsonl").write_text(
            "".join(
                json.dumps(row, ensure_ascii=False, separators=(",", ":")) + "\n"
                for row in quarantine
            ),
            encoding="utf-8",
        )
        (destination / "manifest.json").write_text(
            json.dumps(manifest, indent=2, ensure_ascii=False), encoding="utf-8"
        )

    if report_path:
        target = Path(report_path)
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(
            json.dumps(manifest, indent=2, ensure_ascii=False), encoding="utf-8"
        )
    return manifest
