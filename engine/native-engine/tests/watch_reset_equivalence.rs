//! C4 equivalence: reset #1 in a fresh process matches reset #5 in a reused one.
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

const PAIRS: &[(&str, &str)] = &[
    ("hog26", "log-bait"),
    ("hog26", "pixel9_mortar"),
    ("log-bait", "pixel9_mortar"),
];
const SEEDS: &[u64] = &[1, 7, 42, 1000, 44219];

fn checkpoint() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../analysis/native-pipeline/oracle-export-01/stage01-attempt001/state")
}

fn binary() -> PathBuf {
    // This Cargo does not export CARGO_BIN_EXE_* to integration tests. The test
    // executable lives in target/<profile>/deps, next to the native-watch bin.
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_native_watch") {
        return PathBuf::from(path);
    }
    let exe = std::env::current_exe().expect("test executable path");
    let bin = exe
        .parent()
        .and_then(|deps| deps.parent())
        .map(|profile| profile.join("native-watch"))
        .expect("target profile directory");
    assert!(
        bin.is_file(),
        "native-watch was not built next to the test harness: {}",
        bin.display()
    );
    bin
}

struct Engine {
    child: Child,
    stdin: std::process::ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
}

impl Engine {
    fn spawn(preload: bool) -> Self {
        let mut command = Command::new(binary());
        command.arg("--checkpoint").arg(checkpoint());
        if preload {
            command.arg("--preload");
        }
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());
        let mut child = command.spawn().expect("spawn native-watch");
        let stdin = child.stdin.take().expect("stdin");
        let stdout = BufReader::new(child.stdout.take().expect("stdout"));
        Self {
            child,
            stdin,
            stdout,
        }
    }

    fn request(&mut self, value: &Value) -> Value {
        serde_json::to_writer(&mut self.stdin, value).expect("write request");
        self.stdin.write_all(b"\n").expect("write newline");
        self.stdin.flush().expect("flush request");
        let mut line = String::new();
        let read = self.stdout.read_line(&mut line).expect("read reply");
        assert!(read > 0, "native-watch closed stdout");
        serde_json::from_str(&line).unwrap_or_else(|error| {
            panic!("reply is not JSON ({error}): {line}");
        })
    }

    fn close(mut self) {
        let reply = self.request(&json!({"op":"close"}));
        assert_eq!(reply["ok"], true, "close failed: {reply}");
        let status = self.child.wait().expect("wait native-watch");
        assert!(status.success(), "native-watch exited {status}");
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

fn scrub(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|key, _| !key.ends_with("_seconds"));
            for child in map.values_mut() {
                scrub(child);
            }
        }
        Value::Array(items) => {
            for child in items {
                scrub(child);
            }
        }
        _ => {}
    }
}

fn canonical(value: &Value) -> Vec<u8> {
    let mut copy = value.clone();
    scrub(&mut copy);
    serde_json::to_vec(&copy).expect("canonical json")
}

fn first_diff(left: &Value, right: &Value, path: &str) -> Option<String> {
    match (left, right) {
        (Value::Object(a), Value::Object(b)) => {
            let mut keys: Vec<&String> = a.keys().chain(b.keys()).collect();
            keys.sort();
            keys.dedup();
            for key in keys {
                let next = format!("{path}.{key}");
                match (a.get(key), b.get(key)) {
                    (None, Some(_)) => return Some(format!("{next} missing from fresh")),
                    (Some(_), None) => return Some(format!("{next} missing from reused")),
                    (Some(av), Some(bv)) => {
                        if let Some(diff) = first_diff(av, bv, &next) {
                            return Some(diff);
                        }
                    }
                    (None, None) => {}
                }
            }
            None
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return Some(format!("{path} length {} != {}", a.len(), b.len()));
            }
            for (index, (av, bv)) in a.iter().zip(b.iter()).enumerate() {
                if let Some(diff) = first_diff(av, bv, &format!("{path}[{index}]")) {
                    return Some(diff);
                }
            }
            None
        }
        _ if left == right => None,
        _ => {
            let mut text = format!("{path}: {left} != {right}");
            text.truncate(500);
            Some(text)
        }
    }
}

/// Fixed 600-tick script. Actions are submitted at the start of the step that
/// begins on that tick, then the world advances 20 ticks. Both processes send
/// this exact sequence after their compared reset.
fn action_script(seed: u64, deck0: &str, deck1: &str) -> Vec<Value> {
    let mut requests = vec![json!({
        "op": "reset",
        "deck_preset": deck0,
        "deck_preset_side1": deck1,
        "seed": seed,
        "max_ticks": 4000,
    })];
    let mut tick = 0u64;
    while tick < 600 {
        let mut actions = Vec::new();
        if tick == 100 {
            actions.push(json!({"side": 0, "slot": 0, "x": 9000, "y": 26000}));
        }
        if tick == 220 {
            actions.push(json!({"side": 1, "slot": 0, "x": 9000, "y": 6000}));
        }
        if tick == 340 {
            actions.push(json!({"side": 0, "slot": 1, "x": 4500, "y": 24000}));
            actions.push(json!({"side": 1, "slot": 1, "x": 13000, "y": 8000}));
        }
        if tick == 460 {
            actions.push(json!({"side": 0, "slot": 2, "x": 7000, "y": 28000}));
        }
        requests.push(json!({"op": "step", "ticks": 20, "actions": actions}));
        tick += 20;
    }
    requests
}

fn expect_ok<'a>(reply: &'a Value, what: &str) -> &'a Value {
    assert_eq!(
        reply["ok"],
        true,
        "{what} failed: {}",
        reply["error"].as_str().unwrap_or("<no error>")
    );
    &reply["result"]
}

fn run_script(engine: &mut Engine, seed: u64, deck0: &str, deck1: &str) -> Vec<Value> {
    let mut replies = Vec::new();
    let mut advanced = 0u64;
    for request in action_script(seed, deck0, deck1) {
        let reply = engine.request(&request);
        let result = expect_ok(&reply, "script request").clone();
        if request["op"] == "step" {
            advanced += result["ticks_advanced"].as_u64().unwrap_or(0);
        }
        replies.push(reply);
    }
    assert_eq!(
        advanced, 600,
        "script advanced {advanced} ticks, expected 600"
    );
    replies
}

#[test]
fn ping_before_load_is_not_ready() {
    let mut engine = Engine::spawn(false);
    let reply = engine.request(&json!({"op":"ping"}));
    let result = expect_ok(&reply, "ping");
    assert_eq!(result["ready"], false);
    assert_eq!(result["load_seconds"], 0.0);
    assert_eq!(result["resets"], 0);
    engine.close();
}

#[test]
fn preload_of_missing_checkpoint_exits() {
    let dir = std::env::temp_dir().join(format!("native-watch-missing-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let output = Command::new(binary())
        .arg("--checkpoint")
        .arg(&dir)
        .arg("--preload")
        .output()
        .expect("spawn native-watch");
    assert_eq!(
        output.status.code(),
        Some(2),
        "stderr {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("checkpoint.json"),
        "verification failure should name checkpoint.json: {stderr}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn first_hash_mismatch_aborts_preload_and_reset() {
    let source = checkpoint();
    assert!(
        source.join("checkpoint.json").is_file(),
        "checkpoint missing at {}",
        source.display()
    );
    let dir = std::env::temp_dir().join(format!("native-watch-bad-hash-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let mut document: Value = serde_json::from_slice(
        &std::fs::read(source.join("checkpoint.json")).expect("read checkpoint.json"),
    )
    .expect("parse checkpoint.json");
    document["regions"][0]["sha256"] =
        json!("0000000000000000000000000000000000000000000000000000000000000000");
    let region = document["regions"][0]["file"]
        .as_str()
        .expect("region file")
        .to_string();
    std::fs::write(
        dir.join("checkpoint.json"),
        serde_json::to_vec(&document).unwrap(),
    )
    .unwrap();
    std::fs::write(dir.join(&region), b"not-the-region").unwrap();

    let preloaded = Command::new(binary())
        .arg("--checkpoint")
        .arg(&dir)
        .arg("--preload")
        .output()
        .expect("spawn preload");
    assert_eq!(
        preloaded.status.code(),
        Some(2),
        "preload stderr {}",
        String::from_utf8_lossy(&preloaded.stderr)
    );
    let stderr = String::from_utf8_lossy(&preloaded.stderr);
    assert!(
        stderr.contains("hash mismatch") || stderr.contains("SHA-256"),
        "preload should abort on the hash check: {stderr}"
    );

    let mut command = Command::new(binary());
    command
        .arg("--checkpoint")
        .arg(&dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn reset");
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let reset = json!({
        "op": "reset",
        "deck_preset": "hog26",
        "deck_preset_side1": "log-bait",
        "seed": 1,
        "max_ticks": 100,
    });
    serde_json::to_writer(&mut stdin, &reset).unwrap();
    stdin.write_all(b"\n").unwrap();
    stdin.flush().unwrap();
    let mut line = String::new();
    stdout.read_line(&mut line).unwrap();
    let reply: Value = serde_json::from_str(&line).expect("reset reply");
    assert_eq!(
        reply["ok"], false,
        "bad checkpoint reset should fail: {reply}"
    );
    assert_eq!(reply["reset_required"], true);
    let error = reply["error"].as_str().unwrap_or("");
    assert!(
        error.contains("hash mismatch") || error.contains("SHA-256"),
        "reset error should be the verification failure: {error}"
    );
    serde_json::to_writer(&mut stdin, &json!({"op":"close"})).unwrap();
    stdin.write_all(b"\n").unwrap();
    stdin.flush().unwrap();
    line.clear();
    stdout.read_line(&mut line).unwrap();
    let closed: Value = serde_json::from_str(&line).expect("close reply");
    assert_eq!(
        closed["ok"], true,
        "process should stay up to report the failure"
    );
    let status = child.wait().unwrap();
    assert!(status.success(), "close should exit 0, got {status}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn fresh_reset_matches_fifth_reused_reset() {
    const RESET_KEYS: &[&str] = &[
        "observation",
        "frame",
        "configured_decks",
        "deck_source_rows",
        "seed",
        "reset_seconds",
        "max_ticks",
        "engine_reset",
    ];
    for &seed in SEEDS {
        for (index, (deck0, deck1)) in PAIRS.iter().enumerate() {
            eprintln!("equivalence seed {seed} {deck0} vs {deck1}");
            let mut fresh = Engine::spawn(false);
            let before = fresh.request(&json!({"op":"ping"}));
            let before = expect_ok(&before, "fresh ping");
            assert_eq!(before["ready"], false);
            assert_eq!(before["resets"], 0);
            let fresh_replies = run_script(&mut fresh, seed, deck0, deck1);
            let after = fresh.request(&json!({"op":"ping"}));
            let after = expect_ok(&after, "fresh ping after reset");
            assert_eq!(after["ready"], true);
            assert_eq!(after["resets"], 1);
            assert!(after["load_seconds"].as_f64().unwrap_or(0.0) > 0.0);
            fresh.close();

            let mut reused = Engine::spawn(true);
            let ready = reused.request(&json!({"op":"ping"}));
            let ready = expect_ok(&ready, "preloaded ping");
            assert_eq!(ready["ready"], true);
            assert_eq!(ready["resets"], 0);
            assert!(ready["load_seconds"].as_f64().unwrap_or(0.0) > 0.0);
            for warm in 0..4 {
                let (warm0, warm1) = PAIRS[(index + warm) % PAIRS.len()];
                let reply = reused.request(&json!({
                    "op": "reset",
                    "deck_preset": warm0,
                    "deck_preset_side1": warm1,
                    "seed": 9000 + warm as u64,
                    "max_ticks": 4000,
                }));
                expect_ok(&reply, "warmup reset");
            }
            let reused_replies = run_script(&mut reused, seed, deck0, deck1);
            let done = reused.request(&json!({"op":"ping"}));
            let done = expect_ok(&done, "reused ping");
            assert_eq!(done["ready"], true);
            assert_eq!(done["resets"], 5, "compared battle must be reset #5");
            reused.close();

            assert_eq!(fresh_replies.len(), reused_replies.len());
            let reset_result = &fresh_replies[0]["result"];
            for key in RESET_KEYS {
                assert!(reset_result.get(key).is_some(), "reset reply missing {key}");
            }
            assert_eq!(
                reset_result["engine_reset"],
                "fresh original-native harness from loaded context; deck rows verified"
            );
            for (step, (left, right)) in fresh_replies.iter().zip(reused_replies.iter()).enumerate()
            {
                let left_bytes = canonical(left);
                let right_bytes = canonical(right);
                if left_bytes != right_bytes {
                    let mut left_value = left.clone();
                    let mut right_value = right.clone();
                    scrub(&mut left_value);
                    scrub(&mut right_value);
                    let diff = first_diff(&left_value, &right_value, "$")
                        .unwrap_or_else(|| "canonical bytes differ with no structural diff".into());
                    panic!(
                        "seed {seed} {deck0} vs {deck1} reply {step} is not byte-identical excluding *_seconds: {diff}"
                    );
                }
            }
        }
    }
}
