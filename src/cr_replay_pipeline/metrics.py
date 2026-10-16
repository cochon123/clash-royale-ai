"""Shared evaluation metrics.

Custom rank-sum AUC implementations assigned distinct ranks to tied scores,
which inflated results (for example 0.75 instead of 0.50). Always go through
``sklearn.metrics.roc_auc_score``.
"""

from __future__ import annotations

import numpy as np


def binary_auc(labels: np.ndarray, scores: np.ndarray) -> float:
    """ROC AUC for a binary label, returning 0.5 when it is undefined."""
    labels = np.asarray(labels).astype(np.int64).reshape(-1)
    scores = np.asarray(scores, dtype=np.float64).reshape(-1)
    if labels.size == 0 or scores.size != labels.size:
        return 0.5
    if int(labels.min()) == int(labels.max()):
        return 0.5
    from sklearn.metrics import roc_auc_score

    try:
        return float(roc_auc_score(labels, scores))
    except ValueError:
        return 0.5
