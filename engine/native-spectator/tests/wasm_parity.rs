//! The parity measurement itself is `tests/wasm_parity/run_parity.py`.
//! This checks the runner parses and, once the wasm artifact exists, that the
//! C2 exports are actually present.

use std::path::PathBuf;
use std::process::Command;

fn node() -> PathBuf {
    PathBuf::from("/home/cochon/.local/bin/node")
}

#[test]
fn parity_runner_parses() {
    let runner = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/wasm_parity/runner.mjs");
    let output = Command::new(node())
        .arg("--check")
        .arg(&runner)
        .output()
        .expect("node --check");
    assert!(
        output.status.success(),
        "node --check failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn built_wasm_exports_the_c2_abi() {
    let wasm = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/wasm32-unknown-unknown/release/clash_painter.wasm");
    if !wasm.exists() {
        eprintln!("skip: {} is not built yet", wasm.display());
        return;
    }
    let script = r#"
const fs = await import('node:fs');
const bytes = fs.readFileSync(process.argv[1]);
const mod = await WebAssembly.compile(bytes);
const names = new Set(WebAssembly.Module.exports(mod).map((item) => item.name));
const required = [
  'memory','painter_alloc','painter_free','painter_vfs_put','painter_init',
  'painter_set_lag_ticks','painter_observe','painter_step','painter_result_ptr',
  'painter_result_len','painter_texture_ptr','painter_texture_len',
  'painter_texture_release','painter_reset','painter_stats_json','painter_last_error'
];
const missing = required.filter((name) => !names.has(name));
if (missing.length) {
  console.error(missing.join(','));
  process.exit(1);
}
"#;
    let output = Command::new(node())
        .arg("--input-type=module")
        .arg("--eval")
        .arg(script)
        .arg(&wasm)
        .output()
        .expect("node wasm export check");
    assert!(
        output.status.success(),
        "missing exports: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
