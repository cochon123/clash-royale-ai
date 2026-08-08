---
license: mit
tags:
- clash-royale
- behavior-cloning
- pytorch
- game-ai
- reinforcement-learning
library_name: pytorch
pipeline_tag: reinforcement-learning
---

# Clash Royale policy BC v4

Behavior-cloning next-action policy trained on RoyaleAPI replays. Predicts
deck-slot, placement (zone + XY), and timing from causal action history.

## Summary

| Split | Slot top-1 | Slot top-3 | Zone acc | XY MAE | Timing MAE |
|-------|------------|------------|----------|--------|------------|
| Test | 53.1% | 93.3% | 42.9% | 5287 | 1.54s |

- **Battles:** 22,554 total (train 15,788 / val 3,383 / test 3,383)
- **Train samples:** 480,032 action prefixes
- **Created:** 2026-08-08

## Architecture

- Causal transformer trunk (`d_model=160`, 2 layers)
- Card-conditioned slot head + threat features (from v3)
- Card-conditioned zone / XY placement heads
- Reaction-window upweighting for defense-style answers

## Files

| File | Description |
|------|-------------|
| `best_model.pt` | PyTorch checkpoint (weights + vocab + config) |
| `vocab.json` | Card vocabulary |
| `report.json` | Full training report |
| `training_stages.json` | Per-epoch curves |

## Usage

```bash
pip install git+https://github.com/cochon123/clash-royale-ai.git
hf download Cochon123/clash-royale-policy-bc-v4 --local-dir models/policy_bc_v4

cr-replays predict-policy data/raw/SOME_BATTLE.json --model-dir models/policy_bc_v4
cr-replays phone-lab --policy-v4 models/policy_bc_v4
```

## Limits

Offline metrics are strong on slot choice; placement XY and live tempo are
still weak relative to humans. Treat live phone play as an evaluation harness,
not a finished agent.
