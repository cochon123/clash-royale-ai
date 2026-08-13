"""Train and offline-evaluate a behavior-cloning next-action policy."""

from __future__ import annotations

import inspect
import json
import pickle
import random
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import numpy as np
import torch
import torch.nn as nn
from torch.optim import AdamW
from torch.optim.lr_scheduler import CosineAnnealingLR

from .policy_dataset import (
    DEFAULT_MAX_CONTEXT,
    DEFAULT_MIN_CONTEXT,
    DEFAULT_REACTION_SECONDS,
    GLOBAL_DIM,
    THREAT_DIM,
    TILE_UNITS,
    baseline_cycle_slot,
    baseline_frequency_slot,
    build_vocab,
    collect_battles,
    create_policy_dataloaders,
    encode_policy_sample,
    load_card_costs,
    split_battles,
    summarize_split,
)
from .policy_model import NUM_TILES, TILE_COLS, TILE_ROWS, PolicyBC
from .policy_manifest import (
    battles_from_manifest,
    build_manifest,
    load_manifest,
)
from .realism_generate import TimingPrior, generate_easy_negative, generate_medium_negative
from .realism_train import extract_realism_features
from .winner_dataset import BattleExample, CardVocab


class _MirroredEvent:
    """Small lazy view of an event reflected across the arena's x-axis."""

    __slots__ = ("_event",)

    def __init__(self, event: dict[str, Any]):
        self._event = event

    def __getitem__(self, key: str):
        if key == "x":
            return 18000 - int(self._event["x"])
        return self._event[key]

    def get(self, key: str, default=None):
        try:
            return self[key]
        except KeyError:
            return default


class _MirroredBattle:
    """Memory-light mirrored battle view; raw event dictionaries are shared."""

    __slots__ = ("battle_id", "team_deck", "opponent_deck", "team_wins", "events")

    def __init__(self, battle: BattleExample):
        self.battle_id = battle.battle_id + "-mirror"
        self.team_deck = battle.team_deck
        self.opponent_deck = battle.opponent_deck
        self.team_wins = battle.team_wins
        self.events = tuple(_MirroredEvent(event) for event in battle.events)


def _move_batch(batch, device: torch.device):
    (
        continuous,
        card_ids,
        team_deck,
        opponent_deck,
        globals_,
        slot_feats,
        hand_mask,
        slots,
        types,
        zones,
        xy,
        timing,
        lengths,
        weights,
    ) = batch
    return (
        continuous.to(device),
        card_ids.to(device),
        team_deck.to(device),
        opponent_deck.to(device),
        globals_.to(device),
        slot_feats.to(device),
        hand_mask.to(device),
        slots.to(device),
        types.to(device),
        zones.to(device),
        xy.to(device),
        timing.to(device),
        lengths.to(device),
        weights.to(device),
    )


@torch.no_grad()
def evaluate_policy(
    model: nn.Module,
    loader,
    device: torch.device,
    loss_kwargs: dict[str, float] | None = None,
    arena_control: str = "aligned",
    think_steps: int | None = None,
) -> dict[str, Any]:
    model.eval()
    total_loss = 0.0
    n_batches = 0
    slot_correct = 0
    slot_top3 = 0
    type_correct = 0
    zone_correct = 0
    total = 0
    xy_err = 0.0
    tile_hits = 0
    tile_class_correct = 0
    tile_top5_correct = 0
    tile_nll = 0.0
    has_tile_logits = False
    model_xy_rows: list[np.ndarray] = []
    argmax_xy_rows: list[np.ndarray] = []
    tile_entropy_sum = 0.0
    tile_top1_mass_sum = 0.0
    tile_top5_mass_sum = 0.0
    timing_err = 0.0
    loss_kwargs = loss_kwargs or {}

    for batch_index, batch in enumerate(loader):
        (
            continuous,
            card_ids,
            team_deck,
            opp_deck,
            globals_,
            slot_feats,
            hand_mask,
            slots,
            types,
            zones,
            xy,
            timing,
            lengths,
            _weights,
        ) = _move_batch(batch, device)
        permutation = None
        if arena_control == "shuffled" and continuous.size(0) > 1:
            # A deterministic derangement keeps the control reproducible and
            # avoids accidentally leaving a row in its original position.
            permutation = torch.roll(
                torch.arange(continuous.size(0), device=device), shifts=1
            )
        out = model(
            continuous,
            card_ids,
            team_deck,
            opp_deck,
            globals_,
            lengths,
            slot_feats,
            hand_mask,
            target_slots=None,
            arena_permutation=permutation,
            think_steps=think_steps,
        )
        losses = model.loss(out, slots, types, zones, xy, timing, **loss_kwargs)
        total_loss += float(losses["loss"].item())
        n_batches += 1

        preds = out["slot_logits"].argmax(dim=-1)
        top3 = out["slot_logits"].topk(3, dim=-1).indices
        slot_correct += int((preds == slots).sum().item())
        slot_top3 += int((top3 == slots.unsqueeze(-1)).any(dim=-1).sum().item())
        type_correct += int((out["type_logits"].argmax(dim=-1) == types).sum().item())
        zone_correct += int((out["zone_logits"].argmax(dim=-1) == zones).sum().item())
        total += int(slots.size(0))

        pred_xy = out["xy"].cpu().numpy()
        true_xy = xy.cpu().numpy()
        dx = (pred_xy[:, 0] - true_xy[:, 0]) * 18000.0
        dy = (pred_xy[:, 1] - true_xy[:, 1]) * 32000.0
        dist = np.sqrt(dx * dx + dy * dy)
        xy_err += float(dist.sum())
        tile_hits += int((dist <= TILE_UNITS).sum())
        if out.get("tile_logits") is not None:
            has_tile_logits = True
            tile_probs = torch.softmax(out["tile_logits"], dim=-1)
            tile_entropy_sum += float((-(tile_probs * tile_probs.clamp_min(1e-12).log2()).sum(dim=-1)).sum().item())
            top5_mass = tile_probs.topk(5, dim=-1).values
            tile_top1_mass_sum += float(top5_mass[:, 0].sum().item())
            tile_top5_mass_sum += float(top5_mass.sum().item())
            argmax_tiles = tile_probs.argmax(dim=-1)
            argmax_rows = torch.div(argmax_tiles, TILE_COLS, rounding_mode="floor")
            argmax_cols = argmax_tiles % TILE_COLS
            argmax_xy_rows.append(torch.stack([
                (argmax_cols.float() + 0.5) / TILE_COLS,
                (argmax_rows.float() + 0.5) / TILE_ROWS,
            ], dim=-1).cpu().numpy())
            true_tile_x = (xy[:, 0] * TILE_COLS).floor().long().clamp(0, TILE_COLS - 1)
            true_tile_y = (xy[:, 1] * TILE_ROWS).floor().long().clamp(0, TILE_ROWS - 1)
            true_tiles = true_tile_y * TILE_COLS + true_tile_x
            tile_class_correct += int(
                (out["tile_logits"].argmax(dim=-1) == true_tiles).sum().item()
            )
            tile_top5_correct += int(
                (out["tile_logits"].topk(5, dim=-1).indices == true_tiles.unsqueeze(-1))
                .any(dim=-1)
                .sum()
                .item()
            )
            tile_nll += float(
                torch.nn.functional.cross_entropy(
                    out["tile_logits"], true_tiles, reduction="sum"
                ).item()
            )
        model_xy_rows.append(pred_xy)

        pred_dt = np.expm1(out["timing"].cpu().numpy())
        true_dt = np.expm1(timing.cpu().numpy())
        timing_err += float(np.abs(pred_dt - true_dt).sum())

    return {
        "loss": total_loss / max(n_batches, 1),
        "slot_top1": slot_correct / max(total, 1),
        "slot_top3": slot_top3 / max(total, 1),
        "type_acc": type_correct / max(total, 1),
        "zone_acc": zone_correct / max(total, 1),
        "xy_mae": xy_err / max(total, 1),
        "tile_acc": tile_hits / max(total, 1),
        "tile_class_acc": (tile_class_correct / max(total, 1)) if has_tile_logits else None,
        "tile_top5_acc": (tile_top5_correct / max(total, 1)) if has_tile_logits else None,
        "tile_nll": (tile_nll / max(total, 1)) if has_tile_logits else None,
        "tile_entropy_bits": (tile_entropy_sum / max(total, 1)) if has_tile_logits else None,
        "tile_effective_count": (2.0 ** (tile_entropy_sum / max(total, 1))) if has_tile_logits else None,
        "tile_top1_mass": (tile_top1_mass_sum / max(total, 1)) if has_tile_logits else None,
        "tile_top5_mass": (tile_top5_mass_sum / max(total, 1)) if has_tile_logits else None,
        "timing_mae": timing_err / max(total, 1),
        "n": total,
        "model_x_std": float(np.concatenate(model_xy_rows)[:, 0].std())
        if model_xy_rows
        else None,
        "model_y_std": float(np.concatenate(model_xy_rows)[:, 1].std())
        if model_xy_rows
        else None,
        "argmax_x_std": float(np.concatenate(argmax_xy_rows)[:, 0].std())
        if argmax_xy_rows else None,
        "argmax_y_std": float(np.concatenate(argmax_xy_rows)[:, 1].std())
        if argmax_xy_rows else None,
    }


def _load_realism_scorer(path: Path):
    if not path.exists():
        return None
    with path.open("rb") as handle:
        return pickle.load(handle)


def _score_realism(artifact: dict[str, Any], battles: list[BattleExample], costs: dict[str, int]) -> list[float]:
    if not battles:
        return []
    features = np.stack([extract_realism_features(b, costs) for b in battles])
    hgb = artifact["models"]["hist_gradient_boosting"]
    tree = artifact["models"]["extra_trees"]
    weight = float(artifact["hgb_weight"])
    probs = weight * hgb.predict_proba(features)[:, 1] + (1.0 - weight) * tree.predict_proba(features)[:, 1]
    return [float(p) for p in probs]


def placement_diversity_stats(
    battles: list[BattleExample], start_event: int = 12, max_events: int = 40
) -> dict[str, float]:
    """Measure realized placement diversity on the model's own 18×32 grid."""
    unique_counts: list[float] = []
    max_shares: list[float] = []
    for battle in battles:
        tiles: list[int] = []
        for event in battle.events[start_event : start_event + max_events]:
            if event.get("event_type") == "ability_activation":
                continue
            x_norm = float(event.get("x", 9000)) / 18000.0
            y_norm = float(event.get("y", 16000)) / 32000.0
            if event.get("side") == "opponent":
                y_norm = 1.0 - y_norm
            col = min(TILE_COLS - 1, max(0, int(x_norm * TILE_COLS)))
            row = min(TILE_ROWS - 1, max(0, int(y_norm * TILE_ROWS)))
            tiles.append(row * TILE_COLS + col)
        if tiles:
            counts = np.unique(tiles, return_counts=True)[1]
            unique_counts.append(float(len(counts)))
            max_shares.append(float(counts.max() / len(tiles)))
    return {
        "battles": float(len(unique_counts)),
        "mean_unique_tiles": float(np.mean(unique_counts)) if unique_counts else 0.0,
        "mean_max_tile_share": float(np.mean(max_shares)) if max_shares else 0.0,
    }


@torch.no_grad()
def rollout_policy_battles(
    model: nn.Module,
    vocab,
    costs: dict[str, int],
    seed_battles: list[BattleExample],
    device: torch.device,
    n_battles: int = 64,
    warmup_events: int = 12,
    max_new_events: int = 40,
    temperature: float = 0.8,
    seed: int = 0,
    max_context: int = DEFAULT_MAX_CONTEXT,
    threat_dim: int = 0,
    placement_decode: str = "expected",
    placement_temperature: float = 1.0,
    placement_top_k: int | None = None,
    think_steps: int = 0,
    scheduling: str = "race",
) -> list[BattleExample]:
    """Generate model-faithful continuations.

    The deployment protocol is a two-sided timing race: both sides propose an
    action from the same history and the shorter predicted delay acts. This
    permits same-side double plays. ``alternate`` remains available solely to
    reproduce older reports that hard-flipped the actor after every action.
    """
    if scheduling not in {"race", "alternate"}:
        raise ValueError("scheduling must be race or alternate")
    from .policy_infer import predict_next_action

    model.eval()
    rng = random.Random(seed)
    chosen = list(seed_battles)
    rng.shuffle(chosen)
    chosen = chosen[:n_battles]
    out_battles: list[BattleExample] = []

    for battle in chosen:
        if len(battle.events) < warmup_events + 4:
            continue
        events = [dict(event) for event in battle.events[:warmup_events]]
        seconds = float(events[-1]["seconds"])
        next_side = battle.events[warmup_events]["side"]

        for _ in range(max_new_events):
            current = BattleExample(
                battle_id=battle.battle_id + "-rollout",
                team_deck=battle.team_deck,
                opponent_deck=battle.opponent_deck,
                team_wins=battle.team_wins,
                events=tuple(events),
            )
            sides = [next_side] if scheduling == "alternate" else ["team", "opponent"]
            predictions = []
            for side in sides:
                pred = predict_next_action(
                    model, vocab, costs, current, device,
                    acting_side=side, temperature=temperature, slot_decode="sample",
                    max_context=max_context, threat_dim=threat_dim,
                    placement_decode=placement_decode,
                    placement_temperature=placement_temperature,
                    placement_top_k=placement_top_k, think_steps=think_steps,
                    now_seconds=seconds, rng=rng,
                )
                predictions.append((side, pred))
            next_side, pred = min(
                predictions, key=lambda row: float(row[1]["delay_seconds"])
            )
            dt = float(pred["delay_seconds"])
            seconds = min(330.0, seconds + dt)
            events.append(
                {
                    "seconds": seconds,
                    "side": next_side,
                    "event_type": pred["event_type"],
                    "card": pred["card"],
                    "x": pred["x"],
                    "y": pred["y"],
                }
            )
            if scheduling == "alternate":
                next_side = "opponent" if next_side == "team" else "team"

        out_battles.append(
            BattleExample(
                battle_id=battle.battle_id + "-rollout",
                team_deck=battle.team_deck,
                opponent_deck=battle.opponent_deck,
                team_wins=battle.team_wins,
                events=tuple(events),
            )
        )
    return out_battles


def train_policy_model(*args: Any, **kwargs: Any) -> dict[str, Any]:
    """Train a policy, retrying v7 CUDA OOMs with deterministic accumulation."""
    version = str(kwargs.get("version", args[16] if len(args) > 16 else "2"))
    requested_batch = int(kwargs.get("batch_size", args[5] if len(args) > 5 else 256))
    if not version.startswith("7") or requested_batch < 512:
        return _train_policy_model_impl(*args, **kwargs)
    attempts = [(requested_batch, 1)]
    if requested_batch >= 512:
        attempts.extend([(384, 2), (256, 2)])
    last_error: RuntimeError | None = None
    normalized = dict(inspect.signature(_train_policy_model_impl).bind_partial(*args, **kwargs).arguments)
    for batch_size, accumulation in attempts:
        try:
            call_kwargs = dict(normalized)
            call_kwargs["batch_size"] = batch_size
            call_kwargs["gradient_accumulation_steps"] = accumulation
            if batch_size != requested_batch:
                print(
                    f"CUDA OOM fallback: batch_size={batch_size}, "
                    f"gradient_accumulation_steps={accumulation}",
                    flush=True,
                )
            return _train_policy_model_impl(**call_kwargs)
        except RuntimeError as exc:
            if "out of memory" not in str(exc).lower() or batch_size == attempts[-1][0]:
                raise
            last_error = exc
            if torch.cuda.is_available():
                torch.cuda.empty_cache()
    raise last_error or RuntimeError("v7 training failed after CUDA OOM fallbacks")


def _train_policy_model_impl(
    input_dir: str | Path = "data/raw",
    output_dir: str | Path = "models/policy_bc",
    card_costs_path: str | Path = "data/card_costs.json",
    realism_model_dir: str | Path = "models/realism_scorer",
    epochs: int = 25,
    batch_size: int = 256,
    gradient_accumulation_steps: int = 1,
    learning_rate: float = 2e-4,
    d_model: int = 160,
    num_layers: int = 2,
    min_card_plays: int = 12,
    seed: int = 42,
    device_name: str | None = None,
    patience: int = 7,
    dropout: float = 0.2,
    max_context: int = DEFAULT_MAX_CONTEXT,
    max_samples_per_battle: int | None = 40,
    version: str = "2",
    reaction_weight: float = 3.0,
    reaction_repeats: int = 2,
    reaction_seconds: float = DEFAULT_REACTION_SECONDS,
    max_battles: int | None = None,
    hide_opponent_deck: bool = False,
    hide_opponent_prob: float = 0.0,
    warmstart_dir: str | Path | None = None,
    freeze_backbone: bool = False,
    split_manifest: str | Path | None = None,
    write_split_manifest: str | Path | None = None,
    training_stage: str | None = None,
    arena_control: str = "aligned",
    arena_gate_bias: float = -2.2,
    progress_path: str | Path | None = None,
    mirror_training: bool = False,
    train_fraction: float = 0.9,
    training_log_path: str | Path | None = None,
    max_think_steps: int | None = None,
    eval_think_steps: int | None = None,
    winner_only: bool = False,
) -> dict[str, Any]:
    torch.manual_seed(seed)
    np.random.seed(seed)
    random.seed(seed)

    if device_name:
        device = torch.device(device_name)
    else:
        device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

    version = str(version)
    gradient_accumulation_steps = max(int(gradient_accumulation_steps), 1)
    use_v7 = version.startswith("7")
    use_v6 = version.startswith("6") or use_v7
    use_v61 = version.startswith("6.1")
    use_v4 = version.startswith("4") or use_v6
    use_v3 = version.startswith("3") or use_v4
    use_v43 = version.startswith("4.3")
    use_v44 = version.startswith("4.4")
    use_v442 = version.startswith("4.4.2")
    use_v441 = version.startswith("4.4.1") or use_v442
    winner_only = bool(winner_only or use_v442)
    card_conditioned_placement = use_v4
    # v4.4 / v6+: classify an 18×32 tile heatmap instead of regressing one XY mean.
    placement_mode = "heatmap" if (use_v6 or use_v44) else "xy"
    placement_card_mode = "selected" if use_v441 else "soft"
    rollout_placement_decode = (
        "sample" if use_v441 else ("argmax" if placement_mode == "heatmap" else "expected")
    )
    rollout_placement_temperature = 0.6 if use_v441 else 1.0
    rollout_placement_top_k = 5 if use_v441 else None
    rollout_slot_decode = "sample"
    rollout_slot_temperature = 0.8
    rollout_scheduling = "race"
    threat_dim = THREAT_DIM if use_v3 else 0
    global_dim = GLOBAL_DIM + threat_dim
    if max_think_steps is None:
        if use_v43:
            max_think_steps = 8
        elif use_v44:
            max_think_steps = 3
        else:
            max_think_steps = 0
    max_think_steps = max(int(max_think_steps), 0)
    if eval_think_steps is None:
        eval_think_steps = max_think_steps
    eval_think_steps = max(0, min(int(eval_think_steps), max_think_steps))
    if use_v7:
        model_name, model_version = "policy-bc-v7", "7.0.0"
    elif use_v6:
        model_name = "policy-bc-v6.1" if use_v61 else "policy-bc-v6"
        model_version = "6.1.0" if use_v61 else "6.0.0"
    elif use_v4:
        # "4" / "4.0" → 4.0.0; "4.1"/"4.2"/"4.3"/"4.4" keep their minor versions.
        if use_v442:
            model_name, model_version = "policy-bc-v4.4.2", "4.4.2"
        elif use_v441:
            model_name, model_version = "policy-bc-v4.4.1", "4.4.1"
        else:
            parts = version.split(".")
            minor = parts[1] if len(parts) > 1 else "0"
            model_name, model_version = f"policy-bc-v4.{minor}", f"4.{minor}.0"
    elif use_v3:
        model_name, model_version = "policy-bc-v3", "3.0.0"
    else:
        model_name, model_version = "policy-bc-v2", "2.0.0"
    if use_v3:
        # Defaults for v3/v4 reaction objectives; callers can still override.
        rw = reaction_weight
        rr = reaction_repeats
    else:
        rw = 1.0
        rr = 1
    # v4: upweight placement — experiments showed XY/zone are the main offline gap.
    loss_kwargs = (
        {
            "slot_weight": 0.0,
            "type_weight": 0.0,
            "zone_weight": 0.0,
            "xy_weight": 0.0,
            "timing_weight": 0.0,
            "tile_weight": 1.0,
        }
        if (use_v61 and freeze_backbone) or use_v7
        else {"zone_weight": 0.9, "xy_weight": 0.0, "tile_weight": 0.35, "slot_weight": 2.2}
        if use_v6
        else {
            "zone_weight": 1.1,
            "xy_weight": 0.0,
            "tile_weight": 0.55,
            "slot_weight": 1.4,
        }
        if use_v44
        else {"zone_weight": 1.1, "xy_weight": 0.55, "slot_weight": 1.4}
        if use_v4
        else {}
    )

    output = Path(output_dir)
    output.mkdir(parents=True, exist_ok=True)
    started = time.time()
    created_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    if use_v7:
        training_stage = training_stage or "arena-adapter"
        if training_stage not in {"arena-adapter", "placement-calibration"}:
            raise ValueError("v7 training_stage must be arena-adapter or placement-calibration")
        if arena_control not in {"aligned", "shuffled"}:
            raise ValueError("arena_control must be aligned or shuffled")
        freeze_backbone = True
        if warmstart_dir is None:
            warmstart_dir = "models/policy_bc_v6_1"
    print(f"Loading battles from {input_dir} ...", flush=True)
    battles = collect_battles(
        input_dir,
        min_card_plays=min_card_plays,
        require_decisive_result=winner_only,
    )
    manifest: dict[str, Any] | None = None
    if write_split_manifest:
        if split_manifest:
            raise ValueError("Use either split_manifest or write_split_manifest, not both")
        selected = battles[: int(max_battles)] if max_battles is not None else battles
        manifest = build_manifest(
            selected,
            write_split_manifest,
            seed=seed,
            train_fraction=train_fraction,
        )
        split_manifest = write_split_manifest
    if split_manifest:
        manifest = load_manifest(split_manifest)
        train_battles, val_battles, test_battles = battles_from_manifest(battles, manifest)
    else:
        if max_battles is not None:
            battles = battles[: int(max_battles)]
        if train_fraction == 0.9:
            rng = random.Random(seed)
            ordered = list(battles)
            rng.shuffle(ordered)
            n_train = int(len(ordered) * 0.9)
            n_val = int((len(ordered) - n_train) / 2)
            train_battles, val_battles, test_battles = (
                ordered[:n_train], ordered[n_train:n_train + n_val], ordered[n_train + n_val:]
            )
        else:
            train_battles, val_battles, test_battles = split_battles(battles, seed=seed)
    lazy_mirror_training = bool(mirror_training and use_v441)
    if mirror_training and not lazy_mirror_training:
        mirrored = [_MirroredBattle(battle) for battle in train_battles]
        train_battles = train_battles + mirrored
        print(f"Mirroring enabled: {len(mirrored):,} augmented training battles", flush=True)
    elif lazy_mirror_training:
        print(
            f"Memory-safe lazy mirroring enabled: {len(train_battles):,} source battles; "
            "encoded streams are shared",
            flush=True,
        )
    manifest_hash = (manifest or {}).get("ordered_id_sha256")
    if len(battles) < 50:
        # A manifest may resolve to fewer battles than the current collector
        # cache, so validate the resolved partitions as well.
        resolved_count = len(train_battles) + len(val_battles) + len(test_battles)
        if resolved_count < 50:
            raise RuntimeError(f"Need at least 50 usable battles; found {resolved_count}")
    battles = train_battles + val_battles + test_battles
    warmstart_artifact = None
    if warmstart_dir:
        warmstart_path = Path(warmstart_dir) / "best_model.pt"
        if not warmstart_path.exists():
            raise FileNotFoundError(f"Warm-start checkpoint not found: {warmstart_path}")
        warmstart_artifact = torch.load(
            warmstart_path, map_location="cpu", weights_only=False
        )
        # Preserve the checkpoint's card-id mapping. A small smoke subset may
        # omit cards present in v4.1, which would otherwise change embedding
        # dimensions or silently remap card identities.
        vocab = CardVocab.from_dict(warmstart_artifact["vocab"])
    else:
        vocab = build_vocab(train_battles)
    costs = load_card_costs(card_costs_path)

    train_loader, val_loader, test_loader = create_policy_dataloaders(
        train_battles,
        val_battles,
        test_battles,
        vocab,
        costs,
        batch_size=batch_size,
        max_context=max_context,
        max_samples_per_battle=max_samples_per_battle,
        threat_dim=threat_dim,
        reaction_seconds=reaction_seconds,
        reaction_weight=rw,
        reaction_repeats=rr,
        hide_opponent_deck=hide_opponent_deck,
        hide_opponent_prob=hide_opponent_prob,
        mirror_augmentation=lazy_mirror_training,
        stream_cache_size=256 if use_v441 else None,
        winner_only=winner_only,
    )
    print(
        f"Training {model_name} (threat_dim={threat_dim}, "
        f"card_conditioned_placement={card_conditioned_placement}, "
        f"placement_mode={placement_mode}, "
        f"placement_card_mode={placement_card_mode}, "
        f"winner_only={winner_only}, "
        f"reaction_weight={rw}, reaction_repeats={rr}, "
        f"max_think_steps={max_think_steps}, eval_think_steps={eval_think_steps})",
        flush=True,
    )
    print(
        f"Samples train/val/test: {len(train_loader.dataset)}/"
        f"{len(val_loader.dataset)}/{len(test_loader.dataset)} on {device}",
        flush=True,
    )

    model = PolicyBC(
        vocab_size=vocab.vocab_size,
        global_dim=global_dim,
        d_model=d_model,
        num_layers=num_layers,
        dropout=dropout,
        card_conditioned_placement=card_conditioned_placement,
        placement_mode=placement_mode,
        placement_card_mode=placement_card_mode,
        arena_memory_channels=16 if use_v7 else 0,
        arena_hidden_channels=32,
        arena_memory_version="decay-v1" if use_v7 else "none",
        arena_gate_bias=arena_gate_bias,
        max_think_steps=max_think_steps,
    ).to(device)
    warmstart_info: dict[str, Any] = {}
    if warmstart_artifact is not None:
        loaded = model.load_state_dict(warmstart_artifact["model_state"], strict=False)
        warmstart_info = {
            "dir": str(warmstart_dir),
            "missing_keys": list(loaded.missing_keys),
            "unexpected_keys": list(loaded.unexpected_keys),
        }
        print(
            f"Warm-started from {warmstart_dir} (missing={len(loaded.missing_keys)}, "
            f"unexpected={len(loaded.unexpected_keys)})",
            flush=True,
        )
    if freeze_backbone:
        for parameter in model.parameters():
            parameter.requires_grad = False
        if use_v7:
            if not hasattr(model, "arena_adapter"):
                raise RuntimeError("v7 requires a heatmap arena adapter")
            for parameter in model.arena_rasterizer.parameters():
                parameter.requires_grad = True
            for parameter in model.arena_adapter.parameters():
                parameter.requires_grad = True
            if training_stage == "placement-calibration":
                for parameter in model.tile_head[3].parameters():
                    parameter.requires_grad = True
            print(
                "Frozen-trunk v7 mode: training arena adapter"
                + (" + final tile projection" if training_stage == "placement-calibration" else ""),
                flush=True,
            )
        else:
            if not hasattr(model, "tile_head"):
                raise RuntimeError("freeze_backbone requires a heatmap placement head")
            for parameter in model.tile_head.parameters():
                parameter.requires_grad = True
            print("Frozen-trunk mode: training tile_head only", flush=True)
    trainable_parameters = [p for p in model.parameters() if p.requires_grad]
    optimizer = AdamW(trainable_parameters, lr=learning_rate, weight_decay=1e-4)
    scheduler = CosineAnnealingLR(optimizer, T_max=max(epochs, 1))

    history: list[dict[str, Any]] = []
    best_val = float("inf")
    best_state = None
    bad_epochs = 0
    progress_target = Path(progress_path) if progress_path else output / "progress.jsonl"
    progress_target.parent.mkdir(parents=True, exist_ok=True)
    table_log_target = Path(training_log_path) if training_log_path else output / "training.log"
    table_log_target.parent.mkdir(parents=True, exist_ok=True)
    table_columns = (
        "time", "phase", "epoch", "batch", "work", "done", "ETA", "loss",
        "card@1", "zone", "tile", "timing",
    )
    with table_log_target.open("w", encoding="utf-8") as table_handle:
        table_handle.write(" | ".join(table_columns) + "\n")
        table_handle.write("-+-".join("-" * len(column) for column in table_columns) + "\n")

    def append_table_row(
        phase: str,
        epoch_value: str,
        batch_value: str,
        work_value: float,
        done_value: str,
        eta_value: float,
        loss_value: float | None = None,
        card_value: float | None = None,
        zone_value: float | None = None,
        tile_value: float | None = None,
        timing_value: float | None = None,
    ) -> None:
        def pct_value(value: float | None) -> str:
            return "—" if value is None else f"{100.0 * value:6.2f}%"

        def num_value(value: float | None, digits: int = 3) -> str:
            return "—" if value is None else f"{value:.{digits}f}"

        def seconds_value(value: float) -> str:
            if value <= 0:
                return "done"
            minutes, seconds = divmod(int(value), 60)
            hours, minutes = divmod(minutes, 60)
            return f"{hours:d}h{minutes:02d}m" if hours else f"{minutes:02d}m{seconds:02d}s"

        row_values = (
            datetime.now(timezone.utc).strftime("%H:%M:%S"), phase, epoch_value,
            batch_value, f"{work_value:6.2f}%", done_value, seconds_value(eta_value),
            num_value(loss_value), pct_value(card_value), pct_value(zone_value),
            pct_value(tile_value), num_value(timing_value),
        )
        with table_log_target.open("a", encoding="utf-8") as table_handle:
            table_handle.write(" | ".join(row_values) + "\n")

    progress_target.write_text("", encoding="utf-8")
    log_target = Path("logs/policy_bc_v7.log") if use_v7 else None
    if log_target is not None:
        log_target.parent.mkdir(parents=True, exist_ok=True)
        with log_target.open("a", encoding="utf-8") as log_handle:
            log_handle.write(
                f"run model={model_name} stage={training_stage} control={arena_control} "
                f"batch={batch_size} accumulation={gradient_accumulation_steps}\n"
            )
    control_generator = torch.Generator(device=device)
    control_generator.manual_seed(seed + 43)
    run_started = time.time()

    for epoch in range(1, epochs + 1):
        # Frozen-trunk mode keeps dropout and batch-independent feature
        # extraction deterministic; only the new tile head is in train mode.
        if freeze_backbone:
            model.eval()
            if use_v7:
                model.arena_adapter.train()
                if training_stage == "placement-calibration":
                    model.tile_head[3].train()
            else:
                model.tile_head.train()
        else:
            model.train()
        running = 0.0
        n_batches = 0
        optimizer.zero_grad(set_to_none=True)
        for batch_index, batch in enumerate(train_loader, start=1):
            (
                continuous,
                card_ids,
                team_deck,
                opp_deck,
                globals_,
                slot_feats,
                hand_mask,
                slots,
                types,
                zones,
                xy,
                timing,
                lengths,
                weights,
            ) = _move_batch(batch, device)
            permutation = None
            if use_v7 and arena_control == "shuffled" and continuous.size(0) > 1:
                permutation = torch.randperm(
                    continuous.size(0), device=device, generator=control_generator
                )
            out = model(
                continuous,
                card_ids,
                team_deck,
                opp_deck,
                globals_,
                lengths,
                slot_feats,
                hand_mask,
                target_slots=slots,
                arena_permutation=permutation,
            )
            losses = model.loss(
                out,
                slots,
                types,
                zones,
                xy,
                timing,
                sample_weights=weights,
                **loss_kwargs,
            )
            (losses["loss"] / gradient_accumulation_steps).backward()
            should_step = (
                batch_index % gradient_accumulation_steps == 0
                or batch_index == len(train_loader)
            )
            if should_step:
                nn.utils.clip_grad_norm_(model.parameters(), 1.0)
                optimizer.step()
                optimizer.zero_grad(set_to_none=True)
            running += float(losses["loss"].item())
            n_batches += 1
            if batch_index == 1 or batch_index % 25 == 0 or batch_index == len(train_loader):
                elapsed = time.time() - run_started
                work = (epoch - 1) * len(train_loader) + batch_index
                total_work = max(epochs * len(train_loader), 1)
                rate = work / max(elapsed, 1e-6)
                eta = max(total_work - work, 0) / max(rate, 1e-6)
                progress_row = {
                    "timestamp": datetime.now(timezone.utc).isoformat(),
                    "phase": training_stage or ("v6.1" if use_v61 else version),
                    "epoch": epoch,
                    "epochs_total": epochs,
                    "batch": batch_index,
                    "batches_total": len(train_loader),
                    "samples_done": min(work * batch_size, epochs * len(train_loader.dataset)),
                    "samples_total": epochs * len(train_loader.dataset),
                    "progress_percent": 100.0 * work / total_work,
                    "elapsed_seconds": elapsed,
                    "eta_seconds": eta,
                    "loss": float(losses["loss"].item()),
                    "tile_loss": float(losses["tile_loss"].item()),
                    "learning_rate": float(optimizer.param_groups[0]["lr"]),
                    "gpu_memory_mb": (
                        torch.cuda.max_memory_allocated(device) / (1024.0 * 1024.0)
                        if device.type == "cuda"
                        else 0.0
                    ),
                }
                with progress_target.open("a", encoding="utf-8") as progress_handle:
                    progress_handle.write(json.dumps(progress_row) + "\n")
                print(
                    f"progress phase={progress_row['phase']} epoch={epoch}/{epochs} "
                    f"batch={batch_index}/{len(train_loader)} "
                    f"work={progress_row['progress_percent']:.1f}% "
                    f"elapsed={elapsed:.0f}s ETA={eta:.0f}s "
                    f"tile_loss={progress_row['tile_loss']:.4f}",
                    flush=True,
                )
                if log_target is not None:
                    with log_target.open("a", encoding="utf-8") as log_handle:
                        log_handle.write(
                            f"progress phase={progress_row['phase']} epoch={epoch}/{epochs} "
                            f"batch={batch_index}/{len(train_loader)} "
                            f"work={progress_row['progress_percent']:.1f}% "
                            f"elapsed={elapsed:.0f}s ETA={eta:.0f}s "
                            f"tile_loss={progress_row['tile_loss']:.4f} "
                            f"lr={progress_row['learning_rate']:.3e} "
                            f"vram_mb={progress_row['gpu_memory_mb']:.0f}\n"
                        )
                append_table_row(
                    str(progress_row["phase"]), f"{epoch}/{epochs}",
                    f"{batch_index}/{len(train_loader)}", progress_row["progress_percent"],
                    f"{progress_row['samples_done']:,}/{progress_row['samples_total']:,}",
                    progress_row["eta_seconds"], loss_value=progress_row["loss"],
                )
        scheduler.step()

        train_loss = running / max(n_batches, 1)
        val_metrics = evaluate_policy(
            model,
            val_loader,
            device,
            loss_kwargs=loss_kwargs,
            arena_control=arena_control if use_v7 else "aligned",
            think_steps=eval_think_steps if max_think_steps else None,
        )
        row = {
            "epoch": epoch,
            "train_loss": train_loss,
            **{f"val_{k}": v for k, v in val_metrics.items() if k != "n"},
            "val_n": val_metrics["n"],
            "lr": float(scheduler.get_last_lr()[0]),
        }
        history.append(row)
        tile_class = val_metrics.get("tile_class_acc")
        tile_top5 = val_metrics.get("tile_top5_acc")
        tile_extra = (
            f"  val_tile_cls={tile_class:.3f}  val_tile@5={tile_top5:.3f}"
            if tile_class is not None and tile_top5 is not None
            else ""
        )
        print(
            f"epoch {epoch:02d}  train_loss={train_loss:.4f}  "
            f"val_slot@1={val_metrics['slot_top1']:.3f}  "
            f"val_zone={val_metrics['zone_acc']:.3f}  "
            f"val_tile={val_metrics['tile_acc']:.3f}"
            f"{tile_extra}  "
            f"val_loss={val_metrics['loss']:.4f}",
            flush=True,
        )
        epoch_work = 100.0 * epoch / max(epochs, 1)
        elapsed = time.time() - run_started
        total_estimate = elapsed * epochs / max(epoch, 1)
        append_table_row(
            "validation", f"{epoch}/{epochs}", "epoch-end", epoch_work,
            f"{min(epoch * len(train_loader.dataset), epochs * len(train_loader.dataset)):,}/"
            f"{epochs * len(train_loader.dataset):,}",
            max(total_estimate - elapsed, 0.0), loss_value=train_loss,
            card_value=val_metrics["slot_top1"], zone_value=val_metrics["zone_acc"],
            tile_value=val_metrics["tile_acc"], timing_value=val_metrics["timing_mae"],
        )
        if val_metrics["loss"] < best_val - 1e-4:
            best_val = val_metrics["loss"]
            best_state = {k: v.detach().cpu().clone() for k, v in model.state_dict().items()}
            bad_epochs = 0
            # Persist best weights each improvement so a crash does not wipe the run.
            torch.save(
                {
                    "model_state": best_state,
                    "vocab": vocab.to_dict(),
                    "config": {
                        "batch_size": batch_size,
                        "gradient_accumulation_steps": gradient_accumulation_steps,
                        "d_model": d_model,
                        "num_layers": num_layers,
                        "dropout": dropout,
                        "max_context": max_context,
                        "max_samples_per_battle": max_samples_per_battle,
                        "min_context": DEFAULT_MIN_CONTEXT,
                        "global_dim": global_dim,
                        "threat_dim": threat_dim,
                        "reaction_seconds": reaction_seconds,
                        "card_conditioned_placement": card_conditioned_placement,
                        "placement_mode": placement_mode,
                        "placement_card_mode": placement_card_mode,
                        "rollout_placement_decode": rollout_placement_decode,
                        "rollout_placement_temperature": rollout_placement_temperature,
                        "rollout_placement_top_k": rollout_placement_top_k,
                        "rollout_slot_decode": rollout_slot_decode,
                        "rollout_slot_temperature": rollout_slot_temperature,
                        "rollout_scheduling": rollout_scheduling,
                        "version": model_version,
                        "warmstart": warmstart_info,
                        "freeze_backbone": freeze_backbone,
                        "training_stage": training_stage,
                        "arena_control": arena_control,
                        "arena_memory_channels": 16 if use_v7 else 0,
                        "arena_hidden_channels": 32,
                        "arena_memory_version": "decay-v1" if use_v7 else "none",
                        "arena_gate_bias": arena_gate_bias,
                        "split_manifest": str(split_manifest) if split_manifest else None,
                        "manifest_hash": manifest_hash,
                        "progress_path": str(progress_target),
                        "log_path": str(log_target) if log_target else None,
                        "trainable_parameters": sum(p.numel() for p in trainable_parameters),
                        "max_think_steps": max_think_steps,
                        "eval_think_steps": eval_think_steps,
                        "winner_only": winner_only,
                    },
                    "created_at": created_at,
                    "epoch": epoch,
                    "best_val_loss": best_val,
                },
                output / "best_model.pt",
            )
            with (output / "training_stages.json").open("w", encoding="utf-8") as handle:
                json.dump(history, handle, indent=2)
        else:
            bad_epochs += 1
            if bad_epochs >= patience:
                print(f"Early stop at epoch {epoch}", flush=True)
                break

    if best_state is not None:
        model.load_state_dict(best_state)

    test_metrics = evaluate_policy(
        model,
        test_loader,
        device,
        loss_kwargs=loss_kwargs,
        arena_control=arena_control if use_v7 else "aligned",
        think_steps=eval_think_steps if max_think_steps else None,
    )
    test_metrics_think_off: dict[str, Any] | None = None
    if max_think_steps > 0 and eval_think_steps != 0:
        test_metrics_think_off = evaluate_policy(
            model,
            test_loader,
            device,
            loss_kwargs=loss_kwargs,
            arena_control=arena_control if use_v7 else "aligned",
            think_steps=0,
        )
    freq_base = baseline_frequency_slot(test_battles)
    cycle_base = baseline_cycle_slot(test_battles)

    realism_path = Path(realism_model_dir) / "realism_ensemble.pkl"
    # v7 is an offline placement isolation experiment; avoid loading the
    # optional sklearn realism artifact and keep its environment-independent.
    realism_artifact = None if use_v7 else _load_realism_scorer(realism_path)
    rollout_stats: dict[str, Any] = {"available": False}
    if realism_artifact is not None:
        print("Rolling out policy continuations for realism scoring ...", flush=True)
        rollouts = rollout_policy_battles(
            model,
            vocab,
            costs,
            test_battles,
            device,
            n_battles=96,
            seed=seed + 7,
            max_context=max_context,
            threat_dim=threat_dim,
            placement_decode=rollout_placement_decode,
            placement_temperature=rollout_placement_temperature,
            placement_top_k=rollout_placement_top_k,
            think_steps=eval_think_steps,
            scheduling="race",
        )
        timing_prior = TimingPrior.from_battles(train_battles)
        rng = random.Random(seed + 9)
        easy = [
            generate_easy_negative(b, costs, rng, timing_prior)
            for b in test_battles[: len(rollouts)]
        ]
        medium = [
            generate_medium_negative(b, costs, rng, timing_prior)
            for b in test_battles[: len(rollouts)]
        ]
        real_slice = test_battles[: len(rollouts)]
        scores_real = _score_realism(realism_artifact, real_slice, costs)
        scores_policy = _score_realism(realism_artifact, rollouts, costs)
        scores_easy = _score_realism(realism_artifact, easy, costs)
        scores_medium = _score_realism(realism_artifact, medium, costs)
        rollout_stats = {
            "available": True,
            "n": len(rollouts),
            "mean_score_real": float(np.mean(scores_real)) if scores_real else 0.0,
            "mean_score_policy": float(np.mean(scores_policy)) if scores_policy else 0.0,
            "mean_score_easy": float(np.mean(scores_easy)) if scores_easy else 0.0,
            "mean_score_medium": float(np.mean(scores_medium)) if scores_medium else 0.0,
            "policy_vs_easy_lift": float(np.mean(scores_policy) - np.mean(scores_easy))
            if scores_policy and scores_easy
            else 0.0,
            "policy_vs_medium_lift": float(np.mean(scores_policy) - np.mean(scores_medium))
            if scores_policy and scores_medium
            else 0.0,
            "policy_gap_to_real": float(np.mean(scores_real) - np.mean(scores_policy))
            if scores_policy and scores_real
            else 0.0,
            "hist": {
                "real": scores_real,
                "policy": scores_policy,
                "easy": scores_easy,
                "medium": scores_medium,
            },
            "placement_diversity": {
                "human": placement_diversity_stats(real_slice),
                "policy": placement_diversity_stats(rollouts),
                "grid": f"{TILE_ROWS}x{TILE_COLS}",
                "decode": rollout_placement_decode,
                "temperature": rollout_placement_temperature,
                "top_k": rollout_placement_top_k,
            },
        }

    ckpt_path = output / "best_model.pt"
    torch.save(
        {
            "model_state": model.state_dict(),
            "vocab": vocab.to_dict(),
            "config": {
                "batch_size": batch_size,
                "gradient_accumulation_steps": gradient_accumulation_steps,
                "d_model": d_model,
                "num_layers": num_layers,
                "dropout": dropout,
                "max_context": max_context,
                "max_samples_per_battle": max_samples_per_battle,
                "min_context": DEFAULT_MIN_CONTEXT,
                "global_dim": global_dim,
                "threat_dim": threat_dim,
                "reaction_seconds": reaction_seconds,
                "card_conditioned_placement": card_conditioned_placement,
                "placement_mode": placement_mode,
                "placement_card_mode": placement_card_mode,
                "rollout_placement_decode": rollout_placement_decode,
                "rollout_placement_temperature": rollout_placement_temperature,
                "rollout_placement_top_k": rollout_placement_top_k,
                "rollout_slot_decode": rollout_slot_decode,
                "rollout_slot_temperature": rollout_slot_temperature,
                "rollout_scheduling": rollout_scheduling,
                "version": model_version,
                "warmstart": warmstart_info,
                "freeze_backbone": freeze_backbone,
                "training_stage": training_stage,
                "arena_control": arena_control,
                "arena_memory_channels": 16 if use_v7 else 0,
                "arena_hidden_channels": 32,
                "arena_memory_version": "decay-v1" if use_v7 else "none",
                "arena_gate_bias": arena_gate_bias,
                "split_manifest": str(split_manifest) if split_manifest else None,
                "manifest_hash": manifest_hash,
                "progress_path": str(progress_target),
                "log_path": str(log_target) if log_target else None,
                "trainable_parameters": sum(p.numel() for p in trainable_parameters),
                "max_think_steps": max_think_steps,
                "eval_think_steps": eval_think_steps,
                "winner_only": winner_only,
            },
            "created_at": created_at,
        },
        ckpt_path,
    )
    with (output / "vocab.json").open("w", encoding="utf-8") as handle:
        json.dump(vocab.to_dict(), handle, indent=2)

    n_params = sum(p.numel() for p in model.parameters())
    v6_lessons = [
        "v6 keeps the v4 threat-conditioned trunk but predicts a card-conditioned 18×32 placement heatmap instead of a single XY point.",
        "Tile cross-entropy models multimodal legal placements; expected XY is retained for compatibility while rollouts can sample tiles.",
        "Training can hide unrevealed opponent cards so the deck encoder is less dependent on an offline oracle deck.",
        "This is an offline action-prior experiment; it does not restore missing arena state or establish live-play readiness.",
    ]
    v7_lessons = [
        "v7 adds a causal 16-channel arena-memory proxy over the frozen v6.1 heatmap prior.",
        "The proxy remembers decayed action locations; it does not observe troop movement, death, health, or targeting.",
        "Aligned-versus-shuffled memory and adapter-off probes are required before calling the state hypothesis supported.",
        "This is an offline placement experiment and is not a live-play readiness signal.",
    ]
    v4_lessons = [
        "v4 keeps v3 threat conditioning + reaction upweight; adds jointly trained card-conditioned zone/XY heads.",
        "Offline probe: oracle card conditioning lifted zone to ~52%; e2e on frozen argmax did not — so v4 trains placement with the trunk (70% teacher / 30% soft slot mix).",
        "Hand-audit showed exact cycle reconstruction does not move slot accuracy — heuristic hand masks stay.",
        "Rollout autopsy blamed XY for most collapses; judge v4 on zone/XY MAE and collapse rate, not the auto live-play flag.",
        "Action-clock initiative (+6pp) is for rollout harness next — not baked into this checkpoint.",
    ]
    v3_lessons = [
        "v3 appends a 14-d recent-opponent-threat vector to globals (hog/balloon/GY/golem + wincon).",
        "Reaction windows (≤5s after threat) are preferentially sampled, repeated, and loss-upweighted.",
        "Success for v3 is measured on natural-hand support audit cells (GY→poison, hog→tornado), not synthetic forced hands.",
        "Do not regress overall slot top-1 / real defense-slice while chasing those cells.",
    ]
    v2_lessons = [
        "v1's flat 8-way head ignored per-card identity; v2 scores each deck card with embed + cycle features.",
        "Soft hand masking from wait-time heuristics helps after the cycle is observable (≥4 plays).",
        "Discrete placement zones are a better first target than raw XY on replay-only data.",
        "Rollout realism can look strong even when exact card top-1 is modest — judge both.",
    ]
    if use_v7:
        lessons = v7_lessons
    elif use_v6:
        lessons = v6_lessons
    elif use_v4:
        lessons = list(v4_lessons)
        if model_version.startswith("4.4"):
            if model_version == "4.4.2":
                lessons.insert(
                    0,
                    "v4.4.2 is trained only on the eventual winner's actions from decisive replays; the full opponent context remains visible, but loser actions are not imitation targets.",
                )
                lessons.insert(
                    1,
                    "Winner-side targets are perspective-normalized at encode time, so the same checkpoint can play either physical arena side without receiving the outcome label at inference.",
                )
                lessons.insert(
                    2,
                    "This is an offline winner-move prior, not proof of live win rate: validate with held-out winner actions and offline policy matchups before phone play.",
                )
            elif model_version == "4.4.1":
                lessons.insert(
                    0,
                    "v4.4.1 warm-starts from v4.4 on a fresh expanded-data split and conditions placement on the card actually selected, removing the train/eval soft-card mismatch.",
                )
                lessons.insert(
                    1,
                    "The shared rollout and phone harness decodes placement with temperature-0.6 sampling over the top five tiles; diversity is measured on the same 18×32 grid as the head.",
                )
            lessons.insert(
                3 if model_version == "4.4.2" else (2 if model_version == "4.4.1" else 0),
                "v4.4 keeps the v4.2 trunk size and v4.3 data recipe, but replaces XY regression with a card-conditioned 18×32 tile heatmap so placement can be multimodal.",
            )
            lessons.insert(
                4 if model_version == "4.4.2" else (3 if model_version == "4.4.1" else 1),
                "Primary placement signal is tile cross-entropy; expected XY is kept for compatibility. Think loop defaults to max K=3 for a cheap compute dial.",
            )
            lessons.insert(
                5 if model_version == "4.4.2" else (4 if model_version == "4.4.1" else 2),
                "Judge v4.4 on tile_class_acc / tile_top5_acc and placement spread, not only soft within-tile MAE from the XY mean.",
            )
        elif model_version.startswith("4.3"):
            lessons.insert(
                0,
                "v4.3 keeps the v4.2 recipe (90/5/5 + mirror + 40 windows), scales the trunk, and adds a toggled latent think loop so inference can spend more compute.",
            )
            lessons.insert(
                1,
                "Train samples K~Uniform(0..max_think_steps); inference think_steps=0 is the fast off path and higher K scales shared-weight refine steps.",
            )
        elif model_version.startswith("4.2"):
            lessons.insert(
                0,
                "v4.2 is v4.1 with the current data cut and horizontal arena mirroring applied to training battles only.",
            )
        elif model_version.startswith("4.1"):
            lessons.insert(
                0,
                "v4.1 keeps the v4 architecture and retrains on a newer data cut.",
            )
    elif use_v3:
        lessons = v3_lessons
    else:
        lessons = v2_lessons
    report = {
        "model_name": model_name,
        "model_version": model_version,
        "created_at": created_at,
        "seconds": round(time.time() - started, 1),
        "compute": {
            "device": str(device),
            "framework": "pytorch",
            "parameters": n_params,
            "d_model": d_model,
            "num_layers": num_layers,
            "epochs_requested": epochs,
            "epochs_ran": len(history),
            "batch_size": batch_size,
            "gradient_accumulation_steps": gradient_accumulation_steps,
            "learning_rate": learning_rate,
            "dropout": dropout,
            "max_context": max_context,
            "max_samples_per_battle": max_samples_per_battle,
            "global_dim": global_dim,
            "threat_dim": threat_dim,
            "card_conditioned_placement": card_conditioned_placement,
            "placement_mode": placement_mode,
            "placement_card_mode": placement_card_mode,
            "rollout_placement_decode": rollout_placement_decode,
            "rollout_placement_temperature": rollout_placement_temperature,
            "rollout_placement_top_k": rollout_placement_top_k,
            "rollout_slot_decode": rollout_slot_decode,
            "rollout_slot_temperature": rollout_slot_temperature,
            "rollout_scheduling": rollout_scheduling,
            "reaction_weight": rw,
            "reaction_repeats": rr,
            "reaction_seconds": reaction_seconds,
            "max_battles": max_battles,
            "hide_opponent_deck": hide_opponent_deck,
            "hide_opponent_prob": hide_opponent_prob,
            "loss_kwargs": loss_kwargs,
            "warmstart": warmstart_info,
            "training_log_path": str(table_log_target),
            "freeze_backbone": freeze_backbone,
            "training_stage": training_stage,
            "arena_control": arena_control,
            "arena_memory_channels": 16 if use_v7 else 0,
            "arena_hidden_channels": 32,
            "arena_memory_version": "decay-v1" if use_v7 else "none",
            "arena_gate_bias": arena_gate_bias,
            "split_manifest": str(split_manifest) if split_manifest else None,
            "manifest_hash": manifest_hash,
            "progress_path": str(progress_target),
            "log_path": str(log_target) if log_target else None,
            "trainable_parameters": sum(p.numel() for p in trainable_parameters),
            "mirror_training": mirror_training,
            "lazy_mirror_training": lazy_mirror_training,
            "max_think_steps": max_think_steps,
            "eval_think_steps": eval_think_steps,
            "winner_only": winner_only,
        },
        "data": {
            "battles_total": len(battles),
            "min_card_plays": min_card_plays,
            "train_samples": len(train_loader.dataset),
            "val_samples": len(val_loader.dataset),
            "test_samples": len(test_loader.dataset),
            "vocab_size": vocab.vocab_size,
            "splits": [
                summarize_split("train", train_battles),
                summarize_split("val", val_battles),
                summarize_split("test", test_battles),
            ],
        },
        "baselines": {
            "frequency": freq_base,
            "cycle": cycle_base,
            "chance_slot_top1": 0.125,
        },
        "val": {
            k: history[-1][f"val_{k}"]
            for k in (
                "loss",
                "slot_top1",
                "slot_top3",
                "type_acc",
                "zone_acc",
                "xy_mae",
                "tile_acc",
                "tile_class_acc",
                "tile_top5_acc",
                "tile_nll",
                "timing_mae",
                "model_x_std",
                "model_y_std",
            )
        }
        if history
        else {},
        "test": test_metrics,
        "test_think_off": test_metrics_think_off,
        "rollouts": {k: v for k, v in rollout_stats.items() if k != "hist"},
        "rollout_hist": rollout_stats.get("hist"),
        "history": history,
        "checkpoint": str(ckpt_path),
        "lessons": lessons,
        "live_play_readiness": _readiness(
            test_metrics,
            freq_base,
            cycle_base,
            rollout_stats,
            offline_only=use_v6,
        ),
    }

    report_path = output / "report.json"
    with report_path.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    with (output / "training_stages.json").open("w", encoding="utf-8") as handle:
        json.dump(history, handle, indent=2)

    print(
        json.dumps(
            {
                "test": test_metrics,
                "baselines": report["baselines"],
                "rollouts": report["rollouts"],
                "live_play_readiness": report["live_play_readiness"],
            },
            indent=2,
        )
    )
    print(f"Wrote {report_path}")
    return report


def _readiness(
    test: dict[str, Any],
    freq: dict[str, Any],
    cycle: dict[str, Any],
    rollouts: dict[str, Any],
    *,
    offline_only: bool = False,
) -> dict[str, Any]:
    slot = float(test.get("slot_top1", 0.0))
    tile = float(test.get("tile_acc", 0.0))
    zone = float(test.get("zone_acc", 0.0))
    best_base = max(float(freq.get("slot_top1", 0.0)), float(cycle.get("slot_top1", 0.0)))
    policy_score = float(rollouts.get("mean_score_policy", 0.0)) if rollouts.get("available") else 0.0
    easy_score = float(rollouts.get("mean_score_easy", 0.0)) if rollouts.get("available") else 0.0
    real_score = float(rollouts.get("mean_score_real", 0.0)) if rollouts.get("available") else 1.0

    checks = {
        "beats_baselines": slot >= best_base + 0.04,
        "slot_floor": slot >= 0.28,
        "zone_floor": zone >= 0.22,
        "tile_or_zone": tile >= 0.08 or zone >= 0.25,
        "rollout_beats_easy": (not rollouts.get("available")) or (policy_score >= easy_score + 0.15),
        "rollout_not_collapsed": (not rollouts.get("available")) or (policy_score >= 0.20),
        "rollout_near_real": (not rollouts.get("available")) or (policy_score >= 0.45 * real_score),
    }
    # The heatmap/action-prior v6 experiment is intentionally offline-only.
    # Passing generic metric gates must not be mistaken for authorization to
    # spend an expensive live-game run before the missing-state problem is
    # solved and the policy is independently validated.
    ready = all(checks.values()) and not offline_only
    return {
        "ready_for_live_smoke_test": ready,
        "checks": checks,
        "rationale": (
            "All offline gates passed; a short supervised live smoke test is justified."
            if ready
            else (
                "Offline-only experiment; do not run live yet."
                if offline_only
                else "Keep iterating offline — one or more readiness checks failed."
            )
        ),
    }
