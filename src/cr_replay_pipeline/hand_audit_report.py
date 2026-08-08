"""Interactive report for experiment A — exact-hand reconstruction audit.

Story: is the oldest-four hand heuristic the bottleneck? No. After the 4th play
it is essentially perfect, and swapping it for an exact posterior changes slot
top-1 by ~0.003pp. The report is framed as a null-result discovery, with a
cycle wheel and a bucketed recall chart as the centrepieces.
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


def render_hand_audit_report(
    audit_path: str | Path = "reports/hand_audit_v1.json",
    output_path: str | Path = "reports/hand_audit_v1.html",
) -> Path:
    report = load_json(audit_path)
    if not report:
        raise FileNotFoundError(audit_path)

    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)

    verdict = report.get("verdict", {})
    hq = report.get("heuristic_quality", {})
    buckets = hq.get("by_bucket", {})
    rescore = report.get("policy_rescore", {})
    overall = rescore.get("overall", {})
    by_bucket_rescore = rescore.get("by_bucket", {})
    defense = rescore.get("defense_slice", {})
    support = report.get("support_audit", {})
    setup = report.get("setup", {})
    compute = report.get("compute", {})
    model = report.get("model", {})
    data = report.get("data", {})

    payload = {
        "buckets": buckets,
        "rescore": {
            "overall": overall,
            "byBucket": by_bucket_rescore,
            "defense": defense,
        },
        "supportFocus": support.get("focus_cells", []),
        "untrackable": hq.get("untrackable_side_rate", 0),
        "gates": {
            "pass_slot": verdict.get("pass_slot_top1_ge_2pp", False),
            "pass_defense": verdict.get("pass_defense_slice_ge_5pp", False),
            "pass_support": verdict.get("pass_support_n_rel_ge_15pct", False),
            "fail_small": verdict.get("fail_small_deltas", False),
            "fail_recall": verdict.get("fail_high_recall_4plus", False),
        },
        "deltas": {
            "slot": verdict.get("delta_slot_top1_pp", 0),
            "defense": verdict.get("delta_defense_slice_top1_pp", 0),
            "support": verdict.get("support_focus_max_abs_rel_change", 0),
            "recall47": verdict.get("recall_4_7", 0),
            "recall8": verdict.get("recall_8_plus", 0),
        },
    }

    body = f"""
    <header class="report-header hero">
      {badge_row(
          "experiment A",
          "exact-hand audit",
          esc(report.get("model_name", "policy-bc-v3")),
          f"{hq.get('trackable_events', 0):,} trackable plays",
          esc(compute.get("device", "cpu")),
      )}
      <h1>Is the hand heuristic the bottleneck?</h1>
      <p class="hero-sub">{esc(setup.get("hypothesis", ""))}</p>
      <p class="meta">Created {esc(report.get("created_at", ""))} ·
      {compute.get("total_seconds", 0):.0f}s · {data.get("battles_total", 0):,} battles ·
      no retraining — rescoring only</p>
      {hero_scores([
          ("Slot top-1 delta", round(verdict.get("delta_slot_top1_pp", 0), 3), "flat"),
          ("Recall @ 4–7 plays", round(100 * verdict.get("recall_4_7", 0), 2), "up"),
          ("Recall @ 8+ plays", round(100 * verdict.get("recall_8_plus", 0), 2), "up"),
          ("Defense-slice delta", round(verdict.get("delta_defense_slice_top1_pp", 0), 2), "flat"),
      ])}
      {verdict_banner(verdict.get("status", "FAIL"),
          "Null result: exact hand tracking does not move the policy. The oldest-four heuristic "
          "was already good enough after the opening cycle.")}
    </header>

    <section class="report-section">
      <h2>How the 8-card cycle works</h2>
      <p class="caption">Clash Royale hands are the oldest four cards of an 8-card queue. Play
      a card and it goes to the back. The heuristic just reads the first four of that queue.
      Click <em>Play next</em> to watch a card cycle through.</p>
      <div class="cycle-stage">
        <div class="cycle-wheel" id="cycleWheel"></div>
        <div class="cycle-side">
          <div class="toolbar">
            <button type="button" class="play-btn" id="cyclePlay">▶ Play next</button>
            <button type="button" class="play-btn ghost" id="cycleReset">Reset</button>
            <span class="hint" id="cycleHint">hand = slots 0–3 (oldest four)</span>
          </div>
          <div class="stat-grid" id="cycleStats"></div>
          <p class="caption" style="margin-top:12px">The heuristic has no idea what the opening
          hand was — so for plays 0–3 it refuses to guess (recall 0). The moment the fourth card
          has been played, the queue is fully observed and the heuristic becomes essentially
          perfect.</p>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>Where the heuristic actually fails</h2>
      <p class="caption">Recall of "card is in hand" against a smoothed posterior over every
      initial hand consistent with the full side sequence. Hover a bucket.</p>
      <div class="bucket-stage rel" id="bucketHost">
        <div id="bucketBars"></div>
      </div>
      <div class="stat-grid" style="margin-top:16px">
        <div class="stat"><div class="k">Trackable events</div>
          <div class="v">{hq.get("trackable_events", 0):,}</div>
          <div class="s">of {hq.get("trackable_events", 0) + hq.get("untrackable_events", 0):,}</div></div>
        <div class="stat"><div class="k">Untrackable sides</div>
          <div class="v">{_fmt_pct(hq.get("untrackable_side_rate"))}</div>
          <div class="s">weird / incomplete sequences</div></div>
        <div class="stat"><div class="k">Slot agreement (all)</div>
          <div class="v">{_fmt_pct((buckets.get("all") or {}).get("slot_agreement"))}</div>
          <div class="s">heuristic vs posterior mode</div></div>
        <div class="stat"><div class="k">Mean consistent initials</div>
          <div class="v">{(hq.get("mean_n_consistent") or 0):,.0f}</div>
          <div class="s">posterior support size</div></div>
      </div>
    </section>

    <section class="report-section">
      <h2>What if we force the exact hand into the policy?</h2>
      <p class="caption">Rescore the frozen v3 checkpoint twice on the same held-out actions —
      once with the heuristic hand mask, once with the exact posterior mask. If the heuristic
      were the bottleneck, exact should jump.</p>
      <div class="toolbar">
        <button type="button" class="play-btn" id="rescoreBtn">▶ Run the diff</button>
        <span class="hint" id="rescoreHint">bars parked at the heuristic mask</span>
      </div>
      <div class="race" id="rescoreRace"></div>
      <div class="null-banner">
        <b>Δ slot top-1 = {verdict.get("delta_slot_top1_pp", 0):+.3f}pp</b>
        — smaller than rounding error. Exact hand tracking is not the next lever.
      </div>
    </section>

    <section class="report-section block-grid">
      <div class="block">
        <h2>Rescore by cycle age</h2>
        <p class="caption">Even the early-cycle bucket, where the heuristic is blind, barely
        moves under the exact mask — the policy was not relying on the mask that hard.</p>
        <div id="bucketRescore"></div>
      </div>
      <div class="block">
        <h2>Support-cell sensitivity</h2>
        <p class="caption">Focus cells from the defense support audit: how much does
        "answer in hand" count change under the exact mask?</p>
        <div id="supportCells"></div>
      </div>
    </section>

    <section class="report-section">
      <h2>Gates</h2>
      <div class="gate-grid" id="gates"></div>
    </section>

    <section class="report-section">
      <h2>Lessons learned</h2>
      <ul class="lessons">{lesson_list(report.get("lessons", []))}</ul>
      <p class="caption">Policy {esc(model.get("model_version", ""))} checkpoint
      {esc(model.get("path", ""))}. Keep the oldest-four heuristic; spend compute on placement
      and threat conditioning instead.</p>
    </section>

    <footer class="report-footer">
      <p class="meta">Offline audit · no live games · cycle model: {esc(setup.get("cycle_model", ""))}</p>
    </footer>
    """

    script = f"""
const DATA = {_json_script(payload)};
{_hand_script()}
"""
    html_doc = page(
        title="Hand audit — is oldest-four the bottleneck?",
        body=body,
        script=script,
        extra_styles=_hand_styles(),
    )
    out.write_text(html_doc, encoding="utf-8")
    return out


def _fmt_pct(value: float | None) -> str:
    if value is None:
        return "—"
    return f"{100 * value:.1f}%"


def _hand_styles() -> str:
    return """
    .cycle-stage { display: grid; grid-template-columns: 320px 1fr; gap: 28px; align-items: start; }
    .cycle-wheel {
      width: 300px; height: 300px; margin: 0 auto; position: relative;
      border-radius: 50%; background: radial-gradient(circle at 50% 50%, #0b1220 42%, transparent 43%),
        conic-gradient(from -90deg, #22d3ee33 0 180deg, #6366f133 180deg 360deg);
      border: 1px solid rgba(148,163,184,0.2);
    }
    .cycle-slot {
      position: absolute; width: 64px; height: 64px; margin: -32px; border-radius: 14px;
      display: grid; place-items: center; text-align: center; font-size: 0.72rem; font-weight: 700;
      border: 2px solid rgba(148,163,184,0.35); background: #0f172a; color: #e2e8f0;
      transition: transform .35s cubic-bezier(.22,1,.36,1), border-color .2s, box-shadow .2s;
    }
    .cycle-slot.in-hand { border-color: #22d3ee; box-shadow: 0 0 0 2px #22d3ee55; background: #12233a; }
    .cycle-slot.just-played { border-color: #fbbf24; box-shadow: 0 0 0 2px #fbbf2455; }
    .cycle-slot .idx { position: absolute; top: 4px; left: 6px; font-size: 0.6rem; color: #94a3b8; }
    .bucket-row {
      display: grid; grid-template-columns: 100px 1fr 160px; gap: 12px; align-items: center;
      font-size: 0.85rem; margin-bottom: 10px;
    }
    .bucket-track { height: 22px; border-radius: 7px; background: rgba(148,163,184,0.1); overflow: hidden; }
    .bucket-fill { height: 100%; border-radius: 7px; }
    .null-banner {
      margin-top: 16px; padding: 14px 18px; border-radius: 12px;
      background: rgba(251,191,36,0.1); border: 1px solid rgba(251,191,36,0.35);
      font-size: 0.92rem;
    }
    .gate-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 12px; }
    .gate-card {
      padding: 12px 14px; border-radius: 12px; border: 1px solid rgba(148,163,184,0.18);
      background: rgba(148,163,184,0.06); font-size: 0.85rem;
    }
    .gate-card.pass { border-color: rgba(52,211,153,0.4); }
    .gate-card.fail { border-color: rgba(248,113,113,0.4); }
    .gate-card .g-status { font-weight: 800; letter-spacing: 0.06em; font-size: 0.72rem; margin-bottom: 4px; }
    .cell-row {
      display: grid; grid-template-columns: 1fr 70px 70px; gap: 8px; font-size: 0.82rem;
      padding: 6px 0; border-bottom: 1px solid rgba(148,163,184,0.12);
    }
    @media (max-width: 900px) { .cycle-stage { grid-template-columns: 1fr; } }
    """


def _hand_script() -> str:
    return r"""
mountCounters();

/* ---------- interactive 8-card cycle wheel ---------- */
const DECK = ["Knight","Archers","Goblin Gang","Fireball","Cannon","Skeletons","The Log","Ice Spirit"];
let queue = DECK.slice();
let plays = 0;

function slotPos(i) {
  // 0 at top, clockwise
  const ang = -Math.PI / 2 + i * (Math.PI / 4);
  const r = 105;
  return { x: 150 + r * Math.cos(ang), y: 150 + r * Math.sin(ang) };
}

function renderCycle() {
  const host = document.getElementById("cycleWheel");
  host.innerHTML = "";
  queue.forEach((card, i) => {
    const p = slotPos(i);
    const d = document.createElement("div");
    d.className = "cycle-slot" + (i < 4 ? " in-hand" : "");
    d.style.left = p.x + "px";
    d.style.top = p.y + "px";
    d.innerHTML = `<span class="idx">${i}</span>${card}`;
    host.appendChild(d);
  });
  const hand = queue.slice(0, 4).join(", ");
  document.getElementById("cycleHint").textContent =
    plays < 4
      ? `play ${plays}/3 — heuristic still blind to the opening hand`
      : `play ${plays} — heuristic hand: ${hand}`;
  document.getElementById("cycleStats").innerHTML = [
    ["Plays so far", plays, "this demo"],
    ["Heuristic active", plays >= 4 ? "yes" : "no", "needs 4 observations"],
    ["Cards in hand", "4", queue.slice(0, 4).join(" · ")],
    ["Next up (queue)", queue[4], "will enter hand after 4 more plays"],
  ].map(([k, v, s]) =>
    `<div class="stat"><div class="k">${k}</div><div class="v">${v}</div><div class="s">${s}</div></div>`
  ).join("");
}

document.getElementById("cyclePlay").addEventListener("click", () => {
  const played = queue.shift();
  queue.push(played);
  plays += 1;
  renderCycle();
  // flash the card that just went to the back
  const slots = document.querySelectorAll(".cycle-slot");
  if (slots[7]) slots[7].classList.add("just-played");
});
document.getElementById("cycleReset").addEventListener("click", () => {
  queue = DECK.slice();
  plays = 0;
  renderCycle();
});
renderCycle();

/* ---------- bucket recall ---------- */
const order = ["0-3", "4-7", "8+", "all"];
const tip = makeTip("bucketHost");
document.getElementById("bucketBars").innerHTML = order.map((key) => {
  const b = DATA.buckets[key] || {};
  const recall = b.recall || 0;
  const color = key === "0-3" ? "#f87171" : (key === "all" ? "#a78bfa" : "#34d399");
  return `<div class="bucket-row" data-key="${key}">
    <span>plays ${key}<br><small style="color:#94a3b8">${(b.events || 0).toLocaleString()} events</small></span>
    <span class="bucket-track"><span class="bucket-fill" style="width:${recall * 100}%;background:${color}"></span></span>
    <span style="text-align:right">recall ${pct(recall, 2)}<br><small style="color:#94a3b8">precision ${pct(b.precision || 0, 2)}</small></span>
  </div>`;
}).join("");
document.querySelectorAll(".bucket-row").forEach((row) => {
  row.addEventListener("mousemove", (ev) => {
    const b = DATA.buckets[row.dataset.key] || {};
    tip.show(ev, `<b>plays ${row.dataset.key}</b><br>tp ${b.tp} · fp ${b.fp} · fn ${b.fn}<br>slot agreement ${pct(b.slot_agreement || 0, 2)}`);
  });
  row.addEventListener("mouseleave", tip.hide);
});

/* ---------- rescore race ---------- */
const o = DATA.rescore.overall || {};
const h = o.heuristic || {};
const e = o.exact || {};
const def = DATA.rescore.defense || {};
mountRace("rescoreRace", [
  {
    label: "Slot top-1",
    note: `Same ${h.n || 0} held-out actions, only the hand mask changes.`,
    old: h.slot_top1 || 0, new: e.slot_top1 || 0, fmt: "pct", higher: true,
  },
  {
    label: "Slot top-3",
    note: "Human's card inside the model's three best guesses.",
    old: h.slot_top3 || 0, new: e.slot_top3 || 0, fmt: "pct", higher: true,
  },
  {
    label: "Defense-slice top-1",
    note: "Real reaction windows only.",
    old: (def.heuristic || {}).top1 || 0, new: (def.exact || {}).top1 || 0, fmt: "pct", higher: true,
  },
], {
  buttonId: "rescoreBtn", hintId: "rescoreHint", oldName: "heur",
  playLabel: "▶ Run the diff", backLabel: "↺ Back to heuristic",
  hintBefore: "bars parked at the heuristic mask",
  hintAfter: "bars moved to the exact-hand mask — barely a wiggle",
});

/* ---------- rescore by bucket ---------- */
const rb = DATA.rescore.byBucket || {};
document.getElementById("bucketRescore").innerHTML = ["0-3", "4-7", "8+"].map((key) => {
  const blk = rb[key] || {};
  const d = ((blk.exact || {}).slot_top1 || 0) - ((blk.heuristic || {}).slot_top1 || 0);
  return `<div class="cell-row">
    <span>plays ${key}</span>
    <span>${pct((blk.heuristic || {}).slot_top1 || 0)}</span>
    <span>${pct((blk.exact || {}).slot_top1 || 0)}
      <span class="${d >= 0 ? "up" : "down"}">${d >= 0 ? "+" : ""}${(d * 100).toFixed(2)}pp</span></span>
  </div>`;
}).join("") + `<div class="cell-row" style="color:#94a3b8;font-size:.75rem">
  <span></span><span>heuristic</span><span>exact</span></div>`;

/* ---------- support cells ---------- */
const cells = DATA.supportFocus || [];
document.getElementById("supportCells").innerHTML = cells.length
  ? cells.map((c) => {
      const hN = (c.train || {}).n_threat_answer_in_hand ?? c.n_threat_answer_in_hand ?? "—";
      return `<div class="cell-row">
        <span>${titleCase(c.threat)} → ${titleCase(c.answer)}</span>
        <span colspan="2" style="grid-column:2/4">in-hand n (train) ${hN}</span>
      </div>`;
    }).join("") +
    `<p class="caption" style="margin-top:10px">Max |relative change| on focus cells:
      <b>${(DATA.deltas.support * 100).toFixed(1)}%</b> (gate was ≥15%)</p>`
  : `<p class="caption">No focus cells recorded.</p>`;

/* ---------- gates ---------- */
const gates = [
  ["pass_slot", "Slot top-1 ≥ +2pp", DATA.gates.pass_slot],
  ["pass_defense", "Defense-slice ≥ +5pp", DATA.gates.pass_defense],
  ["pass_support", "Support n relative change ≥ 15%", DATA.gates.pass_support],
  ["fail_small", "Fail: deltas tiny (triggered)", !DATA.gates.fail_small],
  ["fail_recall", "Fail: recall@4+ already ~1 (triggered)", !DATA.gates.fail_recall],
];
document.getElementById("gates").innerHTML = gates.map(([, label, passed]) =>
  `<div class="gate-card ${passed ? "pass" : "fail"}">
    <div class="g-status ${passed ? "up" : "down"}">${passed ? "PASS" : "FAIL"}</div>
    <div>${label}</div></div>`
).join("");
"""
