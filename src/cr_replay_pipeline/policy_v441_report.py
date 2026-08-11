"""Rich interactive report for PolicyBC v4.4.1 (native HTML/SVG/JS)."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from .policy_v44_report import ROOT, _fmt_compact, _fmt_duration, _js, _load, _source_battles


def _history(model_dir: Path, report: dict[str, Any], *, include_prior: bool) -> list[dict[str, Any]]:
    current_path = model_dir / "training_stages.json"
    current = _load(current_path) if current_path.exists() else list(report.get("history") or [])
    if not include_prior:
        return current
    prior_path = model_dir / "training_stage_epoch1.json"
    prior = _load(prior_path) if prior_path.exists() else []
    merged = [dict(row) for row in prior]
    offset = len(merged)
    for row in current:
        shifted = dict(row)
        shifted["epoch"] = int(row.get("epoch", 0)) + offset
        merged.append(shifted)
    return merged


def _payload(label: str, path: Path, color: str, *, include_prior: bool = False) -> dict[str, Any]:
    report = _load(path / "report.json")
    history = _history(path, report, include_prior=include_prior)
    compute = report["compute"]
    return {
        "id": label,
        "color": color,
        "history": history,
        "test": report["test"],
        "sourceBattles": _source_battles(report),
        "trainSamples": int(report["data"]["train_samples"]),
        "testSamples": int(report["data"]["test_samples"]),
        "epochs": len(history),
        "batch": int(compute["batch_size"]),
        "parameters": int(compute.get("parameters") or 0),
        "duration": _fmt_duration(float(report["seconds"])),
    }


def render_policy_v441_report(
    model_dir: str | Path = "models/policy_bc_v4.4.1",
    output_path: str | Path = "reports/policy_bc_v4_4_1.html",
) -> Path:
    target = Path(model_dir)
    if not target.is_absolute():
        target = ROOT / target
    old_path = ROOT / "models" / "policy_bc_v4.4"
    report = _load(target / "report.json")
    triple_path = ROOT / "reports" / "policy_bc_v4_4_1_triple_replay.json"
    triple = _load(triple_path) if triple_path.exists() else None
    old = _payload("v4.4", old_path, "#e8f58b")
    new = _payload("v4.4.1", target, "#38bdf8", include_prior=True)
    rollout = report.get("rollouts") or {}
    diversity = rollout.get("placement_diversity") or {}
    human_div = diversity.get("human") or {}
    policy_div = diversity.get("policy") or {}
    test = report["test"]
    compute = report["compute"]

    progress = target / "progress.jsonl"
    peak_vram = 0.0
    if progress.exists():
        for line in progress.read_text(encoding="utf-8").splitlines():
            try:
                peak_vram = max(peak_vram, float(json.loads(line).get("gpu_memory_mb") or 0))
            except (ValueError, json.JSONDecodeError):
                pass

    created = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
    delta = {
        "slot": 100 * (new["test"]["slot_top1"] - old["test"]["slot_top1"]),
        "zone": 100 * (new["test"]["zone_acc"] - old["test"]["zone_acc"]),
        "tile": 100 * (new["test"]["tile_class_acc"] - old["test"]["tile_class_acc"]),
        "top5": 100 * (new["test"]["tile_top5_acc"] - old["test"]["tile_top5_acc"]),
    }
    human_unique = float(human_div.get("mean_unique_tiles") or 0)
    policy_unique = float(policy_div.get("mean_unique_tiles") or 0)
    diversity_ratio = policy_unique / human_unique if human_unique else 0.0

    data = {
        "models": [old, new],
        "diversity": diversity,
        "rollout": rollout,
        "tripleReplay": triple,
        "distribution": {
            "entropy": test.get("tile_entropy_bits"),
            "effective": test.get("tile_effective_count"),
            "top1mass": test.get("tile_top1_mass"),
            "top5mass": test.get("tile_top5_mass"),
            "expectedX": test.get("model_x_std"),
            "expectedY": test.get("model_y_std"),
            "argmaxX": test.get("argmax_x_std"),
            "argmaxY": test.get("argmax_y_std"),
        },
    }

    html = f"""<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>PolicyBC v4.4.1 — diversity harness report</title>
<style>
:root{{--bg:#071018;--panel:#0d171f;--line:#1e3140;--text:#edf4f8;--muted:#8aa3b3;--green:#70e1b1;--gold:#ffca63;--sky:#38bdf8;--red:#ff7e78;--star:#e8f58b;--ink:#061018}}
*{{box-sizing:border-box}}html{{scroll-behavior:smooth}}body{{margin:0;background:radial-gradient(circle at 12% -8%,#16384a 0,transparent 34%),radial-gradient(circle at 90% 0,#1d3f32 0,transparent 28%),var(--bg);color:var(--text);font:15px/1.55 "IBM Plex Sans",ui-sans-serif,system-ui,sans-serif}}
body:before{{content:"";position:fixed;inset:0;pointer-events:none;opacity:.045;background-image:linear-gradient(#fff 1px,transparent 1px),linear-gradient(90deg,#fff 1px,transparent 1px);background-size:42px 42px;mask-image:linear-gradient(to bottom,#000,transparent 72%)}}
main{{width:min(1220px,calc(100% - 36px));margin:auto;padding:52px 0 90px}}.eyebrow{{font:700 11px/1.2 "IBM Plex Mono",ui-monospace,monospace;letter-spacing:.16em;text-transform:uppercase;color:var(--sky)}}
h1{{font-size:clamp(42px,7.4vw,82px);line-height:.92;letter-spacing:-.065em;margin:18px 0 22px;max-width:1020px}}h1 em{{color:var(--sky);font-style:normal}}h2{{font-size:clamp(25px,4vw,40px);letter-spacing:-.04em;margin:0 0 10px}}h3{{font-size:17px;margin:0 0 8px}}p{{color:var(--muted);margin:0}}.lede{{font-size:clamp(17px,2.1vw,22px);max-width:900px;color:#bdd0db}}.hero{{padding:18px 0 48px}}.stamp{{margin-top:20px;color:#607888;font:12px "IBM Plex Mono",ui-monospace,monospace}}
.verdict{{margin-top:38px;border:1px solid #2f6f8a;background:linear-gradient(135deg,rgba(56,189,248,.14),rgba(112,225,177,.05));border-radius:22px;padding:24px;display:grid;grid-template-columns:auto 1fr;gap:18px;align-items:start}}.verdict .mark{{width:50px;height:50px;border-radius:14px;display:grid;place-items:center;background:var(--sky);color:var(--ink);font-size:22px;font-weight:900}}.verdict strong{{display:block;font-size:20px;margin-bottom:5px}}
.section{{border-top:1px solid var(--line);padding:52px 0}}.section-head{{display:flex;justify-content:space-between;align-items:end;gap:20px;margin-bottom:26px}}.section-head p{{max-width:580px}}.kpis{{display:grid;grid-template-columns:repeat(4,1fr);gap:12px}}.grid2{{display:grid;grid-template-columns:1fr 1fr;gap:14px}}.grid3{{display:grid;grid-template-columns:repeat(3,1fr);gap:13px}}
.kpi,.card{{border:1px solid var(--line);background:linear-gradient(160deg,rgba(17,28,37,.94),rgba(8,14,20,.94));border-radius:18px;padding:20px;position:relative;overflow:hidden}}.kpi:after{{content:"";position:absolute;width:100px;height:100px;border-radius:50%;background:var(--tone,var(--sky));filter:blur(55px);opacity:.14;right:-30px;top:-35px}}.kpi span{{display:block;color:var(--muted);font-size:12px;text-transform:uppercase;letter-spacing:.08em}}.kpi b{{display:block;font-size:clamp(24px,3.4vw,38px);letter-spacing:-.045em;margin:6px 0 2px}}.kpi small{{color:#6f8796}}
.delta{{font:750 13px "IBM Plex Mono",monospace}}.up{{color:var(--green)}}.down{{color:var(--red)}}.neutral{{color:var(--muted)}}.metric-table{{width:100%;border-collapse:separate;border-spacing:0}}.metric-table th,.metric-table td{{padding:13px 14px;border-bottom:1px solid var(--line);text-align:right}}.metric-table th:first-child,.metric-table td:first-child{{text-align:left}}.metric-table thead th{{font:700 11px "IBM Plex Mono",monospace;text-transform:uppercase;letter-spacing:.08em;color:var(--muted)}}.metric-table tbody tr:hover{{background:#12202a}}.winner{{color:var(--sky);font-weight:750}}.metric-note{{font-size:11px;color:#617888;display:block}}
.chart-card{{padding:22px}}.chart-tools{{display:flex;flex-wrap:wrap;gap:9px;margin:14px 0}}button,.toggle{{border:1px solid var(--line);background:#0a151c;color:var(--muted);border-radius:10px;padding:8px 11px;font:600 12px "IBM Plex Mono",monospace;cursor:pointer}}button:hover{{border-color:#416a80;color:var(--text)}}button.active{{background:var(--sky);color:var(--ink);border-color:var(--sky)}}.toggle input{{accent-color:var(--tone)}}#curve{{width:100%;height:390px;display:block}}.gridline{{stroke:#1e3140;stroke-width:1}}.axis-label{{fill:#668090;font:11px "IBM Plex Mono",monospace}}.curve{{fill:none;stroke-width:3;stroke-linecap:round;stroke-linejoin:round}}.point{{cursor:crosshair}}.tooltip{{position:fixed;pointer-events:none;z-index:9;background:#edf4f8;color:#071018;border-radius:10px;padding:8px 10px;font:12px "IBM Plex Mono",monospace;box-shadow:0 8px 28px #0008;opacity:0;transform:translate(-50%,-120%)}}
.bar-set{{display:grid;gap:16px}}.bar-row{{display:grid;grid-template-columns:120px 1fr 85px;gap:12px;align-items:center}}.bar-track{{height:15px;border-radius:99px;background:#08131a;overflow:hidden;border:1px solid #1c2d38}}.bar-fill{{height:100%;width:0;border-radius:99px;background:var(--tone);transition:width 1s cubic-bezier(.2,.8,.2,1)}}.bar-value{{font:700 12px "IBM Plex Mono",monospace;text-align:right}}.expr{{margin-top:14px;padding:14px 16px;border:1px dashed #355065;border-radius:12px;background:#0a151c;font:13px/1.55 "IBM Plex Mono",monospace;color:#c5d7e2}}.expr b{{color:var(--sky)}}.callout{{border-left:3px solid var(--gold);padding:15px 18px;background:#19180f;border-radius:0 12px 12px 0;margin-top:18px;color:#d7cfaf}}.decoder{{display:grid;grid-template-columns:repeat(5,1fr);gap:10px;align-items:stretch}}.step{{padding:16px;border:1px solid var(--line);border-radius:14px;background:#0a151c}}.step b{{display:block;color:var(--sky);font:700 11px "IBM Plex Mono",monospace;letter-spacing:.08em;margin-bottom:8px}}.lesson .n{{font:800 12px "IBM Plex Mono",monospace;color:var(--sky)}}.lesson h3{{margin:14px 0 7px}}.lesson p{{font-size:13px}}.foot{{padding-top:26px;color:#55707f;font:11px "IBM Plex Mono",monospace}}
.replay-chips{{display:flex;flex-wrap:wrap;gap:8px;margin:0 0 14px}}.replay-chip{{border-radius:999px;padding:8px 13px}}.replay-chip.active{{border-color:var(--sky);background:rgba(56,189,248,.14);color:var(--text)}}.triple-grid{{display:grid;grid-template-columns:repeat(3,1fr);gap:12px;margin-top:8px}}.replay-pane h3{{display:flex;justify-content:space-between;align-items:baseline;gap:8px;margin-bottom:10px;font-size:15px}}.pane-score{{font:700 11px "IBM Plex Mono",monospace;color:var(--muted)}}.arena{{width:100%;max-width:280px;height:auto;display:block;margin:0 auto;border-radius:12px}}.anim-toolbar{{display:flex;align-items:center;gap:12px;margin-top:14px}}.anim-scrubber{{flex:1;accent-color:var(--sky)}}.anim-readout,.replay-log{{font:12px "IBM Plex Mono",monospace;color:var(--muted);margin-top:8px}}
@media(max-width:900px){{.kpis,.grid2,.grid3,.decoder,.triple-grid{{grid-template-columns:1fr}}.section-head{{align-items:start;flex-direction:column}}}}
</style></head><body><main>
<header class="hero"><div class="eyebrow">Clash Royale AI · placement diversity experiment</div><h1>Pick a mode.<br><em>Do not repeat it forever.</em></h1>
<p class="lede">v4.4.1 keeps the 18×32 heatmap, conditions placement on the card actually selected, and turns the deployment harness into a controlled top-5 sampler. The result is more varied play—but the human diversity gap is still measurable.</p>
<div class="stamp">REPORT GENERATED {created} · MODEL policy-bc-v4.4.1 · VERSION 4.4.1 · 51,055 BATTLES · DECODE TOP-5 @ T=0.6</div>
<div class="verdict"><div class="mark">↯</div><div><strong>Realized placement diversity reaches {diversity_ratio*100:.0f}% of human unique-tile coverage.</strong><p>The policy uses {policy_unique:.2f} unique tiles per continuation versus {human_unique:.2f} for humans. This is a real improvement over deterministic mode collapse, but its favorite tile still receives {float(policy_div.get('mean_max_tile_share') or 0)*100:.1f}% of placements—about twice the human concentration.</p></div></div></header>

<section class="section"><div class="section-head"><div><div class="eyebrow">01 · run at a glance</div><h2>More data, exact card context</h2></div><p>The first completed epoch is merged with the 11 resumed epochs. Mirroring is lazy and streams are LRU-bounded, keeping the final run near 3.1 GB RAM.</p></div>
<div class="kpis"><div class="kpi" style="--tone:var(--sky)"><span>Source battles</span><b>{new['sourceBattles']:,}</b><small>+{new['sourceBattles']-old['sourceBattles']:,} vs v4.4</small></div><div class="kpi" style="--tone:var(--green)"><span>Actions / epoch</span><b>{_fmt_compact(new['trainSamples'])}</b><small>mirrored · cap 40</small></div><div class="kpi" style="--tone:var(--gold)"><span>Completed epochs</span><b>{new['epochs']}</b><small>1 preserved + 11 resumed</small></div><div class="kpi" style="--tone:var(--star)"><span>Compute</span><b>{_fmt_compact(new['parameters'])}</b><small>params · batch {new['batch']} · peak VRAM {peak_vram:.0f} MB</small></div></div></section>

<section class="section"><div class="section-head"><div><div class="eyebrow">02 · v4.4 → v4.4.1</div><h2>Small accuracy gains, different behavior</h2></div><p>Different held-out cuts make cross-version deltas directional rather than perfectly paired. The architectural point is selected-card consistency and decode diversity.</p></div>
<div class="card" style="padding:4px 0;overflow:auto"><table class="metric-table" id="metricTable"></table></div>
<div class="callout"><b>Bottom line:</b> card top-1 {delta['slot']:+.2f} pt · zone {delta['zone']:+.2f} pt · exact tile {delta['tile']:+.2f} pt · tile top-5 {delta['top5']:+.2f} pt. Longer training mostly plateaued; the meaningful change is how placement modes are conditioned and decoded.</div></section>

<section class="section"><div class="section-head"><div><div class="eyebrow">03 · realized diversity</div><h2>Broad enough to vary, still too repetitive</h2></div><p>Fair 18×32 quantization over 96 held-out continuations. Ability activations are excluded because they have no placement target.</p></div>
<div class="grid2"><div class="card"><h3>Unique tiles per continuation</h3><div class="bar-set" id="uniqueBars"></div><div class="expr"><b>coverage ratio:</b> {diversity_ratio:.3f} · policy gap = {policy_unique-human_unique:+.2f} tiles</div></div><div class="card"><h3>Largest single-tile share</h3><div class="bar-set" id="shareBars"></div><div class="expr"><b>lower is better:</b> policy {float(policy_div.get('mean_max_tile_share') or 0)*100:.2f}% · human {float(human_div.get('mean_max_tile_share') or 0)*100:.2f}%</div></div></div></section>

<section class="section"><div class="section-head"><div><div class="eyebrow">04 · distribution anatomy</div><h2>The logits are multimodal; time creates repetition</h2></div><p>Expected XY still compresses modes. Argmax spread is broad, while realized sequences revisit their favorite modes too often.</p></div>
<div class="kpis"><div class="kpi" style="--tone:var(--sky)"><span>Entropy</span><b>{float(test.get('tile_entropy_bits') or 0):.2f} bits</b><small>per placement distribution</small></div><div class="kpi" style="--tone:var(--green)"><span>Effective tiles</span><b>{float(test.get('tile_effective_count') or 0):.1f}</b><small>2^entropy</small></div><div class="kpi" style="--tone:var(--gold)"><span>Top-1 mass</span><b>{float(test.get('tile_top1_mass') or 0)*100:.1f}%</b><small>single strongest mode</small></div><div class="kpi" style="--tone:var(--star)"><span>Top-5 mass</span><b>{float(test.get('tile_top5_mass') or 0)*100:.1f}%</b><small>decoder candidate set</small></div></div>
<div class="card" style="margin-top:14px"><div class="decoder"><div class="step"><b>01 · CARD</b>Sample a legal deck slot.</div><div class="step"><b>02 · CONDITION</b>Use that exact card embedding.</div><div class="step"><b>03 · HEATMAP</b>Score 576 arena tiles.</div><div class="step"><b>04 · FILTER</b>Keep the strongest five.</div><div class="step"><b>05 · SAMPLE</b>Temperature 0.6, then tap center.</div></div></div></section>

<section class="section" id="tripleReplaySection"><div class="section-head"><div><div class="eyebrow">05 · same prefix, three futures</div><h2>Human vs v4.4 vs v4.4.1</h2></div><p>Grey dots are the same 12 recorded actions. After the cut, compare the human continuation, v4.4’s deterministic tile mode, and v4.4.1’s actual top-5 sampler.</p></div>
<div class="card chart-card"><div class="replay-chips" id="tripleChips"></div><div class="triple-grid">
<div class="replay-pane"><h3>Human <span class="pane-score" id="hScore"></span></h3><svg class="arena" id="hArena"></svg></div>
<div class="replay-pane"><h3>AI · v4.4 tiles <span class="pane-score" id="a44Score"></span></h3><svg class="arena" id="a44Arena"></svg></div>
<div class="replay-pane"><h3>AI · v4.4.1 <span class="pane-score" id="a441Score"></span></h3><svg class="arena" id="a441Arena"></svg></div>
</div><div class="anim-toolbar"><button type="button" id="triplePlay">▶ play</button><input type="range" class="anim-scrubber" id="tripleScrub" min="1" max="1" value="1" step="1"><div class="anim-readout" id="tripleReadout"></div></div><div class="replay-log" id="tripleLog"></div>
<div class="expr"><b>paired decode:</b> v4.4 = saved historical argmax-tile rollout · v4.4.1 = two-sided delay race, slots sampled at T=0.8, placement sampled among top 5 at T=0.6 · K=3 · first 12 events shared</div>
<div class="callout"><b>Score caution:</b> P(human) comes from the exploitable style judge discussed below. The trajectories are the evidence; the score is context, not the verdict.</div></div></section>

<section class="section"><div class="section-head"><div><div class="eyebrow">06 · validation during training</div><h2>Curves by epoch</h2></div><p>Toggle runs and metrics, hover points, or replay the draw. v4.4.1 includes the preserved first epoch plus all 11 resumed epochs.</p></div>
<div class="card chart-card"><div class="chart-tools" id="metricButtons"></div><div class="chart-tools" id="modelToggles"></div><button id="play">replay curves</button><svg id="curve" viewBox="0 0 1040 390" role="img" aria-label="Interactive curves by epoch"></svg></div></section>

<section class="section"><div class="section-head"><div><div class="eyebrow">07 · interpretation</div><h2>What v4.4.1 actually taught us</h2></div><p>The diversity diagnosis is more important than a green readiness badge.</p></div><div class="grid3"><div class="card lesson"><div class="n">01 / FIXED</div><h3>Card context now matches the action.</h3><p>Training uses the human-selected card; inference and rollouts use the model-selected card. The placement head no longer receives a soft card mixture at evaluation.</p></div><div class="card lesson"><div class="n">02 / IMPROVED</div><h3>Sampling recovers spatial variety.</h3><p>{policy_unique:.1f} unique tiles is materially broader than deterministic demos, and argmax spread reaches x={float(test.get('argmax_x_std') or 0):.3f}, y={float(test.get('argmax_y_std') or 0):.3f}.</p></div><div class="card lesson"><div class="n">03 / OPEN</div><h3>Temporal mode reuse remains.</h3><p>The policy’s favorite tile share is still about {float(policy_div.get('mean_max_tile_share') or 0)/max(float(human_div.get('mean_max_tile_share') or 1),1e-9):.1f}× human. A future objective should penalize repeated modes conditionally—not flatten every heatmap.</p></div></div>
<div class="callout"><b>Judge warning:</b> the realism scorer rates policy {float(rollout.get('mean_score_policy') or 0):.3f} versus real {float(rollout.get('mean_score_real') or 0):.3f}. Treat that inversion as evidence the judge is incomplete or exploitable, not evidence that the policy is “more human than human.”</div></section>
<footer class="foot">Native HTML + SVG + JavaScript · no matplotlib · selected-card placement · top-5 T=0.6 decode · report generated from the final v4.4.1 checkpoint.</footer></main><div class="tooltip" id="tip"></div>
<script id="report-data" type="application/json">{_js(data)}</script><script>
const D=JSON.parse(document.getElementById('report-data').textContent),$=s=>document.querySelector(s),pct=n=>(100*n).toFixed(2)+'%',fmt=n=>new Intl.NumberFormat('en-US').format(Math.round(n));
const metrics=[['Card top-1','slot_top1','pct',true,'Correct deck slot'],['Card top-3','slot_top3','pct',true,'Human card in top three'],['Zone accuracy','zone_acc','pct',true,'Correct 12-way region'],['Exact tile top-1','tile_class_acc','pct',true,'576-way class'],['Tile top-5','tile_top5_acc','pct',true,'Human tile among five modes'],['Placement MAE','xy_mae','int',false,'Expected XY · API units'],['Timing MAE','timing_mae','sec',false,'Seconds']];
const format=(v,t)=>t==='pct'?pct(v):t==='sec'?v.toFixed(3)+'s':t==='int'?fmt(v):v.toFixed(3);
$('#metricTable').innerHTML='<thead><tr><th>Metric</th>'+D.models.map(m=>`<th style="color:${{m.color}}">${{m.id}}</th>`).join('')+'</tr></thead><tbody>'+metrics.map(([label,key,type,higher,note])=>{{const vals=D.models.map(m=>m.test[key]),best=higher?Math.max(...vals):Math.min(...vals);return `<tr><td><b>${{label}}</b><span class="metric-note">${{note}}</span></td>`+vals.map(v=>`<td class="${{v===best?'winner':''}}">${{format(v,type)}}</td>`).join('')+'</tr>'}}).join('')+'</tbody>';
function bars(host,rows,max,percent=false){{$(host).innerHTML=rows.map(r=>`<div class="bar-row"><b style="color:${{r.color}}">${{r.label}}</b><div class="bar-track"><div class="bar-fill" style="--tone:${{r.color}}" data-width="${{100*r.value/max}}"></div></div><div class="bar-value">${{percent?pct(r.value):r.value.toFixed(2)}}</div></div>`).join('');requestAnimationFrame(()=>$(host).querySelectorAll('.bar-fill').forEach(b=>b.style.width=b.dataset.width+'%'))}}
const h=D.diversity.human||{{}},pdiv=D.diversity.policy||{{}};bars('#uniqueBars',[{{label:'Human',value:h.mean_unique_tiles||0,color:'#70e1b1'}},{{label:'v4.4.1',value:pdiv.mean_unique_tiles||0,color:'#38bdf8'}}],Math.max(h.mean_unique_tiles||1,pdiv.mean_unique_tiles||1));bars('#shareBars',[{{label:'Human',value:h.mean_max_tile_share||0,color:'#70e1b1'}},{{label:'v4.4.1',value:pdiv.mean_max_tile_share||0,color:'#ffca63'}}],Math.max(h.mean_max_tile_share||.01,pdiv.mean_max_tile_share||.01),true);
const curveMetrics={{val_loss:['Validation loss','loss'],val_slot_top1:['Card top-1','pct'],val_zone_acc:['Zone accuracy','pct'],val_tile_class_acc:['Exact tile top-1','pct'],val_tile_top5_acc:['Tile top-5','pct'],val_tile_entropy_bits:['Tile entropy','bits'],val_tile_effective_count:['Effective tiles','count']}};
let active='val_tile_class_acc',visible=new Set(D.models.map(m=>m.id)),reveal=1;$('#metricButtons').innerHTML=Object.entries(curveMetrics).map(([k,v])=>`<button data-metric="${{k}}" class="${{k===active?'active':''}}">${{v[0]}}</button>`).join('');$('#modelToggles').innerHTML=D.models.map(m=>`<label class="toggle" style="--tone:${{m.color}}"><input type="checkbox" data-model="${{m.id}}" checked> ${{m.id}}</label>`).join('');
const svg=$('#curve'),tip=$('#tip');function axis(v,type){{return type==='pct'?pct(v):type==='bits'?v.toFixed(2)+'b':type==='count'?v.toFixed(1):v.toFixed(2)}}
function draw(){{const W=1040,H=390,p={{l:72,r:24,t:24,b:48}},shown=D.models.filter(m=>visible.has(m.id)),all=shown.flatMap(m=>m.history.map(r=>r[active]).filter(Number.isFinite));if(!all.length){{svg.innerHTML='<text x="40" y="40" fill="#8aa3b3">No history for this metric</text>';return}}let lo=Math.min(...all),hi=Math.max(...all),pad=(hi-lo||1)*.12;lo-=pad;hi+=pad;const maxEpoch=Math.max(...shown.map(m=>m.history.length)),x=e=>p.l+(e-1)/Math.max(maxEpoch-1,1)*(W-p.l-p.r),y=v=>p.t+(hi-v)/(hi-lo)*(H-p.t-p.b),type=curveMetrics[active][1];let out='';for(let i=0;i<5;i++){{const yy=p.t+i*(H-p.t-p.b)/4,v=hi-i*(hi-lo)/4;out+=`<line class="gridline" x1="${{p.l}}" y1="${{yy}}" x2="${{W-p.r}}" y2="${{yy}}"/><text class="axis-label" x="${{p.l-10}}" y="${{yy+4}}" text-anchor="end">${{axis(v,type)}}</text>`}}for(let i=1;i<=maxEpoch;i++)out+=`<text class="axis-label" x="${{x(i)}}" y="${{H-17}}" text-anchor="middle">${{i}}</text>`;shown.forEach(m=>{{const rows=m.history.slice(0,Math.ceil(m.history.length*reveal)).filter(r=>Number.isFinite(r[active])),pts=rows.map(r=>[x(r.epoch),y(r[active]),r]);out+=`<path class="curve" stroke="${{m.color}}" d="${{pts.map((q,i)=>(i?'L':'M')+q[0].toFixed(1)+','+q[1].toFixed(1)).join(' ')}}"/>`;out+=pts.map(q=>`<circle class="point" data-model="${{m.id}}" data-epoch="${{q[2].epoch}}" data-value="${{q[2][active]}}" fill="${{m.color}}" cx="${{q[0]}}" cy="${{q[1]}}" r="4"/>`).join('')}});svg.innerHTML=out;svg.querySelectorAll('.point').forEach(el=>{{el.onmouseenter=()=>{{tip.style.opacity=1;tip.innerHTML=`<b>${{el.dataset.model}} · epoch ${{el.dataset.epoch}}</b><br>${{curveMetrics[active][0]}}: ${{axis(Number(el.dataset.value),type)}}`}};el.onmousemove=e=>{{tip.style.left=e.clientX+'px';tip.style.top=e.clientY+'px'}};el.onmouseleave=()=>tip.style.opacity=0}})}}
$('#metricButtons').onclick=e=>{{if(!e.target.dataset.metric)return;active=e.target.dataset.metric;document.querySelectorAll('[data-metric]').forEach(b=>b.classList.toggle('active',b===e.target));reveal=1;draw()}};$('#modelToggles').onchange=e=>{{e.target.checked?visible.add(e.target.dataset.model):visible.delete(e.target.dataset.model);draw()}};$('#play').onclick=()=>{{reveal=0;const start=performance.now();(function tick(t){{reveal=Math.min(1,(t-start)/1800);draw();if(reveal<1)requestAnimationFrame(tick)}})(start)}};draw();

(function(){{
const T=D.tripleReplay,section=$('#tripleReplaySection');
if(!T?.trajectories?.length){{section.innerHTML='<div class="section-head"><div><div class="eyebrow">05 · same prefix, three futures</div><h2>Triple replay pending</h2></div><p>Run <code>PYTHONPATH=src .venv/bin/python scripts/build_v441_triple_replay.py</code>.</p></div>';return;}}
const pairs=T.trajectories,NS='http://www.w3.org/2000/svg';let activePair=0,playTimer=null;
const el=(tag,attrs={{}})=>{{const n=document.createElementNS(NS,tag);for(const[k,v]of Object.entries(attrs))n.setAttribute(k,v);return n}},svgText=(attrs,s)=>{{const n=el('text',attrs);n.textContent=s;return n}},sci=v=>{{const n=Number(v);return !Number.isFinite(n)?'—':Math.abs(n)>=.01?n.toFixed(3):n.toExponential(2)}},title=s=>String(s).replace(/[-_]/g,' ').replace(/\\b\\w/g,c=>c.toUpperCase());
function towerOverlay(g,w,h,ox,oy){{const tw=w*.11,tower=(cx,cy,king,friendly)=>{{const z=king?tw*1.35:tw;g.appendChild(el('rect',{{x:cx-z/2,y:cy-z/2,width:z,height:z,rx:3,fill:friendly?'#1e3a8a':'#7f1d1d',stroke:friendly?'#93c5fd':'#fca5a5','stroke-width':1.2,opacity:.85}}))}};tower(ox+w*.22,oy+h*.19,false,false);tower(ox+w*.78,oy+h*.19,false,false);tower(ox+w*.5,oy+h*.075,true,false);tower(ox+w*.22,oy+h*.81,false,true);tower(ox+w*.78,oy+h*.81,false,true);tower(ox+w*.5,oy+h*.925,true,true)}}
function mount(svg){{const W=280,H=420,P=6,TOP=20,AW=W-P*2,AH=H-TOP-22;svg.setAttribute('viewBox',`0 0 ${{W}} ${{H}}`);svg.innerHTML='';const g=el('g');svg.appendChild(g);g.appendChild(el('rect',{{x:P,y:TOP,width:AW,height:AH,rx:10,fill:'#0d1a2b'}}));g.appendChild(el('rect',{{x:P,y:TOP,width:AW,height:AH*.5,rx:10,fill:'#991b1b',opacity:.28}}));g.appendChild(el('rect',{{x:P,y:TOP+AH*.5,width:AW,height:AH*.5,fill:'#1d4ed8',opacity:.22}}));g.appendChild(el('rect',{{x:P,y:TOP+AH*.465,width:AW,height:AH*.07,fill:'#1d4ed8',opacity:.45}}));g.appendChild(el('rect',{{x:P+AW*.16,y:TOP+AH*.455,width:AW*.12,height:AH*.09,fill:'#7c5c33',opacity:.75}}));g.appendChild(el('rect',{{x:P+AW*.72,y:TOP+AH*.455,width:AW*.12,height:AH*.09,fill:'#7c5c33',opacity:.75}}));towerOverlay(g,AW,AH,P,TOP);g.appendChild(svgText({{x:P,y:13,fill:'#fca5a5','font-size':11,'letter-spacing':'.12em','font-weight':700}},'ENEMY SIDE ↑'));g.appendChild(svgText({{x:P,y:H-6,fill:'#93c5fd','font-size':11,'letter-spacing':'.12em','font-weight':700}},'YOUR SIDE ↓'));return{{g,x:n=>P+n*AW,y:n=>TOP+(1-n)*AH}}}}
function paint(svg,track,upto,warm,accent){{const a=mount(svg);track.events.slice(0,upto).forEach((e,i)=>{{const shared=i<warm,latest=i===upto-1,color=shared?'#94a3b8':e.side==='team'?'#60a5fa':accent;a.g.appendChild(el('circle',{{cx:a.x(e.x),cy:a.y(e.y),r:latest?8:5,fill:color,opacity:shared?.42:.9,stroke:latest?'#e2e8f0':'none','stroke-width':latest?2:0}}))}})}}
function drawReplay(upto){{const p=pairs[activePair],warm=p.warmupEvents||12;paint($('#hArena'),p.human,upto,warm,'#70e1b1');paint($('#a44Arena'),p.v44,upto,warm,p.v44.color||'#e8f58b');paint($('#a441Arena'),p.v441,upto,warm,p.v441.color||'#38bdf8');$('#hScore').textContent=`P(human)=${{sci(p.human.score)}}`;$('#a44Score').textContent=`P(human)=${{sci(p.v44.score)}}`;$('#a441Score').textContent=`P(human)=${{sci(p.v441.score)}}`;const n=Number($('#tripleScrub').max);$('#tripleReadout').textContent=`event ${{upto}}/${{n}} ${{upto<=warm?'(shared warm-up)':'(diverged)'}}`;const h=p.human.events[upto-1],a=p.v44.events[upto-1],b=p.v441.events[upto-1];$('#tripleLog').innerHTML=h&&a&&b?`human: <b>${{title(h.card)}}</b> @ ${{h.t}}s · v4.4: <b>${{title(a.card)}}</b> @ ${{a.t}}s · v4.4.1: <b>${{title(b.card)}}</b> @ ${{b.t}}s`:''}}
function setPair(i){{activePair=i;[...$('#tripleChips').children].forEach((c,j)=>c.classList.toggle('active',j===i));const p=pairs[i],n=Math.min(p.human.events.length,p.v44.events.length,p.v441.events.length),s=$('#tripleScrub');s.max=n;s.value=n;drawReplay(n)}}
$('#tripleChips').innerHTML=pairs.map((p,i)=>`<button type="button" class="replay-chip${{i===0?' active':''}}" data-i="${{i}}">battle ${{p.battleId}}</button>`).join('');$('#tripleChips').onclick=e=>{{const b=e.target.closest('[data-i]');if(b)setPair(Number(b.dataset.i))}};$('#tripleScrub').oninput=()=>drawReplay(Number($('#tripleScrub').value));$('#triplePlay').onclick=()=>{{if(playTimer){{clearInterval(playTimer);playTimer=null;$('#triplePlay').textContent='▶ play';return}}$('#triplePlay').textContent='❚❚';let i=1,max=Number($('#tripleScrub').max);playTimer=setInterval(()=>{{$('#tripleScrub').value=i;drawReplay(i++);if(i>max){{clearInterval(playTimer);playTimer=null;$('#triplePlay').textContent='▶ play'}}}},130)}};setPair(0);
}})();
</script></body></html>"""

    output = Path(output_path)
    if not output.is_absolute():
        output = ROOT / output
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(html, encoding="utf-8")
    return output


if __name__ == "__main__":
    print(render_policy_v441_report())
