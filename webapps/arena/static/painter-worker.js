/* Painter worker: packs, wasm (or the fake painter), ordered ticks, 30 Hz step, local composite.
   A tick is observed as soon as it arrives. Compositing never gates that. */
(function (root) {
  "use strict";
  const STEP_MS = 1000 / 30;
  const EVENTS = ["meta", "ticks", "status", "closed", "error", "round", "resync"];

  function percentile(samples, p) {
    if (!samples.length) return 0;
    const sorted = samples.slice().sort((a, b) => a - b);
    return sorted[Math.min(sorted.length - 1, Math.max(0, Math.ceil(sorted.length * p) - 1))];
  }
  function round2(value) { return Math.round(value * 100) / 100; }

  function createWasmPainter(instance) {
    const exp = instance.exports;
    const encode = new TextEncoder();
    const decode = new TextDecoder();
    const memory = () => new Uint8Array(exp.memory.buffer);
    function write(text) {
      const bytes = typeof text === "string" ? encode.encode(text) : text;
      const ptr = exp.painter_alloc(bytes.length);
      memory().set(bytes, ptr);
      return {ptr, len: bytes.length, bytes};
    }
    function free(block) { if (block) exp.painter_free(block.ptr, block.len); }
    function readResult() {
      const ptr = exp.painter_result_ptr();
      const len = exp.painter_result_len();
      return decode.decode(memory().slice(ptr, ptr + len));
    }
    return {
      vfsPut(path, data) {
        const pathBlock = write(path), dataBlock = write(data);
        try { return exp.painter_vfs_put(pathBlock.ptr, pathBlock.len, dataBlock.ptr, dataBlock.len); }
        finally { free(pathBlock); free(dataBlock); }
      },
      init(cardsJson, width, height) {
        const block = write(cardsJson);
        try { return exp.painter_init(block.ptr, block.len, width, height); }
        finally { free(block); }
      },
      setLagTicks(value) { exp.painter_set_lag_ticks(value); },
      observe(json) {
        const block = write(json);
        try { return exp.painter_observe(block.ptr, block.len); }
        finally { free(block); }
      },
      step(dt) { return exp.painter_step(dt); },
      result() { return readResult(); },
      texture(id) {
        const block = write(id);
        try {
          const ptr = exp.painter_texture_ptr(block.ptr, block.len);
          const len = exp.painter_texture_len(block.ptr, block.len);
          if (!ptr || len <= 0) return null;
          return {rgba: memory().slice(ptr, ptr + len)};
        } finally { free(block); }
      },
      textureRelease(id) {
        const block = write(id);
        try { exp.painter_texture_release(block.ptr, block.len); }
        finally { free(block); }
      },
      prewarm() { return exp.painter_prewarm ? exp.painter_prewarm() : 0; },
      reset() { exp.painter_reset(); },
      statsJson() { exp.painter_stats_json(); return readResult(); },
      lastError() { exp.painter_last_error(); return readResult(); }
    };
  }

  function createPainterSession(env) {
    const post = (message, transfer) => env.postMessage(message, transfer || []);
    const now = env.now || (() => performance.now());
    const fetchImpl = env.fetch;
    const setTimer = env.setInterval || setInterval;
    const clearTimer = env.clearInterval || clearInterval;
    const clock = new env.JitterClock();
    const state = {
      painter: null, compositor: null, fallback: false, canvas: null, ready: false,
      base: "", matchId: "", width: 540, height: 900, mode: "auto", recordingUrl: "", recording: null,
      buffer: [], observed: 0, observeBacklog: 0, newestTick: null, oldestTick: null,
      speed: 1, flow: "running", assetBytes: 0, drawSamples: [], trace: [], freezes: {count: 0, total_ms: 0},
      lastShown: null, lastAdvanceWall: null, freezeStart: null, timer: 0, lastStep: 0, t0: 0,
      sentTextures: new Set(), pixels: new Map(), destroyed: false, meta: null, viewReady: false,
      lagMs: 150, jitterMs: 0, appliedLagTicks: 3, booting: null, stepFn: null
    };
    let chain = Promise.resolve();

    function fail(error) {
      const message = error instanceof Error ? error.message : String(error);
      post({type: "error", message});
    }
    function diagnostics() {
      let wasmMb = 0;
      if (state.painter && state.painter.statsJson) {
        try {
          const stats = JSON.parse(state.painter.statsJson());
          wasmMb = Number.isFinite(stats.wasm_mb) ? stats.wasm_mb : (Number(stats.wasm_bytes) || 0) / (1024 * 1024);
        } catch { wasmMb = 0; }
      }
      return {
        painter: {
          draw_ms_p50: round2(percentile(state.drawSamples, 0.5)),
          draw_ms_p95: round2(percentile(state.drawSamples, 0.95)),
          wasm_mb: round2(wasmMb),
          observe_backlog: state.observeBacklog,
          display_tick: state.painter && state.painter.displayTick ? state.painter.displayTick() : null,
          error: state.compositor && state.compositor.metrics ? state.compositor.metrics.error : null
        },
        lag_ms: round2(state.lagMs),
        jitter_ms: round2(state.jitterMs),
        freezes: {count: state.freezes.count, total_ms: Math.round(state.freezes.total_ms)},
        asset_bytes: state.assetBytes
      };
    }
    function absolute(url) {
      return new URL(url, env.origin || (typeof location !== "undefined" ? location.href : "http://localhost/")).href;
    }
    async function getResponse(url) {
      const response = await fetchImpl(url, {credentials: "same-origin"});
      if (!response.ok) throw new Error("asset fetch failed (" + response.status + "): " + url);
      return response;
    }
    function applyLag() {
      const wall = now();
      state.jitterMs = clock.jitterMs;
      state.lagMs = clock.appliedLagMs(wall);
      const desired = clock.lagTicks(wall, state.speed || 1);
      const display = state.painter && state.painter.displayTick ? state.painter.displayTick() : null;
      state.appliedLagTicks = env.lagTicksForDisplay(desired, display, state.newestTick);
      if (state.painter) state.painter.setLagTicks(state.appliedLagTicks);
    }
    function notePresented(shown) {
      const wall = now() - state.t0;
      const shownTick = shown / 50;
      const behind = state.newestTick == null ? 0 : Math.max(0, Math.round(state.newestTick - shownTick));
      state.trace.push([Math.round(wall), Math.round(shown), Math.max(state.observeBacklog, behind), 0]);
      if (state.trace.length > 12000) state.trace.splice(0, state.trace.length - 12000);
      post({type: "trace", sample: state.trace[state.trace.length - 1]});
      if (state.flow !== "running") {
        state.freezeStart = null;
        state.lastShown = shown;
        state.lastAdvanceWall = wall;
        return;
      }
      if (state.lastShown == null || shown > state.lastShown + 0.5) {
        if (state.freezeStart != null && wall - state.freezeStart > 100) {
          state.freezes.count++;
          state.freezes.total_ms += wall - state.freezeStart;
        }
        state.freezeStart = null;
        state.lastShown = shown;
        state.lastAdvanceWall = wall;
      } else if (state.freezeStart == null) state.freezeStart = state.lastAdvanceWall ?? wall;
    }
    function takeTexture(id, descriptor) {
      const tex = state.painter.texture(id);
      if (!tex || !tex.rgba) return null;
      const rgba = tex.rgba instanceof Uint8Array ? tex.rgba.slice() : new Uint8Array(tex.rgba);
      state.painter.textureRelease(id);
      return {id, width: tex.width || descriptor.width, height: tex.height || descriptor.height, rgba};
    }
    function drawTextureIds(draws, out = []) {
      for (const draw of draws || []) {
        if (!draw) continue;
        if (draw.kind === "sprite" && draw.tex) out.push(draw.tex);
        else if (draw.kind === "shadow") drawTextureIds(draw.draws, out);
      }
      return out;
    }
    function rememberTexture(id, descriptor) {
      if (state.pixels.has(id)) return true;
      if (state.compositor && state.compositor.textures.has(id)) return true;
      const record = takeTexture(id, descriptor || {});
      if (!record || !record.width || !record.height || !record.rgba) return false;
      state.pixels.set(id, record);
      state.sentTextures.add(id);
      return true;
    }
    function publish(packet) {
      const textures = [];
      for (const descriptor of packet.textures || []) if (rememberTexture(descriptor.id, descriptor)) textures.push(state.pixels.get(descriptor.id));
      const missing = [];
      for (const id of drawTextureIds(packet.draws)) if (!rememberTexture(id, {})) missing.push(id);
      if (missing.length) throw new Error("Texture is not available: " + missing.join(","));
      for (const id of packet.evicted || []) {
        if ([...state.compositor ? state.compositor.protectedIds() : []].includes(id)) continue;
        state.pixels.delete(id);
        state.sentTextures.delete(id);
      }
      if (state.compositor) {
        state.compositor.push(packet);
        if (state.compositor.halted) throw new Error(state.compositor.metrics.error || "compositor halted");
        for (const record of textures) state.pixels.delete(record.id);
        notePresented(state.compositor.shownNativeMs);
      } else if (state.fallback) {
        const payload = textures.map((record) => ({id: record.id, width: record.width, height: record.height, buffer: record.rgba.buffer}));
        post({type: "composite", packet, textures: payload, evicted: packet.evicted || []}, payload.map((item) => item.buffer));
        for (const record of textures) state.pixels.delete(record.id);
        notePresented(Number(packet.display_tick) * 50);
      }
      if (packet.layout) post({type: "layout", layout: packet.layout, match_id: packet.match_id});
      if (!state.viewReady && (state.compositor || state.fallback)) {
        state.viewReady = true;
        post({type: "ready"});
      }
    }
    function stepOnce() {
      if (!state.painter || state.destroyed) return 0;
      const wall = now();
      let dt = state.lastStep ? (wall - state.lastStep) / 1000 : STEP_MS / 1000;
      state.lastStep = wall;
      if (!(dt > 0)) dt = STEP_MS / 1000;
      if (dt > 0.25) dt = 0.25;
      applyLag();
      const started = now();
      const rc = state.painter.step(dt);
      if (rc < 0) throw new Error(state.painter.lastError() || "painter step failed");
      if (rc === 1) publish(JSON.parse(state.painter.result()));
      state.drawSamples.push(now() - started);
      if (state.drawSamples.length > 240) state.drawSamples.shift();
      if (state.drawSamples.length % 30 === 0) post({type: "diagnostics", diagnostics: diagnostics()});
      return rc;
    }
    function startLoop() {
      if (state.timer || state.destroyed || !state.painter) return;
      if (!state.compositor && !state.fallback) return;
      state.lastStep = now();
      state.stepFn = () => { try { stepOnce(); } catch (error) { fail(error); } };
      state.timer = setTimer(state.stepFn, STEP_MS);
    }
    function resetPainter(kind) {
      if (state.painter) state.painter.reset();
      state.oldestTick = null;
      state.newestTick = null;
      state.sentTextures.clear();
      state.pixels.clear();
      state.lastShown = null;
      state.freezeStart = null;
      if (state.compositor) state.compositor.reset();
      post({type: kind});
    }
    function observeItem(item) {
      const snapshot = item.snapshot;
      if (!snapshot) return;
      if (item.hud) post({type: "hud", hud: item.hud});
      const rc = state.painter.observe(JSON.stringify(snapshot));
      if (rc) throw new Error(state.painter.lastError() || "observe failed");
      state.observed++;
      const tick = Number(snapshot.tick);
      if (state.oldestTick == null || tick < state.oldestTick) state.oldestTick = tick;
      if (Number.isFinite(tick)) state.newestTick = tick;
      state.speed = Number(snapshot.playback_speed || (item.hud && item.hud.speed) || state.speed || 1);
      const status = snapshot.runtime_status || (item.hud && item.hud.status) || "running";
      state.flow = status === "paused" ? "paused" : status === "ended" || status === "closed" ? "finished" : "running";
      clock.noteArrival(now());
      applyLag();
    }
    function handleReadyEvent(event, data) {
      if (event === "round" || event === "resync") { resetPainter(event); return; }
      if (event === "ticks") {
        const body = JSON.parse(data);
        for (const item of body.items || []) observeItem(item);
        state.observeBacklog = 0;
        return;
      }
      if (event === "status") post({type: "status", status: JSON.parse(data)});
      else if (event === "error") post({type: "error", message: (JSON.parse(data).error) || "match error"});
      else if (event === "closed") post({type: "closed"});
    }
    function flushBuffer() {
      const pending = state.buffer.splice(0);
      for (const entry of pending) handleReadyEvent(entry.event, entry.data);
      state.observeBacklog = 0;
    }
    async function boot(meta) {
      const manifestUrl = absolute(meta.assets.manifest);
      let manifest = await (await getResponse(manifestUrl)).json();
      let resolved = manifestUrl;
      if (manifest.current && !manifest.v) {
        resolved = env.resolveAssetUrl(manifestUrl, manifest.current);
        manifest = await (await getResponse(resolved)).json();
      }
      const plan = env.planPackFiles(manifest, meta.cards || []);
      let loaded = 0;
      const total = plan.bytes;
      let cursor = 0;
      async function fetchOne() {
        while (cursor < plan.files.length) {
          const file = plan.files[cursor++];
          const bytes = new Uint8Array(await (await getResponse(env.resolveAssetUrl(resolved, file.url))).arrayBuffer());
          file.bytesLoaded = bytes;
          loaded += file.bytes || bytes.byteLength;
          state.assetBytes = loaded;
          post({type: "progress", loaded, total: total || loaded, phase: "assets"});
        }
      }
      await Promise.all([fetchOne(), fetchOne(), fetchOne(), fetchOne()]);
      if (state.mode === "fake" || state.recording || state.recordingUrl) {
        const recording = state.recording || await (await getResponse(absolute(state.recordingUrl))).json();
        state.painter = await env.createFakePainter(recording);
      } else {
        if (!plan.wasm) throw new Error("asset manifest has no painter wasm");
        const wasmUrl = env.resolveAssetUrl(resolved, plan.wasm.url);
        const response = await getResponse(wasmUrl);
        const streaming = env.instantiateStreaming || ((body, imports) => WebAssembly.instantiateStreaming(body, imports));
        let compiled;
        try { compiled = await streaming(response.clone(), {}); }
        catch { compiled = await WebAssembly.instantiate(await response.arrayBuffer(), {}); }
        state.painter = createWasmPainter(compiled.instance || compiled);
        const wasmBytes = plan.wasm.bytes || 0;
        if (wasmBytes) { loaded += wasmBytes; state.assetBytes = loaded; post({type: "progress", loaded, total: total || loaded, phase: "wasm"}); }
      }
      for (const file of plan.files) {
        const rc = state.painter.vfsPut(file.path, file.bytesLoaded);
        if (rc) throw new Error(state.painter.lastError() || "painter rejected " + file.path);
        file.bytesLoaded = null;
      }
      const rc = state.painter.init(JSON.stringify(meta.cards || []), state.width, state.height);
      if (rc) throw new Error(state.painter.lastError() || "painter_init failed");
      // Unpack every sprite sheet on the loading screen, not when a unit first appears.
      post({type: "progress", loaded, total: total || loaded, phase: "prewarm"});
      if (state.painter.prewarm && state.painter.prewarm()) throw new Error(state.painter.lastError() || "painter_prewarm failed");
      state.ready = true;
      if (!state.canvas && !state.fallback) post({type: "need-canvas"});
      flushBuffer();
      startLoop();
    }
    async function onSse(event, data) {
      if (event === "meta") {
        const meta = JSON.parse(data);
        state.meta = meta;
        if (meta.protocol !== 2 && state.mode !== "fake") { post({type: "legacy"}); return; }
        post({type: "meta", meta});
        await boot(meta);
        return;
      }
      if (!state.ready) {
        state.buffer.push({event, data});
        state.observeBacklog = state.buffer.filter((entry) => entry.event === "ticks").length;
        return;
      }
      handleReadyEvent(event, data);
    }
    function attachCanvas(message) {
      state.fallback = !message.canvas || !!message.fallback;
      state.canvas = message.canvas || null;
      if (state.canvas) {
        try {
          state.compositor = new env.SceneCompositor(state.canvas, {textureSource: (id) => state.pixels.get(id) || null, alpha: false});
          if (state.compositor.stats.backend !== "webgl2") {
            state.compositor.destroy();
            state.compositor = null;
            state.fallback = true;
            post({type: "error", message: "OffscreenCanvas WebGL2 was not available after transfer"});
            return;
          }
          state.fallback = false;
        } catch (error) {
          state.compositor = null;
          state.fallback = false;
          fail(error);
          return;
        }
      }
      startLoop();
    }
    function connectStream() {
      const url = (state.base || "") + "/api/matches/" + encodeURIComponent(state.matchId) + "/ticks";
      if (env.useEventSource !== false && typeof EventSource !== "undefined") {
        const source = new EventSource(url);
        state.eventSource = source;
        for (const name of EVENTS) source.addEventListener(name, (event) => { onMessage({type: "sse", event: name, data: event.data}); });
        source.onerror = () => { if (!state.meta && !state.destroyed) post({type: "legacy", reason: "ticks stream failed"}); };
        return;
      }
      post({type: "need-ticks", url});
    }
    async function handle(message) {
      if (state.destroyed || !message) return;
      if (message.type === "init") {
        state.base = message.base || "";
        state.matchId = message.matchId;
        state.width = message.width || 540;
        state.height = message.height || 900;
        state.mode = message.painter || "auto";
        state.recordingUrl = message.recordingUrl || "";
        state.recording = message.recording || null;
        state.t0 = now();
        state.lastStep = state.t0;
        if (message.canvas || message.fallback) attachCanvas(message);
        connectStream();
        return;
      }
      if (message.type === "canvas") { attachCanvas(message); return; }
      if (message.type === "sse") { await onSse(message.event, message.data); return; }
      if (message.type === "destroy") destroy();
    }
    function onMessage(message) {
      chain = chain.then(() => handle(message)).catch((error) => fail(error));
      return chain;
    }
    function destroy() {
      if (state.destroyed) return;
      state.destroyed = true;
      if (state.timer) clearTimer(state.timer);
      state.timer = 0;
      if (state.eventSource) state.eventSource.close();
      if (state.compositor) state.compositor.destroy();
    }
    return {
      onMessage, stepOnce, destroy, diagnostics,
      displayTrace: () => state.trace.slice(),
      observed: () => state.observed,
      backlog: () => state.observeBacklog,
      painter: () => state.painter,
      lagMs: () => state.lagMs,
      jitterMs: () => state.jitterMs
    };
  }

  root.createPainterSession = createPainterSession;
  root.createWasmPainter = createWasmPainter;
  if (typeof module !== "undefined" && module.exports) module.exports = {createPainterSession, createWasmPainter};

  const inWorker = typeof WorkerGlobalScope !== "undefined" && typeof self !== "undefined" && self instanceof WorkerGlobalScope;
  if (inWorker) {
    importScripts("jitter-clock.js", "scene-compositor.js", "fake-painter.js", "asset-plan.js");
    const session = createPainterSession({
      postMessage: (message, transfer) => self.postMessage(message, transfer || []),
      JitterClock: self.JitterClock,
      lagTicksForDisplay: self.lagTicksForDisplay,
      SceneCompositor: self.SceneCompositor,
      createFakePainter: self.createFakePainter,
      planPackFiles: self.planPackFiles,
      resolveAssetUrl: self.resolveAssetUrl,
      fetch: self.fetch.bind(self),
      now: () => performance.now(),
      origin: self.location.href
    });
    self.onmessage = (event) => { session.onMessage(event.data); };
  }
})(typeof globalThis !== "undefined" ? globalThis : this);
