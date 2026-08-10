"""Decision-oriented HTML report for the policy-bc-v6 heatmap experiment."""

from __future__ import annotations

import html
import json
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


def _load(path: str | Path) -> dict[str, Any]:
    path = Path(path)
    if not path.exists():
        return {}
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def _pp(value: float | None, baseline: float | None) -> str:
    if value is None or baseline is None:
        return "—"
    delta = 100.0 * (float(value) - float(baseline))
    sign = "+" if delta > 0 else ""
    return f"{sign}{delta:.1f} pp"


def _delta_units(value: float | None, baseline: float | None) -> str:
    if value is None or baseline is None:
        return "—"
    delta = float(value) - float(baseline)
    sign = "+" if delta > 0 else ""
    return f"{sign}{delta:,.0f}"


def _metric_class(value: float | None, baseline: float | None, *, lower: bool = False) -> str:
    if value is None or baseline is None:
        return "flat"
    improved = value < baseline if lower else value > baseline
    return "up" if improved else "down" if value != baseline else "flat"


def _probe_path(model_dir: Path, report_dir: Path) -> Path:
    suffix = model_dir.name.removeprefix("policy_bc_")
    candidates = [
        report_dir / f"blind_spot_probe_{suffix}.json",
        report_dir / "blind_spot_probe_v6_full.json",
        report_dir / "blind_spot_probe_v6_tune.json",
    ]
    return next((path for path in candidates if path.exists()), candidates[0])


def render_policy_v6_report(
    model_dir: str | Path = "models/policy_bc_v6_full",
    old_model_dir: str | Path | None = None,
    v5_model_dir: str | Path | None = None,
    probe_path: str | Path | None = None,
    output_path: str | Path = "reports/policy_bc_v6_full.html",
) -> Path:
    model_dir = Path(model_dir)
    output_path = Path(output_path)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    report = _load(model_dir / "report.json")
    if not report:
        raise FileNotFoundError(f"Missing {model_dir / 'report.json'}")

    model_root = model_dir.parent
    v41 = _load(Path(old_model_dir) / "report.json") if old_model_dir else _load(model_root / "policy_bc_v4.1" / "report.json")
    v5 = _load(Path(v5_model_dir) / "report.json") if v5_model_dir else _load(model_root / "policy_bc_v5" / "report.json")
    probe = _load(probe_path or _probe_path(model_dir, output_path.parent))
    probe_v41 = _load(output_path.parent / "blind_spot_probe_v1.json")
    probe_v5 = _load(output_path.parent / "blind_spot_probe_v5.json")

    compute = report.get("compute") or {}
    data = report.get("data") or {}
    test = report.get("test") or {}
    history = report.get("history") or []
    rollouts = report.get("rollouts") or {}
    lessons = report.get("lessons") or []
    created = _report_timestamp(model_dir, "report.json", "best_model.pt")
    v41_test = v41.get("test") or {}
    v5_test = v5.get("test") or {}
    reference = v5_test or v41_test

    spread = probe.get("placement_spread") or {}
    global_spread = spread.get("global") or {}
    decode = spread.get("heatmap_decode") or {}
    expected = decode.get("expected_xy") or {
        "mean_l1_units": global_spread.get("mean_l1_units"),
        "x_std": global_spread.get("model_x_std"),
        "y_std": global_spread.get("model_y_std"),
    }
    argmax_tile = decode.get("argmax_tile") or {}
    human = decode.get("human") or {
        "x_std": global_spread.get("human_x_std"),
        "y_std": global_spread.get("human_y_std"),
    }

    model_rows: list[tuple[str, dict[str, Any], dict[str, Any], str]] = []
    if v41:
        model_rows.append(("v4.1", v41_test, v41.get("data") or {}, "incumbent"))
    if v5:
        model_rows.append(("v5", v5_test, v5.get("data") or {}, "style-trained"))
    candidate_label = "v6.1" if str(report.get("model_version", "")).startswith("6.1") else "v6"
    model_rows.append((candidate_label, test, data, "frozen-head isolation" if candidate_label == "v6.1" else "heatmap experiment"))

    comparison_rows = ""
    for label, metrics, model_data, note in model_rows:
        is_v6 = label == "v6"
        comparison_rows += (
            f'<tr class="{"candidate" if is_v6 else ""}">'
            f'<td><strong>{html.escape(label)}</strong><small>{html.escape(note)}</small></td>'
            f'<td>{int(model_data.get("train_samples", 0)):,}</td>'
            f'<td>{_fmt_pct(metrics.get("slot_top1"))}</td>'
            f'<td>{_fmt_pct(metrics.get("zone_acc"))}</td>'
            f'<td>{_fmt_float(metrics.get("xy_mae"), 0)}</td>'
            f'<td>{_fmt_pct(metrics.get("tile_class_acc"))}</td>'
            f'<td>{_fmt_float(metrics.get("timing_mae"), 2)}s</td>'
            "</tr>"
        )

    splits = data.get("splits") or []
    split_rows = "".join(
        f"<tr><td>{html.escape(str(row.get('split', '—')))}</td>"
        f"<td>{int(row.get('battles', 0)):,}</td>"
        f"<td>{_fmt_pct(row.get('team_win_rate'))}</td>"
        f"<td>{_fmt_float(row.get('mean_events'), 1)}</td></tr>"
        for row in splits
    )

    oracle = (probe.get("ablations") or {}).get("oracle_full_opponent_deck") or {}
    revealed = (probe.get("ablations") or {}).get("revealed_opponent_cards_only") or {}
    no_threat = (probe.get("ablations") or {}).get("no_recent_threat_vector") or {}
    neutral_xy = (probe.get("ablations") or {}).get("neutralized_history_xy") or {}
    winner = (probe.get("outcome_slice") or {}).get("winner_actions") or {}
    loser = (probe.get("outcome_slice") or {}).get("loser_actions") or {}

    def spatial_drop(one_probe: dict[str, Any]) -> float | None:
        ablations = one_probe.get("ablations") or {}
        full = ablations.get("oracle_full_opponent_deck") or {}
        neutral = ablations.get("neutralized_history_xy") or {}
        if full.get("zone_acc") is None or neutral.get("zone_acc") is None:
            return None
        return 100.0 * (float(full["zone_acc"]) - float(neutral["zone_acc"]))

    spatial_rows = "".join(
        f"<tr><td>{label}</td><td>{'—' if drop is None else f'{drop:.1f} pp'}</td>"
        f'<td><div class="meter"><i style="width:{max(2.0, min(100.0, (drop or 0) * 7.5)):.1f}%"></i></div></td></tr>'
        for label, drop in (
            ("v4.1", spatial_drop(probe_v41)),
            ("v5", spatial_drop(probe_v5)),
            ("v6", spatial_drop(probe)),
        )
    )

    v5_actions = float((v5.get("data") or {}).get("train_samples") or 0)
    v6_actions = float(data.get("train_samples") or 0)
    action_ratio = 100.0 * v6_actions / v5_actions if v5_actions else 0.0
    hide_prob = float(compute.get("hide_opponent_prob") or 0.0)
    masking_status = "active" if hide_prob > 0 else "implemented · off in this control"
    epoch_count = len(history)
    verdict = "DO NOT PROMOTE"

    lesson_html = "".join(f"<li>{html.escape(str(item))}</li>" for item in lessons)
    if not lesson_html:
        lesson_html = "<li>The heatmap hypothesis remains an offline diagnostic.</li>"

    template = r"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>PolicyBC @@MODEL_VERSION@@ — heatmap experiment report</title>
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
  .hero h1 { font-size:clamp(2.55rem,5.7vw,4.55rem); line-height:.98; max-width:13ch; margin:18px 0; letter-spacing:-.055em; }
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
  .decision-grid { display:grid; grid-template-columns:minmax(0,1.55fr) minmax(290px,.45fr); gap:20px; }
  .compare-table td:first-child small { display:block; color:#64748b; font-size:.72rem; margin-top:2px; }
  .compare-table tr.candidate { background:rgba(167,139,250,.07); }
  .compare-table tr.candidate td { color:#ddd6fe; }
  .compare-table tr.candidate td:first-child { padding-left:10px; border-left:2px solid var(--violet); }
  .budget-stack { display:grid; gap:14px; }
  .budget-stat { padding-bottom:13px; border-bottom:1px solid var(--line-soft); }
  .budget-stat:last-child { border-bottom:0; padding-bottom:0; }
  .budget-stat strong { display:block; font-size:1.3rem; margin-top:4px; }
  .budget-note { color:#fcd34d; font-size:.82rem; padding:11px 12px; background:rgba(251,191,36,.08); border:1px solid rgba(251,191,36,.24); border-radius:11px; }

  .decode-grid { display:grid; grid-template-columns:minmax(340px,.8fr) minmax(0,1.2fr); gap:24px; }
  .arena-shell { display:grid; grid-template-columns:180px 1fr; gap:24px; align-items:center; }
  .arena { width:180px; height:320px; border-radius:16px; display:block; background:#0a1720; box-shadow:inset 0 0 0 1px rgba(148,163,184,.24),0 18px 40px rgba(0,0,0,.28); }
  .arena-note { font-size:.74rem; color:#64748b; }
  .decode-toggle { display:inline-flex; gap:5px; padding:4px; border-radius:999px; background:rgba(148,163,184,.09); border:1px solid rgba(148,163,184,.16); }
  .decode-toggle button { border:0; border-radius:999px; padding:8px 13px; background:transparent; color:#94a3b8; font:inherit; font-size:.8rem; cursor:pointer; }
  .decode-toggle button.active { background:#172033; color:#f1f5f9; box-shadow:0 0 0 1px #334155; }
  .decode-kpis { display:grid; grid-template-columns:repeat(2,minmax(0,1fr)); gap:10px; margin:18px 0; }
  .decode-kpi { border:1px solid var(--line); border-radius:12px; padding:12px; }
  .decode-kpi span { display:block; font-size:.65rem; color:#64748b; text-transform:uppercase; letter-spacing:.09em; }
  .decode-kpi strong { display:block; font-size:1.25rem; margin-top:4px; }
  .finding { display:flex; gap:12px; padding:14px 0; border-bottom:1px solid var(--line-soft); }
  .finding:last-child { border-bottom:0; }
  .finding-mark { width:9px; height:9px; border-radius:999px; margin-top:7px; flex:0 0 auto; background:var(--amber); box-shadow:0 0 18px currentColor; }
  .finding strong { display:block; color:#e2e8f0; font-size:.9rem; }
  .finding p { margin:3px 0 0; font-size:.8rem; }

  .blind-grid { display:grid; grid-template-columns:repeat(3,minmax(0,1fr)); gap:14px; }
  .blind-card { padding:18px; border:1px solid rgba(148,163,184,.16); border-radius:16px; background:var(--panel); }
  .blind-card .big { display:block; font-size:1.65rem; font-weight:760; margin:8px 0 6px; }
  .blind-card p { font-size:.8rem; margin:0; }
  .meter { width:100%; height:7px; border-radius:999px; background:rgba(148,163,184,.10); overflow:hidden; }
  .meter i { display:block; height:100%; border-radius:inherit; background:linear-gradient(90deg,var(--cyan),var(--violet)); }
  .spatial-table td:nth-child(2) { width:84px; }

  .wire { display:grid; grid-template-columns:1fr 34px 1fr; gap:12px; align-items:stretch; }
  .wire-col { padding:17px; border:1px solid rgba(148,163,184,.16); border-radius:16px; background:var(--panel); }
  .wire-title { color:#94a3b8; letter-spacing:.12em; text-transform:uppercase; font-size:.66rem; margin-bottom:12px; }
  .wire-box { padding:10px 12px; border:1px solid rgba(34,211,238,.28); background:rgba(34,211,238,.09); border-radius:10px; text-align:center; font-size:.82rem; margin-bottom:8px; }
  .wire-box.new { border-color:rgba(167,139,250,.58); background:rgba(167,139,250,.10); }
  .wire-box.off { border-style:dashed; color:#94a3b8; }
  .wire-arrow { align-self:center; height:2px; background:linear-gradient(90deg,var(--cyan),var(--violet)); }
  .heat-mini { height:42px; border-radius:8px; margin-bottom:8px; border:1px solid rgba(167,139,250,.36); background-color:rgba(167,139,250,.08); background-image:linear-gradient(rgba(148,163,184,.09) 1px,transparent 1px),linear-gradient(90deg,rgba(148,163,184,.09) 1px,transparent 1px),radial-gradient(circle at 62% 38%,rgba(251,191,36,.85),rgba(167,139,250,.36) 16%,transparent 42%); background-size:10px 10px,10px 10px,100% 100%; }

  .curve-grid { display:grid; grid-template-columns:repeat(2,minmax(0,1fr)); gap:22px; }
  svg.chart { min-height:300px; aspect-ratio:1000/440; }
  .film { display:flex; gap:14px; flex-wrap:wrap; align-items:center; margin-top:18px; }
  .film button { border:0; border-radius:999px; padding:9px 16px; background:linear-gradient(120deg,var(--cyan),var(--violet)); color:#061018; font:inherit; font-weight:760; cursor:pointer; }
  .film input { flex:1; min-width:210px; accent-color:var(--violet); }
  .film-readout { flex-basis:100%; font-size:.82rem; color:#94a3b8; }
  .rollout-grid { display:grid; grid-template-columns:repeat(4,minmax(0,1fr)); gap:12px; }
  .rollout-card { padding:14px; border-radius:13px; border:1px solid var(--line); background:rgba(15,23,42,.55); }
  .rollout-card strong { display:block; font-size:1.35rem; margin-top:5px; }
  .lock { display:flex; gap:12px; align-items:flex-start; margin-top:18px; padding:14px; border:1px solid rgba(251,113,133,.32); background:rgba(251,113,133,.07); border-radius:13px; }
  .lock-icon { font-size:1.25rem; }
  .lock p { margin:0; font-size:.82rem; }
  .footnote { color:#64748b; font-size:.76rem; }
  abbr[title] { text-decoration:underline dotted; text-underline-offset:3px; cursor:help; }

  @media(max-width:920px) {
    .hero-grid,.decision-grid,.decode-grid { grid-template-columns:1fr; }
    .hero-scores,.blind-grid,.rollout-grid { grid-template-columns:repeat(2,minmax(0,1fr)); }
    .wire { grid-template-columns:1fr; } .wire-arrow { width:2px;height:24px;margin:0 auto; }
    .section-head { display:block; } .section-head p { text-align:left; margin-top:8px; }
  }
  @media(max-width:620px) {
    main { padding:24px 16px 60px; }
    .hero-scores,.blind-grid,.rollout-grid,.curve-grid { grid-template-columns:1fr; }
    .arena-shell { grid-template-columns:1fr; } .arena { margin:0 auto; }
    .panel { padding:16px; overflow-x:auto; }
  }
  </style>
</head>
<body>
<main>
  <header class="hero">
    <div class="badge-row">
      <span class="badge">@@MODEL_NAME@@</span><span class="badge">heatmap experiment</span>
      <span class="badge">@@EPOCH_COUNT@@ epochs</span><span class="badge">@@DEVICE@@</span>
      <span class="badge">offline only</span>
    </div>
    <div class="hero-grid">
      <div>
        <h1>The heatmap learned modes. The policy did not win.</h1>
        <p class="hero-sub">v6 asks a narrow question: can a card-conditioned 18×32 tile distribution fix point-regression collapse? The answer is mixed. A mode decode recovers human-like horizontal spread, but action, zone and coordinate accuracy all fall under this cheap training budget.</p>
        <p class="meta">Created @@CREATED@@ · @@BATTLES@@ battles · @@TRAIN_ACTIONS@@ train actions · @@PARAMS@@ parameters · @@SECONDS@@s</p>
      </div>
      <aside class="verdict-card">
        <span class="eyebrow">experiment verdict</span>
        <div class="verdict mono">@@VERDICT@@</div>
        <p>Keep v4.1/v5 as incumbents. Preserve the heatmap code as a representation experiment; do not spend a live-game run on this checkpoint.</p>
      </aside>
    </div>
    <div class="hero-scores">
      <div class="score"><span class="score-label">Slot top-1</span><span class="score-value mono down">@@SLOT@@</span><span class="score-delta">@@SLOT_DELTA@@ vs v5</span></div>
      <div class="score"><span class="score-label">Zone accuracy</span><span class="score-value mono down">@@ZONE@@</span><span class="score-delta">@@ZONE_DELTA@@ vs v5</span></div>
      <div class="score"><span class="score-label">XY MAE</span><span class="score-value mono down">@@XY@@</span><span class="score-delta">@@XY_DELTA@@ vs v5 · lower is better</span></div>
      <div class="score"><span class="score-label">Tile mode x-spread</span><span class="score-value mono up">@@ARG_X_STD@@</span><span class="score-delta">human @@HUMAN_X_STD@@ · normalized σ</span></div>
    </div>
  </header>

  <section class="report-section">
    <div class="section-head"><h2>Did @@CANDIDATE_LABEL@@ beat the incumbents?</h2><p>No. This is the decision table the generic report was missing.</p></div>
    <div class="decision-grid">
      <div class="panel">
        <table class="compare-table">
          <thead><tr><th>Model</th><th>Train actions</th><th>Slot@1</th><th>Zone</th><th><abbr title="Mean absolute X+Y error in replay API coordinate units">XY MAE</abbr></th><th>Exact tile</th><th>Timing</th></tr></thead>
          <tbody>@@COMPARISON_ROWS@@</tbody>
        </table>
      </div>
      <aside class="panel budget-stack">
        <div class="budget-stat"><span class="eyebrow">same battle cut</span><strong class="mono">@@BATTLES@@</strong><span class="caption">matches v5 battle count</span></div>
        <div class="budget-stat"><span class="eyebrow">action-window budget</span><strong class="mono">@@ACTION_RATIO@@%</strong><span class="caption">of v5 train actions</span></div>
        <div class="budget-stat"><span class="eyebrow">opponent masking</span><strong>@@MASKING_STATUS@@</strong><span class="caption">probability @@HIDE_PROB@@</span></div>
        <div class="budget-note">Fairness caveat: same battles does not mean equal optimization. v6 kept eight windows per battle and ran only two epochs; this is a cheap falsification test, not a converged architecture contest.</div>
      </aside>
    </div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>One distribution, two decoders</h2><p>Expected XY minimizes average error but collapses toward the center. Tile argmax restores spread, then pays for picking the wrong mode.</p></div>
    <div class="decode-grid">
      <div class="panel arena-shell">
        <svg class="arena" viewBox="0 0 180 320" role="img" aria-label="Arena spread comparison">
          <defs><pattern id="grid" width="10" height="10" patternUnits="userSpaceOnUse"><path d="M10 0H0V10" fill="none" stroke="#203040" stroke-width=".55"/></pattern></defs>
          <rect width="180" height="320" rx="16" fill="#0b1923"/><rect width="180" height="320" rx="16" fill="url(#grid)"/>
          <rect y="151" width="180" height="18" fill="#0e7490" opacity=".65"/>
          <rect x="35" y="149" width="28" height="22" rx="3" fill="#94a3b8" opacity=".72"/><rect x="117" y="149" width="28" height="22" rx="3" fill="#94a3b8" opacity=".72"/>
          <line x1="90" y1="0" x2="90" y2="320" stroke="#334155" stroke-dasharray="3 5"/>
          <ellipse id="humanSpread" cx="90" cy="160" rx="@@HUMAN_RX@@" ry="@@HUMAN_RY@@" fill="rgba(34,211,238,.06)" stroke="#22d3ee" stroke-width="2"><title>Human global spread envelope, centered only for shape comparison</title></ellipse>
          <ellipse id="modelSpread" cx="90" cy="160" rx="@@EXPECTED_RX@@" ry="@@EXPECTED_RY@@" fill="rgba(251,191,36,.08)" stroke="#fbbf24" stroke-width="2"><title>Model spread envelope</title></ellipse>
          <circle cx="90" cy="160" r="3" fill="#e2e8f0"/>
        </svg>
        <div>
          <div class="eyebrow">centered spread envelope</div>
          <p class="arena-note">Ellipse radii are global standard deviations mapped into the 18×32 arena. Centers are aligned intentionally; this compares diversity, not mean position.</p>
          <p class="arena-note"><span class="dot" style="background:#22d3ee"></span>human <span class="dot" style="background:#fbbf24;margin-left:12px"></span>model</p>
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
        <div class="finding"><span class="finding-mark" style="background:#34d399"></span><div><strong>Representation win</strong><p>Argmax x-spread @@ARG_X_STD@@ almost matches human @@HUMAN_X_STD@@. The heatmap can express multiple placement modes.</p></div></div>
        <div class="finding"><span class="finding-mark" style="background:#fb7185"></span><div><strong>Decision loss</strong><p>Argmax coordinate error rises to @@ARGMAX_MAE@@ API units. The model recovers diversity without knowing which mode is correct now.</p></div></div>
        <div class="finding"><span class="finding-mark"></span><div><strong>Deployment implication</strong><p>Sampling this distribution may look more human, but it is not evidence of better play. A state-aware selector is still missing.</p></div></div>
      </div>
    </div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>Blind-spot lab</h2><p>Frozen checkpoint, post-training replay slice. These are perturbation tests, not gameplay scores.</p></div>
    <div class="blind-grid">
      <div class="blind-card"><span class="eyebrow">oracle deck → revealed only</span><span class="big mono">@@DECK_SLOT_DELTA@@</span><p>Slot accuracy barely moves. Knowing the unrevealed opponent deck is not the missing capability.</p></div>
      <div class="blind-card"><span class="eyebrow">neutralize history XY</span><span class="big mono">@@HISTORY_ZONE_DELTA@@</span><p>Zone accuracy loss is much smaller than v4.1/v5. v6 is underusing spatial history.</p></div>
      <div class="blind-card"><span class="eyebrow">winner − loser actions</span><span class="big mono">@@WINNER_SLOT_DELTA@@</span><p>A small imitation gap does not establish that chosen actions improve outcomes.</p></div>
    </div>
    <div class="decision-grid" style="margin-top:20px">
      <div class="panel">
        <h2>Spatial-history sensitivity</h2>
        <p class="caption">Zone accuracy drop after replacing historical coordinates with the arena center. Larger means the policy was using spatial context.</p>
        <table class="spatial-table"><thead><tr><th>Model</th><th>Zone drop</th><th>Relative</th></tr></thead><tbody>@@SPATIAL_ROWS@@</tbody></table>
      </div>
      <div class="panel">
        <h2>What the perturbations say</h2>
        <div class="finding"><span class="finding-mark" style="background:#22d3ee"></span><div><strong>Deck oracle is a distraction</strong><p>Revealed-only deck changes zone by @@DECK_ZONE_DELTA@@ and XY error by @@DECK_XY_DELTA@@ units.</p></div></div>
        <div class="finding"><span class="finding-mark" style="background:#a78bfa"></span><div><strong>Threat vector still helps</strong><p>Removing it changes zone by @@THREAT_ZONE_DELTA@@ and XY by @@THREAT_XY_DELTA@@ units.</p></div></div>
      </div>
    </div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>What changed in v6</h2><p>The model keeps v4's causal action trunk and changes only the placement representation plus optional deck masking.</p></div>
    <div class="wire">
      <div class="wire-col">
        <div class="wire-title">kept from v4</div>
        <div class="wire-box">card embeddings + causal GRU</div><div class="wire-box">cycle + threat features</div><div class="wire-box">card-conditioned slot and zone heads</div>
      </div>
      <div class="wire-arrow"></div>
      <div class="wire-col">
        <div class="wire-title">v6 experiment</div>
        <div class="heat-mini" title="18 by 32 categorical tile distribution"></div>
        <div class="wire-box new">576-way placement cross-entropy</div><div class="wire-box new">expected XY compatibility decode</div><div class="wire-box off">unrevealed-card masking · @@MASKING_STATUS@@</div>
      </div>
    </div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>Training film</h2><p>Two epochs are enough to reject promotion, not enough to claim the architecture cannot converge.</p></div>
    <div class="curve-grid">
      <div class="panel"><h2>Loss</h2><svg class="chart" id="lossChart"></svg><div class="legend"></div></div>
      <div class="panel"><h2>Validation heads</h2><svg class="chart" id="metricChart"></svg><div class="legend"></div></div>
    </div>
    <div class="film"><button type="button" id="filmPlay">Play epoch film</button><input type="range" id="filmScrub" min="0" max="0" value="0" step="1"><div class="film-readout mono" id="filmReadout">—</div></div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>Compute, data and rollout context</h2><p>Enough detail to reproduce the cheap control without treating realism as gameplay.</p></div>
    <div class="decision-grid">
      <div class="panel">
        <table><thead><tr><th>Split</th><th>Battles</th><th>Win rate</th><th>Mean events</th></tr></thead><tbody>@@SPLIT_ROWS@@</tbody></table>
        <p class="footnote" style="margin-top:14px">batch @@BATCH@@ · lr @@LR@@ · d_model @@D_MODEL@@ · @@LAYERS@@ GRU layers · max context @@CONTEXT@@ · max @@MAX_SAMPLES@@ windows/battle · heatmap loss weight @@TILE_WEIGHT@@</p>
      </div>
      <div class="panel">
        <div class="rollout-grid">
          <div class="rollout-card"><span class="eyebrow">real prefix</span><strong class="mono">@@REAL_SCORE@@</strong></div>
          <div class="rollout-card"><span class="eyebrow">v6 rollout</span><strong class="mono">@@POLICY_SCORE@@</strong></div>
          <div class="rollout-card"><span class="eyebrow">easy random</span><strong class="mono">@@EASY_SCORE@@</strong></div>
          <div class="rollout-card"><span class="eyebrow">medium random</span><strong class="mono">@@MEDIUM_SCORE@@</strong></div>
        </div>
        <p class="caption">Frozen realism scorer P(real), n=@@ROLLOUT_N@@. It measures replay likeness—not towers, damage, legality or winning.</p>
        <div class="lock"><span class="lock-icon">⌁</span><p><strong>Live gate locked.</strong> Offline metric thresholds happen to pass, but this experiment is explicitly marked offline-only because its policy metrics regress and arena state is still absent.</p></div>
      </div>
    </div>
  </section>

  <section class="report-section">
    <div class="section-head"><h2>Lessons and next decision</h2><p>The honest outcome is useful: the representation hypothesis partly worked, the policy hypothesis did not.</p></div>
    <div class="decision-grid">
      <div class="panel"><ul class="lessons">@@LESSONS@@</ul></div>
      <div class="panel"><span class="eyebrow">recommended next move</span><div class="verdict" style="color:#e2e8f0">Add state before scale.</div><p>Vectorize the replay-window pipeline only if we want an equal-action-budget rerun. The larger modeling gain is a latent arena-state tracker that can choose among heatmap modes using troops, towers, elixir and current threats.</p></div>
    </div>
  </section>
</main>
<script>
@@CHART_SCRIPT@@
const history = @@HISTORY_JSON@@;
const decode = @@DECODE_JSON@@;

function drawTraining(limit) {
  const rows = history.slice(0, limit + 1);
  if (!rows.length) return;
  const labels = rows.map(row => "epoch " + row.epoch);
  renderLineChart("lossChart", [
    {color:"#60a5fa",label:"Train loss",values:rows.map(row=>row.train_loss)},
    {color:"#34d399",label:"Val loss",values:rows.map(row=>row.val_loss)},
  ], labels, {yFormat:"float"});
  renderLineChart("metricChart", [
    {color:"#fbbf24",label:"Slot top-1",values:rows.map(row=>row.val_slot_top1)},
    {color:"#34d399",label:"Zone",values:rows.map(row=>row.val_zone_acc)},
    {color:"#a78bfa",label:"Exact tile",values:rows.map(row=>row.val_tile_class_acc || 0)},
  ], labels, {yFormat:"percent"});
}

const scrub = document.getElementById("filmScrub");
const play = document.getElementById("filmPlay");
const readout = document.getElementById("filmReadout");
let filmTimer = null;
let filmPlaying = false;
if (history.length) {
  scrub.max = String(history.length - 1); scrub.value = String(history.length - 1);
  function setFrame(index) {
    const row = history[index]; drawTraining(index);
    readout.textContent = "epoch " + row.epoch + " · slot " + (100*row.val_slot_top1).toFixed(1) + "% · zone " + (100*row.val_zone_acc).toFixed(1) + "% · exact tile " + (100*(row.val_tile_class_acc||0)).toFixed(1) + "%";
  }
  function setPlaying(next) {
    filmPlaying = next; play.textContent = next ? "Pause" : "Play epoch film";
    if (filmTimer) window.clearInterval(filmTimer);
    filmTimer = next ? window.setInterval(() => { const nextFrame=(Number(scrub.value)+1)%history.length; scrub.value=String(nextFrame); setFrame(nextFrame); }, 850) : null;
  }
  scrub.addEventListener("input",()=>{setPlaying(false);setFrame(Number(scrub.value));});
  play.addEventListener("click",()=>setPlaying(!filmPlaying));
  setFrame(history.length - 1);
}

const decodeButtons = document.querySelectorAll("#decodeToggle button");
const modelSpread = document.getElementById("modelSpread");
function setDecode(mode) {
  const row = mode === "argmax" ? (decode.argmax_tile || {}) : (decode.expected_xy || {});
  const rx = 180 * Number(row.x_std || 0); const ry = 320 * Number(row.y_std || 0);
  modelSpread.setAttribute("rx", String(rx)); modelSpread.setAttribute("ry", String(ry));
  modelSpread.setAttribute("stroke", mode === "argmax" ? "#a78bfa" : "#fbbf24");
  document.getElementById("decodeMae").textContent = Number(row.mean_l1_units || 0).toLocaleString(undefined,{maximumFractionDigits:0});
  document.getElementById("decodeSpread").textContent = Number(row.x_std || 0).toFixed(3) + " / " + Number(row.y_std || 0).toFixed(3);
  document.getElementById("decodeTop1").textContent = row.tile_top1 == null ? "n/a" : (100*row.tile_top1).toFixed(1) + "%";
  document.getElementById("decodeTop5").textContent = row.tile_top5 == null ? "n/a" : (100*row.tile_top5).toFixed(1) + "%";
  decodeButtons.forEach(button=>button.classList.toggle("active",button.dataset.mode===mode));
}
decodeButtons.forEach(button=>button.addEventListener("click",()=>setDecode(button.dataset.mode)));
setDecode("expected");
</script>
</body>
</html>
"""

    replacements = {
        "@@BASE_STYLES@@": _base_styles(),
        "@@CHART_SCRIPT@@": _chart_script(),
        "@@MODEL_NAME@@": html.escape(str(report.get("model_name", "policy-bc-v6"))),
        "@@MODEL_VERSION@@": html.escape(str(report.get("model_version", "6.0.0"))),
        "@@CANDIDATE_LABEL@@": candidate_label,
        "@@EPOCH_COUNT@@": str(epoch_count),
        "@@DEVICE@@": html.escape(str(compute.get("device", "cuda"))),
        "@@CREATED@@": html.escape(created),
        "@@BATTLES@@": f"{int(data.get('battles_total', 0)):,}",
        "@@TRAIN_ACTIONS@@": f"{int(data.get('train_samples', 0)):,}",
        "@@PARAMS@@": f"{int(compute.get('parameters', 0)):,}",
        "@@SECONDS@@": _fmt_float(report.get("seconds"), 1),
        "@@VERDICT@@": verdict,
        "@@SLOT@@": _fmt_pct(test.get("slot_top1")),
        "@@SLOT_DELTA@@": _pp(test.get("slot_top1"), reference.get("slot_top1")),
        "@@ZONE@@": _fmt_pct(test.get("zone_acc")),
        "@@ZONE_DELTA@@": _pp(test.get("zone_acc"), reference.get("zone_acc")),
        "@@XY@@": _fmt_float(test.get("xy_mae"), 0),
        "@@XY_DELTA@@": _delta_units(test.get("xy_mae"), reference.get("xy_mae")),
        "@@ARG_X_STD@@": _fmt_float(argmax_tile.get("x_std"), 3),
        "@@HUMAN_X_STD@@": _fmt_float(human.get("x_std"), 3),
        "@@COMPARISON_ROWS@@": comparison_rows,
        "@@ACTION_RATIO@@": f"{action_ratio:.1f}",
        "@@MASKING_STATUS@@": html.escape(masking_status),
        "@@HIDE_PROB@@": _fmt_float(hide_prob, 2),
        "@@HUMAN_RX@@": _fmt_float(180.0 * float(human.get("x_std") or 0), 2),
        "@@HUMAN_RY@@": _fmt_float(320.0 * float(human.get("y_std") or 0), 2),
        "@@EXPECTED_RX@@": _fmt_float(180.0 * float(expected.get("x_std") or 0), 2),
        "@@EXPECTED_RY@@": _fmt_float(320.0 * float(expected.get("y_std") or 0), 2),
        "@@ARGMAX_MAE@@": _fmt_float(argmax_tile.get("mean_l1_units"), 0),
        "@@DECK_SLOT_DELTA@@": _pp(revealed.get("slot_top1"), oracle.get("slot_top1")),
        "@@HISTORY_ZONE_DELTA@@": _pp(neutral_xy.get("zone_acc"), oracle.get("zone_acc")),
        "@@WINNER_SLOT_DELTA@@": _pp(winner.get("slot_top1"), loser.get("slot_top1")),
        "@@SPATIAL_ROWS@@": spatial_rows,
        "@@DECK_ZONE_DELTA@@": _pp(revealed.get("zone_acc"), oracle.get("zone_acc")),
        "@@DECK_XY_DELTA@@": _delta_units(revealed.get("xy_mae_units"), oracle.get("xy_mae_units")),
        "@@THREAT_ZONE_DELTA@@": _pp(no_threat.get("zone_acc"), oracle.get("zone_acc")),
        "@@THREAT_XY_DELTA@@": _delta_units(no_threat.get("xy_mae_units"), oracle.get("xy_mae_units")),
        "@@HISTORY_JSON@@": _json_script(history),
        "@@DECODE_JSON@@": _json_script(decode),
        "@@SPLIT_ROWS@@": split_rows,
        "@@BATCH@@": str(compute.get("batch_size", "—")),
        "@@LR@@": str(compute.get("learning_rate", "—")),
        "@@D_MODEL@@": str(compute.get("d_model", "—")),
        "@@LAYERS@@": str(compute.get("num_layers", "—")),
        "@@CONTEXT@@": str(compute.get("max_context", "—")),
        "@@MAX_SAMPLES@@": str(compute.get("max_samples_per_battle", "—")),
        "@@TILE_WEIGHT@@": str((compute.get("loss_kwargs") or {}).get("tile_weight", "—")),
        "@@REAL_SCORE@@": _fmt_float(rollouts.get("mean_score_real"), 3),
        "@@POLICY_SCORE@@": _fmt_float(rollouts.get("mean_score_policy"), 3),
        "@@EASY_SCORE@@": _fmt_float(rollouts.get("mean_score_easy"), 4),
        "@@MEDIUM_SCORE@@": _fmt_float(rollouts.get("mean_score_medium"), 4),
        "@@ROLLOUT_N@@": str(rollouts.get("n", 0)),
        "@@LESSONS@@": lesson_html,
    }
    body = template
    for token, value in replacements.items():
        body = body.replace(token, value)
    output_path.write_text(body, encoding="utf-8")
    return output_path
