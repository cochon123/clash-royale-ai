"""One warm native-watch process for Arena matches.

``ARENA_ENGINE_POOL=1`` keeps a single idle engine. ``acquire`` returns it and
starts a replacement in the background. ``release`` puts a healthy engine back
only when that idle slot is empty; anything else is closed so the service does
not accumulate processes.

``--preload`` and ``{"op":"ping"}`` are used when ``native-watch --help``
advertises them. A binary that does not is spawned the same way as today's
``JsonlEngine``, with no ping.
"""
from __future__ import annotations

import os
from pathlib import Path
import re
import subprocess
import threading

from webapps.common import paths


def parse_capabilities(help_text: str) -> tuple[bool, bool]:
    """Whether this native-watch advertises ``--preload`` and the ``ping`` op."""
    text = help_text or ""
    # A whole word, so "mapping" does not count and {"op":"ping"} does.
    return "--preload" in text, re.search(r"(?<![A-Za-z0-9_])ping(?![A-Za-z0-9_])", text) is not None


def probe_binary(binary: Path) -> tuple[bool, bool]:
    try:
        result = subprocess.run([str(binary), "--help"], capture_output=True, text=True, timeout=5)
    except (OSError, subprocess.TimeoutExpired):
        return False, False
    return parse_capabilities(f"{result.stdout}\n{result.stderr}")


class PooledEngine:
    """Line-oriented native-watch client that can be reused across matches."""

    def __init__(self, command):
        self.command = list(command)
        self.broken = False
        self._closed = False
        self.proc = subprocess.Popen(
            self.command, cwd=str(paths.ENGINE), stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=None, text=True, bufsize=1)

    def healthy(self) -> bool:
        return not self.broken and not self._closed and self.proc.poll() is None

    def request(self, value: dict) -> dict:
        try:
            if self.proc.poll() is not None:
                raise RuntimeError(f"native engine exited with status {self.proc.returncode}")
            if self.proc.stdin is None or self.proc.stdout is None:
                raise RuntimeError("native engine pipes are closed")
            import json
            self.proc.stdin.write(json.dumps(value, separators=(",", ":")) + "\n")
            self.proc.stdin.flush()
            line = self.proc.stdout.readline()
            if not line:
                raise RuntimeError("native engine closed JSONL stdout before replying")
            response = json.loads(line)
            if response.get("ok") is not True:
                raise RuntimeError(f"native engine rejected {value.get('op')}: {response.get('error')}")
            result = response.get("result")
            if not isinstance(result, dict):
                raise RuntimeError("native engine response has no result object")
            return result
        except Exception:
            self.broken = True
            raise

    def close(self) -> None:
        if self._closed:
            return
        self._closed = True
        if self.proc.poll() is None:
            try:
                if not self.broken:
                    self.request({"op": "close"})
            except Exception:
                self.broken = True
            if self.proc.stdin:
                try:
                    self.proc.stdin.close()
                except OSError:
                    pass
            try:
                self.proc.wait(timeout=10)
            except subprocess.TimeoutExpired:
                self.proc.terminate()
                try:
                    self.proc.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    self.proc.kill()
                    self.proc.wait(timeout=5)


class EnginePool:
    def __init__(self, binary, checkpoint, *, enabled=None, capabilities=None, process_factory=None):
        self.binary = Path(binary)
        self.checkpoint = Path(checkpoint)
        self.enabled = os.environ.get("ARENA_ENGINE_POOL", "0") == "1" if enabled is None else bool(enabled)
        self._capabilities = capabilities
        self.process_factory = process_factory
        self.idle = None
        self.lock = threading.Lock()
        self._starting = False
        self.thread = None

    def capabilities(self) -> tuple[bool, bool]:
        if self._capabilities is None:
            self._capabilities = probe_binary(self.binary)
        return self._capabilities

    def command(self) -> list[str]:
        preload, _ping = self.capabilities()
        command = [str(self.binary), "--checkpoint", str(self.checkpoint)]
        if preload:
            command.append("--preload")
        return command

    def warm(self):
        """Start the idle engine when the pool is enabled. Failures are logged, not fatal."""
        if self.enabled:
            self._spawn_replacement()

    def acquire(self):
        if not self.enabled:
            return self._open()
        with self.lock:
            engine = self.idle
            self.idle = None
        if engine is not None and not engine.healthy():
            engine.close()
            engine = None
        if engine is None:
            engine = self._open()
        self._spawn_replacement()
        return engine

    def release(self, engine):
        if engine is None:
            return
        if not self.enabled or not engine.healthy():
            engine.close()
            return
        with self.lock:
            if self.idle is None:
                self.idle = engine
                return
        engine.close()

    def close(self):
        with self.lock:
            engine = self.idle
            self.idle = None
        if engine is not None:
            engine.close()

    def _open(self):
        preload, ping = self.capabilities()
        command = self.command()
        engine = self.process_factory(command) if self.process_factory else PooledEngine(command)
        if preload and ping:
            try:
                result = engine.request({"op": "ping"})
            except Exception:
                engine.close()
                raise
            if not isinstance(result, dict) or result.get("ready") is not True:
                engine.close()
                raise RuntimeError(f"native-watch ping was not ready: {result!r}")
        return engine

    def _spawn_replacement(self):
        with self.lock:
            if not self.enabled or self.idle is not None or self._starting:
                return
            self._starting = True

        def run():
            engine = None
            try:
                engine = self._open()
            except Exception as exc:
                print(f"arena: engine pool spawn failed: {exc}", flush=True)
            with self.lock:
                self._starting = False
                if engine is None:
                    return
                if self.idle is None and engine.healthy():
                    self.idle = engine
                else:
                    engine.close()

        self.thread = threading.Thread(target=run, name="arena-engine-pool", daemon=True)
        self.thread.start()


_POOL = None
_POOL_LOCK = threading.Lock()


def get_pool() -> EnginePool:
    global _POOL
    with _POOL_LOCK:
        if _POOL is None:
            _POOL = EnginePool(paths.NATIVE_WATCH, paths.ENGINE_CHECKPOINT)
        return _POOL
