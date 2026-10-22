"""Check original logic inheritance used by spawned spectator units."""
import unittest

from tools.spectator_presentation import DEFAULT_LOGIC, index_logic, pick_row


class LogicInheritanceTests(unittest.TestCase):
    def test_barrel_goblin_inherits_original_art_and_keeps_load_override(self):
        index, _ = index_logic(DEFAULT_LOGIC)
        _, kind, fields = pick_row(index, "BarrelGoblin")
        self.assertEqual(kind, "CHARACTER")
        self.assertEqual(fields["FileName"], "sc/chr_goblin.sc")
        self.assertEqual(fields["BlueExportName"], "goblin1")
        self.assertEqual(fields["RedExportName"], "goblin1_enemy")
        self.assertEqual(fields["LoadTime"], 800)

    def test_explicit_base_type_does_not_resolve_to_an_unrelated_row(self):
        index = {
            "Spawn": [("ext.toml", "EXT", {"Base": "CHARACTER.Unit", "LoadTime": 80})],
            "Unit": [
                ("characters/unit.toml", "CHARACTER", {"FileName": "sc/unit.sc", "LoadTime": 70}),
                ("projectiles.toml", "PROJECTILE", {"FileName": "sc/wrong.sc"}),
            ],
        }
        _, kind, fields = pick_row(index, "Spawn")
        self.assertEqual(kind, "CHARACTER")
        self.assertEqual(fields["FileName"], "sc/unit.sc")
        self.assertEqual(fields["LoadTime"], 80)

    def test_invalid_and_cyclic_bases_remain_unresolved(self):
        index = {
            "A": [("ext.toml", "EXT", {"Base": "CHARACTER.B"})],
            "B": [("ext.toml", "EXT", {"Base": "CHARACTER.A"})],
        }
        self.assertIsNone(pick_row(index, "A"))
        self.assertIsNone(pick_row(index, "Missing"))


if __name__ == "__main__":
    unittest.main()
