"""HTTP framing checks against a real short-lived standard-library server."""
from __future__ import annotations

import json
import socket
import threading
import unittest
from http.client import HTTPConnection
from http.server import ThreadingHTTPServer
from unittest.mock import patch

from webapps.common.httpkit import App, MAX_BODY_BYTES


class HTTPFramingTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        app = App("validation-test")
        app.route("POST", "/echo")(lambda request: request.json())
        ready = threading.Event()

        def server_factory(address, handler):
            cls.server = ThreadingHTTPServer(address, handler)
            ready.set()
            return cls.server

        with patch("webapps.common.httpkit.ThreadingHTTPServer", side_effect=server_factory):
            cls.thread = threading.Thread(target=app.serve, args=("127.0.0.1", 0), daemon=True)
            cls.thread.start()
            if not ready.wait(5):
                raise RuntimeError("test server did not start")
        cls.address = cls.server.server_address

    @classmethod
    def tearDownClass(cls):
        cls.server.shutdown()
        cls.thread.join(5)

    def raw_request(self, headers, body=b""):
        with socket.create_connection(self.address, timeout=5) as connection:
            connection.sendall(b"POST /echo HTTP/1.1\r\nHost: localhost\r\n" + headers + b"\r\n\r\n" + body)
            connection.shutdown(socket.SHUT_WR)
            received = bytearray()
            while chunk := connection.recv(65536):
                received.extend(chunk)
        header, payload = bytes(received).split(b"\r\n\r\n", 1)
        return int(header.split(b" ", 2)[1]), json.loads(payload)

    def test_malformed_or_ambiguous_content_lengths_return_json_400(self):
        for headers in (b"Content-Length: nope", b"Content-Length: -1", b"Content-Length: 1.5",
                        b"Content-Length: 0\r\nContent-Length: 1", b"Transfer-Encoding: chunked"):
            with self.subTest(headers=headers):
                status, body = self.raw_request(headers)
                self.assertEqual(status, 400)
                self.assertIn("error", body)

    def test_truncated_body_is_400_and_oversize_is_413(self):
        self.assertEqual(self.raw_request(b"Content-Length: 8", b"{}")[0], 400)
        self.assertEqual(self.raw_request(f"Content-Length: {MAX_BODY_BYTES + 1}".encode())[0], 413)

    def test_valid_json_still_roundtrips(self):
        connection = HTTPConnection(*self.address, timeout=5)
        self.addCleanup(connection.close)
        connection.request("POST", "/echo", json.dumps({"ok": True}), {"Content-Type": "application/json"})
        response = connection.getresponse()
        self.assertEqual(response.status, 200)
        self.assertEqual(json.loads(response.read()), {"ok": True})


if __name__ == "__main__":
    unittest.main()
