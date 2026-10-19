use clash_native_spectator::{backend::Battle, draw::Painter, frame::BattleFrame};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

type Result<T> = std::result::Result<T, String>;
struct Args {
    checkpoint: PathBuf,
    assets: PathBuf,
    ticks: Option<u32>,
    until: bool,
    render: Option<PathBuf>,
    record: Option<PathBuf>,
    audit: Option<PathBuf>,
    width: u32,
    height: u32,
    window_frames: Option<u32>,
}
fn args() -> Result<Args> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut a = Args {
        checkpoint: root.join("analysis/native-pipeline/oracle-export-01/stage01-attempt001/state"),
        assets: root.join("analysis/native-spectator"),
        ticks: None,
        until: false,
        render: None,
        record: None,
        audit: None,
        width: 540,
        height: 900,
        window_frames: None,
    };
    let mut it = std::env::args().skip(1);
    while let Some(k) = it.next() {
        match k.as_str() {
            "--checkpoint" => a.checkpoint = it.next().ok_or("checkpoint path")?.into(),
            "--assets" => a.assets = it.next().ok_or("asset directory")?.into(),
            "--ticks" => {
                a.ticks = Some(
                    it.next()
                        .ok_or("tick count")?
                        .parse()
                        .map_err(|_| "invalid ticks")?,
                )
            }
            "--run-to-end" => a.until = true,
            "--render" => a.render = Some(it.next().ok_or("PNG path")?.into()),
            "--record" => a.record = Some(it.next().ok_or("JSONL path")?.into()),
            "--audit-out" => a.audit = Some(it.next().ok_or("new audit directory")?.into()),
            "--width" => a.width = it.next().ok_or("width")?.parse().map_err(|_| "width")?,
            "--height" => a.height = it.next().ok_or("height")?.parse().map_err(|_| "height")?,
            "--window-frames" => {
                a.window_frames = Some(
                    it.next()
                        .ok_or("frame count")?
                        .parse()
                        .map_err(|_| "frame count")?,
                )
            }
            "--help" | "-h" => {
                println!("Original-engine spectator (built-in Normal scenario bots).\nUsage: clash-native-spectator [--ticks N | --run-to-end] [--render PNG] [--record JSONL] [--audit-out NEWDIR] [--assets DIR] [--checkpoint DIR]\nWithout --ticks/--run-to-end: desktop viewer. Space pause, Right one tick, R restart, +/- speed, S screenshot, Esc close.\nNo human controls or Banana policy connection in this viewer.");
                std::process::exit(0);
            }
            _ => return Err(format!("unknown argument {k}")),
        }
    }
    if a.width < 240 || a.height < 320 || a.width > 2160 || a.height > 3840 {
        return Err("image dimensions out of bounds".into());
    }
    Ok(a)
}
fn record(out: &mut Option<BufWriter<File>>, frame: &BattleFrame) -> Result<()> {
    if let Some(f) = out {
        serde_json::to_writer(&mut *f, frame).map_err(|e| e.to_string())?;
        f.write_all(b"\n").map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn headless(a: Args) -> Result<()> {
    let start = Instant::now();
    let mut battle = Battle::load(&a.checkpoint)?;
    let mut out = a
        .record
        .as_ref()
        .map(File::create)
        .transpose()
        .map_err(|e| e.to_string())?
        .map(BufWriter::new);
    let mut frame = battle.snapshot()?;
    record(&mut out, &frame)?;
    let mut steps = 0;
    for _ in 0..if a.until { 7200 } else { a.ticks.unwrap_or(0) } {
        if frame.terminated {
            break;
        }
        battle.step()?;
        frame = battle.snapshot()?;
        record(&mut out, &frame)?;
        steps += 1;
    }
    if let Some(out) = out.as_mut() {
        out.flush().map_err(|e| e.to_string())?;
    }
    if let Some(path) = a.audit {
        battle.session.audit(&path)?;
    }
    let mut unresolved = Vec::new();
    if let Some(path) = a.render {
        let mut painter = Painter::load(&a.assets, a.width, a.height)?;
        painter
            .render(&frame)?
            .save(&path)
            .map_err(|e| e.to_string())?;
        unresolved.extend(painter.unresolved_rows);
    }
    println!(
        "{}",
        serde_json::json!({"backend":"original ARM64 statically recompiled to Rust","steps":steps,"wall_seconds":start.elapsed().as_secs_f64(),"frame":frame,"unresolved_visual_rows":unresolved})
    );
    if a.until && !frame.terminated {
        return Err("native match did not terminate in 7200 steps".into());
    }
    Ok(())
}
enum Control {
    Pause,
    Step,
    Reset,
    Speed(f64),
    Close,
}
struct Shared {
    frame: Option<BattleFrame>,
    error: Option<String>,
    paused: bool,
    speed: f64,
}
fn desktop(a: Args) -> Result<()> {
    let mut painter = Painter::load(&a.assets, a.width, a.height)?;
    let mut window = Window::new(
        "Recovered Clash spectator · loading original engine",
        a.width as usize,
        a.height as usize,
        WindowOptions::default(),
    )
    .map_err(|e| e.to_string())?;
    window.set_target_fps(30);
    let shared = Arc::new(Mutex::new(Shared {
        frame: None,
        error: None,
        paused: false,
        speed: 1.,
    }));
    let worker_shared = shared.clone();
    let (tx, rx) = mpsc::channel();
    let checkpoint = a.checkpoint;
    let worker = thread::spawn(move || {
        let run = || -> Result<()> {
            let mut battle = Battle::load(&checkpoint)?;
            let mut paused = false;
            let mut speed = 1.;
            let mut due = Instant::now();
            worker_shared.lock().unwrap().frame = Some(battle.snapshot()?);
            loop {
                let mut one = false;
                while let Ok(c) = rx.try_recv() {
                    match c {
                        Control::Close => return Ok(()),
                        Control::Pause => {
                            paused = !paused;
                            due = Instant::now();
                        }
                        Control::Step => {
                            paused = true;
                            one = true;
                        }
                        Control::Reset => {
                            battle.reset()?;
                            due = Instant::now();
                            worker_shared.lock().unwrap().frame = Some(battle.snapshot()?);
                        }
                        Control::Speed(s) => {
                            speed = s;
                            due = Instant::now();
                        }
                    }
                }
                if one || (!paused && !battle.session.is_terminal() && Instant::now() >= due) {
                    battle.step()?;
                    let mut state = worker_shared.lock().unwrap();
                    state.frame = Some(battle.snapshot()?);
                    due = due + Duration::from_secs_f64(0.05 / speed);
                    if due + Duration::from_millis(500) < Instant::now() {
                        due = Instant::now();
                    }
                } else {
                    thread::sleep(Duration::from_millis(2));
                }
                let mut state = worker_shared.lock().unwrap();
                state.paused = paused;
                state.speed = speed;
            }
        };
        if let Err(e) = run() {
            worker_shared.lock().unwrap().error = Some(e);
        }
    });
    let mut buffer = vec![0u32; (a.width * a.height) as usize];
    let mut last = None;
    let mut speed = 1f64;
    let mut window_frames = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            let _ = tx.send(Control::Pause);
        }
        if window.is_key_pressed(Key::Right, KeyRepeat::No) {
            let _ = tx.send(Control::Step);
        }
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            let _ = tx.send(Control::Reset);
            last = None;
        }
        if window.is_key_pressed(Key::Equal, KeyRepeat::No) {
            speed = (speed * 2.).min(16.);
            let _ = tx.send(Control::Speed(speed));
        }
        if window.is_key_pressed(Key::Minus, KeyRepeat::No) {
            speed = (speed / 2.).max(0.125);
            let _ = tx.send(Control::Speed(speed));
        }
        let (frame, error, paused) = {
            let s = shared.lock().unwrap();
            (s.frame.clone(), s.error.clone(), s.paused)
        };
        if let Some(error) = error {
            let _ = tx.send(Control::Close);
            let _ = worker.join();
            return Err(error);
        }
        if let Some(frame) = frame {
            if last != Some(frame.tick) {
                let rgba = painter.render(&frame)?;
                for (dest, px) in buffer.iter_mut().zip(rgba.pixels()) {
                    *dest = ((px[0] as u32) << 16) | ((px[1] as u32) << 8) | px[2] as u32;
                }
                last = Some(frame.tick);
                let status = if frame.terminated {
                    format!("ended · winner {:?}", frame.winner_side)
                } else if paused {
                    "paused".into()
                } else {
                    "playing".into()
                };
                window.set_title(&format!("Original engine · built-in bots · {} · {:.1}s · {} entities · {}× · Space pause / R restart",status,frame.elapsed_ms as f64/1000.,frame.entities.len(),speed));
            }
            if window.is_key_pressed(Key::S, KeyRepeat::No) {
                let path = PathBuf::from("runs/native-spectator.png");
                std::fs::create_dir_all("runs").map_err(|e| e.to_string())?;
                painter
                    .render(&frame)?
                    .save(&path)
                    .map_err(|e| e.to_string())?;
                eprintln!("Saved {}", path.display());
            }
        }
        window
            .update_with_buffer(&buffer, a.width as usize, a.height as usize)
            .map_err(|e| e.to_string())?;
        window_frames += 1;
        if a.window_frames.is_some_and(|limit| window_frames >= limit) {
            break;
        }
    }
    let _ = tx.send(Control::Close);
    let _ = worker.join();
    Ok(())
}
fn main() {
    if let Err(e) = (|| {
        let a = args()?;
        if a.ticks.is_some() || a.until {
            headless(a)
        } else {
            desktop(a)
        }
    })() {
        eprintln!("native-spectator: {e}");
        std::process::exit(1);
    }
}
