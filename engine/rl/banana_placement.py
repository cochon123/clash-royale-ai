"""Authoritative deployment coordinates returned by the native watch harness."""
from __future__ import annotations


def resolved_play_target(result: dict) -> tuple[int, int]:
    """Read an accepted play's effective target without falling back to its aim.

    The engine can accept a troop aimed across the river and deploy it back in
    its own territory. Feeding the requested aim into replay history then tells
    both policies that this troop was played on the wrong side of the arena.
    Older native-watch binaries must be rebuilt instead of silently doing that.
    """
    if result.get("accepted") is not True:
        raise ValueError("only accepted native plays have a deployment target")
    target = result.get("resolved_target")
    if not isinstance(target, dict):
        raise RuntimeError("native-watch did not return resolved_target; rebuild native-watch")
    x, y = target.get("x"), target.get("y")
    if type(x) is not int or type(y) is not int or not (0 <= x < 18000 and 0 <= y < 32000):
        raise ValueError(f"invalid native resolved_target: {target!r}")
    return x, y
