import json
import unittest
from pathlib import Path
from unittest.mock import Mock

import torch

from .banana_hog_overnight import OvernightLeague
from .banana_hog_adaptive import AdaptiveHogPolicy
from .banana_train import ROOT


class OvernightTests(unittest.TestCase):
    def test_every_archetype_and_controller_is_sampled_and_seats_alternate(self):
        league = object.__new__(OvernightLeague)
        decks = json.loads((ROOT / "data/hog2_opponent_pool.json").read_text())["decks"]
        for deck in decks:
            deck["holdout"] = True
        sampler = league.make_sampler(decks, 21)
        self.assertFalse(any(deck["holdout"] for deck in decks))
        jobs = [sampler.draw() for _ in range(33)]
        self.assertEqual({(j["deck_id"], j["controller"]) for j in jobs}, set(sampler.cells))
        seats = {}
        for job in jobs + [sampler.draw() for _ in range(200)]:
            cell = job["deck_id"], job["controller"]
            self.assertEqual(job["seat"], seats.get(cell, 0) % 2)
            self.assertLess(job["seed"], 1_000_000_000)
            seats[cell] = seats.get(cell, 0) + 1

    def test_hp_parent_loads_without_losing_a_single_saved_weight(self):
        torch.set_num_threads(2)
        parent = ROOT / "runs/banana-hog-1/models/final.pt"
        saved = torch.load(parent, map_location="cpu", weights_only=True)
        league = object.__new__(OvernightLeague)
        policy = league.make_parent_policy(device="cpu", weights=parent)
        learner = AdaptiveHogPolicy(device="cpu", weights=parent)
        self.assertTrue(policy.tower_hp_enabled)
        for name, value in policy.model.state_dict().items():
            torch.testing.assert_close(value, saved["model_state"][name], rtol=0, atol=0)
        for name, value in policy.state_dict().items():
            torch.testing.assert_close(value, learner.state_dict()[name], rtol=0, atol=0)


if __name__ == "__main__":
    unittest.main()
