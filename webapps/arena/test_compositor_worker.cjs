/* Run with: node --test webapps/arena/test_compositor_worker.cjs */
const test = require("node:test");
const assert = require("node:assert/strict");
const {createCompositorSession} = require("./static/compositor-worker.js");

function stubCompositor(log) {
  return class {
    constructor(canvas, options) {
      this.options = options;
      this.stats = {backend: "webgl2"};
      this.metrics = {error: null};
      this.halted = false;
      log.created = options;
    }
    push(packet, timing) {
      log.pushes.push({packet, at: timing.at, pixels: this.options.textureSource("ab")});
    }
    reset() { log.resets++; }
    destroy() { log.destroyed = true; }
  };
}

function session(hasAnimationFrame = true) {
  const log = {pushes: [], resets: 0, posts: []};
  const compositor = createCompositorSession({
    postMessage: (message) => log.posts.push(message),
    SceneCompositor: stubCompositor(log),
    hasAnimationFrame,
    timeOrigin: 5000
  });
  const port = {onmessage: null, close() { log.portClosed = true; }};
  compositor.onMessage({type: "init", canvas: {}, port});
  return {compositor, log, send: (message) => port.onmessage({data: message})};
}

test("frames from the painter port are pushed with step time on this worker's clock and pixels are dropped after upload", () => {
  const {compositor, log, send} = session();
  assert.equal(log.created.renderOnPush, false);
  assert.equal(log.created.adaptiveDelay, true);
  assert.equal(log.posts[0].type, "compositor-ready");
  const buffer = new Uint8Array([1, 2, 3, 4]).buffer;
  send({type: "composite", packet: {sequence: 1}, textures: [{id: "ab", width: 1, height: 1, buffer}], evicted: [], at: 5123.5});
  assert.equal(log.pushes.length, 1);
  assert.equal(log.pushes[0].at, 123.5);
  assert.deepEqual([...log.pushes[0].pixels.rgba], [1, 2, 3, 4]);
  send({type: "composite", packet: {sequence: 2}, textures: [], evicted: []});
  assert.equal(log.pushes[1].pixels, null, "the CPU copy left after its upload");
  assert.equal(log.pushes[1].at, undefined);
  send({type: "reset"});
  assert.equal(log.resets, 1);
  compositor.onMessage({type: "destroy"});
  assert.equal(log.destroyed, true);
  assert.equal(log.portClosed, true);
});

test("without requestAnimationFrame in workers the compositor draws on push", () => {
  const {log} = session(false);
  assert.equal(log.created.renderOnPush, true);
});
