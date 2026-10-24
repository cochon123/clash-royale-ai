// Replay a fixture through clash_painter.wasm and compare it to the native
// recorder. Draw times are host performance.now() around painter_step.
//
// Usage:
//   node runner.mjs --wasm clash_painter.wasm --frames frames.jsonl \
//     --vfs vfs.json --native-dir recording/ --out report.json --cards cards.json

import { readFileSync, writeFileSync } from "node:fs";
import { performance } from "node:perf_hooks";

const args = parseArgs(process.argv.slice(2));
const wasmBytes = readFileSync(args.wasm);
const { instance } = await WebAssembly.instantiate(wasmBytes, {});
const api = instance.exports;
const required = [
  "memory",
  "painter_alloc",
  "painter_free",
  "painter_vfs_put",
  "painter_init",
  "painter_set_lag_ticks",
  "painter_observe",
  "painter_step",
  "painter_result_ptr",
  "painter_result_len",
  "painter_texture_ptr",
  "painter_texture_len",
  "painter_texture_release",
  "painter_reset",
  "painter_stats_json",
  "painter_last_error",
];
const missing = required.filter((name) => typeof api[name] !== "function" && name !== "memory");
if (missing.length || !api.memory) {
  throw new Error(`wasm is missing C2 exports: ${missing.concat(api.memory ? [] : ["memory"]).join(", ")}`);
}

const encoder = new TextEncoder();
const decoder = new TextDecoder();
let peakWasm = api.memory.buffer.byteLength;
let peakRss = process.memoryUsage().rss;

function noteMemory() {
  peakWasm = Math.max(peakWasm, api.memory.buffer.byteLength);
  peakRss = Math.max(peakRss, process.memoryUsage().rss);
}

function view() {
  return new Uint8Array(api.memory.buffer);
}

function allocCopy(bytes) {
  const ptr = api.painter_alloc(bytes.length);
  if (bytes.length === 0) return 0;
  if (!ptr) throw new Error(`painter_alloc(${bytes.length}) failed`);
  view().set(bytes, ptr);
  noteMemory();
  return ptr;
}

function callChecked(rc, what) {
  if (rc === 0) return;
  api.painter_last_error();
  const message = readResult();
  throw new Error(`${what} failed (${rc}): ${message}`);
}

function readResult() {
  const ptr = api.painter_result_ptr();
  const len = api.painter_result_len();
  if (!ptr || !len) return "";
  return decoder.decode(view().subarray(ptr, ptr + len));
}

const vfs = JSON.parse(readFileSync(args.vfs, "utf8"));
let vfsBytes = 0;
for (const file of vfs) {
  const data = readFileSync(file.source);
  vfsBytes += data.length;
  const pathBytes = encoder.encode(file.path);
  const pathPtr = allocCopy(pathBytes);
  const dataPtr = allocCopy(data);
  const rc = api.painter_vfs_put(pathPtr, pathBytes.length, dataPtr, data.length);
  api.painter_free(pathPtr, pathBytes.length);
  api.painter_free(dataPtr, data.length);
  callChecked(rc, `painter_vfs_put ${file.path}`);
}

const cards = readFileSync(args.cards);
const cardsPtr = allocCopy(cards);
api.painter_set_lag_ticks(args.lag);
const initRc = api.painter_init(cardsPtr, cards.length, args.width, args.height);
api.painter_free(cardsPtr, cards.length);
callChecked(initRc, "painter_init");
noteMemory();

const frames = readFileSync(args.frames, "utf8")
  .split("\n")
  .filter((line) => line.trim().length > 0);
const nativePackets = readFileSync(`${args.nativeDir}/packets.jsonl`, "utf8")
  .split("\n")
  .filter((line) => line.trim().length > 0)
  .map((line) => JSON.parse(line));

const tickDt = 1 / 20;
const dt = 1 / args.fps;
let wall = 0;
let next = 0;
let idle = 0;
const drawMs = [];
const stepMs = [];
const wasmPackets = [];
const pixelNotes = [];
const failures = [];
let pixelMismatches = 0;
let idMismatches = 0;
let identicalPixelsDifferentId = 0;
const idMap = new Map();
const cap = frames.length * 4 + 64;

for (let stepIndex = 0; stepIndex < cap; stepIndex += 1) {
  while (next < frames.length && next * tickDt <= wall + 1e-9) {
    const bytes = encoder.encode(frames[next]);
    const ptr = allocCopy(bytes);
    const rc = api.painter_observe(ptr, bytes.length);
    api.painter_free(ptr, bytes.length);
    callChecked(rc, `painter_observe ${next}`);
    next += 1;
  }
  const started = performance.now();
  const code = api.painter_step(dt);
  const elapsed = performance.now() - started;
  stepMs.push(elapsed);
  noteMemory();
  if (code < 0) {
    api.painter_last_error();
    throw new Error(`painter_step failed: ${readResult()}`);
  }
  if (code > 0) {
    drawMs.push(elapsed);
    const packet = JSON.parse(readResult());
    wasmPackets.push(packet);
    comparePacket(nativePackets[wasmPackets.length - 1], packet, wasmPackets.length - 1);
    releaseTextures(packet);
    idle = 0;
  } else if (next >= frames.length) {
    idle += 1;
    if (idle >= 8) break;
  }
  wall += dt;
}

api.painter_stats_json();
const stats = JSON.parse(readResult() || "{}");
noteMemory();
const report = {
  snapshots: frames.length,
  native_packets: nativePackets.length,
  wasm_packets: wasmPackets.length,
  vfs_files: vfs.length,
  vfs_bytes: vfsBytes,
  draw_samples: drawMs.length,
  draw_p50_ms: percentile(drawMs, 0.5),
  draw_p95_ms: percentile(drawMs, 0.95),
  step_p50_ms: percentile(stepMs, 0.5),
  step_p95_ms: percentile(stepMs, 0.95),
  peak_wasm_bytes: Math.max(peakWasm, Number(stats.wasm_memory_peak || 0)),
  peak_wasm_mb: Math.max(peakWasm, Number(stats.wasm_memory_peak || 0)) / (1024 * 1024),
  peak_node_rss_bytes: peakRss,
  peak_node_rss_mb: peakRss / (1024 * 1024),
  stats,
  texture_id_mismatches: idMismatches,
  identical_pixels_different_id: identicalPixelsDifferentId,
  pixel_mismatches: pixelMismatches,
  failures: failures.slice(0, 12),
  pixel_notes: pixelNotes.slice(0, 8),
  equal: wasmPackets.length === nativePackets.length && pixelMismatches === 0,
};
writeFileSync(args.out, JSON.stringify(report, null, 2));
console.log(JSON.stringify({
  equal: report.equal,
  packets: report.wasm_packets,
  draw_p50_ms: report.draw_p50_ms,
  draw_p95_ms: report.draw_p95_ms,
  peak_wasm_mb: Number(report.peak_wasm_mb.toFixed(1)),
  peak_node_rss_mb: Number(report.peak_node_rss_mb.toFixed(1)),
  texture_id_mismatches: idMismatches,
  identical_pixels_different_id: identicalPixelsDifferentId,
  pixel_mismatches: pixelMismatches,
  first_failure: failures[0] || null,
}));
if (!report.equal) process.exit(1);

function releaseTextures(packet) {
  for (const texture of packet.textures || []) {
    const id = encoder.encode(texture.id);
    const ptr = allocCopy(id);
    api.painter_texture_release(ptr, id.length);
    api.painter_free(ptr, id.length);
  }
}

function comparePacket(native, wasm, index) {
  if (!native) {
    pixelMismatches += 1;
    failures.push({ packet: index, reason: "wasm produced a packet the native recorder did not" });
    return;
  }
  const nativeDraws = native.draws || [];
  const wasmDraws = wasm.draws || [];
  if (nativeDraws.length !== wasmDraws.length) {
    pixelMismatches += 1;
    const n = Math.min(nativeDraws.length, wasmDraws.length);
    let at = 0;
    let detail = null;
    for (; at < n; at += 1) {
      detail = jsonClose(stripIds(nativeDraws[at]), stripIds(wasmDraws[at]), `$.draws[${at}]`);
      if (detail) break;
    }
    const longer = nativeDraws.length > wasmDraws.length ? "native" : "wasm";
    failures.push({
      packet: index,
      reason: detail || `draw count ${nativeDraws.length} != ${wasmDraws.length}`,
      display_tick: [native.display_tick, wasm.display_tick],
      first_diff_index: at,
      native_at: brief(nativeDraws[at]),
      wasm_at: brief(wasmDraws[at]),
      extra_side: at === n ? longer : null,
      extra: at === n ? stripIds((longer === "native" ? nativeDraws : wasmDraws)[n]) : null,
    });
    return;
  }
  pairTextures(native, wasm, index);
  for (let i = 0; i < nativeDraws.length; i += 1) {
    const reason = jsonClose(remapIds(nativeDraws[i]), wasmDraws[i], `$.draws[${i}]`);
    if (reason) {
      pixelMismatches += 1;
      failures.push({ packet: index, reason });
      return;
    }
  }
  const leftRest = remapIds(rest(native));
  const rightRest = rest(wasm);
  const reason = jsonClose(leftRest, rightRest, "$");
  if (reason) {
    pixelMismatches += 1;
    failures.push({ packet: index, reason });
  }
}

function stripIds(value) {
  if (Array.isArray(value)) return value.map(stripIds);
  if (isObject(value)) {
    const out = {};
    for (const key of Object.keys(value)) {
      if (key === "tex" || key === "id") continue;
      out[key] = stripIds(value[key]);
    }
    return out;
  }
  return value;
}

function brief(draw) {
  if (!draw) return null;
  if (draw.kind === "sprite") {
    return { kind: "sprite", alpha: draw.alpha, m: draw.m, anchor: draw.anchor || null };
  }
  if (draw.kind === "shadow") return { kind: "shadow", n: (draw.draws || []).length };
  if (draw.kind === "rect") {
    return { kind: "rect", x: draw.x, y: draw.y, w: draw.w, h: draw.h, color: draw.color };
  }
  return { kind: draw.kind, alpha: draw.alpha, cx: draw.cx, cy: draw.cy, rx: draw.rx, ry: draw.ry };
}

function remapIds(value) {
  if (Array.isArray(value)) return value.map(remapIds);
  if (isObject(value)) {
    const out = {};
    for (const key of Object.keys(value)) {
      const child = value[key];
      if ((key === "tex" || key === "id") && typeof child === "string" && idMap.has(child)) {
        out[key] = idMap.get(child);
      } else if (key === "evicted" && Array.isArray(child)) {
        out[key] = child.map((id) => (typeof id === "string" && idMap.has(id) ? idMap.get(id) : id));
      } else out[key] = remapIds(child);
    }
    return out;
  }
  return value;
}

function rest(packet) {
  const copy = { ...packet };
  delete copy.render_ms;
  delete copy.draws;
  delete copy.textures;
  return copy;
}

function pairTextures(native, wasm, index) {
  const left = native.textures || [];
  const right = wasm.textures || [];
  if (left.length !== right.length) {
    idMismatches += 1;
    pixelMismatches += 1;
    failures.push({
      packet: index,
      reason: `texture publication count ${left.length} != ${right.length}`,
    });
    return;
  }
  for (let i = 0; i < left.length; i += 1) {
    if (left[i].width !== right[i].width || left[i].height !== right[i].height) {
      idMismatches += 1;
      pixelMismatches += 1;
      failures.push({
        packet: index,
        reason: `texture ${i} size ${left[i].width}x${left[i].height} != ${right[i].width}x${right[i].height}`,
      });
      continue;
    }
    if (left[i].id === right[i].id) continue;
    idMismatches += 1;
    const nativePixels = readFileSync(`${args.nativeDir}/textures/${left[i].id}.rgba`);
    const wasmPixels = copyTexture(right[i].id, left[i].width * left[i].height * 4);
    if (!wasmPixels) {
      failures.push({ packet: index, id: right[i].id, reason: "wasm texture missing" });
      pixelMismatches += 1;
      continue;
    }
    const diff = pixelDiff(nativePixels, wasmPixels);
    const within = diff.max <= 2 && diff.mean <= 0.05;
    if (diff.max === 0) identicalPixelsDifferentId += 1;
    if (pixelNotes.length < 8) {
      pixelNotes.push({
        packet: index,
        native_id: left[i].id,
        wasm_id: right[i].id,
        max: diff.max,
        mean: diff.mean,
        within_tolerance: within,
      });
    }
    if (within) {
      idMap.set(left[i].id, right[i].id);
    } else {
      pixelMismatches += 1;
      failures.push({
        packet: index,
        native_id: left[i].id,
        wasm_id: right[i].id,
        max: diff.max,
        mean: diff.mean,
        reason: "pixels exceed max 2 and mean 0.05",
      });
    }
  }
}

function copyTexture(id, byteLength) {
  const bytes = encoder.encode(id);
  const ptr = allocCopy(bytes);
  const dataPtr = api.painter_texture_ptr(ptr, bytes.length);
  if (!dataPtr || !byteLength) {
    api.painter_free(ptr, bytes.length);
    return null;
  }
  // Copy before any other wasm call. A later alloc can move linear memory.
  const copy = view().slice(dataPtr, dataPtr + byteLength);
  api.painter_free(ptr, bytes.length);
  return copy;
}

function pixelDiff(left, right) {
  const n = Math.min(left.length, right.length);
  let max = Math.abs(left.length - right.length);
  let sum = 0;
  for (let i = 0; i < n; i += 1) {
    const delta = Math.abs(left[i] - right[i]);
    if (delta > max) max = delta;
    sum += delta;
  }
  return { max, mean: n ? sum / n : max };
}

function jsonClose(left, right, path) {
  if (typeof left === "number" && typeof right === "number") {
    const scale = Math.max(Math.abs(left), Math.abs(right), 1);
    if (Math.abs(left - right) <= 1e-9 * scale) return null;
    return `${path}: ${left} != ${right}`;
  }
  if (Array.isArray(left) && Array.isArray(right)) {
    if (left.length !== right.length) return `${path}: length ${left.length} != ${right.length}`;
    for (let i = 0; i < left.length; i += 1) {
      const reason = jsonClose(left[i], right[i], `${path}[${i}]`);
      if (reason) return reason;
    }
    return null;
  }
  if (isObject(left) && isObject(right)) {
    const leftKeys = Object.keys(left);
    const rightKeys = Object.keys(right);
    if (leftKeys.length !== rightKeys.length) {
      return `${path}: field count ${leftKeys.length} != ${rightKeys.length}`;
    }
    for (const key of leftKeys) {
      if (!Object.prototype.hasOwnProperty.call(right, key)) return `${path}: missing ${key}`;
      const reason = jsonClose(left[key], right[key], `${path}.${key}`);
      if (reason) return reason;
    }
    return null;
  }
  if (Object.is(left, right)) return null;
  return `${path}: ${JSON.stringify(left)} != ${JSON.stringify(right)}`;
}

function isObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function percentile(samples, p) {
  if (!samples.length) return null;
  const sorted = [...samples].sort((a, b) => a - b);
  const index = Math.min(sorted.length - 1, Math.max(0, Math.ceil(p * sorted.length) - 1));
  return sorted[index];
}

function parseArgs(argv) {
  const out = {
    wasm: "",
    frames: "",
    vfs: "",
    nativeDir: "",
    out: "wasm-parity-report.json",
    cards: "",
    lag: 3,
    fps: 30,
    width: 540,
    height: 900,
  };
  for (let i = 0; i < argv.length; i += 1) {
    const key = argv[i];
    const value = argv[i + 1];
    if (key === "--wasm") out.wasm = value;
    else if (key === "--frames") out.frames = value;
    else if (key === "--vfs") out.vfs = value;
    else if (key === "--native-dir") out.nativeDir = value;
    else if (key === "--out") out.out = value;
    else if (key === "--cards") out.cards = value;
    else if (key === "--lag") out.lag = Number(value);
    else if (key === "--fps") out.fps = Number(value);
    else if (key === "--width") out.width = Number(value);
    else if (key === "--height") out.height = Number(value);
    else throw new Error(`unknown argument ${key}`);
    i += 1;
  }
  for (const name of ["wasm", "frames", "vfs", "nativeDir", "cards"]) {
    if (!out[name]) throw new Error(`missing --${name === "nativeDir" ? "native-dir" : name}`);
  }
  return out;
}
