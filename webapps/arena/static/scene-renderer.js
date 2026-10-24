/* Original-asset display-list compositor. No simulation or replacement artwork lives here.
   Painters and interpolation live in scene-compositor.js so the worker can use them. */
(function (root) {
  "use strict";
  const shared = typeof module !== "undefined" && module.exports ? require("./scene-compositor.js") : root;
  const {collectTextures, interpolateDraws, SoftwarePainter, GLPainter} = shared;
  const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));
  const number = (v, otherwise = 0) => Number.isFinite(v) ? v : otherwise;
  const finite = v => typeof v === "number" && Number.isFinite(v);
  const textureId = id => typeof id === "string" && /^[a-f0-9]{1,128}$/.test(id);
  const now = () => root.performance ? root.performance.now() : Date.now();
  const stopped = packet => packet.status === "paused" || packet.status === "ended" || packet.status === "closed";
  const dimensions = n => Number.isInteger(n) && n > 0 && n <= 4096;

  function pngBlob(encoded) {
    if (typeof encoded !== "string" || encoded.length === 0 || encoded.length > 2800000) throw new Error("inline pose is the wrong size");
    const binary = atob(encoded);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
    return new root.Blob([bytes], {type: "image/png"});
  }

  function galleryFileUrl(base, relative) {
    if (typeof relative !== "string" || relative === "" || relative.startsWith("/") || relative.includes("\\") || relative.includes("?") || relative.includes("#")) throw new Error("Gallery path is not relative");
    const parts = relative.split("/");
    if (parts.some(part => part === "" || part === "." || part === "..")) throw new Error("Gallery path is not relative");
    const prefix = typeof base === "string" ? base.replace(/\/$/, "") : "";
    return prefix + "/api/visual-gallery/files/" + parts.map(part => encodeURIComponent(part)).join("/");
  }

  class ArenaSceneRenderer {
    constructor(canvas, options = {}) {
      if (!canvas || typeof options.textureUrl !== "function") throw new Error("ArenaSceneRenderer needs a canvas and textureUrl callback");
      this.canvas = canvas; this.options = options;
      this.maxFrames = clamp(number(options.maxPendingFrames, 4), 2, 8); this.maxBytes = Math.max(4 * 1024 * 1024, number(options.maxTextureBytes, 192 * 1024 * 1024));
      this.maxTextures = clamp(number(options.maxTextures, 4096), 32, 8192); this.loadConcurrency = clamp(number(options.loadConcurrency, 4), 1, 8);
      // Interpolation delay is wall-clock milliseconds, including fast playback.
      this.delay = clamp(number(options.interpolationDelay, 50), 0, 250); this.textures = new Map(); this.inflight = new Map(); this.inlinePng = new Map(); this.descriptors = new Map(); this.pending = []; this.timeline = [];
      this.generation = 0; this.matchId = undefined; this.lastSequence = -Infinity; this.lastTick = -Infinity; this.destroyed = false; this.ready = false; this.bytes = 0; this.frameHandle = 0; this.exactToken = 0; this.exactWait = null;
      this.metrics = {backend: "", fps: 0, render_ms: 0, draws: 0, texture_bytes: 0, texture_count: 0, pending_frames: 0, missing: 0, frames: 0, dropped: 0, requests: 0, download_bytes: 0, evicted_textures: 0, expired_textures: 0, error: null};
      this.samples = [];
      const gl = options.forceCanvas ? null : canvas.getContext("webgl2", {alpha: options.alpha !== false, antialias: false, premultipliedAlpha: false, preserveDrawingBuffer: !!options.preserveDrawingBuffer, powerPreference: "high-performance"});
      this.painter = gl ? new GLPainter(canvas, gl) : new SoftwarePainter(canvas); this.metrics.backend = gl ? "webgl2" : "canvas2d";
      this.contextLost = event => {event.preventDefault(); this.fail(new Error("The graphics context was lost; reload the scene or choose the original stream")); this.halted = true;};
      canvas.addEventListener("webglcontextlost", this.contextLost);
    }
    get stats() {
      const missing = new Set(); for (const frame of this.pending) for (const id of frame.ids) if (!this.textures.has(id)) missing.add(id);
      return Object.assign({}, this.metrics, {texture_bytes: this.bytes, texture_count: this.textures.size, pending_frames: this.pending.length, missing: missing.size, loading_textures: this.inflight.size});
    }
    fail(error) {
      if (this.destroyed) return;
      const message = error instanceof Error ? error.message : String(error); this.metrics.error = message;
      if (this.options.onError) this.options.onError(error instanceof Error ? error : new Error(message));
    }
    abandonExact(reason) {
      this.exactToken++;
      this.pending = this.pending.filter(frame => !frame.exact);
      this.timeline = this.timeline.filter(frame => !frame.exact);
      if (!this.exactWait) return;
      const wait = this.exactWait; this.exactWait = null; wait.reject(reason);
    }
    reset(matchId) {
      this.abandonExact(new DOMException("Reset", "AbortError"));
      this.generation++; for (const request of this.inflight.values()) request.abort(); this.inflight.clear(); this.inlinePng.clear();
      // Pose ids are the pixels. A new match keeps textures already on the GPU
      // instead of downloading the same walk and spell frames again.
      this.pending = []; this.timeline = [];
      this.matchId = matchId; this.lastSequence = -Infinity; this.lastTick = -Infinity; this.ready = false; this.halted = !!(this.painter.gl && this.painter.gl.isContextLost()); this.metrics.error = null;
    }
    push(packet) {
      if (this.destroyed || this.halted) return;
      try {
        if (!packet || packet.v !== 1 || !dimensions(packet.width) || !dimensions(packet.height) || !finite(packet.sequence) || !finite(packet.tick)) throw new Error("Unsupported scene packet");
        this.abandonExact(new DOMException("Superseded", "AbortError"));
        const reset = this.matchId !== packet.match_id || packet.sequence > this.lastSequence && packet.tick < this.lastTick;
        if (reset) this.reset(packet.match_id);
        if (packet.sequence <= this.lastSequence) {this.metrics.dropped++; return;}
        const ids = this.acceptTextures(packet);
        this.lastSequence = packet.sequence; this.lastTick = packet.tick;
        const nativeTime = number(packet.display_tick, packet.tick) * 50;
        this.pending.push({packet, ids, received: now(), nativeTime});
        // Preserve an initial completeable pose while shedding intermediate arrivals.
        while (this.pending.length > this.maxFrames) {this.pending.splice(this.ready ? 0 : 1, 1); this.metrics.dropped++;}
        this.promote(); this.load(); this.schedule();
      } catch (error) { this.fail(error); }
    }
    acceptTextures(packet) {
      const ids = collectTextures(packet.draws); if (ids.size > 1024) throw new Error("Scene uses too many textures");
      for (const descriptor of packet.textures || []) {
        if (!textureId(descriptor.id) || !dimensions(descriptor.width) || !dimensions(descriptor.height)) throw new Error("Invalid scene texture descriptor");
        if (descriptor.width * descriptor.height * 4 > this.maxBytes) throw new Error("Scene texture exceeds the cache budget");
        this.descriptors.set(descriptor.id, descriptor);
      }
      const blobs = packet.blobs;
      if (packet.blobs) delete packet.blobs;
      if (Array.isArray(blobs)) {
        const limit = Math.min(blobs.length, 1024);
        for (let i = 0; i < limit; i++) {
          const blob = blobs[i];
          if (!blob || !textureId(blob.id) || typeof blob.png !== "string" || blob.png.length === 0 || blob.png.length > 2800000) continue;
          if (!this.textures.has(blob.id)) this.inlinePng.set(blob.id, blob.png);
        }
      }
      while (this.descriptors.size > 8192) this.descriptors.delete(this.descriptors.keys().next().value);
      return ids;
    }
    // Paint one authored pose. Fixture scrubbing may move backward in time or
    // across match ids; content-addressed textures stay cached either way.
    displayExact(packet) {
      if (this.destroyed || this.halted) return Promise.reject(new Error(this.metrics.error || "Scene renderer is stopped"));
      this.abandonExact(new DOMException("Superseded", "AbortError"));
      const token = this.exactToken;
      let resolve, reject;
      const promise = new Promise((ok, fail) => {resolve = ok; reject = fail;});
      this.exactWait = {token, resolve, reject};
      try {
        if (!packet || packet.v !== 1 || !dimensions(packet.width) || !dimensions(packet.height) || !finite(packet.sequence) || !finite(packet.tick)) throw new Error("Unsupported scene packet");
        const ids = this.acceptTextures(packet);
        this.matchId = packet.match_id;
        this.pending = [{packet, ids, received: now(), nativeTime: number(packet.display_tick, packet.tick) * 50, exact: true, token}];
        this.promote(); this.load(); this.schedule();
      } catch (error) {
        this.exactWait = null; reject(error instanceof Error ? error : new Error(String(error))); this.fail(error);
      }
      return promise;
    }
    diagnostics() {
      const latest = this.timeline[this.timeline.length - 1];
      return Object.assign({}, this.stats, {exact: !!(latest && latest.exact), match_id: this.matchId ?? null, sequence: latest ? latest.packet.sequence : null, tick: latest ? latest.packet.tick : null, cached_textures: [...this.textures.keys()]});
    }
    snapshot() {
      const width = this.canvas.width, height = this.canvas.height;
      if (!width || !height) return null;
      if (this.painter.output && this.painter.output.data) return {width, height, data: new Uint8ClampedArray(this.painter.output.data)};
      const gl = this.painter.gl;
      if (!gl || gl.isContextLost()) return null;
      const raw = new Uint8Array(width * height * 4);
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
      gl.readPixels(0, 0, width, height, gl.RGBA, gl.UNSIGNED_BYTE, raw);
      const data = new Uint8ClampedArray(width * height * 4), row = width * 4;
      for (let y = 0; y < height; y++) data.set(raw.subarray((height - 1 - y) * row, (height - y) * row), y * row);
      return {width, height, data};
    }
    protectedIds() {
      const result = new Set(); for (const frame of [...this.pending, ...this.timeline]) for (const id of frame.ids) result.add(id);
      return result;
    }
    trim(extraBytes = 0) {
      const protectedIds = this.protectedIds();
      const candidates = [...this.textures.entries()].filter(([id]) => !protectedIds.has(id)).sort((a, b) => a[1].used - b[1].used);
      for (const [id, entry] of candidates) {
        if (this.bytes + extraBytes <= this.maxBytes && this.textures.size < this.maxTextures) break;
        this.textures.delete(id); this.painter.release(entry.resource); this.bytes -= entry.bytes; this.metrics.evicted_textures++;
      }
      return this.bytes + extraBytes <= this.maxBytes && this.textures.size < this.maxTextures;
    }
    load() {
      if (this.destroyed || this.halted) return;
      const wanted = new Set(); for (const frame of this.pending) for (const id of frame.ids) if (!this.textures.has(id) && !this.inflight.has(id)) wanted.add(id);
      for (const id of wanted) {
        if (this.inflight.size >= this.loadConcurrency) break;
        const controller = new AbortController(), generation = this.generation; this.inflight.set(id, controller); this.metrics.requests++;
        const timeout = root.setTimeout(() => controller.abort(new Error("Original texture load timed out: " + id)), number(this.options.textureTimeoutMs, 20000));
        this.loadTexture(id, controller.signal).then(({image, bytes}) => {
          if (this.destroyed || generation !== this.generation) {if (image.close) image.close(); return;}
          const size = image.width * image.height * 4;
          if (!dimensions(image.width) || !dimensions(image.height) || !this.trim(size)) {if (image.close) image.close(); throw new Error("Visible scene exceeds the texture cache budget");}
          const descriptor = this.descriptors.get(id);
          if (descriptor && (descriptor.width !== image.width || descriptor.height !== image.height)) {if (image.close) image.close(); throw new Error("Texture dimensions disagree with the scene: " + id);}
          try {this.textures.set(id, {resource: this.painter.upload(image), bytes: size, used: now()}); this.bytes += size; this.metrics.download_bytes += bytes;}
          finally {if (image.close) image.close();}
        }).catch(error => {
          if (this.destroyed || generation !== this.generation || error.name === "AbortError") return;
          if (error.code === "ARENA_TEXTURE_EXPIRED") {this.expireTexture(id); return;}
          // A missing texture is never drawn as a blank object. Stop retries until an explicit reset.
          this.halted = true; this.fail(error);
          if (this.exactWait) {const wait = this.exactWait; this.exactWait = null; wait.reject(error);}
        }).finally(() => {
          root.clearTimeout(timeout);
          if (generation !== this.generation || this.destroyed) return;
          this.inflight.delete(id); this.promote(); this.load(); this.schedule();
        });
      }
    }
    expireTexture(id) {
      // Slow clients may request a pose after the presenter's bounded disk LRU
      // retired it. Never paint a partial pose or permanently poison this ID:
      // a later native packet can reintroduce it after re-encoding its original.
      const before = this.pending.length;
      const droppedExact = this.pending.some(frame => frame.exact && frame.ids.has(id));
      this.pending = this.pending.filter(frame => !frame.ids.has(id));
      this.metrics.dropped += before - this.pending.length;
      if (droppedExact && this.exactWait) {
        const wait = this.exactWait; this.exactWait = null;
        const error = new Error("Original pose texture expired: " + id); error.code = "ARENA_TEXTURE_EXPIRED"; error.textureId = id;
        wait.reject(error);
      }
      this.metrics.expired_textures++;
      const needed = this.protectedIds();
      for (const [other, request] of this.inflight) if (other !== id && !needed.has(other)) request.abort();
      this.trim();
    }
    async loadTexture(id, signal) {
      let blob;
      const inline = this.inlinePng.get(id);
      if (inline) {
        this.inlinePng.delete(id);
        try { blob = pngBlob(inline); }
        catch { blob = null; }
      }
      if (!blob) {
        const response = await root.fetch(this.options.textureUrl(id), {signal, credentials: "same-origin", cache: "force-cache"});
        if (response.status === 410) {
          const error = new Error("Original pose texture expired: " + id);
          error.code = "ARENA_TEXTURE_EXPIRED"; error.textureId = id; throw error;
        }
        if (!response.ok) throw new Error("Original texture could not be loaded (" + response.status + "): " + id);
        blob = await response.blob();
      }
      if (signal.aborted) throw signal.reason || new DOMException("Cancelled", "AbortError");
      if (root.createImageBitmap) {
        const image = await root.createImageBitmap(blob, {premultiplyAlpha: "none", colorSpaceConversion: "none"});
        if (signal.aborted) {image.close(); throw signal.reason || new DOMException("Cancelled", "AbortError");}
        return {image, bytes: blob.size};
      }
      const url = root.URL.createObjectURL(blob);
      try {
        const image = new root.Image(); image.src = url; await image.decode();
        if (signal.aborted) throw signal.reason || new DOMException("Cancelled", "AbortError");
        return {image, bytes: blob.size};
      } finally {root.URL.revokeObjectURL(url);}
    }
    promote() {
      // Decode completion order is unrelated to native pose order. Only consume
      // a complete prefix, retaining each coherent pose for interpolation.
      // push() bounds the queue and may shed an old blocked pose once visible.
      let promoted = false;
      while (this.pending.length && [...this.pending[0].ids].every(id => this.textures.has(id))) {
        const frame = this.pending.shift();
        frame.readyAt = now();
        if (frame.exact) {
          if (frame.token !== this.exactToken) continue;
          this.timeline = [frame];
        } else {
          const previous = this.timeline[this.timeline.length - 1];
          if (previous && (previous.exact || frame.nativeTime < previous.nativeTime)) this.timeline = [];
          this.timeline.push(frame); while (this.timeline.length > 3) this.timeline.shift();
        }
        for (const id of frame.ids) this.textures.get(id).used = now();
        promoted = true;
      }
      if (promoted) this.trim();
    }
    schedule() {
      if (!this.frameHandle && !this.destroyed && !this.halted && this.timeline.length) this.frameHandle = root.requestAnimationFrame(time => {this.frameHandle = 0; this.render(time);});
    }
    render(time) {
      if (this.destroyed || this.halted || !this.timeline.length) return;
      try {
        const latest = this.timeline[this.timeline.length - 1];
        const readyAt = latest.readyAt ?? latest.received;
        let packet = latest.packet, draws = packet.draws;
        if (latest.exact) {packet = latest.packet; draws = packet.draws;}
        else if (!stopped(packet) && this.timeline.length > 1) {
          const speed = clamp(number(packet.playback_speed, 1), .1, 16);
          const target = latest.nativeTime + (Math.max(0, time - readyAt) - this.delay) * speed;
          let older = this.timeline[0], newer = latest;
          for (let i = 1; i < this.timeline.length; i++) if (this.timeline[i].nativeTime >= target) {newer = this.timeline[i]; older = this.timeline[i - 1]; break;}
          const distance = newer.nativeTime - older.nativeTime;
          const amount = distance > 0 ? clamp((target - older.nativeTime) / distance, 0, 1) : 1;
          if (!stopped(older.packet) && distance <= 500 && older.packet.match_id === newer.packet.match_id) {
            packet = amount < 1 ? older.packet : newer.packet; draws = interpolateDraws(older.packet.draws, newer.packet.draws, amount);
          }
        }
        const started = now(); this.painter.paint(packet, draws, this.textures);
        this.metrics.render_ms = Math.round((now() - started) * 100) / 100; this.metrics.draws = draws.length; this.metrics.frames++;
        this.samples.push(time); while (this.samples.length && this.samples[0] < time - 1000) this.samples.shift();
        this.metrics.fps = this.samples.length > 1 ? Math.round((this.samples.length - 1) * 1000 / Math.max(1, time - this.samples[0])) : 0;
        if (!this.ready) {this.ready = true; if (this.options.onReady) this.options.onReady(latest.packet);}
        if (latest.exact && this.exactWait && this.exactWait.token === latest.token) {
          const wait = this.exactWait; this.exactWait = null;
          wait.resolve({sequence: packet.sequence, tick: packet.tick, width: packet.width, height: packet.height, backend: this.metrics.backend, exact: true});
        }
        const moving = !latest.exact && !stopped(latest.packet) && time - readyAt < this.delay + 250;
        if (moving) this.schedule();
      } catch (error) {
        this.halted = true; this.fail(error);
        if (this.exactWait) {const wait = this.exactWait; this.exactWait = null; wait.reject(error instanceof Error ? error : new Error(String(error)));}
      }
    }
    destroy() {
      if (this.destroyed) return;
      if (this.frameHandle) root.cancelAnimationFrame(this.frameHandle); this.frameHandle = 0;
      for (const entry of this.textures.values()) this.painter.release(entry.resource);
      this.textures.clear(); this.descriptors.clear(); this.bytes = 0;
      this.reset(); this.destroyed = true; this.canvas.removeEventListener("webglcontextlost", this.contextLost); this.painter.destroy(); this.samples = [];
    }
  }
  root.ArenaSceneRenderer = ArenaSceneRenderer;
  root.arenaGalleryFileUrl = galleryFileUrl;
  root.SoftwarePainter = SoftwarePainter;
  root.GLPainter = GLPainter;
  root.interpolateDraws = interpolateDraws;
  root.collectTextures = collectTextures;
  if (typeof module !== "undefined" && module.exports) module.exports = {ArenaSceneRenderer, SoftwarePainter, GLPainter, interpolateDraws, collectTextures, galleryFileUrl, SceneCompositor: shared.SceneCompositor};
})(typeof window !== "undefined" ? window : globalThis);
