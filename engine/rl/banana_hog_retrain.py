"""Conservative three-hour Hog1 fine-tuning, without changing actor inputs.

The pretrained representation and placement networks stay bit-identical. Only
card scoring, waiting and the detached critic learn. Native tower HP is used
solely for potential-based reward shaping; it never enters the actor or critic.
"""
from __future__ import annotations

import argparse
import concurrent.futures as futures
import copy
import datetime as dt
import json
import os
import random
import signal
import time
import uuid
from contextlib import contextmanager
from pathlib import Path

import torch
from torch.nn import functional as F

from . import banana_hog_league as core
from .banana_fast_policy import FastBananaPolicy
from .banana_tower_hp import TowerHPPolicy
from .banana_policy import BananaEpisode
from .banana_train import ROOT, atomic_json, sha256, utc_now


def validate_resume_config(saved, args):
    if bool(saved.get("args", {}).get("tower_hp", False)) != bool(getattr(args, "tower_hp", False)):
        raise ValueError("resume cannot change experiment setting tower_hp")
    fields = ("seed", "workers", "eval_seeds", "match_ticks", "duration_seconds", "learning_rate",
              "critic_learning_rate", "shaping_coefficient", "gae_lambda", "card_anchor", "wait_anchor",
              "target_kl", "max_parent_kl", "entropy_coef", "regression_threshold", "epochs", "minibatch_size")
    for key in fields:
        if key in saved.get("args", {}) and saved["args"][key] != getattr(args, key):
            raise ValueError(f"resume cannot change experiment setting {key}")


def tower_potential(state, seat):
    towers = state.get("towers", [])
    if len(towers) != 6 or {row["side"] for row in towers} != {0, 1}:
        raise ValueError("reward state must retain the six starting native towers")
    ratios = []
    for side in (0, 1):
        rows = [r for r in towers if r["side"] == side]
        if len(rows) != 3 or any(r["max_hp"] <= 0 or r["hp"] < 0 or r["hp"] > r["max_hp"] for r in rows):
            raise ValueError("invalid native tower HP; refusing guessed damage")
        ratios.append(sum(r["hp"] for r in rows) / sum(r["max_hp"] for r in rows))
    return ratios[seat] - ratios[1 - seat]


def shaped_rewards(potentials, outcome, coefficient):
    # Terminal potential is zero, so undiscounted shaped return differs from
    # the true W/L/D return only by an action-independent initial-state term.
    states = potentials[:-1] + [0.]
    result = [coefficient * (b - a) for a, b in zip(states, states[1:])]
    result[-1] += outcome
    if abs(sum(result) + coefficient * potentials[0] - outcome) > 1e-6:
        raise RuntimeError("potential reward changed the win/loss objective")
    return result


def gae_returns(rewards, values, lam=.97):
    advantages = [0.] * len(values)
    running = 0.
    for i in reversed(range(len(values))):
        next_value = values[i + 1] if i + 1 < len(values) else 0.
        running = rewards[i] + next_value - values[i] + lam * running
        advantages[i] = running
    return advantages, [a + v for a, v in zip(advantages, values)]


class StableHogPolicy(TowerHPPolicy):
    """Same architecture/observations/decoder, with a frozen pretrained body."""
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        for p in self.model.parameters():
            p.requires_grad_(False)
        heads = [self.model.slot_scorer, self.wait_head, self.critic]
        if self.tower_hp_enabled:
            heads.append(self.model.tower_hp_adapter)
        for head in heads:
            for p in head.parameters():
                p.requires_grad_(True)
        self.eval()

    @contextmanager
    def _ppo_backend(self):
        # No GRU backward is needed: the entire pretrained representation is
        # frozen. Card/wait/value heads have their own differentiable graphs.
        yield


class MatchedSampler(core.Sampler):
    def __init__(self, decks, seed):
        super().__init__(decks, seed)
        self.cells = [cell for cell in self.cells if cell[1] in core.EVAL_CONTROLLERS]
        self.results = {cell: self.results[cell] for cell in self.cells}
        self.warm = self.cells[:]
        self.rng.shuffle(self.warm)


class StableLeague(core.League):
    def __init__(self, args):
        self.guard = {"frozen_backbone": True, "frozen_placement": True, "anchor_kl": None,
                      "rollback_count": 0, "evaluation_rollbacks": 0, "anchor_rollbacks": 0}
        self.trace_stats = {"terminal_return_checks": 0, "shaping_steps": 0, "wait_decisions": 0,
                            "voluntary_waits": 0, "controllable_decisions": 0}
        self.last_eval_updates = 0
        super().__init__(args)
        self.frozen_body = {name: value.detach().cpu().clone() for name, value in self.learner.model.state_dict().items()
                            if not name.startswith(("slot_scorer.", "tower_hp_adapter."))}
        self.guard.update(actor_parameters=sum(p.numel() for p in self.actor_parameters()),
                          critic_parameters=sum(p.numel() for p in self.learner.critic.parameters()))
        self.publish(force=True)

    def make_sampler(self, decks, seed):
        return MatchedSampler(decks, seed)

    def make_learner(self, **kwargs):
        return StableHogPolicy(tower_hp=getattr(self.args, "tower_hp", False), **kwargs)

    def actor_parameters(self):
        parameters = [*self.learner.model.slot_scorer.parameters(), *self.learner.wait_head.parameters()]
        if self.learner.tower_hp_enabled:
            parameters.extend(self.learner.model.tower_hp_adapter.parameters())
        return parameters

    def make_optimizer(self):
        return torch.optim.Adam([
            {"params": self.actor_parameters(), "lr": self.args.learning_rate, "role": "actor"},
            {"params": self.learner.critic.parameters(), "lr": self.args.critic_learning_rate, "role": "critic"},
        ])

    def manifest_extra(self):
        metadata = {"format": "banana-hog-retrain-v2", "duration_seconds": self.args.duration_seconds,
                "retrain_trainer_sha256": sha256(Path(__file__)),
                "controllers": list(core.EVAL_CONTROLLERS),
                "selection_seed_range": [2_100_000_000, 2_100_000_000 + self.args.eval_seeds - 1],
                "confirmation_seed_range": [3_100_000_000, 3_100_000_000 + self.args.eval_seeds - 1],
                "reward": "Native W/L/D ±1/0 plus terminal-zero tower-HP potential differences (gamma=1); shaped episode return is exactly outcome minus its constant starting potential. GAE lambda=.97. No card-specific bonuses.",
                "experiment": {"description": "Three-hour conservative fine-tune from original banana-hog-1",
                    "observation_unchanged": True,
                    "training_changes": ["Frozen pretrained encoder, embeddings, placement and timing heads",
                        "Only card scoring, wait decision and detached critic learn",
                        "Exact KL penalty to the original parent on card/wait distributions",
                        "Native tower HP supplies reward feedback only; no new model inputs",
                        "Potential shaping preserves the terminal win/loss objective",
                        "GAE advantages, equal episode weights, and controlled-action normalization",
                        "Training opponents match evaluation; separate greedy and sampled diagnostics",
                        "Automatic rollback and smaller actor learning rate after large evaluation regressions",
                        "Portable tensor/plain-data checkpoints, disjoint selection/confirmation seeds"],
                    "notes": ["Frozen opponents are bots, not expert deck pilots or ladder players.",
                              "The original model and the failed overnight artifacts are preserved.",
                              "Improvement is a measured outcome, not a promised result."]}}

        if getattr(self.args, "tower_hp", False):
            metadata["format"] = "banana-hog-retrain-tower-hp-v1"
            metadata["tower_hp_source_sha256"] = sha256(ROOT / "rl/banana_tower_hp.py")
            metadata["observation_contract"] = "Existing causal inputs plus own/enemy king, weaker princess, stronger princess HP fractions; no unit positions or lane identity."
            experiment = metadata["experiment"]
            experiment["observation_unchanged"] = False
            experiment["description"] = "Fine-tune with zero-initialized tower-health residuals"
            experiment["training_changes"][1] = "Card scoring, waiting, detached critic and tower-health residual branch learn"
            experiment["training_changes"][3] = "Native tower HP enters card/wait/value inputs and potential rewards; placement stays unchanged"
        return metadata

    def extra_state(self):
        return {"duration_seconds": self.args.duration_seconds, "learning_guard": self.guard,
                "experiment": self.manifest_extra()["experiment"], "training_diagnostics": self.trace_stats,
                "notes": (["Tower HP feeds card choice, waiting and value; placement stays unchanged."] if getattr(self.args, "tower_hp", False) else ["Actor inputs and architecture are unchanged from Hog1.", "Native tower health is used only to construct rewards, never as model input."]) + [
                          "W/L/D metrics always use native terminal outcomes; VOID is excluded.",
                          "Miner Wall Breakers and Giant Double Prince are evaluation only.",
                          "Checkpoint selection uses greedy results; sampled-policy results are reported separately.",
                          "Final confirmation uses a separate seed bank and cannot restart learning."]}

    def runtime_extra(self):
        return {"learning_guard": self.guard, "trace_stats": self.trace_stats,
                "last_eval_updates": self.last_eval_updates}

    def restore_extra(self, runtime):
        self.guard.update(runtime.get("learning_guard", {}))
        self.trace_stats.update(runtime.get("trace_stats", {}))
        self.last_eval_updates = runtime.get("last_eval_updates", 0)

    def restore(self, path):
        manifest = self.run_dir / "manifest.json"
        if manifest.exists():
            saved = json.loads(manifest.read_text())
            validate_resume_config(saved, self.args)
            if getattr(self.args, "tower_hp", False) and saved.get("tower_hp_source_sha256") != sha256(ROOT / "rl/banana_tower_hp.py"):
                raise RuntimeError("tower HP policy source changed")
            expected = saved.get("retrain_trainer_sha256")
            if expected and expected != sha256(Path(__file__)):
                raise RuntimeError("retrain source changed; refusing an unrecorded experiment change")
        super().restore(path)

    def reset(self, worker, job):
        deck = self.decks[job["deck_id"]]
        native = [self.hog["native_cards"], deck["native_cards"]]
        slugs = [tuple(self.hog["cards"]), tuple(deck["cards"])]
        if job["seat"] == 1:
            native.reverse()
            slugs.reverse()
        reset = self.clients[worker].request({"op": "reset", "deck0": native[0], "deck1": native[1],
            "seed": job["seed"], "max_ticks": self.args.match_ticks + 100, "reward_state": True})
        self.verify_decks(reset, slugs)
        warm = self.clients[worker].request({"op": "step", "actions": [], "ticks": 100})
        if warm["ticks_advanced"] != 100:
            raise RuntimeError("native_warmup_failed")
        potential = tower_potential(warm["reward_state"], job["seat"])
        warm["observation"]["tower_state"] = warm["reward_state"]
        game = core.Game(worker, job, BananaEpisode(tuple(slugs), battle_id=uuid.uuid4().hex),
                         warm["observation"], int(warm["observation"]["tick"]))
        game.potentials = [potential]
        return game

    def learner_actions(self, policy, requests, games, collecting):
        mode = games[0].job.get("learner_mode", "greedy")
        if any(game.job.get("learner_mode", "greedy") != mode for game in games):
            raise ValueError("mixed learner modes in one native wave")
        return policy.act_batch(requests, deterministic=not collecting and mode == "greedy")

    def after_native_step(self, game, reply):
        potential = tower_potential(reply["reward_state"], game.job["seat"])
        game.potentials.append(potential)
        action = game.decisions[-1]
        self.trace_stats["wait_decisions"] += int(action.wait)
        self.trace_stats["controllable_decisions"] += int(action.input_cache["can_play"])
        self.trace_stats["voluntary_waits"] += int(action.wait and action.input_cache["can_play"])

    def game_transitions(self, game, outcome):
        if len(game.potentials) != len(game.decisions) + 1:
            raise RuntimeError("reward trajectory does not match decisions")
        rewards = shaped_rewards(game.potentials, outcome, self.args.shaping_coefficient)
        values = [float(action.value) for action in game.decisions]
        advantages, returns = gae_returns(rewards, values, self.args.gae_lambda)
        self.trace_stats["terminal_return_checks"] += 1
        self.trace_stats["shaping_steps"] += sum(abs(r) > 1e-8 for r in rewards[:-1])
        # Save a compact trajectory audit for every native terminal match.
        with (self.run_dir / "reward-audit.jsonl").open("a") as out:
            out.write(json.dumps({"id": game.episode.battle_id, "outcome": outcome,
                "initial_potential": game.potentials[0], "sum_rewards": sum(rewards),
                "decisions": len(values), "hog_plays": sum(e["card"] == "hog-rider" and e["side"] == ("team" if game.job["seat"] == 0 else "opponent") for e in game.episode.events),
                "nonterminal_damage_signals": sum(abs(r) > 1e-8 for r in rewards[:-1])}) + "\n")
        weight = 1. / len(values)
        return [{"action": action, "advantage": adv, "return": ret, "weight": weight}
                for action, adv, ret in zip(game.decisions, advantages, returns)]

    def anchor_terms(self, actions):
        caches = [action.input_cache for action in actions]
        with torch.no_grad():
            parent = self.parent_policy._forward_batch(caches)
            parent_wait = self.parent_policy.wait_head(parent["fused"]).squeeze(-1)
        with self.learner._ppo_backend():
            current = self.learner._forward_batch(caches)
        if self.guard.get("frozen_backbone", True) and any(
                p.requires_grad for name, p in self.learner.model.named_parameters()
                if not name.startswith(("slot_scorer.", "tower_hp_adapter."))):
            raise RuntimeError("critic/actor gradient can reach the frozen representation")
        wait = self.learner.wait_head(current["fused"]).squeeze(-1)
        p = torch.sigmoid(parent_wait)
        wait_kl = p * (F.logsigmoid(parent_wait) - F.logsigmoid(wait)) + (1-p) * (F.logsigmoid(-parent_wait) - F.logsigmoid(-wait))
        mask = torch.stack([cache["inputs"][7] for cache in caches]).to(self.learner.device)
        temperature = wait.new_tensor([cache["temperature"] for cache in caches])[:, None]
        old = (parent["slot_logits"] / temperature).masked_fill(~mask, -1e9).log_softmax(-1)
        new = (current["slot_logits"] / temperature).masked_fill(~mask, -1e9).log_softmax(-1)
        card_kl = (old.exp() * (old - new)).sum(-1)
        valid = mask.any(-1).float()
        return card_kl.clamp_min(0) * valid, wait_kl.clamp_min(0) * valid

    def audit_frozen_body(self):
        for name, value in self.learner.model.state_dict().items():
            if name in self.frozen_body and not torch.equal(value.detach().cpu(), self.frozen_body[name]):
                raise RuntimeError(f"pretrained frozen parameter changed: {name}")

    def update(self, transitions):
        if len(transitions) < 8 or self.stopping():
            return
        self.phase = "PPO · configured trainable layers · parent KL · GAE"
        self.publish(force=True)
        actions = [row["action"] for row in transitions]
        device = self.learner.device
        old = torch.stack([a.log_prob for a in actions]).to(device)
        targets = torch.tensor([row["return"] for row in transitions], device=device)
        advantages = torch.tensor([row["advantage"] for row in transitions], device=device)
        weights = torch.tensor([row["weight"] for row in transitions], device=device)
        controlled = torch.tensor([a.input_cache["can_play"] for a in actions], device=device).float()
        actor_weights = weights * controlled
        if actor_weights.sum() <= 0:
            return
        mean = (advantages * actor_weights).sum() / actor_weights.sum()
        variance = ((advantages - mean).square() * actor_weights).sum() / actor_weights.sum()
        advantages = ((advantages - mean) / variance.sqrt().clamp_min(.05)).clamp(-5, 5)
        before = {"model": copy.deepcopy(self.learner.model.state_dict()),
                  "wait": copy.deepcopy(self.learner.wait_head.state_dict()),
                  "critic": copy.deepcopy(self.learner.critic.state_dict()),
                  "optimizer": copy.deepcopy(self.optimizer.state_dict())}
        if self.learner.tower_hp_enabled:
            before["tower_hp"] = copy.deepcopy(self.learner.model.tower_hp_adapter.state_dict())
        logs = []
        early = False
        for _epoch in range(self.args.epochs):
            order = torch.randperm(len(actions)).tolist()
            for start in range(0, len(actions), self.args.minibatch_size):
                if self.stopping():
                    break
                ids = order[start:start + self.args.minibatch_size]
                ix = torch.tensor(ids, device=device)
                rows = [actions[i] for i in ids]
                logp, entropy, value = self.learner.evaluate_actions(rows)
                ratio = (logp - old[ix]).exp()
                aw, vw = actor_weights[ix], weights[ix]
                avg = lambda x, w: (x * w).sum() / w.sum().clamp_min(1e-9)
                kl = avg((ratio - 1) - (logp - old[ix]), aw)
                if logs and float(kl.detach()) > self.args.target_kl:
                    early = True
                    break
                actor = avg(-torch.minimum(ratio * advantages[ix], ratio.clamp(.9, 1.1) * advantages[ix]), aw)
                critic = avg((value - targets[ix]).square(), vw)
                card_kl, wait_kl = self.anchor_terms(rows)
                anchor = self.args.card_anchor * avg(card_kl, aw) + self.args.wait_anchor * avg(wait_kl, aw)
                ent = avg(entropy, aw)
                loss = actor + .5 * critic + anchor - self.args.entropy_coef * ent
                if not torch.isfinite(loss):
                    raise RuntimeError("nonfinite stable PPO loss")
                self.optimizer.zero_grad(set_to_none=True)
                loss.backward()
                norm = torch.nn.utils.clip_grad_norm_([p for group in self.optimizer.param_groups for p in group["params"]], .5, error_if_nonfinite=True)
                self.optimizer.step()
                logs.append({"loss": float(loss.detach()), "policy_loss": float(actor.detach()),
                    "value_loss": float(critic.detach()), "entropy": float(ent.detach()),
                    "approx_kl": float(kl.detach()), "parent_card_kl": float(avg(card_kl, aw).detach()),
                    "parent_wait_kl": float(avg(wait_kl, aw).detach()),
                    "clip_fraction": float(avg(((ratio - 1).abs() > .1).float(), aw).detach()), "grad_norm": float(norm)})
                self.publish()
            if early or self.stopping():
                break
        if not logs:
            return
        probe = actions[::max(1, len(actions)//256)][:256]
        with torch.no_grad():
            card, wait = self.anchor_terms(probe)
            valid = torch.tensor([a.input_cache["can_play"] for a in probe], device=device).float()
            parent_kl = float(((card + wait) * valid).sum() / valid.sum().clamp_min(1))
        if parent_kl > self.args.max_parent_kl:
            self.learner.model.load_state_dict(before["model"])
            self.learner.wait_head.load_state_dict(before["wait"])
            self.learner.critic.load_state_dict(before["critic"])
            if self.learner.tower_hp_enabled:
                self.learner.model.tower_hp_adapter.load_state_dict(before["tower_hp"])
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
        self.ppo.update(samples=len(actions), minibatches=len(logs),
                        learning_rate=self.optimizer.param_groups[0]["lr"], kl_early_stop=early,
                        advantage_std=float(variance.sqrt()), parent_kl=parent_kl)
        with (self.run_dir / "updates.jsonl").open("a") as out:
            out.write(json.dumps({"at": utc_now(), "updates": self.version, **self.ppo}) + "\n")
        self.log(f"Stable PPO {self.version}: {len(actions)} decisions · parent KL {parent_kl:.5f} · trainability verified")
        self.save("latest")

    def lower_actor_lr(self):
        self.optimizer.param_groups[0]["lr"] = max(1e-6, self.optimizer.param_groups[0]["lr"] * .5)

    def eval_jobs(self, bank):
        base = 2_100_000_000 if bank == "selection" else 3_100_000_000
        return [{"deck_id": deck["id"], "controller": controller, "seat": seat, "seed": base + seed}
                for seed in range(self.args.eval_seeds) for deck in self.pool
                for controller in core.EVAL_CONTROLLERS for seat in (0, 1)]

    def evaluate(self, bank="selection", parent=False):
        complete = super().evaluate(bank, parent)
        if bank == "selection" and not parent and complete:
            summary = self.evaluation["latest"]
            if summary["delta"] is not None and summary["delta"] < -self.args.regression_threshold:
                old_lr = self.optimizer.param_groups[0]["lr"]
                best = torch.load(self.run_dir / "models/best.pt", map_location=self.args.device, weights_only=False)
                self.load_weights(best)
                self.optimizer.param_groups[0]["lr"] = max(1e-6, old_lr * .5)
                self.guard["rollback_count"] += 1
                self.guard["evaluation_rollbacks"] += 1
                self.log(f"Evaluation regression {summary['delta']:+.1%}: restored best and halved actor LR")
                self.save("latest")
        self.publish(force=True)
        return complete

    def load_weights(self, payload):
        self.learner.model.load_state_dict(payload["model_state"])
        self.learner.critic.load_state_dict(payload["critic_state"])
        self.learner.wait_head.load_state_dict(payload["wait_state"])
        self.optimizer.load_state_dict(payload["optimizer_state"])
        self.audit_frozen_body()

    def save_selected_final(self):
        # A hard deadline or stop during evaluation must still deploy the best
        # selected snapshot, never silently relabel a regressed last iterate.
        selected = torch.load(self.run_dir / "models/best.pt", map_location=self.args.device, weights_only=False)
        self.load_weights(selected)
        self.version = selected["updates"]
        self.evaluation["final_artifact"] = {"source": "best.pt", "updates": self.version,
            "confirmation_complete": bool((self.evaluation.get("confirmation") or {}).get("complete"))}
        self.save("final")

    def sampling_diagnostic(self):
        self.status, self.phase = "evaluating", "sampled-versus-greedy diagnostic · separate from selection"
        jobs = [{"deck_id": deck["id"], "controller": "bc-greedy", "seat": side,
                 "seed": 1_500_000_000, "learner_mode": "sampled"}
                for deck in self.pool for side in (0, 1)]
        scores = {}
        with torch.random.fork_rng(devices=[self.learner.device] if self.learner.device.type == "cuda" else []):
            for name, policy in (("parent", self.parent_policy), ("candidate", self.learner)):
                torch.manual_seed(self.args.seed + 76543)
                rows, _ = self.play(jobs, policy, f"evaluation-sampled-{name}")
                scores[name] = {**core.metric(rows), "rows": rows}
        self.evaluation["sampled_diagnostic"] = {"at": utc_now(), "checkpoint": self.version, **scores}
        self.status = "running"
        self.save("latest")
        self.publish(force=True)

    def final_evaluation_reserve(self):
        return max(600., 3.3 * self.eval_seconds + 60)

    def may_start_selection(self):
        return True

    def run(self):
        try:
            finalized = self.evaluation.get("confirmation") is not None
            self.eval_seconds = float(self.evaluation.get("measured_bank_seconds", 240.))
            self.last_eval = float(self.evaluation.get("last_selection_epoch", time.time()))
            while not self.stopping() and not finalized:
                # Final selection + fresh-parent confirmation + candidate
                # confirmation, with a small save/cleanup allowance.
                reserve = self.final_evaluation_reserve()
                if "selection" in self.evaluation["baseline"] and self.deadline - time.time() <= reserve:
                    break
                self.status, self.phase = "running", f"training · stable heads · wave {self.version}"
                jobs = [self.sampler.draw() for _ in range(self.args.workers)]
                rows, transitions = self.play(jobs, self.learner, "train", collecting=True)
                if not any(r["result"] != "VOID" for r in rows) and not self.stopping():
                    raise RuntimeError("whole native training wave VOID")
                self.update(transitions)
                self.save("latest")
                self.publish(force=True)
                if self.args.max_waves and self.counters["updates"] >= self.args.max_waves:
                    break
                if self.args.max_waves:
                    continue
                if "selection" not in self.evaluation["baseline"] and not self.stopping():
                    self.evaluate("selection", parent=True)
                    if not self.stopping():
                        self.evaluate()
                    if not self.stopping():
                        self.sampling_diagnostic()
                    self.last_eval = time.time()
                elif time.time() - self.last_eval >= self.args.eval_every and not self.stopping() and self.may_start_selection():
                    self.evaluate()
                    self.last_eval = time.time()
                self.evaluation["measured_bank_seconds"] = self.eval_seconds
                self.evaluation["last_selection_epoch"] = self.last_eval
            if not self.stopping() and not self.args.max_waves and not finalized:
                self.save("last-training")
                self.evaluate()
                best = torch.load(self.run_dir / "models/best.pt", map_location=self.args.device, weights_only=False)
                self.load_weights(best)
                self.version = best["updates"]
                if not self.stopping() and "confirmation" not in self.evaluation["baseline"]:
                    self.evaluate("confirmation", parent=True)
                if not self.stopping():
                    self.evaluate("confirmation")
                self.save_selected_final()
                finalized = True
            if finalized:
                self.status, self.phase = "running", "final model confirmed · waiting for three-hour deadline"
                while not self.stopping():
                    self.publish()
                    time.sleep(2)
            self.status = "completed" if time.time() >= self.deadline or self.args.max_waves else "stopped"
            self.phase = "three-hour experiment finished · checkpoints saved"
        except BaseException as exc:
            self.status, self.phase = "failed", f"{type(exc).__name__}: {exc}"
            self.errors[type(exc).__name__] += 1
            self.log(self.phase)
            raise
        finally:
            self.save("latest")
            if self.args.max_waves:
                self.save("pilot")
            elif self.status in ("completed", "stopped"):
                self.save_selected_final()
            self.publish(force=True)
            for client in self.clients:
                client.close()
            self.executor.shutdown(wait=True, cancel_futures=True)


def main(league_class=StableLeague, configure_parser=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--tower-hp", action="store_true", help="Add zero-initialized native tower-HP inputs; use a fresh run directory")
    parser.add_argument("--name", default="banana-hog-1-r2")
    parser.add_argument("--parent", type=Path, default=ROOT / "runs/banana-league-20261019/models/hog/final.pt")
    parser.add_argument("--pool", type=Path, default=ROOT / "data/hog2_opponent_pool.json")
    parser.add_argument("--engine", type=Path, required=True)
    parser.add_argument("--checkpoint", type=Path, default=ROOT / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state")
    parser.add_argument("--duration-seconds", type=float, default=10800)
    parser.add_argument("--deadline", help="Persisted absolute UTC end; otherwise fresh run starts a three-hour budget")
    parser.add_argument("--workers", type=int, default=10)
    parser.add_argument("--device", default="cuda")
    parser.add_argument("--torch-threads", type=int, default=2)
    parser.add_argument("--learning-rate", type=float, default=3e-5)
    parser.add_argument("--critic-learning-rate", type=float, default=3e-4)
    parser.add_argument("--target-kl", type=float, default=.01)
    parser.add_argument("--max-parent-kl", type=float, default=.20)
    parser.add_argument("--card-anchor", type=float, default=.1)
    parser.add_argument("--wait-anchor", type=float, default=.02)
    parser.add_argument("--shaping-coefficient", type=float, default=.5)
    parser.add_argument("--gae-lambda", type=float, default=.97)
    parser.add_argument("--entropy-coef", type=float, default=.002)
    parser.add_argument("--regression-threshold", type=float, default=.08)
    parser.add_argument("--minibatch-size", type=int, default=128)
    parser.add_argument("--epochs", type=int, default=2)
    parser.add_argument("--match-ticks", type=int, default=7300)
    parser.add_argument("--eval-every", type=float, default=1200)
    parser.add_argument("--eval-seeds", type=int, default=2)
    parser.add_argument("--seed", type=int, default=20261020)
    parser.add_argument("--resume", action="store_true")
    parser.add_argument("--max-waves", type=int, default=0)
    if configure_parser is not None:
        configure_parser(parser)
    args = parser.parse_args()
    manifest = args.run_dir / "manifest.json"
    if args.resume and manifest.exists():
        saved = json.loads(manifest.read_text())
        args.deadline = args.deadline or saved["deadline"]
        if saved["name"] != args.name:
            parser.error("resume name differs from existing run")
    if not args.deadline:
        args.deadline = dt.datetime.fromtimestamp(time.time() + args.duration_seconds, dt.timezone.utc).isoformat()
    if core.deadline_epoch(args.deadline) <= time.time():
        print("Three-hour deadline passed; declining to restart training.", flush=True)
        return
    if args.workers < 1 or args.eval_seeds < 1 or args.duration_seconds <= 0:
        parser.error("workers, eval seeds, and duration must be positive")
    def stop(_signal, _frame):
        core.STOPPING = True
    signal.signal(signal.SIGINT, stop)
    signal.signal(signal.SIGTERM, stop)
    league_class(args).run()


if __name__ == "__main__":
    main()
