use crate::{cpu::Cpu, generated, heap::GuestHeap, memory::Memory, runtime::Runtime};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

pub const SOURCE_SHA256: &str = "79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6";
pub const CHECKPOINT_SHA256: &str =
    "66f9bf75bfde9b5e0c1f4056573079016d41576020018b9e57fca538fd1b840e";
pub fn num(v: &Value) -> Result<u64, String> {
    if let Some(n) = v.as_u64() {
        Ok(n)
    } else if let Some(s) = v.as_str() {
        if let Some(s) = s.strip_prefix("0x") {
            u64::from_str_radix(s, 16).map_err(|e| e.to_string())
        } else {
            s.parse().map_err(|e| format!("integer: {e}"))
        }
    } else {
        Err("expected integer".into())
    }
}
pub fn sha256(path: &Path) -> Result<String, String> {
    let out = Command::new("sha256sum")
        .arg(path)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!("hash {} failed", path.display()));
    }
    Ok(String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string())
}

pub struct Session {
    pub cpu: Cpu,
    pub runtime: Runtime,
    pub checkpoint: Value,
    pub checkpoint_dir: PathBuf,
    initial_cpu: Cpu,
    pub world: u64,
    pub battle: u64,
    pub harness: u64,
}
impl Session {
    pub fn load(dir: &Path) -> Result<Self, String> {
        if sha256(&dir.join("checkpoint.json"))? != CHECKPOINT_SHA256 {
            return Err("unsupported initial checkpoint identity".into());
        }
        let checkpoint: Value = serde_json::from_slice(
            &std::fs::read(dir.join("checkpoint.json")).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let source =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/libg-cipher/libg.decrypted.so");
        if checkpoint["source_sha256"] != SOURCE_SHA256 || sha256(&source)? != SOURCE_SHA256 {
            return Err("source engine identity mismatch".into());
        }
        for r in checkpoint["regions"].as_array().ok_or("missing regions")? {
            let path = dir.join(r["file"].as_str().ok_or("region missing file")?);
            if sha256(&path)? != r["sha256"].as_str().ok_or("region missing hash")? {
                return Err(format!("checkpoint region changed: {}", path.display()));
            }
        }
        let mut cpu = Cpu::default();
        for (name, value) in checkpoint["registers"]
            .as_object()
            .ok_or("missing registers")?
        {
            let value = u128::from_str_radix(
                value
                    .as_str()
                    .ok_or("register must be hex")?
                    .trim_start_matches("0x"),
                16,
            )
            .map_err(|e| e.to_string())?;
            if let Some(i) = name.strip_prefix('x') {
                cpu.x[i.parse::<usize>().map_err(|e| e.to_string())?] = value as u64;
            } else if let Some(i) = name.strip_prefix('q') {
                cpu.q[i.parse::<usize>().map_err(|e| e.to_string())?] = value;
            } else {
                match name.as_str() {
                    "sp" => cpu.sp = value as u64,
                    "pc" => cpu.pc = value as u64,
                    "nzcv" => cpu.nzcv = value as u64,
                    "tpidr_el0" => cpu.tpidr_el0 = value as u64,
                    "fpcr" => cpu.fpcr = value as u64,
                    "fpsr" => cpu.fpsr = value as u64,
                    "cpacr_el1" => cpu.cpacr_el1 = value as u64,
                    _ => return Err(format!("unknown register {name}")),
                }
            }
        }
        let abi = &checkpoint["emulation_abi"];
        let mut memory = Memory::load(dir, &checkpoint)?;
        memory.map_zeroed(
            num(&abi["external_base"])?,
            num(&abi["external_size"])? as usize,
        )?;
        memory.map_zeroed(num(&abi["return_address"])? & !0xfff, 4096)?;
        let heap = GuestHeap::from_json(&checkpoint["native_heap"])?;
        let imports = abi["import_symbols"]
            .as_object()
            .ok_or("missing imports")?
            .iter()
            .map(|(k, v)| {
                Ok((
                    num(&Value::String(k.clone()))?,
                    v.as_str().ok_or("import symbol")?.to_string(),
                ))
            })
            .collect::<Result<BTreeMap<_, _>, String>>()?;
        let runtime = Runtime {
            memory,
            heap,
            imports,
            baseline_clock: checkpoint["import_counts"]["clock_gettime"]
                .as_u64()
                .unwrap_or(0),
            clock_calls: 0,
            calls: BTreeMap::new(),
        };
        let values = &checkpoint["call_plan"]["values"];
        let (world, battle, harness) = (
            num(&values["battle_world"])?,
            num(&values["battle_state"])?,
            num(&values["battle_harness"])?,
        );
        Ok(Self {
            initial_cpu: cpu.clone(),
            cpu,
            runtime,
            checkpoint,
            checkpoint_dir: dir.to_owned(),
            world,
            battle,
            harness,
        })
    }
    pub fn reset(&mut self) -> Result<(), String> {
        self.runtime.memory.reset()?;
        self.cpu = self.initial_cpu.clone();
        self.runtime.heap.reset();
        self.runtime.clock_calls = 0;
        self.runtime.calls.clear();
        Ok(())
    }
    pub fn call(&mut self, rva: u64, args: &[u64], x8: u64) -> Result<u64, String> {
        if args.len() > 8 {
            return Err("too many arguments".into());
        }
        self.cpu.x[..8].fill(0);
        self.cpu.x[..args.len()].copy_from_slice(args);
        self.cpu.x[8] = x8;
        self.cpu.sp = num(&self.checkpoint["emulation_abi"]["call_stack_pointer"])?;
        let ret = num(&self.checkpoint["emulation_abi"]["return_address"])?;
        self.cpu.x[30] = ret;
        self.cpu.pc = 0x10000000 + rva;
        unsafe {
            generated::dispatch(&mut self.cpu, &mut self.runtime, 0x10000000 + rva)?;
        }
        if self.cpu.pc != ret {
            return Err(format!(
                "function {rva:#x} returned to {:#x}, expected {ret:#x}",
                self.cpu.pc
            ));
        }
        Ok(self.cpu.x[0])
    }
    pub fn ticks(&self) -> [u32; 2] {
        unsafe {
            [
                self.runtime.memory.read(self.world + 0x60),
                self.runtime.memory.read(self.world + 0x70),
            ]
        }
    }
    pub fn is_terminal(&self) -> bool {
        unsafe { self.runtime.memory.read::<u8>(self.battle + 0x1e0) != 0 }
    }
    pub fn world_step(&mut self) -> Result<(), String> {
        self.call(0x10ca81c, &[self.world], 0)?;
        Ok(())
    }
    pub fn native_match(&mut self) -> Result<Value, String> {
        let slot = self.runtime.heap.alloc(8, 16, &mut self.runtime.memory)?;
        self.runtime.memory.write_bytes(slot, &[0; 8])?;
        let before = self.ticks();
        let now = Instant::now();
        self.call(0x11d6364, &[self.harness], slot)?;
        let elapsed = now.elapsed().as_secs_f64();
        let event = unsafe { self.runtime.memory.read::<u64>(slot) };
        let winner = unsafe { self.runtime.memory.read::<i32>(self.battle + 0x1b8) };
        if !self.is_terminal() || event == 0 {
            return Err("native match did not terminate".into());
        }
        Ok(
            json!({"engine":"compiled Rust routines; no Unicorn","ticks_before":before,"ticks_after":self.ticks(),"winner_side":winner,"terminated":true,"event":format!("{event:#x}"),"event_code":unsafe{self.runtime.memory.read::<u32>(event)},"elapsed_seconds":elapsed}),
        )
    }
    pub fn audit(&self, out: &Path) -> Result<(), String> {
        if out.exists() {
            return Err(format!("audit output already exists: {}", out.display()));
        }
        std::fs::create_dir_all(out.join("regions")).map_err(|e| e.to_string())?;
        let heap = self.runtime.heap.snapshot();
        let mut rows = Vec::new();
        for r in self.checkpoint["regions"].as_array().unwrap() {
            let name = r["file"].as_str().unwrap();
            let base = num(&r["base"])?;
            let size = if name == "heap.bin" {
                (num(&heap["primary_pointer"])? - num(&heap["base"])? + 4095) & !4095
            } else {
                num(&r["size"])?
            };
            let path = out.join("regions").join(name);
            std::fs::write(&path, self.runtime.memory.read_bytes(base, size as usize)?)
                .map_err(|e| e.to_string())?;
            rows.push(json!({"file":format!("regions/{name}"),"base":format!("{base:#x}"),"size":size,"sha256":sha256(&path)?}));
        }
        let mut registers = serde_json::Map::new();
        for i in 0..31 {
            registers.insert(format!("x{i}"), json!(format!("{:#x}", self.cpu.x[i])));
        }
        for i in 0..32 {
            registers.insert(format!("q{i}"), json!(format!("{:#x}", self.cpu.q[i])));
        }
        for (name, v) in [
            ("sp", self.cpu.sp),
            ("pc", self.cpu.pc),
            ("nzcv", self.cpu.nzcv),
            ("fpcr", self.cpu.fpcr),
            ("fpsr", self.cpu.fpsr),
            ("tpidr_el0", self.cpu.tpidr_el0),
            ("cpacr_el1", self.cpu.cpacr_el1),
        ] {
            registers.insert(name.into(), json!(format!("{v:#x}")));
        }
        std::fs::write(
            out.join("allocator.json"),
            serde_json::to_vec_pretty(&heap).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        let executable = PathBuf::from(format!("/proc/{}/exe", std::process::id()));
        let meta = json!({"source_sha256":SOURCE_SHA256,"checkpoint":self.checkpoint_dir,"checkpoint_sha256":CHECKPOINT_SHA256,"regions":rows,"registers":registers,"allocator_file":"allocator.json","ticks":self.ticks(),"imports_called":self.runtime.calls,"executable_sha256":sha256(&executable)?,"build_provenance":serde_json::from_str::<Value>(crate::BUILD_PROVENANCE).map_err(|e|e.to_string())?});
        std::fs::write(
            out.join("audit.json"),
            serde_json::to_vec_pretty(&meta).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
