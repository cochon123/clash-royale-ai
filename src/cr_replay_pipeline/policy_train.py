"""Train and offline-evaluate a behavior-cloning next-action policy."""

from __future__ import annotations

import contextlib
import inspect
import json
import math
import os
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
    baselines_for_dataset,
    build_vocab,
    collect_battles,
    create_policy_dataloaders,
    encode_policy_sample,
    load_card_costs,
    summarize_split,
)
from .policy_model import NUM_TILES, TILE_COLS, TILE_ROWS, PolicyBC
from .policy_manifest import resolve_battle_splits
from .policy_spec import loss_kwargs_for_spec, policy_spec_from_version
from .realism_generate import TimingPrior, generate_easy_negative, generate_medium_negative
from .realism_train import extract_realism_features
from .winner_dataset import BattleExample, CardVocab


def eager_module(module: nn.Module) -> nn.Module:
    """Unwrap torch.compile's OptimizedModule to the original nn.Module."""
    current = module
    seen: set[int] = set()
    while True:
        orig = getattr(current, "_orig_mod", None)
        if orig is None or id(orig) in seen:
            return current
        seen.add(id(current))
        current = orig


def cpu_state_dict(module: nn.Module) -> dict[str, Any]:
    """Clone weights on CPU without tracing through a compiled wrapper."""
    eager = eager_module(module)
    stance = getattr(torch.compiler, "set_stance", None)
    context: Any = contextlib.nullcontext()
    if callable(stance):
        try:
            maybe = stance("force_eager")
            if hasattr(maybe, "__enter__"):
                context = maybe
        except Exception:
            context = contextlib.nullcontext()
    with context, torch.no_grad():
        if torch.cuda.is_available():
            torch.cuda.synchronize()
        return {
            key: (
                value.detach().cpu().contiguous().clone()
                if torch.is_tensor(value)
                else value
            )
            for key, value in eager.state_dict().items()
        }


def atomic_torch_save(payload: dict[str, Any], path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + ".tmp")
    torch.save(payload, tmp)
    os.replace(tmp, path)


def resume_start_epoch(
    checkpoint: dict[str, Any], history: list[dict[str, Any]]
) -> int:
    if checkpoint.get("epoch") is not None:
        return int(checkpoint["epoch"]) + 1
    if history:
        return int(history[-1]["epoch"]) + 1
    return 1


def clip_history_to_checkpoint(
    history: list[dict[str, Any]], checkpoint: dict[str, Any]
) -> list[dict[str, Any]]:
    epoch = checkpoint.get("epoch")
    if epoch is None:
        return list(history)
    last = int(epoch)
    return [row for row in history if int(row.get("epoch") or 0) <= last]


def load_resume_checkpoint(
    output_dir: str | Path,
) -> tuple[dict[str, Any], list[dict[str, Any]], int]:
    output = Path(output_dir)
    last_path = output / "last.pt"
    best_path = output / "best_model.pt"
    ckpt_path = last_path if last_path.exists() else best_path
    if not ckpt_path.exists():
        raise FileNotFoundError(f"Cannot resume: missing {best_path}")
    checkpoint = torch.load(ckpt_path, map_location="cpu", weights_only=False)
    checkpoint["_resume_path"] = str(ckpt_path)
    history: list[dict[str, Any]] = []
    stages_path = output / "training_stages.json"
    if stages_path.exists():
        raw = json.loads(stages_path.read_text(encoding="utf-8"))
        if isinstance(raw, list):
            history = clip_history_to_checkpoint(raw, checkpoint)
    start_epoch = resume_start_epoch(checkpoint, history)
    return checkpoint, history, start_epoch


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
    kwargs = {"non_blocking": device.type == "cuda"}
    return (
        continuous.to(device, **kwargs),
        card_ids.to(device, **kwargs),
        team_deck.to(device, **kwargs),
        opponent_deck.to(device, **kwargs),
        globals_.to(device, **kwargs),
        slot_feats.to(device, **kwargs),
        hand_mask.to(device, **kwargs),
        slots.to(device, **kwargs),
        types.to(device, **kwargs),
        zones.to(device, **kwargs),
        xy.to(device, **kwargs),
        timing.to(device, **kwargs),
        lengths.to(device, **kwargs),
        weights.to(device, **kwargs),
    )


WAIT_HIT_SECONDS = 1.5


def _amp_autocast(device: torch.device, enabled: bool, dtype: torch.dtype):
    return torch.amp.autocast(device.type, dtype=dtype, enabled=enabled and device.type == "cuda")


def _float_outputs(outputs: dict[str, Any]) -> dict[str, Any]:
    return {
        key: value.float() if torch.is_tensor(value) and value.is_floating_point() else value
        for key, value in outputs.items()
    }


def batch_play_metrics(
    outputs: dict[str, torch.Tensor],
    slots: torch.Tensor,
    xy: torch.Tensor,
    timing: torch.Tensor,
) -> dict[str, float | None]:
    """Card top-1, exact tile, 3×3 tile neighborhood, and wait@1.5s hit rate."""
    card = float((outputs["slot_logits"].argmax(dim=-1) == slots).float().mean().item())
    pred_dt = torch.expm1(outputs["timing"].detach().float())
    true_dt = torch.expm1(timing.float())
    wait = float((pred_dt - true_dt).abs().lt(WAIT_HIT_SECONDS).float().mean().item())
    wait_mae = float((pred_dt - true_dt).abs().mean().item())
    tile_exact = None
    tile_near = None
    if outputs.get("tile_logits") is not None:
        pred = outputs["tile_logits"].argmax(dim=-1)
        true_x = (xy[:, 0] * TILE_COLS).floor().long().clamp(0, TILE_COLS - 1)
        true_y = (xy[:, 1] * TILE_ROWS).floor().long().clamp(0, TILE_ROWS - 1)
        true = true_y * TILE_COLS + true_x
        pred_y = torch.div(pred, TILE_COLS, rounding_mode="floor")
        pred_x = pred % TILE_COLS
        cheb = torch.maximum((pred_y - true_y).abs(), (pred_x - true_x).abs())
        tile_exact = float((pred == true).float().mean().item())
        tile_near = float((cheb <= 1).float().mean().item())
    return {
        "card": card,
        "tile_exact": tile_exact,
        "tile_near": tile_near,
        "wait": wait,
        "wait_mae": wait_mae,
    }


@torch.no_grad()
def evaluate_policy(
    model: nn.Module,
    loader,
    device: torch.device,
    loss_kwargs: dict[str, float] | None = None,
    arena_control: str = "aligned",
    think_steps: int | None = None,
    amp_enabled: bool | None = None,
    amp_dtype: torch.dtype | None = None,
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
    joint_card_tile = 0
    has_tile_logits = False
    clock_nll_sum = 0.0
    has_clock = False
    tile_near_correct = 0
    wait_hits = 0
    wait_err = 0.0
    model_xy_rows: list[np.ndarray] = []
    argmax_xy_rows: list[np.ndarray] = []
    tile_entropy_sum = 0.0
    tile_top1_mass_sum = 0.0
    tile_top5_mass_sum = 0.0
    timing_err = 0.0
    loss_kwargs = loss_kwargs or {}
    if amp_enabled is None:
        amp_enabled = device.type == "cuda"
    if amp_dtype is None:
        amp_dtype = torch.float16

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
        with _amp_autocast(device, amp_enabled, amp_dtype):
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
        losses = model.loss(_float_outputs(out), slots, types, zones, xy, timing, **loss_kwargs)
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
            tile_probs = torch.softmax(out["tile_logits"].float(), dim=-1)
            tile_entropy_sum += float(
                (-(tile_probs * tile_probs.clamp_min(1e-12).log2()).sum(dim=-1)).sum().item()
            )
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
            pred_rows = torch.div(argmax_tiles, TILE_COLS, rounding_mode="floor")
            pred_cols = argmax_tiles % TILE_COLS
            cheb = torch.maximum((pred_rows - true_tile_y).abs(), (pred_cols - true_tile_x).abs())
            tile_near_correct += int((cheb <= 1).sum().item())
            tile_top5_correct += int(
                (out["tile_logits"].topk(5, dim=-1).indices == true_tiles.unsqueeze(-1))
                .any(dim=-1)
                .sum()
                .item()
            )
            tile_nll += float(
                torch.nn.functional.cross_entropy(
                    out["tile_logits"].float(), true_tiles, reduction="sum"
                ).item()
            )
            joint_card_tile += int(((preds == slots) & (argmax_tiles == true_tiles)).sum().item())
        model_xy_rows.append(pred_xy)

        pred_dt = np.expm1(out["timing"].float().cpu().numpy())
        true_dt = np.expm1(timing.float().cpu().numpy())
        abs_wait = np.abs(pred_dt - true_dt)
        timing_err += float(abs_wait.sum())
        wait_err += float(abs_wait.sum())
        wait_hits += int((abs_wait < WAIT_HIT_SECONDS).sum())
        if out.get("clock_rates") is not None:
            from .event_clock import competing_risk_nll

            delay = torch.expm1(timing.float())
            clock_nll_sum += float(
                competing_risk_nll(out["clock_rates"].float(), delay).sum().item()
            )
            has_clock = True

    return {
        "loss": total_loss / max(n_batches, 1),
        "card": slot_correct / max(total, 1),
        "slot_top1": slot_correct / max(total, 1),
        "slot_top3": slot_top3 / max(total, 1),
        "type_acc": type_correct / max(total, 1),
        "zone_acc": zone_correct / max(total, 1),
        "xy_mae": xy_err / max(total, 1),
        "tile_acc": tile_hits / max(total, 1),
        "tile_exact": (tile_class_correct / max(total, 1)) if has_tile_logits else None,
        "tile_near": (tile_near_correct / max(total, 1)) if has_tile_logits else None,
        "tile_class_acc": (tile_class_correct / max(total, 1)) if has_tile_logits else None,
        "tile_top5_acc": (tile_top5_correct / max(total, 1)) if has_tile_logits else None,
        "tile_nll": (tile_nll / max(total, 1)) if has_tile_logits else None,
        "joint_card_tile": (joint_card_tile / max(total, 1)) if has_tile_logits else None,
        "clock_nll": (clock_nll_sum / max(total, 1)) if has_clock else None,
        "tile_entropy_bits": (tile_entropy_sum / max(total, 1)) if has_tile_logits else None,
        "tile_effective_count": (2.0 ** (tile_entropy_sum / max(total, 1))) if has_tile_logits else None,
        "tile_top1_mass": (tile_top1_mass_sum / max(total, 1)) if has_tile_logits else None,
        "tile_top5_mass": (tile_top5_mass_sum / max(total, 1)) if has_tile_logits else None,
        "timing_mae": timing_err / max(total, 1),
        "wait": wait_hits / max(total, 1),
        "wait_mae": wait_err / max(total, 1),
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
    """Train a policy, retrying CUDA OOMs with deterministic accumulation."""
    try:
        return _train_policy_model_with_oom(*args, **kwargs)
    finally:
        from .training_tui import TrainingDashboard

        active = getattr(TrainingDashboard, "_active", None)
        if active is not None:
            active.close()


def _train_policy_model_with_oom(*args: Any, **kwargs: Any) -> dict[str, Any]:
    version = str(kwargs.get("version", args[16] if len(args) > 16 else "2"))
    requested_batch = int(kwargs.get("batch_size", args[5] if len(args) > 5 else 256))
    if version.startswith("8"):
        attempts = [(requested_batch, 1)]
        if requested_batch >= 96:
            attempts.extend([(96, 1), (64, 2)])
        last_error: RuntimeError | None = None
        normalized = dict(
            inspect.signature(_train_policy_model_impl).bind_partial(*args, **kwargs).arguments
        )
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
        raise last_error or RuntimeError("v8 training failed after CUDA OOM fallbacks")
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
    metadata_path: str | Path | None = None,
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
    train_fraction: float = 0.7,
    training_log_path: str | Path | None = None,
    max_think_steps: int | None = None,
    eval_think_steps: int | None = None,
    winner_only: bool = False,
    no_tui: bool = False,
    resume: bool = False,
) -> dict[str, Any]:
    torch.manual_seed(seed)
    np.random.seed(seed)
    random.seed(seed)

    if device_name:
        device = torch.device(device_name)
    else:
        device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

    version = str(version)
    spec = policy_spec_from_version(version)
    gradient_accumulation_steps = max(int(gradient_accumulation_steps), 1)
    winner_only = bool(winner_only or spec.winner_only_default)
    card_conditioned_placement = spec.card_conditioned_placement
    placement_mode = spec.placement_mode
    placement_card_mode = spec.placement_card_mode
    rollout_placement_decode = spec.rollout_placement_decode
    rollout_placement_temperature = spec.rollout_placement_temperature
    rollout_placement_top_k = spec.rollout_placement_top_k
    rollout_slot_decode = "sample"
    rollout_slot_temperature = 0.8
    rollout_scheduling = "race"
    threat_dim = spec.threat_dim
    global_dim = spec.global_dim
    if max_think_steps is None:
        max_think_steps = spec.default_max_think_steps
    max_think_steps = max(int(max_think_steps), 0)
    if eval_think_steps is None:
        eval_think_steps = max_think_steps
    eval_think_steps = max(0, min(int(eval_think_steps), max_think_steps))
    model_name, model_version = spec.model_name, spec.model_version
    rw = reaction_weight if spec.threat_dim > 0 else 1.0
    rr = reaction_repeats if spec.threat_dim > 0 else 1
    freeze_backbone = bool(freeze_backbone or spec.freeze_backbone_default)
    loss_kwargs = loss_kwargs_for_spec(spec, freeze_backbone=freeze_backbone)
    use_v8 = spec.family == "v8"
    if use_v8:
        raise ValueError("The original v8 objective is retired. Use python -m cr_replay_pipeline.controlled_experiment; archived v8 inference remains supported.")
    use_v7 = spec.family == "v7"
    use_v6 = spec.family.startswith("v6") or use_v7
    use_v61 = spec.family == "v6.1"
    use_v4 = spec.card_conditioned_placement
    use_v3 = spec.threat_dim > 0
    use_v441 = spec.lazy_mirror
    if use_v8 and d_model == 160:
        d_model = spec.default_d_model
    if use_v8 and num_layers == 2:
        num_layers = spec.default_num_layers
    if use_v8:
        dropout = min(dropout, 0.1)
        torch.set_float32_matmul_precision("high")

    output = Path(output_dir)
    output.mkdir(parents=True, exist_ok=True)
    started = time.time()
    created_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    resume = bool(resume)
    resume_artifact: dict[str, Any] | None = None
    history: list[dict[str, Any]] = []
    start_epoch = 1
    best_val = float("inf")
    best_state = None
    best_epoch = 0
    bad_epochs = 0
    if resume:
        resume_artifact, history, start_epoch = load_resume_checkpoint(output)
        if start_epoch > epochs:
            raise RuntimeError(
                f"Cannot resume: checkpoint is already at epoch {start_epoch - 1} "
                f"of {epochs}"
            )
        if resume_artifact.get("created_at"):
            created_at = str(resume_artifact["created_at"])
        best_source = resume_artifact
        best_path = output / "best_model.pt"
        if best_path.exists():
            best_source = torch.load(best_path, map_location="cpu", weights_only=False)
        best_val = float(best_source.get("best_val_loss") or best_val)
        if math.isfinite(best_val) and best_source.get("model_state"):
            best_state = {
                key: value.detach().cpu().clone() if torch.is_tensor(value) else value
                for key, value in best_source["model_state"].items()
            }
        if history:
            best_epoch = 0
            rolling_best = float("inf")
            for row in history:
                value = row.get("val_loss")
                if value is None or not math.isfinite(float(value)):
                    continue
                if float(value) < rolling_best - 1e-4:
                    rolling_best = float(value)
                    best_epoch = int(row["epoch"])
            if best_epoch:
                bad_epochs = max(int(history[-1]["epoch"]) - best_epoch, 0)
    from .training_tui import TrainingDashboard, _want_trainer_tui

    progress_target = Path(progress_path) if progress_path else output / "progress.jsonl"
    log_target = Path("logs/policy_bc_v8.log") if use_v8 else (
        Path("logs/policy_bc_v7.log") if use_v7 else output / "console.log"
    )
    dashboard = TrainingDashboard(
        title=f"{model_name} · {model_version}",
        progress_path=progress_target,
        log_path=log_target,
        epochs=epochs,
        batches_total=1,
        samples_total=1,
        batch_size=batch_size,
        device=str(device),
        parameters=0,
        use_tui=_want_trainer_tui(not no_tui),
        start_live=False,
        persist=True,
        reset=not resume,
    )
    dashboard.update(phase="loading")
    dashboard.log(f"loading battles from {input_dir}")

    if use_v7:
        training_stage = training_stage or "arena-adapter"
        if training_stage not in {"arena-adapter", "placement-calibration"}:
            raise ValueError("v7 training_stage must be arena-adapter or placement-calibration")
        if arena_control not in {"aligned", "shuffled"}:
            raise ValueError("arena_control must be aligned or shuffled")
        freeze_backbone = True
        if warmstart_dir is None:
            warmstart_dir = "models/policy_bc_v6_1"
    battles = collect_battles(
        input_dir,
        min_card_plays=min_card_plays,
        require_decisive_result=winner_only,
        metadata_path=metadata_path,
        unresolved_abilities="preserve_event",
    )
    if use_v8 and split_manifest is None:
        cohort_path = Path("reports/astra_v8_evidence/cohort.json")
        if cohort_path.exists():
            holdout = {
                row["battle_id"]
                for row in json.loads(cohort_path.read_text(encoding="utf-8"))
            }
            before = len(battles)
            battles = [battle for battle in battles if battle.battle_id not in holdout]
            dashboard.log(
                f"held out {before - len(battles):,} Astra probe battles from the v8 split"
            )
        if max_battles is None:
            max_battles = 7143
        if write_split_manifest is None:
            write_split_manifest = "data/splits/policy_v8_pilot_5000_seed42.json"
    train_battles, val_battles, test_battles, manifest, split_manifest_path = resolve_battle_splits(
        battles,
        seed=seed,
        train_fraction=train_fraction,
        split_manifest=split_manifest,
        write_split_manifest=write_split_manifest,
        output_dir=output,
        warmstart_dir=warmstart_dir,
        max_battles=max_battles,
        min_card_plays=min_card_plays,
        create=True,
    )
    split_manifest = split_manifest_path
    lazy_mirror_training = bool(mirror_training and use_v441)
    if mirror_training and not lazy_mirror_training:
        mirrored = [_MirroredBattle(battle) for battle in train_battles]
        train_battles = train_battles + mirrored
        dashboard.log(f"mirroring enabled: {len(mirrored):,} augmented training battles")
    elif lazy_mirror_training:
        dashboard.log(
            f"lazy mirroring: {len(train_battles):,} source battles; encoded streams are shared"
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
    if resume_artifact is not None:
        vocab = CardVocab.from_dict(resume_artifact["vocab"])
        donor_vocab = vocab
        current_vocab = build_vocab(train_battles)
        if not donor_vocab.compatible_with(current_vocab):
            extra = sorted(set(current_vocab.name_to_id) - set(donor_vocab.name_to_id))
            missing = sorted(set(donor_vocab.name_to_id) - set(current_vocab.name_to_id))
            dashboard.log(
                "resume vocabulary mapping preserved from the checkpoint "
                f"(extra={len(extra)}, missing_in_current={len(missing)})"
            )
    elif warmstart_dir:
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
        donor_vocab = vocab
        current_vocab = build_vocab(train_battles)
        if not donor_vocab.compatible_with(current_vocab):
            extra = sorted(set(current_vocab.name_to_id) - set(donor_vocab.name_to_id))
            missing = sorted(set(donor_vocab.name_to_id) - set(current_vocab.name_to_id))
            dashboard.log(
                "warm-start vocabulary mapping preserved from the donor checkpoint "
                f"(extra={len(extra)}, missing_in_current={len(missing)})"
            )
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
        stream_cache_size=spec.stream_cache_size,
        winner_only=winner_only,
        num_workers=2 if use_v8 else 0,
        pad_to=max_context if use_v8 else None,
        pin_memory=device.type == "cuda",
    )
    dashboard.log(
        f"training {model_name} threat_dim={threat_dim} placement={placement_mode}/"
        f"{placement_card_mode} winner_only={winner_only}"
    )
    dashboard.log(
        f"samples train/val/test {len(train_loader.dataset)}/"
        f"{len(val_loader.dataset)}/{len(test_loader.dataset)} on {device}"
    )

    from .belief_policy import build_policy_model

    model_config = {
        "d_model": d_model,
        "num_layers": num_layers,
        "dropout": dropout,
        "global_dim": global_dim,
        "threat_dim": threat_dim,
        "card_conditioned_placement": card_conditioned_placement,
        "placement_mode": placement_mode,
        "placement_card_mode": placement_card_mode,
        "arena_memory_channels": 16 if use_v7 else 0,
        "arena_hidden_channels": 32,
        "arena_memory_version": "decay-v1" if use_v7 else "none",
        "arena_gate_bias": arena_gate_bias,
        "max_think_steps": max_think_steps,
        "memory_tokens": spec.memory_tokens,
        "memory_heads": spec.memory_heads,
        "interaction_blocks": spec.interaction_blocks,
        "event_clock": spec.event_clock,
    }
    model = build_policy_model(vocab.vocab_size, model_config).to(device)
    warmstart_info: dict[str, Any] = {}
    if resume_artifact is not None:
        loaded = model.load_state_dict(resume_artifact["model_state"], strict=True)
        warmstart_info = {
            "dir": str(output),
            "resume": True,
            "from_epoch": start_epoch - 1,
            "missing_keys": list(loaded.missing_keys),
            "unexpected_keys": list(loaded.unexpected_keys),
        }
        dashboard.log(
            f"resuming weights from {resume_artifact.get('_resume_path', output / 'best_model.pt')} "
            f"(completed epoch {start_epoch - 1}, next {start_epoch}/{epochs})"
        )
    elif warmstart_artifact is not None:
        loaded = model.load_state_dict(warmstart_artifact["model_state"], strict=False)
        warmstart_info = {
            "dir": str(warmstart_dir),
            "missing_keys": list(loaded.missing_keys),
            "unexpected_keys": list(loaded.unexpected_keys),
        }
        dashboard.log(
            f"warm-started from {warmstart_dir} (missing={len(loaded.missing_keys)}, "
            f"unexpected={len(loaded.unexpected_keys)})"
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
            dashboard.log(
                "frozen-trunk v7 mode: training arena adapter"
                + (" + final tile projection" if training_stage == "placement-calibration" else "")
            )
        else:
            if not hasattr(model, "tile_head"):
                raise RuntimeError("freeze_backbone requires a heatmap placement head")
            for parameter in model.tile_head.parameters():
                parameter.requires_grad = True
            dashboard.log("frozen-trunk mode: training tile_head only")
    raw_model = model
    trainable_parameters = [p for p in raw_model.parameters() if p.requires_grad]
    optimizer = AdamW(trainable_parameters, lr=learning_rate, weight_decay=1e-4)
    scheduler = CosineAnnealingLR(optimizer, T_max=max(epochs, 1))
    restored_optimizer = False
    if resume_artifact is not None:
        if resume_artifact.get("optimizer_state"):
            optimizer.load_state_dict(resume_artifact["optimizer_state"])
            restored_optimizer = True
        if resume_artifact.get("scheduler_state"):
            scheduler.load_state_dict(resume_artifact["scheduler_state"])
        else:
            for _ in range(max(start_epoch - 1, 0)):
                scheduler.step()
        dashboard.log(
            f"cosine LR at {float(optimizer.param_groups[0]['lr']):.6g}; "
            + (
                "Adam moments restored from checkpoint"
                if restored_optimizer
                else "Adam moments reset (checkpoint had weights only)"
            )
        )
    use_amp = False
    amp_dtype: torch.dtype = torch.float32
    if spec.mixed_precision and device.type == "cuda" and torch.cuda.is_bf16_supported():
        use_amp = True
        amp_dtype = torch.bfloat16
    scaler = torch.amp.GradScaler("cuda", enabled=False)
    compiled = False
    if use_v8 and device.type == "cuda":
        torch.backends.cudnn.benchmark = True
        try:
            model = torch.compile(raw_model, mode="reduce-overhead", dynamic=False)
            compiled = True
        except Exception as exc:
            dashboard.log(f"torch.compile unavailable ({exc}); running eager")
            model = raw_model

    table_log_target = Path(training_log_path) if training_log_path else output / "training.log"
    table_log_target.parent.mkdir(parents=True, exist_ok=True)
    table_columns = (
        "time", "phase", "epoch", "batch", "work", "done", "ETA", "loss",
        "card@1", "zone", "tile", "timing",
    )
    if not (resume and table_log_target.exists()):
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

    log_target.parent.mkdir(parents=True, exist_ok=True)
    with log_target.open("a", encoding="utf-8") as log_handle:
        log_handle.write(
            f"run model={model_name} stage={training_stage} control={arena_control} "
            f"batch={batch_size} accumulation={gradient_accumulation_steps}\n"
        )
    dashboard.batches_total = max(len(train_loader), 1)
    dashboard.samples_total = max(epochs * len(train_loader.dataset), 1)
    dashboard.parameters = sum(p.numel() for p in model.parameters())
    dashboard.start_live()
    dashboard.log(
        f"train {len(train_loader.dataset):,}  val {len(val_loader.dataset):,}  "
        f"test {len(test_loader.dataset):,}  amp={'bf16' if use_amp else 'off'}  "
        f"compile={'on' if compiled else 'off'}  "
        f"workers={2 if use_v8 else 0}  params={dashboard.parameters:,}"
    )
    control_generator = torch.Generator(device=device)
    control_generator.manual_seed(seed + 43)
    run_started = time.time()
    progress_every = 25
    completed_batches = (start_epoch - 1) * max(len(train_loader), 1)

    def checkpoint_config() -> dict[str, Any]:
        return {
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
            "train_fraction": train_fraction,
            "policy_spec": spec.to_dict(),
            "progress_path": str(progress_target),
            "log_path": str(log_target) if log_target else None,
            "trainable_parameters": sum(p.numel() for p in trainable_parameters),
            "max_think_steps": max_think_steps,
            "eval_think_steps": eval_think_steps,
            "winner_only": winner_only,
            "memory_tokens": spec.memory_tokens,
            "memory_heads": spec.memory_heads,
            "interaction_blocks": spec.interaction_blocks,
            "event_clock": spec.event_clock,
            "mixed_precision": use_amp,
            "amp_dtype": "bf16" if amp_dtype == torch.bfloat16 else "fp32",
            "compiled": compiled,
        }

    from .artifact_cache import source_fingerprint, sha256_file, DATASET_PREPROCESS_VERSION
    resume_contract = {
        "source": source_fingerprint(input_dir), "preprocess": DATASET_PREPROCESS_VERSION,
        "observation": "causal-observation-1", "manifest": manifest_hash,
        "metadata": sha256_file(metadata_path) if metadata_path else None,
        "costs": sha256_file(card_costs_path), "loss": loss_kwargs,
        "model": model_config, "batch_size": batch_size, "seed": seed,
        "mirror": mirror_training, "winner_only": winner_only,
        "reaction_repeats": rr, "reaction_weight": rw, "max_samples": max_samples_per_battle,
        "hide_opponent": [hide_opponent_deck, hide_opponent_prob],
    }
    if resume_artifact is not None and resume_artifact.get("resume_contract") != resume_contract:
        raise ValueError("Resume contract changed or missing; create a new run lineage")

    def save_checkpoint(
        path: Path,
        state: dict[str, Any],
        epoch: int,
        val_loss: float,
        *,
        include_optim: bool,
    ) -> None:
        payload: dict[str, Any] = {
            "model_state": state,
            "vocab": vocab.to_dict(),
            "config": checkpoint_config(),
            "created_at": created_at,
            "epoch": epoch,
            "weights_epoch": epoch,
            "resume_contract": resume_contract,
            "best_val_loss": val_loss,
        }
        if include_optim:
            payload["optimizer_state"] = optimizer.state_dict()
            payload["scheduler_state"] = scheduler.state_dict()
        atomic_torch_save(payload, path)

    for epoch in range(start_epoch, epochs + 1):
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
            if (use_v7 or use_v8) and arena_control == "shuffled" and continuous.size(0) > 1:
                permutation = torch.randperm(
                    continuous.size(0), device=device, generator=control_generator
                )
            with _amp_autocast(device, use_amp, amp_dtype):
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
                _float_outputs(out),
                slots,
                types,
                zones,
                xy,
                timing,
                sample_weights=weights,
                **loss_kwargs,
            )
            loss = losses["loss"]
            if not torch.isfinite(loss).all():
                optimizer.zero_grad(set_to_none=True)
                dashboard.log(
                    f"skipped non-finite batch  epoch {epoch} batch {batch_index}"
                )
                continue
            scaler.scale(loss / gradient_accumulation_steps).backward()
            should_step = (
                batch_index % gradient_accumulation_steps == 0
                or batch_index == len(train_loader)
            )
            if should_step:
                scaler.unscale_(optimizer)
                grad_norm = nn.utils.clip_grad_norm_(trainable_parameters, 1.0)
                if not math.isfinite(float(grad_norm)):
                    optimizer.zero_grad(set_to_none=True)
                    dashboard.log(
                        f"skipped non-finite grad  epoch {epoch} batch {batch_index}"
                    )
                else:
                    scaler.step(optimizer)
                scaler.update()
                optimizer.zero_grad(set_to_none=True)
            running += float(losses["loss"].item())
            n_batches += 1
            if (
                batch_index == 1
                or batch_index % progress_every == 0
                or batch_index == len(train_loader)
            ):
                with torch.no_grad():
                    play = batch_play_metrics(out, slots, xy, timing)
                elapsed = time.time() - run_started
                work = (epoch - 1) * len(train_loader) + batch_index
                total_work = max(epochs * len(train_loader), 1)
                work_this_run = max(work - completed_batches, 1)
                rate = work_this_run / max(elapsed, 1e-6)
                eta = max(total_work - work, 0) / max(rate, 1e-6)
                phase_name = training_stage or ("v8" if use_v8 else ("v6.1" if use_v61 else version))
                dashboard.update(
                    phase=phase_name,
                    epoch=epoch,
                    epochs_total=epochs,
                    batch=batch_index,
                    batches_total=len(train_loader),
                    samples_done=min(work * batch_size, epochs * len(train_loader.dataset)),
                    samples_total=epochs * len(train_loader.dataset),
                    progress_percent=100.0 * work / total_work,
                    elapsed_seconds=elapsed,
                    eta_seconds=eta,
                    loss=float(losses["loss"].item()),
                    acc=play["card"],
                    card=play["card"],
                    tile_exact=play["tile_exact"],
                    tile_near=play["tile_near"],
                    wait=play["wait"],
                    wait_mae=play["wait_mae"],
                    tile_loss=float(losses["tile_loss"].item()),
                    clock=float(losses["clock_loss"].item()) if "clock_loss" in losses else None,
                    lr=float(optimizer.param_groups[0]["lr"]),
                    batch_size=batch_size,
                    parameters=sum(p.numel() for p in model.parameters()),
                    device=str(device),
                    model_name=model_name,
                )
                append_table_row(
                    str(phase_name), f"{epoch}/{epochs}",
                    f"{batch_index}/{len(train_loader)}", 100.0 * work / total_work,
                    f"{min(work * batch_size, epochs * len(train_loader.dataset)):,}/"
                    f"{epochs * len(train_loader.dataset):,}",
                    eta, loss_value=float(losses["loss"].item()),
                    card_value=play["card"],
                )
        scheduler.step()

        train_loss = running / max(n_batches, 1)
        val_metrics = evaluate_policy(
            model,
            val_loader,
            device,
            loss_kwargs=loss_kwargs,
            arena_control=arena_control if (use_v7 or use_v8) else "aligned",
            think_steps=eval_think_steps if max_think_steps else None,
            amp_enabled=use_amp,
            amp_dtype=amp_dtype,
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
        tile_exact = val_metrics.get("tile_exact")
        tile_near = val_metrics.get("tile_near")
        tile_extra = ""
        if tile_exact is not None and tile_near is not None:
            tile_extra = f"  val_tile_exact={tile_exact:.3f}  val_tile_near={tile_near:.3f}"
        elif tile_class is not None and tile_top5 is not None:
            tile_extra = f"  val_tile_cls={tile_class:.3f}  val_tile@5={tile_top5:.3f}"
        val_loss = float(val_metrics["loss"])
        val_loss_text = f"{val_loss:.4f}" if math.isfinite(val_loss) else "nonfinite"
        epoch_work = 100.0 * epoch / max(epochs, 1)
        elapsed = time.time() - run_started
        done_this_run = max(epoch - start_epoch + 1, 1)
        remaining_epochs = max(epochs - epoch, 0)
        eta_seconds = elapsed * remaining_epochs / done_this_run
        dashboard.log(
            f"epoch {epoch:02d}  train_loss={train_loss:.4f}  "
            f"val_slot@1={val_metrics['slot_top1']:.3f}  "
            f"val_zone={val_metrics['zone_acc']:.3f}"
            f"{tile_extra}  "
            f"val_loss={val_loss_text}"
        )
        with (output / "training_stages.json").open("w", encoding="utf-8") as handle:
            json.dump(history, handle, indent=2)
        dashboard.update(
            phase="validation",
            epoch=epoch,
            loss=train_loss,
            acc=val_metrics["slot_top1"],
            card=val_metrics.get("card", val_metrics["slot_top1"]),
            val_acc=val_metrics["slot_top1"],
            tile_exact=val_metrics.get("tile_exact"),
            tile_near=val_metrics.get("tile_near"),
            wait=val_metrics.get("wait"),
            wait_mae=val_metrics.get("wait_mae") or val_metrics["timing_mae"],
            tile=val_metrics.get("tile_class_acc") or val_metrics["tile_acc"],
            zone=val_metrics["zone_acc"],
            timing=val_metrics["timing_mae"],
            clock=val_metrics.get("clock_nll"),
            progress_percent=epoch_work,
            eta_seconds=eta_seconds,
        )
        append_table_row(
            "validation", f"{epoch}/{epochs}", "epoch-end", epoch_work,
            f"{min(epoch * len(train_loader.dataset), epochs * len(train_loader.dataset)):,}/"
            f"{epochs * len(train_loader.dataset):,}",
            eta_seconds, loss_value=train_loss,
            card_value=val_metrics["slot_top1"], zone_value=val_metrics["zone_acc"],
            tile_value=val_metrics.get("tile_exact") or val_metrics["tile_acc"], timing_value=val_metrics["timing_mae"],
        )
        dashboard.log(f"writing checkpoint for epoch {epoch:02d}")
        if math.isfinite(val_loss):
            epoch_state = cpu_state_dict(raw_model)
            save_checkpoint(
                output / "last.pt",
                epoch_state,
                epoch,
                val_loss,
                include_optim=True,
            )
            if val_loss < best_val - 1e-4:
                best_val = val_loss
                best_epoch = epoch
                best_state = epoch_state
                bad_epochs = 0
                save_checkpoint(
                    output / "best_model.pt",
                    best_state,
                    epoch,
                    best_val,
                    include_optim=True,
                )
                dashboard.log(f"saved best checkpoint  val_loss={best_val:.4f}")
            else:
                bad_epochs += 1
                dashboard.log(f"saved last checkpoint  epoch={epoch:02d}")
                if bad_epochs >= patience:
                    dashboard.log(f"Early stop at epoch {epoch}")
                    break
        else:
            dashboard.log("val objective non-finite; not counting toward early stop")

    if best_state is not None:
        raw_model.load_state_dict(best_state)
    selected_val_metrics = evaluate_policy(model, val_loader, device, loss_kwargs=loss_kwargs,
        think_steps=eval_think_steps, amp_enabled=use_amp, amp_dtype=amp_dtype)

    dashboard.log("evaluating held-out test split")
    test_metrics = evaluate_policy(
        model,
        test_loader,
        device,
        loss_kwargs=loss_kwargs,
        arena_control=arena_control if (use_v7 or use_v8) else "aligned",
        think_steps=eval_think_steps if max_think_steps else None,
        amp_enabled=use_amp,
        amp_dtype=amp_dtype,
    )
    test_metrics_think_off: dict[str, Any] | None = None
    if max_think_steps > 0 and eval_think_steps != 0:
        test_metrics_think_off = evaluate_policy(
            model,
            test_loader,
            device,
            loss_kwargs=loss_kwargs,
            arena_control=arena_control if (use_v7 or use_v8) else "aligned",
            think_steps=0,
            amp_enabled=use_amp,
            amp_dtype=amp_dtype,
        )
    baselines = baselines_for_dataset(test_loader.dataset)
    freq_base = baselines["frequency"]
    cycle_base = baselines["cycle"]

    realism_path = Path(realism_model_dir) / "realism_ensemble.pkl"
    # v7 is an offline placement isolation experiment; avoid loading the
    # optional sklearn realism artifact and keep its environment-independent.
    realism_artifact = None if spec.skip_realism else _load_realism_scorer(realism_path)
    rollout_stats: dict[str, Any] = {"available": False}
    if realism_artifact is not None:
        dashboard.log("rolling out policy continuations for realism scoring")
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
    final_epoch = int(history[-1]["epoch"]) if history else max(start_epoch - 1, 0)
    final_state = best_state if best_state is not None else cpu_state_dict(raw_model)
    save_checkpoint(
        ckpt_path,
        final_state,
        best_epoch or final_epoch,
        best_val if math.isfinite(best_val) else float("inf"),
        include_optim=False,
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
    v8_lessons = [
        "v8 Stage A replaces the GRU trunk with a 24-token time-aware belief memory and a joint own/opponent event clock.",
        "Memory tokens attend to friendly vs enemy events, mix through two interaction blocks, and evolve with elapsed query time.",
        "Action heads keep the selected-card 18×32 heatmap plus a within-tile offset. Waiting is part of the likelihood, not a missing action.",
        "This run is replay-only Stage A. Board observations (Stage B) are required before claiming a large playing-strength gain.",
        "Judge on joint card+tile, clock NLL / timing MAE, and shuffled-memory controls — not a live win rate.",
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
    if use_v8:
        lessons = v8_lessons
    elif use_v7:
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
            "memory_tokens": spec.memory_tokens,
            "memory_heads": spec.memory_heads,
            "interaction_blocks": spec.interaction_blocks,
            "event_clock": spec.event_clock,
            "mixed_precision": use_amp,
            "amp_dtype": "bf16" if amp_dtype == torch.bfloat16 else "fp32",
            "compiled": compiled,
            "resume": resume,
            "resumed_from_epoch": start_epoch - 1 if resume else 0,
            "peak_vram_mb": (
                float(torch.cuda.max_memory_allocated()) / (1024.0 ** 2)
                if device.type == "cuda"
                else 0.0
            ),
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
        "last_epoch_val": {
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
                "joint_card_tile",
                "clock_nll",
                "timing_mae",
                "model_x_std",
                "model_y_std",
            )
            if f"val_{k}" in history[-1]
        }
        if history
        else {},
        "test": test_metrics,
        "val": selected_val_metrics,
        "weights_epoch": best_epoch or final_epoch,
        "last_completed_epoch": final_epoch,
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
            offline_only=use_v6 or use_v8,
        ),
    }

    report_path = output / "report.json"
    with report_path.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    with (output / "training_stages.json").open("w", encoding="utf-8") as handle:
        json.dump(history, handle, indent=2)

    dashboard.log(f"wrote {report_path}")
    dashboard.close()
    if use_v8:
        from .policy_v8_report import render_policy_v8_report

        html_path = Path("reports") / "policy_bc_v8.html"
        try:
            render_policy_v8_report(output, html_path)
            print(f"Wrote {html_path}", flush=True)
        except Exception as exc:
            print(f"v8 HTML report deferred: {exc}", flush=True)

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
