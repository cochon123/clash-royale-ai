"""Experiment B: card-conditioned placement probe + per-card lookup control.

Offline only. Freezes policy_bc_v3 trunk; trains a small MLP on
[fused | card_embed | threat] to predict zone (+ optional XY).
"""

from __future__ import annotations

import html
import json
import time
from collections import Counter, defaultdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import numpy as np
import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.optim import AdamW
from torch.utils.data import DataLoader, TensorDataset

from .policy_dataset import (
    DEFAULT_MAX_CONTEXT,
    DEFAULT_REACTION_SECONDS,
    GLOBAL_DIM,
    THREAT_DIM,
    TILE_UNITS,
    collect_battles,
    create_policy_dataloaders,
    load_card_costs,
    split_battles,
    summarize_split,
)
from .policy_infer import load_policy
from .policy_model import NUM_ZONES, PolicyBC
from .policy_train import _move_batch

class PlacementProbe(nn.Module):
    """2-layer MLP: fused + card embed + threat → zone (+ XY)."""

    def __init__(
        self,
        d_model: int,
        card_embed_dim: int,
        threat_dim: int = THREAT_DIM,
        num_zones: int = NUM_ZONES,
        hidden: int = 256,
        dropout: float = 0.1,
    ):
        super().__init__()
        in_dim = d_model + card_embed_dim + threat_dim
        self.threat_dim = threat_dim
        self.backbone = nn.Sequential(
            nn.Linear(in_dim, hidden),
            nn.LayerNorm(hidden),
            nn.GELU(),
            nn.Dropout(dropout),
            nn.Linear(hidden, hidden),
            nn.LayerNorm(hidden),
            nn.GELU(),
            nn.Dropout(dropout),
        )
        self.zone_head = nn.Linear(hidden, num_zones)
        self.xy_head = nn.Linear(hidden + num_zones, 2)

    def forward(
        self, fused: torch.Tensor, card_embed: torch.Tensor, threat: torch.Tensor
    ) -> dict[str, torch.Tensor]:
        h = self.backbone(torch.cat([fused, card_embed, threat], dim=-1))
        zone_logits = self.zone_head(h)
        zone_probs = F.softmax(zone_logits, dim=-1)
        xy = torch.sigmoid(self.xy_head(torch.cat([h, zone_probs], dim=-1)))
        return {"zone_logits": zone_logits, "xy": xy}


def _xy_mae_raw(pred_xy: np.ndarray, true_xy: np.ndarray) -> tuple[float, float]:
    dx = (pred_xy[:, 0] - true_xy[:, 0]) * 18000.0
    dy = (pred_xy[:, 1] - true_xy[:, 1]) * 32000.0
    dist = np.sqrt(dx * dx + dy * dy)
    return float(dist.mean()), float((dist <= TILE_UNITS).mean())


@torch.no_grad()
def extract_placement_cache(
    model: PolicyBC,
    loader: DataLoader,
    device: torch.device,
    threat_dim: int,
) -> dict[str, torch.Tensor]:
    """Run frozen trunk once; cache fused / targets / slot preds / card ids."""
    model.eval()
    fused_l: list[torch.Tensor] = []
    threat_l: list[torch.Tensor] = []
    zone_l: list[torch.Tensor] = []
    xy_l: list[torch.Tensor] = []
    card_true_l: list[torch.Tensor] = []
    card_pred_l: list[torch.Tensor] = []
    zone_v3_l: list[torch.Tensor] = []
    xy_v3_l: list[torch.Tensor] = []

    for batch in loader:
        (
            continuous,
            card_ids,
            team_deck,
            opp_deck,
            globals_,
            slot_feats,
            hand_mask,
            slots,
            _types,
            zones,
            xy,
            _timing,
            lengths,
            _weights,
        ) = _move_batch(batch, device)
        out = model(
            continuous,
            card_ids,
            team_deck,
            opp_deck,
            globals_,
            lengths,
            slot_feats,
            hand_mask,
        )
        if threat_dim > 0:
            threat = globals_[:, -threat_dim:]
        else:
            threat = torch.zeros(globals_.size(0), THREAT_DIM, device=device)

        slot_pred = out["slot_logits"].argmax(dim=-1)
        batch_idx = torch.arange(slots.size(0), device=device)
        card_true = team_deck[batch_idx, slots]
        card_pred = team_deck[batch_idx, slot_pred]

        fused_l.append(out["fused"].detach().cpu())
        threat_l.append(threat.detach().cpu())
        zone_l.append(zones.detach().cpu())
        xy_l.append(xy.detach().cpu())
        card_true_l.append(card_true.detach().cpu())
        card_pred_l.append(card_pred.detach().cpu())
        zone_v3_l.append(out["zone_logits"].argmax(dim=-1).detach().cpu())
        xy_v3_l.append(out["xy"].detach().cpu())

    return {
        "fused": torch.cat(fused_l),
        "threat": torch.cat(threat_l),
        "zone": torch.cat(zone_l),
        "xy": torch.cat(xy_l),
        "card_true": torch.cat(card_true_l),
        "card_pred": torch.cat(card_pred_l),
        "zone_v3": torch.cat(zone_v3_l),
        "xy_v3": torch.cat(xy_v3_l),
    }


def build_per_card_table(
    card_ids: torch.Tensor, zones: torch.Tensor, xy: torch.Tensor
) -> dict[str, Any]:
    """Per-card majority zone + mean XY from train targets."""
    zones_np = zones.numpy()
    xy_np = xy.numpy()
    cards_np = card_ids.numpy()

    zone_counts: dict[int, Counter] = defaultdict(Counter)
    xy_sums: dict[int, np.ndarray] = defaultdict(lambda: np.zeros(2, dtype=np.float64))
    xy_n: dict[int, int] = defaultdict(int)

    for card, zone, xy_row in zip(cards_np, zones_np, xy_np):
        cid = int(card)
        zone_counts[cid][int(zone)] += 1
        xy_sums[cid] += xy_row.astype(np.float64)
        xy_n[cid] += 1

    global_zone = int(Counter(int(z) for z in zones_np).most_common(1)[0][0])
    global_xy = xy_np.mean(axis=0).astype(np.float64)

    majority_zone: dict[int, int] = {}
    mean_xy: dict[int, list[float]] = {}
    for cid, counts in zone_counts.items():
        majority_zone[cid] = int(counts.most_common(1)[0][0])
        mean_xy[cid] = (xy_sums[cid] / max(xy_n[cid], 1)).tolist()

    return {
        "majority_zone": majority_zone,
        "mean_xy": mean_xy,
        "global_zone": global_zone,
        "global_xy": global_xy.tolist(),
        "n_cards": len(majority_zone),
        "n_samples": int(len(cards_np)),
    }


def score_lookup_table(
    table: dict[str, Any],
    card_ids: torch.Tensor,
    zones: torch.Tensor,
    xy: torch.Tensor,
) -> dict[str, float]:
    maj = table["majority_zone"]
    mean_xy = table["mean_xy"]
    g_zone = int(table["global_zone"])
    g_xy = np.asarray(table["global_xy"], dtype=np.float64)

    cards = card_ids.numpy()
    true_z = zones.numpy()
    true_xy = xy.numpy()
    pred_z = np.empty(len(cards), dtype=np.int64)
    pred_xy = np.empty((len(cards), 2), dtype=np.float64)
    for i, cid in enumerate(cards):
        key = int(cid)
        pred_z[i] = maj.get(key, g_zone)
        pred_xy[i] = np.asarray(mean_xy.get(key, g_xy), dtype=np.float64)

    zone_acc = float((pred_z == true_z).mean())
    xy_mae, tile_acc = _xy_mae_raw(pred_xy, true_xy)
    return {
        "zone_acc": zone_acc,
        "xy_mae": xy_mae,
        "tile_acc": tile_acc,
        "n": int(len(cards)),
    }


def score_v3_cache(cache: dict[str, torch.Tensor]) -> dict[str, float]:
    zone_acc = float((cache["zone_v3"].numpy() == cache["zone"].numpy()).mean())
    xy_mae, tile_acc = _xy_mae_raw(cache["xy_v3"].numpy(), cache["xy"].numpy())
    return {
        "zone_acc": zone_acc,
        "xy_mae": xy_mae,
        "tile_acc": tile_acc,
        "n": int(cache["zone"].size(0)),
    }


def _probe_loader(
    cache: dict[str, torch.Tensor], batch_size: int, shuffle: bool
) -> DataLoader:
    ds = TensorDataset(
        cache["fused"],
        cache["threat"],
        cache["card_true"],
        cache["card_pred"],
        cache["zone"],
        cache["xy"],
    )
    return DataLoader(ds, batch_size=batch_size, shuffle=shuffle)


def train_probe(
    probe: PlacementProbe,
    trunk: PolicyBC,
    train_cache: dict[str, torch.Tensor],
    val_cache: dict[str, torch.Tensor],
    device: torch.device,
    epochs: int = 10,
    batch_size: int = 512,
    lr: float = 1e-3,
    xy_weight: float = 0.35,
) -> list[dict[str, float]]:
    probe.to(device)
    probe.train()
    for p in trunk.parameters():
        p.requires_grad_(False)
    trunk.eval()

    opt = AdamW(probe.parameters(), lr=lr, weight_decay=1e-4)
    history: list[dict[str, float]] = []
    train_loader = _probe_loader(train_cache, batch_size, shuffle=True)

    for epoch in range(1, epochs + 1):
        probe.train()
        running = 0.0
        n_batches = 0
        for fused, threat, card_true, _card_pred, zones, xy in train_loader:
            fused = fused.to(device)
            threat = threat.to(device)
            card_true = card_true.to(device)
            zones = zones.to(device)
            xy = xy.to(device)
            with torch.no_grad():
                card_embed = trunk.card_embedding(card_true)
            opt.zero_grad(set_to_none=True)
            out = probe(fused, card_embed, threat)
            zone_loss = F.cross_entropy(out["zone_logits"], zones, label_smoothing=0.02)
            xy_loss = F.smooth_l1_loss(out["xy"], xy)
            loss = zone_loss + xy_weight * xy_loss
            loss.backward()
            nn.utils.clip_grad_norm_(probe.parameters(), 1.0)
            opt.step()
            running += float(loss.item())
            n_batches += 1

        val_oracle = evaluate_probe(
            probe, trunk, val_cache, device, condition="oracle", batch_size=batch_size
        )
        row = {
            "epoch": epoch,
            "train_loss": running / max(n_batches, 1),
            "val_zone_acc": val_oracle["zone_acc"],
            "val_xy_mae": val_oracle["xy_mae"],
            "val_tile_acc": val_oracle["tile_acc"],
        }
        history.append(row)
        print(
            f"probe epoch {epoch:02d}  loss={row['train_loss']:.4f}  "
            f"val_zone={row['val_zone_acc']:.3f}  val_xy_mae={row['val_xy_mae']:.0f}",
            flush=True,
        )
    return history


@torch.no_grad()
def evaluate_probe(
    probe: PlacementProbe,
    trunk: PolicyBC,
    cache: dict[str, torch.Tensor],
    device: torch.device,
    condition: str = "oracle",
    batch_size: int = 512,
) -> dict[str, float]:
    """condition: oracle (true card) or e2e (v3 slot-argmax card)."""
    probe.eval()
    trunk.eval()
    loader = _probe_loader(cache, batch_size, shuffle=False)
    zone_correct = 0
    total = 0
    pred_xy_l: list[np.ndarray] = []
    true_xy_l: list[np.ndarray] = []

    for fused, threat, card_true, card_pred, zones, xy in loader:
        fused = fused.to(device)
        threat = threat.to(device)
        zones = zones.to(device)
        card_ids = card_true.to(device) if condition == "oracle" else card_pred.to(device)
        card_embed = trunk.card_embedding(card_ids)
        out = probe(fused, card_embed, threat)
        zone_correct += int((out["zone_logits"].argmax(dim=-1) == zones).sum().item())
        total += int(zones.size(0))
        pred_xy_l.append(out["xy"].cpu().numpy())
        true_xy_l.append(xy.numpy())

    pred_xy = np.concatenate(pred_xy_l)
    true_xy = np.concatenate(true_xy_l)
    xy_mae, tile_acc = _xy_mae_raw(pred_xy, true_xy)
    return {
        "zone_acc": zone_correct / max(total, 1),
        "xy_mae": xy_mae,
        "tile_acc": tile_acc,
        "n": total,
        "condition": condition,
    }


def _decide(
    control: dict[str, float],
    oracle: dict[str, float],
    e2e: dict[str, float],
    v3: dict[str, float],
) -> dict[str, Any]:
    ctrl_z = float(control["zone_acc"])
    ora_z = float(oracle["zone_acc"])
    e2e_z = float(e2e["zone_acc"])
    v3_z = float(v3["zone_acc"])
    ora_xy = float(oracle["xy_mae"])

    oracle_vs_table_pp = (ora_z - ctrl_z) * 100.0
    e2e_vs_v3_pp = (e2e_z - v3_z) * 100.0

    success_oracle = ora_z >= 0.45 and oracle_vs_table_pp >= 3.0
    success_e2e = e2e_vs_v3_pp >= 3.0
    success_xy = ora_xy <= 5000.0
    hard_fail = oracle_vs_table_pp < 2.0

    if hard_fail:
        verdict = "FAIL"
        recommendation = (
            "Oracle probe does not beat the per-card lookup table by ≥2pp. "
            "Placement needs board / spatial features, not just card conditioning."
        )
    elif success_oracle and success_e2e and success_xy:
        verdict = "PASS"
        recommendation = (
            "Card conditioning lifts placement offline. Next: wire card-conditioned "
            "placement heads into a small v3.1 fine-tune (freeze trunk, train heads), "
            "still offline — do not live-play yet."
        )
    elif success_oracle:
        missed = []
        if not success_e2e:
            missed.append(f"e2e zone only {e2e_vs_v3_pp:+.1f}pp vs v3 (need ≥+3pp)")
        if not success_xy:
            missed.append(f"oracle XY MAE {ora_xy:.0f} (need ≤5000)")
        verdict = "PARTIAL"
        recommendation = (
            "Oracle card conditioning clearly helps placement "
            f"(zone {ora_z:.1%}, {oracle_vs_table_pp:+.1f}pp vs table). "
            + (
                "Missed: " + "; ".join(missed) + ". "
                if missed
                else ""
            )
            + "Next offline: joint slot→placement training or teacher-forced card "
            "conditioning in a frozen-trunk head fine-tune — not live play."
        )
    else:
        verdict = "FAIL"
        recommendation = (
            "Card-conditioned probe underperformed success gates. Prefer richer "
            "board/threat spatial features over more card-only placement capacity."
        )

    return {
        "verdict": verdict,
        "recommendation": recommendation,
        "gates": {
            "oracle_zone_ge_45": ora_z >= 0.45,
            "oracle_ge_3pp_over_table": oracle_vs_table_pp >= 3.0,
            "e2e_ge_3pp_over_v3": e2e_vs_v3_pp >= 3.0,
            "oracle_xy_mae_le_5000": success_xy,
            "hard_fail_oracle_lt_2pp_over_table": hard_fail,
        },
        "deltas_pp": {
            "oracle_vs_table": oracle_vs_table_pp,
            "e2e_vs_v3": e2e_vs_v3_pp,
            "oracle_vs_v3": (ora_z - v3_z) * 100.0,
        },
    }


# HTML rendering lives in placement_probe_report.py

def _per_card_highlights(
    table: dict[str, Any],
    trunk: PolicyBC,
    probe: PlacementProbe,
    cache: dict[str, torch.Tensor],
    vocab,
    device: torch.device,
    top_n: int = 12,
) -> list[dict[str, Any]]:
    """Oracle vs table zone accuracy for frequent test cards."""
    cards = cache["card_true"].numpy()
    zones = cache["zone"].numpy()
    counts = Counter(int(c) for c in cards)
    top = [cid for cid, _ in counts.most_common(top_n)]

    maj = table["majority_zone"]
    g_zone = int(table["global_zone"])
    rows: list[dict[str, Any]] = []

    probe.eval()
    trunk.eval()
    with torch.no_grad():
        for cid in top:
            mask = cards == cid
            n = int(mask.sum())
            if n < 20:
                continue
            true_z = zones[mask]
            table_pred = maj.get(cid, g_zone)
            table_acc = float((true_z == table_pred).mean())

            idx = np.where(mask)[0]
            fused = cache["fused"][idx].to(device)
            threat = cache["threat"][idx].to(device)
            card_t = torch.full((len(idx),), cid, dtype=torch.long, device=device)
            out = probe(fused, trunk.card_embedding(card_t), threat)
            ora_acc = float(
                (out["zone_logits"].argmax(dim=-1).cpu().numpy() == true_z).mean()
            )
            rows.append(
                {
                    "card": _card_name(vocab, cid),
                    "card_id": cid,
                    "n": n,
                    "table_zone_acc": table_acc,
                    "oracle_zone_acc": ora_acc,
                    "lift_pp": (ora_acc - table_acc) * 100.0,
                }
            )
    rows.sort(key=lambda r: -r["n"])
    return rows


def _card_name(vocab, cid: int) -> str:
    inv = getattr(vocab, "id_to_name", None)
    if isinstance(inv, dict) and cid in inv:
        return str(inv[cid])
    mapping = getattr(vocab, "name_to_id", None)
    if isinstance(mapping, dict):
        for name, i in mapping.items():
            if int(i) == cid:
                return str(name)
    return str(cid)


def run_placement_probe(
    input_dir: str | Path = "data/raw",
    policy_dir: str | Path = "models/policy_bc_v3",
    card_costs_path: str | Path = "data/card_costs.json",
    output_json: str | Path = "reports/placement_probe_v1.json",
    output_html: str | Path = "reports/placement_probe_v1.html",
    epochs: int = 10,
    batch_size: int = 256,
    probe_batch_size: int = 512,
    lr: float = 1e-3,
    hidden: int = 256,
    min_card_plays: int = 12,
    seed: int = 42,
    device_name: str | None = None,
    max_samples_per_battle: int | None = 40,
) -> dict[str, Any]:
    started = time.time()
    created_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    torch.manual_seed(seed)
    np.random.seed(seed)

    model, vocab, cfg, device = load_policy(policy_dir, device_name=device_name)
    for p in model.parameters():
        p.requires_grad_(False)
    model.eval()

    threat_dim = int(cfg.get("threat_dim", THREAT_DIM))
    max_context = int(cfg.get("max_context", DEFAULT_MAX_CONTEXT))
    d_model = int(cfg.get("d_model", 160))
    card_embed_dim = int(model.card_embedding.embedding_dim)
    reaction_seconds = float(cfg.get("reaction_seconds", DEFAULT_REACTION_SECONDS))

    print(f"Loading battles from {input_dir} ...", flush=True)
    battles = collect_battles(input_dir, min_card_plays=min_card_plays)
    train_battles, val_battles, test_battles = split_battles(battles, seed=seed)
    costs = load_card_costs(card_costs_path)

    # Match v3 sample construction (no reaction oversampling for fair placement eval).
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
        reaction_weight=1.0,
        reaction_repeats=1,
    )
    print(
        f"Extracting frozen fused states on {device} "
        f"(train/val/test={len(train_loader.dataset)}/"
        f"{len(val_loader.dataset)}/{len(test_loader.dataset)}) ...",
        flush=True,
    )
    train_cache = extract_placement_cache(model, train_loader, device, threat_dim)
    val_cache = extract_placement_cache(model, val_loader, device, threat_dim)
    test_cache = extract_placement_cache(model, test_loader, device, threat_dim)

    print("Building per-card lookup control from TRAIN ...", flush=True)
    table = build_per_card_table(
        train_cache["card_true"], train_cache["zone"], train_cache["xy"]
    )
    control = score_lookup_table(
        table, test_cache["card_true"], test_cache["zone"], test_cache["xy"]
    )
    v3_metrics = score_v3_cache(test_cache)
    print(
        f"Control zone={control['zone_acc']:.3f}  "
        f"v3 zone={v3_metrics['zone_acc']:.3f}  "
        f"v3 xy_mae={v3_metrics['xy_mae']:.0f}",
        flush=True,
    )

    probe = PlacementProbe(
        d_model=d_model,
        card_embed_dim=card_embed_dim,
        threat_dim=threat_dim if threat_dim > 0 else THREAT_DIM,
        hidden=hidden,
    )
    n_probe = sum(p.numel() for p in probe.parameters())
    print(f"Training placement probe ({n_probe:,} params, lr={lr}) ...", flush=True)
    history = train_probe(
        probe,
        model,
        train_cache,
        val_cache,
        device,
        epochs=epochs,
        batch_size=probe_batch_size,
        lr=lr,
    )

    oracle = evaluate_probe(probe, model, test_cache, device, condition="oracle")
    e2e = evaluate_probe(probe, model, test_cache, device, condition="e2e")
    decision = _decide(control, oracle, e2e, v3_metrics)

    # Patch card names for highlights
    highlights = _per_card_highlights(
        table, model, probe, test_cache, vocab, device
    )

    lessons = [
        (
            f"Per-card lookup control zone={control['zone_acc']:.1%} is the number "
            "to beat for card-only placement."
        ),
        (
            f"Oracle probe zone={oracle['zone_acc']:.1%} "
            f"({decision['deltas_pp']['oracle_vs_table']:+.1f}pp vs table, "
            f"{decision['deltas_pp']['oracle_vs_v3']:+.1f}pp vs frozen v3)."
        ),
        (
            f"E2E conditioning on v3 slot-argmax card: zone={e2e['zone_acc']:.1%} "
            f"({decision['deltas_pp']['e2e_vs_v3']:+.1f}pp vs v3)."
        ),
        (
            f"Oracle XY MAE={oracle['xy_mae']:.0f} "
            f"(gate ≤5000: {'met' if decision['gates']['oracle_xy_mae_le_5000'] else 'missed'})."
        ),
        decision["recommendation"],
    ]
    if decision["gates"]["hard_fail_oracle_lt_2pp_over_table"]:
        lessons.insert(
            0,
            "Hard fail: oracle <2pp over per-card table → board features required.",
        )

    report: dict[str, Any] = {
        "model_name": "placement-probe-v1",
        "model_version": "1.0.0",
        "experiment": "B",
        "created_at": created_at,
        "seconds": round(time.time() - started, 1),
        "hypothesis": (
            "Zone ~39.5% / tile ~2.6% / XY MAE ~5719 because placement heads "
            "do not see which card is placed."
        ),
        "compute": {
            "device": str(device),
            "framework": "pytorch",
            "trunk_model": "policy-bc-v3",
            "trunk_frozen": True,
            "probe_parameters": n_probe,
            "d_model": d_model,
            "card_embed_dim": card_embed_dim,
            "threat_dim": threat_dim if threat_dim > 0 else THREAT_DIM,
            "hidden": hidden,
            "epochs": epochs,
            "batch_size": batch_size,
            "probe_batch_size": probe_batch_size,
            "learning_rate": lr,
            "xy_weight": 0.35,
            "max_context": max_context,
            "global_dim": int(cfg.get("global_dim", GLOBAL_DIM + threat_dim)),
        },
        "data": {
            "battles_total": len(battles),
            "min_card_plays": min_card_plays,
            "seed": seed,
            "train_samples": int(train_cache["zone"].size(0)),
            "val_samples": int(val_cache["zone"].size(0)),
            "test_samples": int(test_cache["zone"].size(0)),
            "vocab_size": vocab.vocab_size,
            "splits": [
                summarize_split("train", train_battles),
                summarize_split("val", val_battles),
                summarize_split("test", test_battles),
            ],
            "table_cards": table["n_cards"],
            "table_train_samples": table["n_samples"],
        },
        "frozen_v3": v3_metrics,
        "control": {**control, "name": "per_card_majority_zone_mean_xy"},
        "oracle": oracle,
        "e2e": e2e,
        "decision": decision,
        "history": history,
        "per_card_highlights": highlights,
        "lessons": lessons,
        "offline_only": True,
        "live_play_ready": False,
    }

    out_json = Path(output_json)
    out_json.parent.mkdir(parents=True, exist_ok=True)
    with out_json.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2, ensure_ascii=False)

    html_path = render_placement_probe_report(report, output_html)
    report["report_json"] = str(out_json)
    report["report_html"] = str(html_path)
    print(
        f"Done [{decision['verdict']}]  control={control['zone_acc']:.3f}  "
        f"oracle={oracle['zone_acc']:.3f}  e2e={e2e['zone_acc']:.3f}  "
        f"xy_mae={oracle['xy_mae']:.0f}",
        flush=True,
    )
    print(f"Wrote {out_json} and {html_path}", flush=True)
    return report


def main(argv: list[str] | None = None) -> None:
    import argparse

    parser = argparse.ArgumentParser(
        description="Experiment B: card-conditioned placement probe (offline)"
    )
    parser.add_argument("--input", default="data/raw")
    parser.add_argument("--policy-dir", default="models/policy_bc_v3")
    parser.add_argument("--card-costs", default="data/card_costs.json")
    parser.add_argument("--output-json", default="reports/placement_probe_v1.json")
    parser.add_argument("--output-html", default="reports/placement_probe_v1.html")
    parser.add_argument("--epochs", type=int, default=10)
    parser.add_argument("--batch-size", type=int, default=256)
    parser.add_argument("--probe-batch-size", type=int, default=512)
    parser.add_argument("--lr", type=float, default=1e-3)
    parser.add_argument("--hidden", type=int, default=256)
    parser.add_argument("--min-card-plays", type=int, default=12)
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--device", default=None)
    args = parser.parse_args(argv)
    report = run_placement_probe(
        input_dir=args.input,
        policy_dir=args.policy_dir,
        card_costs_path=args.card_costs,
        output_json=args.output_json,
        output_html=args.output_html,
        epochs=args.epochs,
        batch_size=args.batch_size,
        probe_batch_size=args.probe_batch_size,
        lr=args.lr,
        hidden=args.hidden,
        min_card_plays=args.min_card_plays,
        seed=args.seed,
        device_name=args.device,
    )
    print(json.dumps({"verdict": report["decision"]["verdict"], **report["decision"]["deltas_pp"]}, indent=2))


if __name__ == "__main__":
    main()
