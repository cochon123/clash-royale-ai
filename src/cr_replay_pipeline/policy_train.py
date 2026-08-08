"""Train and offline-evaluate a behavior-cloning next-action policy."""

from __future__ import annotations

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
from .policy_model import PolicyBC
from .realism_generate import TimingPrior, generate_easy_negative, generate_medium_negative
from .realism_train import extract_realism_features
from .winner_dataset import BattleExample


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
    timing_err = 0.0
    loss_kwargs = loss_kwargs or {}

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
            types,
            zones,
            xy,
            timing,
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
            target_slots=None,
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
        "timing_mae": timing_err / max(total, 1),
        "n": total,
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
) -> list[BattleExample]:
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
            dummy = {
                "seconds": seconds + 1.0,
                "side": next_side,
                "event_type": "card_play",
                "card": battle.team_deck[0] if next_side == "team" else battle.opponent_deck[0],
                "x": 9000,
                "y": 8000 if next_side == "team" else 24000,
            }
            probe = BattleExample(
                battle_id=battle.battle_id + "-rollout",
                team_deck=battle.team_deck,
                opponent_deck=battle.opponent_deck,
                team_wins=battle.team_wins,
                events=tuple(events) + (dummy,),
            )
            sample = encode_policy_sample(
                probe,
                len(events),
                vocab,
                costs,
                max_context=max_context,
                threat_dim=threat_dim,
            )
            if sample is None:
                break
            (
                continuous,
                card_ids,
                team_deck,
                opp_deck,
                global_feat,
                slot_feats,
                hand_mask,
                _slot,
                _type,
                _zone,
                _xy,
                _timing,
                length,
            ) = sample
            out = model(
                continuous.unsqueeze(0).to(device),
                card_ids.unsqueeze(0).to(device),
                team_deck.unsqueeze(0).to(device),
                opp_deck.unsqueeze(0).to(device),
                global_feat.unsqueeze(0).to(device),
                length.unsqueeze(0).to(device),
                slot_feats.unsqueeze(0).to(device),
                hand_mask.unsqueeze(0).to(device),
            )
            logits = out["slot_logits"][0] / max(temperature, 1e-3)
            probs = torch.softmax(logits, dim=-1).cpu().numpy()
            slot = int(rng.choices(range(8), weights=probs.tolist(), k=1)[0])
            acting_deck = battle.team_deck if next_side == "team" else battle.opponent_deck
            card = acting_deck[slot]
            event_type = (
                "ability_activation"
                if int(out["type_logits"][0].argmax().item()) == 1
                else "card_play"
            )
            xy = out["xy"][0].cpu().numpy()
            x = int(np.clip(xy[0] * 18000.0, 3000, 15000))
            y_norm = float(xy[1])
            if next_side == "opponent":
                y_norm = 1.0 - y_norm
            y = int(np.clip(y_norm * 32000.0, 500, 31500))
            if event_type == "ability_activation":
                x, y = 9000, 16000
            dt = float(np.clip(np.expm1(out["timing"][0].item()), 0.2, 12.0))
            seconds = min(330.0, seconds + dt)
            events.append(
                {
                    "seconds": seconds,
                    "side": next_side,
                    "event_type": event_type,
                    "card": card,
                    "x": x,
                    "y": y,
                }
            )
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


def train_policy_model(
    input_dir: str | Path = "data/raw",
    output_dir: str | Path = "models/policy_bc",
    card_costs_path: str | Path = "data/card_costs.json",
    realism_model_dir: str | Path = "models/realism_scorer",
    epochs: int = 25,
    batch_size: int = 256,
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
) -> dict[str, Any]:
    torch.manual_seed(seed)
    np.random.seed(seed)
    random.seed(seed)

    if device_name:
        device = torch.device(device_name)
    else:
        device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

    version = str(version)
    use_v4 = version.startswith("4")
    use_v3 = version.startswith("3") or use_v4
    card_conditioned_placement = use_v4
    threat_dim = THREAT_DIM if use_v3 else 0
    global_dim = GLOBAL_DIM + threat_dim
    if use_v4:
        model_name, model_version = "policy-bc-v4", "4.0.0"
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
        {"zone_weight": 1.1, "xy_weight": 0.55, "slot_weight": 1.4}
        if use_v4
        else {}
    )

    output = Path(output_dir)
    output.mkdir(parents=True, exist_ok=True)
    started = time.time()
    created_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    print(f"Loading battles from {input_dir} ...", flush=True)
    battles = collect_battles(input_dir, min_card_plays=min_card_plays)
    if len(battles) < 50:
        raise RuntimeError(f"Need at least 50 usable battles; found {len(battles)}")

    train_battles, val_battles, test_battles = split_battles(battles, seed=seed)
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
    )
    print(
        f"Training {model_name} (threat_dim={threat_dim}, "
        f"card_conditioned_placement={card_conditioned_placement}, "
        f"reaction_weight={rw}, reaction_repeats={rr})",
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
    ).to(device)
    optimizer = AdamW(model.parameters(), lr=learning_rate, weight_decay=1e-4)
    scheduler = CosineAnnealingLR(optimizer, T_max=max(epochs, 1))

    history: list[dict[str, Any]] = []
    best_val = float("inf")
    best_state = None
    bad_epochs = 0

    for epoch in range(1, epochs + 1):
        model.train()
        running = 0.0
        n_batches = 0
        for batch in train_loader:
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
            optimizer.zero_grad(set_to_none=True)
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
            losses["loss"].backward()
            nn.utils.clip_grad_norm_(model.parameters(), 1.0)
            optimizer.step()
            running += float(losses["loss"].item())
            n_batches += 1
        scheduler.step()

        train_loss = running / max(n_batches, 1)
        val_metrics = evaluate_policy(
            model, val_loader, device, loss_kwargs=loss_kwargs
        )
        row = {
            "epoch": epoch,
            "train_loss": train_loss,
            **{f"val_{k}": v for k, v in val_metrics.items() if k != "n"},
            "val_n": val_metrics["n"],
            "lr": float(scheduler.get_last_lr()[0]),
        }
        history.append(row)
        print(
            f"epoch {epoch:02d}  train_loss={train_loss:.4f}  "
            f"val_slot@1={val_metrics['slot_top1']:.3f}  "
            f"val_zone={val_metrics['zone_acc']:.3f}  "
            f"val_tile={val_metrics['tile_acc']:.3f}  "
            f"val_loss={val_metrics['loss']:.4f}",
            flush=True,
        )
        if val_metrics["loss"] < best_val - 1e-4:
            best_val = val_metrics["loss"]
            best_state = {k: v.detach().cpu().clone() for k, v in model.state_dict().items()}
            bad_epochs = 0
        else:
            bad_epochs += 1
            if bad_epochs >= patience:
                print(f"Early stop at epoch {epoch}", flush=True)
                break

    if best_state is not None:
        model.load_state_dict(best_state)

    test_metrics = evaluate_policy(
        model, test_loader, device, loss_kwargs=loss_kwargs
    )
    freq_base = baseline_frequency_slot(test_battles)
    cycle_base = baseline_cycle_slot(test_battles)

    realism_path = Path(realism_model_dir) / "realism_ensemble.pkl"
    realism_artifact = _load_realism_scorer(realism_path)
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
        }

    ckpt_path = output / "best_model.pt"
    torch.save(
        {
            "model_state": model.state_dict(),
            "vocab": vocab.to_dict(),
            "config": {
                "d_model": d_model,
                "num_layers": num_layers,
                "dropout": dropout,
                "max_context": max_context,
                "min_context": DEFAULT_MIN_CONTEXT,
                "global_dim": global_dim,
                "threat_dim": threat_dim,
                "reaction_seconds": reaction_seconds,
                "card_conditioned_placement": card_conditioned_placement,
                "version": model_version,
            },
            "created_at": created_at,
        },
        ckpt_path,
    )
    with (output / "vocab.json").open("w", encoding="utf-8") as handle:
        json.dump(vocab.to_dict(), handle, indent=2)

    n_params = sum(p.numel() for p in model.parameters())
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
    if use_v4:
        lessons = v4_lessons
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
            "learning_rate": learning_rate,
            "dropout": dropout,
            "max_context": max_context,
            "max_samples_per_battle": max_samples_per_battle,
            "global_dim": global_dim,
            "threat_dim": threat_dim,
            "card_conditioned_placement": card_conditioned_placement,
            "reaction_weight": rw,
            "reaction_repeats": rr,
            "reaction_seconds": reaction_seconds,
            "loss_kwargs": loss_kwargs,
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
                "timing_mae",
            )
        }
        if history
        else {},
        "test": test_metrics,
        "rollouts": {k: v for k, v in rollout_stats.items() if k != "hist"},
        "rollout_hist": rollout_stats.get("hist"),
        "history": history,
        "checkpoint": str(ckpt_path),
        "lessons": lessons,
        "live_play_readiness": _readiness(test_metrics, freq_base, cycle_base, rollout_stats),
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
    ready = all(checks.values())
    return {
        "ready_for_live_smoke_test": ready,
        "checks": checks,
        "rationale": (
            "All offline gates passed; a short supervised live smoke test is justified."
            if ready
            else "Keep iterating offline — one or more readiness checks failed."
        ),
    }
