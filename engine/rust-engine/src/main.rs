use clash_recompiled::session::Session;
use std::{path::PathBuf, time::Instant};
fn main() {
    if let Err(e) = run() {
        eprintln!("rust-engine: {e}");
        std::process::exit(2);
    }
}
fn run() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut checkpoint =
        root.join("analysis/native-pipeline/oracle-export-01/stage01-attempt001/state");
    let mut steps = 0u64;
    let mut until = false;
    let mut audit = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--checkpoint" => checkpoint = PathBuf::from(args.next().ok_or("checkpoint path")?),
            "--world-steps" => {
                steps = args
                    .next()
                    .ok_or("step count")?
                    .parse()
                    .map_err(|e| format!("steps: {e}"))?
            }
            "--until-terminal" => until = true,
            "--audit-out" => audit = Some(PathBuf::from(args.next().ok_or("audit directory")?)),
            _ => return Err(format!("unknown argument {a}")),
        }
    }
    let start = Instant::now();
    let mut session = Session::load(&checkpoint)?;
    let load = start.elapsed().as_secs_f64();
    let started = Instant::now();
    for _ in 0..steps {
        session.world_step()?;
    }
    let report = if until {
        session.native_match()?
    } else {
        serde_json::json!({"world_steps":steps,"ticks":session.ticks(),"elapsed_seconds":started.elapsed().as_secs_f64()})
    };
    if let Some(out) = audit {
        session.audit(&out)?;
    }
    println!(
        "{}",
        serde_json::json!({"load_seconds":load,"execution":report})
    );
    Ok(())
}
