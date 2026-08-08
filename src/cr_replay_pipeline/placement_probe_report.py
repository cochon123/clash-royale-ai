"""Interactive report for experiment B — card-conditioned placement probe.

Story: placement is bad because the XY/zone heads never see which card is being
played. An oracle that gets the true card jumps to 52.5% zone; end-to-end with
the frozen v3 slot head barely moves. The visual language is an arena ladder
(table → v3 → e2e → oracle) and a per-card lift chart — not a loss curve.
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


def render_placement_probe_report(
    report: dict[str, Any] | None = None,
    output_path: str | Path = "reports/placement_probe_v1.html",
    json_path: str | Path | None = "reports/placement_probe_v1.json",
) -> Path:
    if report is None:
        report = load_json(json_path or "reports/placement_probe_v1.json")
        if not report:
            raise FileNotFoundError(json_path)

    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)

    decision = report.get("decision", {})
    compute = report.get("compute", {})
    data = report.get("data", {})
    control = report.get("control", {})
    oracle = report.get("oracle", {})
    e2e = report.get("e2e", {})
    frozen = report.get("frozen_v3", {})
    cards = report.get("per_card_highlights", [])
    history = report.get("history", [])
    gates = decision.get("gates", {})
    deltas = decision.get("deltas_pp", {})

    payload = {
        "ladder": [
            {
                "key": "table",
                "label": "Per-card table",
                "note": "Majority zone + mean XY for that card. No game state.",
                "zone": control.get("zone_acc", 0),
                "tile": control.get("tile_acc", 0),
                "xy": control.get("xy_mae", 0),
            },
            {
                "key": "v3",
                "label": "Frozen policy v3",
                "note": "Card-blind placement heads — the status quo.",
                "zone": frozen.get("zone_acc", 0),
                "tile": frozen.get("tile_acc", 0),
                "xy": frozen.get("xy_mae", 0),
            },
            {
                "key": "e2e",
                "label": "E2E (v3 slot → probe)",
                "note": "Probe sees the slot head's soft card pick, trunk frozen.",
                "zone": e2e.get("zone_acc", 0),
                "tile": e2e.get("tile_acc", 0),
                "xy": e2e.get("xy_mae", 0),
            },
            {
                "key": "oracle",
                "label": "Oracle card",
                "note": "Probe sees the true card. Upper bound for this architecture.",
                "zone": oracle.get("zone_acc", 0),
                "tile": oracle.get("tile_acc", 0),
                "xy": oracle.get("xy_mae", 0),
            },
        ],
        "cards": cards,
        "history": history,
        "gates": gates,
        "deltas": deltas,
    }

    body = f"""
    <header class="report-header hero">
      {badge_row(
          esc(report.get("model_name", "placement-probe-v1")),
          "experiment B",
          "frozen trunk " + esc(compute.get("trunk_model", "policy-bc-v3")),
          f"{data.get('test_samples', 0):,} test plays",
          esc(compute.get("device", "cpu")),
      )}
      <h1>Does the placement head need to know the card?</h1>
      <p class="hero-sub">{esc(report.get("hypothesis", ""))}</p>
      <p class="meta">Created {esc(report.get("created_at", ""))} ·
      {report.get("seconds", 0)}s · probe params {compute.get("probe_parameters", 0):,} ·
      trunk frozen · {compute.get("epochs", 0)} epochs</p>
      {hero_scores([
          ("Oracle zone", round(100 * oracle.get("zone_acc", 0), 1), "up"),
          ("Oracle vs table", round(deltas.get("oracle_vs_table", 0), 1), "up"),
          ("E2E vs v3", round(deltas.get("e2e_vs_v3", 0), 1),
           "up" if deltas.get("e2e_vs_v3", 0) >= 3 else "down"),
          ("Oracle tile", round(100 * oracle.get("tile_acc", 0), 1), "up"),
      ])}
      {verdict_banner(decision.get("verdict", "PARTIAL"), decision.get("recommendation", ""))}
    </header>

    <section class="report-section">
      <h2>The blind spot</h2>
      <p class="caption">v3 decides the card and the place as two siblings hanging off the same
      GRU state. The placement head never gets told which card won. This probe freezes that trunk
      and asks: if we <em>did</em> tell it, how much would placement improve?</p>
      <div class="wire" id="wire"></div>
    </section>

    <section class="report-section">
      <h2>The ladder</h2>
      <p class="caption">Four ways to place a card, scored on the same held-out plays. Scrub the
      slider to climb from the dumbest baseline to the oracle upper bound.</p>
      <div class="toolbar">
        <input type="range" id="ladderScrub" min="0" max="3" value="0" step="1" class="anim-scrubber">
        <div class="anim-readout" id="ladderReadout"></div>
      </div>
      <div class="ladder-stage">
        <div class="ladder-list" id="ladderList"></div>
        <div class="ladder-arena rel" id="arenaHost">
          <svg id="arena" class="arena"></svg>
        </div>
      </div>
      <div class="stat-grid" id="ladderStats" style="margin-top:16px"></div>
    </section>

    <section class="report-section">
      <h2>Which cards care the most</h2>
      <p class="caption">Oracle zone accuracy minus the per-card majority table. Spells and
      cycle cards with sharp placement conventions gain the most — The Log and Barbarian Barrel
      nearly triple their table baseline.</p>
      <div class="chip-row" id="sortChips" style="margin-bottom:12px">
        <button type="button" class="chip active" data-sort="lift">by lift</button>
        <button type="button" class="chip" data-sort="oracle">by oracle accuracy</button>
        <button type="button" class="chip" data-sort="n">by sample size</button>
      </div>
      <div id="cardLift"></div>
    </section>

    <section class="report-section block-grid">
      <div class="block">
        <h2>Probe training</h2>
        <p class="caption">Frozen trunk, trainable placement head only. Zone accuracy on the
        validation set as the oracle card was taught.</p>
        <svg class="chart" id="trainChart"></svg>
        <div class="legend" id="trainLegend"></div>
      </div>
      <div class="block">
        <h2>Gates</h2>
        <div class="gate-grid" id="gates"></div>
      </div>
    </section>

    <section class="report-section">
      <h2>Lessons learned</h2>
      <ul class="lessons">{lesson_list(report.get("lessons", []))}</ul>
      <p class="caption">This probe is why policy-bc-v4 jointly trains the slot and
      card-conditioned placement heads instead of bolting a probe onto a frozen trunk.</p>
    </section>

    <footer class="report-footer">
      <p class="meta">Offline experiment only · live play blocked ·
      {esc("See also: reports/policy_bc_v4_showcase.html")}</p>
    </footer>
    """

    script = f"""
const DATA = {_json_script(payload)};
{_probe_script()}
"""
    html_doc = page(
        title="Placement probe — does the card identity unlock where?",
        body=body,
        script=script,
        extra_styles=_probe_styles(),
        include_arena=True,
    )
    out.write_text(html_doc, encoding="utf-8")
    return out


def _probe_styles() -> str:
    return """
    .wire {
      display: grid; grid-template-columns: 1fr 1fr; gap: 20px; max-width: 720px;
    }
    .wire-col {
      padding: 16px; border-radius: 14px; background: rgba(148,163,184,0.06);
      border: 1px solid rgba(148,163,184,0.16); position: relative;
    }
    .wire-col h4 { margin: 0 0 12px; font-size: 0.78rem; letter-spacing: 0.1em; color: #94a3b8; }
    .wire-box {
      padding: 10px; border-radius: 10px; text-align: center; font-size: 0.85rem;
      background: rgba(99,102,241,0.16); border: 1px solid rgba(148,163,184,0.25); margin-bottom: 8px;
    }
    .wire-box.place { background: rgba(148,163,184,0.1); }
    .wire-box.live { border-color: #34d399; box-shadow: 0 0 0 1px #34d39955; }
    .wire-note { font-size: 0.76rem; color: #94a3b8; text-align: center; margin-top: 8px; }
    .wire-note.up { color: #6ee7b7; }
    .wire-link {
      position: absolute; left: 18%; right: 18%; bottom: 52px; height: 40px; width: 64%;
    }
    .ladder-stage { display: grid; grid-template-columns: 1fr 300px; gap: 24px; align-items: start; }
    .ladder-step {
      padding: 12px 14px; border-radius: 12px; margin-bottom: 8px; cursor: pointer;
      border: 1px solid rgba(148,163,184,0.16); background: rgba(148,163,184,0.05);
      transition: border-color .2s, background .2s;
    }
    .ladder-step.active { border-color: #22d3ee; background: rgba(34,211,238,0.08); }
    .ladder-step h4 { margin: 0 0 2px; font-size: 0.95rem; }
    .ladder-step small { color: #94a3b8; font-size: 0.75rem; }
    .ladder-step .nums { margin-top: 6px; font-variant-numeric: tabular-nums; font-size: 0.85rem; }
    .lift-row {
      display: grid; grid-template-columns: 160px 1fr 110px; gap: 12px; align-items: center;
      font-size: 0.85rem; margin-bottom: 8px;
    }
    .lift-track { position: relative; height: 20px; background: rgba(148,163,184,0.1); border-radius: 6px; }
    .lift-table { position: absolute; top: 3px; height: 14px; border-radius: 4px; background: rgba(248,113,113,0.55); }
    .lift-oracle { position: absolute; top: 3px; height: 14px; border-radius: 4px;
      background: linear-gradient(90deg,#34d399,#22d3ee); opacity: 0.85; }
    .gate-grid { display: grid; gap: 10px; }
    .gate-card {
      padding: 12px 14px; border-radius: 12px; border: 1px solid rgba(148,163,184,0.18);
      background: rgba(148,163,184,0.06); font-size: 0.85rem;
    }
    .gate-card.pass { border-color: rgba(52,211,153,0.4); }
    .gate-card.fail { border-color: rgba(248,113,113,0.4); }
    .gate-card .g-status { font-weight: 800; letter-spacing: 0.06em; font-size: 0.72rem; margin-bottom: 4px; }
    @media (max-width: 900px) {
      .ladder-stage, .wire { grid-template-columns: 1fr; }
    }
    """


def _probe_script() -> str:
    return r"""
mountCounters();

/* ---------- the blind-spot wire diagram ---------- */
document.getElementById("wire").innerHTML = `
  <div class="wire-col">
    <h4>STATUS QUO (v3)</h4>
    <div class="wire-box">GRU state</div>
    <div style="display:grid;grid-template-columns:1fr 1fr;gap:8px">
      <div class="wire-box">card slot</div>
      <div class="wire-box place">zone + xy</div>
    </div>
    <p class="wire-note">placement is card-blind</p>
  </div>
  <div class="wire-col">
    <h4>THIS PROBE</h4>
    <div class="wire-box">GRU state (frozen)</div>
    <div style="display:grid;grid-template-columns:1fr 1fr;gap:8px">
      <div class="wire-box">card (oracle / soft)</div>
      <div class="wire-box place live">zone + xy probe</div>
    </div>
    <svg class="wire-link" viewBox="0 0 240 40" preserveAspectRatio="none">
      <path d="M46,6 C46,34 194,34 194,8" fill="none" stroke="#34d399" stroke-width="2.5"
            stroke-dasharray="6 5">
        <animate attributeName="stroke-dashoffset" from="0" to="-22" dur="1.4s" repeatCount="indefinite"/>
      </path>
    </svg>
    <p class="wire-note up">card identity → placement</p>
  </div>`;

/* ---------- ladder ---------- */
let rung = 0;
const tip = makeTip("arenaHost");

/* synthetic but distinctive zone distributions per rung, for the arena illustration */
const ILLUS = {
  table:  [0.02,0.04,0.02, 0.05,0.08,0.05, 0.18,0.02,0.18, 0.12,0.16,0.08],
  v3:     [0.01,0.06,0.01, 0.07,0.07,0.06, 0.16,0.01,0.14, 0.12,0.17,0.12],
  e2e:    [0.02,0.06,0.02, 0.07,0.07,0.06, 0.15,0.02,0.14, 0.12,0.16,0.11],
  oracle: [0.03,0.06,0.03, 0.07,0.08,0.07, 0.14,0.04,0.13, 0.11,0.14,0.10],
};

function renderLadder() {
  const list = document.getElementById("ladderList");
  list.innerHTML = DATA.ladder.map((r, i) =>
    `<div class="ladder-step ${i === rung ? "active" : ""}" data-i="${i}">
       <h4>${r.label}</h4><small>${r.note}</small>
       <div class="nums">zone <b>${pct(r.zone)}</b> · tile ${pct(r.tile)} · MAE ${Math.round(r.xy).toLocaleString()}</div>
     </div>`).join("");
  list.querySelectorAll(".ladder-step").forEach((el) => {
    el.addEventListener("click", () => {
      rung = Number(el.dataset.i);
      document.getElementById("ladderScrub").value = rung;
      renderLadder();
    });
  });
  const cur = DATA.ladder[rung];
  document.getElementById("ladderReadout").textContent =
    `${cur.label} · zone ${pct(cur.zone)} · tile ${pct(cur.tile)}`;
  document.getElementById("ladderStats").innerHTML = [
    ["Zone accuracy", pct(cur.zone), "of 12 zones"],
    ["Within one tile", pct(cur.tile), "hardest target"],
    ["XY MAE", Math.round(cur.xy).toLocaleString(), "API units"],
    ["vs v3 zone", ((cur.zone - DATA.ladder[1].zone) * 100).toFixed(1) + "pp", "this rung"],
  ].map(([k, v, s]) =>
    `<div class="stat"><div class="k">${k}</div><div class="v">${v}</div><div class="s">${s}</div></div>`
  ).join("");

  const { g, geom } = mountArena(document.getElementById("arena"));
  drawZoneBubbles(g, ILLUS[cur.key], geom, {
    color: rung === 3 ? "#34d399" : (rung === 0 ? "#f87171" : "#22d3ee"),
    onHover: (ev, z) => tip.show(ev, `<b>${ZONE_LABELS[z]}</b><br>illustrative share for ${cur.label}`),
    onLeave: tip.hide,
  });
  geom.captions();
}
document.getElementById("ladderScrub").addEventListener("input", (e) => {
  rung = Number(e.target.value);
  renderLadder();
});
renderLadder();

/* ---------- per-card lift ---------- */
let sortKey = "lift";
function renderCards() {
  const cards = DATA.cards.slice().sort((a, b) => {
    if (sortKey === "oracle") return b.oracle_zone_acc - a.oracle_zone_acc;
    if (sortKey === "n") return b.n - a.n;
    return b.lift_pp - a.lift_pp;
  });
  const maxZ = Math.max(...cards.map((c) => c.oracle_zone_acc), 0.1);
  document.getElementById("cardLift").innerHTML = cards.map((c) =>
    `<div class="lift-row">
       <span>${titleCase(c.card)}<br><small style="color:#94a3b8">n=${c.n}</small></span>
       <span class="lift-track">
         <span class="lift-table" style="width:${(c.table_zone_acc / maxZ) * 100}%"></span>
         <span class="lift-oracle" style="width:${(c.oracle_zone_acc / maxZ) * 100}%"></span>
       </span>
       <span style="text-align:right">${pct(c.oracle_zone_acc)}
         <span class="up">+${c.lift_pp.toFixed(1)}pp</span></span>
     </div>`).join("");
}
document.querySelectorAll("#sortChips .chip").forEach((chip) => {
  chip.addEventListener("click", () => {
    document.querySelectorAll("#sortChips .chip").forEach((c) => c.classList.remove("active"));
    chip.classList.add("active");
    sortKey = chip.dataset.sort;
    renderCards();
  });
});
renderCards();

/* ---------- training curve ---------- */
const hist = DATA.history || [];
if (hist.length) {
  lineChart(document.getElementById("trainChart"), [
    { label: "val zone", color: "#34d399", values: hist.map((r) => r.val_zone_acc) },
    { label: "val tile", color: "#22d3ee", values: hist.map((r) => r.val_tile_acc) },
  ], { yFormat: "percent", xLabel: "epoch" });
  document.getElementById("trainLegend").innerHTML = legendHTML([
    { label: "val zone", color: "#34d399" },
    { label: "val tile", color: "#22d3ee" },
  ]);
}

/* ---------- gates ---------- */
const gateLabels = {
  oracle_zone_ge_45: "Oracle zone ≥ 45%",
  oracle_ge_3pp_over_table: "Oracle ≥ +3pp over table",
  e2e_ge_3pp_over_v3: "E2E ≥ +3pp over v3",
  oracle_xy_mae_le_5000: "Oracle XY MAE ≤ 5000",
  hard_fail_oracle_lt_2pp_over_table: "Hard-fail: oracle <+2pp over table (inverted)",
};
document.getElementById("gates").innerHTML = Object.entries(DATA.gates).map(([k, v]) => {
  const inverted = k.startsWith("hard_fail");
  const passed = inverted ? !v : !!v;
  return `<div class="gate-card ${passed ? "pass" : "fail"}">
    <div class="g-status ${passed ? "up" : "down"}">${passed ? "PASS" : "FAIL"}</div>
    <div>${gateLabels[k] || titleCase(k)}</div></div>`;
}).join("");
"""
