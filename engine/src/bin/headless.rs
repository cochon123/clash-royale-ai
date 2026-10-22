//! One JSON request and response per line on stdin/stdout. This interface is a
//! deterministic local approximation suitable for experiments, not an emulator
//! of the original game's server. See docs/headless.md for the protocol.
use clash_rust::{Action, Battle, BattleConfig, Team};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    io::{self, BufRead, Write},
    time::Instant,
};

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum Request {
    Reset {
        #[serde(default = "default_seed")]
        seed: u64,
        decks: Option<[Vec<String>; 2]>,
        config: Option<BattleConfig>,
    },
    Step {
        #[serde(default)]
        actions: Vec<Action>,
        #[serde(default = "default_ticks")]
        ticks: u32,
        #[serde(default)]
        bots: Vec<Team>,
    },
    Observe,
}
fn default_seed() -> u64 {
    1
}
fn default_ticks() -> u32 {
    1
}

fn response(battle: &Battle, action_results: Vec<Value>, newly_finished: bool) -> Value {
    let mut rewards = [0, 0];
    if newly_finished {
        if let Some(winner) = battle.outcome.and_then(|o| o.winner) {
            rewards[winner.index()] = 1;
            rewards[winner.opponent().index()] = -1;
        }
    }
    json!({"ok": true, "observation": battle.observation(), "rewards": rewards,
        "done": battle.outcome.is_some(), "action_results": action_results})
}

fn handle(battle: &mut Battle, request: Request) -> Value {
    match request {
        Request::Reset {
            seed,
            decks,
            config,
        } => {
            let deck = clash_rust::default_deck();
            match Battle::with_decks(
                seed,
                decks.unwrap_or([deck.clone(), deck]),
                config.unwrap_or_default(),
            ) {
                Ok(new) => {
                    *battle = new;
                    response(battle, vec![], false)
                }
                Err(error) => json!({"ok": false, "error": error.to_string()}),
            }
        }
        Request::Observe => response(battle, vec![], false),
        Request::Step {
            actions,
            ticks,
            bots,
        } => {
            if !(1..=10000).contains(&ticks) {
                return json!({"ok": false, "error": "ticks must be between 1 and 10000"});
            }
            let already_done = battle.outcome.is_some();
            let results = actions
                .into_iter()
                .map(|action| match battle.apply_action(action) {
                    Ok(()) => json!({"action": action, "ok": true}),
                    Err(error) => {
                        json!({"action": action, "ok": false, "error": error.to_string()})
                    }
                })
                .collect();
            for _ in 0..ticks {
                if battle.outcome.is_some() {
                    break;
                }
                for &team in &bots {
                    battle.bot_step(team);
                }
                battle.step();
            }
            response(battle, results, !already_done && battle.outcome.is_some())
        }
    }
}

fn benchmark(count: u32) {
    let start = Instant::now();
    let mut total_ticks = 0u64;
    let mut wins = [0u32; 3];
    for seed in 1..=count {
        let mut battle = Battle::new(u64::from(seed));
        while battle.outcome.is_none() {
            battle.bot_step(Team::Blue);
            battle.bot_step(Team::Red);
            battle.step();
        }
        total_ticks += battle.tick;
        match battle.outcome.unwrap().winner {
            Some(Team::Blue) => wins[0] += 1,
            Some(Team::Red) => wins[1] += 1,
            None => wins[2] += 1,
        }
    }
    let seconds = start.elapsed().as_secs_f64();
    println!(
        "{}",
        json!({"matches": count, "ticks": total_ticks, "elapsed_seconds": seconds,
        "matches_per_second": f64::from(count)/seconds, "ticks_per_second": total_ticks as f64/seconds,
        "outcomes": {"blue": wins[0], "red": wins[1], "draw": wins[2]},
        "tick_ms": clash_rust::TICK_MS, "includes": "match initialization and seeded bots; excludes JSON observations and rendering"})
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|s| s == "--benchmark") {
        let count = args
            .get(1)
            .map(|s| s.parse::<u32>())
            .transpose()?
            .unwrap_or(100);
        if count == 0 {
            return Err("benchmark count must be positive".into());
        }
        benchmark(count);
        return Ok(());
    }
    let mut battle = Battle::new(1);
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let value = match serde_json::from_str::<Request>(&line) {
            Ok(request) => handle(&mut battle, request),
            Err(error) => json!({"ok": false, "error": error.to_string()}),
        };
        serde_json::to_writer(&mut out, &value)?;
        out.write_all(b"\n")?;
        out.flush()?;
    }
    Ok(())
}
