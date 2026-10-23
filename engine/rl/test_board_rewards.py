"""Elixir-trade potential and the terminal-zero telescoping audit."""
import random
import unittest

from .board_features import fallback_row_table
from .board_rewards import audit_elixir_trade, elixir_trade_potential, shaped_elixir_terms


def entity(row, side, hp=100.0, max_hp=100.0):
    return {
        "id": 1, "row": row, "side": side, "x": 9000, "y": 8000,
        "hp": hp, "max_hp": max_hp, "dx": 0, "dy": 0, "first_seen_tick": 0,
    }


class ElixirTradeTests(unittest.TestCase):
    def setUp(self):
        self.table = fallback_row_table()

    def trade(self, entities, own_elixir, enemy_elixir, perspective=0):
        return elixir_trade_potential(
            {"tick": 1, "entities": entities}, perspective, own_elixir, enemy_elixir, self.table,
        )

    def test_formula_excludes_towers_and_unknown_rows(self):
        trade = self.trade(
            [
                entity("HogRider", 0, hp=704, max_hp=1408),
                entity("Skeleton", 0),
                entity("PrincessTower", 0, hp=1, max_hp=1000),
                entity("KingTower", 1, hp=1, max_hp=1000),
                entity("MysteryBolt", 1, hp=10, max_hp=10),
                entity("Pekka", 1, hp=100, max_hp=200),
            ],
            own_elixir=5,
            enemy_elixir=3,
        )
        self.assertAlmostEqual(trade.own_board_value, 4 * 0.5 + 1 / 3)
        self.assertAlmostEqual(trade.enemy_board_value, 7 * 0.5)
        self.assertEqual(trade.unknown_entities, 1)
        self.assertAlmostEqual(
            trade.potential,
            (5 + trade.own_board_value) - (3 + trade.enemy_board_value),
        )
        flipped = self.trade(
            [entity("HogRider", 0, hp=1408, max_hp=1408)], 1, 9, perspective=1,
        )
        self.assertAlmostEqual(flipped.enemy_board_value, 4.0)
        self.assertAlmostEqual(flipped.own_board_value, 0.0)
        self.assertAlmostEqual(flipped.potential, (1 + 0) - (9 + 4))

    def test_shaped_terms_telescope_to_negative_initial_potential(self):
        initial = self.trade([entity("HogRider", 0)], 5, 3).potential
        later = self.trade([entity("HogRider", 0, hp=50, max_hp=100)], 4, 6).potential
        end = self.trade([], 2, 2).potential
        potentials = [initial, later, end]
        terms = shaped_elixir_terms(potentials, coefficient=1.0)
        self.assertEqual(len(terms), 2)
        self.assertAlmostEqual(terms[0], later - initial)
        self.assertAlmostEqual(terms[1], 0.0 - later)
        audit = audit_elixir_trade(potentials, 1.0, terms)
        self.assertTrue(audit["ok"])
        self.assertAlmostEqual(audit["sum_shaped"], -initial)
        self.assertEqual(audit["terminal_potential"], 0.0)
        scaled = audit_elixir_trade(potentials, 0.25)
        self.assertAlmostEqual(scaled["sum_shaped"], -0.25 * initial)
        rng = random.Random(0)
        samples = [rng.uniform(-8, 8) for _ in range(12)]
        self.assertAlmostEqual(sum(shaped_elixir_terms(samples, 0.5)), -0.5 * samples[0])
        with self.assertRaises(ValueError):
            shaped_elixir_terms([1.0])
        with self.assertRaises(RuntimeError):
            audit_elixir_trade(potentials, 1.0, [0.0, 0.0])


if __name__ == "__main__":
    unittest.main()
