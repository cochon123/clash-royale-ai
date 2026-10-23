"""Async collection, staleness, and sync-mode transition equality."""
import tempfile
import time
import unittest
from collections import Counter, deque
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

import torch

from .async_collection import ContinuousCollector, policy_lag, within_policy_lag
from .banana_hog_league import Game, League
from .engine_pool import ThreadPoolEngine


class _Action:
    def __init__(self, side, mark):
        self.side = side
        self.wait = True
        self.card = "hog-rider"
        self.log_prob = torch.tensor(float(mark))
        self.value = torch.tensor(0.)
        self.behavior_version = None


class _Script:
    def __init__(self, delay, steps):
        self.delay, self.limit = delay, steps
        self.n = 0
        self.starts, self.ends, self.ops = [], [], []

    def request(self, payload, timeout=45):
        self.starts.append(time.perf_counter())
        self.ops.append(payload.get("op"))
        time.sleep(self.delay)
        self.n += 1
        self.ends.append(time.perf_counter())
        return {"observation": {"tick": 100 + self.n * 20, "winner_side": 0, "entity_count": 0},
                "actions": [], "terminated": self.n >= self.limit, "truncated": False,
                "ticks_advanced": payload.get("ticks", 0)}


def _league(clients, *, step_groups=1, pipeline=False, pool=False, lag=2, version=0):
    league = object.__new__(League)
    league.decks = {"hog-mirror": {"name": "Hog 2.6", "cards": ["hog-rider"] * 8}}
    league.hog = {"cards": ["hog-rider"] * 8}
    league.args = type("A", (), {})()
    league.args.workers = len(clients)
    league.args.step_groups = step_groups
    league.args.pipeline = pipeline
    league.args.max_policy_lag = lag
    league.args.async_collection = False
    league.args.async_ready_games = 0
    league.clients = clients
    league.executor = ThreadPoolExecutor(max_workers=max(2, len(clients)))
    league.engine_pool = ThreadPoolEngine(clients, league.executor) if pool else None
    league.worker_states = [{} for _ in clients]
    league.parent_policy = object()
    league.base = object()
    league.learner = object()
    league.fun = {"card_plays": {}, "streak": 0, "max_streak": 0, "quickest_win": None, "longest_game": None}
    league.counters = Counter()
    league.version = version
    league._collector = None
    league.stopping = lambda: False
    league.publish = lambda force=False: None
    league.log = lambda text: None
    league.recent = deque()
    league.train_rows = []
    league.sampler = type("S", (), {"record": staticmethod(lambda *_a, **_k: None)})()
    league.errors = Counter()
    directory = tempfile.TemporaryDirectory()
    league.run_dir = Path(directory.name)
    league._directory = directory
    league.reset = lambda worker, job: Game(worker, job, type("E", (), {"battle_id": f"{worker}-{job['seed']}", "events": []})(),
                                            {"tick": 100}, 100)

    def learner_actions(policy, requests, games, collecting):
        actions = []
        for game in games:
            action = _Action(game.job["seat"], game.steps)
            actions.append(action)
        return actions

    league.learner_actions = learner_actions
    league.opponent_actions = lambda policy, requests, games, controller: [_Action(1 - game.job["seat"], 100 + game.steps) for game in games]
    return league


def _jobs(n):
    return [{"deck_id": "hog-mirror", "controller": "bc-greedy", "seat": 0, "seed": i + 1} for i in range(n)]


def _signature(transitions):
    rows = []
    for action, outcome in transitions:
        rows.append((round(float(action.log_prob), 5), action.side, action.behavior_version, outcome))
    return sorted(rows)


class StalenessTests(unittest.TestCase):
    def test_lag_uses_the_oldest_behavior_version(self):
        old, new = _Action(0, 1), _Action(0, 2)
        old.behavior_version, new.behavior_version = 0, 3
        self.assertEqual(policy_lag([old, new], 3), 3)
        self.assertFalse(within_policy_lag([old, new], 3, 2))
        self.assertTrue(within_policy_lag([new], 3, 2))
        self.assertTrue(within_policy_lag([], 3, 2))


class SchedulerTests(unittest.TestCase):
    def test_finished_worker_resets_while_the_other_game_is_in_flight(self):
        fast, slow = _Script(0.01, 1), _Script(0.08, 1)
        league = _league([fast, slow])
        resets = []
        real_reset = league.reset

        def reset(worker, job):
            resets.append((worker, time.perf_counter(), job["seed"]))
            game = real_reset(worker, job)
            game.behavior_version = league.version
            return game

        league.reset = reset
        drawn = {"n": 0}

        def draw():
            drawn["n"] += 1
            return {"deck_id": "hog-mirror", "controller": "bc-greedy", "seat": 0, "seed": drawn["n"]}

        league.sampler.draw = draw
        collector = ContinuousCollector(league)
        try:
            rows, transitions = collector.collect(2)
        finally:
            collector.close()
            league.executor.shutdown(wait=True)
            league._directory.cleanup()
        self.assertEqual(sum(row["result"] != "VOID" for row in rows), 2)
        self.assertGreaterEqual(len(resets), 3)
        fast_second = next(stamp for worker, stamp, _seed in resets if worker == 0 and stamp != resets[0][1])
        self.assertLess(fast_second, slow.ends[0])
        self.assertTrue(all(action.behavior_version == 0 for action, _outcome in transitions))

    def test_stale_games_stay_in_the_log_and_leave_the_update(self):
        client = _Script(0.0, 2)
        league = _league([client], lag=2, version=0)
        league.sampler.draw = lambda: {"deck_id": "hog-mirror", "controller": "bc-greedy", "seat": 0, "seed": 7}
        stamps = {"n": 0}
        real_launch = league._launch_steps

        def launch(games, policy, collecting, step_jobs):
            if stamps["n"] == 0:
                league.version = 0
            else:
                league.version = 5
            stamps["n"] += 1
            return real_launch(games, policy, collecting, step_jobs)

        league._launch_steps = launch
        real_record = league.record

        def record(game, kind, result, error=None):
            row = real_record(game, kind, result, error)
            league.stopping = lambda: True
            return row

        league.record = record
        collector = ContinuousCollector(league)
        try:
            rows, transitions = collector.collect(1)
        finally:
            collector.close()
            league.executor.shutdown(wait=True)
            league._directory.cleanup()
        self.assertEqual(len(rows), 1)
        self.assertEqual(rows[0]["result"], "W")
        self.assertEqual(transitions, [])
        self.assertEqual(league.counters["stale_games"], 1)


class SyncEqualityTests(unittest.TestCase):
    def test_thread_pool_matches_direct_clients_for_both_schedulers(self):
        for step_groups in (1, 2):
            with self.subTest(step_groups=step_groups):
                direct = _league([_Script(0.0, 3), _Script(0.0, 2)], step_groups=step_groups, pool=False)
                pooled = _league([_Script(0.0, 3), _Script(0.0, 2)], step_groups=step_groups, pool=True)
                try:
                    rows_a, trans_a = direct.play(_jobs(2), direct.learner, "train", collecting=True)
                    rows_b, trans_b = pooled.play(_jobs(2), pooled.learner, "train", collecting=True)
                finally:
                    direct.executor.shutdown(wait=True, cancel_futures=True)
                    pooled.executor.shutdown(wait=True, cancel_futures=True)
                    direct._directory.cleanup()
                    pooled._directory.cleanup()
                self.assertEqual(sorted(row["result"] for row in rows_a), sorted(row["result"] for row in rows_b))
                self.assertEqual(_signature(trans_a), _signature(trans_b))
                self.assertTrue(trans_a)
                self.assertTrue(all(action.behavior_version == 0 for action, _outcome in trans_a))
                self.assertEqual([client.ops.count("step") for client in direct.clients],
                                 [client.ops.count("step") for client in pooled.clients])


if __name__ == "__main__":
    unittest.main()
