"""Batched, multi-process native-engine PPO runner for the Banana specialists.

Each worker owns one original-engine process/session because the recovered
engine uses fixed guest mappings and permits one live session per process.
Policy inference is batched across all worker observations at each decision
round; episode outcomes remain genuine native terminal rewards.
"""
from __future__ import annotations

import argparse
import concurrent.futures
import json
import random
import sys
import time
from collections import Counter
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Mapping

import torch

try:
    from . import banana_train as base
    from .banana_policy import BananaEpisode, SampledAction
    from .banana_fast_policy import FastBananaPolicy
    from .banana_engine import ReplayFallbackEngine
except ImportError:
    import banana_train as base
    from banana_policy import BananaEpisode, SampledAction
    from banana_fast_policy import FastBananaPolicy
    from banana_engine import ReplayFallbackEngine

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_RUN_DIR = ROOT / "runs" / "banana-fast-league-20261019"
DEFAULT_ENGINE = ROOT / "rust-engine" / "target" / "release" / "rust_deck_rl"


class TracingEngine:
    """Optional exact protocol trace, isolated per worker for source replays."""
    def __init__(self, client, path: Path):
        path.parent.mkdir(parents=True, exist_ok=True)
        self.trace = path.open("x", buffering=1)
        self.client = client

    def __getattr__(self, name):
        return getattr(self.client, name)

    def request(self, value):
        response = self.client.request(value)
        semantic = {k:v for k,v in response.items() if k not in ("reset_seconds", "step_seconds")}
        self.trace.write(json.dumps({"request": value, "expected": semantic}) + "\n")
        return response

    def close(self):
        try:
            self.client.close()
        finally:
            self.trace.close()


@dataclass
class WorkerGame:
    worker_id: int
    key: str
    seed: int
    episode: BananaEpisode
    observation: dict[str, Any]
    start_tick: int
    specialist_side: int
    actions: list[SampledAction] = field(default_factory=list)
    steps: int = 0
    accepted: int = 0
    rejected: int = 0
    rejections: Counter[str] = field(default_factory=Counter)
    started: float = field(default_factory=time.monotonic)


class FastBananaTrainer:
    """Vector-step worker pool and batched PPO loop layered on BananaTrainer."""

    def __init__(self, args: argparse.Namespace):
        self.args = args
        if args.run_dir.exists() and any(args.run_dir.iterdir()):
            raise ValueError("choose an empty run directory; existing training artifacts are never overwritten")
        # Reuse checkpoint and reward logic without changing its source or
        # leaving a different policy factory installed for other importers.
        factory = base.BananaPolicy
        try:
            base.BananaPolicy = FastBananaPolicy
            self.core = base.BananaTrainer(args, None)
        finally:
            base.BananaPolicy = factory
        self.engines = []
        try:
            for worker in range(args.workers):
                client = (ReplayFallbackEngine(args.checkpoint, args.engine_bin, args.fallback_bin) if args.fallback_bin
                          else base.JsonlEngine(args.checkpoint, args.engine_bin))
                self.engines.append(TracingEngine(client, args.trace_dir / f"worker-{worker}.jsonl") if args.trace_dir else client)
        except BaseException:
            for engine in self.engines:
                engine.close()
            raise
        self.core.engine = self.engines[0]
        self.executor = concurrent.futures.ThreadPoolExecutor(max_workers=args.workers)
        self.active: dict[int, WorkerGame] = {}
        self.seat_counts = Counter()
        self.next_game_id = 0
        self.timings = {name: 0.0 for name in ("reset", "inference", "native_step", "update")}
        self.timing_counts = Counter()
        self.started = 0.0
        self.deadline = 0.0
        self.failed = False
        self.last_eval = {"hog": 0, "mortar": 0}
        self.last_heartbeat = 0.0
        self.backend_label = f"Original recovered engine · {args.engine_bin.name} · parallel workers"
        self._write_fast_state()
        manifest = {
            "format": "banana-fast-run-v1", "created_at": base.utc_now(),
            "engine": str(args.engine_bin.resolve()), "engine_sha256": base.sha256(args.engine_bin),
            "fallback_engine": str(args.fallback_bin) if args.fallback_bin else None,
            "fallback_sha256": base.sha256(args.fallback_bin) if args.fallback_bin else None,
            "source_checkpoint": str(base.BASE_CHECKPOINT), "source_checkpoint_sha256": base.sha256(base.BASE_CHECKPOINT),
            "workers": args.workers, "seed": args.seed,
            "arguments": {name: str(value) if isinstance(value, Path) else value
                          for name, value in vars(args).items()},
            "torch_version": torch.__version__, "torch_threads": torch.get_num_threads(),
            "reward": "original native terminal +1/-1/0; truncations have no training reward",
            "collection": "synchronous complete-game waves; policy weights held fixed within each wave",
            "source_files_sha256": {str(path.relative_to(ROOT)): base.sha256(path) for path in
                [Path(__file__), ROOT / "rl/banana_fast_policy.py", ROOT / "rl/banana_engine.py", ROOT / "rl/banana_train.py", ROOT / "rl/banana_policy.py"]},
        }
        base.atomic_json(self.core.run_dir / "run-manifest.json", manifest)

    def _timed(self, name: str, started: float) -> None:
        self.timings[name] += time.monotonic() - started
        self.timing_counts[name] += 1

    def _reset_worker(self, worker_id: int, key: str, seed: int) -> WorkerGame:
        started = time.monotonic()
        engine = self.engines[worker_id]
        reset = engine.request({"op": "reset", "deck_preset": base.PRESETS[key],
                                "seed": seed, "max_ticks": base.MATCH_TICKS + base.INPUT_WARMUP_TICKS})
        self.core._verify_configured_decks(key, reset)
        obs = reset.get("observation")
        if not isinstance(obs, dict) or not isinstance(obs.get("hands"), list):
            raise RuntimeError(f"worker {worker_id} reset returned no native observation")
        warm = engine.request({"op": "step", "actions": [], "ticks": base.INPUT_WARMUP_TICKS})
        obs = warm.get("observation")
        if not isinstance(obs, dict) or int(warm.get("ticks_advanced", 0)) < base.INPUT_WARMUP_TICKS:
            raise RuntimeError(f"worker {worker_id} failed the native 100-tick input warmup")
        episode = BananaEpisode((base.DECKS[key], base.DECKS[key]), battle_id=f"{key}-{seed}")
        side = self.seat_counts[key] % 2
        self.seat_counts[key] += 1
        game = WorkerGame(worker_id, key, seed, episode, obs, int(obs["tick"]), side)
        self._timed("reset", started)
        return game

    def _start_game(self, worker_id: int) -> bool:
        if time.time() >= self.deadline or (self.core.run_dir / "STOP").exists():
            return False
        key = "hog" if self.next_game_id % 2 == 0 else "mortar"
        seed = self.args.seed + 100 + self.next_game_id
        self.next_game_id += 1
        game = self._reset_worker(worker_id, key, seed)
        self.active[worker_id] = game
        return True

    def preflight(self) -> None:
        self.core.preflight()
        # Prove each worker process can independently configure both physical
        # sides and pass a native play through the same preflight path.
        for worker_id in range(1, len(self.engines)):
            for index, key in enumerate(("hog", "mortar")):
                t0 = time.monotonic()
                reset = self.engines[worker_id].request({"op": "reset", "deck_preset": base.PRESETS[key],
                    "seed": self.args.seed + 10_000 + worker_id * 10 + index,
                    "max_ticks": base.MATCH_TICKS + base.INPUT_WARMUP_TICKS})
                self.core._verify_configured_decks(key, reset)
                obs = reset.get("observation")
                if not isinstance(obs, dict) or len(obs.get("hands", [])) != 2:
                    raise RuntimeError(f"worker {worker_id} {key} reset failed full-hand validation")
                warm = self.engines[worker_id].request({"op": "step", "actions": [], "ticks": base.INPUT_WARMUP_TICKS})
                if int(warm.get("ticks_advanced", 0)) < base.INPUT_WARMUP_TICKS:
                    raise RuntimeError(f"worker {worker_id} failed 100-tick native warmup")
                self._timed("reset", t0)
                base.log_line(self.core.run_dir,
                    f"Worker {worker_id} preflight {key} passed: exact configured native decks and 100-tick warmup.",
                    {"event": "worker_preflight", "worker": worker_id, "deck": key})
        self.core.state["workers"] = self.args.workers
        self.core.state["engine_binary"] = str(self.args.engine_bin)
        self._write_fast_state()

    def _batch_sample(self, games: list[WorkerGame]) -> dict[int, tuple[SampledAction, SampledAction]]:
        specialist_requests: dict[str, list[tuple[WorkerGame, tuple[Any, ...]]]] = {"hog": [], "mortar": []}
        baseline_requests: list[tuple[WorkerGame, tuple[Any, ...]]] = []
        for game in games:
            seconds = max(0.0, (int(game.observation["tick"]) - game.start_tick) / base.TICKS_PER_SECOND)
            specialist_requests[game.key].append((game, (game.episode, game.observation, game.specialist_side, seconds)))
            opponent_side = 1 - game.specialist_side
            baseline_requests.append((game, (game.episode, game.observation, opponent_side, seconds)))
        started = time.monotonic()
        by_worker: dict[int, tuple[SampledAction, SampledAction]] = {}
        for key, rows in specialist_requests.items():
            if not rows:
                continue
            sampled = self.core.policies[key].act_batch([row for _, row in rows], deterministic=False)
            for (game, _), action in zip(rows, sampled):
                by_worker.setdefault(game.worker_id, (None, None))
                pair = list(by_worker[game.worker_id])
                pair[game.specialist_side] = action
                by_worker[game.worker_id] = tuple(pair)
        baseline = self.core.baseline.act_batch([row for _, row in baseline_requests], deterministic=True)
        for (game, _), action in zip(baseline_requests, baseline):
            pair = list(by_worker[game.worker_id])
            pair[1 - game.specialist_side] = action
            by_worker[game.worker_id] = tuple(pair)
        self._timed("inference", started)
        return by_worker

    def _step_one(self, game: WorkerGame, actions: tuple[SampledAction, SampledAction]) -> tuple[WorkerGame, dict[str, Any], list[dict[str, Any]]]:
        native_actions = [action.native_request() for action in actions if not action.wait]
        result = self.engines[game.worker_id].request({"op": "step", "actions": native_actions, "ticks": 20})
        rows = result.get("actions")
        if not isinstance(rows, list) or len(rows) != len(native_actions):
            raise RuntimeError(f"worker {game.worker_id} native engine returned malformed action results")
        return game, result, rows

    def _finish_game(self, game: WorkerGame, result: Mapping[str, Any]) -> dict[str, Any]:
        key = game.key
        obs = result["observation"]
        terminal = result.get("terminated") is True
        truncated = result.get("truncated") is True or not terminal
        game_seconds = max(0.0, (int(obs["tick"]) - game.start_tick) / base.TICKS_PER_SECOND)
        outcome = None
        winner = "Truncated"
        if terminal:
            winner_side = obs.get("winner_side")
            if winner_side not in (0, 1, -1):
                raise RuntimeError(f"native terminal result has unresolved winner {winner_side!r}")
            outcome = 0.0 if winner_side == -1 else (1.0 if winner_side == game.specialist_side else -1.0)
            for action in game.actions:
                self.core.buffers[key].append(base.Transition(action, outcome, float(action.value.detach().cpu())))
            winner = "Draw" if winner_side == -1 else (base.NAMES[key] if outcome > 0 else "frozen-4.4.2")
            self.core.training_games[key] += 1
            self.core.training_results[key].append("draw" if outcome == 0 else ("win" if outcome > 0 else "loss"))
            if outcome > 0:
                self.core.streaks[key] += 1
                if self.core.streaks[key] > self.core.longest_streak[0]:
                    self.core.longest_streak = (self.core.streaks[key], base.NAMES[key])
            else:
                self.core.streaks[key] = 0
            self.core.completed_games += 1
        else:
            self.core.truncated_games += 1
        summary = {"game": self.core.completed_games + self.core.truncated_games, "model": base.NAMES[key],
                   "winner": winner, "game_seconds": game_seconds,
                   "kind": "training vs frozen 4.4.2 · original native engine",
                   "truncated": bool(truncated), "decision_steps": game.steps * 2,
                   "accepted_plays": game.accepted, "rejected_plays": game.rejected,
                   "rejections": dict(game.rejections), "specialist_side": game.specialist_side,
                   "outcome": outcome, "worker": game.worker_id,
                   "elapsed_wall_seconds": time.monotonic() - game.started}
        if outcome is not None:
            self.core._record_completed(summary)
        else:
            self.core.recent_games.append(summary)
        return summary

    def _write_fast_state(self) -> None:
        self.core.state["workers"] = self.args.workers
        self.core.state["backend"] = self.backend_label
        self.core.state["throughput"] = {
            name: {"seconds": round(value, 3), "count": self.timing_counts[name],
                   "mean_seconds": round(value / max(1, self.timing_counts[name]), 4)}
            for name, value in self.timings.items()
        }
        self.core.state["active_workers"] = len(self.active)
        self.core.state["reference_recoveries"] = sum(getattr(engine, "fallback_count", 0) for engine in self.engines)
        self.core.state["reference_recovery_events"] = [event for engine in self.engines for event in getattr(engine, "fallback_events", [])]
        self.core._write_state()

    def run(self) -> None:
        self.preflight()
        for key in ("hog", "mortar"):
            self.core._save_checkpoint(key, initial=True)
        if self.args.preflight_only:
            self.core.state.update({"status": "preflight_complete", "phase": "preflight_complete",
                                    "message": "All parallel workers passed exact-deck setup and native warmup checks."})
            self._write_fast_state()
            return
        self.started = time.time()
        self.deadline = self.started + self.args.duration_seconds
        self.timings = {name: 0.0 for name in self.timings}
        self.timing_counts.clear()
        self.core.run_started = self.started
        self.core.run_deadline = self.deadline
        self.core.state.update({"status": "running", "phase": "training", "duration_seconds": self.args.duration_seconds,
                                "workers": self.args.workers,
                                "message": f"Batched PPO training on original native engine with {self.args.workers} workers.",
                                "backend": self.backend_label})
        self._write_fast_state()
        base.log_line(self.core.run_dir,
                      f"Fast training clock started · {self.args.duration_seconds:.0f}s · {self.args.workers} workers · seed {self.args.seed}",
                      {"event": "training_started", "duration_seconds": self.args.duration_seconds,
                       "workers": self.args.workers, "seed": self.args.seed})
        summaries: list[dict[str, Any]] = []
        try:
            while time.time() < self.deadline:
                if (self.core.run_dir / "STOP").exists():
                    break
                # Fill a new wave only after all previous games finish and
                # their PPO updates complete. This avoids stale-policy games
                # continuing while their specialist weights are changed.
                if not self.active:
                    for wid in range(self.args.workers):
                        if time.time() < self.deadline:
                            self._start_game(wid)
                if not self.active:
                    break
                games = list(self.active.values())
                actions_by_worker = self._batch_sample(games)
                t0 = time.monotonic()
                futures = [self.executor.submit(self._step_one, game, actions_by_worker[game.worker_id]) for game in games]
                stepped = [future.result() for future in futures]
                self._timed("native_step", t0)
                for game, response, result_rows in stepped:
                    actions = actions_by_worker[game.worker_id]
                    side_result = 0
                    game_seconds = max(0.0, (int(game.observation["tick"]) - game.start_tick) / base.TICKS_PER_SECOND)
                    for side, action in enumerate(actions):
                        if side == game.specialist_side:
                            game.actions.append(action)
                        if action.wait:
                            continue
                        row = result_rows[side_result]; side_result += 1
                        if not isinstance(row, Mapping) or not isinstance(row.get("accepted"), bool):
                            raise RuntimeError("native action result omitted boolean accepted")
                        agent = self.core.policies[game.key] if side == game.specialist_side else self.core.baseline
                        if row["accepted"]:
                            agent.record_action(game.episode, action, game_seconds, True)
                            game.accepted += 1
                            self.core.accepted_plays += 1
                            if side == game.specialist_side:
                                model_state = self.core.state["models"][game.key]
                                model_state["accepted_plays"] = int(model_state.get("accepted_plays", 0)) + 1
                            cost = int(row.get("cost", self.core.policies[game.key].costs.get(action.card, 0)))
                            self.core.deck_plays[action.card] += 1
                            self.core.elixir_spent[action.card] += cost
                        else:
                            reason = str(row.get("rejection") or "native_rejection_unspecified")
                            game.rejections[reason] += 1
                            self.core.native_rejections[reason] += 1
                            self.core.rejected_plays += 1
                            if side == game.specialist_side:
                                game.rejected += 1
                                self.core.state["models"][game.key]["rejected_plays"] += 1
                    game.steps += 1
                    self.core.decision_steps += 2
                    game.observation = response.get("observation")
                    if not isinstance(game.observation, dict):
                        raise RuntimeError("native engine returned no observation after vector step")
                    done = response.get("terminated") is True or response.get("truncated") is True
                    if not done and int(game.observation["tick"]) >= game.start_tick + base.MATCH_TICKS:
                        raise RuntimeError("native engine reached max ticks without terminal/truncated status")
                    if done:
                        summary = self._finish_game(game, response)
                        summaries.append(summary)
                        self.active.pop(game.worker_id)
                        base.log_line(self.core.run_dir,
                            f"Game {summary['game']} {summary['model']} {summary['kind']} · {summary['winner']} · {summary['game_seconds']:.1f}s · worker {game.worker_id}",
                            {"event": "game", **summary})
                if not self.active:
                    for key in ("hog", "mortar"):
                        u0 = time.monotonic()
                        self.core._update(key)
                        self._timed("update", u0)
                        if (self.args.eval_interval > 0 and time.time() < self.deadline
                                and self.core.training_games[key] - self.last_eval[key] >= self.args.eval_interval):
                            self.core._evaluate(key, self.args.eval_games)
                            self.last_eval[key] = self.core.training_games[key]
                if time.monotonic() - self.last_heartbeat >= self.args.heartbeat_seconds:
                    self.last_heartbeat = time.monotonic()
                    self.core._heartbeat(f"Batched PPO · {self.core.completed_games} completed games · {len(self.active)}/{self.args.workers} active workers")
                    self._write_fast_state()
            for game in list(self.active.values()):
                # Deadline/stop is not an engine win or loss. Retain no actor
                # reward for an unfinished native game.
                summary = self._finish_game(game, {"observation": game.observation, "truncated": True})
                base.log_line(self.core.run_dir, f"Worker {game.worker_id} truncated at stop/deadline", {"event": "game", **summary})
            self.active.clear()
            for key in ("hog", "mortar"):
                if self.core.buffers[key]:
                    t0 = time.monotonic(); self.core._update(key, force=True); self._timed("update", t0)
            self.core.state.update({"status": "completed" if time.time() >= self.deadline else "stopped",
                                    "phase": "completed" if time.time() >= self.deadline else "stopped",
                                    "message": "Parallel training duration completed." if time.time() >= self.deadline else "Parallel training stop requested; completed work checkpointed."})
        except Exception as exc:
            self.core.state.update({"status": "failed", "phase": "failed", "error": f"{type(exc).__name__}: {exc}",
                                    "message": "Fast training stopped on an error; last valid checkpoints were retained."})
            base.log_line(self.core.run_dir, f"FAILED · {type(exc).__name__}: {exc}", {"event": "failed", "error": str(exc)})
            raise
        finally:
            for key in ("hog", "mortar"):
                self.core._save_checkpoint(key, final=True)
            self._write_fast_state()

    def close(self) -> None:
        self.executor.shutdown(wait=True, cancel_futures=True)
        for engine in self.engines:
            engine.close()


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", type=Path, default=DEFAULT_RUN_DIR)
    parser.add_argument("--checkpoint", type=Path, required=True)
    parser.add_argument("--engine-bin", type=Path, default=DEFAULT_ENGINE)
    parser.add_argument("--trace-dir", type=Path, help="optional per-worker exact protocol traces for validation")
    parser.add_argument("--fallback-bin", type=Path, help="optional original reference executable for checked episode recovery on a coverage gap")
    parser.add_argument("--workers", type=int, default=3)
    parser.add_argument("--duration-seconds", type=float, default=10800.)
    parser.add_argument("--seed", type=int, default=41019)
    parser.add_argument("--device", default="cuda" if torch.cuda.is_available() else "cpu")
    parser.add_argument("--torch-threads", type=int, default=3)
    parser.add_argument("--learning-rate", type=float, default=3e-5)
    parser.add_argument("--clip-ratio", type=float, default=.2)
    parser.add_argument("--epochs", type=int, default=2)
    parser.add_argument("--minibatch-size", type=int, default=32)
    parser.add_argument("--value-coef", type=float, default=.5)
    parser.add_argument("--entropy-coef", type=float, default=.01)
    parser.add_argument("--heartbeat-seconds", type=float, default=15.)
    parser.add_argument("--eval-interval", type=int, default=24)
    parser.add_argument("--eval-games", type=int, default=4)
    parser.add_argument("--preflight-only", action="store_true")
    args = parser.parse_args(argv)
    if not 1 <= args.workers <= 4:
        parser.error("--workers must be between 1 and 4")
    if args.duration_seconds <= 0 or args.minibatch_size < 1 or args.epochs < 1 or args.torch_threads < 1:
        parser.error("duration, minibatch size, epochs, and torch threads must be positive")
    if not args.checkpoint.is_dir():
        parser.error("--checkpoint must be an existing native-engine checkpoint directory")
    if not args.engine_bin.is_file():
        parser.error("--engine-bin must be an existing executable")
    if args.fallback_bin and not args.fallback_bin.is_file():
        parser.error("--fallback-bin must be an existing executable")
    return args


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)
    random.seed(args.seed); torch.manual_seed(args.seed); torch.set_num_threads(args.torch_threads)
    runner = None
    try:
        runner = FastBananaTrainer(args)
        runner.core.elixir_spent_init()
        runner.run()
        return 0
    except Exception as exc:
        print(f"banana_fast_train failed: {type(exc).__name__}: {exc}", file=sys.stderr, flush=True)
        return 1
    finally:
        if runner:
            runner.close()


if __name__ == "__main__":
    raise SystemExit(main())
