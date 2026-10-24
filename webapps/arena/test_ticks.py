"""Protocol-2 tick stream: order, resume, resync, batching, hands, remap, legacy."""
from __future__ import annotations

import json
import os
import queue
import tempfile
import threading
import time
import unittest
from email.message import Message
from pathlib import Path
from unittest.mock import patch

from webapps.arena import match as match_module
from webapps.arena import renderer, server
from webapps.arena.renderer import LazyStreamRenderer
from webapps.arena.scene import scene_legacy_enabled
from webapps.arena.ticks import AddressRemap, TickLog, addresses_remapped
from webapps.common import catalog
from webapps.common.httpkit import EventStream, Request, event_stream_chunks


def _snap(tick, **extra):
    body = {"tick": tick, "entities": []}
    body.update(extra)
    return body


def _drain(log, last_event_id=None):
    return list(log.events(last_event_id, match_id="match-1", summary=lambda: {"status": "running"}))


def _tick_numbers(events):
    numbers = []
    for event, payload, _event_id in events:
        if event == "ticks":
            numbers.extend(item["snapshot"]["tick"] for item in payload["items"])
    return numbers


class TickDeliveryTests(unittest.TestCase):
    def test_two_subscribers_receive_every_tick_in_order_including_a_slow_one(self):
        log = TickLog(batch_seconds=0, idle_seconds=2)
        results = [[], []]
        ready = [threading.Event(), threading.Event()]
        finished = [threading.Event(), threading.Event()]

        def read(index, delay):
            for event, payload, _event_id in log.events(None, match_id="m", summary=lambda: {}):
                if event == "ticks":
                    results[index].extend(item["snapshot"]["n"] for item in payload["items"])
                    if results[index] and results[index][0] == 0 and not ready[index].is_set():
                        ready[index].set()
                    if delay:
                        time.sleep(delay)
                elif event in ("closed", "error"):
                    finished[index].set()
                    return

        threads = [threading.Thread(target=read, args=(0, 0)),
                   threading.Thread(target=read, args=(1, 0.01))]
        for thread in threads:
            thread.start()
        log.publish(1, 0, {"n": 0, "tick": 0}, {"n": 0})
        self.assertTrue(ready[0].wait(2) and ready[1].wait(2))
        for number in range(1, 30):
            log.publish(1, number, {"n": number, "tick": number}, {"n": number})
        time.sleep(0.45)
        log.close()
        self.assertTrue(all(event.wait(2) for event in finished))
        for thread in threads:
            thread.join(2)
        self.assertEqual(results[0], list(range(30)))
        self.assertEqual(results[1], list(range(30)))

    def test_last_event_id_resumes_after_that_tick_without_repeating_earlier_ones(self):
        log = TickLog(batch_seconds=0, idle_seconds=2)
        for tick in range(1, 21):
            log.publish(1, tick, _snap(tick), {"tick": tick})
        log.close()
        events = _drain(log, "1:10")
        self.assertNotIn("resync", [event for event, _payload, _id in events])
        self.assertEqual(_tick_numbers(events), list(range(11, 21)))
        ticks = [event for event in events if event[0] == "ticks"]
        self.assertTrue(all(event[2] == f"1:{event[1]['items'][-1]['snapshot']['tick']}" for event in ticks))
        self.assertEqual(events[-1][0], "closed")

    def test_a_subscriber_behind_the_ring_is_resent_the_ring_after_resync(self):
        log = TickLog(batch_seconds=0, idle_seconds=2)
        for tick in range(1, 451):
            log.publish(1, tick, _snap(tick), {"tick": tick})
        log.close()
        events = _drain(log, "1:1")
        self.assertEqual(events[0][0], "resync")
        self.assertEqual(events[0][1]["reason"], "behind")
        numbers = _tick_numbers(events)
        self.assertEqual(numbers, list(range(51, 451)))
        self.assertEqual(len(numbers), 400)

    def test_falling_behind_during_a_live_subscription_resyncs(self):
        log = TickLog(batch_seconds=0, idle_seconds=2)
        log.publish(1, 0, _snap(0), {})
        generator = log.events(None, match_id="m", summary=lambda: {})
        first = next(generator)
        self.assertEqual(_tick_numbers([first]), [0])
        for tick in range(1, 501):
            log.publish(1, tick, _snap(tick), {})
        self.assertEqual(next(generator)[0], "resync")
        rest = _tick_numbers([next(generator)])
        self.assertEqual(rest[0], 101)
        self.assertEqual(rest[-1], 500)
        self.assertEqual(len(rest), 400)
        generator.close()

    def test_a_new_round_is_announced_before_its_ticks(self):
        log = TickLog(batch_seconds=0, idle_seconds=2)
        log.publish(1, 5, _snap(5), {}, seed=4)
        log.publish(1, 6, _snap(6), {}, seed=4)
        log.publish(2, 100, _snap(100), {}, seed=9)
        log.close()
        names = [event for event, _payload, _id in _drain(log)]
        self.assertEqual(names, ["ticks", "round", "ticks", "closed"])
        events = _drain(log)
        # The log is closed; a second subscriber still sees the ring.
        round_event = events[1]
        self.assertEqual(round_event[1], {"v": 2, "match_number": 2, "seed": 9})
        self.assertEqual(_tick_numbers(events), [5, 6, 100])

    def test_four_times_speed_emits_about_three_ticks_per_33ms_batch(self):
        log = TickLog(batch_seconds=0.033, idle_seconds=5)
        received = []
        sizes = []
        times = []
        finished = threading.Event()

        def read():
            for event, payload, _event_id in log.events(None, match_id="m", summary=lambda: {}):
                if event == "ticks":
                    times.append(time.monotonic())
                    sizes.append(len(payload["items"]))
                    received.extend(item["snapshot"]["tick"] for item in payload["items"])
                elif event in ("closed", "error"):
                    finished.set()
                    return

        thread = threading.Thread(target=read)
        thread.start()
        time.sleep(0.03)
        started = time.monotonic()
        for tick in range(24):
            delay = started + tick * 0.0125 - time.monotonic()
            if delay > 0:
                time.sleep(delay)
            log.publish(1, tick, _snap(tick), {"tick": tick})
        time.sleep(0.08)
        log.close()
        self.assertTrue(finished.wait(3))
        thread.join(2)
        self.assertEqual(received, list(range(24)))
        self.assertGreaterEqual(len(sizes), 4)
        self.assertLess(len(sizes), 24)
        gaps = [times[index] - times[index - 1] for index in range(1, len(times))]
        self.assertTrue(gaps and all(gap >= 0.025 for gap in gaps), gaps)
        mean = sum(sizes) / len(sizes)
        self.assertGreaterEqual(mean, 1.5)
        self.assertLessEqual(mean, 6.0)

    def test_status_error_and_closed_and_gzip_event_ids(self):
        log = TickLog(batch_seconds=0, idle_seconds=30)
        found = []

        def read():
            for event, payload, event_id in log.events(None, match_id="match-9", summary=lambda: {"status": "loading"}):
                found.append((event, payload, event_id))
                if event == "status":
                    return

        thread = threading.Thread(target=read)
        thread.start()
        time.sleep(0.02)
        log.nudge()
        thread.join(2)
        self.assertEqual(found, [("status", {"status": "loading"}, None)])

        log.fail("engine stopped")
        events = _drain(log)
        self.assertEqual(events[0][0], "error")
        self.assertEqual(events[0][1], {"error": "engine stopped"})

        closed = TickLog(batch_seconds=0, idle_seconds=2)
        closed.close()
        self.assertEqual(_drain(closed)[-1], ("closed", {"id": "match-1"}, None))

        import gzip
        raw = b"".join(event_stream_chunks([("ticks", {"v": 2, "items": [{"snapshot": {}, "hud": {}}]}, "3:12")], gzip=True))
        text = gzip.decompress(raw)
        self.assertEqual(text, b'id: 3:12\nevent: ticks\ndata: {"v":2,"items":[{"snapshot":{},"hud":{}}]}\n\n')
        self.assertIn(b"\n\n", text)


class PublicationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)

    def _match(self):
        match = match_module.Match.__new__(match_module.Match)
        match.ticks = TickLog(batch_seconds=0, idle_seconds=2)
        match.remapper = AddressRemap()
        return match

    def test_remap_is_stable_inside_a_round_and_hidden_when_the_flag_is_off(self):
        original = {"tick": 4, "entities": [
            {"address": 1000, "native_entity_id": 7, "native_focus_entity": 2000,
             "native_focus_entity_id": 8, "row": "Knight"},
            {"address": 2000, "native_entity_id": 8, "native_focus_entity": None, "row": "Archer"},
        ]}
        payload = {"tick": 4, "hands": [[{"name": "Knight"}], [{"name": "Visible"}]],
                   "entities": [{"i": 7, "a": 1000}]}
        match = self._match()
        with patch.dict(os.environ, {"ARENA_REMAP_ADDRESSES": "0"}):
            self.assertFalse(addresses_remapped())
            match.deliver_tick(1, 4, original, payload, seed=1)
        stored = match.ticks.items[0]
        self.assertEqual(stored.snapshot["entities"][0]["address"], 1000)
        self.assertNotIn("entities", stored.hud)
        self.assertEqual(original["entities"][0]["address"], 1000)

        remapped = self._match()
        with patch.dict(os.environ, {"ARENA_REMAP_ADDRESSES": "1"}):
            remapped.deliver_tick(1, 4, original, payload, seed=1)
            again = {"tick": 5, "entities": [
                {"address": 1000, "native_entity_id": 7, "native_focus_entity": 2000,
                 "native_focus_entity_id": 8, "row": "Knight"},
                {"address": 2000, "native_entity_id": 8, "row": "Archer"},
            ]}
            remapped.deliver_tick(1, 5, again, {"tick": 5, "entities": []}, seed=1)
            # A fresh round must not keep the previous round's id for this address.
            remapped.deliver_tick(2, 1, {"tick": 1, "entities": [
                {"address": 2000, "native_entity_id": 8, "row": "Archer"}]},
                {"tick": 1, "entities": []}, seed=2)
        first, second, third = remapped.ticks.items
        knight, archer = first.snapshot["entities"]
        self.assertEqual(knight["native_focus_entity"], archer["address"])
        self.assertEqual(knight["native_focus_entity_id"], archer["native_entity_id"])
        self.assertNotEqual(knight["address"], 1000)
        self.assertNotEqual(knight["native_entity_id"], knight["address"])
        self.assertEqual(second.snapshot["entities"][0]["address"], knight["address"])
        self.assertEqual(second.snapshot["entities"][0]["native_entity_id"], knight["native_entity_id"])
        self.assertNotEqual(third.snapshot["entities"][0]["address"], archer["address"])
        self.assertNotIn("entities", first.hud)
        self.assertNotIn("1000", json.dumps(first.hud))
        self.assertEqual(original["entities"][0]["address"], 1000)

    def test_publish_hides_the_opponent_hand_and_remaps_only_the_streamed_snapshot(self):
        cards = catalog.cards()
        deck = next(preset["cards"] for preset in catalog.presets() if len(preset["cards"]) == 8)
        natives = [cards[slug]["native"] for slug in deck]
        towers = []
        for side, kind, ident, x in (
            (0, "PrincessTower", 1, 3000), (0, "PrincessTower", 2, 14000), (0, "KingTower", 3, 9000),
            (1, "PrincessTower", 4, 3000), (1, "PrincessTower", 5, 14000), (1, "KingTower", 6, 9000),
        ):
            towers.append({"row": kind, "side": side, "native_entity_id": ident, "address": 5000 + ident,
                           "native_focus_entity": 5002 if ident == 1 else None,
                           "native_focus_entity_id": 2 if ident == 1 else None,
                           "x": x, "y": 2000 if side == 0 else 30000, "z": 0, "hp": 1000, "max_hp": 2000})
        secret = "ShouldStayHidden"
        hands = [
            [{"slot": 0, "card_id": 11, "name": natives[0], "cost": 3, "available": True}],
            [{"slot": 0, "card_id": 99, "name": secret, "cost": 4, "available": True}],
        ]
        observation = {"tick": 40, "elixir": [5.0, 6.0], "hands": hands, "winner_side": None}
        frame = {"tick": 40, "entities": towers, "elixir": [5.0, 6.0], "arena_size": [18000, 32000]}

        class Engine:
            def __init__(self, owner):
                self.owner = owner

            def request(self, value):
                if value["op"] == "step":
                    self.owner.closed.set()
                    obs = dict(observation, tick=140)
                else:
                    obs = dict(observation, tick=40)
                result = {"frame": frame, "observation": obs, "terminated": False, "truncated": False, "actions": []}
                if value["op"] == "reset":
                    result["configured_decks"] = [{"cards": list(natives)}, {"cards": list(natives)}]
                return result

        match = match_module.Match.__new__(match_module.Match)
        match.id = "hidden"
        match.human = True
        match.sides = [{"controller": "human", "deck": deck, "label": "You"},
                       {"controller": "random", "deck": deck, "label": "Bot"}]
        match.labels = ["You", "Bot"]
        match.seed = 3
        match.created_at = "2026-10-24T00:00:00+00:00"
        match.message = "loading"
        match.rows = {}
        match.status = "loading"
        match.run_dir = self.root / "run"
        match.log_streams = []
        match.cond = threading.Condition()
        match.payload = None
        match.seq = 0
        match.subscribers = 0
        match.last_watched = time.monotonic()
        match.closed = threading.Event()
        match.inputs = queue.Queue()
        match.controls = queue.Queue()
        match.ticks = TickLog(batch_seconds=0, idle_seconds=2)
        match.remapper = AddressRemap()
        match.result_history = []
        match.renderer = LazyStreamRenderer(match.run_dir, cards=natives)
        engine = Engine(match)
        with patch.dict(os.environ, {"ARENA_REMAP_ADDRESSES": "1", "ARENA_SCENE_LEGACY": "0"}), \
                patch.object(match_module, "engine_sha256", return_value="0" * 64):
            match._loop(engine, [None, None])
        self.assertIsInstance(match.renderer, LazyStreamRenderer)
        rendered = match.renderer.last["entities"][0]["address"]
        self.assertEqual(rendered, 5001)
        streamed = match.ticks.items[-1]
        body = json.dumps(streamed.snapshot) + json.dumps(streamed.hud)
        self.assertNotIn(secret, body)
        self.assertIsNone(streamed.hud["hands"][1])
        self.assertEqual(streamed.snapshot["hands"][1], [])
        self.assertTrue(streamed.hud["hands"][0])
        self.assertTrue(streamed.snapshot["hands"][0])
        self.assertNotIn("entities", streamed.hud)
        knight = streamed.snapshot["entities"][0]
        self.assertNotEqual(knight["address"], 5001)
        focused = next(entity for entity in streamed.snapshot["entities"] if entity["native_entity_id"] == knight["native_focus_entity_id"])
        self.assertEqual(knight["native_focus_entity"], focused["address"])
        match.ticks.close()
        match.renderer.close()
        for stream in match.log_streams:
            stream.close()

    def test_legacy_flag_selects_the_scene_presenter_and_the_default_does_not_spawn_it(self):
        with patch.dict(os.environ, {"ARENA_SCENE_LEGACY": "0"}):
            self.assertFalse(scene_legacy_enabled())
        with patch.dict(os.environ, {"ARENA_SCENE_LEGACY": "1", "ARENA_BROWSER_SCENE": "1"}), \
                patch.object(match_module, "NativeSceneRenderer") as scene_cls, \
                patch.object(match_module, "NativeRenderer") as jpeg_cls, \
                patch.object(match_module, "LazyStreamRenderer") as lazy_cls:
            match_module.make_renderer(self.root / "legacy", ["Knight"])
            scene_cls.assert_called_once()
            jpeg_cls.assert_not_called()
            lazy_cls.assert_not_called()
        with patch.dict(os.environ, {"ARENA_SCENE_LEGACY": "1", "ARENA_BROWSER_SCENE": "0"}), \
                patch.object(match_module, "NativeSceneRenderer") as scene_cls, \
                patch.object(match_module, "NativeRenderer") as jpeg_cls, \
                patch.object(match_module, "LazyStreamRenderer") as lazy_cls:
            match_module.make_renderer(self.root / "jpeg", ["Knight"])
            jpeg_cls.assert_called_once()
            scene_cls.assert_not_called()
        with patch.dict(os.environ, {"ARENA_SCENE_LEGACY": "0", "ARENA_BROWSER_SCENE": "1"}), \
                patch.object(match_module, "NativeSceneRenderer") as scene_cls, \
                patch.object(match_module, "LazyStreamRenderer") as lazy_cls:
            match_module.make_renderer(self.root / "ticks", ["Knight"])
            lazy_cls.assert_called_once()
            scene_cls.assert_not_called()

    def test_stream_view_starts_the_jpeg_process_only_when_asked(self):
        view = LazyStreamRenderer(self.root / "lazy", cards=["Knight"])
        with patch.object(renderer, "NativeRenderer") as jpeg:
            view.submit({"tick": 7, "entities": []})
            jpeg.assert_not_called()
            inner = jpeg.return_value
            inner.image.return_value = ({"sequence": 1}, b"jpeg")
            self.assertEqual(view.image(0)[1], b"jpeg")
            jpeg.assert_called_once()
            inner.submit.assert_called_once_with({"tick": 7, "entities": []})
            view.submit({"tick": 8})
            self.assertEqual(inner.submit.call_args.args[0], {"tick": 8})
            view.close()
            inner.close.assert_called_once()

    def test_ticks_route_is_gzipped_owner_checked_and_passes_last_event_id(self):
        captured = {}

        class Stub:
            id = "fixture"
            owner = "browser-a"
            subscribers = 0
            last_watched = 0

            def tick_stream(self, last_event_id):
                captured["last"] = last_event_id
                yield ("meta", {"protocol": 2, "cards": []}, None)
                yield ("ticks", {"v": 2, "items": []}, "1:4")

        stub = Stub()
        headers = Message()
        headers["Last-Event-ID"] = "1:4"
        request = Request("GET", "/api/matches/fixture/ticks", {}, {"id": "fixture"}, headers, b"",
                          session_id="browser-a")
        with patch.object(server, "manager") as manager:
            manager.get.return_value = stub
            stream = server.ticks(request)
        self.assertIsInstance(stream, EventStream)
        self.assertTrue(stream.gzip)
        self.assertEqual(list(stream.events), [
            ("meta", {"protocol": 2, "cards": []}, None),
            ("ticks", {"v": 2, "items": []}, "1:4"),
        ])
        self.assertEqual(captured["last"], "1:4")
        self.assertEqual(stub.subscribers, 0)

        stranger = Request("GET", "/api/matches/fixture/ticks", {}, {"id": "fixture"}, {}, b"",
                           session_id="browser-b")
        with patch.object(server, "manager") as manager:
            manager.get.return_value = stub
            from webapps.common.httpkit import HTTPError
            with self.assertRaises(HTTPError) as caught:
                server.ticks(stranger)
        self.assertEqual(caught.exception.status, 404)


if __name__ == "__main__":
    unittest.main()
