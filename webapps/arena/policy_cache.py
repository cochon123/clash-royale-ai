"""Process-wide weight and engine-digest caches.

Each match still builds its own policy module and RNG from ``seed``. The
checkpoint tensors are read from disk once. The native-watch sha256 used in
match manifests is hashed once per path for the life of this process.
"""
from __future__ import annotations

import hashlib
from pathlib import Path
import threading


_LOCK = threading.RLock()
_TENSORS: dict[str, object] = {}
_SHA: dict[str, str] = {}


def engine_sha256(path: Path) -> str:
    key = str(Path(path).resolve())
    with _LOCK:
        cached = _SHA.get(key)
        if cached is None:
            cached = hashlib.sha256(Path(key).read_bytes()).hexdigest()
            _SHA[key] = cached
        return cached


def load_match_policy(checkpoint, device, seed, specialization):
    """Fresh policy wrapper. Weights come from the process cache."""
    import torch
    from rl.banana_watch import load_watched_policy

    real_load = torch.load

    def cached_load(path, *args, **kwargs):
        key = str(Path(path).resolve())
        with _LOCK:
            hit = _TENSORS.get(key)
        if hit is not None:
            return hit
        payload = real_load(path, *args, **kwargs)
        with _LOCK:
            return _TENSORS.setdefault(key, payload)

    with _LOCK:
        torch.load = cached_load
        try:
            return load_watched_policy(checkpoint, device, seed, specialization)
        finally:
            torch.load = real_load
