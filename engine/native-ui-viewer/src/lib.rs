//! SC5 display-list playback using recovered original polygons and timelines.
//! No game rules, card selection, or invented UI layout are implemented here.

use image::{Rgba, RgbaImage};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::path::{Path, PathBuf};

pub mod vfs;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
const MAX_DIMENSION: u32 = 8192;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct Matrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}
impl Matrix {
    pub const IDENTITY: Self = Self {
        a: 1.,
        b: 0.,
        c: 0.,
        d: 1.,
        tx: 0.,
        ty: 0.,
    };
    fn compose(self, child: Self) -> Self {
        Self {
            a: self.a * child.a + self.b * child.c,
            b: self.a * child.b + self.b * child.d,
            c: self.c * child.a + self.d * child.c,
            d: self.c * child.b + self.d * child.d,
            tx: self.a * child.tx + self.b * child.ty + self.tx,
            ty: self.c * child.tx + self.d * child.ty + self.ty,
        }
    }
    fn transform(self, x: f64, y: f64) -> (f64, f64) {
        (
            self.a * x + self.b * y + self.tx,
            self.c * x + self.d * y + self.ty,
        )
    }
    fn translation_only(self) -> bool {
        self.a == 1. && self.b == 0. && self.c == 0. && self.d == 1.
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct ColorTransform {
    pub r_mul: i32,
    pub g_mul: i32,
    pub b_mul: i32,
    pub alpha: i32,
    pub r_add: i32,
    pub g_add: i32,
    pub b_add: i32,
}
impl ColorTransform {
    const IDENTITY: Self = Self {
        r_mul: 255,
        g_mul: 255,
        b_mul: 255,
        alpha: 255,
        r_add: 0,
        g_add: 0,
        b_add: 0,
    };
    fn apply(self, image: &mut RgbaImage) {
        if self == Self::IDENTITY {
            return;
        }
        for pixel in image.pixels_mut() {
            pixel.0 = [
                (i32::from(pixel[0]) * self.r_mul / 255 + self.r_add).clamp(0, 255) as u8,
                (i32::from(pixel[1]) * self.g_mul / 255 + self.g_add).clamp(0, 255) as u8,
                (i32::from(pixel[2]) * self.b_mul / 255 + self.b_add).clamp(0, 255) as u8,
                (i32::from(pixel[3]) * self.alpha / 255).clamp(0, 255) as u8,
            ];
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DrawCommand {
    pub texture_index: usize,
    pub vertices: Vec<[f64; 4]>,
}
#[derive(Debug, Deserialize)]
pub struct Shape {
    pub id: u32,
    pub commands: Vec<DrawCommand>,
}
#[derive(Debug, Deserialize)]
pub struct MovieClip {
    pub id: u32,
    pub children_ids: Vec<u32>,
    pub children_names: Vec<String>,
    pub children_blending: Vec<u8>,
    pub frame_elements_offset: u32,
    pub matrix_bank_index: usize,
    pub frame_element_counts: Vec<usize>,
    pub frame_labels: Vec<String>,
    pub framerate: u32,
}
#[derive(Debug, Deserialize)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub external: String,
}
#[derive(Debug, Deserialize)]
pub struct TextField {
    pub id: u32,
    pub font_name: String,
    pub text: String,
    #[serde(default)]
    pub left: f64,
    #[serde(default)]
    pub top: f64,
    #[serde(default)]
    pub right: f64,
    #[serde(default)]
    pub bottom: f64,
    #[serde(default)]
    pub font_color: u32,
    #[serde(default)]
    pub outline_color: u32,
    #[serde(default)]
    pub font_size: f32,
    #[serde(default)]
    pub align: u8,
    #[serde(default)]
    pub styles: u32,
}
#[derive(Debug, Deserialize)]
pub struct Document {
    pub source: String,
    pub sha256: String,
    pub exports: BTreeMap<String, u32>,
    pub movies: BTreeMap<u32, MovieClip>,
    pub shapes: Vec<Shape>,
    pub text_fields: BTreeMap<u32, TextField>,
    pub modifiers: BTreeMap<u32, u8>,
    pub matrix_banks: Vec<Vec<Matrix>>,
    pub color_banks: Vec<Vec<ColorTransform>>,
    pub frame_elements: Vec<u16>,
    pub textures: Vec<Texture>,
}

pub struct Scene {
    pub document: Document,
    pub document_path: PathBuf,
    textures: Vec<RgbaImage>,
    shape_index: BTreeMap<u32, Vec<usize>>,
}

#[derive(Default)]
pub struct Playback {
    /// A source-authored filter transform applied to the rendered root.
    pub root_transform: Option<Matrix>,
    pub root_color: Option<ColorTransform>,
    /// Explicit root frame, otherwise the root uses the selected label or frame 0.
    pub frame: Option<usize>,
    /// Select the same authored state label in every descendant that has it.
    pub label: Option<String>,
    /// Native code drives descendant states. These explicit selections expose
    /// that binding point without fabricating game state or automatic transitions.
    pub object_frames: BTreeMap<u32, usize>,
    /// Replace recovered text field contents by original text object id.
    pub text_values: BTreeMap<u32, String>,
    /// Omit these original objects and their descendants.
    pub hidden_objects: BTreeSet<u32>,
    /// Optional elapsed seconds for descendant timelines. Explicit frames and
    /// labels take precedence; root frame selection remains authoritative.
    pub time_seconds: Option<f64>,
    /// Draw text fields from recovered metadata and fonts. Defaults off.
    pub render_text: bool,
}

#[derive(Default, Debug, Serialize)]
pub struct RenderStats {
    pub shapes: usize,
    pub triangle_commands: usize,
    pub skipped_text_fields: BTreeSet<u32>,
    pub unsupported_blends: BTreeSet<u8>,
    pub invalid_references: usize,
    pub recursion_cycles: usize,
}

pub struct Rendered {
    pub image: RgbaImage,
    pub x: f64,
    pub y: f64,
    pub stats: RenderStats,
}

#[derive(Clone)]
struct Part {
    image: RgbaImage,
    x: f64,
    y: f64,
    blend: u8,
}

impl Scene {
    /// Draw a short spectator-owned label with a recovered game font.
    pub fn render_text(
        &self,
        text: &str,
        font_name: &str,
        size: f32,
        color: u32,
        outline: Option<u32>,
        align: u8,
        width: u32,
    ) -> Result<Rendered> {
        let field = TextField {
            id: 0,
            font_name: font_name.into(),
            text: text.into(),
            left: 0.,
            top: 0.,
            right: width.max(1) as f64,
            bottom: (size.max(1.) * 1.5).ceil() as f64,
            font_color: color,
            outline_color: outline.unwrap_or(0),
            font_size: size,
            align,
            styles: 0,
        };
        Ok(Rendered {
            image: self.render_text_field(&field, text)?,
            x: 0.,
            y: 0.,
            stats: RenderStats::default(),
        })
    }

    pub fn load(root: &Path, document: &str) -> Result<Self> {
        if Path::new(document).components().count() != 1 || document == "." || document == ".." {
            return Err("--document must be one recovered document name".into());
        }
        let document_path = root.join("display-trees").join(format!("{document}.json"));
        #[cfg(not(feature = "wasm"))]
        let document: Document = serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open(&document_path)?,
        ))?;
        #[cfg(feature = "wasm")]
        let document: Document = {
            let _ = &document_path;
            let bytes = crate::vfs::read(&format!("display-trees/{document}.json"))?;
            serde_json::from_slice(&bytes)?
        };
        let mut textures = Vec::new();
        for texture in &document.textures {
            let filename = Path::new(&texture.external)
                .file_stem()
                .ok_or("missing atlas name")?;
            let mut atlas_path = root.join("atlases").join(filename);
            atlas_path.set_extension("png");
            #[cfg(not(feature = "wasm"))]
            let atlas = image::open(&atlas_path)?.to_rgba8();
            #[cfg(feature = "wasm")]
            let atlas = {
                let name = atlas_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or("missing atlas name")?;
                let bytes = crate::vfs::read(&format!("atlases/{name}"))?;
                image::load_from_memory(&bytes)?.to_rgba8()
            };
            if atlas.dimensions() != (texture.width, texture.height) {
                return Err(format!(
                    "Atlas dimensions differ from recovered SC data: {}",
                    atlas_path.display()
                )
                .into());
            }
            textures.push(atlas);
        }
        let mut shape_index: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
        for (index, shape) in document.shapes.iter().enumerate() {
            shape_index.entry(shape.id).or_default().push(index);
        }
        Ok(Self {
            document,
            document_path,
            textures,
            shape_index,
        })
    }

    pub fn export_id(&self, export: &str) -> Result<u32> {
        self.document
            .exports
            .get(export)
            .copied()
            .ok_or_else(|| format!("Original export not present: {export}; use --list").into())
    }

    pub fn frame_count(&self, export: &str) -> Result<usize> {
        Ok(self
            .document
            .movies
            .get(&self.export_id(export)?)
            .map_or(1, |m| m.frame_element_counts.len().max(1)))
    }

    pub fn framerate(&self, export: &str) -> Result<u32> {
        Ok(self
            .document
            .movies
            .get(&self.export_id(export)?)
            .map_or(24, |m| m.framerate.max(1)))
    }

    pub fn render(&self, export: &str, playback: &Playback) -> Result<Option<Rendered>> {
        let id = self.export_id(export)?;
        if let Some(frame) = playback.frame {
            if frame >= self.frame_count(export)? {
                return Err(format!("Frame {frame} is outside the original root timeline").into());
            }
        }
        self.render_id(id, playback)
    }

    /// Render one recovered display object by its original id. Positions come
    /// from that object's own timeline, so a child rendered alone is not placed
    /// where its parent would have put it.
    pub fn render_id(&self, id: u32, playback: &Playback) -> Result<Option<Rendered>> {
        let mut stats = RenderStats::default();
        let parts = self.render_object(
            id,
            playback.root_transform.unwrap_or(Matrix::IDENTITY),
            playback.root_color.unwrap_or(ColorTransform::IDENTITY),
            &mut BTreeSet::new(),
            0,
            playback,
            true,
            &mut stats,
        )?;
        Ok(composite(parts)?.map(|p| Rendered {
            image: p.image,
            x: p.x,
            y: p.y,
            stats,
        }))
    }

    fn matrix(&self, movie: &MovieClip, index: u16) -> Matrix {
        if index == u16::MAX {
            return Matrix::IDENTITY;
        }
        self.document
            .matrix_banks
            .get(movie.matrix_bank_index)
            .and_then(|bank| bank.get(index as usize))
            .copied()
            .unwrap_or(Matrix::IDENTITY)
    }

    fn color(&self, movie: &MovieClip, index: u16) -> ColorTransform {
        if index == u16::MAX {
            return ColorTransform::IDENTITY;
        }
        self.document
            .color_banks
            .get(movie.matrix_bank_index)
            .and_then(|bank| bank.get(index as usize))
            .copied()
            .unwrap_or(ColorTransform::IDENTITY)
    }

    #[allow(clippy::too_many_arguments)]
    fn render_object(
        &self,
        id: u32,
        matrix: Matrix,
        color: ColorTransform,
        visited: &mut BTreeSet<u32>,
        depth: usize,
        playback: &Playback,
        root: bool,
        stats: &mut RenderStats,
    ) -> Result<Vec<Part>> {
        if playback.hidden_objects.contains(&id) {
            return Ok(Vec::new());
        }
        if depth > 50 || visited.contains(&id) {
            stats.recursion_cycles += 1;
            return Ok(Vec::new());
        }
        let mut parts = Vec::new();
        if let Some(indices) = self.shape_index.get(&id) {
            for &index in indices {
                let shape = &self.document.shapes[index];
                let mut commands = Vec::new();
                for command in &shape.commands {
                    let Some(atlas) = self.textures.get(command.texture_index) else {
                        stats.invalid_references += 1;
                        continue;
                    };
                    if let Some(p) = rasterize(command, atlas, matrix)? {
                        if p.image.pixels().any(|pixel| pixel[3] > 0) {
                            commands.push(p);
                            stats.triangle_commands += 1;
                        }
                    }
                }
                let shape_part = if commands.len() == 1 {
                    commands.pop()
                } else {
                    composite(commands)?
                };
                if let Some(mut p) = shape_part {
                    color.apply(&mut p.image);
                    if matrix.translation_only() {
                        p.x += matrix.tx;
                        p.y += matrix.ty;
                    }
                    parts.push(p);
                    stats.shapes += 1;
                }
            }
        }
        if let Some(field) = self.document.text_fields.get(&id) {
            if playback.render_text {
                let value = playback.text_values.get(&id).unwrap_or(&field.text);
                if !value.is_empty() {
                    let image = self.render_text_field(field, value)?;
                    if image.pixels().any(|p| p[3] > 0) {
                        parts.push(Part {
                            image,
                            x: matrix.tx + field.left,
                            y: matrix.ty + field.top,
                            blend: 0,
                        });
                    }
                }
            } else {
                stats.skipped_text_fields.insert(id);
            }
        }
        let Some(movie) = self.document.movies.get(&id) else {
            return Ok(parts);
        };
        visited.insert(id);
        let frame = playback
            .object_frames
            .get(&id)
            .copied()
            .or(if root { playback.frame } else { None })
            .or_else(|| {
                if !root && playback.label.is_none() {
                    playback.time_seconds.map(|t| {
                        (t.max(0.) * movie.framerate.max(1) as f64).floor() as usize
                            % movie.frame_element_counts.len().max(1)
                    })
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                playback
                    .label
                    .as_ref()
                    .and_then(|label| movie.frame_labels.iter().position(|s| s == label))
                    .unwrap_or(0)
            });
        let mut elements = Vec::new();
        if movie.frame_elements_offset == u32::MAX {
            elements.extend((0..movie.children_ids.len()).map(|i| [i as u16, u16::MAX, u16::MAX]));
        } else if let Some(&count) = movie.frame_element_counts.get(frame) {
            let start = movie.frame_elements_offset as usize
                + movie.frame_element_counts[..frame].iter().sum::<usize>() * 3;
            for i in 0..count {
                let offset = start + i * 3;
                if let Some(slice) = self.document.frame_elements.get(offset..offset + 3) {
                    elements.push([slice[0], slice[1], slice[2]]);
                } else {
                    stats.invalid_references += 1;
                    break;
                }
            }
        } else {
            stats.invalid_references += 1;
        }
        let mut capture_mask = false;
        let mut apply_mask = false;
        let mut mask: Option<Part> = None;
        for [child_index, matrix_index, color_index] in elements {
            let child_index = child_index as usize;
            let Some(&child_id) = movie.children_ids.get(child_index) else {
                stats.invalid_references += 1;
                continue;
            };
            match self.document.modifiers.get(&child_id) {
                Some(38) => {
                    capture_mask = true;
                    continue;
                }
                Some(39) => {
                    apply_mask = true;
                    continue;
                }
                Some(40) => {
                    apply_mask = false;
                    mask = None;
                    continue;
                }
                _ => {}
            }
            let child_matrix = matrix.compose(self.matrix(movie, matrix_index));
            let child_color = self.color(movie, color_index);
            // Preserve the independent Python reference's color selection.
            // Original native propagation remains an explicit parity question.
            let selected_color = if child_color == ColorTransform::IDENTITY {
                color
            } else {
                child_color
            };
            let mut child_parts = self.render_object(
                child_id,
                child_matrix,
                selected_color,
                visited,
                depth + 1,
                playback,
                false,
                stats,
            )?;
            let blend = movie
                .children_blending
                .get(child_index)
                .copied()
                .unwrap_or(0);
            if blend != 0 {
                if child_parts.len() > 1 {
                    if let Some(mut grouped) = composite(child_parts)? {
                        grouped.blend = blend;
                        child_parts = vec![grouped];
                    } else {
                        child_parts = Vec::new();
                    }
                } else if let Some(part) = child_parts.first_mut() {
                    part.blend = blend;
                }
                if blend == 1 || blend > 14 {
                    stats.unsupported_blends.insert(blend);
                }
            }
            if capture_mask {
                capture_mask = false;
                for p in &mut child_parts {
                    p.blend = 0;
                }
                mask = composite(child_parts)?;
                continue;
            }
            if apply_mask {
                if let Some(mask) = &mask {
                    parts.extend(
                        child_parts
                            .into_iter()
                            .filter_map(|p| clip_to_mask(p, mask)),
                    );
                } else {
                    parts.extend(child_parts);
                }
            } else {
                parts.extend(child_parts);
            }
        }
        visited.remove(&id);
        Ok(parts)
    }

    fn render_text_field(&self, field: &TextField, value: &str) -> Result<RgbaImage> {
        let width = (field.right - field.left).ceil().max(1.) as u32;
        let height = (field.bottom - field.top).ceil().max(1.) as u32;
        if width > 4096 || height > 4096 {
            return Err("Text field exceeds bounds".into());
        }
        let filename = font_filename(&field.font_name);
        let path = self
            .document_path
            .parent()
            .unwrap_or(Path::new("."))
            .parent()
            .unwrap_or(Path::new("."))
            .join("fonts")
            .join(filename);
        #[cfg(not(feature = "wasm"))]
        let bytes = std::fs::read(&path)
            .map_err(|e| format!("Cannot load recovered font {}: {e}", path.display()))?;
        #[cfg(feature = "wasm")]
        let bytes = crate::vfs::read(&format!("fonts/{filename}")).map_err(|e| {
            format!("Cannot load recovered font {}: {e}", path.display())
        })?;
        let font = fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default())
            .map_err(|e| format!("Invalid recovered font: {e}"))?;
        let size = field.font_size.max(1.);
        let mut image = RgbaImage::new(width, height);
        let color = argb(field.font_color);
        let outline = argb(field.outline_color);
        let scale = width as f32 / (field.right - field.left).max(1.) as f32;
        let glyphs: Vec<_> = value.chars().map(|ch| font.rasterize(ch, size)).collect();
        let total: f32 = glyphs.iter().map(|(m, _)| m.advance_width).sum();
        let mut x = match field.align {
            1 => (width as f32 - total * scale) / 2.,
            2 => width as f32 - total * scale,
            _ => 0.,
        }
        .max(0.);
        let baseline = ((height as f32 - size * 0.72) / 2.).max(0.);
        for (metrics, bitmap) in glyphs {
            let gx = x + metrics.xmin as f32 * scale;
            let gy = baseline + (size - metrics.height as f32 - metrics.ymin as f32) * scale;
            for py in 0..height {
                for px in 0..width {
                    let sx = ((px as f32 - gx) / scale).floor() as isize;
                    let sy = ((py as f32 - gy) / scale).floor() as isize;
                    if sx < 0
                        || sy < 0
                        || sx >= metrics.width as isize
                        || sy >= metrics.height as isize
                    {
                        continue;
                    }
                    let alpha = bitmap[sy as usize * metrics.width + sx as usize];
                    if alpha == 0 {
                        continue;
                    }
                    if outline[3] > 0
                        && (sx == 0
                            || sy == 0
                            || sx + 1 >= metrics.width as isize
                            || sy + 1 >= metrics.height as isize)
                    {
                        blend_over(
                            image.get_pixel_mut(px, py),
                            Rgba([
                                outline[0],
                                outline[1],
                                outline[2],
                                ((u16::from(alpha) * u16::from(outline[3])) / 255) as u8,
                            ]),
                        );
                    }
                    blend_over(
                        image.get_pixel_mut(px, py),
                        Rgba([
                            color[0],
                            color[1],
                            color[2],
                            ((u16::from(alpha) * u16::from(color[3])) / 255) as u8,
                        ]),
                    );
                }
            }
            x += metrics.advance_width * scale;
        }
        Ok(image)
    }
}

/// Rasterize text with an original recovered font, useful for spectator-owned
/// overlays such as a clock. Colors are ARGB, matching recovered metadata.
pub fn rasterize_text(
    root: &Path,
    font_name: &str,
    text: &str,
    size: f32,
    color: u32,
) -> Result<RgbaImage> {
    let filename = font_filename(font_name);
    #[cfg(not(feature = "wasm"))]
    let bytes = std::fs::read(root.join("fonts").join(filename))?;
    #[cfg(feature = "wasm")]
    let bytes = {
        let _ = root;
        crate::vfs::read(&format!("fonts/{filename}"))?
    };
    let font = fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default())
        .map_err(|e| format!("Invalid recovered font: {e}"))?;
    let glyphs: Vec<_> = text
        .chars()
        .map(|ch| font.rasterize(ch, size.max(1.)))
        .collect();
    let width = glyphs
        .iter()
        .map(|(m, _)| m.advance_width.ceil() as u32)
        .sum::<u32>()
        .max(1);
    let height = glyphs
        .iter()
        .map(|(m, _)| m.height as u32)
        .max()
        .unwrap_or(1)
        .max(1);
    let mut image = RgbaImage::new(width, height);
    let c = argb(color);
    let mut x = 0i32;
    for (m, b) in glyphs {
        let y = (height as i32 - m.height as i32) / 2 - m.ymin;
        for gy in 0..m.height {
            for gx in 0..m.width {
                let xx = x + m.xmin + gx as i32;
                let yy = y + gy as i32;
                if xx >= 0 && yy >= 0 && xx < width as i32 && yy < height as i32 {
                    let a = (u16::from(b[gy * m.width + gx]) * u16::from(c[3]) / 255) as u8;
                    blend_over(
                        image.get_pixel_mut(xx as u32, yy as u32),
                        Rgba([c[0], c[1], c[2], a]),
                    );
                }
            }
        }
        x += m.advance_width.ceil() as i32;
    }
    Ok(image)
}
fn font_filename(name: &str) -> &'static str {
    match name {
        "Supercell-Magic" => "supercell-magic_0.ttf",
        "SC CCBackBeat" | "CCBackBeat Light" => "sc_ccbackbeatregular.ttf",
        "Clash Title" => "clash_title_vi_th.ttf",
        "Clash Text" | "Clash Regular" | "Supercell Text Bold" => "clash_text_vi_th.ttf",
        "Noto Naskh Arabic" => "NotoNaskhArabic-Bold.ttf",
        "SGB" => "sgb_20170216.ttf",
        _ => "",
    }
}
fn argb(v: u32) -> [u8; 4] {
    [(v >> 16) as u8, (v >> 8) as u8, v as u8, (v >> 24) as u8]
}
fn blend_over(dst: &mut Rgba<u8>, src: Rgba<u8>) {
    let a = u16::from(src[3]);
    for c in 0..3 {
        dst[c] = ((u16::from(src[c]) * a + u16::from(dst[c]) * (255 - a)) / 255) as u8;
    }
    dst[3] = (a + u16::from(dst[3]) * (255 - a) / 255).min(255) as u8;
}

fn rasterize(command: &DrawCommand, atlas: &RgbaImage, matrix: Matrix) -> Result<Option<Part>> {
    if command.vertices.len() < 3 {
        return Ok(None);
    }
    let vertices: Vec<[f64; 4]> = command
        .vertices
        .iter()
        .map(|v| {
            let (x, y) = if matrix.translation_only() {
                (v[0], v[1])
            } else {
                matrix.transform(v[0], v[1])
            };
            [
                x,
                y,
                v[2] / 65535. * atlas.width() as f64,
                v[3] / 65535. * atlas.height() as f64,
            ]
        })
        .collect();
    let xmin = vertices.iter().map(|v| v[0]).fold(f64::INFINITY, f64::min);
    let ymin = vertices.iter().map(|v| v[1]).fold(f64::INFINITY, f64::min);
    let xmax = vertices
        .iter()
        .map(|v| v[0])
        .fold(f64::NEG_INFINITY, f64::max);
    let ymax = vertices
        .iter()
        .map(|v| v[1])
        .fold(f64::NEG_INFINITY, f64::max);
    let width = ((xmax - xmin + 1.5) as u32).max(1);
    let height = ((ymax - ymin + 1.5) as u32).max(1);
    if width > 4096 || height > 4096 {
        return Err(format!("Shape exceeds reference rasterizer bounds: {width}x{height}").into());
    }
    let mut image = RgbaImage::new(width, height);
    let mut filled = vec![false; (width * height) as usize];
    for index in 0..vertices.len() - 2 {
        let (i0, i1, i2) = if index % 2 == 0 {
            (index, index + 1, index + 2)
        } else {
            (index, index + 2, index + 1)
        };
        let mut triangle = [vertices[i0], vertices[i1], vertices[i2]];
        for v in &mut triangle {
            v[0] -= xmin;
            v[1] -= ymin;
        }
        let mut denominator = (triangle[1][1] - triangle[2][1]) * (triangle[0][0] - triangle[2][0])
            + (triangle[2][0] - triangle[1][0]) * (triangle[0][1] - triangle[2][1]);
        if denominator.abs() < 1e-6 {
            continue;
        }
        if denominator < 0. {
            triangle.swap(1, 2);
            denominator = -denominator;
        }
        let [v0, v1, v2] = triangle;
        let inverse_denominator = 1.0 / denominator;
        let min_y = (v0[1].min(v1[1]).min(v2[1]) as i32).max(0);
        let max_y = ((v0[1].max(v1[1]).max(v2[1]) + 0.5) as i32).min(height as i32 - 1);
        for py in min_y..=max_y {
            let mut intersections = Vec::with_capacity(3);
            for (start, end) in [(v0, v1), (v1, v2), (v2, v0)] {
                let y = py as f64;
                if start[1] != end[1]
                    && ((start[1] <= y && y < end[1]) || (end[1] <= y && y < start[1]))
                {
                    intersections.push(
                        start[0] + (y - start[1]) / (end[1] - start[1]) * (end[0] - start[0]),
                    );
                }
            }
            if intersections.len() < 2 {
                continue;
            }
            let min_x = (intersections.iter().copied().fold(f64::INFINITY, f64::min) as i32).max(0);
            let max_x = ((intersections
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max)
                + 0.5) as i32)
                .min(width as i32 - 1);
            for px in min_x..=max_x {
                let w0 = ((v1[1] - v2[1]) * (px as f64 - v2[0])
                    + (v2[0] - v1[0]) * (py as f64 - v2[1]))
                    * inverse_denominator;
                let w1 = ((v2[1] - v0[1]) * (px as f64 - v2[0])
                    + (v0[0] - v2[0]) * (py as f64 - v2[1]))
                    * inverse_denominator;
                let w2 = 1. - w0 - w1;
                let offset = (py as u32 * width + px as u32) as usize;
                let mut weights = [w0, w1, w2];
                if weights.iter().any(|w| *w < -0.01) {
                    if filled[offset] {
                        continue;
                    }
                    for weight in &mut weights {
                        *weight = weight.max(0.);
                    }
                    let sum = weights.iter().sum::<f64>();
                    if sum < 1e-6 {
                        continue;
                    }
                    for weight in &mut weights {
                        *weight /= sum;
                    }
                } else {
                    filled[offset] = true;
                }
                let u = weights[0] * v0[2] + weights[1] * v1[2] + weights[2] * v2[2];
                let v = weights[0] * v0[3] + weights[1] * v1[3] + weights[2] * v2[3];
                let tx = ((u + 0.5) as i32).clamp(0, atlas.width() as i32 - 1) as u32;
                let ty = ((v + 0.5) as i32).clamp(0, atlas.height() as i32 - 1) as u32;
                image.put_pixel(px as u32, py as u32, *atlas.get_pixel(tx, ty));
            }
        }
    }
    Ok(Some(Part {
        image,
        x: xmin,
        y: ymin,
        blend: 0,
    }))
}

fn clip_to_mask(part: Part, mask: &Part) -> Option<Part> {
    let (ix, iy, mx, my) = (part.x as i32, part.y as i32, mask.x as i32, mask.y as i32);
    let left = ix.max(mx);
    let top = iy.max(my);
    let right = (ix + part.image.width() as i32).min(mx + mask.image.width() as i32);
    let bottom = (iy + part.image.height() as i32).min(my + mask.image.height() as i32);
    if left >= right || top >= bottom {
        return None;
    }
    let mut image = RgbaImage::new((right - left) as u32, (bottom - top) as u32);
    let mut visible = false;
    for (x, y, target) in image.enumerate_pixels_mut() {
        let mut pixel = *part
            .image
            .get_pixel((left - ix) as u32 + x, (top - iy) as u32 + y);
        let alpha = mask
            .image
            .get_pixel((left - mx) as u32 + x, (top - my) as u32 + y)[3];
        pixel[3] = (u16::from(pixel[3]) * u16::from(alpha) / 255) as u8;
        visible |= pixel[3] > 0;
        *target = pixel;
    }
    visible.then_some(Part {
        image,
        x: left as f64,
        y: top as f64,
        blend: part.blend,
    })
}

fn composite(parts: Vec<Part>) -> Result<Option<Part>> {
    if parts.is_empty() {
        return Ok(None);
    }
    let left = parts
        .iter()
        .map(|p| p.x)
        .fold(f64::INFINITY, f64::min)
        .floor();
    let top = parts
        .iter()
        .map(|p| p.y)
        .fold(f64::INFINITY, f64::min)
        .floor();
    let right = parts
        .iter()
        .map(|p| p.x + p.image.width() as f64)
        .fold(f64::NEG_INFINITY, f64::max)
        .ceil();
    let bottom = parts
        .iter()
        .map(|p| p.y + p.image.height() as f64)
        .fold(f64::NEG_INFINITY, f64::max)
        .ceil();
    let (width, height) = ((right - left) as u32, (bottom - top) as u32);
    if width == 0 || height == 0 {
        return Ok(None);
    }
    if width > MAX_DIMENSION || height > MAX_DIMENSION {
        return Err(format!("Display object exceeds bounded canvas size: {width}x{height}").into());
    }
    let mut image = RgbaImage::new(width, height);
    for part in parts {
        let px = part.x.round_ties_even() as i32 - left as i32;
        let py = part.y.round_ties_even() as i32 - top as i32;
        for (x, y, pixel) in part.image.enumerate_pixels() {
            let (dx, dy) = (px + x as i32, py + y as i32);
            if dx >= 0 && dy >= 0 && dx < width as i32 && dy < height as i32 {
                blend(image.get_pixel_mut(dx as u32, dy as u32), pixel, part.blend);
            }
        }
    }
    Ok(Some(Part {
        image,
        x: left,
        y: top,
        blend: 0,
    }))
}

fn blend(base: &mut Rgba<u8>, source: &Rgba<u8>, mode: u8) {
    let (sa, ba) = (i32::from(source[3]), i32::from(base[3]));
    if mode == 0 || mode == 1 || mode == 2 || mode > 14 {
        if sa == 0 {
            return;
        }
        // Match the independent reference's Pillow 13 integer quantization.
        // Its straight-alpha operation uses seven guard bits. Provenance and
        // license are in THIRD_PARTY.md; original GPU parity is not implied.
        let alpha = sa * 255 + ba * (255 - sa);
        let coefficient = sa * 255 * 255 * 128 / alpha;
        let remainder = 255 * 128 - coefficient;
        let shift_div_255 = |value: i32| ((value >> 8) + value) >> 8;
        for c in 0..3 {
            let value = i32::from(source[c]) * coefficient + i32::from(base[c]) * remainder;
            base[c] = (shift_div_255(value + (128 << 7)) >> 7) as u8;
        }
        base[3] = shift_div_255(alpha + 128) as u8;
        return;
    }
    if mode == 11 {
        base[3] = (ba + sa * (255 - ba) / 255).min(255) as u8;
        return;
    }
    if mode == 12 {
        base[3] = (ba * (255 - sa) / 255) as u8;
        return;
    }
    for c in 0..3 {
        let (b, s) = (i32::from(base[c]), i32::from(source[c]));
        if mode == 8 {
            base[c] = (b + s * sa / 255).min(255) as u8;
            continue;
        }
        let blended = match mode {
            3 => b * s / 255,
            4 => b + s - b * s / 255,
            5 => b.max(s),
            6 => b.min(s),
            7 => (b - s).abs(),
            9 => (b - s).max(0),
            10 => 255 - b,
            13 if b < 128 => 2 * b * s / 255,
            14 if s < 128 => 2 * b * s / 255,
            13 | 14 => 255 - 2 * (255 - b) * (255 - s) / 255,
            _ => s,
        };
        base[c] = (b + ((blended - b) * sa).div_euclid(255)).clamp(0, 255) as u8;
    }
    base[3] = if mode == 8 {
        ba.max(sa)
    } else {
        ba + sa * (255 - ba) / 255
    }
    .clamp(0, 255) as u8;
}
