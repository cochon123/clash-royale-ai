//! Optional semantic trace of what the Painter decides to show, for offline
//! visual QA (`clash-visual-check`). Inactive unless a recording was started
//! on this thread, so live rendering only pays a thread-local check.
//!
//! The trace states presentation decisions (clip, frame, alpha, bounds,
//! effects, inferred impacts); it never reads or changes engine state.
use image::RgbaImage;
use serde::Serialize;
use std::cell::RefCell;

/// One entity as drawn in one display frame.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Shown {
    pub uid: u64,
    pub address: u64,
    pub native_id: Option<u64>,
    pub row: String,
    pub side: i32,
    /// `unit`, `projectile`, `king`, `princess`, `tower-destroyed` or
    /// `marker` (no original art bound: drawn as a fallback dot).
    pub kind: &'static str,
    /// Presentation state: `idle`, `moving`, `attacking` or `none`.
    pub state: &'static str,
    /// Original export of the main sprite and its authored frame index.
    pub export: String,
    pub frame: usize,
    pub flip: bool,
    pub deploying: bool,
    pub flash: Option<String>,
    /// Root color alpha multiplier applied to the main sprite (0..1).
    pub alpha: f32,
    /// Interpolated native position and health.
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub hp: i32,
    pub max_hp: i32,
    /// Screen pixel of the (elevated) entity origin.
    pub anchor: [f64; 2],
    /// Union of drawn part rectangles in screen pixels [x0, y0, x1, y1].
    pub bbox: Option<[f64; 4]>,
    /// Sum of alpha/255 over the main sprite's pixels: its visible area.
    pub coverage: f64,
    pub parts: usize,
}

/// One original effect resolved into the presentation.
#[derive(Clone, Debug, Serialize)]
pub struct Effect {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub red: bool,
    pub start_ms: f64,
    pub attach: Option<u64>,
    /// False when the name has no original effect definition (renders nothing).
    pub known: bool,
    pub components: usize,
    pub depth: u8,
}

/// An attack impact the Painter attributed to an actor (projectile birth or
/// melee HP loss); attack clips align their `action_frame` with these.
#[derive(Clone, Debug, Serialize)]
pub struct Impact {
    pub uid: u64,
    pub at_ms: f64,
}

#[derive(Debug, Default, Serialize)]
pub struct Record {
    pub shown: Vec<Shown>,
    pub effects: Vec<Effect>,
    pub impacts: Vec<Impact>,
}

thread_local! { static ACTIVE: RefCell<Option<Record>> = const { RefCell::new(None) }; }

/// Start recording on this thread, discarding any unfinished record.
pub fn start() {
    ACTIVE.with(|a| *a.borrow_mut() = Some(Record::default()));
}

/// Stop recording and return everything observed since `start`.
pub fn take() -> Record {
    ACTIVE.with(|a| a.borrow_mut().take()).unwrap_or_default()
}

pub fn active() -> bool {
    ACTIVE.with(|a| a.borrow().is_some())
}

pub fn with(f: impl FnOnce(&mut Record)) {
    ACTIVE.with(|a| {
        if let Some(record) = a.borrow_mut().as_mut() {
            f(record);
        }
    });
}

/// Visible area of a sprite: the sum of its alpha channel over 255.
pub fn coverage(image: &RgbaImage) -> f64 {
    image.pixels().map(|p| p.0[3] as u64).sum::<u64>() as f64 / 255.
}
