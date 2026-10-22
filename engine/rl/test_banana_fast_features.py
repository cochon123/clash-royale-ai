from __future__ import annotations

import sys
import unittest
from pathlib import Path
from unittest.mock import patch

import torch

ROOT = Path(__file__).resolve().parents[1]
SIBLING_SRC = ROOT.parent / "src"
if str(SIBLING_SRC) not in sys.path:
    sys.path.insert(0, str(SIBLING_SRC))

from cr_replay_pipeline import policy_dataset, policy_infer, winner_dataset  # noqa: E402
from rl.banana_fast_features import encode_online_sample  # noqa: E402


HOG = ("hog-rider", "musketeer", "ice-golem", "skeletons", "ice-spirit", "cannon", "fireball", "the-log")
MORTAR = ("mortar", "knight", "archers", "skeletons", "barbarian-barrel", "arrows", "mini-pekka", "rocket")


class FastFeatureParity(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        torch.set_num_threads(2)
        _, cls.vocab, _, _ = policy_infer.load_policy(
            ROOT.parent / "models" / "policy_bc_v4.4.2", "cpu"
        )
        cls.costs = winner_dataset.load_card_costs(
            ROOT.parent / "data" / "card_costs.json"
        )

    @staticmethod
    def battle(history_len: int) -> winner_dataset.BattleExample:
        rows = []
        for i in range(history_len + 1):
            side = "team" if i % 2 == 0 else "opponent"
            deck = HOG if side == "team" else MORTAR
            # Crosses double/triple-elixir boundaries in the longer prefixes.
            seconds = i * 2.15
            rows.append({"seconds": seconds, "side": side,
                         "event_type": "ability_activation" if i % 17 == 3 else "card_play",
                         "card": deck[(i * 3) % 8], "x": (i * 997) % 18000,
                         "y": (i * 1237) % 32000})
        return winner_dataset.BattleExample("feature-parity", HOG, MORTAR, 1, tuple(rows))

    def assert_equal_sample(self, battle, index, clock):
        expected = policy_dataset.encode_policy_sample(
            battle, index, self.vocab, self.costs, max_context=64,
            threat_dim=policy_dataset.THREAT_DIM, min_context=0,
            observation_seconds=clock,
        )
        calls = 0
        original = policy_dataset._encode_causal_stream

        def counted(*args, **kwargs):
            nonlocal calls
            calls += 1
            return original(*args, **kwargs)

        with patch.object(policy_dataset, "_encode_causal_stream", side_effect=counted):
            actual = encode_online_sample(
                policy_dataset, battle, index, self.vocab, self.costs,
                max_context=64, threat_dim=policy_dataset.THREAT_DIM,
                min_context=0, observation_seconds=clock,
            )
        self.assertEqual(len(expected), 13)
        self.assertEqual(len(actual), 13)
        for position, (left, right) in enumerate(zip(expected, actual)):
            self.assertTrue(torch.equal(left, right), f"tensor {position} differs at index={index}, clock={clock}")
        if index > 0 and clock is not None:
            self.assertEqual(calls, 1, "online path should encode the accepted prefix once")

    def test_lengths_sides_clocks_and_context_truncation(self):
        for history_len in (0, 1, 4, 65, 130):
            battle = self.battle(history_len)
            index = history_len
            if history_len == 0:
                self.assert_equal_sample(battle, index, 0.0)
                continue
            latest = battle.events[index - 1]["seconds"]
            # Both physical perspectives, exact event clocks, quiet clocks,
            # plus explicit double/triple-elixir observation times.
            for clock in (latest, latest + 13.0, max(latest, 120.0), max(latest, 240.0)):
                self.assert_equal_sample(battle, index, clock)

    def test_inferred_clock_matches_reference(self):
        battle = self.battle(4)
        self.assert_equal_sample(battle, 4, None)


if __name__ == "__main__":
    unittest.main()
