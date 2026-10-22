//! Per-entity presentation state derived from consecutive native snapshots.
//!
//! The engine does not expose animation states, so these are explicit
//! presentation inferences: movement with a short hysteresis (some units hold
//! still for single ticks while walking), engagement from the native focus
//! target and original `Range`, and an attack cadence from the original
//! `HitSpeed`/`LoadTime` that is re-synchronized to every observed impact.
use crate::animation::{HeadingSource, PresentationHeading};
use crate::frame::Entity;

/// Ticks without displacement before a walking unit is shown as stopped.
pub const MOVE_HOLD_TICKS: u32 = 3;
pub const TICK_MS: f64 = 50.;

pub struct Actor {
    pub uid: u64,
    pub row: String,
    pub native_id: Option<u64>,
    pub side: i32,
    pub spawn_tick: u32,
    /// False for entities already present when the match was first observed.
    pub deploys: bool,
    pub last: Entity,
    pub last_tick: u32,
    /// Smoothed native displacement per tick (x, y, z).
    pub velocity: [f64; 3],
    pub heading: Option<PresentationHeading>,
    pub last_move_tick: Option<u32>,
    pub moving: bool,
    /// Recent walking intervals (start, end) in game ms; the display clock
    /// trails observation, so state is kept by time rather than as a flag.
    pub walks: Vec<(f64, Option<f64>)>,
    pub engaged_since_ms: Option<f64>,
    pub impacts: Vec<f64>,
    pub predicted_ms: Option<f64>,
    /// Damage filter exports and start times.
    pub flashes: Vec<(String, f64)>,
    /// Launch point of a projectile (native x, y, z) for its first interval.
    pub origin: Option<[f64; 3]>,
    pub max_z: i32,
    /// Displayed turret yaw (degrees clockwise from screen-up) and the
    /// display time it was last advanced.
    pub turret: Option<(f64, f64)>,
    /// Native loss of the last attributed melee hit.
    pub hit_damage: Option<i32>,
}

impl Actor {
    pub fn new(uid: u64, entity: &Entity, tick: u32, deploys: bool) -> Self {
        Self {
            uid,
            row: entity.row.clone(),
            native_id: entity.native_entity_id,
            side: entity.side,
            spawn_tick: tick,
            deploys,
            last: entity.clone(),
            last_tick: tick,
            velocity: [0.; 3],
            heading: None,
            last_move_tick: None,
            moving: false,
            walks: Vec::new(),
            engaged_since_ms: None,
            impacts: Vec::new(),
            predicted_ms: None,
            flashes: Vec::new(),
            origin: None,
            max_z: entity.z,
            turret: None,
            hit_damage: None,
        }
    }

    /// Same native object (addresses are reused after removal).
    pub fn matches(&self, entity: &Entity) -> bool {
        self.row == entity.row
            && match (self.native_id, entity.native_entity_id) {
                (Some(a), Some(b)) => a == b,
                _ => true,
            }
    }

    pub fn observe(&mut self, entity: &Entity, tick: u32) {
        let dt = tick.saturating_sub(self.last_tick).max(1) as f64;
        let delta = [
            (entity.x - self.last.x) as f64 / dt,
            (entity.y - self.last.y) as f64 / dt,
            (entity.z - self.last.z) as f64 / dt,
        ];
        let moved = entity.x != self.last.x || entity.y != self.last.y;
        if moved {
            for (v, d) in self.velocity.iter_mut().zip(delta) {
                *v = if *v == 0. { d } else { 0.55 * *v + 0.45 * d };
            }
            if !self.moving {
                self.moving = true;
                self.walks.push((self.last_tick as f64 * TICK_MS, None));
                if self.walks.len() > 8 {
                    self.walks.remove(0);
                }
            }
            self.last_move_tick = Some(tick);
        } else {
            self.velocity[2] = delta[2];
            if self.moving
                && tick.saturating_sub(self.last_move_tick.unwrap_or(tick)) > MOVE_HOLD_TICKS
            {
                self.moving = false;
                if let Some(walk) = self.walks.last_mut() {
                    walk.1 = Some(self.last_move_tick.unwrap_or(tick) as f64 * TICK_MS);
                }
            }
        }
        self.max_z = self.max_z.max(entity.z);
        self.last = entity.clone();
        self.last_tick = tick;
    }

    /// Start of the walk containing display time `t_ms`, if walking then.
    pub fn walk_at(&self, t_ms: f64) -> Option<f64> {
        self.walks
            .iter()
            .rev()
            .find(|(start, end)| t_ms >= *start && end.is_none_or(|end| t_ms < end))
            .map(|(start, _)| *start)
    }

    pub fn moving_at(&self, t_ms: f64) -> bool {
        self.walk_at(t_ms).is_some()
    }

    pub fn add_flash(&mut self, export: String, at_ms: f64) {
        self.flashes.push((export, at_ms));
        if self.flashes.len() > 6 {
            self.flashes.remove(0);
        }
    }

    /// Most recent damage filter started at or before `t_ms`.
    pub fn flash_at(&self, t_ms: f64) -> Option<(&str, f64)> {
        self.flashes
            .iter()
            .rev()
            .find(|(_, at)| *at <= t_ms)
            .map(|(export, at)| (export.as_str(), t_ms - at))
    }

    /// Ticks since the last displacement (large when it never moved).
    pub fn still_ticks(&self, tick: u32) -> u32 {
        self.last_move_tick
            .map_or(u32::MAX, |m| tick.saturating_sub(m))
    }

    pub fn register_impact(&mut self, at_ms: f64, hit_speed_ms: f64) {
        if self.impacts.last().is_some_and(|last| at_ms - last < 100.) {
            return;
        }
        self.impacts.push(at_ms);
        if self.impacts.len() > 8 {
            self.impacts.remove(0);
        }
        if self.engaged_since_ms.is_some() && hit_speed_ms > 0. {
            self.predicted_ms = Some(at_ms + hit_speed_ms);
        }
    }

    pub fn set_engaged(&mut self, engaged: bool, now_ms: f64, hit_speed_ms: f64, load_ms: f64) {
        if !engaged {
            self.engaged_since_ms = None;
            self.predicted_ms = None;
            return;
        }
        if self.engaged_since_ms.is_none() {
            self.engaged_since_ms = Some(now_ms);
            let recent = self
                .impacts
                .last()
                .is_some_and(|last| now_ms - last < hit_speed_ms);
            self.predicted_ms = Some(if recent {
                self.impacts.last().copied().unwrap_or(now_ms) + hit_speed_ms
            } else {
                now_ms + load_ms
            });
        }
        // Keep cycling while engaged; observations re-synchronize the phase.
        if let Some(predicted) = self.predicted_ms {
            if hit_speed_ms > 0. && now_ms > predicted + hit_speed_ms * 0.6 {
                let behind = ((now_ms - predicted) / hit_speed_ms).floor().max(1.);
                self.predicted_ms = Some(predicted + behind * hit_speed_ms);
            }
        }
    }

    /// Turn the displayed turret toward `target` (degrees clockwise from
    /// screen-up) at `deg_per_s`, holding its aim without a target.
    pub fn aim_turret(&mut self, target: Option<f64>, t_ms: f64, deg_per_s: f64) -> f64 {
        let rest = if self.side == 0 { 0. } else { 180. };
        let (yaw, at) = self.turret.unwrap_or((target.unwrap_or(rest), t_ms));
        let dt = (t_ms - at).clamp(0., 250.) / 1000.;
        let yaw = match target {
            Some(goal) => {
                let diff = (goal - yaw + 540.).rem_euclid(360.) - 180.;
                yaw + diff.clamp(-deg_per_s * dt, deg_per_s * dt)
            }
            None => yaw,
        };
        let yaw = (yaw + 540.).rem_euclid(360.) - 180.;
        self.turret = Some((yaw, t_ms.max(at)));
        yaw
    }

    /// Milliseconds into the authored attack clip at display time `t_ms`.
    /// `action_ms` is the authored `action_frame` time and `clip_ms` its
    /// length; `speed` compresses the clip when the cadence is shorter.
    pub fn attack_clip_ms(
        &self,
        t_ms: f64,
        action_ms: f64,
        clip_ms: f64,
        speed: f64,
    ) -> Option<f64> {
        let pre = action_ms / speed;
        let post = (clip_ms - action_ms).max(TICK_MS) / speed;
        let mut best: Option<f64> = None;
        for &impact in self.impacts.iter().chain(self.predicted_ms.iter()) {
            if t_ms >= impact - pre && t_ms < impact + post && best.is_none_or(|b| impact > b) {
                best = Some(impact);
            }
        }
        best.map(|impact| ((t_ms - (impact - pre)) * speed).clamp(0., clip_ms))
    }
}

/// Nine-way heading with angular hysteresis so a unit walking diagonally does
/// not flicker between neighbouring authored views.
pub fn stable_heading(
    dx: f64,
    dy: f64,
    side: i32,
    last: Option<PresentationHeading>,
) -> PresentationHeading {
    if dx.abs() + dy.abs() < 1e-6 {
        return last
            .map(|heading| PresentationHeading {
                source: HeadingSource::RetainedPresentationHeading,
                ..heading
            })
            .unwrap_or(PresentationHeading {
                direction_index: if side == 0 { 1 } else { 9 },
                flip_horizontal: false,
                source: HeadingSource::SidePresentationDefault,
            });
    }
    let angle = dx.atan2(dy).to_degrees();
    if let Some(last) = last {
        let center = (last.direction_index as f64 - 1.) * 22.5;
        let center = if last.flip_horizontal {
            -center
        } else {
            center
        };
        let mut diff = (angle - center).abs() % 360.;
        if diff > 180. {
            diff = 360. - diff;
        }
        if diff < 15. {
            return PresentationHeading {
                source: HeadingSource::MotionDeltaPresentation,
                ..last
            };
        }
    }
    let quantized = (angle.abs().min(180.) / 22.5).round() as u8;
    PresentationHeading {
        direction_index: (quantized + 1).clamp(1, 9),
        flip_horizontal: angle < 0.,
        source: HeadingSource::MotionDeltaPresentation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit(x: i32, y: i32) -> Entity {
        Entity {
            address: 1,
            row: "IceGolemite".into(),
            x,
            y,
            hp: 10,
            max_hp: 10,
            ..Entity::default()
        }
    }

    #[test]
    fn single_tick_pauses_do_not_stop_the_walk_cycle() {
        let mut actor = Actor::new(1, &unit(0, 0), 10, true);
        for tick in 11u32..=30 {
            // Every eleventh tick holds position, as observed natively.
            let step = if tick == 21 { tick - 11 } else { tick - 10 } as i32;
            let y = step * 50;
            actor.observe(&unit(0, y), tick);
            assert!(actor.moving, "tick {tick}");
        }
        assert_eq!(actor.walk_at(20. * TICK_MS), Some(10. * TICK_MS));
        for tick in 31..=36 {
            actor.observe(&unit(0, 1000), tick);
        }
        assert!(!actor.moving);
        assert!(actor.moving_at(29. * TICK_MS));
        assert!(!actor.moving_at(31. * TICK_MS));
    }

    #[test]
    fn attack_window_plays_windup_before_the_next_impact() {
        let mut actor = Actor::new(1, &unit(0, 0), 0, false);
        actor.set_engaged(true, 1000., 1000., 300.);
        assert_eq!(actor.predicted_ms, Some(1300.));
        actor.register_impact(1310., 1000.);
        assert_eq!(actor.predicted_ms, Some(2310.));
        // 467 ms wind-up to the action frame in a 1000 ms clip.
        let windup = actor.attack_clip_ms(2000., 467., 1000., 1.).unwrap();
        // A later engagement does not hide earlier, already displayed swings.
        assert!((windup - 157.).abs() < 1.);
        let recovery = actor.attack_clip_ms(1400., 467., 1000., 1.).unwrap();
        assert!((recovery - 557.).abs() < 1.);
        actor.set_engaged(false, 2100., 1000., 300.);
        assert!(actor.attack_clip_ms(2200., 467., 1000., 1.).is_none());
        // Observed impacts ahead of the display clock still show their wind-up.
        actor.register_impact(3000., 1000.);
        let ahead = actor.attack_clip_ms(2600., 467., 1000., 1.).unwrap();
        assert!((ahead - 67.).abs() < 1.);
    }

    #[test]
    fn heading_hysteresis_holds_neighbouring_directions() {
        let east = stable_heading(10., 0., 0, None);
        assert_eq!(east.direction_index, 5);
        let slightly_north = stable_heading(10., 2.5, 0, Some(east));
        assert_eq!(slightly_north.direction_index, 5);
        let north_east = stable_heading(10., 10., 0, Some(east));
        assert_eq!(north_east.direction_index, 3);
    }
}
