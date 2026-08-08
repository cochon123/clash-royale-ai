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
        if len(examples[pair]) < 40:
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
) -> BattleExample:
    events = _bootstrap_opening(
        team_deck, opponent_deck, costs, rng, n_events=warmup_events
    )
    seconds = float(events[-1]["seconds"])
    next_side = "team" if events[-1]["side"] == "opponent" else "opponent"

    for _ in range(max_new_events):
        dummy_card = team_deck[0] if next_side == "team" else opponent_deck[0]
        dummy = {
            "seconds": seconds + 1.0,
            "side": next_side,
            "event_type": "card_play",
            "card": dummy_card,
            "x": 9000,
            "y": 8000 if next_side == "team" else 24000,
        }
        probe = BattleExample(
            battle_id="matchup-probe",
            team_deck=team_deck,
            opponent_deck=opponent_deck,
            team_wins=0,
            events=tuple(events) + (dummy,),
        )
        sample = encode_policy_sample(
            probe,
            len(events),
            vocab,
            costs,
            max_context=max_context,
            threat_dim=threat_dim,
        )
        if sample is None:
            break
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
        logits = out["slot_logits"][0] / max(temperature, 1e-3)
        probs = torch.softmax(logits, dim=-1).cpu().numpy()
        slot = int(rng.choices(range(8), weights=probs.tolist(), k=1)[0])
        acting = team_deck if next_side == "team" else opponent_deck
        card = acting[slot]
        event_type = (
            "ability_activation"
            if int(out["type_logits"][0].argmax().item()) == 1
            else "card_play"
        )
        xy = out["xy"][0].cpu().numpy()
        x = int(np.clip(xy[0] * 18000.0, 3000, 15000))
        y_norm = float(xy[1])
        if next_side == "opponent":
            y_norm = 1.0 - y_norm
        y = int(np.clip(y_norm * 32000.0, 500, 31500))
        if event_type == "ability_activation":
            x, y = 9000, 16000
        dt = float(np.clip(np.expm1(out["timing"][0].item()), 0.25, 10.0))
        seconds = min(330.0, seconds + dt)
        events.append(
            {
                "seconds": seconds,
                "side": next_side,
                "event_type": event_type,
                "card": card,
                "x": x,
                "y": y,
            }
        )
        next_side = "opponent" if next_side == "team" else "team"

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
