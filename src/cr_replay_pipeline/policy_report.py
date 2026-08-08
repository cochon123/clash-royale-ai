"""HTML training report for the behavior-cloning policy."""

from __future__ import annotations

import html
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


def _version_key(report: dict[str, Any], model_dir: Path) -> str:
    ver = str(report.get("model_version") or report.get("version") or "")
    name = str(report.get("model_name") or model_dir.name)
    if ver.startswith("4") or "v4" in name:
        return "4"
    if ver.startswith("3") or "v3" in name:
        return "3"
    if ver.startswith("2") or "v2" in name:
        return "2"
    return "1"


def _version_story(version: str) -> tuple[str, str]:
    """Return (headline, subcopy) for the hero."""
    stories = {
        "1": (
            "Next-action policy — baseline",
            "Causal GRU that predicts the next deck-slot, type, placement and timing "
            "from action history. No cycle features, no threat conditioning, no card-"
            "aware placement — the floor everything else has to beat.",
        ),
        "2": (
            "Next-action policy — cycle-aware",
            "Adds per-card cycle features and a card-conditioned slot head so the model "
            "knows which cards are likely in hand. Placement is still card-blind.",
        ),
        "3": (
            "Next-action policy — threat-conditioned",
            "Keeps cycle features and adds recent-opponent-threat features plus "
            "reaction-window upweighting. Built to fix GY→poison / hog→tornado misses "
            "on real defense slices.",
        ),
        "4": (
            "Next-action policy — card-conditioned placement",
            "Keeps v3 threat/reaction and jointly trains zone/XY heads that see which "
            "card is being played. The offline probe that justified this is in the "
            "placement-probe report; the interactive comparison is in the showcase.",
        ),
    }
    return stories.get(version, stories["1"])


def _version_diagram_html(version: str, compute: dict[str, Any]) -> str:
    layers = compute.get("num_layers", "?")
    d_model = compute.get("d_model", "?")
    threat = int(compute.get("threat_dim", 0) or 0)
    card_place = bool(compute.get("card_conditioned_placement", version == "4"))

    def box(label: str, kind: str = "") -> str:
        return f'<div class="arch-box {kind}">{label}</div>'

    trunk = (
        f'<div class="arch-col">'
        f'<div class="arch-label">shared trunk</div>'
        f'{box(f"card embed + continuous → causal GRU ({layers}×{d_model})", "trunk")}'
        f'{box("deck matchup + global features", "trunk")}'
        f"</div>"
    )

    if version == "1":
        heads = (
            f'<div class="arch-col">'
            f'<div class="arch-label">heads (all card-blind)</div>'
            f'<div class="arch-heads">'
            f'{box("slot")}'
            f'{box("type")}'
            f'{box("zone + xy", "mute")}'
            f'{box("timing")}'
            f"</div>"
            f'<p class="arch-note">no cycle · no threat · placement does not see the card</p>'
            f"</div>"
        )
    elif version == "2":
        heads = (
            f'<div class="arch-col">'
            f'<div class="arch-label">what changed in v2</div>'
            f'<div class="arch-heads">'
            f'{box("slot (card-conditioned)", "live")}'
            f'{box("type")}'
            f'{box("zone + xy", "mute")}'
            f'{box("timing")}'
            f"</div>"
            f'<p class="arch-note up">+ per-card cycle features · slot head sees cycle state</p>'
            f"</div>"
        )
    elif version == "3":
        heads = (
            f'<div class="arch-col">'
            f'<div class="arch-label">what changed in v3</div>'
            f'{box(f"threat features ({threat}d) → global", "live")}'
            f'<div class="arch-heads">'
            f'{box("slot", "live")}'
            f'{box("type")}'
            f'{box("zone + xy", "mute")}'
            f'{box("timing")}'
            f"</div>"
            f'<p class="arch-note up">+ reaction-window upweighting · placement still card-blind</p>'
            f"</div>"
        )
    else:
        heads = (
            f'<div class="arch-col">'
            f'<div class="arch-label">what changed in v4</div>'
            f'{box(f"threat features ({threat}d) kept", "trunk")}'
            f'<div class="arch-heads">'
            f'{box("slot", "live")}'
            f'{box("type")}'
            f'{box("zone + xy ← card embed", "live")}'
            f'{box("timing")}'
            f"</div>"
            f'<p class="arch-note up">jointly trained card-conditioned placement '
            f'({"on" if card_place else "flag off"})</p>'
            f"</div>"
        )

    return (
        f'<div class="arch">{trunk}<div class="arch-arrow"></div>{heads}</div>'
    )


def render_policy_report(
    model_dir: str | Path = "models/policy_bc",
    output_path: str | Path | None = None,
) -> Path:
    model_dir = Path(model_dir)
    with (model_dir / "report.json").open(encoding="utf-8") as handle:
        import json

        report = json.load(handle)

    report_dir = Path(output_path).parent if output_path else Path("reports")
    report_dir.mkdir(parents=True, exist_ok=True)
    out = Path(output_path) if output_path else report_dir / "policy_bc_v1.html"

    created = _report_timestamp(model_dir, "report.json", "best_model.pt")
    data = report["data"]
    test = report["test"]
    compute = report["compute"]
    baselines = report.get("baselines", {})
    rollouts = report.get("rollouts", {})
    readiness = report.get("live_play_readiness", {})
    history = report.get("history", [])
    lessons = report.get("lessons", [])
    splits = data.get("splits", [])

    split_table = "".join(
        f"<tr><td>{html.escape(row['split'])}</td>"
        f"<td>{row['battles']:,}</td>"
        f"<td>{_fmt_pct(row['team_win_rate'])}</td>"
        f"<td>{row['mean_events']:.1f}</td></tr>"
        for row in splits
    )

    freq = baselines.get("frequency", {})
    cycle = baselines.get("cycle", {})
    checks = readiness.get("checks", {})
    check_rows = "".join(
        f"<tr><td>{html.escape(name)}</td><td>{'pass' if ok else 'fail'}</td></tr>"
        for name, ok in checks.items()
    )

    ready_badge = (
        "ready for live smoke test"
        if readiness.get("ready_for_live_smoke_test")
        else "keep iterating offline"
    )

    showcase = out.with_name(f"{out.stem}_showcase.html")
    showcase_link = (
        f'<p class="callout"><a href="{showcase.name}">Open the interactive showcase →</a> '
        "arena heatmaps, the placement league table and a defense quiz you can play against "
        "the model.</p>"
        if showcase.exists()
        else ""
    )
    version = _version_key(report, model_dir)
    headline, subcopy = _version_story(version)
    arch_html = _version_diagram_html(version, compute)

    body = f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Policy BC report — {html.escape(report.get("model_name", "policy-bc-v1"))}</title>
  <style>{_base_styles()}
    .callout {{
      margin-top: 16px; padding: 12px 16px; border-radius: 12px;
      background: rgba(34,211,238,0.1); border: 1px solid rgba(34,211,238,0.3);
      font-size: 0.9rem;
    }}
    .callout a {{ color: #67e8f9; font-weight: 700; }}
    .arch {{
      display: grid; grid-template-columns: 1fr 28px 1fr; gap: 12px; align-items: center;
      max-width: 820px;
    }}
    .arch-col {{
      padding: 14px; border-radius: 14px; background: rgba(148,163,184,0.06);
      border: 1px solid rgba(148,163,184,0.16);
    }}
    .arch-label {{
      font-size: 0.68rem; letter-spacing: 0.1em; text-transform: uppercase;
      color: #94a3b8; margin-bottom: 10px;
    }}
    .arch-box {{
      padding: 9px 12px; border-radius: 10px; text-align: center; font-size: 0.82rem;
      margin-bottom: 8px; background: rgba(99,102,241,0.16);
      border: 1px solid rgba(148,163,184,0.25);
    }}
    .arch-box.trunk {{ background: rgba(34,211,238,0.12); }}
    .arch-box.live {{ border-color: #34d399; box-shadow: 0 0 0 1px #34d39955; }}
    .arch-box.mute {{ opacity: 0.55; }}
    .arch-heads {{ display: grid; grid-template-columns: 1fr 1fr; gap: 8px; }}
    .arch-arrow {{
      width: 100%; height: 2px; background: linear-gradient(90deg, #22d3ee, #6366f1);
    }}
    .arch-note {{ font-size: 0.76rem; color: #94a3b8; margin: 8px 0 0; text-align: center; }}
    .arch-note.up {{ color: #6ee7b7; }}
    @media (max-width: 800px) {{
      .arch {{ grid-template-columns: 1fr; }}
      .arch-arrow {{ width: 2px; height: 24px; margin: 0 auto; }}
    }}
  </style>
</head>
<body>
  <main>
    <header class="report-header">
      <div class="badge-row">
        <span class="badge">{html.escape(report.get("model_name", "policy-bc-v1"))}</span>
        <span class="badge">behavior cloning</span>
        <span class="badge">v{version}</span>
        <span class="badge">{html.escape(str(compute.get("device", "cpu")))}</span>
        <span class="badge">{html.escape(ready_badge)}</span>
      </div>
      <h1>{html.escape(headline)}</h1>
      <p>{html.escape(subcopy)}</p>
      <p class="meta">Created {html.escape(created)} · trained in {report["seconds"]}s ·
      {compute.get("parameters", 0):,} params · {data["train_samples"]:,} train actions ·
      {data["battles_total"]:,} battles</p>
      {showcase_link}
    </header>

    <section class="report-section kpi-row">
      <div><span class="kpi-label">Test slot top-1</span><span class="kpi-value">{_fmt_pct(test["slot_top1"])}</span></div>
      <div><span class="kpi-label">Test slot top-3</span><span class="kpi-value">{_fmt_pct(test["slot_top3"])}</span></div>
      <div><span class="kpi-label">Zone accuracy</span><span class="kpi-value">{_fmt_pct(test.get("zone_acc"))}</span></div>
      <div><span class="kpi-label">Timing MAE</span><span class="kpi-value">{_fmt_float(test["timing_mae"], 2)}s</span></div>
    </section>

    <section class="report-section">
      <h2>What this version is trying to do</h2>
      {arch_html}
    </section>

    <section class="report-section block-grid">
      <div class="block">
        <h2>Test vs baselines</h2>
        <table>
          <thead><tr><th>Model</th><th>Slot top-1</th><th>Slot top-3</th><th>N</th></tr></thead>
          <tbody>
            <tr><td>Policy BC</td><td>{_fmt_pct(test["slot_top1"])}</td><td>{_fmt_pct(test["slot_top3"])}</td><td>{test["n"]:,}</td></tr>
            <tr><td>Online frequency</td><td>{_fmt_pct(freq.get("slot_top1"))}</td><td>{_fmt_pct(freq.get("slot_top3"))}</td><td>{freq.get("n", 0):,}</td></tr>
            <tr><td>Approx cycle</td><td>{_fmt_pct(cycle.get("slot_top1"))}</td><td>{_fmt_pct(cycle.get("slot_top3"))}</td><td>{cycle.get("n", 0):,}</td></tr>
            <tr><td>Chance (1/8)</td><td>{_fmt_pct(baselines.get("chance_slot_top1", 0.125))}</td><td>—</td><td>—</td></tr>
          </tbody>
        </table>
      </div>
      <div class="block">
        <h2>Placement &amp; type</h2>
        <table>
          <thead><tr><th>Metric</th><th>Value</th></tr></thead>
          <tbody>
            <tr><td>Type accuracy</td><td>{_fmt_pct(test["type_acc"])}</td></tr>
            <tr><td>Zone accuracy</td><td>{_fmt_pct(test.get("zone_acc"))}</td></tr>
            <tr><td>XY MAE (API units)</td><td>{_fmt_float(test["xy_mae"], 1)}</td></tr>
            <tr><td>Within 1 tile</td><td>{_fmt_pct(test["tile_acc"])}</td></tr>
            <tr><td>Timing MAE</td><td>{_fmt_float(test["timing_mae"], 2)} s</td></tr>
            <tr><td>Test loss</td><td>{_fmt_float(test["loss"])}</td></tr>
          </tbody>
        </table>
      </div>
      <div class="block">
        <h2>Battle splits</h2>
        <table>
          <thead><tr><th>Split</th><th>Battles</th><th>Team win rate</th><th>Mean events</th></tr></thead>
          <tbody>{split_table}</tbody>
        </table>
      </div>
      <div class="block">
        <h2>Live-play readiness</h2>
        <p class="caption">{html.escape(readiness.get("rationale", ""))}</p>
        <table>
          <thead><tr><th>Check</th><th>Status</th></tr></thead>
          <tbody>{check_rows or "<tr><td colspan='2'>No checks recorded</td></tr>"}</tbody>
        </table>
      </div>
    </section>

    <section class="report-section block-grid">
      <div class="block block-wide">
        <h2>Offline rollout realism</h2>
        <p class="caption">Policy continues real prefixes for ~40 actions; realism scorer rates finished sequences vs real / easy / medium negatives.</p>
        <table>
          <thead><tr><th>Source</th><th>Mean P(real)</th><th>N</th></tr></thead>
          <tbody>
            <tr><td>Real test battles</td><td>{_fmt_float(rollouts.get("mean_score_real"))}</td><td>{rollouts.get("n", 0)}</td></tr>
            <tr><td>Policy rollouts</td><td>{_fmt_float(rollouts.get("mean_score_policy"))}</td><td>{rollouts.get("n", 0)}</td></tr>
            <tr><td>Easy random</td><td>{_fmt_float(rollouts.get("mean_score_easy"))}</td><td>{rollouts.get("n", 0)}</td></tr>
            <tr><td>Medium random</td><td>{_fmt_float(rollouts.get("mean_score_medium"))}</td><td>{rollouts.get("n", 0)}</td></tr>
          </tbody>
        </table>
      </div>
    </section>

    <section class="report-section">
      <div class="chart-grid">
        <div class="chart-block">
          <h2>Training loss</h2>
          <svg class="chart" id="lossChart"></svg>
          <div class="legend"></div>
        </div>
        <div class="chart-block">
          <h2>Validation slot accuracy</h2>
          <svg class="chart" id="slotChart"></svg>
          <div class="legend"></div>
        </div>
        <div class="chart-block">
          <h2>Validation tile &amp; type</h2>
          <svg class="chart" id="auxChart"></svg>
          <div class="legend"></div>
        </div>
        <div class="chart-block">
          <h2>Rollout score comparison</h2>
          <svg class="chart" id="rolloutChart"></svg>
          <div class="legend"></div>
        </div>
      </div>
    </section>

    {(
        "<section class='report-section'>"
        "<h2>Training animation</h2>"
        "<p class='caption'>Validation metrics over epochs. Play or scrub.</p>"
        "<div class='chart-animation' id='policyTrainingAnim'>"
        "<svg class='chart' id='policyTrainingChart'></svg>"
        "<div class='legend'></div>"
        "<div class='anim-toolbar'>"
        "<button type='button' class='anim-btn anim-btn-icon' id='policyAnimToggle' aria-label='Play'>"
        "<svg class='anim-icon anim-icon-play' viewBox='0 0 16 16' aria-hidden='true'><path d='M4 2.5v11l9-5.5-9-5.5z'/></svg>"
        "<svg class='anim-icon anim-icon-pause' viewBox='0 0 16 16' aria-hidden='true' hidden><path d='M3.5 2h3v12h-3V2zm6 0h3v12h-3V2z'/></svg>"
        "</button>"
        "<input type='range' class='anim-scrubber' id='policyAnimScrubber' min='0' max='0' value='0' step='1'>"
        "<div class='anim-readout' id='policyAnimReadout'></div>"
        "</div>"
        "</div></section>"
    ) if history else ""}

    <section class="report-section">
      <h2>Lessons learned</h2>
      <ul class="lessons">
        {"".join(f"<li>{html.escape(lesson)}</li>" for lesson in lessons)}
      </ul>
    </section>
  </main>
  <script>
    {_chart_script()}
    const history = {_json_script(history)};
    const rollouts = {_json_script({k: v for k, v in rollouts.items() if k != "hist"})};
    const rolloutHist = {_json_script(report.get("rollout_hist") or {})};

    if (history.length) {{
      const labels = history.map((row) => String(row.epoch));
      renderLineChart("lossChart", [
        {{ color: "#60a5fa", label: "Train loss", values: history.map((r) => r.train_loss) }},
        {{ color: "#34d399", label: "Val loss", values: history.map((r) => r.val_loss) }},
      ], labels, {{ yFormat: "float" }});
      renderLineChart("slotChart", [
        {{ color: "#fbbf24", label: "Slot top-1", values: history.map((r) => r.val_slot_top1) }},
        {{ color: "#a78bfa", label: "Slot top-3", values: history.map((r) => r.val_slot_top3) }},
      ], labels, {{ yFormat: "percent" }});
      renderLineChart("auxChart", [
        {{ color: "#34d399", label: "Zone acc", values: history.map((r) => r.val_zone_acc ?? r.val_tile_acc) }},
        {{ color: "#60a5fa", label: "Tile acc", values: history.map((r) => r.val_tile_acc) }},
        {{ color: "#a78bfa", label: "Type acc", values: history.map((r) => r.val_type_acc) }},
      ], labels, {{ yFormat: "percent" }});
      mountPolicyTrainingAnimation({{
        svgId: "policyTrainingChart",
        stages: history,
        scrubberId: "policyAnimScrubber",
        toggleId: "policyAnimToggle",
        readoutId: "policyAnimReadout",
      }});
    }}

    const scoreBars = [
      {{ label: "real", value: rollouts.mean_score_real || 0 }},
      {{ label: "policy", value: rollouts.mean_score_policy || 0 }},
      {{ label: "easy", value: rollouts.mean_score_easy || 0 }},
      {{ label: "medium", value: rollouts.mean_score_medium || 0 }},
    ];
    renderLineChart("rolloutChart", [
      {{
        color: "#60a5fa",
        label: "Mean P(real)",
        values: scoreBars.map((row) => row.value),
      }},
    ], scoreBars.map((row) => row.label), {{ yFormat: "float" }});
    void rolloutHist;

    function mountPolicyTrainingAnimation(config) {{
      const svg = document.getElementById(config.svgId);
      const scrubber = document.getElementById(config.scrubberId);
      const toggle = document.getElementById(config.toggleId);
      const readout = document.getElementById(config.readoutId);
      const stages = config.stages || [];
      if (!svg || !scrubber || !toggle || !stages.length) return;

      let frame = stages.length - 1;
      let playing = false;
      let timer = null;
      const playIcon = toggle.querySelector(".anim-icon-play");
      const pauseIcon = toggle.querySelector(".anim-icon-pause");
      scrubber.max = String(stages.length - 1);
      scrubber.value = String(frame);

      function setPlaying(next) {{
        playing = next;
        if (playIcon) playIcon.hidden = playing;
        if (pauseIcon) pauseIcon.hidden = !playing;
        toggle.setAttribute("aria-label", playing ? "Pause" : "Play");
        if (playing) {{
          timer = window.setInterval(() => {{
            frame = (frame + 1) % stages.length;
            scrubber.value = String(frame);
            draw();
          }}, 220);
        }} else if (timer) {{
          window.clearInterval(timer);
          timer = null;
        }}
      }}

      function draw() {{
        const prefix = stages.slice(0, frame + 1);
        const labels = prefix.map((row) => String(row.epoch));
        renderLineChart(config.svgId, [
          {{ color: "#fbbf24", label: "Slot top-1", values: prefix.map((r) => r.val_slot_top1) }},
          {{ color: "#34d399", label: "Zone acc", values: prefix.map((r) => r.val_zone_acc ?? r.val_tile_acc) }},
          {{ color: "#60a5fa", label: "Tile acc", values: prefix.map((r) => r.val_tile_acc) }},
        ], labels, {{ yFormat: "percent" }});
        const current = stages[frame];
        readout.innerHTML = "Epoch <strong>" + current.epoch + "</strong> · "
          + "slot@1 <strong>" + (100 * current.val_slot_top1).toFixed(1) + "%</strong> · "
          + "zone <strong>" + (100 * (current.val_zone_acc ?? 0)).toFixed(1) + "%</strong> · "
          + "loss <strong>" + Number(current.val_loss).toFixed(3) + "</strong>";
      }}

      scrubber.addEventListener("input", () => {{
        frame = Number(scrubber.value);
        if (playing) setPlaying(false);
        draw();
      }});
      toggle.addEventListener("click", () => setPlaying(!playing));
      draw();
    }}
  </script>
</body>
</html>
"""
    out.write_text(body, encoding="utf-8")
    return out
