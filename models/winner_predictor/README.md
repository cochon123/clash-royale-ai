---
license: mit
tags:
- clash-royale
- tabular-classification
- scikit-learn
- game-ai
library_name: scikit-learn
pipeline_tag: tabular-classification
---

# Clash Royale full-game winner predictor

Perspective-symmetric ensemble that predicts which player wins a Clash Royale battle from replay action sequences (card plays and Hero/Champion ability activations).

## Model summary

| Metric | Validation | Test |
|--------|------------|------|
| Accuracy | 78.9% | 78.9% |
| AUC | 0.888 | 0.879 |
| Log loss | 0.430 | 0.442 |

Trained on **13,873** RoyaleAPI replays with battle-level train/val/test splits (70/15/15). Baseline (most-common legal action) test accuracy: **77.4%**.

## Architecture

- **HistGradientBoostingClassifier** + **ExtraTreesClassifier** blend (30% HGB / 70% Extra Trees, selected on validation log-loss)
- **426** tabular features from deck composition, elixir/leak tables, and action-prefix statistics
- **Symmetric inference**: every battle is scored from both player perspectives and averaged
- **Confidence head**: separate blend optimized for area under the risk-coverage curve, with isotonic calibration to estimate P(prediction is correct)

## Files

| File | Description |
|------|-------------|
| `hgb_ensemble.pkl` | Pickled artifact: both sklearn models, blend weights, confidence calibrator, card index |
| `hgb_report.json` | Full training report, splits, duration breakdown, confidence curves |
| `vocab.json` | Card name vocabulary used during feature extraction |
| `accuracy_vs_confidence.png` | Selective prediction curve on held-out test set |
| `accuracy_vs_confidence.json` | Raw curve data |
| `confidence_training_stages.json` | Per-tree-stage metrics for the training animation |
| `accuracy_vs_confidence_training.mp4` | Animation of confidence curve evolution during Extra Trees training |

## Usage

Install the companion pipeline from [cochon123/clash-royale-ai](https://github.com/cochon123/clash-royale-ai), then download this checkpoint:

```bash
pip install git+https://github.com/cochon123/clash-royale-ai.git
huggingface-cli download Cochon123/clash-royale-winner-predictor --local-dir models/winner_predictor
```

Load the artifact in Python:

```python
import pickle
from pathlib import Path

with Path("models/winner_predictor/hgb_ensemble.pkl").open("rb") as f:
    artifact = pickle.load(f)

hgb = artifact["models"]["hist_gradient_boosting"]
trees = artifact["models"]["extra_trees"]
hgb_weight = artifact["hgb_weight"]
card_index = artifact["card_index"]
```

Retrain or reproduce from raw replays:

```bash
cr-replays train-winner-hgb --input data/raw --trees 100
```

## Training data

Features are extracted from RoyaleAPI replay HTML payloads. The model does **not** see live arena state (unit positions, HP, projectiles)—only the ordered sequence of card plays and ability activations plus deck metadata. See the [investigation report](https://github.com/cochon123/clash-royale-ai/blob/main/docs/investigation.md) for labeling details around Hero/Champion abilities.

## Limitations

- Replay-only features; not suitable as a standalone real-time bot without synchronized game-state input
- Trained on a specific meta window; performance may drift with balance patches and new cards
- Confidence scores are calibrated on the validation split and should be treated as estimates, not guarantees
