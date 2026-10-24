"""Rule, comparison and verdict checks for tools/visual_check.py on synthetic traces."""
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import visual_check as vc  # noqa: E402


def make_assets(root: Path, logic=None, clips=None, cards=None):
    documents = [{"name": "chr_test", "animation_exports": clips or []}]
    manifest = {"documents": documents, "scene": {"observed_decks": {"cards_observed_across_traces": cards or []}}}
    (root / "assets.json").write_text(json.dumps(manifest))
    rows = {name: {"fields": fields} for name, fields in (logic or {}).items()}
    (root / "presentation.json").write_text(json.dumps({"rows": rows, "effects": {}}))
    return vc.Assets(root)


def clip(export, token, frames=60, fps=60, labels=None):
    return {"export": export, "action_token": token, "frames": frames, "fps": fps,
            "labels": [{"name": k, "frame": v} for k, v in (labels or {}).items()]}


GHOST_LOGIC = {"BlueExportName": "ghost_blue", "RedExportName": "ghost_red", "DeathEffect": "ghost_die",
               "HitSpeed": 1800, "LoadTime": 1200, "DeployTime": 1000, "BuffWhenNotAttacking": "Invisibility",
               "BuffWhenNotAttackingTime": 2000, "HideTimeMs": 400}


def shown(uid=1, row="Ghost", state="idle", export="ghost_blue_idle1_1", frame=0, address=100, **extra):
    s = {"uid": uid, "address": address, "native_id": 7, "row": row, "side": 0, "kind": "unit", "state": state,
         "export": export, "frame": frame, "flip": False, "deploying": False, "flash": None, "alpha": 1.0,
         "x": 0, "y": 0, "z": 0, "hp": 10, "max_hp": 10, "anchor": [50, 50], "bbox": [40, 30, 60, 60],
         "coverage": 0, "parts": 1}
    s.update(extra)
    return s


def record(i, t, shown_list, tex=None, effects=(), impacts=()):
    draws = [{"kind": "sprite", "id": f"entity-{s['uid']}-part-0-0", "tex": tex or "big", "m": [1, 0, 0, 1, 0, 0],
              "alpha": 1.0, "blend": "normal", "add": [0, 0, 0]} for s in shown_list]
    return {"i": i, "t": t, "observed_tick": int(t), "draws": draws, "shown": shown_list,
            "effects": list(effects), "impacts": list(impacts), "particles": 0}


def lives_for(first, last, final, address=100, row="Ghost"):
    life = {"row": row, "side": 0, "first": first, "last": last, "max_hp": 10,
            "pos": {t: (0, 0, 0, 10) for t in range(first, last + 1)}}
    life["removed"] = last + 1 if last < final else None
    return {(address, 7): life}


class Fixture(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        self.textures = self.root / "textures"
        self.textures.mkdir()
        from PIL import Image
        Image.new("RGBA", (40, 40), (255, 255, 255, 255)).save(self.textures / "big.png")
        Image.new("RGBA", (10, 10), (255, 255, 255, 255)).save(self.textures / "small.png")
        Image.new("RGBA", (40, 40), (255, 255, 255, 80)).save(self.textures / "faint.png")
        vc.TEXTURE_MASS.cache.clear()

    def tearDown(self):
        self.tmp.cleanup()

    def rules(self, findings):
        return sorted({f["rule"] for f in findings.sorted()})


class ArtIdentity(Fixture):
    def test_attached_character_family_is_foreign(self):
        assets = make_assets(self.root, logic={"Ghost": GHOST_LOGIC, "GhostOverlay": {
            "BlueExportName": "ghost_blue_overlay", "RedExportName": "ghost_red_overlay"}},
            clips=[clip("ghost_blue_run1_1", "run")])
        self.assertIn("GhostOverlay", assets.foreign("Ghost", "ghost_blue_overlay_run1_1"))
        self.assertIsNone(assets.foreign("Ghost", "ghost_blue_run1_1"))
        self.assertIsNone(assets.foreign("GhostOverlay", "ghost_blue_overlay_run1_1"))

    def test_unknown_word_after_stem_is_foreign_without_owner_row(self):
        assets = make_assets(self.root, logic={"Ghost": GHOST_LOGIC},
                             clips=[clip("ghost_blue_run1_1", "run"), clip("ghost_blue_idle1_1", "idle")])
        self.assertIn("overlay", assets.foreign("Ghost", "ghost_blue_overlay_run1_1"))
        self.assertIsNone(assets.foreign("Ghost", "ghost_blue_idle1_3"))


class Continuity(Fixture):
    def test_visible_area_jump_and_its_severity(self):
        assets = make_assets(self.root, logic={"Ghost": GHOST_LOGIC})
        trace = [record(0, 100, [shown()]), record(1, 101, [shown(state="moving", export="ghost_blue_run1_1")], tex="small")]
        findings, *_ = vc.analyze({"name": "s"}, assets, trace, lives_for(90, 200, 300), self.textures)
        jump = [f for f in findings.sorted() if f["rule"] == "visible-area-jump"]
        self.assertEqual(len(jump), 1)
        self.assertEqual(jump[0]["severity"], "error")  # 1600 -> 100 px is x16
        self.assertEqual(jump[0]["evidence"]["ratio"], 16.0)

    def test_deploy_filter_frames_are_not_area_jumps(self):
        assets = make_assets(self.root, logic={"Ghost": GHOST_LOGIC})
        trace = [record(0, 100, [shown(deploying=True)], tex="small"), record(1, 101, [shown()])]
        findings, *_ = vc.analyze({"name": "s"}, assets, trace, lives_for(90, 200, 300), self.textures)
        self.assertNotIn("visible-area-jump", self.rules(findings))

    def test_pop_only_while_engine_alive(self):
        assets = make_assets(self.root, logic={"Ghost": GHOST_LOGIC})
        trace = [record(0, 100, [shown()]), record(1, 101, []), record(2, 102, []), record(3, 103, [shown()])]
        findings, *_ = vc.analyze({"name": "s"}, assets, trace, lives_for(90, 200, 300), self.textures)
        self.assertIn("pop", self.rules(findings))
        gone = {(100, 7): dict(lives_for(90, 200, 300)[(100, 7)], pos={100: (0, 0, 0, 10), 103: (0, 0, 0, 10)})}
        findings, *_ = vc.analyze({"name": "s"}, assets, trace, gone, self.textures)
        self.assertNotIn("pop", self.rules(findings))

    def test_one_frame_clip_flicker(self):
        assets = make_assets(self.root, logic={"Ghost": GHOST_LOGIC})
        exports = ["ghost_blue_run1_1", "ghost_blue_idle1_1", "ghost_blue_run1_1", "ghost_blue_run1_1"]
        trace = [record(i, 100 + i, [shown(export=e, state="moving" if "run" in e else "idle")])
                 for i, e in enumerate(exports)]
        findings, *_ = vc.analyze({"name": "s"}, assets, trace, lives_for(90, 200, 300), self.textures)
        self.assertIn("flicker", self.rules(findings))

    def test_loop_wrap_is_not_a_restart_but_a_mid_clip_jump_is(self):
        assets = make_assets(self.root, logic={"Ghost": GHOST_LOGIC}, clips=[clip("ghost_blue_idle1_1", "idle", 60, 60)])
        wrap = [record(0, 100, [shown(frame=57)]), record(1, 101, [shown(frame=0)])]
        findings, *_ = vc.analyze({"name": "s"}, assets, wrap, lives_for(90, 200, 300), self.textures)
        self.assertNotIn("anim-restart", self.rules(findings))
        jump = [record(0, 100, [shown(frame=30)]), record(1, 101, [shown(frame=2)])]
        findings, *_ = vc.analyze({"name": "s"}, assets, jump, lives_for(90, 200, 300), self.textures)
        self.assertIn("anim-restart", self.rules(findings))


class Lifecycle(Fixture):
    def test_death_effect_named_by_logic_must_play(self):
        assets = make_assets(self.root, logic={"Ghost": GHOST_LOGIC})
        trace = [record(i, 100 + i, [shown()] if i < 5 else []) for i in range(10)]
        lives = lives_for(90, 104, 300)
        findings, *_ = vc.analyze({"name": "s"}, assets, trace, lives, self.textures)
        self.assertIn("vanish-without-effect", self.rules(findings))
        death = {"name": "ghost_die", "x": 0, "y": 0, "z": 0, "red": False, "start_ms": 105 * 50, "attach": None,
                 "known": True, "components": 1, "depth": 0}
        trace[5]["effects"] = [death]
        findings, *_ = vc.analyze({"name": "s"}, assets, trace, lives, self.textures)
        self.assertNotIn("vanish-without-effect", self.rules(findings))

    def test_invisibility_must_fade_after_not_attacking(self):
        assets = make_assets(self.root, logic={"Ghost": GHOST_LOGIC})
        # Deployed at tick 10; deploy ends at 30; invisible from ~78 (2000 + 400 ms later).
        solid = [record(i, 10 + i, [shown()]) for i in range(120)]
        findings, *_ = vc.analyze({"name": "s"}, assets, solid, lives_for(10, 300, 400), self.textures)
        self.assertIn("invisibility-not-shown", self.rules(findings))
        faded = [record(i, 10 + i, [shown()], tex="big" if i < 60 else "faint") for i in range(120)]
        findings, *_ = vc.analyze({"name": "s"}, assets, faded, lives_for(10, 300, 400), self.textures)
        self.assertNotIn("invisibility-not-shown", self.rules(findings))


class BaselineAndVerdict(unittest.TestCase):
    def test_changes_are_attributed_to_rows(self):
        base = [record(0, 1, [shown(uid=3, row="Knight", export="knight_idle1_1")])]
        new = [record(0, 1, [shown(uid=3, row="Knight", export="knight_run1_1")])]
        new[0]["draws"][0]["tex"] = "other"
        diff = vc.compare_traces(base, new, {3: "Knight"}, {3: "Knight"})
        self.assertEqual(diff["changed_frames"], [0])
        self.assertEqual(diff["rows"], {"Knight": 1})
        self.assertIn("export: knight_idle1_1 -> knight_run1_1", diff["details"]["Knight"])
        self.assertEqual(vc.compare_traces(base, base, {}, {})["changed_frames"], [])

    def test_verdict_gates_new_errors_and_unexpected_rows(self):
        with tempfile.TemporaryDirectory() as tmp:
            baseline = Path(tmp)
            (baseline / "s").mkdir()
            old = {"key": "s/pop/Knight#3", "severity": "error", "count": 1}
            (baseline / "s" / "scenario.json").write_text(json.dumps({"findings": [old]}))

            def result(findings, rows=None):
                return [{"name": "s", "findings": findings, "comparison": {"rows": rows or {}}}]
            same = [dict(old)]
            self.assertEqual(vc.verdict(result(same), baseline, None, False)[0], "PASS")
            new = [dict(old), {"key": "s/pop/Knight#4", "severity": "error", "count": 1}]
            status, reasons, *_ = vc.verdict(result(new), baseline, None, False)
            self.assertEqual(status, "FAIL")
            self.assertIn("1 new error", reasons[0])
            status, _, fixed, _ = vc.verdict(result([]), baseline, None, False)
            self.assertEqual((status, len(fixed)), ("PASS", 1))
            status, _, _, unexpected = vc.verdict(result([], {"Ghost": 4, "Knight": 2}), baseline, ["Ghost"], False)
            self.assertEqual((status, unexpected), ("FAIL", {"s": {"Knight": 2}}))
            warn = [{"key": "s/flicker/Knight#3", "severity": "warning", "count": 1}]
            self.assertEqual(vc.verdict(result(warn), baseline, None, False)[0], "PASS")
            self.assertEqual(vc.verdict(result(warn), baseline, None, True)[0], "FAIL")


class HarnessSafeguards(unittest.TestCase):
    def test_expected_rows_require_baseline_comparisons(self):
        result = [{"name": "s", "findings": [], "comparison": None}]
        self.assertEqual(vc.verdict(result, None, ["Ghost"], False)[0], "FAIL")
        self.assertEqual(vc.verdict(result, Path('/unused'), [], False)[0], "FAIL")

    def test_strict_gates_unbaselined_warnings(self):
        result = [{"name": "s", "findings": [{"key": "s/flicker/Ghost#1", "severity": "warning", "count": 1}], "comparison": None}]
        self.assertEqual(vc.verdict(result, None, None, True)[0], "FAIL")

    def test_worsened_existing_findings_fail(self):
        with tempfile.TemporaryDirectory() as tmp:
            baseline = Path(tmp)
            (baseline / 's').mkdir()
            old = {"key": "s/visible-area-jump/Ghost#1", "severity": "warning", "count": 1}
            (baseline / 's/scenario.json').write_text(json.dumps({"findings": [old]}))
            for severity, count, strict in [('error', 1, False), ('warning', 2, True)]:
                finding = dict(old, severity=severity, count=count)
                result = [{"name": "s", "findings": [finding], "comparison": {"rows": {}}}]
                self.assertEqual(vc.verdict(result, baseline, None, strict)[0], 'FAIL')
                self.assertEqual(finding['status'], 'regressed')

    def test_time_grid_cannot_silently_skip_frames(self):
        trace = [record(i, i, []) for i in range(3)]
        for changed in [trace[:-1], trace + [record(3, 3, [])], [record(0, 0.5, [])]]:
            with self.assertRaisesRegex(RuntimeError, 'display-time grid'):
                vc.compare_traces(trace, changed, {}, {})

    def test_small_transform_and_draw_order_changes_are_detected(self):
        import copy
        base = [record(0, 1, [shown(uid=3), shown(uid=4, row='Knight')])]
        changed = copy.deepcopy(base)
        changed[0]['draws'][0]['m'][4] += 0.0001
        self.assertEqual(vc.compare_traces(base, changed, {3:'Ghost',4:'Knight'}, {})['rows'], {'Ghost':1})
        changed = copy.deepcopy(base)
        changed[0]['draws'].reverse()
        diff = vc.compare_traces(base, changed, {}, {})
        self.assertEqual(diff['rows'], {'effects/HUD':1})
        self.assertEqual(diff['details']['effects/HUD'], {'draw order changed':1})

    def test_no_build_checks_contents_even_with_preserved_mtimes(self):
        import os
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            binary, source = root/'binary', root/'source.rs'
            binary.write_bytes(b'original binary')
            source.write_text('original source')
            with patch.object(vc, 'BINARY', binary), patch.object(vc, 'source_paths', return_value=[source]), patch.object(vc, 'ENGINE', root):
                self.assertTrue(vc.binary_is_stale())
                vc.build_stamp().write_text(json.dumps({'source':vc.source_fingerprint(), 'binary':vc.file_hash(binary)}))
                self.assertFalse(vc.binary_is_stale())
                stat = source.stat()
                source.write_text('modified source')
                os.utime(source, ns=(stat.st_atime_ns, stat.st_mtime_ns))
                self.assertTrue(vc.binary_is_stale())
                source.write_text('original source')
                binary.write_bytes(b'regression binary')
                self.assertTrue(vc.binary_is_stale())

    def test_accepting_fail_needs_reason_and_cannot_delete_own_source(self):
        import argparse
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            run = root/'runs/example'
            run.mkdir(parents=True)
            (run/'report.json').write_text(json.dumps({'verdict':'FAIL'}))
            with patch.object(vc, 'STATE', root):
                with self.assertRaisesRegex(RuntimeError, 'requires --note'):
                    vc.cmd_accept(argparse.Namespace(run=str(run), note=''))
                vc.cmd_accept(argparse.Namespace(run=str(run), note='Known pre-existing findings'))
                with self.assertRaisesRegex(RuntimeError, 'outside'):
                    vc.cmd_accept(argparse.Namespace(run=str(root/'baseline'), note='test'))
                self.assertTrue((root/'baseline/report.json').exists())
                (run/'report.json').write_text(json.dumps({'verdict':'PASS'}))
                self.assertEqual(json.loads((root/'baseline/report.json').read_text())['verdict'], 'FAIL')


class MemoryBudget(unittest.TestCase):
    def test_unsafe_parallel_budget_is_rejected_before_building(self):
        import contextlib
        import io
        from unittest.mock import patch
        with patch.object(vc, 'available_memory_gb', return_value=8), patch.object(vc, 'build_binary') as build:
            with contextlib.redirect_stderr(io.StringIO()) as stderr:
                code = vc.main(['run', '--suite', 'ghost', '--jobs', '2', '--memory-gb', '4'])
            self.assertEqual(code, 2)
            self.assertIn('reserve 4 GiB', stderr.getvalue())
            build.assert_not_called()

    @unittest.skipUnless(sys.platform == 'linux', 'Linux address-space cap')
    def test_worker_cap_prevents_large_allocation(self):
        import subprocess
        code = """
import sys
sys.path.insert(0, sys.argv[1])
import visual_check as vc
vc.worker_memory_limit(1)
try:
    value = bytearray(2 * 1024**3)
except MemoryError:
    print('bounded')
else:
    raise SystemExit('allocation was not bounded')
"""
        result = subprocess.run([sys.executable, '-c', code, str(Path(vc.__file__).parent)],
                                capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout.strip(), 'bounded')


if __name__ == "__main__":
    unittest.main()
