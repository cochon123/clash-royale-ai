"""Auditable coarse-board labels. OCR suggestions are never oracle labels."""

import hashlib
import json
from pathlib import Path
import torch

FIELDS = (
    "team_left_hp",
    "team_king_hp",
    "team_right_hp",
    "opponent_left_hp",
    "opponent_king_hp",
    "opponent_right_hp",
    "team_left_presence",
    "team_center_presence",
    "team_right_presence",
    "opponent_left_presence",
    "opponent_center_presence",
    "opponent_right_presence",
)


def oracle_features(row):
    if not row.get("verified_by") or not row.get("verified_at"):
        raise ValueError("Human verification required; OCR is not ground truth")
    if row.get("battle_id") is None or row.get("event_index") is None:
        raise ValueError("Labels must be aligned to a replay decision")
    if float(row["frame_seconds"]) > float(row["observation_seconds"]):
        raise ValueError("Board frame comes from the future")
    values = []
    mask = []
    for field in FIELDS:
        value = row["labels"].get(field)
        if value is not None and not 0 <= float(value) <= 1:
            raise ValueError(f"{field} must be normalized to [0,1]")
        values.append(float(value or 0))
        mask.append(float(value is not None))
    return torch.tensor(values + mask)


def validate_group_splits(rows):
    seen = {}
    for row in rows:
        group = row["game_id"]
        split = row["split"]
        if split not in {"train", "validation", "test"}:
            raise ValueError("Invalid split")
        if group in seen and seen[group] != split:
            raise ValueError("Adjacent frames from one game cross splits")
        seen[group] = split


def prepare_queue(source, output, per_game=10):
    output = Path(output)
    if output.exists():
        raise ValueError("Refusing to overwrite annotations")
    files = sorted(Path(source).glob("*/game_*/tower_hp.jsonl"))
    games = sorted(
        (str(p.parent) for p in files),
        key=lambda x: hashlib.sha256(x.encode()).hexdigest(),
    )
    if len(games) < 3:
        raise ValueError("Need three recorded games for separate splits")
    train_end = max(1, int(len(games) * 0.7))
    val_end = min(len(games) - 1, train_end + max(1, int(len(games) * 0.15)))
    splits = {
        g: "train" if i < train_end else "validation" if i < val_end else "test"
        for i, g in enumerate(games)
    }
    rows = []
    for path in files:
        samples = [
            json.loads(line) for line in path.read_text().splitlines() if line.strip()
        ]
        if len(samples) > per_game:
            samples = [samples[i * len(samples) // per_game] for i in range(per_game)]
        for sample in samples:
            frame = path.parent / "crops" / f"sample_{sample['sample']:04d}.webp"
            if not frame.exists():
                continue
            rows.append(
                dict(
                    game_id=str(path.parent),
                    split=splits[str(path.parent)],
                    frame=str(frame),
                    frame_sha256=hashlib.sha256(frame.read_bytes()).hexdigest(),
                    frame_seconds=sample["match_time"],
                    observation_seconds=sample["match_time"],
                    battle_id=None,
                    event_index=None,
                    labels={field: None for field in FIELDS},
                    verified_by=None,
                    verified_at=None,
                    ocr_suggestions=sample.get("tower_hp", {}),
                )
            )
    validate_group_splits(rows)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text("".join(json.dumps(r) + "\n" for r in rows))
    return {
        "games": len(games),
        "frames": len(rows),
        "output": str(output),
        "verified_labels": 0,
    }


if __name__ == "__main__":
    import argparse

    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--source", default="data/tower_hp_runs")
    p.add_argument("--output", default="data/board_oracle/annotation_queue.jsonl")
    a = p.parse_args()
    print(json.dumps(prepare_queue(a.source, a.output), indent=2))
