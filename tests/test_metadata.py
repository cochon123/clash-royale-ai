from cr_replay_pipeline.metadata import active_heroes, battle_identity, card_slug


def test_active_hero_requires_asset_and_active_level():
    cards = [
        {
            "name": "Barbarian Barrel",
            "evolutionLevel": 2,
            "iconUrls": {"heroMedium": "hero.png"},
        },
        {
            "name": "Wizard",
            "iconUrls": {"heroMedium": "capable-but-inactive.png"},
        },
        {
            "name": "Knight",
            "evolutionLevel": 3,
            "iconUrls": {"heroMedium": "hero-and-evolution.png"},
        },
    ]
    assert active_heroes(cards) == ["barbarian-barrel", "knight"]


def test_card_slug_normalizes_punctuation():
    assert card_slug("P.E.K.K.A") == "p-e-k-k-a"


def test_battle_identity_is_orientation_independent():
    first = {
        "battleTime": "20260726T000000.000Z",
        "team": [{"tag": "#AAA", "crowns": 1}],
        "opponent": [{"tag": "#BBB", "crowns": 0}],
    }
    reversed_record = {
        "battleTime": "20260726T000000.000Z",
        "team": [{"tag": "#BBB", "crowns": 0}],
        "opponent": [{"tag": "#AAA", "crowns": 1}],
    }
    assert battle_identity(first) == battle_identity(reversed_record)
