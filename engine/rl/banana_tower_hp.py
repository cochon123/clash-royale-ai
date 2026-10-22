"""Optional tower-health inputs for the fast Banana adapter.

Use TowerHPPolicy to load these checkpoints. Old checkpoints remain valid.
Features are own/enemy [king, weaker princess, stronger princess] HP fractions;
the native reward snapshot does not expose lane coordinates.
"""
from __future__ import annotations

import math
from pathlib import Path

import torch
from torch import nn

from .banana_fast_policy import FastBananaPolicy


def tower_hp_features(state, side):
    if side not in (0, 1):
        raise ValueError("tower HP side must be 0 or 1")
    towers = state.get("towers", [])
    if len(towers) != 6 or {r.get("side") for r in towers} != {0, 1}:
        raise ValueError("tower HP requires six authoritative native towers")
    features = []
    identities = set()
    for team in (side, 1 - side):
        king, princess = [], []
        for row in (r for r in towers if r["side"] == team):
            identity = (team, row["id"])
            if identity in identities:
                raise ValueError("duplicate native tower identity")
            identities.add(identity)
            hp, maximum = float(row["hp"]), float(row["max_hp"])
            if not (math.isfinite(hp) and math.isfinite(maximum) and maximum > 0 and 0 <= hp <= maximum):
                raise ValueError("invalid native tower HP")
            if row["row"] == "KingTower":
                king.append(hp / maximum)
            elif row["row"] == "PrincessTower":
                princess.append(hp / maximum)
            else:
                raise ValueError("unexpected native tower kind")
        if len(king) != 1 or len(princess) != 2:
            raise ValueError("expected one king and two princess towers per side")
        features.extend(king + sorted(princess))
    return torch.tensor(features, dtype=torch.float32)


class TowerHPPolicy(FastBananaPolicy):
    def __init__(self, *args, tower_hp=False, weights=None, **kwargs):
        payload = None if weights is None else torch.load(Path(weights), map_location="cpu", weights_only=False)
        if payload is not None and payload.get("format") != "banana-native-ppo-v1":
            raise ValueError("expected banana-native-ppo-v1 checkpoint")
        has_adapter = payload is not None and any(k.startswith("tower_hp_adapter.") for k in payload["model_state"])
        super().__init__(*args, weights=None, **kwargs)
        self.tower_hp_enabled = bool(tower_hp or has_adapter)
        if self.tower_hp_enabled:
            # Residuals for card scores and the wait/value context. Placement
            # keeps the original context and receives no HP gradient.
            width = int(self.config.get("d_model", 160))
            self.model.tower_hp_adapter = nn.Sequential(nn.Linear(6, 32), nn.Tanh(), nn.Linear(32, 8 + width)).to(self.device)
            nn.init.zeros_(self.model.tower_hp_adapter[-1].weight)
            nn.init.zeros_(self.model.tower_hp_adapter[-1].bias)
        if payload is not None:
            missing, unexpected = self.model.load_state_dict(payload["model_state"], strict=False)
            allowed = {k for k in self.model.state_dict() if k.startswith("tower_hp_adapter.")} if not has_adapter else set()
            if set(missing) != allowed or unexpected:
                raise ValueError(f"incompatible model checkpoint: missing={missing}, unexpected={unexpected}")
            self.critic.load_state_dict(payload["critic_state"])
            self.wait_head.load_state_dict(payload["wait_state"])
            self.checkpoint_source = str(weights)
            self.checkpoint_name = payload.get("name")
            self.checkpoint_updates = int(payload.get("updates", 0))
        self.eval()

    @staticmethod
    def _cache_request(policy, episode, observation, side, *args, **kwargs):
        cache = FastBananaPolicy._cache_request(policy, episode, observation, side, *args, **kwargs)
        if policy.tower_hp_enabled:
            if "tower_state" not in observation:
                raise ValueError("tower-aware policy requires observation.tower_state")
            cache["tower_hp"] = tower_hp_features(observation["tower_state"], side)
        return cache

    def _forward_batch(self, caches, placement_slots=None):
        result = super()._forward_batch(caches, placement_slots)
        if self.tower_hp_enabled:
            hp = torch.stack([c["tower_hp"] for c in caches]).to(self.device)
            residual = self.model.tower_hp_adapter(hp)
            result["placement_fused"] = result["fused"]
            result["fused"] = result["fused"] + residual[:, 8:]
            result["slot_logits"] = result["slot_logits"] + residual[:, :8]
        return result

    def _placement_from_context(self, first, caches, indices, slots):
        if "placement_fused" in first:
            first = dict(first, fused=first["placement_fused"])
        return super()._placement_from_context(first, caches, indices, slots)
