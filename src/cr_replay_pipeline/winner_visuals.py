from __future__ import annotations

from pathlib import Path
from typing import Any, Sequence

import numpy as np


CONFIDENCE_THRESHOLDS = np.linspace(0.0, 0.45, 46)
CORRECTNESS_THRESHOLDS = np.linspace(0.5, 0.99, 50)


def selective_curve(
    correct: np.ndarray,
    confidence: np.ndarray,
    thresholds: Sequence[float],
) -> list[dict[str, float | int]]:
    correct = np.asarray(correct, dtype=np.int64)
    confidence = np.asarray(confidence, dtype=np.float64)
    curve: list[dict[str, float | int]] = []
    for threshold in thresholds:
        mask = confidence >= threshold
        if not mask.any():
            continue
        curve.append(
            {
                "min_confidence": float(threshold),
                "accuracy": float(correct[mask].mean()),
                "coverage": float(mask.mean()),
                "n": int(mask.sum()),
            }
        )
    return curve


def area_under_risk_coverage(correct: np.ndarray, confidence: np.ndarray) -> float:
    correct = np.asarray(correct, dtype=np.int64)
    confidence = np.asarray(confidence, dtype=np.float64)
    order = np.argsort(-confidence, kind="stable")
    cumulative_risk = np.cumsum(1 - correct[order]) / np.arange(1, len(correct) + 1)
    return float(cumulative_risk.mean())


def confidence_curve(
    labels: np.ndarray,
    probabilities: np.ndarray,
    thresholds: Sequence[float] = CONFIDENCE_THRESHOLDS,
) -> list[dict[str, float | int]]:
    labels = np.asarray(labels, dtype=np.int64)
    probabilities = np.asarray(probabilities, dtype=np.float64)
    correct = (probabilities >= 0.5) == labels
    return selective_curve(correct, np.abs(probabilities - 0.5), thresholds)


def _curve_arrays(curve: list[dict[str, Any]]) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
    return (
        np.asarray([point["min_confidence"] for point in curve]),
        np.asarray([point["accuracy"] for point in curve]),
        np.asarray([point["coverage"] for point in curve]),
    )


def save_confidence_plot(
    baseline_curve: list[dict[str, Any]],
    improved_curve: list[dict[str, Any]],
    output_path: str | Path,
) -> None:
    import matplotlib.pyplot as plt

    base_x, base_accuracy, base_coverage = _curve_arrays(baseline_curve)
    new_x, new_accuracy, new_coverage = _curve_arrays(improved_curve)
    x_min = min(float(base_x.min()), float(new_x.min()))
    x_max = max(float(base_x.max()), float(new_x.max()))
    fig, axis = plt.subplots(figsize=(12, 7), dpi=160)
    coverage_axis = axis.twinx()

    axis.plot(
        base_x,
        base_accuracy,
        color="#7a8699",
        linewidth=2.4,
        linestyle="--",
        label="Previous model accuracy",
    )
    axis.plot(
        new_x,
        new_accuracy,
        color="#1769e0",
        linewidth=3.2,
        label="Improved model accuracy",
    )
    coverage_axis.plot(
        base_x,
        base_coverage,
        color="#9a6a12",
        linewidth=2.1,
        linestyle="--",
        alpha=0.72,
        label="Previous model coverage",
    )
    coverage_axis.plot(
        new_x,
        new_coverage,
        color="#bd7c00",
        linewidth=2.5,
        alpha=0.9,
        label="Improved model coverage",
    )
    axis.axhline(0.90, color="#e63946", linestyle="--", linewidth=1.5, alpha=0.8)
    axis.axhline(0.95, color="#7c4dff", linestyle=":", linewidth=1.5, alpha=0.8)

    axis.set(
        xlim=(x_min, x_max),
        ylim=(0.70, 1.005),
        xlabel="Minimum estimated P(prediction is correct)",
        ylabel="Accuracy on covered games",
        title="Winner prediction: accuracy vs confidence (held-out test set)",
    )
    coverage_axis.set(ylim=(0.0, 1.05), ylabel="Coverage (fraction of games)")
    axis.grid(True, alpha=0.22)
    axis.tick_params(axis="y", colors="#1769e0")
    axis.yaxis.label.set_color("#1769e0")
    coverage_axis.tick_params(axis="y", colors="#bd7c00")
    coverage_axis.yaxis.label.set_color("#bd7c00")

    handles_a, labels_a = axis.get_legend_handles_labels()
    handles_b, labels_b = coverage_axis.get_legend_handles_labels()
    axis.legend(handles_a + handles_b, labels_a + labels_b, loc="lower left")

    annotation_thresholds = np.linspace(x_min, x_max, 4)
    for annotation_index, threshold in enumerate(annotation_thresholds):
        index = int(np.argmin(np.abs(new_x - threshold)))
        axis.scatter(new_x[index], new_accuracy[index], color="#1769e0", s=28, zorder=5)
        is_last = annotation_index == len(annotation_thresholds) - 1
        axis.annotate(
            f"{new_accuracy[index]:.1%} acc\n{new_coverage[index]:.0%} games",
            (new_x[index], new_accuracy[index]),
            xytext=(-8 if is_last else 7, -34 if is_last else 8),
            textcoords="offset points",
            fontsize=9,
            horizontalalignment="right" if is_last else "left",
        )

    fig.tight_layout()
    fig.savefig(output_path, bbox_inches="tight")
    plt.close(fig)


def save_training_curve_video(
    baseline_curve: list[dict[str, Any]],
    stages: list[dict[str, Any]],
    output_path: str | Path,
    fps: int = 10,
) -> None:
    import matplotlib.animation as animation
    import matplotlib.pyplot as plt

    if not stages:
        raise ValueError("At least one recorded training stage is required")

    base_x, base_accuracy, base_coverage = _curve_arrays(baseline_curve)
    stage_x, _, _ = _curve_arrays(stages[-1]["confidence_curve"])
    x_min = min(float(base_x.min()), float(stage_x.min()))
    x_max = max(float(base_x.max()), float(stage_x.max()))
    fig, axis = plt.subplots(figsize=(12, 7), dpi=120)
    coverage_axis = axis.twinx()
    axis.plot(
        base_x,
        base_accuracy,
        color="#7a8699",
        linewidth=2.2,
        linestyle="--",
        label="Previous model accuracy",
    )
    baseline_coverage_line, = coverage_axis.plot(
        base_x,
        base_coverage,
        color="#9a6a12",
        linewidth=2.1,
        linestyle="--",
        alpha=0.72,
    )
    accuracy_line, = axis.plot([], [], color="#1769e0", linewidth=3.2)
    coverage_line, = coverage_axis.plot([], [], color="#bd7c00", linewidth=2.5)
    stage_text = axis.text(
        0.02,
        0.97,
        "",
        transform=axis.transAxes,
        va="top",
        fontsize=12,
        bbox={"facecolor": "white", "alpha": 0.9, "edgecolor": "#d9dfe8"},
    )
    axis.axhline(0.90, color="#e63946", linestyle="--", linewidth=1.4, alpha=0.8)
    axis.axhline(0.95, color="#7c4dff", linestyle=":", linewidth=1.4, alpha=0.8)
    axis.set(
        xlim=(x_min, x_max),
        ylim=(0.70, 1.005),
        xlabel="Minimum estimated P(prediction is correct)",
        ylabel="Accuracy on covered games",
        title="Confidence curve during ensemble training",
    )
    coverage_axis.set(ylim=(0.0, 1.05), ylabel="Coverage (fraction of games)")
    axis.grid(True, alpha=0.22)
    axis.tick_params(axis="y", colors="#1769e0")
    axis.yaxis.label.set_color("#1769e0")
    coverage_axis.tick_params(axis="y", colors="#bd7c00")
    coverage_axis.yaxis.label.set_color("#bd7c00")
    axis.legend(
        [axis.lines[0], accuracy_line, baseline_coverage_line, coverage_line],
        [
            "Previous model accuracy",
            "Training ensemble accuracy",
            "Previous model coverage",
            "Training ensemble coverage",
        ],
        loc="lower left",
    )
    fig.tight_layout()

    hold_frames = fps * 2
    total_frames = len(stages) + hold_frames

    def update(frame: int):
        stage = stages[min(frame, len(stages) - 1)]
        x, accuracy, coverage = _curve_arrays(stage["confidence_curve"])
        accuracy_line.set_data(x, accuracy)
        coverage_line.set_data(x, coverage)
        metrics = f"accuracy {stage['accuracy']:.2%}  ·  AUC {stage['auc']:.4f}"
        if "aurc" in stage:
            metrics += f"  ·  AURC {stage['aurc']:.4f}"
        stage_text.set_text(f"{stage['trees']:,} trees\n{metrics}")
        return accuracy_line, coverage_line, stage_text

    movie = animation.FuncAnimation(
        fig,
        update,
        frames=total_frames,
        interval=1000 / fps,
        blit=False,
    )
    writer = animation.FFMpegWriter(
        fps=fps,
        codec="libx264",
        bitrate=2400,
        extra_args=["-pix_fmt", "yuv420p", "-movflags", "+faststart"],
    )
    movie.save(str(output_path), writer=writer)
    plt.close(fig)
