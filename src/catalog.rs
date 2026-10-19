use serde::{Deserialize, Serialize};

pub type CardId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardKind {
    Troop,
    Spell,
    Building,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Target {
    Ground,
    AirGround,
    Buildings,
}

/// All distances use 1000 units per tile; speed is units per second.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub elixir: u8,
    pub kind: CardKind,
    pub hp: i32,
    pub damage: i32,
    pub attack_interval_ms: u32,
    pub range: i32,
    pub speed: i32,
    pub count: u8,
    pub target: Target,
    pub flying: bool,
    pub radius: i32,
    pub splash_radius: i32,
    pub lifetime_ms: u32,
    pub deploy_ms: u32,
    #[serde(default)]
    pub can_deploy_enemy_side: bool,
    #[serde(default)]
    pub kamikaze: bool,
    #[serde(default)]
    pub support_status: String,
    #[serde(default)]
    pub limitations: Vec<String>,
    #[serde(default)]
    pub crown_tower_damage: Option<i32>,
    #[serde(default)]
    pub crown_tower_damage_percent_raw: Option<i32>,
}

pub fn card_catalog() -> Vec<Card> {
    serde_json::from_str(include_str!("../data/cards.json"))
        .expect("checked-in card catalog must match its documented schema")
}

pub fn default_deck() -> Vec<CardId> {
    [
        "knight",
        "archers",
        "giant",
        "minions",
        "skeletons",
        "musketeer",
        "mini_pekka",
        "fireball",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

pub fn tower_catalog() -> Vec<Card> {
    serde_json::from_str(include_str!("../data/towers.json"))
        .expect("checked-in tower catalog must match its documented schema")
}
