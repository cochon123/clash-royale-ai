"""Experiment C — Action Clock: who acts next + delay, with clock-aware rollouts.

Offline only. Trains CPU HistGradientBoosting models for initiative (SAME vs
OPPONENT) and inter-event delay, compares against trivial baselines, then plugs
the clock into a rollout variant (does not modify the v2/v3 training path).
"""

from __future__ import annotations

import html
import json
import pickle
import random
import time
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import numpy as np
import torch

from .policy_dataset import (
    DEFAULT_MAX_CONTEXT,
    DEFAULT_MIN_CONTEXT,
    DEFAULT_REACTION_SECONDS,
    GLOBAL_DIM,
    THREAT_DIM,
    _encode_causal_stream,
    build_vocab,
    collect_battles,
    encode_policy_sample,
    load_card_costs,
    recent_opponent_threat,
    split_battles,
    summarize_split,
)
from .policy_infer import load_policy
from .policy_train import _load_realism_scorer, _score_realism, rollout_policy_battles
from .winner_dataset import BattleExample

# Labels relative to the side that just acted at event i.
SAME = 1
OPPONENT = 0
LABEL_NAMES = {SAME: "SAME", OPPONENT: "OPPONENT"}

CLOCK_EXTRA_DIM = 6  # last_side, streak, elixir_team, elixir_opp, phase, seconds
FEATURE_DIM = GLOBAL_DIM + THREAT_DIM + CLOCK_EXTRA_DIM
COLLAPSE_THRESHOLD = 0.25
MIN_EVENT_INDEX = DEFAULT_MIN_CONTEXT  # i >= 8


def _phase_id(seconds: float) -> int:
    if seconds >= 240:
        return 2  # triple
    if seconds >= 120:
        return 1  # double
    return 0  # single


def _phase_name(phase: int) -> str:
    return ("single", "double", "triple")[int(phase)]


def _streak_ending_at(events: tuple[dict[str, Any], ...], index: int) -> int:
    side = events[index]["side"]
    streak = 1
    for j in range(index - 1, -1, -1):
        if events[j]["side"] == side:
            streak += 1
        else:
            break
    return streak


def extract_clock_features(
    battle: BattleExample,
    event_index: int,
    vocab,
    costs: dict[str, int],
    *,
    stream: tuple[torch.Tensor, ...] | None = None,
    reaction_seconds: float = DEFAULT_REACTION_SECONDS,
) -> np.ndarray | None:
    """Feature vector after event ``event_index`` (causal; no peek at i+1)."""
    if event_index < 0 or event_index >= len(battle.events):
        return None
    if stream is None:
        stream = _encode_causal_stream(battle, vocab, costs, swap_sides=False)
    if stream is None:
        return None
    _continuous, _card_ids, _team_deck, _opp_deck, globals_all = stream
    if event_index >= globals_all.size(0):
        return None

    global_feat = globals_all[event_index].detach().cpu().numpy().astype(np.float32)
    current = battle.events[event_index]
    acting_side = current["side"]
    # Prefix = events before current; now = current timestamp (causal for next-actor).
    threat, _ = recent_opponent_threat(
        battle,
        event_index,
        acting_side=acting_side,
        costs=costs,
        max_age=reaction_seconds,
    )

    seconds = float(current["seconds"])
    # Globals indices 18/19 are team/opp elixir (see _encode_causal_stream).
    elixir_team = float(global_feat[18]) if global_feat.shape[0] > 18 else 0.5
    elixir_opp = float(global_feat[19]) if global_feat.shape[0] > 19 else 0.5
    extras = np.asarray(
        [
            1.0 if acting_side == "team" else 0.0,
            min(_streak_ending_at(battle.events, event_index) / 6.0, 1.0),
            elixir_team,
            elixir_opp,
            float(_phase_id(seconds)) / 2.0,
            min(seconds / 300.0, 1.5),
        ],
        dtype=np.float32,
    )
    return np.concatenate(
        [global_feat, threat.detach().cpu().numpy().astype(np.float32), extras]
    )


def build_clock_arrays(
    battles: list[BattleExample],
    vocab,
    costs: dict[str, int],
    *,
    max_samples_per_battle: int | None = None,
    seed: int = 0,
    reaction_seconds: float = DEFAULT_REACTION_SECONDS,
) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
    """Return X, y_actor (SAME=1), y_delay (seconds), phases."""
    rng = random.Random(seed)
    xs: list[np.ndarray] = []
    y_actor: list[int] = []
    y_delay: list[float] = []
    phases: list[int] = []

    for battle in battles:
        if len(battle.events) < MIN_EVENT_INDEX + 1:
            continue
        stream = _encode_causal_stream(battle, vocab, costs, swap_sides=False)
        if stream is None:
            continue
        indices = list(range(MIN_EVENT_INDEX, len(battle.events) - 1))
        if max_samples_per_battle is not None and len(indices) > max_samples_per_battle:
            indices = rng.sample(indices, max_samples_per_battle)
            indices.sort()
        for i in indices:
            feat = extract_clock_features(
                battle,
                i,
                vocab,
                costs,
                stream=stream,
                reaction_seconds=reaction_seconds,
            )
            if feat is None:
                continue
            cur = battle.events[i]
            nxt = battle.events[i + 1]
            same = SAME if nxt["side"] == cur["side"] else OPPONENT
            dt = float(nxt["seconds"]) - float(cur["seconds"])
            if dt < 0:
                continue
            xs.append(feat)
            y_actor.append(same)
            y_delay.append(dt)
            phases.append(_phase_id(float(cur["seconds"])))

    if not xs:
        empty = np.zeros((0, FEATURE_DIM), dtype=np.float32)
        return empty, np.zeros(0, dtype=np.int64), np.zeros(0, dtype=np.float32), np.zeros(
            0, dtype=np.int64
        )
    return (
        np.stack(xs).astype(np.float32),
        np.asarray(y_actor, dtype=np.int64),
        np.asarray(y_delay, dtype=np.float32),
        np.asarray(phases, dtype=np.int64),
    )


@dataclass
class ActionClockModels:
    actor: Any
    delay: Any
    feature_dim: int
    majority_class: int
    phase_median_delay: dict[int, float]
    global_median_delay: float
    created_at: str

    def predict_actor_proba(self, x: np.ndarray) -> np.ndarray:
        x2 = np.atleast_2d(x)
        return self.actor.predict_proba(x2)[:, 1]

    def predict_actor(self, x: np.ndarray) -> np.ndarray:
        return self.actor.predict(np.atleast_2d(x))

    def predict_delay(self, x: np.ndarray) -> np.ndarray:
        log_dt = self.delay.predict(np.atleast_2d(x))
        return np.expm1(log_dt).astype(np.float64)

    def sample_next_side(
        self,
        x: np.ndarray,
        current_side: str,
        rng: random.Random,
    ) -> str:
        p_same = float(self.predict_actor_proba(x)[0])
        same = rng.random() < p_same
        if same:
            return current_side
        return "opponent" if current_side == "team" else "team"

    def sample_delay(self, x: np.ndarray, rng: random.Random | None = None) -> float:
        dt = float(self.predict_delay(x)[0])
        # Light noise so rollouts are not perfectly deterministic.
        if rng is not None:
            dt *= float(np.exp(rng.gauss(0.0, 0.08)))
        return float(np.clip(dt, 0.15, 12.0))


def _baseline_actor_metrics(
    y_true: np.ndarray,
    *,
    majority_class: int,
) -> dict[str, Any]:
    n = len(y_true)
    if n == 0:
        return {
            "alternation_acc": 0.0,
            "majority_acc": 0.0,
            "alternation_pred": OPPONENT,
            "majority_pred": majority_class,
            "same_rate": 0.0,
        }
    alt_pred = np.full(n, OPPONENT, dtype=np.int64)
    maj_pred = np.full(n, majority_class, dtype=np.int64)
    return {
        "alternation_acc": float((alt_pred == y_true).mean()),
        "majority_acc": float((maj_pred == y_true).mean()),
        "alternation_pred": OPPONENT,
        "majority_pred": int(majority_class),
        "same_rate": float((y_true == SAME).mean()),
    }


def _delay_mae(y_true: np.ndarray, y_pred: np.ndarray) -> float:
    if len(y_true) == 0:
        return 0.0
    return float(np.mean(np.abs(y_true - y_pred)))


def _phase_median_baseline(
    phases: np.ndarray,
    y_delay: np.ndarray,
    phase_medians: dict[int, float],
    fallback: float,
) -> np.ndarray:
    out = np.empty(len(phases), dtype=np.float64)
    for i, phase in enumerate(phases):
        out[i] = phase_medians.get(int(phase), fallback)
    return out


@torch.no_grad()
def rollout_policy_battles_with_clock(
    model,
    vocab,
    costs: dict[str, int],
    seed_battles: list[BattleExample],
    device: torch.device,
    clock: ActionClockModels,
    *,
    n_battles: int = 96,
    warmup_events: int = 12,
    max_new_events: int = 40,
    temperature: float = 0.8,
    seed: int = 0,
    max_context: int = DEFAULT_MAX_CONTEXT,
    threat_dim: int = 0,
    use_clock_actor: bool = True,
    use_clock_delay: bool = True,
) -> list[BattleExample]:
    """Rollout variant: clock samples replace hardcoded side flip and/or timing.

    Leaves ``rollout_policy_battles`` untouched for the v2/v3 training path.
    """
    model.eval()
    rng = random.Random(seed)
    chosen = list(seed_battles)
    rng.shuffle(chosen)
    chosen = chosen[:n_battles]
    out_battles: list[BattleExample] = []

    for battle in chosen:
        if len(battle.events) < warmup_events + 4:
            continue
        events = [dict(event) for event in battle.events[:warmup_events]]
        seconds = float(events[-1]["seconds"])
        next_side = battle.events[warmup_events]["side"]

        for _ in range(max_new_events):
            dummy = {
                "seconds": seconds + 1.0,
                "side": next_side,
                "event_type": "card_play",
                "card": battle.team_deck[0]
                if next_side == "team"
                else battle.opponent_deck[0],
                "x": 9000,
                "y": 8000 if next_side == "team" else 24000,
            }
            probe = BattleExample(
                battle_id=battle.battle_id + "-clock-rollout",
                team_deck=battle.team_deck,
                opponent_deck=battle.opponent_deck,
                team_wins=battle.team_wins,
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
            logits = out["slot_logits"][0] / max(temperature, 1e-3)
            probs = torch.softmax(logits, dim=-1).cpu().numpy()
            slot = int(rng.choices(range(8), weights=probs.tolist(), k=1)[0])
            acting_deck = battle.team_deck if next_side == "team" else battle.opponent_deck
            card = acting_deck[slot]
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

            if use_clock_delay:
                # Delay conditioned on state *before* this play (last committed event).
                feat = extract_clock_features(
                    BattleExample(
                        battle_id=battle.battle_id,
                        team_deck=battle.team_deck,
                        opponent_deck=battle.opponent_deck,
                        team_wins=battle.team_wins,
                        events=tuple(events),
                    ),
                    len(events) - 1,
                    vocab,
                    costs,
                )
                if feat is None:
                    dt = float(np.clip(np.expm1(out["timing"][0].item()), 0.2, 12.0))
                else:
                    dt = clock.sample_delay(feat, rng)
            else:
                dt = float(np.clip(np.expm1(out["timing"][0].item()), 0.2, 12.0))

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

            if use_clock_actor:
                feat_after = extract_clock_features(
                    BattleExample(
                        battle_id=battle.battle_id,
                        team_deck=battle.team_deck,
                        opponent_deck=battle.opponent_deck,
                        team_wins=battle.team_wins,
                        events=tuple(events),
                    ),
                    len(events) - 1,
                    vocab,
                    costs,
                )
                if feat_after is None:
                    next_side = "opponent" if next_side == "team" else "team"
                else:
                    next_side = clock.sample_next_side(feat_after, next_side, rng)
            else:
                next_side = "opponent" if next_side == "team" else "team"

        out_battles.append(
            BattleExample(
                battle_id=battle.battle_id + "-clock-rollout",
                team_deck=battle.team_deck,
                opponent_deck=battle.opponent_deck,
                team_wins=battle.team_wins,
                events=tuple(events),
            )
        )
    return out_battles


def _collapse_stats(scores: list[float], threshold: float = COLLAPSE_THRESHOLD) -> dict[str, Any]:
    if not scores:
        return {
            "n": 0,
            "mean_score": 0.0,
            "collapse_rate": 0.0,
            "n_collapsed": 0,
            "threshold": threshold,
        }
    arr = np.asarray(scores, dtype=np.float64)
    collapsed = arr < threshold
    return {
        "n": int(len(arr)),
        "mean_score": float(arr.mean()),
        "median_score": float(np.median(arr)),
        "collapse_rate": float(collapsed.mean()),
        "n_collapsed": int(collapsed.sum()),
        "threshold": threshold,
        "p10": float(np.percentile(arr, 10)),
        "p90": float(np.percentile(arr, 90)),
    }


def _verdict(
    actor_acc: float,
    best_baseline_acc: float,
    alternation_acc: float,
    delay_mae: float,
    collapse_before: float,
    collapse_after: float,
) -> dict[str, Any]:
    lift_pp = (actor_acc - best_baseline_acc) * 100.0
    vs_alt_pp = (actor_acc - alternation_acc) * 100.0
    actor_pass = lift_pp >= 5.0
    delay_pass = delay_mae <= 1.25
    primary_pass = actor_pass and delay_pass
    fail_near_alt = abs(vs_alt_pp) <= 2.0 and actor_acc <= alternation_acc + 0.02

    if collapse_before > 1e-9:
        relative_reduction = (collapse_before - collapse_after) / collapse_before
    else:
        relative_reduction = 0.0 if collapse_after <= collapse_before else -1.0
    secondary_pass = relative_reduction >= 0.50

    if fail_near_alt and not actor_pass:
        status = "FAIL"
        summary = (
            f"Learned actor within 2pp of alternation "
            f"({actor_acc:.3f} vs {alternation_acc:.3f}) — initiative near-random "
            f"given these features."
        )
    elif primary_pass:
        status = "PASS"
        summary = (
            f"Actor +{lift_pp:.1f}pp over best trivial baseline and delay MAE "
            f"{delay_mae:.3f}s ≤ 1.25s."
        )
        if secondary_pass:
            summary += (
                f" Secondary: collapse rate reduced {relative_reduction * 100:.0f}% "
                f"relative ({collapse_before:.3f} → {collapse_after:.3f})."
            )
        else:
            summary += (
                f" Secondary (collapse −50% rel) not met "
                f"({collapse_before:.3f} → {collapse_after:.3f}, "
                f"{relative_reduction * 100:.0f}% rel)."
            )
    else:
        status = "FAIL"
        reasons = []
        if not actor_pass:
            reasons.append(
                f"actor lift {lift_pp:.1f}pp < 5pp over best baseline ({best_baseline_acc:.3f})"
            )
        if not delay_pass:
            reasons.append(f"delay MAE {delay_mae:.3f}s > 1.25s")
        summary = "Primary criteria not met: " + "; ".join(reasons) + "."

    return {
        "status": status,
        "primary_pass": primary_pass,
        "secondary_pass": secondary_pass,
        "fail_near_alternation": fail_near_alt,
        "actor_lift_pp": lift_pp,
        "actor_vs_alternation_pp": vs_alt_pp,
        "collapse_relative_reduction": relative_reduction,
        "summary": summary,
        "live_play_readiness": False,
        "note": "Offline experiment only — do not interpret as live-play readiness.",
    }


def run_action_clock_experiment(
    input_dir: str | Path = "data/raw",
    output_json: str | Path = "reports/action_clock_v1.json",
    output_html: str | Path = "reports/action_clock_v1.html",
    model_dir: str | Path = "models/action_clock_v1",
    policy_dir: str | Path = "models/policy_bc_v3",
    realism_model_dir: str | Path = "models/realism_scorer",
    card_costs_path: str | Path = "data/card_costs.json",
    min_card_plays: int = 12,
    seed: int = 42,
    max_samples_per_battle: int | None = 48,
    n_rollouts: int = 96,
    device_name: str | None = None,
    skip_rollouts: bool = False,
) -> dict[str, Any]:
    from sklearn.ensemble import HistGradientBoostingClassifier, HistGradientBoostingRegressor

    started = time.time()
    created_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    model_dir = Path(model_dir)
    model_dir.mkdir(parents=True, exist_ok=True)
    output_json = Path(output_json)
    output_html = Path(output_html)
    output_json.parent.mkdir(parents=True, exist_ok=True)

    print("Loading battles ...", flush=True)
    battles = collect_battles(input_dir, min_card_plays=min_card_plays)
    train_battles, val_battles, test_battles = split_battles(battles, seed=seed)
    vocab = build_vocab(train_battles)
    costs = load_card_costs(card_costs_path)

    print(
        f"Building clock datasets (max_samples_per_battle={max_samples_per_battle}) ...",
        flush=True,
    )
    x_train, y_actor_train, y_delay_train, phases_train = build_clock_arrays(
        train_battles,
        vocab,
        costs,
        max_samples_per_battle=max_samples_per_battle,
        seed=seed + 1,
    )
    x_val, y_actor_val, y_delay_val, phases_val = build_clock_arrays(
        val_battles,
        vocab,
        costs,
        max_samples_per_battle=max_samples_per_battle,
        seed=seed + 2,
    )
    x_test, y_actor_test, y_delay_test, phases_test = build_clock_arrays(
        test_battles,
        vocab,
        costs,
        max_samples_per_battle=max_samples_per_battle,
        seed=seed + 3,
    )
    print(
        f"Samples train/val/test: {len(x_train)}/{len(x_val)}/{len(x_test)}",
        flush=True,
    )

    majority_class = int(SAME if (y_actor_train == SAME).mean() >= 0.5 else OPPONENT)
    phase_medians: dict[int, float] = {}
    for phase in (0, 1, 2):
        mask = phases_train == phase
        if mask.any():
            phase_medians[phase] = float(np.median(y_delay_train[mask]))
    global_median = float(np.median(y_delay_train)) if len(y_delay_train) else 1.0
    for phase in (0, 1, 2):
        phase_medians.setdefault(phase, global_median)

    actor_baselines = _baseline_actor_metrics(
        y_actor_test, majority_class=majority_class
    )
    delay_phase_pred = _phase_median_baseline(
        phases_test, y_delay_test, phase_medians, global_median
    )
    delay_baseline_mae = _delay_mae(y_delay_test, delay_phase_pred)

    print("Training HGB actor + delay ...", flush=True)
    actor = HistGradientBoostingClassifier(
        max_depth=8,
        learning_rate=0.06,
        max_iter=250,
        l2_regularization=0.1,
        min_samples_leaf=20,
        random_state=seed,
    )
    actor.fit(x_train, y_actor_train)

    delay = HistGradientBoostingRegressor(
        max_depth=8,
        learning_rate=0.06,
        max_iter=250,
        l2_regularization=0.1,
        min_samples_leaf=20,
        random_state=seed,
        loss="squared_error",
    )
    delay.fit(x_train, np.log1p(y_delay_train))

    clock = ActionClockModels(
        actor=actor,
        delay=delay,
        feature_dim=int(x_train.shape[1]) if len(x_train) else FEATURE_DIM,
        majority_class=majority_class,
        phase_median_delay={int(k): float(v) for k, v in phase_medians.items()},
        global_median_delay=global_median,
        created_at=created_at,
    )

    actor_pred_test = actor.predict(x_test)
    actor_acc = float((actor_pred_test == y_actor_test).mean()) if len(x_test) else 0.0
    actor_proba = actor.predict_proba(x_test)[:, 1] if len(x_test) else np.zeros(0)
    delay_pred_test = np.expm1(delay.predict(x_test)) if len(x_test) else np.zeros(0)
    delay_mae = _delay_mae(y_delay_test, delay_pred_test)

    # Validation for reporting curves / early signal.
    actor_acc_val = (
        float((actor.predict(x_val) == y_actor_val).mean()) if len(x_val) else 0.0
    )
    delay_mae_val = (
        _delay_mae(y_delay_val, np.expm1(delay.predict(x_val))) if len(x_val) else 0.0
    )

    best_baseline_acc = max(
        actor_baselines["alternation_acc"], actor_baselines["majority_acc"]
    )

    # Delay MAE by phase for learned vs baseline.
    delay_by_phase: dict[str, Any] = {}
    for phase in (0, 1, 2):
        mask = phases_test == phase
        name = _phase_name(phase)
        if not mask.any():
            delay_by_phase[name] = {"n": 0}
            continue
        delay_by_phase[name] = {
            "n": int(mask.sum()),
            "learned_mae": _delay_mae(y_delay_test[mask], delay_pred_test[mask]),
            "phase_median_mae": _delay_mae(
                y_delay_test[mask], delay_phase_pred[mask]
            ),
            "median_true": float(np.median(y_delay_test[mask])),
            "phase_median_train": phase_medians[phase],
        }

    with (model_dir / "action_clock.pkl").open("wb") as handle:
        pickle.dump(
            {
                "actor": actor,
                "delay": delay,
                "majority_class": majority_class,
                "phase_median_delay": clock.phase_median_delay,
                "global_median_delay": global_median,
                "feature_dim": clock.feature_dim,
                "created_at": created_at,
                "vocab": vocab.to_dict(),
            },
            handle,
        )

    # --- Rollouts: alternation (existing) vs clock ---
    rollout_compare: dict[str, Any] = {"available": False}
    if not skip_rollouts:
        realism_path = Path(realism_model_dir) / "realism_ensemble.pkl"
        realism_artifact = _load_realism_scorer(realism_path)
        if realism_artifact is None:
            print(f"Realism scorer missing at {realism_path}; skipping rollouts.", flush=True)
        else:
            print("Loading policy for rollouts ...", flush=True)
            policy, policy_vocab, cfg, device = load_policy(
                policy_dir, device_name=device_name
            )
            threat_dim = int(cfg.get("threat_dim", 0))
            max_context = int(cfg.get("max_context", DEFAULT_MAX_CONTEXT))
            rollout_seed = seed + 7

            print(
                f"Generating {n_rollouts} alternation rollouts (seed={rollout_seed}) ...",
                flush=True,
            )
            alt_rollouts = rollout_policy_battles(
                policy,
                policy_vocab,
                costs,
                test_battles,
                device,
                n_battles=n_rollouts,
                seed=rollout_seed,
                max_context=max_context,
                threat_dim=threat_dim,
            )
            print(
                f"Generating {n_rollouts} clock rollouts (seed={rollout_seed}) ...",
                flush=True,
            )
            clock_rollouts = rollout_policy_battles_with_clock(
                policy,
                policy_vocab,
                costs,
                test_battles,
                device,
                clock,
                n_battles=n_rollouts,
                seed=rollout_seed,
                max_context=max_context,
                threat_dim=threat_dim,
                use_clock_actor=True,
                use_clock_delay=True,
            )
            scores_alt = _score_realism(realism_artifact, alt_rollouts, costs)
            scores_clock = _score_realism(realism_artifact, clock_rollouts, costs)
            real_slice = test_battles[: max(len(alt_rollouts), len(clock_rollouts))]
            scores_real = _score_realism(
                realism_artifact, real_slice[: len(alt_rollouts)], costs
            )
            alt_stats = _collapse_stats(scores_alt)
            clock_stats = _collapse_stats(scores_clock)
            rollout_compare = {
                "available": True,
                "policy_dir": str(policy_dir),
                "seed": rollout_seed,
                "n_requested": n_rollouts,
                "collapse_threshold": COLLAPSE_THRESHOLD,
                "alternation": {**alt_stats, "scores": scores_alt},
                "clock": {**clock_stats, "scores": scores_clock},
                "real_slice": _collapse_stats(scores_real),
            }
            print(
                f"Collapse <{COLLAPSE_THRESHOLD}: alternation={alt_stats['collapse_rate']:.3f} "
                f"clock={clock_stats['collapse_rate']:.3f}",
                flush=True,
            )

    collapse_before = float(
        rollout_compare.get("alternation", {}).get("collapse_rate", 0.0)
    )
    collapse_after = float(rollout_compare.get("clock", {}).get("collapse_rate", 0.0))
    verdict = _verdict(
        actor_acc,
        best_baseline_acc,
        actor_baselines["alternation_acc"],
        delay_mae,
        collapse_before,
        collapse_after,
    )

    # Recommendation
    actor_ok = verdict["actor_lift_pp"] >= 5.0
    delay_ok = delay_mae <= 1.25
    if verdict["status"] == "PASS" and verdict["secondary_pass"]:
        recommendation = (
            "Adopt the action clock in rollout generation; keep policy card/placement "
            "heads separate. Next: calibrate SAME probability and condition delay on "
            "predicted actor."
        )
    elif verdict["status"] == "PASS":
        recommendation = (
            "Clock beats trivial initiative/delay baselines offline — keep it for "
            "supervised timing/initiative heads. Collapse secondary miss means "
            "alternation is not the sole realism failure mode; pair with Experiment D "
            "autopsy before trusting rollout gates."
        )
    elif verdict.get("fail_near_alternation"):
        recommendation = (
            "Kill or redesign: current globals+threat features do not encode "
            "initiative beyond alternation. Need hand/elixir-cycle or board-state "
            "features before replacing rollout flip."
        )
    elif actor_ok and not delay_ok:
        recommendation = (
            "Partial win: initiative is real (+≥5pp over alternation) — replace "
            "hardcoded side flip with the actor clock in rollouts. Delay misses the "
            "1.25s MAE gate (only a small edge over phase-median); keep policy timing "
            "or phase medians until cycle/affordability features improve Δt."
        )
    elif delay_ok and not actor_ok:
        recommendation = (
            "Delay model is usable; actor initiative is not. Use phase/learned delay "
            "in rollouts but keep alternation (or majority) for side until better "
            "initiative features exist."
        )
    else:
        recommendation = (
            "Do not plug clock into rollouts yet. Revisit features (per-side elixir "
            "affordability, cycle waits) or accept alternation as a hard prior."
        )

    # Compact delay residual hist for HTML (no matplotlib).
    residuals = (y_delay_test - delay_pred_test).tolist() if len(y_delay_test) else []
    delay_bins = _histogram(y_delay_test.tolist(), bins=24, lo=0.0, hi=12.0)
    residual_bins = _histogram(residuals, bins=24, lo=-6.0, hi=6.0)

    same_rate_by_phase = {}
    for phase in (0, 1, 2):
        mask = phases_test == phase
        same_rate_by_phase[_phase_name(phase)] = {
            "n": int(mask.sum()),
            "same_rate": float((y_actor_test[mask] == SAME).mean()) if mask.any() else 0.0,
            "actor_acc": float((actor_pred_test[mask] == y_actor_test[mask]).mean())
            if mask.any()
            else 0.0,
        }

    report = {
        "model_name": "action-clock-v1",
        "model_version": "1.0.0",
        "experiment": "C",
        "hypothesis": (
            "Who-acts-next and when is unmodeled (rollouts hardcode side alternation); "
            "a learned clock beats trivial baselines."
        ),
        "created_at": created_at,
        "seconds": round(time.time() - started, 1),
        "compute": {
            "device": "cpu",
            "framework": "sklearn HistGradientBoosting",
            "actor": "HistGradientBoostingClassifier",
            "delay": "HistGradientBoostingRegressor(log1p Δt)",
            "max_iter": 250,
            "max_samples_per_battle": max_samples_per_battle,
            "feature_dim": clock.feature_dim,
            "global_dim": GLOBAL_DIM,
            "threat_dim": THREAT_DIM,
            "clock_extra_dim": CLOCK_EXTRA_DIM,
        },
        "data": {
            "battles_total": len(battles),
            "min_card_plays": min_card_plays,
            "min_event_index": MIN_EVENT_INDEX,
            "train_samples": int(len(x_train)),
            "val_samples": int(len(x_val)),
            "test_samples": int(len(x_test)),
            "same_rate_train": float((y_actor_train == SAME).mean())
            if len(y_actor_train)
            else 0.0,
            "same_rate_test": float((y_actor_test == SAME).mean())
            if len(y_actor_test)
            else 0.0,
            "mean_delay_train": float(y_delay_train.mean()) if len(y_delay_train) else 0.0,
            "mean_delay_test": float(y_delay_test.mean()) if len(y_delay_test) else 0.0,
            "splits": [
                summarize_split("train", train_battles),
                summarize_split("val", val_battles),
                summarize_split("test", test_battles),
            ],
        },
        "baselines": {
            "actor": actor_baselines,
            "delay_phase_median_mae": delay_baseline_mae,
            "phase_median_delay_train": {
                _phase_name(k): v for k, v in phase_medians.items()
            },
        },
        "metrics": {
            "val": {"actor_acc": actor_acc_val, "delay_mae": delay_mae_val},
            "test": {
                "actor_acc": actor_acc,
                "delay_mae": delay_mae,
                "delay_phase_median_mae": delay_baseline_mae,
                "best_trivial_actor_acc": best_baseline_acc,
                "actor_lift_pp": verdict["actor_lift_pp"],
                "mean_pred_p_same": float(actor_proba.mean()) if len(actor_proba) else 0.0,
            },
            "delay_by_phase": delay_by_phase,
            "actor_by_phase": same_rate_by_phase,
        },
        "rollouts": {"available": False},
        "rollout_hist": {},
        "charts": {
            "delay_true_hist": delay_bins,
            "delay_residual_hist": residual_bins,
        },
        "success_criteria": {
            "actor_acc_ge_5pp_over_best_baseline": verdict["primary_pass"]
            and verdict["actor_lift_pp"] >= 5.0,
            "delay_mae_le_1_25": delay_mae <= 1.25,
            "collapse_relative_reduction_ge_50pct": verdict["secondary_pass"],
            "fail_if_within_2pp_of_alternation": verdict["fail_near_alternation"],
        },
        "verdict": verdict,
        "recommendation": recommendation,
        "lessons": [
            "Rollout_policy_battles hardcodes next_side flip; any initiative signal must "
            "live outside the policy heads.",
            "Labels are SAME vs OPPONENT relative to the side that just acted — not "
            "absolute team/opponent classification.",
            "Delay target is log1p(Δt); report MAE in real seconds after expm1.",
            "Collapse rate = fraction of rollouts with realism P(real) < 0.25.",
            "Offline only: passing this experiment does not justify live play.",
        ],
        "checkpoint": str(model_dir / "action_clock.pkl"),
    }

    # Slim rollout section for JSON (keep scores in rollout_hist only).
    if rollout_compare.get("available"):
        report["rollouts"] = {
            "available": True,
            "policy_dir": rollout_compare["policy_dir"],
            "seed": rollout_compare["seed"],
            "n_requested": rollout_compare["n_requested"],
            "collapse_threshold": COLLAPSE_THRESHOLD,
            "alternation": {
                k: v
                for k, v in rollout_compare["alternation"].items()
                if k != "scores"
            },
            "clock": {
                k: v for k, v in rollout_compare["clock"].items() if k != "scores"
            },
            "real_slice": {
                k: v
                for k, v in rollout_compare.get("real_slice", {}).items()
                if k != "scores"
            },
        }
        report["rollout_hist"] = {
            "alternation": rollout_compare["alternation"].get("scores"),
            "clock": rollout_compare["clock"].get("scores"),
        }

    with output_json.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    render_action_clock_report(report, output_html)
    print(json.dumps({"verdict": verdict, "metrics": report["metrics"]["test"]}, indent=2))
    print(f"Wrote {output_json}")
    print(f"Wrote {output_html}")
    return report


def _histogram(
    values: list[float], *, bins: int, lo: float, hi: float
) -> dict[str, Any]:
    if not values:
        return {"bins": [], "counts": [], "lo": lo, "hi": hi}
    counts, edges = np.histogram(values, bins=bins, range=(lo, hi))
    centers = ((edges[:-1] + edges[1:]) / 2.0).tolist()
    return {
        "centers": centers,
        "counts": counts.astype(int).tolist(),
        "lo": lo,
        "hi": hi,
        "bins": bins,
    }


# HTML rendering lives in action_clock_report.py

def main() -> None:
    import argparse

    parser = argparse.ArgumentParser(description="Experiment C — Action Clock")
    parser.add_argument("--input", default="data/raw")
    parser.add_argument("--output-json", default="reports/action_clock_v1.json")
    parser.add_argument("--output-html", default="reports/action_clock_v1.html")
    parser.add_argument("--model-dir", default="models/action_clock_v1")
    parser.add_argument("--policy-dir", default="models/policy_bc_v3")
    parser.add_argument("--realism-model-dir", default="models/realism_scorer")
    parser.add_argument("--card-costs", default="data/card_costs.json")
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--max-samples-per-battle", type=int, default=48)
    parser.add_argument("--n-rollouts", type=int, default=96)
    parser.add_argument("--device", default=None)
    parser.add_argument("--skip-rollouts", action="store_true")
    args = parser.parse_args()
    run_action_clock_experiment(
        input_dir=args.input,
        output_json=args.output_json,
        output_html=args.output_html,
        model_dir=args.model_dir,
        policy_dir=args.policy_dir,
        realism_model_dir=args.realism_model_dir,
        card_costs_path=args.card_costs,
        seed=args.seed,
        max_samples_per_battle=args.max_samples_per_battle,
        n_rollouts=args.n_rollouts,
        device_name=args.device,
        skip_rollouts=args.skip_rollouts,
    )


if __name__ == "__main__":
    main()
