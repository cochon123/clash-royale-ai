"""Showcase HTML for policy_bc_v5 — anti-detector training, not a plain loss dump.

Inspired by the interactive density of the v4 showcase, but built around v5's
actual story: style moment-matching, REINFORCE polish, tell gaps vs humans /
v4.1, clock vs alternation protocols, multi-run training timeline, and battle
royale standings when available.
"""

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
    p = Path(path)
    if not p.exists():
        return {}
    with p.open(encoding="utf-8") as handle:
        return json.load(handle)


def _delta(a: float | None, b: float | None) -> float | None:
    if a is None or b is None:
        return None
    return float(a) - float(b)


def render_policy_v5_report(
    model_dir: str | Path = "models/policy_bc_v5",
    old_model_dir: str | Path = "models/policy_bc_v4.1",
    battle_royale_path: str | Path = "reports/battle_royale_v5.json",
    output_path: str | Path = "reports/policy_bc_v5.html",
) -> Path:
    model_dir = Path(model_dir)
    report = _load(model_dir / "report.json")
    if not report:
        raise FileNotFoundError(f"Missing {model_dir / 'report.json'}")
    old = _load(Path(old_model_dir) / "report.json")
    br = _load(battle_royale_path)

    created = _report_timestamp(model_dir, "report.json", "best_model.pt")
    compute = report.get("compute") or {}
    data = report.get("data") or {}
    test = report.get("test") or {}
    style = report.get("style") or {}
    rollouts = report.get("rollouts") or {}
    history = report.get("history") or []
    style_history = report.get("style_history") or []
    timeline = report.get("training_timeline") or []
    lessons = report.get("lessons") or []
    readiness = report.get("live_play_readiness") or {}

    tell_gaps = style.get("tell_gaps") or {}
    v41_tells = ((style.get("v4_1_compare") or {}).get("tell_gaps")) or {}
    v41_style = style.get("v4_1_compare") or {}
    alt = (style.get("alternation") or {}).get("full") or {}
    clock_full = style.get("full") or {}
    feat_l2 = style.get("feature_l2") or clock_full.get("feature_l2")
    v41_l2 = v41_style.get("feature_l2")
    l2_delta = _delta(v41_l2, feat_l2)  # positive = v5 closer to human

    tell_rows = []
    for name, vals in tell_gaps.items():
        human = float(vals.get("human") or 0)
        ai = float(vals.get("ai") or 0)
        old_ai = float((v41_tells.get(name) or {}).get("ai") or 0)
        tell_rows.append(
            {
                "feature": name,
                "human": human,
                "v5": ai,
                "v41": old_ai,
                "v5_err": abs(ai - human),
                "v41_err": abs(old_ai - human),
                "improved": abs(ai - human) < abs(old_ai - human) - 1e-9,
            }
        )
    tell_rows.sort(key=lambda r: -(r["v41_err"] - r["v5_err"]))

    n_improved = sum(1 for r in tell_rows if r["improved"])
    timing_tells = [r for r in tell_rows if "gap" in r["feature"]]
    place_tells = [
        r for r in tell_rows if any(x in r["feature"] for x in ("x_std", "y_std", "tile"))
    ]

    br_standings = br.get("standings") or []
    br_setup = br.get("setup") or {}
    br_pairs = br.get("pairs") or []
    v5_standing = next(
        (
            s
            for s in br_standings
            if "v5" in str(s.get("policy_id", "")).lower()
            or "5" in str(s.get("policy_id", ""))
        ),
        None,
    )
    # Also match policy-bc-v5 / policy_bc_v5
    if v5_standing is None:
        for s in br_standings:
            pid = str(s.get("policy_id", "")).lower().replace("-", "_").replace(".", "_")
            if "v5" in pid or pid.endswith("_5") or "bc_v5" in pid:
                v5_standing = s
                break

    global_epochs = len(history)
    runs = len(timeline) or int(compute.get("runs") or 1)
    polish_count = sum(len(t.get("polishes") or []) for t in timeline)

    payload = {
        "history": history,
        "styleHistory": style_history,
        "timeline": timeline,
        "tells": tell_rows,
        "timingTells": timing_tells,
        "placeTells": place_tells,
        "style": {
            "feat_l2": feat_l2,
            "v41_l2": v41_l2,
            "clock_P": clock_full.get("mean_P_human_ai"),
            "clock_fool": clock_full.get("fool_rate_at_0.5"),
            "clock_auc": clock_full.get("auc"),
            "alt_P": alt.get("mean_P_human_ai"),
            "alt_fool": alt.get("fool_rate_at_0.5"),
            "alt_auc": alt.get("auc"),
            "v41_clock_P": (v41_style.get("full") or {}).get("mean_P_human_ai"),
        },
        "test": test,
        "oldTest": old.get("test") or {},
        "rollouts": {k: v for k, v in rollouts.items() if k != "hist"},
        "battleRoyale": {
            "ready": bool(br_standings),
            "standings": br_standings,
            "pairs": br_pairs,
            "setup": br_setup,
            "champion": br.get("champion"),
            "v5": v5_standing,
        },
        "meta": {
            "global_epochs": global_epochs,
            "runs": runs,
            "polishes": polish_count,
            "n_improved_tells": n_improved,
            "n_tells": len(tell_rows),
        },
    }

    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)

    ready = readiness.get("ready_for_live_smoke_test")
    ready_badge = "offline gates passed" if ready else "keep iterating offline"

    timeline_cards = "".join(
        f"""
        <div class="run-card {'crash' if t.get('crashed') else 'ok'}">
          <div class="run-label">Run {t.get('run')}</div>
          <div class="run-epochs">{t.get('n_epochs', 0)} epochs</div>
          <div class="run-meta">{(
            'crashed on first polish' if t.get('crashed')
            else (
              f"{len(t.get('polishes') or [])} REINFORCE polish"
              + ('es' if len(t.get('polishes') or []) != 1 else '')
              if (t.get('polishes') or [])
              else 'stalled before polish'
            )
          )}</div>
          <div class="run-warm">warm ← {html.escape(str(t.get('warmstart') or '—').split('/')[-1])}</div>
        </div>
        """
        for t in timeline
    ) or "<p class='caption'>No timeline parsed from the training log.</p>"

    lesson_html = "".join(f"<li>{html.escape(str(x))}</li>" for x in lessons)

    body = f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>PolicyBC v5 — hide from the style judge</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=DM+Sans:wght@450;600;700&family=IBM+Plex+Mono:wght@400;500&display=swap" rel="stylesheet">
  <style>
  {_base_styles()}
  body {{
    font-family: "DM Sans", ui-sans-serif, system-ui, sans-serif;
    background:
      radial-gradient(1200px 600px at 10% -10%, rgba(34,211,238,0.10), transparent 55%),
      radial-gradient(900px 500px at 90% 0%, rgba(251,191,36,0.08), transparent 50%),
      var(--bg);
  }}
  .hero {{
    padding: 8px 0 36px;
    border-bottom: 1px solid var(--line-soft);
  }}
  .hero h1 {{
    font-size: clamp(2.1rem, 4.5vw, 3rem);
    letter-spacing: -0.035em;
    max-width: 18ch;
    line-height: 1.05;
  }}
  .hero-sub {{ max-width: 62ch; font-size: 1.05rem; color: #cbd5e1; }}
  .hero-scores {{
    display: flex; flex-wrap: wrap; gap: 14px; margin-top: 22px;
  }}
  .hero-score {{
    min-width: 140px; padding: 14px 16px; border-radius: 14px;
    background: rgba(15,23,42,0.65); border: 1px solid rgba(148,163,184,0.18);
  }}
  .hero-score-label {{
    font-size: 0.68rem; letter-spacing: 0.1em; text-transform: uppercase; color: #94a3b8;
  }}
  .hero-score-value {{
    display: block; margin-top: 6px; font-size: 1.7rem; font-weight: 700;
    font-variant-numeric: tabular-nums; letter-spacing: -0.02em;
  }}
  .hero-score-value.up {{ color: #34d399; }}
  .hero-score-value.warn {{ color: #fbbf24; }}
  .hero-score-value.flat {{ color: #e2e8f0; }}
  .mono {{ font-family: "IBM Plex Mono", ui-monospace, monospace; }}

  .run-row {{
    display: grid; grid-template-columns: repeat(3, minmax(0,1fr)); gap: 14px;
  }}
  .run-card {{
    padding: 16px; border-radius: 14px;
    border: 1px solid rgba(148,163,184,0.16);
    background: rgba(15,23,42,0.55);
  }}
  .run-card.crash {{ border-color: rgba(248,113,113,0.45); }}
  .run-card.ok {{ border-color: rgba(52,211,153,0.35); }}
  .run-label {{ font-size: 0.7rem; letter-spacing: 0.12em; text-transform: uppercase; color: #94a3b8; }}
  .run-epochs {{ font-size: 1.6rem; font-weight: 700; margin: 6px 0; }}
  .run-meta, .run-warm {{ font-size: 0.85rem; color: #94a3b8; }}

  .protocol-toggle {{
    display: inline-flex; gap: 6px; padding: 4px; border-radius: 999px;
    background: rgba(148,163,184,0.1); border: 1px solid rgba(148,163,184,0.18);
    margin-bottom: 16px;
  }}
  .protocol-toggle button {{
    border: 0; background: transparent; color: #94a3b8; cursor: pointer;
    padding: 8px 14px; border-radius: 999px; font: inherit; font-size: 0.88rem;
  }}
  .protocol-toggle button.active {{
    background: #0f172a; color: #e2e8f0; box-shadow: 0 0 0 1px #334155;
  }}
  .protocol-panel {{ display: none; }}
  .protocol-panel.active {{ display: block; }}

  .race {{ display: grid; gap: 12px; }}
  .race-row {{
    display: grid; grid-template-columns: 160px 1fr 90px; gap: 12px; align-items: center;
  }}
  .race-name {{ font-size: 0.88rem; }}
  .race-name small {{ display: block; color: #64748b; font-size: 0.72rem; }}
  .race-track {{
    position: relative; height: 22px; border-radius: 8px;
    background: rgba(148,163,184,0.1); overflow: hidden;
  }}
  .race-fill {{
    position: absolute; inset: 0 auto 0 0; width: 0%;
    background: linear-gradient(90deg, #22d3ee, #34d399);
    transition: width 1s cubic-bezier(.22,1,.36,1);
  }}
  .race-fill.old {{ background: linear-gradient(90deg, #64748b, #94a3b8); opacity: 0.55; }}
  .race-val {{ font-variant-numeric: tabular-nums; font-size: 0.9rem; text-align: right; }}

  .radar-wrap {{ width: 100%; max-width: 520px; margin: 0 auto; }}
  svg.radar {{ width: 100%; height: auto; aspect-ratio: 1; display: block; }}

  .wire {{
    display: grid; grid-template-columns: 1.1fr 28px 1fr; gap: 10px; align-items: stretch;
    max-width: 900px;
  }}
  .wire-col {{
    padding: 16px; border-radius: 16px;
    background: rgba(148,163,184,0.06); border: 1px solid rgba(148,163,184,0.16);
  }}
  .wire-title {{
    font-size: 0.68rem; letter-spacing: 0.12em; text-transform: uppercase;
    color: #94a3b8; margin-bottom: 12px;
  }}
  .wire-box {{
    padding: 10px 12px; border-radius: 10px; text-align: center; font-size: 0.84rem;
    margin-bottom: 8px; background: rgba(34,211,238,0.12);
    border: 1px solid rgba(148,163,184,0.22);
  }}
  .wire-box.live {{ border-color: #34d399; box-shadow: 0 0 0 1px #34d39944; }}
  .wire-box.warn {{ border-color: #fbbf24; }}
  .wire-arrow {{
    align-self: center; height: 2px;
    background: linear-gradient(90deg, #22d3ee, #fbbf24);
  }}
  .wire-note {{ font-size: 0.78rem; color: #94a3b8; margin-top: 8px; text-align: center; }}

  .br-table td.good {{ color: #34d399; font-weight: 600; }}
  .tell-chip {{
    display: inline-flex; align-items: center; gap: 6px; padding: 4px 10px;
    border-radius: 999px; font-size: 0.78rem; margin: 2px 4px 2px 0;
    background: rgba(148,163,184,0.1); border: 1px solid rgba(148,163,184,0.16);
  }}
  .tell-chip.up {{ border-color: rgba(52,211,153,0.45); color: #6ee7b7; }}
  .tell-chip.down {{ border-color: rgba(248,113,113,0.4); color: #fca5a5; }}

  .scrub-bar {{
    display: flex; flex-wrap: wrap; gap: 12px; align-items: center; margin-top: 14px;
  }}
  .scrub-bar input[type=range] {{ flex: 1; min-width: 180px; accent-color: #22d3ee; }}
  .play-btn {{
    border: 0; border-radius: 999px; padding: 9px 18px; cursor: pointer;
    font: inherit; font-weight: 700; font-size: 0.88rem;
    background: linear-gradient(120deg, #22d3ee, #34d399); color: #041016;
  }}
  .play-btn:hover {{ filter: brightness(1.08); }}

  @media (max-width: 820px) {{
    .run-row, .wire {{ grid-template-columns: 1fr; }}
    .wire-arrow {{ width: 2px; height: 24px; margin: 0 auto; }}
    .race-row {{ grid-template-columns: 1fr; }}
  }}
  </style>
</head>
<body>
<main>
  <header class="hero report-header">
    <div class="badge-row">
      <span class="badge">policy-bc-v5</span>
      <span class="badge">anti-detector</span>
      <span class="badge">{global_epochs} logged epochs · {runs} runs</span>
      <span class="badge">{html.escape(str(compute.get("device", "cuda")))}</span>
      <span class="badge">{html.escape(ready_badge)}</span>
    </div>
    <h1>Mimic humans. Confuse the judge.</h1>
    <p class="hero-sub">
      v5 keeps the v4.1 trunk, then trains against the frozen human-vs-AI style discriminator:
      differentiable moment matching on timing/placement tells, plus REINFORCE polish on
      clock-aware rollouts. This page is the full multi-run story — not the truncated stages file.
    </p>
    <p class="meta">Snapshot eval {html.escape(created)} ·
      {data.get("train_samples", 0):,} train actions ·
      {data.get("battles_total", 0):,} battles ·
      {compute.get("parameters", 0):,} params ·
      style_match={compute.get("style_match_weight", 0.4)}</p>
    <div class="hero-scores">
      <div class="hero-score">
        <span class="hero-score-label">Tell distance vs v4.1</span>
        <span class="hero-score-value up mono" id="heroL2">
          {('−' + _fmt_float(l2_delta, 2)) if l2_delta and l2_delta > 0 else _fmt_float(feat_l2, 2)}
        </span>
      </div>
      <div class="hero-score">
        <span class="hero-score-label">Test slot top-1</span>
        <span class="hero-score-value flat mono">{_fmt_pct(test.get("slot_top1"))}</span>
      </div>
      <div class="hero-score">
        <span class="hero-score-label">Tells improved</span>
        <span class="hero-score-value up mono">{n_improved}/{len(tell_rows)}</span>
      </div>
      <div class="hero-score">
        <span class="hero-score-label">Clock fool @0.5</span>
        <span class="hero-score-value warn mono">{_fmt_pct(clock_full.get("fool_rate_at_0.5"))}</span>
      </div>
    </div>
  </header>

  <section class="report-section">
    <h2>What v5 actually changes</h2>
    <div class="wire">
      <div class="wire-col">
        <div class="wire-title">kept from v4.1</div>
        <div class="wire-box">causal GRU trunk</div>
        <div class="wire-box">threat + reaction upweight</div>
        <div class="wire-box live">card-conditioned zone / XY</div>
      </div>
      <div class="wire-arrow"></div>
      <div class="wire-col">
        <div class="wire-title">new anti-detector stack</div>
        <div class="wire-box live">BC loss (still primary)</div>
        <div class="wire-box live">style moment match</div>
        <div class="wire-box warn">REINFORCE vs tell distance</div>
        <div class="wire-box">clock-aware who-acts-next</div>
        <p class="wire-note">Judge is non-differentiable → match features + black-box polish</p>
      </div>
    </div>
  </section>

  <section class="report-section">
    <h2>Training timeline — {global_epochs} epochs across {runs} runs</h2>
    <p class="caption">Parsed from <span class="mono">logs/policy_bc_v5.log</span>. Run 1 died on the first polish; run 2 is where REINFORCE actually landed; run 3 resumed and stalled again.</p>
    <div class="run-row">{timeline_cards}</div>
    <div class="chart-grid" style="margin-top:36px">
      <div class="chart-block">
        <h2>BC losses</h2>
        <svg class="chart" id="lossChart"></svg>
        <div class="legend"></div>
      </div>
      <div class="chart-block">
        <h2>Style match loss (↓ means batch moments closer to humans)</h2>
        <svg class="chart" id="styleMatchChart"></svg>
        <div class="legend"></div>
      </div>
      <div class="chart-block">
        <h2>Slot@1 stays healthy while tell distance falls</h2>
        <p class="caption">Feature L2 is probed sparsely; the line carries the last measured value forward.</p>
        <svg class="chart" id="slotStyleChart"></svg>
        <div class="legend"></div>
      </div>
    </div>
    <div class="scrub-bar">
      <button type="button" class="play-btn" id="animPlay">Play epoch film</button>
      <input type="range" id="animScrub" min="0" max="0" value="0" step="1">
      <div class="caption" id="animReadout">—</div>
    </div>
  </section>

  <section class="report-section">
    <h2>Style judge — two protocols</h2>
    <p class="caption">The frozen discriminator was trained on forced side-alternation rollouts. Under that harness it still sees v5 as AI. With the action-clock deciding who acts next (deploy-shaped), fool rate jumps. v5 optimizes the residual: <span class="mono">feature_l2</span> on controllable tells.</p>
    <div class="protocol-toggle" id="protoToggle">
      <button type="button" class="active" data-panel="clock">Clock (deploy)</button>
      <button type="button" data-panel="alt">Alternation (legacy harness)</button>
    </div>
    <div class="protocol-panel active" id="panel-clock">
      <div class="kpi-row">
        <div><span class="kpi-label">P(human)</span><span class="kpi-value mono">{_fmt_float(clock_full.get("mean_P_human_ai"), 4)}</span></div>
        <div><span class="kpi-label">Fool @0.5</span><span class="kpi-value mono">{_fmt_pct(clock_full.get("fool_rate_at_0.5"))}</span></div>
        <div><span class="kpi-label">Detection AUC</span><span class="kpi-value mono">{_fmt_float(clock_full.get("auc"), 3)}</span></div>
        <div><span class="kpi-label">Feature L2</span><span class="kpi-value mono">{_fmt_float(feat_l2, 3)}</span></div>
      </div>
    </div>
    <div class="protocol-panel" id="panel-alt">
      <div class="kpi-row">
        <div><span class="kpi-label">P(human)</span><span class="kpi-value mono">{_fmt_float(alt.get("mean_P_human_ai"), 4)}</span></div>
        <div><span class="kpi-label">Fool @0.5</span><span class="kpi-value mono">{_fmt_pct(alt.get("fool_rate_at_0.5"))}</span></div>
        <div><span class="kpi-label">Detection AUC</span><span class="kpi-value mono">{_fmt_float(alt.get("auc"), 3)}</span></div>
        <div><span class="kpi-label">v4.1 clock P</span><span class="kpi-value mono">{_fmt_float((v41_style.get("full") or {}).get("mean_P_human_ai"), 4)}</span></div>
      </div>
      <p class="caption" style="margin-top:14px">Near-zero fool rate here is expected: harness features (<span class="mono">alt_rate</span>, <span class="mono">n_events</span>) dominate the frozen judge.</p>
    </div>
  </section>

  <section class="report-section">
    <h2>Tell map — human · v5 · v4.1</h2>
    <p class="caption">Closer to the human ring is better. Green chips = v5 closer than v4.1.</p>
    <div id="tellChips" style="margin-bottom:18px"></div>
    <div class="block-grid">
      <div class="block">
        <div class="radar-wrap"><svg class="radar" id="tellRadar" viewBox="0 0 400 400"></svg></div>
      </div>
      <div class="block">
        <h2>Error race (abs distance to human)</h2>
        <div class="race" id="tellRace"></div>
        <button type="button" class="play-btn" id="racePlay" style="margin-top:16px">Animate race</button>
      </div>
    </div>
  </section>

  <section class="report-section">
    <h2>Where the gap closed — and where it didn't</h2>
    <div class="block-grid">
      <div class="block">
        <h2>Timing (mostly fixed)</h2>
        <table>
          <thead><tr><th>Tell</th><th>Human</th><th>v5</th><th>v4.1</th></tr></thead>
          <tbody>
            {"".join(
              f"<tr><td class='mono'>{html.escape(r['feature'])}</td>"
              f"<td>{_fmt_float(r['human'],3)}</td>"
              f"<td>{_fmt_float(r['v5'],3)}</td>"
              f"<td>{_fmt_float(r['v41'],3)}</td></tr>"
              for r in timing_tells
            ) or "<tr><td colspan='4'>No timing tells</td></tr>"}
          </tbody>
        </table>
      </div>
      <div class="block">
        <h2>Placement spread (still short)</h2>
        <table>
          <thead><tr><th>Tell</th><th>Human</th><th>v5</th><th>v4.1</th></tr></thead>
          <tbody>
            {"".join(
              f"<tr><td class='mono'>{html.escape(r['feature'])}</td>"
              f"<td>{_fmt_float(r['human'],3)}</td>"
              f"<td>{_fmt_float(r['v5'],3)}</td>"
              f"<td>{_fmt_float(r['v41'],3)}</td></tr>"
              for r in place_tells
            ) or "<tr><td colspan='4'>No placement tells</td></tr>"}
          </tbody>
        </table>
        <p class="caption">Humans spray X across lanes (~0.22 std). Policies still cluster (~0.13–0.14). Next lever after v5.</p>
      </div>
    </div>
  </section>

  <section class="report-section">
    <h2>Cloning quality (still required)</h2>
    <div class="kpi-row">
      <div><span class="kpi-label">Slot top-1</span><span class="kpi-value mono">{_fmt_pct(test.get("slot_top1"))}</span></div>
      <div><span class="kpi-label">Zone acc</span><span class="kpi-value mono">{_fmt_pct(test.get("zone_acc"))}</span></div>
      <div><span class="kpi-label">Timing MAE</span><span class="kpi-value mono">{_fmt_float(test.get("timing_mae"), 2)}s</span></div>
      <div><span class="kpi-label">Rollout P(real)</span><span class="kpi-value mono">{_fmt_float(rollouts.get("mean_score_policy"), 3)}</span></div>
    </div>
    <p class="caption">Style gains that destroy slot@1 are rejected at checkpoint time. Offline readiness: {html.escape(str(readiness.get("rationale") or "—"))}</p>
  </section>

  <section class="report-section" id="brSection">
    <h2>Battle royale</h2>
    <p class="caption" id="brCaption">Loading standings when <span class="mono">reports/battle_royale_v5.json</span> is ready — 5 policies, 100 games/pair (400 games each).</p>
    <div id="brBody"></div>
  </section>

  <section class="report-section">
    <h2>Lessons</h2>
    <ul class="lessons">{lesson_html}</ul>
  </section>
</main>
<script>
{_chart_script()}
const DATA = {_json_script(payload)};

function fill(v) {{ return v == null || Number.isNaN(v) ? null : Number(v); }}

(function charts() {{
  const hist = DATA.history || [];
  if (!hist.length) return;
  const labels = hist.map((r) => String(r.epoch));
  renderLineChart("lossChart", [
    {{ color: "#60a5fa", label: "Train loss", values: hist.map((r) => r.train_loss) }},
    {{ color: "#34d399", label: "Val loss", values: hist.map((r) => r.val_loss) }},
  ], labels, {{ yFormat: "float" }});
  renderLineChart("styleMatchChart", [
    {{ color: "#fbbf24", label: "Style match", values: hist.map((r) => r.train_style_match) }},
  ], labels, {{ yFormat: "float" }});

  const startL2 = (DATA.styleHistory || []).find((s) => s.feature_l2 != null);
  let last = startL2 ? fill(startL2.feature_l2) : null;
  const l2filled = hist.map((r) => {{
    const v = fill(r.style_feature_l2);
    if (v != null) last = v;
    return last;
  }});
  renderLineChart("slotStyleChart", [
    {{ color: "#a78bfa", label: "Val slot@1", values: hist.map((r) => r.val_slot_top1) }},
    {{ color: "#f87171", label: "Feature L2", values: l2filled }},
  ], labels, {{ yFormat: "float" }});
}})();

(function protocol() {{
  const root = document.getElementById("protoToggle");
  if (!root) return;
  root.addEventListener("click", (ev) => {{
    const btn = ev.target.closest("button");
    if (!btn) return;
    root.querySelectorAll("button").forEach((b) => b.classList.toggle("active", b === btn));
    document.querySelectorAll(".protocol-panel").forEach((p) => {{
      p.classList.toggle("active", p.id === "panel-" + btn.dataset.panel);
    }});
  }});
}})();

(function tells() {{
  const tells = DATA.tells || [];
  const chips = document.getElementById("tellChips");
  chips.innerHTML = tells.map((t) =>
    `<span class="tell-chip ${{t.improved ? "up" : "down"}}"><b>${{t.feature}}</b> ${{t.improved ? "closer" : "still off"}}</span>`
  ).join("");

  // Radar: normalize each axis by human magnitude (or 1)
  const svg = document.getElementById("tellRadar");
  const names = tells.map((t) => t.feature);
  const n = Math.max(names.length, 1);
  const cx = 200, cy = 200, R = 150;
  function pt(i, radius) {{
    const a = (-Math.PI / 2) + (i / n) * Math.PI * 2;
    return [cx + Math.cos(a) * radius, cy + Math.sin(a) * radius];
  }}
  function ring(vals, color, fill) {{
    const pts = vals.map((v, i) => pt(i, Math.max(8, Math.min(1, v) * R)));
    const d = pts.map((p, i) => (i ? "L" : "M") + p[0].toFixed(1) + "," + p[1].toFixed(1)).join(" ") + " Z";
    return `<path d="${{d}}" fill="${{fill}}" stroke="${{color}}" stroke-width="2" />`;
  }}
  // Similarity to human: 1 - clipped relative error
  function sim(val, human) {{
    const denom = Math.max(Math.abs(human), 0.05);
    return Math.max(0, 1 - Math.abs(val - human) / denom);
  }}
  const hum = tells.map(() => 1);
  const v5 = tells.map((t) => sim(t.v5, t.human));
  const v41 = tells.map((t) => sim(t.v41, t.human));
  let grid = "";
  for (const g of [0.25, 0.5, 0.75, 1]) {{
    const pts = Array.from({{length: n}}, (_, i) => pt(i, R * g));
    grid += `<polygon points="${{pts.map(p => p.join(",")).join(" ")}}" fill="none" stroke="#1e293b" />`;
  }}
  const spokes = names.map((_, i) => {{
    const [x, y] = pt(i, R);
    return `<line x1="${{cx}}" y1="${{cy}}" x2="${{x}}" y2="${{y}}" stroke="#1e293b" />`;
  }}).join("");
  const labels = names.map((name, i) => {{
    const [x, y] = pt(i, R + 22);
    return `<text x="${{x}}" y="${{y}}" text-anchor="middle" dominant-baseline="middle" fill="#94a3b8" font-size="9">${{name.replace("frac_","").replace("gap_","g.")}}</text>`;
  }}).join("");
  svg.innerHTML = grid + spokes
    + ring(hum, "#94a3b8", "rgba(148,163,184,0.08)")
    + ring(v41, "#64748b", "rgba(100,116,139,0.12)")
    + ring(v5, "#34d399", "rgba(52,211,153,0.18)")
    + labels
    + `<text x="20" y="24" fill="#94a3b8" font-size="11">human</text>`
    + `<text x="20" y="40" fill="#64748b" font-size="11">v4.1</text>`
    + `<text x="20" y="56" fill="#34d399" font-size="11">v5</text>`;

  const race = document.getElementById("tellRace");
  const maxErr = Math.max(...tells.map((t) => Math.max(t.v5_err, t.v41_err)), 1e-6);
  race.innerHTML = tells.slice(0, 8).map((t) => `
    <div class="race-row" data-v5="${{(100 * t.v5_err / maxErr).toFixed(2)}}" data-v41="${{(100 * t.v41_err / maxErr).toFixed(2)}}">
      <div class="race-name">${{t.feature}}<small>abs err to human</small></div>
      <div class="race-track">
        <div class="race-fill old" style="width:0%"></div>
        <div class="race-fill" style="width:0%"></div>
      </div>
      <div class="race-val mono">${{t.v5_err.toFixed(3)}}</div>
    </div>`).join("");

  function runRace() {{
    race.querySelectorAll(".race-row").forEach((row) => {{
      const fills = row.querySelectorAll(".race-fill");
      fills[0].style.width = row.dataset.v41 + "%";
      fills[1].style.width = row.dataset.v5 + "%";
    }});
  }}
  document.getElementById("racePlay").addEventListener("click", runRace);
  setTimeout(runRace, 350);
}})();

(function anim() {{
  const hist = DATA.history || [];
  const scrub = document.getElementById("animScrub");
  const readout = document.getElementById("animReadout");
  const btn = document.getElementById("animPlay");
  if (!hist.length || !scrub) return;
  scrub.max = String(hist.length - 1);
  scrub.value = String(hist.length - 1);
  let timer = null;
  function draw() {{
    const i = Number(scrub.value);
    const r = hist[i];
    const polish = r.polish ? ` · polish reward ${{Number(r.polish.reward).toFixed(3)}}` : "";
    readout.innerHTML = `Global epoch <strong>${{r.epoch}}</strong> (run ${{r.run}} #${{r.run_epoch}}) · `
      + `slot <strong>${{(100 * r.val_slot_top1).toFixed(1)}}%</strong> · `
      + `style_match <strong>${{Number(r.train_style_match).toFixed(3)}}</strong>`
      + (r.style_feature_l2 != null ? ` · L2 <strong>${{Number(r.style_feature_l2).toFixed(3)}}</strong>` : "")
      + polish;
  }}
  scrub.addEventListener("input", () => {{ if (timer) {{ clearInterval(timer); timer = null; btn.textContent = "Play epoch film"; }} draw(); }});
  btn.addEventListener("click", () => {{
    if (timer) {{ clearInterval(timer); timer = null; btn.textContent = "Play epoch film"; return; }}
    btn.textContent = "Pause";
    timer = setInterval(() => {{
      let i = Number(scrub.value);
      i = (i + 1) % hist.length;
      scrub.value = String(i);
      draw();
    }}, 220);
  }});
  draw();
}})();

(function battleRoyale() {{
  const body = document.getElementById("brBody");
  const caption = document.getElementById("brCaption");
  const br = DATA.battleRoyale || {{}};
  if (!br.ready) {{
    body.innerHTML = `<p class="caption">Tournament still running or JSON missing. Re-open this page after <span class="mono">logs/battle_royale_v5.log</span> finishes — or regenerate with the report command.</p>`;
    return;
  }}
  const setup = br.setup || {{}};
  caption.innerHTML = `${{(setup.policies || []).length}} policies · ${{setup.games_per_pair || "?"}} games/pair`
    + ` (= <b>400 raw games each</b>) · min confidence ${{setup.min_confidence ?? 0.8}}`
    + ` · champion <b>${{br.champion || "—"}}</b>`
    + ` · full table in <a href="battle_royale_v5.html">battle_royale_v5.html</a>`;
  const standings = br.standings || [];
  const rows = standings.map((s) => {{
    const pid = String(s.policy_id || "");
    const isV5 = /v5/i.test(pid);
    return `<tr class="${{isV5 ? "champ" : ""}}">
      <td>${{s.rank ?? ""}}</td>
      <td>${{pid}}${{isV5 ? " ←" : ""}}</td>
      <td>${{s.wins ?? 0}}-${{s.losses ?? 0}}</td>
      <td class="${{(s.win_rate || 0) >= 0.5 ? "good" : ""}}">${{((s.win_rate || 0) * 100).toFixed(1)}}%</td>
      <td>${{((s.raw_win_rate || 0) * 100).toFixed(1)}}%</td>
      <td>${{s.raw_games ?? s.games ?? 0}}</td>
      <td>${{(s.elo || 0).toFixed(0)}}</td>
    </tr>`;
  }}).join("");
  const pairs = (br.pairs || []).map((p) =>
    `<tr><td>${{p.a}}</td><td>${{p.b}}</td><td>${{p.a_wins}}-${{p.b_wins}}</td><td>${{p.confident_games}}/${{p.games}}</td></tr>`
  ).join("");
  body.innerHTML = `
    <div class="block-grid">
      <div class="block block-wide">
        <h2>Standings</h2>
        <table class="br-table">
          <thead><tr><th>#</th><th>Policy</th><th>Conf W-L</th><th>Conf WR</th><th>Raw WR</th><th>Raw games</th><th>Elo</th></tr></thead>
          <tbody>${{rows}}</tbody>
        </table>
        <p class="caption">Confidence gate keeps only winner-model decisions ≥ threshold. Raw WR counts every offline game.</p>
      </div>
      <div class="block block-wide">
        <h2>Pair results</h2>
        <table>
          <thead><tr><th>A</th><th>B</th><th>Conf score</th><th>Kept/raw</th></tr></thead>
          <tbody>${{pairs}}</tbody>
        </table>
      </div>
    </div>`;
}})();
</script>
</body>
</html>
"""
    out.write_text(body, encoding="utf-8")
    return out


def refresh_battle_royale_into_report(
    model_dir: str | Path = "models/policy_bc_v5",
    battle_royale_path: str | Path = "reports/battle_royale_v5.json",
    output_path: str | Path = "reports/policy_bc_v5.html",
) -> Path:
    """Re-render after battle royale finishes."""
    return render_policy_v5_report(
        model_dir=model_dir,
        battle_royale_path=battle_royale_path,
        output_path=output_path,
    )
