use clash_native_core::oracle::NativeSession;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let root=PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let checkpoint=root.join("analysis/native-pipeline/oracle-export-01/stage01-attempt001/state");
    let output=std::env::args().nth(1).map(PathBuf::from).unwrap_or(root.join("analysis/native-port/profile-original.json"));
    let mut session=NativeSession::load(&checkpoint)?;
    let report=session.profile_original_match()?;
    if let Some(parent)=output.parent(){std::fs::create_dir_all(parent).map_err(|e|e.to_string())?;}
    std::fs::write(&output,serde_json::to_vec_pretty(&report).unwrap()).map_err(|e|e.to_string())?;
    println!("{}",report["outcome"]);
    println!("{} blocks written to {}",report["blocks"].as_array().unwrap().len(),output.display());
    Ok(())
}
