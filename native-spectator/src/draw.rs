//! Read-only battle renderer. All character, arena, and decoration artwork is
//! loaded from the original SC5 bundle; spectator-owned bars/text use a font
//! extracted from that same bundle.
use crate::{
    animation::{self, AnimationState, KingLayerOrder, PresentationHeading},
    frame::{BattleFrame, Entity},
    hud::HandHud,
};
use clash_native_ui_viewer::{Matrix, Playback, Rendered, Scene};
use image::{imageops, Rgba, RgbaImage};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, String>;

const BLUE: Rgba<u8> = Rgba([58, 147, 255, 255]);
const RED: Rgba<u8> = Rgba([238, 72, 65, 255]);
const DARK: Rgba<u8> = Rgba([29, 34, 42, 242]);
const WHITE: u32 = 0xffffffff;

#[derive(Clone)]
struct Visual {
    document: String,
    prefix: String,
    red_prefix: Option<String>,
    scale_percent: f64,
}

pub struct Painter {
    root: PathBuf,
    width: u32,
    height: u32,
    scenes: BTreeMap<String, Scene>,
    bindings: BTreeMap<String, Visual>,
    sprite_cache: BTreeMap<(String, String, usize, u64), Option<Rendered>>,
    deco_cache: BTreeMap<String, RgbaImage>,
    text_cache: BTreeMap<(String, u32, u32, u8), RgbaImage>,
    arena_field: Option<RgbaImage>,
    field_cache: Option<RgbaImage>,
    sprite_fit_cache: BTreeMap<(String, String, usize, u64, u64, bool), (RgbaImage, i64, i64)>,
    font_scene: String,
    pub unresolved_rows: BTreeSet<String>,
    assets: Value,
    hand_hud: HandHud,
    headings: BTreeMap<u64, PresentationHeading>,
    previous: BTreeMap<u64, Entity>,
    attack_cues: BTreeMap<u64, u32>,
    king_activations: BTreeMap<u64, u32>,
    state_clocks: BTreeMap<u64, (AnimationState, u32)>,
    last_match: Option<(u64, u32)>,
}

impl Painter {
    pub fn load(root: &Path, width: u32, height: u32) -> Result<Self> {
        let manifest: Value = serde_json::from_slice(
            &std::fs::read(root.join("assets.json")).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let mut scenes = BTreeMap::new();
        let mut exports = BTreeMap::new();
        if let Some(docs) = manifest["documents"].as_array() {
            for doc in docs {
                let Some(name) = doc["name"].as_str() else {
                    continue;
                };
                let scene = Scene::load(root, name).map_err(|e| e.to_string())?;
                let set: BTreeSet<String> = scene.document.exports.keys().cloned().collect();
                exports.insert(name.to_owned(), set);
                scenes.insert(name.to_owned(), scene);
            }
        }
        let mut bindings = BTreeMap::new();
        // Exact SC5 documents and export-family stems referenced by recovered
        // logic rows or observed cards. Export selection is checked against the
        // source export table at draw time.
        for (row, doc, prefix, red_prefix) in [
            (
                "KingTower",
                "building_tower",
                "KingTower_blue",
                Some("KingTower_red"),
            ),
            (
                "PrincessTower",
                "building_tower",
                "StarTower_base_blue",
                Some("StarTower_base_red"),
            ),
        ] {
            if exports
                .get(doc)
                .is_some_and(|s| s.iter().any(|e| e.starts_with(prefix)))
            {
                bindings.insert(
                    row.to_owned(),
                    Visual {
                        document: doc.into(),
                        prefix: prefix.into(),
                        red_prefix: red_prefix.map(str::to_owned),
                        scale_percent: 100.0,
                    },
                );
            }
        }
        let cards = manifest["scene"]["observed_decks"]["cards_observed_across_traces"]
            .as_array()
            .into_iter()
            .flatten()
            .chain(
                manifest["scene"]["fixed_training_deck_assets"]
                    .as_array()
                    .into_iter()
                    .flatten(),
            );
        for card in cards {
            let (Some(row), Some(doc)) =
                (card["native_card_row"].as_str(), card["document"].as_str())
            else {
                continue;
            };
            let blue = card["character_logic_binding"]["blue_export"]
                .as_str()
                .or_else(|| card["blue_export"].as_str())
                .or_else(|| {
                    card["export_prefixes"]
                        .as_array()
                        .and_then(|v| v.first())
                        .and_then(Value::as_str)
                });
            let red = card["character_logic_binding"]["red_export"]
                .as_str()
                .or_else(|| card["red_export"].as_str())
                .or_else(|| {
                    card["export_prefixes"]
                        .as_array()
                        .and_then(|v| v.get(1))
                        .and_then(Value::as_str)
                });
            let Some(prefix) = blue else { continue };
            if exports.get(doc).is_some_and(|set| {
                set.contains(prefix) || set.iter().any(|e| e.starts_with(prefix))
            }) {
                let visual = Visual {
                    document: doc.to_owned(),
                    prefix: prefix.to_owned(),
                    red_prefix: red.map(str::to_owned),
                    scale_percent: card["character_logic_binding"]["scale_percent"]
                        .as_f64()
                        .or_else(|| card["scale_percent"].as_f64())
                        .unwrap_or(100.0),
                };
                bindings.insert(row.to_owned(), visual.clone());
                if row.ends_with('s') {
                    bindings.insert(row[..row.len() - 1].to_owned(), visual);
                }
            }
        }
        // Live entity rows (including spawned units and projectile rows) are
        // included only when recorded frames can be joined to an original
        // nested logic row with a source SC document and export name.
        if let Some(rows) = manifest["scene"]["observed_entity_bindings"].as_array() {
            for row in rows {
                let (Some(name), Some(doc), Some(blue)) = (
                    row["native_row"].as_str(),
                    row["document"].as_str(),
                    row["blue_export"].as_str(),
                ) else {
                    continue;
                };
                if !exports.get(doc).is_some_and(|set| {
                    set.contains(blue) || set.iter().any(|e| e.starts_with(blue))
                }) {
                    continue;
                }
                bindings.insert(
                    name.to_owned(),
                    Visual {
                        document: doc.to_owned(),
                        prefix: blue.to_owned(),
                        red_prefix: row["red_export"].as_str().map(str::to_owned),
                        scale_percent: row["scale_percent"].as_f64().unwrap_or(100.0),
                    },
                );
            }
        }
        let font_scene = scenes
            .keys()
            .next()
            .cloned()
            .ok_or("asset bundle contains no recovered SC5 documents")?;
        let arena_field = if let Some(scene) = scenes.get("arena_training") {
            let rendered = scene
                .render("training_area_bg", &Playback::default())
                .map_err(|e| e.to_string())?
                .ok_or("training_area_bg export rendered no image")?;
            let crop_x = (-270.0 - rendered.x).round() as i64;
            let crop_y = (-60.0 - rendered.y).round() as i64;
            if crop_x < 0
                || crop_y < 0
                || crop_x + 540 > rendered.image.width() as i64
                || crop_y + 828 > rendered.image.height() as i64
            {
                return Err(format!("calibrated arena crop is outside original background bounds: origin ({}, {}), image {}x{}", rendered.x, rendered.y, rendered.image.width(), rendered.image.height()));
            }
            Some(
                imageops::crop_imm(&rendered.image, crop_x as u32, crop_y as u32, 540, 828)
                    .to_image(),
            )
        } else {
            None
        };
        Ok(Self {
            root: root.to_path_buf(),
            width: width.max(1),
            height: height.max(1),
            scenes,
            bindings,
            sprite_cache: BTreeMap::new(),
            deco_cache: BTreeMap::new(),
            text_cache: BTreeMap::new(),
            arena_field,
            field_cache: None,
            sprite_fit_cache: BTreeMap::new(),
            font_scene,
            unresolved_rows: BTreeSet::new(),
            assets: manifest,
            hand_hud: HandHud::default(),
            headings: BTreeMap::new(),
            previous: BTreeMap::new(),
            attack_cues: BTreeMap::new(),
            king_activations: BTreeMap::new(),
            state_clocks: BTreeMap::new(),
            last_match: None,
        })
    }

    pub fn render(&mut self, frame: &BattleFrame) -> Result<RgbaImage> {
        let mut out = RgbaImage::from_pixel(self.width, self.height, Rgba([19, 27, 31, 255]));
        self.observe_visual_events(frame);
        let hud = ((self.width as f32 * 0.235).round() as u32).min(self.height / 3);
        let field_h = self.height.saturating_sub(hud * 2).max(1);
        let field_w = ((field_h as f32 * 540.0 / 768.0).round() as u32)
            .min(self.width.saturating_sub(24).max(1));
        let field_x = (self.width - field_w) / 2;
        let field_y = hud;

        if self
            .field_cache
            .as_ref()
            .is_none_or(|f| f.dimensions() != (field_w, field_h))
        {
            let mut field_layer = RgbaImage::new(field_w, field_h);
            self.draw_arena(&mut field_layer, 0, 0, field_w, field_h)?;
            // Original location CSV props use their exact coordinates and export
            // identities under the documented asset-calibrated viewport.
            self.draw_decorations(&mut field_layer, 0, 0, field_w, field_h)?;
            self.field_cache = Some(field_layer);
        }
        imageops::overlay(
            &mut out,
            self.field_cache.as_ref().expect("field cache initialized"),
            field_x as i64,
            field_y as i64,
        );

        let world_w = frame.arena_size[0].max(1) as f64;
        let world_h = frame.arena_size[1].max(1) as f64;
        let mut entities: Vec<_> = frame.entities.iter().collect();
        entities.sort_by_key(|entity| -entity.y);
        for entity in entities {
            let px = field_x as f64 + ((entity.x as f64 / world_w) * field_w as f64);
            // Native side 0 is blue and uses the lower half of the board.
            let py = field_y as f64
                + (((1.0 - entity.y as f64 / world_h) * 768. + 60.) / 828. * field_h as f64);
            self.draw_entity(
                &mut out,
                frame,
                entity,
                px as i32,
                py as i32,
                field_w as f64 / 540.0,
            )?;
        }
        self.hand_hud
            .render(&mut out, &self.root, &self.scenes, &self.assets, frame, hud)?;
        let secs = frame.elapsed_ms / 1000;
        self.text(
            &mut out,
            &format!("{}:{:02}", secs / 60, secs % 60),
            field_x + field_w - 34,
            field_y + 6,
            15.,
            WHITE,
            1,
        )?;
        if frame.terminated {
            let msg = match frame.winner_side {
                Some(0) => "BLUE WINS",
                Some(1) => "RED WINS",
                _ => "DRAW",
            };
            self.text(
                &mut out,
                msg,
                self.width / 2,
                field_y + field_h / 2,
                26.,
                WHITE,
                1,
            )?;
        }
        self.previous = frame
            .entities
            .iter()
            .map(|e| (e.address, e.clone()))
            .collect();
        Ok(out)
    }

    fn draw_arena(&mut self, out: &mut RgbaImage, x: u32, y: u32, w: u32, h: u32) -> Result<()> {
        let Some(arena) = self.arena_field.as_ref() else {
            return Ok(());
        };
        let fitted = imageops::resize(arena, w, h, imageops::FilterType::Lanczos3);
        imageops::overlay(out, &fitted, x as i64, y as i64);
        Ok(())
    }

    fn draw_decorations(
        &mut self,
        out: &mut RgbaImage,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    ) -> Result<()> {
        let manifest: Value = serde_json::from_slice(
            &std::fs::read(self.root.join("assets.json")).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let Some(objects) = manifest["scene"]["objects"].as_array() else {
            return Ok(());
        };
        let mut drawn = 0usize;
        for object in objects {
            let Some(asset) = object.get("asset") else {
                continue;
            };
            let (Some(doc), Some(export)) = (asset["document"].as_str(), asset["export"].as_str())
            else {
                continue;
            };
            let locations = object["placements"].as_array().cloned().unwrap_or_default();
            for loc in locations {
                if drawn >= 240 {
                    return Ok(());
                }
                let (Some(px), Some(py)) = (loc["x"].as_f64(), loc["y"].as_f64()) else {
                    continue;
                };
                let px = x as f64 + px / 18000.0 * w as f64;
                let py = y as f64 + ((1.0 - py / 32000.) * 768. + 60.) / 828. * h as f64;
                if px < x as f64 || px > (x + w) as f64 || py < y as f64 || py > (y + h) as f64 {
                    continue;
                }
                let scale = asset["scale_x_percent"].as_f64().unwrap_or(95.0) / 100.0;
                let cache_key = format!("{doc}/{export}/{scale:.3}");
                if !self.deco_cache.contains_key(&cache_key) {
                    if let Some(scene) = self.scenes.get(doc) {
                        if let Some(image) = scene
                            .render(export, &Playback::default())
                            .map_err(|e| e.to_string())?
                        {
                            let target_w =
                                ((image.image.width() as f64 * 0.18 * scale).round() as u32).max(1);
                            let target_h = ((image.image.height() as f64 * 0.18 * scale).round()
                                as u32)
                                .max(1);
                            self.deco_cache.insert(
                                cache_key.clone(),
                                imageops::resize(
                                    &image.image,
                                    target_w,
                                    target_h,
                                    imageops::FilterType::Triangle,
                                ),
                            );
                        }
                    }
                }
                if let Some(sprite) = self.deco_cache.get(&cache_key) {
                    imageops::overlay(
                        out,
                        sprite,
                        px as i64 - sprite.width() as i64 / 2,
                        py as i64 - sprite.height() as i64 / 2,
                    );
                }
                drawn += 1;
            }
        }
        Ok(())
    }

    fn row_meta(&self, row: &str) -> Option<&Value> {
        self.assets["scene"]["observed_entity_bindings"]
            .as_array()?
            .iter()
            .find(|v| v["native_row"] == row)
    }
    fn is_projectile(&self, row: &str) -> bool {
        self.row_meta(row)
            .and_then(|v| v["source_row"].as_str())
            .is_some_and(|v| v.starts_with("PROJECTILE."))
            || self.assets["scene"]["projectile_bindings"]
                .as_array()
                .is_some_and(|v| v.iter().any(|v| v["native_row"] == row))
    }
    fn observe_visual_events(&mut self, frame: &BattleFrame) {
        if self.last_match == Some((frame.match_id, frame.tick)) {
            return;
        }
        if self
            .last_match
            .is_some_and(|(id, tick)| id != frame.match_id || tick > frame.tick)
        {
            self.headings.clear();
            self.previous.clear();
            self.attack_cues.clear();
            self.king_activations.clear();
            self.state_clocks.clear();
        }
        self.last_match = Some((frame.match_id, frame.tick));
        // The original tower logic activates on King damage or an allied
        // Princess tower's destruction. Trigger its source movie from those
        // observations; this is not a recovered native activation flag.
        let fallen_sides: BTreeSet<i32> = self
            .previous
            .values()
            .filter(|e| {
                e.row == "PrincessTower"
                    && !frame.entities.iter().any(|now| now.address == e.address)
            })
            .map(|e| e.side)
            .chain(
                frame
                    .entities
                    .iter()
                    .filter(|e| e.row == "PrincessTower" && e.hp == 0)
                    .map(|e| e.side),
            )
            .collect();
        for king in frame
            .entities
            .iter()
            .filter(|e| e.row == "KingTower" && e.max_hp > 0)
        {
            if king.hp < king.max_hp || fallen_sides.contains(&king.side) {
                self.king_activations
                    .entry(king.address)
                    .or_insert(frame.tick);
            }
        }
        // These are presentation cues from actual native observations. They
        // are never written back as engine attack flags or simulated damage.
        for projectile in frame
            .entities
            .iter()
            .filter(|e| self.is_projectile(&e.row) && !self.previous.contains_key(&e.address))
            .collect::<Vec<_>>()
        {
            let shooter = frame
                .entities
                .iter()
                .filter(|e| {
                    e.side == projectile.side && e.max_hp > 0 && e.native_focus_entity.is_some()
                })
                .filter_map(|e| {
                    let dx = e.x as i64 - projectile.x as i64;
                    let dy = e.y as i64 - projectile.y as i64;
                    let d = dx * dx + dy * dy;
                    (d < 2500 * 2500).then_some((d, e.address))
                })
                .min();
            if let Some((_, address)) = shooter {
                self.attack_cues.insert(address, frame.tick);
            }
        }
        for target in &frame.entities {
            if !self
                .previous
                .get(&target.address)
                .is_some_and(|old| old.hp > target.hp && target.hp >= 0)
            {
                continue;
            }
            for actor in &frame.entities {
                let dx = actor.x as i64 - target.x as i64;
                let dy = actor.y as i64 - target.y as i64;
                if actor.side != target.side
                    && !actor.moving
                    && actor.native_focus_entity == Some(target.address)
                    && dx * dx + dy * dy < 2500 * 2500
                {
                    self.attack_cues.insert(actor.address, frame.tick);
                }
            }
        }
        self.attack_cues
            .retain(|_, t| frame.tick.saturating_sub(*t) < 60);
        self.headings
            .retain(|id, _| frame.entities.iter().any(|e| e.address == *id));
        self.state_clocks
            .retain(|id, _| frame.entities.iter().any(|e| e.address == *id));
    }
    #[allow(clippy::too_many_arguments)]
    fn layer(
        &mut self,
        out: &mut RgbaImage,
        doc: &str,
        export: &str,
        playback: Playback,
        frame_index: usize,
        x: f64,
        y: f64,
        scale: f64,
        flip: bool,
    ) -> Result<f64> {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        playback.object_frames.hash(&mut hasher);
        playback.hidden_objects.hash(&mut hasher);
        playback.label.hash(&mut hasher);
        playback
            .root_transform
            .map(|m| {
                [
                    m.a.to_bits(),
                    m.b.to_bits(),
                    m.c.to_bits(),
                    m.d.to_bits(),
                    m.tx.to_bits(),
                    m.ty.to_bits(),
                ]
            })
            .hash(&mut hasher);
        playback
            .root_color
            .map(|c| {
                [
                    c.r_mul, c.g_mul, c.b_mul, c.alpha, c.r_add, c.g_add, c.b_add,
                ]
            })
            .hash(&mut hasher);
        let variant = hasher.finish();
        let key = (doc.to_owned(), export.to_owned(), frame_index, variant);
        if !self.sprite_cache.contains_key(&key) {
            let scene = self
                .scenes
                .get(doc)
                .ok_or_else(|| format!("original scene missing {doc}"))?;
            let rendered = if let Some(id) = export.strip_prefix('#') {
                scene.render_id(id.parse().map_err(|_| "object id")?, &playback)
            } else {
                scene.render(export, &playback)
            }
            .map_err(|e| e.to_string())?;
            if self.sprite_cache.len() > 900 {
                self.sprite_cache.clear();
                self.sprite_fit_cache.clear();
            }
            self.sprite_cache.insert(key.clone(), rendered);
        }
        let fit = (
            doc.to_owned(),
            export.to_owned(),
            frame_index,
            variant,
            scale.to_bits(),
            flip,
        );
        if !self.sprite_fit_cache.contains_key(&fit) {
            if let Some(Some(sprite)) = self.sprite_cache.get(&key) {
                let sw = (sprite.image.width() as f64 * scale).round().max(1.) as u32;
                let sh = (sprite.image.height() as f64 * scale).round().max(1.) as u32;
                let mut image =
                    imageops::resize(&sprite.image, sw, sh, imageops::FilterType::Triangle);
                let fx = if flip {
                    -(sprite.x + sprite.image.width() as f64) * scale
                } else {
                    sprite.x * scale
                };
                if flip {
                    image = imageops::flip_horizontal(&image);
                }
                self.sprite_fit_cache.insert(
                    fit.clone(),
                    (image, fx.round() as i64, (sprite.y * scale).round() as i64),
                );
            }
        }
        if let Some((image, fx, fy)) = self.sprite_fit_cache.get(&fit) {
            imageops::overlay(out, image, x.round() as i64 + *fx, y.round() as i64 + *fy);
            return Ok(y + *fy as f64);
        }
        Ok(y)
    }
    fn draw_entity(
        &mut self,
        out: &mut RgbaImage,
        frame: &BattleFrame,
        entity: &Entity,
        x: i32,
        y: i32,
        field_scale: f64,
    ) -> Result<()> {
        let Some(visual) = self.bindings.get(&entity.row).cloned() else {
            self.unresolved_rows.insert(entity.row.clone());
            if entity.max_hp > 0 {
                fill_circle(out, x, y, 5, if entity.side == 0 { BLUE } else { RED });
                self.text(
                    out,
                    &entity.row,
                    x.max(0) as u32,
                    y.max(0) as u32,
                    9.,
                    WHITE,
                    1,
                )?;
            }
            return Ok(());
        };
        let projectile = self.is_projectile(&entity.row);
        let old = self.previous.get(&entity.address);
        let motion_dz = old.map(|p| i64::from(entity.z) - i64::from(p.z));
        let motion = (
            entity
                .motion_dx
                .unwrap_or_else(|| old.map_or(0, |p| entity.x as i64 - p.x as i64)),
            entity
                .motion_dy
                .unwrap_or_else(|| old.map_or(0, |p| entity.y as i64 - p.y as i64)),
        );
        let target = entity
            .native_focus_entity
            .and_then(|p| frame.entities.iter().find(|e| e.address == p));
        let vector = if let Some(target) = target {
            if !entity.moving {
                (
                    target.x as i64 - entity.x as i64,
                    target.y as i64 - entity.y as i64,
                )
            } else {
                motion
            }
        } else {
            motion
        };
        let heading = animation::direction_for_motion(
            vector.0,
            vector.1,
            entity.side,
            self.headings.get(&entity.address).copied(),
        );
        self.headings.insert(entity.address, heading);
        let cue = self.attack_cues.get(&entity.address).copied();
        let visual_attack =
            entity.attacking || cue.is_some_and(|t| frame.tick.saturating_sub(t) < 16);
        let state = if visual_attack {
            AnimationState::Attacking
        } else if entity.moving {
            AnimationState::Moving
        } else {
            AnimationState::Idle
        };
        let clock = self
            .state_clocks
            .entry(entity.address)
            .or_insert((state, frame.tick));
        if clock.0 != state {
            *clock = (state, frame.tick);
        }
        let phase = frame
            .tick
            .saturating_sub(if state == AnimationState::Attacking {
                cue.unwrap_or(clock.1)
            } else {
                clock.1
            }) as u64
            * 50;
        let age = frame
            .tick
            .saturating_sub(entity.first_seen_tick.unwrap_or(clock.1)) as u64
            * 50;
        let scale = field_scale * visual.scale_percent / 100.;
        let red = entity.side != 0;
        // Source z is actual native elevation. Projection scale is the same
        // asset-calibrated camera convention as the arena plane.
        let py = y as f64 - entity.z as f64 * 0.03 * field_scale;
        let mut sprite_top = py - 40. * scale;
        if entity.hp == 0
            && entity.max_hp > 0
            && matches!(entity.row.as_str(), "KingTower" | "PrincessTower")
        {
            let export = if entity.row == "KingTower" {
                "kingtower_destroyed"
            } else {
                "princesstower_destroyed"
            };
            self.layer(
                out,
                "building_tower",
                export,
                Playback::default(),
                0,
                x as f64,
                py,
                scale,
                false,
            )?;
            return Ok(());
        }
        if entity.row == "KingTower" {
            let layers = animation::king_tower_layers_with_activation(
                self.scenes.get("building_tower").ok_or("tower scene")?,
                self.scenes.get("chr_king").ok_or("king scene")?,
                red,
                heading,
                phase,
                self.king_activations
                    .get(&entity.address)
                    .map(|tick| frame.tick.saturating_sub(*tick) as u64 * 50),
            )?;
            let root_frame = layers.root_playback.frame.unwrap_or(0);
            let king_frame = layers.king_playback.frame.unwrap_or(0);
            sprite_top = self.layer(
                out,
                "building_tower",
                layers.root_export,
                layers.root_playback,
                root_frame,
                x as f64,
                py,
                scale,
                false,
            )?;
            let turret_frame = layers.turret_playback.frame.unwrap_or(0);
            match layers.order {
                KingLayerOrder::TurretThenKing => {
                    self.layer(
                        out,
                        "building_tower",
                        &format!("#{}", layers.turret_id),
                        layers.turret_playback,
                        turret_frame,
                        x as f64 + layers.turret_offset[0] * scale,
                        py + layers.turret_offset[1] * scale,
                        scale,
                        layers.turret_flip_horizontal,
                    )?;
                    self.layer(
                        out,
                        "chr_king",
                        layers.king_export,
                        layers.king_playback,
                        king_frame,
                        x as f64,
                        py,
                        scale,
                        false,
                    )?;
                }
                KingLayerOrder::KingThenTurret => {
                    self.layer(
                        out,
                        "chr_king",
                        layers.king_export,
                        layers.king_playback,
                        king_frame,
                        x as f64,
                        py,
                        scale,
                        false,
                    )?;
                    self.layer(
                        out,
                        "building_tower",
                        &format!("#{}", layers.turret_id),
                        layers.turret_playback,
                        turret_frame,
                        x as f64 + layers.turret_offset[0] * scale,
                        py + layers.turret_offset[1] * scale,
                        scale,
                        layers.turret_flip_horizontal,
                    )?;
                }
            }
        } else if entity.row == "PrincessTower" {
            let base = if red {
                "StarTower_base_red"
            } else {
                "StarTower_base_blue"
            };
            let top = if red {
                "StarTower_top_red"
            } else {
                "StarTower_top_blue"
            };
            sprite_top = self.layer(
                out,
                "building_tower",
                base,
                Playback::default(),
                0,
                x as f64,
                py,
                scale,
                false,
            )?;
            self.layer(
                out,
                "building_tower",
                top,
                Playback::default(),
                0,
                x as f64,
                py,
                scale,
                false,
            )?;
            let scene = self
                .scenes
                .get("chr_princess")
                .ok_or("attached TowerPrincess scene missing")?;
            if let Some(mut selected) = animation::select_for_heading(
                scene,
                "princess_tower",
                Some("princess_tower_red"),
                red,
                state,
                heading,
                phase,
            )? {
                Self::align_attack_frame(scene, &mut selected, state, phase);
                let top_y = py - 2200. * 0.03 * field_scale;
                let sy = self.layer(
                    out,
                    "chr_princess",
                    &selected.export,
                    selected.playback,
                    selected.frame_index,
                    x as f64,
                    top_y,
                    field_scale,
                    selected.flip_horizontal,
                )?;
                sprite_top = sprite_top.min(sy);
            }
        } else {
            let scene = self
                .scenes
                .get(&visual.document)
                .ok_or("entity scene missing")?;
            let selection = if projectile {
                // Original projectile art, with separate treatment for view
                // banks and timed clips. The original client view accessor is
                // unrecovered; in-plane flight orientation is presentation.
                let export = if red {
                    visual.red_prefix.as_deref().unwrap_or(&visual.prefix)
                } else {
                    &visual.prefix
                };
                let empty = serde_json::Map::new();
                let fields = self
                    .row_meta(&entity.row)
                    .and_then(|v| v["source_fields"].as_object())
                    .unwrap_or(&empty);
                let mode = crate::projectile::mode_for_source(fields, export);
                let flight = if motion == (0, 0) {
                    (0, if red { -1 } else { 1 })
                } else {
                    motion
                };
                let mut presentation = crate::projectile::select_projectile(
                    scene, export, mode, flight.0, flight.1, age, motion_dz,
                )?;
                let angle = presentation.rotation_radians;
                if angle.abs() > 1e-8 {
                    presentation.selection.playback.root_transform = Some(Matrix {
                        a: angle.cos(),
                        b: -angle.sin(),
                        c: angle.sin(),
                        d: angle.cos(),
                        tx: 0.,
                        ty: 0.,
                    });
                }
                Some(presentation.selection)
            } else {
                animation::select_for_heading(
                    scene,
                    &visual.prefix,
                    visual.red_prefix.as_deref(),
                    red,
                    state,
                    heading,
                    phase,
                )?
            };
            if let Some(mut selected) = selection {
                Self::align_attack_frame(scene, &mut selected, state, phase);
                if !projectile {
                    self.apply_deploy_filter(
                        &entity.row,
                        entity.first_seen_tick.map(|_| age),
                        &mut selected.playback,
                    );
                }
                sprite_top = self.layer(
                    out,
                    &visual.document,
                    &selected.export,
                    selected.playback,
                    selected.frame_index,
                    x as f64,
                    py,
                    scale,
                    selected.flip_horizontal,
                )?;
            } else {
                self.unresolved_rows.insert(entity.row.clone());
            }
        }
        if entity.max_hp > 0 {
            let width = if matches!(entity.row.as_str(), "KingTower" | "PrincessTower") {
                48.
            } else {
                32.
            };
            let w = (width * field_scale).round().max(20.) as i32;
            let by = (sprite_top - 9. * field_scale).round() as i32;
            fill_rect(out, x - w / 2 - 1, by - 1, w + 2, 6, DARK);
            fill_rect(
                out,
                x - w / 2,
                by,
                (w as f64 * entity.hp.max(0) as f64 / entity.max_hp as f64).round() as i32,
                4,
                if red { RED } else { BLUE },
            );
            if matches!(entity.row.as_str(), "KingTower" | "PrincessTower") {
                self.text(
                    out,
                    &entity.hp.to_string(),
                    x.max(0) as u32,
                    (by - 15).max(0) as u32,
                    12. * field_scale as f32,
                    WHITE,
                    1,
                )?;
            }
        }
        Ok(())
    }
    fn apply_deploy_filter(&self, row: &str, age: Option<u64>, playback: &mut Playback) {
        let Some(age) = age else {
            return;
        };
        let Some(export) = self
            .row_meta(row)
            .and_then(|v| v["source_fields"]["DeployBaseAnimExportName"].as_str())
        else {
            return;
        };
        let Some(scene) = self.scenes.get("effects") else {
            return;
        };
        let Some(movie) = scene
            .document
            .exports
            .get(export)
            .and_then(|id| scene.document.movies.get(id))
        else {
            return;
        };
        let index = (age * movie.framerate as u64 / 1000) as usize;
        if index >= movie.frame_element_counts.len() || movie.frame_element_counts[index] != 1 {
            return;
        }
        let offset = movie.frame_elements_offset as usize
            + movie.frame_element_counts[..index]
                .iter()
                .map(|v| *v as usize * 3)
                .sum::<usize>();
        let Some(elements) = scene.document.frame_elements.get(offset..offset + 3) else {
            return;
        };
        if elements[1] != u16::MAX {
            playback.root_transform = scene
                .document
                .matrix_banks
                .get(movie.matrix_bank_index)
                .and_then(|bank| bank.get(elements[1] as usize))
                .copied();
        }
        if elements[2] != u16::MAX {
            playback.root_color = scene
                .document
                .color_banks
                .get(movie.matrix_bank_index)
                .and_then(|bank| bank.get(elements[2] as usize))
                .copied();
        }
    }
    fn align_attack_frame(
        scene: &Scene,
        selection: &mut animation::PlaybackSelection,
        state: AnimationState,
        phase: u64,
    ) {
        if state != AnimationState::Attacking {
            return;
        }
        let Some(movie) = scene
            .document
            .exports
            .get(&selection.export)
            .and_then(|id| scene.document.movies.get(id))
        else {
            return;
        };
        if let Some(action) = movie.frame_labels.iter().position(|l| l == "action_frame") {
            // Native projectile birth/target HP loss is the observed impact.
            // Align it to the source authored action frame, then play recovery.
            let elapsed = (phase * movie.framerate as u64 / 1000) as usize;
            let index = (action + elapsed).min(movie.frame_element_counts.len().saturating_sub(1));
            selection.frame_index = index;
            selection.playback.frame = Some(index);
        }
    }

    fn text(
        &mut self,
        out: &mut RgbaImage,
        text: &str,
        cx: u32,
        y: u32,
        size: f32,
        color: u32,
        align: u8,
    ) -> Result<()> {
        let key = (text.to_owned(), size.to_bits(), color, align);
        if !self.text_cache.contains_key(&key) {
            let scene = self
                .scenes
                .get(&self.font_scene)
                .ok_or("font scene missing")?;
            let rendered = scene
                .render_text(
                    text,
                    "Clash Text",
                    size,
                    color,
                    Some(0xff15202b),
                    align,
                    self.width,
                )
                .map_err(|e| e.to_string())?;
            self.text_cache.insert(key.clone(), rendered.image);
        }
        let rendered = self
            .text_cache
            .get(&key)
            .ok_or("cached font render missing")?;
        let x = cx as i64 - rendered.width() as i64 / 2;
        imageops::overlay(out, rendered, x, y as i64);
        Ok(())
    }
}

fn fill_rect(img: &mut RgbaImage, x: i32, y: i32, w: i32, h: i32, color: Rgba<u8>) {
    if w <= 0 || h <= 0 {
        return;
    }
    let x0 = x.max(0) as u32;
    let y0 = y.max(0) as u32;
    let x1 = (x.saturating_add(w)).max(0).min(img.width() as i32) as u32;
    let y1 = (y.saturating_add(h)).max(0).min(img.height() as i32) as u32;
    for py in y0..y1 {
        for px in x0..x1 {
            img.put_pixel(px, py, color);
        }
    }
}

fn fill_circle(img: &mut RgbaImage, cx: i32, cy: i32, radius: i32, color: Rgba<u8>) {
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                let (x, y) = (cx + dx, cy + dy);
                if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
                    img.put_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tower(address: u64, row: &str, side: i32, x: i32, y: i32) -> Entity {
        Entity {
            address,
            row: row.to_owned(),
            side,
            x,
            y,
            z: 0,
            hp: 3000,
            max_hp: 3000,
            moving: false,
            attacking: false,
            ..Entity::default()
        }
    }

    #[test]
    fn original_deploy_filter_ends_instead_of_repeating() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let painter = Painter::load(&root, 540, 900).expect("source bundle");
        let mut start = Playback::default();
        painter.apply_deploy_filter("Knight", Some(0), &mut start);
        let transform = start
            .root_transform
            .expect("Knight source deploy transform");
        assert!(
            transform.ty < -60.0,
            "authored deployment begins above the board"
        );
        assert!(start.root_color.expect("source deploy color").alpha < 255);
        let mut done = Playback::default();
        painter.apply_deploy_filter("Knight", Some(600), &mut done);
        assert!(done.root_transform.is_none());
        assert!(done.root_color.is_none());
        let mut unknown_age = Playback::default();
        painter.apply_deploy_filter("Knight", None, &mut unknown_age);
        assert!(unknown_age.root_transform.is_none());
    }

    #[test]
    fn renders_original_assets_for_initial_towers() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let mut painter = Painter::load(&root, 720, 1080).expect("load source asset bundle");
        let frame = BattleFrame {
            tick: 0,
            elapsed_ms: 0,
            terminated: false,
            winner_side: None,
            arena_size: [18000, 32000],
            elixir: [5.0, 5.0],
            entities: vec![
                tower(1, "KingTower", 0, 9000, 3000),
                tower(2, "PrincessTower", 0, 3500, 6500),
                tower(3, "PrincessTower", 0, 14500, 6500),
                tower(4, "KingTower", 1, 9000, 29000),
                tower(5, "PrincessTower", 1, 3500, 25500),
                tower(6, "PrincessTower", 1, 14500, 25500),
            ],
            hands: Default::default(),
            controller_names: Default::default(),
            plays: Vec::new(),
            match_id: 0,
        };
        let mut frame = frame;
        let mut image = painter.render(&frame).expect("render initial tower frame");
        let started = std::time::Instant::now();
        for i in 1..=31 {
            frame.tick = i;
            frame.elapsed_ms = u64::from(i) * 50;
            image = painter.render(&frame).expect("render cached tower frame");
        }
        eprintln!(
            "warm spectator frame: {:.2} ms",
            started.elapsed().as_secs_f64() * 1000.0 / 31.0
        );
        assert_eq!(image.dimensions(), (720, 1080));
        assert!(
            painter.unresolved_rows.is_empty(),
            "tower rows should use original tower exports"
        );
        image
            .save("/tmp/native-spectator-initial-towers.png")
            .expect("write smoke preview");
    }
}
