"""Source-availability coverage for original spectator documents."""
import json
import tempfile
import unittest
from pathlib import Path

from tools.asset_coverage import (
    ALWAYS_STEMS, apply_bundle_checks, coverage_status, load_spell_cards, output_is_readonly_tree,
    parse_cards_argument, plan_coverage, selectable_native_cards,
)
from tools.spectator_presentation import DEFAULT_APK, DEFAULT_LOGIC, ROW_LINK_FIELDS, ROOT

ENGINE = ROOT
WORKTREE = ROOT.parent


class CardListTests(unittest.TestCase):
    def test_cards_argument_keeps_internal_spaces(self):
        self.assertEqual(parse_cards_argument("Pekka,Elixir Collector,BabyDragon"),
                         ["Pekka", "Elixir Collector", "BabyDragon"])
        self.assertEqual(parse_cards_argument(" Pekka,, BabyDragon "), ["Pekka", "BabyDragon"])
        self.assertEqual(parse_cards_argument(""), [])

    def test_selectable_cards_are_the_119_with_original_art(self):
        names = selectable_native_cards(ENGINE)
        self.assertEqual(len(names), 119)
        self.assertEqual(len(set(names)), 119)
        for name in ("Pekka", "Golem", "LavaHound", "BabyDragon", "Elixir Collector", "Mirror"):
            self.assertIn(name, names)
        self.assertNotIn("BarbarianHut", names)
        self.assertNotIn("Snowball", names)

    def test_readonly_engine_trees_are_refused(self):
        self.assertTrue(output_is_readonly_tree(ENGINE / "analysis" / "native-spectator", ENGINE))
        self.assertTrue(output_is_readonly_tree(ENGINE / "runs" / "watch", ENGINE))
        self.assertFalse(output_is_readonly_tree(WORKTREE / ".local-arena" / "assets" / "native-spectator", ENGINE))


class SpellRootTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.spells = load_spell_cards(DEFAULT_LOGIC)

    def test_csv_roots_keep_source_hashes(self):
        skeletons = self.spells["Skeletons"]
        summon = next(root for root in skeletons["roots"] if root["field"] == "SummonCharacter")
        self.assertEqual(summon["target"], "Skeleton")
        self.assertEqual(summon["source_file"], "spells_characters.csv")
        self.assertEqual(len(summon["source_sha256"]), 64)
        fireball = next(root for root in self.spells["Fireball"]["roots"] if root["field"] == "Projectile")
        self.assertEqual(fireball["target"], "FireballSpell")
        self.assertEqual(fireball["source_file"], "spells_other.csv")
        gang = {root["field"]: root["target"] for root in self.spells["GoblinGang"]["roots"]}
        self.assertEqual(gang["SummonCharacter"], "Goblin_Stab")
        self.assertEqual(gang["SummonCharacterSecond"], "SpearGoblin")

    def test_toml_summon_list_is_provenance_not_a_guess(self):
        roots = [root for root in self.spells["IceWizard"]["roots"] if root["field"] == "SummonCharactersList"]
        self.assertTrue(any(root["target"] == "IceWizard" and root["source_file"] == "spells_characters.toml" for root in roots))
        self.assertEqual(len(roots[0]["source_sha256"]), 64)

    def test_link_fields_include_spawn_projectile_and_attached_rows(self):
        for field in ("AttachedCharacter", "SummonCharacter", "AreaEffectObject", "CustomFirstProjectile", "DeathSpawnCharacter"):
            self.assertIn(field, ROW_LINK_FIELDS)


class CoverageWalkTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.plan = plan_coverage(
            DEFAULT_LOGIC, DEFAULT_APK, ENGINE,
            "Pekka,Golem,LavaHound,BabyDragon,IceWizard,Mirror",
            frame_paths=[],
        )

    def test_priority_cards_name_original_documents(self):
        docs = self.plan["cards"]
        self.assertIn("chr_pekka", docs["Pekka"]["documents"])
        self.assertIn("chr_golem_dl", docs["Golem"]["documents"])
        self.assertIn("chr_golemite", docs["Golem"]["documents"])
        self.assertIn("chr_lava_hound", docs["LavaHound"]["documents"])
        self.assertIn("chr_lava_pups", docs["LavaHound"]["documents"])
        self.assertIn("chr_baby_dragon", docs["BabyDragon"]["documents"])
        self.assertIn("BabyDragonProjectile", {row["native_row"] for row in docs["BabyDragon"]["source_rows"]})
        self.assertNotIn("GolemDeathExplosion", docs["Golem"]["missing_effects"])
        self.assertNotIn("GolemDeathExplosion", docs["Golem"]["unresolved_rows"])
        self.assertEqual(docs["Pekka"]["documents"], sorted(docs["Pekka"]["documents"]))

    def test_ice_wizard_follows_the_toml_summon_list(self):
        rows = {row["native_row"] for row in self.plan["cards"]["IceWizard"]["source_rows"]}
        self.assertIn("IceWizard", rows)

    def test_mirror_has_no_invented_unit_document(self):
        mirror = self.plan["cards"]["Mirror"]
        self.assertTrue(mirror["spell_found"])
        self.assertEqual(mirror["roots"], [])
        self.assertIsNotNone(mirror["note"])

    def test_always_documents_cover_arena_hud_and_towers(self):
        always = set(self.plan["always_documents"])
        for stem in ALWAYS_STEMS:
            self.assertIn(stem, always)
        self.assertIn("building_tower", always)
        self.assertIn("chr_king", always)
        self.assertIn("ui", always)

    def test_pekka_binding_keeps_the_runtime_row_shape(self):
        binding = next(row for row in self.plan["bindings"] if row["native_row"] == "Pekka")
        for key in ("native_row", "document", "source_file", "blue_export", "red_export", "source_fields", "match_basis"):
            self.assertIn(key, binding)
        self.assertEqual(binding["document"], "chr_pekka")
        self.assertEqual(binding["blue_export"], "pekka")
        self.assertEqual(binding["red_export"], "pekka_enemy")

    def test_selective_cards_do_not_pull_unrequested_card_documents(self):
        pekka_only = plan_coverage(DEFAULT_LOGIC, DEFAULT_APK, ENGINE, "Pekka", frame_paths=[])
        self.assertIn("chr_pekka", pekka_only["documents"])
        self.assertNotIn("chr_golem_dl", pekka_only["documents"])
        self.assertEqual(set(pekka_only["cards"]), {"Pekka"})

    def test_unknown_card_stays_unresolved(self):
        plan = plan_coverage(DEFAULT_LOGIC, DEFAULT_APK, ENGINE, "Pekka,ZzzNotACard", frame_paths=[])
        self.assertEqual(plan["cards"]["ZzzNotACard"]["spell_found"], False)
        self.assertNotIn("ZzzNotACard", {
            name: docs for name, docs in (
                (name, card["documents"]) for name, card in plan["cards"].items() if card["spell_found"]
            )
        })


class FrameAliasTests(unittest.TestCase):
    def test_fireball_frame_row_resolves_through_its_projectile_root(self):
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder) / "frames.jsonl"
            path.write_text(json.dumps({"entities": [{"row": "Fireball"}], "hands": []}) + "\n")
            plan = plan_coverage(DEFAULT_LOGIC, DEFAULT_APK, ENGINE, "Pekka", [path])
        binding = next(row for row in plan["bindings"] if row["native_row"] == "Fireball")
        self.assertEqual(binding["alias_of"], "FireballSpell")
        self.assertIn("sha256=", binding["match_basis"])
        self.assertIn("FireballSpell", {row["native_row"] for row in plan["bindings"]})


class StatusTests(unittest.TestCase):
    def test_status_is_source_availability(self):
        self.assertEqual(coverage_status(False, [], [], [], [], []), "unresolved")
        self.assertEqual(coverage_status(True, [], [], [], [], []), "complete")
        self.assertEqual(coverage_status(True, ["chr_pekka"], [], [], [], []), "partial")

    def test_bundle_check_records_hash_and_missing_export(self):
        record = {
            "spell_found": True,
            "documents": ["chr_pekka"],
            "unresolved_rows": [],
            "missing_effects": [],
            "missing_emitters": [],
            "export_refs": [
                {"document": "chr_pekka", "export": "pekka", "role": "blue", "owner": "Pekka"},
                {"document": "chr_pekka", "export": "pekka_enemy", "role": "red", "owner": "Pekka"},
            ],
        }
        checked = apply_bundle_checks(record, {"chr_pekka": {"pekka_run1_1", "pekka_idle1_1"}}, {"chr_pekka": "abc"}, {"chr_pekka": "assets/sc/chr_pekka.sc"})
        self.assertEqual(checked["status"], "partial")
        self.assertEqual(checked["document_checks"][0]["source_sha256"], "abc")
        self.assertEqual(checked["missing_exports"], ["chr_pekka:pekka_enemy"])
        pekka = next(check for check in checked["export_checks"] if check["export"] == "pekka")
        self.assertTrue(pekka["present"])
        self.assertFalse(pekka["exact_export_present"])
        self.assertGreater(pekka["family_count"], 0)
        self.assertEqual(checked["status_basis"].startswith("source availability"), True)


class CapturePresetTests(unittest.TestCase):
    def test_six_preset_decks_are_selectable(self):
        captures = WORKTREE / ".local-arena" / "captures"
        if not captures.is_dir():
            self.skipTest("preset captures are not in this worktree")
        cards = set()
        for path in captures.glob("*/capture.json"):
            data = json.loads(path.read_text())
            for deck in data.get("decks") or []:
                cards.update(deck)
        selectable = set(selectable_native_cards(ENGINE))
        self.assertEqual(len(cards), 39)
        self.assertTrue(cards <= selectable)


if __name__ == "__main__":
    unittest.main()
