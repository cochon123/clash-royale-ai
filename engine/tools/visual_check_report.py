"""HTML report for tools/visual_check.py: native SVG/canvas, no plotting libs.

One self-contained page per run. Images (frames, filmstrips, thumbnails) are
referenced relative to the run directory, so the folder can be moved whole.
"""
from __future__ import annotations

import html
import json
import os
from pathlib import Path


def _json(value):
    return json.dumps(value, separators=(",", ":")).replace("</", "<\\/")


def write_report(run_dir: Path, report: dict):
    views = {}
    for scenario in report["scenarios"]:
        path = run_dir / scenario["name"] / "view.json"
        if scenario["images"] and path.exists():
            views[scenario["name"]] = json.loads(path.read_text())
    baseline_rel = None
    if report.get("baseline"):
        baseline_rel = os.path.relpath(report["baseline"], run_dir)
    page = TEMPLATE.replace("__TITLE__", html.escape(f"Arena visual check — {report['verdict']}"))
    page = page.replace("__REPORT__", _json(report))
    page = page.replace("__VIEWS__", _json(views))
    page = page.replace("__BASELINE__", _json(baseline_rel))
    (run_dir / "report.html").write_text(page)


TEMPLATE = r"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>__TITLE__</title>
<style>
:root{color-scheme:dark;--bg:#071018;--panel:#0d171f;--panel2:#101d27;--line:#1e3140;--soft:#15222c;--text:#edf4f8;
--muted:#8aa3b3;--sky:#38bdf8;--green:#70e1b1;--gold:#ffca63;--blue:#70a1ff;--red:#ff7e78;--violet:#c792ea}
*{box-sizing:border-box}html{scroll-behavior:smooth}
body{margin:0;color:var(--text);font:15px/1.55 "IBM Plex Sans",ui-sans-serif,system-ui,sans-serif;
background:radial-gradient(circle at 12% -8%,#16384a 0,transparent 34%),radial-gradient(circle at 90% 0,#1d3f32 0,transparent 28%),var(--bg)}
main{width:min(1280px,calc(100% - 36px));margin:auto;padding:40px 0 90px}
a{color:var(--sky)}h1{font-size:clamp(2rem,4.6vw,3.2rem);line-height:1;letter-spacing:-.04em;margin:12px 0 16px}
h2{font-size:1.6rem;letter-spacing:-.03em;margin:44px 0 10px}h3{font-size:17px;margin:18px 0 8px}
p,li{color:var(--muted)}code{font:13px "IBM Plex Mono",ui-monospace,monospace;color:#cfe8f7}
.eyebrow{font:700 11px/1.2 "IBM Plex Mono",ui-monospace,monospace;letter-spacing:.16em;text-transform:uppercase;color:var(--sky)}
.panel{background:linear-gradient(180deg,var(--panel2),var(--panel));border:1px solid var(--line);border-radius:14px;padding:18px 20px;margin:14px 0}
.verdict{display:inline-flex;gap:10px;align-items:center;font:700 20px "IBM Plex Mono",monospace;padding:8px 16px;border-radius:10px}
.verdict.PASS{background:#123a2b;color:var(--green);border:1px solid #2f7d5d}.verdict.FAIL{background:#3a1616;color:var(--red);border:1px solid #7d2f2f}
.meta{display:grid;grid-template-columns:repeat(auto-fit,minmax(220px,1fr));gap:10px;margin-top:14px}
.meta div{background:var(--soft);border:1px solid var(--line);border-radius:10px;padding:10px 12px}
.meta b{display:block;font:600 11px "IBM Plex Mono",monospace;color:var(--muted);text-transform:uppercase;letter-spacing:.1em}
table{width:100%;border-collapse:collapse;font-size:14px}th,td{padding:8px 10px;border-bottom:1px solid var(--line);text-align:left;vertical-align:top}
th{color:var(--muted);font:600 12px "IBM Plex Mono",monospace;text-transform:uppercase;letter-spacing:.08em}
tbody tr:hover{background:#12222e}
.n-error{color:var(--red);font-weight:700}.n-warning{color:var(--gold)}.n-zero{color:#4f6676}
.chip{display:inline-block;padding:2px 9px;border-radius:999px;border:1px solid var(--line);font:12px "IBM Plex Mono",monospace;cursor:pointer;margin:2px;user-select:none;transition:.15s}
.chip:hover{border-color:var(--sky)}.chip.off{opacity:.35}
.sev-error{background:#3a1616;color:var(--red);border-color:#6b2a2a}.sev-warning{background:#3a2e12;color:var(--gold);border-color:#6b5520}
.st-regressed,.st-new{background:#3a1630;color:#ff8ad8}.st-fixed{background:#123a2b;color:var(--green)}.st-persisting{color:var(--muted)}.st-unbaselined{color:var(--muted)}
.filters{display:flex;flex-wrap:wrap;gap:6px;align-items:center;margin-bottom:10px}
.filters input{background:var(--soft);border:1px solid var(--line);color:var(--text);border-radius:8px;padding:6px 10px;min-width:240px}
.cards{display:grid;gap:10px;max-height:1100px;overflow:auto;padding-right:6px}
.card{border:1px solid var(--line);border-left:4px solid var(--muted);border-radius:10px;padding:10px 14px;background:var(--panel);transition:.15s}
.card:hover{border-color:var(--sky);transform:translateY(-1px)}
.card.error{border-left-color:var(--red)}.card.warning{border-left-color:var(--gold)}
.card .msg{color:var(--text);margin:6px 0}.card img{max-width:100%;border-radius:6px;margin-top:6px;cursor:zoom-in;border:1px solid var(--line)}
.card .where{font:12px "IBM Plex Mono",monospace;color:var(--muted)}
.tip{position:fixed;pointer-events:none;background:#04090dee;border:1px solid var(--sky);border-radius:8px;padding:8px 10px;font:12px/1.45 "IBM Plex Mono",monospace;color:var(--text);z-index:20;display:none;max-width:420px;white-space:pre-wrap}
.scen{border-top:1px solid var(--line);padding-top:10px}
.viewer{display:grid;grid-template-columns:minmax(300px,420px) 1fr;gap:18px;align-items:start}
@media(max-width:900px){.viewer{grid-template-columns:1fr}}
.stage{position:relative;width:100%;aspect-ratio:540/900;background:#000;border-radius:10px;overflow:hidden;border:1px solid var(--line)}
.stage img,.stage canvas{position:absolute;inset:0;width:100%;height:100%}
.controls{display:flex;flex-wrap:wrap;gap:8px;align-items:center;margin:10px 0}
.controls button,.controls select{background:var(--soft);color:var(--text);border:1px solid var(--line);border-radius:8px;padding:5px 10px;cursor:pointer}
.controls button:hover{border-color:var(--sky)}.controls input[type=range]{flex:1;min-width:160px;accent-color:var(--sky)}
.clock{font:13px "IBM Plex Mono",monospace;color:var(--sky);min-width:150px}
.legend{display:flex;flex-wrap:wrap;gap:6px;margin:6px 0}
svg text{font:11px "IBM Plex Mono",monospace;fill:var(--muted)}
.lane-label{cursor:pointer}.lane-label:hover{fill:var(--sky)}
.strips img{max-width:100%;border-radius:8px;border:1px solid var(--line);margin:6px 0}
details>summary{cursor:pointer;color:var(--sky);margin:8px 0}
.explain{font-size:13px;color:var(--muted);border-left:2px solid var(--line);padding-left:10px;margin:8px 0}
.kv{font:12px "IBM Plex Mono",monospace;color:var(--muted);white-space:pre-line}
.lightbox{position:fixed;inset:0;background:#000d;display:none;align-items:center;justify-content:center;z-index:30;cursor:zoom-out}
.lightbox img{max-width:96vw;max-height:94vh}
</style>
</head>
<body>
<div class="tip" id="tip"></div>
<div class="lightbox" id="lightbox"><img alt=""></div>
<main id="app"></main>
<script id="report-data" type="application/json">__REPORT__</script>
<script id="view-data" type="application/json">__VIEWS__</script>
<script id="baseline-data" type="application/json">__BASELINE__</script>
<script>
const R = JSON.parse(document.getElementById('report-data').textContent);
const VIEWS = JSON.parse(document.getElementById('view-data').textContent);
const BASE = JSON.parse(document.getElementById('baseline-data').textContent);
const STATE_COLORS = {idle:'#8aa3b3', moving:'#70a1ff', attacking:'#ffca63', deploying:'#c792ea', none:'#4f6676'};
const SERIES = ['#38bdf8','#ff7e78','#70e1b1','#ffca63','#c792ea','#f78c6c','#89ddff','#c3e88d'];
const $ = (tag, attrs={}, ...kids) => { const e = document.createElement(tag);
  for (const [k,v] of Object.entries(attrs)) { if (k==='class') e.className=v; else if (k==='text') e.textContent=v;
    else if (k.startsWith('on')) e.addEventListener(k.slice(2), v); else e.setAttribute(k, v); }
  for (const k of kids.flat()) if (k!=null) e.append(k.nodeType ? k : document.createTextNode(k)); return e; };
const svgEl = (tag, attrs={}) => { const e = document.createElementNS('http://www.w3.org/2000/svg', tag);
  for (const [k,v] of Object.entries(attrs)) e.setAttribute(k, v); return e; };
const tip = document.getElementById('tip');
function showTip(ev, text){ tip.textContent=text; tip.style.display='block';
  tip.style.left=Math.min(ev.clientX+14, innerWidth-440)+'px'; tip.style.top=(ev.clientY+14)+'px'; }
function hideTip(){ tip.style.display='none'; }
function hoverTip(el, text){ el.addEventListener('mousemove', ev=>showTip(ev, typeof text==='function'?text(ev):text)); el.addEventListener('mouseleave', hideTip); }
const lightbox = document.getElementById('lightbox');
function zoom(src){ lightbox.querySelector('img').src=src; lightbox.style.display='flex'; }
lightbox.addEventListener('click', ()=> lightbox.style.display='none');
const pad = i => String(i).padStart(5,'0');
const viewers = {};

const app = document.getElementById('app');
const allFindings = R.scenarios.flatMap(s => s.findings.map(f => ({...f, scenario:s.name})));
const counts = sev => allFindings.filter(f=>f.severity===sev).length;

// ---------------------------------------------------------------- header
app.append($('div',{class:'eyebrow',text:'Arena renderer · deterministic visual QA'}),
  $('h1',{}, 'Visual check ', $('span',{class:`verdict ${R.verdict}`, text:R.verdict})),
  $('p',{}, R.reasons.length ? R.reasons.join(' · ') : (R.baseline ? 'No new or worsened error findings and no unexpected drawing changes against the baseline. Existing findings remain listed below.' : 'No error findings.')));
const meta = $('div',{class:'meta'});
const metaItem = (k, v, title) => { const d=$('div',{},$('b',{text:k}), v); if (title) hoverTip(d, title); meta.append(d); };
metaItem('Code', `${R.git.branch} @ ${R.git.commit}${R.git.dirty.length?` + ${R.git.dirty.length} changed file(s)`:''}`,
  `renderer source fingerprint ${R.git.source}\n${R.git.dirty.join('\n') || 'clean'}`);
metaItem('Created', R.created);
metaItem('Suite', `${R.suite} · ${R.scenarios.length} scenario(s) · ${R.seconds}s`, 'Wall time including replay, analysis and screenshots.');
metaItem('Frames judged', String(R.scenarios.reduce((a,s)=>a+s.summary.frames,0)), 'Display frames on an exact grid (20 per second unless a scenario sets fps).');
metaItem('Findings', `${counts('error')} errors · ${counts('warning')} warnings`);
metaItem('Baseline', R.baseline ? `${R.baseline_report?.commit||'?'} · source ${R.baseline_report?.source||'?'}` : 'none', R.baseline || 'Run `visual_check.py accept` to make a run the reference.');
metaItem('Expected change', R.expect==null ? 'not declared' : (R.expect.length? R.expect.join(', ') : 'none (no visual change allowed)'),
  'Rows this change is meant to affect (--expect). Any drawing change outside them fails the run.');
metaItem('Assets', R.assets.split('/').slice(-3).join('/'), R.assets);
app.append(meta);

app.append($('div',{class:'panel'},
  $('h3',{text:'How this check works'}),
  $('p',{}, 'Recorded original-engine matches are replayed through the real presenter on an exact time grid, so the same code always draws the same pixels. Each display frame records the display list the browser would receive and what the Painter chose for every entity (clip, frame, alpha, bounds, effects, inferred hits).'),
  $('p',{}, $('b',{text:'Rules '}), 'derived from original game data (logic rows, authored clips and effects) and from continuity judge every frame. ',
    $('b',{text:'Baseline comparison '}), 'shows new/fixed findings and every frame whose drawing changed, attributed to rows. ',
    $('b',{text:'Screenshots '}), '(viewer, filmstrips, thumbnails) support visual review. These checks detect regressions; they do not prove parity with the official client. Visible mass = Σ texture alpha × draw opacity of an entity\'s sprites, in full-alpha pixels.')));

// ---------------------------------------------------------------- summary
app.append($('h2',{text:'Scenarios'}));
const st = $('table',{}, $('thead',{}, $('tr',{}, ...['Scenario','Window','Frames','Errors','Warnings','New/worse','Changed frames','Rows changed','Time'].map(h=>$('th',{text:h})))));
const tb = $('tbody'); st.append(tb);
for (const s of R.scenarios) {
  const e = s.findings.filter(f=>f.severity==='error').length, w = s.findings.filter(f=>f.severity==='warning').length;
  const nw = s.findings.filter(f=>f.status==='new'||f.status==='regressed').length;
  const c = s.comparison; const changed = c ? c.changed_frames.length : null;
  const tr = $('tr',{},
    $('td',{}, $('a',{href:`#scen-${s.name}`, text:s.name})),
    $('td',{class:'kv', text:`${s.summary.from_tick}–${s.summary.to_tick}`}),
    $('td',{text:s.summary.frames}),
    $('td',{class:e?'n-error':'n-zero', text:e}), $('td',{class:w?'n-warning':'n-zero', text:w}),
    $('td',{class:nw?'n-error':'n-zero', text:nw}),
    $('td',{class:changed?'':'n-zero', text: changed==null ? '—' : `${changed} / ${c.frames}`}),
    $('td',{class:'kv', text: c ? Object.entries(c.rows).slice(0,4).map(([r,n])=>`${r} ${n}`).join(', ') : '—'}),
    $('td',{class:'kv', text:`${s.seconds}s`}));
  hoverTip(tr, s.scenario.why || s.name);
  tb.append(tr);
}
app.append($('div',{class:'panel'}, st));

// ---------------------------------------------------------------- findings
app.append($('h2',{text:'Findings'}));
if (R.fixed && R.fixed.length) app.append($('div',{class:'panel'}, $('h3',{text:`Fixed since baseline (${R.fixed.length})`}),
  ...R.fixed.map(f => $('div',{class:'kv'}, $('span',{class:'chip st-fixed',text:'fixed'}), ` ${f.severity} ${f.rule} — ${f.message}`))));
if (R.unexpected_changes && Object.keys(R.unexpected_changes).length) app.append($('div',{class:'panel'},
  $('h3',{text:'Unexpected drawing changes'}),
  $('p',{text:'These rows changed although --expect did not list them.'}),
  ...Object.entries(R.unexpected_changes).map(([s,rows]) => $('div',{class:'kv', text:`${s}: ${Object.entries(rows).map(([r,n])=>`${r} (${n} frames)`).join(', ')}`}))));
const filt = {severity:new Set(['error','warning']), status:new Set(['new','regressed','persisting','unbaselined']), rule:new Set(Object.keys(R.rules)), text:''};
const fbar = $('div',{class:'filters'});
const cards = $('div',{class:'cards'});
const chipSet = (key, values, cls) => values.forEach(v => { const c=$('span',{class:`chip ${cls(v)}`, text:v});
  if (key==='rule') hoverTip(c, `${R.rules[v].severity}: ${R.rules[v].what}\nbasis: ${R.rules[v].basis}`);
  c.onclick=()=>{ filt[key].has(v)?filt[key].delete(v):filt[key].add(v); c.classList.toggle('off'); renderCards(); }; fbar.append(c); });
chipSet('severity', ['error','warning'], v=>`sev-${v}`);
chipSet('status', ['new','regressed','persisting','unbaselined'], v=>`st-${v}`);
const rulesSeen = [...new Set(allFindings.map(f=>f.rule))];
chipSet('rule', rulesSeen, ()=> '');
const search = $('input',{placeholder:'filter text (row, scenario, export…)'}); search.oninput=()=>{filt.text=search.value.toLowerCase(); renderCards();};
fbar.append(search);
function renderCards(){
  cards.replaceChildren();
  const shown = allFindings.filter(f => filt.severity.has(f.severity) && filt.status.has(f.status) && filt.rule.has(f.rule)
    && (!filt.text || JSON.stringify(f).toLowerCase().includes(filt.text)));
  if (!shown.length) cards.append($('p',{text:'No findings match these filters.'}));
  for (const f of shown.slice(0,300)) {
    const rule = $('span',{class:'chip', text:f.rule}); hoverTip(rule, `${R.rules[f.rule].what}\nbasis: ${R.rules[f.rule].basis}`);
    const card = $('div',{class:`card ${f.severity}`},
      $('div',{}, $('span',{class:`chip sev-${f.severity}`, text:f.severity}), $('span',{class:`chip st-${f.status}`, text:f.status}), rule,
        $('span',{class:'where', text:` ${f.scenario} · t=${(f.first_t/20).toFixed(2)}s · ${f.count}× ${f.baseline_count!=null?`(baseline ${f.baseline_count}×)`:''}`})),
      $('div',{class:'msg', text:f.message}));
    if (f.thumbnail) { const img=$('img',{src:`${f.scenario}/${f.thumbnail}`, loading:'lazy', alt:f.rule});
      img.onclick=()=>zoom(img.src); card.append(img); }
    if (f.frame!=null && viewers[f.scenario]) card.append($('div',{}, $('a',{href:`#scen-${f.scenario}`, text:'open in viewer →',
      onclick:()=>viewers[f.scenario].seek(f.frame, f.uid)})));
    cards.append(card);
  }
}
app.append($('div',{class:'panel'}, fbar, cards));

// ---------------------------------------------------------------- scenarios
for (const s of R.scenarios) {
  const sec = $('section',{class:'scen', id:`scen-${s.name}`});
  sec.append($('h2',{text:s.name}), $('p',{text:s.scenario.why||''}),
    $('div',{class:'kv', text:`capture ${s.scenario.capture} · ticks ${s.summary.from_tick}–${s.summary.to_tick} · ${s.summary.fps} fps · lag ${s.summary.lag} · mean draw ${s.summary.mean_draw_ms.toFixed(1)} ms · ${s.summary.textures} textures`}));
  if (s.summary.unresolved_rows.length) sec.append($('div',{class:'kv', text:`unresolved rows: ${s.summary.unresolved_rows.join(', ')}`}));
  const view = VIEWS[s.name];
  if (view) sec.append(scenarioViewer(s, view));
  else sec.append($('p',{text:'Data-only scenario (no screenshots). Use `visual_check.py show` on an image scenario, or add a focused window to scenarios.json.'}));
  if (s.comparison) sec.append(diffPanel(s));
  if (s.filmstrips.length) {
    const d = $('details',{}, $('summary',{text:`Filmstrips (${s.filmstrips.length})`}));
    d.append($('p',{class:'explain', text:'Each tile: time, state, authored frame index; clip action and visible mass as % of the entity median. Tiles are sampled evenly plus every state/clip change and the frame before it.'}));
    for (const f of s.filmstrips) { const img=$('img',{src:`${s.name}/${f.file}`, loading:'lazy', alt:f.row}); img.onclick=()=>zoom(img.src);
      d.append($('h3',{text:`${f.row} #${f.uid} (side ${f.side})`}), $('div',{class:'strips'}, img)); }
    sec.append(d);
  }
  app.append(sec);
}
renderCards();
if (location.hash) setTimeout(() => document.getElementById(decodeURIComponent(location.hash.slice(1)))?.scrollIntoView(), 50);

function diffPanel(s){
  const c = s.comparison, p = $('div',{class:'panel'}, $('h3',{text:`Display-list changes vs baseline: ${c.changed_frames.length} of ${c.frames} frames`}));
  if (!c.changed_frames.length) { p.append($('p',{text:'Identical display lists in every frame: pixel-identical to the baseline.'})); return p; }
  const w = 1000, h = 26, svg = svgEl('svg',{viewBox:`0 0 ${w} ${h}`, width:'100%', height:h});
  svg.append(svgEl('rect',{x:0,y:0,width:w,height:h,fill:'#15222c'}));
  for (const i of c.changed_frames) svg.append(svgEl('rect',{x:i/c.frames*w, y:2, width:Math.max(1.5,w/c.frames), height:h-4, fill:'#ff8ad8'}));
  p.append($('div',{class:'explain', text:'Pink = frames whose display list differs from the baseline (any texture, transform, opacity or draw).'}), svg);
  const t = $('table',{}, $('thead',{}, $('tr',{}, $('th',{text:'Row'}), $('th',{text:'Frames'}), $('th',{text:'What changed (probe)'}))));
  const b = $('tbody'); t.append(b);
  for (const [row,n] of Object.entries(c.rows)) b.append($('tr',{}, $('td',{text:row}), $('td',{text:n}),
    $('td',{class:'kv', text:Object.entries(c.details[row]||{}).map(([d,k])=>`${d} ×${k}`).join('\n')||'—'})));
  p.append(t); return p;
}

function scenarioViewer(s, view){
  const frames = view.frames, T = view.timelines, n = frames.length;
  const focus = new Set(s.scenario.focus||[]);
  const findingsAt = {}; s.findings.forEach(f => (f.occurrences||[]).forEach(o => { if (o.frame!=null) (findingsAt[o.frame] ||= []).push(f); }));
  const wrap = $('div',{class:'panel'});
  const stage = $('div',{class:'stage'}), img = $('img',{alt:'frame'}), canvas = $('canvas',{width:540,height:900});
  stage.append(img, canvas);
  const slider = $('input',{type:'range', min:0, max:n-1, value:0});
  const clock = $('span',{class:'clock'});
  const speed = $('select',{}, ...[0.1,0.25,0.5,1].map(v=>$('option',{value:v, text:`${v}×`})));
  speed.value = '0.25';
  let cur = 0, playing = false, timer = null, showBoxes = true, showLabels = true, focusOnly = false, source = 'current', blink = null, selected = null;
  const play = $('button',{text:'▶ play'});
  const toggles = $('span',{class:'legend'});
  const tog = (label, get, set) => { const c=$('span',{class:'chip', text:label}); c.onclick=()=>{ set(!get()); c.classList.toggle('off', !get()); draw(); }; toggles.append(c); return c; };
  tog('boxes', ()=>showBoxes, v=>showBoxes=v); tog('labels', ()=>showLabels, v=>showLabels=v);
  tog('focus rows only', ()=>focusOnly, v=>focusOnly=v).classList.add('off');
  const srcSel = $('select',{}, $('option',{value:'current',text:'current'}));
  if (BASE && s.comparison) { srcSel.append($('option',{value:'baseline',text:'baseline'}), $('option',{value:'blink',text:'blink current/baseline'})); }
  srcSel.onchange = () => { source = srcSel.value; clearInterval(blink); blink=null;
    if (source==='blink') { let on=false; blink=setInterval(()=>{on=!on; img.src=frameSrc(on?'baseline':'current');}, 450); } draw(); };
  function frameSrc(which){ const i=frames[cur].i; return which==='baseline' ? `${BASE}/${s.name}/frames/${pad(i)}.jpg` : `${s.name}/frames/${pad(i)}.jpg`; }
  const controls = $('div',{class:'controls'},
    $('button',{text:'⏮', onclick:()=>seek(cur-1)}), play, $('button',{text:'⏭', onclick:()=>seek(cur+1)}), slider, clock, speed, srcSel);
  play.onclick = () => { playing=!playing; play.textContent = playing?'⏸ pause':'▶ play';
    clearInterval(timer); if (playing) timer=setInterval(()=>{ seek(cur+1>=n?0:cur+1); }, 1000/s.summary.fps/parseFloat(speed.value)); };
  speed.onchange = () => { if (playing) { playing=false; play.onclick(); } };
  slider.oninput = () => seek(+slider.value);
  const ctx = canvas.getContext('2d');
  function boxes(){ return frames[cur].e.filter(e => e[7] && (!focusOnly || focus.has(e[1]))); }
  function draw(){
    const fr = frames[cur]; if (source!=='blink') img.src = frameSrc(source);
    slider.value = cur; clock.textContent = `frame ${fr.i} · tick ${fr.t.toFixed(2)} · ${(fr.t/20).toFixed(2)}s`;
    ctx.clearRect(0,0,540,900);
    const bad = new Set((findingsAt[fr.i]||[]).map(f=>f.uid));
    if (showBoxes) for (const [uid,row,side,state,exp,fi,alpha,bb] of boxes()) {
      ctx.strokeStyle = bad.has(uid) ? '#ff4d4d' : (uid===selected ? '#ffffff' : STATE_COLORS[state]||'#888');
      ctx.lineWidth = bad.has(uid)||uid===selected ? 2.5 : 1.2; ctx.strokeRect(bb[0]+.5,bb[1]+.5,bb[2]-bb[0],bb[3]-bb[1]);
      if (showLabels && (focus.has(row) || bad.has(uid))) { ctx.font='11px monospace'; ctx.fillStyle='#000a';
        const label = `${row}#${uid} ${state}`; ctx.fillRect(bb[0], bb[1]-13, ctx.measureText(label).width+6, 13);
        ctx.fillStyle = bad.has(uid)?'#ff9a9a':'#e6f4ff'; ctx.fillText(label, bb[0]+3, bb[1]-3); }
    }
    playhead.forEach(ph => { const x = xOf(cur); ph.setAttribute('x1',x); ph.setAttribute('x2',x); });
  }
  function seek(i, uid){ cur = Math.max(0, Math.min(n-1, i)); if (uid!=null) selected = uid; draw(); }
  canvas.addEventListener('mousemove', ev => { const r = canvas.getBoundingClientRect();
    const x = (ev.clientX-r.left)/r.width*540, y = (ev.clientY-r.top)/r.height*900;
    const hit = boxes().filter(b => x>=b[7][0]&&x<=b[7][2]&&y>=b[7][1]&&y<=b[7][3]);
    if (!hit.length) return hideTip();
    showTip(ev, hit.map(([uid,row,side,state,exp,fi,alpha]) => {
      const fs = (findingsAt[frames[cur].i]||[]).filter(f=>f.uid===uid).map(f=>`  ✖ ${f.rule}`).join('\n');
      return `${row} #${uid} side ${side}\n${state} · ${exp} · frame ${fi} · alpha ${alpha}${fs?'\n'+fs:''}`; }).join('\n\n')); });
  canvas.addEventListener('mouseleave', hideTip);
  canvas.addEventListener('click', ev => { const r = canvas.getBoundingClientRect();
    const x = (ev.clientX-r.left)/r.width*540, y = (ev.clientY-r.top)/r.height*900;
    const hit = boxes().find(b => x>=b[7][0]&&x<=b[7][2]&&y>=b[7][1]&&y<=b[7][3]); selected = hit ? hit[0] : null; draw(); });

  // Timeline lanes: one per unit, colored by state, faint frames hatched.
  const uids = Object.keys(T).map(Number).sort((a,b) => {
    const fa = focus.has(T[a].row)?0:(T[a].kind==='unit'?1:2), fb = focus.has(T[b].row)?0:(T[b].kind==='unit'?1:2);
    return fa-fb || T[a].row.localeCompare(T[b].row) || a-b; }).slice(0, 32);
  const W = 1000, LX = 150, laneH = 16, top = 22, H = top + (uids.length+1)*laneH + 8;
  const xOf = i => LX + i/(Math.max(1,n-1))*(W-LX-8);
  const tl = svgEl('svg',{viewBox:`0 0 ${W} ${H}`, width:'100%'});
  const defs = svgEl('defs'); defs.innerHTML = '<pattern id="hatch" width="4" height="4" patternUnits="userSpaceOnUse" patternTransform="rotate(45)"><line x1="0" y1="0" x2="0" y2="4" stroke="#071018" stroke-width="2"/></pattern>';
  tl.append(defs);
  for (let k=0;k<=10;k++){ const i=Math.round(k*(n-1)/10), x=xOf(i); tl.append(svgEl('line',{x1:x,x2:x,y1:12,y2:H,stroke:'#15222c'}));
    const t=svgEl('text',{x:x-14,y:10}); t.textContent=`${(frames[i].t/20).toFixed(1)}s`; tl.append(t); }
  const sceneY = top; const sl = svgEl('text',{x:4,y:sceneY+11}); sl.textContent='scene findings'; tl.append(sl);
  s.findings.filter(f=>f.uid==null && f.frame!=null).forEach(f => { const x=xOf(f.frame); const m=svgEl('text',{x:x-4,y:sceneY+12,fill:'#ff7e78'}); m.textContent='✖'; m.style.fill='#ff7e78';
    hoverTip(m, `${f.rule}: ${f.message}`); tl.append(m); });
  const frameIndex = {}; frames.forEach((f,k)=>frameIndex[f.i]=k);
  uids.forEach((uid, li) => { const lane = T[uid], y = top + (li+1)*laneH;
    const label = svgEl('text',{x:4,y:y+11,class:'lane-label'}); label.textContent = `${lane.row}#${uid}${lane.side?' R':' B'}`;
    if (focus.has(lane.row)) label.style.fill = '#edf4f8';
    label.addEventListener('click', ()=>{ selected=uid; draw(); }); tl.append(label);
    const masses = lane.states.map(s=>s[3]).filter(m=>m>0).sort((a,b)=>a-b); const med = masses[Math.floor(masses.length/2)]||1;
    let runStart = 0;
    const st = lane.states;
    for (let k=1;k<=st.length;k++) {
      const faint = j => st[j][3] < 0.6*med;
      if (k<st.length && st[k][1]===st[runStart][1] && faint(k)===faint(runStart) && frameIndex[st[k][0]]===frameIndex[st[k-1][0]]+1) continue;
      const a = frameIndex[st[runStart][0]], b = frameIndex[st[k-1][0]];
      const r = svgEl('rect',{x:xOf(a), y:y+2, width:Math.max(1.2, xOf(b+1)-xOf(a)), height:laneH-5, fill:STATE_COLORS[st[runStart][1]]||'#888', rx:2});
      const info = st.slice(runStart, k); hoverTip(r, ev => { const rr = tl.getBoundingClientRect(); const fx = (ev.clientX-rr.left)/rr.width*W;
        const kk = Math.round((fx-LX)/(W-LX-8)*(n-1)); const row = info.find(q=>frameIndex[q[0]]===kk) || info[0];
        return `${lane.row} #${uid}\nframe ${row[0]} · ${row[1]}\n${row[2]}\nvisible mass ${row[3]} (${Math.round(100*row[3]/med)}% of median) · alpha ${row[4]}`; });
      r.addEventListener('click', ev => { const rr = tl.getBoundingClientRect(); const fx = (ev.clientX-rr.left)/rr.width*W; seek(Math.round((fx-LX)/(W-LX-8)*(n-1)), uid); });
      tl.append(r);
      if (faint(runStart)) tl.append(svgEl('rect',{x:xOf(a), y:y+2, width:Math.max(1.2, xOf(b+1)-xOf(a)), height:laneH-5, fill:'url(#hatch)', 'pointer-events':'none'}));
      runStart = k;
    }
    lane.impacts.forEach(tk => { const k = frames.findIndex(f=>f.t>=tk); if (k<0) return; const x = xOf(k);
      const m = svgEl('path',{d:`M${x-3},${y} L${x+3},${y} L${x},${y+5} Z`, fill:'#fff'}); hoverTip(m, `hit at tick ${tk}`); tl.append(m); });
    s.findings.filter(f=>f.uid===uid).forEach(f => (f.occurrences||[]).forEach(o => { if (o.frame==null || frameIndex[o.frame]==null) return;
      const x = xOf(frameIndex[o.frame]); const m = svgEl('text',{x:x-4,y:y+12}); m.textContent='✖'; m.style.fill='#ff4d4d'; m.style.cursor='pointer';
      hoverTip(m, `${f.severity} ${f.rule}\n${f.message}`); m.addEventListener('click',()=>seek(frameIndex[o.frame], uid)); tl.append(m); }));
  });
  const playhead = [svgEl('line',{x1:LX,x2:LX,y1:12,y2:H,stroke:'#38bdf8','stroke-width':1.5})]; tl.append(playhead[0]);
  const legend = $('div',{class:'legend'}, ...Object.entries(STATE_COLORS).slice(0,4).map(([k,c])=>$('span',{class:'chip', style:`border-color:${c};color:${c}`, text:k})),
    $('span',{class:'chip', text:'▨ faint (<60% of median mass)'}), $('span',{class:'chip', text:'▼ hit'}), $('span',{class:'chip', style:'color:#ff4d4d', text:'✖ finding'}));

  // Visible-mass curves of focus entities, toggleable.
  const chartUids = uids.filter(u => focus.has(T[u].row));
  const CH = 200, chart = svgEl('svg',{viewBox:`0 0 ${W} ${CH+24}`, width:'100%'});
  const cLegend = $('div',{class:'legend'});
  if (chartUids.length) {
    const yOf = p => 6 + (1 - Math.min(p,160)/160)*CH;
    [0,50,100,150].forEach(p => { chart.append(svgEl('line',{x1:LX,x2:W-8,y1:yOf(p),y2:yOf(p),stroke:'#15222c'}));
      const t=svgEl('text',{x:LX-36,y:yOf(p)+4}); t.textContent=`${p}%`; chart.append(t); });
    chartUids.forEach((uid, k) => { const lane = T[uid];
      const masses = lane.states.map(s=>s[3]).filter(m=>m>0).sort((a,b)=>a-b); const med = masses[Math.floor(masses.length/2)]||1;
      const pts = lane.states.map(q => `${xOf(frameIndex[q[0]]).toFixed(1)},${yOf(100*q[3]/med).toFixed(1)}`).join(' ');
      const line = svgEl('polyline',{points:pts, fill:'none', stroke:SERIES[k%SERIES.length], 'stroke-width':1.6}); chart.append(line);
      const chip = $('span',{class:'chip', style:`color:${SERIES[k%SERIES.length]};border-color:${SERIES[k%SERIES.length]}`, text:`${lane.row}#${uid}`});
      chip.onclick = () => { chip.classList.toggle('off'); line.style.display = chip.classList.contains('off') ? 'none' : ''; };
      cLegend.append(chip); });
    const cross = svgEl('line',{x1:LX,x2:LX,y1:4,y2:CH+8,stroke:'#38bdf8','stroke-dasharray':'3 3'}); chart.append(cross); playhead.push(cross);
    chart.addEventListener('mousemove', ev => { const rr = chart.getBoundingClientRect(); const fx = (ev.clientX-rr.left)/rr.width*W;
      const kk = Math.max(0, Math.min(n-1, Math.round((fx-LX)/(W-LX-8)*(n-1))));
      showTip(ev, `${(frames[kk].t/20).toFixed(2)}s\n` + chartUids.map(u => { const q = T[u].states.find(q=>frameIndex[q[0]]===kk);
        return `${T[u].row}#${u}: ${q ? `${q[1]} ${q[3]} px · ${q[2]}` : 'not drawn'}`; }).join('\n')); });
    chart.addEventListener('mouseleave', hideTip);
    chart.addEventListener('click', ev => { const rr = chart.getBoundingClientRect(); const fx = (ev.clientX-rr.left)/rr.width*W; seek(Math.round((fx-LX)/(W-LX-8)*(n-1))); });
  }
  const side = $('div',{},
    $('h3',{text:'Entity timelines'}), legend, tl,
    $('div',{class:'explain', text:'One lane per entity (focus rows first). Colors show the Painter state per display frame; hatching marks frames drawn with less than 60% of that entity\'s median visible mass. Click a lane to jump the viewer there; hover for clip, frame and mass.'}),
    chartUids.length ? $('h3',{text:'Visible mass of focus entities (% of each entity\'s median)'}) : null,
    chartUids.length ? cLegend : null, chartUids.length ? chart : null,
    chartUids.length ? $('div',{class:'explain', text:'Visible mass = Σ alpha × opacity over the entity\'s drawn sprites. Consistent art keeps this curve near 100%; a cliff that is not a deploy/hit filter means the art itself was swapped (rule visible-area-jump). Click chips to hide curves.'}) : null);
  wrap.append($('div',{class:'viewer'}, $('div',{}, stage, controls, toggles,
    $('div',{class:'explain', text:'Boxes: entity bounds colored by state; red = a finding at this frame. Hover a box for its clip; click to highlight that entity. Keys: ←/→ step, space play.'})), side));
  viewers[s.name] = {seek};
  wrap.tabIndex = 0;
  wrap.addEventListener('keydown', ev => { if (ev.key==='ArrowRight') seek(cur+1); else if (ev.key==='ArrowLeft') seek(cur-1);
    else if (ev.key===' ') { ev.preventDefault(); play.onclick(); } });
  setTimeout(draw, 0);
  return wrap;
}

// ---------------------------------------------------------------- rules
app.append($('h2',{text:'Rules'}));
const rt = $('table',{}, $('thead',{}, $('tr',{}, $('th',{text:'Rule'}), $('th',{text:'Severity'}), $('th',{text:'What it means'}), $('th',{text:'Based on'}))));
const rb = $('tbody'); rt.append(rb);
for (const [k,v] of Object.entries(R.rules)) rb.append($('tr',{}, $('td',{}, $('code',{text:k})), $('td',{class:`n-${v.severity}`, text:v.severity}), $('td',{text:v.what}), $('td',{class:'kv', text:v.basis})));
app.append($('div',{class:'panel'}, rt));
</script>
</body>
</html>
"""
