"""Exact Clash Royale hand / cycle reconstruction from observed plays.

CR cycle model
--------------
- 8-card queue; hand = first 4.
- Playing a hand card removes it from its hand slot; the head of the wait
  queue enters the hand; the played card goes to the back of the queue.

The initial queue order is an unknown permutation of the 8 deck slots.
We enumerate initials consistent with the observed card_play sequence and
form a posterior over which slots are in hand at each play.
"""

from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache
from itertools import permutations
from typing import Any, Iterable, Sequence

import numpy as np

from .policy_dataset import acting_cycle_features, deck_slot_for_card
from .winner_dataset import BattleExample

HAND_SIZE = 4
DECK_SIZE = 8


@lru_cache(maxsize=1)
def _all_initial_queues() -> np.ndarray:
    """All 8! initial queue orderings as int8 array (40320, 8)."""
    return np.asarray(list(permutations(range(DECK_SIZE))), dtype=np.int8)


def side_play_sequence(
    battle: BattleExample, side: str
) -> tuple[list[int], list[int]] | None:
    """Return (event_indices, deck_slots) for card_play events on ``side``.

    Returns None if a played card is missing from the side's deck.
    """
    if side not in ("team", "opponent"):
        raise ValueError(f"side must be team|opponent, got {side!r}")
    deck = battle.team_deck if side == "team" else battle.opponent_deck
    event_indices: list[int] = []
    slots: list[int] = []
    for index, event in enumerate(battle.events):
        if event["side"] != side or event["event_type"] != "card_play":
            continue
        slot = deck_slot_for_card(deck, event["card"])
        if slot is None:
            return None
        event_indices.append(index)
        slots.append(int(slot))
    return event_indices, slots


def _advance_queues(queues: np.ndarray, play: int) -> np.ndarray:
    """Advance a batch of queues after playing ``play`` (assumed in hand)."""
    pos = np.argmax(queues[:, :HAND_SIZE] == play, axis=1)
    out = np.empty_like(queues)
    for hand_pos in range(HAND_SIZE):
        mask = pos == hand_pos
        if not np.any(mask):
            continue
        cols = [i for i in range(DECK_SIZE) if i != hand_pos]
        out[mask, : DECK_SIZE - 1] = queues[mask][:, cols]
        out[mask, DECK_SIZE - 1] = play
    return out


def consistent_initial_indices(play_slots: Sequence[int]) -> np.ndarray:
    """Indices into ``_all_initial_queues()`` consistent with ``play_slots``."""
    all_q = _all_initial_queues()
    alive = np.arange(all_q.shape[0], dtype=np.int32)
    if not play_slots:
        return alive
    current = all_q.copy()
    for play in play_slots:
        play = int(play)
        batch = current[alive]
        in_hand = (batch[:, :HAND_SIZE] == play).any(axis=1)
        alive = alive[in_hand]
        if alive.size == 0:
            return alive
        current[alive] = _advance_queues(current[alive], play)
    return alive


def hand_posteriors_smoothed(
    play_slots: Sequence[int],
) -> tuple[np.ndarray, int] | None:
    """Smoothed P(slot in hand) before each play, given the full sequence.

    Returns
    -------
    posteriors : (n_plays, 8) float64
        Row t is the posterior over the 8 deck slots being in hand *before*
        ``play_slots[t]``, conditioned on the entire play sequence.
    n_consistent : int
        Number of initial orderings consistent with the full sequence.

    None if the sequence is impossible (untrackable).
    """
    if not play_slots:
        # No plays: uniform — every card is in hand with probability 4/8.
        return np.full((0, DECK_SIZE), 0.5, dtype=np.float64), int(
            _all_initial_queues().shape[0]
        )

    alive = consistent_initial_indices(play_slots)
    n = int(alive.size)
    if n == 0:
        return None

    curs = _all_initial_queues()[alive].copy()
    posts = np.empty((len(play_slots), DECK_SIZE), dtype=np.float64)
    for t, play in enumerate(play_slots):
        counts = np.bincount(curs[:, :HAND_SIZE].ravel(), minlength=DECK_SIZE)
        posts[t] = counts / float(n)
        curs = _advance_queues(curs, int(play))
    return posts, n


def hand_posteriors_causal(
    play_slots: Sequence[int],
) -> tuple[np.ndarray, list[int]] | None:
    """Causal P(slot in hand) before each play (prefix-only, no future).

    Also returns the number of still-alive initials before each play.
    Returns None if any prefix is already inconsistent (should not happen for
    real play sequences unless the deck mapping is wrong).
    """
    all_q = _all_initial_queues()
    alive = np.arange(all_q.shape[0], dtype=np.int32)
    current = all_q.copy()
    posts = np.empty((len(play_slots), DECK_SIZE), dtype=np.float64)
    n_alive: list[int] = []
    for t, play in enumerate(play_slots):
        n = int(alive.size)
        if n == 0:
            return None
        n_alive.append(n)
        batch = current[alive]
        counts = np.bincount(batch[:, :HAND_SIZE].ravel(), minlength=DECK_SIZE)
        posts[t] = counts / float(n)
        play = int(play)
        in_hand = (batch[:, :HAND_SIZE] == play).any(axis=1)
        alive = alive[in_hand]
        if alive.size == 0:
            return None
        current[alive] = _advance_queues(current[alive], play)
    return posts, n_alive


@dataclass
class SideHandTrack:
    """Exact hand tracking for one side of one battle."""

    side: str
    event_indices: list[int]
    play_slots: list[int]
    trackable: bool
    n_consistent: int
    # Posterior before each play; shape (n_plays, 8). Empty if untrackable.
    smoothed: np.ndarray
    causal: np.ndarray
    # Map event_index -> row in smoothed/causal
    _row_by_event: dict[int, int]

    def posterior_at(
        self, event_index: int, *, smoothed: bool = True
    ) -> np.ndarray | None:
        """Posterior over deck slots in hand *before* ``event_index``.

        If ``event_index`` is not a card_play by this side, returns the
        posterior after the latest prior play by this side (still before
        ``event_index``), or the opening uniform-in-hand prior if none.
        """
        if not self.trackable:
            return None
        row = self._row_by_event.get(event_index)
        if row is not None:
            src = self.smoothed if smoothed else self.causal
            return src[row]

        # Not a play by this side: use latest play strictly before event_index.
        prev_row = -1
        for play_i, ei in enumerate(self.event_indices):
            if ei >= event_index:
                break
            prev_row = play_i
        if prev_row < 0:
            # Opening: every card equally likely to sit in a 4-card hand —
            # but before any constraint, each card is in hand with prob 0.5.
            return np.full(DECK_SIZE, 0.5, dtype=np.float64)
        src = self.smoothed if smoothed else self.causal
        # After play prev_row, hand equals state AFTER advancing that play.
        # Reconstruct by advancing once from the pre-play posterior state of
        # survivors — cheaper: re-simulate survivors to prev_row+1.
        return self._posterior_after_play(prev_row, smoothed=smoothed)

    def _posterior_after_play(
        self, play_row: int, *, smoothed: bool
    ) -> np.ndarray:
        """Hand posterior after applying plays ``[:play_row+1]``."""
        if smoothed:
            alive = consistent_initial_indices(self.play_slots)
            curs = _all_initial_queues()[alive].copy()
            for play in self.play_slots[: play_row + 1]:
                curs = _advance_queues(curs, int(play))
            n = max(int(curs.shape[0]), 1)
            counts = np.bincount(curs[:, :HAND_SIZE].ravel(), minlength=DECK_SIZE)
            return counts / float(n)
        # Causal: replay prefix through play_row inclusive.
        all_q = _all_initial_queues()
        alive = np.arange(all_q.shape[0], dtype=np.int32)
        current = all_q.copy()
        for play in self.play_slots[: play_row + 1]:
            play = int(play)
            batch = current[alive]
            in_hand = (batch[:, :HAND_SIZE] == play).any(axis=1)
            alive = alive[in_hand]
            if alive.size == 0:
                return np.zeros(DECK_SIZE, dtype=np.float64)
            current[alive] = _advance_queues(current[alive], play)
        batch = current[alive]
        n = max(int(batch.shape[0]), 1)
        counts = np.bincount(batch[:, :HAND_SIZE].ravel(), minlength=DECK_SIZE)
        return counts / float(n)

    def mask_at(
        self,
        event_index: int,
        *,
        threshold: float = 0.5,
        smoothed: bool = False,
        fallback: np.ndarray | None = None,
    ) -> tuple[np.ndarray, str]:
        """Boolean hand mask at ``event_index``.

        Returns (mask, source) where source is ``exact`` or ``heuristic_fallback``.
        """
        post = self.posterior_at(event_index, smoothed=smoothed)
        if post is None:
            if fallback is None:
                fallback = np.zeros(DECK_SIZE, dtype=bool)
            return fallback.astype(bool), "heuristic_fallback"
        return (post >= threshold), "exact"


def track_side(
    battle: BattleExample,
    side: str,
    *,
    mode: str = "both",
) -> SideHandTrack:
    """Build exact hand tracking for one side.

    mode: ``smoothed`` | ``causal`` | ``both``
    """
    empty = np.zeros((0, DECK_SIZE), dtype=np.float64)
    seq = side_play_sequence(battle, side)
    if seq is None:
        return SideHandTrack(
            side=side,
            event_indices=[],
            play_slots=[],
            trackable=False,
            n_consistent=0,
            smoothed=empty,
            causal=empty,
            _row_by_event={},
        )
    event_indices, play_slots = seq
    row_map = {ei: i for i, ei in enumerate(event_indices)}
    if not play_slots:
        return SideHandTrack(
            side=side,
            event_indices=[],
            play_slots=[],
            trackable=True,
            n_consistent=int(_all_initial_queues().shape[0]),
            smoothed=empty,
            causal=empty,
            _row_by_event={},
        )

    want_s = mode in ("smoothed", "both")
    want_c = mode in ("causal", "both")
    smoothed = empty
    causal = empty
    n_consistent = 0
    trackable = True

    if want_s:
        smoothed_pack = hand_posteriors_smoothed(play_slots)
        if smoothed_pack is None:
            trackable = False
        else:
            smoothed, n_consistent = smoothed_pack
    if want_c:
        causal_pack = hand_posteriors_causal(play_slots)
        if causal_pack is None:
            trackable = False
        else:
            causal, _n_alive = causal_pack
            # n_consistent is defined for smoothed/full-sequence survivors;
            # skip the extra pass in causal-only mode.

    if not trackable:
        return SideHandTrack(
            side=side,
            event_indices=event_indices,
            play_slots=play_slots,
            trackable=False,
            n_consistent=0,
            smoothed=empty,
            causal=empty,
            _row_by_event=row_map,
        )
    return SideHandTrack(
        side=side,
        event_indices=event_indices,
        play_slots=play_slots,
        trackable=True,
        n_consistent=n_consistent,
        smoothed=smoothed,
        causal=causal,
        _row_by_event=row_map,
    )


def track_battle(
    battle: BattleExample, *, mode: str = "both"
) -> dict[str, SideHandTrack]:
    return {
        side: track_side(battle, side, mode=mode) for side in ("team", "opponent")
    }


def heuristic_hand_mask(
    battle: BattleExample,
    event_index: int,
    costs: dict[str, int],
    side: str,
) -> np.ndarray:
    """Current oldest-four wait-time heuristic (bool array length 8)."""
    swap = side == "opponent"
    _feats, hand_mask = acting_cycle_features(battle, event_index, costs, swap)
    return hand_mask.detach().cpu().numpy().astype(bool)


def side_play_count_before(
    battle: BattleExample, event_index: int, side: str
) -> int:
    return sum(
        1
        for event in battle.events[:event_index]
        if event["side"] == side and event["event_type"] == "card_play"
    )


def bucket_play_count(n: int) -> str:
    if n <= 3:
        return "0-3"
    if n <= 7:
        return "4-7"
    return "8+"


def iter_play_decision_points(
    battle: BattleExample,
) -> Iterable[tuple[str, int, int]]:
    """Yield (side, event_index, slot) for every card_play with a known slot."""
    for index, event in enumerate(battle.events):
        if event["event_type"] != "card_play":
            continue
        side = event["side"]
        deck = battle.team_deck if side == "team" else battle.opponent_deck
        slot = deck_slot_for_card(deck, event["card"])
        if slot is None:
            continue
        yield side, index, int(slot)


def summarize_trackability(
    battles: Sequence[BattleExample],
) -> dict[str, Any]:
    """Untrackable rate over (battle, side) pairs with at least one play."""
    total = 0
    untrackable = 0
    n_consistent: list[int] = []
    for battle in battles:
        for side in ("team", "opponent"):
            track = track_side(battle, side)
            if not track.play_slots:
                continue
            total += 1
            if not track.trackable:
                untrackable += 1
            else:
                n_consistent.append(track.n_consistent)
    return {
        "sides_with_plays": total,
        "untrackable_sides": untrackable,
        "untrackable_rate": (untrackable / total) if total else 0.0,
        "mean_n_consistent": (
            float(np.mean(n_consistent)) if n_consistent else 0.0
        ),
        "median_n_consistent": (
            float(np.median(n_consistent)) if n_consistent else 0.0
        ),
    }
