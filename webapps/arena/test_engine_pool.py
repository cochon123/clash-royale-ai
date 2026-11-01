"""Engine pool, preload/ping degrade, weight cache and engine digest."""
from __future__ import annotations

import hashlib
import tempfile
import threading
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

from webapps.arena.engine_pool import EnginePool, parse_capabilities, probe_binary
from webapps.arena.policy_cache import engine_sha256, load_match_policy


class FakeEngine:
    def __init__(self, command, *, ping=None):
        self.command = list(command)
        self.closed = False
        self.broken = False
        self.requests = []
        self.ping = {"ready": True, "load_seconds": 0.01, "resets": 0} if ping is None else ping

    def healthy(self):
        return not self.closed and not self.broken

    def close(self):
        self.closed = True

    def request(self, value):
        self.requests.append(value)
        if value.get("op") == "ping":
            if isinstance(self.ping, Exception):
                raise self.ping
            return self.ping
        return {}


class CapabilityTests(unittest.TestCase):
    def test_help_text_detects_preload_and_ping_and_ignores_todays_binary(self):
        current = ("native-watch --checkpoint DIR; JSONL reset, observe, step, close; "
                   "reset takes deck0/deck1 arrays")
        self.assertEqual(parse_capabilities(current), (False, False))
        self.assertEqual(parse_capabilities("native-watch --checkpoint DIR --preload; ops include ping"), (True, True))
        self.assertEqual(parse_capabilities('ops: {"op":"ping"}'), (False, True))
        self.assertEqual(parse_capabilities("mapping preload"), (False, False))

    def test_probe_reads_help_without_a_checkpoint(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        binary = Path(temp.name) / "native-watch"
        binary.write_text("#!/bin/sh\nif [ \"$1\" = \"--help\" ]; then echo 'native-watch --preload ping'; exit 0; fi\n"
                          "echo should-not-load >&2\nexit 2\n")
        binary.chmod(0o755)
        self.assertEqual(probe_binary(binary), (True, True))


class PoolTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.binary = self.root / "native-watch"
        self.binary.write_text("#!/bin/sh\n")
        self.checkpoint = self.root / "state"
        self.checkpoint.mkdir()
        self.opened = []

    def factory(self, command):
        engine = FakeEngine(command)
        self.opened.append(engine)
        return engine

    def pool(self, **kwargs):
        return EnginePool(self.binary, self.checkpoint, process_factory=self.factory, **kwargs)

    def test_disabled_pool_spawns_plain_and_closes_on_release(self):
        pool = self.pool(enabled=False, capabilities=(False, False))
        engine = pool.acquire()
        self.assertNotIn("--preload", engine.command)
        self.assertEqual(engine.requests, [])
        pool.release(engine)
        self.assertTrue(engine.closed)
        self.assertIsNone(pool.idle)

    def test_preload_ping_acquire_release_keeps_one_idle_and_replaces_it(self):
        pool = self.pool(enabled=True, capabilities=(True, True))
        pool.warm()
        pool.thread.join(2)
        self.assertIs(pool.idle, self.opened[0])
        self.assertIn("--preload", self.opened[0].command)
        self.assertEqual(self.opened[0].requests, [{"op": "ping"}])
        first = pool.acquire()
        pool.thread.join(2)
        self.assertIs(first, self.opened[0])
        self.assertIs(pool.idle, self.opened[1])
        second = pool.acquire()
        pool.thread.join(2)
        replacement = pool.idle
        pool.release(first)
        pool.release(second)
        self.assertTrue(first.closed)
        self.assertTrue(second.closed)
        self.assertFalse(replacement.closed)
        self.assertIs(pool.idle, replacement)
        idle_count = sum(engine.healthy() and not engine.closed for engine in self.opened)
        self.assertEqual(idle_count, 1)

    def test_unhealthy_engine_is_closed_instead_of_pooled(self):
        pool = self.pool(enabled=True, capabilities=(False, False))
        engine = pool.acquire()
        pool.thread.join(2)
        # Drop the background replacement so the slot under test is the released engine.
        with pool.lock:
            extra = pool.idle
            pool.idle = None
        if extra is not None:
            extra.close()
        engine.broken = True
        pool.release(engine)
        self.assertTrue(engine.closed)
        self.assertIsNone(pool.idle)

    def test_ping_that_is_not_ready_closes_the_process(self):
        def factory(command):
            engine = FakeEngine(command, ping={"ready": False, "load_seconds": 1, "resets": 0})
            self.opened.append(engine)
            return engine

        pool = EnginePool(self.binary, self.checkpoint, enabled=False, capabilities=(True, True),
                          process_factory=factory)
        with self.assertRaises(RuntimeError):
            pool.acquire()
        self.assertTrue(self.opened[0].closed)

    def test_a_binary_without_ping_is_not_pinged_even_if_preload_is_present(self):
        pool = self.pool(enabled=False, capabilities=(True, False))
        engine = pool.acquire()
        self.assertIn("--preload", engine.command)
        self.assertEqual(engine.requests, [])
        engine.close()


class CacheTests(unittest.TestCase):
    def test_engine_sha256_is_read_once_per_process(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        path = Path(temp.name) / "native-watch"
        path.write_bytes(b"engine-bytes")
        calls = []
        original = Path.read_bytes

        def counting(self_path):
            if Path(self_path) == path.resolve():
                calls.append(1)
            return original(self_path)

        with patch.object(Path, "read_bytes", counting):
            first = engine_sha256(path)
            second = engine_sha256(path)
        self.assertEqual(first, hashlib.sha256(b"engine-bytes").hexdigest())
        self.assertEqual(second, first)
        self.assertEqual(calls, [1])

    def test_policy_weights_load_once_and_each_match_gets_its_own_seed(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        path = Path(temp.name) / "model.pt"
        path.write_bytes(b"weights")
        loads = []

        def real_load(file, *args, **kwargs):
            loads.append(Path(file).name)
            return {"tensors": "shared"}

        def policy(checkpoint, device, seed, specialization):
            import torch
            return SimpleNamespace(seed=seed, specialization=specialization, weights=torch.load(checkpoint))

        with patch("torch.load", side_effect=real_load), \
                patch("rl.banana_watch.load_watched_policy", side_effect=policy):
            first = load_match_policy(path, "cpu", 3, "banana-1")
            second = load_match_policy(path, "cpu", 9, "banana-1")
        self.assertEqual(loads, ["model.pt"])
        self.assertEqual((first.seed, second.seed), (3, 9))
        self.assertIs(first.weights, second.weights)
        self.assertEqual(first.specialization, "banana-1")


if __name__ == "__main__":
    unittest.main()
