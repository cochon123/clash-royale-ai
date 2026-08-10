"""Interactive HTML report for the human-vs-AI style discriminator."""

from __future__ import annotations

import json
import random
from pathlib import Path
from typing import Any

from .report_kit import badge_row, esc, hero_scores, lesson_list, load_json, page, verdict_banner
from .style_train import MAX_SEQUENCE_EVENTS, _score_style, truncate_battle
from .winner_report import _json_script, _report_timestamp

FEATURE_EXPLAINERS: dict[str, dict[str, str]] = {
    "duration_norm": {
        "title": "Battle length",
        "blurb": "How long the sequence lasts, scaled toward a ~5 minute match (seconds / 300).",
    },
    "n_events": {
        "title": "Total actions",
        "blurb": "Count of every card play and ability in the truncated sequence.",
    },
    "n_plays": {
        "title": "Card plays",
        "blurb": "Number of normal card deployments (abilities excluded).",
    },
    "n_abilities": {
        "title": "Ability activations",
        "blurb": "How many champion/ability activations appear in the sequence.",
    },
    "plays_per_min": {
        "title": "Play tempo",
        "blurb": "Card plays per minute of battle time — overall activity rate.",
    },
    "gap_mean": {
        "title": "Average delay between actions",
        "blurb": "Mean seconds between consecutive actions. Low = frantic; high = patient.",
    },
    "gap_std": {
        "title": "Delay variability",
        "blurb": "How uneven the waits between actions are. Humans usually mix short and long gaps.",
    },
    "gap_p10": {
        "title": "Shortest delays (10th percentile)",
        "blurb": "The quick end of the delay distribution — how snappy the fastest plays feel.",
    },
    "gap_p50": {
        "title": "Median delay",
        "blurb": "Typical wait between one action and the next.",
    },
    "gap_p90": {
        "title": "Longest delays (90th percentile)",
        "blurb": "The slow end — long pauses for thinking, cycling, or waiting for elixir.",
    },
    "frac_gap_lt_1": {
        "title": "Rapid-fire plays (<1s)",
        "blurb": "Fraction of consecutive actions less than 1 second apart. Humans often chain "
        "defenses or cycle plays this quickly; a policy that always waits longer looks robotic.",
    },
    "frac_gap_lt_2": {
        "title": "Fast plays (<2s)",
        "blurb": "Fraction of gaps under 2 seconds — still brisk reactions / cycle tempo.",
    },
    "frac_gap_gt_8": {
        "title": "Long pauses (>8s)",
        "blurb": "Fraction of gaps longer than 8 seconds — banked elixir, dead time, or stalls.",
    },
    "alt_rate": {
        "title": "Side alternation rate",
        "blurb": "How often consecutive actions switch team ↔ opponent. Our rollout loop forces "
        "strict flip-flops, so AI sequences sit near 1.0 while humans are messier.",
    },
    "max_streak": {
        "title": "Longest same-side streak",
        "blurb": "Max number of actions in a row from the same player before the other acts.",
    },
    "frac_single": {
        "title": "Single-elixir phase share",
        "blurb": "Fraction of plays that happen before 2× elixir (first ~2 minutes).",
    },
    "frac_double": {
        "title": "Double-elixir phase share",
        "blurb": "Fraction of plays during 2× elixir (roughly minutes 2–4).",
    },
    "frac_triple": {
        "title": "Triple-elixir phase share",
        "blurb": "Fraction of plays after overtime / 3× elixir starts (~4 minutes).",
    },
    "leak_team": {
        "title": "Team elixir leak",
        "blurb": "Estimated elixir the team wasted by sitting at 10 while more would have accrued.",
    },
    "leak_opp": {
        "title": "Opponent elixir leak",
        "blurb": "Same leak estimate for the opponent side.",
    },
    "leak_diff": {
        "title": "Elixir leak difference",
        "blurb": "Team leak minus opponent leak — who wasted more idle elixir.",
    },
    "pre_elixir_mean": {
        "title": "Elixir before each play",
        "blurb": "Average estimated elixir a player held right before spending.",
    },
    "pre_elixir_std": {
        "title": "Elixir-before-play variability",
        "blurb": "Spread of pre-play elixir — do they always dump, or play at mixed levels?",
    },
    "dump_rate": {
        "title": "Near-empty dump rate",
        "blurb": "Fraction of plays that leave the side nearly empty (≤1.5 elixir left).",
    },
    "spend_team": {
        "title": "Team total spend",
        "blurb": "Sum of card costs spent by the team over the sequence.",
    },
    "spend_opp": {
        "title": "Opponent total spend",
        "blurb": "Sum of card costs spent by the opponent.",
    },
    "spend_diff": {
        "title": "Spend difference",
        "blurb": "Team spend minus opponent spend.",
    },
    "cost_mean": {
        "title": "Average card cost",
        "blurb": "Mean elixir cost of the cards that were actually played.",
    },
    "cost_std": {
        "title": "Card-cost spread",
        "blurb": "How mixed cheap and expensive plays are in the sequence.",
    },
    "cost_delta_mean": {
        "title": "Cost swing between plays",
        "blurb": "Average absolute cost change from one play to the next (cycle rhythm).",
    },
    "cost_delta_std": {
        "title": "Cost-swing variability",
        "blurb": "Whether cost changes between plays are steady or erratic.",
    },
    "team_x_mean": {
        "title": "Team left–right center",
        "blurb": "Average horizontal placement for the team (0 = left, 1 = right).",
    },
    "team_x_std": {
        "title": "Team left–right spread",
        "blurb": "How widely the team spreads placements across the lane. Regression-to-the-mean "
        "policies often pile up near the center, so this std is low versus humans.",
    },
    "team_y_mean": {
        "title": "Team depth center",
        "blurb": "Average vertical placement for the team (backline → bridge → enemy half).",
    },
    "team_y_std": {
        "title": "Team depth spread",
        "blurb": "How much the team varies backline vs bridge vs push depth.",
    },
    "opp_x_mean": {
        "title": "Opponent left–right center",
        "blurb": "Average horizontal placement for the opponent.",
    },
    "opp_x_std": {
        "title": "Opponent left–right spread",
        "blurb": "How widely the opponent spreads left/right. Same tell as team_x_std for the other side.",
    },
    "opp_y_mean": {
        "title": "Opponent depth center",
        "blurb": "Average vertical placement for the opponent.",
    },
    "opp_y_std": {
        "title": "Opponent depth spread",
        "blurb": "How much the opponent varies placement depth.",
    },
    "team_deep_rate": {
        "title": "Team deep-push rate",
        "blurb": "Fraction of team plays placed deep into the opponent half.",
    },
    "opp_deep_rate": {
        "title": "Opponent deep-push rate",
        "blurb": "Fraction of opponent plays placed deep into the team half.",
    },
    "team_bridge_rate": {
        "title": "Team bridge rate",
        "blurb": "Fraction of team plays dropped near the river / bridge.",
    },
    "opp_bridge_rate": {
        "title": "Opponent bridge rate",
        "blurb": "Fraction of opponent plays near the river / bridge.",
    },
    "team_back_rate": {
        "title": "Team backline rate",
        "blurb": "Fraction of team plays kept safely in the back.",
    },
    "opp_back_rate": {
        "title": "Opponent backline rate",
        "blurb": "Fraction of opponent plays kept in their backline.",
    },
    "team_left_rate": {
        "title": "Team left-lane bias",
        "blurb": "Fraction of team placements on the left half of the arena.",
    },
    "opp_left_rate": {
        "title": "Opponent left-lane bias",
        "blurb": "Fraction of opponent placements on the left half.",
    },
    "tile_diversity": {
        "title": "Placement diversity",
        "blurb": "How many distinct coarse tiles get used, relative to event count.",
    },
    "team_unique_frac": {
        "title": "Team card variety",
        "blurb": "How many unique team cards appear / 8 — did they cycle the whole deck?",
    },
    "opp_unique_frac": {
        "title": "Opponent card variety",
        "blurb": "How many unique opponent cards appear / 8.",
    },
    "team_spell_rate": {
        "title": "Team spell rate",
        "blurb": "Fraction of team plays that are spells (Fireball, Zap, …).",
    },
    "opp_spell_rate": {
        "title": "Opponent spell rate",
        "blurb": "Fraction of opponent plays that are spells.",
    },
    "team_wincon_rate": {
        "title": "Team win-condition rate",
        "blurb": "Fraction of team plays that look like win conditions (Hog, Balloon, …).",
    },
    "opp_wincon_rate": {
        "title": "Opponent win-condition rate",
        "blurb": "Fraction of opponent plays that look like win conditions.",
    },
    "resp_latency_mean": {
        "title": "Mean answer latency",
        "blurb": "Average seconds to respond after the other side places a deep / threatening play.",
    },
    "resp_latency_std": {
        "title": "Answer-latency spread",
        "blurb": "How consistent those reaction times are.",
    },
    "resp_rate": {
        "title": "Threat answer rate",
        "blurb": "How often a deep push is followed by a measured response from the other side.",
    },
    "pending_deep": {
        "title": "Unanswered deep pushes",
        "blurb": "Count of deep plays still waiting for a response at sequence end.",
    },
    "team_deck_cost_mean": {
        "title": "Team deck average cost",
        "blurb": "Mean elixir cost of the 8 cards in the team deck (composition, not plays).",
    },
    "opp_deck_cost_mean": {
        "title": "Opponent deck average cost",
        "blurb": "Mean elixir cost of the opponent deck.",
    },
    "team_deck_spells": {
        "title": "Team spell count in deck",
        "blurb": "How many spells are in the team deck list.",
    },
    "opp_deck_spells": {
        "title": "Opponent spell count in deck",
        "blurb": "How many spells are in the opponent deck list.",
    },
    "team_deck_wincons": {
        "title": "Team win-cons in deck",
        "blurb": "How many win-condition-ish cards the team deck contains.",
    },
    "opp_deck_wincons": {
        "title": "Opponent win-cons in deck",
        "blurb": "How many win-condition-ish cards the opponent deck contains.",
    },
    "side_count_imbalance": {
        "title": "Action-count imbalance",
        "blurb": "How uneven the number of team vs opponent actions is in the sequence.",
    },
    "in_overtime": {
        "title": "Reached overtime?",
        "blurb": "1 if the truncated sequence lasts into overtime (≥240s), else 0.",
    },
    "in_single": {
        "title": "Still in single elixir?",
        "blurb": "1 if the sequence ends before 2× elixir (<120s), else 0.",
    },
}


def _explain_feature(name: str) -> dict[str, str]:
    info = FEATURE_EXPLAINERS.get(name)
    if info:
        return {"title": info["title"], "blurb": info["blurb"]}
    return {
        "title": name.replace("_", " "),
        "blurb": "Sequence statistic used by the human-vs-AI judge (no glossary entry yet).",
    }


def _build_spot_the_ai(
    report: dict[str, Any],
    model_dir: Path,
    input_dir: str | Path = "data/raw",
    card_costs_path: str | Path = "data/card_costs.json",
    n_rounds: int = 8,
    seed: int = 11,
) -> list[dict[str, Any]]:
    try:
        import pickle

        from .style_train import _load_rollout_cache, _rollout_cache_path
        from .winner_dataset import collect_battles, load_card_costs, split_battles
    except Exception:
        return []

    ckpt = model_dir / "style_ensemble.pkl"
    if not ckpt.exists():
        return []
    with ckpt.open("rb") as handle:
        artifact = pickle.load(handle)

    costs = load_card_costs(card_costs_path)
    battles = collect_battles(input_dir)
    _tr, _va, test = split_battles(battles, seed=42)
    rng = random.Random(seed)
    pool = [b for b in test if len(b.events) >= MAX_SEQUENCE_EVENTS]
    pool_by_id = {b.battle_id: b for b in pool}

    train_policy = report.get("data", {}).get("train_policy_id", "policy_bc_v2")
    ai_rollouts = _load_rollout_cache(_rollout_cache_path(model_dir, train_policy, "test"))
    if not ai_rollouts:
        return []

    def fingerprint(battle: Any) -> dict[str, Any]:
        sides = [e["side"] for e in battle.events if e.get("event_type") == "card_play"]
        alt = sum(1 for a, b in zip(sides, sides[1:]) if a != b)
        alt_rate = alt / max(len(sides) - 1, 1)
        delays = []
        prev = None
        for event in battle.events:
            if event.get("event_type") != "card_play":
                continue
            if prev is not None:
                delays.append(max(0.0, float(event["seconds"]) - prev))
            prev = float(event["seconds"])
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
    rng.shuffle(ai_rollouts)
    for ai in ai_rollouts:
        if len(rounds) >= n_rounds:
            break
        source_id = ai.battle_id.replace("-rollout", "")
        real = pool_by_id.get(source_id)
        if real is None or len(real.events) < MAX_SEQUENCE_EVENTS:
            continue
        human = truncate_battle(real)
        scores = _score_style(artifact, [human, ai], costs)
        human_first = rng.random() < 0.5
        left = human if human_first else ai
        right = ai if human_first else human
        left_score = scores[0] if human_first else scores[1]
        right_score = scores[1] if human_first else scores[0]
        rounds.append(
            {
                "policy_id": train_policy,
                "human_side": "left" if human_first else "right",
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


def render_style_report(
    model_dir: str | Path = "models/style_discriminator",
    output_path: str | Path | None = None,
    build_quiz: bool = True,
) -> Path:
    model_dir = Path(model_dir)
    report = load_json(model_dir / "report.json")
    if not report:
        raise FileNotFoundError(model_dir / "report.json")

    report_dir = Path(output_path).parent if output_path else Path("reports")
    report_dir.mkdir(parents=True, exist_ok=True)
    out = Path(output_path) if output_path else report_dir / "style_discriminator_v1.html"

    created = _report_timestamp(model_dir, "report.json", "style_ensemble.pkl")
    data = report["data"]
    compute = report["compute"]
    matched = report.get("matched", {}).get("eval", [])
    transfer = report.get("transfer", {}).get("eval", [])
    ranking = report.get("ranking", [])
    stages = report.get("transfer", {}).get("training_stages", [])
    lessons = report.get("lessons", [])
    best = report.get("matched", {}).get("best_policy_id", "?")

    quiz_path = report_dir / "style_spot_the_ai.json"
    quiz: list[dict[str, Any]] = []
    if build_quiz:
        if quiz_path.exists():
            quiz = load_json(quiz_path)  # type: ignore[assignment]
            if isinstance(quiz, dict):
                quiz = quiz.get("rounds", [])
        else:
            print("Building spot-the-AI rounds (one-time)…", flush=True)
            quiz = _build_spot_the_ai(report, model_dir)
            quiz_path.write_text(json.dumps({"rounds": quiz}, indent=2), encoding="utf-8")
            print(f"Wrote {quiz_path} ({len(quiz)} rounds)", flush=True)

    forensics = report.get("forensics", {})
    best_row = matched[0] if matched else {}
    worst_row = matched[-1] if matched else {}
    tells = forensics.get("features", [])[:16]
    ablation = forensics.get("ablation", [])
    truncation = forensics.get("truncation", [])
    tell_profile = forensics.get("tell_profile", [])
    trajectories = report.get("trajectories", [])
    boost_curve = report.get("transfer", {}).get("fit", {}).get("boost_curve", [])
    harness_names = set(forensics.get("harness_features", []))
    harness_free = forensics.get("harness_free", {})
    for row in tells:
        row["kind"] = "harness" if row["feature"] in harness_names else "policy"
        row.update(_explain_feature(row["feature"]))
    for row in tell_profile:
        row.update(_explain_feature(row["feature"]))
    top_tell = next((row for row in tells if row["kind"] == "policy"), tells[0] if tells else {})
    ablation_last = ablation[-1] if ablation else {}
    first_detect = next((row for row in truncation if row.get("auc", 0) >= 0.99), None)

    payload = {
        "matched": matched,
        "transfer": transfer,
        "ranking": ranking,
        "stages": stages,
        "boost": boost_curve,
        "quiz": quiz,
        "sanity": report.get("sanity_controls", {}),
        "tells": tells,
        "ablation": ablation,
        "truncation": truncation,
        "tellProfile": tell_profile,
        "trajectories": trajectories,
        "trainPolicy": data.get("train_policy_id", "?"),
        "glossary": {name: _explain_feature(name) for name in FEATURE_EXPLAINERS},
    }

    rollout = compute.get("rollout", {})
    n_ai_train = data.get("train_battles_used", 0)
    n_ai_eval = data.get("eval_battles_used", 0)
    n_policies = len(data.get("eval_policies", []))
    detect_events = first_detect.get("ai_events") if first_detect else None

    body = f"""
    <header class="report-header hero">
      {badge_row(
          esc(report.get("model_name", "style-discriminator-v1")),
          f"v{esc(report.get('model_version', '1.0.0'))}",
          "human vs AI judge",
          esc(compute.get("framework", "sklearn")),
          f"{data.get('battles_total', 0):,} battles",
          esc(compute.get("device", "cpu")),
      )}
      <h1>Can you tell human from AI?</h1>
      <p class="hero-sub">A binary judge scoring P(sequence is human) from action statistics
      alone. Its negatives are real policy rollouts, not random noise. The question it answers:
      <b>how far is each behaviour-cloning policy from passing as a person?</b></p>
      <p class="meta">Created {esc(created)} · built in {report.get("seconds", 0)}s ·
      {data.get("feature_dim", "?")} features · {n_policies} policies judged ·
      negatives from <b>{esc(data.get('train_policy_id', '?'))}</b></p>
      {hero_scores([
          ("Detection AUC", round(best_row.get("auc", 0), 3), "down"),
          ("Best fool rate", f"{100 * (best_row.get('fool_rate_at_0.5') or 0):.1f}%", "up"),
          ("Most human-like", best, "up"),
          ("AI actions to catch", detect_events if detect_events else "—", "neutral"),
      ])}
      {verdict_banner(
          "CAUGHT",
          "Every policy is separated from humans perfectly (AUC 1.000, 0% fool rate). The "
          "ranking below is therefore a margin comparison, not a contest anyone is winning yet.",
      )}
    </header>

    <section class="report-section">
      <h2>How the experiment works</h2>
      <p class="caption">Each AI sequence shares its first {rollout.get("warmup_events", 12)}
      events with a real battle, then the policy generates the next
      {rollout.get("max_new_events", 40)} actions at temperature
      {rollout.get("temperature", 0.8)}. Humans are truncated to the same length, so the only
      difference between the two classes is the generated suffix.</p>
      <div class="flow">
        <div class="flow-step"><span class="flow-num">1</span><b>Real battle</b>
          <small>held-out human replay</small></div>
        <div class="flow-arrow">→</div>
        <div class="flow-step"><span class="flow-num">2</span><b>Split at event {rollout.get("warmup_events", 12)}</b>
          <small>shared prefix</small></div>
        <div class="flow-arrow">→</div>
        <div class="flow-step two"><span class="flow-num">3</span><b>Two futures</b>
          <small>human continuation vs policy rollout</small></div>
        <div class="flow-arrow">→</div>
        <div class="flow-step"><span class="flow-num">4</span><b>67 features</b>
          <small>timing, elixir, placement, tempo</small></div>
        <div class="flow-arrow">→</div>
        <div class="flow-step"><span class="flow-num">5</span><b>P(human)</b>
          <small>HGB + ExtraTrees blend</small></div>
      </div>
      <div class="datagrid">
        <div><span>Human sequences</span><b>{n_ai_train:,} train / {n_ai_eval:,} test</b></div>
        <div><span>AI sequences</span><b>{n_ai_train:,} × {n_policies} policies</b></div>
        <div><span>Compute</span><b>{esc(compute.get("device", "cpu"))} rollouts + CPU trees</b></div>
        <div><span>Wall clock</span><b>{report.get("seconds", 0)}s</b></div>
      </div>
    </section>

    <section class="report-section">
      <h2>Policy human-likeness ranking</h2>
      <p class="caption">Matched panel: every policy faces a detector trained on its own rollouts,
      which is the fair test. Since all of them are caught, the separator is mean P(human) —
      how close the AI gets to the human side before being rejected.</p>
      <div class="rank-table" id="rankTable"></div>
      <p class="caption">Bar length is log-scaled human-likeness; all values sit far below the
      0.5 decision line, drawn in red.</p>
    </section>

    <section class="report-section">
      <h2>Where the scores land</h2>
      <p class="caption">Score distribution for humans and AI on held-out battles. Pick a policy.
      The two classes do not overlap at all — that is what AUC 1.0 looks like.</p>
      <div class="chips" id="histChips"></div>
      <div class="rel" id="scoreHost">
        <svg class="chart" id="scoreHist"></svg>
        <div class="legend-row">
          <span><i class="swatch bar" style="background:#34d399"></i>human</span>
          <span><i class="swatch bar" style="background:#f87171"></i>AI rollout</span>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>The tells</h2>
      <p class="caption">Every feature ranked by how well it separates human from AI <em>on its
      own</em>. A bar at 1.00 means that single number is enough to catch the policy. Click a row
      — the left panel explains the term; the right shows the two distributions.</p>
      <div class="callout">
        <b>Read this before trusting the top of the list.</b> Some tells are produced by the
        rollout harness rather than the policy: continuations always alternate sides and always
        run exactly {rollout.get("max_new_events", 40)} events, so
        <code>alt_rate</code>, <code>n_events</code> and their relatives are free giveaways.
        They are tagged <span class="kind-tag harness">harness</span> below. Retraining the judge
        without any of those {len(harness_names)} features still reaches
        <b>AUC {harness_free.get("auc", 0):.3f}</b> with a
        {100 * (harness_free.get("fool_rate_at_0.5") or 0):.1f}% fool rate, so the style gap is
        genuine — the harness just makes it trivial.
      </div>
      <div class="tell-layout">
        <div class="tell-left">
          <div class="tell-list" id="tellList"></div>
          <div class="tell-readout" id="tellReadout"></div>
        </div>
        <div class="tell-detail">
          <div class="rel" id="tellHost">
            <svg class="chart" id="tellChart"></svg>
          </div>
          <div class="legend-row" style="margin-top:8px">
            <span><i class="swatch bar" style="background:#34d399"></i>human</span>
            <span><i class="swatch bar" style="background:#f87171"></i>AI</span>
          </div>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>Tell profile across policies</h2>
      <p class="caption">For the top tells, how far each policy sits from the human mean, in human
      standard deviations. Green is close to human, red is far. This shows whether newer policies
      repaired any specific tell.</p>
      <div class="heatmap" id="tellHeatmap"></div>
    </section>

    <section class="report-section">
      <h2>How fast is the AI caught?</h2>
      <p class="caption">Truncate both sequences after N generated actions and re-score. This is
      how much evidence the judge needs. Press play to watch detection build up.</p>
      <div class="chart-animation">
        <svg class="chart" id="truncChart"></svg>
        <div class="legend" id="truncLegend"></div>
        <div class="anim-toolbar">
          <button type="button" class="play-btn" id="truncPlay">▶</button>
          <input type="range" class="anim-scrubber" id="truncScrub" min="1" max="1" value="1" step="1">
          <div class="anim-readout" id="truncReadout"></div>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>Could the policy hide?</h2>
      <p class="caption">Delete the strongest tells from the feature set and retrain the judge.
      If accuracy survives, the AI signature is spread across many statistics rather than one
      fixable bug.</p>
      <div class="chart-animation">
        <svg class="chart" id="ablChart"></svg>
        <div class="legend" id="ablLegend"></div>
        <div class="anim-toolbar">
          <button type="button" class="play-btn" id="ablPlay">▶</button>
          <input type="range" class="anim-scrubber" id="ablScrub" min="1" max="1" value="1" step="1">
          <div class="anim-readout" id="ablReadout"></div>
        </div>
      </div>
      <p class="caption">With the {ablation_last.get("dropped", 0)} strongest tells removed —
      leaving only {ablation_last.get("features_left", 0)} features — the judge still reaches AUC
      {ablation_last.get("auc", 0):.3f}. Removing the {len(harness_names)} harness artifacts
      instead leaves AUC {harness_free.get("auc", 0):.3f}. Both say the same thing: the AI
      signature is spread across the whole statistical profile.</p>
    </section>

    <section class="report-section">
      <h2>Replay: same prefix, two futures</h2>
      <p class="caption">The shared human warm-up is drawn in grey; then the human continuation
      and the policy rollout diverge. Press play to watch both timelines advance together.</p>
      <div class="chips" id="replayChips"></div>
      <div class="replay-grid">
        <div class="replay-pane">
          <h3>Human <span class="pane-score" id="humanScore"></span></h3>
          <svg class="arena" id="humanArena"></svg>
        </div>
        <div class="replay-pane">
          <h3>AI rollout <span class="pane-score" id="aiScore"></span></h3>
          <svg class="arena" id="aiArena"></svg>
        </div>
      </div>
      <div class="anim-toolbar">
        <button type="button" class="play-btn" id="replayPlay">▶</button>
        <input type="range" class="anim-scrubber" id="replayScrub" min="1" max="1" value="1" step="1">
        <div class="anim-readout" id="replayReadout"></div>
      </div>
      <div class="replay-log" id="replayLog"></div>
    </section>

    <section class="report-section">
      <h2>ROC curves</h2>
      <p class="caption">Matched detector per policy. All curves hug the top-left corner, so the
      judge trades nothing to catch everything.</p>
      <div class="rel" id="rocHost">
        <svg class="chart" id="rocChart"></svg>
        <div class="legend" id="rocLegend"></div>
      </div>
    </section>

    <section class="report-section">
      <h2>Transfer vs matched</h2>
      <p class="caption">Transfer keeps one detector trained on {esc(data.get('train_policy_id', 'v2'))}
      rollouts and points it at every policy. Where transfer beats matched, the newer policy has
      drifted away from the style the old detector memorised — the closest thing to progress here.</p>
      <div class="dual-bars" id="dualBars"></div>
    </section>

    <section class="report-section quiz-section">
      <h2>Spot the human</h2>
      <p class="caption">Two fingerprints from held-out battles. One is a truncated human game,
      the other is the same prefix continued by {esc(data.get('train_policy_id', 'the policy'))}.
      Pick the human, then compare with what the judge said.</p>
      <div id="quiz"></div>
      <div class="quiz-score" id="quizScore"></div>
    </section>

    <section class="report-section">
      <h2>Training curves</h2>
      <p class="caption">Left: boosting log-loss per iteration. Right: ensemble accuracy and AUC
      as ExtraTrees accumulate. Scrub or press play to replay the fit.</p>
      <div class="twin-charts">
        <div class="chart-animation">
          <svg class="chart" id="lossChart"></svg>
          <div class="legend" id="lossLegend"></div>
        </div>
        <div class="chart-animation">
          <svg class="chart" id="trainChart"></svg>
          <div class="legend" id="trainLegend"></div>
          <div class="anim-toolbar">
            <button type="button" class="play-btn" id="trainPlay">▶</button>
            <input type="range" class="anim-scrubber" id="trainScrub" min="1" max="1" value="1" step="1">
            <div class="anim-readout" id="trainReadout"></div>
          </div>
        </div>
      </div>
    </section>

    <section class="report-section">
      <h2>Sanity controls</h2>
      <p class="caption">Legal-but-random synthetics scored by the same judge. They should be
      rejected outright; if they were not, the judge would be detecting chaos instead of style.</p>
      <div class="sanity" id="sanityPanel"></div>
    </section>

    <section class="report-section">
      <h2>Lessons learned</h2>
      <ul class="lessons">{lesson_list(lessons)}</ul>
      <h3>What to fix next</h3>
      <ol class="lessons next-steps">
        <li>Fix the harness before blaming the policy: sample who acts next instead of forcing
          strict alternation, and vary continuation length so <code>n_events</code> stops being a
          constant.</li>
        <li>Then attack the strongest genuine tell, <b>{esc(top_tell.get("feature", "—"))}</b>:
          human {top_tell.get("human_mean", 0):.3f} vs AI {top_tell.get("ai_mean", 0):.3f}
          ({abs(top_tell.get("cohens_d", 0)):.1f}σ apart).</li>
        <li>Placement and timing are regressed to the conditional mean, which halves their spread
          versus humans. Sampling from the predicted distribution should close several tells at
          once.</li>
        <li>Re-run this judge after each policy change — it needs no live play and the rollout
          cache makes repeat runs cheap.</li>
      </ol>
      <p class="caption">{esc(report.get("notes", ""))}</p>
    </section>

    <footer class="report-footer">
      <p class="meta">Offline judge · checkpoint {esc(report.get("checkpoint", ""))} ·
      generated {esc(created)}</p>
    </footer>
    """

    script = f"""
const DATA = {_json_script(payload)};
{_style_script()}
"""
    html_doc = page(
        title="Style discriminator — human vs AI",
        body=body,
        script=script,
        extra_styles=_style_styles(),
        include_arena=True,
    )
    out.write_text(html_doc, encoding="utf-8")
    return out


def _style_styles() -> str:
    return """
    .flow { display: flex; align-items: stretch; gap: 8px; flex-wrap: wrap; margin: 16px 0 20px; }
    .flow-step {
      flex: 1 1 130px; padding: 12px 14px; border-radius: 12px; position: relative;
      border: 1px solid rgba(148,163,184,0.2); background: rgba(148,163,184,0.06);
    }
    .flow-step.two { border-color: rgba(56,189,248,0.4); }
    .flow-step b { display: block; font-size: 0.9rem; margin-bottom: 3px; }
    .flow-step small { color: #94a3b8; font-size: 0.75rem; }
    .flow-num {
      position: absolute; top: -9px; left: 12px; width: 18px; height: 18px; border-radius: 50%;
      background: #22d3ee; color: #04131f; font-size: 11px; font-weight: 800;
      display: grid; place-items: center;
    }
    .flow-arrow { align-self: center; color: #64748b; }
    .datagrid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; }
    .datagrid > div {
      padding: 10px 14px; border-radius: 10px; background: rgba(148,163,184,0.06);
      border: 1px solid rgba(148,163,184,0.14);
    }
    .datagrid span { display: block; font-size: 0.7rem; text-transform: uppercase;
      letter-spacing: 0.07em; color: #94a3b8; margin-bottom: 4px; }
    .datagrid b { font-size: 0.92rem; }
    .rank-table { display: grid; gap: 10px; }
    .rank-row {
      display: grid; grid-template-columns: 34px 1.1fr 1.6fr 0.9fr 0.9fr;
      gap: 12px; align-items: center; padding: 12px 14px;
      border-radius: 12px; border: 1px solid rgba(148,163,184,0.18);
      background: rgba(148,163,184,0.06); font-size: 0.88rem;
    }
    .rank-row.best { border-color: rgba(52,211,153,0.45); background: rgba(52,211,153,0.08); }
    .rank-row .metric { font-variant-numeric: tabular-nums; }
    .rank-bar { position: relative; height: 12px; border-radius: 999px;
      background: rgba(148,163,184,0.14); overflow: hidden; }
    .rank-bar i { position: absolute; left: 0; top: 0; bottom: 0; border-radius: 999px;
      background: linear-gradient(90deg,#38bdf8,#34d399); width: 0; transition: width .9s ease; }
    .rank-line { position: absolute; top: -3px; bottom: -3px; width: 2px; background: #f87171; }
    .chips { display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 14px; }
    .chip {
      padding: 6px 13px; border-radius: 999px; cursor: pointer; font-size: 0.8rem;
      border: 1px solid rgba(148,163,184,0.25); background: rgba(148,163,184,0.06); color: #cbd5f5;
    }
    .chip.active { border-color: #22d3ee; background: rgba(34,211,238,0.14); color: #e2e8f0; }
    .tell-layout {
      display: grid; grid-template-columns: 1.2fr 0.9fr; gap: 18px; align-items: stretch;
    }
    .tell-left { display: flex; flex-direction: column; gap: 12px; min-height: 560px; }
    .tell-list {
      flex: 1 1 auto; min-height: 260px; max-height: 360px; overflow-y: auto;
      display: grid; gap: 5px; padding-right: 6px; align-content: start;
    }
    .tell-readout {
      flex: 0 0 auto; font-size: 0.85rem; color: #cbd5f5; line-height: 1.6;
      padding: 14px 16px; border-radius: 12px;
      border: 1px solid rgba(148,163,184,0.22); background: rgba(15,23,42,0.72);
    }
    .tell-readout b { color: #e2e8f0; }
    .tell-detail {
      position: sticky; top: 12px; align-self: start; padding: 12px; border-radius: 12px;
      border: 1px solid rgba(148,163,184,0.16); background: rgba(148,163,184,0.04);
    }
    .tell-row {
      display: grid; grid-template-columns: 1.7fr 62px 1fr 50px; gap: 9px; align-items: center;
      padding: 8px 10px; border-radius: 9px; cursor: pointer; font-size: 0.8rem;
      border: 1px solid transparent; background: rgba(148,163,184,0.05);
    }
    .tell-name { min-width: 0; }
    .tell-name code { display: block; font-size: 0.76rem; color: #e2e8f0; }
    .tell-name small {
      display: block; color: #94a3b8; font-size: 0.68rem; margin-top: 2px;
      white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
    }
    .tell-title { font-size: 1.08rem; font-weight: 700; color: #e2e8f0; margin-bottom: 4px; }
    .tell-code { font-size: 0.78rem; color: #94a3b8; }
    .tell-blurb {
      margin: 10px 0 12px; color: #cbd5f5; font-size: 0.9rem; line-height: 1.55;
      padding: 12px 14px; border-radius: 10px; background: rgba(56,189,248,0.08);
      border: 1px solid rgba(56,189,248,0.22);
    }
    .tell-stats { font-size: 0.84rem; color: #cbd5f5; line-height: 1.65; }
    .tell-warn { margin: 10px 0 0; color: #fcd34d; font-size: 0.82rem; }
    .callout {
      padding: 14px 16px; border-radius: 12px; margin: 14px 0 18px; font-size: 0.86rem;
      line-height: 1.65; color: #cbd5f5;
      border: 1px solid rgba(251,191,36,0.35); background: rgba(251,191,36,0.07);
    }
    .callout b { color: #fcd34d; }
    .callout code { font-size: 0.8rem; }
    .kind-tag {
      font-size: 0.65rem; text-transform: uppercase; letter-spacing: 0.06em; padding: 2px 7px;
      border-radius: 999px; font-weight: 700;
    }
    .kind-tag.harness { background: rgba(251,191,36,0.2); color: #fcd34d; }
    .kind-tag.policy { background: rgba(56,189,248,0.18); color: #7dd3fc; }
    .tell-row:hover { background: rgba(148,163,184,0.11); }
    .tell-row.active { border-color: #22d3ee; background: rgba(34,211,238,0.12); }
    .tell-row .sep { font-variant-numeric: tabular-nums; text-align: right; color: #94a3b8; }
    .tell-bar { height: 7px; border-radius: 999px; background: rgba(148,163,184,0.16); overflow: hidden; }
    .tell-bar i { display: block; height: 100%; border-radius: 999px; background: #f87171; }
    .heatmap { display: grid; gap: 4px; font-size: 0.78rem; overflow-x: auto; }
    .heat-row { display: grid; align-items: center; gap: 4px; }
    .heat-label { color: #cbd5f5; padding-right: 8px; white-space: nowrap; }
    .heat-cell {
      padding: 7px 4px; border-radius: 6px; text-align: center; font-variant-numeric: tabular-nums;
      color: #04131f; font-weight: 700;
    }
    .heat-head { color: #94a3b8; font-weight: 600; text-align: center; padding: 4px; }
    .twin-charts { display: grid; grid-template-columns: 1fr 1fr; gap: 18px; }
    .replay-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 18px; margin: 12px 0; }
    .replay-pane { text-align: center; }
    .replay-pane h3 { margin: 0 0 8px; font-size: 0.95rem; }
    .pane-score { font-size: 0.78rem; color: #94a3b8; font-weight: 400; }
    .arena { width: 100%; max-width: 290px; }
    .replay-log {
      margin-top: 12px; font-size: 0.8rem; color: #94a3b8; min-height: 22px;
      font-variant-numeric: tabular-nums;
    }
    .dual-bars { display: grid; gap: 12px; }
    .dual-row {
      display: grid; grid-template-columns: 140px 1fr 1fr; gap: 10px; align-items: center;
      font-size: 0.85rem;
    }
    .bar-track { height: 10px; border-radius: 999px; background: rgba(148,163,184,0.15); overflow: hidden; }
    .bar-fill { height: 100%; border-radius: 999px; }
    .bar-fill.transfer { background: #38bdf8; }
    .bar-fill.matched { background: #34d399; }
    .quiz-section { background: rgba(99,102,241,0.06); border-radius: 20px; }
    .quiz-card {
      border: 1px solid rgba(148,163,184,0.2); border-radius: 18px; padding: 20px;
      background: rgba(11,18,32,0.6);
    }
    .quiz-pair { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin: 14px 0; }
    .battle-card {
      padding: 16px; border-radius: 14px; border: 1px solid rgba(148,163,184,0.25);
      background: rgba(148,163,184,0.06); cursor: pointer; text-align: left; color: #e2e8f0;
    }
    .battle-card:hover:not(:disabled) { border-color: #22d3ee; }
    .battle-card.correct { border-color: #34d399; }
    .battle-card.wrong { border-color: #f87171; }
    .battle-card h4 { margin: 0 0 8px; }
    .battle-card .fp { font-size: 0.8rem; color: #cbd5f5; line-height: 1.45; }
    .quiz-score { display: flex; gap: 16px; flex-wrap: wrap; margin-top: 14px; }
    .score-pill { padding: 9px 16px; border-radius: 12px; background: rgba(148,163,184,0.1); }
    .quiz-verdict { min-height: 22px; font-size: 0.88rem; margin-top: 10px; color: #cbd5f5; }
    .sanity { display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px; }
    .sanity-card {
      padding: 14px; border-radius: 12px; border: 1px solid rgba(148,163,184,0.18);
      background: rgba(148,163,184,0.06); font-size: 0.85rem;
    }
    .sanity-card h3 { margin: 0 0 8px; font-size: 0.9rem; }
    .next-steps { color: #cbd5f5; line-height: 1.7; }
    @media (max-width: 900px) {
      .rank-row, .dual-row, .quiz-pair, .sanity, .tell-layout,
      .twin-charts, .replay-grid, .datagrid { grid-template-columns: 1fr; }
      .flow-arrow { display: none; }
    }
    """


def _style_script() -> str:
    return r"""
mountCounters();

const div = (cls) => { const d = document.createElement("div"); if (cls) d.className = cls; return d; };
const sci = (v) => {
  if (v === null || v === undefined) return "—";
  if (v === 0) return "0";
  if (v >= 0.01) return v.toFixed(3);
  const exp = Math.floor(Math.log10(Math.abs(v)));
  return (v / Math.pow(10, exp)).toFixed(1) + "e" + exp;
};
const logFrac = (v) => {
  const floor = -6, ceil = Math.log10(0.5);
  const lv = Math.log10(Math.max(v, Math.pow(10, floor)));
  return clamp(((lv - floor) / (ceil - floor)) * 100, 1.5, 100);
};

/* ---------- ranking ---------- */
(function () {
  const host = document.getElementById("rankTable");
  if (!host || !DATA.ranking) return;
  DATA.ranking.forEach((row, i) => {
    const node = div("rank-row" + (i === 0 ? " best" : ""));
    node.innerHTML = `
      <div>#${i + 1}</div>
      <div><b>${row.policy_id}</b><br><small style="color:#94a3b8">fool ${pct(row["fool_rate_at_0.5"] || 0)}</small></div>
      <div class="rank-bar"><i data-w="${logFrac(row.human_likeness || 0)}"></i><span class="rank-line" style="left:100%"></span></div>
      <div class="metric">P(human) ${sci(row.human_likeness || 0)}</div>
      <div class="metric">AUC ${(row.detection_auc || 0).toFixed(3)}</div>`;
    host.appendChild(node);
  });
  const fills = [...host.querySelectorAll(".rank-bar i")];
  new IntersectionObserver((entries, obs) => {
    entries.forEach((e) => {
      if (!e.isIntersecting) return;
      fills.forEach((f, i) => setTimeout(() => (f.style.width = f.dataset.w + "%"), i * 120));
      obs.disconnect();
    });
  }, { threshold: 0.3 }).observe(host);
})();

/* ---------- score distributions ---------- */
(function () {
  const rows = DATA.matched || [];
  const chips = document.getElementById("histChips");
  const svg = document.getElementById("scoreHist");
  if (!rows.length || !chips || !svg) return;
  const tip = makeTip("scoreHost");
  let active = 0;
  function draw() {
    const row = rows[active];
    const hist = row.score_histogram || {};
    const edges = hist.edges || [];
    const human = hist.real || [];
    const ai = hist.synthetic || [];
    const W = 720, H = 250, ML = 46, MB = 34, MT = 14, MR = 12;
    svg.setAttribute("viewBox", `0 0 ${W} ${H}`);
    svg.innerHTML = "";
    const peak = Math.max(...human, ...ai, 1);
    const n = Math.max(human.length, ai.length, 1);
    const bw = (W - ML - MR) / n;
    const bars = (counts, color, offset, width, label) => counts.forEach((c, i) => {
      const h = (c / peak) * (H - MB - MT);
      const bar = el("rect", {
        x: ML + i * bw + offset, y: H - MB - h, width, height: Math.max(h, c ? 1.5 : 0),
        fill: color, opacity: 0.85, rx: 1.5,
      });
      bar.addEventListener("mousemove", (ev) => tip.show(ev,
        `<b>${label}: ${c}</b><br>P(human) ${(edges[i] || 0).toFixed(2)}–${(edges[i + 1] || 0).toFixed(2)}`));
      bar.addEventListener("mouseleave", tip.hide);
      svg.appendChild(bar);
    });
    bars(ai, "#f87171", 1, Math.max(1, bw * 0.46), "AI");
    bars(human, "#34d399", bw * 0.5, Math.max(1, bw * 0.46), "human");
    svg.appendChild(el("line", { x1: ML, x2: W - MR, y1: H - MB, y2: H - MB, stroke: "rgba(148,163,184,0.3)" }));
    const midX = ML + 0.5 * (W - ML - MR);
    svg.appendChild(el("line", { x1: midX, x2: midX, y1: MT, y2: H - MB, stroke: "#f87171",
      "stroke-dasharray": "4 4", opacity: 0.55 }));
    svg.appendChild(svgText({ x: midX + 5, y: MT + 12, fill: "#f87171", "font-size": 10 }, "decision 0.5"));
    [0, 0.25, 0.5, 0.75, 1].forEach((t) => {
      svg.appendChild(svgText({ x: ML + t * (W - ML - MR), y: H - MB + 16, "text-anchor": "middle",
        fill: "#94a3b8", "font-size": 11 }, t.toFixed(2)));
    });
    svg.appendChild(svgText({ x: 6, y: MT + 8, fill: "#94a3b8", "font-size": 11 }, "battles"));
  }
  rows.forEach((row, i) => {
    const chip = div("chip" + (i === 0 ? " active" : ""));
    chip.textContent = row.policy_id;
    chip.addEventListener("click", () => {
      active = i;
      [...chips.children].forEach((c, j) => c.classList.toggle("active", i === j));
      draw();
    });
    chips.appendChild(chip);
  });
  draw();
})();

/* ---------- the tells ---------- */
(function () {
  const tells = DATA.tells || [];
  const list = document.getElementById("tellList");
  const svg = document.getElementById("tellChart");
  const readout = document.getElementById("tellReadout");
  if (!tells.length || !list) return;
  const tip = makeTip("tellHost");
  let active = 0;
  function drawDetail() {
    const t = tells[active];
    const hist = t.histogram || {};
    const edges = hist.edges || [];
    const W = 460, H = 230, ML = 44, MB = 34, MT = 14, MR = 12;
    svg.setAttribute("viewBox", `0 0 ${W} ${H}`);
    svg.innerHTML = "";
    const peak = Math.max(...(hist.human || [0]), ...(hist.ai || [0]), 1);
    const n = Math.max((hist.human || []).length, 1);
    const bw = (W - ML - MR) / n;
    const bars = (counts, color, offset, label) => (counts || []).forEach((c, i) => {
      const h = (c / peak) * (H - MB - MT);
      const bar = el("rect", { x: ML + i * bw + offset, y: H - MB - h,
        width: Math.max(1, bw * 0.46), height: Math.max(h, c ? 1.5 : 0), fill: color, opacity: 0.85, rx: 1.5 });
      bar.addEventListener("mousemove", (ev) => tip.show(ev,
        `<b>${label}: ${c}</b><br>${(edges[i] || 0).toFixed(3)} – ${(edges[i + 1] || 0).toFixed(3)}`));
      bar.addEventListener("mouseleave", tip.hide);
      svg.appendChild(bar);
    });
    bars(hist.ai, "#f87171", 1, "AI");
    bars(hist.human, "#34d399", bw * 0.5, "human");
    svg.appendChild(el("line", { x1: ML, x2: W - MR, y1: H - MB, y2: H - MB, stroke: "rgba(148,163,184,0.3)" }));
    [0, 0.5, 1].forEach((f) => {
      const v = (edges[0] || 0) + f * ((edges[edges.length - 1] || 1) - (edges[0] || 0));
      svg.appendChild(svgText({ x: ML + f * (W - ML - MR), y: H - MB + 16, "text-anchor": "middle",
        fill: "#94a3b8", "font-size": 11 }, v.toFixed(2)));
    });
    const higher = (t.human_mean || 0) >= (t.ai_mean || 0) ? "humans" : "AI";
    const lower = higher === "humans" ? "AI" : "humans";
    const glossary = (DATA.glossary && DATA.glossary[t.feature]) || {};
    const title = t.title || glossary.title || titleCase(t.feature);
    const blurb = t.blurb || glossary.blurb || "";
    readout.innerHTML = `
      <div class="tell-title">${title} <span class="kind-tag ${t.kind}">${t.kind}</span></div>
      <code class="tell-code">${t.feature}</code>
      <p class="tell-blurb">${blurb}</p>
      <div class="tell-stats">
        separation AUC <b>${(t.separation_auc || 0).toFixed(3)}</b> ·
        effect size ${Math.abs(t.cohens_d || 0).toFixed(2)}σ<br>
        human ${t.human_mean.toFixed(3)} ± ${t.human_std.toFixed(3)} ·
        AI ${t.ai_mean.toFixed(3)} ± ${t.ai_std.toFixed(3)}<br>
        <span style="color:#94a3b8">${higher} score higher on this metric; ${lower} sit lower.</span><br>
        AUC lost if this feature is scrambled: ${(t.permutation_drop || 0).toFixed(4)}
      </div>
      ${t.kind === "harness"
        ? "<p class='tell-warn'>Harness artifact — produced by the rollout loop (forced side flips or fixed length), not by the policy’s play style.</p>"
        : ""}`;
  }
  tells.forEach((t, i) => {
    const row = div("tell-row" + (i === 0 ? " active" : ""));
    const glossary = (DATA.glossary && DATA.glossary[t.feature]) || {};
    const title = t.title || glossary.title || "";
    row.title = title + (t.blurb || glossary.blurb ? " — " + (t.blurb || glossary.blurb) : "");
    row.innerHTML = `<div class="tell-name"><code>${t.feature}</code><small>${title}</small></div>
      <span class="kind-tag ${t.kind}">${t.kind}</span>
      <div class="tell-bar"><i style="width:${clamp((t.separation_auc - 0.5) * 200, 2, 100)}%;
        background:${t.kind === "harness" ? "#fbbf24" : "#f87171"}"></i></div>
      <div class="sep">${(t.separation_auc || 0).toFixed(3)}</div>`;
    row.addEventListener("click", () => {
      active = i;
      [...list.children].forEach((c, j) => c.classList.toggle("active", i === j));
      drawDetail();
    });
    list.appendChild(row);
  });
  drawDetail();
})();

/* ---------- tell profile heatmap ---------- */
(function () {
  const profile = DATA.tellProfile || [];
  const host = document.getElementById("tellHeatmap");
  if (!profile.length || !host) return;
  const policies = Object.keys(profile[0].policies || {});
  const cols = `220px repeat(${policies.length}, minmax(72px, 1fr))`;
  const head = div("heat-row");
  head.style.gridTemplateColumns = cols;
  head.innerHTML = `<div class="heat-label"></div>` +
    policies.map((p) => `<div class="heat-head">${p.replace("policy_bc_", "")}</div>`).join("");
  host.appendChild(head);
  profile.forEach((row) => {
    const line = div("heat-row");
    line.style.gridTemplateColumns = cols;
    let html = `<div class="heat-label"><code>${row.feature}</code></div>`;
    policies.forEach((p) => {
      const z = (row.policies[p] || {}).z_vs_human || 0;
      const mag = clamp(Math.abs(z) / 4, 0, 1);
      const color = Math.abs(z) < 1 ? "52,211,153" : (Math.abs(z) < 2.5 ? "251,191,36" : "248,113,113");
      html += `<div class="heat-cell" style="background:rgba(${color},${(0.18 + 0.75 * mag).toFixed(2)})"
        title="${row.feature} — ${p}: ${z.toFixed(2)}σ from human">${z >= 0 ? "+" : ""}${z.toFixed(1)}σ</div>`;
    });
    line.innerHTML = html;
    host.appendChild(line);
  });
})();

/* ---------- detection vs sequence length ---------- */
(function () {
  const rows = DATA.truncation || [];
  const svg = document.getElementById("truncChart");
  const scrub = document.getElementById("truncScrub");
  if (!rows.length || !svg || !scrub) return;
  scrub.max = rows.length; scrub.value = rows.length;
  document.getElementById("truncLegend").innerHTML = legendHTML([
    { label: "Detection AUC", color: "#38bdf8" },
    { label: "Mean P(human) on AI", color: "#f87171" },
  ]);
  function draw(upto) {
    const slice = rows.slice(0, upto);
    lineChart(svg, [
      { label: "AUC", color: "#38bdf8", values: slice.map((r) => r.auc) },
      { label: "P(human)", color: "#f87171", values: slice.map((r) => r.mean_P_human_ai) },
    ], { yFormat: "percent", min: 0, max: 1, xLabel: "generated AI actions" });
    const last = slice[slice.length - 1];
    document.getElementById("truncReadout").textContent = last
      ? `${last.ai_events} AI actions · AUC ${last.auc.toFixed(3)} · mean P(human) ${sci(last.mean_P_human_ai)}`
      : "";
  }
  draw(rows.length);
  scrub.addEventListener("input", () => draw(Number(scrub.value)));
  let timer = null;
  document.getElementById("truncPlay").addEventListener("click", (ev) => {
    const btn = ev.currentTarget;
    if (timer) { clearInterval(timer); timer = null; btn.textContent = "▶"; return; }
    btn.textContent = "❚❚";
    let i = 1;
    timer = setInterval(() => {
      scrub.value = i; draw(i); i += 1;
      if (i > rows.length) { clearInterval(timer); timer = null; btn.textContent = "▶"; }
    }, 260);
  });
})();

/* ---------- ablation ---------- */
(function () {
  const rows = DATA.ablation || [];
  const svg = document.getElementById("ablChart");
  const scrub = document.getElementById("ablScrub");
  if (!rows.length || !svg || !scrub) return;
  scrub.max = rows.length; scrub.value = rows.length;
  document.getElementById("ablLegend").innerHTML = legendHTML([
    { label: "AUC after removing tells", color: "#a78bfa" },
    { label: "Accuracy", color: "#34d399" },
  ]);
  function draw(upto) {
    const slice = rows.slice(0, upto);
    lineChart(svg, [
      { label: "AUC", color: "#a78bfa", values: slice.map((r) => r.auc) },
      { label: "acc", color: "#34d399", values: slice.map((r) => r.acc) },
    ], { yFormat: "percent", min: 0.5, max: 1.01, xLabel: "strongest tells removed" });
    const last = slice[slice.length - 1];
    document.getElementById("ablReadout").textContent = last
      ? `${last.dropped} tells removed · ${last.features_left} features left · AUC ${last.auc.toFixed(4)}`
      : "";
  }
  draw(rows.length);
  scrub.addEventListener("input", () => draw(Number(scrub.value)));
  let timer = null;
  document.getElementById("ablPlay").addEventListener("click", (ev) => {
    const btn = ev.currentTarget;
    if (timer) { clearInterval(timer); timer = null; btn.textContent = "▶"; return; }
    btn.textContent = "❚❚";
    let i = 1;
    timer = setInterval(() => {
      scrub.value = i; draw(i); i += 1;
      if (i > rows.length) { clearInterval(timer); timer = null; btn.textContent = "▶"; }
    }, 300);
  });
})();

/* ---------- replay viewer ---------- */
(function () {
  const pairs = DATA.trajectories || [];
  const chips = document.getElementById("replayChips");
  const scrub = document.getElementById("replayScrub");
  if (!pairs.length || !chips || !scrub) return;
  let active = 0;
  const panes = {
    human: { svg: document.getElementById("humanArena"), score: document.getElementById("humanScore") },
    ai: { svg: document.getElementById("aiArena"), score: document.getElementById("aiScore") },
  };
  function drawPane(kind, upto) {
    const pair = pairs[active];
    const track = pair[kind];
    const { g, geom } = mountArena(panes[kind].svg, { W: 290, H: 430 });
    geom.captions();
    const warm = pair.warmup_events || 12;
    track.events.slice(0, upto).forEach((e, i) => {
      const isWarm = i < warm;
      const color = isWarm ? "#94a3b8" : (e.side === "team" ? "#60a5fa" : "#fca5a5");
      const latest = i === upto - 1;
      g.appendChild(el("circle", {
        cx: geom.xPx(e.x), cy: geom.yPx(e.y), r: latest ? 8 : 5,
        fill: color, opacity: isWarm ? 0.45 : 0.85,
        stroke: latest ? "#e2e8f0" : "none", "stroke-width": latest ? 2 : 0,
      }));
    });
    panes[kind].score.textContent = `P(human) = ${sci(track.score)}`;
  }
  function draw(upto) {
    drawPane("human", upto);
    drawPane("ai", upto);
    const pair = pairs[active];
    const h = pair.human.events[upto - 1], a = pair.ai.events[upto - 1];
    const warm = pair.warmup_events || 12;
    document.getElementById("replayReadout").textContent =
      `event ${upto}/${pair.human.events.length} ${upto <= warm ? "(shared warm-up)" : "(diverged)"}`;
    document.getElementById("replayLog").innerHTML = (h && a)
      ? `human: <b>${titleCase(h.card)}</b> @ ${h.t}s (${h.side}) &nbsp;·&nbsp;
         AI: <b>${titleCase(a.card)}</b> @ ${a.t}s (${a.side})`
      : "";
  }
  function setPair(i) {
    active = i;
    [...chips.children].forEach((c, j) => c.classList.toggle("active", i === j));
    const n = Math.min(pairs[i].human.events.length, pairs[i].ai.events.length);
    scrub.max = n; scrub.value = n;
    draw(n);
  }
  pairs.forEach((pair, i) => {
    const chip = div("chip" + (i === 0 ? " active" : ""));
    chip.textContent = "battle " + pair.battle_id;
    chip.addEventListener("click", () => setPair(i));
    chips.appendChild(chip);
  });
  setPair(0);
  scrub.addEventListener("input", () => draw(Number(scrub.value)));
  let timer = null;
  document.getElementById("replayPlay").addEventListener("click", (ev) => {
    const btn = ev.currentTarget;
    if (timer) { clearInterval(timer); timer = null; btn.textContent = "▶"; return; }
    btn.textContent = "❚❚";
    let i = 1;
    const max = Number(scrub.max);
    timer = setInterval(() => {
      scrub.value = i; draw(i); i += 1;
      if (i > max) { clearInterval(timer); timer = null; btn.textContent = "▶"; }
    }, 130);
  });
})();

/* ---------- ROC ---------- */
(function () {
  const rows = (DATA.matched || []).filter((r) => (r.roc || []).length);
  const svg = document.getElementById("rocChart");
  if (!rows.length || !svg) return;
  const palette = ["#38bdf8", "#34d399", "#fbbf24", "#f472b6", "#a78bfa"];
  const W = 460, H = 340, ML = 48, MB = 40, MT = 14, MR = 14;
  svg.setAttribute("viewBox", `0 0 ${W} ${H}`);
  svg.innerHTML = "";
  const px = (v) => ML + v * (W - ML - MR);
  const py = (v) => H - MB - v * (H - MB - MT);
  for (let t = 0; t <= 4; t++) {
    svg.appendChild(el("line", { x1: ML, x2: W - MR, y1: py(t / 4), y2: py(t / 4), stroke: "rgba(148,163,184,0.14)" }));
    svg.appendChild(svgText({ x: ML - 8, y: py(t / 4) + 4, "text-anchor": "end", fill: "#94a3b8", "font-size": 11 },
      (t / 4).toFixed(2)));
  }
  svg.appendChild(el("line", { x1: px(0), y1: py(0), x2: px(1), y2: py(1),
    stroke: "#64748b", "stroke-dasharray": "5 4" }));
  rows.forEach((row, i) => {
    const pts = [[0, 0], ...row.roc.map((p) => [p.fpr, p.tpr]), [1, 1]];
    svg.appendChild(el("polyline", {
      points: pts.map(([x, y]) => `${px(x)},${py(y)}`).join(" "),
      fill: "none", stroke: palette[i % palette.length], "stroke-width": 2.4, opacity: 0.9,
    }));
  });
  svg.appendChild(svgText({ x: (W + ML) / 2, y: H - 8, "text-anchor": "middle", fill: "#94a3b8", "font-size": 11 },
    "false positive rate"));
  document.getElementById("rocLegend").innerHTML = legendHTML(
    rows.map((r, i) => ({ label: `${r.policy_id} (AUC ${(r.auc || 0).toFixed(3)})`, color: palette[i % palette.length] })));
})();

/* ---------- transfer vs matched ---------- */
(function () {
  const host = document.getElementById("dualBars");
  if (!host) return;
  const byId = (rows) => Object.fromEntries((rows || []).map((r) => [r.policy_id, r]));
  const transfer = byId(DATA.transfer);
  const matched = byId(DATA.matched);
  const ids = [...new Set((DATA.ranking || []).map((r) => r.policy_id))];
  ids.forEach((id) => {
    const t = transfer[id] || {};
    const m = matched[id] || {};
    const row = div("dual-row");
    row.innerHTML = `<div><b>${id}</b></div>`;
    [["transfer", t.human_likeness || 0], ["matched", m.human_likeness || 0]].forEach(([kind, val]) => {
      const cell = div();
      cell.innerHTML = `<div style="font-size:11px;color:#94a3b8;margin-bottom:4px">${kind} P(human) ${sci(val)}</div>
        <div class="bar-track"><div class="bar-fill ${kind}" style="width:${logFrac(val)}%"></div></div>`;
      row.appendChild(cell);
    });
    host.appendChild(row);
  });
})();

/* ---------- boosting loss ---------- */
(function () {
  const curve = DATA.boost || [];
  const svg = document.getElementById("lossChart");
  if (!curve.length || !svg) return;
  lineChart(svg, [
    { label: "train", color: "#22d3ee", values: curve.map((r) => r.train_loss) },
    { label: "val", color: "#fbbf24", values: curve.map((r) => r.val_loss), dash: "4 3" },
  ], { xLabel: "boosting iteration", yDigits: 3, width: 460, height: 260 });
  document.getElementById("lossLegend").innerHTML = legendHTML([
    { label: "train log-loss", color: "#22d3ee" },
    { label: "val log-loss", color: "#fbbf24" },
  ]);
})();

/* ---------- sanity controls ---------- */
(function () {
  const host = document.getElementById("sanityPanel");
  const sanity = DATA.sanity || {};
  if (!host) return;
  ["easy", "medium"].forEach((tier) => {
    const row = sanity[tier];
    if (!row) return;
    const card = div("sanity-card");
    card.innerHTML = `<h3>${titleCase(tier)} synthetic negatives</h3>
      <p>mean P(human): <b>${sci(row.mean_P_human_ai || 0)}</b></p>
      <p>fooled the judge: <b>${pct(row["fool_rate_at_0.5"] || 0)}</b> — expected 0%</p>
      <p style="color:#94a3b8">Random-but-legal sequences stay rejected, so the judge is reading
      style rather than raw chaos.</p>`;
    host.appendChild(card);
  });
})();

(function () {
  const stages = DATA.stages || [];
  const scrub = document.getElementById("trainScrub");
  const chart = document.getElementById("trainChart");
  if (!scrub || !chart || !stages.length) return;
  scrub.max = Math.max(stages.length, 1);
  scrub.value = stages.length;
  function drawTrain(upto) {
    const slice = stages.slice(0, upto);
    lineChart(chart, [
      { label: "AUC", color: "#38bdf8", values: slice.map((s) => s.auc) },
      { label: "Accuracy", color: "#34d399", values: slice.map((s) => s.acc) },
      { label: "Fool@0.5", color: "#fbbf24", values: slice.map((s) => s["fool_rate_at_0.5"]) },
    ], { yFormat: "percent", xLabel: "ExtraTrees stage", width: 460, height: 260 });
    const last = slice[slice.length - 1];
    document.getElementById("trainReadout").textContent = last
      ? `trees ${last.trees} · AUC ${(last.auc || 0).toFixed(3)} · fool ${pct(last["fool_rate_at_0.5"] || 0)}`
      : "";
  }
  document.getElementById("trainLegend").innerHTML = legendHTML([
    { label: "AUC", color: "#38bdf8" },
    { label: "Accuracy", color: "#34d399" },
    { label: "Fool@0.5", color: "#fbbf24" },
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
})();

(function () {
  const host = document.getElementById("quiz");
  const scoreHost = document.getElementById("quizScore");
  const rounds = DATA.quiz || [];
  if (!host || !rounds.length) {
    if (host) host.innerHTML = "<p class='caption'>Quiz cache empty — rerun report-style after training.</p>";
    return;
  }
  let idx = 0, correct = 0, answered = 0, judge = 0;
  function renderRound() {
    const round = rounds[idx];
    host.innerHTML = "";
    const card = div("quiz-card");
    card.innerHTML = `<div class="caption">Round ${idx + 1}/${rounds.length} ·
      AI continuation from ${round.policy_id}</div>`;
    const pair = div("quiz-pair");
    ["left", "right"].forEach((side) => {
      const b = round[side];
      const btn = document.createElement("button");
      btn.className = "battle-card";
      btn.type = "button";
      btn.innerHTML = `<h4>Battle ${b.battle_id}</h4>
        <div class="fp">${b.n_plays} plays · ${b.duration}s long<br>
        alternation ${pct(b.alt_rate)} · mean Δt ${b.mean_delay}s<br>
        team: ${(b.team_cards || []).map(titleCase).join(", ")}…</div>`;
      btn.addEventListener("click", () => {
        if (btn.disabled) return;
        [...pair.querySelectorAll("button")].forEach((x) => {
          x.disabled = true;
          if (x.dataset.side === round.human_side) x.classList.add("correct");
        });
        if (side === round.human_side) correct += 1;
        else btn.classList.add("wrong");
        const judgePick = round.left.score >= round.right.score ? "left" : "right";
        if (judgePick === round.human_side) judge += 1;
        answered += 1;
        verdict.innerHTML =
          `${side === round.human_side ? "<span class='up'>Correct.</span>" : "<span class='down'>That one was the AI.</span>"}
           Judge: left ${sci(round.left.score)}, right ${sci(round.right.score)} →
           picked <b>${judgePick}</b> ${judgePick === round.human_side ? "(correct)" : "(wrong)"}.`;
        scoreHost.innerHTML = [["You", correct], ["Judge", judge]]
          .map(([n, v]) => `<span class="score-pill">${n}: <b>${v}/${answered}</b></span>`).join("");
      });
      btn.dataset.side = side;
      pair.appendChild(btn);
    });
    card.appendChild(pair);
    const verdict = div("quiz-verdict");
    card.appendChild(verdict);
    const next = document.createElement("button");
    next.className = "play-btn";
    next.type = "button";
    next.textContent = "Next round →";
    next.style.marginTop = "12px";
    next.addEventListener("click", () => { idx = (idx + 1) % rounds.length; renderRound(); });
    card.appendChild(next);
    host.appendChild(card);
  }
  renderRound();
  scoreHost.innerHTML = `<span class="score-pill">Pick a side to start scoring.</span>`;
})();
"""
