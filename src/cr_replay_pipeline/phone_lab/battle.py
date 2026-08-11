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
        "arrows",
        "fireball",
        "giant",
        "mini-pekka",
        "musketeer",
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
POST_PLAY_SLEEP_S = 0.28  # just enough for CR to accept the next tap
HAND_MAX_AGE_S = 0.55  # reuse YOLO result unless older than this

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
        if name:
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
    policy_dirs: dict[str, Path] = field(default_factory=lambda: dict(DEFAULT_POLICY_DIRS))
    mirror_tta: bool = False
    think_steps: int | None = None


class BattleRunner:
    """Background match loop shared by both phones."""

    def __init__(
        self,
        *,
        detect_hand: Callable[[str], list[dict[str, Any]]],
        execute_action: Callable[[str, int, float, float], dict[str, Any]],
        log: Callable[[str, str], None] | None = None,
    ):
        self._detect_hand = detect_hand
        self._execute_action = execute_action
        self._log = log or (lambda msg, level="info": None)
        self._lock = threading.Lock()
        self._thread: threading.Thread | None = None
        self._stop = threading.Event()
        self._running = False
        self._elixir = ElixirTracker()
        self._events: list[dict[str, Any]] = []
        self._log_lines: list[dict[str, Any]] = []
        self._hands: dict[str, list[dict[str, Any]]] = {k: [] for k in PHONE_KEYS}
        self._hand_at: dict[str, float] = {k: 0.0 for k in PHONE_KEYS}
        self._next: dict[str, dict[str, Any] | None] = {k: None for k in PHONE_KEYS}
        self._config: BattleConfig | None = None
        self._policies: dict[str, PolicyBundle] = {}
        self._costs: dict[str, int] = {}
        self._started_at = 0.0
        self._match_time = 0.0
        self._error: str | None = None
        self._last_play_phone: str | None = None
        self._rng = random.Random(441)

    @property
    def running(self) -> bool:
        return self._running

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
                    "think_steps": (
                        self._config.think_steps if self._config else None
                    ),
                    "history": "confirmed deployments only",
                },
                "log": list(self._log_lines[-80:]),
                "events": len(self._events),
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
        )

        needed = {c for c in controllers.values() if c != CONTROLLER_MANUAL}
        policies: dict[str, PolicyBundle] = {}
        for key in needed:
            path = Path(config.policy_dirs.get(key) or DEFAULT_POLICY_DIRS[key])
            if not (path / "best_model.pt").exists():
                raise FileNotFoundError(
                    f"missing policy checkpoint: {path / 'best_model.pt'}"
                )
            model, vocab, cfg, device = load_policy(path)
            policies[key] = PolicyBundle(
                key=key, model=model, vocab=vocab, cfg=cfg, device=device
            )

        self.stop()
        self._stop.clear()
        self._error = None
        self._config = config
        self._policies = policies
        self._costs = costs
        self._events = []
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
            k
            for k in PHONE_KEYS
            if self._config.controllers[k] != CONTROLLER_MANUAL
        ]

    def _refresh_hand(
        self, phone: str, *, force: bool = False
    ) -> list[dict[str, Any]]:
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
        hand_cards = set(detected_hand_cards(hand))
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
            hand = self._refresh_hand(phone, force=hand_is_empty(self._hands.get(phone) or []))
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
            delay = self._plan_delay(phone, float(pred.get("delay_seconds") or MIN_DELAY_S))
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
                mystery = [
                    n for n in detected_hand_cards(slots) if n not in deck_set
                ]
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

                nonempty = [
                    p for p in ai_phones if not hand_is_empty(self._hands[p])
                ]
                if ai_phones and not nonempty:
                    empty_streak += 1
                    self._append_log(
                        f"all AI hands empty ({empty_streak}/{MAX_EMPTY_HAND_STREAK})",
                        "err",
                    )
                    if empty_streak >= MAX_EMPTY_HAND_STREAK:
                        self._append_log(
                            "stopping: YOLO lost both hands "
                            "(match over / overlay / wrong screen?)",
                            "err",
                        )
                        break
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

        if not self._elixir.can_afford(phone, cost):
            self._append_log(
                f"{phone}: wait elixir ({self._elixir.values[phone]:.1f} < {cost}) "
                f"for {card}",
                "info",
            )
            return False

        u, v = logic_xy_to_uv(pred["x"], pred["y"])
        try:
            self._execute_action(phone, slot, u, v)
        except Exception as exc:  # noqa: BLE001
            self._append_log(f"{phone}: tap failed: {exc}", "err")
            return False

        # Do not mutate policy history or estimated elixir merely because ADB/scrcpy
        # accepted the input. Confirm that Clash Royale actually replaced the slot.
        if settle_sleep > 0:
            time.sleep(settle_sleep)
        self._hand_at[phone] = 0.0
        after_hand = self._refresh_hand(phone, force=True)
        confirmed = hand_confirms_play(hand, after_hand, slot, card)
        if confirmed is None:
            self._hand_at[phone] = 0.0
            after_hand = self._refresh_hand(phone, force=True)
            confirmed = hand_confirms_play(hand, after_hand, slot, card)
        if confirmed is not True:
            self._append_log(
                f"{phone}: tap not confirmed — slot {slot} still shows {card}",
                "err",
            )
            return False

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
