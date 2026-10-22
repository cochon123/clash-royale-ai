"""Integration check against the recovered validator and submission path."""
import unittest

from .banana_train import JsonlEngine, ROOT


class NativePlacementMask(unittest.TestCase):
    def test_query_preserves_native_trajectory_and_filters_rejected_hog_target(self):
        binary = ROOT / "native-engine/target/release/native-watch"
        checkpoint = ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"
        if not binary.exists() or not checkpoint.exists():
            self.skipTest("native engine assets unavailable")
        engines = [JsonlEngine(checkpoint, binary) for _ in range(2)]
        try:
            for engine in engines:
                engine.request(dict(op="reset", deck_preset="hog26", deck_preset_side1="log-bait", seed=44219, max_ticks=7300))
                engine.request(dict(op="step", actions=[], ticks=100))
            before = engines[0].request(dict(op="observe"))
            preview = engines[0].request(dict(op="validate_target", side=0, slot=0, x=1406, y=15111))
            self.assertFalse(preview['valid'])
            self.assertEqual(before['observation'], engines[0].request(dict(op='observe'))['observation'])
            result = engines[0].request(dict(op="placement_mask", side=0, slot=0))
            repeated = engines[0].request(dict(op="placement_mask", side=0, slot=0))
            after = engines[0].request(dict(op="observe"))
            self.assertEqual(before["observation"], after["observation"])
            self.assertEqual(result["mask"], repeated["mask"])
            self.assertEqual(len(result["mask"]), 576)
            self.assertFalse(result["mask"][8 * 32 + 2])
            # The exact target rejected in the user's watched match.
            bad = engines[0].request(dict(op="step", actions=[dict(side=0, slot=0, x=1406, y=15111)], ticks=1))
            self.assertFalse(bad["actions"][0]["accepted"])
            engines[1].request(dict(op="step", actions=[], ticks=1))
            tile = next(i for i, valid in enumerate(result["mask"]) if valid)
            action = dict(side=0, slot=0, x=round((tile % 32 + .5) * 18000 / 32), y=round((tile // 32 + .5) * 32000 / 18))
            replies = [engine.request(dict(op="step", actions=[action], ticks=20)) for engine in engines]
            for reply in replies:
                self.assertTrue(reply["actions"][0]["accepted"])
            self.assertEqual(replies[0]["observation"], replies[1]["observation"])
            fields = ("native_entity_id", "row", "side", "x", "y", "hp", "max_hp")
            entities = lambda reply: [{k: entity.get(k) for k in fields} for entity in reply["frame"]["entities"]]
            self.assertEqual(entities(replies[0]), entities(replies[1]))
            # Side 1 masks use the policy's flipped-y coordinates as well.
            red = engines[0].request(dict(op="placement_mask", side=1, slot=0))
            tile = next(i for i, valid in enumerate(red["mask"]) if valid)
            action = dict(side=1, slot=0, x=round((tile % 32 + .5) * 18000 / 32), y=round((1 - (tile // 32 + .5) / 18) * 32000))
            self.assertTrue(engines[0].request(dict(op="step", actions=[action], ticks=1))["actions"][0]["accepted"])
        finally:
            for engine in engines:
                engine.close()
                if engine.proc.stdout:
                    engine.proc.stdout.close()


if __name__ == "__main__":
    unittest.main()
