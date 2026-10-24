"""Ordered, complete per-tick delivery for the browser Painter.

The match loop appends every publication. Subscribers each walk that history
at their own pace. Nothing here keeps only the latest tick: a slow reader
receives the ticks it has not seen yet, and a reader that falls out of the
ring is told to resync instead of skipping ahead silently.
"""
from __future__ import annotations

from dataclasses import dataclass
import os
import threading
import time


RING_TICKS = 400
BATCH_SECONDS = 0.033
IDLE_SECONDS = 10.0

# Guest pointers and native object ids are different spaces. The Painter
# compares ``address`` with ``native_focus_entity``, and ``native_entity_id``
# with ``native_focus_entity_id``. Mixing those numbers in one table would
# alias a small entity id with an equal pointer.
_ADDRESS_KEYS = ("address", "native_focus_entity")
_ID_KEYS = ("native_entity_id", "native_focus_entity_id")


def addresses_remapped() -> bool:
    """Off until the painter address gate says a remap is visually identical."""
    return os.environ.get("ARENA_REMAP_ADDRESSES", "0") == "1"


def parse_last_event_id(value) -> tuple[int, int] | None:
    """``match_number:tick`` from the browser's ``Last-Event-ID`` header."""
    if not isinstance(value, str):
        return None
    match_text, separator, tick_text = value.partition(":")
    if not separator or not match_text.isdecimal() or not tick_text.isdecimal():
        return None
    return int(match_text), int(tick_text)


class AddressRemap:
    """Stable opaque ids for one round, cleared when the round changes."""

    def __init__(self):
        self.round_id = None
        self._addresses: dict[int, int] = {}
        self._entity_ids: dict[int, int] = {}
        self._next = 1

    def begin_round(self, match_number: int):
        if match_number != self.round_id:
            self.round_id = match_number
            self._addresses.clear()
            self._entity_ids.clear()
            self._next = 1

    def _assign(self, table: dict[int, int], value: int) -> int:
        assigned = table.get(value)
        if assigned is None:
            assigned = self._next
            self._next += 1
            table[value] = assigned
        return assigned

    def apply(self, node):
        """Rewrite address fields in ``node`` in place."""
        if isinstance(node, dict):
            for key in _ADDRESS_KEYS:
                value = node.get(key)
                if type(value) is int:
                    node[key] = self._assign(self._addresses, value)
            for key in _ID_KEYS:
                value = node.get(key)
                if type(value) is int:
                    node[key] = self._assign(self._entity_ids, value)
            for value in node.values():
                self.apply(value)
        elif isinstance(node, list):
            for value in node:
                self.apply(value)
        return node


@dataclass
class TickRecord:
    seq: int
    match_number: int
    tick: int
    snapshot: dict
    hud: dict
    seed: int | None


class TickLog:
    """Per-match ring of the last 400 publications and one cursor per subscriber."""

    def __init__(self, *, batch_seconds: float = BATCH_SECONDS, idle_seconds: float = IDLE_SECONDS,
                 clock=None):
        self.batch_seconds = batch_seconds
        self.idle_seconds = idle_seconds
        self.clock = clock or time.monotonic
        self.cond = threading.Condition()
        self.items: list[TickRecord] = []
        self.next_seq = 0
        self.closed = False
        self.error = None
        self.status_gen = 0

    def publish(self, match_number: int, tick: int, snapshot: dict, hud: dict, seed: int | None = None):
        with self.cond:
            self.items.append(TickRecord(self.next_seq, int(match_number), int(tick), snapshot, hud, seed))
            self.next_seq += 1
            overflow = len(self.items) - RING_TICKS
            if overflow > 0:
                del self.items[:overflow]
            self.cond.notify_all()

    def nudge(self):
        """Wake subscribers for a status change that did not publish a tick."""
        with self.cond:
            self.status_gen += 1
            self.cond.notify_all()

    def fail(self, error: str):
        with self.cond:
            self.error = error
            self.cond.notify_all()

    def close(self):
        with self.cond:
            self.closed = True
            self.cond.notify_all()

    def events(self, last_event_id, *, match_id, summary):
        """Yield ``(event, payload, id)`` until the match closes or fails.

        ``id`` is set only on ``ticks`` events. A new subscriber receives the
        retained ring. ``Last-Event-ID`` continues after that tick. Falling
        out of the ring yields ``resync`` and then the ring.
        """
        with self.cond:
            cursor, resync = self._cursor_locked(last_event_id)
            seen_status = self.status_gen
        if resync:
            yield ("resync", {"v": 2, "reason": "behind"}, None)
        last_round = None
        next_emit = 0.0
        idle_deadline = self.clock() + self.idle_seconds
        while True:
            decision = self._next(cursor, last_round, next_emit, seen_status, idle_deadline)
            kind = decision[0]
            if kind == "behind":
                cursor = decision[1]
                last_round = None
                next_emit = 0.0
                yield ("resync", {"v": 2, "reason": "behind"}, None)
                continue
            if kind == "error":
                yield ("error", {"error": decision[1]}, None)
                return
            if kind == "closed":
                yield ("closed", {"id": match_id}, None)
                return
            if kind == "status":
                seen_status = decision[1]
                idle_deadline = self.clock() + self.idle_seconds
                yield ("status", summary(), None)
                continue
            if kind == "round":
                last_round = decision[1]
                next_emit = 0.0
                payload = {"v": 2, "match_number": last_round}
                if decision[2] is not None:
                    payload["seed"] = decision[2]
                yield ("round", payload, None)
                continue
            records = decision[1]
            cursor = records[-1].seq + 1
            last_round = records[0].match_number
            next_emit = self.clock() + self.batch_seconds
            idle_deadline = self.clock() + self.idle_seconds
            yield ("ticks", {
                "v": 2,
                "items": [{"snapshot": record.snapshot, "hud": record.hud} for record in records],
            }, f"{records[-1].match_number}:{records[-1].tick}")

    def _cursor_locked(self, last_event_id):
        parsed = parse_last_event_id(last_event_id)
        if parsed is None:
            if self.items:
                return self.items[0].seq, False
            return self.next_seq, False
        match_number, tick = parsed
        found = [item for item in self.items if item.match_number == match_number and item.tick == tick]
        if found:
            return found[-1].seq + 1, False
        if not self.items:
            return self.next_seq, True
        oldest, newest = self.items[0], self.items[-1]
        key = (match_number, tick)
        if key > (newest.match_number, newest.tick):
            return self.next_seq, False
        return oldest.seq, True

    def _available(self, cursor):
        return [item for item in self.items if item.seq >= cursor]

    def _next(self, cursor, last_round, next_emit, seen_status, idle_deadline):
        with self.cond:
            while True:
                if self.items and self.items[0].seq > cursor:
                    return ("behind", self.items[0].seq)
                available = self._available(cursor)
                now = self.clock()
                if self.error and not available:
                    return ("error", self.error)
                if available and last_round is not None and available[0].match_number != last_round:
                    return ("round", available[0].match_number, available[0].seed)
                if available and now >= next_emit:
                    run = []
                    for item in available:
                        if run and item.match_number != run[0].match_number:
                            break
                        run.append(item)
                    return ("ticks", run)
                if not available and self.closed:
                    return ("closed",)
                if not available and (self.status_gen != seen_status or now >= idle_deadline):
                    return ("status", self.status_gen)
                if available:
                    timeout = max(0.0, next_emit - now)
                else:
                    timeout = max(0.0, idle_deadline - now)
                self.cond.wait(timeout=timeout if timeout > 0 else 0.001)
