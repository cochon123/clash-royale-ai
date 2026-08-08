"""Interactive report for experiment C — the action clock.

Story: rollouts used to hardcode side alternation. A learned "who acts next /
when" clock beats that on initiative (+6pp) but misses the delay MAE gate.
The visual language is a timeline conveyor and phase-split delay histograms,
not another generic metric table.
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
from .winner_report import _json_script


def render_action_clock_report(
    report: dict[str, Any] | None = None,
    output_path: str | Path = "reports/action_clock_v1.html",
    json_path: str | Path | None = "reports/action_clock_v1.json",
) -> Path:
    if report is None:
        report = load_json(json_path or "reports/action_clock_v1.json")
        if not report:
            raise FileNotFoundError(json_path)

    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)

    compute = report.get("compute", {})
    data = report.get("data", {})
    metrics = report.get("metrics", {})
    test = metrics.get("test", {})
    baselines = report.get("baselines", {})
    actor_base = baselines.get("actor", {})
    rollouts = report.get("rollouts", {})
    charts = report.get("charts", {})
    verdict = report.get("verdict", {})
    success = report.get("success_criteria", {})

    lift = test.get("actor_lift_pp", 0) / 100.0
    actor_acc = test.get("actor_acc", 0)
    alt_acc = actor_base.get("alternation_acc", test.get("best_trivial_actor_acc", 0))
    delay_mae = test.get("delay_mae", 0)
    delay_base = test.get("delay_phase_median_mae", baselines.get("delay_phase_median_mae", 0))

    alt_roll = rollouts.get("alternation", {})
    clock_roll = rollouts.get("clock", {})
    real_roll = rollouts.get("real_slice", {})

    payload = {
        "actorByPhase": metrics.get("actor_by_phase", {}),
        "delayByPhase": metrics.get("delay_by_phase", {}),
        "trueHist": charts.get("delay_true_hist", {}),
        "residHist": charts.get("delay_residual_hist", {}),
        "actor": {
            "learned": actor_acc,
            "alternation": alt_acc,
            "majority": actor_base.get("majority_acc", alt_acc),
            "sameRate": data.get("same_rate_test", 0),
            "liftPp": test.get("actor_lift_pp", 0),
        },
        "delay": {
            "learned": delay_mae,
            "phaseMedian": delay_base,
            "gate": 1.25,
        },
        "rollouts": {
            "alternation": {k: v for k, v in alt_roll.items() if k != "scores"},
            "clock": {k: v for k, v in clock_roll.items() if k != "scores"},
            "real": {k: v for k, v in real_roll.items() if k != "scores"},
            "hist": report.get("rollout_hist", {}),
        },
        "gates": success,
    }

    status = verdict.get("status", "FAIL")
    body = f"""
    <header class="report-header hero">
      {badge_row(
          esc(report.get("model_name", "action-clock-v1")),
          "experiment C",
          esc(compute.get("framework", "sklearn")),
          f"{data.get('test_samples', 0):,} test ticks",
          esc(compute.get("device", "cpu")),
      )}
      <h1>Who acts next — and when</h1>
      <p class="hero-sub">{esc(report.get("hypothesis", ""))}</p>
      <p class="meta">Created {esc(report.get("created_at", ""))} ·
      trained in {report.get("seconds", 0)}s · {data.get("battles_total", 0):,} battles ·
      feature dim {compute.get("feature_dim", "?")}</p>
      {hero_scores([
          ("Actor lift vs alternation", round(test.get("actor_lift_pp", 0), 1), "up"),
          ("Actor accuracy", round(100 * actor_acc, 1), "up"),
          ("Delay MAE (s)", round(delay_mae, 2), "down" if delay_mae > 1.25 else "up"),
          ("Collapse cut (relative)", round(100 * (verdict.get("collapse_relative_reduction") or 0)), "up"),
      ])}
      {verdict_banner(status, verdict.get("summary", ""))}
    </header>

    <section class="report-section">
      <h2>The problem this clock is for</h2>
      <p class="caption">Policy rollouts used to flip sides every turn: team, opponent, team,
      opponent… Real games do not. Same-side double-taps happen about
      {100 * data.get("same_rate_test", 0):.0f}% of the time. The clock's job is to predict
      whether the next action stays on the same side, and how many seconds until it lands.</p>
      <div class="conveyor" id="conveyor"></div>
      <div class="legend-row" style="margin-top:12px">
        <span><i class="swatch" style="background:#60a5fa"></i>team</span>
        <span><i class="swatch" style="background:#f87171"></i>opponent</span>
        <span><i class="swatch bar" style="background:#fbbf24"></i>same-side double-tap (the thing alternation gets wrong)</span>
      </div>
    </section>

    <section class="report-section">
      <h2>Initiative: learned vs the dumb rules</h2>
      <p class="caption">Hit <em>Run the diff</em>. Alternation and majority are the same baseline
      here (both always predict "switch"), so the only real comparison is learned vs that.</p>
      <div class="toolbar">
        <button type="button" class="play-btn" id="actorRaceBtn">▶ Run the diff</button>
        <span class="hint" id="actorRaceHint">bars parked at the alternation baseline</span>
      </div>
      <div class="race" id="actorRace"></div>
    </section>

    <section class="report-section">
      <h2>Phase dial</h2>
      <p class="caption">Elixir phase changes the rhythm. Single elixir is slow and long;
      triple is frantic. Pick a phase to see how the clock behaves there.</p>
      <div class="chip-row" id="phaseChips">
        <button type="button" class="chip active" data-phase="single">Single elixir</button>
        <button type="button" class="chip" data-phase="double">Double elixir</button>
        <button type="button" class="chip" data-phase="triple">Triple elixir</button>
      </div>
      <div class="stat-grid" id="phaseStats" style="margin-top:16px"></div>
      <div class="phase-bars" id="phaseBars" style="margin-top:18px"></div>
    </section>

    <section class="report-section block-grid">
      <div class="block">
        <h2>True delay distribution</h2>
        <p class="caption">How long humans actually wait between actions (capped at 12s).</p>
        <svg class="chart" id="trueDelayHist"></svg>
      </div>
      <div class="block">
        <h2>Delay residuals</h2>
        <p class="caption">Predicted − true. A good clock would pile up around zero; this one
        still has a fat right shoulder — it under-predicts long pauses.</p>
        <svg class="chart" id="residHist"></svg>
        <div class="gate-line-note">Delay gate is ≤ 1.25s MAE · clock got {_fmt_sec(delay_mae)} ·
        phase-median baseline {_fmt_sec(delay_base)}</div>
      </div>
    </section>

    <section class="report-section">
      <h2>Does the clock help rollouts?</h2>
      <p class="caption">Same policy (v3), same seeds — only the "who acts next" rule changes.
      Collapse rate is the fraction of rollouts the realism scorer rates below 0.25.</p>
      <div id="rolloutRace" class="race"></div>
      <div class="toolbar" style="margin-top:8px">
        <button type="button" class="play-btn" id="rollRaceBtn">▶ Run the diff</button>
        <span class="hint" id="rollRaceHint">bars parked at alternation rollouts</span>
      </div>
      <div class="score-clouds" id="scoreClouds"></div>
    </section>

    <section class="report-section">
      <h2>Gates</h2>
      <div class="gate-grid" id="gates"></div>
    </section>

    <section class="report-section">
      <h2>Lessons learned</h2>
      <ul class="lessons">{lesson_list(report.get("lessons", []))}</ul>
      <p class="caption">{esc(report.get("recommendation", ""))}</p>
      <p class="caption">{esc(verdict.get("note", ""))}</p>
    </section>

    <footer class="report-footer">
      <p class="meta">Offline experiment · checkpoint {esc(report.get("checkpoint", ""))}</p>
    </footer>
    """

    script = f"""
const DATA = {_json_script(payload)};
{_clock_script()}
"""
    html_doc = page(
        title="Action clock — who acts next, and when",
        body=body,
        script=script,
        extra_styles=_clock_styles(),
    )
    out.write_text(html_doc, encoding="utf-8")
    return out


def _fmt_sec(v: float) -> str:
    return f"{v:.2f}s"


def _clock_styles() -> str:
    return """
    .conveyor {
      display: flex; gap: 6px; align-items: flex-end; overflow-x: auto;
      padding: 18px 8px 8px; min-height: 110px;
      background: linear-gradient(180deg, rgba(15,23,42,0.2), rgba(148,163,184,0.05));
      border-radius: 14px; border: 1px solid rgba(148,163,184,0.16);
    }
    .tick {
      flex: 0 0 auto; width: 28px; border-radius: 6px 6px 3px 3px; position: relative;
      transition: transform .2s ease, opacity .2s ease;
    }
    .tick.team { background: linear-gradient(180deg, #60a5fa, #1d4ed8); }
    .tick.opp { background: linear-gradient(180deg, #f87171, #b91c1c); }
    .tick.double::after {
      content: ""; position: absolute; top: -8px; left: 50%; width: 8px; height: 8px;
      border-radius: 50%; background: #fbbf24; transform: translateX(-50%);
      box-shadow: 0 0 0 2px #0b1220;
    }
    .tick:hover { transform: translateY(-4px); }
    .gate-line-note { font-size: 0.78rem; color: #94a3b8; margin-top: 8px; }
    .phase-bars .bar-row {
      display: grid; grid-template-columns: 140px 1fr 80px; gap: 10px; align-items: center;
      font-size: 0.85rem; margin-bottom: 8px;
    }
    .phase-bars .bar-track { height: 16px; border-radius: 5px; background: rgba(148,163,184,0.1); overflow: hidden; }
    .phase-bars .bar-fill { height: 100%; border-radius: 5px; }
    .gate-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 12px; }
    .gate-card {
      padding: 14px; border-radius: 12px; border: 1px solid rgba(148,163,184,0.18);
      background: rgba(148,163,184,0.06); font-size: 0.85rem;
    }
    .gate-card.pass { border-color: rgba(52,211,153,0.4); }
    .gate-card.fail { border-color: rgba(248,113,113,0.4); }
    .gate-card .g-status { font-weight: 800; letter-spacing: 0.06em; font-size: 0.72rem; }
    .score-clouds { display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 14px; margin-top: 18px; }
    .cloud {
      padding: 12px; border-radius: 12px; background: rgba(148,163,184,0.06);
      border: 1px solid rgba(148,163,184,0.16); font-size: 0.82rem;
    }
    .cloud h4 { margin: 0 0 8px; font-size: 0.85rem; }
    .cloud-bar { display: flex; height: 48px; border-radius: 6px; overflow: hidden; margin-bottom: 8px; }
    .cloud-bar span { display: block; height: 100%; }
    @media (max-width: 900px) { .score-clouds { grid-template-columns: 1fr; } }
    """


def _clock_script() -> str:
    return r"""
mountCounters();

/* ---------- conveyor of a synthetic but realistic tick sequence ---------- */
(function buildConveyor() {
  const host = document.getElementById("conveyor");
  const ticks = [];
  let side = 0; // 0 team, 1 opp
  // Start with strict alternation, then inject same-side doubles at ~33% rate
  for (let i = 0; i < 36; i++) {
    const same = Math.random() < DATA.actor.sameRate && i > 0;
    if (!same) side = 1 - side;
    const delay = 0.4 + Math.random() * 3.2;
    ticks.push({ side, delay, same });
  }
  host.innerHTML = "";
  ticks.forEach((t) => {
    const d = document.createElement("div");
    d.className = "tick " + (t.side === 0 ? "team" : "opp") + (t.same ? " double" : "");
    d.style.height = (18 + t.delay * 14) + "px";
    d.title = (t.side === 0 ? "team" : "opponent") + " · Δt " + t.delay.toFixed(1) + "s"
      + (t.same ? " · same-side double-tap" : "");
    host.appendChild(d);
  });
})();

/* ---------- actor race ---------- */
mountRace("actorRace", [
  {
    label: "Who-acts-next accuracy",
    note: "Learned clock vs hardcoded alternation on the held-out test ticks.",
    old: DATA.actor.alternation, new: DATA.actor.learned, fmt: "pct", higher: true,
  },
  {
    label: "Same-side rate (truth)",
    note: "Share of real ticks that stay on the same side — what alternation always gets wrong.",
    old: 0, new: DATA.actor.sameRate, fmt: "pct", higher: true, soft: true,
  },
], {
  buttonId: "actorRaceBtn", hintId: "actorRaceHint", oldName: "alt",
  playLabel: "▶ Run the diff", backLabel: "↺ Back to alternation",
  hintBefore: "bars parked at the alternation baseline",
  hintAfter: "bars moved to the learned clock",
});

/* ---------- phase dial ---------- */
let phase = "single";
function renderPhase() {
  const a = DATA.actorByPhase[phase] || {};
  const d = DATA.delayByPhase[phase] || {};
  document.getElementById("phaseStats").innerHTML = [
    ["Ticks", (a.n || 0).toLocaleString(), "held-out"],
    ["Same-side rate", pct(a.same_rate || 0), "truth"],
    ["Actor accuracy", pct(a.actor_acc || 0), "learned"],
    ["Delay MAE", (d.learned_mae || 0).toFixed(2) + "s", "learned"],
    ["Phase-median MAE", (d.phase_median_mae || 0).toFixed(2) + "s", "baseline"],
    ["Median true Δt", (d.median_true || 0).toFixed(2) + "s", "humans"],
  ].map(([k, v, s]) =>
    `<div class="stat"><div class="k">${k}</div><div class="v">${v}</div><div class="s">${s}</div></div>`
  ).join("");

  const rows = [
    { label: "Actor accuracy", value: a.actor_acc || 0, color: "#22d3ee", fmt: "pct" },
    { label: "Alternation (always switch)", value: 1 - (a.same_rate || 0), color: "#f87171", fmt: "pct" },
    { label: "Delay MAE (lower better)", value: d.learned_mae || 0, color: "#34d399", fmt: "sec", invert: true, max: 3.5 },
    { label: "Phase-median MAE", value: d.phase_median_mae || 0, color: "#fbbf24", fmt: "sec", invert: true, max: 3.5 },
  ];
  document.getElementById("phaseBars").innerHTML = rows.map((r) => {
    const denom = r.max || 1;
    const w = r.invert ? Math.max(4, (1 - r.value / denom) * 100) : r.value * 100;
    const shown = r.fmt === "pct" ? pct(r.value) : r.value.toFixed(2) + "s";
    return `<div class="bar-row"><span>${r.label}</span>
      <span class="bar-track"><span class="bar-fill" style="width:${w}%;background:${r.color}"></span></span>
      <span style="text-align:right;font-variant-numeric:tabular-nums">${shown}</span></div>`;
  }).join("");
}
document.querySelectorAll("#phaseChips .chip").forEach((chip) => {
  chip.addEventListener("click", () => {
    document.querySelectorAll("#phaseChips .chip").forEach((c) => c.classList.remove("active"));
    chip.classList.add("active");
    phase = chip.dataset.phase;
    renderPhase();
  });
});
renderPhase();

/* ---------- histograms ---------- */
function drawHist(svgId, hist, color) {
  const svg = document.getElementById(svgId);
  if (!hist || !hist.counts) return;
  histChart(svg, [{ counts: hist.counts, color }], {
    ticks: [
      { at: 0, label: String(hist.lo ?? 0) },
      { at: 0.5, label: String(((hist.lo ?? 0) + (hist.hi ?? 0)) / 2) },
      { at: 1, label: String(hist.hi ?? "") },
    ],
  });
}
drawHist("trueDelayHist", DATA.trueHist, "#60a5fa");
drawHist("residHist", DATA.residHist, "#a78bfa");

/* ---------- rollout comparison ---------- */
const alt = DATA.rollouts.alternation || {};
const clk = DATA.rollouts.clock || {};
const real = DATA.rollouts.real || {};
mountRace("rolloutRace", [
  {
    label: "Mean P(real)",
    note: "Realism scorer on the finished continuations.",
    old: alt.mean_score || 0, new: clk.mean_score || 0, fmt: "float", higher: true,
  },
  {
    label: "Collapse rate",
    note: "Share of rollouts with P(real) < 0.25 (lower is better).",
    old: alt.collapse_rate || 0, new: clk.collapse_rate || 0, fmt: "pct", higher: false,
  },
], {
  buttonId: "rollRaceBtn", hintId: "rollRaceHint", oldName: "alt",
  playLabel: "▶ Run the diff", backLabel: "↺ Back to alternation",
  hintBefore: "bars parked at alternation rollouts",
  hintAfter: "bars moved to the clock-driven rollouts",
});

function cloudHTML(title, blk, color) {
  const collapse = blk.collapse_rate || 0;
  const healthy = Math.max(0, 1 - collapse - 0.05); // rough mid band
  const mid = Math.max(0, 1 - collapse - healthy);
  return `<div class="cloud"><h4>${title}</h4>
    <div class="cloud-bar">
      <span style="width:${collapse * 100}%;background:#f87171"></span>
      <span style="width:${mid * 100}%;background:#fbbf24"></span>
      <span style="width:${healthy * 100}%;background:#34d399"></span>
    </div>
    <div>mean P(real) <b style="color:${color}">${(blk.mean_score || 0).toFixed(3)}</b>
      · collapse ${pct(collapse)} · n=${blk.n || 0}</div></div>`;
}
document.getElementById("scoreClouds").innerHTML =
  cloudHTML("Alternation rollouts", alt, "#f87171") +
  cloudHTML("Clock rollouts", clk, "#34d399") +
  cloudHTML("Real battles (same length)", real, "#60a5fa");

/* ---------- gates ---------- */
const gateMeta = [
  ["actor_acc_ge_5pp_over_best_baseline", "Actor ≥ +5pp over alternation"],
  ["delay_mae_le_1_25", "Delay MAE ≤ 1.25s"],
  ["collapse_relative_reduction_ge_50pct", "Collapse rate cut ≥ 50%"],
  ["fail_if_within_2pp_of_alternation", "Fail if within 2pp of alternation (inverted)"],
];
document.getElementById("gates").innerHTML = gateMeta.map(([key, label]) => {
  const raw = DATA.gates[key];
  // the inverted fail-gate is "true" when the fail condition triggers
  const isFailGate = key.startsWith("fail_if");
  const passed = isFailGate ? !raw : !!raw;
  return `<div class="gate-card ${passed ? "pass" : "fail"}">
    <div class="g-status ${passed ? "up" : "down"}">${passed ? "PASS" : "FAIL"}</div>
    <div>${label}</div></div>`;
}).join("");
"""
