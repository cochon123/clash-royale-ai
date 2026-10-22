"""Per-row selection, batched masks and policy-object grouping contracts."""
from concurrent.futures import ThreadPoolExecutor, wait
from pathlib import Path
from threading import Barrier, Event, Lock
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch

import torch
from torch.distributions import Bernoulli, Categorical

from .banana_fast_policy import FastBananaPolicy
from .banana_hog_adaptive import AdaptiveLeague, SamplingPolicy
from .banana_hog_league import League
from .test_banana_fast_policy import episode_with_prefix, observation


class PerRowInferenceTests(unittest.TestCase):
    device = "cpu"

    @classmethod
    def setUpClass(cls):
        torch.set_num_threads(2)
        weights = Path(__file__).resolve().parents[1] / "runs/banana-1/base.pt"
        cls.policy = SamplingPolicy(device=cls.device, weights=weights)
        cls.policy.eval()
        with torch.no_grad():
            cls.policy.wait_head.weight.zero_()
            cls.policy.wait_head.bias.fill_(-1.)

    def requests(self):
        requests = []
        for i, n in enumerate((1, 2, 4, 8, 16, 32, 63, 65, 3, 5, 7, 0)):
            ep = episode_with_prefix(n)
            ep.policy_seed = 99 + i
            obs = observation(self.policy)
            if i == 0:
                obs["elixir"] = [0., 0.]
            requests.append((ep, obs, i % 2, 90. + i))
        return requests

    def assert_actions_equal(self, expected, actual):
        self.assertEqual(len(expected), len(actual))
        for old, new in zip(expected, actual):
            self.assertEqual((old.wait, old.side, old.slot, old.deck_slot, old.card, old.tile, old.x, old.y),
                             (new.wait, new.side, new.slot, new.deck_slot, new.card, new.tile, new.x, new.y))
            for key in ("log_prob", "entropy", "value"):
                torch.testing.assert_close(getattr(old, key), getattr(new, key), rtol=1e-4, atol=1e-5)
            self.assertEqual(old.input_cache.keys(), new.input_cache.keys())
            for key, value in old.input_cache.items():
                other = new.input_cache[key]
                if isinstance(value, torch.Tensor):
                    torch.testing.assert_close(value, other, rtol=0, atol=0)
                elif key == "inputs":
                    for a, b in zip(value, other):
                        torch.testing.assert_close(a, b, rtol=0, atol=0)
                else:
                    self.assertEqual(value, other, key)

    def test_merged_modes_equal_separate_controller_calls(self):
        requests = self.requests()
        flags = [i % 3 == 0 for i in range(len(requests))]
        for top_k in (5, 576):
            with self.subTest(top_k=top_k):
                expected = [None] * len(requests)
                for mode in (True, False):
                    ids = [i for i, flag in enumerate(flags) if flag == mode]
                    rows = self.policy.act_batch([requests[i] for i in ids], deterministic=mode,
                                                 placement_top_k=top_k)
                    for i, row in zip(ids, rows):
                        expected[i] = row
                actual = self.policy.act_batch(requests, deterministic=flags, placement_top_k=top_k)
                self.assert_actions_equal(expected, actual)
                self.assertTrue(any(a.wait for a in actual))
                self.assertTrue(any(not a.wait for a in actual))
                for mode in (True, False):
                    self.assert_actions_equal(
                        self.policy.act_batch(requests, deterministic=mode, placement_top_k=top_k),
                        self.policy.act_batch(requests, deterministic=[mode]*len(requests), placement_top_k=top_k))
        with self.assertRaisesRegex(ValueError, "deterministic must match"):
            self.policy.act_batch(requests, deterministic=[True])
        self.assertEqual(self.policy.act_batch([], deterministic=[]), [])

    def test_batched_masks_equal_sequential_and_intersect_supplied_masks(self):
        requests = self.requests()
        flags = [i % 3 == 0 for i in range(len(requests))]
        supplied = torch.zeros(576, dtype=torch.bool)
        supplied[100:400] = True
        def query(request, slot):
            mask = torch.zeros(576, dtype=torch.bool)
            mask[90 + slot:110 + slot] = True
            return mask
        calls = []
        def batch(pairs):
            calls.append(pairs)
            return [query(request, slot) for request, slot in pairs]
        provider = Mock(side_effect=AssertionError("single-row provider must not be called"))
        provider.batch = batch
        kwargs = dict(deterministic=flags, placement_top_k=576, legal_tile_masks=[supplied]*len(requests))
        sequential = self.policy.act_batch(requests, placement_mask_provider=query, **kwargs)
        batched = self.policy.act_batch(requests, placement_mask_provider=provider, **kwargs)
        self.assert_actions_equal(sequential, batched)
        # Supplying the same masks ahead of time uses the original two-sync
        # decode path. This checks the joint wait/slot transfer independently.
        combined = [supplied if a.wait else supplied & query(r, a.slot)
                    for r, a in zip(requests, sequential)]
        precomputed = self.policy.act_batch(requests, deterministic=flags, placement_top_k=576,
                                            legal_tile_masks=combined)
        self.assert_actions_equal(precomputed, batched)
        self.assertEqual(len(calls), 1)
        self.assertEqual(len(calls[0]), sum(not a.wait for a in batched))
        for action in batched:
            if not action.wait:
                self.assertTrue(supplied[action.tile])
                self.assertTrue(all(supplied[tile] for tile in action.input_cache["tile_candidates"]))
        invalid = [lambda pairs: [], lambda pairs: [[True]*575 for _ in pairs],
                   lambda pairs: [[False]*576 for _ in pairs]]
        for batch in invalid:
            provider.batch = batch
            with self.assertRaisesRegex(ValueError, "native placement mask"):
                self.policy.act_batch(requests, placement_mask_provider=provider, **kwargs)
        provider.batch = lambda pairs: [torch.arange(576) < 50 for _ in pairs]
        with self.assertRaisesRegex(ValueError, "no legal locations"):
            self.policy.act_batch(requests, placement_mask_provider=provider, **kwargs)
        empty = observation(self.policy)
        empty["elixir"] = [0., 0.]
        provider.batch = Mock(side_effect=AssertionError("wait-only rows must not query masks"))
        self.policy.act_batch([(requests[0][0], empty, 0, 90.)], placement_mask_provider=provider)

    def test_global_rng_draws_only_for_sampled_rows(self):
        caches = [{} for _ in range(4)]
        flags = [True, False, True, False]
        device = self.policy.device
        for dist in (Bernoulli(logits=torch.tensor([-.2, .3, .5, -.7], device=device)),
                     Categorical(logits=torch.tensor([[.1, .4, .2]]*4, device=device))):
            torch.manual_seed(985)
            rng_before = torch.get_rng_state()
            cuda_before = torch.cuda.get_rng_state() if device.type == "cuda" else None
            if isinstance(dist, Bernoulli):
                sampled = Bernoulli(logits=dist.logits[[1, 3]], validate_args=False).sample()
                expected = (dist.probs >= .5).to(dist.probs.dtype)
            else:
                sampled = Categorical(logits=dist.logits[[1, 3]], validate_args=False).sample()
                expected = dist.logits.argmax(-1)
            expected[[1, 3]] = sampled
            cpu_after = torch.get_rng_state()
            cuda_after = torch.cuda.get_rng_state() if device.type == "cuda" else None
            torch.set_rng_state(rng_before)
            if cuda_before is not None:
                torch.cuda.set_rng_state(cuda_before)
            actual = self.policy._select_batch(dist, caches, 0, flags)
            torch.testing.assert_close(expected, actual, rtol=0, atol=0)
            self.assertTrue(torch.equal(cpu_after, torch.get_rng_state()))
            if cuda_after is not None:
                self.assertTrue(torch.equal(cuda_after, torch.cuda.get_rng_state()))

    def test_global_rng_merged_call_matches_controller_order_and_rng_state(self):
        requests = self.requests()
        flags = [i % 3 == 0 for i in range(len(requests))]
        original = self.policy._cache_request
        def without_uniforms(*args, **kwargs):
            cache = original(*args, **kwargs)
            cache.pop("sample_uniforms")
            return cache
        with patch.object(self.policy, "_cache_request", side_effect=without_uniforms):
            torch.manual_seed(941)
            expected = [None]*len(requests)
            for mode in (True, False):
                ids = [i for i, flag in enumerate(flags) if flag == mode]
                rows = self.policy.act_batch([requests[i] for i in ids], deterministic=mode)
                for i, row in zip(ids, rows):
                    expected[i] = row
            cpu_after = torch.get_rng_state()
            cuda_after = torch.cuda.get_rng_state() if self.policy.device.type == "cuda" else None
            torch.manual_seed(941)
            actual = self.policy.act_batch(requests, deterministic=flags)
            self.assert_actions_equal(expected, actual)
            self.assertTrue(torch.equal(cpu_after, torch.get_rng_state()))
            if cuda_after is not None:
                self.assertTrue(torch.equal(cuda_after, torch.cuda.get_rng_state()))


@unittest.skipUnless(torch.cuda.is_available(), "CUDA unavailable")
class CudaPerRowInferenceTests(PerRowInferenceTests):
    device = "cuda"


class MaskConcurrencyTests(unittest.TestCase):
    def test_masks_overlap_distinct_clients_serialize_duplicates_and_ignore_other_steps(self):
        league = object.__new__(AdaptiveLeague)
        barrier = Barrier(2)
        lock = Lock()
        active = set()
        overlap = []
        class Client:
            def __init__(self, n):
                self.n, self.calls = n, 0
            def request(self, payload):
                with lock:
                    if self.n in active:
                        raise AssertionError("concurrent requests on one native pipe")
                    active.add(self.n)
                    overlap.append(len(active))
                    self.calls += 1
                try:
                    if self.calls == 1:
                        barrier.wait(timeout=2)
                    return {"mask": [self.n, payload["side"], payload["slot"]]}
                finally:
                    with lock:
                        active.remove(self.n)
        league.clients = [Client(0), Client(1)]
        games = [SimpleNamespace(episode=object(), worker=i) for i in range(2)]
        requests = [(g.episode, {}, 0, 0.) for g in games]
        # One engine from ANOTHER ready group is already occupying the pool.
        release = Event()
        with ThreadPoolExecutor(max_workers=3) as league.executor:
            other_step = league.executor.submit(release.wait, 5)
            try:
                provider = league.mask_provider(games)
                pairs = [(requests[0], 3), (requests[1], 2), ((games[0].episode, {}, 1, 0.), 1)]
                self.assertEqual(provider.batch(pairs), [[0, 0, 3], [1, 0, 2], [0, 1, 1]])
                self.assertFalse(other_step.done())
                self.assertEqual(max(overlap), 2)
                self.assertEqual(provider(requests[1], 0), [1, 0, 0])
            finally:
                release.set()

    def test_mask_failure_drains_own_futures_before_returning(self):
        league = object.__new__(AdaptiveLeague)
        done = Event()
        barrier = Barrier(2)
        def fail(payload):
            barrier.wait(timeout=2)
            raise ValueError("bad mask reply")
        def succeed(payload):
            barrier.wait(timeout=2)
            done.set()
            return {"mask": [True]*576}
        league.clients = [SimpleNamespace(request=fail), SimpleNamespace(request=succeed)]
        games = [SimpleNamespace(episode=object(), worker=i) for i in range(2)]
        pairs = [((g.episode, {}, 0, 0.), 0) for g in games]
        with ThreadPoolExecutor(max_workers=2) as league.executor:
            with self.assertRaisesRegex(ValueError, "bad mask reply"):
                league.mask_provider(games).batch(pairs)
            self.assertTrue(done.is_set())


class PolicyGroupingTests(unittest.TestCase):
    def test_launch_groups_by_identity_and_keeps_physical_side_order(self):
        for league_type in (League, AdaptiveLeague):
            for shared_parent in (False, True):
                with self.subTest(league=league_type.__name__, shared_parent=shared_parent):
                    league = object.__new__(league_type)
                    league.args = SimpleNamespace(native_placement_masks=False)
                    def action(side):
                        return SimpleNamespace(side=side, wait=True)
                    def infer(requests, **kwargs):
                        return [action(r[2]) for r in requests]
                    league.base = SimpleNamespace(act_batch=Mock(side_effect=infer))
                    league.parent_policy = league.base if shared_parent else SimpleNamespace(act_batch=Mock(side_effect=infer))
                    league.learner_actions = lambda policy, requests, games, collecting: [action(r[2]) for r in requests]
                    games = [SimpleNamespace(worker=i, episode=object(), obs={"tick": 120}, start_tick=100,
                                             job={"seat": i % 2, "controller": c}, decisions=[])
                             for i, c in enumerate(("bc-sampled", "hog1-greedy", "bc-greedy"))]
                    league.clients = [SimpleNamespace(request=Mock(return_value={})) for _ in games]
                    steps = {}
                    with ThreadPoolExecutor(max_workers=3) as league.executor:
                        league._launch_steps(games, object(), True, steps)
                        wait(steps)
                    self.assertEqual(league.base.act_batch.call_count, 1)
                    kwargs = league.base.act_batch.call_args.kwargs
                    self.assertEqual(kwargs["deterministic"], [True, False, True] if shared_parent else [True, False])
                    if not shared_parent:
                        self.assertEqual(league.parent_policy.act_batch.call_count, 1)
                    for game in games:
                        self.assertEqual([a.side for _, a in game.pending_rows], [0, 1])
                        self.assertEqual(len(game.decisions), 1)
                        league.clients[game.worker].request.assert_called_once_with(
                            {"op": "step", "actions": [], "ticks": 20}, 45)


if __name__ == "__main__":
    unittest.main()
