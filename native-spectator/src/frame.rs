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
}
