"""Board-aware Hog 2.6 league for a fresh three-hour run.

One process is the production entry point: ``python -m rl.crazy_hog_train``.
The learner is ``BoardHogPolicy``, warm-started from banana-hog-2. Frozen
bc-greedy, bc-sampled, and hog-2 parent opponents stay on the board-free
policies. Self-play snapshots are frozen copies of the learner, so they read
the board, and they play only the Hog 2.6 mirror.

Scheduling batches every game that is waiting for a decision into one learner
forward and one forward per frozen policy. State-bank episodes restore an
in-process engine snapshot on the same worker, replay a short horizon, and
bootstrap the critic at truncation. They do not enter win-rate stats.
"""
from __future__ import annotations

import argparse
import copy
import datetime as dt
import hashlib
import json
import os
import random
import signal
import time
import uuid
from collections import Counter
from pathlib import Path

import torch

from . import banana_hog_league as core
from .async_collection import ContinuousCollector
from .banana_fast_policy import fused_act_round
from .banana_hog_adaptive import AdaptiveHogPolicy, AdaptiveLeague
from .banana_policy import BananaEpisode
from .banana_train import ROOT, atomic_json, sha256, utc_now
from .board_policy import BoardHogPolicy
from .board_rewards import elixir_trade_potential
from .crazy_hog_core import (
    PINNED_ENGINE_PREFIX,
    assert_engine_pin,
    audit_game_rewards,
    budget_percent,
    capture_match_python,
    critical_moments,
    eta_seconds,
    fresh_hog_stats,
    gae_with_bootstrap,
    hog_summary,
    note_critical,
    restore_match_python,
    update_hog_stats,
    validate_crazy_resume,
    PFSPSampler,
)
DEFAULT_PARENT = Path(
    "/home/cochon/Documents/mini-project/clash-royale-ai/engine/runs/banana-hog-2/models/final.pt"
)
DEFAULT_ENGINE = ROOT / "rust-engine/target/pgo/rust_league_rl"
HOG_MIRROR = "hog-mirror"


class TrainBoardPolicy(BoardHogPolicy):
    """Board policy whose ``state_dict`` still answers the parent's critic keys.

    ``League`` checks warm-start parity by looking up every parent key in the
    learner state dict. The privileged critic stores the loaded value head
    under ``critic.base``. Aliasing those tensors back to ``critic.*`` keeps
    the check exact without putting the same parameter in two optimizer groups.
    """

    def state_dict(self, *args, **kwargs):
        data = super().state_dict(*args, **kwargs)
        for key, value in list(data.items()):
            marker = "critic.base."
            if marker in key:
                data[key.replace(marker, "critic.", 1)] = value
        return data


class _SlotPool:
    def __init__(self, league: "CrazyHogLeague"):
        self.league = league

    def call(self, slot: int, request: dict, timeout: int = 90):
        return self.league.native_request(slot, request, timeout)


def load_expert(row_table):
    """Prefer W5's ``hog_search`` module. Fall back to the C7 stub."""
    try:
        from . import hog_search as module
    except ImportError:
        from . import crazy_hog_search_stub as module
    return module.ExpertIteration(module.SearchConfig(), row_table), module.__name__


class ReadyBatchCollector(ContinuousCollector):
    """W3's continuous collector, with every waiting game in one forward."""

    def _launch_one_group(self):
        if not self.ready:
            return
        batch = list(self.ready)
        self.ready = []
        self.league.counters["learner_batches"] += 1
        self.league.counters["learner_batch_rows"] += len(batch)
        self.league._launch_steps(batch, self.league.learner, True, self.step_jobs)

    def _start_idle(self):
        league = self.league
        for worker in self._idle_workers():
            if league.stopping():
                return
            job = league.draw_job(worker)
            name = league.decks[job["deck_id"]]["name"]
            league.worker_states[worker] = {
                "id": worker, "status": "resetting", "deck": name,
                "controller": job["controller"], "seat": job["seat"],
                "bank": bool(job.get("bank")),
            }
            future = league.executor.submit(league.reset, worker, job)
            self.reset_jobs[future] = (worker, job)


class CrazyHogLeague(AdaptiveLeague):
    def __init__(self, args):
        # Parent construction publishes state and saves initial checkpoints via
        # our overrides before returning.
        self.worker_banks = [[] for _ in range(args.workers)]
        self.bank_index = {}
        self.snapshot_by_name = {}
        self.snapshot_order = []
        self.expert = None
        self.expert_module = None
        args.async_collection = not getattr(args, "sync_waves", False)
        self._install_snapshot_cap(args)
        try:
            super().__init__(args)
        except BaseException:
            self._remove_snapshot_cap()
            closer = getattr(self, "close_engines", None)
            if closer is not None:
                try:
                    closer()
                except Exception:
                    pass
            raise
        self._remove_snapshot_cap()
        try:
            assert_engine_pin(self.engine["sha256"], allow=bool(getattr(args, "allow_unpinned_engine", False)))
        except BaseException:
            self.close_engines()
            raise
        if args.search == "expert":
            self.expert, self.expert_module = load_expert(self.learner.row_table)
        self.slot_pool = _SlotPool(self)
        self._write_notes(resumed=bool(args.resume))
        self.publish(force=True)

    def _install_snapshot_cap(self, args):
        original = core.NativeClient.__init__
        cap = int(args.max_snapshots)

        def patched(client, binary, checkpoint, error_path, command=None):
            if command is None:
                command = [str(binary), "--checkpoint", str(checkpoint), "--max-snapshots", str(cap)]
            original(client, binary, checkpoint, error_path, command)

        self._native_init = original
        core.NativeClient.__init__ = patched

    def _remove_snapshot_cap(self):
        if getattr(self, "_native_init", None) is not None:
            core.NativeClient.__init__ = self._native_init
            self._native_init = None

    @property
    def evaluation_controllers(self):
        return ("bc-greedy", "bc-sampled", "hog2-greedy")

    def make_sampler(self, decks, seed):
        for deck in decks:
            deck["holdout"] = False
        return PFSPSampler(
            decks, self.evaluation_controllers, seed,
            power=getattr(self.args, "pfsp_power", 2.0),
            floor=getattr(self.args, "pfsp_floor", 0.05),
            mirror_id=HOG_MIRROR,
        )

    def make_learner(self, **kwargs):
        kwargs["trainable"] = getattr(self.args, "trainable", "all")
        return TrainBoardPolicy(**kwargs)

    def make_parent_policy(self, **kwargs):
        # Hog-2 parent. Board-free: it never receives observe_board.
        return AdaptiveHogPolicy(trainable="heads", **kwargs)

    def make_optimizer(self):
        optimizer = super().make_optimizer()
        owned = {id(parameter) for group in optimizer.param_groups for parameter in group["params"]}
        board = [parameter for parameter in self.learner.board_parameters()
                 if parameter.requires_grad and id(parameter) not in owned]
        if not board:
            raise RuntimeError("board parameter group is empty or overlaps another group")
        optimizer.add_param_group({
            "params": board, "lr": self.args.board_learning_rate, "role": "board",
        })
        return optimizer

    def _source_hashes(self):
        return {
            "crazy_train": sha256(Path(__file__)),
            "crazy_core": sha256(Path(__file__).with_name("crazy_hog_core.py")),
            "board_policy": sha256(ROOT / "rl/board_policy.py"),
            "board_rewards": sha256(ROOT / "rl/board_rewards.py"),
        }

    def manifest_extra(self):
        metadata = super().manifest_extra()
        metadata.update(self._source_hashes_as_manifest())
        metadata.update(
            format="crazy-hog-v1",
            observation_contract="Causal accepted-play history, both decks and elixirs, own hand, native tower HP and current board entities. No future state or opponent hand.",
            pinned_engine_prefix=PINNED_ENGINE_PREFIX,
            controllers=list(self.evaluation_controllers),
            reward=("Native terminal ±1/0, plus tower-HP potential "
                    f"(coef {self.args.shaping_coefficient}) and elixir-trade potential "
                    f"(coef {self.args.trade_coef}). Both telescope. Terminal Phi is 0. "
                    "Bank truncations keep the last Phi and bootstrap the critic."),
        )
        metadata["experiment"]["description"] = (
            "Board-aware Hog 2.6 league: PFSP, self-play snapshots, state-bank curriculum"
        )
        metadata["experiment"]["training_changes"] = [
            "Learner is BoardHogPolicy warm-started from banana-hog-2 with zero board branches",
            "Optimizer groups: board, action heads, body, critic",
            "Frozen bc-greedy, bc-sampled, and hog-2 parent stay board-free",
            "Self-play snapshots of the learner play only the Hog mirror",
            "PFSP weight is floor + (1 - score)^power",
            "State-bank episodes restore an in-process snapshot and bootstrap at truncation",
            "Parent-KL rollback and best-relative regression guard are unchanged",
            "One learner forward per scheduling round across every game waiting for a decision",
        ]
        return metadata

    def _source_hashes_as_manifest(self):
        return {f"{name}_sha256": digest for name, digest in self._source_hashes().items()}

    def extra_state(self):
        result = super().extra_state()
        now = time.time()
        elapsed = max(0.0, now - self.started)
        episodes = int(self.counters.get("games", 0)) + int(self.counters.get("bank_episodes", 0))
        diag_games = int(self.counters.get("diag_games", 0))
        hog_plays = int(self.counters.get("hog_plays", 0))
        batches = int(self.counters.get("learner_batches", 0))
        rows = int(self.counters.get("learner_batch_rows", 0))
        result.update(
            duration_seconds=float(self.deadline - self.started),
            elapsed_seconds=elapsed,
            progress={
                "games": int(self.counters.get("games", 0)),
                "bank_episodes": int(self.counters.get("bank_episodes", 0)),
                "episodes": episodes,
                "decisions": int(self.counters.get("decisions", 0)),
                "updates": int(self.counters.get("updates", 0)),
                "budget_percent": budget_percent(now, self.started, self.deadline),
                "eta_seconds": eta_seconds(now, self.started, self.deadline, self.status),
            },
            hog={
                "hog_plays": hog_plays,
                "hog_plays_per_game": (hog_plays / diag_games) if diag_games else None,
                "plays_per_game": (int(self.counters.get("plays", 0)) / diag_games) if diag_games else None,
                "connect_rate": (int(self.counters.get("hog_connects", 0)) / hog_plays) if hog_plays else None,
                "elixir_leak_seconds": (float(self.counters.get("leak_seconds", 0.0)) / diag_games) if diag_games else None,
                "card_plays": dict(self.fun.get("card_plays") or {}),
            },
            batch={
                "learner_batches": batches,
                "mean_learner_batch": (rows / batches) if batches else None,
            },
            rates_extra={
                "episodes_per_minute": episodes * 60.0 / elapsed if elapsed else 0.0,
                "decisions_per_second": int(self.counters.get("decisions", 0)) / elapsed if elapsed else 0.0,
            },
            sampler_state=self.sampler.public_state() if hasattr(self.sampler, "public_state") else {},
            snapshots=list(self.snapshot_order),
            search=self.args.search,
            expert_module=self.expert_module,
        )
        result["notes"] = [
            "Win rate excludes bank episodes and VOID. Bank episodes still train, with critic bootstrap when truncated.",
            "Frozen bc and hog-2 parent policies do not read the board. Self-play snapshots do, because they are the learner.",
            "Scores are against frozen bots plus our own snapshots, not against live human decks.",
            "PEKKA Bridge Spam is one cell of the PFSP distribution; a low score there is a measured result.",
            f"Absolute deadline: {self.args.deadline}.",
        ]
        return result

    def restore(self, path):
        manifest_path = self.run_dir / "manifest.json"
        if manifest_path.exists():
            saved = json.loads(manifest_path.read_text())
            validate_crazy_resume(saved, self.args, self._source_hashes())
        super().restore(path)
        payload = torch.load(path, map_location=self.args.device, weights_only=False)
        self._load_board(payload)
        self.worker_banks = [[] for _ in range(self.args.workers)]
        self.bank_index = {}
        self.log("In-process state banks do not survive a restart; curriculum snapshots start empty.")

    def save(self, stage):
        super().save(stage)
        path = self.run_dir / "models" / f"{stage}.pt"
        payload = torch.load(path, map_location="cpu", weights_only=False)
        payload["format"] = "crazy-hog-ppo-v1"
        board = self.learner.checkpoint_payload()
        payload["board_state"] = board["board_state"]
        payload["rows_source"] = board.get("rows_source")
        temporary = path.with_name(f".{path.name}.tmp")
        torch.save(payload, temporary)
        with temporary.open("rb") as stream:
            os.fsync(stream.fileno())
        os.replace(temporary, path)
        atomic_json(path.with_suffix(".sha256.json"), {"sha256": sha256(path), "at": utc_now(), "updates": self.version})

    def load_weights(self, payload):
        super().load_weights(payload)
        self._load_board(payload)

    def _load_board(self, payload):
        board = payload.get("board_state")
        if not board:
            raise RuntimeError("crazy-hog checkpoint is missing board_state")
        vocab_names = board.get("vocab", {}).get("names")
        if vocab_names is not None and list(vocab_names) != list(self.learner.row_vocab.names):
            raise RuntimeError("board checkpoint vocabulary does not match this run")
        self.learner.board_encoder.load_state_dict(board["encoder"])
        self.learner.context_residual.load_state_dict(board["context_residual"])
        self.learner.raster.load_state_dict(board["raster"])
        self.learner.critic.proj.load_state_dict(board["critic_proj"])
        self.learner.critic.set_embed(None)

    def _policy_for_controller(self, controller: str):
        if controller in {"hog1-greedy", "hog2-greedy"}:
            return self.parent_policy
        if controller.startswith("selfplay-"):
            policy = self.snapshot_by_name.get(controller)
            if policy is None:
                raise RuntimeError(f"missing self-play snapshot {controller}")
            return policy
        if controller in {"bc-greedy", "bc-sampled"}:
            return self.base
        raise RuntimeError(f"unknown controller {controller}")

    def opponent_actions(self, policy, requests, games, controller):
        if hasattr(policy, "observe_board"):
            deterministic = controller != "bc-sampled" if isinstance(controller, str) else [
                name != "bc-sampled" for name in controller]
            kwargs = {"placement_top_k": 576}
            if getattr(self.args, "native_placement_masks", False):
                kwargs["placement_mask_provider"] = self.mask_provider(games)
            return policy.act_batch(requests, deterministic=deterministic, **kwargs)
        return super().opponent_actions(policy, requests, games, controller)

    def draw_job(self, worker: int) -> dict:
        bank = self.worker_banks[worker]
        use_bank = bool(bank) and self.rng.random() < float(self.args.state_bank_fraction)
        if use_bank:
            item = bank[self.rng.randrange(len(bank))]
            job = dict(item["job"])
            job.update(bank=True, snapshot_id=item["snapshot_id"], bank_token=item["token"], collect_bank=False)
            self.counters["bank_starts"] += 1
        else:
            job = self.sampler.draw()
            job["bank"] = False
            job["collect_bank"] = True
        self._log_sampler(job)
        return job

    def _log_sampler(self, job: dict) -> None:
        if not hasattr(self.sampler, "public_state"):
            return
        row = {"at": utc_now(), "job": {key: job.get(key) for key in ("deck_id", "controller", "seat", "seed", "bank")},
               "sampler": self.sampler.public_state()}
        with (self.run_dir / "sampler.jsonl").open("a") as stream:
            stream.write(json.dumps(row) + "\n")

    def collect_training(self):
        if not getattr(self.args, "async_collection", False):
            jobs = [self.draw_job(worker) for worker in range(self.args.workers)]
            return self.play(jobs, self.learner, "train", collecting=True)
        if getattr(self, "_collector", None) is None:
            self._collector = ReadyBatchCollector(self)
        target = getattr(self.args, "async_ready_games", 0) or self.args.workers
        self.phase = f"training · batched async v{self.version} · {target} games"
        return self._collector.collect(target)

    def reset(self, worker, job):
        if job.get("bank"):
            return self._reset_bank(worker, job)
        return self._reset_fresh(worker, job)

    def _reset_fresh(self, worker, job):
        deck = self.decks[job["deck_id"]]
        native = [self.hog["native_cards"], deck["native_cards"]]
        slugs = [tuple(self.hog["cards"]), tuple(deck["cards"])]
        if job["seat"] == 1:
            native.reverse()
            slugs.reverse()
        opened = self.native_request(worker, {
            "op": "reset", "deck0": native[0], "deck1": native[1], "seed": job["seed"],
            "max_ticks": self.args.match_ticks + 100, "reward_state": True, "board": True,
        })
        if "board" not in opened:
            raise RuntimeError("reset reply missing board")
        self.verify_decks(opened, slugs)
        warm = self.native_request(worker, {"op": "step", "actions": [], "ticks": 100})
        if warm.get("ticks_advanced") != 100:
            raise RuntimeError("native_warmup_failed")
        if "board" not in warm or "reward_state" not in warm:
            raise RuntimeError("warmup reply missing board or reward_state")
        self._stamp_observation(warm)
        game = self._new_game(worker, job, tuple(slugs), warm)
        if job.get("collect_bank"):
            self._maybe_snapshot(game, warm)
        return game

    def _reset_bank(self, worker, job):
        item = self.bank_index.get(job.get("bank_token"))
        if item is None or item["worker"] != worker:
            raise RuntimeError("bank snapshot is not on this worker")
        reply = self.native_request(worker, {"op": "restore", "snapshot_id": job["snapshot_id"]})
        if "board" not in reply or "reward_state" not in reply:
            raise RuntimeError("restore reply missing board or reward_state")
        self._stamp_observation(reply)
        episode = BananaEpisode(item["python"]["decks"], battle_id=item["python"]["battle_id"])
        view = restore_match_python(episode, self._cache_policies(), item["python"])
        game = core.Game(worker, job, episode, view["obs"], int(view["start_tick"]))
        game.obs = reply["observation"]
        game.start_tick = int(view["start_tick"])
        game.potentials = [view["tower_potential"]]
        game.trade_potentials = [view["trade_potential"]]
        game.behavior_version = self.version
        game.horizon_tick = int(game.obs["tick"]) + int(round(self.args.bank_horizon_seconds * 20))
        game.bank_truncated = False
        game.bootstrap_value = 0.0
        self._prepare_game(game)
        self.learner.observe_board(episode, game.obs["board"], job["seat"])
        game.episode.policy_seed = job["seed"]
        return game

    def _new_game(self, worker, job, slugs, reply):
        from .banana_hog_retrain import tower_potential
        potential = tower_potential(reply["reward_state"], job["seat"])
        elixir = reply["observation"]["elixir"]
        trade = elixir_trade_potential(
            reply["board"], job["seat"], elixir[job["seat"]], elixir[1 - job["seat"]], self.learner.row_table,
        )
        game = core.Game(
            worker, job, BananaEpisode(tuple(slugs), battle_id=uuid.uuid4().hex),
            reply["observation"], int(reply["observation"]["tick"]),
        )
        game.potentials = [potential]
        game.trade_potentials = [trade.potential]
        game.behavior_version = self.version
        game.episode.policy_seed = job["seed"]
        game.bank_truncated = False
        game.bootstrap_value = 0.0
        self._prepare_game(game)
        self.learner.observe_board(game.episode, reply["board"], job["seat"])
        return game

    def _prepare_game(self, game):
        game.hog_stats = fresh_hog_stats()
        game.snap_seen = {"seen_cross": set(), "seen_towers": set(), "saw_double": False, "saw_hog_hand": False}
        game.event_cursor = len(game.episode.events)
        game.audited = False

    def _stamp_observation(self, reply):
        reply["observation"]["board"] = reply["board"]
        reply["observation"]["tower_state"] = reply["reward_state"]

    def _cache_policies(self):
        # Bank cache indices must remain stable as self-play opponents rotate.
        # A snapshot opponent can rebuild its causal encoder from event history.
        policies = [self.learner, self.parent_policy, self.base]
        return [policy for policy in policies if policy is not None]

    def _launch_steps(self, games, policy, collecting, step_jobs):
        if not games:
            return
        if collecting and not self.args.async_collection:
            self.counters["learner_batches"] += 1
            self.counters["learner_batch_rows"] += len(games)
        for game in games:
            board = game.obs.get("board")
            if not isinstance(board, dict) or "entities" not in board:
                raise RuntimeError(f"worker {game.worker} has no board on the observation")
            self.learner.observe_board(game.episode, board, game.job["seat"])
        requests = [(game.episode, game.obs, game.job["seat"], (int(game.obs["tick"]) - game.start_tick) / 20.0)
                    for game in games]
        started = time.perf_counter()
        grouped = {}
        for game in games:
            opponent = self._policy_for_controller(game.job["controller"])
            grouped.setdefault(id(opponent), (opponent, []))[1].append(game)
        with fused_act_round():
            actions = self.learner_actions(policy, requests, games, collecting)
            learner_pairs = list(zip(games, actions))
            opponent_pairs = []
            for opponent, subset in grouped.values():
                for game in subset:
                    if hasattr(opponent, "observe_board"):
                        opponent.observe_board(game.episode, game.obs["board"], 1 - game.job["seat"])
                opponent_requests = [(game.episode, game.obs, 1 - game.job["seat"],
                                      (int(game.obs["tick"]) - game.start_tick) / 20.0) for game in subset]
                controllers = [game.job["controller"] for game in subset]
                controller = controllers[0] if len(set(controllers)) == 1 else controllers
                opponent_pairs.append((subset, self.opponent_actions(opponent, opponent_requests, subset, controller)))
        sampled = {game.worker: [] for game in games}
        version = self.version
        concrete_learner = []
        for game, action in learner_pairs:
            action = self._concrete(action)
            concrete_learner.append((game, action))
            if collecting:
                action.behavior_version = version
                game.decisions.append(action)
            sampled[game.worker].append((policy, action))
        for subset, opponent_actions in opponent_pairs:
            for game, action in zip(subset, opponent_actions):
                sampled[game.worker].append((self._policy_for_controller(game.job["controller"]), self._concrete(action)))
        if collecting and self.expert is not None:
            self._maybe_search(concrete_learner)
        pace = self._pace()
        pace["infer_s"] += time.perf_counter() - started
        pace["infer_batches"] += 1
        pace["infer_games"] += len(games)
        self.counters["opponent_forwards"] += len(opponent_pairs)
        payloads, ordered = [], []
        for game in games:
            rows = sampled[game.worker]
            rows.sort(key=lambda pair: pair[1].side)
            game.pending_rows = rows
            game.step_sent = time.perf_counter()
            native = [action.native_request() for _, action in rows if not action.wait]
            payloads.append((game.worker, {"op": "step", "actions": native, "ticks": 20}))
            ordered.append(game)
        for game, future in zip(ordered, self._submit_steps(payloads, timeout=45)):
            step_jobs[future] = game

    def _maybe_search(self, pairs):
        for game, action in pairs:
            rng = self._search_rng(game)
            events_before = len(game.episode.events)
            cache_before = len(self.learner._board_cache)
            if not self.expert.should_search(game.obs, game.obs["board"], game.job["seat"], rng):
                continue
            target = self.expert.search(
                self.slot_pool, game.worker, self.learner, game.episode, game.job["seat"],
                game.obs, game.obs["board"], None,
            )
            if len(game.episode.events) != events_before or len(self.learner._board_cache) != cache_before:
                raise RuntimeError("search mutated the episode or the learner board cache")
            if target is None:
                continue
            if getattr(target, "cache", None) is None:
                target.cache = action.input_cache
            action.input_cache["search_target"] = target
            self.counters["searches"] += 1

    def _search_rng(self, game):
        raw = f"{game.job.get('seed')}:{game.job['seat']}:{game.obs.get('tick')}".encode()
        seed = int.from_bytes(hashlib.blake2b(raw, digest_size=8).digest(), "little")
        return random.Random(seed)

    def _consume_step(self, game, future, kind, collecting, active, completed, transitions):
        self._attach_board_result(future)
        alive = super()._consume_step(game, future, kind, collecting, active, completed, transitions)
        if (alive and game.job.get("bank") and int(game.obs.get("tick", 0)) >= int(getattr(game, "horizon_tick", 10**18))):
            self._truncate_bank(game, kind, collecting, active, completed, transitions)
            return False
        return alive

    @staticmethod
    def _attach_board_result(future):
        if getattr(future, "_board_wrapped", False):
            return
        raw = future.result

        def wrapped(*args, **kwargs):
            reply = raw(*args, **kwargs)
            if isinstance(reply, dict) and isinstance(reply.get("observation"), dict) and "board" in reply:
                reply["observation"]["board"] = reply["board"]
                if "reward_state" in reply:
                    reply["observation"]["tower_state"] = reply["reward_state"]
            return reply

        future.result = wrapped
        future._board_wrapped = True

    def after_native_step(self, game, reply):
        from .banana_hog_retrain import tower_potential
        if "board" not in reply or "reward_state" not in reply:
            raise RuntimeError("step reply missing board or reward_state")
        super().after_native_step(game, reply)
        seat = game.job["seat"]
        elixir = reply["observation"]["elixir"]
        trade = elixir_trade_potential(
            reply["board"], seat, elixir[seat], elixir[1 - seat], self.learner.row_table,
        )
        game.trade_potentials.append(trade.potential)
        self.learner.observe_board(game.episode, reply["board"], seat)
        new_events = game.episode.events[game.event_cursor:]
        game.event_cursor = len(game.episode.events)
        update_hog_stats(
            game.hog_stats, reply["board"], seat, int(reply["observation"]["tick"]),
            new_events, float(elixir[seat]), step_seconds=1.0,
        )
        self._maybe_snapshot(game, reply)

    def _maybe_snapshot(self, game, reply):
        if (self.args.state_bank_fraction <= 0 or game.job.get("bank")
                or not game.job.get("collect_bank") or int(self.args.max_snapshots) <= 0):
            return
        seen = game.snap_seen
        elapsed = (int(reply["observation"]["tick"]) - int(game.start_tick)) / 20.0
        reasons = critical_moments(
            reply["board"], reply["observation"], game.job["seat"],
            seen_cross=seen["seen_cross"], seen_towers=seen["seen_towers"],
            saw_double=seen["saw_double"], saw_hog_hand=seen["saw_hog_hand"],
            elapsed_seconds=elapsed,
        )
        if not reasons:
            return
        note_critical(seen, reasons)
        # Free a slot before capture: the engine enforces the same hard cap.
        bank = self.worker_banks[game.worker]
        if len(bank) >= int(self.args.max_snapshots):
            old = bank.pop(0)
            self.bank_index.pop(old["token"], None)
            self.native_request(game.worker, {"op": "drop_snapshot", "snapshot_id": old["snapshot_id"]})
        taken = self.native_request(game.worker, {"op": "snapshot"})
        observation = copy.deepcopy(reply["observation"])
        observation["board"] = copy.deepcopy(reply["board"])
        observation["tower_state"] = copy.deepcopy(reply["reward_state"])
        python_state = capture_match_python(game.episode, self._cache_policies(), {
            "obs": observation,
            "start_tick": int(game.start_tick),
            "tower_potential": float(game.potentials[-1]),
            "trade_potential": float(game.trade_potentials[-1]),
        })
        token = uuid.uuid4().hex
        item = {
            "token": token, "worker": game.worker, "snapshot_id": taken["snapshot_id"],
            "tick": taken.get("tick"), "reasons": reasons,
            "job": {"deck_id": game.job["deck_id"], "controller": game.job["controller"],
                    "seat": game.job["seat"], "seed": game.job["seed"]},
            "python": python_state,
        }
        bank = self.worker_banks[game.worker]
        bank.append(item)
        self.bank_index[token] = item
        self.counters["snapshots_taken"] += 1
        while len(bank) > int(self.args.max_snapshots):
            old = bank.pop(0)
            self.bank_index.pop(old["token"], None)
            try:
                self.native_request(game.worker, {"op": "drop_snapshot", "snapshot_id": old["snapshot_id"]})
            except Exception as exc:
                self.log(f"drop_snapshot failed on worker {game.worker}: {exc}")

    def _truncate_bank(self, game, kind, collecting, active, completed, transitions):
        game.bank_truncated = True
        game.bootstrap_value = self._bootstrap_value(game)
        if collecting and game.decisions:
            transitions.extend(self.game_transitions(game, 0.0))
        completed.append(self.record(game, kind, "BANK"))
        active.pop(game.worker, None)
        if game.worker < len(self.worker_states):
            self.worker_states[game.worker]["status"] = "finished"

    def _bootstrap_value(self, game) -> float:
        board = game.obs.get("board")
        if not isinstance(board, dict):
            raise RuntimeError("cannot bootstrap without a board")
        self.learner.observe_board(game.episode, board, game.job["seat"])
        now = (int(game.obs["tick"]) - int(game.start_tick)) / 20.0
        kwargs = {"deterministic": True, "placement_top_k": 576}
        if getattr(self.args, "native_placement_masks", False):
            kwargs["placement_mask_provider"] = self.mask_provider([game])
        with torch.no_grad():
            action = self.learner.act_batch(
                [(game.episode, game.obs, game.job["seat"], now)], **kwargs)[0]
        return float(action.value)

    def game_transitions(self, game, outcome):
        terminal = not bool(getattr(game, "bank_truncated", False))
        if len(game.potentials) != len(game.decisions) + 1 or len(game.trade_potentials) != len(game.decisions) + 1:
            raise RuntimeError("reward trajectory does not match decisions")
        if not game.decisions:
            return []
        audit = audit_game_rewards(
            game.potentials, game.trade_potentials, float(outcome) if terminal else 0.0,
            self.args.shaping_coefficient, self.args.trade_coef, terminal=terminal,
        )
        rewards = audit.pop("rewards")
        values = [float(action.value) for action in game.decisions]
        bootstrap = 0.0 if terminal else float(getattr(game, "bootstrap_value", 0.0))
        advantages, returns = gae_with_bootstrap(rewards, values, self.args.gae_lambda, bootstrap)
        audit.update({
            "id": game.episode.battle_id, "at": utc_now(), "bank": bool(game.job.get("bank")),
            "bootstrap": bootstrap, "decisions": len(values),
            "deck_id": game.job.get("deck_id"), "controller": game.job.get("controller"),
            **hog_summary(getattr(game, "hog_stats", fresh_hog_stats())),
        })
        with (self.run_dir / "reward-audit.jsonl").open("a") as stream:
            stream.write(json.dumps(audit) + "\n")
        game.audited = True
        weight = 1.0 / len(values)
        rows = []
        for action, advantage, ret in zip(game.decisions, advantages, returns):
            row = {"action": action, "advantage": advantage, "return": ret, "weight": weight}
            target = action.input_cache.get("search_target")
            if target is not None:
                row["search"] = target
            rows.append(row)
        return rows

    def record(self, game, kind, result, error=None):
        bank = bool(getattr(game, "job", {}).get("bank"))
        if result == "BANK":
            row = self._record_neutral(game, kind, "BANK", error)
        else:
            saved = {key: self.fun.get(key) for key in ("streak", "max_streak", "quickest_win", "longest_game")}
            row = super().record(game, kind, result, error)
            if bank and result in {"W", "L", "D"}:
                self._undo_winrate(game, result)
                self.fun.update(saved)
                row["counts_for_winrate"] = False
                self.counters["bank_episodes"] += 1
            else:
                row["counts_for_winrate"] = result in {"W", "L", "D"}
        row["bank"] = bank
        row.update(hog_summary(getattr(game, "hog_stats", fresh_hog_stats())))
        self._rewrite_match_tail(row)
        if kind == "train" and result != "VOID":
            self.counters["diag_games"] += 1
            summary = hog_summary(getattr(game, "hog_stats", fresh_hog_stats()))
            self.counters["hog_plays"] += summary["hog_plays"]
            self.counters["hog_connects"] += summary["hog_connects"]
            self.counters["plays"] += summary["plays"]
            self.counters["leak_seconds"] += summary["leak_seconds"]
        if result == "VOID" and not getattr(game, "audited", False):
            self._void_audit(game, error)
        # Match tensors are stored on actions and bank snapshots separately.
        # Retaining every completed battle here would grow RAM throughout a run.
        for cached_policy in [*self._cache_policies(), *self.snapshot_by_name.values()]:
            cache = getattr(cached_policy, "_board_cache", {})
            cache.pop(("episode", id(game.episode)), None)
            cache.pop(("battle", game.episode.battle_id), None)
        return row

    def _undo_winrate(self, game, result):
        self.counters["games"] -= 1
        self.counters[{"W": "wins", "L": "losses", "D": "draws"}[result]] -= 1
        self.counters["training_games"] -= 1
        cell = (game.job["deck_id"], game.job["controller"])
        bucket = self.sampler.results.get(cell)
        if bucket and bucket[-1] == result:
            bucket.pop()
        if self.train_rows and self.train_rows[-1].get("id") == game.episode.battle_id:
            self.train_rows.pop()

    def _record_neutral(self, game, kind, result, error):
        job = game.job
        seconds = (int(game.obs.get("tick", game.start_tick)) - int(game.start_tick)) / 20.0
        row = {
            "id": game.episode.battle_id, "at": utc_now(), "kind": kind,
            "deck_id": job["deck_id"], "deck_name": self.decks[job["deck_id"]]["name"],
            "controller": job["controller"], "seat": job["seat"], "seed": job["seed"],
            "result": result, "seconds": seconds,
            "policy_version": getattr(game, "behavior_version", self.version),
            "error": error, "bank": True, "counts_for_winrate": False,
        }
        self.recent.appendleft(row)
        with (self.run_dir / "matches.jsonl").open("a") as stream:
            stream.write(json.dumps(row, allow_nan=False) + "\n")
        self.counters["simulated_seconds"] += seconds
        self.counters["bank_episodes"] += 1
        self.fun["arena_hours"] = self.counters["simulated_seconds"] / 3600.0
        self.log(f"{kind} {row['deck_name']} / {job['controller']} seat{job['seat']} BANK · {seconds:.1f}s native")
        return row

    def _void_audit(self, game, error):
        row = {"id": getattr(getattr(game, "episode", None), "battle_id", None), "at": utc_now(),
               "ok": False, "void": True, "reason": "incomplete", "error": error,
               "bank": bool(getattr(game, "job", {}).get("bank"))}
        with (self.run_dir / "reward-audit.jsonl").open("a") as stream:
            stream.write(json.dumps(row) + "\n")

    def _rewrite_match_tail(self, row):
        path = self.run_dir / "matches.jsonl"
        if not path.exists():
            return
        data = path.read_bytes()
        head, _sep, _tail = data.rpartition(b"\n")
        previous, sep, _last = head.rpartition(b"\n")
        encoded = (json.dumps(row, allow_nan=False) + "\n").encode()
        path.write_bytes((previous + sep if sep else b"") + encoded)

    def restart_worker(self, worker):
        for item in self.worker_banks[worker]:
            self.bank_index.pop(item["token"], None)
        self.worker_banks[worker].clear()
        super().restart_worker(worker)

    def _aux_tensor(self, actions):
        if self.expert is None:
            return torch.zeros((), device=self.learner.device)
        targets = [action.input_cache.get("search_target") for action in actions
                   if action.input_cache.get("search_target") is not None]
        if not targets:
            return torch.zeros((), device=self.learner.device)
        with self.learner._ppo_backend():
            return self.expert.aux_loss(self.learner, targets)

    def update(self, transitions):
        if len(transitions) < 8 or self.stopping():
            return
        self.phase = "PPO · board · parent KL · GAE"
        self.publish(force=True)
        actions = [row["action"] for row in transitions]
        device = self.learner.device
        old = torch.stack([action.log_prob for action in actions]).to(device)
        targets = torch.tensor([row["return"] for row in transitions], device=device)
        advantages = torch.tensor([row["advantage"] for row in transitions], device=device)
        weights = torch.tensor([row["weight"] for row in transitions], device=device)
        controlled = torch.tensor([action.input_cache["can_play"] for action in actions], device=device).float()
        actor_weights = weights * controlled
        if actor_weights.sum() <= 0:
            return
        mean = (advantages * actor_weights).sum() / actor_weights.sum()
        variance = ((advantages - mean).square() * actor_weights).sum() / actor_weights.sum()
        advantages = ((advantages - mean) / variance.sqrt().clamp_min(0.05)).clamp(-5, 5)
        before = {
            "model": copy.deepcopy(self.learner.model.state_dict()),
            "wait": copy.deepcopy(self.learner.wait_head.state_dict()),
            "critic": copy.deepcopy(self.learner.critic.state_dict()),
            "optimizer": copy.deepcopy(self.optimizer.state_dict()),
            "board": copy.deepcopy(self.learner.checkpoint_payload()["board_state"]),
        }
        logs = []
        early = False
        for _epoch in range(self.args.epochs):
            order = torch.randperm(len(actions)).tolist()
            for start in range(0, len(actions), self.args.minibatch_size):
                if self.stopping():
                    break
                ids = order[start:start + self.args.minibatch_size]
                index = torch.tensor(ids, device=device)
                rows = [actions[i] for i in ids]
                logp, entropy, value = self.learner.evaluate_actions(rows)
                ratio = (logp - old[index]).exp()
                actor_weight, value_weight = actor_weights[index], weights[index]
                average = lambda tensor, weight: (tensor * weight).sum() / weight.sum().clamp_min(1e-9)
                kl = average((ratio - 1) - (logp - old[index]), actor_weight)
                if logs and float(kl.detach()) > self.args.target_kl:
                    early = True
                    break
                actor = average(-torch.minimum(ratio * advantages[index], ratio.clamp(0.9, 1.1) * advantages[index]), actor_weight)
                critic = average((value - targets[index]).square(), value_weight)
                card_kl, wait_kl = self.anchor_terms(rows)
                anchor = self.args.card_anchor * average(card_kl, actor_weight) + self.args.wait_anchor * average(wait_kl, actor_weight)
                ent = average(entropy, actor_weight)
                aux = self._aux_tensor(rows)
                loss = actor + 0.5 * critic + anchor - self.args.entropy_coef * ent + aux
                if not torch.isfinite(loss):
                    raise RuntimeError("nonfinite crazy-hog PPO loss")
                self.optimizer.zero_grad(set_to_none=True)
                loss.backward()
                norm = torch.nn.utils.clip_grad_norm_(
                    [parameter for group in self.optimizer.param_groups for parameter in group["params"]],
                    0.5, error_if_nonfinite=True)
                self.optimizer.step()
                logs.append({
                    "loss": float(loss.detach()), "policy_loss": float(actor.detach()),
                    "value_loss": float(critic.detach()), "entropy": float(ent.detach()),
                    "approx_kl": float(kl.detach()), "parent_card_kl": float(average(card_kl, actor_weight).detach()),
                    "parent_wait_kl": float(average(wait_kl, actor_weight).detach()),
                    "aux_loss": float(aux.detach()),
                    "clip_fraction": float(average(((ratio - 1).abs() > 0.1).float(), actor_weight).detach()),
                    "grad_norm": float(norm),
                })
                self.publish()
            if early or self.stopping():
                break
        if not logs:
            return
        probe = actions[::max(1, len(actions) // 256)][:256]
        with torch.no_grad():
            card, wait = self.anchor_terms(probe)
            valid = torch.tensor([action.input_cache["can_play"] for action in probe], device=device).float()
            parent_kl = float(((card + wait) * valid).sum() / valid.sum().clamp_min(1))
        if parent_kl > self.args.max_parent_kl:
            self.learner.model.load_state_dict(before["model"])
            self.learner.wait_head.load_state_dict(before["wait"])
            self.learner.critic.load_state_dict(before["critic"])
            self._load_board({"board_state": before["board"]})
            self.optimizer.load_state_dict(before["optimizer"])
            self.lower_actor_lr()
            self.guard["rollback_count"] += 1
            self.guard["anchor_rollbacks"] += 1
            self.log(f"Rolled back update: parent KL {parent_kl:.4f} > {self.args.max_parent_kl}")
        self.guard["anchor_kl"] = parent_kl
        self.audit_frozen_body()
        self.version += 1
        self.counters["updates"] += 1
        self.ppo = {key: sum(row[key] for row in logs) / len(logs) for key in logs[0]}
        rates = {group["role"]: group["lr"] for group in self.optimizer.param_groups}
        self.ppo.update(samples=len(actions), minibatches=len(logs), learning_rate=rates.get("actor", self.args.learning_rate),
                        group_learning_rates=rates, kl_early_stop=early, advantage_std=float(variance.sqrt()),
                        parent_kl=parent_kl)
        with (self.run_dir / "updates.jsonl").open("a") as stream:
            stream.write(json.dumps({"at": utc_now(), "updates": self.version, **self.ppo}) + "\n")
        self.log(f"Crazy Hog PPO {self.version}: {len(actions)} decisions · parent KL {parent_kl:.5f} · aux {self.ppo['aux_loss']:.5f}")
        self.save("latest")
        self._maybe_save_snapshot()

    def _maybe_save_snapshot(self):
        every = int(self.args.snapshot_every)
        if every <= 0 or self.version <= 0 or self.version % every != 0:
            return
        name = f"selfplay-v{self.version}"
        directory = self.run_dir / "models" / "snapshots"
        directory.mkdir(parents=True, exist_ok=True)
        path = directory / f"{name}.pt"
        self.learner.save_checkpoint(path)
        policy = BoardHogPolicy.from_checkpoint(path, device=self.args.device, trainable="heads")
        policy.eval()
        for parameter in policy.parameters():
            parameter.requires_grad_(False)
        self.snapshot_by_name[name] = policy
        self.snapshot_order.append(name)
        self.sampler.add_controller(name, [HOG_MIRROR])
        while len(self.snapshot_order) > int(self.args.snapshot_keep):
            retired = self.snapshot_order.pop(0)
            self.snapshot_by_name.pop(retired, None)
            self.sampler.remove_controller(retired)
            for worker, bank in enumerate(self.worker_banks):
                for item in list(bank):
                    if item["job"]["controller"] == retired:
                        self.native_request(worker, {"op": "drop_snapshot", "snapshot_id": item["snapshot_id"]})
                        bank.remove(item)
                        self.bank_index.pop(item["token"], None)
        self.counters["selfplay_snapshots"] += 1
        self.log(f"Self-play snapshot {name} plays {HOG_MIRROR} only; kept {len(self.snapshot_order)}")

    def runtime_extra(self):
        result = super().runtime_extra()
        result["selfplay_snapshots"] = list(self.snapshot_order)
        return result

    def restore_extra(self, runtime):
        super().restore_extra(runtime)
        for name in runtime.get("selfplay_snapshots", []):
            path = self.run_dir / "models" / "snapshots" / f"{name}.pt"
            policy = BoardHogPolicy.from_checkpoint(path, device=self.args.device, trainable="heads")
            policy.eval()
            for parameter in policy.parameters():
                parameter.requires_grad_(False)
            self.snapshot_by_name[name] = policy
            self.snapshot_order.append(name)

    def publish(self, force=False):
        due = force or (time.monotonic() - self.last_write) >= 2
        super().publish(force=force)
        if not due:
            return
        now = time.time()
        elapsed = max(1e-6, now - self.started)
        episodes = int(self.counters.get("games", 0)) + int(self.counters.get("bank_episodes", 0))
        row = {
            "at": utc_now(), "elapsed_seconds": now - self.started, "updates": int(self.counters.get("updates", 0)),
            "games": int(self.counters.get("games", 0)), "bank_episodes": int(self.counters.get("bank_episodes", 0)),
            "decisions": int(self.counters.get("decisions", 0)),
            "games_per_minute": int(self.counters.get("games", 0)) * 60.0 / elapsed,
            "episodes_per_minute": episodes * 60.0 / elapsed,
            "decisions_per_second": int(self.counters.get("decisions", 0)) / elapsed,
            "budget_percent": budget_percent(now, self.started, self.deadline),
            "eta_seconds": eta_seconds(now, self.started, self.deadline, self.status),
            "mean_learner_batch": (int(self.counters.get("learner_batch_rows", 0)) / int(self.counters["learner_batches"])
                                   if self.counters.get("learner_batches") else None),
            "hog_plays_per_game": (int(self.counters.get("hog_plays", 0)) / int(self.counters["diag_games"])
                                   if self.counters.get("diag_games") else None),
            "connect_rate": (int(self.counters.get("hog_connects", 0)) / int(self.counters["hog_plays"])
                             if self.counters.get("hog_plays") else None),
        }
        if self.ppo:
            row["loss"] = self.ppo.get("loss")
            row["approx_kl"] = self.ppo.get("approx_kl")
            row["entropy"] = self.ppo.get("entropy")
            row["aux_loss"] = self.ppo.get("aux_loss")
            row["parent_kl"] = self.ppo.get("parent_kl")
        with (self.run_dir / "metrics.jsonl").open("a") as stream:
            stream.write(json.dumps(row) + "\n")

    def _write_notes(self, resumed: bool):
        path = self.run_dir / "notes.md"
        if resumed and path.exists():
            with path.open("a") as stream:
                stream.write(f"\n- Resumed at {utc_now()} from policy version {self.version}. In-process banks were cleared.\n")
            return
        text = f"""# Crazy Hog 2.6 notes

- Created: {utc_now()}
- Parent: `{self.args.parent}`
- Engine: `{self.args.engine}` ({self.engine.get('sha256', '')[:16]})
- Workers: {self.args.workers}. Trade coefficient: {self.args.trade_coef}. Search: {self.args.search}.
- Deadline: {self.args.deadline}

## Prior evidence

banana-hog-2 (22 Oct 2026, 13,632 games) played about one Hog Rider per game and about sixteen Skeletons and Ice Spirits. PEKKA Bridge Spam stayed near a 0.5% win rate. The actor did not see the board. This run gives the learner the board, an elixir-trade potential, PFSP, and short state-bank episodes.

## This run

Lessons are appended when the run finishes. Until then, treat every win rate as unfinished.
"""
        path.write_text(text)

    def _finish_notes(self):
        path = self.run_dir / "notes.md"
        hog = (self.extra_state().get("hog") or {})
        lines = [
            "",
            f"## Close-out {utc_now()}",
            "",
            f"- Status: {self.status}. Updates: {self.version}. Win-rate games: {self.counters.get('games', 0)}. Bank episodes: {self.counters.get('bank_episodes', 0)}.",
            f"- Hog plays/game: {hog.get('hog_plays_per_game')}. Connect rate: {hog.get('connect_rate')}. Elixir seconds at 10: {hog.get('elixir_leak_seconds')}.",
            f"- Mean learner batch: {(self.extra_state().get('batch') or {}).get('mean_learner_batch')}.",
            "- Bank episodes are excluded from win rate. A truncated bank episode has no ±1 unless the match actually ended.",
            "- Self-play snapshots only enter the Hog mirror. PEKKA Bridge Spam weight follows its loss rate.",
            "",
        ]
        with path.open("a") as stream:
            stream.write("\n".join(lines))

    def run(self):
        try:
            super().run()
        finally:
            if getattr(self, "run_dir", None) is not None:
                try:
                    self._finish_notes()
                    from .crazy_hog_report import build_report
                    build_report(self.run_dir, self.run_dir / "report.html")
                except Exception:
                    pass


def configure_parser(parser):
    from .banana_hog_adaptive import configure_parser as adaptive_parser
    adaptive_parser(parser)
    for action in parser._actions:
        if "--engine" in getattr(action, "option_strings", ()):
            action.required = False
    parser.set_defaults(
        name="crazy-hog-2.6",
        parent=DEFAULT_PARENT,
        parent_name="banana-hog-2",
        pool=ROOT / "data/fast_league_decks.json",
        engine=DEFAULT_ENGINE,
        opponents="original",
        trainable="all",
        tower_hp=True,
        native_placement_masks=True,
        async_collection=True,
        engine_pool="thread",
        step_groups=1,
        workers=2,
        duration_seconds=10800,
        learning_rate=3e-5,
        backbone_learning_rate=3e-6,
        critic_learning_rate=3e-4,
        eval_seeds=1,
        confirmation_seeds=1,
        eval_every=2400,
        regression_threshold=0.02,
        shaping_coefficient=0.5,
    )
    parser.add_argument("--board-learning-rate", type=float, default=3e-4)
    parser.add_argument("--trade-coef", type=float, default=0.05, help="Elixir-trade potential coefficient, per elixir")
    parser.add_argument("--state-bank-fraction", type=float, default=0.3)
    parser.add_argument("--max-snapshots", type=int, default=2, help="In-process snapshots kept per worker")
    parser.add_argument("--bank-horizon-seconds", type=float, default=40.0)
    parser.add_argument("--snapshot-every", type=int, default=10, help="Learner updates between self-play snapshots")
    parser.add_argument("--snapshot-keep", type=int, default=4, help="Newest self-play snapshots kept in the league")
    parser.add_argument("--pfsp-power", type=float, default=2.0)
    parser.add_argument("--pfsp-floor", type=float, default=0.05)
    parser.add_argument("--search", choices=("none", "expert"), default="none")
    parser.add_argument("--allow-unpinned-engine", action="store_true",
                        help="Allow an engine binary whose sha256 does not start with the pinned PGO prefix")
    parser.add_argument("--sync-waves", action=argparse.BooleanOptionalAction, default=True,
                        help="Finish batched waves before updates/evaluation (default); async mode is experimental")


def main():
    from .banana_hog_retrain import main as retrain_main
    retrain_main(CrazyHogLeague, configure_parser)


if __name__ == "__main__":
    main()
