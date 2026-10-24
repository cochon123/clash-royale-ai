"""Scene transport contracts without native/model processes or live matches."""
from __future__ import annotations

import io
import json
from pathlib import Path
import queue
import tempfile
import threading
from types import SimpleNamespace
import unittest
from unittest.mock import MagicMock, patch

from webapps.arena import scene, renderer as renderer_module
from webapps.common.httpkit import App, EventStream, HTTPError, Request


FIRST = 'a' * 64
SECOND = 'b' * 64


class SceneTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.renderer = scene.NativeSceneRenderer.__new__(scene.NativeSceneRenderer)
        self.renderer.run_dir = self.root
        self.renderer.texture_dir = self.root / 'scene-textures'
        self.renderer.texture_dir.mkdir()
        self.renderer.snapshots = queue.Queue(maxsize=scene.MAX_PENDING_SNAPSHOTS)
        self.renderer.condition = threading.Condition()
        self.renderer.closed = threading.Event()
        self.renderer.error = None
        self.renderer.latest = None
        self.renderer.last_snapshot = None
        self.renderer.texture_ids = set()
        self.renderer.texture_sizes = {}
        self.renderer.expired_texture_ids = scene.OrderedDict()
        self.renderer.texture_bytes = 0
        self.renderer.fallback = None
        self.renderer.fallback_lock = threading.Lock()
        self.match = SimpleNamespace(owner='owner-a', error=None, renderer=self.renderer,
                                     subscribers=0, last_watched=0)
        self.app = App('scene-fixture')
        scene.register_routes(self.app, SimpleNamespace(get=lambda name: self.match if name == 'fixture' else None))

    def asset(self, identity=FIRST, payload=b'png!'):
        path = self.renderer.texture_dir / (identity + '.png')
        path.write_bytes(payload)
        return path

    def packet(self, sequence=1, **changes):
        value = dict(v=1, sequence=sequence, tick=100, display_tick=100., match_id=1,
                     width=540, height=900, clear=[19, 27, 31, 255],
                     layout={'field': [43, 125, 454, 648], 'world': [18000, 32000], 'cards': []},
                     textures=[], draws=[])
        value.update(changes)
        return value

    def route(self, path, *, owner='owner-a', headers=None):
        for method, pattern, handler in self.app.routes:
            found = pattern.fullmatch(path)
            if method == 'GET' and found:
                return handler(Request('GET', path, {}, found.groupdict(), headers or {}, session_id=owner))
        raise AssertionError('no matching fixture route')

    def assert_http(self, status, action):
        with self.assertRaises(HTTPError) as caught:
            action()
        self.assertEqual(caught.exception.status, status)
        return caught.exception

    def test_snapshot_queue_preserves_every_tick_and_never_constructs_jpeg(self):
        with patch.object(scene, 'NativeRenderer') as jpeg:
            for tick in range(scene.MAX_PENDING_SNAPSHOTS):
                self.renderer.submit({'tick': tick})
            self.assertEqual(self.renderer.snapshots.qsize(), scene.MAX_PENDING_SNAPSHOTS)
            self.assertEqual([self.renderer.snapshots.get_nowait()['tick']
                              for _ in range(scene.MAX_PENDING_SNAPSHOTS)],
                             list(range(scene.MAX_PENDING_SNAPSHOTS)))
            self.assertEqual(self.renderer.last_snapshot['tick'], scene.MAX_PENDING_SNAPSHOTS - 1)
            jpeg.assert_not_called()

    def test_snapshot_overflow_fails_explicitly_without_silently_dropping_history(self):
        for tick in range(scene.MAX_PENDING_SNAPSHOTS):
            self.renderer.submit({'tick': tick})
        with self.assertRaisesRegex(RuntimeError, 'snapshot queue overflow'):
            self.renderer.submit({'tick': scene.MAX_PENDING_SNAPSHOTS})
        self.assertIn('preserve every engine tick', self.renderer.error)
        with self.assertRaisesRegex(RuntimeError, 'snapshot queue overflow'):
            self.renderer.submit({'tick': scene.MAX_PENDING_SNAPSHOTS + 1})
        self.assertEqual([self.renderer.snapshots.get_nowait()['tick']
                          for _ in range(scene.MAX_PENDING_SNAPSHOTS)],
                         list(range(scene.MAX_PENDING_SNAPSHOTS)))

    def test_writer_delivers_every_accepted_snapshot_in_order_without_pacing_or_drain(self):
        count = 200
        renderer = self.renderer

        class Sink(io.BytesIO):
            writes = 0

            def write(self, value):
                result = super().write(value)
                self.writes += 1
                if self.writes == count:
                    renderer.closed.set()
                return result

        sink = Sink()
        self.renderer.proc = SimpleNamespace(stdin=sink)
        for tick in range(count):
            self.renderer.submit({'tick': tick})
        self.renderer._write()
        self.assertEqual([json.loads(line)['tick'] for line in sink.getvalue().splitlines()], list(range(count)))
        self.assertEqual(self.renderer.snapshots.qsize(), 0)
        self.assertIsNone(self.renderer.error)

    def test_scene_constructor_starts_only_scene_presenter(self):
        process = MagicMock()
        process.poll.return_value = 0
        with patch.object(scene.subprocess, 'Popen', return_value=process) as popen, \
                patch.object(scene.threading, 'Thread') as threads, \
                patch.object(scene, 'NativeRenderer') as jpeg:
            renderer = scene.NativeSceneRenderer(self.root / 'constructor')
            try:
                self.assertIn('--texture-cache', popen.call_args.args[0])
                self.assertIn('--pose-cache', popen.call_args.args[0])
                arguments = popen.call_args.args[0]
                self.assertEqual(arguments[arguments.index('--fps') + 1],
                                 scene.os.environ.get('ARENA_SCENE_FPS', '30'))
                self.assertEqual(renderer.snapshots.maxsize, 256)
                self.assertFalse(hasattr(renderer, 'frame_interval'))
                self.assertEqual(threads.call_count, 2)
                jpeg.assert_not_called()
            finally:
                renderer.close()

    def test_jpeg_is_lazy_explicit_and_receives_current_and_future_snapshots(self):
        fallback = MagicMock()
        fallback.image.return_value = ({'sequence': 1}, b'jpeg')
        fallback.frames.return_value = iter([({'sequence': 2}, b'jpeg2')])
        with patch.object(scene, 'NativeRenderer', return_value=fallback) as jpeg:
            self.renderer.submit({'tick': 100})
            self.renderer._accept(self.packet())
            events = self.renderer.events()
            self.assertEqual(next(events)[0], 'scene')
            events.close()
            jpeg.assert_not_called()
            self.assertEqual(self.renderer.image(17), ({'sequence': 1}, b'jpeg'))
            jpeg.assert_called_once_with(self.root / 'stream-fallback')
            fallback.image.assert_called_once_with(17)
            fallback.submit.assert_called_once_with({'tick': 100})
            self.renderer.submit({'tick': 101})
            self.assertEqual(fallback.submit.call_args.args[0], {'tick': 101})
            self.assertEqual(list(self.renderer.frames()), [({'sequence': 2}, b'jpeg2')])
            self.assertEqual(jpeg.call_count, 1)

    def test_entire_decks_select_original_dependencies_in_both_views(self):
        bundle = self.root / 'assets'
        bundle.mkdir()
        (bundle / 'assets.json').write_text(json.dumps({'scene': {
            'card_dependency_documents': {'Pekka': ['chr_pekka'], 'Golem': ['chr_golem_dl', 'chr_golemite']}}}))
        process = MagicMock()
        process.poll.return_value = 0
        cards = ['Pekka', 'Golem', 'Pekka']
        with patch.object(renderer_module, 'ASSETS', bundle), patch.object(scene, 'ASSETS', bundle), \
                patch.object(scene.subprocess, 'Popen', return_value=process) as popen, \
                patch.object(scene.threading, 'Thread'), patch.object(scene, 'NativeRenderer') as jpeg:
            viewer = scene.NativeSceneRenderer(self.root / 'selected-decks', cards=cards)
            try:
                arguments = popen.call_args.args[0]
                self.assertEqual(arguments[arguments.index('--cards') + 1], 'Pekka,Golem')
                self.assertEqual(arguments[arguments.index('--assets') + 1], str(bundle))
                viewer._fallback()
                jpeg.assert_called_once_with(viewer.run_dir / 'stream-fallback', cards=tuple(cards))
            finally:
                viewer.close()

        # Existing baseline bundles remain usable without a dependency index.
        (bundle / 'assets.json').write_text('{}')
        with patch.object(renderer_module, 'ASSETS', bundle):
            self.assertEqual(renderer_module.asset_selection_args(cards), [])

    def test_failed_explicit_stream_does_not_kill_browser_presentation(self):
        self.renderer.fallback = MagicMock()
        self.renderer.fallback.submit.side_effect = RuntimeError('stream failed')
        self.renderer.submit({'tick': 120})
        self.assertIsNone(self.renderer.error)
        self.assertEqual(self.renderer.snapshots.get_nowait(), {'tick': 120})

    def test_closed_renderer_never_constructs_a_fallback(self):
        self.renderer.closed.set()
        with patch.object(scene, 'NativeRenderer') as jpeg:
            self.assert_http(404, self.renderer.image)
            jpeg.assert_not_called()

    def test_top_level_native_diagnostics_and_hidden_data_are_not_published(self):
        self.renderer.last_snapshot = {'runtime_status': 'paused'}
        self.renderer._accept(self.packet(native_path='/private/engine', hands=[[], ['secret']],
                                          plans={'enemy': 'hidden'}, diagnostic={'address': '0xff'}))
        self.assertFalse({'native_path', 'hands', 'plans', 'diagnostic'} & self.renderer.latest.keys())
        self.assertEqual(self.renderer.latest['status'], 'paused')
        self.assertEqual(self.renderer.latest['layout'], self.packet()['layout'])

    def test_sequence_must_advance_even_across_new_match_numbers(self):
        self.renderer._accept(self.packet(sequence=2))
        for packet in [self.packet(sequence=2), self.packet(sequence=1, match_id=2),
                       self.packet(sequence=True), self.packet(sequence=3, v=True),
                       self.packet(sequence=3, width=541)]:
            with self.subTest(packet=packet), self.assertRaises(ValueError):
                self.renderer._accept(packet)
        self.renderer._accept(self.packet(sequence=3, match_id=2))
        self.assertEqual(self.renderer.latest['sequence'], 3)

    def test_only_referenced_textures_become_readable_for_this_match(self):
        self.asset(FIRST)
        self.asset(SECOND, b'not-referenced')
        draws = [{'kind': 'shadow', 'draws': [{'kind': 'sprite', 'tex': FIRST}]}]
        self.renderer._accept(self.packet(draws=draws))
        self.assertEqual(self.renderer.texture_ids, {FIRST})
        self.assertEqual(self.renderer.texture_bytes, 4)
        self.assertEqual(self.renderer.texture(FIRST).read_bytes(), b'png!')
        self.assert_http(404, lambda: self.renderer.texture(SECOND))
        # Repeated poses do not account for the immutable texture twice.
        self.renderer._accept(self.packet(sequence=2, draws=draws))
        self.assertEqual(self.renderer.texture_bytes, 4)

    def test_eviction_frees_accounted_bytes_and_expired_requests_return_410(self):
        path = self.asset(FIRST)
        self.asset(SECOND, b'next-pose!')
        self.renderer._accept(self.packet(draws=[{'kind': 'sprite', 'tex': FIRST}]))
        path.unlink()  # native LRU removes its encoded PNG before announcing
        self.renderer._accept(self.packet(sequence=2, evicted=[FIRST],
                                          draws=[{'kind': 'sprite', 'tex': SECOND}]))
        self.assertEqual(self.renderer.texture_ids, {SECOND})
        self.assertEqual(self.renderer.texture_sizes, {SECOND: 10})
        self.assertEqual(self.renderer.texture_bytes, 10)
        self.assertIn(FIRST, self.renderer.expired_texture_ids)
        self.assertNotIn('evicted', self.renderer.latest)
        self.assert_http(410, lambda: self.renderer.texture(FIRST))
        self.assert_http(410, lambda: self.route(f'/api/matches/fixture/textures/{FIRST}.png',
                                               headers={'If-None-Match': f'"{FIRST}"'}))
        self.assert_http(404, lambda: self.route(f'/api/matches/fixture/textures/{FIRST}.png', owner='owner-b'))
        self.assert_http(404, lambda: self.renderer.texture('c' * 64))

    def test_reappearing_content_id_is_reaccounted_and_clears_its_tombstone(self):
        self.asset(FIRST)
        self.renderer._accept(self.packet(draws=[{'kind': 'sprite', 'tex': FIRST}]))
        self.renderer._accept(self.packet(sequence=2, evicted=[FIRST]))
        self.asset(FIRST, b'reencoded')
        self.renderer._accept(self.packet(sequence=3, draws=[{'kind': 'sprite', 'tex': FIRST}]))
        self.assertEqual(self.renderer.texture_sizes, {FIRST: 9})
        self.assertEqual(self.renderer.texture_bytes, 9)
        self.assertNotIn(FIRST, self.renderer.expired_texture_ids)
        self.assertEqual(self.renderer.texture(FIRST).read_bytes(), b'reencoded')

    def test_eviction_tombstones_are_bounded_and_ancient_ids_return_404(self):
        identities = [format(value, '064x') for value in range(7)]
        with patch.object(scene, 'MAX_TEXTURE_TOMBSTONES', 3), patch.object(scene, 'MAX_TEXTURES', 1):
            for index, identity in enumerate(identities):
                self.asset(identity, b'xx')
                evicted = identities[index - 1:index] if index else []
                self.renderer._accept(self.packet(sequence=index + 1, evicted=evicted,
                                                  draws=[{'kind': 'sprite', 'tex': identity}]))
            self.assertEqual(list(self.renderer.expired_texture_ids), identities[3:6])
            self.assertEqual(self.renderer.texture_ids, {identities[-1]})
            self.assertEqual(self.renderer.texture_sizes, {identities[-1]: 2})
            self.assertEqual(self.renderer.texture_bytes, 2)
            self.assert_http(404, lambda: self.renderer.texture(identities[0]))
            self.assert_http(410, lambda: self.renderer.texture(identities[-2]))

    def test_evictions_are_validated_before_any_publication_or_accounting_change(self):
        self.asset(FIRST)
        self.renderer._accept(self.packet(draws=[{'kind': 'sprite', 'tex': FIRST}]))
        for evicted in [None, {}, FIRST, [1], ['../private'], ['g' * 64], [FIRST]]:
            with self.subTest(evicted=evicted), self.assertRaises(ValueError):
                self.renderer._accept(self.packet(sequence=2, evicted=evicted,
                                                  draws=[{'kind': 'sprite', 'tex': FIRST}]))
            self.assertEqual(self.renderer.latest['sequence'], 1)
            self.assertEqual(self.renderer.texture_sizes, {FIRST: 4})
            self.assertEqual(self.renderer.texture_bytes, 4)
            self.assertEqual(len(self.renderer.expired_texture_ids), 0)
        with self.assertRaisesRegex(ValueError, 'missing scene texture'):
            self.renderer._accept(self.packet(sequence=2, evicted=[FIRST],
                                              draws=[{'kind': 'sprite', 'tex': SECOND}]))
        self.assertEqual(self.renderer.texture_sizes, {FIRST: 4})

    def test_duplicate_evictions_are_accounted_once_and_unknown_announcements_stay_404(self):
        self.asset(FIRST)
        self.renderer._accept(self.packet(draws=[{'kind': 'sprite', 'tex': FIRST}]))
        self.renderer._accept(self.packet(sequence=2, evicted=[FIRST, FIRST, SECOND]))
        self.assertEqual(self.renderer.texture_sizes, {})
        self.assertEqual(self.renderer.texture_bytes, 0)
        self.assertEqual(list(self.renderer.expired_texture_ids), [FIRST])
        self.assert_http(410, lambda: self.renderer.texture(FIRST))
        self.assert_http(404, lambda: self.renderer.texture(SECOND))

    def test_evicted_bytes_make_room_for_new_references_in_the_same_transaction(self):
        self.asset(FIRST)
        self.asset(SECOND, b'1234567890')
        self.renderer._accept(self.packet(draws=[{'kind': 'sprite', 'tex': FIRST}]))
        with patch.object(scene, 'MAX_TEXTURE_BYTES', 10), patch.object(scene, 'MAX_TEXTURES', 1):
            self.renderer._accept(self.packet(sequence=2, evicted=[FIRST],
                                              draws=[{'kind': 'sprite', 'tex': SECOND}]))
        self.assertEqual(self.renderer.texture_sizes, {SECOND: 10})
        self.assertEqual(self.renderer.texture_bytes, 10)
        self.assertEqual(list(self.renderer.expired_texture_ids), [FIRST])

    def test_over_budget_eviction_transaction_leaves_current_whitelist_intact(self):
        self.asset(FIRST)
        self.asset(SECOND, b'12345678901')
        self.renderer._accept(self.packet(draws=[{'kind': 'sprite', 'tex': FIRST}]))
        with patch.object(scene, 'MAX_TEXTURE_BYTES', 10), self.assertRaisesRegex(ValueError, 'budget'):
            self.renderer._accept(self.packet(sequence=2, evicted=[FIRST],
                                              draws=[{'kind': 'sprite', 'tex': SECOND}]))
        self.assertEqual(self.renderer.latest['sequence'], 1)
        self.assertEqual(self.renderer.texture_ids, {FIRST})
        self.assertEqual(self.renderer.texture_sizes, {FIRST: 4})
        self.assertEqual(self.renderer.texture_bytes, 4)
        self.assertEqual(len(self.renderer.expired_texture_ids), 0)

    def test_eviction_during_file_read_is_reported_as_410(self):
        self.asset(FIRST)
        self.renderer._accept(self.packet(draws=[{'kind': 'sprite', 'tex': FIRST}]))

        def evict_while_reading():
            self.renderer._accept(self.packet(sequence=2, evicted=[FIRST]))
            raise FileNotFoundError('native LRU removed PNG during read')

        with patch.object(Path, 'read_bytes', side_effect=evict_while_reading):
            self.assert_http(410, lambda: self.route(f'/api/matches/fixture/textures/{FIRST}.png'))

    def test_invalid_texture_ids_and_path_traversal_are_not_readable(self):
        self.asset()
        self.renderer.texture_ids.add(FIRST)
        for identity in [None, 7, '../secret', FIRST + '/../secret', FIRST.upper(),
                         FIRST + '.png', FIRST + '\x00', 'a' * 65, 'a' * 15]:
            with self.subTest(identity=identity):
                self.assert_http(404, lambda: self.renderer.texture(identity))

    def test_validly_named_symlink_cannot_escape_texture_directory(self):
        outside = self.root / 'private-data'
        outside.write_bytes(b'private')
        (self.renderer.texture_dir / (FIRST + '.png')).symlink_to(outside)
        self.renderer.texture_ids.add(FIRST)
        self.assert_http(404, lambda: self.renderer.texture(FIRST))
        with self.assertRaises(ValueError):
            self.renderer._accept(self.packet(draws=[{'kind': 'sprite', 'tex': SECOND}]))

    def test_missing_texture_is_a_404_not_a_filesystem_500(self):
        path = self.asset()
        self.renderer._accept(self.packet(draws=[{'kind': 'sprite', 'tex': FIRST}]))
        path.unlink()
        self.assert_http(404, lambda: self.renderer.texture(FIRST))
        self.assert_http(404, lambda: self.route(f'/api/matches/fixture/textures/{FIRST}.png'))

    def test_texture_disappearing_after_path_lookup_is_still_a_404(self):
        path = self.asset()
        self.renderer.texture_ids.add(FIRST)
        with patch.object(Path, 'read_bytes', side_effect=FileNotFoundError('removed during request')):
            self.assert_http(404, lambda: self.route(f'/api/matches/fixture/textures/{FIRST}.png'))
        self.assertTrue(path.exists())

    def test_scene_and_texture_routes_are_owner_scoped_before_cache_checks(self):
        self.asset()
        self.renderer.texture_ids.add(FIRST)
        for path in ['/api/matches/fixture/scene', f'/api/matches/fixture/textures/{FIRST}.png']:
            self.assert_http(404, lambda: self.route(path, owner='owner-b', headers={'If-None-Match': f'"{FIRST}"'}))
        self.assert_http(404, lambda: self.route('/api/matches/missing/scene'))
        response = self.route(f'/api/matches/fixture/textures/{FIRST}.png')
        self.assertEqual(response.body, b'png!')
        self.assertEqual(response.headers['Cache-Control'], 'private, max-age=31536000, immutable')
        cached = self.route(f'/api/matches/fixture/textures/{FIRST}.png', headers={'If-None-Match': f'"{FIRST}"'})
        self.assertEqual((cached.status, cached.body), (304, b''))

    def test_missing_or_failed_renderer_reports_preparing_or_unavailable(self):
        self.match.renderer = None
        self.assert_http(503, lambda: self.route('/api/matches/fixture/scene'))
        self.match.error = 'fixture engine failed'
        error = self.assert_http(503, lambda: self.route('/api/matches/fixture/scene'))
        self.assertEqual(error.message, 'fixture engine failed')

    def test_draw_recursion_and_total_commands_have_explicit_bounds(self):
        draws = [{'kind': 'sprite', 'tex': FIRST}]
        for _ in range(scene.MAX_SHADOW_DEPTH + 1):
            draws = [{'kind': 'shadow', 'draws': draws}]
        with self.assertRaises(ValueError):
            scene.texture_references(draws)
        oversized = [{'kind': 'shadow', 'draws': [{'kind': 'rect'}] * 2000} for _ in range(6)]
        with self.assertRaisesRegex(ValueError, 'budget'):
            scene.texture_references(oversized)
        for draws in [None, {}, [1], [{'kind': 'shadow', 'draws': {}}], [{'tex': '../private'}]]:
            with self.subTest(draws=draws), self.assertRaises(ValueError):
                scene.texture_references(draws)

    def test_missing_references_and_texture_budgets_do_not_publish_partial_packets(self):
        self.asset(FIRST)
        self.asset(SECOND)
        draws = [{'kind': 'sprite', 'tex': FIRST}, {'kind': 'sprite', 'tex': SECOND}]
        for patcher in [patch.object(scene, 'MAX_TEXTURES', 1), patch.object(scene, 'MAX_TEXTURE_BYTES', 7)]:
            with patcher, self.assertRaisesRegex(ValueError, 'budget'):
                self.renderer._accept(self.packet(draws=draws))
            self.assertIsNone(self.renderer.latest)
            self.assertEqual(self.renderer.texture_ids, set())
        with self.assertRaisesRegex(ValueError, 'missing scene texture'):
            self.renderer._accept(self.packet(draws=[{'kind': 'sprite', 'tex': 'c' * 64}]))

    def test_error_event_does_not_suspend_while_holding_condition_lock(self):
        self.renderer.error = 'fixture presenter failed'
        events = self.renderer.events()
        self.assertEqual(next(events), ('error', {'error': 'fixture presenter failed'}))
        acquired = []

        def inspect_lock():
            success = self.renderer.condition.acquire(timeout=.25)
            acquired.append(success)
            if success:
                self.renderer.condition.release()

        observer = threading.Thread(target=inspect_lock)
        observer.start()
        observer.join(timeout=1)
        self.assertEqual(acquired, [True])
        events.close()

    def test_scene_subscriber_is_released_on_explicit_stream_close(self):
        self.renderer._accept(self.packet())
        stream = self.route('/api/matches/fixture/scene')
        self.assertIsInstance(stream, EventStream)
        self.assertEqual(self.match.subscribers, 0)
        self.assertEqual(next(stream.events)[0], 'scene')
        self.assertEqual(self.match.subscribers, 1)
        stream.on_close()
        stream.on_close()
        self.assertEqual(self.match.subscribers, 0)
        self.assertGreater(self.match.last_watched, 0)

    def test_scene_subscriber_is_released_if_source_creation_or_close_raises(self):
        with patch.object(self.renderer, 'events', side_effect=RuntimeError('source could not start')):
            stream = self.route('/api/matches/fixture/scene')
            with self.assertRaisesRegex(RuntimeError, 'could not start'):
                next(stream.events)
            self.assertEqual(self.match.subscribers, 0)
        self.match.last_watched = 0

        class BadClose:
            def __iter__(self):
                return self

            def __next__(self):
                return 'heartbeat', {}

            def close(self):
                raise RuntimeError('source could not close')

        with patch.object(self.renderer, 'events', return_value=BadClose()):
            stream = self.route('/api/matches/fixture/scene')
            next(stream.events)
            with self.assertRaisesRegex(RuntimeError, 'could not close'):
                stream.on_close()
            self.assertEqual(self.match.subscribers, 0)
            self.assertGreater(self.match.last_watched, 0)

    def test_scene_stream_inlines_pose_bytes_on_the_open_connection(self):
        import base64
        payload = b'\x89PNG\r\n\x1a\n'
        self.asset(FIRST, payload)
        self.renderer._accept(self.packet(draws=[{'kind': 'sprite', 'tex': FIRST}]))
        events = self.renderer.events()
        self.addCleanup(events.close)
        kind, packet = next(events)
        self.assertEqual(kind, 'scene')
        self.assertEqual(packet['blobs'], [{'id': FIRST, 'png': base64.b64encode(payload).decode('ascii')}])
        self.assertNotIn('blobs', self.renderer.latest)
        self.asset(SECOND, b'next-pose')
        self.renderer._accept(self.packet(
            sequence=2,
            draws=[{'kind': 'sprite', 'tex': FIRST}, {'kind': 'sprite', 'tex': SECOND}],
            textures=[{'id': SECOND, 'width': 2, 'height': 2}]))
        kind, again = next(events)
        self.assertEqual(kind, 'scene')
        self.assertEqual(again['blobs'], [{'id': SECOND, 'png': base64.b64encode(b'next-pose').decode('ascii')}])
        self.assertNotIn('blobs', self.renderer.latest)

    def test_incomplete_and_deep_json_packets_fail_cleanly(self):
        for payload in [b'{"v":1}', b'[' * 1100 + b']' * 1100 + b'\n']:
            self.renderer.error = None
            self.renderer.proc = SimpleNamespace(stdout=io.BytesIO(payload))
            self.renderer._read()
            self.assertIn('scene presenter stopped', self.renderer.error)
            self.assertIsNone(self.renderer.latest)


if __name__ == '__main__':
    unittest.main()
