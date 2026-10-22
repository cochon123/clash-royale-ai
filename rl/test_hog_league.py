"""Invariants that can silently invalidate an overnight training experiment."""
import datetime as dt
import json
import tempfile
import time
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import Mock
from zoneinfo import ZoneInfo

from .banana_hog_league import HOLDOUTS, ROOT, League, Sampler, deadline_epoch, macro_score, metric, paired_delta


class OvernightInvariants(unittest.TestCase):
    def test_absolute_edmonton_deadline(self):
        at = dt.datetime.fromtimestamp(deadline_epoch("2026-10-20T11:00:00Z"), ZoneInfo("America/Edmonton"))
        self.assertEqual((at.year, at.month, at.day, at.hour, at.minute), (2026, 10, 20, 5, 0))
        with self.assertRaises(ValueError):
            deadline_epoch("2026-10-20T05:00:00")

    def test_heldouts_cannot_be_sampled_and_seats_alternate_per_cell(self):
        decks = json.loads((ROOT / "data/hog2_opponent_pool.json").read_text())["decks"]
        for deck in decks:
            deck["holdout"] = deck["id"] in HOLDOUTS
        sampler = Sampler(decks, 7)
        seen = {}
        for _ in range(2000):
            job = sampler.draw()
            self.assertNotIn(job["deck_id"], HOLDOUTS)
            self.assertLess(job["seed"], 1_000_000_000)
            cell = job["deck_id"], job["controller"]
            self.assertEqual(job["seat"], len(seen.get(cell, [])) % 2)
            seen.setdefault(cell, []).append(job["seat"])
            sampler.record(job, "L" if job["deck_id"] == "log-bait" else "W")
        probs = sampler.probabilities()
        self.assertAlmostEqual(sum(probs), 1.)
        self.assertGreaterEqual(min(probs), .75 / len(probs))
        self.assertLessEqual(max(probs), 2.5 / len(probs))
        self.assertEqual(len(seen), 27)

    def test_voids_are_not_draws_or_wins(self):
        result = metric(["W", "D", "L", "VOID", "VOID"])
        self.assertEqual(result["n"], 3)
        self.assertEqual(result["voids"], 2)
        self.assertEqual(result["score"], .5)
        self.assertIsNone(metric(["VOID"])["score"])

    def test_macro_does_not_reward_easy_cell_oversampling(self):
        rows = [{"deck_id": "easy", "controller": "bc", "result": "W"}] * 100
        rows += [{"deck_id": "hard", "controller": "bc", "result": "L"}]
        self.assertEqual(macro_score(rows), .5)

    def test_pairing_uses_deck_controller_seat_and_seed(self):
        row = {"deck_id": "a", "controller": "bc", "seat": 0, "seed": 123, "result": "L"}
        candidate = {**row, "result": "W"}
        self.assertEqual(paired_delta([candidate], [row])["delta"], 1.)
        self.assertEqual(paired_delta([{**candidate, "seat": 1}], [row])["pairs"], 0)
        self.assertEqual(paired_delta([{**candidate, "result": "VOID"}], [row])["pairs"], 0)
        self.assertIsNone(paired_delta([candidate], [row])["low"])

    def test_resume_after_confirmation_never_trains_or_reselects(self):
        # A crash during the final supervised wait must not expose confirmation
        # outcomes to another optimizer/selection cycle on restart.
        with tempfile.TemporaryDirectory() as directory:
            league = object.__new__(League)
            league.evaluation = {"confirmation": {"complete": True}}
            league.args = SimpleNamespace(max_waves=0)
            league.deadline = time.time() + 100
            league.run_dir = Path(directory)
            league.clients = []
            league.executor = Mock()
            league.stopping = Mock(side_effect=[False, False, True])
            league.play = Mock(side_effect=AssertionError("confirmation leaked into training"))
            league.evaluate = Mock(side_effect=AssertionError("confirmation reused for selection"))
            league.save = Mock()
            league.publish = Mock()
            league.run()
            league.play.assert_not_called()
            league.evaluate.assert_not_called()


if __name__ == "__main__":
    unittest.main()
