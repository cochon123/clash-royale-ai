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
    device = "cpu"
    @classmethod
    def setUpClass(cls) -> None:
        if not Path(DEFAULT_CHECKPOINT).exists():
            raise unittest.SkipTest(f"4.4.2 checkpoint not present: {DEFAULT_CHECKPOINT}")
        torch.set_num_threads(2)
        cls.policy = FastBananaPolicy(model_dir=DEFAULT_CHECKPOINT, device=cls.device, seed=431)
        cls.policy.eval()

    def test_batched_actions_match_single_observation_actions(self) -> None:
        requests = [
            (episode_with_prefix(1), observation(self.policy), 0, 4.0),
            (episode_with_prefix(2), observation(self.policy), 1, 5.2),
            (episode_with_prefix(4), observation(self.policy), 0, 7.0),
        ]
        self.policy.model.padded_encoder = False
        reference = [
            BananaPolicy.act(self.policy, *request, deterministic=True)
            for request in requests
        ]
        self.policy.model.padded_encoder = True
        batched = self.policy.act_batch(requests, deterministic=True)
        singles = [self.policy.act(*request, deterministic=True) for request in requests]
        self.assertEqual(len(reference), len(batched))
        for old, new in [*zip(reference, batched), *zip(reference, singles)]:
            self.assertEqual(
                (old.wait, old.side, old.slot, old.deck_slot, old.card, old.x, old.y, old.tile),
                (new.wait, new.side, new.slot, new.deck_slot, new.card, new.x, new.y, new.tile),
            )
            torch.testing.assert_close(old.log_prob.cpu(), new.log_prob, rtol=1e-4, atol=1e-5)
            torch.testing.assert_close(old.entropy.cpu(), new.entropy, rtol=1e-4, atol=1e-5)
            torch.testing.assert_close(old.value.cpu(), new.value, rtol=1e-4, atol=1e-5)

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
            self.policy.model.padded_encoder = False
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
        self.policy.model.padded_encoder = True
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
        self.policy.model.padded_encoder = False
        reference = [
            BananaPolicy.act(self.policy, *request, deterministic=True)
            for request in requests
        ]
        self.policy.model.padded_encoder = True
        batched = self.policy.act_batch(requests, deterministic=True)
        self.assertEqual([action.wait for action in batched], [True, False, False])
        for old, new in zip(reference, batched):
            self.assertEqual((old.wait, old.card, old.tile), (new.wait, new.card, new.tile))
            torch.testing.assert_close(old.log_prob.cpu(), new.log_prob, rtol=1e-4, atol=1e-5)
        logp, entropy, values = self.policy.evaluate_actions(batched)
        self.assertEqual(logp.shape, (3,))
        self.assertTrue(torch.isfinite(logp).all())
        self.assertTrue(torch.isfinite(entropy).all())
        self.assertTrue(torch.isfinite(values).all())

    def test_variable_tile_support_and_fixed_support_after_update(self) -> None:
        requests = [(episode_with_prefix(n), observation(self.policy), n % 2, 90.) for n in (0, 1, 65)]
        masks = []
        for locations in ([3], [40, 41, 500], [0, 32, 95, 108, 543, 575]):
            mask = torch.zeros(576, dtype=torch.bool)
            mask[locations] = True
            masks.append(mask)
        self.policy.model.padded_encoder = False
        reference = [BananaPolicy.act(self.policy, *r, deterministic=True, legal_tile_mask=m)
                     for r, m in zip(requests, masks)]
        self.policy.model.padded_encoder = True
        batched = self.policy.act_batch(requests, deterministic=True, legal_tile_masks=masks)
        for old, new, mask in zip(reference, batched, masks):
            self.assertEqual((old.card, old.tile, old.x, old.y), (new.card, new.tile, new.x, new.y))
            self.assertTrue(mask[new.tile])
            torch.testing.assert_close(old.log_prob.cpu(), new.log_prob, rtol=1e-4, atol=1e-5)
            self.assertEqual(old.input_cache["tile_candidates"].tolist(), new.input_cache["tile_candidates"].tolist())
        # Force a new top-k ranking. PPO must still evaluate the original
        # support and propagate finite gradients, including one-location rows.
        head = self.policy.model.tile_head[-1]
        original = head.bias.detach().clone()
        try:
            with torch.no_grad():
                head.bias.add_(torch.linspace(-10., 10., head.bias.numel(), device=self.policy.device))
            self.policy.model.padded_encoder = False
            reference_values = BananaPolicy.evaluate_actions(self.policy, batched)
            self.policy.model.padded_encoder = True
            actual_values = self.policy.evaluate_actions(batched)
            for old, new in zip(reference_values, actual_values):
                # Dense cuDNN and the packed/native reference use different
                # FP32 reduction orders, most visible on long histories.
                torch.testing.assert_close(old, new, rtol=3e-4, atol=5e-5)
            sum(v.sum() for v in actual_values).backward()
            self.assertTrue(all(torch.isfinite(p.grad).all() for p in self.policy.parameters() if p.grad is not None))
        finally:
            with torch.no_grad():
                head.bias.copy_(original)
            self.policy.zero_grad(set_to_none=True)

    def test_ppo_restores_gru_eval_and_dropout_on_error(self) -> None:
        encoder = self.policy.model.encoder
        original = encoder.training, encoder.dropout
        with self.assertRaisesRegex(RuntimeError, "test failure"):
            with self.policy._ppo_backend():
                self.assertFalse(self.policy.model.training)
                if self.device == "cuda":
                    self.assertTrue(encoder.training)
                    self.assertEqual(encoder.dropout, 0.)
                raise RuntimeError("test failure")
        self.assertEqual((encoder.training, encoder.dropout), original)

    def test_sampled_likelihoods_match_before_any_optimizer_step(self) -> None:
        torch.manual_seed(904)
        requests = [(episode_with_prefix(i % 4), observation(self.policy), i % 2, 10.) for i in range(12)]
        original = self.policy.wait_head.bias.detach().clone()
        try:
            with torch.no_grad():
                self.policy.wait_head.bias.zero_()
            actions = self.policy.act_batch(requests, temperature=.7, placement_temperature=.6,
                                            placement_top_k=3, deterministic=False)
            logp, entropy, values = self.policy.evaluate_actions(actions)
            for expected, actual in (([a.log_prob for a in actions], logp),
                                     ([a.entropy for a in actions], entropy),
                                     ([a.value for a in actions], values)):
                torch.testing.assert_close(torch.stack(expected).to(self.policy.device), actual,
                                           rtol=3e-4, atol=5e-5)
            self.assertTrue(any(a.wait for a in actions))
            self.assertTrue(any(not a.wait for a in actions))
        finally:
            with torch.no_grad():
                self.policy.wait_head.bias.copy_(original)


@unittest.skipUnless(torch.cuda.is_available(), "CUDA unavailable")
class CudaFastPolicyEquivalenceTests(FastPolicyEquivalenceTests):
    device = "cuda"


if __name__ == "__main__":
    unittest.main()
