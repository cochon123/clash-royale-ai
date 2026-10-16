"""Read-only architecture audit. Frozen models; no training, cache writes or games.

Run from the repository root. Outputs a self-contained evidence bundle and
battle-cluster bootstrap intervals. The report builder consumes this bundle.
"""
from __future__ import annotations

import argparse
from collections import Counter
from dataclasses import replace
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import pickle
import time

import numpy as np
import torch
import torch.nn.functional as F
from torch.utils.data import DataLoader

from cr_replay_pipeline.parser import parse_replay, ReplayParseError
from cr_replay_pipeline.policy_dataset import (
    PolicyActionDataset, collate_policy_batch, deck_slot_for_card,
    recent_opponent_threat,
)
from cr_replay_pipeline.policy_infer import load_policy
from cr_replay_pipeline.winner_dataset import BattleExample, load_card_costs


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def signature(b):
    return hashlib.sha256(json.dumps(
        [b.team_deck, b.opponent_deck, b.events], sort_keys=True,
        separators=(",", ":"),
    ).encode()).hexdigest()


def progress(phase, done, total, started):
    elapsed = time.monotonic() - started
    eta = elapsed / done * (total - done) if done else float("nan")
    print(f"{phase:<24} {done:>7}/{total:<7} {elapsed:>8.1f}s {eta:>9.1f}s", flush=True)


def fresh_battles(limit):
    started = time.monotonic()
    known, fingerprints = set(), set()
    audit = {"excluded_caches": {}, "manifest_overlap": {}, "ability_counts": {}}
    for source in ("data/policy_battles_cache.pkl", "data/winner_battles_cache.pkl"):
        with open(source, "rb") as handle:
            cache = pickle.load(handle)
        battles = cache["battles"]
        audit["excluded_caches"][source] = {"battles": len(battles), "file_count": cache.get("file_count"), "sha256": digest(source)}
        known.update(b.battle_id for b in battles)
        if "policy_" in source:
            counts = Counter()
            for b in battles:
                fingerprints.add(signature(b))
                for e in b.events:
                    counts[e["event_type"]] += 1
                    if e["event_type"] == "ability_activation":
                        deck = b.team_deck if e["side"] == "team" else b.opponent_deck
                        counts["ability_target_in_deck" if deck_slot_for_card(deck, e["card"]) is not None else "ability_target_outside_deck"] += 1
            audit["ability_counts"] = dict(counts)
        else:
            fingerprints.update(signature(b) for b in battles)
        del cache, battles
    manifests = {}
    for p in sorted(Path("data/splits").glob("*.json")):
        d = json.loads(p.read_text())
        manifests[p.name] = d
        for key in ("train_ids", "validation_ids", "test_ids"):
            known.update(d.get(key, []))
    for a, da in manifests.items():
        for b, db in manifests.items():
            if a == b:
                continue
            audit["manifest_overlap"][f"{a}:test in {b}:train"] = {
                "overlap": len(set(da["test_ids"]) & set(db["train_ids"])),
                "test_total": len(da["test_ids"]),
            }
    raw = list(Path("data/raw").rglob("*.json"))
    paths = sorted((p for p in raw if p.stem not in known), key=lambda p: hashlib.sha256(p.name.encode()).digest())
    audit.update(raw_files=len(raw), excluded_known_ids=len(known), candidate_files=len(paths),
                 raw_latest_mtime=max(p.stat().st_mtime for p in raw))
    selected, rows, reject, seen = [], [], Counter(), set()
    for i, p in enumerate(paths):
        try:
            r = parse_replay(p)
        except (ReplayParseError, ValueError, TypeError, OSError):
            reject["parse_error"] += 1
            continue
        if r.battle_id in known or r.battle_id in seen:
            reject["known_or_duplicate_id"] += 1
            continue
        td, od = tuple(r.decks.get("team") or []), tuple(r.decks.get("opponent") or [])
        if len(td) != 8 or len(od) != 8:
            reject["not_eight_cards"] += 1
            continue
        events = []
        for e in r.events:
            if e.event_type == "card_play" and e.card and e.x is not None and e.y is not None:
                events.append(dict(seconds=float(e.seconds), side=e.side, event_type=e.event_type, card=e.card, x=int(e.x), y=int(e.y)))
            elif e.event_type == "ability_activation":
                card = e.ability_card or e.card
                if not card:
                    continue
                events.append(dict(seconds=float(e.seconds), side=e.side, event_type=e.event_type, card=card, x=int(e.x) if e.x is not None else 9000, y=int(e.y) if e.y is not None else 16000))
        events.sort(key=lambda e: (e["seconds"], e["side"], e["card"]))
        if sum(e["event_type"] == "card_play" for e in events) < 12:
            reject["too_short"] += 1
            continue
        crowns = r.crowns
        decisive = crowns.get("team") is not None and crowns.get("opponent") is not None and crowns["team"] != crowns["opponent"]
        b = BattleExample(r.battle_id, td, od, int(decisive and crowns["team"] > crowns["opponent"]), tuple(events))
        sig = signature(b)
        if sig in fingerprints:
            reject["duplicate_event_signature"] += 1
            continue
        fingerprints.add(sig)
        seen.add(b.battle_id)
        selected.append(b)
        rows.append(dict(battle_id=b.battle_id, path=str(p), sha256=digest(p), decisive=decisive))
        if len(selected) % 128 == 0:
            progress("fresh cohort", len(selected), limit, started)
        if len(selected) == limit:
            break
    audit.update(scanned=i + 1, rejected=dict(reject), selected=len(selected), selection="SHA256(filename) order; excluded union of both caches and all manifests, then exact normalized event/deck signatures; no mtime selection")
    return selected, rows, audit


def evaluate(model_dir, battles, output):
    started = time.monotonic()
    model, vocab, cfg, device = load_policy(model_dir)
    costs = load_card_costs("data/card_costs.json")
    ds = PolicyActionDataset(battles, vocab, costs, max_context=int(cfg.get("max_context", 64)),
        max_samples_per_battle=24, stride=3, seed=20260906, threat_dim=14,
        reaction_weight=1, prefer_reactions=False, stream_cache_size=64)
    # Card-only targets avoid presenting unsupported ability labels as valid.
    keep = [k for k, (bi, ei) in enumerate(ds.index) if battles[bi].events[ei]["event_type"] == "card_play"]
    ds.index = [ds.index[k] for k in keep]
    ds._mirror_flags = [False] * len(ds.index)
    groups = np.array([bi for bi, _ in ds.index])
    causal, rollout, revealed = [], [], []
    recoverable = 0
    for bi, ei in ds.index:
        b, target = battles[bi], battles[bi].events[ei]
        last = float(b.events[ei - 1]["seconds"])
        def threat_at(t):
            probe = replace(b, events=b.events[:ei] + ({**target, "seconds": t},))
            return recent_opponent_threat(probe, ei, target["side"], costs)[0]
        causal.append(threat_at(last))
        rollout.append(threat_at(last + 1))
        original = recent_opponent_threat(b, ei, target["side"], costs)[0]
        recoverable += bool(original[10] > 0)
        opp = "opponent" if target["side"] == "team" else "team"
        seen = {e["card"] for e in b.events[:ei] if e["side"] == opp}
        deck = b.opponent_deck if target["side"] == "team" else b.team_deck
        revealed.append(torch.tensor([vocab.encode(c) if c in seen else vocab.unk_id for c in deck]))
    causal, rollout, revealed = map(torch.stack, (causal, rollout, revealed))
    records = {name: [] for name in ["archived_contract", "causal_full_deck", "rollout_plus_1s", "causal_revealed", "causal_revealed_neutral_xy", "causal_revealed_oracle_card"]}
    uniform, oldest, changed = [], [], 0
    offset = 0
    with torch.inference_mode():
        for batch in DataLoader(ds, batch_size=256, collate_fn=collate_policy_batch):
            c, ids, td, od, g, sf, hm, slots, types, zones, xy, dt, lengths, weights = [x.to(device) for x in batch]
            n = len(slots)
            gc, gr = g.clone(), g.clone()
            gc[:, -14:] = causal[offset:offset+n].to(device)
            gr[:, -14:] = rollout[offset:offset+n].to(device)
            changed += int((gc[:, -14:] != g[:, -14:]).any(1).sum())
            rd = revealed[offset:offset+n].to(device)
            nc = c.clone(); nc[:, :, 4:6] = 0.5
            target_tile = (xy[:, 1] * 18).long().clamp(0,17) * 32 + (xy[:, 0] * 32).long().clamp(0,31)
            settings = [("archived_contract", c, od, g, None), ("causal_full_deck", c, od, gc, None),
                ("rollout_plus_1s", c, od, gr, None), ("causal_revealed", c, rd, gc, None),
                ("causal_revealed_neutral_xy", nc, rd, gc, None), ("causal_revealed_oracle_card", c, rd, gc, slots)]
            for name, cc, oo, gg, pp in settings:
                out = model(cc, ids, td, oo, gg, lengths, sf, hm, placement_slots=pp,
                    think_steps=int(cfg.get("eval_think_steps", cfg.get("max_think_steps", 0))))
                card_ok = out["slot_logits"].argmax(-1) == slots
                top3 = (out["slot_logits"].topk(3).indices == slots[:,None]).any(1)
                tile = out.get("tile_logits")
                if tile is not None:
                    tok = tile.argmax(-1) == target_tile
                    t5 = (tile.topk(5).indices == target_tile[:,None]).any(1)
                    tnll = F.cross_entropy(tile, target_tile, reduction="none")
                else:
                    tok = (out["xy"][:,1]*18).long().clamp(0,17)*32+(out["xy"][:,0]*32).long().clamp(0,31) == target_tile
                    t5 = torch.zeros_like(tok); tnll = torch.zeros_like(dt)
                values = torch.stack([card_ok.float(), top3.float(), F.cross_entropy(out["slot_logits"],slots,reduction="none"),
                    tok.float(), t5.float(), tnll, (card_ok & tok).float(), (top3 & t5).float(),
                    (out["zone_logits"].argmax(-1)==zones).float(),
                    ((out["xy"]-xy).abs()*xy.new_tensor([18000,32000])).sum(1),
                    (torch.expm1(out["timing"])-torch.expm1(dt)).abs()],1)
                records[name].append(values.cpu().numpy())
            has = hm.any(1)
            p = torch.where(has, hm[torch.arange(n,device=device),slots].float()/hm.sum(1).clamp_min(1), torch.full((n,),1/8,device=device))
            uniform.extend(p.cpu().tolist())
            # Same indexed target population, prior-count frequency baseline.
            for bi, ei in ds.index[offset:offset+n]:
                b = battles[bi]; e = b.events[ei]
                deck = b.team_deck if e["side"] == "team" else b.opponent_deck
                counts = Counter(v["card"] for v in b.events[:ei] if v["side"] == e["side"] and v["event_type"] == "card_play")
                pred = max(deck, key=lambda card: counts[card])
                oldest.append(float(pred == e["card"]))
            offset += n
            if offset % 1024 == 0 or offset == len(ds):
                progress(Path(model_dir).name, offset, len(ds), started)
    columns = ["card_top1", "card_top3", "card_nll", "grid_top1", "grid_top5", "grid_nll", "joint_card_grid_top1", "card_top3_grid_top5_overlap", "zone_top1", "xy_l1_raw_units", "timing_mae_s"]
    arrays = {k: np.concatenate(v) for k,v in records.items()}
    # Paired battle-cluster bootstrap, action-weighted means, shared draws.
    rng = np.random.default_rng(20260906)
    group_n = np.bincount(groups, minlength=len(battles))
    draws = rng.integers(0,len(battles),(1000,len(battles)))
    denom = group_n[draws].sum(1)
    summary = {}
    boot = {}
    for name, a in arrays.items():
        sums = np.stack([np.bincount(groups,weights=a[:,j],minlength=len(battles)) for j in range(a.shape[1])],1)
        boot[name] = sums[draws].sum(1)/denom[:,None]
        ci = np.quantile(boot[name],[.025,.975],axis=0)
        summary[name] = {col: {"mean":float(a[:,j].mean()),"ci95":[float(ci[0,j]),float(ci[1,j])]} for j,col in enumerate(columns)}
        if cfg.get("placement_mode", "xy") != "heatmap":
            for col in ("grid_top5", "grid_nll", "card_top3_grid_top5_overlap"):
                summary[name][col] = {"mean": None, "ci95": None}
    deltas = {}
    for a,b in [("causal_full_deck","archived_contract"),("causal_revealed","causal_full_deck"),
                ("causal_revealed_oracle_card","causal_revealed"),("causal_revealed_neutral_xy","causal_revealed")]:
        ci = np.quantile(boot[a]-boot[b],[.025,.975],axis=0)
        deltas[a+" minus "+b] = {col:{"mean":float((arrays[a][:,j]-arrays[b][:,j]).mean()), "ci95":[float(ci[0,j]),float(ci[1,j])]} for j,col in enumerate(columns)}
    np.savez_compressed(output / (Path(model_dir).name+"_rows.npz"), groups=groups, **arrays)
    return dict(model_dir=model_dir, checkpoint_sha256=digest(Path(model_dir)/"best_model.pt"), vocab_sha256=hashlib.sha256(json.dumps(vocab.to_dict(),sort_keys=True).encode()).hexdigest(),
        samples=len(ds), battles=len(battles), seconds=time.monotonic()-started,
        parameters=sum(p.numel() for p in model.parameters()), config=cfg,
        changed_threat_inputs=changed, recoverable_delay_from_wincon_age=recoverable,
        uniform_hand_baseline=float(np.mean(uniform)), prefix_frequency_baseline=float(np.mean(oldest)),
        metrics=summary, paired_deltas=deltas, columns=columns,
        oracle_card_supported=cfg.get("placement_card_mode", "soft") == "selected",
        sample_index_sha256=hashlib.sha256(json.dumps([(battles[bi].battle_id,ei) for bi,ei in ds.index]).encode()).hexdigest())


def tower_audit():
    rows, games, direct, towers, sources, masks = 0, 0, 0, 0, Counter(), 0
    runs = []
    for p in sorted(Path("data/tower_hp_runs").glob("*/manifest.json")):
        d=json.loads(p.read_text())
        runs.append(dict(path=str(p), complete=d.get("complete"), games_done=d.get("games_done"), samples=d.get("samples"),error=d.get("error")))
    for p in Path("data/tower_hp_runs").glob("*/game_*/tower_hp.jsonl"):
        games += 1
        for line in p.read_text().splitlines():
            d=json.loads(line); rows += 1; masks += bool(d.get("training_mask"))
            for v in d.get("tower_hp",{}).values():
                towers += 1; direct += bool(v.get("ocr_observed")); sources[v.get("label_source","unknown")] += 1
    return dict(runs=runs, games_with_hp_files=games, hp_rows=rows, training_mask_rows=masks,
                tower_values=towers, direct_ocr_values=direct, label_sources=dict(sources),
                frame_files=len(list(Path("data/tower_hp_runs").glob("*/game_*/crops/*.webp"))))


def main():
    ap=argparse.ArgumentParser(); ap.add_argument("--battles",type=int,default=512)
    ap.add_argument("--output",default="reports/astra_v8_evidence")
    args=ap.parse_args(); output=Path(args.output); output.mkdir(parents=True,exist_ok=True)
    torch.set_num_threads(4); torch.manual_seed(20260906)
    started=time.monotonic()
    print(f"{'PHASE':<24} {'WORK DONE':>15} {'ELAPSED':>9} {'ETA':>10}",flush=True)
    battles, manifest, audit=fresh_battles(args.battles)
    if not battles: raise RuntimeError("No unseen battles available")
    (output/"cohort.json").write_text(json.dumps(manifest,indent=2)+"\n")
    result=dict(created_at=datetime.now(timezone.utc).isoformat(), seed=20260906,
        compute=dict(device=torch.cuda.get_device_name(0) if torch.cuda.is_available() else "CPU", torch=torch.__version__, threads=4, training=False),
        audit=audit, towers=tower_audit(), models=[],
        methodology=["Evaluation only: no training or game execution.",
            "Card-only targets, both players, fixed sampled indices after 8 events; max 24 per battle, stride 3.",
            "Each checkpoint uses its own vocabulary. No old evaluation/rollout cache is read.",
            "Frozen-model feature intervention is a distribution-shift diagnostic, not a retrained causal baseline.",
            "Causal conditions query at the last observed event timestamp; actor identity is still supplied by the target.",
            "Oracle-card condition supplies the demonstrated card only to placement; not deployable and not a win-rate ceiling.",
            "Grid is legacy 32 columns x 18 rows; exact grid hits are not physical in-game tile accuracy.",
            "Intervals: 1000 paired bootstrap resamples of battles. No correction for multiple exploratory comparisons.",
            "Unseen excludes all available cache/manifests and exact canonical event signatures; unavailable historical parent manifests prevent a complete lineage proof."])
    for name in ["policy_bc_v4.3", "policy_bc_v4.4.1", "policy_bc_v4.4.2"]:
        result["models"].append(evaluate("models/"+name,battles,output))
        result["seconds"]=time.monotonic()-started
        (output/"results.json").write_text(json.dumps(result,indent=2)+"\n")
    progress("complete",3,3,started)


if __name__ == "__main__":
    main()
