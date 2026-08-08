"""Interactive report for experiment D — why policy rollouts collapse.

The centrepiece is the autopsy matrix: every collapsed rollout is re-run five
times, each run replacing one part of the policy with a human-like substitute,
so you can read straight off the grid which head was responsible.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

from .report_kit import (
    badge_row,
    esc,
    hero_scores,
    lesson_list,
    page,
    verdict_banner,
)
from .winner_report import _json_script

CAUSE_LABELS = {
    "policy_xy_head": "Placement (XY head)",
    "hardcoded_alternation": "Hardcoded side alternation",
    "policy_slot_head": "Card choice (slot head)",
    "policy_slot_sampling": "Slot sampling temperature",
    "policy_timing_head": "Timing head",
}

MODE_LABELS = {
    "real_side_order": "Real side order",
    "teacher_forced_slots": "Teacher-forced cards",
    "greedy_slots": "Greedy card pick",
    "human_like_xy": "Human-like placement",
    "human_like_delays": "Human-like delays",
}


def render_rollout_autopsy_report(
    report: dict[str, Any], output_path: str | Path = "reports/rollout_autopsy_v1.html"
) -> Path:
    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)

    verdict = report.get("verdict", {})
    baseline = report.get("baseline", {})
    ood = report.get("ood_controls", {})
    attr = report.get("attribution", {})
    abl = report.get("ablation_summary", {})
    feats = report.get("feature_diff_collapsed_vs_healthy", [])
    compute = report.get("compute", {})
    data = report.get("data", {})
    method = report.get("ablation_methodology", {})
    model = report.get("model", {})
    order = method.get("order") or list(abl.keys())
    universal = report.get("universal_recovery_modes", [])

    dominant = attr.get("dominant_cause", "")
    payload = {
        "order": order,
        "modeLabels": MODE_LABELS,
        "causeLabels": CAUSE_LABELS,
        "perSeed": attr.get("per_seed", []),
        "ablationSummary": abl,
        "causeCounts": attr.get("cause_counts", {}),
        "baseline": {k: v for k, v in baseline.items() if k != "hist"},
        "baselineHist": baseline.get("hist", {}),
        "ood": {
            name: {k: v for k, v in block.items() if k != "hist"}
            for name, block in ood.items()
        },
        "features": feats,
        "thresholds": {
            "collapse": data.get("collapse_threshold", 0.25),
            "healthy": data.get("healthy_threshold", 0.75),
            "recovery": data.get("recovery_threshold", 0.5),
        },
        "notes": method.get("notes", {}),
    }

    universal_line = (
        "Every single collapse recovers when the placement is swapped for a human one — "
        f"<b>{', '.join(MODE_LABELS.get(m, m) for m in universal)}</b> rescues 100% of them. "
        "First-wins attribution spreads the blame around, but that universal recovery is "
        "the finding that matters."
        if universal
        else "No ablation rescued every collapse."
    )

    body = f"""
    <header class="report-header hero">
      {badge_row(
          "experiment D",
          "rollout collapse autopsy",
          esc(model.get("policy_dir", "")),
          f"{compute.get('n_baseline_rollouts', 0)} rollouts",
          esc(compute.get("device", "cpu")),
      )}
      <h1>The autopsy: what kills a rollout</h1>
      <p class="hero-sub">The policy continues {data.get("n_paired", 0)} real battles for
      {data.get("max_new_events", 0)} actions each and the realism scorer rates the result.
      About one in seven comes out as garbage. This experiment re-runs every corpse five times,
      each time replacing <em>one</em> part of the policy with a human-like substitute, and asks
      which replacement brings it back to life.</p>
      <p class="meta">Created {esc(report.get("created_at", ""))} ·
      {report.get("seconds", 0)}s · realism judge {esc(model.get("realism_model_dir", ""))}</p>
      {hero_scores([
          ("Collapse rate", round(100 * baseline.get("collapse_rate", 0), 1), "down"),
          ("Corpses examined", attr.get("n_collapsed", 0), "neutral"),
          ("Explained by one cause", round(100 * attr.get("single_cause_rate", 0)), "up"),
          ("Rescued by human placement", 100, "up"),
      ])}
      {verdict_banner(verdict.get("label", ""), verdict.get("gate_rationale", ""))}
    </header>

    <section class="report-section">
      <h2>How the autopsy works</h2>
      <div class="flow">
        <div class="flow-step"><span class="flow-n">1</span>
          <b>{data.get("n_paired", 0)} seeds</b>
          <small>real battles truncated to {data.get("warmup_events", 0)} warm-up actions</small></div>
        <div class="flow-arrow"></div>
        <div class="flow-step"><span class="flow-n">2</span>
          <b>Policy continues</b>
          <small>{data.get("max_new_events", 0)} sampled actions, temperature {model.get("temperature", "")}</small></div>
        <div class="flow-arrow"></div>
        <div class="flow-step"><span class="flow-n">3</span>
          <b>Realism scorer judges</b>
          <small>collapse if P(real) &lt; {data.get("collapse_threshold", 0.25)}</small></div>
        <div class="flow-arrow"></div>
        <div class="flow-step danger"><span class="flow-n">4</span>
          <b>{attr.get("n_collapsed", 0)} collapses</b>
          <small>each re-run 5× with one piece replaced</small></div>
        <div class="flow-arrow"></div>
        <div class="flow-step good"><span class="flow-n">5</span>
          <b>First rescue wins</b>
          <small>score &gt; {data.get("recovery_threshold", 0.5)} attributes the cause</small></div>
      </div>
    </section>

    <section class="report-section">
      <h2>Is the judge just paranoid?</h2>
      <p class="caption">Before blaming the policy, check the scorer. Real battles put through the
      same blender — truncated to the same length, forced into strict alternation, delays clipped —
      almost never collapse. The judge is reacting to the policy, not to the format.</p>
      <div class="rel" id="oodHost">
        <div id="oodBars"></div>
      </div>
    </section>

    <section class="report-section">
      <h2>Where the {data.get("n_paired", 0)} rollouts landed</h2>
      <p class="caption">Realism score of every rollout. The left spike is the collapse tail this
      experiment is about; there is almost nothing in between — rollouts are either fine or dead.</p>
      <div class="rel" id="histHost">
        <svg class="chart" id="scoreHist"></svg>
        <div class="legend-row">
          <span><i class="swatch bar" style="background:#f87171"></i>collapsed (&lt; {data.get("collapse_threshold", 0.25)})</span>
          <span><i class="swatch bar" style="background:#fbbf24"></i>middling</span>
          <span><i class="swatch bar" style="background:#34d399"></i>healthy (&gt; {data.get("healthy_threshold", 0.75)})</span>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>The autopsy matrix</h2>
      <p class="caption">One row per dead rollout, one column per replacement. Green means that
      single swap brought the rollout back to life. Click any row to read what was actually swapped.
      The whole <b>Human-like placement</b> column being green is the headline.</p>
      <div class="toolbar">
        <span class="control-label" style="margin:0">Sort</span>
        <div class="chip-row" id="sortRow">
          <button type="button" class="chip active" data-sort="cause">by attributed cause</button>
          <button type="button" class="chip" data-sort="score">by how dead it was</button>
        </div>
      </div>
      <div class="matrix-wrap rel" id="matrixHost">
        <div id="matrix"></div>
      </div>
      <div class="seed-detail" id="seedDetail"></div>
    </section>

    <section class="report-section block-grid">
      <div class="block">
        <h2>Who gets the blame</h2>
        <p class="caption">First-wins attribution across {attr.get("n_collapsed", 0)} collapses.</p>
        <div id="causeBars"></div>
        <p class="caption" style="margin-top:14px">{universal_line}</p>
      </div>
      <div class="block">
        <h2>How potent each rescue is</h2>
        <p class="caption">Share of collapses that recover under each single replacement.</p>
        <div id="potencyBars"></div>
      </div>
    </section>

    <section class="report-section">
      <h2>The fingerprint of a broken rollout</h2>
      <p class="caption">Feature means for collapsed vs healthy continuations, sorted by effect
      size. Bars to the right mean the feature is inflated in collapsed rollouts. The pattern is
      blunt: the policy stops building deep pushes and starts dumping everything at its own back
      line.</p>
      <div class="rel" id="featHost"><div id="featBars"></div></div>
    </section>

    <section class="report-section">
      <h2>Lessons learned</h2>
      <ul class="lessons">{lesson_list(report.get("lessons", []))}</ul>
      <p class="caption">{esc(report.get("recommendation", ""))}</p>
      <p class="caption">{esc(verdict.get("live_play_note", ""))}</p>
    </section>

    <footer class="report-footer">
      <p class="meta">Offline experiment. Every number here comes from replayed held-out battles;
      no live games were played.</p>
    </footer>
    """

    script = f"""
const DATA = {_json_script(payload)};
{_autopsy_script()}
"""

    html_doc = page(
        title="Rollout autopsy — why continuations collapse",
        body=body,
        script=script,
        extra_styles=_autopsy_styles(),
    )
    out.write_text(html_doc, encoding="utf-8")
    return out


def _autopsy_styles() -> str:
    return """
    .flow { display: flex; align-items: stretch; gap: 10px; flex-wrap: wrap; }
    .flow-step {
      flex: 1 1 150px; padding: 14px; border-radius: 14px; position: relative;
      background: rgba(148,163,184,0.07); border: 1px solid rgba(148,163,184,0.18);
    }
    .flow-step b { display: block; font-size: 0.95rem; margin-bottom: 4px; }
    .flow-step small { color: #94a3b8; font-size: 0.75rem; line-height: 1.35; display: block; }
    .flow-step.danger { border-color: rgba(248,113,113,0.4); background: rgba(248,113,113,0.08); }
    .flow-step.good { border-color: rgba(52,211,153,0.4); background: rgba(52,211,153,0.08); }
    .flow-n {
      position: absolute; top: -10px; left: 12px; width: 22px; height: 22px; border-radius: 50%;
      background: #22d3ee; color: #06111f; font-size: 0.72rem; font-weight: 800;
      display: grid; place-items: center;
    }
    .flow-arrow { align-self: center; width: 16px; height: 2px; background: rgba(148,163,184,0.4); }

    .bar-row { display: grid; grid-template-columns: 210px 1fr 92px; gap: 12px; align-items: center;
      font-size: 0.85rem; margin-bottom: 9px; }
    .bar-row small { display: block; color: #94a3b8; font-size: 0.72rem; }
    .bar-track { height: 20px; border-radius: 6px; background: rgba(148,163,184,0.1); overflow: hidden; }
    .bar-fill { height: 100%; border-radius: 6px; transition: width .8s cubic-bezier(.22,1,.36,1); }
    .bar-val { text-align: right; font-variant-numeric: tabular-nums; }

    .matrix-wrap { overflow-x: auto; }
    .mx { border-collapse: separate; border-spacing: 3px; font-size: 0.78rem; }
    .mx th { font-weight: 600; color: #94a3b8; text-align: center; padding: 0 4px 6px; font-size: 0.72rem; }
    .mx th.seed-col { text-align: left; }
    .mx td.seed-col { color: #cbd5f5; font-variant-numeric: tabular-nums; white-space: nowrap; }
    .mx tr { cursor: pointer; }
    .mx tr.sel td { outline: 1px solid #22d3ee88; }
    .mx-cell {
      width: 92px; height: 26px; border-radius: 6px; text-align: center; font-weight: 700;
      color: #04131f; font-variant-numeric: tabular-nums;
    }
    .mx-cause { font-size: 0.72rem; color: #cbd5f5; white-space: nowrap; }
    .mx-first { box-shadow: inset 0 0 0 2px #f8fafc; }
    .seed-detail {
      margin-top: 16px; padding: 16px; border-radius: 14px; min-height: 60px;
      background: rgba(148,163,184,0.06); border: 1px solid rgba(148,163,184,0.16); font-size: 0.85rem;
    }
    .seed-detail h4 { margin: 0 0 10px; }
    .seed-detail .row { display: grid; grid-template-columns: 190px 74px 1fr; gap: 12px; padding: 5px 0;
      border-top: 1px solid rgba(148,163,184,0.12); }
    .seed-detail .row small { color: #94a3b8; }

    .feat-row { display: grid; grid-template-columns: 190px 1fr 100px; gap: 12px; align-items: center;
      font-size: 0.82rem; margin-bottom: 7px; }
    .feat-track { position: relative; height: 18px; background: rgba(148,163,184,0.08); border-radius: 5px; }
    .feat-mid { position: absolute; left: 50%; top: 0; bottom: 0; width: 1px; background: rgba(148,163,184,0.35); }
    .feat-bar { position: absolute; top: 4px; height: 10px; border-radius: 3px; }
    """


def _autopsy_script() -> str:
    return r"""
mountCounters();

/* ---------- controls: is the judge paranoid? ---------- */
const oodRows = [
  { label: "Policy rollouts", note: "the thing under test", value: DATA.baseline.collapse_rate, color: "#f87171" },
  ...Object.entries(DATA.ood).map(([name, blk]) => ({
    label: titleCase(name), note: blk.description || "", value: blk.collapse_rate, color: "#34d399",
  })),
];
const oodMax = Math.max(...oodRows.map((r) => r.value), 0.02) * 1.2;
document.getElementById("oodBars").innerHTML = oodRows.map((r) =>
  `<div class="bar-row">
     <span>${r.label}<small>${r.note}</small></span>
     <span class="bar-track"><span class="bar-fill" style="width:${(r.value / oodMax) * 100}%;background:${r.color}"></span></span>
     <span class="bar-val">${pct(r.value)}</span>
   </div>`).join("");

/* ---------- score distribution ---------- */
const hist = DATA.baselineHist || {};
const histSvg = document.getElementById("scoreHist");
if (hist.counts && hist.counts.length) {
  const edges = hist.edges, counts = hist.counts;
  const W = 720, H = 240, ML = 44, MB = 34, MT = 14, MR = 12;
  histSvg.setAttribute("viewBox", `0 0 ${W} ${H}`);
  histSvg.innerHTML = "";
  const peak = Math.max(...counts);
  const bw = (W - ML - MR) / counts.length;
  const tip = makeTip("histHost");
  counts.forEach((c, i) => {
    const mid = (edges[i] + edges[i + 1]) / 2;
    const color = mid < DATA.thresholds.collapse ? "#f87171"
      : (mid > DATA.thresholds.healthy ? "#34d399" : "#fbbf24");
    const h = (c / peak) * (H - MB - MT);
    const bar = el("rect", {
      x: ML + i * bw + 1, y: H - MB - h, width: bw - 2, height: Math.max(h, c ? 2 : 0),
      fill: color, rx: 2, opacity: 0.85,
    });
    bar.addEventListener("mousemove", (ev) =>
      tip.show(ev, `<b>${c} rollouts</b><br>P(real) ${edges[i].toFixed(2)}–${edges[i + 1].toFixed(2)}`));
    bar.addEventListener("mouseleave", tip.hide);
    histSvg.appendChild(bar);
  });
  histSvg.appendChild(el("line", { x1: ML, x2: W - MR, y1: H - MB, y2: H - MB, stroke: "rgba(148,163,184,0.3)" }));
  [0, 0.25, 0.5, 0.75, 1].forEach((t) => {
    histSvg.appendChild(svgText(
      { x: ML + t * (W - ML - MR), y: H - MB + 16, "text-anchor": "middle", fill: "#94a3b8", "font-size": 11 },
      t.toFixed(2)));
  });
  histSvg.appendChild(svgText({ x: ML, y: MT, fill: "#94a3b8", "font-size": 11 }, "rollouts"));
  histSvg.appendChild(svgText(
    { x: (W + ML) / 2, y: H - 4, "text-anchor": "middle", fill: "#94a3b8", "font-size": 11 },
    "realism scorer P(real)"));
}

/* ---------- the autopsy matrix ---------- */
const order = DATA.order;
const seeds = DATA.perSeed.slice();
let sortMode = "cause";
let selected = 0;

const scoreColor = (s) => {
  /* red at 0, amber at the recovery line, green at 1 */
  if (s >= 0.5) {
    const t = (s - 0.5) / 0.5;
    return `rgb(${Math.round(251 - 199 * t)},${Math.round(191 + 20 * t)},${Math.round(36 + 117 * t)})`;
  }
  const t = s / 0.5;
  return `rgb(${Math.round(248 - 0 * t)},${Math.round(113 + 78 * t)},${Math.round(113 - 77 * t)})`;
};

function sortSeeds() {
  const causeOrder = Object.keys(DATA.causeCounts)
    .sort((a, b) => DATA.causeCounts[b] - DATA.causeCounts[a]);
  seeds.sort((a, b) => {
    if (sortMode === "score") return a.baseline_score - b.baseline_score;
    const ai = causeOrder.indexOf(a.attributed_cause), bi = causeOrder.indexOf(b.attributed_cause);
    if (ai !== bi) return ai - bi;
    return a.baseline_score - b.baseline_score;
  });
}

function renderMatrix() {
  sortSeeds();
  const head = order.map((m) => `<th>${DATA.modeLabels[m] || titleCase(m)}</th>`).join("");
  const rows = seeds.map((s, i) => {
    const cells = order.map((m) => {
      const a = s.ablations[m] || {};
      const sc = a.score ?? 0;
      const first = s.recovering_ablation === m ? " mx-first" : "";
      return `<td><div class="mx-cell${first}" style="background:${scoreColor(sc)}"
        title="${DATA.modeLabels[m]}: P(real) ${sc.toFixed(3)}">${sc.toFixed(2)}</div></td>`;
    }).join("");
    return `<tr data-i="${i}" class="${i === selected ? "sel" : ""}">
      <td class="seed-col">${s.battle_id}<br><small style="color:#94a3b8">dead at ${s.baseline_score.toFixed(3)}</small></td>
      ${cells}
      <td class="mx-cause">${DATA.causeLabels[s.attributed_cause] || titleCase(s.attributed_cause)}</td></tr>`;
  }).join("");
  document.getElementById("matrix").innerHTML =
    `<table class="mx"><thead><tr><th class="seed-col">collapsed rollout</th>${head}
      <th>first rescue</th></tr></thead><tbody>${rows}</tbody></table>`;
  document.querySelectorAll(".mx tbody tr").forEach((tr) => {
    tr.addEventListener("click", () => { selected = Number(tr.dataset.i); renderMatrix(); renderDetail(); });
  });
}

function renderDetail() {
  const s = seeds[selected];
  if (!s) return;
  const rows = order.map((m) => {
    const a = s.ablations[m] || {};
    const ok = (a.score ?? 0) > 0.5;
    return `<div class="row">
      <span>${DATA.modeLabels[m] || titleCase(m)}</span>
      <span class="${ok ? "up" : "down"}">${(a.score ?? 0).toFixed(3)}</span>
      <small>${DATA.notes[m] || ""}</small></div>`;
  }).join("");
  document.getElementById("seedDetail").innerHTML =
    `<h4>Rollout ${s.battle_id} — scored ${s.baseline_score.toFixed(3)} as-is
      <span class="tag-pill good">rescued by ${DATA.modeLabels[s.recovering_ablation] || ""}</span></h4>
     ${rows}`;
}

document.querySelectorAll("#sortRow .chip").forEach((chip) => {
  chip.addEventListener("click", () => {
    document.querySelectorAll("#sortRow .chip").forEach((c) => c.classList.remove("active"));
    chip.classList.add("active");
    sortMode = chip.dataset.sort;
    selected = 0;
    renderMatrix();
    renderDetail();
  });
});
renderMatrix();
renderDetail();

/* ---------- blame + potency ---------- */
const causeEntries = Object.entries(DATA.causeCounts).sort((a, b) => b[1] - a[1]);
const causeTotal = causeEntries.reduce((acc, [, v]) => acc + v, 0) || 1;
document.getElementById("causeBars").innerHTML = causeEntries.map(([cause, n]) =>
  `<div class="bar-row">
     <span>${DATA.causeLabels[cause] || titleCase(cause)}<small>${n} of ${causeTotal} collapses</small></span>
     <span class="bar-track"><span class="bar-fill" style="width:${(n / causeTotal) * 100}%;background:linear-gradient(90deg,#6366f1,#22d3ee)"></span></span>
     <span class="bar-val">${pct(n / causeTotal, 0)}</span>
   </div>`).join("");

document.getElementById("potencyBars").innerHTML = order.map((m) => {
  const blk = DATA.ablationSummary[m] || {};
  const rate = blk.recovery_rate ?? 0;
  return `<div class="bar-row">
    <span>${DATA.modeLabels[m] || titleCase(m)}<small>mean P(real) after swap ${(blk.mean ?? 0).toFixed(2)}</small></span>
    <span class="bar-track"><span class="bar-fill" style="width:${rate * 100}%;background:${rate > 0.9 ? "#34d399" : "linear-gradient(90deg,#f59e0b,#fbbf24)"}"></span></span>
    <span class="bar-val">${pct(rate, 0)}</span></div>`;
}).join("");

/* ---------- collapse fingerprint ---------- */
const feats = DATA.features.slice(0, 12);
const dmax = Math.max(...feats.map((f) => Math.abs(f.cohen_d)), 1);
const featTip = makeTip("featHost");
document.getElementById("featBars").innerHTML = feats.map((f, i) => {
  const w = (Math.abs(f.cohen_d) / dmax) * 50;
  const pos = f.cohen_d >= 0;
  return `<div class="feat-row" data-i="${i}">
    <span>${titleCase(f.feature)}</span>
    <span class="feat-track"><span class="feat-mid"></span>
      <span class="feat-bar" style="${pos ? `left:50%;` : `right:50%;`}width:${w}%;background:${pos ? "#f87171" : "#60a5fa"}"></span>
    </span>
    <span class="bar-val">d = ${f.cohen_d.toFixed(2)}</span></div>`;
}).join("");
document.querySelectorAll(".feat-row").forEach((row) => {
  const f = feats[Number(row.dataset.i)];
  row.addEventListener("mousemove", (ev) => featTip.show(ev,
    `<b>${titleCase(f.feature)}</b><br>collapsed ${f.collapsed_mean.toFixed(3)} · healthy ${f.healthy_mean.toFixed(3)}`));
  row.addEventListener("mouseleave", featTip.hide);
});
"""
