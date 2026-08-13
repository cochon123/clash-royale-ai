"""Generate self-contained HTML training reports for winner prediction models."""

from __future__ import annotations

import html
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from .report_kit import FONT_LINKS, favicon_link, shared_styles


def _fmt_pct(value: float | None, digits: int = 1) -> str:
    if value is None:
        return "—"
    return f"{100 * value:.{digits}f}%"


def _fmt_float(value: float | None, digits: int = 4) -> str:
    if value is None:
        return "—"
    return f"{value:.{digits}f}"


def _json_script(data: Any) -> str:
    return json.dumps(data, separators=(",", ":"))


def _asset_href(model_dir: Path, report_dir: Path, asset: str | None) -> str | None:
    if not asset:
        return None
    asset_path = Path(asset)
    if asset_path.is_absolute():
        try:
            asset_path = asset_path.relative_to(model_dir.parent.parent)
        except ValueError:
            asset_path = Path(*asset_path.parts[-2:])
    href = Path("../") / asset_path
    return href.as_posix()


def _report_timestamp(model_dir: Path, *candidates: str) -> str:
    times: list[float] = []
    for name in candidates:
        path = model_dir / name
        if path.exists():
            times.append(path.stat().st_mtime)
    if not times:
        return datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
    return datetime.fromtimestamp(max(times), tz=timezone.utc).strftime(
        "%Y-%m-%d %H:%M UTC"
    )


def _base_styles() -> str:
    return shared_styles()


def _chart_script() -> str:
    return r"""
    const SVG_NS = "http://www.w3.org/2000/svg";

    function svgEl(tag, attrs) {
      const el = document.createElementNS(SVG_NS, tag);
      Object.entries(attrs || {}).forEach(([key, value]) => {
        if (value !== undefined && value !== null) el.setAttribute(key, String(value));
      });
      return el;
    }

    function formatAxisValue(value, kind) {
      if (kind === "percent") return (value * 100).toFixed(1) + "%";
      if (kind === "float") return Number(value).toFixed(3);
      if (Math.abs(value) >= 1000) return Math.round(value).toLocaleString();
      if (Number.isInteger(value)) return String(value);
      return Number(value).toFixed(2);
    }

    function inferYFormat(values) {
      const min = Math.min(...values);
      const max = Math.max(...values);
      if (min >= 0 && max <= 1.05) return "percent";
      return "float";
    }

    function niceTicks(min, max, targetCount) {
      const span = Math.max(max - min, 1e-9);
      const rawStep = span / Math.max(1, targetCount - 1);
      const magnitude = Math.pow(10, Math.floor(Math.log10(rawStep)));
      const residual = rawStep / magnitude;
      let step = magnitude;
      if (residual > 5) step = 10 * magnitude;
      else if (residual > 2) step = 5 * magnitude;
      else if (residual > 1) step = 2 * magnitude;
      const tickMin = Math.floor(min / step) * step;
      const tickMax = Math.ceil(max / step) * step;
      const ticks = [];
      for (let value = tickMin; value <= tickMax + step * 0.001; value += step) {
        ticks.push(Number(value.toFixed(10)));
      }
      return { min: tickMin, max: tickMax, ticks, step };
    }

    function sampleTickIndexes(length, maxTicks) {
      if (length <= 1) return [0];
      const count = Math.min(maxTicks, length);
      const indexes = [];
      for (let i = 0; i < count; i += 1) {
        indexes.push(Math.round((i / (count - 1)) * (length - 1)));
      }
      return [...new Set(indexes)];
    }

    function ensureChartWrap(svg) {
      let wrap = svg.parentElement;
      if (!wrap || !wrap.classList.contains("chart-wrap")) {
        wrap = document.createElement("div");
        wrap.className = "chart-wrap";
        svg.parentNode.insertBefore(wrap, svg);
        wrap.appendChild(svg);
      }
      let tooltip = wrap.querySelector(".chart-tooltip");
      if (!tooltip) {
        tooltip = document.createElement("div");
        tooltip.className = "chart-tooltip";
        wrap.appendChild(tooltip);
      }
      return { wrap, tooltip };
    }

    function createLegendButton(meta) {
      const button = document.createElement("button");
      button.type = "button";
      button.className = "legend-item is-on";
      button.setAttribute("aria-pressed", "true");
      button.style.setProperty("--legend-color", meta.color);
      if (meta.dashed) button.classList.add("legend-dashed");

      const preview = document.createElement("span");
      preview.className = "legend-preview";
      preview.setAttribute("aria-hidden", "true");

      const label = document.createElement("span");
      label.className = "legend-label";
      label.textContent = meta.label;

      button.appendChild(preview);
      button.appendChild(label);
      button.addEventListener("click", () => {
        meta.onToggle();
        const visible = meta.getVisible();
        button.classList.toggle("is-on", visible);
        button.classList.toggle("is-off", !visible);
        button.setAttribute("aria-pressed", visible ? "true" : "false");
      });
      return button;
    }

    function bindInteractiveLegend(chartBlock, series, seriesLayers) {
      const legend = chartBlock && chartBlock.querySelector(".legend");
      if (!legend) return;
      legend.innerHTML = "";
      legend.classList.add("legend-interactive");
      series.forEach((entry, index) => {
        legend.appendChild(createLegendButton({
          color: entry.color,
          label: entry.label || ("Series " + (index + 1)),
          dashed: !!entry.dashed,
          onToggle: () => {
            const layer = seriesLayers[index];
            layer.visible = !layer.visible;
            layer.line.style.display = layer.visible ? "" : "none";
            if (layer.area) layer.area.style.display = layer.visible ? "" : "none";
          },
          getVisible: () => seriesLayers[index].visible,
        }));
      });
    }

    function mountInteractiveLineChart(svgId, config) {
      const svg = document.getElementById(svgId);
      if (!svg) return;

      const width = 1000;
      const height = 540;
      const pad = { top: 40, right: 32, bottom: 64, left: 88 };
      const series = config.series || [];
      const xLabels = config.xLabels || series[0].values.map((_, index) => String(index));
      const allY = series.flatMap((entry) => entry.values);
      const yFormat = config.yFormat || inferYFormat(allY);
      const rawMin = Math.min(...allY);
      const rawMax = Math.max(...allY);
      const yTicks = niceTicks(rawMin, rawMax, 6);
      const yPad = Math.max((yTicks.max - yTicks.min) * 0.08, yFormat === "percent" ? 0.005 : 0.01);
      const yMin = yTicks.min - yPad;
      const yMax = yTicks.max + yPad;
      const plotW = width - pad.left - pad.right;
      const plotH = height - pad.top - pad.bottom;
      const xMax = Math.max(1, xLabels.length - 1);

      svg.setAttribute("viewBox", "0 0 " + width + " " + height);
      svg.setAttribute("preserveAspectRatio", "xMidYMid meet");
      svg.innerHTML = "";
      svg.classList.add("chart-interactive");
      const { wrap, tooltip } = ensureChartWrap(svg);

      function px(index) {
        return pad.left + (plotW * index / xMax);
      }
      function py(value) {
        return pad.top + plotH - ((value - yMin) / (yMax - yMin)) * plotH;
      }

      const gridGroup = svgEl("g", { class: "chart-grid" });
      yTicks.ticks.forEach((tick) => {
        if (tick < yMin - 1e-9 || tick > yMax + 1e-9) return;
        const y = py(tick);
        gridGroup.appendChild(svgEl("line", {
          x1: pad.left,
          y1: y,
          x2: width - pad.right,
          y2: y,
          stroke: "#1e293b",
          "stroke-width": 1,
        }));
        const yLabel = svgEl("text", {
          x: pad.left - 14,
          y: y + 5,
          fill: "#94a3b8",
          "font-size": 16,
          "text-anchor": "end",
        });
        yLabel.textContent = formatAxisValue(tick, yFormat);
        gridGroup.appendChild(yLabel);
      });

      sampleTickIndexes(xLabels.length, 7).forEach((index) => {
        const x = px(index);
        gridGroup.appendChild(svgEl("line", {
          x1: x,
          y1: pad.top,
          x2: x,
          y2: height - pad.bottom,
          stroke: "#141c28",
          "stroke-width": 1,
          "stroke-dasharray": "4 7",
        }));
        const xLabel = svgEl("text", {
          x: x,
          y: height - pad.bottom + 30,
          fill: "#94a3b8",
          "font-size": 16,
          "text-anchor": "middle",
        });
        xLabel.textContent = xLabels[index];
        gridGroup.appendChild(xLabel);
      });
      svg.appendChild(gridGroup);

      const axesGroup = svgEl("g", { class: "chart-axes" });
      axesGroup.appendChild(svgEl("line", {
        x1: pad.left,
        y1: height - pad.bottom,
        x2: width - pad.right,
        y2: height - pad.bottom,
        stroke: "#334155",
        "stroke-width": 1.5,
      }));
      axesGroup.appendChild(svgEl("line", {
        x1: pad.left,
        y1: pad.top,
        x2: pad.left,
        y2: height - pad.bottom,
        stroke: "#334155",
        "stroke-width": 1.5,
      }));
      svg.appendChild(axesGroup);

      const linesGroup = svgEl("g", { class: "chart-lines" });
      const seriesLayers = series.map((entry) => {
        let area = null;
        if (config.area && series.length === 1) {
          const areaPoints = [
            pad.left + "," + (height - pad.bottom),
            ...entry.values.map((value, index) => px(index) + "," + py(value)),
            (pad.left + plotW) + "," + (height - pad.bottom),
          ].join(" ");
          area = svgEl("polygon", {
            points: areaPoints,
            fill: entry.color,
            opacity: 0.1,
          });
          linesGroup.appendChild(area);
        }
        const line = svgEl("polyline", {
          class: entry.dashed ? "curve-dashed" : "curve",
          points: entry.values.map((value, index) => px(index) + "," + py(value)).join(" "),
          fill: "none",
          stroke: entry.color,
          "stroke-width": 4,
          "stroke-linecap": "round",
          "stroke-linejoin": "round",
          "stroke-dasharray": entry.dashed ? "8 6" : "none",
          opacity: entry.dashed ? 0.85 : 1,
        });
        if (!entry.dashed) line.setAttribute("pathLength", "1");
        linesGroup.appendChild(line);
        return { area: area, line: line, visible: true };
      });
      svg.appendChild(linesGroup);

      const hoverGroup = svgEl("g", { class: "chart-hover", opacity: 0 });
      const crosshair = svgEl("line", {
        class: "chart-crosshair",
        x1: 0,
        y1: pad.top,
        x2: 0,
        y2: height - pad.bottom,
        stroke: "#94a3b8",
        "stroke-width": 1.2,
        "stroke-dasharray": "4 4",
        opacity: 0.75,
      });
      hoverGroup.appendChild(crosshair);
      const dots = series.map((entry) => svgEl("circle", {
        class: "chart-dot",
        r: 8,
        fill: entry.color,
        stroke: "#0a0f18",
        "stroke-width": 2.5,
        opacity: 0,
      }));
      dots.forEach((dot) => hoverGroup.appendChild(dot));
      svg.appendChild(hoverGroup);

      const overlay = svgEl("rect", {
        x: pad.left,
        y: pad.top,
        width: plotW,
        height: plotH,
        fill: "transparent",
        class: "chart-overlay",
      });
      svg.appendChild(overlay);

      function showHover(clientX, index) {
        const x = px(index);
        crosshair.setAttribute("x1", x);
        crosshair.setAttribute("x2", x);
        hoverGroup.setAttribute("opacity", 1);
        let tooltipHtml = '<div class="chart-tooltip-title">' + xLabels[index] + "</div>";
        let visibleCount = 0;
        series.forEach((entry, seriesIndex) => {
          const dot = dots[seriesIndex];
          const layer = seriesLayers[seriesIndex];
          if (!layer.visible) {
            dot.setAttribute("opacity", 0);
            return;
          }
          visibleCount += 1;
          dot.setAttribute("cx", x);
          dot.setAttribute("cy", py(entry.values[index]));
          dot.setAttribute("opacity", 1);
          tooltipHtml += '<div class="chart-tooltip-row"><span class="chart-tooltip-swatch" style="background:'
            + entry.color + '"></span><span>' + (entry.label || "Value")
            + '</span><strong>' + formatAxisValue(entry.values[index], yFormat) + "</strong></div>";
        });
        if (!visibleCount) {
          hideHover();
          return;
        }
        tooltip.innerHTML = tooltipHtml;
        tooltip.classList.add("visible");
        const wrapRect = wrap.getBoundingClientRect();
        const relX = clientX - wrapRect.left;
        tooltip.style.left = Math.min(wrapRect.width - tooltip.offsetWidth - 8, Math.max(8, relX + 14)) + "px";
      }

      function hideHover() {
        hoverGroup.setAttribute("opacity", 0);
        dots.forEach((dot) => dot.setAttribute("opacity", 0));
        tooltip.classList.remove("visible");
      }

      overlay.addEventListener("mousemove", (event) => {
        const rect = svg.getBoundingClientRect();
        const relX = ((event.clientX - rect.left) / rect.width) * width;
        const index = Math.round(((relX - pad.left) / plotW) * xMax);
        showHover(event.clientX, Math.max(0, Math.min(xMax, index)));
      });
      overlay.addEventListener("mouseleave", hideHover);
      bindInteractiveLegend(wrap.parentElement, series, seriesLayers);
    }

    function renderLineChart(svgId, series, xLabels, options) {
      mountInteractiveLineChart(svgId, Object.assign({
        series: series.map((entry, index) => Object.assign({}, entry, {
          label: entry.label || ("Series " + (index + 1)),
        })),
        xLabels: xLabels,
      }, options || {}));
    }

    function renderConfidenceChart(svgId, baseline, improved) {
      renderLineChart(svgId, [
        {
          label: "Baseline",
          values: baseline.map((row) => row.accuracy),
          color: "#7a8699",
          dashed: true,
        },
        {
          label: "Improved",
          values: improved.map((row) => row.accuracy),
          color: "#60a5fa",
        },
      ], improved.map((row) => formatAxisValue(row.min_confidence, "percent")), {
        yFormat: "percent",
      });
    }

    function nearestConfidenceIndex(curve, confidence) {
      let bestIndex = 0;
      let bestDistance = Infinity;
      curve.forEach((row, index) => {
        const distance = Math.abs(row.min_confidence - confidence);
        if (distance < bestDistance) {
          bestDistance = distance;
          bestIndex = index;
        }
      });
      return bestIndex;
    }

    function curvePoints(curve, field, px, py) {
      return curve.map((row) => px(row.min_confidence) + "," + py(row[field])).join(" ");
    }

    function setAnimToggleState(toggleBtn, playing) {
      if (!toggleBtn) return;
      toggleBtn.setAttribute("aria-label", playing ? "Pause" : "Play");
      const playIcon = toggleBtn.querySelector(".anim-icon-play");
      const pauseIcon = toggleBtn.querySelector(".anim-icon-pause");
      if (playIcon) playIcon.hidden = playing;
      if (pauseIcon) pauseIcon.hidden = !playing;
    }

    function mountTrainingAnimation(config) {
      const svg = document.getElementById(config.svgId);
      const container = document.getElementById(config.containerId);
      const baseline = config.baseline || [];
      const stages = config.stages || [];
      const scrubber = document.getElementById(config.scrubberId);
      const toggleBtn = document.getElementById(config.toggleId);
      const readout = document.getElementById(config.readoutId);
      if (!svg || !container || !stages.length || !baseline.length) return;

      const width = 1000;
      const height = 540;
      const pad = { top: 40, right: 72, bottom: 64, left: 88 };
      const plotW = width - pad.left - pad.right;
      const plotH = height - pad.top - pad.bottom;
      const allX = baseline.map((row) => row.min_confidence).concat(
        stages[stages.length - 1].confidence_curve.map((row) => row.min_confidence)
      );
      const xMin = Math.min(...allX);
      const xMax = Math.max(...allX);
      const xSpan = Math.max(xMax - xMin, 1e-9);
      const leftMin = 0.70;
      const leftMax = 1.005;
      const rightMin = 0.0;
      const rightMax = 1.05;

      svg.setAttribute("viewBox", "0 0 " + width + " " + height);
      svg.setAttribute("preserveAspectRatio", "xMidYMid meet");
      svg.innerHTML = "";
      svg.classList.add("chart-interactive", "chart-dual");

      function px(value) {
        return pad.left + ((value - xMin) / xSpan) * plotW;
      }
      function pyLeft(value) {
        return pad.top + plotH - ((value - leftMin) / (leftMax - leftMin)) * plotH;
      }
      function pyRight(value) {
        return pad.top + plotH - ((value - rightMin) / (rightMax - rightMin)) * plotH;
      }

      const gridGroup = svgEl("g", { class: "chart-grid" });
      for (let i = 0; i <= 5; i += 1) {
        const value = leftMin + ((leftMax - leftMin) * i / 5);
        const y = pyLeft(value);
        gridGroup.appendChild(svgEl("line", {
          x1: pad.left, y1: y, x2: width - pad.right, y2: y,
          stroke: "#1e293b", "stroke-width": 1,
        }));
        const leftLabel = svgEl("text", {
          x: pad.left - 14, y: y + 5, fill: "#60a5fa", "font-size": 16, "text-anchor": "end",
        });
        leftLabel.textContent = formatAxisValue(value, "percent");
        gridGroup.appendChild(leftLabel);
        const rightValue = rightMin + ((rightMax - rightMin) * i / 5);
        const rightLabel = svgEl("text", {
          x: width - pad.right + 14, y: y + 5, fill: "#bd7c00", "font-size": 16, "text-anchor": "start",
        });
        rightLabel.textContent = formatAxisValue(rightValue, "percent");
        gridGroup.appendChild(rightLabel);
      }
      sampleTickIndexes(baseline.length, 7).forEach((index) => {
        const x = px(baseline[index].min_confidence);
        gridGroup.appendChild(svgEl("line", {
          x1: x, y1: pad.top, x2: x, y2: height - pad.bottom,
          stroke: "#141c28", "stroke-width": 1, "stroke-dasharray": "4 7",
        }));
        const xLabel = svgEl("text", {
          x: x, y: height - pad.bottom + 30, fill: "#94a3b8", "font-size": 16, "text-anchor": "middle",
        });
        xLabel.textContent = formatAxisValue(baseline[index].min_confidence, "percent");
        gridGroup.appendChild(xLabel);
      });
      svg.appendChild(gridGroup);

      const axesGroup = svgEl("g", { class: "chart-axes" });
      axesGroup.appendChild(svgEl("line", {
        x1: pad.left, y1: height - pad.bottom, x2: width - pad.right, y2: height - pad.bottom,
        stroke: "#334155", "stroke-width": 1.5,
      }));
      axesGroup.appendChild(svgEl("line", {
        x1: pad.left, y1: pad.top, x2: pad.left, y2: height - pad.bottom,
        stroke: "#334155", "stroke-width": 1.5,
      }));
      axesGroup.appendChild(svgEl("line", {
        x1: width - pad.right, y1: pad.top, x2: width - pad.right, y2: height - pad.bottom,
        stroke: "#334155", "stroke-width": 1.5,
      }));
      [0.90, 0.95].forEach((refValue, index) => {
        axesGroup.appendChild(svgEl("line", {
          class: "chart-refline",
          x1: pad.left, y1: pyLeft(refValue), x2: width - pad.right, y2: pyLeft(refValue),
          stroke: index === 0 ? "#e63946" : "#7c4dff",
          "stroke-width": 1.4,
        }));
      });
      svg.appendChild(axesGroup);

      const seriesDefs = [
        { label: "Previous accuracy", color: "#7a8699", dashed: true, field: "accuracy", axis: "left", staticCurve: baseline },
        { label: "Training accuracy", color: "#60a5fa", dashed: false, field: "accuracy", axis: "left", animated: true },
        { label: "Previous coverage", color: "#9a6a12", dashed: true, field: "coverage", axis: "right", staticCurve: baseline },
        { label: "Training coverage", color: "#bd7c00", dashed: false, field: "coverage", axis: "right", animated: true },
      ];

      const { wrap, tooltip } = ensureChartWrap(svg);

      const linesGroup = svgEl("g", { class: "chart-lines" });
      const seriesLayers = seriesDefs.map((def) => {
        const py = def.axis === "left" ? pyLeft : pyRight;
        const source = def.staticCurve || stages[0].confidence_curve;
        const line = svgEl("polyline", {
          class: def.dashed ? "curve-dashed" : "curve",
          points: curvePoints(source, def.field, px, py),
          fill: "none",
          stroke: def.color,
          "stroke-width": def.animated ? 4 : 3.2,
          "stroke-linecap": "round",
          "stroke-linejoin": "round",
          "stroke-dasharray": def.dashed ? "8 6" : "none",
          opacity: def.dashed ? 0.85 : 1,
        });
        if (!def.dashed) line.setAttribute("pathLength", "1");
        linesGroup.appendChild(line);
        return { def: def, line: line, visible: true, py: py };
      });
      svg.appendChild(linesGroup);

      const hoverGroup = svgEl("g", { class: "chart-hover", opacity: 0 });
      const crosshair = svgEl("line", {
        class: "chart-crosshair",
        x1: 0,
        y1: pad.top,
        x2: 0,
        y2: height - pad.bottom,
        stroke: "#94a3b8",
        "stroke-width": 1.2,
        "stroke-dasharray": "4 4",
        opacity: 0.75,
      });
      hoverGroup.appendChild(crosshair);
      const hoverDots = seriesLayers.map((layer) => svgEl("circle", {
        class: "chart-dot",
        r: 8,
        fill: layer.def.color,
        stroke: "#0a0f18",
        "stroke-width": 2.5,
        opacity: 0,
      }));
      hoverDots.forEach((dot) => hoverGroup.appendChild(dot));
      svg.appendChild(hoverGroup);

      const overlay = svgEl("rect", {
        x: pad.left,
        y: pad.top,
        width: plotW,
        height: plotH,
        fill: "transparent",
        class: "chart-overlay",
      });
      svg.appendChild(overlay);

      let hoverState = null;

      function getLayerCurve(layer) {
        if (layer.def.staticCurve) return layer.def.staticCurve;
        return stages[frame].confidence_curve;
      }

      function showAnimHover(clientX, confidence) {
        hoverState = { clientX: clientX, confidence: confidence };
        const pointIndex = nearestConfidenceIndex(baseline, confidence);
        const row = baseline[pointIndex];
        const x = px(row.min_confidence);
        crosshair.setAttribute("x1", x);
        crosshair.setAttribute("x2", x);
        hoverGroup.setAttribute("opacity", 1);
        let tooltipHtml = '<div class="chart-tooltip-title">Min confidence '
          + formatAxisValue(row.min_confidence, "percent") + "</div>";
        let visibleCount = 0;
        seriesLayers.forEach((layer, index) => {
          const dot = hoverDots[index];
          if (!layer.visible) {
            dot.setAttribute("opacity", 0);
            return;
          }
          const curve = getLayerCurve(layer);
          const curveRow = curve[pointIndex];
          const value = curveRow[layer.def.field];
          visibleCount += 1;
          dot.setAttribute("cx", x);
          dot.setAttribute("cy", layer.py(value));
          dot.setAttribute("opacity", 1);
          tooltipHtml += '<div class="chart-tooltip-row"><span class="chart-tooltip-swatch" style="background:'
            + layer.def.color + '"></span><span>' + layer.def.label
            + '</span><strong>' + formatAxisValue(value, "percent") + "</strong></div>";
        });
        if (!visibleCount) {
          hideAnimHover();
          return;
        }
        tooltip.innerHTML = tooltipHtml;
        tooltip.classList.add("visible");
        const wrapRect = wrap.getBoundingClientRect();
        const relX = clientX - wrapRect.left;
        tooltip.style.left = Math.min(wrapRect.width - tooltip.offsetWidth - 8, Math.max(8, relX + 14)) + "px";
      }

      function hideAnimHover() {
        hoverState = null;
        hoverGroup.setAttribute("opacity", 0);
        hoverDots.forEach((dot) => dot.setAttribute("opacity", 0));
        tooltip.classList.remove("visible");
      }

      overlay.addEventListener("mousemove", (event) => {
        const rect = svg.getBoundingClientRect();
        const relX = ((event.clientX - rect.left) / rect.width) * width;
        const confidence = xMin + ((relX - pad.left) / plotW) * xSpan;
        showAnimHover(
          event.clientX,
          Math.max(xMin, Math.min(xMax, confidence))
        );
      });
      overlay.addEventListener("mouseleave", hideAnimHover);

      const legend = container.querySelector(".legend");
      if (legend) {
        legend.innerHTML = "";
        legend.classList.add("legend-interactive");
        seriesLayers.forEach((layer) => {
          legend.appendChild(createLegendButton({
            color: layer.def.color,
            label: layer.def.label,
            dashed: layer.def.dashed,
            onToggle: () => {
              layer.visible = !layer.visible;
              layer.line.style.display = layer.visible ? "" : "none";
              if (hoverState) showAnimHover(hoverState.clientX, hoverState.confidence);
            },
            getVisible: () => layer.visible,
          }));
        });
      }

      let frame = 0;
      let playing = false;
      let timer = null;
      const fps = 8;

      function updateReadout(stage) {
        if (!readout || !stage) return;
        readout.innerHTML = "<strong>" + stage.trees.toLocaleString() + " trees</strong>"
          + " · accuracy " + formatAxisValue(stage.accuracy, "percent")
          + " · AUC " + Number(stage.auc).toFixed(4)
          + " · AURC " + Number(stage.aurc).toFixed(4);
      }

      function setFrame(index) {
        frame = Math.max(0, Math.min(stages.length - 1, index));
        if (scrubber) {
          scrubber.max = String(stages.length - 1);
          scrubber.value = String(frame);
        }
        const stage = stages[frame];
        seriesLayers.forEach((layer) => {
          if (!layer.def.animated) return;
          layer.line.setAttribute(
            "points",
            curvePoints(stage.confidence_curve, layer.def.field, px, layer.py)
          );
        });
        updateReadout(stage);
        if (hoverState) showAnimHover(hoverState.clientX, hoverState.confidence);
      }

      function pause() {
        playing = false;
        setAnimToggleState(toggleBtn, false);
        if (timer) clearInterval(timer);
        timer = null;
      }

      function play() {
        if (playing) return;
        playing = true;
        setAnimToggleState(toggleBtn, true);
        timer = setInterval(() => {
          if (frame >= stages.length - 1) {
            pause();
            return;
          }
          setFrame(frame + 1);
        }, 1000 / fps);
      }

      if (toggleBtn) {
        toggleBtn.addEventListener("click", () => {
          if (playing) pause();
          else play();
        });
      }
      if (scrubber) {
        scrubber.addEventListener("input", () => {
          pause();
          setFrame(Number(scrubber.value));
        });
      }

      setFrame(0);
      setAnimToggleState(toggleBtn, false);
    }
    """


def render_hgb_report(
    model_dir: str | Path = "models/winner_predictor",
    output_path: str | Path | None = None,
) -> Path:
    model_dir = Path(model_dir)
    report_path = model_dir / "hgb_report.json"
    with report_path.open(encoding="utf-8") as handle:
        report = json.load(handle)

    confidence_path = model_dir / "accuracy_vs_confidence.json"
    confidence_data: dict[str, Any] = {}
    if confidence_path.exists():
        with confidence_path.open(encoding="utf-8") as handle:
            confidence_data = json.load(handle)

    report_dir = Path(output_path).parent if output_path else Path("reports")
    report_dir.mkdir(parents=True, exist_ok=True)
    out = Path(output_path) if output_path else report_dir / "winner_hgb_v1.html"

    created = _report_timestamp(model_dir, "hgb_report.json", "hgb_ensemble.pkl")

    animation_stages: list[Any] = []
    animation_path = model_dir / "confidence_training_stages.json"
    if animation_path.exists():
        with animation_path.open(encoding="utf-8") as handle:
            animation_stages = json.load(handle)

    training_stages = report.get("training_stages", [])
    duration_rows = report.get("test_by_duration", [])
    split_rows = report.get("splits", [])

    lessons = [
        "Replay action sequences carry enough signal for ~79% full-game winner accuracy without live arena state.",
        "Perspective symmetry (train on both sides, average at inference) removes arbitrary team/opponent orientation.",
        "Blending HistGradientBoosting with Extra Trees beats a single HGB copy; trees add diversity the booster lacks.",
        "Winner probability and confidence should use separate blend weights — accuracy and selective prediction optimize differently.",
        "Short games (&lt;2 min) are nearly solved; mid-length games (~2–3 min) remain the hardest bucket.",
    ]

    body = f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Winner predictor report — HGB ensemble v1</title>
  {favicon_link()}
  {FONT_LINKS}
  <style>{_base_styles()}</style>
</head>
<body>
  <main>
    <header class="report-header">
      <div class="badge-row">
        <span class="badge">winner-hgb-v1</span>
        <span class="badge">tabular ensemble</span>
        <span class="badge">scikit-learn · CPU</span>
      </div>
      <h1>Full-game winner predictor</h1>
      <p>Perspective-symmetric blend of HistGradientBoosting and Extra Trees over {report["battles_total"]:,} RoyaleAPI replays. Predicts which player wins from deck metadata, elixir/leak tables, and the ordered card-play sequence.</p>
      <p class="meta">Created {html.escape(created)} · trained in {report["seconds"]}s · {report["feature_dim"]} features</p>
    </header>

    <section class="report-section kpi-row">
      <div><span class="kpi-label">Test accuracy</span><span class="kpi-value">{_fmt_pct(report["test"]["acc"])}</span></div>
      <div><span class="kpi-label">Test AUC</span><span class="kpi-value">{_fmt_float(report["test"]["auc"])}</span></div>
      <div><span class="kpi-label">Battles</span><span class="kpi-value">{report["battles_total"]:,}</span></div>
      <div><span class="kpi-label">Baseline test acc</span><span class="kpi-value">{_fmt_pct(report["baseline"]["test"]["acc"])}</span></div>
    </section>

    <section class="report-section">
      <h2>Architecture</h2>
      <pre class="diagram">Replay JSON → feature extractor ({report["feature_dim"]} dims)
  ├─ deck composition &amp; card costs
  ├─ official elixir / leak tables
  └─ prefix action statistics
        ↓ symmetric inference (team view + swapped view, averaged)
HistGradientBoosting ({_fmt_pct(report["blend"]["hgb_weight"], 0)} blend) + Extra Trees ({_fmt_pct(report["blend"]["extra_trees_weight"], 0)} blend)
        ↓ separate confidence head (AURC-optimal blend + isotonic calibration)
Winner probability + P(prediction is correct)</pre>
    </section>

    <section class="report-section block-grid">
      <div class="block">
        <h2>Validation vs test</h2>
        <table>
          <thead><tr><th>Split</th><th>Accuracy</th><th>AUC</th><th>Log loss</th><th>N</th></tr></thead>
          <tbody>
            <tr><td>Validation</td><td>{_fmt_pct(report["val"]["acc"])}</td><td>{_fmt_float(report["val"]["auc"])}</td><td>{_fmt_float(report["val"]["log_loss"])}</td><td>{report["val"]["n"]:,}</td></tr>
            <tr><td>Test</td><td>{_fmt_pct(report["test"]["acc"])}</td><td>{_fmt_float(report["test"]["auc"])}</td><td>{_fmt_float(report["test"]["log_loss"])}</td><td>{report["test"]["n"]:,}</td></tr>
            <tr><td>Baseline test</td><td>{_fmt_pct(report["baseline"]["test"]["acc"])}</td><td>{_fmt_float(report["baseline"]["test"]["auc"])}</td><td>—</td><td>{report["test"]["n"]:,}</td></tr>
          </tbody>
        </table>
      </div>

      <div class="block">
        <h2>Data splits</h2>
        <table>
          <thead><tr><th>Split</th><th>Battles</th><th>Team win rate</th><th>Mean events</th></tr></thead>
          <tbody>
            {"".join(
                f'<tr><td>{html.escape(row["split"])}</td><td>{row["battles"]:,}</td>'
                f'<td>{_fmt_pct(row["team_win_rate"])}</td><td>{row["mean_events"]:.1f}</td></tr>'
                for row in split_rows
            )}
          </tbody>
        </table>
      </div>

      <div class="block">
        <h2>Accuracy by game duration (test)</h2>
        <table>
          <thead><tr><th>Duration (s)</th><th>N</th><th>Accuracy</th></tr></thead>
          <tbody>
            {"".join(
                f'<tr><td>{html.escape(row["duration"])}</td><td>{row["n"]}</td>'
                f'<td>{_fmt_pct(row["accuracy"])}</td></tr>'
                for row in duration_rows
            )}
          </tbody>
        </table>
      </div>
    </section>

    <section class="report-section">
      <div class="chart-grid">
        <div class="chart-block">
          <h2>Ensemble training curve</h2>
          <svg class="chart" id="stageAcc"></svg>
          <div class="legend"></div>
        </div>
        <div class="chart-block">
          <h2>Confidence calibration</h2>
          <svg class="chart" id="confidenceChart"></svg>
          <div class="legend"></div>
        </div>
      </div>
    </section>

    {(
        "<section class='report-section'>"
        "<h2>Training animation</h2>"
        "<p class='caption'>Confidence curves while Extra Trees are added. Hover the chart, toggle series in the legend, then play or scrub below.</p>"
        "<div class='chart-animation' id='confidenceTrainingAnim'>"
        "<svg class='chart' id='confidenceTrainingChart'></svg>"
        "<div class='legend'></div>"
        "<div class='anim-toolbar'>"
        "<button type='button' class='anim-btn anim-btn-icon' id='confidenceAnimToggle' aria-label='Play'>"
        "<svg class='anim-icon anim-icon-play' viewBox='0 0 16 16' aria-hidden='true'><path d='M4 2.5v11l9-5.5-9-5.5z'/></svg>"
        "<svg class='anim-icon anim-icon-pause' viewBox='0 0 16 16' aria-hidden='true' hidden><path d='M3.5 2h3v12h-3V2zm6 0h3v12h-3V2z'/></svg>"
        "</button>"
        "<input type='range' class='anim-scrubber' id='confidenceAnimScrubber' min='0' max='0' value='0' step='1'>"
        "<div class='anim-readout' id='confidenceAnimReadout'></div>"
        "</div>"
        "</div></section>"
    ) if animation_stages and confidence_data.get("baseline") else ""}

    <section class="report-section">
      <h2>Lessons learned</h2>
      <ul class="lessons">
        {"".join(f"<li>{lesson}</li>" for lesson in lessons)}
      </ul>
      <p>{html.escape(report.get("notes", ""))}</p>
    </section>
  </main>
  <script>
    {_chart_script()}
    const trainingStages = {_json_script(training_stages)};
    const confidenceData = {_json_script(confidence_data)};
    const trainingAnimationStages = {_json_script(animation_stages)};
    renderLineChart("stageAcc", [
      {{ color: "#34d399", label: "Test accuracy", values: trainingStages.map((row) => row.accuracy) }}
    ], trainingStages.map((row) => String(row.trees)), {{ area: true, yFormat: "percent" }});
    if (confidenceData.baseline && confidenceData.improved) {{
      renderConfidenceChart("confidenceChart", confidenceData.baseline, confidenceData.improved);
    }}
    if (trainingAnimationStages.length && confidenceData.baseline) {{
      mountTrainingAnimation({{
        containerId: "confidenceTrainingAnim",
        svgId: "confidenceTrainingChart",
        baseline: confidenceData.baseline,
        stages: trainingAnimationStages,
        scrubberId: "confidenceAnimScrubber",
        toggleId: "confidenceAnimToggle",
        readoutId: "confidenceAnimReadout",
      }});
    }}
  </script>
</body>
</html>
"""

    out.write_text(body, encoding="utf-8")
    return out


def render_transformer_report(
    model_dir: str | Path = "models/winner_predictor",
    output_path: str | Path | None = None,
) -> Path:
    model_dir = Path(model_dir)
    report_path = model_dir / "report.json"
    with report_path.open(encoding="utf-8") as handle:
        report = json.load(handle)

    report_dir = Path(output_path).parent if output_path else Path("reports")
    report_dir.mkdir(parents=True, exist_ok=True)
    out = Path(output_path) if output_path else report_dir / "winner_transformer_v1.html"

    curves_href = _asset_href(model_dir, report_dir, str(model_dir / "training_curves.png"))
    created = _report_timestamp(model_dir, "report.json", "best_model.pt")
    history = report.get("history", {})
    epochs = list(range(1, len(history.get("train_loss", [])) + 1))
    prefix_rows = report.get("test_by_prefix_ratio", {})
    split_rows = report.get("splits", [])

    gpu = report.get("gpu_name") or report.get("device") or "CPU"
    lessons = [
        "Sequence transformers can learn winner signal from replay prefixes, but underperform tabular features on full games.",
        "Mixed-prefix training helps generalization; full-game AUC peaks around epoch 13 then overfits.",
        "Early-game prefixes (50% of actions) are barely above chance — most signal arrives in the final quarter of the battle.",
        "This model remains useful as a prefix prior and for ablations, but production winner prediction should use the HGB ensemble.",
    ]

    prefix_table = "".join(
        f'<tr><td>{html.escape(ratio)}</td><td>{row["n"]:,}</td>'
        f'<td>{_fmt_pct(row["acc"])}</td><td>{_fmt_float(row["auc"])}</td></tr>'
        for ratio, row in sorted(prefix_rows.items(), key=lambda item: float(item[0]))
    )

    body = f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Winner predictor report — Transformer v1</title>
  {favicon_link()}
  {FONT_LINKS}
  <style>{_base_styles()}</style>
</head>
<body>
  <main>
    <header class="report-header">
      <div class="badge-row">
        <span class="badge">winner-transformer-v1</span>
        <span class="badge">sequence model</span>
        <span class="badge">PyTorch · {html.escape(str(gpu))}</span>
      </div>
      <h1>Prefix sequence winner model</h1>
      <p>GPU transformer trained on random action-prefix samples from {report["battles_total"]:,} battles. Evaluated on held-out prefixes and full-game sequences.</p>
      <p class="meta">Created {html.escape(created)} · trained in {report["seconds"]}s · majority baseline {_fmt_pct(report.get("majority_baseline"))}</p>
    </header>

    <section class="report-section kpi-row">
      <div><span class="kpi-label">Test accuracy</span><span class="kpi-value">{_fmt_pct(report["test"]["acc"])}</span></div>
      <div><span class="kpi-label">Test AUC</span><span class="kpi-value">{_fmt_float(report["test"]["auc"])}</span></div>
      <div><span class="kpi-label">Best val full AUC</span><span class="kpi-value">{_fmt_float(report.get("best_val_auc"))}</span></div>
      <div><span class="kpi-label">Prefix samples (train)</span><span class="kpi-value">{report["sequences"]["train"]:,}</span></div>
    </section>

    <section class="report-section">
      <div class="chart-grid">
        <div class="chart-block">
          <h2>Training loss</h2>
          <svg class="chart" id="lossChart"></svg>
          <div class="legend"></div>
        </div>
        <div class="chart-block">
          <h2>Training accuracy</h2>
          <svg class="chart" id="accChart"></svg>
          <div class="legend"></div>
        </div>
        <div class="chart-block">
          <h2>Validation AUC</h2>
          <svg class="chart" id="aucChart"></svg>
          <div class="legend"></div>
        </div>
      </div>
    </section>

    <section class="report-section block-grid">
      <div class="block">
        <h2>Test accuracy by prefix length</h2>
        <table>
          <thead><tr><th>Prefix ratio</th><th>N</th><th>Accuracy</th><th>AUC</th></tr></thead>
          <tbody>{prefix_table}</tbody>
        </table>
      </div>

      <div class="block">
        <h2>Data splits</h2>
        <table>
          <thead><tr><th>Split</th><th>Battles</th><th>Team win rate</th><th>Mean events</th></tr></thead>
          <tbody>
            {"".join(
                f'<tr><td>{html.escape(row["split"])}</td><td>{row["battles"]:,}</td>'
                f'<td>{_fmt_pct(row["team_win_rate"])}</td><td>{row["mean_events"]:.1f}</td></tr>'
                for row in split_rows
            )}
          </tbody>
        </table>
      </div>
    </section>

    {"<section class='report-section'><h2>Static training curves</h2><img class='media' src='" + html.escape(curves_href) + "' alt='Training curves'></section>" if curves_href else ""}

    <section class="report-section">
      <h2>Lessons learned</h2>
      <ul class="lessons">
        {"".join(f"<li>{lesson}</li>" for lesson in lessons)}
      </ul>
    </section>
  </main>
  <script>
    {_chart_script()}
    const epochs = {_json_script(epochs)};
    const history = {_json_script(history)};
    renderLineChart("lossChart", [
      {{ color: "#a78bfa", label: "Train", values: history.train_loss }},
      {{ color: "#60a5fa", label: "Val", values: history.val_loss }}
    ], epochs, {{ yFormat: "float" }});
    renderLineChart("accChart", [
      {{ color: "#22c55e", label: "Train", values: history.train_acc }},
      {{ color: "#34d399", label: "Val", values: history.val_acc }}
    ], epochs, {{ yFormat: "percent" }});
    renderLineChart("aucChart", [
      {{ color: "#f472b6", label: "Mixed prefix", values: history.val_auc }},
      {{ color: "#34d399", label: "Full game", values: history.val_full_auc }}
    ], epochs, {{ yFormat: "float" }});
  </script>
</body>
</html>
"""

    out.write_text(body, encoding="utf-8")
    return out


def render_winner_reports(
    model_dir: str | Path = "models/winner_predictor",
    output_dir: str | Path = "reports",
) -> list[Path]:
    model_dir = Path(model_dir)
    output_dir = Path(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    written: list[Path] = []

    # Interactive kit-based reports are the default surface.
    from .winner_interactive import (
        render_hgb_interactive,
        render_transformer_interactive,
    )

    hgb_report = model_dir / "hgb_report.json"
    if hgb_report.exists():
        written.append(
            render_hgb_interactive(model_dir, output_dir / "winner_hgb_v1.html")
        )

    transformer_report = model_dir / "report.json"
    if transformer_report.exists():
        written.append(
            render_transformer_interactive(
                model_dir, output_dir / "winner_transformer_v1.html"
            )
        )

    return written
