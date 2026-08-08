"""Train a realism scorer: real battles vs legal-but-random synthetics."""

from __future__ import annotations

import json
import pickle
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import numpy as np

from .realism_generate import (
    TIERS,
    TimingPrior,
    generate_negatives_for_battles,
)
from .winner_dataset import (
    SPELL_CARDS,
    WIN_CONDITIONISH,
    BattleExample,
    collect_battles,
    load_card_costs,
    split_battles,
    summarize_split,
    _elixir_gain_rate,
)


def _auc_binary(labels: np.ndarray, probs: np.ndarray) -> float:
    labels = labels.astype(np.int64)
    probs = np.asarray(probs, dtype=np.float64)
    if labels.min() == labels.max():
        return 0.5
    if float(probs.max()) - float(probs.min()) < 1e-12:
        return 0.5
    order = np.argsort(probs)
    ranks = np.empty_like(order, dtype=np.float64)
    ranks[order] = np.arange(1, len(probs) + 1)
    pos = labels == 1
    n_pos = int(pos.sum())
    n_neg = int((~pos).sum())
    if n_pos == 0 or n_neg == 0:
        return 0.5
    return float((ranks[pos].sum() - n_pos * (n_pos + 1) / 2.0) / (n_pos * n_neg))


def _safe_stats(values: list[float]) -> tuple[float, float, float, float, float]:
    if not values:
        return 0.0, 0.0, 0.0, 0.0, 0.0
    arr = np.asarray(values, dtype=np.float64)
    return (
        float(arr.mean()),
        float(arr.std()),
        float(np.percentile(arr, 10)),
        float(np.percentile(arr, 50)),
        float(np.percentile(arr, 90)),
    )


def extract_realism_features(
    battle: BattleExample,
    costs: dict[str, int],
) -> np.ndarray:
    """Sequence-only features; no official elixir tables (synthetics lack them)."""
    events = battle.events
    now = float(events[-1]["seconds"]) if events else 0.0
    n = len(events)
    plays = [event for event in events if event["event_type"] == "card_play"]
    abilities = n - len(plays)

    gaps = [
        float(events[index]["seconds"]) - float(events[index - 1]["seconds"])
        for index in range(1, n)
    ]
    gap_mean, gap_std, gap_p10, gap_p50, gap_p90 = _safe_stats(gaps)

    sides = [event["side"] for event in events]
    alternations = 0
    streak = 1
    max_streak = 1
    for previous, current in zip(sides, sides[1:]):
        if previous != current:
            alternations += 1
            streak = 1
        else:
            streak += 1
            max_streak = max(max_streak, streak)
    alt_rate = alternations / max(n - 1, 1)

    team_elixir = 5.0
    opp_elixir = 5.0
    last = 0.0
    leak_team = 0.0
    leak_opp = 0.0
    pre_play_elixir: list[float] = []
    spend_team = 0.0
    spend_opp = 0.0
    costs_played: list[float] = []
    consecutive_cost_delta: list[float] = []
    last_cost: float | None = None
    team_x: list[float] = []
    team_y: list[float] = []
    opp_x: list[float] = []
    opp_y: list[float] = []
    team_deep = 0
    opp_deep = 0
    team_bridge = 0
    opp_bridge = 0
    team_back = 0
    opp_back = 0
    team_left = 0
    opp_left = 0
    team_spells = 0
    opp_spells = 0
    team_wincons = 0
    opp_wincons = 0
    team_unique: set[str] = set()
    opp_unique: set[str] = set()
    team_times: list[float] = []
    opp_times: list[float] = []
    response_latencies: list[float] = []
    pending_deep: dict[str, float] = {}
    single_plays = 0
    double_plays = 0
    triple_plays = 0
    dump_plays = 0
    tiles: set[tuple[int, int]] = set()

    for event in events:
        seconds = float(event["seconds"])
        gain = (seconds - last) / _elixir_gain_rate(seconds)
        # Leak approximation: elixir that would accrue past 10.
        if team_elixir >= 10:
            leak_team += max(0.0, gain)
        if opp_elixir >= 10:
            leak_opp += max(0.0, gain)
        team_elixir = min(10.0, team_elixir + gain)
        opp_elixir = min(10.0, opp_elixir + gain)

        card = event["card"]
        cost = (
            1.0
            if event["event_type"] == "ability_activation"
            else float(costs.get(card, 4))
        )
        x = float(event["x"]) / 18000.0
        y = float(event["y"]) / 32000.0
        side = event["side"]
        pre = team_elixir if side == "team" else opp_elixir
        pre_play_elixir.append(pre)
        if pre - cost <= 1.5:
            dump_plays += 1
        if last_cost is not None:
            consecutive_cost_delta.append(abs(cost - last_cost))
        last_cost = cost
        costs_played.append(cost)
        tiles.add((int(event["x"]) // 1000, int(event["y"]) // 1000))

        if seconds < 120:
            single_plays += 1
        elif seconds < 240:
            double_plays += 1
        else:
            triple_plays += 1

        is_spell = card in SPELL_CARDS
        is_wincon = card in WIN_CONDITIONISH
        if side == "team":
            team_elixir = max(0.0, team_elixir - cost)
            spend_team += cost
            team_x.append(x)
            team_y.append(y)
            team_times.append(seconds)
            team_unique.add(card)
            if is_spell:
                team_spells += 1
            if is_wincon:
                team_wincons += 1
            if y > 0.55:
                team_deep += 1
                pending_deep["team"] = seconds
                if "opponent" in pending_deep:
                    response_latencies.append(seconds - pending_deep.pop("opponent"))
            elif 0.42 <= y <= 0.58:
                team_bridge += 1
            else:
                team_back += 1
            if x < 0.5:
                team_left += 1
        else:
            opp_elixir = max(0.0, opp_elixir - cost)
            spend_opp += cost
            opp_x.append(x)
            opp_y.append(y)
            opp_times.append(seconds)
            opp_unique.add(card)
            if is_spell:
                opp_spells += 1
            if is_wincon:
                opp_wincons += 1
            if y < 0.45:
                opp_deep += 1
                pending_deep["opponent"] = seconds
                if "team" in pending_deep:
                    response_latencies.append(seconds - pending_deep.pop("team"))
            elif 0.42 <= y <= 0.58:
                opp_bridge += 1
            else:
                opp_back += 1
            if x < 0.5:
                opp_left += 1
        last = seconds

    def side_pos(values: list[float]) -> tuple[float, float]:
        if not values:
            return 0.5, 0.0
        arr = np.asarray(values, dtype=np.float64)
        return float(arr.mean()), float(arr.std())

    team_x_mean, team_x_std = side_pos(team_x)
    team_y_mean, team_y_std = side_pos(team_y)
    opp_x_mean, opp_x_std = side_pos(opp_x)
    opp_y_mean, opp_y_std = side_pos(opp_y)
    cost_mean, cost_std, _, _, _ = _safe_stats(costs_played)
    delta_mean, delta_std, _, _, _ = _safe_stats(consecutive_cost_delta)
    resp_mean, resp_std, _, _, _ = _safe_stats(response_latencies)
    pre_mean, pre_std, _, _, _ = _safe_stats(pre_play_elixir)

    team_plays = max(len(team_times), 1)
    opp_plays = max(len(opp_times), 1)
    plays_n = max(len(plays), 1)

    team_deck_cost = [float(costs.get(card, 4)) for card in battle.team_deck]
    opp_deck_cost = [float(costs.get(card, 4)) for card in battle.opponent_deck]

    features = np.asarray(
        [
            now / 300.0,
            float(n),
            float(len(plays)),
            float(abilities),
            float(len(plays)) / max(now / 60.0, 1e-3),
            gap_mean,
            gap_std,
            gap_p10,
            gap_p50,
            gap_p90,
            float(sum(1 for gap in gaps if gap < 1.0)) / max(len(gaps), 1),
            float(sum(1 for gap in gaps if gap < 2.0)) / max(len(gaps), 1),
            float(sum(1 for gap in gaps if gap > 8.0)) / max(len(gaps), 1),
            alt_rate,
            float(max_streak),
            single_plays / plays_n,
            double_plays / plays_n,
            triple_plays / plays_n,
            leak_team,
            leak_opp,
            leak_team - leak_opp,
            pre_mean,
            pre_std,
            dump_plays / max(n, 1),
            spend_team,
            spend_opp,
            spend_team - spend_opp,
            cost_mean,
            cost_std,
            delta_mean,
            delta_std,
            team_x_mean,
            team_x_std,
            team_y_mean,
            team_y_std,
            opp_x_mean,
            opp_x_std,
            opp_y_mean,
            opp_y_std,
            team_deep / team_plays,
            opp_deep / opp_plays,
            team_bridge / team_plays,
            opp_bridge / opp_plays,
            team_back / team_plays,
            opp_back / opp_plays,
            team_left / team_plays,
            opp_left / opp_plays,
            float(len(tiles)) / max(n, 1),
            len(team_unique) / 8.0,
            len(opp_unique) / 8.0,
            team_spells / team_plays,
            opp_spells / opp_plays,
            team_wincons / team_plays,
            opp_wincons / opp_plays,
            resp_mean,
            resp_std,
            float(len(response_latencies)) / max(team_deep + opp_deep, 1),
            float(len(pending_deep)),
            float(np.mean(team_deck_cost)),
            float(np.mean(opp_deck_cost)),
            float(sum(card in SPELL_CARDS for card in battle.team_deck)),
            float(sum(card in SPELL_CARDS for card in battle.opponent_deck)),
            float(sum(card in WIN_CONDITIONISH for card in battle.team_deck)),
            float(sum(card in WIN_CONDITIONISH for card in battle.opponent_deck)),
            abs(len(team_times) - len(opp_times)) / max(n, 1),
            1.0 if now >= 240 else 0.0,
            1.0 if now < 120 else 0.0,
        ],
        dtype=np.float64,
    )
    return features


def _feature_matrix(
    battles: list[BattleExample], costs: dict[str, int]
) -> np.ndarray:
    return np.stack([extract_realism_features(battle, costs) for battle in battles])


def _build_labeled_split(
    battles: list[BattleExample],
    costs: dict[str, int],
    timing: TimingPrior,
    seed: int,
    per_tier: int,
) -> tuple[list[BattleExample], np.ndarray, list[str]]:
    negatives = generate_negatives_for_battles(
        battles, costs, timing, seed=seed, per_tier=per_tier
    )
    examples: list[BattleExample] = list(battles)
    labels = [1] * len(battles)
    tiers = ["real"] * len(battles)
    for synthetic, tier in negatives:
        examples.append(synthetic)
        labels.append(0)
        tiers.append(tier)
    return examples, np.asarray(labels, dtype=np.int64), tiers


def _metrics(labels: np.ndarray, probs: np.ndarray) -> dict[str, float]:
    preds = (probs >= 0.5).astype(np.int64)
    return {
        "acc": float((preds == labels).mean()),
        "auc": _auc_binary(labels, probs),
        "n": int(len(labels)),
        "mean_score_real": float(probs[labels == 1].mean()) if (labels == 1).any() else 0.0,
        "mean_score_fake": float(probs[labels == 0].mean()) if (labels == 0).any() else 0.0,
    }


def _tier_metrics(
    labels: np.ndarray,
    probs: np.ndarray,
    tiers: list[str],
) -> list[dict[str, Any]]:
    rows = []
    tiers_arr = np.asarray(tiers)
    real_mask = labels == 1
    for tier in TIERS:
        neg_mask = tiers_arr == tier
        if not neg_mask.any() or not real_mask.any():
            continue
        mask = real_mask | neg_mask
        sub_labels = labels[mask]
        sub_probs = probs[mask]
        rows.append(
            {
                "tier": tier,
                "n_negatives": int(neg_mask.sum()),
                **_metrics(sub_labels, sub_probs),
                "mean_score_negatives": float(probs[neg_mask].mean()),
                "reject_rate_at_0.5": float((probs[neg_mask] < 0.5).mean()),
            }
        )
    return rows


def _score_histogram(
    labels: np.ndarray, probs: np.ndarray, bins: int = 20
) -> dict[str, Any]:
    edges = np.linspace(0.0, 1.0, bins + 1)
    real = probs[labels == 1]
    fake = probs[labels == 0]
    return {
        "edges": edges.tolist(),
        "real": np.histogram(real, bins=edges)[0].tolist() if len(real) else [0] * bins,
        "synthetic": (
            np.histogram(fake, bins=edges)[0].tolist() if len(fake) else [0] * bins
        ),
    }


def train_realism_scorer(
    input_dir: str | Path = "data/raw",
    output_dir: str | Path = "models/realism_scorer",
    card_costs_path: str | Path = "data/card_costs.json",
    min_card_plays: int = 12,
    seed: int = 42,
    trees: int = 120,
    per_tier: int = 1,
) -> dict[str, Any]:
    from sklearn.ensemble import ExtraTreesClassifier, HistGradientBoostingClassifier
    from sklearn.metrics import log_loss

    started = time.time()
    created_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    output = Path(output_dir)
    output.mkdir(parents=True, exist_ok=True)

    print(f"Loading battles from {input_dir} ...", flush=True)
    battles = collect_battles(input_dir, min_card_plays=min_card_plays)
    if len(battles) < 50:
        raise RuntimeError(
            f"Need at least 50 usable battles; found {len(battles)}"
        )

    train_battles, val_battles, test_battles = split_battles(battles, seed=seed)
    costs = load_card_costs(card_costs_path)
    timing = TimingPrior.from_battles(train_battles)

    print(
        f"Generating negatives (tiers={list(TIERS)}, per_tier={per_tier}) ...",
        flush=True,
    )
    train_examples, y_train, train_tiers = _build_labeled_split(
        train_battles, costs, timing, seed=seed + 11, per_tier=per_tier
    )
    val_examples, y_val, val_tiers = _build_labeled_split(
        val_battles, costs, timing, seed=seed + 22, per_tier=per_tier
    )
    test_examples, y_test, test_tiers = _build_labeled_split(
        test_battles, costs, timing, seed=seed + 33, per_tier=per_tier
    )

    print("Extracting realism features ...", flush=True)
    x_train = _feature_matrix(train_examples, costs)
    x_val = _feature_matrix(val_examples, costs)
    x_test = _feature_matrix(test_examples, costs)

    hgb = HistGradientBoostingClassifier(
        max_depth=8,
        learning_rate=0.05,
        max_iter=400,
        l2_regularization=0.1,
        min_samples_leaf=12,
        random_state=seed,
    )
    hgb.fit(x_train, y_train)
    hgb_val = hgb.predict_proba(x_val)[:, 1]
    hgb_test = hgb.predict_proba(x_test)[:, 1]

    tree = ExtraTreesClassifier(
        n_estimators=trees,
        max_features="sqrt",
        min_samples_leaf=2,
        n_jobs=-1,
        random_state=seed,
        class_weight="balanced",
    )
    tree.fit(x_train, y_train)
    tree_val = tree.predict_proba(x_val)[:, 1]
    tree_test = tree.predict_proba(x_test)[:, 1]

    blend_candidates = np.linspace(0.0, 1.0, 21)
    blend_losses = [
        float(log_loss(y_val, weight * hgb_val + (1.0 - weight) * tree_val))
        for weight in blend_candidates
    ]
    hgb_weight = float(blend_candidates[int(np.argmin(blend_losses))])
    val_prob = hgb_weight * hgb_val + (1.0 - hgb_weight) * tree_val
    test_prob = hgb_weight * hgb_test + (1.0 - hgb_weight) * tree_test

    # Majority baseline: always predict real? Better: always predict the majority class.
    majority = int(np.round(y_train.mean()))
    baseline_prob = np.full(len(y_test), float(majority), dtype=np.float64)

    # Cumulative tree stages for animation / curves. Keep tree-only metrics even
    # when the selected blend weight is ~100% HGB so the curves are not flat.
    if trees <= 40:
        checkpoints = set(range(1, trees + 1))
    else:
        checkpoints = {1, 2, 3, 5, 8, 10, trees, *range(15, trees + 1, 5)}
    running = np.zeros(len(y_test), dtype=np.float64)
    training_stages: list[dict[str, Any]] = []
    hgb_metrics = _metrics(y_test, hgb_test)
    for tree_count, estimator in enumerate(tree.estimators_, start=1):
        running += estimator.predict_proba(x_test)[:, 1]
        if tree_count not in checkpoints:
            continue
        stage_tree = running / tree_count
        stage_prob = hgb_weight * hgb_test + (1.0 - hgb_weight) * stage_tree
        stage_metrics = _metrics(y_test, stage_prob)
        tree_metrics = _metrics(y_test, stage_tree)
        training_stages.append(
            {
                "trees": tree_count,
                "acc": stage_metrics["acc"],
                "auc": stage_metrics["auc"],
                "tree_acc": tree_metrics["acc"],
                "tree_auc": tree_metrics["auc"],
                "hgb_acc": hgb_metrics["acc"],
                "hgb_auc": hgb_metrics["auc"],
            }
        )

    val_metrics = _metrics(y_val, val_prob)
    test_metrics = _metrics(y_test, test_prob)
    tier_rows = _tier_metrics(y_test, test_prob, test_tiers)
    histogram = _score_histogram(y_test, test_prob)

    artifact = {
        "models": {
            "hist_gradient_boosting": hgb,
            "extra_trees": tree,
        },
        "hgb_weight": hgb_weight,
        "seed": seed,
        "trees": trees,
        "per_tier": per_tier,
        "feature_version": 1,
        "tiers": list(TIERS),
        "created_at": created_at,
    }
    artifact_path = output / "realism_ensemble.pkl"
    with artifact_path.open("wb") as handle:
        pickle.dump(artifact, handle)

    report = {
        "model_name": "realism-scorer-v1",
        "model_version": "1.0.0",
        "created_at": created_at,
        "seconds": round(time.time() - started, 1),
        "compute": {
            "device": "cpu",
            "framework": "scikit-learn",
            "hist_gradient_boosting_max_iter": 400,
            "extra_trees": trees,
            "blend": {
                "hgb_weight": hgb_weight,
                "extra_trees_weight": 1.0 - hgb_weight,
                "selection": "minimum validation log-loss",
            },
        },
        "data": {
            "battles_total": len(battles),
            "min_card_plays": min_card_plays,
            "negatives_per_real": len(TIERS) * per_tier,
            "tiers": list(TIERS),
            "feature_dim": int(x_train.shape[1]),
            "train_rows": int(len(y_train)),
            "val_rows": int(len(y_val)),
            "test_rows": int(len(y_test)),
            "splits": [
                summarize_split("train", train_battles),
                summarize_split("val", val_battles),
                summarize_split("test", test_battles),
            ],
        },
        "val": {
            **val_metrics,
            "log_loss": float(log_loss(y_val, val_prob)),
            "by_tier": _tier_metrics(y_val, val_prob, val_tiers),
        },
        "test": {
            **test_metrics,
            "log_loss": float(log_loss(y_test, test_prob)),
            "by_tier": tier_rows,
        },
        "baseline": {
            "name": "majority_class",
            "test": _metrics(y_test, baseline_prob),
        },
        "score_histogram": histogram,
        "training_stages": training_stages,
        "checkpoint": str(artifact_path),
        "lessons": [
            "Random-but-legal negatives are only informative when tiered; easy chaos is a weak test.",
            "Hard negatives (perturbed real games) measure whether the model learned structure beyond deck identity.",
            "Sequence-only features avoid official elixir tables so synthetics stay comparable to real rows.",
            "A high easy-tier AUC with a weak hard-tier AUC means the scorer detects chaos, not strategic realism.",
        ],
        "notes": (
            "Binary realism scorer: P(battle is real) from action-sequence statistics. "
            "Negatives obey deck membership, elixir affordability, and placement-half "
            "constraints. Hard tier perturbs real timelines/placements/card mappings."
        ),
    }

    report_path = output / "report.json"
    with report_path.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)

    stages_path = output / "training_stages.json"
    with stages_path.open("w", encoding="utf-8") as handle:
        json.dump(training_stages, handle, indent=2)

    print(
        json.dumps(
            {
                "test": report["test"],
                "by_tier": tier_rows,
                "blend": report["compute"]["blend"],
            },
            indent=2,
        )
    )
    print(f"Wrote {report_path}")
    return report
