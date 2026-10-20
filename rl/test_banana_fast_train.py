"""Scheduling checks independent of game speed or neural checkpoint weights."""
from concurrent.futures import ThreadPoolExecutor
from collections import Counter
from types import SimpleNamespace
import threading
import unittest
from unittest.mock import patch

from rl.banana_fast_train import FastBananaTrainer, default_workers


class PipelineTests(unittest.TestCase):
    def test_cpu_step_starts_before_other_specialist_inference_and_preserves_seats(self):
        dispatched = threading.Event()
        order = []

        class Policy:
            def __init__(self, name):
                self.name = name

            def act_batch(self, requests, deterministic):
                order.append(self.name)
                if self.name == "mortar":
                    if not dispatched.wait(timeout=5):
                        raise AssertionError("Hog CPU engine was not dispatched before Mortar inference")
                return [SimpleNamespace(side=row[2], policy=self.name) for row in requests]

        runner = FastBananaTrainer.__new__(FastBananaTrainer)
        runner.core = SimpleNamespace(policies={key: Policy(key) for key in ("hog", "mortar")},
                                      baseline=Policy("baseline"))
        runner.timings = {"inference": 0.}
        runner.timing_counts = Counter()
        runner.pipeline_rounds = 0
        games = [SimpleNamespace(worker_id=i, key=key, observation={"tick": 120},
                                 start_tick=100, specialist_side=i, episode=object())
                 for i, key in enumerate(("hog", "mortar"))]

        def step(game, actions):
            if game.key == "hog":
                dispatched.set()
            return game.worker_id, actions

        runner._step_one = step
        with ThreadPoolExecutor(max_workers=2) as runner.executor:
            actions, futures = runner._dispatch_round(games)
            result = dict(f.result() for f in futures)
        self.assertEqual(order, ["baseline", "hog", "mortar"])
        self.assertEqual(result, actions)
        self.assertEqual([a.policy for a in actions[0]], ["hog", "baseline"])
        self.assertEqual([a.policy for a in actions[1]], ["baseline", "mortar"])
        for pair in actions.values():
            self.assertEqual([a.side for a in pair], [0, 1])
        self.assertEqual(runner.pipeline_rounds, 1)

    def test_worker_default_respects_cpu_affinity(self):
        for cpus, expected in ((32, 16), (12, 10), (4, 2), (2, 1), (1, 1)):
            with self.subTest(cpus=cpus), patch("rl.banana_fast_train.os.sched_getaffinity", return_value=set(range(cpus))):
                self.assertEqual(default_workers(), expected)


if __name__ == "__main__":
    unittest.main()
