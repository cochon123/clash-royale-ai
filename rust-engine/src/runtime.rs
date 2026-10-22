//! Explicit host imports used by the AOT translation backend.

use crate::cpu::Cpu;
use crate::heap::GuestHeap;
use crate::memory::Memory;
use std::collections::BTreeMap;

pub struct Runtime {
    pub memory: Memory,
    pub heap: GuestHeap,
    pub imports: BTreeMap<u64, String>,
    pub baseline_clock: u64,
    pub clock_calls: u64,
    pub calls: BTreeMap<String, u64>,
    /// Temporary host-side deck-vector substitution at the same two original
    /// instruction addresses used by the verified reference's code hooks.
    pub deck_swap: Option<DeckSwap>,
}

pub struct DeckSwap {
    pub slot: u64,
    pub second_ptr: u64,
    pub second_count: u32,
    pub original_ptr: u64,
    pub original_count: u32,
    pub calls: u32,
}

impl Runtime {
    pub fn deck_swap_hook(&mut self, address: u64) -> Result<(), String> {
        let Some(state) = self.deck_swap.as_mut() else { return Ok(()); };
        let (pointer, count) = match address {
            0x111d6f18 => {
                state.calls += 1;
                if state.calls != 2 { return Ok(()); }
                (state.second_ptr, state.second_count)
            }
            0x111d6068 if state.calls == 2 => (state.original_ptr, state.original_count),
            _ => return Ok(()),
        };
        self.memory.write_bytes(state.slot, &pointer.to_le_bytes())?;
        self.memory.write_bytes(state.slot + 8, &count.to_le_bytes())?;
        self.memory.write_bytes(state.slot + 12, &count.to_le_bytes())
    }

    /// Execute a modeled host import at `address` and return through X30.
    /// Returns `false` when the address is not an import.
    pub fn imported(&mut self, cpu: &mut Cpu, address: u64) -> Result<bool, String> {
        let Some(name) = self.imports.get(&address).cloned() else {
            return Ok(false);
        };
        *self.calls.entry(name.clone()).or_default() += 1;
        let a0 = cpu.x[0];
        let a1 = cpu.x[1];
        let a2 = cpu.x[2];
        let result = match name.as_str() {
            "malloc" => self.heap.alloc(a0, 16, &mut self.memory)?,
            "free" => {
                self.heap.free_ptr(a0)?;
                0
            }
            "memcpy" | "memmove" => {
                let n = usize::try_from(a2).map_err(|_| "copy length too large")?;
                if n > 256 * 1024 * 1024 {
                    return Err(format!("{name} request too large: {n}"));
                }
                if n != 0 {
                    let bytes = self.memory.read_bytes(a1, n)?;
                    self.memory.write_bytes(a0, &bytes)?;
                }
                a0
            }
            "memset" => {
                let n = usize::try_from(a2).map_err(|_| "set length too large")?;
                if n > 256 * 1024 * 1024 {
                    return Err(format!("memset request too large: {n}"));
                }
                let bytes = vec![a1 as u8; n];
                self.memory.write_bytes(a0, &bytes)?;
                a0
            }
            "memcmp" => {
                let n = usize::try_from(a2).map_err(|_| "compare length too large")?;
                if n > 64 * 1024 * 1024 {
                    return Err(format!("memcmp request too large: {n}"));
                }
                let left = if n == 0 {
                    Vec::new()
                } else {
                    self.memory.read_bytes(a0, n)?
                };
                let right = if n == 0 {
                    Vec::new()
                } else {
                    self.memory.read_bytes(a1, n)?
                };
                match left.iter().zip(&right).find(|(x, y)| x != y) {
                    Some((x, y)) => (*x as i32 - *y as i32) as i64 as u64,
                    None => 0,
                }
            }
            "strlen" | "strnlen" => {
                let limit = if name == "strlen" {
                    16 * 1024 * 1024
                } else {
                    usize::try_from(a1.min(16 * 1024 * 1024)).map_err(|_| "length overflow")?
                };
                self.cstring_bounded(a0, limit)?.len() as u64
            }
            "strcmp" => {
                let left = self.cstring(a0, 16 * 1024 * 1024)?;
                let right = self.cstring(a1, 16 * 1024 * 1024)?;
                let common = left.len().min(right.len());
                if let Some(i) = (0..common).find(|&i| left[i] != right[i]) {
                    (left[i] as i32 - right[i] as i32) as i64 as u64
                } else {
                    (left.len() as i64 - right.len() as i64) as u64
                }
            }
            "clock_gettime" => {
                if !matches!(a0, 0 | 1 | 4 | 7) {
                    return Err(format!("unmodeled clock ID {a0}"));
                }
                self.clock_calls += 1;
                let nanos = (self.baseline_clock + self.clock_calls) * 1_000_000;
                let mut sec = nanos / 1_000_000_000;
                if a0 == 0 {
                    sec += 1_700_000_000;
                }
                let ns = nanos % 1_000_000_000;
                let mut out = [0u8; 16];
                out[..8].copy_from_slice(&sec.to_le_bytes());
                out[8..].copy_from_slice(&ns.to_le_bytes());
                self.memory.write_bytes(a1, &out)?;
                0
            }
            "vsnprintf" => self.format_arm64(a0, a1, a2, cpu.x[3])? as i64 as u64,
            _ => return Err(format!("unsupported native import {name} at {address:#x}")),
        };
        cpu.x[0] = result;
        cpu.pc = cpu.x[30];
        Ok(true)
    }

    fn cstring(&self, address: u64, max: usize) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        while out.len() < max {
            let n = (max - out.len()).min(256);
            let bytes = self.memory.read_bytes(address + out.len() as u64, n)?;
            if let Some(end) = bytes.iter().position(|b| *b == 0) {
                out.extend_from_slice(&bytes[..end]);
                return Ok(out);
            }
            out.extend_from_slice(&bytes);
        }
        Err(format!(
            "unterminated C string at {address:#x} within {max} bytes"
        ))
    }

    fn cstring_bounded(&self, address: u64, max: usize) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        while out.len() < max {
            let n = (max - out.len()).min(256);
            let bytes = self.memory.read_bytes(address + out.len() as u64, n)?;
            if let Some(end) = bytes.iter().position(|b| *b == 0) {
                out.extend_from_slice(&bytes[..end]);
                return Ok(out);
            }
            out.extend_from_slice(&bytes);
        }
        Ok(out)
    }

    fn format_arm64(
        &mut self,
        dst: u64,
        capacity: u64,
        fmt_ptr: u64,
        va_ptr: u64,
    ) -> Result<i32, String> {
        if capacity > 1_048_576 {
            return Err(format!("vsnprintf output bound too large: {capacity}"));
        }
        let fmt = self.cstring(fmt_ptr, 4096)?;
        let layout = self.memory.read_bytes(va_ptr, 32)?;
        let u64at = |off: usize| u64::from_le_bytes(layout[off..off + 8].try_into().unwrap());
        let mut stack = u64at(0);
        let gr_top = u64at(8);
        let mut gr_off = i32::from_le_bytes(layout[24..28].try_into().unwrap());
        let next_arg =
            |memory: &Memory, stack: &mut u64, gr_off: &mut i32| -> Result<u64, String> {
                let p = if *gr_off < 0 {
                    let p = (gr_top as i64 + *gr_off as i64) as u64;
                    *gr_off += 8;
                    p
                } else {
                    let p = *stack;
                    *stack = stack.checked_add(8).ok_or("va_list stack overflow")?;
                    p
                };
                let bytes = memory.read_bytes(p, 8)?;
                Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
            };
        let mut out = Vec::new();
        let mut i = 0;
        while i < fmt.len() {
            if fmt[i] != b'%' {
                out.push(fmt[i]);
                i += 1;
                continue;
            }
            i += 1;
            if i >= fmt.len() {
                return Err("trailing % in vsnprintf format".into());
            }
            match fmt[i] {
                b'%' => {
                    out.push(b'%');
                    i += 1;
                }
                b's' => {
                    let p = next_arg(&self.memory, &mut stack, &mut gr_off)?;
                    if p == 0 {
                        out.extend_from_slice(b"(null)");
                    } else {
                        out.extend_from_slice(&self.cstring(p, 1_048_576)?);
                    }
                    i += 1;
                }
                b'i' | b'd' => {
                    let v = next_arg(&self.memory, &mut stack, &mut gr_off)? as u32 as i32;
                    out.extend_from_slice(v.to_string().as_bytes());
                    i += 1;
                }
                other => {
                    return Err(format!(
                        "unsupported vsnprintf conversion %{}",
                        other as char
                    ))
                }
            }
        }
        let cap = capacity as usize;
        if cap > 0 {
            let n = out.len().min(cap - 1);
            if dst == 0 {
                return Err("vsnprintf has null destination with nonzero capacity".into());
            }
            let mut bytes = out[..n].to_vec();
            bytes.push(0);
            self.memory.write_bytes(dst, &bytes)?;
        }
        Ok(out.len().try_into().unwrap_or(i32::MAX))
    }
}
