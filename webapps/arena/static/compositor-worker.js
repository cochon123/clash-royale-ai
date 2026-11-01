/* Compositor worker: owns the battle canvas and draws on every display frame.
   The painter worker runs the wasm Painter on its own thread and sends each
   frame here over a MessagePort, so a slow painter step never holds a frame. */
(function (root) {
  "use strict";

  function createCompositorSession(env) {
    const post = (message) => env.postMessage(message);
    const state = {compositor: null, port: null, pixels: new Map(), destroyed: false, frames: 0, epoch: env.timeOrigin || 0};

    function fail(error) {
      post({type: "error", message: error instanceof Error ? error.message : String(error)});
    }
    function onFrame(message) {
      if (state.destroyed || !message) return;
      if (message.type === "reset") {
        state.pixels.clear();
        if (state.compositor) state.compositor.reset();
        return;
      }
      if (message.type !== "composite" || !state.compositor) return;
      for (const tex of message.textures || []) state.pixels.set(tex.id, {width: tex.width, height: tex.height, rgba: new Uint8Array(tex.buffer)});
      // The painter stamps frames with its own performance.timeOrigin + now().
      const at = Number.isFinite(message.at) ? message.at - state.epoch : undefined;
      state.compositor.push(message.packet, {at});
      // The GPU copy is the only one kept; the painter can rebuild a pose.
      for (const tex of message.textures || []) state.pixels.delete(tex.id);
      for (const id of message.evicted || []) state.pixels.delete(id);
      if (state.compositor.halted) fail(state.compositor.metrics.error || "compositor halted");
      if (++state.frames % 30 === 0) post({type: "stats", stats: state.compositor.stats});
    }
    function init(message) {
      try {
        state.compositor = new env.SceneCompositor(message.canvas, {
          textureSource: (id) => state.pixels.get(id) || null,
          alpha: false,
          // Without requestAnimationFrame in workers, draw each frame as it comes.
          renderOnPush: !env.hasAnimationFrame,
          adaptiveDelay: true,
          onError: fail
        });
        if (state.compositor.stats.backend !== "webgl2") {
          state.compositor.destroy();
          state.compositor = null;
          fail(new Error("OffscreenCanvas WebGL2 was not available after transfer"));
          return;
        }
      } catch (error) {
        state.compositor = null;
        fail(error);
        return;
      }
      state.port = message.port;
      state.port.onmessage = (event) => onFrame(event.data);
      post({type: "compositor-ready"});
    }
    function destroy() {
      if (state.destroyed) return;
      state.destroyed = true;
      if (state.port) state.port.close();
      if (state.compositor) state.compositor.destroy();
      state.pixels.clear();
    }
    function onMessage(message) {
      if (!message || state.destroyed) return;
      if (message.type === "init") init(message);
      else if (message.type === "destroy") destroy();
    }
    return {onMessage, onFrame, destroy, compositor: () => state.compositor};
  }

  root.createCompositorSession = createCompositorSession;
  if (typeof module !== "undefined" && module.exports) module.exports = {createCompositorSession};

  const inWorker = typeof WorkerGlobalScope !== "undefined" && typeof self !== "undefined" && self instanceof WorkerGlobalScope;
  if (inWorker) {
    importScripts("scene-compositor.js");
    const session = createCompositorSession({
      postMessage: (message) => self.postMessage(message),
      SceneCompositor: self.SceneCompositor,
      hasAnimationFrame: typeof self.requestAnimationFrame === "function",
      timeOrigin: performance.timeOrigin
    });
    self.onmessage = (event) => session.onMessage(event.data);
  }
})(typeof globalThis !== "undefined" ? globalThis : this);
