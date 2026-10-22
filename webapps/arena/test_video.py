"""MJPEG framing, bounded delivery and transport cleanup without an engine."""
from __future__ import annotations

import asyncio
from contextlib import contextmanager
import io
from http.client import HTTPConnection
from http.server import ThreadingHTTPServer
from pathlib import Path
import socket
import struct
import tempfile
import threading
from types import SimpleNamespace
import unittest
from unittest.mock import MagicMock, patch

from webapps.arena import renderer, server
from webapps.common.httpkit import App, ByteStream, HTTPError, Request

try:
    from webapps.arena import asgi
    from starlette.testclient import TestClient
except ImportError:
    asgi = None


class TrackedIterator:
    """Retain the iterator so garbage collection cannot hide missing close()."""

    def __init__(self, values, *, repeat=False):
        self.values = iter(values)
        self.repeat = repeat
        self.previous = None
        self.closed = threading.Event()
        self.close_count = 0

    def __iter__(self):
        return self

    def __next__(self):
        if self.closed.is_set():
            raise StopIteration
        try:
            self.previous = next(self.values)
        except StopIteration:
            if not self.repeat:
                raise
        return self.previous

    def close(self):
        self.close_count += 1
        self.closed.set()


def video_request(*, session_id="owner"):
    return Request("GET", "/api/matches/fixture/video", {}, {"id": "fixture"}, {},
                   session_id=session_id)


def match_with_frames(source):
    return SimpleNamespace(id="fixture", owner="owner", error=None,
                           message="Preparing viewer", subscribers=0, last_watched=0,
                           renderer=SimpleNamespace(frames=lambda: source))


@contextmanager
def http_server(app):
    """Run the real stdlib adapter on an ephemeral loopback-only port."""
    ready = threading.Event()
    box = {}

    def server_factory(address, handler):
        box["server"] = ThreadingHTTPServer(address, handler)
        ready.set()
        return box["server"]

    with patch("webapps.common.httpkit.ThreadingHTTPServer", side_effect=server_factory):
        thread = threading.Thread(target=app.serve, args=("127.0.0.1", 0), daemon=True)
        thread.start()
        if not ready.wait(5):
            raise RuntimeError("stream test HTTP server did not start")
    listener = box["server"]
    try:
        yield listener.server_address
    finally:
        listener.shutdown()
        thread.join(5)


class VideoRouteTests(unittest.TestCase):
    def setUp(self):
        # Include newlines, nulls and a boundary-like string in the image. The
        # receiver must use Content-Length rather than splitting JPEG contents.
        self.images = [b"\xff\xd8\x00\r\n--arenaframe\r\n\xff\xd9", b"\xff\xd8second\xff\xd9"]
        self.source = TrackedIterator([({"sequence": i + 1}, image)
                                       for i, image in enumerate(self.images)])
        self.match = match_with_frames(self.source)
        self.manager = MagicMock()
        self.manager.get.return_value = self.match
        patcher = patch.object(server, "manager", self.manager)
        patcher.start()
        self.addCleanup(patcher.stop)

    def test_missing_and_other_browser_video_are_404_before_subscription(self):
        for match, session in ((None, "owner"), (self.match, "another-browser")):
            with self.subTest(match=match, session=session):
                self.manager.get.return_value = match
                with self.assertRaises(HTTPError) as caught:
                    server.match_video(video_request(session_id=session))
                self.assertEqual(caught.exception.status, 404)
        self.assertEqual(self.match.subscribers, 0)
        self.assertFalse(self.source.closed.is_set())

    def test_unavailable_viewer_returns_503_before_stream_headers(self):
        for error, viewer in (("renderer failed", self.match.renderer), (None, None)):
            with self.subTest(error=error):
                self.match.error, self.match.renderer = error, viewer
                with self.assertRaises(HTTPError) as caught:
                    server.match_video(video_request())
                self.assertEqual(caught.exception.status, 503)
        self.assertEqual(self.match.subscribers, 0)

    def test_multipart_parts_preserve_binary_images_and_lengths(self):
        stream = server.match_video(video_request())
        self.assertIsInstance(stream, ByteStream)
        self.assertEqual(stream.content_type, "multipart/x-mixed-replace; boundary=arenaframe")
        chunks = iter(stream.chunks)
        for image in self.images:
            part = next(chunks)
            header, payload = part.split(b"\r\n\r\n", 1)
            self.assertEqual(header.split(b"\r\n")[0], b"--arenaframe")
            self.assertIn(b"Content-Type: image/jpeg", header)
            self.assertIn(f"Content-Length: {len(image)}".encode(), header)
            self.assertEqual(payload, image + b"\r\n")
            self.assertEqual(self.match.subscribers, 1)
        # A terminal boundary is also valid; no extra image may appear.
        self.assertTrue(all(part == b"--arenaframe--\r\n" for part in chunks))
        self.assertEqual(self.match.subscribers, 0)
        self.assertGreater(self.match.last_watched, 0)
        self.assertTrue(self.source.closed.is_set())

    def test_closing_multipart_iterator_closes_source_and_releases_subscriber(self):
        stream = server.match_video(video_request())
        chunks = iter(stream.chunks)
        next(chunks)
        self.assertEqual(self.match.subscribers, 1)
        chunks.close()
        self.assertEqual(self.match.subscribers, 0)
        self.assertEqual(self.source.close_count, 1)
        self.assertGreater(self.match.last_watched, 0)


class LatestRendererFrameTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        proc = MagicMock()
        proc.stdin, proc.stdout = io.BytesIO(), io.BytesIO()
        proc.poll.return_value = 0
        # Run the actual initializer: this catches collisions between the
        # snapshot input queue and the public frames() output iterator.
        with patch.object(renderer.subprocess, "Popen", return_value=proc), \
                patch.object(renderer.threading, "Thread"):
            self.viewer = renderer.NativeRenderer(Path(directory.name))
        self.addCleanup(self.viewer.close)

    def publish(self, sequence, jpeg):
        with self.viewer.condition:
            self.viewer.latest = ({"sequence": sequence}, jpeg)
            self.viewer.condition.notify_all()

    def test_initialized_renderer_exposes_callable_frame_iterator(self):
        self.assertTrue(callable(self.viewer.frames))

    def test_slow_subscriber_receives_latest_frame_without_replaying_backlog(self):
        frames = self.viewer.frames()
        self.publish(1, b"first")
        self.assertEqual(next(frames), ({"sequence": 1}, b"first"))
        # Simulate rendering while the previous frame is stuck on the network.
        self.publish(2, b"stale")
        self.publish(9, b"latest")
        self.assertEqual(next(frames), ({"sequence": 9}, b"latest"))
        frames.close()

    def test_closing_renderer_wakes_waiting_frame_consumer(self):
        frames = self.viewer.frames()
        waiting = threading.Event()
        finished = threading.Event()
        outcome = []
        original_wait = self.viewer.condition.wait

        def wait_for_frame(timeout=None):
            waiting.set()
            return original_wait(timeout)

        def consume():
            try:
                outcome.append(next(frames))
            except StopIteration:
                outcome.append("closed")
            finally:
                finished.set()

        consumer = threading.Thread(target=consume, daemon=True)
        with patch.object(self.viewer.condition, "wait", side_effect=wait_for_frame):
            consumer.start()
            self.assertTrue(waiting.wait(1), "consumer did not wait for its first frame")
            self.viewer.close()
            self.assertTrue(finished.wait(1), "renderer close did not wake the streaming consumer")
        consumer.join(1)
        self.assertEqual(outcome, ["closed"])


class StdlibByteStreamTests(unittest.TestCase):
    def test_finite_binary_stream_closes_source_and_callback_once(self):
        source = TrackedIterator([b"\x00\xfffirst", b"\r\nsecond"])
        on_close = MagicMock()
        app = App("finite-video-test")
        app.route("GET", "/api/video")(lambda _request: ByteStream(source, "application/octet-stream", on_close))
        with http_server(app) as address:
            connection = HTTPConnection(*address, timeout=5)
            self.addCleanup(connection.close)
            connection.request("GET", "/api/video")
            response = connection.getresponse()
            self.assertEqual(response.status, 200)
            self.assertEqual(response.getheader("Content-Type"), "application/octet-stream")
            self.assertEqual(response.getheader("X-Accel-Buffering"), "no")
            self.assertEqual(response.read(), b"\x00\xfffirst\r\nsecond")
            self.assertTrue(source.closed.wait(1))
            self.assertEqual(source.close_count, 1)
            on_close.assert_called_once_with()

    def test_disconnected_video_closes_source_and_reclaims_subscriber(self):
        source = TrackedIterator([({"sequence": 1}, b"x" * (1024 * 1024))], repeat=True)
        match = match_with_frames(source)
        app = App("disconnect-video-test")
        finished = threading.Event()
        on_close = MagicMock(side_effect=finished.set)

        def route(_request):
            stream = server.match_video(video_request(session_id=None))
            stream.on_close = on_close
            return stream

        app.route("GET", "/api/video")(route)
        with patch.object(server.manager, "get", return_value=match), http_server(app) as address:
            with socket.create_connection(address, timeout=5) as connection:
                connection.sendall(b"GET /api/video HTTP/1.1\r\nHost: localhost\r\n\r\n")
                self.assertIn(b"200", connection.recv(4096))
                # RST makes the next write fail, without relying on small socket
                # buffer sizes or timing a full JPEG download.
                connection.setsockopt(socket.SOL_SOCKET, socket.SO_LINGER, struct.pack("ii", 1, 0))
            self.assertTrue(finished.wait(3), "disconnected stream did not run cleanup")
            self.assertTrue(source.closed.is_set(), "disconnected source was retained")
            self.assertEqual(match.subscribers, 0)
            self.assertEqual(source.close_count, 1)
            on_close.assert_called_once_with()


@unittest.skipIf(asgi is None, "install arena-web for ASGI stream checks")
class ASGIByteStreamTests(unittest.TestCase):
    def application(self, stream):
        router = App("asgi-video-test")
        router.route("GET", "/api/video")(lambda _request: stream)
        with patch.object(asgi, "router", router):
            return asgi.create_app(secret="fixture-only-secret-" * 3, secure=True,
                                   base_path="/clashroyale", allowed_hosts=["arena.example"],
                                   start_warmup=False)

    def test_finite_binary_stream_under_base_path_closes_source_and_callback(self):
        source = TrackedIterator([b"\xff\x00one", b"two\r\n"])
        on_close = MagicMock()
        stream = ByteStream(source, "multipart/x-mixed-replace; boundary=fixture", on_close)
        router = App("asgi-video-test")
        router.route("GET", "/api/video")(lambda _request: stream)
        with patch.object(asgi, "router", router), TestClient(
                self.application(stream), base_url="https://arena.example") as client:
            response = client.get("/clashroyale/api/video")
        self.assertEqual(response.status_code, 200)
        self.assertEqual(response.headers["Content-Type"], stream.content_type)
        self.assertEqual(response.headers["X-Accel-Buffering"], "no")
        self.assertEqual(response.content, b"\xff\x00onetwo\r\n")
        self.assertEqual(source.close_count, 1)
        on_close.assert_called_once_with()

    def test_disconnect_cancellation_closes_retained_source_and_subscriber(self):
        for spec_version in ("2.3", "2.4"):
            with self.subTest(asgi_spec=spec_version):
                source = TrackedIterator([({"sequence": 1}, b"jpeg")], repeat=True)
                match = match_with_frames(source)
                with patch.object(server.manager, "get", return_value=match):
                    stream = server.match_video(video_request())
                on_close = MagicMock()
                stream.on_close = on_close
                application = self.application(stream)
                router = App("asgi-video-test")
                router.route("GET", "/api/video")(lambda _request: stream)
                with patch.object(asgi, "router", router):
                    asyncio.run(self.disconnect(application, spec_version))
                self.assertEqual(match.subscribers, 0)
                self.assertTrue(source.closed.is_set())
                self.assertEqual(source.close_count, 1)
                on_close.assert_called_once_with()

    async def disconnect(self, application, spec_version):
        disconnected = asyncio.Event()
        requested = False
        sent = []
        scope = {"type": "http", "asgi": {"version": "3.0", "spec_version": spec_version},
                 "http_version": "1.1", "method": "GET", "scheme": "https",
                 "path": "/clashroyale/api/video", "raw_path": b"/clashroyale/api/video",
                 "root_path": "", "query_string": b"", "headers": [(b"host", b"arena.example")],
                 "server": ("arena.example", 443), "client": ("127.0.0.1", 12345)}

        async def receive():
            nonlocal requested
            if not requested:
                requested = True
                return {"type": "http.request", "body": b"", "more_body": False}
            await disconnected.wait()
            return {"type": "http.disconnect"}

        async def send(message):
            sent.append(message)
            if message["type"] == "http.response.body" and message.get("body"):
                self.assertEqual(message["more_body"], True)
                disconnected.set()
                if spec_version == "2.4":
                    raise OSError("client disconnected")
                # On the older ASGI branch the disconnect listener cancels the
                # task while send is suspended, like a backpressured socket.
                await asyncio.Event().wait()

        try:
            await asyncio.wait_for(application(scope, receive, send), timeout=3)
        except Exception as error:
            # Starlette >=ASGI2.4 signals the failed write as ClientDisconnect.
            from starlette.requests import ClientDisconnect
            if spec_version != "2.4" or not isinstance(error, ClientDisconnect):
                raise
        self.assertTrue(any(item["type"] == "http.response.body" for item in sent))


if __name__ == "__main__":
    unittest.main()
