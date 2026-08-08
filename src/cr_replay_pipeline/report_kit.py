"""Shared building blocks for the interactive HTML reports.

Every report is a single self-contained file, so the CSS and JS here get
inlined into each one. The kit only carries the pieces that are genuinely
generic — hero counters, comparison bars, chip toggles, tooltips and the arena
drawing primitives. Anything model-specific belongs in that model's report.
"""

from __future__ import annotations

import html
import json
from pathlib import Path
from typing import Any


def load_json(path: str | Path) -> dict[str, Any]:
    p = Path(path)
    if not p.exists():
        return {}
    with p.open(encoding="utf-8") as handle:
        return json.load(handle)


def esc(value: Any) -> str:
    return html.escape(str(value))


def badge_row(*labels: str) -> str:
    chips = "".join(f'<span class="badge">{esc(label)}</span>' for label in labels)
    return f'<div class="badge-row">{chips}</div>'


def hero_scores(items: list[tuple[str, Any, str]]) -> str:
    """items: (label, value, tone) where tone is up / down / neutral."""
    cells = []
    for label, value, tone in items:
        numeric = isinstance(value, (int, float))
        attr = f' data-count="{value}"' if numeric else ""
        shown = 0 if numeric else esc(value)
        cells.append(
            f'<div class="hero-score"><span class="hero-score-label">{esc(label)}</span>'
            f'<span class="hero-score-value {tone}"{attr}>{shown}</span></div>'
        )
    return f'<div class="hero-scores">{"".join(cells)}</div>'


def lesson_list(lessons: list[Any]) -> str:
    return "".join(f"<li>{esc(item)}</li>" for item in lessons)


def verdict_banner(status: str, text: str) -> str:
    tone = {
        "PASS": "pass",
        "FAIL": "fail",
        "PARTIAL": "partial",
    }.get(str(status).upper(), "partial")
    return (
        f'<div class="verdict verdict-{tone}">'
        f'<span class="verdict-tag">{esc(status)}</span>'
        f"<span>{esc(text)}</span></div>"
    )


def kit_styles() -> str:
    return """
    .hero h1 { font-size: 2.5rem; letter-spacing: -0.02em; }
    .hero-sub { font-size: 1.05rem; max-width: 72ch; color: #cbd5f5; }
    .hero-scores { display: flex; flex-wrap: wrap; gap: 16px; margin-top: 18px; }
    .hero-score {
      display: flex; flex-direction: column; gap: 4px; padding: 12px 18px;
      border-radius: 14px; background: rgba(148,163,184,0.08);
      border: 1px solid rgba(148,163,184,0.18);
    }
    .hero-score-label { font-size: 0.7rem; text-transform: uppercase; letter-spacing: 0.08em; color: #94a3b8; }
    .hero-score-value { font-size: 1.85rem; font-weight: 700; font-variant-numeric: tabular-nums; }
    .hero-score-value.up { color: #34d399; }
    .hero-score-value.down { color: #f87171; }
    .hero-score-value.neutral { color: #e2e8f0; }

    .verdict {
      display: flex; align-items: center; gap: 14px; padding: 14px 18px; border-radius: 14px;
      margin: 18px 0 4px; font-size: 0.95rem; border: 1px solid;
    }
    .verdict-tag { font-weight: 800; letter-spacing: 0.08em; font-size: 0.78rem; padding: 4px 10px; border-radius: 999px; }
    .verdict-pass { background: rgba(52,211,153,0.1); border-color: rgba(52,211,153,0.35); }
    .verdict-pass .verdict-tag { background: #34d399; color: #052e21; }
    .verdict-fail { background: rgba(248,113,113,0.1); border-color: rgba(248,113,113,0.35); }
    .verdict-fail .verdict-tag { background: #f87171; color: #2a0b0b; }
    .verdict-partial { background: rgba(251,191,36,0.1); border-color: rgba(251,191,36,0.35); }
    .verdict-partial .verdict-tag { background: #fbbf24; color: #2a1e05; }

    .play-btn {
      background: linear-gradient(120deg, #6366f1, #22d3ee);
      color: #06111f; border: 0; border-radius: 999px; padding: 9px 20px;
      font-weight: 700; cursor: pointer; font-size: 0.9rem;
    }
    .play-btn:hover { filter: brightness(1.12); }
    .play-btn.ghost { background: rgba(148,163,184,0.14); color: #e2e8f0; }
    .toolbar { display: flex; align-items: center; gap: 14px; margin-bottom: 16px; flex-wrap: wrap; }
    .toolbar .hint { color: #94a3b8; font-size: 0.85rem; }

    .chip-row { display: flex; flex-wrap: wrap; gap: 8px; }
    .chip {
      background: rgba(148,163,184,0.1); border: 1px solid rgba(148,163,184,0.2);
      color: #e2e8f0; border-radius: 999px; padding: 6px 13px; font-size: 0.82rem; cursor: pointer;
    }
    .chip:hover { border-color: #22d3ee88; }
    .chip.active { background: linear-gradient(120deg, #6366f1, #22d3ee); color: #06111f; font-weight: 700; border-color: transparent; }
    .chip .chip-n { opacity: 0.65; font-size: 0.72rem; margin-left: 5px; }
    .control-label {
      display: block; font-size: 0.7rem; text-transform: uppercase;
      letter-spacing: 0.08em; color: #94a3b8; margin-bottom: 8px;
    }
    .control-group { margin-bottom: 16px; }

    .race { display: grid; gap: 12px; }
    .race-row { display: grid; grid-template-columns: 230px 1fr 132px; gap: 14px; align-items: center; }
    .race-name { font-size: 0.9rem; }
    .race-name small { display: block; color: #94a3b8; font-size: 0.74rem; line-height: 1.35; }
    .race-track { position: relative; height: 26px; border-radius: 8px; background: rgba(148,163,184,0.1); overflow: hidden; }
    .race-fill {
      position: absolute; inset: 0 auto 0 0; width: 0%;
      background: linear-gradient(90deg, #6366f1, #22d3ee);
      transition: width 1.1s cubic-bezier(.22,1,.36,1);
    }
    .race-fill-soft { background: linear-gradient(90deg, #64748b, #94a3b8); }
    .race-ghost { position: absolute; top: 0; bottom: 0; width: 2px; background: #f8fafc88; }
    .race-ghost span { position: absolute; top: -1px; left: 4px; font-size: 0.62rem; color: #cbd5f5; }
    .race-val { text-align: right; font-variant-numeric: tabular-nums; font-size: 0.9rem; }
    .race-val b { display: block; }
    .race-delta { font-size: 0.76rem; }
    .up { color: #34d399; } .down { color: #f87171; } .flat { color: #94a3b8; }

    .tag-pill {
      display: inline-block; font-size: 0.6rem; font-weight: 700; letter-spacing: 0.05em;
      text-transform: uppercase; padding: 2px 7px; border-radius: 999px; margin-left: 6px; vertical-align: 2px;
    }
    .tag-pill.good { background: rgba(52,211,153,0.18); color: #6ee7b7; }
    .tag-pill.mute { background: rgba(148,163,184,0.16); color: #cbd5f5; }
    .tag-pill.warn { background: rgba(251,191,36,0.18); color: #fcd34d; }

    .legend-row { display: flex; gap: 18px; flex-wrap: wrap; font-size: 0.78rem; color: #cbd5f5; margin-bottom: 12px; }
    .legend-row .swatch { display: inline-block; width: 12px; height: 12px; border-radius: 50%; margin-right: 6px; vertical-align: -2px; }
    .legend-row .swatch.tick { width: 3px; height: 14px; border-radius: 1px; }
    .legend-row .swatch.bar { width: 16px; height: 6px; border-radius: 2px; vertical-align: 0; }

    .callout {
      margin-top: 16px; padding: 12px 16px; border-radius: 12px;
      background: rgba(34,211,238,0.1); border: 1px solid rgba(34,211,238,0.3); font-size: 0.9rem;
    }
    .callout a { color: #67e8f9; font-weight: 700; }

    .tip {
      position: absolute; pointer-events: none; padding: 7px 10px; border-radius: 9px;
      background: #0b1220ee; border: 1px solid rgba(148,163,184,0.3); font-size: 0.78rem;
      transform: translate(-50%, -125%); white-space: nowrap; z-index: 5;
    }
    .rel { position: relative; }

    .stat-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 12px; }
    .stat {
      padding: 12px 14px; border-radius: 12px; background: rgba(148,163,184,0.07);
      border: 1px solid rgba(148,163,184,0.16);
    }
    .stat .k { font-size: 0.68rem; text-transform: uppercase; letter-spacing: 0.07em; color: #94a3b8; }
    .stat .v { font-size: 1.35rem; font-weight: 700; font-variant-numeric: tabular-nums; }
    .stat .s { font-size: 0.72rem; color: #94a3b8; }

    .arena { width: 100%; height: auto; display: block; }
    .zone-cell { cursor: crosshair; }
    .mini-arena { width: 100%; height: auto; border-radius: 8px; }

    @media (max-width: 900px) {
      .race-row { grid-template-columns: 1fr; }
    }
    """


def kit_script() -> str:
    """Generic helpers. Every report script can assume these exist."""
    return r"""
const SVGNS = "http://www.w3.org/2000/svg";
const el = (tag, attrs = {}) => {
  const node = document.createElementNS(SVGNS, tag);
  for (const [k, v] of Object.entries(attrs)) node.setAttribute(k, v);
  return node;
};
const svgText = (attrs, text) => { const t = el("text", attrs); t.textContent = text; return t; };
const pct = (v, d = 1) => (v === null || v === undefined ? "—" : (100 * v).toFixed(d) + "%");
const titleCase = (s) => String(s).replace(/[-_]/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));

/* count-up for anything carrying data-count */
function mountCounters(root = document) {
  root.querySelectorAll("[data-count]").forEach((node) => {
    const target = Number(node.dataset.count);
    const dec = node.dataset.decimals ? Number(node.dataset.decimals) : 0;
    const suffix = node.dataset.suffix || "";
    const start = performance.now();
    const step = (now) => {
      const t = Math.min(1, (now - start) / 1200);
      const eased = 1 - Math.pow(1 - t, 3);
      const v = target * eased;
      node.textContent = (dec ? v.toFixed(dec) : Math.round(v).toLocaleString()) + suffix;
      if (t < 1) requestAnimationFrame(step);
    };
    requestAnimationFrame(step);
  });
}

/* comparison bars that travel from a baseline value to the new value */
function mountRace(hostId, rows, opts = {}) {
  const host = document.getElementById(hostId);
  if (!host) return null;
  const oldName = opts.oldName || "before";
  const fmt = (row, v) => {
    if (row.fmt === "pct") return pct(v);
    if (row.fmt === "int") return Math.round(v).toLocaleString();
    if (row.fmt === "sec") return v.toFixed(2) + "s";
    if (row.fmt === "x") return v.toFixed(2) + "×";
    return v.toFixed(3);
  };
  host.innerHTML = "";
  rows.forEach((row) => {
    const span = Math.max(Math.abs(row.old), Math.abs(row.new)) * 1.18 || 1;
    const frac = (v) => row.higher === false
      ? clamp((1 - v / span) * 100, 2, 100)
      : clamp((v / span) * 100, 2, 100);
    const better = row.higher === false ? row.new < row.old : row.new > row.old;
    const delta = row.new - row.old;
    const deltaTxt = row.fmt === "pct"
      ? (delta * 100).toFixed(1) + "pp"
      : (row.fmt === "int" ? Math.round(delta).toLocaleString() : delta.toFixed(2));
    const wrap = document.createElement("div");
    wrap.className = "race-row";
    const tag = row.tag
      ? `<span class="tag-pill ${row.tagTone || "mute"}">${row.tag}</span>` : "";
    wrap.innerHTML =
      `<div class="race-name">${row.label}${tag}<small>${row.note || ""}</small></div>
       <div class="race-track">
         <div class="race-fill${row.soft ? " race-fill-soft" : ""}" data-old="${frac(row.old)}" data-new="${frac(row.new)}"></div>
         <div class="race-ghost" style="left:${frac(row.old)}%"><span>${oldName}</span></div>
       </div>
       <div class="race-val"><b>${fmt(row, row.new)}</b>
         <span class="race-delta ${better ? "up" : (delta === 0 ? "flat" : "down")}">
           ${delta > 0 ? "+" : ""}${deltaTxt}</span></div>`;
    host.appendChild(wrap);
  });
  const fills = [...host.querySelectorAll(".race-fill")];
  const park = () => fills.forEach((f) => (f.style.width = f.dataset.old + "%"));
  const run = () => fills.forEach((f, i) => setTimeout(() => (f.style.width = f.dataset.new + "%"), i * 100));
  park();
  let on = false;
  const api = {
    toggle() { on = !on; on ? run() : park(); return on; },
    play() { if (!on) { on = true; run(); } },
  };
  if (opts.buttonId) {
    const btn = document.getElementById(opts.buttonId);
    const hint = opts.hintId ? document.getElementById(opts.hintId) : null;
    btn.addEventListener("click", () => {
      const now = api.toggle();
      btn.textContent = now ? (opts.backLabel || "↺ Reset") : (opts.playLabel || "▶ Run the diff");
      if (hint) hint.textContent = now ? (opts.hintAfter || "") : (opts.hintBefore || "");
    });
    new IntersectionObserver((entries, obs) => {
      entries.forEach((e) => {
        if (e.isIntersecting && !on) { btn.click(); obs.disconnect(); }
      });
    }, { threshold: 0.35 }).observe(host);
  }
  return api;
}

/* single tooltip element shared by every chart in a report */
function makeTip(hostId) {
  const host = document.getElementById(hostId);
  const tip = document.createElement("div");
  tip.className = "tip";
  tip.hidden = true;
  host.appendChild(tip);
  return {
    show(ev, html) {
      const box = host.getBoundingClientRect();
      tip.hidden = false;
      tip.innerHTML = html;
      tip.style.left = ev.clientX - box.left + "px";
      tip.style.top = ev.clientY - box.top + "px";
    },
    hide() { tip.hidden = true; },
  };
}

/* compact multi-series line chart with an optional epoch cursor */
function lineChart(svg, series, opts = {}) {
  const W = opts.width || 720, H = opts.height || 260;
  const ML = opts.marginLeft || 48, MB = 30, MT = 12, MR = 14;
  svg.setAttribute("viewBox", `0 0 ${W} ${H}`);
  svg.innerHTML = "";
  const all = series.flatMap((s) => s.values.filter((v) => v != null && !isNaN(v)));
  if (!all.length) return;
  let lo = opts.min != null ? opts.min : Math.min(...all);
  let hi = opts.max != null ? opts.max : Math.max(...all);
  if (hi === lo) { hi = lo + 1; }
  const pad = (hi - lo) * 0.08;
  lo -= pad; hi += pad;
  const n = Math.max(...series.map((s) => s.values.length), 2);
  const px = (i) => ML + (i / (n - 1)) * (W - ML - MR);
  const py = (v) => H - MB - ((v - lo) / (hi - lo)) * (H - MB - MT);
  for (let t = 0; t <= 4; t++) {
    const v = lo + (t / 4) * (hi - lo);
    svg.appendChild(el("line", { x1: ML, x2: W - MR, y1: py(v), y2: py(v), stroke: "rgba(148,163,184,0.14)" }));
    svg.appendChild(svgText(
      { x: ML - 8, y: py(v) + 4, "text-anchor": "end", fill: "#94a3b8", "font-size": 11 },
      opts.yFormat === "percent" ? (100 * v).toFixed(0) + "%" : v.toFixed(opts.yDigits ?? 2)));
  }
  series.forEach((s) => {
    const pts = s.values.map((v, i) => [px(i), py(v)]).filter((p) => !isNaN(p[1]));
    if (pts.length < 2) return;
    svg.appendChild(el("polyline", {
      points: pts.map((p) => p.join(",")).join(" "), fill: "none", stroke: s.color,
      "stroke-width": s.width || 2.2, "stroke-dasharray": s.dash || "none", "stroke-linejoin": "round",
    }));
    const last = pts[pts.length - 1];
    svg.appendChild(el("circle", { cx: last[0], cy: last[1], r: 3.4, fill: s.color }));
  });
  if (opts.xLabel) {
    svg.appendChild(svgText(
      { x: (W + ML) / 2, y: H - 6, "text-anchor": "middle", fill: "#94a3b8", "font-size": 11 }, opts.xLabel));
  }
  return { px, py, W, H, ML, MR, MB, MT };
}

function legendHTML(series) {
  return series.map((s) =>
    `<span style="display:inline-flex;align-items:center;gap:6px;margin-right:14px;font-size:.8rem">
      <span style="width:14px;height:3px;background:${s.color};display:inline-block"></span>${s.label}</span>`).join("");
}

/* histogram from {edges, counts} or {centers, counts} */
function histChart(svg, groups, opts = {}) {
  const W = opts.width || 720, H = opts.height || 240, ML = 44, MB = 32, MT = 12, MR = 12;
  svg.setAttribute("viewBox", `0 0 ${W} ${H}`);
  svg.innerHTML = "";
  const nb = Math.max(...groups.map((g) => g.counts.length));
  const peak = Math.max(...groups.flatMap((g) => g.counts), 1);
  const bw = (W - ML - MR) / nb;
  groups.forEach((g, gi) => {
    g.counts.forEach((c, i) => {
      const h = (c / peak) * (H - MB - MT);
      const x = ML + i * bw + (gi * bw) / (groups.length * 1.6);
      svg.appendChild(el("rect", {
        x: x + 0.5, y: H - MB - h, width: Math.max(1, bw / groups.length - 1), height: h,
        fill: g.color, opacity: g.opacity || 0.8, rx: 1.5,
      }));
    });
  });
  svg.appendChild(el("line", { x1: ML, x2: W - MR, y1: H - MB, y2: H - MB, stroke: "rgba(148,163,184,0.3)" }));
  const ticks = opts.ticks || [];
  ticks.forEach((t) => {
    const x = ML + t.at * (W - ML - MR);
    svg.appendChild(svgText({ x, y: H - MB + 16, "text-anchor": "middle", fill: "#94a3b8", "font-size": 11 }, t.label));
  });
  if (opts.yLabel) {
    svg.appendChild(svgText({ x: 6, y: MT + 10, fill: "#94a3b8", "font-size": 11 }, opts.yLabel));
  }
}
"""


def arena_script() -> str:
    """Clash-Royale board drawing shared by placement-flavoured reports.

    Coordinates are the acting side's view: ny = 0 is your own back line, so the
    board is drawn with your side at the bottom.
    """
    return r"""
const XB = [0, 0.4, 0.6, 1];
const YB = [0, 0.25, 0.45, 0.55, 1];
const ZONE_LABELS = [
  "own back left", "own back center", "own back right",
  "own front left", "own front center", "own front right",
  "river left", "river center", "river right",
  "enemy left", "enemy center", "enemy right",
];

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

function arenaBackdrop(g, w, h, ox, oy, scale) {
  const rect = (x, y, ww, hh, attrs) =>
    g.appendChild(el("rect", Object.assign({ x, y, width: ww, height: hh }, attrs)));
  rect(ox, oy, w, h, { fill: "#0d1a2b", rx: 10 * scale });
  rect(ox, oy, w, h * 0.5, { fill: "#991b1b", opacity: 0.28, rx: 10 * scale });
  rect(ox, oy + h * 0.5, w, h * 0.5, { fill: "#1d4ed8", opacity: 0.22 });
  rect(ox, oy + h * 0.465, w, h * 0.07, { fill: "#1d4ed8", opacity: 0.45 });
  rect(ox + w * 0.16, oy + h * 0.455, w * 0.12, h * 0.09, { fill: "#7c5c33", opacity: 0.75 });
  rect(ox + w * 0.72, oy + h * 0.455, w * 0.12, h * 0.09, { fill: "#7c5c33", opacity: 0.75 });
  towerOverlay(g, w, h, ox, oy, scale, 0.85);
}

function zoneCenter(z) {
  const col = z % 3, row = Math.floor(z / 3);
  return [(XB[col] + XB[col + 1]) / 2, (YB[row] + YB[row + 1]) / 2];
}

/* draws the 12-zone bubble map; values are shares (or deltas when diverging) */
function drawZoneBubbles(g, values, geom, opts = {}) {
  const { xPx, yPx } = geom;
  const diverging = !!opts.diverging;
  const peak = Math.max(...values.map((v) => Math.abs(v)), 1e-6);
  for (let row = 0; row < 4; row++) {
    for (let col = 0; col < 3; col++) {
      const z = row * 3 + col;
      const x0 = xPx(XB[col]), x1 = xPx(XB[col + 1]);
      const y0 = yPx(YB[row + 1]), y1 = yPx(YB[row]);
      const v = values[z];
      const mag = Math.abs(v) / peak;
      const fill = diverging ? (v >= 0 ? "#34d399" : "#f87171") : (opts.color || "#22d3ee");
      const cell = el("rect", {
        x: x0 + 1, y: y0 + 1, width: x1 - x0 - 2, height: y1 - y0 - 2, rx: 5,
        fill: diverging ? fill : "#e2e8f0",
        opacity: (diverging ? 0.04 + 0.16 * mag : 0.02 + 0.05 * mag).toFixed(3),
        class: "zone-cell", stroke: "rgba(226,232,240,0.22)", "stroke-width": 1,
      });
      g.appendChild(cell);
      const cx = (x0 + x1) / 2, cy = (y0 + y1) / 2;
      const rmax = Math.min(x1 - x0, y1 - y0) * 0.44;
      g.appendChild(el("circle", {
        cx, cy, r: Math.max(9, Math.sqrt(mag) * rmax), fill, opacity: 0.62,
        stroke: fill, "stroke-width": 1.5, "pointer-events": "none",
      }));
      g.appendChild(svgText(
        { x: cx, y: cy + 4, "text-anchor": "middle", fill: "#04131f", "font-size": 11,
          "font-weight": 800, "pointer-events": "none" },
        diverging ? (v >= 0 ? "+" : "") + (v * 100).toFixed(1) : (v * 100).toFixed(0) + "%"));
      if (opts.onHover) {
        cell.addEventListener("mousemove", (ev) => opts.onHover(ev, z));
        cell.addEventListener("mouseleave", () => opts.onLeave && opts.onLeave());
      }
    }
  }
}

/* full arena with side captions; returns pixel mappers */
function mountArena(svg, opts = {}) {
  const W = opts.W || 300, H = opts.H || 470, PAD = 6, TOP = 20;
  const AW = W - PAD * 2, AH = H - TOP - 22;
  svg.setAttribute("viewBox", `0 0 ${W} ${H}`);
  svg.innerHTML = "";
  const g = el("g");
  svg.appendChild(g);
  arenaBackdrop(g, AW, AH, PAD, TOP, 1);
  const geom = {
    xPx: (nx) => PAD + nx * AW,
    yPx: (ny) => TOP + (1 - ny) * AH,
    W, H, PAD, TOP, AW, AH,
  };
  geom.captions = () => {
    g.appendChild(svgText(
      { x: PAD, y: 13, fill: "#fca5a5", "font-size": 11, "letter-spacing": "0.14em", "font-weight": 700 },
      "ENEMY SIDE ↑"));
    g.appendChild(svgText(
      { x: PAD, y: H - 6, fill: "#93c5fd", "font-size": 11, "letter-spacing": "0.14em", "font-weight": 700 },
      "YOUR SIDE ↓"));
  };
  return { g, geom };
}
"""


def page(
    title: str,
    body: str,
    script: str,
    extra_styles: str = "",
    include_arena: bool = False,
) -> str:
    """Assemble a standalone report page."""
    from .winner_report import _base_styles

    arena = arena_script() if include_arena else ""
    return f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{esc(title)}</title>
  <style>{_base_styles()}{kit_styles()}{extra_styles}</style>
</head>
<body>
  <main>
{body}
  </main>
  <script>
    {kit_script()}
    {arena}
    {script}
  </script>
</body>
</html>
"""
