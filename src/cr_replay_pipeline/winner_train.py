from __future__ import annotations

import json
import time
from pathlib import Path
from typing import Any

import numpy as np
import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.optim import AdamW
from torch.optim.lr_scheduler import CosineAnnealingLR
from torch.utils.data import DataLoader

from .winner_dataset import (
    CONTINUOUS_DIM,
    GLOBAL_DIM,
    WinnerSequenceDataset,
    build_vocab,
    collate_winner_batch,
    collect_battles,
    create_dataloaders,
    load_card_costs,
    split_battles,
    summarize_split,
)
from .winner_model import WinnerPredictor


def _auc_binary(labels: np.ndarray, probs: np.ndarray) -> float:
    labels = labels.astype(np.int64)
    if labels.min() == labels.max():
        return 0.5
    order = np.argsort(probs)
    ranks = np.empty_like(order, dtype=np.float64)
    ranks[order] = np.arange(1, len(probs) + 1)
    pos = labels == 1
    n_pos = int(pos.sum())
    n_neg = int((~pos).sum())
    if n_pos == 0 or n_neg == 0:
        return 0.5
    return float((ranks[pos].sum() - n_pos * (n_pos + 1) / 2.0) / (n_pos * n_neg))


def _move_batch(batch, device: torch.device):
    continuous, card_ids, team_deck, opponent_deck, globals_, labels, lengths = batch
    return (
        continuous.to(device),
        card_ids.to(device),
        team_deck.to(device),
        opponent_deck.to(device),
        globals_.to(device),
        labels.to(device),
        lengths.to(device),
    )


@torch.no_grad()
def evaluate(model: nn.Module, loader, device: torch.device) -> dict[str, Any]:
    model.eval()
    total_loss = 0.0
    correct = 0
    total = 0
    labels_all: list[int] = []
    probs_all: list[float] = []
    preds_all: list[int] = []

    for batch in loader:
        continuous, card_ids, team_deck, opponent_deck, globals_, labels, lengths = (
            _move_batch(batch, device)
        )
        out = model(
            continuous, card_ids, team_deck, opponent_deck, globals_, lengths
        )
        loss = F.cross_entropy(out["logits"], labels, label_smoothing=0.02)
        total_loss += float(loss.item())
        preds = out["logits"].argmax(dim=-1)
        correct += int((preds == labels).sum().item())
        total += int(labels.size(0))
        labels_all.extend(labels.cpu().tolist())
        preds_all.extend(preds.cpu().tolist())
        probs_all.extend(out["team_win_prob"].cpu().tolist())

    labels_np = np.asarray(labels_all)
    probs_np = np.asarray(probs_all)
    preds_np = np.asarray(preds_all)
    return {
        "loss": total_loss / max(len(loader), 1),
        "acc": correct / max(total, 1),
        "auc": _auc_binary(labels_np, probs_np),
        "n": total,
        "labels": labels_np,
        "preds": preds_np,
        "probs": probs_np,
    }


def train_winner_model(
    input_dir: str | Path = "data/raw",
    output_dir: str | Path = "models/winner_predictor",
    card_costs_path: str | Path = "data/card_costs.json",
    epochs: int = 40,
    batch_size: int = 64,
    learning_rate: float = 2e-4,
    d_model: int = 160,
    num_layers: int = 2,
    min_card_plays: int = 12,
    seed: int = 42,
    device_name: str | None = None,
    patience: int = 12,
    dropout: float = 0.3,
) -> dict[str, Any]:
    torch.manual_seed(seed)
    np.random.seed(seed)

    if device_name:
        device = torch.device(device_name)
    else:
        device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

    output = Path(output_dir)
    output.mkdir(parents=True, exist_ok=True)

    print(f"Loading battles from {input_dir} ...", flush=True)
    battles = collect_battles(input_dir, min_card_plays=min_card_plays)
    if len(battles) < 50:
        raise RuntimeError(
            f"Need at least 50 usable battles with clear winners; found {len(battles)}"
        )

    train_battles, val_battles, test_battles = split_battles(battles, seed=seed)
    vocab = build_vocab(train_battles)
    costs = load_card_costs(card_costs_path)
    # Late prefixes keep the strongest outcome signal without mid-game noise.
    sample_ratios = [0.85, 0.95, 1.0]
    train_loader, val_loader, test_loader = create_dataloaders(
        train_battles,
        val_battles,
        test_battles,
        vocab,
        costs,
        batch_size=batch_size,
        sample_ratios=sample_ratios,
    )

    majority = max(
        sum(b.team_wins for b in train_battles) / len(train_battles),
        1.0 - sum(b.team_wins for b in train_battles) / len(train_battles),
    )
    split_stats = [
        summarize_split("train", train_battles),
        summarize_split("val", val_battles),
        summarize_split("test", test_battles),
    ]
    print(
        json.dumps(
            {
                "device": str(device),
                "majority_baseline": majority,
                "splits": split_stats,
            },
            indent=2,
        )
    )
    print(
        f"Sequences: train={len(train_loader.dataset)} "
        f"val={len(val_loader.dataset)} test={len(test_loader.dataset)} "
        f"vocab={vocab.vocab_size}"
    )

    model = WinnerPredictor(
        vocab_size=vocab.vocab_size,
        continuous_dim=CONTINUOUS_DIM,
        global_dim=GLOBAL_DIM,
        d_model=d_model,
        num_layers=num_layers,
        dropout=dropout,
    ).to(device)
    optimizer = AdamW(model.parameters(), lr=learning_rate, weight_decay=0.08)
    scheduler = CosineAnnealingLR(optimizer, T_max=epochs, eta_min=learning_rate * 0.05)

    full_game_val = DataLoader(
        WinnerSequenceDataset(
            val_battles,
            vocab,
            costs,
            sample_ratios=[1.0],
            augment_swap=False,
        ),
        batch_size=batch_size,
        shuffle=False,
        collate_fn=collate_winner_batch,
    )

    history: dict[str, list[float]] = {
        "train_loss": [],
        "train_acc": [],
        "val_loss": [],
        "val_acc": [],
        "val_auc": [],
        "val_full_auc": [],
    }
    best_val_auc = -1.0
    best_val_acc = -1.0
    bad_epochs = 0
    best_path = output / "best_model.pt"
    started = time.time()

    for epoch in range(1, epochs + 1):
        model.train()
        running_loss = 0.0
        correct = 0
        total = 0
        for batch in train_loader:
            continuous, card_ids, team_deck, opponent_deck, globals_, labels, lengths = (
                _move_batch(batch, device)
            )
            optimizer.zero_grad(set_to_none=True)
            out = model(
                continuous, card_ids, team_deck, opponent_deck, globals_, lengths
            )
            main_loss = F.cross_entropy(out["logits"], labels, label_smoothing=0.03)
            aux_loss = F.cross_entropy(
                out["global_logits"], labels, label_smoothing=0.03
            )
            loss = main_loss + 0.5 * aux_loss
            loss.backward()
            torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
            optimizer.step()
            running_loss += float(loss.item())
            preds = out["logits"].argmax(dim=-1)
            correct += int((preds == labels).sum().item())
            total += int(labels.size(0))
        scheduler.step()

        train_metrics = {
            "loss": running_loss / max(len(train_loader), 1),
            "acc": correct / max(total, 1),
        }
        val_metrics = evaluate(model, val_loader, device)
        full_metrics = evaluate(model, full_game_val, device)
        history["train_loss"].append(train_metrics["loss"])
        history["train_acc"].append(train_metrics["acc"])
        history["val_loss"].append(val_metrics["loss"])
        history["val_acc"].append(val_metrics["acc"])
        history["val_auc"].append(val_metrics["auc"])
        history["val_full_auc"].append(full_metrics["auc"])

        print(
            f"E{epoch:02d} | loss {train_metrics['loss']:.4f}/{val_metrics['loss']:.4f} "
            f"| acc {train_metrics['acc']:.3f}/{val_metrics['acc']:.3f} "
            f"| auc {val_metrics['auc']:.3f} | full {full_metrics['auc']:.3f}",
            flush=True,
        )

        # Prefer full-game ranking quality for checkpoint selection.
        score = full_metrics["auc"]
        if score > best_val_auc + 1e-4:
            best_val_auc = score
            best_val_acc = full_metrics["acc"]
            bad_epochs = 0
            torch.save(
                {
                    "epoch": epoch,
                    "model_state_dict": model.state_dict(),
                    "vocab": vocab.to_dict(),
                    "config": {
                        "d_model": d_model,
                        "num_layers": num_layers,
                        "vocab_size": vocab.vocab_size,
                        "continuous_dim": CONTINUOUS_DIM,
                        "global_dim": GLOBAL_DIM,
                        "dropout": dropout,
                    },
                    "val_metrics": {
                        "acc": full_metrics["acc"],
                        "auc": full_metrics["auc"],
                        "loss": full_metrics["loss"],
                        "mixed_auc": val_metrics["auc"],
                    },
                },
                best_path,
            )
            print(f"  saved best checkpoint (full_auc={best_val_auc:.4f})")
        else:
            bad_epochs += 1
            if bad_epochs >= patience:
                print(f"Early stop at epoch {epoch} (patience={patience})")
                break

    checkpoint = torch.load(best_path, map_location=device, weights_only=False)
    model.load_state_dict(checkpoint["model_state_dict"])
    test_metrics = evaluate(model, test_loader, device)

    ratio_report = {}
    for ratio in (0.5, 0.75, 0.9, 1.0):
        ds = WinnerSequenceDataset(
            test_battles,
            vocab,
            costs,
            sample_ratios=[ratio],
            augment_swap=False,
        )
        loader = DataLoader(
            ds, batch_size=batch_size, shuffle=False, collate_fn=collate_winner_batch
        )
        metrics = evaluate(model, loader, device)
        ratio_report[str(ratio)] = {
            "acc": metrics["acc"],
            "auc": metrics["auc"],
            "n": metrics["n"],
        }

    report = {
        "device": str(device),
        "gpu_name": torch.cuda.get_device_name(0) if device.type == "cuda" else None,
        "seconds": round(time.time() - started, 1),
        "battles_total": len(battles),
        "majority_baseline": majority,
        "splits": split_stats,
        "sequences": {
            "train": len(train_loader.dataset),
            "val": len(val_loader.dataset),
            "test": len(test_loader.dataset),
        },
        "best_val_auc": best_val_auc,
        "best_val_acc": best_val_acc,
        "test": {
            "acc": test_metrics["acc"],
            "auc": test_metrics["auc"],
            "loss": test_metrics["loss"],
            "n": test_metrics["n"],
        },
        "test_by_prefix_ratio": ratio_report,
        "history": history,
        "checkpoint": str(best_path),
    }

    report_path = output / "report.json"
    with report_path.open("w") as handle:
        json.dump(report, handle, indent=2)
    with (output / "vocab.json").open("w") as handle:
        json.dump(vocab.to_dict(), handle, indent=2)
    torch.save(
        {
            "model_state_dict": model.state_dict(),
            "vocab": vocab.to_dict(),
            "config": checkpoint["config"],
            "report": {
                "test_acc": test_metrics["acc"],
                "test_auc": test_metrics["auc"],
                "best_val_auc": best_val_auc,
                "best_val_acc": best_val_acc,
            },
        },
        output / "final_model.pt",
    )

    try:
        import matplotlib.pyplot as plt

        epochs_axis = range(1, len(history["train_loss"]) + 1)
        fig, axes = plt.subplots(1, 3, figsize=(12, 3.5))
        axes[0].plot(epochs_axis, history["train_loss"], label="train")
        axes[0].plot(epochs_axis, history["val_loss"], label="val")
        axes[0].set_title("Loss")
        axes[0].legend()
        axes[1].plot(epochs_axis, history["train_acc"], label="train")
        axes[1].plot(epochs_axis, history["val_acc"], label="val")
        axes[1].set_title("Accuracy")
        axes[1].legend()
        axes[2].plot(epochs_axis, history["val_auc"], label="mixed")
        axes[2].plot(epochs_axis, history["val_full_auc"], label="full", color="green")
        axes[2].set_title("Val AUC")
        axes[2].legend()
        fig.tight_layout()
        fig.savefig(output / "training_curves.png", dpi=140)
        plt.close(fig)
    except Exception as exc:  # pragma: no cover - plotting is best-effort
        report["plot_error"] = str(exc)

    print(json.dumps({"test": report["test"], "by_ratio": ratio_report}, indent=2))
    print(f"Wrote {report_path}")
    return report
