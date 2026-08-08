from __future__ import annotations

import json
from pathlib import Path

from cr_replay_pipeline.phone_lab.calibration import (
    arena_uv_to_pixel,
    card_slot_rects_for_size,
    load_scaled_calibration,
    placement_pixel,
    rect_center,
    resolve_placement,
    scale_rect,
)
from cr_replay_pipeline.phone_lab.stream_source import (
    ACTION_DOWN,
    ACTION_UP,
    PacketHub,
    VideoPacket,
    pack_back_or_screen_on,
    pack_touch_event,
)
from cr_replay_pipeline.phone_lab.websocket_util import accept_key


CALIB_DIR = Path("data/phone_lab/calibrations")


def test_scale_rect_preserves_ratios():
    src = {"x_min": 0, "y_min": 0, "x_max": 100, "y_max": 200}
    out = scale_rect(src, src_w=100, src_h=200, dst_w=200, dst_h=400)
    assert out == {"x_min": 0, "y_min": 0, "x_max": 200, "y_max": 400}


def test_load_scaled_calibration_for_pixel8():
    calib = load_scaled_calibration(
        "41060DLJH000KW",
        width=1080,
        height=2400,
        calib_dir=CALIB_DIR,
    )
    assert calib["screen_height"] == 2400
    assert "card_slot_0" in calib["zones"]
    assert "blue_arena" in calib["zones"]
    cx, cy = rect_center(calib["zones"]["card_slot_0"])
    assert 0 < cx < 1080
    assert 0 < cy < 2400
    px, py = placement_pixel(calib, "bottom_left")
    arena = calib["zones"]["blue_arena"]
    assert arena["x_min"] <= px <= arena["x_max"]
    assert arena["y_min"] <= py <= arena["y_max"]


def test_placement_uses_clicked_points_only(tmp_path):
    src = CALIB_DIR / "4B090DLAQ002ZT_unified.json"
    data = json.loads(src.read_text(encoding="utf-8"))
    data["placement_points"] = {
        "bridge_left": {"x": 111, "y": 1222},
        "bridge_right": {"x": 999, "y": 1222},
        "bottom_left": {"x": 101, "y": 1700},
        "bottom_right": {"x": 980, "y": 1700},
        "enemy_left": {"x": 120, "y": 500},
        "enemy_right": {"x": 960, "y": 500},
    }
    out = tmp_path / "4B090DLAQ002ZT_unified.json"
    out.write_text(json.dumps(data), encoding="utf-8")
    calib = load_scaled_calibration(
        "4B090DLAQ002ZT",
        width=1080,
        height=2424,
        calib_dir=tmp_path,
    )
    assert placement_pixel(calib, "bridge_left") == (111, 1222)
    assert placement_pixel(calib, "enemy_right") == (960, 500)
    assert resolve_placement(calib, preset="bottom_left") == (101, 1700)
    # Mid-bridge: u=0.5, v=0.5
    mx, my = arena_uv_to_pixel(calib, 0.5, 0.5)
    assert abs(mx - (111 + 999) // 2) <= 1
    assert abs(my - 1222) <= 1


def test_placement_requires_clicked_calibration(tmp_path):
    src = CALIB_DIR / "4B090DLAQ002ZT_unified.json"
    data = json.loads(src.read_text(encoding="utf-8"))
    data.pop("placement_points", None)
    out = tmp_path / "4B090DLAQ002ZT_unified.json"
    out.write_text(json.dumps(data), encoding="utf-8")
    calib = load_scaled_calibration(
        "4B090DLAQ002ZT",
        width=1080,
        height=2424,
        calib_dir=tmp_path,
    )
    try:
        placement_pixel(calib, "bottom_left")
        assert False, "expected RuntimeError"
    except RuntimeError as exc:
        assert "phone-lab-calibrate" in str(exc)


def test_websocket_accept_key_rfc6455():
    # Example from RFC 6455 section 4.2.2
    assert accept_key("dGhlIHNhbXBsZSBub25jZQ==") == "s3pPLMBiTxaQ9kYGzzhZRbK+xOo="


def test_packet_hub_drops_to_keyframe_under_backpressure():
    hub = PacketHub(maxsize=2)
    hub.push(VideoPacket(b"cfg", is_config=True, is_keyframe=False, pts=0))
    hub.push(VideoPacket(b"k1", is_config=False, is_keyframe=True, pts=1))
    hub.push(VideoPacket(b"p1", is_config=False, is_keyframe=False, pts=2))
    hub.push(VideoPacket(b"p2", is_config=False, is_keyframe=False, pts=3))
    hub.push(VideoPacket(b"k2", is_config=False, is_keyframe=True, pts=4))

    seen = []
    while True:
        pkt = hub.pop(timeout=0.01)
        if pkt is None:
            break
        seen.append(pkt.data)
    assert seen == [b"cfg", b"k2"]
    hub.close()


def test_card_slot_rects_scale_to_stream_size():
    calib = load_scaled_calibration(
        "4B090DLAQ002ZT",
        width=1080,
        height=2424,
        calib_dir=CALIB_DIR,
    )
    slots = card_slot_rects_for_size(calib, width=424, height=960)
    assert len(slots) == 4
    for rect in slots:
        assert 0 <= rect["x_min"] < rect["x_max"] <= 424
        assert 0 <= rect["y_min"] < rect["y_max"] <= 960


def test_pack_touch_event_is_32_bytes():
    pkt = pack_touch_event(ACTION_DOWN, 100, 200, 424, 960, pressure=1.0)
    assert len(pkt) == 32
    assert pkt[0] == 2  # INJECT_TOUCH
    assert pkt[1] == ACTION_DOWN
    assert pack_touch_event(ACTION_UP, 0, 0, 10, 10, pressure=0.0)[1] == ACTION_UP
    assert pack_back_or_screen_on(ACTION_DOWN) == bytes((4, 0))


def test_logic_xy_to_uv_own_side_is_high_v():
    from cr_replay_pipeline.phone_lab.battle import logic_xy_to_uv

    u, v = logic_xy_to_uv(9000, 8000)  # own-ish y in replay space
    assert abs(u - 0.5) < 1e-6
    assert v > 0.5
    u2, v2 = logic_xy_to_uv(0, 32000)  # enemy back
    assert abs(u2) < 1e-6
    assert abs(v2) < 1e-6


def test_find_hand_slot_normalizes_names():
    from cr_replay_pipeline.phone_lab.battle import find_hand_slot, resolve_playable_card

    hand = [
        {"slot": 0, "card_name": "Hog Rider"},
        {"slot": 1, "card_name": "the-log"},
        {"slot": 2, "card_name": None},
        {"slot": 3, "card_name": "ice_spirit"},
    ]
    assert find_hand_slot("hog-rider", hand) == 0
    assert find_hand_slot("ice-spirit", hand) == 3
    assert find_hand_slot("cannon", hand) is None

    # YOLO labels evolutions as ``*-evo``; policy decks use the base id.
    evo_hand = [
        {"slot": 0, "card_name": "knight-evo"},
        {"slot": 1, "card_name": "mortar-evo"},
        {"slot": 2, "card_name": "archers"},
        {"slot": 3, "card_name": None},
    ]
    assert find_hand_slot("knight", evo_hand) == 0
    assert find_hand_slot("mortar", evo_hand) == 1
    got = resolve_playable_card(
        {"card": "knight", "ranked_slots": [{"card": "knight", "prob": 0.9}]},
        evo_hand,
        {"knight": 3, "mortar": 4, "archers": 3},
    )
    assert got == ("knight", 0, 3)


def test_validate_deck_and_elixir():
    from cr_replay_pipeline.phone_lab.battle import (
        ElixirTracker,
        PRESET_DECKS,
        validate_deck,
    )

    known = set(PRESET_DECKS["hog_cycle_2.6"]) | {"zap"}
    deck = validate_deck(PRESET_DECKS["hog_cycle_2.6"], known)
    assert len(deck) == 8
    try:
        validate_deck(deck[:7], known)
        assert False, "expected ValueError"
    except ValueError:
        pass

    elixir = ElixirTracker()
    elixir.update(2.8)
    assert elixir.values["pixel8"] > 5.0
    assert elixir.spend("pixel8", 4)
    assert elixir.values["pixel8"] < 6.0


def test_battle_start_validation_without_phones():
    from cr_replay_pipeline.phone_lab.battle import BattleConfig, BattleRunner

    calls = {"detect": 0, "exec": 0}

    def detect(_phone):
        calls["detect"] += 1
        return [
            {"slot": i, "card_name": None, "confidence": 0.0} for i in range(4)
        ]

    def execute(*_args):
        calls["exec"] += 1
        return {}

    runner = BattleRunner(detect_hand=detect, execute_action=execute)
    try:
        runner.start(
            BattleConfig(
                decks={"pixel8": ["a"] * 8, "pixel9": ["b"] * 8},
                controllers={
                    "pixel8": "policy_bc_v4",
                    "pixel9": "manual",
                },
            )
        )
        assert False, "expected validation error"
    except ValueError:
        pass
    assert not runner.running


def test_flip_logic_y_and_resolve_playable():
    from cr_replay_pipeline.phone_lab.battle import (
        flip_logic_y,
        opponent_perspective_xy,
        resolve_playable_card,
    )

    assert flip_logic_y(8000) == 24000
    assert flip_logic_y(24000) == 8000
    # Their screen-left (x=4000) is our screen-right when facing.
    assert opponent_perspective_xy(4000, 8000) == (14000, 24000)

    hand = [
        {"slot": 0, "card_name": "cannon"},
        {"slot": 1, "card_name": "ice-spirit"},
        {"slot": 2, "card_name": None},
        {"slot": 3, "card_name": "skeletons"},
    ]
    pred = {
        "card": "hog-rider",
        "slot": 0,
        "ranked_slots": [
            {"slot": 0, "card": "hog-rider", "prob": 0.5},
            {"slot": 1, "card": "cannon", "prob": 0.3},
            {"slot": 2, "card": "ice-spirit", "prob": 0.1},
        ],
    }
    costs = {"hog-rider": 4, "cannon": 3, "ice-spirit": 1, "skeletons": 1}
    got = resolve_playable_card(pred, hand, costs)
    assert got == ("cannon", 0, 3)


def test_phone_battle_view_flips_opponent_y():
    from cr_replay_pipeline.phone_lab.battle import BattleConfig, BattleRunner

    runner = BattleRunner(
        detect_hand=lambda _p: [],
        execute_action=lambda *_a: {},
    )
    runner._config = BattleConfig(
        decks={
            "pixel8": [
                "hog-rider",
                "cannon",
                "fireball",
                "ice-golem",
                "ice-spirit",
                "musketeer",
                "skeletons",
                "the-log",
            ],
            "pixel9": [
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
        controllers={"pixel8": "manual", "pixel9": "manual"},
    )
    runner._events = [
        {
            "seconds": 5.0,
            "side": "pixel9",
            "event_type": "card_play",
            "card": "knight",
            "x": 9000,
            "y": 8000,
        },
        {
            "seconds": 8.0,
            "side": "pixel8",
            "event_type": "card_play",
            "card": "cannon",
            "x": 7000,
            "y": 6000,
        },
    ]
    view8 = runner._phone_battle_view("pixel8")
    # pixel8's own play stays team / same xy; pixel9's play is opponent with X+Y flip
    assert view8.events[0]["side"] == "opponent"
    assert view8.events[0]["x"] == 9000  # 18000 - 9000
    assert view8.events[0]["y"] == 24000
    assert view8.events[1]["side"] == "team"
    assert view8.events[1]["x"] == 7000
    assert view8.events[1]["y"] == 6000

    view9 = runner._phone_battle_view("pixel9")
    assert view9.events[0]["side"] == "team"
    assert view9.events[0]["x"] == 9000
    assert view9.events[0]["y"] == 8000
    assert view9.events[1]["side"] == "opponent"
    assert view9.events[1]["x"] == 11000  # 18000 - 7000
    assert view9.events[1]["y"] == 26000


def test_plan_delay_is_short_when_answering():
    from cr_replay_pipeline.phone_lab.battle import (
        BattleConfig,
        BattleRunner,
        MAX_PLAN_DELAY_S,
        REACT_DELAY_CAP_S,
    )

    runner = BattleRunner(
        detect_hand=lambda _p: [],
        execute_action=lambda *_a: {},
    )
    runner._config = BattleConfig(
        decks={
            "pixel8": [
                "hog-rider",
                "cannon",
                "fireball",
                "ice-golem",
                "ice-spirit",
                "musketeer",
                "skeletons",
                "the-log",
            ],
            "pixel9": [
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
        controllers={"pixel8": "manual", "pixel9": "manual"},
    )
    # No history → normal (capped) delay.
    assert runner._plan_delay("pixel8", 5.0) == MAX_PLAN_DELAY_S
    runner._events = [
        {
            "seconds": 1.0,
            "side": "pixel9",
            "event_type": "card_play",
            "card": "knight",
            "x": 9000,
            "y": 8000,
        }
    ]
    # pixel8 answering pixel9 → react cap.
    assert runner._plan_delay("pixel8", 2.5) == REACT_DELAY_CAP_S
    # pixel9 just played → not answering itself.
    assert runner._plan_delay("pixel9", 2.5) == MAX_PLAN_DELAY_S


def test_empty_hands_auto_stop_battle():
    import time

    from cr_replay_pipeline.phone_lab.battle import (
        PRESET_DECKS,
        BattleConfig,
        BattleRunner,
    )

    empty = [{"slot": i, "card_name": None, "confidence": 0.0} for i in range(4)]
    detects = {"n": 0}

    def detect(_phone):
        detects["n"] += 1
        return list(empty)

    execs = {"n": 0}

    def execute(*_args):
        execs["n"] += 1
        return {}

    runner = BattleRunner(detect_hand=detect, execute_action=execute)
    # Manual on both so we don't need to load policy checkpoints; force AI
    # by patching after start is awkward — use v4 only if model exists.
    from pathlib import Path

    if not Path("models/policy_bc_v4/best_model.pt").exists():
        return
    runner.start(
        BattleConfig(
            decks={
                "pixel8": list(PRESET_DECKS["hog_cycle_2.6"]),
                "pixel9": list(PRESET_DECKS["logbait"]),
            },
            controllers={
                "pixel8": "policy_bc_v4",
                "pixel9": "policy_bc_v4",
            },
            timeout_s=30.0,
        )
    )
    deadline = time.time() + 20.0
    while runner.running and time.time() < deadline:
        time.sleep(0.2)
    assert not runner.running
    assert execs["n"] == 0
    messages = " ".join(line["message"] for line in runner.status()["log"])
    assert "YOLO lost both hands" in messages or "all AI hands empty" in messages
