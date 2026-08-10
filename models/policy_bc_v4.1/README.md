# Clash Royale policy BC v4.1

Same architecture as v4.0 (card-conditioned placement), retrained on a larger replay cut.

## Summary

| Split | Slot top-1 | Slot top-3 | Zone acc | XY MAE | Timing MAE |
|-------|------------|------------|----------|--------|------------|
| Test (finalize) | 54.5% | 93.9% | 44.3% | 5156 | 1.55s |

- **Trained on:** 28,102 usable battles (598,192 train samples), best checkpoint at epoch 17/25
- **Created:** 2026-08-08
- **Compare report:** `reports/policy_bc_v4_1_compare.html`

## Note

The training process was killed by memory pressure before finishing all 25 epochs.
Artifacts are from the best validation checkpoint (epoch 17).
