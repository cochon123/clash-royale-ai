"""Deadline-bound, warm-started Hog league using verified asymmetric native games.

One GPU batches policy inference/learning; independent native subprocesses use
the CPU. Entire waves use one policy version, and only native terminal outcomes
enter PPO. Frozen seed banks never enter the training buffer.
"""
from __future__ import annotations

import argparse
import concurrent.futures as futures
import datetime as dt
import json
import math
import os
import random
import selectors
import signal
import subprocess
import time
import uuid
from collections import Counter, deque
from dataclasses import dataclass, field
from pathlib import Path

import torch
from torch.nn import functional as F

from .banana_fast_policy import FastBananaPolicy
from .banana_policy import BananaEpisode, _native_card
from .banana_train import ROOT, atomic_json, sha256, utc_now

HOLDOUTS = {"miner-wallbreakers", "giant-princes"}
CONTROLLERS = ("bc-greedy", "bc-sampled", "hog1-greedy")
EVAL_CONTROLLERS = ("bc-greedy", "hog1-greedy")
STOPPING = False
# After the first engine in a wave returns, wait this long for others that are
# already finishing so the GPU still sees a batch. Games still inside the
# engine are not held for this window.
PIPELINE_GATHER_S = 0.015


def deadline_epoch(value: str) -> float:
    parsed = dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    if parsed.tzinfo is None:
        raise ValueError("deadline must include its UTC offset")
    return parsed.timestamp()


def metric(rows):
    """Outcome score with a conservative Hoeffding 95% interval, draws=.5."""
    counts = Counter(r["result"] if isinstance(r, dict) else r for r in rows)
    n = sum(counts[k] for k in ("W", "L", "D"))
    score = (counts["W"] + .5 * counts["D"]) / n if n else None
    radius = math.sqrt(math.log(40) / (2 * n)) if n else None
    return {"n": n, "w": counts["W"], "l": counts["L"], "d": counts["D"],
            "score": score, "low": max(0, score - radius) if n else None,
            "high": min(1, score + radius) if n else None, "voids": counts["VOID"]}


def macro_score(rows):
    cells = {}
    for row in rows:
        cells.setdefault((row["deck_id"], row["controller"]), []).append(row)
    scores = [metric(v)["score"] for v in cells.values()]
    valid = [v for v in scores if v is not None]
    return sum(valid) / len(valid) if valid else None


def paired_delta(candidate, baseline, seed=991):
    """Paired bootstrap by seed/seat block; keeps all deck/controller cells."""
    def key(r):
        return r["deck_id"], r["controller"], r["seat"], r["seed"]
    points = {"W": 1., "D": .5, "L": 0.}
    old = {key(r): points[r["result"]] for r in baseline if r["result"] in points}
    blocks = {}
    for row in candidate:
        k = key(row)
        if k in old and row["result"] in points:
            blocks.setdefault((row["seed"], row["seat"]), []).append(points[row["result"]] - old[k])
    vals = [sum(v) / len(v) for v in blocks.values()]
    if not vals:
        return {"delta": None, "low": None, "high": None, "pairs": 0, "blocks": 0}
    rng = random.Random(seed)
    boot = sorted(sum(rng.choices(vals, k=len(vals))) / len(vals) for _ in range(2000))
    return {"delta": sum(vals) / len(vals), "low": boot[49] if len(vals) >= 4 else None,
            "high": boot[1949] if len(vals) >= 4 else None,
            "pairs": sum(len(v) for v in blocks.values()), "blocks": len(vals)}


class Sampler:
    def __init__(self, decks, seed):
        self.cells = [(d["id"], c) for d in decks if not d["holdout"] for c in CONTROLLERS]
        self.rng = random.Random(seed)
        self.seats = Counter()
        self.results = {cell: deque(maxlen=200) for cell in self.cells}
        self.warm = self.cells[:]
        self.rng.shuffle(self.warm)

    def probabilities(self):
        # At least 75% uniform coverage, with a bounded posterior-hardness tail.
        hardness = [(1 + sum({"W": 0., "D": .5, "L": 1.}[v] for v in self.results[cell])) /
                    (2 + len(self.results[cell])) for cell in self.cells]
        n = len(self.cells)
        probs, active, remaining = [.75 / n] * n, list(range(n)), .25
        cap = 2.5 / n
        while active and remaining > 1e-12:
            total = sum(hardness[i] for i in active)
            portions = {i: remaining * hardness[i] / total for i in active}
            capped = [i for i in active if probs[i] + portions[i] > cap]
            if not capped:
                for i in active:
                    probs[i] += portions[i]
                break
            for i in capped:
                remaining -= cap - probs[i]
                probs[i] = cap
                active.remove(i)
        return probs

    def draw(self):
        cell = self.warm.pop() if self.warm else self.rng.choices(self.cells, self.probabilities())[0]
        seat = self.seats[cell] % 2
        self.seats[cell] += 1
        return {"deck_id": cell[0], "controller": cell[1], "seat": seat,
                "seed": self.rng.randrange(1, 1_000_000_000)}

    def record(self, job, result):
        if result != "VOID":
            self.results[(job["deck_id"], job["controller"])].append(result)

    def state_dict(self):
        return {"cells": self.cells, "rng": self.rng.getstate(), "seats": dict(self.seats),
                "results": {cell: list(rows) for cell, rows in self.results.items()}, "warm": self.warm}

    @classmethod
    def from_state_dict(cls, state):
        result = object.__new__(cls)
        result.cells = state["cells"]
        result.rng = random.Random()
        result.rng.setstate(state["rng"])
        result.seats = Counter(state["seats"])
        result.results = {cell: deque(rows, maxlen=200) for cell, rows in state["results"].items()}
        result.warm = state["warm"]
        return result


class NativeClient:
    def __init__(self, binary, checkpoint, error_path):
        self.err = error_path.open("ab")
        self.proc = subprocess.Popen([str(binary), "--checkpoint", str(checkpoint)],
                                     stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=self.err,
                                     cwd=ROOT, bufsize=0)
        self.selector = selectors.DefaultSelector()
        self.selector.register(self.proc.stdout, selectors.EVENT_READ)
        self.pending = b""

    def request(self, request, timeout=90):
        if self.proc.poll() is not None:
            raise RuntimeError(f"native_exit:{self.proc.returncode}")
        self.proc.stdin.write(json.dumps(request, separators=(",", ":")).encode() + b"\n")
        end = time.monotonic() + timeout
        while b"\n" not in self.pending:
            remaining = end - time.monotonic()
            if remaining <= 0 or not self.selector.select(max(0, remaining)):
                raise TimeoutError("native_response_timeout")
            chunk = os.read(self.proc.stdout.fileno(), 65536)
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
        if self.proc.poll() is None:
            self.proc.terminate()
            try:
                self.proc.wait(timeout=3)
            except subprocess.TimeoutExpired:
                self.proc.kill()
                self.proc.wait(timeout=3)
        self.selector.close()
        self.err.close()


@dataclass
class Game:
    worker: int
    job: dict
    episode: BananaEpisode
    obs: dict
    start_tick: int
    decisions: list = field(default_factory=list)
    steps: int = 0


class League:
    def __init__(self, args):
        self.args, self.run_dir = args, args.run_dir.resolve()
        self.run_dir.mkdir(parents=True, exist_ok=True)
        (self.run_dir / "models").mkdir(exist_ok=True)
        self.deadline = deadline_epoch(args.deadline)
        self.started = time.time()
        self.rng = random.Random(args.seed)
        torch.manual_seed(args.seed)
        torch.set_num_threads(args.torch_threads)
        torch.set_num_interop_threads(1)
        self.pool = json.loads(args.pool.read_text())["decks"]
        for deck in self.pool:
            deck["holdout"] = deck["id"] in HOLDOUTS
        self.decks = {d["id"]: d for d in self.pool}
        self.hog = self.decks["hog-mirror"]
        self.sampler = self.make_sampler(self.pool, args.seed + 14)
        self.counters = Counter()
        self.errors = Counter()
        self.pace = {"infer_s": 0., "infer_batches": 0, "infer_games": 0, "engine_s": 0., "engine_steps": 0}
        self.train_rows = []
        self.recent = deque(maxlen=40)
        self.fun = {"card_plays": {}, "quickest_win": None, "longest_game": None,
                    "streak": 0, "max_streak": 0}
        self.evaluation = {"history": [], "best": None, "baseline_complete": False,
                           "selection_seed_count": args.eval_seeds,
                           "confirmation": None, "baseline": {}}
        self.checkpoints = deque(maxlen=30)
        self.worker_states = [{"id": i, "status": "starting"} for i in range(args.workers)]
        self.resources, self.ppo, self.last_write, self.last_resource = {}, {}, 0., 0.
        self.version = 0
        self.status, self.phase = "preparing", "loading exact banana-hog-1 weights"
        self.parent = {"name": getattr(args, "parent_name", "banana-hog-1"), "path": str(args.parent.resolve()),
                       "sha256": sha256(args.parent), "initial_tensor_parity": False}
        self.engine = {"name": "Original native engine · asymmetric reference runner",
                       "sha256": sha256(args.engine), "path": str(args.engine.resolve())}
        self.publish(force=True)
        self.learner = self.make_learner(device=args.device, seed=args.seed,
                                        specialization=getattr(args, "name", "banana-hog-2"), weights=args.parent)
        self.parent_policy = self.make_parent_policy(device=args.device, seed=args.seed + 1,
                                              specialization="frozen-banana-hog-1", weights=args.parent)
        self.base = self.make_base_policy(device=args.device, seed=args.seed + 2,
                                     specialization="frozen-policy-4.4.2")
        for policy in (self.learner, self.parent_policy, self.base):
            policy.eval()
        for policy in (self.parent_policy, self.base):
            for p in policy.parameters():
                p.requires_grad_(False)
        for deck in self.pool:
            for card, native in zip(deck["cards"], deck["native_cards"]):
                if self.learner.vocab.encode(card) == self.learner.vocab.unk_id:
                    raise ValueError(f"Unknown policy card: {card}")
                compact = "".join(c.lower() for c in native if c.isalnum())
                mapped = self.learner.native_name_to_policy.get(compact) or _native_card(native, deck["cards"])
                if mapped != card:
                    raise ValueError(f"Native alias mismatch: {native} maps to {mapped}, expected {card}")
        self.parent["initial_tensor_parity"] = all(
            torch.equal(self.learner.state_dict()[name], value)
            for name, value in self.parent_policy.state_dict().items())
        if not self.parent["initial_tensor_parity"]:
            raise RuntimeError("warm start differs from exact parent weights")
        self.frozen_signature = self.policy_signature(self.parent_policy), self.policy_signature(self.base)
        self.optimizer = self.make_optimizer()
        if args.resume and (self.run_dir / "models/latest.pt").exists():
            self.restore(self.run_dir / "models/latest.pt")
        else:
            if (self.run_dir / "models/initial.pt").exists():
                raise RuntimeError("existing run needs --resume; refusing to overwrite")
            self.save("initial")
            self.save("best")
            self.save("latest")
        self.clients = [NativeClient(args.engine, args.checkpoint, self.run_dir / f"native-worker-{i}.log")
                        for i in range(args.workers)]
        self.executor = futures.ThreadPoolExecutor(max_workers=args.workers)
        self.last_eval = time.time()
        self.eval_seconds = 120.
        manifest = {"format": "banana-hog-league-v1", "name": getattr(args, "name", "banana-hog-2"), "parent": self.parent,
                    "engine": self.engine, "deadline": args.deadline, "timezone": "America/Edmonton",
                    "pool_sha256": sha256(args.pool), "decks": self.pool,
                    "trainer_sha256": sha256(Path(__file__)),
                    "native_checkpoint_metadata_sha256": sha256(args.checkpoint / "checkpoint.json"),
                    "card_aliases_sha256": sha256(ROOT / "data/policy_card_aliases.json"),
                    "train_seed_range": [1, 999_999_999],
                    "selection_seed_range": [2_000_000_000, 2_000_000_000 + args.eval_seeds - 1],
                    "confirmation_seed_range": [3_000_000_000, 3_000_000_000 + args.eval_seeds - 1],
                    "controllers": list(CONTROLLERS), "args": {k: str(v) if isinstance(v, Path) else v
                                                               for k, v in vars(args).items()},
                    "observation_contract": "Both decks, own authoritative hand, both elixirs, causal accepted-play history. No entity positions, tower HP, images, or opponent hand.",
                    "reward": "Native terminal W=+1, L=-1, D=0. Errors, caps and deadline interruptions are VOID and excluded from PPO and W/L/D.",
                    "evaluation_contract": "Frozen original-4.4.2 and frozen banana-hog-1 bots, greedy play, both seats, same native seeds for parent and candidate; two unseen archetypes. Selection bank chooses best; separate confirmation bank used only at end. No claim of human or expert deck pilots."}
        manifest.update(self.manifest_extra())
        atomic_json(self.run_dir / "manifest.json", manifest)
        self.log("Exact parent tensor parity verified; CUDA policies loaded; starting native games.")

    @staticmethod
    def policy_signature(policy):
        # Full state equality is checked at initialization; this checksum catches
        # unexpected mutation of frozen actors/heads after each evaluation.
        return tuple((name, float(p.detach().double().sum().cpu())) for name, p in policy.state_dict().items())

    def make_sampler(self, decks, seed):
        return Sampler(decks, seed)

    def make_base_policy(self, **kwargs):
        return FastBananaPolicy(**kwargs)

    def make_parent_policy(self, **kwargs):
        return FastBananaPolicy(**kwargs)

    def make_learner(self, **kwargs):
        return FastBananaPolicy(**kwargs)

    def make_optimizer(self):
        return torch.optim.Adam(self.learner.parameters(), lr=self.args.learning_rate)

    def manifest_extra(self):
        return {}

    def extra_state(self):
        return {}

    def runtime_extra(self):
        return {}

    def restore_extra(self, runtime):
        pass

    def log(self, text):
        line = f"[{utc_now()}] {text}"
        print(line, flush=True)
        with (self.run_dir / "train.log").open("a") as out:
            out.write(line + "\n")

    def stopping(self):
        return STOPPING or time.time() >= self.deadline or (self.run_dir / "STOP").exists()

    def update_resources(self):
        if time.monotonic() - self.last_resource < 10:
            return
        self.last_resource = time.monotonic()
        try:
            import psutil
            self.resources.update(cpu_percent=psutil.cpu_percent(), ram_gb=psutil.virtual_memory().used / 2**30)
        except ImportError:
            pass
        if self.args.device.startswith("cuda"):
            try:
                result = subprocess.run(["nvidia-smi", "--query-gpu=utilization.gpu,memory.used",
                                         "--format=csv,noheader,nounits"], capture_output=True, text=True, timeout=2)
                gpu, mem = result.stdout.splitlines()[0].split(",")
                self.resources.update(gpu_util_percent=float(gpu), gpu_memory_mb=float(mem))
            except (OSError, ValueError, IndexError, subprocess.TimeoutExpired):
                pass

    def publish(self, force=False):
        if not force and time.monotonic() - self.last_write < 2:
            return
        self.last_write = time.monotonic()
        self.update_resources()
        latest = self.evaluation.get("latest_rows", [])
        baseline = self.evaluation.get("baseline", {}).get("selection", [])
        matchups = []
        for deck in self.pool:
            ident = deck["id"]
            ev = metric([r for r in latest if r["deck_id"] == ident])
            parent = metric([r for r in baseline if r["deck_id"] == ident])
            matchups.append({"deck_id": ident, "deck_name": deck["name"], "holdout": deck["holdout"],
                             "train": metric([r for r in self.train_rows if r["deck_id"] == ident]),
                             "evaluation": ev, "parent": parent,
                             "delta": ev["score"] - parent["score"] if ev["n"] and parent["n"] else None})
        probs = self.sampler.probabilities()
        sampling = []
        for deck in self.pool:
            if deck["holdout"]:
                continue
            cells = [i for i, cell in enumerate(self.sampler.cells) if cell[0] == deck["id"]]
            sampling.append({"deck_id": deck["id"], "probability": sum(probs[i] for i in cells),
                             "actual_games": sum(1 for r in self.train_rows if r["deck_id"] == deck["id"] and r["result"] != "VOID")})
        elapsed = max(1., time.time() - self.started)
        state = {"name": getattr(self.args, "name", "banana-hog-2"), "status": self.status, "phase": self.phase,
                 "updated_at": utc_now(), "started_at": dt.datetime.fromtimestamp(self.started, dt.timezone.utc).isoformat(),
                 "deadline": self.args.deadline, "timezone": "America/Edmonton", "parent": self.parent,
                 "engine": self.engine, "device": self.args.device, "workers": self.worker_states,
                 "decks": self.pool, "counters": dict(self.counters),
                 "rates": {"games_per_minute": self.counters["games"] * 60 / elapsed,
                           "decisions_per_second": self.counters["decisions"] / elapsed},
                 "resources": self.resources, "ppo": self.ppo, "recent_matches": list(self.recent),
                 "matchups": matchups, "evaluation": self.evaluation, "sampler": sampling,
                 "fun": self.fun, "checkpoints": list(self.checkpoints), "errors": dict(self.errors),
                 "policy_version": self.version,
                 "notes": ["Frozen bot policies are not expert deck pilots or human ladder opponents.",
                           "Model sees both decks, own hand, both elixirs and accepted-play history; no unit positions or tower HP.",
                           "Miner Wall Breakers and Giant Double Prince are evaluation-only archetypes.",
                           "Win rate excludes VOID; draws score half a win. Small samples have wide uncertainty.",
                           "Training waves are on-policy; only original native terminal outcomes enter learning.",
                           f"Absolute deadline: {self.args.deadline}."]}
        state.update(self.extra_state())
        atomic_json(self.run_dir / "state.json", state)

    def save(self, stage):
        path = self.run_dir / "models" / f"{stage}.pt"
        payload = {"format": "banana-native-ppo-v1", "name": getattr(self.args, "name", "banana-hog-2"), "specialist": "hog",
                   "checkpoint_stage": stage, "model_state": self.learner.model.state_dict(),
                   "critic_state": self.learner.critic.state_dict(), "wait_state": self.learner.wait_head.state_dict(),
                   "optimizer_state": self.optimizer.state_dict(), "vocab": self.learner.vocab.to_dict(),
                   "config": dict(self.learner.config), "deck": self.hog["cards"], "model_version": "4.4.2",
                   "parent": self.parent, "updates": self.version, "created_at": utc_now(),
                   "runtime": {"started": self.started, "counters": self.counters, "errors": self.errors,
                               "train_rows": self.train_rows, "recent": list(self.recent), "fun": self.fun,
                               "evaluation": self.evaluation, "sampler": self.sampler.state_dict(),
                               "checkpoints": list(self.checkpoints), "ppo": self.ppo,
                               "rng": self.rng.getstate(), "torch_rng": torch.get_rng_state(),
                               "cuda_rng": torch.cuda.get_rng_state_all() if torch.cuda.is_available() else None}}
        payload["runtime"].update(self.runtime_extra())
        payload["experiment"] = self.manifest_extra().get("experiment", {})
        temp = path.with_name(f".{path.name}.tmp")
        torch.save(payload, temp)
        with temp.open("rb") as stream:
            os.fsync(stream.fileno())
        os.replace(temp, path)
        atomic_json(path.with_suffix(".sha256.json"), {"sha256": sha256(path), "at": utc_now(), "updates": self.version})
        self.checkpoints.append({"at": utc_now(), "stage": stage, "path": str(path), "updates": self.version})

    def restore(self, path):
        payload = torch.load(path, map_location=self.args.device, weights_only=False)
        if payload["parent"]["sha256"] != self.parent["sha256"]:
            raise RuntimeError("resume parent hash mismatch")
        previous_manifest = self.run_dir / "manifest.json"
        if previous_manifest.exists():
            previous = json.loads(previous_manifest.read_text())
            if previous["engine"]["sha256"] != self.engine["sha256"] or previous["pool_sha256"] != sha256(self.args.pool):
                raise RuntimeError("resume engine/deck pool hash mismatch")
            if deadline_epoch(previous["deadline"]) != self.deadline:
                raise RuntimeError("resume cannot change the absolute overnight deadline")
        self.learner.model.load_state_dict(payload["model_state"])
        self.learner.critic.load_state_dict(payload["critic_state"])
        self.learner.wait_head.load_state_dict(payload["wait_state"])
        self.optimizer.load_state_dict(payload["optimizer_state"])
        self.version = payload["updates"]
        runtime = payload["runtime"]
        for attr in ("started", "counters", "errors", "train_rows", "fun", "evaluation", "sampler", "ppo"):
            setattr(self, attr, runtime[attr])
        if isinstance(self.sampler, dict):
            self.sampler = self.make_sampler(self.pool, self.args.seed + 14).from_state_dict(self.sampler)
        self.recent = deque(runtime["recent"], maxlen=40)
        self.checkpoints = deque(runtime["checkpoints"], maxlen=30)
        self.rng.setstate(runtime["rng"])
        torch.set_rng_state(runtime["torch_rng"].cpu())
        if runtime["cuda_rng"] is not None and torch.cuda.is_available():
            torch.cuda.set_rng_state_all([s.cpu() for s in runtime["cuda_rng"]])
        self.restore_extra(runtime)
        self.log(f"Resumed {path.name}, policy version {self.version}, {self.counters['games']} native games.")

    def verify_decks(self, reset, expected):
        actual = reset.get("configured_decks")
        if not isinstance(actual, list) or len(actual) != 2:
            raise RuntimeError("reset_missing_decks")
        for side in range(2):
            rows = actual[side]
            rows = rows.get("cards", []) if isinstance(rows, dict) else rows
            cards = []
            for row in rows:
                name = row if isinstance(row, str) else row.get("name", "")
                compact = "".join(c.lower() for c in name if c.isalnum())
                cards.append(self.learner.native_name_to_policy.get(compact) or _native_card(name, expected[side]))
            if sorted(cards) != sorted(expected[side]):
                raise RuntimeError(f"native_deck_mismatch:side{side}:{cards}")

    def reset(self, worker, job):
        deck = self.decks[job["deck_id"]]
        native = [self.hog["native_cards"], deck["native_cards"]]
        slugs = [tuple(self.hog["cards"]), tuple(deck["cards"])]
        if job["seat"] == 1:
            native.reverse()
            slugs.reverse()
        response = self.clients[worker].request({"op": "reset", "deck0": native[0], "deck1": native[1],
                                                 "seed": job["seed"], "max_ticks": self.args.match_ticks + 100})
        self.verify_decks(response, slugs)
        response = self.clients[worker].request({"op": "step", "actions": [], "ticks": 100})
        if response["ticks_advanced"] != 100:
            raise RuntimeError("native_warmup_failed")
        return Game(worker, job, BananaEpisode(tuple(slugs), battle_id=uuid.uuid4().hex),
                    response["observation"], int(response["observation"]["tick"]))

    def record(self, game, kind, result, error=None):
        job = game.job
        seconds = (int(game.obs["tick"]) - game.start_tick) / 20.
        row = {"id": game.episode.battle_id, "at": utc_now(), "kind": kind,
               "deck_id": job["deck_id"], "deck_name": self.decks[job["deck_id"]]["name"],
               "controller": job["controller"], "seat": job["seat"], "seed": job["seed"],
               "result": result, "seconds": seconds, "policy_version": self.version,
               "error": error}
        self.recent.appendleft(row)
        with (self.run_dir / "matches.jsonl").open("a") as stream:
            stream.write(json.dumps(row, allow_nan=False) + "\n")
        self.counters["simulated_seconds"] += seconds
        self.fun["arena_hours"] = self.counters["simulated_seconds"] / 3600.
        if result == "VOID":
            self.counters["voids"] += 1
            self.errors[(error or "unknown").split(":")[0]] += 1
        else:
            self.counters["games"] += 1
            self.counters[{"W": "wins", "L": "losses", "D": "draws"}[result]] += 1
        if kind == "train":
            self.train_rows.append(row)
            self.sampler.record(job, result)
            if result != "VOID":
                self.counters["training_games"] += 1
                self.fun["streak"] = self.fun["streak"] + 1 if result == "W" else 0
                self.fun["max_streak"] = max(self.fun["streak"], self.fun["max_streak"])
                if result == "W" and (self.fun["quickest_win"] is None or seconds < self.fun["quickest_win"]["seconds"]):
                    self.fun["quickest_win"] = row
                if self.fun["longest_game"] is None or seconds > self.fun["longest_game"]["seconds"]:
                    self.fun["longest_game"] = row
        else:
            self.counters["evaluation_games"] += int(result != "VOID")
        self.log(f"{kind} {row['deck_name']} / {job['controller']} seat{job['seat']} {result} · {seconds:.1f}s native" + (f" · {error}" if error else ""))
        return row

    def play(self, jobs, policy, kind, collecting=False):
        """Fixed-weight waves; no optimizer mutation while games are active."""
        completed, transitions = [], []
        for offset in range(0, len(jobs), self.args.workers):
            if self.stopping():
                break
            active = {}
            pending = {}
            for worker, job in enumerate(jobs[offset:offset + self.args.workers]):
                self.worker_states[worker] = {"id": worker, "status": "resetting", "deck": self.decks[job["deck_id"]]["name"],
                                              "controller": job["controller"], "seat": job["seat"]}
                pending[self.executor.submit(self.reset, worker, job)] = (worker, job)
            self.publish(force=True)
            while pending:
                done, _ = futures.wait(pending, timeout=.5, return_when=futures.FIRST_COMPLETED)
                self.publish()
                for future in done:
                    worker, job = pending.pop(future)
                    try:
                        active[worker] = future.result()
                    except Exception as exc:
                        dummy = Game(worker, job, BananaEpisode((tuple(self.hog["cards"]), tuple(self.decks[job["deck_id"]]["cards"]))), {"tick": 0}, 0)
                        completed.append(self.record(dummy, kind, "VOID", str(exc)))
                        self.restart_worker(worker)
            step_jobs = {}
            ready = list(active.values())
            held = []  # barrier mode keeps early finishers until the whole wave returns
            while active:
                if self.stopping():
                    if step_jobs:
                        futures.wait(step_jobs, timeout=45)
                        for future in list(step_jobs):
                            game = step_jobs.pop(future)
                            if game.worker in active:
                                self._consume_step(game, future, kind, collecting, active, completed, transitions)
                    for game in list(active.values()):
                        completed.append(self.record(game, kind, "VOID", "deadline_or_stop"))
                    break
                if ready:
                    self._launch_steps(ready, policy, collecting, step_jobs)
                    ready = []
                if not step_jobs:
                    break
                done = self._completed_steps(step_jobs, pipeline=getattr(self.args, "pipeline", False))
                self.publish()
                finished_early = []
                for future in done:
                    game = step_jobs.pop(future)
                    if self._consume_step(game, future, kind, collecting, active, completed, transitions):
                        finished_early.append(game)
                if getattr(self.args, "pipeline", False):
                    ready = finished_early
                else:
                    held.extend(finished_early)
                    if not step_jobs:
                        ready = held
                        held = []
                self.publish()
        return completed, transitions

    def _pace(self):
        pace = getattr(self, "pace", None)
        if pace is None:
            pace = {"infer_s": 0., "infer_batches": 0, "infer_games": 0, "engine_s": 0., "engine_steps": 0}
            self.pace = pace
        return pace

    def _launch_steps(self, games, policy, collecting, step_jobs):
        """Infer one ready batch and send those games back to their engines.

        Only games whose previous step has already returned are included, so a
        worker's client is never asked for a decision and a step at once.
        """
        sampled = {game.worker: [] for game in games}
        requests = [(game.episode, game.obs, game.job["seat"], (int(game.obs["tick"]) - game.start_tick) / 20.)
                    for game in games]
        started = time.perf_counter()
        actions = self.learner_actions(policy, requests, games, collecting)
        for game, action in zip(games, actions):
            sampled[game.worker].append((policy, action))
            if collecting:
                game.decisions.append(action)
        opponents = {}
        # Keep controller/row ordering stable while sharing each policy's
        # forward pass across its greedy and sampled controllers.
        for controller in CONTROLLERS:
            opponent = self.parent_policy if controller == "hog1-greedy" else self.base
            for game in games:
                if game.job["controller"] == controller:
                    opponents.setdefault(id(opponent), (opponent, []))[1].append(game)
        for opponent, subset in opponents.values():
            controllers = [game.job["controller"] for game in subset]
            controller = controllers[0] if len(set(controllers)) == 1 else controllers
            requests = [(game.episode, game.obs, 1 - game.job["seat"], (int(game.obs["tick"]) - game.start_tick) / 20.)
                        for game in subset]
            if isinstance(controller, list) and getattr(self.opponent_actions, "__func__", None) is League.opponent_actions:
                # The base hook predates per-row modes; its only decode option
                # is deterministic. AdaptiveLeague's hook also supplies masks.
                actions = opponent.act_batch(requests, deterministic=[name != "bc-sampled" for name in controllers])
            else:
                actions = self.opponent_actions(opponent, requests, subset, controller)
            for game, action in zip(subset, actions):
                sampled[game.worker].append((opponent, action))
        pace = self._pace()
        pace["infer_s"] += time.perf_counter() - started
        pace["infer_batches"] += 1
        pace["infer_games"] += len(games)
        for game in games:
            rows = sampled[game.worker]
            # Physical side order keeps event history deterministic and
            # independent of which side the learner occupies.
            rows.sort(key=lambda pair: pair[1].side)
            game.pending_rows = rows
            native = [action.native_request() for _, action in rows if not action.wait]
            game.step_sent = time.perf_counter()
            step_jobs[self.executor.submit(self.clients[game.worker].request,
                                           {"op": "step", "actions": native, "ticks": 20}, 45)] = game

    def _completed_steps(self, step_jobs, pipeline):
        if not pipeline:
            done, _ = futures.wait(step_jobs, timeout=.5, return_when=futures.ALL_COMPLETED)
            return done if len(done) == len(step_jobs) else set()
        done, _ = futures.wait(step_jobs, timeout=.5, return_when=futures.FIRST_COMPLETED)
        if not done:
            return set()
        pending = [future for future in step_jobs if future not in done]
        if pending:
            extra, _ = futures.wait(pending, timeout=PIPELINE_GATHER_S, return_when=futures.ALL_COMPLETED)
            done = set(done) | set(extra)
        return done

    def _consume_step(self, game, future, kind, collecting, active, completed, transitions):
        """Apply one engine reply. True means the game still needs another decision."""
        pace = self._pace()
        pace["engine_s"] += max(0., time.perf_counter() - getattr(game, "step_sent", time.perf_counter()))
        pace["engine_steps"] += 1
        try:
            reply = future.result()
            if "reward_state" in reply:
                reply["observation"]["tower_state"] = reply["reward_state"]
            now = (int(game.obs["tick"]) - game.start_tick) / 20.
            rows = game.pending_rows
            submitted = [(agent, action) for agent, action in rows if not action.wait]
            if len(reply["actions"]) != len(submitted):
                raise RuntimeError("native_action_result_count")
            for (agent, action), response in zip(submitted, reply["actions"]):
                if not isinstance(response.get("accepted"), bool):
                    raise RuntimeError("native_action_result_missing_status")
                if response["accepted"]:
                    agent.record_action(game.episode, action, now, True)
                    self.counters["accepted_plays"] += 1
                    if collecting and action.side == game.job["seat"]:
                        plays = self.fun["card_plays"]
                        plays[action.card] = plays.get(action.card, 0) + 1
                else:
                    self.counters["rejected_plays"] += 1
            self.counters["decisions"] += len(rows)
            if collecting:
                self.after_native_step(game, reply)
            game.obs = reply["observation"]
            game.steps += 1
            self.worker_states[game.worker].update(status="playing", tick=game.obs["tick"],
                                                  entity_count=game.obs.get("entity_count"))
            if reply["terminated"]:
                winner = game.obs["winner_side"]
                if winner not in (0, 1, -1):
                    raise RuntimeError("unresolved_native_winner")
                result = "D" if winner == -1 else "W" if winner == game.job["seat"] else "L"
                completed.append(self.record(game, kind, result))
                if collecting:
                    outcome = {"W": 1., "D": 0., "L": -1.}[result]
                    transitions.extend(self.game_transitions(game, outcome))
                del active[game.worker]
                self.worker_states[game.worker]["status"] = "finished"
                return False
            if reply["truncated"]:
                completed.append(self.record(game, kind, "VOID", "native_tick_cap"))
                del active[game.worker]
                return False
            return True
        except Exception as exc:
            completed.append(self.record(game, kind, "VOID", str(exc)))
            active.pop(game.worker, None)
            self.restart_worker(game.worker)
            return False

    def learner_actions(self, policy, requests, games, collecting):
        return policy.act_batch(requests, deterministic=not collecting)

    def opponent_actions(self, policy, requests, games, controller):
        return policy.act_batch(requests, deterministic=controller != "bc-sampled")

    def after_native_step(self, game, reply):
        pass

    def game_transitions(self, game, outcome):
        return [(action, outcome) for action in game.decisions]

    def restart_worker(self, worker):
        self.clients[worker].close()
        self.clients[worker] = NativeClient(self.args.engine, self.args.checkpoint,
                                           self.run_dir / f"native-worker-{worker}.log")
        self.worker_states[worker]["status"] = "restarting"

    def update(self, transitions):
        if len(transitions) < 8 or self.stopping():
            return
        self.phase = "PPO update · GPU"
        self.publish(force=True)
        actions, returns = zip(*transitions)
        device = self.learner.device
        rewards = torch.tensor(returns, device=device)
        old = torch.stack([a.log_prob for a in actions]).to(device)
        old_values = torch.stack([a.value for a in actions]).to(device)
        advantages = rewards - old_values
        advantages /= advantages.square().mean().sqrt().clamp_min(1.)
        logs = []
        early_stop = False
        for _ in range(self.args.epochs):
            order = torch.randperm(len(actions)).tolist()
            for start in range(0, len(actions), self.args.minibatch_size):
                if self.stopping():
                    break
                ids = order[start:start + self.args.minibatch_size]
                ix = torch.tensor(ids, device=device)
                logp, entropy, value = self.learner.evaluate_actions([actions[i] for i in ids])
                delta = logp - old[ix]
                ratio = delta.exp()
                kl = ((ratio - 1) - delta).mean()
                if logs and float(kl.detach()) > self.args.target_kl:
                    early_stop = True
                    break
                actor = -torch.minimum(ratio * advantages[ix], ratio.clamp(.8, 1.2) * advantages[ix]).mean()
                critic = F.mse_loss(value, rewards[ix])
                ent = entropy.mean()
                loss = actor + .5 * critic - .005 * ent
                if not torch.isfinite(loss):
                    raise RuntimeError("nonfinite_PPO_loss")
                self.optimizer.zero_grad(set_to_none=True)
                loss.backward()
                norm = torch.nn.utils.clip_grad_norm_(self.learner.parameters(), 1., error_if_nonfinite=True)
                self.optimizer.step()
                logs.append({"loss": float(loss.detach()), "policy_loss": float(actor.detach()),
                             "value_loss": float(critic.detach()), "entropy": float(ent.detach()),
                             "approx_kl": float(kl.detach()), "clip_fraction": float(((ratio - 1).abs() > .2).float().mean().detach()),
                             "grad_norm": float(norm)})
                self.publish()
            if early_stop or self.stopping():
                break
        if logs:
            self.version += 1
            self.counters["updates"] += 1
            self.ppo = {key: sum(row[key] for row in logs) / len(logs) for key in logs[0]}
            self.ppo.update(samples=len(actions), minibatches=len(logs), learning_rate=self.args.learning_rate,
                            kl_early_stop=early_stop)
            with (self.run_dir / "updates.jsonl").open("a") as out:
                out.write(json.dumps({"at": utc_now(), "updates": self.version, **self.ppo}) + "\n")
            self.log(f"PPO {self.version}: {len(actions)} on-policy samples · KL {self.ppo['approx_kl']:.5f} · loss {self.ppo['loss']:.4f}")
            self.save("latest")

    def eval_jobs(self, bank):
        base = 2_000_000_000 if bank == "selection" else 3_000_000_000
        return [{"deck_id": deck["id"], "controller": controller, "seat": seat, "seed": base + seed}
                for seed in range(self.args.eval_seeds) for deck in self.pool
                for controller in EVAL_CONTROLLERS for seat in (0, 1)]

    def evaluate(self, bank="selection", parent=False):
        start = time.monotonic()
        self.status = "evaluating"
        label = "banana-hog-1 baseline" if parent else f"banana-hog-2 v{self.version}"
        self.phase = f"{bank} evaluation · {label}"
        self.publish(force=True)
        rows, _ = self.play(self.eval_jobs(bank), self.parent_policy if parent else self.learner,
                            f"evaluation-{bank}-{'parent' if parent else 'candidate'}")
        expected = len(self.eval_jobs(bank))
        complete = len(rows) == expected and all(r["result"] != "VOID" for r in rows)
        score = macro_score(rows)
        if parent:
            if complete:
                self.evaluation["baseline"][bank] = rows
                self.evaluation["baseline_complete"] = all(k in self.evaluation["baseline"] for k in ("selection", "confirmation"))
                if bank == "selection" and self.evaluation["best"] is None:
                    self.evaluation["best"] = {"score": score, "path": str(self.run_dir / "models/best.pt"),
                                               "at": utc_now(), "updates": 0, "source": "exact parent safety fallback"}
        else:
            baseline = self.evaluation["baseline"].get(bank, [])
            paired = paired_delta(rows, baseline)
            summary = {"at": utc_now(), "bank": bank, "games": metric(rows)["n"], "score": score,
                       "parent_score": macro_score(baseline), "complete": complete,
                       "checkpoint": self.version, **paired}
            self.evaluation["history"].append(summary)
            if bank == "selection":
                self.evaluation["latest_rows"] = rows
                self.evaluation["latest"] = summary
                best = self.evaluation["best"]
                if complete and score is not None and (best is None or score > best["score"]):
                    self.evaluation["best"] = {"score": score, "path": str(self.run_dir / "models/best.pt"),
                                               "at": utc_now(), "updates": self.version, "source": "selection bank"}
                    self.save("best")
            else:
                self.evaluation["confirmation"] = {**summary, "rows": rows}
        if self.frozen_signature != (self.policy_signature(self.parent_policy), self.policy_signature(self.base)):
            raise RuntimeError("frozen_opponent_weights_changed")
        self.eval_seconds = max(self.eval_seconds, time.monotonic() - start)
        self.log(f"{self.phase} · {metric(rows)['n']}/{expected} valid · score {score} · complete={complete}")
        self.status = "running"
        self.save("latest")
        self.publish(force=True)
        return complete

    def run(self):
        try:
            already_confirmed = self.evaluation.get("confirmation") is not None
            while not self.stopping() and not already_confirmed:
                # Reserve a measured evaluation budget, minimum ten minutes.
                reserve = max(600., 3 * self.eval_seconds + 180)
                if self.evaluation["baseline_complete"] and self.deadline - time.time() <= reserve:
                    break
                self.status, self.phase = "running", f"training · on-policy wave v{self.version}"
                jobs = [self.sampler.draw() for _ in range(self.args.workers)]
                rows, transitions = self.play(jobs, self.learner, "train", collecting=True)
                if not any(r["result"] != "VOID" for r in rows) and not self.stopping():
                    raise RuntimeError("whole_native_wave_void; inspect native worker errors")
                self.update(transitions)
                self.save("latest")
                self.publish(force=True)
                if self.args.max_waves and self.version >= self.args.max_waves:
                    break
                # Launch real games and an update before the longer baseline bank.
                if not self.evaluation["baseline_complete"] and not self.stopping():
                    for bank in ("selection", "confirmation"):
                        if bank not in self.evaluation["baseline"] and not self.stopping():
                            self.evaluate(bank, parent=True)
                    if not self.stopping():
                        self.evaluate()
                    self.last_eval = time.time()
                elif time.time() - self.last_eval >= self.args.eval_every and not self.stopping():
                    self.evaluate()
                    self.last_eval = time.time()
            if not self.stopping() and not self.args.max_waves and not already_confirmed:
                # A final selection look, then evaluate only the selected snapshot
                # on the independent confirmation bank. Preserve last iterate.
                self.save("last-training")
                self.evaluate()
                payload = torch.load(self.run_dir / "models/best.pt", map_location=self.args.device, weights_only=False)
                self.learner.model.load_state_dict(payload["model_state"])
                self.learner.critic.load_state_dict(payload["critic_state"])
                self.learner.wait_head.load_state_dict(payload["wait_state"])
                self.optimizer.load_state_dict(payload["optimizer_state"])
                self.version = payload["updates"]
                self.evaluate("confirmation")
                self.phase = "training finished · final selected model saved · waiting for 05:00 deadline"
                self.save("final")
                self.status = "running"
                already_confirmed = True
            if already_confirmed and not self.args.max_waves:
                self.phase = "final selected model saved · waiting for 05:00 deadline"
                self.status = "running"
                # Stay supervised until the requested wall-clock end; no further
                # gradient updates after using the confirmation bank.
                while not self.stopping():
                    self.publish()
                    time.sleep(2)
            self.status = "completed" if time.time() >= self.deadline or self.args.max_waves else "stopped"
            self.phase = "finished · checkpoints saved"
        except BaseException as exc:
            self.status, self.phase = "failed", f"{type(exc).__name__}: {exc}"
            self.errors[type(exc).__name__] += 1
            self.log(self.phase)
            raise
        finally:
            self.save("latest")
            if not (self.run_dir / "models/final.pt").exists() and self.status in ("completed", "stopped"):
                self.save("final")
            self.publish(force=True)
            for client in self.clients:
                client.close()
            self.executor.shutdown(wait=True, cancel_futures=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--parent", type=Path, default=ROOT / "runs/banana-league-20261019/models/hog/final.pt")
    parser.add_argument("--pool", type=Path, default=ROOT / "data/hog2_opponent_pool.json")
    parser.add_argument("--engine", type=Path, default=ROOT / "analysis/hog2/reference-target/release/league_reference")
    parser.add_argument("--checkpoint", type=Path, default=ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state")
    parser.add_argument("--deadline", default="2026-10-20T11:00:00Z")
    parser.add_argument("--workers", type=int, default=10)
    parser.add_argument("--device", default="cuda")
    parser.add_argument("--torch-threads", type=int, default=2)
    parser.add_argument("--learning-rate", type=float, default=1e-5)
    parser.add_argument("--target-kl", type=float, default=.03)
    parser.add_argument("--minibatch-size", type=int, default=64)
    parser.add_argument("--epochs", type=int, default=2)
    parser.add_argument("--match-ticks", type=int, default=7300)
    parser.add_argument("--eval-every", type=float, default=2700)
    parser.add_argument("--eval-seeds", type=int, default=2)
    parser.add_argument("--seed", type=int, default=20261019)
    parser.add_argument("--resume", action="store_true")
    parser.add_argument("--max-waves", type=int, default=0, help="Bounded real-engine supervision run")
    args = parser.parse_args()
    if args.workers < 1 or args.eval_seeds < 1:
        parser.error("workers and eval-seeds must be positive")
    if deadline_epoch(args.deadline) <= time.time():
        # A restarted overnight service must not begin another run after 05:00.
        if (args.run_dir / "state.json").exists():
            state = json.loads((args.run_dir / "state.json").read_text())
            state.update(status="completed", phase="deadline reached; restart declined", updated_at=utc_now())
            atomic_json(args.run_dir / "state.json", state)
        print("Deadline has already passed; no new training started.", flush=True)
        return
    def stop(_signum, _frame):
        global STOPPING
        STOPPING = True
    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGINT, stop)
    League(args).run()


if __name__ == "__main__":
    main()
