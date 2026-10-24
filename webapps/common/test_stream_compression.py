"""SSE gzip framing and immediate delivery through the stdlib adapter."""
from __future__ import annotations

import gzip
from http.client import HTTPConnection
from http.server import ThreadingHTTPServer
import socket
import struct
import threading
from types import SimpleNamespace
import unittest
from unittest.mock import patch
import zlib

from webapps.common.httpkit import App, EventStream, Raw, accepts_gzip, event_stream_chunks


class CompressionUnitTests(unittest.TestCase):
    def test_encoding_negotiation_honors_explicit_refusal_over_wildcard(self):
        cases = {
            '': False, 'identity': False, 'br': False, 'gzip': True,
            'br, GZIP': True, 'gzip;q=0': False, 'gzip;q=0.000': False,
            'gzip;q=0.125': True, 'gzip;q=1.000': True, '*;q=0.5': True,
            '*;q=1, gzip;q=0': False, '*;q=0, gzip;q=1': True,
            'gzip;q=0, gzip;q=1': False, 'gzip;q=NaN': False,
            'gzip;q=1.1': False, 'gzip;q=-1': False, 'gzip;q=0;q=1': False,
            'x-gzip': False, 'gzip;q=0.1234': False,
        }
        for header, expected in cases.items():
            with self.subTest(header=header):
                self.assertEqual(accepts_gzip(header), expected)

    def test_each_event_decodes_before_the_next_is_requested(self):
        delivered = threading.Event()
        closed = []

        def events():
            try:
                yield 'scene', {'tick': 1}
                self.assertTrue(delivered.is_set(), 'compressor read ahead instead of flushing')
                yield 'scene', {'tick': 2}
            finally:
                closed.append(True)

        chunks = event_stream_chunks(events(), gzip=True)
        decoder = zlib.decompressobj(16 + zlib.MAX_WBITS)
        first = decoder.decompress(next(chunks))
        self.assertEqual(first, b'event: scene\ndata: {"tick":1}\n\n')
        self.assertFalse(decoder.eof)
        delivered.set()
        second = decoder.decompress(next(chunks))
        self.assertEqual(second, b'event: scene\ndata: {"tick":2}\n\n')
        self.assertEqual(decoder.decompress(next(chunks)), b'')
        self.assertTrue(decoder.eof, 'graceful response must finish its gzip checksum/trailer')
        with self.assertRaises(StopIteration):
            next(chunks)
        self.assertEqual(closed, [True])

    def test_identity_bytes_stay_compatible_and_closing_compression_closes_source(self):
        expected = b'event: scene\ndata: {"tick":1}\n\ndata: {}\n\n'
        self.assertEqual(b''.join(event_stream_chunks([('scene', {'tick': 1}), (None, {})])), expected)
        identified = b''.join(event_stream_chunks([('ticks', {'v': 2}, '1:10')]))
        self.assertEqual(identified, b'id: 1:10\nevent: ticks\ndata: {"v":2}\n\n')
        self.assertEqual(gzip.decompress(b''.join(event_stream_chunks(
            [('scene', {'tick': 1}), (None, {})], gzip=True))), expected)
        closed = []

        def events():
            try:
                yield 'scene', {}
                yield 'scene', {}
            finally:
                closed.append(True)

        chunks = event_stream_chunks(events(), gzip=True)
        next(chunks)
        chunks.close()
        self.assertEqual(closed, [True])


class StdlibCompressionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.states = {}
        cls.counter = 0
        app = App('stream-test')

        @app.route('GET', '/events/{mode}')
        def stream(request):
            state = cls.states[request.query['token']]

            def events():
                try:
                    yield 'scene', {'tick': 1, 'art': 'original-' * 200}
                    if state.gated:
                        state.gate.wait(3)
                    yield 'scene', {'tick': 2}
                finally:
                    state.source_closed.set()

            return EventStream(events(), on_close=state.callback_closed.set,
                               gzip=request.params['mode'] == 'gzip')

        app.route('GET', '/cache')(lambda request: Raw(b'png', 'image/png', headers={
            'cache-control': 'private, max-age=31536000, immutable'}))
        app.route('GET', '/default-cache')(lambda request: Raw(b'plain'))
        ready = threading.Event()

        def server_factory(address, handler):
            cls.server = ThreadingHTTPServer(address, handler)
            ready.set()
            return cls.server

        with patch('webapps.common.httpkit.ThreadingHTTPServer', side_effect=server_factory):
            cls.thread = threading.Thread(target=app.serve, args=('127.0.0.1', 0), daemon=True)
            cls.thread.start()
            if not ready.wait(5):
                raise RuntimeError('test server did not start')
        cls.address = cls.server.server_address

    @classmethod
    def tearDownClass(cls):
        cls.server.shutdown()
        cls.thread.join(5)

    def fixture(self, *, gated=False):
        type(self).counter += 1
        token = str(type(self).counter)
        state = SimpleNamespace(gated=gated, gate=threading.Event(), source_closed=threading.Event(),
                                callback_closed=threading.Event())
        self.states[token] = state
        self.addCleanup(state.gate.set)
        return token, state

    def request(self, mode='gzip', encoding='gzip', *, gated=False):
        token, state = self.fixture(gated=gated)
        connection = HTTPConnection(*self.address, timeout=2)
        self.addCleanup(connection.close)
        connection.request('GET', f'/events/{mode}?token={token}', headers={'Accept-Encoding': encoding})
        return connection, connection.getresponse(), state

    def test_compressed_event_arrives_before_next_native_tick(self):
        _, response, state = self.request(gated=True)
        self.assertEqual(response.getheader('Content-Encoding'), 'gzip')
        self.assertEqual(response.getheader('Vary'), 'Accept-Encoding')
        self.assertEqual(response.getheader('X-Accel-Buffering'), 'no')
        decoder = zlib.decompressobj(16 + zlib.MAX_WBITS)
        decoded = b''
        while not decoded.endswith(b'\n\n'):
            decoded += decoder.decompress(response.read1(65536))
        self.assertIn(b'"tick":1', decoded)
        self.assertFalse(state.gate.is_set())
        self.assertFalse(state.source_closed.is_set())
        state.gate.set()
        decoded += decoder.decompress(response.read())
        self.assertIn(b'"tick":2', decoded)
        self.assertTrue(decoder.eof)
        self.assertTrue(state.callback_closed.wait(1))
        self.assertTrue(state.source_closed.is_set())

    def test_unsupported_or_refused_gzip_keeps_plain_sse(self):
        for encoding in ['', 'br', 'gzip;q=0', '*;q=1,gzip;q=0']:
            with self.subTest(encoding=encoding):
                _, response, state = self.request(encoding=encoding)
                self.assertIsNone(response.getheader('Content-Encoding'))
                self.assertEqual(response.getheader('Vary'), 'Accept-Encoding')
                self.assertTrue(response.read().startswith(b'event: scene\ndata: '))
                self.assertTrue(state.callback_closed.wait(1))

    def test_ordinary_streams_are_not_compressed_even_when_client_supports_gzip(self):
        _, response, state = self.request(mode='plain')
        self.assertIsNone(response.getheader('Content-Encoding'))
        self.assertIsNone(response.getheader('Vary'))
        self.assertTrue(response.read().startswith(b'event: scene\ndata: '))
        self.assertTrue(state.callback_closed.wait(1))

    def test_disconnected_compressed_subscriber_is_cleaned_up(self):
        connection, response, state = self.request(gated=True)
        self.assertTrue(response.read1(65536))
        # Force the next server write to observe a disconnected client.
        sock = response.fp.raw._sock
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_LINGER, struct.pack('ii', 1, 0))
        response.close()
        connection.close()
        state.gate.set()
        self.assertTrue(state.callback_closed.wait(2))
        self.assertTrue(state.source_closed.is_set())

    def test_explicit_cache_header_replaces_default_instead_of_duplicating_it(self):
        connection = HTTPConnection(*self.address, timeout=2)
        self.addCleanup(connection.close)
        connection.request('GET', '/cache')
        response = connection.getresponse()
        self.assertEqual(response.headers.get_all('Cache-Control'),
                         ['private, max-age=31536000, immutable'])
        response.read()
        connection.request('GET', '/default-cache')
        response = connection.getresponse()
        self.assertEqual(response.headers.get_all('Cache-Control'), ['no-store'])
        response.read()


if __name__ == '__main__':
    unittest.main()
