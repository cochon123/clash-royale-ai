"""Learning and checkpoint invariants for the conservative Hog1 experiment."""
import json
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import Mock, patch

import torch

from .banana_hog_retrain import (MatchedSampler, StableHogPolicy, StableLeague, gae_returns,
                                  shaped_rewards, tower_potential, validate_resume_config)
from .banana_hog_league import ROOT, Sampler
from .banana_policy import BananaEpisode


class RewardInvariants(unittest.TestCase):
    def test_resume_cannot_change_the_seed_bank_or_reward(self):
        args = SimpleNamespace(eval_seeds=2, shaping_coefficient=.5)
        validate_resume_config({"args": {"eval_seeds": 2, "shaping_coefficient": .5}}, args)
        with self.assertRaisesRegex(ValueError, "eval_seeds"):
            validate_resume_config({"args": {"eval_seeds": 3}}, args)
        with self.assertRaisesRegex(ValueError, "shaping_coefficient"):
            validate_resume_config({"args": {"shaping_coefficient": 1.}}, args)

    def test_interrupted_finalization_saves_selected_weights(self):
        league = object.__new__(StableLeague)
        league.args = SimpleNamespace(device="cpu")
        league.run_dir = Path("/unused-test-directory")
        league.version = 456
        league.evaluation = {"confirmation": None}
        league.load_weights, league.save = Mock(), Mock()
        selected = {"updates": 7, "name": "selected-test"}
        with patch("rl.banana_hog_retrain.torch.load", return_value=selected):
            league.save_selected_final()
        league.load_weights.assert_called_once_with(selected)
        league.save.assert_called_once_with("final")
        self.assertEqual(league.version, 7)
        self.assertFalse(league.evaluation["final_artifact"]["confirmation_complete"])

    def test_damage_shaping_preserves_native_win_loss_return(self):
        for outcome in [-1., 0., 1.]:
            for potentials in [[0., .1, .2, -.1, .7], [.1, -.4, .8], [0., .5]]:
                rewards = shaped_rewards(potentials, outcome, .5)
                self.assertAlmostEqual(sum(rewards) + .5 * potentials[0], outcome)

    def test_gae_terminal_and_lambda_one_equal_monte_carlo(self):
        rewards, values = [.2, -.1, .9], [.1, -.2, .3]
        advantages, returns = gae_returns(rewards, values, lam=1.)
        self.assertEqual(len(advantages), 3)
        for actual, expected in zip(returns, [1., .8, .9]):
            self.assertAlmostEqual(actual, expected)

    def test_potential_uses_both_native_sides_and_rejects_missing_towers(self):
        towers = [{"id": side*3+i, "side": side, "hp": 100, "max_hp": 100}
                  for side in (0, 1) for i in range(3)]
        towers[-1]["hp"] = 0
        self.assertAlmostEqual(tower_potential({"towers": towers}, 0), 1/3)
        self.assertAlmostEqual(tower_potential({"towers": towers}, 1), -1/3)
        with self.assertRaises(ValueError):
            tower_potential({"towers": towers[:-1]}, 0)

    def test_sampler_checkpoint_contains_no_python_sampler_object(self):
        decks = json.loads((ROOT / "data/hog2_opponent_pool.json").read_text())["decks"]
        for d in decks:
            d["holdout"] = d["id"] in {"miner-wallbreakers", "giant-princes"}
        sampler = MatchedSampler(decks, 17)
        for _ in range(8):
            sampler.draw()
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "state.pt"
            torch.save({"sampler": sampler.state_dict()}, path)
            state = torch.load(path, weights_only=True)["sampler"]
            restored = MatchedSampler.from_state_dict(state)
        self.assertEqual(sampler.draw(), restored.draw())
        self.assertEqual(len(sampler.cells), 18)
        self.assertNotIn("bc-sampled", [c for _, c in sampler.cells])


class FrozenPolicyInvariants(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        torch.set_num_threads(2)
        cls.parent = ROOT / "runs/banana-1/base.pt"
        cls.policy = StableHogPolicy(device="cuda" if torch.cuda.is_available() else "cpu", weights=cls.parent)
        cls.deck = ("hog-rider", "musketeer", "ice-golem", "skeletons", "ice-spirit", "cannon", "fireball", "the-log")

    def test_parent_weights_and_input_contract_are_preserved(self):
        saved = torch.load(self.parent, map_location="cpu", weights_only=False)
        for name, value in self.policy.model.state_dict().items():
            self.assertTrue(torch.equal(value.cpu(), saved["model_state"][name]), name)
        for name, param in self.policy.model.named_parameters():
            self.assertEqual(param.requires_grad, name.startswith("slot_scorer."), name)

    def test_gpu_likelihood_and_gradient_update_leave_body_exact(self):
        episode = BananaEpisode((self.deck, self.deck))
        hands = [[{"slot": i, "name": name, "cost": cost, "available": True}
                  for i, (name, cost) in enumerate(zip(self.deck[:4], [4,4,2,1]))]] * 2
        requests = [(episode, {"hands": hands, "elixir": [10., 10.]}, i % 2, 0.) for i in range(8)]
        before = {name: value.detach().clone() for name, value in self.policy.model.state_dict().items()}
        actions = self.policy.act_batch(requests)
        logp, entropy, value = self.policy.evaluate_actions(actions)
        torch.testing.assert_close(logp.detach().cpu(), torch.stack([a.log_prob for a in actions]), atol=2e-5, rtol=2e-5)
        optimizer = torch.optim.Adam([p for p in self.policy.parameters() if p.requires_grad], lr=1e-4)
        optimizer.zero_grad()
        (-logp.mean() + value.square().mean() - .01*entropy.mean()).backward()
        self.assertTrue(any(p.grad is not None for p in self.policy.wait_head.parameters()))
        self.assertTrue(all(p.grad is None for p in self.policy.model.encoder.parameters()))
        optimizer.step()
        for name, param in self.policy.model.state_dict().items():
            if not name.startswith("slot_scorer."):
                self.assertTrue(torch.equal(param, before[name]), name)
        self.policy.model.load_state_dict(before)


if __name__ == "__main__":
    unittest.main()
