"""CPU equivalence tests against the untouched original BananaPolicy."""
from __future__ import annotations

import unittest
from pathlib import Path

import torch

from banana_fast_policy import FastBananaPolicy
from banana_policy import BananaEpisode, BananaPolicy, DEFAULT_CHECKPOINT


DECK = (
    "hog-rider", "musketeer", "ice-golem", "skeletons",
    "ice-spirit", "cannon", "fireball", "the-log",
)


def episode_with_prefix(count: int) -> BananaEpisode:
    events = []
    for i in range(count):
        side = i % 2
        events.append({
            "seconds": 1.0 + 0.7 * i,
            "side": "team" if side == 0 else "opponent",
            "event_type": "card_play",
            "card": DECK[i % len(DECK)],
            "x": 4800 + 550 * i,
            "y": 5200 + 1350 * i,
        })
    return BananaEpisode((DECK, DECK), events=events, battle_id=f"batch-{count}")


def observation(policy: BananaPolicy) -> dict:
    hands = []
    for offset in (0, 2):
        rows = []
        for slot in range(4):
            card = DECK[(slot + offset) % len(DECK)]
            rows.append({
                "slot": slot, "name": card,
                "cost": int(policy.costs.get(card, 4)), "available": True,
            })
        hands.append(rows)
    return {"hands": hands, "elixir": [10.0, 10.0], "tick": 200}


class FastPolicyEquivalenceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        if not Path(DEFAULT_CHECKPOINT).exists():
            raise unittest.SkipTest(f"4.4.2 checkpoint not present: {DEFAULT_CHECKPOINT}")
        cls.policy = FastBananaPolicy(model_dir=DEFAULT_CHECKPOINT, device="cpu", seed=431)
        cls.policy.eval()

    def test_batched_actions_match_single_observation_actions(self) -> None:
        requests = [
            (episode_with_prefix(1), observation(self.policy), 0, 4.0),
            (episode_with_prefix(2), observation(self.policy), 1, 5.2),
            (episode_with_prefix(4), observation(self.policy), 0, 7.0),
        ]
        reference = [
            BananaPolicy.act(self.policy, *request, deterministic=True)
            for request in requests
        ]
        batched = self.policy.act_batch(requests, deterministic=True)
        self.assertEqual(len(reference), len(batched))
        for old, new in zip(reference, batched):
            self.assertEqual(
                (old.wait, old.side, old.slot, old.deck_slot, old.card, old.x, old.y, old.tile),
                (new.wait, new.side, new.slot, new.deck_slot, new.card, new.x, new.y, new.tile),
            )
            torch.testing.assert_close(old.log_prob, new.log_prob, rtol=1e-5, atol=1e-6)
            torch.testing.assert_close(old.entropy, new.entropy, rtol=1e-5, atol=1e-6)
            torch.testing.assert_close(old.value, new.value, rtol=1e-5, atol=1e-6)

    def test_batched_ppo_likelihood_entropy_value_and_gradients_match(self) -> None:
        requests = [
            (episode_with_prefix(1), observation(self.policy), 0, 4.0),
            (episode_with_prefix(2), observation(self.policy), 1, 5.2),
            (episode_with_prefix(4), observation(self.policy), 0, 7.0),
        ]
        actions = self.policy.act_batch(requests, deterministic=True)
        self.assertTrue(all(not action.wait for action in actions))

        def run_reference():
            self.policy.zero_grad(set_to_none=True)
            logp, entropy, value = BananaPolicy.evaluate_actions(self.policy, actions)
            loss = (0.31 * logp + 0.07 * entropy + 0.19 * value).sum()
            loss.backward()
            grads = {
                name: param.grad.detach().clone()
                for name, param in self.policy.named_parameters()
                if param.grad is not None
            }
            return logp.detach(), entropy.detach(), value.detach(), grads

        ref_logp, ref_entropy, ref_value, ref_grads = run_reference()
        self.policy.zero_grad(set_to_none=True)
        logp, entropy, value = self.policy.evaluate_actions(actions)
        loss = (0.31 * logp + 0.07 * entropy + 0.19 * value).sum()
        loss.backward()
        torch.testing.assert_close(logp, ref_logp, rtol=2e-5, atol=2e-6)
        torch.testing.assert_close(entropy, ref_entropy, rtol=2e-5, atol=2e-6)
        torch.testing.assert_close(value, ref_value, rtol=2e-5, atol=2e-6)
        for name, param in self.policy.named_parameters():
            if name in ref_grads:
                self.assertIsNotNone(param.grad, name)
                torch.testing.assert_close(param.grad, ref_grads[name], rtol=3e-4, atol=3e-6, msg=name)
        self.policy.zero_grad(set_to_none=True)

    def test_wait_only_rows_batch_without_placement_forward(self) -> None:
        ep = episode_with_prefix(2)
        obs = observation(self.policy)
        obs["elixir"] = [0.0, 0.0]
        rows = self.policy.act_batch(
            [(ep, obs, 0, 5.0), (ep, obs, 1, 5.0)], deterministic=True
        )
        self.assertTrue(all(row.wait for row in rows))
        logp, entropy, values = self.policy.evaluate_actions(rows)
        self.assertEqual(tuple(logp.shape), (2,))
        self.assertEqual(tuple(entropy.shape), (2,))
        self.assertEqual(tuple(values.shape), (2,))
        self.assertTrue(torch.isfinite(logp).all())
        self.assertTrue(torch.isfinite(entropy).all())

    def test_mixed_wait_and_play_rows_preserve_action_order(self) -> None:
        episode = episode_with_prefix(3)
        no_elixir = observation(self.policy)
        no_elixir["elixir"] = [0.0, 0.0]
        full_elixir = observation(self.policy)
        requests = [
            (episode, no_elixir, 0, 6.0),
            (episode, full_elixir, 1, 6.0),
            (episode_with_prefix(1), full_elixir, 0, 4.0),
        ]
        reference = [
            BananaPolicy.act(self.policy, *request, deterministic=True)
            for request in requests
        ]
        batched = self.policy.act_batch(requests, deterministic=True)
        self.assertEqual([action.wait for action in batched], [True, False, False])
        for old, new in zip(reference, batched):
            self.assertEqual((old.wait, old.card, old.tile), (new.wait, new.card, new.tile))
            torch.testing.assert_close(old.log_prob, new.log_prob, rtol=1e-5, atol=1e-6)
        logp, entropy, values = self.policy.evaluate_actions(batched)
        self.assertEqual(logp.shape, (3,))
        self.assertTrue(torch.isfinite(logp).all())
        self.assertTrue(torch.isfinite(entropy).all())
        self.assertTrue(torch.isfinite(values).all())


if __name__ == "__main__":
    unittest.main()
