//! Minimal sequential JSON-lines interface to the original native engine.
use clash_native_core::oracle::{NativeSession, PlayCardRequest};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::time::Instant;

struct Server {
    checkpoint: PathBuf,
    session: Option<NativeSession>,
    initial_tick: u64,
    max_ticks: u64,
    finished: bool,
}

fn tick(observation: &Value) -> Result<u64, String> {
    observation["tick"]
        .as_u64()
        .ok_or("native observation has no tick".into())
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
    let coordinate = |key| -> Result<i32, String> {
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

impl Server {
    fn handle(&mut self, request: &Value) -> Result<Value, String> {
        match request["op"].as_str().ok_or("request needs op")? {
            "reset" => {
                self.max_ticks = request["max_ticks"].as_u64().unwrap_or(7200);
                if !(1..=36000).contains(&self.max_ticks) {
                    return Err("max_ticks must be 1..36000".into());
                }
                // Release the old instance before parsing the large snapshot again.
                self.session = None;
                let started = Instant::now();
                let mut session = NativeSession::load(&self.checkpoint)?;
                let observation = session.observe()?;
                self.initial_tick = tick(&observation)?;
                self.finished = false;
                self.session = Some(session);
                Ok(
                    json!({"observation":observation,"reset_seconds":started.elapsed().as_secs_f64(),"max_ticks":self.max_ticks,"engine_reset":"same verified prepared snapshot"}),
                )
            }
            "observe" => {
                Ok(json!({"observation":self.session.as_mut().ok_or("reset required")?.observe()?}))
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
                let before = tick(&session.observe()?)?;
                let mut results = Vec::new();
                for action in actions {
                    results.push(session.play_card(action)?);
                }
                let mut calls = 0;
                for _ in 0..count {
                    if session.is_terminal()?
                        || before + calls - self.initial_tick >= self.max_ticks
                    {
                        break;
                    }
                    session.world_step()?;
                    calls += 1;
                }
                let observation = session.observe()?;
                let after = tick(&observation)?;
                let terminated = observation["terminated"]
                    .as_bool()
                    .ok_or("missing native terminal flag")?;
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
                println!(
                    "native-rl --checkpoint DIR; JSONL operations: reset, step, observe, close"
                );
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
    };
    let input = io::stdin();
    let mut output = io::BufWriter::new(io::stdout().lock());
    for line in input.lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        let request = serde_json::from_str::<Value>(&line);
        let close = request.as_ref().is_ok_and(|v| v["op"] == "close");
        let response = if close {
            json!({"ok":true,"result":{}})
        } else {
            let result = request
                .map_err(|e| e.to_string())
                .and_then(|r| server.handle(&r));
            match result {
                Ok(value) => json!({"ok":true,"result":value}),
                Err(error) => {
                    // Native errors can leave partial updates: never continue them.
                    server.session = None;
                    json!({"ok":false,"error":error,"reset_required":true})
                }
            }
        };
        writeln!(output, "{}", response).map_err(|e| e.to_string())?;
        output.flush().map_err(|e| e.to_string())?;
        if close {
            break;
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("native-rl: {error}");
        std::process::exit(2);
    }
}
