//! Display clock for interpolated playback of 20 Hz native snapshots.
//!
//! The clock runs in fractional native ticks, advancing with wall time at the
//! playback speed and trailing the newest snapshot by a fixed lag, so the
//! renderer always has the next snapshots to interpolate toward and already
//! knows upcoming impacts. Arrival jitter is absorbed by a soft correction
//! instead of visible jumps.
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
            last: Instant::now(),
            lag: lag_ticks.max(0.),
        }
    }

    pub fn reset(&mut self) {
        self.display = None;
    }

    /// Advance to now and return the fractional tick to draw.
    pub fn advance(&mut self, oldest: u32, latest: u32, speed: f64, flow: Flow) -> f64 {
        let now = Instant::now();
        let dt = now.duration_since(self.last).as_secs_f64().min(0.25);
        self.last = now;
        self.step(oldest, latest, speed, flow, dt)
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
}
