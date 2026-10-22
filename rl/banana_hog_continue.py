"""Continue Hog1 with its optimizer/RNG state and a fresh confirmation bank."""
from __future__ import annotations

import argparse
import copy
import datetime as dt
import json
import os
import shutil
import signal
import time
from pathlib import Path
from types import SimpleNamespace

import torch

from . import banana_hog_league as core
from .banana_hog_retrain import StableLeague
from .banana_train import ROOT, atomic_json, sha256, utc_now


def phase_difference(current, origin):
    return {key: max(0, value - origin.get(key, 0)) for key, value in current.items() if isinstance(value, (float, int))}


def final_budget(selection_seconds, selection_seeds, confirmation_seeds, safety=1.2):
    """Budget a final selection plus both full confirmation runs and cleanup."""
    return safety * selection_seconds * (1 + 2 * confirmation_seeds / selection_seeds) + 120


def may_start_evaluation(remaining, evaluation_seconds, reserved_final_seconds):
    return remaining > 1.2 * evaluation_seconds + reserved_final_seconds


def plain_args(values):
    result = dict(values)
    for key in ("run_dir", "parent", "pool", "engine", "checkpoint", "source_run"):
        if key in result:
            result[key] = Path(result[key])
    return SimpleNamespace(**result)


def prepare_continuation(source_run, run_dir, duration_seconds, confirmation_seeds, eval_every):
    """Create a separate phase atomically; never modify the completed run."""
    source_run, run_dir = source_run.resolve(), run_dir.resolve()
    marker = run_dir / "continuation.json"
    if marker.exists():
        lineage = json.loads(marker.read_text())
        if Path(lineage["source_run"]) != source_run:
            raise ValueError("existing continuation belongs to a different source run")
        manifest = run_dir / "manifest.json"
        return plain_args(json.loads(manifest.read_text())["args"] if manifest.exists() else lineage["args"])
    if (run_dir / "models/latest.pt").exists() or (run_dir / "state.json").exists():
        raise ValueError("destination already contains an unrelated run")
    manifest = json.loads((source_run / "manifest.json").read_text())
    source_state = json.loads((source_run / "state.json").read_text())
    source = source_run / "models/last-training.pt"
    if not source.exists():
        raise ValueError("continuation needs the last training iterate, not the final selected weights")
    payload = torch.load(source, map_location="cpu", weights_only=False)
    best = source_run / "models/best.pt"
    best_payload = torch.load(best, map_location="cpu", weights_only=False)
    if payload["parent"]["sha256"] != manifest["parent"]["sha256"]:
        raise ValueError("source checkpoint has a different original parent")
    if payload["runtime"]["evaluation"].get("confirmation") is not None:
        raise ValueError("last-training checkpoint already contains confirmation results")
    now = time.time()
    args = plain_args(manifest["args"])
    args.run_dir, args.source_run = run_dir, source_run
    args.duration_seconds = duration_seconds
    args.deadline = dt.datetime.fromtimestamp(now + duration_seconds, dt.timezone.utc).isoformat()
    args.confirmation_seeds, args.eval_every = confirmation_seeds, eval_every
    args.resume, args.max_waves = True, 0
    (run_dir / "models").mkdir(parents=True, exist_ok=True)
    (run_dir / "bin").mkdir(exist_ok=True)
    original_engine = Path(manifest["engine"]["path"])
    if sha256(original_engine) != manifest["engine"]["sha256"]:
        raise ValueError("source native engine changed")
    args.engine = run_dir / "bin/league_reference"
    shutil.copy2(original_engine, args.engine)
    runtime = payload["runtime"]
    origin = {"counters": copy.deepcopy(runtime["counters"]), "card_plays": copy.deepcopy(runtime["fun"]["card_plays"]),
              "trace_stats": copy.deepcopy(runtime.get("trace_stats", {}))}
    lineage = {"phase": 2, "source_run": str(source_run), "source_checkpoint": str(source),
        "source_checkpoint_sha256": sha256(source), "source_updates": payload["updates"],
        "source_best_updates": best_payload["updates"], "source_best_sha256": sha256(best),
        "source_counters": origin["counters"], "origins": origin,
        "prior_confirmation": {k: v for k, v in (source_state["evaluation"].get("confirmation") or {}).items() if k != "rows"},
        "started_at": dt.datetime.fromtimestamp(now, dt.timezone.utc).isoformat(),
        "deadline": args.deadline, "created_at": utc_now(),
        "args": {k: str(v) if isinstance(v, Path) else v for k, v in vars(args).items()}}
    # Keep model, optimizer, sampler and Torch/CUDA/Python RNG values intact.
    # Only phase clock, paths and evaluation bookkeeping are changed.
    runtime["started"] = now
    evaluation = runtime["evaluation"]
    evaluation["confirmation"] = None
    evaluation.pop("final_artifact", None)
    evaluation["baseline"].pop("confirmation", None)
    evaluation["baseline_complete"] = False
    evaluation["best"]["path"] = str(run_dir / "models/best.pt")
    evaluation["last_selection_epoch"] = now
    evaluation["last_evaluated_updates"] = payload["updates"]
    evaluation["history"] = [row for row in evaluation["history"] if row.get("bank") == "selection"]
    runtime["continuation"] = lineage
    payload["checkpoint_stage"] = "continuation-initial"
    payload["created_at"] = utc_now()
    temp = run_dir / "models/.initial.tmp"
    torch.save(payload, temp)
    os.replace(temp, run_dir / "models/initial.pt")
    shutil.copy2(run_dir / "models/initial.pt", run_dir / "models/latest.pt")
    shutil.copy2(best, run_dir / "models/best.pt")
    for stage in ("initial", "latest", "best"):
        path = run_dir / "models" / f"{stage}.pt"
        atomic_json(path.with_suffix(".sha256.json"), {"sha256": sha256(path), "at": utc_now()})
    atomic_json(marker, lineage)
    return args


class ContinuingLeague(StableLeague):
    def __init__(self, args):
        self.lineage = json.loads((args.run_dir / "continuation.json").read_text())
        self.selection_seconds = 460.
        super().__init__(args)
        self.log(f"Continuing update {self.version}; rollback/selection best is update {self.lineage['source_best_updates']}; original parent anchor unchanged.")

    def manifest_extra(self):
        result = super().manifest_extra()
        result["continuation_trainer_sha256"] = sha256(Path(__file__))
        result["continuation"] = self.lineage
        result["confirmation_seed_range"] = [3_200_000_000, 3_200_000_000 + self.args.confirmation_seeds - 1]
        experiment = result["experiment"]
        experiment["description"] = "Three more hours continuing Hog1-r2 from its last trained weights, optimizer and RNG state"
        experiment["training_changes"] += ["Less frequent selection evaluation; cached frozen-parent baseline retained",
            "Fresh, larger final confirmation bank; deadline planner reserves its full measured cost",
            "Avoid repeated evaluation of unchanged weights at finalization"]
        experiment["notes"] += [f"Continuation starts at update {self.lineage['source_updates']}; previous selected best remains update {self.lineage['source_best_updates']}.",
            "Displayed totals include the first phase; phase counts and rates show this additional compute budget.",
            "The earlier incomplete confirmation is archived and never enters learning or the new confirmation result."]
        return result

    def extra_state(self):
        result = super().extra_state()
        origin = self.lineage["origins"]
        delta = phase_difference(self.counters, origin["counters"])
        elapsed = max(1., time.time() - self.started)
        rates = {"games_per_minute": delta.get("games", 0)*60/elapsed,
                 "decisions_per_second": delta.get("decisions", 0)/elapsed}
        result.update(continuation=self.lineage, phase_counters=delta, phase_rates=rates, rates=rates,
            phase_card_plays=phase_difference(self.fun["card_plays"], origin["card_plays"]),
            phase_training_diagnostics=phase_difference(self.trace_stats, origin["trace_stats"]))
        return result

    def runtime_extra(self):
        return {**super().runtime_extra(), "continuation": self.lineage, "selection_seconds": self.selection_seconds}

    def restore_extra(self, runtime):
        super().restore_extra(runtime)
        self.selection_seconds = runtime.get("selection_seconds", runtime["evaluation"].get("measured_bank_seconds", 460.))

    def restore(self, path):
        manifest = self.run_dir / "manifest.json"
        if manifest.exists():
            saved = json.loads(manifest.read_text())
            if saved.get("continuation_trainer_sha256") != sha256(Path(__file__)):
                raise ValueError("continuation trainer changed; refusing a silent experiment change")
            if saved["args"]["confirmation_seeds"] != self.args.confirmation_seeds:
                raise ValueError("continuation confirmation bank changed")
        super().restore(path)

    def eval_jobs(self, bank):
        if bank != "confirmation":
            return super().eval_jobs(bank)
        return [{"deck_id": deck["id"], "controller": controller, "seat": seat, "seed": 3_200_000_000 + seed}
                for seed in range(self.args.confirmation_seeds) for deck in self.pool
                for controller in core.EVAL_CONTROLLERS for seat in (0, 1)]

    def evaluate(self, bank="selection", parent=False):
        if bank == "confirmation":
            return self.evaluate_confirmation(parent)
        started = time.monotonic()
        rollbacks = self.guard["rollback_count"]
        complete = super().evaluate(bank, parent)
        if bank == "selection":
            self.selection_seconds = max(self.selection_seconds, time.monotonic() - started)
            if complete and not parent:
                # If a guard restores different weights, evaluate that state
                # before considering it a fresh result for the current version.
                self.evaluation["last_evaluated_updates"] = self.version if self.guard["rollback_count"] == rollbacks else None
        return complete

    def evaluate_confirmation(self, parent=False):
        """Finish missing native games after a restart, with weights frozen."""
        if not self.evaluation.get("final_selection_frozen"):
            raise RuntimeError("confirmation requires a frozen final checkpoint")
        jobs = self.eval_jobs("confirmation")
        kind = f"evaluation-confirmation-{'parent' if parent else 'candidate'}"
        key = lambda row: (row["deck_id"], row["controller"], row["seat"], row["seed"])
        expected = {key(job) for job in jobs}
        completed = {}
        path = self.run_dir / "matches.jsonl"
        if path.exists():
            for line in path.read_text().splitlines():
                try:
                    row = json.loads(line)
                    if row.get("kind") == kind and row.get("policy_version") == self.version and row.get("result") in ("W", "L", "D") and key(row) in expected:
                        completed[key(row)] = row
                except (ValueError, KeyError):
                    continue
        missing = [job for job in jobs if key(job) not in completed]
        self.status, self.phase = "evaluating", f"fresh confirmation · {'parent' if parent else 'selected model'} · {len(completed)}/{len(jobs)} retained"
        self.publish(force=True)
        rows, _ = self.play(missing, self.parent_policy if parent else self.learner, kind)
        for row in rows:
            if row["result"] in ("W", "L", "D"):
                completed[key(row)] = row
        combined = [completed[key(job)] for job in jobs if key(job) in completed]
        complete = len(combined) == len(jobs)
        if parent:
            if complete:
                self.evaluation["baseline"]["confirmation"] = combined
                self.evaluation["baseline_complete"] = True
            self.evaluation["confirmation_parent_progress"] = {"games": len(combined), "target_games": len(jobs), "complete": complete}
        else:
            baseline = self.evaluation["baseline"].get("confirmation", [])
            summary = {"at": utc_now(), "bank": "confirmation", "games": len(combined), "target_games": len(jobs),
                       "score": core.macro_score(combined), "parent_score": core.macro_score(baseline),
                       "complete": complete, "checkpoint": self.version, **core.paired_delta(combined, baseline)}
            self.evaluation["history"].append(summary)
            self.evaluation["confirmation"] = {**summary, "rows": combined}
        if self.frozen_signature != (self.policy_signature(self.parent_policy), self.policy_signature(self.base)):
            raise RuntimeError("frozen opponent weights changed")
        self.log(f"Fresh confirmation {'parent' if parent else 'candidate'}: {len(combined)}/{len(jobs)} valid, complete={complete}")
        self.save("latest")
        self.publish(force=True)
        return complete

    def run(self):
        try:
            finalized = bool((self.evaluation.get("confirmation") or {}).get("complete"))
            selection_frozen = self.evaluation.get("final_selection_frozen", False) or self.evaluation.get("confirmation") is not None
            last_eval = float(self.evaluation.get("last_selection_epoch", time.time()))
            while not self.stopping() and not selection_frozen:
                reserve = final_budget(self.selection_seconds, self.args.eval_seeds, self.args.confirmation_seeds)
                remaining = self.deadline - time.time()
                if remaining <= reserve + 90:
                    break
                self.status, self.phase = "running", f"continuation · stable heads · wave {self.version}"
                jobs = [self.sampler.draw() for _ in range(self.args.workers)]
                rows, transitions = self.play(jobs, self.learner, "train", collecting=True)
                if not any(r["result"] != "VOID" for r in rows) and not self.stopping():
                    raise RuntimeError("whole native continuation wave VOID")
                self.update(transitions)
                self.evaluation["last_selection_epoch"] = last_eval
                self.save("latest")
                self.publish(force=True)
                remaining = self.deadline - time.time()
                reserve = final_budget(self.selection_seconds, self.args.eval_seeds, self.args.confirmation_seeds)
                if time.time()-last_eval >= self.args.eval_every and may_start_evaluation(remaining, self.selection_seconds, reserve) and not self.stopping():
                    self.evaluate()
                    last_eval = time.time()
                    self.evaluation["last_selection_epoch"] = last_eval
            if not self.stopping() and not finalized:
                if not selection_frozen:
                    self.save("last-training")
                    if self.evaluation.get("last_evaluated_updates") != self.version:
                        self.evaluate()
                self.save_selected_final()
                self.evaluation["final_selection_frozen"] = True
                self.save("latest")
                while not self.stopping() and "confirmation" not in self.evaluation["baseline"]:
                    self.evaluate("confirmation", parent=True)
                while not self.stopping() and not bool((self.evaluation.get("confirmation") or {}).get("complete")):
                    self.evaluate("confirmation")
                self.save_selected_final()
                finalized = True
            if finalized:
                confirmed = bool((self.evaluation.get("confirmation") or {}).get("complete"))
                self.status = "running"
                self.phase = "final confirmation complete · waiting for deadline" if confirmed else "final evaluation interrupted · selected checkpoint retained"
                while not self.stopping():
                    self.publish()
                    time.sleep(2)
            self.status = "completed" if time.time() >= self.deadline else "stopped"
            self.phase = "three-hour continuation finished · selected checkpoint saved"
        except BaseException as exc:
            self.status, self.phase = "failed", f"{type(exc).__name__}: {exc}"
            self.errors[type(exc).__name__] += 1
            self.log(self.phase)
            raise
        finally:
            self.save("latest")
            if self.status in ("completed", "stopped"):
                self.save_selected_final()
            self.publish(force=True)
            for client in self.clients:
                client.close()
            self.executor.shutdown(wait=True, cancel_futures=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-run", type=Path, default=ROOT / "runs/banana-hog-1-retrain-20261020")
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--duration-seconds", type=float, default=10800)
    parser.add_argument("--confirmation-seeds", type=int, default=4)
    parser.add_argument("--eval-every", type=float, default=2700)
    parser.add_argument("--prepare-only", action="store_true")
    cli = parser.parse_args()
    if cli.duration_seconds <= 0 or cli.confirmation_seeds < 1 or cli.eval_every <= 0:
        parser.error("duration, confirmation seeds and evaluation period must be positive")
    args = prepare_continuation(cli.source_run, cli.run_dir, cli.duration_seconds, cli.confirmation_seeds, cli.eval_every)
    if cli.prepare_only:
        print(json.dumps({"run_dir": str(args.run_dir), "deadline": args.deadline}))
        return
    if core.deadline_epoch(args.deadline) <= time.time():
        print("Continuation deadline passed; no further training started.", flush=True)
        return
    def stop(_signal, _frame):
        core.STOPPING = True
    signal.signal(signal.SIGINT, stop)
    signal.signal(signal.SIGTERM, stop)
    ContinuingLeague(args).run()


if __name__ == "__main__":
    main()
