//! Exact session images as a dirty-page diff against the reset checkpoint.

use crate::cpu::Cpu;
use crate::heap::{GuestHeap, HeapDelta};
use crate::memory::DirtyPages;
use crate::runtime::DeckSwap;
use crate::session::{sha256, Session, CHECKPOINT_SHA256};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const MAGIC: &[u8] = b"CRSNAP01";

/// Full checkpoint image, or the net edit against the loaded checkpoint.
pub(crate) enum SnapshotHeap {
    Full(GuestHeap),
    Delta(HeapDelta),
}

/// Guest memory, architectural state, allocator, and protocol bookkeeping.
pub struct EngineSnapshot {
    pub tick: u64,
    pub bytes: u64,
    pub cpu: Cpu,
    pub clock_calls: u64,
    pub calls: BTreeMap<String, u64>,
    pub deck_swap: Option<DeckSwap>,
    pub world: u64,
    pub battle: u64,
    pub harness: u64,
    pub descriptor_scratch: u64,
    pub dirty: DirtyPages,
    pub protocol: Value,
    pub(crate) heap: SnapshotHeap,
}

impl EngineSnapshot {
    pub fn from_live(
        engine: &Session,
        descriptor_scratch: u64,
        tick: u64,
        protocol: Value,
    ) -> Result<Self, String> {
        let dirty = engine.runtime.memory.capture_dirty()?;
        let bytes = dirty.payload_bytes();
        Ok(Self {
            tick,
            bytes,
            cpu: engine.cpu.clone(),
            clock_calls: engine.runtime.clock_calls,
            calls: engine.runtime.calls.clone(),
            deck_swap: engine.runtime.deck_swap.clone(),
            world: engine.world,
            battle: engine.battle,
            harness: engine.harness,
            descriptor_scratch,
            dirty,
            protocol,
            heap: SnapshotHeap::Delta(engine.runtime.heap.capture_delta()),
        })
    }

    /// Put this image into a session that was loaded from the same checkpoint.
    /// Guest pages are reset to the checkpoint and then the dirty diff is written.
    pub fn restore_into(
        &self,
        engine: &mut Session,
        descriptor_scratch: &mut u64,
    ) -> Result<(), String> {
        engine.runtime.memory.apply_dirty(&self.dirty)?;
        engine.cpu = self.cpu.clone();
        match &self.heap {
            SnapshotHeap::Full(heap) => engine.runtime.heap = heap.clone(),
            SnapshotHeap::Delta(delta) => engine.runtime.heap.restore_delta(delta)?,
        }
        engine.runtime.clock_calls = self.clock_calls;
        engine.runtime.calls = self.calls.clone();
        engine.runtime.deck_swap = self.deck_swap.clone();
        engine.world = self.world;
        engine.battle = self.battle;
        engine.harness = self.harness;
        *descriptor_scratch = self.descriptor_scratch;
        Ok(())
    }

    /// Write the image. Returns the file size in bytes.
    pub fn save(&self, path: &Path) -> Result<u64, String> {
        if path.exists() {
            return Err(format!("snapshot path already exists: {}", path.display()));
        }
        let header = self.header_json()?;
        let header_bytes = serde_json::to_vec(&header).map_err(|e| e.to_string())?;
        let tmp = path.with_extension("crsnap-partial");
        let mut file = File::create(&tmp).map_err(|e| format!("{}: {e}", tmp.display()))?;
        file.write_all(MAGIC).map_err(|e| e.to_string())?;
        file.write_all(&(header_bytes.len() as u64).to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&header_bytes).map_err(|e| e.to_string())?;
        for (_address, page) in &self.dirty.pages {
            file.write_all(page).map_err(|e| e.to_string())?;
        }
        file.sync_all().map_err(|e| e.to_string())?;
        drop(file);
        std::fs::rename(&tmp, path).map_err(|e| format!("rename snapshot: {e}"))?;
        let len = std::fs::metadata(path)
            .map_err(|e| e.to_string())?
            .len();
        Ok(len)
    }

    /// Load an image after checking the pinned checkpoint and this executable.
    pub fn load(path: &Path) -> Result<Self, String> {
        let mut file = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut magic = [0u8; 8];
        file.read_exact(&mut magic)
            .map_err(|e| format!("snapshot header: {e}"))?;
        if magic != MAGIC {
            return Err("snapshot file is not a CRSNAP01 image".into());
        }
        let mut len_buf = [0u8; 8];
        file.read_exact(&mut len_buf).map_err(|e| e.to_string())?;
        let header_len = u64::from_le_bytes(len_buf);
        if header_len > 512 * 1024 * 1024 {
            return Err("snapshot header is too large".into());
        }
        let mut header_bytes = vec![0u8; header_len as usize];
        file.read_exact(&mut header_bytes)
            .map_err(|e| format!("snapshot header: {e}"))?;
        let header: Value = serde_json::from_slice(&header_bytes).map_err(|e| e.to_string())?;
        let version = header["version"].as_u64().unwrap_or(0);
        if version != 1 && version != 2 {
            return Err("unsupported snapshot version".into());
        }
        if header["checkpoint_sha256"].as_str() != Some(CHECKPOINT_SHA256) {
            return Err(
                "snapshot checkpoint hash does not match the pinned checkpoint".into(),
            );
        }
        let executable = format!("/proc/{}/exe", std::process::id());
        let binary = sha256(Path::new(&executable))?;
        if header["binary_sha256"].as_str() != Some(binary.as_str()) {
            return Err("snapshot binary hash does not match this executable".into());
        }
        let page_meta = header["pages"]
            .as_array()
            .ok_or("snapshot pages missing")?;
        let mut pages = Vec::with_capacity(page_meta.len());
        for page in page_meta {
            let address = parse_u64(&page["address"])?;
            let length = usize::try_from(parse_u64(&page["length"])?)
                .map_err(|_| "snapshot page length does not fit")?;
            if length != 4096 {
                return Err(format!("snapshot page {address:#x} is not 4096 bytes"));
            }
            let mut bytes = vec![0u8; length];
            file.read_exact(&mut bytes)
                .map_err(|e| format!("snapshot page {address:#x}: {e}"))?;
            pages.push((address, bytes));
        }
        let mut rest = [0u8; 1];
        match file.read(&mut rest) {
            Ok(0) => {}
            Ok(_) => return Err("snapshot file has trailing bytes".into()),
            Err(error) => return Err(error.to_string()),
        }
        let mappings = header["mappings"]
            .as_array()
            .ok_or("snapshot mappings missing")?
            .iter()
            .map(|mapping| {
                Ok((
                    parse_u64(&mapping["address"])?,
                    usize::try_from(parse_u64(&mapping["size"])?)
                        .map_err(|_| "mapping size does not fit")?,
                ))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let dirty = DirtyPages { pages, mappings };
        let bytes = dirty.payload_bytes();
        Ok(Self {
            tick: header["tick"].as_u64().ok_or("snapshot tick missing")?,
            bytes,
            cpu: cpu_from_json(&header["cpu"])?,
            clock_calls: header["clock_calls"]
                .as_u64()
                .ok_or("snapshot clock_calls missing")?,
            calls: header["calls"]
                .as_object()
                .ok_or("snapshot calls missing")?
                .iter()
                .map(|(name, count)| {
                    Ok((
                        name.clone(),
                        count.as_u64().ok_or("call count is not an integer")?,
                    ))
                })
                .collect::<Result<BTreeMap<_, _>, String>>()?,
            deck_swap: deck_from_json(&header["deck_swap"])?,
            world: parse_u64(&header["world"])?,
            battle: parse_u64(&header["battle"])?,
            harness: parse_u64(&header["harness"])?,
            descriptor_scratch: parse_u64(&header["descriptor_scratch"])?,
            dirty,
            protocol: header["protocol"].clone(),
            heap: if version == 1 {
                SnapshotHeap::Full(GuestHeap::import_full(&header["heap"])?)
            } else {
                SnapshotHeap::Delta(HeapDelta::from_json(&header["heap_delta"])?)
            },
        })
    }

    fn header_json(&self) -> Result<Value, String> {
        let executable = format!("/proc/{}/exe", std::process::id());
        let binary = sha256(Path::new(&executable))?;
        let (version, heap_json) = match &self.heap {
            SnapshotHeap::Full(heap) => (1, ("heap", heap.export_full())),
            SnapshotHeap::Delta(delta) => (2, ("heap_delta", delta.to_json())),
        };
        let mut header = json!({
            "version": version,
            "checkpoint_sha256": CHECKPOINT_SHA256,
            "binary_sha256": binary,
            "tick": self.tick,
            "world": self.world.to_string(),
            "battle": self.battle.to_string(),
            "harness": self.harness.to_string(),
            "descriptor_scratch": self.descriptor_scratch.to_string(),
            "clock_calls": self.clock_calls,
            "calls": self.calls,
            "deck_swap": deck_to_json(&self.deck_swap),
            "cpu": cpu_to_json(&self.cpu),
            "mappings": self.dirty.mappings.iter().map(|(address, size)| json!({
                "address": address.to_string(),
                "size": size.to_string(),
            })).collect::<Vec<_>>(),
            "pages": self.dirty.pages.iter().map(|(address, page)| json!({
                "address": address.to_string(),
                "length": page.len().to_string(),
            })).collect::<Vec<_>>(),
            "protocol": self.protocol,
        });
        header[heap_json.0] = heap_json.1;
        Ok(header)
    }
}

fn cpu_to_json(cpu: &Cpu) -> Value {
    json!({
        "x": cpu.x.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
        "q": cpu.q.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
        "sp": cpu.sp.to_string(),
        "pc": cpu.pc.to_string(),
        "nzcv": cpu.nzcv.to_string(),
        "tpidr_el0": cpu.tpidr_el0.to_string(),
        "fpcr": cpu.fpcr.to_string(),
        "fpsr": cpu.fpsr.to_string(),
        "cpacr_el1": cpu.cpacr_el1.to_string(),
    })
}

fn cpu_from_json(value: &Value) -> Result<Cpu, String> {
    let mut cpu = Cpu::default();
    let x = value["x"].as_array().ok_or("snapshot cpu.x missing")?;
    if x.len() != 31 {
        return Err("snapshot cpu.x must contain 31 registers".into());
    }
    for (index, item) in x.iter().enumerate() {
        cpu.x[index] = parse_u64(item)?;
    }
    let q = value["q"].as_array().ok_or("snapshot cpu.q missing")?;
    if q.len() != 32 {
        return Err("snapshot cpu.q must contain 32 registers".into());
    }
    for (index, item) in q.iter().enumerate() {
        let text = item.as_str().ok_or("snapshot q register is not a string")?;
        cpu.q[index] = text
            .parse::<u128>()
            .map_err(|e| format!("bad q register {text}: {e}"))?;
    }
    cpu.sp = parse_u64(&value["sp"])?;
    cpu.pc = parse_u64(&value["pc"])?;
    cpu.nzcv = parse_u64(&value["nzcv"])?;
    cpu.tpidr_el0 = parse_u64(&value["tpidr_el0"])?;
    cpu.fpcr = parse_u64(&value["fpcr"])?;
    cpu.fpsr = parse_u64(&value["fpsr"])?;
    cpu.cpacr_el1 = parse_u64(&value["cpacr_el1"])?;
    Ok(cpu)
}

fn deck_to_json(deck: &Option<DeckSwap>) -> Value {
    let Some(deck) = deck else {
        return Value::Null;
    };
    json!({
        "slot": deck.slot.to_string(),
        "second_ptr": deck.second_ptr.to_string(),
        "second_count": deck.second_count,
        "original_ptr": deck.original_ptr.to_string(),
        "original_count": deck.original_count,
        "calls": deck.calls,
    })
}

fn deck_from_json(value: &Value) -> Result<Option<DeckSwap>, String> {
    if value.is_null() {
        return Ok(None);
    }
    Ok(Some(DeckSwap {
        slot: parse_u64(&value["slot"])?,
        second_ptr: parse_u64(&value["second_ptr"])?,
        second_count: value["second_count"]
            .as_u64()
            .and_then(|v| u32::try_from(v).ok())
            .ok_or("snapshot deck second_count missing")?,
        original_ptr: parse_u64(&value["original_ptr"])?,
        original_count: value["original_count"]
            .as_u64()
            .and_then(|v| u32::try_from(v).ok())
            .ok_or("snapshot deck original_count missing")?,
        calls: value["calls"]
            .as_u64()
            .and_then(|v| u32::try_from(v).ok())
            .ok_or("snapshot deck calls missing")?,
    }))
}

fn parse_u64(value: &Value) -> Result<u64, String> {
    if let Some(number) = value.as_u64() {
        Ok(number)
    } else if let Some(text) = value.as_str() {
        text.parse::<u64>()
            .map_err(|e| format!("bad integer {text:?}: {e}"))
    } else {
        Err("expected integer string".into())
    }
}
