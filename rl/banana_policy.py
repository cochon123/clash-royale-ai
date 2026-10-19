"""Warm-started actor/critic adapter for the original native Clash session.

The actor is loaded from sibling clash-royale-ai policy-bc-v4.4.2. The model
only knows decks, elixir, and causal card-play history; the native observation
currently contains no battlefield entity features. Call ``record_action`` only
for accepted native actions so self-play history stays authoritative.
"""
from __future__ import annotations

import importlib
import json
import random
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Mapping, Sequence

import torch
from torch import Tensor, nn
from torch.distributions import Categorical

ROOT = Path(__file__).resolve().parents[1]
SIBLING = ROOT.parent / "clash-royale-ai"
DEFAULT_CHECKPOINT = SIBLING / "models" / "policy_bc_v4.4.2"


def _pipeline():
    src = str(SIBLING / "src")
    if src not in sys.path:
        sys.path.insert(0, src)
    return (
        importlib.import_module("cr_replay_pipeline.policy_infer"),
        importlib.import_module("cr_replay_pipeline.policy_dataset"),
        importlib.import_module("cr_replay_pipeline.winner_dataset"),
    )


def _slug(name: str) -> str:
    return "-".join(name.strip().lower().replace("_", " ").split())


_NATIVE_CARD_ALIASES = {
    "hogrider": "hog-rider", "minipekka": "mini-pekka",
    "icespirit": "ice-spirit", "icespirits": "ice-spirit",
    "barblog": "barbarian-barrel", "barbarianbarrel": "barbarian-barrel",
    "thelog": "the-log",
}


def _native_card(name: str, known_cards: Sequence[str] = ()) -> str:
    compact = "".join(ch.lower() for ch in name if ch.isalnum())
    if compact in _NATIVE_CARD_ALIASES:
        return _NATIVE_CARD_ALIASES[compact]
    known = {"".join(ch.lower() for ch in card if ch.isalnum()): card for card in known_cards}
    return known.get(compact, _slug(name))


@dataclass
class BananaEpisode:
    """Physical-side decks and accepted actions for one native episode.

    ``decks[0]`` and ``decks[1]`` must each contain the exact eight-card native
    deck in replay-policy vocabulary. The actual native four-card hand is read
    from every observation, so no cycle heuristic is used for legal actions.
    """

    decks: tuple[tuple[str, ...], tuple[str, ...]]
    events: list[dict[str, Any]] = field(default_factory=list)
    battle_id: str = "native-self-play"

    def __post_init__(self) -> None:
        if any(len(deck) != 8 for deck in self.decks):
            raise ValueError("each physical-side deck must contain exactly 8 cards")

    def record_action(
        self, side: int, card: str, x: int, y: int, seconds: float,
        accepted: bool = True, event_type: str = "card_play",
    ) -> None:
        """Append an action after the native engine reports it was accepted."""
        if not accepted:
            return
        if side not in (0, 1):
            raise ValueError("side must be 0 or 1")
        normalized = _native_card(card, self.decks[side])
        if normalized not in self.decks[side]:
            raise ValueError(f"native card {card!r} is absent from side {side}'s configured deck")
        self.events.append({
            "seconds": float(seconds), "side": "team" if side == 0 else "opponent",
            "event_type": event_type, "card": normalized, "x": int(x), "y": int(y),
        })


@dataclass
class SampledAction:
    side: int
    slot: int                 # native hand slot 0..3
    deck_slot: int            # policy deck slot 0..7
    card: str
    x: int
    y: int
    tile: int
    log_prob: Tensor
    entropy: Tensor
    value: Tensor
    legal_deck_slots: tuple[int, ...]
    wait: bool = False
    input_cache: dict[str, Any] = field(default_factory=dict, repr=False)

    def native_request(self) -> dict[str, int]:
        if self.wait:
            raise ValueError("wait action has no native card request")
        return {"side": self.side, "slot": self.slot, "x": self.x, "y": self.y}


class BananaPolicy(nn.Module):
    """4.4.2 behavior-cloning actor plus a newly initialized PPO value head."""

    def __init__(
        self,
        model_dir: str | Path = DEFAULT_CHECKPOINT,
        device: str | torch.device | None = None,
        seed: int = 0,
        specialization: str = "banana-hog-1",
    ) -> None:
        super().__init__()
        infer, self._dataset, winner_dataset = _pipeline()
        self._infer = infer
        self._winner_dataset = winner_dataset
        self.device = torch.device(device or ("cuda" if torch.cuda.is_available() else "cpu"))
        self.model, self.vocab, self.config, _ = infer.load_policy(model_dir, str(self.device))
        if self.config.get("model_version") not in (None, "4.4.2") and self.config.get("model_name") != "policy-bc-v4.4.2":
            raise ValueError(f"expected policy-bc-v4.4.2 checkpoint, got {self.config.get('model_name')}")
        if self.config.get("placement_mode") != "heatmap" or self.config.get("placement_card_mode") != "selected":
            raise ValueError("checkpoint does not have the selected-card heatmap architecture")
        self.costs = winner_dataset.load_card_costs(SIBLING / "data" / "card_costs.json")
        self.global_index = self._dataset.GLOBAL_INDEX
        alias_data = json.loads((ROOT / "data" / "policy_card_aliases.json").read_text())
        self.native_id_to_policy = {int(row["numeric_id"]): str(row["policy_id"])
                                    for row in alias_data["cards"] if row.get("numeric_id") is not None}
        self.native_name_to_policy = {
            "".join(ch.lower() for ch in str(name) if ch.isalnum()): str(row["policy_id"])
            for row in alias_data["cards"]
            for name in (row.get("internal_id"), row.get("name"), row.get("engine_id"))
            if name
        }
        self.threat_dim = int(self.config.get("threat_dim", 0))
        self.max_context = int(self.config.get("max_context", 64))
        self.think_steps = int(self.config.get("eval_think_steps", 3))
        self.specialization = specialization
        self.rng = random.Random(seed)
        d_model = int(self.config.get("d_model", 160))
        self.critic = nn.Sequential(nn.LayerNorm(d_model), nn.Linear(d_model, d_model // 2), nn.Tanh(), nn.Linear(d_model // 2, 1)).to(self.device)
        self.wait_head = nn.Linear(d_model, 1).to(self.device)
        nn.init.zeros_(self.wait_head.weight)
        nn.init.constant_(self.wait_head.bias, -2.0)

    def _inputs(
        self, episode: BananaEpisode, side: int, now_seconds: float,
        hand_rows: Sequence[Mapping[str, Any]], elixir: float, opponent_elixir: float,
    ) -> tuple[tuple[Tensor, ...], list[tuple[int, int, str]], Tensor]:
        if side not in (0, 1):
            raise ValueError("side must be 0 or 1")
        team, opponent = episode.decks
        acting = team if side == 0 else opponent
        if not acting:
            raise ValueError("acting deck is empty")
        # Append a probe target exactly as sibling inference does. This selects
        # the physical-side perspective but the target itself is not encoded.
        dummy = {"seconds": max(float(now_seconds), episode.events[-1]["seconds"] + 1e-3 if episode.events else 0.),
                 "side": "team" if side == 0 else "opponent", "event_type": "card_play",
                 "card": acting[0], "x": 9000, "y": 16000}
        BattleExample = self._winner_dataset.BattleExample
        battle = BattleExample(episode.battle_id, team, opponent, 1, tuple(episode.events + [dummy]))
        sample = self._dataset.encode_policy_sample(
            battle, len(episode.events), self.vocab, self.costs,
            max_context=self.max_context, threat_dim=self.threat_dim,
            min_context=0, observation_seconds=float(dummy["seconds"]),
        )
        if sample is None:
            raise RuntimeError("4.4.2 preprocessing rejected native episode prefix")
        # The replay preprocessing infers hand from cycle history. Replace that
        # heuristic with authoritative native rows and current elixir.
        inputs = list(sample)
        # Native elixir is authoritative. Replayed action timing approximates
        # it, so substitute these two normalized globals for online rollouts.
        globals_ = inputs[4].clone()
        globals_[self.global_index["team_elixir"]] = max(0., min(float(elixir), 10.)) / 10.
        globals_[self.global_index["opp_elixir"]] = max(0., min(float(opponent_elixir), 10.)) / 10.
        inputs[4] = globals_
        deck = acting
        deck_lookup = {card: i for i, card in enumerate(deck)}
        legal: list[tuple[int, int, str]] = []
        for row in hand_rows:
            if not row.get("available", True) or not row.get("name"):
                continue
            card_id = int(row.get("card_id", -1))
            raw_name = str(row["name"])
            compact_name = "".join(ch.lower() for ch in raw_name if ch.isalnum())
            card = (self.native_id_to_policy.get(card_id)
                    or self.native_name_to_policy.get(compact_name)
                    or _native_card(raw_name, self.vocab.name_to_id))
            deck_slot = deck_lookup.get(card)
            native_slot = int(row["slot"])
            if deck_slot is None or not 0 <= native_slot < 4:
                continue
            cost = float(row.get("cost", self.costs.get(card, 4)))
            # Native observations and card descriptors report elixir in units.
            if cost <= float(elixir) + 1e-6 and self.vocab.encode(card) != self.vocab.unk_id:
                legal.append((native_slot, deck_slot, card))
        legal_slots = sorted({d for _, d, _ in legal})
        hard_mask = torch.zeros(8, dtype=torch.bool)
        hard_mask[legal_slots] = True
        inputs[6] = hard_mask
        return tuple(inputs), legal, hard_mask

    def act(
        self,
        episode: BananaEpisode,
        observation: Mapping[str, Any],
        side: int,
        now_seconds: float,
        temperature: float = 1.0,
        placement_temperature: float = 0.8,
        placement_top_k: int = 5,
        deterministic: bool = False,
        legal_tile_mask: Tensor | None = None,
    ) -> SampledAction:
        """No-grad rollout sample; cache only detached CPU tensors for PPO."""
        hands = observation.get("hands")
        elixirs = observation.get("elixir")
        if not isinstance(hands, Sequence) or len(hands) != 2 or not isinstance(elixirs, Sequence) or len(elixirs) != 2:
            raise ValueError("expected native observation with hands[2][4] and elixir[2]")
        inputs, legal, mask = self._inputs(episode, side, now_seconds, hands[side], float(elixirs[side]), float(elixirs[1-side]))
        cached = {
            "inputs": tuple(t.detach().cpu().clone() for t in
                            (inputs[0], inputs[1], inputs[2], inputs[3], inputs[4], inputs[12], inputs[5], mask)),
            "legal": tuple(legal),
            "tile_mask": None if legal_tile_mask is None else legal_tile_mask.detach().cpu().bool().flatten().clone(),
            "temperature": float(temperature),
            "placement_temperature": float(placement_temperature),
            "placement_top_k": int(placement_top_k),
            "think_steps": self.think_steps,
            "can_play": bool(legal),
        }
        with torch.no_grad():
            first = self._forward(cached)
            wait_dist = torch.distributions.Bernoulli(logits=self.wait_head(first["fused"]).squeeze())
            can_play = bool(cached["can_play"])
            wait_choice = (bool(wait_dist.sample().item()) if not deterministic else bool(wait_dist.probs.item() >= 0.5)) if can_play else True
            legal_deck_slots = tuple(sorted({d for _, d, _ in legal}))
            value = self.critic(first["fused"]).squeeze().detach()
            old_log_prob = wait_dist.log_prob(torch.tensor(float(wait_choice), device=self.device)) if can_play else first["fused"].sum() * 0.
            entropy = wait_dist.entropy() if can_play else old_log_prob * 0.
            if wait_choice:
                return SampledAction(side, -1, -1, "", 0, 0, -1, old_log_prob.detach(), entropy.detach(),
                                     value, legal_deck_slots, True, cached)
            slot_logits = first["slot_logits"][0] / max(float(temperature), 1e-4)
            legal_mask = cached["inputs"][7].to(self.device)
            slot_logits = slot_logits.masked_fill(~legal_mask, torch.finfo(slot_logits.dtype).min)
            slot_dist = Categorical(logits=slot_logits)
            slot_tensor = slot_logits.argmax() if deterministic else slot_dist.sample()
            deck_slot = int(slot_tensor.item())
            chosen_native_slot, _, card = next(row for row in legal if row[1] == deck_slot)
            second = self._forward(cached, deck_slot)
            tile_dist, indices = self._tile_distribution(second["tile_logits"][0], cached)
            # PPO must evaluate the same categorical support used at sampling.
            # A fresh top-k from updated logits can drop the sampled tile and
            # makes its likelihood undefined after the first optimizer step.
            cached["tile_candidates"] = indices.detach().cpu().clone()
            choice_tensor = tile_dist.logits.argmax() if deterministic else tile_dist.sample()
            tile = int(indices[choice_tensor].item())
            xy = torch.tensor([((tile % 32) + .5) / 32, ((tile // 32) + .5) / 18], device=self.device)
            offset = second.get("offset")
            if offset is not None:
                xy = (xy + offset[0].detach() * xy.new_tensor([.5 / 32, .5 / 18])).clamp(0., 1.)
            if side == 1:
                xy = torch.stack((xy[0], 1. - xy[1]))
            x = int(torch.round(xy[0] * 18000).clamp(0, 18000).item())
            y = int(torch.round(xy[1] * 32000).clamp(0, 32000).item())
            old_log_prob = old_log_prob + slot_dist.log_prob(slot_tensor) + tile_dist.log_prob(choice_tensor)
            entropy = entropy + slot_dist.entropy() + tile_dist.entropy()
        return SampledAction(side, chosen_native_slot, deck_slot, card, x, y, tile,
                             old_log_prob.detach(), entropy.detach(), value, legal_deck_slots, False, cached)

    def _forward(self, cached: Mapping[str, Any], placement_slot: int | None = None) -> dict[str, Tensor]:
        continuous, card_ids, team_deck, opp_deck, globals_, length, slot_features, hand_mask = cached["inputs"]
        args = [t.unsqueeze(0).to(self.device) for t in
                (continuous, card_ids, team_deck, opp_deck, globals_, length, slot_features)]
        kwargs: dict[str, Any] = {"hand_mask": hand_mask.to(self.device).unsqueeze(0),
                                  "think_steps": int(cached["think_steps"])}
        if placement_slot is not None:
            kwargs["placement_slots"] = torch.tensor([placement_slot], device=self.device)
        return self.model(*args, **kwargs)

    def _tile_distribution(self, logits: Tensor, cached: Mapping[str, Any]):
        logits = logits / max(float(cached["placement_temperature"]), 1e-4)
        supplied_mask = cached["tile_mask"]
        fixed_candidates = cached.get("tile_candidates")
        candidates = (fixed_candidates.to(self.device).long() if fixed_candidates is not None else
                      supplied_mask.to(self.device).nonzero(as_tuple=False).flatten()
                      if supplied_mask is not None else torch.arange(logits.numel(), device=self.device))
        if candidates.numel() == 0:
            raise ValueError("legal tile mask contains no legal locations")
        if fixed_candidates is not None:
            return Categorical(logits=logits[candidates]), candidates
        k = max(1, min(int(cached["placement_top_k"]), int(candidates.numel())))
        values, local_indices = torch.topk(logits[candidates], k)
        return Categorical(logits=values), candidates[local_indices]

    def evaluate_actions(self, actions: Sequence[SampledAction]) -> tuple[Tensor, Tensor, Tensor]:
        """Recompute PPO log-probs, entropy, and critic values from detached CPU caches."""
        log_probs, entropies, values = [], [], []
        # PyTorch's cuDNN GRU rejects backward if the module was loaded in eval
        # mode. Disable cuDNN for this differentiable recomputation; behavior
        # and dropout remain the eval behavior used when the rollout was sampled.
        with torch.backends.cudnn.flags(enabled=False):
            for action in actions:
                cached = action.input_cache
                first = self._forward(cached)
                wait_dist = torch.distributions.Bernoulli(logits=self.wait_head(first["fused"]).squeeze())
                wait_tensor = torch.tensor(float(action.wait), device=self.device)
                can_play = bool(cached["can_play"])
                log_prob, entropy = ((wait_dist.log_prob(wait_tensor), wait_dist.entropy()) if can_play
                                     else (first["fused"].sum() * 0., first["fused"].sum() * 0.))
                if not action.wait:
                    slot_logits = first["slot_logits"][0] / max(float(cached["temperature"]), 1e-4)
                    mask = cached["inputs"][7].to(self.device)
                    slot_dist = Categorical(logits=slot_logits.masked_fill(~mask, torch.finfo(slot_logits.dtype).min))
                    slot_tensor = torch.tensor(action.deck_slot, device=self.device)
                    second = self._forward(cached, action.deck_slot)
                    tile_dist, indices = self._tile_distribution(second["tile_logits"][0], cached)
                    choices = (indices == action.tile).nonzero(as_tuple=False).flatten()
                    if choices.numel() != 1:
                        raise ValueError(f"stored tile {action.tile} is not in this action's legal top-k set")
                    log_prob = log_prob + slot_dist.log_prob(slot_tensor) + tile_dist.log_prob(choices[0])
                    entropy = entropy + slot_dist.entropy() + tile_dist.entropy()
                values.append(self.critic(first["fused"]).squeeze())
                log_probs.append(log_prob)
                entropies.append(entropy)
        return torch.stack(log_probs), torch.stack(entropies), torch.stack(values)

    def record_action(self, episode: BananaEpisode, action: SampledAction,
                      seconds: float, accepted: bool) -> None:
        if action.wait:
            return
        episode.record_action(action.side, action.card, action.x, action.y, seconds, accepted)

def choose_specialist(card_names: Sequence[str]) -> str:
    """Name the intended specialization from an explicit native deck."""
    normalized = {_slug(name) for name in card_names}
    if "hog-rider" in normalized:
        return "banana-hog-1"
    if "mortar" in normalized:
        return "banana-mortar-1"
    raise ValueError("deck must include Hog Rider or Mortar for a named specialist")
