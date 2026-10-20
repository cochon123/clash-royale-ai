//! Isolated reference runner with optional fresh-deck coverage and state audits.
//! The active training executable is never rebuilt by this tool.
use clash_native_core::oracle::{NativeSession, PlayCardRequest};
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
    session: Option<NativeSession>,
    initial_tick: u64,
    max_ticks: u64,
    finished: bool,
    configured: Value,
    profile_path: Option<PathBuf>,
    profile_active: bool,
    profile_serial: u64,
}

impl Server {
    fn save_profile(&mut self) -> Result<(), String> {
        if !self.profile_active {
            return Ok(());
        }
        let profile = self
            .session
            .as_mut()
            .ok_or("session missing")?
            .finish_block_profile()?;
        let path = self.profile_path.as_ref().ok_or("profile path missing")?;
        std::fs::create_dir_all(path.parent().ok_or("profile parent missing")?)
            .map_err(|e| e.to_string())?;
        std::fs::write(
            path,
            serde_json::to_vec_pretty(&profile).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        self.profile_active = false;
        Ok(())
    }
    fn handle(&mut self, request: &Value) -> Result<Value, String> {
        match request["op"].as_str().ok_or("request needs op")? {
            "reset" => {
                self.save_profile()?;
                self.session = None;
                self.max_ticks = request["max_ticks"].as_u64().unwrap_or(7200);
                if !(1..=36000).contains(&self.max_ticks) {
                    return Err("max_ticks must be 1..36000".into());
                }
                let seed = request["seed"].as_u64().unwrap_or(1);
                let seed: u32 = seed
                    .try_into()
                    .map_err(|_| "seed must be in 0..=4294967295")?;
                let started = Instant::now();
                let mut session = NativeSession::load(&self.checkpoint)?;
                if let Some(root) = std::env::var_os("CLASH_DECK_PROFILE_DIR") {
                    self.profile_serial += 1;
                    let deck = request["deck_preset"].as_str().unwrap_or("unknown");
                    self.profile_path = Some(
                        PathBuf::from(root)
                            .join(format!("{deck}-{seed}-{}.json", self.profile_serial)),
                    );
                    session.start_block_profile()?;
                    self.profile_active = true;
                }
                self.configured = session.configure_normal_decks_and_rebuild(
                    preset(&request["deck_preset"])?,
                    preset(
                        request
                            .get("deck_preset_side1")
                            .unwrap_or(&request["deck_preset"]),
                    )?,
                    seed,
                )?;
                self.initial_tick = self.configured["observation"]["tick"]
                    .as_u64()
                    .ok_or("configured observation has no tick")?;
                let observation = self.configured["observation"].clone();
                self.session = Some(session);
                self.finished = false;
                Ok(
                    json!({"observation":observation,"configured_decks":self.configured["actual_native_decks"],"deck_source_rows":self.configured["deck_source_rows"],"seed":seed,"reset_seconds":started.elapsed().as_secs_f64(),"max_ticks":self.max_ticks,"engine_reset":"fresh original-native harness from loaded context; exact Normal row source pointers verified"}),
                )
            }
            "profile" => {
                self.save_profile()?;
                Ok(json!({"saved":self.profile_path}))
            }
            "audit" => {
                let path = PathBuf::from(request["out"].as_str().ok_or("audit needs out")?);
                self.session
                    .as_ref()
                    .ok_or("reset required")?
                    .audit_snapshot(&self.checkpoint, &path)?;
                Ok(json!({"saved":path}))
            }
            "observe" => {
                Ok(json!({"observation":self.session.as_mut().ok_or("reset required")?.observe()?}))
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
                    if session.is_terminal()?
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
                println!("deck-setup-probe --checkpoint DIR; JSONL reset requires deck_preset=hog26|pixel9_mortar, optional deck_preset_side1; step, observe, finish, close supported");
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
        profile_path: None,
        profile_active: false,
        profile_serial: 0,
    };
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        let parsed = serde_json::from_str::<Value>(&line);
        let close = parsed.as_ref().is_ok_and(|v| v["op"] == "close");
        let response = if close {
            server.save_profile()?;
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
