//! Presentation snapshots read from the recovered engine, without modifying it.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Entity {
    pub address: u64,
    pub row: String,
    pub side: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub moving: bool,
    pub attacking: bool,
    /// Exact position deltas; used as a presentation heading while moving.
    #[serde(default)]
    pub motion_dx: Option<i64>,
    #[serde(default)]
    pub motion_dy: Option<i64>,
    #[serde(default)]
    pub first_seen_tick: Option<u32>,
    #[serde(default)]
    pub native_entity_id: Option<u64>,
    #[serde(default)]
    pub native_focus_entity: Option<u64>,
    #[serde(default)]
    pub native_focus_entity_id: Option<u64>,
    #[serde(default)]
    pub native_action_timer_raw: Option<i32>,
    #[serde(default)]
    pub native_secondary_timer_raw: Option<i32>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HandCard {
    pub slot: u32,
    pub card_id: u32,
    pub name: String,
    pub cost: u32,
    pub available: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CardPlay {
    pub id: u64,
    pub side: i32,
    pub card_id: u32,
    pub name: String,
    pub cost: u32,
    pub x: i32,
    pub y: i32,
    pub tick: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardSelection {
    pub slot: u32,
    pub card_id: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HumanTarget {
    pub slot: u32,
    pub card_id: u32,
    pub x: i32,
    pub y: i32,
    pub valid: bool,
    pub affordable: bool,
    pub cost: u32,
    /// Own-side-only card aimed past the player's territory; the engine will
    /// move it back. A presentation hint from original card data.
    #[serde(default)]
    pub snaps: bool,
    /// Where the card will be submitted; differs from x/y when it snaps.
    #[serde(default)]
    pub land_x: Option<i32>,
    #[serde(default)]
    pub land_y: Option<i32>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HumanFrame {
    pub side: usize,
    pub selection: Option<CardSelection>,
    pub target: Option<HumanTarget>,
    pub last_serial: u64,
    pub paused: bool,
    pub feedback: String,
    /// Native-unit rectangles [x0, y0, x1, y1] the selected card cannot land in.
    pub forbidden: Vec<[i32; 4]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BattleFrame {
    pub tick: u32,
    pub elapsed_ms: u64,
    pub terminated: bool,
    pub winner_side: Option<i32>,
    pub arena_size: [i32; 2],
    pub elixir: [f64; 2],
    pub entities: Vec<Entity>,
    #[serde(default)]
    pub hands: [Vec<HandCard>; 2],
    #[serde(default)]
    pub controller_names: [String; 2],
    #[serde(default)]
    pub plays: Vec<CardPlay>,
    #[serde(default)]
    pub match_id: u64,
    #[serde(default)]
    pub human: Option<HumanFrame>,
}
