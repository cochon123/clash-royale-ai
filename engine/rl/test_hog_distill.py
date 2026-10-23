"""Synthetic KL checks for teacher-to-student distillation."""
import unittest

import torch

from .hog_distill import build_parser, distribution_kl


class DistillTests(unittest.TestCase):
    def test_matching_logits_have_zero_kl_and_student_receives_gradients(self):
        wait = torch.tensor([0.4, -1.2])
        slots = torch.tensor([[0.0, 2.0, -1.0, 0.5, -3.0, 1.0, 0.2, -0.2],
                              [1.0, 0.0, 0.0, -2.0, 0.3, 0.3, -0.4, 4.0]])
        tiles = torch.zeros(2, 576)
        tiles[0, 10] = 3.0
        tiles[1, 400] = 2.5
        matched = distribution_kl(wait, wait, slots, slots, tiles, tiles)
        self.assertAlmostEqual(float(matched["total"]), 0.0, places=6)
        self.assertAlmostEqual(float(matched["wait"]), 0.0, places=6)
        self.assertAlmostEqual(float(matched["slots"]), 0.0, places=6)
        self.assertAlmostEqual(float(matched["tiles"]), 0.0, places=6)

        student_wait = wait.detach().clone().requires_grad_(True)
        student_slots = (slots + torch.tensor([0.5, -0.25, 0, 0, 0, 0, 0, 0])).detach().requires_grad_(True)
        student_tiles = tiles.detach().clone()
        student_tiles[0, 11] = 1.0
        student_tiles.requires_grad_(True)
        shifted = distribution_kl(wait, student_wait, slots, student_slots, tiles, student_tiles)
        self.assertGreater(float(shifted["total"].detach()), 0.0)
        shifted["total"].backward()
        self.assertGreater(float(student_wait.grad.abs().sum()), 0.0)
        self.assertGreater(float(student_slots.grad.abs().sum()), 0.0)
        self.assertGreater(float(student_tiles.grad.abs().sum()), 0.0)

    def test_cli_parser_accepts_the_rollout_contract(self):
        args = build_parser().parse_args(["--rollout", "rollout.pt", "--out", "student.pt", "--steps", "2"])
        self.assertEqual(args.steps, 2)
        self.assertEqual(args.trainable, "all")
        self.assertEqual(args.device, "cpu")


if __name__ == "__main__":
    unittest.main()
