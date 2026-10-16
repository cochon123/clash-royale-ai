"""Bounded, validation-only screening; no live games or automatic scaling.

python -m cr_replay_pipeline.controlled_experiment --help
Every variant shares the same immutable data/config contract. Suite wall-time
is a conservative upper bound on GPU time and is persisted across invocations.
"""

import argparse
from datetime import datetime, timezone
import fcntl
import html
import json
import math
from pathlib import Path
import signal
import sys
import time
import traceback

import numpy as np
import torch
from torch.utils.data import DataLoader
from .artifact_cache import (
    source_fingerprint,
    sha256_file,
    sha256_json,
    DATASET_PREPROCESS_VERSION,
)
from .causal_observation import OBSERVATION_VERSION
from .controlled_dataset import ActionExamples, ClockExamples, collate_clock
from .controlled_policy import ControlledPolicy, VARIANTS
from .event_clock import competing_risk_nll
from .feature_schema import GLOBAL_INDEX
from .policy_dataset import (
    baseline_cycle_slot,
    baseline_frequency_slot,
    collate_policy_batch,
)
from .policy_manifest import load_manifest, battles_from_manifest
from .policy_model import TILE_COLS, TILE_ROWS
from .winner_dataset import collect_battles, build_vocab, load_card_costs

ROW_COLUMNS = (
    "card",
    "cell",
    "joint",
    "joint_nll",
    "timing_mae",
    "card_top3",
    "card_nll",
    "cell_teacher",
    "now_s",
)
RATE_FLOOR = 1e-3
ACTION_REGRESSION = 0.005
MEMORY_GAIN = 0.01


def atomic_json(path, value):
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(json.dumps(value, indent=2, allow_nan=False) + "\n")
    tmp.replace(path)


class DurableLog:
    """Keep writing after the Cursor/tee pipe dies. Training must not depend on stdout."""

    def __init__(self, *streams):
        self.streams = [stream for stream in streams if stream is not None]

    def write(self, data):
        surviving = []
        for stream in self.streams:
            try:
                stream.write(data)
                stream.flush()
                surviving.append(stream)
            except BrokenPipeError:
                continue
        self.streams = surviving
        return len(data)

    def flush(self):
        surviving = []
        for stream in self.streams:
            try:
                stream.flush()
                surviving.append(stream)
            except BrokenPipeError:
                continue
        self.streams = surviving

    def isatty(self):
        return False


def attach_suite_log(output: Path):
    signal.signal(signal.SIGPIPE, signal.SIG_IGN)
    log_file = (Path(output) / "trainer.log").open("a", encoding="utf-8")
    sys.stdout = DurableLog(sys.stdout, log_file)
    sys.stderr = DurableLog(sys.stderr, log_file)
    return log_file


def write_crash(output: Path, text: str) -> None:
    path = Path(output) / "crash.txt"
    path.write_text(text)
    print(text, file=sys.stderr, flush=True)


class Budget:
    def __init__(self, path, limit):
        self.path = Path(path)
        self.limit = float(limit)
        self.started = time.monotonic()
        if not 0 < self.limit <= 10800:
            raise ValueError("Suite cap must be in (0,10800] seconds")
        old = json.loads(self.path.read_text()) if self.path.exists() else {}
        if old and old["limit_seconds"] != self.limit:
            raise ValueError("Cannot silently change suite budget")
        self.used = old.get("used_seconds", 0.0)
        self.persist()

    @property
    def remaining(self):
        return max(0.0, self.limit - self.used - (time.monotonic() - self.started))

    def persist(self):
        atomic_json(
            self.path,
            dict(
                limit_seconds=self.limit,
                used_seconds=self.used + time.monotonic() - self.started,
            ),
        )


def move(batch, device):
    return tuple(t.to(device) for t in batch)


def action_loss(out, batch):
    # Same proper teacher-forced joint objective for every variant.
    slot, xy, dt, weight = batch[7], batch[10], batch[11], batch[13]
    tile = (xy[:, 1] * TILE_ROWS).long().clamp(0, TILE_ROWS - 1) * TILE_COLS + (
        xy[:, 0] * TILE_COLS
    ).long().clamp(0, TILE_COLS - 1)
    f = torch.nn.functional
    per = (
        f.cross_entropy(out["slot_logits"], slot, reduction="none")
        + f.cross_entropy(out["tile_logits"], tile, reduction="none")
        + 0.25 * f.smooth_l1_loss(out["timing"], dt, reduction="none")
    )
    return (per * weight).sum() / weight.clamp_min(1e-6).sum()


def history_at(report, epoch):
    return next((row for row in report.get("history") or [] if row["epoch"] == epoch), None)


def shared_epoch(*reports):
    sets = [{row["epoch"] for row in report.get("history") or []} for report in reports]
    if not sets or any(not s for s in sets):
        return None
    common = set.intersection(*sets)
    return max(common) if common else None


def decide_clock_gate(baseline, clock):
    """Equal-epoch clock continuation rule from the v8 review."""
    epoch = shared_epoch(baseline, clock)
    reasons = []
    if epoch is None:
        return {
            "pass": False,
            "epoch": None,
            "reasons": ["No shared completed epoch for an equal-exposure comparison"],
        }
    base = history_at(baseline, epoch)
    cand = history_at(clock, epoch)
    own = float(cand.get("mean_own_rate") or 0)
    opp = float(cand.get("mean_opponent_rate") or 0)
    empirical = float((clock.get("empirical_clock_validation") or {}).get("clock_nll") or 1e9)
    clock_nll = cand.get("clock_nll")
    card_delta = cand["card"] - base["card"]
    joint_delta = cand["joint"] - base["joint"]
    timing_delta = cand["timing_mae"] - base["timing_mae"]
    rates_ok = own >= RATE_FLOOR and opp >= RATE_FLOOR
    nll_ok = clock_nll is not None and clock_nll < empirical
    no_regression = card_delta >= -ACTION_REGRESSION and joint_delta >= -ACTION_REGRESSION
    if not rates_ok:
        reasons.append(f"Actor rates not both above {RATE_FLOOR}: own={own:.6f} opp={opp:.6f}")
    if not nll_ok:
        reasons.append(f"Clock NLL {clock_nll} is not better than empirical {empirical}")
    if not no_regression:
        reasons.append(
            f"Action regression at epoch {epoch}: card {card_delta:+.4f}, joint {joint_delta:+.4f}"
        )
    if not reasons:
        reasons.append("Rates sensible, clock NLL improved, no material action regression")
        if timing_delta <= -0.10:
            reasons.append(f"Timing MAE also improved by {-timing_delta:.3f}s")
    return {
        "pass": rates_ok and nll_ok and no_regression,
        "epoch": epoch,
        "comparison": "equal_completed_epoch",
        "card_delta": card_delta,
        "joint_delta": joint_delta,
        "timing_mae_delta": timing_delta,
        "mean_own_rate": own,
        "mean_opponent_rate": opp,
        "clock_nll": clock_nll,
        "empirical_clock_nll": empirical,
        "baseline_epoch": base,
        "clock_epoch": cand,
        "reasons": reasons,
        "selected_checkpoint_warning": (
            "Do not compare baseline weights_epoch="
            f"{baseline.get('weights_epoch')} with clock weights_epoch="
            f"{clock.get('weights_epoch')}; those are unequal exposures."
        ),
    }


def decide_memory_gate(baseline, memory):
    """Equal-epoch memory continuation rule from the v8 review."""
    epoch = shared_epoch(baseline, memory)
    if epoch is None:
        return {
            "pass": False,
            "epoch": None,
            "reasons": ["No shared completed epoch for an equal-exposure comparison"],
        }
    base = history_at(baseline, epoch)
    cand = history_at(memory, epoch)
    card_delta = cand["card"] - base["card"]
    joint_delta = cand["joint"] - base["joint"]
    passed = card_delta >= MEMORY_GAIN or (
        joint_delta >= MEMORY_GAIN and card_delta >= -ACTION_REGRESSION
    )
    reasons = [
        f"card {card_delta:+.4f} (need +{MEMORY_GAIN:.2f} or joint +{MEMORY_GAIN:.2f} with card >= -{ACTION_REGRESSION:.3f})",
        f"joint {joint_delta:+.4f}",
    ]
    if passed:
        reasons.append("Action gate passed; shuffled/frozen belong in a new budgeted suite")
    else:
        reasons.append("Aligned memory did not earn controls or extra seeds")
    return {
        "pass": passed,
        "epoch": epoch,
        "comparison": "equal_completed_epoch",
        "card_delta": card_delta,
        "joint_delta": joint_delta,
        "baseline_epoch": base,
        "memory_epoch": cand,
        "reasons": reasons,
    }


def compare_run_dirs(left, right):
    """Paired battle bootstrap plus an equal-epoch history comparison."""
    left = Path(left)
    right = Path(right)
    a = json.loads((left / "report.json").read_text())
    b = json.loads((right / "report.json").read_text())
    if a.get("contract") != b.get("contract"):
        raise ValueError("Different data/feature contracts")
    epoch = shared_epoch(a, b)
    result = {
        "baseline": str(left),
        "candidate": str(right),
        "equal_epoch": epoch,
        "selected_epochs": {
            "baseline": a.get("weights_epoch"),
            "candidate": b.get("weights_epoch"),
        },
        "wall_seconds": {
            "baseline": a.get("elapsed_seconds"),
            "candidate": b.get("elapsed_seconds"),
        },
        "limitations": (
            "equal_epoch uses the last shared completed epoch. selected_checkpoint "
            "may mix unequal presentations. Intervals are battle resamples, not seeds."
        ),
    }
    if epoch is not None:
        base = history_at(a, epoch)
        cand = history_at(b, epoch)
        result["equal_epoch_delta"] = {
            key: cand[key] - base[key]
            for key in ("card", "cell", "joint", "joint_nll", "timing_mae")
            if key in base and key in cand
        }
        result["equal_epoch_baseline"] = base
        result["equal_epoch_candidate"] = cand
    pair_left = left / (f"validation_e{epoch}.npz" if epoch else "validation_rows.npz")
    pair_right = right / (f"validation_e{epoch}.npz" if epoch else "validation_rows.npz")
    if not pair_left.exists() or not pair_right.exists():
        pair_left = left / "validation_rows.npz"
        pair_right = right / "validation_rows.npz"
    if pair_left.exists() and pair_right.exists():
        x = np.load(pair_left)
        y = np.load(pair_right)
        if not np.array_equal(x["groups"], y["groups"]):
            raise ValueError("Different indexed targets")
        groups = x["groups"]
        delta = y["rows"][:, :5] - x["rows"][:, :5]
        n = int(groups.max()) + 1
        counts = np.bincount(groups, minlength=n)
        sums = np.stack(
            [
                np.bincount(groups, weights=delta[:, j], minlength=n)
                for j in range(5)
            ],
            1,
        )
        draws = np.random.default_rng(20260906).integers(0, n, (2000, n))
        boots = sums[draws].sum(1) / counts[draws].sum(1)[:, None]
        ci = np.quantile(boots, [0.025, 0.975], axis=0)
        source = "equal_epoch_rows" if f"validation_e{epoch}.npz" in pair_left.name else "selected_checkpoint_rows"
        result[source] = {
            "battles": n,
            "samples": len(groups),
            "delta": {
                k: {"mean": float(delta[:, j].mean()), "ci95": ci[:, j].tolist()}
                for j, k in enumerate(ROW_COLUMNS[:5])
            },
        }
    return result


@torch.inference_mode()
def evaluate(model, loader, clock_loader, device, deadline=None):
    model.eval()
    rows = []
    clocks = []
    latency_ms = None
    for batch in loader:
        if deadline and time.monotonic() >= deadline:
            raise TimeoutError("Evaluation budget exhausted")
        batch = move(batch, device)
        if latency_ms is None:
            if device.type == "cuda":
                torch.cuda.synchronize()
            started = time.perf_counter()
            _ = model(batch)
            if device.type == "cuda":
                torch.cuda.synchronize()
            latency_ms = 1000.0 * (time.perf_counter() - started) / max(len(batch[7]), 1)
        out = model(batch, teacher=True)
        greedy = model(batch)
        slot, xy = batch[7], batch[10]
        tile = (xy[:, 1] * TILE_ROWS).long().clamp(0, TILE_ROWS - 1) * TILE_COLS + (
            xy[:, 0] * TILE_COLS
        ).long().clamp(0, TILE_COLS - 1)
        card = greedy["slot_logits"].argmax(-1) == slot
        cell = greedy["tile_logits"].argmax(-1) == tile
        cell_teacher = out["tile_logits"].argmax(-1) == tile
        top3 = greedy["slot_logits"].topk(3, dim=-1).indices
        card_top3 = (top3 == slot.unsqueeze(-1)).any(-1)
        card_nll = torch.nn.functional.cross_entropy(
            out["slot_logits"], slot, reduction="none"
        )
        tile_nll = torch.nn.functional.cross_entropy(
            out["tile_logits"], tile, reduction="none"
        )
        now_s = batch[4][:, GLOBAL_INDEX["now"]] * 300.0
        rows.append(
            torch.stack(
                [
                    card.float(),
                    cell.float(),
                    (card & cell).float(),
                    card_nll + tile_nll,
                    (greedy["timing"].expm1() - batch[11].expm1()).abs(),
                    card_top3.float(),
                    card_nll,
                    cell_teacher.float(),
                    now_s,
                ],
                1,
            )
            .cpu()
            .numpy()
        )
    if not rows:
        raise ValueError("No validation actions")
    if model.clock_head is not None:
        for batch, side, observed in clock_loader:
            if deadline and time.monotonic() >= deadline:
                raise TimeoutError("Clock evaluation budget exhausted")
            batch = move(batch, device)
            side = side.to(device)
            observed = observed.to(device)
            rates = model(batch)["clock_rates"]
            delay = batch[11].expm1()
            nll = competing_risk_nll(rates, delay, side, observed)
            pred = rates.argmax(-1)
            own = rates[:, 0] / rates.sum(-1)
            brier = (own - (side == 0).float()).square()
            extra = []
            for horizon in (2.0, 5.0, 10.0):
                probability = 1 - torch.exp(-rates.sum(-1) * horizon)
                target = (delay <= horizon) & observed
                valid = (delay >= horizon) | observed
                extra.extend(
                    [(probability - target.float()).square() * valid, valid.float()]
                )
            clocks.append(
                torch.stack(
                    [
                        nll,
                        (pred == side).float(),
                        brier,
                        (observed & (side >= 0)).float(),
                        rates[:, 0],
                        rates[:, 1],
                        *extra,
                        (0.69314718056 / rates.sum(-1) - delay).abs() * observed,
                        observed.float(),
                    ],
                    1,
                )
                .cpu()
                .numpy()
            )
    array = np.concatenate(rows)
    metrics = dict(zip(ROW_COLUMNS, map(float, array.mean(0))))
    for name, mask in (
        ("opening", array[:, 8] < 60),
        ("mid", (array[:, 8] >= 60) & (array[:, 8] < 180)),
        ("late", array[:, 8] >= 180),
    ):
        if mask.any():
            metrics[f"{name}_card"] = float(array[mask, 0].mean())
            metrics[f"{name}_joint"] = float(array[mask, 2].mean())
            metrics[f"{name}_n"] = int(mask.sum())
    if latency_ms is not None:
        metrics["inference_ms_per_example"] = float(latency_ms)
    if clocks:
        c = np.concatenate(clocks)
        valid = c[:, 3] > 0
        metrics.update(
            clock_nll=float(c[:, 0].mean()),
            actor_accuracy=float(c[valid, 1].mean()) if valid.any() else None,
            actor_brier=float(c[valid, 2].mean()) if valid.any() else None,
            mean_own_rate=float(c[:, 4].mean()),
            mean_opponent_rate=float(c[:, 5].mean()),
        )
        for j, horizon in enumerate((2, 5, 10)):
            metrics[f"survival_brier_{horizon}s"] = float(
                c[:, 6 + 2 * j].sum() / max(c[:, 7 + 2 * j].sum(), 1)
            )
        metrics["clock_median_mae"] = float(c[:, 12].sum() / max(c[:, 13].sum(), 1))
    return metrics, array


def write_report(output, report):
    epoch = report.get("weights_epoch")
    report["selected_validation"] = next(
        (row for row in report["history"] if row["epoch"] == epoch), None
    )
    atomic_json(output / "report.json", report)
    history = report["history"]
    points = []
    if history:
        vals = [r["joint_nll"] for r in history]
        lo = min(vals) - 0.01
        hi = max(vals) + 0.01
        points = [
            f"{40 + i * 720 / max(1, len(vals) - 1):.1f},{220 - (v - lo) * 180 / (hi - lo):.1f}"
            for i, v in enumerate(vals)
        ]
    details = html.escape(
        json.dumps({k: v for k, v in report.items() if k != "history"}, indent=2)
    )
    table = "".join(
        "<tr>"
        + "".join(
            f"<td>{html.escape(str(r.get(k, '')))}</td>"
            for k in ("epoch", "train_loss", "card", "joint", "joint_nll", "timing_mae")
        )
        + "</tr>"
        for r in history
    )
    (output / "report.html").write_text(
        '''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width"><title>Controlled policy experiment</title><style>body{background:#121820;color:#e4f1eb;font:17px/1.6 system-ui;max-width:1000px;margin:40px auto;padding:20px}a{color:#8cdbb6}svg{width:100%}td,th{padding:10px;border-bottom:1px solid #456}table{display:block;overflow:auto}pre{white-space:pre-wrap;overflow-wrap:anywhere}polyline{stroke-dasharray:2000;stroke-dashoffset:2000;animation:draw 2s forwards}@keyframes draw{to{stroke-dashoffset:0}}@media(prefers-reduced-motion:reduce){polyline{animation:none;stroke-dashoffset:0}}</style><h1>Controlled policy experiment</h1><p>Validation screening only. Lower joint NLL is better. No live-win claim; no automatic corpus scaling.</p><svg viewBox="0 0 800 250" role="img" aria-label="Validation joint action negative log likelihood by completed epoch"><polyline fill="none" stroke="#8cdbb6" stroke-width="3" points="'''
        + " ".join(points)
        + """"/></svg><table><thead><tr><th>Epoch</th><th>Train loss</th><th>Card</th><th>Joint</th><th>Joint NLL</th><th>Timing MAE</th></tr></thead><tbody>"""
        + table
        + "</tbody></table><h2>Provenance, compute and outcome</h2><pre>"
        + details
        + "</pre></html>"
    )


def run_variant(args, variant, train, val, vocab, costs, contract, budget):
    output = Path(args.output) / f"{variant}_seed{args.seed}"
    if output.exists():
        raise ValueError(
            f"Run already exists: {output}; use a new suite, never overwrite evidence"
        )
    output.mkdir()
    torch.manual_seed(args.seed)
    np.random.seed(args.seed)
    device = torch.device(
        args.device or ("cuda" if torch.cuda.is_available() else "cpu")
    )
    model = ControlledPolicy(vocab.vocab_size, variant, args.d_model).to(device)
    optimizer = torch.optim.AdamW(model.parameters(), lr=2e-4, weight_decay=0.01)
    reaction_repeats = int(getattr(args, "reaction_repeats", 2))
    reaction_weight = float(getattr(args, "reaction_weight", 3.0))
    train_ds = ActionExamples(
        train,
        vocab,
        costs,
        args.samples_per_battle,
        reaction_repeats=reaction_repeats,
        reaction_weight=reaction_weight,
    )
    val_ds = ActionExamples(
        val,
        vocab,
        costs,
        args.samples_per_battle,
        reaction_repeats=1,
        reaction_weight=1.0,
    )
    clock_ds = ClockExamples(train, vocab, costs)
    val_clock = ClockExamples(val, vocab, costs)
    if not len(train_ds) or not len(clock_ds):
        raise ValueError("Empty training population")
    vl = DataLoader(val_ds, batch_size=args.batch_size, collate_fn=collate_policy_batch)
    vc = DataLoader(val_clock, batch_size=args.batch_size, collate_fn=collate_clock)
    # A simple empirical constant-rate reference uses the exact clock population.
    exposure = sum(row[3] for row in clock_ds.index)
    empirical = [
        sum(
            (1.0 if row[4] == side else 0.5 if row[4] < 0 else 0.0) * row[5]
            for row in clock_ds.index
        )
        / exposure
        for side in (0, 1)
    ]
    reference_delay = torch.tensor([r[3] for r in val_clock.index])
    reference_side = torch.tensor([r[4] for r in val_clock.index])
    reference_observed = torch.tensor([r[5] for r in val_clock.index])
    reference_rates = torch.tensor(empirical).clamp_min(1e-6).expand(len(val_clock), -1)
    reference_metrics = {
        "clock_nll": float(
            competing_risk_nll(
                reference_rates, reference_delay, reference_side, reference_observed
            ).mean()
        ),
        "clock_median_mae": float(
            (0.69314718056 / reference_rates.sum(-1) - reference_delay)
            .abs()[reference_observed]
            .mean()
        ),
    }
    started = time.monotonic()
    per_variant = float(getattr(args, "run_seconds", 0) or 0)
    cap = budget.remaining if per_variant <= 0 else min(per_variant, budget.remaining)
    deadline = started + cap
    history = []
    best = float("inf")
    bad = 0
    done = 0
    status = "completed"
    best_epoch = None
    total = args.epochs * len(train_ds)
    suite_index = int(getattr(args, "_suite_variant_index", 0))
    suite_count = int(getattr(args, "_suite_variant_count", 1))
    suite_progress = Path(args.output) / "progress.jsonl"
    report = {
        "variant": variant,
        "seed": args.seed,
        "created_at": datetime.now(timezone.utc).isoformat(),
        "contract": contract,
        "parameters": sum(p.numel() for p in model.parameters()),
        "device": str(device),
        "train_battles": len(train),
        "validation_battles": len(val),
        "unique_actions": len(train_ds.unique_index),
        "train_presentations": len(train_ds),
        "validation_actions": len(val_ds.unique_index),
        "clock_intervals": len(clock_ds),
        "frequency_baseline": baseline_frequency_slot(val, val_ds.unique_index),
        "cycle_baseline": baseline_cycle_slot(val, val_ds.unique_index),
        "empirical_clock_rates": empirical,
        "empirical_clock_validation": reference_metrics,
        "history": history,
        "config": vars(args),
        "model_config": {
            "variant": variant,
            "d_model": args.d_model,
            "vocab_size": vocab.vocab_size,
        },
    }
    log = output / "progress.jsonl"
    try:
        for epoch in range(1, args.epochs + 1):
            model.train()
            losses = []
            generator = torch.Generator().manual_seed(args.seed + epoch)
            tl = DataLoader(
                train_ds,
                batch_size=args.batch_size,
                shuffle=True,
                generator=generator,
                collate_fn=collate_policy_batch,
            )
            cl = DataLoader(
                clock_ds,
                batch_size=args.batch_size,
                shuffle=True,
                generator=torch.Generator().manual_seed(args.seed + epoch + 10000),
                collate_fn=collate_clock,
            )
            ci = iter(cl)
            for bi, batch in enumerate(tl):
                if time.monotonic() >= deadline:
                    raise TimeoutError("Training budget exhausted")
                batch = move(batch, device)
                optimizer.zero_grad(set_to_none=True)
                action_out = model(batch, teacher=True)
                loss = action_loss(action_out, batch)
                if model.clock_head is not None:
                    try:
                        cb, side, observed = next(ci)
                    except StopIteration:
                        ci = iter(cl)
                        cb, side, observed = next(ci)
                    cb = move(cb, device)
                    rates = model(cb)["clock_rates"]
                    loss = (
                        loss
                        + 0.1
                        * competing_risk_nll(
                            rates, cb[11].expm1(), side.to(device), observed.to(device)
                        ).mean()
                    )
                if not torch.isfinite(loss):
                    raise FloatingPointError("Non-finite loss; run invalid")
                loss.backward()
                torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
                optimizer.step()
                losses.append(float(loss.detach()))
                done += len(batch[7])
                if bi % 10 == 0:
                    elapsed = time.monotonic() - started
                    suite_elapsed = time.monotonic() - float(
                        getattr(args, "_suite_started", started)
                    )
                    suite_done = suite_index * total + done
                    suite_total = suite_count * total
                    eta = min(
                        max(0, deadline - time.monotonic()),
                        suite_elapsed / max(suite_done, 1) * (suite_total - suite_done),
                    )
                    row = dict(
                        timestamp=datetime.now(timezone.utc).isoformat(),
                        variant=variant,
                        phase=variant,
                        model_name=f"controlled · {variant}",
                        epoch=suite_index * args.epochs + epoch,
                        epochs_total=suite_count * args.epochs,
                        batch=bi + 1,
                        batches_total=len(tl),
                        work_done=done,
                        work_total=total,
                        samples_done=suite_index * total + done,
                        samples_total=suite_count * total,
                        progress_percent=100 * suite_done / suite_total,
                        elapsed_seconds=suite_elapsed,
                        eta_seconds=eta,
                        loss=losses[-1],
                        parameters=report["parameters"],
                        device=str(device),
                        batch_size=args.batch_size,
                        lr=optimizer.param_groups[0]["lr"],
                        card=float(
                            (action_out["slot_logits"].argmax(-1) == batch[7])
                            .float()
                            .mean()
                        ),
                    )
                    target_xy = batch[10]
                    target_tile = (
                        (target_xy[:, 1] * TILE_ROWS)
                        .long()
                        .clamp(0, TILE_ROWS - 1)
                        * TILE_COLS
                        + (target_xy[:, 0] * TILE_COLS)
                        .long()
                        .clamp(0, TILE_COLS - 1)
                    )
                    row["tile_exact"] = float(
                        (action_out["tile_logits"].argmax(-1) == target_tile)
                        .float()
                        .mean()
                    )
                    encoded = json.dumps(row) + "\n"
                    for progress_path in (log, suite_progress):
                        with progress_path.open("a") as f:
                            f.write(encoded)
                    try:
                        print(
                            f"{variant:12} epoch={epoch:2} work={done}/{total} ETA={eta:.0f}s loss={losses[-1]:.4f}",
                            flush=True,
                        )
                    except BrokenPipeError:
                        pass
                    budget.persist()
            metrics, rows = evaluate(model, vl, vc, device, deadline)
            if not all(v is None or math.isfinite(v) for v in metrics.values()):
                raise FloatingPointError("Invalid validation metrics")
            history.append(
                dict(epoch=epoch, train_loss=float(np.mean(losses)), **metrics)
            )
            validation_row = {
                "timestamp": datetime.now(timezone.utc).isoformat(),
                "variant": variant,
                "phase": "validation",
                "model_name": f"controlled · {variant}",
                "epoch": suite_index * args.epochs + epoch,
                "epochs_total": suite_count * args.epochs,
                "batch": len(tl),
                "batches_total": len(tl),
                "samples_done": suite_index * total + done,
                "samples_total": suite_count * total,
                "progress_percent": 100
                * (suite_index * total + done)
                / (suite_count * total),
                "eta_seconds": max(0.0, budget.remaining),
                "card": metrics["card"],
                "tile_exact": metrics["cell"],
                "wait": 1.0
                - metrics["timing_mae"] / max(metrics["timing_mae"] + 1.0, 1.0),
                "loss": metrics["joint_nll"],
            }
            with suite_progress.open("a") as f:
                f.write(json.dumps(validation_row) + "\n")
            payload = dict(
                model_state=model.state_dict(),
                optimizer_state=optimizer.state_dict(),
                vocab=vocab.to_dict(),
                config=report["model_config"],
                contract=contract,
                weights_epoch=epoch,
                last_completed_epoch=epoch,
            )
            temp = output / "last.tmp"
            torch.save(payload, temp)
            temp.replace(output / "last.pt")
            groups = np.array([row[0] for row in val_ds.unique_index])
            epoch_rows = dict(rows=rows, groups=groups, columns=np.array(ROW_COLUMNS))
            np.savez_compressed(output / f"validation_e{epoch}.npz", **epoch_rows)
            if metrics["joint_nll"] < best - 1e-4:
                best = metrics["joint_nll"]
                bad = 0
                best_epoch = epoch
                torch.save(payload, output / "best.tmp")
                (output / "best.tmp").replace(output / "best.pt")
                np.savez_compressed(output / "validation_rows.npz", **epoch_rows)
            else:
                bad += 1
            report.update(
                status="running",
                weights_epoch=best_epoch,
                elapsed_seconds=time.monotonic() - started,
            )
            write_report(output, report)
            if bad >= args.patience:
                status = "early_stopped"
                break
    except TimeoutError:
        status = "budget_stopped"
    except BaseException as exc:
        crash = traceback.format_exc()
        report.update(
            status="failed",
            error=repr(exc),
            elapsed_seconds=time.monotonic() - started,
        )
        write_report(output, report)
        write_crash(output, crash)
        raise
    finally:
        budget.persist()
    report.update(
        status=status,
        weights_epoch=best_epoch,
        last_completed_epoch=history[-1]["epoch"] if history else 0,
        elapsed_seconds=time.monotonic() - started,
        samples_presented=done,
    )
    write_report(output, report)
    return report


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--input", default="data/raw")
    p.add_argument("--metadata", default="data/metadata/battles.jsonl")
    p.add_argument("--costs", default="data/card_costs.json")
    p.add_argument("--manifest", default="data/splits/policy_v8_pilot_5000_seed42.json")
    p.add_argument(
        "--final-holdout", default="data/splits/controlled_final_holdout.json"
    )
    p.add_argument("--output", default="models/controlled_screening_gated")
    p.add_argument("--variant", choices=(*VARIANTS, "screen"), default="baseline")
    p.add_argument("--epochs", type=int, default=12)
    p.add_argument("--patience", type=int, default=3)
    p.add_argument("--seed", type=int, default=42)
    p.add_argument("--batch-size", type=int, default=128)
    p.add_argument("--d-model", type=int, default=160)
    p.add_argument("--samples-per-battle", type=int, default=40)
    p.add_argument(
        "--run-seconds",
        type=float,
        default=0,
        help="Optional per-variant wall cap. 0 uses remaining suite budget only so variants can share epoch count.",
    )
    p.add_argument("--suite-seconds", type=float, default=10800)
    p.add_argument("--reaction-repeats", type=int, default=2)
    p.add_argument("--reaction-weight", type=float, default=3.0)
    p.add_argument("--device")
    p.add_argument("--prepare", action="store_true")
    args = p.parse_args()
    if min(args.epochs, args.batch_size, args.samples_per_battle, args.patience) <= 0:
        p.error("Positive limits required")
    if args.run_seconds < 0 or args.reaction_repeats < 1 or args.reaction_weight <= 0:
        p.error("Invalid reaction or run-seconds limits")
    torch.set_num_threads(2)
    output = Path(args.output)
    output.mkdir(parents=True, exist_ok=True)
    attach_suite_log(output)
    # Exclusive lock prevents two processes spending the same remaining budget.
    with (output / "suite.lock").open("w") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        budget = Budget(output / "budget.json", args.suite_seconds)
        try:
            contract = {
                "observation": OBSERVATION_VERSION,
                "preprocess": DATASET_PREPROCESS_VERSION,
                "source": source_fingerprint(args.input),
                "metadata": sha256_file(args.metadata) if args.metadata else None,
                "costs": sha256_file(args.costs),
                "manifest": sha256_file(args.manifest),
                "final_holdout": sha256_file(args.final_holdout),
                "samples_per_battle": args.samples_per_battle,
                "d_model": args.d_model,
                "batch_size": args.batch_size,
                "reaction_repeats_train": args.reaction_repeats,
                "reaction_weight_train": args.reaction_weight,
                "objective": "v441-shaped-gru;joint-card-cell-nll+0.25-timing;clock=0.1;reaction-train-only;clock-unbiased",
                "source_code": sha256_json(
                    {f.name: sha256_file(f) for f in Path(__file__).parent.glob("*.py")}
                ),
            }
            contract["metadata_records"] = (
                sum(
                    bool(line.strip())
                    for line in Path(args.metadata).read_text().splitlines()
                )
                if args.metadata
                else 0
            )
            cp = output / "contract.json"
            if cp.exists() and json.loads(cp.read_text()) != contract:
                raise ValueError(
                    "Suite contract changed; choose a new output directory"
                )
            atomic_json(cp, contract)
            if not (output / "source.zip").exists():
                import zipfile

                with zipfile.ZipFile(
                    output / "source.zip", "w", zipfile.ZIP_DEFLATED
                ) as archive:
                    for source in Path(__file__).parent.glob("*.py"):
                        archive.write(source, arcname=source.name)
                atomic_json(
                    output / "environment.json",
                    {
                        "torch": torch.__version__,
                        "numpy": np.__version__,
                        "cuda": torch.version.cuda,
                        "gpu": torch.cuda.get_device_name(0)
                        if torch.cuda.is_available()
                        else None,
                    },
                )
            manifest = load_manifest(args.manifest)
            locked = load_manifest(args.final_holdout)
            if set(locked["test_ids"]) & set(
                manifest["train_ids"]
                + manifest["validation_ids"]
                + manifest["test_ids"]
            ):
                raise ValueError("Screening manifest touches the locked final holdout")
            atomic_json(output / "split_manifest.json", manifest)
            atomic_json(
                output / "plan.json",
                {
                    "variants": list(VARIANTS),
                    "initial_screen": ["baseline", "clock"],
                    "memory_requires": "clock_gate at equal completed epoch",
                    "controls_require": "memory_gate, then a newly budgeted suite",
                    "suite_seconds": args.suite_seconds,
                    "run_seconds": args.run_seconds,
                    "epochs": args.epochs,
                    "d_model": args.d_model,
                    "dropout": 0.2,
                    "reaction_repeats_train": args.reaction_repeats,
                    "equal_exposure": "compare at last shared completed epoch; selected checkpoints are labeled separately",
                    "test_access": "none; validation-only screening",
                    "promotion": "manual evidence review, paired battle CIs, then two more seeds",
                    "clock_gate": "both actor rates >= 1e-3; clock NLL better than empirical; card/joint regression <= 0.5 pp",
                    "memory_gate": "+1 pp card or joint, card regression <=0.5 pp; aligned must beat shuffled and frozen time in a later suite",
                    "board_oracle": "separate annotated-game experiment; never inferred from deployment history",
                },
            )
            if args.prepare:
                print(f"Prepared {output}/plan.json; no training launched")
                return
            battles = collect_battles(
                args.input,
                metadata_path=args.metadata,
                require_decisive_result=False,
                unresolved_abilities="preserve_event",
                battle_ids=set(
                    manifest["train_ids"]
                    + manifest["validation_ids"]
                    + manifest["test_ids"]
                ),
            )
            train, val, _ = battles_from_manifest(battles, manifest)
            vocab = build_vocab(train)
            costs = load_card_costs(args.costs)
            queue = (
                ["baseline", "clock"]
                if args.variant == "screen"
                else [args.variant]
            )
            results = {}
            args._suite_started = time.monotonic()
            args._suite_variant_count = 3 if args.variant == "screen" else 1
            index = 0
            while queue:
                if budget.remaining <= 0:
                    print("Suite budget exhausted; stopping", flush=True)
                    break
                variant = queue.pop(0)
                args._suite_variant_index = index
                results[variant] = run_variant(
                    args, variant, train, val, vocab, costs, contract, budget
                )
                index += 1
                if args.variant != "screen":
                    continue
                if variant == "clock" and "baseline" in results:
                    decision = decide_clock_gate(results["baseline"], results["clock"])
                    atomic_json(output / "clock_gate.json", decision)
                    try:
                        comparison = compare_run_dirs(
                            Path(args.output) / f"baseline_seed{args.seed}",
                            Path(args.output) / f"clock_seed{args.seed}",
                        )
                        atomic_json(output / "clock_comparison.json", comparison)
                    except Exception as exc:
                        print(f"clock comparison skipped: {exc}", flush=True)
                    print(
                        f"clock_gate pass={decision['pass']} epoch={decision['epoch']} {decision['reasons']}",
                        flush=True,
                    )
                    if decision["pass"] and budget.remaining > 60:
                        queue.append("memory")
                    else:
                        print(
                            "Not running memory: clock gate failed or remaining budget < 60s",
                            flush=True,
                        )
                if variant == "memory" and "baseline" in results:
                    decision = decide_memory_gate(
                        results["baseline"], results["memory"]
                    )
                    atomic_json(output / "memory_gate.json", decision)
                    try:
                        comparison = compare_run_dirs(
                            Path(args.output) / f"baseline_seed{args.seed}",
                            Path(args.output) / f"memory_seed{args.seed}",
                        )
                        atomic_json(output / "memory_comparison.json", comparison)
                    except Exception as exc:
                        print(f"memory comparison skipped: {exc}", flush=True)
                    print(
                        f"memory_gate pass={decision['pass']} epoch={decision['epoch']} {decision['reasons']}",
                        flush=True,
                    )
                    if decision["pass"]:
                        print(
                            "Memory earned shuffled/frozen controls; run them in a new suite, do not spend this budget",
                            flush=True,
                        )
        except BaseException:
            write_crash(output, traceback.format_exc())
            raise
        finally:
            budget.persist()


if __name__ == "__main__":
    main()
