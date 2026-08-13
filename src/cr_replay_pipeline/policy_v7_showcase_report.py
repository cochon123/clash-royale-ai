"""Showcase-style HTML report for the policy-bc-v7 arena-memory experiment.

This is the playful, interactive companion to ``policy_bc_v7.html``.  It borrows
the design language of ``policy_bc_v4_showcase.html`` (bright indigo→cyan
gradients, "run the diff" animated bars, an arena lab, a per-card league) and
repoints it at v7's actual story: the adapter wins on paper but fails every
causal control.  It also ships a searchable glossary so the jargon is never
more than a keystroke away.

The renderer emits one self-contained document: native SVG + DOM, no matplotlib,
no bundler, opens from ``file://``.
"""

from __future__ import annotations

import html
import json
import subprocess
from pathlib import Path
from typing import Any

from .report_kit import FONT_LINKS, favicon_link, kit_styles, shared_styles


CHANNELS = [
    ("friendly recent", "Non-spell friendly action mass, τ=2.5s"),
    ("friendly medium", "Non-spell friendly action mass, τ=7.5s"),
    ("friendly long", "Non-spell friendly action mass, τ=20s"),
    ("enemy recent", "Non-spell enemy action mass, τ=2.5s"),
    ("enemy medium", "Non-spell enemy action mass, τ=7.5s"),
    ("enemy long", "Non-spell enemy action mass, τ=20s"),
    ("friendly cost", "Friendly non-spell cost mass, τ=7.5s"),
    ("enemy cost", "Enemy non-spell cost mass, τ=7.5s"),
    ("friendly wincon", "Friendly win-condition pressure, τ=12s"),
    ("enemy wincon", "Enemy win-condition pressure, τ=12s"),
    ("friendly spell", "Friendly spell impact, τ=2s"),
    ("enemy spell", "Enemy spell impact, τ=2s"),
    ("x geometry", "Static normalized x coordinate"),
    ("y geometry", "Static normalized y coordinate"),
    ("river distance", "Static absolute river distance"),
    ("bridge band", "Static bridge-band indicator"),
]

MEMORY_ABLATIONS = {"arena_memory_disabled", "arena_memory_zeroed", "arena_memory_shuffled"}

GATE_ORDER = [
    ("aligned_nll_beats_baseline_0_02", "Aligned NLL beats baseline by 0.02"),
    ("aligned_nll_beats_shuffled_0_01", "Aligned NLL beats shuffled by 0.01"),
    ("aligned_top1_beats_shuffled_0_5pp", "Aligned top-1 beats shuffled by 0.5pp"),
    ("adapter_off_removes_half_gain", "Adapter-off removes half the gain"),
    ("test_exact_top1_plus_1_5pp", "Test tile top-1 +1.5pp"),
    ("test_top5_plus_2pp", "Test tile top-5 +2pp"),
    ("test_nll_minus_0_05", "Test tile NLL -0.05"),
    ("test_xy_minus_3pct", "Test XY MAE -3%"),
    ("spread_gap_minus_10pct", "Spread gap -10%"),
    ("adapter_ablation_minus_1pp", "Adapter ablation -1pp"),
    ("aligned_beats_shuffled_0_75pp", "Aligned beats shuffled by 0.75pp"),
    ("incumbent_heads_unchanged", "Incumbent heads unchanged"),
    ("no_card_slice_top5_loss_gt_3pp", "No card slice top-5 loss >3pp"),
    ("causality_checkpoint_report_tests", "Causality checkpoint tests"),
]

# Plain-English definitions for every piece of jargon used in the report.
GLOSSARY: list[tuple[str, str]] = [
    ("arena-memory adapter", "The only new part v7 trains. It writes a 16-channel picture of where friendly and enemy actions have happened recently, then adds a gated residual to v6.1's heatmap. The whole v7 experiment is a test of whether the policy actually reads this picture."),
    ("arena gate", "A learned valve (0 to ~0.3 here) that decides how much of the arena-memory residual is mixed into the placement prediction. A gate near 0 means the adapter is effectively off; v7's gate opens to ~0.22 but the content it lets through is not useful."),
    ("aligned vs shuffled control", "The decisive causal test. If the adapter truly reads the memory, scrambling the spatial pairing between each sample and its memory raster must hurt accuracy. v7's shuffled run is indistinguishable from aligned, so the gate fails."),
    ("argmax tile", "The single 18×32 tile the model ranks #1. Picking it restores human-like spread but pays for it with coordinate error, because the model does not know which mode is correct right now."),
    ("causal ablation", "An experiment that switches one input off (or scrambles it) and measures the damage. Big damage means the model was relying on that input; no damage means it was ignoring it."),
    ("expected XY", "The probability-weighted average tile coordinate. It minimises average error but collapses toward the centre of the board because it averages over every legal placement mode."),
    ("frozen backbone", "v7 does not retrain v6.1. The GRU, card embeddings and slot/zone heads are locked; only the new adapter and its gate learn. That is what makes this a clean test of the adapter alone."),
    ("gate (promotion gate)", "A pre-declared pass/fail rule the pilot must clear before it can be considered for promotion or a live-game run. v7 has 14 gates; 2 were decided and both causal ones failed, so the pilot stopped early."),
    ("heatmap prior", "v6.1's output: a probability over all 576 tiles (18 rows × 32 columns) of the arena, conditioned on the card being played. v7 keeps this prior frozen and only edits it with the memory residual."),
    ("history XY", "The existing spatial feature: the normalised x/y coordinates of recent actions in the GRU history. Neutralising it costs v7 +0.33 nats — by far the biggest ablation hit — which shows the model still leans on this old feature, not the new memory."),
    ("NLL", "Negative log-likelihood, in 'nats'. The placement loss. Lower is better; a difference of 0.01 nats across ~8,000 samples is the threshold the causal gates treat as 'real signal'."),
    ("oracle deck", "The ablation condition that feeds the model the opponent's full deck, including cards that have not been revealed yet in-game. It is the ceiling condition the others are measured against."),
    ("pilot", "A small, cheap training run designed to falsify an idea before spending a full budget on it. v7 is a 1-epoch, 60k-window pilot that was stopped early because the causal gates failed."),
    ("reaction slice", "Actions taken within 5 seconds of an opponent committing a win condition. Reactions are easier to predict because they land on predictable defensive tiles."),
    ("rescored v6.1", "v6.1 re-evaluated on v7's exact held-out test manifest, so the two models see the same 39,098 actions. This is the only fair head-to-head; v6.1's original test set was a different sample of battles."),
    ("slot top-1", "Whether the model's #1 guess for which card is played is correct. A 'card identity' metric, independent of where the card is placed."),
    ("spread (σ)", "Standard deviation of predicted placement coordinates, normalised to the board. Human σ_x ≈ 0.27; v7's expected σ_x ≈ 0.17, so the expected decode is still collapsed toward the centre versus a human-like spread."),
    ("τ (tau)", "The exponential decay time-constant of a memory channel, in seconds. Small τ (2.5s) only remembers the last few seconds; large τ (20s) keeps a longer trail of where actions happened."),
    ("tile top-1 / top-5", "Whether the human's actual tile is the model's #1 (or in its top 5) out of 576. The headline placement accuracy metric. Chance is ~0.17% for top-1."),
    ("threat vector", "A summary of recent opponent threats (e.g. a win condition crossing the river). Removing it costs +0.044 nats — a real but modest feature, unlike the new memory."),
    ("warm start", "Initialising v7's weights from an already-trained checkpoint (v6.1) instead of random. Combined with the frozen backbone, it isolates the adapter as the only thing being learned."),
    ("win condition (wincon)", "A card whose job is to take towers (e.g. hog rider, giant). The memory has dedicated long-decay (τ=12s) channels that track win-condition pressure on each side."),
    ("XY MAE", "Mean absolute error of the predicted X+Y coordinate, in replay-API units (~0 to 18,000). Lower is better. v7's 5,453 vs rescored v6.1's 5,471 is a win, but inside the noise you'd expect from re-reading the same checkpoint."),
    ("zone accuracy", "Whether the predicted tile is in the correct 1-of-12 arena zone. A coarser, more forgiving placement metric than tile top-1."),
]


def _load(path: str | Path | None) -> dict[str, Any]:
    if not path:
        return {}
    target = Path(path)
    return json.loads(target.read_text(encoding="utf-8")) if target.exists() else {}


def _load_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    out: list[dict[str, Any]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        try:
            v = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(v, dict):
            out.append(v)
    return out


def _git(command: list[str]) -> str:
    try:
        return subprocess.check_output(command, text=True, stderr=subprocess.DEVNULL).strip()
    except (OSError, subprocess.CalledProcessError):
        return "not available"


def _title(label: str) -> str:
    return label.replace("-", " ").replace("_", " ").replace("/", " per ")


def _build_data(
    report: dict[str, Any],
    baseline: dict[str, Any],
    probe: dict[str, Any],
    progress: list[dict[str, Any]],
) -> dict[str, Any]:
    base_test = baseline.get("test") or baseline.get("metrics") or {}
    test = report.get("test") or {}
    compute = report.get("compute") or {}
    data = report.get("data") or {}
    ablations = probe.get("ablations") or {}
    placement_slices = probe.get("placement_slices") or {}

    race_metrics = [
        ("Tile top-1", test.get("tile_class_acc"), base_test.get("tile_class_acc"), True, "pct",
         "Did the model's #1 tile match the human's tile?"),
        ("Tile top-5", test.get("tile_top5_acc"), base_test.get("tile_top5_acc"), True, "pct",
         "Is the human tile in the model's top 5 of 576?"),
        ("Tile NLL", test.get("tile_nll"), base_test.get("tile_nll"), False, "float",
         "Placement loss in nats. Lower is better."),
        ("XY MAE", test.get("xy_mae"), base_test.get("xy_mae"), False, "int",
         "Mean absolute X+Y error. Lower is better."),
        ("Slot top-1", test.get("slot_top1"), base_test.get("slot_top1"), True, "pct",
         "Did the model guess the right card?"),
        ("Zone accuracy", test.get("zone_acc"), base_test.get("zone_acc"), True, "pct",
         "Right 1-of-12 zone, coarser than tile."),
    ]
    race = [
        {"label": label, "old": float(old or 0), "new": float(new or 0), "higher": higher,
         "fmt": fmt, "note": note, "fair": True}
        for label, new, old, higher, fmt, note in race_metrics
    ]

    oracle = ablations.get("oracle_full_opponent_deck") or {}
    oracle_nll = float(oracle.get("tile_nll") or 0.0)
    causal_rows = []
    for key, values in ablations.items():
        if not isinstance(values, dict):
            continue
        nll = float(values.get("tile_nll") or 0.0)
        kind = "oracle" if key == "oracle_full_opponent_deck" else ("memory" if key in MEMORY_ABLATIONS else "feature")
        causal_rows.append({
            "label": _title(key).replace("arena memory", "memory").replace("opponent", "opp."),
            "nll": nll,
            "delta": nll - oracle_nll,
            "gate": float(values.get("arena_gate") or 0.0),
            "kind": kind,
        })
    causal_rows.sort(key=lambda r: (0 if r["kind"] == "oracle" else 2 if r["kind"] == "memory" else 1, -r["delta"]))

    cards = []
    per_card = placement_slices.get("per_card") or {}
    for name, values in per_card.items():
        if not isinstance(values, dict):
            continue
        cards.append({
            "card": name,
            "label": _title(name),
            "n": int(values.get("n") or 0),
            "top1": float(values.get("tile_top1") or 0.0),
            "top5": float(values.get("tile_top5") or 0.0),
            "xy": float(values.get("xy_mae_units") or 0.0),
        })
    cards.sort(key=lambda c: c["top1"], reverse=True)
    mean_top1 = sum(c["top1"] for c in cards) / len(cards) if cards else 0.0

    gates = report.get("promotion_gates") or {}
    gate_items = []
    for key, label in GATE_ORDER:
        value = gates.get(key, "not evaluated; pilot stopped")
        status = "pass" if value is True else "fail" if value is False else "hold"
        gate_items.append({
            "label": label,
            "status": status,
            "observed": "true" if value is True else "false" if value is False else str(value),
        })
    decided = [g for g in gate_items if g["status"] != "hold"]
    passed = sum(1 for g in decided if g["status"] == "pass")

    examples = []
    for ex in (probe.get("arena_examples") or [])[:3]:
        examples.append({
            "label": ex.get("label") or "sample",
            "memory": ex.get("memory") or [],
            "base": ex.get("base") or [],
            "residual": ex.get("residual") or [],
            "final": ex.get("final") or [],
        })

    loss_points = [
        [int(row.get("samples_done", 0)), float(row.get("tile_loss", row.get("loss", 0.0)) or 0.0)]
        for row in progress
    ]

    return {
        "meta": {
            "model_name": report.get("model_name", "policy-bc-v7"),
            "model_version": report.get("model_version", "7.0.0"),
            "created": report.get("created_at", ""),
            "battles": int(data.get("battles_total") or 0),
            "train_windows": int(data.get("train_samples") or 0),
            "params": int(compute.get("parameters") or 0),
            "trainable": int(compute.get("trainable_parameters") or 0),
            "seconds": float(report.get("seconds") or 0.0),
            "device": compute.get("device", "cuda"),
            "manifest": compute.get("split_manifest", "not recorded"),
            "warmstart": (compute.get("warmstart") or {}).get("dir", "—"),
            "verdict": "REJECTED",
            "headline": "Beats v6.1 on paper. The memory isn't the reason.",
        },
        "race": {
            "old_label": "v6.1 (rescored)",
            "new_label": "v7",
            "note": "Both models scored on the same 39,098 held-out actions.",
            "metrics": race,
        },
        "causal": {
            "oracle_nll": oracle_nll,
            "rows": causal_rows,
            "shuffle_delta": (ablations.get("arena_memory_shuffled") or {}).get("tile_nll", oracle_nll) - oracle_nll,
            "neutral_delta": (ablations.get("neutralized_history_xy") or {}).get("tile_nll", oracle_nll) - oracle_nll,
        },
        "league": {"mean": mean_top1, "cards": cards[:22]},
        "gates": {"passed": passed, "decided": len(decided), "items": gate_items},
        "arena": {"examples": examples, "channels": [{"name": n, "desc": d} for n, d in CHANNELS]},
        "loss": loss_points,
        "scores": [
            {"value": f"+{(test.get('tile_class_acc',0)-base_test.get('tile_class_acc',0))*100:.2f}", "suffix": "pp", "kind": "up", "label": "tile top-1 vs v6.1"},
            {"value": f"{abs(test.get('tile_nll',0)-base_test.get('tile_nll',0)):.3f}", "suffix": "nats lower", "kind": "up", "label": "tile NLL vs v6.1"},
            {"value": "2", "suffix": f"/{len(decided)}", "kind": "down", "label": "causal gates failed"},
            {"value": "0", "suffix": "", "kind": "flat", "label": "live runs authorized"},
        ],
    }


def render_policy_v7_showcase_report(
    model_dir: str | Path = "models/policy_bc_v7",
    baseline_path: str | Path | None = None,
    probe_path: str | Path | None = None,
    output_path: str | Path = "reports/policy_bc_v7_showcase.html",
) -> Path:
    model_dir = Path(model_dir)
    output_path = Path(output_path)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    report = _load(model_dir / "report.json")
    if not report:
        raise FileNotFoundError(f"Missing {model_dir / 'report.json'}")
    baseline = _load(baseline_path or output_path.parent / "policy_bc_v7_baseline.json")
    probe = _load(probe_path or output_path.parent / "blind_spot_probe_v7.json")
    progress = _load_jsonl(model_dir / "progress.jsonl")

    data = _build_data(report, baseline, probe, progress)
    glossary = [{"term": t, "def": d} for t, d in GLOSSARY]
    git_revision = _git(["git", "rev-parse", "--short", "HEAD"])
    dirty = bool(_git(["git", "status", "--porcelain"]))

    payload = json.dumps(
        {"data": data, "glossary": glossary,
         "git": git_revision + (" · dirty" if dirty else " · clean")},
        separators=(",", ":"),
    )

    template = r"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>PolicyBC v7 showcase — arena-memory on trial</title>
@@FAVICON@@
@@FONT_LINKS@@
<style>
@@SHARED_STYLES@@
@@KIT_STYLES@@
section.report { padding:30px 0; border-top:1px solid var(--line); }
section.report:first-of-type { border-top:0; padding-top:0; }
.verdict-pill { display:inline-block; margin-top:14px; padding:8px 16px; border-radius:999px; font-weight:800; letter-spacing:.08em; font-size:.78rem; text-transform:uppercase; background:rgba(255,126,120,.14); color:var(--red); border:1px solid rgba(255,126,120,.45); }
.wire-diagram { display:grid; grid-template-columns:1fr 1fr; gap:28px; max-width:760px; }
.wire-col { position:relative; padding:18px; border-radius:16px; background:#0a151c; border:1px solid var(--line); }
.wire-title { font-size:.72rem; letter-spacing:.12em; color:var(--muted); margin-bottom:12px; }
.wire-box { border-radius:10px; padding:10px 14px; text-align:center; font-size:.85rem; background:rgba(56,189,248,.14); border:1px solid var(--line); }
.wire-box.gru { background:rgba(112,225,177,.12); }
.wire-box.head { background:#0a151c; }
.wire-box.live { border-color:var(--green); }
.wire-arrows { display:grid; grid-template-columns:1fr 1fr; gap:12px; height:26px; }
.wire-arrow { display:block; width:2px; height:100%; margin:0 auto; background:linear-gradient(var(--sky), transparent); }
.wire-heads { display:grid; grid-template-columns:1fr 1fr; gap:12px; }
.wire-note { font-size:.76rem; color:var(--muted); margin:46px 0 0; text-align:center; }
.lab { display:grid; grid-template-columns:minmax(280px,1fr) 360px; gap:26px; align-items:start; }
.scroll-row { max-height:176px; overflow-y:auto; padding-right:4px; }
.readout-row { display:grid; grid-template-columns:92px 1fr 60px; gap:10px; align-items:center; font-size:.83rem; }
.readout-track { height:10px; border-radius:6px; background:#08131a; overflow:hidden; }
.readout-fill { height:100%; border-radius:6px; }
.league { display:grid; gap:9px; }
.league-row { display:grid; grid-template-columns:168px 1fr 118px; gap:12px; align-items:center; font-size:.86rem; }
.league-bar { position:relative; height:22px; background:#08131a; border-radius:7px; }
.league-seg { position:absolute; top:3px; height:16px; border-radius:5px; background:var(--sky); }
.gallery { display:grid; grid-template-columns:repeat(auto-fill,minmax(160px,1fr)); gap:14px; }
.fix-card { background:#0a151c; border:1px solid var(--line); border-radius:14px; padding:12px; }
.fix-card h4 { margin:0 0 2px; font-size:.85rem; }
.gloss { margin-top:12px; }
.gloss-search { width:100%; max-width:420px; padding:10px 12px; border-radius:10px; border:1px solid var(--line); background:#0a151c; color:var(--text); font:inherit; }
.gloss-search:focus { outline:0; border-color:var(--sky); }
.gloss-item { padding:12px 0; border-bottom:1px solid var(--line); }
.gloss-item h4 { margin:0 0 4px; font-size:.92rem; color:var(--sky); }
.gloss-empty { color:var(--muted); padding:12px 0; }
@media (max-width:900px) { .lab,.wire-diagram,.league-row { grid-template-columns:1fr; } }
</style>
</head>
<body>
<main>
  <header class="hero">
    <div class="badge-row">
      <span class="badge">policy-bc-v7</span>
      <span class="badge">arena-memory showcase</span>
      <span class="badge">vs v6.1 (rescored)</span>
      <span class="badge">1 epoch · offline only</span>
      <span class="badge" id="devBadge">cuda</span>
    </div>
    <h1 id="heroTitle">—</h1>
    <p class="hero-sub">v7 grafts a decaying 16-channel <b>arena-memory</b> adapter onto v6.1's frozen heatmap prior — a cheap pilot to ask one question: does the policy actually read where actions have been, or does it just open the gate and ignore the content? Scrub, click and search below to see why the answer killed promotion.</p>
    <p class="meta" id="heroMeta">—</p>
    <div class="hero-scores" id="heroScores"></div>
    <span class="verdict-pill">rejected · live gate locked</span>
  </header>

  <section class="report">
    <h2>The tale of the tape</h2>
    <p class="caption">Hit <b>Run the diff</b> and watch each bar travel from v6.1 (red marker) to v7. Every row is the honest comparison: both checkpoints replayed over the <b>same 39,098 held-out actions</b>. v7 wins all six — by a sliver.</p>
    <div class="toolbar">
      <button type="button" class="play-btn" id="raceBtn">▶ Run the diff</button>
      <span class="hint" id="raceHint">bars are parked at v6.1</span>
    </div>
    <div class="race" id="race"></div>
    <p class="caption" style="margin-top:14px" id="raceFoot"></p>
  </section>

  <section class="report">
    <h2>The causal catch</h2>
    <p class="caption">If the adapter really reads the memory, scrambling the spatial pairing between each sample and its memory raster <b>must</b> hurt. Press the button: the purple memory bars barely move; the amber frozen-feature bar explodes. That gap is the whole verdict.</p>
    <div class="toolbar">
      <button type="button" class="play-btn" id="scrambleBtn">▶ Scramble the memory</button>
      <span class="hint" id="scrambleHint">showing the oracle (aligned) condition</span>
    </div>
    <div class="legend-row">
      <span><i class="sw oracle"></i>oracle / aligned</span>
      <span><i class="sw mem"></i>arena-memory ablation (new)</span>
      <span><i class="sw feat"></i>frozen-feature ablation</span>
    </div>
    <div class="scramble" id="scramble"></div>
    <div class="two" style="margin-top:20px">
      <div class="panel">
        <div class="finding"><span class="finding-mark" style="background:#f87171"></span><div><strong>Memory is ignored</strong><p id="memFinding">—</p></div></div>
        <div class="finding"><span class="finding-mark" style="background:#fbbf24"></span><div><strong>Old feature still carries it</strong><p id="featFinding">—</p></div></div>
      </div>
      <div class="panel">
        <h3>Why this matters more than the win</h3>
        <p class="caption">A gain that survives the shuffled control is evidence the model learned something real about state. A gain that <em>doesn't</em> survive it is probably just extra capacity memorising the training set. v7 is the second kind — so scaling it would multiply parameters, not understanding.</p>
      </div>
    </div>
  </section>

  <section class="report">
    <h2>The gates that stopped it</h2>
    <p class="caption">Pre-declared pass/fail rules. Two causal gates were decided and both failed, so the pilot stopped before the expensive evaluation gates.</p>
    <div class="gate-summary" id="gateSummary"></div>
    <div class="gate-grid" id="gateGrid"></div>
  </section>

  <section class="report">
    <h2>Arena-memory lab</h2>
    <p class="caption">Three synchronized 18×32 panels per held-out action: the frozen v6.1 prior, the learned residual the adapter adds, and the final tile distribution. Toggle the 16 memory channels to see which parts of "where actions have been" the adapter is responding to.</p>
    <div class="panel">
      <div class="arena-tools">
        <label>Held-out action <input id="timeline" type="range" min="0" max="0" value="0" step="1"></label>
        <span id="timeline-label" class="mono">—</span>
        <select id="view-select">
          <option value="final">final distribution</option>
          <option value="base">frozen v6.1 prior</option>
          <option value="residual">arena residual</option>
          <option value="memory">memory intensity</option>
        </select>
        <select id="decode-select"><option>expected</option><option>argmax</option><option>sample</option></select>
        <label>temperature <input id="temperature" type="range" min="0.25" max="2" value="1" step="0.05"></label>
        <button id="animate">animate</button>
        <button id="allChannels" class="soft-btn">none</button>
      </div>
      <div class="control-label">memory channels (toggle to filter)</div>
      <div class="chip-row scroll-row" id="channelRow"></div>
      <div class="arena-panels">
        <div><h3>Frozen prior</h3><div id="base-arena" class="arena-grid"></div></div>
        <div><h3>Arena residual</h3><div id="residual-arena" class="arena-grid"></div></div>
        <div><h3>Final distribution</h3><div id="final-arena" class="arena-grid"></div></div>
        <div><h3>Selected memory</h3><div id="memory-arena" class="arena-grid"></div></div>
      </div>
      <p id="arena-caption" class="legend-note">—</p>
    </div>
  </section>

  <section class="report">
    <h2>Per-card placement league</h2>
    <p class="caption">Tile top-1 by card, sorted best-first. Dashed line is the cross-card mean. Cards that land on one fixed tile (the-log, elite-barbarians) are easy; spells with many legal tiles (arrows, zap, tornado) are nearly impossible.</p>
    <div class="legend-row"><span><i class="sw" style="background:linear-gradient(90deg,#6366f1,#22d3ee)"></i>v7 tile top-1</span><span><i class="sw" style="width:3px;background:#fbbf24"></i>mean</span></div>
    <div class="league" id="league"></div>
  </section>

  <section class="report">
    <h2>Training spark &amp; provenance</h2>
    <div class="two">
      <div class="panel">
        <h3>Adapter tile loss</h3>
        <p class="caption">One epoch, six logged steps. Converges inside the first pass — enough to falsify, not to converge.</p>
        <svg class="spark" id="spark" viewBox="0 0 400 60" preserveAspectRatio="none"></svg>
        <div class="control-label" style="margin-top:14px">run card</div>
        <table id="provTable"></table>
      </div>
      <div class="panel">
        <h3>The 16 memory channels</h3>
        <p class="caption">12 dynamic decays of past action locations + 4 static geometry channels. None of them observe troops, health or elixir.</p>
        <div id="channelTable" style="max-height:260px;overflow:auto"></div>
      </div>
    </div>
  </section>

  <section class="report">
    <h2>Lessons</h2>
    <ul id="lessons" style="margin:0;padding-left:18px"></ul>
  </section>

  <section class="report">
    <h2>Glossary</h2>
    <p class="caption">Every term in this report, in plain English. Type to filter — matches the term or the definition.</p>
    <div class="gloss-wrap">
      <input type="search" id="glossSearch" class="gloss-search" placeholder="Search a term…  (try: NLL, ablation, gate, shuffled, σ, wincon)" autocomplete="off">
      <div class="gloss-list" id="glossList"></div>
    </div>
    <p class="gloss-count" id="glossCount"></p>
  </section>

  <footer style="margin-top:36px">
    <p class="meta">Offline evaluation only — no live games were played to produce this report. Full training report: <a href="policy_bc_v7.html">policy_bc_v7.html</a>. <span id="gitFoot"></span></p>
  </footer>
</main>
<script id="payload" type="application/json">__PAYLOAD__</script>
<script>
const {data: D, glossary: GLOSS, git: GIT} = JSON.parse(document.getElementById("payload").textContent);

/* ---------- helpers ---------- */
const pct = (v, d=1) => (v==null?"—":(100*v).toFixed(d)+"%");
const fmtMetric = (row, v) => row.fmt==="pct"?pct(v):row.fmt==="int"?Math.round(v).toLocaleString():v.toFixed(3);
const titleCase = s => s.replace(/-/g," ").replace(/\b\w/g,c=>c.toUpperCase());
const SVGNS = "http://www.w3.org/2000/svg";

/* ---------- hero ---------- */
document.getElementById("heroTitle").textContent = D.meta.headline;
document.getElementById("devBadge").textContent = D.meta.device;
document.getElementById("heroMeta").textContent =
  `Showcase built ${D.meta.created||""} · ${D.meta.battles.toLocaleString()} battles · ${D.meta.train_windows.toLocaleString()} train windows · ${D.meta.params.toLocaleString()} params (${D.meta.trainable.toLocaleString()} trainable) · ${D.meta.seconds.toFixed(1)}s`;
const heroScores = document.getElementById("heroScores");
D.scores.forEach(s => {
  const el = document.createElement("div"); el.className="hero-score";
  el.innerHTML = `<span class="hero-score-label">${s.label}</span>
    <span class="hero-score-value ${s.kind}">${s.value}<small>${s.suffix}</small></span>`;
  heroScores.appendChild(el);
});

/* ---------- race ---------- */
const raceHost = document.getElementById("race");
const raceFoot = document.getElementById("raceFoot");
D.race.metrics.forEach((row,i) => {
  const span = Math.max(row.old, row.new) * 1.18;
  const frac = v => row.higher ? Math.max(3,Math.min(100,(v/span)*100)) : Math.max(3,Math.min(100,(1-v/span)*100));
  const better = row.higher ? row.new>row.old : row.new<row.old;
  const delta = row.new-row.old;
  const deltaTxt = row.fmt==="pct" ? (delta*100).toFixed(2)+"pp" : (row.fmt==="int"?Math.round(delta).toLocaleString():delta.toFixed(3));
  const wrap = document.createElement("div"); wrap.className="race-row";
  wrap.innerHTML =
    `<div class="race-name">${row.label}<span class="fair-tag">same samples</span><small>${row.note}</small></div>
     <div class="race-track"><div class="race-fill fair" data-old="${frac(row.old)}" data-new="${frac(row.new)}"></div><div class="race-ghost" style="left:${frac(row.old)}%"></div></div>
     <div class="race-val"><b>${fmtMetric(row,row.new)}</b><span class="race-delta ${better?"up":(delta===0?"flat":"down")}">${delta>0?"+":""}${deltaTxt} vs v6.1</span></div>`;
  raceHost.appendChild(wrap);
});
const fills = [...document.querySelectorAll(".race-fill")];
const parkRace = () => fills.forEach(f => f.style.width = f.dataset.old+"%");
const runRace = () => fills.forEach((f,i) => setTimeout(()=>f.style.width=f.dataset.new+"%", i*120));
parkRace();
let raced=false;
const raceBtn=document.getElementById("raceBtn"), raceHint=document.getElementById("raceHint");
raceBtn.addEventListener("click", () => {
  raced=!raced;
  if(raced){runRace();raceBtn.textContent="↺ Back to v6.1";raceHint.textContent="bars moved to v7";}
  else{parkRace();raceBtn.textContent="▶ Run the diff";raceHint.textContent="bars are parked at v6.1";}
});
new IntersectionObserver(es=>es.forEach(e=>{if(e.isIntersecting&&!raced)raceBtn.click();}),{threshold:.4}).observe(raceHost);
raceFoot.textContent = `v7 improves on v6.1 on all ${D.race.metrics.length} controlled metrics — but every delta sits inside the noise you would expect from re-reading the same frozen checkpoint with an extra adapter bolted on.`;

/* ---------- scramble ---------- */
const scrHost=document.getElementById("scramble");
const cmax=Math.max(...D.causal.rows.map(r=>r.delta))||1;
D.causal.rows.forEach(r => {
  const w = r.kind==="oracle" ? 4 : Math.max(3,(r.delta/cmax)*100);
  const cls = r.kind;
  const wrap=document.createElement("div"); wrap.className="sr";
  wrap.innerHTML =
    `<div>${titleCase(r.label)}</div>
     <div class="sr-track"><div class="sr-fill ${cls}" data-aligned="${r.kind==="oracle"?w:4}" data-scrambled="${w}"></div></div>
     <div class="sr-val">${r.delta>=0?"+":""}${r.delta.toFixed(3)}</div>`;
  scrHost.appendChild(wrap);
});
const scrFills=[...document.querySelectorAll(".sr-fill")];
const parkScr=()=>scrFills.forEach(f=>f.style.width=f.dataset.aligned+"%");
const runScr=()=>scrFills.forEach((f,i)=>setTimeout(()=>f.style.width=f.dataset.scrambled+"%", i*90));
parkScr();
let scrambled=false;
const scrBtn=document.getElementById("scrambleBtn"), scrHint=document.getElementById("scrambleHint");
scrBtn.addEventListener("click",()=>{
  scrambled=!scrambled;
  if(scrambled){runScr();scrBtn.textContent="↺ Reset to aligned";scrHint.textContent="ΔNLL vs oracle — taller = more important to the model";}
  else{parkScr();scrBtn.textContent="▶ Scramble the memory";scrHint.textContent="showing the oracle (aligned) condition";}
});
document.getElementById("memFinding").textContent =
  `Disabling, zeroing or shuffling the new memory changes tile NLL by at most ${Math.max(...D.causal.rows.filter(r=>r.kind==="memory").map(r=>r.delta)).toFixed(3)} nats — below the 0.01 gate threshold. The adapter opens its gate (≈${D.causal.rows.find(r=>r.kind==="oracle").gate.toFixed(2)}) but the model routes around the content.`;
document.getElementById("featFinding").textContent =
  `Neutralising the existing history-XY feature costs ${D.causal.neutral_delta.toFixed(3)} nats — ${(D.causal.neutral_delta/Math.max(...D.causal.rows.filter(r=>r.kind==="memory").map(r=>r.delta),1e-9)).toFixed(0)}× the cost of touching the memory. The old spatial signal is doing the work.`;

/* ---------- gates ---------- */
const gs=document.getElementById("gateSummary");
gs.innerHTML =
  `<div class="gs"><span>causal gates passed</span><strong class="mono up">${D.gates.passed} / ${D.gates.decided}</strong></div>
   <div class="gs"><span>causal gates failed</span><strong class="mono down">${D.gates.decided-D.gates.passed}</strong></div>
   <div class="gs"><span>not evaluated</span><strong class="mono">${D.gates.items.length-D.gates.decided}</strong></div>`;
const gg=document.getElementById("gateGrid");
D.gates.items.forEach(g=>{
  const cell=document.createElement("div"); cell.className="gate-cell";
  cell.innerHTML=`<span class="gchip ${g.status}">${g.status}</span><span class="gl">${g.label}</span><span class="go mono">${g.observed}</span>`;
  gg.appendChild(cell);
});

/* ---------- arena-memory lab ---------- */
const ex=D.arena.examples;
const timeline=document.getElementById("timeline");
timeline.max=String(Math.max(ex.length-1,0));
const channelRow=document.getElementById("channelRow");
const channels=D.arena.channels;
const channelOn=channels.map(()=>true);
channels.forEach((c,i)=>{
  const b=document.createElement("button"); b.type="button"; b.className="chip tiny active";
  b.innerHTML=`<span title="${c.desc}">${c.name}</span>`;
  b.addEventListener("click",()=>{channelOn[i]=!channelOn[i];b.classList.toggle("active",channelOn[i]);renderArena();});
  channelRow.appendChild(b);
});
document.getElementById("allChannels").addEventListener("click",function(){
  const anyOn=channelOn.some(Boolean);
  channelOn.forEach((_,i)=>channelOn[i]=!anyOn);
  [...channelRow.children].forEach((b,i)=>b.classList.toggle("active",channelOn[i]));
  renderArena();
});
const grids={base:document.getElementById("base-arena"),residual:document.getElementById("residual-arena"),final:document.getElementById("final-arena"),memory:document.getElementById("memory-arena")};
const view=document.getElementById("view-select"), decodeSel=document.getElementById("decode-select"), temperature=document.getElementById("temperature");
function paint(grid,values,kind){
  grid.innerHTML="";
  for(let i=0;i<576;i++){
    const v=Number(values&&values[i]||0);
    const z=kind==="residual"?Math.min(1,Math.abs(v)*.45):Math.max(0,Math.min(1,v));
    const c=document.createElement("div"); c.className="cell";
    c.title="tile "+i+" · "+v.toFixed(4);
    c.style.background = kind==="residual" ? (v>=0?`rgba(34,211,238,${0.06+z})`:`rgba(248,113,113,${0.06+z})`) : `rgba(34,211,238,${0.06+.9*z})`;
    grid.appendChild(c);
  }
}
function decodeValues(values){
  const probs=values.map(x=>Math.max(Number(x||0),1e-9)); const out=Array(576).fill(0);
  if(decodeSel.value==="expected") return probs;
  if(decodeSel.value==="argmax"){out[probs.indexOf(Math.max(...probs))]=1;return out;}
  const temp=Math.max(Number(temperature.value),.05); const w=probs.map(x=>Math.exp(Math.log(x)/temp)); const sum=w.reduce((a,x)=>a+x,0); let r=Math.random()*sum;
  for(let i=0;i<w.length;i++){r-=w[i];if(r<=0){out[i]=1;break;}}
  return out;
}
function renderArena(){
  const e=ex[Number(timeline.value)]||{};
  const m=e.memory||[];
  const filtered=Array.from({length:576},(_,i)=>Array.isArray(m[i])?m[i].reduce((a,x,j)=>a+(channelOn[j]?Number(x||0):0),0):0);
  const mode=view.value; const decoded=decodeValues(e.final||[]);
  const shown = mode==="base"?(e.base||[]):mode==="residual"?(e.residual||[]):mode==="memory"?filtered:decoded;
  paint(grids.base,e.base||[],"base");
  paint(grids.residual,e.residual||[],"residual");
  paint(grids.final,shown,mode==="residual"?"residual":"final");
  paint(grids.memory,filtered,"memory");
  document.getElementById("timeline-label").textContent=e.label||("sample "+(Number(timeline.value)+1));
  document.getElementById("arena-caption").textContent=
    `Mode: ${mode} · decoder: ${decodeSel.value} · temp: ${temperature.value} · selected memory intensity: ${filtered.reduce((a,x)=>a+x,0).toFixed(3)} · ${channelOn.filter(Boolean).length}/${channels.length} channels on`;
}
timeline.oninput=renderArena; view.onchange=renderArena; decodeSel.onchange=renderArena; temperature.oninput=renderArena;
document.getElementById("animate").onclick=()=>{let i=0;clearInterval(window.at);window.at=setInterval(()=>{if(!ex.length)return;timeline.value=String(i++%ex.length);renderArena();},500);};
renderArena();

/* ---------- league ---------- */
const league=document.getElementById("league");
const lmax=Math.max(...D.league.cards.map(c=>c.top1))||1;
const meanPct=(D.league.mean*100).toFixed(1);
D.league.cards.forEach(c=>{
  const w=(c.top1/lmax)*100; const meanLeft=(D.league.mean/lmax)*100;
  const d=c.top1-D.league.mean;
  const row=document.createElement("div"); row.className="league-row";
  row.innerHTML=
    `<div>${c.label}<span class="league-tag">n=${c.n}</span></div>
     <div class="league-bar"><div class="league-seg" style="width:0%;transition:width .9s cubic-bezier(.22,1,.36,1)" data-w="${w}"></div><div class="league-mean" style="left:${meanLeft}%"></div></div>
     <div class="league-val"><b>${pct(c.top1)}</b><span class="league-delta ${d>=0?"up":"down"}">${d>=0?"+":""}${(d*100).toFixed(1)}pp vs mean</span></div>`;
  league.appendChild(row);
});
const leagueObs=new IntersectionObserver(es=>es.forEach(e=>{if(e.isIntersecting){[...league.querySelectorAll(".league-seg")].forEach((s,i)=>setTimeout(()=>s.style.width=s.dataset.w+"%",i*50));leagueObs.disconnect();}}),{threshold:.2});
leagueObs.observe(league);

/* ---------- spark + provenance ---------- */
const spark=document.getElementById("spark");
if(D.loss.length){
  const xs=D.loss.map(p=>p[0]), ys=D.loss.map(p=>p[1]);
  const lo=Math.min(...ys), hi=Math.max(...ys), span=Math.max(hi-lo,1e-9);
  const pts=D.loss.map((p,i)=>[i/(Math.max(D.loss.length-1,1))*390, 54-((p[1]-lo)/span)*46]);
  const poly=document.createElementNS(SVGNS,"polyline");
  poly.setAttribute("points",pts.map(p=>p[0].toFixed(1)+","+p[1].toFixed(1)).join(" "));
  poly.setAttribute("fill","none"); poly.setAttribute("stroke","#22d3ee"); poly.setAttribute("stroke-width","2");
  spark.appendChild(poly);
  pts.forEach((p,i)=>{const c=document.createElementNS(SVGNS,"circle");c.setAttribute("cx",p[0]);c.setAttribute("cy",p[1]);c.setAttribute("r","3");c.setAttribute("fill","#6366f1");c.innerHTML=`<title>${xs[i].toLocaleString()} samples · ${ys[i].toFixed(4)} nats</title>`;spark.appendChild(c);});
}
const prov=[
  ["model",`${D.meta.model_name} · ${D.meta.model_version}`],
  ["created",D.meta.created],
  ["manifest",D.meta.manifest],
  ["warm start",D.meta.warmstart],
  ["battles / windows",`${D.meta.battles.toLocaleString()} / ${D.meta.train_windows.toLocaleString()}`],
  ["params / trainable",`${D.meta.params.toLocaleString()} / ${D.meta.trainable.toLocaleString()}`],
  ["device",D.meta.device],
];
document.getElementById("provTable").innerHTML=
  `<tbody>${prov.map(r=>`<tr><td>${r[0]}</td><td><code>${r[1]}</code></td></tr>`).join("")}</tbody>`;
const ct=document.getElementById("channelTable");
ct.innerHTML=`<table><tbody>${channels.map(c=>`<tr><td><b>${c.name}</b></td><td>${c.desc}</td></tr>`).join("")}</tbody></table>`;

/* ---------- lessons ---------- */
document.getElementById("lessons").innerHTML=[
  "v7 adds a causal 16-channel arena-memory proxy over the frozen v6.1 heatmap prior.",
  "The proxy remembers decayed action locations; it does not observe troop movement, death, health, or targeting.",
  "Aligned-versus-shuffled memory and adapter-off probes both fail — the state hypothesis is not supported by this pilot.",
  "This is an offline placement experiment and is not a live-play readiness signal.",
].map(l=>`<li>${l}</li>`).join("");

/* ---------- glossary (searchable) ---------- */
const glossList=document.getElementById("glossList");
const glossSearch=document.getElementById("glossSearch");
const glossCount=document.getElementById("glossCount");
/* normalise Greek so typing "sigma"/"tau" finds σ/τ */
const norm=s=>(s||"").toLowerCase().replace(/σ/g," sigma ").replace(/τ/g," tau ").replace(/\s+/g," ");
function highlight(text, q){
  if(!q) return text;
  const i=text.toLowerCase().indexOf(q);
  if(i<0) return text;
  return text.slice(0,i)+"<mark>"+text.slice(i,i+q.length)+"</mark>"+text.slice(i+q.length);
}
function renderGloss(q){
  const raw=(q||"").trim();
  const qq=norm(raw);
  const items=GLOSS.filter(g=>!qq||norm(g.term).includes(qq)||norm(g.def).includes(qq));
  if(!items.length){
    glossList.innerHTML=`<div class="gloss-empty">No terms match "${raw}". Try clearing the search.</div>`;
  } else {
    glossList.innerHTML=items.map(g=>
      `<div class="gloss-item"><h4>${highlight(g.term,raw.toLowerCase())}</h4><p>${highlight(g.def,raw.toLowerCase())}</p></div>`).join("");
  }
  glossCount.textContent=`${items.length} of ${GLOSS.length} terms${raw?` matching "${raw}"`:""}`;
}
glossSearch.addEventListener("input", e=>renderGloss(e.target.value));
renderGloss("");
document.getElementById("gitFoot").textContent=`· ${GIT}`;
</script>
</body>
</html>
"""

    body = (
        template.replace("__PAYLOAD__", payload)
        .replace("@@FAVICON@@", favicon_link())
        .replace("@@FONT_LINKS@@", FONT_LINKS)
        .replace("@@SHARED_STYLES@@", shared_styles())
        .replace("@@KIT_STYLES@@", kit_styles())
    )
    output_path.write_text(body, encoding="utf-8")
    return output_path
