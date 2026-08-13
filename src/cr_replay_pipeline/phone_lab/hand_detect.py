"""Hand-card detection from calibrated slot crops + YOLO."""

from __future__ import annotations

import io
import os
import re
import threading
from pathlib import Path
from typing import Any

from PIL import Image

DEFAULT_YOLO = Path(
    os.environ.get(
        "CR_CARD_DETECTOR_MODEL",
        "/home/cochon/Documents/ClashRoyaleAI/models/yolo/card_detector.pt",
    )
)
# Match the dual-phone v8 detector; do not force a tiny imgsz (hurts accuracy).
DEFAULT_CONF = 0.45


def normalize_detector_label(label: str | None) -> str | None:
    """Map Roboflow clash-cards v3 labels onto policy card ids."""
    if not label:
        return None
    raw = str(label).strip().lower()
    raw = re.sub(r"\.(png|jpe?g|webp)$", "", raw)
    raw = raw.replace("evoluted", "evo").replace("evolved", "evo")
    value = re.sub(r"[\s_]+", "-", raw)
    value = re.sub(r"-+", "-", value).strip("-")
    if value.endswith("-hero"):
        value = value[: -len("-hero")]
    if value.endswith("-evo"):
        value = value[: -len("-evo")] + "-evo"
    if not value or value.isdigit() or value in {"unknown", "none", "null"}:
        return None
    return value


class HandDetector:
    def __init__(self, model_path: str | Path | None = None):
        self.model_path = Path(model_path or DEFAULT_YOLO)
        self._model = None
        self._lock = threading.Lock()
        self._load_error: str | None = None
        self._warmed = False

    @property
    def ready(self) -> bool:
        self._ensure_model()
        return self._model is not None

    @property
    def status(self) -> dict[str, Any]:
        self._ensure_model()
        return {
            "ready": self._model is not None,
            "model_path": str(self.model_path),
            "error": self._load_error,
            "num_classes": len(getattr(self._model, "names", {}) or {})
            if self._model
            else 0,
            "conf": DEFAULT_CONF,
        }

    def _ensure_model(self) -> None:
        if self._model is not None or self._load_error:
            return
        with self._lock:
            if self._model is not None or self._load_error:
                return
            if not self.model_path.is_file():
                self._load_error = f"YOLO model not found: {self.model_path}"
                return
            try:
                from ultralytics import YOLO

                self._model = YOLO(str(self.model_path))
            except Exception as exc:  # noqa: BLE001 - surface to UI
                self._load_error = f"{type(exc).__name__}: {exc}"

    def warm_up(self) -> None:
        """Run a tiny batch once so the first real detect is not a cold start."""
        self._ensure_model()
        if self._model is None or self._warmed:
            return
        crops = [Image.new("RGB", (180, 240), (20, 20, 20)) for _ in range(4)]
        try:
            with self._lock:
                self._model.predict(
                    source=crops,
                    conf=DEFAULT_CONF,
                    verbose=False,
                )
            self._warmed = True
        except Exception:
            pass

    def detect(
        self,
        png_bytes: bytes,
        card_zones: dict[str, dict[str, int]],
        *,
        conf: float = DEFAULT_CONF,
    ) -> list[dict[str, Any]]:
        image = Image.open(io.BytesIO(png_bytes)).convert("RGB")
        crops: list[Image.Image | None] = []
        rects: list[dict[str, int] | None] = []
        for idx in range(4):
            key = f"card_slot_{idx}"
            rect = card_zones.get(key)
            if not rect:
                crops.append(None)
                rects.append(None)
                continue
            crop = image.crop(
                (rect["x_min"], rect["y_min"], rect["x_max"], rect["y_max"])
            )
            crops.append(crop)
            rects.append(rect)
        return self._detect_crop_list(crops, rects, conf=conf)

    def detect_crop_images(
        self,
        crops: list[Image.Image],
        *,
        conf: float = DEFAULT_CONF,
        rects: list[dict[str, int] | None] | None = None,
    ) -> list[dict[str, Any]]:
        if len(crops) != 4:
            raise ValueError("expected exactly 4 slot crops")
        rects = rects or [None] * 4
        return self._detect_crop_list(list(crops), list(rects), conf=conf)

    def _detect_crop_list(
        self,
        crops: list[Image.Image | None],
        rects: list[dict[str, int] | None],
        *,
        conf: float,
    ) -> list[dict[str, Any]]:
        self._ensure_model()
        valid_idx: list[int] = []
        valid_imgs: list[Image.Image] = []
        for i, crop in enumerate(crops):
            if crop is not None:
                valid_idx.append(i)
                valid_imgs.append(crop)

        inferred: dict[int, tuple[str | None, float, str | None]] = {}
        if valid_imgs:
            if self._model is None:
                for i in valid_idx:
                    inferred[i] = (None, 0.0, self._load_error or "model not loaded")
            else:
                try:
                    with self._lock:
                        results = self._model.predict(
                            source=valid_imgs,
                            conf=conf,
                            verbose=False,
                        )
                    for i, result in zip(valid_idx, results):
                        inferred[i] = self._best_from_result(result)
                except Exception as exc:  # noqa: BLE001
                    for i in valid_idx:
                        inferred[i] = (None, 0.0, f"{type(exc).__name__}: {exc}")

        slots: list[dict[str, Any]] = []
        for idx in range(4):
            crop = crops[idx]
            rect = rects[idx]
            if crop is None:
                slots.append(
                    {
                        "slot": idx,
                        "card_name": None,
                        "confidence": 0.0,
                        "crop_jpeg": b"",
                        "error": f"missing card_slot_{idx}",
                        "rect": rect,
                    }
                )
                continue
            name, score, error = inferred.get(idx, (None, 0.0, "no result"))
            buf = io.BytesIO()
            crop.save(buf, format="JPEG", quality=85)
            slots.append(
                {
                    "slot": idx,
                    "card_name": name,
                    "confidence": score,
                    "crop_jpeg": buf.getvalue(),
                    "error": error,
                    "rect": rect,
                }
            )
        return slots

    @staticmethod
    def _best_from_result(result: Any) -> tuple[str | None, float, str | None]:
        best_name = None
        best_conf = 0.0
        names = result.names
        if result.boxes is None:
            return None, 0.0, None
        for box in result.boxes:
            score = float(box.conf[0])
            cls = int(box.cls[0])
            candidate = normalize_detector_label(names.get(cls, str(cls)))
            if candidate and score > best_conf:
                best_conf = score
                best_name = candidate
        return best_name, best_conf, None
