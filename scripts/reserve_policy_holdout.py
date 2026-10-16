"""Reserve unexposed replay IDs; do not score any model on them.

Conservatively excludes all battles in historical caches, manifests, and the
design-probe cohort. File mtime orders capture freshness, not match chronology.
"""

import argparse
import hashlib
import json
from pathlib import Path
import pickle
import time
from cr_replay_pipeline.parser import parse_replay
from cr_replay_pipeline.winner_dataset import battle_example_from_replay


def fingerprint(b):
    def view(swap):
        rows = sorted(
            (
                float(e["seconds"]),
                ("opponent" if e["side"] == "team" else "team") if swap else e["side"],
                e["card"],
                int(e["x"]),
                32000 - int(e["y"]) if swap else int(e["y"]),
            )
            for e in b.events
            if e["event_type"] == "card_play"
        )
        return json.dumps(
            [
                b.opponent_deck if swap else b.team_deck,
                b.team_deck if swap else b.opponent_deck,
                rows,
            ],
            sort_keys=True,
        )

    return hashlib.sha256(min(view(False), view(True)).encode()).hexdigest()


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--input", default="data/raw")
    p.add_argument("--count", type=int, default=1000)
    p.add_argument("--output", default="data/splits/controlled_final_holdout.json")
    args = p.parse_args()
    target = Path(args.output)
    if target.exists():
        raise ValueError("Holdout already exists; refusing to replace it")
    known = set()
    fingerprints = set()
    evidence = []
    started = time.monotonic()
    for cache in ("data/policy_battles_cache.pkl", "data/winner_battles_cache.pkl"):
        path = Path(cache)
        if path.exists():
            with path.open("rb") as f:
                battles = pickle.load(f)["battles"]
            known.update(b.battle_id for b in battles)
            fingerprints.update(fingerprint(b) for b in battles)
            evidence.append(cache)
    cached_ids = set(known)
    for path in list(Path("data/splits").glob("*.json")) + list(
        Path("models").rglob("split_manifest.json")
    ):
        m = json.loads(path.read_text())
        for key in ("train_ids", "validation_ids", "test_ids"):
            known.update(m.get(key, []))
        evidence.append(str(path))
    cohort = Path("reports/astra_v8_evidence/cohort.json")
    if cohort.exists():
        known.update(r["battle_id"] for r in json.loads(cohort.read_text()))
    # Newer manifests/probes may contain battles absent from historical caches.
    # Compare their content as well as IDs, including reversed perspectives.
    uncached_ids = known - cached_ids
    for path in Path(args.input).rglob("*.json"):
        if path.stem in uncached_ids:
            try:
                b, _ = battle_example_from_replay(
                    parse_replay(path),
                    require_decisive_result=False,
                    unresolved_abilities="preserve_event",
                )
                if b is not None:
                    fingerprints.add(fingerprint(b))
            except (ValueError, TypeError, OSError):
                pass
    candidates = sorted(
        (p for p in Path(args.input).rglob("*.json") if p.stem not in known),
        key=lambda p: (-p.stat().st_mtime_ns, p.name),
    )
    selected = []
    hashes = set()
    for i, path in enumerate(candidates):
        try:
            replay = parse_replay(path)
            b, _ = battle_example_from_replay(
                replay,
                require_decisive_result=False,
                unresolved_abilities="preserve_event",
            )
        except (ValueError, TypeError, OSError):
            continue
        if b is None or b.battle_id in known or replay.content_hash in hashes:
            continue
        fp = fingerprint(b)
        if fp in fingerprints:
            continue
        fingerprints.add(fp)
        hashes.add(replay.content_hash)
        known.add(b.battle_id)
        selected.append(
            {
                "battle_id": b.battle_id,
                "path": str(path),
                "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
            }
        )
        if len(selected) % 100 == 0:
            elapsed = time.monotonic() - started
            print(
                f"holdout work={len(selected)}/{args.count} ETA={elapsed / len(selected) * (args.count - len(selected)):.0f}s",
                flush=True,
            )
        if len(selected) == args.count:
            break
    if len(selected) < args.count:
        raise ValueError(
            f"Only {len(selected)} eligible unexposed battles; need {args.count}. No manifest written."
        )
    ids = [r["battle_id"] for r in selected]
    result = {
        "schema_version": 2,
        "train_ids": [],
        "validation_ids": [],
        "test_ids": ids,
        "battle_count": len(ids),
        "ordered_id_sha256": hashlib.sha256("\n".join(ids).encode()).hexdigest(),
        "source_files": selected,
        "excluded_sources": evidence,
        "purpose": "locked final holdout; no screening access",
        "ordering": "latest file capture mtime, not claimed match chronology",
    }
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(result, indent=2) + "\n")
    print(f"holdout work={len(ids)}/{args.count} ETA=0s saved={target}")


if __name__ == "__main__":
    main()
