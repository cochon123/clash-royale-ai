#!/usr/bin/env python3
"""Download Roboflow clash-cards v3 and train the phone hand detector."""

from __future__ import annotations

import argparse
import json
import os
import shutil
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
WORKSPACE = "new-workspace-v9zo5"
PROJECT = "clash-cards-1nnw7"
VERSION = 3
DATASET_URL = f"https://universe.roboflow.com/{WORKSPACE}/{PROJECT}/dataset/{VERSION}"


def train(
    *,
    epochs: int,
    image_size: int,
    batch_size: int,
    base_model: str,
    output: Path,
) -> Path:
    api_key = os.environ.get("ROBOFLOW_API_KEY")
    if not api_key:
        raise RuntimeError(
            "ROBOFLOW_API_KEY is required to download the public dataset export. "
            f"Get an API key, then rerun this command. Dataset: {DATASET_URL}"
        )
    try:
        from roboflow import Roboflow
        from ultralytics import YOLO
    except ImportError as exc:
        raise RuntimeError(
            "Install detector training extras: pip install roboflow ultralytics"
        ) from exc

    dataset_dir = ROOT / "data" / "card_detector" / f"{PROJECT}-v{VERSION}"
    dataset = (
        Roboflow(api_key=api_key)
        .workspace(WORKSPACE)
        .project(PROJECT)
        .version(VERSION)
        .download("yolov11", location=str(dataset_dir), overwrite=True)
    )
    data_yaml = Path(dataset.location) / "data.yaml"
    if not data_yaml.is_file():
        raise FileNotFoundError(f"Roboflow export did not contain {data_yaml}")

    run_root = ROOT / "models" / "card_detector_runs"
    model = YOLO(base_model)
    model.train(
        data=str(data_yaml),
        epochs=epochs,
        imgsz=image_size,
        batch=batch_size,
        device=0,
        patience=15,
        project=str(run_root),
        name=f"{PROJECT}-v{VERSION}",
        exist_ok=True,
        plots=True,
    )
    best = run_root / f"{PROJECT}-v{VERSION}" / "weights" / "best.pt"
    if not best.is_file():
        raise FileNotFoundError(f"training finished without {best}")
    output.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(best, output)
    metadata = {
        "dataset": f"{WORKSPACE}/{PROJECT}/{VERSION}",
        "dataset_url": DATASET_URL,
        "images": 195,
        "base_model": base_model,
        "epochs_requested": epochs,
        "image_size": image_size,
        "batch_size": batch_size,
        "created_at": datetime.now(timezone.utc).isoformat(),
        "checkpoint": str(output.relative_to(ROOT)),
        "training_run": str(best.parent.parent.relative_to(ROOT)),
    }
    output.with_suffix(".json").write_text(
        json.dumps(metadata, indent=2) + "\n", encoding="utf-8"
    )
    return output


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--epochs", type=int, default=75)
    parser.add_argument("--image-size", type=int, default=640)
    parser.add_argument("--batch-size", type=int, default=16)
    parser.add_argument("--base-model", default="yolo11n.pt")
    parser.add_argument(
        "--output",
        type=Path,
        default=ROOT / "models" / "card_detector_clash_cards_v3.pt",
    )
    args = parser.parse_args()
    print(
        train(
            epochs=args.epochs,
            image_size=args.image_size,
            batch_size=args.batch_size,
            base_model=args.base_model,
            output=args.output.resolve(),
        )
    )
