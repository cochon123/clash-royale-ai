"""Interactive HTML report for Astra v8 Stage A (causal belief policy)."""

from __future__ import annotations

import html as html_lib
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from .policy_v44_report import ROOT, _fmt_compact, _fmt_duration, _js, _load
from .report_kit import FONT_LINKS, favicon_link, shared_styles


def _pct(value: Any) -> str:
    if value is None:
        return "—"
    return f"{100.0 * float(value):.2f}%"


def _num(value: Any, digits: int = 2) -> str:
    if value is None:
        return "—"
    return f"{float(value):.{digits}f}"


def render_policy_v8_report(
    model_dir: str | Path = "models/policy_bc_v8",
    output_path: str | Path = "reports/policy_bc_v8.html",
) -> Path:
    target = Path(model_dir)
    if not target.is_absolute():
        target = ROOT / target
    report = _load(target / "report.json")
    history_path = target / "training_stages.json"
    history = _load(history_path) if history_path.exists() else list(report.get("history") or [])
    if isinstance(history, dict):
        history = list(history.get("history") or [])
    compute = report.get("compute") or {}
    data = report.get("data") or {}
    test = report.get("test") or {}
    val = report.get("val") or {}
    lessons = list(report.get("lessons") or [])
    splits = data.get("splits") or []
    train_battles = int(splits[0].get("battles") or 0) if splits else 0
    peak_vram = float(compute.get("peak_vram_mb") or 0.0)

    created = str(report.get("created_at") or datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"))
    payload = {
        "history": history,
        "test": test,
        "val": val,
        "compute": compute,
        "data": data,
        "lessons": lessons,
        "gates": {
            "card": {"now": test.get("slot_top1"), "low": 0.57, "high": 0.61, "base": 0.529},
            "tile": {"now": test.get("tile_class_acc"), "low": 0.20, "high": 0.24, "base": 0.147},
            "joint": {"now": test.get("joint_card_tile"), "low": 0.17, "high": 0.21, "base": 0.119},
            "timing": {"now": test.get("timing_mae"), "low": 1.35, "high": 1.55, "base": 1.79},
        },
    }
    esc = html_lib.escape
    page = f"""<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>PolicyBC v8 — causal belief policy</title>
{favicon_link()}
{FONT_LINKS}
<style>
{shared_styles()}
.blueprint {{display:grid;grid-template-columns:repeat(4,1fr);gap:10px;margin-top:12px}}
.mod {{border:1px dashed #355065;border-radius:14px;padding:14px;min-height:110px}}
.mod b {{display:block;color:var(--sky);font:700 11px/1.3 "IBM Plex Mono",monospace;letter-spacing:.08em;text-transform:uppercase;margin-bottom:8px}}
.formula {{font:13px/1.7 "IBM Plex Mono",ui-monospace,monospace;background:#08141c;border:1px solid var(--line);border-radius:12px;padding:16px;color:#9ee7c8;overflow:auto}}
.gate {{display:grid;grid-template-columns:170px 1fr 90px;gap:10px;align-items:center;margin:12px 0}}
.track {{height:22px;background:#15222c;border-radius:6px;position:relative;overflow:hidden}}
.fill {{height:100%;background:var(--sky);border-radius:6px}}
.band {{position:absolute;top:0;bottom:0;background:#70e1b133}}
.chart-tools {{display:flex;flex-wrap:wrap;gap:8px;margin-bottom:12px}}
.chart-tools button,.card button {{background:#12202a;color:var(--muted);border:1px solid var(--line);border-radius:999px;padding:6px 12px;cursor:pointer}}
.chart-tools button.on {{color:var(--ink);background:var(--sky);border-color:transparent}}
.grid2 {{display:grid;grid-template-columns:1fr 1fr;gap:14px}}
.tooltip {{position:fixed;z-index:9;background:#0b1620;border:1px solid #355065;padding:8px 10px;border-radius:8px;font-size:12px;pointer-events:none;display:none}}
.tooltip.on {{display:block}}
@media(max-width:800px){{.blueprint,.kpis,.grid2,.gate{{grid-template-columns:1fr}}}}
</style></head><body><main>
<header class="hero">
  <div class="eyebrow">Clash Royale AI · Astra Stage A</div>
  <h1>A memory that can wait.<br><em>Then play.</em></h1>
  <p class="lede">v8 replaces the GRU trunk with 24 latent memory tokens, a joint own/opponent event clock, and the compact selected-card heatmap prior. This is replay-only Stage A — board observations remain Stage B.</p>
  <div class="stamp">{esc(created)} · {esc(str(compute.get("device") or "cuda"))} · {_fmt_compact(int(compute.get("parameters") or 0))} params · {_fmt_duration(float(report.get("seconds") or 0))}</div>
</header>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">01 · scoreboard</div><h2>Held-out imitation, not win rate</h2></div><p>Causal observation time. Actor identity is still supplied by the target. Joint card+tile is the behavioral gate from the architecture report.</p></div>
  <div class="kpis">
    <div class="kpi" style="--tone:var(--sky)"><span>Card top-1</span><b>{_pct(test.get("slot_top1"))}</b><small>target 57–61%</small></div>
    <div class="kpi" style="--tone:var(--green)"><span>Tile top-1</span><b>{_pct(test.get("tile_class_acc"))}</b><small>target 20–24%</small></div>
    <div class="kpi" style="--tone:var(--gold)"><span>Card + tile</span><b>{_pct(test.get("joint_card_tile"))}</b><small>target 17–21%</small></div>
    <div class="kpi" style="--tone:var(--blue)"><span>Timing MAE</span><b>{_num(test.get("timing_mae"))}s</b><small>target 1.35–1.55s</small></div>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">02 · architecture</div><h2>Belief first, then the action prior</h2></div><p>Memory tokens are latent. None of them is advertised as a tracked living troop.</p></div>
  <div class="blueprint">
    <div class="mod"><b>events</b>64-step causal encoder with relative time and XY. Padding is masked.</div>
    <div class="mod"><b>memory</b>24 × 128 tokens. Friendly/enemy cross-attention, then two interaction blocks.</div>
    <div class="mod"><b>clock</b>Own and opponent rates. Survival of the quiet interval is a trained term.</div>
    <div class="mod"><b>action</b>Type → selected card → 18×32 heatmap + within-tile offset.</div>
  </div>
  <div class="formula" style="margin-top:16px">p(next @ t, side s, action a | H) = survival(t|H) × rate(s,t|H) × p(a | belief(t), s)<br>survival(t) = exp(−∫ Σ rate(s,u) du) · piecewise-constant on 250 ms</div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">03 · compute &amp; data</div><h2>Small enough to falsify</h2></div><p>Pilot split holds out the Astra probe cohort. Mixed precision on the local GPU.</p></div>
  <div class="kpis">
    <div class="kpi" style="--tone:var(--sky)"><span>Parameters</span><b>{_fmt_compact(int(compute.get("parameters") or 0))}</b><small>d={esc(str(compute.get("d_model")))} · mem={esc(str(compute.get("memory_tokens")))}</small></div>
    <div class="kpi" style="--tone:var(--green)"><span>Train samples</span><b>{_fmt_compact(int(data.get("train_samples") or 0))}</b><small>{train_battles:,} battles</small></div>
    <div class="kpi" style="--tone:var(--gold)"><span>Batch</span><b>{esc(str(compute.get("batch_size")))}</b><small>accum {esc(str(compute.get("gradient_accumulation_steps") or 1))} · amp {esc(str(compute.get("mixed_precision")).lower())}</small></div>
    <div class="kpi" style="--tone:var(--blue)"><span>Peak VRAM</span><b>{peak_vram:.0f} MB</b><small>{esc(str(compute.get("device") or ""))}</small></div>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">04 · investment gates</div><h2>Did Stage A earn more compute?</h2></div><p>Ranges are Astra's engineering targets against the causal v4.4.2 diagnostic, not confidence intervals.</p></div>
  <div id="gates"></div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">05 · curves</div><h2>Loss down, agreement up</h2></div><p>Toggle series. Hover a point. Replay draws the run.</p></div>
  <div class="card">
    <div class="chart-tools" id="metricButtons"></div>
    <button id="play" type="button">replay curves</button>
    <svg id="curve" viewBox="0 0 1040 360" role="img" aria-label="Training curves"></svg>
  </div>
</section>

<section class="section">
  <div class="section-head"><div><div class="eyebrow">06 · lessons</div><h2>What this run is allowed to claim</h2></div></div>
  <div class="grid2" id="lessons"></div>
</section>

<footer class="foot">Native HTML + SVG + JS · no matplotlib · Stage A replay-only · not a live Clash Royale readiness claim.</footer>
</main>
<div class="tooltip" id="tip"></div>
<script id="report-data" type="application/json">{_js(payload)}</script>
<script>
const D=JSON.parse(document.getElementById('report-data').textContent);
const $=s=>document.querySelector(s);
const pct=n=>n==null||n===undefined?'—':(100*n).toFixed(2)+'%';
const num=n=>n==null||n===undefined?'—':Number(n).toFixed(3);
const tip=$('#tip');
function showTip(e,html){{tip.innerHTML=html;tip.style.left=(e.clientX+14)+'px';tip.style.top=(e.clientY+14)+'px';tip.classList.add('on')}}
window.addEventListener('pointermove',()=>tip.classList.remove('on'));

const metrics={{
  train_loss:['Train loss','loss'],
  val_loss:['Val loss','loss'],
  val_slot_top1:['Card top-1','pct'],
  val_tile_class_acc:['Tile top-1','pct'],
  val_joint_card_tile:['Card+tile','pct'],
  val_timing_mae:['Timing MAE','sec'],
  val_clock_nll:['Clock NLL','loss']
}};
let active=new Set(['train_loss','val_loss']);
const hist=D.history||[];
const buttons=$('#metricButtons');
Object.entries(metrics).forEach(([key,meta])=>{{
  const b=document.createElement('button');
  b.textContent=meta[0];
  b.className=active.has(key)?'on':'';
  b.onclick=()=>{{if(active.has(key))active.delete(key);else active.add(key);b.className=active.has(key)?'on':'';draw(1)}};
  buttons.appendChild(b);
}});

function axis(v,kind){{
  if(kind==='pct')return (100*v).toFixed(0)+'%';
  if(kind==='sec')return v.toFixed(2)+'s';
  return v.toFixed(2);
}}
function draw(reveal=1){{
  const svg=$('#curve');
  const W=1040,H=360,p={{l:70,r:24,t:24,b:44}};
  const keys=[...active].filter(k=>hist.some(r=>Number.isFinite(r[k])));
  if(!keys.length||!hist.length){{svg.innerHTML='<text x="40" y="40" fill="#8aa3b3">No history yet</text>';return}}
  const rows=hist.slice(0,Math.max(1,Math.ceil(hist.length*reveal)));
  const vals=keys.flatMap(k=>rows.map(r=>r[k]).filter(Number.isFinite));
  let lo=Math.min(...vals),hi=Math.max(...vals),pad=(hi-lo||1)*.12;lo-=pad;hi+=pad;
  const x=i=>p.l+i/Math.max(rows.length-1,1)*(W-p.l-p.r);
  const y=v=>p.t+(hi-v)/(hi-lo)*(H-p.t-p.b);
  const colors={{train_loss:'#638bc4',val_loss:'#86ead2',val_slot_top1:'#70e1b1',val_tile_class_acc:'#ffca63',val_joint_card_tile:'#f29665',val_timing_mae:'#70a1ff',val_clock_nll:'#e8f58b'}};
  let out='';
  for(let i=0;i<5;i++){{const yy=p.t+i*(H-p.t-p.b)/4,v=hi-i*(hi-lo)/4;out+=`<line x1="${{p.l}}" y1="${{yy}}" x2="${{W-p.r}}" y2="${{yy}}" stroke="#1e3140"/><text x="${{p.l-10}}" y="${{yy+4}}" text-anchor="end" fill="#8aa3b3" font-size="12" font-family="IBM Plex Mono,monospace">${{axis(v,metrics[keys[0]][1])}}</text>`}}
  keys.forEach(k=>{{
    const pts=rows.map((r,i)=>Number.isFinite(r[k])?`${{x(i)}},${{y(r[k])}}`:null).filter(Boolean);
    out+=`<polyline fill="none" stroke="${{colors[k]||'#38bdf8'}}" stroke-width="2.4" points="${{pts.join(' ')}}"/>`;
    rows.forEach((r,i)=>{{if(!Number.isFinite(r[k]))return;out+=`<circle cx="${{x(i)}}" cy="${{y(r[k])}}" r="4" fill="${{colors[k]||'#38bdf8'}}" data-k="${{k}}" data-i="${{i}}"/>`}});
  }});
  rows.forEach((r,i)=>{{if(i===0||i===rows.length-1||i%Math.ceil(rows.length/8)===0)out+=`<text x="${{x(i)}}" y="${{H-16}}" text-anchor="middle" fill="#8aa3b3" font-size="12">${{r.epoch}}</text>`}});
  svg.innerHTML=out;
  svg.querySelectorAll('circle').forEach(c=>c.addEventListener('pointerenter',e=>{{
    const r=rows[Number(c.dataset.i)],k=c.dataset.k,kind=metrics[k][1];
    showTip(e, `<b>${{metrics[k][0]}}</b><br>epoch ${{r.epoch}} · ${{kind==='pct'?pct(r[k]):num(r[k])}}`);
    e.stopPropagation();
  }}));
}}
draw(1);
let timer=null;
$('#play').onclick=()=>{{
  if(timer){{clearInterval(timer);timer=null;$('#play').textContent='replay curves';return}}
  let f=0;$('#play').textContent='pause';
  timer=setInterval(()=>{{f=(f+1)%Math.max(hist.length,1)+1;draw(Math.min(1,f/Math.max(hist.length,1)));}},180);
}};

const labels={{card:'Next-card top-1',tile:'Placement-bin top-1',joint:'Card + tile',timing:'Timing MAE (lower better)'}};
Object.entries(D.gates).forEach(([key,g])=>{{
  const now=g.now, low=g.low, high=g.high;
  const max=key==='timing'?2.2:Math.max(high, now||0, g.base||0)*1.15;
  const row=document.createElement('div');
  row.className='gate';
  const nowPct=now==null?0:100*(now/max);
  row.innerHTML=`<div>${{labels[key]}}</div><div class="track"><div class="band" style="left:${{100*(low/max)}}%;width:${{100*((high-low)/max)}}%"></div><div class="fill" style="width:${{nowPct}}%"></div></div><div class="mono">${{key==='timing'?num(now)+'s':pct(now)}}</div>`;
  $('#gates').appendChild(row);
}});
(D.lessons||[]).forEach((text,i)=>{{
  const card=document.createElement('div');
  card.className='card';
  card.innerHTML=`<div class="eyebrow">${{String(i+1).padStart(2,'0')}} / note</div><p>${{text}}</p>`;
  $('#lessons').appendChild(card);
}});
</script>
</body></html>
"""
    out = Path(output_path)
    if not out.is_absolute():
        out = ROOT / out
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(page, encoding="utf-8")
    return out
