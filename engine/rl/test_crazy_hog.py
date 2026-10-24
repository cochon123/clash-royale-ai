"""Unit tests for the Hog 2.6 trainer pieces. No engine process is started."""
import json
import random
import tempfile
import unittest
from collections import Counter
from pathlib import Path

import torch

from .banana_fast_features import _CACHE_LOCK, _STREAM_CACHE
from .banana_hog_retrain import gae_returns
from .banana_policy import BananaEpisode
from .board_policy import DEFAULT_HOG2_WEIGHTS, BoardHogPolicy
from .crazy_hog_core import (
    PFSPSampler,
    assert_engine_pin,
    audit_game_rewards,
    budget_percent,
    capture_match_python,
    critical_moments,
    eta_seconds,
    format_eta,
    fresh_hog_stats,
    gae_with_bootstrap,
    hog_in_hand,
    restore_match_python,
    update_hog_stats,
    validate_crazy_resume,
)
from .crazy_hog_search_stub import ExpertIteration, SearchConfig, SearchTarget
from .crazy_hog_train import ReadyBatchCollector, TrainBoardPolicy
from .test_banana_tower_hp import state


DECK = ("hog-rider", "musketeer", "ice-golem", "skeletons", "ice-spirit", "cannon", "fireball", "the-log")


def _decks():
    return [
        {"id": "hog-mirror", "name": "Hog 2.6", "holdout": False},
        {"id": "pekka-bridge", "name": "PEKKA Bridge Spam", "holdout": False},
        {"id": "log-bait", "name": "Classic Log Bait", "holdout": False},
    ]


def _entity(row, side, x, y, hp=1000, max_hp=1000, entity_id=1):
    return {"id": entity_id, "row": row, "side": side, "x": x, "y": y, "hp": hp, "max_hp": max_hp,
            "dx": 0, "dy": 0, "first_seen_tick": 0}


class SamplerTests(unittest.TestCase):
    def test_hard_matchups_are_sampled_more_and_selfplay_stays_on_the_mirror(self):
        sampler = PFSPSampler(_decks(), ("bc-greedy", "bc-sampled", "hog2-greedy"), seed=3, power=2, floor=0.05)
        sampler.add_controller("selfplay-v1", ["hog-mirror", "pekka-bridge"])
        self.assertIn(("hog-mirror", "selfplay-v1"), sampler.cells)
        self.assertNotIn(("pekka-bridge", "selfplay-v1"), sampler.cells)
        for _ in range(8):
            sampler.record({"deck_id": "pekka-bridge", "controller": "bc-greedy"}, "L")
            sampler.record({"deck_id": "hog-mirror", "controller": "bc-greedy"}, "W")
        weights = {cell: sampler.weight(cell) for cell in sampler.cells}
        self.assertGreater(weights[("pekka-bridge", "bc-greedy")], weights[("hog-mirror", "bc-greedy")])
        probs = sampler.probabilities()
        self.assertAlmostEqual(sum(probs), 1.0)
        self.assertTrue(all(item > 0 for item in probs))
        jobs = [sampler.draw() for _ in range(len(sampler.cells))]
        self.assertEqual({(job["deck_id"], job["controller"]) for job in jobs}, set(sampler.cells))
        state = sampler.state_dict()
        restored = PFSPSampler.from_state_dict(state)
        self.assertEqual(restored.public_state()["cells"][0]["deck_id"], sampler.public_state()["cells"][0]["deck_id"])
        self.assertEqual(restored.draw()["deck_id"], sampler.draw()["deck_id"])
        sampler.remove_controller("selfplay-v1")
        self.assertTrue(all(cell[1] != "selfplay-v1" for cell in sampler.cells))
        logged = sampler.public_state()
        self.assertIn("pekka-bridge", json.dumps(logged))

    def test_floor_keeps_a_solved_matchup_in_the_distribution(self):
        sampler = PFSPSampler(_decks(), ("bc-greedy",), seed=1, power=2, floor=0.05)
        for _ in range(20):
            sampler.record({"deck_id": "hog-mirror", "controller": "bc-greedy"}, "W")
        self.assertGreater(sampler.weight(("hog-mirror", "bc-greedy")), 0.0)
        self.assertGreater(sampler.probabilities()[0], 0.0)


class RewardAndGaeTests(unittest.TestCase):
    def test_terminal_terms_telescope_and_truncated_terms_keep_the_last_potential(self):
        tower = [0.4, 0.1, -0.2]
        trade = [1.5, 0.5, 2.0]
        terminal = audit_game_rewards(tower, trade, 1.0, 0.5, 0.05, terminal=True)
        self.assertTrue(terminal["ok"])
        self.assertAlmostEqual(terminal["tower"]["sum_shaped"], 1.0 - 0.5 * 0.4)
        self.assertAlmostEqual(terminal["trade"]["sum_shaped"], -0.05 * 1.5)
        self.assertAlmostEqual(sum(terminal["rewards"]), 1.0 - 0.5 * 0.4 - 0.05 * 1.5)
        truncated = audit_game_rewards(tower, trade, 0.0, 0.5, 0.05, terminal=False)
        self.assertAlmostEqual(truncated["tower"]["sum_shaped"], 0.5 * (-0.2 - 0.4))
        self.assertAlmostEqual(truncated["trade"]["sum_shaped"], 0.05 * (2.0 - 1.5))
        with self.assertRaises(RuntimeError):
            audit_game_rewards(tower, trade, 1.0, 0.5, 0.05, terminal=False)
        with self.assertRaises(RuntimeError):
            audit_game_rewards(tower, trade, 1.0, 0.5, 0.05, terminal=True, tower_terms=[0.0, 0.0])

    def test_bootstrap_changes_only_the_truncated_tail(self):
        rewards, values = [0.1, -0.2, 0.0], [0.4, 0.3, 0.2]
        base_adv, base_ret = gae_with_bootstrap(rewards, values, 0.97, 0.0)
        old_adv, old_ret = gae_returns(rewards, values, 0.97)
        self.assertEqual(base_adv, old_adv)
        self.assertEqual(base_ret, old_ret)
        boot_adv, boot_ret = gae_with_bootstrap(rewards, values, 1.0, 0.5)
        self.assertNotEqual(boot_adv[-1], base_adv[-1])
        self.assertAlmostEqual(boot_adv[-1], rewards[-1] + 0.5 - values[-1])
        self.assertAlmostEqual(boot_ret[-1], boot_adv[-1] + values[-1])


class CriticalAndHogTests(unittest.TestCase):
    def test_critical_moments_fire_once_per_reason(self):
        board = {"tick": 10, "entities": [
            _entity("Pekka", 1, 8000, 14000, entity_id=4),
            _entity("PrincessTower", 0, 3500, 5000, hp=400, max_hp=1000, entity_id=2),
            _entity("HogRider", 0, 9000, 4000, entity_id=9),
        ]}
        observation = {"elixir": [5.0, 4.0], "hands": [[{"name": "HogRider", "slot": 0}], []]}
        seen_cross, seen_towers = set(), set()
        reasons = critical_moments(board, observation, 0, seen_cross=seen_cross, seen_towers=seen_towers,
                                   saw_double=False, saw_hog_hand=False, elapsed_seconds=10)
        self.assertTrue(any(item.startswith("bridge:Pekka") for item in reasons))
        self.assertTrue(any(item.startswith("tower:") for item in reasons))
        self.assertIn("hog-hand", reasons)
        self.assertTrue(hog_in_hand(observation, 0))
        from .crazy_hog_core import note_critical
        bucket = {"seen_cross": seen_cross, "seen_towers": seen_towers, "saw_double": False, "saw_hog_hand": False}
        note_critical(bucket, reasons)
        again = critical_moments(board, observation, 0, seen_cross=seen_cross, seen_towers=seen_towers,
                                 saw_double=bucket["saw_double"], saw_hog_hand=bucket["saw_hog_hand"],
                                 elapsed_seconds=130)
        self.assertEqual(again, ["double-elixir"])

    def test_hog_connects_on_tower_range_or_damage_within_eight_seconds(self):
        stats = fresh_hog_stats()
        far = {"entities": [_entity("HogRider", 0, 1000, 1000, entity_id=9),
                            _entity("PrincessTower", 1, 14000, 27000, hp=1000, max_hp=2000, entity_id=3)]}
        update_hog_stats(stats, far, 0, tick=100, new_events=[{"side": "team", "card": "hog-rider"}], elixir=10)
        self.assertEqual(stats["hog_plays"], 1)
        self.assertEqual(stats["hog_connects"], 0)
        self.assertGreater(stats["leak_seconds"], 0)
        near = {"entities": [_entity("HogRider", 0, 14000, 26000, entity_id=9),
                             _entity("PrincessTower", 1, 14500, 27000, hp=900, max_hp=2000, entity_id=3)]}
        update_hog_stats(stats, near, 0, tick=140, new_events=[], elixir=6)
        self.assertEqual(stats["hog_connects"], 1)
        late = fresh_hog_stats()
        update_hog_stats(late, far, 0, tick=0, new_events=[{"side": "team", "card": "hog-rider"}], elixir=3)
        update_hog_stats(late, {"entities": [_entity("PrincessTower", 1, 14000, 27000, hp=100, max_hp=2000, entity_id=3)]},
                         0, tick=400, new_events=[], elixir=3)
        self.assertEqual(late["hog_connects"], 0)


class SnapshotConsistencyTests(unittest.TestCase):
    def test_restore_then_replay_matches_observations_and_features(self):
        class Fold:
            def __init__(self):
                self.dataset = object()
                self.vocab = object()
                self.costs = {"hog-rider": 4}
                self.events = []
                self.token = 0

            def encode(self, events):
                if list(events)[:len(self.events)] != self.events:
                    raise RuntimeError("encoder prefix diverged")
                for event in events[len(self.events):]:
                    self.events.append(dict(event))
                    self.token += 1
                return self.token

        fold = Fold()
        policy = type("P", (), {"_dataset": fold.dataset, "vocab": fold.vocab, "costs": fold.costs, "_board_cache": {}})()
        episode = BananaEpisode((DECK, DECK), battle_id="snap-replay")
        key = (episode.battle_id, False, id(fold.dataset), id(fold.vocab), id(fold.costs))
        with _CACHE_LOCK:
            _STREAM_CACHE[key] = fold

        def feature():
            with _CACHE_LOCK:
                live = _STREAM_CACHE[key]
            return live.encode(episode.events)

        class Engine:
            def __init__(self):
                self.tick = 100
                self.actions = []
                self.snaps = {}
                self.ids = 1

            def request(self, request, timeout=90):
                op = request["op"]
                if op == "snapshot":
                    snap_id = self.ids
                    self.ids += 1
                    self.snaps[snap_id] = (self.tick, list(self.actions))
                    return {"snapshot_id": snap_id, "tick": self.tick, "bytes": 3}
                if op == "restore":
                    self.tick, self.actions = self.snaps[request["snapshot_id"]]
                    self.actions = list(self.actions)
                    return self.reply()
                if op == "step":
                    self.tick += int(request["ticks"])
                    self.actions.append(list(request.get("actions") or []))
                    return self.reply()
                if op == "drop_snapshot":
                    return {"snapshot_id": request["snapshot_id"]}
                raise AssertionError(op)

            def reply(self):
                return {"observation": {"tick": self.tick, "elixir": [5, 5], "winner_side": -1},
                        "board": {"tick": self.tick, "entities": [_entity("HogRider", 0, 1000, self.tick, entity_id=1)]},
                        "terminated": False, "truncated": False, "actions": [], "ticks_advanced": 20}

        engine = Engine()
        opening = engine.request({"op": "step", "ticks": 20, "actions": [{"side": 0, "slot": 0}]})
        episode.events.append({"seconds": 1.0, "side": "team", "event_type": "card_play", "card": "hog-rider", "x": 1, "y": 2})
        feature()
        python_state = capture_match_python(episode, [policy], {"obs": opening["observation"], "start_tick": 100,
                                                                "tower_potential": 0.2, "trade_potential": -0.4})
        taken = engine.request({"op": "snapshot"})
        continued_obs = []
        continued_features = []
        script = [[{"side": 0, "slot": 1}], [{"side": 1, "slot": 2}]]
        for actions in script:
            episode.events.append({"seconds": 2.0, "side": "opponent", "event_type": "card_play",
                                   "card": "musketeer", "x": 3, "y": actions[0]["slot"]})
            reply = engine.request({"op": "step", "ticks": 20, "actions": actions})
            continued_obs.append(reply["observation"]["tick"])
            continued_features.append(feature())
        self.assertNotEqual(continued_features[-1], python_state["encoders"][0][2].token)
        engine.request({"op": "restore", "snapshot_id": taken["snapshot_id"]})
        view = restore_match_python(episode, [policy], python_state)
        self.assertEqual(view["tower_potential"], 0.2)
        replay_obs = []
        replay_features = []
        for actions in script:
            episode.events.append({"seconds": 2.0, "side": "opponent", "event_type": "card_play",
                                   "card": "musketeer", "x": 3, "y": actions[0]["slot"]})
            reply = engine.request({"op": "step", "ticks": 20, "actions": actions})
            replay_obs.append(reply["observation"]["tick"])
            replay_features.append(feature())
        self.assertEqual(replay_obs, continued_obs)
        self.assertEqual(replay_features, continued_features)
        with _CACHE_LOCK:
            _STREAM_CACHE.pop(key, None)


class SearchStubTests(unittest.TestCase):
    def test_search_restores_the_engine_and_aux_loss_is_differentiable(self):
        class Pool:
            def __init__(self):
                self.tick = 7
                self.ops = []

            def call(self, slot, request, timeout=90):
                self.ops.append(request["op"])
                if request["op"] == "snapshot":
                    self.saved = self.tick
                    return {"snapshot_id": 4, "tick": self.tick, "bytes": 1}
                if request["op"] == "restore":
                    self.tick = self.saved
                    return {"observation": {"tick": self.tick}}
                if request["op"] == "drop_snapshot":
                    return {"snapshot_id": request["snapshot_id"]}
                raise AssertionError(request["op"])

        pool = Pool()
        episode = BananaEpisode((DECK, DECK), battle_id="search")
        episode.events.append({"seconds": 0.0, "side": "team", "event_type": "card_play", "card": "hog-rider", "x": 1, "y": 1})
        expert = ExpertIteration(SearchConfig(search_fraction=1.0), row_table=None)
        self.assertTrue(expert.should_search({"tick": 1}, {"entities": []}, 0, random.Random(0)))
        target = expert.search(pool, 0, policy=None, episode=episode, side=0, observation={}, board={"entities": []})
        self.assertEqual(pool.ops, ["snapshot", "restore", "drop_snapshot"])
        self.assertEqual(pool.tick, 7)
        self.assertEqual(len(episode.events), 1)
        self.assertIsInstance(target, SearchTarget)


class ResumeAndEtaTests(unittest.TestCase):
    def test_resume_guard_and_eta(self):
        args = type("A", (), {"seed": 1, "workers": 4, "trade_coef": 0.05, "search": "none"})()
        saved = {"format": "crazy-hog-v1", "args": {"seed": 1, "workers": 4, "trade_coef": 0.05, "search": "none"},
                 "crazy_train_sha256": "abc"}
        validate_crazy_resume(saved, args, {"crazy_train": "abc"})
        with self.assertRaises(ValueError):
            validate_crazy_resume(saved, args, {"crazy_train": "def"})
        changed = type("A", (), {"seed": 1, "workers": 8, "trade_coef": 0.05, "search": "none"})()
        with self.assertRaises(ValueError):
            validate_crazy_resume(saved, changed, {"crazy_train": "abc"})
        with self.assertRaises(RuntimeError):
            assert_engine_pin("deadbeef")
        assert_engine_pin("32b6deb2f5116ebc")
        assert_engine_pin("nope", allow=True)
        self.assertEqual(eta_seconds(150, 100, 200, "running"), 50)
        self.assertEqual(eta_seconds(250, 100, 200, "running"), 0)
        self.assertEqual(eta_seconds(150, 100, 200, "completed"), 0)
        self.assertAlmostEqual(budget_percent(150, 100, 200), 50.0)
        self.assertEqual(format_eta(3661), "01:01:01")


class SchedulerTests(unittest.TestCase):
    def test_waiting_games_share_one_learner_call(self):
        calls = []

        class League:
            def __init__(self):
                self.counters = Counter()
                self.args = type("A", (), {"workers": 2, "step_groups": 1, "max_policy_lag": 2})()
                self.stopping = lambda: False
                self.ready_jobs = [
                    {"deck_id": "hog-mirror", "controller": "bc-greedy", "seat": 0, "seed": 1},
                    {"deck_id": "pekka-bridge", "controller": "hog2-greedy", "seat": 1, "seed": 2},
                ]

            def draw_job(self, worker):
                return self.ready_jobs[worker]

            def _launch_steps(self, games, policy, collecting, step_jobs):
                calls.append([game.worker for game in games])
                for game in games:
                    future = type("F", (), {"result": lambda self: {"observation": {"tick": 200, "winner_side": 0},
                                                                    "actions": [], "terminated": True, "truncated": False}})()
                    step_jobs[future] = game

        # The collector reaches _launch_one_group once both resets have landed.
        from .banana_hog_league import Game
        league = League()
        league.learner = object()
        league.decks = {"hog-mirror": {"name": "Hog"}, "pekka-bridge": {"name": "PEKKA"}}
        league.worker_states = [{}, {}]
        league.publish = lambda force=False: None
        league.log = lambda text: None
        from concurrent.futures import ThreadPoolExecutor
        league.executor = ThreadPoolExecutor(max_workers=2)

        def reset(worker, job):
            episode = type("E", (), {"battle_id": str(worker), "events": []})()
            return Game(worker, job, episode, {"tick": 100}, 100)

        league.reset = reset
        league._consume_step = lambda game, future, kind, collecting, active, completed, transitions: (
            completed.append({"result": "W"}) or active.pop(game.worker, None) or False)
        collector = ReadyBatchCollector(league)
        # Drive one scheduling round by calling the pieces directly.
        collector._start_idle()
        import concurrent.futures as futures
        futures.wait(collector.reset_jobs)
        collector._drain_resets([])
        self.assertEqual(len(collector.ready), 2)
        collector._launch_one_group()
        self.assertEqual(len(calls), 1)
        self.assertEqual(set(calls[0]), {0, 1})
        league.executor.shutdown(wait=True)


class ModelTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        torch.set_num_threads(2)
        if not DEFAULT_HOG2_WEIGHTS.is_file():
            raise unittest.SkipTest(f"missing {DEFAULT_HOG2_WEIGHTS}")
        cls.policy = TrainBoardPolicy(device="cpu")
        cls.parent = __import__("rl.banana_hog_adaptive", fromlist=["AdaptiveHogPolicy"]).AdaptiveHogPolicy(
            device="cpu", weights=DEFAULT_HOG2_WEIGHTS)

    def _observation(self, side, tick):
        from .test_board_policy import sample_board
        hands = [[dict(slot=i, name=card, cost=cost, available=True)
                  for i, (card, cost) in enumerate(zip(DECK[:4], [4, 4, 2, 1]))]] * 2
        return dict(hands=hands, elixir=[10.0, 9.0], tower_state=state(), board=sample_board(side, tick=tick))

    def test_parent_parity_keys_and_disjoint_optimizer_groups(self):
        data = self.policy.state_dict()
        for name, value in self.parent.state_dict().items():
            self.assertIn(name, data)
            self.assertTrue(torch.equal(data[name], value), name)
        league = object.__new__(__import__("rl.crazy_hog_train", fromlist=["CrazyHogLeague"]).CrazyHogLeague)
        league.learner = self.policy
        league.args = type("A", (), {"learning_rate": 3e-5, "backbone_learning_rate": 3e-6,
                                    "critic_learning_rate": 3e-4, "board_learning_rate": 3e-4, "trainable": "all"})()
        optimizer = league.make_optimizer()
        seen = []
        roles = []
        for group in optimizer.param_groups:
            roles.append(group["role"])
            for parameter in group["params"]:
                self.assertNotIn(id(parameter), seen)
                seen.append(id(parameter))
        self.assertIn("board", roles)
        self.assertIn("critic", roles)
        self.assertIn("actor", roles)

    def test_batched_act_matches_unbatched_act(self):
        requests = []
        for side in (0, 1):
            episode = BananaEpisode((DECK, DECK), battle_id=f"batch-{side}-{random.randrange(1, 10**9)}")
            episode.policy_seed = 11 + side
            requests.append((episode, self._observation(side, 1400 + side), side, 2.0))
        self.policy.clear_boards()
        batched = self.policy.act_batch(requests, deterministic=True, placement_top_k=576)
        singles = []
        for episode, observation, side, now in requests:
            self.policy.observe_board(episode, observation["board"], side)
            singles.append(self.policy.act(episode, observation, side, now, deterministic=True, placement_top_k=576))
        for left, right in zip(batched, singles):
            self.assertEqual((left.wait, left.card, left.tile, left.x, left.y, left.slot),
                             (right.wait, right.card, right.tile, right.x, right.y, right.slot))
            torch.testing.assert_close(left.log_prob, right.log_prob, rtol=1e-5, atol=1e-6)
            torch.testing.assert_close(left.value, right.value, rtol=1e-5, atol=1e-6)
            self.assertIn("board", left.input_cache)

    def test_real_encoder_replay_matches_after_restore(self):
        episode = BananaEpisode((DECK, DECK), battle_id=f"enc-{random.randrange(1, 10**9)}")
        observation = self._observation(0, 1500)
        self.policy._inputs(episode, 0, 1.0, observation["hands"][0], 10.0, 9.0)
        before = capture_match_python(episode, [self.policy], {"obs": observation, "start_tick": 100,
                                                               "tower_potential": 0.0, "trade_potential": 0.0})
        episode.events.append({"seconds": 1.0, "side": "team", "event_type": "card_play", "card": "hog-rider", "x": 9, "y": 8})
        continued = self.policy._inputs(episode, 0, 2.0, observation["hands"][0], 6.0, 9.0)
        continued_tensors = [tensor.detach().cpu().clone() for tensor in continued[0]]
        episode.events.append({"seconds": 3.0, "side": "opponent", "event_type": "card_play", "card": "musketeer", "x": 4, "y": 5})
        self.policy._inputs(episode, 0, 3.0, observation["hands"][0], 6.0, 5.0)
        restore_match_python(episode, [self.policy], before)
        episode.events.append({"seconds": 1.0, "side": "team", "event_type": "card_play", "card": "hog-rider", "x": 9, "y": 8})
        replay = self.policy._inputs(episode, 0, 2.0, observation["hands"][0], 6.0, 9.0)
        for expected, actual in zip(continued_tensors, replay[0]):
            torch.testing.assert_close(expected, actual.detach().cpu(), rtol=0, atol=0)

    def test_stub_aux_loss_backpropagates_into_the_wait_head(self):
        episode = BananaEpisode((DECK, DECK), battle_id=f"aux-{random.randrange(1, 10**9)}")
        observation = self._observation(0, 1600)
        self.policy.observe_board(episode, observation["board"], 0)
        action = self.policy.act_batch([(episode, observation, 0, 1.0)], deterministic=True, placement_top_k=576)[0]
        target = SearchTarget(side=0, wait_prob=0.2, cache=action.input_cache)
        expert = ExpertIteration(SearchConfig(aux_coef=0.5), self.policy.row_table)
        self.policy.zero_grad(set_to_none=True)
        loss = expert.aux_loss(self.policy, [target])
        loss.backward()
        self.assertIsNotNone(self.policy.wait_head.weight.grad)
        self.assertGreater(float(self.policy.wait_head.weight.grad.abs().sum()), 0)

    def test_training_checkpoint_loads_for_inference_with_board_and_full_critic(self):
        payload = self.policy.checkpoint_payload()
        payload["format"] = "crazy-hog-ppo-v1"
        payload["critic_state"] = self.policy.critic.state_dict()
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "final.pt"
            torch.save(payload, path)
            loaded = BoardHogPolicy.from_checkpoint(path, device="cpu")
        for name in ("model", "wait_head", "critic", "board_encoder", "context_residual", "raster"):
            expected = getattr(self.policy, name).state_dict()
            actual = getattr(loaded, name).state_dict()
            self.assertEqual(set(expected), set(actual))
            for key in expected:
                torch.testing.assert_close(expected[key], actual[key], rtol=0, atol=0)


class ReportAndMonitorTests(unittest.TestCase):
    def test_report_and_monitor_math_on_a_fake_run(self):
        from .crazy_hog_monitor import render_row
        from .crazy_hog_report import build_report
        with tempfile.TemporaryDirectory() as directory:
            run = Path(directory)
            (run / "notes.md").write_text("## Lessons\n\nPEKKA still needs more games.\n")
            (run / "manifest.json").write_text(json.dumps({
                "name": "crazy-hog-2.6", "format": "crazy-hog-v1", "deadline": "2026-10-23T20:00:00+00:00",
                "engine": {"sha256": "32b6deb2abcd", "path": "/engine"},
                "parent": {"sha256": "parent", "path": "/parent", "name": "banana-hog-2"},
                "args": {"workers": 4, "device": "cpu"},
            }))
            (run / "state.json").write_text(json.dumps({
                "name": "crazy-hog-2.6", "status": "completed", "phase": "done",
                "started_at": "2026-10-23T18:00:00+00:00", "deadline": "2026-10-23T21:00:00+00:00",
                "elapsed_seconds": 600, "duration_seconds": 10800, "device": "cpu", "workers": [],
                "counters": {"games": 4, "decisions": 400, "updates": 1, "wins": 1, "losses": 3, "draws": 0,
                             "bank_episodes": 1},
                "progress": {"games": 4, "decisions": 400, "updates": 1, "bank_episodes": 1, "episodes": 5,
                             "budget_percent": 5.5, "eta_seconds": 10200},
                "rates": {"games_per_minute": 0.4, "decisions_per_second": 0.6},
                "hog": {"hog_plays_per_game": 2.0, "connect_rate": 0.5, "elixir_leak_seconds": 3.0},
                "ppo": {"loss": 0.2, "approx_kl": 0.01, "entropy": 1.2, "parent_kl": 0.02, "aux_loss": 0.0},
                "parent": {"name": "banana-hog-2", "sha256": "abc"},
                "engine": {"sha256": "32b6deb2abcd"},
            }))
            (run / "updates.jsonl").write_text(json.dumps({
                "at": "2026-10-23T18:05:00+00:00", "updates": 1, "loss": 0.2, "policy_loss": 0.1,
                "value_loss": 0.05, "entropy": 1.2, "approx_kl": 0.01, "parent_kl": 0.02, "aux_loss": 0.0,
            }) + "\n")
            (run / "matches.jsonl").write_text("\n".join([
                json.dumps({"at": "2026-10-23T18:02:00+00:00", "kind": "train", "deck_id": "pekka-bridge",
                            "deck_name": "PEKKA Bridge Spam", "result": "L", "counts_for_winrate": True,
                            "hog_plays": 1, "plays": 10, "connect_rate": 0.0, "leak_seconds": 4, "bank": False,
                            "seconds": 30}),
                json.dumps({"at": "2026-10-23T18:03:00+00:00", "kind": "train", "deck_id": "hog-mirror",
                            "deck_name": "Hog 2.6", "result": "W", "counts_for_winrate": True,
                            "hog_plays": 3, "plays": 12, "connect_rate": 0.66, "leak_seconds": 1, "bank": False,
                            "seconds": 40}),
                json.dumps({"at": "2026-10-23T18:04:00+00:00", "kind": "train", "deck_id": "hog-mirror",
                            "deck_name": "Hog 2.6", "result": "BANK", "counts_for_winrate": False,
                            "hog_plays": 1, "plays": 4, "connect_rate": 1.0, "leak_seconds": 0, "bank": True,
                            "seconds": 10}),
            ]) + "\n")
            (run / "reward-audit.jsonl").write_text(json.dumps({
                "at": "2026-10-23T18:03:00+00:00", "ok": True, "terminal": True,
                "trade": {"sum_shaped": -0.02}, "tower": {"sum_shaped": 0.3},
            }) + "\n")
            (run / "metrics.jsonl").write_text(json.dumps({
                "at": "2026-10-23T18:05:00+00:00", "elapsed_seconds": 300, "decisions_per_second": 1.2,
                "games_per_minute": 0.8, "episodes_per_minute": 1.0,
            }) + "\n")
            target = run / "report.html"
            build_report(run, target)
            html = target.read_text()
            self.assertIn("crazy-hog-2.6", html)
            self.assertIn("PEKKA", html)
            self.assertIn("Lessons", html)
            self.assertIn("<svg", html)
            self.assertNotIn("matplotlib", html)
            self.assertNotIn("http://", html)
            self.assertNotIn("https://", html)
            row = render_row({
                "status": "running", "started_at": "2026-10-23T18:00:00+00:00",
                "deadline": "2026-10-23T21:00:00+00:00",
                "progress": {"games": 4, "bank_episodes": 1, "episodes": 5, "decisions": 400, "updates": 1,
                             "budget_percent": 5.5, "eta_seconds": 10200},
                "counters": {"games": 4, "decisions": 400, "updates": 1},
                "rates": {"games_per_minute": 0.4, "decisions_per_second": 0.66},
                "hog": {"hog_plays_per_game": 2.0, "connect_rate": 0.5},
            })
            self.assertIn("02:50:00", row)
            self.assertIn("5.5%", row)
            self.assertIn("400", row)


if __name__ == "__main__":
    unittest.main()
