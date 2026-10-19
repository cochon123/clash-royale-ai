//! Read-only battle renderer. All character, arena, and decoration artwork is
//! loaded from the original SC5 bundle; spectator-owned bars/text use a font
//! extracted from that same bundle.
use crate::frame::{BattleFrame, Entity};
use clash_native_ui_viewer::{Playback, Rendered, Scene};
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
    exports: BTreeMap<String, BTreeSet<String>>,
    bindings: BTreeMap<String, Visual>,
    sprite_cache: BTreeMap<(String, String, usize), Option<Rendered>>,
    deco_cache: BTreeMap<String, RgbaImage>,
    text_cache: BTreeMap<(String, u32, u32, u8), RgbaImage>,
    arena_field: Option<RgbaImage>,
    hud_cache: BTreeMap<String, Option<Rendered>>,
    hud_fit_cache: BTreeMap<String, RgbaImage>,
    field_cache: Option<RgbaImage>,
    sprite_fit_cache: BTreeMap<(String, String, usize, u64), (RgbaImage, i64, i64)>,
    font_scene: String,
    pub unresolved_rows: BTreeSet<String>,
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
            let crop_y = (0.0 - rendered.y).round() as i64;
            if crop_x < 0
                || crop_y < 0
                || crop_x + 540 > rendered.image.width() as i64
                || crop_y + 768 > rendered.image.height() as i64
            {
                return Err(format!("calibrated arena crop is outside original background bounds: origin ({}, {}), image {}x{}", rendered.x, rendered.y, rendered.image.width(), rendered.image.height()));
            }
            Some(
                imageops::crop_imm(&rendered.image, crop_x as u32, crop_y as u32, 540, 768)
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
            exports,
            bindings,
            sprite_cache: BTreeMap::new(),
            deco_cache: BTreeMap::new(),
            text_cache: BTreeMap::new(),
            arena_field,
            hud_cache: BTreeMap::new(),
            hud_fit_cache: BTreeMap::new(),
            field_cache: None,
            sprite_fit_cache: BTreeMap::new(),
            font_scene,
            unresolved_rows: BTreeSet::new(),
        })
    }

    pub fn render(&mut self, frame: &BattleFrame) -> Result<RgbaImage> {
        let mut out = RgbaImage::from_pixel(self.width, self.height, Rgba([19, 27, 31, 255]));
        let hud = ((self.width as f32 * 308.0 / 965.0).round() as u32).min(self.height / 3);
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
            let py = field_y as f64 + ((1.0 - entity.y as f64 / world_h) * field_h as f64);
            self.draw_entity(
                &mut out,
                frame,
                entity,
                px as i32,
                py as i32,
                field_w as f64 / 540.0,
            )?;
        }
        self.draw_hud(&mut out, frame, field_x, field_y, field_w, field_h)?;
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
                let py = y as f64 + (1.0 - py / 32000.0) * h as f64;
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
            let team = if entity.side == 0 { BLUE } else { RED };
            fill_circle(out, x, y, 9, team);
            return Ok(());
        };
        let action = if entity.attacking {
            "attack"
        } else if entity.moving {
            "run"
        } else {
            "idle"
        };
        let is_red = entity.side != 0;
        let exports = self
            .exports
            .get(&visual.document)
            .cloned()
            .unwrap_or_default();
        let chosen_prefix = if is_red {
            visual.red_prefix.as_deref().unwrap_or(&visual.prefix)
        } else {
            &visual.prefix
        };
        let matches = |name: &&String| {
            name.starts_with(chosen_prefix)
                && name.to_lowercase().contains(action)
                && (if is_red {
                    name.to_lowercase().contains("enemy") || name.to_lowercase().contains("red")
                } else {
                    name.to_lowercase().contains("blue") || !name.to_lowercase().contains("enemy")
                })
        };
        let export = exports
            .iter()
            .find(matches)
            .or_else(|| exports.iter().find(|name| name.starts_with(chosen_prefix)))
            .cloned();
        let Some(export) = export else {
            self.unresolved_rows.insert(entity.row.clone());
            return Ok(());
        };
        let scene = self
            .scenes
            .get(&visual.document)
            .ok_or("entity scene missing")?;
        let fps = scene.framerate(&export).map_err(|e| e.to_string())? as u64;
        let frame_count = scene
            .frame_count(&export)
            .map_err(|e| e.to_string())?
            .max(1) as u64;
        let frame_index = ((frame.elapsed_ms * fps / 1000) % frame_count) as usize;
        let key = (visual.document.clone(), export.clone(), frame_index);
        if !self.sprite_cache.contains_key(&key) {
            let rendered = scene
                .render(
                    &export,
                    &Playback {
                        frame: Some(frame_index),
                        time_seconds: Some(frame.elapsed_ms as f64 / 1000.0),
                        ..Playback::default()
                    },
                )
                .map_err(|e| e.to_string())?;
            self.sprite_cache.insert(key.clone(), rendered);
        }
        let scale = field_scale * visual.scale_percent / 100.0;
        let fit_key = (
            visual.document.clone(),
            export.clone(),
            frame_index,
            scale.to_bits(),
        );
        if !self.sprite_fit_cache.contains_key(&fit_key) {
            if let Some(Some(sprite)) = self.sprite_cache.get(&key) {
                let sw = (sprite.image.width() as f64 * scale).round().max(1.0) as u32;
                let sh = (sprite.image.height() as f64 * scale).round().max(1.0) as u32;
                let img = imageops::resize(&sprite.image, sw, sh, imageops::FilterType::Triangle);
                let fx = if sprite.x.is_finite() {
                    sprite.x * scale
                } else {
                    0.0
                };
                let fy = if sprite.y.is_finite() {
                    sprite.y * scale
                } else {
                    0.0
                };
                self.sprite_fit_cache
                    .insert(fit_key.clone(), (img, fx.round() as i64, fy.round() as i64));
            }
        }
        if let Some((img, fx, fy)) = self.sprite_fit_cache.get(&fit_key) {
            let sh = img.height();
            imageops::overlay(out, img, x as i64 + *fx, y as i64 + *fy);
            let color = if entity.side == 0 { BLUE } else { RED };
            let bar_w = 36i32;
            let bar_y = y - sh as i32 / 2 - 8;
            fill_rect(out, x - bar_w / 2, bar_y, bar_w as i32, 5, DARK);
            let ratio = if entity.max_hp > 0 {
                (entity.hp as f32 / entity.max_hp as f32).clamp(0., 1.)
            } else {
                0.
            };
            fill_rect(
                out,
                x - bar_w / 2,
                bar_y,
                (bar_w as f32 * ratio).round() as i32,
                5,
                color,
            );
        }
        Ok(())
    }

    fn draw_hud(
        &mut self,
        out: &mut RgbaImage,
        frame: &BattleFrame,
        x: u32,
        y: u32,
        _w: u32,
        h: u32,
    ) -> Result<()> {
        for (export, team_label, clan_label, top) in [
            ("spectate_hud_1v1_enemy", "RED PLAYER", "RIVAL", true),
            ("spectate_hud_1v1_player", "BLUE PLAYER", "YOU", false),
        ] {
            if !self.hud_cache.contains_key(export) {
                let scene = self
                    .scenes
                    .get("ui_spectate_hud")
                    .ok_or("original spectator HUD SC5 document missing")?;
                let mut text_values = BTreeMap::new();
                text_values.insert(38, team_label.to_owned());
                text_values.insert(39, clan_label.to_owned());
                let rendered = scene
                    .render(
                        export,
                        &Playback {
                            text_values,
                            hidden_objects: BTreeSet::from([11]),
                            render_text: true,
                            ..Playback::default()
                        },
                    )
                    .map_err(|e| e.to_string())?;
                self.hud_cache.insert(export.to_owned(), rendered);
            }
            if !self.hud_fit_cache.contains_key(export) {
                if let Some(Some(hud)) = self.hud_cache.get(export) {
                    let target_w = self.width;
                    let scale = target_w as f64 / hud.image.width().max(1) as f64;
                    let target_h = (hud.image.height() as f64 * scale).round().max(1.0) as u32;
                    self.hud_fit_cache.insert(
                        export.to_owned(),
                        imageops::resize(
                            &hud.image,
                            target_w,
                            target_h,
                            imageops::FilterType::Lanczos3,
                        ),
                    );
                }
            }
            if let Some(fitted) = self.hud_fit_cache.get(export) {
                let target_h = fitted.height();
                imageops::overlay(
                    out,
                    fitted,
                    0,
                    if top {
                        0
                    } else {
                        self.height as i64 - target_h as i64
                    },
                );
                let value_y = if top {
                    (target_h as f64 * 0.54) as u32
                } else {
                    self.height.saturating_sub(target_h) + (target_h as f64 * 0.54) as u32
                };
                self.text(
                    out,
                    &format!("ELIXIR {:.1}", frame.elixir[if top { 1 } else { 0 }]),
                    self.width / 2,
                    value_y,
                    14.0,
                    WHITE,
                    1,
                )?;
            }
        }
        let secs = frame.elapsed_ms / 1000;
        let title = format!("{}:{:02}", secs / 60, secs % 60);
        self.text(out, &title, x / 2, y + 8, 19., WHITE, 1)?;
        if frame.terminated {
            let msg = match frame.winner_side {
                Some(0) => "BLUE WINS",
                Some(_) => "RED WINS",
                None => "BATTLE ENDED",
            };
            self.text(out, msg, self.width / 2, y + h - 28, 20., WHITE, 1)?;
        }
        Ok(())
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
        }
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
