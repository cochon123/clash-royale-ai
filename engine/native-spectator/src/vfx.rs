//! Presentation-only playback of the original legacy effects.
//!
//! Effects (`legacy_vfx/effects.json`) list SWF clips, particle emitters and
//! nested effects per layer; emitters (`legacy_vfx/emitters.json`) carry the
//! particle parameters. The original client simulation code is not recovered,
//! so the integration below is an interpretation of those fields: times in
//! ms, distances in SC pixels, speeds per second, `ParticleCount` as the live
//! particle budget and `Inertia` as velocity damping. Nothing here feeds back
//! into the engine.
use crate::blit::{self, Affine, Blend, ShadowMask};
use crate::presentation::{EffectDef, EmitterDef, Presentation};
use clash_native_ui_viewer::{Playback, Scene};
use image::RgbaImage;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Layer {
    Ground,
    Shadow,
    Base,
    Object,
    Above,
}

impl Layer {
    /// Ground decals, shadows and debris below `Object` are emitted on the
    /// ground under their parent, even while the parent is airborne.
    fn on_ground(self) -> bool {
        matches!(self, Self::Ground | Self::Shadow | Self::Base)
    }

    fn parse(value: Option<&str>) -> Self {
        match value.unwrap_or("Object") {
            "Ground" => Self::Ground,
            "Shadow" => Self::Shadow,
            "Base" => Self::Base,
            "Above" | "Top" => Self::Above,
            _ => Self::Object,
        }
    }
}

/// World anchor: native x/y and elevation in SC pixels.
#[derive(Clone, Copy, Debug, Default)]
pub struct Anchor {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

struct Emitter {
    def: Rc<EmitterDef>,
    layer: Layer,
    anchor: Anchor,
    attach: Option<u64>,
    start_ms: f64,
    end_ms: f64,
    next_ms: f64,
    budget: usize,
    emitted_index: usize,
    live: usize,
    scale: f64,
    heading_deg: f64,
    repeat: bool,
    id: u64,
}

struct Particle {
    emitter: u64,
    doc: String,
    export: String,
    layer: Layer,
    anchor: Anchor,
    born_ms: f64,
    life_ms: f64,
    pos: [f64; 3],
    vel: [f64; 3],
    gravity: f64,
    damping: f64,
    scale: [f64; 2],
    fade: [f64; 2],
    rotation: f64,
    spin: f64,
    rotate_to_direction: bool,
    loop_clip: bool,
    ground_collide: bool,
    bounce: GroundContact,
}

#[derive(Clone, Copy)]
enum GroundContact {
    Bounce,
    Settle,
    Stop,
}

struct Clip {
    doc: String,
    export: String,
    layer: Layer,
    anchor: Anchor,
    attach: Option<u64>,
    start_ms: f64,
    scale: f64,
    looping: bool,
}

struct PendingEffect {
    name: String,
    anchor: Anchor,
    attach: Option<u64>,
    red: bool,
    scale: f64,
    start_ms: f64,
    heading_deg: f64,
    depth: u8,
    layer_override: Option<Layer>,
}

/// One sprite to composite. `offset` is in SC pixels on the screen plane
/// (x right, y down), already including elevation.
pub struct VfxSprite {
    pub layer: Layer,
    pub anchor: Anchor,
    pub offset: [f64; 2],
    pub sort_y: f64,
    pub doc: String,
    pub export: String,
    pub frame: usize,
    pub scale: f64,
    pub rotation: f64,
    pub opacity: f32,
}

/// Frame count, frames per second and compositing mode of an export.
pub type ClipMeta = (usize, u32, Blend);

pub struct CachedSprite {
    pub image: RgbaImage,
    pub x: f64,
    pub y: f64,
    pub blend: Blend,
}

#[derive(Default)]
pub struct SpriteCache {
    entries: BTreeMap<(String, String, usize), Option<Rc<CachedSprite>>>,
    meta: BTreeMap<(String, String), Option<ClipMeta>>,
}

impl SpriteCache {
    /// Frame count, fps and the compositing mode implied by the root
    /// children's SC blend modes. A clip rendered alone onto transparency
    /// keeps additive (8) or screen (4) children premultiplied.
    pub fn meta(
        &mut self,
        scenes: &BTreeMap<String, Scene>,
        doc: &str,
        export: &str,
    ) -> Option<ClipMeta> {
        let key = (doc.to_owned(), export.to_owned());
        if let Some(meta) = self.meta.get(&key) {
            return *meta;
        }
        let meta = scenes.get(doc).and_then(|scene| {
            let id = scene.document.exports.get(export)?;
            Some(match scene.document.movies.get(id) {
                Some(movie) => {
                    let all = |mode: u8| {
                        !movie.children_blending.is_empty()
                            && movie.children_blending.iter().all(|b| *b == mode)
                    };
                    (
                        movie.frame_element_counts.len().max(1),
                        movie.framerate.max(1),
                        if all(8) {
                            Blend::PremultipliedAdditive
                        } else if all(4) {
                            Blend::PremultipliedScreen
                        } else {
                            Blend::Normal
                        },
                    )
                }
                None => (1, 30, Blend::Normal),
            })
        });
        self.meta.insert(key, meta);
        meta
    }

    pub fn get(
        &mut self,
        scenes: &BTreeMap<String, Scene>,
        doc: &str,
        export: &str,
        frame: usize,
    ) -> Option<Rc<CachedSprite>> {
        let key = (doc.to_owned(), export.to_owned(), frame);
        if let Some(entry) = self.entries.get(&key) {
            return entry.clone();
        }
        let (count, _, blend) = self.meta(scenes, doc, export)?;
        let rendered = scenes.get(doc).and_then(|scene| {
            scene
                .render(
                    export,
                    &Playback {
                        frame: Some(frame.min(count - 1)),
                        ..Playback::default()
                    },
                )
                .ok()
                .flatten()
        });
        let entry = rendered.map(|r| {
            Rc::new(CachedSprite {
                image: r.image,
                x: r.x,
                y: r.y,
                blend,
            })
        });
        if self.entries.len() > 6000 {
            self.entries.clear();
        }
        self.entries.insert(key, entry.clone());
        entry
    }
}

pub struct Vfx {
    emitters: Vec<Emitter>,
    particles: Vec<Particle>,
    clips: Vec<Clip>,
    pending: Vec<PendingEffect>,
    emitter_defs: BTreeMap<String, Rc<EmitterDef>>,
    attachments: BTreeMap<u64, Option<Anchor>>,
    now_ms: f64,
    rng: u64,
    next_id: u64,
    pub cache: SpriteCache,
}

const MAX_PARTICLES: usize = 1600;
/// Presentation choice: particle heights are drawn at this fraction so
/// high-velocity debris stays near its source, as in the original client.
const Z_FORESHORTEN: f64 = 0.65;

impl Default for Vfx {
    fn default() -> Self {
        Self {
            emitters: Vec::new(),
            particles: Vec::new(),
            clips: Vec::new(),
            pending: Vec::new(),
            emitter_defs: BTreeMap::new(),
            attachments: BTreeMap::new(),
            now_ms: f64::NAN,
            rng: 0x9e37_79b9_7f4a_7c15,
            next_id: 1,
            cache: SpriteCache::default(),
        }
    }
}

impl Vfx {
    pub fn clear(&mut self) {
        self.emitters.clear();
        self.particles.clear();
        self.clips.clear();
        self.pending.clear();
        self.attachments.clear();
        self.now_ms = f64::NAN;
    }

    fn random(&mut self) -> f64 {
        next_random(&mut self.rng)
    }

    fn range(&mut self, lo: Option<f64>, hi: Option<f64>) -> f64 {
        let (lo, hi) = match (lo, hi) {
            (Some(lo), Some(hi)) => (lo, hi),
            (Some(v), None) | (None, Some(v)) => (v.min(0.), v.max(0.)),
            (None, None) => return 0.,
        };
        lo + (hi - lo) * self.random()
    }

    /// Queue an original effect by name. `red` selects its `EnemyVersion`.
    #[allow(clippy::too_many_arguments)]
    pub fn spawn(
        &mut self,
        name: &str,
        anchor: Anchor,
        red: bool,
        scale: f64,
        start_ms: f64,
        heading_deg: f64,
        attach: Option<u64>,
    ) {
        if name.is_empty() {
            return;
        }
        self.pending.push(PendingEffect {
            name: name.to_owned(),
            anchor,
            attach,
            red,
            scale,
            start_ms,
            heading_deg,
            depth: 0,
            layer_override: None,
        });
        if let Some(id) = attach {
            self.attachments.entry(id).or_insert(Some(anchor));
        }
    }

    /// Update or detach (`None`) the parent anchor of following effects.
    pub fn attach(&mut self, id: u64, anchor: Option<Anchor>) {
        if let Some(slot) = self.attachments.get_mut(&id) {
            *slot = anchor;
        }
    }

    fn resolve(&mut self, defs: &Presentation, effect: PendingEffect) {
        let Some(def) = Self::effect_def(defs, &effect.name, effect.red) else {
            return;
        };
        let def = def.clone();
        // Most effects give Scale as a percentage; muzzle flashes such as
        // cannon_fire_projectile use 1, which only makes sense as 1x.
        let percent = match def.scale {
            Some(s) if s > 10. => s,
            Some(s) if s > 0. => s * 100.,
            _ => 100.,
        };
        let base_scale = effect.scale * percent / 100.;
        for component in &def.renderable_components {
            let delay = component.time.unwrap_or(0.).max(0.);
            let start_ms = effect.start_ms + delay;
            let scale = base_scale * component.renderable_scale.unwrap_or(100.) / 100.;
            let layer = effect
                .layer_override
                .unwrap_or_else(|| Layer::parse(component.layer.as_deref()));
            match component.r#type.as_str() {
                "ParticleEmitter" => {
                    let Some(name) = component.particle_emitter_name.as_deref() else {
                        continue;
                    };
                    let emitter = match self.emitter_defs.get(name) {
                        Some(e) => e.clone(),
                        None => {
                            let Some(e) = defs.emitters.get(name) else {
                                continue;
                            };
                            let e = Rc::new(e.clone());
                            self.emitter_defs.insert(name.to_owned(), e.clone());
                            e
                        }
                    };
                    let life = self.range(emitter.min_life, emitter.max_life);
                    let id = self.next_id;
                    self.next_id += 1;
                    self.emitters.push(Emitter {
                        budget: emitter.particle_count.unwrap_or(1.).max(1.) as usize,
                        end_ms: start_ms + life,
                        next_ms: start_ms,
                        emitted_index: 0,
                        live: 0,
                        def: emitter,
                        layer,
                        anchor: effect.anchor,
                        attach: effect.attach,
                        start_ms,
                        scale,
                        heading_deg: effect.heading_deg,
                        repeat: def.looping && effect.attach.is_some(),
                        id,
                    });
                }
                "SWF" => {
                    let (Some(file), Some(export)) = (
                        component.file_name.as_deref(),
                        component.export_name.as_deref(),
                    ) else {
                        continue;
                    };
                    let doc = std::path::Path::new(file)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default()
                        .to_owned();
                    self.clips.push(Clip {
                        doc,
                        export: export.to_owned(),
                        layer,
                        anchor: effect.anchor,
                        attach: effect.attach,
                        start_ms,
                        scale,
                        looping: def.looping && effect.attach.is_some(),
                    });
                }
                "Effect" if effect.depth < 6 => {
                    if let Some(name) = component.effect.as_deref() {
                        self.pending.push(PendingEffect {
                            name: name.to_owned(),
                            start_ms,
                            scale,
                            depth: effect.depth + 1,
                            ..effect_clone(&effect)
                        });
                    }
                }
                _ => {}
            }
        }
    }

    fn effect_def<'a>(defs: &'a Presentation, name: &str, red: bool) -> Option<&'a EffectDef> {
        let def = defs.effects.get(name)?;
        if red {
            if let Some(enemy) = def.enemy_version.as_deref() {
                if let Some(enemy_def) = defs.effects.get(enemy) {
                    return Some(enemy_def);
                }
            }
        }
        Some(def)
    }

    /// Advance the simulation to `now_ms` (presentation clock, game ms).
    pub fn update(&mut self, defs: &Presentation, now_ms: f64) {
        while let Some(effect) = self.pending.pop() {
            self.resolve(defs, effect);
        }
        if self.now_ms.is_nan() || now_ms < self.now_ms - 1. {
            self.now_ms = now_ms;
        }
        // Refresh attached anchors.
        for emitter in &mut self.emitters {
            if let Some(Some(anchor)) = emitter.attach.and_then(|id| self.attachments.get(&id)) {
                emitter.anchor = *anchor;
            }
        }
        for clip in &mut self.clips {
            if let Some(Some(anchor)) = clip.attach.and_then(|id| self.attachments.get(&id)) {
                clip.anchor = *anchor;
            }
        }
        let mut t = self.now_ms;
        while t < now_ms {
            let dt = (now_ms - t).min(16.);
            t += dt;
            self.emit_until(t);
            self.integrate(t, dt / 1000.);
        }
        self.now_ms = now_ms;
        let attachments = &self.attachments;
        let alive = |attach: Option<u64>| {
            attach.is_none_or(|id| attachments.get(&id).is_some_and(Option::is_some))
        };
        self.emitters.retain(|e| {
            if now_ms < e.start_ms {
                return true;
            }
            if e.repeat {
                return alive(e.attach);
            }
            e.emitted_index < e.budget && now_ms <= e.end_ms.max(e.start_ms + 1.)
        });
        let cache = &mut self.cache;
        self.clips.retain(|c| {
            if now_ms < c.start_ms {
                return true;
            }
            if c.looping {
                return alive(c.attach);
            }
            match cache.meta_cached(&c.doc, &c.export) {
                Some((count, fps, _)) => (now_ms - c.start_ms) * fps as f64 / 1000. < count as f64,
                None => (now_ms - c.start_ms) < 3000.,
            }
        });
        self.attachments.retain(|_, a| a.is_some());
    }

    fn emit_until(&mut self, t: f64) {
        let mut rng = self.rng;
        let mut spawns = Vec::new();
        for (index, e) in self.emitters.iter_mut().enumerate() {
            if t < e.start_ms {
                continue;
            }
            let def = &e.def;
            let has_interval =
                def.particle_min_interval.is_some() || def.particle_max_interval.is_some();
            while e.next_ms <= t
                && e.live < e.budget
                && (e.repeat || e.emitted_index < e.budget)
                && (e.repeat || !has_interval || e.emitted_index == 0 || e.next_ms <= e.end_ms)
            {
                spawns.push((index, e.next_ms));
                e.emitted_index += 1;
                e.live += 1;
                if has_interval {
                    let lo = def.particle_min_interval.unwrap_or(0.);
                    let hi = def.particle_max_interval.unwrap_or(lo);
                    e.next_ms += (lo + (hi - lo) * next_random(&mut rng)).max(1.);
                }
            }
        }
        self.rng = rng;
        for (index, born) in spawns {
            if self.particles.len() >= MAX_PARTICLES {
                self.emitters[index].live -= 1;
                continue;
            }
            let def = self.emitters[index].def.clone();
            self.spawn_particle(index, born, &def);
        }
    }

    fn spawn_particle(&mut self, index: usize, born_ms: f64, def: &EmitterDef) {
        let (layer, anchor, scale, heading, id, n, budget) = {
            let e = &self.emitters[index];
            (
                e.layer,
                e.anchor,
                e.scale,
                e.heading_deg,
                e.id,
                e.emitted_index,
                e.budget,
            )
        };
        let choices = def
            .particle_export_name
            .len()
            .min(def.particle_resource.len());
        if choices == 0 {
            self.emitters[index].live -= 1;
            return;
        }
        let pick = ((self.random() * choices as f64) as usize).min(choices - 1);
        let doc = std::path::Path::new(&def.particle_resource[pick])
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_owned();
        let export = def.particle_export_name[pick].clone();
        let min_angle = def.particle_min_angle.unwrap_or(0.);
        let max_angle = def.particle_max_angle.unwrap_or(min_angle);
        let mut angle = if def.particle_random_angle {
            min_angle + (max_angle - min_angle) * self.random()
        } else {
            min_angle
                + (max_angle - min_angle) * (n.saturating_sub(1)) as f64 / budget.max(1) as f64
        };
        angle += heading;
        let (sin, cos) = angle.to_radians().sin_cos();
        let dir = [sin, -cos];
        let radius = self.range(def.particle_min_radius, def.particle_max_radius);
        let aspect = def.particle_radius_aspect.unwrap_or(100.) / 100.;
        let mut pos = [
            dir[0] * radius + def.particle_start_x.unwrap_or(0.),
            dir[1] * radius * aspect,
            def.particle_start_z.unwrap_or(0.),
        ];
        if let Some(area) = def.particle_start_xy_area_radius {
            let r = area * self.random().sqrt();
            let a = self.random() * std::f64::consts::TAU;
            pos[0] += r * a.cos();
            pos[1] += r * a.sin();
        }
        let speed = self.range(def.particle_min_speed, def.particle_max_speed);
        let vz = self.range(def.particle_min_velocity_z, def.particle_max_velocity_z);
        let life = self
            .range(def.particle_min_life, def.particle_max_life)
            .max(16.);
        let random_scale = def
            .random_scale
            .map(|r| 1. - self.random() * r.clamp(0., 100.) / 100.)
            .unwrap_or(1.);
        let spin = if def.rotate_min_speed.is_some() || def.rotate_max_speed.is_some() {
            let s = self.range(def.rotate_min_speed, def.rotate_max_speed);
            if self.random() < 0.5 {
                -s
            } else {
                s
            }
        } else {
            0.
        };
        let rotation = if spin != 0. { self.random() * 360. } else { 0. };
        let gravity = def.particle_gravity.unwrap_or(0.);
        self.particles.push(Particle {
            emitter: id,
            doc,
            export,
            layer,
            anchor: Anchor { z: 0., ..anchor },
            born_ms,
            life_ms: life,
            pos: [
                pos[0] * scale,
                pos[1] * scale,
                pos[2] * scale + if layer.on_ground() { 0. } else { anchor.z },
            ],
            vel: [dir[0] * speed * scale, dir[1] * speed * scale, vz * scale],
            gravity: gravity * scale,
            damping: def.inertia.unwrap_or(0.) / 10.,
            scale: [
                def.start_scale.unwrap_or(100.) / 100. * scale * random_scale,
                def.end_scale.unwrap_or(def.start_scale.unwrap_or(100.)) / 100.
                    * scale
                    * random_scale,
            ],
            fade: [
                def.fade_in_duration.unwrap_or(0.),
                def.fade_out_duration.unwrap_or(0.),
            ],
            rotation,
            spin,
            rotate_to_direction: def.rotate_to_direction,
            loop_clip: def.loop_particle_clip,
            ground_collide: layer != Layer::Shadow && gravity < 0.,
            bounce: if def.stop_on_bounce {
                GroundContact::Stop
            } else if def.no_bounce {
                GroundContact::Settle
            } else {
                GroundContact::Bounce
            },
        });
    }

    fn integrate(&mut self, t: f64, dt: f64) {
        let mut finished = Vec::new();
        self.particles.retain_mut(|p| {
            if t < p.born_ms {
                return true;
            }
            if t - p.born_ms > p.life_ms {
                finished.push(p.emitter);
                return false;
            }
            let damping = (-p.damping * dt).exp();
            p.vel[0] *= damping;
            p.vel[1] *= damping;
            p.vel[2] = p.vel[2] * damping + p.gravity * dt;
            for axis in 0..3 {
                p.pos[axis] += p.vel[axis] * dt;
            }
            if p.ground_collide && p.pos[2] < 0. {
                p.pos[2] = 0.;
                match p.bounce {
                    GroundContact::Bounce if p.vel[2].abs() > 40. => {
                        p.vel[2] = -p.vel[2] * 0.35;
                        p.vel[0] *= 0.6;
                        p.vel[1] *= 0.6;
                    }
                    GroundContact::Stop => p.vel = [0.; 3],
                    _ => {
                        p.vel[2] = 0.;
                        p.vel[0] *= 0.5;
                        p.vel[1] *= 0.5;
                        p.gravity = 0.;
                    }
                }
            }
            p.rotation += p.spin * dt;
            true
        });
        // Repeating (trail) emitters refill their live budget as particles expire.
        for id in finished {
            if let Some(e) = self.emitters.iter_mut().find(|e| e.id == id) {
                e.live = e.live.saturating_sub(1);
                if e.next_ms < t {
                    e.next_ms = t;
                }
            }
        }
    }

    /// Visible sprites at the current presentation time.
    pub fn sprites(&mut self, scenes: &BTreeMap<String, Scene>) -> Vec<VfxSprite> {
        let now = self.now_ms;
        let mut out = Vec::with_capacity(self.particles.len() + self.clips.len());
        for p in &self.particles {
            let age = now - p.born_ms;
            if age < 0. {
                continue;
            }
            let Some((count, fps, _)) = self.cache.meta(scenes, &p.doc, &p.export) else {
                continue;
            };
            let f = (age / p.life_ms).clamp(0., 1.);
            let mut opacity = 1.;
            if p.fade[0] > 0. && age < p.fade[0] {
                opacity *= age / p.fade[0];
            }
            let remaining = p.life_ms - age;
            if p.fade[1] > 0. && remaining < p.fade[1] {
                opacity *= remaining / p.fade[1];
            }
            let raw = (age * fps as f64 / 1000.) as usize;
            let frame = if p.loop_clip {
                raw % count
            } else {
                raw.min(count - 1)
            };
            let rotation = if p.rotate_to_direction {
                let (vx, vy) = (p.vel[0], p.vel[1] - p.vel[2]);
                if vx.abs() + vy.abs() > 1. {
                    vy.atan2(vx)
                } else {
                    0.
                }
            } else {
                p.rotation.to_radians()
            };
            // Elevation is foreshortened by the tilted battle camera. Shadow
            // emitters mirror their smoke with negative z in the source data.
            let z = p.pos[2] * Z_FORESHORTEN;
            out.push(VfxSprite {
                layer: p.layer,
                anchor: p.anchor,
                offset: [p.pos[0], p.pos[1] - z],
                sort_y: p.pos[1],
                doc: p.doc.clone(),
                export: p.export.clone(),
                frame,
                scale: p.scale[0] + (p.scale[1] - p.scale[0]) * f,
                rotation,
                opacity: opacity as f32,
            });
        }
        for c in &self.clips {
            let age = now - c.start_ms;
            if age < 0. {
                continue;
            }
            let Some((count, fps, _)) = self.cache.meta(scenes, &c.doc, &c.export) else {
                continue;
            };
            let raw = (age * fps as f64 / 1000.) as usize;
            let frame = if c.looping {
                raw % count
            } else if raw >= count {
                continue;
            } else {
                raw
            };
            out.push(VfxSprite {
                layer: c.layer,
                anchor: Anchor { z: 0., ..c.anchor },
                offset: [0., if c.layer.on_ground() { 0. } else { -c.anchor.z }],
                sort_y: 0.,
                doc: c.doc.clone(),
                export: c.export.clone(),
                frame,
                scale: c.scale,
                rotation: 0.,
                opacity: 1.,
            });
        }
        out
    }

    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }
}

impl SpriteCache {
    fn meta_cached(&self, doc: &str, export: &str) -> Option<ClipMeta> {
        self.meta
            .get(&(doc.to_owned(), export.to_owned()))
            .copied()
            .flatten()
    }
}

fn next_random(state: &mut u64) -> f64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    (*state >> 11) as f64 / (1u64 << 53) as f64
}

fn effect_clone(effect: &PendingEffect) -> PendingEffect {
    PendingEffect {
        name: effect.name.clone(),
        anchor: effect.anchor,
        attach: effect.attach,
        red: effect.red,
        scale: effect.scale,
        start_ms: effect.start_ms,
        heading_deg: effect.heading_deg,
        depth: effect.depth,
        layer_override: effect.layer_override,
    }
}

/// Composite one effect sprite at screen position `(sx, sy)` of its anchor.
#[allow(clippy::too_many_arguments)]
pub fn draw_sprite(
    out: &mut RgbaImage,
    shadows: &mut ShadowMask,
    cache: &mut SpriteCache,
    scenes: &BTreeMap<String, Scene>,
    sprite: &VfxSprite,
    sx: f64,
    sy: f64,
    field_scale: f64,
) {
    let Some(image) = cache.get(scenes, &sprite.doc, &sprite.export, sprite.frame) else {
        return;
    };
    let scale = sprite.scale * field_scale;
    if scale <= 0.01 {
        return;
    }
    let t = Affine::scale_rotate(
        scale,
        sprite.rotation,
        sx + sprite.offset[0] * field_scale,
        sy + sprite.offset[1] * field_scale,
    );
    if sprite.layer == Layer::Shadow {
        shadows.add_silhouette(&image.image, image.x, image.y, &t, 0.45 * sprite.opacity);
        return;
    }
    blit::blit_affine(
        out,
        &image.image,
        image.x,
        image.y,
        &t,
        sprite.opacity,
        image.blend,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;
    use std::path::Path;

    fn bundle() -> (Presentation, BTreeMap<String, Scene>) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let defs = Presentation::load(&root).unwrap();
        let mut scenes = BTreeMap::new();
        scenes.insert("effects".to_owned(), Scene::load(&root, "effects").unwrap());
        (defs, scenes)
    }

    #[test]
    fn death_effect_emits_original_particles_that_expire() {
        let (defs, scenes) = bundle();
        if defs.effects.is_empty() {
            eprintln!("presentation.json absent; skipping");
            return;
        }
        let mut vfx = Vfx::default();
        vfx.update(&defs, 0.);
        vfx.spawn("knight_die", Anchor::default(), false, 1., 0., 0., None);
        vfx.update(&defs, 100.);
        let early = vfx.sprites(&scenes);
        assert!(!early.is_empty(), "death effect produced no sprites");
        assert!(early.iter().all(|s| s.doc == "effects"));
        // The Knight's dropped sword lingers for 24-25 s in the source data.
        vfx.update(&defs, 10_000.);
        assert_eq!(vfx.particle_count(), 1);
        vfx.update(&defs, 26_000.);
        assert_eq!(vfx.particle_count(), 0, "particles outlive their life");
    }

    #[test]
    #[ignore = "writes an original-effect playback contact sheet to /tmp"]
    fn write_effect_contact_sheet() {
        let (defs, scenes) = bundle();
        let effects = [
            "Fireball_explosion",
            "cannon_hit",
            "musketeer_die",
            "skeleton_die",
            "snowman_deploy",
            "log_die",
            "TowerExplode",
            "King_player_die",
            "icespirit_projectile_hit",
            "Rocket_explosion",
            "mortar_hit",
            "ArrowHitGround",
        ];
        let times = [0., 80., 160., 300., 500., 800., 1200., 1800.];
        let cell = 150u32;
        let mut sheet = RgbaImage::from_pixel(
            cell * times.len() as u32,
            cell * effects.len() as u32,
            Rgba([96, 150, 72, 255]),
        );
        for (row, name) in effects.iter().enumerate() {
            let mut vfx = Vfx::default();
            vfx.update(&defs, 0.);
            vfx.spawn(name, Anchor::default(), false, 1., 0., 0., None);
            for (col, t) in times.iter().enumerate() {
                vfx.update(&defs, *t);
                let sprites = vfx.sprites(&scenes);
                let mut shadows = ShadowMask::new(sheet.width(), sheet.height());
                let (cx, cy) = (
                    col as f64 * cell as f64 + cell as f64 / 2.,
                    row as f64 * cell as f64 + cell as f64 * 0.62,
                );
                for layer in [
                    Layer::Ground,
                    Layer::Shadow,
                    Layer::Base,
                    Layer::Object,
                    Layer::Above,
                ] {
                    for sprite in sprites.iter().filter(|s| s.layer == layer) {
                        draw_sprite(
                            &mut sheet,
                            &mut shadows,
                            &mut vfx.cache,
                            &scenes,
                            sprite,
                            cx,
                            cy,
                            0.84,
                        );
                    }
                    if layer == Layer::Shadow {
                        shadows.apply(&mut sheet);
                    }
                }
                eprintln!(
                    "{name} t={t} particles={} sprites={}",
                    vfx.particle_count(),
                    sprites.len()
                );
            }
        }
        sheet.save("/tmp/native-spectator-effects.png").unwrap();
    }
}
