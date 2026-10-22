"""Continuation must preserve actual training state and keep new tests separate."""
import json
import tempfile
import unittest
import time
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import Mock

import torch

from .banana_hog_continue import (ContinuingLeague, final_budget, may_start_evaluation,
                                   phase_difference, prepare_continuation)
from .banana_train import ROOT, sha256


def equal_tree(a, b):
    if isinstance(a, torch.Tensor):
        return isinstance(b, torch.Tensor) and torch.equal(a, b)
    if isinstance(a, dict):
        return a.keys() == b.keys() and all(equal_tree(a[k], b[k]) for k in a)
    if isinstance(a, (tuple, list)):
        return type(a) is type(b) and len(a) == len(b) and all(equal_tree(x, y) for x, y in zip(a, b))
    return a == b


class ContinuationInvariants(unittest.TestCase):
    def test_warm_start_preserves_last_iterate_optimizer_rng_and_original_files(self):
        source = ROOT / "runs/banana-hog-1-retrain-20261020"
        if not source.exists():
            self.skipTest("historical Hog checkpoints were deleted at the user's request")
        tracked = [source / "models/last-training.pt", source / "models/best.pt", source / "models/final.pt", source / "state.json", source / "manifest.json"]
        hashes = {p: sha256(p) for p in tracked}
        original = torch.load(tracked[0], map_location="cpu", weights_only=False)
        with tempfile.TemporaryDirectory() as directory:
            target = Path(directory) / "phase2"
            args = prepare_continuation(source, target, 10800, 4, 2700)
            seeded = torch.load(target / "models/latest.pt", map_location="cpu", weights_only=False)
            selected = torch.load(target / "models/best.pt", map_location="cpu", weights_only=False)
            for key in ("model_state", "critic_state", "wait_state", "optimizer_state"):
                self.assertTrue(equal_tree(original[key], seeded[key]), key)
            for key in ("rng", "torch_rng", "cuda_rng", "sampler"):
                self.assertTrue(equal_tree(original["runtime"][key], seeded["runtime"][key]), key)
            self.assertEqual(seeded["updates"], 130)
            self.assertEqual(selected["updates"], 78)
            self.assertIsNone(seeded["runtime"]["evaluation"]["confirmation"])
            self.assertNotIn("confirmation", seeded["runtime"]["evaluation"]["baseline"])
            self.assertEqual(seeded["runtime"]["evaluation"]["best"]["path"], str(target / "models/best.pt"))
            again = prepare_continuation(source, target, 10800, 4, 2700)
            self.assertEqual(args.deadline, again.deadline)
            lineage = json.loads((target / "continuation.json").read_text())
            self.assertFalse(lineage["prior_confirmation"]["complete"])
        self.assertEqual(hashes, {p: sha256(p) for p in tracked})

    def test_new_confirmation_is_larger_and_disjoint(self):
        league = object.__new__(ContinuingLeague)
        league.args = SimpleNamespace(confirmation_seeds=4, eval_seeds=2)
        league.pool = json.loads((ROOT / "data/hog2_opponent_pool.json").read_text())["decks"]
        jobs = league.eval_jobs("confirmation")
        self.assertEqual(len(jobs), 176)
        self.assertEqual({j["seed"] for j in jobs}, set(range(3_200_000_000, 3_200_000_004)))
        self.assertEqual({j["seat"] for j in jobs}, {0,1})
        self.assertTrue(all(j["seed"] >= 3_200_000_000 for j in jobs))

    def test_final_budget_accounts_for_both_larger_confirmation_runs(self):
        reserved = final_budget(460, 2, 4)
        self.assertGreaterEqual(reserved, 460 * (1+2*2) + 120)
        self.assertFalse(may_start_evaluation(reserved + 460, 460, reserved))
        self.assertTrue(may_start_evaluation(reserved + 600, 460, reserved))

    def test_phase_rates_use_phase_counts(self):
        self.assertEqual(phase_difference({"games":1320,"hogs":1659}, {"games":1300,"hogs":1649}), {"games":20,"hogs":10})
        self.assertEqual(phase_difference({"games":0}, {"games":1300}), {"games":0})

    def test_confirmation_resume_only_runs_missing_valid_games(self):
        with tempfile.TemporaryDirectory() as directory:
            league = object.__new__(ContinuingLeague)
            league.run_dir, league.version = Path(directory), 78
            league.args = SimpleNamespace(confirmation_seeds=1, eval_seeds=2)
            league.pool = [{"id": "hog-mirror"}]
            jobs = league.eval_jobs("confirmation")
            kind = "evaluation-confirmation-candidate"
            row = lambda job: {**job, "kind": kind, "policy_version": 78, "result": "W"}
            recorded = [row(jobs[0]), {**row(jobs[1]), "policy_version": 77}, {**row(jobs[2]), "result": "VOID"}]
            (league.run_dir / "matches.jsonl").write_text("\n".join(json.dumps(r) for r in recorded)+"\n")
            league.evaluation = {"final_selection_frozen": True, "baseline": {"confirmation": [row(job) for job in jobs]}, "history": []}
            league.parent_policy, league.base, league.learner = object(), object(), object()
            league.frozen_signature = (0, 0)
            league.policy_signature = lambda policy: 0
            league.play = Mock(return_value=([row(job) for job in jobs[1:]], []))
            league.log, league.publish, league.save = Mock(), Mock(), Mock()
            self.assertTrue(league.evaluate_confirmation())
            self.assertEqual(league.play.call_args.args[0], jobs[1:])
            self.assertEqual(league.evaluation["confirmation"]["games"], 4)
            self.assertTrue(league.evaluation["confirmation"]["complete"])

    def test_incomplete_confirmation_resume_never_reenters_training(self):
        league = object.__new__(ContinuingLeague)
        league.evaluation = {"confirmation": {"complete": False}, "final_selection_frozen": True,
                             "baseline": {"confirmation": []}}
        league.deadline = time.time()+100
        league.stopping = lambda: league.evaluation["confirmation"]["complete"]
        league.play = Mock(side_effect=AssertionError("cannot train after looking at confirmation"))
        def finish(bank):
            self.assertEqual(bank, "confirmation")
            league.evaluation["confirmation"]["complete"] = True
            return True
        league.evaluate = Mock(side_effect=finish)
        league.save_selected_final, league.save, league.publish = Mock(), Mock(), Mock()
        league.clients, league.executor = [], Mock()
        league.run()
        league.play.assert_not_called()
        league.evaluate.assert_called_once_with("confirmation")


if __name__ == "__main__":
    unittest.main()
