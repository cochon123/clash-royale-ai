//! Read-only entity snapshots and original-native actions over JSON lines.
use clash_native_core::oracle::{NativeSession, PlayCardRequest, PristineTemplate};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::time::Instant;

const HOG26: [&str; 8] = [
    "HogRider",
    "Musketeer",
    "IceGolemite",
    "Skeletons",
    "IceSpirits",
    "Cannon",
    "Fireball",
    "Log",
];
const MORTAR: [&str; 8] = [
    "Mortar",
    "Knight",
    "Archer",
    "Skeletons",
    "BarbLog",
    "Arrows",
    "MiniPekka",
    "Rocket",
];
const LOG_BAIT: [&str; 8] = [
    "GoblinBarrel",
    "Princess",
    "Knight",
    "GoblinGang",
    "InfernoTower",
    "Rocket",
    "Log",
    "IceSpirits",
];

fn preset(value: &Value) -> Result<&'static [&'static str; 8], String> {
    match value.as_str().ok_or("reset needs deck_preset")? {
        "hog26" => Ok(&HOG26),
        "mortar" | "pixel9_mortar" => Ok(&MORTAR),
        "log-bait" | "logbait" => Ok(&LOG_BAIT),
        other => Err(format!(
            "unknown deck_preset {other:?}; use hog26, pixel9_mortar or log-bait"
        )),
    }
}

/// A side's deck: an explicit `deck0`/`deck1` array of eight native card
/// names, or a named preset (`deck_preset`, `deck_preset_side1`).
fn side_deck(request: &Value, side: usize) -> Result<Vec<String>, String> {
    let key = if side == 0 { "deck0" } else { "deck1" };
    if let Some(values) = request.get(key).filter(|v| !v.is_null()) {
        let names = values
            .as_array()
            .ok_or_else(|| format!("{key} must be an array of native card names"))?
            .iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| format!("{key} entries must be strings"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(names);
    }
    let named = if side == 0 {
        &request["deck_preset"]
    } else {
        request
            .get("deck_preset_side1")
            .unwrap_or(&request["deck_preset"])
    };
    Ok(preset(named)?.iter().map(|name| name.to_string()).collect())
}

fn action(value: &Value) -> Result<PlayCardRequest, String> {
    let side = value["side"]
        .as_u64()
        .filter(|v| *v < 2)
        .ok_or("action side must be 0 or 1")?;
    let slot = value["slot"]
        .as_u64()
        .filter(|v| *v < 4)
        .ok_or("action slot must be 0..3")?;
    let coordinate = |key| {
        value[key]
            .as_i64()
            .and_then(|v| i32::try_from(v).ok())
            .ok_or_else(|| format!("action {key} must be i32"))
    };
    Ok(PlayCardRequest {
        side: side as u32,
        slot: slot as u32,
        x: coordinate("x")?,
        y: coordinate("y")?,
    })
}

struct Server {
    checkpoint: PathBuf,
    /// Verified once per process. Later resets rebuild a session from it.
    template: Option<PristineTemplate>,
    load_seconds: f64,
    resets: u64,
    session: Option<NativeSession>,
    initial_tick: u64,
    max_ticks: u64,
    finished: bool,
    configured: Value,
}

impl Server {
    fn ensure_template(&mut self) -> Result<(), String> {
        if self.template.is_some() {
            return Ok(());
        }
        let started = Instant::now();
        let template = PristineTemplate::load_verified(&self.checkpoint)?;
        self.load_seconds = started.elapsed().as_secs_f64();
        self.template = Some(template);
        Ok(())
    }

    fn handle(&mut self, request: &Value) -> Result<Value, String> {
        match request["op"].as_str().ok_or("request needs op")? {
            "ping" => Ok(json!({
                "ready": self.template.is_some(),
                "load_seconds": self.load_seconds,
                "resets": self.resets,
            })),
            "reset" => {
                self.max_ticks = request["max_ticks"].as_u64().unwrap_or(7200);
                if !(1..=36000).contains(&self.max_ticks) {
                    return Err("max_ticks must be 1..36000".into());
                }
                let seed = request["seed"].as_u64().unwrap_or(1);
                let seed: u32 = seed
                    .try_into()
                    .map_err(|_| "seed must be in 0..=4294967295")?;
                let started = Instant::now();
                let deck0 = side_deck(request, 0)?;
                let deck1 = side_deck(request, 1)?;
                let deck0: Vec<&str> = deck0.iter().map(String::as_str).collect();
                let deck1: Vec<&str> = deck1.iter().map(String::as_str).collect();
                // A failed first verification leaves no template, so the reset aborts.
                self.ensure_template()?;
                self.template
                    .as_ref()
                    .ok_or("checkpoint template missing")?
                    .identity_unchanged()?;
                let mut session = {
                    let template = self
                        .template
                        .as_ref()
                        .ok_or("checkpoint template missing")?;
                    NativeSession::from_template(template)?
                };
                self.configured =
                    session.configure_normal_decks_and_rebuild(&deck0, &deck1, seed)?;
                self.initial_tick = self.configured["observation"]["tick"]
                    .as_u64()
                    .ok_or("configured observation missing tick")?;
                let frame = session.presentation_snapshot()?;
                self.session = Some(session);
                self.finished = false;
                self.resets += 1;
                Ok(json!({
                    "observation":self.configured["observation"],
                    "frame":frame,
                    "configured_decks":self.configured["actual_native_decks"],
                    "deck_source_rows":self.configured["deck_source_rows"],
                    "seed":seed,
                    "reset_seconds":started.elapsed().as_secs_f64(),
                    "max_ticks":self.max_ticks,
                    "engine_reset":"fresh original-native harness from loaded context; deck rows verified",
                }))
            }
            "observe" => {
                let session = self.session.as_mut().ok_or("reset required")?;
                Ok(
                    json!({"frame":session.presentation_snapshot()?,"observation":session.observe()?}),
                )
            }
            "placement_mask" => {
                let side = request["side"]
                    .as_u64()
                    .filter(|v| *v < 2)
                    .ok_or("invalid placement side")? as u32;
                let slot = request["slot"]
                    .as_u64()
                    .filter(|v| *v < 4)
                    .ok_or("invalid placement slot")? as u32;
                self.session
                    .as_mut()
                    .ok_or("reset required")?
                    .placement_mask(side, slot)
            }
            "placement_tiles" => {
                let side = request["side"]
                    .as_u64()
                    .filter(|v| *v < 2)
                    .ok_or("invalid placement side")? as u32;
                let slot = request["slot"]
                    .as_u64()
                    .filter(|v| *v < 4)
                    .ok_or("invalid placement slot")? as u32;
                self.session
                    .as_mut()
                    .ok_or("reset required")?
                    .placement_tiles(side, slot)
            }
            "validate_target" => self
                .session
                .as_mut()
                .ok_or("reset required")?
                .validate_target(action(request)?),
            "resolve_target" => self
                .session
                .as_mut()
                .ok_or("reset required")?
                .resolve_target(action(request)?),
            "inspect_components" => {
                let session = self.session.as_mut().ok_or("reset required")?;
                Ok(json!({"components":session.presentation_component_vtables()?}))
            }
            "step" => {
                if self.finished {
                    return Err("episode finished; reset required".into());
                }
                let count = request["ticks"].as_u64().unwrap_or(1);
                if !(1..=200).contains(&count) {
                    return Err("ticks must be 1..200".into());
                }
                let values = request["actions"]
                    .as_array()
                    .ok_or("step needs actions array (empty is wait)")?;
                if values.len() > 2 {
                    return Err("at most one action per side per step".into());
                }
                let actions = values.iter().map(action).collect::<Result<Vec<_>, _>>()?;
                if actions.len() == 2 && actions[0].side == actions[1].side {
                    return Err("duplicate action side".into());
                }
                let session = self.session.as_mut().ok_or("reset required")?;
                let started = Instant::now();
                let before = session.presentation_snapshot()?["tick"]
                    .as_u64()
                    .ok_or("frame tick is not an unsigned integer")?;
                let mut results = Vec::new();
                // Raw submission is retained only for native differential probes.
                // Live harnesses need the resolved target returned by default.
                let raw_targets = request["raw_targets"].as_bool().unwrap_or(false);
                for action in actions {
                    results.push(if raw_targets {
                        session.play_card(action)?
                    } else {
                        session.play_card_resolved(action)?
                    });
                }
                let mut advanced = 0u64;
                for _ in 0..count {
                    if session.is_terminal()?
                        || before
                            .saturating_add(advanced)
                            .saturating_sub(self.initial_tick)
                            >= self.max_ticks
                    {
                        break;
                    }
                    session.world_step()?;
                    advanced += 1;
                }
                let frame = session.presentation_snapshot()?;
                let after = frame["tick"]
                    .as_u64()
                    .ok_or("frame tick is not an unsigned integer")?;
                let terminated = frame["terminated"]
                    .as_bool()
                    .ok_or("frame terminal flag missing")?;
                let truncated =
                    !terminated && after.saturating_sub(self.initial_tick) >= self.max_ticks;
                if after == before && !terminated && !truncated {
                    return Err("native world did not advance".into());
                }
                self.finished = terminated || truncated;
                let observation = session.observe()?;
                Ok(
                    json!({"frame":frame,"observation":observation,"actions":results,"terminated":terminated,"truncated":truncated,"ticks_advanced":after.saturating_sub(before),"step_seconds":started.elapsed().as_secs_f64()}),
                )
            }
            other => Err(format!("unknown operation {other}")),
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut checkpoint = None;
    let mut preload = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--checkpoint" => {
                checkpoint = Some(PathBuf::from(
                    args.next().ok_or("--checkpoint needs directory")?,
                ))
            }
            "--preload" => preload = true,
            "--help" | "-h" => {
                println!("native-watch --checkpoint DIR [--preload]; JSONL ping, reset, observe, step, close; --preload verifies the checkpoint before stdin and later resets rebuild from that in-memory template; reset takes deck0/deck1 arrays of 8 native card names or deck_preset=hog26|pixel9_mortar|log-bait with optional deck_preset_side1, and accepts seed");
                return Ok(());
            }
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    let mut server = Server {
        checkpoint: checkpoint.ok_or("--checkpoint required")?,
        template: None,
        load_seconds: 0.0,
        resets: 0,
        session: None,
        initial_tick: 0,
        max_ticks: 7200,
        finished: false,
        configured: Value::Null,
    };
    if preload {
        server.ensure_template()?;
    }
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        let parsed = serde_json::from_str::<Value>(&line);
        let close = parsed.as_ref().is_ok_and(|v| v["op"] == "close");
        let response = if close {
            Ok(json!({"ok":true,"result":{}}))
        } else {
            parsed
                .map_err(|e| e.to_string())
                .and_then(|request| server.handle(&request))
                .map(|result| json!({"ok":true,"result":result}))
        };
        let response = response
            .unwrap_or_else(|error| json!({"ok":false,"error":error,"reset_required":true}));
        serde_json::to_writer(&mut stdout, &response).map_err(|e| e.to_string())?;
        stdout.write_all(b"\n").map_err(|e| e.to_string())?;
        stdout.flush().map_err(|e| e.to_string())?;
        if close {
            break;
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
