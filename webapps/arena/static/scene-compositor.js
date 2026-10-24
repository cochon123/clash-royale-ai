/* Display-list compositor shared by the painter worker and the main thread.
   Textures come from memory (textureSource), never from pose URLs. */
(function (root) {
  "use strict";
  const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));
  const number = (v, otherwise = 0) => Number.isFinite(v) ? v : otherwise;
  const finite = v => typeof v === "number" && Number.isFinite(v);
  const validAnchor = anchor => Array.isArray(anchor) && anchor.length === 2 && anchor.every(finite);
  const textureId = id => typeof id === "string" && /^[a-f0-9]{1,128}$/.test(id);
  const now = () => root.performance ? root.performance.now() : Date.now();
  const stopped = packet => packet.status === "paused" || packet.status === "ended" || packet.status === "closed";
  const dimensions = n => Number.isInteger(n) && n > 0 && n <= 4096;
  const BLENDS = new Set(["normal", "add", "premultiplied-add", "premultiplied-screen"]);

  function collectTextures(draws, target = new Set(), depth = 0) {
    if (!Array.isArray(draws) || depth > 4 || draws.length > 8192) throw new Error("Invalid scene draw list");
    for (const draw of draws) {
      if (!draw || typeof draw !== "object") throw new Error("Invalid scene command");
      if (draw.kind === "sprite") {
        if (!textureId(draw.tex) || !Array.isArray(draw.m) || draw.m.length !== 6 || !draw.m.every(finite) ||
            (draw.blend && !BLENDS.has(draw.blend)) ||
            (draw.anchor !== undefined && !validAnchor(draw.anchor))) throw new Error("Invalid scene sprite");
        target.add(draw.tex);
      } else if (draw.kind === "shadow") collectTextures(draw.draws, target, depth + 1);
      else if (draw.kind === "rect") {
        if (![draw.x, draw.y, draw.w, draw.h].every(finite) || !Array.isArray(draw.color) || draw.color.length !== 4 || !draw.color.every(finite)) throw new Error("Invalid scene rectangle");
      } else if (draw.kind === "ellipse") {
        if (![draw.cx, draw.cy, draw.rx, draw.ry, draw.alpha].every(finite)) throw new Error("Invalid scene ellipse");
      } else throw new Error("Unsupported scene command: " + draw.kind);
    }
    return target;
  }

  function interpolateDraws(older, newer, amount) {
    // Use the older pose/order until its native tick ends. Only stable IDs move.
    if (amount >= 1) return newer;
    if (amount <= 0) return older;
    const targets = new Map(newer.filter(d => d.id != null).map(d => [d.id, d]));
    return older.map(draw => {
      // HUD/root IDs identify painter draw order, not semantic moving objects.
      // Affordability shading can insert a HUD command and renumber later IDs.
      if (typeof draw.id === "string" && /^(hud-|root-)/.test(draw.id)) return draw;
      const next = draw.id != null && targets.get(draw.id);
      if (!next || draw.kind !== next.kind || draw.blend !== next.blend) return draw;
      const result = Object.assign({}, draw);
      const lerp = (a, b) => a + (b - a) * amount;
      if (draw.kind === "sprite") {
        if (draw.tex === next.tex) result.m = draw.m.map((v, i) => lerp(v, next.m[i]));
        else {
          // anchor is the native screen point [screenX, screenY]. It is present
          // only when the presenter emitted it. Never synthesize one.
          // A new pose has its own crop and scale. Keep the older affine,
          // including that crop's translation, and slide only the anchor.
          if (!validAnchor(draw.anchor) || !validAnchor(next.anchor)) return draw;
          result.m = draw.m.slice();
          result.m[4] += (next.anchor[0] - draw.anchor[0]) * amount;
          result.m[5] += (next.anchor[1] - draw.anchor[1]) * amount;
        }
        if (validAnchor(draw.anchor) && validAnchor(next.anchor)) result.anchor = draw.anchor.map((v, i) => lerp(v, next.anchor[i]));
      }
      else if (draw.kind === "rect") for (const key of ["x", "y", "w", "h"]) result[key] = lerp(draw[key], next[key]);
      else if (draw.kind === "ellipse") for (const key of ["cx", "cy", "rx", "ry"]) result[key] = lerp(draw[key], next[key]);
      else if (draw.kind === "shadow") result.draws = interpolateDraws(draw.draws, next.draws, amount);
      if (finite(draw.alpha) && finite(next.alpha)) result.alpha = lerp(draw.alpha, next.alpha);
      return result;
    });
  }

  // A faithful software fallback also makes blend/shadow behavior testable without a GPU.
  class SoftwarePainter {
    constructor(canvas) { this.canvas = canvas; this.ctx = canvas.getContext("2d", {alpha: true}); if (!this.ctx) throw new Error("Canvas rendering is unavailable"); }
    upload(image) {
      if (image && image.pixels && dimensions(image.width) && dimensions(image.height)) {
        const pixels = image.pixels instanceof Uint8ClampedArray ? image.pixels : new Uint8ClampedArray(image.pixels);
        return {width: image.width, height: image.height, pixels};
      }
      const surface = root.document.createElement("canvas"); surface.width = image.width; surface.height = image.height;
      const ctx = surface.getContext("2d", {willReadFrequently: true}); ctx.drawImage(image, 0, 0);
      return {width: image.width, height: image.height, pixels: ctx.getImageData(0, 0, image.width, image.height).data};
    }
    release() {}
    resize(w, h) {
      if (this.width === w && this.height === h) return;
      this.width = this.canvas.width = w; this.height = this.canvas.height = h;
      this.output = this.ctx.createImageData(w, h); this.mask = new Uint8Array(w * h);
    }
    clear(color) {
      const out = this.output.data;
      for (let i = 0; i < out.length; i += 4) {out[i] = color[0]; out[i + 1] = color[1]; out[i + 2] = color[2]; out[i + 3] = color[3];}
    }
    sprite(draw, tex, shadow = false) {
      const [a, b, c, d, tx, ty] = draw.m, det = a * d - b * c;
      const alpha = clamp(number(draw.alpha, 1), 0, 1);
      if (Math.abs(det) < 1e-9 || alpha <= 0.003) return;
      const sw = tex.width, sh = tex.height, pixels = tex.pixels, out = this.output.data, width = this.width;
      if (draw.replace && !shadow && a === 1 && b === 0 && c === 0 && d === 1 && Number.isInteger(tx) && Number.isInteger(ty) && alpha === 1 && !(draw.add || []).some(v => v)) {
        // The static arena and HUD panels are literal RGBA copies, not software-filtered sprites.
        const left = Math.max(0, tx), right = Math.min(width, tx + sw), top = Math.max(0, ty), bottom = Math.min(this.height, ty + sh);
        if (right > left) for (let y = top; y < bottom; y++) {
          const start = ((y - ty) * sw + left - tx) * 4;
          out.set(pixels.subarray(start, start + (right - left) * 4), (y * width + left) * 4);
        }
        return;
      }
      if (!shadow && !draw.replace && a === 1 && b === 0 && c === 0 && d === 1 && Number.isInteger(tx) && Number.isInteger(ty)) {
        const left = Math.max(0, tx), right = Math.min(width, tx + sw), top = Math.max(0, ty), bottom = Math.min(this.height, ty + sh);
        const opacity = Math.floor(alpha * 256), blend = draw.blend || "normal", add = draw.add || [0, 0, 0];
        for (let y = top; y < bottom; y++) for (let x = left; x < right; x++) {
          const si = ((y - ty) * sw + x - tx) * 4, di = (y * width + x) * 4, a = Math.floor(pixels[si + 3] * opacity / 256);
          if (!a) continue;
          for (let ch = 0; ch < 3; ch++) {
            const s = clamp(pixels[si + ch] + number(add[ch]), 0, 255), d = out[di + ch];
            out[di + ch] = Math.floor(blend === "normal" ? (s * a + d * (255 - a)) / 255 : blend === "add" ? Math.min(255, d + Math.floor(s * a / 255)) : blend === "premultiplied-screen" ? 255 - (255 - d) * (255 - Math.floor(s * opacity / 256)) / 255 : Math.min(255, d + Math.floor(s * opacity / 256)));
          }
          if (blend === "normal") out[di + 3] = Math.floor(a + out[di + 3] * (255 - a) / 255);
        }
        return;
      }
      const corners = [[0, 0], [sw, 0], [0, sh], [sw, sh]].map(([x, y]) => [a * x + c * y + tx, b * x + d * y + ty]);
      const x0 = Math.max(0, Math.floor(Math.min(...corners.map(p => p[0]))) - 1), y0 = Math.max(0, Math.floor(Math.min(...corners.map(p => p[1]))) - 1);
      const x1 = Math.min(width, Math.ceil(Math.max(...corners.map(p => p[0]))) + 1), y1 = Math.min(this.height, Math.ceil(Math.max(...corners.map(p => p[1]))) + 1);
      const blend = draw.blend || "normal", add = draw.add || [0, 0, 0], premult = blend.startsWith("premultiplied");
      for (let y = y0; y < y1; y++) for (let x = x0; x < x1; x++) {
        const dx = x + .5 - tx, dy = y + .5 - ty;
        const sx = (d * dx - c * dy) / det, sy = (-b * dx + a * dy) / det;
        const di = (y * width + x) * 4;
        if (shadow) {
          const ix = Math.floor(sx), iy = Math.floor(sy);
          if (ix < 0 || iy < 0 || ix >= sw || iy >= sh) continue;
          const value = Math.floor(pixels[(iy * sw + ix) * 4 + 3] * Math.floor(alpha * 255) / 255);
          const mi = y * width + x; this.mask[mi] = Math.max(this.mask[mi], value); continue;
        }
        if (draw.replace && (sx < 0 || sy < 0 || sx >= sw || sy >= sh)) continue;
        const fx = Math.floor(sx - .5), fy = Math.floor(sy - .5), wx = sx - .5 - fx, wy = sy - .5 - fy;
        const points = [[fx, fy, (1 - wx) * (1 - wy)], [fx + 1, fy, wx * (1 - wy)], [fx, fy + 1, (1 - wx) * wy], [fx + 1, fy + 1, wx * wy]];
        let sa = 0; const rgb = [0, 0, 0];
        for (const [px, py, weight] of points) {
          if (px < 0 || py < 0 || px >= sw || py >= sh) continue;
          const si = (py * sw + px) * 4, sampleAlpha = pixels[si + 3] / 255;
          sa += sampleAlpha * weight;
          for (let ch = 0; ch < 3; ch++) rgb[ch] += clamp(pixels[si + ch] + number(add[ch]), 0, 255) * weight * (premult || draw.replace ? 1 : sampleAlpha);
        }
        if (sa * 255 <= .5 && !draw.replace) continue;
        sa *= alpha;
        for (let ch = 0; ch < 3; ch++) {
          const source = rgb[ch] * alpha, target = out[di + ch];
          if (draw.replace) {out[di + ch] = Math.floor(source); continue;}
          out[di + ch] = Math.floor(blend === "normal" ? source + target * (1 - sa) : blend === "premultiplied-screen" ? 255 - (255 - target) * (255 - Math.min(255, source)) / 255 : Math.min(255, source + target));
        }
        if (draw.replace) out[di + 3] = Math.floor(sa * 255);
        else if (blend === "normal") out[di + 3] = Math.floor(sa * 255 + out[di + 3] * (1 - sa));
      }
    }
    ellipse(draw) {
      if (draw.rx < .5 || draw.ry < .5) return;
      const x0 = Math.max(0, Math.floor(draw.cx - draw.rx)), y0 = Math.max(0, Math.floor(draw.cy - draw.ry));
      const x1 = Math.min(this.width, Math.ceil(draw.cx + draw.rx) + 1), y1 = Math.min(this.height, Math.ceil(draw.cy + draw.ry) + 1);
      for (let y = y0; y < y1; y++) for (let x = x0; x < x1; x++) {
        const radius = ((x + .5 - draw.cx) / draw.rx) ** 2 + ((y + .5 - draw.cy) / draw.ry) ** 2;
        if (radius >= 1) continue;
        const value = Math.floor(clamp(draw.alpha, 0, 1) * 255 * Math.min(1 - radius, .35) / .35), i = y * this.width + x;
        this.mask[i] = Math.max(this.mask[i], value);
      }
    }
    paint(packet, draws, textures) {
      this.resize(packet.width, packet.height); this.clear(packet.clear || [19, 27, 31, 255]);
      for (const draw of draws) {
        if (draw.kind === "sprite") this.sprite(draw, textures.get(draw.tex).resource);
        else if (draw.kind === "rect") {
          const color = draw.color, alpha = color[3] / 255, out = this.output.data;
          for (let y = Math.max(0, Math.floor(draw.y)); y < Math.min(this.height, Math.ceil(draw.y + draw.h)); y++) for (let x = Math.max(0, Math.floor(draw.x)); x < Math.min(this.width, Math.ceil(draw.x + draw.w)); x++) {
            const i = (y * this.width + x) * 4;
            for (let ch = 0; ch < 3; ch++) out[i + ch] = draw.replace ? color[ch] : Math.floor(color[ch] * alpha + out[i + ch] * (1 - alpha));
            out[i + 3] = draw.replace ? color[3] : Math.floor(color[3] + out[i + 3] * (1 - alpha));
          }
        } else if (draw.kind === "shadow") {
          this.mask.fill(0);
          for (const part of draw.draws) {
            if (part.kind === "sprite") this.sprite(part, textures.get(part.tex).resource, true);
            else if (part.kind === "ellipse") this.ellipse(part);
            else throw new Error("Unsupported shadow command: " + part.kind);
          }
          const out = this.output.data;
          for (let i = 0; i < this.mask.length; i++) if (this.mask[i]) {
            const m = this.mask[i], keep = 255 - m, p = i * 4;
            out[p] = Math.floor(out[p] * keep / 255); out[p + 1] = Math.floor(out[p + 1] * keep / 255); out[p + 2] = Math.floor((out[p + 2] * keep + 100 * m) / 255);
          }
        } else throw new Error("Ellipse must belong to a shadow group");
      }
      this.ctx.putImageData(this.output, 0, 0);
    }
    destroy() { this.output = null; this.mask = null; }
  }

  const VERTEX = `#version 300 es
    in vec2 corner; uniform vec2 viewport; uniform mat3 matrix; uniform vec2 size; uniform float border;
    out vec2 uv;
    void main(){ vec2 local=corner*(size+2.0*border)-border; uv=local/size;
      vec2 p=(matrix*vec3(local,1)).xy; gl_Position=vec4(p.x/viewport.x*2.0-1.0,1.0-p.y/viewport.y*2.0,0,1); }`;
  const FRAGMENT = `#version 300 es
    precision highp float;
    uniform sampler2D image; uniform vec2 size; uniform float opacity; uniform vec3 addColor;
    uniform vec4 color; uniform int mode; uniform bool premultiplied; uniform bool replace; in vec2 uv; out vec4 outputColor;
    vec4 pixel(ivec2 p){ if(any(lessThan(p,ivec2(0)))||any(greaterThanEqual(p,ivec2(size))))return vec4(0);
      vec4 c=texelFetch(image,p,0); c.rgb=clamp(c.rgb+addColor,0.0,1.0); return c; }
    void main(){
      if(mode==1){outputColor=vec4(replace ? color.rgb : color.rgb*color.a,color.a);return;}
      if(mode==2){vec2 p=uv*2.0-1.0;float r=dot(p,p);if(r>=1.0)discard;
        float a=opacity*min(1.0-r,0.35)/0.35;outputColor=vec4(a);return;}
      if(mode==3){vec2 p=floor(uv*size);if(any(lessThan(p,vec2(0)))||any(greaterThanEqual(p,size)))discard;
        float a=texelFetch(image,ivec2(p),0).a*opacity;outputColor=vec4(a);return;}
      if(mode==4){float a=texture(image,vec2(uv.x,1.0-uv.y)).r;outputColor=vec4(vec3(0,0,100.0/255.0)*a,a);return;}
      vec2 p=uv*size-0.5;ivec2 a=ivec2(floor(p));vec2 w=fract(p);
      vec4 p0=pixel(a),p1=pixel(a+ivec2(1,0)),p2=pixel(a+ivec2(0,1)),p3=pixel(a+ivec2(1,1));
      vec4 weights=vec4((1.0-w.x)*(1.0-w.y),w.x*(1.0-w.y),(1.0-w.x)*w.y,w.x*w.y);
      float alpha=dot(vec4(p0.a,p1.a,p2.a,p3.a),weights);if(alpha<=0.5/255.0&&!replace)discard;
      if(!premultiplied&&!replace){p0.rgb*=p0.a;p1.rgb*=p1.a;p2.rgb*=p2.a;p3.rgb*=p3.a;}
      vec3 rgb=p0.rgb*weights.x+p1.rgb*weights.y+p2.rgb*weights.z+p3.rgb*weights.w;
      outputColor=vec4(rgb*opacity,alpha*opacity);
    }`;

  class GLPainter {
    constructor(canvas, gl) {
      this.canvas = canvas; this.gl = gl;
      const compile = (kind, source) => {const shader = gl.createShader(kind); gl.shaderSource(shader, source); gl.compileShader(shader); if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {const error = gl.getShaderInfoLog(shader); gl.deleteShader(shader); throw new Error(error);} return shader;};
      const vertex = compile(gl.VERTEX_SHADER, VERTEX), fragment = compile(gl.FRAGMENT_SHADER, FRAGMENT);
      this.program = gl.createProgram(); gl.attachShader(this.program, vertex); gl.attachShader(this.program, fragment); gl.linkProgram(this.program); gl.deleteShader(vertex); gl.deleteShader(fragment);
      if (!gl.getProgramParameter(this.program, gl.LINK_STATUS)) throw new Error(gl.getProgramInfoLog(this.program));
      this.buffer = gl.createBuffer(); this.vao = gl.createVertexArray(); gl.bindVertexArray(this.vao); gl.bindBuffer(gl.ARRAY_BUFFER, this.buffer);
      gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1]), gl.STATIC_DRAW);
      const attr = gl.getAttribLocation(this.program, "corner"); gl.enableVertexAttribArray(attr); gl.vertexAttribPointer(attr, 2, gl.FLOAT, false, 0, 0);
      this.uniforms = Object.fromEntries(["viewport", "matrix", "size", "border", "image", "opacity", "addColor", "color", "mode", "premultiplied", "replace"].map(name => [name, gl.getUniformLocation(this.program, name)]));
      this.matrix = new Float32Array(9); this.fbo = gl.createFramebuffer(); this.mask = gl.createTexture();
      this.white = this.upload({width: 1, height: 1, pixels: new Uint8Array([255, 255, 255, 255])});
    }
    upload(image) {
      const gl = this.gl, texture = gl.createTexture();
      if (Math.max(image.width, image.height) > gl.getParameter(gl.MAX_TEXTURE_SIZE)) throw new Error("Original texture exceeds this device’s GPU limit");
      gl.bindTexture(gl.TEXTURE_2D, texture);
      gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, false); gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, false); gl.pixelStorei(gl.UNPACK_COLORSPACE_CONVERSION_WEBGL, gl.NONE);
      if (image.pixels) gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, image.width, image.height, 0, gl.RGBA, gl.UNSIGNED_BYTE, image.pixels);
      else gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, gl.RGBA, gl.UNSIGNED_BYTE, image);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST); gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE); gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
      const error = gl.getError();
      if (error !== gl.NO_ERROR) {gl.deleteTexture(texture); throw new Error("Original texture upload failed (GPU " + error + ")");}
      return {texture, width: image.width, height: image.height};
    }
    release(resource) { this.gl.deleteTexture(resource.texture); }
    resize(w, h) {
      if (this.width === w && this.height === h) return;
      const gl = this.gl; this.width = this.canvas.width = w; this.height = this.canvas.height = h;
      gl.bindTexture(gl.TEXTURE_2D, this.mask); gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, w, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST); gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE); gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
      gl.bindFramebuffer(gl.FRAMEBUFFER, this.fbo); gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, this.mask, 0);
      if (gl.checkFramebufferStatus(gl.FRAMEBUFFER) !== gl.FRAMEBUFFER_COMPLETE) throw new Error("Shadow framebuffer is incomplete");
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    }
    blend(mode) {
      const gl = this.gl; gl.blendEquation(gl.FUNC_ADD);
      if (mode === "normal") gl.blendFuncSeparate(gl.ONE, gl.ONE_MINUS_SRC_ALPHA, gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
      else if (mode === "premultiplied-screen") gl.blendFuncSeparate(gl.ONE, gl.ONE_MINUS_SRC_COLOR, gl.ZERO, gl.ONE);
      else gl.blendFuncSeparate(gl.ONE, gl.ONE, gl.ZERO, gl.ONE);
    }
    quad(resource, matrix, alpha, mode, add, color, border = 0, premult = false, replace = false) {
      const gl = this.gl, u = this.uniforms, m = this.matrix;
      m.set([matrix[0], matrix[1], 0, matrix[2], matrix[3], 0, matrix[4], matrix[5], 1]);
      gl.uniformMatrix3fv(u.matrix, false, m); gl.uniform2f(u.size, resource.width, resource.height); gl.uniform1f(u.border, border);
      gl.uniform1f(u.opacity, clamp(alpha, 0, 1)); gl.uniform1i(u.mode, mode); gl.uniform1i(u.premultiplied, premult ? 1 : 0); gl.uniform1i(u.replace, replace ? 1 : 0);
      gl.uniform3f(u.addColor, number(add && add[0]) / 255, number(add && add[1]) / 255, number(add && add[2]) / 255);
      if (color) gl.uniform4f(u.color, color[0] / 255, color[1] / 255, color[2] / 255, color[3] / 255);
      gl.bindTexture(gl.TEXTURE_2D, resource.texture); gl.drawArrays(gl.TRIANGLES, 0, 6);
    }
    paint(packet, draws, textures) {
      const gl = this.gl; this.resize(packet.width, packet.height);
      gl.bindFramebuffer(gl.FRAMEBUFFER, null); gl.viewport(0, 0, this.width, this.height);
      const clear = packet.clear || [19, 27, 31, 255]; gl.clearColor(...clear.map(v => v / 255)); gl.clear(gl.COLOR_BUFFER_BIT);
      gl.useProgram(this.program); gl.bindVertexArray(this.vao); gl.activeTexture(gl.TEXTURE0); gl.uniform1i(this.uniforms.image, 0);
      gl.uniform2f(this.uniforms.viewport, this.width, this.height); gl.enable(gl.BLEND);
      for (const draw of draws) {
        if (draw.kind === "sprite") {
          const blend = draw.blend || "normal"; this.blend(blend); if (draw.replace) gl.disable(gl.BLEND); else gl.enable(gl.BLEND);
          this.quad(textures.get(draw.tex).resource, draw.m, number(draw.alpha, 1), 0, draw.add, null, draw.replace ? 0 : 1, blend.startsWith("premultiplied"), !!draw.replace);
        } else if (draw.kind === "rect") {
          this.blend("normal"); if (draw.replace) gl.disable(gl.BLEND); else gl.enable(gl.BLEND);
          this.quad(this.white, [draw.w, 0, 0, draw.h, draw.x, draw.y], 1, 1, null, draw.color, 0, false, !!draw.replace);
        } else if (draw.kind === "shadow") {
          gl.bindFramebuffer(gl.FRAMEBUFFER, this.fbo); gl.enable(gl.BLEND); gl.clearColor(0, 0, 0, 0); gl.clear(gl.COLOR_BUFFER_BIT);
          gl.blendEquation(gl.MAX); gl.blendFunc(gl.ONE, gl.ONE);
          for (const part of draw.draws) {
            if (part.kind === "sprite") this.quad(textures.get(part.tex).resource, part.m, number(part.alpha, 1), 3);
            else if (part.kind === "ellipse") this.quad(this.white, [part.rx * 2, 0, 0, part.ry * 2, part.cx - part.rx, part.cy - part.ry], part.alpha, 2);
            else throw new Error("Unsupported shadow command: " + part.kind);
          }
          gl.bindFramebuffer(gl.FRAMEBUFFER, null); this.blend("normal");
          this.quad({texture: this.mask, width: this.width, height: this.height}, [1, 0, 0, 1, 0, 0], 1, 4);
        } else throw new Error("Ellipse must belong to a shadow group");
      }
    }
    destroy() {
      const gl = this.gl; this.release(this.white); gl.deleteTexture(this.mask); gl.deleteFramebuffer(this.fbo); gl.deleteBuffer(this.buffer); gl.deleteVertexArray(this.vao); gl.deleteProgram(this.program);
    }
  }

  class SceneCompositor {
    constructor(canvas, options = {}) {
      if (!canvas || typeof options.textureSource !== "function") throw new Error("SceneCompositor needs a canvas and textureSource callback");
      this.canvas = canvas;
      this.options = options;
      this.textureSource = options.textureSource;
      this.delay = clamp(number(options.interpolationDelay, 50), 0, 250);
      this.maxBytes = Math.max(4 * 1024 * 1024, number(options.maxTextureBytes, 192 * 1024 * 1024));
      this.maxTextures = clamp(number(options.maxTextures, 4096), 32, 8192);
      this.textures = new Map();
      this.timeline = [];
      this.bytes = 0;
      this.destroyed = false;
      this.halted = false;
      this.ready = false;
      this.frameHandle = 0;
      this.shownNativeMs = 0;
      this.metrics = {backend: "", fps: 0, render_ms: 0, draws: 0, frames: 0, error: null};
      this.samples = [];
      this.now = typeof options.now === "function" ? options.now : now;
      const gl = options.forceCanvas ? null : canvas.getContext("webgl2", {
        alpha: options.alpha !== false,
        antialias: false,
        premultipliedAlpha: false,
        preserveDrawingBuffer: !!options.preserveDrawingBuffer,
        powerPreference: "high-performance"
      });
      this.painter = gl ? new GLPainter(canvas, gl) : new SoftwarePainter(canvas);
      this.metrics.backend = gl ? "webgl2" : "canvas2d";
      this.contextLost = event => {
        event.preventDefault();
        this.fail(new Error("The graphics context was lost; reload the scene or choose the original stream"));
        this.halted = true;
      };
      if (canvas.addEventListener) canvas.addEventListener("webglcontextlost", this.contextLost);
    }
    get stats() {
      return Object.assign({}, this.metrics, {
        texture_bytes: this.bytes,
        texture_count: this.textures.size,
        pending_frames: 0,
        missing: 0,
        loading_textures: 0,
        shown_native_ms: this.shownNativeMs
      });
    }
    fail(error) {
      if (this.destroyed) return;
      const message = error instanceof Error ? error.message : String(error);
      this.metrics.error = message;
      if (this.options.onError) this.options.onError(error instanceof Error ? error : new Error(message));
    }
    reset() {
      if (this.frameHandle && root.cancelAnimationFrame) root.cancelAnimationFrame(this.frameHandle);
      this.frameHandle = 0;
      this.timeline = [];
      this.ready = false;
      this.halted = !!(this.painter.gl && this.painter.gl.isContextLost());
      this.metrics.error = null;
      this.shownNativeMs = 0;
    }
    ensureTexture(id) {
      const existing = this.textures.get(id);
      if (existing) { existing.used = this.now(); return existing; }
      const source = this.textureSource(id);
      if (!source || !dimensions(source.width) || !dimensions(source.height)) throw new Error("Texture is not available: " + id);
      const pixels = source.rgba || source.pixels;
      if (!pixels) throw new Error("Texture is not available: " + id);
      const size = source.width * source.height * 4;
      if (size > this.maxBytes || !this.trim(size)) throw new Error("Visible scene exceeds the texture cache budget");
      const resource = this.painter.upload({width: source.width, height: source.height, pixels});
      const entry = {resource, bytes: size, used: this.now()};
      this.textures.set(id, entry);
      this.bytes += size;
      return entry;
    }
    releaseTexture(id) {
      const entry = this.textures.get(id);
      if (!entry) return;
      this.textures.delete(id);
      this.painter.release(entry.resource);
      this.bytes -= entry.bytes;
    }
    protectedIds() {
      const result = new Set();
      for (const frame of this.timeline) for (const id of frame.ids) result.add(id);
      return result;
    }
    trim(extraBytes = 0) {
      const protectedIds = this.protectedIds();
      const candidates = [...this.textures.entries()].filter(([id]) => !protectedIds.has(id)).sort((a, b) => a[1].used - b[1].used);
      for (const [id, entry] of candidates) {
        if (this.bytes + extraBytes <= this.maxBytes && this.textures.size < this.maxTextures) break;
        this.textures.delete(id);
        this.painter.release(entry.resource);
        this.bytes -= entry.bytes;
      }
      return this.bytes + extraBytes <= this.maxBytes && this.textures.size < this.maxTextures;
    }
    // A local pose is painted in this turn. Nothing stays pending on a texture fetch.
    push(packet) {
      if (this.destroyed || this.halted) return;
      try {
        if (!packet || packet.v !== 1 || !dimensions(packet.width) || !dimensions(packet.height) || !finite(packet.sequence) || !finite(packet.tick)) throw new Error("Unsupported scene packet");
        const ids = collectTextures(packet.draws);
        if (ids.size > 1024) throw new Error("Scene uses too many textures");
        for (const id of ids) this.ensureTexture(id);
        const nativeTime = number(packet.display_tick, packet.tick) * 50;
        const frame = {packet, ids, received: this.now(), readyAt: this.now(), nativeTime};
        const previous = this.timeline[this.timeline.length - 1];
        if (previous && (previous.packet.match_id !== packet.match_id || frame.nativeTime < previous.nativeTime)) this.timeline = [];
        this.timeline.push(frame);
        while (this.timeline.length > 3) this.timeline.shift();
        const keep = this.protectedIds();
        for (const id of packet.evicted || []) if (!keep.has(id)) this.releaseTexture(id);
        this.trim();
        this.render(this.now());
      } catch (error) {
        this.halted = true;
        this.fail(error);
      }
    }
    schedule() {
      if (this.frameHandle || this.destroyed || this.halted || !this.timeline.length || !root.requestAnimationFrame) return;
      this.frameHandle = root.requestAnimationFrame(time => { this.frameHandle = 0; this.render(time); });
    }
    render(time) {
      if (this.destroyed || this.halted || !this.timeline.length) return;
      try {
        const latest = this.timeline[this.timeline.length - 1];
        const readyAt = latest.readyAt ?? latest.received;
        let packet = latest.packet, draws = packet.draws, shown = latest.nativeTime;
        if (!stopped(packet) && this.timeline.length > 1) {
          const speed = clamp(number(packet.playback_speed, 1), .1, 16);
          const target = latest.nativeTime + (Math.max(0, time - readyAt) - this.delay) * speed;
          let older = this.timeline[0], newer = latest;
          for (let i = 1; i < this.timeline.length; i++) if (this.timeline[i].nativeTime >= target) { newer = this.timeline[i]; older = this.timeline[i - 1]; break; }
          const distance = newer.nativeTime - older.nativeTime;
          const amount = distance > 0 ? clamp((target - older.nativeTime) / distance, 0, 1) : 1;
          shown = distance > 0 ? older.nativeTime + amount * distance : newer.nativeTime;
          if (!stopped(older.packet) && distance <= 500 && older.packet.match_id === newer.packet.match_id) {
            packet = amount < 1 ? older.packet : newer.packet;
            draws = interpolateDraws(older.packet.draws, newer.packet.draws, amount);
          }
        }
        shown = clamp(shown, this.timeline[0].nativeTime, latest.nativeTime);
        this.shownNativeMs = shown;
        const started = this.now();
        this.painter.paint(packet, draws, this.textures);
        this.metrics.render_ms = Math.round((this.now() - started) * 100) / 100;
        this.metrics.draws = draws.length;
        this.metrics.frames++;
        this.samples.push(time);
        while (this.samples.length && this.samples[0] < time - 1000) this.samples.shift();
        this.metrics.fps = this.samples.length > 1 ? Math.round((this.samples.length - 1) * 1000 / Math.max(1, time - this.samples[0])) : 0;
        if (!this.ready) { this.ready = true; if (this.options.onReady) this.options.onReady(latest.packet); }
        const moving = !stopped(latest.packet) && time - readyAt < this.delay + 250;
        if (moving) this.schedule();
      } catch (error) {
        this.halted = true;
        this.fail(error);
      }
    }
    destroy() {
      if (this.destroyed) return;
      if (this.frameHandle && root.cancelAnimationFrame) root.cancelAnimationFrame(this.frameHandle);
      this.frameHandle = 0;
      for (const entry of this.textures.values()) this.painter.release(entry.resource);
      this.textures.clear();
      this.bytes = 0;
      this.timeline = [];
      this.destroyed = true;
      if (this.canvas.removeEventListener) this.canvas.removeEventListener("webglcontextlost", this.contextLost);
      this.painter.destroy();
      this.samples = [];
    }
  }

  root.SceneCompositor = SceneCompositor;
  root.SoftwarePainter = SoftwarePainter;
  root.GLPainter = GLPainter;
  root.interpolateDraws = interpolateDraws;
  root.collectTextures = collectTextures;
  const api = {SceneCompositor, SoftwarePainter, GLPainter, interpolateDraws, collectTextures};
  if (typeof module !== "undefined" && module.exports) module.exports = api;
})(typeof globalThis !== "undefined" ? globalThis : this);
