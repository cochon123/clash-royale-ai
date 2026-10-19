//! Presentation snapshots read from the recovered engine, without modifying it.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
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
}
