"""Score v4/v5 policies on the frozen style transfer discriminator (human vs v2)."""

from __future__ import annotations

import json
import pickle
import random
import time
from pathlib import Path

from cr_replay_pipeline.style_train import (
    _eval_detection,
    generate_policy_rollouts,
)
from cr_replay_pipeline.winner_dataset import collect_battles, load_card_costs, split_battles


STYLE_DIR = Path("models/style_discriminator")
OUT_PATH = Path("reports/style_policy_v4_v5_eval.json")

POLICIES = [
    ("policy_bc_v4", "models/policy_bc_v4"),
    ("policy_bc_v4_1", "models/policy_bc_v4.1"),
    ("policy_bc_v4_2", "models/policy_bc_v4.2_full"),
    ("policy_bc_v4_3", "models/policy_bc_v4.3"),
    ("policy_bc_v5", "models/policy_bc_v5"),
]


def main() -> None:
    report = json.load((STYLE_DIR / "report.json").open(encoding="utf-8"))
    with (STYLE_DIR / "style_ensemble.pkl").open("rb") as handle:
        artifact = pickle.load(handle)

    seed = 42
    eval_battles = int(report["data"]["eval_battles_used"])
    min_card_plays = int(report["data"]["min_card_plays"])
    costs = load_card_costs("data/card_costs.json")

    print(f"Loading battles (min_card_plays={min_card_plays}) ...", flush=True)
    battles = collect_battles("data/raw", min_card_plays=min_card_plays)
    _, _, test_split = split_battles(battles, seed=seed)
    rng = random.Random(seed + 3)
    test_pool = list(test_split)
    rng.shuffle(test_pool)
    test_pool = test_pool[: min(eval_battles, len(test_pool))]
    print(f"Test pool: {len(test_pool)} human seed battles", flush=True)
    print(f"Transfer judge trained on: {report['transfer']['trained_on']}", flush=True)

    results: list[dict] = []
    for policy_id, path in POLICIES:
        if not Path(path).exists():
            print(f"SKIP missing {path}", flush=True)
            continue
        t0 = time.time()
        print(f"\n=== {policy_id} ===", flush=True)
        rolls = generate_policy_rollouts(
            path,
            test_pool,
            policy_id=policy_id,
            split_name="test",
            output_dir=STYLE_DIR,
            device_name="cuda",
            seed=seed + 22,
            force=False,
        )
        metrics = _eval_detection(artifact, test_pool, rolls, costs)
        row = {
            "policy_id": policy_id,
            "path": path,
            "protocol": "alternation",
            "n_human": len(test_pool),
            "n_ai": len(rolls),
            "seconds": round(time.time() - t0, 1),
            **{
                k: metrics[k]
                for k in [
                    "acc",
                    "auc",
                    "fool_rate_at_0.5",
                    "detect_rate_at_0.5",
                    "mean_P_human_ai",
                    "mean_P_human_real",
                    "human_likeness",
                    "n",
                ]
            },
        }
        results.append(row)
        print(
            f"P(human|AI)={row['mean_P_human_ai']:.6f}  "
            f"fool@0.5={row['fool_rate_at_0.5'] * 100:.2f}%  "
            f"AUC={row['auc']:.4f}  detect@0.5={row['detect_rate_at_0.5'] * 100:.1f}%",
            flush=True,
        )

    archived = [
        {
            "policy_id": row["policy_id"],
            "mean_P_human_ai": row["mean_P_human_ai"],
            "fool_rate_at_0.5": row["fool_rate_at_0.5"],
            "auc": row["auc"],
            "source": "style_discriminator/report.json transfer.eval",
        }
        for row in report["transfer"]["eval"]
    ]

    v5 = json.load(open("models/policy_bc_v5/report.json", encoding="utf-8"))
    clock_note = {
        "policy_id": "policy_bc_v5",
        "protocol": "clock_actor (from v5 report)",
        "mean_P_human_ai": v5["style"]["clock"]["full"]["mean_P_human_ai"],
        "fool_rate_at_0.5": v5["style"]["clock"]["full"]["fool_rate_at_0.5"],
        "auc": v5["style"]["clock"]["full"]["auc"],
        "v4_1_clock_P_human": v5["style"]["v4_1_compare"]["full"]["mean_P_human_ai"],
        "v4_1_alt_P_human": v5["style"]["v4_1_compare"]["alternation"]["full"]["mean_P_human_ai"],
        "v5_alt_P_human": v5["style"]["alternation"]["full"]["mean_P_human_ai"],
        "note": (
            "Clock protocol changes timing/actor tells the judge was trained on; "
            "high P(human) here is mostly harness mismatch, not true stealth."
        ),
    }

    out = {
        "judge": {
            "checkpoint": "models/style_discriminator/style_ensemble.pkl",
            "trained_on": report["transfer"]["trained_on"],
            "protocol": "alternation rollouts (warmup=12, max_new=40, T=0.8)",
            "eval_battles": len(test_pool),
            "human_mean_P": results[0]["mean_P_human_real"] if results else None,
        },
        "alternation_transfer": sorted(results, key=lambda r: -r["mean_P_human_ai"]),
        "archived_transfer_eval": archived,
        "clock_protocol_caveat": clock_note,
    }
    OUT_PATH.write_text(json.dumps(out, indent=2), encoding="utf-8")
    print(f"\nWrote {OUT_PATH}", flush=True)
    print(
        json.dumps(
            {
                "ranking": [
                    {
                        k: r[k]
                        for k in ["policy_id", "mean_P_human_ai", "fool_rate_at_0.5", "auc"]
                    }
                    for r in out["alternation_transfer"]
                ]
            },
            indent=2,
        ),
        flush=True,
    )


if __name__ == "__main__":
    main()
