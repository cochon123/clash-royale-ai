//! Prepared-snapshot runner for the original ARM64 engine in Unicorn.
//!
//! This restores a source-bound native checkpoint and invokes original code. It is
//! intentionally a thin execution adapter, not a Rust battle implementation.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

const UC_ARCH_ARM64: c_int = 2;
const UC_MODE_ARM: u32 = 0;
const UC_PROT_ALL: u32 = 7;
const UC_HOOK_CODE: c_int = 1 << 2;
const UC_HOOK_BLOCK: c_int = 1 << 3;
const SOURCE_SHA256: &str = "79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6";

#[link(name = "dl")]
extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
}

#[derive(Clone)]
struct UnicornApi {
    _handle: *mut c_void,
    open: unsafe extern "C" fn(c_int, u32, *mut *mut c_void) -> c_int,
    close: unsafe extern "C" fn(*mut c_void) -> c_int,
    mem_map: unsafe extern "C" fn(*mut c_void, u64, usize, u32) -> c_int,
    mem_read: unsafe extern "C" fn(*mut c_void, u64, *mut c_void, usize) -> c_int,
    mem_write: unsafe extern "C" fn(*mut c_void, u64, *const c_void, usize) -> c_int,
    reg_read: unsafe extern "C" fn(*mut c_void, c_int, *mut c_void) -> c_int,
    reg_write: unsafe extern "C" fn(*mut c_void, c_int, *const c_void) -> c_int,
    emu_start: unsafe extern "C" fn(*mut c_void, u64, u64, u64, usize) -> c_int,
    emu_stop: unsafe extern "C" fn(*mut c_void) -> c_int,
    strerror: unsafe extern "C" fn(c_int) -> *const c_char,
    hook_add: unsafe extern "C" fn(
        *mut c_void,
        *mut usize,
        c_int,
        *const c_void,
        *mut c_void,
        u64,
        u64,
        ...
    ) -> c_int,
    hook_del: unsafe extern "C" fn(*mut c_void, usize) -> c_int,
}

impl UnicornApi {
    unsafe fn load(path: &Path) -> Result<Self, String> {
        let path_c =
            CString::new(path.as_os_str().as_encoded_bytes()).map_err(|e| e.to_string())?;
        let handle = dlopen(path_c.as_ptr(), 2); // RTLD_NOW
        if handle.is_null() {
            let e = dlerror();
            return Err(if e.is_null() {
                "dlopen failed".into()
            } else {
                CStr::from_ptr(e).to_string_lossy().into_owned()
            });
        }
        macro_rules! sym {
            ($name:literal, $ty:ty) => {{
                let p = dlsym(handle, concat!($name, "\0").as_ptr().cast());
                if p.is_null() {
                    return Err(format!("missing Unicorn symbol {}", $name));
                }
                std::mem::transmute::<*mut c_void, $ty>(p)
            }};
        }
        Ok(Self {
            _handle: handle,
            open: sym!(
                "uc_open",
                unsafe extern "C" fn(c_int, u32, *mut *mut c_void) -> c_int
            ),
            close: sym!("uc_close", unsafe extern "C" fn(*mut c_void) -> c_int),
            hook_del: sym!(
                "uc_hook_del",
                unsafe extern "C" fn(*mut c_void, usize) -> c_int
            ),
            mem_map: sym!(
                "uc_mem_map",
                unsafe extern "C" fn(*mut c_void, u64, usize, u32) -> c_int
            ),
            mem_read: sym!(
                "uc_mem_read",
                unsafe extern "C" fn(*mut c_void, u64, *mut c_void, usize) -> c_int
            ),
            mem_write: sym!(
                "uc_mem_write",
                unsafe extern "C" fn(*mut c_void, u64, *const c_void, usize) -> c_int
            ),
            reg_read: sym!(
                "uc_reg_read",
                unsafe extern "C" fn(*mut c_void, c_int, *mut c_void) -> c_int
            ),
            reg_write: sym!(
                "uc_reg_write",
                unsafe extern "C" fn(*mut c_void, c_int, *const c_void) -> c_int
            ),
            emu_start: sym!(
                "uc_emu_start",
                unsafe extern "C" fn(*mut c_void, u64, u64, u64, usize) -> c_int
            ),
            emu_stop: sym!("uc_emu_stop", unsafe extern "C" fn(*mut c_void) -> c_int),
            strerror: sym!("uc_strerror", unsafe extern "C" fn(c_int) -> *const c_char),
            hook_add: sym!(
                "uc_hook_add",
                unsafe extern "C" fn(
                    *mut c_void,
                    *mut usize,
                    c_int,
                    *const c_void,
                    *mut c_void,
                    u64,
                    u64,
                    ...
                ) -> c_int
            ),
        })
    }
    fn err(&self, code: c_int) -> String {
        unsafe {
            let p = (self.strerror)(code);
            if p.is_null() {
                format!("Unicorn error {code}")
            } else {
                CStr::from_ptr(p).to_string_lossy().into_owned()
            }
        }
    }
}

#[derive(Clone)]
struct Arena {
    base: u64,
    capacity: u64,
    pointer: u64,
}
#[derive(Clone)]
struct GuestHeap {
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
    fn from_json(v: &Value) -> Result<Self, String> {
        let int = |x: &Value| -> Result<u64, String> {
            if let Some(s) = x.as_str() {
                u64::from_str_radix(s.trim_start_matches("0x"), 16)
                    .or_else(|_| s.parse())
                    .map_err(|e| e.to_string())
            } else {
                x.as_u64().ok_or("expected integer".into())
            }
        };
        let base = int(&v["base"])?;
        let capacity = int(&v["capacity"])?;
        let pointer = int(&v["primary_pointer"])?;
        if pointer < base || pointer > base + capacity {
            return Err("checkpoint heap pointer out of range".into());
        }
        let mut allocations = BTreeMap::new();
        for (k, val) in v["allocations"]
            .as_object()
            .ok_or("heap allocations missing")?
        {
            allocations.insert(
                k.parse()
                    .map_err(|e| format!("bad allocation address: {e}"))?,
                int(val)?,
            );
        }
        let mut free = BTreeMap::new();
        if let Some(m) = v["free_buckets"].as_object() {
            for (k, items) in m {
                let size = k.parse().map_err(|e| format!("bad free bucket: {e}"))?;
                let arr = items
                    .as_array()
                    .ok_or("free bucket not array")?
                    .iter()
                    .map(int)
                    .collect::<Result<Vec<_>, _>>()?;
                free.insert(size, arr);
            }
        }
        let mut extras = Vec::new();
        if let Some(a) = v["extra_heaps"].as_array() {
            for x in a {
                extras.push(Arena {
                    base: int(&x["base"])?,
                    capacity: int(&x["capacity"])?,
                    pointer: int(&x["pointer"])?,
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
    fn size_class(n: u64) -> Result<u64, String> {
        n.checked_add(15)
            .map(|x| x & !15)
            .map(|x| x.max(16))
            .ok_or("allocation size overflow".into())
    }
    fn alloc(
        &mut self,
        size: u64,
        alignment: u64,
        api: &UnicornApi,
        uc: *mut c_void,
    ) -> Result<u64, String> {
        if alignment < 8 || !alignment.is_power_of_two() {
            return Err(format!("invalid guest alignment {alignment}"));
        }
        let class = Self::size_class(size)?;
        if let Some(bucket) = self.free.get_mut(&class) {
            if let Some(pos) = bucket.iter().rposition(|p| p % alignment == 0) {
                let p = bucket.remove(pos);
                if bucket.is_empty() {
                    self.free.remove(&class);
                }
                self.allocations.insert(p, size);
                return Ok(p);
            }
        }
        let mut regions = Vec::with_capacity(self.extras.len() + 1);
        regions.push((self.base, self.capacity, self.pointer, None));
        regions.extend(
            self.extras
                .iter()
                .enumerate()
                .map(|(i, a)| (a.base, a.capacity, a.pointer, Some(i))),
        );
        for (base, cap, pointer, index) in regions {
            let aligned = align(pointer, alignment)?;
            let end = aligned.checked_add(class).ok_or("guest heap overflow")?;
            if end <= base + cap {
                if let Some(i) = index {
                    self.extras[i].pointer = end
                } else {
                    self.pointer = end
                }
                self.allocations.insert(aligned, size);
                return Ok(aligned);
            }
        }
        if self.extras.len() >= 4 {
            return Err("guest heap exhausted primary and four extra arenas".into());
        }
        let i = self.extras.len();
        let base = Self::EXTRA_BASE + i as u64 * Self::EXTRA_STEP;
        let rc = unsafe { (api.mem_map)(uc, base, Self::EXTRA_CAPACITY as usize, UC_PROT_ALL) };
        if rc != 0 {
            return Err(format!("mapping extra heap: {}", api.err(rc)));
        }
        let p = align(base, alignment)?;
        let end = p.checked_add(class).ok_or("guest extra heap overflow")?;
        if end > base + Self::EXTRA_CAPACITY {
            return Err("allocation does not fit extra heap".into());
        }
        self.extras.push(Arena {
            base,
            capacity: Self::EXTRA_CAPACITY,
            pointer: end,
        });
        self.allocations.insert(p, size);
        Ok(p)
    }
    fn free_ptr(&mut self, p: u64) -> Result<(), String> {
        if p == 0 {
            return Ok(());
        }
        let n = self
            .allocations
            .remove(&p)
            .ok_or_else(|| format!("free of unknown/double-free pointer {p:#x}"))?;
        let class = Self::size_class(n)?;
        self.free.entry(class).or_default().push(p);
        Ok(())
    }
}
fn align(v: u64, a: u64) -> Result<u64, String> {
    v.checked_add(a - 1)
        .map(|x| x & !(a - 1))
        .ok_or("alignment overflow".into())
}
fn engine_root() -> PathBuf {
    std::env::var_os("CLASH_ENGINE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(".."))
}

/// Host paths recorded in a checkpoint before clash-rust moved into
/// clash-royale-ai/engine. A missing path under the old checkout is remapped to
/// this engine root; callers still verify the file's recorded SHA-256.
fn recorded_host_path(recorded: &str) -> PathBuf {
    let path = PathBuf::from(recorded);
    if path.exists() {
        return path;
    }
    match recorded.split_once("/clash-rust/") {
        Some((_, relative)) => engine_root().join(relative),
        None => path,
    }
}

fn sha256(path: &Path) -> Result<String, String> {
    let out = Command::new("sha256sum")
        .arg(path)
        .output()
        .map_err(|e| format!("sha256sum: {e}"))?;
    if !out.status.success() {
        return Err(format!("sha256sum failed for {}", path.display()));
    }
    let s = String::from_utf8_lossy(&out.stdout);
    Ok(s.split_whitespace().next().unwrap_or("").to_string())
}
fn num(v: &Value) -> Result<u64, String> {
    if let Some(n) = v.as_u64() {
        Ok(n)
    } else if let Some(s) = v.as_str() {
        if let Some(x) = s.strip_prefix("0x") {
            u64::from_str_radix(x, 16).map_err(|e| e.to_string())
        } else {
            s.parse::<u64>().map_err(|e| e.to_string())
        }
    } else {
        Err("expected integer/string address".into())
    }
}
fn value_u128(v: &Value) -> Result<u128, String> {
    let s = v.as_str().ok_or("register value must be hex string")?;
    u128::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|e| e.to_string())
}
fn write_mem(api: &UnicornApi, uc: *mut c_void, address: u64, bytes: &[u8]) -> Result<(), String> {
    let rc = unsafe { (api.mem_write)(uc, address, bytes.as_ptr().cast(), bytes.len()) };
    if rc == 0 {
        Ok(())
    } else {
        Err(format!("mem_write {address:#x}: {}", api.err(rc)))
    }
}
fn read_mem(api: &UnicornApi, uc: *mut c_void, address: u64, n: usize) -> Result<Vec<u8>, String> {
    let mut b = vec![0; n];
    let rc = unsafe { (api.mem_read)(uc, address, b.as_mut_ptr().cast(), n) };
    if rc == 0 {
        Ok(b)
    } else {
        Err(format!("mem_read {address:#x}+{n:#x}: {}", api.err(rc)))
    }
}
fn read_mem_u64(api: &UnicornApi, uc: *mut c_void, address: u64) -> Result<u64, String> {
    Ok(u64::from_le_bytes(
        read_mem(api, uc, address, 8)?.try_into().unwrap(),
    ))
}
fn read_mem_u32(api: &UnicornApi, uc: *mut c_void, address: u64) -> Result<u32, String> {
    Ok(u32::from_le_bytes(
        read_mem(api, uc, address, 4)?.try_into().unwrap(),
    ))
}
fn read_mem_i32(api: &UnicornApi, uc: *mut c_void, address: u64) -> Result<i32, String> {
    Ok(i32::from_le_bytes(
        read_mem(api, uc, address, 4)?.try_into().unwrap(),
    ))
}
fn read_u64(api: &UnicornApi, uc: *mut c_void, id: i32) -> Result<u64, String> {
    let mut v = 0u64;
    let rc = unsafe { (api.reg_read)(uc, id, (&mut v as *mut u64).cast()) };
    if rc == 0 {
        Ok(v)
    } else {
        Err(format!("reg_read {id}: {}", api.err(rc)))
    }
}
fn write_u64(api: &UnicornApi, uc: *mut c_void, id: i32, v: u64) -> Result<(), String> {
    let rc = unsafe { (api.reg_write)(uc, id, (&v as *const u64).cast()) };
    if rc == 0 {
        Ok(())
    } else {
        Err(format!("reg_write {id}: {}", api.err(rc)))
    }
}

struct HookState {
    api: *const UnicornApi,
    uc: *mut c_void,
    imports: BTreeMap<u64, String>,
    ids: BTreeMap<String, i32>,
    heap: GuestHeap,
    baseline_clock: u64,
    clock_calls: u64,
    calls: BTreeMap<String, u64>,
    error: Option<String>,
    returned: bool,
    return_address: u64,
}
unsafe fn arg(s: &HookState, i: usize) -> Result<u64, String> {
    read_u64(
        &*s.api,
        s.uc,
        *s.ids.get(&format!("x{i}")).ok_or("missing register ID")?,
    )
}
unsafe fn ret(s: &HookState, v: u64) -> Result<(), String> {
    write_u64(&*s.api, s.uc, *s.ids.get("x0").ok_or("missing x0")?, v)?;
    let lr = arg(s, 30)?;
    write_u64(&*s.api, s.uc, *s.ids.get("pc").ok_or("missing pc")?, lr)
}
unsafe fn cstring(s: &HookState, p: u64, max: usize) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    while out.len() < max {
        let n = (max - out.len()).min(256);
        let b = read_mem(&*s.api, s.uc, p + out.len() as u64, n)?;
        if let Some(i) = b.iter().position(|x| *x == 0) {
            out.extend_from_slice(&b[..i]);
            return Ok(out);
        }
        out.extend_from_slice(&b);
    }
    Err(format!(
        "unterminated C string at {p:#x} within {max} bytes"
    ))
}
unsafe fn cstring_bounded(s: &HookState, p: u64, max: usize) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    while out.len() < max {
        let n = (max - out.len()).min(256);
        let b = read_mem(&*s.api, s.uc, p + out.len() as u64, n)?;
        if let Some(i) = b.iter().position(|x| *x == 0) {
            out.extend_from_slice(&b[..i]);
            return Ok(out);
        }
        out.extend_from_slice(&b);
    }
    Ok(out)
}
unsafe fn format_arm64(
    s: &HookState,
    dst: u64,
    capacity: u64,
    fmt_ptr: u64,
    va_ptr: u64,
) -> Result<i32, String> {
    if capacity > 1_048_576 {
        return Err(format!("vsnprintf output bound too large: {capacity}"));
    }
    let fmt = cstring(s, fmt_ptr, 4096)?;
    let layout = read_mem(&*s.api, s.uc, va_ptr, 32)?;
    let u64at = |off: usize| u64::from_le_bytes(layout[off..off + 8].try_into().unwrap());
    let mut stack = u64at(0);
    let gr_top = u64at(8);
    let mut gr_off = i32::from_le_bytes(layout[24..28].try_into().unwrap());
    let next_arg = |stack: &mut u64, gr_off: &mut i32| -> Result<u64, String> {
        let a = if *gr_off < 0 {
            let p = (gr_top as i64 + *gr_off as i64) as u64;
            *gr_off += 8;
            p
        } else {
            let p = *stack;
            *stack = stack.checked_add(8).ok_or("va_list stack overflow")?;
            p
        };
        let b = read_mem(&*s.api, s.uc, a, 8)?;
        Ok(u64::from_le_bytes(b.try_into().unwrap()))
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
                i += 1
            }
            b's' => {
                let p = next_arg(&mut stack, &mut gr_off)?;
                if p == 0 {
                    out.extend_from_slice(b"(null)")
                } else {
                    out.extend_from_slice(&cstring(s, p, 1_048_576)?);
                }
                i += 1
            }
            b'i' | b'd' => {
                let v = next_arg(&mut stack, &mut gr_off)? as u32 as i32;
                out.extend_from_slice(v.to_string().as_bytes());
                i += 1
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
        let mut b = out[..n].to_vec();
        b.push(0);
        write_mem(&*s.api, s.uc, dst, &b)?;
    }
    Ok(out.len().try_into().unwrap_or(i32::MAX))
}
unsafe extern "C" fn code_hook(uc: *mut c_void, address: u64, _size: u32, user: *mut c_void) {
    let s = &mut *(user as *mut HookState);
    if address == s.return_address {
        s.returned = true;
        let _ = ((*s.api).emu_stop)(uc);
        return;
    }
    let Some(name) = s.imports.get(&address).cloned() else {
        return;
    };
    *s.calls.entry(name.clone()).or_default() += 1;
    let run = (|| -> Result<(), String> {
        let a0 = arg(s, 0)?;
        let a1 = arg(s, 1)?;
        let a2 = arg(s, 2)?;
        let result = match name.as_str() {
            "malloc" => s.heap.alloc(a0, 16, &*s.api, uc)?,
            "free" => {
                s.heap.free_ptr(a0)?;
                0
            }
            "memcpy" | "memmove" => {
                let n = usize::try_from(a2).map_err(|_| "copy length too large")?;
                if n > 256 * 1024 * 1024 {
                    return Err(format!("{name} request too large: {n}"));
                }
                if n != 0 {
                    let b = read_mem(&*s.api, uc, a1, n)?;
                    write_mem(&*s.api, uc, a0, &b)?;
                }
                a0
            }
            "memset" => {
                let n = usize::try_from(a2).map_err(|_| "set length too large")?;
                if n > 256 * 1024 * 1024 {
                    return Err(format!("memset request too large: {n}"));
                }
                let b = vec![a1 as u8; n];
                write_mem(&*s.api, uc, a0, &b)?;
                a0
            }
            "memcmp" => {
                let n = usize::try_from(a2).map_err(|_| "compare length too large")?;
                if n > 64 * 1024 * 1024 {
                    return Err(format!("memcmp request too large: {n}"));
                }
                let l = if n == 0 {
                    Vec::new()
                } else {
                    read_mem(&*s.api, uc, a0, n)?
                };
                let r = if n == 0 {
                    Vec::new()
                } else {
                    read_mem(&*s.api, uc, a1, n)?
                };
                match l.iter().zip(r.iter()).find(|(x, y)| x != y) {
                    Some((x, y)) => (*x as i32 - *y as i32) as i64 as u64,
                    None => 0,
                }
            }
            "strlen" | "strnlen" => {
                let lim = if name == "strlen" {
                    16 * 1024 * 1024
                } else {
                    usize::try_from(a1.min(16 * 1024 * 1024)).map_err(|_| "length overflow")?
                };
                let bytes = cstring_bounded(s, a0, lim)?;
                bytes.len() as u64
            }
            "strcmp" => {
                let l = cstring(s, a0, 16 * 1024 * 1024)?;
                let r = cstring(s, a1, 16 * 1024 * 1024)?;
                let n = l.len().min(r.len());
                if let Some(i) = (0..n).find(|i| l[*i] != r[*i]) {
                    (l[i] as i32 - r[i] as i32) as i64 as u64
                } else {
                    (l.len() as i64 - r.len() as i64) as u64
                }
            }
            "clock_gettime" => {
                if !matches!(a0, 0 | 1 | 4 | 7) {
                    return Err(format!("unmodeled clock ID {a0}"));
                }
                s.clock_calls += 1;
                let nanos = (s.baseline_clock + s.clock_calls) * 1_000_000;
                let mut sec = nanos / 1_000_000_000;
                if a0 == 0 {
                    sec += 1_700_000_000;
                }
                let ns = nanos % 1_000_000_000;
                write_mem(
                    &*s.api,
                    uc,
                    a1,
                    &[sec.to_le_bytes(), ns.to_le_bytes()].concat(),
                )?;
                0
            }
            "vsnprintf" => format_arm64(s, a0, a1, a2, arg(s, 3)?)? as i64 as u64,
            _ => return Err(format!("unsupported native import {name} at {address:#x}")),
        };
        ret(s, result)
    })();
    if let Err(e) = run {
        s.error = Some(format!("import {name} at {address:#x}: {e}"));
        let _ = ((*s.api).emu_stop)(uc);
    }
}

struct BlockProfile {
    hook: usize,
    counts: Box<BTreeMap<u64, (u64, u32)>>,
}

unsafe extern "C" fn block_profile_hook(
    _: *mut c_void,
    address: u64,
    size: u32,
    opaque: *mut c_void,
) {
    // Unicorn calls this across all executable blocks. Count only source ELF
    // executable PT_LOAD ranges so every output address has a valid source RVA.
    let rva = address.wrapping_sub(0x1000_0000);
    if !(rva < 0x0187_ed20 || (0x01ab_8000..0x01ae_3cb4).contains(&rva)) {
        return;
    }
    let counts = &mut *opaque.cast::<BTreeMap<u64, (u64, u32)>>();
    let entry = counts.entry(address).or_insert((0, size));
    entry.0 += 1;
}

pub struct OracleReport {
    pub json: Value,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayCardRequest {
    pub side: u32,
    pub slot: u32,
    pub x: i32,
    pub y: i32,
}

fn invoke_native(
    api: &UnicornApi,
    uc: *mut c_void,
    ids: &BTreeMap<String, i32>,
    state: &mut HookState,
    abi: &Value,
    rva: u64,
    args: &[u64],
    x8: u64,
) -> Result<u64, String> {
    invoke_native_at(
        api,
        uc,
        ids,
        state,
        abi,
        0x1000_0000 + rva,
        &format!("{rva:#x}"),
        args,
        x8,
    )
}

fn invoke_native_at(
    api: &UnicornApi,
    uc: *mut c_void,
    ids: &BTreeMap<String, i32>,
    state: &mut HookState,
    abi: &Value,
    entry: u64,
    label: &str,
    args: &[u64],
    x8: u64,
) -> Result<u64, String> {
    if args.len() > 8 {
        return Err("native call has more than eight register arguments".into());
    }
    for i in 0..8 {
        write_u64(
            api,
            uc,
            ids[&format!("x{i}")],
            args.get(i).copied().unwrap_or(0),
        )?;
    }
    write_u64(api, uc, ids["x8"], x8)?;
    write_u64(api, uc, ids["sp"], num(&abi["call_stack_pointer"])?)?;
    let retaddr = num(&abi["return_address"])?;
    write_u64(api, uc, ids["x30"], retaddr)?;
    write_u64(api, uc, ids["pc"], entry)?;
    state.returned = false;
    state.error = None;
    let rc = unsafe { (api.emu_start)(uc, entry, 0, 40_000_000, 0) };
    if let Some(e) = state.error.take() {
        return Err(e);
    }
    if rc != 0 {
        return Err(format!("native function {label}: {}", api.err(rc)));
    }
    if !state.returned && read_u64(api, uc, ids["pc"])? != retaddr {
        return Err(format!(
            "native function {label} did not return; PC={:#x}",
            read_u64(api, uc, ids["pc"])?
        ));
    }
    read_u64(api, uc, ids["x0"])
}

pub fn run_checkpoint(
    checkpoint_dir: &Path,
    world_steps: u64,
    harness_steps: u64,
    until_terminal: bool,
    play_card: Option<PlayCardRequest>,
    after_play_steps: u64,
    audit_out: Option<&Path>,
) -> Result<OracleReport, String> {
    if after_play_steps > 0 && play_card.is_none() {
        return Err("--after-play-steps requires --play-card".into());
    }
    if let Some(dir) = audit_out {
        if dir.exists() {
            return Err(format!("audit output already exists: {}", dir.display()));
        }
    }
    let checkpoint_path = checkpoint_dir.join("checkpoint.json");
    let raw =
        fs::read(&checkpoint_path).map_err(|e| format!("{}: {e}", checkpoint_path.display()))?;
    let cp: Value = serde_json::from_slice(&raw).map_err(|e| e.to_string())?;
    let checkpoint_hash = sha256(&checkpoint_path)?;
    let abi = &cp["emulation_abi"];
    let lib = recorded_host_path(
        abi["unicorn_library"]
            .as_str()
            .ok_or("missing Unicorn path")?,
    );
    let libhash = sha256(&lib)?;
    if libhash
        != abi["unicorn_library_sha256"]
            .as_str()
            .ok_or("missing Unicorn hash")?
    {
        return Err("Unicorn library SHA-256 mismatch".into());
    }
    let source = engine_root().join("analysis/libg-cipher/libg.decrypted.so");
    let sourcehash = sha256(&source)?;
    if sourcehash != SOURCE_SHA256 || cp["source_sha256"] != SOURCE_SHA256 {
        return Err("original engine SHA-256 mismatch".into());
    }
    let api = unsafe { UnicornApi::load(&lib)? };
    let mut uc: *mut c_void = std::ptr::null_mut();
    let rc = unsafe { (api.open)(UC_ARCH_ARM64, UC_MODE_ARM, &mut uc) };
    if rc != 0 {
        return Err(format!("uc_open: {}", api.err(rc)));
    }
    let result = (|| -> Result<OracleReport, String> {
        let heap = GuestHeap::from_json(&cp["native_heap"])?;
        let mut mapped_bases = BTreeSet::new();
        for region in cp["regions"]
            .as_array()
            .ok_or("checkpoint regions missing")?
        {
            let file = region["file"].as_str().ok_or("region file missing")?;
            let path = checkpoint_dir.join(file);
            let data = fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            if sha256(&path)? != region["sha256"].as_str().ok_or("region SHA missing")? {
                return Err(format!("region hash mismatch: {file}"));
            }
            let base = num(&region["base"])?;
            let size = num(&region["size"])?;
            if data.len() as u64 != size {
                return Err(format!("region byte count mismatch: {file}"));
            }
            let map_size = if file == "heap.bin" {
                num(&cp["native_heap"]["capacity"])? as usize
            } else if let Some(extra) = heap.extras.iter().find(|a| a.base == base) {
                extra.capacity as usize
            } else {
                usize::try_from(size).map_err(|_| "region too large")?
            };
            let rc = unsafe { (api.mem_map)(uc, base, map_size, UC_PROT_ALL) };
            if rc != 0 {
                return Err(format!("uc_mem_map {file} at {base:#x}: {}", api.err(rc)));
            }
            write_mem(&api, uc, base, &data)?;
            mapped_bases.insert(base);
        }
        for a in &heap.extras {
            if !mapped_bases.contains(&a.base) {
                let rc = unsafe { (api.mem_map)(uc, a.base, a.capacity as usize, UC_PROT_ALL) };
                if rc != 0 {
                    return Err(format!("map checkpoint extra heap: {}", api.err(rc)));
                }
            }
        }
        let external = num(&abi["external_base"])?;
        let extsize = num(&abi["external_size"])?;
        let rc = unsafe { (api.mem_map)(uc, external, extsize as usize, UC_PROT_ALL) };
        if rc != 0 {
            return Err(format!("map externals: {}", api.err(rc)));
        }
        let retaddr = num(&abi["return_address"])?;
        let retbase = retaddr & !0xfff;
        let rc = unsafe { (api.mem_map)(uc, retbase, 0x1000, UC_PROT_ALL) };
        if rc != 0 {
            return Err(format!("map return stub: {}", api.err(rc)));
        }
        let ids = abi["register_ids"]
            .as_object()
            .ok_or("register IDs missing")?
            .iter()
            .map(|(k, v)| {
                Ok((
                    k.clone(),
                    v.as_i64().ok_or("register id must be number")? as i32,
                ))
            })
            .collect::<Result<BTreeMap<_, _>, String>>()?;
        for (name, hex) in cp["registers"].as_object().ok_or("registers missing")? {
            let id = *ids
                .get(name)
                .ok_or_else(|| format!("missing register ID for {name}"))?;
            let v = value_u128(hex)?;
            if name.starts_with('q') {
                let b = v.to_le_bytes();
                let rc = unsafe { (api.reg_write)(uc, id, b.as_ptr().cast()) };
                if rc != 0 {
                    return Err(format!("restore {name}: {}", api.err(rc)));
                }
            } else {
                let n = v as u64;
                let rc = unsafe { (api.reg_write)(uc, id, (&n as *const u64).cast()) };
                if rc != 0 {
                    return Err(format!("restore {name}: {}", api.err(rc)));
                }
            }
        }
        let values = &cp["call_plan"]["values"];
        let world = num(&values["battle_world"])?;
        let entities = num(&values["battle_entities"])?;
        let harness = num(&values["battle_harness"])?;
        let world_rva = 0x10ca81c_u64;
        let base = 0x1000_0000_u64;
        let mut before = Vec::new();
        let mut after = Vec::new();
        let mut entity_before = Vec::new();
        let mut entity_after = Vec::new();
        let mut execs = Vec::new();
        let mut harness_elapsed = Vec::new();
        let mut harness_events = Vec::new();
        let mut harness_completed = 0u64;
        let symbols = abi["import_symbols"]
            .as_object()
            .ok_or("import symbols missing")?
            .iter()
            .filter_map(|(k, v)| {
                let name = v.as_str()?.to_string();
                let addr = num(&Value::String(k.clone())).ok()?;
                Some((addr, name))
            })
            .collect::<BTreeMap<_, _>>();
        let import_calls = cp["import_counts"]
            .as_object()
            .ok_or("import counts missing")?
            .get("clock_gettime")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        let mut state = Box::new(HookState {
            api: &api,
            uc,
            imports: symbols,
            ids: ids.clone(),
            heap,
            baseline_clock: import_calls,
            clock_calls: 0,
            calls: BTreeMap::new(),
            error: None,
            returned: false,
            return_address: retaddr,
        });
        let mut hook = 0usize;
        let rc = unsafe {
            (api.hook_add)(
                uc,
                &mut hook,
                UC_HOOK_CODE,
                code_hook as *const c_void,
                (&mut *state as *mut HookState).cast(),
                external,
                external + extsize - 1,
            )
        };
        if rc != 0 {
            return Err(format!("uc_hook_add imports: {}", api.err(rc)));
        }
        let mut ret_hook = 0usize;
        let rc = unsafe {
            (api.hook_add)(
                uc,
                &mut ret_hook,
                UC_HOOK_CODE,
                code_hook as *const c_void,
                (&mut *state as *mut HookState).cast(),
                retaddr,
                retaddr,
            )
        };
        if rc != 0 {
            return Err(format!("uc_hook_add return: {}", api.err(rc)));
        }
        let function = base + world_rva;
        let counter1 = world
            .checked_add(0x60)
            .ok_or("world counter address overflow")?;
        let counter2 = world + 0x70;
        for _ in 0..world_steps {
            before.push([
                u32::from_le_bytes(read_mem(&api, uc, counter1, 4)?.try_into().unwrap()),
                u32::from_le_bytes(read_mem(&api, uc, counter2, 4)?.try_into().unwrap()),
            ]);
            entity_before.push(u32::from_le_bytes(
                read_mem(&api, uc, entities + 0x14, 4)?.try_into().unwrap(),
            ));
            for i in 0..8 {
                write_u64(
                    &api,
                    uc,
                    ids[&format!("x{i}")],
                    if i == 0 { world } else { 0 },
                )?;
            }
            write_u64(&api, uc, ids["x8"], 0)?;
            write_u64(&api, uc, ids["sp"], num(&abi["call_stack_pointer"])?)?;
            write_u64(&api, uc, ids["x30"], retaddr)?;
            write_u64(&api, uc, ids["pc"], function)?;
            state.returned = false;
            state.error = None;
            let start = Instant::now();
            let rc = unsafe { (api.emu_start)(uc, function, 0, 0, 0) };
            let elapsed = start.elapsed().as_secs_f64();
            if let Some(e) = state.error.take() {
                return Err(e);
            }
            if rc != 0 {
                return Err(format!("world native execution: {}", api.err(rc)));
            }
            if !state.returned {
                return Err(format!(
                    "world step did not reach return stub; PC={:#x}",
                    read_u64(&api, uc, ids["pc"])?
                ));
            }
            let x0 = read_u64(&api, uc, ids["x0"])?;
            after.push([
                u32::from_le_bytes(read_mem(&api, uc, counter1, 4)?.try_into().unwrap()),
                u32::from_le_bytes(read_mem(&api, uc, counter2, 4)?.try_into().unwrap()),
            ]);
            entity_after.push(u32::from_le_bytes(
                read_mem(&api, uc, entities + 0x14, 4)?.try_into().unwrap(),
            ));
            execs.push(elapsed);
            if let Some(e) = state.error.take() {
                return Err(e);
            }
            let _ = x0;
        }
        let mut play_report = Value::Null;
        let mut play_world_before = Vec::new();
        let mut play_world_after = Vec::new();
        if let Some(request) = play_card {
            if request.side > 1 {
                return Err("--play-card SIDE must be 0 or 1".into());
            }
            if request.slot > 3 {
                return Err("--play-card SLOT must be in 0..=3".into());
            }
            let battle = num(&values["battle_state"])?;
            let command_manager = num(&values["battle_commands"])?;
            let side_obj = invoke_native(
                &api,
                uc,
                &ids,
                &mut state,
                abi,
                0xd35d5c,
                &[battle, request.side as u64],
                0,
            )?;
            if side_obj == 0 {
                return Err(format!("native battle side {} is null", request.side));
            }
            let actor = invoke_native(
                &api,
                uc,
                &ids,
                &mut state,
                abi,
                0xd3c980,
                &[battle, request.side as u64],
                0,
            )?;
            if actor == 0 {
                return Err(format!("native actor for side {} is null", request.side));
            }
            let wrapper = invoke_native(
                &api,
                uc,
                &ids,
                &mut state,
                abi,
                0xf9dd98,
                &[side_obj, request.slot as u64],
                0,
            )?;
            if wrapper == 0 {
                return Err(format!(
                    "native hand slot {} has no card wrapper",
                    request.slot
                ));
            }
            let descriptor = state.heap.alloc(24, 16, &api, uc)?;
            write_mem(&api, uc, descriptor, &[0; 24])?;
            invoke_native(
                &api,
                uc,
                &ids,
                &mut state,
                abi,
                0xe80bbc,
                &[wrapper, side_obj, 1],
                descriptor,
            )?;
            let proxy = invoke_native(&api, uc, &ids, &mut state, abi, 0xe80680, &[descriptor], 0)?;
            if proxy == 0 {
                return Err("native card descriptor produced a null selection proxy".into());
            }
            let validity = invoke_native(
                &api,
                uc,
                &ids,
                &mut state,
                abi,
                0xd42214,
                &[
                    battle,
                    request.x as i64 as u64,
                    request.y as i64 as u64,
                    proxy,
                    0,
                ],
                0,
            )? as u32;
            let mana =
                invoke_native(&api, uc, &ids, &mut state, abi, 0xfa1c84, &[side_obj, 1], 0)? as u32;
            let cost =
                invoke_native(&api, uc, &ids, &mut state, abi, 0xe80638, &[descriptor], 0)? as u32;
            let queue_before = u32::from_le_bytes(
                read_mem(&api, uc, command_manager + 0x14, 4)?
                    .try_into()
                    .unwrap(),
            );
            let mut command_ptr = 0u64;
            let mut submitted_slot_ptr = 0u64;
            let mut submit_return = Value::Null;
            let mut slot_after = 0u64;
            let mut queue_after = queue_before;
            let mut entity_count_after_submit =
                u32::from_le_bytes(read_mem(&api, uc, entities + 0x14, 4)?.try_into().unwrap());
            let mut command_live_after_submit = Value::Null;
            let mut preflight_rejection = if validity != 0 {
                Some(format!("invalid_target_code_{validity}"))
            } else if mana < cost {
                Some("insufficient_mana".to_string())
            } else {
                None
            };
            let preflight_accepted = preflight_rejection.is_none();
            if preflight_accepted {
                // The native constructor initializes the allocation; do not zero it.
                command_ptr = state.heap.alloc(0x58, 16, &api, uc)?;
                let context =
                    u64::from_le_bytes(read_mem(&api, uc, harness, 8)?.try_into().unwrap());
                invoke_native(
                    &api,
                    uc,
                    &ids,
                    &mut state,
                    abi,
                    0xd774ac,
                    &[command_ptr, context],
                    0,
                )?;
                invoke_native(
                    &api,
                    uc,
                    &ids,
                    &mut state,
                    abi,
                    0xd77c8c,
                    &[command_ptr, descriptor],
                    0,
                )?;
                invoke_native(
                    &api,
                    uc,
                    &ids,
                    &mut state,
                    abi,
                    0xd77a14,
                    &[
                        command_ptr,
                        request.x as i64 as u64,
                        request.y as i64 as u64,
                    ],
                    0,
                )?;
                let slot_ptr = state.heap.alloc(8, 8, &api, uc)?;
                submitted_slot_ptr = slot_ptr;
                write_mem(&api, uc, slot_ptr, &command_ptr.to_le_bytes())?;
                let submit = invoke_native(
                    &api,
                    uc,
                    &ids,
                    &mut state,
                    abi,
                    0x11d680c,
                    &[harness, slot_ptr, actor],
                    0,
                )?;
                submit_return = json!(format!("{submit:#x}"));
                slot_after =
                    u64::from_le_bytes(read_mem(&api, uc, slot_ptr, 8)?.try_into().unwrap());
                queue_after = u32::from_le_bytes(
                    read_mem(&api, uc, command_manager + 0x14, 4)?
                        .try_into()
                        .unwrap(),
                );
                entity_count_after_submit =
                    u32::from_le_bytes(read_mem(&api, uc, entities + 0x14, 4)?.try_into().unwrap());
                command_live_after_submit =
                    json!(state.heap.allocations.contains_key(&command_ptr));
            }
            let mut accepted = false;
            if preflight_accepted {
                let live = state.heap.allocations.contains_key(&command_ptr);
                accepted = queue_after == queue_before.wrapping_add(1)
                    && slot_after == command_ptr
                    && live;
                if !accepted {
                    preflight_rejection = Some("original_submission_rejected".into());
                }
            }
            play_report = json!({
                "side":request.side,"hand_slot":request.slot,"x":request.x,"y":request.y,
                "side_object":format!("{side_obj:#x}"),"actor":format!("{actor:#x}"),
                "wrapper":format!("{wrapper:#x}"),"descriptor":format!("{descriptor:#x}"),
                "selection_proxy":format!("{proxy:#x}"),"target_validity_code":validity,
                "mana":mana,"cost":cost,"preflight_accepted":preflight_accepted,
                "accepted":accepted,"rejection":preflight_rejection,"command":if command_ptr==0{Value::Null}else{json!(format!("{command_ptr:#x}"))},
                "native_submit_x0_unspecified":submit_return,"original_slot_after_submit":format!("{slot_after:#x}"),
                "queue_count_before":queue_before,"queue_count_after_submit":queue_after,
                "submission_attempted":preflight_accepted,
                "native_entity_count_after_submit":if preflight_accepted{json!(entity_count_after_submit)}else{Value::Null},
                "command_allocation_live_after_submit":command_live_after_submit
            });
            if after_play_steps > 0 && accepted {
                for _ in 0..after_play_steps {
                    play_world_before.push([
                        u32::from_le_bytes(read_mem(&api, uc, counter1, 4)?.try_into().unwrap()),
                        u32::from_le_bytes(read_mem(&api, uc, counter2, 4)?.try_into().unwrap()),
                    ]);
                    invoke_native(&api, uc, &ids, &mut state, abi, world_rva, &[world], 0)?;
                    play_world_after.push([
                        u32::from_le_bytes(read_mem(&api, uc, counter1, 4)?.try_into().unwrap()),
                        u32::from_le_bytes(read_mem(&api, uc, counter2, 4)?.try_into().unwrap()),
                    ]);
                }
                let queue_now = u32::from_le_bytes(
                    read_mem(&api, uc, command_manager + 0x14, 4)?
                        .try_into()
                        .unwrap(),
                );
                play_report["queue_count_after_world_steps"] = json!(queue_now);
                let slot_now = u64::from_le_bytes(
                    read_mem(&api, uc, submitted_slot_ptr, 8)?
                        .try_into()
                        .unwrap(),
                );
                play_report["original_slot_after_world_steps"] = json!(format!("{slot_now:#x}"));
                let mana_after =
                    invoke_native(&api, uc, &ids, &mut state, abi, 0xfa1c84, &[side_obj, 1], 0)?
                        as u32;
                let entity_count_after =
                    u32::from_le_bytes(read_mem(&api, uc, entities + 0x14, 4)?.try_into().unwrap());
                let command_live_after = json!(state.heap.allocations.contains_key(&command_ptr));
                play_report["mana_after_world_steps"] = json!(mana_after);
                play_report["native_entity_count_after_world_steps"] = json!(entity_count_after);
                play_report["command_allocation_live_after_world_steps"] = command_live_after;
            }
            play_report["after_play_steps_completed"] = json!(play_world_after.len());
        } else if after_play_steps > 0 {
            return Err("--after-play-steps requires --play-card".into());
        }
        let scratch = if harness_steps > 0 || until_terminal {
            let p = state.heap.alloc(8, 16, &api, uc)?;
            write_mem(&api, uc, p, &[0; 8])?;
            Some(p)
        } else {
            None
        };
        let mut terminal_elapsed = None;
        let mut terminal_event = Value::Null;
        let terminal_counters_before = [
            u32::from_le_bytes(read_mem(&api, uc, counter1, 4)?.try_into().unwrap()),
            u32::from_le_bytes(read_mem(&api, uc, counter2, 4)?.try_into().unwrap()),
        ];
        if until_terminal {
            let sret = scratch.ok_or("terminal result slot was not allocated")?;
            for i in 0..8 {
                write_u64(
                    &api,
                    uc,
                    ids[&format!("x{i}")],
                    if i == 0 { harness } else { 0 },
                )?;
            }
            write_u64(&api, uc, ids["x8"], sret)?;
            write_u64(&api, uc, ids["sp"], num(&abi["call_stack_pointer"])?)?;
            write_u64(&api, uc, ids["x30"], retaddr)?;
            let entry = base + 0x11d6364;
            write_u64(&api, uc, ids["pc"], entry)?;
            state.returned = false;
            state.error = None;
            let start = Instant::now();
            // Match the prepared Python runner's wall-clock bound; an instruction
            // count makes Unicorn's hot hooks disproportionately slow.
            let rc = unsafe { (api.emu_start)(uc, entry, 0, 40_000_000, 0) };
            terminal_elapsed = Some(start.elapsed().as_secs_f64());
            if let Some(e) = state.error.take() {
                return Err(e);
            }
            if rc != 0 {
                return Err(format!("original until-terminal harness: {}", api.err(rc)));
            }
            if !state.returned {
                return Err(format!(
                    "original until-terminal call did not return within 40 seconds; PC={:#x}",
                    read_u64(&api, uc, ids["pc"])?
                ));
            }
            let event = u64::from_le_bytes(read_mem(&api, uc, sret, 8)?.try_into().unwrap());
            if event == 0 {
                return Err("original until-terminal harness returned a null event".into());
            }
            let code = u32::from_le_bytes(read_mem(&api, uc, event, 4)?.try_into().unwrap());
            terminal_event = json!({"pointer":format!("{event:#x}"),"event_code":code});
        }
        let terminal_counters_after = [
            u32::from_le_bytes(read_mem(&api, uc, counter1, 4)?.try_into().unwrap()),
            u32::from_le_bytes(read_mem(&api, uc, counter2, 4)?.try_into().unwrap()),
        ];
        let terminal_steps = if until_terminal {
            terminal_counters_after[0].wrapping_sub(terminal_counters_before[0])
        } else {
            0
        };
        let mut harness_counters_before = Vec::new();
        let mut harness_counters_after = Vec::new();
        let mut harness_entities_before = Vec::new();
        let mut harness_entities_after = Vec::new();
        for _ in 0..harness_steps {
            let scratch = scratch.ok_or("harness result slot was not allocated")?;
            harness_counters_before.push([
                u32::from_le_bytes(read_mem(&api, uc, counter1, 4)?.try_into().unwrap()),
                u32::from_le_bytes(read_mem(&api, uc, counter2, 4)?.try_into().unwrap()),
            ]);
            harness_entities_before.push(u32::from_le_bytes(
                read_mem(&api, uc, entities + 0x14, 4)?.try_into().unwrap(),
            ));
            write_mem(&api, uc, scratch, &[0; 8])?;
            for i in 0..8 {
                write_u64(
                    &api,
                    uc,
                    ids[&format!("x{i}")],
                    if i == 0 { harness } else { 0 },
                )?;
            }
            write_u64(&api, uc, ids["x8"], scratch)?;
            write_u64(&api, uc, ids["sp"], num(&abi["call_stack_pointer"])?)?;
            write_u64(&api, uc, ids["x30"], retaddr)?;
            write_u64(&api, uc, ids["pc"], base + 0x11d6398)?;
            state.returned = false;
            state.error = None;
            let start = Instant::now();
            let rc = unsafe { (api.emu_start)(uc, base + 0x11d6398, 0, 0, 0) };
            let elapsed = start.elapsed().as_secs_f64();
            if let Some(e) = state.error.take() {
                return Err(e);
            }
            if rc != 0 {
                return Err(format!("harness native execution: {}", api.err(rc)));
            }
            if !state.returned {
                return Err(format!(
                    "harness step did not return; PC={:#x}",
                    read_u64(&api, uc, ids["pc"])?
                ));
            }
            let event = u64::from_le_bytes(read_mem(&api, uc, scratch, 8)?.try_into().unwrap());
            let code = if event == 0 {
                None
            } else {
                Some(u32::from_le_bytes(
                    read_mem(&api, uc, event, 4)?.try_into().unwrap(),
                ))
            };
            harness_events.push(json!({"event":if event==0{Value::Null}else{json!(format!("{event:#x}"))},"event_code":code}));
            harness_counters_after.push([
                u32::from_le_bytes(read_mem(&api, uc, counter1, 4)?.try_into().unwrap()),
                u32::from_le_bytes(read_mem(&api, uc, counter2, 4)?.try_into().unwrap()),
            ]);
            harness_entities_after.push(u32::from_le_bytes(
                read_mem(&api, uc, entities + 0x14, 4)?.try_into().unwrap(),
            ));
            harness_elapsed.push(elapsed);
            harness_completed += 1;
            if event != 0 {
                break;
            }
        }
        let deltas = before
            .iter()
            .zip(after.iter())
            .map(|(a, b)| [b[0].wrapping_sub(a[0]), b[1].wrapping_sub(a[1])])
            .collect::<Vec<_>>();
        let audit_regions = if let Some(dir) = audit_out {
            fs::create_dir(dir).map_err(|e| e.to_string())?;
            let region_dir = dir.join("regions");
            fs::create_dir_all(&region_dir).map_err(|e| e.to_string())?;
            let mut rows = Vec::new();
            let mut captured_heaps = BTreeSet::new();
            for region in cp["regions"]
                .as_array()
                .ok_or("checkpoint regions missing")?
            {
                let file = region["file"].as_str().ok_or("region filename missing")?;
                let base = num(&region["base"])?;
                let size_u64 = if file == "heap.bin" {
                    align(state.heap.pointer - state.heap.base, 0x1000)?
                } else if let Some((index, arena)) = state
                    .heap
                    .extras
                    .iter()
                    .enumerate()
                    .find(|(_, a)| a.base == base)
                {
                    captured_heaps.insert(index);
                    align(arena.pointer - arena.base, 0x1000)?
                } else {
                    num(&region["size"])?
                };
                let size = usize::try_from(size_u64).map_err(|_| "audit region too large")?;
                let bytes = read_mem(&api, uc, base, size)?;
                let path = region_dir.join(file);
                fs::write(&path, &bytes).map_err(|e| format!("write {}: {e}", path.display()))?;
                rows.push(json!({"file":format!("regions/{file}"),"base":region["base"],"size":size,"sha256":sha256(&path)?}));
                if file == "heap.bin" {
                    captured_heaps.insert(usize::MAX);
                }
            }
            if !captured_heaps.contains(&usize::MAX) {
                let size = usize::try_from(align(state.heap.pointer - state.heap.base, 0x1000)?)
                    .map_err(|_| "audit heap too large")?;
                let file = "heap.bin";
                let path = region_dir.join(file);
                let bytes = read_mem(&api, uc, state.heap.base, size)?;
                fs::write(&path, &bytes).map_err(|e| format!("write {}: {e}", path.display()))?;
                rows.push(json!({"file":format!("regions/{file}"),"base":format!("{:#x}",state.heap.base),"size":size,"sha256":sha256(&path)?}));
            }
            for (index, arena) in state.heap.extras.iter().enumerate() {
                if captured_heaps.contains(&index) {
                    continue;
                }
                let size = usize::try_from(align(arena.pointer - arena.base, 0x1000)?)
                    .map_err(|_| "audit extra heap too large")?;
                let file = format!("heap_extra_{index:02}.bin");
                let path = region_dir.join(&file);
                let bytes = read_mem(&api, uc, arena.base, size)?;
                fs::write(&path, &bytes).map_err(|e| format!("write {}: {e}", path.display()))?;
                rows.push(json!({"file":format!("regions/{file}"),"base":format!("{:#x}",arena.base),"size":size,"sha256":sha256(&path)?}));
            }
            let mut registers = serde_json::Map::new();
            for (name, id) in &ids {
                let value = if name.starts_with('q') {
                    let mut bytes = [0u8; 16];
                    let rc = unsafe { (api.reg_read)(uc, *id, bytes.as_mut_ptr().cast()) };
                    if rc != 0 {
                        return Err(format!("audit register {name}: {}", api.err(rc)));
                    }
                    u128::from_le_bytes(bytes)
                } else {
                    read_u64(&api, uc, *id)? as u128
                };
                registers.insert(name.clone(), json!(format!("0x{value:x}")));
            }
            let allocations = state
                .heap
                .allocations
                .iter()
                .map(|(p, n)| (p.to_string(), json!(n)))
                .collect::<serde_json::Map<_, _>>();
            let free_buckets = state
                .heap
                .free
                .iter()
                .map(|(size, items)| (size.to_string(), json!(items)))
                .collect::<serde_json::Map<_, _>>();
            let allocator = json!({"version":1,"base":state.heap.base,"capacity":state.heap.capacity,"primary_pointer":state.heap.pointer,"allocations":allocations,"free_buckets":free_buckets,"extra_heaps":state.heap.extras.iter().map(|a|json!({"base":a.base,"capacity":a.capacity,"pointer":a.pointer})).collect::<Vec<_>>()});
            let mut meta = json!({"source_sha256":sourcehash,"checkpoint_sha256":checkpoint_hash,"checkpoint":checkpoint_dir.display().to_string(),"world_steps":world_steps,"world_counters_before":before,"world_counters_after":after,"until_terminal":until_terminal,"terminal_counters_before":terminal_counters_before,"terminal_counters_after":terminal_counters_after,"terminal_steps":terminal_steps,"terminal_event":terminal_event,"harness_steps_completed":harness_completed,"harness_counters_before":harness_counters_before,"harness_counters_after":harness_counters_after,"native_entity_counts_before":entity_before,"native_entity_counts_after":entity_after,"harness_entity_counts_before":harness_entities_before,"harness_entity_counts_after":harness_entities_after,"regions":rows,"registers":registers,"allocator_file":"allocator.json"});
            meta["play_card"] = play_report.clone();
            meta["play_world_steps_requested"] = json!(after_play_steps);
            meta["play_world_steps"] = json!(play_world_after.len());
            meta["play_world_counters_before"] = json!(play_world_before);
            meta["play_world_counters_after"] = json!(play_world_after);
            fs::write(
                dir.join("allocator.json"),
                serde_json::to_vec(&allocator).map_err(|e| e.to_string())?,
            )
            .map_err(|e| format!("write allocator.json: {e}"))?;
            fs::write(
                dir.join("audit.json"),
                serde_json::to_vec_pretty(&meta).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            Some(meta["regions"].clone())
        } else {
            None
        };
        let mut summary = json!({"source_sha256":sourcehash,"checkpoint_sha256":checkpoint_hash,"checkpoint":checkpoint_dir.display().to_string(),"function_rva":"0x10ca81c","battle_world":format!("{world:#x}"),"world_steps":world_steps,"world_counters_before":before,"world_counters_after":after,"world_counter_deltas":deltas,"native_entity_manager":format!("{entities:#x}"),"native_entity_count_offset":"0x14 (original f8cd90 reads manager+0x14)","native_entity_counts_before":entity_before,"native_entity_counts_after":entity_after,"step_seconds":execs,"terminal_function_rva":"0x11d6364","until_terminal":until_terminal,"terminal_counters_before":terminal_counters_before,"terminal_counters_after":terminal_counters_after,"terminal_steps":terminal_steps,"terminal_event":terminal_event,"terminal_seconds":terminal_elapsed,"harness_function_rva":"0x11d6398","harness_steps_requested":harness_steps,"harness_steps_completed":harness_completed,"harness_counters_before":harness_counters_before,"harness_counters_after":harness_counters_after,"harness_entity_counts_before":harness_entities_before,"harness_entity_counts_after":harness_entities_after,"harness_step_seconds":harness_elapsed,"harness_events":harness_events,"total_seconds":execs.iter().sum::<f64>()+harness_elapsed.iter().sum::<f64>()+terminal_elapsed.unwrap_or(0.0),"imports_called":state.calls,"clock_gettime_calls":state.clock_calls,"heap":{"primary_pointer":format!("{:#x}",state.heap.pointer),"used_bytes":state.heap.pointer-state.heap.base+state.heap.extras.iter().map(|x|x.pointer-x.base).sum::<u64>(),"live_allocations":state.heap.allocations.len(),"free_blocks":state.heap.free.values().map(Vec::len).sum::<usize>(),"extra_arenas":state.heap.extras.len()},"audit_regions":audit_regions});
        summary["play_card"] = play_report;
        summary["play_world_steps_requested"] = json!(after_play_steps);
        summary["play_world_steps"] = json!(play_world_after.len());
        summary["play_world_counters_before"] = json!(play_world_before);
        summary["play_world_counters_after"] = json!(play_world_after);
        Ok(OracleReport { json: summary })
    })();
    unsafe {
        (api.close)(uc);
    }
    result
}

/// Persistent native engine state for interactive environments.
///
/// The session owns its Unicorn API and boxed hook state because registered
/// callbacks retain raw pointers to both for the lifetime of the emulator.
pub struct NativeSession {
    api: Box<UnicornApi>,
    uc: *mut c_void,
    ids: BTreeMap<String, i32>,
    abi: Value,
    state: Box<HookState>,
    world: u64,
    battle: u64,
    harness: u64,
    entities: u64,
    commands: u64,
    context: u64,
    normal_test_row: u64,
    descriptor_scratch: u64,
    initial_checkpoint_sha256: String,
    block_profile: Option<BlockProfile>,
    presentation_start_tick: u32,
    presentation_names: BTreeMap<u64, String>,
    presentation_positions: BTreeMap<u64, (i32, i32)>,
    presentation_first_seen_ticks: BTreeMap<u64, u32>,
}

/// One-shot code hooks around the original per-side deck builder. The loaded
/// Normal row carries a single `AlwaysSpells` vector; `11d59dc` calls
/// `11d6f18` once for side 0 and once for side 1, so the second entry is the
/// point where an asymmetric side-1 deck can be supplied.
struct DeckSwapHook {
    api: *const UnicornApi,
    uc: *mut c_void,
    entry: u64,
    restore: u64,
    slot: u64,
    second_ptr: u64,
    second_count: u32,
    original_ptr: u64,
    original_count: u32,
    calls: u32,
    error: Option<String>,
}

unsafe extern "C" fn deck_swap_hook(
    _: *mut c_void,
    address: u64,
    _size: u32,
    opaque: *mut c_void,
) {
    let state = &mut *opaque.cast::<DeckSwapHook>();
    let (pointer, count) = if address == state.entry {
        state.calls += 1;
        if state.calls != 2 {
            return;
        }
        (state.second_ptr, state.second_count)
    } else if address == state.restore && state.calls == 2 {
        (state.original_ptr, state.original_count)
    } else {
        return;
    };
    let api = &*state.api;
    let slot = state.slot;
    let pointer_bytes = pointer.to_le_bytes();
    let count_bytes = count.to_le_bytes();
    let failed = (api.mem_write)(state.uc, slot, pointer_bytes.as_ptr().cast(), 8) != 0
        || (api.mem_write)(state.uc, slot + 8, count_bytes.as_ptr().cast(), 4) != 0
        || (api.mem_write)(state.uc, slot + 12, count_bytes.as_ptr().cast(), 4) != 0;
    if failed {
        state.error = Some(format!(
            "deck-swap hook could not rewrite the AlwaysSpells vector at {slot:#x}"
        ));
    }
}

/// Device, inode, size and mtime captured when a checkpoint file was hashed.
#[derive(Clone, Debug, PartialEq, Eq)]
struct FileIdentity {
    path: PathBuf,
    dev: u64,
    ino: u64,
    len: u64,
    mtime: i64,
    mtime_nsec: i64,
}

fn identity_of(path: &Path) -> Result<FileIdentity, String> {
    let meta = fs::metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(FileIdentity {
        path: path.to_path_buf(),
        dev: meta.dev(),
        ino: meta.ino(),
        len: meta.len(),
        mtime: meta.mtime(),
        mtime_nsec: meta.mtime_nsec(),
    })
}

fn identities_match(saved: &[FileIdentity]) -> Result<(), String> {
    for expected in saved {
        let now = identity_of(&expected.path)?;
        if now.dev != expected.dev
            || now.ino != expected.ino
            || now.len != expected.len
            || now.mtime != expected.mtime
            || now.mtime_nsec != expected.mtime_nsec
        {
            return Err(format!(
                "checkpoint file identity changed: {}",
                expected.path.display()
            ));
        }
    }
    Ok(())
}

struct RegionImage {
    file: String,
    base: u64,
    size: u64,
    bytes: Vec<u8>,
}

struct Populated {
    ids: BTreeMap<String, i32>,
    abi: Value,
    state: Box<HookState>,
    world: u64,
    battle: u64,
    harness: u64,
    entities: u64,
    commands: u64,
    context: u64,
    normal_test_row: u64,
    descriptor_scratch: u64,
}

/// Checkpoint JSON, region images, and allocator/register state kept after one
/// successful verification. Later `native-watch` sessions are built from this
/// value. It is not mutated: each session clones `heap` and copies region bytes
/// into a new emulator.
pub struct PristineTemplate {
    checkpoint_json: Vec<u8>,
    checkpoint_hash: String,
    regions: Vec<RegionImage>,
    heap: GuestHeap,
    abi: Value,
    registers: Value,
    call_values: Value,
    baseline_clock: u64,
    identities: Vec<FileIdentity>,
    /// One `dlopen` of the verified Unicorn library, shared by every session.
    unicorn: UnicornApi,
}

impl PristineTemplate {
    /// Read and hash the checkpoint. A mismatch returns an error and does not
    /// produce a template, which is what makes the first failure abort.
    pub fn load_verified(checkpoint_dir: &Path) -> Result<Self, String> {
        let checkpoint_path = checkpoint_dir.join("checkpoint.json");
        let checkpoint_json = fs::read(&checkpoint_path)
            .map_err(|e| format!("{}: {e}", checkpoint_path.display()))?;
        let cp: Value = serde_json::from_slice(&checkpoint_json).map_err(|e| e.to_string())?;
        let checkpoint_hash = sha256(&checkpoint_path)?;
        let mut identities = vec![identity_of(&checkpoint_path)?];
        let abi = cp["emulation_abi"].clone();
        let lib = recorded_host_path(
            abi["unicorn_library"]
                .as_str()
                .ok_or("missing Unicorn path")?,
        );
        if sha256(&lib)?
            != abi["unicorn_library_sha256"]
                .as_str()
                .ok_or("missing Unicorn hash")?
        {
            return Err("Unicorn library SHA-256 mismatch".into());
        }
        identities.push(identity_of(&lib)?);
        let source = engine_root().join("analysis/libg-cipher/libg.decrypted.so");
        if sha256(&source)? != SOURCE_SHA256 || cp["source_sha256"] != SOURCE_SHA256 {
            return Err("original engine SHA-256 mismatch".into());
        }
        identities.push(identity_of(&source)?);
        let unicorn = unsafe { UnicornApi::load(&lib)? };
        let heap = GuestHeap::from_json(&cp["native_heap"])?;
        let registers = cp["registers"].clone();
        let call_values = cp["call_plan"]["values"].clone();
        let baseline_clock = cp["import_counts"]
            .as_object()
            .and_then(|object| object.get("clock_gettime"))
            .and_then(Value::as_u64)
            .unwrap_or(0);
        let mut region_meta = Vec::new();
        for region in cp["regions"].as_array().ok_or("checkpoint regions missing")? {
            region_meta.push((
                region["file"].as_str().ok_or("region file missing")?.to_string(),
                num(&region["base"])?,
                num(&region["size"])?,
                region["sha256"]
                    .as_str()
                    .ok_or("region SHA missing")?
                    .to_string(),
            ));
        }
        drop(cp);
        let mut regions = Vec::with_capacity(region_meta.len());
        for (file, base, size, expected_hash) in region_meta {
            let path = checkpoint_dir.join(&file);
            let bytes = fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            if sha256(&path)? != expected_hash {
                return Err(format!("region hash mismatch: {file}"));
            }
            if bytes.len() as u64 != size {
                return Err(format!("region byte count mismatch: {file}"));
            }
            identities.push(identity_of(&path)?);
            regions.push(RegionImage {
                file,
                base,
                size,
                bytes,
            });
        }
        let checkpoint_id = identities
            .iter()
            .find(|id| id.path == checkpoint_path)
            .ok_or("checkpoint.json identity missing")?;
        if checkpoint_id.len != checkpoint_json.len() as u64 {
            return Err("checkpoint.json size changed while verifying".into());
        }
        Ok(Self {
            checkpoint_json,
            checkpoint_hash,
            regions,
            heap,
            abi,
            registers,
            call_values,
            baseline_clock,
            identities,
            unicorn,
        })
    }

    /// Stat each verified file. This does not read or hash the contents.
    pub fn identity_unchanged(&self) -> Result<(), String> {
        identities_match(&self.identities)?;
        let checkpoint_len = self
            .identities
            .iter()
            .find(|id| id.path.ends_with("checkpoint.json"))
            .map(|id| id.len)
            .ok_or("checkpoint.json identity missing")?;
        if checkpoint_len != self.checkpoint_json.len() as u64 {
            return Err(
                "in-memory checkpoint JSON does not match the verified file size".into(),
            );
        }
        Ok(())
    }

    fn instantiate(&self) -> Result<NativeSession, String> {
        open_session(
            &self.unicorn,
            self.heap.clone(),
            &self.regions,
            &self.abi,
            &self.registers,
            &self.call_values,
            self.baseline_clock,
            &self.checkpoint_hash,
        )
    }

    fn into_session(self) -> Result<NativeSession, String> {
        let Self {
            unicorn,
            heap,
            regions,
            abi,
            registers,
            call_values,
            baseline_clock,
            checkpoint_hash,
            checkpoint_json: _,
            identities: _,
        } = self;
        open_session(
            &unicorn,
            heap,
            &regions,
            &abi,
            &registers,
            &call_values,
            baseline_clock,
            &checkpoint_hash,
        )
    }
}

fn open_session(
    unicorn: &UnicornApi,
    heap: GuestHeap,
    regions: &[RegionImage],
    abi: &Value,
    registers: &Value,
    call_values: &Value,
    baseline_clock: u64,
    checkpoint_hash: &str,
) -> Result<NativeSession, String> {
    let api = Box::new(unicorn.clone());
    let mut uc = std::ptr::null_mut();
    let rc = unsafe { (api.open)(UC_ARCH_ARM64, UC_MODE_ARM, &mut uc) };
    if rc != 0 {
        return Err(format!("uc_open: {}", api.err(rc)));
    }
    match populate(
        &api,
        uc,
        heap,
        regions,
        abi,
        registers,
        call_values,
        baseline_clock,
    ) {
        Ok(parts) => {
            let presentation_start_tick = match read_mem_u32(&api, uc, parts.world + 0x60) {
                Ok(tick) => tick,
                Err(error) => {
                    unsafe { (api.close)(uc) };
                    return Err(error);
                }
            };
            Ok(NativeSession {
                api,
                uc,
                ids: parts.ids,
                abi: parts.abi,
                state: parts.state,
                world: parts.world,
                battle: parts.battle,
                harness: parts.harness,
                entities: parts.entities,
                commands: parts.commands,
                context: parts.context,
                normal_test_row: parts.normal_test_row,
                descriptor_scratch: parts.descriptor_scratch,
                initial_checkpoint_sha256: checkpoint_hash.to_string(),
                block_profile: None,
                presentation_start_tick,
                presentation_names: BTreeMap::new(),
                presentation_positions: BTreeMap::new(),
                presentation_first_seen_ticks: BTreeMap::new(),
            })
        }
        Err(error) => {
            unsafe { (api.close)(uc) };
            Err(error)
        }
    }
}

fn populate(
    api: &UnicornApi,
    uc: *mut c_void,
    heap: GuestHeap,
    regions: &[RegionImage],
    abi_src: &Value,
    registers: &Value,
    call_values: &Value,
    baseline_clock: u64,
) -> Result<Populated, String> {
    let mut mapped_bases = BTreeSet::new();
    for region in regions {
        let data = &region.bytes;
        if data.len() as u64 != region.size {
            return Err(format!("region byte count mismatch: {}", region.file));
        }
        let base = region.base;
        let size = region.size;
        let map_size = if region.file == "heap.bin" {
            heap.capacity as usize
        } else if let Some(extra) = heap.extras.iter().find(|arena| arena.base == base) {
            extra.capacity as usize
        } else {
            usize::try_from(size).map_err(|_| "region too large")?
        };
        let rc = unsafe { (api.mem_map)(uc, base, map_size, UC_PROT_ALL) };
        if rc != 0 {
            return Err(format!(
                "uc_mem_map {} at {base:#x}: {}",
                region.file,
                api.err(rc)
            ));
        }
        write_mem(api, uc, base, data)?;
        mapped_bases.insert(base);
    }
    for arena in &heap.extras {
        if !mapped_bases.contains(&arena.base) {
            let rc = unsafe { (api.mem_map)(uc, arena.base, arena.capacity as usize, UC_PROT_ALL) };
            if rc != 0 {
                return Err(format!("map checkpoint extra heap: {}", api.err(rc)));
            }
        }
    }
    let abi = abi_src.clone();
    let external = num(&abi["external_base"])?;
    let external_size = num(&abi["external_size"])?;
    let rc = unsafe { (api.mem_map)(uc, external, external_size as usize, UC_PROT_ALL) };
    if rc != 0 {
        return Err(format!("map externals: {}", api.err(rc)));
    }
    let retaddr = num(&abi["return_address"])?;
    let rc = unsafe { (api.mem_map)(uc, retaddr & !0xfff, 0x1000, UC_PROT_ALL) };
    if rc != 0 {
        return Err(format!("map return stub: {}", api.err(rc)));
    }
    let ids = abi["register_ids"]
        .as_object()
        .ok_or("register IDs missing")?
        .iter()
        .map(|(name, id)| {
            Ok((
                name.clone(),
                id.as_i64().ok_or("register id must be number")? as i32,
            ))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    for (name, hex) in registers.as_object().ok_or("registers missing")? {
        let id = *ids
            .get(name)
            .ok_or_else(|| format!("missing register ID for {name}"))?;
        let value = value_u128(hex)?;
        if name.starts_with('q') {
            let bytes = value.to_le_bytes();
            let rc = unsafe { (api.reg_write)(uc, id, bytes.as_ptr().cast()) };
            if rc != 0 {
                return Err(format!("restore {name}: {}", api.err(rc)));
            }
        } else {
            let value = value as u64;
            let rc = unsafe { (api.reg_write)(uc, id, (&value as *const u64).cast()) };
            if rc != 0 {
                return Err(format!("restore {name}: {}", api.err(rc)));
            }
        }
    }
    let context = num(&call_values["context"])?;
    let normal_test_row = num(&call_values["confirm_original_Normal"])?;
    let world = num(&call_values["battle_world"])?;
    let battle = num(&call_values["battle_state"])?;
    let harness = num(&call_values["battle_harness"])?;
    let entities = num(&call_values["battle_entities"])?;
    let commands = num(&call_values["battle_commands"])?;
    let imports = abi["import_symbols"]
        .as_object()
        .ok_or("import symbols missing")?
        .iter()
        .filter_map(|(key, value)| {
            Some((
                num(&Value::String(key.clone())).ok()?,
                value.as_str()?.to_string(),
            ))
        })
        .collect::<BTreeMap<_, _>>();
    let mut state = Box::new(HookState {
        api: api as *const UnicornApi,
        uc,
        imports,
        ids: ids.clone(),
        heap,
        baseline_clock,
        clock_calls: 0,
        calls: BTreeMap::new(),
        error: None,
        returned: false,
        return_address: retaddr,
    });
    let mut hook = 0usize;
    let rc = unsafe {
        (api.hook_add)(
            uc,
            &mut hook,
            UC_HOOK_CODE,
            code_hook as *const c_void,
            (&mut *state as *mut HookState).cast(),
            external,
            external + external_size - 1,
        )
    };
    if rc != 0 {
        return Err(format!("uc_hook_add imports: {}", api.err(rc)));
    }
    let mut ret_hook = 0usize;
    let rc = unsafe {
        (api.hook_add)(
            uc,
            &mut ret_hook,
            UC_HOOK_CODE,
            code_hook as *const c_void,
            (&mut *state as *mut HookState).cast(),
            retaddr,
            retaddr,
        )
    };
    if rc != 0 {
        return Err(format!("uc_hook_add return: {}", api.err(rc)));
    }
    let descriptor_scratch = state.heap.alloc(24, 16, api, uc)?;
    write_mem(api, uc, descriptor_scratch, &[0; 24])?;
    Ok(Populated {
        ids,
        abi,
        state,
        world,
        battle,
        harness,
        entities,
        commands,
        context,
        normal_test_row,
        descriptor_scratch,
    })
}


impl NativeSession {
    /// Collect original basic-block execution counts for selecting source ports.
    /// Profiling is opt-in and the hook is removed before this method returns.
    pub fn profile_original_match(&mut self) -> Result<Value, String> {
        unsafe extern "C" fn count_block(
            _: *mut c_void,
            address: u64,
            size: u32,
            opaque: *mut c_void,
        ) {
            let counts = &mut *opaque.cast::<BTreeMap<u64, (u64, u32)>>();
            let entry = counts.entry(address).or_insert((0, size));
            entry.0 += 1;
        }
        let mut counts = Box::new(BTreeMap::<u64, (u64, u32)>::new());
        let mut hook = 0;
        let rc = unsafe {
            (self.api.hook_add)(
                self.uc,
                &mut hook,
                1 << 3,
                count_block as *const c_void,
                (&mut *counts as *mut BTreeMap<u64, (u64, u32)>).cast(),
                1u64,
                0u64,
            )
        };
        if rc != 0 {
            return Err(format!("profile hook: {}", self.api.err(rc)));
        }
        let result = self.native_match_until_terminal();
        let rc = unsafe { (self.api.hook_del)(self.uc, hook) };
        if rc != 0 {
            return Err(format!("remove profile hook: {}", self.api.err(rc)));
        }
        let outcome = result?;
        let blocks: Vec<_> = counts.iter().map(|(address, (calls, size))|
            json!({"rva":format!("{:#x}",address-0x10000000),"executions":calls,"bytes":size})).collect();
        Ok(
            json!({"source_sha256":SOURCE_SHA256,"outcome":outcome,"blocks":blocks,
            "scope":"Original fixed-snapshot harness match. Counts are dynamic basic blocks, not wall-time samples; profiling overhead excluded from performance claims."}),
        )
    }

    pub fn load(checkpoint_dir: &Path) -> Result<Self, String> {
        PristineTemplate::load_verified(checkpoint_dir)?.into_session()
    }

    /// Build a session from a verified in-memory template. Does not read or
    /// hash checkpoint files.
    pub fn from_template(template: &PristineTemplate) -> Result<Self, String> {
        template.instantiate()
    }

    /// Start counting every executed basic block in the original source ELF.
    /// Profiling is disabled by default and is intended for code coverage, not
    /// for measuring runtime performance.
    pub fn start_block_profile(&mut self) -> Result<(), String> {
        if self.block_profile.is_some() {
            return Err("basic-block profiling is already active".into());
        }
        let mut counts = Box::new(BTreeMap::<u64, (u64, u32)>::new());
        let mut hook = 0usize;
        let rc = unsafe {
            (self.api.hook_add)(
                self.uc,
                &mut hook,
                UC_HOOK_BLOCK,
                block_profile_hook as *const c_void,
                (&mut *counts as *mut BTreeMap<u64, (u64, u32)>).cast(),
                1,
                0,
            )
        };
        if rc != 0 {
            return Err(format!("start basic-block profile: {}", self.api.err(rc)));
        }
        self.block_profile = Some(BlockProfile { hook, counts });
        Ok(())
    }

    /// Remove the active counter hook and return its source-relative profile.
    pub fn finish_block_profile(&mut self) -> Result<Value, String> {
        let profile = self
            .block_profile
            .as_ref()
            .ok_or("basic-block profiling is not active")?;
        let rc = unsafe { (self.api.hook_del)(self.uc, profile.hook) };
        if rc != 0 {
            return Err(format!("remove basic-block profile: {}", self.api.err(rc)));
        }
        let profile = self.block_profile.take().expect("profile checked above");
        let blocks = profile
            .counts
            .iter()
            .map(|(address, (executions, bytes))| {
                json!({
                    "rva": format!("{:#x}", address - 0x1000_0000),
                    "bytes": bytes,
                    "executions": executions,
                })
            })
            .collect::<Vec<_>>();
        Ok(json!({
            "source_sha256": SOURCE_SHA256,
            "blocks": blocks,
            "scope": "Original session-wide basic-block counts across the external RL episode. Counts are not wall-time samples; profiling overhead is excluded from performance claims.",
        }))
    }

    /// Read-only state audit for a fresh-deck session. Kept separate from the
    /// training protocol so reference validation cannot modify a live episode.
    pub fn audit_snapshot(&self, checkpoint_dir: &Path, out: &Path) -> Result<(), String> {
        if out.exists() {
            return Err(format!("audit output already exists: {}", out.display()));
        }
        if sha256(&checkpoint_dir.join("checkpoint.json"))? != self.initial_checkpoint_sha256 {
            return Err("audit checkpoint identity mismatch".into());
        }
        let cp: Value = serde_json::from_slice(
            &fs::read(checkpoint_dir.join("checkpoint.json")).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        fs::create_dir_all(out.join("regions")).map_err(|e| e.to_string())?;
        let mut rows = Vec::new();
        let mut captured = BTreeSet::new();
        for region in cp["regions"].as_array().ok_or("missing regions")? {
            let base = num(&region["base"])?;
            let file = region["file"].as_str().ok_or("missing region filename")?;
            let size = if base == self.state.heap.base {
                align(self.state.heap.pointer - base, 4096)?
            } else if let Some(arena) = self.state.heap.extras.iter().find(|a| a.base == base) {
                align(arena.pointer - base, 4096)?
            } else {
                num(&region["size"])?
            };
            let path = out.join("regions").join(file);
            fs::write(&path, read_mem(&self.api, self.uc, base, size as usize)?)
                .map_err(|e| e.to_string())?;
            rows.push(json!({"file":format!("regions/{file}"),"base":format!("{base:#x}"),"size":size,"sha256":sha256(&path)?}));
            captured.insert(base);
        }
        for (index, arena) in self
            .state
            .heap
            .extras
            .iter()
            .enumerate()
            .filter(|(_, a)| !captured.contains(&a.base))
        {
            let size = align(arena.pointer - arena.base, 4096)?;
            let file = format!("heap_extra_{index:02}.bin");
            let path = out.join("regions").join(&file);
            fs::write(
                &path,
                read_mem(&self.api, self.uc, arena.base, size as usize)?,
            )
            .map_err(|e| e.to_string())?;
            rows.push(json!({"file":format!("regions/{file}"),"base":format!("{:#x}",arena.base),"size":size,"sha256":sha256(&path)?}));
        }
        let mut registers = serde_json::Map::new();
        for (name, id) in &self.ids {
            let value = if name.starts_with('q') {
                let mut bytes = [0u8; 16];
                let rc = unsafe { (self.api.reg_read)(self.uc, *id, bytes.as_mut_ptr().cast()) };
                if rc != 0 {
                    return Err(format!("audit register {name}: {}", self.api.err(rc)));
                }
                u128::from_le_bytes(bytes)
            } else {
                read_u64(&self.api, self.uc, *id)? as u128
            };
            registers.insert(name.clone(), json!(format!("0x{value:x}")));
        }
        let heap = &self.state.heap;
        let allocations = heap
            .allocations
            .iter()
            .map(|(p, n)| (p.to_string(), json!(n)))
            .collect::<serde_json::Map<_, _>>();
        let free = heap
            .free
            .iter()
            .map(|(size, items)| (size.to_string(), json!(items)))
            .collect::<serde_json::Map<_, _>>();
        let allocator = json!({"version":1,"base":heap.base,"capacity":heap.capacity,"primary_pointer":heap.pointer,"allocations":allocations,"free_buckets":free,"extra_heaps":heap.extras.iter().map(|a|json!({"base":a.base,"capacity":a.capacity,"pointer":a.pointer})).collect::<Vec<_>>()});
        fs::write(
            out.join("allocator.json"),
            serde_json::to_vec(&allocator).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let meta = json!({"source_sha256":SOURCE_SHA256,"checkpoint_sha256":self.initial_checkpoint_sha256,"regions":rows,"registers":registers,"allocator_file":"allocator.json","imports_called":self.state.calls});
        fs::write(
            out.join("audit.json"),
            serde_json::to_vec_pretty(&meta).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn call(&mut self, rva: u64, args: &[u64], x8: u64) -> Result<u64, String> {
        invoke_native(
            &self.api,
            self.uc,
            &self.ids,
            &mut self.state,
            &self.abi,
            rva,
            args,
            x8,
        )
    }

    /// Mirrored-deck convenience wrapper for
    /// [`configure_normal_decks_and_rebuild`](Self::configure_normal_decks_and_rebuild).
    pub fn configure_normal_deck_and_rebuild(
        &mut self,
        cards: &[&str],
        seed: u32,
    ) -> Result<Value, String> {
        self.configure_normal_decks_and_rebuild(cards, cards, seed)
    }

    /// Replace the loaded Normal testing row's AlwaysSpells source-row vector,
    /// then call the original input/harness constructors to make a fresh game.
    /// The scenario supplies one vector to both sides, so an asymmetric
    /// `cards_side1` is installed only for the original `11d6f18` call that
    /// builds side 1. No battle object or game instruction is patched.
    pub fn configure_normal_decks_and_rebuild(
        &mut self,
        cards: &[&str],
        cards_side1: &[&str],
        seed: u32,
    ) -> Result<Value, String> {
        for (label, deck) in [("side 0", cards), ("side 1", cards_side1)] {
            if deck.len() != 8 {
                return Err(format!(
                    "the {label} deck must contain exactly 8 cards, got {}",
                    deck.len()
                ));
            }
            let mut unique = BTreeSet::new();
            if deck
                .iter()
                .any(|card| card.is_empty() || !unique.insert(*card))
            {
                return Err(format!("{label} deck names must be nonempty and unique"));
            }
        }

        let row_count = self.call(0xe1ac8c, &[self.context], 0)?;
        if row_count == 0 || row_count > 100_000 {
            return Err(format!("unexpected native card-row count {row_count}"));
        }
        let mut source_rows = BTreeMap::<String, u64>::new();
        for index in 0..row_count {
            let row = self.call(0xe1ad4c, &[self.context, index], 0)?;
            if row == 0 {
                return Err(format!("native card row {index} is null"));
            }
            let name = self.native_string(row + 0x28)?;
            if (cards.contains(&name.as_str()) || cards_side1.contains(&name.as_str()))
                && source_rows.insert(name.clone(), row).is_some()
            {
                return Err(format!("duplicate native source row name {name}"));
            }
        }
        let missing: Vec<_> = cards
            .iter()
            .chain(cards_side1)
            .filter(|name| !source_rows.contains_key(**name))
            .copied()
            .collect();
        if !missing.is_empty() {
            return Err(format!(
                "native source rows not found by exact name: {}",
                missing.join(", ")
            ));
        }
        let mut deck_vector = |deck: &[&str]| -> Result<(u64, Vec<u64>), String> {
            let row_ptrs: Vec<u64> = deck.iter().map(|name| source_rows[*name]).collect();
            let data = self
                .state
                .heap
                .alloc(8 * row_ptrs.len() as u64, 8, &self.api, self.uc)?;
            write_mem(
                &self.api,
                self.uc,
                data,
                &row_ptrs
                    .iter()
                    .flat_map(|p| p.to_le_bytes())
                    .collect::<Vec<_>>(),
            )?;
            Ok((data, row_ptrs))
        };
        let (vector_data, row_ptrs) = deck_vector(cards)?;
        write_mem(
            &self.api,
            self.uc,
            self.normal_test_row + 0x70,
            &vector_data.to_le_bytes(),
        )?;
        write_mem(
            &self.api,
            self.uc,
            self.normal_test_row + 0x78,
            &(row_ptrs.len() as u32).to_le_bytes(),
        )?;
        write_mem(
            &self.api,
            self.uc,
            self.normal_test_row + 0x7c,
            &(row_ptrs.len() as u32).to_le_bytes(),
        )?;
        let second_vector = if cards_side1 != cards {
            let (data, pointers) = deck_vector(cards_side1)?;
            Some((data, pointers.len() as u32))
        } else {
            None
        };

        // Re-run the same original constructor chain as the scenario harness.
        let name_bytes = self.state.heap.alloc(7, 8, &self.api, self.uc)?;
        write_mem(&self.api, self.uc, name_bytes, b"Normal\0")?;
        let name_string = self.state.heap.alloc(0x10, 8, &self.api, self.uc)?;
        write_mem(&self.api, self.uc, name_string, &[0; 0x10])?;
        self.call(0x142fde4, &[name_string, name_bytes], 0)?;
        let resolved = self.call(0xe1b7e8, &[self.context, name_string, 0], 0)?;
        if resolved != self.normal_test_row {
            return Err(format!(
                "Normal lookup changed unexpectedly: {resolved:#x} != {:#x}",
                self.normal_test_row
            ));
        }
        let input_sret = self.state.heap.alloc(8, 8, &self.api, self.uc)?;
        write_mem(&self.api, self.uc, input_sret, &[0; 8])?;
        self.call(
            0x11d7968,
            &[self.context, name_string, seed as u64, 0],
            input_sret,
        )?;
        let battle_input = u64::from_le_bytes(
            read_mem(&self.api, self.uc, input_sret, 8)?
                .try_into()
                .unwrap(),
        );
        if battle_input == 0 {
            return Err("original battle-input constructor returned null".into());
        }
        let new_harness = self.state.heap.alloc(0x78, 16, &self.api, self.uc)?;
        write_mem(&self.api, self.uc, new_harness, &[0; 0x78])?;
        let mut swap = second_vector.map(|(data, count)| {
            Box::new(DeckSwapHook {
                api: &*self.api as *const UnicornApi,
                uc: self.uc,
                entry: 0x1000_0000 + 0x11d6f18,
                restore: 0x1000_0000 + 0x11d6_068,
                slot: self.normal_test_row + 0x70,
                second_ptr: data,
                second_count: count,
                original_ptr: vector_data,
                original_count: row_ptrs.len() as u32,
                calls: 0,
                error: None,
            })
        });
        let mut hooks = Vec::new();
        if let Some(state) = swap.as_mut() {
            for address in [state.entry, state.restore] {
                let mut hook = 0usize;
                let rc = unsafe {
                    (self.api.hook_add)(
                        self.uc,
                        &mut hook,
                        UC_HOOK_CODE,
                        deck_swap_hook as *const c_void,
                        (&mut **state as *mut DeckSwapHook).cast(),
                        address,
                        address,
                    )
                };
                if rc != 0 {
                    for installed in hooks.drain(..) {
                        unsafe { (self.api.hook_del)(self.uc, installed) };
                    }
                    return Err(format!("deck-swap hook: {}", self.api.err(rc)));
                }
                hooks.push(hook);
            }
        }
        let constructed = self.call(0x11d58e0, &[new_harness, self.context, input_sret], 0);
        for hook in hooks {
            unsafe { (self.api.hook_del)(self.uc, hook) };
        }
        constructed?;
        if let Some(state) = swap {
            if let Some(error) = state.error {
                return Err(error);
            }
            if state.calls != 2 {
                return Err(format!(
                    "expected two native 11d6f18 deck builds, observed {}",
                    state.calls
                ));
            }
        }
        let new_world = u64::from_le_bytes(
            read_mem(&self.api, self.uc, new_harness + 0x20, 8)?
                .try_into()
                .unwrap(),
        );
        if new_world == 0 {
            return Err("original harness constructor returned a null world".into());
        }
        let new_battle = u64::from_le_bytes(
            read_mem(&self.api, self.uc, new_world + 0xa8, 8)?
                .try_into()
                .unwrap(),
        );
        let new_entities = u64::from_le_bytes(
            read_mem(&self.api, self.uc, new_world + 0x40, 8)?
                .try_into()
                .unwrap(),
        );
        let new_commands = u64::from_le_bytes(
            read_mem(&self.api, self.uc, new_world + 0x38, 8)?
                .try_into()
                .unwrap(),
        );
        if new_battle == 0 || new_entities == 0 || new_commands == 0 {
            return Err(
                "fresh native harness has a null battle, entity manager, or command queue".into(),
            );
        }
        self.harness = new_harness;
        self.world = new_world;
        self.battle = new_battle;
        self.entities = new_entities;
        self.commands = new_commands;
        self.presentation_start_tick = read_mem_u32(&self.api, self.uc, self.world + 0x60)?;
        self.presentation_names.clear();
        self.presentation_positions.clear();
        self.presentation_first_seen_ticks.clear();

        let mut actual_decks = Vec::new();
        for side in 0..2u64 {
            let deck = u64::from_le_bytes(
                read_mem(&self.api, self.uc, self.battle + 0x88 + side * 8, 8)?
                    .try_into()
                    .unwrap(),
            );
            if deck == 0 {
                return Err(format!("fresh native side {side} has null deck"));
            }
            let count = u32::from_le_bytes(
                read_mem(&self.api, self.uc, deck + 0x2c, 4)?
                    .try_into()
                    .unwrap(),
            );
            if count != 8 {
                return Err(format!(
                    "fresh native side {side} deck has {count} rows, expected 8"
                ));
            }
            let entries = u64::from_le_bytes(
                read_mem(&self.api, self.uc, deck + 0x20, 8)?
                    .try_into()
                    .unwrap(),
            );
            let side_obj = self.side_object(side as u32)?;
            let mut names = Vec::new();
            for index in 0..count as u64 {
                let wrapper = u64::from_le_bytes(
                    read_mem(&self.api, self.uc, entries + index * 8, 8)?
                        .try_into()
                        .unwrap(),
                );
                if wrapper == 0 {
                    return Err(format!("native side {side} deck entry {index} is null"));
                }
                write_mem(&self.api, self.uc, self.descriptor_scratch, &[0; 24])?;
                self.call(0xe80bbc, &[wrapper, side_obj, 1], self.descriptor_scratch)?;
                let row = self.call(0xe80680, &[self.descriptor_scratch], 0)?;
                if row == 0 {
                    return Err(format!(
                        "native side {side} entry {index} has null source row"
                    ));
                }
                names.push(self.native_string(row + 0x28)?);
            }
            let mut sorted_actual = names.clone();
            let expected = if side == 0 { cards } else { cards_side1 };
            let mut sorted_expected = expected
                .iter()
                .map(|name| name.to_string())
                .collect::<Vec<_>>();
            sorted_actual.sort();
            sorted_expected.sort();
            if sorted_actual != sorted_expected {
                return Err(format!("native side {side} deck rows mismatch: {names:?}"));
            }
            actual_decks.push(json!({"side":side,"cards":names}));
        }
        let observation = self.observe()?;
        let deck_source_rows = if cards == cards_side1 {
            json!(cards)
        } else {
            json!([cards, cards_side1])
        };
        Ok(
            json!({"scenario":"Normal","seed":seed,"deck_source_rows":deck_source_rows,"actual_native_decks":actual_decks,"observation":observation,"construction":"Original 11d7968 input and 11d58e0 fresh-harness constructors after setting the loaded Normal row's AlwaysSpells vector; for asymmetric decks the second native 11d6f18 call receives a separate source-row vector. No game instruction is patched."}),
        )
    }

    /// Return an authoritative read-only presentation snapshot matching
    /// native-spectator's BattleFrame JSON layout. Only guest memory is read;
    /// movement is derived from this session's preceding snapshot.
    pub fn presentation_snapshot(&mut self) -> Result<Value, String> {
        let world = self.world;
        let battle = self.battle;
        let tick = read_mem_u32(&self.api, self.uc, world + 0x60)?;
        let manager = read_mem_u64(&self.api, self.uc, world + 0x40)?;
        let count = read_mem_i32(&self.api, self.uc, manager + 0x14)?;
        if !(0..=8192).contains(&count) {
            return Err(format!("invalid native entity count {count}"));
        }
        let array = read_mem_u64(&self.api, self.uc, manager + 8)?;
        let entity_addresses = (0..count as u64)
            .map(|index| read_mem_u64(&self.api, self.uc, array + index * 8))
            .collect::<Result<Vec<_>, _>>()?;
        // Original f8c9d8 passes each entity's second word (entity+8) as an
        // integer identity when removing it; fc7e20 stores that same word in
        // the candidate-target ID vector.
        let entity_ids = entity_addresses
            .iter()
            .map(|address| Ok((*address, read_mem_i32(&self.api, self.uc, address + 8)?)))
            .collect::<Result<BTreeMap<_, _>, String>>()?;
        let mut entities = Vec::with_capacity(count as usize);
        let mut positions = BTreeMap::new();
        for address in entity_addresses {
            let row = read_mem_u64(&self.api, self.uc, address + 0x48)?;
            if !self.presentation_names.contains_key(&row) {
                let name = self.native_string(row + 0x28)?;
                self.presentation_names.insert(row, name);
            }
            let first_seen_tick = *self
                .presentation_first_seen_ticks
                .entry(address)
                .or_insert(tick);
            let side = read_mem_i32(&self.api, self.uc, address + 0x78)?;
            let x = read_mem_i32(&self.api, self.uc, address + 0x7c)?;
            let y = read_mem_i32(&self.api, self.uc, address + 0x80)?;
            let z = read_mem_i32(&self.api, self.uc, address + 0x74)?.saturating_add(read_mem_i32(
                &self.api,
                self.uc,
                address + 0x8c,
            )?);
            let component_count = read_mem_i32(&self.api, self.uc, address + 0x24)?;
            if !(0..=32).contains(&component_count) {
                return Err("unrecognized native component layout".into());
            }
            let mut hp = -1;
            let mut max_hp = 0;
            let mut native_focus_entity = None;
            let mut native_action_timer_raw = None;
            let mut native_secondary_timer_raw = None;
            if component_count > 0 {
                let component_mask = read_mem_u32(&self.api, self.uc, address + 0x30)?;
                let component_table = read_mem_u64(&self.api, self.uc, address + 0x18)?;
                // Concrete target/behavior component observed on native unit
                // and tower entities. Its class is confirmed by the original
                // vtable at RVA 0x193b570; original f8cd90 dispatches its
                // update at virtual slot +0x18 when entity mask bit 0 is set.
                // In original fc7e20, +0x10 is used as an entity target and
                // +0x48/+0x44 are countdown fields. Keep the timer values raw:
                // they are not a source-proven render animation flag.
                if component_mask & 1 != 0 && component_table != 0 {
                    let behavior = read_mem_u64(&self.api, self.uc, component_table)?;
                    if behavior != 0 && read_mem_u64(&self.api, self.uc, behavior)? == 0x1193b570 {
                        let target = read_mem_u64(&self.api, self.uc, behavior + 0x10)?;
                        native_focus_entity = (target != 0).then_some(target);
                        native_action_timer_raw =
                            Some(read_mem_i32(&self.api, self.uc, behavior + 0x48)?);
                        native_secondary_timer_raw =
                            Some(read_mem_i32(&self.api, self.uc, behavior + 0x44)?);
                    }
                }
            }
            let base_vtable = read_mem_u64(&self.api, self.uc, address)?;
            // Projectile class observed as TowerPrincessProjectile at runtime
            // has original vtable RVA 0x1936050; its virtual getter f93450
            // returns this byte verbatim. Preserve the byte's identity rather
            // than assigning an undocumented active/visibility meaning.
            let projectile_flag_0x170 = if base_vtable == 0x11936050 {
                Some(read_mem(&self.api, self.uc, address + 0x170, 1)?[0] != 0)
            } else {
                None
            };
            if component_count > 2 {
                let table = read_mem_u64(&self.api, self.uc, address + 0x18)?;
                let health = read_mem_u64(&self.api, self.uc, table + 16)?;
                if health != 0 && read_mem_u64(&self.api, self.uc, health)? == 0x1193b660 {
                    hp = read_mem_i32(&self.api, self.uc, health + 0x10)?;
                    max_hp = read_mem_i32(&self.api, self.uc, health + 0x14)?;
                }
            }
            let moving = self
                .presentation_positions
                .get(&address)
                .is_some_and(|previous| *previous != (x, y));
            // Keep the exact world-space displacement as presentation data.
            // This is movement direction only; it is not a claimed native
            // facing/animation state. The first observation of an entity has
            // no prior position and therefore no displacement.
            let motion_delta = self.presentation_positions.get(&address).map(|previous| {
                (
                    i64::from(x) - i64::from(previous.0),
                    i64::from(y) - i64::from(previous.1),
                )
            });
            positions.insert(address, (x, y));
            entities.push(json!({
                "address":address,
                "native_entity_id":entity_ids[&address],
                "row":self.presentation_names[&row],
                "side":side,
                "x":x,
                "y":y,
                "z":z,
                "hp":hp,
                "max_hp":max_hp,
                "moving":moving,
                "motion_dx":motion_delta.map(|delta| delta.0),
                "motion_dy":motion_delta.map(|delta| delta.1),
                "first_seen_tick":first_seen_tick,
                "native_focus_entity":native_focus_entity,
                "native_focus_entity_id":native_focus_entity.and_then(|target| entity_ids.get(&target).copied()),
                "native_action_timer_raw":native_action_timer_raw,
                "native_secondary_timer_raw":native_secondary_timer_raw,
                "projectile_flag_0x170":projectile_flag_0x170,
                "attacking":false,
            }));
        }
        self.presentation_first_seen_ticks
            .retain(|address, _| positions.contains_key(address));
        self.presentation_positions = positions;

        let mut elixir = [0.0f64; 2];
        for side in 0..2u64 {
            let side_obj = read_mem_u64(&self.api, self.uc, battle + 0xe0 + side * 8)?;
            let raw = read_mem_i32(&self.api, self.uc, side_obj + 0x2f8)?;
            let reservations = read_mem_u64(&self.api, self.uc, side_obj + 0x248)?;
            let mut reserved = 0i32;
            if reservations != 0 {
                let n = read_mem_i32(&self.api, self.uc, reservations + 0x1c)?;
                if !(0..=4096).contains(&n) {
                    return Err("native mana reservations out of range".into());
                }
                let data = read_mem_u64(&self.api, self.uc, reservations + 0x10)?;
                for index in 0..n as u64 {
                    let item = read_mem_u64(&self.api, self.uc, data + index * 8)?;
                    reserved =
                        reserved.wrapping_add(read_mem_i32(&self.api, self.uc, item + 0x10)?);
                }
            }
            let context_link = read_mem_u64(&self.api, self.uc, side_obj + 0x48)?;
            let context = read_mem_u64(&self.api, self.uc, context_link + 0x10)?;
            let globals_link = read_mem_u64(&self.api, self.uc, context + 0x78)?;
            let globals = read_mem_u64(&self.api, self.uc, globals_link + 0x18)?;
            let cap = read_mem_i32(&self.api, self.uc, globals + 0x1a8)?;
            elixir[side as usize] =
                (raw as f64 / 10000.0 - reserved as f64).clamp(0.0, cap.max(0) as f64);
        }
        let terminated = read_mem(&self.api, self.uc, battle + 0x1e0, 1)?[0] != 0;
        let winner_side = if terminated {
            Some(read_mem_i32(&self.api, self.uc, battle + 0x1b8)?)
        } else {
            None
        };
        let arena_size = [
            read_mem_i32(&self.api, self.uc, battle + 0x188)?.saturating_mul(500),
            read_mem_i32(&self.api, self.uc, battle + 0x18c)?.saturating_mul(500),
        ];
        Ok(json!({
            "tick":tick,
            "elapsed_ms":tick.saturating_sub(self.presentation_start_tick) as u64 * 50,
            "terminated":terminated,
            "winner_side":winner_side,
            "arena_size":arena_size,
            "elixir":elixir,
            "entities":entities,
        }))
    }

    /// Return the concrete component vtables for current entities. This is a
    /// read-only reverse-engineering aid; callers must not assign semantics to
    /// a slot or class without an original-code guard/accessor.
    pub fn presentation_component_vtables(&mut self) -> Result<Value, String> {
        let world = self.world;
        let manager = read_mem_u64(&self.api, self.uc, world + 0x40)?;
        let count = read_mem_i32(&self.api, self.uc, manager + 0x14)?;
        if !(0..=8192).contains(&count) {
            return Err(format!("invalid native entity count {count}"));
        }
        let array = read_mem_u64(&self.api, self.uc, manager + 8)?;
        let mut entities = Vec::with_capacity(count as usize);
        for index in 0..count as u64 {
            let address = read_mem_u64(&self.api, self.uc, array + index * 8)?;
            let row = read_mem_u64(&self.api, self.uc, address + 0x48)?;
            let name = if let Some(name) = self.presentation_names.get(&row) {
                name.clone()
            } else {
                self.native_string(row + 0x28)?
            };
            let component_count = read_mem_i32(&self.api, self.uc, address + 0x24)?;
            if !(0..=32).contains(&component_count) {
                return Err("unrecognized native component layout".into());
            }
            let mask = read_mem_u32(&self.api, self.uc, address + 0x30)?;
            let table = read_mem_u64(&self.api, self.uc, address + 0x18)?;
            let mut components = Vec::new();
            for slot in 0..component_count as u64 {
                let component = read_mem_u64(&self.api, self.uc, table + slot * 8)?;
                if component != 0 {
                    components.push(json!({
                        "slot":slot,
                        "enabled_by_mask":mask & (1u32 << slot) != 0,
                        "component":component,
                        "vtable":read_mem_u64(&self.api, self.uc, component)?,
                    }));
                }
            }
            let base_vtable = read_mem_u64(&self.api, self.uc, address)?;
            entities.push(
                json!({"entity":address,"row":name,"base_vtable":base_vtable,"components":components}),
            );
        }
        Ok(Value::Array(entities))
    }

    fn ticks(&self) -> Result<[u32; 2], String> {
        Ok([
            u32::from_le_bytes(
                read_mem(&self.api, self.uc, self.world + 0x60, 4)?
                    .try_into()
                    .unwrap(),
            ),
            u32::from_le_bytes(
                read_mem(&self.api, self.uc, self.world + 0x70, 4)?
                    .try_into()
                    .unwrap(),
            ),
        ])
    }

    fn side_object(&mut self, side: u32) -> Result<u64, String> {
        self.call(0xd35d5c, &[self.battle, side as u64], 0)
    }

    fn native_string(&self, address: u64) -> Result<String, String> {
        let header = read_mem(&self.api, self.uc, address, 16)?;
        let len = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
        if len > 256 {
            return Err(format!("native row string too long: {len}"));
        }
        if len == 0 {
            return Ok(String::new());
        }
        let bytes = if len < 8 {
            header[8..8 + len].to_vec()
        } else {
            let ptr = u64::from_le_bytes(header[8..16].try_into().unwrap());
            read_mem(&self.api, self.uc, ptr, len)?
        };
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }

    /// Advance one original world tick.
    pub fn world_step(&mut self) -> Result<(), String> {
        self.call(0x10ca81c, &[self.world], 0)?;
        Ok(())
    }

    /// Read the source-proven native match-completion byte without a full observation.
    pub fn is_terminal(&self) -> Result<bool, String> {
        Ok(read_mem(&self.api, self.uc, self.battle + 0x1e0, 1)?[0] != 0)
    }

    /// Query the original target validator for every policy heatmap center.
    /// This never submits a command or advances the world.
    pub fn placement_mask(&mut self, side: u32, slot: u32) -> Result<Value, String> {
        if side > 1 || slot > 3 {
            return Err("placement_mask needs side 0..1 and slot 0..3".into());
        }
        let side_obj = self.side_object(side)?;
        if side_obj == 0 { return Err("null native battle side".into()); }
        let tick = read_mem_u32(&self.api, self.uc, self.world + 0x60)?;
        let queue = read_mem_u32(&self.api, self.uc, self.commands + 0x14)?;
        let wrapper = self.call(0xf9dd98, &[side_obj, slot as u64], 0)?;
        if wrapper == 0 { return Err("placement_mask requested an empty hand slot".into()); }
        write_mem(&self.api, self.uc, self.descriptor_scratch, &[0; 24])?;
        self.call(0xe80bbc, &[wrapper, side_obj, 1], self.descriptor_scratch)?;
        let proxy = self.call(0xe80680, &[self.descriptor_scratch], 0)?;
        if proxy == 0 { return Err("null native card selection proxy".into()); }
        let mut mask = Vec::with_capacity(576);
        let started = Instant::now();
        for tile in 0..576 {
            let x = (((tile % 32) as f64 + 0.5) * 18000.0 / 32.0).round() as i32;
            let y = ((tile / 32) as f64 + 0.5) / 18.0;
            let y = ((if side == 1 { 1.0 - y } else { y }) * 32000.0).round() as i32;
            let code = self.call(0xd42214, &[self.battle, x as u64, y as u64, proxy, 0], 0)? as u32;
            mask.push(code == 0);
        }
        if tick != read_mem_u32(&self.api, self.uc, self.world + 0x60)?
            || queue != read_mem_u32(&self.api, self.uc, self.commands + 0x14)? {
            return Err("native placement query changed tick or command queue".into());
        }
        Ok(json!({"side":side,"slot":slot,"mask":mask,"tick":tick,"query_seconds":started.elapsed().as_secs_f64()}))
    }

    /// Query the original target validator at the centre of every real arena
    /// tile (18 x 32, 1000 native units each), indexed `row * 18 + column` in
    /// absolute coordinates. For interactive placement previews; never submits
    /// a command or advances the world.
    pub fn placement_tiles(&mut self, side: u32, slot: u32) -> Result<Value, String> {
        if side > 1 || slot > 3 {
            return Err("placement_tiles needs side 0..1 and slot 0..3".into());
        }
        let side_obj = self.side_object(side)?;
        if side_obj == 0 { return Err("null native battle side".into()); }
        let tick = read_mem_u32(&self.api, self.uc, self.world + 0x60)?;
        let queue = read_mem_u32(&self.api, self.uc, self.commands + 0x14)?;
        let wrapper = self.call(0xf9dd98, &[side_obj, slot as u64], 0)?;
        if wrapper == 0 { return Err("placement_tiles requested an empty hand slot".into()); }
        write_mem(&self.api, self.uc, self.descriptor_scratch, &[0; 24])?;
        self.call(0xe80bbc, &[wrapper, side_obj, 1], self.descriptor_scratch)?;
        let proxy = self.call(0xe80680, &[self.descriptor_scratch], 0)?;
        if proxy == 0 { return Err("null native card selection proxy".into()); }
        let mut mask = Vec::with_capacity(18 * 32);
        for tile in 0..18 * 32 {
            let x = (tile % 18) * 1000 + 500;
            let y = (tile / 18) * 1000 + 500;
            let code = self.call(0xd42214, &[self.battle, x as u64, y as u64, proxy, 0], 0)? as u32;
            mask.push(code == 0);
        }
        if tick != read_mem_u32(&self.api, self.uc, self.world + 0x60)?
            || queue != read_mem_u32(&self.api, self.uc, self.commands + 0x14)? {
            return Err("native placement query changed tick or command queue".into());
        }
        Ok(json!({"side":side,"slot":slot,"columns":18,"rows":32,"mask":mask,"tick":tick}))
    }

    /// Resolve a requested deployment through the original placement routine.
    /// Unlike d42214 (which only rejects blocked/out-of-arena targets),
    /// f9e16c applies territory, pocket and card-specific placement rules.
    /// Both the human caller be1634 and native deployment fa35c4 use it.
    /// No command is submitted and no world time is advanced.
    pub fn resolve_target(&mut self, request: PlayCardRequest) -> Result<Value, String> {
        if request.side > 1 || request.slot > 3 {
            return Err("target resolution needs side 0..1 and slot 0..3".into());
        }
        let side_obj = self.side_object(request.side)?;
        if side_obj == 0 { return Err("null native battle side".into()); }
        let tick = read_mem_u32(&self.api, self.uc, self.world + 0x60)?;
        let queue = read_mem_u32(&self.api, self.uc, self.commands + 0x14)?;
        let wrapper = self.call(0xf9dd98, &[side_obj, request.slot as u64], 0)?;
        if wrapper == 0 {
            return Ok(json!({"valid":false,"rejection":"empty_hand_slot"}));
        }
        write_mem(&self.api, self.uc, self.descriptor_scratch, &[0; 24])?;
        self.call(0xe80bbc, &[wrapper, side_obj, 1], self.descriptor_scratch)?;
        let proxy = self.call(0xe80680, &[self.descriptor_scratch], 0)?;
        if proxy == 0 { return Err("null native card selection proxy".into()); }
        let target = self.state.heap.alloc(8, 8, &self.api, self.uc)?;
        let resolved = (|| -> Result<Option<(i32, i32)>, String> {
            write_mem(&self.api, self.uc, target, &[0xff; 8])?;
            let mode = self.call(0xf93d3c, &[side_obj], 0)?;
            // The ninth AArch64 argument is a false byte on the caller's stack,
            // as in fa3688; the first eight arguments are passed in x0..x7.
            let stack = num(&self.abi["call_stack_pointer"])?;
            write_mem(&self.api, self.uc, stack, &[0; 8])?;
            let valid = self.call(0xf9e16c, &[
                side_obj, proxy, request.x as i64 as u64, request.y as i64 as u64,
                target, self.world, 0, mode,
            ], 0)?;
            if valid & 1 == 0 { return Ok(None); }
            Ok(Some((read_mem_i32(&self.api, self.uc, target)?,
                     read_mem_i32(&self.api, self.uc, target + 4)?)))
        })();
        self.state.heap.free_ptr(target)?;
        let resolved = resolved?;
        if tick != read_mem_u32(&self.api, self.uc, self.world + 0x60)?
            || queue != read_mem_u32(&self.api, self.uc, self.commands + 0x14)? {
            return Err("native target resolution changed tick or command queue".into());
        }
        Ok(match resolved {
            Some((x, y)) => json!({"valid":true,"x":x,"y":y}),
            None => json!({"valid":false,"rejection":"no_resolved_target"}),
        })
    }

    /// Watched matches submit and report the resolved deployment coordinate.
    /// A successful raw validator result does not imply that a troop lands at
    /// the requested point. Recording that point would corrupt policy history.
    pub fn play_card_resolved(&mut self, request: PlayCardRequest) -> Result<Value, String> {
        let target = self.resolve_target(request)?;
        if target["valid"] != true {
            return Ok(json!({"accepted":false,"rejection":target["rejection"]}));
        }
        let effective = PlayCardRequest {
            x: target["x"].as_i64().ok_or("resolved target missing x")? as i32,
            y: target["y"].as_i64().ok_or("resolved target missing y")? as i32,
            ..request
        };
        let mut result = self.play_card(effective)?;
        result["resolved_target"] = json!({"x":effective.x,"y":effective.y});
        Ok(result)
    }

    /// Check an exact human cursor position through the original validator.
    /// No command is submitted and no world time is advanced.
    pub fn validate_target(&mut self, request: PlayCardRequest) -> Result<Value, String> {
        if request.side > 1 || request.slot > 3 {
            return Err("target check needs side 0..1 and slot 0..3".into());
        }
        let side_obj = self.side_object(request.side)?;
        if side_obj == 0 { return Err("null native battle side".into()); }
        let tick = read_mem_u32(&self.api, self.uc, self.world + 0x60)?;
        let queue = read_mem_u32(&self.api, self.uc, self.commands + 0x14)?;
        let wrapper = self.call(0xf9dd98, &[side_obj, request.slot as u64], 0)?;
        if wrapper == 0 { return Ok(json!({"valid":false,"affordable":false,"rejection":"empty_hand_slot"})); }
        write_mem(&self.api, self.uc, self.descriptor_scratch, &[0; 24])?;
        self.call(0xe80bbc, &[wrapper, side_obj, 1], self.descriptor_scratch)?;
        let proxy = self.call(0xe80680, &[self.descriptor_scratch], 0)?;
        if proxy == 0 { return Err("null native card selection proxy".into()); }
        let code = self.call(0xd42214, &[self.battle, request.x as i64 as u64,
            request.y as i64 as u64, proxy, 0], 0)? as u32;
        let cost = self.call(0xe80638, &[self.descriptor_scratch], 0)? as u32;
        let mana = self.call(0xfa1c84, &[side_obj, 1], 0)? as u32;
        if tick != read_mem_u32(&self.api, self.uc, self.world + 0x60)?
            || queue != read_mem_u32(&self.api, self.uc, self.commands + 0x14)? {
            return Err("native target check changed tick or command queue".into());
        }
        Ok(json!({"valid":code==0,"affordable":mana>=cost,"target_validity_code":code,"cost":cost,"mana":mana}))
    }

    /// Submit one card using the native hand descriptor, validator and queue.
    pub fn play_card(&mut self, request: PlayCardRequest) -> Result<Value, String> {
        if request.side > 1 {
            return Err("side must be 0 or 1".into());
        }
        if request.slot > 3 {
            return Err("hand slot must be in 0..=3".into());
        }
        let side_obj = self.side_object(request.side)?;
        if side_obj == 0 {
            return Err(format!("native battle side {} is null", request.side));
        }
        let actor = self.call(0xd3c980, &[self.battle, request.side as u64], 0)?;
        if actor == 0 {
            return Err(format!("native actor for side {} is null", request.side));
        }
        let wrapper = self.call(0xf9dd98, &[side_obj, request.slot as u64], 0)?;
        if wrapper == 0 {
            return Ok(
                json!({"accepted":false,"rejection":"empty_hand_slot","side":request.side,"hand_slot":request.slot}),
            );
        }
        write_mem(&self.api, self.uc, self.descriptor_scratch, &[0; 24])?;
        self.call(0xe80bbc, &[wrapper, side_obj, 1], self.descriptor_scratch)?;
        let proxy = self.call(0xe80680, &[self.descriptor_scratch], 0)?;
        if proxy == 0 {
            return Err("native card descriptor produced a null selection proxy".into());
        }
        let validity = self.call(
            0xd42214,
            &[
                self.battle,
                request.x as i64 as u64,
                request.y as i64 as u64,
                proxy,
                0,
            ],
            0,
        )? as u32;
        let mana = self.call(0xfa1c84, &[side_obj, 1], 0)? as u32;
        let cost = self.call(0xe80638, &[self.descriptor_scratch], 0)? as u32;
        let queue_before = u32::from_le_bytes(
            read_mem(&self.api, self.uc, self.commands + 0x14, 4)?
                .try_into()
                .unwrap(),
        );
        let rejection = if validity != 0 {
            Some(format!("invalid_target_code_{validity}"))
        } else if mana < cost {
            Some("insufficient_mana".to_string())
        } else {
            None
        };
        if let Some(reason) = rejection {
            return Ok(
                json!({"accepted":false,"rejection":reason,"target_validity_code":validity,"mana":mana,"cost":cost,"queue_count_before":queue_before,"queue_count_after":queue_before}),
            );
        }
        let command = self.state.heap.alloc(0x58, 16, &self.api, self.uc)?;
        let context = u64::from_le_bytes(
            read_mem(&self.api, self.uc, self.harness, 8)?
                .try_into()
                .unwrap(),
        );
        self.call(0xd774ac, &[command, context], 0)?;
        self.call(0xd77c8c, &[command, self.descriptor_scratch], 0)?;
        self.call(
            0xd77a14,
            &[command, request.x as i64 as u64, request.y as i64 as u64],
            0,
        )?;
        // Native caller supplies a temporary pointer slot; the helper does not retain it.
        let slot = self.state.heap.alloc(8, 8, &self.api, self.uc)?;
        write_mem(&self.api, self.uc, slot, &command.to_le_bytes())?;
        self.call(0x11d680c, &[self.harness, slot, actor], 0)?;
        let slot_after =
            u64::from_le_bytes(read_mem(&self.api, self.uc, slot, 8)?.try_into().unwrap());
        let queue_after = u32::from_le_bytes(
            read_mem(&self.api, self.uc, self.commands + 0x14, 4)?
                .try_into()
                .unwrap(),
        );
        self.state.heap.free_ptr(slot)?;
        let command_live = self.state.heap.allocations.contains_key(&command);
        let accepted =
            queue_after == queue_before.wrapping_add(1) && slot_after == command && command_live;
        Ok(
            json!({"accepted":accepted,"rejection":if accepted {Value::Null} else {json!("original_submission_rejected")},"target_validity_code":validity,"mana":mana,"cost":cost,"queue_count_before":queue_before,"queue_count_after":queue_after,"command_allocation_live":command_live}),
        )
    }

    /// Read current authoritative native state, resolving hand rows afresh.
    pub fn observe(&mut self) -> Result<Value, String> {
        let ticks = self.ticks()?;
        let terminated = read_mem(&self.api, self.uc, self.battle + 0x1e0, 1)?[0] != 0;
        let winner_side = if terminated {
            let winner = i32::from_le_bytes(
                read_mem(&self.api, self.uc, self.battle + 0x1b8, 4)?
                    .try_into()
                    .unwrap(),
            );
            if !matches!(winner, -1 | 0 | 1) {
                return Err(format!(
                    "native terminal winner field has unsupported value {winner}"
                ));
            }
            json!(winner)
        } else {
            Value::Null
        };
        let entities = u32::from_le_bytes(
            read_mem(&self.api, self.uc, self.entities + 0x14, 4)?
                .try_into()
                .unwrap(),
        );
        let width = self.call(0xd358a0, &[self.battle], 0)? as u32;
        let height = self.call(0xd358a8, &[self.battle], 0)? as u32;
        let mut sides = Vec::new();
        let mut elixir = Vec::new();
        let mut hands = Vec::new();
        for side in 0..2u32 {
            let side_obj = self.side_object(side)?;
            if side_obj == 0 {
                return Err(format!(
                    "native battle side {side} is null during observation"
                ));
            }
            let mana = self.call(0xfa1c84, &[side_obj, 1], 0)? as u32;
            elixir.push(mana);
            let mut hand = Vec::with_capacity(4);
            for slot in 0..4u32 {
                let wrapper = self.call(0xf9dd98, &[side_obj, slot as u64], 0)?;
                if wrapper == 0 {
                    hand.push(json!({"slot":slot,"card_id":Value::Null,"name":Value::Null,"cost":Value::Null,"available":false}));
                    continue;
                }
                write_mem(&self.api, self.uc, self.descriptor_scratch, &[0; 24])?;
                self.call(0xe80bbc, &[wrapper, side_obj, 1], self.descriptor_scratch)?;
                let row = self.call(0xe80680, &[self.descriptor_scratch], 0)?;
                if row == 0 {
                    hand.push(json!({"slot":slot,"card_id":Value::Null,"name":Value::Null,"cost":Value::Null,"available":false}));
                    continue;
                }
                let global_id = u32::from_le_bytes(
                    read_mem(&self.api, self.uc, row + 0x40, 4)?
                        .try_into()
                        .unwrap(),
                );
                let name = self.native_string(row + 0x28)?;
                let cost = self.call(0xe80638, &[self.descriptor_scratch], 0)? as u32;
                hand.push(json!({"slot":slot,"card_id":global_id,"name":name,"cost":cost,"available":true}));
            }
            hands.push(hand);
            sides.push(json!({"side":side,"elixir":mana}));
        }
        Ok(
            json!({"tick":ticks[0],"ticks":ticks,"terminated":terminated,"winner_side":winner_side,"entity_count":entities,"elixir":elixir,"hands":hands,"arena_cells":[width,height],"sides":sides}),
        )
    }

    /// Run the original native harness (including its original controller logic) to completion.
    pub fn native_match_until_terminal(&mut self) -> Result<Value, String> {
        let start = Instant::now();
        let before = self.ticks()?;
        let sret = self.state.heap.alloc(8, 8, &self.api, self.uc)?;
        write_mem(&self.api, self.uc, sret, &[0; 8])?;
        self.call(0x11d6364, &[self.harness], sret)?;
        let event = u64::from_le_bytes(read_mem(&self.api, self.uc, sret, 8)?.try_into().unwrap());
        self.state.heap.free_ptr(sret)?;
        if event == 0 {
            return Err("original until-terminal harness returned a null event".into());
        }
        let event_code =
            u32::from_le_bytes(read_mem(&self.api, self.uc, event, 4)?.try_into().unwrap());
        let after = self.ticks()?;
        if !self.is_terminal()? {
            return Err("original until-terminal harness returned before native completion".into());
        }
        let observation = self.observe()?;
        Ok(
            json!({"ticks_before":before,"ticks_after":after,"world_steps":after[0].wrapping_sub(before[0]),"event":format!("{event:#x}"),"event_code":event_code,"terminated":true,"winner_side":observation["winner_side"],"elapsed_seconds":start.elapsed().as_secs_f64()}),
        )
    }

    pub fn checkpoint_sha256(&self) -> &str {
        &self.initial_checkpoint_sha256
    }
}

impl Drop for NativeSession {
    fn drop(&mut self) {
        if !self.uc.is_null() {
            unsafe {
                (self.api.close)(self.uc);
            }
            self.uc = std::ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod persistent_session_tests {
    use super::NativeSession;
    use super::PlayCardRequest;
    use std::path::PathBuf;

    #[test]
    fn observes_native_state_and_advances_a_tick() {
        let checkpoint = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../analysis/native-pipeline/oracle-export-01/stage01-attempt001/state");
        let mut session = NativeSession::load(&checkpoint).expect("load native checkpoint");
        let before = session.observe().expect("native observation");
        assert_eq!(before["winner_side"], serde_json::Value::Null);
        assert_eq!(before["arena_cells"][0].as_u64(), Some(36));
        assert_eq!(before["arena_cells"][1].as_u64(), Some(64));
        assert_eq!(before["hands"].as_array().map(Vec::len), Some(2));
        assert_eq!(before["hands"][0].as_array().map(Vec::len), Some(4));
        assert_eq!(before["hands"][0][0]["card_id"].as_u64(), Some(27_000_002));
        assert_eq!(before["hands"][0][0]["name"].as_str(), Some("Mortar"));
        assert_eq!(before["hands"][0][0]["cost"].as_u64(), Some(4));
        let tick = before["tick"].as_u64().expect("u32 tick");
        session.world_step().expect("original world update");
        let after = session.observe().expect("native observation after step");
        assert_eq!(after["tick"].as_u64(), Some(tick + 1));
        for _ in 0..99 {
            session
                .world_step()
                .expect("advance to prepared input tick");
        }
        let submitted = session
            .play_card(PlayCardRequest {
                side: 0,
                slot: 0,
                x: 4500,
                y: 4500,
            })
            .expect("submit native card");
        assert_eq!(submitted["accepted"].as_bool(), Some(true));
        for _ in 0..100 {
            session.world_step().expect("consume native command");
        }
        let final_observation = session.observe().expect("state after card use");
        assert_eq!(final_observation["entity_count"].as_u64(), Some(7));
        assert_eq!(final_observation["elixir"][0].as_u64(), Some(5));
    }

    #[test]
    #[ignore = "runs one complete original bot-vs-bot match"]
    fn original_harness_match_reaches_terminal() {
        let checkpoint = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../analysis/native-pipeline/oracle-export-01/stage01-attempt001/state");
        let mut session = NativeSession::load(&checkpoint).expect("load native checkpoint");
        let report = session
            .native_match_until_terminal()
            .expect("complete original harness match");
        assert_eq!(report["terminated"].as_bool(), Some(true));
        assert!(report["world_steps"].as_u64().unwrap_or(0) > 0);
        let winner = report["winner_side"].as_i64().expect("signed winner side");
        eprintln!("original harness completed with winner_side={winner}");
        assert!((-1..=1).contains(&winner));
    }
}

#[cfg(test)]
mod file_identity_tests {
    use super::{identities_match, identity_of};
    use std::fs;

    #[test]
    fn file_identity_matches_until_the_file_changes() {
        let dir = std::env::temp_dir().join(format!(
            "native-watch-identity-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("region.bin");
        fs::write(&path, b"abc").unwrap();
        let saved = vec![identity_of(&path).unwrap()];
        identities_match(&saved).expect("unchanged file");
        fs::write(&path, b"abcd").unwrap();
        let error = identities_match(&saved).expect_err("replaced file");
        assert!(
            error.contains("identity changed"),
            "unexpected identity error: {error}"
        );
        let _ = fs::remove_dir_all(&dir);
    }
}
