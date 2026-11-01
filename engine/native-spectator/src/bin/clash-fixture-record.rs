//! Record clash-web-scene packets and RGBA poses, and run the address gate.
//!
//! The display loop is the one the wasm parity runner repeats: observe snapshot
//! `i` when `i/20 <= wall`, then `step(1/fps)`.

use clash_native_spectator::export_session::{
    address_permutation, json_close, ExportSession,
};
use clash_native_spectator::frame::BattleFrame;
use image::{ImageFormat, RgbaImage};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

struct Args {
    assets: PathBuf,
    frames: PathBuf,
    out: PathBuf,
    cards: Vec<String>,
    from_tick: Option<u32>,
    to_tick: Option<u32>,
    limit: Option<usize>,
    lag: f64,
    fps: f64,
    address_gate: bool,
    gate_frames: usize,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("clash-fixture-record: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = parse_args()?;
    fs::create_dir_all(&args.out).map_err(|e| e.to_string())?;
    let frames = load_frames(&args)?;
    if frames.is_empty() {
        return Err("no snapshots in the requested window".into());
    }
    let mut session = ExportSession::open(&args.assets, &args.cards, 540, 900, args.lag)?;
    if args.address_gate {
        let gate = address_gate(&mut session, &frames, args.gate_frames, args.fps)?;
        fs::write(
            args.out.join("address-gate.json"),
            serde_json::to_vec_pretty(&gate).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        println!("{}", serde_json::to_string(&gate).map_err(|e| e.to_string())?);
        session.reset();
        session.clear_textures();
    }
    let packets = record(&mut session, &args, &frames)?;
    println!(
        "recorded {} packets from {} snapshots into {}",
        packets,
        frames.len(),
        args.out.display()
    );
    Ok(())
}

fn parse_args() -> Result<Args, String> {
    let mut assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
    let mut frames = None;
    let mut out = None;
    let mut cards = Vec::new();
    let mut from_tick = None;
    let mut to_tick = None;
    let mut limit = None;
    let mut lag = 3.0;
    let mut fps = 30.0;
    let mut address_gate = false;
    let mut gate_frames = 40usize;
    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--assets" => assets = iter.next().ok_or("missing --assets")?.into(),
            "--frames" => frames = Some(PathBuf::from(iter.next().ok_or("missing --frames")?)),
            "--out" => out = Some(PathBuf::from(iter.next().ok_or("missing --out")?)),
            "--cards" => {
                cards = clash_native_spectator::export_session::parse_cards(
                    &iter.next().ok_or("missing --cards")?,
                )
            }
            "--from-tick" => {
                from_tick = Some(
                    iter.next()
                        .ok_or("missing --from-tick")?
                        .parse()
                        .map_err(|_| "bad tick")?,
                )
            }
            "--to-tick" => {
                to_tick = Some(
                    iter.next()
                        .ok_or("missing --to-tick")?
                        .parse()
                        .map_err(|_| "bad tick")?,
                )
            }
            "--limit" => {
                limit = Some(
                    iter.next()
                        .ok_or("missing --limit")?
                        .parse()
                        .map_err(|_| "bad limit")?,
                )
            }
            "--lag" => {
                lag = iter
                    .next()
                    .ok_or("missing --lag")?
                    .parse()
                    .map_err(|_| "bad lag")?
            }
            "--fps" => {
                fps = iter
                    .next()
                    .ok_or("missing --fps")?
                    .parse()
                    .map_err(|_| "bad fps")?
            }
            "--address-gate" => address_gate = true,
            "--gate-frames" => {
                gate_frames = iter
                    .next()
                    .ok_or("missing --gate-frames")?
                    .parse()
                    .map_err(|_| "bad gate frame count")?
            }
            "--help" | "-h" => {
                println!(
                    "clash-fixture-record --frames snapshots.jsonl --out DIR [--assets DIR] [--cards a,b] [--from-tick N] [--to-tick N] [--limit N] [--lag 3] [--fps 30] [--address-gate] [--gate-frames 40]"
                );
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    if !(1.0..=120.0).contains(&fps) {
        return Err("fps must be in 1..120".into());
    }
    Ok(Args {
        assets,
        frames: frames.ok_or("--frames is required")?,
        out: out.ok_or("--out is required")?,
        cards,
        from_tick,
        to_tick,
        limit,
        lag,
        fps,
        address_gate,
        gate_frames,
    })
}

fn load_frames(args: &Args) -> Result<Vec<Vec<u8>>, String> {
    let file = fs::File::open(&args.frames).map_err(|e| format!("{}: {e}", args.frames.display()))?;
    let mut frames = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| e.to_string())?;
        if line.trim().is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(&line).map_err(|e| format!("snapshot: {e}"))?;
        let tick = value["tick"].as_u64().unwrap_or(0) as u32;
        if args.from_tick.is_some_and(|start| tick < start) {
            continue;
        }
        if args.to_tick.is_some_and(|end| tick > end) {
            break;
        }
        frames.push(line.into_bytes());
        if args.limit.is_some_and(|limit| frames.len() >= limit) {
            break;
        }
    }
    Ok(frames)
}

struct Replay {
    packets: Vec<Value>,
}

fn drive(
    session: &mut ExportSession,
    frames: &[Vec<u8>],
    dt: f64,
    mut on_packet: impl FnMut(&mut ExportSession, &Value) -> Result<(), String>,
) -> Result<Replay, String> {
    let tick_dt = 1.0 / 20.0;
    let mut wall = 0.0;
    let mut next = 0;
    let mut packets = Vec::new();
    let mut idle = 0;
    let cap = frames.len().saturating_mul(4).saturating_add(64);
    for _ in 0..cap {
        while next < frames.len() && (next as f64) * tick_dt <= wall + 1e-9 {
            session.observe_json(&frames[next])?;
            next += 1;
        }
        if session.step(dt)? > 0 {
            let packet: Value =
                serde_json::from_slice(session.packet()).map_err(|e| e.to_string())?;
            on_packet(session, &packet)?;
            packets.push(packet);
            idle = 0;
        } else if next >= frames.len() {
            idle += 1;
            if idle >= 8 {
                break;
            }
        }
        wall += dt;
    }
    Ok(Replay { packets })
}

fn record(session: &mut ExportSession, args: &Args, frames: &[Vec<u8>]) -> Result<usize, String> {
    let texture_dir = args.out.join("textures");
    fs::create_dir_all(&texture_dir).map_err(|e| e.to_string())?;
    let mut packet_file = fs::File::create(args.out.join("packets.jsonl")).map_err(|e| e.to_string())?;
    let mut index = Vec::new();
    let replay = drive(session, frames, 1.0 / args.fps, |session, packet| {
        if let Some(list) = packet["textures"].as_array() {
            for texture in list {
                let Some(id) = texture["id"].as_str() else {
                    continue;
                };
                let width = texture["width"].as_u64().unwrap_or(0) as u32;
                let height = texture["height"].as_u64().unwrap_or(0) as u32;
                let Some(pixels) = session.copy_rgba(id) else {
                    continue;
                };
                if pixels.len() != width as usize * height as usize * 4 {
                    return Err(format!(
                        "texture {id} is {} bytes, expected {}x{} RGBA",
                        pixels.len(),
                        width,
                        height
                    ));
                }
                fs::write(texture_dir.join(format!("{id}.rgba")), &pixels).map_err(|e| e.to_string())?;
                let image = RgbaImage::from_raw(width, height, pixels.clone())
                    .ok_or_else(|| format!("texture {id} is not a raw image"))?;
                image
                    .save_with_format(texture_dir.join(format!("{id}.png")), ImageFormat::Png)
                    .map_err(|e| e.to_string())?;
                index.push(json!({
                    "id": id,
                    "width": width,
                    "height": height,
                    "rgba": format!("textures/{id}.rgba"),
                    "png": format!("textures/{id}.png"),
                }));
                session.release_texture(id);
            }
        }
        writeln!(packet_file, "{packet}").map_err(|e| e.to_string())?;
        Ok(())
    })?;
    fs::write(
        args.out.join("textures.json"),
        serde_json::to_vec_pretty(&index).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let meta = json!({
        "format": "clash-painter-fixture-v1",
        "lag_ticks": args.lag,
        "fps": args.fps,
        "dt_seconds": 1.0 / args.fps,
        "width": 540,
        "height": 900,
        "cards": args.cards,
        "snapshots": frames.len(),
        "packets": replay.packets.len(),
        "observe_rule": "observe snapshot i when i/20 <= wall",
        "step_rule": "painter_step(1/fps) after the due observes; wall then advances by 1/fps",
        "rgba": "width*height*4 bytes, row-major, straight alpha, matching the PNG",
        "packet": "one JSON object per line, the clash-web-scene field set, including render_ms",
        "release": "RGBA is released after it is written, in textures[] order, before the next step",
    });
    fs::write(
        args.out.join("meta.json"),
        serde_json::to_vec_pretty(&meta).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(replay.packets.len())
}

fn address_gate(
    session: &mut ExportSession,
    frames: &[Vec<u8>],
    gate_frames: usize,
    fps: f64,
) -> Result<Value, String> {
    let window: Vec<Vec<u8>> = frames.iter().take(gate_frames).cloned().collect();
    let parsed: Vec<BattleFrame> = window
        .iter()
        .map(|line| serde_json::from_slice(line).map_err(|e| format!("gate snapshot: {e}")))
        .collect::<Result<Vec<_>, String>>()?;
    let permutation = address_permutation(&parsed, 0xC0FFEE);
    let permuted = remap_lines(&window, &permutation, false)?;
    let mut shift = BTreeMap::new();
    for address in permutation.keys() {
        let next = address.wrapping_add(0x1_0000);
        if permutation.contains_key(&next) || shift.values().any(|value| *value == next) {
            return Err(format!("address shift collided at {address}"));
        }
        shift.insert(*address, next);
    }
    let shifted = remap_lines(&window, &shift, false)?;
    let with_ids = remap_lines(&window, &permutation, true)?;
    let original = pass(session, &window, fps)?;
    let shuffled = pass(session, &permuted, fps)?;
    let shifted_packets = pass(session, &shifted, fps)?;
    let id_packets = pass(session, &with_ids, fps)?;
    let random = packet_delta("random_permutation", &original, &shuffled);
    let order = packet_delta("order_preserving_shift", &original, &shifted_packets);
    let ids = packet_delta("permutation_including_entity_ids", &original, &id_packets);
    let unchanged = random["equal"].as_bool() == Some(true);
    Ok(json!({
        "frames": window.len(),
        "packets": original.len(),
        "seed": "0xC0FFEE",
        "ignored_fields": ["render_ms"],
        "random_permutation": random,
        "order_preserving_shift": order,
        "permutation_including_entity_ids": ids,
        "output_independent_of_address": unchanged,
        "gate": if unchanged {
            "output does not depend on Entity.address or native_focus_entity"
        } else {
            "output depends on Entity.address / native_focus_entity; W4 must not remap"
        },
    }))
}

fn pass(session: &mut ExportSession, frames: &[Vec<u8>], fps: f64) -> Result<Vec<Value>, String> {
    session.reset();
    session.clear_textures();
    Ok(drive(session, frames, 1.0 / fps, |_, _| Ok(()))?.packets)
}

fn remap_lines(frames: &[Vec<u8>], map: &BTreeMap<u64, u64>, ids: bool) -> Result<Vec<Vec<u8>>, String> {
    frames
        .iter()
        .map(|line| remap_snapshot(line, map, ids))
        .collect()
}

fn remap_snapshot(line: &[u8], map: &BTreeMap<u64, u64>, ids: bool) -> Result<Vec<u8>, String> {
    let mut value: Value = serde_json::from_slice(line).map_err(|e| format!("remap snapshot: {e}"))?;
    if let Some(entities) = value.get_mut("entities").and_then(Value::as_array_mut) {
        for entity in entities {
            apply_key(entity, "address", map);
            apply_key(entity, "native_focus_entity", map);
            if ids {
                apply_key(entity, "native_entity_id", map);
                apply_key(entity, "native_focus_entity_id", map);
            }
        }
    }
    serde_json::to_vec(&value).map_err(|e| e.to_string())
}

fn apply_key(entity: &mut Value, key: &str, map: &BTreeMap<u64, u64>) {
    let Some(number) = entity.get(key).and_then(Value::as_u64) else {
        return;
    };
    if let Some(next) = map.get(&number) {
        entity[key] = json!(*next);
    }
}

fn packet_delta(name: &str, left: &[Value], right: &[Value]) -> Value {
    if left.len() != right.len() {
        return json!({
            "name": name,
            "equal": false,
            "reason": format!("packet count {} != {}", left.len(), right.len()),
        });
    }
    for (index, (a, b)) in left.iter().zip(right).enumerate() {
        let mut a = a.clone();
        let mut b = b.clone();
        if let Some(object) = a.as_object_mut() {
            object.remove("render_ms");
        }
        if let Some(object) = b.as_object_mut() {
            object.remove("render_ms");
        }
        if let Err(reason) = json_close(&a, &b) {
            return json!({
                "name": name,
                "equal": false,
                "packet": index,
                "reason": reason,
            });
        }
    }
    json!({"name": name, "equal": true, "packets": left.len()})
}
