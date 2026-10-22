"""Repository locations shared by the Arena and Lab websites."""
from __future__ import annotations

from pathlib import Path
import os

REPO = Path(__file__).resolve().parents[2]
ENGINE = REPO / "engine"
MODELS = REPO / "models"
REPORTS = REPO / "reports"
DATA = REPO / "data"
DOCS = REPO / "docs"
LOGS = REPO / "logs"

ENGINE_RUNS = Path(os.environ.get("ARENA_RUNS_DIR", str(ENGINE / "runs")))
ENGINE_DATA = ENGINE / "data"
CARD_ART = ENGINE / "web" / "cards"
ENGINE_CHECKPOINT = ENGINE / "analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"
NATIVE_WATCH = ENGINE / "native-engine/target/release/native-watch"
MODEL_REGISTRY = MODELS / "registry.json"
PRESET_DECKS = ENGINE_DATA / "fast_league_decks.json"


def relative(path: Path | str) -> str:
    """Repository-relative display path, or the absolute path if outside it."""
    path = Path(path)
    try:
        return str(path.resolve().relative_to(REPO))
    except ValueError:
        return str(path)
