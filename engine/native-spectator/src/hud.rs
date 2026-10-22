//! Spectator hand panels using authoritative native hands and original art.
//!
//! Panels use the original `ui_spectate_hud` wood planks and its elixir bar.
//! The client assembles that bar in code (a narrow body stretched by elixir,
//! nine dividers placed at runtime); the same pieces are composed here.
use crate::{frame::BattleFrame, scene as target};
use clash_native_ui_viewer::{Playback, Scene};
use image::{imageops, Rgba, RgbaImage};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

type Result<T> = std::result::Result<T, String>;

/// An original piece rendered at source scale; `(x, y)` is its top-left in
/// the coordinate space of its spectate HUD export.
struct Piece {
    image: RgbaImage,
    x: f64,
    y: f64,
}

struct SpectateArt {
    /// `panel_back` and `panel` planks, per side (0 blue/player, 1 red/enemy).
    planks: [(Piece, Piece); 2],
    bar_bg: Piece,
    bar_body: Piece,
    bar_end: Piece,
    bar_left: Piece,
    divider: Piece,
    /// Column of the divider artwork's visible line.
    divider_line: f64,
}

#[derive(Default)]
pub struct HandHud {
    cards: BTreeMap<(String, u32), RgbaImage>,
    labels: BTreeMap<(String, u32), RgbaImage>,
    art: Option<Option<SpectateArt>>,
    fitted: BTreeMap<(&'static str, usize, u32, u32), RgbaImage>,
}

/// Shared card rectangles for drawing and mouse hit testing.
pub struct HandLayout {
    pub card_w: u32,
    pub card_h: u32,
    pub gap: u32,
    pub row_w: u32,
    pub x: u32,
    pub y: u32,
}

impl HandLayout {
    pub fn new(width: u32, height: u32, panel_h: u32, side: usize) -> Self {
        let scale = width as f32 / 540.;
        let card_w = ((panel_h.saturating_sub(58)) as f64 * 130. / 163.)
            .round()
            .max(24.) as u32;
        let card_h = (card_w as f64 * 163. / 130.).round() as u32;
        let gap = (6. * scale).round().max(3.) as u32;
        let row_w = card_w * 4 + gap * 3;
        Self {
            card_w,
            card_h,
            gap,
            row_w,
            x: width.saturating_sub(row_w) / 2,
            y: (if side == 1 { 0 } else { height - panel_h }) + (11. * scale).round() as u32,
        }
    }
    pub fn card_rect(&self, slot: u32) -> [f64; 4] {
        [
            (self.x + slot * (self.card_w + self.gap)) as f64,
            self.y as f64,
            self.card_w as f64,
            self.card_h as f64,
        ]
    }
}

fn render_children(scene: &Scene, export: &str, keep: &[&str]) -> Option<Piece> {
    let id = *scene.document.exports.get(export)?;
    let movie = scene.document.movies.get(&id)?;
    let hidden: BTreeSet<u32> = movie
        .children_ids
        .iter()
        .zip(&movie.children_names)
        .filter(|(_, name)| !keep.contains(&name.as_str()))
        .map(|(id, _)| *id)
        .collect();
    let rendered = scene
        .render(
            export,
            &Playback {
                hidden_objects: hidden,
                ..Playback::default()
            },
        )
        .ok()??;
    Some(Piece {
        image: rendered.image,
        x: rendered.x,
        y: rendered.y,
    })
}

/// One named child of the player HUD's `elixir_bar`, in export coordinates.
fn bar_piece(scene: &Scene, name: &str) -> Option<Piece> {
    let export = "spectate_hud_1v1_player";
    let root = scene
        .document
        .movies
        .get(scene.document.exports.get(export)?)?;
    let bar_id = root
        .children_ids
        .iter()
        .zip(&root.children_names)
        .find(|(_, n)| n.as_str() == "elixir_bar")
        .map(|(id, _)| *id)?;
    let bar = scene.document.movies.get(&bar_id)?;
    let mut hidden: BTreeSet<u32> = root
        .children_ids
        .iter()
        .copied()
        .filter(|id| *id != bar_id)
        .collect();
    // Dividers d1..d9 share one object id; keep that id visible.
    let keep = bar
        .children_ids
        .iter()
        .zip(&bar.children_names)
        .find(|(_, child)| child.as_str() == name)
        .map(|(id, _)| *id)?;
    hidden.extend(bar.children_ids.iter().copied().filter(|id| *id != keep));
    let rendered = scene
        .render(
            export,
            &Playback {
                hidden_objects: hidden,
                ..Playback::default()
            },
        )
        .ok()??;
    Some(Piece {
        image: rendered.image,
        x: rendered.x,
        y: rendered.y,
    })
}

impl SpectateArt {
    fn load(scenes: &BTreeMap<String, Scene>) -> Option<Self> {
        let scene = scenes.get("ui_spectate_hud")?;
        let planks = |export| {
            Some((
                render_children(scene, export, &["panel_back"])?,
                render_children(scene, export, &["panel"])?,
            ))
        };
        Some(Self {
            planks: [
                planks("spectate_hud_1v1_player")?,
                planks("spectate_hud_1v1_enemy")?,
            ],
            bar_bg: bar_piece(scene, "bar_bg")?,
            bar_body: bar_piece(scene, "bar_body")?,
            bar_end: bar_piece(scene, "bar_end")?,
            bar_left: bar_piece(scene, "elixirBarLeft")?,
            divider_line: {
                let d = bar_piece(scene, "d1")?;
                (0..d.image.width())
                    .max_by_key(|x| {
                        (0..d.image.height())
                            .map(|y| d.image.get_pixel(*x, y)[3] as u32)
                            .sum::<u32>()
                    })
                    .unwrap_or(0) as f64
            },
            divider: bar_piece(scene, "d1")?,
        })
    }
}

fn place(out: &mut RgbaImage, image: &RgbaImage, x: f64, y: f64) {
    target::overlay(out, image, x.round() as i64, y.round() as i64);
}

fn scaled(piece: &Piece, sx: f64, sy: f64) -> RgbaImage {
    imageops::resize(
        &piece.image,
        ((piece.image.width() as f64 * sx).round() as u32).max(1),
        ((piece.image.height() as f64 * sy).round() as u32).max(1),
        imageops::FilterType::Triangle,
    )
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
        target::overlay(out, text, x - text.width() as i64 / 2, y);
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
        target::overlay(&mut card, &art, 2, 2);
        let export = metadata["ui"]["frame_export"]
            .as_str()
            .unwrap_or("card_item_frame");
        if let Some(frame) = scene
            .render(export, &Playback::default())
            .map_err(|e| e.to_string())?
        {
            let frame = imageops::resize(&frame.image, width, h, imageops::FilterType::Triangle);
            target::overlay(&mut card, &frame, 0, 0);
        }
        self.cards.insert(key, card.clone());
        Ok(Some(card))
    }

    /// Plank backgrounds stretched to a panel, cached per side and size.
    fn planks(&mut self, side: usize, width: u32, height: u32, inner: (u32, u32)) -> bool {
        let Some(Some(art)) = &self.art else {
            return false;
        };
        let (back, front) = &art.planks[side];
        for (key, piece, w, h) in [
            ("back", back, width, height),
            ("front", front, inner.0, inner.1),
        ] {
            self.fitted.entry((key, side, w, h)).or_insert_with(|| {
                imageops::resize(
                    &piece.image,
                    w.max(1),
                    h.max(1),
                    imageops::FilterType::Triangle,
                )
            });
        }
        true
    }

    /// Original elixir bar: background, stretched body to the current elixir,
    /// end cap, nine dividers and the drop icon carrying the value.
    #[allow(clippy::too_many_arguments)]
    fn elixir_bar(
        &mut self,
        out: &mut RgbaImage,
        scene: &Scene,
        cx: f64,
        cy: f64,
        bar_w: f64,
        elixir: f64,
        size: f32,
    ) -> Result<bool> {
        let Some(Some(art)) = &self.art else {
            return Ok(false);
        };
        let s = bar_w / art.bar_bg.image.width() as f64;
        let origin_x = cx - bar_w / 2. - art.bar_bg.x * s;
        let origin_y = cy - (art.bar_bg.y + art.bar_bg.image.height() as f64 / 2.) * s;
        let at = |p: &Piece| (origin_x + p.x * s, origin_y + p.y * s);
        let (bx, by) = at(&art.bar_bg);
        place(out, &scaled(&art.bar_bg, s, s), bx, by);
        // The fillable span runs from the body's origin to the background's
        // inner end, mirrored from its left inset.
        let (fx, fy) = at(&art.bar_body);
        let inset = fx - bx;
        let span = (art.bar_bg.image.width() as f64 * s - 2. * inset).max(1.);
        let fraction = (elixir / 10.).clamp(0., 1.);
        let fill_w = span * fraction;
        if fill_w >= 1. {
            let body = imageops::resize(
                &art.bar_body.image,
                fill_w.round().max(1.) as u32,
                ((art.bar_body.image.height() as f64 * s).round() as u32).max(1),
                imageops::FilterType::Triangle,
            );
            place(out, &body, fx, fy);
            let (_, ey) = at(&art.bar_end);
            let cap = scaled(&art.bar_end, s, s);
            place(out, &cap, fx + fill_w - cap.width() as f64 / 2., ey);
        }
        let divider = scaled(&art.divider, s, s);
        let (_, dy) = at(&art.divider);
        let line = art.divider_line * s;
        for k in 1..10 {
            place(out, &divider, fx + span * k as f64 / 10. - line, dy);
        }
        let (lx, ly) = at(&art.bar_left);
        let left = scaled(&art.bar_left, s, s);
        place(out, &left, lx, ly);
        self.label(
            out,
            scene,
            &format!("{}", elixir.floor() as i64),
            (lx + left.width() as f64 / 2.) as i64,
            (ly + left.height() as f64 * 0.32) as i64,
            size,
        )?;
        Ok(true)
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
        if self.art.is_none() {
            self.art = Some(SpectateArt::load(scenes));
        }
        let (width, height) = out.dimensions();
        let scale = width as f32 / 540.;
        let geometry = HandLayout::new(width, height, panel_h, 0);
        let (cw, card_h, gap, row_w) = (
            geometry.card_w,
            geometry.card_h,
            geometry.gap,
            geometry.row_w,
        );
        let inner = (row_w + 4 * gap, card_h + 2 * gap);
        for side in [1usize, 0] {
            let _scope = target::ScopeGuard::enter(format!("hud-{side}"));
            let top = if side == 1 { 0 } else { height - panel_h };
            let accent = if side == 0 {
                [58, 151, 246]
            } else {
                [224, 73, 72]
            };
            let hand_layout = HandLayout::new(width, height, panel_h, side);
            let (x0, cy) = (hand_layout.x, hand_layout.y);
            if self.planks(side, width, panel_h, inner) {
                let back = &self.fitted[&("back", side, width, panel_h)];
                target::replace(out, back, 0, top as i64);
                let front = &self.fitted[&("front", side, inner.0, inner.1)];
                target::overlay(out, front, (x0 - 2 * gap) as i64, cy as i64 - gap as i64);
            } else {
                for y in top..top + panel_h {
                    let t = (y - top) as f32 / panel_h as f32;
                    let c = Rgba([
                        (15. + accent[0] as f32 * 0.13 * (1. - t)) as u8,
                        (23. + accent[1] as f32 * 0.13 * (1. - t)) as u8,
                        (32. + accent[2] as f32 * 0.13 * (1. - t)) as u8,
                        255,
                    ]);
                    target::fill_rect(out, 0, y as i32, width as i32, 1, c);
                }
            }
            let edge = if side == 1 { top + panel_h - 2 } else { top };
            target::fill_rect(
                out,
                0,
                edge as i32,
                width as i32,
                2,
                Rgba([accent[0], accent[1], accent[2], 255]),
            );
            let who = if frame.controller_names[side].is_empty() {
                if side == 0 {
                    "BLUE"
                } else {
                    "RED"
                }
            } else {
                &frame.controller_names[side]
            };
            // Side and controller in the plank's left margin, beside the cards.
            let margin_x = (x0.saturating_sub(2 * gap) / 2) as i64;
            let mid = (cy + card_h / 2) as i64;
            self.label(
                out,
                scene,
                if frame.human.is_some() {
                    if side == 0 {
                        "YOU"
                    } else {
                        "AI"
                    }
                } else if side == 0 {
                    "BLUE"
                } else {
                    "RED"
                },
                margin_x,
                mid - (20. * scale) as i64,
                14. * scale,
            )?;
            self.label(
                out,
                scene,
                if frame.human.is_some() && side == 0 {
                    "Player"
                } else {
                    who
                },
                margin_x,
                mid + (2. * scale) as i64,
                11. * scale,
            )?;
            if frame.human.as_ref().is_some_and(|h| h.side != side) {
                self.label(
                    out,
                    scene,
                    "Opponent hand hidden",
                    (width as f32 * 0.59) as i64,
                    (top as f32 + 23. * scale) as i64,
                    13. * scale,
                )?;
                self.label(
                    out,
                    scene,
                    "1-4 select | Click arena to play",
                    (width as f32 * 0.59) as i64,
                    (top as f32 + 50. * scale) as i64,
                    12. * scale,
                )?;
                self.label(
                    out,
                    scene,
                    "Space pause | R restart | Q quit",
                    (width as f32 * 0.59) as i64,
                    (top as f32 + 73. * scale) as i64,
                    11. * scale,
                )?;
                continue;
            }
            for slot in 0..4u32 {
                let x = x0 + slot * (cw + gap);
                let row = frame.hands[side]
                    .iter()
                    .find(|c| c.slot == slot && c.available);
                if let Some(card) = row {
                    let meta = &manifest["scene"]["card_visuals"][&card.name];
                    if let Some(image) = self.card(root, scene, meta, &card.name, cw)? {
                        target::overlay(out, &image, x as i64, cy as i64);
                        if frame.elixir[side] + 0.001 < card.cost as f64 {
                            let shade = RgbaImage::from_pixel(
                                image.width(),
                                image.height(),
                                Rgba([5, 9, 16, 128]),
                            );
                            target::overlay(out, &shade, x as i64, cy as i64);
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
                            let iy = cy + card_h - ih / 2;
                            target::overlay(out, &image, ix as i64, iy as i64);
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
            let bar_cy = (top + panel_h) as f64 - 19. * scale as f64;
            let drew = self.elixir_bar(
                out,
                scene,
                width as f64 * 0.52,
                bar_cy,
                width as f64 * 0.46,
                frame.elixir[side],
                12. * scale,
            )?;
            if !drew {
                let bar_y = top + panel_h - 10;
                let bar_x = (width as f32 * 0.23) as u32;
                let bar_w = (width as f32 * 0.54) as u32;
                let fill_w =
                    (bar_w as f64 * frame.elixir[side] / 10.).clamp(0., bar_w as f64) as i32;
                target::fill_rect(
                    out,
                    bar_x as i32,
                    bar_y as i32,
                    bar_w as i32,
                    5,
                    Rgba([40, 30, 57, 255]),
                );
                target::fill_rect(
                    out,
                    bar_x as i32,
                    bar_y as i32,
                    fill_w,
                    5,
                    Rgba([209, 68, 233, 255]),
                );
                self.label(
                    out,
                    scene,
                    &format!("{:.1}", frame.elixir[side]),
                    (bar_x.saturating_sub(25)) as i64,
                    bar_y as i64 - 15,
                    13. * scale,
                )?;
            }
        }
        Ok(())
    }
}
