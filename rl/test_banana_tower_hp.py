"""Tower inputs, warm-start parity, PPO gradients and portable loading."""
import tempfile
import unittest
from pathlib import Path

import torch

from .banana_fast_policy import FastBananaPolicy
from .banana_hog_retrain import StableHogPolicy
from .banana_policy import BananaEpisode, ROOT
from .banana_tower_hp import TowerHPPolicy, tower_hp_features


def state():
    return {"towers": [dict(id=side * 3 + i, side=side,
                            row="KingTower" if i == 0 else "PrincessTower",
                            hp=100, max_hp=100)
                       for side in (0, 1) for i in range(3)]}


class TowerHPTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        torch.set_num_threads(2)
        cls.parent = ROOT / "runs/banana-1/base.pt"
        cls.deck = ("hog-rider", "musketeer", "ice-golem", "skeletons", "ice-spirit", "cannon", "fireball", "the-log")

    def test_perspective_destroyed_towers_and_validation(self):
        hp = state()
        hp["towers"][1]["hp"] = 0
        hp["towers"][3]["hp"] = 50
        self.assertEqual(tower_hp_features(hp, 0).tolist(), [1, 0, 1, .5, 1, 1])
        self.assertEqual(tower_hp_features(hp, 1).tolist(), [.5, 1, 1, 1, 0, 1])
        hp["towers"].reverse()
        self.assertEqual(tower_hp_features(hp, 0).tolist(), [1, 0, 1, .5, 1, 1])
        for value in (-1, 101, float("nan")):
            bad = state()
            bad["towers"][0]["hp"] = value
            with self.assertRaises(ValueError):
                tower_hp_features(bad, 0)
        with self.assertRaises(ValueError):
            tower_hp_features({"towers": []}, 0)

    def test_parity_learning_and_checkpoint_roundtrip(self):
        policy = StableHogPolicy(device="cpu", weights=self.parent, tower_hp=True)
        base = FastBananaPolicy(device="cpu", weights=self.parent)
        episode = BananaEpisode((self.deck, self.deck))
        hands = [[dict(slot=i, name=card, cost=cost, available=True)
                  for i, (card, cost) in enumerate(zip(self.deck[:4], [4, 4, 2, 1]))]] * 2
        obs = dict(hands=hands, elixir=[10., 10.], tower_state=state())
        request = [(episode, obs, 0, 0.)]
        action = policy.act_batch(request, deterministic=True)[0]
        original = base.act_batch(request, deterministic=True)[0]
        self.assertEqual((action.wait, action.slot, action.x, action.y),
                         (original.wait, original.slot, original.x, original.y))
        caches = [action.input_cache]
        out = policy._forward_batch(caches)
        old = base._forward_batch(caches)
        self.assertTrue(torch.equal(out["slot_logits"], old["slot_logits"]))
        self.assertTrue(torch.equal(out["fused"], old["fused"]))
        logp, entropy, value = policy.evaluate_actions([action])
        self.assertAlmostEqual(float(logp[0].detach()), float(action.log_prob), places=5)
        frozen = {k: v.clone() for k, v in policy.model.state_dict().items()
                  if not k.startswith(("slot_scorer.", "tower_hp_adapter."))}
        optimizer = torch.optim.Adam([p for p in policy.parameters() if p.requires_grad], lr=.01)
        (-logp.mean() + (value - 1).square().mean()).backward()
        self.assertGreater(float(policy.model.tower_hp_adapter[-1].weight.grad.abs().sum()), 0)
        optimizer.step()
        for key, tensor in frozen.items():
            self.assertTrue(torch.equal(tensor, policy.model.state_dict()[key]), key)
        changed = dict(caches[0], tower_hp=torch.zeros(6))
        self.assertFalse(torch.equal(policy._forward_batch(caches)["fused"], policy._forward_batch([changed])["fused"]))
        # Both fast placement paths retain the original placement context.
        slots = torch.tensor([0])
        first = policy._forward_batch(caches)
        place = policy._placement_from_context(first, caches, [0], slots)
        self.assertTrue(torch.equal(place["tile_logits"], base._placement_from_context(old, caches, [0], slots)["tile_logits"]))
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "hp.pt"
            torch.save(dict(format="banana-native-ppo-v1", model_state=policy.model.state_dict(),
                            wait_state=policy.wait_head.state_dict(), critic_state=policy.critic.state_dict()), path)
            loaded = TowerHPPolicy(device="cpu", weights=path)
            self.assertTrue(loaded.tower_hp_enabled)
            self.assertTrue(torch.equal(first["fused"], loaded._forward_batch(caches)["fused"]))
        with self.assertRaisesRegex(ValueError, "tower_state"):
            policy.act_batch([(episode, dict(hands=hands, elixir=[10., 10.]), 0, 0.)])


if __name__ == "__main__":
    unittest.main()
