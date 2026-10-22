"""Public serving checks: signed cookies, base paths and write boundaries."""
from __future__ import annotations

import unittest

try:
    from starlette.testclient import TestClient
    from .asgi import create_app
except ImportError:
    TestClient = None


@unittest.skipIf(TestClient is None, "install the arena-web extra for production adapter checks")
class PublicServingTests(unittest.TestCase):
    def client(self, prefix=""):
        return TestClient(create_app(secret="fixture-only-secret-" * 3, secure=True,
                                     base_path=prefix, allowed_hosts=["arena.example"],
                                     start_warmup=False), base_url="https://arena.example")

    def test_signed_cookie_is_secure_and_persists(self):
        with self.client() as client:
            first = client.get("/api/health")
            self.assertEqual(first.status_code, 200)
            cookie = first.headers["set-cookie"]
            self.assertIn("httponly", cookie.lower())
            self.assertIn("secure", cookie.lower())
            value = client.cookies.get("arena_session")
            client.get("/api/catalog")
            self.assertEqual(value, client.cookies.get("arena_session"))

    def test_subdirectory_serves_html_and_assets_from_same_prefix(self):
        with self.client("/clashroyale") as client:
            page = client.get("/clashroyale/")
            self.assertEqual(page.status_code, 200)
            self.assertIn('src="/clashroyale/lobby.js"', page.text)
            self.assertIn('href="/clashroyale/style.css"', page.text)
            self.assertEqual(client.get("/clashroyale/lobby.js").status_code, 200)
            self.assertEqual(client.get("/clashroyale/api/health").status_code, 200)

    def test_cross_origin_write_and_bad_host_are_rejected(self):
        with self.client() as client:
            self.assertEqual(client.post("/api/matches", json={}, headers={"Origin": "https://elsewhere.example"}).status_code, 403)
            self.assertEqual(client.get("/api/health", headers={"Host": "elsewhere.example"}).status_code, 400)

    def test_oversized_body_and_invalid_json_return_json_errors(self):
        with self.client() as client:
            response = client.post("/api/matches", content=b"x" * (1024 * 1024 + 1))
            self.assertEqual(response.status_code, 413)
            self.assertIn("error", response.json())
        with self.client() as client:
            response = client.post("/api/matches", content=b"{")
            self.assertEqual(response.status_code, 400)
            self.assertIn("error", response.json())

    def test_admission_attempts_are_throttled_per_browser(self):
        with self.client() as client:
            self.assertEqual(client.post("/api/matches", json={}).status_code, 400)
            self.assertEqual(client.post("/api/matches", json={}).status_code, 429)


if __name__ == "__main__":
    unittest.main()
