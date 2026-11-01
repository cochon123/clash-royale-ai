/* Run with: node --test webapps/arena/test_compositor.cjs */
const test = require("node:test");
const assert = require("node:assert/strict");
const {SceneCompositor, interpolateDraws} = require("./static/scene-compositor.js");

function canvas() {
  const context = {createImageData: (w, h) => ({data: new Uint8ClampedArray(w * h * 4)}), putImageData() {}};
  return {width: 0, height: 0, getContext: (kind) => kind === "2d" ? context : null, addEventListener() {}, removeEventListener() {}};
}
function sprite(overrides = {}) {
  return {id: "troop-12", kind: "sprite", tex: "ab", m: [1, 0, 0, 1, 0, 0], alpha: 1, blend: "normal", ...overrides};
}
function packet(sequence, overrides = {}) {
  return {v: 1, sequence, tick: sequence, display_tick: sequence, match_id: 1, width: 2, height: 1, clear: [19, 27, 31, 255], draws: [sprite()], textures: [], playback_speed: 1, ...overrides};
}
function compositor(pixels, options = {}) {
  const errors = [];
  const view = new SceneCompositor(canvas(), {
    textureSource: (id) => pixels.get(id) || null,
    forceCanvas: true,
    onError: (error) => errors.push(error.message),
    ...options
  });
  return {view, errors};
}
const red = () => ({width: 1, height: 1, rgba: new Uint8Array([200, 40, 40, 255])});

test("a local texture is uploaded and painted in the same turn, with nothing left pending", () => {
  const pixels = new Map([["ab", red()]]);
  const {view, errors} = compositor(pixels);
  view.push(packet(1, {draws: [sprite({m: [1, 0, 0, 1, 0, 0]})]}));
  assert.deepEqual(errors, []);
  assert.equal(view.stats.pending_frames, 0);
  assert.equal(view.stats.loading_textures, 0);
  assert.equal(view.stats.frames, 1);
  assert.equal(view.stats.texture_count, 1);
  assert.equal(view.shownNativeMs, 50);
  assert.deepEqual([...view.painter.output.data.slice(0, 4)], [200, 40, 40, 255]);
  view.destroy();
  assert.equal(view.stats.texture_count, 0);
});

test("a missing local texture fails the frame instead of holding the previous one", () => {
  const pixels = new Map([["ab", red()]]);
  const {view, errors} = compositor(pixels);
  view.push(packet(1));
  const shown = view.shownNativeMs;
  view.push(packet(2, {draws: [sprite({tex: "cd"})]}));
  assert.equal(view.halted, true);
  assert.match(errors[0], /not available/);
  assert.equal(view.shownNativeMs, shown);
  assert.equal(view.stats.pending_frames, 0);
  view.destroy();
});

test("interpolation still blends stable sprites across the 50 ms delay", () => {
  const pixels = new Map([["ab", red()]]);
  let clock = 1000;
  const {view} = compositor(pixels, {interpolationDelay: 50, now: () => clock});
  let shown;
  view.painter.paint = (_packet, draws) => { shown = draws; };
  view.push(packet(1, {display_tick: 0, tick: 0, playback_speed: 4, draws: [sprite({m: [1, 0, 0, 1, 0, 0]})]}));
  view.push(packet(2, {display_tick: 4, tick: 4, playback_speed: 4, draws: [sprite({m: [1, 0, 0, 1, 100, 0]})]}));
  view.render(clock + 25);
  assert.equal(shown[0].m[4], 50);
  assert.equal(view.stats.pending_frames, 0);
  view.destroy();
});

test("textureSource is the only pixel path; urls are not consulted", () => {
  let asked = null;
  const {view, errors} = compositor(new Map(), {textureSource: (id) => { asked = id; return red(); }});
  view.push(packet(1));
  assert.equal(asked, "ab");
  assert.deepEqual(errors, []);
  assert.equal(typeof view.options.textureUrl, "undefined");
  view.destroy();
});

test("a pose the painter still knows survives after it leaves the last three frames", () => {
  const pixels = new Map([["aa", red()], ["bb", red()], ["cc", red()]]);
  const {view, errors} = compositor(pixels, {maxTextureBytes: 8});
  const draw = (sequence, tex, extra = {}) => packet(sequence, {draws: [sprite({tex, id: "troop-" + sequence})], ...extra});
  view.push(draw(1, "aa"));
  view.push(draw(2, "bb"));
  view.push(draw(3, "bb"));
  view.push(draw(4, "bb"));
  assert.equal(view.textures.has("aa"), true);
  view.push(draw(5, "cc"));
  assert.equal(view.halted, true);
  assert.match(errors[0], /texture cache budget/);
  assert.equal(view.textures.has("aa"), true);
  view.destroy();
});

test("painter eviction frees a pose before the replacement is uploaded, and the pose can return", () => {
  const pixels = new Map([["aa", red()], ["bb", red()], ["cc", red()]]);
  const {view, errors} = compositor(pixels, {maxTextureBytes: 8});
  const draw = (sequence, tex, extra = {}) => packet(sequence, {draws: [sprite({tex, id: "troop-" + sequence})], ...extra});
  view.push(draw(1, "aa"));
  view.push(draw(2, "bb"));
  view.push(draw(3, "bb"));
  view.push(draw(4, "bb"));
  view.push(draw(5, "cc", {evicted: ["aa"]}));
  assert.deepEqual(errors, []);
  assert.equal(view.textures.has("aa"), false);
  assert.equal(view.textures.has("cc"), true);
  view.push(draw(6, "cc"));
  view.push(draw(7, "cc"));
  view.push(draw(8, "aa", {evicted: ["bb"]}));
  assert.deepEqual(errors, []);
  assert.equal(view.halted, false);
  assert.equal(view.textures.has("aa"), true);
  assert.equal(view.textures.has("bb"), false);
  view.destroy();
});

test("moved interpolation still refuses to lerp a texture change without anchors", () => {
  const older = [sprite()];
  const newer = [sprite({tex: "cd", m: [2, 0, 0, 2, 10, 20]})];
  const middle = interpolateDraws(older, newer, 0.5);
  assert.equal(middle[0].tex, "ab");
  assert.deepEqual(middle[0].m, [1, 0, 0, 1, 0, 0]);
});
