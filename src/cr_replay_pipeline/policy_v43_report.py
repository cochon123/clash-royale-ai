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
    }

    html = f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>PolicyBC v4.3 — think-loop training report</title>
<style>
:root{{--bg:#07100e;--panel:#0d1916;--line:#20332d;--text:#edf7f2;--muted:#91a9a0;--green:#70e1b1;--gold:#ffca63;--blue:#70a1ff;--red:#ff7e78;--star:#e8f58b;--ink:#06100d}}
*{{box-sizing:border-box}}html{{scroll-behavior:smooth}}body{{margin:0;background:radial-gradient(circle at 10% -8%,#1d3f32 0,transparent 32%),radial-gradient(circle at 90% 4%,#3a3418 0,transparent 26%),var(--bg);color:var(--text);font:15px/1.55 "IBM Plex Sans",ui-sans-serif,system-ui,sans-serif}}
body:before{{content:"";position:fixed;inset:0;pointer-events:none;opacity:.05;background-image:linear-gradient(#fff 1px,transparent 1px),linear-gradient(90deg,#fff 1px,transparent 1px);background-size:42px 42px;mask-image:linear-gradient(to bottom,#000,transparent 72%)}}
main{{width:min(1220px,calc(100% - 36px));margin:auto;padding:52px 0 90px}}
.eyebrow{{font:700 11px/1.2 "IBM Plex Mono",ui-monospace,monospace;letter-spacing:.16em;text-transform:uppercase;color:var(--star)}}
h1{{font-size:clamp(42px,7.6vw,84px);line-height:.92;letter-spacing:-.065em;margin:18px 0 22px;max-width:980px}}
h1 em{{color:var(--star);font-style:normal}}h2{{font-size:clamp(25px,4vw,40px);letter-spacing:-.04em;margin:0 0 10px}}h3{{font-size:17px;margin:0 0 8px}}
p{{color:var(--muted);margin:0}}.lede{{font-size:clamp(17px,2.1vw,22px);max-width:860px;color:#bfd0ca}}
.hero{{padding:18px 0 48px}}.stamp{{margin-top:20px;color:#607d72;font:12px "IBM Plex Mono",ui-monospace,monospace}}
.verdict{{margin-top:38px;border:1px solid #5d6b34;background:linear-gradient(135deg,rgba(232,245,139,.12),rgba(112,225,177,.05));border-radius:22px;padding:24px;display:grid;grid-template-columns:auto 1fr;gap:18px;align-items:start}}
.verdict .mark{{width:50px;height:50px;border-radius:14px;display:grid;place-items:center;background:var(--star);color:var(--ink);font-size:24px;font-weight:900}}
.verdict strong{{display:block;font-size:20px;margin-bottom:5px}}.verdict p{{max-width:880px}}
.section{{border-top:1px solid var(--line);padding:52px 0}}.section-head{{display:flex;justify-content:space-between;align-items:end;gap:20px;margin-bottom:26px}}.section-head p{{max-width:560px}}
.kpis{{display:grid;grid-template-columns:repeat(4,1fr);gap:12px}}
.kpi,.card{{border:1px solid var(--line);background:linear-gradient(160deg,rgba(17,33,29,.94),rgba(8,17,14,.94));border-radius:18px;padding:20px;position:relative;overflow:hidden}}
.kpi:after{{content:"";position:absolute;width:100px;height:100px;border-radius:50%;background:var(--tone,var(--star));filter:blur(55px);opacity:.12;right:-30px;top:-35px}}
.kpi span{{display:block;color:var(--muted);font-size:12px;text-transform:uppercase;letter-spacing:.08em}}
.kpi b{{display:block;font-size:clamp(24px,3.4vw,38px);letter-spacing:-.045em;margin:6px 0 2px}}.kpi small{{color:#789188}}
.model-grid{{display:grid;grid-template-columns:repeat(4,1fr);gap:12px}}
.model-card{{border-top:3px solid var(--tone,var(--blue))}}
.model-tag{{display:inline-flex;align-items:center;gap:7px;font:700 12px "IBM Plex Mono",ui-monospace,monospace;color:var(--tone)}}
.dot{{width:8px;height:8px;background:var(--tone);border-radius:50%;box-shadow:0 0 14px var(--tone)}}
.big-number{{font-size:30px;font-weight:750;letter-spacing:-.05em;margin:14px 0 2px}}
.facts{{display:grid;grid-template-columns:1fr 1fr;gap:9px;margin-top:16px;padding-top:14px;border-top:1px solid var(--line)}}
.fact b,.fact span{{display:block}}.fact b{{font-size:13px}}.fact span{{font-size:10px;color:var(--muted);text-transform:uppercase;letter-spacing:.06em}}
.metric-table{{width:100%;border-collapse:separate;border-spacing:0}}
.metric-table th,.metric-table td{{padding:13px 14px;border-bottom:1px solid var(--line);text-align:right}}
.metric-table th:first-child,.metric-table td:first-child{{text-align:left}}
.metric-table thead th{{font:700 11px "IBM Plex Mono",ui-monospace,monospace;text-transform:uppercase;letter-spacing:.08em;color:var(--muted)}}
.metric-table tbody tr:hover{{background:#13231e}}.winner{{color:var(--star);font-weight:750}}
.metric-note{{font-size:11px;color:#617d73;display:block}}
.star-col{{background:rgba(232,245,139,.045);border-left:1px solid #596331!important}}
.star-point{{margin-top:18px;border:1px solid #778143;background:radial-gradient(circle at 88% 0,rgba(232,245,139,.14),transparent 36%),linear-gradient(145deg,#162018,#0b1511);border-radius:18px;padding:22px;display:grid;grid-template-columns:56px 1fr;gap:18px}}
.star-icon{{width:56px;height:56px;border-radius:16px;display:grid;place-items:center;background:var(--star);color:#152014;font-size:28px;box-shadow:0 0 30px rgba(232,245,139,.18)}}
.star-stats{{display:grid;grid-template-columns:repeat(4,1fr);gap:10px;margin-top:16px}}
.star-stat{{padding:11px 13px;border:1px solid #33432d;border-radius:11px;background:#0a1410}}
.star-stat b,.star-stat span{{display:block}}.star-stat b{{color:var(--star);font:750 18px "IBM Plex Mono",ui-monospace,monospace}}.star-stat span{{color:var(--muted);font-size:11px}}
.arch{{display:grid;grid-template-columns:1.15fr .85fr;gap:14px}}
.flow{{display:grid;grid-template-columns:repeat(5,1fr);gap:8px;margin-top:8px}}
.flow-step{{border:1px solid var(--line);border-radius:14px;padding:14px 12px;background:#0a1512;min-height:118px}}
.flow-step b{{display:block;font:700 12px "IBM Plex Mono",ui-monospace,monospace;color:var(--star);margin-bottom:8px}}
.flow-step p{{font-size:12px;line-height:1.45}}.flow-step.active{{border-color:#778143;background:rgba(232,245,139,.06)}}
.toggle-demo{{display:grid;gap:10px;margin-top:14px}}
.toggle-row{{display:grid;grid-template-columns:90px 1fr auto;gap:12px;align-items:center;padding:12px;border:1px solid var(--line);border-radius:12px;background:#0a1512}}
.toggle-row.on{{border-color:#778143}}
.depth-bar{{height:10px;border-radius:99px;background:#08110f;overflow:hidden;border:1px solid #1c2d28}}
.depth-fill{{height:100%;width:0;background:linear-gradient(90deg,var(--green),var(--star));transition:width .8s cubic-bezier(.2,.8,.2,1)}}
.chart-card{{padding:22px}}.chart-tools{{display:flex;flex-wrap:wrap;align-items:center;gap:9px;margin:16px 0}}
button,.toggle{{border:1px solid var(--line);background:#0a1512;color:var(--muted);border-radius:10px;padding:8px 11px;font:600 12px "IBM Plex Mono",ui-monospace,monospace;cursor:pointer}}
button:hover,.toggle:hover{{border-color:#416459;color:var(--text)}}button.active{{background:var(--star);color:var(--ink);border-color:var(--star)}}
.toggle input{{accent-color:var(--tone);vertical-align:-2px}}
#curve{{width:100%;height:390px;display:block}}
.gridline{{stroke:#20332d;stroke-width:1}}.axis-label{{fill:#668078;font:11px "IBM Plex Mono",ui-monospace,monospace}}
.curve{{fill:none;stroke-width:3;stroke-linecap:round;stroke-linejoin:round}}.point{{cursor:crosshair}}
.ref-line{{stroke:#e8f58b;stroke-width:1.5;stroke-dasharray:4 5;opacity:.85}}.ref-point{{fill:#e8f58b;stroke:#152014;stroke-width:3}}
.tooltip{{position:fixed;pointer-events:none;z-index:9;background:#edf7f2;color:#07100e;border-radius:10px;padding:8px 10px;font:12px "IBM Plex Mono",ui-monospace,monospace;box-shadow:0 8px 28px #0008;opacity:0;transform:translate(-50%,-120%)}}
.bars{{display:grid;gap:15px;margin-top:18px}}.bar-row{{display:grid;grid-template-columns:90px 1fr 88px;gap:12px;align-items:center}}
.bar-track{{height:13px;border-radius:99px;background:#08110f;overflow:hidden;border:1px solid #1c2d28}}
.bar-fill{{height:100%;width:0;border-radius:99px;background:var(--tone);transition:width 1s cubic-bezier(.2,.8,.2,1)}}
.bar-value{{font:700 12px "IBM Plex Mono",ui-monospace,monospace;text-align:right}}
.compare-grid{{display:grid;grid-template-columns:1fr 1fr;gap:14px}}
.delta-list{{display:grid;gap:10px;margin-top:14px}}
.delta-item{{display:grid;grid-template-columns:1fr auto;gap:10px;padding:12px 14px;border:1px solid var(--line);border-radius:12px;background:#0a1512}}
.delta-item b{{font:750 16px "IBM Plex Mono",ui-monospace,monospace}}.up{{color:var(--green)}}.down{{color:var(--red)}}
.lessons{{display:grid;grid-template-columns:repeat(3,1fr);gap:13px}}
.lesson .n{{font:800 12px "IBM Plex Mono",ui-monospace,monospace;color:var(--star)}}.lesson h3{{margin:14px 0 7px}}.lesson p{{font-size:13px}}
.callout{{border-left:3px solid var(--gold);padding:15px 18px;background:#19180f;border-radius:0 12px 12px 0;margin-top:18px;color:#d7cfaf}}
.expr{{margin-top:14px;padding:14px 16px;border:1px dashed #355247;border-radius:12px;background:#0a1512;font:13px/1.55 "IBM Plex Mono",ui-monospace,monospace;color:#c5ddd2}}
.expr b{{color:var(--star)}}
.foot{{padding-top:26px;color:#557067;font:11px "IBM Plex Mono",ui-monospace,monospace}}
.pill{{display:inline-block;padding:4px 8px;border:1px solid var(--line);border-radius:99px;color:var(--muted);font:11px "IBM Plex Mono",ui-monospace,monospace;margin-right:6px}}
@media(max-width:980px){{.model-grid,.kpis{{grid-template-columns:1fr 1fr}}.arch,.compare-grid,.lessons{{grid-template-columns:1fr}}.flow{{grid-template-columns:1fr 1fr}}}}
@media(max-width:560px){{main{{width:min(100% - 22px,1220px)}}.kpis,.model-grid,.star-stats,.flow{{grid-template-columns:1fr}}.verdict,.star-point{{grid-template-columns:1fr}}}}
</style>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@500;700&family=IBM+Plex+Sans:wght@400;600;700&display=swap" rel="stylesheet">
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

<section class="section">
  <div class="section-head"><div><div class="eyebrow">08 · lessons</div><h2>What v4.3 taught us</h2></div><p>Capacity finally moved the needle after v4.2 proved the data recipe.</p></div>
  <div class="lessons">
    <div class="card lesson"><div class="n">01 / CAPACITY</div><h3>A 3.9× larger trunk helped.</h3><p>Moving from 633k to 2.47M parameters on more data improved every primary offline action metric versus full-data v4.2, without blowing the 6GB laptop budget (peak ~{peak_vram:.0f} MB).</p></div>
    <div class="card lesson"><div class="n">02 / THINKING</div><h3>Train random depth; toggle at serve time.</h3><p>Sampling K during training keeps both the fast path and the deep path competent. At inference, <code>--think-steps</code> is just a compute dial.</p></div>
    <div class="card lesson"><div class="n">03 / NEXT</div><h3>Pair think with mirror TTA carefully.</h3><p>Mirror TTA and think steps multiply latency. The next clean experiment is a frozen-manifest bake-off of v4.2★ / v4.3 off / v4.3★ before spending live phone time.</p></div>
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
    out+=`<path class="curve" stroke="${{m.color}}" d="${{pts.map((q,i)=>(i?'L':'M')+q[0].toFixed(1)+','+q[1].toFixed(1)).join(' ')}}"/>`;
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
</script></body></html>"""

    output = Path(output_path)
    if not output.is_absolute():
        output = ROOT / output
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(html, encoding="utf-8")
    return output


if __name__ == "__main__":
    print(render_policy_v43_report())
