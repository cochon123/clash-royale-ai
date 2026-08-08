"""Experiment A — Exact-hand reconstruction audit vs oldest-four heuristic."""

from __future__ import annotations

import json
import pickle
import time
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import numpy as np
import torch

from .defense_slice_eval import FOCUS_THREATS, mine_reaction_windows
from .defense_support_audit import AUDIT_CELLS, CHEAP_ALTS
from .hand_tracker import (
    DECK_SIZE,
    bucket_play_count,
    heuristic_hand_mask,
    side_play_count_before,
    track_battle,
    track_side,
)
from .policy_dataset import (
    DEFAULT_MIN_CONTEXT,
    deck_slot_for_card,
    encode_policy_sample,
)
from .policy_infer import load_policy
from .winner_dataset import (
    BattleExample,
    collect_battles,
    load_card_costs,
    split_battles,
)

TRUE_IN_HAND_THRESHOLD = 0.99
MASK_THRESHOLD = 0.5
SUPPORT_FOCUS = (("graveyard", "poison"), ("hog-rider", "tornado"))


def _load_battles(
    input_dir: str | Path,
    cache_path: str | Path | None = None,
    min_card_plays: int = 12,
    allow_stale_cache: bool = True,
) -> list[BattleExample]:
    """Load battles; optionally accept a stale pickle while collection grows."""
    source = Path(input_dir)
    cache_file = (
        Path(cache_path)
        if cache_path is not None
        else source.parent / "winner_battles_cache.pkl"
    )
    if allow_stale_cache and cache_file.exists():
        try:
            with cache_file.open("rb") as handle:
                cached = pickle.load(handle)
            if (
                isinstance(cached, dict)
                and isinstance(cached.get("battles"), list)
                and cached.get("min_card_plays", min_card_plays) == min_card_plays
            ):
                battles = cached["battles"]
                print(
                    f"Loaded {len(battles)} battles from cache "
                    f"({cache_file}, stale_ok={allow_stale_cache})",
                    flush=True,
                )
                return battles
        except Exception as exc:
            print(f"Cache load failed ({exc}); rebuilding ...", flush=True)
    return collect_battles(
        input_dir, min_card_plays=min_card_plays, cache_path=cache_path
    )


def _prf(tp: int, fp: int, fn: int) -> dict[str, float]:
    prec = tp / (tp + fp) if (tp + fp) else 0.0
    rec = tp / (tp + fn) if (tp + fn) else 0.0
    f1 = (
        2 * prec * rec / (prec + rec)
        if (prec + rec)
        else 0.0
    )
    return {
        "precision": prec,
        "recall": rec,
        "f1": f1,
        "tp": tp,
        "fp": fp,
        "fn": fn,
        "support_true": tp + fn,
        "support_pred": tp + fp,
    }


def _quality_chunk(
    battles: list[BattleExample],
    costs: dict[str, int],
    true_threshold: float,
) -> dict[str, Any]:
    """Worker-safe chunk accumulator for heuristic quality."""
    buckets = ("0-3", "4-7", "8+", "all")
    stats = {
        b: {"tp": 0, "fp": 0, "fn": 0, "events": 0, "active_events": 0}
        for b in buckets
    }
    agree_num = {b: 0 for b in buckets}
    agree_den = {b: 0 for b in buckets}
    trackable_events = 0
    untrackable_events = 0
    sides_total = 0
    sides_untrackable = 0
    n_consistent: list[int] = []

    for battle in battles:
        tracks = track_battle(battle, mode="smoothed")
        for side, track in tracks.items():
            if not track.play_slots:
                continue
            sides_total += 1
            if not track.trackable:
                sides_untrackable += 1
                untrackable_events += len(track.play_slots)
                continue
            n_consistent.append(track.n_consistent)
            for row, event_index in enumerate(track.event_indices):
                trackable_events += 1
                gt = track.smoothed[row] >= true_threshold
                heur = heuristic_hand_mask(battle, event_index, costs, side)
                n_plays = side_play_count_before(battle, event_index, side)
                bucket = bucket_play_count(n_plays)
                active = bool(heur.any())
                for key in (bucket, "all"):
                    stats[key]["events"] += 1
                    if active:
                        stats[key]["active_events"] += 1
                    tp = int(np.logical_and(heur, gt).sum())
                    fp = int(np.logical_and(heur, np.logical_not(gt)).sum())
                    fn = int(np.logical_and(np.logical_not(heur), gt).sum())
                    stats[key]["tp"] += tp
                    stats[key]["fp"] += fp
                    stats[key]["fn"] += fn
                    agree_num[key] += int((heur == gt).sum())
                    agree_den[key] += DECK_SIZE
    return {
        "stats": stats,
        "agree_num": agree_num,
        "agree_den": agree_den,
        "trackable_events": trackable_events,
        "untrackable_events": untrackable_events,
        "sides_total": sides_total,
        "sides_untrackable": sides_untrackable,
        "n_consistent": n_consistent,
    }


def measure_heuristic_quality(
    battles: list[BattleExample],
    costs: dict[str, int],
    true_threshold: float = TRUE_IN_HAND_THRESHOLD,
    max_battles: int | None = None,
    workers: int = 1,
) -> dict[str, Any]:
    """Precision/recall of oldest-four mask vs smoothed posterior ≥ threshold."""
    buckets = ("0-3", "4-7", "8+", "all")
    use = battles if max_battles is None else battles[:max_battles]
    t0 = time.time()

    if workers <= 1 or len(use) < 64:
        partials = [_quality_chunk(use, costs, true_threshold)]
    else:
        from concurrent.futures import ProcessPoolExecutor

        n = min(workers, max(1, len(use) // 32))
        chunk_size = (len(use) + n - 1) // n
        chunks = [use[i : i + chunk_size] for i in range(0, len(use), chunk_size)]
        print(
            f"  heuristic quality: {len(use)} battles on {len(chunks)} workers",
            flush=True,
        )
        with ProcessPoolExecutor(max_workers=len(chunks)) as pool:
            futures = [
                pool.submit(_quality_chunk, chunk, costs, true_threshold)
                for chunk in chunks
            ]
            partials = [f.result() for f in futures]

    stats = {
        b: {"tp": 0, "fp": 0, "fn": 0, "events": 0, "active_events": 0}
        for b in buckets
    }
    agree_num = defaultdict(int)
    agree_den = defaultdict(int)
    trackable_events = 0
    untrackable_events = 0
    sides_total = 0
    sides_untrackable = 0
    n_consistent: list[int] = []
    for part in partials:
        for key in buckets:
            for field in ("tp", "fp", "fn", "events", "active_events"):
                stats[key][field] += part["stats"][key][field]
            agree_num[key] += part["agree_num"][key]
            agree_den[key] += part["agree_den"][key]
        trackable_events += part["trackable_events"]
        untrackable_events += part["untrackable_events"]
        sides_total += part["sides_total"]
        sides_untrackable += part["sides_untrackable"]
        n_consistent.extend(part["n_consistent"])

    by_bucket = {}
    for key in buckets:
        prf = _prf(stats[key]["tp"], stats[key]["fp"], stats[key]["fn"])
        by_bucket[key] = {
            **prf,
            "events": stats[key]["events"],
            "active_heuristic_events": stats[key]["active_events"],
            "slot_agreement": (
                agree_num[key] / agree_den[key] if agree_den[key] else 0.0
            ),
        }

    return {
        "true_in_hand_threshold": true_threshold,
        "battles_scored": len(use),
        "sides_with_plays": sides_total,
        "untrackable_side_rate": (
            sides_untrackable / sides_total if sides_total else 0.0
        ),
        "untrackable_sides": sides_untrackable,
        "trackable_events": trackable_events,
        "untrackable_events": untrackable_events,
        "mean_n_consistent": float(np.mean(n_consistent)) if n_consistent else 0.0,
        "median_n_consistent": (
            float(np.median(n_consistent)) if n_consistent else 0.0
        ),
        "by_bucket": by_bucket,
        "seconds": time.time() - t0,
        "workers": workers,
    }


def _exact_mask_for_sample(
    tracks: dict[str, Any],
    battle: BattleExample,
    event_index: int,
    costs: dict[str, int],
    threshold: float = MASK_THRESHOLD,
) -> tuple[torch.Tensor, str]:
    target = battle.events[event_index]
    side = target["side"]
    track = tracks[side]
    fallback = heuristic_hand_mask(battle, event_index, costs, side)
    if not track.trackable:
        return torch.tensor(fallback, dtype=torch.bool), "heuristic_fallback"
    # Causal posterior before this play (no future leakage for scoring).
    row = track._row_by_event.get(event_index)
    if row is None:
        return torch.tensor(fallback, dtype=torch.bool), "heuristic_fallback"
    post = track.causal[row]
    # Opening / unconstrained: every slot is 0.5 — do not mark all 8 in-hand.
    if float(np.max(post) - np.min(post)) < 1e-9:
        return torch.tensor(fallback, dtype=torch.bool), "heuristic_fallback"
    mask = post >= threshold
    # Prefer a proper 4-card hand; if threshold is too loose/tight, fall back.
    n_on = int(mask.sum())
    if n_on == 0 or n_on > 6:
        return torch.tensor(fallback, dtype=torch.bool), "heuristic_fallback"
    return torch.tensor(mask, dtype=torch.bool), "exact"


@torch.no_grad()
def rescore_policy_masks(
    battles: list[BattleExample],
    costs: dict[str, int],
    policy_dir: str | Path,
    device_name: str | None = None,
    max_samples_per_battle: int = 40,
    stride: int = 2,
    seed: int = 42,
    mask_threshold: float = MASK_THRESHOLD,
) -> dict[str, Any]:
    """Rescore frozen policy with heuristic vs causal-exact hand masks."""
    model, vocab, cfg, device = load_policy(policy_dir, device_name=device_name)
    max_context = int(cfg.get("max_context", 64))
    threat_dim = int(cfg.get("threat_dim", 0))
    rng = np.random.default_rng(seed)

    totals = {
        "heuristic": {"top1": 0, "top3": 0, "n": 0},
        "exact": {"top1": 0, "top3": 0, "n": 0},
        "exact_only": {"top1": 0, "top3": 0, "n": 0},
    }
    by_bucket = {
        b: {
            "heuristic": {"top1": 0, "top3": 0, "n": 0},
            "exact": {"top1": 0, "top3": 0, "n": 0},
        }
        for b in ("0-3", "4-7", "8+")
    }
    defense_slice = {
        "heuristic": {"top1": 0, "top3": 0, "n": 0},
        "exact": {"top1": 0, "top3": 0, "n": 0},
    }
    n_exact_source = 0
    n_fallback_source = 0
    t0 = time.time()
    flush_every = 64

    # Pre-mine defense windows on this battle list for subset metrics.
    windows = mine_reaction_windows(
        battles, max_response_seconds=5.0, focus_threats=FOCUS_THREATS
    )
    defense_keys = {
        (w["battle"].battle_id, int(w["response_index"])) for w in windows
    }

    pending: list[dict[str, Any]] = []

    def _flush() -> None:
        nonlocal pending, n_exact_source, n_fallback_source
        if not pending:
            return
        from torch.nn.utils.rnn import pad_sequence

        continuous = pad_sequence(
            [p["continuous"] for p in pending], batch_first=True
        ).to(device)
        card_ids = pad_sequence(
            [p["card_ids"] for p in pending], batch_first=True
        ).to(device)
        team = torch.stack([p["team"] for p in pending]).to(device)
        opp = torch.stack([p["opp"] for p in pending]).to(device)
        glob = torch.stack([p["glob"] for p in pending]).to(device)
        slots_f = torch.stack([p["slot_feats"] for p in pending]).to(device)
        lengths = torch.tensor([p["length"] for p in pending], dtype=torch.long).to(
            device
        )
        heur = torch.stack([p["heur"] for p in pending]).to(device)
        exact = torch.stack([p["exact"] for p in pending]).to(device)
        # One forward for heuristic masks, one for exact (batched).
        out_h = model(continuous, card_ids, team, opp, glob, lengths, slots_f, heur)
        out_e = model(continuous, card_ids, team, opp, glob, lengths, slots_f, exact)
        logits_h = out_h["slot_logits"].cpu()
        logits_e = out_e["slot_logits"].cpu()
        for i, p in enumerate(pending):
            target_slot = int(p["slot"])
            for name, logits in (("heuristic", logits_h[i]), ("exact", logits_e[i])):
                pred = int(logits.argmax().item())
                top3 = set(int(x) for x in logits.topk(3).indices.tolist())
                top1 = int(pred == target_slot)
                top3_hit = int(target_slot in top3)
                totals[name]["top1"] += top1
                totals[name]["top3"] += top3_hit
                totals[name]["n"] += 1
                bucket = p["bucket"]
                by_bucket[bucket][name]["top1"] += top1
                by_bucket[bucket][name]["top3"] += top3_hit
                by_bucket[bucket][name]["n"] += 1
                if p["is_defense"]:
                    defense_slice[name]["top1"] += top1
                    defense_slice[name]["top3"] += top3_hit
                    defense_slice[name]["n"] += 1
                if name == "exact" and p["source"] == "exact":
                    totals["exact_only"]["top1"] += top1
                    totals["exact_only"]["top3"] += top3_hit
                    totals["exact_only"]["n"] += 1
            if p["source"] == "exact":
                n_exact_source += 1
            else:
                n_fallback_source += 1
        pending = []

    for bi, battle in enumerate(battles):
        tracks = track_battle(battle, mode="causal")
        candidates = list(
            range(DEFAULT_MIN_CONTEXT, len(battle.events), stride)
        )
        if max_samples_per_battle is not None and len(candidates) > max_samples_per_battle:
            pick = rng.choice(
                len(candidates), size=max_samples_per_battle, replace=False
            )
            candidates = [candidates[int(i)] for i in sorted(pick)]

        for event_index in candidates:
            target = battle.events[event_index]
            if target["event_type"] != "card_play":
                continue
            sample = encode_policy_sample(
                battle,
                event_index,
                vocab,
                costs,
                max_context=max_context,
                threat_dim=threat_dim,
            )
            if sample is None:
                continue
            (
                continuous,
                card_ids,
                team_deck_t,
                opp_deck_t,
                global_feat,
                slot_feats,
                heur_mask,
                slot_t,
                _type,
                _zone,
                _xy,
                _timing,
                length,
            ) = sample
            exact_mask, source = _exact_mask_for_sample(
                tracks, battle, event_index, costs, threshold=mask_threshold
            )
            side = target["side"]
            n_plays = side_play_count_before(battle, event_index, side)
            pending.append(
                {
                    "continuous": continuous,
                    "card_ids": card_ids,
                    "team": team_deck_t,
                    "opp": opp_deck_t,
                    "glob": global_feat,
                    "slot_feats": slot_feats,
                    "heur": heur_mask,
                    "exact": exact_mask,
                    "slot": int(slot_t.item()),
                    "length": int(length.item())
                    if hasattr(length, "item")
                    else int(length),
                    "bucket": bucket_play_count(n_plays),
                    "is_defense": (battle.battle_id, event_index) in defense_keys,
                    "source": source,
                }
            )
            if len(pending) >= flush_every:
                _flush()

        if (bi + 1) % 200 == 0:
            _flush()
            print(
                f"  rescore {bi + 1}/{len(battles)} "
                f"({time.time() - t0:.1f}s)",
                flush=True,
            )

    _flush()

    def _rate(block: dict[str, int]) -> dict[str, float | int]:
        n = max(int(block["n"]), 1)
        return {
            "n": int(block["n"]),
            "slot_top1": block["top1"] / n,
            "slot_top3": block["top3"] / n,
        }

    heur = _rate(totals["heuristic"])
    exact = _rate(totals["exact"])
    return {
        "model_dir": str(policy_dir),
        "device": str(device),
        "mask_threshold": mask_threshold,
        "max_samples_per_battle": max_samples_per_battle,
        "stride": stride,
        "n_mask_exact": n_exact_source,
        "n_mask_fallback": n_fallback_source,
        "overall": {
            "heuristic": heur,
            "exact": exact,
            "delta_top1_pp": 100.0
            * (float(exact["slot_top1"]) - float(heur["slot_top1"])),
            "delta_top3_pp": 100.0
            * (float(exact["slot_top3"]) - float(heur["slot_top3"])),
        },
        "exact_source_only": _rate(totals["exact_only"]),
        "by_bucket": {
            b: {
                "heuristic": _rate(by_bucket[b]["heuristic"]),
                "exact": _rate(by_bucket[b]["exact"]),
                "delta_top1_pp": 100.0
                * (
                    _rate(by_bucket[b]["exact"])["slot_top1"]
                    - _rate(by_bucket[b]["heuristic"])["slot_top1"]
                ),
            }
            for b in by_bucket
        },
        "defense_slice": {
            "heuristic": _rate(defense_slice["heuristic"]),
            "exact": _rate(defense_slice["exact"]),
            "delta_top1_pp": 100.0
            * (
                _rate(defense_slice["exact"])["slot_top1"]
                - _rate(defense_slice["heuristic"])["slot_top1"]
            ),
        },
        "seconds": time.time() - t0,
    }


def _answer_in_exact_hand(
    track,
    response_index: int,
    answer_slot: int,
    threshold: float = MASK_THRESHOLD,
) -> bool:
    if not track.trackable:
        return False
    row = track._row_by_event.get(response_index)
    if row is None:
        return False
    return bool(track.causal[row, answer_slot] >= threshold)


@torch.no_grad()
def rescore_support_cells(
    train: list[BattleExample],
    test: list[BattleExample],
    costs: dict[str, int],
    policy_dir: str | Path,
    device_name: str | None = None,
    max_response_seconds: float = 5.0,
    mask_threshold: float = MASK_THRESHOLD,
) -> dict[str, Any]:
    """Compare support-audit cell counts under heuristic vs exact hands."""
    model, vocab, cfg, device = load_policy(policy_dir, device_name=device_name)
    max_context = int(cfg.get("max_context", 64))
    threat_dim = int(cfg.get("threat_dim", 0))

    print("Mining support-audit reaction windows ...", flush=True)
    train_windows = mine_reaction_windows(
        train, max_response_seconds=max_response_seconds, focus_threats=FOCUS_THREATS
    )
    test_windows = mine_reaction_windows(
        test, max_response_seconds=max_response_seconds, focus_threats=FOCUS_THREATS
    )

    # Cache tracks per battle_id
    track_cache: dict[str, dict[str, Any]] = {}

    def _tracks_for(battle: BattleExample):
        key = battle.battle_id
        if key not in track_cache:
            track_cache[key] = track_battle(battle, mode="causal")
        return track_cache[key]

    def _defender_deck(battle: BattleExample, defender: str) -> tuple[str, ...]:
        return battle.team_deck if defender == "team" else battle.opponent_deck

    def _in_hand_heuristic(battle, response_index, defender, answer) -> bool:
        mask = heuristic_hand_mask(battle, response_index, costs, defender)
        if not bool(mask.any()):
            return False
        slot = deck_slot_for_card(_defender_deck(battle, defender), answer)
        return False if slot is None else bool(mask[slot])

    def _in_hand_exact(battle, response_index, defender, answer) -> bool:
        tracks = _tracks_for(battle)
        track = tracks[defender]
        deck = _defender_deck(battle, defender)
        slot = deck_slot_for_card(deck, answer)
        if slot is None:
            return False
        if not track.trackable:
            return _in_hand_heuristic(battle, response_index, defender, answer)
        return _answer_in_exact_hand(track, response_index, slot, mask_threshold)

    cells_out: list[dict[str, Any]] = []
    focus_set = set(SUPPORT_FOCUS)
    for threat, answer, role in AUDIT_CELLS:
        if (threat, answer) not in focus_set and role != "failing":
            # Still compute all AUDIT_CELLS for completeness; cheap.
            pass
        train_threat = [w for w in train_windows if w["threat"] == threat]
        test_threat = [w for w in test_windows if w["threat"] == threat]

        def _subset(windows, in_hand_fn):
            in_deck = [
                w
                for w in windows
                if answer in _defender_deck(w["battle"], w["defender"])
            ]
            in_hand = [
                w
                for w in in_deck
                if in_hand_fn(
                    w["battle"], w["response_index"], w["defender"], answer
                )
            ]
            human_hand = [w for w in in_hand if w["response"] == answer]
            return in_deck, in_hand, human_hand

        tr_deck_h, tr_hand_h, tr_human_h = _subset(train_threat, _in_hand_heuristic)
        tr_deck_e, tr_hand_e, tr_human_e = _subset(train_threat, _in_hand_exact)
        te_deck_h, te_hand_h, te_human_h = _subset(test_threat, _in_hand_heuristic)
        te_deck_e, te_hand_e, te_human_e = _subset(test_threat, _in_hand_exact)

        def _score(windows_subset):
            scored = []
            for w in windows_subset:
                battle = w["battle"]
                j = int(w["response_index"])
                sample = encode_policy_sample(
                    battle,
                    j,
                    vocab,
                    costs,
                    max_context=max_context,
                    threat_dim=threat_dim,
                )
                if sample is None:
                    continue
                tracks = _tracks_for(battle)
                exact_mask, _src = _exact_mask_for_sample(
                    tracks, battle, j, costs, threshold=mask_threshold
                )
                (
                    continuous,
                    card_ids,
                    team_deck_t,
                    opp_deck_t,
                    global_feat,
                    slot_feats,
                    _heur,
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
                    exact_mask.unsqueeze(0).to(device),
                )
                probs = torch.softmax(out["slot_logits"][0], dim=-1).cpu().numpy()
                deck = _defender_deck(battle, w["defender"])
                answer_slot = deck_slot_for_card(deck, answer)
                if answer_slot is None:
                    continue
                pred = int(probs.argmax())
                scored.append(
                    {
                        "top1": pred == answer_slot,
                        "P_answer": float(probs[answer_slot]),
                        "human": w["response"] == answer,
                    }
                )
            return scored

        # Score test natural-hand (exact) when human chose answer.
        scored_human = _score([w for w in te_hand_e if w["response"] == answer])
        scored_all = _score(te_hand_e)
        n_h = len(tr_hand_h)
        n_e = len(tr_hand_e)
        rel = ((n_e - n_h) / n_h) if n_h else (1.0 if n_e else 0.0)
        cells_out.append(
            {
                "threat": threat,
                "answer": answer,
                "role": role,
                "train": {
                    "heuristic": {
                        "n_in_deck": len(tr_deck_h),
                        "n_in_hand": n_h,
                        "n_human_in_hand": len(tr_human_h),
                        "human_rate_in_hand": (
                            len(tr_human_h) / n_h if n_h else 0.0
                        ),
                    },
                    "exact": {
                        "n_in_deck": len(tr_deck_e),
                        "n_in_hand": n_e,
                        "n_human_in_hand": len(tr_human_e),
                        "human_rate_in_hand": (
                            len(tr_human_e) / n_e if n_e else 0.0
                        ),
                    },
                    "n_hand_rel_change": rel,
                    "n_hand_abs_change": n_e - n_h,
                },
                "test": {
                    "heuristic": {
                        "n_in_hand": len(te_hand_h),
                        "n_human_in_hand": len(te_human_h),
                    },
                    "exact": {
                        "n_in_hand": len(te_hand_e),
                        "n_human_in_hand": len(te_human_e),
                        "model_top1_when_human": (
                            float(np.mean([s["top1"] for s in scored_human]))
                            if scored_human
                            else None
                        ),
                        "model_top1_in_hand": (
                            float(np.mean([s["top1"] for s in scored_all]))
                            if scored_all
                            else None
                        ),
                        "n_scored_when_human": len(scored_human),
                        "n_scored_in_hand": len(scored_all),
                    },
                },
            }
        )
        print(
            f"  {threat}→{answer}: train n_hand heur={n_h} exact={n_e} "
            f"rel={rel:+.1%}",
            flush=True,
        )

    focus_rows = [
        c
        for c in cells_out
        if (c["threat"], c["answer"]) in focus_set
    ]
    return {
        "mask_threshold": mask_threshold,
        "cells": cells_out,
        "focus_cells": focus_rows,
        "focus_max_abs_rel_change": max(
            (abs(c["train"]["n_hand_rel_change"]) for c in focus_rows),
            default=0.0,
        ),
    }


def _decide_verdict(
    quality: dict[str, Any],
    rescore: dict[str, Any],
    support: dict[str, Any],
) -> dict[str, Any]:
    delta_all = float(rescore["overall"]["delta_top1_pp"])
    delta_def = float(rescore["defense_slice"]["delta_top1_pp"])
    rel_support = float(support.get("focus_max_abs_rel_change", 0.0))

    pass_slot = delta_all >= 2.0
    pass_def = delta_def >= 5.0
    pass_support = rel_support >= 0.15

    recall_4_7 = float(quality["by_bucket"]["4-7"]["recall"])
    recall_8 = float(quality["by_bucket"]["8+"]["recall"])
    fail_small = (
        abs(delta_all) < 0.5
        and abs(delta_def) < 0.5
        and rel_support < 0.15
    )
    fail_recall = recall_4_7 >= 0.95 and recall_8 >= 0.95
    failed = fail_small and fail_recall
    passed = pass_slot or pass_def or pass_support

    if passed and not failed:
        status = "PASS"
    elif failed:
        status = "FAIL"
    elif passed and failed:
        status = "PASS"  # success criteria take priority if both somehow fire
    else:
        status = "INCONCLUSIVE"

    return {
        "status": status,
        "pass_slot_top1_ge_2pp": pass_slot,
        "pass_defense_slice_ge_5pp": pass_def,
        "pass_support_n_rel_ge_15pct": pass_support,
        "fail_small_deltas": fail_small,
        "fail_high_recall_4plus": fail_recall,
        "delta_slot_top1_pp": delta_all,
        "delta_defense_slice_top1_pp": delta_def,
        "support_focus_max_abs_rel_change": rel_support,
        "recall_4_7": recall_4_7,
        "recall_8_plus": recall_8,
    }


def run_hand_audit(
    input_dir: str | Path = "data/raw",
    policy_dir: str | Path = "models/policy_bc_v3",
    card_costs_path: str | Path = "data/card_costs.json",
    output_json: str | Path = "reports/hand_audit_v1.json",
    output_html: str | Path = "reports/hand_audit_v1.html",
    seed: int = 42,
    device_name: str | None = "cpu",
    max_quality_battles: int | None = None,
    allow_stale_cache: bool = True,
    cache_path: str | Path | None = None,
    quality_workers: int = 4,
) -> dict[str, Any]:
    t_all = time.time()
    costs = load_card_costs(card_costs_path)
    battles = _load_battles(
        input_dir, cache_path=cache_path, allow_stale_cache=allow_stale_cache
    )
    train, val, test = split_battles(battles, seed=seed)

    # Snapshot model metadata
    model_dir = Path(policy_dir)
    model_meta: dict[str, Any] = {"path": str(model_dir)}
    report_path = model_dir / "report.json"
    if report_path.exists():
        with report_path.open(encoding="utf-8") as handle:
            model_meta.update(
                {
                    k: v
                    for k, v in json.load(handle).items()
                    if k
                    in {
                        "model_name",
                        "model_version",
                        "created_at",
                        "compute",
                        "data",
                        "test",
                    }
                }
            )

    print("=== 1/3 Heuristic quality vs exact posterior ===", flush=True)
    quality = measure_heuristic_quality(
        battles,
        costs,
        max_battles=max_quality_battles,
        workers=quality_workers,
    )
    print(
        json.dumps(
            {
                "untrackable_side_rate": quality["untrackable_side_rate"],
                "recall": {
                    b: quality["by_bucket"][b]["recall"]
                    for b in ("0-3", "4-7", "8+")
                },
            },
            indent=2,
        ),
        flush=True,
    )

    print("=== 2/3 Rescore policy_bc_v3 (test split) ===", flush=True)
    rescore = rescore_policy_masks(
        test,
        costs,
        policy_dir=policy_dir,
        device_name=device_name,
        seed=seed,
    )
    print(
        json.dumps(rescore["overall"], indent=2)
        + "\ndefense "
        + json.dumps(rescore["defense_slice"], indent=2),
        flush=True,
    )

    print("=== 3/3 Support-audit cells (exact hands) ===", flush=True)
    support = rescore_support_cells(
        train,
        test,
        costs,
        policy_dir=policy_dir,
        device_name=device_name,
    )

    verdict = _decide_verdict(quality, rescore, support)
    lessons = [
        "Oldest-four is disabled for <4 side plays, so early-game masks are empty while exact posteriors already peak on the imminent cycle cards.",
        "Even with a full play sequence, many initial queues remain consistent (~6k); hand membership is only sharp after the cycle has been observed.",
        "Rescoring uses causal (prefix-only) posteriors at ≥0.5 so the mask is deployable without future leakage; quality GT uses smoothed ≥0.99.",
        "If slot top-1 deltas stay <0.5pp and 4+/8+ heuristic recall is already high, hand-label noise is not the main policy bottleneck — look elsewhere.",
        "Offline only: exact reconstruction audits labels; it does not claim live-play readiness.",
    ]

    report = {
        "experiment": "A_exact_hand_reconstruction_audit",
        "created_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "model_name": model_meta.get("model_name", model_dir.name),
        "model_version": model_meta.get("model_version", "unknown"),
        "model": model_meta,
        "compute": {
            "device": device_name or "cpu",
            "note": "CPU-preferred audit; no retraining",
            "quality_seconds": quality["seconds"],
            "rescore_seconds": rescore["seconds"],
            "total_seconds": time.time() - t_all,
        },
        "data": {
            "battles_total": len(battles),
            "train": len(train),
            "val": len(val),
            "test": len(test),
            "split_seed": seed,
            "quality_battles": quality["battles_scored"],
        },
        "setup": {
            "hypothesis": (
                "The oldest-four hand heuristic mislabels card availability "
                "early-game, distorting slot accuracy and support-audit cell counts."
            ),
            "true_in_hand_threshold": TRUE_IN_HAND_THRESHOLD,
            "rescore_mask_threshold": MASK_THRESHOLD,
            "cycle_model": "8-card queue, hand=first 4, played→back",
            "posterior_quality": "smoothed over initials consistent with full side sequence",
            "posterior_rescore": "causal prefix-only (no future plays)",
            "soft_mask": -8.0,
        },
        "heuristic_quality": quality,
        "policy_rescore": rescore,
        "support_audit": support,
        "verdict": verdict,
        "lessons": lessons,
        "success_criteria": {
            "pass_if": [
                "≥2pp overall slot top-1",
                "OR ≥5pp defense-slice subset",
                "OR ≥15% relative change in gated support-audit cell n",
            ],
            "fail_if": [
                "<0.5pp everywhere AND heuristic recall ≥95% in 4+ plays buckets",
            ],
        },
    }

    out_json = Path(output_json)
    out_json.parent.mkdir(parents=True, exist_ok=True)
    with out_json.open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2)
    print(f"Wrote {out_json}", flush=True)

    from .hand_audit_report import render_hand_audit_report

    out_html = render_hand_audit_report(out_json, output_html)
    print(f"Wrote {out_html}", flush=True)
    print(json.dumps({"verdict": verdict}, indent=2), flush=True)
    return report
