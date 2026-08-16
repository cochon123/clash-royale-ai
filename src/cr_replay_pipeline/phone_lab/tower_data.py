"""Autonomous dual-phone tower-HP data collection.

The collector deliberately keeps the expensive part (real Clash Royale play) small:
it opens a friendly battle, lets the existing policy harness play, samples tower HP
from one phone, persists every raw OCR crop/reading, and repeats.  OCR is optional at
import time so the rest of phone-lab still works on machines without RapidOCR.
"""

from __future__ import annotations

import ctypes
import gc
import io
import json
import re
import sys
import threading
import time
import uuid
from dataclasses import asdict, dataclass
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any

import numpy as np
from PIL import Image

from .battle import PHONE_KEYS, BattleConfig

TOWER_KEYS = (
    "opponent_left_princess",
    "opponent_king",
    "opponent_right_princess",
    "team_left_princess",
    "team_king",
    "team_right_princess",
)

# Friendly battles are tournament standard. Keep these as metadata/validation
# bounds, not hard labels: balance updates can change the actual values.
DEFAULT_MAX_HP = {"princess": 3052, "king": 4824}
RAPIDOCR_SITE = Path(
    "/home/cochon/.local/share/flameshot-ocr/venv/lib/python3.14/site-packages"
)


def _release_native_ocr_buffers() -> None:
    """Return OpenCV/ONNX temporary allocations to the OS.

    RapidOCR's full-frame detector uses dynamically-sized native buffers.  On
    glibc those buffers remain in malloc arenas after each inference, which made
    a long collection grow by roughly 40--60 MB per sample.  Python GC alone
    cannot release them; ``malloc_trim`` can.
    """
    gc.collect()
    try:
        libc = ctypes.CDLL("libc.so.6")
        libc.malloc_trim(0)
    except (OSError, AttributeError):
        # Non-glibc platforms still benefit from dropping Python references.
        pass


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat()


def _atomic_json(path: Path, payload: dict[str, Any]) -> None:
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(json.dumps(payload, indent=2, sort_keys=True), encoding="utf-8")
    tmp.replace(path)


def _append_jsonl(path: Path, payload: dict[str, Any]) -> None:
    with path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(payload, separators=(",", ":")) + "\n")


def _digits(text: str) -> int | None:
    compact = re.sub(r"[^0-9]", "", str(text))
    if not (2 <= len(compact) <= 4):
        return None
    value = int(compact)
    return value if 20 <= value <= 6000 else None


@dataclass
class OcrToken:
    text: str
    score: float
    cx: float
    cy: float
    box: list[list[float]]


class RapidOcrReader:
    """Lazy wrapper around the already-installed Flameshot RapidOCR runtime."""

    def __init__(self) -> None:
        self._engine: Any = None
        self._lock = threading.Lock()
        self.error: str | None = None

    @property
    def ready(self) -> bool:
        return self._engine is not None and self.error is None

    def _load(self) -> Any:
        if self._engine is not None:
            return self._engine
        try:
            if RAPIDOCR_SITE.is_dir() and str(RAPIDOCR_SITE) not in sys.path:
                sys.path.insert(0, str(RAPIDOCR_SITE))
            from rapidocr import RapidOCR  # type: ignore[import-not-found]

            self._engine = RapidOCR(
                params={
                    "Global.log_level": "warning",
                    # Clash UI text is horizontal, so the orientation model is
                    # pure overhead.  Bound OCR to two CPU workers so YOLO,
                    # policy inference, streaming, and the browser stay fluid.
                    "Global.use_cls": False,
                    "EngineConfig.onnxruntime.intra_op_num_threads": 2,
                    "EngineConfig.onnxruntime.inter_op_num_threads": 1,
                    # Downscale the long edge rather than upscaling the short
                    # edge of a portrait screenshot.  960 px retains the tower
                    # digits while substantially reducing detector work.
                    "Det.limit_type": "max",
                    "Det.limit_side_len": 960,
                }
            )
            self.error = None
            return self._engine
        except Exception as exc:  # noqa: BLE001
            self.error = str(exc)
            raise RuntimeError(f"RapidOCR unavailable: {exc}") from exc

    def tokens(self, image: Image.Image | bytes) -> list[OcrToken]:
        if isinstance(image, bytes):
            image = Image.open(io.BytesIO(image)).convert("RGB")
        arr = np.asarray(image.convert("RGB"))
        try:
            with self._lock:
                out = self._load()(arr)
            boxes = out.boxes if out and out.boxes is not None else []
            texts = out.txts if out and out.txts is not None else []
            scores = out.scores if out and out.scores is not None else []
            result: list[OcrToken] = []
            for box, text, score in zip(boxes, texts, scores):
                points = [[float(p[0]), float(p[1])] for p in box]
                result.append(
                    OcrToken(
                        text=str(text),
                        score=float(score),
                        cx=sum(p[0] for p in points) / len(points),
                        cy=sum(p[1] for p in points) / len(points),
                        box=points,
                    )
                )
            return result
        finally:
            # Drop RapidOCROutput.imgs and its native backing allocations before
            # the next sample.  This is the critical long-run RAM bound.
            if "out" in locals():
                del out
            del arr
            _release_native_ocr_buffers()


class TowerHpReader:
    """Read six tower values and retain evidence for every assignment."""

    def __init__(self, ocr: RapidOcrReader):
        self.ocr = ocr

    @staticmethod
    def expected_centres(width: int, height: int) -> dict[str, tuple[float, float]]:
        # These are HP-label centres in the fixed portrait battle UI. Browser
        # calibration can override them; full OCR still gives us auditable boxes.
        return {
            "opponent_left_princess": (0.202 * width, 0.199 * height),
            "opponent_king": (0.500 * width, 0.155 * height),
            "opponent_right_princess": (0.792 * width, 0.199 * height),
            "team_left_princess": (0.202 * width, 0.627 * height),
            "team_king": (0.500 * width, 0.735 * height),
            "team_right_princess": (0.792 * width, 0.627 * height),
        }

    @staticmethod
    def _candidate_centres(
        key: str,
        centre: tuple[float, float],
        width: int,
        height: int,
    ) -> tuple[tuple[float, float], ...]:
        """Return every stable UI position used by one tower's HP label.

        An activated king gets a wide HP bar above/below the dormant crown.
        Tiebreak/result animations also shift both king bars. Princess labels
        stay anchored, so their user calibration remains authoritative.
        """
        if key == "opponent_king":
            return (centre, (0.520 * width, 0.095 * height))
        if key == "team_king":
            return (centre, (0.520 * width, 0.756 * height))
        return (centre,)

    def read(
        self,
        image: Image.Image,
        centres: dict[str, tuple[float, float]] | None = None,
    ) -> tuple[dict[str, Any], list[OcrToken]]:
        width, height = image.size
        centres = centres or self.expected_centres(width, height)
        tokens = self.ocr.tokens(image)
        return self.assign_tokens(width, height, tokens, centres), tokens

    def assign_tokens(
        self,
        width: int,
        height: int,
        tokens: list[OcrToken],
        centres: dict[str, tuple[float, float]] | None = None,
    ) -> dict[str, Any]:
        centres = centres or self.expected_centres(width, height)
        candidates = [t for t in tokens if _digits(t.text) is not None]
        assigned: dict[str, Any] = {}
        used: set[int] = set()
        # Nearest-neighbour matching with strict distance guards prevents clock,
        # elixir and spectator text from silently becoming tower labels.
        for key, centre in centres.items():
            best: tuple[float, int, OcrToken] | None = None
            for i, token in enumerate(candidates):
                if i in used:
                    continue
                dist = min(
                    (
                        ((token.cx - ex) / max(width, 1)) ** 2
                        + 2.4 * ((token.cy - ey) / max(height, 1)) ** 2
                    )
                    ** 0.5
                    for ex, ey in self._candidate_centres(key, centre, width, height)
                )
                if best is None or dist < best[0]:
                    best = (dist, i, token)
            if best is None or best[0] > 0.115:
                assigned[key] = {"hp": None, "confidence": 0.0, "reason": "not_visible"}
                continue
            dist, i, token = best
            used.add(i)
            assigned[key] = {
                "hp": _digits(token.text),
                "ocr_observed": True,
                "confidence": round(token.score * max(0.0, 1.0 - dist / 0.115), 5),
                "ocr_score": round(token.score, 5),
                "text": token.text,
                "center": [round(token.cx, 1), round(token.cy, 1)],
                "box": token.box,
                "distance": round(dist, 5),
                "layout": (
                    "activated"
                    if "_king" in key
                    and min(
                        abs(token.cy - ey) / max(height, 1)
                        for _ex, ey in self._candidate_centres(
                            key, centres[key], width, height
                        )[1:]
                    )
                    < 0.04
                    else "dormant"
                ),
            }
        return assigned


def stabilize_tower_hp(
    hp: dict[str, Any],
    stable_hp: dict[str, int],
    sample_index: int,
    pending_hp: dict[str, int] | None = None,
) -> dict[str, Any]:
    """Apply HP bounds plus confidence-aware temporal consistency.

    Clean OCR at the expected geometry is accepted immediately, which preserves
    rapid damage. Borderline OCR needs a second consecutive lower reading. The
    newer reading wins, so a false-low first frame cannot poison the trajectory.
    """
    pending_hp = pending_hp if pending_hp is not None else {}
    for key, reading in hp.items():
        value = reading.get("hp")
        maximum = (
            DEFAULT_MAX_HP["king"] if "_king" in key else DEFAULT_MAX_HP["princess"]
        )
        valid = (
            value is not None
            and 0 <= int(value) <= maximum
            and float(reading.get("confidence") or 0.0) >= 0.20
            and int(value) <= stable_hp[key]
        )
        high_quality = (
            valid
            and float(reading.get("ocr_score") or 0.0) >= 0.90
            and float(reading.get("distance") or 1.0) <= 0.045
        )
        if valid and int(value) == stable_hp[key]:
            pending_hp.pop(key, None)
            reading["label_source"] = "ocr_monotonic"
        elif high_quality:
            stable_hp[key] = int(value)
            pending_hp.pop(key, None)
            reading["label_source"] = "ocr_direct_decrease"
        elif valid and key in pending_hp:
            stable_hp[key] = int(value)
            pending_hp.pop(key, None)
            reading["label_source"] = "ocr_confirmed_decrease"
        elif valid:
            pending_hp[key] = int(value)
            reading["raw_hp"] = value
            reading["hp"] = stable_hp[key]
            reading["confidence"] = 0.25
            reading["label_source"] = "pending_decrease"
        else:
            pending_hp.pop(key, None)
            reading["raw_hp"] = value
            reading["hp"] = stable_hp[key]
            reading["confidence"] = 0.35 if sample_index == 0 else 0.55
            reading["label_source"] = (
                "known_friendly_max" if sample_index == 0 else "monotonic_carry_forward"
            )
    return hp


def _result_phase(tokens: list[OcrToken]) -> str:
    text = " ".join(token.text.lower() for token in tokens)
    if "tiebreak" in text:
        return "tiebreak"
    if re.search(r"\b(enemy|winner|defeat|victory)\b", text):
        return "result"
    if re.search(r"\b(ok|continue)\b", text):
        return "result_sheet"
    return "battle"


def _apply_known_destroyed_towers(
    hp: dict[str, Any],
    destroyed: set[str],
    *,
    label_source: str = "destroyed_visible_audit",
) -> dict[str, Any]:
    for key in destroyed:
        reading = hp[key]
        reading["raw_hp"] = reading.get("hp")
        reading["hp"] = 0
        reading["confidence"] = 1.0
        reading["label_source"] = label_source
        reading["destroyed"] = True
    return hp


def infer_destroyed_towers(
    hp: dict[str, Any],
    stable_hp: dict[str, int],
    missing_streak: dict[str, int],
    destroyed: set[str],
) -> set[str]:
    """Infer a princess destruction from its missing label + active king bar."""
    for side in ("opponent", "team"):
        king = hp[f"{side}_king"]
        active_king = bool(
            king.get("ocr_observed") and king.get("layout") == "activated"
        )
        damaged_lanes = [
            lane
            for lane in ("left", "right")
            if stable_hp[f"{side}_{lane}_princess"] < DEFAULT_MAX_HP["princess"]
        ]
        for lane in ("left", "right"):
            key = f"{side}_{lane}_princess"
            if key in destroyed:
                continue
            if stable_hp[key] < DEFAULT_MAX_HP["princess"] and not hp[key].get(
                "ocr_observed"
            ):
                missing_streak[key] = missing_streak.get(key, 0) + 1
            else:
                missing_streak[key] = 0
            # If both princesses are damaged, disappearance + king activation
            # cannot identify which lane fell. Preserve both values for audit.
            if active_king and damaged_lanes == [lane] and missing_streak[key] >= 2:
                destroyed.add(key)
                stable_hp[key] = 0
    return destroyed


def _add_training_weight(hp: dict[str, Any], *, phase: str) -> dict[str, Any]:
    weights = {
        "ocr_monotonic": 1.0,
        "ocr_direct_decrease": 1.0,
        "ocr_confirmed_decrease": 0.85,
        "destroyed_visible_audit": 1.0,
        "destroyed_ui_inference": 0.90,
        "hp_visible_audit": 1.0,
        "pending_decrease": 0.20,
        "monotonic_carry_forward": 0.25,
        "known_friendly_max": 0.15,
    }
    for reading in hp.values():
        reading["training_weight"] = (
            0.0 if phase != "battle" else weights.get(reading["label_source"], 0.0)
        )
    return hp


def relabel_saved_game(
    game_dir: str | Path,
    *,
    width: int = 1080,
    height: int = 2424,
    destroyed_from: dict[str, int] | None = None,
    audited_hp_from: dict[str, list[tuple[int, int]]] | None = None,
    tiebreak_from: int | None = None,
    result_from: int | None = None,
) -> Path:
    """Rebuild labels from saved evidence without replaying a match."""
    game_dir = Path(game_dir)
    source = game_dir / "tower_hp.jsonl"
    target = game_dir / "tower_hp_relabelled.jsonl"
    reader = TowerHpReader(RapidOcrReader())
    stable_hp = {
        key: (DEFAULT_MAX_HP["king"] if "_king" in key else DEFAULT_MAX_HP["princess"])
        for key in TOWER_KEYS
    }
    pending_hp: dict[str, int] = {}
    destroyed_from = dict(destroyed_from or {})
    audited_hp_from = dict(audited_hp_from or {})
    rows: list[str] = []
    for sample_index, raw in enumerate(source.read_text(encoding="utf-8").splitlines()):
        row = json.loads(raw)
        tokens = [OcrToken(**token) for token in row.get("ocr_tokens") or []]
        hp = reader.assign_tokens(width, height, tokens)
        phase = _result_phase(tokens)
        if tiebreak_from is not None and sample_index >= tiebreak_from:
            phase = "tiebreak"
        if result_from is not None and sample_index >= result_from:
            phase = "result"
        row["game_phase"] = phase
        row["training_mask"] = phase == "battle"
        hp = stabilize_tower_hp(hp, stable_hp, sample_index, pending_hp)
        for key, breakpoints in audited_hp_from.items():
            applicable = [
                value
                for first_sample, value in breakpoints
                if sample_index >= first_sample
            ]
            if applicable:
                value = int(applicable[-1])
                stable_hp[key] = value
                pending_hp.pop(key, None)
                reading = hp[key]
                reading["raw_hp"] = reading.get("hp")
                reading["hp"] = value
                reading["confidence"] = 1.0
                reading["label_source"] = "hp_visible_audit"
        destroyed = {
            key
            for key, first_sample in destroyed_from.items()
            if sample_index >= first_sample
        }
        for key in destroyed:
            stable_hp[key] = 0
            pending_hp.pop(key, None)
        hp = _apply_known_destroyed_towers(hp, destroyed)
        row["tower_hp"] = _add_training_weight(hp, phase=phase)
        if destroyed:
            row["audited_destroyed_towers"] = sorted(destroyed)
        row["relabelled_at"] = utc_now()
        rows.append(json.dumps(row, separators=(",", ":")))
    target.write_text("\n".join(rows) + ("\n" if rows else ""), encoding="utf-8")
    battle_path = game_dir / "battle.json"
    if rows and battle_path.exists():
        battle = json.loads(battle_path.read_text(encoding="utf-8"))
        first = json.loads(rows[0])
        inferred_start = datetime.fromisoformat(first["timestamp"]) - timedelta(
            seconds=float(first["match_time"])
        )
        old_started_at = battle.get("started_at")
        if old_started_at and "saved_at" not in battle:
            battle["saved_at"] = old_started_at
        battle["started_at"] = inferred_start.isoformat()
        battle["started_at_source"] = "first_sample_timestamp_minus_match_time"
        _atomic_json(battle_path, battle)
    return target


@dataclass
class CollectionConfig:
    games: int
    sample_interval_s: float
    decks: dict[str, list[str]]
    controllers: dict[str, str]
    timeout_s: float = 330.0
    observer_phone: str = "pixel9"


class TowerDataCollector:
    """Own the menu → match → persist → repeat lifecycle in one thread."""

    def __init__(
        self,
        *,
        phones: dict[str, Any],
        battle: Any,
        card_costs_path: Path,
        policy_dirs: dict[str, Path],
        output_root: Path = Path("data/tower_hp_runs"),
        mirror_tta: bool = False,
        think_steps: int | None = None,
    ) -> None:
        self.phones = phones
        self.battle = battle
        self.card_costs_path = Path(card_costs_path)
        self.policy_dirs = {k: Path(v) for k, v in policy_dirs.items()}
        self.output_root = Path(output_root)
        self.mirror_tta = mirror_tta
        self.think_steps = think_steps
        self.ocr = RapidOcrReader()
        self.hp_reader = TowerHpReader(self.ocr)
        self._thread: threading.Thread | None = None
        self._stop = threading.Event()
        self._lock = threading.Lock()
        self._running = False
        self._phase = "idle"
        self._error: str | None = None
        self._run_dir: Path | None = None
        self._config: CollectionConfig | None = None
        self._games_done = 0
        self._samples = 0
        self._started_at = 0.0
        self._game_started_at = 0.0
        self._last_hp: dict[str, Any] = {}
        self._log: list[dict[str, Any]] = []
        self._centres: dict[str, dict[str, tuple[float, float]]] = {}

    def status(self) -> dict[str, Any]:
        with self._lock:
            total = self._config.games if self._config else 0
            remaining = max(0, total - self._games_done)
            timeout_s = self._config.timeout_s if self._config else 330.0
            active_s = (
                max(0.0, time.monotonic() - self._game_started_at)
                if self._running and self._game_started_at
                else 0.0
            )
            eta_s = max(0.0, remaining * timeout_s - active_s)
            return {
                "running": self._running,
                "phase": self._phase,
                "error": self._error,
                "run_dir": str(self._run_dir) if self._run_dir else None,
                "games_done": self._games_done,
                "games_total": total,
                "work": f"{self._games_done}/{total} games · {self._samples} samples",
                "eta_s": round(eta_s, 1) if total else 0.0,
                "samples": self._samples,
                "last_hp": dict(self._last_hp),
                "ocr_ready": self.ocr.ready,
                "ocr_error": self.ocr.error,
                "calibration": {
                    phone: {key: list(value) for key, value in points.items()}
                    for phone, points in self._centres.items()
                },
                "log": list(self._log[-80:]),
            }

    def set_calibration(
        self, phone: str, centres: dict[str, list[float] | tuple[float, float]]
    ) -> dict[str, Any]:
        if phone not in PHONE_KEYS:
            raise ValueError(f"unknown phone: {phone}")
        missing = [key for key in TOWER_KEYS if key not in centres]
        if missing:
            raise ValueError(f"tower calibration missing: {missing}")
        clean: dict[str, tuple[float, float]] = {}
        for key in TOWER_KEYS:
            raw = centres[key]
            if len(raw) != 2:
                raise ValueError(f"invalid centre for {key}")
            u, v = float(raw[0]), float(raw[1])
            if not (0.0 <= u <= 1.0 and 0.0 <= v <= 1.0):
                raise ValueError(f"centre for {key} must be normalized")
            clean[key] = (u, v)
        with self._lock:
            self._centres[phone] = clean
        return self.status()

    def _say(self, message: str, level: str = "info") -> None:
        with self._lock:
            self._log.append({"at": utc_now(), "level": level, "message": message})
            self._log = self._log[-200:]

    def start(self, config: CollectionConfig) -> dict[str, Any]:
        if self._running:
            raise RuntimeError("tower data collection already running")
        if config.games < 1:
            raise ValueError("games must be >= 1")
        if config.observer_phone not in PHONE_KEYS:
            raise ValueError(f"unknown observer phone: {config.observer_phone}")
        # Fail before touching either phone if OCR cannot initialize.
        self.ocr._load()
        run_id = datetime.now().strftime("%Y%m%d_%H%M%S") + "_" + uuid.uuid4().hex[:6]
        self._run_dir = self.output_root / run_id
        self._run_dir.mkdir(parents=True, exist_ok=False)
        self._config = config
        self._stop.clear()
        self._error = None
        self._games_done = 0
        self._samples = 0
        self._started_at = time.time()
        self._game_started_at = 0.0
        self._last_hp = {}
        self._log = []
        self._running = True
        self._phase = "starting"
        self._thread = threading.Thread(
            target=self._run, name="tower-data", daemon=True
        )
        self._thread.start()
        return self.status()

    def stop(self) -> None:
        self._stop.set()
        self.battle.stop()
        thread = self._thread
        if thread and thread.is_alive() and thread is not threading.current_thread():
            thread.join(timeout=10.0)
        with self._lock:
            self._running = False
            self._phase = "stopped"

    def _screen(self, phone: str) -> Image.Image:
        return self.phones[phone].screencap_image()

    def _tokens(self, phone: str) -> list[OcrToken]:
        return self.ocr.tokens(self._screen(phone))

    @staticmethod
    def _matches(token: OcrToken, choices: tuple[str, ...]) -> bool:
        text = re.sub(r"\s+", " ", token.text.lower()).strip()
        return any(choice.lower() in text for choice in choices)

    def _tap_text(
        self,
        phone: str,
        choices: tuple[str, ...],
        *,
        timeout_s: float = 12.0,
        prefer_lowest: bool = False,
    ) -> bool:
        deadline = time.monotonic() + timeout_s
        while not self._stop.is_set() and time.monotonic() < deadline:
            matches = [t for t in self._tokens(phone) if self._matches(t, choices)]
            if matches:
                token = (
                    max(matches, key=lambda t: t.cy) if prefer_lowest else matches[0]
                )
                self.phones[phone].tap(round(token.cx), round(token.cy))
                self._say(f"{phone}: tapped {token.text!r}")
                return True
            time.sleep(0.45)
        return False

    def _open_friendly_battle(self) -> None:
        """Challenge from Pixel 8 (I am a Noob), accept on Pixel 9 (cochon)."""
        self._phase = "recovering_home"
        for key in PHONE_KEYS:
            phone = self.phones[key]
            # A previous run may have ended on the result sheet. Clear it by
            # text before using stable bottom navigation coordinates.
            tokens = self._tokens(key)
            result_buttons = [
                token for token in tokens if self._matches(token, ("ok", "continue"))
            ]
            if result_buttons:
                button = max(result_buttons, key=lambda token: token.cy)
                phone.tap(round(button.cx), round(button.cy))
                self._say(f"{key}: dismissed {button.text!r}")
                time.sleep(2.0)
            # Social is a top-level tab; Android Back intentionally leaves it in
            # place. Select the bottom Battle tab, then verify the home screen.
            phone.tap(round(phone.width * 0.42), round(phone.height * 0.96))
            time.sleep(0.8)
            for _attempt in range(3):
                tokens = self._tokens(key)
                if any(self._matches(t, ("pass royale",)) for t in tokens):
                    break
                phone.tap(round(phone.width * 0.42), round(phone.height * 0.96))
                time.sleep(0.7)
            else:
                raise RuntimeError(f"could not recover {key} to the home screen")
        self._phase = "opening_social"
        # Bottom Social icon is stable across both 1080px phones.
        for key in PHONE_KEYS:
            phone = self.phones[key]
            phone.tap(round(phone.width * 0.76), round(phone.height * 0.96))
        time.sleep(1.7)

        self._phase = "selecting_friend"
        if not self._tap_text("pixel8", ("cochon",), timeout_s=15.0):
            raise RuntimeError("could not find cochon in Pixel 8 friend list")
        time.sleep(0.7)
        if not self._tap_text(
            "pixel8", ("friendly battle", "friendly"), timeout_s=10.0
        ):
            raise RuntimeError("could not open Friendly Battle on Pixel 8")
        time.sleep(0.8)
        if not self._tap_text("pixel8", ("1v1 battle", "1v1"), timeout_s=10.0):
            raise RuntimeError("could not select 1v1 Battle on Pixel 8")

        self._phase = "accepting_invite"
        time.sleep(1.0)
        if not self._tap_text(
            "pixel9",
            ("friendly battle", "friendly"),
            timeout_s=18.0,
            prefer_lowest=True,
        ):
            raise RuntimeError("Pixel 9 did not expose the Friendly Battle invite")
        self._say("friendly battle accepted", "ok")
        self._phase = "loading_match"
        time.sleep(7.0)

    def _dismiss_results(self) -> None:
        self._phase = "dismissing_results"
        for key in PHONE_KEYS:
            if self._tap_text(
                key, ("ok", "continue"), timeout_s=3.0, prefer_lowest=True
            ):
                continue
            # Result buttons are bottom-centre. A fallback tap is safe here and
            # is followed by a fresh OCR-driven Social navigation next game.
            phone = self.phones[key]
            phone.tap(round(phone.width * 0.5), round(phone.height * 0.82))
        time.sleep(2.5)

    def _sample_game(self, game_dir: Path, started: float) -> None:
        assert self._config is not None
        observer = self._config.observer_phone
        sample_path = game_dir / "tower_hp.jsonl"
        crops_dir = game_dir / "crops"
        crops_dir.mkdir(parents=True, exist_ok=True)
        normalized = self._centres.get(observer)
        if normalized:
            width, height = self.phones[observer].width, self.phones[observer].height
            centres = {
                key: (u * width, v * height) for key, (u, v) in normalized.items()
            }
        else:
            centres = self.hp_reader.expected_centres(
                self.phones[observer].width, self.phones[observer].height
            )
        sample_index = 0
        stable_hp = {
            key: (
                DEFAULT_MAX_HP["king"] if "_king" in key else DEFAULT_MAX_HP["princess"]
            )
            for key in TOWER_KEYS
        }
        pending_hp: dict[str, int] = {}
        missing_streak: dict[str, int] = {}
        destroyed: set[str] = set()
        while self.battle.running and not self._stop.is_set():
            tick = time.monotonic()
            image = self._screen(observer)
            hp, tokens = self.hp_reader.read(image, centres)
            hp = stabilize_tower_hp(hp, stable_hp, sample_index, pending_hp)
            infer_destroyed_towers(hp, stable_hp, missing_streak, destroyed)
            hp = _apply_known_destroyed_towers(
                hp, destroyed, label_source="destroyed_ui_inference"
            )
            match_t = max(0.0, time.monotonic() - started)
            phase = _result_phase(tokens)
            row = {
                "sample": sample_index,
                "timestamp": utc_now(),
                "match_time": round(match_t, 3),
                "observer": observer,
                "game_phase": phase,
                "training_mask": phase == "battle",
                "tower_hp": _add_training_weight(hp, phase=phase),
                "ocr_tokens": [asdict(t) for t in tokens],
            }
            _append_jsonl(sample_path, row)
            # Save one compact evidence image per sample. PNG preserves the glyphs
            # exactly; labels can be corrected later without replaying a match.
            image.save(
                crops_dir / f"sample_{sample_index:04d}.webp",
                "WEBP",
                quality=88,
                method=0,
            )
            with self._lock:
                self._samples += 1
                self._last_hp = hp
            sample_index += 1
            normalized_tokens = {
                re.sub(r"[^a-z]", "", token.text.lower()) for token in tokens
            }
            if match_t >= 30.0 and (
                "ok" in normalized_tokens or "continue" in normalized_tokens
            ):
                self._say("result screen detected; match capture complete", "ok")
                self.battle.stop()
                break
            delay = self._config.sample_interval_s - (time.monotonic() - tick)
            if delay > 0:
                self._stop.wait(delay)

    def _write_manifest(self, started: float, *, complete: bool) -> None:
        assert self._run_dir is not None and self._config is not None
        _atomic_json(
            self._run_dir / "manifest.json",
            {
                "schema_version": 1,
                "created_at": datetime.fromtimestamp(started, timezone.utc).isoformat(),
                "updated_at": utc_now(),
                "complete": complete,
                "elapsed_s": round(time.time() - started, 2),
                "config": asdict(self._config),
                "games_done": self._games_done,
                "samples": self._samples,
                "error": self._error,
                "notes": [
                    "All actions are confirmed deployments from the live harness.",
                    "Every HP row retains full-frame WEBP evidence for relabeling.",
                    "OCR confidence is not ground truth; audit before model training.",
                ],
            },
        )

    def _run(self) -> None:
        assert self._config is not None and self._run_dir is not None
        started_wall = time.time()
        try:
            for game_no in range(1, self._config.games + 1):
                if self._stop.is_set():
                    break
                self._say(f"preparing game {game_no}/{self._config.games}")
                self._open_friendly_battle()
                game_dir = self._run_dir / f"game_{game_no:03d}"
                game_dir.mkdir(parents=True, exist_ok=False)
                cfg = BattleConfig(
                    decks={k: list(self._config.decks[k]) for k in PHONE_KEYS},
                    controllers=dict(self._config.controllers),
                    timeout_s=self._config.timeout_s,
                    card_costs_path=self.card_costs_path,
                    policy_dirs=self.policy_dirs,
                    mirror_tta=self.mirror_tta,
                    think_steps=self.think_steps,
                    stop_on_empty_hands=False,
                )
                self._phase = "playing"
                battle_started = time.monotonic()
                battle_started_wall = time.time()
                self._game_started_at = battle_started
                self.battle.start(cfg)
                self._sample_game(game_dir, battle_started)
                self.battle.stop()
                _atomic_json(
                    game_dir / "battle.json",
                    {
                        "game": game_no,
                        "started_at": datetime.fromtimestamp(
                            battle_started_wall, timezone.utc
                        ).isoformat(),
                        "saved_at": utc_now(),
                        "duration_s": round(time.monotonic() - battle_started, 3),
                        "events": self.battle.events_snapshot(),
                        "attempts": self.battle.attempts_snapshot(),
                        "battle_status": self.battle.status(),
                    },
                )
                self._games_done = game_no
                self._game_started_at = 0.0
                self._say(f"game {game_no} saved", "ok")
                self._write_manifest(started_wall, complete=False)
                if game_no < self._config.games and not self._stop.is_set():
                    self._dismiss_results()
            self._phase = "complete" if not self._stop.is_set() else "stopped"
        except Exception as exc:  # noqa: BLE001
            self._error = str(exc)
            self._phase = "error"
            self._say(f"collection failed: {exc}", "err")
        finally:
            self.battle.stop()
            with self._lock:
                self._running = False
            self._write_manifest(started_wall, complete=self._phase == "complete")
