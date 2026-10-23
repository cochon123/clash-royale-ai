"""Board JSON encoding, perspective, truncation and the row-cost fallback."""
import json
import unittest
from pathlib import Path

import torch

from .board_features import (
    ARENA_X,
    ARENA_Y,
    CELL_SIZE,
    MAX_ENTITIES,
    NATIVE_CELLS,
    TILE_COLS,
    TILE_ROWS,
    collate_boards,
    encode_board,
    fallback_row_table,
    load_row_table,
    tile_of,
    vocab_for_table,
)


def entity(row, side, x, y, hp=100.0, max_hp=100.0, dx=0, dy=0, entity_id=1, first_seen=0):
    return {
        "id": entity_id, "row": row, "side": side, "x": x, "y": y,
        "hp": hp, "max_hp": max_hp, "dx": dx, "dy": dy, "first_seen_tick": first_seen,
    }


def board(*entities, tick=100):
    return {"tick": tick, "entities": list(entities)}


def tile_center(row, column, side):
    """Physical point at a placement-tile center, using the side-1 Y mirror."""
    nx = (column + 0.5) / TILE_COLS
    ny = (row + 0.5) / TILE_ROWS
    if side == 1:
        ny = 1.0 - ny
    return round(nx * ARENA_X), round(ny * ARENA_Y)


class BoardFeatureTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.table = fallback_row_table()
        cls.vocab = vocab_for_table(cls.table)

    def encode(self, payload, perspective):
        return encode_board(payload, perspective, self.table, self.vocab)

    def test_fallback_covers_pool_decks_and_spawn_aliases(self):
        self.assertEqual(self.table.source, "fallback:fast_league_decks")
        hog = self.table.lookup("HogRider")
        self.assertEqual((hog.elixir, hog.units_per_card, hog.card), (4.0, 1.0, "hog_rider"))
        self.assertEqual(self.table.lookup("Assassin").elixir, 3.0)
        self.assertEqual(self.table.lookup("Pekka").elixir, 7.0)
        self.assertEqual(self.table.lookup("IceGolemite").elixir, 2.0)
        self.assertEqual(self.table.lookup("IceSpirits").elixir, 1.0)
        skeleton = self.table.lookup("Skeleton")
        self.assertEqual((skeleton.elixir, skeleton.units_per_card), (1.0, 3.0))
        self.assertAlmostEqual(skeleton.entity_value(1.0), 1.0 / 3.0)
        self.assertEqual(self.table.lookup("Tombstone").elixir, 3.0)
        self.assertTrue(self.table.lookup("NotARealRow").unknown)
        self.assertEqual(self.table.lookup("NotARealRow").entity_value(1.0), 0.0)
        self.assertEqual(self.table.lookup("KingTower").kind, "tower")
        self.assertEqual(self.table.lookup("PrincessTower").entity_value(0.5), 0.0)

    def test_explicit_table_replaces_fallback_and_missing_path_errors(self):
        with self.assertRaises(FileNotFoundError):
            load_row_table("/tmp/does-not-exist-board-rows.json")
        payload = {
            "format": "board-rows-v1",
            "unknown": {"elixir": 0, "units_per_card": 1},
            "rows": [{"row": "HogRider", "card": "hog-rider", "elixir": 9, "units_per_card": 1, "kind": "troop"}],
        }
        path = Path("/tmp/board-rows-w2-test.json")
        path.write_text(json.dumps(payload))
        try:
            table = load_row_table(path)
        finally:
            path.unlink(missing_ok=True)
        self.assertEqual(table.lookup("HogRider").elixir, 9.0)
        self.assertTrue(table.lookup("Pekka").unknown)
        self.assertEqual(load_row_table().source, "fallback:fast_league_decks")

    def test_native_cell_and_placement_scales_agree(self):
        self.assertEqual(NATIVE_CELLS, (36, 64))
        self.assertEqual(NATIVE_CELLS[0] * CELL_SIZE, ARENA_X)
        self.assertEqual(NATIVE_CELLS[1] * CELL_SIZE, ARENA_Y)
        self.assertEqual(TILE_ROWS, 18)
        self.assertEqual(TILE_COLS, 32)

    def test_perspective_matches_placement_mirror(self):
        for side in (0, 1):
            x, y = tile_center(7, 4, side)
            encoded = self.encode(board(entity("HogRider", side, x, y, dx=0, dy=30, entity_id=7)), side)
            self.assertTrue(bool(encoded.mask[0]))
            self.assertEqual(int(encoded.type_ids[0]), self.vocab.encode("HogRider"))
            self.assertEqual(float(encoded.features[0, 0]), 1.0)
            row, column = tile_of(float(encoded.features[0, 1]), float(encoded.features[0, 2]))
            self.assertEqual((row, column), (7, 4))
            self.assertGreater(float(encoded.raster[0, 7, 4]), 0.0)
            self.assertAlmostEqual(float(encoded.raster[2, 7, 4]), 4.0)
            expected_motion_y = -0.03 if side == 1 else 0.03
            self.assertAlmostEqual(float(encoded.features[0, 5]), expected_motion_y)
        enemy = self.encode(board(entity("Pekka", 0, 1000, 1000, entity_id=3)), 1)
        self.assertEqual(float(enemy.features[0, 0]), 0.0)
        self.assertGreater(float(enemy.features[0, 2]), 0.5)

    def test_lane_scalars_unknown_rows_and_deterministic_truncation(self):
        hog_x, hog_y = tile_center(2, 3, 0)
        encoded = self.encode(board(
            entity("HogRider", 1, hog_x, hog_y, hp=50, max_hp=100, entity_id=2),
            entity("PrincessTower", 0, 3000, 5000, hp=1000, max_hp=2000, entity_id=9),
            entity("MysteryBolt", 0, 1000, 1000, entity_id=11),
        ), 0)
        self.assertAlmostEqual(encoded.enemy_value, 2.0)
        self.assertAlmostEqual(encoded.own_value, 0.0)
        self.assertAlmostEqual(float(encoded.scalars[2]), 2.0)
        self.assertAlmostEqual(float(encoded.scalars[4]), 2.0)
        self.assertEqual(encoded.unknown_count, 1)
        present = {int(index) for index in encoded.type_ids[encoded.mask].tolist()}
        mystery = self.vocab.encode("MysteryBolt")
        self.assertIn(mystery, present)
        self.assertEqual(mystery, self.vocab.encode("MysteryBolt"))
        self.assertGreaterEqual(mystery, self.vocab.bucket_offset)

        troops = [
            entity("Skeleton", 0, 500 + (i % 30) * 500, 8000 + (i // 30) * 400, entity_id=i + 1)
            for i in range(70)
        ]
        towers = [
            entity("KingTower" if i == 0 else "PrincessTower", side, 2000 + i * 4000, 1000 if side == 0 else 30000, entity_id=100 + side * 3 + i)
            for side in (0, 1) for i in range(3)
        ]
        full = board(*(towers + troops), tick=5000)
        forward = self.encode(full, 0)
        backward = self.encode({"tick": 5000, "entities": list(reversed(full["entities"]))}, 0)
        self.assertEqual(forward.dropped, len(towers) + len(troops) - MAX_ENTITIES)
        self.assertTrue(torch.equal(forward.type_ids, backward.type_ids))
        self.assertTrue(torch.equal(forward.features, backward.features))
        self.assertTrue(torch.equal(forward.mask, backward.mask))
        self.assertTrue(torch.equal(forward.raster, backward.raster))
        kept_rows = [self.vocab.names[int(index) - 1] for index in forward.type_ids[forward.mask].tolist() if int(index) < self.vocab.bucket_offset]
        self.assertGreaterEqual(sum(name in ("KingTower", "PrincessTower") for name in kept_rows), 6)
        self.assertAlmostEqual(forward.own_value, backward.own_value)
        self.assertGreater(forward.own_value, 0.0)

    def test_collate_and_rejections(self):
        batch = collate_boards([
            self.encode(board(entity("Musketeer", 0, 9000, 16000)), 0),
            self.encode({"tick": 1, "entities": []}, 1),
        ])
        self.assertEqual(tuple(batch.type_ids.shape), (2, MAX_ENTITIES))
        self.assertEqual(tuple(batch.raster.shape), (2, 8, TILE_ROWS, TILE_COLS))
        self.assertEqual(int(batch.mask.sum()), 1)
        with self.assertRaises(ValueError):
            collate_boards([])
        with self.assertRaises(ValueError):
            self.encode(board(entity("HogRider", 0, -1, 0)), 0)
        with self.assertRaises(ValueError):
            self.encode(board(entity("HogRider", 0, 1000, 1000, hp=5, max_hp=4)), 0)
        with self.assertRaises(ValueError):
            self.encode(board(entity("HogRider", 2, 1000, 1000)), 0)


if __name__ == "__main__":
    unittest.main()
