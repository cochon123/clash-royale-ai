"""Interactive report for real defense-slice evaluation.

Story: score the policy on held-out reaction windows — opponent just played a
win condition, human answered within N seconds. No forced hands, no synthetic
warmups. The visual is a threat radar (per-wincon accuracy vs frequency
baseline) and a cheap-vs-expensive response split.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

from .report_kit import (
    badge_row,
    esc,
    hero_scores,
    load_json,
    page,
    verdict_banner,
)
from .winner_report import _json_script


def render_defense_slice_report(
    slice_path: str | Path = "reports/defense_slice_eval.json",
    fair_probe_path: str | Path = "reports/defense_eval_fair.json",
    output_path: str | Path = "reports/defense_slice_v1.html",
) -> Path:
    slice_report = load_json(slice_path)
    if not slice_report:
        raise FileNotFoundError(slice_path)

    fair = load_json(fair_probe_path) if Path(fair_probe_path).exists() else {}
    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)

    overall = slice_report.get("overall", {})
    threats = slice_report.get("by_threat", [])
    cost = slice_report.get("by_response_cost", {})
    gates = slice_report.get("gates", {})
    setup = slice_report.get("setup", {})
    cheap = cost.get("cheap_le_2", {})
    expensive = cost.get("expensive_ge_4", {})

    payload = {
        "overall": overall,
        "threats": threats,
        "cheap": cheap,
        "expensive": expensive,
        "gates": gates,
        "fair": {
            "byThreat": fair.get("by_threat", []) if fair else [],
            "scenarios": fair.get("scenarios", []) if fair else [],
            "overall": fair.get("overall", {}) if fair else {},
        },
    }

    body = f"""
    <header class="report-header hero">
      {badge_row(
          esc(slice_report.get("model_name", "policy-bc")),
          "defense slice",
          f"{overall.get('n', 0):,} real windows",
          esc(slice_report.get("created_at", "")),
      )}
      <h1>Defense on real reaction windows</h1>
      <p class="hero-sub">{esc(setup.get("note",
          "Prefix is the real replay up to the human response. No synthetic warmup, no forced hand mask."))}</p>
      <p class="meta">Split: {esc(setup.get("split", "test"))} ·
      max response {setup.get("max_response_seconds", 5)}s ·
      threats: {esc(", ".join(setup.get("focus_threats", [])))}</p>
      {hero_scores([
          ("Top-1", round(100 * overall.get("top1", 0), 1), "up"),
          ("Top-3", round(100 * overall.get("top3", 0), 1), "up"),
          ("vs frequency", round(100 * (overall.get("top1", 0) - overall.get("freq_top1", 0)), 1), "up"),
          ("Mean P(target)", round(overall.get("mean_P_target", 0), 3), "neutral"),
      ])}
      {verdict_banner(
          "PASS" if gates.get("pass") else "FAIL",
          slice_report.get("verdict", "") if isinstance(slice_report.get("verdict"), str)
          else ("Gates passed." if gates.get("pass") else "Gates missed."),
      )}
    </header>

    <section class="report-section">
      <h2>Threat radar</h2>
      <p class="caption">Each spoke is a win-condition threat. The outer ring is perfect top-1;
      the filled wedge is the policy; the dashed ring is the online-frequency baseline.
      Click a threat to pin its numbers.</p>
      <div class="radar-stage">
        <svg id="radar" viewBox="0 0 420 420" class="radar"></svg>
        <div class="stat-grid" id="threatStats"></div>
      </div>
    </section>

    <section class="report-section">
      <h2>Policy vs frequency, per threat</h2>
      <p class="caption">Hit <em>Run the diff</em> to watch each threat climb from the
      frequency baseline to the policy.</p>
      <div class="toolbar">
        <button type="button" class="play-btn" id="threatRaceBtn">▶ Run the diff</button>
        <span class="hint" id="threatRaceHint">bars parked at frequency baseline</span>
      </div>
      <div class="race" id="threatRace"></div>
    </section>

    <section class="report-section block-grid">
      <div class="block">
        <h2>Cheap answers (≤2 elixir)</h2>
        <p class="caption">Tornado, skeletons, spirits — the fast cycle answers.
        n = {cheap.get("n", 0):,}</p>
        <div class="stat-grid" id="cheapStats"></div>
      </div>
      <div class="block">
        <h2>Expensive answers (≥4 elixir)</h2>
        <p class="caption">Valkyrie, inferno, PEKKA — the committed answers.
        n = {expensive.get("n", 0):,}</p>
        <div class="stat-grid" id="expStats"></div>
      </div>
    </section>

    <section class="report-section">
      <h2>Cost honesty</h2>
      <p class="caption">Does the model dump cheap cards when the human spent big, or match
      the spend? Bars compare mean predicted cost vs mean human response cost.</p>
      <div id="costBars"></div>
    </section>

    {"<section class='report-section'><h2>Fair synthetic probe (optional)</h2>"
     "<p class='caption'>Forced-hand counterfactuals — useful as a stress test, not the "
     "primary metric. Real windows above are the source of truth.</p>"
     "<div id='fairBlock'></div></section>" if fair else ""}

    <section class="report-section">
      <h2>Gates</h2>
      <div class="gate-grid" id="gates"></div>
    </section>

    <footer class="report-footer">
      <p class="meta">Offline evaluation · source {esc(str(slice_path))}</p>
    </footer>
    """

    script = f"""
const DATA = {_json_script(payload)};
{_slice_script()}
"""
    html_doc = page(
        title="Defense slice — real reaction windows",
        body=body,
        script=script,
        extra_styles=_slice_styles(),
    )
    out.write_text(html_doc, encoding="utf-8")
    return out


def _slice_styles() -> str:
    return """
    .radar { width: 100%; max-width: 420px; height: auto; display: block; }
    .radar-stage { display: grid; grid-template-columns: 420px 1fr; gap: 24px; align-items: center; }
    .gate-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 12px; }
    .gate-card {
      padding: 12px 14px; border-radius: 12px; border: 1px solid rgba(148,163,184,0.18);
      background: rgba(148,163,184,0.06); font-size: 0.85rem;
    }
    .gate-card.pass { border-color: rgba(52,211,153,0.4); }
    .gate-card.fail { border-color: rgba(248,113,113,0.4); }
    .gate-card .g-status { font-weight: 800; letter-spacing: 0.06em; font-size: 0.72rem; margin-bottom: 4px; }
    .cost-row {
      display: grid; grid-template-columns: 160px 1fr 90px; gap: 12px; align-items: center;
      font-size: 0.85rem; margin-bottom: 10px;
    }
    .cost-track { position: relative; height: 22px; background: rgba(148,163,184,0.1); border-radius: 6px; }
    .cost-human { position: absolute; top: 2px; height: 8px; border-radius: 4px; background: #f8fafc88; }
    .cost-pred { position: absolute; top: 12px; height: 8px; border-radius: 4px;
      background: linear-gradient(90deg,#34d399,#22d3ee); }
    .fair-row {
      display: grid; grid-template-columns: 1.2fr 80px 80px 1fr; gap: 10px; font-size: 0.82rem;
      padding: 6px 0; border-bottom: 1px solid rgba(148,163,184,0.12);
    }
    @media (max-width: 900px) { .radar-stage { grid-template-columns: 1fr; } }
    """


def _slice_script() -> str:
    return r"""
mountCounters();

const threats = DATA.threats || [];
let pinned = 0;

/* ---------- radar ---------- */
(function drawRadar() {
  const svg = document.getElementById("radar");
  const cx = 210, cy = 210, R = 150;
  const n = Math.max(threats.length, 3);
  const angle = (i) => -Math.PI / 2 + (i / n) * Math.PI * 2;
  const pt = (i, r) => [cx + R * r * Math.cos(angle(i)), cy + R * r * Math.sin(angle(i))];

  // rings
  [0.25, 0.5, 0.75, 1].forEach((r) => {
    const pts = Array.from({ length: n }, (_, i) => pt(i, r).join(",")).join(" ");
    svg.appendChild(el("polygon", {
      points: pts, fill: "none", stroke: "rgba(148,163,184,0.18)", "stroke-width": 1,
    }));
  });

  // frequency baseline (dashed)
  const freqPts = threats.map((t, i) => pt(i, t.freq_top1 || 0).join(",")).join(" ");
  svg.appendChild(el("polygon", {
    points: freqPts, fill: "rgba(248,113,113,0.08)", stroke: "#f87171",
    "stroke-width": 1.5, "stroke-dasharray": "4 3",
  }));

  // policy
  const polPts = threats.map((t, i) => pt(i, t.top1 || 0).join(",")).join(" ");
  svg.appendChild(el("polygon", {
    points: polPts, fill: "rgba(34,211,238,0.18)", stroke: "#22d3ee", "stroke-width": 2.2,
  }));

  threats.forEach((t, i) => {
    const [x, y] = pt(i, 1.12);
    const [dx, dy] = pt(i, t.top1 || 0);
    svg.appendChild(el("circle", {
      cx: dx, cy: dy, r: 5, fill: "#22d3ee", stroke: "#04131f", "stroke-width": 1.5,
      class: "radar-dot", "data-i": i, style: "cursor:pointer",
    }));
    svg.appendChild(svgText({
      x, y: y + 4, "text-anchor": "middle", fill: "#cbd5f5", "font-size": 12, "font-weight": 600,
      style: "cursor:pointer",
    }, titleCase(t.threat)));
  });

  svg.querySelectorAll("[data-i]").forEach((node) => {
    node.addEventListener("click", () => { pinned = Number(node.dataset.i); renderThreatStats(); });
  });
  // also make labels clickable roughly by re-querying texts — skip, dots enough
})();

function renderThreatStats() {
  const t = threats[pinned] || threats[0] || {};
  document.getElementById("threatStats").innerHTML = [
    ["Threat", titleCase(t.threat || "—"), `${(t.n || 0).toLocaleString()} windows`],
    ["Policy top-1", pct(t.top1 || 0), "what we care about"],
    ["Frequency top-1", pct(t.freq_top1 || 0), "online baseline"],
    ["Lift", ((t.top1 || 0) - (t.freq_top1 || 0) >= 0 ? "+" : "") +
      (((t.top1 || 0) - (t.freq_top1 || 0)) * 100).toFixed(1) + "pp", "policy − frequency"],
    ["Top-3", pct(t.top3 || 0), "softer"],
    ["Mean P(target)", (t.mean_P_target || 0).toFixed(3), "confidence on the human card"],
    ["Mean delay", (t.mean_delay || 0).toFixed(2) + "s", "human response time"],
    ["Cheap pred rate", pct(t.cheap_pred_rate || 0), "model plays ≤2 elixir"],
  ].map(([k, v, s]) =>
    `<div class="stat"><div class="k">${k}</div><div class="v">${v}</div><div class="s">${s}</div></div>`
  ).join("");
}
renderThreatStats();

/* ---------- threat race ---------- */
mountRace("threatRace", threats.map((t) => ({
  label: titleCase(t.threat),
  note: `${(t.n || 0).toLocaleString()} windows · top-3 ${pct(t.top3 || 0)}`,
  old: t.freq_top1 || 0,
  new: t.top1 || 0,
  fmt: "pct",
  higher: true,
})), {
  buttonId: "threatRaceBtn", hintId: "threatRaceHint", oldName: "freq",
  playLabel: "▶ Run the diff", backLabel: "↺ Back to frequency",
  hintBefore: "bars parked at frequency baseline",
  hintAfter: "bars moved to the policy",
});

/* ---------- cost buckets ---------- */
function fillCost(id, blk) {
  document.getElementById(id).innerHTML = [
    ["Top-1", pct(blk.top1 || 0), ""],
    ["Top-3", pct(blk.top3 || 0), ""],
    ["Mean P(target)", (blk.mean_P_target || 0).toFixed(3), ""],
    ["Mean human cost", (blk.mean_response_cost || 0).toFixed(2) + "⚡", ""],
    ["Mean pred cost", (blk.mean_pred_cost || 0).toFixed(2) + "⚡", ""],
    ["Cheap pred rate", pct(blk.cheap_pred_rate || 0), "model ≤2⚡"],
  ].map(([k, v, s]) =>
    `<div class="stat"><div class="k">${k}</div><div class="v">${v}</div><div class="s">${s}</div></div>`
  ).join("");
}
fillCost("cheapStats", DATA.cheap || {});
fillCost("expStats", DATA.expensive || {});

const costMax = Math.max(
  DATA.overall.mean_response_cost || 0, DATA.overall.mean_pred_cost || 0,
  DATA.cheap.mean_response_cost || 0, DATA.cheap.mean_pred_cost || 0,
  DATA.expensive.mean_response_cost || 0, DATA.expensive.mean_pred_cost || 0,
  6,
);
document.getElementById("costBars").innerHTML = [
  ["All windows", DATA.overall],
  ["Cheap answers", DATA.cheap],
  ["Expensive answers", DATA.expensive],
].map(([label, blk]) =>
  `<div class="cost-row">
     <span>${label}<br><small style="color:#94a3b8">n=${(blk.n || 0).toLocaleString()}</small></span>
     <span class="cost-track">
       <span class="cost-human" style="width:${((blk.mean_response_cost || 0) / costMax) * 100}%"></span>
       <span class="cost-pred" style="width:${((blk.mean_pred_cost || 0) / costMax) * 100}%"></span>
     </span>
     <span style="text-align:right;font-size:.78rem">hum ${(blk.mean_response_cost || 0).toFixed(1)}⚡
       <br>pred ${(blk.mean_pred_cost || 0).toFixed(1)}⚡</span>
   </div>`
).join("") + `<div class="legend-row" style="margin-top:8px">
  <span><i class="swatch bar" style="background:#f8fafc88"></i>human response cost</span>
  <span><i class="swatch bar" style="background:#34d399"></i>model predicted cost</span>
</div>`;

/* ---------- fair probe ---------- */
const fairHost = document.getElementById("fairBlock");
if (fairHost) {
  const sc = DATA.fair.scenarios || [];
  fairHost.innerHTML = sc.length
    ? sc.map((s) =>
        `<div class="fair-row">
           <span><b>${titleCase(s.scenario || s.strong || "")}</b>
             <br><small style="color:#94a3b8">${titleCase(s.threat)} · strong ${titleCase(s.strong)}</small></span>
           <span>${pct(s.top1_strong || 0)}</span>
           <span>${(s.mean_P_strong_among_hand || 0).toFixed(2)}</span>
           <span style="color:#94a3b8;font-size:.75rem">wrong: ${JSON.stringify(s.wrong_pick_counts || {})}</span>
         </div>`
      ).join("")
    : "<p class='caption'>No fair-probe scenarios on disk for this run.</p>";
}

/* ---------- gates ---------- */
document.getElementById("gates").innerHTML = Object.entries(DATA.gates || {}).map(([k, v]) => {
  if (k === "pass") return "";
  return `<div class="gate-card ${v ? "pass" : "fail"}">
    <div class="g-status ${v ? "up" : "down"}">${v ? "PASS" : "FAIL"}</div>
    <div>${titleCase(k)}</div></div>`;
}).join("");
"""
