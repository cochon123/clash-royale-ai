"""Offline policy battle royale judged by the winner predictor.

Every policy checkpoint plays every other policy. The winner model scores each
game; only decisions with calibrated confidence >= threshold count toward the
standings. No live play.
"""

from __future__ import annotations

import itertools
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

from .matchup_eval import _bootstrap_opening
from .policy_dataset import encode_policy_sample
from .policy_infer import load_policy
from .policy_model import PolicyBC
from .winner_dataset import (
    BattleExample,
    CardVocab,
    collect_battles,
    load_card_costs,
    split_battles,
)
from .winner_tabular import extract_tabular_features, swap_battle_perspective

DEFAULT_POLICIES = (
    "models/policy_bc",
    "models/policy_bc_v3",
    "models/policy_bc_v4",
    "models/policy_bc_v4.1",
    "models/policy_bc_v5",
)


@dataclass
class LoadedPolicy:
    policy_id: str
    model_dir: Path
    model: PolicyBC
    vocab: CardVocab
    cfg: dict[str, Any]
    report: dict[str, Any]
    max_context: int
    threat_dim: int


def _policy_id(model_dir: Path) -> str:
    report_path = model_dir / "report.json"
    if report_path.exists():
        with report_path.open(encoding="utf-8") as handle:
            name = json.load(handle).get("model_name")
        if name:
            return str(name)
    return model_dir.name


def load_policies(
    policy_dirs: list[str | Path],
    device_name: str | None = None,
) -> tuple[list[LoadedPolicy], torch.device]:
    loaded: list[LoadedPolicy] = []
    device: torch.device | None = None
    for raw in policy_dirs:
        model_dir = Path(raw)
        model, vocab, cfg, dev = load_policy(model_dir, device_name=device_name)
        if device is None:
            device = dev
        report: dict[str, Any] = {}
        report_path = model_dir / "report.json"
        if report_path.exists():
            with report_path.open(encoding="utf-8") as handle:
                report = json.load(handle)
        loaded.append(
            LoadedPolicy(
                policy_id=_policy_id(model_dir),
                model_dir=model_dir,
                model=model,
                vocab=vocab,
                cfg=cfg,
                report=report,
                max_context=int(cfg.get("max_context", 64)),
                threat_dim=int(cfg.get("threat_dim", 0) or 0),
            )
        )
    if device is None:
        raise RuntimeError("No policies loaded")
    return loaded, device


@torch.no_grad()
def heterogeneous_policy_game(
    team_policy: LoadedPolicy,
    opp_policy: LoadedPolicy,
    costs: dict[str, int],
    team_deck: tuple[str, ...],
    opponent_deck: tuple[str, ...],
    device: torch.device,
    rng: random.Random,
    warmup_events: int = 10,
    max_new_events: int = 50,
    temperature: float = 0.85,
) -> BattleExample:
    """Play one offline game: team seat uses team_policy, opponent uses opp_policy."""
    events = _bootstrap_opening(
        team_deck, opponent_deck, costs, rng, n_events=warmup_events
    )
    seconds = float(events[-1]["seconds"])
    next_side = "team" if events[-1]["side"] == "opponent" else "opponent"

    for _ in range(max_new_events):
        acting = team_policy if next_side == "team" else opp_policy
        acting_deck = team_deck if next_side == "team" else opponent_deck
        dummy_card = acting_deck[0]
        dummy = {
            "seconds": seconds + 1.0,
            "side": next_side,
            "event_type": "card_play",
            "card": dummy_card,
            "x": 9000,
            "y": 8000 if next_side == "team" else 24000,
        }
        probe = BattleExample(
            battle_id="royale-probe",
            team_deck=team_deck,
            opponent_deck=opponent_deck,
            team_wins=0,
            events=tuple(events) + (dummy,),
        )
        sample = encode_policy_sample(
            probe,
            len(events),
            acting.vocab,
            costs,
            max_context=acting.max_context,
            threat_dim=acting.threat_dim,
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
        out = acting.model(
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
        if getattr(acting.model, "placement_card_mode", "soft") == "selected":
            out = acting.model(
                continuous.unsqueeze(0).to(device), card_ids.unsqueeze(0).to(device),
                team_deck_t.unsqueeze(0).to(device), opp_deck_t.unsqueeze(0).to(device),
                global_feat.unsqueeze(0).to(device), length.unsqueeze(0).to(device),
                slot_feats.unsqueeze(0).to(device), hand_mask.unsqueeze(0).to(device),
                placement_slots=torch.tensor([slot], device=device),
            )
        card = acting_deck[slot]
        event_type = (
            "ability_activation"
            if int(out["type_logits"][0].argmax().item()) == 1
            else "card_play"
        )
        xy = out["xy"][0].cpu().numpy()
        if getattr(acting.model, "placement_card_mode", "soft") == "selected" and out.get("tile_logits") is not None:
            tile_probs = torch.softmax(out["tile_logits"][0] / 0.6, dim=-1).cpu().numpy()
            candidates = np.argsort(tile_probs)[-5:]
            tile = int(rng.choices(candidates.tolist(), weights=tile_probs[candidates].tolist(), k=1)[0])
            row, col = divmod(tile, 32)
            xy = np.asarray([(col + 0.5) / 32.0, (row + 0.5) / 18.0])
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
        battle_id="royale-game",
        team_deck=team_deck,
        opponent_deck=opponent_deck,
        team_wins=0,
        events=tuple(events),
    )


def _score_battles(
    winner_artifact: dict[str, Any],
    battles: list[BattleExample],
    costs: dict[str, int],
) -> tuple[np.ndarray, np.ndarray]:
    """Return (P(team wins), calibrated confidence P(prediction correct))."""
    card_index = winner_artifact["card_index"]
    hgb = winner_artifact["models"]["hist_gradient_boosting"]
    trees = winner_artifact["models"]["extra_trees"]
    w = float(winner_artifact["hgb_weight"])
    cw = float(winner_artifact["confidence_hgb_weight"])
    calibrator = winner_artifact["confidence_calibrator"]
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
    hgb_direct = hgb.predict_proba(feats)[:, 1]
    tree_direct = trees.predict_proba(feats)[:, 1]
    hgb_rev = hgb.predict_proba(swapped)[:, 1]
    tree_rev = trees.predict_proba(swapped)[:, 1]
    direct = w * hgb_direct + (1.0 - w) * tree_direct
    reverse = 1.0 - (w * hgb_rev + (1.0 - w) * tree_rev)
    p_team = (direct + reverse) / 2.0

    # Confidence head uses its own blend, then isotonic P(correct).
    conf_direct = cw * hgb_direct + (1.0 - cw) * tree_direct
    conf_rev = cw * hgb_rev + (1.0 - cw) * tree_rev
    # Symmetric margin: how far the averaged prediction sits from a coin flip.
    p_conf = (conf_direct + (1.0 - conf_rev)) / 2.0
    raw_score = np.abs(p_conf - 0.5)
    confidence = np.asarray(calibrator.predict(raw_score), dtype=np.float64)
    return p_team.astype(np.float64), confidence


def _elo_update(
    ratings: dict[str, float],
    winner: str,
    loser: str,
    k: float = 24.0,
) -> None:
    ra = ratings[winner]
    rb = ratings[loser]
    ea = 1.0 / (1.0 + 10 ** ((rb - ra) / 400.0))
    eb = 1.0 - ea
    ratings[winner] = ra + k * (1.0 - ea)
    ratings[loser] = rb + k * (0.0 - eb)


def _write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    with tmp.open("w", encoding="utf-8") as handle:
        json.dump(payload, handle, indent=2)
    tmp.replace(path)


def _append_progress(path: Path, row: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(row, separators=(",", ":")) + "\n")


def run_battle_royale(
    policy_dirs: list[str | Path] | None = None,
    input_dir: str | Path = "data/raw",
    winner_dir: str | Path = "models/winner_predictor",
    card_costs_path: str | Path = "data/card_costs.json",
    output_path: str | Path = "reports/battle_royale.json",
    html_output: str | Path | None = "reports/battle_royale.html",
    games_per_pair: int = 48,
    min_confidence: float = 0.80,
    seed: int = 42,
    device_name: str | None = None,
    warmup_events: int = 10,
    max_new_events: int = 50,
    temperature: float = 0.85,
) -> dict[str, Any]:
    started = time.time()
    policy_dirs = list(policy_dirs or DEFAULT_POLICIES)
    if len(policy_dirs) < 2:
        raise ValueError("Need at least two policies for a battle royale")

    out = Path(output_path)
    progress_path = out.with_name(out.stem + "_progress.jsonl")
    if progress_path.exists():
        progress_path.unlink()

    print("Loading policies ...", flush=True)
    policies, device = load_policies(policy_dirs, device_name=device_name)
    policy_ids = [p.policy_id for p in policies]
    print(f"Policies ({device}): {', '.join(policy_ids)}", flush=True)

    print("Loading battles / decks ...", flush=True)
    battles = collect_battles(input_dir)
    train, _val, _test = split_battles(battles, seed=42)
    deck_pool = [(b.team_deck, b.opponent_deck) for b in train if len(b.team_deck) == 8]
    if len(deck_pool) < 32:
        raise RuntimeError("Not enough train decks to sample matchups")

    costs = load_card_costs(card_costs_path)
    with Path(winner_dir, "hgb_ensemble.pkl").open("rb") as handle:
        winner = pickle.load(handle)

    rng = random.Random(seed)
    pairs = list(itertools.combinations(range(len(policies)), 2))
    total_games = len(pairs) * games_per_pair
    print(
        f"Battle royale: {len(pairs)} pairs × {games_per_pair} games "
        f"= {total_games} (keep conf>={min_confidence:.0%})",
        flush=True,
    )

    # Accumulator state
    wins = {pid: 0 for pid in policy_ids}
    losses = {pid: 0 for pid in policy_ids}
    played = {pid: 0 for pid in policy_ids}
    raw_wins = {pid: 0 for pid in policy_ids}
    raw_played = {pid: 0 for pid in policy_ids}
    elo = {pid: 1500.0 for pid in policy_ids}
    h2h: dict[str, dict[str, dict[str, int]]] = {
        a: {b: {"wins": 0, "losses": 0, "games": 0} for b in policy_ids if b != a}
        for a in policy_ids
    }
    game_rows: list[dict[str, Any]] = []
    pair_summaries: list[dict[str, Any]] = []
    confidences: list[float] = []
    kept_confidences: list[float] = []
    games_done = 0

    def snapshot(status: str = "running") -> dict[str, Any]:
        standings = []
        for pid in policy_ids:
            n = played[pid]
            wr = (wins[pid] / n) if n else None
            standings.append(
                {
                    "policy_id": pid,
                    "wins": wins[pid],
                    "losses": losses[pid],
                    "games": n,
                    "win_rate": wr,
                    "raw_games": raw_played[pid],
                    "raw_wins": raw_wins[pid],
                    "raw_win_rate": (
                        raw_wins[pid] / raw_played[pid] if raw_played[pid] else None
                    ),
                    "elo": elo[pid],
                    "model_dir": str(
                        next(p.model_dir for p in policies if p.policy_id == pid)
                    ),
                    "created_at": next(
                        (
                            p.report.get("created_at")
                            for p in policies
                            if p.policy_id == pid
                        ),
                        None,
                    ),
                    "data_battles": next(
                        (
                            (p.report.get("data") or {}).get("battles_total")
                            or (p.report.get("data") or {}).get("usable_battles")
                            for p in policies
                            if p.policy_id == pid
                        ),
                        None,
                    ),
                    "compute": next(
                        (p.report.get("compute") for p in policies if p.policy_id == pid),
                        None,
                    ),
                }
            )
        standings.sort(
            key=lambda row: (
                -(row["win_rate"] if row["win_rate"] is not None else -1.0),
                -row["elo"],
                -row["wins"],
            )
        )
        for rank, row in enumerate(standings, start=1):
            row["rank"] = rank
        coverage = (sum(played.values()) / 2) / max(games_done, 1)
        return {
            "status": status,
            "created_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "model_name": "policy-battle-royale-v1",
            "judge": "winner-predictor-hgb-symmetric",
            "seconds": round(time.time() - started, 1),
            "setup": {
                "policies": [
                    {
                        "policy_id": p.policy_id,
                        "model_dir": str(p.model_dir),
                        "threat_dim": p.threat_dim,
                        "card_conditioned_placement": bool(
                            p.cfg.get("card_conditioned_placement", False)
                        ),
                        "created_at": p.report.get("created_at"),
                        "data": p.report.get("data"),
                        "compute": p.report.get("compute"),
                        "test": p.report.get("test"),
                    }
                    for p in policies
                ],
                "games_per_pair": games_per_pair,
                "min_confidence": min_confidence,
                "warmup_events": warmup_events,
                "policy_events": max_new_events,
                "temperature": temperature,
                "seed": seed,
                "device": str(device),
                "deck_pool": len(deck_pool),
                "note": (
                    "Round-robin offline self-play. Each seat uses a different "
                    "policy checkpoint. Winners come from the offline winner "
                    "predictor; only games with calibrated confidence "
                    f">= {min_confidence:.0%} count in the standings."
                ),
            },
            "progress": {
                "games_done": games_done,
                "games_total": total_games,
                "confident_games": int(sum(played.values()) / 2),
                "coverage": coverage,
                "mean_confidence": float(np.mean(confidences)) if confidences else None,
                "mean_kept_confidence": (
                    float(np.mean(kept_confidences)) if kept_confidences else None
                ),
            },
            "standings": standings,
            "head_to_head": h2h,
            "pairs": pair_summaries,
            "champion": standings[0]["policy_id"] if standings and standings[0]["games"] else None,
            "verdict": None,
            "lessons": [
                "Filter by judge confidence, not raw win rate — the winner model is ~79% accurate overall but ~94% at conf≥80%.",
                "Heterogeneous seats matter: older policies without threat features still compete on the same decks.",
                "This is still sequence-level offline play judged by another model — not live Clash Royale.",
            ],
            "games_sample": game_rows[-40:],
        }

    for pair_idx, (i, j) in enumerate(pairs):
        a = policies[i]
        b = policies[j]
        pair_wins = {a.policy_id: 0, b.policy_id: 0}
        pair_kept = 0
        pair_raw = 0
        pending_battles: list[BattleExample] = []
        pending_meta: list[dict[str, Any]] = []

        def flush_batch() -> None:
            nonlocal pair_kept, pair_raw, games_done
            if not pending_battles:
                return
            p_team, confidence = _score_battles(winner, pending_battles, costs)
            for meta, p, conf in zip(pending_meta, p_team, confidence):
                games_done += 1
                pair_raw += 1
                team_id = meta["team_id"]
                opp_id = meta["opp_id"]
                team_won = bool(p >= 0.5)
                winner_id = team_id if team_won else opp_id
                loser_id = opp_id if team_won else team_id
                confidences.append(float(conf))
                raw_played[team_id] += 1
                raw_played[opp_id] += 1
                raw_wins[winner_id] += 1
                kept = float(conf) >= min_confidence
                row = {
                    "pair": f"{a.policy_id} vs {b.policy_id}",
                    "team": team_id,
                    "opponent": opp_id,
                    "p_team": float(p),
                    "confidence": float(conf),
                    "kept": kept,
                    "winner": winner_id if kept else None,
                    "raw_winner": winner_id,
                    "events": meta["events"],
                }
                game_rows.append(row)
                if kept:
                    pair_kept += 1
                    kept_confidences.append(float(conf))
                    wins[winner_id] += 1
                    losses[loser_id] += 1
                    played[winner_id] += 1
                    played[loser_id] += 1
                    pair_wins[winner_id] += 1
                    h2h[winner_id][loser_id]["wins"] += 1
                    h2h[winner_id][loser_id]["games"] += 1
                    h2h[loser_id][winner_id]["losses"] += 1
                    h2h[loser_id][winner_id]["games"] += 1
                    _elo_update(elo, winner_id, loser_id)
                _append_progress(
                    progress_path,
                    {
                        "t": datetime.now(timezone.utc).strftime("%H:%M:%S"),
                        "games_done": games_done,
                        "games_total": total_games,
                        "pair": row["pair"],
                        "confidence": row["confidence"],
                        "kept": kept,
                        "raw_winner": winner_id,
                        "standings": {
                            pid: {
                                "w": wins[pid],
                                "l": losses[pid],
                                "wr": (wins[pid] / played[pid] if played[pid] else None),
                                "elo": round(elo[pid], 1),
                            }
                            for pid in policy_ids
                        },
                    },
                )
            pending_battles.clear()
            pending_meta.clear()
            _write_json(out, snapshot("running"))

        for g in range(games_per_pair):
            team_deck, opp_deck = rng.choice(deck_pool)
            # Seat balance: alternate who sits on team.
            if g % 2 == 0:
                team_p, opp_p = a, b
            else:
                team_p, opp_p = b, a
            battle = heterogeneous_policy_game(
                team_p,
                opp_p,
                costs,
                team_deck,
                opp_deck,
                device,
                random.Random(rng.randint(0, 10**9)),
                warmup_events=warmup_events,
                max_new_events=max_new_events,
                temperature=temperature,
            )
            pending_battles.append(battle)
            pending_meta.append(
                {
                    "team_id": team_p.policy_id,
                    "opp_id": opp_p.policy_id,
                    "events": len(battle.events),
                }
            )
            if len(pending_battles) >= 16:
                flush_batch()
                print(
                    f"[{games_done}/{total_games}] {a.policy_id} vs {b.policy_id} "
                    f"kept={pair_kept}/{pair_raw} "
                    f"leader={snapshot()['champion']}",
                    flush=True,
                )

        flush_batch()
        a_kept_wins = pair_wins[a.policy_id]
        b_kept_wins = pair_wins[b.policy_id]
        pair_summaries.append(
            {
                "a": a.policy_id,
                "b": b.policy_id,
                "games": pair_raw,
                "confident_games": pair_kept,
                "a_wins": a_kept_wins,
                "b_wins": b_kept_wins,
                "a_win_rate": (a_kept_wins / pair_kept) if pair_kept else None,
                "coverage": pair_kept / pair_raw if pair_raw else 0.0,
            }
        )
        print(
            f"Pair done {a.policy_id} vs {b.policy_id}: "
            f"{a_kept_wins}-{b_kept_wins} on {pair_kept} confident / {pair_raw} raw",
            flush=True,
        )

    report = snapshot("complete")
    champion = report["champion"]
    if champion and report["standings"][0]["games"]:
        top = report["standings"][0]
        report["verdict"] = (
            f"{champion} wins the battle royale under the winner-predictor judge "
            f"at confidence ≥ {min_confidence:.0%}: "
            f"{top['wins']}-{top['losses']} "
            f"({(top['win_rate'] or 0):.1%} WR, Elo {top['elo']:.0f}) "
            f"across {top['games']} confident games."
        )
    else:
        report["verdict"] = (
            f"No confident games at threshold {min_confidence:.0%}; "
            "lower the threshold or play more games."
        )
    # Attach a compact confidence histogram for the report.
    if confidences:
        edges = np.linspace(0.5, 1.0, 11)
        hist, _ = np.histogram(confidences, bins=edges)
        report["confidence_hist"] = {
            "edges": edges.tolist(),
            "counts": hist.tolist(),
            "kept_fraction": float(np.mean(np.asarray(confidences) >= min_confidence)),
        }
    _write_json(out, report)
    print(report["verdict"], flush=True)
    print(f"Wrote {out}", flush=True)

    if html_output:
        from .battle_royale_report import write_battle_royale_report

        html_path = write_battle_royale_report(report, html_output)
        print(f"Wrote {html_path}", flush=True)
        report["html_report"] = str(html_path)

    return report
