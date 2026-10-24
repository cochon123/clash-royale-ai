//! Original-asset presentation without full-frame pixels. One Snapshot JSON
//! snapshots are all observed; display lists are paced in wall-clock time.
//! Immutable PNG poses are cached once and cold poses expire by bounded LRU.
use clash_native_spectator::{
    clock::{Flow, SceneDisplayClock},
    draw::{parse_card_list, Painter},
    frame::BattleFrame,
    scene::{CaptureGuard, TextureCache},
};
use serde::Deserialize;
use serde_json::json;
use std::{
    cell::RefCell,
    io::{self, BufRead, Write},
    path::PathBuf,
    rc::Rc,
    sync::mpsc::{self, Receiver, RecvTimeoutError},
    thread,
    time::{Duration, Instant},
};

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

trait SceneSink {
    fn observe(&mut self, snapshot: &Snapshot);
    fn emit(&mut self, snapshot: &Snapshot, display_tick: f64) -> Result<(), String>;
}

struct Presenter<W: Write> {
    painter: Painter,
    cache: Rc<RefCell<TextureCache>>,
    stdout: W,
    sequence: u64,
}
impl<W: Write> SceneSink for Presenter<W> {
    fn observe(&mut self, snapshot: &Snapshot) {
        self.painter.observe(&snapshot.frame);
    }
    fn emit(&mut self, snapshot: &Snapshot, display_tick: f64) -> Result<(), String> {
        let frame = &snapshot.frame;
        let started = Instant::now();
        let oldest = self.painter.oldest_tick().unwrap_or(frame.tick) as f64;
        let latest = self.painter.latest_tick().unwrap_or(frame.tick) as f64;
        let display_tick = display_tick.clamp(oldest, latest);
        let capture = CaptureGuard::begin(self.cache.clone())?;
        self.painter.draw(display_tick)?;
        let scene = capture.finish()?;
        self.sequence += 1;
        let layout = self
            .painter
            .snapshot_at(display_tick)
            .as_ref()
            .map(|shown| self.painter.controls_layout(shown))
            .unwrap_or_else(|| self.painter.controls_layout(frame));
        let packet = json!({
            "v":1, "sequence":self.sequence, "tick":frame.tick, "display_tick":display_tick,
            "match_id":frame.match_id, "width":540, "height":900,
            "clear":[19,27,31,255], "layout":{"field":layout.field,"world":layout.world,"cards":layout.cards},
            "textures":scene.textures, "draws":scene.draws, "status":status(snapshot),
            "playback_speed":snapshot.playback_speed, "render_ms":started.elapsed().as_secs_f64()*1000.,
            "texture_count":self.cache.borrow().len(), "texture_bytes":self.cache.borrow().encoded_bytes,
            "evicted":scene.evicted,
            "unresolved_rows":self.painter.unresolved_rows,
        });
        writeln!(self.stdout, "{packet}")
            .and_then(|_| self.stdout.flush())
            .map_err(|e| e.to_string())
    }
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

struct Horizon {
    match_id: u64,
    oldest: u32,
    latest: u32,
}

fn flow_of(snapshot: &Snapshot) -> Flow {
    match status(snapshot) {
        "paused" => Flow::Paused,
        "ended" | "closed" => Flow::Finished,
        _ => Flow::Running,
    }
}

fn note_horizon(clock: &mut SceneDisplayClock, horizon: &mut Option<Horizon>, snapshot: &Snapshot) {
    let tick = snapshot.frame.tick;
    let match_id = snapshot.frame.match_id;
    let reset = horizon
        .as_ref()
        .is_none_or(|seen| seen.match_id != match_id || tick < seen.latest);
    if reset {
        clock.reset();
        *horizon = Some(Horizon {
            match_id,
            oldest: tick,
            latest: tick,
        });
    } else if let Some(seen) = horizon.as_mut() {
        if tick > seen.latest {
            seen.latest = tick;
        }
    }
}

/// Observe every snapshot. Drawing stays one engine tick behind the newest
/// snapshot and is capped at `fps`.
///
/// A running match emits one pose per snapshot, not a second in-between
/// sample. The browser interpolates those poses on its own display clock.
/// Filling the gap on the server builds another authored texture almost every
/// frame and made live matches heavier than the measured browser sample.
fn drive(
    sink: &mut impl SceneSink,
    rx: Receiver<Result<Snapshot, String>>,
    fps: u32,
) -> Result<(), String> {
    let interval = Duration::from_secs_f64(1. / fps as f64);
    let mut due = Instant::now();
    let mut latest: Option<Snapshot> = None;
    let mut dirty = false;
    let mut clock = SceneDisplayClock::new();
    let mut horizon = None;
    let mut last_step = Instant::now();
    let publish = |sink: &mut dyn SceneSink,
                   clock: &mut SceneDisplayClock,
                   horizon: &Horizon,
                   snapshot: &Snapshot,
                   last_step: &mut Instant|
     -> Result<bool, String> {
        let now = Instant::now();
        let dt = now.duration_since(*last_step).as_secs_f64();
        *last_step = now;
        let flow = flow_of(snapshot);
        let display = clock.step(horizon.oldest, horizon.latest, snapshot.playback_speed, flow, dt);
        sink.emit(snapshot, display)?;
        // Caught-up is reported for the end-of-match drain. A running match
        // does not keep painting until the clock reaches the lag target.
        Ok(clock.caught_up(horizon.oldest, horizon.latest, flow))
    };
    loop {
        let timeout = if dirty {
            due.saturating_duration_since(Instant::now())
        } else {
            Duration::from_secs(1)
        };
        match rx.recv_timeout(timeout) {
            Ok(value) => {
                // Bound each drain batch so an unusually fast producer cannot
                // starve output, while never skipping an observation.
                let mut values = vec![value];
                for _ in 1..256 {
                    match rx.try_recv() {
                        Ok(value) => values.push(value),
                        Err(_) => break,
                    }
                }
                for value in values {
                    let snapshot = value?;
                    let forced = latest.as_ref().is_none_or(|old| {
                        old.frame.match_id != snapshot.frame.match_id
                            || snapshot.frame.tick < old.frame.tick
                            || status(old) != status(&snapshot)
                                && matches!(status(&snapshot), "paused" | "ended" | "closed")
                    });
                    sink.observe(&snapshot);
                    note_horizon(&mut clock, &mut horizon, &snapshot);
                    latest = Some(snapshot);
                    dirty = true;
                    if forced {
                        let started = Instant::now();
                        publish(
                            sink,
                            &mut clock,
                            horizon.as_ref().unwrap(),
                            latest.as_ref().unwrap(),
                            &mut last_step,
                        )?;
                        dirty = false;
                        due = started + interval;
                    }
                }
                if dirty && Instant::now() >= due {
                    let started = Instant::now();
                    publish(
                        sink,
                        &mut clock,
                        horizon.as_ref().unwrap(),
                        latest.as_ref().unwrap(),
                        &mut last_step,
                    )?;
                    dirty = false;
                    due = started + interval;
                }
            }
            Err(RecvTimeoutError::Timeout) => {
                if dirty {
                    let started = Instant::now();
                    publish(
                        sink,
                        &mut clock,
                        horizon.as_ref().unwrap(),
                        latest.as_ref().unwrap(),
                        &mut last_step,
                    )?;
                    dirty = false;
                    due = started + interval;
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                if dirty {
                    publish(
                        sink,
                        &mut clock,
                        horizon.as_ref().unwrap(),
                        latest.as_ref().unwrap(),
                        &mut last_step,
                    )?;
                }
                if let Some(snapshot) = latest.as_ref() {
                    if flow_of(snapshot) == Flow::Finished {
                        for _ in 0..64 {
                            if clock.caught_up(
                                horizon.as_ref().unwrap().oldest,
                                horizon.as_ref().unwrap().latest,
                                Flow::Finished,
                            ) {
                                break;
                            }
                            let display = clock.step(
                                horizon.as_ref().unwrap().oldest,
                                horizon.as_ref().unwrap().latest,
                                snapshot.playback_speed,
                                Flow::Finished,
                                1. / fps as f64,
                            );
                            sink.emit(snapshot, display)?;
                        }
                    }
                }
                return Ok(());
            }
        }
    }
}

fn run() -> Result<(), String> {
    let mut assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
    let mut texture_root = None;
    let mut pose_root = None;
    let mut fps = 30;
    let mut cards = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--assets" => assets = args.next().ok_or("missing asset directory")?.into(),
            "--cards" => cards.extend(parse_card_list(&args.next().ok_or("missing card names")?)),
            "--texture-cache" => {
                texture_root = Some(PathBuf::from(
                    args.next().ok_or("missing texture cache directory")?,
                ))
            }
            "--pose-cache" => {
                pose_root = Some(PathBuf::from(
                    args.next().ok_or("missing pose cache directory")?,
                ))
            }
            "--fps" => {
                fps = args
                    .next()
                    .ok_or("missing fps")?
                    .parse::<u32>()
                    .map_err(|_| "invalid fps")?
            }
            "--help" | "-h" => {
                println!("clash-web-scene [--assets DIR] --texture-cache DIR [--pose-cache DIR] [--cards NAMES] [--fps 1..120]; reads every Snapshot JSONL, writes paced original-sprite display-list JSONL. NAMES is one comma-separated argument; spaces stay inside a native card name. Omitted --cards loads the whole bundle.");
                return Ok(());
            }
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    if !(1..=120).contains(&fps) {
        return Err("fps must be in 1..120".into());
    }
    let cache = Rc::new(RefCell::new(TextureCache::new(
        &texture_root.ok_or("--texture-cache DIR is required")?,
    )?));
    // Reading starts before asset loading, so startup backpressure is bounded
    // without discarding the first transient engine events.
    let (tx, rx) = mpsc::sync_channel(256);
    thread::spawn(move || {
        for line in io::stdin().lock().lines() {
            let value = match line {
                Ok(line) if line.trim().is_empty() => continue,
                Ok(line) => serde_json::from_str::<Snapshot>(&line)
                    .map_err(|e| format!("invalid snapshot: {e}")),
                Err(e) => Err(format!("read snapshot: {e}")),
            };
            if tx.send(value).is_err() {
                break;
            }
        }
    });
    let mut painter = Painter::load_for_cards(&assets, 540, 900, &cards)?;
    if let Some(dir) = pose_root {
        painter.enable_pose_cache(&dir)?;
    }
    let mut presenter = Presenter {
        painter,
        cache,
        stdout: io::BufWriter::new(io::stdout().lock()),
        sequence: 0,
    };
    drive(&mut presenter, rx, fps)
}

fn main() {
    if let Err(error) = run() {
        eprintln!("clash-web-scene: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clash_native_spectator::frame::Entity;
    use std::sync::{Arc, Mutex};

    struct Recorder {
        observed: Arc<Mutex<Vec<(u32, usize)>>>,
        output: mpsc::Sender<(u64, u32, String)>,
    }
    impl SceneSink for Recorder {
        fn observe(&mut self, s: &Snapshot) {
            self.observed
                .lock()
                .unwrap()
                .push((s.frame.tick, s.frame.entities.len()));
        }
        fn emit(&mut self, s: &Snapshot, _display_tick: f64) -> Result<(), String> {
            self.output
                .send((s.frame.match_id, s.frame.tick, status(s).into()))
                .map_err(|e| e.to_string())
        }
    }
    fn snapshot(tick: u32, match_id: u64, runtime_status: &str, transient: bool) -> Snapshot {
        Snapshot {
            frame: BattleFrame {
                tick,
                elapsed_ms: u64::from(tick) * 50,
                terminated: runtime_status == "ended",
                winner_side: None,
                arena_size: [18000, 32000],
                elixir: [5., 5.],
                entities: if transient {
                    vec![Entity {
                        address: 1,
                        row: "Skeleton".into(),
                        ..Entity::default()
                    }]
                } else {
                    Vec::new()
                },
                hands: Default::default(),
                controller_names: Default::default(),
                plays: Vec::new(),
                match_id,
                human: None,
            },
            runtime_status: runtime_status.into(),
            playback_speed: 4.,
        }
    }

    #[test]
    fn observes_transient_between_draws_and_flushes_latest_while_pipe_idle() {
        let (tx, rx) = mpsc::sync_channel(256);
        let (output, packets) = mpsc::channel();
        let observed = Arc::new(Mutex::new(Vec::new()));
        let mut sink = Recorder {
            observed: observed.clone(),
            output,
        };
        let worker = thread::spawn(move || drive(&mut sink, rx, 5));
        tx.send(Ok(snapshot(100, 1, "running", false))).unwrap();
        assert_eq!(packets.recv_timeout(Duration::from_secs(1)).unwrap().1, 100);
        tx.send(Ok(snapshot(101, 1, "running", true))).unwrap();
        tx.send(Ok(snapshot(102, 1, "running", false))).unwrap();
        // Keep tx alive: no following stdin line is needed to publish tick102.
        assert_eq!(packets.recv_timeout(Duration::from_secs(1)).unwrap().1, 102);
        assert_eq!(
            *observed.lock().unwrap(),
            vec![(100, 0), (101, 1), (102, 0)]
        );
        drop(tx);
        worker.join().unwrap().unwrap();
    }

    #[test]
    fn pause_end_reset_are_emitted_even_inside_the_draw_interval() {
        let (tx, rx) = mpsc::sync_channel(256);
        let (output, packets) = mpsc::channel();
        let mut sink = Recorder {
            observed: Arc::new(Mutex::new(Vec::new())),
            output,
        };
        let worker = thread::spawn(move || drive(&mut sink, rx, 1));
        for item in [
            snapshot(100, 1, "running", false),
            snapshot(101, 1, "paused", false),
            snapshot(102, 1, "ended", false),
            snapshot(0, 2, "running", false),
        ] {
            tx.send(Ok(item)).unwrap();
        }
        for expected in [
            (1, 100, "running"),
            (1, 101, "paused"),
            (1, 102, "ended"),
            (2, 0, "running"),
        ] {
            let actual = packets.recv_timeout(Duration::from_millis(500)).unwrap();
            assert_eq!(actual, (expected.0, expected.1, expected.2.into()));
        }
        drop(tx);
        worker.join().unwrap().unwrap();
    }

    #[test]
    fn eof_flushes_pending_pose_and_surfaces_bad_input() {
        let (tx, rx) = mpsc::sync_channel(256);
        let (output, packets) = mpsc::channel();
        let mut sink = Recorder {
            observed: Arc::new(Mutex::new(Vec::new())),
            output,
        };
        tx.send(Ok(snapshot(100, 1, "running", false))).unwrap();
        tx.send(Ok(snapshot(101, 1, "running", false))).unwrap();
        drop(tx);
        drive(&mut sink, rx, 1).unwrap();
        assert_eq!(
            packets.try_iter().map(|p| p.1).collect::<Vec<_>>(),
            vec![100, 101]
        );
        let (tx, rx) = mpsc::sync_channel(1);
        tx.send(Err("invalid snapshot".into())).unwrap();
        drop(tx);
        assert_eq!(drive(&mut sink, rx, 30).unwrap_err(), "invalid snapshot");
    }

    struct LagSink {
        output: mpsc::Sender<(u32, f64)>,
    }
    impl SceneSink for LagSink {
        fn observe(&mut self, _s: &Snapshot) {}
        fn emit(&mut self, s: &Snapshot, display_tick: f64) -> Result<(), String> {
            self.output
                .send((s.frame.tick, display_tick))
                .map_err(|e| e.to_string())
        }
    }

    #[test]
    fn running_display_stays_one_tick_behind_the_latest_snapshot() {
        let (tx, rx) = mpsc::sync_channel(256);
        let (output, packets) = mpsc::channel();
        let mut sink = LagSink { output };
        let worker = thread::spawn(move || drive(&mut sink, rx, 30));
        let mut first = snapshot(200, 1, "running", false);
        first.playback_speed = 1.;
        tx.send(Ok(first)).unwrap();
        let (tick, display) = packets.recv_timeout(Duration::from_secs(1)).unwrap();
        assert_eq!(tick, 200);
        assert!((display - 200.).abs() < 1e-6, "{display}");
        for tick in [201u32, 202] {
            let mut frame = snapshot(tick, 1, "running", false);
            frame.playback_speed = 1.;
            tx.send(Ok(frame)).unwrap();
        }
        let mut seen = Vec::new();
        let deadline = Instant::now() + Duration::from_millis(400);
        while Instant::now() < deadline {
            match packets.recv_timeout(Duration::from_millis(50)) {
                Ok(packet) => seen.push(packet),
                Err(_) => continue,
            }
        }
        assert!(seen.iter().any(|(tick, _)| *tick == 202), "{seen:?}");
        assert!(
            seen.iter().all(|(_, display)| *display <= 201. + 1e-6),
            "{seen:?}"
        );
        drop(tx);
        worker.join().unwrap().unwrap();
    }
}
