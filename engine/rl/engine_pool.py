"""C4 engine pool: per-process threads, or one C3 multi-session host.

``request_many([(slot, request), ...])`` returns replies in request order.
``step`` batches on the session host become one ``step_many``. Other ops send
``session`` and stay equivalent to a single-session reply.
"""
from __future__ import annotations

import json
import os
import selectors
import subprocess
import threading
import time
from concurrent import futures


def add_engine_pool_args(parser):
    parser.add_argument("--engine-pool", choices=("thread", "session"), default="thread",
                        help="thread: one rust_league_rl process per worker. session: one process, C3 step_many.")


class EnginePool:
    shared_process = False

    def call(self, slot, request, timeout=90):
        return self.request_many([(slot, request)], timeout=timeout)[0]

    def request_many(self, items, timeout=90):
        raise NotImplementedError

    def submit(self, slot, request, timeout=90):
        return self.submit_many([(slot, request)], timeout=timeout)[0]

    def submit_many(self, items, timeout=90):
        raise NotImplementedError

    def close(self):
        pass

    def alive(self):
        return True


class ThreadPoolEngine(EnginePool):
    """Current trainer: one process per slot, fanned out on a thread pool."""

    def __init__(self, clients, executor):
        self.clients = clients
        self.executor = executor

    def call(self, slot, request, timeout=90):
        # Direct call. Reset and mask workers already occupy this executor;
        # submitting back onto it deadlocks.
        return self.clients[slot].request(request, timeout)

    def request_many(self, items, timeout=90):
        if not items:
            return []
        pending = [self.executor.submit(self.call, slot, request, timeout) for slot, request in items]
        return [item.result() for item in pending]

    def submit_many(self, items, timeout=90):
        return [self.executor.submit(self.call, slot, request, timeout) for slot, request in items]


def _step_item(slot, request):
    item = {key: value for key, value in request.items() if key != "op"}
    item["session"] = slot
    return item


class MultiSessionEngine(EnginePool):
    """One ``rust_league_rl --sessions N`` process speaking C3.

    The real binary is produced by the engine workstream. Tests inject an
    in-process pipe that implements the same line protocol.
    """

    shared_process = True

    def __init__(self, binary=None, checkpoint=None, sessions=1, error_path=None, cwd=None,
                 stdin=None, stdout=None):
        if sessions < 1:
            raise ValueError("sessions must be positive")
        self.sessions = sessions
        self.lock = threading.Lock()
        self.pending = b""
        self.selector = None
        self.proc = None
        self.err = None
        self._closed = False
        if stdin is not None or stdout is not None:
            if stdin is None or stdout is None:
                raise ValueError("stdin and stdout are both required for an injected transport")
            self.stdin, self.stdout = stdin, stdout
        else:
            if binary is None or checkpoint is None:
                raise ValueError("session pool needs a binary and a checkpoint")
            self.err = error_path.open("ab")
            self.proc = subprocess.Popen(
                [str(binary), "--checkpoint", str(checkpoint), "--sessions", str(sessions)],
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=self.err, cwd=cwd, bufsize=0)
            self.stdin, self.stdout = self.proc.stdin, self.proc.stdout
        self.selector = selectors.DefaultSelector()
        self.selector.register(self.stdout, selectors.EVENT_READ)

    def alive(self):
        return self.proc is None or self.proc.poll() is None

    def request_many(self, items, timeout=90):
        if not items:
            return []
        with self.lock:
            replies = [None] * len(items)
            index = 0
            while index < len(items):
                slot, request = items[index]
                if request.get("op") == "step":
                    end = index + 1
                    while end < len(items) and items[end][1].get("op") == "step":
                        end += 1
                    chunk = items[index:end]
                    payload = {"op": "step_many", "items": [_step_item(s, req) for s, req in chunk]}
                    result = self._exchange(payload, timeout)
                    got = result.get("items") if isinstance(result, dict) else None
                    if not isinstance(got, list) or len(got) != len(chunk):
                        raise RuntimeError("step_many_count")
                    for offset, reply in enumerate(got):
                        replies[index + offset] = reply
                    index = end
                else:
                    body = dict(request)
                    body["session"] = slot
                    replies[index] = self._exchange(body, timeout)
                    index += 1
            return replies

    def submit_many(self, items, timeout=90):
        # One RPC thread. Per-item futures complete together, in request order,
        # so the trainer can still wait on a single game.
        holders = [futures.Future() for _ in items]

        def work():
            try:
                replies = self.request_many(items, timeout)
            except BaseException as exc:
                for holder in holders:
                    if not holder.done():
                        holder.set_exception(exc)
                return
            for holder, reply in zip(holders, replies):
                holder.set_result(reply)

        threading.Thread(target=work, daemon=True).start()
        return holders

    def _exchange(self, payload, timeout):
        if self.proc is not None and self.proc.poll() is not None:
            raise RuntimeError(f"native_exit:{self.proc.returncode}")
        self.stdin.write(json.dumps(payload, separators=(",", ":")).encode() + b"\n")
        self.stdin.flush()
        end = time.monotonic() + timeout
        while b"\n" not in self.pending:
            remaining = end - time.monotonic()
            if remaining <= 0 or not self.selector.select(max(0, remaining)):
                raise TimeoutError("native_response_timeout")
            chunk = os.read(self.stdout.fileno(), 65536)
            if not chunk:
                raise RuntimeError("native_stdout_closed")
            self.pending += chunk
            if len(self.pending) > 16_000_000:
                raise RuntimeError("native_response_oversized")
        line, self.pending = self.pending.split(b"\n", 1)
        value = json.loads(line)
        if value.get("ok") is not True:
            raise RuntimeError("native_rejected:" + str(value.get("error"))[:300])
        return value["result"]

    def close(self):
        if self._closed:
            return
        self._closed = True
        if self.proc is not None and self.proc.poll() is None:
            self.proc.terminate()
            try:
                self.proc.wait(timeout=3)
            except subprocess.TimeoutExpired:
                self.proc.kill()
                self.proc.wait(timeout=3)
        if self.selector is not None:
            self.selector.close()
        if self.stdin is not None:
            try:
                self.stdin.close()
            except OSError:
                pass
        if self.err is not None:
            self.err.close()


def serve_c3(read_fd, write_fd, sessions, transcript=None):
    """In-process C3 host. One line in, one envelope out. Sessions are isolated."""
    reader = os.fdopen(read_fd, "rb", buffering=0)
    writer = os.fdopen(write_fd, "wb", buffering=0)
    state = [{"tick": 0, "resets": 0, "steps": 0, "seed": None} for _ in range(sessions)]

    def handle(request):
        session = int(request.get("session", 0))
        if not 0 <= session < sessions:
            raise RuntimeError(f"bad_session:{session}")
        row = state[session]
        op = request["op"]
        if op == "reset":
            row["tick"] = 0
            row["resets"] += 1
            row["seed"] = request.get("seed")
            return {"observation": {"tick": 0, "winner_side": -1, "entity_count": 0},
                    "configured_decks": [request.get("deck0"), request.get("deck1")],
                    "session": session, "seed": row["seed"]}
        if op == "step":
            ticks = int(request.get("ticks", 0))
            row["tick"] += ticks
            row["steps"] += 1
            actions = request.get("actions") or []
            return {"observation": {"tick": row["tick"], "winner_side": -1, "entity_count": len(actions)},
                    "actions": [{"accepted": True} for _ in actions],
                    "ticks_advanced": ticks, "terminated": False, "truncated": False,
                    "session": session, "steps": row["steps"]}
        if op == "observe":
            return {"observation": {"tick": row["tick"], "winner_side": -1}, "session": session}
        if op == "placement_mask":
            return {"mask": [1] * 576, "session": session, "side": request.get("side"), "slot": request.get("slot")}
        raise RuntimeError(f"unsupported_op:{op}")

    try:
        pending = b""
        while True:
            chunk = reader.read(65536)
            if not chunk:
                break
            pending += chunk
            while b"\n" in pending:
                line, pending = pending.split(b"\n", 1)
                if not line.strip():
                    continue
                request = json.loads(line)
                if transcript is not None:
                    transcript.append(request)
                try:
                    if request.get("op") == "step_many":
                        result = {"items": [handle({"op": "step", **item}) for item in request.get("items") or []]}
                    else:
                        result = handle(request)
                    payload = {"ok": True, "result": result}
                except Exception as exc:
                    payload = {"ok": False, "error": str(exc)}
                writer.write(json.dumps(payload, separators=(",", ":")).encode() + b"\n")
                writer.flush()
    finally:
        reader.close()
        writer.close()


def open_fake_c3(sessions):
    """Return ``(engine, transcript, closer)`` connected to :func:`serve_c3`."""
    server_read, client_write = os.pipe()
    client_read, server_write = os.pipe()
    transcript = []
    thread = threading.Thread(target=serve_c3, args=(server_read, server_write, sessions, transcript), daemon=True)
    thread.start()
    engine = MultiSessionEngine(sessions=sessions, stdin=os.fdopen(client_write, "wb", buffering=0),
                                stdout=os.fdopen(client_read, "rb", buffering=0))

    def closer():
        engine.close()
        thread.join(timeout=2)

    return engine, transcript, closer
