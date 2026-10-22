#[cfg(feature = "compiled-engine")]
use clash_native_spectator::backend::Battle;
use clash_native_spectator::{
    clock::{Flow, PlaybackClock, DEFAULT_LAG_TICKS},
    draw::Painter,
    frame::BattleFrame,
    input::PlayerInput,
};
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};
#[cfg(feature = "compiled-engine")]
use std::{
    io::BufWriter,
    sync::{mpsc, Arc, Mutex},
    thread,
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
    follow: Option<PathBuf>,
    control_file: Option<PathBuf>,
    ready_file: Option<PathBuf>,
    title: String,
    sequence: Option<PathBuf>,
    from_tick: Option<u32>,
    to_tick: Option<u32>,
    fps: f64,
    input_file: Option<PathBuf>,
    display_lag_ticks: f64,
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
        follow: None,
        control_file: None,
        ready_file: None,
        title: "Recovered Clash spectator".into(),
        sequence: None,
        from_tick: None,
        to_tick: None,
        fps: 30.,
        input_file: None,
        display_lag_ticks: DEFAULT_LAG_TICKS,
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
            "--follow" => a.follow = Some(it.next().ok_or("frame path")?.into()),
            "--control-file" => a.control_file = Some(it.next().ok_or("control file path")?.into()),
            "--input-file" => a.input_file = Some(it.next().ok_or("input queue path")?.into()),
            "--display-lag-ticks" => {
                a.display_lag_ticks = it
                    .next()
                    .ok_or("display lag")?
                    .parse()
                    .map_err(|_| "display lag")?
            }
            "--ready-file" => a.ready_file = Some(it.next().ok_or("ready file path")?.into()),
            "--title" => a.title = it.next().ok_or("window title")?,
            "--render-sequence" => {
                a.sequence = Some(it.next().ok_or("new output directory")?.into())
            }
            "--from-tick" => {
                a.from_tick = Some(it.next().ok_or("tick")?.parse().map_err(|_| "from tick")?)
            }
            "--to-tick" => {
                a.to_tick = Some(it.next().ok_or("tick")?.parse().map_err(|_| "to tick")?)
            }
            "--fps" => a.fps = it.next().ok_or("fps")?.parse().map_err(|_| "fps")?,
            "--help" | "-h" => {
                println!("Original-engine spectator (built-in Normal scenario bots).\nUsage: clash-native-spectator [--ticks N | --run-to-end] [--render PNG] [--record JSONL] [--audit-out NEWDIR] [--assets DIR] [--checkpoint DIR]\nFollow external snapshots: clash-native-spectator --follow FRAME.json [--control-file COMMAND.json] [--ready-file READY.json] [--title LABEL]\nInterpolated PNG sequence from a capture: clash-native-spectator --follow FRAMES.jsonl --render-sequence NEWDIR [--from-tick N] [--to-tick M] [--fps 30]\nEngine mode: Space pause, Right one tick, R restart, +/- speed, S screenshot, Esc close. Follow mode: Space pause and R reset write commands when --control-file is set; S screenshot, Esc close. No human controls or Banana policy connection in this viewer.");
                std::process::exit(0);
            }
            _ => return Err(format!("unknown argument {k}")),
        }
    }
    if a.width < 240 || a.height < 320 || a.width > 2160 || a.height > 3840 {
        return Err("image dimensions out of bounds".into());
    }
    if a.follow.is_some()
        && (a.ticks.is_some() || a.until || a.record.is_some() || a.audit.is_some())
    {
        return Err("--follow cannot be combined with engine execution/recording options".into());
    }
    if a.sequence.is_some()
        && !a
            .follow
            .as_ref()
            .is_some_and(|p| p.extension().is_some_and(|e| e == "jsonl"))
    {
        return Err("--render-sequence needs --follow FRAMES.jsonl".into());
    }
    if !(1. ..=240.).contains(&a.fps) {
        return Err("--fps must be within 1..240".into());
    }
    if a.ready_file.is_some() && a.follow.is_none() {
        return Err("--ready-file currently requires --follow".into());
    }
    if a.input_file.is_some() && a.follow.is_none() {
        return Err("--input-file requires --follow".into());
    }
    if !a.display_lag_ticks.is_finite() || !(0. ..=30.).contains(&a.display_lag_ticks) {
        return Err("display lag must be within 0..30 ticks".into());
    }
    Ok(a)
}
#[cfg(feature = "compiled-engine")]
fn record(out: &mut Option<BufWriter<File>>, frame: &BattleFrame) -> Result<()> {
    if let Some(f) = out {
        serde_json::to_writer(&mut *f, frame).map_err(|e| e.to_string())?;
        f.write_all(b"\n").map_err(|e| e.to_string())?;
    }
    Ok(())
}
#[cfg(feature = "compiled-engine")]
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
#[cfg(feature = "compiled-engine")]
enum Control {
    Pause,
    Step,
    Reset,
    Speed(f64),
    Close,
}
#[cfg(feature = "compiled-engine")]
struct Shared {
    error: Option<String>,
    paused: bool,
    speed: f64,
}

fn present(buffer: &mut [u32], rgba: &image::RgbaImage) {
    for (dest, px) in buffer.iter_mut().zip(rgba.pixels()) {
        *dest = ((px[0] as u32) << 16) | ((px[1] as u32) << 8) | px[2] as u32;
    }
}

#[cfg(feature = "compiled-engine")]
fn desktop(a: Args) -> Result<()> {
    let mut painter = Painter::load(&a.assets, a.width, a.height)?;
    let mut window = Window::new(
        "Recovered Clash spectator · loading original engine",
        a.width as usize,
        a.height as usize,
        WindowOptions::default(),
    )
    .map_err(|e| e.to_string())?;
    window.set_target_fps(60);
    let shared = Arc::new(Mutex::new(Shared {
        error: None,
        paused: false,
        speed: 1.,
    }));
    let worker_shared = shared.clone();
    let (tx, rx) = mpsc::channel();
    // Every native snapshot is forwarded, so presentation events (births,
    // impacts, deaths) are never skipped when drawing is slower than ticks.
    let (frame_tx, frame_rx) = mpsc::channel::<BattleFrame>();
    let checkpoint = a.checkpoint;
    let worker = thread::spawn(move || {
        let run = || -> Result<()> {
            let mut battle = Battle::load(&checkpoint)?;
            let mut paused = false;
            let mut speed = 1.;
            let mut due = Instant::now();
            let _ = frame_tx.send(battle.snapshot()?);
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
                            let _ = frame_tx.send(battle.snapshot()?);
                        }
                        Control::Speed(s) => {
                            speed = s;
                            due = Instant::now();
                        }
                    }
                }
                if one || (!paused && !battle.session.is_terminal() && Instant::now() >= due) {
                    battle.step()?;
                    let _ = frame_tx.send(battle.snapshot()?);
                    due += Duration::from_secs_f64(0.05 / speed);
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
    let mut clock = PlaybackClock::default();
    let mut speed = 1f64;
    let mut window_frames = 0;
    let mut current: Option<BattleFrame> = None;
    let mut last_title = Instant::now() - Duration::from_secs(1);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            let _ = tx.send(Control::Pause);
        }
        if window.is_key_pressed(Key::Right, KeyRepeat::No) {
            let _ = tx.send(Control::Step);
        }
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            let _ = tx.send(Control::Reset);
            clock.reset();
        }
        if window.is_key_pressed(Key::Equal, KeyRepeat::No) {
            speed = (speed * 2.).min(16.);
            let _ = tx.send(Control::Speed(speed));
        }
        if window.is_key_pressed(Key::Minus, KeyRepeat::No) {
            speed = (speed / 2.).max(0.125);
            let _ = tx.send(Control::Speed(speed));
        }
        for frame in frame_rx.try_iter() {
            painter.observe(&frame);
            current = Some(frame);
        }
        let (error, paused) = {
            let s = shared.lock().unwrap();
            (s.error.clone(), s.paused)
        };
        if let Some(error) = error {
            let _ = tx.send(Control::Close);
            let _ = worker.join();
            return Err(error);
        }
        if let (Some(frame), Some(oldest), Some(latest)) =
            (&current, painter.oldest_tick(), painter.latest_tick())
        {
            let flow = if frame.terminated {
                Flow::Finished
            } else if paused {
                Flow::Paused
            } else {
                Flow::Running
            };
            let display = clock.advance(oldest, latest, speed, flow);
            let rgba = painter.draw(display)?;
            present(&mut buffer, &rgba);
            if last_title.elapsed() > Duration::from_millis(250) {
                last_title = Instant::now();
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
                rgba.save(&path).map_err(|e| e.to_string())?;
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

#[cfg(not(feature = "compiled-engine"))]
fn headless(_: Args) -> Result<()> {
    Err("Standalone built-in bots require --features compiled-engine. Use python -m rl.banana_play for human games.".into())
}

#[cfg(not(feature = "compiled-engine"))]
fn desktop(_: Args) -> Result<()> {
    Err("Launch python -m rl.banana_play to play, or rl.banana_watch to watch. Standalone built-in bots require --features compiled-engine.".into())
}

fn atomic_json_file(path: &std::path::Path, value: &serde_json::Value) -> Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let tmp = path.with_extension(format!("tmp-{}", std::process::id()));
    let mut file = File::create(&tmp).map_err(|e| e.to_string())?;
    serde_json::to_writer(&mut file, value).map_err(|e| e.to_string())?;
    file.write_all(b"\n").map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, path).map_err(|e| e.to_string())
}

fn send_follow_control(path: &std::path::Path, op: &str, speed: Option<f64>) -> Result<()> {
    let serial = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos() as u64;
    let mut command = serde_json::json!({"serial":serial,"op":op});
    if let Some(speed) = speed {
        command["speed"] = serde_json::json!(speed);
    }
    atomic_json_file(path, &command)
}

fn append_input(path: &std::path::Path, value: &serde_json::Value) -> Result<()> {
    let mut bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?
        .write_all(&bytes)
        .map_err(|e| e.to_string())
}

/// Interpolated PNG frames from a recorded capture, for visual review.
fn render_sequence(
    a: &Args,
    painter: &mut Painter,
    frame_path: &std::path::Path,
    out_dir: &std::path::Path,
) -> Result<()> {
    if out_dir.exists() {
        return Err(format!(
            "{} already exists; choose a new directory",
            out_dir.display()
        ));
    }
    std::fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
    let bytes = std::fs::read(frame_path).map_err(|e| e.to_string())?;
    let frames: Vec<BattleFrame> = bytes
        .split(|v| *v == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).map_err(|e| e.to_string()))
        .collect::<Result<_>>()?;
    let first = frames.first().ok_or("empty frame stream")?;
    // A capture may contain several matches; use the first tick range.
    let from = a.from_tick.unwrap_or(first.tick) as f64;
    let to = a.to_tick.map_or_else(
        || {
            frames
                .iter()
                .take_while(|f| f.match_id == first.match_id)
                .last()
                .map_or(from, |f| f.tick as f64)
        },
        |t| t as f64,
    );
    let step = 20. / a.fps;
    let mut next = 0usize;
    let mut t = from;
    let mut index = 0u32;
    let mut draw_seconds = 0.;
    while t <= to + 1e-9 {
        // Observe ahead by the live viewer's display lag.
        while next < frames.len() && (frames[next].tick as f64) <= t.ceil() + DEFAULT_LAG_TICKS {
            if frames[next].match_id != first.match_id {
                next = frames.len();
                break;
            }
            painter.observe(&frames[next]);
            next += 1;
        }
        let started = Instant::now();
        let image = painter.draw(t)?;
        draw_seconds += started.elapsed().as_secs_f64();
        image
            .save(out_dir.join(format!("{index:05}.png")))
            .map_err(|e| e.to_string())?;
        index += 1;
        t += step;
    }
    println!(
        "{}",
        serde_json::json!({"sequence":out_dir,"frames":index,"from_tick":from,"to_tick":to,"fps":a.fps,
            "mean_draw_ms":draw_seconds*1000./index.max(1) as f64,"unresolved_rows":painter.unresolved_rows})
    );
    Ok(())
}

fn follow(a: Args, frame_path: PathBuf) -> Result<()> {
    let mut painter = Painter::load(&a.assets, a.width, a.height)?;
    if let Some(dir) = &a.sequence {
        return render_sequence(&a, &mut painter, &frame_path, dir);
    }
    if let Some(path) = &a.render {
        let bytes = std::fs::read(&frame_path).map_err(|e| e.to_string())?;
        let mut tick = None;
        if frame_path.extension().is_some_and(|v| v == "jsonl") {
            for line in bytes.split(|v| *v == b'\n').filter(|line| !line.is_empty()) {
                let frame: BattleFrame = serde_json::from_slice(line).map_err(|e| e.to_string())?;
                tick = Some(frame.tick);
                painter.observe(&frame);
            }
        } else {
            let frame: BattleFrame = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
            tick = Some(frame.tick);
            painter.observe(&frame);
        }
        let tick = tick.ok_or("empty frame stream")?;
        painter
            .draw(tick as f64)?
            .save(path)
            .map_err(|e| e.to_string())?;
        println!(
            "{}",
            serde_json::json!({"rendered":path,"tick":tick,"unresolved_rows":painter.unresolved_rows})
        );
        return Ok(());
    }
    let mut window = Window::new(
        &a.title,
        a.width as usize,
        a.height as usize,
        WindowOptions::default(),
    )
    .map_err(|e| e.to_string())?;
    window.set_target_fps(60);
    if let Some(path) = &a.ready_file {
        atomic_json_file(
            path,
            &serde_json::json!({"ready":true,"pid":std::process::id(),"title":a.title}),
        )?;
    }
    let mut buffer = vec![0u32; (a.width * a.height) as usize];
    let mut current: Option<BattleFrame> = None;
    let mut last_stamp: Option<(std::time::SystemTime, u64)> = None;
    let mut last_error: Option<String> = None;
    let mut last_logged_error: Option<String> = None;
    let mut window_frames = 0u32;
    let mut playback_speed = 1.0f64;
    let mut runtime_status = "waiting".to_owned();
    let mut controller_names = String::new();
    let mut clock = PlaybackClock::new(a.display_lag_ticks);
    let mut player = PlayerInput::default();
    let (mut left_down, mut right_down) = (false, false);
    // A press that started on a card; releasing it over the arena plays it.
    let mut dragging = false;
    let mut last_aim = Instant::now() - Duration::from_secs(1);
    let mut last_aim_target = None;
    let mut last_title = Instant::now() - Duration::from_secs(1);
    while window.is_open() && (a.input_file.is_some() || !window.is_key_down(Key::Escape)) {
        if a.input_file.is_none() && window.is_key_pressed(Key::Space, KeyRepeat::No) {
            if let Some(path) = &a.control_file {
                send_follow_control(path, "pause", None)?;
            } else {
                eprintln!("follow: Space ignored; no --control-file was configured");
            }
        }
        if a.input_file.is_none() && window.is_key_pressed(Key::R, KeyRepeat::No) {
            if let Some(path) = &a.control_file {
                send_follow_control(path, "reset", None)?;
            } else {
                eprintln!("follow: R ignored; no --control-file was configured");
            }
        }
        if a.input_file.is_none() && window.is_key_pressed(Key::Equal, KeyRepeat::No) {
            playback_speed = (playback_speed * 2.).min(16.);
            if let Some(path) = &a.control_file {
                send_follow_control(path, "speed", Some(playback_speed))?;
            }
        }
        if a.input_file.is_none() && window.is_key_pressed(Key::Minus, KeyRepeat::No) {
            playback_speed = (playback_speed / 2.).max(0.125);
            if let Some(path) = &a.control_file {
                send_follow_control(path, "speed", Some(playback_speed))?;
            }
        }
        // The producer replaces the file atomically; re-read only on change.
        let stamp = std::fs::metadata(&frame_path)
            .ok()
            .and_then(|m| Some((m.modified().ok()?, m.len())));
        if stamp.is_some() && stamp != last_stamp {
            match std::fs::read(&frame_path)
                .map_err(|e| e.to_string())
                .and_then(|bytes| {
                    let value: serde_json::Value =
                        serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
                    let frame: BattleFrame =
                        serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
                    Ok((frame, value))
                }) {
                Ok((frame, metadata)) => {
                    last_stamp = stamp;
                    runtime_status = metadata["runtime_status"]
                        .as_str()
                        .unwrap_or("running")
                        .to_owned();
                    if let Some(names) = metadata["controller_names"].as_array() {
                        controller_names = names
                            .iter()
                            .filter_map(serde_json::Value::as_str)
                            .collect::<Vec<_>>()
                            .join(" vs ");
                    }
                    if let Some(speed) = metadata["playback_speed"].as_f64() {
                        playback_speed = speed;
                    }
                    last_error = metadata["runtime_error"]
                        .as_str()
                        .filter(|s| !s.is_empty())
                        .map(str::to_owned);
                    if runtime_status == "failed" && last_error.is_none() {
                        last_error = Some("bridge reported failed without details".into());
                    }
                    if current
                        .as_ref()
                        .is_some_and(|old| old.match_id != frame.match_id)
                    {
                        clock.reset();
                    }
                    painter.observe(&frame);
                    current = Some(frame);
                }
                // A partially visible write is retried on the next poll.
                Err(error) => last_error = Some(format!("frame: {error}")),
            }
        }
        if let (Some(path), Some(frame)) = (&a.input_file, &current) {
            player.synchronize(frame);
            let geometry = painter.controls_layout(frame);
            let mouse = window.get_mouse_pos(MouseMode::Discard);
            player.cursor = mouse.and_then(|(x, y)| geometry.to_world(x as f64, y as f64));
            let mut commands = Vec::new();
            for (key, op) in [(Key::Space, "pause"), (Key::R, "reset"), (Key::Q, "close")] {
                if window.is_key_pressed(key, KeyRepeat::No) {
                    commands.push(player.command(op));
                }
            }
            for (slot, key) in [Key::Key1, Key::Key2, Key::Key3, Key::Key4]
                .iter()
                .enumerate()
            {
                if window.is_key_pressed(*key, KeyRepeat::No) {
                    if let Some(command) = player.select(frame, slot as u32) {
                        commands.push(command);
                    }
                }
            }
            let left = window.get_mouse_down(MouseButton::Left);
            let right = window.get_mouse_down(MouseButton::Right);
            if (right && !right_down) || window.is_key_pressed(Key::Escape, KeyRepeat::No) {
                dragging = false;
                commands.push(player.cancel());
            } else if left && !left_down {
                if let Some((x, y)) = mouse {
                    if let Some(slot) = geometry.card_at(x as f64, y as f64) {
                        if let Some(command) = player.select(frame, slot) {
                            commands.push(command);
                            dragging = true;
                        }
                    } else if let Some(command) = player.target_command("play") {
                        commands.push(command);
                    }
                }
            } else if !left && left_down && dragging {
                // Drag and drop: release over the arena deploys; releasing on
                // the hand keeps the selection for click-to-place.
                dragging = false;
                if let Some(command) = player.target_command("play") {
                    commands.push(command);
                }
            }
            (left_down, right_down) = (left, right);
            // Re-check on movement, and periodically because elixir and the
            // legal area (after a tower falls) change while the cursor rests.
            let aim_target = player.selection.zip(player.cursor);
            let since = last_aim.elapsed();
            if aim_target.is_some()
                && since >= Duration::from_millis(50)
                && (aim_target != last_aim_target || since >= Duration::from_millis(500))
            {
                if let Some(command) = player.target_command("aim") {
                    commands.push(command);
                    last_aim = Instant::now();
                    last_aim_target = aim_target;
                }
            }
            let closing = commands.iter().any(|c| c["op"] == "close");
            for command in commands {
                append_input(path, &command)?;
            }
            if closing {
                break;
            }
        }
        let flow = if runtime_status == "ended" || current.as_ref().is_some_and(|f| f.terminated) {
            Flow::Finished
        } else if runtime_status == "running" {
            Flow::Running
        } else {
            Flow::Paused
        };
        if let (Some(oldest), Some(latest)) = (painter.oldest_tick(), painter.latest_tick()) {
            let display = clock.advance(oldest, latest, playback_speed, flow);
            match painter.draw(display) {
                Ok(mut rgba) => {
                    if let Some(frame) = &current {
                        painter.draw_player_controls(&mut rgba, frame, &player)?;
                    }
                    if window.is_key_pressed(Key::S, KeyRepeat::No) {
                        let path = a
                            .input_file
                            .as_ref()
                            .and_then(|p| p.parent())
                            .map(|p| p.join("screenshot.png"))
                            .unwrap_or_else(|| PathBuf::from("runs/native-spectator.png"));
                        if let Some(parent) = path.parent() {
                            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                        }
                        rgba.save(&path).map_err(|e| e.to_string())?;
                        eprintln!("Saved {}", path.display());
                    }
                    present(&mut buffer, &rgba);
                }
                Err(e) => last_error = Some(format!("render: {e}")),
            }
        }
        if last_title.elapsed() > Duration::from_millis(250) {
            last_title = Instant::now();
            if let Some(error) = &last_error {
                window.set_title(&format!("{} · ERROR: {}", a.title, error));
                if last_logged_error.as_deref() != Some(error) {
                    eprintln!("follow viewer: {error}");
                    last_logged_error = Some(error.clone());
                }
            } else if let Some(frame) = &current {
                last_logged_error = None;
                let status = if frame.terminated {
                    format!("ended · winner {:?}", frame.winner_side)
                } else {
                    "live".into()
                };
                window.set_title(&format!(
                    "{} · {} · {} · tick {} · {:.1}s · {:.2}×",
                    a.title,
                    if controller_names.is_empty() {
                        runtime_status.clone()
                    } else {
                        format!("{} · {}", runtime_status, controller_names)
                    },
                    status,
                    frame.tick,
                    frame.elapsed_ms as f64 / 1000.,
                    playback_speed,
                ));
            } else {
                window.set_title(&format!(
                    "{} · waiting for {}",
                    a.title,
                    frame_path.display()
                ));
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
    if let Some(path) = &a.input_file {
        append_input(path, &player.command("close"))?;
    } else if let Some(path) = &a.control_file {
        send_follow_control(path, "close", None)?;
    }
    Ok(())
}
fn main() {
    if let Err(e) = (|| {
        let a = args()?;
        if a.ticks.is_some() || a.until {
            headless(a)
        } else if let Some(frame) = a.follow.clone() {
            follow(a, frame)
        } else {
            desktop(a)
        }
    })() {
        eprintln!("native-spectator: {e}");
        std::process::exit(1);
    }
}
