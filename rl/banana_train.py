"""Native-engine PPO league for the two Banana specialists.

The actual engine protocol is intentionally strict: every reset must return
configured_decks and a native observation; every step must return one action
result per submitted action. Use --smoke for the bounded in-process protocol
exercise. Real training starts its 3-hour clock only after both native deck
preflights pass.
"""
from __future__ import annotations

import argparse
import copy
import datetime as dt
import hashlib
import json
import math
import os
import random
import subprocess
import sys
import tempfile
import time
from collections import Counter, deque
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Mapping, Sequence

import torch
from torch.nn import functional as F

try:
    from .banana_policy import BananaEpisode, BananaPolicy, SampledAction, _native_card
except ImportError:
    from banana_policy import BananaEpisode, BananaPolicy, SampledAction, _native_card

ROOT = Path(__file__).resolve().parents[1]
RUN_DIR = ROOT / "runs" / "banana-league-20261019"
ENGINE_DIR = ROOT / "native-engine"
CHECKPOINT_ROOT = ROOT.parent / "clash-royale-ai" / "models" / "policy_bc_v4.4.2"
BASE_CHECKPOINT = CHECKPOINT_ROOT / "best_model.pt"
SOURCE_SHA = ""
DECKS: dict[str, tuple[str, ...]] = {
    "hog": ("hog-rider", "musketeer", "ice-golem", "skeletons", "ice-spirit", "cannon", "fireball", "the-log"),
    "mortar": ("mortar", "knight", "archers", "skeletons", "barbarian-barrel", "arrows", "mini-pekka", "rocket"),
}
PRESETS = {"hog": "hog26", "mortar": "pixel9_mortar"}
NAMES = {"hog": "banana-hog-1", "mortar": "banana-mortar-1"}
# The captured original game's timer advances in 50 ms units; seconds here are
# a presentation conversion for model timestamps and dashboard match lengths.
TICKS_PER_SECOND = 20.0
INPUT_WARMUP_TICKS = 100
MATCH_TICKS = 7200
PROBE_CARDS = {
    "hog": {"hog-rider", "musketeer", "ice-golem", "skeletons", "ice-spirit"},
    "mortar": {"knight", "archers", "skeletons", "mini-pekka"},
}


def utc_now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat()


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as src:
        for block in iter(lambda: src.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def atomic_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temp_name = tempfile.mkstemp(prefix=f".{path.name}.", suffix=".tmp", dir=path.parent)
    try:
        with os.fdopen(fd, "w") as out:
            json.dump(value, out, indent=2, allow_nan=False)
            out.write("\n")
            out.flush()
            os.fsync(out.fileno())
        os.replace(temp_name, path)
    finally:
        if os.path.exists(temp_name):
            os.unlink(temp_name)


def log_line(run_dir: Path, message: str, progress: Mapping[str, Any] | None = None) -> None:
    stamp = utc_now()
    line = f"[{stamp}] {message}"
    print(line, flush=True)
    with (run_dir / "train.log").open("a") as out:
        out.write(line + "\n")
    if progress is not None:
        with (run_dir / "progress.jsonl").open("a") as out:
            out.write(json.dumps({"at": stamp, **progress}, allow_nan=False) + "\n")


class JsonlEngine:
    """Strict line-oriented client for the independently verified native bin."""

    def __init__(self, checkpoint: Path, engine_bin: Path | None = None):
        if engine_bin:
            command = [str(engine_bin), "--checkpoint", str(checkpoint)]
        else:
            command = ["cargo", "run", "--quiet", "--release", "--manifest-path",
                       str(ENGINE_DIR / "Cargo.toml"), "--bin", "deck-setup-probe", "--",
                       "--checkpoint", str(checkpoint)]
        self.proc = subprocess.Popen(command, cwd=ROOT, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                     stderr=None, text=True, bufsize=1)

    def request(self, value: Mapping[str, Any]) -> dict[str, Any]:
        if self.proc.poll() is not None:
            raise RuntimeError(f"native engine exited with status {self.proc.returncode}")
        assert self.proc.stdin is not None and self.proc.stdout is not None
        self.proc.stdin.write(json.dumps(value, separators=(",", ":")) + "\n")
        self.proc.stdin.flush()
        line = self.proc.stdout.readline()
        if not line:
            raise RuntimeError("native engine closed JSONL stdout before replying")
        try:
            response = json.loads(line)
        except json.JSONDecodeError as exc:
            raise RuntimeError(f"native engine returned invalid JSON: {line[:500]!r}") from exc
        if response.get("ok") is not True:
            raise RuntimeError(f"native engine rejected {value.get('op')}: {response.get('error')}")
        result = response.get("result")
        if not isinstance(result, dict):
            raise RuntimeError("native engine response has no result object")
        return result

    def close(self) -> None:
        if self.proc.poll() is None:
            try:
                self.request({"op": "close"})
            except Exception:
                pass
            if self.proc.stdin:
                self.proc.stdin.close()
            try:
                self.proc.wait(timeout=10)
            except subprocess.TimeoutExpired:
                self.proc.terminate()
                self.proc.wait(timeout=5)


class FakeEngine:
    """Tiny protocol simulator used only by --smoke; never reports real wins."""

    def __init__(self, steps_to_end: int = 32):
        self.steps_to_end = steps_to_end
        self.counter = 0
        self.seed = 0
        self.deck: tuple[str, ...] = ()
        self.observation: dict[str, Any] = {}
        self.done = False

    @staticmethod
    def _obs(deck: Sequence[str], tick: int = 2, elixir: int = 10) -> dict[str, Any]:
        hands = []
        for side in range(2):
            rows = []
            for slot, card in enumerate(deck[:4]):
                rows.append({"slot": slot, "card_id": 0, "name": card, "cost": 1, "available": True})
            hands.append(rows)
        return {"tick": tick, "ticks": [tick, tick], "terminated": False, "winner_side": None,
                "entity_count": 6, "elixir": [elixir, elixir], "hands": hands,
                "sides": [{"side": 0, "elixir": elixir}, {"side": 1, "elixir": elixir}],
                "arena_cells": [36, 64]}

    def request(self, value: Mapping[str, Any]) -> dict[str, Any]:
        op = value.get("op")
        if op == "reset":
            key = "hog" if value.get("deck_preset") == "hog26" else "mortar"
            self.deck = DECKS[key]
            self.seed = int(value.get("seed", 0))
            self.counter = 0
            self.done = False
            self.observation = self._obs(self.deck)
            return {"observation": self.observation, "configured_decks": [list(self.deck), list(self.deck)],
                    "engine_reset": "smoke protocol emulator", "max_ticks": 7200}
        if op == "step":
            if self.done:
                raise RuntimeError("fake episode already done")
            actions = list(value.get("actions", []))
            results = [{"accepted": True, "side": int(a["side"]), "hand_slot": int(a["slot"]),
                        "cost": 1, "rejection": None} for a in actions]
            self.counter += 1
            tick = 2 + self.counter * 20
            self.done = self.counter >= self.steps_to_end
            self.observation = self._obs(self.deck, tick)
            winner = self.seed % 2 if self.done else None
            self.observation["terminated"] = self.done
            self.observation["winner_side"] = winner
            return {"observation": self.observation, "actions": results,
                    "terminated": self.done, "truncated": False, "ticks_advanced": 20}
        raise RuntimeError(f"unsupported fake op {op!r}")

    def close(self) -> None:
        pass


@dataclass
class Transition:
    action: SampledAction
    outcome: float
    old_value: float


class BananaTrainer:
    def __init__(self, args: argparse.Namespace, engine: Any):
        self.args = args
        self.engine = engine
        self.fake_protocol = isinstance(engine, FakeEngine)
        self.run_dir: Path = args.run_dir
        self.run_dir.mkdir(parents=True, exist_ok=True)
        self.policies = {
            key: BananaPolicy(device=args.device, seed=args.seed + i, specialization=NAMES[key])
            for i, key in enumerate(("hog", "mortar"))
        }
        self.baseline = BananaPolicy(device=args.device, seed=args.seed + 100, specialization="frozen-4.4.2")
        for param in self.baseline.parameters():
            param.requires_grad_(False)
        self.optimizers = {
            key: torch.optim.Adam(policy.parameters(), lr=args.learning_rate)
            for key, policy in self.policies.items()
        }
        self.buffers: dict[str, list[Transition]] = {"hog": [], "mortar": []}
        self.training_results: dict[str, list[str]] = {"hog": [], "mortar": []}
        self.evaluation_results: dict[str, list[str]] = {"hog": [], "mortar": []}
        self.deck_plays: Counter[str] = Counter()
        self.run_started = 0.0
        self.run_deadline = 0.0
        self.game_number = 0
        self.training_games: dict[str, int] = {"hog": 0, "mortar": 0}
        self.decision_steps = 0
        self.accepted_plays = 0
        self.rejected_plays = 0
        self.native_rejections: Counter[str] = Counter()
        self.completed_games = 0
        self.truncated_games = 0
        self.updates = {"hog": 0, "mortar": 0}
        self.latest_update: dict[str, float] | None = None
        self.recent_games: deque[dict[str, Any]] = deque(maxlen=20)
        self.history: deque[dict[str, Any]] = deque(maxlen=20)
        self.streaks = {"hog": 0, "mortar": 0}
        self.longest_streak: tuple[int, str] = (0, "")
        self.fun_quickest: dict[str, Any] | None = None
        self.fun_longest: dict[str, Any] | None = None
        self.state = self._initial_state()
        self._save_initial_models()

    def _initial_state(self) -> dict[str, Any]:
        if (self.run_dir / "STOP").exists():
            raise RuntimeError("run directory has an existing STOP request; use a fresh run directory or remove it intentionally")
        if (self.run_dir / "state.json").exists():
            try:
                state = json.loads((self.run_dir / "state.json").read_text())
                if state.get("status") == "running":
                    raise RuntimeError("run directory already has a running state; choose another --run-dir")
                state["status"] = "preparing"
                state["phase"] = "native_deck_validation"
                state["message"] = "Validating native deck presets before training clock starts."
                state["error"] = None
                return state
            except (json.JSONDecodeError, OSError):
                pass
        return {
            "status": "preparing", "phase": "native_deck_validation",
            "message": "Validating both native deck presets before starting the training clock.",
            "duration_seconds": self.args.duration_seconds, "elapsed_seconds": 0,
            "completed_games": 0, "truncated_games": 0, "decision_steps": 0,
            "accepted_plays": 0, "rejected_plays": 0, "native_rejections": {},
            "models": {}, "history": [], "recent_games": [],
            "fun": {}, "backend": "Original engine (Unicorn) · preflight pending",
        }

    def _model_state(self, key: str) -> dict[str, Any]:
        policy = self.policies[key]
        return {
            "name": NAMES[key], "deck": list(DECKS[key]), "updates": self.updates[key],
            "stats": {"wins": 0, "losses": 0, "draws": 0},
            "evaluation": {"games": 0, "wins": 0, "losses": 0, "draws": 0},
            "accepted_plays": 0, "rejected_plays": 0,
        }

    def _save_initial_models(self) -> None:
        self.state.setdefault("models", {})
        for key in ("hog", "mortar"):
            self.state["models"][key] = self._model_state(key)
        self.state["source_checkpoint"] = str(BASE_CHECKPOINT)
        self.state["source_checkpoint_sha256"] = sha256(BASE_CHECKPOINT)
        self.state["backend"] = ("FAKE protocol emulator · smoke only" if self.fake_protocol
                                  else "Original engine (Unicorn) · deck setup pending")
        self._write_state()

    def _write_state(self) -> None:
        now = time.time()
        if self.run_started:
            elapsed = max(0.0, now - self.run_started)
            self.state.update({"elapsed_seconds": elapsed, "deadline": utc_from_epoch(self.run_deadline),
                               "started_at": utc_from_epoch(self.run_started)})
        self.state.update({
            "updated_at": utc_now(), "completed_games": self.completed_games,
            "truncated_games": self.truncated_games, "decision_steps": self.decision_steps,
            "accepted_plays": self.accepted_plays, "models": self.state.get("models", {}),
            "rejected_plays": self.rejected_plays, "native_rejections": dict(self.native_rejections),
            "history": list(self.history), "recent_games": list(self.recent_games),
            "last_update": self.latest_update, "elixir_spent": sum(self.elixir_spent.values()) if hasattr(self, "elixir_spent") else 0,
        })
        atomic_json(self.run_dir / "state.json", self.state)

    def _save_checkpoint(self, key: str, initial: bool = False, final: bool = False) -> None:
        policy = self.policies[key]
        outdir = self.run_dir / "models" / key
        outdir.mkdir(parents=True, exist_ok=True)
        name = "initial.pt" if initial else "final.pt" if final else "latest.pt"
        path = outdir / name
        temp = outdir / f".{name}.{os.getpid()}.tmp"
        payload = {
            "format": "banana-native-ppo-v1", "name": NAMES[key], "specialist": key,
            "checkpoint_stage": "initial" if initial else "final" if final else "latest",
            "model_state": policy.model.state_dict(), "critic_state": policy.critic.state_dict(),
            "wait_state": policy.wait_head.state_dict(),
            "optimizer_state": None if initial else self.optimizers[key].state_dict(),
            "vocab": policy.vocab.to_dict(), "source_checkpoint": str(BASE_CHECKPOINT),
            "source_checkpoint_sha256": self.state.get("source_checkpoint_sha256"),
            "model_version": "4.4.2", "config": dict(policy.config),
            "deck": list(DECKS[key]), "seed": self.args.seed, "updates": self.updates[key],
            "trainer": {"learning_rate": self.args.learning_rate, "clip_ratio": self.args.clip_ratio,
                        "epochs": self.args.epochs, "minibatch_size": self.args.minibatch_size,
                        "duration_seconds": self.args.duration_seconds, "ticks_per_second": TICKS_PER_SECOND},
            "created_at": utc_now(),
        }
        torch.save(payload, temp)
        with temp.open("rb") as f:
            os.fsync(f.fileno())
        os.replace(temp, path)
        model_state = self.state["models"][key]
        model_state["checkpoint_at"] = utc_now()
        model_state["checkpoint"] = str(path)
        self._write_state()

    def _verify_configured_decks(self, key: str, reset: Mapping[str, Any]) -> None:
        actual = reset.get("configured_decks")
        if not isinstance(actual, Sequence) or len(actual) != 2:
            raise RuntimeError("native reset must return configured_decks for both sides")
        expected = list(DECKS[key])
        for side, rows in enumerate(actual):
            if isinstance(rows, Mapping):
                rows = rows.get("cards")
            if not isinstance(rows, Sequence):
                raise RuntimeError(f"native configured_decks[{side}] is malformed")
            actual_slugs = []
            for row in rows:
                if isinstance(row, str):
                    compact = "".join(ch.lower() for ch in row if ch.isalnum())
                    actual_slugs.append(self.policies[key].native_name_to_policy.get(compact)
                                        or _native_card(row, expected))
                elif isinstance(row, Mapping):
                    card_id = row.get("card_id", row.get("id"))
                    if card_id is not None:
                        mapped = self.policies[key].native_id_to_policy.get(int(card_id))
                    else:
                        mapped = None
                    raw_name = str(row.get("name", ""))
                    compact = "".join(ch.lower() for ch in raw_name if ch.isalnum())
                    actual_slugs.append(mapped or self.policies[key].native_name_to_policy.get(compact)
                                        or _native_card(raw_name, expected))
                else:
                    raise RuntimeError(f"native deck row has unsupported shape: {row!r}")
            if len(actual_slugs) != len(expected) or sorted(actual_slugs) != sorted(expected):
                raise RuntimeError(f"native side {side} deck mismatch for {key}: expected exact card set {expected}, got {tuple(actual_slugs)}")

    def _reset(self, key: str, seed: int) -> tuple[BananaEpisode, dict[str, Any], int]:
        reset = self.engine.request({"op": "reset", "max_ticks": MATCH_TICKS + INPUT_WARMUP_TICKS, "seed": seed,
                                     "deck_preset": PRESETS[key]})
        self._verify_configured_decks(key, reset)
        observation = reset.get("observation")
        if not isinstance(observation, dict) or not isinstance(observation.get("hands"), list):
            raise RuntimeError("native reset returned no full observation")
        if not self.fake_protocol:
            warm = self.engine.request({"op": "step", "actions": [], "ticks": INPUT_WARMUP_TICKS})
            observation = warm.get("observation")
            if not isinstance(observation, dict) or int(warm.get("ticks_advanced", 0)) < INPUT_WARMUP_TICKS:
                raise RuntimeError("original engine did not advance through its 100-tick input warm-up")
        configured = reset["configured_decks"]
        # Both sides are exact configured deck copies for this deck-preset mode.
        episode = BananaEpisode((DECKS[key], DECKS[key]), battle_id=f"{key}-{seed}")
        # Dashboard/model clock begins after original harness input warm-up.
        start_tick = int(observation["tick"])
        return episode, observation, start_tick

    def preflight(self) -> None:
        self.state["phase"] = "native_deck_validation"
        self.state["message"] = ("Checking fake-protocol plumbing only; this is not a native engine test."
                                  if self.fake_protocol else
                                  "Checking both exact native presets and a real play/step round trip.")
        self._write_state()
        for i, key in enumerate(("hog", "mortar")):
            episode, obs, start_tick = self._reset(key, self.args.seed + i)
            accepted_sides: set[int] = set()
            tried_cards: dict[int, set[str]] = {0: set(), 1: set()}
            probe_cards: dict[int, str] = {}
            ticks_advanced = 0
            for _attempt in range(8):
                native_actions = []
                for side in (0, 1):
                    if side in accepted_sides:
                        continue
                    rows = [row for row in obs["hands"][side] if row.get("available", True)
                            and _native_card(str(row.get("name", "")), DECKS[key]) in PROBE_CARDS[key]]
                    rows.sort(key=lambda row: int(row.get("cost", 99)))
                    choice = next((row for row in rows
                                   if _native_card(str(row.get("name", "")), DECKS[key]) not in tried_cards[side]
                                   and _native_card(str(row.get("name", "")), DECKS[key]) in DECKS[key]
                                   and int(row.get("cost", 99)) <= int(obs["elixir"][side])), None)
                    if choice is None:
                        continue
                    card = _native_card(str(choice["name"]), DECKS[key])
                    tried_cards[side].add(card)
                    probe_cards[side] = card
                    native_actions.append({"side": side, "slot": int(choice["slot"]), "x": 4500, "y": 4500})
                if not native_actions:
                    break
                response = self.engine.request({"op": "step", "actions": native_actions, "ticks": 20})
                results = response.get("actions")
                if not isinstance(results, list) or len(results) != len(native_actions):
                    raise RuntimeError(f"preflight {key}: native step did not return one result per action")
                ticks_advanced += int(response.get("ticks_advanced", 0))
                obs = response.get("observation")
                if not isinstance(obs, dict):
                    raise RuntimeError(f"preflight {key}: native step returned no observation")
                for action, result in zip(native_actions, results):
                    if not isinstance(result, Mapping) or not isinstance(result.get("accepted"), bool):
                        raise RuntimeError("native play response omitted boolean accepted status")
                    if result["accepted"]:
                        accepted_sides.add(int(action["side"]))
            accepted = len(accepted_sides)
            backend_label = "fake emulator" if self.fake_protocol else "native engine"
            if accepted_sides != {0, 1}:
                raise RuntimeError(f"preflight {key}: expected accepted {backend_label} deployments from both physical sides, got {accepted}/2")
            log_line(self.run_dir, f"Preflight {key} passed: exact deck sets; probe deployments accepted by {backend_label} for both sides.",
                     {"event": "preflight", "deck": key, "accepted": accepted,
                      "probe_cards": {str(side): card for side, card in probe_cards.items()},
                      "test_xy": [4500, 4500], "ticks_advanced": ticks_advanced, "start_tick": start_tick})
        self.state["backend"] = ("FAKE protocol emulator · smoke only; not a native engine result"
                                  if self.fake_protocol else
                                  "Original engine (Unicorn) · both deck presets and card-play path verified")
        self._write_state()

    def run_episode(self, key: str, game_seed: int, evaluation: bool = False) -> dict[str, Any]:
        episode, obs, start_tick = self._reset(key, game_seed)
        specialist_side = (len(self.evaluation_results[key]) if evaluation else self.training_games[key]) % 2
        train_policy = self.policies[key]
        decisions: list[SampledAction] = []
        accepted_cards: list[tuple[str, int]] = []
        steps = 0
        accepted_count = 0
        rejected_count = 0
        rejection_counts: Counter[str] = Counter()
        step_started = time.monotonic()
        final = None
        while True:
            if (self.run_dir / "STOP").exists() and self.run_started:
                raise StopRequested()
            if self.run_started and time.time() >= self.run_deadline:
                final = {"winner_side": None, "terminated": False, "truncated": True}
                break
            tick = int(obs["tick"])
            game_seconds = max(0.0, (tick - start_tick) / TICKS_PER_SECOND)
            sampled: list[tuple[int, BananaPolicy, SampledAction]] = []
            for side in (0, 1):
                agent = train_policy if side == specialist_side else self.baseline
                action = agent.act(episode, obs, side, game_seconds,
                                   deterministic=evaluation or side != specialist_side)
                sampled.append((side, agent, action))
            native_actions = [action.native_request() for _, _, action in sampled if not action.wait]
            for side, agent, action in sampled:
                self.decision_steps += 1
                if side == specialist_side and not evaluation:
                    decisions.append(action)
            response = self.engine.request({"op": "step", "actions": native_actions, "ticks": 20})
            results = response.get("actions")
            if not isinstance(results, list) or len(results) != len(native_actions):
                raise RuntimeError(f"native step returned {len(results) if isinstance(results, list) else 'no'} results for {len(native_actions)} actions")
            result_index = 0
            for side, agent, action in sampled:
                if action.wait:
                    continue
                result = results[result_index]
                result_index += 1
                if not isinstance(result, Mapping) or not isinstance(result.get("accepted"), bool):
                    raise RuntimeError("native action result missing boolean accepted")
                if result["accepted"]:
                    agent.record_action(episode, action, game_seconds, True)
                    accepted_count += 1
                    self.accepted_plays += 1
                    self.state["models"][key]["accepted_plays"] = int(self.state["models"][key].get("accepted_plays", 0)) + int(side == specialist_side)
                    accepted_cards.append((action.card, int(result.get("cost", self.policies[key].costs.get(action.card, 0)))))
                    self.deck_plays[action.card] += 1
                    if hasattr(self, "elixir_spent"):
                        self.elixir_spent[action.card] += accepted_cards[-1][1]
                else:
                    reason = str(result.get("rejection") or "native_rejection_unspecified")
                    rejection_counts[reason] += 1
                    self.rejected_plays += 1
                    self.native_rejections[reason] += 1
                    if side == specialist_side:
                        rejected_count += 1
                        self.state["models"][key]["rejected_plays"] = int(
                            self.state["models"][key].get("rejected_plays", 0)) + 1
            steps += 1
            obs = response.get("observation")
            if not isinstance(obs, dict):
                raise RuntimeError("native step returned no observation")
            if self.run_started and not evaluation:
                self._heartbeat(f"{NAMES[key]} game in progress · step {steps}")
            if response.get("terminated") is True:
                winner = obs.get("winner_side")
                if winner not in (0, 1, -1):
                    raise RuntimeError(f"native terminal state has unresolved winner {winner!r}; refusing to invent reward")
                final = {"winner_side": int(winner), "terminated": True, "truncated": False}
                break
            if response.get("truncated") is True:
                final = {"winner_side": None, "terminated": False, "truncated": True}
                break
            if int(obs["tick"]) >= start_tick + MATCH_TICKS:
                raise RuntimeError("native engine reached max ticks without a terminal or truncated flag")
        game_seconds = max(0.0, (int(obs["tick"]) - start_tick) / TICKS_PER_SECOND)
        if final["terminated"]:
            winner_side = final["winner_side"]
            outcome = 0.0 if winner_side == -1 else (1.0 if winner_side == specialist_side else -1.0)
            if not evaluation:
                for action in decisions:
                    self.buffers[key].append(Transition(action, outcome, float(action.value.detach().cpu())))
            winner_name = "Draw" if winner_side == -1 else (NAMES[key] if winner_side == specialist_side else "frozen-4.4.2")
            if evaluation:
                self.evaluation_results[key].append("draw" if outcome == 0 else ("win" if outcome > 0 else "loss"))
            else:
                self.training_games[key] += 1
                self.training_results[key].append("draw" if outcome == 0 else ("win" if outcome > 0 else "loss"))
                if outcome > 0:
                    self.streaks[key] += 1
                    if self.streaks[key] > self.longest_streak[0]:
                        self.longest_streak = (self.streaks[key], NAMES[key])
                else:
                    self.streaks[key] = 0
            self.completed_games += 1
        else:
            winner_name = "Truncated"
            self.truncated_games += 1
            outcome = None
        result = {"game": self.completed_games + self.truncated_games, "model": NAMES[key],
                  "winner": winner_name, "game_seconds": game_seconds,
                  "kind": (("fake smoke evaluation" if evaluation else "fake protocol smoke") if self.fake_protocol else
                           ("evaluation vs frozen 4.4.2" if evaluation else "training vs frozen 4.4.2")),
                  "truncated": bool(final["truncated"]), "decision_steps": steps * 2,
                  "accepted_plays": accepted_count, "specialist_side": specialist_side,
                  "rejected_plays": rejected_count, "rejections": dict(rejection_counts),
                  "outcome": outcome, "elapsed_wall_seconds": time.monotonic() - step_started}
        return result

    def _update(self, key: str, force: bool = False) -> None:
        buffer = self.buffers[key]
        if len(buffer) < self.args.minibatch_size:
            return
        policy, optimizer = self.policies[key], self.optimizers[key]
        chunk = buffer[:]
        random.Random(self.args.seed + self.updates[key] * 7919).shuffle(chunk)
        outcomes = torch.tensor([t.outcome for t in chunk], dtype=torch.float32, device=policy.device)
        # A single-episode batch has a constant sparse terminal reward. Mean
        # centering would erase that actor signal, so preserve its sign and
        # normalize only by RMS when mixed outcomes share a buffer.
        advantages = outcomes / outcomes.square().mean().sqrt().clamp_min(1.0)
        old_log_probs = torch.stack([t.action.log_prob.detach().to(policy.device) for t in chunk])
        action_rows = [t.action for t in chunk]
        actor_losses, value_losses, entropy_values = [], [], []
        for _epoch in range(self.args.epochs):
            order = torch.randperm(len(chunk), device="cpu").tolist()
            for start in range(0, len(chunk), self.args.minibatch_size):
                ids = order[start:start + self.args.minibatch_size]
                if not ids:
                    continue
                rows = [action_rows[i] for i in ids]
                new_log_prob, entropy, values = policy.evaluate_actions(rows)
                ix = torch.tensor(ids, dtype=torch.long, device=policy.device)
                ratio = torch.exp(new_log_prob - old_log_probs[ix])
                adv = advantages[ix]
                unclipped = ratio * adv
                clipped = ratio.clamp(1.0 - self.args.clip_ratio, 1.0 + self.args.clip_ratio) * adv
                actor_loss = -torch.minimum(unclipped, clipped).mean()
                value_loss = F.mse_loss(values, outcomes[ix])
                entropy_mean = entropy.mean()
                loss = actor_loss + self.args.value_coef * value_loss - self.args.entropy_coef * entropy_mean
                if not torch.isfinite(loss):
                    raise RuntimeError(f"non-finite PPO loss for {key}")
                optimizer.zero_grad(set_to_none=True)
                loss.backward()
                torch.nn.utils.clip_grad_norm_(policy.parameters(), 1.0)
                optimizer.step()
                actor_losses.append(float(actor_loss.detach().cpu()))
                value_losses.append(float(value_loss.detach().cpu()))
                entropy_values.append(float(entropy_mean.detach().cpu()))
        self.buffers[key] = buffer[len(chunk):]
        self.updates[key] += 1
        self.state["models"][key]["updates"] = self.updates[key]
        self.latest_update = {"policy_loss": sum(actor_losses) / max(1, len(actor_losses)),
                              "value_loss": sum(value_losses) / max(1, len(value_losses)),
                              "entropy": sum(entropy_values) / max(1, len(entropy_values)),
                              "model": NAMES[key], "samples": len(chunk)}
        self._save_checkpoint(key)
        log_line(self.run_dir, f"PPO update {self.updates[key]} {NAMES[key]} · {len(chunk)} actions · actor {self.latest_update['policy_loss']:.4f} · critic {self.latest_update['value_loss']:.4f} · entropy {self.latest_update['entropy']:.3f}",
                 {"event": "update", "model": key, **self.latest_update})
        self._write_state()

    def _record_completed(self, result: Mapping[str, Any]) -> None:
        key = "hog" if result["model"] == NAMES["hog"] else "mortar"
        result_key = "evaluation" if str(result["kind"]).startswith("evaluation") else "stats"
        metric = self.state["models"][key][result_key]
        if result_key == "evaluation":
            metric["games"] = int(metric.get("games", 0)) + 1
        if result["outcome"] is not None:
            metric["wins" if result["outcome"] > 0 else "losses" if result["outcome"] < 0 else "draws"] += 1
        if self.fun_quickest is None or float(result["game_seconds"]) < float(self.fun_quickest["seconds"]):
            self.fun_quickest = {"seconds": result["game_seconds"], "winner": result["winner"]}
        if self.fun_longest is None or float(result["game_seconds"]) > float(self.fun_longest["seconds"]):
            self.fun_longest = {"seconds": result["game_seconds"]}
        self.recent_games.append(dict(result))
        if result_key == "stats":
            def rolling(model_key: str) -> float | None:
                values = self.training_results[model_key][-20:]
                if not values:
                    return None
                return (values.count("win") + .5 * values.count("draw")) / len(values)
            self.history.append({"game": self.completed_games, "hog": rolling("hog"), "mortar": rolling("mortar")})
        self._refresh_fun()

    def _refresh_fun(self) -> None:
        fav = self.deck_plays.most_common(1)
        self.state["fun"] = {
            "quickest": self.fun_quickest, "longest": self.fun_longest,
            "streak": {"wins": self.longest_streak[0], "model": self.longest_streak[1]} if self.longest_streak[0] else None,
            "favourite": {"card": fav[0][0], "count": fav[0][1]} if fav else None,
        }

    def _heartbeat(self, message: str | None = None) -> None:
        now = time.time()
        self.state["status"] = "running"
        self.state["phase"] = "training"
        if message:
            self.state["message"] = message
        self._write_state()
        if now - getattr(self, "_last_terminal_heartbeat", 0) >= self.args.heartbeat_seconds:
            elapsed = max(1.0, now - self.run_started)
            rate = self.completed_games / elapsed
            remain = max(0.0, self.run_deadline - now)
            eta_games = int(rate * remain)
            log_line(self.run_dir, f"Heartbeat · {self.completed_games} completed + {self.truncated_games} truncated · {self.decision_steps} decisions · {self.updates['hog'] + self.updates['mortar']} updates · ETA {remain/3600:.2f}h · projected {eta_games} more matches")
            self._last_terminal_heartbeat = now

    def _evaluate(self, key: str, count: int) -> None:
        self.state["phase"] = "baseline_evaluation"
        self.state["message"] = f"Evaluating {NAMES[key]} against frozen 4.4.2."
        self._write_state()
        for index in range(count):
            if self.run_started and time.time() >= self.run_deadline:
                break
            seed = self.args.seed + 1_000_000 + self.game_number * 100 + index
            result = self.run_episode(key, seed, evaluation=True)
            self.game_number += 1
            if result["outcome"] is not None:
                self._record_completed(result)
            else:
                self.recent_games.append(dict(result))
            self._heartbeat(f"{NAMES[key]} fixed-opponent evaluation {index+1}/{count}")

    def run(self) -> None:
        self.preflight()
        for key in ("hog", "mortar"):
            self._save_checkpoint(key, initial=True)
        if self.args.preflight_only:
            self.state.update({"status": "preflight_complete", "phase": "preflight_complete",
                               "message": ("Fake protocol smoke preflight passed; no native behavior was checked."
                                           if self.fake_protocol else
                                           "Both native presets and play/step path passed. Training clock was not started.")})
            self._write_state()
            return
        self.run_started = time.time()
        self.run_deadline = self.run_started + self.args.duration_seconds
        self.state.update({"status": "running", "phase": "training", "duration_seconds": self.args.duration_seconds,
                           "message": ("FAKE protocol smoke; outcomes are synthetic and isolated from the live league."
                                       if self.fake_protocol else "Native deck preflight passed; winning-focused PPO is running."),
                           "backend": ("FAKE protocol emulator · smoke only; outcomes are synthetic" if self.fake_protocol
                                       else "Original engine (Unicorn) · verified native decks and card-play path")})
        self._write_state()
        log_line(self.run_dir, f"{'Fake smoke started' if self.fake_protocol else 'Training clock started'} · budget {self.args.duration_seconds:.0f}s · seed {self.args.seed}",
                 {"event": "training_started", "duration_seconds": self.args.duration_seconds, "seed": self.args.seed})
        try:
            while time.time() < self.run_deadline:
                if (self.run_dir / "STOP").exists():
                    raise StopRequested()
                key = "hog" if self.game_number % 2 == 0 else "mortar"
                seed = self.args.seed + self.game_number + 100
                result = self.run_episode(key, seed, evaluation=False)
                self.game_number += 1
                if result["outcome"] is not None:
                    self._record_completed(result)
                    self._update(key)
                    if self.args.eval_interval > 0 and self.training_games[key] % self.args.eval_interval == 0:
                        self._evaluate(key, self.args.eval_games)
                else:
                    self.recent_games.append(dict(result))
                elapsed = max(1., time.time() - self.run_started)
                eta = max(0., self.run_deadline - time.time())
                duration_label = "truncated" if result["truncated"] else f"{result['game_seconds']:.1f}s"
                log_line(self.run_dir,
                         f"Game {self.game_number} {NAMES[key]} {'fake protocol smoke' if self.fake_protocol else 'vs frozen 4.4.2'} · {result['winner']} · {duration_label} · {self.completed_games} completed/{self.truncated_games} truncated · {self.decision_steps} decisions · ETA {eta/3600:.2f}h",
                         {"event": "game", **result, "elapsed_seconds": elapsed, "eta_seconds": eta})
                self._heartbeat()
            self.state.update({"status": "completed", "phase": "completed",
                               "message": ("Fake protocol smoke completed; outcomes are synthetic and must not be reported as training results."
                                           if self.fake_protocol else "Three-hour PPO training budget completed.")})
        except StopRequested:
            self.state.update({"status": "stopped", "phase": "stopped", "message": "Stop requested; current completed work saved."})
        except Exception as exc:
            self.state.update({"status": "failed", "phase": "failed", "error": f"{type(exc).__name__}: {exc}",
                               "message": "Training stopped on an error; last valid checkpoints were saved."})
            log_line(self.run_dir, f"FAILED · {type(exc).__name__}: {exc}", {"event": "failed", "error": str(exc)})
            raise
        finally:
            for key in ("hog", "mortar"):
                self._save_checkpoint(key, final=True)
            if self.run_started:
                self.state["elapsed_seconds"] = time.time() - self.run_started
            self.state["models"] = self.state.get("models", {})
            self._write_state()
        log_line(self.run_dir, f"Run {self.state['status']} · {self.completed_games} completed · {self.truncated_games} truncated · {self.decision_steps} decisions · {self.updates['hog']+self.updates['mortar']} PPO updates")

    def elixir_spent_init(self) -> None:
        self.elixir_spent = Counter()


class StopRequested(Exception):
    pass


def utc_from_epoch(value: float) -> str:
    return dt.datetime.fromtimestamp(value, dt.timezone.utc).isoformat()


def parse_args(argv: Sequence[str] | None = None) -> argparse.Namespace:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--run-dir", type=Path, default=None)
    p.add_argument("--checkpoint", type=Path, default=None, help="Prepared native-engine checkpoint directory")
    p.add_argument("--engine-bin", type=Path, default=None, help="Verified banana-native-rl executable")
    p.add_argument("--duration-seconds", type=float, default=10800.)
    p.add_argument("--seed", type=int, default=41019)
    p.add_argument("--device", default="cuda" if torch.cuda.is_available() else "cpu")
    p.add_argument("--torch-threads", type=int, default=3)
    p.add_argument("--learning-rate", type=float, default=3e-5)
    p.add_argument("--clip-ratio", type=float, default=.2)
    p.add_argument("--epochs", type=int, default=2)
    p.add_argument("--minibatch-size", type=int, default=32)
    p.add_argument("--value-coef", type=float, default=.5)
    p.add_argument("--entropy-coef", type=float, default=.01)
    p.add_argument("--heartbeat-seconds", type=float, default=15.)
    p.add_argument("--eval-interval", type=int, default=24, help="training games per specialist between baseline evaluations; 0 disables")
    p.add_argument("--eval-games", type=int, default=4)
    p.add_argument("--preflight-only", action="store_true", help="verify native decks and play path without starting training")
    p.add_argument("--smoke", action="store_true", help="bounded fake-protocol trainer smoke; never real training")
    p.add_argument("--smoke-games", type=int, default=2)
    p.add_argument("--smoke-match-steps", type=int, default=64)
    a = p.parse_args(argv)
    if a.run_dir is None:
        a.run_dir = (ROOT / "runs" / "banana-trainer-smoke") if a.smoke else RUN_DIR
    if a.duration_seconds <= 0 or a.minibatch_size < 1 or a.epochs < 1 or a.torch_threads < 1:
        p.error("duration, minibatch size, and epochs must be positive")
    if not a.smoke and a.checkpoint is None:
        p.error("--checkpoint is required for the real native engine")
    return a


def main(argv: Sequence[str] | None = None) -> int:
    args = parse_args(argv)
    random.seed(args.seed)
    torch.manual_seed(args.seed)
    torch.set_num_threads(args.torch_threads)
    engine = FakeEngine(args.smoke_match_steps) if args.smoke else JsonlEngine(args.checkpoint, args.engine_bin)
    trainer: BananaTrainer | None = None
    try:
        trainer = BananaTrainer(args, engine)
        trainer.elixir_spent_init()
        if args.smoke:
            # Keep the smoke bounded independent of duration and exercise PPO
            # updates using long enough fake episodes to fill a 32-action batch.
            args.duration_seconds = max(30., args.duration_seconds)
            args.eval_interval = 0
            args.heartbeat_seconds = 1.
            original = trainer.run_episode
            counter = {"n": 0}
            def stop_after_bound(key: str, seed: int, evaluation: bool = False):
                result = original(key, seed, evaluation)
                counter["n"] += 1
                if counter["n"] >= args.smoke_games:
                    trainer.run_deadline = 0.0
                return result
            trainer.run_episode = stop_after_bound  # type: ignore[method-assign]
        trainer.run()
        return 0
    except Exception as exc:
        print(f"banana_train failed: {type(exc).__name__}: {exc}", file=sys.stderr, flush=True)
        if trainer is not None:
            trainer.state.update({"status": "failed", "phase": "failed", "error": f"{type(exc).__name__}: {exc}", "updated_at": utc_now()})
            trainer._write_state()
        return 1
    finally:
        engine.close()


if __name__ == "__main__":
    raise SystemExit(main())
