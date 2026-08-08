"""Controlled defense probes: threat at bridge/tower, 3 weak answers + 1 strong."""

from __future__ import annotations

import json
import random
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import numpy as np
import torch

from .policy_dataset import encode_policy_sample
from .policy_infer import load_policy
from .winner_dataset import BattleExample, load_card_costs


@dataclass(frozen=True)
class DefenseScenario:
    name: str
    threat: str
    strong: str
    weak: tuple[str, str, str]
    # Opponent placement for the threat (RoyaleAPI units).
    threat_x: int
    threat_y: int
    note: str


SCENARIOS: tuple[DefenseScenario, ...] = (
    # Hog rider at the bridge — building / pull answers.
    DefenseScenario(
        "hog-vs-cannon",
        "hog-rider",
        "cannon",
        ("arrows", "zap", "ice-spirit"),
        5000,
        21000,
        "Hog left bridge; cannon is the real answer among chip spells/spirit.",
    ),
    DefenseScenario(
        "hog-vs-tesla",
        "hog-rider",
        "tesla",
        ("the-log", "goblins", "arrows"),
        13000,
        21500,
        "Hog right bridge; tesla vs log/goblins/arrows.",
    ),
    DefenseScenario(
        "hog-vs-tornado",
        "hog-rider",
        "tornado",
        ("zap", "skeletons", "ice-spirit"),
        9000,
        20500,
        "Hog mid-bridge; tornado pull vs useless chip.",
    ),
    # Balloon.
    DefenseScenario(
        "balloon-vs-musketeer",
        "balloon",
        "musketeer",
        ("skeletons", "ice-spirit", "the-log"),
        5000,
        20000,
        "Balloon left; musketeer is the air answer.",
    ),
    DefenseScenario(
        "balloon-vs-executioner",
        "balloon",
        "executioner",
        ("goblins", "zap", "arrows"),
        13000,
        20500,
        "Balloon right; executioner vs ground chip.",
    ),
    # Graveyard on our tower.
    DefenseScenario(
        "graveyard-vs-poison",
        "graveyard",
        "poison",
        ("zap", "ice-spirit", "the-log"),
        5000,
        9000,
        "GY on left tower; poison is the classic spell answer.",
    ),
    DefenseScenario(
        "graveyard-vs-valkyrie",
        "graveyard",
        "valkyrie",
        ("skeletons", "arrows", "zap"),
        13000,
        9500,
        "GY on right tower; valkyrie splash vs weak swarm/spells.",
    ),
    # Golem (played in the back — defend the push).
    DefenseScenario(
        "golem-vs-inferno-tower",
        "golem",
        "inferno-tower",
        ("skeletons", "ice-spirit", "archers"),
        9000,
        29000,
        "Golem in back; inferno tower is the tank shredder.",
    ),
    DefenseScenario(
        "golem-vs-inferno-dragon",
        "golem",
        "inferno-dragon",
        ("goblins", "zap", "spear-goblins"),
        7000,
        28500,
        "Golem back-left; inferno dragon vs weak ground chips.",
    ),
    DefenseScenario(
        "golem-vs-pekka",
        "golem",
        "pekka",
        ("skeletons", "arrows", "ice-spirit"),
        11000,
        29500,
        "Golem back-right; PEKKA vs cards that don't stop a golem.",
    ),
)


def _filler_cards(exclude: set[str]) -> list[str]:
    pool = [
        "knight",
        "archers",
        "fireball",
        "cannon",
        "musketeer",
        "mini-pekka",
        "baby-dragon",
        "electro-spirit",
        "royal-delivery",
        "bomber",
    ]
    out = [card for card in pool if card not in exclude]
    return out


def _build_warmup(
    team_fillers: list[str],
    opponent_support: list[str],
    rng: random.Random,
    n_events: int = 12,
) -> list[dict[str, Any]]:
    """Neutral early cycle using ONLY filler/support cards.

    Hand cards (strong+weak) are never played, so the wait-time heuristic
    marks them as the 4-card hand — consistent with cycle features.
    """
    events: list[dict[str, Any]] = []
    seconds = 6.0
    team_cycle = list(team_fillers) or ["knight"]
    opp_cycle = list(opponent_support) or ["archers"]
    ti = oi = 0
    side = "team"
    while len(events) < n_events:
        seconds += rng.uniform(1.4, 2.6)
        if side == "team":
            card = team_cycle[ti % len(team_cycle)]
            ti += 1
            y = rng.randint(2500, 12000)
        else:
            card = opp_cycle[oi % len(opp_cycle)]
            oi += 1
            y = rng.randint(20000, 30000)
        events.append(
            {
                "seconds": seconds,
                "side": side,
                "event_type": "card_play",
                "card": card,
                "x": rng.randint(4000, 14000),
                "y": y,
            }
        )
        side = "opponent" if side == "team" else "team"
    return events


def build_defense_battle(
    scenario: DefenseScenario,
    costs: dict[str, int],
    rng: random.Random,
) -> tuple[BattleExample, int, torch.Tensor]:
    """Return battle prefix ending after the threat, strong slot, cycle-consistent hand mask."""
    weak = list(scenario.weak)
    strong = scenario.strong
    exclude = {strong, *weak, scenario.threat}
    fillers = _filler_cards(exclude)
    rng.shuffle(fillers)
    # Team deck: 4 hand cards + 4 fillers. Warmup only cycles fillers.
    hand = [strong, *weak]
    rng.shuffle(hand)
    team_fillers = fillers[:4]
    team_deck = tuple(hand + team_fillers)
    opp_support = _filler_cards({scenario.threat, *team_deck})
    rng.shuffle(opp_support)
    opponent_deck = tuple([scenario.threat, *opp_support[:7]])

    events = _build_warmup(team_fillers, opp_support[:7], rng, n_events=12)
    last_seconds = float(events[-1]["seconds"])
    events.append(
        {
            "seconds": last_seconds + rng.uniform(1.0, 2.0),
            "side": "opponent",
            "event_type": "card_play",
            "card": scenario.threat,
            "x": scenario.threat_x,
            "y": scenario.threat_y,
        }
    )

    strong_slot = team_deck.index(strong)
    # Never-played hand cards are oldest → heuristic hand.
    hand_mask = torch.zeros(8, dtype=torch.bool)
    for card in hand:
        hand_mask[team_deck.index(card)] = True

    battle = BattleExample(
        battle_id=f"defense-{scenario.name}",
        team_deck=team_deck,
        opponent_deck=opponent_deck,
        team_wins=0,
        events=tuple(events),
    )
    return battle, strong_slot, hand_mask


@torch.no_grad()
def probe_defense(
    model,
    vocab,
    costs: dict[str, int],
    scenario: DefenseScenario,
    device: torch.device,
    rng: random.Random,
    max_context: int = 64,
    force_hand: bool = True,
    threat_dim: int = 0,
) -> dict[str, Any]:
    battle, strong_slot, hand_mask = build_defense_battle(scenario, costs, rng)
    # Dummy next action for team so encode remaps correctly.
    dummy = {
        "seconds": float(battle.events[-1]["seconds"]) + 1.0,
        "side": "team",
        "event_type": "card_play",
        "card": battle.team_deck[0],
        "x": 9000,
        "y": 8000,
    }
    probe = BattleExample(
        battle_id=battle.battle_id,
        team_deck=battle.team_deck,
        opponent_deck=battle.opponent_deck,
        team_wins=0,
        events=tuple(battle.events) + (dummy,),
    )
    sample = encode_policy_sample(
        probe,
        len(battle.events),
        vocab,
        costs,
        max_context=max_context,
        threat_dim=threat_dim,
    )
    if sample is None:
        return {"ok": False, "error": "encode_failed", "scenario": scenario.name}

    (
        continuous,
        card_ids,
        team_deck_t,
        opp_deck_t,
        global_feat,
        slot_feats,
        auto_hand,
        _slot,
        _type,
        _zone,
        _xy,
        _timing,
        length,
    ) = sample

    # Prefer auto mask when it already matches the intended hand (fair probe).
    if force_hand and bool((auto_hand == hand_mask).all()):
        used_mask = auto_hand
    elif force_hand:
        used_mask = hand_mask
    else:
        used_mask = auto_hand
    out = model(
        continuous.unsqueeze(0).to(device),
        card_ids.unsqueeze(0).to(device),
        team_deck_t.unsqueeze(0).to(device),
        opp_deck_t.unsqueeze(0).to(device),
        global_feat.unsqueeze(0).to(device),
        length.unsqueeze(0).to(device),
        slot_feats.unsqueeze(0).to(device),
        used_mask.unsqueeze(0).to(device),
    )
    probs = torch.softmax(out["slot_logits"][0], dim=-1).cpu().numpy()
    # Restrict analysis to the 4 hand cards.
    hand_slots = [i for i in range(8) if bool(used_mask[i])]
    hand_probs = {i: float(probs[i]) for i in hand_slots}
    # Renormalize over hand for a fair 4-way choice metric.
    mass = sum(hand_probs.values()) + 1e-12
    hand_probs_norm = {i: v / mass for i, v in hand_probs.items()}
    pick = max(hand_slots, key=lambda i: hand_probs[i])
    pick_card = battle.team_deck[pick]
    strong_p = float(probs[strong_slot])
    strong_p_hand = float(hand_probs_norm[strong_slot])
    chance = 1.0 / max(len(hand_slots), 1)

    xy = out["xy"][0].cpu().numpy()
    return {
        "ok": True,
        "scenario": scenario.name,
        "threat": scenario.threat,
        "strong": scenario.strong,
        "weak": list(scenario.weak),
        "team_deck": list(battle.team_deck),
        "strong_slot": strong_slot,
        "picked_slot": pick,
        "picked_card": pick_card,
        "correct": pick == strong_slot,
        "P_strong": strong_p,
        "P_strong_among_hand": strong_p_hand,
        "chance_among_hand": chance,
        "hand_probs": {
            battle.team_deck[i]: hand_probs_norm[i] for i in hand_slots
        },
        "predicted_xy": {
            "x": int(np.clip(xy[0] * 18000.0, 3000, 15000)),
            "y": int(np.clip(xy[1] * 32000.0, 500, 16000)),
        },
        "force_hand": force_hand,
        "note": scenario.note,
    }


def evaluate_defense(
    policy_dir: str | Path = "models/policy_bc",
    card_costs_path: str | Path = "data/card_costs.json",
    output_path: str | Path = "reports/defense_eval.json",
    trials_per_scenario: int = 64,
    seed: int = 42,
    device_name: str | None = None,
) -> dict[str, Any]:
    costs = load_card_costs(card_costs_path)
    model, vocab, cfg, device = load_policy(policy_dir, device_name=device_name)
    rng = random.Random(seed)
    max_context = int(cfg.get("max_context", 64))
    threat_dim = int(cfg.get("threat_dim", 0))

    print(
        f"Defense probes: {len(SCENARIOS)} scenarios × {trials_per_scenario} trials on {device}",
        flush=True,
    )
    scenario_rows: list[dict[str, Any]] = []
    for scenario in SCENARIOS:
        trials = []
        for _ in range(trials_per_scenario):
            trials.append(
                probe_defense(
                    model,
                    vocab,
                    costs,
                    scenario,
                    device,
                    random.Random(rng.randint(0, 10**9)),
                    max_context=max_context,
                    force_hand=True,
                    threat_dim=threat_dim,
                )
            )
        ok = [t for t in trials if t.get("ok")]
        if not ok:
            scenario_rows.append({"scenario": scenario.name, "error": "all_failed"})
            continue
        correct = float(np.mean([t["correct"] for t in ok]))
        mean_p = float(np.mean([t["P_strong_among_hand"] for t in ok]))
        chance = float(np.mean([t["chance_among_hand"] for t in ok]))
        # What it picked instead when wrong
        wrong_picks = Counter_like([t["picked_card"] for t in ok if not t["correct"]])
        row = {
            "scenario": scenario.name,
            "threat": scenario.threat,
            "strong": scenario.strong,
            "weak": list(scenario.weak),
            "trials": len(ok),
            "top1_strong": correct,
            "mean_P_strong_among_hand": mean_p,
            "chance": chance,
            "lift_over_chance": mean_p - chance,
            "beats_chance_top1": correct > chance + 0.05,
            "wrong_pick_counts": wrong_picks,
            "note": scenario.note,
            "example_hand_probs": ok[0]["hand_probs"],
        }
        scenario_rows.append(row)
        print(
            f"{scenario.name}: top1={correct:.1%}  P(strong)={mean_p:.3f}  "
            f"chance={chance:.3f}  {'OK' if correct > chance + 0.05 else 'WEAK'}",
            flush=True,
        )

    by_threat: dict[str, list[dict[str, Any]]] = {}
    for row in scenario_rows:
        by_threat.setdefault(row.get("threat", "?"), []).append(row)

    threat_summary = []
    for threat, rows in by_threat.items():
        if not rows or "top1_strong" not in rows[0]:
            continue
        threat_summary.append(
            {
                "threat": threat,
                "scenarios": len(rows),
                "mean_top1_strong": float(np.mean([r["top1_strong"] for r in rows])),
                "mean_P_strong": float(
                    np.mean([r["mean_P_strong_among_hand"] for r in rows])
                ),
                "mean_chance": float(np.mean([r["chance"] for r in rows])),
                "beats_chance": bool(
                    np.mean([r["top1_strong"] for r in rows])
                    > np.mean([r["chance"] for r in rows]) + 0.05
                ),
            }
        )

    overall_top1 = float(
        np.mean([r["top1_strong"] for r in scenario_rows if "top1_strong" in r])
    )
    overall_p = float(
        np.mean(
            [r["mean_P_strong_among_hand"] for r in scenario_rows if "top1_strong" in r]
        )
    )
    report = {
        "created_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "model_name": "policy-bc-v2",
        "setup": {
            "trials_per_scenario": trials_per_scenario,
            "hand": "forced 4-card hand = 1 strong answer + 3 weak answers",
            "history": "12 neutral cycle plays, then opponent drops the threat",
            "metric": "choice among the 4 hand cards (hand-renormalized probs)",
            "chance": 0.25,
        },
        "summary": {
            "scenarios": len(scenario_rows),
            "overall_top1_strong": overall_top1,
            "overall_mean_P_strong": overall_p,
            "overall_chance": 0.25,
            "beats_chance": overall_top1 > 0.30,
        },
        "by_threat": threat_summary,
        "scenarios": scenario_rows,
        "verdict": (
            "Policy often picks the strong defensive answer over weak hand junk."
            if overall_top1 >= 0.40
            else (
                "Policy only slightly prefers the strong answer — defense priors are weak."
                if overall_top1 >= 0.28
                else "Policy does not reliably find the strong defensive card."
            )
        ),
    }
    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)
    with out.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    print(json.dumps({"summary": report["summary"], "by_threat": threat_summary, "verdict": report["verdict"]}, indent=2))
    print(f"Wrote {out}")
    return report


def Counter_like(items: list[str]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for item in items:
        counts[item] = counts.get(item, 0) + 1
    return dict(sorted(counts.items(), key=lambda kv: -kv[1]))
