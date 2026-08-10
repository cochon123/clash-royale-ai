"""Evaluate a policy checkpoint on a fixed battle manifest."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

import torch
from torch.utils.data import DataLoader

from cr_replay_pipeline.policy_dataset import (
    PolicyActionDataset,
    collect_battles,
    collate_policy_batch,
    load_card_costs,
)
from cr_replay_pipeline.policy_infer import load_policy
from cr_replay_pipeline.policy_manifest import battles_from_manifest, load_manifest
from cr_replay_pipeline.policy_train import evaluate_policy


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--model-dir", required=True)
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--raw-dir", default="data/raw")
    parser.add_argument("--card-costs", default="data/card_costs.json")
    parser.add_argument("--batch-size", type=int, default=512)
    parser.add_argument("--device", default=None)
    parser.add_argument("--output", required=True)
    parser.add_argument("--arena-control", choices=["aligned", "shuffled"], default="aligned")
    args = parser.parse_args()

    model, vocab, cfg, device = load_policy(args.model_dir, device_name=args.device)
    manifest = load_manifest(args.manifest)
    battles = collect_battles(args.raw_dir, min_card_plays=int(manifest.get("min_card_plays", 12)))
    _, _, test_battles = battles_from_manifest(battles, manifest)
    costs = load_card_costs(args.card_costs)
    dataset = PolicyActionDataset(
        test_battles,
        vocab,
        costs,
        max_context=int(cfg.get("max_context", 64)),
        max_samples_per_battle=int(cfg.get("max_samples_per_battle", 8)),
        stride=3,
        seed=int(manifest.get("seed", 42)),
        threat_dim=int(cfg.get("threat_dim", 0)),
        reaction_weight=1.0,
        prefer_reactions=False,
    )
    loader = DataLoader(
        dataset,
        batch_size=args.batch_size,
        shuffle=False,
        num_workers=0,
        collate_fn=collate_policy_batch,
    )
    metrics = evaluate_policy(
        model,
        loader,
        device,
        loss_kwargs={"slot_weight": 0.0, "type_weight": 0.0, "zone_weight": 0.0, "xy_weight": 0.0, "timing_weight": 0.0, "tile_weight": 1.0}
        if cfg.get("arena_memory_channels")
        else {},
        arena_control=args.arena_control,
    )
    result = {
        "model_dir": args.model_dir,
        "manifest": args.manifest,
        "manifest_hash": manifest["ordered_id_sha256"],
        "split": "test",
        "battles": len(test_battles),
        "samples": len(dataset),
        "arena_control": args.arena_control,
        "metrics": metrics,
    }
    target = Path(args.output)
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
