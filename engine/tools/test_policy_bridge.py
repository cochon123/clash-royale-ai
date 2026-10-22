#!/usr/bin/env python3
"""Bridge contract checks; no PyTorch installation or checkpoint load required.

Run: python3 tools/test_policy_bridge.py
The separate analysis/policy_bridge_smoke.json records real checkpoint inference.
"""
import copy
from dataclasses import dataclass
import json
from pathlib import Path
import subprocess
import sys
import types
import unittest
from unittest.mock import Mock, patch

import policy_bridge


@dataclass
class BattleExample:
    battle_id: str
    team_deck: tuple
    opponent_deck: tuple
    team_wins: int
    events: tuple


class BridgeContractTests(unittest.TestCase):
    def setUp(self):
        self.bridge = policy_bridge.Bridge(policy_bridge.DEFAULT_AI_REPO, "v9", "cpu")
        self.request = {
            "team": "blue", "elapsed_ms": 3000, "elixir": 3,
            "deck": ["hog_rider", "musketeer", "ice_golem", "skeletons", "ice_spirit", "cannon", "fireball", "the_log"],
            "opponent_deck": ["knight", "archers", "goblins", "giant", "minions", "spear_goblins", "arrows", "zap"],
            "hand": ["hog_rider", "musketeer", "ice_golem", "skeletons"],
            "history": [{"team": "red", "elapsed_ms": 1000, "card": "knight", "x": 4500, "y": 9000}],
        }
        vocab = types.SimpleNamespace(name_to_id={self.bridge.card(c): i for i, c in enumerate(self.request["deck"] + self.request["opponent_deck"])})
        self.bridge.load = Mock(return_value=(object(), vocab, {}))
        self.predictor = Mock(return_value={
            "status": "proposal", "card": "ice-golem", "x": .25, "y": .10,
            "delay_seconds": 1.25, "wait_for_opponent": False,
        })
        self.legacy_predictor = Mock(return_value={
            "card": "ice-golem", "event_type": "card_play", "x": 4500,
            "y": 3200, "delay_seconds": 1.25,
        })
        modules = {
            "cr_replay_pipeline": types.ModuleType("cr_replay_pipeline"),
            "cr_replay_pipeline.winner_dataset": types.SimpleNamespace(BattleExample=BattleExample),
            "cr_replay_pipeline.v9_infer": types.SimpleNamespace(predict=self.predictor),
            "cr_replay_pipeline.policy_infer": types.SimpleNamespace(predict_next_action=self.legacy_predictor),
            "torch": types.SimpleNamespace(device=lambda value: value),
        }
        self.modules = patch.dict(sys.modules, modules)
        self.modules.start()
        self.addCleanup(self.modules.stop)

    def test_every_playable_card_has_explicit_mapping_and_current_cost(self):
        for card in json.loads((policy_bridge.ROOT / "data/cards.json").read_text()):
            with self.subTest(card=card["id"]):
                policy_card = self.bridge.card(card["id"])
                self.assertEqual(self.bridge.costs[policy_card], card["elixir"])

    def test_affordable_observed_hand_is_enforced_and_preserves_ui_slot(self):
        result = self.bridge.predict(self.request)
        self.assertEqual(self.predictor.call_args.kwargs["observed_hand"], {"ice-golem", "skeletons"})
        self.assertEqual(result["hand_index"], 2)
        self.assertEqual(result["card"], "ice_golem")
        self.assertEqual(result["delay_ms"], 1250)

    def test_blue_and_red_policy_positions_are_mirrored_into_absolute_arena(self):
        blue = self.bridge.predict(self.request)
        red_request = dict(self.request, team="red")
        red_request["history"] = [dict(self.request["history"][0], team="blue")]
        red = self.bridge.predict(red_request)
        self.assertEqual((blue["x"], blue["y"]), (4500, 28800))
        self.assertEqual((red["x"], red["y"]), (4500, 3200))

    def test_confirmed_history_and_declared_decks_use_global_replay_frame(self):
        self.bridge.predict(self.request)
        args, kwargs = self.predictor.call_args
        battle = args[3]
        self.assertEqual(battle.events, ({"seconds": 1.0, "side": "opponent", "event_type": "card_play", "card": "knight", "x": 4500, "y": 23000},))
        self.assertEqual(battle.team_deck[0], "hog-rider")
        self.assertEqual(battle.opponent_deck[0], "knight")
        self.assertEqual(args[4], 1)
        self.assertEqual(kwargs["now"], 3.0)

    def test_future_event_is_rejected_even_when_no_card_is_affordable(self):
        request = copy.deepcopy(self.request)
        request["elixir"] = 0
        request["history"][0]["elapsed_ms"] = 4000
        with self.assertRaisesRegex(ValueError, "future events"):
            self.bridge.predict(request)
        self.predictor.assert_not_called()

    def test_no_affordable_card_waits_without_calling_policy(self):
        result = self.bridge.predict(dict(self.request, elixir=0))
        self.assertTrue(result["wait"])
        self.predictor.assert_not_called()

    def test_event_from_wrong_deck_is_rejected(self):
        request = copy.deepcopy(self.request)
        request["history"][0]["card"] = "hog_rider"
        with self.assertRaisesRegex(ValueError, "does not belong"):
            self.bridge.predict(request)

    def test_unknown_card_mapping_is_never_guessed(self):
        with self.assertRaisesRegex(ValueError, "No explicit policy card mapping"):
            self.bridge.card("Knight_EV1")

    def test_policy_cannot_return_a_card_outside_affordable_hand(self):
        self.predictor.return_value = dict(self.predictor.return_value, card="hog-rider")
        with self.assertRaisesRegex(RuntimeError, "affordable hand mask"):
            self.bridge.predict(self.request)

    def test_clock_wait_does_not_become_a_play(self):
        self.predictor.return_value = dict(self.predictor.return_value, wait_for_opponent=True, own_next_probability=.25)
        result = self.bridge.predict(self.request)
        self.assertTrue(result["wait"])
        self.assertNotIn("hand_index", result)

    def test_nonfinite_timing_is_rejected(self):
        self.predictor.return_value = dict(self.predictor.return_value, delay_seconds=float("nan"))
        with self.assertRaisesRegex(RuntimeError, "Nonfinite"):
            self.bridge.predict(self.request)

    def test_legacy_global_position_and_observed_hand_contract(self):
        result = self.bridge.predict(dict(self.request, model="v4.4.1"))
        self.assertEqual((result["x"], result["y"]), (4500, 28800))
        self.assertEqual(self.legacy_predictor.call_args.kwargs["prefer_cards"], {"ice-golem", "skeletons"})
        self.assertEqual(self.legacy_predictor.call_args.kwargs["min_context"], 0)

    def test_discovery_remains_json_without_importing_training_dependencies(self):
        result = subprocess.run([sys.executable, str(Path(policy_bridge.__file__)), "--list"], capture_output=True, text=True, check=True)
        self.assertEqual([m["id"] for m in json.loads(result.stdout)["models"]], ["v9", "v4.4.1", "v4.4.2"])


if __name__ == "__main__":
    unittest.main(verbosity=2)
