"""Cache keys that include source identity, not just file counts."""

from __future__ import annotations

import hashlib
import json
import pickle
from pathlib import Path
from typing import Any


DATASET_PREPROCESS_VERSION = 4


def stable_int_seed(*parts: Any, modulo: int = 2**31) -> int:
    """Process-independent seed derived from stable UTF-8 parts."""
    payload = "\0".join(str(part) for part in parts).encode("utf-8")
    digest = hashlib.sha256(payload).digest()
    return int.from_bytes(digest[:8], "big") % int(modulo)


def sha256_file(path: str | Path, chunk_size: int = 1 << 20) -> str:
    target = Path(path)
    hasher = hashlib.sha256()
    with target.open("rb") as handle:
        while True:
            chunk = handle.read(chunk_size)
            if not chunk:
                break
            hasher.update(chunk)
    return hasher.hexdigest()


def sha256_json(payload: Any) -> str:
    encoded = json.dumps(payload, sort_keys=True, default=str).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def source_fingerprint(source: str | Path) -> dict[str, Any]:
    """Identity of a replay directory: resolved path, count, sizes, mtimes."""
    root = Path(source).resolve()
    hasher = hashlib.sha256()
    hasher.update(str(root).encode("utf-8"))
    file_count = 0
    if root.is_dir():
        for path in sorted(root.rglob("*.json")):
            stat = path.stat()
            rel = path.relative_to(root).as_posix()
            record = f"{rel}:{stat.st_size}:{stat.st_mtime_ns}"
            hasher.update(record.encode("utf-8"))
            file_count += 1
    elif root.is_file():
        stat = root.stat()
        hasher.update(f"{root.name}:{stat.st_size}:{stat.st_mtime_ns}".encode("utf-8"))
        file_count = 1
    return {
        "source": str(root),
        "file_count": file_count,
        "digest": hasher.hexdigest(),
    }


def cache_payload_matches(
    cached: dict[str, Any],
    expected: dict[str, Any],
    *,
    trust: bool = False,
) -> bool:
    if not isinstance(cached, dict):
        return False
    # A debugging environment flag must never waive the dataset contract.
    for key, value in expected.items():
        if cached.get(key) != value:
            return False
    return True


def default_dataset_cache_path(
    source: str | Path,
    *,
    kind: str,
    digest: str,
    min_card_plays: int,
) -> Path:
    root = Path(source).resolve()
    cache_dir = root.parent / ".cr_replay_cache"
    return cache_dir / f"{root.name}_{kind}_p{int(min_card_plays)}_{digest[:16]}.pkl"


def write_pickle(path: str | Path, payload: Any) -> None:
    target = Path(path)
    target.parent.mkdir(parents=True, exist_ok=True)
    with target.open("wb") as handle:
        pickle.dump(payload, handle)


def read_pickle(path: str | Path) -> Any:
    with Path(path).open("rb") as handle:
        return pickle.load(handle)
