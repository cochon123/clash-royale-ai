/* Run with: node --test webapps/arena/test_painter_worker.cjs */
const test = require("node:test");
const assert = require("node:assert/strict");
const zlib = require("node:zlib");
const {createPainterSession} = require("./static/painter-worker.js");
const {createFakePainter, decodePng} = require("./static/fake-painter.js");
const {JitterClock, lagTicksForDisplay} = require("./static/jitter-clock.js");
const {SceneCompositor} = require("./static/scene-compositor.js");
const {planPackFiles, resolveAssetUrl} = require("./static/asset-plan.js");

function crc32(bytes) {
  let crc = ~0;
  for (const value of bytes) {
    crc ^= value;
    for (let bit = 0; bit < 8; bit++) crc = (crc >>> 1) ^ (0xedb88320 & -(crc & 1));
  }
  return (~crc) >>> 0;
}
function chunk(type, data) {
  const body = Buffer.concat([Buffer.from(type), data]);
  const out = Buffer.alloc(12 + data.length);
  out.writeUInt32BE(data.length, 0);
  body.copy(out, 4);
  out.writeUInt32BE(crc32(body), 8 + data.length);
  return out;
}
function encodePng(width, height, rgba) {
  const raw = Buffer.alloc(height * (1 + width * 4));
  for (let y = 0; y < height; y++) {
    raw[y * (1 + width * 4)] = 0;
    Buffer.from(rgba).copy(raw, y * (1 + width * 4) + 1, y * width * 4, (y + 1) * width * 4);
  }
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0); ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8; ihdr[9] = 6;
  return Buffer.concat([
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
    chunk("IHDR", ihdr),
    chunk("IDAT", zlib.deflateSync(raw)),
    chunk("IEND", Buffer.alloc(0))
  ]);
}

test("fixture png bytes decode to the authored rgba", () => {
  const rgba = new Uint8Array([10, 20, 30, 255, 1, 2, 3, 4]);
  const decoded = decodePng(encodePng(2, 1, rgba));
  assert.equal(decoded.width, 2);
  assert.equal(decoded.height, 1);
  assert.deepEqual([...decoded.rgba], [...rgba]);
});

test("pack planning keeps always plus the match cards and resolves content urls", () => {
  const manifest = {
    v: 1,
    painter_wasm: "files/wasm.bin",
    files: {
      "presentation.json": {url: "files/always.json", bytes: 10, sha256: "a"},
      "atlases/hog.png": {url: "files/hog.png", bytes: 40, sha256: "b"},
      "atlases/other.png": {url: "files/other.png", bytes: 99, sha256: "c"},
      "painter.wasm": {url: "files/wasm.bin", bytes: 7, sha256: "d"}
    },
    always: ["presentation.json"],
    cards: {HogRider: ["atlases/hog.png"], Wizard: ["atlases/other.png"]}
  };
  const plan = planPackFiles(manifest, ["HogRider"]);
  assert.deepEqual(plan.files.map((file) => file.path), ["presentation.json", "atlases/hog.png"]);
  assert.equal(plan.wasm.url, "files/wasm.bin");
  assert.equal(plan.bytes, 57);
  assert.equal(resolveAssetUrl("https://arena.test/clashroyale/assets/manifest-abc.json", plan.files[1].url), "https://arena.test/clashroyale/assets/files/hog.png");
  assert.throws(() => resolveAssetUrl("https://arena.test/assets/manifest.json", "../secret"), /not relative/);
});

function recording(ticks = 6) {
  const frames = [];
  for (let tick = 0; tick < ticks; tick++) {
    frames.push({
      tick,
      packet: {
        v: 1, sequence: tick + 1, tick, display_tick: tick, match_id: 1, width: 2, height: 1,
        clear: [19, 27, 31, 255], status: "running", playback_speed: 1,
        layout: {field: [0, 0, 2, 1], world: [18000, 32000], cards: []},
        draws: [{id: "entity-1-part-0", kind: "sprite", tex: "ab", m: [1, 0, 0, 1, tick, 0], alpha: 1, blend: "normal"}]
      }
    });
  }
  return {width: 2, height: 1, frames, textures: {ab: {width: 1, height: 1, rgba: new Uint8Array([9, 8, 7, 255])}}};
}
function manifest() {
  return {
    v: 1,
    painter_wasm: "files/painter.wasm",
    files: {"presentation.json": {url: "files/presentation.json", bytes: 4, sha256: "aa"}},
    always: ["presentation.json"],
    cards: {HogRider: []}
  };
}
function harness(options = {}) {
  const posts = [];
  let stepFn = null;
  const files = {"https://arena.test/assets/manifest.json": {current: "manifest-demo.json"}, "https://arena.test/assets/manifest-demo.json": manifest(), "https://arena.test/assets/files/presentation.json": new Uint8Array([1, 2, 3, 4]).buffer};
  const session = createPainterSession({
    postMessage: (message) => posts.push(message),
    now: options.now || (() => 1000),
    origin: "https://arena.test/play.html",
    useEventSource: false,
    fetch: async (url) => {
      const body = files[url];
      if (!body) return {ok: false, status: 404, json: async () => ({}), arrayBuffer: async () => new ArrayBuffer(0)};
      const encoded = body instanceof ArrayBuffer ? null : Buffer.from(JSON.stringify(body));
      return {ok: true, status: 200, clone() { return this; }, json: async () => JSON.parse(encoded.toString()), arrayBuffer: async () => body instanceof ArrayBuffer ? body : encoded.buffer.slice(encoded.byteOffset, encoded.byteOffset + encoded.byteLength)};
    },
    setInterval: (fn) => { stepFn = fn; return 1; },
    clearInterval: () => { stepFn = null; },
    JitterClock, lagTicksForDisplay, SceneCompositor, createFakePainter, planPackFiles, resolveAssetUrl
  });
  return {session, posts, step: () => stepFn && stepFn()};
}
const meta = {protocol: 2, assets: {manifest: "/assets/manifest.json"}, cards: ["HogRider"], mode: "watch", labels: ["Red", "Blue"], id: "demo"};
function ticks(from, count, hud = {}) {
  const items = [];
  for (let tick = from; tick < from + count; tick++) {
    items.push({snapshot: {tick, runtime_status: "running", playback_speed: 1, match_id: 1}, hud: {status: "running", tick, match_number: 1, speed: 1, hands: [[], []], ...hud}});
  }
  return JSON.stringify({v: 2, items});
}

test("the worker observes every buffered tick before it steps and reports asset progress", async () => {
  const {session, posts} = harness();
  await session.onMessage({type: "init", base: "", matchId: "demo", painter: "fake", recording: recording(), fallback: true, width: 2, height: 1});
  assert.equal(posts.some((message) => message.type === "need-ticks"), true);
  await session.onMessage({type: "sse", event: "ticks", data: ticks(0, 6)});
  assert.equal(session.observed(), 0);
  assert.equal(session.backlog(), 1);
  await session.onMessage({type: "sse", event: "meta", data: JSON.stringify(meta)});
  assert.equal(session.observed(), 6);
  assert.equal(session.backlog(), 0);
  const progress = posts.filter((message) => message.type === "progress");
  assert.ok(progress.length >= 1);
  assert.equal(progress.at(-1).loaded, 4);
  assert.equal(progress.at(-1).total, 4);
  session.stepOnce();
  assert.ok(session.painter().displayTick() > 0);
  assert.equal(session.backlog(), 0);
});

test("a tick that arrives while a frame is posted is observed without waiting for an ack", async () => {
  const {session, posts} = harness();
  await session.onMessage({type: "init", base: "", matchId: "demo", painter: "fake", recording: recording(8), fallback: true, width: 2, height: 1});
  await session.onMessage({type: "sse", event: "meta", data: JSON.stringify(meta)});
  await session.onMessage({type: "sse", event: "ticks", data: ticks(0, 5)});
  const before = posts.length;
  const stepped = session.stepOnce();
  assert.equal(stepped, 1);
  const composite = posts.slice(before).find((message) => message.type === "composite");
  assert.ok(composite);
  assert.equal(composite.textures.length, 1);
  assert.equal(composite.textures[0].id, "ab");
  assert.equal(composite.textures[0].width, 1);
  assert.equal(composite.textures[0].buffer.byteLength, 4);
  assert.equal(composite.packet.draws[0].tex, "ab");
  const seen = session.observed();
  await session.onMessage({type: "sse", event: "ticks", data: ticks(5, 2)});
  assert.equal(session.observed(), seen + 2);
  assert.equal(session.backlog(), 0);
  const again = session.painter().displayTick();
  session.stepOnce();
  assert.ok(session.painter().displayTick() > again);
});

test("ordered ticks are rejected when a snapshot goes backwards, and round resets the painter", async () => {
  const {session, posts} = harness();
  await session.onMessage({type: "init", base: "", matchId: "demo", painter: "fake", recording: recording(), fallback: true});
  await session.onMessage({type: "sse", event: "meta", data: JSON.stringify(meta)});
  await session.onMessage({type: "sse", event: "ticks", data: ticks(0, 3)});
  await session.onMessage({type: "sse", event: "ticks", data: ticks(1, 1)});
  assert.equal(posts.some((message) => message.type === "error" && /order/.test(message.message)), true);
  posts.length = 0;
  await session.onMessage({type: "sse", event: "round", data: "{}"});
  assert.equal(posts.some((message) => message.type === "round"), true);
  assert.equal(session.painter().displayTick(), null);
  await session.onMessage({type: "sse", event: "ticks", data: ticks(0, 2)});
  assert.equal(session.painter().observed.length, 2);
});

test("non-protocol servers are left to the legacy stream without booting a painter", async () => {
  const {session, posts} = harness();
  await session.onMessage({type: "init", base: "", matchId: "demo", painter: "auto"});
  await session.onMessage({type: "sse", event: "meta", data: JSON.stringify({protocol: 1, labels: ["A", "B"], mode: "watch"})});
  assert.equal(posts.some((message) => message.type === "legacy"), true);
  assert.equal(session.painter(), null);
});

test("diagnostics name draw percentiles, lag, jitter, freezes and asset bytes", async () => {
  let wall = 0;
  const {session} = harness({now: () => wall});
  await session.onMessage({type: "init", base: "", matchId: "demo", painter: "fake", recording: recording(12), fallback: true});
  await session.onMessage({type: "sse", event: "meta", data: JSON.stringify(meta)});
  for (let tick = 0; tick < 8; tick++) {
    wall = tick * 50;
    await session.onMessage({type: "sse", event: "ticks", data: ticks(tick, 1)});
  }
  const live = session.diagnostics();
  assert.ok(live.lag_ms >= 100 && live.lag_ms <= 300);
  assert.equal(live.jitter_ms, 50);
  wall = 350 + 500;
  session.stepOnce();
  const diag = session.diagnostics();
  assert.equal(diag.lag_ms, 0);
  assert.equal(typeof diag.painter.draw_ms_p50, "number");
  assert.equal(typeof diag.painter.draw_ms_p95, "number");
  assert.equal(typeof diag.painter.wasm_mb, "number");
  assert.equal(diag.painter.observe_backlog, 0);
  assert.equal(typeof diag.freezes.count, "number");
  assert.equal(diag.asset_bytes, 4);
  assert.ok(session.displayTrace().length >= 1);
  assert.equal(session.displayTrace()[0].length, 4);
  const trace = session.displayTrace();
  assert.ok(trace.at(-1)[1] >= trace[0][1]);
});
