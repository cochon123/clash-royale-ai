"""C4 engine pool: thread fan-out and the C3 session protocol."""
import threading
import time
import unittest
from concurrent.futures import ThreadPoolExecutor

from .engine_pool import ThreadPoolEngine, open_fake_c3


class _Slow:
    def __init__(self, name, delay):
        self.name, self.delay = name, delay
        self.starts, self.ends, self.payloads = [], [], []

    def request(self, payload, timeout=90):
        self.starts.append(time.perf_counter())
        time.sleep(self.delay)
        self.ends.append(time.perf_counter())
        self.payloads.append(payload)
        return {"name": self.name, "op": payload.get("op"), "mark": payload.get("mark")}


class ThreadPoolTests(unittest.TestCase):
    def test_request_many_preserves_order_and_overlaps(self):
        slow, fast = _Slow("slow", 0.05), _Slow("fast", 0.01)
        executor = ThreadPoolExecutor(max_workers=2)
        pool = ThreadPoolEngine([slow, fast], executor)
        try:
            replies = pool.request_many([(0, {"op": "step", "mark": "a"}), (1, {"op": "step", "mark": "b"})])
        finally:
            executor.shutdown(wait=True)
        self.assertEqual([row["name"] for row in replies], ["slow", "fast"])
        self.assertEqual(replies[0]["mark"], "a")
        self.assertLess(fast.starts[0], slow.ends[0])

    def test_submit_many_matches_direct_calls(self):
        left, right = _Slow("L", 0.0), _Slow("R", 0.0)
        executor = ThreadPoolExecutor(max_workers=2)
        pool = ThreadPoolEngine([left, right], executor)
        try:
            futures = pool.submit_many([(1, {"op": "reset", "mark": 1}), (0, {"op": "step", "mark": 0})])
            replies = [future.result(timeout=2) for future in futures]
        finally:
            executor.shutdown(wait=True)
        self.assertEqual([(row["name"], row["mark"]) for row in replies], [("R", 1), ("L", 0)])


class SessionProtocolTests(unittest.TestCase):
    def test_step_many_matches_single_session_replies_and_isolates_sessions(self):
        engine, transcript, close = open_fake_c3(2)
        try:
            engine.call(0, {"op": "reset", "deck0": ["HogRider"], "deck1": ["Knight"], "seed": 11})
            engine.call(1, {"op": "reset", "deck0": ["Knight"], "deck1": ["HogRider"], "seed": 22})
            single = [
                engine.call(0, {"op": "step", "actions": [], "ticks": 20}),
                engine.call(1, {"op": "step", "actions": [{"side": 0}], "ticks": 40}),
            ]
            batched = engine.request_many([
                (0, {"op": "step", "actions": [], "ticks": 20}),
                (1, {"op": "step", "actions": [{"side": 0}], "ticks": 40}),
            ])
            self.assertEqual(batched[0]["ticks_advanced"], single[0]["ticks_advanced"])
            self.assertEqual(batched[1]["ticks_advanced"], single[1]["ticks_advanced"])
            self.assertEqual(batched[0]["session"], 0)
            self.assertEqual(batched[1]["session"], 1)
            self.assertEqual(batched[0]["observation"]["tick"], 40)
            self.assertEqual(batched[1]["observation"]["tick"], 80)
            observed = engine.call(0, {"op": "observe"})
            self.assertEqual(observed["observation"]["tick"], 40)
            mask = engine.call(1, {"op": "placement_mask", "side": 1, "slot": 2})
            self.assertEqual(len(mask["mask"]), 576)
            self.assertEqual(mask["session"], 1)
        finally:
            close()
        batched_ops = [row for row in transcript if row.get("op") == "step_many" and len(row.get("items", [])) == 2]
        self.assertEqual(len(batched_ops), 1)
        self.assertEqual([item["session"] for item in batched_ops[0]["items"]], [0, 1])
        self.assertTrue(all("session" in row for row in transcript if row.get("op") != "step_many"))

    def test_mixed_batch_keeps_request_order(self):
        engine, _transcript, close = open_fake_c3(2)
        try:
            engine.call(0, {"op": "reset", "deck0": ["A"], "deck1": ["B"], "seed": 1})
            engine.call(1, {"op": "reset", "deck0": ["A"], "deck1": ["B"], "seed": 2})
            replies = engine.request_many([
                (1, {"op": "observe"}),
                (0, {"op": "step", "actions": [], "ticks": 15}),
                (1, {"op": "step", "actions": [], "ticks": 5}),
            ])
        finally:
            close()
        self.assertEqual(replies[0]["observation"]["tick"], 0)
        self.assertEqual(replies[1]["observation"]["tick"], 15)
        self.assertEqual(replies[2]["observation"]["tick"], 5)


if __name__ == "__main__":
    unittest.main()
