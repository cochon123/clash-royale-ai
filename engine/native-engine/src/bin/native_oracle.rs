use clash_native_core::oracle::{run_checkpoint, PlayCardRequest};
use std::path::PathBuf;

fn main() {
    if let Err(error) = run() {
        eprintln!("native-oracle: {error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut checkpoint = None;
    let mut steps = None;
    let mut harness_steps = 0;
    let mut until_terminal = false;
    let mut audit_out = None;
    let mut play_card = None;
    let mut after_play_steps = 0u64;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--checkpoint" => {
                checkpoint = Some(PathBuf::from(
                    args.next().ok_or("--checkpoint needs a directory")?,
                ))
            }
            "--world-steps" => {
                steps = Some(
                    args.next()
                        .ok_or("--world-steps needs a number")?
                        .parse::<u64>()
                        .map_err(|e| format!("invalid --world-steps: {e}"))?,
                )
            }
            "--harness-steps" => {
                harness_steps = args
                    .next()
                    .ok_or("--harness-steps needs a number")?
                    .parse::<u64>()
                    .map_err(|e| format!("invalid --harness-steps: {e}"))?
            }
            "--until-terminal" => until_terminal = true,
            "--play-card" => {
                let side = args
                    .next()
                    .ok_or("--play-card needs SIDE SLOT X Y")?
                    .parse::<u32>()
                    .map_err(|e| format!("invalid play side: {e}"))?;
                let slot = args
                    .next()
                    .ok_or("--play-card needs SIDE SLOT X Y")?
                    .parse::<u32>()
                    .map_err(|e| format!("invalid play slot: {e}"))?;
                let x = args
                    .next()
                    .ok_or("--play-card needs SIDE SLOT X Y")?
                    .parse::<i32>()
                    .map_err(|e| format!("invalid play x: {e}"))?;
                let y = args
                    .next()
                    .ok_or("--play-card needs SIDE SLOT X Y")?
                    .parse::<i32>()
                    .map_err(|e| format!("invalid play y: {e}"))?;
                play_card = Some(PlayCardRequest { side, slot, x, y });
            }
            "--after-play-steps" => {
                after_play_steps = args
                    .next()
                    .ok_or("--after-play-steps needs a number")?
                    .parse::<u64>()
                    .map_err(|e| format!("invalid --after-play-steps: {e}"))?;
            }
            "--audit-out" => {
                audit_out = Some(PathBuf::from(
                    args.next().ok_or("--audit-out needs a directory")?,
                ))
            }
            "--help" | "-h" => {
                println!("Usage: native-oracle --checkpoint DIR --world-steps N [--play-card SIDE SLOT X Y [--after-play-steps N]] [--harness-steps N | --until-terminal] [--audit-out DIR]");
                return Ok(());
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    let checkpoint = checkpoint.ok_or("missing --checkpoint DIR")?;
    let steps = steps.ok_or("missing --world-steps N")?;
    if until_terminal && harness_steps != 0 {
        return Err("choose either --harness-steps or --until-terminal".into());
    }
    let report = run_checkpoint(
        &checkpoint,
        steps,
        harness_steps,
        until_terminal,
        play_card,
        after_play_steps,
        audit_out.as_deref(),
    )?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report.json).map_err(|e| e.to_string())?
    );
    Ok(())
}
