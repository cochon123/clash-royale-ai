"""Interactive v4.4 policy training report.

Emphasizes discrete 18×32 tile classification versus continuous XY regression.
Self-contained native HTML/SVG/JS — no matplotlib.
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
    if compute.get("mirror_training") and not compute.get("lazy_mirror_training") and len(splits) >= 3:
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
        "trainSamples": train_samples,
        "testSamples": int(report["data"]["test_samples"]),
        "epochs": epochs,
        "batch": batch,
        "parameters": int(compute.get("parameters") or 0),
        "dModel": int(compute.get("d_model") or 0),
        "numLayers": int(compute.get("num_layers") or 0),
        "maxThinkSteps": int(compute.get("max_think_steps") or 0),
        "placementMode": str(compute.get("placement_mode") or "xy"),
        "mirrored": bool(compute.get("mirror_training", False)),
        "test": report["test"],
        "val": report["val"],
        "history": history,
    }


def render_policy_v44_report(
    output_path: str | Path = "reports/policy_bc_v4_4.html",
    model_dir: str | Path = "models/policy_bc_v4.4",
) -> Path:
    specs = [
        ("v4.2", ROOT / "models/policy_bc_v4.2_full", "#70e1b1"),
        ("v4.3", ROOT / "models/policy_bc_v4.3", "#e8f58b"),
        ("v4.4", Path(model_dir) if Path(model_dir).is_absolute() else ROOT / model_dir, "#38bdf8"),
    ]
    models: list[dict[str, Any]] = []
    for label, path, color in specs:
        report = _load(path / "report.json")
        history_path = path / "training_stages.json"
        history = _load(history_path) if history_path.exists() else report.get("history", [])
        models.append(_model_payload(label, report, history, color))

    v44_path = Path(model_dir) if Path(model_dir).is_absolute() else ROOT / model_dir
    v44_report = _load(v44_path / "report.json")
    think_off = v44_report.get("test_think_off") or v44_report["test"]
    think_on = v44_report["test"]
    v44 = next(m for m in models if m["id"] == "v4.4")
    v43 = next(m for m in models if m["id"] == "v4.3")
    v42 = next(m for m in models if m["id"] == "v4.2")

    probe_path = ROOT / "reports" / "policy_bc_v4_4_tile_probe.json"
    probe = _load(probe_path) if probe_path.exists() else {}
    triple_path = ROOT / "reports" / "policy_bc_v4_4_triple_replay.json"
    triple = _load(triple_path) if triple_path.exists() else None

    progress_path = Path(v44_report["compute"].get("progress_path") or "")
    if not progress_path.is_absolute():
        progress_path = ROOT / progress_path
    peak_vram = 0.0
    if progress_path.exists():
        for line in progress_path.read_text(encoding="utf-8").splitlines():
            if not line.strip():
                continue
            row = json.loads(line)
            peak_vram = max(peak_vram, float(row.get("gpu_memory_mb") or 0.0))

    created = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
    tile_cls = float(think_on.get("tile_class_acc") or 0.0)
    tile_top5 = float(think_on.get("tile_top5_acc") or 0.0)
    soft_tile = float(think_on.get("tile_acc") or 0.0)
    chance_tile = 1.0 / 576.0

    payload = {
        "models": models,
        "probe": probe,
        "tripleReplay": triple,
        "think": {
            "on": {"id": "v4.4 ★", "label": "think K=3", "color": "#38bdf8", "test": think_on},
            "off": {"id": "v4.4 off", "label": "think K=0", "color": "#7aa0b0", "test": think_off},
            "maxSteps": v44["maxThinkSteps"],
        },
        "meta": {
            "gpu": "NVIDIA GeForce RTX 3050 6GB Laptop",
            "peakVramMb": peak_vram,
            "duration": v44["duration"],
            "created": v44["created"],
            "parameters": v44["parameters"],
            "tiles": 576,
            "rows": 18,
            "cols": 32,
            "chanceTile": chance_tile,
        },
    }

    html = f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>PolicyBC v4.4 — tile heatmap training report</title>
{favicon_link()}
{FONT_LINKS}
<style>
{shared_styles()}
.illus {{width:100%;height:auto;display:block;border-radius:14px;background:#08131a}}
.star-col {{background:rgba(56,189,248,.05);border-left:1px solid #2f6f8a!important}}
.mode-label {{font:700 12px "IBM Plex Mono",ui-monospace,monospace;letter-spacing:.08em;text-transform:uppercase;margin-bottom:8px}}
.mode-bad {{color:var(--red)}}.mode-good {{color:var(--sky)}}
.triple-grid {{display:grid;grid-template-columns:repeat(3,1fr);gap:12px;margin-top:8px}}
.replay-pane h3 {{display:flex;justify-content:space-between;align-items:baseline;gap:8px;margin-bottom:10px;font-size:15px}}
.pane-score {{font:700 11px "IBM Plex Mono",ui-monospace,monospace;color:var(--muted)}}
.chart-card {{padding:22px}}
@media(max-width:980px){{.triple-grid{{grid-template-columns:1fr}}}}
</style>
</head>
<body><main>
<header class="hero">
  <div class="eyebrow">Clash Royale AI · placement experiment</div>
  <h1>Stop averaging.<br><em>Classify the tile.</em></h1>
  <p class="lede">v4.4 keeps the v4.2 trunk size and the v4.3 data recipe, but replaces continuous XY regression with a card-conditioned 18×32 heatmap. Placement becomes a 576-way classification problem so the model can pick a mode instead of the river between two good lanes.</p>
  <div class="stamp">REPORT GENERATED {created} · MODEL policy-bc-v4.4 · VERSION 4.4.0 · TRAINED {v44['created']} · PLACEMENT heatmap</div>
  <div class="verdict"><div class="mark">▦</div><div>
    <strong>Exact tile top-1 is {tile_cls*100:.1f}% ({tile_cls/chance_tile:.0f}× chance) · top-5 is {tile_top5*100:.1f}%.</strong>
    <p>Soft within-1-tile MAE accuracy is {soft_tile*100:.1f}% — slightly above v4.3's {v43['test']['tile_acc']*100:.1f}% — but the real story is discrete tile class accuracy, which XY models never trained. Think dial is K∈0…3.</p>
  </div></div>
</header>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">01 · why classification</div><h2>Mean collapse vs tile modes</h2></div><p>If humans place left-bridge or right-bridge, regressing XY learns the midpoint. Softmax over tiles can keep both modes.</p></div>
  <div class="grid2">
    <div class="card">
      <div class="mode-label mode-bad">Continuous XY · mean collapse</div>
      <svg class="illus" id="meanIllus" viewBox="0 0 520 300" role="img" aria-label="Mean collapse illustration"></svg>
      <p style="margin-top:12px">Two human clusters → one predicted point in the river. That point is rarely legal or useful.</p>
    </div>
    <div class="card">
      <div class="mode-label mode-good">Heatmap · pick a cell</div>
      <svg class="illus" id="heatIllus" viewBox="0 0 520 300" role="img" aria-label="Tile heatmap illustration"></svg>
      <p style="margin-top:12px">576 logits → sample or argmax a tile center. Multimodal placements stay multimodal.</p>
    </div>
  </div>
  <div class="expr"><b>grid:</b> {payload['meta']['rows']} × {payload['meta']['cols']} = {payload['meta']['tiles']} tiles &nbsp;·&nbsp; <b>chance top-1:</b> 1/576 ≈ {chance_tile*100:.2f}% &nbsp;·&nbsp; <b>loss:</b> tile CE (w={v44_report['compute']['loss_kwargs']['tile_weight']}) + zone + slot · xy_weight=0</div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">02 · headline metrics</div><h2>Tile accuracy is the new scoreboard</h2></div><p>v4.4 test set · {v44['testSamples']:,} labels. Soft tile = predicted point within 1,000 API units of human.</p></div>
  <div class="kpis">
    <div class="kpi" style="--tone:var(--sky)"><span>Exact tile top-1</span><b>{tile_cls*100:.1f}%</b><small>{tile_cls/chance_tile:.0f}× random · n={think_on['n']:,}</small></div>
    <div class="kpi" style="--tone:var(--green)"><span>Tile top-5</span><b>{tile_top5*100:.1f}%</b><small>human tile in top 5 logits</small></div>
    <div class="kpi" style="--tone:var(--gold)"><span>Soft within-tile</span><b>{soft_tile*100:.1f}%</b><small>v4.3 was {v43['test']['tile_acc']*100:.1f}%</small></div>
    <div class="kpi" style="--tone:var(--blue)"><span>Parameters</span><b>{_fmt_compact(v44['parameters'])}</b><small>d={v44['dModel']} · L={v44['numLayers']} · K≤{v44['maxThinkSteps']}</small></div>
  </div>
  <div class="card" style="margin-top:16px">
    <h3>Lineage placement comparison</h3>
    <div class="bars" id="tileBars"></div>
    <div class="callout"><b>Read carefully:</b> exact tile top-1 / top-5 exist only for heatmap models. For XY models the bars show soft within-1-tile accuracy from the continuous point (and optional snap-to-tile probe when available).</div>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">03 · arena grid</div><h2>What “one tile” looks like</h2></div><p>Coarse board used by the head. Hover a cell to see its index. This is the classification vocabulary — not continuous API units.</p></div>
  <div class="card">
    <svg class="illus" id="arenaGrid" viewBox="0 0 640 420" role="img" aria-label="18 by 32 tile arena grid"></svg>
    <div class="expr" id="tileReadout">hover a tile · index = row×32 + col · center maps back to normalized XY</div>
  </div>
</section>

<section class="section" id="tripleReplaySection">
  <div class="section-head"><div><div class="eyebrow">04 · same prefix, three futures</div><h2>Human vs v4.3 vs v4.4</h2></div><p>Grey dots are the shared warm-up. After the cut: human continuation, continuous-XY v4.3, and tile-argmax v4.4. Four held-out prefixes.</p></div>
  <div class="card chart-card">
    <div class="replay-chips" id="tripleChips"></div>
    <div class="triple-grid">
      <div class="replay-pane">
        <h3>Human <span class="pane-score" id="hScore"></span></h3>
        <svg class="arena" id="hArena"></svg>
      </div>
      <div class="replay-pane">
        <h3>AI · v4.3 <span class="pane-score" id="a43Score"></span></h3>
        <svg class="arena" id="a43Arena"></svg>
      </div>
      <div class="replay-pane">
        <h3>AI · v4.4 tiles <span class="pane-score" id="a44Score"></span></h3>
        <svg class="arena" id="a44Arena"></svg>
      </div>
    </div>
    <div class="anim-toolbar">
      <button type="button" id="triplePlay">▶ play</button>
      <input type="range" class="anim-scrubber" id="tripleScrub" min="1" max="1" value="1" step="1">
      <div class="anim-readout" id="tripleReadout"></div>
    </div>
    <div class="replay-log" id="tripleLog"></div>
    <div class="expr" style="margin-top:12px"><b>decode:</b> v4.3 = continuous XY (style rollout cache) · v4.4 = argmax over 576 tiles at K=3 · warm-up = first 12 events</div>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">05 · held-out test</div><h2>Action metrics still matter</h2></div><p>Heatmap should not wreck card/zone/timing. ★ is think K=3; off is the same checkpoint at K=0.</p></div>
  <div class="card" style="padding:4px 0;overflow:auto">
    <table class="metric-table" id="metricTable"></table>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">06 · training curves</div><h2>Tile class climbs with the trunk</h2></div><p>Validation curves. Toggle metrics; the dashed reference is v4.4 think-on test.</p></div>
  <div class="card chart-card">
    <div class="chart-tools" id="metricButtons"></div>
    <button id="play">replay curves</button>
    <svg id="curve" viewBox="0 0 1040 360" role="img" aria-label="Training curves"></svg>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">07 · recipe</div><h2>How this run was built</h2></div><p>Comparable trunk to v4.2, data scale of v4.3, think dial capped at 3 for a fast test.</p></div>
  <div class="kpis">
    <div class="kpi" style="--tone:var(--sky)"><span>Source battles</span><b>{v44['sourceBattles']:,}</b><small>mirrored train</small></div>
    <div class="kpi" style="--tone:var(--green)"><span>Train samples</span><b>{_fmt_compact(v44['trainSamples'])}</b><small>/ epoch · cap 40</small></div>
    <div class="kpi" style="--tone:var(--gold)"><span>Wall time</span><b>{v44['duration']}</b><small>peak VRAM {peak_vram:.0f} MB</small></div>
    <div class="kpi" style="--tone:var(--blue)"><span>Batch × epochs</span><b>512 × 10</b><small>think max K=3</small></div>
  </div>
  <div class="expr" style="margin-top:18px">
    cr-replays train-policy --version 4.4 --d-model 160 --num-layers 2<br>
    --max-think-steps 3 --eval-think-steps 3 --mirror-training<br>
    --batch-size 512 --epochs 10 --max-samples-per-battle 40
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">08 · lessons</div><h2>What the heatmap test showed</h2></div><p>A small trunk can learn discrete placement modes when the loss asks for them.</p></div>
  <div class="grid3">
    <div class="card lesson"><div class="n">01 / SIGNAL</div><h3>Tile CE is learnable offline.</h3><p>Top-1 at {tile_cls*100:.1f}% and top-5 at {tile_top5*100:.1f}% on a 576-way head means the model is not guessing randomly — the board structure is in the replays.</p></div>
    <div class="card lesson"><div class="n">02 / TRADE</div><h3>Card metrics stayed in family.</h3><p>Slot top-1 {think_on['slot_top1']*100:.2f}% sits next to v4.3 ({v43['test']['slot_top1']*100:.2f}%) with a much smaller trunk. Zone {think_on['zone_acc']*100:.2f}% is the best of this trio.</p></div>
    <div class="card lesson"><div class="n">03 / NEXT</div><h3>Decode choice is the dial.</h3><p>Train with CE; at serve time try argmax vs temperature sample. Pair with style matchup and matchup-lineage before growing K or trunk size.</p></div>
  </div>
</section>

<footer class="foot">Native HTML + SVG + JS · no matplotlib · heatmap placement · think-on K=3 is checkpoint selection · not a live Clash Royale readiness claim.</footer>
</main>
<div class="tooltip" id="tip"></div>
<script id="report-data" type="application/json">{_js(payload)}</script>
<script>
const D=JSON.parse(document.getElementById('report-data').textContent);
const $=s=>document.querySelector(s);
const pct=n=>(100*n).toFixed(2)+'%';
const fmt=n=>new Intl.NumberFormat('en-US').format(Math.round(n));
const compact=n=>n>=1e6?(n/1e6).toFixed(2)+'m':n>=1e3?(n/1e3).toFixed(1)+'k':fmt(n);

/* --- mean collapse illustration --- */
(function(){{
  const svg=$('#meanIllus');
  const W=520,H=300, ox=40, oy=28, aw=440, ah=240;
  let t=0;
  function frame(){{
    t=(t+0.016)%4;
    const phase=Math.min(1, Math.max(0,(t-1.2)/1.6));
    const e=1-Math.pow(1-phase,3);
    const left={{x:ox+aw*0.28,y:oy+ah*0.42}}, right={{x:ox+aw*0.72,y:oy+ah*0.42}};
    const mean={{x:(left.x+right.x)/2, y:oy+ah*0.5}};
    let out=`<rect x="${{ox}}" y="${{oy}}" width="${{aw}}" height="${{ah}}" rx="14" fill="#0d1a2b"/>
      <rect x="${{ox}}" y="${{oy}}" width="${{aw}}" height="${{ah*0.5}}" fill="#991b1b" opacity=".25"/>
      <rect x="${{ox}}" y="${{oy+ah*0.465}}" width="${{aw}}" height="${{ah*0.07}}" fill="#1d4ed8" opacity=".45"/>
      <text x="${{ox+12}}" y="${{oy+18}}" fill="#fca5a5" font-size="11" font-family="IBM Plex Mono,monospace">ENEMY</text>
      <text x="${{ox+12}}" y="${{oy+ah-8}}" fill="#93c5fd" font-size="11" font-family="IBM Plex Mono,monospace">YOU</text>`;
    // human clouds
    for(const c of [left,right]){{
      out+=`<circle cx="${{c.x}}" cy="${{c.y}}" r="34" fill="#70e1b1" opacity=".12"/>
        <circle cx="${{c.x-10}}" cy="${{c.y+6}}" r="5" fill="#70e1b1" opacity=".85"/>
        <circle cx="${{c.x+8}}" cy="${{c.y-8}}" r="5" fill="#70e1b1" opacity=".75"/>
        <circle cx="${{c.x+4}}" cy="${{c.y+10}}" r="4" fill="#70e1b1" opacity=".65"/>`;
    }}
    out+=`<text x="${{left.x}}" y="${{left.y-44}}" text-anchor="middle" fill="#9bb6ad" font-size="12">human left</text>
      <text x="${{right.x}}" y="${{right.y-44}}" text-anchor="middle" fill="#9bb6ad" font-size="12">human right</text>`;
    const px=left.x+(mean.x-left.x)*e, py=left.y+(mean.y-left.y)*e;
    const qx=right.x+(mean.x-right.x)*e, qy=right.y+(mean.y-right.y)*e;
    out+=`<line x1="${{left.x}}" y1="${{left.y}}" x2="${{px}}" y2="${{py}}" stroke="#ff7e78" stroke-dasharray="4 4" opacity=".7"/>
      <line x1="${{right.x}}" y1="${{right.y}}" x2="${{qx}}" y2="${{qy}}" stroke="#ff7e78" stroke-dasharray="4 4" opacity=".7"/>
      <circle cx="${{mean.x}}" cy="${{mean.y}}" r="${{6+4*e}}" fill="#ff7e78" opacity="${{.35+.5*e}}"/>
      <text x="${{mean.x}}" y="${{mean.y+28}}" text-anchor="middle" fill="#ff7e78" font-size="12" font-family="IBM Plex Mono,monospace" opacity="${{e}}">predicted mean</text>`;
    svg.innerHTML=out;
    requestAnimationFrame(frame);
  }}
  frame();
}})();

/* --- heatmap illustration --- */
(function(){{
  const svg=$('#heatIllus');
  const W=520,H=300, ox=40, oy=28, aw=440, ah=240;
  const cols=16, rows=12;
  const modes=[{{c:4,r:5,p:.22}},{{c:11,r:5,p:.2}},{{c:4,r:6,p:.08}},{{c:12,r:6,p:.07}},{{c:8,r:3,p:.03}}];
  function cell(c,r){{
    let p=0.008;
    for(const m of modes){{
      const d=Math.hypot(c-m.c,r-m.r);
      p+=m.p*Math.exp(-d*d/1.8);
    }}
    return p;
  }}
  let peak=0; for(let r=0;r<rows;r++)for(let c=0;c<cols;c++)peak=Math.max(peak,cell(c,r));
  let out=`<rect x="${{ox}}" y="${{oy}}" width="${{aw}}" height="${{ah}}" rx="14" fill="#0d1a2b"/>
    <rect x="${{ox}}" y="${{oy+ah*0.465}}" width="${{aw}}" height="${{ah*0.07}}" fill="#1d4ed8" opacity=".35"/>`;
  const cw=aw/cols, ch=ah/rows;
  for(let r=0;r<rows;r++)for(let c=0;c<cols;c++){{
    const a=cell(c,r)/peak;
    out+=`<rect x="${{ox+c*cw+1}}" y="${{oy+r*ch+1}}" width="${{cw-2}}" height="${{ch-2}}" rx="2"
      fill="#38bdf8" opacity="${{(0.05+0.85*a).toFixed(3)}}"/>`;
  }}
  // argmax marker
  let best={{c:4,r:5,p:0}};
  for(let r=0;r<rows;r++)for(let c=0;c<cols;c++){{const p=cell(c,r); if(p>best.p)best={{c,r,p}};}}
  out+=`<rect x="${{ox+best.c*cw}}" y="${{oy+best.r*ch}}" width="${{cw}}" height="${{ch}}" fill="none" stroke="#e8f58b" stroke-width="2.5"/>
    <text x="${{ox+12}}" y="${{oy+ah-10}}" fill="#e8f58b" font-size="12" font-family="IBM Plex Mono,monospace">argmax tile</text>`;
  svg.innerHTML=out;
}})();

/* --- interactive arena grid --- */
(function(){{
  const svg=$('#arenaGrid');
  const rows=D.meta.rows, cols=D.meta.cols;
  const W=640,H=420, ox=48, oy=36, aw=544, ah=340;
  const cw=aw/cols, ch=ah/rows;
  let out=`<rect x="${{ox}}" y="${{oy}}" width="${{aw}}" height="${{ah}}" rx="12" fill="#0d1a2b"/>
    <rect x="${{ox}}" y="${{oy}}" width="${{aw}}" height="${{ah*0.5}}" fill="#991b1b" opacity=".22"/>
    <rect x="${{ox}}" y="${{oy+ah*0.465}}" width="${{aw}}" height="${{ah*0.07}}" fill="#1d4ed8" opacity=".4"/>
    <text x="${{ox}}" y="22" fill="#fca5a5" font-size="12" font-family="IBM Plex Mono,monospace" letter-spacing="0.12em">ENEMY SIDE</text>
    <text x="${{ox}}" y="${{H-8}}" fill="#93c5fd" font-size="12" font-family="IBM Plex Mono,monospace" letter-spacing="0.12em">YOUR SIDE</text>`;
  for(let r=0;r<rows;r++)for(let c=0;c<cols;c++){{
    const idx=r*cols+c;
    out+=`<rect class="tile" data-r="${{r}}" data-c="${{c}}" data-i="${{idx}}" x="${{ox+c*cw}}" y="${{oy+r*ch}}" width="${{cw}}" height="${{ch}}"
      fill="transparent" stroke="rgba(148,163,184,0.18)" stroke-width="0.6"/>`;
  }}
  svg.innerHTML=out;
  const tip=$('#tileReadout');
  svg.querySelectorAll('.tile').forEach(el=>{{
    el.addEventListener('mouseenter',()=>{{
      el.setAttribute('fill','rgba(56,189,248,0.35)');
      const r=+el.dataset.r,c=+el.dataset.c,i=+el.dataset.i;
      const nx=((c+0.5)/cols).toFixed(3), ny=((r+0.5)/rows).toFixed(3);
      tip.innerHTML=`tile <b>${{i}}</b> · row ${{r}} col ${{c}} · center (x,y)=(${{nx}}, ${{ny}}) → API (~${{Math.round(nx*18000)}}, ~${{Math.round(ny*32000)}})`;
    }});
    el.addEventListener('mouseleave',()=>el.setAttribute('fill','transparent'));
  }});
}})();

/* --- comparison bars --- */
(function(){{
  const host=$('#tileBars');
  const rows=[
    {{key:'soft', label:'Soft ≤1 tile', note:'continuous point within 1000 API units'}},
    {{key:'snap', label:'Exact tile / snap', note:'heatmap argmax · XY models snap continuous XY to tile'}},
    {{key:'top5', label:'Tile top-5 / neighborhood', note:'heatmap top-5 · XY models: Chebyshev ≤1 neighbor'}},
  ];
  const order=['v4.2','v4.3','v4.4'];
  function val(id, key){{
    const m=D.models.find(x=>x.id===id);
    const p=D.probe[id]||{{}};
    if(key==='soft') return m.test.tile_acc||0;
    if(key==='snap') return p.snap_tile_acc!=null?p.snap_tile_acc:(m.test.tile_class_acc||0);
    if(key==='top5') return p.neighbor_or_top5!=null?p.neighbor_or_top5:(m.test.tile_top5_acc||0);
    return 0;
  }}
  host.innerHTML=rows.map(row=>{{
    const vals=order.map(id=>val(id,row.key));
    const max=Math.max(...vals,0.01);
    return `<div style="margin-bottom:18px"><div style="display:flex;justify-content:space-between;gap:12px;margin-bottom:8px"><b>${{row.label}}</b><span class="chip">${{row.note}}</span></div>`+
      order.map((id,i)=>{{
        const m=D.models.find(x=>x.id===id);
        const v=vals[i];
        return `<div class="bar-row"><b style="color:${{m.color}}">${{id}}</b><div class="bar-track"><div class="bar-fill" style="--tone:${{m.color}}" data-width="${{100*v/max}}"></div></div><div class="bar-value">${{pct(v)}}</div></div>`;
      }}).join('')+'</div>';
  }}).join('');
  requestAnimationFrame(()=>host.querySelectorAll('.bar-fill').forEach(b=>b.style.width=b.dataset.width+'%'));
}})();

/* --- metric table --- */
(function(){{
  const metrics=[
    ['Card top-1','slot_top1','pct',true,'Correct card slot'],
    ['Zone accuracy','zone_acc','pct',true,'12-way region'],
    ['Exact tile top-1','tile_class_acc','pct',true,'576-way class · heatmap only'],
    ['Tile top-5','tile_top5_acc','pct',true,'Human tile in top 5'],
    ['Soft within-tile','tile_acc','pct',true,'≤1,000 API units'],
    ['Placement MAE','xy_mae','int',false,'API units · lower better'],
    ['Timing MAE','timing_mae','sec',false,'Seconds · lower better'],
  ];
  const variants=[D.models.find(m=>m.id==='v4.2'), D.models.find(m=>m.id==='v4.3'), D.think.off, D.think.on];
  const format=(v,t)=>v==null||v===undefined?'—':t==='pct'?pct(v):t==='sec'?v.toFixed(3)+'s':t==='int'?fmt(v):v.toFixed(3);
  $('#metricTable').innerHTML='<thead><tr><th>Metric</th>'+variants.map(m=>`<th class="${{m.id.includes('★')?'star-col':''}}" style="color:${{m.color}}">${{m.id}}</th>`).join('')+'</tr></thead><tbody>'+
    metrics.map(([label,key,type,higher,note])=>{{
      const vals=variants.map(m=>m.test[key]);
      const finite=vals.filter(v=>v!=null&&Number.isFinite(v));
      const best=finite.length?(higher?Math.max(...finite):Math.min(...finite)):null;
      return `<tr><td><b>${{label}}</b><span class="metric-note">${{note}}</span></td>`+
        vals.map((v,i)=>`<td class="${{v===best&&v!=null?'winner ':''}}${{variants[i].id.includes('★')?'star-col':''}}">${{format(v,type)}}</td>`).join('')+
        '</tr>';
    }}).join('')+'</tbody>';
}})();

/* --- curves --- */
(function(){{
  const curveMetrics={{
    val_tile_class_acc:['Val exact tile top-1',true],
    val_tile_top5_acc:['Val tile top-5',true],
    val_tile_acc:['Val soft within-tile',true],
    val_slot_top1:['Val card top-1',true],
    val_zone_acc:['Val zone accuracy',true],
    val_loss:['Validation loss',false],
  }};
  let active='val_tile_class_acc', reveal=1;
  const m44=D.models.find(m=>m.id==='v4.4');
  $('#metricButtons').innerHTML=Object.entries(curveMetrics).map(([k,v])=>`<button data-metric="${{k}}" class="${{k===active?'active':''}}">${{v[0]}}</button>`).join('');
  const svg=$('#curve'), tip=$('#tip');
  function draw(){{
    const W=1040,H=360,p={{l:68,r:24,t:24,b:48}};
    const hist=m44.history.filter(r=>r[active]!=null);
    if(!hist.length){{svg.innerHTML='<text x="40" y="40" fill="#8aa3b3">No history for this metric</text>';return;}}
    const testKey=active.replace(/^val_/,'');
    const ref=D.think.on.test[testKey];
    const all=hist.map(r=>r[active]).concat(ref!=null?[ref]:[]);
    let lo=Math.min(...all), hi=Math.max(...all); const pad=(hi-lo||1)*.12; lo-=pad; hi+=pad;
    const x=e=>p.l+(e-1)/Math.max(hist.length-1,1)*(W-p.l-p.r);
    const y=v=>p.t+(hi-v)/(hi-lo)*(H-p.t-p.b);
    let out='';
    for(let i=0;i<5;i++){{const yy=p.t+i*(H-p.t-p.b)/4,v=hi-i*(hi-lo)/4;out+=`<line class="gridline" x1="${{p.l}}" y1="${{yy}}" x2="${{W-p.r}}" y2="${{yy}}"/><text class="axis-label" x="${{p.l-10}}" y="${{yy+4}}" text-anchor="end">${{active.includes('loss')?v.toFixed(2):pct(v)}}</text>`}}
    const rows=hist.slice(0,Math.ceil(hist.length*reveal));
    const pts=rows.map(r=>[x(r.epoch),y(r[active]),r]);
    out+=`<path class="curve" pathLength="1" stroke="#38bdf8" d="${{pts.map((q,i)=>(i?'L':'M')+q[0].toFixed(1)+','+q[1].toFixed(1)).join(' ')}}"/>`;
    out+=pts.map(q=>`<circle class="point" data-epoch="${{q[2].epoch}}" data-value="${{q[2][active]}}" fill="#38bdf8" cx="${{q[0]}}" cy="${{q[1]}}" r="4"/>`).join('');
    if(ref!=null){{const sy=y(ref); out+=`<line stroke="#e8f58b" stroke-dasharray="4 5" x1="${{p.l}}" y1="${{sy}}" x2="${{W-p.r}}" y2="${{sy}}"/><circle fill="#e8f58b" cx="${{W-p.r-8}}" cy="${{sy}}" r="6"/>`;}}
    svg.innerHTML=out;
    svg.querySelectorAll('.point').forEach(el=>{{
      el.onmouseenter=()=>{{tip.style.opacity=1;tip.innerHTML=`<b>epoch ${{el.dataset.epoch}}</b><br>${{curveMetrics[active][0]}}: ${{Number(el.dataset.value).toFixed(4)}}`}};
      el.onmousemove=e=>{{tip.style.left=e.clientX+'px';tip.style.top=e.clientY+'px'}};
      el.onmouseleave=()=>tip.style.opacity=0;
    }});
  }}
  $('#metricButtons').onclick=e=>{{if(!e.target.dataset.metric)return;active=e.target.dataset.metric;document.querySelectorAll('[data-metric]').forEach(b=>b.classList.toggle('active',b===e.target));reveal=1;draw()}};
  $('#play').onclick=()=>{{reveal=0;const start=performance.now();(function tick(t){{reveal=Math.min(1,(t-start)/1600);draw();if(reveal<1)requestAnimationFrame(tick)}})(start)}};
  draw();
}})();

/* --- human vs v4.3 vs v4.4 replay --- */
(function(){{
  const T=D.tripleReplay;
  const section=$('#tripleReplaySection');
  if(!T||!T.trajectories||!T.trajectories.length){{
    if(section) section.innerHTML='<div class="section-head"><div><div class="eyebrow">04 · same prefix, three futures</div><h2>Triple replay pending</h2></div><p>Missing <code>reports/policy_bc_v4_4_triple_replay.json</code>.</p></div>';
    return;
  }}
  const pairs=T.trajectories;
  let active=0, playTimer=null;
  const SVGNS='http://www.w3.org/2000/svg';
  const el=(tag,attrs={{}})=>{{const n=document.createElementNS(SVGNS,tag);for(const[k,v]of Object.entries(attrs))n.setAttribute(k,v);return n;}};
  const svgText=(attrs,text)=>{{const t=el('text',attrs);t.textContent=text;return t;}};
  const sci=v=>{{const n=Number(v); if(!Number.isFinite(n))return '—'; if(Math.abs(n)>=0.01)return n.toFixed(3); return n.toExponential(2);}};
  const titleCase=s=>String(s).replace(/[-_]/g,' ').replace(/\\b\\w/g,c=>c.toUpperCase());

  function towerOverlay(g,w,h,ox,oy,scale,opacity){{
    const tw=w*0.11;
    const tower=(cx,cy,king,friendly)=>{{
      const size=king?tw*1.35:tw;
      g.appendChild(el('rect',{{x:cx-size/2,y:cy-size/2,width:size,height:size,rx:3*scale,fill:friendly?'#1e3a8a':'#7f1d1d',stroke:friendly?'#93c5fd':'#fca5a5','stroke-width':1.2*scale,opacity}}));
    }};
    tower(ox+w*0.22,oy+h*0.19,false,false);tower(ox+w*0.78,oy+h*0.19,false,false);tower(ox+w*0.5,oy+h*0.075,true,false);
    tower(ox+w*0.22,oy+h*0.81,false,true);tower(ox+w*0.78,oy+h*0.81,false,true);tower(ox+w*0.5,oy+h*0.925,true,true);
  }}
  function mountArena(svg){{
    const W=280,H=420,PAD=6,TOP=20;
    const AW=W-PAD*2,AH=H-TOP-22;
    svg.setAttribute('viewBox',`0 0 ${{W}} ${{H}}`);
    svg.innerHTML='';
    const g=el('g'); svg.appendChild(g);
    g.appendChild(el('rect',{{x:PAD,y:TOP,width:AW,height:AH,rx:10,fill:'#0d1a2b'}}));
    g.appendChild(el('rect',{{x:PAD,y:TOP,width:AW,height:AH*0.5,rx:10,fill:'#991b1b',opacity:0.28}}));
    g.appendChild(el('rect',{{x:PAD,y:TOP+AH*0.5,width:AW,height:AH*0.5,fill:'#1d4ed8',opacity:0.22}}));
    g.appendChild(el('rect',{{x:PAD,y:TOP+AH*0.465,width:AW,height:AH*0.07,fill:'#1d4ed8',opacity:0.45}}));
    g.appendChild(el('rect',{{x:PAD+AW*0.16,y:TOP+AH*0.455,width:AW*0.12,height:AH*0.09,fill:'#7c5c33',opacity:0.75}}));
    g.appendChild(el('rect',{{x:PAD+AW*0.72,y:TOP+AH*0.455,width:AW*0.12,height:AH*0.09,fill:'#7c5c33',opacity:0.75}}));
    towerOverlay(g,AW,AH,PAD,TOP,1,0.85);
    g.appendChild(svgText({{x:PAD,y:13,fill:'#fca5a5','font-size':11,'letter-spacing':'0.12em','font-weight':700}},'ENEMY SIDE ↑'));
    g.appendChild(svgText({{x:PAD,y:H-6,fill:'#93c5fd','font-size':11,'letter-spacing':'0.12em','font-weight':700}},'YOUR SIDE ↓'));
    return {{g, xPx:nx=>PAD+nx*AW, yPx:ny=>TOP+(1-ny)*AH}};
  }}
  function paint(svg, track, upto, warm, accent){{
    const {{g,xPx,yPx}}=mountArena(svg);
    track.events.slice(0,upto).forEach((e,i)=>{{
      const isWarm=i<warm;
      const color=isWarm?'#94a3b8':(e.side==='team'?'#60a5fa':accent);
      const latest=i===upto-1;
      g.appendChild(el('circle',{{
        cx:xPx(e.x), cy:yPx(e.y), r:latest?8:5,
        fill:color, opacity:isWarm?0.42:0.9,
        stroke:latest?'#e2e8f0':'none','stroke-width':latest?2:0
      }}));
    }});
  }}
  function draw(upto){{
    const pair=pairs[active];
    const warm=pair.warmupEvents||12;
    paint($('#hArena'), pair.human, upto, warm, '#70a1ff');
    paint($('#a43Arena'), pair.v43, upto, warm, pair.v43.color||'#e8f58b');
    paint($('#a44Arena'), pair.v44, upto, warm, pair.v44.color||'#38bdf8');
    $('#hScore').textContent=`P(human)=${{sci(pair.human.score)}}`;
    $('#a43Score').textContent=`P(human)=${{sci(pair.v43.score)}}`;
    $('#a44Score').textContent=`P(human)=${{sci(pair.v44.score)}}`;
    const n=Number($('#tripleScrub').max);
    $('#tripleReadout').textContent=`event ${{upto}}/${{n}} ${{upto<=warm?'(shared warm-up)':'(diverged)'}}`;
    const h=pair.human.events[upto-1], a=pair.v43.events[upto-1], b=pair.v44.events[upto-1];
    $('#tripleLog').innerHTML=(h&&a&&b)
      ?`human: <b>${{titleCase(h.card)}}</b> @ ${{h.t}}s (${{h.side}}) &nbsp;·&nbsp; v4.3: <b>${{titleCase(a.card)}}</b> @ ${{a.t}}s &nbsp;·&nbsp; v4.4: <b>${{titleCase(b.card)}}</b> @ ${{b.t}}s`
      :'';
  }}
  function setPair(i){{
    active=i;
    [...$('#tripleChips').children].forEach((c,j)=>c.classList.toggle('active',j===i));
    const pair=pairs[i];
    const n=Math.min(pair.human.events.length, pair.v43.events.length, pair.v44.events.length);
    const scrub=$('#tripleScrub');
    scrub.max=n; scrub.value=n;
    draw(n);
  }}
  $('#tripleChips').innerHTML=pairs.map((p,i)=>`<button type="button" class="replay-chip${{i===0?' active':''}}" data-i="${{i}}">battle ${{p.battleId}}</button>`).join('');
  $('#tripleChips').onclick=e=>{{const b=e.target.closest('[data-i]'); if(b) setPair(Number(b.dataset.i));}};
  $('#tripleScrub').oninput=()=>draw(Number($('#tripleScrub').value));
  $('#triplePlay').onclick=()=>{{
    if(playTimer){{clearInterval(playTimer);playTimer=null;$('#triplePlay').textContent='▶ play';return;}}
    $('#triplePlay').textContent='❚❚';
    let i=1; const max=Number($('#tripleScrub').max);
    playTimer=setInterval(()=>{{
      $('#tripleScrub').value=i; draw(i); i+=1;
      if(i>max){{clearInterval(playTimer);playTimer=null;$('#triplePlay').textContent='▶ play';}}
    }},130);
  }};
  setPair(0);
}})();
</script></body></html>"""

    output = Path(output_path)
    if not output.is_absolute():
        output = ROOT / output
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(html, encoding="utf-8")
    return output


if __name__ == "__main__":
    print(render_policy_v44_report())
