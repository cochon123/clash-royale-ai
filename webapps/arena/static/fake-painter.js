/* C2-shaped painter that replays recorded packets. Pixels come from RGBA or fixture PNGs. */
(function (root) {
  "use strict";
  const clocks = typeof module !== "undefined" && module.exports ? require("./jitter-clock.js") : root;
  const {SceneDisplayClock} = clocks;

  function readU32(bytes, offset) {
    return (bytes[offset] << 24 | bytes[offset + 1] << 16 | bytes[offset + 2] << 8 | bytes[offset + 3]) >>> 0;
  }
  function concat(parts) {
    const size = parts.reduce((sum, part) => sum + part.length, 0);
    const out = new Uint8Array(size);
    let offset = 0;
    for (const part of parts) { out.set(part, offset); offset += part.length; }
    return out;
  }
  function paeth(a, b, c) {
    const p = a + b - c, pa = Math.abs(p - a), pb = Math.abs(p - b), pc = Math.abs(p - c);
    if (pa <= pb && pa <= pc) return a;
    if (pb <= pc) return b;
    return c;
  }
  function inflateBytes(bytes) {
    if (typeof require === "function") {
      try { return require("node:zlib").inflateSync(bytes); }
      catch (error) { if (error.code !== "MODULE_NOT_FOUND") throw error; }
    }
    throw new Error("PNG inflate is unavailable in this runtime");
  }
  function decodePng(bytes) {
    const data = bytes instanceof Uint8Array ? bytes : new Uint8Array(bytes);
    const signature = [137, 80, 78, 71, 13, 10, 26, 10];
    for (let i = 0; i < signature.length; i++) if (data[i] !== signature[i]) throw new Error("not a png");
    let offset = 8, width = 0, height = 0, bitDepth = 0, colorType = 0, interlace = 0;
    const idat = [];
    while (offset + 12 <= data.length) {
      const length = readU32(data, offset);
      const type = String.fromCharCode(data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7]);
      const chunk = data.subarray(offset + 8, offset + 8 + length);
      offset += 12 + length;
      if (type === "IHDR") {
        width = readU32(chunk, 0); height = readU32(chunk, 4);
        bitDepth = chunk[8]; colorType = chunk[9]; interlace = chunk[12];
      } else if (type === "IDAT") idat.push(chunk);
      else if (type === "IEND") break;
    }
    if (!width || !height || bitDepth !== 8 || interlace !== 0 || (colorType !== 6 && colorType !== 2)) throw new Error("unsupported png");
    const channels = colorType === 6 ? 4 : 3;
    const raw = inflateBytes(concat(idat));
    const stride = width * channels;
    const rgba = new Uint8Array(width * height * 4);
    let src = 0;
    let previous = new Uint8Array(stride);
    for (let y = 0; y < height; y++) {
      const filter = raw[src++];
      const row = raw.subarray(src, src + stride);
      src += stride;
      const recon = new Uint8Array(stride);
      for (let i = 0; i < stride; i++) {
        const left = i >= channels ? recon[i - channels] : 0;
        const up = previous[i];
        const ul = i >= channels ? previous[i - channels] : 0;
        let value = row[i];
        if (filter === 1) value += left;
        else if (filter === 2) value += up;
        else if (filter === 3) value += (left + up) >> 1;
        else if (filter === 4) value += paeth(left, up, ul);
        else if (filter !== 0) throw new Error("bad png filter");
        recon[i] = value & 255;
      }
      for (let x = 0; x < width; x++) {
        const di = (y * width + x) * 4, si = x * channels;
        rgba[di] = recon[si]; rgba[di + 1] = recon[si + 1]; rgba[di + 2] = recon[si + 2];
        rgba[di + 3] = channels === 4 ? recon[si + 3] : 255;
      }
      previous = recon;
    }
    return {width, height, rgba};
  }
  function asBytes(value) {
    if (value instanceof Uint8Array) return value;
    if (typeof value === "string") {
      const binary = atob(value);
      const bytes = new Uint8Array(binary.length);
      for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
      return bytes;
    }
    return new Uint8Array(value);
  }
  function decodeTexture(record) {
    if (!record || typeof record !== "object") throw new Error("recorded texture is missing");
    if (record.rgba || record.rgba_b64) {
      const rgba = asBytes(record.rgba || record.rgba_b64);
      return {width: record.width, height: record.height, rgba};
    }
    if (record.png || record.png_b64) return decodePng(asBytes(record.png || record.png_b64));
    throw new Error("recorded texture has no pixels");
  }
  async function decodeTextureAsync(record) {
    if (record && (record.png || record.png_b64) && typeof createImageBitmap === "function" && typeof OffscreenCanvas !== "undefined") {
      const bytes = asBytes(record.png || record.png_b64);
      const bitmap = await createImageBitmap(new Blob([bytes], {type: "image/png"}), {premultiplyAlpha: "none", colorSpaceConversion: "none"});
      const width = bitmap.width;
      const height = bitmap.height;
      const surface = new OffscreenCanvas(width, height);
      const ctx = surface.getContext("2d", {willReadFrequently: true});
      ctx.drawImage(bitmap, 0, 0);
      const image = ctx.getImageData(0, 0, width, height);
      if (bitmap.close) bitmap.close();
      return {width, height, rgba: new Uint8Array(image.data)};
    }
    return decodeTexture(record);
  }
  function drawTextures(draws, out = []) {
    for (const draw of draws || []) {
      if (!draw || typeof draw !== "object") continue;
      if (draw.kind === "sprite" && draw.tex) out.push(draw.tex);
      else if (draw.kind === "shadow") drawTextures(draw.draws, out);
    }
    return out;
  }

  class FakePainter {
    constructor(recording) {
      this.width = recording.width || 540;
      this.height = recording.height || 900;
      this.frames = (recording.frames || []).slice().sort((a, b) => a.tick - b.tick);
      this.byTick = new Map(this.frames.map((frame) => [frame.tick, frame]));
      this.textures = recording.textures instanceof Map ? recording.textures : new Map(Object.entries(recording.textures || {}));
      this.clock = new SceneDisplayClock();
      this.observed = [];
      this.emitted = new Set();
      this.lastPacket = null;
      this.error = "";
      this.initialized = false;
      this.wasmBytes = 0;
    }
    vfsPut(_path, data) { this.wasmBytes += data ? data.length : 0; return 0; }
    init(_cardsJson, width, height) {
      if (width) this.width = width;
      if (height) this.height = height;
      this.initialized = true;
      return 0;
    }
    setLagTicks(value) { this.clock.setLag(value); }
    displayTick() { return this.clock.display; }
    observe(snapshotJson) {
      let snapshot;
      try { snapshot = JSON.parse(snapshotJson); }
      catch { this.error = "snapshot is not json"; return 1; }
      const tick = Number(snapshot.tick);
      if (!Number.isFinite(tick)) { this.error = "snapshot has no tick"; return 1; }
      const previous = this.observed[this.observed.length - 1];
      if (previous && tick < previous.tick) { this.error = "ticks arrived out of order"; return 1; }
      this.observed.push(snapshot);
      return 0;
    }
    step(dt) {
      if (!this.observed.length) return 0;
      const oldest = this.observed[0].tick;
      const latestSnap = this.observed[this.observed.length - 1];
      const latest = latestSnap.tick;
      const speed = Number(latestSnap.playback_speed || 1);
      const status = latestSnap.runtime_status || "running";
      const flow = status === "paused" ? "paused" : status === "ended" || status === "closed" ? "finished" : "running";
      const before = this.clock.display;
      const display = this.clock.step(oldest, latest, speed, flow, dt);
      if (before != null && Math.abs(display - before) < 1e-9 && this.clock.caughtUp(oldest, latest, flow)) return 0;
      this.lastPacket = this.packetAt(display);
      return this.lastPacket ? 1 : 0;
    }
    packetAt(display) {
      const tick = Math.floor(display);
      let frame = this.byTick.get(tick);
      if (!frame) {
        for (let i = this.frames.length - 1; i >= 0; i--) if (this.frames[i].tick <= tick) { frame = this.frames[i]; break; }
        frame = frame || this.frames[0];
      }
      if (!frame) return null;
      const packet = JSON.parse(JSON.stringify(frame.packet));
      packet.display_tick = display;
      packet.tick = frame.tick;
      packet.width = packet.width || this.width;
      packet.height = packet.height || this.height;
      const fresh = [];
      for (const id of drawTextures(packet.draws)) {
        if (this.emitted.has(id)) continue;
        const tex = this.textures.get(id);
        if (!tex) continue;
        fresh.push({id, width: tex.width, height: tex.height});
        this.emitted.add(id);
      }
      packet.textures = fresh;
      return packet;
    }
    result() { return JSON.stringify(this.lastPacket || {}); }
    texture(id) {
      const tex = this.textures.get(id);
      if (!tex) return null;
      return {width: tex.width, height: tex.height, rgba: tex.rgba.slice()};
    }
    textureRelease() {}
    reset() {
      this.observed = [];
      this.emitted.clear();
      this.clock.reset();
      this.lastPacket = null;
      this.error = "";
    }
    statsJson() {
      return JSON.stringify({draw_ms_p50: 0, draw_ms_p95: 0, wasm_bytes: this.wasmBytes, wasm_mb: this.wasmBytes / (1024 * 1024), observe_backlog: 0});
    }
    lastError() { return this.error; }
  }

  async function createFakePainter(recording) {
    const textures = new Map();
    for (const [id, record] of Object.entries(recording.textures || {})) textures.set(id, await decodeTextureAsync(record));
    return new FakePainter({...recording, textures});
  }

  root.decodePng = decodePng;
  root.FakePainter = FakePainter;
  root.createFakePainter = createFakePainter;
  if (typeof module !== "undefined" && module.exports) module.exports = {decodePng, FakePainter, createFakePainter};
})(typeof globalThis !== "undefined" ? globalThis : this);
