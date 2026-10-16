"""Local match control: decks, legality, loopback protocol, and the web UI."""

from __future__ import annotations

import json
import struct
import threading
import zipfile
from http.server import ThreadingHTTPServer
from pathlib import Path
from urllib.error import HTTPError
from urllib.request import Request, urlopen

import pytest

from cr_replay_pipeline.local_match.agents import list_agents
from cr_replay_pipeline.local_match.catalog import load_catalog, repo_root
from cr_replay_pipeline.local_match.engine import (
    Harness,
    IllegalAction,
    add_elixir,
    row_bounds,
)
from cr_replay_pipeline.local_match.native_replay import (
    CREATE_GAME_MAIN,
    ReplayEngineUnavailable,
    replay_towers,
)
from cr_replay_pipeline.local_match.prepare import (
    APKPURE_XAPK_URL,
    PACKAGE,
    assert_download_host,
    inspect_apk_zip,
    prepare,
    read_manifest_identity,
)
from cr_replay_pipeline.local_match.presets import cards_for_preset, preset_payload
from cr_replay_pipeline.local_match.server import MatchControl, make_handler, run_headless
from cr_replay_pipeline.local_match.wire import ProtocolClient, RemoteHostRefused, assert_loopback
from cr_replay_pipeline.policy_model import TILE_COLS, TILE_ROWS
from cr_replay_pipeline.local_match.engine import ARENA_COLS, ARENA_ROWS


def test_tile_grid_matches_the_policy():
    assert (ARENA_COLS, ARENA_ROWS) == (TILE_COLS, TILE_ROWS)


def test_catalog_exposes_the_full_roster_not_a_starter_subset():
    catalog = load_catalog()
    assert len(catalog) >= 100
    monk = catalog.resolve("Monk")
    assert monk.key == "monk"
    assert monk.card_id == 26000077
    assert monk.champion
    assert catalog.resolve("P.E.K.K.A").key == "pekka"
    assert catalog.resolve("The Log").key == "the-log"
    assert any(card.type == "Spell" for card in catalog.cards)
    assert any(card.type == "Building" for card in catalog.cards)
    # Descriptions from the source table are not part of the public card record.
    assert "description" not in monk.public()


def test_hog_2_9_is_the_23_elixir_cycle():
    catalog = load_catalog()
    cards = catalog.validate_deck(cards_for_preset("hog_cycle_2.9"))
    keys = [card.key for card in cards]
    assert keys == [
        "hog-rider",
        "tesla",
        "fireball",
        "knight",
        "ice-spirit",
        "musketeer",
        "skeletons",
        "the-log",
    ]
    assert "ice-golem" not in keys
    assert "cannon" not in keys
    assert keys != cards_for_preset("hog_cycle_2.6")
    assert sum(card.elixir for card in cards) == 23
    assert sum(card.elixir for card in cards) / 8 == pytest.approx(2.875)
    bait = catalog.validate_deck(cards_for_preset("logbait"))
    assert [card.key for card in bait] == cards_for_preset("logbait")
    assert sum(card.elixir for card in bait) == 26
    assert sum(card.elixir for card in bait) / 8 == pytest.approx(3.25)


def test_famous_presets_are_eight_unique_engine_cards():
    catalog = load_catalog()
    assert len(preset_payload()) >= 8
    for preset in preset_payload():
        cards = catalog.validate_deck(list(preset["cards"]))
        assert [card.key for card in cards] == list(preset["cards"])


def test_one_champion_is_legal_and_two_are_not():
    catalog = load_catalog()
    legal = [
        "monk",
        "knight",
        "archers",
        "giant",
        "minions",
        "arrows",
        "fireball",
        "musketeer",
    ]
    assert len(catalog.validate_deck(legal)) == 8
    with pytest.raises(ValueError, match="champion"):
        catalog.validate_deck(
            [
                "monk",
                "archer-queen",
                "knight",
                "archers",
                "giant",
                "minions",
                "arrows",
                "fireball",
            ]
        )


def test_elixir_curve_and_cap():
    assert add_elixir(5, 0, 2.8) == pytest.approx(6)
    assert add_elixir(5, 0, 30) == 10
    # One second of single elixir, then 2.8s of double elixir.
    gained = add_elixir(0, 119, 122.8)
    assert gained == pytest.approx((1 / 2.8) + (2.8 / 1.4))


def test_placement_hand_cycle_and_opening_rules():
    catalog = load_catalog()
    harness = Harness(catalog, seed=3)
    harness.start(cards_for_preset("hog_cycle_2.6"), cards_for_preset("logbait"))
    assert harness.phase == "battle"
    red = harness.sides["red"]
    assert len(red.hand) == 4
    cannon = catalog.resolve("cannon")
    others = [card for card in red.queue if card.key != "cannon"]
    red.queue = [cannon, *others]
    before = [card.key for card in red.queue]
    with pytest.raises(IllegalAction):
        harness.act("red", "cannon", 4, 8)
    harness.act("red", "cannon", 4, 4)
    after = [card.key for card in red.queue]
    assert after[-1] == "cannon"
    assert after[0] == before[1]
    assert red.elixir == pytest.approx(2)
    # Rounded elixir is what the agent sees. A value that displays as the cost
    # must be playable.
    red.elixir = 1.99996
    red.queue = [catalog.resolve("the-log")] + [
        card for card in red.queue if card.key != "the-log"
    ]
    harness.act("red", "the-log", 3, 12)
    assert row_bounds("red", catalog.resolve("fireball")) == (0, 17)
    assert row_bounds("red", catalog.resolve("miner")) == (0, 17)
    assert row_bounds("blue", catalog.resolve("tesla")) == (10, 17)
    miner_deck = [
        "miner",
        "knight",
        "archers",
        "giant",
        "minions",
        "arrows",
        "fireball",
        "musketeer",
    ]
    mirror_deck = [
        "mirror",
        "knight",
        "archers",
        "giant",
        "minions",
        "arrows",
        "fireball",
        "musketeer",
    ]
    opened = Harness(catalog, seed=1)
    opened.start(miner_deck, mirror_deck)
    opened.sides["red"].queue = [catalog.resolve("miner")] + [
        card for card in opened.sides["red"].queue if card.key != "miner"
    ]
    event = opened.act("red", "miner", 10, 15)
    assert event["card_id"] == catalog.resolve("miner").card_id
    assert "mirror" not in opened.sides["blue"].hand


def test_agents_include_a_loadable_random_baseline():
    rows = list_agents()
    random_agent = next(row for row in rows if row["id"] == "random")
    assert random_agent["loadable"] is True
    v9 = next(row for row in rows if row["id"] == "policy_v9")
    assert v9["loadable"] is False
    for row in rows:
        if row["kind"] != "policy_bc" or not row["path"]:
            continue
        weights = list((repo_root() / row["path"]).glob("*.pt"))
        assert row["loadable"] is bool(weights)


def test_remote_supercell_host_is_refused_before_any_socket(monkeypatch):
    def boom(*_args, **_kwargs):
        raise AssertionError("socket opened")

    monkeypatch.setattr("cr_replay_pipeline.local_match.wire.urlopen", boom)
    with pytest.raises(RemoteHostRefused):
        ProtocolClient("https://game.clashroyale.com/v1").hello()
    with pytest.raises(RemoteHostRefused):
        assert_loopback("http://1.1.1.1/v1")
    assert assert_loopback("http://127.0.0.1:9/v1").startswith("http://127.0.0.1")


def test_headless_match_opens_in_battle_and_both_ais_play():
    result = run_headless(
        red_agent="random",
        blue_agent="random",
        red=cards_for_preset("hog_cycle_2.6"),
        blue=cards_for_preset("logbait"),
        seed=7,
        max_seconds=4,
        speed=0,
    )
    assert result["opened_in"] == "battle"
    assert result["menu_steps"] == 0
    assert result["real_engine"] is False
    assert result["engine"] == "control-harness"
    assert result["work"] == len(result["events"])
    assert result["work"] >= 2
    assert {event["side"] for event in result["events"]} == {"red", "blue"}
    catalog = load_catalog()
    replay = Harness(catalog, seed=result["seed"])
    replay.start(result["red_deck"], result["blue_deck"])
    for event in result["events"]:
        assert event["card_id"] == catalog.resolve(event["card"]).card_id
        assert 0 <= event["elixir_after"] <= 10
        replay.advance_to(event["seconds"])
        replay.act(
            event["side"],
            event["card"],
            event["tile_col"],
            event["tile_row"],
            event["event_type"],
        )
    assert len(replay.events) == len(result["events"])
    assert len(result["red"]["hand"]) == 4
    assert len(result["blue"]["hand"]) == 4


def test_prepare_records_apk_identity_without_extracting_it(tmp_path: Path):
    apk = tmp_path / "toy.apk"
    with zipfile.ZipFile(apk, "w") as archive:
        archive.writestr("AndroidManifest.xml", b"fixture-manifest")
        archive.writestr("classes.dex", b"fixture-dex")
        archive.writestr("lib/arm64-v8a/libg.so", b"fixture-native")
    inspected = inspect_apk_zip(apk)
    assert inspected["native_libraries"] == ["libg.so"]
    assert inspected["has_android_manifest"] is True
    assert inspected["has_classes_dex"] is True
    assert inspected["accepted"] is False
    assert inspected["package"] is None
    report = prepare(apk=apk, out_dir=tmp_path / "var", run_rea=False)
    assert report["apk"]["sha256"] == inspected["sha256"]
    assert report["apk"]["accepted"] is False
    assert any("Supercell" in line or "supercell" in line.lower() for line in report["blockers"])
    saved = json.loads((tmp_path / "var" / "prepare-status.json").read_text())
    assert saved["apk"]["bytes"] == apk.stat().st_size
    # The fixture bytes are not copied into a second extracted tree.
    assert not (tmp_path / "var" / "lib").exists()


def _start_ui(tmp_path: Path):
    control = MatchControl(load_catalog(), repo_root())
    handler = make_handler(control, tmp_path / "missing-prepare.json")
    server = ThreadingHTTPServer(("127.0.0.1", 0), handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    return server, control


def _json(url: str, payload: dict | None = None, method: str = "GET") -> tuple[int, dict]:
    data = None if payload is None else json.dumps(payload).encode("utf-8")
    request = Request(url, data=data, method=method, headers={"Content-Type": "application/json"})
    try:
        with urlopen(request, timeout=10) as response:
            return response.status, json.loads(response.read().decode("utf-8"))
    except HTTPError as exc:
        return exc.code, json.loads(exc.read().decode("utf-8"))


def test_web_control_selects_ais_and_decks_then_launches(tmp_path: Path):
    server, control = _start_ui(tmp_path)
    base = f"http://127.0.0.1:{server.server_address[1]}"
    try:
        icon = urlopen(base + "/favicon.ico", timeout=5)
        assert icon.status == 204
        page = urlopen(base + "/", timeout=5).read().decode("utf-8")
        assert 'id="launch"' in page
        assert 'id="red-agent"' in page
        assert 'id="blue-agent"' in page
        assert 'id="red-preset"' in page
        status, catalog = _json(base + "/api/catalog")
        assert status == 200
        assert len(catalog["cards"]) >= 100
        assert any(row["id"] == "hog_cycle_2.6" for row in catalog["presets"])
        assert any(row["id"] == "hog_cycle_2.9" for row in catalog["presets"])
        assert "hog_cycle_2.9" in page
        assert 'id="speed"' in page
        status, agents = _json(base + "/api/agents")
        assert status == 200
        assert any(row["id"] == "random" and row["loadable"] for row in agents["agents"])
        status, rejected = _json(
            base + "/api/match",
            {
                "red_agent": "random",
                "blue_agent": "random",
                "red_cards": ["knight"],
                "blue_preset": "logbait",
            },
            method="POST",
        )
        assert status == 400
        assert "8" in rejected["error"]
        status, started = _json(
            base + "/api/match",
            {
                "red_agent": "random",
                "blue_agent": "random",
                "red_preset": "hog_cycle_2.6",
                "blue_cards": cards_for_preset("lavaloon"),
                "max_seconds": 3,
                "speed": 0,
                "seed": 4,
            },
            method="POST",
        )
        assert status == 200
        assert started["phase"] == "battle"
        deadline = threading.Event()

        def _wait() -> dict:
            view = {}
            for _ in range(100):
                view = control.view()
                if view.get("finished"):
                    return view
                deadline.wait(0.05)
            return view

        finished = _wait()
        assert finished["error"] is None
        assert finished["finished"] is True
        assert finished["opened_in"] == "battle"
        assert finished["menu_steps"] == 0
        assert finished["work"] >= 2
        assert {event["side"] for event in finished["events"]} == {"red", "blue"}
        assert finished["red_agent"] == "random"
        assert finished["blue_agent"] == "random"
        assert finished["real_engine"] is False
        assert "tower_hp" not in finished
        table = urlopen(base + "/api/match/table", timeout=5).read().decode("utf-8")
        assert "work" in table
        assert "eta_s" in table
    finally:
        control.stop()
        server.shutdown()
        server.server_close()


def _utf16(text: str) -> bytes:
    encoded = text.encode("utf-16-le")
    return struct.pack("<H", len(text)) + encoded + b"\x00\x00"


def _manifest_axml(package: str, version_name: str, version_code: int) -> bytes:
    strings = ["manifest", "versionCode", "versionName", "package", version_name, package]
    blob = b""
    offsets = []
    for item in strings:
        offsets.append(len(blob))
        blob += _utf16(item)
    pool_size = 28 + 4 * len(strings) + len(blob)
    pool = struct.pack(
        "<HHIIIIII",
        0x0001,
        28,
        pool_size,
        len(strings),
        0,
        0,
        28 + 4 * len(strings),
        0,
    )
    pool += b"".join(struct.pack("<I", offset) for offset in offsets) + blob

    def attribute(name_index: int, raw_index: int, value_type: int, data: int) -> bytes:
        return struct.pack("<III", 0xFFFFFFFF, name_index, raw_index) + struct.pack(
            "<HBBI", 8, 0, value_type, data
        )

    attrs = b"".join(
        [
            attribute(1, 0xFFFFFFFF, 0x10, version_code),
            attribute(2, 4, 0x03, 4),
            attribute(3, 5, 0x03, 5),
        ]
    )
    ext = struct.pack("<IIHHHHHH", 0xFFFFFFFF, 0, 20, 20, 3, 0, 0, 0)
    elem_size = 16 + len(ext) + len(attrs)
    elem = struct.pack("<HHIII", 0x0102, 16, elem_size, 1, 0xFFFFFFFF) + ext + attrs
    file_size = 8 + len(pool) + len(elem)
    return struct.pack("<HHI", 0x0003, 8, file_size) + pool + elem


def test_manifest_identity_roundtrip():
    identity = read_manifest_identity(_manifest_axml(PACKAGE, "160402020", 160402020))
    assert identity == {
        "package": PACKAGE,
        "version_name": "160402020",
        "version_code": 160402020,
    }


def test_download_host_allowlist_refuses_supercell_before_any_socket():
    assert assert_download_host(APKPURE_XAPK_URL) == "d.apkpure.net"
    assert assert_download_host("https://d.cdnpure.com/b/APK/com.supercell.clashroyale") == "d.cdnpure.com"
    assert assert_download_host("https://data.winudf.com/XAPK/example") == "data.winudf.com"
    for blocked in (
        "https://apkpure.com/clash-royale/com.supercell.clashroyale",
        "https://game.clashroyale.com/battle",
        "https://link.clashroyale.com/join",
        "https://api.supercell.com/v1",
        "http://127.0.0.1:8790/v1",
    ):
        with pytest.raises(ValueError, match="refusing download host"):
            assert_download_host(blocked)


def test_html_challenge_page_is_not_an_apk(tmp_path: Path):
    page = tmp_path / "base.apk"
    page.write_text("<!DOCTYPE html><html>Just a moment...</html>", encoding="utf-8")
    with pytest.raises(ValueError, match="HTML"):
        inspect_apk_zip(page)
    report = prepare(apk=page, out_dir=tmp_path / "var", run_rea=False)
    assert report["apk"] is None
    assert any("HTML" in line for line in report["blockers"])


def test_wrong_package_is_not_accepted(tmp_path: Path):
    apk = tmp_path / "other.apk"
    with zipfile.ZipFile(apk, "w") as archive:
        archive.writestr("AndroidManifest.xml", _manifest_axml("com.example.other", "1", 1))
        archive.writestr("classes.dex", b"dex")
    report = prepare(apk=apk, out_dir=tmp_path / "var", run_rea=False)
    assert report["apk"]["accepted"] is False
    assert report["apk"]["package"] == "com.example.other"
    assert report["rea"] is None
    assert any("com.example.other" in line for line in report["blockers"])


def test_xapk_prepare_keeps_only_the_base_split(tmp_path: Path):
    base = tmp_path / "inner.apk"
    with zipfile.ZipFile(base, "w") as archive:
        archive.writestr(
            "AndroidManifest.xml",
            _manifest_axml(PACKAGE, "160402020", 160402020),
        )
        archive.writestr("classes.dex", b"dex-bytes")
    xapk = tmp_path / "clash-royale.xapk"
    with zipfile.ZipFile(xapk, "w") as archive:
        archive.write(base, "com.supercell.clashroyale.apk")
        archive.writestr("config.en.apk", b"locale-split")
        archive.writestr("install_time_asset_pack.apk", b"assets-stay-packed")
        archive.writestr(
            "manifest.json",
            json.dumps(
                {
                    "package_name": PACKAGE,
                    "version_code": "160402020",
                    "version_name": "160402020",
                    "split_apks": [
                        {"file": "com.supercell.clashroyale.apk", "id": "base"},
                        {"file": "config.en.apk", "id": "config.en"},
                        {"file": "install_time_asset_pack.apk", "id": "install_time_asset_pack"},
                    ],
                }
            ),
        )
    out = tmp_path / "var"
    report = prepare(apk=xapk, out_dir=out, run_rea=False)
    assert report["apk"]["accepted"] is True
    assert report["apk"]["package"] == PACKAGE
    assert report["apk"]["version_name"] == "160402020"
    assert report["apk"]["version_code"] == 160402020
    assert report["xapk"]["base_member"] == "com.supercell.clashroyale.apk"
    assert (out / "base.apk").is_file()
    assert not (out / "config.en.apk").exists()
    assert not (out / "install_time_asset_pack.apk").exists()
    assert report["local_battle"]["real_engine"] is False
    assert report["local_battle"]["executed_apk"] is False


def test_replay_towers_does_not_invent_hit_points():
    actions = [
        {"card_id": 26000000, "tile_col": 10, "tile_row": 4, "seconds": 3.2, "side": "red"}
    ]
    with pytest.raises(ReplayEngineUnavailable, match="No tower HP was read from libg.so") as caught:
        replay_towers(
            red_deck=["knight"] * 8,
            blue_deck=["archers"] * 8,
            actions=actions,
        )
    message = str(caught.value)
    assert "Ghidra 12.1.4" in message
    assert "halt_baddata" in message
    assert "0x1560bd4" in message
    assert "socket" in message
    assert "tower_hp" not in message
    with pytest.raises(ValueError, match="8 cards"):
        replay_towers(red_deck=["knight"], blue_deck=["archers"] * 8, actions=[])


def test_harness_snapshot_has_no_tower_hp():
    catalog = load_catalog()
    harness = Harness(catalog, seed=1)
    harness.start(cards_for_preset("hog_cycle_2.6"), cards_for_preset("logbait"))
    harness.advance_to(2.0)
    snap = harness.snapshot()
    assert snap["real_engine"] is False
    assert snap["combat"] is False
    assert "tower_hp" not in snap
    assert "crowns" not in snap
    assert CREATE_GAME_MAIN.startswith("Java_com_supercell_titan_GameApp_")


def test_xapk_with_the_wrong_package_is_not_extracted(tmp_path: Path):
    xapk = tmp_path / "other.xapk"
    with zipfile.ZipFile(xapk, "w") as archive:
        archive.writestr("com.example.other.apk", b"PK\x03\x04not-used")
        archive.writestr(
            "manifest.json",
            json.dumps(
                {
                    "package_name": "com.example.other",
                    "split_apks": [{"file": "com.example.other.apk", "id": "base"}],
                }
            ),
        )
    out = tmp_path / "var"
    report = prepare(apk=xapk, out_dir=out, run_rea=False)
    assert report["apk"] is None
    assert report["xapk"]["accepted_package"] is False
    assert not (out / "base.apk").exists()
