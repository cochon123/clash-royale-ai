"""Offline matchup stress test: empirical deck edges vs policy-vs-policy rollouts."""

from __future__ import annotations

import json
import pickle
import random
from collections import Counter, defaultdict
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import numpy as np
import torch

from .policy_dataset import encode_policy_sample
from .policy_infer import load_policy
from .policy_model import PolicyBC
from .winner_dataset import (
    DEFAULT_ELIXIR_COST,
    WIN_CONDITIONISH,
    BattleExample,
    CardVocab,
    collect_battles,
    load_card_costs,
    split_battles,
)
from .winner_tabular import extract_tabular_features, swap_battle_perspective


@dataclass(frozen=True)
class MatchupSpec:
    favorite: str
    underdog: str
    empirical_n: int
    empirical_fav_wr: float
    deck_pairs: tuple[tuple[tuple[str, ...], tuple[str, ...]], ...]


def _wincon_freq(battles: list[BattleExample]) -> Counter:
    freq: Counter = Counter()
    for battle in battles:
        for deck in (battle.team_deck, battle.opponent_deck):
            for card in deck:
                if card in WIN_CONDITIONISH:
                    freq[card] += 1
    return freq


def archetype(deck: tuple[str, ...], wc_freq: Counter) -> str:
    wcs = [card for card in deck if card in WIN_CONDITIONISH]
    if not wcs:
        return "no-wincon"
    return max(wcs, key=lambda card: wc_freq[card])


def discover_matchups(
    battles: list[BattleExample],
    min_n: int = 60,
    top_k: int = 8,
) -> list[MatchupSpec]:
    wc_freq = _wincon_freq(battles)
    stats: dict[tuple[str, str], dict[str, int]] = defaultdict(
        lambda: {"n": 0, "a_wins": 0}
    )
    examples: dict[tuple[str, str], list[tuple[tuple[str, ...], tuple[str, ...]]]] = (
        defaultdict(list)
    )

    for battle in battles:
        a = archetype(battle.team_deck, wc_freq)
        b = archetype(battle.opponent_deck, wc_freq)
        if a == b:
            continue
        if a < b:
            pair = (a, b)
            a_won = battle.team_wins == 1
            decks = (battle.team_deck, battle.opponent_deck)
        else:
            pair = (b, a)
            a_won = battle.team_wins == 0
            decks = (battle.opponent_deck, battle.team_deck)
        stats[pair]["n"] += 1
        stats[pair]["a_wins"] += int(a_won)
        if len(examples[pair]) < 120:
            examples[pair].append(decks)

    ranked: list[tuple[float, MatchupSpec]] = []
    for (arch_a, arch_b), s in stats.items():
        if s["n"] < min_n:
            continue
        rate_a = s["a_wins"] / s["n"]
        if rate_a >= 0.5:
            fav, und, fav_wr = arch_a, arch_b, rate_a
            pairs = tuple(examples[(arch_a, arch_b)])
        else:
            fav, und, fav_wr = arch_b, arch_a, 1.0 - rate_a
            pairs = tuple((opp, team) for team, opp in examples[(arch_a, arch_b)])
        edge = abs(fav_wr - 0.5) * (s["n"] ** 0.5)
        ranked.append(
            (
                edge,
                MatchupSpec(
                    favorite=fav,
                    underdog=und,
                    empirical_n=s["n"],
                    empirical_fav_wr=fav_wr,
                    deck_pairs=pairs,
                ),
            )
        )
    ranked.sort(key=lambda item: -item[0])
    return [spec for _, spec in ranked[:top_k]]


def _bootstrap_opening(
    team_deck: tuple[str, ...],
    opponent_deck: tuple[str, ...],
    costs: dict[str, int],
    rng: random.Random,
    n_events: int = 10,
) -> list[dict[str, Any]]:
    """Cheap alternating cycle plays so the policy has a legal history."""
    events: list[dict[str, Any]] = []
    seconds = 5.0
    hands = {
        "team": list(team_deck[:4]),
        "opponent": list(opponent_deck[:4]),
    }
    queues = {
        "team": list(team_deck[4:]),
        "opponent": list(opponent_deck[4:]),
    }
    elixir = {"team": 5.0, "opponent": 5.0}
    side = "team"
    while len(events) < n_events:
        seconds += rng.uniform(1.2, 2.8)
        for key in elixir:
            elixir[key] = min(10.0, elixir[key] + 1.0)
        affordable = [
            card
            for card in hands[side]
            if float(costs.get(card, DEFAULT_ELIXIR_COST)) <= elixir[side] + 1e-9
        ]
        if not affordable:
            side = "opponent" if side == "team" else "team"
            continue
        # Prefer cheapest to avoid dumping.
        card = min(affordable, key=lambda c: costs.get(c, DEFAULT_ELIXIR_COST))
        cost = float(costs.get(card, DEFAULT_ELIXIR_COST))
        elixir[side] -= cost
        hands[side].remove(card)
        if queues[side]:
            hands[side].append(queues[side].pop(0))
        queues[side].append(card)
        y = rng.randint(2000, 14000) if side == "team" else rng.randint(18000, 30000)
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


@torch.no_grad()
def policy_vs_policy_game(
    model: PolicyBC,
    vocab: CardVocab,
    costs: dict[str, int],
    team_deck: tuple[str, ...],
    opponent_deck: tuple[str, ...],
    device: torch.device,
    rng: random.Random,
    warmup_events: int = 10,
    max_new_events: int = 50,
    temperature: float = 0.85,
    max_context: int = 64,
    threat_dim: int = 0,
    think_steps: int | None = None,
) -> BattleExample:
    events = _bootstrap_opening(
        team_deck, opponent_deck, costs, rng, n_events=warmup_events
    )
    seconds = float(events[-1]["seconds"])
    from .policy_infer import predict_next_action

    placement_decode = (
        "sample"
        if getattr(model, "placement_card_mode", "soft") == "selected"
        else ("argmax" if getattr(model, "placement_mode", "xy") == "heatmap" else "expected")
    )

    for _ in range(max_new_events):
        current = BattleExample(
            battle_id="matchup-probe",
            team_deck=team_deck,
            opponent_deck=opponent_deck,
            team_wins=0,
            events=tuple(events),
        )
        predictions = []
        for side in ("team", "opponent"):
            pred = predict_next_action(
                model, vocab, costs, current, device,
                acting_side=side, temperature=temperature, slot_decode="sample",
                max_context=max_context, threat_dim=threat_dim,
                placement_decode=placement_decode, placement_temperature=0.6,
                placement_top_k=5, think_steps=think_steps,
                now_seconds=seconds, rng=rng,
            )
            predictions.append((side, pred))
        next_side, pred = min(
            predictions, key=lambda row: float(row[1]["delay_seconds"])
        )
        dt = float(pred["delay_seconds"])
        seconds = min(330.0, seconds + dt)
        events.append(
            {
                "seconds": seconds,
                "side": next_side,
                "event_type": pred["event_type"],
                "card": pred["card"],
                "x": pred["x"],
                "y": pred["y"],
            }
        )

    return BattleExample(
        battle_id="matchup-game",
        team_deck=team_deck,
        opponent_deck=opponent_deck,
        team_wins=0,
        events=tuple(events),
    )


def _score_team_win_prob(
    winner_artifact: dict[str, Any],
    battles: list[BattleExample],
    costs: dict[str, int],
) -> np.ndarray:
    card_index = winner_artifact["card_index"]
    hgb = winner_artifact["models"]["hist_gradient_boosting"]
    trees = winner_artifact["models"]["extra_trees"]
    w = float(winner_artifact["hgb_weight"])
    official: dict[str, Any] = {}
    feats = np.stack(
        [extract_tabular_features(b, costs, card_index, official) for b in battles]
    )
    swapped = np.stack(
        [
            extract_tabular_features(
                swap_battle_perspective(b), costs, card_index, official
            )
            for b in battles
        ]
    )
    direct = w * hgb.predict_proba(feats)[:, 1] + (1.0 - w) * trees.predict_proba(feats)[:, 1]
    reverse = 1.0 - (
        w * hgb.predict_proba(swapped)[:, 1]
        + (1.0 - w) * trees.predict_proba(swapped)[:, 1]
    )
    return (direct + reverse) / 2.0


def evaluate_matchups(
    input_dir: str | Path = "data/raw",
    policy_dir: str | Path = "models/policy_bc",
    winner_dir: str | Path = "models/winner_predictor",
    card_costs_path: str | Path = "data/card_costs.json",
    output_path: str | Path = "reports/matchup_eval.json",
    games_per_matchup: int = 48,
    top_k: int = 6,
    min_n: int = 60,
    seed: int = 42,
    device_name: str | None = None,
) -> dict[str, Any]:
    rng = random.Random(seed)
    print("Loading battles ...", flush=True)
    battles = collect_battles(input_dir)
    # Discover matchups on full corpus; roll decks only from train to limit leakage
    # into the winner model test set (winner was split with seed 42 too).
    train, _val, _test = split_battles(battles, seed=42)
    matchups = discover_matchups(train, min_n=min_n, top_k=top_k)
    if not matchups:
        raise RuntimeError("No matchups found with the current thresholds")

    costs = load_card_costs(card_costs_path)
    model, vocab, cfg, device = load_policy(policy_dir, device_name=device_name)
    with Path(winner_dir, "hgb_ensemble.pkl").open("rb") as handle:
        winner = pickle.load(handle)

    print(
        f"Evaluating {len(matchups)} matchups × {games_per_matchup} games on {device}",
        flush=True,
    )
    rows: list[dict[str, Any]] = []
    for spec in matchups:
        games: list[BattleExample] = []
        for i in range(games_per_matchup):
            team_deck, opp_deck = rng.choice(spec.deck_pairs)
            # Half the games swap seats so "favorite" isn't always team.
            if i % 2 == 0:
                fav_is_team = True
                battle = policy_vs_policy_game(
                    model,
                    vocab,
                    costs,
                    team_deck,
                    opp_deck,
                    device,
                    random.Random(rng.randint(0, 10**9)),
                    max_context=int(cfg.get("max_context", 64)),
                    threat_dim=int(cfg.get("threat_dim", 0)),
                )
            else:
                fav_is_team = False
                battle = policy_vs_policy_game(
                    model,
                    vocab,
                    costs,
                    opp_deck,
                    team_deck,
                    device,
                    random.Random(rng.randint(0, 10**9)),
                    max_context=int(cfg.get("max_context", 64)),
                    threat_dim=int(cfg.get("threat_dim", 0)),
                )
            games.append(battle)
            # stash orientation on the object via battle_id
            battle_id = f"{spec.favorite}-vs-{spec.underdog}-{'T' if fav_is_team else 'O'}-{i}"
            games[-1] = BattleExample(
                battle_id=battle_id,
                team_deck=battle.team_deck,
                opponent_deck=battle.opponent_deck,
                team_wins=1 if fav_is_team else 0,  # marker: 1 => fav is team
                events=battle.events,
            )

        probs_team = _score_team_win_prob(winner, games, costs)
        fav_probs = []
        for battle, p_team in zip(games, probs_team):
            fav_is_team = battle.team_wins == 1
            fav_probs.append(float(p_team if fav_is_team else 1.0 - p_team))
        fav_probs_arr = np.asarray(fav_probs, dtype=np.float64)
        policy_fav_wr = float((fav_probs_arr >= 0.5).mean())
        mean_p = float(fav_probs_arr.mean())
        # Does the model preserve the edge direction?
        preserves = policy_fav_wr >= 0.5
        # Stronger: within 10pp of empirical or still clearly fav-favored
        agrees_strongly = (
            abs(policy_fav_wr - spec.empirical_fav_wr) <= 0.12
            or (policy_fav_wr >= 0.55 and spec.empirical_fav_wr >= 0.55)
        )
        row = {
            "favorite": spec.favorite,
            "underdog": spec.underdog,
            "empirical_n": spec.empirical_n,
            "empirical_fav_wr": spec.empirical_fav_wr,
            "games": games_per_matchup,
            "policy_fav_wr": policy_fav_wr,
            "policy_mean_P_fav": mean_p,
            "policy_fav_wr_ci95": float(
                1.96 * np.sqrt(max(policy_fav_wr * (1 - policy_fav_wr), 1e-9) / games_per_matchup)
            ),
            "delta_wr": policy_fav_wr - spec.empirical_fav_wr,
            "preserves_favorite": preserves,
            "agrees_strongly": agrees_strongly,
            "mean_events": float(np.mean([len(g.events) for g in games])),
        }
        rows.append(row)
        print(
            f"{spec.favorite} > {spec.underdog}: empirical={spec.empirical_fav_wr:.1%} "
            f"policy={policy_fav_wr:.1%} (meanP={mean_p:.3f}) "
            f"{'OK' if preserves else 'FLIPPED'}",
            flush=True,
        )

    preserve_rate = float(np.mean([r["preserves_favorite"] for r in rows]))
    strong_rate = float(np.mean([r["agrees_strongly"] for r in rows]))
    mean_abs_delta = float(np.mean([abs(r["delta_wr"]) for r in rows]))
    report = {
        "created_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "model_name": "policy-bc-v2",
        "judge": "winner-predictor-hgb-symmetric",
        "setup": {
            "games_per_matchup": games_per_matchup,
            "top_k": top_k,
            "min_empirical_n": min_n,
            "warmup_events": 10,
            "policy_events": 50,
            "temperature": 0.85,
            "archetype": "primary win-condition by corpus frequency",
            "seat_balanced": True,
            "note": (
                "Both seats use the same policy weights. Favorite/underdog are "
                "wincon archetypes mined from training battles. Winners are "
                "judged by the offline winner model (no live play)."
            ),
        },
        "summary": {
            "matchups": len(rows),
            "preserve_favorite_rate": preserve_rate,
            "strong_agreement_rate": strong_rate,
            "mean_abs_delta_wr": mean_abs_delta,
        },
        "matchups": rows,
        "verdict": (
            "Policy self-play mostly preserves empirical matchup favorites."
            if preserve_rate >= 0.67
            else "Policy self-play often flips or washes out empirical matchup edges."
        ),
    }
    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)
    with out.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    print(json.dumps(report["summary"], indent=2))
    print(report["verdict"])
    print(f"Wrote {out}")
    return report


DEFAULT_LINEAGE_POLICIES: tuple[tuple[str, str, str], ...] = (
    ("v4", "models/policy_bc_v4", "#70a1ff"),
    ("v4.1", "models/policy_bc_v4.1", "#ffca63"),
    ("v4.2", "models/policy_bc_v4.2_full", "#70e1b1"),
    ("v4.3", "models/policy_bc_v4.3", "#e8f58b"),
    ("v4.4", "models/policy_bc_v4.4", "#38bdf8"),
    ("v5", "models/policy_bc_v5", "#f472b6"),
)


def _policy_think_steps(cfg: dict[str, Any], report: dict[str, Any]) -> int | None:
    """Use the checkpoint's eval think depth when the model supports thinking."""
    max_think = int(cfg.get("max_think_steps", 0) or 0)
    if max_think <= 0:
        return None
    compute = report.get("compute") or {}
    eval_think = compute.get("eval_think_steps")
    if eval_think is None:
        eval_think = cfg.get("eval_think_steps", max_think)
    return max(0, min(int(eval_think), max_think))


def _fmt_eta(seconds: float) -> str:
    seconds = max(0.0, float(seconds))
    if seconds < 60:
        return f"{seconds:.0f}s"
    if seconds < 3600:
        return f"{seconds / 60:.1f}m"
    return f"{seconds / 3600:.2f}h"


def _score_matchup_games(
    winner: dict[str, Any],
    games: list[BattleExample],
    fav_is_team: list[bool],
    costs: dict[str, int],
    empirical_fav_wr: float,
) -> dict[str, Any]:
    probs_team = _score_team_win_prob(winner, games, costs)
    fav_probs = [
        float(p if fav else 1.0 - p) for p, fav in zip(probs_team, fav_is_team)
    ]
    fav_probs_arr = np.asarray(fav_probs, dtype=np.float64)
    n = len(fav_probs_arr)
    policy_fav_wr = float((fav_probs_arr >= 0.5).mean()) if n else 0.0
    mean_p = float(fav_probs_arr.mean()) if n else 0.0
    delta = policy_fav_wr - empirical_fav_wr
    preserves = policy_fav_wr >= 0.5
    agrees_strongly = abs(delta) <= 0.12 or (
        policy_fav_wr >= 0.55 and empirical_fav_wr >= 0.55
    )
    return {
        "games": n,
        "policy_fav_wr": policy_fav_wr,
        "policy_mean_P_fav": mean_p,
        "policy_fav_wr_ci95": float(
            1.96 * np.sqrt(max(policy_fav_wr * (1 - policy_fav_wr), 1e-9) / max(n, 1))
        ),
        "delta_wr": delta,
        "preserves_favorite": preserves,
        "agrees_strongly": agrees_strongly,
        "mean_events": float(np.mean([len(g.events) for g in games])) if games else 0.0,
    }


def evaluate_matchup_lineage(
    input_dir: str | Path = "data/raw",
    policy_specs: list[tuple[str, str, str]] | None = None,
    winner_dir: str | Path = "models/winner_predictor",
    card_costs_path: str | Path = "data/card_costs.json",
    output_path: str | Path = "reports/matchup_lineage.json",
    html_output: str | Path | None = "reports/matchup_lineage.html",
    games_per_matchup: int = 80,
    top_k: int = 8,
    min_n: int = 80,
    seed: int = 42,
    device_name: str | None = None,
    warmup_events: int = 10,
    max_new_events: int = 50,
    temperature: float = 0.85,
) -> dict[str, Any]:
    """Mine shared wincon matchups, run each policy on the same schedule, report.

    Progress lines include done/total and ETA so long lineage runs are monitorable.
    """
    import time

    from .matchup_lineage_report import write_matchup_lineage_report

    specs = list(policy_specs or DEFAULT_LINEAGE_POLICIES)
    specs = [(label, path, color) for label, path, color in specs if Path(path).exists()]
    if not specs:
        raise RuntimeError("No policy checkpoints found for lineage matchup eval")

    rng = random.Random(seed)
    print("Loading battles ...", flush=True)
    battles = collect_battles(input_dir)
    train, _val, _test = split_battles(battles, seed=42)
    matchups = discover_matchups(train, min_n=min_n, top_k=top_k)
    if not matchups:
        raise RuntimeError("No matchups found with the current thresholds")

    costs = load_card_costs(card_costs_path)
    with Path(winner_dir, "hgb_ensemble.pkl").open("rb") as handle:
        winner = pickle.load(handle)

    # Shared schedule: identical decks/seeds/seats for every policy.
    schedule: list[dict[str, Any]] = []
    matchup_meta: list[dict[str, Any]] = []
    for mi, spec in enumerate(matchups):
        matchup_meta.append(
            {
                "id": f"{spec.favorite}>{spec.underdog}",
                "favorite": spec.favorite,
                "underdog": spec.underdog,
                "label": f"{spec.favorite} > {spec.underdog}",
                "empirical_n": spec.empirical_n,
                "empirical_fav_wr": spec.empirical_fav_wr,
            }
        )
        for i in range(games_per_matchup):
            team_deck, opp_deck = rng.choice(spec.deck_pairs)
            fav_is_team = i % 2 == 0
            schedule.append(
                {
                    "matchup_index": mi,
                    "game_index": i,
                    "team_deck": team_deck if fav_is_team else opp_deck,
                    "opp_deck": opp_deck if fav_is_team else team_deck,
                    "fav_is_team": fav_is_team,
                    "seed": rng.randint(0, 10**9),
                }
            )

    total_games = len(specs) * len(schedule)
    print(
        f"Lineage matchups: {len(matchups)} · policies: {len(specs)} · "
        f"games/policy: {len(schedule)} · total: {total_games}",
        flush=True,
    )
    for mi, meta in enumerate(matchup_meta):
        print(
            f"  [{mi+1}/{len(matchup_meta)}] {meta['label']} "
            f"human={meta['empirical_fav_wr']:.1%} n={meta['empirical_n']}",
            flush=True,
        )

    models_out: list[dict[str, Any]] = []
    done = 0
    t0 = time.time()
    progress_path = Path(output_path).with_name(
        Path(output_path).stem + "_progress.jsonl"
    )
    if progress_path.exists():
        progress_path.unlink()

    for label, path, color in specs:
        model, vocab, cfg, device = load_policy(path, device_name=device_name)
        report_json: dict[str, Any] = {}
        report_path = Path(path) / "report.json"
        if report_path.exists():
            with report_path.open(encoding="utf-8") as handle:
                report_json = json.load(handle)
        think_steps = _policy_think_steps(cfg, report_json)
        print(
            f"\n== {label} ({path}) think={think_steps if think_steps is not None else 0} "
            f"on {device} ==",
            flush=True,
        )

        by_matchup: dict[int, list[BattleExample]] = defaultdict(list)
        fav_flags: dict[int, list[bool]] = defaultdict(list)
        policy_t0 = time.time()

        for step_i, job in enumerate(schedule):
            battle = policy_vs_policy_game(
                model,
                vocab,
                costs,
                job["team_deck"],
                job["opp_deck"],
                device,
                random.Random(job["seed"]),
                warmup_events=warmup_events,
                max_new_events=max_new_events,
                temperature=temperature,
                max_context=int(cfg.get("max_context", 64)),
                threat_dim=int(cfg.get("threat_dim", 0) or 0),
                think_steps=think_steps,
            )
            mi = int(job["matchup_index"])
            by_matchup[mi].append(battle)
            fav_flags[mi].append(bool(job["fav_is_team"]))
            done += 1
            if (step_i + 1) % 16 == 0 or (step_i + 1) == len(schedule):
                elapsed = time.time() - t0
                rate = done / max(elapsed, 1e-6)
                remaining = (total_games - done) / max(rate, 1e-6)
                print(
                    f"{label}\t{done}/{total_games}\t"
                    f"policy {step_i+1}/{len(schedule)}\t"
                    f"ETA {_fmt_eta(remaining)}\t"
                    f"{rate:.2f} games/s",
                    flush=True,
                )
                with progress_path.open("a", encoding="utf-8") as handle:
                    handle.write(
                        json.dumps(
                            {
                                "ts": datetime.now(timezone.utc).strftime(
                                    "%Y-%m-%dT%H:%M:%SZ"
                                ),
                                "policy": label,
                                "done": done,
                                "total": total_games,
                                "eta_seconds": remaining,
                                "games_per_sec": rate,
                            }
                        )
                        + "\n"
                    )

        rows = []
        for mi, meta in enumerate(matchup_meta):
            scored = _score_matchup_games(
                winner,
                by_matchup[mi],
                fav_flags[mi],
                costs,
                meta["empirical_fav_wr"],
            )
            row = {**meta, **scored}
            rows.append(row)
            print(
                f"  {meta['label']}: human={meta['empirical_fav_wr']:.1%} "
                f"AI={scored['policy_fav_wr']:.1%} "
                f"Δ={scored['delta_wr']:+.1%} "
                f"{'OK' if scored['preserves_favorite'] else 'FLIP'}",
                flush=True,
            )

        deltas = [abs(r["delta_wr"]) for r in rows]
        mean_abs = float(np.mean(deltas)) if deltas else 0.0
        # Bootstrap mean |Δ| over matchups (lightweight uncertainty).
        rs = np.random.default_rng(seed + 17)
        boot = []
        arr = np.asarray(deltas, dtype=np.float64)
        for _ in range(800):
            sample = arr[rs.integers(0, len(arr), size=len(arr))]
            boot.append(float(sample.mean()))
        boot_arr = np.asarray(boot)
        models_out.append(
            {
                "id": label,
                "policyId": report_json.get("model_name") or Path(path).name,
                "modelDir": path,
                "color": color,
                "thinkSteps": think_steps if think_steps is not None else 0,
                "seconds": time.time() - policy_t0,
                "matchups": rows,
                "meanAbsDelta": mean_abs,
                "meanAbsDeltaCI": [
                    float(np.percentile(boot_arr, 2.5)),
                    float(np.percentile(boot_arr, 97.5)),
                ],
                "preserveFavoriteRate": float(
                    np.mean([r["preserves_favorite"] for r in rows])
                ),
                "strongAgreementRate": float(
                    np.mean([r["agrees_strongly"] for r in rows])
                ),
                "meanPolicyFavWr": float(np.mean([r["policy_fav_wr"] for r in rows])),
            }
        )
        # Free GPU memory between policies.
        del model
        if device.type == "cuda":
            torch.cuda.empty_cache()

    best = min(models_out, key=lambda m: m["meanAbsDelta"])
    report = {
        "created_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "model_name": "matchup-lineage",
        "judge": "winner-predictor-hgb-symmetric",
        "seconds": time.time() - t0,
        "setup": {
            "games_per_matchup": games_per_matchup,
            "top_k": top_k,
            "min_empirical_n": min_n,
            "warmup_events": warmup_events,
            "policy_events": max_new_events,
            "temperature": temperature,
            "archetype": "primary win-condition by corpus frequency",
            "seat_balanced": True,
            "shared_schedule": True,
            "policies": [
                {"id": label, "path": path, "color": color}
                for label, path, color in specs
            ],
            "note": (
                "Same wincon matchups and identical deck/seed/seat schedule for every "
                "policy. Each seat uses that policy's weights (self-play). Winners are "
                "judged by the offline winner model — no live Clash Royale."
            ),
        },
        "matchups": matchup_meta,
        "models": models_out,
        "summary": {
            "matchups": len(matchup_meta),
            "policies": len(models_out),
            "games_per_policy": len(schedule),
            "best_mean_abs_delta": best["id"],
            "best_mean_abs_delta_wr": best["meanAbsDelta"],
        },
        "verdict": (
            f"{best['id']} is closest to human matchup edges "
            f"(mean |Δ| = {100*best['meanAbsDelta']:.1f} pp) under shared-schedule "
            f"self-play judged by the winner model."
        ),
        "protocol": (
            f"shared {len(matchup_meta)} matchups × {games_per_matchup} games · "
            "wincon archetypes · seat-balanced self-play"
        ),
        "note": (
            "Δ = AI favorite WR − human favorite WR. Positive means the policy "
            "overstates the empirical edge; negative means it washes or flips it."
        ),
    }

    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)
    with out.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    print(json.dumps(report["summary"], indent=2))
    print(report["verdict"])
    print(f"Wrote {out}")

    if html_output:
        html_path = write_matchup_lineage_report(report, html_output)
        report["html_report"] = str(html_path)
        print(f"Wrote {html_path}")
    return report
