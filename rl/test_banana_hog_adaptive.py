import copy
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import Mock

import torch

from .banana_hog_adaptive import AdaptiveHogPolicy, AdaptiveLeague, SamplingPolicy, regression_reason
from .banana_policy import BananaEpisode
from .test_banana_tower_hp import state


class AdaptiveTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        torch.set_num_threads(2)
        cls.parent = Path(__file__).resolve().parents[1] / "runs/banana-1/base.pt"
        cls.deck = ("hog-rider", "musketeer", "ice-golem", "skeletons", "ice-spirit", "cannon", "fireball", "the-log")

    def requests(self, count=6):
        hands = [[dict(slot=i, name=card, cost=cost, available=True)
                  for i, (card, cost) in enumerate(zip(self.deck[:4], [4, 4, 2, 1]))]] * 2
        requests = []
        for i in range(count):
            episode = BananaEpisode((self.deck, self.deck))
            episode.policy_seed = 99 + i
            requests.append((episode, dict(hands=hands, elixir=[10., 10.], tower_state=state()), i % 2, 0.))
        return requests

    def test_joint_gpu_gradients_full_support_and_scheduling_independent_sampling(self):
        policy = AdaptiveHogPolicy(device="cuda" if torch.cuda.is_available() else "cpu", weights=self.parent)
        requests = self.requests()
        actions = policy.act_batch(requests, placement_top_k=576)
        reverse = policy.act_batch(list(reversed(requests)), placement_top_k=576)
        identify = lambda a: (a.wait, a.slot, a.tile)
        self.assertEqual(list(map(identify, actions)), list(reversed(list(map(identify, reverse)))))
        self.assertTrue(any(not a.wait and len(a.input_cache["tile_candidates"]) > 5 for a in actions))
        logp, entropy, values = policy.evaluate_actions(actions)
        self.assertTrue(torch.allclose(logp.detach(), torch.stack([a.log_prob for a in actions]).to(logp.device), atol=2e-5))
        before = {name: tensor.clone() for name, tensor in policy.model.state_dict().items()}
        optimizer = torch.optim.Adam(policy.parameters(), lr=1e-4)
        (-logp.mean() + (values - .5).square().mean() - .001 * entropy.mean()).backward()
        for prefix in ("encoder.", "tile_head.", "tower_hp_adapter."):
            gradients = [p.grad for name, p in policy.model.named_parameters() if name.startswith(prefix) and p.grad is not None]
            self.assertGreater(sum(float(g.detach().abs().sum()) for g in gradients), 0, prefix)
        optimizer.step()
        for prefix in ("encoder.", "tile_head.", "tower_hp_adapter."):
            self.assertTrue(any(not torch.equal(value, before[name]) for name, value in policy.model.state_dict().items() if name.startswith(prefix)), prefix)
        # Rollout and optimization use a full legal distribution after weights change.
        updated, _, _ = policy.evaluate_actions(actions)
        self.assertTrue(torch.isfinite(updated).all())

    def test_trainability_modes_and_action_selection_contract(self):
        for mode in ("heads", "placement", "all"):
            policy = AdaptiveHogPolicy(device="cpu", weights=self.parent, trainable=mode)
            params = dict(policy.model.named_parameters())
            self.assertEqual(params["encoder.weight_ih_l0"].requires_grad, mode == "all")
            self.assertEqual(next(p for n, p in params.items() if n.startswith("tile_head.")).requires_grad, mode != "heads")
        league = object.__new__(AdaptiveLeague)
        policy = Mock()
        games = [SimpleNamespace(job={})]
        league.learner_actions(policy, [], games, True)
        self.assertEqual(policy.act_batch.call_args.kwargs, dict(deterministic=False, placement_top_k=576))
        league.learner_actions(policy, [], games, False)
        self.assertFalse(policy.act_batch.call_args.kwargs["deterministic"])
        league.learner_actions(policy, [], [SimpleNamespace(job={"learner_mode": "greedy"})], False)
        self.assertTrue(policy.act_batch.call_args.kwargs["deterministic"])
        reference = SamplingPolicy(device="cpu", weights=self.parent)
        requests = self.requests()
        a = reference.act_batch(requests, placement_top_k=576)
        b = reference.act_batch(list(reversed(requests)), placement_top_k=576)
        self.assertEqual([(x.wait, x.tile) for x in a], [(x.wait, x.tile) for x in reversed(b)])

    def test_native_mask_uses_selected_hand_slot_and_preserves_ppo_support(self):
        policy = AdaptiveHogPolicy(device="cpu", weights=self.parent)
        seen = {}
        def query(request, native_slot):
            seen[(id(request[0]), request[2])] = native_slot
            mask = torch.zeros(576, dtype=torch.bool)
            mask[130 + native_slot] = True
            return mask
        requests = self.requests()
        actions = policy.act_batch(requests, placement_top_k=576, placement_mask_provider=query)
        for request, action in zip(requests, actions):
            if not action.wait:
                self.assertEqual(seen[(id(request[0]), request[2])], action.slot)
                self.assertEqual(action.tile, 130 + action.slot)
                self.assertEqual(action.input_cache["tile_candidates"].tolist(), [action.tile])
        logp, _, _ = policy.evaluate_actions(actions)
        self.assertTrue(torch.allclose(logp.detach(), torch.stack([a.log_prob for a in actions]), atol=2e-5))
        with self.assertRaisesRegex(ValueError, "native placement mask"):
            policy.act_batch(requests, placement_mask_provider=lambda request, slot: [False] * 576)

    def test_best_relative_rollback_restores_encoder_placement_and_optimizer(self):
        league = object.__new__(AdaptiveLeague)
        league.args = SimpleNamespace(device="cpu", regression_threshold=.02, learning_rate=3e-5,
                                      backbone_learning_rate=3e-6, critic_learning_rate=.0003)
        league.learner = AdaptiveHogPolicy(device="cpu", weights=self.parent)
        league.optimizer = league.make_optimizer()
        league.guard = dict(rollback_count=0, evaluation_rollbacks=0)
        league.frozen_body = {}
        league.version = 10
        league.parent_policy = league.base = Mock()
        league.policy_signature = lambda p: ()
        league.weights_digest = lambda p: "test-weights"
        league.frozen_signature = ((), ())
        league.publish = league.save = league.log = Mock()
        league.eval_seconds = 0
        league._retained_rows = Mock(return_value={})
        def rows(wins):
            return [dict(deck_id="hog-mirror", controller="bc-greedy", seat=i % 2, seed=100 + i,
                         result="W" if i < wins else "L", policy_version=10) for i in range(100)]
        league.eval_jobs = lambda bank: rows(50)
        league.play = Mock(side_effect=[(rows(44), []), (rows(49), [])])
        league.evaluation = dict(best=dict(score=.5, greedy_score=.5), baseline={}, history=[])
        before = copy.deepcopy(league.learner.model.state_dict())
        with tempfile.TemporaryDirectory() as directory:
            league.run_dir = Path(directory)
            (league.run_dir / "models").mkdir()
            torch.save(dict(model_state=before, wait_state=league.learner.wait_head.state_dict(),
                            critic_state=league.learner.critic.state_dict(), optimizer_state=league.optimizer.state_dict()),
                       league.run_dir / "models/best.pt")
            with torch.no_grad():
                for p in league.learner.model.parameters():
                    p.add_(1.)
            self.assertTrue(league.evaluate())
        for name, value in before.items():
            self.assertTrue(torch.equal(value, league.learner.model.state_dict()[name]), name)
        self.assertEqual(league.guard["evaluation_rollbacks"], 1)
        rates = {g["role"]: g["lr"] for g in league.optimizer.param_groups}
        self.assertEqual(rates, dict(actor=1.5e-5, backbone=1.5e-6, critic=.0003))
        self.assertIn("greedy", regression_reason(.51, .45, .5, .5))
        self.assertIsNone(regression_reason(None, None, .5, .5))
        # A strong partial sampled bank plus an unfinished greedy bank cannot
        # promote, roll back, or relabel a selected checkpoint.
        best = copy.deepcopy(league.evaluation["best"])
        league.play = Mock(side_effect=[(rows(90), []), (rows(90)[:-1], [])])
        self.assertFalse(league.evaluate())
        self.assertEqual(league.evaluation["best"], best)
        self.assertEqual(league.guard["evaluation_rollbacks"], 1)
        self.assertNotIn(unittest.mock.call("best"), league.save.call_args_list)


if __name__ == "__main__":
    unittest.main()
