"""Extract interactive-report data comparing two policy checkpoints.

Produces arena heatmaps, per-card placement stats, "what v4 fixed" examples and
playable defense scenarios so the HTML report can show *what* improved instead
of only listing scalars. Offline only: frozen checkpoints, held-out battles.
"""

from __future__ import annotations

import json
import random
import time
from collections import Counter, defaultdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import numpy as np
import torch
from torch.nn.utils.rnn import pad_sequence

from .policy_dataset import (
    DEFAULT_MIN_CONTEXT,
    TILE_UNITS,
    _normalize_xy,
    deck_slot_for_card,
    encode_policy_sample,
    recent_opponent_threat,
)
from .policy_infer import load_policy
from .winner_dataset import (
    DEFAULT_ELIXIR_COST,
    WIN_CONDITIONISH,
    BattleExample,
    collect_battles,
    load_card_costs,
    split_battles,
)

ZONE_ROWS = 4  # y bands: own back → enemy half
ZONE_COLS = 3  # x lanes: left / center / right
ZONE_NAMES = (
    "own back left",
    "own back center",
    "own back right",
    "own front left",
    "own front center",
    "own front right",
    "river left",
    "river center",
    "river right",
    "enemy left",
    "enemy center",
    "enemy right",
)


def pretty_card(card: str) -> str:
    return card.replace("-", " ").title()


def _batched(items: list[Any], size: int):
    for start in range(0, len(items), size):
        yield items[start : start + size]


@torch.no_grad()
def _run_model(model, device: torch.device, samples: list[tuple]) -> dict[str, np.ndarray]:
    """Forward a list of encoded samples; returns slot/zone/xy predictions."""
    continuous = pad_sequence([s[0] for s in samples], batch_first=True).to(device)
    card_ids = pad_sequence([s[1] for s in samples], batch_first=True).to(device)
    team = torch.stack([s[2] for s in samples]).to(device)
    opp = torch.stack([s[3] for s in samples]).to(device)
    glob = torch.stack([s[4] for s in samples]).to(device)
    slot_feats = torch.stack([s[5] for s in samples]).to(device)
    hand = torch.stack([s[6] for s in samples]).to(device)
    lengths = torch.stack([s[12] for s in samples]).to(device)
    out = model(continuous, card_ids, team, opp, glob, lengths, slot_feats, hand)
    slot_probs = torch.softmax(out["slot_logits"], dim=-1).cpu().numpy()
    zone_probs = torch.softmax(out["zone_logits"], dim=-1).cpu().numpy()
    return {
        "slot_probs": slot_probs,
        "zone_probs": zone_probs,
        "xy": out["xy"].cpu().numpy(),
    }


def _zone_grid(counts: Counter, total: int) -> list[float]:
    if total <= 0:
        return [0.0] * (ZONE_ROWS * ZONE_COLS)
    return [counts.get(z, 0) / total for z in range(ZONE_ROWS * ZONE_COLS)]


def build_policy_showcase(
    input_dir: str | Path = "data/raw",
    new_policy_dir: str | Path = "models/policy_bc_v4",
    old_policy_dir: str | Path = "models/policy_bc_v3",
    card_costs_path: str | Path = "data/card_costs.json",
    output_path: str | Path = "reports/policy_showcase_v4.json",
    max_battles: int = 700,
    max_samples_per_battle: int = 10,
    batch_size: int = 128,
    top_cards: int = 12,
    seed: int = 42,
    device_name: str | None = None,
) -> dict[str, Any]:
    started = time.time()
    costs = load_card_costs(card_costs_path)
    battles = collect_battles(input_dir)
    _train, _val, test = split_battles(battles, seed=seed)

    new_model, vocab, new_cfg, device = load_policy(
        new_policy_dir, device_name=device_name
    )
    old_model, _old_vocab, old_cfg, _old_device = load_policy(
        old_policy_dir, device_name=device_name
    )
    new_threat = int(new_cfg.get("threat_dim", 0))
    old_threat = int(old_cfg.get("threat_dim", 0))
    max_context = int(new_cfg.get("max_context", 64))

    rng = random.Random(seed)
    pool = list(test)
    rng.shuffle(pool)
    pool = pool[:max_battles]
    print(
        f"Showcase: {len(pool)} held-out battles on {device} "
        f"(new threat_dim={new_threat}, old threat_dim={old_threat})",
        flush=True,
    )

    # ---- collect encoded samples + metadata -------------------------------
    new_samples: list[tuple] = []
    old_samples: list[tuple] = []
    meta: list[dict[str, Any]] = []

    for battle in pool:
        indices = list(range(DEFAULT_MIN_CONTEXT, len(battle.events), 3))
        if len(indices) > max_samples_per_battle:
            indices = sorted(rng.sample(indices, max_samples_per_battle))
        for event_index in indices:
            target = battle.events[event_index]
            if target["event_type"] != "card_play":
                continue
            acting_side = target["side"]
            deck = (
                battle.opponent_deck
                if acting_side == "opponent"
                else battle.team_deck
            )
            slot = deck_slot_for_card(deck, target["card"])
            if slot is None:
                continue
            sample_new = encode_policy_sample(
                battle,
                event_index,
                vocab,
                costs,
                max_context=max_context,
                threat_dim=new_threat,
            )
            if sample_new is None:
                continue
            if old_threat == new_threat:
                sample_old = sample_new
            else:
                sample_old = encode_policy_sample(
                    battle,
                    event_index,
                    vocab,
                    costs,
                    max_context=max_context,
                    threat_dim=old_threat,
                )
                if sample_old is None:
                    continue
            threat_feat, is_reaction = recent_opponent_threat(
                battle, event_index, acting_side, costs
            )
            hand = [bool(v) for v in sample_new[6].tolist()]
            new_samples.append(sample_new)
            old_samples.append(sample_old)
            meta.append(
                {
                    "battle_id": battle.battle_id,
                    "event_index": event_index,
                    "card": target["card"],
                    "slot": slot,
                    "deck": list(deck),
                    "hand": hand,
                    "true_zone": int(sample_new[9].item()),
                    "true_xy": [float(v) for v in sample_new[10].tolist()],
                    "is_reaction": bool(is_reaction),
                    "seconds": float(target["seconds"]),
                }
            )

    print(f"Scoring {len(meta):,} held-out actions with both models ...", flush=True)

    for start in range(0, len(meta), batch_size):
        chunk_new = new_samples[start : start + batch_size]
        chunk_old = old_samples[start : start + batch_size]
        out_new = _run_model(new_model, device, chunk_new)
        out_old = _run_model(old_model, device, chunk_old)
        for i in range(len(chunk_new)):
            row = meta[start + i]
            row["new_slot"] = int(out_new["slot_probs"][i].argmax())
            row["old_slot"] = int(out_old["slot_probs"][i].argmax())
            row["new_slot_p"] = float(out_new["slot_probs"][i].max())
            row["new_slot_top3"] = [
                int(x) for x in out_new["slot_probs"][i].argsort()[-3:][::-1]
            ]
            row["old_slot_top3"] = [
                int(x) for x in out_old["slot_probs"][i].argsort()[-3:][::-1]
            ]
            row["new_zone"] = int(out_new["zone_probs"][i].argmax())
            row["old_zone"] = int(out_old["zone_probs"][i].argmax())
            row["new_zone_p"] = float(out_new["zone_probs"][i].max())
            row["new_xy"] = [float(v) for v in out_new["xy"][i]]
            row["old_xy"] = [float(v) for v in out_old["xy"][i]]

    scored = [r for r in meta if "new_zone" in r]

    # ---- aggregate: overall arena heatmaps --------------------------------
    human_counts: Counter = Counter()
    new_counts: Counter = Counter()
    old_counts: Counter = Counter()
    for row in scored:
        human_counts[row["true_zone"]] += 1
        new_counts[row["new_zone"]] += 1
        old_counts[row["old_zone"]] += 1
    n_all = len(scored)

    def _xy_stats(pred_key: str) -> tuple[float, float]:
        if not scored:
            return 0.0, 0.0
        d = []
        for row in scored:
            dx = (row[pred_key][0] - row["true_xy"][0]) * 18000.0
            dy = (row[pred_key][1] - row["true_xy"][1]) * 32000.0
            d.append(float(np.sqrt(dx * dx + dy * dy)))
        arr = np.asarray(d)
        return float(arr.mean()), float((arr <= TILE_UNITS).mean())

    new_xy_mae, new_tile = _xy_stats("new_xy")
    old_xy_mae, old_tile = _xy_stats("old_xy")

    overall = {
        "n": n_all,
        "human_grid": _zone_grid(human_counts, n_all),
        "new_grid": _zone_grid(new_counts, n_all),
        "old_grid": _zone_grid(old_counts, n_all),
        "new_zone_acc": (
            sum(1 for r in scored if r["new_zone"] == r["true_zone"]) / max(n_all, 1)
        ),
        "old_zone_acc": (
            sum(1 for r in scored if r["old_zone"] == r["true_zone"]) / max(n_all, 1)
        ),
        "new_slot_acc": (
            sum(1 for r in scored if r["new_slot"] == r["slot"]) / max(n_all, 1)
        ),
        "old_slot_acc": (
            sum(1 for r in scored if r["old_slot"] == r["slot"]) / max(n_all, 1)
        ),
        "new_slot_top3": (
            sum(1 for r in scored if r["slot"] in r["new_slot_top3"]) / max(n_all, 1)
        ),
        "old_slot_top3": (
            sum(1 for r in scored if r["slot"] in r["old_slot_top3"]) / max(n_all, 1)
        ),
        "new_xy_mae": new_xy_mae,
        "old_xy_mae": old_xy_mae,
        "new_tile_acc": new_tile,
        "old_tile_acc": old_tile,
        "zone_entropy_human": _entropy(_zone_grid(human_counts, n_all)),
        "zone_entropy_new": _entropy(_zone_grid(new_counts, n_all)),
        "zone_entropy_old": _entropy(_zone_grid(old_counts, n_all)),
    }

    # ---- per-card placement ------------------------------------------------
    by_card: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in scored:
        by_card[row["card"]].append(row)
    ranked = sorted(by_card.items(), key=lambda kv: -len(kv[1]))[:top_cards]

    cards_out: list[dict[str, Any]] = []
    for card, rows in ranked:
        n = len(rows)
        hc, nc, oc = Counter(), Counter(), Counter()
        for r in rows:
            hc[r["true_zone"]] += 1
            nc[r["new_zone"]] += 1
            oc[r["old_zone"]] += 1
        scatter = [
            {
                "hx": round(r["true_xy"][0], 4),
                "hy": round(r["true_xy"][1], 4),
                "nx": round(r["new_xy"][0], 4),
                "ny": round(r["new_xy"][1], 4),
                "ox": round(r["old_xy"][0], 4),
                "oy": round(r["old_xy"][1], 4),
            }
            for r in rows[:90]
        ]
        cards_out.append(
            {
                "card": card,
                "label": pretty_card(card),
                "n": n,
                "cost": int(costs.get(card, DEFAULT_ELIXIR_COST)),
                "is_wincon": card in WIN_CONDITIONISH,
                "human_grid": _zone_grid(hc, n),
                "new_grid": _zone_grid(nc, n),
                "old_grid": _zone_grid(oc, n),
                "new_zone_acc": sum(1 for r in rows if r["new_zone"] == r["true_zone"]) / n,
                "old_zone_acc": sum(1 for r in rows if r["old_zone"] == r["true_zone"]) / n,
                "scatter": scatter,
            }
        )
    cards_out.sort(key=lambda c: -(c["new_zone_acc"] - c["old_zone_acc"]))

    # ---- "what v4 fixed" gallery ------------------------------------------
    fixes = [
        r
        for r in scored
        if r["new_zone"] == r["true_zone"] and r["old_zone"] != r["true_zone"]
    ]
    regressions = [
        r
        for r in scored
        if r["old_zone"] == r["true_zone"] and r["new_zone"] != r["true_zone"]
    ]
    rng.shuffle(fixes)
    gallery = [
        {
            "battle_id": r["battle_id"],
            "card": r["card"],
            "label": pretty_card(r["card"]),
            "seconds": round(r["seconds"], 1),
            "true_zone": r["true_zone"],
            "old_zone": r["old_zone"],
            "new_zone": r["new_zone"],
            "true_xy": [round(v, 4) for v in r["true_xy"]],
            "new_xy": [round(v, 4) for v in r["new_xy"]],
            "old_xy": [round(v, 4) for v in r["old_xy"]],
            "is_reaction": r["is_reaction"],
        }
        for r in fixes[:18]
    ]

    # ---- playable defense scenarios ---------------------------------------
    scenarios: list[dict[str, Any]] = []
    reaction_rows = [
        r for r in scored if r["is_reaction"] and sum(r["hand"]) == 4 and r["hand"][r["slot"]]
    ]
    rng.shuffle(reaction_rows)
    seen_battles: set[str] = set()
    for r in reaction_rows:
        if len(scenarios) >= 12:
            break
        if r["battle_id"] in seen_battles:
            continue
        seen_battles.add(r["battle_id"])
        battle = next((b for b in pool if b.battle_id == r["battle_id"]), None)
        if battle is None:
            continue
        threat = _last_opponent_threat(battle, r["event_index"])
        if threat is None:
            continue
        hand_slots = [i for i, v in enumerate(r["hand"]) if v]
        threat_xy = _normalize_xy(
            int(threat["x"]),
            int(threat["y"]),
            swap_sides=(battle.events[r["event_index"]]["side"] == "opponent"),
        )
        scenarios.append(
            {
                "battle_id": r["battle_id"],
                "seconds": round(r["seconds"], 1),
                "threat": threat["card"],
                "threat_label": pretty_card(threat["card"]),
                "threat_delay": round(r["seconds"] - float(threat["seconds"]), 1),
                "threat_xy": [round(v, 4) for v in threat_xy],
                "human_xy": [round(v, 4) for v in r["true_xy"]],
                "new_xy": [round(v, 4) for v in r["new_xy"]],
                "hand": [
                    {
                        "slot": s,
                        "card": r["deck"][s],
                        "label": pretty_card(r["deck"][s]),
                        "cost": int(costs.get(r["deck"][s], DEFAULT_ELIXIR_COST)),
                    }
                    for s in hand_slots
                ],
                "human_slot": r["slot"],
                "human_card": r["card"],
                "new_slot": r["new_slot"],
                "old_slot": r["old_slot"],
                "new_slot_p": round(r["new_slot_p"], 4),
                "human_zone": r["true_zone"],
                "new_zone": r["new_zone"],
            }
        )

    # ---- reaction vs non-reaction split ------------------------------------
    react = [r for r in scored if r["is_reaction"]]
    calm = [r for r in scored if not r["is_reaction"]]

    def _split_stats(rows: list[dict[str, Any]]) -> dict[str, Any]:
        n = max(len(rows), 1)
        return {
            "n": len(rows),
            "new_slot_acc": sum(1 for r in rows if r["new_slot"] == r["slot"]) / n,
            "old_slot_acc": sum(1 for r in rows if r["old_slot"] == r["slot"]) / n,
            "new_zone_acc": sum(1 for r in rows if r["new_zone"] == r["true_zone"]) / n,
            "old_zone_acc": sum(1 for r in rows if r["old_zone"] == r["true_zone"]) / n,
        }

    report = {
        "created_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "new_model": {
            "dir": str(new_policy_dir),
            "version": str(new_cfg.get("version", "?")),
            "card_conditioned_placement": bool(
                new_cfg.get("card_conditioned_placement", False)
            ),
        },
        "old_model": {
            "dir": str(old_policy_dir),
            "version": str(old_cfg.get("version", "?")),
            "card_conditioned_placement": bool(
                old_cfg.get("card_conditioned_placement", False)
            ),
        },
        "compute": {
            "device": str(device),
            "battles_scored": len(pool),
            "actions_scored": len(scored),
            "seconds": round(time.time() - started, 1),
        },
        "zone_names": list(ZONE_NAMES),
        "overall": overall,
        "cards": cards_out,
        "fix_gallery": gallery,
        "fix_counts": {
            "fixed": len(fixes),
            "regressed": len(regressions),
            "net": len(fixes) - len(regressions),
        },
        "scenarios": scenarios,
        "splits": {
            "reaction": _split_stats(react),
            "non_reaction": _split_stats(calm),
        },
    }

    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)
    with out.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    print(
        json.dumps(
            {
                "actions": len(scored),
                "zone_acc": {
                    "old": round(overall["old_zone_acc"], 4),
                    "new": round(overall["new_zone_acc"], 4),
                },
                "fixed": len(fixes),
                "regressed": len(regressions),
                "scenarios": len(scenarios),
            },
            indent=2,
        )
    )
    print(f"Wrote {out}")
    return report


def _entropy(grid: list[float]) -> float:
    total = sum(grid)
    if total <= 0:
        return 0.0
    acc = 0.0
    for p in grid:
        if p > 0:
            acc -= p * float(np.log2(p))
    return acc


def _last_opponent_threat(
    battle: BattleExample, event_index: int, max_age: float = 5.0
) -> dict[str, Any] | None:
    target = battle.events[event_index]
    now = float(target["seconds"])
    opponent = "opponent" if target["side"] == "team" else "team"
    best = None
    for event in battle.events[:event_index]:
        if event["side"] != opponent or event["event_type"] != "card_play":
            continue
        age = now - float(event["seconds"])
        if age < 0 or age > max_age:
            continue
        if event["card"] in WIN_CONDITIONISH:
            if best is None or float(event["seconds"]) >= float(best["seconds"]):
                best = event
    return best
