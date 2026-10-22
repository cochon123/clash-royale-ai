//! Compiled original-engine league protocol; asymmetric decks and native tower HP.
//! Uses private checkpoint reset and fails explicitly on uncovered code paths.
use clash_recompiled::rl::{RlSession, PlayCardRequest};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Instant;

fn deck<'a>(request: &'a Value, key: &str) -> Result<Vec<&'a str>, String> {
    let values = request[key].as_array().ok_or_else(|| format!("reset needs {key} array"))?;
    let names = values.iter().map(|v| v.as_str().ok_or_else(|| format!("{key} entries must be native card names"))).collect::<Result<Vec<_>, _>>()?;
    if names.len() != 8 { return Err(format!("{key} must contain exactly 8 cards")); }
    let mut unique = std::collections::BTreeSet::new();
    if names.iter().any(|name| name.is_empty() || !unique.insert(*name)) {
        return Err(format!("{key} names must be nonempty and unique"));
    }
    Ok(names)
}

fn action(value: &Value) -> Result<PlayCardRequest, String> {
    let side = value["side"].as_u64().filter(|v| *v < 2).ok_or("action side must be 0 or 1")?;
    let slot = value["slot"].as_u64().filter(|v| *v < 4).ok_or("action slot must be 0..3")?;
    let coordinate = |key| value[key].as_i64().and_then(|v| i32::try_from(v).ok()).ok_or_else(|| format!("action {key} must be i32"));
    Ok(PlayCardRequest { side: side as u32, slot: slot as u32, x: coordinate("x")?, y: coordinate("y")? })
}

#[derive(Clone)]
struct Tower { id: i32, side: i32, row: String, hp: i32, max_hp: i32 }

impl Tower {
    fn json(&self) -> Value {
        json!({"id":self.id,"side":self.side,"row":self.row,"hp":self.hp,"max_hp":self.max_hp})
    }
}

struct Server {
    checkpoint: PathBuf,
    session: Option<RlSession>,
    initial_tick: u64,
    max_ticks: u64,
    finished: bool,
    configured: Value,
    reward_state_enabled: bool,
    initial_towers: BTreeMap<(i32, i32), Tower>,
}

fn tower_state_from_frame(frame: &Value, initial: Option<&BTreeMap<(i32, i32), Tower>>) -> Result<BTreeMap<(i32, i32), Tower>, String> {
    let entities = frame["entities"].as_array().ok_or("presentation snapshot has no entities")?;
    let mut towers = BTreeMap::new();
    for entity in entities {
        let row = entity["row"].as_str().ok_or("presentation entity has no row")?;
        if row != "KingTower" && row != "PrincessTower" { continue; }
        let id = entity["native_entity_id"].as_i64().and_then(|v| i32::try_from(v).ok()).ok_or("tower has invalid native_entity_id")?;
        let side = entity["side"].as_i64().and_then(|v| i32::try_from(v).ok()).filter(|v| (0..=1).contains(v)).ok_or("tower has invalid side")?;
        let hp = entity["hp"].as_i64().and_then(|v| i32::try_from(v).ok()).filter(|v| *v >= 0).ok_or("tower has invalid hp")?;
        let max_hp = entity["max_hp"].as_i64().and_then(|v| i32::try_from(v).ok()).filter(|v| *v > 0).ok_or("tower has invalid max_hp")?;
        if hp > max_hp { return Err(format!("tower {id} hp exceeds max_hp")); }
        let tower = Tower { id, side, row: row.to_string(), hp, max_hp };
        if let Some(expected) = initial {
            let key = (side, id);
            let prior = expected.get(&key).ok_or_else(|| format!("unexpected tower identity side={side} id={id}"))?;
            if prior.row != tower.row || prior.max_hp != tower.max_hp {
                return Err(format!("tower identity changed row/max_hp side={side} id={id}"));
            }
        }
        if towers.insert((side, id), tower).is_some() { return Err(format!("duplicate tower identity side={side} id={id}")); }
    }
    Ok(towers)
}

fn validate_starting_towers(towers: &BTreeMap<(i32, i32), Tower>) -> Result<(), String> {
    if towers.len() != 6 { return Err(format!("expected 6 starting towers, found {}", towers.len())); }
    for side in 0..=1 {
        let rows: Vec<_> = towers.values().filter(|tower| tower.side == side).collect();
        if rows.len() != 3 || rows.iter().filter(|tower| tower.row == "KingTower").count() != 1 || rows.iter().filter(|tower| tower.row == "PrincessTower").count() != 2 {
            return Err(format!("side {side} does not have one king and two princess towers"));
        }
        if rows.iter().any(|tower| tower.hp <= 0 || tower.max_hp <= 0) { return Err(format!("side {side} starting tower has nonpositive HP")); }
    }
    Ok(())
}

fn reward_state(current: BTreeMap<(i32, i32), Tower>, initial: &BTreeMap<(i32, i32), Tower>) -> Value {
    let towers = initial.iter().map(|(key, prior)| {
        current.get(key).cloned().unwrap_or_else(|| Tower { hp: 0, ..prior.clone() }).json()
    }).collect::<Vec<_>>();
    json!({"towers":towers})
}

impl Server {
    fn handle(&mut self, req: &Value) -> Result<Value, String> {
        match req["op"].as_str().ok_or("request needs op")? {
            "reset" => {
                let d0 = deck(req, "deck0")?;
                let d1 = deck(req, "deck1")?;
                self.max_ticks = req["max_ticks"].as_u64().unwrap_or(7200);
                if !(1..=36000).contains(&self.max_ticks) { return Err("max_ticks must be 1..36000".into()); }
                let seed: u32 = req["seed"].as_u64().unwrap_or(1).try_into().map_err(|_| "seed must be in 0..=4294967295")?;
                let started = Instant::now();
                if let Some(session) = self.session.as_mut() {
                    session.reset()?;
                } else {
                    self.session = Some(RlSession::load(&self.checkpoint)?);
                }
                let session = self.session.as_mut().ok_or("session failed to load")?;
                self.configured = session.configure_normal_decks_and_rebuild(&d0, &d1, seed)?;
                self.initial_tick = self.configured["observation"]["tick"].as_u64().ok_or("configured observation has no tick")?;
                let observation = self.configured["observation"].clone();
                self.reward_state_enabled = req["reward_state"].as_bool().unwrap_or(false);
                self.initial_towers.clear();
                let initial_reward = if self.reward_state_enabled {
                    let frame = session.tower_snapshot()?;
                    self.initial_towers = tower_state_from_frame(&frame, None)?;
                    validate_starting_towers(&self.initial_towers)?;
                    Some(reward_state(self.initial_towers.clone(), &self.initial_towers))
                } else { None };
                self.finished = false;
                let mut response = json!({"observation":observation,"configured_decks":self.configured["actual_native_decks"],"deck_source_rows":self.configured["deck_source_rows"],"seed":seed,"reset_seconds":started.elapsed().as_secs_f64(),"max_ticks":self.max_ticks,"engine_reset":"fresh original-native harness from loaded context; exact Normal row source pointers verified"});
                if let Some(state) = initial_reward { response["reward_state"] = state; }
                Ok(response)
            }
            "observe" => {
                let session = self.session.as_mut().ok_or("reset required")?;
                let observation = session.observe()?;
                let mut response = json!({"observation":observation});
                if self.reward_state_enabled {
                    let frame = session.tower_snapshot()?;
                    let current = tower_state_from_frame(&frame, Some(&self.initial_towers))?;
                    response["reward_state"] = reward_state(current, &self.initial_towers);
                }
                Ok(response)
            }
            "reward_state" => {
                if !self.reward_state_enabled { return Err("reset with reward_state:true required".into()); }
                let session = self.session.as_mut().ok_or("reset required")?;
                let frame = session.tower_snapshot()?;
                let current = tower_state_from_frame(&frame, Some(&self.initial_towers))?;
                Ok(reward_state(current, &self.initial_towers))
            }
            "placement_mask" => {
                let side = req["side"].as_u64().filter(|v| *v < 2).ok_or("invalid placement side")? as u32;
                let slot = req["slot"].as_u64().filter(|v| *v < 4).ok_or("invalid placement slot")? as u32;
                self.session.as_mut().ok_or("reset required")?.placement_mask(side, slot)
            }
            "audit" => {
                let path = PathBuf::from(req["out"].as_str().ok_or("audit needs out")?);
                self.session.as_ref().ok_or("reset required")?.engine.audit(&path)?;
                Ok(json!({"saved":path}))
            }
            "finish" => {
                let session = self.session.as_mut().ok_or("reset required")?;
                let outcome = session.native_match_until_terminal()?;
                self.finished = true;
                Ok(json!({"outcome":outcome,"verified_decks":self.configured["actual_native_decks"]}))
            }
            "step" => {
                if self.finished { return Err("episode finished; reset required".into()); }
                let count = req["ticks"].as_u64().unwrap_or(20);
                if !(1..=200).contains(&count) { return Err("ticks must be 1..200".into()); }
                let values = req["actions"].as_array().ok_or("step needs actions array (empty is wait)")?;
                if values.len() > 2 { return Err("at most one action per side per step".into()); }
                let actions = values.iter().map(action).collect::<Result<Vec<_>, _>>()?;
                if actions.len() == 2 && actions[0].side == actions[1].side { return Err("duplicate action side".into()); }
                let session = self.session.as_mut().ok_or("reset required")?;
                let started = Instant::now();
                let before = session.observe()?["tick"].as_u64().ok_or("missing native tick")?;
                let mut results = Vec::new();
                for a in actions { results.push(session.play_card(a)?); }
                let mut advanced = 0u64;
                for _ in 0..count {
                    if session.is_terminal() || before.saturating_add(advanced).saturating_sub(self.initial_tick) >= self.max_ticks { break; }
                    session.world_step()?;
                    advanced += 1;
                }
                let observation = session.observe()?;
                let after = observation["tick"].as_u64().ok_or("missing native tick")?;
                let terminated = observation["terminated"].as_bool().ok_or("missing terminal flag")?;
                let truncated = !terminated && after.saturating_sub(self.initial_tick) >= self.max_ticks;
                if after == before && !terminated && !truncated { return Err("native world did not advance".into()); }
                self.finished = terminated || truncated;
                let mut response = json!({"observation":observation,"actions":results,"terminated":terminated,"truncated":truncated,"ticks_advanced":after.saturating_sub(before),"step_seconds":started.elapsed().as_secs_f64()});
                if self.reward_state_enabled {
                    let frame = session.tower_snapshot()?;
                    let current = tower_state_from_frame(&frame, Some(&self.initial_towers))?;
                    response["reward_state"] = reward_state(current, &self.initial_towers);
                }
                Ok(response)
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
            "--checkpoint" => checkpoint = Some(PathBuf::from(args.next().ok_or("--checkpoint needs directory")?)),
            "--help" | "-h" => { println!("rust-league-rl --checkpoint DIR; JSONL reset: {{\"op\":\"reset\",\"deck0\":[8 native card names],\"deck1\":[8 native card names],\"seed\":1,\"max_ticks\":7200}}; then step/observe/finish/close"); return Ok(()); }
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    let mut server = Server { checkpoint: checkpoint.ok_or("--checkpoint required")?, session: None, initial_tick: 0, max_ticks: 7200, finished: false, configured: Value::Null, reward_state_enabled: false, initial_towers: BTreeMap::new() };
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let parsed = serde_json::from_str::<Value>(&line.map_err(|e| e.to_string())?);
        let close = parsed.as_ref().is_ok_and(|v| v["op"] == "close");
        let response = if close { Ok(json!({})) } else { parsed.map_err(|e| e.to_string()).and_then(|req| server.handle(&req)) };
        let response = response.map(|result| json!({"ok":true,"result":result})).unwrap_or_else(|error| json!({"ok":false,"error":error,"reset_required":true}));
        serde_json::to_writer(&mut stdout, &response).map_err(|e| e.to_string())?;
        stdout.write_all(b"\n").map_err(|e| e.to_string())?;
        stdout.flush().map_err(|e| e.to_string())?;
        if close { break; }
    }
    Ok(())
}

fn main() { if let Err(error) = run() { eprintln!("{error}"); std::process::exit(2); } }
