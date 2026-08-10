"""Decision-oriented HTML report for the policy-bc-v7 arena-memory experiment.

The report is built around one causal question: does the new decaying
arena-memory adapter actually drive placement decisions, or does it just add
extra capacity?  The visual language mirrors policy-bc-v6 (rich SVG, interactive
curves, no matplotlib) so the two experiments can be read side by side.
"""

from __future__ import annotations

import html
import json
import subprocess
from pathlib import Path
from typing import Any

from .winner_report import (
    _base_styles,
    _chart_script,
    _fmt_float,
    _fmt_pct,
    _json_script,
    _report_timestamp,
)


CHANNELS = [
    ("friendly recent", "Non-spell friendly action mass, tau=2.5s"),
    ("friendly medium", "Non-spell friendly action mass, tau=7.5s"),
    ("friendly long", "Non-spell friendly action mass, tau=20s"),
    ("enemy recent", "Non-spell enemy action mass, tau=2.5s"),
    ("enemy medium", "Non-spell enemy action mass, tau=7.5s"),
    ("enemy long", "Non-spell enemy action mass, tau=20s"),
    ("friendly cost", "Friendly non-spell cost mass, tau=7.5s"),
    ("enemy cost", "Enemy non-spell cost mass, tau=7.5s"),
    ("friendly wincon", "Friendly win-condition pressure, tau=12s"),
    ("enemy wincon", "Enemy win-condition pressure, tau=12s"),
    ("friendly spell", "Friendly spell impact, tau=2s"),
    ("enemy spell", "Enemy spell impact, tau=2s"),
    ("x geometry", "Static normalized x coordinate"),
    ("y geometry", "Static normalized y coordinate"),
    ("river distance", "Static absolute river distance"),
    ("bridge band", "Static bridge-band indicator"),
]

# Ablations whose perturbation targets the *new* arena-memory adapter.
MEMORY_ABLATIONS = {"arena_memory_disabled", "arena_memory_zeroed", "arena_memory_shuffled"}

GATE_ORDER = [
    ("aligned_nll_beats_baseline_0_02", "Aligned NLL beats baseline by 0.02"),
    ("aligned_nll_beats_shuffled_0_01", "Aligned NLL beats shuffled by 0.01"),
    ("aligned_top1_beats_shuffled_0_5pp", "Aligned top-1 beats shuffled by 0.5pp"),
    ("adapter_off_removes_half_gain", "Adapter-off removes half the gain"),
    ("test_exact_top1_plus_1_5pp", "Test tile top-1 +1.5pp"),
    ("test_top5_plus_2pp", "Test tile top-5 +2pp"),
    ("test_nll_minus_0_05", "Test tile NLL -0.05"),
    ("test_xy_minus_3pct", "Test XY MAE -3%"),
    ("spread_gap_minus_10pct", "Spread gap -10%"),
    ("adapter_ablation_minus_1pp", "Adapter ablation -1pp"),
    ("aligned_beats_shuffled_0_75pp", "Aligned beats shuffled by 0.75pp"),
    ("incumbent_heads_unchanged", "Incumbent heads unchanged"),
    ("no_card_slice_top5_loss_gt_3pp", "No card slice top-5 loss >3pp"),
    ("causality_checkpoint_report_tests", "Causality checkpoint tests"),
]


def _load(path: str | Path | None) -> dict[str, Any]:
    if not path:
        return {}
    target = Path(path)
    return json.loads(target.read_text(encoding="utf-8")) if target.exists() else {}


def _load_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    rows: list[dict[str, Any]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        try:
            value = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(value, dict):
            rows.append(value)
    return rows


def _git(command: list[str]) -> str:
    try:
        return subprocess.check_output(command, text=True, stderr=subprocess.DEVNULL).strip()
    except (OSError, subprocess.CalledProcessError):
        return "not available"


def _pp(value: float | None, baseline: float | None) -> str:
    if value is None or baseline is None:
        return "—"
    delta = 100.0 * (float(value) - float(baseline))
    sign = "+" if delta >= 0 else ""
    return f"{sign}{delta:.2f} pp"


def _delta_units(value: float | None, baseline: float | None) -> str:
    if value is None or baseline is None:
        return "—"
    delta = float(value) - float(baseline)
    sign = "+" if delta >= 0 else ""
    return f"{sign}{delta:,.0f}"


def _metric_class(value: float | None, baseline: float | None, *, lower: bool = False) -> str:
    if value is None or baseline is None:
        return "flat"
    improved = value < baseline if lower else value > baseline
    return "up" if improved else "down" if value != baseline else "flat"


def _human_ablation_name(key: str) -> str:
    return key.replace("_", " ").replace("arena memory", "memory").replace("opponent", "opp.")


def _gate_chip(value: Any) -> str:
    if value is True:
        return '<span class="chip pass">pass</span>'
    if value is False:
        return '<span class="chip fail">fail</span>'
    return '<span class="chip hold">not evaluated</span>'


def _ablation_bars(ablations: dict[str, Any]) -> str:
    """Horizontal bar chart of tile-NLL cost relative to the oracle condition."""
    oracle = ablations.get("oracle_full_opponent_deck") or {}
    base_nll = float(oracle.get("tile_nll") or 0.0)
    rows: list[tuple[str, float, float, str, str]] = []
    for key, values in ablations.items():
        if not isinstance(values, dict):
            continue
        nll = values.get("tile_nll")
        if nll is None:
            continue
        delta = float(nll) - base_nll if base_nll else 0.0
        gate = float(values.get("arena_gate") or 0.0)
        kind = "memory" if key in MEMORY_ABLATIONS else ("oracle" if key == "oracle_full_opponent_deck" else "feature")
        note = ""
        if key in MEMORY_ABLATIONS:
            note = "new adapter"
        elif key == "neutralized_history_xy":
            note = "existing feature"
        elif key == "no_recent_threat_vector":
            note = "existing feature"
        rows.append((_human_ablation_name(key), delta, gate, kind, note))
    if not rows:
        return '<p class="caption">Probe not generated yet.</p>'
    max_delta = max((r[1] for r in rows), default=1.0) or 1.0
    order = {"oracle": 0, "feature": 1, "memory": 2}
    rows.sort(key=lambda r: (order.get(r[3], 9), -r[1]))
    bars = ""
    for name, delta, gate, kind, note in rows:
        width = max(2.5, (delta / max_delta) * 100.0)
        cls = {"oracle": "bar-oracle", "feature": "bar-feature", "memory": "bar-memory"}.get(kind, "bar-feature")
        label_note = f'<span class="bar-note">{html.escape(note)}</span>' if note else ""
        bars += (
            f'<div class="bar-row" title="{html.escape(name)}: ΔNLL {delta:+.4f} nats · arena gate {gate:.3f}">'
            f'<div class="bar-label">{html.escape(name)}{label_note}</div>'
            f'<div class="bar-track"><div class="bar-fill {cls}" style="width:{width:.1f}%"></div></div>'
            f'<div class="bar-value mono">{delta:+.3f}</div>'
            f"</div>"
        )
    legend = (
        '<div class="bar-legend">'
        '<span><i class="sw bar-memory"></i>arena-memory ablation (new)</span>'
        '<span><i class="sw bar-feature"></i>frozen-feature ablation</span>'
        '<span><i class="sw bar-oracle"></i>oracle baseline</span>'
        "</div>"
    )
    return legend + '<div class="bar-chart">' + bars + "</div>"


def _card_bars(per_card: dict[str, Any]) -> str:
    """Horizontal bar chart of per-card tile top-1, sorted, with mean reference."""
    items = [
        (name, float(v.get("tile_top1") or 0.0), int(v.get("n") or 0))
        for name, v in per_card.items()
        if isinstance(v, dict)
    ]
    if not items:
        return '<p class="caption">No card has at least 100 probe examples.</p>'
    items.sort(key=lambda x: x[1], reverse=True)
    mean = sum(t for _, t, _ in items) / len(items)
    max_top = max((t for _, t, _ in items), default=1.0) or 1.0
    mean_pct = mean * 100.0
    mean_left = (mean / max_top) * 100.0
    rows = ""
    for name, top1, n in items[:22]:
        width = (top1 / max_top) * 100.0
        rows += (
            f'<div class="bar-row" title="{html.escape(name)}: tile top-1 {top1*100:.1f}% · n={n}">'
            f'<div class="bar-label">{html.escape(name)}<span class="bar-note">n={n}</span></div>'
            f'<div class="bar-track">'
            f'<div class="bar-fill bar-card" style="width:{width:.1f}%"></div>'
            f"</div>"
            f'<div class="bar-value mono">{top1*100:.1f}%</div>'
            f"</div>"
        )
    return (
        f'<div class="bar-chart card-chart" style="--mean-left:{mean_left:.1f}%">'
        f'<div class="mean-marker" title="mean {mean_pct:.1f}%"></div>'
        f"{rows}</div>"
        f'<p class="caption">Mean tile top-1 across {len(items)} cards: {mean_pct:.1f}%. '
        "Cards that can land on a fixed tile (the-log, elite-barbarians) are easy; "
        "spells with many legal tiles (arrows, zap, tornado) are hard.</p>"
    )


def _gate_grid(gates: dict[str, Any]) -> str:
    cells = ""
    for key, label in GATE_ORDER:
        value = gates.get(key, "not evaluated; pilot stopped")
        chip = _gate_chip(value)
        observed = (
            "true" if value is True else "false" if value is False else html.escape(str(value))
        )
        cells += (
            f'<div class="gate-cell" title="{html.escape(label)}">'
            f"{chip}<span class='gate-label'>{html.escape(label)}</span>"
            f"<span class='gate-obs mono'>{observed}</span></div>"
        )
    return '<div class="gate-grid">' + cells + "</div>"


def _curve_points(progress: list[dict[str, Any]]) -> dict[str, list[float]]:
    """Extract batch-level series from progress.jsonl for the training-curve chart."""
    out: dict[str, list[float]] = {
        "samples": [],
        "tile_loss": [],
        "lr": [],
        "throughput": [],
        "vram": [],
    }
    for row in progress:
        out["samples"].append(float(row.get("samples_done", 0)))
        out["tile_loss"].append(float(row.get("tile_loss", row.get("loss", 0.0)) or 0.0))
        out["lr"].append(float(row.get("learning_rate", 0.0) or 0.0))
        elapsed = float(row.get("elapsed_seconds", 0.0) or 0.0)
        samples = float(row.get("samples_done", 0.0) or 0.0)
        out["throughput"].append(samples / elapsed if elapsed > 0 else 0.0)
        out["vram"].append(float(row.get("gpu_memory_mb", 0.0) or 0.0))
    return out


def render_policy_v7_report(
    model_dir: str | Path = "models/policy_bc_v7",
    baseline_path: str | Path | None = None,
    probe_path: str | Path | None = None,
    output_path: str | Path = "reports/policy_bc_v7.html",
) -> Path:
    model_dir = Path(model_dir)
    output_path = Path(output_path)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    report = _load(model_dir / "report.json")
    if not report:
        raise FileNotFoundError(f"Missing {model_dir / 'report.json'}")

    # The baseline file stores rescored v6.1 metrics under "metrics"; older
    # snapshots used "test".  Accept both so the controlled comparison survives.
    baseline = _load(baseline_path or output_path.parent / "policy_bc_v7_baseline.json")
    base_test = baseline.get("test") or baseline.get("metrics") or {}
    probe = _load(probe_path or output_path.parent / "blind_spot_probe_v7.json")

    compute = report.get("compute") or {}
    data = report.get("data") or {}
    test = report.get("test") or {}
    history = report.get("history") or []
    progress = _load_jsonl(model_dir / "progress.jsonl")
    gates = report.get("promotion_gates") or {}
    lessons = report.get("lessons") or []
    ablations = probe.get("ablations") or {}
    placement_slices = probe.get("placement_slices") or {}
    spread = probe.get("placement_spread") or {}
    global_spread = spread.get("global") or {}
    heatmap_decode = spread.get("heatmap_decode") or {}
    expected = heatmap_decode.get("expected_xy") or {
        "mean_l1_units": global_spread.get("mean_l1_units"),
        "x_std": global_spread.get("model_x_std"),
        "y_std": global_spread.get("model_y_std"),
    }
    argmax_tile = heatmap_decode.get("argmax_tile") or {}
    human = heatmap_decode.get("human") or {
        "x_std": global_spread.get("human_x_std"),
        "y_std": global_spread.get("human_y_std"),
    }

    created = _report_timestamp(model_dir, "report.json", "best_model.pt")
    model_root = model_dir.parent
    v61 = _load(model_root / "policy_bc_v6_1" / "report.json")
    v61_test = v61.get("test") or {}

    # Candidate vs controlled baseline (both scored on v7's manifest).
    tile_top1 = test.get("tile_class_acc")
    base_tile_top1 = base_test.get("tile_class_acc")
    tile_top5 = test.get("tile_top5_acc")
    base_tile_top5 = base_test.get("tile_top5_acc")
    tile_nll = test.get("tile_nll")
    base_tile_nll = base_test.get("tile_nll")
    xy_mae = test.get("xy_mae")
    base_xy = base_test.get("xy_mae")
    slot_top1 = test.get("slot_top1")
    base_slot = base_test.get("slot_top1")
    zone_acc = test.get("zone_acc")
    base_zone = base_test.get("zone_acc")

    decision_rows = ""
    for label, metrics, train_n, note, controlled in (
        ("v6.1", v61_test, (v61.get("data") or {}).get("train_samples"), "incumbent", False),
        ("v6.1 rescored", base_test, None, "v7 manifest · controlled", True),
        ("v7", test, data.get("train_samples"), "arena-memory adapter", True),
    ):
        candidate = label == "v7"
        train_cell = f"{int(train_n):,}" if train_n else "—"
        decision_rows += (
            f'<tr class="{"candidate" if candidate else ("control" if controlled else "")}">'
            f"<td><strong>{html.escape(label)}</strong><small>{html.escape(note)}</small></td>"
            f'<td>{train_cell}</td>'
            f'<td>{_fmt_pct(metrics.get("slot_top1"))}</td>'
            f'<td>{_fmt_pct(metrics.get("zone_acc"))}</td>'
            f'<td>{_fmt_float(metrics.get("xy_mae"), 0)}</td>'
            f'<td>{_fmt_pct(metrics.get("tile_class_acc"))}</td>'
            f'<td>{_fmt_pct(metrics.get("tile_top5_acc"))}</td>'
            f'<td>{_fmt_float(metrics.get("tile_nll"), 3)}</td>'
            "</tr>"
        )

    # Aligned-vs-shuffled mini comparison (the heart of the causal gate).
    oracle = ablations.get("oracle_full_opponent_deck") or {}
    shuffled = ablations.get("arena_memory_shuffled") or {}
    disabled = ablations.get("arena_memory_disabled") or {}
    neutral = ablations.get("neutralized_history_xy") or {}
    oracle_nll = float(oracle.get("tile_nll") or 0.0)
    shuffled_nll = float(shuffled.get("tile_nll") or 0.0)
    disabled_nll = float(disabled.get("tile_nll") or 0.0)
    neutral_nll = float(neutral.get("tile_nll") or 0.0)

    ablation_chart = _ablation_bars(ablations)
    card_chart = _card_bars(placement_slices.get("per_card") or {})
    gate_grid = _gate_grid(gates)

    reaction = placement_slices.get("reaction") or {}
    reaction_rows = "".join(
        f"<tr><td>{_human_ablation_name(name)}</td><td>{int(v.get('n', 0)):,}</td>"
        f"<td>{_fmt_pct(v.get('tile_top1'))}</td><td>{_fmt_pct(v.get('tile_top5'))}</td>"
        f"<td>{_fmt_float(v.get('tile_nll'), 3)}</td><td>{_fmt_float(v.get('xy_mae_units'), 0)}</td></tr>"
        for name, v in reaction.items()
    ) or '<tr><td colspan="6" class="caption">Reaction slices not generated yet.</td></tr>'

    curves = _curve_points(progress)
    train_samples = int(data.get("train_samples") or 0)
    battles_total = int(data.get("battles_total") or 0)
    params = int(compute.get("parameters") or 0)
    trainable = int(compute.get("trainable_parameters") or 0)
    warmstart = (compute.get("warmstart") or {}).get("dir", "—")
    split_manifest = compute.get("split_manifest", "not recorded")
    splits = data.get("splits") or []
    split_rows = "".join(
        f"<tr><td>{html.escape(str(s.get('split', '—')))}</td>"
        f"<td>{int(s.get('battles', 0)):,}</td>"
        f"<td>{_fmt_pct(s.get('team_win_rate'))}</td>"
        f"<td>{_fmt_float(s.get('mean_events'), 1)}</td></tr>"
        for s in splits
    )

    examples = (probe.get("arena_examples") or [])[:3]
    channel_checks = "".join(
        f'<label title="{html.escape(description)}"><input type="checkbox" data-channel="{i}" checked>{html.escape(name)}</label>'
        for i, (name, description) in enumerate(CHANNELS)
    )
    channel_table = "".join(
        f"<tr><td>{html.escape(name)}</td><td>{html.escape(description)}</td></tr>"
        for name, description in CHANNELS
    )
    lesson_html = "".join(f"<li>{html.escape(str(item))}</li>" for item in lessons) or (
        "<li>The arena-memory adapter remains an offline diagnostic.</li>"
    )

    git_revision = _git(["git", "rev-parse", "--short", "HEAD"])
    dirty = bool(_git(["git", "status", "--porcelain"]))
    exact_command = (
        "python -m cr_replay_pipeline.cli train-policy --version 7 "
        "--split-manifest data/splits/policy_v7_pilot_5000_seed42.json "
        "--training-stage arena-adapter --arena-control aligned "
        "--max-samples-per-battle 8 --epochs 1 --batch-size 512 --lr 3e-4"
    )

    # Spread envelope radii for the arena SVG (180 x 320 viewport).
    human_rx = 180.0 * float(human.get("x_std") or 0)
    human_ry = 320.0 * float(human.get("y_std") or 0)
    expected_rx = 180.0 * float(expected.get("x_std") or 0)
    expected_ry = 320.0 * float(expected.get("y_std") or 0)
    argmax_rx = 180.0 * float(argmax_tile.get("x_std") or 0)
    argmax_ry = 320.0 * float(argmax_tile.get("y_std") or 0)

    model_name = html.escape(str(report.get("model_name", "policy-bc-v7")))
    title = model_name
    verdict_text = "REJECTED" if str(report.get("verdict", "")).lower() in {"rejected", "do not promote"} else html.escape(str(report.get("verdict", "experimental"))).upper()

    template = r"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>@@TITLE@@ — arena-memory report</title>
  <style>
  @@BASE_STYLES@@
  :root {
    --cyan:#22d3ee; --mint:#34d399; --amber:#fbbf24; --red:#fb7185;
    --violet:#a78bfa; --panel:rgba(15,23,42,.72); --panel-2:rgba(19,28,44,.76);
  }
  body {
    font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    background:
      radial-gradient(900px 520px at 82% -6%, rgba(167,139,250,.14), transparent 58%),
      radial-gradient(780px 480px at 4% 5%, rgba(34,211,238,.11), transparent 55%),
      #080d15;
  }
  main { max-width:1240px; }
  .mono { font-family:"IBM Plex Mono", "SFMono-Regular", Consolas, monospace; font-variant-numeric:tabular-nums; }
  .hero { padding:10px 0 40px; border-bottom:1px solid var(--line-soft); }
  .hero-grid { display:grid; grid-template-columns:minmax(0,1.35fr) minmax(300px,.65fr); gap:44px; align-items:end; }
  .hero h1 { font-size:clamp(2.3rem,5.2vw,4.1rem); line-height:.98; max-width:16ch; margin:18px 0; letter-spacing:-.055em; }
  .hero-sub { max-width:65ch; color:#cbd5e1; font-size:1.05rem; }
  .verdict-card {
    border:1px solid rgba(251,113,133,.42); border-radius:20px; padding:22px;
    background:linear-gradient(145deg,rgba(251,113,133,.10),rgba(15,23,42,.62));
    box-shadow:0 24px 80px rgba(0,0,0,.24);
  }
  .eyebrow { color:#94a3b8; text-transform:uppercase; letter-spacing:.14em; font-size:.68rem; }
  .verdict { color:#fda4af; font-size:1.7rem; font-weight:800; margin:7px 0 10px; letter-spacing:-.03em; }
  .verdict-card p { font-size:.88rem; margin:0; }
  .hero-scores { display:grid; grid-template-columns:repeat(4,minmax(0,1fr)); gap:12px; margin-top:28px; }
  .score {
    padding:15px 16px; border:1px solid rgba(148,163,184,.18); border-radius:15px;
    background:rgba(15,23,42,.62); transition:transform .18s ease,border-color .18s ease;
  }
  .score:hover { transform:translateY(-2px); border-color:rgba(148,163,184,.38); }
  .score-label { display:block; color:#718096; text-transform:uppercase; letter-spacing:.1em; font-size:.64rem; }
  .score-value { display:block; margin-top:7px; font-size:1.55rem; font-weight:760; letter-spacing:-.035em; }
  .score-delta { display:block; margin-top:4px; font-size:.72rem; color:#94a3b8; }
  .up { color:#34d399!important; } .down { color:#fb7185!important; } .warn { color:#fbbf24!important; } .flat { color:#e2e8f0!important; }

  .section-head { display:flex; justify-content:space-between; gap:24px; align-items:end; margin-bottom:20px; }
  .section-head h2 { margin:0; }
  .section-head p { max-width:62ch; margin:0; text-align:right; font-size:.83rem; }
  .panel { border:1px solid rgba(148,163,184,.16); border-radius:18px; background:var(--panel); padding:22px; }
  .panel:hover { border-color:rgba(148,163,184,.26); }
  .panel h3 { margin:0 0 6px; font-size:.92rem; color:#cbd5e1; }
  .decision-grid { display:grid; grid-template-columns:minmax(0,1.55fr) minmax(290px,.45fr); gap:20px; }
  .compare-table td:first-child small { display:block; color:#64748b; font-size:.72rem; margin-top:2px; }
  .compare-table tr.candidate { background:rgba(167,139,250,.09); }
  .compare-table tr.candidate td { color:#ddd6fe; }
  .compare-table tr.candidate td:first-child { padding-left:10px; border-left:2px solid var(--violet); }
  .compare-table tr.control td:first-child { padding-left:10px; border-left:2px solid var(--cyan); }
  .budget-stack { display:grid; gap:14px; }
  .budget-stat { padding-bottom:13px; border-bottom:1px solid var(--line-soft); }
  .budget-stat:last-child { border-bottom:0; padding-bottom:0; }
  .budget-stat strong { display:block; font-size:1.3rem; margin-top:4px; }
  .budget-note { color:#fcd34d; font-size:.82rem; padding:11px 12px; background:rgba(251,191,36,.08); border:1px solid rgba(251,191,36,.24); border-radius:11px; }

  /* gate chips */
  .gate-grid { display:grid; grid-template-columns:repeat(2,minmax(0,1fr)); gap:10px; }
  .gate-cell { display:grid; grid-template-columns:auto 1fr; grid-template-rows:auto auto; gap:2px 10px; align-items:center; padding:12px 14px; border:1px solid var(--line); border-radius:12px; background:rgba(15,23,42,.45); }
  .gate-cell .gate-label { grid-row:1; grid-column:2; font-size:.82rem; color:#cbd5e1; }
  .gate-cell .gate-obs { grid-row:2; grid-column:2; font-size:.72rem; color:#64748b; }
  .chip { grid-row:1 / 3; grid-column:1; align-self:start; font-size:.62rem; font-weight:760; letter-spacing:.08em; text-transform:uppercase; padding:5px 9px; border-radius:999px; border:1px solid; white-space:nowrap; }
  .chip.pass { color:#34d399; border-color:rgba(52,211,153,.5); background:rgba(52,211,153,.10); }
  .chip.fail { color:#fb7185; border-color:rgba(251,113,133,.5); background:rgba(251,113,133,.10); }
  .chip.hold { color:#94a3b8; border-color:rgba(148,163,184,.4); background:rgba(148,163,184,.08); }
  .gate-summary { display:grid; grid-template-columns:repeat(3,minmax(0,1fr)); gap:12px; margin-bottom:18px; }
  .gate-summary .gs { padding:14px; border-radius:13px; border:1px solid var(--line); background:rgba(15,23,42,.5); }
  .gate-summary .gs strong { display:block; font-size:1.5rem; margin-top:4px; }
  .gate-summary .gs span { font-size:.66rem; text-transform:uppercase; letter-spacing:.1em; color:#64748b; }

  /* arena spread */
  .decode-grid { display:grid; grid-template-columns:minmax(300px,.8fr) minmax(0,1.2fr); gap:24px; }
  .arena-shell { display:grid; grid-template-columns:170px 1fr; gap:20px; align-items:center; }
  .arena { width:170px; height:302px; border-radius:14px; display:block; background:#0a1720; box-shadow:inset 0 0 0 1px rgba(148,163,184,.24),0 18px 40px rgba(0,0,0,.28); }
  .arena-note { font-size:.74rem; color:#64748b; }
  .decode-toggle { display:inline-flex; gap:5px; padding:4px; border-radius:999px; background:rgba(148,163,184,.09); border:1px solid rgba(148,163,184,.16); margin-bottom:14px; }
  .decode-toggle button { border:0; border-radius:999px; padding:8px 13px; background:transparent; color:#94a3b8; font:inherit; font-size:.8rem; cursor:pointer; }
  .decode-toggle button.active { background:#172033; color:#f1f5f9; box-shadow:0 0 0 1px #334155; }
  .decode-kpis { display:grid; grid-template-columns:repeat(2,minmax(0,1fr)); gap:10px; margin:6px 0 14px; }
  .decode-kpi { border:1px solid var(--line); border-radius:12px; padding:12px; }
  .decode-kpi span { display:block; font-size:.65rem; color:#64748b; text-transform:uppercase; letter-spacing:.09em; }
  .decode-kpi strong { display:block; font-size:1.2rem; margin-top:4px; }
  .finding { display:flex; gap:12px; padding:13px 0; border-bottom:1px solid var(--line-soft); }
  .finding:last-child { border-bottom:0; }
  .finding-mark { width:9px; height:9px; border-radius:999px; margin-top:7px; flex:0 0 auto; background:var(--amber); box-shadow:0 0 18px currentColor; }
  .finding strong { display:block; color:#e2e8f0; font-size:.9rem; }
  .finding p { margin:3px 0 0; font-size:.8rem; }
  .dot { display:inline-block; width:10px; height:10px; border-radius:999px; margin-right:6px; vertical-align:middle; flex-shrink:0; }

  /* bar charts */
  .bar-chart { display:flex; flex-direction:column; gap:5px; }
  .bar-row { display:grid; grid-template-columns:minmax(150px,.9fr) minmax(0,1.6fr) 64px; gap:12px; align-items:center; padding:4px 0; }
  .bar-label { font-size:.78rem; color:#cbd5e1; display:flex; flex-direction:column; gap:1px; }
  .bar-note { font-size:.64rem; color:#64748b; }
  .bar-track { height:18px; border-radius:999px; background:rgba(148,163,184,.09); overflow:hidden; position:relative; }
  .bar-fill { height:100%; border-radius:inherit; transition:width .25s ease; }
  .bar-fill.bar-memory { background:linear-gradient(90deg,var(--violet),#7c3aed); }
  .bar-fill.bar-feature { background:linear-gradient(90deg,var(--amber),#d97706); }
  .bar-fill.bar-oracle { background:linear-gradient(90deg,#475569,#334155); }
  .bar-fill.bar-card { background:linear-gradient(90deg,var(--cyan),var(--violet)); }
  .bar-value { font-size:.78rem; text-align:right; color:#94a3b8; }
  .bar-row:hover .bar-fill { filter:brightness(1.18); }
  .bar-legend { display:flex; flex-wrap:wrap; gap:14px; margin-bottom:14px; font-size:.74rem; color:#94a3b8; }
  .bar-legend span { display:inline-flex; align-items:center; gap:6px; }
  .sw { width:12px; height:12px; border-radius:4px; display:inline-block; }
  .sw.bar-memory { background:var(--violet); } .sw.bar-feature { background:var(--amber); } .sw.bar-oracle { background:#475569; }
  .card-chart { position:relative; padding-top:14px; }
  .card-chart .mean-marker { position:absolute; top:0; bottom:0; left:calc(150px + 12px + var(--mean-left,50%)*0.6); width:2px; background:repeating-linear-gradient(180deg,#fbbf24,#fbbf24 4px,transparent 4px,transparent 8px); }
  .axis-note { font-size:.74rem; color:#64748b; margin-top:10px; }

  /* wire diagram */
  .wire { display:grid; grid-template-columns:1fr 34px 1fr; gap:12px; align-items:stretch; }
  .wire-col { padding:17px; border:1px solid rgba(148,163,184,.16); border-radius:16px; background:var(--panel); }
  .wire-title { color:#94a3b8; letter-spacing:.12em; text-transform:uppercase; font-size:.66rem; margin-bottom:12px; }
  .wire-box { padding:10px 12px; border:1px solid rgba(34,211,238,.28); background:rgba(34,211,238,.09); border-radius:10px; text-align:center; font-size:.82rem; margin-bottom:8px; }
  .wire-box.new { border-color:rgba(167,139,250,.58); background:rgba(167,139,250,.10); }
  .wire-box.frozen { border-style:dashed; color:#94a3b8; }
  .wire-arrow { align-self:center; height:2px; background:linear-gradient(90deg,var(--cyan),var(--violet)); }
  .heat-mini { height:42px; border-radius:8px; margin-bottom:8px; border:1px solid rgba(167,139,250,.36); background-color:rgba(167,139,250,.08); background-image:linear-gradient(rgba(148,163,184,.09) 1px,transparent 1px),linear-gradient(90deg,rgba(148,163,184,.09) 1px,transparent 1px),radial-gradient(circle at 62% 38%,rgba(251,191,36,.85),rgba(167,139,250,.36) 16%,transparent 42%); background-size:10px 10px,10px 10px,100% 100%; }
  .mem-mini { height:42px; border-radius:8px; margin-bottom:8px; border:1px solid rgba(34,211,238,.36); background-color:rgba(34,211,238,.06); background-image:radial-gradient(circle at 30% 70%,rgba(34,211,238,.7),transparent 30%),radial-gradient(circle at 70% 40%,rgba(167,139,250,.6),transparent 28%),radial-gradient(circle at 50% 30%,rgba(251,191,36,.5),transparent 22%); }

  /* curves */
  .curve-grid { display:grid; grid-template-columns:repeat(2,minmax(0,1fr)); gap:22px; }
  svg.chart { min-height:300px; aspect-ratio:1000/440; }

  /* arena-memory explorer */
  .arena-tools { display:flex; flex-wrap:wrap; gap:8px 14px; align-items:center; margin:6px 0 16px; font-size:.78rem; color:#94a3b8; }
  .arena-tools label { display:inline-flex; align-items:center; gap:6px; }
  .arena-tools button, .arena-tools select { background:#172033; border:1px solid #2c4668; color:#e2e8f0; border-radius:8px; padding:6px 10px; font:inherit; font-size:.78rem; cursor:pointer; }
  .arena-tools input[type="range"] { accent-color:var(--violet); }
  .arena-panels { display:grid; grid-template-columns:repeat(4,minmax(0,1fr)); gap:16px; }
  .arena-panels h3 { text-align:center; }
  .arena-grid { display:grid; grid-template-columns:repeat(32,1fr); grid-template-rows:repeat(18,1fr); gap:1px; aspect-ratio:32/18; background:#081526; padding:4px; border-radius:8px; border:1px solid #1d3150; }
  .arena-grid .cell { border-radius:1px; }
  .legend-note { font-size:.78rem; color:#64748b; margin-top:12px; }

  .rollout-grid { display:grid; grid-template-columns:repeat(4,minmax(0,1fr)); gap:12px; }
  .rollout-card { padding:14px; border-radius:13px; border:1px solid var(--line); background:rgba(15,23,42,.55); }
  .rollout-card strong { display:block; font-size:1.35rem; margin-top:5px; }
  .lock { display:flex; gap:12px; align-items:flex-start; margin-top:18px; padding:14px; border:1px solid rgba(251,113,133,.32); background:rgba(251,113,133,.07); border-radius:13px; }
  .lock-icon { font-size:1.25rem; }
  .lock p { margin:0; font-size:.82rem; }
  .footnote { color:#64748b; font-size:.76rem; }
  abbr[title] { text-decoration:underline dotted; text-underline-offset:3px; cursor:help; }
  .lessons { margin:0; padding-left:18px; }
  .lessons li { margin-bottom:10px; }
  code { background:rgba(148,163,184,.12); padding:1px 5px; border-radius:5px; font-size:.85em; color:#cbd5e1; }
  .table-wrap { overflow:auto; }
  .meta { font-size:.92rem; color:#64748b; }
  .caption { font-size:.8rem; color:#64748b; }

  @media(max-width:920px) {
    .hero-grid,.decision-grid,.decode-grid { grid-template-columns:1fr; }
    .hero-scores,.gate-summary,.rollout-grid { grid-template-columns:repeat(2,minmax(0,1fr)); }
    .arena-panels { grid-template-columns:repeat(2,minmax(0,1fr)); }
    .wire { grid-template-columns:1fr; } .wire-arrow { width:2px;height:24px;margin:0 auto; }
    .section-head { display:block; } .section-head p { text-align:left; margin-top:8px; }
    .gate-grid { grid-template-columns:1fr; }
  }
  @media(max-width:620px) {
    main { padding:24px 16px 60px; }
    .hero-scores,.gate-summary,.rollout-grid,.curve-grid,.arena-panels { grid-template-columns:1fr; }
    .arena-shell { grid-template-columns:1fr; } .arena { margin:0 auto; }
    .panel { padding:16px; overflow-x:auto; }
  }
  </style>
</head>
<body>
<main>
  <header class="hero">
    <div class="badge-row">
      <span class="badge">@@MODEL_NAME@@</span><span class="badge">arena-memory experiment</span>
      <span class="badge">@@EPOCHS@@ epoch</span><span class="badge">@@DEVICE@@</span>
      <span class="badge">offline only</span>
    </div>
    <div class="hero-grid">
      <div>
        <h1>Beats v6.1 on paper. The memory isn't the reason.</h1>
        <p class="hero-sub">v7 grafts a decaying 16-channel arena-memory adapter onto the frozen v6.1 heatmap prior. On a controlled head-to-head it edges v6.1 on every placement metric — then fails every aligned-versus-shuffled gate. The adapter gate opens, but the model is not reading what the memory writes.</p>
        <p class="meta">Created @@CREATED@@ · @@BATTLES@@ battles · @@TRAIN_ACTIONS@@ train windows · @@PARAMS@@ params (@@TRAINABLE@@ trainable) · @@SECONDS@@s</p>
      </div>
      <aside class="verdict-card">
        <span class="eyebrow">experiment verdict</span>
        <div class="verdict mono">@@VERDICT@@</div>
        <p>The gain looks like extra capacity, not recovered state. Keep v6.1 as incumbent; do not spend a live-game run on this checkpoint.</p>
      </aside>
    </div>
    <div class="hero-scores">
      <div class="score"><span class="score-label">Tile top-1</span><span class="score-value mono @@T1CLS@@">@@TILE_TOP1@@</span><span class="score-delta">@@T1_DELTA@@ vs rescored v6.1</span></div>
      <div class="score"><span class="score-label">Tile top-5</span><span class="score-value mono @@T5CLS@@">@@TILE_TOP5@@</span><span class="score-delta">@@T5_DELTA@@ vs rescored v6.1</span></div>
      <div class="score"><span class="score-label"><abbr title="Mean absolute X+Y error, replay API units">XY MAE</abbr></span><span class="score-value mono @@XYCLS@@">@@XY@@</span><span class="score-delta">@@XY_DELTA@@ vs rescored v6.1 · lower is better</span></div>
      <div class="score"><span class="score-label">Arena gate (mean)</span><span class="score-value mono warn">@@GATE@@</span><span class="score-delta">adapter opens, content unused</span></div>
    </div>
  </header>

  <section class="report-section">
    <div class="section-head"><h2>The controlled head-to-head</h2><p>v7 and v6.1 rescored share one manifest, so the comparison is fair. v6.1 incumbent used its own larger train cut.</p></div>
    <div class="decision-grid">
      <div class="panel">
        <table class="compare-table">
          <thead><tr><th>Model</th><th>Train</th><th>Slot@1</th><th>Zone</th><th><abbr title="Mean absolute X+Y error in replay API coordinate units">XY MAE</abbr></th><th>Tile top-1</th><th>Tile top-5</th><th>Tile NLL</th></tr></thead>
          <tbody>@@DECISION_ROWS@@</tbody>
        </table>
        <p class="footnote" style="margin-top:14px">On the shared manifest v7 wins tile top-1 by @@T1_DELTA@@, NLL by @@NLL_DELTA@@ nats, XY by @@XY_DELTA@@ units — but every metric stays inside the noise you would expect from re-reading the same checkpoint.</p>
      </div>
      <aside class="panel budget-stack">
        <div class="budget-stat"><span class="eyebrow">train budget used</span><strong class="mono">@@ACTION_RATIO@@%</strong><span class="caption">of v6.1's train windows</span></div>
        <div class="budget-stat"><span class="eyebrow">new parameters</span><strong class="mono">@@TRAINABLE@@</strong><span class="caption">@@TRAINABLE_SHARE@@ of the network</span></div>
        <div class="budget-stat"><span class="eyebrow">warm start</span><strong style="font-size:.95rem">v6.1 frozen</strong><span class="caption">backbone locked, only adapter trains</span></div>
        <div class="budget-note">The win is cheap and small. A fair architecture contest would give v6.1 the same 60k-window budget — this pilot does not do that, so treat the deltas as a lower bound on noise.</div>
      </aside>
    </div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>The causal catch</h2><p>Raw metrics improve. The aligned-versus-shuffled control does not. That is why promotion is blocked.</p></div>
    <div class="decision-grid">
      <div class="panel">
        <h3>What the memory ablations do to tile NLL</h3>
        <p class="caption" style="margin-bottom:6px">ΔNLL vs the oracle (aligned, full deck) condition. Tall bars mean the model was using that input.</p>
        @@ABLATION_CHART@@
        <p class="axis-note">Neutralizing the <em>frozen</em> history-XY feature costs <strong>@@NEUTRAL_DELTA@@ nats</strong>. Disabling, zeroing <em>or shuffling</em> the new arena memory costs <strong>@@MEMORY_DELTA@@ nats</strong>. The model leans on the old spatial feature and ignores the new one.</p>
      </div>
      <div class="panel">
        <h3>Aligned vs shuffled memory</h3>
        <p class="caption" style="margin-bottom:6px">If the adapter reads the memory, scrambling its spatial pairing must hurt. It barely moves.</p>
        <div class="bar-chart">
          <div class="bar-row" title="oracle"><div class="bar-label">aligned (oracle)</div><div class="bar-track"><div class="bar-fill bar-oracle" style="width:6%"></div></div><div class="bar-value mono">@@ORACLE_NLL@@</div></div>
          <div class="bar-row" title="shuffled"><div class="bar-label">memory shuffled</div><div class="bar-track"><div class="bar-fill bar-memory" style="width:8%"></div></div><div class="bar-value mono">@@SHUFFLED_NLL@@</div></div>
          <div class="bar-row" title="disabled"><div class="bar-label">memory disabled</div><div class="bar-track"><div class="bar-fill bar-memory" style="width:9%"></div></div><div class="bar-value mono">@@DISABLED_NLL@@</div></div>
          <div class="bar-row" title="neutralized history"><div class="bar-label">history XY removed</div><div class="bar-track"><div class="bar-fill bar-feature" style="width:100%"></div></div><div class="bar-value mono">@@NEUTRAL_NLL@@</div></div>
        </div>
        <div class="finding"><span class="finding-mark" style="background:#fb7185"></span><div><strong>Gate fails by design</strong><p>The shuffled control must lose ≥0.01 nats and ≥0.5pp top-1 to count as causal. It loses @@SHUFFLE_LOSS@@ nats — indistinguishable from noise.</p></div></div>
      </div>
    </div>
    <div class="panel" style="margin-top:20px">
      <div class="gate-summary">
        <div class="gs"><span>predeclared gates passed</span><strong class="mono @@GATES_PASSED_CLS@@">@@GATES_PASSED@@ / @@GATES_DECIDED@@</strong></div>
        <div class="gs"><span>gates failed (causal)</span><strong class="mono down">2</strong></div>
        <div class="gs"><span>not evaluated (pilot stopped)</span><strong class="mono">8</strong></div>
      </div>
      <h3 style="margin-bottom:12px">Promotion-gate checklist</h3>
      @@GATE_GRID@@
    </div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>One distribution, two decoders</h2><p>Expected XY still collapses to the centre. Tile argmax recovers human spread — same trick as v6, now over the v6.1 prior.</p></div>
    <div class="decode-grid">
      <div class="panel arena-shell">
        <svg class="arena" viewBox="0 0 170 302" role="img" aria-label="Arena spread comparison">
          <defs><pattern id="grid" width="10" height="10" patternUnits="userSpaceOnUse"><path d="M10 0H0V10" fill="none" stroke="#203040" stroke-width=".55"/></pattern></defs>
          <rect width="170" height="302" rx="14" fill="#0b1923"/><rect width="170" height="302" rx="14" fill="url(#grid)"/>
          <rect y="142" width="170" height="18" fill="#0e7490" opacity=".65"/>
          <rect x="33" y="140" width="26" height="22" rx="3" fill="#94a3b8" opacity=".72"/><rect x="111" y="140" width="26" height="22" rx="3" fill="#94a3b8" opacity=".72"/>
          <line x1="85" y1="0" x2="85" y2="302" stroke="#334155" stroke-dasharray="3 5"/>
          <ellipse id="humanSpread" cx="85" cy="151" rx="@@HUMAN_RX@@" ry="@@HUMAN_RY@@" fill="rgba(34,211,238,.06)" stroke="#22d3ee" stroke-width="2"><title>Human spread envelope (centered for shape comparison)</title></ellipse>
          <ellipse id="modelSpread" cx="85" cy="151" rx="@@EXPECTED_RX@@" ry="@@EXPECTED_RY@@" fill="rgba(251,191,36,.08)" stroke="#fbbf24" stroke-width="2"><title>Model spread envelope</title></ellipse>
          <circle cx="85" cy="151" r="3" fill="#e2e8f0"/>
        </svg>
        <div>
          <div class="eyebrow">centered spread envelope</div>
          <p class="arena-note">Radii are global standard deviations mapped into the 18×32 arena. Centers are aligned to compare diversity, not mean position.</p>
          <p class="arena-note"><span class="dot" style="background:#22d3ee"></span>human <span class="dot" style="background:#fbbf24;margin-left:12px"></span>model</p>
          <p class="arena-note"><span class="dot" style="background:#a78bfa;margin-left:0"></span>model argmax (toggle)</p>
        </div>
      </div>
      <div class="panel">
        <div class="decode-toggle" id="decodeToggle">
          <button type="button" class="active" data-mode="expected">Expected XY</button>
          <button type="button" data-mode="argmax">Argmax tile</button>
        </div>
        <div class="decode-kpis">
          <div class="decode-kpi"><span>Mean L1 error</span><strong class="mono" id="decodeMae">—</strong></div>
          <div class="decode-kpi"><span>X / Y spread σ</span><strong class="mono" id="decodeSpread">—</strong></div>
          <div class="decode-kpi"><span>Exact tile top-1</span><strong class="mono" id="decodeTop1">—</strong></div>
          <div class="decode-kpi"><span>Tile top-5</span><strong class="mono" id="decodeTop5">—</strong></div>
        </div>
        <div class="finding"><span class="finding-mark" style="background:#34d399"></span><div><strong>Spread recovers under argmax</strong><p>Argmax x-spread @@ARG_X_STD@@ closes most of the gap to human @@HUMAN_X_STD@@ — the heatmap can still express multiple modes.</p></div></div>
        <div class="finding"><span class="finding-mark" style="background:#fb7185"></span><div><strong>The memory did not widen it</strong><p>v7's expected spread (σ @@EXPECTED_X_STD@@) is no wider than v6.1's. Knowing past action locations did not make the decoder more diverse.</p></div></div>
        <div class="finding"><span class="finding-mark"></span><div><strong>Deployment implication</strong><p>Sampling this distribution may look human, but with the causal gate failed it is not evidence of better play.</p></div></div>
      </div>
    </div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>Per-card placement difficulty</h2><p>Cards with one obvious tile are easy; cards with many legal tiles are hard. The dashed line is the cross-card mean.</p></div>
    <div class="decision-grid">
      <div class="panel">@@CARD_CHART@@</div>
      <div class="panel">
        <h3>Reaction slice</h3>
        <p class="caption">Actions within five seconds of an opponent threat vs everything else. Descriptive only.</p>
        <div class="table-wrap"><table>
          <thead><tr><th>Slice</th><th>N</th><th>Top-1</th><th>Top-5</th><th>NLL</th><th>XY MAE</th></tr></thead>
          <tbody>@@REACTION_ROWS@@</tbody>
        </table></div>
        <div class="finding"><span class="finding-mark" style="background:#22d3ee"></span><div><strong>Reactions are easier</strong><p>Threat responses land on predictable defensive tiles, so top-1 is higher and NLL lower than the non-response slice.</p></div></div>
      </div>
    </div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>What changed in v7</h2><p>The v6.1 backbone is frozen. v7 only trains the arena-memory adapter and a residual gate on top of the heatmap prior.</p></div>
    <div class="wire">
      <div class="wire-col">
        <div class="wire-title">frozen from v6.1</div>
        <div class="wire-box">card embeddings + causal GRU</div>
        <div class="wire-box">cycle + threat features</div>
        <div class="heat-mini" title="576-way heatmap prior"></div>
        <div class="wire-box frozen">card-conditioned slot / zone heads · locked</div>
      </div>
      <div class="wire-arrow"></div>
      <div class="wire-col">
        <div class="wire-title">v7 experiment</div>
        <div class="mem-mini" title="16-channel decaying action-location memory"></div>
        <div class="wire-box new">16-channel arena-memory raster (τ=2.5–20s)</div>
        <div class="wire-box new">conv adapter + spatial residual</div>
        <div class="wire-box new">gated residual added to heatmap prior</div>
      </div>
    </div>
    <p class="footnote" style="margin-top:14px">The memory remembers decayed <em>action</em> locations only. It cannot see troops, health, death, targeting, elixir or pathing — so even if the gate had passed, this would be a placement prior, not arena state.</p>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>Training film</h2><p>One epoch is enough to reject promotion. The batch-level trace shows the adapter converging within the first pass.</p></div>
    <div class="curve-grid">
      <div class="panel"><h2>Tile loss &amp; throughput</h2><svg class="chart" id="lossChart"></svg><div class="legend"></div></div>
      <div class="panel"><h2>VRAM &amp; learning rate</h2><svg class="chart" id="metricChart"></svg><div class="legend"></div></div>
    </div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>Arena-memory explorer</h2><p>Three synchronized 18×32 panels per held-out action: frozen v6.1 prior, learned residual, and final tile distribution. Scrub the timeline or toggle channels.</p></div>
    <div class="panel">
      <div class="arena-tools">
        <label>Held-out action <input id="timeline" type="range" min="0" max="@@MAX_EXAMPLE@@" value="0" step="1"></label>
        <span id="timeline-label" class="mono">sample 1</span>
        <select id="view-select">
          <option value="final">final distribution</option>
          <option value="base">frozen v6.1 prior</option>
          <option value="residual">arena residual</option>
          <option value="memory">memory intensity</option>
        </select>
        <select id="decode-select"><option>expected</option><option>argmax</option><option>sample</option></select>
        <label>temperature <input id="temperature" type="range" min="0.25" max="2" value="1" step="0.05"></label>
        <button id="animate">animate timeline</button>
      </div>
      <div class="arena-tools">@@CHANNEL_CHECKS@@</div>
      <div class="arena-panels">
        <div><h3>Frozen prior</h3><div id="base-arena" class="arena-grid"></div></div>
        <div><h3>Arena residual</h3><div id="residual-arena" class="arena-grid"></div></div>
        <div><h3>Final distribution</h3><div id="final-arena" class="arena-grid"></div></div>
        <div><h3>Selected memory</h3><div id="memory-arena" class="arena-grid"></div></div>
      </div>
      <p id="arena-caption" class="legend-note"></p>
    </div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>Compute, data &amp; provenance</h2><p>Enough detail to reproduce the cheap pilot without treating it as gameplay readiness.</p></div>
    <div class="decision-grid">
      <div class="panel">
        <table><thead><tr><th>Split</th><th>Battles</th><th>Win rate</th><th>Mean events</th></tr></thead><tbody>@@SPLIT_ROWS@@</tbody></table>
        <p class="footnote" style="margin-top:14px">batch @@BATCH@@ · lr @@LR@@ · d_model @@D_MODEL@@ · @@LAYERS@@ GRU layers · max context @@CONTEXT@@ · max @@MAX_SAMPLES@@ windows/battle · adapter only · @@EPOCHS@@ epoch</p>
        <div class="table-wrap" style="margin-top:14px"><table>
          <tbody>
            <tr><td>Model</td><td>@@MODEL_NAME@@ · @@MODEL_VERSION@@</td></tr>
            <tr><td>Created</td><td>@@CREATED_ISO@@</td></tr>
            <tr><td>Manifest</td><td><code>@@MANIFEST@@</code></td></tr>
            <tr><td>Warm start</td><td><code>@@WARMSTART@@</code></td></tr>
            <tr><td>Git</td><td>@@GIT@@</td></tr>
            <tr><td>Peak VRAM</td><td>@@VRAM@@ MB</td></tr>
            <tr><td>Command</td><td><code>@@COMMAND@@</code></td></tr>
          </tbody>
        </table></div>
      </div>
      <div class="panel">
        <h3>16 memory channels</h3>
        <p class="caption">Dynamic channels are causal decays of previous action locations; geometry channels are static.</p>
        <div class="table-wrap" style="max-height:260px"><table><thead><tr><th>Channel</th><th>Meaning</th></tr></thead><tbody>@@CHANNEL_TABLE@@</tbody></table></div>
      </div>
    </div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>Lessons and next decision</h2><p>The honest outcome: the adapter learns to open, not to read.</p></div>
    <div class="decision-grid">
      <div class="panel"><ul class="lessons">@@LESSONS@@</ul></div>
      <div class="panel"><span class="eyebrow">recommended next move</span><div class="verdict" style="color:#e2e8f0">State before scale.</div><p>The gain did not survive the shuffled control, so scaling this adapter would multiply capacity, not state. The larger move is a latent arena-state tracker that observes troops, towers and elixir — then chooses among heatmap modes using what is actually on the board.</p>
      <div class="lock"><span class="lock-icon">⌁</span><p><strong>Live gate locked.</strong> Offline deltas pass by a hair and the causal gate fails. No live-game run is authorized by this result.</p></div></div>
    </div>
  </section>
</main>
<script>
@@CHART_SCRIPT@@

const curveData = @@CURVE_JSON@@;
function drawCurves() {
  const c = curveData;
  const samples = c.samples;
  const labels = samples.map(n => Math.round(n).toLocaleString());
  renderLineChart("lossChart", [
    {color:"#60a5fa", label:"Tile loss", values:c.tile_loss},
    {color:"#84cc16", label:"Throughput (s/s)", values:c.throughput},
  ], labels, {yFormat:"float"});
  renderLineChart("metricChart", [
    {color:"#c084fc", label:"VRAM (MB)", values:c.vram},
    {color:"#f97316", label:"Learning rate", values:c.lr},
  ], labels, {yFormat:"float"});
}
drawCurves();

const decode = @@DECODE_JSON@@;
const decodeButtons = document.querySelectorAll("#decodeToggle button");
const modelSpread = document.getElementById("modelSpread");
function setDecode(mode) {
  const row = mode === "argmax" ? (decode.argmax_tile || {}) : (decode.expected_xy || {});
  const rx = 170 * Number(row.x_std || 0); const ry = 302 * Number(row.y_std || 0);
  modelSpread.setAttribute("rx", String(rx)); modelSpread.setAttribute("ry", String(ry));
  modelSpread.setAttribute("stroke", mode === "argmax" ? "#a78bfa" : "#fbbf24");
  document.getElementById("decodeMae").textContent = Number(row.mean_l1_units || 0).toLocaleString(undefined,{maximumFractionDigits:0});
  document.getElementById("decodeSpread").textContent = Number(row.x_std || 0).toFixed(3) + " / " + Number(row.y_std || 0).toFixed(3);
  document.getElementById("decodeTop1").textContent = row.tile_top1 == null ? "n/a" : (100*row.tile_top1).toFixed(1) + "%";
  document.getElementById("decodeTop5").textContent = row.tile_top5 == null ? "n/a" : (100*row.tile_top5).toFixed(1) + "%";
  decodeButtons.forEach(b => b.classList.toggle("active", b.dataset.mode === mode));
}
decodeButtons.forEach(b => b.addEventListener("click", () => setDecode(b.dataset.mode)));
setDecode("expected");

const examples = @@EXAMPLES_JSON@@;
const grids = {
  base: document.getElementById("base-arena"),
  residual: document.getElementById("residual-arena"),
  final: document.getElementById("final-arena"),
  memory: document.getElementById("memory-arena"),
};
const timeline = document.getElementById("timeline");
const view = document.getElementById("view-select");
const decodeSel = document.getElementById("decode-select");
const temperature = document.getElementById("temperature");

function paint(grid, values, kind) {
  grid.innerHTML = "";
  for (let i = 0; i < 576; i++) {
    const v = Number(values && values[i] || 0);
    const z = kind === "residual" ? Math.min(1, Math.abs(v) * .45) : Math.max(0, Math.min(1, v));
    const c = document.createElement("div");
    c.className = "cell";
    c.title = "tile " + i + " · " + v.toFixed(4);
    c.style.background = kind === "residual"
      ? (v >= 0 ? "rgba(94,234,212," + (0.06 + z) + ")" : "rgba(251,113,133," + (0.06 + z) + ")")
      : "rgba(94,234,212," + (0.06 + .9 * z) + ")";
    grid.appendChild(c);
  }
}
function decodeValues(values) {
  const probs = values.map(x => Math.max(Number(x || 0), 1e-9));
  const out = Array(576).fill(0);
  if (decodeSel.value === "expected") return probs;
  if (decodeSel.value === "argmax") { out[probs.indexOf(Math.max(...probs))] = 1; return out; }
  const temp = Math.max(Number(temperature.value), .05);
  const weights = probs.map(x => Math.exp(Math.log(x) / temp));
  const sum = weights.reduce((a, x) => a + x, 0);
  let r = Math.random() * sum;
  for (let i = 0; i < weights.length; i++) { r -= weights[i]; if (r <= 0) { out[i] = 1; break; } }
  return out;
}
function renderArena() {
  const e = examples[Number(timeline.value)] || {};
  const checked = [...document.querySelectorAll("[data-channel]")].map(x => x.checked);
  const m = e.memory || [];
  const filtered = Array.from({length: 576}, (_, i) =>
    Array.isArray(m[i]) ? m[i].reduce((a, x, j) => a + (checked[j] ? Number(x || 0) : 0), 0) : 0);
  const mode = view.value;
  const decoded = decodeValues(e.final || []);
  const shown = mode === "base" ? (e.base || []) : mode === "residual" ? (e.residual || []) : mode === "memory" ? filtered : decoded;
  paint(grids.base, e.base || [], "base");
  paint(grids.residual, e.residual || [], "residual");
  paint(grids.final, shown, mode === "residual" ? "residual" : "final");
  paint(grids.memory, filtered, "memory");
  document.getElementById("timeline-label").textContent = e.label || ("sample " + (Number(timeline.value) + 1));
  document.getElementById("arena-caption").textContent =
    "Mode: " + mode + " · decoder: " + decodeSel.value + " · temperature: " + temperature.value +
    " · aligned memory intensity: " + filtered.reduce((a, x) => a + x, 0).toFixed(3);
}
timeline.oninput = renderArena;
view.onchange = renderArena;
decodeSel.onchange = renderArena;
temperature.oninput = renderArena;
document.querySelectorAll("[data-channel]").forEach(x => x.onchange = renderArena);
document.getElementById("animate").onclick = () => {
  let i = 0;
  clearInterval(window.arenaTimer);
  window.arenaTimer = setInterval(() => {
    if (!examples.length) return;
    timeline.value = String(i++ % examples.length);
    renderArena();
  }, 500);
};
renderArena();
</script>
</body>
</html>
"""

    # Gate accounting.
    decided = [k for k, _ in GATE_ORDER if gates.get(k) in (True, False)]
    passed = [k for k in decided if gates.get(k) is True]
    gates_passed = len(passed)
    gates_decided = len(decided)

    v61_train = float((v61.get("data") or {}).get("train_samples") or 0)
    action_ratio = 100.0 * train_samples / v61_train if v61_train else 0.0
    trainable_share = f"{100.0 * trainable / params:.1f}%" if params else "—"
    peak_vram = max((float(row.get("gpu_memory_mb", 0.0)) for row in progress), default=0.0)

    replacements = {
        "@@BASE_STYLES@@": _base_styles(),
        "@@CHART_SCRIPT@@": _chart_script(),
        "@@TITLE@@": title,
        "@@MODEL_NAME@@": model_name,
        "@@MODEL_VERSION@@": html.escape(str(report.get("model_version", "7.0.0"))),
        "@@EPOCHS@@": str(len(history) or 1),
        "@@DEVICE@@": html.escape(str(compute.get("device", "cuda"))),
        "@@CREATED@@": html.escape(created),
        "@@CREATED_ISO@@": html.escape(str(report.get("created_at", created))),
        "@@BATTLES@@": f"{battles_total:,}",
        "@@TRAIN_ACTIONS@@": f"{train_samples:,}",
        "@@PARAMS@@": f"{params:,}",
        "@@TRAINABLE@@": f"{trainable:,}",
        "@@TRAINABLE_SHARE@@": trainable_share,
        "@@SECONDS@@": _fmt_float(report.get("seconds"), 1),
        "@@VERDICT@@": verdict_text,
        "@@TILE_TOP1@@": _fmt_pct(tile_top1),
        "@@T1_DELTA@@": _pp(tile_top1, base_tile_top1),
        "@@T1CLS@@": _metric_class(tile_top1, base_tile_top1),
        "@@TILE_TOP5@@": _fmt_pct(tile_top5),
        "@@T5_DELTA@@": _pp(tile_top5, base_tile_top5),
        "@@T5CLS@@": _metric_class(tile_top5, base_tile_top5),
        "@@XY@@": _fmt_float(xy_mae, 0),
        "@@XY_DELTA@@": _delta_units(xy_mae, base_xy),
        "@@XYCLS@@": _metric_class(xy_mae, base_xy, lower=True),
        "@@GATE@@": _fmt_float(oracle.get("arena_gate"), 3),
        "@@DECISION_ROWS@@": decision_rows,
        "@@ACTION_RATIO@@": f"{action_ratio:.1f}",
        "@@NLL_DELTA@@": _fmt_float((tile_nll or 0) - (base_tile_nll or 0) if tile_nll and base_tile_nll else None, 3),
        "@@ABLATION_CHART@@": ablation_chart,
        "@@CARD_CHART@@": card_chart,
        "@@GATE_GRID@@": gate_grid,
        "@@GATES_PASSED@@": str(gates_passed),
        "@@GATES_DECIDED@@": str(gates_decided),
        "@@GATES_PASSED_CLS@@": "up" if gates_passed and gates_passed == gates_decided else ("down" if gates_passed < gates_decided else "warn"),
        "@@ORACLE_NLL@@": f"{oracle_nll:.3f}",
        "@@SHUFFLED_NLL@@": f"{shuffled_nll:.3f}",
        "@@DISABLED_NLL@@": f"{disabled_nll:.3f}",
        "@@NEUTRAL_NLL@@": f"{neutral_nll:.3f}",
        "@@NEUTRAL_DELTA@@": f"{neutral_nll - oracle_nll:+.3f}",
        "@@MEMORY_DELTA@@": f"{max(shuffled_nll, disabled_nll) - oracle_nll:+.3f}",
        "@@SHUFFLE_LOSS@@": f"{shuffled_nll - oracle_nll:+.3f}",
        "@@HUMAN_RX@@": _fmt_float(human_rx, 2),
        "@@HUMAN_RY@@": _fmt_float(human_ry, 2),
        "@@EXPECTED_RX@@": _fmt_float(expected_rx, 2),
        "@@EXPECTED_RY@@": _fmt_float(expected_ry, 2),
        "@@ARG_X_STD@@": _fmt_float(argmax_tile.get("x_std"), 3),
        "@@HUMAN_X_STD@@": _fmt_float(human.get("x_std"), 3),
        "@@EXPECTED_X_STD@@": _fmt_float(expected.get("x_std"), 3),
        "@@REACTION_ROWS@@": reaction_rows,
        "@@SPLIT_ROWS@@": split_rows,
        "@@BATCH@@": str(compute.get("batch_size", "—")),
        "@@LR@@": str(compute.get("learning_rate", "—")),
        "@@D_MODEL@@": str(compute.get("d_model", "—")),
        "@@LAYERS@@": str(compute.get("num_layers", "—")),
        "@@CONTEXT@@": str(compute.get("max_context", "—")),
        "@@MAX_SAMPLES@@": str(compute.get("max_samples_per_battle", "—")),
        "@@MANIFEST@@": html.escape(str(split_manifest)),
        "@@WARMSTART@@": html.escape(str(warmstart)),
        "@@GIT@@": html.escape(git_revision) + (" · dirty worktree" if dirty else " · clean worktree"),
        "@@VRAM@@": _fmt_float(peak_vram, 0),
        "@@COMMAND@@": html.escape(exact_command),
        "@@CHANNEL_TABLE@@": channel_table,
        "@@CHANNEL_CHECKS@@": channel_checks,
        "@@MAX_EXAMPLE@@": str(max(len(examples) - 1, 0)),
        "@@CURVE_JSON@@": _json_script(curves),
        "@@DECODE_JSON@@": _json_script(heatmap_decode),
        "@@EXAMPLES_JSON@@": _json_script(examples),
        "@@LESSONS@@": lesson_html,
    }
    body = template
    for token, value in replacements.items():
        body = body.replace(token, value)
    output_path.write_text(body, encoding="utf-8")
    return output_path
