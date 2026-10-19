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
#[derive(Debug)]
pub struct GuestHeap {
    base: u64,
    capacity: u64,
    pointer: u64,
    allocations: BTreeMap<u64, u64>,
    free: BTreeMap<u64, Vec<u64>>,
    extras: Vec<Arena>,
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
            free,
            extras,
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
                self.allocations.insert(pointer, size);
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
                self.allocations.insert(aligned, size);
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
        self.allocations.insert(pointer, size);
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
        self.free
            .entry(size_class(size)?)
            .or_default()
            .push(pointer);
        Ok(())
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
