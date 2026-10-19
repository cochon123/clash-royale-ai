//! Spectator hand panels using authoritative native hands and original art.
use crate::frame::BattleFrame;
use clash_native_ui_viewer::{Playback, Scene};
use image::{imageops, Rgba, RgbaImage};
use serde_json::Value;
use std::{collections::BTreeMap, path::Path};

type Result<T> = std::result::Result<T, String>;
#[derive(Default)]
pub struct HandHud {
    cards: BTreeMap<(String, u32), RgbaImage>,
    labels: BTreeMap<(String, u32), RgbaImage>,
}
impl HandHud {
    fn label(
        &mut self,
        out: &mut RgbaImage,
        scene: &Scene,
        value: &str,
        x: i64,
        y: i64,
        size: f32,
    ) -> Result<()> {
        let key = (value.to_owned(), size.to_bits());
        if !self.labels.contains_key(&key) {
            let text = scene
                .render_text(
                    value,
                    "Clash Text",
                    size,
                    0xffffffff,
                    Some(0xff101521),
                    1,
                    600,
                )
                .map_err(|e| e.to_string())?;
            if self.labels.len() > 512 {
                self.labels.clear();
            }
            self.labels.insert(key.clone(), text.image);
        }
        let text = &self.labels[&key];
        imageops::overlay(out, text, x - text.width() as i64 / 2, y);
        Ok(())
    }
    fn card(
        &mut self,
        root: &Path,
        scene: &Scene,
        metadata: &Value,
        name: &str,
        width: u32,
    ) -> Result<Option<RgbaImage>> {
        let key = (name.to_owned(), width);
        if let Some(image) = self.cards.get(&key) {
            return Ok(Some(image.clone()));
        }
        let Some(relative) = metadata["portrait"]["file"].as_str() else {
            return Ok(None);
        };
        let path = [
            root.join(relative),
            root.join(
                metadata["portrait"]["recovered_path"]
                    .as_str()
                    .unwrap_or("cards/missing.png"),
            ),
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join(relative),
        ]
        .into_iter()
        .find(|p| p.is_file());
        let Some(path) = path else {
            return Err(format!("original portrait missing: {name} {relative}"));
        };
        let portrait = image::open(path).map_err(|e| e.to_string())?.to_rgba8();
        let h = (width as f64 * 163. / 130.).round() as u32;
        let mut card = RgbaImage::new(width, h);
        let art = imageops::resize(
            &portrait,
            width.saturating_sub(5).max(1),
            h.saturating_sub(5).max(1),
            imageops::FilterType::Triangle,
        );
        imageops::overlay(&mut card, &art, 2, 2);
        let export = metadata["ui"]["frame_export"]
            .as_str()
            .unwrap_or("card_item_frame");
        if let Some(frame) = scene
            .render(export, &Playback::default())
            .map_err(|e| e.to_string())?
        {
            let frame = imageops::resize(&frame.image, width, h, imageops::FilterType::Triangle);
            imageops::overlay(&mut card, &frame, 0, 0);
        }
        self.cards.insert(key, card.clone());
        Ok(Some(card))
    }
    pub fn render(
        &mut self,
        out: &mut RgbaImage,
        root: &Path,
        scenes: &BTreeMap<String, Scene>,
        manifest: &Value,
        frame: &BattleFrame,
        panel_h: u32,
    ) -> Result<()> {
        let scene = scenes
            .get("ui")
            .ok_or("original ui.sc missing for card hands")?;
        let (width, height) = out.dimensions();
        let scale = width as f32 / 540.;
        for side in [1usize, 0] {
            let top = if side == 1 { 0 } else { height - panel_h };
            let accent = if side == 0 {
                [58, 151, 246]
            } else {
                [224, 73, 72]
            };
            for y in top..top + panel_h {
                let t = (y - top) as f32 / panel_h as f32;
                let c = Rgba([
                    (15. + accent[0] as f32 * 0.13 * (1. - t)) as u8,
                    (23. + accent[1] as f32 * 0.13 * (1. - t)) as u8,
                    (32. + accent[2] as f32 * 0.13 * (1. - t)) as u8,
                    255,
                ]);
                for x in 0..width {
                    out.put_pixel(x, y, c);
                }
            }
            let edge = if side == 1 { top + panel_h - 2 } else { top };
            for y in edge..edge + 2 {
                for x in 0..width {
                    out.put_pixel(x, y, Rgba([accent[0], accent[1], accent[2], 255]));
                }
            }
            let who = if frame.controller_names[side].is_empty() {
                if side == 0 {
                    "BLUE"
                } else {
                    "RED"
                }
            } else {
                &frame.controller_names[side]
            };
            self.label(
                out,
                scene,
                &format!("{}  ·  {}", if side == 0 { "BLUE" } else { "RED" }, who),
                width as i64 / 2,
                top as i64 + 3,
                12. * scale,
            )?;
            let cw = ((panel_h.saturating_sub(45)) as f64 * 130. / 163.)
                .round()
                .max(24.) as u32;
            let gap = (6. * scale).round().max(3.) as u32;
            let x0 = (width.saturating_sub(cw * 4 + gap * 3)) / 2;
            let cy = top + (20. * scale).round() as u32;
            for slot in 0..4u32 {
                let x = x0 + slot * (cw + gap);
                let row = frame.hands[side]
                    .iter()
                    .find(|c| c.slot == slot && c.available);
                if let Some(card) = row {
                    let meta = &manifest["scene"]["card_visuals"][&card.name];
                    if let Some(image) = self.card(root, scene, meta, &card.name, cw)? {
                        imageops::overlay(out, &image, x as i64, cy as i64);
                        if frame.elixir[side] + 0.001 < card.cost as f64 {
                            let shade = RgbaImage::from_pixel(
                                image.width(),
                                image.height(),
                                Rgba([5, 9, 16, 128]),
                            );
                            imageops::overlay(out, &shade, x as i64, cy as i64);
                        }
                        let iconw = (cw as f64 * 0.34).round().max(14.) as u32;
                        if let Some(icon) = scene
                            .render("card_item_elixir_ingame", &Playback::default())
                            .map_err(|e| e.to_string())?
                        {
                            let ih = (icon.image.height() as f64 * iconw as f64
                                / icon.image.width() as f64)
                                .round() as u32;
                            let image = imageops::resize(
                                &icon.image,
                                iconw,
                                ih,
                                imageops::FilterType::Triangle,
                            );
                            let ix = x + cw / 2 - iconw / 2;
                            let iy = cy + (cw as f64 * 163. / 130.).round() as u32 - ih / 2;
                            imageops::overlay(out, &image, ix as i64, iy as i64);
                            self.label(
                                out,
                                scene,
                                &card.cost.to_string(),
                                (x + cw / 2) as i64,
                                iy as i64 + (ih as f32 * 0.27) as i64,
                                14. * scale,
                            )?;
                        }
                    } else {
                        self.label(
                            out,
                            scene,
                            &card.name,
                            (x + cw / 2) as i64,
                            cy as i64 + 25,
                            9. * scale,
                        )?;
                    }
                }
            }
            if frame.hands[side].is_empty() {
                self.label(
                    out,
                    scene,
                    "Hand not recorded",
                    width as i64 / 2,
                    (top + panel_h / 2) as i64,
                    12. * scale,
                )?;
            }
            let bar_y = top + panel_h - 10;
            let bar_x = (width as f32 * 0.23) as u32;
            let bar_w = (width as f32 * 0.54) as u32;
            for y in bar_y..bar_y + 5 {
                for x in bar_x..bar_x + bar_w {
                    out.put_pixel(
                        x,
                        y,
                        if x - bar_x
                            < (bar_w as f64 * frame.elixir[side] / 10.).clamp(0., bar_w as f64)
                                as u32
                        {
                            Rgba([209, 68, 233, 255])
                        } else {
                            Rgba([40, 30, 57, 255])
                        },
                    );
                }
            }
            self.label(
                out,
                scene,
                &format!("{:.1}", frame.elixir[side]),
                (bar_x.saturating_sub(25)) as i64,
                bar_y as i64 - 15,
                13. * scale,
            )?;
        }
        Ok(())
    }
}
