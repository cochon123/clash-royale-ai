"""Gymnasium wrapper for the original engine, with deliberately partial observations.

Run the native-rl binary in one subprocess. Reset reloads the original prepared
snapshot; seeds affect Python opponent sampling, not the fixed native scenario.
"""
from __future__ import annotations

import json
from pathlib import Path
import select
import subprocess
import tempfile
from typing import Any

import gymnasium as gym
from gymnasium import spaces
import numpy as np

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_CHECKPOINT = ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"
DEFAULT_BINARY = ROOT / "native-engine/target/release/native-rl"


def sample_affordable_action(raw: dict, side: int, rng: np.random.Generator) -> list[int]:
    """A weak opponent: select an affordable hand card and a random native cell."""
    available = [c for c in raw["hands"][side] if c["available"] and c["cost"] <= raw["elixir"][side]]
    if not available:
        return [0, 0, 0]
    card = available[int(rng.integers(len(available)))]
    width, height = raw["arena_cells"]
    return [int(card["slot"]) + 1, int(rng.integers(width)), int(rng.integers(height))]


class NativeClashEnv(gym.Env):
    """Control side 0 against a simple external random opponent on side 1.

    Action [card, x_cell, y_cell]: card 0 waits, 1..4 selects a current hand
    slot. Coordinates are centers of 500-unit native cells. The engine performs
    placement, cost and queue validation. Reward is +1 win, -1 loss, 0 draw or
    truncation. Observations omit unit positions/HP in this first version.
    """
    metadata = {"render_modes": []}

    def __init__(self, *, checkpoint=DEFAULT_CHECKPOINT, binary=DEFAULT_BINARY,
                 ticks_per_step=20, max_ticks=7200, opponent="random", timeout=120):
        super().__init__()
        if not 1 <= ticks_per_step <= 200:
            raise ValueError("ticks_per_step must be 1..200")
        if not 1 <= max_ticks <= 36000:
            raise ValueError("max_ticks must be 1..36000")
        if opponent not in ("random", "wait"):
            raise ValueError("opponent must be random or wait")
        self.checkpoint = Path(checkpoint).resolve()
        self.binary = Path(binary).resolve()
        self.ticks_per_step, self.max_ticks = ticks_per_step, max_ticks
        self.opponent, self.timeout = opponent, timeout
        self.action_space = spaces.MultiDiscrete([5, 36, 64])
        limit = np.iinfo(np.int32).max
        self.observation_space = spaces.Dict({
            "tick": spaces.Box(0, limit, (1,), dtype=np.int32),
            "elixir": spaces.Box(0, limit, (2,), dtype=np.int32),
            "card_ids": spaces.Box(0, limit, (2, 4), dtype=np.int32),
            "card_costs": spaces.Box(0, 15, (2, 4), dtype=np.int32),
            "card_available": spaces.MultiBinary((2, 4)),
            "entity_count": spaces.Box(0, limit, (1,), dtype=np.int32),
        })
        self._process = None
        self._stderr = None
        self._raw = None
        self._finished = True

    @property
    def raw_observation(self):
        """Native names and outcome fields alongside the fixed numeric Gym observation."""
        return self._raw

    def _start(self):
        if self._process is not None and self._process.poll() is None:
            return
        if self._process is not None:
            self.close()
        if not self.binary.exists():
            raise FileNotFoundError(f"Build first: cargo build --manifest-path {ROOT / 'native-engine/Cargo.toml'} --release --offline --bin native-rl")
        self._stderr = tempfile.TemporaryFile(mode="w+t")
        self._process = subprocess.Popen(
            [str(self.binary), "--checkpoint", str(self.checkpoint)], cwd=ROOT,
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=self._stderr,
            text=True, bufsize=1,
        )

    def _request(self, request):
        self._start()
        p = self._process
        try:
            p.stdin.write(json.dumps(request, separators=(",", ":")) + "\n")
            p.stdin.flush()
            if not select.select([p.stdout], [], [], self.timeout)[0]:
                self.close()
                raise TimeoutError("Native engine did not respond; session closed")
            line = p.stdout.readline()
        except (BrokenPipeError, OSError) as error:
            raise RuntimeError("Native engine process failed") from error
        if not line:
            self._stderr.seek(0)
            raise RuntimeError(f"Native engine exited: {self._stderr.read()[-4000:]}")
        response = json.loads(line)
        if not response.get("ok"):
            self._finished = True
            raise RuntimeError(f"Native engine error (reset required): {response.get('error')}")
        return response["result"]

    @staticmethod
    def _observation(raw):
        cards = raw["hands"]
        return {
            "tick": np.array([raw["tick"]], dtype=np.int32),
            "elixir": np.array(raw["elixir"], dtype=np.int32),
            "card_ids": np.array([[c["card_id"] or 0 for c in hand] for hand in cards], dtype=np.int32),
            "card_costs": np.array([[c["cost"] or 0 for c in hand] for hand in cards], dtype=np.int32),
            "card_available": np.array([[c["available"] for c in hand] for hand in cards], dtype=np.int8),
            "entity_count": np.array([raw["entity_count"]], dtype=np.int32),
        }

    def reset(self, *, seed=None, options=None):
        super().reset(seed=seed)
        result = self._request({"op": "reset", "max_ticks": self.max_ticks})
        raw = result["observation"]
        if raw["arena_cells"] != [36, 64]:
            raise RuntimeError("This first Gym action space supports the prepared 36x64 native arena")
        self._raw, self._finished = raw, False
        return self._observation(raw), {**result, "policy_seed": seed, "engine_seed": "fixed by checkpoint"}

    @staticmethod
    def _native_action(action, side):
        card, x, y = map(int, action)
        return None if card == 0 else {"side": side, "slot": card - 1, "x": x * 500 + 250, "y": y * 500 + 250}

    def step(self, action):
        if self._finished or self._raw is None:
            raise RuntimeError("reset() is required before step()")
        if not self.action_space.contains(action):
            raise ValueError("Action must be [0..4, 0..35, 0..63]")
        opponent = sample_affordable_action(self._raw, 1, self.np_random) if self.opponent == "random" else [0, 0, 0]
        actions = [a for a in (self._native_action(action, 0), self._native_action(opponent, 1)) if a is not None]
        result = self._request({"op": "step", "actions": actions, "ticks": self.ticks_per_step})
        raw = result["observation"]
        self._raw = raw
        terminated, truncated = result["terminated"], result["truncated"]
        self._finished = terminated or truncated
        reward = 0.0
        if terminated:
            winner = raw.get("winner_side")
            if winner not in (-1, 0, 1):
                raise RuntimeError("Native battle ended but winner is unresolved; refusing to invent a reward")
            reward = 1.0 if winner == 0 else -1.0 if winner == 1 else 0.0
        # Wall-clock timings belong in benchmarks, not deterministic transition info.
        info = {k: v for k, v in result.items() if k != "step_seconds"}
        return self._observation(raw), reward, terminated, truncated, {**info, "opponent_action": opponent}

    def close(self):
        p = self._process
        self._process = None
        if p is not None:
            if p.poll() is None:
                try:
                    p.stdin.write('{"op":"close"}\n')
                    p.stdin.flush()
                    p.wait(timeout=2)
                except (OSError, subprocess.TimeoutExpired):
                    p.terminate()
                    try:
                        p.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        p.kill()
                        p.wait()
            for pipe in (p.stdin, p.stdout):
                if pipe is not None:
                    pipe.close()
        if self._stderr is not None:
            self._stderr.close()
            self._stderr = None
        self._finished = True

    def __del__(self):
        if hasattr(self, "_process"):
            self.close()
