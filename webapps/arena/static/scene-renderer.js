/* Original-asset display-list compositor. No simulation or replacement artwork lives here. */
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

  // A faithful software fallback also makes blend/shadow behavior testable without a GPU.
  class SoftwarePainter {
    constructor(canvas) { this.canvas = canvas; this.ctx = canvas.getContext("2d", {alpha: true}); if (!this.ctx) throw new Error("Canvas rendering is unavailable"); }
    upload(image) {
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
  if (typeof module !== "undefined" && module.exports) module.exports = {ArenaSceneRenderer, SoftwarePainter, GLPainter, interpolateDraws, collectTextures, galleryFileUrl};
})(typeof window !== "undefined" ? window : globalThis);
