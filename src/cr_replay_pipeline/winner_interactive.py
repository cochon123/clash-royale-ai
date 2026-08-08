"""Interactive HTML reports for the winner predictors.

HGB ensemble: full-game tabular judge with a risk-coverage dial (abstain when
unsure) and duration buckets. Transformer: prefix-aware sequence model — the
visual is "how early can you call the match".
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

from .report_kit import (
    badge_row,
    esc,
    hero_scores,
    lesson_list,
    load_json,
    page,
    verdict_banner,
)
from .winner_report import _json_script, _report_timestamp


def render_hgb_interactive(
    model_dir: str | Path = "models/winner_predictor",
    output_path: str | Path | None = None,
) -> Path:
    model_dir = Path(model_dir)
    report = load_json(model_dir / "hgb_report.json")
    if not report:
        raise FileNotFoundError(model_dir / "hgb_report.json")

    out = Path(output_path) if output_path else Path("reports/winner_hgb_v1.html")
    out.parent.mkdir(parents=True, exist_ok=True)
    created = _report_timestamp(model_dir, "hgb_report.json", "hgb_ensemble.pkl")

    curve = report.get("confidence_curve", [])
    base_curve = report.get("baseline_confidence_curve", [])
    stages = report.get("training_stages", [])
    durations = report.get("test_by_duration", [])
    conf = report.get("confidence", {})
    blend = report.get("blend", {})

    payload = {
        "test": report.get("test", {}),
        "val": report.get("val", {}),
        "baseline": report.get("baseline", {}),
        "curve": curve,
        "baseCurve": base_curve,
        "stages": stages,
        "durations": durations,
        "confidence": conf,
        "blend": blend,
    }

    lessons = [
        "Replay sequences alone reach ~79% full-game winner accuracy — no live arena state needed.",
        "Perspective symmetry (train both sides, average at inference) removes team/opponent orientation bias.",
        "Winner probability and confidence use separate blend weights; accuracy and selective prediction disagree.",
        "Short games (<2 min) are nearly solved; mid-length games remain the hard bucket.",
    ]

    body = f"""
    <header class="report-header hero">
      {badge_row(
          "winner-hgb-v1",
          "tabular ensemble",
          f"{report.get('feature_dim', '?')} features",
          f"{report.get('battles_total', 0):,} battles",
          "CPU",
      )}
      <h1>Who wins this replay?</h1>
      <p class="hero-sub">Perspective-symmetric blend of HistGradientBoosting and Extra Trees.
      Predicts the winner from decks, elixir/leak tables, and the ordered card-play sequence —
      the offline judge we use for matchup stress tests.</p>
      <p class="meta">Created {esc(created)} · trained in {report.get("seconds", 0)}s ·
      HGB blend {_pct(blend.get("hgb_weight"))} / trees {_pct(blend.get("extra_trees_weight"))}</p>
      {hero_scores([
          ("Test accuracy", round(100 * report["test"]["acc"], 1), "up"),
          ("Test AUC", round(report["test"]["auc"], 3), "up"),
          ("Baseline acc", round(100 * report["baseline"]["test"]["acc"], 1), "neutral"),
          ("AURC (lower better)", round(conf.get("selected_score_test_aurc", 0), 3), "up"),
      ])}
      {verdict_banner("OFFLINE JUDGE",
          "Good enough to score policy-vs-policy matchups. Not a live-play agent.")}
    </header>

    <section class="report-section">
      <h2>Risk–coverage dial</h2>
      <p class="caption">The model can abstain when its confidence is low. Drag the threshold:
      coverage is how often it answers; accuracy is how often those answers are right.
      A useful judge climbs above the diagonal.</p>
      <div class="dial">
        <input type="range" id="confScrub" min="50" max="99" value="50" step="1" class="anim-scrubber">
        <div class="stat-grid" id="confStats"></div>
      </div>
      <svg class="chart" id="rcChart" style="margin-top:16px"></svg>
      <div class="legend-row">
        <span><i class="swatch bar" style="background:#22d3ee"></i>selected confidence score</span>
        <span><i class="swatch bar" style="background:#f87171"></i>raw-margin baseline</span>
      </div>
    </section>

    <section class="report-section">
      <h2>When is the match already decided?</h2>
      <p class="caption">Accuracy by game length. Short steamrolls are easy; the 2–3 minute
      midgame is where the model still guesses.</p>
      <div id="durationBars"></div>
    </section>

    <section class="report-section">
      <h2>Vs the majority baseline</h2>
      <div class="toolbar">
        <button type="button" class="play-btn" id="raceBtn">▶ Run the diff</button>
        <span class="hint" id="raceHint">bars parked at baseline</span>
      </div>
      <div class="race" id="race"></div>
    </section>

    <section class="report-section">
      <h2>Boosting stages</h2>
      <div class="chart-animation">
        <svg class="chart" id="trainChart"></svg>
        <div class="legend" id="trainLegend"></div>
        <div class="anim-toolbar">
          <button type="button" class="play-btn" id="trainPlay">▶</button>
          <input type="range" class="anim-scrubber" id="trainScrub" min="1" max="1" value="1">
          <div class="anim-readout" id="trainReadout"></div>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>Lessons learned</h2>
      <ul class="lessons">{lesson_list(lessons)}</ul>
      <p class="caption">{esc(report.get("notes", ""))}</p>
    </section>

    <footer class="report-footer">
      <p class="meta">Offline judge · checkpoint {esc(report.get("checkpoint", ""))}</p>
    </footer>
    """

    script = f"const DATA = {_json_script(payload)};\n{_hgb_script()}"
    out.write_text(
        page(
            title="Winner HGB — who wins this replay?",
            body=body,
            script=script,
            extra_styles=_winner_styles(),
        ),
        encoding="utf-8",
    )
    return out


def render_transformer_interactive(
    model_dir: str | Path = "models/winner_predictor",
    output_path: str | Path | None = None,
) -> Path:
    model_dir = Path(model_dir)
    report = load_json(model_dir / "report.json")
    if not report:
        raise FileNotFoundError(model_dir / "report.json")

    out = Path(output_path) if output_path else Path("reports/winner_transformer_v1.html")
    out.parent.mkdir(parents=True, exist_ok=True)
    created = _report_timestamp(model_dir, "report.json", "best_model.pt")

    history = report.get("history", {})
    by_prefix = report.get("test_by_prefix_ratio", {})
    # history may be dict of lists
    epochs = len(history.get("val_auc", history.get("train_loss", [])))

    payload = {
        "test": report.get("test", {}),
        "byPrefix": by_prefix,
        "history": history,
        "majority": report.get("majority_baseline", 0.5),
        "bestValAuc": report.get("best_val_auc"),
        "epochs": epochs,
    }

    lessons = [
        "Prefix accuracy rises smoothly with how much of the match the model has seen.",
        "Full-game AUC ~0.80 is competitive with the tabular ensemble; early prefixes are near chance.",
        "Useful as a 'when is this match decided?' probe, not as a live agent.",
    ]

    body = f"""
    <header class="report-header hero">
      {badge_row(
          "winner-transformer-v1",
          "causal sequence model",
          esc(report.get("device", "cuda")),
          f"{report.get('battles_total', 0):,} battles",
          esc(report.get("gpu_name", "")),
      )}
      <h1>How early can you call the match?</h1>
      <p class="hero-sub">A causal transformer over the action sequence. Same winner target as
      the HGB ensemble, but scored at 50% / 75% / 90% / 100% of the match so you can see when
      the outcome becomes knowable.</p>
      <p class="meta">Created {esc(created)} · trained in {report.get("seconds", 0)}s ·
      best val full AUC {report.get("best_val_auc", 0):.3f}</p>
      {hero_scores([
          ("Test accuracy", round(100 * report["test"]["acc"], 1), "up"),
          ("Test AUC", round(report["test"]["auc"], 3), "up"),
          ("Full-game AUC", round(by_prefix.get("1.0", {}).get("auc", 0), 3), "up"),
          ("Half-match AUC", round(by_prefix.get("0.5", {}).get("auc", 0), 3), "neutral"),
      ])}
      {verdict_banner("PREFIX PROBE",
          "Strong at the end of the match; near chance at halfway. Confirms outcome signal "
          "accumulates — it does not justify early-concede automation.")}
    </header>

    <section class="report-section">
      <h2>The match timeline</h2>
      <p class="caption">Scrub how much of the battle the model has seen. The gauge shows
      accuracy and AUC at that prefix ratio.</p>
      <div class="toolbar">
        <input type="range" id="prefixScrub" min="0" max="3" value="3" step="1" class="anim-scrubber">
        <div class="anim-readout" id="prefixReadout"></div>
      </div>
      <div class="timeline" id="timeline"></div>
      <div class="stat-grid" id="prefixStats" style="margin-top:16px"></div>
    </section>

    <section class="report-section">
      <h2>Prefix ladder</h2>
      <div class="toolbar">
        <button type="button" class="play-btn" id="raceBtn">▶ Run the diff</button>
        <span class="hint" id="raceHint">bars parked at majority baseline</span>
      </div>
      <div class="race" id="race"></div>
    </section>

    <section class="report-section">
      <h2>Training</h2>
      <div class="chart-animation">
        <svg class="chart" id="trainChart"></svg>
        <div class="legend" id="trainLegend"></div>
        <div class="anim-toolbar">
          <button type="button" class="play-btn" id="trainPlay">▶</button>
          <input type="range" class="anim-scrubber" id="trainScrub" min="1" max="1" value="1">
          <div class="anim-readout" id="trainReadout"></div>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>Lessons learned</h2>
      <ul class="lessons">{lesson_list(lessons)}</ul>
    </section>

    <footer class="report-footer">
      <p class="meta">Offline probe · checkpoint {esc(report.get("checkpoint", ""))}</p>
    </footer>
    """

    script = f"const DATA = {_json_script(payload)};\n{_transformer_script()}"
    out.write_text(
        page(
            title="Winner transformer — how early can you call it?",
            body=body,
            script=script,
            extra_styles=_winner_styles(),
        ),
        encoding="utf-8",
    )
    return out


def _pct(v: Any) -> str:
    if v is None:
        return "—"
    return f"{100 * float(v):.0f}%"


def _winner_styles() -> str:
    return """
    .dial { margin-top: 8px; }
    .timeline {
      display: flex; gap: 4px; height: 28px; border-radius: 8px; overflow: hidden;
      background: rgba(148,163,184,0.1); margin-top: 12px;
    }
    .timeline span { display: block; height: 100%; transition: flex .3s ease, background .3s; }
    .dur-row {
      display: grid; grid-template-columns: 120px 1fr 80px; gap: 12px; align-items: center;
      font-size: 0.85rem; margin-bottom: 8px;
    }
    .dur-track { height: 18px; border-radius: 6px; background: rgba(148,163,184,0.1); overflow: hidden; }
    .dur-fill { height: 100%; border-radius: 6px; background: linear-gradient(90deg,#6366f1,#22d3ee); }
    """


def _hgb_script() -> str:
    return r"""
mountCounters();

const curve = DATA.curve || [];
const baseCurve = DATA.baseCurve || [];

function nearest(curve, conf) {
  if (!curve.length) return null;
  let best = curve[0], bd = 1e9;
  for (const row of curve) {
    const d = Math.abs((row.min_confidence || 0) - conf);
    if (d < bd) { bd = d; best = row; }
  }
  return best;
}

function renderConf() {
  const conf = Number(document.getElementById("confScrub").value) / 100;
  const row = nearest(curve, conf) || {};
  const base = nearest(baseCurve, conf) || {};
  document.getElementById("confStats").innerHTML = [
    ["Min confidence", pct(conf, 0), "threshold"],
    ["Coverage", pct(row.coverage || 0), `${row.n || 0} battles answered`],
    ["Accuracy on answers", pct(row.accuracy || 0), "selected score"],
    ["Baseline accuracy", pct(base.accuracy || 0), "raw margin at same coverage"],
  ].map(([k,v,s]) =>
    `<div class="stat"><div class="k">${k}</div><div class="v">${v}</div><div class="s">${s}</div></div>`
  ).join("");

  // risk-coverage style: accuracy vs coverage
  const svg = document.getElementById("rcChart");
  const series = [
    { label: "selected", color: "#22d3ee",
      values: curve.map((r) => r.accuracy),
      xs: curve.map((r) => r.coverage) },
  ];
  // custom draw: coverage on x, accuracy on y
  const W = 720, H = 260, ML = 48, MB = 34, MT = 12, MR = 14;
  svg.setAttribute("viewBox", `0 0 ${W} ${H}`);
  svg.innerHTML = "";
  const pts = curve.map((r) => [
    ML + (1 - (r.coverage || 0)) * (W - ML - MR),  // risk = 1-coverage on x sometimes; we want coverage
    H - MB - ((r.accuracy || 0) - 0.7) / 0.3 * (H - MB - MT),
  ]);
  // better: coverage on x from 0..1, accuracy from 0.7..1
  const px = (c) => ML + c * (W - ML - MR);
  const py = (a) => H - MB - ((a - 0.7) / 0.3) * (H - MB - MT);
  for (let t = 0; t <= 3; t++) {
    const a = 0.7 + t * 0.1;
    svg.appendChild(el("line", { x1: ML, x2: W - MR, y1: py(a), y2: py(a), stroke: "rgba(148,163,184,0.14)" }));
    svg.appendChild(svgText({ x: ML - 8, y: py(a) + 4, "text-anchor": "end", fill: "#94a3b8", "font-size": 11 }, pct(a, 0)));
  }
  const line = (rows, color, dash) => {
    const p = rows.map((r) => `${px(r.coverage)},${py(r.accuracy)}`).join(" ");
    svg.appendChild(el("polyline", {
      points: p, fill: "none", stroke: color, "stroke-width": 2.2, "stroke-dasharray": dash || "none",
    }));
  };
  line(baseCurve, "#f87171", "4 3");
  line(curve, "#22d3ee");
  // cursor
  svg.appendChild(el("line", {
    x1: px(row.coverage || 0), x2: px(row.coverage || 0), y1: MT, y2: H - MB,
    stroke: "#fbbf24", "stroke-width": 1.5, "stroke-dasharray": "3 3",
  }));
  svg.appendChild(el("circle", {
    cx: px(row.coverage || 0), cy: py(row.accuracy || 0.75), r: 5, fill: "#fbbf24",
  }));
  svg.appendChild(svgText({
    x: (W + ML) / 2, y: H - 6, "text-anchor": "middle", fill: "#94a3b8", "font-size": 11,
  }, "coverage (share of battles answered) →"));
}
document.getElementById("confScrub").addEventListener("input", renderConf);
renderConf();

/* duration bars */
const durs = DATA.durations || [];
const dmax = Math.max(...durs.map((d) => d.accuracy || 0), 0.5);
document.getElementById("durationBars").innerHTML = durs.map((d) =>
  `<div class="dur-row">
     <span>${d.duration}<br><small style="color:#94a3b8">n=${d.n}</small></span>
     <span class="dur-track"><span class="dur-fill" style="width:${((d.accuracy || 0) / dmax) * 100}%"></span></span>
     <span style="text-align:right">${pct(d.accuracy || 0)}</span>
   </div>`
).join("");

mountRace("race", [
  { label: "Test accuracy", note: "Full-game winner call.",
    old: (DATA.baseline.test || {}).acc || 0.5, new: DATA.test.acc || 0, fmt: "pct", higher: true },
  { label: "Test AUC", note: "Ranking quality.",
    old: (DATA.baseline.test || {}).auc || 0.5, new: DATA.test.auc || 0, fmt: "float", higher: true },
], {
  buttonId: "raceBtn", hintId: "raceHint", oldName: "base",
  playLabel: "▶ Run the diff", backLabel: "↺ Back to baseline",
  hintBefore: "bars parked at baseline", hintAfter: "bars moved to the ensemble",
});

const stages = DATA.stages || [];
const scrub = document.getElementById("trainScrub");
scrub.max = Math.max(stages.length, 1);
scrub.value = stages.length;
function drawTrain(upto) {
  const slice = stages.slice(0, upto);
  lineChart(document.getElementById("trainChart"), [
    { label: "accuracy", color: "#22d3ee", values: slice.map((s) => s.accuracy) },
    { label: "AUC", color: "#34d399", values: slice.map((s) => s.auc) },
  ], { yFormat: "percent", xLabel: "trees" });
  const last = slice[slice.length - 1];
  document.getElementById("trainReadout").textContent = last
    ? `trees ${last.trees} · acc ${pct(last.accuracy)} · AUC ${last.auc.toFixed(3)}`
    : "";
}
document.getElementById("trainLegend").innerHTML = legendHTML([
  { label: "accuracy", color: "#22d3ee" }, { label: "AUC", color: "#34d399" },
]);
drawTrain(stages.length);
scrub.addEventListener("input", () => drawTrain(Number(scrub.value)));
let timer = null;
document.getElementById("trainPlay").addEventListener("click", () => {
  const btn = document.getElementById("trainPlay");
  if (timer) { clearInterval(timer); timer = null; btn.textContent = "▶"; return; }
  btn.textContent = "❚❚";
  let i = 1;
  timer = setInterval(() => {
    scrub.value = i; drawTrain(i); i += 1;
    if (i > stages.length) { clearInterval(timer); timer = null; btn.textContent = "▶"; }
  }, 40);
});
"""


def _transformer_script() -> str:
    return r"""
mountCounters();

const ratios = ["0.5", "0.75", "0.9", "1.0"];
const labels = ["50% of match", "75%", "90%", "Full game"];

function renderPrefix() {
  const i = Number(document.getElementById("prefixScrub").value);
  const key = ratios[i];
  const row = DATA.byPrefix[key] || {};
  document.getElementById("prefixReadout").textContent = labels[i];
  // timeline fill
  const fills = ratios.map((r, idx) => {
    const active = idx <= i;
    const w = idx === 0 ? 50 : (idx === 1 ? 25 : (idx === 2 ? 15 : 10));
    return `<span style="flex:${w};background:${active ? "linear-gradient(90deg,#6366f1,#22d3ee)" : "transparent"}"></span>`;
  });
  document.getElementById("timeline").innerHTML = fills.join("");
  document.getElementById("prefixStats").innerHTML = [
    ["Prefix", labels[i], `${row.n || 0} battles`],
    ["Accuracy", pct(row.acc || 0), ""],
    ["AUC", (row.auc || 0).toFixed(3), ""],
    ["vs majority", ((row.acc || 0) - DATA.majority >= 0 ? "+" : "") +
      (((row.acc || 0) - DATA.majority) * 100).toFixed(1) + "pp", ""],
  ].map(([k,v,s]) =>
    `<div class="stat"><div class="k">${k}</div><div class="v">${v}</div><div class="s">${s}</div></div>`
  ).join("");
}
document.getElementById("prefixScrub").addEventListener("input", renderPrefix);
renderPrefix();

mountRace("race", ratios.map((r, i) => ({
  label: labels[i],
  note: `AUC ${((DATA.byPrefix[r] || {}).auc || 0).toFixed(3)}`,
  old: DATA.majority,
  new: (DATA.byPrefix[r] || {}).acc || 0,
  fmt: "pct",
  higher: true,
})), {
  buttonId: "raceBtn", hintId: "raceHint", oldName: "maj",
  playLabel: "▶ Run the diff", backLabel: "↺ Back to majority",
  hintBefore: "bars parked at majority baseline",
  hintAfter: "bars moved to the transformer",
});

const hist = DATA.history || {};
const n = (hist.val_auc || []).length;
const scrub = document.getElementById("trainScrub");
scrub.max = Math.max(n, 1);
scrub.value = n;
function drawTrain(upto) {
  lineChart(document.getElementById("trainChart"), [
    { label: "val AUC", color: "#34d399", values: (hist.val_auc || []).slice(0, upto) },
    { label: "val full AUC", color: "#22d3ee", values: (hist.val_full_auc || []).slice(0, upto) },
    { label: "val acc", color: "#a78bfa", values: (hist.val_acc || []).slice(0, upto) },
  ], { yFormat: "percent", xLabel: "epoch" });
  document.getElementById("trainReadout").textContent = `epoch ${upto}`;
}
document.getElementById("trainLegend").innerHTML = legendHTML([
  { label: "val AUC", color: "#34d399" },
  { label: "val full AUC", color: "#22d3ee" },
  { label: "val acc", color: "#a78bfa" },
]);
drawTrain(n);
scrub.addEventListener("input", () => drawTrain(Number(scrub.value)));
let timer = null;
document.getElementById("trainPlay").addEventListener("click", () => {
  const btn = document.getElementById("trainPlay");
  if (timer) { clearInterval(timer); timer = null; btn.textContent = "▶"; return; }
  btn.textContent = "❚❚";
  let i = 1;
  timer = setInterval(() => {
    scrub.value = i; drawTrain(i); i += 1;
    if (i > n) { clearInterval(timer); timer = null; btn.textContent = "▶"; }
  }, 120);
});
"""
