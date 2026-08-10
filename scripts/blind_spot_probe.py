"""Cheap post-training probes for policy blind spots.

Uses replay files collected after policy-bc-v5's creation time so all
perturbations are compared on one fixed, plausibly unseen slice.  This is an
evaluation probe, not a training run.
"""

from __future__ import annotations

import argparse
import json
import math
import pickle
import time
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path

import numpy as np
import torch
from torch.utils.data import DataLoader

from cr_replay_pipeline.policy_dataset import (
    THREAT_DIM,
    PolicyActionDataset,
    collate_policy_batch,
    load_card_costs,
    recent_opponent_threat,
)
from cr_replay_pipeline.policy_infer import load_policy
from cr_replay_pipeline.policy_manifest import battles_from_manifest, load_manifest


def _metrics() -> dict[str, float]:
    return defaultdict(float)


def _finish(values: dict[str, float]) -> dict[str, float]:
    n = max(values["n"], 1.0)
    mean_x = values.get("pred_x_sum", 0.0) / n
    mean_y = values.get("pred_y_sum", 0.0) / n
    arg_n = max(values.get("argmax_n", 0.0), 1.0)
    arg_x = values.get("argmax_x_sum", 0.0) / arg_n
    arg_y = values.get("argmax_y_sum", 0.0) / arg_n
    return {
        "n": int(values["n"]),
        "slot_top1": values["slot_top1"] / n,
        "slot_top3": values["slot_top3"] / n,
        "slot_nll": values["slot_nll"] / n,
        "slot_entropy_bits": values["slot_entropy"] / n,
        "zone_acc": values["zone_acc"] / n,
        "xy_mae_units": values["xy_mae"] / n,
        "expected_x_std": math.sqrt(max(values.get("pred_x2_sum", 0.0) / n - mean_x * mean_x, 0.0)),
        "expected_y_std": math.sqrt(max(values.get("pred_y2_sum", 0.0) / n - mean_y * mean_y, 0.0)),
        "argmax_x_std": math.sqrt(max(values.get("argmax_x2_sum", 0.0) / arg_n - arg_x * arg_x, 0.0))
        if values.get("argmax_n", 0)
        else None,
        "argmax_y_std": math.sqrt(max(values.get("argmax_y2_sum", 0.0) / arg_n - arg_y * arg_y, 0.0))
        if values.get("argmax_n", 0)
        else None,
        "tile_top1": values["tile_top1"] / n if values.get("tile_n", 0) else None,
        "tile_top5": values["tile_top5"] / n if values.get("tile_n", 0) else None,
        "tile_nll": values["tile_nll"] / n if values.get("tile_n", 0) else None,
        "arena_gate": values["arena_gate"] / n if values.get("arena_n", 0) else None,
        "arena_residual_abs": values["arena_residual_abs"] / n
        if values.get("arena_n", 0)
        else None,
    }


def _update(
    values: dict[str, float],
    outputs: dict[str, torch.Tensor],
    slots: torch.Tensor,
    zones: torch.Tensor,
    xy: torch.Tensor,
    row_mask: torch.Tensor | None = None,
) -> None:
    probs = outputs["slot_probs"]
    if row_mask is None:
        row_mask = torch.ones(slots.size(0), dtype=torch.bool, device=slots.device)
    if not bool(row_mask.any()):
        return
    probs = probs[row_mask]
    slots = slots[row_mask]
    zones = zones[row_mask]
    xy = xy[row_mask]
    zone_logits = outputs["zone_logits"][row_mask]
    pred_xy = outputs["xy"][row_mask]
    rows = torch.arange(slots.size(0), device=slots.device)
    values["n"] += slots.numel()
    values["slot_top1"] += (probs.argmax(-1) == slots).sum().item()
    values["slot_top3"] += (
        probs.topk(3, dim=-1).indices == slots.unsqueeze(-1)
    ).any(-1).sum().item()
    values["slot_nll"] += (-torch.log(probs[rows, slots].clamp_min(1e-9))).sum().item()
    values["slot_entropy"] += (
        -(probs * torch.log2(probs.clamp_min(1e-9))).sum(-1)
    ).sum().item()
    values["zone_acc"] += (zone_logits.argmax(-1) == zones).sum().item()
    values["xy_mae"] += (
        (pred_xy - xy).abs().mul(pred_xy.new_tensor([18000.0, 32000.0])).sum(-1)
    ).sum().item()
    values["pred_x_sum"] += pred_xy[:, 0].sum().item()
    values["pred_y_sum"] += pred_xy[:, 1].sum().item()
    values["pred_x2_sum"] += pred_xy[:, 0].square().sum().item()
    values["pred_y2_sum"] += pred_xy[:, 1].square().sum().item()
    tile_logits = outputs.get("tile_logits")
    if tile_logits is not None:
        target_col = (xy[:, 0] * 32.0).long().clamp(0, 31)
        target_row = (xy[:, 1] * 18.0).long().clamp(0, 17)
        target_tile = target_row * 32 + target_col
        tile_logits = tile_logits[row_mask]
        argmax = tile_logits.argmax(-1)
        arg_y = torch.div(argmax, 32, rounding_mode="floor").float().add(0.5).div(18.0)
        arg_x = argmax.remainder(32).float().add(0.5).div(32.0)
        values["argmax_n"] += argmax.numel()
        values["argmax_x_sum"] += arg_x.sum().item()
        values["argmax_y_sum"] += arg_y.sum().item()
        values["argmax_x2_sum"] += arg_x.square().sum().item()
        values["argmax_y2_sum"] += arg_y.square().sum().item()
        values["tile_n"] += target_tile.numel()
        values["tile_top1"] += (tile_logits.argmax(-1) == target_tile).sum().item()
        values["tile_top5"] += (
            tile_logits.topk(5, dim=-1).indices == target_tile.unsqueeze(-1)
        ).any(-1).sum().item()
        values["tile_nll"] += torch.nn.functional.cross_entropy(
            tile_logits, target_tile, reduction="sum"
        ).item()
    if outputs.get("arena_gate") is not None:
        values["arena_n"] += slots.numel()
        values["arena_gate"] += outputs["arena_gate"][row_mask].sum().item()
        values["arena_residual_abs"] += (
            outputs["arena_residual_logits"][row_mask].abs().mean(dim=-1).sum().item()
        )


def _revealed_opponent_deck(
    opponent_deck: torch.Tensor,
    card_ids: torch.Tensor,
    continuous: torch.Tensor,
    lengths: torch.Tensor,
    unknown_id: int,
) -> tuple[torch.Tensor, torch.Tensor]:
    """Replace unrevealed opponent cards with UNK, preserving revealed slots."""
    revealed = torch.full_like(opponent_deck, unknown_id)
    counts = torch.zeros(opponent_deck.size(0), device=opponent_deck.device)
    for row in range(opponent_deck.size(0)):
        length = int(lengths[row].item())
        # Samples are normalized to the acting player's perspective: side=0
        # in the encoded history is therefore the opponent.
        opp_history = card_ids[row, :length][continuous[row, :length, 1] < 0.5]
        seen = set(int(v) for v in opp_history.tolist())
        keep = torch.tensor(
            [int(v) in seen for v in opponent_deck[row].tolist()],
            dtype=torch.bool,
            device=opponent_deck.device,
        )
        revealed[row, keep] = opponent_deck[row, keep]
        counts[row] = keep.sum()
    return revealed, counts


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--model-dir", default="models/policy_bc_v4.1")
    parser.add_argument("--cache", default="data/winner_battles_cache.pkl")
    parser.add_argument("--raw-dir", default="data/raw")
    parser.add_argument("--collected-after", default="2026-08-09T07:20:01-06:00")
    parser.add_argument("--max-battles", type=int, default=512)
    parser.add_argument("--batch-size", type=int, default=512)
    parser.add_argument("--output", default="reports/blind_spot_probe_v1.json")
    parser.add_argument("--split-manifest", default=None)
    args = parser.parse_args()

    started = time.time()
    with Path(args.cache).open("rb") as handle:
        cache = pickle.load(handle)
    if args.split_manifest:
        manifest = load_manifest(args.split_manifest)
        _, _, battles = battles_from_manifest(cache["battles"], manifest)
        battles = battles[: args.max_battles]
        new_ids = set(battle.battle_id for battle in battles)
    else:
        cutoff = datetime.fromisoformat(args.collected_after).timestamp()
        new_ids = {
            path.stem
            for path in Path(args.raw_dir).rglob("*.json")
            if path.stat().st_mtime > cutoff
        }
        battles = [battle for battle in cache["battles"] if battle.battle_id in new_ids]
        battles = battles[: args.max_battles]
    if not battles:
        raise RuntimeError("No cached battles match the post-training file slice")

    model, vocab, cfg, device = load_policy(args.model_dir)
    costs = load_card_costs("data/card_costs.json")
    threat_dim = int(cfg.get("threat_dim", 0))
    dataset = PolicyActionDataset(
        battles,
        vocab,
        costs,
        max_context=int(cfg.get("max_context", 64)),
        max_samples_per_battle=24,
        stride=3,
        seed=20260809,
        threat_dim=threat_dim,
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

    winner_flags: list[bool] = []
    for battle_i, event_i in dataset.index:
        battle = dataset.battles[battle_i]
        side = battle.events[event_i]["side"]
        winner_flags.append(bool(battle.team_wins) if side == "team" else not bool(battle.team_wins))

    conditions = {
        "oracle_full_opponent_deck": _metrics(),
        "revealed_opponent_cards_only": _metrics(),
        "no_opponent_deck": _metrics(),
        "no_recent_threat_vector": _metrics(),
        "neutralized_history_xy": _metrics(),
        "arena_memory_disabled": _metrics(),
        "arena_memory_zeroed": _metrics(),
        "arena_memory_shuffled": _metrics(),
    }
    outcome = {"winner_actions": _metrics(), "loser_actions": _metrics()}
    reaction_slices = {
        "responses_within_5s": _metrics(),
        "non_response_actions": _metrics(),
    }
    card_slices: dict[str, dict[str, float]] = defaultdict(_metrics)
    sample_reactions: list[bool] = []
    sample_cards: list[str] = []
    for battle_i, event_i in dataset.index:
        battle = dataset.battles[battle_i]
        target = battle.events[event_i]
        _threat, is_reaction = recent_opponent_threat(
            battle,
            event_i,
            acting_side=target["side"],
            costs=costs,
            max_age=5.0,
        )
        sample_reactions.append(bool(is_reaction))
        sample_cards.append(str(target["card"]))
    target_xy_rows: list[np.ndarray] = []
    pred_xy_rows: list[np.ndarray] = []
    tile_argmax_xy_rows: list[np.ndarray] = []
    tile_top1_correct = 0
    tile_top5_correct = 0
    tile_n = 0
    card_rows: list[np.ndarray] = []
    revealed_counts: list[np.ndarray] = []
    arena_examples: list[dict[str, object]] = []
    offset = 0

    model.eval()
    with torch.no_grad():
        for batch in loader:
            (
                continuous,
                card_ids,
                team_deck,
                opponent_deck,
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
            ) = [item.to(device) if torch.is_tensor(item) else item for item in batch]

            debug = int(cfg.get("arena_memory_channels", 0)) > 0

            def infer(
                opp: torch.Tensor,
                glob: torch.Tensor,
                cont: torch.Tensor,
                *,
                arena_permutation: torch.Tensor | None = None,
                disable_arena: bool = False,
                zero_arena_memory: bool = False,
            ):
                return model(
                    cont,
                    card_ids,
                    team_deck,
                    opp,
                    glob,
                    lengths,
                    slot_feats,
                    hand_mask,
                    arena_permutation=arena_permutation,
                    disable_arena=disable_arena,
                    zero_arena_memory=zero_arena_memory,
                    return_debug=debug,
                )

            base = infer(opponent_deck, globals_, continuous)
            _update(conditions["oracle_full_opponent_deck"], base, slots, zones, xy)
            if debug and len(arena_examples) < 3:
                memory = base["arena_memory"].detach().cpu()
                per_tile_memory = memory.permute(0, 2, 3, 1).reshape(memory.size(0), -1, 16)
                base_probs = torch.softmax(base["base_tile_logits"], dim=-1).detach().cpu()
                final_probs = torch.softmax(base["tile_logits"], dim=-1).detach().cpu()
                residual = base["arena_residual_logits"].detach().cpu()
                for row in range(min(3 - len(arena_examples), slots.size(0))):
                    arena_examples.append(
                        {
                            "label": f"sample {len(arena_examples) + 1}",
                            "memory": per_tile_memory[row].tolist(),
                            "base": base_probs[row].tolist(),
                            "residual": residual[row].tolist(),
                            "final": final_probs[row].tolist(),
                        }
                    )

            revealed, counts = _revealed_opponent_deck(
                opponent_deck, card_ids, continuous, lengths, vocab.unk_id
            )
            revealed_out = infer(revealed, globals_, continuous)
            _update(conditions["revealed_opponent_cards_only"], revealed_out, slots, zones, xy)
            unknown = torch.full_like(opponent_deck, vocab.unk_id)
            _update(conditions["no_opponent_deck"], infer(unknown, globals_, continuous), slots, zones, xy)

            no_threat = globals_.clone()
            if threat_dim:
                no_threat[:, -threat_dim:] = 0.0
            _update(conditions["no_recent_threat_vector"], infer(opponent_deck, no_threat, continuous), slots, zones, xy)

            neutral_xy = continuous.clone()
            neutral_xy[:, :, 4:6] = 0.5
            _update(conditions["neutralized_history_xy"], infer(opponent_deck, globals_, neutral_xy), slots, zones, xy)
            _update(
                conditions["arena_memory_disabled"],
                infer(opponent_deck, globals_, continuous, disable_arena=True),
                slots,
                zones,
                xy,
            )
            _update(
                conditions["arena_memory_zeroed"],
                infer(opponent_deck, globals_, continuous, zero_arena_memory=True),
                slots,
                zones,
                xy,
            )
            permutation = torch.roll(
                torch.arange(continuous.size(0), device=device), shifts=1
            ) if continuous.size(0) > 1 else None
            if permutation is not None:
                _update(
                    conditions["arena_memory_shuffled"],
                    infer(
                        opponent_deck,
                        globals_,
                        continuous,
                        arena_permutation=permutation,
                    ),
                    slots,
                    zones,
                    xy,
                )

            flags = torch.tensor(
                winner_flags[offset : offset + slots.size(0)], device=device, dtype=torch.bool
            )
            _update(outcome["winner_actions"], base, slots, zones, xy, flags)
            _update(outcome["loser_actions"], base, slots, zones, xy, ~flags)
            reaction_flags = torch.tensor(
                sample_reactions[offset : offset + slots.size(0)],
                device=device,
                dtype=torch.bool,
            )
            _update(
                reaction_slices["responses_within_5s"],
                base,
                slots,
                zones,
                xy,
                reaction_flags,
            )
            _update(
                reaction_slices["non_response_actions"],
                base,
                slots,
                zones,
                xy,
                ~reaction_flags,
            )
            for row, card in enumerate(sample_cards[offset : offset + slots.size(0)]):
                row_mask = torch.zeros(slots.size(0), device=device, dtype=torch.bool)
                row_mask[row] = True
                _update(card_slices[card], base, slots, zones, xy, row_mask)
            offset += slots.size(0)

            target_xy_rows.append(xy.cpu().numpy())
            pred_xy_rows.append(base["xy"].cpu().numpy())
            tile_logits = base.get("tile_logits")
            if tile_logits is not None:
                tile_pred = tile_logits.argmax(dim=-1)
                tile_row = torch.div(tile_pred, 32, rounding_mode="floor")
                tile_col = tile_pred.remainder(32)
                tile_argmax_xy = torch.stack(
                    [
                        (tile_col.float() + 0.5) / 32.0,
                        (tile_row.float() + 0.5) / 18.0,
                    ],
                    dim=-1,
                )
                target_col = (xy[:, 0] * 32.0).long().clamp(0, 31)
                target_row = (xy[:, 1] * 18.0).long().clamp(0, 17)
                target_tile = target_row * 32 + target_col
                tile_top1_correct += (tile_pred == target_tile).sum().item()
                tile_top5_correct += (
                    tile_logits.topk(5, dim=-1).indices == target_tile.unsqueeze(-1)
                ).any(dim=-1).sum().item()
                tile_n += int(target_tile.numel())
                tile_argmax_xy_rows.append(tile_argmax_xy.cpu().numpy())
            card_rows.append(team_deck.gather(1, slots[:, None]).squeeze(1).cpu().numpy())
            revealed_counts.append(counts.cpu().numpy())

    target_xy = np.concatenate(target_xy_rows)
    pred_xy = np.concatenate(pred_xy_rows)
    target_card = np.concatenate(card_rows)
    reveal_n = np.concatenate(revealed_counts)
    scale = np.array([18000.0, 32000.0])

    per_card = []
    for card_id in np.unique(target_card):
        mask = target_card == card_id
        if mask.sum() < 30:
            continue
        per_card.append(
            {
                "card": vocab.id_to_name.get(int(card_id), "<unk>"),
                "n": int(mask.sum()),
                "human_x_std": float(target_xy[mask, 0].std()),
                "model_x_std": float(pred_xy[mask, 0].std()),
                "human_y_std": float(target_xy[mask, 1].std()),
                "model_y_std": float(pred_xy[mask, 1].std()),
            }
        )
    weights = np.array([row["n"] for row in per_card], dtype=float)
    within_card = {}
    for key in ("human_x_std", "model_x_std", "human_y_std", "model_y_std"):
        within_card[key] = float(np.average([row[key] for row in per_card], weights=weights))

    heatmap_decode: dict[str, Any] = {}
    if tile_argmax_xy_rows:
        tile_argmax_xy = np.concatenate(tile_argmax_xy_rows)
        heatmap_decode = {
            "expected_xy": {
                "mean_l1_units": float(
                    (np.abs(target_xy - pred_xy) * scale).sum(1).mean()
                ),
                "x_std": float(pred_xy[:, 0].std()),
                "y_std": float(pred_xy[:, 1].std()),
            },
            "argmax_tile": {
                "mean_l1_units": float(
                    (np.abs(target_xy - tile_argmax_xy) * scale).sum(1).mean()
                ),
                "x_std": float(tile_argmax_xy[:, 0].std()),
                "y_std": float(tile_argmax_xy[:, 1].std()),
                "tile_top1": tile_top1_correct / max(tile_n, 1),
                "tile_top5": tile_top5_correct / max(tile_n, 1),
            },
            "human": {
                "x_std": float(target_xy[:, 0].std()),
                "y_std": float(target_xy[:, 1].std()),
            },
        }

    result = {
        "name": "blind-spot-probe-v7" if debug else "blind-spot-probe-v1",
        "created_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "seconds": round(time.time() - started, 1),
        "model_dir": args.model_dir,
        "compute": {"device": str(device), "evaluation_only": True},
        "data": {
            "cache_file_count": cache.get("file_count"),
            "post_training_raw_ids": len(new_ids),
            "post_training_usable_battles": len(battles),
            "samples": len(dataset),
            "cutoff": args.collected_after,
            "mean_revealed_opponent_cards": float(reveal_n.mean()),
            "revealed_opponent_cards_p10": float(np.quantile(reveal_n, 0.1)),
            "revealed_opponent_cards_p90": float(np.quantile(reveal_n, 0.9)),
        },
        "ablations": {name: _finish(values) for name, values in conditions.items()},
        "outcome_slice": {name: _finish(values) for name, values in outcome.items()},
        "placement_slices": {
            "reaction": {
                name: _finish(values) for name, values in reaction_slices.items()
            },
            "per_card": {
                card: _finish(values)
                for card, values in card_slices.items()
                if values["n"] >= 100
            },
        },
        "placement_spread": {
            "global": {
                "human_x_std": float(target_xy[:, 0].std()),
                "model_x_std": float(pred_xy[:, 0].std()),
                "human_y_std": float(target_xy[:, 1].std()),
                "model_y_std": float(pred_xy[:, 1].std()),
                "human_to_model_x_std_ratio": float(target_xy[:, 0].std() / max(pred_xy[:, 0].std(), 1e-9)),
                "human_to_model_y_std_ratio": float(target_xy[:, 1].std() / max(pred_xy[:, 1].std(), 1e-9)),
                "mean_l1_units": float((np.abs(target_xy - pred_xy) * scale).sum(1).mean()),
            },
            "weighted_within_card": within_card,
            "heatmap_decode": heatmap_decode,
            "per_card": sorted(per_card, key=lambda row: row["n"], reverse=True),
        },
        "arena_examples": arena_examples,
        "notes": [
            "All conditions use exactly the same samples and frozen checkpoint.",
            "Full opponent deck is an oracle feature in offline evaluation and dual-phone self-play.",
            "UNK replacement is distribution shift because the current policy was not trained with hidden opponent cards.",
            "Spread compares deterministic one-step predictions with held-out human targets; it is not a gameplay score.",
        ],
    }
    output = Path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
