use crate::catalog::{card_catalog, default_deck, tower_catalog, Card, CardKind, Target};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt};

pub const TICK_MS: u32 = 50;
pub const ARENA_WIDTH: i32 = 18_000;
pub const ARENA_HEIGHT: i32 = 32_000;
pub const RIVER_Y: i32 = 16_000;
pub const BRIDGES: [i32; 2] = [4_500, 13_500];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Team {
    Blue,
    Red,
}
impl Team {
    pub fn index(self) -> usize {
        if self == Self::Blue {
            0
        } else {
            1
        }
    }
    pub fn opponent(self) -> Self {
        if self == Self::Blue {
            Self::Red
        } else {
            Self::Blue
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}
impl Vec2 {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn distance_squared(self, other: Self) -> i64 {
        let dx = i64::from(self.x) - i64::from(other.x);
        let dy = i64::from(self.y) - i64::from(other.y);
        dx * dx + dy * dy
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityKind {
    Troop,
    Building,
    PrincessTower,
    KingTower,
}
impl EntityKind {
    pub fn is_tower(self) -> bool {
        matches!(self, Self::PrincessTower | Self::KingTower)
    }
    pub fn is_building(self) -> bool {
        self != Self::Troop
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: u64,
    pub team: Team,
    pub kind: EntityKind,
    pub pos: Vec2,
    pub hp: i32,
    pub max_hp: i32,
    pub card_id: Option<String>,
    pub flying: bool,
    pub active: bool,
    pub radius: i32,
    pub damage: i32,
    pub range: i32,
    pub speed: i32,
    pub target: Target,
    pub splash_radius: i32,
    pub cooldown_ticks: u64,
    pub attack_interval_ticks: u64,
    pub ready_tick: u64,
    pub expires_tick: Option<u64>,
    pub target_id: Option<u64>,
    pub kamikaze: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub hand: Vec<String>,
    pub queue: VecDeque<String>,
    pub elixir_milli: i32,
    #[serde(default)]
    regen_remainder: u64,
}
impl Player {
    pub fn next_card(&self) -> Option<&str> {
        self.queue.front().map(String::as_str)
    }
    pub fn elixir(&self) -> f32 {
        self.elixir_milli as f32 / 1000.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinishReason {
    KingDestroyed,
    Crowns,
    SuddenDeath,
    Tiebreaker,
    Draw,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outcome {
    pub winner: Option<Team>,
    pub reason: FinishReason,
    pub crowns: [u8; 2],
}
impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.winner {
            Some(team) => write!(f, "{team:?} wins ({:?})", self.reason),
            None => write!(f, "Draw"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Played {
        team: Team,
        card_id: String,
        pos: Vec2,
    },
    Spawned {
        id: u64,
    },
    Attack {
        attacker: u64,
        target: u64,
        from: Vec2,
        to: Vec2,
    },
    Spell {
        team: Team,
        card_id: String,
        pos: Vec2,
        radius: i32,
    },
    Damaged {
        id: u64,
        amount: i32,
    },
    Destroyed {
        id: u64,
        team: Team,
        kind: EntityKind,
    },
    Finished(Outcome),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleConfig {
    pub regulation_ms: u32,
    pub overtime_ms: u32,
    pub initial_elixir_milli: i32,
    pub elixir_interval_ms: u32,
}
impl Default for BattleConfig {
    fn default() -> Self {
        Self {
            regulation_ms: 180_000,
            overtime_ms: 120_000,
            initial_elixir_milli: 5000,
            elixir_interval_ms: 2800,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayError {
    MatchFinished,
    InvalidHandSlot,
    InsufficientElixir,
    OutsideArena,
    EnemyTerritory,
    InvalidDeck(String),
    InvalidConfig,
}
impl fmt::Display for PlayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MatchFinished => write!(f, "The match has finished"),
            Self::InvalidHandSlot => write!(f, "Select one of the four cards in hand"),
            Self::InsufficientElixir => write!(f, "Not enough elixir"),
            Self::OutsideArena => write!(f, "Deploy inside the arena"),
            Self::EnemyTerritory => {
                write!(f, "Deploy troops and buildings on your side of the river")
            }
            Self::InvalidDeck(s) => write!(f, "Invalid deck: {s}"),
            Self::InvalidConfig => write!(f, "Invalid match configuration"),
        }
    }
}
impl std::error::Error for PlayError {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Action {
    pub team: Team,
    pub hand_index: usize,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct LegalCard<'a> {
    pub id: &'a str,
    pub elixir: u8,
    pub playable: bool,
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerObservation<'a> {
    pub team: Team,
    pub elixir_milli: i32,
    pub hand: Vec<LegalCard<'a>>,
    pub next_card: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EntityObservation<'a> {
    pub id: u64,
    pub team: Team,
    pub kind: EntityKind,
    pub card_id: Option<&'a str>,
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub flying: bool,
    pub active: bool,
    pub ready_in_ticks: u64,
    pub attack_in_ticks: u64,
}

/// Structured observation for external agents. Full battle state serialization
/// also exists for replay/debugging; this form excludes catalog and RNG internals.
#[derive(Debug, Clone, Serialize)]
pub struct Observation<'a> {
    pub tick: u64,
    pub elapsed_ms: u64,
    pub remaining_ms: u64,
    pub overtime: bool,
    pub players: [PlayerObservation<'a>; 2],
    pub entities: Vec<EntityObservation<'a>>,
    pub crowns: [u8; 2],
    pub outcome: Option<Outcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PendingSpell {
    team: Team,
    card_id: String,
    pos: Vec2,
    ready_tick: u64,
}

/// A deterministic fixed-step reconstruction. Movement, collision-free troop
/// spacing, targeting, fallback spell tower damage, elixir, and tie breaking are
/// locally implemented approximations, not claimed to match a server binary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battle {
    pub catalog: Vec<Card>,
    pub entities: Vec<Entity>,
    pub players: [Player; 2],
    pub tick: u64,
    pub outcome: Option<Outcome>,
    pub events: Vec<Event>,
    pub config: BattleConfig,
    pub overtime: bool,
    spells: Vec<PendingSpell>,
    next_id: u64,
    rng: u64,
}

fn ticks(ms: u32) -> u64 {
    u64::from(ms).div_ceil(u64::from(TICK_MS))
}
fn integer_sqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

impl Battle {
    pub fn new(seed: u64) -> Self {
        Self::with_decks(
            seed,
            [default_deck(), default_deck()],
            BattleConfig::default(),
        )
        .expect("default deck is valid")
    }

    pub fn with_decks(
        seed: u64,
        decks: [Vec<String>; 2],
        config: BattleConfig,
    ) -> Result<Self, PlayError> {
        let catalog = card_catalog();
        if config.elixir_interval_ms == 0
            || config.regulation_ms == 0
            || !(0..=10_000).contains(&config.initial_elixir_milli)
        {
            return Err(PlayError::InvalidConfig);
        }
        for deck in &decks {
            if deck.len() != 8 {
                return Err(PlayError::InvalidDeck("eight cards required".into()));
            }
            for (i, id) in deck.iter().enumerate() {
                if !catalog
                    .iter()
                    .any(|c| &c.id == id && c.support_status != "unsupported")
                {
                    return Err(PlayError::InvalidDeck(format!(
                        "unknown or unsupported card {id}"
                    )));
                }
                if deck[..i].contains(id) {
                    return Err(PlayError::InvalidDeck(format!("duplicate card {id}")));
                }
            }
        }
        let make_player = |deck: &Vec<String>| Player {
            hand: deck[..4].to_vec(),
            queue: deck[4..].iter().cloned().collect(),
            elixir_milli: config.initial_elixir_milli,
            regen_remainder: 0,
        };
        let players = [make_player(&decks[0]), make_player(&decks[1])];
        let mut battle = Self {
            catalog,
            entities: vec![],
            players,
            tick: 0,
            outcome: None,
            events: vec![],
            config,
            overtime: false,
            spells: vec![],
            next_id: 1,
            rng: seed.max(1),
        };
        for team in [Team::Blue, Team::Red] {
            let princess_y = if team == Team::Blue { 25_500 } else { 6500 };
            let king_y = if team == Team::Blue { 29_500 } else { 2500 };
            let towers = tower_catalog();
            for (id, pos, kind) in [
                (
                    "princess_tower",
                    Vec2::new(4500, princess_y),
                    EntityKind::PrincessTower,
                ),
                (
                    "princess_tower",
                    Vec2::new(13500, princess_y),
                    EntityKind::PrincessTower,
                ),
                ("king_tower", Vec2::new(9000, king_y), EntityKind::KingTower),
            ] {
                let card = towers
                    .iter()
                    .find(|c| c.id == id)
                    .expect("tower catalog contains both towers");
                let mut e = battle.make_entity(team, card, pos, kind);
                e.active = kind != EntityKind::KingTower;
                e.ready_tick = 0;
                battle.entities.push(e);
            }
        }
        Ok(battle)
    }

    pub fn card(&self, id: &str) -> Option<&Card> {
        self.catalog.iter().find(|c| c.id == id)
    }
    pub fn apply_action(&mut self, action: Action) -> Result<(), PlayError> {
        self.play(
            action.team,
            action.hand_index,
            Vec2::new(action.x, action.y),
        )
    }
    pub fn legal_hand(&self, team: Team) -> Vec<LegalCard<'_>> {
        self.players[team.index()]
            .hand
            .iter()
            .map(|id| {
                let c = self.card(id).expect("validated deck");
                let anywhere = c.kind == CardKind::Spell || c.can_deploy_enemy_side;
                LegalCard {
                    id,
                    elixir: c.elixir,
                    playable: self.outcome.is_none()
                        && self.players[team.index()].elixir_milli >= i32::from(c.elixir) * 1000,
                    x_min: 500,
                    x_max: ARENA_WIDTH - 500,
                    y_min: if anywhere || team == Team::Red {
                        500
                    } else {
                        17_500
                    },
                    y_max: if anywhere || team == Team::Blue {
                        ARENA_HEIGHT - 500
                    } else {
                        14_500
                    },
                }
            })
            .collect()
    }
    pub fn observation(&self) -> Observation<'_> {
        let player = |team| PlayerObservation {
            team,
            elixir_milli: self.players[team.index()].elixir_milli,
            hand: self.legal_hand(team),
            next_card: self.players[team.index()].next_card(),
        };
        Observation {
            tick: self.tick,
            elapsed_ms: self.elapsed_ms(),
            remaining_ms: self.remaining_ms(),
            overtime: self.overtime,
            players: [player(Team::Blue), player(Team::Red)],
            crowns: self.crowns(),
            outcome: self.outcome,
            entities: self
                .entities
                .iter()
                .map(|e| EntityObservation {
                    id: e.id,
                    team: e.team,
                    kind: e.kind,
                    card_id: e.card_id.as_deref(),
                    x: e.pos.x,
                    y: e.pos.y,
                    hp: e.hp,
                    max_hp: e.max_hp,
                    flying: e.flying,
                    active: e.active,
                    ready_in_ticks: e.ready_tick.saturating_sub(self.tick),
                    attack_in_ticks: e.cooldown_ticks.saturating_sub(self.tick),
                })
                .collect(),
        }
    }
    pub fn elapsed_ms(&self) -> u64 {
        self.tick * u64::from(TICK_MS)
    }
    pub fn remaining_ms(&self) -> u64 {
        let end = u64::from(self.config.regulation_ms)
            + if self.overtime {
                u64::from(self.config.overtime_ms)
            } else {
                0
            };
        end.saturating_sub(self.elapsed_ms())
    }
    pub fn crowns(&self) -> [u8; 2] {
        let mut crowns = [0; 2];
        for team in [Team::Blue, Team::Red] {
            let opponent = team.opponent();
            crowns[team.index()] = if self
                .entities
                .iter()
                .any(|e| e.team == opponent && e.kind == EntityKind::KingTower && e.hp <= 0)
            {
                3
            } else {
                self.entities
                    .iter()
                    .filter(|e| {
                        e.team == opponent && e.kind == EntityKind::PrincessTower && e.hp <= 0
                    })
                    .count() as u8
            };
        }
        crowns
    }

    fn make_entity(&mut self, team: Team, card: &Card, pos: Vec2, kind: EntityKind) -> Entity {
        let id = self.next_id;
        self.next_id += 1;
        Entity {
            id,
            team,
            kind,
            pos,
            hp: card.hp,
            max_hp: card.hp,
            card_id: Some(card.id.clone()),
            flying: card.flying,
            active: true,
            radius: card.radius,
            damage: card.damage,
            range: card.range,
            speed: card.speed,
            target: card.target,
            splash_radius: card.splash_radius,
            cooldown_ticks: 0,
            attack_interval_ticks: ticks(card.attack_interval_ms).max(1),
            ready_tick: self.tick + ticks(card.deploy_ms),
            expires_tick: if card.lifetime_ms > 0 {
                Some(self.tick + ticks(card.deploy_ms) + ticks(card.lifetime_ms))
            } else {
                None
            },
            target_id: None,
            kamikaze: card.kamikaze,
        }
    }

    pub fn play(&mut self, team: Team, hand_index: usize, pos: Vec2) -> Result<(), PlayError> {
        if self.outcome.is_some() {
            return Err(PlayError::MatchFinished);
        }
        let id = self.players[team.index()]
            .hand
            .get(hand_index)
            .ok_or(PlayError::InvalidHandSlot)?
            .clone();
        let card = self.card(&id).expect("validated deck").clone();
        if pos.x < 500 || pos.x > ARENA_WIDTH - 500 || pos.y < 500 || pos.y > ARENA_HEIGHT - 500 {
            return Err(PlayError::OutsideArena);
        }
        if card.kind != CardKind::Spell
            && !card.can_deploy_enemy_side
            && ((team == Team::Blue && pos.y < 17_500) || (team == Team::Red && pos.y > 14_500))
        {
            return Err(PlayError::EnemyTerritory);
        }
        if self.players[team.index()].elixir_milli < i32::from(card.elixir) * 1000 {
            return Err(PlayError::InsufficientElixir);
        }
        let player = &mut self.players[team.index()];
        player.elixir_milli -= i32::from(card.elixir) * 1000;
        player.hand[hand_index] = player.queue.pop_front().expect("four cards remain queued");
        player.queue.push_back(id.clone());
        self.events.push(Event::Played {
            team,
            card_id: id.clone(),
            pos,
        });
        if card.kind == CardKind::Spell {
            self.spells.push(PendingSpell {
                team,
                card_id: id,
                pos,
                ready_tick: self.tick + ticks(card.deploy_ms),
            });
        } else {
            let count = i32::from(card.count.max(1));
            for n in 0..count {
                let offset = (n * 2 - (count - 1)) * 220;
                let spawn = Vec2::new((pos.x + offset).clamp(500, ARENA_WIDTH - 500), pos.y);
                let kind = if card.kind == CardKind::Building {
                    EntityKind::Building
                } else {
                    EntityKind::Troop
                };
                let e = self.make_entity(team, &card, spawn, kind);
                self.events.push(Event::Spawned { id: e.id });
                self.entities.push(e);
            }
        }
        Ok(())
    }

    fn can_target(attacker: &Entity, target: &Entity) -> bool {
        target.hp > 0
            && target.team != attacker.team
            && match attacker.target {
                Target::AirGround => true,
                Target::Ground => !target.flying,
                Target::Buildings => target.kind.is_building(),
            }
    }
    fn in_range(attacker: &Entity, target: &Entity) -> bool {
        let reach = i64::from(attacker.range + attacker.radius + target.radius);
        attacker.pos.distance_squared(target.pos) <= reach * reach
    }

    fn target_for(&self, attacker: &Entity) -> Option<usize> {
        if let Some(id) = attacker.target_id {
            if let Some((i, target)) = self.entities.iter().enumerate().find(|(_, e)| e.id == id) {
                let sight = i64::from(6500 + attacker.radius + target.radius);
                if Self::can_target(attacker, target)
                    && attacker.pos.distance_squared(target.pos) <= sight * sight
                {
                    return Some(i);
                }
            }
        }
        let targets = self
            .entities
            .iter()
            .enumerate()
            .filter(|(_, e)| Self::can_target(attacker, e));
        let visible = targets.clone().filter(|(_, e)| {
            let sight = i64::from(5500 + attacker.radius + e.radius);
            attacker.pos.distance_squared(e.pos) <= sight * sight || Self::in_range(attacker, e)
        });
        visible
            .min_by_key(|(_, e)| (attacker.pos.distance_squared(e.pos), e.id))
            .map(|(i, _)| i)
            .or_else(|| {
                if attacker.kind == EntityKind::Troop {
                    targets
                        .filter(|(_, e)| e.kind.is_tower())
                        .min_by_key(|(_, e)| (attacker.pos.distance_squared(e.pos), e.id))
                        .map(|(i, _)| i)
                } else {
                    None
                }
            })
    }

    /// Ground troops route through a bridge center and exit bank before pursuing
    /// a target. Air troops can cross the river directly. Collision avoidance and
    /// original pathfinding are intentionally outside this reconstruction.
    pub fn movement_waypoint(entity: &Entity, target: Vec2) -> Vec2 {
        if entity.flying {
            return target;
        }
        let crossing = (entity.pos.y > RIVER_Y && target.y < 17_000)
            || (entity.pos.y < RIVER_Y && target.y > 15_000);
        let in_river = (15_000..=17_000).contains(&entity.pos.y);
        if !crossing && !in_river {
            return target;
        }
        let bridge = *BRIDGES
            .iter()
            .min_by_key(|x| {
                (entity.pos.x - **x).abs() + if in_river { 0 } else { (target.x - **x).abs() }
            })
            .unwrap();
        if entity.pos.x != bridge {
            let bank = if entity.pos.y >= RIVER_Y {
                17_100
            } else {
                14_900
            };
            return Vec2::new(bridge, if in_river { entity.pos.y } else { bank });
        }
        Vec2::new(bridge, if target.y < RIVER_Y { 14_900 } else { 17_100 })
    }

    fn move_towards(pos: Vec2, target: Vec2, distance: i32) -> Vec2 {
        let length = integer_sqrt(pos.distance_squared(target));
        if length <= i64::from(distance) {
            return target;
        }
        let dx = i64::from(target.x - pos.x) * i64::from(distance) / length;
        let dy = i64::from(target.y - pos.y) * i64::from(distance) / length;
        Vec2::new(pos.x + dx as i32, pos.y + dy as i32)
    }

    pub fn step(&mut self) {
        if self.outcome.is_some() {
            return;
        }
        self.events.clear();
        self.tick += 1;
        let interval = u64::from(self.config.elixir_interval_ms);
        // Double elixir in the last minute and overtime; integer carry preserves
        // the exact rate independent of frame rate or platform float behavior.
        let multiplier = if self.elapsed_ms() + 60_000 >= u64::from(self.config.regulation_ms) {
            2
        } else {
            1
        };
        for player in &mut self.players {
            player.regen_remainder += u64::from(TICK_MS) * 1000 * multiplier;
            let gained = player.regen_remainder / interval;
            player.regen_remainder %= interval;
            player.elixir_milli = (player.elixir_milli + gained as i32).min(10_000);
        }
        let mut hits: Vec<(u64, i32)> = Vec::new();
        let mut remaining_spells = Vec::new();
        for spell in std::mem::take(&mut self.spells) {
            if spell.ready_tick > self.tick {
                remaining_spells.push(spell);
                continue;
            }
            let card = self.card(&spell.card_id).expect("spell exists in catalog");
            let radius = card.splash_radius.max(card.radius);
            for e in &self.entities {
                let reach = i64::from(radius + e.radius);
                let legal_target = match card.target {
                    Target::AirGround => true,
                    Target::Ground => !e.flying,
                    Target::Buildings => e.kind.is_building(),
                };
                if e.team != spell.team
                    && legal_target
                    && e.hp > 0
                    && e.pos.distance_squared(spell.pos) <= reach * reach
                {
                    let damage = if e.kind.is_tower() {
                        card.crown_tower_damage.unwrap_or_else(|| {
                            card.damage
                                * card
                                    .crown_tower_damage_percent_raw
                                    .map(|n| (100 + n).clamp(0, 100))
                                    .unwrap_or(30)
                                / 100
                        })
                    } else {
                        card.damage
                    };
                    hits.push((e.id, damage));
                }
            }
            self.events.push(Event::Spell {
                team: spell.team,
                card_id: spell.card_id,
                pos: spell.pos,
                radius,
            });
        }
        self.spells = remaining_spells;

        // Decisions use one immutable snapshot; damage is applied simultaneously.
        let mut actions = Vec::new();
        for (i, e) in self.entities.iter().enumerate() {
            if e.hp <= 0 || !e.active || e.ready_tick > self.tick {
                continue;
            }
            if e.expires_tick.is_some_and(|t| self.tick >= t) {
                hits.push((e.id, e.hp));
                continue;
            }
            if let Some(target_index) = self.target_for(e) {
                let target = &self.entities[target_index];
                if Self::in_range(e, target) {
                    if e.cooldown_ticks <= self.tick {
                        self.events.push(Event::Attack {
                            attacker: e.id,
                            target: target.id,
                            from: e.pos,
                            to: target.pos,
                        });
                        if e.splash_radius > 0 {
                            for victim in &self.entities {
                                let radius = i64::from(e.splash_radius + victim.radius);
                                if Self::can_target(e, victim)
                                    && target.pos.distance_squared(victim.pos) <= radius * radius
                                {
                                    hits.push((victim.id, e.damage));
                                }
                            }
                        } else {
                            hits.push((target.id, e.damage));
                        }
                        if e.kamikaze {
                            hits.push((e.id, e.hp));
                        }
                        actions.push((
                            i,
                            e.pos,
                            Some(target.id),
                            Some(self.tick + e.attack_interval_ticks),
                        ));
                    } else {
                        actions.push((i, e.pos, Some(target.id), None));
                    }
                } else if e.kind == EntityKind::Troop {
                    let waypoint = Self::movement_waypoint(e, target.pos);
                    let distance = (i64::from(e.speed) * i64::from(TICK_MS) / 1000) as i32;
                    let pos = Self::move_towards(e.pos, waypoint, distance);
                    actions.push((i, pos, Some(target.id), None));
                }
            }
        }
        for (i, pos, target_id, cooldown) in actions {
            self.entities[i].pos = pos;
            self.entities[i].target_id = target_id;
            if let Some(cooldown) = cooldown {
                self.entities[i].cooldown_ticks = cooldown;
            }
        }
        for (id, damage) in hits {
            if let Some(e) = self.entities.iter_mut().find(|e| e.id == id) {
                if e.hp <= 0 {
                    continue;
                }
                e.hp = (e.hp - damage.max(0)).max(0);
                if e.kind == EntityKind::KingTower && !e.active {
                    e.active = true;
                    // 3300 ms activation delay recovered from the king's
                    // action state; all subsequent combat is local rules.
                    e.ready_tick = self.tick + ticks(3300);
                }
                self.events.push(Event::Damaged { id, amount: damage });
                if e.hp == 0 {
                    self.events.push(Event::Destroyed {
                        id,
                        team: e.team,
                        kind: e.kind,
                    });
                }
            }
        }
        for team in [Team::Blue, Team::Red] {
            if self
                .entities
                .iter()
                .any(|e| e.team == team && e.kind == EntityKind::PrincessTower && e.hp == 0)
            {
                for e in &mut self.entities {
                    if e.team == team && e.kind == EntityKind::KingTower && !e.active {
                        e.active = true;
                        e.ready_tick = self.tick + ticks(3300);
                    }
                }
            }
        }
        self.entities.retain(|e| e.hp > 0 || e.kind.is_tower());
        self.check_outcome();
    }

    fn finish(&mut self, winner: Option<Team>, reason: FinishReason) {
        let outcome = Outcome {
            winner,
            reason,
            crowns: self.crowns(),
        };
        self.outcome = Some(outcome);
        self.events.push(Event::Finished(outcome));
    }
    fn check_outcome(&mut self) {
        let crowns = self.crowns();
        if crowns[0] == 3 || crowns[1] == 3 {
            let winner = if crowns[0] == crowns[1] {
                None
            } else if crowns[0] > crowns[1] {
                Some(Team::Blue)
            } else {
                Some(Team::Red)
            };
            self.finish(winner, FinishReason::KingDestroyed);
            return;
        }
        if self.overtime && crowns[0] != crowns[1] {
            self.finish(
                Some(if crowns[0] > crowns[1] {
                    Team::Blue
                } else {
                    Team::Red
                }),
                FinishReason::SuddenDeath,
            );
            return;
        }
        if !self.overtime && self.elapsed_ms() >= u64::from(self.config.regulation_ms) {
            if crowns[0] != crowns[1] {
                self.finish(
                    Some(if crowns[0] > crowns[1] {
                        Team::Blue
                    } else {
                        Team::Red
                    }),
                    FinishReason::Crowns,
                );
                return;
            }
            self.overtime = true;
        }
        if self.overtime
            && self.elapsed_ms()
                >= u64::from(self.config.regulation_ms) + u64::from(self.config.overtime_ms)
        {
            let min_hp = |team| {
                self.entities
                    .iter()
                    .filter(|e| e.team == team && e.kind.is_tower() && e.hp > 0)
                    .map(|e| e.hp)
                    .min()
                    .unwrap_or(0)
            };
            let blue = min_hp(Team::Blue);
            let red = min_hp(Team::Red);
            match blue.cmp(&red) {
                std::cmp::Ordering::Greater => {
                    self.finish(Some(Team::Blue), FinishReason::Tiebreaker)
                }
                std::cmp::Ordering::Less => self.finish(Some(Team::Red), FinishReason::Tiebreaker),
                std::cmp::Ordering::Equal => self.finish(None, FinishReason::Draw),
            }
        }
    }

    fn random(&mut self) -> u64 {
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng = x;
        x
    }

    /// Simple reproducible practice opponent, evaluated once each simulated
    /// second. It spends available elixir and alternates bridge lanes.
    pub fn bot_step(&mut self, team: Team) {
        if self.outcome.is_some() || !self.tick.is_multiple_of(20) {
            return;
        }
        let start = (self.random() % 4) as usize;
        for offset in 0..4 {
            let slot = (start + offset) % 4;
            let id = &self.players[team.index()].hand[slot];
            let card = self.card(id).unwrap();
            if i32::from(card.elixir) * 1000 > self.players[team.index()].elixir_milli {
                continue;
            }
            let spell = card.kind == CardKind::Spell;
            let lane = if self.random() & 1 == 0 {
                BRIDGES[0]
            } else {
                BRIDGES[1]
            };
            let own_y = if team == Team::Blue { 23_000 } else { 9000 };
            let pos = if spell {
                self.entities
                    .iter()
                    .filter(|e| e.team != team && e.hp > 0 && e.kind == EntityKind::Troop)
                    .max_by_key(|e| (e.hp, std::cmp::Reverse(e.id)))
                    .map(|e| e.pos)
                    .unwrap_or(Vec2::new(
                        lane,
                        if team == Team::Blue { 6500 } else { 25_500 },
                    ))
            } else {
                Vec2::new(lane, own_y)
            };
            let _ = self.play(team, slot, pos);
            break;
        }
    }
}
