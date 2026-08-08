"""Interactive HTML showcase comparing policy v4 against v3.

Everything is rendered with native SVG/CSS/JS: an explorable arena heatmap, a
"what got fixed" gallery, and a playable defense quiz that pits the reader
against both checkpoints on real held-out reaction windows.
"""

from __future__ import annotations

import html
import json
from pathlib import Path
from typing import Any

from .winner_report import _base_styles, _fmt_pct, _json_script


def _load(path: str | Path) -> dict[str, Any]:
    p = Path(path)
    if not p.exists():
        return {}
    with p.open(encoding="utf-8") as handle:
        return json.load(handle)


def _metric_rows(
    overall: dict[str, Any],
    new_report: dict[str, Any],
    old_report: dict[str, Any],
    new_slice: dict[str, Any],
    old_slice: dict[str, Any],
) -> list[dict[str, Any]]:
    """Head-to-head rows (both checkpoints rescored on identical actions) come
    first; archived training-run numbers are flagged because the dataset kept
    growing between the two runs."""
    new_test = new_report.get("test", {})
    old_test = old_report.get("test", {})
    new_roll = new_report.get("rollouts", {})
    old_roll = old_report.get("rollouts", {})
    n_head = overall.get("n", 0)
    old_n = (old_slice.get("overall") or {}).get("n", 0)
    new_n = (new_slice.get("overall") or {}).get("n", 0)
    rows = [
        {
            "label": "Which card (slot top-1)",
            "old": overall.get("old_slot_acc"),
            "new": overall.get("new_slot_acc"),
            "fmt": "pct",
            "higher": True,
            "fair": True,
            "note": f"Right card out of the 8-card deck, on {n_head:,} identical actions.",
        },
        {
            "label": "Card in top-3",
            "old": overall.get("old_slot_top3"),
            "new": overall.get("new_slot_top3"),
            "fmt": "pct",
            "higher": True,
            "fair": True,
            "note": "Human's card sits inside the model's three best guesses.",
        },
        {
            "label": "Where (zone accuracy)",
            "old": overall.get("old_zone_acc"),
            "new": overall.get("new_zone_acc"),
            "fmt": "pct",
            "higher": True,
            "fair": True,
            "note": "12 arena zones. v4 sees which card it is placing.",
        },
        {
            "label": "Within one tile",
            "old": overall.get("old_tile_acc"),
            "new": overall.get("new_tile_acc"),
            "fmt": "pct",
            "higher": True,
            "fair": True,
            "note": "Pixel-tight placement, the hardest target of all.",
        },
        {
            "label": "Placement error",
            "old": overall.get("old_xy_mae"),
            "new": overall.get("new_xy_mae"),
            "fmt": "int",
            "higher": False,
            "fair": True,
            "note": "Mean distance to the human tile in API units (lower is better).",
        },
        {
            "label": "Real defense windows",
            "old": (old_slice.get("overall") or {}).get("top1"),
            "new": (new_slice.get("overall") or {}).get("top1"),
            "fmt": "pct",
            "higher": True,
            "fair": False,
            "note": f"Top-1 answer to a real push · {old_n:,} vs {new_n:,} windows.",
        },
        {
            "label": "Rollout realism P(real)",
            "old": old_roll.get("mean_score_policy"),
            "new": new_roll.get("mean_score_policy"),
            "fmt": "float",
            "higher": True,
            "fair": False,
            "note": "Realism scorer on 40-action self-continuations.",
        },
        {
            "label": "Timing error",
            "old": old_test.get("timing_mae"),
            "new": new_test.get("timing_mae"),
            "fmt": "sec",
            "higher": False,
            "fair": False,
            "note": "Seconds until the next action (lower is better).",
        },
    ]
    return [r for r in rows if r["old"] is not None and r["new"] is not None]


def _cell_pairs(
    new_support: dict[str, Any], old_support: dict[str, Any]
) -> list[dict[str, Any]]:
    old_by_key = {
        (c["threat"], c["answer"]): c for c in old_support.get("cells", [])
    }
    pairs = []
    for cell in new_support.get("cells", []):
        key = (cell["threat"], cell["answer"])
        old = old_by_key.get(key, {})
        new_v = cell.get("model_top1_answer_given_in_hand")
        old_v = old.get("model_top1_answer_given_in_hand")
        if new_v is None or old_v is None:
            continue
        pairs.append(
            {
                "threat": cell["threat"],
                "answer": cell["answer"],
                "label": f"{cell['threat'].replace('-', ' ')} → {cell['answer'].replace('-', ' ')}",
                "old": old_v,
                "new": new_v,
                "human": cell.get("human_use_rate_given_in_hand"),
                "support": cell.get("support") or cell.get("verdict") or "",
                "n": cell.get("n_threat_answer_in_hand", 0),
                "role": cell.get("role", ""),
            }
        )
    pairs.sort(key=lambda p: -(p["new"] - p["old"]))
    return pairs


def render_policy_showcase_report(
    showcase_path: str | Path = "reports/policy_showcase_v4.json",
    model_dir: str | Path = "models/policy_bc_v4",
    old_model_dir: str | Path = "models/policy_bc_v3",
    slice_path: str | Path = "reports/defense_slice_eval_v4.json",
    support_path: str | Path = "reports/defense_support_audit_v4.json",
    old_slice_path: str | Path | None = None,
    old_support_path: str | Path | None = None,
    output_path: str | Path = "reports/policy_bc_v4_showcase.html",
) -> Path:
    show = _load(showcase_path)
    if not show:
        raise FileNotFoundError(
            f"{showcase_path} not found — run `cr-replays showcase-policy` first"
        )
    new_report = _load(Path(model_dir) / "report.json")
    old_report = _load(Path(old_model_dir) / "report.json")
    new_slice = _load(slice_path)
    old_slice = _load(
        old_slice_path or str(slice_path).replace("_v4", "_v3")
    )
    new_support = _load(support_path)
    old_support = _load(
        old_support_path or str(support_path).replace("_v4", "_v3")
    )

    overall = show["overall"]
    metrics = _metric_rows(overall, new_report, old_report, new_slice, old_slice)
    cells = _cell_pairs(new_support, old_support)
    compute = show.get("compute", {})
    new_meta = show.get("new_model", {})
    old_meta = show.get("old_model", {})
    fix_counts = show.get("fix_counts", {})
    history = new_report.get("history", [])
    old_history = old_report.get("history", [])
    lessons = new_report.get("lessons", [])

    zone_delta = overall["new_zone_acc"] - overall["old_zone_acc"]
    slot_delta = overall["new_slot_acc"] - overall["old_slot_acc"]

    lesson_items = "".join(
        f"<li>{html.escape(str(item))}</li>" for item in lessons
    )

    payload = {
        "show": show,
        "metrics": metrics,
        "cells": cells,
        "history": history,
        "oldHistory": old_history,
    }

    body = f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>PolicyBC v4 — what actually got better</title>
  <style>{_base_styles()}{_showcase_styles()}</style>
</head>
<body>
  <main>
    <header class="report-header hero">
      <div class="badge-row">
        <span class="badge">policy-bc-v{html.escape(str(new_meta.get("version", "4")))}</span>
        <span class="badge">vs v{html.escape(str(old_meta.get("version", "3")))}</span>
        <span class="badge">card-conditioned placement</span>
        <span class="badge">{compute.get("actions_scored", 0):,} held-out actions rescored</span>
        <span class="badge">{html.escape(str(compute.get("device", "cpu")))}</span>
      </div>
      <h1>What actually got better in v4</h1>
      <p class="hero-sub">v3 knew <em>when</em> to defend. v4 also knows <em>where</em> to put the card,
      because the placement heads finally get to see which card is being played.
      Everything below is measured offline on held-out battles — click, scrub and play with it.</p>
      <p class="meta">Showcase built {html.escape(show.get("created_at", ""))} ·
      {compute.get("battles_scored", 0):,} battles · scored in {compute.get("seconds", 0)}s</p>
      <div class="hero-scores">
        <div class="hero-score">
          <span class="hero-score-label">Zones fixed by v4</span>
          <span class="hero-score-value up" data-count="{fix_counts.get("fixed", 0)}">0</span>
        </div>
        <div class="hero-score">
          <span class="hero-score-label">Zones broken by v4</span>
          <span class="hero-score-value down" data-count="{fix_counts.get("regressed", 0)}">0</span>
        </div>
        <div class="hero-score">
          <span class="hero-score-label">Net placements gained</span>
          <span class="hero-score-value up" data-count="{fix_counts.get("net", 0)}">0</span>
        </div>
      </div>
    </header>

    <section class="report-section">
      <h2>The one-wire change</h2>
      <p class="caption">v3 asked the placement heads to guess a location from the game state alone,
      before anything decided <em>which</em> card was going down. v4 feeds the chosen card's
      embedding into those heads (teacher-forced 70% / model's own soft pick 30% during training),
      so "where" finally knows "what". Hover the wire.</p>
      <div class="wire-diagram">
        <div class="wire-col">
          <div class="wire-title">v3</div>
          <div class="wire-box gru">GRU state</div>
          <div class="wire-arrows">
            <span class="wire-arrow"></span><span class="wire-arrow"></span>
          </div>
          <div class="wire-heads">
            <div class="wire-box head">card slot</div>
            <div class="wire-box head">zone + xy</div>
          </div>
          <p class="wire-note">placement is card-blind</p>
        </div>
        <div class="wire-col">
          <div class="wire-title">v4</div>
          <div class="wire-box gru">GRU state</div>
          <div class="wire-arrows">
            <span class="wire-arrow"></span><span class="wire-arrow"></span>
          </div>
          <div class="wire-heads">
            <div class="wire-box head" id="wireSlot">card slot</div>
            <div class="wire-box head live" id="wirePlace">zone + xy</div>
          </div>
          <svg class="wire-link" viewBox="0 0 240 60" preserveAspectRatio="none">
            <path d="M46,10 C46,48 194,48 194,12" fill="none" stroke="#34d399" stroke-width="2.5"
                  stroke-dasharray="6 5" class="wire-flow"/>
          </svg>
          <p class="wire-note up">card embedding → placement</p>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>The tale of the tape</h2>
      <p class="caption">Hit <em>Run the diff</em> and watch each bar travel from v3 to v4.
      The first five rows are the honest comparison: both frozen checkpoints replayed over the
      <b>same {overall.get("n", 0):,} held-out actions</b> for this report. The last rows are copied
      from each model's own training run — the replay dataset kept growing between them, so treat
      those as indicative, not head-to-head.</p>
      <div class="race-toolbar">
        <button type="button" class="play-btn" id="raceBtn">▶ Run the diff</button>
        <span class="race-hint" id="raceHint">bars are parked at v3</span>
      </div>
      <div class="race" id="race"></div>
    </section>

    <section class="report-section">
      <h2>Arena lab</h2>
      <p class="caption">Where do cards actually land? Pick a layer and a card. The grid is the
      model's 12-zone placement space drawn to scale on the board — your side is at the bottom,
      the river is the thin band in the middle.</p>
      <div class="lab">
        <div class="lab-controls">
          <div class="control-group">
            <span class="control-label">Layer</span>
            <div class="chip-row" id="layerRow">
              <button type="button" class="chip active" data-layer="human">Humans</button>
              <button type="button" class="chip" data-layer="new">v4</button>
              <button type="button" class="chip" data-layer="old">v3</button>
              <button type="button" class="chip" data-layer="dnew">v4 − humans</button>
              <button type="button" class="chip" data-layer="dver">v4 − v3</button>
            </div>
          </div>
          <div class="control-group">
            <span class="control-label">Card</span>
            <div class="chip-row scroll-row" id="cardRow"></div>
          </div>
          <label class="switch">
            <input type="checkbox" id="scatterToggle">
            <span>show individual plays (white = human, amber = v4, line = error)</span>
          </label>
          <div class="lab-readout" id="labReadout"></div>
        </div>
        <div class="lab-arena">
          <svg id="arena" viewBox="0 0 300 440" class="arena"></svg>
          <div class="arena-tip" id="arenaTip" hidden></div>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>Placement league table</h2>
      <p class="caption">Zone accuracy per card, v3 → v4. Sorted by who gained the most from
      telling the placement head which card it is holding.</p>
      <div class="league" id="league"></div>
    </section>

    <section class="report-section">
      <h2>Caught in the act</h2>
      <p class="caption">Real held-out plays where v3 put the card in the wrong zone and v4 got it
      right. White star = what the human did, red = v3, green = v4.</p>
      <div class="gallery" id="gallery"></div>
    </section>

    <section class="report-section quiz-section">
      <h2>Can you beat the bots?</h2>
      <p class="caption">Real reaction windows from held-out battles: the opponent just committed a
      win condition, these four cards were in hand. Pick what you would play, then see what the
      human actually did — and what v3 and v4 said.</p>
      <div class="quiz" id="quiz"></div>
      <div class="quiz-score" id="quizScore"></div>
    </section>

    <section class="report-section">
      <h2>Defense cells: v3 → v4</h2>
      <p class="caption">Probability the model plays the textbook answer <em>when that answer is
      naturally in hand</em>. Grey tick = how often humans themselves take the answer, which is the
      real ceiling here — humans are not textbooks.</p>
      <div class="legend-row">
        <span><i class="swatch" style="background:#f87171"></i>v3</span>
        <span><i class="swatch" style="background:#34d399"></i>v4</span>
        <span><i class="swatch tick"></i>human use rate</span>
      </div>
      <div class="dumbbells" id="dumbbells"></div>
    </section>

    <section class="report-section block-grid">
      <div class="block">
        <h2>Under pressure vs. free time</h2>
        <p class="caption">Reaction = acting within 5s of an opponent win condition.</p>
        <div id="splitBars"></div>
      </div>
      <div class="block">
        <h2>Placement spread</h2>
        <p class="caption">Entropy of the zone distribution (bits). A model that mashes one zone
        looks confident and plays badly; v4 is closer to the human spread.</p>
        <div id="entropyBars"></div>
      </div>
    </section>

    <section class="report-section">
      <h2>Training, side by side</h2>
      <p class="caption">Validation curves for both runs. Scrub the epoch slider to replay training.</p>
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
      <ul class="lessons">
        {lesson_items}
        <li>Card-conditioned placement moved zone accuracy {_fmt_pct(overall["old_zone_acc"])}
        → {_fmt_pct(overall["new_zone_acc"])} ({zone_delta * 100:+.1f}pp) on the same rescored
        held-out actions, while slot accuracy also drifted {slot_delta * 100:+.1f}pp — the two heads
        did not fight each other.</li>
        <li>Net {fix_counts.get("net", 0)} placements flipped to the human's zone
        ({fix_counts.get("fixed", 0)} fixed vs {fix_counts.get("regressed", 0)} broken), so the gain
        is broad rather than one card carrying it.</li>
      </ul>
    </section>

    <footer class="report-footer">
      <p class="meta">Offline evaluation only. No live games were played to produce this report.
      Full training report: <a href="{html.escape(Path(output_path).name.replace("_showcase", ""))}">
      {html.escape(Path(output_path).name.replace("_showcase", ""))}</a></p>
    </footer>
  </main>
  <script>
    const DATA = {_json_script(payload)};
    {_showcase_script()}
  </script>
</body>
</html>
"""

    out = Path(output_path)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(body, encoding="utf-8")
    return out


def _showcase_styles() -> str:
    return """
    .hero h1 { font-size: 2.6rem; letter-spacing: -0.02em; }
    .hero-sub { font-size: 1.05rem; max-width: 70ch; color: #cbd5f5; }
    .hero-scores { display: flex; flex-wrap: wrap; gap: 18px; margin-top: 18px; }
    .hero-score {
      display: flex; flex-direction: column; gap: 4px; padding: 12px 18px;
      border-radius: 14px; background: rgba(148,163,184,0.08);
      border: 1px solid rgba(148,163,184,0.18);
    }
    .hero-score-label { font-size: 0.72rem; text-transform: uppercase; letter-spacing: 0.08em; color: #94a3b8; }
    .hero-score-value { font-size: 1.9rem; font-weight: 700; font-variant-numeric: tabular-nums; }
    .hero-score-value.up { color: #34d399; }
    .hero-score-value.down { color: #f87171; }

    .play-btn {
      background: linear-gradient(120deg, #6366f1, #22d3ee);
      color: #06111f; border: 0; border-radius: 999px; padding: 9px 20px;
      font-weight: 700; cursor: pointer; font-size: 0.9rem;
    }
    .play-btn:hover { filter: brightness(1.12); }
    .race-toolbar { display: flex; align-items: center; gap: 14px; margin-bottom: 16px; }
    .race-hint { color: #94a3b8; font-size: 0.85rem; }

    .race { display: grid; gap: 12px; }
    .race-row {
      display: grid; grid-template-columns: 220px 1fr 132px; gap: 14px; align-items: center;
    }
    .race-name { font-size: 0.9rem; }
    .race-name small { display: block; color: #94a3b8; font-size: 0.74rem; line-height: 1.3; }
    .race-track {
      position: relative; height: 26px; border-radius: 8px;
      background: rgba(148,163,184,0.1); overflow: hidden;
    }
    .race-fill {
      position: absolute; inset: 0 auto 0 0; width: 0%;
      background: linear-gradient(90deg, #6366f1, #22d3ee);
      transition: width 1.1s cubic-bezier(.22,1,.36,1);
    }
    .race-ghost {
      position: absolute; top: 0; bottom: 0; width: 2px; background: #f8fafc88;
    }
    .race-ghost::after {
      content: "v3"; position: absolute; top: -1px; left: 4px;
      font-size: 0.62rem; color: #cbd5f5;
    }
    .wire-diagram { display: grid; grid-template-columns: 1fr 1fr; gap: 28px; max-width: 760px; }
    .wire-col {
      position: relative; padding: 18px; border-radius: 16px;
      background: rgba(148,163,184,0.06); border: 1px solid rgba(148,163,184,0.16);
    }
    .wire-title { font-size: 0.72rem; letter-spacing: 0.12em; color: #94a3b8; margin-bottom: 12px; }
    .wire-box {
      border-radius: 10px; padding: 10px 14px; text-align: center; font-size: 0.85rem;
      background: rgba(99,102,241,0.18); border: 1px solid rgba(148,163,184,0.25);
    }
    .wire-box.gru { background: rgba(34,211,238,0.14); }
    .wire-box.head { background: rgba(148,163,184,0.1); }
    .wire-box.live { border-color: #34d399; box-shadow: 0 0 0 1px #34d39944; }
    .wire-arrows { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; height: 26px; }
    .wire-arrow { display: block; width: 2px; height: 100%; margin: 0 auto; background: linear-gradient(#22d3ee, transparent); }
    .wire-heads { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
    .wire-link { position: absolute; left: 18px; right: 18px; bottom: 34px; height: 60px; width: calc(100% - 36px); }
    .wire-flow { animation: wireDash 1.6s linear infinite; }
    @keyframes wireDash { to { stroke-dashoffset: -22; } }
    .wire-note { font-size: 0.76rem; color: #94a3b8; margin: 46px 0 0; text-align: center; }
    .wire-note.up { color: #6ee7b7; }

    .race-fill-soft { background: linear-gradient(90deg, #64748b, #94a3b8); }
    .fair-tag {
      display: inline-block; font-size: 0.6rem; font-weight: 700; letter-spacing: 0.05em;
      text-transform: uppercase; padding: 2px 7px; border-radius: 999px; margin-left: 6px;
      vertical-align: 2px;
    }
    .fair-tag.fair { background: rgba(52,211,153,0.18); color: #6ee7b7; }
    .fair-tag.unfair { background: rgba(148,163,184,0.16); color: #cbd5f5; }
    .race-val { text-align: right; font-variant-numeric: tabular-nums; font-size: 0.9rem; }
    .race-val b { display: block; }
    .race-delta { font-size: 0.76rem; }
    .up { color: #34d399; } .down { color: #f87171; } .flat { color: #94a3b8; }

    .lab { display: grid; grid-template-columns: minmax(280px, 1fr) 360px; gap: 26px; align-items: start; }
    .control-group { margin-bottom: 16px; }
    .control-label {
      display: block; font-size: 0.72rem; text-transform: uppercase;
      letter-spacing: 0.08em; color: #94a3b8; margin-bottom: 8px;
    }
    .chip-row { display: flex; flex-wrap: wrap; gap: 8px; }
    .scroll-row { max-height: 176px; overflow-y: auto; padding-right: 4px; }
    .chip {
      background: rgba(148,163,184,0.1); border: 1px solid rgba(148,163,184,0.2);
      color: #e2e8f0; border-radius: 999px; padding: 6px 13px; font-size: 0.82rem; cursor: pointer;
    }
    .chip:hover { border-color: #22d3ee88; }
    .chip.active { background: linear-gradient(120deg, #6366f1, #22d3ee); color: #06111f; font-weight: 700; border-color: transparent; }
    .chip .chip-n { opacity: 0.65; font-size: 0.72rem; margin-left: 5px; }
    .switch { display: flex; gap: 9px; align-items: flex-start; font-size: 0.84rem; color: #cbd5f5; cursor: pointer; }
    .lab-readout { margin-top: 18px; display: grid; gap: 10px; }
    .readout-row { display: grid; grid-template-columns: 92px 1fr 60px; gap: 10px; align-items: center; font-size: 0.83rem; }
    .readout-track { height: 10px; border-radius: 6px; background: rgba(148,163,184,0.12); overflow: hidden; }
    .readout-fill { height: 100%; border-radius: 6px; transition: width .6s ease; }
    .lab-arena { position: relative; }
    .arena { width: 100%; height: auto; display: block; }
    .zone-cell { cursor: crosshair; transition: opacity .25s ease; }
    .arena-tip {
      position: absolute; pointer-events: none; padding: 7px 10px; border-radius: 9px;
      background: #0b1220ee; border: 1px solid rgba(148,163,184,0.3); font-size: 0.78rem;
      transform: translate(-50%, -125%); white-space: nowrap; z-index: 5;
    }

    .league { display: grid; gap: 9px; }
    .league-row { display: grid; grid-template-columns: 168px 1fr 118px; gap: 12px; align-items: center; font-size: 0.86rem; }
    .league-bar { position: relative; height: 22px; background: rgba(148,163,184,0.1); border-radius: 7px; }
    .league-seg { position: absolute; top: 3px; height: 16px; border-radius: 5px; }
    .league-old { background: rgba(248,113,113,0.55); }
    .league-new { background: linear-gradient(90deg,#34d399,#22d3ee); }
    .league-tag { font-size: 0.7rem; color: #94a3b8; margin-left: 6px; }

    .gallery { display: grid; grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); gap: 14px; }
    .fix-card {
      background: rgba(148,163,184,0.06); border: 1px solid rgba(148,163,184,0.16);
      border-radius: 14px; padding: 12px; transition: transform .2s ease, border-color .2s ease;
    }
    .fix-card:hover { transform: translateY(-4px); border-color: #34d39966; }
    .fix-card h4 { margin: 0 0 2px; font-size: 0.85rem; }
    .fix-card .meta { font-size: 0.7rem; margin: 0 0 8px; }
    .mini-arena { width: 100%; height: auto; border-radius: 8px; }

    .quiz-section { background: rgba(99,102,241,0.06); border-radius: 20px; }
    .quiz { display: grid; gap: 16px; }
    .quiz-card {
      border: 1px solid rgba(148,163,184,0.2); border-radius: 18px; padding: 20px;
      background: rgba(11,18,32,0.6);
    }
    .quiz-threat { font-size: 1.15rem; margin: 0 0 4px; }
    .quiz-threat b { color: #f87171; }
    .quiz-meta { color: #94a3b8; font-size: 0.82rem; margin: 0 0 16px; }
    .hand { display: flex; flex-wrap: wrap; gap: 12px; }
    .hand-card {
      position: relative; min-width: 118px; padding: 14px 12px 12px; border-radius: 14px;
      background: linear-gradient(160deg, rgba(99,102,241,0.25), rgba(34,211,238,0.12));
      border: 1px solid rgba(148,163,184,0.25); cursor: pointer; text-align: left;
      color: #e2e8f0; transition: transform .15s ease, box-shadow .15s ease;
    }
    .hand-card:hover:not(:disabled) { transform: translateY(-5px); box-shadow: 0 10px 24px rgba(34,211,238,0.16); }
    .hand-card:disabled { cursor: default; opacity: 0.92; }
    .hand-card .cost {
      position: absolute; top: -9px; right: -8px; width: 26px; height: 26px; border-radius: 50%;
      background: #a855f7; color: #0b1220; font-weight: 800; font-size: 0.8rem;
      display: grid; place-items: center; border: 2px solid #0b1220;
    }
    .hand-card .name { font-size: 0.88rem; font-weight: 600; }
    .hand-card .tags { display: flex; gap: 4px; flex-wrap: wrap; margin-top: 8px; min-height: 18px; }
    .tag { font-size: 0.62rem; padding: 2px 7px; border-radius: 999px; font-weight: 700; letter-spacing: 0.03em; }
    .tag-human { background: #f8fafc; color: #0b1220; }
    .tag-v4 { background: #34d399; color: #06281c; }
    .tag-v3 { background: #f87171; color: #2a0b0b; }
    .tag-you { background: #22d3ee; color: #06111f; }
    .hand-card.correct { border-color: #34d399; box-shadow: 0 0 0 2px #34d39955; }
    .hand-card.wrong { border-color: #f87171; }
    .quiz-stage { display: grid; grid-template-columns: 168px 1fr; gap: 22px; align-items: start; }
    .quiz-board .mini-arena { width: 100%; }
    .quiz-board .meta { font-size: 0.68rem; margin: 8px 0 0; line-height: 1.35; }
    .quiz-verdict { margin-top: 14px; font-size: 0.9rem; min-height: 22px; }
    .quiz-nav { display: flex; align-items: center; gap: 14px; margin-top: 16px; }
    .dots { display: flex; gap: 6px; }
    .dot { width: 9px; height: 9px; border-radius: 50%; background: rgba(148,163,184,0.3); }
    .dot.done { background: #22d3ee; }
    .dot.current { background: #f8fafc; }
    .quiz-score { margin-top: 18px; display: flex; gap: 20px; flex-wrap: wrap; font-size: 0.9rem; }
    .score-pill { padding: 9px 16px; border-radius: 12px; background: rgba(148,163,184,0.1); }
    .score-pill b { font-size: 1.15rem; }

    .legend-row { display: flex; gap: 18px; font-size: 0.78rem; color: #cbd5f5; margin-bottom: 12px; }
    .legend-row .swatch {
      display: inline-block; width: 12px; height: 12px; border-radius: 50%;
      margin-right: 6px; vertical-align: -2px;
    }
    .legend-row .swatch.tick { width: 3px; height: 14px; border-radius: 1px; background: #cbd5f5aa; }
    .dumbbells { display: grid; gap: 10px; }
    .dumb-row { display: grid; grid-template-columns: 190px 1fr 96px; gap: 12px; align-items: center; font-size: 0.85rem; }
    .dumb-track { position: relative; height: 26px; }
    .dumb-line { position: absolute; top: 12px; height: 3px; border-radius: 2px; }
    .dumb-dot { position: absolute; top: 7px; width: 13px; height: 13px; border-radius: 50%; transform: translateX(-50%); }
    .dumb-old { background: #f87171; }
    .dumb-new { background: #34d399; }
    .dumb-human { position: absolute; top: 3px; width: 2px; height: 20px; background: #cbd5f5aa; }
    .dumb-axis { position: absolute; inset: 12px 0 auto 0; height: 1px; background: rgba(148,163,184,0.15); }

    .split-row { display: grid; grid-template-columns: 120px 1fr 108px; gap: 10px; align-items: center; font-size: 0.84rem; margin-bottom: 9px; }
    .split-track { height: 18px; border-radius: 6px; background: rgba(148,163,184,0.1); position: relative; }
    .split-old, .split-new { position: absolute; left: 0; height: 8px; border-radius: 4px; }
    .split-old { top: 1px; background: rgba(248,113,113,0.6); }
    .split-new { top: 9px; background: linear-gradient(90deg,#34d399,#22d3ee); }

    @media (max-width: 900px) {
      .lab { grid-template-columns: 1fr; }
      .race-row { grid-template-columns: 1fr; }
      .league-row, .dumb-row { grid-template-columns: 1fr; }
    }
    """


def _showcase_script() -> str:
    return r"""
const SVGNS = "http://www.w3.org/2000/svg";
const el = (tag, attrs = {}) => {
  const node = document.createElementNS(SVGNS, tag);
  for (const [k, v] of Object.entries(attrs)) node.setAttribute(k, v);
  return node;
};
const pct = (v, d = 1) => (v === null || v === undefined ? "—" : (100 * v).toFixed(d) + "%");
const titleCase = (s) => s.replace(/-/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());

/* zone geometry: x bins 0-.4-.6-1, y bins 0-.25-.45-.55-1 (own side at ny=0) */
const XB = [0, 0.4, 0.6, 1];
const YB = [0, 0.25, 0.45, 0.55, 1];
const ZONE_NAMES = DATA.show.zone_names;

/* ---------------- animated hero counters ---------------- */
document.querySelectorAll("[data-count]").forEach((node) => {
  const target = Number(node.dataset.count);
  const start = performance.now();
  const dur = 1200;
  const step = (now) => {
    const t = Math.min(1, (now - start) / dur);
    const eased = 1 - Math.pow(1 - t, 3);
    node.textContent = Math.round(target * eased).toLocaleString();
    if (t < 1) requestAnimationFrame(step);
  };
  requestAnimationFrame(step);
});

/* ---------------- tale of the tape ---------------- */
const raceHost = document.getElementById("race");
const fmtMetric = (row, v) => {
  if (row.fmt === "pct") return pct(v);
  if (row.fmt === "int") return Math.round(v).toLocaleString();
  if (row.fmt === "sec") return v.toFixed(2) + "s";
  return v.toFixed(3);
};
DATA.metrics.forEach((row) => {
  /* bars always encode "goodness", so lower-is-better metrics still grow right */
  const span = Math.max(row.old, row.new) * 1.18;
  const frac = (v) =>
    row.higher
      ? Math.max(2, Math.min(100, (v / span) * 100))
      : Math.max(2, Math.min(100, (1 - v / span) * 100));
  const better = row.higher ? row.new > row.old : row.new < row.old;
  const delta = row.new - row.old;
  const deltaTxt = row.fmt === "pct"
    ? (delta * 100).toFixed(1) + "pp"
    : (row.fmt === "int" ? Math.round(delta).toLocaleString() : delta.toFixed(2));
  const wrap = document.createElement("div");
  wrap.className = "race-row";
  wrap.innerHTML =
    `<div class="race-name">${row.label}
       <span class="fair-tag ${row.fair ? "fair" : "unfair"}">${row.fair ? "same samples" : "archived run"}</span>
       <small>${row.note}</small></div>
     <div class="race-track">
       <div class="race-fill${row.fair ? "" : " race-fill-soft"}" data-old="${frac(row.old)}" data-new="${frac(row.new)}"></div>
       <div class="race-ghost" style="left:${frac(row.old)}%"></div>
     </div>
     <div class="race-val"><b>${fmtMetric(row, row.new)}</b>
       <span class="race-delta ${better ? "up" : (delta === 0 ? "flat" : "down")}">
         ${delta > 0 ? "+" : ""}${deltaTxt} vs v3</span></div>`;
  raceHost.appendChild(wrap);
});
const fills = [...document.querySelectorAll(".race-fill")];
const park = () => fills.forEach((f) => (f.style.width = f.dataset.old + "%"));
const run = () => fills.forEach((f, i) => setTimeout(() => (f.style.width = f.dataset.new + "%"), i * 110));
park();
let raced = false;
const raceBtn = document.getElementById("raceBtn");
const raceHint = document.getElementById("raceHint");
raceBtn.addEventListener("click", () => {
  raced = !raced;
  if (raced) { run(); raceBtn.textContent = "↺ Back to v3"; raceHint.textContent = "bars moved to v4"; }
  else { park(); raceBtn.textContent = "▶ Run the diff"; raceHint.textContent = "bars are parked at v3"; }
});
const raceObs = new IntersectionObserver((entries) => {
  entries.forEach((e) => { if (e.isIntersecting && !raced) raceBtn.click(); });
}, { threshold: 0.4 });
raceObs.observe(raceHost);

/* ---------------- arena lab ---------------- */
const arena = document.getElementById("arena");
const tip = document.getElementById("arenaTip");
const W = 300, H = 470, PAD = 6, TOP = 20;
const AW = W - PAD * 2, AH = H - TOP - 22;
const xPx = (nx) => PAD + nx * AW;
const yPx = (ny) => TOP + (1 - ny) * AH;  /* own side (ny=0) at the bottom */

let layer = "human";
let cardIdx = -1;  /* -1 = all actions */
let showScatter = false;

const cards = DATA.show.cards;
const cardRow = document.getElementById("cardRow");
const mkChip = (label, n, idx) => {
  const b = document.createElement("button");
  b.type = "button";
  b.className = "chip" + (idx === cardIdx ? " active" : "");
  b.innerHTML = label + (n ? `<span class="chip-n">${n}</span>` : "");
  b.addEventListener("click", () => {
    cardIdx = idx;
    [...cardRow.children].forEach((c) => c.classList.remove("active"));
    b.classList.add("active");
    drawArena();
  });
  return b;
};
cardRow.appendChild(mkChip("All actions", DATA.show.overall.n, -1));
cards.forEach((c, i) => cardRow.appendChild(mkChip(c.label + (c.is_wincon ? " ⚔" : ""), c.n, i)));

document.querySelectorAll("#layerRow .chip").forEach((chip) => {
  chip.addEventListener("click", () => {
    document.querySelectorAll("#layerRow .chip").forEach((c) => c.classList.remove("active"));
    chip.classList.add("active");
    layer = chip.dataset.layer;
    drawArena();
  });
});
document.getElementById("scatterToggle").addEventListener("change", (e) => {
  showScatter = e.target.checked;
  drawArena();
});

const currentSet = () => (cardIdx < 0 ? DATA.show.overall : cards[cardIdx]);

function arenaBackdrop(g, w, h, ox, oy, scale) {
  const rect = (x, y, ww, hh, attrs) => g.appendChild(el("rect", Object.assign({ x, y, width: ww, height: hh }, attrs)));
  rect(ox, oy, w, h, { fill: "#0d1a2b", rx: 10 * scale });
  /* enemy half tint */
  rect(ox, oy, w, h * 0.5, { fill: "#991b1b", opacity: 0.28, rx: 10 * scale });
  rect(ox, oy + h * 0.5, w, h * 0.5, { fill: "#1d4ed8", opacity: 0.22 });
  /* river */
  rect(ox, oy + h * 0.465, w, h * 0.07, { fill: "#1d4ed8", opacity: 0.45 });
  /* bridges */
  rect(ox + w * 0.16, oy + h * 0.455, w * 0.12, h * 0.09, { fill: "#7c5c33", opacity: 0.75 });
  rect(ox + w * 0.72, oy + h * 0.455, w * 0.12, h * 0.09, { fill: "#7c5c33", opacity: 0.75 });
  towerOverlay(g, w, h, ox, oy, scale, 0.85);
}

/* towers drawn on their own so they can also sit *above* the heatmap */
function towerOverlay(g, w, h, ox, oy, scale, opacity) {
  const tw = w * 0.11;
  const tower = (cx, cy, king, friendly) => {
    const size = king ? tw * 1.35 : tw;
    g.appendChild(el("rect", {
      x: cx - size / 2, y: cy - size / 2, width: size, height: size, rx: 3 * scale,
      fill: friendly ? "#1e3a8a" : "#7f1d1d",
      stroke: friendly ? "#93c5fd" : "#fca5a5", "stroke-width": 1.2 * scale, opacity,
    }));
  };
  tower(ox + w * 0.22, oy + h * 0.19, false, false);
  tower(ox + w * 0.78, oy + h * 0.19, false, false);
  tower(ox + w * 0.5, oy + h * 0.075, true, false);
  tower(ox + w * 0.22, oy + h * 0.81, false, true);
  tower(ox + w * 0.78, oy + h * 0.81, false, true);
  tower(ox + w * 0.5, oy + h * 0.925, true, true);
}

function drawArena() {
  const set = currentSet();
  arena.innerHTML = "";
  const g = el("g");
  arena.appendChild(g);
  arenaBackdrop(g, AW, AH, PAD, TOP, 1);

  const human = set.human_grid, nv = set.new_grid, ov = set.old_grid;
  let values, diverging = false;
  if (layer === "human") values = human;
  else if (layer === "new") values = nv;
  else if (layer === "old") values = ov;
  else if (layer === "dnew") { values = nv.map((v, i) => v - human[i]); diverging = true; }
  else { values = nv.map((v, i) => v - ov[i]); diverging = true; }
  const peak = Math.max(...values.map((v) => Math.abs(v)), 1e-6);

  for (let row = 0; row < 4; row++) {
    for (let col = 0; col < 3; col++) {
      const z = row * 3 + col;
      const x0 = xPx(XB[col]), x1 = xPx(XB[col + 1]);
      const y0 = yPx(YB[row + 1]), y1 = yPx(YB[row]);
      const v = values[z];
      const mag = Math.abs(v) / peak;
      const fill = diverging ? (v >= 0 ? "#34d399" : "#f87171") : "#22d3ee";
      /* the cell tint stays neutral so the red/blue halves of the board survive */
      const cell = el("rect", {
        x: x0 + 1, y: y0 + 1, width: x1 - x0 - 2, height: y1 - y0 - 2, rx: 5,
        fill: diverging ? fill : "#e2e8f0",
        opacity: (diverging ? 0.04 + 0.16 * mag : 0.02 + 0.05 * mag).toFixed(3),
        class: "zone-cell", stroke: "rgba(226,232,240,0.22)", "stroke-width": 1,
      });
      g.appendChild(cell);
      /* magnitude as a bubble: area scales with the share of plays */
      const cx = (x0 + x1) / 2, cy = (y0 + y1) / 2;
      const rmax = Math.min(x1 - x0, y1 - y0) * 0.44;
      const r = Math.max(9, Math.sqrt(mag) * rmax);
      g.appendChild(el("circle", {
        cx, cy, r, fill, opacity: 0.62, stroke: fill, "stroke-width": 1.5,
        "pointer-events": "none",
      }));
      const label = el("text", {
        x: cx, y: cy + 4, "text-anchor": "middle",
        fill: "#04131f", "font-size": 11, "font-weight": 800,
        "pointer-events": "none",
      });
      label.textContent = diverging
        ? (v >= 0 ? "+" : "") + (v * 100).toFixed(1)
        : (v * 100).toFixed(0) + "%";
      g.appendChild(label);
      cell.addEventListener("mousemove", (ev) => {
        const box = arena.getBoundingClientRect();
        tip.hidden = false;
        tip.style.left = ev.clientX - box.left + "px";
        tip.style.top = ev.clientY - box.top + "px";
        tip.innerHTML =
          `<b>${ZONE_NAMES[z]}</b><br>humans ${pct(human[z])} · v4 ${pct(nv[z])} · v3 ${pct(ov[z])}`;
      });
      cell.addEventListener("mouseleave", () => { tip.hidden = true; });
    }
  }

  const sideLabel = (text, y, color) => {
    const t = el("text", {
      x: PAD, y, fill: color, "font-size": 11, "letter-spacing": "0.14em", "font-weight": 700,
    });
    t.textContent = text;
    g.appendChild(t);
  };
  sideLabel("ENEMY SIDE ↑", 13, "#fca5a5");
  sideLabel("YOUR SIDE ↓", H - 6, "#93c5fd");

  if (showScatter && cardIdx >= 0) {
    const pts = cards[cardIdx].scatter;
    pts.forEach((p) => {
      g.appendChild(el("line", {
        x1: xPx(p.hx), y1: yPx(p.hy), x2: xPx(p.nx), y2: yPx(p.ny),
        stroke: "#fbbf24", "stroke-width": 0.8, opacity: 0.4,
      }));
    });
    pts.forEach((p) => {
      g.appendChild(el("circle", { cx: xPx(p.hx), cy: yPx(p.hy), r: 2.6, fill: "#f8fafc", opacity: 0.85 }));
      g.appendChild(el("circle", { cx: xPx(p.nx), cy: yPx(p.ny), r: 2.2, fill: "#fbbf24", opacity: 0.85 }));
    });
  } else if (showScatter) {
    const note = el("text", { x: W / 2, y: 13, "text-anchor": "middle", fill: "#94a3b8", "font-size": 11 });
    note.textContent = "pick a single card to see individual plays";
    g.appendChild(note);
  }

  const readout = document.getElementById("labReadout");
  const rows = [
    { label: "v4 zone acc", value: set.new_zone_acc, color: "linear-gradient(90deg,#34d399,#22d3ee)" },
    { label: "v3 zone acc", value: set.old_zone_acc, color: "rgba(248,113,113,0.7)" },
  ];
  const denom = Math.max(set.new_zone_acc, set.old_zone_acc, 0.05) * 1.25;
  readout.innerHTML =
    `<div style="font-size:.85rem;color:#cbd5f5">
       ${cardIdx < 0 ? "All held-out card plays" : titleCase(cards[cardIdx].card)} ·
       n = ${set.n.toLocaleString()}</div>` +
    rows.map((r) =>
      `<div class="readout-row"><span>${r.label}</span>
        <span class="readout-track"><span class="readout-fill" style="width:${(r.value / denom) * 100}%;background:${r.color}"></span></span>
        <span style="text-align:right">${pct(r.value)}</span></div>`).join("") +
    `<div style="font-size:.78rem;color:#94a3b8">
       ${cardIdx < 0
        ? "Chance is 8.3% (1 of 12 zones); the plain most-common-zone rule gets " +
          pct(Math.max(...set.human_grid)) + "."
        : "Most common human zone for this card: " + ZONE_NAMES[set.human_grid.indexOf(Math.max(...set.human_grid))] +
          " (" + pct(Math.max(...set.human_grid)) + ")."}
     </div>`;
}
drawArena();

/* ---------------- league table ---------------- */
const league = document.getElementById("league");
const leagueMax = Math.max(...cards.map((c) => Math.max(c.new_zone_acc, c.old_zone_acc))) * 1.1;
cards.forEach((c) => {
  const d = c.new_zone_acc - c.old_zone_acc;
  const row = document.createElement("div");
  row.className = "league-row";
  row.innerHTML =
    `<div>${c.label}<span class="league-tag">${c.n} plays · ${c.cost}⚡</span></div>
     <div class="league-bar">
       <div class="league-seg league-old" style="left:0;width:${(c.old_zone_acc / leagueMax) * 100}%"></div>
       <div class="league-seg league-new" style="left:0;width:${(c.new_zone_acc / leagueMax) * 100}%;opacity:.85"></div>
     </div>
     <div style="text-align:right">${pct(c.new_zone_acc)}
       <span class="race-delta ${d >= 0 ? "up" : "down"}">${d >= 0 ? "+" : ""}${(d * 100).toFixed(1)}pp</span></div>`;
  league.appendChild(row);
});

/* ---------------- fix gallery ---------------- */
const gallery = document.getElementById("gallery");
DATA.show.fix_gallery.forEach((fx) => {
  const card = document.createElement("div");
  card.className = "fix-card";
  const svg = document.createElementNS(SVGNS, "svg");
  svg.setAttribute("viewBox", "0 0 120 170");
  svg.setAttribute("class", "mini-arena");
  const g = el("g");
  svg.appendChild(g);
  arenaBackdrop(g, 118, 168, 1, 1, 0.5);
  const mx = (nx) => 1 + nx * 118;
  const my = (ny) => 1 + (1 - ny) * 168;
  const zoneCenter = (z) => {
    const col = z % 3, row = Math.floor(z / 3);
    return [(XB[col] + XB[col + 1]) / 2, (YB[row] + YB[row + 1]) / 2];
  };
  const [ox2, oy2] = zoneCenter(fx.old_zone);
  g.appendChild(el("circle", { cx: mx(ox2), cy: my(oy2), r: 9, fill: "none", stroke: "#f87171", "stroke-width": 2, opacity: 0.9 }));
  g.appendChild(el("circle", { cx: mx(fx.new_xy[0]), cy: my(fx.new_xy[1]), r: 5, fill: "#34d399" }));
  const star = el("path", {
    d: "M0,-7 L2,-2 L7,-2 L3,1 L5,6 L0,3 L-5,6 L-3,1 L-7,-2 L-2,-2 Z",
    transform: `translate(${mx(fx.true_xy[0])},${my(fx.true_xy[1])})`,
    fill: "#f8fafc",
  });
  g.appendChild(star);
  card.innerHTML =
    `<h4>${fx.label}</h4>
     <p class="meta">${fx.seconds}s${fx.is_reaction ? " · under pressure" : ""}</p>`;
  card.appendChild(svg);
  const cap = document.createElement("p");
  cap.className = "meta";
  cap.style.marginTop = "8px";
  cap.innerHTML = `v3 said <span class="down">${ZONE_NAMES[fx.old_zone]}</span><br>v4 said <span class="up">${ZONE_NAMES[fx.new_zone]}</span> ✓`;
  card.appendChild(cap);
  gallery.appendChild(card);
});

/* ---------------- defense quiz ---------------- */
const quizHost = document.getElementById("quiz");
const scoreHost = document.getElementById("quizScore");
const scenarios = DATA.show.scenarios;
let qi = 0;
const tally = { you: 0, v4: 0, v3: 0, done: 0 };

function renderScore() {
  if (!tally.done) {
    scoreHost.innerHTML = `<div class="score-pill">Answer a scenario to start the scoreboard.</div>`;
    return;
  }
  scoreHost.innerHTML = [
    ["You", tally.you, "#22d3ee"],
    ["v4", tally.v4, "#34d399"],
    ["v3", tally.v3, "#f87171"],
  ].map(([name, val, color]) =>
    `<div class="score-pill" style="border-left:3px solid ${color}">${name} matched the human
      <b>${val}/${tally.done}</b></div>`).join("");
}

function renderQuiz() {
  if (!scenarios.length) { quizHost.innerHTML = "<p class='caption'>No scenarios captured.</p>"; return; }
  const s = scenarios[qi];
  const wrap = document.createElement("div");
  wrap.className = "quiz-card";
  wrap.innerHTML =
    `<p class="quiz-threat">Opponent dropped <b>${s.threat_label}</b> ${s.threat_delay}s ago.</p>
     <p class="quiz-meta">Battle ${s.battle_id.slice(0, 10)} · ${s.seconds}s into the match · these
     four cards were in hand. What do you answer with?</p>`;

  const board = document.createElement("div");
  board.className = "quiz-board";
  const bsvg = document.createElementNS(SVGNS, "svg");
  bsvg.setAttribute("viewBox", "0 0 130 184");
  bsvg.setAttribute("class", "mini-arena");
  const bg = el("g");
  bsvg.appendChild(bg);
  arenaBackdrop(bg, 128, 182, 1, 1, 0.55);
  const bx = (nx) => 1 + nx * 128;
  const by = (ny) => 1 + (1 - ny) * 182;
  if (s.threat_xy) {
    bg.appendChild(el("circle", {
      cx: bx(s.threat_xy[0]), cy: by(s.threat_xy[1]), r: 7, fill: "#f8717155",
      stroke: "#f87171", "stroke-width": 2,
    }));
    const ty = by(s.threat_xy[1]);
    const lab = el("text", {
      x: bx(s.threat_xy[0]), y: ty < 22 ? ty + 18 : ty - 11, "text-anchor": "middle",
      fill: "#fca5a5", "font-size": 9,
      style: "paint-order:stroke;stroke:#0b1220;stroke-width:3px",
    });
    lab.textContent = s.threat_label;
    bg.appendChild(lab);
  }
  board.appendChild(bsvg);
  const boardNote = document.createElement("p");
  boardNote.className = "meta";
  boardNote.textContent = "incoming threat, seen from your side of the board";
  board.appendChild(boardNote);

  const hand = document.createElement("div");
  hand.className = "hand";
  const verdict = document.createElement("div");
  verdict.className = "quiz-verdict";
  const stage = document.createElement("div");
  stage.className = "quiz-stage";
  const right = document.createElement("div");
  right.appendChild(hand);
  right.appendChild(verdict);
  stage.appendChild(board);
  stage.appendChild(right);
  wrap.appendChild(stage);

  s.hand.forEach((c) => {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "hand-card";
    btn.innerHTML = `<span class="cost">${c.cost}</span><span class="name">${c.label}</span><span class="tags"></span>`;
    btn.addEventListener("click", () => reveal(c.slot), { once: true });
    hand.appendChild(btn);
  });

  const nav = document.createElement("div");
  nav.className = "quiz-nav";
  const next = document.createElement("button");
  next.type = "button";
  next.className = "play-btn";
  next.textContent = "Next scenario →";
  next.addEventListener("click", () => { qi = (qi + 1) % scenarios.length; renderQuiz(); });
  const dots = document.createElement("div");
  dots.className = "dots";
  scenarios.forEach((_, i) => {
    const d = document.createElement("span");
    d.className = "dot" + (i === qi ? " current" : (i < qi ? " done" : ""));
    dots.appendChild(d);
  });
  nav.appendChild(next);
  nav.appendChild(dots);
  wrap.appendChild(nav);

  quizHost.innerHTML = "";
  quizHost.appendChild(wrap);

  function reveal(picked) {
    tally.done += 1;
    if (picked === s.human_slot) tally.you += 1;
    if (s.new_slot === s.human_slot) tally.v4 += 1;
    if (s.old_slot === s.human_slot) tally.v3 += 1;
    [...hand.children].forEach((btn, i) => {
      const slot = s.hand[i].slot;
      btn.disabled = true;
      const tags = btn.querySelector(".tags");
      if (slot === picked) tags.innerHTML += `<span class="tag tag-you">you</span>`;
      if (slot === s.human_slot) { tags.innerHTML += `<span class="tag tag-human">human</span>`; btn.classList.add("correct"); }
      if (slot === s.new_slot) tags.innerHTML += `<span class="tag tag-v4">v4 ${(100 * s.new_slot_p).toFixed(0)}%</span>`;
      if (slot === s.old_slot) tags.innerHTML += `<span class="tag tag-v3">v3</span>`;
      if (slot === picked && slot !== s.human_slot) btn.classList.add("wrong");
    });
    if (s.human_xy) {
      bg.appendChild(el("path", {
        d: "M0,-7 L2,-2 L7,-2 L3,1 L5,6 L0,3 L-5,6 L-3,1 L-7,-2 L-2,-2 Z",
        transform: `translate(${bx(s.human_xy[0])},${by(s.human_xy[1])})`, fill: "#f8fafc",
      }));
    }
    if (s.new_xy) {
      bg.appendChild(el("circle", {
        cx: bx(s.new_xy[0]), cy: by(s.new_xy[1]), r: 4.5, fill: "#34d399",
      }));
    }
    boardNote.textContent = "white star = human answer · green = where v4 would put it";
    const you = picked === s.human_slot;
    verdict.innerHTML =
      `${you ? "<span class='up'>Match.</span>" : "<span class='down'>Different call.</span>"}
       The human played <b>${titleCase(s.human_card)}</b> in the ${ZONE_NAMES[s.human_zone]};
       v4 predicted ${ZONE_NAMES[s.new_zone]}.
       ${s.new_slot === s.human_slot ? "v4 got the card right" : "v4 missed the card"},
       ${s.old_slot === s.human_slot ? "v3 got it too" : "v3 missed it"}.`;
    renderScore();
    [...dots.children].forEach((d, i) => { if (i === qi) d.classList.add("done"); });
  }
}
renderQuiz();
renderScore();

/* ---------------- defense cell dumbbells ---------------- */
const dumb = document.getElementById("dumbbells");
const dmax = Math.max(...DATA.cells.flatMap((c) => [c.new, c.old, c.human || 0])) * 1.12 || 1;
DATA.cells.forEach((c) => {
  const d = c.new - c.old;
  const l = (v) => (v / dmax) * 100;
  const lo = Math.min(l(c.old), l(c.new)), hi = Math.max(l(c.old), l(c.new));
  const row = document.createElement("div");
  row.className = "dumb-row";
  row.innerHTML =
    `<div>${c.label}<span class="league-tag">n=${c.n}</span></div>
     <div class="dumb-track">
       <div class="dumb-axis"></div>
       <div class="dumb-line" style="left:${lo}%;width:${hi - lo}%;background:${d >= 0 ? "#34d399" : "#f87171"};opacity:.5"></div>
       <div class="dumb-dot dumb-old" style="left:${l(c.old)}%"></div>
       <div class="dumb-dot dumb-new" style="left:${l(c.new)}%"></div>
       ${c.human ? `<div class="dumb-human" style="left:${l(c.human)}%" title="human rate"></div>` : ""}
     </div>
     <div style="text-align:right">${pct(c.new)}
       <span class="race-delta ${d >= 0 ? "up" : "down"}">${d >= 0 ? "+" : ""}${(d * 100).toFixed(1)}pp</span></div>`;
  dumb.appendChild(row);
});

/* ---------------- splits + entropy ---------------- */
const splits = DATA.show.splits;
const splitHost = document.getElementById("splitBars");
const splitRows = [
  ["Reaction · card", splits.reaction.old_slot_acc, splits.reaction.new_slot_acc, splits.reaction.n],
  ["Reaction · zone", splits.reaction.old_zone_acc, splits.reaction.new_zone_acc, splits.reaction.n],
  ["Free · card", splits.non_reaction.old_slot_acc, splits.non_reaction.new_slot_acc, splits.non_reaction.n],
  ["Free · zone", splits.non_reaction.old_zone_acc, splits.non_reaction.new_zone_acc, splits.non_reaction.n],
];
const smax = Math.max(...splitRows.map((r) => Math.max(r[1], r[2]))) * 1.15;
splitHost.innerHTML = splitRows.map(([label, o, n, count]) =>
  `<div class="split-row"><span>${label}<br><small style="color:#94a3b8">n=${count.toLocaleString()}</small></span>
    <span class="split-track">
      <span class="split-old" style="width:${(o / smax) * 100}%"></span>
      <span class="split-new" style="width:${(n / smax) * 100}%"></span>
    </span>
    <span style="text-align:right">${pct(n)} <span class="race-delta ${n >= o ? "up" : "down"}">${n >= o ? "+" : ""}${((n - o) * 100).toFixed(1)}pp</span></span>
  </div>`).join("");

const ent = DATA.show.overall;
const entHost = document.getElementById("entropyBars");
const entRows = [
  ["Humans", ent.zone_entropy_human, "#f8fafc"],
  ["v4", ent.zone_entropy_new, "#34d399"],
  ["v3", ent.zone_entropy_old, "#f87171"],
];
const emax = Math.max(...entRows.map((r) => r[1])) * 1.15;
entHost.innerHTML = entRows.map(([label, v, color]) =>
  `<div class="split-row"><span>${label}</span>
    <span class="split-track"><span class="split-new" style="top:5px;height:8px;width:${(v / emax) * 100}%;background:${color}"></span></span>
    <span style="text-align:right">${v.toFixed(2)} bits</span></div>`).join("");

/* ---------------- training chart ---------------- */
const hist = DATA.history || [];
const oldHist = DATA.oldHistory || [];
const chart = document.getElementById("trainChart");
const scrub = document.getElementById("trainScrub");
const readout = document.getElementById("trainReadout");
const playBtn = document.getElementById("trainPlay");
const series = [
  { key: "val_slot_top1", label: "v4 slot top-1", color: "#34d399", hist },
  { key: "val_zone_acc", label: "v4 zone acc", color: "#22d3ee", hist },
  { key: "val_slot_top1", label: "v3 slot top-1", color: "#f87171", hist: oldHist, dash: "4 3" },
  { key: "val_zone_acc", label: "v3 zone acc", color: "#fbbf24", hist: oldHist, dash: "4 3" },
];
const CW = 720, CH = 260, ML = 46, MB = 30, MT = 12, MR = 12;
const maxEpochs = Math.max(hist.length, oldHist.length, 1);
scrub.max = maxEpochs;
scrub.value = maxEpochs;

function drawTrain(upto) {
  chart.setAttribute("viewBox", `0 0 ${CW} ${CH}`);
  chart.innerHTML = "";
  const vals = series.flatMap((s) => s.hist.map((r) => r[s.key]).filter((v) => v != null));
  if (!vals.length) return;
  const lo = Math.min(...vals) * 0.95, hi = Math.max(...vals) * 1.05;
  const px = (i) => ML + (i / Math.max(maxEpochs - 1, 1)) * (CW - ML - MR);
  const py = (v) => CH - MB - ((v - lo) / (hi - lo)) * (CH - MB - MT);
  for (let t = 0; t <= 4; t++) {
    const v = lo + (t / 4) * (hi - lo);
    chart.appendChild(el("line", { x1: ML, x2: CW - MR, y1: py(v), y2: py(v), stroke: "rgba(148,163,184,0.14)" }));
    const lab = el("text", { x: ML - 8, y: py(v) + 4, "text-anchor": "end", fill: "#94a3b8", "font-size": 11 });
    lab.textContent = (100 * v).toFixed(0) + "%";
    chart.appendChild(lab);
  }
  series.forEach((s) => {
    const pts = s.hist.slice(0, upto).map((r, i) => [px(i), py(r[s.key])]).filter((p) => !isNaN(p[1]));
    if (pts.length < 2) return;
    chart.appendChild(el("polyline", {
      points: pts.map((p) => p.join(",")).join(" "), fill: "none",
      stroke: s.color, "stroke-width": 2.2, "stroke-dasharray": s.dash || "none",
      "stroke-linejoin": "round",
    }));
    const last = pts[pts.length - 1];
    chart.appendChild(el("circle", { cx: last[0], cy: last[1], r: 3.4, fill: s.color }));
  });
  const axis = el("text", { x: CW / 2, y: CH - 6, "text-anchor": "middle", fill: "#94a3b8", "font-size": 11 });
  axis.textContent = "epoch";
  chart.appendChild(axis);
  const row = hist[Math.min(upto, hist.length) - 1];
  readout.textContent = row
    ? `epoch ${row.epoch} · v4 slot ${pct(row.val_slot_top1)} · v4 zone ${pct(row.val_zone_acc ?? 0)}`
    : `epoch ${upto}`;
}
document.getElementById("trainLegend").innerHTML = series.map((s) =>
  `<span style="display:inline-flex;align-items:center;gap:6px;margin-right:14px;font-size:.8rem">
    <span style="width:14px;height:3px;background:${s.color};display:inline-block"></span>${s.label}</span>`).join("");
drawTrain(maxEpochs);
scrub.addEventListener("input", () => drawTrain(Number(scrub.value)));
let timer = null;
playBtn.addEventListener("click", () => {
  if (timer) { clearInterval(timer); timer = null; playBtn.textContent = "▶"; return; }
  playBtn.textContent = "❚❚";
  let i = 1;
  drawTrain(i);
  timer = setInterval(() => {
    i += 1;
    scrub.value = i;
    drawTrain(i);
    if (i >= maxEpochs) { clearInterval(timer); timer = null; playBtn.textContent = "▶"; }
  }, 140);
});
"""
