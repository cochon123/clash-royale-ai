"""Tower-aware PPO with trainable placement/body and matched sampled evaluation.

Fresh experiments only. Defaults: all actor layers train, body LR 10x smaller,
the full 576-tile placement grid, sampled selection plus greedy checks, 2pp
regression tolerance measured against the selected best in either mode.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import random
import time
from collections import deque

import torch

from . import banana_hog_league as core
from .banana_fast_policy import FastBananaPolicy
from .banana_hog_retrain import StableLeague, main as stable_main
from .banana_tower_hp import TowerHPPolicy
from .banana_train import ROOT, sha256, utc_now


def sampling_cache(policy, episode, side, now_seconds, cache):
    # No shared/global RNG: a match's actions cannot change when another
    # worker finishes early. Parent and candidate use common random numbers.
    seed = getattr(episode, "policy_seed", getattr(policy, "sampling_seed", 0))
    raw = f"{seed}:{side}:{round(now_seconds * 20)}".encode()
    rng = random.Random(int.from_bytes(hashlib.blake2b(raw, digest_size=8).digest(), "little"))
    cache["sample_uniforms"] = tuple(rng.random() for _ in range(3))
    return cache


class SamplingPolicy(FastBananaPolicy):
    @staticmethod
    def _cache_request(policy, episode, observation, side, now_seconds, *args, **kwargs):
        cache = FastBananaPolicy._cache_request(policy, episode, observation, side, now_seconds, *args, **kwargs)
        return sampling_cache(policy, episode, side, now_seconds, cache)


class AdaptiveHogPolicy(TowerHPPolicy):
    def __init__(self, *args, trainable="all", **kwargs):
        super().__init__(*args, tower_hp=True, **kwargs)
        if trainable not in ("all", "placement", "heads"):
            raise ValueError("invalid trainable setting")
        self.trainable = trainable
        for name, parameter in self.model.named_parameters():
            heads = name.startswith(("slot_scorer.", "tower_hp_adapter."))
            placement = name.startswith(("zone_head.", "tile_head.", "xy_head."))
            parameter.requires_grad_(trainable == "all" or heads or (trainable == "placement" and placement))
        for parameter in (*self.wait_head.parameters(), *self.critic.parameters()):
            parameter.requires_grad_(True)
        if (not self.model.card_conditioned_placement or self.model.placement_mode != "heatmap"
                or self.model.placement_card_mode != "selected" or getattr(self.model, "arena_memory_channels", 0)):
            raise ValueError("adaptive placement requires the selected-card heatmap architecture")

    _ppo_backend = FastBananaPolicy._ppo_backend

    @staticmethod
    def _cache_request(policy, episode, observation, side, now_seconds, *args, **kwargs):
        cache = TowerHPPolicy._cache_request(policy, episode, observation, side, now_seconds, *args, **kwargs)
        return sampling_cache(policy, episode, side, now_seconds, cache)

    def _placement_from_context(self, first, caches, indices, slots):
        # Unlike the conservative experiment, HP can affect placement too.
        return FastBananaPolicy._placement_from_context(self, first, caches, indices, slots)


def regression_reason(sampled, greedy, best_sampled, best_greedy, tolerance=.02):
    for mode, current, best in (("sampled", sampled, best_sampled), ("greedy", greedy, best_greedy)):
        if current is None or best is None:
            continue
        gap = current - best
        if gap <= -tolerance:
            return f"{mode} score declined {gap:+.2%} versus selected best"
    return None


class AdaptiveLeague(StableLeague):
    def __init__(self, args):
        super().__init__(args)
        self.guard.update(frozen_backbone=args.trainable != "all", frozen_placement=args.trainable == "heads")
        self.publish(force=True)

    @property
    def evaluation_controllers(self):
        return ("bc-greedy", "bc-sampled") if getattr(self.args, "opponents", "legacy") == "original" else core.EVAL_CONTROLLERS

    def make_sampler(self, decks, seed):
        sampler = core.Sampler(decks, seed)
        sampler.cells = [(d["id"], c) for d in decks if not d["holdout"] for c in self.evaluation_controllers]
        sampler.results = {cell: deque(maxlen=200) for cell in sampler.cells}
        sampler.warm = sampler.cells[:]
        sampler.rng.shuffle(sampler.warm)
        return sampler

    def make_base_policy(self, **kwargs):
        return SamplingPolicy(**kwargs)

    def make_learner(self, **kwargs):
        return AdaptiveHogPolicy(trainable=self.args.trainable, **kwargs)

    def make_parent_policy(self, **kwargs):
        return SamplingPolicy(**kwargs)

    def actor_parameters(self):
        return [p for p in self.learner.model.parameters() if p.requires_grad] + list(self.learner.wait_head.parameters())

    def make_optimizer(self):
        body, heads = [], list(self.learner.wait_head.parameters())
        for name, parameter in self.learner.model.named_parameters():
            if parameter.requires_grad:
                (heads if name.startswith(("slot_scorer.", "tower_hp_adapter.", "zone_head.", "tile_head.", "xy_head.")) else body).append(parameter)
        groups = [{"params": heads, "lr": self.args.learning_rate, "role": "actor"}]
        if body:
            groups.append({"params": body, "lr": self.args.backbone_learning_rate, "role": "backbone"})
        groups.append({"params": self.learner.critic.parameters(), "lr": self.args.critic_learning_rate, "role": "critic"})
        return torch.optim.Adam(groups)

    def audit_frozen_body(self):
        # Only audit parameters intentionally frozen in this experiment.
        frozen = {name for name, p in self.learner.model.named_parameters() if not p.requires_grad}
        for name, value in self.learner.model.state_dict().items():
            if name in frozen and name in self.frozen_body and not torch.equal(value.detach().cpu(), self.frozen_body[name]):
                raise RuntimeError(f"frozen parameter changed: {name}")

    def anchor_terms(self, actions):
        # StableLeague's assertion concerns the old heads-only experiment.
        # Here encoder/placement gradients are explicitly authorized.
        frozen = self.guard.get("frozen_backbone", True)
        self.guard["frozen_backbone"] = False
        try:
            return super().anchor_terms(actions)
        finally:
            self.guard["frozen_backbone"] = frozen

    def lower_actor_lr(self):
        for group in self.optimizer.param_groups:
            if group["role"] != "critic":
                group["lr"] = max(1e-7, group["lr"] * .5)

    def reset(self, worker, job):
        game = super().reset(worker, job)
        game.episode.policy_seed = job["seed"]
        return game

    def learner_actions(self, policy, requests, games, collecting):
        mode = "sampled" if collecting else games[0].job.get("learner_mode", "sampled")
        if any(game.job.get("learner_mode", mode) != mode for game in games):
            raise ValueError("mixed action-selection modes")
        # Training placement on a changing top-k support invalidates PPO's
        # likelihood ratio. Use the entire grid in training AND testing. Native action
        # validation remains authoritative; optional per-card masks come
        # from the rebuilt original-native target validator.
        kwargs = {}
        if getattr(getattr(self, "args", None), "native_placement_masks", False):
            kwargs["placement_mask_provider"] = self.mask_provider(games)
        return policy.act_batch(requests, deterministic=mode == "greedy", placement_top_k=576, **kwargs)

    def mask_provider(self, games):
        workers = {id(game.episode): game.worker for game in games}
        def query(request, native_slot):
            return self.clients[workers[id(request[0])]].request(
                {"op": "placement_mask", "side": request[2], "slot": native_slot})["mask"]
        return query

    def opponent_actions(self, policy, requests, games, controller):
        kwargs = {}
        if getattr(self.args, "native_placement_masks", False):
            kwargs["placement_mask_provider"] = self.mask_provider(games)
        return policy.act_batch(requests, deterministic=controller != "bc-sampled", **kwargs)

    def eval_jobs(self, bank):
        mode = "greedy" if bank.endswith("-greedy") else "sampled"
        base = 2_200_000_000 if bank.startswith("selection") else 3_300_000_000
        seeds = getattr(self.args, "confirmation_seeds", self.args.eval_seeds) if bank.startswith("confirmation") else self.args.eval_seeds
        return [dict(deck_id=deck["id"], controller=controller, seat=seat, seed=base + seed, learner_mode=mode)
                for seed in range(seeds) for deck in self.pool
                for controller in self.evaluation_controllers for seat in (0, 1)]

    def final_evaluation_reserve(self):
        selection_seconds = self.evaluation.get("selection_seconds", 480.)
        ratio = getattr(self.args, "confirmation_seeds", self.args.eval_seeds) / self.args.eval_seeds
        return max(600., 1.1 * selection_seconds * (1 + 2 * ratio) + 60)

    def may_start_selection(self):
        return self.deadline - time.time() > self.final_evaluation_reserve() + 1.1 * self.evaluation.get("selection_seconds", 480.)

    @staticmethod
    def weights_digest(policy):
        digest = hashlib.sha256()
        for name, tensor in policy.state_dict().items():
            digest.update(name.encode())
            digest.update(tensor.detach().cpu().contiguous().view(torch.uint8).numpy().tobytes())
        return digest.hexdigest()

    def _retained_rows(self, kind, parent):
        path = self.run_dir / "matches.jsonl"
        rows = {}
        if path.exists():
            for line in path.read_text().splitlines():
                try:
                    row = json.loads(line)
                except json.JSONDecodeError:
                    continue  # A power failure can leave a partial last line.
                if row["kind"] == kind and row["result"] != "VOID":
                    rows[(row["deck_id"], row["controller"], row["seat"], row["seed"])] = row
        return rows

    def evaluate(self, bank="selection", parent=False):
        start = time.monotonic()
        self.status = "evaluating"
        summaries, results = {}, {}
        weights_hash = self.weights_digest(self.parent_policy if parent else self.learner)
        for mode in ("sampled", "greedy"):
            label = bank if mode == "sampled" else bank + "-greedy"
            kind = f"evaluation-{label}-{'parent' if parent else 'candidate'}-{weights_hash[:16]}"
            jobs = self.eval_jobs(label)
            retained = self._retained_rows(kind, parent)
            key = lambda row: (row["deck_id"], row["controller"], row["seat"], row["seed"])
            expected = {key(job) for job in jobs}
            retained = {k: v for k, v in retained.items() if k in expected}
            missing = [job for job in jobs if key(job) not in retained]
            self.phase = f"{bank} · {mode} · {'parent' if parent else 'candidate'} · {len(retained)}/{len(jobs)} retained"
            self.publish(force=True)
            if missing:
                rows, _ = self.play(missing, self.parent_policy if parent else self.learner, kind)
                retained.update({key(row): row for row in rows if row["result"] != "VOID"})
            rows = list(retained.values())
            complete = len(retained) == len(jobs)
            score = core.macro_score(rows)
            results[mode] = rows
            summaries[mode] = dict(at=utc_now(), bank=label, mode=mode, games=len(rows), complete=complete,
                                   score=score, checkpoint=self.version, weights_sha256=weights_hash)
            if parent and complete:
                self.evaluation["baseline"][label] = rows
            elif not parent:
                baseline = self.evaluation["baseline"].get(label, [])
                summaries[mode].update(parent_score=core.macro_score(baseline), **core.paired_delta(rows, baseline))
        complete = all(summary["complete"] for summary in summaries.values())
        if parent and bank == "selection" and complete and self.evaluation["best"] is None:
            self.evaluation["best"] = dict(score=summaries["sampled"]["score"], greedy_score=summaries["greedy"]["score"],
                                           updates=0, source="exact parent fallback", path=str(self.run_dir / "models/best.pt"))
        if not parent:
            summary = dict(summaries["sampled"], complete=complete, greedy=summaries["greedy"])
            self.evaluation["history"].append(summary)
            if bank == "selection":
                self.evaluation.update(latest=summary, latest_rows=results["sampled"])
                best = self.evaluation["best"]
                if complete and best is not None:
                    reason = regression_reason(summary["score"], summaries["greedy"]["score"], best["score"], best.get("greedy_guard_score", best["greedy_score"]), self.args.regression_threshold)
                    if reason:
                        rates = {g["role"]: g["lr"] for g in self.optimizer.param_groups}
                        payload = torch.load(self.run_dir / "models/best.pt", map_location=self.args.device, weights_only=False)
                        self.load_weights(payload)  # Entire model AND optimizer, not just card/wait heads.
                        for group in self.optimizer.param_groups:
                            group["lr"] = rates[group["role"]]
                        self.lower_actor_lr()
                        self.guard["rollback_count"] += 1
                        self.guard["evaluation_rollbacks"] += 1
                        self.log(f"Regression rollback: {reason}; restored full best model/optimizer and halved actor/body rates")
                    elif summary["score"] > best["score"]:
                        self.evaluation["best"] = dict(score=summary["score"], greedy_score=summaries["greedy"]["score"],
                                                       updates=self.version, greedy_guard_score=max(best.get("greedy_guard_score", best["greedy_score"]), summaries["greedy"]["score"]), source="sampled selection with greedy guard", path=str(self.run_dir / "models/best.pt"))
                        self.save("best")
            else:
                self.evaluation["confirmation"] = dict(summary, rows=results["sampled"], greedy_rows=results["greedy"])
        if self.frozen_signature != (self.policy_signature(self.parent_policy), self.policy_signature(self.base)):
            raise RuntimeError("frozen opponent changed")
        elapsed = time.monotonic() - start
        self.eval_seconds = max(self.eval_seconds, elapsed)
        if bank == "selection":
            self.evaluation["selection_seconds"] = max(self.evaluation.get("selection_seconds", 0.), elapsed)
        self.status = "running"
        self.save("latest")
        self.publish(force=True)
        return complete

    def sampling_diagnostic(self):
        pass  # Every selection/confirmation already evaluates both modes.

    def manifest_extra(self):
        metadata = super().manifest_extra()
        metadata.update(controllers=list(self.evaluation_controllers), format="banana-hog-adaptive-v1", adaptive_source_sha256=sha256(ROOT / "rl/banana_hog_adaptive.py"),
                        fast_policy_sha256=sha256(ROOT / "rl/banana_fast_policy.py"),
                        selection_seed_range=[2_200_000_000, 2_200_000_000 + self.args.eval_seeds - 1],
                        confirmation_seed_range=[3_300_000_000, 3_300_000_000 + getattr(self.args, "confirmation_seeds", self.args.eval_seeds) - 1],
                        evaluation_contract="Sampled primary selection and confirmation, with greedy diagnostics/guard; identical full-grid placement support and reproducible per-game action RNG in training/testing.")
        metadata["native_placement_masks"] = getattr(self.args, "native_placement_masks", False)
        metadata["experiment"]["description"] = "Tower-aware training with configurable representation/placement gradients and best-relative regression guard"
        metadata["experiment"]["training_changes"] = [f"Trainable actor layers: {self.args.trainable}; body uses separate lower LR",
            "Tower health residuals feed card selection, wait/value context and placement",
            "Full-grid placement support in rollout, PPO and evaluation; native engine validates legality",
            "Sampled selection/confirmation match training; greedy performance also checked",
            "Full-model/optimizer rollback against selected best if either mode drops by the tolerance",
            "Native potential rewards, GAE and parent card/wait KL retained"]
        return metadata

    def extra_state(self):
        result = super().extra_state()
        result["notes"] = ["Tower HP feeds card/wait/value/placement; actor layers train according to trainable setting.",
                           "Primary evaluation samples actions just as training does; greedy scores are separate.",
                           "Regression checks compare both modes with the selected best; incomplete banks cannot promote a model."]
        return result

    def restore(self, path):
        manifest = self.run_dir / "manifest.json"
        if manifest.exists():
            saved = json.loads(manifest.read_text())
            for key, file in (("adaptive_source_sha256", "banana_hog_adaptive.py"), ("fast_policy_sha256", "banana_fast_policy.py")):
                if saved.get(key) != sha256(ROOT / "rl" / file):
                    raise ValueError(f"adaptive source changed: {file}")
            for key in ("trainable", "backbone_learning_rate", "opponents", "parent_name", "confirmation_seeds", "native_placement_masks"):
                if saved["args"].get(key) != getattr(self.args, key):
                    raise ValueError(f"resume cannot change {key}")
        super().restore(path)


def configure_parser(parser):
    parser.set_defaults(tower_hp=True, name="banana-hog-1", parent=ROOT / "runs/banana-1/base.pt", regression_threshold=.02, eval_every=2700)
    parser.add_argument("--native-placement-masks", action=argparse.BooleanOptionalAction, default=True, help="Condition each selected card on the original native target validator; needs a rebuilt league_reference engine")
    parser.add_argument("--confirmation-seeds", type=int, default=2)
    parser.add_argument("--parent-name", default="banana-1")
    parser.add_argument("--opponents", choices=("legacy", "original"), default="original")
    parser.add_argument("--trainable", choices=("all", "placement", "heads"), default="all")
    parser.add_argument("--backbone-learning-rate", type=float, default=3e-6)


if __name__ == "__main__":
    stable_main(AdaptiveLeague, configure_parser)
