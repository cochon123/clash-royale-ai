#!/usr/bin/env python3
"""Finalize a crashed v4.1 run from the mid-training best checkpoint."""

from __future__ import annotations

import json
import time
from datetime import datetime, timezone
from pathlib import Path

import torch

from cr_replay_pipeline.policy_dataset import (
    DEFAULT_MAX_CONTEXT,
    DEFAULT_MIN_CONTEXT,
    DEFAULT_REACTION_SECONDS,
    baseline_cycle_slot,
    baseline_frequency_slot,
    build_vocab,
    collect_battles,
    create_policy_dataloaders,
    load_card_costs,
    split_battles,
    summarize_split,
)
from cr_replay_pipeline.policy_model import PolicyBC
from cr_replay_pipeline.policy_train import (
    _load_realism_scorer,
    _readiness,
    _score_realism,
    evaluate_policy,
    rollout_policy_battles,
)
from cr_replay_pipeline.realism_generate import TimingPrior, generate_easy_negative, generate_medium_negative
from cr_replay_pipeline.winner_dataset import CardVocab

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "models" / "policy_bc_v4.1"
CKPT = OUTPUT / "best_model.pt"
STAGES = OUTPUT / "training_stages.json"


def main() -> None:
    started = time.time()
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    ckpt = torch.load(CKPT, map_location=device, weights_only=False)
    config = ckpt["config"]
    history = json.loads(STAGES.read_text(encoding="utf-8"))
    vocab = CardVocab.from_dict(ckpt["vocab"])
    costs = load_card_costs(ROOT / "data" / "card_costs.json")

    print("Loading battles ...", flush=True)
    battles = collect_battles(ROOT / "data" / "raw", min_card_plays=12)
    train_battles, val_battles, test_battles = split_battles(battles, seed=42)
    # Rebuild vocab from train for sample counts / splits consistency, but keep
    # checkpoint vocab for the model (training used train-built vocab).
    train_vocab = build_vocab(train_battles)
    assert train_vocab.vocab_size == vocab.vocab_size, (
        train_vocab.vocab_size,
        vocab.vocab_size,
    )

    threat_dim = int(config.get("threat_dim", 14))
    max_context = int(config.get("max_context", DEFAULT_MAX_CONTEXT))
    loss_kwargs = {"zone_weight": 1.1, "xy_weight": 0.55, "slot_weight": 1.4}
    train_loader, val_loader, test_loader = create_policy_dataloaders(
        train_battles,
        val_battles,
        test_battles,
        vocab,
        costs,
        batch_size=256,
        max_context=max_context,
        max_samples_per_battle=40,
        reaction_weight=3.0,
        reaction_repeats=2,
        reaction_seconds=float(config.get("reaction_seconds", DEFAULT_REACTION_SECONDS)),
        threat_dim=threat_dim,
    )

    model = PolicyBC(
        vocab_size=vocab.vocab_size,
        global_dim=int(config["global_dim"]),
        d_model=int(config["d_model"]),
        num_layers=int(config["num_layers"]),
        dropout=float(config.get("dropout", 0.2)),
        card_conditioned_placement=bool(config.get("card_conditioned_placement", True)),
    ).to(device)
    model.load_state_dict(ckpt["model_state"])
    model.eval()

    print("Evaluating val/test ...", flush=True)
    val_metrics = evaluate_policy(model, val_loader, device, loss_kwargs=loss_kwargs)
    test_metrics = evaluate_policy(model, test_loader, device, loss_kwargs=loss_kwargs)
    freq_base = baseline_frequency_slot(test_battles)
    cycle_base = baseline_cycle_slot(test_battles)

    realism_path = ROOT / "models" / "realism_scorer" / "realism_ensemble.pkl"
    realism_artifact = _load_realism_scorer(realism_path)
    rollout_stats: dict = {"available": False}
    if realism_artifact is not None:
        print("Rolling out policy continuations ...", flush=True)
        import random
        import numpy as np

        rollouts = rollout_policy_battles(
            model,
            vocab,
            costs,
            test_battles,
            device,
            n_battles=96,
            seed=42 + 7,
            max_context=max_context,
            threat_dim=threat_dim,
        )
        timing_prior = TimingPrior.from_battles(train_battles)
        rng = random.Random(42 + 9)
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

    created_at = ckpt.get("created_at") or datetime.now(timezone.utc).strftime(
        "%Y-%m-%dT%H:%M:%SZ"
    )
    n_params = sum(p.numel() for p in model.parameters())
    lessons = [
        "v4.1 is architecture-identical to v4.0; the only change is training on the larger replay cut (~28k raw).",
        "v4 keeps v3 threat conditioning + reaction upweight; adds jointly trained card-conditioned zone/XY heads.",
        "Training crashed twice around epoch 17 under memory pressure; finalized from best mid-run checkpoint "
        f"(epoch {ckpt.get('epoch')}, best_val_loss={ckpt.get('best_val_loss')}).",
        "Per-improvement checkpointing was added so a crash no longer wipes the run.",
        "Judge v4.1 on zone/XY MAE and slot top-1 vs v4.0 — not the auto live-play flag.",
    ]

    # Prefer last history val; fall back to fresh val_metrics
    val_block = {
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
    } if history else val_metrics

    report = {
        "model_name": "policy-bc-v4.1",
        "model_version": "4.1.0",
        "created_at": created_at,
        "seconds": round(time.time() - started, 1),
        "finalize_note": (
            f"Finalized from best checkpoint at epoch {ckpt.get('epoch')} "
            f"after training process was killed; epochs_requested=25."
        ),
        "compute": {
            "device": str(device),
            "framework": "pytorch",
            "parameters": n_params,
            "d_model": int(config["d_model"]),
            "num_layers": int(config["num_layers"]),
            "epochs_requested": 25,
            "epochs_ran": len(history),
            "batch_size": 256,
            "learning_rate": 2e-4,
            "dropout": float(config.get("dropout", 0.2)),
            "max_context": max_context,
            "max_samples_per_battle": 40,
            "global_dim": int(config["global_dim"]),
            "threat_dim": threat_dim,
            "card_conditioned_placement": True,
            "reaction_weight": 3.0,
            "reaction_repeats": 2,
            "reaction_seconds": float(
                config.get("reaction_seconds", DEFAULT_REACTION_SECONDS)
            ),
            "loss_kwargs": loss_kwargs,
            "min_context": int(config.get("min_context", DEFAULT_MIN_CONTEXT)),
        },
        "data": {
            "battles_total": len(battles),
            "min_card_plays": 12,
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
        "val": val_block,
        "test": test_metrics,
        "rollouts": {k: v for k, v in rollout_stats.items() if k != "hist"},
        "rollout_hist": rollout_stats.get("hist"),
        "history": history,
        "checkpoint": str(CKPT),
        "lessons": lessons,
        "live_play_readiness": _readiness(
            test_metrics, freq_base, cycle_base, rollout_stats
        ),
    }

    with (OUTPUT / "vocab.json").open("w", encoding="utf-8") as handle:
        json.dump(vocab.to_dict(), handle, indent=2)
    # Rewrite final checkpoint in canonical form (no mid-run-only fields required)
    torch.save(
        {
            "model_state": model.state_dict(),
            "vocab": vocab.to_dict(),
            "config": {
                "d_model": int(config["d_model"]),
                "num_layers": int(config["num_layers"]),
                "dropout": float(config.get("dropout", 0.2)),
                "max_context": max_context,
                "min_context": int(config.get("min_context", DEFAULT_MIN_CONTEXT)),
                "global_dim": int(config["global_dim"]),
                "threat_dim": threat_dim,
                "reaction_seconds": float(
                    config.get("reaction_seconds", DEFAULT_REACTION_SECONDS)
                ),
                "card_conditioned_placement": True,
                "version": "4.1.0",
            },
            "created_at": created_at,
        },
        CKPT,
    )
    report_path = OUTPUT / "report.json"
    with report_path.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    with STAGES.open("w", encoding="utf-8") as handle:
        json.dump(history, handle, indent=2)

    print(json.dumps({"test": test_metrics, "rollouts": report["rollouts"]}, indent=2))
    print(f"Wrote {report_path}")


if __name__ == "__main__":
    main()
