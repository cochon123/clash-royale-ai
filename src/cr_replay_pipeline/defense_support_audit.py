"""Data-support + natural-hand counterfactual audit for defense probe cells."""

from __future__ import annotations

import json
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import numpy as np
import torch

from .defense_slice_eval import FOCUS_THREATS, mine_reaction_windows
from .policy_dataset import acting_cycle_features, deck_slot_for_card, encode_policy_sample
from .policy_infer import load_policy
from .winner_dataset import (
    DEFAULT_ELIXIR_COST,
    BattleExample,
    collect_battles,
    load_card_costs,
    split_battles,
)

# Probe cells to audit (failing + passing controls).
AUDIT_CELLS: tuple[tuple[str, str, str], ...] = (
    # (threat, answer, role)
    ("graveyard", "poison", "failing"),
    ("graveyard", "valkyrie", "failing"),
    ("hog-rider", "tornado", "failing"),
    ("hog-rider", "cannon", "control"),
    ("hog-rider", "tesla", "control"),
    ("balloon", "musketeer", "control"),
    ("balloon", "executioner", "failing"),
    ("golem", "inferno-tower", "failing"),
    ("golem", "inferno-dragon", "control"),
    ("golem", "pekka", "control"),
)

CHEAP_ALTS = frozenset(
    {
        "ice-spirit",
        "skeletons",
        "zap",
        "the-log",
        "arrows",
        "goblins",
        "spear-goblins",
        "bats",
        "electro-spirit",
        "fire-spirit",
    }
)


def _defender_deck(battle: BattleExample, defender: str) -> tuple[str, ...]:
    return battle.team_deck if defender == "team" else battle.opponent_deck


def _answer_in_hand(
    battle: BattleExample,
    response_index: int,
    defender: str,
    answer: str,
    costs: dict[str, int],
) -> bool:
    swap = defender == "opponent"
    _feats, hand_mask = acting_cycle_features(battle, response_index, costs, swap)
    if not bool(hand_mask.any()):
        # Early game: heuristic off — treat "in deck" only via caller filters.
        return False
    deck = _defender_deck(battle, defender)
    slot = deck_slot_for_card(deck, answer)
    if slot is None:
        return False
    return bool(hand_mask[slot])


@torch.no_grad()
def _score_window(
    model,
    vocab,
    costs: dict[str, int],
    window: dict[str, Any],
    answer: str,
    max_context: int,
    device: torch.device,
    threat_dim: int = 0,
) -> dict[str, Any] | None:
    battle: BattleExample = window["battle"]
    j = int(window["response_index"])
    sample = encode_policy_sample(
        battle, j, vocab, costs, max_context=max_context, threat_dim=threat_dim
    )
    if sample is None:
        return None
    (
        continuous,
        card_ids,
        team_deck_t,
        opp_deck_t,
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
        team_deck_t.unsqueeze(0).to(device),
        opp_deck_t.unsqueeze(0).to(device),
        global_feat.unsqueeze(0).to(device),
        length.unsqueeze(0).to(device),
        slot_feats.unsqueeze(0).to(device),
        hand_mask.unsqueeze(0).to(device),
    )
    probs = torch.softmax(out["slot_logits"][0], dim=-1).cpu().numpy()
    deck = _defender_deck(battle, window["defender"])
    answer_slot = deck_slot_for_card(deck, answer)
    if answer_slot is None:
        return None
    pred = int(probs.argmax())
    top3 = set(int(x) for x in probs.argsort()[-3:][::-1])
    # Best cheap alternative probability among deck cards that are cheap.
    cheap_ps = [
        float(probs[i])
        for i, card in enumerate(deck)
        if card in CHEAP_ALTS or float(costs.get(card, 4)) <= 2.0
    ]
    best_cheap_p = max(cheap_ps) if cheap_ps else 0.0
    p_answer = float(probs[answer_slot])
    return {
        "top1_answer": pred == answer_slot,
        "top3_answer": answer_slot in top3,
        "P_answer": p_answer,
        "P_margin_vs_cheap": p_answer - best_cheap_p,
        "pred_card": deck[pred],
        "human_played_answer": window["response"] == answer,
        "human_played_cheap": window["response"] in CHEAP_ALTS
        or float(costs.get(window["response"], 4)) <= 2.0,
    }


def _decide_cell(row: dict[str, Any]) -> dict[str, Any]:
    """Apply Fable/Sol decision rules."""
    n_deck = int(row["n_threat_answer_in_deck"])
    n_hand = int(row["n_threat_answer_in_hand"])
    human_rate_deck = float(row["human_use_rate_given_in_deck"])
    human_rate_hand = float(row["human_use_rate_given_in_hand"])
    # Prefer hand-conditioned numbers when available.
    n = n_hand if n_hand >= 20 else n_deck
    human_rate = human_rate_hand if n_hand >= 20 else human_rate_deck
    model_top1_when_human = row.get("model_top1_when_human_chose_answer")
    model_top1_in_hand = row.get("model_top1_answer_given_in_hand")

    unsupported = n < 200 or human_rate < 0.15
    if unsupported:
        status = "unsupported"
        action = (
            "Drop from success gates / collect more matching data. "
            "Do not retrain architecture for this cell."
        )
    else:
        # Supported: check model failure on natural cases.
        fail_metric = (
            model_top1_when_human
            if model_top1_when_human is not None
            else model_top1_in_hand
        )
        if fail_metric is not None and fail_metric < 0.35:
            status = "supported_but_model_fails"
            action = (
                "Justify policy v3 (threat conditioning + reaction upweight) "
                "for this cell."
            )
        else:
            status = "supported_and_ok"
            action = "No architecture change needed for this cell."

    # Sol criteria when humans chose the answer
    sol = {
        "n_human_chose_answer_in_hand": row.get("n_human_chose_answer_in_hand", 0),
        "model_top1_when_human_chose": model_top1_when_human,
        "model_beats_cheap_rate_when_human_chose": row.get(
            "model_beats_cheap_when_human_chose"
        ),
        "sol_pass": (
            (row.get("n_human_chose_answer_in_hand") or 0) >= 50
            and (model_top1_when_human or 0) >= 0.50
            and (row.get("model_beats_cheap_when_human_chose") or 0) >= 0.70
        ),
    }
    return {
        "status": status,
        "action": action,
        "support_n_used": n,
        "support_human_rate_used": human_rate,
        "sol_natural_counterfactual": sol,
    }


def audit_defense_support(
    input_dir: str | Path = "data/raw",
    policy_dir: str | Path = "models/policy_bc",
    card_costs_path: str | Path = "data/card_costs.json",
    output_path: str | Path = "reports/defense_support_audit.json",
    max_response_seconds: float = 5.0,
    seed: int = 42,
    device_name: str | None = None,
    score_model: bool = True,
) -> dict[str, Any]:
    costs = load_card_costs(card_costs_path)
    battles = collect_battles(input_dir)
    train, _val, test = split_battles(battles, seed=seed)

    print("Mining train + test reaction windows ...", flush=True)
    train_windows = mine_reaction_windows(
        train, max_response_seconds=max_response_seconds, focus_threats=FOCUS_THREATS
    )
    test_windows = mine_reaction_windows(
        test, max_response_seconds=max_response_seconds, focus_threats=FOCUS_THREATS
    )
    print(
        f"Windows: train={len(train_windows)} test={len(test_windows)}",
        flush=True,
    )

    model = vocab = cfg = device = None
    max_context = 64
    threat_dim = 0
    model_name = "policy-bc"
    if score_model:
        model, vocab, cfg, device = load_policy(policy_dir, device_name=device_name)
        max_context = int(cfg.get("max_context", 64))
        threat_dim = int(cfg.get("threat_dim", 0))
        ver = str(cfg.get("version", ""))
        if ver.startswith("4"):
            model_name = "policy-bc-v4"
        elif ver.startswith("3"):
            model_name = "policy-bc-v3"
        elif ver.startswith("2"):
            model_name = "policy-bc-v2"
        else:
            model_name = Path(policy_dir).name

    cell_rows: list[dict[str, Any]] = []
    for threat, answer, role in AUDIT_CELLS:
        print(f"Auditing {threat} → {answer} ({role}) ...", flush=True)
        # Coverage primarily from TRAIN (what BC can learn).
        train_threat = [w for w in train_windows if w["threat"] == threat]
        in_deck = [
            w
            for w in train_threat
            if answer in _defender_deck(w["battle"], w["defender"])
        ]
        in_hand = [
            w
            for w in in_deck
            if _answer_in_hand(
                w["battle"], w["response_index"], w["defender"], answer, costs
            )
        ]
        human_in_deck = [w for w in in_deck if w["response"] == answer]
        human_in_hand = [w for w in in_hand if w["response"] == answer]

        # Model scoring on TEST natural-hand subset (honest).
        test_threat = [w for w in test_windows if w["threat"] == threat]
        test_in_deck = [
            w
            for w in test_threat
            if answer in _defender_deck(w["battle"], w["defender"])
        ]
        test_in_hand = [
            w
            for w in test_in_deck
            if _answer_in_hand(
                w["battle"], w["response_index"], w["defender"], answer, costs
            )
        ]
        test_human_answer = [w for w in test_in_hand if w["response"] == answer]

        scored_in_hand: list[dict[str, Any]] = []
        scored_when_human: list[dict[str, Any]] = []
        if score_model and model is not None:
            for w in test_in_hand:
                s = _score_window(
                    model,
                    vocab,
                    costs,
                    w,
                    answer,
                    max_context,
                    device,
                    threat_dim=threat_dim,
                )
                if s is None:
                    continue
                scored_in_hand.append(s)
                if s["human_played_answer"]:
                    scored_when_human.append(s)

        def _mean(rows: list[dict[str, Any]], key: str) -> float | None:
            if not rows:
                return None
            return float(np.mean([r[key] for r in rows]))

        row: dict[str, Any] = {
            "threat": threat,
            "answer": answer,
            "role": role,
            "train": {
                "n_threat_windows": len(train_threat),
                "n_threat_answer_in_deck": len(in_deck),
                "n_threat_answer_in_hand": len(in_hand),
                "n_human_chose_answer_in_deck": len(human_in_deck),
                "n_human_chose_answer_in_hand": len(human_in_hand),
                "human_use_rate_given_in_deck": (
                    len(human_in_deck) / len(in_deck) if in_deck else 0.0
                ),
                "human_use_rate_given_in_hand": (
                    len(human_in_hand) / len(in_hand) if in_hand else 0.0
                ),
            },
            "test": {
                "n_threat_windows": len(test_threat),
                "n_threat_answer_in_deck": len(test_in_deck),
                "n_threat_answer_in_hand": len(test_in_hand),
                "n_human_chose_answer_in_hand": len(test_human_answer),
            },
            # Flatten keys used by decision helper
            "n_threat_answer_in_deck": len(in_deck),
            "n_threat_answer_in_hand": len(in_hand),
            "human_use_rate_given_in_deck": (
                len(human_in_deck) / len(in_deck) if in_deck else 0.0
            ),
            "human_use_rate_given_in_hand": (
                len(human_in_hand) / len(in_hand) if in_hand else 0.0
            ),
            "n_human_chose_answer_in_hand": len(test_human_answer),
            "model_top1_answer_given_in_hand": _mean(scored_in_hand, "top1_answer"),
            "model_top3_answer_given_in_hand": _mean(scored_in_hand, "top3_answer"),
            "model_mean_P_answer_given_in_hand": _mean(scored_in_hand, "P_answer"),
            "model_top1_when_human_chose_answer": _mean(
                scored_when_human, "top1_answer"
            ),
            "model_beats_cheap_when_human_chose": (
                float(np.mean([r["P_margin_vs_cheap"] > 0 for r in scored_when_human]))
                if scored_when_human
                else None
            ),
            "model_mean_margin_vs_cheap_when_human_chose": _mean(
                scored_when_human, "P_margin_vs_cheap"
            ),
            "n_scored_in_hand_test": len(scored_in_hand),
            "n_scored_when_human_chose_test": len(scored_when_human),
        }
        row["decision"] = _decide_cell(row)
        cell_rows.append(row)
        d = row["decision"]
        print(
            f"  train in_deck={len(in_deck)} in_hand={len(in_hand)} "
            f"human_hand_rate={row['human_use_rate_given_in_hand']:.1%} "
            f"→ {d['status']}",
            flush=True,
        )

    status_counts = defaultdict(int)
    for row in cell_rows:
        status_counts[row["decision"]["status"]] += 1

    failing = [r for r in cell_rows if r["role"] == "failing"]
    controls = [r for r in cell_rows if r["role"] == "control"]
    any_v3 = any(
        r["decision"]["status"] == "supported_but_model_fails" for r in failing
    )
    all_failing_unsupported = all(
        r["decision"]["status"] == "unsupported" for r in failing
    )

    if any_v3:
        verdict = (
            "At least one failing probe cell is data-supported but the model "
            "still misses it — threat conditioning / reaction upweight (or a "
            "stronger variant) is justified for those cells."
        )
        next_step = "improve_threat_conditioning"
    elif all_failing_unsupported:
        verdict = (
            "Failing probe cells lack training support (rare human answers). "
            "Retire them from success gates; do not retrain for unlearnable targets. "
            "Bias collection if you care about those matchups."
        )
        next_step = "drop_unsupported_gates_continue_offline"
    else:
        verdict = (
            "No supported-but-failing cells. Unsupported cells stay out of gates; "
            "keep iterating offline."
        )
        next_step = "continue_offline"

    report = {
        "created_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "model_name": model_name,
        "setup": {
            "coverage_split": "train",
            "model_score_split": "test",
            "max_response_seconds": max_response_seconds,
            "hand_heuristic": "wait-time oldest-4 after ≥4 defender plays",
            "thresholds": {
                "unsupported_if_n_lt": 200,
                "unsupported_if_human_rate_lt": 0.15,
                "model_fail_if_top1_when_human_lt": 0.35,
                "sol_min_examples": 50,
                "sol_top1": 0.50,
                "sol_beats_cheap": 0.70,
            },
        },
        "summary": {
            "cells": len(cell_rows),
            "status_counts": dict(status_counts),
            "failing_cells": len(failing),
            "control_cells": len(controls),
            "recommend_v3": any_v3,
            "next_step": next_step,
        },
        "cells": cell_rows,
        "verdict": verdict,
        "lessons": [
            "Real-slice top-1 measures in-distribution imitation; probe cells need data support to be fair success gates.",
            "Poison-vs-graveyard is often a mirror/rare defender holding — check n before blaming the model.",
            "Score model on test natural-hand subsets; measure coverage on train.",
            "Only train v3 when a failing cell is supported and still missed.",
        ],
    }

    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)
    with out.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    print(json.dumps({"summary": report["summary"], "verdict": verdict}, indent=2))
    print(f"Wrote {out}")
    return report
