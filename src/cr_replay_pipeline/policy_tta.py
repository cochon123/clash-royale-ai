"""Test-time augmentation for horizontally symmetric policy inference."""

from __future__ import annotations

import json
import math
import os
import random
import time
from pathlib import Path
from typing import Any

import numpy as np
import torch
import torch.nn.functional as F
from torch.utils.data import DataLoader

from .policy_dataset import (
    TILE_UNITS,
    PolicyActionDataset,
    collate_policy_batch,
    collect_battles,
    load_card_costs,
)
from .policy_infer import load_policy
from .policy_model import TILE_COLS, TILE_ROWS
from .policy_train import _MirroredBattle, _move_batch


ZONE_MIRROR = (2, 1, 0, 5, 4, 3, 8, 7, 6, 11, 10, 9)


def _paired_exact_p(corrected: int, broken: int) -> float:
    """Two-sided exact McNemar/binomial p-value for paired decisions."""
    discordant = corrected + broken
    if discordant == 0:
        return 1.0
    tail = min(corrected, broken)
    probability = 2.0 * sum(math.comb(discordant, i) for i in range(tail + 1)) / (
        2**discordant
    )
    return min(float(probability), 1.0)


def _mean_prob_logits(a: torch.Tensor, b: torch.Tensor) -> torch.Tensor:
    """Average categorical probabilities and return stable equivalent logits."""
    probs = 0.5 * (F.softmax(a, dim=-1) + F.softmax(b, dim=-1))
    return probs.clamp_min(1e-8).log()


def mirror_ensemble_outputs(
    original: dict[str, torch.Tensor],
    mirrored: dict[str, torch.Tensor],
) -> dict[str, torch.Tensor | None]:
    """Map a mirrored prediction home and average it with the original pass."""
    zone_index = torch.tensor(ZONE_MIRROR, device=mirrored["zone_logits"].device)
    mirrored_zone_logits = mirrored["zone_logits"].index_select(-1, zone_index)
    mirrored_xy = torch.stack(
        [1.0 - mirrored["xy"][:, 0], mirrored["xy"][:, 1]], dim=-1
    )

    tile_logits = None
    if original.get("tile_logits") is not None and mirrored.get("tile_logits") is not None:
        mirrored_tiles = mirrored["tile_logits"].reshape(-1, TILE_ROWS, TILE_COLS).flip(-1)
        tile_logits = _mean_prob_logits(
            original["tile_logits"], mirrored_tiles.flatten(1)
        )

    slot_logits = _mean_prob_logits(original["slot_logits"], mirrored["slot_logits"])
    return {
        "slot_logits": slot_logits,
        "type_logits": _mean_prob_logits(original["type_logits"], mirrored["type_logits"]),
        "zone_logits": _mean_prob_logits(original["zone_logits"], mirrored_zone_logits),
        "tile_logits": tile_logits,
        "xy": 0.5 * (original["xy"] + mirrored_xy),
        "timing": 0.5 * (original["timing"] + mirrored["timing"]),
        "slot_probs": F.softmax(slot_logits, dim=-1),
        "fused": original["fused"],
    }


class _MetricAccumulator:
    def __init__(self) -> None:
        self.loss = 0.0
        self.batches = 0
        self.total = 0
        self.slot_correct = 0
        self.slot_top3 = 0
        self.type_correct = 0
        self.zone_correct = 0
        self.xy_error = 0.0
        self.tile_hits = 0
        self.timing_error = 0.0
        self.xy_rows: list[np.ndarray] = []

    def update(
        self,
        model,
        outputs: dict[str, torch.Tensor | None],
        slots: torch.Tensor,
        types: torch.Tensor,
        zones: torch.Tensor,
        xy: torch.Tensor,
        timing: torch.Tensor,
        loss_kwargs: dict[str, float],
    ) -> None:
        losses = model.loss(outputs, slots, types, zones, xy, timing, **loss_kwargs)
        self.loss += float(losses["loss"].item())
        self.batches += 1
        self.total += int(slots.size(0))
        slot_logits = outputs["slot_logits"]
        type_logits = outputs["type_logits"]
        zone_logits = outputs["zone_logits"]
        pred_xy_tensor = outputs["xy"]
        pred_timing = outputs["timing"]
        assert slot_logits is not None
        assert type_logits is not None
        assert zone_logits is not None
        assert pred_xy_tensor is not None
        assert pred_timing is not None
        self.slot_correct += int((slot_logits.argmax(-1) == slots).sum().item())
        self.slot_top3 += int(
            (slot_logits.topk(3, dim=-1).indices == slots.unsqueeze(-1))
            .any(dim=-1)
            .sum()
            .item()
        )
        self.type_correct += int((type_logits.argmax(-1) == types).sum().item())
        self.zone_correct += int((zone_logits.argmax(-1) == zones).sum().item())

        pred_xy = pred_xy_tensor.detach().cpu().numpy()
        true_xy = xy.detach().cpu().numpy()
        dx = (pred_xy[:, 0] - true_xy[:, 0]) * 18000.0
        dy = (pred_xy[:, 1] - true_xy[:, 1]) * 32000.0
        distance = np.sqrt(dx * dx + dy * dy)
        self.xy_error += float(distance.sum())
        self.tile_hits += int((distance <= TILE_UNITS).sum())
        self.xy_rows.append(pred_xy)

        pred_dt = np.expm1(pred_timing.detach().cpu().numpy())
        true_dt = np.expm1(timing.detach().cpu().numpy())
        self.timing_error += float(np.abs(pred_dt - true_dt).sum())

    def finish(self) -> dict[str, Any]:
        total = max(self.total, 1)
        xy = np.concatenate(self.xy_rows) if self.xy_rows else None
        return {
            "loss": self.loss / max(self.batches, 1),
            "slot_top1": self.slot_correct / total,
            "slot_top3": self.slot_top3 / total,
            "type_acc": self.type_correct / total,
            "zone_acc": self.zone_correct / total,
            "xy_mae": self.xy_error / total,
            "tile_acc": self.tile_hits / total,
            "timing_mae": self.timing_error / total,
            "n": self.total,
            "model_x_std": float(xy[:, 0].std()) if xy is not None else None,
            "model_y_std": float(xy[:, 1].std()) if xy is not None else None,
        }


@torch.no_grad()
def evaluate_mirror_ensemble(
    model,
    original_loader: DataLoader,
    mirrored_loader: DataLoader,
    device: torch.device,
    loss_kwargs: dict[str, float],
) -> dict[str, Any]:
    """Evaluate baseline and two-pass TTA on paired original/mirrored batches."""
    model.eval()
    baseline = _MetricAccumulator()
    ensemble = _MetricAccumulator()
    baseline_seconds = 0.0
    ensemble_seconds = 0.0
    timed_samples = 0
    card_changes = card_gains = card_losses = 0
    zone_changes = zone_gains = zone_losses = 0

    started = time.perf_counter()
    for batch_index, (batch, mirrored_batch) in enumerate(
        zip(original_loader, mirrored_loader, strict=True)
    ):
        moved = _move_batch(batch, device)
        moved_mirror = _move_batch(mirrored_batch, device)
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
        ) = moved
        (
            mirror_continuous,
            mirror_card_ids,
            mirror_team_deck,
            mirror_opp_deck,
            mirror_globals,
            mirror_slot_feats,
            mirror_hand_mask,
            mirror_slots,
            mirror_types,
            mirror_zones,
            mirror_xy,
            mirror_timing,
            mirror_lengths,
            _mirror_weights,
        ) = moved_mirror

        if not torch.equal(slots, mirror_slots) or not torch.equal(types, mirror_types):
            raise RuntimeError("Original and mirrored test samples are not aligned")
        if not torch.equal(lengths, mirror_lengths):
            raise RuntimeError("Original and mirrored context lengths differ")
        expected_mirror_zones = torch.tensor(
            ZONE_MIRROR, device=device, dtype=torch.long
        )[zones]
        if not torch.equal(mirror_zones, expected_mirror_zones):
            raise RuntimeError("Mirrored zone labels are inconsistent")
        if not torch.allclose(mirror_xy[:, 0], 1.0 - xy[:, 0], atol=1e-6):
            raise RuntimeError("Mirrored x labels are inconsistent")
        if not torch.allclose(mirror_xy[:, 1], xy[:, 1], atol=1e-6):
            raise RuntimeError("Mirrored y labels are inconsistent")
        if not torch.allclose(mirror_timing, timing, atol=1e-6):
            raise RuntimeError("Mirrored timing labels are inconsistent")

        if device.type == "cuda":
            torch.cuda.synchronize(device)
        t0 = time.perf_counter()
        original_out = model(
            continuous,
            card_ids,
            team_deck,
            opp_deck,
            globals_,
            lengths,
            slot_feats,
            hand_mask,
        )
        if device.type == "cuda":
            torch.cuda.synchronize(device)
        t1 = time.perf_counter()
        mirrored_out = model(
            mirror_continuous,
            mirror_card_ids,
            mirror_team_deck,
            mirror_opp_deck,
            mirror_globals,
            mirror_lengths,
            mirror_slot_feats,
            mirror_hand_mask,
        )
        combined = mirror_ensemble_outputs(original_out, mirrored_out)
        if device.type == "cuda":
            torch.cuda.synchronize(device)
        t2 = time.perf_counter()

        # Skip the first two batches from timing so CUDA warm-up is excluded.
        if batch_index >= 2:
            baseline_seconds += t1 - t0
            ensemble_seconds += t2 - t0
            timed_samples += int(slots.size(0))

        baseline.update(
            model, original_out, slots, types, zones, xy, timing, loss_kwargs
        )
        ensemble.update(model, combined, slots, types, zones, xy, timing, loss_kwargs)

        base_card = original_out["slot_logits"].argmax(-1)
        tta_card = combined["slot_logits"].argmax(-1)
        base_zone = original_out["zone_logits"].argmax(-1)
        tta_zone = combined["zone_logits"].argmax(-1)
        card_changes += int((base_card != tta_card).sum().item())
        card_gains += int(((base_card != slots) & (tta_card == slots)).sum().item())
        card_losses += int(((base_card == slots) & (tta_card != slots)).sum().item())
        zone_changes += int((base_zone != tta_zone).sum().item())
        zone_gains += int(((base_zone != zones) & (tta_zone == zones)).sum().item())
        zone_losses += int(((base_zone == zones) & (tta_zone != zones)).sum().item())

    baseline_metrics = baseline.finish()
    ensemble_metrics = ensemble.finish()
    metric_keys = (
        "loss",
        "slot_top1",
        "slot_top3",
        "zone_acc",
        "xy_mae",
        "tile_acc",
        "timing_mae",
    )
    deltas = {
        key: ensemble_metrics[key] - baseline_metrics[key] for key in metric_keys
    }
    return {
        "baseline": baseline_metrics,
        "mirror_ensemble": ensemble_metrics,
        "delta_ensemble_minus_baseline": deltas,
        "decision_changes": {
            "card_changed": card_changes,
            "card_corrected": card_gains,
            "card_broken": card_losses,
            "card_paired_exact_p": _paired_exact_p(card_gains, card_losses),
            "zone_changed": zone_changes,
            "zone_corrected": zone_gains,
            "zone_broken": zone_losses,
            "zone_paired_exact_p": _paired_exact_p(zone_gains, zone_losses),
        },
        "latency": {
            "timed_samples": timed_samples,
            "baseline_batch_compute_seconds": baseline_seconds,
            "ensemble_batch_compute_seconds": ensemble_seconds,
            "compute_multiplier": ensemble_seconds / max(baseline_seconds, 1e-9),
            "baseline_samples_per_second": timed_samples / max(baseline_seconds, 1e-9),
            "ensemble_samples_per_second": timed_samples / max(ensemble_seconds, 1e-9),
            "evaluation_wall_seconds": time.perf_counter() - started,
        },
    }


def run_v42_mirror_tta_evaluation(
    model_dir: str | Path = "models/policy_bc_v4.2_full",
    input_dir: str | Path = "data/raw",
    card_costs_path: str | Path = "data/card_costs.json",
    output_path: str | Path = "reports/policy_bc_v4_2_mirror_tta.json",
    device_name: str | None = None,
) -> dict[str, Any]:
    """Reproduce v4.2's test split and evaluate horizontal-mirror TTA."""
    model_dir = Path(model_dir)
    report = json.loads((model_dir / "report.json").read_text(encoding="utf-8"))
    model, vocab, cfg, device = load_policy(model_dir, device_name=device_name)
    expected_source_battles = int(
        report["data"]["splits"][0]["battles"] // 2
        + report["data"]["splits"][1]["battles"]
        + report["data"]["splits"][2]["battles"]
    )

    # Collection may continue after training. Trust the cache deliberately: it
    # is the historical 37,011-battle snapshot used by this checkpoint.
    os.environ["CR_REPLAY_TRUST_CACHE"] = "1"
    battles = collect_battles(input_dir, min_card_plays=int(report["data"]["min_card_plays"]))
    if len(battles) != expected_source_battles:
        raise RuntimeError(
            f"Historical cache has {len(battles):,} battles; expected "
            f"{expected_source_battles:,} for this checkpoint"
        )
    ordered = list(battles)
    random.Random(42).shuffle(ordered)
    n_train = int(len(ordered) * 0.9)
    n_val = int((len(ordered) - n_train) / 2)
    test_battles = ordered[n_train + n_val :]
    mirrored_battles = [_MirroredBattle(battle) for battle in test_battles]
    costs = load_card_costs(card_costs_path)
    dataset_kwargs = {
        "vocab": vocab,
        "costs": costs,
        "max_context": int(cfg.get("max_context", 64)),
        "max_samples_per_battle": min(24, int(cfg.get("max_samples_per_battle", 40))),
        "stride": 3,
        "seed": 33,
        "threat_dim": int(cfg.get("threat_dim", 0)),
        "reaction_seconds": float(cfg.get("reaction_seconds", 5.0)),
        "reaction_weight": 1.0,
        "prefer_reactions": False,
        "reaction_repeats": 1,
    }
    original_dataset = PolicyActionDataset(test_battles, **dataset_kwargs)
    mirrored_dataset = PolicyActionDataset(mirrored_battles, **dataset_kwargs)
    if original_dataset.index != mirrored_dataset.index:
        raise RuntimeError("Mirrored dataset selected different test samples")
    expected_test_samples = int(report["test"]["n"])
    if len(original_dataset) != expected_test_samples:
        raise RuntimeError(
            f"Reproduced {len(original_dataset):,} test samples; expected "
            f"{expected_test_samples:,}"
        )
    batch_size = int(cfg.get("batch_size", 512))
    original_loader = DataLoader(
        original_dataset,
        batch_size=batch_size,
        shuffle=False,
        collate_fn=collate_policy_batch,
        num_workers=0,
    )
    mirrored_loader = DataLoader(
        mirrored_dataset,
        batch_size=batch_size,
        shuffle=False,
        collate_fn=collate_policy_batch,
        num_workers=0,
    )
    loss_kwargs = {"zone_weight": 1.1, "xy_weight": 0.55, "slot_weight": 1.4}
    results = evaluate_mirror_ensemble(
        model, original_loader, mirrored_loader, device, loss_kwargs
    )
    results["setup"] = {
        "checkpoint": str(model_dir),
        "device": str(device),
        "parameters": int(report["compute"]["parameters"]),
        "source_battles": len(battles),
        "test_battles": len(test_battles),
        "test_samples": len(original_dataset),
        "batch_size": batch_size,
        "method": "two-pass horizontal mirror probability ensemble",
        "retrained": False,
    }
    archived = report["test"]
    results["baseline_reproduction"] = {
        key: results["baseline"][key] - archived[key]
        for key in ("loss", "slot_top1", "slot_top3", "zone_acc", "xy_mae", "tile_acc", "timing_mae")
    }
    output = Path(output_path)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(results, indent=2), encoding="utf-8")
    return results


if __name__ == "__main__":
    result = run_v42_mirror_tta_evaluation()
    print(json.dumps(result, indent=2))
