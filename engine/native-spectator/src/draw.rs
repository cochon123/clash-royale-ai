//! Read-only battle renderer. All character, arena, effect and decoration
//! artwork is loaded from the original SC5 bundle; spectator-owned text uses
//! a font extracted from that same bundle.
//!
//! Native snapshots arrive at 20 ticks per second. `observe` turns each new
//! snapshot into presentation state (movement/attack phases, effects) and
//! `draw` renders any fractional tick between the two latest snapshots, so the
//! window can run at display rate with interpolated motion and animation.
use crate::{
    actors::{self, Actor, TICK_MS},
    animation::{self, AnimationState, KingLayerOrder},
    blit::{self, Affine, Blend, ShadowMask},
    frame::{BattleFrame, Entity},
    hud::{HandHud, HandLayout},
    input::{ControlLayout, PlayerInput},
    presentation::Presentation,
    pose_cache::{self, PoseCache},
    scene,
    vfx::{self, Anchor, Layer, Vfx, VfxSprite},
};
use clash_native_ui_viewer::{ColorTransform, Matrix, Playback, Rendered, Scene};
use image::{imageops, Rgba, RgbaImage};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::{Path, PathBuf};
use std::rc::Rc;

type Result<T> = std::result::Result<T, String>;

const BLUE: Rgba<u8> = Rgba([58, 147, 255, 255]);
const RED: Rgba<u8> = Rgba([238, 72, 65, 255]);
const DARK: Rgba<u8> = Rgba([29, 34, 42, 242]);
const WHITE: u32 = 0xffffffff;
/// Native elevation to SC pixels. The arena maps a 1000-unit tile to 30 px
/// across and 24 px deep, i.e. an orthographic camera pitched so depth is
/// foreshortened by 0.8; heights then project at 0.6 x 30 px per tile. The
/// original art agrees: the princess (`AttachedCharacterHeight` 2200) stands
/// on her platform, tower arrows (`ProjectileStartZ` 3000) leave her bow, the
/// King cannonball (3500) leaves the turret 57 px up and the Mortar shell
/// (2500) its barrel mouth.
const Z_TO_SC: f64 = 0.018;
/// Displayed turret turn rate for `HasRotationOnTimeline` buildings.
const TURRET_DEG_PER_S: f64 = 540.;
/// Snapshots retained for interpolation; must exceed the display lag.
const HISTORY: usize = 40;
/// TrainingCamp location `ShadowA` (60) as a fraction, slightly softened
/// because silhouettes are composited without the client's blur.
const SHADOW_ALPHA: f32 = 0.5;

#[derive(Clone)]
struct Visual {
    document: String,
    prefix: String,
    red_prefix: Option<String>,
    scale_percent: f64,
}

/// Document, export, frame, playback variant, scale bits, mirrored.
type FitKey = (String, String, usize, u64, u64, bool);

/// A resized sprite whose top-left corner is `(dx, dy)` from its anchor.
struct Fitted {
    image: RgbaImage,
    dx: i64,
    dy: i64,
}

#[derive(Clone)]
struct Part {
    sprite: Rc<Fitted>,
    x: f64,
    y: f64,
}

struct ShadowSpec {
    part: usize,
    /// Unfiltered silhouette while a deploy filter lifts the sprite, so the
    /// shadow stays on the ground under the dropping unit.
    source: Option<Part>,
    scale: [f64; 2],
    skew: f64,
    offset: [f64; 2],
}

#[derive(Clone, Copy, PartialEq)]
enum BarKind {
    Small,
    Medium,
    High,
    Tower,
}

struct HpBar {
    kind: BarKind,
    red: bool,
    hp: i32,
    max_hp: i32,
    x: f64,
    top: f64,
}

struct Plan {
    uid: u64,
    parts: Vec<Part>,
    shadow: Option<ShadowSpec>,
    ground_parts: Vec<Part>,
    sort_y: f64,
    air: bool,
    hp: Option<HpBar>,
    /// Projected entity or projectile point, shared by body parts.
    screen_anchor: [f64; 2],
    /// Ground point shared by the shadow and ground-layer projectile art.
    ground_anchor: [f64; 2],
}

/// A troop or tower that was planned for the current display tick.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShownSubject {
    pub row: String,
    pub team: i32,
    pub state: &'static str,
    pub heading: u8,
    pub projectile: bool,
    pub tower: bool,
}

/// One `--cards` argument: comma-separated native names, spaces kept.
pub fn parse_card_list(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect()
}

/// `None` means the whole bundle. A non-empty card list never falls back to it.
fn documents_for_cards(manifest: &Value, cards: &[String]) -> Result<Option<BTreeSet<String>>> {
    if cards.is_empty() {
        return Ok(None);
    }
    let scene = &manifest["scene"];
    let Some(always) = scene["always_documents"].as_array() else {
        return Err(
            "--cards requires scene.always_documents; refusing to load the full bundle".into(),
        );
    };
    let Some(deps) = scene["card_dependency_documents"].as_object() else {
        return Err(
            "--cards requires scene.card_dependency_documents; refusing to load the full bundle"
                .into(),
        );
    };
    if always.is_empty() {
        return Err("scene.always_documents is empty".into());
    }
    let missing: Vec<_> = cards
        .iter()
        .filter(|card| !deps.contains_key(card.as_str()))
        .cloned()
        .collect();
    if !missing.is_empty() {
        return Err(format!(
            "native cards absent from scene.card_dependency_documents: {}",
            missing.join(", ")
        ));
    }
    let mut set = BTreeSet::new();
    for doc in always {
        if let Some(name) = doc.as_str() {
            set.insert(name.to_owned());
        }
    }
    for card in cards {
        let Some(list) = deps.get(card).and_then(Value::as_array) else {
            return Err(format!("native card {card} has no document list"));
        };
        for doc in list {
            if let Some(name) = doc.as_str() {
                set.insert(name.to_owned());
            }
        }
    }
    if set.is_empty() {
        return Err("card selection produced no original documents".into());
    }
    Ok(Some(set))
}

#[derive(Clone, Copy)]
struct Layout {
    field_x: f64,
    field_y: f64,
    field_w: f64,
    field_h: f64,
    world_w: f64,
    world_h: f64,
    scale: f64,
}

impl Layout {
    fn ground(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.field_x + x / self.world_w * self.field_w,
            // Native side 0 is blue and uses the lower half of the board.
            self.field_y + ((1. - y / self.world_h) * 768. + 60.) / 828. * self.field_h,
        )
    }
    fn project(&self, x: f64, y: f64, z_native: f64) -> (f64, f64) {
        let (sx, sy) = self.ground(x, y);
        (sx, sy - z_native * Z_TO_SC * self.scale)
    }
}

pub struct Painter {
    root: PathBuf,
    width: u32,
    height: u32,
    scenes: BTreeMap<String, Scene>,
    bindings: BTreeMap<String, Visual>,
    sprite_cache: BTreeMap<(String, String, usize, u64), Option<Rc<Rendered>>>,
    /// Fitted poses already composited by an earlier fight or match.
    pose_cache: Option<PoseCache>,
    deco_cache: BTreeMap<String, RgbaImage>,
    text_cache: BTreeMap<(String, u32, u32, u8), RgbaImage>,
    arena_field: Option<RgbaImage>,
    field_cache: Option<RgbaImage>,
    sprite_fit_cache: BTreeMap<FitKey, Rc<Fitted>>,
    font_scene: String,
    pub unresolved_rows: BTreeSet<String>,
    assets: Value,
    hand_hud: HandHud,
    logic: Presentation,
    vfx: Vfx,
    shadows: ShadowMask,
    history: VecDeque<BattleFrame>,
    actors: BTreeMap<u64, Actor>,
    /// Actors removed by the latest snapshots, still drawn while the display
    /// clock interpolates toward the snapshot that removed them.
    departed: BTreeMap<u64, (u32, Actor)>,
    destroyed: BTreeSet<u64>,
    next_uid: u64,
    king_activations: BTreeMap<u64, u32>,
    pending_detach: Vec<(u64, f64)>,
    /// Impact times (ms) for one replay, keyed by native address. Filled by
    /// `anticipate` so a wind-up can start before the damage tick.
    scheduled_impacts: BTreeMap<u64, Vec<f64>>,
    scheduled_match: Option<u64>,
}

impl Painter {
    /// Decode every loaded document's atlas pages ahead of the first draw.
    pub fn prewarm_atlases(&self) -> Result<()> {
        for (name, scene) in &self.scenes {
            scene
                .prewarm_atlases()
                .map_err(|e| format!("prewarm {name}: {e}"))?;
        }
        Ok(())
    }

    /// Remember fitted sprites under `root` so a later match does not composite
    /// the same walk, attack, spell or health-bar frame again.
    pub fn enable_pose_cache(&mut self, root: &Path) -> Result<()> {
        let cache = PoseCache::open(root)?;
        self.vfx.enable_pose_cache(cache.clone());
        self.pose_cache = Some(cache);
        Ok(())
    }

    pub fn load(root: &Path, width: u32, height: u32) -> Result<Self> {
        Self::load_for_cards(root, width, height, &[])
    }

    /// Load `always_documents` plus each named card's dependency documents.
    /// An empty list keeps the previous whole-bundle load.
    pub fn load_for_cards(root: &Path, width: u32, height: u32, cards: &[String]) -> Result<Self> {
        #[cfg(not(feature = "wasm"))]
        let manifest_bytes =
            std::fs::read(root.join("assets.json")).map_err(|e| e.to_string())?;
        #[cfg(feature = "wasm")]
        let manifest_bytes = {
            let _ = root;
            clash_native_ui_viewer::vfs::take("assets.json")?
        };
        let manifest: Value =
            serde_json::from_slice(&manifest_bytes).map_err(|e| e.to_string())?;
        let selected = documents_for_cards(&manifest, cards)?;
        if let Some(set) = &selected {
            let present: BTreeSet<&str> = manifest["documents"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|doc| doc["name"].as_str())
                .collect();
            let missing: Vec<_> = set
                .iter()
                .filter(|name| !present.contains(name.as_str()))
                .cloned()
                .collect();
            if !missing.is_empty() {
                return Err(format!(
                    "selected original documents are not in the bundle: {}",
                    missing.join(", ")
                ));
            }
        }
        let mut scenes = BTreeMap::new();
        let mut exports = BTreeMap::new();
        if let Some(docs) = manifest["documents"].as_array() {
            for doc in docs {
                let Some(name) = doc["name"].as_str() else {
                    continue;
                };
                if selected.as_ref().is_some_and(|set| !set.contains(name)) {
                    continue;
                }
                let scene = Scene::load(root, name).map_err(|e| e.to_string())?;
                let set: BTreeSet<String> = scene.document.exports.keys().cloned().collect();
                exports.insert(name.to_owned(), set);
                scenes.insert(name.to_owned(), scene);
            }
        }
        let has_export = |doc: &str, prefix: &str| {
            exports.get(doc).is_some_and(|set: &BTreeSet<String>| {
                set.contains(prefix) || set.iter().any(|e| e.starts_with(prefix))
            })
        };
        let mut bindings = BTreeMap::new();
        // Exact SC5 documents and export-family stems referenced by recovered
        // logic rows or observed cards. Export selection is checked against the
        // source export table at draw time.
        for (row, doc, prefix, red_prefix) in [
            (
                "KingTower",
                "building_tower",
                "KingTower_blue",
                "KingTower_red",
            ),
            (
                "PrincessTower",
                "building_tower",
                "StarTower_base_blue",
                "StarTower_base_red",
            ),
        ] {
            if has_export(doc, prefix) {
                bindings.insert(
                    row.to_owned(),
                    Visual {
                        document: doc.into(),
                        prefix: prefix.into(),
                        red_prefix: Some(red_prefix.into()),
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
            if has_export(doc, prefix) {
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
                if !has_export(doc, blue) {
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
        let logic = Presentation::load(root)?;
        // Original logic rows name exact exports for every projectile and
        // spell missile, including rows absent from the observed bindings
        // (Musketeer shots, cannonballs, Fireball, Log, Ice Spirits).
        for (name, row) in &logic.rows {
            if bindings.contains_key(name) || !row.is_projectile() {
                continue;
            }
            let (Some(file), Some(export)) = (row.str("FileName"), row.str("ExportName")) else {
                continue;
            };
            let doc = Path::new(file)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_owned();
            if !exports.get(&doc).is_some_and(|set| set.contains(export)) {
                continue;
            }
            let red = row
                .str("RedExportName")
                .filter(|red| exports.get(&doc).is_some_and(|set| set.contains(*red)));
            bindings.insert(
                name.clone(),
                Visual {
                    document: doc,
                    prefix: export.to_owned(),
                    red_prefix: red.map(str::to_owned),
                    scale_percent: row.num("Scale").unwrap_or(100.),
                },
            );
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
            let crop_y = (-60.0 - rendered.y).round() as i64;
            if crop_x < 0
                || crop_y < 0
                || crop_x + 540 > rendered.image.width() as i64
                || crop_y + 828 > rendered.image.height() as i64
            {
                return Err(format!("calibrated arena crop is outside original background bounds: origin ({}, {}), image {}x{}", rendered.x, rendered.y, rendered.image.width(), rendered.image.height()));
            }
            Some(
                imageops::crop_imm(&rendered.image, crop_x as u32, crop_y as u32, 540, 828)
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
            bindings,
            sprite_cache: BTreeMap::new(),
            pose_cache: None,
            deco_cache: BTreeMap::new(),
            text_cache: BTreeMap::new(),
            arena_field,
            field_cache: None,
            sprite_fit_cache: BTreeMap::new(),
            font_scene,
            unresolved_rows: BTreeSet::new(),
            assets: manifest,
            hand_hud: HandHud::default(),
            logic,
            vfx: Vfx::default(),
            shadows: ShadowMask::new(width.max(1), height.max(1)),
            history: VecDeque::new(),
            actors: BTreeMap::new(),
            departed: BTreeMap::new(),
            destroyed: BTreeSet::new(),
            next_uid: 1,
            king_activations: BTreeMap::new(),
            pending_detach: Vec::new(),
            scheduled_impacts: BTreeMap::new(),
            scheduled_match: None,
        })
    }

    /// Observe one snapshot and render it exactly (no interpolation).
    pub fn render(&mut self, frame: &BattleFrame) -> Result<RgbaImage> {
        self.observe(frame);
        self.draw(frame.tick as f64)
    }

    pub fn latest_tick(&self) -> Option<u32> {
        self.history.back().map(|f| f.tick)
    }

    pub fn oldest_tick(&self) -> Option<u32> {
        self.history.front().map(|f| f.tick)
    }

    pub fn snapshot_at(&self, display_tick: f64) -> Option<BattleFrame> {
        self.history
            .iter()
            .rev()
            .find(|frame| frame.tick as f64 <= display_tick)
            .cloned()
            .or_else(|| self.history.front().cloned())
    }

    pub fn shown_subjects(&self) -> Vec<ShownSubject> {
        self.actors
            .values()
            .filter(|actor| actor.shown)
            .map(|actor| ShownSubject {
                row: actor.row.clone(),
                team: actor.side,
                state: actor.shown_state,
                heading: actor.shown_heading,
                projectile: actor.shown_projectile,
                tower: matches!(actor.row.as_str(), "KingTower" | "PrincessTower"),
            })
            .collect()
    }

    pub fn particle_count(&self) -> usize {
        self.vfx.particle_count()
    }

    /// Decoded atlas bytes and how many scenes are loaded. The VFS remainder
    /// is reported separately; wasm linear memory does not shrink after a free.
    pub fn memory_profile(&self) -> (usize, usize) {
        let atlas = self
            .scenes
            .values()
            .map(|scene| scene.atlas_rgba_bytes())
            .sum();
        (atlas, self.scenes.len())
    }

    /// New round: drop actors, history and the display-clock inputs. Loaded
    /// documents stay. The wasm `painter_reset` entry point calls this.
    pub fn reset_round(&mut self) {
        self.reset_presentation();
        self.scheduled_impacts.clear();
        self.scheduled_match = None;
        self.next_uid = 1;
        self.unresolved_rows.clear();
    }

    fn reset_presentation(&mut self) {
        self.history.clear();
        self.actors.clear();
        self.departed.clear();
        self.destroyed.clear();
        self.king_activations.clear();
        self.pending_detach.clear();
        self.vfx.clear();
    }

    fn is_projectile(&self, row: &str) -> bool {
        self.logic.row(row).is_some_and(|r| r.is_projectile())
            || self
                .row_meta(row)
                .and_then(|v| v["source_row"].as_str())
                .is_some_and(|v| v.starts_with("PROJECTILE."))
            || self.assets["scene"]["projectile_bindings"]
                .as_array()
                .is_some_and(|v| v.iter().any(|v| v["native_row"] == row))
    }

    fn row_meta(&self, row: &str) -> Option<&Value> {
        self.assets["scene"]["observed_entity_bindings"]
            .as_array()?
            .iter()
            .find(|v| v["native_row"] == row)
    }

    /// Cadence and reach of an attacking row, from its original logic.
    /// Returns (hit speed ms, load time ms, range, own radius, ranged).
    fn attack_profile(&self, row: &str) -> Option<(f64, f64, f64, f64, bool)> {
        if row == "KingTower" {
            return None;
        }
        let logic = self.logic.row(row)?;
        let hit = logic.num("HitSpeed").filter(|h| *h > 0.)?;
        Some((
            hit,
            logic.num("LoadTime").unwrap_or(hit * 0.5),
            logic.num("Range").unwrap_or(1000.),
            logic.num("CollisionRadius").unwrap_or(500.),
            logic.str("Projectile").is_some(),
        ))
    }

    fn radius(&self, row: &str) -> f64 {
        self.logic
            .row(row)
            .and_then(|r| r.num("CollisionRadius"))
            .unwrap_or(500.)
    }

    fn row_str(&self, row: &str, key: &str) -> Option<String> {
        self.logic
            .row(row)
            .and_then(|r| r.str(key))
            .map(str::to_owned)
    }

    /// Process one native snapshot into presentation state and effects.
    pub fn observe(&mut self, frame: &BattleFrame) {
        if let Some(last) = self.history.back() {
            if last.match_id != frame.match_id || frame.tick < last.tick {
                self.reset_presentation();
            } else if last.tick == frame.tick {
                // A producer may republish the same tick with fresh HUD data.
                *self.history.back_mut().expect("history") = frame.clone();
                return;
            }
        }
        let first = self.history.is_empty();
        let tick = frame.tick;
        let now_ms = tick as f64 * TICK_MS;
        let by_addr: BTreeMap<u64, &Entity> =
            frame.entities.iter().map(|e| (e.address, e)).collect();

        // Removals, including addresses reused by a different native object.
        let mut removed = Vec::new();
        let stale: Vec<u64> = self
            .actors
            .iter()
            .filter(|(addr, actor)| !by_addr.get(addr).is_some_and(|e| actor.matches(e)))
            .map(|(addr, _)| *addr)
            .collect();
        for addr in stale {
            if let Some(actor) = self.actors.remove(&addr) {
                removed.push(actor);
            }
        }
        let mut spawned = Vec::new();
        let mut hp_drops = Vec::new();
        for entity in &frame.entities {
            if let Some(actor) = self.actors.get_mut(&entity.address) {
                if entity.hp >= 0 && entity.hp < actor.last.hp {
                    hp_drops.push((entity.address, actor.last.hp));
                }
                actor.observe(entity, tick);
            } else {
                let uid = self.next_uid;
                self.next_uid += 1;
                self.actors
                    .insert(entity.address, Actor::new(uid, entity, tick, !first));
                spawned.push(entity.address);
            }
        }

        // Engagement: stationary, a native focus target, within original reach.
        let profiles: BTreeMap<u64, (f64, f64, f64, f64, bool)> = self
            .actors
            .iter()
            .filter_map(|(addr, a)| self.attack_profile(&a.row).map(|p| (*addr, p)))
            .collect();
        let radii: BTreeMap<u64, f64> = frame
            .entities
            .iter()
            .map(|e| (e.address, self.radius(&e.row)))
            .collect();
        for (addr, actor) in self.actors.iter_mut() {
            let Some(&(hit, load, range, own, ranged)) = profiles.get(addr) else {
                continue;
            };
            let target = actor
                .last
                .native_focus_entity
                .and_then(|t| by_addr.get(&t))
                .filter(|t| t.side != actor.side && t.hp != 0);
            let in_range = target.is_some_and(|t| {
                let dx = (t.x - actor.last.x) as f64;
                let dy = (t.y - actor.last.y) as f64;
                (dx * dx + dy * dy).sqrt()
                    <= range + own + radii.get(&t.address).copied().unwrap_or(500.) + 400.
            });
            let deploy = self
                .logic
                .row(&actor.row)
                .and_then(|r| r.num("DeployTime"))
                .unwrap_or(0.);
            let age_ms = now_ms - actor.spawn_tick as f64 * TICK_MS;
            let ready = !actor.deploys || age_ms >= deploy;
            // Ranged units wind up only after a real stop. The four-tick gaps
            // between pathing steps are shorter than MOVE_HOLD_TICKS, so they
            // do not start a shot. Melee units start the swing as soon as the
            // target is in reach, which puts the wind-up on screen before the hit.
            let aiming = !ranged || actor.still_ticks(tick) >= actors::MOVE_HOLD_TICKS;
            let engaged = actor.last.hp != 0 && ready && in_range && aiming;
            actor.set_engaged(engaged, now_ms, hit, load);
        }

        // Projectile births: attribute to the nearest same-side shooter whose
        // original `Projectile` field names this row.
        for addr in &spawned {
            let Some(projectile) = by_addr.get(addr).copied() else {
                continue;
            };
            if !self.is_projectile(&projectile.row) {
                continue;
            }
            let shooter = self
                .actors
                .iter()
                .filter(|(a, actor)| {
                    *a != addr
                        && actor.side == projectile.side
                        && actor.last.max_hp > 0
                        && self.row_str(&actor.row, "Projectile").as_deref()
                            == Some(projectile.row.as_str())
                })
                .map(|(a, actor)| {
                    let dx = (actor.last.x - projectile.x) as f64;
                    let dy = (actor.last.y - projectile.y) as f64;
                    (dx * dx + dy * dy, *a)
                })
                .filter(|(d, _)| *d < 5000. * 5000.)
                .min_by(|a, b| a.0.total_cmp(&b.0))
                .map(|(_, a)| a);
            let red = projectile.side != 0;
            if let Some(shooter) = shooter {
                let hit = profiles.get(&shooter).map_or(1000., |p| p.0);
                let (sx, sy, shooter_row) = {
                    let a = &self.actors[&shooter];
                    (a.last.x as f64, a.last.y as f64, a.row.clone())
                };
                if let Some(actor) = self.actors.get_mut(&shooter) {
                    actor.register_impact(now_ms - TICK_MS * 0.5, hit);
                }
                if let Some(p) = self.actors.get_mut(addr) {
                    p.origin = Some([sx, sy, projectile.z as f64]);
                }
                if let Some(effect) = self.row_str(&shooter_row, "ProjectileEffect") {
                    self.vfx.spawn(
                        &effect,
                        Anchor {
                            x: sx,
                            y: sy,
                            z: projectile.z as f64 * Z_TO_SC,
                        },
                        red,
                        1.,
                        now_ms - TICK_MS * 0.5,
                        0.,
                        None,
                    );
                }
            }
            if let Some(trail) = self.row_str(&projectile.row, "TrailEffect") {
                let uid = self.actors[addr].uid;
                self.vfx.spawn(
                    &trail,
                    Anchor {
                        x: projectile.x as f64,
                        y: projectile.y as f64,
                        z: projectile.z as f64 * Z_TO_SC,
                    },
                    red,
                    1.,
                    now_ms,
                    0.,
                    Some(uid),
                );
            }
        }

        // Melee impacts: a target's HP loss is attributed to engaged melee
        // attackers focused on it. A unit swings at most once per `HitSpeed`
        // and each swing removes its original `Damage` scaled to its level
        // (observed max HP over original `Hitpoints`), so arrows, spells and
        // other units hitting the same target do not restart its swing.
        for (target_addr, before) in &hp_drops {
            let Some(target) = by_addr.get(target_addr).copied() else {
                continue;
            };
            let loss = (before - target.hp.max(0)) as f64;
            let candidates: Vec<MeleeCandidate> = self
                .actors
                .iter()
                .filter(|(_, a)| {
                    a.side != target.side
                        && a.last.native_focus_entity == Some(*target_addr)
                        && a.engaged_since_ms.is_some()
                })
                .filter_map(|(addr, a)| {
                    let (hit, _, _, _, ranged) = *profiles.get(addr)?;
                    if ranged
                        || a.impacts
                            .last()
                            .is_some_and(|last| now_ms - last < hit * 0.75)
                    {
                        return None;
                    }
                    let damage = self.logic.row(&a.row).and_then(|r| {
                        let base = r.num("Hitpoints").filter(|h| *h > 0.)?;
                        Some(r.num("Damage")? * a.last.max_hp as f64 / base)
                    });
                    let error = a.predicted_ms.map_or(hit, |p| (p - now_ms).abs().min(hit));
                    Some(MeleeCandidate {
                        addr: *addr,
                        hit,
                        damage,
                        on_time: error <= hit * 0.35,
                    })
                })
                .collect();
            let matched = attribute_loss(loss, &candidates);
            let mut push_from = None;
            for MeleeCandidate {
                addr: attacker,
                hit,
                ..
            } in matched
            {
                let (ax, ay, row) = {
                    let a = &self.actors[&attacker];
                    (a.last.x, a.last.y, a.row.clone())
                };
                if let Some(a) = self.actors.get_mut(&attacker) {
                    a.register_impact(now_ms - TICK_MS * 0.5, hit);
                }
                push_from = Some((ax, ay));
                if let Some(effect) = self.row_str(&row, "DamageEffect") {
                    self.vfx.spawn(
                        &effect,
                        Anchor {
                            x: target.x as f64,
                            y: target.y as f64,
                            z: 12.,
                        },
                        target.side == 0,
                        1.,
                        now_ms,
                        0.,
                        None,
                    );
                }
            }
            // Original directional damage filters push away from the hit.
            if self
                .logic
                .row(&target.row)
                .and_then(|r| r.str("DamageExportName"))
                .is_some()
            {
                let (dx, dy) = push_from
                    .map(|(ax, ay)| ((target.x - ax) as f64, -(target.y - ay) as f64))
                    .unwrap_or((0., if target.side == 0 { 1. } else { -1. }));
                let dir = if dx.abs() > dy.abs() {
                    if dx > 0. {
                        "right"
                    } else {
                        "left"
                    }
                } else if dy > 0. {
                    "down"
                } else {
                    "up"
                };
                let export = format!("filter_damage_{dir}");
                if let Some(a) = self.actors.get_mut(target_addr) {
                    a.add_flash(export, now_ms);
                }
            }
        }

        // Tower destruction can leave a zero-HP native object in place.
        for (addr, _) in &hp_drops {
            let Some(actor) = self.actors.get(addr) else {
                continue;
            };
            if actor.last.hp == 0
                && matches!(actor.row.as_str(), "KingTower" | "PrincessTower")
                && self.destroyed.insert(actor.uid)
            {
                let (x, y, side, row) = (actor.last.x, actor.last.y, actor.side, actor.row.clone());
                self.spawn_death(&row, x as f64, y as f64, 0., side, now_ms);
            }
        }

        // Removed objects: projectile impacts and unit deaths.
        for actor in removed {
            // Trails follow the projectile until the display reaches its end.
            self.pending_detach.push((actor.uid, now_ms));
            let e = &actor.last;
            let red = actor.side != 0;
            if self.is_projectile(&actor.row) {
                // Continue one tick along the observed path to the impact.
                let x = e.x as f64 + actor.velocity[0];
                let y = e.y as f64 + actor.velocity[1];
                let z = (e.z as f64 + actor.velocity[2]).max(0.);
                let heading = actor.velocity[0].atan2(actor.velocity[1]).to_degrees();
                for key in ["HitEffect", "DeathEffect"] {
                    if let Some(effect) = self.row_str(&actor.row, key) {
                        self.vfx.spawn(
                            &effect,
                            Anchor {
                                x,
                                y,
                                z: z * Z_TO_SC,
                            },
                            red,
                            1.,
                            now_ms,
                            heading,
                            None,
                        );
                    }
                }
            } else if e.max_hp > 0 && self.destroyed.insert(actor.uid) {
                let (row, x, y) = (actor.row.clone(), e.x as f64, e.y as f64);
                self.spawn_death(&row, x, y, e.z as f64, actor.side, now_ms);
            }
            self.departed.insert(e.address, (tick, actor));
        }
        self.departed
            .retain(|_, (gone, _)| tick.saturating_sub(*gone) <= HISTORY as u32);

        // Spawns: original deploy effects and area-effect visuals.
        if !first {
            for addr in &spawned {
                let Some(e) = by_addr.get(addr).copied() else {
                    continue;
                };
                let red = e.side != 0;
                let anchor = Anchor {
                    x: e.x as f64,
                    y: e.y as f64,
                    z: 0.,
                };
                let Some(row) = self.logic.row(&e.row) else {
                    continue;
                };
                let names: Vec<String> = if row.is_area_effect() {
                    ["OneShotEffect", "ScaledEffect", "Effect", "LoopingEffect"]
                        .iter()
                        .filter_map(|k| row.str(k).map(str::to_owned))
                        .collect()
                } else if e.max_hp > 0 {
                    row.str("SpawnEffect")
                        .map(str::to_owned)
                        .into_iter()
                        .collect()
                } else {
                    Vec::new()
                };
                for name in names {
                    self.vfx.spawn(&name, anchor, red, 1., now_ms, 0., None);
                }
            }
        }

        // The original tower logic activates on King damage or an allied
        // Princess tower's destruction. Trigger its source movie from those
        // observations; this is not a recovered native activation flag.
        let previous = self.history.back();
        let fallen_sides: BTreeSet<i32> = previous
            .into_iter()
            .flat_map(|p| p.entities.iter())
            .filter(|e| {
                e.row == "PrincessTower"
                    && !frame.entities.iter().any(|now| now.address == e.address)
            })
            .map(|e| e.side)
            .chain(
                frame
                    .entities
                    .iter()
                    .filter(|e| e.row == "PrincessTower" && e.hp == 0)
                    .map(|e| e.side),
            )
            .collect();
        for king in frame
            .entities
            .iter()
            .filter(|e| e.row == "KingTower" && e.max_hp > 0)
        {
            if king.hp < king.max_hp || fallen_sides.contains(&king.side) {
                self.king_activations.entry(king.address).or_insert(tick);
            }
        }

        self.history.push_back(frame.clone());
        while self.history.len() > HISTORY {
            self.history.pop_front();
        }
    }

    /// Replay impact attribution for one match without spawning effects.
    /// Attack clips and Royal Ghost visibility then begin `LoadTime` before
    /// the damage tick. Uses the same engagement and damage rules as `observe`.
    pub fn anticipate(&mut self, frames: &[BattleFrame]) {
        self.scheduled_impacts.clear();
        self.scheduled_match = None;
        let Some(first) = frames.first() else {
            return;
        };
        let match_id = first.match_id;
        let mut actors: BTreeMap<u64, Actor> = BTreeMap::new();
        let mut next_uid = 1u64;
        let mut scheduled: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
        for (index, frame) in frames.iter().enumerate() {
            if frame.match_id != match_id {
                break;
            }
            let first_frame = index == 0;
            let tick = frame.tick;
            let now_ms = tick as f64 * TICK_MS;
            let by_addr: BTreeMap<u64, &Entity> =
                frame.entities.iter().map(|e| (e.address, e)).collect();
            let stale: Vec<u64> = actors
                .iter()
                .filter(|(addr, actor)| !by_addr.get(addr).is_some_and(|e| actor.matches(e)))
                .map(|(addr, _)| *addr)
                .collect();
            for addr in stale {
                actors.remove(&addr);
            }
            let mut spawned = Vec::new();
            let mut hp_drops = Vec::new();
            for entity in &frame.entities {
                if let Some(actor) = actors.get_mut(&entity.address) {
                    if entity.hp >= 0 && entity.hp < actor.last.hp {
                        hp_drops.push((entity.address, actor.last.hp));
                    }
                    actor.observe(entity, tick);
                } else {
                    actors.insert(
                        entity.address,
                        Actor::new(next_uid, entity, tick, !first_frame),
                    );
                    next_uid += 1;
                    spawned.push(entity.address);
                }
            }
            let profiles: BTreeMap<u64, (f64, f64, f64, f64, bool)> = actors
                .iter()
                .filter_map(|(addr, actor)| self.attack_profile(&actor.row).map(|p| (*addr, p)))
                .collect();
            let radii: BTreeMap<u64, f64> = frame
                .entities
                .iter()
                .map(|e| (e.address, self.radius(&e.row)))
                .collect();
            for (addr, actor) in actors.iter_mut() {
                let Some(&(hit, load, range, own, ranged)) = profiles.get(addr) else {
                    continue;
                };
                let target = actor
                    .last
                    .native_focus_entity
                    .and_then(|t| by_addr.get(&t))
                    .filter(|t| t.side != actor.side && t.hp != 0);
                let in_range = target.is_some_and(|t| {
                    let dx = (t.x - actor.last.x) as f64;
                    let dy = (t.y - actor.last.y) as f64;
                    (dx * dx + dy * dy).sqrt()
                        <= range + own + radii.get(&t.address).copied().unwrap_or(500.) + 400.
                });
                let deploy = self
                    .logic
                    .row(&actor.row)
                    .and_then(|r| r.num("DeployTime"))
                    .unwrap_or(0.);
                let age_ms = now_ms - actor.spawn_tick as f64 * TICK_MS;
                let ready = !actor.deploys || age_ms >= deploy;
                let aiming = !ranged || actor.still_ticks(tick) >= actors::MOVE_HOLD_TICKS;
                let engaged = actor.last.hp != 0 && ready && in_range && aiming;
                actor.set_engaged(engaged, now_ms, hit, load);
            }
            for addr in &spawned {
                let Some(projectile) = by_addr.get(addr).copied() else {
                    continue;
                };
                if !self.is_projectile(&projectile.row) {
                    continue;
                }
                let shooter = actors
                    .iter()
                    .filter(|(a, actor)| {
                        *a != addr
                            && actor.side == projectile.side
                            && actor.last.max_hp > 0
                            && self.row_str(&actor.row, "Projectile").as_deref()
                                == Some(projectile.row.as_str())
                    })
                    .map(|(a, actor)| {
                        let dx = (actor.last.x - projectile.x) as f64;
                        let dy = (actor.last.y - projectile.y) as f64;
                        (dx * dx + dy * dy, *a)
                    })
                    .filter(|(d, _)| *d < 5000. * 5000.)
                    .min_by(|a, b| a.0.total_cmp(&b.0))
                    .map(|(_, a)| a);
                if let Some(shooter) = shooter {
                    let hit = profiles.get(&shooter).map_or(1000., |p| p.0);
                    note_scheduled(&mut scheduled, &mut actors, shooter, now_ms - TICK_MS * 0.5, hit);
                }
            }
            for (target_addr, before) in &hp_drops {
                let Some(target) = by_addr.get(target_addr).copied() else {
                    continue;
                };
                let loss = (before - target.hp.max(0)) as f64;
                let candidates: Vec<MeleeCandidate> = actors
                    .iter()
                    .filter(|(_, actor)| {
                        actor.side != target.side
                            && actor.last.native_focus_entity == Some(*target_addr)
                            && actor.engaged_since_ms.is_some()
                    })
                    .filter_map(|(addr, actor)| {
                        let (hit, _, _, _, ranged) = *profiles.get(addr)?;
                        if ranged
                            || actor
                                .impacts
                                .last()
                                .is_some_and(|last| now_ms - last < hit * 0.75)
                        {
                            return None;
                        }
                        let damage = self.logic.row(&actor.row).and_then(|r| {
                            let base = r.num("Hitpoints").filter(|h| *h > 0.)?;
                            Some(r.num("Damage")? * actor.last.max_hp as f64 / base)
                        });
                        let error = actor
                            .predicted_ms
                            .map_or(hit, |p| (p - now_ms).abs().min(hit));
                        Some(MeleeCandidate {
                            addr: *addr,
                            hit,
                            damage,
                            on_time: error <= hit * 0.35,
                        })
                    })
                    .collect();
                for candidate in attribute_loss(loss, &candidates) {
                    note_scheduled(
                        &mut scheduled,
                        &mut actors,
                        candidate.addr,
                        now_ms - TICK_MS * 0.5,
                        candidate.hit,
                    );
                }
            }
        }
        self.scheduled_impacts = scheduled;
        self.scheduled_match = Some(match_id);
    }

    fn spawn_death(&mut self, row: &str, x: f64, y: f64, z: f64, side: i32, now_ms: f64) {
        let effect = self
            .row_str(row, "DeathEffect")
            .or_else(|| (row == "PrincessTower").then(|| "TowerExplode".to_owned()));
        if let Some(effect) = effect {
            self.vfx.spawn(
                &effect,
                Anchor {
                    x,
                    y,
                    z: z * Z_TO_SC,
                },
                side != 0,
                1.,
                now_ms,
                0.,
                None,
            );
        }
    }

    fn layout(&self, frame: &BattleFrame) -> (Layout, u32) {
        let hud = ((self.width as f32 * 0.235).round() as u32).min(self.height / 3);
        let field_h = self.height.saturating_sub(hud * 2).max(1);
        let field_w = ((field_h as f32 * 540.0 / 768.0).round() as u32)
            .min(self.width.saturating_sub(24).max(1));
        (
            Layout {
                field_x: ((self.width - field_w) / 2) as f64,
                field_y: hud as f64,
                field_w: field_w as f64,
                field_h: field_h as f64,
                world_w: frame.arena_size[0].max(1) as f64,
                world_h: frame.arena_size[1].max(1) as f64,
                scale: field_w as f64 / 540.,
            },
            hud,
        )
    }

    pub fn controls_layout(&self, frame: &BattleFrame) -> ControlLayout {
        let (layout, panel) = self.layout(frame);
        let hand = HandLayout::new(
            self.width,
            self.height,
            panel,
            frame.human.as_ref().map_or(0, |h| h.side),
        );
        ControlLayout {
            field: [
                layout.field_x,
                layout.field_y,
                layout.field_w,
                layout.field_h,
            ],
            world: [layout.world_w, layout.world_h],
            cards: std::array::from_fn(|i| hand.card_rect(i as u32)),
        }
    }

    pub fn draw_player_controls(
        &mut self,
        out: &mut RgbaImage,
        frame: &BattleFrame,
        input: &PlayerInput,
    ) -> Result<()> {
        let Some(human) = &frame.human else {
            return Ok(());
        };
        let geometry = self.controls_layout(frame);
        let scale = self.width as f64 / 540.;
        for (slot, &[x, y, w, h]) in geometry.cards.iter().enumerate() {
            if input.selection.is_some_and(|s| s.slot == slot as u32) {
                for (rx, ry, rw, rh) in [
                    (x - 3., y - 3., w + 6., 3.),
                    (x - 3., y + h, w + 6., 3.),
                    (x - 3., y - 3., 3., h + 6.),
                    (x + w, y - 3., 3., h + 6.),
                ] {
                    fill_rect(
                        out,
                        rx as i32,
                        ry as i32,
                        rw as i32,
                        rh as i32,
                        Rgba([98, 232, 255, 255]),
                    );
                }
            }
        }
        if let (Some(selection), Some((x, y))) = (input.selection, input.cursor) {
            let checked = human.target.as_ref().filter(|t| {
                t.slot == selection.slot && t.card_id == selection.card_id && t.x == x && t.y == y
            });
            let color = match checked {
                Some(t) if !t.valid => Rgba([255, 91, 90, 255]),
                Some(t) if !t.affordable => Rgba([255, 194, 71, 255]),
                Some(_) => Rgba([108, 255, 128, 255]),
                None => Rgba([98, 232, 255, 255]),
            };
            if checked.is_some_and(|t| t.snaps) {
                // Red no-deploy zone, as in the real game while aiming there.
                for &[x0, y0, x1, y1] in &human.forbidden {
                    let (ax, ay) = geometry.to_screen(x0, y1);
                    let (bx, by) = geometry.to_screen(x1, y0);
                    for py in ay.max(0.) as u32..(by as u32).min(out.height()) {
                        for px in ax.max(0.) as u32..(bx as u32).min(out.width()) {
                            let p = out.get_pixel_mut(px, py);
                            p.0[0] = ((p.0[0] as u16 * 3 + 255 * 2) / 5) as u8;
                            p.0[1] = (p.0[1] as u16 * 3 / 5) as u8;
                            p.0[2] = (p.0[2] as u16 * 3 / 5) as u8;
                        }
                    }
                }
            }
            // Draw the preview where the card will actually land.
            let (lx, ly) = checked
                .and_then(|t| t.land_x.zip(t.land_y))
                .unwrap_or((x, y));
            let (sx, sy) = geometry.to_screen(lx, ly);
            let r = (12. * scale).round().max(8.) as i32;
            for dy in -r - 2..=r + 2 {
                for dx in -r - 2..=r + 2 {
                    let distance = dx * dx + dy * dy;
                    if distance >= (r - 1) * (r - 1) && distance <= (r + 1) * (r + 1) {
                        fill_rect(out, sx as i32 + dx, sy as i32 + dy, 1, 1, color);
                    }
                }
            }
            fill_rect(out, sx as i32 - r - 5, sy as i32 - 1, 2 * r + 10, 2, color);
            fill_rect(out, sx as i32 - 1, sy as i32 - r - 5, 2, 2 * r + 10, color);
            let label = match checked {
                Some(t) if !t.valid => Some("Cannot place here".to_owned()),
                Some(t) if !t.affordable => Some(format!("Need {} elixir", t.cost)),
                _ => None,
            };
            if let Some(label) = label {
                self.text(
                    out,
                    &label,
                    sx as u32,
                    (sy + 20. * scale).max(0.) as u32,
                    (12. * scale) as f32,
                    WHITE,
                    1,
                )?;
            }
        }
        if !human.feedback.is_empty() {
            self.text(
                out,
                &human.feedback,
                self.width / 2,
                (98. * scale) as u32,
                (12. * scale) as f32,
                WHITE,
                1,
            )?;
        }
        if human.paused && !frame.terminated {
            fill_rect(
                out,
                self.width as i32 / 2 - 100,
                self.height as i32 / 2 - 25,
                200,
                50,
                DARK,
            );
            self.text(
                out,
                "PAUSED",
                self.width / 2,
                self.height / 2 - 15,
                24.,
                WHITE,
                1,
            )?;
        }
        Ok(())
    }

    /// Render the scene at a fractional native tick between observed snapshots.
    pub fn draw(&mut self, display_tick: f64) -> Result<RgbaImage> {
        let mut out = RgbaImage::from_pixel(self.width, self.height, Rgba([19, 27, 31, 255]));
        scene::register_root(&out);
        for actor in self.actors.values_mut() {
            actor.shown = false;
        }
        let Some(latest) = self.history.back() else {
            return Ok(out);
        };
        let oldest = self.history.front().expect("history").tick as f64;
        let t = display_tick.clamp(oldest, latest.tick as f64);
        let index = self
            .history
            .iter()
            .rposition(|f| f.tick as f64 <= t)
            .unwrap_or(0);
        let a = self.history[index].clone();
        let b = self.history.get(index + 1).cloned();
        let alpha = b.as_ref().map_or(0., |b| {
            ((t - a.tick as f64) / (b.tick - a.tick).max(1) as f64).clamp(0., 1.)
        });
        let now_ms = t * TICK_MS;
        self.vfx.update(&self.logic, now_ms);
        let (layout, hud) = self.layout(&a);
        let (field_w, field_h) = (layout.field_w as u32, layout.field_h as u32);

        if self
            .field_cache
            .as_ref()
            .is_none_or(|f| f.dimensions() != (field_w, field_h))
        {
            let mut field_layer = RgbaImage::new(field_w, field_h);
            self.draw_arena(&mut field_layer, field_w, field_h);
            // Original location CSV props use their exact coordinates and export
            // identities under the documented asset-calibrated viewport.
            self.draw_decorations(&mut field_layer, field_w, field_h)?;
            self.field_cache = Some(field_layer);
        }
        scene::replace(
            &mut out,
            self.field_cache.as_ref().expect("field cache initialized"),
            layout.field_x as i64,
            layout.field_y as i64,
        );

        // Interpolated entity states for this instant.
        let b_index: BTreeMap<u64, &Entity> = b
            .iter()
            .flat_map(|b| b.entities.iter())
            .map(|e| (e.address, e))
            .collect();
        let span = b.as_ref().map_or(1., |b| (b.tick - a.tick).max(1) as f64);
        let mut states: Vec<(Entity, u64, [f64; 3])> = Vec::new();
        for e in &a.entities {
            let Some(actor) = self.actor_for(e) else {
                continue;
            };
            let mut state = e.clone();
            let mut velocity = [0.; 3];
            match b_index.get(&e.address).filter(|n| actor.matches(n)) {
                Some(next) => {
                    let lerp = |p: i32, q: i32| (p as f64 + (q - p) as f64 * alpha).round() as i32;
                    state.x = lerp(e.x, next.x);
                    state.y = lerp(e.y, next.y);
                    state.z = lerp(e.z, next.z);
                    velocity = [
                        (next.x - e.x) as f64 / span,
                        (next.y - e.y) as f64 / span,
                        (next.z - e.z) as f64 / span,
                    ];
                }
                None if self.is_projectile(&e.row) => {
                    velocity = actor.velocity;
                    state.x += (velocity[0] * alpha) as i32;
                    state.y += (velocity[1] * alpha) as i32;
                    state.z = (state.z + (velocity[2] * alpha) as i32).max(0);
                }
                None => {}
            }
            states.push((state, actor.uid, velocity));
        }
        // Newly launched projectiles leave their shooter during the interval
        // before the snapshot that first contains them.
        if let Some(b) = &b {
            let a_addrs: BTreeSet<u64> = a.entities.iter().map(|e| e.address).collect();
            for e in &b.entities {
                if a_addrs.contains(&e.address) || !self.is_projectile(&e.row) || alpha < 0.2 {
                    continue;
                }
                let Some(actor) = self.actor_for(e) else {
                    continue;
                };
                let Some(origin) = actor.origin else { continue };
                let mut state = e.clone();
                state.x = (origin[0] + (e.x as f64 - origin[0]) * alpha) as i32;
                state.y = (origin[1] + (e.y as f64 - origin[1]) * alpha) as i32;
                let velocity = [
                    (e.x as f64 - origin[0]) / span,
                    (e.y as f64 - origin[1]) / span,
                    0.,
                ];
                states.push((state, actor.uid, velocity));
            }
        }
        let due: Vec<u64> = self
            .pending_detach
            .iter()
            .filter(|(_, at)| *at <= now_ms)
            .map(|(uid, _)| *uid)
            .collect();
        self.pending_detach.retain(|(_, at)| *at > now_ms);
        for uid in due {
            self.vfx.attach(uid, None);
        }
        for (state, uid, _) in &states {
            self.vfx.attach(
                *uid,
                Some(Anchor {
                    x: state.x as f64,
                    y: state.y as f64,
                    z: state.z as f64 * Z_TO_SC,
                }),
            );
        }

        let mut plans = Vec::with_capacity(states.len());
        let mut beam_anchors = BTreeMap::new();
        for (state, uid, velocity) in &states {
            if let Some(mut plan) = self.plan_entity(&a, state, *velocity, now_ms, &layout)? {
                plan.uid = *uid;
                if let Some(bar) = &plan.hp {
                    let bottom = plan
                        .parts
                        .iter()
                        .map(|p| p.y + p.sprite.dy as f64 + p.sprite.image.height() as f64)
                        .fold(bar.top, f64::max);
                    let height = (bottom - bar.top).max(1.);
                    // Artwork bounds provide presentation attachments; the
                    // native client's named effect attachment is unrecovered.
                    beam_anchors.insert(
                        state.address,
                        (
                            (bar.x, bar.top + height * 0.15),
                            (bar.x, bar.top + height * 0.55),
                        ),
                    );
                }
                plans.push(plan);
            }
        }
        let sprites = self.vfx.sprites(&self.scenes);

        self.draw_vfx_layer(&mut out, &sprites, Layer::Ground, &layout);
        for plan in &plans {
            if let Some(spec) = &plan.shadow {
                let _scope = scene::ScopeGuard::enter_anchored(
                    format!("entity-{}-shadow", plan.uid),
                    plan.ground_anchor,
                );
                let part = spec.source.as_ref().unwrap_or(&plan.parts[spec.part]);
                let t = Affine {
                    m: [
                        spec.scale[0],
                        -spec.skew * spec.scale[1].abs(),
                        0.,
                        spec.scale[1],
                    ],
                    tx: part.x + spec.offset[0] * layout.scale,
                    ty: part.y + spec.offset[1] * layout.scale,
                };
                self.shadows.add_silhouette(
                    &part.sprite.image,
                    part.sprite.dx as f64,
                    part.sprite.dy as f64,
                    &t,
                    SHADOW_ALPHA,
                );
            }
        }
        for sprite in sprites.iter().filter(|s| s.layer == Layer::Shadow) {
            let (sx, sy) = layout.ground(sprite.anchor.x, sprite.anchor.y);
            vfx::draw_sprite(
                &mut out,
                &mut self.shadows,
                &mut self.vfx.cache,
                &self.scenes,
                sprite,
                sx,
                sy,
                layout.scale,
            );
        }
        scene::scope("shadows", || self.shadows.apply(&mut out));
        for plan in &plans {
            for (index, part) in plan.ground_parts.iter().enumerate() {
                scene::scope_anchored(
                    format!("entity-{}-ground-{index}", plan.uid),
                    plan.ground_anchor,
                    || draw_part(&mut out, part),
                );
            }
        }
        self.draw_vfx_layer(&mut out, &sprites, Layer::Base, &layout);

        // Ground objects and object-layer effects, back to front.
        enum Item<'p> {
            Plan(&'p Plan),
            Sprite(&'p VfxSprite),
        }
        let mut items: Vec<(f64, Item)> = plans
            .iter()
            .filter(|p| !p.air)
            .map(|p| (p.sort_y, Item::Plan(p)))
            .collect();
        for sprite in sprites.iter().filter(|s| s.layer == Layer::Object) {
            let (_, sy) = layout.ground(sprite.anchor.x, sprite.anchor.y);
            items.push((
                sy + sprite.sort_y * layout.scale + 0.5,
                Item::Sprite(sprite),
            ));
        }
        let mut air: Vec<&Plan> = plans.iter().filter(|p| p.air).collect();
        items.sort_by(|x, y| x.0.total_cmp(&y.0));
        air.sort_by(|x, y| x.sort_y.total_cmp(&y.sort_y));
        for (_, item) in items {
            match item {
                Item::Plan(plan) => draw_plan(&mut out, plan),
                Item::Sprite(sprite) => {
                    let (sx, sy) = layout.ground(sprite.anchor.x, sprite.anchor.y);
                    vfx::draw_sprite(
                        &mut out,
                        &mut self.shadows,
                        &mut self.vfx.cache,
                        &self.scenes,
                        sprite,
                        sx,
                        sy,
                        layout.scale,
                    );
                }
            }
        }
        for plan in air {
            draw_plan(&mut out, plan);
        }
        self.draw_vfx_layer(&mut out, &sprites, Layer::Above, &layout);
        self.draw_targeted_beams(&mut out, &states, &beam_anchors, now_ms, &layout)?;
        for plan in &plans {
            if let Some(bar) = &plan.hp {
                let _scope = scene::ScopeGuard::enter(format!("entity-{}-hp", plan.uid));
                self.draw_hp_bar(&mut out, bar, layout.scale)?;
            }
        }

        let live_hud = self.history.back().filter(|f| f.human.is_some()).cloned();
        let hud_frame = if let Some(live) = &live_hud {
            live
        } else if alpha >= 0.5 {
            b.as_ref().unwrap_or(&a)
        } else {
            &a
        };
        self.hand_hud.render(
            &mut out,
            &self.root,
            &self.scenes,
            &self.assets,
            hud_frame,
            hud,
        )?;
        let secs = hud_frame.elapsed_ms / 1000;
        self.text(
            &mut out,
            &format!("{}:{:02}", secs / 60, secs % 60),
            (layout.field_x + layout.field_w) as u32 - 34,
            layout.field_y as u32 + 6,
            15.,
            WHITE,
            1,
        )?;
        if hud_frame.terminated {
            let msg = if let Some(human) = &hud_frame.human {
                match hud_frame.winner_side {
                    Some(s) if s == human.side as i32 => "YOU WIN",
                    Some(0 | 1) => "AI WINS",
                    _ => "DRAW",
                }
            } else {
                match hud_frame.winner_side {
                    Some(0) => "BLUE WINS",
                    Some(1) => "RED WINS",
                    _ => "DRAW",
                }
            };
            self.text(
                &mut out,
                msg,
                self.width / 2,
                (layout.field_y + layout.field_h / 2.) as u32,
                26.,
                WHITE,
                1,
            )?;
        }
        Ok(out)
    }

    fn actor_for(&self, e: &Entity) -> Option<&Actor> {
        self.actors
            .get(&e.address)
            .filter(|a| a.matches(e))
            .or_else(|| {
                self.departed
                    .get(&e.address)
                    .map(|(_, a)| a)
                    .filter(|a| a.matches(e))
            })
    }

    fn draw_vfx_layer(
        &mut self,
        out: &mut RgbaImage,
        sprites: &[VfxSprite],
        layer: Layer,
        layout: &Layout,
    ) {
        for sprite in sprites.iter().filter(|s| s.layer == layer) {
            let (sx, sy) = layout.ground(sprite.anchor.x, sprite.anchor.y);
            vfx::draw_sprite(
                out,
                &mut self.shadows,
                &mut self.vfx.cache,
                &self.scenes,
                sprite,
                sx,
                sy,
                layout.scale,
            );
        }
    }

    fn draw_arena(&mut self, out: &mut RgbaImage, w: u32, h: u32) {
        let Some(arena) = self.arena_field.as_ref() else {
            return;
        };
        let fitted = imageops::resize(arena, w, h, imageops::FilterType::Lanczos3);
        scene::overlay(out, &fitted, 0, 0);
    }

    fn draw_decorations(&mut self, out: &mut RgbaImage, w: u32, h: u32) -> Result<()> {
        let objects = self.assets["scene"]["objects"]
            .as_array()
            .cloned()
            .unwrap_or_default();
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
                let px = px / 18000.0 * w as f64;
                let py = ((1.0 - py / 32000.) * 768. + 60.) / 828. * h as f64;
                if px < 0. || px > w as f64 || py < 0. || py > h as f64 {
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
                    scene::overlay(
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

    /// Original stretchable beam art, anchored to an observed, in-range focus.
    /// Warm-up and damage stages are presentation estimates from engagement
    /// time and source logic; no attack flag or damage is written to the engine.
    fn draw_targeted_beams(
        &mut self,
        out: &mut RgbaImage,
        states: &[(Entity, u64, [f64; 3])],
        anchors: &BTreeMap<u64, ((f64, f64), (f64, f64))>,
        now_ms: f64,
        layout: &Layout,
    ) -> Result<()> {
        let mut beams = Vec::new();
        for (source, source_uid, _) in states {
            let Some(row) = self.logic.row(&source.row) else {
                continue;
            };
            if row.str("TargettedDamageEffect1").is_none() || source.hp <= 0 {
                continue;
            }
            let Some(since) = self.actor_for(source).and_then(|a| a.engaged_since_ms) else {
                continue;
            };
            let age = now_ms - since - row.num("LoadTime").unwrap_or(0.);
            if age < 0. {
                continue;
            }
            let Some((target, _, _)) = states.iter().find(|(e, _, _)| {
                Some(e.address) == source.native_focus_entity && e.hp > 0 && e.side != source.side
            }) else {
                continue;
            };
            let dx = (target.x - source.x) as f64;
            let dy = (target.y - source.y) as f64;
            let distance = dx.hypot(dy);
            if distance <= 0.
                || distance
                    > row.num("Range").unwrap_or(0.)
                        + self.radius(&source.row)
                        + self.radius(&target.row)
            {
                continue;
            }
            let first = row.num("VariableDamageTime1").unwrap_or(f64::INFINITY);
            let second = row.num("VariableDamageTime2").unwrap_or(f64::INFINITY);
            let key = if age >= first + second {
                "TargettedDamageEffect3"
            } else if age >= first {
                "TargettedDamageEffect2"
            } else {
                "TargettedDamageEffect1"
            };
            let Some(effect) = row.str(key).and_then(|name| self.logic.effects.get(name)) else {
                continue;
            };
            let (Some((start, _)), Some((_, end))) =
                (anchors.get(&source.address), anchors.get(&target.address))
            else {
                continue;
            };
            let (start, end) = (*start, *end);
            for component in &effect.renderable_components {
                if component.r#type != "SWF" {
                    continue;
                }
                let (Some(file), Some(export), Some(width)) = (
                    &component.file_name,
                    &component.export_name,
                    component.flash_width,
                ) else {
                    continue;
                };
                if width <= 0. {
                    continue;
                }
                let Some(doc) = Path::new(file).file_stem().and_then(|s| s.to_str()) else {
                    continue;
                };
                let Some(scene) = self.scenes.get(doc) else {
                    continue;
                };
                let count = scene.frame_count(export).map_err(|e| e.to_string())?;
                let fps = scene.framerate(export).map_err(|e| e.to_string())?;
                let frame = (age * fps as f64 / 1000.) as usize % count.max(1);
                beams.push((
                    *source_uid,
                    doc.to_owned(),
                    export.clone(),
                    frame,
                    age,
                    width,
                    start,
                    end,
                ));
            }
        }
        for (uid, doc, export, frame, age, width, start, end) in beams {
            let _scope = scene::ScopeGuard::enter(format!("entity-{uid}-beam-{doc}-{export}"));
            let playback = Playback {
                frame: Some(frame),
                time_seconds: Some(age / 1000.),
                ..Playback::default()
            };
            let Some(sprite) = self.fit(&doc, &export, playback, frame, 1., false)? else {
                continue;
            };
            let dx = end.0 - start.0;
            let dy = end.1 - start.1;
            let distance = dx.hypot(dy);
            if distance < 1. {
                continue;
            }
            let (c, s) = (dx / distance, dy / distance);
            blit::blit_affine(
                out,
                &sprite.image,
                sprite.dx as f64,
                sprite.dy as f64,
                &Affine {
                    m: [dx / width, -s * layout.scale, dy / width, c * layout.scale],
                    tx: start.0,
                    ty: start.1,
                },
                1.,
                Blend::Normal,
            );
        }
        Ok(())
    }

    /// Render (cached) and fit an original export at a draw scale.
    #[allow(clippy::too_many_arguments)]
    fn fit(
        &mut self,
        doc: &str,
        export: &str,
        playback: Playback,
        frame_index: usize,
        scale: f64,
        flip: bool,
    ) -> Result<Option<Rc<Fitted>>> {
        use std::hash::{Hash, Hasher};
        // Same 8-byte usize prefix as x86_64, so a fitted-pose cache key does
        // not change between the native spectator and wasm32.
        let mut hasher = scene::Width64Hasher::new();
        playback.object_frames.hash(&mut hasher);
        playback.hidden_objects.hash(&mut hasher);
        playback.label.hash(&mut hasher);
        playback.text_values.hash(&mut hasher);
        playback.render_text.hash(&mut hasher);
        playback
            .root_transform
            .map(|m| {
                [
                    m.a.to_bits(),
                    m.b.to_bits(),
                    m.c.to_bits(),
                    m.d.to_bits(),
                    m.tx.to_bits(),
                    m.ty.to_bits(),
                ]
            })
            .hash(&mut hasher);
        playback
            .root_color
            .map(|c| {
                [
                    c.r_mul, c.g_mul, c.b_mul, c.alpha, c.r_add, c.g_add, c.b_add,
                ]
            })
            .hash(&mut hasher);
        let variant = hasher.finish();
        let fit = (
            doc.to_owned(),
            export.to_owned(),
            frame_index,
            variant,
            scale.to_bits(),
            flip,
        );
        if let Some(fitted) = self.sprite_fit_cache.get(&fit) {
            return Ok(Some(fitted.clone()));
        }
        // Thousandths of a draw-scale are enough to share a raster. Exact
        // float bits were splitting one pose into many near-duplicate files.
        let scale_q = (scale * 1000.).round() as u64;
        let disk_key = pose_cache::key(
            &[doc, export],
            &[frame_index as u64, variant, scale_q, u64::from(flip)],
        );
        let cached = self
            .pose_cache
            .as_ref()
            .and_then(|cache| cache.load(&disk_key));
        if let Some((image, dx, dy)) = cached {
            let fitted = Rc::new(Fitted {
                image,
                dx: dx.round() as i64,
                dy: dy.round() as i64,
            });
            pose_cache::trim_map(&mut self.sprite_fit_cache, 4000);
            self.sprite_fit_cache.insert(fit, fitted.clone());
            return Ok(Some(fitted));
        }
        let key = (doc.to_owned(), export.to_owned(), frame_index, variant);
        if !self.sprite_cache.contains_key(&key) {
            let scene = self
                .scenes
                .get(doc)
                .ok_or_else(|| format!("original scene missing {doc}"))?;
            let rendered = if let Some(id) = export.strip_prefix('#') {
                scene.render_id(id.parse().map_err(|_| "object id")?, &playback)
            } else {
                scene.render(export, &playback)
            }
            .map_err(|e| e.to_string())?;
            pose_cache::trim_map(&mut self.sprite_cache, 2500);
            self.sprite_cache.insert(key.clone(), rendered.map(Rc::new));
        }
        let Some(Some(sprite)) = self.sprite_cache.get(&key).cloned() else {
            return Ok(None);
        };
        let sw = (sprite.image.width() as f64 * scale).round().max(1.) as u32;
        let sh = (sprite.image.height() as f64 * scale).round().max(1.) as u32;
        let mut image = imageops::resize(&sprite.image, sw, sh, imageops::FilterType::Triangle);
        let dx = if flip {
            -(sprite.x + sprite.image.width() as f64) * scale
        } else {
            sprite.x * scale
        };
        if flip {
            image = imageops::flip_horizontal(&image);
        }
        let fitted = Rc::new(Fitted {
            image,
            dx: dx.round() as i64,
            dy: (sprite.y * scale).round() as i64,
        });
        if let Some(cache) = &self.pose_cache {
            cache.store(&disk_key, &fitted.image, fitted.dx as f64, fitted.dy as f64);
        }
        pose_cache::trim_map(&mut self.sprite_fit_cache, 4000);
        self.sprite_fit_cache.insert(fit, fitted.clone());
        Ok(Some(fitted))
    }

    #[allow(clippy::too_many_arguments)]
    fn part(
        &mut self,
        doc: &str,
        export: &str,
        playback: Playback,
        frame_index: usize,
        x: f64,
        y: f64,
        scale: f64,
        flip: bool,
    ) -> Result<Option<Part>> {
        Ok(self
            .fit(doc, export, playback, frame_index, scale, flip)?
            .map(|sprite| Part { sprite, x, y }))
    }

    /// Legacy skew shadow from the row's original fields: the sprite
    /// silhouette scaled by `ShadowScaleX/Y` (negative flips), sheared by
    /// `ShadowSkew` and offset by `ShadowX/Y` SC pixels.
    fn shadow_spec(&self, row: &str, part: usize) -> Option<ShadowSpec> {
        let logic = self.logic.row(row)?;
        let sx = logic.num("ShadowScaleX")?;
        let sy = logic.num("ShadowScaleY").unwrap_or(sx);
        Some(ShadowSpec {
            part,
            source: None,
            scale: [sx / 100., sy / 100.],
            skew: logic.num("ShadowSkew").unwrap_or(0.) / 100.,
            offset: [
                logic.num("ShadowX").unwrap_or(0.),
                logic.num("ShadowY").unwrap_or(0.),
            ],
        })
    }

    /// Choose original artwork and placement for one interpolated entity.
    fn plan_entity(
        &mut self,
        frame: &BattleFrame,
        entity: &Entity,
        velocity: [f64; 3],
        now_ms: f64,
        layout: &Layout,
    ) -> Result<Option<Plan>> {
        let (gx, gy) = layout.ground(entity.x as f64, entity.y as f64);
        let (x, py) = layout.project(entity.x as f64, entity.y as f64, entity.z as f64);
        let Some(visual) = self.bindings.get(&entity.row).cloned() else {
            // Area-effect rows are drawn through their original effects.
            if self
                .logic
                .row(&entity.row)
                .is_some_and(|r| r.is_area_effect())
            {
                return Ok(None);
            }
            self.unresolved_rows.insert(entity.row.clone());
            if entity.max_hp > 0 {
                let mut marker = RgbaImage::new(11, 11);
                fill_circle(
                    &mut marker,
                    5,
                    5,
                    5,
                    if entity.side == 0 { BLUE } else { RED },
                );
                let part = Part {
                    sprite: Rc::new(Fitted {
                        image: marker,
                        dx: -5,
                        dy: -5,
                    }),
                    x,
                    y: py,
                };
                probe_shown(
                    entity,
                    0,
                    "marker",
                    "none",
                    None,
                    None,
                    (x, py),
                    std::slice::from_ref(&part),
                );
                return Ok(Some(Plan {
                    uid: 0,
                    parts: vec![part],
                    shadow: None,
                    ground_parts: Vec::new(),
                    sort_y: gy,
                    air: false,
                    hp: None,
                    screen_anchor: [x, py],
                    ground_anchor: [gx, gy],
                }));
            }
            probe_shown(entity, 0, "missing", "none", None, None, (x, py), &[]);
            return Ok(None);
        };
        let Some(actor) = self.actor_for(entity) else {
            return Ok(None);
        };
        let projectile = self.is_projectile(&entity.row);
        let red = entity.side != 0;
        let field_scale = layout.scale;
        let scale = field_scale * visual.scale_percent / 100.;
        let profile = self.attack_profile(&entity.row);
        let attack_target = entity
            .native_focus_entity
            .and_then(|p| frame.entities.iter().find(|e| e.address == p));
        let walk_since = actor.walk_at(now_ms);
        let moving = walk_since.is_some();
        // Face the native focus target unless walking; walking units face
        // their displayed (interpolated) motion.
        let vector = match attack_target {
            Some(target) if !moving => ((target.x - entity.x) as f64, (target.y - entity.y) as f64),
            _ if velocity[0].abs() + velocity[1].abs() > 0.5 => (velocity[0], velocity[1]),
            _ => (0., 0.),
        };
        let heading = actors::stable_heading(vector.0, vector.1, entity.side, actor.heading);
        let (uid, spawn_tick, deploys) = (actor.uid, actor.spawn_tick, actor.deploys);
        // Hits known from the whole replay. The wind-up is drawn before the
        // damage tick; hit effects still spawn when that tick is observed.
        let known = if self.scheduled_match == Some(frame.match_id) {
            let key = entity.native_entity_id.unwrap_or(entity.address);
            self.scheduled_impacts
                .get(&key)
                .cloned()
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        let mut impacts = actor.impacts.clone();
        for &at in &known {
            if !impacts.iter().any(|hit| (hit - at).abs() < 100.) {
                impacts.push(at);
            }
        }
        let hidden_now = if projectile {
            1.
        } else {
            self.stealth_alpha(&entity.row, spawn_tick, now_ms, deploys, &impacts)
        };
        let flash = actor
            .flash_at(now_ms)
            .map(|(export, age)| (export.to_owned(), age));
        let age_ms = now_ms - spawn_tick as f64 * TICK_MS;
        // Attack phase in the authored clip, resolved after export selection.
        let actor_attack = |action_ms: f64, clip_ms: f64, windup_ms: f64| {
            actor.attack_clip_known(now_ms, action_ms, clip_ms, windup_ms, &known)
        };
        let mut attack_phase = None;
        if let (Some((_hit, load, ..)), false) = (profile, projectile) {
            let attack_scene_prefix = if entity.row == "PrincessTower" {
                Some(("chr_princess", "princess_tower", Some("princess_tower_red")))
            } else {
                None
            };
            let (doc, prefix, red_prefix) = attack_scene_prefix.unwrap_or((
                visual.document.as_str(),
                visual.prefix.as_str(),
                visual.red_prefix.as_deref(),
            ));
            if let Some(scene) = self.scenes.get(doc) {
                if let Ok(Some(sel)) = animation::select_for_heading(
                    scene,
                    prefix,
                    red_prefix,
                    red,
                    AnimationState::Attacking,
                    heading,
                    0.0,
                ) {
                    if sel.matched_state {
                        if let Some(movie) = scene
                            .document
                            .exports
                            .get(&sel.export)
                            .and_then(|id| scene.document.movies.get(id))
                        {
                            let fps = movie.framerate.max(1) as f64;
                            let count = movie.frame_element_counts.len().max(1) as f64;
                            let action = movie
                                .frame_labels
                                .iter()
                                .position(|l| l == "action_frame")
                                .map_or(count * 0.5, |p| p as f64);
                            attack_phase = actor_attack(
                                action * 1000. / fps,
                                count * 1000. / fps,
                                load.max(TICK_MS),
                            );
                        }
                    }
                }
            }
        }
        // A faded invisible unit keeps walking. A swing that never lands must
        // not pop it back to a solid attack pose.
        if hidden_now < 0.75 {
            attack_phase = None;
        }
        let state = if attack_phase.is_some() {
            AnimationState::Attacking
        } else if moving {
            AnimationState::Moving
        } else {
            AnimationState::Idle
        };
        let phase_ms = match state {
            AnimationState::Attacking => attack_phase.unwrap_or(0.),
            AnimationState::Moving => now_ms - walk_since.unwrap_or(now_ms),
            _ => now_ms - spawn_tick as f64 * TICK_MS,
        }
        .max(0.);
        // Buildings with `HasRotationOnTimeline` keep their turret aim in a
        // child timeline of views; the native target gives the aim.
        let rotating = !projectile
            && entity.row != "KingTower"
            && self
                .logic
                .row(&entity.row)
                .is_some_and(|r| r.flag("HasRotationOnTimeline"));
        let mut turret_yaw = None;
        if let Some(actor) = self.actors.get_mut(&entity.address) {
            if actor.uid == uid {
                actor.heading = Some(heading);
                actor.shown = true;
                actor.shown_state = match state {
                    AnimationState::Attacking => "attack",
                    AnimationState::Moving => "walk",
                    AnimationState::Deploying => "deploy",
                    AnimationState::Idle => "idle",
                };
                actor.shown_heading = heading.direction_index;
                actor.shown_projectile = projectile;
                if rotating {
                    let goal = attack_target.map(|t| {
                        ((t.x - entity.x) as f64)
                            .atan2((t.y - entity.y) as f64)
                            .to_degrees()
                    });
                    turret_yaw = Some(actor.aim_turret(goal, now_ms, TURRET_DEG_PER_S));
                }
            }
        }

        let mut parts = Vec::new();
        let mut ground_parts = Vec::new();
        let mut shadow = None;
        let mut conceal = false;
        let mut top = py - 40. * scale;
        let air = projectile || entity.z > 400;
        let tower = matches!(entity.row.as_str(), "KingTower" | "PrincessTower");
        let mut pick: Option<(String, usize, bool, f32, bool)> = None;
        if entity.hp == 0 && entity.max_hp > 0 && tower {
            let export = if entity.row == "KingTower" {
                "kingtower_destroyed"
            } else {
                "princesstower_destroyed"
            };
            if let Some(p) = self.part(
                "building_tower",
                export,
                Playback::default(),
                0,
                x,
                py,
                scale,
                false,
            )? {
                parts.push(p);
            }
            let pick = Some((export.to_owned(), 0, false, 1., false));
            probe_shown(
                entity,
                uid,
                "tower-destroyed",
                "none",
                pick,
                None,
                (x, py),
                &parts,
            );
            return Ok(Some(Plan {
                uid: 0,
                parts,
                shadow: None,
                ground_parts,
                sort_y: gy,
                air: false,
                hp: None,
                screen_anchor: [x, py],
                ground_anchor: [gx, gy],
            }));
        }
        if entity.row == "KingTower" {
            let activation = self
                .king_activations
                .get(&entity.address)
                .map(|tick| now_ms - *tick as f64 * TICK_MS)
                .filter(|age| *age >= 0.)
                .map(|age| age as u64);
            let layers = animation::king_tower_layers_with_activation(
                self.scenes.get("building_tower").ok_or("tower scene")?,
                self.scenes.get("chr_king").ok_or("king scene")?,
                red,
                heading,
                age_ms.max(0.) as u64,
                activation,
            )?;
            let root_frame = layers.root_playback.frame.unwrap_or(0);
            let king_frame = layers.king_playback.frame.unwrap_or(0);
            let turret_frame = layers.turret_playback.frame.unwrap_or(0);
            pick = Some((layers.king_export.to_owned(), king_frame, false, 1., false));
            let root = self.part(
                "building_tower",
                layers.root_export,
                layers.root_playback,
                root_frame,
                x,
                py,
                scale,
                false,
            )?;
            let turret = self.part(
                "building_tower",
                &format!("#{}", layers.turret_id),
                layers.turret_playback,
                turret_frame,
                x + layers.turret_offset[0] * scale,
                py + layers.turret_offset[1] * scale,
                scale,
                layers.turret_flip_horizontal,
            )?;
            let king = self.part(
                "chr_king",
                layers.king_export,
                layers.king_playback,
                king_frame,
                x,
                py,
                scale,
                false,
            )?;
            if let Some(root) = root {
                top = py + root.sprite.dy as f64;
                parts.push(root);
                shadow = self.shadow_spec("KingTower", 0);
            }
            let (first, second) = match layers.order {
                KingLayerOrder::TurretThenKing => (turret, king),
                KingLayerOrder::KingThenTurret => (king, turret),
            };
            parts.extend(first);
            parts.extend(second);
        } else if entity.row == "PrincessTower" {
            let (base, top_export) = if red {
                ("StarTower_base_red", "StarTower_top_red")
            } else {
                ("StarTower_base_blue", "StarTower_top_blue")
            };
            if let Some(p) = self.part(
                "building_tower",
                base,
                Playback::default(),
                0,
                x,
                py,
                scale,
                false,
            )? {
                top = py + p.sprite.dy as f64;
                parts.push(p);
                shadow = self.shadow_spec("PrincessTower", 0);
            }
            if let Some(p) = self.part(
                "building_tower",
                top_export,
                Playback::default(),
                0,
                x,
                py,
                scale,
                false,
            )? {
                parts.push(p);
            }
            let scene = self
                .scenes
                .get("chr_princess")
                .ok_or("attached TowerPrincess scene missing")?;
            if let Some(mut selected) = animation::select_for_heading(
                scene,
                "princess_tower",
                Some("princess_tower_red"),
                red,
                state,
                heading,
                phase_ms,
            )? {
                set_attack_frame(scene, &mut selected, state, phase_ms);
                pick = Some((
                    selected.export.clone(),
                    selected.frame_index,
                    selected.flip_horizontal,
                    1.,
                    false,
                ));
                let top_y = py - 2200. * Z_TO_SC * field_scale;
                if let Some(p) = self.part(
                    "chr_princess",
                    &selected.export,
                    selected.playback,
                    selected.frame_index,
                    x,
                    top_y,
                    field_scale,
                    selected.flip_horizontal,
                )? {
                    top = top.min(top_y + p.sprite.dy as f64);
                    parts.push(p);
                }
            }
        } else {
            let scene = self
                .scenes
                .get(&visual.document)
                .ok_or("entity scene missing")?;
            let selection = if projectile {
                // Original projectile art, with separate treatment for view
                // banks and timed clips. The original client view accessor is
                // unrecovered; in-plane flight orientation follows the
                // projected screen velocity, including the native arc.
                let export = if red {
                    visual.red_prefix.as_deref().unwrap_or(&visual.prefix)
                } else {
                    &visual.prefix
                };
                let fields = self
                    .logic
                    .row(&entity.row)
                    .map(|r| r.fields.clone())
                    .or_else(|| {
                        self.row_meta(&entity.row)
                            .and_then(|v| v["source_fields"].as_object().cloned())
                    })
                    .unwrap_or_default();
                let mode = crate::projectile::mode_for_source(&fields, export);
                let sx = velocity[0] / layout.world_w * layout.field_w;
                let sy = -velocity[1] / layout.world_h * 768. / 828. * layout.field_h
                    - velocity[2] * Z_TO_SC * field_scale;
                let flight = if sx.abs() + sy.abs() < 0.01 {
                    (0, if red { -1 } else { 1 })
                } else {
                    ((sx * 1000.) as i64, (-sy * 1000.) as i64)
                };
                let mut presentation = crate::projectile::select_projectile(
                    scene,
                    export,
                    mode,
                    flight.0,
                    flight.1,
                    age_ms.max(0.) as u64,
                    Some(velocity[2] as i64),
                )?;
                let angle = presentation.rotation_radians;
                if angle.abs() > 1e-8 {
                    // Quantize rotation so the fitted-sprite cache stays small.
                    let angle = (angle / 0.06).round() * 0.06;
                    presentation.selection.playback.root_transform = Some(Matrix {
                        a: angle.cos(),
                        b: -angle.sin(),
                        c: angle.sin(),
                        d: angle.cos(),
                        tx: 0.,
                        ty: 0.,
                    });
                }
                // Original projectile shadow export on the ground.
                if let Some(shadow_export) = self.row_str(&entity.row, "ShadowExportName") {
                    let shadow_doc = self
                        .row_str(&entity.row, "FileName")
                        .and_then(|f| {
                            Path::new(&f)
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .map(str::to_owned)
                        })
                        .unwrap_or_else(|| visual.document.clone());
                    if self
                        .scenes
                        .get(&shadow_doc)
                        .is_some_and(|s| s.document.exports.contains_key(&shadow_export))
                    {
                        let pb = Playback {
                            root_transform: presentation.selection.playback.root_transform,
                            ..Playback::default()
                        };
                        if let Some(p) =
                            self.part(&shadow_doc, &shadow_export, pb, 0, gx, gy, scale, false)?
                        {
                            ground_parts.push(p);
                        }
                    }
                }
                Some(presentation.selection)
            } else {
                animation::select_for_heading(
                    scene,
                    &visual.prefix,
                    visual.red_prefix.as_deref(),
                    red,
                    state,
                    heading,
                    phase_ms,
                )?
            };
            if let Some(mut selected) = selection {
                let scene = self.scenes.get(&visual.document).expect("scene");
                set_attack_frame(scene, &mut selected, state, phase_ms);
                if let Some(yaw) = turret_yaw {
                    aim_turret_view(scene, &mut selected, yaw);
                }
                let mut deploying = false;
                if !projectile {
                    deploying = deploys
                        && self.apply_filter(
                            self.row_str(&entity.row, "DeployBaseAnimExportName")
                                .as_deref(),
                            age_ms,
                            &mut selected.playback,
                        );
                    if !deploying {
                        if let Some((export, age)) = &flash {
                            self.apply_filter(Some(export), *age, &mut selected.playback);
                        }
                    }
                    let hidden = hidden_now;
                    if hidden < 0.99 {
                        let mut color = selected.playback.root_color.unwrap_or(ColorTransform {
                            r_mul: 255,
                            g_mul: 255,
                            b_mul: 255,
                            alpha: 255,
                            r_add: 0,
                            g_add: 0,
                            b_add: 0,
                        });
                        color.alpha = (color.alpha as f64 * hidden).round() as i32;
                        selected.playback.root_color = Some(color);
                        if hidden < 0.75 {
                            conceal = true;
                        }
                    }
                }
                pick = Some((
                    selected.export.clone(),
                    selected.frame_index,
                    selected.flip_horizontal,
                    selected
                        .playback
                        .root_color
                        .map_or(1., |c| c.alpha as f32 / 255.),
                    deploying,
                ));
                if let Some(p) = self.part(
                    &visual.document,
                    &selected.export,
                    selected.playback,
                    selected.frame_index,
                    x,
                    py,
                    scale,
                    selected.flip_horizontal,
                )? {
                    top = py + p.sprite.dy as f64;
                    parts.push(p);
                    if !projectile && !conceal {
                        shadow = self.shadow_spec(&entity.row, 0);
                    }
                }
                if let (true, Some(spec)) = (deploying, shadow.as_mut()) {
                    spec.source = self.part(
                        &visual.document,
                        &selected.export,
                        Playback {
                            frame: Some(selected.frame_index),
                            ..Playback::default()
                        },
                        selected.frame_index,
                        x,
                        py,
                        scale,
                        selected.flip_horizontal,
                    )?;
                }
            } else {
                self.unresolved_rows.insert(entity.row.clone());
            }
        }
        let hp = (entity.max_hp > 0).then(|| HpBar {
            kind: if tower {
                BarKind::Tower
            } else {
                match self
                    .row_str(&entity.row, "HealthBar")
                    .as_deref()
                    .unwrap_or("Medium")
                {
                    "Small" => BarKind::Small,
                    "High" => BarKind::High,
                    _ => BarKind::Medium,
                }
            },
            red,
            hp: entity.hp.max(0),
            max_hp: entity.max_hp,
            x,
            top,
        });
        let kind = match entity.row.as_str() {
            _ if projectile => "projectile",
            "KingTower" => "king",
            "PrincessTower" => "princess",
            _ => "unit",
        };
        let shown_state = match state {
            AnimationState::Attacking => "attacking",
            AnimationState::Moving => "moving",
            _ => "idle",
        };
        let flash_name = flash
            .as_ref()
            .filter(|_| !pick.as_ref().is_some_and(|p| p.4))
            .map(|(export, _)| export.as_str());
        probe_shown(
            entity,
            uid,
            kind,
            shown_state,
            pick,
            flash_name,
            (x, py),
            &parts,
        );
        Ok(Some(Plan {
            uid: 0,
            parts,
            shadow,
            ground_parts,
            sort_y: gy,
            air,
            hp,
            screen_anchor: [x, py],
            ground_anchor: [gx, gy],
        }))
    }

    /// Opacity for `BuffWhenNotAttacking = Invisibility` (Royal Ghost).
    /// Fully visible while deploying or inside a recorded attack window,
    /// then faded across `HideTimeMs` after `BuffWhenNotAttackingTime`
    /// without an attack. A predicted swing is not an attack: the ghost
    /// stays faded until a hit that actually lands.
    fn stealth_alpha(
        &self,
        row: &str,
        spawn_tick: u32,
        now_ms: f64,
        deploys: bool,
        impacts: &[f64],
    ) -> f64 {
        let Some(logic) = self.logic.row(row) else {
            return 1.;
        };
        if logic.str("BuffWhenNotAttacking") != Some("Invisibility") {
            return 1.;
        }
        invisibility_alpha(
            now_ms,
            spawn_tick as f64 * TICK_MS,
            deploys,
            logic.num("DeployTime").unwrap_or(1000.),
            logic.num("BuffWhenNotAttackingTime").unwrap_or(2000.),
            logic.num("HideTimeMs").unwrap_or(400.),
            logic.num("LoadTime").unwrap_or(0.),
            logic.num("HitSpeed").unwrap_or(1000.),
            impacts,
        )
    }

    /// Apply one frame of an original `filter_*` movie (root matrix and color)
    /// from the effects document. Returns false once its clip has ended.
    fn apply_filter(&self, export: Option<&str>, age_ms: f64, playback: &mut Playback) -> bool {
        let Some(export) = export else {
            return false;
        };
        if age_ms < 0. {
            return false;
        }
        let Some(scene) = self.scenes.get("effects") else {
            return false;
        };
        let Some(movie) = scene
            .document
            .exports
            .get(export)
            .and_then(|id| scene.document.movies.get(id))
        else {
            return false;
        };
        let index = (age_ms * movie.framerate as f64 / 1000.) as usize;
        if index >= movie.frame_element_counts.len() || movie.frame_element_counts[index] != 1 {
            return false;
        }
        let offset = movie.frame_elements_offset as usize
            + movie.frame_element_counts[..index]
                .iter()
                .map(|v| *v * 3)
                .sum::<usize>();
        let Some(elements) = scene.document.frame_elements.get(offset..offset + 3) else {
            return false;
        };
        if elements[1] != u16::MAX {
            playback.root_transform = scene
                .document
                .matrix_banks
                .get(movie.matrix_bank_index)
                .and_then(|bank| bank.get(elements[1] as usize))
                .copied();
        }
        if elements[2] != u16::MAX {
            playback.root_color = scene
                .document
                .color_banks
                .get(movie.matrix_bank_index)
                .and_then(|bank| bank.get(elements[2] as usize))
                .copied();
        }
        true
    }

    /// Original `ui.sc` health bars: the `bar` child's 100 frames are the
    /// depletion percentage. Own (blue) troops at full health show no bar, as in
    /// the original `health_bars.csv` (`ShowEnemyAlways` only).
    fn draw_hp_bar(&mut self, out: &mut RgbaImage, bar: &HpBar, field_scale: f64) -> Result<()> {
        if bar.kind != BarKind::Tower && !bar.red && bar.hp >= bar.max_hp {
            return Ok(());
        }
        let team = if bar.red { "enemy" } else { "player" };
        let export = match bar.kind {
            BarKind::Small => format!("hp_{team}_small"),
            BarKind::Medium => format!("hp_{team}_medium"),
            BarKind::High => format!("hp_{team}_high"),
            BarKind::Tower => format!("hp_{team}_tower_withNumber"),
        };
        let Some(scene) = self.scenes.get("ui") else {
            return self.fallback_bar(out, bar, field_scale);
        };
        let Some(movie) = scene
            .document
            .exports
            .get(&export)
            .and_then(|id| scene.document.movies.get(id))
        else {
            return self.fallback_bar(out, bar, field_scale);
        };
        let mut playback = Playback::default();
        // Frame 0 is a full bar and frame 99 an empty one.
        let percent = 99
            - ((bar.hp as f64 / bar.max_hp.max(1) as f64) * 99.)
                .round()
                .clamp(0., 99.) as usize;
        for (id, name) in movie.children_ids.iter().zip(&movie.children_names) {
            match name.as_str() {
                "bar" => {
                    playback.object_frames.insert(*id, percent);
                }
                "downgrade" | "chef_upgrade" | "crown" => {
                    playback.hidden_objects.insert(*id);
                }
                "hpNumber" => {
                    playback.render_text = true;
                    if let Some(text_id) = scene
                        .document
                        .movies
                        .get(id)
                        .and_then(|m| m.children_ids.first())
                    {
                        playback.text_values.insert(*text_id, bar.hp.to_string());
                    }
                }
                "level" => {
                    playback.text_values.insert(*id, String::new());
                }
                _ => {}
            }
        }
        let ui_scale = field_scale
            * if bar.kind == BarKind::Tower {
                0.82
            } else {
                0.72
            };
        let Some(sprite) = self.fit("ui", &export, playback, 0, ui_scale, false)? else {
            return self.fallback_bar(out, bar, field_scale);
        };
        let width = sprite.image.width() as f64;
        let x = bar.x - width / 2.;
        let y = bar.top - sprite.image.height() as f64 - 2. * field_scale;
        blit::blit(
            out,
            &sprite.image,
            x.round() as i64,
            y.round() as i64,
            1.,
            Blend::Normal,
            [0; 3],
        );
        Ok(())
    }

    fn fallback_bar(&mut self, out: &mut RgbaImage, bar: &HpBar, field_scale: f64) -> Result<()> {
        let width = if bar.kind == BarKind::Tower { 48. } else { 32. };
        let w = (width * field_scale).round().max(20.) as i32;
        let x = bar.x as i32;
        let by = (bar.top - 9. * field_scale).round() as i32;
        fill_rect(out, x - w / 2 - 1, by - 1, w + 2, 6, DARK);
        fill_rect(
            out,
            x - w / 2,
            by,
            (w as f64 * bar.hp as f64 / bar.max_hp.max(1) as f64).round() as i32,
            4,
            if bar.red { RED } else { BLUE },
        );
        if bar.kind == BarKind::Tower {
            self.text(
                out,
                &bar.hp.to_string(),
                x.max(0) as u32,
                (by - 15).max(0) as u32,
                12. * field_scale as f32,
                WHITE,
                1,
            )?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
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
            if self.text_cache.len() > 400 {
                self.text_cache.clear();
            }
            self.text_cache.insert(key.clone(), rendered.image);
        }
        let rendered = self
            .text_cache
            .get(&key)
            .ok_or("cached font render missing")?;
        let x = cx as i64 - rendered.width() as i64 / 2;
        scene::overlay(out, rendered, x, y as i64);
        Ok(())
    }
}

fn draw_plan(out: &mut RgbaImage, plan: &Plan) {
    for (index, part) in plan.parts.iter().enumerate() {
        scene::scope_anchored(
            format!("entity-{}-part-{index}", plan.uid),
            plan.screen_anchor,
            || draw_part(out, part),
        );
    }
}

/// Record one entity's presentation for `clash-visual-check`; no-op otherwise.
#[allow(clippy::too_many_arguments)]
fn probe_shown(
    entity: &Entity,
    uid: u64,
    kind: &'static str,
    state: &'static str,
    pick: Option<(String, usize, bool, f32, bool)>,
    flash: Option<&str>,
    anchor: (f64, f64),
    parts: &[Part],
) {
    if !crate::probe::active() {
        return;
    }
    let (export, frame, flip, alpha, deploying) =
        pick.unwrap_or_else(|| (String::new(), 0, false, 1., false));
    let bbox = parts
        .iter()
        .map(|p| {
            let x0 = p.x.round() + p.sprite.dx as f64;
            let y0 = p.y.round() + p.sprite.dy as f64;
            [
                x0,
                y0,
                x0 + p.sprite.image.width() as f64,
                y0 + p.sprite.image.height() as f64,
            ]
        })
        .reduce(|a, b| {
            [
                a[0].min(b[0]),
                a[1].min(b[1]),
                a[2].max(b[2]),
                a[3].max(b[3]),
            ]
        });
    let shown = crate::probe::Shown {
        uid,
        address: entity.address,
        native_id: entity.native_entity_id,
        row: entity.row.clone(),
        side: entity.side,
        kind,
        state,
        export,
        frame,
        flip,
        deploying,
        flash: flash.map(str::to_owned),
        alpha,
        x: entity.x,
        y: entity.y,
        z: entity.z,
        hp: entity.hp,
        max_hp: entity.max_hp,
        anchor: [anchor.0, anchor.1],
        bbox,
        coverage: parts
            .iter()
            .map(|p| crate::probe::coverage(&p.sprite.image))
            .sum(),
        parts: parts.len(),
    };
    crate::probe::with(|r| r.shown.push(shown));
}

fn draw_part(out: &mut RgbaImage, part: &Part) {
    blit::blit(
        out,
        &part.sprite.image,
        part.x.round() as i64 + part.sprite.dx,
        part.y.round() as i64 + part.sprite.dy,
        1.,
        Blend::Normal,
        [0; 3],
    );
}

/// Attack clips: `phase_ms` is already the position in the authored clip,
/// aligned so its `action_frame` coincides with the observed/predicted impact.
fn set_attack_frame(
    scene: &Scene,
    selection: &mut animation::PlaybackSelection,
    state: AnimationState,
    phase_ms: f64,
) {
    if state != AnimationState::Attacking {
        return;
    }
    let Some(movie) = scene
        .document
        .exports
        .get(&selection.export)
        .and_then(|id| scene.document.movies.get(id))
    else {
        return;
    };
    let count = movie.frame_element_counts.len().max(1);
    let index = animation::authored_frame_index(phase_ms, movie.framerate as f64, count, false);
    selection.frame_index = index;
    selection.playback.frame = Some(index);
    selection.playback.label = None;
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct MeleeCandidate {
    addr: u64,
    hit: f64,
    /// Expected native loss of one swing, when the original row has damage.
    damage: Option<f64>,
    /// Within the cadence predicted from earlier swings.
    on_time: bool,
}

/// Melee attackers that dealt a target's HP `loss`. Prefer the set whose
/// expected damage sums to the loss; otherwise the loss also includes
/// other damage, and candidates on their cadence (or the closest one
/// without known damage) are credited.
/// Record one attributed hit on the rehearsal actor and on the schedule.
/// The schedule key is the native id so a reused address does not inherit
/// the previous object's swings.
fn note_scheduled(
    scheduled: &mut BTreeMap<u64, Vec<f64>>,
    actors: &mut BTreeMap<u64, Actor>,
    addr: u64,
    at_ms: f64,
    hit: f64,
) {
    let Some(actor) = actors.get_mut(&addr) else {
        return;
    };
    let key = actor.last.native_entity_id.unwrap_or(addr);
    let last = actor.impacts.last().copied();
    actor.register_impact(at_ms, hit);
    if actor.impacts.last().copied() != last {
        scheduled.entry(key).or_default().push(at_ms);
    }
}

/// Royal Ghost opacity from recorded hits. `1` while deploying or inside an
/// attack window (including the checker's 150 ms lead-in), then a fade to
/// 0.28 after `wait_ms` without an attack. A hit that has not started yet
/// does not keep the ghost solid.
fn invisibility_alpha(
    now_ms: f64,
    spawn_ms: f64,
    deploys: bool,
    deploy_ms: f64,
    wait_ms: f64,
    hide_ms: f64,
    load_ms: f64,
    hit_ms: f64,
    impacts: &[f64],
) -> f64 {
    let hide = hide_ms.max(1.);
    if deploys && now_ms < spawn_ms + deploy_ms {
        return 1.;
    }
    let mut last_end = if deploys {
        spawn_ms + deploy_ms
    } else {
        spawn_ms
    };
    for &impact in impacts {
        let start = impact - load_ms;
        let end = impact + (hit_ms - load_ms).max(0.);
        if now_ms >= start - 150. && now_ms <= end {
            return 1.;
        }
        if end <= now_ms && end > last_end {
            last_end = end;
        }
    }
    let since = now_ms - last_end;
    if since <= wait_ms {
        return 1.;
    }
    let fade = ((since - wait_ms) / hide).clamp(0., 1.);
    let faded = 1. - fade * 0.72;
    // Rise across the 100 ms before the visible window so a fully faded
    // sprite does not jump to solid in one frame. The peak stays under the
    // invisible-mass ceiling; the window itself is solid.
    let mut eased = faded;
    for &impact in impacts {
        let lead = impact - load_ms - 150.;
        if now_ms < lead && lead - now_ms <= 100. {
            let u = 1. - ((lead - now_ms) / 100.).clamp(0., 1.);
            eased = eased.max(faded + (0.58 - faded).max(0.) * u);
        }
    }
    eased
}

fn attribute_loss(loss: f64, candidates: &[MeleeCandidate]) -> Vec<MeleeCandidate> {
    // A swing cannot be part of a smaller loss.
    let candidates: Vec<MeleeCandidate> = candidates
        .iter()
        .filter(|c| c.damage.is_none_or(|d| loss >= d * 0.96 - 2.))
        .copied()
        .collect();
    let known: Vec<&MeleeCandidate> = candidates.iter().filter(|c| c.damage.is_some()).collect();
    let mut best: Option<(usize, usize, Vec<MeleeCandidate>)> = None;
    for mask in 1u32..(1 << known.len().min(10)) {
        let set: Vec<MeleeCandidate> = known
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, c)| **c)
            .collect();
        let sum: f64 = set.iter().filter_map(|c| c.damage).sum();
        if (sum - loss).abs() > loss * 0.04 + 2. {
            continue;
        }
        // Most attackers on their cadence, then the fewest attackers.
        let rank = (
            set.iter().filter(|c| c.on_time).count(),
            usize::MAX - set.len(),
        );
        if best.as_ref().is_none_or(|(t, n, _)| rank > (*t, *n)) {
            best = Some((rank.0, rank.1, set));
        }
    }
    if let Some((_, _, set)) = best {
        return set;
    }
    let on_time: Vec<MeleeCandidate> = candidates.iter().filter(|c| c.on_time).copied().collect();
    if !on_time.is_empty() {
        return on_time;
    }
    // Without damage data there is no better evidence than the cadence.
    candidates
        .iter()
        .filter(|c| c.damage.is_none())
        .take(1)
        .copied()
        .collect()
}

/// Select the view of an original `turret` child timeline for a yaw in
/// degrees clockwise from screen-up. The authored bank is a half-turn from
/// north through east to south (Cannon: 19 views, 10 degrees apart); the
/// western half mirrors it, as with the King tower turret.
fn aim_turret_view(scene: &Scene, selection: &mut animation::PlaybackSelection, yaw: f64) {
    let Some(movie) = scene
        .document
        .exports
        .get(&selection.export)
        .and_then(|id| scene.document.movies.get(id))
    else {
        return;
    };
    let Some(turret) = movie
        .children_names
        .iter()
        .position(|name| name == "turret")
        .map(|i| movie.children_ids[i])
    else {
        return;
    };
    let views = scene
        .document
        .movies
        .get(&turret)
        .map_or(1, |m| m.frame_element_counts.len().max(1));
    let view = (yaw.abs().min(180.) / 180. * (views - 1) as f64).round() as usize;
    selection.frame_index = 0;
    selection.playback.frame = Some(0);
    selection.playback.time_seconds = None;
    selection.playback.label = None;
    selection.playback.object_frames.insert(turret, view);
    selection.flip_horizontal = yaw < 0.;
}

fn fill_rect(img: &mut RgbaImage, x: i32, y: i32, w: i32, h: i32, color: Rgba<u8>) {
    scene::fill_rect(img, x, y, w, h, color);
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

    #[test]
    fn other_damage_on_a_melee_target_is_not_a_swing() {
        let knight = |addr, on_time| MeleeCandidate {
            addr,
            hit: 1200.,
            damage: Some(202.2),
            on_time,
        };
        let skeleton = MeleeCandidate {
            addr: 3,
            hit: 1100.,
            damage: Some(81.),
            on_time: false,
        };
        // An Archer arrow (112) on the Knight's tower is nobody's swing here.
        assert!(attribute_loss(112., &[knight(1, true)]).is_empty());
        // The Knight's own hit, even off its predicted cadence.
        assert_eq!(
            attribute_loss(202., &[knight(1, false)]),
            vec![knight(1, false)]
        );
        // Simultaneous hits split by amount; the on-cadence Knight is preferred.
        assert_eq!(
            attribute_loss(283., &[knight(1, false), knight(2, true), skeleton]),
            vec![knight(2, true), skeleton]
        );
        // Knight plus an arrow: the Knight is credited only on its cadence.
        assert_eq!(
            attribute_loss(314., &[knight(1, true)]),
            vec![knight(1, true)]
        );
        assert!(attribute_loss(314., &[knight(1, false)]).is_empty());
    }

    #[test]
    fn turret_views_cover_a_mirrored_half_turn() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let scene = Scene::load(&root, "building_basic_cannon").unwrap();
        let view = |yaw| {
            let mut selection = animation::PlaybackSelection {
                export: "building_cannon".into(),
                playback: Playback {
                    time_seconds: Some(0.4),
                    ..Playback::default()
                },
                frame_index: 7,
                direction_index: None,
                flip_horizontal: false,
                matched_state: true,
            };
            aim_turret_view(&scene, &mut selection, yaw);
            assert_eq!(selection.playback.time_seconds, None);
            let frames: Vec<usize> = selection.playback.object_frames.values().copied().collect();
            (frames, selection.flip_horizontal)
        };
        assert_eq!(view(0.), (vec![0], false));
        assert_eq!(view(90.), (vec![9], false));
        assert_eq!(view(-90.), (vec![9], true));
        assert_eq!(view(180.), (vec![18], false));
    }

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
            ..Entity::default()
        }
    }

    #[test]
    fn original_deploy_filter_ends_instead_of_repeating() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let painter = Painter::load(&root, 540, 900).expect("source bundle");
        let filter = Some("filter_deploy_unit_default");
        let mut start = Playback::default();
        assert!(painter.apply_filter(filter, 0., &mut start));
        let transform = start
            .root_transform
            .expect("Knight source deploy transform");
        assert!(
            transform.ty < -60.0,
            "authored deployment begins above the board"
        );
        assert!(start.root_color.expect("source deploy color").alpha < 255);
        let mut done = Playback::default();
        assert!(!painter.apply_filter(filter, 600., &mut done));
        assert!(done.root_transform.is_none());
        assert!(done.root_color.is_none());
    }

    #[test]
    fn renders_original_assets_for_initial_towers() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let mut painter = Painter::load(&root, 720, 1080).expect("load source asset bundle");
        let frame = BattleFrame {
            human: None,
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
            hands: Default::default(),
            controller_names: Default::default(),
            plays: Vec::new(),
            match_id: 0,
        };
        let mut frame = frame;
        let mut image = painter.render(&frame).expect("render initial tower frame");
        let started = std::time::Instant::now();
        for i in 1..=31 {
            frame.tick = i;
            frame.elapsed_ms = u64::from(i) * 50;
            painter.observe(&frame);
            for sub in 0..3 {
                image = painter
                    .draw(i as f64 - 1. + sub as f64 / 3.)
                    .expect("render cached tower frame");
            }
        }
        eprintln!(
            "warm spectator frame: {:.2} ms",
            started.elapsed().as_secs_f64() * 1000.0 / 93.0
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

    #[test]
    fn inferno_beam_requires_a_live_enemy_in_range_after_warmup() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let mut painter = Painter::load(&root, 540, 900).expect("source bundle");
        let mut source = tower(10, "InfernoTower", 1, 9000, 20000);
        source.native_focus_entity = Some(11);
        let target = tower(11, "HogRider", 0, 9000, 17000);
        let mut actor = Actor::new(1, &source, 0, false);
        actor.engaged_since_ms = Some(0.);
        painter.actors.insert(source.address, actor);
        let layout = Layout {
            field_x: 0.,
            field_y: 0.,
            field_w: 540.,
            field_h: 828.,
            world_w: 18000.,
            world_h: 32000.,
            scale: 1.,
        };
        let anchors = BTreeMap::from([
            (10, ((250., 250.), (250., 280.))),
            (11, ((250., 350.), (250., 380.))),
        ]);
        let mut render = |source: Entity, target: Entity, now| {
            let mut image = RgbaImage::new(540, 900);
            painter
                .draw_targeted_beams(
                    &mut image,
                    &[(source, 1, [0.; 3]), (target, 2, [0.; 3])],
                    &anchors,
                    now,
                    &layout,
                )
                .unwrap();
            image.pixels().any(|p| p[3] != 0)
        };
        assert!(render(source.clone(), target.clone(), 3000.));
        assert!(!render(source.clone(), target.clone(), 100.));
        let mut invalid = target.clone();
        invalid.y = 1000;
        assert!(!render(source.clone(), invalid, 3000.));
        let mut invalid = target.clone();
        invalid.hp = 0;
        assert!(!render(source.clone(), invalid, 3000.));
        let mut invalid = target.clone();
        invalid.side = source.side;
        assert!(!render(source.clone(), invalid, 3000.));
        source.native_focus_entity = None;
        assert!(!render(source, target, 3000.));
    }

    #[test]
    fn projectile_rows_without_observed_bindings_use_original_logic_exports() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let painter = Painter::load(&root, 540, 900).expect("source bundle");
        if painter.logic.rows.is_empty() {
            eprintln!("presentation.json absent; skipping");
            return;
        }
        for (row, export) in [
            ("MusketeerProjectile", "projectile_cannonball_small"),
            ("TowerCannonball", "projectile_cannonball_large"),
            ("FireballSpell", "fireball_projectile1"),
            ("LogProjectileRolling", "log_roll_projectile"),
            ("IceSpiritsProjectile", "projectile_icespirit"),
            ("PrincessProjectile", "projectile_princess"),
        ] {
            let visual = painter
                .bindings
                .get(row)
                .unwrap_or_else(|| panic!("{row} bound"));
            assert_eq!(visual.prefix, export);
            assert_eq!(visual.document, "effects");
            assert!(painter.is_projectile(row));
        }
    }

    #[test]
    fn card_list_keeps_spaces_and_unknown_metadata_does_not_load_everything() {
        assert_eq!(
            parse_card_list("P.E.K.K.A, Hog Rider,Knight,,"),
            vec!["P.E.K.K.A".to_owned(), "Hog Rider".into(), "Knight".into()]
        );
        let old = serde_json::json!({"documents":[{"name":"arena"}]});
        assert!(documents_for_cards(&old, &[]).unwrap().is_none());
        assert!(documents_for_cards(&old, &["Knight".into()]).is_err());
        let bundle = serde_json::json!({
            "scene": {
                "always_documents": ["arena", "ui"],
                "card_dependency_documents": {
                    "Hog Rider": ["chr_hog", "ui"],
                    "Knight": ["chr_knight"]
                }
            }
        });
        let set = documents_for_cards(&bundle, &parse_card_list("Hog Rider,Knight"))
            .unwrap()
            .unwrap();
        assert_eq!(
            set.into_iter().collect::<Vec<_>>(),
            vec!["arena", "chr_hog", "chr_knight", "ui"]
        );
        let missing = documents_for_cards(&bundle, &["Missing".into()]).unwrap_err();
        assert!(missing.contains("Missing"), "{missing}");
        let dir = std::env::temp_dir().join(format!(
            "spectator-cards-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("assets.json"),
            r#"{"documents":[{"name":"arena"}],"scene":{"always_documents":["arena"],"card_dependency_documents":{"Knight":["chr_knight"]}}}"#,
        )
        .unwrap();
        match Painter::load_for_cards(&dir, 540, 900, &["Knight".into()]) {
            Err(err) => assert!(err.contains("not in the bundle"), "{err}"),
            Ok(_) => panic!("card selection must not load a bundle that lacks the documents"),
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn invisibility_follows_recorded_hits_and_ignores_a_later_one() {
        let solid = |now: f64, hits: &[f64]| {
            invisibility_alpha(now, 0., true, 1000., 2000., 400., 1200., 1800., hits)
        };
        assert_eq!(solid(500., &[]), 1.);
        assert_eq!(solid(13750., &[14925.]), 1.);
        assert_eq!(solid(13575., &[14925.]), 1.);
        let faded = solid(13400., &[14925.]);
        assert!((faded - 0.28).abs() < 1e-9, "{faded}");
        let rising = solid(13525., &[14925.]);
        assert!(rising > 0.4 && rising < 0.6, "{rising}");
        let between = invisibility_alpha(
            16050.,
            5000.,
            true,
            1000.,
            2000.,
            400.,
            1200.,
            1800.,
            &[12875., 20000.],
        );
        assert!((between - 0.28).abs() < 1e-9, "{between}");
        assert_eq!(
            invisibility_alpha(12875., 5000., true, 1000., 2000., 400., 1200., 1800., &[12875.]),
            1.
        );
    }
}
