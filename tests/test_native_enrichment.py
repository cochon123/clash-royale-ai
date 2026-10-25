from pathlib import Path
from types import SimpleNamespace
import tempfile
import unittest
from unittest.mock import patch

from cr_replay_pipeline.enrichment import inspect_replay
from cr_replay_pipeline.models import ParsedReplay, ReplayEvent
from cr_replay_pipeline.native_enrichment import consistent_queue, run_trace, side_hypothesis
from cr_replay_pipeline.enrichment_corpus import pack_one


def replay(events):
    return ParsedReplay('battle', 'raw.json', None, {}, {},
                        {'team': list('abcdefgh'), 'opponent': list('abcdefgh')}, events, 'hash')


class FakeEngine:
    def __init__(self, reject=False):
        self.requests = []
        self.tick = 0
        self.reject = reject

    def request(self, request):
        self.requests.append(request)
        if request['op'] == 'reset':
            self.tick = 0
        else:
            self.tick += request['ticks']
        return {'observation': {'tick': self.tick, 'hands': [
            [{'name': 'A', 'slot': 0, 'available': True}],
            [{'name': 'A', 'slot': 0, 'available': True}]]},
                'actions': [{'accepted': not self.reject, 'rejection': 'insufficient_mana'}
                            for _ in request.get('actions', [])]}


class NativeEnrichmentTests(unittest.TestCase):
    def test_native_coverage_error_retains_only_the_accepted_prefix(self):
        class CoverageGap(FakeEngine):
            def request(self, request):
                if request['op'] == 'step' and not request.get('actions') and self.tick >= 101:
                    raise RuntimeError('untranslated function 0x1234')
                return super().request(request)
        value = replay([ReplayEvent(100, 5, 'team', 'card_play', 'a', 3500, 14500),
                        ReplayEvent(200, 10, 'opponent', 'card_play', 'a', 3500, 17500)])
        result, frames = run_trace(CoverageGap(), value, dict.fromkeys('abcdefgh', 'A'), 1, 0, save_frames=True)
        self.assertFalse(result['aligned'])
        self.assertEqual(result['accepted'], 1)
        self.assertEqual(result['failure'], 'native_runtime_error')
        self.assertEqual(len(frames), 1)

    def test_full_coordinator_resumes_without_duplicate_skip_records(self):
        import contextlib
        import io
        import json
        import sqlite3
        import sys
        from cr_replay_pipeline.native_enrichment_full import main
        with tempfile.TemporaryDirectory() as root:
            root = Path(root)
            inputs = root/'input.sqlite3'
            with sqlite3.connect(inputs) as db:
                db.execute('CREATE TABLE inputs (file TEXT PRIMARY KEY, metadata TEXT, payload BLOB)')
                db.execute('CREATE TABLE config (key TEXT PRIMARY KEY, value TEXT)')
                db.execute('INSERT INTO config VALUES (?, ?)', ('config', json.dumps({'mapping': {}, 'spells': []})))
                db.execute('INSERT INTO inputs VALUES (?, ?, NULL)', ('bad.json', json.dumps({'outcome': 'skipped', 'skip_reason': 'invalid_input'})))
            binary = root/'binary'
            binary.write_text('No engine execution should be needed for preflight skips')
            argv = ['sweep', '--input-db', str(inputs), '--run-dir', str(root/'run'),
                    '--binary', str(binary), '--checkpoint', str(root)]
            with patch.object(sys, 'argv', argv), patch('signal.signal'), contextlib.redirect_stdout(io.StringIO()):
                main()
                main()
            with sqlite3.connect(root/'run/results.sqlite3') as db:
                self.assertEqual(db.execute('SELECT COUNT(*) FROM results').fetchone()[0], 1)
            state = json.loads((root/'run/state.json').read_text())
            self.assertEqual(state['status'], 'completed')
            self.assertEqual(state['progress']['preflight_skipped'], 1)
            self.assertEqual(state['progress']['attempted'], 0)
            self.assertFalse(state['validation']['training_usable'])

    def test_corpus_prefix_stops_before_every_event_at_first_ability_tick(self):
        import hashlib
        import json
        import zlib
        value = replay([ReplayEvent(100, 5, 'team', 'card_play', 'a', 3500, 14500),
                        ReplayEvent(200, 10, 'opponent', 'ability_activation'),
                        ReplayEvent(200, 10, 'team', 'card_play', 'b', 3500, 14500)])
        with tempfile.TemporaryDirectory() as root:
            path = Path(root)/'raw.json'
            path.write_text('{}')
            audited = {'file': path.name, 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
                       'decks': value.decks, 'battle_id': 'battle', 'content_hash': 'hash'}
            with patch('cr_replay_pipeline.enrichment_corpus.parse_replay', return_value=value):
                meta, payload = pack_one((root, audited, dict.fromkeys('abcdefgh', 'native')))
        self.assertEqual(meta['outcome'], 'candidate')
        self.assertEqual(meta['stop_reason'], 'unsupported_ability_activation')
        self.assertEqual(meta['stop_tick'], 200)
        self.assertEqual([e['ticks'] for e in json.loads(zlib.decompress(payload))['events']], [100])

    def test_aligned_supported_prefix_is_never_reported_as_complete_replay(self):
        import hashlib
        import gzip
        import json
        from cr_replay_pipeline.native_enrichment import attempt
        with tempfile.TemporaryDirectory() as root:
            root = Path(root)
            source = root/'raw.json'
            source.write_text('{}')
            (root/'quarantine').mkdir()
            meta = {'battle_id': 'battle', 'file': source.name,
                    'sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
                    'stop_reason': 'unsupported_ability_activation', 'stop_tick': 200}
            args = SimpleNamespace(source=root, run_dir=root, seeds=1, battle_timeout=90)
            value = replay([ReplayEvent(100, 5, 'team', 'card_play', 'a', 3500, 14500)])
            with patch('cr_replay_pipeline.native_enrichment.parse_replay', return_value=value), \
                 patch('cr_replay_pipeline.native_enrichment.worker_engine'), \
                 patch('cr_replay_pipeline.native_enrichment.inferred_deck_inputs', return_value=None), \
                 patch('cr_replay_pipeline.native_enrichment.run_trace', return_value=({'aligned': True, 'accepted': 1}, [{'event_index': 0}])):
                result = attempt((args, meta, {'a': 'A'}))
            self.assertEqual(result['outcome'], 'partial_unverified')
            with gzip.open(result['output'], 'rt') as f:
                output = json.load(f)
            self.assertFalse(output['alignment_complete'])
            self.assertFalse(output['training_usable'])
            self.assertTrue(output['supported_prefix_aligned'])

    def test_initial_queue_preserves_the_complete_observed_cycle(self):
        from collections import deque
        plays = list('abcdeafgbh')
        initial = consistent_queue(list('abcdefgh'), plays)
        hand, waiting = set(initial[:4]), deque(initial[4:])
        for card in plays:
            self.assertIn(card, hand)
            hand.remove(card)
            hand.add(waiting.popleft())
            waiting.append(card)
        with self.assertRaises(ValueError):
            consistent_queue(list('abcdefgh'), ['a', 'a'])

    def test_unsupported_cards_and_abilities_are_rejected_together(self):
        value = replay([ReplayEvent(123, 6.15, 'team', 'ability_activation')])
        with tempfile.TemporaryDirectory() as root:
            path = Path(root) / 'raw.json'
            path.write_text('{}')
            with patch('cr_replay_pipeline.enrichment.parse_replay', return_value=value):
                record = inspect_replay((str(path), ['a'], set()))
        self.assertEqual(record['outcome'], 'rejected')
        self.assertIn('unsupported_ability_activation', record['reasons'])
        self.assertIn('cards_outside_validated_set', record['reasons'])
        self.assertEqual(len(record['sha256']), 64)

    def test_simultaneous_opposite_side_deployments_are_submitted_together(self):
        value = replay([ReplayEvent(153, 7.65, 'team', 'card_play', 'a', 3500, 14500),
                        ReplayEvent(153, 7.65, 'opponent', 'card_play', 'a', 3500, 17500)])
        engine = FakeEngine()
        result, frames = run_trace(engine, value, {c: c.upper() for c in 'abcdefgh'}, 1, 100, save_frames=True)
        self.assertTrue(result['aligned'])
        self.assertEqual(result['accepted'], 2)
        actions = [r for r in engine.requests if r.get('actions')]
        self.assertEqual(len(actions), 1)
        self.assertEqual(len(actions[0]['actions']), 2)
        self.assertEqual(frames[0]['native_tick'], 253)
        self.assertEqual(frames[0]['replay_tick'], 153)

    def test_rejected_deployment_is_not_forced_or_counted(self):
        value = replay([ReplayEvent(153, 7.65, 'team', 'card_play', 'a', 3500, 14500),
                        ReplayEvent(200, 10, 'opponent', 'card_play', 'a', 3500, 17500)])
        engine = FakeEngine(reject=True)
        result, _ = run_trace(engine, value, {c: c.upper() for c in 'abcdefgh'}, 1, 0)
        self.assertFalse(result['aligned'])
        self.assertEqual(result['accepted'], 0)
        self.assertEqual(result['failure'], 'insufficient_mana')
        self.assertEqual(sum(bool(r.get('actions')) for r in engine.requests), 1)

    def test_ambiguous_side_orientation_fails_explicitly(self):
        value = replay([ReplayEvent(153, 7.65, 'team', 'card_play', 'a', 3500, 14500),
                        ReplayEvent(200, 10, 'opponent', 'card_play', 'a', 3500, 14500)])
        with self.assertRaises(ValueError):
            side_hypothesis(value)


if __name__ == '__main__':
    unittest.main()
