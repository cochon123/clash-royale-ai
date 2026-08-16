"""Live dual-phone battle loop for phone-lab (policy vs policy / manual)."""

from __future__ import annotations

import random
import threading
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Callable

import numpy as np
import torch

from ..policy_infer import load_policy, predict_next_action, rollout_decode_settings
from ..winner_dataset import BattleExample, CardVocab, load_card_costs

PHONE_KEYS = ("pixel8", "pixel9")
CONTROLLER_MANUAL = "manual"
CONTROLLER_V3 = "policy_bc_v3"
CONTROLLER_V4 = "policy_bc_v4"
CONTROLLER_V41 = "policy_bc_v4.1"
CONTROLLER_V42 = "policy_bc_v4.2"
CONTROLLER_V43 = "policy_bc_v4.3"
CONTROLLER_V44 = "policy_bc_v4.4"
CONTROLLER_V441 = "policy_bc_v4.4.1"
DEFAULT_CONTROLLERS = {
    "pixel8": CONTROLLER_V441,
    "pixel9": CONTROLLER_V441,
}
DEFAULT_POLICY_DIRS = {
    CONTROLLER_V3: Path("models/policy_bc_v3"),
    CONTROLLER_V4: Path("models/policy_bc_v4"),
    CONTROLLER_V41: Path("models/policy_bc_v4.1"),
    CONTROLLER_V42: Path("models/policy_bc_v4.2_full"),
    CONTROLLER_V43: Path("models/policy_bc_v4.3"),
    CONTROLLER_V44: Path("models/policy_bc_v4.4"),
    CONTROLLER_V441: Path("models/policy_bc_v4.4.1"),
}
# Lab default decks (Pixel 8 noob beatdown / Pixel 9 mortar bait-ish).
DEFAULT_DECKS = {
    "pixel8": [
        "knight",
        "archers",
        "minions",
        "skeletons",
        "arrows",
        "fireball",
        "giant",
        "mini-pekka",
    ],
    "pixel9": [
        "mortar",
        "knight",
        "archers",
        "skeletons",
        "barbarian-barrel",
        "arrows",
        "mini-pekka",
        "rocket",
    ],
}

ARENA_X_MAX = 18000
ARENA_Y_MAX = 32000
# Consecutive loop iterations where every AI phone has an empty YOLO hand
# before we assume the match UI is gone and stop.
MAX_EMPTY_HAND_STREAK = 6
# The model was trained and evaluated with delays clipped to this range.
MAX_PLAN_DELAY_S = 12.0
MIN_DELAY_S = 0.05
POST_PLAY_SLEEP_S = 0.35
HAND_MAX_AGE_S = 0.55  # reuse YOLO result unless older than this
ELIXIR_MAX_AGE_S = 1.25
ELIXIR_MIN_CONFIDENCE = 0.72
CONFIRM_TIMEOUT_S = 1.8
CONFIRM_POLL_S = 0.12
CONFIRM_STABLE_FRAMES = 2
HAND_MIN_CONFIDENCE = 0.50

PRESET_DECKS: dict[str, list[str]] = {
    "hog_cycle_2.6": [
        "hog-rider",
        "cannon",
        "fireball",
        "ice-golem",
        "ice-spirit",
        "musketeer",
        "skeletons",
        "the-log",
    ],
    "logbait": [
        "goblin-barrel",
        "princess",
        "goblin-gang",
        "ice-spirit",
        "inferno-tower",
        "knight",
        "rocket",
        "the-log",
    ],
    "xbow_3.0": [
        "archers",
        "fireball",
        "ice-spirit",
        "knight",
        "skeletons",
        "tesla",
        "the-log",
        "x-bow",
    ],
    "pekka_bridgespam": [
        "pekka",
        "battle-ram",
        "bandit",
        "electro-wizard",
        "minions",
        "poison",
        "royal-ghost",
        "zap",
    ],
    "golem_beatdown": [
        "golem",
        "baby-dragon",
        "mega-minion",
        "night-witch",
        "lumberjack",
        "tornado",
        "lightning",
        "barbarian-barrel",
    ],
    "miner_poison": [
        "miner",
        "poison",
        "electro-wizard",
        "inferno-tower",
        "ice-golem",
        "skeletons",
        "the-log",
        "bats",
    ],
}


def normalize_card(name: str | None) -> str:
    if not name:
        return ""
    return str(name).strip().lower().replace(" ", "-").replace("_", "-")


def base_card(name: str | None) -> str:
    """Map YOLO / UI names onto policy deck ids.

    Evolutions are detected as ``knight-evo`` etc., but decks and the BC
    policy only know the base card ``knight``.
    """
    card = normalize_card(name)
    if card.endswith("-evo"):
        return card[: -len("-evo")]
    if card.endswith("evo") and "-" in card:
        # e.g. rare alternate spellings
        return card.rsplit("-", 1)[0]
    return card


def logic_xy_to_uv(x: int | float, y: int | float) -> tuple[float, float]:
    """Map replay logic coords → phone-lab arena (u, v).

    Policy / replays: low x ≈ left, low y ≈ own side.
    phone-lab: u=0 left … 1 right; v=0 enemy … 1 own.
    """
    u = float(np.clip(float(x) / float(ARENA_X_MAX), 0.0, 1.0))
    v = float(np.clip(1.0 - float(y) / float(ARENA_Y_MAX), 0.0, 1.0))
    return u, v


def flip_logic_x(x: int | float) -> int:
    """Mirror X across the river for a facing opponent's screen perspective."""
    return int(ARENA_X_MAX) - int(x)


def flip_logic_y(y: int | float) -> int:
    """Map a team-perspective Y into the opposite side's team perspective."""
    return int(ARENA_Y_MAX) - int(y)


def opponent_perspective_xy(x: int | float, y: int | float) -> tuple[int, int]:
    """Convert the other phone's team-perspective (x, y) into ours.

    Phones face each other, each with own side at the bottom of the screen.
    Their screen-left is our screen-right, and their own half is our enemy half.
    RoyaleAPI team-perspective uses that same mirrored frame for the opponent.
    """
    return flip_logic_x(x), flip_logic_y(y)


def find_hand_slot(card_name: str, hand_slots: list[dict[str, Any]]) -> int | None:
    want = base_card(card_name)
    if not want:
        return None
    for slot in hand_slots:
        if base_card(slot.get("card_name")) == want:
            return int(slot["slot"])
    return None


def detected_hand_cards(hand_slots: list[dict[str, Any]]) -> list[str]:
    """Base card ids currently visible (evo suffix stripped)."""
    out: list[str] = []
    for slot in hand_slots:
        name = base_card(slot.get("card_name"))
        confidence = float(slot.get("confidence", 1.0))
        if name and confidence >= HAND_MIN_CONFIDENCE:
            out.append(name)
    return out


def hand_is_empty(hand_slots: list[dict[str, Any]]) -> bool:
    return not detected_hand_cards(hand_slots)


def resolve_playable_card(
    pred: dict[str, Any],
    hand_slots: list[dict[str, Any]],
    costs: dict[str, int],
    available_elixir: float | None = None,
) -> tuple[str, int, int] | None:
    """Resolve only the model-selected card; never silently substitute one."""
    card = base_card(pred.get("card"))
    slot = find_hand_slot(card, hand_slots)
    cost = int(costs.get(card, 4)) if card else 0
    if slot is None or not card:
        return None
    if available_elixir is not None and cost > available_elixir + 1e-6:
        return None
    return card, slot, cost


def hand_confirms_play(
    before: list[dict[str, Any]],
    after: list[dict[str, Any]],
    slot: int,
    card: str,
) -> bool | None:
    """Confirm a deployment from the selected slot changing to another card.

    ``None`` means the slot could not be read, so callers may retry detection.
    The same card cannot naturally cycle back into the hand after one play.
    """
    before_card = next(
        (base_card(row.get("card_name")) for row in before if row.get("slot") == slot),
        None,
    )
    after_card = next(
        (base_card(row.get("card_name")) for row in after if row.get("slot") == slot),
        None,
    )
    if before_card is None or after_card is None:
        return None
    return before_card == base_card(card) and after_card != base_card(card)


def validate_deck(deck: list[str], known: set[str]) -> list[str]:
    cards = [base_card(c) for c in deck]
    if len(cards) != 8:
        raise ValueError(f"deck must have 8 cards, got {len(cards)}")
    if len(set(cards)) != 8:
        raise ValueError("deck cards must be unique")
    unknown = [c for c in cards if c not in known]
    if unknown:
        raise ValueError(f"unknown cards: {unknown}")
    return cards


def list_controllers() -> list[str]:
    return [
        CONTROLLER_V441,
        CONTROLLER_V44,
        CONTROLLER_V43,
        CONTROLLER_V42,
        CONTROLLER_V41,
        CONTROLLER_V4,
        CONTROLLER_V3,
        CONTROLLER_MANUAL,
    ]


def cards_payload(card_costs_path: str | Path) -> dict[str, Any]:
    costs = load_card_costs(card_costs_path)
    names = sorted(costs.keys())
    return {
        "cards": [{"name": n, "cost": int(costs[n])} for n in names],
        "presets": {k: list(v) for k, v in PRESET_DECKS.items()},
        "controllers": list_controllers(),
        "default_controllers": dict(DEFAULT_CONTROLLERS),
        "default_decks": {k: list(v) for k, v in DEFAULT_DECKS.items()},
    }


class ElixirTracker:
    """Match-clock elixir for both phones (blue-perspective rates from dual-phone v8)."""

    MAX_ELIXIR = 10.0
    STARTING = 5.0
    NORMAL_RATE = 1.0 / 2.8
    DOUBLE_RATE = 2.0 / 2.8
    # Triple elixir is one elixir every 0.9 seconds, not 0.9 elixir/second.
    OVERTIME_RATE = 1.0 / 0.9
    DOUBLE_AT = 120.0
    OVERTIME_AT = 240.0

    def __init__(self) -> None:
        self.values = {k: self.STARTING for k in PHONE_KEYS}
        self.last_t = 0.0

    def reset(self) -> None:
        self.values = {k: self.STARTING for k in PHONE_KEYS}
        self.last_t = 0.0

    def rate(self, match_time: float) -> float:
        if match_time > self.OVERTIME_AT:
            return self.OVERTIME_RATE
        if match_time > self.DOUBLE_AT:
            return self.DOUBLE_RATE
        return self.NORMAL_RATE

    def update(self, match_time: float) -> None:
        delta = match_time - self.last_t
        if delta <= 0:
            return
        # Step in small chunks so phase transitions (120/240) are roughly right.
        t = self.last_t
        while t < match_time - 1e-9:
            step = min(0.25, match_time - t)
            mid = t + step
            gain = step * self.rate(mid)
            for key in PHONE_KEYS:
                self.values[key] = min(self.MAX_ELIXIR, self.values[key] + gain)
            t = mid
        self.last_t = match_time

    def can_afford(self, phone: str, cost: int | float) -> bool:
        return self.values[phone] + 1e-6 >= float(cost)

    def spend(self, phone: str, cost: int | float) -> bool:
        if not self.can_afford(phone, cost):
            return False
        self.values[phone] -= float(cost)
        return True

    def reconcile(self, phone: str, observed: int | float) -> None:
        """Anchor simulated state to the game's visually observed bar."""
        self.values[phone] = max(0.0, min(self.MAX_ELIXIR, float(observed)))


@dataclass
class PolicyBundle:
    key: str
    model: Any
    vocab: CardVocab
    cfg: dict[str, Any]
    device: torch.device


@dataclass
class BattleConfig:
    decks: dict[str, list[str]]
    controllers: dict[str, str]
    timeout_s: float = 300.0
    card_costs_path: Path = Path("data/card_costs.json")
    policy_dirs: dict[str, Path] = field(
        default_factory=lambda: dict(DEFAULT_POLICY_DIRS)
    )
    mirror_tta: bool = False
    think_steps: int | None = None
    stop_on_empty_hands: bool = True


class BattleRunner:
    """Background match loop shared by both phones."""

    def __init__(
        self,
        *,
        detect_hand: Callable[[str], list[dict[str, Any]]],
        execute_action: Callable[[str, int, float, float], dict[str, Any]],
        observe_elixir: Callable[[str], dict[str, Any] | None] | None = None,
        log: Callable[[str, str], None] | None = None,
    ):
        self._detect_hand = detect_hand
        self._execute_action = execute_action
        self._observe_elixir = observe_elixir
        self._log = log or (lambda msg, level="info": None)
        self._lock = threading.Lock()
        self._thread: threading.Thread | None = None
        self._stop = threading.Event()
        self._running = False
        self._elixir = ElixirTracker()
        self._events: list[dict[str, Any]] = []
        self._attempts: list[dict[str, Any]] = []
        self._log_lines: list[dict[str, Any]] = []
        self._hands: dict[str, list[dict[str, Any]]] = {k: [] for k in PHONE_KEYS}
        self._hand_at: dict[str, float] = {k: 0.0 for k in PHONE_KEYS}
        self._next: dict[str, dict[str, Any] | None] = {k: None for k in PHONE_KEYS}
        self._config: BattleConfig | None = None
        self._policies: dict[str, PolicyBundle] = {}
        self._policy_cache: dict[tuple[str, str], PolicyBundle] = {}
        self._costs: dict[str, int] = {}
        self._started_at = 0.0
        self._match_time = 0.0
        self._error: str | None = None
        self._last_play_phone: str | None = None
        self._rng = random.Random(441)

    @property
    def running(self) -> bool:
        return self._running

    def events_snapshot(self) -> list[dict[str, Any]]:
        """Return a stable copy for data collectors and reports."""
        with self._lock:
            return [dict(event) for event in self._events]

    def attempts_snapshot(self) -> list[dict[str, Any]]:
        with self._lock:
            return [dict(attempt) for attempt in self._attempts]

    def status(self) -> dict[str, Any]:
        with self._lock:
            match_t = (
                (time.time() - self._started_at)
                if self._running and self._started_at
                else self._match_time
            )
            return {
                "running": self._running,
                "error": self._error,
                "match_time": round(match_t, 2),
                "decks": {
                    k: list(self._config.decks[k]) if self._config else []
                    for k in PHONE_KEYS
                },
                "controllers": {
                    k: (
                        self._config.controllers[k]
                        if self._config
                        else DEFAULT_CONTROLLERS[k]
                    )
                    for k in PHONE_KEYS
                },
                "elixir": {k: round(self._elixir.values[k], 2) for k in PHONE_KEYS},
                "hands": {
                    k: [
                        {
                            "slot": s.get("slot"),
                            "card_name": s.get("card_name"),
                            "confidence": s.get("confidence"),
                        }
                        for s in self._hands.get(k, [])
                    ]
                    for k in PHONE_KEYS
                },
                "next": dict(self._next),
                "protocol": {
                    "scheduler": "two-sided predicted-delay race",
                    "slot": "checkpoint rollout decoder + detected-hand hard mask",
                    "placement": "checkpoint rollout decoder conditioned on selected card",
                    "think_steps": (self._config.think_steps if self._config else None),
                    "history": "confirmed deployments only",
                },
                "log": list(self._log_lines[-80:]),
                "events": len(self._events),
                "attempts": len(self._attempts),
            }

    def stop(self) -> None:
        self._stop.set()
        thread = self._thread
        if thread and thread.is_alive() and thread is not threading.current_thread():
            thread.join(timeout=8.0)
        with self._lock:
            self._running = False
            self._thread = None

    def start(self, config: BattleConfig) -> dict[str, Any]:
        if self._running:
            raise RuntimeError("battle already running")
        costs = load_card_costs(config.card_costs_path)
        known = set(costs.keys())
        decks = {k: validate_deck(config.decks[k], known) for k in PHONE_KEYS}
        controllers = {}
        for k in PHONE_KEYS:
            ctrl = str(config.controllers.get(k) or DEFAULT_CONTROLLERS[k])
            if ctrl not in list_controllers():
                raise ValueError(f"unknown controller for {k}: {ctrl}")
            controllers[k] = ctrl
        ai_count = sum(ctrl != CONTROLLER_MANUAL for ctrl in controllers.values())
        if ai_count == 1:
            raise ValueError(
                "mixed manual/AI battles are not model-faithful: manual deployments "
                "cannot be reconstructed from hand crops with placement coordinates"
            )
        config = BattleConfig(
            decks=decks,
            controllers=controllers,
            timeout_s=config.timeout_s,
            card_costs_path=config.card_costs_path,
            policy_dirs=config.policy_dirs,
            mirror_tta=config.mirror_tta,
            think_steps=config.think_steps,
            stop_on_empty_hands=config.stop_on_empty_hands,
        )

        needed = {c for c in controllers.values() if c != CONTROLLER_MANUAL}
        policies: dict[str, PolicyBundle] = {}
        for key in needed:
            path = Path(config.policy_dirs.get(key) or DEFAULT_POLICY_DIRS[key])
            if not (path / "best_model.pt").exists():
                raise FileNotFoundError(
                    f"missing policy checkpoint: {path / 'best_model.pt'}"
                )
            cache_key = (key, str(path.resolve()))
            bundle = self._policy_cache.get(cache_key)
            if bundle is None:
                model, vocab, cfg, device = load_policy(path)
                bundle = PolicyBundle(
                    key=key, model=model, vocab=vocab, cfg=cfg, device=device
                )
                self._policy_cache[cache_key] = bundle
            policies[key] = bundle

        self.stop()
        self._stop.clear()
        self._error = None
        self._config = config
        self._policies = policies
        self._costs = costs
        self._events = []
        self._attempts = []
        self._log_lines = []
        self._next = {k: None for k in PHONE_KEYS}
        self._hand_at = {k: 0.0 for k in PHONE_KEYS}
        self._last_play_phone = None
        self._rng = random.Random(441)
        self._elixir.reset()
        self._match_time = 0.0
        self._started_at = time.time()
        self._running = True
        self._thread = threading.Thread(
            target=self._run_loop, name="phone-battle", daemon=True
        )
        self._thread.start()
        self._append_log("battle started", "ok")
        return self.status()

    def _append_log(self, message: str, level: str = "info") -> None:
        line = {
            "t": round(time.time() - self._started_at, 1) if self._started_at else 0.0,
            "level": level,
            "message": message,
        }
        with self._lock:
            self._log_lines.append(line)
            if len(self._log_lines) > 200:
                self._log_lines = self._log_lines[-200:]
        self._log(message, level)

    def _ai_phones(self) -> list[str]:
        assert self._config is not None
        return [
            k for k in PHONE_KEYS if self._config.controllers[k] != CONTROLLER_MANUAL
        ]

    def _refresh_hand(self, phone: str, *, force: bool = False) -> list[dict[str, Any]]:
        age = time.time() - self._hand_at.get(phone, 0.0)
        if (
            not force
            and age < HAND_MAX_AGE_S
            and self._hands.get(phone)
            and not hand_is_empty(self._hands[phone])
        ):
            return self._hands[phone]
        slots = self._detect_hand(phone)
        with self._lock:
            self._hands[phone] = slots
            self._hand_at[phone] = time.time()
        return slots

    def _plan_delay(self, phone: str, raw_delay: float) -> float:
        """Bound a policy's delay without favoring either player.

        Both phones receive independent predictions and the smallest ready time
        wins. Giving the opponent a special reaction cap forced near-perfect
        blue/red alternation, even when the policy preferred the same side.
        """
        delay = float(raw_delay or MIN_DELAY_S)
        return min(max(delay, MIN_DELAY_S), MAX_PLAN_DELAY_S)

    def _phone_battle_view(self, phone: str) -> BattleExample:
        """Build a team-perspective BattleExample for ``phone``.

        Each phone stores plays in its own team perspective (low x = left on
        *their* screen, low y = own side). Facing phones mirror left/right, so
        opponent events must flip both X and Y into this phone's frame —
        matching RoyaleAPI team/opponent polarity the policy was trained on.
        """
        assert self._config is not None
        other = "pixel9" if phone == "pixel8" else "pixel8"
        events: list[dict[str, Any]] = []
        for ev in self._events:
            side_phone = ev["side"]
            if side_phone == phone:
                mapped = "team"
                x, y = int(ev["x"]), int(ev["y"])
            else:
                mapped = "opponent"
                x, y = opponent_perspective_xy(ev["x"], ev["y"])
            events.append(
                {
                    "seconds": ev["seconds"],
                    "side": mapped,
                    "event_type": ev["event_type"],
                    "card": ev["card"],
                    "x": x,
                    "y": y,
                }
            )
        return BattleExample(
            battle_id="live-battle",
            team_deck=tuple(self._config.decks[phone]),
            opponent_deck=tuple(self._config.decks[other]),
            team_wins=0,
            events=tuple(events),
        )

    def _predict(self, phone: str) -> dict[str, Any] | None:
        assert self._config is not None
        ctrl = self._config.controllers[phone]
        if ctrl == CONTROLLER_MANUAL:
            return None
        hand = self._hands.get(phone) or []
        declared = set(self._config.decks[phone])
        hand_cards = set(detected_hand_cards(hand)) & declared
        if not hand_cards:
            return None
        bundle = self._policies[ctrl]
        battle = self._phone_battle_view(phone)
        decode = rollout_decode_settings(bundle.cfg)
        think_steps = (
            decode["think_steps"]
            if self._config.think_steps is None
            else int(self._config.think_steps)
        )
        try:
            pred = predict_next_action(
                bundle.model,
                bundle.vocab,
                self._costs,
                battle,
                bundle.device,
                acting_side="team",
                temperature=float(decode["temperature"]),
                slot_decode=str(decode["slot_decode"]),
                max_context=int(bundle.cfg.get("max_context", 64)),
                threat_dim=int(bundle.cfg.get("threat_dim", 0)),
                min_context=0,
                prefer_cards=hand_cards,
                placement_decode=str(decode["placement_decode"]),
                placement_temperature=float(decode["placement_temperature"]),
                placement_top_k=decode["placement_top_k"],
                mirror_tta=bool(self._config.mirror_tta),
                think_steps=think_steps,
                now_seconds=self._match_time,
                rng=self._rng,
            )
        except Exception as exc:  # noqa: BLE001
            self._append_log(f"{phone} predict failed: {exc}", "err")
            return None
        pred["phone"] = phone
        pred["controller"] = ctrl
        return pred

    def _collect_candidates(self, match_t: float) -> list[dict[str, Any]]:
        assert self._config is not None
        candidates: list[dict[str, Any]] = []
        for phone in PHONE_KEYS:
            if self._config.controllers[phone] == CONTROLLER_MANUAL:
                with self._lock:
                    self._next[phone] = {"controller": CONTROLLER_MANUAL}
                continue
            # Prefer cached hand; force only when empty/stale.
            hand = self._refresh_hand(
                phone, force=hand_is_empty(self._hands.get(phone) or [])
            )
            if hand_is_empty(hand):
                with self._lock:
                    self._next[phone] = {
                        "controller": self._config.controllers[phone],
                        "skip": "empty_hand",
                    }
                continue
            pred = self._predict(phone)
            with self._lock:
                self._next[phone] = pred
            if not pred:
                continue
            if pred.get("event_type") == "ability_activation":
                pred["skip"] = "ability_not_calibrated"
                self._append_log(
                    f"{phone}: model proposed an ability; ability control is not calibrated",
                    "err",
                )
                continue
            delay = self._plan_delay(
                phone, float(pred.get("delay_seconds") or MIN_DELAY_S)
            )
            last_event_t = float(self._events[-1]["seconds"]) if self._events else 0.0
            candidates.append(
                {
                    "phone": phone,
                    "pred": pred,
                    "ready_at": max(match_t, last_event_t + delay),
                }
            )
        candidates.sort(key=lambda c: c["ready_at"])
        return candidates

    def _run_loop(self) -> None:
        assert self._config is not None
        empty_streak = 0
        try:
            for key in PHONE_KEYS:
                slots = self._refresh_hand(key, force=True)
                names = [s.get("card_name") or "?" for s in slots]
                self._append_log(f"{key} hand: {names}", "ok")
                deck_set = set(self._config.decks[key])
                mystery = [n for n in detected_hand_cards(slots) if n not in deck_set]
                if mystery:
                    self._append_log(
                        f"{key}: detected {mystery} not in declared deck — "
                        "fix the deck pickers or plays will look random",
                        "err",
                    )

            while not self._stop.is_set():
                match_t = time.time() - self._started_at
                self._match_time = match_t
                if match_t >= self._config.timeout_s:
                    self._append_log("timeout reached", "info")
                    break

                self._elixir.update(match_t)

                ai_phones = self._ai_phones()
                # Cheap emptiness check from cache; only force-detect empties.
                for phone in ai_phones:
                    if hand_is_empty(self._hands.get(phone) or []):
                        self._refresh_hand(phone, force=True)

                nonempty = [p for p in ai_phones if not hand_is_empty(self._hands[p])]
                if ai_phones and not nonempty:
                    empty_streak += 1
                    self._append_log(
                        f"all AI hands empty ({empty_streak}/{MAX_EMPTY_HAND_STREAK})",
                        "err",
                    )
                    if empty_streak >= MAX_EMPTY_HAND_STREAK:
                        if self._config.stop_on_empty_hands:
                            self._append_log(
                                "stopping: YOLO lost both hands "
                                "(match over / overlay / wrong screen?)",
                                "err",
                            )
                            break
                        self._append_log(
                            "both hands temporarily hidden; collection mode keeps waiting",
                            "info",
                        )
                        empty_streak = 0
                    time.sleep(0.35)
                    continue
                empty_streak = 0

                candidates = self._collect_candidates(match_t)
                if not candidates:
                    time.sleep(0.15)
                    continue

                played_phone: str | None = None
                for chosen in candidates:
                    if self._stop.is_set():
                        break
                    # A failed earlier candidate must not let the other phone
                    # act before its own model-predicted ready time.
                    wait = chosen["ready_at"] - (time.time() - self._started_at)
                    if wait > 0:
                        deadline = time.time() + wait
                        while time.time() < deadline and not self._stop.is_set():
                            time.sleep(min(0.05, max(0.0, deadline - time.time())))
                    if self._stop.is_set():
                        break
                    match_t = time.time() - self._started_at
                    self._match_time = match_t
                    self._elixir.update(match_t)
                    if self._try_execute(
                        chosen["phone"],
                        chosen["pred"],
                        match_t,
                        settle_sleep=POST_PLAY_SLEEP_S,
                        force_hand=False,
                    ):
                        played_phone = chosen["phone"]
                        break

                if played_phone is None:
                    time.sleep(0.12)
                    continue

        except Exception as exc:  # noqa: BLE001
            self._error = str(exc)
            self._append_log(f"battle crashed: {exc}", "err")
        finally:
            self._match_time = (
                time.time() - self._started_at if self._started_at else 0.0
            )
            with self._lock:
                self._running = False
            self._append_log("battle stopped", "info")

    def _fresh_observed_elixir(
        self, phone: str, *, reconcile: bool = False
    ) -> dict[str, Any] | None:
        if self._observe_elixir is None:
            return None
        observation = self._observe_elixir(phone)
        if not observation or observation.get("value") is None:
            return None
        if float(observation.get("confidence") or 0.0) < ELIXIR_MIN_CONFIDENCE:
            return None
        if float(observation.get("age_s", 999.0)) > ELIXIR_MAX_AGE_S:
            return None
        clean = dict(observation)
        clean["value"] = float(clean["value"])
        if reconcile:
            self._elixir.reconcile(phone, clean["value"])
        return clean

    @staticmethod
    def _hand_frame_stamp(hand: list[dict[str, Any]]) -> float:
        return max((float(row.get("_captured_at") or 0.0) for row in hand), default=0.0)

    def _elixir_confirms_spend(
        self,
        before: dict[str, Any] | None,
        after: dict[str, Any] | None,
        cost: int,
        match_t: float,
    ) -> bool:
        if before is None or after is None:
            return False
        if float(after.get("captured_at") or 0.0) <= float(
            before.get("captured_at") or 0.0
        ):
            return False
        elapsed = max(
            0.0,
            float(after["captured_at"]) - float(before["captured_at"]),
        )
        drop = float(before["value"]) - float(after["value"])
        # Between the two visual samples, regeneration can hide part of the
        # spend.  Capping at 10 can only make the raw drop stronger evidence.
        possible_spend = drop + elapsed * self._elixir.rate(match_t)
        return drop >= 0.20 and drop <= cost + 0.8 and possible_spend >= cost - 0.75

    def _record_attempt(self, attempt: dict[str, Any]) -> None:
        with self._lock:
            self._attempts.append(attempt)

    def _try_execute(
        self,
        phone: str,
        pred: dict[str, Any],
        match_t: float,
        *,
        settle_sleep: float = POST_PLAY_SLEEP_S,
        force_hand: bool = True,
    ) -> bool:
        """Attempt one play. Returns True if a tap was performed."""
        assert self._config is not None

        hand = self._refresh_hand(phone, force=force_hand)
        if hand_is_empty(hand):
            self._append_log(f"{phone}: skip — empty hand", "err")
            return False

        card = base_card(pred.get("card"))
        slot = find_hand_slot(card, hand)
        if slot is None:
            self._append_log(
                f"{phone}: model-selected {card or '?'} is not in the refreshed hand; replan",
                "err",
            )
            return False
        cost = int(self._costs.get(card, 4))

        observed_before = self._fresh_observed_elixir(phone, reconcile=True)
        if self._observe_elixir is not None and observed_before is None:
            self._append_log(
                f"{phone}: wait — no fresh high-confidence elixir observation",
                "info",
            )
            return False
        available = (
            float(observed_before["value"])
            if observed_before is not None
            else self._elixir.values[phone]
        )
        if available + 1e-6 < cost + 0.08:
            self._append_log(
                f"{phone}: wait elixir ({available:.1f} < {cost}) for {card}",
                "info",
            )
            return False

        u, v = logic_xy_to_uv(pred["x"], pred["y"])
        try:
            self._execute_action(phone, slot, u, v)
        except Exception as exc:  # noqa: BLE001
            self._append_log(f"{phone}: tap failed: {exc}", "err")
            return False

        attempt: dict[str, Any] = {
            "seconds": match_t,
            "side": phone,
            "intended_card": card,
            "slot": slot,
            "cost": cost,
            "x": int(pred.get("x") or 9000),
            "y": int(pred.get("y") or 8000),
            "elixir_before": observed_before,
            "hand_before": [
                {
                    "slot": row.get("slot"),
                    "card_name": row.get("card_name"),
                    "confidence": row.get("confidence"),
                }
                for row in hand
            ],
        }

        # A deployment is accepted only after fresh, stable post-action evidence.
        # This covers game/UI latency and prevents one YOLO flicker from creating
        # a phantom card event.
        if settle_sleep > 0:
            time.sleep(settle_sleep)
        deadline = time.monotonic() + CONFIRM_TIMEOUT_S
        seen_stamps: set[float] = set()
        changed_name: str | None = None
        changed_streak = 0
        confirmed = False
        confirmation_method = "none"
        after_hand = hand
        observed_after: dict[str, Any] | None = None
        selected_confidence = float(
            next(
                (
                    row.get("confidence") or 0.0
                    for row in hand
                    if row.get("slot") == slot
                ),
                0.0,
            )
        )
        while time.monotonic() < deadline and not self._stop.is_set():
            self._hand_at[phone] = 0.0
            after_hand = self._refresh_hand(phone, force=True)
            stamp = self._hand_frame_stamp(after_hand)
            if stamp and stamp not in seen_stamps:
                seen_stamps.add(stamp)
                after_card = next(
                    (
                        base_card(row.get("card_name"))
                        for row in after_hand
                        if row.get("slot") == slot
                    ),
                    None,
                )
                slot_changed = hand_confirms_play(hand, after_hand, slot, card)
                if slot_changed is True and after_card:
                    if after_card == changed_name:
                        changed_streak += 1
                    else:
                        changed_name = after_card
                        changed_streak = 1
                elif slot_changed is False:
                    changed_name = None
                    changed_streak = 0

                candidate_after = self._fresh_observed_elixir(phone)
                if candidate_after is not None:
                    observed_after = candidate_after
                spend_seen = self._elixir_confirms_spend(
                    observed_before, observed_after, cost, match_t
                )
                stable_change = changed_streak >= CONFIRM_STABLE_FRAMES
                if stable_change and (
                    spend_seen or observed_after is None or cost == 1
                ):
                    confirmed = True
                    confirmation_method = (
                        "stable_slot+elixir" if spend_seen else "stable_slot"
                    )
                    break
                if spend_seen and selected_confidence >= 0.60:
                    confirmed = True
                    confirmation_method = "elixir_drop"
                    break
            time.sleep(CONFIRM_POLL_S)

        if observed_after is not None:
            self._elixir.reconcile(phone, float(observed_after["value"]))
        attempt.update(
            {
                "outcome": "confirmed" if confirmed else "unconfirmed",
                "confirmation_method": confirmation_method,
                "elixir_after": observed_after,
                "hand_after": [
                    {
                        "slot": row.get("slot"),
                        "card_name": row.get("card_name"),
                        "confidence": row.get("confidence"),
                    }
                    for row in after_hand
                ],
            }
        )
        self._record_attempt(attempt)
        if not confirmed:
            self._append_log(
                f"{phone}: deployment unresolved — slot {slot}, intended {card}",
                "err",
            )
            return False

        if observed_after is None:
            self._elixir.spend(phone, cost)
        event = {
            "seconds": match_t,
            "side": phone,
            "event_type": "card_play",
            "card": card,
            "x": int(pred.get("x") or 9000),
            "y": int(pred.get("y") or 8000),
            "model_slot": int(pred.get("slot", -1)),
            "tile": pred.get("tile"),
            "slot_decode": pred.get("slot_decode"),
            "placement_decode": pred.get("placement_decode"),
            "think_steps": pred.get("think_steps"),
            "confirmation_method": confirmation_method,
            "observed_elixir_before": (
                observed_before.get("value") if observed_before else None
            ),
            "observed_elixir_after": (
                observed_after.get("value") if observed_after else None
            ),
        }
        with self._lock:
            self._events.append(event)
        self._last_play_phone = phone
        # Hand is stale after a play until we redetect.
        self._hand_at[phone] = 0.0
        self._append_log(
            f"{match_t:5.1f}s  {phone}  {card}  uv=({u:.2f},{v:.2f})  "
            f"elixir→{self._elixir.values[phone]:.1f} "
            f"[{pred.get('slot_decode')} / {pred.get('placement_decode')} / K={pred.get('think_steps')} ]",
            "ok",
        )
        return True
