"""Reproducible battle-id manifests for training and evaluation."""

from __future__ import annotations

import hashlib
import json
import shutil
from pathlib import Path
from typing import Any

from .policy_dataset import collect_battles
from .winner_dataset import BattleExample, split_battles


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
    train_fraction: float | None = 0.7,
) -> dict[str, Any]:
    """Write a deterministic manifest using the canonical train/val/test split."""
    selected = list(battles[:max_battles] if max_battles is not None else battles)
    if len(selected) < 50:
        raise ValueError(f"Need at least 50 battles for a manifest; got {len(selected)}")
    fraction = 0.7 if train_fraction is None else float(train_fraction)
    train, val, test = split_battles(selected, seed=seed, train_fraction=fraction)
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
        "schema_version": 2,
        "seed": int(seed),
        "min_card_plays": int(min_card_plays),
        "train_fraction": fraction,
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


def _manifest_path_from_json(path: Path) -> Path | None:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None
    for key in ("split_manifest",):
        raw = data.get(key)
        if raw:
            candidate = Path(str(raw))
            if candidate.exists():
                return candidate
    compute = data.get("compute") if isinstance(data.get("compute"), dict) else {}
    raw = compute.get("split_manifest")
    if raw:
        candidate = Path(str(raw))
        if candidate.exists():
            return candidate
    return None


def discover_split_manifest(*candidates: str | Path | None) -> Path | None:
    """Find a frozen split next to a run directory or from an explicit path."""
    for raw in candidates:
        if raw is None:
            continue
        path = Path(raw)
        if path.is_file():
            return path
        if not path.is_dir():
            continue
        for name in ("split_manifest.json", "split.json"):
            direct = path / name
            if direct.exists():
                return direct
        report = path / "report.json"
        if report.exists():
            found = _manifest_path_from_json(report)
            if found is not None:
                return found
    return None


def persist_manifest_copy(manifest_path: str | Path, output_dir: str | Path | None) -> Path:
    source = Path(manifest_path)
    if output_dir is None:
        return source
    destination = Path(output_dir) / "split_manifest.json"
    destination.parent.mkdir(parents=True, exist_ok=True)
    if source.resolve() != destination.resolve():
        shutil.copy2(source, destination)
    return destination


def resolve_battle_splits(
    battles: list[BattleExample],
    *,
    seed: int = 42,
    train_fraction: float = 0.7,
    split_manifest: str | Path | None = None,
    write_split_manifest: str | Path | None = None,
    output_dir: str | Path | None = None,
    warmstart_dir: str | Path | None = None,
    model_dir: str | Path | None = None,
    max_battles: int | None = None,
    min_card_plays: int = 12,
    create: bool = False,
) -> tuple[
    list[BattleExample],
    list[BattleExample],
    list[BattleExample],
    dict[str, Any],
    Path,
]:
    """Load or create a frozen train/val/test split.

    Evaluation (``create=False``) requires an existing manifest so held-out
    IDs cannot silently drift into the training cut. Training writes the
    resolved manifest into ``output_dir``.
    """
    if split_manifest and write_split_manifest:
        raise ValueError("Use either split_manifest or write_split_manifest, not both")

    found = discover_split_manifest(
        split_manifest,
        write_split_manifest if write_split_manifest and Path(write_split_manifest).exists() else None,
        output_dir,
        model_dir,
        warmstart_dir,
    )
    if found is None:
        if not create:
            raise ValueError(
                "Evaluation requires a split manifest. Pass split_manifest=... "
                "or train a model that writes split_manifest.json next to the checkpoint."
            )
        target = Path(
            write_split_manifest
            or (Path(output_dir) / "split_manifest.json" if output_dir is not None else "")
        )
        if not str(target):
            raise ValueError("Training must write a split manifest (pass output_dir)")
        selected = battles[: int(max_battles)] if max_battles is not None else battles
        manifest = build_manifest(
            selected,
            target,
            seed=seed,
            min_card_plays=min_card_plays,
            train_fraction=train_fraction,
        )
        if warmstart_dir:
            assert_ancestor_disjoint(manifest, warmstart_dir)
        stored = persist_manifest_copy(target, output_dir)
        train, val, test = battles_from_manifest(battles, manifest)
        return train, val, test, manifest, stored

    manifest = load_manifest(found)
    if warmstart_dir:
        assert_ancestor_disjoint(manifest, warmstart_dir)
    stored = persist_manifest_copy(found, output_dir) if create or output_dir else found
    train, val, test = battles_from_manifest(battles, manifest)
    return train, val, test, manifest, stored


def assert_ancestor_disjoint(manifest, donor_dir, _seen=None):
    """Reject held-out exposure anywhere in the donor lineage, fail closed."""
    seen = set() if _seen is None else _seen
    donor = Path(donor_dir).resolve()
    if donor in seen:
        return
    seen.add(donor)
    path = discover_split_manifest(donor)
    if path is None:
        raise ValueError(f"Warm-start ancestor has no split manifest: {donor}")
    ancestor = load_manifest(path)
    heldout = set(manifest['validation_ids']) | set(manifest['test_ids'])
    overlap = heldout & set(ancestor['train_ids'])
    if overlap:
        raise ValueError(f"Warm-start ancestor trained on {len(overlap)} held-out battles: {donor}")
    import torch
    checkpoint = donor / 'best_model.pt'
    if checkpoint.exists():
        artifact = torch.load(checkpoint, map_location='cpu', weights_only=False)
        parent = artifact.get('config', {}).get('warmstart', {}).get('dir')
        if parent and Path(parent).resolve() != donor:
            assert_ancestor_disjoint(manifest, parent, seen)


def collect_or_load_manifest(
    input_dir: str | Path,
    manifest_path: str | Path | None,
    *,
    min_card_plays: int = 12,
    seed: int = 42,
    max_battles: int | None = None,
    write_manifest_path: str | Path | None = None,
    train_fraction: float = 0.7,
    output_dir: str | Path | None = None,
    create: bool = True,
) -> tuple[list[BattleExample], list[BattleExample], list[BattleExample], dict[str, Any] | None]:
    """Collect battles and resolve a frozen split (creating one when allowed)."""
    battles = collect_battles(input_dir, min_card_plays=min_card_plays)
    train, val, test, manifest, _path = resolve_battle_splits(
        battles,
        seed=seed,
        train_fraction=train_fraction,
        split_manifest=manifest_path,
        write_split_manifest=write_manifest_path,
        output_dir=output_dir or write_manifest_path,
        max_battles=max_battles,
        min_card_plays=min_card_plays,
        create=create,
    )
    return train, val, test, manifest
