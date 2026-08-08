"""Interactive HTML report for the realism scorer.

Story: a binary judge of whether an action sequence looks like a real Clash
Royale battle. Easy negatives are chaos (trivial to reject); hard negatives are
perturbed real games (the actual test). The ludic piece is a spot-the-fake
round built from a few held-out battles.
"""

from __future__ import annotations

import json
import random
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


def _build_spot_the_fake(
    input_dir: str | Path = "data/raw",
    card_costs_path: str | Path = "data/card_costs.json",
    model_dir: str | Path = "models/realism_scorer",
    n_rounds: int = 8,
    seed: int = 7,
) -> list[dict[str, Any]]:
    """Build a handful of real-vs-fake rounds for the interactive quiz."""
    try:
        import pickle

        from .realism_generate import (
            generate_easy_negative,
            generate_hard_negative,
            generate_medium_negative,
            TimingPrior,
        )
        from .winner_dataset import collect_battles, load_card_costs, split_battles
    except Exception:
        return []

    costs = load_card_costs(card_costs_path)
    battles = collect_battles(input_dir)
    _tr, _va, test = split_battles(battles, seed=42)
    rng = random.Random(seed)
    pool = [b for b in test if len(b.events) >= 20]
    rng.shuffle(pool)
    pool = pool[: max(n_rounds * 2, 16)]
    timing = TimingPrior.from_battles(_tr[:800] if _tr else pool)

    ckpt = Path(model_dir) / "realism_ensemble.pkl"
    if not ckpt.exists():
        return []
    with ckpt.open("rb") as handle:
        bundle = pickle.load(handle)

    from .policy_train import _score_realism

    gens = {
        "easy": generate_easy_negative,
        "medium": generate_medium_negative,
        "hard": generate_hard_negative,
    }

    def fingerprint(battle: Any) -> dict[str, Any]:
        sides = [e["side"] for e in battle.events if e.get("event_type") == "card_play"]
        alt = sum(1 for a, b in zip(sides, sides[1:]) if a != b)
        alt_rate = alt / max(len(sides) - 1, 1)
        delays = []
        prev = None
        for e in battle.events:
            if e.get("event_type") != "card_play":
                continue
            if prev is not None:
                delays.append(max(0.0, float(e["seconds"]) - prev))
            prev = float(e["seconds"])
        mean_dt = sum(delays) / max(len(delays), 1)
        return {
            "n_plays": len(sides),
            "alt_rate": round(alt_rate, 3),
            "mean_delay": round(mean_dt, 2),
            "duration": round(float(battle.events[-1]["seconds"]), 1) if battle.events else 0,
            "team_cards": list(battle.team_deck[:4]),
            "opp_cards": list(battle.opponent_deck[:4]),
        }

    rounds: list[dict[str, Any]] = []
    tiers = ["easy", "medium", "hard", "hard", "medium", "easy", "hard", "medium"]
    for i, tier in enumerate(tiers[:n_rounds]):
        if i >= len(pool):
            break
        real = pool[i]
        fake = gens[tier](real, costs, random.Random(seed + 17 * i), timing)
        scores = _score_realism(bundle, [real, fake], costs)
        real_first = rng.random() < 0.5
        left = real if real_first else fake
        right = fake if real_first else real
        left_score = scores[0] if real_first else scores[1]
        right_score = scores[1] if real_first else scores[0]
        rounds.append(
            {
                "tier": tier,
                "real_side": "left" if real_first else "right",
                "left": {
                    **fingerprint(left),
                    "score": round(float(left_score), 4),
                    "battle_id": left.battle_id.split("::")[0][:12],
                },
                "right": {
                    **fingerprint(right),
                    "score": round(float(right_score), 4),
                    "battle_id": right.battle_id.split("::")[0][:12],
                },
            }
        )
    return rounds


def render_realism_report(
    model_dir: str | Path = "models/realism_scorer",
    output_path: str | Path | None = None,
    build_quiz: bool = True,
) -> Path:
    model_dir = Path(model_dir)
    report = load_json(model_dir / "report.json")
    if not report:
        raise FileNotFoundError(model_dir / "report.json")

    report_dir = Path(output_path).parent if output_path else Path("reports")
    report_dir.mkdir(parents=True, exist_ok=True)
    out = Path(output_path) if output_path else report_dir / "realism_scorer_v1.html"

    created = _report_timestamp(model_dir, "report.json", "realism_ensemble.pkl")
    data = report["data"]
    test = report["test"]
    compute = report["compute"]
    tiers = test.get("by_tier", [])
    stages = report.get("training_stages", [])
    histogram = report.get("score_histogram", {})
    lessons = report.get("lessons", [])

    quiz_path = report_dir / "realism_spot_the_fake.json"
    quiz: list[dict[str, Any]] = []
    if build_quiz:
        if quiz_path.exists():
            quiz = load_json(quiz_path)  # type: ignore[assignment]
            if isinstance(quiz, dict):
                quiz = quiz.get("rounds", [])
        else:
            print("Building spot-the-fake rounds (one-time)…", flush=True)
            quiz = _build_spot_the_fake(model_dir=model_dir)
            quiz_path.write_text(json.dumps({"rounds": quiz}, indent=2), encoding="utf-8")
            print(f"Wrote {quiz_path} ({len(quiz)} rounds)", flush=True)

    payload = {
        "test": {k: v for k, v in test.items() if k != "by_tier"},
        "tiers": tiers,
        "histogram": histogram,
        "stages": stages,
        "quiz": quiz,
        "baseline": report.get("baseline", {}),
    }

    hard = next((t for t in tiers if t["tier"] == "hard"), {})
    easy = next((t for t in tiers if t["tier"] == "easy"), {})

    body = f"""
    <header class="report-header hero">
      {badge_row(
          esc(report.get("model_name", "realism-scorer-v1")),
          "sequence realism judge",
          esc(compute.get("framework", "sklearn")),
          f"{data.get('battles_total', 0):,} battles",
          esc(compute.get("device", "cpu")),
      )}
      <h1>Does this battle look real?</h1>
      <p class="hero-sub">A binary scorer P(sequence is real) trained against tiered legal
      negatives. Easy chaos is a weak test; hard perturbations of real games are the ones that
      matter. This judge is what grades policy rollouts offline.</p>
      <p class="meta">Created {esc(created)} · trained in {report.get("seconds", 0)}s ·
      {data.get("feature_dim", "?")} features · {data.get("negatives_per_real", 3)} negatives / real</p>
      {hero_scores([
          ("Test AUC", round(test.get("auc", 0), 3), "up"),
          ("Test accuracy", round(100 * test.get("acc", 0), 1), "up"),
          ("Hard-tier AUC", round(hard.get("auc", 0), 3), "up"),
          ("Easy reject @0.5", round(100 * easy.get("reject_rate_at_0.5", 0)), "up"),
      ])}
      {verdict_banner(
          "READY" if hard.get("auc", 0) >= 0.9 else "WEAK HARD",
          "Hard-tier AUC is the number to watch — easy/medium near-perfect scores just mean "
          "the scorer can smell chaos.",
      )}
    </header>

    <section class="report-section">
      <h2>What each negative tier does</h2>
      <p class="caption">Same real battle, three increasingly subtle corruptions. The scorer
      has to keep saying "fake" as the corruption gets quieter.</p>
      <div class="tier-cards">
        <div class="tier-card easy">
          <h3>Easy</h3>
          <p>Shuffle legal cards into nonsense order, ignore elixir tempo. A monkey with a
          deck. Should score near 0.</p>
          <div class="tier-score">mean P(real) = <b>{(easy.get("mean_score_negatives") or 0):.4f}</b></div>
        </div>
        <div class="tier-card medium">
          <h3>Medium</h3>
          <p>Keep side structure, resample delays and some placements. Looks like a game from
          far away; wrong up close.</p>
          <div class="tier-score">mean P(real) =
            <b>{next((t.get("mean_score_negatives", 0) for t in tiers if t["tier"]=="medium"), 0):.4f}</b></div>
        </div>
        <div class="tier-card hard">
          <h3>Hard</h3>
          <p>Perturb a real game: swap a few placements, stretch a delay, drop a play. The
          only tier that still fools the scorer sometimes.</p>
          <div class="tier-score">mean P(real) = <b>{(hard.get("mean_score_negatives") or 0):.3f}</b></div>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>Score distributions</h2>
      <p class="caption">Where the scorer puts real battles vs all synthetics. Real should pile
      up on the right; fakes on the left. Hover a bin.</p>
      <div class="rel" id="histHost">
        <svg class="chart" id="scoreHist"></svg>
        <div class="legend-row">
          <span><i class="swatch bar" style="background:#34d399"></i>real</span>
          <span><i class="swatch bar" style="background:#f87171"></i>synthetic</span>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>Tier ladder</h2>
      <p class="caption">Accuracy and mean fake-score by tier. Hit <em>Run the diff</em> to
      climb from the majority-class baseline to the scorer.</p>
      <div class="toolbar">
        <button type="button" class="play-btn" id="tierRaceBtn">▶ Run the diff</button>
        <span class="hint" id="tierRaceHint">bars parked at majority baseline (75%)</span>
      </div>
      <div class="race" id="tierRace"></div>
    </section>

    <section class="report-section quiz-section">
      <h2>Spot the fake</h2>
      <p class="caption">Two battle fingerprints. One is real, one was corrupted at the shown
      tier. Pick which looks real — then see what the scorer said. Feature stats only; no
      live play.</p>
      <div id="quiz"></div>
      <div class="quiz-score" id="quizScore"></div>
    </section>

    <section class="report-section">
      <h2>Training, tree by tree</h2>
      <p class="caption">Scrub the boosting stages. Solid = HGB ensemble, dashed = ExtraTrees
      alone.</p>
      <div class="chart-animation">
        <svg class="chart" id="trainChart"></svg>
        <div class="legend" id="trainLegend"></div>
        <div class="anim-toolbar">
          <button type="button" class="play-btn" id="trainPlay">▶</button>
          <input type="range" class="anim-scrubber" id="trainScrub" min="1" max="1" value="1" step="1">
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

    script = f"""
const DATA = {_json_script(payload)};
{_realism_script()}
"""
    html_doc = page(
        title="Realism scorer — does this battle look real?",
        body=body,
        script=script,
        extra_styles=_realism_styles(),
    )
    out.write_text(html_doc, encoding="utf-8")
    return out


def _realism_styles() -> str:
    return """
    .tier-cards { display: grid; grid-template-columns: repeat(3, 1fr); gap: 14px; }
    .tier-card {
      padding: 16px; border-radius: 14px; border: 1px solid rgba(148,163,184,0.18);
      background: rgba(148,163,184,0.06); font-size: 0.88rem;
    }
    .tier-card h3 { margin: 0 0 8px; }
    .tier-card.easy { border-color: rgba(52,211,153,0.35); }
    .tier-card.medium { border-color: rgba(251,191,36,0.35); }
    .tier-card.hard { border-color: rgba(248,113,113,0.35); }
    .tier-score { margin-top: 12px; font-size: 0.8rem; color: #94a3b8; }
    .tier-score b { color: #e2e8f0; font-size: 1.05rem; }
    .quiz-section { background: rgba(99,102,241,0.06); border-radius: 20px; }
    .quiz-card {
      border: 1px solid rgba(148,163,184,0.2); border-radius: 18px; padding: 20px;
      background: rgba(11,18,32,0.6);
    }
    .quiz-pair { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin: 14px 0; }
    .battle-card {
      padding: 16px; border-radius: 14px; border: 1px solid rgba(148,163,184,0.25);
      background: rgba(148,163,184,0.06); cursor: pointer; text-align: left; color: #e2e8f0;
      transition: transform .15s, border-color .15s;
    }
    .battle-card:hover:not(:disabled) { transform: translateY(-3px); border-color: #22d3ee; }
    .battle-card:disabled { cursor: default; }
    .battle-card.correct { border-color: #34d399; box-shadow: 0 0 0 2px #34d39955; }
    .battle-card.wrong { border-color: #f87171; }
    .battle-card h4 { margin: 0 0 8px; }
    .battle-card .fp { font-size: 0.8rem; color: #cbd5f5; line-height: 1.45; }
    .quiz-verdict { min-height: 24px; font-size: 0.9rem; margin-top: 8px; }
    .quiz-score { display: flex; gap: 16px; flex-wrap: wrap; margin-top: 14px; }
    .score-pill { padding: 9px 16px; border-radius: 12px; background: rgba(148,163,184,0.1); font-size: 0.9rem; }
    @media (max-width: 900px) {
      .tier-cards, .quiz-pair { grid-template-columns: 1fr; }
    }
    """


def _realism_script() -> str:
    return r"""
mountCounters();

/* ---------- overlapping histograms ---------- */
(function () {
  const hist = DATA.histogram || {};
  const svg = document.getElementById("scoreHist");
  if (!hist.real) return;
  const W = 720, H = 240, ML = 44, MB = 34, MT = 14, MR = 12;
  svg.setAttribute("viewBox", `0 0 ${W} ${H}`);
  const peak = Math.max(...hist.real, ...hist.synthetic, 1);
  const n = hist.real.length;
  const bw = (W - ML - MR) / n;
  const tip = makeTip("histHost");
  const edges = hist.edges || [];
  hist.synthetic.forEach((c, i) => {
    const h = (c / peak) * (H - MB - MT);
    const bar = el("rect", {
      x: ML + i * bw + 1, y: H - MB - h, width: bw - 2, height: Math.max(h, c ? 1 : 0),
      fill: "#f87171", opacity: 0.55, rx: 1,
    });
    bar.addEventListener("mousemove", (ev) => tip.show(ev,
      `<b>synthetic ${c}</b><br>P(real) ${(edges[i]||0).toFixed(2)}–${(edges[i+1]||0).toFixed(2)}`));
    bar.addEventListener("mouseleave", tip.hide);
    svg.appendChild(bar);
  });
  hist.real.forEach((c, i) => {
    const h = (c / peak) * (H - MB - MT);
    const bar = el("rect", {
      x: ML + i * bw + bw * 0.25, y: H - MB - h, width: bw * 0.5, height: Math.max(h, c ? 1 : 0),
      fill: "#34d399", opacity: 0.9, rx: 1,
    });
    bar.addEventListener("mousemove", (ev) => tip.show(ev,
      `<b>real ${c}</b><br>P(real) ${(edges[i]||0).toFixed(2)}–${(edges[i+1]||0).toFixed(2)}`));
    bar.addEventListener("mouseleave", tip.hide);
    svg.appendChild(bar);
  });
  svg.appendChild(el("line", { x1: ML, x2: W - MR, y1: H - MB, y2: H - MB, stroke: "rgba(148,163,184,0.3)" }));
  [0, 0.5, 1].forEach((t) => {
    svg.appendChild(svgText({
      x: ML + t * (W - ML - MR), y: H - MB + 16, "text-anchor": "middle", fill: "#94a3b8", "font-size": 11,
    }, t.toFixed(1)));
  });
})();

/* ---------- tier race ---------- */
const baseAcc = (DATA.baseline.test || DATA.baseline || {}).acc || 0.75;
mountRace("tierRace", [
  { label: "Overall accuracy", note: "Real + all tiers mixed.",
    old: baseAcc, new: DATA.test.acc || 0, fmt: "pct", higher: true },
  ...DATA.tiers.map((t) => ({
    label: titleCase(t.tier) + "-tier accuracy",
    note: `mean fake score ${(t.mean_score_negatives || 0).toFixed(4)} · reject@0.5 ${pct(t["reject_rate_at_0.5"] || 0)}`,
    old: baseAcc, new: t.acc || 0, fmt: "pct", higher: true,
    soft: t.tier === "hard",
  })),
], {
  buttonId: "tierRaceBtn", hintId: "tierRaceHint", oldName: "base",
  playLabel: "▶ Run the diff", backLabel: "↺ Back to baseline",
  hintBefore: "bars parked at majority baseline (75%)",
  hintAfter: "bars moved to the scorer",
});

/* ---------- spot the fake ---------- */
const quiz = DATA.quiz || [];
let qi = 0;
const tally = { you: 0, model: 0, done: 0 };

function renderScore() {
  const host = document.getElementById("quizScore");
  if (!tally.done) {
    host.innerHTML = `<div class="score-pill">${quiz.length ? "Pick a side to start the scoreboard." : "Quiz data not built — re-run report-realism."}</div>`;
    return;
  }
  host.innerHTML = [
    ["You", tally.you, "#22d3ee"],
    ["Scorer", tally.model, "#34d399"],
  ].map(([n, v, c]) =>
    `<div class="score-pill" style="border-left:3px solid ${c}">${n} correct
      <b>${v}/${tally.done}</b></div>`).join("");
}

function fpHTML(side, label) {
  return `<button type="button" class="battle-card" data-side="${label}">
    <h4>Battle ${side.battle_id}</h4>
    <div class="fp">
      ${side.n_plays} card plays · ${side.duration}s long<br>
      alternation rate ${pct(side.alt_rate)} · mean Δt ${side.mean_delay}s<br>
      team: ${side.team_cards.map(titleCase).join(", ")}…
    </div>
  </button>`;
}

function renderQuiz() {
  const host = document.getElementById("quiz");
  if (!quiz.length) {
    host.innerHTML = `<p class="caption">No spot-the-fake rounds on disk. Run
      <code>cr-replays report-realism</code> once to build them.</p>`;
    return;
  }
  const r = quiz[qi];
  const wrap = document.createElement("div");
  wrap.className = "quiz-card";
  wrap.innerHTML = `<p>Tier: <b class="${r.tier}">${titleCase(r.tier)}</b> corruption ·
    round ${qi + 1}/${quiz.length}</p>
    <div class="quiz-pair">${fpHTML(r.left, "left")}${fpHTML(r.right, "right")}</div>
    <div class="quiz-verdict"></div>
    <div class="toolbar" style="margin-top:12px">
      <button type="button" class="play-btn" id="quizNext">Next round →</button>
    </div>`;
  host.innerHTML = "";
  host.appendChild(wrap);
  const verdict = wrap.querySelector(".quiz-verdict");
  wrap.querySelectorAll(".battle-card").forEach((btn) => {
    btn.addEventListener("click", () => {
      const pick = btn.dataset.side;
      const correct = pick === r.real_side;
      tally.done += 1;
      if (correct) tally.you += 1;
      const modelLeft = r.left.score >= r.right.score ? "left" : "right";
      if (modelLeft === r.real_side) tally.model += 1;
      wrap.querySelectorAll(".battle-card").forEach((b) => {
        b.disabled = true;
        if (b.dataset.side === r.real_side) b.classList.add("correct");
        else if (b.dataset.side === pick) b.classList.add("wrong");
      });
      verdict.innerHTML =
        `${correct ? "<span class='up'>You got it.</span>" : "<span class='down'>That was the fake.</span>"}
         Scorer: left P(real)=${r.left.score.toFixed(3)}, right P(real)=${r.right.score.toFixed(3)}
         → picked <b>${modelLeft}</b> ${modelLeft === r.real_side ? "(correct)" : "(wrong)"} .`;
      renderScore();
    }, { once: true });
  });
  wrap.querySelector("#quizNext").addEventListener("click", () => {
    qi = (qi + 1) % quiz.length;
    renderQuiz();
  });
}
renderQuiz();
renderScore();

/* ---------- training scrub ---------- */
const stages = DATA.stages || [];
const scrub = document.getElementById("trainScrub");
const chart = document.getElementById("trainChart");
scrub.max = Math.max(stages.length, 1);
scrub.value = stages.length;
function drawTrain(upto) {
  const slice = stages.slice(0, upto);
  lineChart(chart, [
    { label: "HGB AUC", color: "#34d399", values: slice.map((s) => s.hgb_auc ?? s.auc) },
    { label: "HGB acc", color: "#22d3ee", values: slice.map((s) => s.hgb_acc ?? s.acc) },
    { label: "Trees AUC", color: "#f87171", values: slice.map((s) => s.tree_auc), dash: "4 3" },
  ], { yFormat: "percent", xLabel: "boosting stage" });
  const last = slice[slice.length - 1];
  document.getElementById("trainReadout").textContent = last
    ? `trees ${last.trees} · AUC ${(last.hgb_auc ?? last.auc).toFixed(3)} · acc ${pct(last.hgb_acc ?? last.acc)}`
    : "";
}
document.getElementById("trainLegend").innerHTML = legendHTML([
  { label: "HGB AUC", color: "#34d399" },
  { label: "HGB acc", color: "#22d3ee" },
  { label: "ExtraTrees AUC", color: "#f87171" },
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
  }, 80);
});
"""
