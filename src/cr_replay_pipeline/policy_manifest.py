"""Reproducible battle-id manifests for policy experiments."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
from typing import Any

from .policy_dataset import collect_battles, split_battles
from .winner_dataset import BattleExample


def _id_hash(ids: list[str]) -> str:
    payload = "\n".join(ids).encode("utf-8")
    return hashlib.sha256(payload).hexdigest()


def build_manifest(
    battles: list[BattleExample],
    path: str | Path,
    *,
    seed: int = 42,
    max_battles: int | None = None,
    pilot_train_battles: int | None = None,
    min_card_plays: int = 12,
) -> dict[str, Any]:
    """Write a deterministic manifest using the same split as training."""
    selected = list(battles[:max_battles] if max_battles is not None else battles)
    if len(selected) < 50:
        raise ValueError(f"Need at least 50 battles for a manifest; got {len(selected)}")
    train, val, test = split_battles(selected, seed=seed)
    if pilot_train_battles is not None:
        if pilot_train_battles < 1 or pilot_train_battles > len(train):
            raise ValueError("pilot_train_battles must fit inside the training split")
        train = train[:pilot_train_battles]
    split_ids = {
        "train_ids": [battle.battle_id for battle in train],
        "validation_ids": [battle.battle_id for battle in val],
        "test_ids": [battle.battle_id for battle in test],
    }
    ordered = split_ids["train_ids"] + split_ids["validation_ids"] + split_ids["test_ids"]
    if len(set(ordered)) != len(ordered):
        raise ValueError("Manifest contains duplicate battle IDs")
    manifest: dict[str, Any] = {
        "schema_version": 1,
        "seed": int(seed),
        "min_card_plays": int(min_card_plays),
        "battle_count": len(ordered),
        "train_battle_count": len(split_ids["train_ids"]),
        "validation_battle_count": len(split_ids["validation_ids"]),
        "test_battle_count": len(split_ids["test_ids"]),
        "pilot_train_battles": pilot_train_battles,
        **split_ids,
        "ordered_id_sha256": _id_hash(ordered),
    }
    target = Path(path)
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    return manifest


def load_manifest(path: str | Path) -> dict[str, Any]:
    target = Path(path)
    data = json.loads(target.read_text(encoding="utf-8"))
    required = ("train_ids", "validation_ids", "test_ids", "ordered_id_sha256")
    missing = [key for key in required if key not in data]
    if missing:
        raise ValueError(f"Manifest missing required keys: {', '.join(missing)}")
    ordered = list(data["train_ids"]) + list(data["validation_ids"]) + list(data["test_ids"])
    if len(set(ordered)) != len(ordered):
        raise ValueError("Manifest contains duplicate battle IDs")
    if data.get("battle_count") is not None and int(data["battle_count"]) != len(ordered):
        raise ValueError("Manifest battle_count does not match its split IDs")
    if _id_hash(ordered) != data["ordered_id_sha256"]:
        raise ValueError("Manifest ordered_id_sha256 does not match its IDs")
    return data


def battles_from_manifest(
    battles: list[BattleExample], manifest: dict[str, Any]
) -> tuple[list[BattleExample], list[BattleExample], list[BattleExample]]:
    """Resolve manifest IDs and reject missing or duplicate source battles."""
    by_id: dict[str, BattleExample] = {}
    for battle in battles:
        if battle.battle_id in by_id:
            raise ValueError(f"Duplicate source battle ID: {battle.battle_id}")
        by_id[battle.battle_id] = battle

    def resolve(key: str) -> list[BattleExample]:
        ids = list(manifest[key])
        missing = [battle_id for battle_id in ids if battle_id not in by_id]
        if missing:
            raise FileNotFoundError(
                f"Manifest references {len(missing)} unavailable battles; first={missing[0]}"
            )
        return [by_id[battle_id] for battle_id in ids]

    return resolve("train_ids"), resolve("validation_ids"), resolve("test_ids")


def collect_or_load_manifest(
    input_dir: str | Path,
    manifest_path: str | Path | None,
    *,
    min_card_plays: int = 12,
    seed: int = 42,
    max_battles: int | None = None,
    write_manifest_path: str | Path | None = None,
) -> tuple[list[BattleExample], list[BattleExample], list[BattleExample], dict[str, Any] | None]:
    """Collect battles and either resolve or create a manifest."""
    battles = collect_battles(input_dir, min_card_plays=min_card_plays)
    if manifest_path is None:
        if max_battles is not None:
            battles = battles[: int(max_battles)]
        train, val, test = split_battles(battles, seed=seed)
        return train, val, test, None
    manifest = load_manifest(manifest_path)
    train, val, test = battles_from_manifest(battles, manifest)
    return train, val, test, manifest
