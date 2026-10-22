//! Fixed guest-address memory backed by private host mappings.
//!
//! The checkpoint files are treated as immutable baselines. Writes create
//! private copy-on-write pages, and [`Memory::reset`] drops those private pages
//! so the file baseline and zero-filled anonymous tails are restored.

use serde_json::Value;
use std::ffi::{c_int, c_void};
use std::fs::File;
use std::io;
use std::os::fd::AsRawFd;
use std::path::Path;

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
struct Mapping {
    address: u64,
    size: usize,
    /// The segments are contiguous and cover the complete mapping. File-backed
    /// segments are private mappings; anonymous segments reset to zero.
    segments: Vec<(u64, usize)>,
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
                for &(address, size) in &mapping.segments {
                    let result = unsafe { madvise(address as *mut c_void, size, MADV_DONTNEED) };
                    if result != 0 {
                        return Err(format!(
                            "madvise(MADV_DONTNEED) at {address:#x}+{size:#x}: {}",
                            io::Error::last_os_error()
                        ));
                    }
                }
            }
            Ok(())
        }
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
                segments: vec![(address, size)],
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

        let mut segments = vec![(address, file_size)];
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
            segments.push((tail_address, tail_size));
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
}
