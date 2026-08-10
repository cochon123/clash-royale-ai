"""Train policy_bc_v5: BC + style feature matching + REINFORCE polish vs style judge."""

from __future__ import annotations

import json
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
    baseline_cycle_slot,
    baseline_frequency_slot,
    build_vocab,
    collect_battles,
    create_policy_dataloaders,
    load_card_costs,
    split_battles,
    summarize_split,
)
from .policy_infer import load_policy
from .policy_model import PolicyBC
from .policy_train import (
    _load_realism_scorer,
    _move_batch,
    _readiness,
    _score_realism,
    evaluate_policy,
    rollout_policy_battles,
)
from .realism_generate import TimingPrior, generate_easy_negative, generate_medium_negative
from .style_adversary import (
    batch_style_match_loss,
    eval_style_vs_humans,
    human_style_targets,
    load_action_clock,
    load_style_judge,
    reinforce_style_step,
)


def train_policy_v5(
    input_dir: str | Path = "data/raw",
    output_dir: str | Path = "models/policy_bc_v5",
    card_costs_path: str | Path = "data/card_costs.json",
    realism_model_dir: str | Path = "models/realism_scorer",
    style_model_dir: str | Path = "models/style_discriminator",
    action_clock_dir: str | Path = "models/action_clock_v1",
    warmstart_dir: str | Path = "models/policy_bc_v4.1",
    epochs: int = 18,
    batch_size: int = 256,
    learning_rate: float = 1.5e-4,
    d_model: int = 160,
    num_layers: int = 2,
    min_card_plays: int = 12,
    seed: int = 42,
    device_name: str | None = None,
    patience: int = 8,
    dropout: float = 0.2,
    max_context: int = DEFAULT_MAX_CONTEXT,
    max_samples_per_battle: int | None = 40,
    reaction_weight: float = 3.0,
    reaction_repeats: int = 2,
    reaction_seconds: float = DEFAULT_REACTION_SECONDS,
    style_match_weight: float = 0.4,
    polish_start_epoch: int = 6,
    polish_every: int = 2,
    polish_battles: int = 16,
    polish_events: int = 16,
    style_eval_battles: int = 64,
) -> dict[str, Any]:
    torch.manual_seed(seed)
    np.random.seed(seed)
    random.seed(seed)

    if device_name:
        device = torch.device(device_name)
    else:
        device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

    model_name, model_version = "policy-bc-v5", "5.0.0"
    card_conditioned_placement = True
    threat_dim = THREAT_DIM
    global_dim = GLOBAL_DIM + threat_dim
    loss_kwargs = {"zone_weight": 1.1, "xy_weight": 0.55, "slot_weight": 1.4}

    output = Path(output_dir)
    output.mkdir(parents=True, exist_ok=True)
    started = time.time()
    created_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    style_artifact = load_style_judge(style_model_dir)
    if style_artifact is None:
        raise RuntimeError(
            f"Style discriminator not found at {style_model_dir}/style_ensemble.pkl. "
            "Train it first with: cr-replays train-style"
        )
    clock = load_action_clock(action_clock_dir)
    if clock is None:
        print(
            f"Warning: action clock missing at {action_clock_dir}; "
            "REINFORCE rollouts will use strict alternation.",
            flush=True,
        )

    print(f"Loading battles from {input_dir} ...", flush=True)
    battles = collect_battles(input_dir, min_card_plays=min_card_plays)
    if len(battles) < 50:
        raise RuntimeError(f"Need at least 50 usable battles; found {len(battles)}")

    train_battles, val_battles, test_battles = split_battles(battles, seed=seed)
    vocab = build_vocab(train_battles)
    costs = load_card_costs(card_costs_path)
    human_targets = human_style_targets(train_battles, costs)

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
        reaction_weight=reaction_weight,
        reaction_repeats=reaction_repeats,
    )

    model = PolicyBC(
        vocab_size=vocab.vocab_size,
        global_dim=global_dim,
        d_model=d_model,
        num_layers=num_layers,
        dropout=dropout,
        card_conditioned_placement=card_conditioned_placement,
    ).to(device)

    warmstart = Path(warmstart_dir)
    warmstarted = False
    if warmstart.exists() and (warmstart / "best_model.pt").exists():
        try:
            donor, donor_vocab, donor_cfg, _ = load_policy(warmstart, device_name=str(device))
            if (
                donor_vocab.vocab_size == vocab.vocab_size
                and int(donor_cfg.get("d_model", d_model)) == d_model
                and int(donor_cfg.get("num_layers", num_layers)) == num_layers
                and bool(donor_cfg.get("card_conditioned_placement", False))
            ):
                model.load_state_dict(donor.state_dict())
                warmstarted = True
                print(f"Warm-started from {warmstart}", flush=True)
            else:
                print(
                    f"Warm-start skipped (arch/vocab mismatch vs {warmstart})",
                    flush=True,
                )
            del donor
        except Exception as exc:  # noqa: BLE001 — fall back to scratch
            print(f"Warm-start failed ({exc}); training from scratch", flush=True)

    print(
        f"Training {model_name} (style_match={style_match_weight}, "
        f"polish_start={polish_start_epoch}, clock={'yes' if clock else 'no'}, "
        f"warmstart={warmstarted})",
        flush=True,
    )
    print(
        f"Samples train/val/test: {len(train_loader.dataset)}/"
        f"{len(val_loader.dataset)}/{len(test_loader.dataset)} on {device}",
        flush=True,
    )

    optimizer = AdamW(model.parameters(), lr=learning_rate, weight_decay=1e-4)
    scheduler = CosineAnnealingLR(optimizer, T_max=max(epochs, 1))

    history: list[dict[str, Any]] = []
    best_val = float("inf")
    best_state = None
    best_score = -1.0  # maximize mean_P_human_ai * slot_top1 proxy
    bad_epochs = 0
    baseline_ema: float | None = None
    style_history: list[dict[str, Any]] = []

    # Initial style baseline (pre-train / post-warmstart).
    print("Scoring initial style detectability ...", flush=True)
    style0 = eval_style_vs_humans(
        model,
        vocab,
        costs,
        val_battles,
        device,
        style_artifact,
        human_targets,
        clock,
        n_battles=min(64, style_eval_battles),
        seed=seed + 3,
        max_context=max_context,
        threat_dim=threat_dim,
    )
    style_history.append(
        {
            "epoch": 0,
            **style0["full"],
            "harness_free": style0["harness_free"],
            "feature_l2": style0.get("feature_l2"),
            "alternation": style0.get("alternation"),
            "protocol": style0.get("protocol"),
        }
    )
    alt0 = (style0.get("alternation") or {}).get("full") or {}
    print(
        f"style@0  clock_P={style0['full']['mean_P_human_ai']:.4f}  "
        f"clock_fool={style0['full']['fool_rate_at_0.5']:.3f}  "
        f"feat_l2={style0.get('feature_l2', 0):.3f}  "
        f"alt_P={alt0.get('mean_P_human_ai', 0):.4f}  "
        f"alt_fool={alt0.get('fool_rate_at_0.5', 0):.3f}",
        flush=True,
    )

    for epoch in range(1, epochs + 1):
        model.train()
        running = 0.0
        running_style = 0.0
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
            style_losses = batch_style_match_loss(
                out, human_targets, weight=style_match_weight
            )
            total = losses["loss"] + style_losses["style_match_loss"]
            total.backward()
            nn.utils.clip_grad_norm_(model.parameters(), 1.0)
            optimizer.step()
            running += float(losses["loss"].item())
            running_style += float(style_losses["style_match_raw"].item())
            n_batches += 1
        scheduler.step()

        polish_metrics: dict[str, Any] | None = None
        if epoch >= polish_start_epoch and (epoch - polish_start_epoch) % polish_every == 0:
            print(f"epoch {epoch:02d}  REINFORCE style polish ...", flush=True)
            polish_metrics, baseline_ema = reinforce_style_step(
                model,
                optimizer,
                vocab,
                costs,
                train_battles,
                device,
                style_artifact,
                human_targets,
                clock,
                n_battles=polish_battles,
                max_new_events=polish_events,
                temperature=0.9,
                seed=seed + 1000 + epoch,
                max_context=max_context,
                threat_dim=threat_dim,
                baseline_ema=baseline_ema,
            )
            print(
                f"  polish  n={polish_metrics['n']}  "
                f"reward={polish_metrics['reward_mean']:.4f}  "
                f"feat_l2={polish_metrics.get('feature_l2', 0):.3f}  "
                f"full_P={polish_metrics['full_P_human']:.4f}  "
                f"fool={polish_metrics['fool_rate_at_0.5']:.3f}",
                flush=True,
            )

        train_loss = running / max(n_batches, 1)
        train_style = running_style / max(n_batches, 1)
        val_metrics = evaluate_policy(model, val_loader, device, loss_kwargs=loss_kwargs)

        # Style eval every epoch after polish starts, else every 3.
        do_style = epoch >= polish_start_epoch or epoch % 3 == 0 or epoch == epochs
        style_row: dict[str, Any] | None = None
        if do_style:
            style_eval = eval_style_vs_humans(
                model,
                vocab,
                costs,
                val_battles,
                device,
                style_artifact,
                human_targets,
                clock,
                n_battles=min(80, style_eval_battles),
                seed=seed + 7 + epoch,
                max_context=max_context,
                threat_dim=threat_dim,
                include_alternation=(epoch == epochs or epoch % 6 == 0),
            )
            style_row = {
                "epoch": epoch,
                **style_eval["full"],
                "harness_free": style_eval["harness_free"],
                "tell_gaps": style_eval["tell_gaps"],
                "protocol": style_eval["protocol"],
                "feature_l2": style_eval.get("feature_l2"),
                "alternation": style_eval.get("alternation"),
            }
            style_history.append(style_row)

        row = {
            "epoch": epoch,
            "train_loss": train_loss,
            "train_style_match": train_style,
            **{f"val_{k}": v for k, v in val_metrics.items() if k != "n"},
            "val_n": val_metrics["n"],
            "lr": float(scheduler.get_last_lr()[0]),
            "style_P_human": None if style_row is None else style_row["mean_P_human_ai"],
            "style_fool_rate": None if style_row is None else style_row["fool_rate_at_0.5"],
            "style_auc": None if style_row is None else style_row["auc"],
            "style_feature_l2": None if style_row is None else style_row.get("feature_l2"),
            "style_alt_P_human": None
            if style_row is None
            else ((style_row.get("alternation") or {}).get("full") or {}).get(
                "mean_P_human_ai"
            ),
            "style_free_P_human": None
            if style_row is None
            else style_row["harness_free"]["mean_P_human_ai"],
            "polish": polish_metrics,
        }
        history.append(row)
        print(
            f"epoch {epoch:02d}  train_loss={train_loss:.4f}  "
            f"style_match={train_style:.4f}  "
            f"val_slot@1={val_metrics['slot_top1']:.3f}  "
            f"val_zone={val_metrics['zone_acc']:.3f}  "
            f"val_loss={val_metrics['loss']:.4f}"
            + (
                f"  feat_l2={style_row.get('feature_l2', 0):.3f}  "
                f"clock_P={style_row['mean_P_human_ai']:.4f}  "
                f"alt_P={((style_row.get('alternation') or {}).get('full') or {}).get('mean_P_human_ai', 0):.4f}"
                if style_row
                else ""
            ),
            flush=True,
        )

        # Joint selection: BC quality + lower tell distance (P saturates under clock).
        feat_l2 = float(style_row.get("feature_l2") or 2.0) if style_row is not None else 2.0
        joint = (
            float(val_metrics["slot_top1"])
            + 0.12 * max(0.0, 2.0 - feat_l2)
            - 0.05 * float(val_metrics["loss"])
        )
        improved = False
        if joint > best_score + 1e-4:
            best_score = joint
            best_state = {k: v.detach().cpu().clone() for k, v in model.state_dict().items()}
            best_val = float(val_metrics["loss"])
            bad_epochs = 0
            improved = True
        elif val_metrics["loss"] < best_val - 1e-4 and style_row is None:
            best_val = float(val_metrics["loss"])
            best_state = {k: v.detach().cpu().clone() for k, v in model.state_dict().items()}
            bad_epochs = 0
            improved = True
        else:
            bad_epochs += 1

        if improved:
            torch.save(
                {
                    "model_state": best_state,
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
                        "style_match_weight": style_match_weight,
                    },
                    "created_at": created_at,
                    "epoch": epoch,
                    "best_val_loss": best_val,
                    "best_joint_score": best_score,
                },
                output / "best_model.pt",
            )
            with (output / "training_stages.json").open("w", encoding="utf-8") as handle:
                json.dump({"history": history, "style_history": style_history}, handle, indent=2)

        if bad_epochs >= patience:
            print(f"Early stop at epoch {epoch}", flush=True)
            break

    if best_state is not None:
        model.load_state_dict(best_state)

    test_metrics = evaluate_policy(model, test_loader, device, loss_kwargs=loss_kwargs)
    freq_base = baseline_frequency_slot(test_battles)
    cycle_base = baseline_cycle_slot(test_battles)

    print("Final style evaluation on test battles ...", flush=True)
    style_final = eval_style_vs_humans(
        model,
        vocab,
        costs,
        test_battles,
        device,
        style_artifact,
        human_targets,
        clock,
        n_battles=style_eval_battles,
        seed=seed + 99,
        max_context=max_context,
        threat_dim=threat_dim,
    )

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
                "style_match_weight": style_match_weight,
            },
            "created_at": created_at,
        },
        ckpt_path,
    )
    with (output / "vocab.json").open("w", encoding="utf-8") as handle:
        json.dump(vocab.to_dict(), handle, indent=2)

    n_params = sum(p.numel() for p in model.parameters())
    lessons = [
        "v5 keeps the v4.1 trunk (threat + card-conditioned placement) and adds an anti-detector objective.",
        "The style judge is non-differentiable (HGB+ExtraTrees) and, under the original alternation harness, is dominated by harness features (alt_rate/n_events).",
        "Clock-aware rollouts alone flip full-judge fool rate near 100%; residual AI tells are timing gaps + placement spread (feature_l2).",
        "v5 therefore uses differentiable moment matching + REINFORCE on match-feature distance, not saturated P(human).",
        "Report both clock (deploy) and alternation (legacy judge) protocols; compare feature_l2 to v4.1.",
        "Checkpoint selection maximizes slot@1 − feature_l2 so style gains that destroy cloning are rejected.",
    ]

    report = {
        "model_name": model_name,
        "model_version": model_version,
        "created_at": created_at,
        "seconds": round(time.time() - started, 1),
        "compute": {
            "device": str(device),
            "framework": "pytorch + frozen sklearn style judge",
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
            "reaction_weight": reaction_weight,
            "reaction_repeats": reaction_repeats,
            "reaction_seconds": reaction_seconds,
            "loss_kwargs": loss_kwargs,
            "style_match_weight": style_match_weight,
            "polish_start_epoch": polish_start_epoch,
            "polish_every": polish_every,
            "polish_battles": polish_battles,
            "polish_events": polish_events,
            "warmstart_dir": str(warmstart_dir) if warmstarted else None,
            "style_model_dir": str(style_model_dir),
            "action_clock_dir": str(action_clock_dir) if clock is not None else None,
        },
        "data": {
            "battles_total": len(battles),
            "min_card_plays": min_card_plays,
            "train_samples": len(train_loader.dataset),
            "val_samples": len(val_loader.dataset),
            "test_samples": len(test_loader.dataset),
            "vocab_size": vocab.vocab_size,
            "human_style_targets": {
                k: v
                for k, v in human_targets.items()
                if k not in {"feature_means", "feature_stds"}
            },
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
        "style": {
            "protocol": style_final["protocol"],
            "full": style_final["full"],
            "harness_free": style_final["harness_free"],
            "tell_gaps": style_final["tell_gaps"],
            "feature_l2": style_final.get("feature_l2"),
            "alternation": style_final.get("alternation"),
            "clock": style_final.get("clock"),
            "initial": style_history[0] if style_history else None,
            "note": (
                "Clock protocol is the deploy-style readout (action-clock who-acts-next). "
                "Alternation matches the original style-discriminator training harness; "
                "under that harness the judge is dominated by forced alt_rate/n_events. "
                "v5 optimizes controllable tell distance (feature_l2) + BC."
            ),
        },
        "rollouts": {k: v for k, v in rollout_stats.items() if k != "hist"},
        "rollout_hist": rollout_stats.get("hist"),
        "history": history,
        "style_history": style_history,
        "checkpoint": str(ckpt_path),
        "lessons": lessons,
        "live_play_readiness": _readiness(test_metrics, freq_base, cycle_base, rollout_stats),
    }

    report_path = output / "report.json"
    with report_path.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    with (output / "training_stages.json").open("w", encoding="utf-8") as handle:
        json.dump({"history": history, "style_history": style_history}, handle, indent=2)

    print(
        json.dumps(
            {
                "test": test_metrics,
                "style": report["style"],
                "baselines": report["baselines"],
                "rollouts": report["rollouts"],
                "live_play_readiness": report["live_play_readiness"],
            },
            indent=2,
        )
    )
    print(f"Wrote {report_path}")
    return report
