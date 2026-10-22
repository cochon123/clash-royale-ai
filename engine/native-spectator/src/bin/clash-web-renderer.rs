//! Browser transport for the very same Painter used by rl.banana_play.
//! Stdin: native BattleFrame JSON lines. Stdout: JSON header + length-delimited
//! JPEG bytes (encoded here, so hosts only pass compressed frames through).
//! No window, engine, game logic, or alternative artwork lives here.
use clash_native_spectator::{
    clock::{Flow, PlaybackClock, DEFAULT_LAG_TICKS},
    draw::Painter,
    frame::BattleFrame,
    input::PlayerInput,
};
use serde::Deserialize;
use serde_json::json;
use std::{
    io::{self, BufRead, Write},
    path::PathBuf,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

#[derive(Deserialize)]
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

fn run() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut assets = root.join("analysis/native-spectator");
    let mut fps = 30u32;
    let mut jpeg_quality = 88u32;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--assets" => assets = args.next().ok_or("missing asset directory")?.into(),
            "--fps" => {
                fps = args
                    .next()
                    .ok_or("missing fps")?
                    .parse()
                    .map_err(|_| "invalid fps")?
            }
            "--jpeg-quality" => {
                jpeg_quality = args
                    .next()
                    .ok_or("missing jpeg quality")?
                    .parse()
                    .map_err(|_| "invalid jpeg quality")?
            }
            "--help" | "-h" => {
                println!("clash-web-renderer [--assets DIR] [--fps 1..60] [--jpeg-quality 1..100]; reads BattleFrame JSONL, writes JSON headers followed by JPEG bytes");
                return Ok(());
            }
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    if !(1..=60).contains(&fps) {
        return Err("fps must be in 1..60".into());
    }
    if !(1..=100).contains(&jpeg_quality) {
        return Err("jpeg quality must be in 1..100".into());
    }
    let (width, height) = (540, 900);
    let mut painter = Painter::load(&assets, width, height)?;
    let mut compressor = turbojpeg::Compressor::new().map_err(|e| e.to_string())?;
    compressor.set_quality(jpeg_quality as i32).map_err(|e| e.to_string())?;
    compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2).map_err(|e| e.to_string())?;
    // Keep Rc-backed scene caches and all compositing on this thread. Reading
    // snapshots independently lets animation continue between engine ticks.
    let (tx, rx) = mpsc::sync_channel(120);
    thread::spawn(move || {
        for line in io::stdin().lock().lines() {
            let value = line.map_err(|e| e.to_string()).and_then(|line| {
                serde_json::from_str::<Snapshot>(&line).map_err(|e| e.to_string())
            });
            if tx.send(value).is_err() {
                break;
            }
        }
    });
    let mut current: Option<Snapshot> = None;
    let mut clock = PlaybackClock::new(1.);
    let mut player = PlayerInput::default();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    let mut previous_display = None;
    let mut sequence = 0u64;
    loop {
        let start = Instant::now();
        let mut changed = false;
        let mut eof = false;
        if current.is_none() {
            match rx.recv() {
                Ok(value) => {
                    let value = value?;
                    clock = PlaybackClock::new(if value.frame.human.is_some() {
                        1.
                    } else {
                        DEFAULT_LAG_TICKS
                    });
                    painter.observe(&value.frame);
                    current = Some(value);
                    changed = true;
                }
                Err(_) => return Ok(()),
            }
        }
        loop {
            match rx.try_recv() {
                Ok(value) => {
                    let value = value?;
                    if current
                        .as_ref()
                        .is_some_and(|old| old.frame.match_id != value.frame.match_id)
                    {
                        clock.reset();
                    }
                    painter.observe(&value.frame);
                    current = Some(value);
                    changed = true;
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    eof = true;
                    break;
                }
            }
        }
        let snapshot = current.as_ref().unwrap();
        let frame = &snapshot.frame;
        let flow = if frame.terminated {
            Flow::Finished
        } else if snapshot.runtime_status == "paused" {
            Flow::Paused
        } else {
            Flow::Running
        };
        let display = clock.advance(
            painter.oldest_tick().unwrap(),
            painter.latest_tick().unwrap(),
            snapshot.playback_speed,
            flow,
        );
        if changed || previous_display.is_none_or(|old: f64| (display - old).abs() > 0.001) {
            let draw_started = Instant::now();
            let mut pixels = painter.draw(display)?;
            player.synchronize(frame);
            player.cursor = frame
                .human
                .as_ref()
                .and_then(|h| h.target.as_ref())
                .map(|t| (t.x, t.y));
            painter.draw_player_controls(&mut pixels, frame, &player)?;
            sequence += 1;
            let layout = painter.controls_layout(frame);
            // Encode here: hosts forward compressed frames without any image
            // work of their own, and the pipe carries ~70 KB instead of 1.9 MB.
            let render_ms = draw_started.elapsed().as_secs_f64() * 1000.;
            let encode_started = Instant::now();
            // SIMD encoding reads RGBA directly, avoiding a second full-frame
            // conversion/allocation and keeping the Python host out of encoding.
            let jpeg = compressor.compress_to_owned(turbojpeg::Image {
                pixels: pixels.as_raw().as_slice(),
                width: width as usize,
                pitch: width as usize * 4,
                height: height as usize,
                format: turbojpeg::PixelFormat::RGBA,
            }).map_err(|e| e.to_string())?;
            let header = json!({"bytes":jpeg.len(),"format":"jpeg","quality":jpeg_quality,
                "render_ms":render_ms,"encode_ms":encode_started.elapsed().as_secs_f64()*1000.,
                "sequence":sequence,"tick":frame.tick,
                "display_tick":display,"match_id":frame.match_id,"width":width,"height":height,
                "layout":{"field":layout.field,"world":layout.world,"cards":layout.cards},
                "unresolved_rows":painter.unresolved_rows});
            writeln!(stdout, "{header}")
                .and_then(|_| stdout.write_all(&jpeg))
                .and_then(|_| stdout.flush())
                .map_err(|e| e.to_string())?;
            previous_display = Some(display);
        }
        if eof {
            return Ok(());
        }
        thread::sleep(Duration::from_secs_f64(1. / fps as f64).saturating_sub(start.elapsed()));
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("clash-web-renderer: {error}");
        std::process::exit(1);
    }
}
