"""Interactive report for the defense data-support / natural-hand audit.

Story: before blaming the model for missing GY→poison, ask whether the data
even contains enough natural cases where the answer was in hand. The visual is
a per-cell funnel: threat windows → answer in deck → answer in hand → human
took it → model top-1.
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


def render_defense_support_report(
    audit_path: str | Path = "reports/defense_support_audit.json",
    output_path: str | Path = "reports/defense_support_audit_v1.html",
) -> Path:
    report = load_json(audit_path)
    if not report:
        raise FileNotFoundError(audit_path)

    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)

    summary = report.get("summary", {})
    setup = report.get("setup", {})
    cells = report.get("cells", [])
    status_counts = summary.get("status_counts", {})
    thresholds = setup.get("thresholds", {})

    payload = {
        "cells": [
            {
                "threat": c["threat"],
                "answer": c["answer"],
                "role": c.get("role", ""),
                "status": (c.get("decision") or {}).get("status", ""),
                "action": (c.get("decision") or {}).get("action", ""),
                "n_deck": c.get("n_threat_answer_in_deck", 0),
                "n_hand": c.get("n_threat_answer_in_hand", 0),
                "n_windows": (c.get("train") or {}).get("n_threat_windows", 0),
                "human_deck": c.get("human_use_rate_given_in_deck", 0),
                "human_hand": c.get("human_use_rate_given_in_hand", 0),
                "model_in_hand": c.get("model_top1_answer_given_in_hand"),
                "model_when_human": c.get("model_top1_when_human_chose_answer"),
                "model_top3": c.get("model_top3_answer_given_in_hand"),
                "n_human_chose": c.get("n_human_chose_answer_in_hand", 0),
                "n_scored_hand": c.get("n_scored_in_hand_test", 0),
                "beats_cheap": c.get("model_beats_cheap_when_human_chose"),
            }
            for c in cells
        ],
        "thresholds": thresholds,
        "statusCounts": status_counts,
    }

    ok = status_counts.get("supported_and_ok", 0)
    fail = status_counts.get("supported_but_model_fails", 0)
    unsup = status_counts.get("unsupported", 0)

    body = f"""
    <header class="report-header hero">
      {badge_row(
          esc(report.get("model_name", "policy-bc")),
          "defense support audit",
          f"{summary.get('cells', 0)} cells",
          esc(report.get("created_at", "")),
      )}
      <h1>Does the data even support this answer?</h1>
      <p class="hero-sub">Before we redesign the model for a "failing" defense cell, check
      whether the corpus contains enough natural cases where the textbook answer was actually
      in hand — and whether the model picks it when it is. Unsupported cells are not model
      failures; they are data gaps.</p>
      <p class="meta">Thresholds: support n ≥ {thresholds.get("min_support_n", "?")} ·
      human use-when-in-hand ≥ {_pct(thresholds.get("min_human_use_rate_in_hand"))} ·
      model top-1 gate {_pct(thresholds.get("min_model_top1_when_supported"))}</p>
      {hero_scores([
          ("Supported & ok", ok, "up"),
          ("Supported but fails", fail, "down" if fail else "neutral"),
          ("Unsupported", unsup, "flat"),
          ("Next step", summary.get("next_step", "—"), "neutral"),
      ])}
      {verdict_banner(
          "OK" if fail == 0 else "ACT",
          report.get("verdict", "") if isinstance(report.get("verdict"), str)
          else str(report.get("verdict", "")),
      )}
    </header>

    <section class="report-section">
      <h2>How to read a cell</h2>
      <div class="flow">
        <div class="flow-step"><span class="flow-n">1</span><b>Threat windows</b>
          <small>opponent just played the win condition</small></div>
        <div class="flow-arrow"></div>
        <div class="flow-step"><span class="flow-n">2</span><b>Answer in deck</b>
          <small>the textbook card is somewhere in the 8</small></div>
        <div class="flow-arrow"></div>
        <div class="flow-step"><span class="flow-n">3</span><b>Answer in hand</b>
          <small>oldest-four heuristic says it was available</small></div>
        <div class="flow-arrow"></div>
        <div class="flow-step"><span class="flow-n">4</span><b>Human took it</b>
          <small>how often humans themselves play the answer</small></div>
        <div class="flow-arrow"></div>
        <div class="flow-step"><span class="flow-n">5</span><b>Model top-1</b>
          <small>does the policy pick it when it's in hand?</small></div>
      </div>
    </section>

    <section class="report-section">
      <h2>Pick a cell</h2>
      <p class="caption">Click a threat→answer pair. The funnel updates to that cell's train
      support; the gauges show human rate and model top-1 on the natural-hand test windows.</p>
      <div class="chip-row" id="cellChips"></div>
      <div class="cell-stage">
        <div class="funnel" id="funnel"></div>
        <div class="cell-side">
          <div class="stat-grid" id="cellStats"></div>
          <div class="status-card" id="statusCard"></div>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>All cells at a glance</h2>
      <p class="caption">Dot = model top-1 when the answer is in hand. Grey tick = human use
      rate (the real ceiling — humans are not textbooks). Click a row to select it above.</p>
      <div class="legend-row">
        <span><i class="swatch" style="background:#34d399"></i>supported &amp; ok</span>
        <span><i class="swatch" style="background:#fbbf24"></i>supported but model fails</span>
        <span><i class="swatch" style="background:#f87171"></i>unsupported</span>
        <span><i class="swatch tick" style="background:#cbd5f5"></i>human use rate</span>
      </div>
      <div id="dumbbells"></div>
    </section>

    <section class="report-section">
      <h2>Lessons learned</h2>
      <ul class="lessons">{lesson_list(report.get("lessons", []))}</ul>
    </section>

    <footer class="report-footer">
      <p class="meta">Offline audit · source {esc(str(audit_path))}</p>
    </footer>
    """

    script = f"""
const DATA = {_json_script(payload)};
{_support_script()}
"""
    html_doc = page(
        title="Defense support audit — is the answer even in the data?",
        body=body,
        script=script,
        extra_styles=_support_styles(),
    )
    out.write_text(html_doc, encoding="utf-8")
    return out


def _pct(v: Any) -> str:
    if v is None:
        return "—"
    try:
        return f"{100 * float(v):.0f}%"
    except (TypeError, ValueError):
        return "—"


def _support_styles() -> str:
    return """
    .flow { display: flex; align-items: stretch; gap: 8px; flex-wrap: wrap; }
    .flow-step {
      flex: 1 1 120px; padding: 12px; border-radius: 12px; position: relative;
      background: rgba(148,163,184,0.07); border: 1px solid rgba(148,163,184,0.18);
    }
    .flow-step b { display: block; font-size: 0.9rem; margin-bottom: 3px; }
    .flow-step small { color: #94a3b8; font-size: 0.72rem; display: block; line-height: 1.35; }
    .flow-n {
      position: absolute; top: -9px; left: 10px; width: 20px; height: 20px; border-radius: 50%;
      background: #22d3ee; color: #06111f; font-size: 0.68rem; font-weight: 800;
      display: grid; place-items: center;
    }
    .flow-arrow { align-self: center; width: 14px; height: 2px; background: rgba(148,163,184,0.4); }
    .cell-stage { display: grid; grid-template-columns: 1.2fr 1fr; gap: 24px; margin-top: 18px; }
    .funnel { display: grid; gap: 8px; }
    .funnel-step {
      display: grid; grid-template-columns: 140px 1fr 70px; gap: 12px; align-items: center;
      font-size: 0.85rem;
    }
    .funnel-track { height: 28px; border-radius: 8px; background: rgba(148,163,184,0.1); overflow: hidden; }
    .funnel-fill {
      height: 100%; border-radius: 8px;
      background: linear-gradient(90deg, #6366f1, #22d3ee);
      transition: width .7s cubic-bezier(.22,1,.36,1);
    }
    .status-card {
      margin-top: 16px; padding: 16px; border-radius: 14px; border: 1px solid;
      font-size: 0.9rem;
    }
    .status-card.supported_and_ok { background: rgba(52,211,153,0.1); border-color: rgba(52,211,153,0.4); }
    .status-card.supported_but_model_fails { background: rgba(251,191,36,0.1); border-color: rgba(251,191,36,0.4); }
    .status-card.unsupported { background: rgba(248,113,113,0.1); border-color: rgba(248,113,113,0.4); }
    .dumb-row {
      display: grid; grid-template-columns: 200px 1fr 100px; gap: 12px; align-items: center;
      font-size: 0.85rem; margin-bottom: 8px; cursor: pointer; padding: 4px 0;
    }
    .dumb-row:hover { background: rgba(148,163,184,0.06); }
    .dumb-track { position: relative; height: 24px; }
    .dumb-axis { position: absolute; inset: 11px 0 auto 0; height: 1px; background: rgba(148,163,184,0.2); }
    .dumb-dot { position: absolute; top: 6px; width: 12px; height: 12px; border-radius: 50%; transform: translateX(-50%); }
    .dumb-human { position: absolute; top: 2px; width: 2px; height: 20px; background: #cbd5f5aa; }
    @media (max-width: 900px) { .cell-stage { grid-template-columns: 1fr; } }
    """


def _support_script() -> str:
    return r"""
mountCounters();

const STATUS_COLOR = {
  supported_and_ok: "#34d399",
  supported_but_model_fails: "#fbbf24",
  unsupported: "#f87171",
};
let idx = 0;

function renderChips() {
  const host = document.getElementById("cellChips");
  host.innerHTML = DATA.cells.map((c, i) =>
    `<button type="button" class="chip ${i === idx ? "active" : ""}" data-i="${i}"
       style="${i === idx ? "" : `border-color:${STATUS_COLOR[c.status] || "#64748b"}55`}">
       ${titleCase(c.threat)} → ${titleCase(c.answer)}
       <span class="chip-n">${c.role}</span></button>`
  ).join("");
  host.querySelectorAll(".chip").forEach((chip) => {
    chip.addEventListener("click", () => { idx = Number(chip.dataset.i); renderAll(); });
  });
}

function renderFunnel() {
  const c = DATA.cells[idx];
  const steps = [
    { label: "Threat windows", n: c.n_windows || Math.max(c.n_deck * 3, c.n_deck), note: "train" },
    { label: "Answer in deck", n: c.n_deck, note: "textbook card owned" },
    { label: "Answer in hand", n: c.n_hand, note: "oldest-four says available" },
    { label: "Human took it", n: Math.round(c.n_hand * (c.human_hand || 0)), note: pct(c.human_hand) + " of in-hand" },
  ];
  const peak = Math.max(...steps.map((s) => s.n), 1);
  document.getElementById("funnel").innerHTML = steps.map((s) =>
    `<div class="funnel-step">
       <span>${s.label}<br><small style="color:#94a3b8">${s.note}</small></span>
       <span class="funnel-track"><span class="funnel-fill" style="width:${(s.n / peak) * 100}%"></span></span>
       <span style="text-align:right;font-variant-numeric:tabular-nums">${s.n.toLocaleString()}</span>
     </div>`
  ).join("");

  document.getElementById("cellStats").innerHTML = [
    ["Model top-1 | in hand", c.model_in_hand == null ? "—" : pct(c.model_in_hand), `n=${c.n_scored_hand} test`],
    ["Model top-1 | human chose", c.model_when_human == null ? "—" : pct(c.model_when_human), `n=${c.n_human_chose}`],
    ["Model top-3 | in hand", c.model_top3 == null ? "—" : pct(c.model_top3), "softer gate"],
    ["Beats cheap distractor", c.beats_cheap == null ? "—" : pct(c.beats_cheap), "when human chose answer"],
    ["Human rate | in hand", pct(c.human_hand), "real ceiling"],
    ["Human rate | in deck", pct(c.human_deck), "including not-in-hand"],
  ].map(([k, v, s]) =>
    `<div class="stat"><div class="k">${k}</div><div class="v">${v}</div><div class="s">${s}</div></div>`
  ).join("");

  const card = document.getElementById("statusCard");
  card.className = "status-card " + (c.status || "");
  card.innerHTML = `<b style="color:${STATUS_COLOR[c.status] || "#e2e8f0"}">${titleCase(c.status || "unknown")}</b>
    <p style="margin:8px 0 0">${c.action || ""}</p>`;
}

function renderDumbbells() {
  const max = Math.max(
    ...DATA.cells.flatMap((c) => [c.model_in_hand || 0, c.human_hand || 0]),
    0.2,
  ) * 1.15;
  document.getElementById("dumbbells").innerHTML = DATA.cells.map((c, i) => {
    const m = c.model_in_hand;
    const left = m == null ? 0 : (m / max) * 100;
    return `<div class="dumb-row" data-i="${i}">
      <span>${titleCase(c.threat)} → ${titleCase(c.answer)}
        <br><small style="color:#94a3b8">${c.role} · hand n=${c.n_hand}</small></span>
      <span class="dumb-track">
        <span class="dumb-axis"></span>
        ${c.human_hand ? `<span class="dumb-human" style="left:${(c.human_hand / max) * 100}%"></span>` : ""}
        ${m == null ? "" : `<span class="dumb-dot" style="left:${left}%;background:${STATUS_COLOR[c.status] || "#94a3b8"}"></span>`}
      </span>
      <span style="text-align:right">${m == null ? "—" : pct(m)}</span>
    </div>`;
  }).join("");
  document.querySelectorAll(".dumb-row").forEach((row) => {
    row.addEventListener("click", () => { idx = Number(row.dataset.i); renderAll(); });
  });
}

function renderAll() {
  renderChips();
  renderFunnel();
  renderDumbbells();
}
renderAll();
"""
