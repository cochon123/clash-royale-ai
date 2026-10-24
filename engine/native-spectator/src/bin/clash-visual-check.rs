//! Deterministic offline replay of recorded native frames through the Arena
//! presenter, for visual QA (driven by `tools/visual_check.py`).
//!
//! Display times are an exact grid instead of wall-clock pacing, so two runs
//! of the same code and fixture produce identical output. Before drawing
//! display tick `t`, every snapshot up to `floor(t) + lag` is observed; lag 0
//! matches `clash-web-scene`, which draws the newest snapshot. Like the live
//! presenter, every step is drawn from the first snapshot (draws update
//! effects, facing and turret aim); only `--from-tick..--to-tick` is recorded.
//!
//! Output directory:
//! - `trace.jsonl`: one line per display frame, with the display list exactly
//!   as the browser receives it and the Painter's probe (what each entity
//!   shows, effects resolved, attack impacts inferred).
//! - `textures/`: original sprite textures referenced by the display lists.
//! - `frames/NNNNN.png`: CPU reference image of each frame (`--images`).
use clash_native_spectator::{
    draw::Painter,
    frame::BattleFrame,
    probe,
    scene::{CaptureGuard, TextureCache},
};
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::{ExtendedColorType, ImageEncoder};
use serde_json::json;
use std::{
    cell::RefCell,
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
    rc::Rc,
    time::Instant,
};

struct Args {
    frames: PathBuf,
    assets: PathBuf,
    out: PathBuf,
    start_tick: Option<f64>,
    from_tick: Option<f64>,
    to_tick: Option<f64>,
    fps: f64,
    lag: f64,
    images: bool,
}

fn parse() -> Result<Args, String> {
    let mut frames = None;
    let mut out = None;
    let mut assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
    let (mut start_tick, mut from_tick, mut to_tick) = (None, None, None);
    let (mut fps, mut lag, mut images) = (20., 0., false);
    let mut it = std::env::args().skip(1);
    let number = |v: Option<String>, name: &str| -> Result<f64, String> {
        v.ok_or(format!("missing {name}"))?
            .parse::<f64>()
            .map_err(|_| format!("invalid {name}"))
    };
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--frames" => frames = Some(PathBuf::from(it.next().ok_or("missing frames")?)),
            "--assets" => assets = it.next().ok_or("missing assets")?.into(),
            "--out" => out = Some(PathBuf::from(it.next().ok_or("missing out")?)),
            "--start-tick" => start_tick = Some(number(it.next(), "start tick")?),
            "--from-tick" => from_tick = Some(number(it.next(), "from tick")?),
            "--to-tick" => to_tick = Some(number(it.next(), "to tick")?),
            "--fps" => fps = number(it.next(), "fps")?,
            "--lag" => lag = number(it.next(), "lag")?,
            "--images" => images = true,
            "--help" | "-h" => {
                println!("clash-visual-check --frames FRAMES.jsonl --out NEWDIR [--assets DIR] [--start-tick FIRST] [--from-tick N] [--to-tick M] [--fps 20] [--lag 0] [--images]\nDraws every step from --start-tick (default: first snapshot) and records --from-tick..--to-tick.");
                std::process::exit(0);
            }
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    if !(fps > 0. && fps <= 120.) {
        return Err("fps must be in (0, 120]".into());
    }
    if lag < 0. {
        return Err("lag must be >= 0".into());
    }
    Ok(Args {
        frames: frames.ok_or("--frames is required")?,
        assets,
        out: out.ok_or("--out is required")?,
        start_tick,
        from_tick,
        to_tick,
        fps,
        lag,
        images,
    })
}

fn save_png(image: &image::RgbaImage, path: &std::path::Path) -> Result<(), String> {
    let rgb = image::DynamicImage::ImageRgba8(image.clone()).to_rgb8();
    let file = BufWriter::new(fs::File::create(path).map_err(|e| e.to_string())?);
    PngEncoder::new_with_quality(file, CompressionType::Fast, FilterType::Sub)
        .write_image(
            rgb.as_raw(),
            rgb.width(),
            rgb.height(),
            ExtendedColorType::Rgb8,
        )
        .map_err(|e| e.to_string())
}

fn run() -> Result<(), String> {
    let a = parse()?;
    if a.out.exists()
        && fs::read_dir(&a.out)
            .map_err(|e| e.to_string())?
            .next()
            .is_some()
    {
        return Err(format!(
            "{} is not empty; choose a new directory",
            a.out.display()
        ));
    }
    fs::create_dir_all(a.out.join("frames")).map_err(|e| e.to_string())?;
    let bytes = fs::read(&a.frames).map_err(|e| format!("{}: {e}", a.frames.display()))?;
    let mut frames: Vec<BattleFrame> = bytes
        .split(|v| *v == b'\n')
        .filter(|line| !line.iter().all(u8::is_ascii_whitespace))
        .map(|line| serde_json::from_slice(line).map_err(|e| format!("invalid frame: {e}")))
        .collect::<Result<_, _>>()?;
    let first = frames.first().ok_or("empty frame stream")?;
    let match_id = first.match_id;
    // A capture may hold several matches; QA replays the first one only.
    let keep = frames.iter().take_while(|f| f.match_id == match_id).count();
    frames.truncate(keep);
    let start = a.start_tick.unwrap_or(frames[0].tick as f64);
    let from = a.from_tick.unwrap_or(start).max(start);
    let to = a.to_tick.unwrap_or(frames[frames.len() - 1].tick as f64);
    if to < from {
        return Err("to tick precedes from tick".into());
    }

    let mut painter = Painter::load(&a.assets, 540, 900)?;
    painter.anticipate(&frames);
    let cache = Rc::new(RefCell::new(TextureCache::new(&a.out.join("textures"))?));
    let mut trace =
        BufWriter::new(fs::File::create(a.out.join("trace.jsonl")).map_err(|e| e.to_string())?);
    let step = 20. / a.fps;
    let (mut next, mut step_index, mut index) = (0usize, 0u64, 0u64);
    let (mut draw_s, mut image_s) = (0., 0.);
    let started = Instant::now();
    loop {
        let t = start + step_index as f64 * step;
        step_index += 1;
        if t > to + 1e-9 {
            break;
        }
        let recording = t >= from - 1e-9;
        // Effects resolve and impacts register while observing, so record
        // from before this step's observations through its captured draw.
        if recording {
            probe::start();
        }
        let horizon = (t + 1e-9).floor() + a.lag;
        while next < frames.len() && frames[next].tick as f64 <= horizon {
            painter.observe(&frames[next]);
            next += 1;
        }
        if !recording {
            painter.draw(t)?;
            continue;
        }
        let draw_started = Instant::now();
        let capture = CaptureGuard::begin(cache.clone())?;
        painter.draw(t)?;
        let scene = capture.finish()?;
        draw_s += draw_started.elapsed().as_secs_f64();
        let record = probe::take();
        if a.images {
            // Same display tick again without capture: Painter::draw is
            // idempotent at a fixed time, and this yields the CPU reference.
            let image_started = Instant::now();
            let pixels = painter.draw(t)?;
            save_png(&pixels, &a.out.join(format!("frames/{index:05}.png")))?;
            image_s += image_started.elapsed().as_secs_f64();
        }
        let line = json!({
            "i": index, "t": t, "observed_tick": frames[next.saturating_sub(1)].tick,
            "new_textures": scene.textures.len(), "draws": scene.draws,
            "shown": record.shown, "effects": record.effects, "impacts": record.impacts,
            "particles": painter.particle_count(),
        });
        writeln!(trace, "{line}").map_err(|e| e.to_string())?;
        index += 1;
    }
    trace.flush().map_err(|e| e.to_string())?;
    let summary = json!({
        "frames": index, "start_tick": start, "from_tick": from, "to_tick": to, "fps": a.fps, "lag": a.lag,
        "match_id": match_id, "images": a.images,
        "mean_draw_ms": draw_s * 1000. / index.max(1) as f64,
        "mean_image_ms": image_s * 1000. / index.max(1) as f64,
        "seconds": started.elapsed().as_secs_f64(),
        "textures": cache.borrow().len(),
        "unresolved_rows": painter.unresolved_rows,
    });
    fs::write(a.out.join("summary.json"), summary.to_string()).map_err(|e| e.to_string())?;
    println!("{summary}");
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("clash-visual-check: {error}");
        std::process::exit(1);
    }
}
