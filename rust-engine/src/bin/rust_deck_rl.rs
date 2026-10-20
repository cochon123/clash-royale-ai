//! Compiled Rust runner with the original Normal-row deck override.
use clash_recompiled::rl::{PlayCardRequest, RlSession};
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

fn preset(value: &Value) -> Result<&'static [&'static str; 8], String> {
    match value.as_str().ok_or("reset needs deck_preset")? {
        "hog26" => Ok(&HOG26),
        "mortar" | "pixel9_mortar" => Ok(&MORTAR),
        other => Err(format!(
            "unknown deck_preset {other:?}; use hog26 or pixel9_mortar"
        )),
    }
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
    session: Option<RlSession>,
    initial_tick: u64,
    max_ticks: u64,
    finished: bool,
    configured: Value,
}

impl Server {
    fn handle(&mut self, request: &Value) -> Result<Value, String> {
        match request["op"].as_str().ok_or("request needs op")? {
            "reset" => {
                if let Some(other) = request.get("deck_preset_side1") {
                    if preset(other)? != preset(&request["deck_preset"])? {
                        return Err("asymmetric decks are not yet covered by this compiled runner; use the original reference runner".into());
                    }
                }
                self.max_ticks = request["max_ticks"].as_u64().unwrap_or(7200);
                if !(1..=36000).contains(&self.max_ticks) {
                    return Err("max_ticks must be 1..36000".into());
                }
                let seed: u32 = request["seed"]
                    .as_u64()
                    .unwrap_or(1)
                    .try_into()
                    .map_err(|_| "seed must be in 0..=4294967295")?;
                let started = Instant::now();
                if let Some(session) = self.session.as_mut() {
                    session.reset()?;
                } else {
                    self.session = Some(RlSession::load(&self.checkpoint)?);
                }
                let session = self.session.as_mut().ok_or("session failed to load")?;
                self.configured = session
                    .configure_normal_deck_and_rebuild(preset(&request["deck_preset"])?, seed)?;
                let observation = self.configured["observation"].clone();
                self.initial_tick = observation["tick"]
                    .as_u64()
                    .ok_or("configured observation has no tick")?;
                self.finished = false;
                Ok(
                    json!({"observation":observation,"configured_decks":self.configured["actual_native_decks"],"deck_source_rows":self.configured["deck_source_rows"],"seed":seed,"reset_seconds":started.elapsed().as_secs_f64(),"max_ticks":self.max_ticks,"engine_reset":"fresh original-native harness from loaded context; exact Normal row source pointers verified"}),
                )
            }
            "observe" => {
                Ok(json!({"observation":self.session.as_mut().ok_or("reset required")?.observe()?}))
            }
            "audit" => {
                let path = PathBuf::from(request["out"].as_str().ok_or("audit needs out")?);
                self.session
                    .as_ref()
                    .ok_or("reset required")?
                    .engine
                    .audit(&path)?;
                Ok(json!({"saved":path}))
            }
            "finish" => {
                let session = self.session.as_mut().ok_or("reset required")?;
                let outcome = session.native_match_until_terminal()?;
                self.finished = true;
                Ok(
                    json!({"outcome":outcome,"verified_decks":self.configured["actual_native_decks"]}),
                )
            }
            "step" => {
                if self.finished {
                    return Err("episode finished; reset required".into());
                }
                let count = request["ticks"].as_u64().unwrap_or(20);
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
                let before = session.observe()?["tick"]
                    .as_u64()
                    .ok_or("missing native tick")?;
                let mut results = Vec::new();
                for action in actions {
                    results.push(session.play_card(action)?);
                }
                let mut advanced = 0;
                for _ in 0..count {
                    if session.is_terminal()
                        || before + advanced - self.initial_tick >= self.max_ticks
                    {
                        break;
                    }
                    session.world_step()?;
                    advanced += 1;
                }
                let observation = session.observe()?;
                let after = observation["tick"].as_u64().ok_or("missing native tick")?;
                let terminated = observation["terminated"]
                    .as_bool()
                    .ok_or("missing terminal flag")?;
                let truncated =
                    !terminated && after.saturating_sub(self.initial_tick) >= self.max_ticks;
                if after == before && !terminated && !truncated {
                    return Err("native world did not advance".into());
                }
                self.finished = terminated || truncated;
                Ok(
                    json!({"observation":observation,"actions":results,"terminated":terminated,"truncated":truncated,"ticks_advanced":after.saturating_sub(before),"step_seconds":started.elapsed().as_secs_f64()}),
                )
            }
            other => Err(format!("unknown operation {other}")),
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut checkpoint = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--checkpoint" => {
                checkpoint = Some(PathBuf::from(
                    args.next().ok_or("--checkpoint needs directory")?,
                ))
            }
            "--help" | "-h" => {
                println!("rust-deck-rl --checkpoint DIR; JSONL reset requires deck_preset=hog26|pixel9_mortar; step, observe, finish, close supported");
                return Ok(());
            }
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    let mut server = Server {
        checkpoint: checkpoint.ok_or("--checkpoint required")?,
        session: None,
        initial_tick: 0,
        max_ticks: 7200,
        finished: false,
        configured: Value::Null,
    };
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
        let response = response.unwrap_or_else(|error| {
            server.session = None;
            json!({"ok":false,"error":error,"reset_required":true})
        });
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
        eprintln!("rust-deck-rl: {error}");
        std::process::exit(2);
    }
}
