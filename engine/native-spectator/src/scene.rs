//! Original-art display lists. This is a scoped presentation target for the
//! existing Painter, not another implementation of animation or game rules.
//!
//! Only the registered root image is captured. Offscreen SC timeline renders,
//! cards, fonts and fitted poses continue through their original rasterizers.
//! The scope is thread-local because Painter owns Rc-backed asset caches and is
//! deliberately single-threaded. No pointer is dereferenced or retained beyond
//! a draw; the root allocation's address is only compared for target identity.
use crate::blit::{Affine, Blend};
use image::{imageops, ImageFormat, Rgba, RgbaImage};
use serde::Serialize;
use std::{
    cell::RefCell,
    collections::{hash_map::DefaultHasher, BTreeMap, BTreeSet},
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    rc::Rc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug, Serialize)]
pub struct Texture {
    pub id: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Draw {
    Sprite {
        id: String,
        tex: String,
        m: [f64; 6],
        alpha: f32,
        blend: &'static str,
        add: [i16; 3],
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        replace: bool,
    },
    Rect {
        id: String,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: [u8; 4],
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        replace: bool,
    },
    Shadow {
        id: String,
        draws: Vec<Draw>,
    },
    Ellipse {
        id: String,
        cx: f64,
        cy: f64,
        rx: f64,
        ry: f64,
        alpha: f32,
    },
}

/// An immutable, content-addressed PNG cache. Repeated sprites are hashed but
/// never re-encoded. Content rather than allocation addresses handles temporary
/// HUD pieces and allocator reuse safely. The two domain-separated 64-bit hashes
/// are transport identifiers, not an authentication or cryptographic primitive.
struct CacheEntry {
    texture: Texture,
    bytes: u64,
    last_used: Duration,
}

pub struct TextureCache {
    root: PathBuf,
    entries: BTreeMap<String, CacheEntry>,
    pub encoded_bytes: u64,
    pub encodes: u64,
    max_textures: usize,
    max_bytes: u64,
    grace: Duration,
    clock: Rc<dyn Fn() -> Duration>,
    current: BTreeSet<String>,
    published: BTreeSet<String>,
    evicted: BTreeSet<String>,
}

impl TextureCache {
    pub fn new(root: &Path) -> Result<Self, String> {
        fs::create_dir_all(root).map_err(|e| format!("create texture cache: {e}"))?;
        let mut cache = Self {
            root: root.to_owned(),
            entries: BTreeMap::new(),
            encoded_bytes: 0,
            encodes: 0,
            max_textures: 8192,
            max_bytes: 256 * 1024 * 1024,
            grace: Duration::from_secs(30),
            clock: Rc::new(|| {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
            }),
            current: BTreeSet::new(),
            published: BTreeSet::new(),
            evicted: BTreeSet::new(),
        };
        // Account for immutable files left by a presenter restart. Ignoring
        // these would make the on-disk budget grow without bound across runs.
        for item in fs::read_dir(root).map_err(|e| format!("read texture cache: {e}"))? {
            let item = item.map_err(|e| e.to_string())?;
            if !item.file_type().map_err(|e| e.to_string())?.is_file() {
                continue;
            }
            let name = item.file_name();
            let Some(name) = name.to_str() else {
                continue;
            };
            let Some(id) = name.strip_suffix(".png").filter(|s| {
                s.len() == 32
                    && s.bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            }) else {
                continue;
            };
            let metadata = item.metadata().map_err(|e| e.to_string())?;
            let (width, height) = image::image_dimensions(item.path())
                .map_err(|e| format!("invalid cached original texture: {e}"))?;
            cache.encoded_bytes = cache.encoded_bytes.saturating_add(metadata.len());
            cache.entries.insert(
                id.to_owned(),
                CacheEntry {
                    texture: Texture {
                        id: id.to_owned(),
                        width,
                        height,
                    },
                    bytes: metadata.len(),
                    last_used: metadata
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .unwrap_or_default(),
                },
            );
        }
        cache.make_room(0, 0)?;
        Ok(cache)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn begin_frame(&mut self) {
        self.current.clear();
    }
    fn finish_frame(&mut self) -> Vec<String> {
        self.published = std::mem::take(&mut self.current);
        std::mem::take(&mut self.evicted).into_iter().collect()
    }

    fn make_room(&mut self, bytes: u64, count: usize) -> Result<(), String> {
        if self.entries.len().saturating_add(count) <= self.max_textures
            && self.encoded_bytes.saturating_add(bytes) <= self.max_bytes
        {
            return Ok(());
        }
        let now = (self.clock)();
        let mut cold: Vec<_> = self
            .entries
            .iter()
            .filter(|(id, entry)| {
                !self.current.contains(*id)
                    && !self.published.contains(*id)
                    && now.saturating_sub(entry.last_used) >= self.grace
            })
            .map(|(id, entry)| (entry.last_used, id.clone()))
            .collect();
        cold.sort();
        let mut cold = cold.into_iter();
        while self.entries.len().saturating_add(count) > self.max_textures
            || self.encoded_bytes.saturating_add(bytes) > self.max_bytes
        {
            let Some((_, id)) = cold.next() else {
                return Err(format!("original texture working set exceeds {} textures / {} bytes within the {}s in-flight grace", self.max_textures, self.max_bytes, self.grace.as_secs()));
            };
            fs::remove_file(self.root.join(format!("{id}.png")))
                .map_err(|e| format!("evict cold original texture: {e}"))?;
            let entry = self.entries.remove(&id).expect("candidate exists");
            self.encoded_bytes = self.encoded_bytes.saturating_sub(entry.bytes);
            self.evicted.insert(id);
        }
        Ok(())
    }

    fn texture(&mut self, image: &RgbaImage) -> Result<(Texture, bool), String> {
        if image.width() == 0
            || image.height() == 0
            || image.width() > 4096
            || image.height() > 4096
        {
            return Err("original sprite dimensions exceed texture limit".into());
        }
        let id = texture_id(image);
        self.current.insert(id.clone());
        if let Some(entry) = self.entries.get_mut(&id) {
            entry.last_used = (self.clock)();
            return Ok((entry.texture.clone(), false));
        }
        let path = self.root.join(format!("{id}.png"));
        let temp = self.root.join(format!(".{id}.{}.tmp", std::process::id()));
        let result = image
            .save_with_format(&temp, ImageFormat::Png)
            .map_err(|e| format!("encode original texture: {e}"))
            .and_then(|_| {
                let bytes = fs::metadata(&temp).map_err(|e| e.to_string())?.len();
                self.make_room(bytes, 1)?;
                fs::rename(&temp, &path).map_err(|e| format!("publish original texture: {e}"))?;
                Ok(bytes)
            });
        if result.is_err() {
            let _ = fs::remove_file(&temp);
        }
        let bytes = result?;
        self.encodes += 1;
        self.encoded_bytes += bytes;
        let texture = Texture {
            id: id.clone(),
            width: image.width(),
            height: image.height(),
        };
        // A cold pose can be evicted before another part references it later in
        // this capture. Reintroduced IDs are valid again before publication.
        self.evicted.remove(&id);
        self.entries.insert(
            id,
            CacheEntry {
                texture: texture.clone(),
                bytes,
                last_used: (self.clock)(),
            },
        );
        Ok((texture, true))
    }
}

fn texture_id(image: &RgbaImage) -> String {
    let hash = |salt: u64| {
        let mut h = DefaultHasher::new();
        salt.hash(&mut h);
        image.width().hash(&mut h);
        image.height().hash(&mut h);
        image.as_raw().hash(&mut h);
        h.finish()
    };
    format!(
        "{:016x}{:016x}",
        hash(0x7363656e655f7631),
        hash(0x746578747572655f)
    )
}

#[derive(Default)]
pub struct DisplayList {
    /// Private bridge bookkeeping; never forwarded as a public scene field.
    pub evicted: Vec<String>,
    pub textures: Vec<Texture>,
    pub draws: Vec<Draw>,
}

struct Capture {
    root: usize,
    width: u32,
    height: u32,
    cache: Rc<RefCell<TextureCache>>,
    list: DisplayList,
    shadows: Vec<Draw>,
    scope: String,
    counters: BTreeMap<String, usize>,
    error: Option<String>,
}

impl Capture {
    fn id(&mut self) -> String {
        let counter = self.counters.entry(self.scope.clone()).or_default();
        let id = format!("{}-{}", self.scope, counter);
        *counter += 1;
        id
    }

    fn sprite(
        &mut self,
        src: &RgbaImage,
        m: [f64; 6],
        alpha: f32,
        blend: Blend,
        add: [i16; 3],
        replace: bool,
        shadow: bool,
    ) {
        if self.error.is_some() {
            return;
        }
        if self.list.draws.len() + self.shadows.len() >= 12000 {
            self.error = Some("original display list exceeded 12000 draw commands".into());
            return;
        }
        let texture = self.cache.borrow_mut().texture(src);
        match texture {
            Ok((texture, new)) => {
                if new {
                    self.list.textures.push(texture.clone());
                }
                let draw = Draw::Sprite {
                    id: self.id(),
                    tex: texture.id,
                    m,
                    alpha: alpha.clamp(0., 1.),
                    blend: match blend {
                        Blend::Normal => "normal",
                        Blend::Additive => "add",
                        Blend::PremultipliedAdditive => "premultiplied-add",
                        Blend::PremultipliedScreen => "premultiplied-screen",
                    },
                    add,
                    replace,
                };
                if shadow {
                    self.shadows.push(draw);
                } else {
                    self.list.draws.push(draw);
                }
            }
            Err(e) => self.error = Some(e),
        }
    }
}

thread_local! { static ACTIVE: RefCell<Option<Capture>> = const { RefCell::new(None) }; }

/// RAII scope: failures and unwinding cannot accidentally capture a later CPU
/// fallback frame. Root registration happens inside Painter::draw itself.
pub struct CaptureGuard;
impl CaptureGuard {
    pub fn begin(cache: Rc<RefCell<TextureCache>>) -> Result<Self, String> {
        ACTIVE.with(|a| {
            let mut a = a.borrow_mut();
            if a.is_some() {
                return Err("nested original scene capture".into());
            }
            cache.borrow_mut().begin_frame();
            *a = Some(Capture {
                root: 0,
                width: 0,
                height: 0,
                cache,
                list: DisplayList::default(),
                shadows: Vec::new(),
                scope: "root".into(),
                counters: BTreeMap::new(),
                error: None,
            });
            Ok(Self)
        })
    }

    pub fn finish(self) -> Result<DisplayList, String> {
        let mut capture = ACTIVE
            .with(|a| a.borrow_mut().take())
            .ok_or("scene capture missing")?;
        if let Some(error) = capture.error {
            return Err(error);
        }
        if !capture.shadows.is_empty() {
            return Err("unapplied original shadow group".into());
        }
        capture.list.evicted = capture.cache.borrow_mut().finish_frame();
        Ok(capture.list)
    }
}
impl Drop for CaptureGuard {
    fn drop(&mut self) {
        ACTIVE.with(|a| {
            a.borrow_mut().take();
        });
    }
}

fn image_key(image: &RgbaImage) -> usize {
    image.as_raw().as_ptr() as usize
}

pub fn register_root(image: &RgbaImage) {
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            c.root = image_key(image);
            c.width = image.width();
            c.height = image.height();
        }
    });
}

pub fn scope<T>(name: impl Into<String>, f: impl FnOnce() -> T) -> T {
    let _guard = ScopeGuard::enter(name);
    f()
}

pub struct ScopeGuard(Option<String>);
impl ScopeGuard {
    pub fn enter(name: impl Into<String>) -> Self {
        Self(ACTIVE.with(|a| {
            a.borrow_mut()
                .as_mut()
                .map(|c| std::mem::replace(&mut c.scope, name.into()))
        }))
    }
}
impl Drop for ScopeGuard {
    fn drop(&mut self) {
        if let Some(previous) = self.0.take() {
            ACTIVE.with(|a| {
                if let Some(c) = a.borrow_mut().as_mut() {
                    c.scope = previous;
                }
            });
        }
    }
}

pub fn capture_blit(
    dst: &RgbaImage,
    src: &RgbaImage,
    m: [f64; 6],
    alpha: f32,
    blend: Blend,
    add: [i16; 3],
    replace: bool,
) -> bool {
    ACTIVE.with(|a| {
        let mut a = a.borrow_mut();
        let Some(c) = a
            .as_mut()
            .filter(|c| c.root != 0 && c.root == image_key(dst))
        else {
            return false;
        };
        c.sprite(src, m, alpha, blend, add, replace, false);
        true
    })
}

pub fn affine_matrix(origin_x: f64, origin_y: f64, t: &Affine) -> [f64; 6] {
    let [a, b, c, d] = t.m;
    [
        a,
        c,
        b,
        d,
        t.tx + a * origin_x + b * origin_y,
        t.ty + c * origin_x + d * origin_y,
    ]
}

pub fn capture_shadow(src: &RgbaImage, ox: f64, oy: f64, t: &Affine, alpha: f32) -> bool {
    ACTIVE.with(|a| {
        let mut a = a.borrow_mut();
        let Some(c) = a.as_mut().filter(|c| c.root != 0) else {
            return false;
        };
        c.sprite(
            src,
            affine_matrix(ox, oy, t),
            alpha,
            Blend::Normal,
            [0; 3],
            false,
            true,
        );
        true
    })
}

pub fn capture_ellipse(cx: f64, cy: f64, rx: f64, ry: f64, alpha: f32) -> bool {
    ACTIVE.with(|a| {
        let mut a = a.borrow_mut();
        let Some(c) = a.as_mut().filter(|c| c.root != 0) else {
            return false;
        };
        let id = c.id();
        c.shadows.push(Draw::Ellipse {
            id,
            cx,
            cy,
            rx,
            ry,
            alpha,
        });
        true
    })
}

pub fn capture_shadow_apply(dst: &RgbaImage) -> bool {
    ACTIVE.with(|a| {
        let mut a = a.borrow_mut();
        let Some(c) = a
            .as_mut()
            .filter(|c| c.root != 0 && c.root == image_key(dst))
        else {
            return false;
        };
        if !c.shadows.is_empty() {
            let id = c.id();
            c.list.draws.push(Draw::Shadow {
                id,
                draws: std::mem::take(&mut c.shadows),
            });
        }
        true
    })
}

pub fn capture_rect(dst: &RgbaImage, x: i32, y: i32, w: i32, h: i32, color: Rgba<u8>) -> bool {
    ACTIVE.with(|a| {
        let mut a = a.borrow_mut();
        let Some(c) = a
            .as_mut()
            .filter(|c| c.root != 0 && c.root == image_key(dst))
        else {
            return false;
        };
        let x0 = x.max(0);
        let y0 = y.max(0);
        let w = (x.saturating_add(w)).min(c.width as i32) - x0;
        let h = (y.saturating_add(h)).min(c.height as i32) - y0;
        if w > 0 && h > 0 {
            let id = c.id();
            c.list.draws.push(Draw::Rect {
                id,
                x: x0,
                y: y0,
                w,
                h,
                color: color.0,
                replace: true,
            });
        }
        true
    })
}

pub fn overlay(dst: &mut RgbaImage, src: &RgbaImage, x: i64, y: i64) {
    if !capture_blit(
        dst,
        src,
        [1., 0., 0., 1., x as f64, y as f64],
        1.,
        Blend::Normal,
        [0; 3],
        false,
    ) {
        imageops::overlay(dst, src, x, y);
    }
}
pub fn replace(dst: &mut RgbaImage, src: &RgbaImage, x: i64, y: i64) {
    if !capture_blit(
        dst,
        src,
        [1., 0., 0., 1., x as f64, y as f64],
        1.,
        Blend::Normal,
        [0; 3],
        true,
    ) {
        imageops::replace(dst, src, x, y);
    }
}
pub fn fill_rect(dst: &mut RgbaImage, x: i32, y: i32, w: i32, h: i32, color: Rgba<u8>) {
    if w <= 0 || h <= 0 || capture_rect(dst, x, y, w, h, color) {
        return;
    }
    for py in y.max(0)..(y.saturating_add(h)).min(dst.height() as i32) {
        for px in x.max(0)..(x.saturating_add(w)).min(dst.width() as i32) {
            dst.put_pixel(px as u32, py as u32, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        blit::{self, ShadowMask},
        draw::Painter,
        frame::{BattleFrame, Entity, HandCard, HumanFrame},
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestCache {
        root: PathBuf,
        cache: Rc<RefCell<TextureCache>>,
    }
    impl TestCache {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let root = std::env::temp_dir().join(format!(
                "original-scene-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            let cache = Rc::new(RefCell::new(TextureCache::new(&root).unwrap()));
            Self { root, cache }
        }
    }
    impl Drop for TestCache {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn blend(value: &str) -> Blend {
        match value {
            "add" => Blend::Additive,
            "premultiplied-add" => Blend::PremultipliedAdditive,
            "premultiplied-screen" => Blend::PremultipliedScreen,
            _ => Blend::Normal,
        }
    }

    /// Test reference compositor for the protocol, using the original software
    /// blend equations. The browser has separate GPU/readback parity tests.
    fn replay(draws: &[Draw], root: &Path, out: &mut RgbaImage) {
        for draw in draws {
            match draw {
                Draw::Sprite {
                    tex,
                    m,
                    alpha,
                    blend: mode,
                    add,
                    replace,
                    ..
                } => {
                    let src = image::open(root.join(format!("{tex}.png")))
                        .unwrap()
                        .to_rgba8();
                    if *replace {
                        assert_eq!(&m[..4], &[1., 0., 0., 1.]);
                        imageops::replace(out, &src, m[4] as i64, m[5] as i64);
                    } else if m[..4] == [1., 0., 0., 1.] && m[4].fract() == 0. && m[5].fract() == 0.
                    {
                        blit::blit(
                            out,
                            &src,
                            m[4] as i64,
                            m[5] as i64,
                            *alpha,
                            blend(mode),
                            *add,
                        );
                    } else {
                        assert_eq!(*add, [0; 3]);
                        blit::blit_affine(
                            out,
                            &src,
                            0.,
                            0.,
                            &Affine {
                                m: [m[0], m[2], m[1], m[3]],
                                tx: m[4],
                                ty: m[5],
                            },
                            *alpha,
                            blend(mode),
                        );
                    }
                }
                Draw::Rect {
                    x, y, w, h, color, ..
                } => fill_rect(out, *x, *y, *w, *h, Rgba(*color)),
                Draw::Shadow { draws, .. } => {
                    let mut shadows = ShadowMask::new(out.width(), out.height());
                    for draw in draws {
                        match draw {
                            Draw::Sprite { tex, m, alpha, .. } => {
                                let src = image::open(root.join(format!("{tex}.png")))
                                    .unwrap()
                                    .to_rgba8();
                                shadows.add_silhouette(
                                    &src,
                                    0.,
                                    0.,
                                    &Affine {
                                        m: [m[0], m[2], m[1], m[3]],
                                        tx: m[4],
                                        ty: m[5],
                                    },
                                    *alpha,
                                );
                            }
                            Draw::Ellipse {
                                cx,
                                cy,
                                rx,
                                ry,
                                alpha,
                                ..
                            } => shadows.add_ellipse(*cx, *cy, *rx, *ry, *alpha),
                            _ => panic!("invalid original shadow child"),
                        }
                    }
                    shadows.apply(out);
                }
                Draw::Ellipse { .. } => panic!("ellipse outside original shadow group"),
            }
        }
    }

    fn primitives(root: &mut RgbaImage, art: &RgbaImage) {
        fill_rect(root, -2, 3, 16, 11, Rgba([27, 65, 95, 200]));
        replace(root, art, 2, 2);
        blit::blit(root, art, 15, 8, 0.7, Blend::Normal, [12, 0, -4]);
        for (index, blend) in [
            Blend::Normal,
            Blend::Additive,
            Blend::PremultipliedAdditive,
            Blend::PremultipliedScreen,
        ]
        .iter()
        .enumerate()
        {
            scope(format!("primitive-{index}"), || {
                blit::blit_affine(
                    root,
                    art,
                    -2.,
                    3.,
                    &Affine {
                        m: [0.8, 0.25, -0.3, 1.2],
                        tx: 8. + index as f64 * 11.,
                        ty: 29.,
                    },
                    0.65,
                    *blend,
                )
            });
        }
        let mut shadow = ShadowMask::new(root.width(), root.height());
        shadow.add_ellipse(25., 22., 18., 9., 0.5);
        shadow.add_ellipse(25., 22., 18., 9., 0.5); // MAX, never double-darken.
        shadow.add_silhouette(
            art,
            -3.,
            0.,
            &Affine {
                m: [1.3, 0.2, 0., 0.4],
                tx: 25.,
                ty: 19.,
            },
            0.8,
        );
        shadow.apply(root);
    }

    #[test]
    fn capture_replays_original_affines_blends_clipping_and_max_shadows() {
        let cache = TestCache::new();
        let mut art = RgbaImage::new(7, 9);
        for (x, y, pixel) in art.enumerate_pixels_mut() {
            *pixel = Rgba([
                x as u8 * 27,
                y as u8 * 23,
                170,
                ((x + y) * 17).min(255) as u8,
            ]);
        }
        let background = RgbaImage::from_pixel(80, 64, Rgba([19, 27, 31, 255]));
        let mut expected = background.clone();
        primitives(&mut expected, &art);
        let capture = CaptureGuard::begin(cache.cache.clone()).unwrap();
        let mut root = background.clone();
        register_root(&root);
        // Offscreen work must still rasterize, not become a scene-level draw.
        let mut offscreen = RgbaImage::new(7, 9);
        overlay(&mut offscreen, &art, 0, 0);
        let mut offscreen_expected = RgbaImage::new(7, 9);
        imageops::overlay(&mut offscreen_expected, &art, 0, 0);
        assert_eq!(offscreen, offscreen_expected);
        primitives(&mut root, &art);
        let list = capture.finish().unwrap();
        assert_eq!(root, background, "capture must skip root pixel compositing");
        assert_eq!(list.textures.len(), 1);
        let mut actual = background;
        replay(&list.draws, &cache.root, &mut actual);
        assert_eq!(actual, expected);
        let encoded = cache.cache.borrow().encodes;
        let capture = CaptureGuard::begin(cache.cache.clone()).unwrap();
        register_root(&root);
        primitives(&mut root, &art);
        let second = capture.finish().unwrap();
        assert!(
            second.textures.is_empty(),
            "known art must never be re-encoded or retransmitted"
        );
        assert_eq!(cache.cache.borrow().encodes, encoded);
    }

    #[test]
    fn stable_ids_do_not_follow_z_order_or_texture_identity() {
        let cache = TestCache::new();
        let mut root = RgbaImage::new(20, 20);
        let a = RgbaImage::from_pixel(2, 2, Rgba([255, 10, 0, 255]));
        let b = RgbaImage::from_pixel(2, 2, Rgba([0, 255, 0, 255]));
        let capture = CaptureGuard::begin(cache.cache.clone()).unwrap();
        register_root(&root);
        scope("entity-7-part-0", || overlay(&mut root, &a, 1, 2));
        scope("entity-9-part-0", || overlay(&mut root, &b, 3, 4));
        let first = capture.finish().unwrap();
        let capture = CaptureGuard::begin(cache.cache.clone()).unwrap();
        register_root(&root);
        scope("entity-9-part-0", || overlay(&mut root, &b, 4, 5));
        scope("entity-7-part-0", || overlay(&mut root, &b, 2, 3));
        let second = capture.finish().unwrap();
        let id = |d: &Draw| match d {
            Draw::Sprite { id, .. } => id.clone(),
            _ => panic!(),
        };
        assert_eq!(id(&first.draws[0]), id(&second.draws[1]));
        assert_eq!(id(&first.draws[1]), id(&second.draws[0]));
    }

    #[test]
    fn cache_budget_fails_clearly_and_capture_is_scoped() {
        let cache = TestCache::new();
        cache.cache.borrow_mut().max_textures = 1;
        let mut root = RgbaImage::new(20, 20);
        let a = RgbaImage::from_pixel(2, 2, Rgba([1, 2, 3, 255]));
        let b = RgbaImage::from_pixel(2, 2, Rgba([3, 2, 1, 255]));
        let capture = CaptureGuard::begin(cache.cache.clone()).unwrap();
        register_root(&root);
        overlay(&mut root, &a, 0, 0);
        overlay(&mut root, &b, 2, 2);
        assert!(capture
            .finish()
            .err()
            .unwrap()
            .contains("working set exceeds 1 textures"));
        overlay(&mut root, &a, 0, 0);
        assert_eq!(root.get_pixel(0, 0), a.get_pixel(0, 0));
    }

    fn capture_art(cache: &TestCache, images: &[&RgbaImage]) -> Result<DisplayList, String> {
        let capture = CaptureGuard::begin(cache.cache.clone())?;
        let mut root = RgbaImage::new(32, 32);
        register_root(&root);
        for (index, image) in images.iter().enumerate() {
            overlay(&mut root, image, index as i64 * 3, 0);
        }
        capture.finish()
    }

    #[test]
    fn lru_evicts_cold_poses_with_correct_disk_bytes_and_pins_current_frame() {
        use std::cell::Cell;
        let cache = TestCache::new();
        let time = Rc::new(Cell::new(0u64));
        let timer = time.clone();
        {
            let mut cache = cache.cache.borrow_mut();
            cache.clock = Rc::new(move || Duration::from_secs(timer.get()));
            cache.max_textures = 3;
        }
        let a = RgbaImage::from_pixel(2, 2, Rgba([1, 2, 3, 255]));
        let b = RgbaImage::from_pixel(2, 2, Rgba([4, 5, 6, 255]));
        let c = RgbaImage::from_pixel(2, 2, Rgba([7, 8, 9, 255]));
        let d = RgbaImage::from_pixel(2, 2, Rgba([10, 11, 12, 255]));
        capture_art(&cache, &[&a, &b, &c]).unwrap();
        time.set(1);
        capture_art(&cache, &[&b]).unwrap();
        time.set(2);
        capture_art(&cache, &[&c]).unwrap();
        time.set(40);
        // A is the oldest pose but is used early in this new capture: protect
        // it immediately. C is the previous published pose, also protected.
        let list = capture_art(&cache, &[&a, &d]).unwrap();
        assert_eq!(list.evicted, vec![texture_id(&b)]);
        for image in [&a, &c, &d] {
            assert!(cache
                .root
                .join(format!("{}.png", texture_id(image)))
                .is_file());
        }
        assert!(!cache.root.join(format!("{}.png", texture_id(&b))).exists());
        let bytes: u64 = fs::read_dir(&cache.root)
            .unwrap()
            .map(|p| p.unwrap().metadata().unwrap().len())
            .sum();
        assert_eq!(bytes, cache.cache.borrow().encoded_bytes);
        assert_eq!(cache.cache.borrow().len(), 3);
        // Replaying retained/current artwork stays pixel-identical after LRU.
        let mut expected = RgbaImage::new(32, 32);
        overlay(&mut expected, &a, 0, 0);
        overlay(&mut expected, &d, 3, 0);
        let mut actual = RgbaImage::new(32, 32);
        replay(&list.draws, &cache.root, &mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn lru_keeps_inflight_grace_then_reuses_space_across_many_rematches() {
        use std::cell::Cell;
        let cache = TestCache::new();
        let time = Rc::new(Cell::new(0u64));
        let timer = time.clone();
        {
            let mut c = cache.cache.borrow_mut();
            c.clock = Rc::new(move || Duration::from_secs(timer.get()));
            c.max_textures = 2;
        }
        let a = RgbaImage::from_pixel(2, 2, Rgba([1, 2, 3, 255]));
        let b = RgbaImage::from_pixel(2, 2, Rgba([4, 5, 6, 255]));
        let c = RgbaImage::from_pixel(2, 2, Rgba([7, 8, 9, 255]));
        capture_art(&cache, &[&a]).unwrap();
        capture_art(&cache, &[&b]).unwrap();
        time.set(29);
        assert!(capture_art(&cache, &[&b, &c])
            .err()
            .unwrap()
            .contains("30s in-flight grace"));
        assert!(cache.root.join(format!("{}.png", texture_id(&a))).is_file());
        time.set(31);
        let packet = capture_art(&cache, &[&b, &c]).unwrap();
        assert_eq!(packet.evicted, vec![texture_id(&a)]);
        // Retire that two-texture published scene before beginning rematches;
        // a genuinely full current frame may not be evicted to fit a new one.
        capture_art(&cache, &[]).unwrap();
        // Rematches never accumulate an unbounded process-lifetime pose set.
        for round in 1..=50 {
            time.set(31 + round * 31);
            let next = RgbaImage::from_pixel(2, 2, Rgba([round as u8, 24, 90, 255]));
            capture_art(&cache, &[&next]).unwrap();
            assert!(cache.cache.borrow().len() <= 2);
            assert!(fs::read_dir(&cache.root).unwrap().count() <= 2);
        }
    }

    #[test]
    fn lru_byte_budget_and_restarted_cache_account_existing_pngs() {
        use std::cell::Cell;
        let cache = TestCache::new();
        let time = Rc::new(Cell::new(0u64));
        let timer = time.clone();
        cache.cache.borrow_mut().clock = Rc::new(move || Duration::from_secs(timer.get()));
        let a = RgbaImage::from_pixel(2, 2, Rgba([1, 2, 3, 255]));
        let b = RgbaImage::from_pixel(2, 2, Rgba([4, 5, 6, 255]));
        let c = RgbaImage::from_pixel(2, 2, Rgba([7, 8, 9, 255]));
        capture_art(&cache, &[&a]).unwrap();
        capture_art(&cache, &[&b]).unwrap();
        let bytes = cache.cache.borrow().encoded_bytes;
        cache.cache.borrow_mut().max_bytes = bytes;
        time.set(31);
        capture_art(&cache, &[&c]).unwrap();
        assert!(cache.cache.borrow().encoded_bytes <= bytes);
        let restarted = TextureCache::new(&cache.root).unwrap();
        assert_eq!(restarted.encoded_bytes, cache.cache.borrow().encoded_bytes);
        assert_eq!(restarted.len(), cache.cache.borrow().len());
    }

    #[test]
    fn original_bundle_capture_preserves_art_hud_and_scene_pixels() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let mut painter = Painter::load(&root, 540, 900).expect("original asset bundle");
        let mut entities = Vec::new();
        for (side, ys) in [(0, [3000, 6500]), (1, [29000, 25500])] {
            for (row, x, y) in [
                ("KingTower", 9000, ys[0]),
                ("PrincessTower", 3500, ys[1]),
                ("PrincessTower", 14500, ys[1]),
            ] {
                entities.push(Entity {
                    address: entities.len() as u64 + 100,
                    row: row.into(),
                    side,
                    x,
                    y,
                    hp: 2400,
                    max_hp: 2400,
                    ..Entity::default()
                });
            }
        }
        let frame = BattleFrame {
            tick: 100,
            elapsed_ms: 5000,
            terminated: false,
            winner_side: None,
            arena_size: [18000, 32000],
            elixir: [7.78, 5.],
            entities,
            hands: [
                vec![HandCard {
                    slot: 0,
                    card_id: 26000021,
                    name: "HogRider".into(),
                    cost: 4,
                    available: true,
                }],
                Vec::new(),
            ],
            controller_names: ["Player".into(), "Original AI".into()],
            plays: Vec::new(),
            match_id: 3,
            human: Some(HumanFrame::default()),
        };
        painter.observe(&frame);
        let expected = painter.draw(100.).unwrap();
        let cache = TestCache::new();
        let capture = CaptureGuard::begin(cache.cache.clone()).unwrap();
        let blank = painter.draw(100.).unwrap();
        let list = capture.finish().unwrap();
        assert!(painter.unresolved_rows.is_empty());
        assert!(list.textures.len() > 10);
        assert!(list.draws.iter().any(|d| matches!(d, Draw::Shadow { .. })));
        assert!(
            list.textures
                .iter()
                .all(|t| (t.width, t.height) != (540, 900)),
            "full-frame raster is forbidden"
        );
        assert!(
            blank.pixels().all(|p| p.0 == [19, 27, 31, 255]),
            "root draws must all be captured"
        );
        let mut actual = blank;
        replay(&list.draws, &cache.root, &mut actual);
        let sum: u64 = actual
            .as_raw()
            .iter()
            .zip(expected.as_raw())
            .map(|(a, b)| a.abs_diff(*b) as u64)
            .sum();
        let mean = sum as f64 / actual.as_raw().len() as f64;
        eprintln!(
            "original scene capture: {} textures / {} draws, mean channel error {mean:.6}",
            list.textures.len(),
            list.draws.len()
        );
        // Original imageops::overlay and blit use slightly different integer
        // rounding; WebGL blending has the same sub-byte implementation detail.
        assert!(mean < 0.05, "original art/HUD parity error {mean}");
    }
}
