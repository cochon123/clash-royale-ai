#!/usr/bin/env python3
"""Build shared-pool human-vs-AI matchup payload for the v4.3 report.

Rollout caches define the shared seed set. Humans are recovered by battle_id
from the full battle list so a drifting raw corpus cannot break pairing.

Use --force to regenerate every policy on a fresh shared test pool.
"""

from __future__ import annotations

import argparse
import json
import pickle
import random
from pathlib import Path

import numpy as np

from cr_replay_pipeline.style_train import (
    HARNESS_FEATURES,
    WARMUP_EVENTS,
    _feature_matrix,
    _load_rollout_cache,
    _rollout_cache_path,
    _score_style,
    generate_policy_rollouts,
    truncate_battle,
)
from cr_replay_pipeline.winner_dataset import collect_battles, load_card_costs, split_battles

ROOT = Path(__file__).resolve().parents[1]
STYLE = ROOT / "models" / "style_discriminator"
OUT = ROOT / "reports" / "policy_bc_v4_3_matchup.json"

POLICIES = [
    ("v4", "policy_bc_v4", "models/policy_bc_v4", "#70a1ff"),
    ("v4.1", "policy_bc_v4_1", "models/policy_bc_v4.1", "#ffca63"),
    ("v4.2", "policy_bc_v4_2", "models/policy_bc_v4.2_full", "#70e1b1"),
    ("v4.3", "policy_bc_v4_3", "models/policy_bc_v4.3", "#e8f58b"),
    ("v5", "policy_bc_v5", "models/policy_bc_v5", "#f472b6"),
]


def _seed_id(battle_id: str) -> str:
    return battle_id.replace("-rollout", "")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--force", action="store_true", help="Regenerate all policy rollouts")
    parser.add_argument("--n", type=int, default=512, help="Shared test-pool size when --force")
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--device", default="cuda")
    args = parser.parse_args()

    report = json.loads((STYLE / "report.json").read_text(encoding="utf-8"))
    with (STYLE / "style_ensemble.pkl").open("rb") as handle:
        artifact = pickle.load(handle)
    costs = load_card_costs(ROOT / "data" / "card_costs.json")

    print("Loading battles ...", flush=True)
    battles = collect_battles(ROOT / "data" / "raw", min_card_plays=12)
    all_by_id = {b.battle_id: b for b in battles}

    if args.force:
        _, _, test_split = split_battles(battles, seed=args.seed)
        rng = random.Random(args.seed + 3)
        test_pool = list(test_split)
        rng.shuffle(test_pool)
        test_pool = test_pool[: args.n]
        print(f"Force pool {len(test_pool)}", flush=True)
        for _label, policy_id, path, _color in POLICIES:
            print(f"\nGenerating {policy_id} ...", flush=True)
            generate_policy_rollouts(
                ROOT / path,
                test_pool,
                policy_id=policy_id,
                split_name="test",
                output_dir=STYLE,
                device_name=args.device,
                seed=args.seed + 22,
                force=True,
            )
    else:
        for _label, policy_id, path, _color in POLICIES:
            cache = _rollout_cache_path(STYLE, policy_id, "test")
            if _load_rollout_cache(cache) is None:
                raise SystemExit(
                    f"Missing rollout cache {cache}. Re-run with --force to generate."
                )

    # Shared seed set = intersection of rollout battle ids present in the corpus.
    ai_by_policy = {}
    id_sets = []
    for label, policy_id, _path, _color in POLICIES:
        rolls = _load_rollout_cache(_rollout_cache_path(STYLE, policy_id, "test"))
        mapped = {_seed_id(r.battle_id): r for r in rolls}
        ai_by_policy[label] = mapped
        id_sets.append(set(mapped))
        print(f"loaded {label}: {len(mapped)} rollouts", flush=True)

    common_ids = set.intersection(*id_sets)
    common_ids = {bid for bid in common_ids if bid in all_by_id}
    if len(common_ids) < 200:
        raise SystemExit(
            f"Only {len(common_ids)} shared seed battles found in the current corpus. "
            "Re-run with --force to regenerate a matched pool."
        )
    seed_ids = sorted(common_ids)
    humans_trunc = [truncate_battle(all_by_id[bid]) for bid in seed_ids]
    human_by_id = {bid: all_by_id[bid] for bid in seed_ids}
    print(f"Paired shared pool: {len(seed_ids)}", flush=True)

    x_human = _feature_matrix(humans_trunc, costs)
    human_means = x_human.mean(axis=0)
    human_stds = np.where(x_human.std(axis=0) < 1e-9, 1e-9, x_human.std(axis=0))
    human_scores = np.asarray(_score_style(artifact, humans_trunc, costs))

    top_forensics = [
        row for row in report["forensics"]["features"] if row["feature"] not in HARNESS_FEATURES
    ][:14]
    if len(top_forensics) < 10:
        top_forensics = report["forensics"]["features"][:14]
    feat_idx = [int(r["index"]) for r in top_forensics]
    feat_names = [r["feature"] for r in top_forensics]

    def to_events(battle):
        return [
            {
                "t": round(float(e["seconds"]), 2),
                "side": e["side"],
                "card": e["card"],
                "x": round(float(e["x"]) / 18000.0, 4),
                "y": round(float(e["y"]) / 32000.0, 4),
                "ability": e["event_type"] == "ability_activation",
            }
            for e in battle.events
        ]

    models = []
    for label, policy_id, _path, color in POLICIES:
        paired_ai = [ai_by_policy[label][bid] for bid in seed_ids]
        print(f"{label}: paired={len(paired_ai)}", flush=True)
        x_ai = _feature_matrix(paired_ai, costs)
        ai_scores = np.asarray(_score_style(artifact, paired_ai, costs))
        z = (x_ai.mean(axis=0) - human_means) / human_stds
        focus_z = z[feat_idx]
        focus_abs = np.abs(focus_z)
        rs = np.random.default_rng(0)
        n = len(paired_ai)
        boot = []
        for _ in range(500):
            sample = rs.integers(0, n, size=n)
            m = x_ai[sample].mean(axis=0)
            boot.append(np.abs((m - human_means) / human_stds)[feat_idx].mean())
        boot = np.asarray(boot)
        models.append(
            {
                "id": label,
                "policyId": policy_id,
                "color": color,
                "n": len(paired_ai),
                "meanPHuman": float(ai_scores.mean()),
                "foolRate": float((ai_scores >= 0.5).mean()),
                "humanMeanP": float(human_scores.mean()),
                "featureDelta": [
                    {
                        "feature": feat_names[i],
                        "humanMean": float(human_means[feat_idx[i]]),
                        "aiMean": float(x_ai.mean(axis=0)[feat_idx[i]]),
                        "humanStd": float(human_stds[feat_idx[i]]),
                        "z": float(focus_z[i]),
                        "absZ": float(focus_abs[i]),
                    }
                    for i in range(len(feat_idx))
                ],
                "meanAbsZ": float(focus_abs.mean()),
                "meanAbsZCI": [
                    float(np.percentile(boot, 2.5)),
                    float(np.percentile(boot, 97.5)),
                ],
                "meanAbsZAll": float(np.abs(z).mean()),
            }
        )

    scored = sorted(((len(human_by_id[b].events), b) for b in seed_ids), reverse=True)
    pick = [b for _, b in scored[:24]][::3][:8]
    if len(pick) < 6:
        pick = [b for _, b in scored[:8]]

    trajectories = []
    for bid in pick:
        human = truncate_battle(human_by_id[bid])
        h_score = float(_score_style(artifact, [human], costs)[0])
        entry = {
            "battleId": bid.split("::")[0][:14],
            "warmupEvents": WARMUP_EVENTS,
            "human": {"score": round(h_score, 5), "events": to_events(human)},
            "aiByModel": {},
        }
        for label, mapped in ai_by_policy.items():
            ai = mapped[bid]
            entry["aiByModel"][label] = {
                "score": round(float(_score_style(artifact, [ai], costs)[0]), 5),
                "events": to_events(ai),
            }
        trajectories.append(entry)

    payload = {
        "nHumanPool": len(seed_ids),
        "humanMeanP": float(human_scores.mean()),
        "featureNames": feat_names,
        "models": models,
        "trajectories": trajectories,
        "protocol": f"shared {len(seed_ids)}-battle rollout pool · alternation transfer judge",
        "note": (
            "Δ is (AI − human) / human σ on non-harness style features. "
            "Mean |Δ| is the overall style distance (95% bootstrap CI over games)."
        ),
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(payload), encoding="utf-8")
    print(
        "wrote",
        OUT,
        "traj",
        len(trajectories),
        "meanAbsZ",
        {m["id"]: round(m["meanAbsZ"], 3) for m in models},
        flush=True,
    )


if __name__ == "__main__":
    main()
