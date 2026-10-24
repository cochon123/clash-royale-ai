//! Shared observe/step/draw loop for the wasm painter and the native parity recorder.
//!
//! The packet object matches `clash-web-scene`: same fields, same display-list
//! schema. Textures stay in an in-memory RGBA cache instead of PNG files.

use crate::clock::{Flow, SceneDisplayClock};
use crate::draw::{Painter, parse_card_list};
use crate::frame::{BattleFrame, Entity};
use crate::scene::{CaptureGuard, TextureCache};
use serde::Deserialize;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::Path;
use std::rc::Rc;

#[derive(Clone, Deserialize)]
struct Snapshot {
    #[serde(flatten)]
    frame: BattleFrame,
    #[serde(default)]
    runtime_status: String,
    #[serde(default = "one")]
    playback_speed: f64,
}

fn one() -> f64 {
    1.
}

#[derive(Clone)]
struct Horizon {
    match_id: u64,
    oldest: u32,
    latest: u32,
}

fn status(snapshot: &Snapshot) -> &str {
    if snapshot.frame.terminated {
        "ended"
    } else if snapshot.runtime_status.is_empty() {
        "running"
    } else {
        &snapshot.runtime_status
    }
}

fn flow_of(snapshot: &Snapshot) -> Flow {
    match status(snapshot) {
        "paused" => Flow::Paused,
        "ended" | "closed" => Flow::Finished,
        _ => Flow::Running,
    }
}

pub struct ExportSession {
    painter: Painter,
    cache: Rc<RefCell<TextureCache>>,
    clock: SceneDisplayClock,
    horizon: Option<Horizon>,
    latest: Option<Snapshot>,
    sequence: u64,
    last_display: Option<f64>,
    force_emit: bool,
    pub observe_count: u64,
    pub steps: u64,
    pub packets: u64,
    width: u32,
    height: u32,
    result: Vec<u8>,
}

impl ExportSession {
    pub fn open(
        root: &Path,
        cards: &[String],
        width: u32,
        height: u32,
        lag_ticks: f64,
    ) -> Result<Self, String> {
        let width = width.max(1);
        let height = height.max(1);
        let painter = Painter::load_for_cards(root, width, height, cards)?;
        let mut clock = SceneDisplayClock::new();
        clock.set_lag_ticks(lag_ticks);
        Ok(Self {
            painter,
            cache: Rc::new(RefCell::new(TextureCache::memory())),
            clock,
            horizon: None,
            latest: None,
            sequence: 0,
            last_display: None,
            force_emit: false,
            observe_count: 0,
            steps: 0,
            packets: 0,
            width,
            height,
            result: Vec::new(),
        })
    }

    pub fn set_lag_ticks(&mut self, lag_ticks: f64) {
        self.clock.set_lag_ticks(lag_ticks);
    }

    pub fn observe_json(&mut self, bytes: &[u8]) -> Result<(), String> {
        let snapshot: Snapshot =
            serde_json::from_slice(bytes).map_err(|e| format!("invalid snapshot: {e}"))?;
        self.observe_snapshot(snapshot)
    }

    fn observe_snapshot(&mut self, snapshot: Snapshot) -> Result<(), String> {
        let forced = self.latest.as_ref().is_none_or(|old| {
            old.frame.match_id != snapshot.frame.match_id
                || snapshot.frame.tick < old.frame.tick
                || status(old) != status(&snapshot)
                    && matches!(status(&snapshot), "paused" | "ended" | "closed")
        });
        self.painter.observe(&snapshot.frame);
        self.note_horizon(&snapshot);
        self.latest = Some(snapshot);
        self.observe_count += 1;
        if forced {
            self.force_emit = true;
        }
        Ok(())
    }

    fn note_horizon(&mut self, snapshot: &Snapshot) {
        let tick = snapshot.frame.tick;
        let match_id = snapshot.frame.match_id;
        let reset = self
            .horizon
            .as_ref()
            .is_none_or(|seen| seen.match_id != match_id || tick < seen.latest);
        if reset {
            self.clock.reset();
            self.last_display = None;
            self.force_emit = true;
            self.horizon = Some(Horizon {
                match_id,
                oldest: tick,
                latest: tick,
            });
        } else if let Some(seen) = self.horizon.as_mut() {
            if tick > seen.latest {
                seen.latest = tick;
            }
        }
    }

    /// Advance the display clock and draw. `1` is a new packet, `0` means the
    /// clock did not move and the previous packet is still current.
    pub fn step(&mut self, dt_seconds: f64) -> Result<i32, String> {
        self.steps += 1;
        self.cache.borrow_mut().advance_clock(dt_seconds);
        let Some(snapshot) = self.latest.clone() else {
            return Ok(0);
        };
        let Some(horizon) = self.horizon.clone() else {
            return Ok(0);
        };
        let flow = flow_of(&snapshot);
        let display = self.clock.step(
            horizon.oldest,
            horizon.latest,
            snapshot.playback_speed,
            flow,
            dt_seconds,
        );
        let moved = self
            .last_display
            .map(|previous| (previous - display).abs() > 1e-9)
            .unwrap_or(true);
        if !moved && !self.force_emit {
            return Ok(0);
        }
        self.emit(&snapshot, display)?;
        self.last_display = Some(display);
        self.force_emit = false;
        self.packets += 1;
        Ok(1)
    }

    fn emit(&mut self, snapshot: &Snapshot, display_tick: f64) -> Result<(), String> {
        let frame = &snapshot.frame;
        let oldest = self.painter.oldest_tick().unwrap_or(frame.tick) as f64;
        let latest = self.painter.latest_tick().unwrap_or(frame.tick) as f64;
        let display_tick = display_tick.clamp(oldest, latest);
        #[cfg(not(feature = "wasm"))]
        let started = std::time::Instant::now();
        let capture = CaptureGuard::begin(self.cache.clone())?;
        self.painter.draw(display_tick)?;
        let scene = capture.finish()?;
        #[cfg(not(feature = "wasm"))]
        let render_ms = started.elapsed().as_secs_f64() * 1000.;
        #[cfg(feature = "wasm")]
        let render_ms = 0.0;
        self.sequence += 1;
        let layout = self
            .painter
            .snapshot_at(display_tick)
            .as_ref()
            .map(|shown| self.painter.controls_layout(shown))
            .unwrap_or_else(|| self.painter.controls_layout(frame));
        let cache = self.cache.borrow();
        let packet = json!({
            "v": 1,
            "sequence": self.sequence,
            "tick": frame.tick,
            "display_tick": display_tick,
            "match_id": frame.match_id,
            "width": self.width,
            "height": self.height,
            "clear": [19, 27, 31, 255],
            "layout": {"field": layout.field, "world": layout.world, "cards": layout.cards},
            "textures": scene.textures,
            "draws": scene.draws,
            "status": status(snapshot),
            "playback_speed": snapshot.playback_speed,
            "render_ms": render_ms,
            "texture_count": cache.len(),
            "texture_bytes": cache.encoded_bytes,
            "evicted": scene.evicted,
            "unresolved_rows": self.painter.unresolved_rows,
        });
        drop(cache);
        self.result = serde_json::to_vec(&packet).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn packet(&self) -> &[u8] {
        &self.result
    }

    pub fn copy_rgba(&self, id: &str) -> Option<Vec<u8>> {
        self.cache.borrow().rgba(id).map(|bytes| bytes.to_vec())
    }

    pub fn release_texture(&mut self, id: &str) -> bool {
        self.cache.borrow_mut().release_rgba(id)
    }

    pub fn cache_bytes(&self) -> u64 {
        self.cache.borrow().encoded_bytes
    }

    pub fn texture_count(&self) -> usize {
        self.cache.borrow().len()
    }

    pub fn reset(&mut self) {
        self.painter.reset_round();
        self.clock.reset();
        self.horizon = None;
        self.latest = None;
        self.sequence = 0;
        self.last_display = None;
        self.force_emit = false;
        self.result.clear();
    }

    /// Drop published RGBA. Not part of `painter_reset`: the page keeps ids
    /// across a resync. The address gate clears so each pass starts empty.
    pub fn clear_textures(&mut self) {
        self.cache.borrow_mut().clear();
    }

    pub fn stats_value(&self) -> Value {
        json!({
            "observe_count": self.observe_count,
            "steps": self.steps,
            "packets": self.packets,
            "cache_bytes": self.cache_bytes(),
            "texture_count": self.texture_count(),
            "draw_ms": Value::Null,
            "draw_ms_source": "host",
        })
    }
}

pub fn parse_cards(raw: &str) -> Vec<String> {
    parse_card_list(raw)
}

/// Rewrite `address` and `native_focus_entity` through one map.
pub fn remap_addresses(frame: &mut BattleFrame, map: &BTreeMap<u64, u64>) {
    for entity in &mut frame.entities {
        remap_entity(entity, map, false);
    }
}

/// Also rewrite `native_entity_id` and `native_focus_entity_id`.
pub fn remap_addresses_and_ids(frame: &mut BattleFrame, map: &BTreeMap<u64, u64>) {
    for entity in &mut frame.entities {
        remap_entity(entity, map, true);
    }
}

fn remap_entity(entity: &mut Entity, map: &BTreeMap<u64, u64>, ids: bool) {
    if let Some(next) = map.get(&entity.address) {
        entity.address = *next;
    }
    if let Some(focus) = entity.native_focus_entity {
        if let Some(next) = map.get(&focus) {
            entity.native_focus_entity = Some(*next);
        }
    }
    if ids {
        if let Some(id) = entity.native_entity_id {
            if let Some(next) = map.get(&id) {
                entity.native_entity_id = Some(*next);
            }
        }
        if let Some(id) = entity.native_focus_entity_id {
            if let Some(next) = map.get(&id) {
                entity.native_focus_entity_id = Some(*next);
            }
        }
    }
}

/// Deterministic permutation of the addresses present in `frames`.
pub fn address_permutation(frames: &[BattleFrame], seed: u64) -> BTreeMap<u64, u64> {
    let mut addresses: Vec<u64> = frames
        .iter()
        .flat_map(|frame| frame.entities.iter().map(|entity| entity.address))
        .collect();
    addresses.sort_unstable();
    addresses.dedup();
    let mut order = addresses.clone();
    let mut state = seed | 1;
    for index in (1..order.len()).rev() {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        let swap = (state as usize) % (index + 1);
        order.swap(index, swap);
    }
    // A fixed point is still a valid permutation. If every address landed on
    // itself, rotate once so the gate actually changes values.
    if order == addresses && addresses.len() > 1 {
        order.rotate_left(1);
    }
    addresses.into_iter().zip(order).collect()
}

const REL_TOLERANCE: f64 = 1e-9;

pub fn json_close(left: &Value, right: &Value) -> Result<(), String> {
    json_close_at(left, right, "$")
}

fn json_close_at(left: &Value, right: &Value, path: &str) -> Result<(), String> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => {
            let a = a.as_f64().ok_or_else(|| format!("{path}: non-finite left"))?;
            let b = b.as_f64().ok_or_else(|| format!("{path}: non-finite right"))?;
            let scale = a.abs().max(b.abs()).max(1.);
            if (a - b).abs() <= REL_TOLERANCE * scale {
                Ok(())
            } else {
                Err(format!("{path}: {a} != {b}"))
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return Err(format!("{path}: length {} != {}", a.len(), b.len()));
            }
            for (index, (left, right)) in a.iter().zip(b).enumerate() {
                json_close_at(left, right, &format!("{path}[{index}]"))?;
            }
            Ok(())
        }
        (Value::Object(a), Value::Object(b)) => {
            if a.len() != b.len() {
                return Err(format!("{path}: field count {} != {}", a.len(), b.len()));
            }
            for (key, left) in a {
                let Some(right) = b.get(key) else {
                    return Err(format!("{path}: missing {key}"));
                };
                json_close_at(left, right, &format!("{path}.{key}"))?;
            }
            Ok(())
        }
        _ if left == right => Ok(()),
        _ => Err(format!("{path}: {left} != {right}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entity(address: u64, focus: Option<u64>) -> Entity {
        Entity {
            address,
            native_focus_entity: focus,
            row: "Knight".into(),
            ..Entity::default()
        }
    }

    fn frame(entities: Vec<Entity>) -> BattleFrame {
        BattleFrame {
            tick: 1,
            elapsed_ms: 50,
            terminated: false,
            winner_side: None,
            arena_size: [18000, 32000],
            elixir: [5., 5.],
            entities,
            hands: Default::default(),
            controller_names: Default::default(),
            plays: Vec::new(),
            match_id: 1,
            human: None,
        }
    }

    #[test]
    fn remap_uses_one_map_for_address_and_focus() {
        let mut sample = frame(vec![entity(10, Some(20))]);
        let map = BTreeMap::from([(10, 2), (20, 4)]);
        remap_addresses(&mut sample, &map);
        assert_eq!(sample.entities[0].address, 2);
        assert_eq!(sample.entities[0].native_focus_entity, Some(4));
    }

    #[test]
    fn permutation_changes_values_and_is_bijective() {
        let frames = vec![frame(vec![entity(5, Some(9)), entity(9, None)])];
        let map = address_permutation(&frames, 0xC0FFEE);
        assert_eq!(map.len(), 2);
        let mut images: Vec<u64> = map.values().copied().collect();
        images.sort_unstable();
        images.dedup();
        assert_eq!(images.len(), 2);
        assert!(map.iter().any(|(from, to)| from != to));
    }

    #[test]
    fn json_close_accepts_relative_tolerance() {
        json_close(&json!({"m": [1.0, 0.0]}), &json!({"m": [1.0 + 1e-12, 0.0]})).unwrap();
        assert!(json_close(&json!([1.0]), &json!([1.01])).is_err());
    }
}
