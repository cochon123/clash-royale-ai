//! Source-observed guest allocator state used by the native execution host.

use crate::memory::Memory;
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct Arena {
    base: u64,
    capacity: u64,
    pointer: u64,
}

/// Deterministic adapter for the allocator state recorded in native checkpoints.
///
/// Allocator size classes, free-bucket order, alignment selection, and extra
/// arena addresses match the model in `native-engine/src/oracle.rs`.
#[derive(Clone, Debug)]
pub struct GuestHeap {
    base: u64,
    capacity: u64,
    pointer: u64,
    allocations: BTreeMap<u64, u64>,
    free: BTreeMap<u64, Vec<u64>>,
    extras: Vec<Arena>,
    baseline_pointer: u64,
    baseline_free: BTreeMap<u64, Vec<u64>>,
    baseline_extras: Vec<Arena>,
    journal: Vec<(u64, Option<u64>)>,
}

impl GuestHeap {
    const EXTRA_BASE: u64 = 0x2_0000_0000;
    const EXTRA_STEP: u64 = 0x2000_0000;
    const EXTRA_CAPACITY: u64 = 0x1000_0000;

    /// Restore allocator state from a checkpoint's `native_heap` value.
    pub fn from_json(value: &Value) -> Result<Self, String> {
        let base = json_u64(&value["base"])?;
        let capacity = json_u64(&value["capacity"])?;
        let pointer = json_u64(&value["primary_pointer"])?;
        let end = base
            .checked_add(capacity)
            .ok_or("checkpoint heap address overflow")?;
        if pointer < base || pointer > end {
            return Err("checkpoint heap pointer out of range".into());
        }

        let mut allocations = BTreeMap::new();
        for (address, size) in value["allocations"]
            .as_object()
            .ok_or("heap allocations missing")?
        {
            allocations.insert(
                address
                    .parse::<u64>()
                    .map_err(|e| format!("bad allocation address {address:?}: {e}"))?,
                json_u64(size)?,
            );
        }

        let mut free = BTreeMap::new();
        if let Some(buckets) = value["free_buckets"].as_object() {
            for (size, items) in buckets {
                let class = size
                    .parse::<u64>()
                    .map_err(|e| format!("bad free bucket {size:?}: {e}"))?;
                let pointers = items
                    .as_array()
                    .ok_or("free bucket is not an array")?
                    .iter()
                    .map(json_u64)
                    .collect::<Result<Vec<_>, _>>()?;
                free.insert(class, pointers);
            }
        }

        let mut extras = Vec::new();
        if let Some(arenas) = value["extra_heaps"].as_array() {
            for arena in arenas {
                let base = json_u64(&arena["base"])?;
                let capacity = json_u64(&arena["capacity"])?;
                let pointer = json_u64(&arena["pointer"])?;
                let end = base
                    .checked_add(capacity)
                    .ok_or("checkpoint extra heap address overflow")?;
                if pointer < base || pointer > end {
                    return Err(format!(
                        "checkpoint extra heap pointer out of range at {base:#x}"
                    ));
                }
                extras.push(Arena {
                    base,
                    capacity,
                    pointer,
                });
            }
        }
        if extras.len() > 4 {
            return Err("too many checkpoint extra heaps".into());
        }

        Ok(Self {
            base,
            capacity,
            pointer,
            allocations,
            baseline_pointer: pointer,
            baseline_free: free.clone(),
            baseline_extras: extras.clone(),
            free,
            extras,
            journal: Vec::new(),
        })
    }

    /// Restore checkpoint allocator state without cloning its large allocation
    /// map. Only entries changed since the previous reset are replayed.
    pub fn reset(&mut self) {
        while let Some((pointer, previous_size)) = self.journal.pop() {
            match previous_size {
                Some(size) => {
                    self.allocations.insert(pointer, size);
                }
                None => {
                    self.allocations.remove(&pointer);
                }
            }
        }
        self.pointer = self.baseline_pointer;
        self.free.clone_from(&self.baseline_free);
        self.extras.clone_from(&self.baseline_extras);
    }

    fn record_allocation(&mut self, pointer: u64, size: u64) {
        let previous_size = self.allocations.insert(pointer, size);
        self.journal.push((pointer, previous_size));
    }

    /// Full allocator image, including the reset baseline and journal.
    ///
    /// Addresses are decimal strings so the JSON integer width does not matter.
    pub fn export_full(&self) -> Value {
        let allocations = self
            .allocations
            .iter()
            .map(|(address, size)| (address.to_string(), json!(size)))
            .collect::<serde_json::Map<_, _>>();
        let free_buckets = |free: &BTreeMap<u64, Vec<u64>>| {
            free.iter()
                .map(|(size, pointers)| {
                    (
                        size.to_string(),
                        json!(pointers.iter().map(|p| p.to_string()).collect::<Vec<_>>()),
                    )
                })
                .collect::<serde_json::Map<_, _>>()
        };
        let arenas = |arenas: &[Arena]| {
            arenas
                .iter()
                .map(|arena| {
                    json!({
                        "base": arena.base.to_string(),
                        "capacity": arena.capacity.to_string(),
                        "pointer": arena.pointer.to_string(),
                    })
                })
                .collect::<Vec<_>>()
        };
        json!({
            "base": self.base.to_string(),
            "capacity": self.capacity.to_string(),
            "primary_pointer": self.pointer.to_string(),
            "allocations": allocations,
            "free_buckets": free_buckets(&self.free),
            "extra_heaps": arenas(&self.extras),
            "baseline_pointer": self.baseline_pointer.to_string(),
            "baseline_free": free_buckets(&self.baseline_free),
            "baseline_extras": arenas(&self.baseline_extras),
            "journal": self.journal.iter().map(|(pointer, previous)| json!({
                "pointer": pointer.to_string(),
                "previous": previous.map(|size| size.to_string()),
            })).collect::<Vec<_>>(),
        })
    }

    /// Restore [`export_full`]. This does not touch guest memory.
    pub fn import_full(value: &Value) -> Result<Self, String> {
        let parse_map = |object: &Value, label: &str| -> Result<BTreeMap<u64, u64>, String> {
            let mut out = BTreeMap::new();
            for (key, size) in object.as_object().ok_or_else(|| format!("{label} missing"))? {
                out.insert(parse_u64_value(&Value::String(key.clone()))?, json_u64(size)?);
            }
            Ok(out)
        };
        let parse_free = |object: &Value, label: &str| -> Result<BTreeMap<u64, Vec<u64>>, String> {
            let mut out = BTreeMap::new();
            for (key, pointers) in object
                .as_object()
                .ok_or_else(|| format!("{label} missing"))?
            {
                let class = parse_u64_value(&Value::String(key.clone()))?;
                let values = pointers
                    .as_array()
                    .ok_or_else(|| format!("{label} bucket is not an array"))?
                    .iter()
                    .map(parse_u64_value)
                    .collect::<Result<Vec<_>, _>>()?;
                out.insert(class, values);
            }
            Ok(out)
        };
        let parse_arenas = |value: &Value, label: &str| -> Result<Vec<Arena>, String> {
            value
                .as_array()
                .ok_or_else(|| format!("{label} missing"))?
                .iter()
                .map(|arena| {
                    Ok(Arena {
                        base: parse_u64_value(&arena["base"])?,
                        capacity: parse_u64_value(&arena["capacity"])?,
                        pointer: parse_u64_value(&arena["pointer"])?,
                    })
                })
                .collect()
        };
        let journal = value["journal"]
            .as_array()
            .ok_or("heap journal missing")?
            .iter()
            .map(|entry| {
                let pointer = parse_u64_value(&entry["pointer"])?;
                let previous = if entry["previous"].is_null() {
                    None
                } else {
                    Some(parse_u64_value(&entry["previous"])?)
                };
                Ok((pointer, previous))
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Self {
            base: parse_u64_value(&value["base"])?,
            capacity: parse_u64_value(&value["capacity"])?,
            pointer: parse_u64_value(&value["primary_pointer"])?,
            allocations: parse_map(&value["allocations"], "allocations")?,
            free: parse_free(&value["free_buckets"], "free_buckets")?,
            extras: parse_arenas(&value["extra_heaps"], "extra_heaps")?,
            baseline_pointer: parse_u64_value(&value["baseline_pointer"])?,
            baseline_free: parse_free(&value["baseline_free"], "baseline_free")?,
            baseline_extras: parse_arenas(&value["baseline_extras"], "baseline_extras")?,
            journal,
        })
    }

    /// Serialize the allocator state using the checkpoint-compatible schema.
    pub fn snapshot(&self) -> Value {
        let allocations = self
            .allocations
            .iter()
            .map(|(address, size)| (address.to_string(), json!(size)))
            .collect::<serde_json::Map<_, _>>();
        let free_buckets = self
            .free
            .iter()
            .map(|(size, pointers)| (size.to_string(), json!(pointers)))
            .collect::<serde_json::Map<_, _>>();
        json!({
            "version": 1,
            "base": self.base,
            "capacity": self.capacity,
            "primary_pointer": self.pointer,
            "allocations": allocations,
            "free_buckets": free_buckets,
            "extra_heaps": self.extras.iter().map(|arena| json!({
                "base": arena.base,
                "capacity": arena.capacity,
                "pointer": arena.pointer,
            })).collect::<Vec<_>>(),
        })
    }

    /// Allocate using the checkpoint allocator's exact size classes and reuse
    /// order. Extra arenas are mapped zeroed on first use.
    pub fn alloc(&mut self, size: u64, alignment: u64, memory: &mut Memory) -> Result<u64, String> {
        if alignment < 8 || !alignment.is_power_of_two() {
            return Err(format!("invalid guest alignment {alignment}"));
        }
        let class = size_class(size)?;
        if let Some(bucket) = self.free.get_mut(&class) {
            if let Some(position) = bucket.iter().rposition(|pointer| pointer % alignment == 0) {
                let pointer = bucket.remove(position);
                if bucket.is_empty() {
                    self.free.remove(&class);
                }
                self.record_allocation(pointer, size);
                return Ok(pointer);
            }
        }

        // The original model exhausts primary and checkpoint arenas in their
        // recorded order before creating the next deterministic extra arena.
        for index in 0..=self.extras.len() {
            let arena = if index == 0 {
                Arena {
                    base: self.base,
                    capacity: self.capacity,
                    pointer: self.pointer,
                }
            } else {
                self.extras[index - 1].clone()
            };
            let aligned = align(arena.pointer, alignment)?;
            let end = aligned.checked_add(class).ok_or("guest heap overflow")?;
            let arena_end = arena
                .base
                .checked_add(arena.capacity)
                .ok_or("guest heap address overflow")?;
            if end <= arena_end {
                if index == 0 {
                    self.pointer = end;
                } else {
                    self.extras[index - 1].pointer = end;
                }
                self.record_allocation(aligned, size);
                return Ok(aligned);
            }
        }

        if self.extras.len() >= 4 {
            return Err("guest heap exhausted primary and four extra arenas".into());
        }
        let index = self.extras.len();
        let base = Self::EXTRA_BASE + index as u64 * Self::EXTRA_STEP;
        let capacity = Self::EXTRA_CAPACITY;
        let pointer = align(base, alignment)?;
        let end = pointer
            .checked_add(class)
            .ok_or("guest extra heap overflow")?;
        if end > base + capacity {
            return Err("allocation does not fit extra heap".into());
        }
        memory.map_zeroed(
            base,
            usize::try_from(capacity).map_err(|_| "extra heap capacity too large")?,
        )?;
        self.extras.push(Arena {
            base,
            capacity,
            pointer: end,
        });
        self.record_allocation(pointer, size);
        Ok(pointer)
    }

    /// Free a live allocation into its size-class bucket. Null is a no-op.
    pub fn free_ptr(&mut self, pointer: u64) -> Result<(), String> {
        if pointer == 0 {
            return Ok(());
        }
        let size = self
            .allocations
            .remove(&pointer)
            .ok_or_else(|| format!("free of unknown/double-free pointer {pointer:#x}"))?;
        self.journal.push((pointer, Some(size)));
        self.free
            .entry(size_class(size)?)
            .or_default()
            .push(pointer);
        Ok(())
    }

    /// Whether `pointer` currently names a live allocation.
    pub fn is_live(&self, pointer: u64) -> bool {
        self.allocations.contains_key(&pointer)
    }
}

fn size_class(size: u64) -> Result<u64, String> {
    size.checked_add(15)
        .map(|value| (value & !15).max(16))
        .ok_or_else(|| "allocation size overflow".into())
}

fn align(value: u64, alignment: u64) -> Result<u64, String> {
    value
        .checked_add(alignment - 1)
        .map(|value| value & !(alignment - 1))
        .ok_or_else(|| "alignment overflow".into())
}

fn parse_u64_value(value: &Value) -> Result<u64, String> {
    if let Some(text) = value.as_str() {
        if let Some(hex) = text.strip_prefix("0x") {
            u64::from_str_radix(hex, 16).map_err(|e| format!("bad integer {text:?}: {e}"))
        } else {
            text.parse::<u64>()
                .map_err(|e| format!("bad integer {text:?}: {e}"))
        }
    } else {
        json_u64(value)
    }
}

fn json_u64(value: &Value) -> Result<u64, String> {
    if let Some(number) = value.as_u64() {
        Ok(number)
    } else if let Some(text) = value.as_str() {
        u64::from_str_radix(text.trim_start_matches("0x"), 16)
            .or_else(|_| text.parse())
            .map_err(|e| format!("bad integer {text:?}: {e}"))
    } else {
        Err("expected integer".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline() -> Value {
        let base = 0x5000_0000u64;
        let mut allocations = serde_json::Map::new();
        allocations.insert(base.to_string(), json!(31));
        json!({
            "version": 1,
            "base": base,
            "capacity": 0x10000,
            "primary_pointer": base + 0x100,
            "allocations": allocations,
            "free_buckets": {
                "16": [base + 0x40],
            },
            "extra_heaps": [{
                "base": 0x6000_0000u64,
                "capacity": 0x1000,
                "pointer": 0x6000_0100u64,
            }],
        })
    }

    fn empty_memory() -> Memory {
        Memory::load(std::path::Path::new("."), &json!({"regions": []}))
            .expect("create memory view without mappings")
    }

    #[test]
    fn reset_undoes_allocation_free_and_reuse() {
        let baseline = baseline();
        let mut heap = GuestHeap::from_json(&baseline).expect("restore allocator baseline");
        let mut memory = empty_memory();
        let original = heap.snapshot();

        let reused = heap.alloc(8, 8, &mut memory).expect("reuse free bucket");
        assert_eq!(reused, 0x5000_0040);
        heap.free_ptr(0x5000_0000)
            .expect("free a baseline allocation");
        heap.free_ptr(reused)
            .expect("return the size-16 allocation");
        assert_eq!(heap.alloc(8, 8, &mut memory).unwrap(), reused);
        heap.reset();

        assert_eq!(heap.snapshot(), original);
        assert!(heap.is_live(0x5000_0000));

        let primary = heap.alloc(24, 8, &mut memory).unwrap();
        assert_eq!(primary, 0x5000_0100);
        heap.free_ptr(primary).unwrap();
        assert_eq!(heap.alloc(24, 8, &mut memory).unwrap(), primary);
        heap.reset();
        assert_eq!(heap.snapshot(), original);
    }

    #[test]
    fn reset_discards_journal_after_undoing_checkpoint_extra_arena_change() {
        let baseline = baseline();
        let mut heap = GuestHeap::from_json(&baseline).expect("restore allocator baseline");
        let mut memory = empty_memory();
        let original = heap.snapshot();

        // Fill the primary arena so the checkpoint's existing extra arena is
        // selected. Allocator arithmetic itself does not touch guest memory.
        heap.pointer = heap.base + heap.capacity;
        // The baseline has a free size-16 block. Request another size class so
        // this exercises arena expansion rather than free-bucket reuse.
        heap.alloc(32, 8, &mut memory).unwrap();
        assert_eq!(heap.extras[0].pointer, 0x6000_0120);
        heap.reset();
        assert_eq!(heap.snapshot(), original);
        assert!(heap.journal.is_empty());
    }
}
