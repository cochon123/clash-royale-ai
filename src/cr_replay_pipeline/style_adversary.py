"""Adversarial style helpers for policy_bc_v5.

The frozen style judge is a non-differentiable sklearn ensemble on rollout
statistics. V5 therefore combines:

1. Differentiable batch moment-matching on controllable tells (timing gaps,
   placement spread).
2. Black-box REINFORCE polish with reward = P(human) from the frozen judge,
   using clock-aware turn-taking so harness tells (strict alternation) are not
   forced by the rollout protocol.
"""

from __future__ import annotations

import pickle
import random
from pathlib import Path
from typing import Any

import numpy as np
import torch
from torch.distributions import Categorical

from .action_clock import ActionClockModels, extract_clock_features
from .policy_dataset import encode_policy_sample
from .realism_train import extract_realism_features
from .rollout_autopsy import REALISM_FEATURE_NAMES
from .style_train import HARNESS_FEATURES, truncate_battle
from .winner_dataset import BattleExample

# Controllable (non-harness) tells the style judge uses most strongly.
STYLE_MATCH_FEATURES: tuple[str, ...] = (
    "frac_gap_lt_1",
    "gap_p10",
    "gap_std",
    "frac_gap_gt_8",
    "frac_gap_lt_2",
    "gap_p90",
    "team_x_std",
    "opp_x_std",
    "team_y_std",
    "opp_y_std",
    "tile_diversity",
)


def load_style_judge(model_dir: str | Path) -> dict[str, Any] | None:
    path = Path(model_dir) / "style_ensemble.pkl"
    if not path.exists():
        return None
    with path.open("rb") as handle:
        return pickle.load(handle)


def load_action_clock(model_dir: str | Path = "models/action_clock_v1") -> ActionClockModels | None:
    path = Path(model_dir) / "action_clock.pkl"
    if not path.exists():
        return None
    with path.open("rb") as handle:
        data = pickle.load(handle)
    return ActionClockModels(
        actor=data["actor"],
        delay=data["delay"],
        feature_dim=int(data.get("feature_dim", 0)),
        majority_class=int(data.get("majority_class", 0)),
        phase_median_delay={int(k): float(v) for k, v in data["phase_median_delay"].items()},
        global_median_delay=float(data.get("global_median_delay", 2.0)),
        created_at=str(data.get("created_at", "")),
    )


def feature_name_index() -> dict[str, int]:
    return {name: i for i, name in enumerate(REALISM_FEATURE_NAMES)}


def harness_indices() -> list[int]:
    idx = feature_name_index()
    return [idx[name] for name in HARNESS_FEATURES if name in idx]


def match_feature_indices() -> list[int]:
    idx = feature_name_index()
    return [idx[name] for name in STYLE_MATCH_FEATURES if name in idx]


def battles_to_features(battles: list[BattleExample], costs: dict[str, int]) -> np.ndarray:
    if not battles:
        return np.zeros((0, len(REALISM_FEATURE_NAMES)), dtype=np.float64)
    return np.stack(
        [extract_realism_features(truncate_battle(b), costs) for b in battles]
    )


def human_style_targets(
    battles: list[BattleExample],
    costs: dict[str, int],
    max_battles: int = 2000,
) -> dict[str, Any]:
    """Precompute human feature means and action-level timing/placement moments."""
    rng = random.Random(0)
    pool = list(battles)
    rng.shuffle(pool)
    pool = pool[: min(max_battles, len(pool))]
    feats = battles_to_features(pool, costs)
    means = feats.mean(axis=0) if len(feats) else np.zeros(len(REALISM_FEATURE_NAMES))
    stds = feats.std(axis=0) if len(feats) else np.ones(len(REALISM_FEATURE_NAMES))

    gaps: list[float] = []
    xs: list[float] = []
    ys: list[float] = []
    for battle in pool:
        events = truncate_battle(battle).events
        for i, event in enumerate(events):
            if i > 0:
                gaps.append(float(event["seconds"]) - float(events[i - 1]["seconds"]))
            if event["event_type"] != "card_play":
                continue
            xs.append(float(event["x"]) / 18000.0)
            y = float(event["y"]) / 32000.0
            if event["side"] == "opponent":
                y = 1.0 - y
            ys.append(y)

    gap_arr = np.asarray(gaps, dtype=np.float64) if gaps else np.asarray([2.0])
    x_arr = np.asarray(xs, dtype=np.float64) if xs else np.asarray([0.5])
    y_arr = np.asarray(ys, dtype=np.float64) if ys else np.asarray([0.35])

    return {
        "feature_means": means.astype(np.float64),
        "feature_stds": np.clip(stds.astype(np.float64), 1e-3, None),
        "n_battles": len(pool),
        "gap_mean": float(gap_arr.mean()),
        "gap_std": float(gap_arr.std()),
        "gap_p10": float(np.percentile(gap_arr, 10)),
        "gap_p90": float(np.percentile(gap_arr, 90)),
        "frac_gap_lt_1": float((gap_arr < 1.0).mean()),
        "frac_gap_lt_2": float((gap_arr < 2.0).mean()),
        "frac_gap_gt_8": float((gap_arr > 8.0).mean()),
        "x_std": float(x_arr.std()),
        "y_std": float(y_arr.std()),
        "x_mean": float(x_arr.mean()),
        "y_mean": float(y_arr.mean()),
        "match_features": list(STYLE_MATCH_FEATURES),
    }


def neutralize_harness_features(
    x: np.ndarray,
    human_means: np.ndarray,
) -> np.ndarray:
    """Replace harness columns with human means so reward targets controllable tells."""
    out = np.array(x, copy=True, dtype=np.float64)
    for j in harness_indices():
        out[..., j] = human_means[j]
    return out


def blend_predict(artifact: dict[str, Any], x: np.ndarray) -> np.ndarray:
    hgb = artifact["models"]["hist_gradient_boosting"]
    tree = artifact["models"]["extra_trees"]
    weight = float(artifact["hgb_weight"])
    return weight * hgb.predict_proba(x)[:, 1] + (1.0 - weight) * tree.predict_proba(x)[:, 1]


def score_style_battles(
    artifact: dict[str, Any],
    battles: list[BattleExample],
    costs: dict[str, int],
    *,
    human_means: np.ndarray | None = None,
    neutralize_harness: bool = False,
) -> np.ndarray:
    if not battles:
        return np.zeros(0, dtype=np.float64)
    x = battles_to_features(battles, costs)
    if neutralize_harness and human_means is not None:
        x = neutralize_harness_features(x, human_means)
    return blend_predict(artifact, x)


def style_detection_metrics(
    artifact: dict[str, Any],
    humans: list[BattleExample],
    ai: list[BattleExample],
    costs: dict[str, int],
    *,
    human_means: np.ndarray | None = None,
    neutralize_harness: bool = False,
) -> dict[str, float]:
    from .realism_train import _auc_binary

    n = min(len(humans), len(ai))
    if n == 0:
        return {
            "n": 0,
            "auc": 0.5,
            "fool_rate_at_0.5": 0.0,
            "mean_P_human_ai": 0.0,
            "mean_P_human_real": 0.0,
            "feature_l2": 0.0,
        }
    h = [truncate_battle(b) for b in humans[:n]]
    a = ai[:n]
    x_h = battles_to_features(h, costs)
    x_a = battles_to_features(a, costs)
    if neutralize_harness and human_means is not None:
        x_h_s = neutralize_harness_features(x_h, human_means)
        x_a_s = neutralize_harness_features(x_a, human_means)
    else:
        x_h_s, x_a_s = x_h, x_a
    p_h = blend_predict(artifact, x_h_s)
    p_a = blend_predict(artifact, x_a_s)
    y = np.asarray([1] * n + [0] * n, dtype=np.int64)
    probs = np.concatenate([p_h, p_a])
    match_idx = match_feature_indices()
    if match_idx and human_means is not None:
        std = np.clip(np.std(x_h[:, match_idx], axis=0), 1e-3, None)
        feature_l2 = float(
            np.mean(((x_a[:, match_idx] - human_means[match_idx]) / std) ** 2) ** 0.5
        )
    else:
        feature_l2 = 0.0
    return {
        "n": n,
        "auc": float(_auc_binary(y, probs)),
        "fool_rate_at_0.5": float((p_a >= 0.5).mean()),
        "mean_P_human_ai": float(p_a.mean()),
        "mean_P_human_real": float(p_h.mean()),
        "feature_l2": feature_l2,
    }


def batch_style_match_loss(
    outputs: dict[str, torch.Tensor],
    targets: dict[str, Any],
    *,
    weight: float = 0.25,
) -> dict[str, torch.Tensor]:
    """Push batch timing/placement moments toward human action-level stats."""
    timing = outputs["timing"]
    xy = outputs["xy"]
    dt = torch.expm1(timing).clamp(0.15, 12.0)
    x = xy[:, 0]
    y = xy[:, 1]

    # Soft fraction estimators (differentiable).
    frac_lt_1 = torch.sigmoid((1.0 - dt) * 4.0).mean()
    frac_lt_2 = torch.sigmoid((2.0 - dt) * 3.0).mean()
    frac_gt_8 = torch.sigmoid((dt - 8.0) * 3.0).mean()

    # Batch std needs >1 samples; fall back to zero loss terms otherwise.
    if dt.numel() > 1:
        gap_std = dt.std(unbiased=False)
        x_std = x.std(unbiased=False)
        y_std = y.std(unbiased=False)
        # Approximate percentiles via soft sorting on a sorted view.
        sorted_dt, _ = torch.sort(dt)
        n = sorted_dt.numel()
        p10 = sorted_dt[max(0, int(0.10 * (n - 1)))]
        p90 = sorted_dt[min(n - 1, int(0.90 * (n - 1)))]
    else:
        gap_std = dt.new_tensor(0.0)
        x_std = dt.new_tensor(0.0)
        y_std = dt.new_tensor(0.0)
        p10 = dt.mean()
        p90 = dt.mean()

    def mse(pred: torch.Tensor, target: float) -> torch.Tensor:
        return (pred - pred.new_tensor(float(target))).pow(2)

    loss = (
        1.2 * mse(gap_std, targets["gap_std"])
        + 1.0 * mse(frac_lt_1, targets["frac_gap_lt_1"])
        + 0.7 * mse(frac_lt_2, targets["frac_gap_lt_2"])
        + 1.0 * mse(frac_gt_8, targets["frac_gap_gt_8"])
        + 0.8 * mse(p10, targets["gap_p10"])
        + 0.6 * mse(p90, targets["gap_p90"])
        + 1.4 * mse(x_std, targets["x_std"])
        + 1.2 * mse(y_std, targets["y_std"])
        + 0.3 * mse(x.mean(), targets["x_mean"])
        + 0.3 * mse(y.mean(), targets["y_mean"])
    )
    # Mild entropy bonus so slot head does not collapse while chasing style.
    slot_probs = outputs.get("slot_probs")
    if slot_probs is not None:
        entropy = -(slot_probs * (slot_probs + 1e-8).log()).sum(dim=-1).mean()
        # Target ~ human-ish soft entropy around 1.6 nats (not uniform 2.08).
        loss = loss + 0.05 * (entropy - 1.6).pow(2)

    total = weight * loss
    return {
        "style_match_loss": total,
        "style_match_raw": loss.detach(),
        "pred_gap_std": gap_std.detach(),
        "pred_x_std": x_std.detach(),
        "pred_frac_gap_lt_1": frac_lt_1.detach(),
    }


def _sample_slot(
    out: dict[str, torch.Tensor],
    temperature: float,
) -> tuple[int, torch.Tensor]:
    logits = out["slot_logits"][0] / max(temperature, 1e-3)
    dist = Categorical(logits=logits)
    slot_t = dist.sample()
    return int(slot_t.item()), dist.log_prob(slot_t)


def _sample_conditioned_action(
    out: dict[str, torch.Tensor],
    slot_log_prob: torch.Tensor,
    placement_temperature: float = 0.6,
    placement_top_k: int = 5,
) -> tuple[int, torch.Tensor, torch.Tensor, float]:
    """Sample type/timing after placement has been conditioned on the slot."""
    type_logits = out["type_logits"][0]
    type_dist = Categorical(logits=type_logits)
    type_t = type_dist.sample()
    event_type = int(type_t.item())
    log_prob = slot_log_prob + type_dist.log_prob(type_t)

    # Light Gaussian exploration on timing so REINFORCE can nudge delays.
    timing_mean = out["timing"][0]
    timing_std = timing_mean.new_tensor(0.15)
    timing_noise = torch.randn((), device=timing_mean.device) * timing_std
    timing_sample = timing_mean + timing_noise
    timing_log_prob = -0.5 * (timing_noise / timing_std).pow(2) - timing_std.log()
    log_prob = log_prob + 0.35 * timing_log_prob

    xy = out["xy"][0]
    tile_logits = out.get("tile_logits")
    if tile_logits is not None:
        logits = tile_logits[0] / max(placement_temperature, 1e-3)
        k = max(1, min(int(placement_top_k), int(logits.numel())))
        values, indices = torch.topk(logits, k)
        tile_dist = Categorical(logits=values)
        local_tile = tile_dist.sample()
        tile = indices[local_tile]
        log_prob = log_prob + tile_dist.log_prob(local_tile)
        row = torch.div(tile, 32, rounding_mode="floor")
        col = tile.remainder(32)
        xy = torch.stack(((col.float() + 0.5) / 32.0, (row.float() + 0.5) / 18.0))
    dt = float(np.clip(np.expm1(timing_sample.detach().item()), 0.2, 12.0))
    return event_type, log_prob, xy, dt


def rollout_trajectories_for_reinforce(
    model,
    vocab,
    costs: dict[str, int],
    seed_battles: list[BattleExample],
    device: torch.device,
    clock: ActionClockModels | None,
    *,
    n_battles: int = 24,
    warmup_events: int = 12,
    max_new_events: int = 24,
    temperature: float = 0.9,
    seed: int = 0,
    max_context: int = 64,
    threat_dim: int = 14,
    use_clock_actor: bool = True,
) -> tuple[list[BattleExample], list[torch.Tensor]]:
    """Sample short clock-aware rollouts and accumulate per-trajectory log-probs.

    Must stay in train() mode: cuDNN GRU backward is disabled under eval().
    Dropout noise is acceptable exploration for REINFORCE.
    """
    model.train()
    rng = random.Random(seed)
    chosen = list(seed_battles)
    rng.shuffle(chosen)
    chosen = chosen[:n_battles]
    battles_out: list[BattleExample] = []
    traj_log_probs: list[torch.Tensor] = []

    for battle in chosen:
        if len(battle.events) < warmup_events + 4:
            continue
        events = [dict(event) for event in battle.events[:warmup_events]]
        seconds = float(events[-1]["seconds"])
        next_side = battle.events[warmup_events]["side"]
        log_probs: list[torch.Tensor] = []

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
                battle_id=battle.battle_id + "-v5-rollout",
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
            slot, slot_log_prob = _sample_slot(out, temperature)
            if getattr(model, "placement_card_mode", "soft") == "selected":
                out = model(
                    continuous.unsqueeze(0).to(device), card_ids.unsqueeze(0).to(device),
                    team_deck.unsqueeze(0).to(device), opp_deck.unsqueeze(0).to(device),
                    global_feat.unsqueeze(0).to(device), length.unsqueeze(0).to(device),
                    slot_feats.unsqueeze(0).to(device), hand_mask.unsqueeze(0).to(device),
                    placement_slots=torch.tensor([slot], device=device),
                )
            event_type_i, log_prob, xy_t, dt = _sample_conditioned_action(
                out, slot_log_prob
            )
            log_probs.append(log_prob)

            acting_deck = battle.team_deck if next_side == "team" else battle.opponent_deck
            card = acting_deck[slot]
            event_type = "ability_activation" if event_type_i == 1 else "card_play"
            xy = xy_t.detach().cpu().numpy()
            x = int(np.clip(xy[0] * 18000.0, 3000, 15000))
            y_norm = float(xy[1])
            if next_side == "opponent":
                y_norm = 1.0 - y_norm
            y = int(np.clip(y_norm * 32000.0, 500, 31500))
            if event_type == "ability_activation":
                x, y = 9000, 16000
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

            if use_clock_actor and clock is not None:
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

        if not log_probs:
            continue
        battles_out.append(
            BattleExample(
                battle_id=battle.battle_id + "-v5-rollout",
                team_deck=battle.team_deck,
                opponent_deck=battle.opponent_deck,
                team_wins=battle.team_wins,
                events=tuple(events),
            )
        )
        # Keep step log-probs separate so polish can backward one step at a time
        # (summing first holds every GRU unroll in VRAM until backward).
        traj_log_probs.append(log_probs)

    return battles_out, traj_log_probs


def per_battle_feature_reward(
    battles: list[BattleExample],
    costs: dict[str, int],
    human_targets: dict[str, Any],
) -> np.ndarray:
    """Dense reward in [0, 1]: closer match-feature vectors → higher reward.

    The frozen sklearn judge is dominated by harness features under the
    alternation protocol, so REINFORCE optimizes controllable tell distance.
    """
    if not battles:
        return np.zeros(0, dtype=np.float64)
    x = battles_to_features(battles, costs)
    match_idx = match_feature_indices()
    if not match_idx:
        return np.zeros(len(battles), dtype=np.float64)
    means = human_targets["feature_means"][match_idx]
    std = human_targets["feature_stds"][match_idx]
    z = np.abs((x[:, match_idx] - means) / std)
    # Mean absolute z across match features; map 0→1, 2→0.
    return 1.0 - np.clip(z.mean(axis=1), 0.0, 2.0) / 2.0


def reinforce_style_step(
    model,
    optimizer,
    vocab,
    costs: dict[str, int],
    seed_battles: list[BattleExample],
    device: torch.device,
    style_artifact: dict[str, Any],
    human_targets: dict[str, Any],
    clock: ActionClockModels | None,
    *,
    n_battles: int = 24,
    max_new_events: int = 24,
    temperature: float = 0.9,
    seed: int = 0,
    max_context: int = 64,
    threat_dim: int = 14,
    reward_scale: float = 1.0,
    baseline_ema: float | None = None,
    baseline_momentum: float = 0.9,
    max_grad_norm: float = 1.0,
) -> tuple[dict[str, float], float]:
    """One REINFORCE polish step maximizing human tell-match under clock rollouts."""
    rollouts, traj_log_probs = rollout_trajectories_for_reinforce(
        model,
        vocab,
        costs,
        seed_battles,
        device,
        clock,
        n_battles=n_battles,
        max_new_events=max_new_events,
        temperature=temperature,
        seed=seed,
        max_context=max_context,
        threat_dim=threat_dim,
        use_clock_actor=True,
    )
    if not rollouts:
        return {"n": 0, "reward_mean": 0.0, "loss": 0.0}, float(baseline_ema or 0.0)

    # Primary reward: match-feature closeness (judge P saturates under clock / harness-free).
    rewards = per_battle_feature_reward(rollouts, costs, human_targets)
    # Light judge signal when not saturated.
    judge_probs = score_style_battles(
        style_artifact,
        rollouts,
        costs,
        neutralize_harness=False,
    )
    if float(judge_probs.mean()) < 0.95:
        rewards = 0.7 * rewards + 0.3 * judge_probs

    reward_mean = float(rewards.mean())
    if baseline_ema is None:
        baseline_ema = reward_mean
    else:
        baseline_ema = baseline_momentum * baseline_ema + (1.0 - baseline_momentum) * reward_mean
    adv = rewards - baseline_ema
    if len(adv) > 1 and float(np.std(adv)) > 1e-6:
        adv = (adv - adv.mean()) / (adv.std() + 1e-6)

    # Backward per step so GRU graphs are freed immediately (3050 6GB safe).
    optimizer.zero_grad(set_to_none=True)
    n_traj = max(len(traj_log_probs), 1)
    loss_acc = 0.0
    for step_log_probs, a in zip(traj_log_probs, adv):
        n_steps = max(len(step_log_probs), 1)
        for log_prob in step_log_probs:
            loss_i = (-log_prob * float(a) * reward_scale) / (n_traj * n_steps)
            loss_i.backward()
            loss_acc += float(loss_i.detach().item())
    torch.nn.utils.clip_grad_norm_(model.parameters(), max_grad_norm)
    optimizer.step()
    if device.type == "cuda":
        torch.cuda.empty_cache()
    policy_loss_value = loss_acc

    match_idx = match_feature_indices()
    x_ai = battles_to_features(rollouts, costs)
    if match_idx:
        std = human_targets["feature_stds"][match_idx]
        feature_l2 = float(
            np.mean(
                ((x_ai[:, match_idx] - human_targets["feature_means"][match_idx]) / std) ** 2
            )
            ** 0.5
        )
    else:
        feature_l2 = 0.0
    return {
        "n": len(rollouts),
        "reward_mean": reward_mean,
        "full_P_human": float(judge_probs.mean()) if len(judge_probs) else 0.0,
        "fool_rate_at_0.5": float((judge_probs >= 0.5).mean()) if len(judge_probs) else 0.0,
        "feature_l2": feature_l2,
        "loss": float(policy_loss_value),
        "baseline": float(baseline_ema),
    }, float(baseline_ema)


def _tell_gaps_from_rollouts(
    rollouts: list[BattleExample],
    costs: dict[str, int],
    human_targets: dict[str, Any],
) -> dict[str, dict[str, float]]:
    x_ai = battles_to_features(rollouts, costs)
    idx = feature_name_index()
    tell_gaps: dict[str, dict[str, float]] = {}
    for name in STYLE_MATCH_FEATURES:
        if name not in idx:
            continue
        j = idx[name]
        tell_gaps[name] = {
            "human": float(human_targets["feature_means"][j]),
            "ai": float(x_ai[:, j].mean()) if len(x_ai) else 0.0,
        }
    return tell_gaps


@torch.no_grad()
def eval_style_vs_humans(
    model,
    vocab,
    costs: dict[str, int],
    humans: list[BattleExample],
    device: torch.device,
    style_artifact: dict[str, Any],
    human_targets: dict[str, Any],
    clock: ActionClockModels | None,
    *,
    n_battles: int = 96,
    seed: int = 0,
    max_context: int = 64,
    threat_dim: int = 14,
    temperature: float = 0.8,
    include_alternation: bool = True,
) -> dict[str, Any]:
    """Score policy rollouts under clock (deploy) and optional alternation protocols."""
    from .action_clock import rollout_policy_battles_with_clock
    from .policy_train import rollout_policy_battles

    model.eval()

    def pack(rolls: list[BattleExample]) -> dict[str, Any]:
        full = style_detection_metrics(
            style_artifact,
            humans,
            rolls,
            costs,
            human_means=human_targets["feature_means"],
            neutralize_harness=False,
        )
        free = style_detection_metrics(
            style_artifact,
            humans,
            rolls,
            costs,
            human_means=human_targets["feature_means"],
            neutralize_harness=True,
        )
        return {
            "full": full,
            "harness_free": free,
            "tell_gaps": _tell_gaps_from_rollouts(rolls, costs, human_targets),
            "n_rollouts": len(rolls),
        }

    if clock is not None:
        primary = rollout_policy_battles_with_clock(
            model,
            vocab,
            costs,
            humans,
            device,
            clock,
            n_battles=n_battles,
            seed=seed,
            max_context=max_context,
            threat_dim=threat_dim,
            temperature=temperature,
            use_clock_actor=True,
            use_clock_delay=False,
        )
        protocol = "clock_actor"
    else:
        primary = rollout_policy_battles(
            model,
            vocab,
            costs,
            humans,
            device,
            n_battles=n_battles,
            seed=seed,
            max_context=max_context,
            threat_dim=threat_dim,
            temperature=temperature,
        )
        protocol = "alternation"

    clock_pack = pack(primary)
    alt_pack: dict[str, Any] | None = None
    if include_alternation:
        if protocol == "alternation":
            alt_pack = clock_pack
        else:
            alt_rollouts = rollout_policy_battles(
                model,
                vocab,
                costs,
                humans,
                device,
                n_battles=n_battles,
                seed=seed + 17,
                max_context=max_context,
                threat_dim=threat_dim,
                temperature=temperature,
            )
            alt_pack = pack(alt_rollouts)

    return {
        "protocol": protocol,
        "full": clock_pack["full"],
        "harness_free": clock_pack["harness_free"],
        "tell_gaps": clock_pack["tell_gaps"],
        "n_rollouts": clock_pack["n_rollouts"],
        "alternation": alt_pack,
        "clock": clock_pack,
        "feature_l2": float(clock_pack["full"].get("feature_l2", 0.0)),
        "alt_feature_l2": float((alt_pack or {}).get("full", {}).get("feature_l2", 0.0)),
    }
