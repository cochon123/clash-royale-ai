"""Raw ASGI SSE compression, prefix/session handling and disconnect cleanup."""
from __future__ import annotations

import asyncio
import threading
import unittest
from unittest.mock import patch
import zlib

from webapps.common.httpkit import App, EventStream

try:
    from starlette.requests import ClientDisconnect
    from webapps.arena import asgi
except ImportError:
    asgi = None


@unittest.skipIf(asgi is None, 'install arena-web for ASGI transport tests')
class ASGICompressionTests(unittest.TestCase):
    def run_stream(self, *, encoding='gzip', enabled=True, disconnect=False, prefix=''):
        router = App('stream-fixture')
        delivered = threading.Event()
        cleanup = []
        messages = []
        decoded = bytearray()
        decoder = zlib.decompressobj(16 + zlib.MAX_WBITS)
        compressed = False

        @router.route('GET', '/events')
        def stream(request):
            self.assertIsNotNone(request.session_id)

            def events():
                try:
                    yield 'scene', {'tick': 1}
                    self.assertTrue(delivered.is_set(), 'ASGI buffered the first event')
                    yield 'scene', {'tick': 2}
                finally:
                    cleanup.append('source')

            return EventStream(events(), on_close=lambda: cleanup.append('callback'), gzip=enabled)

        async def request(application):
            sent_request = False

            async def receive():
                nonlocal sent_request
                if not sent_request:
                    sent_request = True
                    return {'type': 'http.request', 'body': b'', 'more_body': False}
                await asyncio.Event().wait()

            async def send(message):
                nonlocal compressed
                messages.append(message)
                if message['type'] == 'http.response.start':
                    compressed = dict(message['headers']).get(b'content-encoding') == b'gzip'
                elif message['type'] == 'http.response.body' and message.get('body'):
                    chunk = message['body']
                    decoded.extend(decoder.decompress(chunk) if compressed else chunk)
                    if b'\n\n' in decoded:
                        delivered.set()
                    if disconnect:
                        raise OSError('fixture socket disconnected')

            path = prefix + '/events'
            scope = {'type': 'http', 'http_version': '1.1', 'asgi': {'version': '3.0', 'spec_version': '2.4'},
                     'method': 'GET', 'scheme': 'https', 'path': path, 'raw_path': path.encode(),
                     'root_path': '', 'query_string': b'', 'server': ('arena.example', 443),
                     'client': ('127.0.0.1', 1),
                     'headers': [(b'host', b'arena.example'), (b'accept-encoding', encoding.encode())]}
            await application(scope, receive, send)

        with patch.object(asgi, 'router', router):
            application = asgi.create_app(secret='fixture-only-secret-' * 3, secure=True,
                                           base_path=prefix, allowed_hosts=['arena.example'], start_warmup=False)
            if disconnect:
                with self.assertRaises(ClientDisconnect):
                    asyncio.run(request(application))
            else:
                asyncio.run(request(application))
        headers = dict(next(message for message in messages if message['type'] == 'http.response.start')['headers'])
        return headers, bytes(decoded), decoder, cleanup, messages

    def test_gzip_chunks_are_immediate_and_form_a_finished_stream_with_prefix_and_session(self):
        headers, decoded, decoder, cleanup, messages = self.run_stream(prefix='/clashroyale')
        self.assertEqual(headers[b'content-encoding'], b'gzip')
        self.assertIn(b'Accept-Encoding', headers[b'vary'].split(b', '))
        self.assertEqual(headers[b'x-accel-buffering'], b'no')
        self.assertIn(b'path=/clashroyale/', headers[b'set-cookie'].lower())
        self.assertEqual(decoded, b'event: scene\ndata: {"tick":1}\n\nevent: scene\ndata: {"tick":2}\n\n')
        self.assertTrue(decoder.eof)
        self.assertEqual(cleanup, ['source', 'callback'])
        self.assertGreaterEqual(len([message for message in messages if message.get('body')]), 3)

    def test_explicit_refusal_or_unsupported_encoding_keeps_identity(self):
        for encoding in ['', 'br', 'gzip;q=0', '*;q=1, gzip;q=0']:
            with self.subTest(encoding=encoding):
                headers, decoded, _, cleanup, _ = self.run_stream(encoding=encoding)
                self.assertNotIn(b'content-encoding', headers)
                self.assertIn(b'Accept-Encoding', headers[b'vary'].split(b', '))
                self.assertIn(b'"tick":1', decoded)
                self.assertIn(b'"tick":2', decoded)
                self.assertEqual(cleanup, ['source', 'callback'])

    def test_streams_remain_identity_unless_explicitly_enabled(self):
        headers, decoded, _, cleanup, _ = self.run_stream(enabled=False)
        self.assertNotIn(b'content-encoding', headers)
        self.assertNotIn(b'Accept-Encoding', headers.get(b'vary', b'').split(b', '))
        self.assertTrue(decoded.startswith(b'event: scene\ndata: '))
        self.assertEqual(cleanup, ['source', 'callback'])

    def test_disconnect_closes_compressed_source_and_callback_once(self):
        headers, decoded, _, cleanup, _ = self.run_stream(disconnect=True)
        self.assertEqual(headers[b'content-encoding'], b'gzip')
        self.assertEqual(decoded, b'event: scene\ndata: {"tick":1}\n\n')
        self.assertEqual(cleanup, ['source', 'callback'])


if __name__ == '__main__':
    unittest.main()
