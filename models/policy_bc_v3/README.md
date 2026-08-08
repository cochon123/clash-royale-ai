---
license: mit
tags:
  - clash-royale
  - behavioral-cloning
  - game-ai
library_name: pytorch
---

# Clash Royale Policy BC v3

Offline behavioral-cloning policy (predecessor to [policy-bc-v4](https://huggingface.co/Cochon123/clash-royale-policy-bc-v4)).

## Files

- `best_model.pt` — PyTorch checkpoint
- `vocab.json` — card / action vocabulary
- `report.json` — training metrics and metadata
- `training_stages.json` — stage curves for reports

## Usage

See the [clash-royale-ai](https://github.com/cochon123/clash-royale-ai) repo (`cr-replay policy-*` / phone-lab).
