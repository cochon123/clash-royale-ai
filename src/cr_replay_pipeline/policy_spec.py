"""Explicit policy experiment configuration.

Version strings still select a recipe, but training and inference should read
named fields from ``PolicySpec`` instead of chaining ``use_v43`` flags and
magic feature indices.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass
from typing import Any

from .feature_schema import GLOBAL_FEATURE_NAMES, THREAT_FEATURE_NAMES


@dataclass(frozen=True)
class PolicySpec:
    version_key: str
    family: str
    model_name: str
    model_version: str
    threat_dim: int
    card_conditioned_placement: bool
    placement_mode: str
    placement_card_mode: str
    rollout_placement_decode: str
    rollout_placement_temperature: float
    rollout_placement_top_k: int | None
    default_max_think_steps: int
    lazy_mirror: bool
    stream_cache_size: int | None
    arena_memory_channels: int
    arena_hidden_channels: int
    arena_memory_version: str
    winner_only_default: bool
    hide_opponent_deck: bool
    skip_realism: bool
    freeze_backbone_default: bool
    default_reaction_weight: float
    default_reaction_repeats: int
    loss_preset: str
    memory_tokens: int
    memory_heads: int
    interaction_blocks: int
    event_clock: bool
    mixed_precision: bool
    default_d_model: int
    default_num_layers: int
    default_batch_size: int

    @property
    def global_dim(self) -> int:
        return len(GLOBAL_FEATURE_NAMES) + int(self.threat_dim)

    def to_dict(self) -> dict[str, Any]:
        data = asdict(self)
        data["global_dim"] = self.global_dim
        data["threat_feature_names"] = list(THREAT_FEATURE_NAMES[: self.threat_dim])
        data["global_feature_names"] = list(GLOBAL_FEATURE_NAMES)
        return data


def policy_spec_from_version(version: str) -> PolicySpec:
    key = str(version)
    is_v8 = key.startswith("8")
    is_v7 = key.startswith("7")
    is_v61 = key.startswith("6.1")
    is_v6 = key.startswith("6") or is_v7 or is_v8
    is_v442 = key.startswith("4.4.2")
    is_v441 = key.startswith("4.4.1") or is_v442 or is_v8
    is_v44 = key.startswith("4.4") or is_v8
    is_v43 = key.startswith("4.3")
    is_v4 = key.startswith("4") or is_v6
    is_v3 = key.startswith("3") or is_v4

    if is_v8:
        family = "v8"
        model_name, model_version = "policy-bc-v8", "8.0.0"
        loss_preset = "v8"
    elif is_v7:
        family = "v7"
        model_name, model_version = "policy-bc-v7", "7.0.0"
        loss_preset = "v7"
    elif is_v61:
        family = "v6.1"
        model_name, model_version = "policy-bc-v6.1", "6.1.0"
        loss_preset = "v6"
    elif is_v6:
        family = "v6"
        model_name, model_version = "policy-bc-v6", "6.0.0"
        loss_preset = "v6"
    elif is_v4:
        if is_v442:
            family, model_name, model_version = "v4.4.2", "policy-bc-v4.4.2", "4.4.2"
            loss_preset = "v44"
        elif is_v441:
            family, model_name, model_version = "v4.4.1", "policy-bc-v4.4.1", "4.4.1"
            loss_preset = "v44"
        else:
            parts = key.split(".")
            minor = parts[1] if len(parts) > 1 else "0"
            family = f"v4.{minor}"
            model_name, model_version = f"policy-bc-v4.{minor}", f"4.{minor}.0"
            loss_preset = "v44" if is_v44 else "v4"
    elif is_v3:
        family, model_name, model_version, loss_preset = "v3", "policy-bc-v3", "3.0.0", "v2"
    else:
        family, model_name, model_version, loss_preset = "v2", "policy-bc-v2", "2.0.0", "v2"

    if is_v43:
        think = 8
    elif is_v44 and not is_v8:
        think = 3
    else:
        think = 0

    placement_mode = "heatmap" if (is_v6 or is_v44) else "xy"
    default_batch = 128 if is_v8 else (512 if (is_v7 or key.startswith("4.2") or is_v44 or is_v43) else 256)
    default_d_model = 128 if is_v8 else 160
    return PolicySpec(
        version_key=key,
        family=family,
        model_name=model_name,
        model_version=model_version,
        threat_dim=len(THREAT_FEATURE_NAMES) if is_v3 else 0,
        card_conditioned_placement=is_v4,
        placement_mode=placement_mode,
        placement_card_mode="selected" if is_v441 else "soft",
        rollout_placement_decode=(
            "sample" if is_v441 else ("argmax" if placement_mode == "heatmap" else "expected")
        ),
        rollout_placement_temperature=0.6 if is_v441 else 1.0,
        rollout_placement_top_k=5 if is_v441 else None,
        default_max_think_steps=think,
        lazy_mirror=is_v441,
        stream_cache_size=256 if is_v441 else None,
        arena_memory_channels=16 if is_v7 else 0,
        arena_hidden_channels=32,
        arena_memory_version="decay-v1" if is_v7 else "none",
        winner_only_default=is_v442,
        hide_opponent_deck=key == "6",
        skip_realism=is_v7 or is_v8,
        freeze_backbone_default=is_v61 or is_v7,
        default_reaction_weight=3.0 if is_v3 else 1.0,
        default_reaction_repeats=2 if is_v3 else 1,
        loss_preset=loss_preset,
        memory_tokens=24 if is_v8 else 0,
        memory_heads=4,
        interaction_blocks=2 if is_v8 else 0,
        event_clock=is_v8,
        mixed_precision=is_v8,
        default_d_model=default_d_model,
        default_num_layers=4 if is_v8 else 2,
        default_batch_size=default_batch,
    )


def loss_kwargs_for_spec(spec: PolicySpec, *, freeze_backbone: bool) -> dict[str, float]:
    if spec.loss_preset == "v8":
        return {
            "slot_weight": 1.4,
            "type_weight": 0.25,
            "zone_weight": 1.1,
            "xy_weight": 0.0,
            "tile_weight": 0.55,
            "timing_weight": 0.25,
            "clock_weight": 0.35,
            "activity_weight": 0.15,
            "offset_weight": 0.12,
        }
    if spec.loss_preset == "v7" or (spec.family == "v6.1" and freeze_backbone):
        return {
            "slot_weight": 0.0,
            "type_weight": 0.0,
            "zone_weight": 0.0,
            "xy_weight": 0.0,
            "timing_weight": 0.0,
            "tile_weight": 1.0,
        }
    if spec.loss_preset == "v6":
        return {"zone_weight": 0.9, "xy_weight": 0.0, "tile_weight": 0.35, "slot_weight": 2.2}
    if spec.loss_preset == "v44":
        return {"zone_weight": 1.1, "xy_weight": 0.0, "tile_weight": 0.55, "slot_weight": 1.4}
    if spec.loss_preset == "v4":
        return {"zone_weight": 1.1, "xy_weight": 0.55, "slot_weight": 1.4}
    return {}
