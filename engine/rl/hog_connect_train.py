"""Train a small Hog-connect value head and expose it as a placement prior.

The head maps a board raster and a placement tile to expected enemy tower
damage. ``ConnectPlacementPrior.scale`` is zero, so attaching it to
``BoardHogPolicy`` and enabling it does not change tile logits until the
scale moves. The policy flag itself defaults to off.
"""
from __future__ import annotations

import argparse
import json
import math
from pathlib import Path

import numpy as np
import torch
from torch import nn
from torch.nn import functional as F

from .hog_connect_dataset import load_rows

ROOT = Path(__file__).resolve().parents[1]
PRIOR_FORMAT = "hog-connect-v1"


class ConnectValueNet(nn.Module):
    """Raster cell plus tile coordinates to one damage number."""

    def __init__(self):
        super().__init__()
        self.conv = nn.Sequential(
            nn.Conv2d(8, 16, kernel_size=3, padding=1),
            nn.ReLU(),
            nn.Conv2d(16, 16, kernel_size=3, padding=1),
            nn.ReLU(),
        )
        self.scorer = nn.Linear(16 + 3, 1)

    def _cells(self, raster: torch.Tensor) -> torch.Tensor:
        return self.conv(raster)

    def forward(self, raster: torch.Tensor, tiles: torch.Tensor) -> torch.Tensor:
        cells = self._cells(raster)
        batch = raster.shape[0]
        played = (tiles >= 0).to(cells.dtype)
        row = torch.div(tiles.clamp(min=0), 32, rounding_mode="floor").clamp(0, 17)
        col = (tiles.clamp(min=0) % 32).clamp(0, 31)
        gathered = cells[torch.arange(batch, device=cells.device), :, row, col]
        gathered = gathered * played[:, None]
        coords = torch.stack((row.to(cells.dtype) / 17.0, col.to(cells.dtype) / 31.0), dim=-1)
        coords = coords * played[:, None]
        return self.scorer(torch.cat((gathered, coords, played[:, None]), dim=-1)).squeeze(-1)

    def tile_values(self, raster: torch.Tensor) -> torch.Tensor:
        """Expected damage at all 576 tiles. Shape ``[batch, 576]``."""
        cells = self._cells(raster)
        batch = cells.shape[0]
        flat = cells.flatten(2).transpose(1, 2)
        rows = torch.arange(18, device=raster.device).repeat_interleave(32)
        cols = torch.arange(32, device=raster.device).repeat(18)
        coords = torch.stack((rows.to(cells.dtype) / 17.0, cols.to(cells.dtype) / 31.0), dim=-1)
        coords = coords.unsqueeze(0).expand(batch, -1, -1)
        played = torch.ones(batch, 576, 1, device=raster.device, dtype=cells.dtype)
        return self.scorer(torch.cat((flat, coords, played), dim=-1)).squeeze(-1)


class ConnectPlacementPrior(nn.Module):
    """Zero-scale residual from connect values onto the 576 tile logits."""

    def __init__(self, net: ConnectValueNet | None = None):
        super().__init__()
        self.net = ConnectValueNet() if net is None else net
        self.scale = nn.Parameter(torch.zeros(()))

    def residual(self, raster: torch.Tensor) -> torch.Tensor:
        return self.scale * self.net.tile_values(raster)


def _pearson(pred: torch.Tensor, actual: torch.Tensor) -> float:
    if pred.numel() < 2:
        return float("nan")
    left = pred - pred.mean()
    right = actual - actual.mean()
    denom = left.norm() * right.norm()
    if float(denom) < 1e-8:
        return float("nan")
    return float((left * right).sum() / denom)


def _groups(rows):
    grouped = {}
    for row in rows:
        grouped.setdefault((row["game"], row["decision"]), []).append(row)
    return list(grouped.values())


def evaluate(net: ConnectValueNet, rows: list[dict]) -> dict[str, float]:
    if not rows:
        return {"rows": 0, "decisions": 0, "pearson": float("nan"), "mae": float("nan"), "top1_accuracy": float("nan")}
    raster = torch.tensor(np.stack([row["raster"] for row in rows]), dtype=torch.float32)
    tiles = torch.tensor([row["tile"] for row in rows], dtype=torch.long)
    actual = torch.tensor([row["tower_damage"] for row in rows], dtype=torch.float32)
    with torch.no_grad():
        pred = net(raster, tiles)
    mae = float((pred - actual).abs().mean())
    correct = 0
    groups = _groups(rows)
    predictions = pred.tolist()
    cursor = 0
    # evaluate() walks rows in the same order they were stacked.
    by_identity = {id(row): predictions[index] for index, row in enumerate(rows)}
    for group in groups:
        best_actual = max(group, key=lambda row: (row["tower_damage"], -row["tile"]))
        best_pred = max(group, key=lambda row: (by_identity[id(row)], -row["tile"]))
        correct += int(best_pred["tile"] == best_actual["tile"])
        cursor += len(group)
    return {
        "rows": len(rows),
        "decisions": len(groups),
        "pearson": _pearson(pred, actual),
        "mae": mae,
        "top1_accuracy": correct / max(1, len(groups)),
    }


def split_rows(rows: list[dict]) -> tuple[list[dict], list[dict], str]:
    games = sorted({int(row["game"]) for row in rows})
    if len(games) >= 2:
        held = games[-1]
        train = [row for row in rows if int(row["game"]) != held]
        test = [row for row in rows if int(row["game"]) == held]
        return train, test, f"held out game {held}"
    decisions = sorted({(int(row["game"]), int(row["decision"])) for row in rows})
    if len(decisions) < 2:
        return rows, [], "too few decisions for a held-out split"
    cut = max(1, int(math.floor(len(decisions) * 0.75)))
    train_keys = set(decisions[:cut])
    train, test = [], []
    for row in rows:
        key = (int(row["game"]), int(row["decision"]))
        (train if key in train_keys else test).append(row)
    return train, test, "held out the last quarter of decisions in the only game"


def train(manifest: Path, out: Path, *, steps: int = 200, lr: float = 1e-3, seed: int = 0) -> dict:
    rows = load_rows(manifest)
    train_rows, test_rows, split = split_rows(rows)
    torch.manual_seed(seed)
    net = ConnectValueNet()
    if train_rows:
        raster = torch.tensor(np.stack([row["raster"] for row in train_rows]), dtype=torch.float32)
        tiles = torch.tensor([row["tile"] for row in train_rows], dtype=torch.long)
        target = torch.tensor([row["tower_damage"] for row in train_rows], dtype=torch.float32)
        optimizer = torch.optim.Adam(net.parameters(), lr=lr)
        net.train()
        for _ in range(int(steps)):
            optimizer.zero_grad(set_to_none=True)
            loss = F.mse_loss(net(raster, tiles), target)
            loss.backward()
            optimizer.step()
    net.eval()
    metrics = {
        "format": PRIOR_FORMAT,
        "rows": len(rows),
        "train_rows": len(train_rows),
        "test_rows": len(test_rows),
        "split": split,
        "steps": int(steps) if train_rows else 0,
        "train": evaluate(net, train_rows),
        "held_out": evaluate(net, test_rows),
    }
    out = Path(out)
    out.parent.mkdir(parents=True, exist_ok=True)
    torch.save({"format": PRIOR_FORMAT, "net": net.state_dict(), "metrics": metrics}, out)
    metrics_path = out.with_suffix(".json")
    metrics_path.write_text(json.dumps(metrics, indent=2) + "\n")
    return metrics


def load_connect_prior(path: Path, device: str = "cpu") -> ConnectPlacementPrior:
    payload = torch.load(Path(path), map_location=device, weights_only=False)
    if payload.get("format") != PRIOR_FORMAT:
        raise ValueError(f"expected {PRIOR_FORMAT}, got {payload.get('format')}")
    net = ConnectValueNet()
    net.load_state_dict(payload["net"])
    prior = ConnectPlacementPrior(net)
    prior.to(device)
    prior.eval()
    return prior


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--out", type=Path, default=ROOT / "analysis-local/hog-connect/connect-value.pt")
    parser.add_argument("--steps", type=int, default=200)
    parser.add_argument("--lr", type=float, default=1e-3)
    parser.add_argument("--seed", type=int, default=0)
    args = parser.parse_args(argv)
    metrics = train(args.manifest, args.out, steps=args.steps, lr=args.lr, seed=args.seed)
    print(json.dumps(metrics, indent=2), flush=True)


if __name__ == "__main__":
    main()
