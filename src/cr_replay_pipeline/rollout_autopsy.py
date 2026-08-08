"""Experiment D — Rollout-Collapse Autopsy (realism gate audit).

Offline-only. Tests whether near-zero realism scores on policy rollouts have
one identifiable cause (policy head, hardcoded side alternation, or scorer OOD).
Does NOT claim live-play readiness.
"""

from __future__ import annotations

import argparse
import json
import pickle
import random
import time
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Literal

import numpy as np
import torch

from .policy_dataset import DEFAULT_MAX_CONTEXT, deck_slot_for_card, encode_policy_sample
from .policy_infer import load_policy
from .policy_train import _score_realism
from .realism_generate import TimingPrior
from .realism_train import extract_realism_features
from .rollout_autopsy_report import render_rollout_autopsy_report
from .winner_dataset import BattleExample, collect_battles, load_card_costs, split_battles

AblationMode = Literal[
    "baseline",
    "real_side_order",
    "teacher_forced_slots",
    "greedy_slots",
    "human_like_xy",
    "human_like_delays",
]

COLLAPSE_THRESH = 0.25
HEALTHY_THRESH = 0.75
RECOVERY_THRESH = 0.5
WARMUP_EVENTS = 12
MAX_NEW_EVENTS = 40

# Order matters: first ablation that lifts a collapsed seed above RECOVERY_THRESH
# is the attributed cause. Documented approximations noted in ABLATION_NOTES.
ABLATION_ORDER: tuple[AblationMode, ...] = (
    "real_side_order",
    "teacher_forced_slots",
    "greedy_slots",
    "human_like_xy",
    "human_like_delays",
)

CAUSE_FROM_ABLATION: dict[str, str] = {
    "real_side_order": "hardcoded_alternation",
    "teacher_forced_slots": "policy_slot_head",
    "greedy_slots": "policy_slot_sampling",
    "human_like_xy": "policy_xy_head",
    "human_like_delays": "policy_timing_head",
}

ABLATION_NOTES: dict[str, str] = {
    "real_side_order": (
        "Approximate: continuation sides follow the seed battle's real side "
        "sequence instead of strict team↔opponent alternation; card/xy/timing "
        "still come from the policy."
    ),
    "teacher_forced_slots": (
        "Approximate: when the acting side's next real play is available in the "
        "seed, force that deck slot; otherwise fall back to policy sampling. "
        "XY/timing still from policy; sides stay alternating (baseline harness)."
    ),
    "greedy_slots": "Exact: argmax slot instead of temperature sampling.",
    "human_like_xy": (
        "Approximate: replace policy XY with an empirical same-side placement "
        "sampled from the test corpus; slot/timing still from policy."
    ),
    "human_like_delays": (
        "Approximate: replace policy Δt with TimingPrior samples from train "
        "battles, clipped to [0.2, 12]; slot/xy still from policy."
    ),
}

REALISM_FEATURE_NAMES: tuple[str, ...] = (
    "duration_norm",
    "n_events",
    "n_plays",
    "n_abilities",
    "plays_per_min",
    "gap_mean",
    "gap_std",
    "gap_p10",
    "gap_p50",
    "gap_p90",
    "frac_gap_lt_1",
    "frac_gap_lt_2",
    "frac_gap_gt_8",
    "alt_rate",
    "max_streak",
    "frac_single",
    "frac_double",
    "frac_triple",
    "leak_team",
    "leak_opp",
    "leak_diff",
    "pre_elixir_mean",
    "pre_elixir_std",
    "dump_rate",
    "spend_team",
    "spend_opp",
    "spend_diff",
    "cost_mean",
    "cost_std",
    "cost_delta_mean",
    "cost_delta_std",
    "team_x_mean",
    "team_x_std",
    "team_y_mean",
    "team_y_std",
    "opp_x_mean",
    "opp_x_std",
    "opp_y_mean",
    "opp_y_std",
    "team_deep_rate",
    "opp_deep_rate",
    "team_bridge_rate",
    "opp_bridge_rate",
    "team_back_rate",
    "opp_back_rate",
    "team_left_rate",
    "opp_left_rate",
    "tile_diversity",
    "team_unique_frac",
    "opp_unique_frac",
    "team_spell_rate",
    "opp_spell_rate",
    "team_wincon_rate",
    "opp_wincon_rate",
    "resp_latency_mean",
    "resp_latency_std",
    "resp_rate",
    "pending_deep",
    "team_deck_cost_mean",
    "opp_deck_cost_mean",
    "team_deck_spells",
    "opp_deck_spells",
    "team_deck_wincons",
    "opp_deck_wincons",
    "side_count_imbalance",
    "in_overtime",
    "in_single",
)


def _load_realism_scorer(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return pickle.load(handle)


def _score_stats(scores: list[float]) -> dict[str, float]:
    if not scores:
        return {
            "n": 0,
            "mean": 0.0,
            "median": 0.0,
            "p10": 0.0,
            "p90": 0.0,
            "collapse_rate": 0.0,
            "healthy_rate": 0.0,
        }
    arr = np.asarray(scores, dtype=np.float64)
    return {
        "n": int(len(arr)),
        "mean": float(arr.mean()),
        "median": float(np.median(arr)),
        "p10": float(np.percentile(arr, 10)),
        "p90": float(np.percentile(arr, 90)),
        "collapse_rate": float((arr < COLLAPSE_THRESH).mean()),
        "healthy_rate": float((arr > HEALTHY_THRESH).mean()),
    }


def _histogram(scores: list[float], bins: int = 20) -> dict[str, Any]:
    if not scores:
        edges = np.linspace(0.0, 1.0, bins + 1)
        return {"edges": edges.tolist(), "counts": [0] * bins}
    counts, edges = np.histogram(scores, bins=bins, range=(0.0, 1.0))
    return {"edges": edges.tolist(), "counts": [int(c) for c in counts]}


def truncate_battle(battle: BattleExample, n_events: int) -> BattleExample:
    events = battle.events[:n_events]
    return BattleExample(
        battle_id=battle.battle_id + "-trunc",
        team_deck=battle.team_deck,
        opponent_deck=battle.opponent_deck,
        team_wins=battle.team_wins,
        events=tuple(dict(e) for e in events),
    )


def clip_battle_delays(
    battle: BattleExample, lo: float = 0.2, hi: float = 12.0
) -> BattleExample:
    if not battle.events:
        return battle
    events = [dict(battle.events[0])]
    for prev, cur in zip(battle.events, battle.events[1:]):
        dt = float(cur["seconds"]) - float(prev["seconds"])
        dt = float(np.clip(dt, lo, hi))
        nxt = dict(cur)
        nxt["seconds"] = float(events[-1]["seconds"]) + dt
        events.append(nxt)
    return BattleExample(
        battle_id=battle.battle_id + "-clipdt",
        team_deck=battle.team_deck,
        opponent_deck=battle.opponent_deck,
        team_wins=battle.team_wins,
        events=tuple(events),
    )


def force_strict_alternation(battle: BattleExample) -> BattleExample | None:
    """Interleave sides while preserving within-side order; retime with original gaps.

    Returns None if one side has zero plays (infeasible).
    """
    events = list(battle.events)
    if len(events) < 4:
        return None
    team = [dict(e) for e in events if e["side"] == "team"]
    opp = [dict(e) for e in events if e["side"] == "opponent"]
    if not team or not opp:
        return None

    gaps = [
        max(0.05, float(events[i]["seconds"]) - float(events[i - 1]["seconds"]))
        for i in range(1, len(events))
    ]
    start = events[0]["side"]
    out: list[dict[str, Any]] = []
    ti = oi = 0
    next_side = start
    while ti < len(team) or oi < len(opp):
        if next_side == "team" and ti < len(team):
            out.append(team[ti])
            ti += 1
        elif next_side == "opponent" and oi < len(opp):
            out.append(opp[oi])
            oi += 1
        elif ti < len(team):
            out.append(team[ti])
            ti += 1
        else:
            out.append(opp[oi])
            oi += 1
        next_side = "opponent" if next_side == "team" else "team"

    # Retime: keep first timestamp, apply original gap sequence (cycled if needed).
    t0 = float(events[0]["seconds"])
    out[0]["seconds"] = t0
    for i in range(1, len(out)):
        gap = gaps[(i - 1) % len(gaps)] if gaps else 1.5
        out[i]["seconds"] = float(out[i - 1]["seconds"]) + gap

    return BattleExample(
        battle_id=battle.battle_id + "-alt",
        team_deck=battle.team_deck,
        opponent_deck=battle.opponent_deck,
        team_wins=battle.team_wins,
        events=tuple(out),
    )


def _build_xy_prior(battles: list[BattleExample]) -> dict[str, list[tuple[int, int]]]:
    prior: dict[str, list[tuple[int, int]]] = {"team": [], "opponent": []}
    for battle in battles:
        for event in battle.events:
            if event["event_type"] != "card_play":
                continue
            side = event["side"]
            prior[side].append((int(event["x"]), int(event["y"])))
            if len(prior["team"]) > 80_000 and len(prior["opponent"]) > 80_000:
                return prior
    if not prior["team"]:
        prior["team"] = [(9000, 8000)]
    if not prior["opponent"]:
        prior["opponent"] = [(9000, 24000)]
    return prior


def _next_real_side(battle: BattleExample, warmup: int, step: int, fallback: str) -> str:
    idx = warmup + step
    if idx < len(battle.events):
        return str(battle.events[idx]["side"])
    return fallback


def _next_real_card_for_side(
    battle: BattleExample, side: str, after_seconds: float
) -> str | None:
    for event in battle.events:
        if float(event["seconds"]) <= after_seconds + 1e-9:
            continue
        if event["side"] != side:
            continue
        return str(event["card"])
    return None


@torch.no_grad()
def rollout_one(
    model: torch.nn.Module,
    vocab,
    costs: dict[str, int],
    seed_battle: BattleExample,
    device: torch.device,
    *,
    mode: AblationMode = "baseline",
    warmup_events: int = WARMUP_EVENTS,
    max_new_events: int = MAX_NEW_EVENTS,
    temperature: float = 0.8,
    max_context: int = DEFAULT_MAX_CONTEXT,
    threat_dim: int = 0,
    timing_prior: TimingPrior | None = None,
    xy_prior: dict[str, list[tuple[int, int]]] | None = None,
    rng: random.Random | None = None,
) -> BattleExample | None:
    """Generate one continuation with optional single-factor ablation."""
    rng = rng or random.Random(0)
    if len(seed_battle.events) < warmup_events + 4:
        return None

    events = [dict(event) for event in seed_battle.events[:warmup_events]]
    seconds = float(events[-1]["seconds"])
    next_side = seed_battle.events[warmup_events]["side"]

    for step in range(max_new_events):
        if mode == "real_side_order":
            next_side = _next_real_side(
                seed_battle,
                warmup_events,
                step,
                fallback="opponent" if next_side == "team" else "team",
            )

        dummy = {
            "seconds": seconds + 1.0,
            "side": next_side,
            "event_type": "card_play",
            "card": (
                seed_battle.team_deck[0]
                if next_side == "team"
                else seed_battle.opponent_deck[0]
            ),
            "x": 9000,
            "y": 8000 if next_side == "team" else 24000,
        }
        probe = BattleExample(
            battle_id=seed_battle.battle_id + "-rollout",
            team_deck=seed_battle.team_deck,
            opponent_deck=seed_battle.opponent_deck,
            team_wins=seed_battle.team_wins,
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
            team_deck,
            opp_deck,
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
            team_deck.unsqueeze(0).to(device),
            opp_deck.unsqueeze(0).to(device),
            global_feat.unsqueeze(0).to(device),
            length.unsqueeze(0).to(device),
            slot_feats.unsqueeze(0).to(device),
            hand_mask.unsqueeze(0).to(device),
        )

        acting_deck = (
            seed_battle.team_deck if next_side == "team" else seed_battle.opponent_deck
        )
        logits = out["slot_logits"][0] / max(temperature, 1e-3)
        probs = torch.softmax(logits, dim=-1).cpu().numpy()

        if mode == "greedy_slots":
            slot = int(probs.argmax())
        elif mode == "teacher_forced_slots":
            real_card = _next_real_card_for_side(seed_battle, next_side, seconds)
            forced = deck_slot_for_card(acting_deck, real_card) if real_card else None
            if forced is not None:
                slot = int(forced)
            else:
                slot = int(rng.choices(range(8), weights=probs.tolist(), k=1)[0])
        else:
            slot = int(rng.choices(range(8), weights=probs.tolist(), k=1)[0])

        card = acting_deck[slot]
        event_type = (
            "ability_activation"
            if int(out["type_logits"][0].argmax().item()) == 1
            else "card_play"
        )

        if mode == "human_like_xy" and xy_prior is not None:
            x, y = rng.choice(xy_prior[next_side])
            if event_type == "ability_activation":
                x, y = 9000, 16000
        else:
            xy = out["xy"][0].cpu().numpy()
            x = int(np.clip(xy[0] * 18000.0, 3000, 15000))
            y_norm = float(xy[1])
            if next_side == "opponent":
                y_norm = 1.0 - y_norm
            y = int(np.clip(y_norm * 32000.0, 500, 31500))
            if event_type == "ability_activation":
                x, y = 9000, 16000

        if mode == "human_like_delays" and timing_prior is not None:
            dt = float(np.clip(timing_prior.sample(rng), 0.2, 12.0))
        else:
            dt = float(np.clip(np.expm1(out["timing"][0].item()), 0.2, 12.0))

        seconds = min(330.0, seconds + dt)
        events.append(
            {
                "seconds": seconds,
                "side": next_side,
                "event_type": event_type,
                "card": card,
                "x": int(x),
                "y": int(y),
            }
        )
        if mode != "real_side_order":
            next_side = "opponent" if next_side == "team" else "team"

    return BattleExample(
        battle_id=seed_battle.battle_id + f"-rollout-{mode}",
        team_deck=seed_battle.team_deck,
        opponent_deck=seed_battle.opponent_deck,
        team_wins=seed_battle.team_wins,
        events=tuple(events),
    )


def _feature_matrix(
    battles: list[BattleExample], costs: dict[str, int]
) -> np.ndarray:
    if not battles:
        return np.zeros((0, len(REALISM_FEATURE_NAMES)), dtype=np.float64)
    return np.stack([extract_realism_features(b, costs) for b in battles])


def _top_divergent_features(
    collapsed: list[BattleExample],
    healthy: list[BattleExample],
    costs: dict[str, int],
    top_k: int = 15,
) -> list[dict[str, Any]]:
    if not collapsed or not healthy:
        return []
    c = _feature_matrix(collapsed, costs)
    h = _feature_matrix(healthy, costs)
    c_mean = c.mean(axis=0)
    h_mean = h.mean(axis=0)
    c_std = c.std(axis=0)
    h_std = h.std(axis=0)
    pooled = np.sqrt(0.5 * (c_std**2 + h_std**2)) + 1e-6
    effect = (c_mean - h_mean) / pooled
    order = np.argsort(-np.abs(effect))
    rows: list[dict[str, Any]] = []
    for idx in order[:top_k]:
        name = (
            REALISM_FEATURE_NAMES[idx]
            if idx < len(REALISM_FEATURE_NAMES)
            else f"f{idx}"
        )
        rows.append(
            {
                "feature": name,
                "collapsed_mean": float(c_mean[idx]),
                "healthy_mean": float(h_mean[idx]),
                "delta": float(c_mean[idx] - h_mean[idx]),
                "cohen_d": float(effect[idx]),
            }
        )
    return rows


def run_rollout_autopsy(
    input_dir: str | Path = "data/raw",
    policy_dir: str | Path = "models/policy_bc_v3",
    realism_model_dir: str | Path = "models/realism_scorer",
    card_costs_path: str | Path = "data/card_costs.json",
    output_json: str | Path = "reports/rollout_autopsy_v1.json",
    output_html: str | Path = "reports/rollout_autopsy_v1.html",
    n_battles: int = 200,
    seed: int = 42,
    temperature: float = 0.8,
    device_name: str | None = None,
    min_card_plays: int = 12,
) -> dict[str, Any]:
    started = time.time()
    created_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    model, vocab, cfg, device = load_policy(policy_dir, device_name=device_name)
    threat_dim = int(cfg.get("threat_dim", 0))
    max_context = int(cfg.get("max_context", DEFAULT_MAX_CONTEXT))
    costs = load_card_costs(card_costs_path)
    realism_path = Path(realism_model_dir) / "realism_ensemble.pkl"
    realism_artifact = _load_realism_scorer(realism_path)

    print(f"Loading battles from {input_dir} ...", flush=True)
    battles = collect_battles(input_dir, min_card_plays=min_card_plays)
    train_battles, _val_battles, test_battles = split_battles(battles, seed=seed)
    timing_prior = TimingPrior.from_battles(train_battles)
    xy_prior = _build_xy_prior(test_battles)

    eligible = [b for b in test_battles if len(b.events) >= WARMUP_EVENTS + 4]
    rng = random.Random(seed)
    rng.shuffle(eligible)
    seeds = eligible[:n_battles]
    print(
        f"Device={device} · seeds={len(seeds)} · threat_dim={threat_dim}",
        flush=True,
    )

    # --- Baseline rollouts (same harness logic as training eval; stable seed pairing) ---
    print("Generating baseline policy rollouts ...", flush=True)
    seed_rollout_pairs: list[tuple[BattleExample, BattleExample]] = []
    for i, seed_battle in enumerate(seeds):
        battle = rollout_one(
            model,
            vocab,
            costs,
            seed_battle,
            device,
            mode="baseline",
            temperature=temperature,
            max_context=max_context,
            threat_dim=threat_dim,
            rng=random.Random(seed + 1000 + i),
        )
        if battle is None:
            continue
        seed_rollout_pairs.append((seed_battle, battle))
        if (i + 1) % 40 == 0:
            print(f"  baseline {i + 1}/{len(seeds)}", flush=True)

    rollout_battles = [rb for _, rb in seed_rollout_pairs]
    scores = _score_realism(realism_artifact, rollout_battles, costs)
    paired: list[tuple[BattleExample, BattleExample, float]] = [
        (sb, rb, float(score))
        for (sb, rb), score in zip(seed_rollout_pairs, scores)
    ]
    collapsed_rows = [(sb, rb, s) for sb, rb, s in paired if s < COLLAPSE_THRESH]
    healthy_rows = [(sb, rb, s) for sb, rb, s in paired if s > HEALTHY_THRESH]
    mid_rows = [
        (sb, rb, s)
        for sb, rb, s in paired
        if COLLAPSE_THRESH <= s <= HEALTHY_THRESH
    ]

    # --- Scorer OOD controls ---
    print("Scoring OOD controls on real test battles ...", flush=True)
    n_ctrl = min(len(seeds), len(paired))
    ctrl_seeds = [sb for sb, _, _ in paired[:n_ctrl]]
    target_len = WARMUP_EVENTS + MAX_NEW_EVENTS

    trunc = [truncate_battle(b, target_len) for b in ctrl_seeds if len(b.events) >= target_len]
    # If short, take as many events as available past warmup+4
    if len(trunc) < n_ctrl // 2:
        trunc = [
            truncate_battle(b, min(len(b.events), target_len))
            for b in ctrl_seeds
            if len(b.events) >= WARMUP_EVENTS + 4
        ]
    scores_trunc = _score_realism(realism_artifact, trunc, costs)

    alt_battles: list[BattleExample] = []
    for b in ctrl_seeds:
        clipped = truncate_battle(b, min(len(b.events), target_len))
        forced = force_strict_alternation(clipped)
        if forced is not None:
            alt_battles.append(forced)
    scores_alt = _score_realism(realism_artifact, alt_battles, costs)

    clipped_dt = [
        clip_battle_delays(truncate_battle(b, min(len(b.events), target_len)))
        for b in ctrl_seeds
        if len(b.events) >= WARMUP_EVENTS + 4
    ]
    scores_clip = _score_realism(realism_artifact, clipped_dt, costs)

    ood = {
        "truncated_real": {
            **_score_stats(scores_trunc),
            "description": (
                f"Real test battles truncated to ≤{target_len} events "
                f"(warmup {WARMUP_EVENTS} + {MAX_NEW_EVENTS})."
            ),
            "hist": _histogram(scores_trunc),
        },
        "strict_alternation_real": {
            **_score_stats(scores_alt),
            "description": (
                "Real battles re-ordered into strict side alternation "
                "(within-side order preserved; timestamps rebuilt from original gaps)."
            ),
            "hist": _histogram(scores_alt),
            "n_infeasible_skipped": int(n_ctrl - len(alt_battles)),
        },
        "clipped_delays_real": {
            **_score_stats(scores_clip),
            "description": "Real battles with inter-event delays clipped to [0.2, 12]s.",
            "hist": _histogram(scores_clip),
        },
    }

    # Scorer-OOD global flag: controls collapse at similar rate to policy collapses.
    policy_collapse_rate = float(len(collapsed_rows) / max(len(paired), 1))
    ctrl_collapse_rates = {
        "truncated_real": ood["truncated_real"]["collapse_rate"],
        "strict_alternation_real": ood["strict_alternation_real"]["collapse_rate"],
        "clipped_delays_real": ood["clipped_delays_real"]["collapse_rate"],
    }
    # If any control collapses at ≥50% of policy collapse rate AND ≥5% absolute,
    # flag that control as scorer-sensitive. Global scorer_ood if truncated collapses
    # like policy (harness length alone) OR if alt+clip both do.
    scorer_ood_flags = {
        name: (
            rate >= max(0.05, 0.5 * policy_collapse_rate)
            and rate >= 0.05
        )
        for name, rate in ctrl_collapse_rates.items()
    }
    # Truncated real staying healthy means length alone is fine; collapse on
    # alternation means scorer+harness interaction, not pure OOD.
    scorer_ood_global = bool(
        scorer_ood_flags["truncated_real"]
        or (
            scorer_ood_flags["clipped_delays_real"]
            and ood["clipped_delays_real"]["mean"] < 0.5
        )
    )

    # --- Head ablations on collapsed seeds ---
    print(
        f"Running head ablations on {len(collapsed_rows)} collapsed seeds ...",
        flush=True,
    )
    # Generate all ablations first, then batch-score per mode.
    abl_battles: dict[str, list[BattleExample | None]] = {m: [] for m in ABLATION_ORDER}
    for ci, (seed_battle, _base_battle, _base_score) in enumerate(collapsed_rows):
        for mode in ABLATION_ORDER:
            abl = rollout_one(
                model,
                vocab,
                costs,
                seed_battle,
                device,
                mode=mode,
                temperature=temperature,
                max_context=max_context,
                threat_dim=threat_dim,
                timing_prior=timing_prior,
                xy_prior=xy_prior,
                rng=random.Random(
                    seed + 5000 + ci * 17 + ABLATION_ORDER.index(mode) * 31
                ),
            )
            abl_battles[mode].append(abl)
        if (ci + 1) % 10 == 0:
            print(f"  ablations {ci + 1}/{len(collapsed_rows)}", flush=True)

    abl_scores: dict[str, list[float]] = {}
    for mode in ABLATION_ORDER:
        valid = [b for b in abl_battles[mode] if b is not None]
        scored = _score_realism(realism_artifact, valid, costs)
        it = iter(scored)
        abl_scores[mode] = [
            float(next(it)) if b is not None else 0.0 for b in abl_battles[mode]
        ]

    attributions: list[dict[str, Any]] = []
    ablation_score_table: dict[str, list[float]] = {m: [] for m in ABLATION_ORDER}
    for ci, (seed_battle, _base_battle, base_score) in enumerate(collapsed_rows):
        row: dict[str, Any] = {
            "battle_id": seed_battle.battle_id,
            "baseline_score": float(base_score),
            "ablations": {},
            "attributed_cause": "unattributed_compound",
            "recovering_ablation": None,
        }
        attributed = False
        if scorer_ood_global and scorer_ood_flags["truncated_real"]:
            row["attributed_cause"] = "scorer_ood"
            row["recovering_ablation"] = "ood_control_truncated_real"
            attributed = True
        for mode in ABLATION_ORDER:
            score = abl_scores[mode][ci]
            ablation_score_table[mode].append(float(score))
            row["ablations"][mode] = {
                "score": float(score),
                "lift": float(score - base_score),
                "recovered": bool(score > RECOVERY_THRESH),
                "note": ABLATION_NOTES[mode],
            }
            if not attributed and score > RECOVERY_THRESH:
                row["attributed_cause"] = CAUSE_FROM_ABLATION[mode]
                row["recovering_ablation"] = mode
                attributed = True
        attributions.append(row)

    cause_counts = Counter(r["attributed_cause"] for r in attributions)
    n_collapsed = len(attributions)
    # Single-cause attribution: any non-compound label.
    n_single = sum(
        1
        for r in attributions
        if r["attributed_cause"] != "unattributed_compound"
    )
    single_cause_rate = float(n_single / max(n_collapsed, 1))
    experiment_pass = bool(n_collapsed > 0 and single_cause_rate >= 0.70)

    # Dominant cause among attributed (first-wins order)
    dominant_cause = None
    dominant_share = 0.0
    if n_collapsed:
        dominant_cause, dom_n = cause_counts.most_common(1)[0]
        dominant_share = float(dom_n / n_collapsed)

    # Universal recovery: modes that lift every collapsed seed (regardless of order)
    universal_recovery_modes = [
        mode
        for mode in ABLATION_ORDER
        if ablation_score_table[mode]
        and all(s > RECOVERY_THRESH for s in ablation_score_table[mode])
    ]

    feature_diff = _top_divergent_features(
        [rb for _, rb, _ in collapsed_rows],
        [rb for _, rb, _ in healthy_rows],
        costs,
    )

    # Alternation feature sanity on baseline rollouts
    alt_rates_collapsed = [
        float(extract_realism_features(rb, costs)[13]) for _, rb, _ in collapsed_rows
    ]
    alt_rates_healthy = [
        float(extract_realism_features(rb, costs)[13]) for _, rb, _ in healthy_rows
    ]

    # Readiness-gate validity
    # Gates use mean policy score (rollout_not_collapsed / rollout_near_real).
    # Valid only when a majority shares one policy-head cause; universal recovery
    # under one ablation is noted separately as a strong secondary signal.
    if scorer_ood_global:
        gate_validity = "invalid"
        gate_rationale = (
            "OOD controls collapse similarly → scorer flags harness/distribution "
            "artifacts, so rollout realism cannot gate single-head improvements."
        )
    elif dominant_cause == "hardcoded_alternation" and dominant_share >= 0.5:
        gate_validity = "invalid"
        gate_rationale = (
            "Most collapses recover when side order follows real battles → "
            "hardcoded alternation in the rollout harness, not a policy head. "
            "Mean realism gates conflate harness artifact with policy quality."
        )
    elif (
        experiment_pass
        and dominant_cause
        and dominant_cause.startswith("policy_")
        and dominant_share >= 0.5
    ):
        gate_validity = "valid_for_that_head"
        gate_rationale = (
            f"Majority of collapses first-attributed to {dominant_cause} "
            f"({dominant_share:.0%}). Rollout realism can gate that head; "
            "live play remains unjustified."
        )
    elif experiment_pass and universal_recovery_modes:
        uni = universal_recovery_modes[0]
        gate_validity = "partially_valid"
        gate_rationale = (
            f"Every collapse recovers under {uni} (universal), but first-wins "
            f"attribution splits across causes (dominant {dominant_cause} "
            f"{dominant_share:.0%}). Mean realism gates are weak single-head "
            "signals — prefer head-wise metrics plus XY-focused checks."
        )
    elif experiment_pass:
        gate_validity = "partially_valid"
        gate_rationale = (
            "Collapses are identifiable (not compound), but no majority cause. "
            "Interpret mean realism gates cautiously."
        )
    else:
        gate_validity = "invalid"
        gate_rationale = (
            "Attribution is diffuse (compound drift). Rollout realism cannot gate "
            "single-head improvements."
        )

    lessons = [
        (
            f"Collapse rate={policy_collapse_rate:.1%} "
            f"({n_collapsed}/{len(paired)}); healthy={len(healthy_rows)} "
            f"mid={len(mid_rows)}."
        ),
        (
            f"Single-cause attribution rate={single_cause_rate:.1%} → "
            f"{'PASS' if experiment_pass else 'FAIL'} (≥70% required)."
        ),
        (
            "OOD truncated-real mean="
            f"{ood['truncated_real']['mean']:.3f}, "
            "strict-alternation-real mean="
            f"{ood['strict_alternation_real']['mean']:.3f}, "
            "clipped-delay-real mean="
            f"{ood['clipped_delays_real']['mean']:.3f}."
        ),
        gate_rationale,
        "Live play remains unjustified — this autopsy is offline only; "
        "v3 ready_for_live_smoke_test is treated as suspect under audit.",
    ]

    report: dict[str, Any] = {
        "experiment": "D",
        "name": "rollout_collapse_autopsy",
        "version": "v1",
        "created_at": created_at,
        "seconds": round(time.time() - started, 1),
        "hypothesis": (
            "~16% of policy rollouts scoring near-zero realism have one identifiable "
            "cause (policy head, hardcoded alternation, or scorer OOD)."
        ),
        "success_criterion": (
            "≥70% of collapses attributed to a single cause → PASS; "
            "else diffuse attribution → FAIL."
        ),
        "model": {
            "policy_dir": str(policy_dir),
            "policy_version": cfg.get("version"),
            "realism_model_dir": str(realism_model_dir),
            "threat_dim": threat_dim,
            "max_context": max_context,
            "temperature": temperature,
        },
        "compute": {
            "device": str(device),
            "cuda": bool(torch.cuda.is_available()),
            "n_baseline_rollouts": len(paired),
            "n_collapsed_ablated": n_collapsed,
            "n_ablation_modes": len(ABLATION_ORDER),
        },
        "data": {
            "battles_total": len(battles),
            "test_eligible": len(eligible),
            "n_seeds_requested": n_battles,
            "n_paired": len(paired),
            "warmup_events": WARMUP_EVENTS,
            "max_new_events": MAX_NEW_EVENTS,
            "collapse_threshold": COLLAPSE_THRESH,
            "healthy_threshold": HEALTHY_THRESH,
            "recovery_threshold": RECOVERY_THRESH,
        },
        "baseline": {
            **_score_stats(scores),
            "n_collapsed": n_collapsed,
            "n_healthy": len(healthy_rows),
            "n_mid": len(mid_rows),
            "hist": _histogram(scores),
        },
        "ood_controls": ood,
        "scorer_ood": {
            "global": scorer_ood_global,
            "flags": scorer_ood_flags,
            "control_collapse_rates": ctrl_collapse_rates,
            "policy_collapse_rate": policy_collapse_rate,
        },
        "ablation_methodology": {
            "order": list(ABLATION_ORDER),
            "notes": ABLATION_NOTES,
            "cause_map": CAUSE_FROM_ABLATION,
            "rule": (
                "For each collapsed seed, run ablations in order; attribute to the "
                f"first mode with score > {RECOVERY_THRESH}. If truncated-real OOD "
                "control itself collapses globally, label scorer_ood."
            ),
        },
        "ablation_summary": {
            mode: {
                **_score_stats(ablation_score_table[mode]),
                "mean_lift_vs_baseline": float(
                    np.mean(ablation_score_table[mode])
                    - np.mean([r["baseline_score"] for r in attributions])
                )
                if attributions and ablation_score_table[mode]
                else 0.0,
                "recovery_rate": float(
                    np.mean(
                        [
                            1.0 if s > RECOVERY_THRESH else 0.0
                            for s in ablation_score_table[mode]
                        ]
                    )
                )
                if ablation_score_table[mode]
                else 0.0,
            }
            for mode in ABLATION_ORDER
        },
        "universal_recovery_modes": universal_recovery_modes,
        "attribution": {
            "n_collapsed": n_collapsed,
            "n_single_cause": n_single,
            "single_cause_rate": single_cause_rate,
            "cause_counts": dict(cause_counts),
            "dominant_cause": dominant_cause,
            "dominant_share": dominant_share,
            "per_seed": attributions,
        },
        "feature_diff_collapsed_vs_healthy": feature_diff,
        "alternation_sanity": {
            "collapsed_alt_rate_mean": float(np.mean(alt_rates_collapsed))
            if alt_rates_collapsed
            else None,
            "healthy_alt_rate_mean": float(np.mean(alt_rates_healthy))
            if alt_rates_healthy
            else None,
        },
        "verdict": {
            "experiment_pass": experiment_pass,
            "label": "PASS" if experiment_pass else "FAIL",
            "collapse_rate": policy_collapse_rate,
            "gate_validity": gate_validity,
            "gate_rationale": gate_rationale,
            "live_play_justified": False,
            "live_play_note": (
                "Live play is still unjustified. This experiment audits offline "
                "rollout realism gates only; v3 ready_for_live_smoke_test remains "
                "suspect under audit."
            ),
        },
        "lessons": lessons,
        "recommendation": _recommendation(
            experiment_pass,
            gate_validity,
            dominant_cause,
            dominant_share,
            ood,
            universal_recovery_modes,
        ),
    }

    out_json = Path(output_json)
    out_json.parent.mkdir(parents=True, exist_ok=True)
    # Compact per-seed for JSON size but keep ablations.
    with out_json.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    print(f"Wrote {out_json}", flush=True)

    out_html = Path(output_html)
    render_rollout_autopsy_report(report, out_html)
    print(f"Wrote {out_html}", flush=True)
    return report


def _recommendation(
    experiment_pass: bool,
    gate_validity: str,
    dominant_cause: str | None,
    dominant_share: float,
    ood: dict[str, Any],
    universal_recovery_modes: list[str] | None = None,
) -> str:
    parts: list[str] = []
    if gate_validity == "invalid":
        parts.append(
            "Do not use mean rollout realism as a go/no-go for single-head "
            "policy work until the attributed artifact is fixed."
        )
    if universal_recovery_modes:
        parts.append(
            f"All collapsed seeds recover under {universal_recovery_modes[0]} — "
            "prioritize placement/XY head fixes for the collapse tail."
        )
    if dominant_cause == "hardcoded_alternation":
        parts.append(
            "Replace strict side alternation in rollout_policy_battles with a "
            "side-decision model or teacher side schedule before trusting gates."
        )
    elif dominant_cause and dominant_cause.startswith("policy_"):
        parts.append(
            f"First-wins dominant cause is {dominant_cause} "
            f"({dominant_share:.0%}); use head-wise offline metrics alongside "
            "realism means."
        )
    if ood["strict_alternation_real"]["collapse_rate"] >= 0.1:
        parts.append(
            "Scorer is alternation-sensitive: treat harness side-order as a "
            "first-class confound in any realism gate."
        )
    if not experiment_pass:
        parts.append(
            "Diffuse collapse → compound distribution drift; prefer head-wise "
            "offline metrics (slot/xy/timing) over rollout realism for gating."
        )
    parts.append("Keep live play blocked.")
    return " ".join(parts)


# HTML rendering lives in rollout_autopsy_report.py (kit-based interactive report).

def main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(description="Experiment D — rollout-collapse autopsy")
    parser.add_argument("--input", default="data/raw")
    parser.add_argument("--policy-dir", default="models/policy_bc_v3")
    parser.add_argument("--realism-model-dir", default="models/realism_scorer")
    parser.add_argument("--card-costs", default="data/card_costs.json")
    parser.add_argument("--output-json", default="reports/rollout_autopsy_v1.json")
    parser.add_argument("--output-html", default="reports/rollout_autopsy_v1.html")
    parser.add_argument("--n-battles", type=int, default=200)
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--temperature", type=float, default=0.8)
    parser.add_argument("--device", default=None)
    args = parser.parse_args(argv)
    report = run_rollout_autopsy(
        input_dir=args.input,
        policy_dir=args.policy_dir,
        realism_model_dir=args.realism_model_dir,
        card_costs_path=args.card_costs,
        output_json=args.output_json,
        output_html=args.output_html,
        n_battles=args.n_battles,
        seed=args.seed,
        temperature=args.temperature,
        device_name=args.device,
    )
    v = report["verdict"]
    print(
        f"\nVERDICT {v['label']} · collapse={v['collapse_rate']:.1%} · "
        f"gates={v['gate_validity']} · live_play_justified={v['live_play_justified']}",
        flush=True,
    )


if __name__ == "__main__":
    main()
