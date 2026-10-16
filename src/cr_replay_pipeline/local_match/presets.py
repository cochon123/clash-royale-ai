"""Famous deck lists using the same card keys as the policy corpus."""

from __future__ import annotations

PRESET_DECKS: dict[str, dict[str, object]] = {
    "hog_cycle_2.6": {
        "name": "2.6 Hog Cycle",
        "cards": [
            "hog-rider",
            "cannon",
            "fireball",
            "ice-golem",
            "ice-spirit",
            "musketeer",
            "skeletons",
            "the-log",
        ],
    },
    "hog_cycle_2.9": {
        "name": "2.9 Hog Cycle",
        "cards": [
            "hog-rider",
            "tesla",
            "fireball",
            "knight",
            "ice-spirit",
            "musketeer",
            "skeletons",
            "the-log",
        ],
    },
    "logbait": {
        "name": "Classic Log Bait",
        "cards": [
            "goblin-barrel",
            "princess",
            "goblin-gang",
            "ice-spirit",
            "inferno-tower",
            "knight",
            "rocket",
            "the-log",
        ],
    },
    "xbow_3.0": {
        "name": "3.0 X-Bow Cycle",
        "cards": [
            "archers",
            "fireball",
            "ice-spirit",
            "knight",
            "skeletons",
            "tesla",
            "the-log",
            "x-bow",
        ],
    },
    "pekka_bridgespam": {
        "name": "P.E.K.K.A Bridge Spam",
        "cards": [
            "pekka",
            "battle-ram",
            "bandit",
            "electro-wizard",
            "minions",
            "poison",
            "royal-ghost",
            "zap",
        ],
    },
    "golem_beatdown": {
        "name": "Golem Beatdown",
        "cards": [
            "golem",
            "baby-dragon",
            "mega-minion",
            "night-witch",
            "lumberjack",
            "tornado",
            "lightning",
            "barbarian-barrel",
        ],
    },
    "splashyard": {
        "name": "Splashyard",
        "cards": [
            "graveyard",
            "knight",
            "baby-dragon",
            "ice-wizard",
            "tombstone",
            "poison",
            "tornado",
            "barbarian-barrel",
        ],
    },
    "lavaloon": {
        "name": "LavaLoon",
        "cards": [
            "lava-hound",
            "balloon",
            "mega-minion",
            "skeleton-dragons",
            "tombstone",
            "arrows",
            "fireball",
            "minions",
        ],
    },
    "hog_earthquake": {
        "name": "Hog Earthquake",
        "cards": [
            "hog-rider",
            "earthquake",
            "tesla",
            "ice-spirit",
            "skeletons",
            "firecracker",
            "knight",
            "the-log",
        ],
    },
    "royal_giant": {
        "name": "Royal Giant Fisherman",
        "cards": [
            "royal-giant",
            "fisherman",
            "hunter",
            "mother-witch",
            "electro-spirit",
            "lightning",
            "barbarian-barrel",
            "bats",
        ],
    },
    "miner_poison": {
        "name": "Miner Poison",
        "cards": [
            "miner",
            "poison",
            "electro-wizard",
            "inferno-tower",
            "ice-golem",
            "skeletons",
            "the-log",
            "bats",
        ],
    },
}


def preset_payload() -> list[dict]:
    rows = []
    for preset_id, row in PRESET_DECKS.items():
        rows.append(
            {
                "id": preset_id,
                "name": row["name"],
                "cards": list(row["cards"]),
            }
        )
    return rows


def cards_for_preset(preset_id: str) -> list[str]:
    row = PRESET_DECKS.get(preset_id)
    if row is None:
        known = ", ".join(sorted(PRESET_DECKS))
        raise ValueError(f"unknown preset {preset_id!r}; known: {known}")
    return list(row["cards"])
