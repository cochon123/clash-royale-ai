"""Init parity, PPO support, gradients and checkpoint round trip for BoardHogPolicy."""
import tempfile
import unittest
from pathlib import Path

import torch

from .banana_hog_adaptive import AdaptiveHogPolicy
from .banana_policy import BananaEpisode
from .board_policy import DEFAULT_HOG2_WEIGHTS, BoardHogPolicy
from .hog_distill import distill_batch
from .test_banana_tower_hp import state


def entity(row, side, x, y, hp, max_hp, dx=0, dy=0, entity_id=1, first_seen=1000):
    return {
        "id": entity_id, "row": row, "side": side, "x": x, "y": y,
        "hp": hp, "max_hp": max_hp, "dx": dx, "dy": dy, "first_seen_tick": first_seen,
    }


def sample_board(side, tick=1200):
    own_y = 4000 if side == 0 else 28000
    enemy_y = 28000 if side == 0 else 4000
    toward = 30 if side == 0 else -30
    return {"tick": tick, "entities": [
        entity("HogRider", side, 8500, own_y, 1200, 1408, dy=toward, entity_id=17, first_seen=1180),
        entity("Pekka", 1 - side, 12000, enemy_y, 3000, 3760, entity_id=4),
        entity("Skeleton", side, 7000, own_y + (1500 if side == 0 else -1500), 60, 67, entity_id=5),
        entity("IceSpirits", 1 - side, 9000, 16000, 90, 190, entity_id=6),
        entity("IceGolemite", side, 6000, own_y, 500, 1000, entity_id=7),
        entity("Assassin", 1 - side, 8000, 20000, 400, 750, dx=40, dy=-10, entity_id=8),
        entity("KingTower", side, 9000, 2000 if side == 0 else 30000, 2400, 4000, entity_id=1),
        entity("PrincessTower", side, 3500, 5000 if side == 0 else 27000, 1400, 2534, entity_id=2),
        entity("PrincessTower", 1 - side, 14500, 27000 if side == 0 else 5000, 800, 2534, entity_id=3),
    ]}


class BoardPolicyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        torch.set_num_threads(2)
        if not DEFAULT_HOG2_WEIGHTS.is_file():
            raise unittest.SkipTest(f"missing {DEFAULT_HOG2_WEIGHTS}")
        cls.deck = ("hog-rider", "musketeer", "ice-golem", "skeletons", "ice-spirit", "cannon", "fireball", "the-log")
        cls.policy = BoardHogPolicy(device="cpu")
        cls.base = AdaptiveHogPolicy(device="cpu", weights=DEFAULT_HOG2_WEIGHTS)
        cls.snapshot = {
            "encoder": {key: value.detach().cpu().clone() for key, value in cls.policy.board_encoder.state_dict().items()},
            "context": {key: value.detach().cpu().clone() for key, value in cls.policy.context_residual.state_dict().items()},
            "raster": {key: value.detach().cpu().clone() for key, value in cls.policy.raster.state_dict().items()},
            "critic": {key: value.detach().cpu().clone() for key, value in cls.policy.critic.proj.state_dict().items()},
            "wait_bias": cls.policy.wait_head.bias.detach().cpu().clone(),
        }

    def setUp(self):
        self.policy.board_encoder.load_state_dict(self.snapshot["encoder"])
        self.policy.context_residual.load_state_dict(self.snapshot["context"])
        self.policy.raster.load_state_dict(self.snapshot["raster"])
        self.policy.critic.proj.load_state_dict(self.snapshot["critic"])
        self.policy.wait_head.bias.data.copy_(self.snapshot["wait_bias"])
        self.policy.zero_grad(set_to_none=True)
        self.policy.critic.set_embed(None)
        self.policy.clear_boards()
        self.policy.eval()

    def hands(self):
        return [[dict(slot=i, name=card, cost=cost, available=True)
                 for i, (card, cost) in enumerate(zip(self.deck[:4], [4, 4, 2, 1]))]] * 2

    def requests(self, tick=1200):
        requests = []
        for side in (0, 1):
            episode = BananaEpisode((self.deck, self.deck), battle_id=f"board-{side}")
            episode.policy_seed = 7 + side
            observation = dict(
                hands=self.hands(), elixir=[10.0, 9.0], tower_state=state(),
                board=sample_board(side, tick=tick),
            )
            requests.append((episode, observation, side, 1.0))
        return requests

    def test_zero_init_matches_adaptive_policy_with_a_nonempty_board(self):
        requests = self.requests()
        learned = self.policy.act_batch(requests, deterministic=True, placement_top_k=576)
        original = self.base.act_batch(requests, deterministic=True, placement_top_k=576)
        for ours, theirs in zip(learned, original):
            self.assertEqual(
                (ours.wait, ours.slot, ours.deck_slot, ours.tile, ours.x, ours.y),
                (theirs.wait, theirs.slot, theirs.deck_slot, theirs.tile, theirs.x, theirs.y),
            )
            self.assertTrue(torch.equal(ours.log_prob, theirs.log_prob))
            self.assertTrue(torch.equal(ours.entropy, theirs.entropy))
            self.assertTrue(torch.equal(ours.value, theirs.value))
            self.assertGreater(int(ours.input_cache["board"].mask.sum()), 0)
        caches = [action.input_cache for action in learned]
        ours = self.policy._forward_batch(caches)
        # Adaptive ignores the board tensors stored on the same caches.
        theirs = self.base._forward_batch(caches)
        self.assertTrue(torch.equal(ours["slot_logits"], theirs["slot_logits"]))
        self.assertTrue(torch.equal(ours["fused"], theirs["fused"]))
        self.assertTrue(torch.isfinite(ours["board_embed"]).all())
        self.assertGreater(float(ours["board_embed"].detach().abs().sum()), 0.0)
        slots = torch.zeros(len(caches), dtype=torch.long)
        our_tiles = self.policy._placement_from_context(ours, caches, list(range(len(caches))), slots)
        their_tiles = self.base._placement_from_context(theirs, caches, list(range(len(caches))), slots)
        self.assertTrue(torch.equal(our_tiles["tile_logits"], their_tiles["tile_logits"]))
        blogp, bentropy, bvalues = self.policy.evaluate_actions(learned)
        alogp, aentropy, avalues = self.base.evaluate_actions(learned)
        self.assertTrue(torch.equal(blogp, alogp))
        self.assertTrue(torch.equal(bentropy, aentropy))
        self.assertTrue(torch.equal(bvalues, avalues))
        self.assertTrue(torch.equal(blogp.detach().cpu(), torch.stack([action.log_prob for action in learned])))
        played = [action for action in learned if not action.wait]
        if played:
            self.assertGreater(len(played[0].input_cache["tile_candidates"]), 5)

    def test_observe_board_cache_is_frozen_into_the_rollout(self):
        episode = BananaEpisode((self.deck, self.deck), battle_id="cached-match")
        observation = dict(hands=self.hands(), elixir=[10.0, 9.0], tower_state=state())
        with self.assertRaisesRegex(ValueError, "observe_board"):
            self.policy.act_batch([(episode, observation, 0, 0.0)])
        self.policy.observe_board(episode, sample_board(0, tick=10), 0)
        with self.assertRaisesRegex(ValueError, "perspective"):
            self.policy.act_batch([(episode, observation, 1, 0.0)])
        action = self.policy.act_batch([(episode, observation, 0, 0.0)], deterministic=True, placement_top_k=576)[0]
        self.assertEqual(action.input_cache["board"].tick, 10)
        self.policy.observe_board(episode.battle_id, sample_board(0, tick=99), 0)
        self.assertEqual(action.input_cache["board"].tick, 10)
        replayed, _, values = self.policy.evaluate_actions([action])
        self.assertTrue(torch.equal(replayed.detach().cpu().reshape(()), action.log_prob.reshape(())))
        self.assertTrue(torch.equal(values.detach().cpu().reshape(()), action.value.reshape(())))

    def test_new_branches_receive_gradients_and_checkpoints_round_trip(self):
        self.policy.wait_head.bias.data.fill_(-20.0)
        actions = self.policy.act_batch(self.requests(), deterministic=True, placement_top_k=576)
        self.assertTrue(all(not action.wait for action in actions))
        self.assertTrue(all(len(action.input_cache["tile_candidates"]) == 576 for action in actions))
        logp, _entropy, values = self.policy.evaluate_actions(actions)
        (-logp.mean() + (values - 1).square().mean()).backward()
        self.assertGreater(float(self.policy.context_residual.weight.grad.abs().sum()), 0.0)
        self.assertGreater(float(self.policy.raster.net[-1].weight.grad.abs().sum()), 0.0)
        self.assertGreater(float(self.policy.critic.proj.weight.grad.abs().sum()), 0.0)
        self.policy.zero_grad(set_to_none=True)
        with torch.no_grad():
            self.policy.context_residual.weight.add_(0.01)
            self.policy.raster.net[-1].weight.add_(0.01)
            self.policy.critic.proj.weight.add_(0.01)
        logp, _entropy, values = self.policy.evaluate_actions(actions)
        (-logp.mean() + (values - 1).square().mean()).backward()
        self.assertGreater(float(self.policy.board_encoder.cls.grad.abs().sum()), 0.0)
        self.assertGreater(float(self.policy.raster.net[0].weight.grad.abs().sum()), 0.0)
        report = self.policy.parameter_report()
        self.assertGreater(report["board_total"], 0)
        self.assertGreater(report["board_encoder"], report["context_residual"])
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "board.pt"
            self.policy.save_checkpoint(path)
            restored = BoardHogPolicy.from_checkpoint(path, device="cpu")
            try:
                left = self.policy.act_batch(self.requests(tick=1400), deterministic=True, placement_top_k=576)
                right = restored.act_batch(self.requests(tick=1400), deterministic=True, placement_top_k=576)
                for ours, loaded in zip(left, right):
                    self.assertEqual((ours.wait, ours.tile, ours.x, ours.y), (loaded.wait, loaded.tile, loaded.x, loaded.y))
                    self.assertTrue(torch.equal(ours.log_prob, loaded.log_prob))
                    self.assertTrue(torch.equal(ours.value, loaded.value))
                again = restored._forward_batch([action.input_cache for action in left])
                current = self.policy._forward_batch([action.input_cache for action in left])
                self.assertTrue(torch.equal(again["fused"], current["fused"]))
                self.assertTrue(torch.equal(again["slot_logits"], current["slot_logits"]))
            finally:
                del restored

    def test_initial_teacher_matches_the_board_free_student(self):
        actions = self.policy.act_batch(self.requests(), deterministic=True, placement_top_k=576)
        caches = [action.input_cache for action in actions]
        loss, stats = distill_batch(self.policy, self.base, caches, [0, 0])
        self.assertAlmostEqual(float(loss.detach()), 0.0, places=5)
        self.assertAlmostEqual(float(stats["wait"].detach()), 0.0, places=5)
        self.assertAlmostEqual(float(stats["slots"].detach()), 0.0, places=5)
        self.assertAlmostEqual(float(stats["tiles"].detach()), 0.0, places=5)


if __name__ == "__main__":
    unittest.main()
