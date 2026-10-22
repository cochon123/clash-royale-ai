//! Bounded JSON-lines interface for differential native/Rust comparisons.
use clash_native_core::deck;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

fn field<'a>(value: &'a Value, name: &str) -> Result<&'a Value, String> {
    value.get(name).ok_or_else(|| format!("Missing {name}"))
}

fn i32_field(value: &Value, name: &str) -> Result<i32, String> {
    field(value, name)?
        .as_i64()
        .and_then(|number| number.try_into().ok())
        .ok_or_else(|| format!("{name} must be an i32"))
}

fn u64_entries(request: &Value) -> Result<Vec<u64>, String> {
    field(request, "values")?
        .as_array()
        .ok_or("values must be an array")?
        .iter()
        .map(|entry| {
            entry
                .as_u64()
                .ok_or("Entries must be u64 identities".into())
        })
        .collect()
}

fn execute(request: Value) -> Result<Value, String> {
    let operation = field(&request, "op")?
        .as_str()
        .ok_or("op must be a string")?;
    match operation {
        "all_nonzero_array_40" | "any_nonzero_array_40" => {
            let entries = u64_entries(&request)?;
            let count = i32_field(&request, "count")?;
            let predicate = if operation == "all_nonzero_array_40" {
                deck::all_nonzero_array_40(&entries, count)
            } else {
                deck::any_nonzero_array_40(&entries, count)
            }
            .map_err(|error| {
                format!(
                    "Count {} exceeds supplied length {}",
                    error.count, error.available
                )
            })?;
            Ok(json!({"result": i32::from(predicate)}))
        }
        "load_array_40" => {
            let entries = u64_entries(&request)?;
            let result = deck::load_array_40(
                &entries,
                i32_field(&request, "count")?,
                i32_field(&request, "index")?,
            )
            .map_err(|error| format!("{error:?}"))?;
            Ok(json!({"result": result}))
        }
        "swap_array_20" | "find_array_40" => {
            let mut entries = u64_entries(&request)?;
            if operation == "swap_array_20" {
                deck::swap_array_20(
                    &mut entries,
                    i32_field(&request, "first")?,
                    i32_field(&request, "second")?,
                )
                .map_err(|error| {
                    format!("Index {} outside length {}", error.index, error.length)
                })?;
                Ok(json!({"result": entries}))
            } else {
                let needle = field(&request, "needle")?
                    .as_u64()
                    .ok_or("needle must be u64")?;
                let result = deck::find_array_40(&entries, i32_field(&request, "count")?, needle)
                    .map_err(|error| {
                    format!(
                        "Count {} exceeds supplied length {}",
                        error.count, error.available
                    )
                })?;
                Ok(json!({"result": result}))
            }
        }
        "load_field_08" | "load_field_2c" | "load_field_4c" => {
            let bytes = field(&request, "prefix")?
                .as_array()
                .ok_or("prefix must be an array")?;
            if bytes.len() != 0x50 {
                return Err("prefix must contain exactly 80 bytes".into());
            }
            let mut prefix = [0u8; 0x50];
            for (output, value) in prefix.iter_mut().zip(bytes) {
                *output = value
                    .as_u64()
                    .and_then(|v| v.try_into().ok())
                    .ok_or("Invalid byte")?;
            }
            let result = match operation {
                "load_field_08" => deck::load_field_08(&prefix),
                "load_field_2c" => u64::from(deck::load_field_2c(&prefix)),
                "load_field_4c" => u64::from(deck::load_field_4c(&prefix)),
                _ => unreachable!(),
            };
            Ok(json!({"result": result}))
        }
        _ => Err(format!("Unknown recovered operation: {operation}")),
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for line in stdin.lock().lines() {
        let result = serde_json::from_str(&line?)
            .map_err(|error| error.to_string())
            .and_then(execute)
            .unwrap_or_else(|error| json!({"error": error}));
        serde_json::to_writer(&mut stdout, &result)?;
        stdout.write_all(b"\n")?;
        stdout.flush()?;
    }
    Ok(())
}
