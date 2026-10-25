"""Request validation without starting model, native engine or renderer processes."""
from __future__ import annotations

import json
import threading
import tempfile
import unittest
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import MagicMock, patch

from webapps.arena import match as match_module, renderer, server
from webapps.common import catalog
from webapps.common.httpkit import App, HTTPError, Request


def request(body, *, raw=False, session_id=None):
    return Request("POST", "/api/matches", {}, {"id": "fixture"}, {},
                   body if raw else json.dumps(body).encode(), session_id=session_id)


class ArenaValidationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        root = Path(self.temp.name)
        self.binary = root / "native-watch"
        self.binary.touch()
        self.checkpoint = root / "state"
        self.checkpoint.mkdir()
        (self.checkpoint / "checkpoint.json").write_text("{}")
        self.deck = [f"card-{i}" for i in range(8)]
        self.match = SimpleNamespace(id="fixture", owner="browser-a", human=False,
                                     control=MagicMock(), submit_input=MagicMock(),
                                     summary=MagicMock(return_value={"id": "fixture"}))
        self.manager = MagicMock()
        self.manager.get.return_value = self.match
        self.manager.create.return_value = self.match
        self.warmer = MagicMock()
        for patcher in (
            patch.object(server, "manager", self.manager),
            patch.object(server, "warmer", self.warmer),
            patch.object(server.paths, "NATIVE_WATCH", self.binary),
            patch.object(server.paths, "ENGINE_CHECKPOINT", self.checkpoint),
            patch.object(server, "_renderer_binary", return_value=self.binary),
            patch.object(server, "_renderer_assets", return_value=self.binary),
            patch.object(catalog, "cards", return_value=dict.fromkeys(self.deck)),
            patch.object(catalog, "engine_model", side_effect=lambda model: {
                "id": "random", "name": "Random bot", "available": True, "checkpoint": None,
            } if model == "random" else None),
        ):
            patcher.start()
            self.addCleanup(patcher.stop)

    def valid_body(self):
        return {"sides": [{"controller": "human", "deck": self.deck},
                          {"controller": "random", "deck": self.deck}], "seed": 4}

    def assert_bad(self, function, body, status=400, *, raw=False):
        with self.assertRaises(HTTPError) as caught:
            function(request(body, raw=raw))
        self.assertEqual(caught.exception.status, status)

    def test_non_object_json_is_400_on_each_json_route(self):
        for body in (None, [], "hello", 1, True):
            for function in (server.create_match, server.match_control):
                with self.subTest(body=body, route=function.__name__):
                    self.assert_bad(function, body)
            self.match.human = True
            self.assert_bad(server.match_input, body)
            self.match.human = False
        self.manager.create.assert_not_called()
        self.match.control.assert_not_called()
        self.match.submit_input.assert_not_called()

    def test_invalid_deck_elements_are_validation_errors(self):
        for value in ([], {}, None, 7, True):
            body = self.valid_body()
            body["sides"][0]["deck"] = [value, *self.deck[1:]]
            with self.subTest(value=value):
                self.assert_bad(server.create_match, body)
        self.manager.create.assert_not_called()

    def test_seed_must_be_an_integer_in_unsigned_32_bit_range(self):
        for seed in (False, 1.25, "1.25", [], {}, float("inf"), 2**32, -1):
            body = self.valid_body()
            body["seed"] = seed
            with self.subTest(seed=seed):
                self.assert_bad(server.create_match, body)
        self.manager.create.assert_not_called()

    def test_valid_request_keeps_exact_decks_and_seed(self):
        body = self.valid_body()
        body["seed"] = "4294967295"
        status, result = server.create_match(request(body))
        self.assertEqual((status, result), (201, {"id": "fixture"}))
        config = self.manager.create.call_args.args[0]
        self.assertEqual(config["seed"], 2**32 - 1)
        self.assertEqual([s["deck"] for s in config["sides"]], [self.deck, self.deck])
        self.warmer.enqueue.assert_called_once_with(self.deck, self.deck)

    def test_engine_and_renderer_files_required_before_enqueue_or_create(self):
        missing = Path(self.temp.name) / "missing"
        cases = (
            patch.object(server.paths, "NATIVE_WATCH", missing),
            patch.object(server.paths, "ENGINE_CHECKPOINT", missing),
            patch.object(server, "_renderer_binary", return_value=missing),
            patch.object(server, "_renderer_assets", return_value=missing),
        )
        for patcher in cases:
            with patcher:
                self.assert_bad(server.create_match, self.valid_body(), 503)
        # An empty directory is not a checkpoint.
        (self.checkpoint / "checkpoint.json").unlink()
        self.assert_bad(server.create_match, self.valid_body(), 503)
        self.warmer.enqueue.assert_not_called()
        self.manager.create.assert_not_called()

    def test_bad_control_values_never_reach_the_match_thread(self):
        for value in (None, [], {}, "fast", "2", True, float("nan"), float("inf"), -float("inf")):
            with self.subTest(value=value):
                self.assert_bad(server.match_control, {"op": "speed", "value": value})
        self.assert_bad(server.match_control, {"op": "pause", "value": "false"})
        self.match.control.assert_not_called()

    def test_valid_control_speed_is_clamped_and_pause_is_preserved(self):
        for value, expected in ((-2, 0.25), (0, 0.25), (2.5, 2.5), (100, 8), (10**1000, 8)):
            server.match_control(request({"op": "speed", "value": value}))
            self.match.control.assert_called_with({"op": "speed", "value": expected})
        server.match_control(request({"op": "speed"}))
        self.match.control.assert_called_with({"op": "speed", "value": 1})
        server.match_control(request({"op": "pause", "value": False}))
        self.match.control.assert_called_with({"op": "pause", "value": False})

    def test_human_speed_is_rejected_and_human_input_is_queued(self):
        self.match.human = True
        self.assert_bad(server.match_control, {"op": "speed", "value": 2})
        command = {"op": "select", "serial": 7, "match_id": 1, "slot": 0, "card_id": 42}
        self.assertEqual(server.match_input(request(command)), {"ok": True})
        self.match.submit_input.assert_called_once_with(command)

    def test_malformed_json_and_nonfinite_constants_are_400(self):
        for body in (b'{"sides":', b'{"seed": NaN}', b'{"value": Infinity}', b'\xff'):
            self.assert_bad(server.create_match, body, raw=True)

    def test_public_create_assigns_owner_and_rejects_existing_session_match(self):
        self.manager.list.return_value = []
        server.create_match(request(self.valid_body(), session_id="browser-a"))
        self.assertEqual(self.manager.create.call_args.args[0]["owner"], "browser-a")
        self.manager.list.assert_called_once_with(owner="browser-a")
        self.manager.create.reset_mock()
        self.warmer.enqueue.reset_mock()
        self.manager.list.return_value = [{"id": "fixture"}]
        with self.assertRaises(HTTPError) as caught:
            server.create_match(request(self.valid_body(), session_id="browser-a"))
        self.assertEqual(caught.exception.status, 429)
        self.manager.create.assert_not_called()
        self.warmer.enqueue.assert_not_called()

    def test_match_lists_are_scoped_to_the_browser_session(self):
        self.manager.list.return_value = [{"id": "fixture"}]
        self.assertEqual(server.list_matches(request({}, session_id="browser-a")),
                         {"matches": [{"id": "fixture"}]})
        self.manager.list.assert_called_with(owner="browser-a")
        server.list_matches(request({}))
        self.manager.list.assert_called_with(owner=None)

    def test_other_sessions_cannot_read_stream_input_control_or_close(self):
        functions = (server.get_match, server.stream, server.match_input,
                     server.match_control, server.close_match)
        for function in functions:
            with self.subTest(route=function.__name__):
                with self.assertRaises(HTTPError) as caught:
                    function(request({"op": "pause"}, session_id="browser-b"))
                self.assertEqual(caught.exception.status, 404)
        self.match.summary.assert_not_called()
        self.match.submit_input.assert_not_called()
        self.match.control.assert_not_called()
        self.manager.close.assert_not_called()

    def test_owner_and_local_requests_can_access_the_match(self):
        for owner in ("browser-a", None):
            self.assertEqual(server.get_match(request({}, session_id=owner)), {"id": "fixture"})
            self.assertEqual(server.close_match(request({}, session_id=owner)),
                             {"ok": self.manager.close.return_value})
        self.manager.close.assert_called_with("fixture")

    def test_original_asset_view_is_private_to_the_owner(self):
        app = App("private-view-test")
        renderer.register_routes(app, self.manager)
        view = app.routes[0][2]
        self.match.error = None
        self.match.renderer = MagicMock()
        self.match.renderer.image.return_value = ({"sequence": 4, "match_id": 1, "tick": 100,
                                                   "layout": {}}, b"jpeg")
        with self.assertRaises(HTTPError) as caught:
            view(request({}, session_id="browser-b"))
        self.assertEqual(caught.exception.status, 404)
        self.match.renderer.image.assert_not_called()
        self.assertEqual(view(request({}, session_id="browser-a")).body, b"jpeg")


class MatchSessionTests(unittest.TestCase):
    class StubMatch:
        def __init__(self, match_id, config):
            self.id = match_id
            self.owner = config.get("owner")
            self.closed = threading.Event()

        def summary(self):
            return {"id": self.id}

        def close(self):
            self.closed.set()

    def test_manager_filters_owners_and_closed_matches_release_the_session(self):
        manager = match_module.MatchManager(max_matches=4)
        with patch.object(match_module, "Match", self.StubMatch):
            first = manager.create({"owner": "browser-a"})
            second = manager.create({"owner": "browser-b"})
            self.assertEqual(manager.list(owner="browser-a"), [first.summary()])
            self.assertEqual(manager.list(owner="browser-b"), [second.summary()])
            self.assertEqual(len(manager.list()), 2)
            with self.assertRaisesRegex(RuntimeError, "already have a match"):
                manager.create({"owner": "browser-a"})
            manager.close(first.id)
            self.assertEqual(manager.list(owner="browser-a"), [])
            replacement = manager.create({"owner": "browser-a"})
            self.assertNotEqual(replacement.id, first.id)
            # Local mode has no per-browser cap.
            manager.create({})
            manager.create({})
            self.assertEqual(len(manager.list()), 4)

    def test_session_limit_is_atomic_for_simultaneous_creates(self):
        manager = match_module.MatchManager(max_matches=3)
        barrier = threading.Barrier(2)

        def create():
            barrier.wait(timeout=3)
            try:
                return manager.create({"owner": "browser-a"})
            except RuntimeError:
                return None

        with patch.object(match_module, "Match", self.StubMatch), ThreadPoolExecutor(max_workers=2) as pool:
            results = list(pool.map(lambda _index: create(), range(2)))
        self.assertEqual(sum(result is not None for result in results), 1)
        self.assertEqual(len(manager.list(owner="browser-a")), 1)

    def test_public_lifetime_timer_closes_even_a_paused_session(self):
        config = {"owner": "browser-a", "seed": 1,
                  "sides": [{"controller": "human", "deck": []}, {"controller": "random", "deck": []}]}
        with patch.object(match_module.threading, "Thread"), patch.object(match_module.threading, "Timer") as timer:
            match = match_module.Match("fixture", config)
            seconds, expire = timer.call_args.args
            self.assertEqual(seconds, 900)
            self.assertTrue(timer.return_value.daemon)
            timer.return_value.start.assert_called_once()
            match.status = "paused"
            expire()
            self.assertTrue(match.closed.is_set())
            timer.return_value.cancel.assert_called_once()

    def test_local_matches_have_no_total_lifetime_timer(self):
        config = {"seed": 1,
                  "sides": [{"controller": "human", "deck": []}, {"controller": "random", "deck": []}]}
        with patch.object(match_module.threading, "Thread"), patch.object(match_module.threading, "Timer") as timer:
            match = match_module.Match("fixture", config)
            self.assertIsNone(match.owner)
            timer.assert_not_called()
            match.close()


class ArenaOpeningMatchTests(unittest.TestCase):
    def test_lobby_opens_on_banana_2_log_bait_against_hog(self):
        body = server.get_catalog(None)
        self.assertEqual(body["defaults"], {
            "opponent_model": "banana-2",
            "opponent_deck": "log-bait",
            "player_model": "banana-hog-2",
            "player_deck": "hog-mirror",
        })
        models = {model["id"]: model for model in body["models"]}
        self.assertTrue(models["banana-1"]["legacy"])
        self.assertFalse(models["banana-1"]["recommended"])
        self.assertTrue(models["banana-2"]["available"])
        self.assertFalse(models["banana-2"].get("legacy"))
        preset_ids = {deck["id"] for deck in body["presets"]}
        self.assertIn(body["defaults"]["opponent_deck"], preset_ids)
        self.assertIn(body["defaults"]["player_deck"], preset_ids)
        names = {deck["id"]: deck["name"] for deck in body["presets"]}
        self.assertEqual(names["hog-mirror"], "Hog 2.6")
        self.assertEqual(names["log-bait"], "Classic Log Bait")
        lobby = Path(__file__).parent.joinpath("static", "lobby.js").read_text()
        self.assertIn("state.catalog.defaults", lobby)
        self.assertIn("legacy-models", lobby)
        self.assertNotIn('find((d) => d.id === "log-bait")', lobby)


if __name__ == "__main__":
    unittest.main()
