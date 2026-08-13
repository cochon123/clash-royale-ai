"""Shared building blocks for the interactive HTML reports.

Every report is a single self-contained file, so the CSS and JS here get
inlined into each one. The kit only carries the pieces that are genuinely
generic — hero counters, comparison bars, chip toggles, tooltips and the arena
drawing primitives. Anything model-specific belongs in that model's report.
"""

from __future__ import annotations

import base64
import html
import json
from functools import lru_cache
from pathlib import Path
from typing import Any

_ASSETS_DIR = Path(__file__).resolve().parent / "assets"
FAVICON_PATH = _ASSETS_DIR / "favicon.png"
FAVICON_INLINE_PATH = _ASSETS_DIR / "favicon-32.png"


@lru_cache(maxsize=1)
def _favicon_inline_data() -> str:
    return base64.b64encode(FAVICON_INLINE_PATH.read_bytes()).decode("ascii")


def favicon_link(*, href: str | None = None) -> str:
    """Return a favicon link tag for HTML pages and standalone reports."""
    if href is not None:
        return f'<link rel="icon" type="image/png" href="{esc(href)}">'
    data = _favicon_inline_data()
    return (
        f'<link rel="icon" type="image/png" '
        f'href="data:image/png;base64,{data}">'
    )


FONT_LINKS = """
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@500;700&family=IBM+Plex+Sans:wght@400;600;700&display=swap" rel="stylesheet">
""".strip()


def shared_styles() -> str:
    """Chrome shared by every training report. Diagrams keep their own CSS."""
    return """
    :root {
      color-scheme: dark;
      --bg: #071018;
      --panel: #0d171f;
      --line: #1e3140;
      --line-soft: #15222c;
      --text: #edf4f8;
      --muted: #8aa3b3;
      --accent: #38bdf8;
      --sky: #38bdf8;
      --star: #e8f58b;
      --green: #70e1b1;
      --gold: #ffca63;
      --blue: #70a1ff;
      --red: #ff7e78;
      --ink: #061018;
    }
    * { box-sizing: border-box; }
    html { scroll-behavior: smooth; }
    body {
      margin: 0;
      color: var(--text);
      font: 15px/1.55 "IBM Plex Sans", ui-sans-serif, system-ui, sans-serif;
      background:
        radial-gradient(circle at 12% -8%, #16384a 0, transparent 34%),
        radial-gradient(circle at 90% 0, #1d3f32 0, transparent 28%),
        var(--bg);
    }
    body:before { content: none !important; }
    main {
      width: min(1220px, calc(100% - 36px));
      margin: auto;
      padding: 48px 0 90px;
    }
    a { color: var(--sky); }
    h1 {
      font-size: clamp(2.2rem, 5.4vw, 3.7rem);
      line-height: 0.95;
      letter-spacing: -0.05em;
      margin: 16px 0 18px;
      font-weight: 700;
      max-width: 980px;
    }
    h1 em { color: var(--sky); font-style: normal; }
    h2 {
      font-size: clamp(1.35rem, 2.5vw, 1.85rem);
      letter-spacing: -0.03em;
      margin: 0 0 10px;
      color: var(--text);
      font-weight: 650;
    }
    h3 { font-size: 17px; margin: 0 0 8px; color: var(--text); }
    p, li { color: var(--muted); margin: 0 0 10px; }
    .eyebrow {
      font: 700 11px/1.2 "IBM Plex Mono", ui-monospace, monospace;
      letter-spacing: 0.16em;
      text-transform: uppercase;
      color: var(--sky);
    }
    .lede { font-size: clamp(16px, 2vw, 20px); max-width: 860px; color: #bdd0db; }
    .hero, .report-header { padding: 8px 0 40px; }
    .hero-sub { font-size: 1.05rem; max-width: 72ch; color: #bdd0db; }
    .stamp, .meta, .foot {
      margin-top: 18px;
      color: #607888;
      font: 12px "IBM Plex Mono", ui-monospace, monospace;
    }
    .foot { padding-top: 26px; display: block; }
    .mono { font-family: "IBM Plex Mono", ui-monospace, monospace; font-variant-numeric: tabular-nums; }

    .badge-row { display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 4px; }
    .badge {
      font: 700 11px/1.2 "IBM Plex Mono", ui-monospace, monospace;
      letter-spacing: 0.08em;
      text-transform: uppercase;
      color: var(--muted);
    }
    .badge + .badge::before { content: "·"; margin-right: 8px; color: #355065; }

    .section, .report-section { border-top: 1px solid var(--line); padding: 44px 0; }
    .section:first-of-type, .report-section:first-of-type { border-top: none; padding-top: 0; }
    .section-head {
      display: flex;
      justify-content: space-between;
      align-items: end;
      gap: 20px;
      margin-bottom: 22px;
    }
    .section-head p { max-width: 560px; }

    .kpis, .kpi-row, .match-kpis {
      display: grid;
      grid-template-columns: repeat(4, minmax(0, 1fr));
      gap: 12px;
    }
    .kpi, .card, .hero-score, .stat {
      border: 1px solid var(--line);
      background: linear-gradient(160deg, rgba(17,28,37,.94), rgba(8,14,20,.94));
      border-radius: 18px;
      padding: 20px;
      position: relative;
      overflow: hidden;
    }
    .kpi:after {
      content: "";
      position: absolute;
      width: 100px;
      height: 100px;
      border-radius: 50%;
      background: var(--tone, var(--sky));
      filter: blur(55px);
      opacity: 0.14;
      right: -30px;
      top: -35px;
    }
    .kpi span, .kpi-label, .hero-score-label, .stat .k {
      display: block;
      color: var(--muted);
      font-size: 12px;
      text-transform: uppercase;
      letter-spacing: 0.08em;
    }
    .kpi b, .kpi-value, .hero-score-value, .stat .v {
      display: block;
      font-size: clamp(1.4rem, 3vw, 2.1rem);
      letter-spacing: -0.04em;
      margin: 6px 0 2px;
      font-weight: 700;
      font-variant-numeric: tabular-nums;
      color: var(--text);
    }
    .kpi small, .stat .s { color: #6f8796; font-size: 0.78rem; }
    .hero-scores { display: flex; flex-wrap: wrap; gap: 12px; margin-top: 18px; }
    .hero-scores .hero-score { min-width: 140px; }
    .up, .hero-score-value.up { color: var(--green); }
    .down, .hero-score-value.down { color: var(--red); }
    .flat, .neutral, .hero-score-value.neutral { color: var(--muted); }

    .grid2, .compare-grid { display: grid; grid-template-columns: 1.1fr 0.9fr; gap: 14px; }
    .grid3 { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 13px; }
    .block-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 24px 32px; }
    .block-wide { grid-column: 1 / -1; }

    table { width: 100%; border-collapse: collapse; font-size: 14px; }
    th, td { padding: 11px 12px; border-bottom: 1px solid var(--line); text-align: left; }
    th {
      color: var(--muted);
      font: 700 11px "IBM Plex Mono", ui-monospace, monospace;
      text-transform: uppercase;
      letter-spacing: 0.08em;
    }
    .metric-table th, .metric-table td { text-align: right; }
    .metric-table th:first-child, .metric-table td:first-child { text-align: left; }
    .metric-table tbody tr:hover, tbody tr:hover { background: #12202a; }
    .winner { color: var(--sky); font-weight: 750; }
    .metric-note { font-size: 11px; color: #617888; display: block; }
    .table-wrap { overflow: auto; border: 1px solid var(--line); border-radius: 16px; }

    button, .toggle, .chip, .replay-chip, .match-chip, .anim-btn, .play-btn {
      border: 1px solid var(--line);
      background: #0a151c;
      color: var(--muted);
      border-radius: 10px;
      padding: 8px 12px;
      font: 600 12px "IBM Plex Mono", ui-monospace, monospace;
      cursor: pointer;
    }
    button:hover, .toggle:hover, .chip:hover, .replay-chip:hover,
    .match-chip:hover, .anim-btn:hover, .play-btn:hover {
      border-color: #416a80;
      color: var(--text);
    }
    button.active, .chip.active, .replay-chip.active, .match-chip.active, .play-btn {
      background: var(--sky);
      color: var(--ink);
      border-color: var(--sky);
    }
    .play-btn { border-radius: 999px; padding: 9px 20px; font-weight: 700; }
    .play-btn.ghost { background: #0a151c; color: var(--text); }
    .chip, .replay-chip, .match-chip, .pill {
      display: inline-block;
      border-radius: 999px;
      padding: 6px 13px;
    }
    .chip .chip-n { opacity: 0.65; font-size: 0.72rem; margin-left: 5px; }
    .chart-tools, .toolbar, .chip-row, .replay-chips, .match-chips {
      display: flex;
      flex-wrap: wrap;
      gap: 8px;
      margin: 14px 0;
      align-items: center;
    }
    .toolbar .hint { color: var(--muted); font-size: 0.85rem; }

    .bars, .bar-set { display: grid; gap: 14px; }
    .bar-row {
      display: grid;
      grid-template-columns: 90px 1fr 88px;
      gap: 12px;
      align-items: center;
    }
    .bar-track {
      height: 13px;
      border-radius: 99px;
      background: #08131a;
      overflow: hidden;
      border: 1px solid #1c2d38;
    }
    .bar-fill {
      height: 100%;
      width: 0;
      border-radius: 99px;
      background: var(--tone, var(--sky));
      transition: width 1s cubic-bezier(.2,.8,.2,1);
    }
    .bar-value { font: 700 12px "IBM Plex Mono", ui-monospace, monospace; text-align: right; }

    .expr {
      margin-top: 14px;
      padding: 14px 16px;
      border: 1px dashed #355065;
      border-radius: 12px;
      background: #0a151c;
      font: 13px/1.55 "IBM Plex Mono", ui-monospace, monospace;
      color: #c5d7e2;
    }
    .expr b { color: var(--sky); }
    .callout {
      border-left: 3px solid var(--gold);
      padding: 15px 18px;
      background: #19180f;
      border-radius: 0 12px 12px 0;
      margin-top: 18px;
      color: #d7cfaf;
    }
    .callout a { color: var(--gold); font-weight: 700; }

    ul.lessons { margin: 0; padding-left: 18px; display: block; }
    .lessons:not(ul) { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 13px; }
    .lesson .n { font: 800 12px "IBM Plex Mono", ui-monospace, monospace; color: var(--sky); }
    .lesson h3 { margin: 14px 0 7px; }
    .lesson p { font-size: 13px; }

    .diagram {
      margin: 0;
      padding: 0 0 0 16px;
      border-left: 2px solid var(--line);
      font: 13px/1.65 "IBM Plex Mono", ui-monospace, monospace;
      color: var(--muted);
      white-space: pre-wrap;
    }
    .media { width: 100%; display: block; border-radius: 8px; }
    .caption { font-size: 13px; color: #607888; margin-top: 10px; }

    .chart-grid { display: flex; flex-direction: column; gap: 48px; }
    .chart-block, .chart-wrap { position: relative; width: 100%; }
    svg.chart {
      width: 100%;
      height: auto;
      min-height: 420px;
      aspect-ratio: 1000 / 540;
      display: block;
      overflow: visible;
    }
    svg.chart-interactive .chart-lines polyline {
      transition: stroke-width 0.15s ease, opacity 0.15s ease;
    }
    svg.chart-interactive .chart-lines polyline:hover { stroke-width: 5.5; }
    .chart-overlay { cursor: crosshair; }
    .chart-crosshair { pointer-events: none; }
    .chart-dot {
      pointer-events: none;
      filter: drop-shadow(0 0 6px currentColor);
    }
    path.curve, polyline.curve {
      fill: none;
      stroke-linecap: round;
      stroke-linejoin: round;
      stroke-dasharray: 1;
      stroke-dashoffset: 1;
      animation: curveIn 1.15s cubic-bezier(.22, 1, .36, 1) forwards;
    }
    @keyframes curveIn {
      to { stroke-dashoffset: 0; }
    }
    .point {
      cursor: crosshair;
      transform-box: fill-box;
      transform-origin: center;
      transition: transform 0.16s cubic-bezier(.2, .8, .2, 1), filter 0.16s ease;
    }
    .point:hover, .point.is-hot {
      transform: scale(1.9);
      filter: drop-shadow(0 0 7px color-mix(in srgb, var(--sky) 80%, transparent));
    }
    .hist-bar {
      transform-box: fill-box;
      transform-origin: bottom center;
      transition: opacity 0.15s ease, filter 0.15s ease;
      cursor: crosshair;
    }
    .hist-bar:hover { filter: brightness(1.28); }
    .chart-tooltip {
      position: absolute;
      top: 12px;
      left: 0;
      min-width: 180px;
      padding: 14px 16px;
      border-radius: 12px;
      border: 1px solid var(--line);
      background: rgba(7, 16, 24, 0.94);
      pointer-events: none;
      opacity: 0;
      transform: translateY(4px);
      transition: opacity 0.12s ease, transform 0.12s ease;
      z-index: 2;
      font-size: 14px;
      color: #c5d7e2;
    }
    .chart-tooltip.visible { opacity: 1; transform: translateY(0); }
    .chart-tooltip-title {
      font-size: 12px;
      letter-spacing: 0.05em;
      text-transform: uppercase;
      color: #607888;
      margin-bottom: 10px;
    }
    .chart-tooltip-row { display: flex; align-items: center; gap: 8px; margin-bottom: 4px; }
    .chart-tooltip-row:last-child { margin-bottom: 0; }
    .chart-tooltip-row strong { margin-left: auto; color: var(--text); font-weight: 600; }
    .chart-tooltip-swatch { width: 10px; height: 10px; border-radius: 999px; flex-shrink: 0; }
    .legend, .legend-interactive, .legend-row {
      display: flex;
      gap: 12px;
      flex-wrap: wrap;
      margin-top: 14px;
      font-size: 14px;
      color: #607888;
    }
    .legend-item {
      display: inline-flex;
      align-items: center;
      gap: 7px;
      padding: 5px 10px;
      border: 1px solid var(--line);
      border-radius: 999px;
      background: transparent;
      color: var(--muted);
      font: inherit;
      font-size: 13px;
      cursor: pointer;
    }
    .legend-item:hover { border-color: #416a80; color: var(--text); }
    .legend-item.is-on { color: var(--text); }
    .legend-item.is-off { opacity: 0.4; }
    .legend-item.is-off .legend-label { text-decoration: line-through; }
    .legend-preview { width: 16px; border-top: 2px solid var(--legend-color, var(--muted)); flex-shrink: 0; }
    .legend-item.legend-dashed .legend-preview { border-top-style: dashed; }
    .legend-row .swatch {
      display: inline-block;
      width: 12px;
      height: 12px;
      border-radius: 50%;
      margin-right: 6px;
      vertical-align: -2px;
    }
    .dot {
      display: inline-block;
      width: 10px;
      height: 10px;
      border-radius: 999px;
      background: var(--tone, var(--sky));
      flex-shrink: 0;
    }

    #curve { width: 100%; height: 360px; display: block; }
    .gridline { stroke: var(--line); stroke-width: 1; }
    .axis-label { fill: #668090; font: 11px "IBM Plex Mono", ui-monospace, monospace; }
    .tooltip {
      position: fixed;
      pointer-events: none;
      z-index: 9;
      background: #edf4f8;
      color: #071018;
      border-radius: 10px;
      padding: 8px 10px;
      font: 12px "IBM Plex Mono", ui-monospace, monospace;
      box-shadow: 0 8px 28px #0008;
      opacity: 0;
      transform: translate(-50%, -120%);
    }
    .tip {
      position: absolute;
      pointer-events: none;
      padding: 7px 10px;
      border-radius: 9px;
      background: #0b1220ee;
      border: 1px solid var(--line);
      font: 12px "IBM Plex Mono", ui-monospace, monospace;
      z-index: 5;
    }

    .anim-toolbar { display: flex; flex-wrap: wrap; align-items: center; gap: 12px; margin-top: 14px; }
    .anim-btn-icon {
      width: 38px;
      height: 38px;
      padding: 0;
      display: inline-flex;
      align-items: center;
      justify-content: center;
    }
    .anim-icon { width: 15px; height: 15px; fill: currentColor; display: block; }
    .anim-icon[hidden] { display: none; }
    .anim-scrubber { flex: 1 1 220px; min-width: 180px; accent-color: var(--sky); cursor: pointer; }
    .anim-readout, .replay-log {
      flex: 1 1 100%;
      font: 12px "IBM Plex Mono", ui-monospace, monospace;
      color: var(--muted);
      font-variant-numeric: tabular-nums;
    }
    .anim-readout strong { color: var(--text); font-weight: 600; }
    svg.chart-dual .chart-refline { stroke-dasharray: 6 5; opacity: 0.75; }

    .arena { width: 100%; max-width: 320px; height: auto; display: block; margin: 0 auto; border-radius: 12px; }
    .mini-arena { width: 100%; height: auto; border-radius: 8px; }
    .zone-cell { cursor: crosshair; }
    .flow { display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 10px; }
    .flow-step {
      border: 1px solid var(--line);
      border-radius: 14px;
      padding: 14px 12px;
      background: #0a151c;
    }
    .decoder { display: grid; grid-template-columns: repeat(5, minmax(0, 1fr)); gap: 10px; }
    .step { padding: 16px; border: 1px solid var(--line); border-radius: 14px; background: #0a151c; }
    .step b { display: block; color: var(--sky); font: 700 11px "IBM Plex Mono", ui-monospace, monospace; letter-spacing: 0.08em; margin-bottom: 8px; }

    .verdict:has(.mark) {
      margin-top: 32px;
      border: 1px solid #2f6f8a;
      background: linear-gradient(135deg, rgba(56,189,248,.14), rgba(112,225,177,.05));
      border-radius: 22px;
      padding: 24px;
      display: grid;
      grid-template-columns: auto 1fr;
      gap: 18px;
      align-items: start;
    }
    .verdict .mark {
      width: 50px;
      height: 50px;
      border-radius: 14px;
      display: grid;
      place-items: center;
      background: var(--sky);
      color: var(--ink);
      font-size: 22px;
      font-weight: 900;
    }
    .verdict:has(.mark) strong { display: block; font-size: 20px; margin-bottom: 5px; color: var(--text); }
    .verdict:has(.verdict-tag) {
      display: flex;
      align-items: center;
      gap: 14px;
      padding: 14px 18px;
      border-radius: 14px;
      margin: 18px 0 4px;
      font-size: 0.95rem;
      border: 1px solid;
    }
    .verdict-tag { font-weight: 800; letter-spacing: 0.08em; font-size: 0.78rem; padding: 4px 10px; border-radius: 999px; }
    .verdict-pass { background: rgba(112,225,177,0.1); border-color: rgba(112,225,177,0.35); }
    .verdict-pass .verdict-tag { background: var(--green); color: var(--ink); }
    .verdict-fail { background: rgba(255,126,120,0.1); border-color: rgba(255,126,120,0.35); }
    .verdict-fail .verdict-tag { background: var(--red); color: var(--ink); }
    .verdict-partial { background: rgba(255,202,99,0.1); border-color: rgba(255,202,99,0.35); }
    .verdict-partial .verdict-tag { background: var(--gold); color: var(--ink); }

    @media (max-width: 900px) {
      .kpis, .kpi-row, .match-kpis, .grid2, .grid3, .block-grid,
      .lessons:not(ul), .decoder, .compare-grid { grid-template-columns: 1fr; }
      .section-head { flex-direction: column; align-items: start; }
      .verdict:has(.mark) { grid-template-columns: 1fr; }
      .bar-row { grid-template-columns: 72px 1fr 72px; }
    }
    """


def report_head(title: str, extra_styles: str = "", include_kit: bool = False) -> str:
    """Standard <head> for a standalone report."""
    styles = shared_styles()
    if include_kit:
        styles += kit_styles()
    styles += extra_styles
    return f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{esc(title)}</title>
  {favicon_link()}
  {FONT_LINKS}
  <style>{styles}</style>
</head>"""


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
    """Widgets used by interactive kit reports. Colors come from shared_styles()."""
    return """
    .control-label {
      display: block; font-size: 0.7rem; text-transform: uppercase;
      letter-spacing: 0.08em; color: var(--muted); margin-bottom: 8px;
    }
    .control-group { margin-bottom: 16px; }
    .rel { position: relative; }

    .race { display: grid; gap: 12px; }
    .race-row { display: grid; grid-template-columns: 230px 1fr 132px; gap: 14px; align-items: center; }
    .race-name { font-size: 0.9rem; }
    .race-name small { display: block; color: var(--muted); font-size: 0.74rem; line-height: 1.35; }
    .race-track { position: relative; height: 26px; border-radius: 8px; background: #08131a; overflow: hidden; }
    .race-fill {
      position: absolute; inset: 0 auto 0 0; width: 0%;
      background: var(--sky);
      transition: width 1.1s cubic-bezier(.22,1,.36,1);
    }
    .race-fill-soft { background: var(--muted); }
    .race-ghost { position: absolute; top: 0; bottom: 0; width: 2px; background: #edf4f888; }
    .race-ghost span { position: absolute; top: -1px; left: 4px; font-size: 0.62rem; color: #bdd0db; }
    .race-val { text-align: right; font-variant-numeric: tabular-nums; font-size: 0.9rem; }
    .race-val b { display: block; }
    .race-delta { font-size: 0.76rem; }

    .tag-pill {
      display: inline-block; font-size: 0.6rem; font-weight: 700; letter-spacing: 0.05em;
      text-transform: uppercase; padding: 2px 7px; border-radius: 999px; margin-left: 6px; vertical-align: 2px;
    }
    .tag-pill.good { background: rgba(112,225,177,0.18); color: var(--green); }
    .tag-pill.mute { background: rgba(138,163,179,0.16); color: #bdd0db; }
    .tag-pill.warn { background: rgba(255,202,99,0.18); color: var(--gold); }

    .stat-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 12px; }

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

/* compact multi-series line chart with hover readout and draw-in */
function lineChart(svg, series, opts = {}) {
  const W = opts.width || 720, H = opts.height || 260;
  const ML = opts.marginLeft || 48, MB = 30, MT = 12, MR = 14;
  svg.setAttribute("viewBox", `0 0 ${W} ${H}`);
  svg.innerHTML = "";
  svg.classList.add("chart-interactive");
  const wrap = svg.parentElement;
  if (wrap) wrap.classList.add("chart-wrap");
  let tip = wrap ? wrap.querySelector(":scope > .chart-tooltip") : null;
  if (wrap && !tip) {
    tip = document.createElement("div");
    tip.className = "chart-tooltip";
    wrap.appendChild(tip);
  }
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
  const fmt = (v) => opts.yFormat === "percent"
    ? (100 * v).toFixed(1) + "%"
    : Number(v).toFixed(opts.yDigits ?? 2);
  for (let t = 0; t <= 4; t++) {
    const v = lo + (t / 4) * (hi - lo);
    svg.appendChild(el("line", { x1: ML, x2: W - MR, y1: py(v), y2: py(v), stroke: "rgba(138,163,179,0.18)" }));
    svg.appendChild(svgText(
      { x: ML - 8, y: py(v) + 4, "text-anchor": "end", fill: "#8aa3b3", "font-size": 11 },
      opts.yFormat === "percent" ? (100 * v).toFixed(0) + "%" : v.toFixed(opts.yDigits ?? 2)));
  }
  series.forEach((s) => {
    const pts = s.values.map((v, i) => [px(i), py(v)]).filter((p) => !isNaN(p[1]));
    if (pts.length < 2) return;
    const attrs = {
      class: s.dash ? "curve-dashed" : "curve",
      points: pts.map((p) => p.join(",")).join(" "),
      fill: "none",
      stroke: s.color,
      "stroke-width": s.width || 2.4,
      "stroke-linecap": "round",
      "stroke-linejoin": "round",
    };
    if (s.dash) attrs["stroke-dasharray"] = s.dash;
    else attrs.pathLength = "1";
    svg.appendChild(el("polyline", attrs));
  });
  if (opts.xLabel) {
    svg.appendChild(svgText(
      { x: (W + ML) / 2, y: H - 6, "text-anchor": "middle", fill: "#8aa3b3", "font-size": 11 }, opts.xLabel));
  }
  const hoverG = el("g", { class: "chart-hover", opacity: "0" });
  const cross = el("line", {
    class: "chart-crosshair", y1: MT, y2: H - MB,
    stroke: "#38bdf8", "stroke-width": 1, "stroke-dasharray": "3 4", opacity: 0.85,
  });
  hoverG.appendChild(cross);
  const dots = series.map((s) => {
    const c = el("circle", {
      class: "chart-dot", r: 5.5, fill: s.color, stroke: "#061018", "stroke-width": 2, opacity: 0,
    });
    hoverG.appendChild(c);
    return c;
  });
  svg.appendChild(hoverG);
  const overlay = el("rect", {
    class: "chart-overlay", x: ML, y: MT, width: W - ML - MR, height: H - MT - MB, fill: "transparent",
  });
  svg.appendChild(overlay);
  overlay.addEventListener("mousemove", (ev) => {
    const box = svg.getBoundingClientRect();
    const x = ((ev.clientX - box.left) / box.width) * W;
    const idx = clamp(Math.round(((x - ML) / (W - ML - MR)) * (n - 1)), 0, n - 1);
    const cx = px(idx);
    cross.setAttribute("x1", cx);
    cross.setAttribute("x2", cx);
    hoverG.setAttribute("opacity", "1");
    let html = `<div class="chart-tooltip-title">${opts.xLabel || "step"} ${idx + 1}</div>`;
    series.forEach((s, si) => {
      const v = s.values[idx];
      const dot = dots[si];
      if (v == null || isNaN(v)) { dot.setAttribute("opacity", 0); return; }
      dot.setAttribute("opacity", 1);
      dot.setAttribute("cx", px(idx));
      dot.setAttribute("cy", py(v));
      html += `<div class="chart-tooltip-row"><span class="chart-tooltip-swatch" style="background:${s.color}"></span>`
        + `<span>${s.label || ""}</span><strong>${fmt(v)}</strong></div>`;
    });
    if (tip && wrap) {
      tip.innerHTML = html;
      tip.classList.add("visible");
      const wr = wrap.getBoundingClientRect();
      tip.style.left = Math.min(wr.width - 12, Math.max(8, ev.clientX - wr.left + 14)) + "px";
      tip.style.top = "12px";
    }
  });
  overlay.addEventListener("mouseleave", () => {
    hoverG.setAttribute("opacity", "0");
    dots.forEach((d) => d.setAttribute("opacity", 0));
    if (tip) tip.classList.remove("visible");
  });
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
        class: "hist-bar",
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
    arena = arena_script() if include_arena else ""
    return f"""{report_head(title, extra_styles=extra_styles, include_kit=True)}
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
