from __future__ import annotations

import json
import pickle
import re
import time
from pathlib import Path
from typing import Any
from urllib.parse import parse_qs, urlparse

import numpy as np
from lxml import html as lxml_html

from .winner_dataset import (
    SPELL_CARDS,
    WIN_CONDITIONISH,
    BattleExample,
    collect_battles,
    load_card_costs,
    split_battles,
    summarize_split,
)

CHIP_SPELLS = frozenset(
    {
        "arrows",
        "earthquake",
        "fireball",
        "goblin-curse",
        "lightning",
        "poison",
        "rocket",
        "royal-delivery",
        "void",
    }
)


def parse_official_elixir_stats(page: str) -> dict[str, dict[str, dict[str, float]]] | None:
    root = lxml_html.fromstring(page)

    def parse_table(table) -> dict[str, dict[str, float]]:
        stats: dict[str, dict[str, float]] = {}
        for row in table.xpath(".//tr"):
            title = "".join(
                row.xpath('.//td[contains(@class,"title")]//text()')
            ).strip().lower()
            count_txt = "".join(row.xpath('.//td[contains(@class,"count")]//text()'))
            elix_txt = "".join(row.xpath('.//td[contains(@class,"elixir")]//text()'))
            if not title:
                continue
            counts = re.findall(r"\d+", count_txt)
            elixirs = re.findall(r"[\d.]+", elix_txt)
            stats[title] = {
                "count": float(counts[0]) if counts else 0.0,
                "elixir": float(elixirs[0]) if elixirs else 0.0,
            }
        leaked = re.search(r"Leaked\s+([\d.]+)", " ".join(table.itertext()), re.I)
        if leaked:
            stats["leaked"] = {"count": 0.0, "elixir": float(leaked.group(1))}
        return stats

    result: dict[str, dict[str, dict[str, float]] | None] = {
        "team": None,
        "opponent": None,
    }
    for table in root.xpath("//table[contains(@class,'replay_elixir_table')]"):
        side = None
        cur = table
        for _ in range(8):
            cur = cur.getparent()
            if cur is None:
                break
            cls = f" {(cur.get('class') or '')} "
            if " opponent " in cls:
                side = "opponent"
                break
            if " replay_team " in cls or re.search(r"(^| )team( |$)", cls.strip()):
                side = "team"
                break
        if side is not None:
            result[side] = parse_table(table)

    if result["team"] is None or result["opponent"] is None:
        tables = root.xpath("//table[contains(@class,'replay_elixir_table')]")
        if len(tables) < 2:
            return None
        result["team"] = parse_table(tables[0])
        result["opponent"] = parse_table(tables[1])
    return {"team": result["team"], "opponent": result["opponent"]}


def build_official_elixir_cache(
    input_dir: str | Path,
    cache_path: str | Path | None = None,
) -> dict[str, Any]:
    source = Path(input_dir)
    cache_file = (
        Path(cache_path)
        if cache_path is not None
        else source.parent / "official_elixir_cache.pkl"
    )
    file_count = sum(1 for _ in source.glob("*.json"))
    if cache_file.exists():
        try:
            with cache_file.open("rb") as handle:
                cached = pickle.load(handle)
            if (
                isinstance(cached, dict)
                and cached.get("file_count") == file_count
                and isinstance(cached.get("stats"), dict)
            ):
                print(
                    f"Loaded official elixir stats for {len(cached['stats'])} files "
                    f"({cache_file})",
                    flush=True,
                )
                return cached["stats"]
        except Exception:
            pass

    stats: dict[str, Any] = {}
    for index, path in enumerate(sorted(source.glob("*.json")), start=1):
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
            page = data["payload"]["html"]
            parsed = parse_official_elixir_stats(page)
            if parsed is None:
                continue
            query = parse_qs(urlparse(data.get("request_url") or "").query)
            battle_id = (query.get("tag") or [path.stem])[0]
            stats[battle_id] = parsed
            stats[path.stem] = parsed
        except Exception:
            continue
        if index % 3000 == 0:
            print(f"official elixir {index}/{file_count}", flush=True)

    try:
        with cache_file.open("wb") as handle:
            pickle.dump({"file_count": file_count, "stats": stats}, handle)
        print(f"Cached official elixir stats -> {cache_file}", flush=True)
    except OSError:
        pass
    return stats


def _build_card_index(battles: list[BattleExample]) -> dict[str, int]:
    names = {
        card
        for battle in battles
        for card in list(battle.team_deck)
        + list(battle.opponent_deck)
        + [event["card"] for event in battle.events if event.get("card")]
    }
    return {name: index for index, name in enumerate(sorted(names))}


def extract_tabular_features(
    battle: BattleExample,
    costs: dict[str, int],
    card_index: dict[str, int],
    official: dict[str, Any] | None = None,
) -> np.ndarray:
    prefix = battle.events
    now = float(prefix[-1]["seconds"]) if prefix else 0.0
    card_count = len(card_index)
    play_team = np.zeros(card_count, dtype=np.float64)
    play_opp = np.zeros(card_count, dtype=np.float64)
    late_team = np.zeros(card_count, dtype=np.float64)
    late_opp = np.zeros(card_count, dtype=np.float64)
    deck_team = np.zeros(card_count, dtype=np.float64)
    deck_opp = np.zeros(card_count, dtype=np.float64)
    for card in battle.team_deck:
        if card in card_index:
            deck_team[card_index[card]] = 1.0
    for card in battle.opponent_deck:
        if card in card_index:
            deck_opp[card_index[card]] = 1.0

    last = 0.0
    team_elixir = 5.0
    opp_elixir = 5.0
    team_spent = 0.0
    opp_spent = 0.0
    team_deep = 0
    opp_deep = 0
    team_high = 0
    opp_high = 0
    team_ot = 0.0
    opp_ot = 0.0
    team_chip = 0
    opp_chip = 0
    last_team_deep = -999.0
    last_opp_deep = -999.0
    deep_team: list[float] = []
    deep_opp: list[float] = []
    spend_t: list[float] = []
    spend_o: list[float] = []
    times_t: list[float] = []
    times_o: list[float] = []
    y_team: list[float] = []
    y_opp: list[float] = []
    rev: list[tuple[str, bool]] = []

    for event in prefix:
        seconds = float(event["seconds"])
        gain = (seconds - last) / (2.8 if seconds < 120 else 1.4 if seconds < 240 else 0.9)
        team_elixir = min(10.0, team_elixir + gain)
        opp_elixir = min(10.0, opp_elixir + gain)
        card = event["card"]
        cost = (
            1.0
            if event["event_type"] == "ability_activation"
            else float(costs.get(card, 4))
        )
        y = float(event["y"]) / 32000.0
        card_id = card_index.get(card)
        if event["side"] == "team":
            team_elixir = max(0.0, team_elixir - cost)
            team_spent += cost
            times_t.append(seconds)
            spend_t.append(cost)
            y_team.append(y)
            if card_id is not None:
                play_team[card_id] += 1.0
            deep = y > 0.55
            if deep:
                team_deep += 1
                deep_team.append(seconds)
                last_team_deep = seconds
            if y > 0.70:
                team_high += 1
            if seconds >= 240:
                team_ot += cost
            if card in CHIP_SPELLS and deep:
                team_chip += 1
        else:
            opp_elixir = max(0.0, opp_elixir - cost)
            opp_spent += cost
            times_o.append(seconds)
            spend_o.append(cost)
            y_opp.append(y)
            if card_id is not None:
                play_opp[card_id] += 1.0
            deep = y < 0.45
            if deep:
                opp_deep += 1
                deep_opp.append(seconds)
                last_opp_deep = seconds
            if y < 0.30:
                opp_high += 1
            if seconds >= 240:
                opp_ot += cost
            if card in CHIP_SPELLS and deep:
                opp_chip += 1
        rev.append((event["side"], deep))
        last = seconds

    for event in prefix:
        if float(event["seconds"]) < now - 45:
            continue
        card_id = card_index.get(event["card"])
        if card_id is None:
            continue
        if event["side"] == "team":
            late_team[card_id] += 1.0
        else:
            late_opp[card_id] += 1.0

    unanswered_team = 0
    unanswered_opp = 0
    for side, deep in reversed(rev):
        if deep and side == "team":
            if unanswered_opp == 0:
                unanswered_team += 1
            else:
                break
        elif deep and side == "opponent":
            if unanswered_team == 0:
                unanswered_opp += 1
            else:
                break
        elif unanswered_team or unanswered_opp:
            break

    def window_sum(times: list[float], values: list[float], window: float) -> float:
        return sum(
            value for time, value in zip(times, values) if time >= now - window
        )

    def window_count(times: list[float], window: float) -> float:
        return float(sum(1 for time in times if time >= now - window))

    features: list[float] = [
        now / 300.0,
        team_spent - opp_spent,
        team_spent,
        opp_spent,
        float(team_deep - opp_deep),
        team_elixir - opp_elixir,
        team_elixir,
        opp_elixir,
        window_sum(times_t, spend_t, 10) - window_sum(times_o, spend_o, 10),
        window_sum(times_t, spend_t, 20) - window_sum(times_o, spend_o, 20),
        window_sum(times_t, spend_t, 30) - window_sum(times_o, spend_o, 30),
        window_sum(times_t, spend_t, 45) - window_sum(times_o, spend_o, 45),
        window_sum(times_t, spend_t, 60) - window_sum(times_o, spend_o, 60),
        window_count(deep_team, 10) - window_count(deep_opp, 10),
        window_count(deep_team, 20) - window_count(deep_opp, 20),
        window_count(deep_team, 30) - window_count(deep_opp, 30),
        window_count(deep_team, 45) - window_count(deep_opp, 45),
        (now - last_team_deep) if last_team_deep > -100 else 200.0,
        (now - last_opp_deep) if last_opp_deep > -100 else 200.0,
        1.0 if last_team_deep >= last_opp_deep else 0.0,
        team_ot - opp_ot,
        float(team_high - opp_high),
        float(team_chip - opp_chip),
        float(unanswered_team),
        float(unanswered_opp),
        float(unanswered_team - unanswered_opp),
        (float(np.mean(y_team)) if y_team else 0.5)
        - (float(np.mean(y_opp)) if y_opp else 0.5),
        1.0 if now < 120 else 0.0,
        1.0 if now < 180 else 0.0,
        1.0 if now >= 240 else 0.0,
    ]
    for width in (3, 5, 8):
        team_y = float(np.mean(y_team[-width:])) if y_team else 0.5
        opp_y = float(np.mean(y_opp[-width:])) if y_opp else 0.5
        features.extend([team_y, opp_y, team_y - opp_y])

    spend20 = window_sum(times_t, spend_t, 20) - window_sum(times_o, spend_o, 20)
    last_deep = 1.0 if last_team_deep >= last_opp_deep else -1.0
    features.extend(
        [
            spend20 * last_deep,
            unanswered_team * max(spend20, 0.0),
            unanswered_opp * max(-spend20, 0.0),
        ]
    )

    side_stats = (official or {}).get(battle.battle_id)
    def official_value(side: str, key: str, field: str) -> float:
        if not side_stats:
            return 0.0
        return float(((side_stats.get(side) or {}).get(key) or {}).get(field) or 0.0)

    team_total = official_value("team", "total", "elixir")
    opp_total = official_value("opponent", "total", "elixir")
    team_leak = official_value("team", "leaked", "elixir")
    opp_leak = official_value("opponent", "leaked", "elixir")
    features.extend(
        [
            team_total,
            opp_total,
            team_total - opp_total,
            team_leak,
            opp_leak,
            team_leak - opp_leak,
            official_value("team", "troop", "elixir")
            - official_value("opponent", "troop", "elixir"),
            official_value("team", "spell", "elixir")
            - official_value("opponent", "spell", "elixir"),
            official_value("team", "building", "elixir")
            - official_value("opponent", "building", "elixir"),
            official_value("team", "total", "count")
            - official_value("opponent", "total", "count"),
            float(
                sum(card in SPELL_CARDS for card in battle.team_deck)
                - sum(card in SPELL_CARDS for card in battle.opponent_deck)
            ),
            float(
                sum(card in WIN_CONDITIONISH for card in battle.team_deck)
                - sum(card in WIN_CONDITIONISH for card in battle.opponent_deck)
            ),
        ]
    )

    play_team /= max(play_team.sum(), 1.0)
    play_opp /= max(play_opp.sum(), 1.0)
    late_team /= max(late_team.sum(), 1.0)
    late_opp /= max(late_opp.sum(), 1.0)
    return np.concatenate(
        [
            np.asarray(features, dtype=np.float64),
            deck_team - deck_opp,
            play_team - play_opp,
            late_team - late_opp,
        ]
    )


def swap_battle_perspective(battle: BattleExample) -> BattleExample:
    """Rotate a battle and relabel it from the other player's perspective."""
    events = []
    for event in battle.events:
        swapped = dict(event)
        swapped["side"] = "opponent" if event["side"] == "team" else "team"
        swapped["x"] = 18000 - int(event["x"])
        swapped["y"] = 32000 - int(event["y"])
        events.append(swapped)
    return BattleExample(
        battle_id=battle.battle_id,
        team_deck=battle.opponent_deck,
        opponent_deck=battle.team_deck,
        team_wins=1 - battle.team_wins,
        events=tuple(events),
    )


def _swapped_official_stats(
    battle_id: str, official: dict[str, Any]
) -> dict[str, Any]:
    stats = official.get(battle_id)
    if not stats:
        return {}
    return {
        battle_id: {
            "team": stats.get("opponent"),
            "opponent": stats.get("team"),
        }
    }


def _feature_matrix(
    battles: list[BattleExample],
    costs: dict[str, int],
    card_index: dict[str, int],
    official: dict[str, Any],
    swap_sides: bool = False,
) -> np.ndarray:
    rows = []
    for battle in battles:
        if swap_sides:
            rows.append(
                extract_tabular_features(
                    swap_battle_perspective(battle),
                    costs,
                    card_index,
                    _swapped_official_stats(battle.battle_id, official),
                )
            )
        else:
            rows.append(extract_tabular_features(battle, costs, card_index, official))
    return np.stack(rows)


def _symmetric_probabilities(
    model: Any, features: np.ndarray, swapped_features: np.ndarray
) -> np.ndarray:
    direct = model.predict_proba(features)[:, 1]
    reversed_probability = 1.0 - model.predict_proba(swapped_features)[:, 1]
    return (direct + reversed_probability) / 2.0


def _auc_binary(labels: np.ndarray, probs: np.ndarray) -> float:
    labels = labels.astype(np.int64)
    if labels.min() == labels.max():
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


def train_winner_tabular(
    input_dir: str | Path = "data/raw",
    output_dir: str | Path = "models/winner_predictor",
    card_costs_path: str | Path = "data/card_costs.json",
    min_card_plays: int = 12,
    seed: int = 42,
    extra_trees: int = 100,
) -> dict[str, Any]:
    from sklearn.ensemble import ExtraTreesClassifier, HistGradientBoostingClassifier
    from sklearn.isotonic import IsotonicRegression
    from sklearn.metrics import log_loss

    from .winner_visuals import (
        CORRECTNESS_THRESHOLDS,
        area_under_risk_coverage,
        save_confidence_plot,
        save_training_curve_video,
        selective_curve,
    )

    started = time.time()
    output = Path(output_dir)
    output.mkdir(parents=True, exist_ok=True)

    battles = collect_battles(input_dir, min_card_plays=min_card_plays)
    if len(battles) < 50:
        raise RuntimeError(
            f"Need at least 50 usable battles with clear winners; found {len(battles)}"
        )
    official = build_official_elixir_cache(input_dir)
    train_battles, val_battles, test_battles = split_battles(battles, seed=seed)
    costs = load_card_costs(card_costs_path)
    card_index = _build_card_index(train_battles)

    x_train = _feature_matrix(train_battles, costs, card_index, official)
    x_train_swapped = _feature_matrix(
        train_battles, costs, card_index, official, swap_sides=True
    )
    y_train = np.asarray([battle.team_wins for battle in train_battles], dtype=np.int64)
    x_train_augmented = np.concatenate([x_train, x_train_swapped])
    y_train_augmented = np.concatenate([y_train, 1 - y_train])
    x_val = _feature_matrix(val_battles, costs, card_index, official)
    x_val_swapped = _feature_matrix(
        val_battles, costs, card_index, official, swap_sides=True
    )
    y_val = np.asarray([battle.team_wins for battle in val_battles], dtype=np.int64)
    x_test = _feature_matrix(test_battles, costs, card_index, official)
    x_test_swapped = _feature_matrix(
        test_battles, costs, card_index, official, swap_sides=True
    )
    y_test = np.asarray([battle.team_wins for battle in test_battles], dtype=np.int64)

    hgb_config = {
        "max_depth": 10,
        "learning_rate": 0.03,
        "max_iter": 900,
        "l2_regularization": 0.1,
        "min_samples_leaf": 8,
        "random_state": seed,
    }
    # Refit the old learner once to provide an exact, same-split comparison.
    baseline_model = HistGradientBoostingClassifier(**hgb_config)
    baseline_model.fit(x_train, y_train)
    baseline_val_prob = baseline_model.predict_proba(x_val)[:, 1]
    baseline_test_prob = baseline_model.predict_proba(x_test)[:, 1]

    hgb_model = HistGradientBoostingClassifier(**hgb_config)
    hgb_model.fit(x_train_augmented, y_train_augmented)
    hgb_val_prob = _symmetric_probabilities(hgb_model, x_val, x_val_swapped)
    hgb_test_prob = _symmetric_probabilities(hgb_model, x_test, x_test_swapped)

    tree_model = ExtraTreesClassifier(
        n_estimators=extra_trees,
        max_features=1.0,
        min_samples_leaf=2,
        class_weight="balanced",
        n_jobs=-1,
        random_state=seed,
    )
    tree_model.fit(x_train_augmented, y_train_augmented)
    tree_val_prob = _symmetric_probabilities(tree_model, x_val, x_val_swapped)
    tree_test_prob = _symmetric_probabilities(tree_model, x_test, x_test_swapped)

    # Select the blend only on validation data, leaving the test set untouched.
    blend_candidates = np.linspace(0.0, 1.0, 21)
    blend_losses = []
    for weight in blend_candidates:
        candidate = weight * hgb_val_prob + (1.0 - weight) * tree_val_prob
        blend_losses.append(float(log_loss(y_val, candidate)))
    hgb_weight = float(blend_candidates[int(np.argmin(blend_losses))])
    val_prob = hgb_weight * hgb_val_prob + (1.0 - hgb_weight) * tree_val_prob
    test_prob = hgb_weight * hgb_test_prob + (1.0 - hgb_weight) * tree_test_prob
    val_acc = float(((val_prob >= 0.5) == y_val).mean())
    test_acc = float(((test_prob >= 0.5) == y_test).mean())
    val_auc = _auc_binary(y_val, val_prob)
    test_auc = _auc_binary(y_test, test_prob)

    # Confidence has a different objective from winner probability. Select the
    # HGB/tree mixture that best ranks correct predictions on validation AURC,
    # then calibrate that score to P(prediction is correct).
    val_correct = ((val_prob >= 0.5) == y_val).astype(np.int64)
    test_correct = ((test_prob >= 0.5) == y_test).astype(np.int64)
    confidence_aurcs = []
    for weight in blend_candidates:
        score = np.abs(
            weight * hgb_val_prob + (1.0 - weight) * tree_val_prob - 0.5
        )
        confidence_aurcs.append(area_under_risk_coverage(val_correct, score))
    confidence_hgb_weight = float(
        blend_candidates[int(np.argmin(confidence_aurcs))]
    )
    val_confidence_score = np.abs(
        confidence_hgb_weight * hgb_val_prob
        + (1.0 - confidence_hgb_weight) * tree_val_prob
        - 0.5
    )
    test_confidence_score = np.abs(
        confidence_hgb_weight * hgb_test_prob
        + (1.0 - confidence_hgb_weight) * tree_test_prob
        - 0.5
    )
    confidence_calibrator = IsotonicRegression(
        out_of_bounds="clip", y_min=0.5, y_max=1.0
    )
    confidence_calibrator.fit(val_confidence_score, val_correct)
    test_correct_probability = confidence_calibrator.predict(test_confidence_score)

    baseline_val_correct = (
        (baseline_val_prob >= 0.5) == y_val
    ).astype(np.int64)
    baseline_test_correct = (
        (baseline_test_prob >= 0.5) == y_test
    ).astype(np.int64)
    baseline_calibrator = IsotonicRegression(
        out_of_bounds="clip", y_min=0.5, y_max=1.0
    )
    baseline_calibrator.fit(
        np.abs(baseline_val_prob - 0.5), baseline_val_correct
    )
    baseline_test_correct_probability = baseline_calibrator.predict(
        np.abs(baseline_test_prob - 0.5)
    )
    baseline_curve = selective_curve(
        baseline_test_correct,
        baseline_test_correct_probability,
        CORRECTNESS_THRESHOLDS,
    )
    improved_curve = selective_curve(
        test_correct, test_correct_probability, CORRECTNESS_THRESHOLDS
    )
    raw_margin_aurc = area_under_risk_coverage(
        test_correct, np.abs(test_prob - 0.5)
    )
    selected_confidence_aurc = area_under_risk_coverage(
        test_correct, test_confidence_score
    )

    # Record the exact cumulative forest predictions for the training animation.
    checkpoints = (
        set(range(1, extra_trees + 1))
        if extra_trees <= 150
        else {1, 2, 5, 10, extra_trees, *range(20, extra_trees + 1, 10)}
    )
    direct_sum = np.zeros(len(y_test), dtype=np.float64)
    swapped_sum = np.zeros(len(y_test), dtype=np.float64)
    training_stages = []
    for tree_count, estimator in enumerate(tree_model.estimators_, start=1):
        direct_sum += estimator.predict_proba(x_test)[:, 1]
        swapped_sum += estimator.predict_proba(x_test_swapped)[:, 1]
        if tree_count not in checkpoints:
            continue
        tree_stage_prob = (
            direct_sum / tree_count + 1.0 - swapped_sum / tree_count
        ) / 2.0
        stage_prob = (
            hgb_weight * hgb_test_prob + (1.0 - hgb_weight) * tree_stage_prob
        )
        stage_correct = ((stage_prob >= 0.5) == y_test).astype(np.int64)
        stage_confidence_score = np.abs(
            confidence_hgb_weight * hgb_test_prob
            + (1.0 - confidence_hgb_weight) * tree_stage_prob
            - 0.5
        )
        stage_correct_probability = confidence_calibrator.predict(
            stage_confidence_score
        )
        training_stages.append(
            {
                "trees": tree_count,
                "accuracy": float(stage_correct.mean()),
                "auc": _auc_binary(y_test, stage_prob),
                "aurc": area_under_risk_coverage(
                    stage_correct, stage_confidence_score
                ),
                "confidence_curve": selective_curve(
                    stage_correct,
                    stage_correct_probability,
                    CORRECTNESS_THRESHOLDS,
                ),
            }
        )

    durations = np.asarray(
        [battle.events[-1]["seconds"] for battle in test_battles], dtype=np.float64
    )
    by_duration = []
    for low, high in ((0, 120), (120, 180), (180, 240), (240, 300), (300, 500)):
        mask = (durations >= low) & (durations < high)
        if mask.sum() < 20:
            continue
        by_duration.append(
            {
                "duration": f"[{low},{high})",
                "n": int(mask.sum()),
                "accuracy": float(((test_prob[mask] >= 0.5) == y_test[mask]).mean()),
            }
        )

    artifact = {
        "models": {
            "hist_gradient_boosting": hgb_model,
            "extra_trees": tree_model,
        },
        "hgb_weight": hgb_weight,
        "confidence_hgb_weight": confidence_hgb_weight,
        "confidence_calibrator": confidence_calibrator,
        "symmetric_inference": True,
        "card_index": card_index,
        "seed": seed,
        "extra_trees": extra_trees,
        "feature_version": 5,
    }
    artifact_path = output / "hgb_ensemble.pkl"
    with artifact_path.open("wb") as handle:
        pickle.dump(artifact, handle)

    report = {
        "seconds": round(time.time() - started, 1),
        "battles_total": len(battles),
        "feature_dim": int(x_train.shape[1]),
        "splits": [
            summarize_split("train", train_battles),
            summarize_split("val", val_battles),
            summarize_split("test", test_battles),
        ],
        "baseline": {
            "val": {
                "acc": float(((baseline_val_prob >= 0.5) == y_val).mean()),
                "auc": _auc_binary(y_val, baseline_val_prob),
            },
            "test": {
                "acc": float(((baseline_test_prob >= 0.5) == y_test).mean()),
                "auc": _auc_binary(y_test, baseline_test_prob),
            },
        },
        "blend": {
            "hgb_weight": hgb_weight,
            "extra_trees_weight": 1.0 - hgb_weight,
            "selection": "minimum validation log-loss over weights 0.00..1.00",
        },
        "confidence": {
            "score_hgb_weight": confidence_hgb_weight,
            "score_extra_trees_weight": 1.0 - confidence_hgb_weight,
            "selection": "minimum validation area under the risk-coverage curve",
            "calibration": "validation-fit isotonic P(prediction is correct)",
            "raw_margin_test_aurc": raw_margin_aurc,
            "selected_score_test_aurc": selected_confidence_aurc,
            "relative_aurc_reduction": (
                raw_margin_aurc - selected_confidence_aurc
            )
            / raw_margin_aurc,
        },
        "val": {
            "acc": val_acc,
            "auc": val_auc,
            "log_loss": float(log_loss(y_val, val_prob)),
            "n": int(len(y_val)),
        },
        "test": {
            "acc": test_acc,
            "auc": test_auc,
            "log_loss": float(log_loss(y_test, test_prob)),
            "n": int(len(y_test)),
        },
        "test_by_duration": by_duration,
        "confidence_curve": improved_curve,
        "baseline_confidence_curve": baseline_curve,
        "training_stages": [
            {
                "trees": stage["trees"],
                "accuracy": stage["accuracy"],
                "auc": stage["auc"],
                "aurc": stage["aurc"],
            }
            for stage in training_stages
        ],
        "checkpoint": str(artifact_path),
        "notes": (
            "Full-game symmetric ensemble from placements and official elixir/leak "
            "tables. Training-time perspective augmentation and test-time averaging "
            "remove arbitrary team/opponent orientation. The diverse Extra Trees "
            "component replaces seven deterministic copies of the same HGB model."
        ),
    }

    confidence_json_path = output / "accuracy_vs_confidence.json"
    with confidence_json_path.open("w", encoding="utf-8") as handle:
        json.dump(
            {
                "confidence_definition": "estimated P(prediction is correct)",
                "baseline": baseline_curve,
                "improved": improved_curve,
            },
            handle,
            indent=2,
        )
    stages_path = output / "confidence_training_stages.json"
    with stages_path.open("w", encoding="utf-8") as handle:
        json.dump(training_stages, handle, indent=2)

    plot_path = output / "accuracy_vs_confidence.png"
    video_path = output / "accuracy_vs_confidence_training.mp4"
    try:
        save_confidence_plot(baseline_curve, improved_curve, plot_path)
        save_training_curve_video(baseline_curve, training_stages, video_path)
        report["confidence_plot"] = str(plot_path)
        report["confidence_video"] = str(video_path)
    except Exception as exc:  # pragma: no cover - visualization is best-effort
        report["visualization_error"] = str(exc)

    report_path = output / "hgb_report.json"
    with report_path.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    print(
        json.dumps(
            {
                "baseline_test": report["baseline"]["test"],
                "test": report["test"],
                "blend": report["blend"],
                "confidence_curve": improved_curve[:5],
            },
            indent=2,
        )
    )
    print(f"Wrote {report_path}")
    return report
