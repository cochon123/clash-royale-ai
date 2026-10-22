//! Exercise the exact generated C++ runtime function for source differential tests.
use clash_recompiled::session::Session;
use std::path::Path;

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let checkpoint = args.next().ok_or("checkpoint directory required")?;
    let mut session = Session::load(Path::new(&checkpoint))?;
    let mut rows = Vec::new();
    for value in args {
        let value = value.parse::<u64>().map_err(|e| e.to_string())?;
        let result = session.call(0x200cffbc, &[value], 0)?;
        rows.push(serde_json::json!({"input": value, "result": result,
            "x": session.cpu.x.to_vec(), "sp": session.cpu.sp,
            "pc": session.cpu.pc, "nzcv": session.cpu.nzcv}));
    }
    println!("{}", serde_json::Value::Array(rows));
    Ok(())
}
