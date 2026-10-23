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


class PlacementSamplingTests(unittest.TestCase):
    """Small CPU regressions independent of checkpoints or native engines."""

    @classmethod
    def setUpClass(cls):
        torch.set_num_threads(1)
        cls.policy = object.__new__(FastBananaPolicy)
        torch.nn.Module.__init__(cls.policy)
        cls.policy.device = torch.device("cpu")

    def cache(self, locations, top_k, uniform=None):
        mask = None
        if locations is not None:
            mask = torch.zeros(576, dtype=torch.bool)
            mask[locations] = True
        cache = {"tile_mask": mask, "placement_top_k": top_k, "placement_temperature": .8}
        if uniform is not None:
            cache["sample_uniforms"] = (uniform, uniform, uniform)
        return cache

    def select(self, logits, caches, flags):
        dist, support, counts = self.policy._tile_batch(logits, caches)
        choices = self.policy._select_batch(dist, caches, 2, flags, counts)
        return (support.gather(1, choices[:, None]).squeeze(1),
                [row[:count].tolist() for row, count in zip(support, counts)],
                dist.log_prob(choices), dist.entropy())

    def assert_selection_equal(self, expected, actual):
        self.assertEqual(expected[1], actual[1])
        for index in (0, 2, 3):
            torch.testing.assert_close(expected[index], actual[index], rtol=1e-6, atol=1e-6)

    def test_canonical_ties_and_uniforms_survive_heterogeneous_partitions(self):
        locations = ([0, 1], [5, 10, 17, 300, 575], None,
                     [1, 17, 19, 301], [80], list(range(576)))
        flags = [False, True, False, True, False, False]
        # Equal scores also straddle the top-k boundary. Score is primary;
        # ascending tile ID is the explicit secondary ranking criterion.
        logits = (torch.arange(576) % 4).float().repeat(len(flags), 1)
        for top_k in (1, 2, 3, 5, 576):
            caches = [self.cache(ids, top_k, uniform) for ids, uniform in
                      zip(locations, (.1, .7, .31, .99, .5, .83))]
            expected = self.select(logits, caches, flags)
            for row, ids, support in zip(logits, locations, expected[1]):
                legal = range(576) if ids is None else ids
                canonical = sorted(legal, key=lambda tile: (-float(row[tile]), tile))[:top_k]
                self.assertEqual(support, canonical)
            partitions = ([[0, 1, 2, 3, 4, 5]], [[0, 2, 4, 5], [1, 3]],
                          [[i] for i in range(6)], [[5, 1], [3, 0, 4], [2]])
            for groups in partitions:
                with self.subTest(top_k=top_k, groups=groups):
                    actual = [None] * 6
                    for ids in groups:
                        result = self.select(logits[ids], [caches[i] for i in ids], [flags[i] for i in ids])
                        for row, index in enumerate(ids):
                            actual[index] = (result[0][row], result[1][row], result[2][row], result[3][row])
                    merged = (torch.stack([row[0] for row in actual]), [row[1] for row in actual],
                              torch.stack([row[2] for row in actual]), torch.stack([row[3] for row in actual]))
                    self.assert_selection_equal(expected, merged)

    def test_unmasked_and_single_paths_use_same_canonical_support(self):
        logits = (torch.arange(576) % 3).float().repeat(3, 1)
        for sizes in ((1, 2, 576), (3, 5, 17)):
            fresh = [self.cache(None, k, .1) for k in sizes]
            masked = [self.cache(list(range(576)), k, .1) for k in sizes]
            expected = self.select(logits, fresh, False)
            self.assert_selection_equal(expected, self.select(logits, masked, False))
            for i, cache in enumerate(fresh):
                dist, support = self.policy._tile_distribution(logits[i], cache)
                self.assertEqual(support.tolist(), expected[1][i])
                torch.testing.assert_close(dist.entropy(), expected[3][i])

    def test_two_equal_tiles_keep_order_when_greedy_mask_widens_batch(self):
        logits = torch.zeros(2, 576)
        caches = [self.cache([0, 1], 576, .1), self.cache([0, 1, 2, 3, 4], 576, .1)]
        actual = self.select(logits, caches, [False, True])
        self.assertEqual(actual[0].tolist(), [0, 0])
        self.assertEqual(actual[1], [[0, 1], [0, 1, 2, 3, 4]])
        self.assert_selection_equal(self.select(logits[:1], caches[:1], False),
                                    (actual[0][:1], actual[1][:1], actual[2][:1], actual[3][:1]))

    def test_global_rng_matches_sampled_only_width_after_top_k(self):
        for top_k in (1, 2, 3, 5, 576):
            for locations, flags in (([list(range(2)), list(range(5))], [False, True]),
                                     ([[1, 575], None, [0, 100, 400], list(range(5))],
                                      [False, True, False, True])):
                caches = [self.cache(ids, top_k) for ids in locations]
                logits = torch.zeros(len(caches), 576)
                sampled_ids = [i for i, flag in enumerate(flags) if not flag]
                torch.manual_seed(0)
                expected = self.select(logits[sampled_ids], [caches[i] for i in sampled_ids], False)
                expected_state = torch.get_rng_state()
                expected_next = torch.rand(8)
                torch.manual_seed(0)
                actual = self.select(logits, caches, flags)
                actual_state = torch.get_rng_state()
                actual_next = torch.rand(8)
                with self.subTest(top_k=top_k, flags=flags):
                    sampled_actual = (actual[0][sampled_ids], [actual[1][i] for i in sampled_ids],
                                      actual[2][sampled_ids], actual[3][sampled_ids])
                    self.assert_selection_equal(expected, sampled_actual)
                    self.assertTrue(torch.equal(expected_state, actual_state))
                    torch.testing.assert_close(expected_next, actual_next, rtol=0, atol=0)
                # Pure greedy partitions cannot advance the global stream.
                before = torch.get_rng_state()
                greedy_ids = [i for i, flag in enumerate(flags) if flag]
                self.select(logits[greedy_ids], [caches[i] for i in greedy_ids], True)
                self.assertTrue(torch.equal(before, torch.get_rng_state()))

    def test_fixed_ppo_support_keeps_order_likelihood_entropy_and_gradients(self):
        caches = [self.cache([0, 1], 576, .1), self.cache([5, 10, 17, 300, 575], 3, .7)]
        rollout = self.select(torch.zeros(2, 576), caches, [False, True])
        for cache, support in zip(caches, rollout[1]):
            cache["tile_candidates"] = torch.tensor(support)
        # PPO must retain the rollout support after scores change. Include a
        # deliberately noncanonical legacy support to catch accidental sorting.
        caches[1]["tile_candidates"] = torch.tensor([17, 5, 10])
        logits = torch.linspace(-2., 2., 576).repeat(2, 1).requires_grad_()
        dist, support, counts = self.policy._tile_batch(logits, caches)
        choices = torch.tensor([0, 2])
        expected_logp, expected_entropy = [], []
        for i, cache in enumerate(caches):
            ids = cache["tile_candidates"]
            self.assertEqual(support[i, :counts[i]].tolist(), ids.tolist())
            reference = Categorical(logits=logits[i, ids] / .8)
            expected_logp.append(reference.log_prob(choices[i]))
            expected_entropy.append(reference.entropy())
        expected = torch.stack(expected_logp), torch.stack(expected_entropy)
        actual = dist.log_prob(choices), dist.entropy()
        for a, b in zip(actual, expected):
            torch.testing.assert_close(a, b)
        actual_grad, = torch.autograd.grad(sum(t.sum() for t in actual), logits, retain_graph=True)
        expected_grad, = torch.autograd.grad(sum(t.sum() for t in expected), logits)
        torch.testing.assert_close(actual_grad, expected_grad)


class PerRowInferenceTests(unittest.TestCase):
    device = "cpu"

    @classmethod
    def setUpClass(cls):
        torch.set_num_threads(1)
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
        requests = self.requests()[1:5]
        flags = [False, True, True, False]
        masks = []
        for locations in ([0, 1], list(range(5)), list(range(576)), [10, 100, 575]):
            mask = torch.zeros(576, dtype=torch.bool)
            mask[locations] = True
            masks.append(mask)
        original = self.policy._cache_request
        def without_uniforms(*args, **kwargs):
            cache = original(*args, **kwargs)
            cache.pop("sample_uniforms")
            return cache
        def tied_placement(first, caches, indices, slots):
            return {"tile_logits": first["fused"].new_zeros((len(indices), 576))}
        # Isolate sampling/support behavior from CUDA critic roundoff when
        # GRU batches change shape. The ordinary parity test checks the critic.
        with patch.object(self.policy, "_placement_from_context", side_effect=tied_placement), \
             patch.object(self.policy.wait_head, "forward", side_effect=lambda fused: fused.new_full((len(fused), 1), -100.)), \
             patch.object(self.policy.critic, "forward", side_effect=lambda fused: fused.new_zeros((len(fused), 1))):
            for uniforms in (True, False):
                cache_method = original if uniforms else without_uniforms
                with patch.object(self.policy, "_cache_request", side_effect=cache_method):
                    for top_k in (3, 576):
                        with self.subTest(uniforms=uniforms, top_k=top_k):
                            torch.manual_seed(0)
                            expected = [None]*len(requests)
                            for mode in (True, False):
                                ids = [i for i, flag in enumerate(flags) if flag == mode]
                                rows = self.policy.act_batch([requests[i] for i in ids], deterministic=mode,
                                                             legal_tile_masks=[masks[i] for i in ids], placement_top_k=top_k)
                                for i, row in zip(ids, rows):
                                    expected[i] = row
                            cpu_after = torch.get_rng_state()
                            cuda_after = torch.cuda.get_rng_state() if self.policy.device.type == "cuda" else None
                            expected_next = torch.rand(4, device=self.policy.device)
                            torch.manual_seed(0)
                            actual = self.policy.act_batch(requests, deterministic=flags,
                                                           legal_tile_masks=masks, placement_top_k=top_k)
                            self.assert_actions_equal(expected, actual)
                            self.assertTrue(torch.equal(cpu_after, torch.get_rng_state()))
                            if cuda_after is not None:
                                self.assertTrue(torch.equal(cuda_after, torch.cuda.get_rng_state()))
                            torch.testing.assert_close(expected_next, torch.rand(4, device=self.policy.device), rtol=0, atol=0)
                            logp, entropy, value = self.policy.evaluate_actions(actual)
                            for name, recomputed in (("log_prob", logp), ("entropy", entropy), ("value", value)):
                                torch.testing.assert_close(torch.stack([getattr(a, name) for a in actual]).to(self.policy.device),
                                                           recomputed, rtol=1e-4, atol=1e-5)
                            if uniforms:
                                singles = [self.policy.act_batch([request], deterministic=flag,
                                                                 legal_tile_masks=[mask], placement_top_k=top_k)[0]
                                           for request, flag, mask in zip(requests, flags, masks)]
                                self.assert_actions_equal(actual, singles)


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
