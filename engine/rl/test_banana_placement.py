"""Regression checks for requested aim leaking into watched policy history."""
from __future__ import annotations

import unittest

from .banana_placement import resolved_play_target


class ResolvedTargetContractTests(unittest.TestCase):
    def test_accepted_history_uses_resolved_coordinates(self):
        result = dict(accepted=True, resolved_target=dict(x=7499, y=17499))
        self.assertEqual(resolved_play_target(result), (7499, 17499))

    def test_old_binaries_cannot_silently_record_the_requested_aim(self):
        with self.assertRaisesRegex(RuntimeError, "rebuild native-watch"):
            resolved_play_target(dict(accepted=True))

    def test_rejected_or_malformed_results_cannot_enter_history(self):
        with self.assertRaises(ValueError):
            resolved_play_target(dict(accepted=False, resolved_target=dict(x=1, y=2)))
        for target in ({}, dict(x=True, y=3), dict(x=1, y=2.5), dict(x=-1, y=3),
                       dict(x=18000, y=3), dict(x=1, y=32000)):
            with self.subTest(target=target), self.assertRaises(ValueError):
                resolved_play_target(dict(accepted=True, resolved_target=target))


class NativeResolvedPlacementTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        from .banana_train import ROOT
        cls.root = ROOT
        cls.binary = ROOT / "native-engine/target/release/native-watch"
        cls.checkpoint = ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"
        if not cls.binary.is_file() or not cls.checkpoint.is_dir():
            raise unittest.SkipTest("native engine assets unavailable")

    def make_engine(self):
        from .banana_train import JsonlEngine
        engine = JsonlEngine(self.checkpoint, self.binary)
        self.addCleanup(engine.close)
        engine.request(dict(op="reset", deck_preset="log-bait", deck_preset_side1="hog26",
                            seed=630580545, max_ticks=7300))
        response = engine.request(dict(op="step", actions=[], ticks=120))
        return engine, response

    def test_resolution_matches_native_spawn_and_preserves_trajectory(self):
        queried, before = self.make_engine()
        reference, _ = self.make_engine()
        action = dict(side=1, slot=3, x=7594, y=889)
        self.assertEqual(before["observation"]["hands"][1][3]["name"], "Musketeer")
        for _ in range(3):
            self.assertEqual(queried.request(dict(op="resolve_target", **action)),
                             dict(valid=True, x=7499, y=17499))
        self.assertEqual(queried.request(dict(op="observe"))["observation"], before["observation"])
        corrected = queried.request(dict(op="step", actions=[action], ticks=22))
        original = reference.request(dict(op="step", actions=[action], ticks=22, raw_targets=True))
        self.assertEqual(resolved_play_target(corrected["actions"][0]), (7499, 17499))
        self.assertTrue(original["actions"][0]["accepted"])
        self.assertEqual(corrected["observation"], original["observation"])
        fields = ("native_entity_id", "row", "side", "x", "y", "hp", "max_hp")
        entities = lambda r: [{k: e.get(k) for k in fields} for e in r["frame"]["entities"]]
        self.assertEqual(entities(corrected), entities(original))
        troop = next(e for e in corrected["frame"]["entities"] if e["row"] == "Musketeer")
        self.assertEqual((troop["x"], troop["y"]), (7499, 17499))

    def test_barrel_keeps_enemy_side_targets_and_troops_resolve_for_both_sides(self):
        engine, before = self.make_engine()
        # Spells that can land on either half retain that choice; there is no
        # Goblin Barrel-specific rule forcing offense to hide a history bug.
        for y in (6222, 25778):
            target = engine.request(dict(op="resolve_target", side=0, slot=1, x=3656, y=y))
            self.assertTrue(target["valid"])
            self.assertEqual(target["y"] < 16000, y < 16000)
        knight = engine.request(dict(op="resolve_target", side=0, slot=2, x=3656, y=25778))
        self.assertTrue(knight["valid"])
        self.assertLess(knight["y"], 15000)
        musketeer = engine.request(dict(op="resolve_target", side=1, slot=3, x=7594, y=889))
        self.assertGreaterEqual(musketeer["y"], 17000)
        self.assertEqual(engine.request(dict(op="observe"))["observation"], before["observation"])

    def test_failing_barrel_opening_is_corrected_for_both_policy_perspectives(self):
        import torch
        from .banana_policy import BananaEpisode
        from .banana_timed_policy import CardPlan, PhoneStyleScheduler
        from .banana_watch import load_watched_policy
        weights = self.root / "runs/banana-1/base.pt"
        if not weights.is_file():
            self.skipTest("saved Banana 1 actor unavailable")
        torch.set_num_threads(2)
        policy = load_watched_policy(weights, "cpu", 630580545, "placement-regression")
        engine, before = self.make_engine()
        result = engine.request(dict(op="step", actions=[dict(side=1, slot=3, x=7594, y=889)], ticks=1))
        land_x, land_y = resolved_play_target(result["actions"][0])
        hog = ("hog-rider", "musketeer", "ice-golem", "skeletons", "ice-spirit", "cannon", "fireball", "the-log")
        bait = ("goblin-barrel", "princess", "knight", "goblin-gang", "inferno-tower", "rocket", "the-log", "ice-spirit")
        for side in (0, 1):
            obs = dict(before["observation"])
            if side == 1:
                obs["hands"] = obs["hands"][::-1]
                obs["elixir"] = obs["elixir"][::-1]
            for corrected in (False, True):
                with self.subTest(side=side, corrected=corrected):
                    episode = BananaEpisode((bait, hog) if side == 0 else (hog, bait))
                    episode.policy_seed = 630580545
                    x, y = (land_x, land_y) if corrected else (7594, 889)
                    episode.record_action(1 - side, "musketeer", x, y if side == 0 else 32000 - y, 1.)
                    scheduler = PhoneStyleScheduler([policy, policy], greedy=True)
                    action = scheduler._execute(side, CardPlan("goblin-barrel", 0, 1, 3., 1.5, 1.5),
                                                episode, obs, 1.5)
                    own_half = (action.y < 16000) == (side == 0)
                    self.assertEqual(own_half, not corrected)


if __name__ == "__main__":
    unittest.main()
