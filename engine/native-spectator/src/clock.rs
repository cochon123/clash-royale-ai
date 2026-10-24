//! Display clock for interpolated playback of 20 Hz native snapshots.
//!
//! The clock runs in fractional native ticks, advancing with wall time at the
//! playback speed and trailing the newest snapshot by a fixed lag, so the
//! renderer always has the next snapshots to interpolate toward and already
//! knows upcoming impacts. Arrival jitter is absorbed by a soft correction
//! instead of visible jumps.
#[cfg(not(feature = "wasm"))]
use std::time::Instant;

pub const TICKS_PER_SECOND: f64 = 20.;
/// Default distance behind the newest snapshot, in ticks. The spectator is
/// not interactive, so 0.8 s of delay lets the renderer know every observed
/// impact before its authored attack wind-up has to start.
pub const DEFAULT_LAG_TICKS: f64 = 16.;

/// What the producer is doing, as far as the display is concerned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Flow {
    /// Snapshots keep arriving at the playback speed.
    Running,
    /// Paused: hold, but still follow single steps beyond the lag window.
    Paused,
    /// The match ended: play out the buffered snapshots to the final one.
    Finished,
}

pub struct PlaybackClock {
    display: Option<f64>,
    #[cfg(not(feature = "wasm"))]
    last: Instant,
    lag: f64,
}

impl Default for PlaybackClock {
    fn default() -> Self {
        Self::new(DEFAULT_LAG_TICKS)
    }
}

impl PlaybackClock {
    pub fn new(lag_ticks: f64) -> Self {
        Self {
            display: None,
            #[cfg(not(feature = "wasm"))]
            last: Instant::now(),
            lag: lag_ticks.max(0.),
        }
    }

    pub fn reset(&mut self) {
        self.display = None;
    }

    /// Advance to now and return the fractional tick to draw.
    /// Wasm has no wall clock; callers pass `dt` to [`PlaybackClock::step`].
    #[cfg(not(feature = "wasm"))]
    pub fn advance(&mut self, oldest: u32, latest: u32, speed: f64, flow: Flow) -> f64 {
        let now = Instant::now();
        let dt = now.duration_since(self.last).as_secs_f64().min(0.25);
        self.last = now;
        self.step(oldest, latest, speed, flow, dt)
    }

    #[cfg(feature = "wasm")]
    pub fn advance(&mut self, oldest: u32, latest: u32, speed: f64, flow: Flow) -> f64 {
        self.step(oldest, latest, speed, flow, 0.)
    }

    pub fn step(&mut self, oldest: u32, latest: u32, speed: f64, flow: Flow, dt: f64) -> f64 {
        let (oldest, latest) = (oldest as f64, latest as f64);
        let target = (latest - self.lag).max(oldest);
        let mut display = match self.display {
            Some(d) if d >= oldest - 0.5 && d <= latest + 0.5 && target - d < 8. => d,
            // A new or restarted match starts at its first retained snapshot.
            _ if latest - oldest <= self.lag => oldest,
            _ => target,
        };
        let advance = dt * TICKS_PER_SECOND * speed.max(0.01);
        match flow {
            Flow::Running => {
                display += advance;
                // Ease toward the lagged target to absorb arrival jitter.
                let error = target - display;
                display += error * (1. - (-dt * 4.).exp());
            }
            Flow::Paused if display < target => display = (display + advance).min(target),
            Flow::Paused => {}
            Flow::Finished => display += advance,
        }
        let display = display.clamp(oldest, latest);
        self.display = Some(display);
        display
    }
}

/// Browser scene lag. One native tick is 50 ms. The scene presenter stays
/// this far behind the newest snapshot so the interval it draws has already
/// been observed. It does not predict combat past that snapshot.
pub const SCENE_LAG_TICKS: f64 = 1.;

/// Fractional display clock for the browser scene presenter.
///
/// Wall time advances the clock at `TICKS_PER_SECOND * speed`, capped one
/// engine tick behind the newest observed snapshot while the match is running.
/// Pause holds, unless a single step moved the lag target forward. The ended
/// match plays out to the final snapshot. A reset clears `display` so the next
/// step starts at the new oldest snapshot.
pub struct SceneDisplayClock {
    display: Option<f64>,
    lag_ticks: f64,
}

impl Default for SceneDisplayClock {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneDisplayClock {
    pub fn new() -> Self {
        Self {
            display: None,
            lag_ticks: SCENE_LAG_TICKS,
        }
    }

    /// Display lag behind the newest observed tick. The native presenter keeps
    /// one tick. The wasm painter defaults to 3 ticks (150 ms at 20 Hz).
    pub fn set_lag_ticks(&mut self, lag_ticks: f64) {
        if lag_ticks.is_finite() {
            self.lag_ticks = lag_ticks.max(0.);
        }
    }

    pub fn lag_ticks(&self) -> f64 {
        self.lag_ticks
    }

    pub fn reset(&mut self) {
        self.display = None;
    }

    pub fn display(&self) -> Option<f64> {
        self.display
    }

    /// `dt` is elapsed wall seconds. The result is a fractional native tick
    /// inside `[oldest, latest]`.
    pub fn step(&mut self, oldest: u32, latest: u32, speed: f64, flow: Flow, dt: f64) -> f64 {
        let oldest_f = oldest as f64;
        let latest_f = latest as f64;
        let target = scene_target(oldest, latest, flow, self.lag_ticks);
        let mut display = match self.display {
            Some(d) if (oldest_f - 0.5..=latest_f + 0.5).contains(&d) => d,
            _ => oldest_f,
        };
        let advance = dt.max(0.).min(0.25) * TICKS_PER_SECOND * speed.clamp(0., 16.);
        match flow {
            Flow::Running | Flow::Finished => display = (display + advance).min(target),
            Flow::Paused if display < target => display = (display + advance).min(target),
            Flow::Paused => {}
        }
        let display = display.clamp(oldest_f, latest_f);
        self.display = Some(display);
        display
    }

    /// True when another wall step cannot move the clock without a new snapshot
    /// or a flow change. Callers use this to stop emitting duplicate poses.
    pub fn caught_up(&self, oldest: u32, latest: u32, flow: Flow) -> bool {
        self.display
            .is_some_and(|display| display + 1e-6 >= scene_target(oldest, latest, flow, self.lag_ticks))
    }
}

fn scene_target(oldest: u32, latest: u32, flow: Flow, lag_ticks: f64) -> f64 {
    let oldest_f = oldest as f64;
    let latest_f = latest as f64;
    match flow {
        Flow::Finished => latest_f,
        Flow::Running | Flow::Paused => (latest_f - lag_ticks).max(oldest_f),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trails_newest_snapshot_and_advances_smoothly() {
        let mut clock = PlaybackClock::new(1.);
        let mut latest = 100u32;
        let mut shown = Vec::new();
        // 60 Hz display, snapshots every 50 ms with some arrival jitter.
        for frame in 0..240 {
            let t = frame as f64 / 60.;
            let arrived = 100 + ((t * 20.) + if frame % 7 == 0 { 0.4 } else { 0. }) as u32;
            latest = latest.max(arrived);
            shown.push(clock.step(
                latest.saturating_sub(6),
                latest,
                1.,
                Flow::Running,
                1. / 60.,
            ));
        }
        let steps: Vec<f64> = shown.windows(2).map(|w| w[1] - w[0]).collect();
        assert!(
            steps[30..].iter().all(|s| *s > 0.15 && *s < 0.55),
            "{steps:?}"
        );
        let lag = latest as f64 - shown.last().unwrap();
        assert!(lag > 0.3 && lag < 2.0, "lag {lag}");
    }

    #[test]
    fn pause_holds_steps_advance_and_finish_plays_out() {
        let mut clock = PlaybackClock::new(16.);
        let start = clock.step(60, 100, 1., Flow::Running, 1. / 60.);
        assert!((start - 84.).abs() < 0.5);
        for _ in 0..60 {
            assert!((clock.step(60, 100, 1., Flow::Paused, 1. / 60.) - start).abs() < 1e-9);
        }
        // A single step beyond the lag window is shown.
        let mut shown = start;
        for _ in 0..30 {
            shown = clock.step(60, 101, 1., Flow::Paused, 1. / 60.);
        }
        assert!((shown - 85.).abs() < 0.5, "{shown}");
        for _ in 0..120 {
            shown = clock.step(60, 101, 1., Flow::Finished, 1. / 60.);
        }
        assert_eq!(shown, 101.);
    }

    #[test]
    fn scene_clock_stays_one_tick_behind_and_samples_between_engine_ticks() {
        let mut clock = SceneDisplayClock::new();
        assert_eq!(clock.step(100, 100, 1., Flow::Running, 0.), 100.);
        assert!(clock.caught_up(100, 100, Flow::Running));
        // The next snapshot is known, but the drawable interval is still tick 100.
        assert_eq!(clock.step(100, 101, 1., Flow::Running, 1. / 30.), 100.);
        let mut shown = Vec::new();
        for _ in 0..4 {
            shown.push(clock.step(100, 102, 1., Flow::Running, 1. / 30.));
        }
        let step = 20. / 30.;
        assert!((shown[0] - (100. + step)).abs() < 1e-9, "{shown:?}");
        assert!((shown[1] - 101.).abs() < 1e-9, "{shown:?}");
        assert!(shown.iter().all(|t| (100.0..=101.0).contains(t)));
        assert!(clock.caught_up(100, 102, Flow::Running));
        // Speed scales wall advance, still without leaving the lag target.
        clock.reset();
        clock.step(0, 10, 2., Flow::Running, 0.);
        let fast = clock.step(0, 10, 2., Flow::Running, 0.05);
        assert!((fast - 2.).abs() < 1e-9, "{fast}");
        assert!(fast <= 9.);
    }

    #[test]
    fn scene_clock_pause_holds_end_plays_out_and_reset_restarts() {
        let mut clock = SceneDisplayClock::new();
        let held = clock.step(50, 52, 1., Flow::Running, 0.);
        assert_eq!(held, 50.);
        let held = clock.step(50, 52, 1., Flow::Paused, 1.);
        assert_eq!(held, 51.);
        let held = clock.step(50, 52, 4., Flow::Paused, 1.);
        assert_eq!(held, 51.);
        let played = clock.step(50, 52, 1., Flow::Finished, 1. / 20.);
        assert!((played - 52.).abs() < 1e-9, "{played}");
        clock.reset();
        assert_eq!(clock.display(), None);
        assert_eq!(clock.step(0, 0, 1., Flow::Running, 1.), 0.);
    }

    #[test]
    fn scene_lag_override_trails_by_the_injected_distance() {
        let mut clock = SceneDisplayClock::new();
        assert!((clock.lag_ticks() - 1.).abs() < 1e-12);
        clock.set_lag_ticks(3.);
        // Each step accepts at most 0.25s, which is 5 native ticks. The lag
        // target is latest - 3 = 7, so the second quarter-second reaches it.
        assert_eq!(clock.step(0, 10, 1., Flow::Running, 0.), 0.);
        let mid = clock.step(0, 10, 1., Flow::Running, 0.25);
        assert!((mid - 5.).abs() < 1e-9, "{mid}");
        let shown = clock.step(0, 10, 1., Flow::Running, 0.25);
        assert!((shown - 7.).abs() < 1e-9, "{shown}");
        assert!(clock.caught_up(0, 10, Flow::Running));
    }
}
