//! Compiled original-engine league protocol; asymmetric decks and native tower HP.
//! Uses private checkpoint reset and fails explicitly on uncovered code paths.
use clash_recompiled::rl::{BoardMemory, PlayCardRequest, RlSession};
use clash_recompiled::snapshot::EngineSnapshot;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Session::load has already converted this JSON map into GuestHeap's owned
/// allocation table. Reset, calls and snapshots use that table, never the
/// duplicate JSON map. Keep the immutable checkpoint file and its identity.
fn load_session(checkpoint: &Path) -> Result<RlSession, String> {
    let mut session = RlSession::load(checkpoint)?;
    if std::env::var("CR_TRIM_CHECKPOINT_JSON").as_deref() == Ok("1") {
        if let Some(heap) = session.engine.checkpoint.get_mut("native_heap").and_then(Value::as_object_mut) {
            heap.remove("allocations");
        }
        #[cfg(target_os = "linux")]
        unsafe {
            unsafe extern "C" { fn malloc_trim(pad: usize) -> i32; }
            malloc_trim(0);
        }
    }
    Ok(session)
}

fn deck<'a>(request: &'a Value, key: &str) -> Result<Vec<&'a str>, String> {
    let values = request[key]
        .as_array()
        .ok_or_else(|| format!("reset needs {key} array"))?;
    let names = values
        .iter()
        .map(|v| {
            v.as_str()
                .ok_or_else(|| format!("{key} entries must be native card names"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if names.len() != 8 {
        return Err(format!("{key} must contain exactly 8 cards"));
    }
    let mut unique = std::collections::BTreeSet::new();
    if names
        .iter()
        .any(|name| name.is_empty() || !unique.insert(*name))
    {
        return Err(format!("{key} names must be nonempty and unique"));
    }
    Ok(names)
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

#[derive(Clone)]
struct Tower {
    id: i32,
    side: i32,
    row: String,
    hp: i32,
    max_hp: i32,
}

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
    board_enabled: bool,
    board: BoardMemory,
    snapshots: BTreeMap<u64, Arc<EngineSnapshot>>,
    next_snapshot: u64,
    max_snapshots: usize,
    /// Worker children ignore a routed `session` field; a lone process rejects
    /// any session other than 0.
    worker: bool,
}

fn tower_state_from_frame(
    frame: &Value,
    initial: Option<&BTreeMap<(i32, i32), Tower>>,
) -> Result<BTreeMap<(i32, i32), Tower>, String> {
    let entities = frame["entities"]
        .as_array()
        .ok_or("presentation snapshot has no entities")?;
    let mut towers = BTreeMap::new();
    for entity in entities {
        let row = entity["row"]
            .as_str()
            .ok_or("presentation entity has no row")?;
        if row != "KingTower" && row != "PrincessTower" {
            continue;
        }
        let id = entity["native_entity_id"]
            .as_i64()
            .and_then(|v| i32::try_from(v).ok())
            .ok_or("tower has invalid native_entity_id")?;
        let side = entity["side"]
            .as_i64()
            .and_then(|v| i32::try_from(v).ok())
            .filter(|v| (0..=1).contains(v))
            .ok_or("tower has invalid side")?;
        let hp = entity["hp"]
            .as_i64()
            .and_then(|v| i32::try_from(v).ok())
            .filter(|v| *v >= 0)
            .ok_or("tower has invalid hp")?;
        let max_hp = entity["max_hp"]
            .as_i64()
            .and_then(|v| i32::try_from(v).ok())
            .filter(|v| *v > 0)
            .ok_or("tower has invalid max_hp")?;
        if hp > max_hp {
            return Err(format!("tower {id} hp exceeds max_hp"));
        }
        let tower = Tower {
            id,
            side,
            row: row.to_string(),
            hp,
            max_hp,
        };
        if let Some(expected) = initial {
            let key = (side, id);
            let prior = expected
                .get(&key)
                .ok_or_else(|| format!("unexpected tower identity side={side} id={id}"))?;
            if prior.row != tower.row || prior.max_hp != tower.max_hp {
                return Err(format!(
                    "tower identity changed row/max_hp side={side} id={id}"
                ));
            }
        }
        if towers.insert((side, id), tower).is_some() {
            return Err(format!("duplicate tower identity side={side} id={id}"));
        }
    }
    Ok(towers)
}

fn validate_starting_towers(towers: &BTreeMap<(i32, i32), Tower>) -> Result<(), String> {
    if towers.len() != 6 {
        return Err(format!(
            "expected 6 starting towers, found {}",
            towers.len()
        ));
    }
    for side in 0..=1 {
        let rows: Vec<_> = towers.values().filter(|tower| tower.side == side).collect();
        if rows.len() != 3
            || rows.iter().filter(|tower| tower.row == "KingTower").count() != 1
            || rows
                .iter()
                .filter(|tower| tower.row == "PrincessTower")
                .count()
                != 2
        {
            return Err(format!(
                "side {side} does not have one king and two princess towers"
            ));
        }
        if rows.iter().any(|tower| tower.hp <= 0 || tower.max_hp <= 0) {
            return Err(format!("side {side} starting tower has nonpositive HP"));
        }
    }
    Ok(())
}

fn reward_state(
    current: BTreeMap<(i32, i32), Tower>,
    initial: &BTreeMap<(i32, i32), Tower>,
) -> Value {
    let towers = initial
        .iter()
        .map(|(key, prior)| {
            current
                .get(key)
                .cloned()
                .unwrap_or_else(|| Tower {
                    hp: 0,
                    ..prior.clone()
                })
                .json()
        })
        .collect::<Vec<_>>();
    json!({"towers":towers})
}

fn tower_entities(board: &Value) -> Value {
    let entities = board["entities"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|entity| {
            matches!(
                entity["row"].as_str(),
                Some("KingTower") | Some("PrincessTower")
            )
        })
        .map(|entity| {
            json!({
                "native_entity_id": entity["id"],
                "row": entity["row"],
                "side": entity["side"],
                "hp": entity["hp"],
                "max_hp": entity["max_hp"],
            })
        })
        .collect::<Vec<_>>();
    json!({"entities": entities})
}

impl Server {
    fn check_session(&self, req: &Value) -> Result<(), String> {
        if self.worker || req["op"] == "step_many" {
            return Ok(());
        }
        if let Some(id) = req["session"].as_u64() {
            if id != 0 {
                return Err(format!("session {id} is outside 0..1"));
            }
        }
        Ok(())
    }

    fn board_value(&mut self) -> Result<Value, String> {
        let Server { session, board, .. } = self;
        session
            .as_ref()
            .ok_or("reset required")?
            .board_snapshot(board)
    }

    fn reward_from_board(&self, board: &Value) -> Result<Value, String> {
        let current = tower_state_from_frame(&tower_entities(board), Some(&self.initial_towers))?;
        Ok(reward_state(current, &self.initial_towers))
    }

    fn protocol_value(&self) -> Value {
        json!({
            "initial_tick": self.initial_tick,
            "max_ticks": self.max_ticks,
            "finished": self.finished,
            "configured": self.configured,
            "reward_state_enabled": self.reward_state_enabled,
            "board_enabled": self.board_enabled,
            "initial_towers": self.initial_towers.values().map(|tower| tower.json()).collect::<Vec<_>>(),
            "positions": self.board.positions.iter().map(|(address, (x, y))| json!([address.to_string(), x, y])).collect::<Vec<_>>(),
            "first_seen": self.board.first_seen.iter().map(|(address, tick)| json!([address.to_string(), tick])).collect::<Vec<_>>(),
        })
    }

    fn apply_protocol(&mut self, value: &Value) -> Result<(), String> {
        self.initial_tick = value["initial_tick"]
            .as_u64()
            .ok_or("snapshot protocol missing initial_tick")?;
        self.max_ticks = value["max_ticks"]
            .as_u64()
            .ok_or("snapshot protocol missing max_ticks")?;
        self.finished = value["finished"]
            .as_bool()
            .ok_or("snapshot protocol missing finished")?;
        self.configured = value["configured"].clone();
        self.reward_state_enabled = value["reward_state_enabled"]
            .as_bool()
            .ok_or("snapshot protocol missing reward_state_enabled")?;
        self.board_enabled = value["board_enabled"]
            .as_bool()
            .ok_or("snapshot protocol missing board_enabled")?;
        let mut towers = BTreeMap::new();
        for tower in value["initial_towers"]
            .as_array()
            .ok_or("snapshot protocol missing initial_towers")?
        {
            let id = tower["id"]
                .as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .ok_or("snapshot tower id")?;
            let side = tower["side"]
                .as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .ok_or("snapshot tower side")?;
            let row = tower["row"]
                .as_str()
                .ok_or("snapshot tower row")?
                .to_string();
            let hp = tower["hp"]
                .as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .ok_or("snapshot tower hp")?;
            let max_hp = tower["max_hp"]
                .as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .ok_or("snapshot tower max_hp")?;
            towers.insert(
                (side, id),
                Tower {
                    id,
                    side,
                    row,
                    hp,
                    max_hp,
                },
            );
        }
        self.initial_towers = towers;
        let mut positions = BTreeMap::new();
        for entry in value["positions"]
            .as_array()
            .ok_or("snapshot protocol missing positions")?
        {
            let address = entry[0]
                .as_str()
                .ok_or("snapshot position address")?
                .parse::<u64>()
                .map_err(|e| e.to_string())?;
            let x = entry[1]
                .as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .ok_or("snapshot position x")?;
            let y = entry[2]
                .as_i64()
                .and_then(|v| i32::try_from(v).ok())
                .ok_or("snapshot position y")?;
            positions.insert(address, (x, y));
        }
        let mut first_seen = BTreeMap::new();
        for entry in value["first_seen"]
            .as_array()
            .ok_or("snapshot protocol missing first_seen")?
        {
            let address = entry[0]
                .as_str()
                .ok_or("snapshot first_seen address")?
                .parse::<u64>()
                .map_err(|e| e.to_string())?;
            let tick = entry[1]
                .as_u64()
                .and_then(|v| u32::try_from(v).ok())
                .ok_or("snapshot first_seen tick")?;
            first_seen.insert(address, tick);
        }
        self.board = BoardMemory {
            positions,
            first_seen,
        };
        Ok(())
    }

    fn install(&mut self, image: &EngineSnapshot) -> Result<(), String> {
        if self.session.is_none() {
            self.session = Some(load_session(&self.checkpoint)?);
        }
        self.session
            .as_mut()
            .ok_or("session failed to load")?
            .restore_snapshot(image)?;
        self.apply_protocol(&image.protocol)
    }

    fn store(&mut self, image: EngineSnapshot) -> Result<(u64, u64, u64), String> {
        if self.snapshots.len() >= self.max_snapshots {
            return Err(format!(
                "snapshot limit {} reached; drop_snapshot required",
                self.max_snapshots
            ));
        }
        let id = self.next_snapshot;
        self.next_snapshot += 1;
        let tick = image.tick;
        let bytes = image.bytes;
        self.snapshots.insert(id, Arc::new(image));
        Ok((id, tick, bytes))
    }

    fn capture(&self) -> Result<EngineSnapshot, String> {
        let protocol = self.protocol_value();
        self.session
            .as_ref()
            .ok_or("reset required")?
            .capture_snapshot(protocol)
    }

    /// Same JSON as `observe`, then roll the session back so the read's native
    /// calls do not change a restored branch.
    fn observe_preserving(&mut self) -> Result<Value, String> {
        let probe = self.capture()?;
        let reply = self.reply_observe();
        self.install(&probe)?;
        reply
    }

    fn reply_observe(&mut self) -> Result<Value, String> {
        let observation = {
            let session = self.session.as_mut().ok_or("reset required")?;
            session.observe()?
        };
        let mut response = json!({"observation": observation});
        if self.reward_state_enabled && !self.board_enabled {
            let frame = self
                .session
                .as_mut()
                .ok_or("reset required")?
                .tower_snapshot()?;
            let current = tower_state_from_frame(&frame, Some(&self.initial_towers))?;
            response["reward_state"] = reward_state(current, &self.initial_towers);
        }
        if self.board_enabled {
            let board = self.board_value()?;
            if self.reward_state_enabled {
                response["reward_state"] = self.reward_from_board(&board)?;
            }
            response["board"] = board;
        }
        Ok(response)
    }

    fn handle(&mut self, req: &Value) -> Result<Value, String> {
        self.check_session(req)?;
        match req["op"].as_str().ok_or("request needs op")? {
            "reset" => {
                let d0 = deck(req, "deck0")?;
                let d1 = deck(req, "deck1")?;
                self.max_ticks = req["max_ticks"].as_u64().unwrap_or(7200);
                if !(1..=36000).contains(&self.max_ticks) {
                    return Err("max_ticks must be 1..36000".into());
                }
                let seed: u32 = req["seed"]
                    .as_u64()
                    .unwrap_or(1)
                    .try_into()
                    .map_err(|_| "seed must be in 0..=4294967295")?;
                let started = Instant::now();
                if let Some(session) = self.session.as_mut() {
                    session.reset()?;
                } else {
                    self.session = Some(load_session(&self.checkpoint)?);
                }
                let session = self.session.as_mut().ok_or("session failed to load")?;
                self.configured = session.configure_normal_decks_and_rebuild(&d0, &d1, seed)?;
                self.initial_tick = self.configured["observation"]["tick"]
                    .as_u64()
                    .ok_or("configured observation has no tick")?;
                let observation = self.configured["observation"].clone();
                self.reward_state_enabled = req["reward_state"].as_bool().unwrap_or(false);
                self.board_enabled = req["board"].as_bool().unwrap_or(false);
                self.board = BoardMemory::default();
                self.initial_towers.clear();
                let initial_reward = if self.reward_state_enabled && !self.board_enabled {
                    let frame = session.tower_snapshot()?;
                    self.initial_towers = tower_state_from_frame(&frame, None)?;
                    validate_starting_towers(&self.initial_towers)?;
                    Some(reward_state(
                        self.initial_towers.clone(),
                        &self.initial_towers,
                    ))
                } else {
                    None
                };
                self.finished = false;
                let mut response = json!({"observation":observation,"configured_decks":self.configured["actual_native_decks"],"deck_source_rows":self.configured["deck_source_rows"],"seed":seed,"reset_seconds":started.elapsed().as_secs_f64(),"max_ticks":self.max_ticks,"engine_reset":"fresh original-native harness from loaded context; exact Normal row source pointers verified"});
                if let Some(state) = initial_reward {
                    response["reward_state"] = state;
                }
                if self.board_enabled {
                    let board = self.board_value()?;
                    if self.reward_state_enabled {
                        self.initial_towers =
                            tower_state_from_frame(&tower_entities(&board), None)?;
                        validate_starting_towers(&self.initial_towers)?;
                        response["reward_state"] =
                            reward_state(self.initial_towers.clone(), &self.initial_towers);
                    }
                    response["board"] = board;
                }
                Ok(response)
            }
            "observe" => self.reply_observe(),
            "reward_state" => {
                if !self.reward_state_enabled {
                    return Err("reset with reward_state:true required".into());
                }
                let session = self.session.as_mut().ok_or("reset required")?;
                let frame = session.tower_snapshot()?;
                let current = tower_state_from_frame(&frame, Some(&self.initial_towers))?;
                Ok(reward_state(current, &self.initial_towers))
            }
            "placement_mask" => {
                let side = req["side"]
                    .as_u64()
                    .filter(|v| *v < 2)
                    .ok_or("invalid placement side")? as u32;
                let slot = req["slot"]
                    .as_u64()
                    .filter(|v| *v < 4)
                    .ok_or("invalid placement slot")? as u32;
                self.session
                    .as_mut()
                    .ok_or("reset required")?
                    .placement_mask(side, slot)
            }
            "audit" => {
                let path = PathBuf::from(req["out"].as_str().ok_or("audit needs out")?);
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
                let count = req["ticks"].as_u64().unwrap_or(20);
                if !(1..=200).contains(&count) {
                    return Err("ticks must be 1..200".into());
                }
                let values = req["actions"]
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
                for a in actions {
                    results.push(session.play_card(a)?);
                }
                let mut advanced = 0u64;
                for _ in 0..count {
                    if session.is_terminal()
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
                let mut response = json!({"observation":observation,"actions":results,"terminated":terminated,"truncated":truncated,"ticks_advanced":after.saturating_sub(before),"step_seconds":started.elapsed().as_secs_f64()});
                if self.reward_state_enabled && !self.board_enabled {
                    let frame = session.tower_snapshot()?;
                    let current = tower_state_from_frame(&frame, Some(&self.initial_towers))?;
                    response["reward_state"] = reward_state(current, &self.initial_towers);
                }
                if self.board_enabled {
                    let board = self.board_value()?;
                    if self.reward_state_enabled {
                        response["reward_state"] = self.reward_from_board(&board)?;
                    }
                    response["board"] = board;
                }
                Ok(response)
            }
            "step_many" => {
                let items = req["items"]
                    .as_array()
                    .ok_or("step_many needs items array")?;
                let mut replies = Vec::with_capacity(items.len());
                for item in items {
                    let session = item["session"].as_u64().unwrap_or(0);
                    if session != 0 {
                        return Err(format!("session {session} is outside 0..1"));
                    }
                    let mut step = item.clone();
                    let object = step
                        .as_object_mut()
                        .ok_or("step_many item must be an object")?;
                    object.insert("op".into(), json!("step"));
                    object.remove("session");
                    replies.push(self.handle(&step)?);
                }
                Ok(json!({"items": replies}))
            }
            "snapshot" => {
                let image = self.capture()?;
                let (id, tick, bytes) = self.store(image)?;
                Ok(json!({"snapshot_id": id, "tick": tick, "bytes": bytes}))
            }
            "restore" => {
                let id = req["snapshot_id"]
                    .as_u64()
                    .ok_or("restore needs snapshot_id")?;
                let image = Arc::clone(
                    self.snapshots
                        .get(&id)
                        .ok_or("unknown snapshot_id")?,
                );
                self.install(&image)?;
                self.observe_preserving()
            }
            "drop_snapshot" => {
                let id = req["snapshot_id"]
                    .as_u64()
                    .ok_or("drop_snapshot needs snapshot_id")?;
                if self.snapshots.remove(&id).is_none() {
                    return Err("unknown snapshot_id".into());
                }
                Ok(json!({"snapshot_id": id}))
            }
            "save_snapshot" => {
                let id = req["snapshot_id"]
                    .as_u64()
                    .ok_or("save_snapshot needs snapshot_id")?;
                let path = PathBuf::from(
                    req["path"]
                        .as_str()
                        .ok_or("save_snapshot needs path")?,
                );
                let image = self
                    .snapshots
                    .get(&id)
                    .ok_or("unknown snapshot_id")?;
                let file_bytes = image.save(&path)?;
                Ok(json!({"snapshot_id": id, "tick": image.tick, "bytes": image.bytes, "file_bytes": file_bytes}))
            }
            "load_snapshot" => {
                if self.snapshots.len() >= self.max_snapshots {
                    return Err(format!(
                        "snapshot limit {} reached; drop_snapshot required",
                        self.max_snapshots
                    ));
                }
                let path = PathBuf::from(
                    req["path"]
                        .as_str()
                        .ok_or("load_snapshot needs path")?,
                );
                let image = EngineSnapshot::load(&path)?;
                self.install(&image)?;
                let (id, tick, bytes) = self.store(image)?;
                Ok(json!({"snapshot_id": id, "tick": tick, "bytes": bytes}))
            }
            other => Err(format!("unknown operation {other}")),
        }
    }
}

struct Config {
    checkpoint: PathBuf,
    sessions: usize,
    max_snapshots: usize,
    worker: bool,
}

fn parse_config() -> Result<Config, String> {
    let mut args = std::env::args().skip(1);
    let mut checkpoint = None;
    let mut sessions = 1usize;
    let mut max_snapshots = 8usize;
    let mut worker = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--checkpoint" => {
                checkpoint = Some(PathBuf::from(
                    args.next().ok_or("--checkpoint needs directory")?,
                ))
            }
            "--sessions" => {
                sessions = args
                    .next()
                    .ok_or("--sessions needs a count")?
                    .parse()
                    .map_err(|_| "--sessions must be an integer")?;
                if !(1..=16).contains(&sessions) {
                    return Err("--sessions must be 1..16".into());
                }
            }
            "--max-snapshots" => {
                max_snapshots = args
                    .next()
                    .ok_or("--max-snapshots needs a count")?
                    .parse()
                    .map_err(|_| "--max-snapshots must be an integer")?;
                if !(1..=1024).contains(&max_snapshots) {
                    return Err("--max-snapshots must be 1..1024".into());
                }
            }
            "--session-worker" => worker = true,
            "--help" | "-h" => {
                println!("rust-league-rl --checkpoint DIR [--sessions N] [--max-snapshots N]");
                println!("JSONL reset: {{\"op\":\"reset\",\"deck0\":[8 native card names],\"deck1\":[8 native card names],\"seed\":1,\"max_ticks\":7200,\"reward_state\":true,\"board\":true}}");
                println!("ops: step, step_many, observe, reward_state, placement_mask, finish, audit, snapshot, restore, drop_snapshot, save_snapshot, load_snapshot, close");
                return Err("help".into());
            }
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    Ok(Config {
        checkpoint: checkpoint.ok_or("--checkpoint required")?,
        sessions,
        max_snapshots,
        worker,
    })
}

fn server_from(config: &Config) -> Server {
    Server {
        checkpoint: config.checkpoint.clone(),
        session: None,
        initial_tick: 0,
        max_ticks: 7200,
        finished: false,
        configured: Value::Null,
        reward_state_enabled: false,
        initial_towers: BTreeMap::new(),
        board_enabled: false,
        board: BoardMemory::default(),
        snapshots: BTreeMap::new(),
        next_snapshot: 1,
        max_snapshots: config.max_snapshots,
        worker: config.worker,
    }
}

fn serve_stdio(config: &Config) -> Result<(), String> {
    let mut server = server_from(config);
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let parsed = serde_json::from_str::<Value>(&line.map_err(|e| e.to_string())?);
        let close = parsed.as_ref().is_ok_and(|v| v["op"] == "close");
        let response = if close {
            Ok(json!({}))
        } else {
            parsed
                .map_err(|e| e.to_string())
                .and_then(|req| server.handle(&req))
        };
        let response = response
            .map(|result| json!({"ok":true,"result":result}))
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

struct Kid {
    child: Child,
    writer: io::BufWriter<std::process::ChildStdin>,
    reader: io::BufReader<std::process::ChildStdout>,
}

impl Drop for Kid {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

fn exchange(kid: &mut Kid, line: &[u8]) -> Result<Vec<u8>, String> {
    kid.writer.write_all(line).map_err(|e| e.to_string())?;
    if !line.ends_with(b"\n") {
        kid.writer.write_all(b"\n").map_err(|e| e.to_string())?;
    }
    kid.writer.flush().map_err(|e| e.to_string())?;
    let mut response = Vec::new();
    let n = kid
        .reader
        .read_until(b'\n', &mut response)
        .map_err(|e| e.to_string())?;
    if n == 0 {
        return Err("session closed".into());
    }
    Ok(response)
}

fn error_line(error: &str) -> Vec<u8> {
    let mut bytes = serde_json::to_vec(&json!({"ok":false,"error":error,"reset_required":true}))
        .unwrap_or_else(|_| b"{\"ok\":false,\"error\":\"serialization\"}".to_vec());
    bytes.push(b'\n');
    bytes
}

fn spawn_workers(config: &Config) -> Result<Vec<Mutex<Kid>>, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let mut slots = Vec::with_capacity(config.sessions);
    for _ in 0..config.sessions {
        let mut command = Command::new(&exe);
        command
            .arg("--checkpoint")
            .arg(&config.checkpoint)
            .arg("--max-snapshots")
            .arg(config.max_snapshots.to_string())
            .arg("--session-worker")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());
        let mut child = command.spawn().map_err(|e| format!("spawn session: {e}"))?;
        let writer = io::BufWriter::new(child.stdin.take().ok_or("session stdin missing")?);
        let reader = io::BufReader::new(child.stdout.take().ok_or("session stdout missing")?);
        slots.push(Mutex::new(Kid {
            child,
            writer,
            reader,
        }));
    }
    Ok(slots)
}

fn step_many_line(slots: &[Mutex<Kid>], request: &Value) -> Result<Vec<u8>, String> {
    let items = request["items"]
        .as_array()
        .ok_or("step_many needs items array")?;
    let mut groups: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
    for (index, item) in items.iter().enumerate() {
        if !item.is_object() {
            return Err("step_many item must be an object".into());
        }
        let session = item["session"].as_u64().unwrap_or(0);
        if session >= slots.len() as u64 {
            return Err(format!(
                "session {session} is outside 0..{}",
                slots.len()
            ));
        }
        groups.entry(session).or_default().push(index);
    }
    let results = Mutex::new(vec![Value::Null; items.len()]);
    let failure: Mutex<Option<String>> = Mutex::new(None);
    std::thread::scope(|scope| {
        for (session, indices) in groups {
            let results = &results;
            let failure = &failure;
            scope.spawn(move || {
                let mut kid = match slots[session as usize].lock() {
                    Ok(guard) => guard,
                    Err(poisoned) => poisoned.into_inner(),
                };
                for index in indices {
                    if failure.lock().ok().and_then(|guard| guard.clone()).is_some() {
                        break;
                    }
                    let mut step = items[index].clone();
                    if let Some(object) = step.as_object_mut() {
                        object.insert("op".into(), json!("step"));
                        object.remove("session");
                    }
                    let mut line = match serde_json::to_vec(&step) {
                        Ok(line) => line,
                        Err(error) => {
                            if let Ok(mut slot) = failure.lock() {
                                *slot = Some(error.to_string());
                            }
                            break;
                        }
                    };
                    line.push(b'\n');
                    let reply = exchange(&mut kid, &line).and_then(|bytes| {
                        serde_json::from_slice::<Value>(bytes.trim_ascii_end())
                            .map_err(|e| e.to_string())
                    });
                    match reply {
                        Ok(envelope) if envelope["ok"] == true => {
                            if let Ok(mut slot) = results.lock() {
                                slot[index] = envelope["result"].clone();
                            }
                        }
                        Ok(envelope) => {
                            let message = envelope["error"]
                                .as_str()
                                .unwrap_or("session step failed")
                                .to_string();
                            if let Ok(mut slot) = failure.lock() {
                                *slot = Some(message);
                            }
                            break;
                        }
                        Err(error) => {
                            if let Ok(mut slot) = failure.lock() {
                                *slot = Some(error);
                            }
                            break;
                        }
                    }
                }
            });
        }
    });
    if let Some(error) = failure.lock().map_err(|e| e.to_string())?.clone() {
        return Err(error);
    }
    let items = results.into_inner().map_err(|e| e.to_string())?;
    let mut bytes =
        serde_json::to_vec(&json!({"ok":true,"result":{"items":items}})).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn serve_sessions(config: &Config) -> Result<(), String> {
    let slots = spawn_workers(config)?;
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        let parsed = serde_json::from_str::<Value>(&line);
        let mut closed = false;
        let response = match parsed {
            Ok(request) if request["op"] == "close" => {
                closed = true;
                for slot in &slots {
                    let mut kid = slot.lock().map_err(|e| e.to_string())?;
                    let _ = exchange(&mut kid, b"{\"op\":\"close\"}\n");
                }
                let mut bytes = serde_json::to_vec(&json!({"ok":true,"result":{}}))
                    .map_err(|e| e.to_string())?;
                bytes.push(b'\n');
                bytes
            }
            Ok(request) if request["op"] == "step_many" => {
                step_many_line(&slots, &request).unwrap_or_else(|error| error_line(&error))
            }
            Ok(request) => {
                let session = request["session"].as_u64().unwrap_or(0);
                if session >= slots.len() as u64 {
                    error_line(&format!(
                        "session {session} is outside 0..{}",
                        slots.len()
                    ))
                } else {
                    let mut kid = slots[session as usize]
                        .lock()
                        .map_err(|e| e.to_string())?;
                    let mut forwarded = line.into_bytes();
                    forwarded.push(b'\n');
                    exchange(&mut kid, &forwarded)?
                }
            }
            Err(_) => {
                let mut kid = slots[0].lock().map_err(|e| e.to_string())?;
                let mut forwarded = line.into_bytes();
                forwarded.push(b'\n');
                exchange(&mut kid, &forwarded)?
            }
        };
        stdout.write_all(&response).map_err(|e| e.to_string())?;
        stdout.flush().map_err(|e| e.to_string())?;
        if closed {
            break;
        }
    }
    Ok(())
}

fn run() -> Result<(), String> {
    let config = match parse_config() {
        Ok(config) => config,
        Err(error) if error == "help" => return Ok(()),
        Err(error) => return Err(error),
    };
    if config.worker || config.sessions == 1 {
        serve_stdio(&config)
    } else {
        serve_sessions(&config)
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
