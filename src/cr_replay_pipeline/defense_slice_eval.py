"""Real-replay defense-slice eval: human reaction windows, no forced hand."""

from __future__ import annotations

import json
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import numpy as np
import torch

from .policy_dataset import DEFAULT_MIN_CONTEXT, deck_slot_for_card, encode_policy_sample
from .policy_infer import load_policy
from .winner_dataset import (
    DEFAULT_ELIXIR_COST,
    WIN_CONDITIONISH,
    BattleExample,
    collect_battles,
    load_card_costs,
    split_battles,
)

FOCUS_THREATS = ("hog-rider", "balloon", "graveyard", "golem")


def mine_reaction_windows(
    battles: list[BattleExample],
    max_response_seconds: float = 5.0,
    focus_threats: tuple[str, ...] | None = FOCUS_THREATS,
    min_context: int = DEFAULT_MIN_CONTEXT,
) -> list[dict[str, Any]]:
    """Find (battle, threat_index, response_index) on held-out data."""
    windows: list[dict[str, Any]] = []
    allow = set(focus_threats) if focus_threats else set(WIN_CONDITIONISH)

    for battle in battles:
        events = battle.events
        for i, threat in enumerate(events):
            if threat["event_type"] != "card_play":
                continue
            if threat["card"] not in allow:
                continue
            attacker = threat["side"]
            defender = "opponent" if attacker == "team" else "team"
            t0 = float(threat["seconds"])
            # Next play by the defender within the response window.
            for j in range(i + 1, len(events)):
                resp = events[j]
                dt = float(resp["seconds"]) - t0
                if dt > max_response_seconds:
                    break
                if resp["side"] != defender:
                    continue
                if resp["event_type"] != "card_play":
                    continue
                if j < min_context:
                    continue
                # Predict at j from prefix [:j], target is human response.
                acting_deck = (
                    battle.team_deck if defender == "team" else battle.opponent_deck
                )
                slot = deck_slot_for_card(acting_deck, resp["card"])
                if slot is None:
                    break
                windows.append(
                    {
                        "battle": battle,
                        "threat_index": i,
                        "response_index": j,
                        "threat": threat["card"],
                        "response": resp["card"],
                        "defender": defender,
                        "delay": dt,
                        "response_cost": float(
                            # filled later if needed
                            0.0
                        ),
                        "target_slot": slot,
                    }
                )
                break
    return windows


@torch.no_grad()
def evaluate_defense_slice(
    input_dir: str | Path = "data/raw",
    policy_dir: str | Path = "models/policy_bc",
    card_costs_path: str | Path = "data/card_costs.json",
    output_path: str | Path = "reports/defense_slice_eval.json",
    max_response_seconds: float = 5.0,
    seed: int = 42,
    device_name: str | None = None,
    max_windows: int | None = None,
) -> dict[str, Any]:
    costs = load_card_costs(card_costs_path)
    battles = collect_battles(input_dir)
    _train, _val, test = split_battles(battles, seed=seed)
    print(f"Mining reaction windows from {len(test)} test battles ...", flush=True)
    windows = mine_reaction_windows(test, max_response_seconds=max_response_seconds)
    for window in windows:
        window["response_cost"] = float(
            costs.get(window["response"], DEFAULT_ELIXIR_COST)
        )
    if max_windows is not None and len(windows) > max_windows:
        # Deterministic subsample for speed.
        rng = np.random.default_rng(seed)
        pick = rng.choice(len(windows), size=max_windows, replace=False)
        windows = [windows[int(i)] for i in sorted(pick)]

    print(f"Found {len(windows)} reaction windows; loading policy ...", flush=True)
    model, vocab, cfg, device = load_policy(policy_dir, device_name=device_name)
    max_context = int(cfg.get("max_context", 64))
    threat_dim = int(cfg.get("threat_dim", 0))

    # Also a naive baseline: most frequent human response card for that threat
    # estimated on the same test windows (leaky but informative as upper sanity).
    # Online baseline: within-deck frequency before the response (no future).
    results: list[dict[str, Any]] = []
    by_threat: dict[str, list[dict[str, Any]]] = defaultdict(list)

    for index, window in enumerate(windows):
        battle: BattleExample = window["battle"]
        j = int(window["response_index"])
        sample = encode_policy_sample(
            battle,
            j,
            vocab,
            costs,
            max_context=max_context,
            threat_dim=threat_dim,
        )
        if sample is None:
            continue
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
        logits = out["slot_logits"][0]
        probs = torch.softmax(logits, dim=-1).cpu().numpy()
        target = int(window["target_slot"])
        pred = int(probs.argmax())
        top3 = set(int(x) for x in probs.argsort()[-3:][::-1])
        defender = window["defender"]
        acting_deck = battle.team_deck if defender == "team" else battle.opponent_deck
        pred_card = acting_deck[pred]
        # Online frequency baseline among deck cards from prior defender plays.
        counts = {card: 0 for card in acting_deck}
        for event in battle.events[:j]:
            if event["side"] == defender and event["event_type"] == "card_play":
                if event["card"] in counts:
                    counts[event["card"]] += 1
        ranked = sorted(counts.keys(), key=lambda c: (-counts[c], acting_deck.index(c)))
        freq_card = ranked[0]
        freq_ok = freq_card == window["response"]
        freq_top3 = window["response"] in ranked[:3]

        row = {
            "threat": window["threat"],
            "response": window["response"],
            "pred_card": pred_card,
            "delay": window["delay"],
            "response_cost": window["response_cost"],
            "pred_cost": float(costs.get(pred_card, DEFAULT_ELIXIR_COST)),
            "top1": pred == target,
            "top3": target in top3,
            "P_target": float(probs[target]),
            "freq_top1": freq_ok,
            "freq_top3": freq_top3,
            "cheap_response": window["response_cost"] <= 2.0 + 1e-9,
            "cheap_pred": float(costs.get(pred_card, DEFAULT_ELIXIR_COST)) <= 2.0 + 1e-9,
        }
        results.append(row)
        by_threat[window["threat"]].append(row)
        if (index + 1) % 500 == 0:
            print(f"  scored {index + 1}/{len(windows)}", flush=True)

    def _agg(rows: list[dict[str, Any]]) -> dict[str, Any]:
        if not rows:
            return {"n": 0}
        return {
            "n": len(rows),
            "top1": float(np.mean([r["top1"] for r in rows])),
            "top3": float(np.mean([r["top3"] for r in rows])),
            "mean_P_target": float(np.mean([r["P_target"] for r in rows])),
            "freq_top1": float(np.mean([r["freq_top1"] for r in rows])),
            "freq_top3": float(np.mean([r["freq_top3"] for r in rows])),
            "mean_response_cost": float(np.mean([r["response_cost"] for r in rows])),
            "mean_pred_cost": float(np.mean([r["pred_cost"] for r in rows])),
            "cheap_pred_rate": float(np.mean([r["cheap_pred"] for r in rows])),
            "cheap_response_rate": float(np.mean([r["cheap_response"] for r in rows])),
            "mean_delay": float(np.mean([r["delay"] for r in rows])),
        }

    threat_rows = [
        {"threat": threat, **_agg(rows)}
        for threat, rows in sorted(by_threat.items(), key=lambda kv: -len(kv[1]))
    ]
    # Cost strata of human responses
    expensive = [r for r in results if r["response_cost"] >= 4.0]
    cheap = [r for r in results if r["response_cost"] <= 2.0]

    overall = _agg(results)
    # Success bar from the Sol/Fable plan: beat frequency baseline clearly.
    beats_freq = overall.get("top1", 0) >= overall.get("freq_top1", 0) + 0.05
    near_global = overall.get("top1", 0) >= 0.40  # vs ~51% global slot@1
    report = {
        "created_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "model_name": "policy-bc-v2",
        "setup": {
            "split": "test battles, seed=42",
            "max_response_seconds": max_response_seconds,
            "focus_threats": list(FOCUS_THREATS),
            "forced_hand": False,
            "note": (
                "Prefix is the real replay up to the human response. "
                "No synthetic warmup, no forced hand mask."
            ),
        },
        "overall": overall,
        "by_threat": threat_rows,
        "by_response_cost": {
            "cheap_le_2": _agg(cheap),
            "expensive_ge_4": _agg(expensive),
        },
        "gates": {
            "beats_online_frequency_by_5pp": beats_freq,
            "top1_at_least_40pct": near_global,
            "pass": bool(beats_freq and near_global),
        },
        "verdict": (
            "Defense-on-real-data looks usable; synthetic probe was likely overstated."
            if beats_freq and near_global
            else (
                "Defense is weak on real reaction windows too — proceed to policy v3 "
                "(threat conditioning + reaction upweight)."
                if results
                else "No reaction windows found."
            )
        ),
    }
    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)
    with out.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    print(json.dumps({"overall": overall, "by_threat": threat_rows, "gates": report["gates"], "verdict": report["verdict"]}, indent=2))
    print(f"Wrote {out}")
    return report
