"""Persistent cache of native placement masks, plus a background warmer.

A full mask asks the original target validator about 576 points and takes
about a second in the emulated engine, which would freeze a live match each
time a model plays (the model samples placement from the legal policy grid)
or a human selects a card (the browser shades illegal tiles). Measured on this
build, a card's mask does not change when units or buildings are on the field;
it depends on the side, the card and which princess towers still stand (a
fallen tower opens its pocket). Masks are cached under exactly that key and
persisted, so each one is computed once. The engine still validates every play.
"""
from __future__ import annotations

import hashlib
import json
import os
import queue
import threading
from functools import lru_cache

from webapps.common import catalog, paths

CACHE_PATH = paths.ENGINE_RUNS / "arena" / "placement-cache.json"
KINDS = {"policy": "placement_mask", "tiles": "placement_tiles"}


@lru_cache(maxsize=1)
def engine_identity() -> str:
    """Masks belong to one engine snapshot and native-watch build."""
    digest = hashlib.sha256()
    for path in (paths.ENGINE_CHECKPOINT / "checkpoint.json", paths.NATIVE_WATCH):
        stat = path.stat()
        digest.update(f"{path.name}:{stat.st_size}:{int(stat.st_mtime)}".encode())
    return digest.hexdigest()[:16]


def tower_signature(frame: dict) -> str:
    """Standing princess towers, e.g. ``0:3500,0:14500,1:3500,1:14500``."""
    alive = sorted(f"{e['side']}:{e['x']}" for e in frame.get("entities", ())
                   if e.get("row") == "PrincessTower" and (e.get("hp") or 0) > 0)
    return ",".join(alive)


class PlacementCache:
    def __init__(self, path=CACHE_PATH):
        self.path = path
        self.lock = threading.Lock()
        self.data: dict[str, str] = {}
        try:
            stored = json.loads(path.read_text())
            if stored.get("engine") == engine_identity():
                self.data = stored.get("masks", {})
        except (OSError, ValueError):
            pass
        self.dirty = 0

    @staticmethod
    def key(kind: str, side: int, card: str, towers: str) -> str:
        return f"{kind}|{side}|{card}|{towers}"

    def get(self, kind, side, card, towers):
        value = self.data.get(self.key(kind, side, card, towers))
        return None if value is None else [ch == "1" for ch in value]

    def put(self, kind, side, card, towers, mask):
        with self.lock:
            self.data[self.key(kind, side, card, towers)] = "".join("1" if m else "0" for m in mask)
            self.dirty += 1
        self.save()

    def save(self):
        with self.lock:
            if not self.dirty:
                return
            self.path.parent.mkdir(parents=True, exist_ok=True)
            tmp = self.path.with_suffix(f".{os.getpid()}.tmp")
            tmp.write_text(json.dumps({"engine": engine_identity(), "masks": self.data}))
            os.replace(tmp, self.path)
            self.dirty = 0

    def lookup(self, engine, kind: str, side: int, slot: int, card: str, frame: dict, compute=True):
        """Cached mask for the card in ``slot``; computes it on a miss if allowed."""
        towers = tower_signature(frame)
        mask = self.get(kind, side, card, towers)
        if mask is None and compute:
            mask = engine.request({"op": KINDS[kind], "side": side, "slot": slot})["mask"]
            self.put(kind, side, card, towers, mask)
        return mask


CACHE = PlacementCache()


class Warmer:
    """Precompute masks for whole decks in a separate engine process.

    Plays cheap cards to cycle each hand until every card of both decks has
    both mask kinds for the all-towers-standing state.
    """

    def __init__(self, cache: PlacementCache = CACHE):
        self.cache = cache
        self.jobs: queue.Queue = queue.Queue()
        self.queued: set[tuple] = set()
        self.status = {"state": "idle", "done": 0, "queued": 0, "current": None, "error": None}
        threading.Thread(target=self._run, name="placement-warmer", daemon=True).start()

    def enqueue(self, deck0: list[str], deck1: list[str]):
        key = (tuple(deck0), tuple(deck1))
        if key in self.queued:
            return
        if self._complete(deck0, deck1, self._full_towers()):
            return
        self.queued.add(key)
        self.status["queued"] += 1
        self.jobs.put(key)

    @staticmethod
    def _full_towers():
        return "0:14500,0:3500,1:14500,1:3500"

    def _complete(self, deck0, deck1, towers):
        cards = catalog.cards()
        return all(self.cache.get(kind, side, cards[c]["native"], towers) is not None
                   for side, deck in enumerate((deck0, deck1)) for c in deck for kind in KINDS)

    def _run(self):
        import sys
        if str(paths.ENGINE) not in sys.path:
            sys.path.insert(0, str(paths.ENGINE))
        while True:
            deck0, deck1 = self.jobs.get()
            self.status.update(state="warming", current=[list(deck0), list(deck1)])
            try:
                self._warm(list(deck0), list(deck1))
            except Exception as exc:  # warming is an optimisation; never fatal
                self.status["error"] = f"{type(exc).__name__}: {exc}"
            self.status["done"] += 1
            self.status["queued"] -= 1
            self.queued.discard((deck0, deck1))
            if self.jobs.empty():
                self.status.update(state="idle", current=None)

    def _warm(self, deck0, deck1):
        from rl.banana_train import JsonlEngine
        cards = catalog.cards()
        natives = [[cards[c]["native"] for c in deck] for deck in (deck0, deck1)]
        engine = JsonlEngine(paths.ENGINE_CHECKPOINT, paths.NATIVE_WATCH)
        full = self._full_towers()
        try:
            # A fallen tower changes the key, so restart with another seed
            # (and therefore another opening hand) instead of giving up.
            for seed in range(1, 7):
                engine.request({"op": "reset", "deck0": natives[0], "deck1": natives[1], "seed": seed,
                                "max_ticks": 7300})
                r = engine.request({"op": "step", "actions": [], "ticks": 100})
                for _round in range(16):
                    frame, obs = r["frame"], r["observation"]
                    if tower_signature(frame) != full or r.get("terminated"):
                        break
                    for side in (0, 1):
                        for row in obs["hands"][side]:
                            if row.get("available", True):
                                for kind in KINDS:
                                    self.cache.lookup(engine, kind, side, int(row["slot"]), row["name"], frame)
                    if all(self.cache.get(k, s, n, full) is not None
                           for s in (0, 1) for n in natives[s] for k in KINDS):
                        return
                    # Cycle each hand: play its cheapest card on its own back row.
                    actions = []
                    for side in (0, 1):
                        hand = sorted((h for h in obs["hands"][side] if h.get("available", True)),
                                      key=lambda h: h["cost"])
                        if hand and hand[0]["cost"] <= obs["elixir"][side]:
                            actions.append({"side": side, "slot": int(hand[0]["slot"]), "x": 9000,
                                            "y": 1500 if side == 0 else 30500})
                    r = engine.request({"op": "step", "actions": actions, "ticks": 60})
        finally:
            engine.close()
            self.cache.save()
