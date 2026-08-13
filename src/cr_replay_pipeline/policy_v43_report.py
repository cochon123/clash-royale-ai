"""Interactive v4.3 policy training report.

Highlights the larger trunk, fuller data cut, and the toggled latent think
loop. Self-contained native HTML/SVG/JS — no matplotlib.
"""

from __future__ import annotations

import json
import math
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from .report_kit import FONT_LINKS, favicon_link, shared_styles


ROOT = Path(__file__).resolve().parents[2]


def _load(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def _js(data: Any) -> str:
    return json.dumps(data, separators=(",", ":")).replace("</", "<\\/")


def _fmt_duration(seconds: float) -> str:
    seconds = int(round(seconds))
    hours, rem = divmod(seconds, 3600)
    minutes, secs = divmod(rem, 60)
    if hours:
        return f"{hours}h {minutes:02d}m"
    return f"{minutes}m {secs:02d}s"


def _fmt_compact(n: float) -> str:
    if abs(n) >= 1_000_000:
        return f"{n / 1_000_000:.2f}m"
    if abs(n) >= 1_000:
        return f"{n / 1_000:.1f}k"
    return f"{int(round(n)):,}"


def _source_battles(report: dict[str, Any]) -> int:
    compute = report.get("compute", {})
    splits = report.get("data", {}).get("splits") or []
    if compute.get("mirror_training") and len(splits) >= 3:
        return int(splits[0]["battles"] // 2 + splits[1]["battles"] + splits[2]["battles"])
    return int(report["data"]["battles_total"])


def _model_payload(
    label: str,
    report: dict[str, Any],
    history: list[dict[str, Any]],
    color: str,
) -> dict[str, Any]:
    compute = report["compute"]
    train_samples = int(report["data"]["train_samples"])
    epochs = int(compute["epochs_ran"])
    batch = int(compute["batch_size"])
    return {
        "id": label,
        "name": report["model_name"],
        "version": report["model_version"],
        "color": color,
        "created": report["created_at"],
        "seconds": float(report["seconds"]),
        "duration": _fmt_duration(float(report["seconds"])),
        "sourceBattles": _source_battles(report),
        "visibleBattles": int(report["data"]["battles_total"]),
        "trainSamples": train_samples,
        "valSamples": int(report["data"]["val_samples"]),
        "testSamples": int(report["data"]["test_samples"]),
        "sampleVisits": train_samples * epochs,
        "updates": math.ceil(train_samples / batch) * epochs,
        "epochs": epochs,
        "batch": batch,
        "sampleCap": int(compute["max_samples_per_battle"]),
        "mirrored": bool(compute.get("mirror_training", False)),
        "parameters": int(compute.get("parameters") or 0),
        "dModel": int(compute.get("d_model") or 0),
        "numLayers": int(compute.get("num_layers") or 0),
        "maxThinkSteps": int(compute.get("max_think_steps") or 0),
        "evalThinkSteps": int(compute.get("eval_think_steps") or 0),
        "test": report["test"],
        "val": report["val"],
        "rollout": report.get("rollouts", {}),
        "history": history,
        "lessons": report.get("lessons", []),
    }


def render_policy_v43_report(
    output_path: str | Path = "reports/policy_bc_v4_3.html",
    model_dir: str | Path = "models/policy_bc_v4.3",
) -> Path:
    specs = [
        ("v4", ROOT / "models/policy_bc_v4", "#70a1ff"),
        ("v4.1", ROOT / "models/policy_bc_v4.1", "#ffca63"),
        ("v4.2", ROOT / "models/policy_bc_v4.2_full", "#70e1b1"),
        ("v4.3", Path(model_dir) if Path(model_dir).is_absolute() else ROOT / model_dir, "#e8f58b"),
    ]
    models: list[dict[str, Any]] = []
    for label, path, color in specs:
        report = _load(path / "report.json")
        history_path = path / "training_stages.json"
        history = _load(history_path) if history_path.exists() else report.get("history", [])
        models.append(_model_payload(label, report, history, color))

    v43_report = _load(
        (Path(model_dir) if Path(model_dir).is_absolute() else ROOT / model_dir) / "report.json"
    )
    think_off = v43_report["test_think_off"]
    think_on = v43_report["test"]
    v43 = next(model for model in models if model["id"] == "v4.3")
    v42 = next(model for model in models if model["id"] == "v4.2")

    progress_path = Path(v43_report["compute"].get("progress_path") or "")
    if not progress_path.is_absolute():
        progress_path = ROOT / progress_path
    peak_vram = 0.0
    if progress_path.exists():
        for line in progress_path.read_text(encoding="utf-8").splitlines():
            if not line.strip():
                continue
            row = json.loads(line)
            peak_vram = max(peak_vram, float(row.get("gpu_memory_mb") or 0.0))

    matchup_path = ROOT / "reports" / "policy_bc_v4_3_matchup.json"
    matchup = _load(matchup_path) if matchup_path.exists() else None

    card_delta = (think_on["slot_top1"] - v42["test"]["slot_top1"]) * 100.0
    zone_delta = (think_on["zone_acc"] - v42["test"]["zone_acc"]) * 100.0
    xy_delta = think_on["xy_mae"] - v42["test"]["xy_mae"]
    timing_delta = think_on["timing_mae"] - v42["test"]["timing_mae"]
    think_card = (think_on["slot_top1"] - think_off["slot_top1"]) * 100.0
    think_zone = (think_on["zone_acc"] - think_off["zone_acc"]) * 100.0
    think_xy = think_off["xy_mae"] - think_on["xy_mae"]
    think_timing = think_off["timing_mae"] - think_on["timing_mae"]
    param_gain = v43["parameters"] / max(v42["parameters"], 1)
    battle_gain = (v43["sourceBattles"] / max(v42["sourceBattles"], 1) - 1.0) * 100.0
    sample_gain = (v43["trainSamples"] / max(v42["trainSamples"], 1) - 1.0) * 100.0
    created = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")

    payload = {
        "models": models,
        "think": {
            "on": {
                "id": "v4.3 ★",
                "label": "think K=8",
                "color": "#e8f58b",
                "test": think_on,
            },
            "off": {
                "id": "v4.3 off",
                "label": "think K=0",
                "color": "#9bb6ad",
                "test": think_off,
            },
            "maxSteps": v43["maxThinkSteps"],
            "evalSteps": v43["evalThinkSteps"],
            "delta": {
                "slot_top1": think_on["slot_top1"] - think_off["slot_top1"],
                "zone_acc": think_on["zone_acc"] - think_off["zone_acc"],
                "xy_mae": think_on["xy_mae"] - think_off["xy_mae"],
                "timing_mae": think_on["timing_mae"] - think_off["timing_mae"],
                "tile_acc": think_on["tile_acc"] - think_off["tile_acc"],
                "loss": think_on["loss"] - think_off["loss"],
            },
        },
        "meta": {
            "gpu": "NVIDIA GeForce RTX 3050 6GB Laptop",
            "peakVramMb": peak_vram,
            "duration": v43["duration"],
            "created": v43["created"],
            "parameters": v43["parameters"],
            "dModel": v43["dModel"],
            "numLayers": v43["numLayers"],
        },
        "matchup": matchup,
    }

    html = f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>PolicyBC v4.3 — think-loop training report</title>
{favicon_link()}
{FONT_LINKS}
<style>
{shared_styles()}
.model-grid{{display:grid;grid-template-columns:repeat(4,1fr);gap:12px}}
.model-card{{border-top:3px solid var(--tone,var(--blue))}}
.model-tag{{display:inline-flex;align-items:center;gap:7px;font:700 12px "IBM Plex Mono",ui-monospace,monospace;color:var(--tone)}}
.big-number{{font-size:30px;font-weight:750;letter-spacing:-.05em;margin:14px 0 2px}}
.facts{{display:grid;grid-template-columns:1fr 1fr;gap:9px;margin-top:16px;padding-top:14px;border-top:1px solid var(--line)}}
.fact b,.fact span{{display:block}}.fact b{{font-size:13px}}.fact span{{font-size:10px;color:var(--muted);text-transform:uppercase;letter-spacing:.06em}}
.star-col{{background:rgba(232,245,139,.045);border-left:1px solid #596331!important}}
.star-point{{margin-top:18px;border:1px solid #778143;background:radial-gradient(circle at 88% 0,rgba(232,245,139,.14),transparent 36%),linear-gradient(145deg,#162018,#0b1511);border-radius:18px;padding:22px;display:grid;grid-template-columns:56px 1fr;gap:18px}}
.star-icon{{width:56px;height:56px;border-radius:16px;display:grid;place-items:center;background:var(--star);color:#152014;font-size:28px}}
.star-stats{{display:grid;grid-template-columns:repeat(4,1fr);gap:10px;margin-top:16px}}
.star-stat{{padding:11px 13px;border:1px solid var(--line);border-radius:11px;background:#0a151c}}
.star-stat b,.star-stat span{{display:block}}.star-stat b{{color:var(--star);font:750 18px "IBM Plex Mono",ui-monospace,monospace}}.star-stat span{{color:var(--muted);font-size:11px}}
.arch{{display:grid;grid-template-columns:1.15fr .85fr;gap:14px}}
.flow-step b{{display:block;font:700 12px "IBM Plex Mono",ui-monospace,monospace;color:var(--star);margin-bottom:8px}}
.flow-step p{{font-size:12px;line-height:1.45}}.flow-step.active{{border-color:#778143;background:rgba(232,245,139,.06)}}
.toggle-demo{{display:grid;gap:10px;margin-top:14px}}
.toggle-row{{display:grid;grid-template-columns:90px 1fr auto;gap:12px;align-items:center;padding:12px;border:1px solid var(--line);border-radius:12px;background:#0a151c}}
.toggle-row.on{{border-color:#778143}}
.depth-bar{{height:10px;border-radius:99px;background:#08131a;overflow:hidden;border:1px solid #1c2d38}}
.depth-fill{{height:100%;width:0;background:linear-gradient(90deg,var(--green),var(--star));transition:width .8s cubic-bezier(.2,.8,.2,1)}}
.ref-line{{stroke:var(--star);stroke-width:1.5;stroke-dasharray:4 5;opacity:.85}}.ref-point{{fill:var(--star);stroke:#152014;stroke-width:3}}
.delta-list{{display:grid;gap:10px;margin-top:14px}}
.delta-item{{display:grid;grid-template-columns:1fr auto;gap:10px;padding:12px 14px;border:1px solid var(--line);border-radius:12px;background:#0a151c}}
.delta-item b{{font:750 16px "IBM Plex Mono",ui-monospace,monospace}}
.replay-grid{{display:grid;grid-template-columns:1fr 1fr;gap:14px;margin-top:8px}}
.replay-pane h3{{display:flex;justify-content:space-between;align-items:baseline;gap:10px;margin-bottom:10px}}
.pane-score{{font:700 12px "IBM Plex Mono",ui-monospace,monospace;color:var(--muted)}}
.mean-ring{{width:64px;height:64px;border-radius:50%;display:grid;place-items:center;border:2px solid var(--tone,var(--star));background:rgba(232,245,139,.08);font:800 15px "IBM Plex Mono",ui-monospace,monospace;color:var(--tone,var(--star))}}
.chart-card{{padding:22px}}
@media(max-width:980px){{.model-grid,.arch,.replay-grid,.star-stats{{grid-template-columns:1fr}}}}
@media(max-width:560px){{.star-point{{grid-template-columns:1fr}}}}
</style>
</head>
<body><main>
<header class="hero">
  <div class="eyebrow">Clash Royale AI · policy retrospective</div>
  <h1>Bigger trunk.<br><em>Spendable compute.</em></h1>
  <p class="lede">v4.3 keeps the v4.2 recipe — 90/5/5 split, 40 windows, mirrored training — then scales the network to 2.47M parameters and adds a shared-weight latent think loop. Inference can stay fast at K=0 or spend extra refine steps at K=8 with no second checkpoint.</p>
  <div class="stamp">REPORT GENERATED {created} · MODEL policy-bc-v4.3 · VERSION 4.3.0 · TRAINED {v43['created']}</div>
  <div class="verdict"><div class="mark">↑</div><div><strong>Verdict: v4.3 is the strongest supervised checkpoint in the v4 lineage, and the think toggle pays for itself on the same test set.</strong><p>Against full-data v4.2, think-on v4.3 gains {card_delta:+.2f} card points, {zone_delta:+.2f} zone points, improves placement by {abs(xy_delta):.0f} API units, and timing by {abs(timing_delta):.3f}s. On identical 33,604 v4.3 test samples, turning thinking on (K=8 vs K=0) adds {think_card:+.2f} card points and {think_timing:+.3f}s of timing without retraining.</p></div></div>
</header>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">01 · experiment at a glance</div><h2>Capacity, data, and wall clock</h2></div><p>Source battles exclude mirrored copies. Peak VRAM is from the training progress log on the local RTX 3050 Laptop.</p></div>
  <div class="kpis">
    <div class="kpi" style="--tone:var(--star)"><span>Parameters</span><b>{_fmt_compact(v43['parameters'])}</b><small>{param_gain:.2f}× vs v4.2 · d={v43['dModel']} · L={v43['numLayers']}</small></div>
    <div class="kpi" style="--tone:var(--green)"><span>Source battles</span><b>{v43['sourceBattles']:,}</b><small>+{battle_gain:.0f}% vs v4.2 · all usable raw</small></div>
    <div class="kpi" style="--tone:var(--green)"><span>Train samples / epoch</span><b>{_fmt_compact(v43['trainSamples'])}</b><small>+{sample_gain:.0f}% vs v4.2 · mirrored</small></div>
    <div class="kpi" style="--tone:var(--gold)"><span>Wall time</span><b>{v43['duration']}</b><small>peak VRAM {peak_vram:.0f} MB · CUDA</small></div>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">02 · think loop</div><h2>One model, two inference budgets</h2></div><p>After the trunk fuses history, decks, and globals, a residual MLP can refine that latent K times before the action heads. K is free at inference.</p></div>
  <div class="arch">
    <div class="card">
      <h3>Forward path</h3>
      <p>Shared weights mean deeper K spends more FLOPs without loading another network.</p>
      <div class="flow">
        <div class="flow-step"><b>01 · encode</b><p>GRU over the last 64 causal events.</p></div>
        <div class="flow-step"><b>02 · fuse</b><p>History + deck matchup + globals.</p></div>
        <div class="flow-step active"><b>03 · think</b><p>Residual refine loop, K ∈ 0…8.</p></div>
        <div class="flow-step"><b>04 · heads</b><p>Card, type, zone, XY, timing.</p></div>
        <div class="flow-step"><b>05 · toggle</b><p><code>--think-steps 0|8</code></p></div>
      </div>
      <div class="expr"><b>training:</b> K ~ Uniform(0, 8) each batch<br><b>eval / report ★:</b> K = 8<br><b>fast path:</b> K = 0 (identity through ThinkRefiner)</div>
    </div>
    <div class="card">
      <h3>Compute dial</h3>
      <p>Same weights, different depth. Useful when phone-lab latency budget allows it.</p>
      <div class="toggle-demo">
        <div class="toggle-row"><b>K=0</b><div class="depth-bar"><div class="depth-fill" data-width="8"></div></div><span class="pill">off / fast</span></div>
        <div class="toggle-row on"><b>K=8</b><div class="depth-bar"><div class="depth-fill" data-width="100"></div></div><span class="pill">★ report default</span></div>
      </div>
      <div class="star-stats" style="grid-template-columns:1fr 1fr;margin-top:18px">
        <div class="star-stat"><b>{think_on['slot_top1']*100:.2f}%</b><span>card @ K=8</span></div>
        <div class="star-stat"><b>{think_off['slot_top1']*100:.2f}%</b><span>card @ K=0</span></div>
        <div class="star-stat"><b>{think_on['timing_mae']:.3f}s</b><span>timing @ K=8</span></div>
        <div class="star-stat"><b>{think_off['timing_mae']:.3f}s</b><span>timing @ K=0</span></div>
      </div>
    </div>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">03 · lineage</div><h2>From small BC trunks to a thinkable policy</h2></div><p>v4–v4.2 share a 633k architecture. v4.3 is the first capacity bump and the first inference-time compute dial.</p></div>
  <div class="model-grid" id="modelCards"></div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">04 · held-out test set</div><h2>Lineage scores + the think toggle</h2></div><p>The ★ column is v4.3 at K=8. The off column is the same checkpoint and same 33,604 labels at K=0 — a paired comparison, not a second model.</p></div>
  <div class="card" style="padding:4px 0;overflow:auto">
    <table class="metric-table" id="metricTable"></table>
  </div>
  <div class="star-point"><div class="star-icon">★</div><div>
    <h3>Star point · thinking is free after training</h3>
    <p>On the exact v4.3 test split, enabling the refine loop beats the fast path on every tracked action metric. Gains are modest but consistent, which is what you want from a toggle rather than a brittle second mode.</p>
    <div class="star-stats">
      <div class="star-stat"><b>{think_card:+.2f}pt</b><span>card top-1</span></div>
      <div class="star-stat"><b>{think_zone:+.2f}pt</b><span>zone accuracy</span></div>
      <div class="star-stat"><b>−{think_xy:.0f}</b><span>XY MAE units</span></div>
      <div class="star-stat"><b>−{think_timing:.3f}s</b><span>timing MAE</span></div>
    </div>
  </div></div>
  <div class="callout"><b>Fairness note:</b> v4/v4.1 used 70/15/15; v4.2/v4.3 used 90/5/5 with mirroring. Cross-version gaps are suggestive. The K=8 vs K=0 delta is the clean experiment inside this report.</div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">05 · validation during training</div><h2>Curves by epoch</h2></div><p>Validation uses each run’s own split. Toggle models, switch metrics, hover points, or replay the animation. The dashed ★ line is v4.3 think-on test, not a training epoch.</p></div>
  <div class="card chart-card">
    <div class="chart-tools" id="metricButtons"></div>
    <div class="chart-tools" id="modelToggles"></div>
    <div id="thinkReference" class="star-point" style="grid-template-columns:1fr;padding:12px 14px;margin:0 0 12px"></div>
    <button id="play">replay curves</button>
    <svg id="curve" viewBox="0 0 1040 390" role="img" aria-label="Interactive training curves"></svg>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">06 · vs v4.2</div><h2>What the scale-up bought</h2></div><p>Same recipe, bigger net, more battles. Think-on is the primary reported mode because that is how the checkpoint was selected.</p></div>
  <div class="compare-grid">
    <div class="card">
      <h3>v4.3 ★ minus v4.2</h3>
      <div class="delta-list">
        <div class="delta-item"><span>Card top-1</span><b class="{'up' if card_delta>=0 else 'down'}">{card_delta:+.2f} pt</b></div>
        <div class="delta-item"><span>Zone accuracy</span><b class="{'up' if zone_delta>=0 else 'down'}">{zone_delta:+.2f} pt</b></div>
        <div class="delta-item"><span>XY MAE</span><b class="{'up' if xy_delta<=0 else 'down'}">{xy_delta:+.0f} units</b></div>
        <div class="delta-item"><span>Timing MAE</span><b class="{'up' if timing_delta<=0 else 'down'}">{timing_delta:+.3f}s</b></div>
      </div>
    </div>
    <div class="card">
      <h3>Budget comparison</h3>
      <div class="bars" id="budgetBars"></div>
      <div class="callout" style="margin-top:16px"><b>Rollout realism:</b> policy mean {v43['rollout'].get('mean_score_policy', 0):.3f} vs real {v43['rollout'].get('mean_score_real', 0):.3f} on {int(v43['rollout'].get('n') or 0)} continuations. Gap to real {v43['rollout'].get('policy_gap_to_real', 0):+.3f}.</div>
    </div>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">07 · compute &amp; data recipe</div><h2>How this run was built</h2></div><p>Explicit knobs used for the archived checkpoint under <code>models/policy_bc_v4.3</code>.</p></div>
  <div class="kpis">
    <div class="kpi" style="--tone:var(--blue)"><span>Architecture</span><b>256×3</b><small>+ ThinkRefiner max K=8</small></div>
    <div class="kpi" style="--tone:var(--green)"><span>Batch × epochs</span><b>512 × 10</b><small>{v43['updates']:,} optimizer updates</small></div>
    <div class="kpi" style="--tone:var(--gold)"><span>Sample cap</span><b>40</b><small>windows / battle · reaction×2</small></div>
    <div class="kpi" style="--tone:var(--star)"><span>Split</span><b>90/5/5</b><small>mirror train only</small></div>
  </div>
  <div class="expr" style="margin-top:18px">
    cr-replays train-policy --version 4.3 --d-model 256 --num-layers 3<br>
    --max-think-steps 8 --eval-think-steps 8 --mirror-training<br>
    --batch-size 512 --epochs 10 --max-samples-per-battle 40
  </div>
</section>

<section class="section" id="matchupSection">
  <div class="section-head"><div><div class="eyebrow">08 · human vs AI style</div><h2>Same prefix, two futures — toggle the policy</h2></div><p>Shared 512-battle test pool. Each policy continues the same warm-up; the style judge scores how far the continuation drifts from human statistics.</p></div>
  <div class="match-chips" id="matchChips"></div>
  <div class="match-kpis">
    <div class="kpi" style="--tone:var(--star)"><span>Overall mean |Δ|</span><b id="meanAbsZ">—</b><small id="meanAbsZCI">95% bootstrap CI</small></div>
    <div class="kpi" style="--tone:var(--green)"><span>Mean P(human|AI)</span><b id="meanPHuman">—</b><small id="meanPNote">transfer judge · lower = more detected</small></div>
    <div class="kpi" style="--tone:var(--blue)"><span>Paired games</span><b id="matchN">—</b><small id="matchProtocol">shared seed battles</small></div>
    <div class="kpi" style="--tone:var(--gold)"><span>Human baseline</span><b id="humanP">—</b><small>mean P(human|human)</small></div>
  </div>
  <div class="compare-grid">
    <div class="card chart-card">
      <h3>Feature distance vs human</h3>
      <p style="margin-bottom:12px">Bars are (AI − human) / human σ on the strongest non-harness tells. Switching models morphs the profile.</p>
      <svg class="delta-chart" id="deltaChart" viewBox="0 0 640 420" role="img" aria-label="Feature deltas vs human"></svg>
      <div class="expr" id="deltaExpr"><b>Δ</b> = (μ<sub>AI</sub> − μ<sub>human</sub>) / σ<sub>human</sub> &nbsp;·&nbsp; <b>mean |Δ|</b> averages absolute z over these features</div>
    </div>
    <div class="card">
      <h3>Lineage distance</h3>
      <p>Lower mean |Δ| means closer to the human feature cloud on this shared pool.</p>
      <div class="bars" id="lineageBars" style="margin-top:16px"></div>
      <div class="callout" style="margin-top:16px"><b>Read:</b> this is not live win-rate. It is how statistically human the offline continuation looks under the alternation-trained style judge.</div>
    </div>
  </div>
  <div class="card chart-card" style="margin-top:14px">
    <h3>Replay · same warm-up, diverging futures</h3>
    <p style="margin-bottom:12px">Grey dots are the shared human prefix. After the cut, left stays human; right is the selected policy. Change model to morph the AI arena.</p>
    <div class="replay-chips" id="replayChips"></div>
    <div class="replay-grid">
      <div class="replay-pane">
        <h3>Human <span class="pane-score" id="humanScore"></span></h3>
        <svg class="arena" id="humanArena"></svg>
      </div>
      <div class="replay-pane">
        <h3>AI · <span id="aiModelLabel">v4.3</span> <span class="pane-score" id="aiScore"></span></h3>
        <svg class="arena" id="aiArena"></svg>
      </div>
    </div>
    <div class="anim-toolbar">
      <button type="button" id="replayPlay">▶ play</button>
      <input type="range" class="anim-scrubber" id="replayScrub" min="1" max="1" value="1" step="1">
      <div class="anim-readout" id="replayReadout"></div>
    </div>
    <div class="replay-log" id="replayLog"></div>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">09 · lessons</div><h2>What v4.3 taught us</h2></div><p>Capacity finally moved the needle after v4.2 proved the data recipe.</p></div>
  <div class="lessons">
    <div class="card lesson"><div class="n">01 / CAPACITY</div><h3>A 3.9× larger trunk helped.</h3><p>Moving from 633k to 2.47M parameters on more data improved every primary offline action metric versus full-data v4.2, without blowing the 6GB laptop budget (peak ~{peak_vram:.0f} MB).</p></div>
    <div class="card lesson"><div class="n">02 / THINKING</div><h3>Train random depth; toggle at serve time.</h3><p>Sampling K during training keeps both the fast path and the deep path competent. At inference, <code>--think-steps</code> is just a compute dial.</p></div>
    <div class="card lesson"><div class="n">03 / STYLE</div><h3>Closer action metrics ≠ human style.</h3><p>Use the matchup toggle above: mean |Δ| ranks how far each lineage step still sits from human continuations on the same prefixes.</p></div>
  </div>
  <div class="callout"><b>How to use it:</b> default live to <code>--think-steps 0</code> when latency matters; flip to <code>8</code> for offline eval or slower lab runs. Phone-lab accepts the same flag.</div>
</section>

<footer class="foot">Native HTML + SVG + JavaScript · no matplotlib · think-on is the checkpoint-selection mode · think-off uses identical test labels · not a live Clash Royale readiness claim.</footer>
</main>
<div class="tooltip" id="tip"></div>
<script id="report-data" type="application/json">{_js(payload)}</script>
<script>
const D=JSON.parse(document.getElementById('report-data').textContent);
const $=s=>document.querySelector(s);
const fmt=n=>new Intl.NumberFormat('en-US').format(Math.round(n));
const pct=n=>(100*n).toFixed(2)+'%';
const compact=n=>n>=1e6?(n/1e6).toFixed(2)+'m':n>=1e3?(n/1e3).toFixed(1)+'k':fmt(n);

requestAnimationFrame(()=>document.querySelectorAll('.depth-fill').forEach(el=>{{el.style.width=el.dataset.width+'%';}}));

$('#modelCards').innerHTML=D.models.map(m=>`
  <article class="card model-card" style="--tone:${{m.color}}">
    <div class="model-tag"><i class="dot"></i>${{m.id}}</div>
    <div class="big-number">${{compact(m.parameters||0)}}</div>
    <p>parameters · d=${{m.dModel}} L=${{m.numLayers}}</p>
    <div class="facts">
      <div class="fact"><b>${{fmt(m.sourceBattles)}}</b><span>source battles</span></div>
      <div class="fact"><b>${{compact(m.trainSamples)}}</b><span>samples / epoch</span></div>
      <div class="fact"><b>${{m.epochs}}</b><span>epochs</span></div>
      <div class="fact"><b>${{m.batch}}</b><span>batch</span></div>
      <div class="fact"><b>${{m.maxThinkSteps||0}}</b><span>max think K</span></div>
      <div class="fact"><b>${{m.duration}}</b><span>wall time</span></div>
    </div>
    ${{m.mirrored?'<p style="margin-top:14px;color:var(--green)">mirrored training</p>':''}}
  </article>`).join('');

const metrics=[
  ['Card top-1','slot_top1','pct',true,'Correct card slot'],
  ['Card top-3','slot_top3','pct',true,'Human card in top 3'],
  ['Zone accuracy','zone_acc','pct',true,'Correct arena region'],
  ['Within one tile','tile_acc','pct',true,'≤1,000 API units'],
  ['Placement error','xy_mae','int',false,'API units · lower better'],
  ['Timing error','timing_mae','sec',false,'Seconds · lower better'],
  ['Test loss','loss','float',false,'Composite objective']
];
const format=(v,t)=>t==='pct'?pct(v):t==='sec'?v.toFixed(3)+'s':t==='int'?fmt(v):v.toFixed(3);
const variants=[...D.models.filter(m=>m.id!=='v4.3'), D.think.off, D.think.on];
$('#metricTable').innerHTML='<thead><tr><th>Metric</th>'+variants.map((m,i)=>`<th class="${{m.id.includes('★')?'star-col':''}}" style="color:${{m.color}}">${{m.id}}</th>`).join('')+'</tr></thead><tbody>'+
  metrics.map(([label,key,type,higher,note])=>{{
    const vals=variants.map(m=>m.test[key]);
    const best=higher?Math.max(...vals):Math.min(...vals);
    return `<tr><td><b>${{label}}</b><span class="metric-note">${{note}}</span></td>`+
      vals.map((v,i)=>`<td class="${{v===best?'winner ':''}}${{variants[i].id.includes('★')?'star-col':''}}">${{format(v,type)}}</td>`).join('')+
      '</tr>';
  }}).join('')+'</tbody>';

const curveMetrics={{
  val_loss:['Validation loss',false],
  val_slot_top1:['Validation card top-1',true],
  val_zone_acc:['Validation zone accuracy',true],
  val_tile_acc:['Validation within one tile',true],
  val_timing_mae:['Validation timing MAE',false]
}};
const thinkMap={{val_loss:'loss',val_slot_top1:'slot_top1',val_zone_acc:'zone_acc',val_tile_acc:'tile_acc',val_timing_mae:'timing_mae'}};
let activeMetric='val_slot_top1', visible=new Set(D.models.map(m=>m.id)), reveal=1;
$('#metricButtons').innerHTML=Object.entries(curveMetrics).map(([k,v],i)=>`<button data-metric="${{k}}" class="${{k===activeMetric?'active':''}}">${{v[0]}}</button>`).join('');
$('#modelToggles').innerHTML=D.models.map(m=>`<label class="toggle" style="--tone:${{m.color}}"><input type="checkbox" data-model="${{m.id}}" checked> ${{m.id}}</label>`).join('');
function updateThinkReference(){{
  const key=thinkMap[activeMetric], value=D.think.on.test[key], label=curveMetrics[activeMetric][0].replace('Validation ','');
  const formatted=activeMetric.includes('acc')||activeMetric==='val_slot_top1'?pct(value):activeMetric==='val_timing_mae'?value.toFixed(3)+'s':value.toFixed(3);
  const off=D.think.off.test[key];
  const offFmt=activeMetric.includes('acc')||activeMetric==='val_slot_top1'?pct(off):activeMetric==='val_timing_mae'?off.toFixed(3)+'s':off.toFixed(3);
  $('#thinkReference').innerHTML=`<div><strong style="color:var(--star)">★ v4.3 think K=8 test · ${{label}}: ${{formatted}}</strong><p style="margin-top:4px">fast path K=0 on the same labels: ${{offFmt}} · dashed line is test reference, not an epoch</p></div>`;
}}
$('#metricButtons').onclick=e=>{{if(!e.target.dataset.metric)return;activeMetric=e.target.dataset.metric;document.querySelectorAll('[data-metric]').forEach(b=>b.classList.toggle('active',b===e.target));reveal=1;updateThinkReference();draw()}};
$('#modelToggles').onchange=e=>{{e.target.checked?visible.add(e.target.dataset.model):visible.delete(e.target.dataset.model);draw()}};

const svg=$('#curve'), tip=$('#tip');
function draw(){{
  const W=1040,H=390,p={{l:68,r:24,t:24,b:48}};
  const shown=D.models.filter(m=>visible.has(m.id));
  const ref=D.think.on.test[thinkMap[activeMetric]];
  const all=shown.flatMap(m=>m.history.map(r=>r[activeMetric]).filter(Number.isFinite)).concat([ref]);
  if(!all.length)return;
  let lo=Math.min(...all), hi=Math.max(...all); const pad=(hi-lo||1)*.1; lo-=pad; hi+=pad;
  const maxEpoch=Math.max(...shown.map(m=>m.history.length));
  const x=e=>p.l+(e-1)/Math.max(maxEpoch-1,1)*(W-p.l-p.r);
  const y=v=>p.t+(hi-v)/(hi-lo)*(H-p.t-p.b);
  let out='';
  for(let i=0;i<5;i++){{const yy=p.t+i*(H-p.t-p.b)/4,v=hi-i*(hi-lo)/4;out+=`<line class="gridline" x1="${{p.l}}" y1="${{yy}}" x2="${{W-p.r}}" y2="${{yy}}"/><text class="axis-label" x="${{p.l-10}}" y="${{yy+4}}" text-anchor="end">${{(activeMetric.includes('acc')||activeMetric==='val_slot_top1')?pct(v):v.toFixed(2)}}</text>`}}
  for(let i=1;i<=maxEpoch;i+=Math.max(1,Math.ceil(maxEpoch/8)))out+=`<text class="axis-label" x="${{x(i)}}" y="${{H-17}}" text-anchor="middle">${{i}}</text>`;
  shown.forEach(m=>{{
    const rows=m.history.slice(0,Math.ceil(m.history.length*reveal));
    const pts=rows.map(r=>[x(r.epoch),y(r[activeMetric]),r]);
    out+=`<path class="curve" pathLength="1" stroke="${{m.color}}" d="${{pts.map((q,i)=>(i?'L':'M')+q[0].toFixed(1)+','+q[1].toFixed(1)).join(' ')}}"/>`;
    out+=pts.map(q=>`<circle class="point" data-model="${{m.id}}" data-epoch="${{q[2].epoch}}" data-value="${{q[2][activeMetric]}}" fill="${{m.color}}" cx="${{q[0]}}" cy="${{q[1]}}" r="4"/>`).join('');
  }});
  const sx=W-p.r-10, sy=Math.max(p.t+12, Math.min(H-p.b-12, y(ref)));
  out+=`<line class="ref-line" x1="${{p.l}}" y1="${{sy}}" x2="${{sx}}" y2="${{sy}}"/><circle class="ref-point point" data-model="v4.3 ★" data-epoch="test" data-value="${{ref}}" cx="${{sx}}" cy="${{sy}}" r="7"/>`;
  out+=`<text class="axis-label" x="${{W/2}}" y="${{H-2}}" text-anchor="middle">epoch</text>`;
  svg.innerHTML=out;
  svg.querySelectorAll('.point').forEach(el=>{{
    el.onmouseenter=()=>{{tip.style.opacity=1;tip.innerHTML=`<b>${{el.dataset.model}} · ${{el.dataset.epoch}}</b><br>${{curveMetrics[activeMetric][0]}}: ${{Number(el.dataset.value).toFixed(4)}}`}};
    el.onmousemove=e=>{{tip.style.left=e.clientX+'px';tip.style.top=e.clientY+'px'}};
    el.onmouseleave=()=>tip.style.opacity=0;
  }});
}}
$('#play').onclick=()=>{{reveal=0;const start=performance.now();(function tick(t){{reveal=Math.min(1,(t-start)/1800);draw();if(reveal<1)requestAnimationFrame(tick)}})(start)}};
updateThinkReference(); draw();

const budget=[
  ['parameters','Parameters'],
  ['sourceBattles','Source battles'],
  ['trainSamples','Samples / epoch'],
  ['sampleVisits','Sample visits'],
];
$('#budgetBars').innerHTML=budget.map(([key,label])=>{{
  const a=D.models.find(m=>m.id==='v4.2')[key];
  const b=D.models.find(m=>m.id==='v4.3')[key];
  const max=Math.max(a,b);
  return `<div style="margin-bottom:14px"><div style="display:flex;justify-content:space-between;margin-bottom:6px"><b>${{label}}</b><span class="pill">${{compact(b)}} / ${{compact(a)}}</span></div>
    <div class="bar-row"><b style="color:#70e1b1">v4.2</b><div class="bar-track"><div class="bar-fill" style="--tone:#70e1b1" data-width="${{100*a/max}}"></div></div><div class="bar-value">${{compact(a)}}</div></div>
    <div class="bar-row"><b style="color:#e8f58b">v4.3</b><div class="bar-track"><div class="bar-fill" style="--tone:#e8f58b" data-width="${{100*b/max}}"></div></div><div class="bar-value">${{compact(b)}}</div></div>
  </div>`;
}}).join('');
requestAnimationFrame(()=>document.querySelectorAll('.bar-fill').forEach(b=>b.style.width=b.dataset.width+'%'));

/* ---------- human vs AI matchup ---------- */
(function(){{
  const M=D.matchup;
  const section=$('#matchupSection');
  if(!M||!M.models||!M.models.length){{
    if(section) section.innerHTML='<div class="section-head"><div><div class="eyebrow">08 · human vs AI style</div><h2>Matchup payload pending</h2></div><p>Generate <code>reports/policy_bc_v4_3_matchup.json</code> then re-run the report.</p></div>';
    return;
  }}
  const byId=Object.fromEntries(M.models.map(m=>[m.id,m]));
  let activeId=byId['v4.3']?'v4.3':M.models[0].id;
  let display={{...byId[activeId], featureDelta:byId[activeId].featureDelta.map(r=>({{...r}}))}};
  let animToken=0;
  let replayIdx=0, scrubN=1, playTimer=null, morphT=1;
  let aiFrom=null, aiTo=null;

  const SVGNS='http://www.w3.org/2000/svg';
  const el=(tag,attrs={{}})=>{{const n=document.createElementNS(SVGNS,tag);for(const[k,v]of Object.entries(attrs))n.setAttribute(k,v);return n;}};
  const svgText=(attrs,text)=>{{const t=el('text',attrs);t.textContent=text;return t;}};
  const ease=t=>1-Math.pow(1-t,3);
  const lerp=(a,b,t)=>a+(b-a)*t;
  const titleCase=s=>String(s).replace(/[-_]/g,' ').replace(/\\b\\w/g,c=>c.toUpperCase());
  const sci=v=>{{
    const n=Number(v);
    if(!Number.isFinite(n))return '—';
    if(Math.abs(n)>=0.01)return n.toFixed(3);
    return n.toExponential(2);
  }};

  function towerOverlay(g,w,h,ox,oy,scale,opacity){{
    const tw=w*0.11;
    const tower=(cx,cy,king,friendly)=>{{
      const size=king?tw*1.35:tw;
      g.appendChild(el('rect',{{x:cx-size/2,y:cy-size/2,width:size,height:size,rx:3*scale,fill:friendly?'#1e3a8a':'#7f1d1d',stroke:friendly?'#93c5fd':'#fca5a5','stroke-width':1.2*scale,opacity}}));
    }};
    tower(ox+w*0.22,oy+h*0.19,false,false);tower(ox+w*0.78,oy+h*0.19,false,false);tower(ox+w*0.5,oy+h*0.075,true,false);
    tower(ox+w*0.22,oy+h*0.81,false,true);tower(ox+w*0.78,oy+h*0.81,false,true);tower(ox+w*0.5,oy+h*0.925,true,true);
  }}
  function arenaBackdrop(g,w,h,ox,oy,scale){{
    const rect=(x,y,ww,hh,attrs)=>g.appendChild(el('rect',Object.assign({{x,y,width:ww,height:hh}},attrs)));
    rect(ox,oy,w,h,{{fill:'#0d1a2b',rx:10*scale}});
    rect(ox,oy,w,h*0.5,{{fill:'#991b1b',opacity:0.28,rx:10*scale}});
    rect(ox,oy+h*0.5,w,h*0.5,{{fill:'#1d4ed8',opacity:0.22}});
    rect(ox,oy+h*0.465,w,h*0.07,{{fill:'#1d4ed8',opacity:0.45}});
    rect(ox+w*0.16,oy+h*0.455,w*0.12,h*0.09,{{fill:'#7c5c33',opacity:0.75}});
    rect(ox+w*0.72,oy+h*0.455,w*0.12,h*0.09,{{fill:'#7c5c33',opacity:0.75}});
    towerOverlay(g,w,h,ox,oy,scale,0.85);
  }}
  function mountArena(svg,opts={{}}){{
    const W=opts.W||300,H=opts.H||450,PAD=6,TOP=20;
    const AW=W-PAD*2,AH=H-TOP-22;
    svg.setAttribute('viewBox',`0 0 ${{W}} ${{H}}`);
    svg.innerHTML='';
    const g=el('g');svg.appendChild(g);
    arenaBackdrop(g,AW,AH,PAD,TOP,1);
    const geom={{xPx:nx=>PAD+nx*AW,yPx:ny=>TOP+(1-ny)*AH,W,H,PAD,TOP,AW,AH}};
    geom.captions=()=>{{
      g.appendChild(svgText({{x:PAD,y:13,fill:'#fca5a5','font-size':11,'letter-spacing':'0.14em','font-weight':700}},'ENEMY SIDE ↑'));
      g.appendChild(svgText({{x:PAD,y:H-6,fill:'#93c5fd','font-size':11,'letter-spacing':'0.14em','font-weight':700}},'YOUR SIDE ↓'));
    }};
    return {{g,geom}};
  }}

  function paintKpis(model,t=1){{
    const from=display, to=model;
    const mean=lerp(from.meanAbsZ,to.meanAbsZ,t);
    const p=lerp(from.meanPHuman,to.meanPHuman,t);
    $('#meanAbsZ').textContent=mean.toFixed(2)+'σ';
    const ci=to.meanAbsZCI||[mean,mean];
    $('#meanAbsZCI').textContent=`95% CI ${{ci[0].toFixed(2)}}–${{ci[1].toFixed(2)}} · n=${{to.n}}`;
    $('#meanPHuman').textContent=sci(p);
    $('#meanPNote').textContent=`fool@0.5 ${{(100*(to.foolRate||0)).toFixed(1)}}% · ${{to.policyId}}`;
    $('#matchN').textContent=fmt(to.n);
    $('#matchProtocol').textContent=M.protocol||'shared test pool';
    $('#humanP').textContent=sci(M.humanMeanP||to.humanMeanP);
  }}

  function drawDelta(model,t=1){{
    const svg=$('#deltaChart'); if(!svg)return;
    const W=640,H=420,L=168,R=24,T=18,B=28;
    const rows=model.featureDelta;
    const fromRows=display.featureDelta;
    const zs=rows.map((r,i)=>lerp((fromRows[i]&&fromRows[i].z)||0,r.z,t));
    const maxAbs=Math.max(1.5,...M.models.flatMap(m=>m.featureDelta.map(r=>Math.abs(r.z))));
    const rowH=(H-T-B)/rows.length;
    const x0=L+(W-L-R)/2;
    const xScale=v=>x0+(v/maxAbs)*((W-L-R)/2);
    let out=`<line x1="${{x0}}" y1="${{T}}" x2="${{x0}}" y2="${{H-B}}" stroke="#355247" stroke-dasharray="4 4"/>`;
    out+=`<text x="${{x0}}" y="${{H-8}}" text-anchor="middle" fill="#668078" font-size="11" font-family="IBM Plex Mono,monospace">human μ</text>`;
    rows.forEach((r,i)=>{{
      const z=zs[i], y=T+i*rowH+rowH*0.18, h=rowH*0.64;
      const x1=xScale(0), x2=xScale(z);
      const fill=z>=0?'#ff7e78':'#70e1b1';
      const left=Math.min(x1,x2), width=Math.max(2,Math.abs(x2-x1));
      out+=`<text x="${{L-8}}" y="${{y+h*0.72}}" text-anchor="end" fill="#9bb6ad" font-size="11" font-family="IBM Plex Mono,monospace">${{r.feature}}</text>`;
      out+=`<rect class="delta-bar" x="${{left}}" y="${{y}}" width="${{width}}" height="${{h}}" rx="5" fill="${{fill}}" opacity="0.85"/>`;
      out+=`<text x="${{z>=0?x2+6:x2-6}}" y="${{y+h*0.72}}" text-anchor="${{z>=0?'start':'end'}}" fill="${{fill}}" font-size="11" font-family="IBM Plex Mono,monospace" font-weight="700">${{z>=0?'+':''}}${{z.toFixed(2)}}σ</text>`;
    }});
    svg.innerHTML=out;
  }}

  function drawLineage(){{
    const host=$('#lineageBars');
    const max=Math.max(...M.models.map(m=>m.meanAbsZ),0.01);
    host.innerHTML=M.models.map(m=>`
      <div class="bar-row" data-lineage="${{m.id}}" style="cursor:pointer">
        <b style="color:${{m.color}}">${{m.id}}</b>
        <div class="bar-track"><div class="bar-fill" style="--tone:${{m.color}}" data-width="${{100*m.meanAbsZ/max}}"></div></div>
        <div class="bar-value">${{m.meanAbsZ.toFixed(2)}}σ</div>
      </div>`).join('');
    requestAnimationFrame(()=>host.querySelectorAll('.bar-fill').forEach(b=>b.style.width=b.dataset.width+'%'));
    host.onclick=e=>{{
      const row=e.target.closest('[data-lineage]');
      if(row) selectModel(row.dataset.lineage);
    }};
  }}

  function eventAt(track,i){{return track&&track.events?track.events[i]:null;}}
  function blendedAI(upto){{
    const pair=M.trajectories[replayIdx];
    const warm=pair.warmupEvents||12;
    const to=aiTo||pair.aiByModel[activeId];
    const from=aiFrom||to;
    const n=Math.min(upto, Math.max(from.events.length,to.events.length));
    const out=[];
    for(let i=0;i<n;i++){{
      const a=eventAt(from,Math.min(i,from.events.length-1));
      const b=eventAt(to,Math.min(i,to.events.length-1));
      if(!a||!b)continue;
      out.push({{
        t:lerp(a.t,b.t,morphT),
        side:morphT<0.5?a.side:b.side,
        card:morphT<0.5?a.card:b.card,
        x:lerp(a.x,b.x,morphT),
        y:lerp(a.y,b.y,morphT),
        warm:i<warm,
      }});
    }}
    return {{events:out, score:lerp(from.score,to.score,morphT)}};
  }}

  function drawArenas(upto){{
    const pair=M.trajectories[replayIdx];
    if(!pair)return;
    const warm=pair.warmupEvents||12;
    const human=pair.human;
    const ai=blendedAI(upto);
    function paint(svg, track, uptoN, accent){{
      const {{g,geom}}=mountArena(svg,{{W:300,H:450}});
      geom.captions();
      track.events.slice(0,uptoN).forEach((e,i)=>{{
        const isWarm=e.warm!=null?e.warm:i<warm;
        const color=isWarm?'#94a3b8':(e.side==='team'?'#60a5fa':accent||'#fca5a5');
        const latest=i===uptoN-1;
        g.appendChild(el('circle',{{
          cx:geom.xPx(e.x), cy:geom.yPx(e.y), r:latest?8:5,
          fill:color, opacity:isWarm?0.42:0.88,
          stroke:latest?'#e2e8f0':'none','stroke-width':latest?2:0
        }}));
      }});
    }}
    paint($('#humanArena'), human, upto, '#70a1ff');
    paint($('#aiArena'), ai, upto, byId[activeId].color);
    $('#humanScore').textContent=`P(human)=${{sci(human.score)}}`;
    $('#aiScore').textContent=`P(human)=${{sci(ai.score)}}`;
    $('#aiModelLabel').textContent=activeId;
    const h=human.events[Math.min(upto,human.events.length)-1];
    const a=ai.events[Math.min(upto,ai.events.length)-1];
    $('#replayReadout').textContent=`event ${{upto}}/${{scrubN}} ${{upto<=warm?'(shared warm-up)':'(diverged)'}}`;
    $('#replayLog').innerHTML=(h&&a)
      ?`human: <b>${{titleCase(h.card)}}</b> @ ${{h.t}}s (${{h.side}}) &nbsp;·&nbsp; AI: <b>${{titleCase(a.card)}}</b> @ ${{a.t}}s (${{a.side}})`
      :'';
  }}

  function setReplay(i){{
    replayIdx=i;
    [...$('#replayChips').children].forEach((c,j)=>c.classList.toggle('active',j===i));
    const pair=M.trajectories[i];
    const ai=pair.aiByModel[activeId];
    aiFrom=ai; aiTo=ai; morphT=1;
    scrubN=Math.min(pair.human.events.length, ai.events.length);
    const scrub=$('#replayScrub');
    scrub.max=scrubN; scrub.value=scrubN;
    drawArenas(scrubN);
  }}

  function selectModel(id){{
    if(!byId[id]||(id===activeId&&morphT===1&&aiFrom===aiTo))return;
    const target=byId[id];
    const pair=M.trajectories[replayIdx];
    aiFrom=pair?pair.aiByModel[activeId]:null;
    aiTo=pair?pair.aiByModel[id]:null;
    activeId=id;
    [...$('#matchChips').children].forEach(c=>{{
      const on=c.dataset.model===id;
      c.classList.toggle('active',on);
      if(on)c.style.setProperty('--tone',target.color);
    }});
    const token=++animToken;
    const start=performance.now();
    const dur=520;
    (function tick(now){{
      if(token!==animToken)return;
      const t=ease(Math.min(1,(now-start)/dur));
      morphT=t;
      paintKpis(target,t);
      drawDelta(target,t);
      drawArenas(Number($('#replayScrub').value));
      if(t<1)requestAnimationFrame(tick);
      else{{
        display={{...target, featureDelta:target.featureDelta.map(r=>({{...r}}))}};
        morphT=1; aiFrom=aiTo;
        paintKpis(target,1); drawDelta(target,1); drawArenas(Number($('#replayScrub').value));
      }}
    }})(start);
  }}

  $('#matchChips').innerHTML=M.models.map(m=>`<button type="button" class="match-chip${{m.id===activeId?' active':''}}" data-model="${{m.id}}" style="--tone:${{m.color}}">${{m.id}}</button>`).join('');
  $('#matchChips').onclick=e=>{{const b=e.target.closest('[data-model]');if(b)selectModel(b.dataset.model);}};
  $('#replayChips').innerHTML=(M.trajectories||[]).map((p,i)=>`<button type="button" class="replay-chip${{i===0?' active':''}}" data-replay="${{i}}">battle ${{p.battleId}}</button>`).join('');
  $('#replayChips').onclick=e=>{{const b=e.target.closest('[data-replay]');if(b)setReplay(Number(b.dataset.replay));}};
  $('#replayScrub').oninput=()=>drawArenas(Number($('#replayScrub').value));
  $('#replayPlay').onclick=()=>{{
    if(playTimer){{clearInterval(playTimer);playTimer=null;$('#replayPlay').textContent='▶ play';return;}}
    $('#replayPlay').textContent='❚❚';
    let i=1; const max=Number($('#replayScrub').max);
    playTimer=setInterval(()=>{{
      $('#replayScrub').value=i; drawArenas(i); i+=1;
      if(i>max){{clearInterval(playTimer);playTimer=null;$('#replayPlay').textContent='▶ play';}}
    }},130);
  }};

  drawLineage();
  paintKpis(byId[activeId],1);
  drawDelta(byId[activeId],1);
  if(M.trajectories&&M.trajectories.length)setReplay(0);
}})();
</script></body></html>"""

    output = Path(output_path)
    if not output.is_absolute():
        output = ROOT / output
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(html, encoding="utf-8")
    return output


if __name__ == "__main__":
    print(render_policy_v43_report())
