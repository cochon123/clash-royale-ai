---
license: mit
tags:
- clash-royale
- scikit-learn
- game-ai
library_name: scikit-learn
pipeline_tag: tabular-classification
---

# Clash Royale realism scorer

Offline classifier that scores how likely a battle sequence is a real human
game versus a legal-but-synthetic negative (easy / medium / hard).

## Files

| File | Description |
|------|-------------|
| `realism_ensemble.pkl` | Trained ensemble artifact |
| `report.json` | Training / eval report |
| `training_stages.json` | Curve stages |

## Usage

```bash
hf download Cochon123/clash-royale-realism-scorer --local-dir models/realism_scorer
cr-replays report-realism --model-dir models/realism_scorer --output-dir reports
```

Companion code: [cochon123/clash-royale-ai](https://github.com/cochon123/clash-royale-ai).
