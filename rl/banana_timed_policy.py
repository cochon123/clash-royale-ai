"""Inference scheduler based on clash-royale-ai's two-phone harness.

Choose from the actual hand, schedule the learned inter-event delay, then check
elixir at execution. Plans persist between polls; confirmed events permit a
replan. This is an inference controller, not a PPO Bernoulli-wait policy.
"""
from __future__ import annotations

from dataclasses import dataclass
import math

import torch
from torch.distributions import Categorical

from .banana_policy import SampledAction, _native_card


def ready_time(now, last_event, raw_timing):
    """Match policy_infer's log1p decoder and phone_lab's event-time anchor."""
    if not math.isfinite(raw_timing):
        raise ValueError("nonfinite learned timing")
    delay = min(12., max(.2, math.expm1(min(raw_timing, math.log1p(12.)))))
    return max(now, last_event + delay), delay


@dataclass
class CardPlan:
    card: str
    deck_slot: int
    native_slot: int
    cost: float
    ready_at: float
    delay: float


class PhoneStyleScheduler:
    def __init__(self, policies, *, greedy=False, placement_mask_provider=None, active_sides=(0, 1)):
        self.policies = policies
        self.active_sides = tuple(active_sides)
        if (len(set(self.active_sides)) != len(self.active_sides)
                or any(s not in (0, 1) or policies[s] is None for s in self.active_sides)):
            raise ValueError("active sides require distinct sides with a policy")
        self.greedy = greedy
        self.placement_mask_provider = placement_mask_provider
        self.plans = [None, None]
        self.status = [{}, {}]
        self.revision = None
        self.decode = [p._infer.rollout_decode_settings(p.config) if p is not None else None for p in policies]

    def _cache(self, side, episode, observation, now):
        policy, decode = self.policies[side], self.decode[side]
        top_k = decode["placement_top_k"] or 576
        return policy._cache_request(policy, episode, observation, side, now,
                                     decode["temperature"], decode["placement_temperature"], top_k, None,
                                     require_affordable=False)

    @staticmethod
    def _card(policy, row):
        name = str(row.get("name", ""))
        compact = ''.join(c.lower() for c in name if c.isalnum())
        return (policy.native_id_to_policy.get(int(row.get("card_id", -1)))
                or policy.native_name_to_policy.get(compact) or _native_card(name, policy.vocab.name_to_id))

    def _hand_row(self, side, observation, card):
        return next((row for row in observation["hands"][side]
                     if row.get("available", True) and self._card(self.policies[side], row) == card), None)

    @torch.no_grad()
    def _plan(self, side, episode, observation, now):
        policy, decode = self.policies[side], self.decode[side]
        cache = self._cache(side, episode, observation, now)
        if not cache["legal"]:
            return None
        out = policy._forward_batch([cache])
        mask = cache["inputs"][7].to(policy.device)
        logits = out["slot_logits"][0] / max(float(decode["temperature"]), 1e-4)
        dist = Categorical(logits=logits.masked_fill(~mask, -torch.inf))
        if self.greedy or decode["slot_decode"] == "argmax":
            slot = int(dist.logits.argmax())
        else:
            slot = int(policy._sample_with_uniform(Categorical(logits=dist.logits[None]), [cache], 1)[0])
        native_slot, _, card = next(row for row in cache["legal"] if row[1] == slot)
        row = self._hand_row(side, observation, card)
        last_event = float(episode.events[-1]["seconds"]) if episode.events else 0.
        due, delay = ready_time(now, last_event, float(out["timing"][0]))
        prior = self.plans[side]
        if prior is not None and prior.card == card:
            due = min(due, prior.ready_at)
        return CardPlan(card, slot, native_slot, float(row["cost"]), due, delay)

    @torch.no_grad()
    def _execute(self, side, plan, episode, observation, now):
        policy, decode = self.policies[side], self.decode[side]
        cache = self._cache(side, episode, observation, now)
        native_slot = next(row[0] for row in cache["legal"] if row[1] == plan.deck_slot)
        if self.placement_mask_provider is not None:
            cache["tile_mask"] = torch.as_tensor(
                self.placement_mask_provider((episode, observation, side, now), native_slot), dtype=torch.bool).flatten()
            if cache["tile_mask"].numel() != 576:
                raise ValueError("native placement mask must contain 576 tiles")
            if not cache["tile_mask"].any():
                self.status[side]["reason"] = "no_legal_target"
                return None
        first = policy._forward_batch([cache])
        slots = torch.tensor([plan.deck_slot], device=policy.device)
        second = policy._placement_from_context(first, [cache], [0], slots)
        dist, indices, _ = policy._tile_batch(second["tile_logits"], [cache])
        choice = (dist.logits.argmax(-1) if self.greedy or decode["placement_decode"] == "argmax"
                  else policy._sample_with_uniform(dist, [cache], 2))
        tile = int(indices[0, choice[0]])
        xy = second["tile_logits"].new_tensor([(tile % 32 + .5) / 32, (tile // 32 + .5) / 18])
        if side == 1:
            xy[1] = 1 - xy[1]
        x, y = torch.round(xy * xy.new_tensor([18000., 32000.])).long().tolist()
        zero = torch.tensor(0.)
        self.status[side]["reason"] = "playing"
        return SampledAction(side, native_slot, plan.deck_slot, plan.card, x, y, tile,
                             zero, zero, zero, tuple(row[1] for row in cache["legal"]), input_cache={"inference_only": True})

    def choose(self, episode, observation, now):
        revision = (id(episode), len(episode.events))
        changed = revision != self.revision
        for side in self.active_sides:
            plan = self.plans[side]
            if changed or plan is None or self._hand_row(side, observation, plan.card) is None:
                self.plans[side] = self._plan(side, episode, observation, now)
            plan = self.plans[side]
            mana = float(observation["elixir"][side])
            if plan is None:
                self.status[side] = dict(reason="empty_hand", elixir=mana)
                continue
            row = self._hand_row(side, observation, plan.card)
            plan.cost = float(row["cost"])
            self.status[side] = dict(card=plan.card, cost=plan.cost, elixir=mana,
                                     ready_at=plan.ready_at, predicted_delay=plan.delay,
                                     reason="timing" if now + 1e-6 < plan.ready_at else
                                     "saving_elixir" if mana + 1e-6 < plan.cost else "ready")
        self.revision = revision
        # Like phone_lab: an unaffordable earlier candidate cannot pull the
        # other player's execution ahead of its independently predicted time.
        for side in sorted((s for s in self.active_sides if self.plans[s] is not None), key=lambda s: self.plans[s].ready_at):
            if self.status[side]["reason"] == "ready":
                action = self._execute(side, self.plans[side], episode, observation, now)
                if action is not None:
                    return [action]
        return []

    def observe_result(self, action, result):
        if result.get("accepted"):
            self.plans[action.side] = None
        elif result.get("rejection") != "insufficient_mana":
            self.plans[action.side] = None
