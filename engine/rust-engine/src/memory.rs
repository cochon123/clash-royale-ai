//! Fixed guest-address memory backed by private host mappings.
//!
//! The checkpoint files are treated as immutable baselines. Writes create
//! private copy-on-write pages, and [`Memory::reset`] drops those private pages
//! so the file baseline and zero-filled anonymous tails are restored.

use serde_json::Value;
use std::collections::HashSet;
use std::ffi::{c_int, c_void};
use std::fs::File;
use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::sync::Arc;

#[cfg(target_os = "linux")]
const PROT_READ: c_int = 0x1;
#[cfg(target_os = "linux")]
const PROT_WRITE: c_int = 0x2;
#[cfg(target_os = "linux")]
const MAP_PRIVATE: c_int = 0x02;
#[cfg(target_os = "linux")]
const MAP_FIXED: c_int = 0x10;
#[cfg(target_os = "linux")]
const MAP_ANONYMOUS: c_int = 0x20;
#[cfg(target_os = "linux")]
const MAP_FIXED_NOREPLACE: c_int = 0x100000;
#[cfg(target_os = "linux")]
const MADV_DONTNEED: c_int = 4;
/// `/proc/self/pagemap` bit 56: the PTE is exclusively mapped.
#[cfg(target_os = "linux")]
const PM_EXCLUSIVE: u64 = 1 << 56;
/// `/proc/self/pagemap` bit 61: the PTE still names a file or shared page.
#[cfg(target_os = "linux")]
const PM_FILE: u64 = 1 << 61;
/// `/proc/self/pagemap` bit 62: the PTE is swapped.
#[cfg(target_os = "linux")]
const PM_SWAP: u64 = 1 << 62;
/// `/proc/self/pagemap` bit 63: the PTE is present.
#[cfg(target_os = "linux")]
const PM_PRESENT: u64 = 1 << 63;

#[cfg(target_os = "linux")]
unsafe extern "C" {
    fn mmap(
        address: *mut c_void,
        length: usize,
        protection: c_int,
        flags: c_int,
        fd: c_int,
        offset: i64,
    ) -> *mut c_void;
    fn munmap(address: *mut c_void, length: usize) -> c_int;
    fn madvise(address: *mut c_void, length: usize, advice: c_int) -> c_int;
}

#[derive(Debug)]
enum Baseline {
    /// Anonymous mapping; reset restores zeros.
    Zero,
    /// Private file mapping; reset restores these checkpoint bytes.
    File { file: Arc<File>, offset: u64 },
}

#[derive(Debug)]
struct Segment {
    address: u64,
    size: usize,
    baseline: Baseline,
}

#[derive(Debug)]
struct Mapping {
    address: u64,
    size: usize,
    /// The segments are contiguous and cover the complete mapping. File-backed
    /// segments are private mappings; anonymous segments reset to zero.
    segments: Vec<Segment>,
}

/// Dirty guest pages relative to the reset checkpoint, plus every reserved
/// mapping so a fresh process can recreate extra arenas before writing them.
#[derive(Clone, Debug)]
pub struct DirtyPages {
    pub pages: Vec<(u64, Vec<u8>)>,
    pub mappings: Vec<(u64, usize)>,
}

impl DirtyPages {
    pub fn payload_bytes(&self) -> u64 {
        self.pages.iter().map(|(_, page)| page.len() as u64).sum()
    }
}

/// A process-local view of fixed-address guest memory.
pub struct Memory {
    mappings: Vec<Mapping>,
}

impl Memory {
    /// Map all checkpoint regions at their recorded guest addresses.
    ///
    /// The caller must verify checkpoint and region hashes before calling this
    /// function. File descriptors are closed as soon as their mappings exist.
    pub fn load(checkpoint_dir: &Path, cp: &Value) -> Result<Self, String> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (checkpoint_dir, cp);
            return Err("fixed guest memory mappings require Linux".into());
        }

        #[cfg(target_os = "linux")]
        {
            let regions = cp["regions"]
                .as_array()
                .ok_or("checkpoint regions missing or not an array")?;
            let mut memory = Self {
                mappings: Vec::with_capacity(regions.len()),
            };
            for region in regions {
                let file = region["file"]
                    .as_str()
                    .ok_or("checkpoint region file missing")?;
                let address = json_u64(&region["base"])?;
                let file_size = usize::try_from(json_u64(&region["size"])?)
                    .map_err(|_| format!("region size too large: {file}"))?;
                let capacity = region_capacity(cp, file, address, file_size)?;
                memory.map_private_file(
                    checkpoint_dir.join(file).as_path(),
                    address,
                    file_size,
                    capacity,
                )?;
            }
            if let Some(extras) = cp["native_heap"]["extra_heaps"].as_array() {
                for extra in extras {
                    let address = json_u64(&extra["base"])?;
                    let capacity = usize::try_from(json_u64(&extra["capacity"])?)
                        .map_err(|_| "extra heap capacity too large")?;
                    if !memory.contains(address, capacity) {
                        memory.map_zeroed(address, capacity)?;
                    }
                }
            }
            Ok(memory)
        }
    }

    /// Drop all private modifications and restore anonymous tails to zero.
    pub fn reset(&mut self) -> Result<(), String> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err("fixed guest memory mappings require Linux".into());
        }

        #[cfg(target_os = "linux")]
        {
            for mapping in &self.mappings {
                for segment in &mapping.segments {
                    let result = unsafe {
                        madvise(segment.address as *mut c_void, segment.size, MADV_DONTNEED)
                    };
                    if result != 0 {
                        return Err(format!(
                            "madvise(MADV_DONTNEED) at {:#x}+{:#x}: {}",
                            segment.address,
                            segment.size,
                            io::Error::last_os_error()
                        ));
                    }
                }
            }
            Ok(())
        }
    }

    /// Pages that differ from the reset checkpoint, and the reserved ranges.
    ///
    /// File-backed pages are compared with the checkpoint file. Anonymous
    /// pages are compared with zeros. A page written back to that baseline is
    /// omitted, so restore reverts every other private page and then writes
    /// these bytes. Only privately modified PTEs are read: a clean file page
    /// and an untouched anonymous page still match the baseline.
    pub fn capture_dirty(&self) -> Result<DirtyPages, String> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err("fixed guest memory mappings require Linux".into());
        }
        #[cfg(target_os = "linux")]
        {
            let page = page_size();
            let pagemap = PageMap::open().ok();
            let mut pages = Vec::new();
            for mapping in &self.mappings {
                for segment in &mapping.segments {
                    if segment.size % page != 0 {
                        return Err(format!(
                            "segment {:#x} is not page aligned in length",
                            segment.address
                        ));
                    }
                    let found = match &pagemap {
                        Some(map) => dirty_segment_pagemap(segment, map)?,
                        None => dirty_segment_scan(segment)?,
                    };
                    pages.extend(found);
                }
            }
            Ok(DirtyPages {
                pages,
                mappings: self
                    .mappings
                    .iter()
                    .map(|mapping| (mapping.address, mapping.size))
                    .collect(),
            })
        }
    }

    /// Restore the checkpoint baseline, ensure every snapshotted range is
    /// reserved, then write the dirty pages back.
    ///
    /// Private pages that are not part of the diff are discarded individually.
    /// A full-range `MADV_DONTNEED` is the fallback when pagemap cannot be read.
    pub fn apply_dirty(&mut self, dirty: &DirtyPages) -> Result<(), String> {
        for &(address, size) in &dirty.mappings {
            if !self
                .mappings
                .iter()
                .any(|mapping| mapping.address == address && mapping.size == size)
            {
                self.map_zeroed(address, size)?;
            }
        }
        let keep = dirty.pages.iter().map(|(address, _)| *address).collect();
        if self.revert_private_except(&keep).is_err() {
            self.reset()?;
        }
        for (address, bytes) in &dirty.pages {
            self.write_bytes(*address, bytes)?;
        }
        Ok(())
    }

    /// Drop private pages whose addresses are not in `keep`.
    #[cfg(target_os = "linux")]
    fn revert_private_except(&mut self, keep: &HashSet<u64>) -> Result<(), String> {
        let page = page_size();
        let map = PageMap::open()?;
        for mapping in &self.mappings {
            for segment in &mapping.segments {
                let count = segment.size / page;
                let entries = map.read(segment.address, count)?;
                let mut run_start: Option<u64> = None;
                let mut run_pages = 0usize;
                let flush = |start: &mut Option<u64>, pages: &mut usize| -> Result<(), String> {
                    if let Some(address) = start.take() {
                        advise_dontneed(address, *pages * page)?;
                        *pages = 0;
                    }
                    Ok(())
                };
                for (index, entry) in entries.iter().enumerate() {
                    let address = segment.address + (index * page) as u64;
                    let discard = may_differ(*entry) && !keep.contains(&address);
                    if discard {
                        match run_start {
                            Some(start) if start + (run_pages as u64) * page as u64 == address => {
                                run_pages += 1;
                            }
                            Some(_) => {
                                flush(&mut run_start, &mut run_pages)?;
                                run_start = Some(address);
                                run_pages = 1;
                            }
                            None => {
                                run_start = Some(address);
                                run_pages = 1;
                            }
                        }
                    } else if run_start.is_some() {
                        flush(&mut run_start, &mut run_pages)?;
                    }
                }
                flush(&mut run_start, &mut run_pages)?;
            }
        }
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    fn revert_private_except(&mut self, _keep: &HashSet<u64>) -> Result<(), String> {
        Err("fixed guest memory mappings require Linux".into())
    }

    /// Map a zero-filled guest region, refusing to replace any existing host
    /// mapping. Both address and size must be page aligned.
    pub fn map_zeroed(&mut self, address: u64, size: usize) -> Result<(), String> {
        // An in-place episode reset retains owned mappings while restoring
        // allocator metadata. Reuse that exact reserved arena on re-expansion.
        if self
            .mappings
            .iter()
            .any(|m| m.address == address && m.size == size)
        {
            return Ok(());
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = (address, size);
            return Err("fixed guest memory mappings require Linux".into());
        }

        #[cfg(target_os = "linux")]
        {
            check_page_range(address, size)?;
            let result = unsafe {
                mmap(
                    address as *mut c_void,
                    size,
                    PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED_NOREPLACE,
                    -1,
                    0,
                )
            };
            if result as isize == -1 {
                return Err(format!(
                    "mmap zeroed guest range {address:#x}+{size:#x}: {}",
                    io::Error::last_os_error()
                ));
            }
            if result as u64 != address {
                unsafe { munmap(result, size) };
                return Err(format!(
                    "mmap returned unexpected address {:#x} for {address:#x}",
                    result as u64
                ));
            }
            self.mappings.push(Mapping {
                address,
                size,
                segments: vec![Segment {
                    address,
                    size,
                    baseline: Baseline::Zero,
                }],
            });
            Ok(())
        }
    }

    /// Read a guest value without requiring natural alignment.
    ///
    /// # Safety
    /// `address..address+size_of::<T>()` must be mapped and readable, and the
    /// bytes must be a valid bit representation of `T`.
    pub unsafe fn read<T: Copy>(&self, address: u64) -> T {
        #[cfg(any(debug_assertions, feature = "checked-memory"))]
        assert!(
            self.contains(address, std::mem::size_of::<T>()),
            "unmapped read at {address:#x}"
        );
        std::ptr::read_unaligned(address as *const T)
    }

    /// Write a guest value without requiring natural alignment.
    ///
    /// # Safety
    /// `address..address+size_of::<T>()` must be mapped and writable.
    pub unsafe fn write<T: Copy>(&mut self, address: u64, value: T) {
        #[cfg(any(debug_assertions, feature = "checked-memory"))]
        assert!(
            self.contains(address, std::mem::size_of::<T>()),
            "unmapped write at {address:#x}"
        );
        std::ptr::write_unaligned(address as *mut T, value);
    }

    /// Copy bytes from a single mapped guest region.
    pub fn read_bytes(&self, address: u64, len: usize) -> Result<Vec<u8>, String> {
        self.check_range(address, len)?;
        let mut bytes = vec![0; len];
        if len != 0 {
            unsafe { std::ptr::copy_nonoverlapping(address as *const u8, bytes.as_mut_ptr(), len) };
        }
        Ok(bytes)
    }

    /// Copy bytes into a single mapped guest region.
    pub fn write_bytes(&mut self, address: u64, bytes: &[u8]) -> Result<(), String> {
        self.check_range(address, bytes.len())?;
        if !bytes.is_empty() {
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), address as *mut u8, bytes.len())
            };
        }
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn map_private_file(
        &mut self,
        path: &Path,
        address: u64,
        file_size: usize,
        capacity: usize,
    ) -> Result<(), String> {
        check_page_range(address, capacity)?;
        if file_size == 0 || file_size > capacity || file_size % page_size() != 0 {
            return Err(format!(
                "invalid checkpoint region length for {}: file={file_size:#x}, capacity={capacity:#x}",
                path.display()
            ));
        }
        let file = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let actual_len = usize::try_from(
            file.metadata()
                .map_err(|e| format!("{}: {e}", path.display()))?
                .len(),
        )
        .map_err(|_| format!("checkpoint region too large: {}", path.display()))?;
        if actual_len != file_size {
            return Err(format!(
                "checkpoint region size mismatch for {}: expected {file_size}, got {actual_len}",
                path.display()
            ));
        }

        // Reserve the complete range first. MAP_FIXED below only replaces this
        // reservation, never an unrelated host mapping.
        let reservation = unsafe {
            mmap(
                address as *mut c_void,
                capacity,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED_NOREPLACE,
                -1,
                0,
            )
        };
        if reservation as isize == -1 {
            return Err(format!(
                "reserve guest region {address:#x}+{capacity:#x}: {}",
                io::Error::last_os_error()
            ));
        }
        if reservation as u64 != address {
            unsafe { munmap(reservation, capacity) };
            return Err(format!(
                "mmap returned unexpected address {:#x} for {address:#x}",
                reservation as u64
            ));
        }

        let file_map = unsafe {
            mmap(
                address as *mut c_void,
                file_size,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_FIXED,
                file.as_raw_fd(),
                0,
            )
        };
        if file_map as isize == -1 {
            let error = io::Error::last_os_error();
            unsafe { munmap(address as *mut c_void, capacity) };
            return Err(format!(
                "map checkpoint {} at {address:#x}: {error}",
                path.display()
            ));
        }
        if file_map as u64 != address {
            unsafe { munmap(address as *mut c_void, capacity) };
            return Err(format!(
                "file mmap returned unexpected address {:#x} for {address:#x}",
                file_map as u64
            ));
        }

        let file = Arc::new(file);
        let mut segments = vec![Segment {
            address,
            size: file_size,
            baseline: Baseline::File {
                file: Arc::clone(&file),
                offset: 0,
            },
        }];
        if capacity > file_size {
            let tail_address = address + file_size as u64;
            let tail_size = capacity - file_size;
            let tail = unsafe {
                mmap(
                    tail_address as *mut c_void,
                    tail_size,
                    PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED,
                    -1,
                    0,
                )
            };
            if tail as isize == -1 || tail as u64 != tail_address {
                let error = if tail as isize == -1 {
                    io::Error::last_os_error().to_string()
                } else {
                    format!("unexpected address {:#x}", tail as u64)
                };
                unsafe { munmap(address as *mut c_void, capacity) };
                return Err(format!("map zero tail at {tail_address:#x}: {error}"));
            }
            segments.push(Segment {
                address: tail_address,
                size: tail_size,
                baseline: Baseline::Zero,
            });
        }
        self.mappings.push(Mapping {
            address,
            size: capacity,
            segments,
        });
        Ok(())
    }

    fn check_range(&self, address: u64, len: usize) -> Result<(), String> {
        if self.contains(address, len) {
            Ok(())
        } else {
            Err(format!("guest range {address:#x}+{len:#x} is not mapped"))
        }
    }

    fn contains(&self, address: u64, len: usize) -> bool {
        let Some(end) = address.checked_add(len as u64) else {
            return false;
        };
        self.mappings.iter().any(|mapping| {
            let start = mapping.address;
            let Some(mapping_end) = start.checked_add(mapping.size as u64) else {
                return false;
            };
            address >= start && end <= mapping_end
        })
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        #[cfg(target_os = "linux")]
        for mapping in self.mappings.drain(..) {
            unsafe { munmap(mapping.address as *mut c_void, mapping.size) };
        }
    }
}

#[cfg(target_os = "linux")]
struct PageMap {
    file: File,
}

#[cfg(target_os = "linux")]
impl PageMap {
    fn open() -> Result<Self, String> {
        File::open("/proc/self/pagemap")
            .map(|file| Self { file })
            .map_err(|error| format!("open /proc/self/pagemap: {error}"))
    }

    fn read(&self, address: u64, pages: usize) -> Result<Vec<u64>, String> {
        if pages == 0 {
            return Ok(Vec::new());
        }
        let mut bytes = vec![0u8; pages * 8];
        self.file
            .read_exact_at(&mut bytes, address / page_size() as u64 * 8)
            .map_err(|error| format!("read /proc/self/pagemap at {address:#x}: {error}"))?;
        Ok(bytes
            .chunks_exact(8)
            .map(|word| u64::from_le_bytes(word.try_into().expect("8-byte pagemap word")))
            .collect())
    }
}

/// A page can differ from the reset baseline only when it is a private
/// anonymous PTE or it has been swapped out. The shared zero page is present
/// but not exclusive. A clean file page keeps the file bit.
#[cfg(target_os = "linux")]
fn may_differ(entry: u64) -> bool {
    if entry & PM_SWAP != 0 {
        return true;
    }
    if entry & PM_PRESENT == 0 || entry & PM_FILE != 0 {
        return false;
    }
    entry & PM_EXCLUSIVE != 0
}

#[cfg(target_os = "linux")]
fn advise_dontneed(address: u64, length: usize) -> Result<(), String> {
    let result = unsafe { madvise(address as *mut c_void, length, MADV_DONTNEED) };
    if result != 0 {
        return Err(format!(
            "madvise(MADV_DONTNEED) at {address:#x}+{length:#x}: {}",
            io::Error::last_os_error()
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn dirty_segment_pagemap(segment: &Segment, map: &PageMap) -> Result<Vec<(u64, Vec<u8>)>, String> {
    let page = page_size();
    let entries = map.read(segment.address, segment.size / page)?;
    let mut out = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        if !may_differ(*entry) {
            continue;
        }
        let address = segment.address + (index * page) as u64;
        if let Some(bytes) = page_if_dirty(segment, address, index)? {
            out.push((address, bytes));
        }
    }
    Ok(out)
}

#[cfg(target_os = "linux")]
fn dirty_segment_scan(segment: &Segment) -> Result<Vec<(u64, Vec<u8>)>, String> {
    let page = page_size();
    let mut out = Vec::new();
    let mut index = 0usize;
    let mut cursor = 0usize;
    while cursor < segment.size {
        let address = segment.address + cursor as u64;
        if let Some(bytes) = page_if_dirty(segment, address, index)? {
            out.push((address, bytes));
        }
        cursor += page;
        index += 1;
    }
    Ok(out)
}

#[cfg(target_os = "linux")]
fn page_if_dirty(segment: &Segment, address: u64, index: usize) -> Result<Option<Vec<u8>>, String> {
    let page = page_size();
    let guest = unsafe { std::slice::from_raw_parts(address as *const u8, page) };
    let differs = match &segment.baseline {
        Baseline::Zero => guest.iter().any(|byte| *byte != 0),
        Baseline::File { file, offset } => {
            let mut baseline = vec![0u8; page];
            file.read_exact_at(&mut baseline, offset + (index * page) as u64)
                .map_err(|error| format!("checkpoint baseline read: {error}"))?;
            guest != baseline.as_slice()
        }
    };
    if differs {
        Ok(Some(guest.to_vec()))
    } else {
        Ok(None)
    }
}

fn json_u64(value: &Value) -> Result<u64, String> {
    if let Some(number) = value.as_u64() {
        Ok(number)
    } else if let Some(text) = value.as_str() {
        if let Some(hex) = text.strip_prefix("0x") {
            u64::from_str_radix(hex, 16).map_err(|e| format!("bad integer {text:?}: {e}"))
        } else {
            text.parse::<u64>()
                .map_err(|e| format!("bad integer {text:?}: {e}"))
        }
    } else {
        Err("expected integer or address string".into())
    }
}

fn region_capacity(
    cp: &Value,
    file: &str,
    address: u64,
    file_size: usize,
) -> Result<usize, String> {
    if file == "heap.bin" {
        return usize::try_from(json_u64(&cp["native_heap"]["capacity"])?)
            .map_err(|_| "heap capacity too large".into());
    }
    if let Some(extras) = cp["native_heap"]["extra_heaps"].as_array() {
        for extra in extras {
            if json_u64(&extra["base"])? == address {
                return usize::try_from(json_u64(&extra["capacity"])?)
                    .map_err(|_| "extra heap capacity too large".into());
            }
        }
    }
    Ok(file_size)
}

#[cfg(target_os = "linux")]
fn page_size() -> usize {
    4096
}

#[cfg(target_os = "linux")]
fn check_page_range(address: u64, size: usize) -> Result<(), String> {
    if size == 0 || address % page_size() as u64 != 0 || size % page_size() != 0 {
        return Err(format!(
            "guest mapping must be nonempty and page aligned: {address:#x}+{size:#x}"
        ));
    }
    address
        .checked_add(size as u64)
        .ok_or_else(|| "guest mapping address overflow".to_string())?;
    Ok(())
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::fs::OpenOptions;
    use std::io::Write;

    #[test]
    fn reset_restores_private_file_and_zero_tail() {
        let page = page_size();
        let path = std::env::temp_dir().join(format!(
            "clash-recompiled-memory-{}-{}.bin",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        let baseline = vec![0x5a; page];
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .expect("create temporary mapping source");
        file.write_all(&baseline).expect("write baseline mapping");
        drop(file);

        // This test runs in its own process, and NOREPLACE still guards against
        // any unexpected collision in that process.
        let address = 0x4f00_0000_0000_u64 + (u64::from(std::process::id()) << 12);
        let mut memory = Memory {
            mappings: Vec::new(),
        };
        memory
            .map_private_file(&path, address, page, page * 2)
            .expect("map private checkpoint plus anonymous tail");
        memory
            .write_bytes(address + 10, &[0xaa, 0xbb])
            .expect("modify private file page");
        memory
            .write_bytes(address + page as u64 + 10, &[0xcc, 0xdd])
            .expect("modify anonymous tail");

        memory.reset().expect("reset mapped pages");
        assert_eq!(memory.read_bytes(address + 10, 2).unwrap(), [0x5a, 0x5a]);
        assert_eq!(
            memory.read_bytes(address + page as u64 + 10, 2).unwrap(),
            [0, 0]
        );
        drop(memory);
        std::fs::remove_file(path).expect("remove temporary mapping source");
    }

    #[test]
    fn dirty_page_diff_round_trips_and_ignores_baseline() {
        let page = page_size();
        let path = std::env::temp_dir().join(format!(
            "clash-recompiled-dirty-{}-{}.bin",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let baseline = vec![0x11; page];
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .expect("create temporary mapping source");
        file.write_all(&baseline).expect("write baseline mapping");
        drop(file);

        let address = 0x4e00_0000_0000_u64 + (u64::from(std::process::id()) << 12);
        let mut memory = Memory {
            mappings: Vec::new(),
        };
        memory
            .map_private_file(&path, address, page, page * 2)
            .expect("map private checkpoint plus anonymous tail");
        memory
            .write_bytes(address + 4, &[0xab, 0xcd])
            .expect("dirty file page");
        memory
            .write_bytes(address + page as u64 + 8, &[0xef])
            .expect("dirty anonymous tail");
        let dirty = memory.capture_dirty().expect("capture dirty pages");
        assert_eq!(dirty.pages.len(), 2, "baseline bytes must not be retained");
        assert_eq!(dirty.payload_bytes(), (page * 2) as u64);

        memory
            .write_bytes(address + 4, &[0x00, 0x00])
            .expect("overwrite the snapshotted file page");
        memory
            .write_bytes(address + page as u64 + 8, &[0x00])
            .expect("overwrite the snapshotted tail");
        memory.apply_dirty(&dirty).expect("restore dirty pages");
        assert_eq!(memory.read_bytes(address + 4, 2).unwrap(), [0xab, 0xcd]);
        assert_eq!(
            memory.read_bytes(address + page as u64 + 8, 1).unwrap(),
            [0xef]
        );
        // A page returned to the checkpoint baseline is not part of the diff.
        memory
            .write_bytes(address + 4, &[0x11, 0x11])
            .expect("restore file baseline bytes");
        let cleaned = memory.capture_dirty().expect("recapture");
        assert_eq!(cleaned.pages.len(), 1);
        assert_eq!(cleaned.pages[0].0, address + page as u64);
        // The omitted file page is still a private PTE. Restoring the cleaned
        // diff must put it back on the checkpoint bytes.
        memory
            .write_bytes(address + 4, &[0x22, 0x33])
            .expect("dirty the baseline-matching page again");
        memory.apply_dirty(&cleaned).expect("restore cleaned diff");
        assert_eq!(memory.read_bytes(address + 4, 2).unwrap(), [0x11, 0x11]);
        assert_eq!(
            memory.read_bytes(address + page as u64 + 8, 1).unwrap(),
            [0xef]
        );
        drop(memory);
        std::fs::remove_file(path).expect("remove temporary mapping source");
    }

    #[test]
    fn pagemap_dirty_set_matches_full_scan() {
        let page = page_size();
        let path = std::env::temp_dir().join(format!(
            "clash-recompiled-pagemap-{}-{}.bin",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .expect("create temporary mapping source");
        file.write_all(&vec![0x42; page]).expect("write baseline mapping");
        drop(file);

        let address = 0x4d00_0000_0000_u64 + (u64::from(std::process::id()) << 12);
        let mut memory = Memory {
            mappings: Vec::new(),
        };
        memory
            .map_private_file(&path, address, page, page * 2)
            .expect("map private checkpoint plus anonymous tail");
        let _ = memory.read_bytes(address, 1).expect("fault the clean file page");
        let _ = memory
            .read_bytes(address + page as u64, 1)
            .expect("fault the shared zero page");
        {
            let map = super::PageMap::open().expect("pagemap");
            let clean_fast = super::dirty_segment_pagemap(&memory.mappings[0].segments[0], &map)
                .expect("pagemap file");
            let clean_scan =
                super::dirty_segment_scan(&memory.mappings[0].segments[0]).expect("scan file");
            assert!(clean_fast.is_empty());
            assert!(clean_scan.is_empty());
            assert!(
                super::dirty_segment_pagemap(&memory.mappings[0].segments[1], &map)
                    .expect("pagemap tail")
                    .is_empty()
            );
        }

        memory
            .write_bytes(address + 3, &[0x7e])
            .expect("dirty file page");
        memory
            .write_bytes(address + page as u64 + 6, &[0x9a])
            .expect("dirty anonymous page");
        let fast = memory.capture_dirty().expect("pagemap capture");
        // Re-scan after the capture so both paths see the same private pages.
        let scanned = super::dirty_segment_scan(&memory.mappings[0].segments[0])
            .expect("scan file")
            .into_iter()
            .chain(
                super::dirty_segment_scan(&memory.mappings[0].segments[1])
                    .expect("scan tail"),
            )
            .collect::<Vec<_>>();
        assert_eq!(fast.pages, scanned);
        drop(memory);
        std::fs::remove_file(path).expect("remove temporary mapping source");
    }
}
