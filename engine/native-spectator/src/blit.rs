//! Small software compositing helpers for the spectator frame buffer.
//! Source sprites use straight (non-premultiplied) alpha, like the SC renderer.
use image::RgbaImage;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Blend {
    Normal,
    /// Source color added to the destination, as SC blend mode 8.
    Additive,
    /// Premultiplied additive source (an SC mode-8 clip rendered alone).
    PremultipliedAdditive,
    /// Premultiplied screen source (an SC mode-4 clip rendered alone).
    PremultipliedScreen,
}

/// Composite an untransformed sprite at an integer position with an opacity
/// multiplier and an optional additive color offset (used for hit flashes).
pub fn blit(
    dst: &mut RgbaImage,
    src: &RgbaImage,
    x: i64,
    y: i64,
    opacity: f32,
    blend: Blend,
    add: [i16; 3],
) {
    if opacity <= 0.003 {
        return;
    }
    let (dw, dh) = (dst.width() as i64, dst.height() as i64);
    let (sw, sh) = (src.width() as i64, src.height() as i64);
    let x0 = x.max(0);
    let y0 = y.max(0);
    let x1 = (x + sw).min(dw);
    let y1 = (y + sh).min(dh);
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    let opacity = (opacity.clamp(0., 1.) * 256.) as u32;
    let sbuf = src.as_raw();
    let dbuf: &mut [u8] = dst.as_mut();
    for py in y0..y1 {
        let srow = ((py - y) * sw) as usize * 4;
        let drow = (py * dw) as usize * 4;
        for px in x0..x1 {
            let si = srow + (px - x) as usize * 4;
            let a = (sbuf[si + 3] as u32 * opacity) >> 8;
            if a == 0 {
                continue;
            }
            let di = drow + px as usize * 4;
            for c in 0..3 {
                let s = (sbuf[si + c] as i32 + add[c] as i32).clamp(0, 255) as u32;
                let d = dbuf[di + c] as u32;
                dbuf[di + c] = match blend {
                    Blend::Normal => ((s * a + d * (255 - a)) / 255) as u8,
                    Blend::Additive => (d + s * a / 255).min(255) as u8,
                    Blend::PremultipliedAdditive => (d + s * opacity / 256).min(255) as u8,
                    Blend::PremultipliedScreen => {
                        let s = s * opacity / 256;
                        (255 - (255 - d) * (255 - s.min(255)) / 255) as u8
                    }
                };
            }
            if blend == Blend::Normal {
                let da = dbuf[di + 3] as u32;
                dbuf[di + 3] = (a + da * (255 - a) / 255).min(255) as u8;
            }
        }
    }
}

/// Affine placement: a source pixel at `(sx, sy)` occupies the local point
/// `(origin_x + sx, origin_y + sy)`, which maps to `m * local + (tx, ty)`.
pub struct Affine {
    pub m: [f64; 4],
    pub tx: f64,
    pub ty: f64,
}

impl Affine {
    pub fn scale_rotate(scale: f64, radians: f64, tx: f64, ty: f64) -> Self {
        let (s, c) = radians.sin_cos();
        Self {
            m: [c * scale, -s * scale, s * scale, c * scale],
            tx,
            ty,
        }
    }
}

/// Bilinear transformed composite with opacity and blend mode.
#[allow(clippy::too_many_arguments)]
pub fn blit_affine(
    dst: &mut RgbaImage,
    src: &RgbaImage,
    origin_x: f64,
    origin_y: f64,
    t: &Affine,
    opacity: f32,
    blend: Blend,
) {
    if opacity <= 0.003 || src.width() == 0 || src.height() == 0 {
        return;
    }
    let [a, b, c, d] = t.m;
    let det = a * d - b * c;
    if det.abs() < 1e-9 {
        return;
    }
    let (ia, ib, ic, id) = (d / det, -b / det, -c / det, a / det);
    let (sw, sh) = (src.width() as f64, src.height() as f64);
    let corners = [
        (origin_x, origin_y),
        (origin_x + sw, origin_y),
        (origin_x, origin_y + sh),
        (origin_x + sw, origin_y + sh),
    ];
    let (mut minx, mut miny, mut maxx, mut maxy) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for (lx, ly) in corners {
        let x = a * lx + b * ly + t.tx;
        let y = c * lx + d * ly + t.ty;
        minx = minx.min(x);
        miny = miny.min(y);
        maxx = maxx.max(x);
        maxy = maxy.max(y);
    }
    let (dw, dh) = (dst.width() as i64, dst.height() as i64);
    let x0 = (minx.floor() as i64).max(0);
    let y0 = (miny.floor() as i64).max(0);
    let x1 = (maxx.ceil() as i64 + 1).min(dw);
    let y1 = (maxy.ceil() as i64 + 1).min(dh);
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    let opacity = opacity.clamp(0., 1.);
    let swi = src.width() as i64;
    let shi = src.height() as i64;
    let sbuf = src.as_raw();
    let dbuf: &mut [u8] = dst.as_mut();
    let sample = |x: i64, y: i64| -> [f32; 4] {
        if x < 0 || y < 0 || x >= swi || y >= shi {
            return [0.; 4];
        }
        let i = ((y * swi + x) * 4) as usize;
        [
            sbuf[i] as f32,
            sbuf[i + 1] as f32,
            sbuf[i + 2] as f32,
            sbuf[i + 3] as f32,
        ]
    };
    for py in y0..y1 {
        for px in x0..x1 {
            let dx = px as f64 + 0.5 - t.tx;
            let dy = py as f64 + 0.5 - t.ty;
            let lx = ia * dx + ib * dy - origin_x - 0.5;
            let ly = ic * dx + id * dy - origin_y - 0.5;
            if lx < -1. || ly < -1. || lx > sw || ly > sh {
                continue;
            }
            let fx = lx.floor();
            let fy = ly.floor();
            let (wx, wy) = ((lx - fx) as f32, (ly - fy) as f32);
            let (ix, iy) = (fx as i64, fy as i64);
            let p00 = sample(ix, iy);
            let p10 = sample(ix + 1, iy);
            let p01 = sample(ix, iy + 1);
            let p11 = sample(ix + 1, iy + 1);
            // Alpha-weighted bilinear filtering avoids dark fringes.
            let w = [
                (1. - wx) * (1. - wy) * p00[3],
                wx * (1. - wy) * p10[3],
                (1. - wx) * wy * p01[3],
                wx * wy * p11[3],
            ];
            let alpha = w[0] + w[1] + w[2] + w[3];
            if alpha <= 0.5 {
                continue;
            }
            let di = ((py * dw + px) * 4) as usize;
            if matches!(
                blend,
                Blend::PremultipliedAdditive | Blend::PremultipliedScreen
            ) {
                let plain = [
                    (1. - wx) * (1. - wy),
                    wx * (1. - wy),
                    (1. - wx) * wy,
                    wx * wy,
                ];
                for ch in 0..3 {
                    let v = (p00[ch] * plain[0]
                        + p10[ch] * plain[1]
                        + p01[ch] * plain[2]
                        + p11[ch] * plain[3])
                        * opacity;
                    let dv = dbuf[di + ch] as f32;
                    dbuf[di + ch] = if blend == Blend::PremultipliedAdditive {
                        (dv + v).min(255.)
                    } else {
                        255. - (255. - dv) * (255. - v.min(255.)) / 255.
                    } as u8;
                }
                continue;
            }
            let mut rgb = [0f32; 3];
            for (ch, value) in rgb.iter_mut().enumerate() {
                *value =
                    (p00[ch] * w[0] + p10[ch] * w[1] + p01[ch] * w[2] + p11[ch] * w[3]) / alpha;
            }
            let sa = (alpha / 255. * opacity).min(1.);
            for ch in 0..3 {
                let dv = dbuf[di + ch] as f32;
                dbuf[di + ch] = match blend {
                    Blend::Additive => (dv + rgb[ch] * sa).min(255.) as u8,
                    _ => (rgb[ch] * sa + dv * (1. - sa)) as u8,
                };
            }
            if blend == Blend::Normal {
                let da = dbuf[di + 3] as f32;
                dbuf[di + 3] = (sa * 255. + da * (1. - sa)).min(255.) as u8;
            }
        }
    }
}

/// Accumulates ground shadows so overlapping shadows darken only once.
pub struct ShadowMask {
    width: u32,
    height: u32,
    mask: Vec<u8>,
    dirty: Option<(i64, i64, i64, i64)>,
}

impl ShadowMask {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            mask: vec![0; (width * height) as usize],
            dirty: None,
        }
    }

    fn touch(&mut self, x0: i64, y0: i64, x1: i64, y1: i64) {
        self.dirty = Some(match self.dirty {
            Some((a, b, c, d)) => (a.min(x0), b.min(y0), c.max(x1), d.max(y1)),
            None => (x0, y0, x1, y1),
        });
    }

    /// Add a sprite silhouette transformed by `t` with peak darkness `strength`.
    pub fn add_silhouette(
        &mut self,
        src: &RgbaImage,
        origin_x: f64,
        origin_y: f64,
        t: &Affine,
        strength: f32,
    ) {
        let [a, b, c, d] = t.m;
        let det = a * d - b * c;
        if det.abs() < 1e-9 || src.width() == 0 {
            return;
        }
        let (ia, ib, ic, id) = (d / det, -b / det, -c / det, a / det);
        let (sw, sh) = (src.width() as f64, src.height() as f64);
        let (mut minx, mut miny, mut maxx, mut maxy) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
        for (lx, ly) in [
            (origin_x, origin_y),
            (origin_x + sw, origin_y),
            (origin_x, origin_y + sh),
            (origin_x + sw, origin_y + sh),
        ] {
            let x = a * lx + b * ly + t.tx;
            let y = c * lx + d * ly + t.ty;
            minx = minx.min(x);
            miny = miny.min(y);
            maxx = maxx.max(x);
            maxy = maxy.max(y);
        }
        let x0 = (minx.floor() as i64).max(0);
        let y0 = (miny.floor() as i64).max(0);
        let x1 = (maxx.ceil() as i64 + 1).min(self.width as i64);
        let y1 = (maxy.ceil() as i64 + 1).min(self.height as i64);
        if x0 >= x1 || y0 >= y1 {
            return;
        }
        self.touch(x0, y0, x1, y1);
        let swi = src.width() as i64;
        let shi = src.height() as i64;
        let sbuf = src.as_raw();
        let peak = (strength.clamp(0., 1.) * 255.) as u32;
        for py in y0..y1 {
            for px in x0..x1 {
                let dx = px as f64 + 0.5 - t.tx;
                let dy = py as f64 + 0.5 - t.ty;
                let sx = (ia * dx + ib * dy - origin_x).floor() as i64;
                let sy = (ic * dx + id * dy - origin_y).floor() as i64;
                if sx < 0 || sy < 0 || sx >= swi || sy >= shi {
                    continue;
                }
                let alpha = sbuf[((sy * swi + sx) * 4 + 3) as usize] as u32;
                let value = (alpha * peak / 255) as u8;
                let m = &mut self.mask[(py * self.width as i64 + px) as usize];
                *m = (*m).max(value);
            }
        }
    }

    /// Soft elliptical blob, for objects without a sprite silhouette.
    pub fn add_ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64, strength: f32) {
        if rx < 0.5 || ry < 0.5 {
            return;
        }
        let x0 = ((cx - rx).floor() as i64).max(0);
        let y0 = ((cy - ry).floor() as i64).max(0);
        let x1 = ((cx + rx).ceil() as i64 + 1).min(self.width as i64);
        let y1 = ((cy + ry).ceil() as i64 + 1).min(self.height as i64);
        if x0 >= x1 || y0 >= y1 {
            return;
        }
        self.touch(x0, y0, x1, y1);
        let peak = strength.clamp(0., 1.) * 255.;
        for py in y0..y1 {
            for px in x0..x1 {
                let nx = (px as f64 + 0.5 - cx) / rx;
                let ny = (py as f64 + 0.5 - cy) / ry;
                let r = nx * nx + ny * ny;
                if r >= 1. {
                    continue;
                }
                let falloff = (1. - r).min(0.35) / 0.35;
                let value = (peak * falloff as f32) as u8;
                let m = &mut self.mask[(py * self.width as i64 + px) as usize];
                *m = (*m).max(value);
            }
        }
    }

    /// Darken `dst` by the accumulated mask and clear it.
    pub fn apply(&mut self, dst: &mut RgbaImage) {
        let Some((x0, y0, x1, y1)) = self.dirty.take() else {
            return;
        };
        let width = self.width as i64;
        let dw = dst.width() as i64;
        let dbuf: &mut [u8] = dst.as_mut();
        for py in y0..y1 {
            for px in x0..x1 {
                let mi = (py * width + px) as usize;
                let m = self.mask[mi] as u32;
                if m == 0 {
                    continue;
                }
                self.mask[mi] = 0;
                let di = ((py * dw + px) * 4) as usize;
                // Location shadow color: TrainingCamp ShadowR/G/B = 0/0/100.
                let keep = 255 - m;
                dbuf[di] = (dbuf[di] as u32 * keep / 255) as u8;
                dbuf[di + 1] = (dbuf[di + 1] as u32 * keep / 255) as u8;
                dbuf[di + 2] = ((dbuf[di + 2] as u32 * keep + 100 * m) / 255) as u8;
            }
        }
    }
}
