"""Meaningful read-only/status/partial-write checks; no training or service mutations."""
from __future__ import annotations

import json
import tempfile
import time
import unittest
from datetime import datetime, timezone
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

from webapps.common.httpkit import HTTPError
from webapps.lab import scanners as s
from webapps.lab.__main__ import document, offline_report, safe_document


class StatusTests(unittest.TestCase):
    def test_only_the_executed_entrypoint_counts_as_a_trainer(self):
        self.assertEqual(s.process_category(['python', '-u', '-m', 'rl.banana_hog_adaptive']), 'training')
        self.assertEqual(s.process_category(['python', 'engine/rl/banana_fast_train.py']), 'training')
        self.assertEqual(s.process_category(['python', '-m', 'rl.hog2_live']), 'pipeline')
        self.assertEqual(s.process_category(['python', '-m', 'rl.banana_watch']), 'watch')
        self.assertIsNone(s.process_category(['python', '-m', 'unittest', 'rl.test_banana_fast_train']))
        self.assertIsNone(s.process_category(['python', '-m', 'rl.test_banana_fast_train']))
        self.assertIsNone(s.process_category(['python', '-c', 'rl.banana_train', '-m', 'rl.banana_train']))
        self.assertEqual(s.process_category(['cr-replays', 'train-policy-v9']), 'training')
        self.assertEqual(s.process_category(['python', '/venv/bin/cr-replays', 'collect']), 'pipeline')

    def test_completed_and_dashboard_are_not_training(self):
        dashboard = {'category': 'pipeline', 'argv': ['python', '-m', 'rl.hog2_live', '--run-dir', str(s.paths.ENGINE_RUNS / 'example')]}
        units = [{'name': 'example-dashboard.service', 'active': 'active', 'sub': 'running'}]
        result = s.run_status({'status': 'completed'}, True, [dashboard], units, 'example')
        self.assertEqual(result['status'], 'finished')

    def test_live_trainer_overrides_completed_and_stop(self):
        trainer = {'category': 'training', 'argv': ['python', '-m', 'rl.banana_train', '--run-dir', str(s.paths.ENGINE_RUNS / 'example')]}
        result = s.run_status({'status': 'completed'}, True, [trainer], [], 'example')
        self.assertEqual(result['status'], 'running')
        self.assertEqual(len(result['evidence']), 3)
        other = {**trainer, 'argv': [str(s.paths.ENGINE_RUNS / 'example-longer')]}
        self.assertEqual(s.run_status({}, False, [other], [], 'example')['status'], 'unknown')

    def test_trainer_belongs_to_output_run_not_parent_checkpoint(self):
        trainer = {'category': 'training', 'cwd': str(s.paths.ENGINE),
                   'argv': ['python', '-m', 'rl.banana_hog_adaptive', '--run-dir', 'runs/new',
                            '--parent-checkpoint', str(s.paths.ENGINE_RUNS / 'parent/models/final.pt')]}
        self.assertEqual(s.run_status({}, False, [trainer], [], 'new')['status'], 'running')
        self.assertEqual(s.run_status({'status': 'completed'}, False, [trainer], [], 'parent')['status'], 'finished')
        trainer['argv'][3:5] = ['--run-dir=runs/new']
        self.assertEqual(s.run_status({}, False, [trainer], [], 'new')['status'], 'running')

    def test_active_unit_freshness_deadline_and_stale(self):
        units = [{'name': 'example-train.service', 'active': 'active', 'sub': 'running'}]
        self.assertEqual(s.run_status({}, False, [], units, 'example')['status'], 'running')
        state = {'status': 'training', 'updated_at': 1000, 'deadline': 2000}
        self.assertEqual(s.run_status(state, False, [], [], 'example', 1050)['status'], 'running')
        self.assertEqual(s.run_status(state, False, [], [], 'example', 2200)['status'], 'stale')
        self.assertEqual(s.run_status(state, True, [], [], 'example', 1050)['status'], 'stopped')
        state['updated_at'] = ['broken']
        self.assertEqual(s.run_status(state, False, [], [], 'example', 1050)['status'], 'stale')

    def test_unknown_when_observation_sources_fail_or_age(self):
        with patch.object(s, 'live', return_value={'processes': [], 'units': [], 'process_scan_ok': False, 'unit_scan_ok': False}), patch.object(s, 'runs', return_value={'runs': []}):
            self.assertEqual(s.overview()['training_status'], 'unknown')
        with patch.object(s, 'live', return_value={'processes': [], 'units': [], 'process_scan_ok': True, 'unit_scan_ok': True, 'sampled_at': '2000-01-01T00:00:00Z'}), patch.object(s, 'runs', return_value={'runs': []}):
            self.assertEqual(s.overview()['training_status'], 'unknown')

    def test_stale_sampler_activity_cannot_keep_training_alive(self):
        trainer = {'category': 'training', 'argv': ['python', '-m', 'rl.banana_train', '--run-dir', str(s.paths.ENGINE_RUNS / 'example')]}
        old = {'sampled_at': time.time() - 60, 'processes': [trainer],
               'units': [{'name': 'example-train.service', 'active': 'active', 'sub': 'running'}],
               'process_scan_ok': True, 'unit_scan_ok': True}
        row = {'id': 'example', 'source': 'engine/runs/example', 'kind': 'training run',
               'stop': False, 'modified_at': None, 'state': {'status': 'training', 'updated_at': time.time() - 3600}, 'launch': {}}
        self.assertEqual(s.run_summary(row, old)['status'], 'stale')
        with patch.object(s, 'live', return_value=old), patch.object(s, 'runs', return_value={'runs': []}):
            self.assertEqual(s.overview()['training_status'], 'unknown')
        fresh = {**old, 'sampled_at': time.time()}
        self.assertEqual(s.run_summary(row, fresh)['status'], 'running')
        with patch.object(s, 'live', return_value=fresh), patch.object(s, 'runs', return_value={'runs': []}):
            self.assertEqual(s.overview()['training_status'], 'running')


class ReaderTests(unittest.TestCase):
    def setUp(self):
        # All fixture writes, including temporary files, stay under webapps/lab/.
        self.temp = tempfile.TemporaryDirectory(prefix='.test-', dir=Path(__file__).parent)
        self.root = Path(self.temp.name)

    def tearDown(self):
        self.temp.cleanup()

    def test_partial_and_bounded_jsonl(self):
        path = self.root / 'partial.jsonl'
        path.write_bytes(b'{"at":1,"loss":2}\nnot-json\n{"at":2')
        before = path.stat().st_mtime_ns, path.read_bytes()
        result = s.jsonl(path)
        self.assertEqual(len(result['rows']), 1)
        self.assertEqual(result['malformed'], 2)
        self.assertEqual(before, (path.stat().st_mtime_ns, path.read_bytes()))
        path.write_text(''.join(json.dumps({'n': n}) + '\n' for n in range(100)))
        result = s.jsonl(path, max_bytes=200, max_rows=4)
        self.assertTrue(result['partial'])
        self.assertLessEqual(result['bytes_read'], 200)
        self.assertEqual(result['rows'][-1]['n'], 99)
        self.assertEqual(len(result['rows']), 4)
        self.assertEqual(s.jsonl(self.root / 'missing')['rows'], [])

    def test_malformed_state_and_model_stats_do_not_crash(self):
        runs = self.root / 'runs'
        run = runs / 'banana-fixture'
        run.mkdir(parents=True)
        state = run / 'state.json'
        with patch.object(s.paths, 'ENGINE_RUNS', runs):
            for value in ['{half', '[]', '{"models":{"hog":{"stats":[1]}},"evaluation":{"history":[null,3]},"status":null}']:
                state.write_text(value)
                row = s.run_scan()[0]
                summary = s.run_summary(row, {'processes': [], 'units': []})
                self.assertIn(summary['status'], {'unknown', 'stale'})

    def test_legacy_path_mapping_and_nonfinite_json(self):
        p = self.root / 'report.json'
        p.write_text(json.dumps({'checkpoint': s.OLD_ROOT + '/runs/a/models/final.pt', 'score': float('nan')}))
        value = s.obj(p)
        self.assertEqual(value['checkpoint'], str(s.paths.ENGINE / 'runs/a/models/final.pt'))
        self.assertIsNone(value['score'])

    def test_document_confinement_and_no_active_content(self):
        docs = self.root / 'docs'
        docs.mkdir()
        path = docs / 'test.md'
        path.write_text('# Test\n\n**Readable**\n\n<script>alert(1)</script>\n\n![remote](https://example.org/pixel)\n\n[bad](javascript:alert(1))')
        with patch.object(s.paths, 'DOCS', docs):
            self.assertEqual(safe_document(str(path)), path.resolve())
            doc = document(path)
            self.assertIn('<strong>Readable</strong>', doc['html'])
            self.assertNotIn('<script', doc['html'])
            self.assertNotIn('<img', doc['html'])
            self.assertNotIn('javascript:', doc['html'])
            with self.assertRaises(HTTPError):
                safe_document(str(self.root / 'outside.md'))
            symlink = docs / 'escape.md'
            target = self.root / 'outside.md'
            target.write_text('outside')
            symlink.symlink_to(target)
            with self.assertRaises(HTTPError):
                safe_document(str(symlink))

    def test_cache_does_not_rescan_until_expired(self):
        calls = []
        key = 'test-cache-' + self.temp.name
        def scan():
            calls.append(True)
            return {'count': len(calls)}
        self.assertEqual(s.cached(key, 60, scan), s.cached(key, 60, scan))
        self.assertEqual(len(calls), 1)
        s.CACHE.pop(key, None)

    def test_served_report_stays_offline_without_editing_original(self):
        path = self.root / 'report.html'
        original = '<link href="https://fonts.googleapis.com/font"><link href="local.css"><script>const chart=1;</script>'
        path.write_text(original)
        with patch.object(s.paths, 'REPORTS', self.root):
            response = offline_report(SimpleNamespace(params={'file': 'report.html'}))
        self.assertNotIn('fonts.googleapis', response.body.decode())
        self.assertIn('local.css', response.body.decode())
        self.assertIn('const chart=1', response.body.decode())
        self.assertIn('Content-Security-Policy', response.headers)
        self.assertEqual(path.read_text(), original)


class OutcomeSpeedTests(unittest.TestCase):
    def test_speeds_are_trailing_window_derivatives(self):
        base = 1_700_000_000.0

        def row(offset, result):
            return {'at': datetime.fromtimestamp(base + offset, timezone.utc).isoformat(), 'result': result}

        records = [row(0, 'W'), row(60, 'W'), row(120, 'L'), row(700, 'W'), row(710, 'VOID'), row(710, None)]
        produced = s.outcome_speed_series('training', records, 'matches.jsonl', window=600.)
        self.assertEqual([c['name'] for c in produced],
                         ['training · wins/min', 'training · losses/min', 'training · net wins/min'])
        wins, losses, net = (c['points'] for c in produced)
        self.assertTrue(all(len(c['points']) == 4 for c in produced))  # VOID/unknown never emit points
        self.assertAlmostEqual(wins[2][1], 2 * 60.0 / 120.0)   # window at t=120: 2 W over a 120 s span
        self.assertAlmostEqual(losses[2][1], 1 * 60.0 / 120.0)
        self.assertAlmostEqual(net[2][1], 60.0 / 120.0)
        self.assertAlmostEqual(wins[3][1], 1 * 60.0 / 580.0)   # at t=700 the two early wins left the window
        self.assertAlmostEqual(losses[3][1], 1 * 60.0 / 580.0)
        self.assertAlmostEqual(net[3][1], 0.0)

    def test_speed_series_skip_incomplete_timestamps(self):
        produced = s.outcome_speed_series('training', [{'result': 'W'}, {'at': 'not-a-date', 'result': 'L'}], 'matches.jsonl')
        self.assertEqual(produced[0]['points'], [])


if __name__ == '__main__':
    unittest.main()
