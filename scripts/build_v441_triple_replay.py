#!/usr/bin/env python3
"""Build paired Human/v4.4/v4.4.1 continuations for the v4.4.1 report."""

from __future__ import annotations

import argparse
import json
import pickle
from pathlib import Path

from cr_replay_pipeline.policy_infer import load_policy
from cr_replay_pipeline.policy_train import (
    _load_realism_scorer,
    _score_realism,
    rollout_policy_battles,
)
from cr_replay_pipeline.winner_dataset import BattleExample, load_card_costs


ROOT = Path(__file__).resolve().parents[1]


def _display_events(battle: BattleExample) -> list[dict[str, object]]:
    return [
        {
            "t": round(float(event["seconds"]), 2),
            "side": event["side"],
            "card": event["card"],
            "x": round(float(event.get("x", 9000)) / 18000.0, 4),
            "y": round(float(event.get("y", 16000)) / 32000.0, 4),
            "ability": event.get("event_type") == "ability_activation",
        }
        for event in battle.events
    ]


def build(output: Path) -> Path:
    baseline_path = ROOT / "reports" / "policy_bc_v4_4_triple_replay.json"
    cache_path = ROOT / "data" / "policy_battles_cache.pkl"
    baseline = json.loads(baseline_path.read_text(encoding="utf-8"))
    with cache_path.open("rb") as handle:
        cache = pickle.load(handle)
    by_id = {battle.battle_id: battle for battle in cache["battles"]}

    model, vocab, cfg, device = load_policy(ROOT / "models" / "policy_bc_v4.4.1")
    costs = load_card_costs(ROOT / "data" / "card_costs.json")
    judge = _load_realism_scorer(ROOT / "models" / "realism_scorer" / "realism_ensemble.pkl")

    trajectories = []
    for index, old in enumerate(baseline["trajectories"]):
        seed_battle = by_id[old["battleId"]]
        rollout = rollout_policy_battles(
            model,
            vocab,
            costs,
            [seed_battle],
            device,
            n_battles=1,
            warmup_events=int(old.get("warmupEvents", 12)),
            max_new_events=40,
            temperature=0.8,
            seed=44100 + index,
            max_context=int(cfg.get("max_context", 64)),
            threat_dim=int(cfg.get("threat_dim", 0)),
            placement_decode="sample",
            placement_temperature=0.6,
            placement_top_k=5,
            think_steps=int(cfg.get("eval_think_steps", cfg.get("max_think_steps", 0))),
            scheduling="race",
        )[0]
        score = _score_realism(judge, [rollout], costs)[0] if judge is not None else None
        trajectories.append(
            {
                "battleId": old["battleId"],
                "warmupEvents": int(old.get("warmupEvents", 12)),
                "human": old["human"],
                "v44": old["v44"],
                "v441": {
                    "label": "v4.4.1 top-5 T=0.6",
                    "color": "#38bdf8",
                    "score": score,
                    "events": _display_events(rollout),
                },
            }
        )
        print(f"built {index + 1}/{len(baseline['trajectories'])}: {old['battleId']}", flush=True)

    payload = {
        "n": len(trajectories),
        "warmupEvents": 12,
        "note": (
            "Paired held-out prefixes; v4.4 uses its saved argmax-tile rollout and "
            "v4.4.1 uses the deployment two-sided timing race with sampled slots "
            "at T=0.8 and top-5 placement tiles at T=0.6."
        ),
        "trajectories": trajectories,
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    return output


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--output",
        type=Path,
        default=ROOT / "reports" / "policy_bc_v4_4_1_triple_replay.json",
    )
    args = parser.parse_args()
    print(build(args.output.resolve()))
