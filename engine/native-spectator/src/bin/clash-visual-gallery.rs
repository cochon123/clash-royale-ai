//! Offline gallery: original scene JSON, shared textures, and a CPU reference
//! PNG at the same display time. Not an official 1:1 comparison.
use clash_native_spectator::{
    draw::{parse_card_list, Painter},
    frame::BattleFrame,
    gallery::{
        self, display_samples, observe_horizon, primary_subject, push_frame, read_manifest,
        synthetic_march, write_manifest, GalleryCase, SubjectKey,
    },
    scene::{CaptureGuard, TextureCache},
};
use serde_json::json;
use std::{
    cell::RefCell,
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    rc::Rc,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("clash-visual-gallery: {error}");
        std::process::exit(1);
    }
}

struct Args {
    frames: Option<PathBuf>,
    output: PathBuf,
    assets: PathBuf,
    from_tick: Option<u32>,
    to_tick: Option<u32>,
    fps: f64,
    cards: Vec<String>,
    synthetic: bool,
}

fn args() -> Result<Args, String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut frames = None;
    let mut output = None;
    let mut assets = root.join("analysis/native-spectator");
    let mut from_tick = None;
    let mut to_tick = None;
    let mut fps = 30.;
    let mut cards = Vec::new();
    let mut synthetic = false;
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--frames" => frames = Some(PathBuf::from(it.next().ok_or("missing frames JSONL")?)),
            "--output" => output = Some(PathBuf::from(it.next().ok_or("missing output directory")?)),
            "--assets" => assets = it.next().ok_or("missing asset directory")?.into(),
            "--from-tick" => {
                from_tick = Some(it.next().ok_or("missing tick")?.parse().map_err(|_| "from tick")?)
            }
            "--to-tick" => {
                to_tick = Some(it.next().ok_or("missing tick")?.parse().map_err(|_| "to tick")?)
            }
            "--fps" => fps = it.next().ok_or("missing fps")?.parse().map_err(|_| "fps")?,
            "--cards" => cards.extend(parse_card_list(&it.next().ok_or("missing card names")?)),
            "--synthetic" => synthetic = true,
            "--help" | "-h" => {
                println!(
                    "clash-visual-gallery --frames JSONL --output DIR [--assets DIR] [--cards NAMES] [--from-tick N] [--to-tick M] [--fps 30] [--synthetic]\n\
NAMES is one comma-separated argument; spaces stay inside a native card name.\n\
--synthetic also writes short isolated walks for both teams and headings 1, 5, and 9.\n\
This does not claim official animation parity."
                );
                std::process::exit(0);
            }
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    if frames.is_none() && !synthetic {
        return Err("--frames JSONL is required unless --synthetic is set".into());
    }
    if synthetic && cards.is_empty() {
        return Err("--synthetic needs --cards".into());
    }
    if !(1. ..=120.).contains(&fps) {
        return Err("--fps must be within 1..120".into());
    }
    Ok(Args {
        frames,
        output: output.ok_or("--output DIR is required")?,
        assets,
        from_tick,
        to_tick,
        fps,
        cards,
        synthetic,
    })
}

fn load_frames(path: &Path, from: Option<u32>, to: Option<u32>) -> Result<Vec<BattleFrame>, String> {
    let file = fs::File::open(path).map_err(|e| format!("open frames: {e}"))?;
    let mut frames: Vec<BattleFrame> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| e.to_string())?;
        if line.trim().is_empty() {
            continue;
        }
        frames.push(serde_json::from_str(&line).map_err(|e| format!("invalid frame: {e}"))?);
    }
    if frames.is_empty() {
        return Err("empty frame stream".into());
    }
    let from = from.unwrap_or(frames.first().unwrap().tick);
    let to = to.unwrap_or(frames.last().unwrap().tick);
    let warmup = from.saturating_sub(40);
    let end = to.saturating_add(1);
    frames.retain(|frame| (warmup..=end).contains(&frame.tick));
    if frames.is_empty() {
        return Err("no frames in the requested tick range".into());
    }
    Ok(frames)
}

fn render_range(
    painter: &mut Painter,
    cache: &Rc<RefCell<TextureCache>>,
    frames: &[BattleFrame],
    from: f64,
    to: f64,
    fps: f64,
    cards: &[String],
    cases: &mut Vec<GalleryCase>,
    output: &Path,
) -> Result<(), String> {
    let mut next = 0usize;
    for t in display_samples(from, to, fps) {
        let horizon = observe_horizon(t);
        while next < frames.len() && frames[next].tick as f64 <= horizon + 1e-9 {
            painter.observe(&frames[next]);
            next += 1;
        }
        if painter.latest_tick().is_none() {
            continue;
        }
        let capture = CaptureGuard::begin_reference(cache.clone())?;
        let image = painter.draw(t)?;
        let list = capture.finish()?;
        let snapshot = painter.snapshot_at(t);
        let subjects = painter.shown_subjects();
        let key = primary_subject(&subjects, cards)
            .map(|subject| SubjectKey {
                card: subject.row.clone(),
                team: subject.team,
                state: subject.state.to_owned(),
                heading: subject.heading,
            })
            .unwrap_or(SubjectKey {
                card: "arena".into(),
                team: 0,
                state: "idle".into(),
                heading: 0,
            });
        let time_ms = (t * 50.).round().max(0.) as u64;
        let (scene_rel, reference_rel) = push_frame(cases, &key, time_ms);
        let scene_path = output.join(&scene_rel);
        let reference_path = output.join(&reference_rel);
        if let Some(parent) = scene_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        image.save(&reference_path).map_err(|e| e.to_string())?;
        let layout = snapshot.as_ref().map(|frame| painter.controls_layout(frame));
        let packet = json!({
            "v": 1,
            "tick": snapshot.as_ref().map(|frame| frame.tick).unwrap_or(t.floor() as u32),
            "display_tick": t,
            "time_ms": time_ms,
            "width": image.width(),
            "height": image.height(),
            "clear": [19, 27, 31, 255],
            "layout": layout.as_ref().map(|layout| json!({
                "field": layout.field, "world": layout.world, "cards": layout.cards
            })),
            "textures": list.textures,
            "draws": list.draws,
        });
        fs::write(scene_path, serde_json::to_vec(&packet).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn run() -> Result<(), String> {
    let args = args()?;
    fs::create_dir_all(&args.output).map_err(|e| e.to_string())?;
    let cache = Rc::new(RefCell::new(TextureCache::new(&args.output.join("textures"))?));
    let mut painter = Painter::load_for_cards(&args.assets, 540, 900, &args.cards)?;
    let mut cases = Vec::new();
    if let Some(path) = &args.frames {
        let frames = load_frames(path, args.from_tick, args.to_tick)?;
        let from = args.from_tick.unwrap_or(frames.first().unwrap().tick) as f64;
        let to = args.to_tick.unwrap_or(frames.last().unwrap().tick) as f64;
        render_range(
            &mut painter,
            &cache,
            &frames,
            from,
            to.min(frames.last().unwrap().tick as f64),
            args.fps,
            &args.cards,
            &mut cases,
            &args.output,
        )?;
    }
    if args.synthetic {
        for card in &args.cards {
            for team in [0, 1] {
                for heading in [1u8, 5, 9] {
                    let frames = synthetic_march(card, team, heading, 5);
                    render_range(
                        &mut painter,
                        &cache,
                        &frames,
                        0.,
                        4.,
                        args.fps,
                        std::slice::from_ref(card),
                        &mut cases,
                        &args.output,
                    )?;
                }
            }
        }
    }
    let manifest_path = args.output.join("manifest.json");
    let mut existing = read_manifest(&manifest_path)?;
    gallery::merge_cases(&mut existing, cases);
    write_manifest(&manifest_path, &existing)?;
    println!(
        "{}",
        json!({
            "manifest": manifest_path,
            "cases": existing.len(),
            "frames": existing.iter().map(|case| case.frames.len()).sum::<usize>(),
            "official_parity": false,
        })
    );
    Ok(())
}
