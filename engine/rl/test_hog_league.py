"""Invariants that can silently invalidate an overnight training experiment."""
import datetime as dt
import json
import tempfile
import time
import unittest
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import Mock, patch
from zoneinfo import ZoneInfo

from .banana_hog_league import HOLDOUTS, ROOT, Game, League, Sampler, deadline_epoch, macro_score, metric, paired_delta


class OvernightInvariants(unittest.TestCase):
    def test_step_groups_cli_defaults_and_validation(self):
        from contextlib import redirect_stderr
        from io import StringIO
        from . import banana_hog_league as core, banana_hog_retrain as retrain

        for entry in (core.main, retrain.main):
            for value in (None, 3, 0, -1):
                with self.subTest(entry=entry.__module__, step_groups=value):
                    argv = ["trainer", "--run-dir", "/tmp/cr-prof/overlap/unused-cli-test",
                            "--engine", "/unused", "--deadline", "2099-01-01T00:00:00Z"]
                    if value is not None:
                        argv.extend(["--step-groups", str(value)])
                    with patch("sys.argv", argv), patch.object(core, "League") as constructor, \
                            patch.object(core.signal, "signal"), redirect_stderr(StringIO()):
                        call = (lambda: entry(constructor)) if entry is retrain.main else entry
                        if value is not None and value <= 0:
                            with self.assertRaises(SystemExit) as error:
                                call()
                            self.assertEqual(error.exception.code, 2)
                            constructor.assert_not_called()
                        else:
                            call()
                            self.assertEqual(constructor.call_args.args[0].step_groups, value or 2)

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


class _Wait:
    def __init__(self, side):
        self.side = side
        self.wait = True
        self.card = "hog-rider"


class _ClockClient:
    def __init__(self, delay, steps):
        self.delay = delay
        self.limit = steps
        self.n = 0
        self.starts = []
        self.ends = []

    def request(self, payload, timeout=45):
        self.starts.append(time.perf_counter())
        time.sleep(self.delay)
        self.n += 1
        self.ends.append(time.perf_counter())
        return {"observation": {"tick": 100 + self.n * 20, "winner_side": 0, "entity_count": 1},
                "actions": [], "terminated": self.n >= self.limit, "truncated": False}


def _scheduling_league(pipeline):
    league = object.__new__(League)
    fast, slow = _ClockClient(0.04, 2), _ClockClient(0.25, 1)
    league.decks = {"hog-mirror": {"name": "Hog 2.6"}}
    league.args = SimpleNamespace(workers=2, pipeline=pipeline, step_groups=1)
    league.clients = [fast, slow]
    league.executor = ThreadPoolExecutor(max_workers=2)
    league.worker_states = [{}, {}]
    league.parent_policy = object()
    league.base = object()
    league.fun = {"card_plays": {}}
    league.counters = __import__("collections").Counter()
    league.infer_at = []
    league.stopping = lambda: False
    league.publish = lambda force=False: None
    league.reset = lambda worker, job: Game(worker, job, object(), {"tick": 100}, 100)
    league.record = lambda game, kind, result, error=None: {"result": result, "worker": game.worker}

    def learner_actions(policy, requests, games, collecting):
        league.infer_at.append(time.perf_counter())
        return [_Wait(game.job["seat"]) for game in games]

    league.learner_actions = learner_actions
    league.opponent_actions = lambda policy, requests, games, controller: [_Wait(1 - game.job["seat"]) for game in games]
    return league, fast, slow


class PipelineScheduling(unittest.TestCase):
    def test_ready_game_is_not_held_for_the_slow_engine(self):
        league, fast, slow = _scheduling_league(pipeline=True)
        jobs = [{"deck_id": "hog-mirror", "controller": "bc-greedy", "seat": 0, "seed": 1},
                {"deck_id": "hog-mirror", "controller": "bc-greedy", "seat": 0, "seed": 2}]
        try:
            rows, _ = league.play(jobs, object(), "train")
        finally:
            league.executor.shutdown(wait=True)
        self.assertEqual(sorted(row["result"] for row in rows), ["W", "W"])
        self.assertLess(fast.starts[1], slow.ends[0])

    def test_barrier_holds_the_fast_game_until_the_slow_one_returns(self):
        league, fast, slow = _scheduling_league(pipeline=False)
        jobs = [{"deck_id": "hog-mirror", "controller": "bc-greedy", "seat": 0, "seed": 1},
                {"deck_id": "hog-mirror", "controller": "bc-greedy", "seat": 0, "seed": 2}]
        try:
            league.play(jobs, object(), "train")
        finally:
            league.executor.shutdown(wait=True)
        self.assertGreaterEqual(fast.starts[1], slow.ends[0])


if __name__ == "__main__":
    unittest.main()
