//! In-memory asset files for the wasm Painter.
//!
//! Keys are bundle-relative paths from `engine/analysis/native-spectator/`,
//! such as `assets.json`, `display-trees/chr_hog_rider.json` and
//! `atlases/chr_hog_rider_0.png`. Bytes are copied in; the caller may free
//! its buffer after `put`.

use std::cell::RefCell;
use std::collections::BTreeMap;

thread_local! {
    static FILES: RefCell<BTreeMap<String, Vec<u8>>> = const { RefCell::new(BTreeMap::new()) };
}

/// Normalize a bundle-relative path. Rejects empty keys and `..`.
pub fn normalize(path: &str) -> Result<String, String> {
    let path = path.trim().replace('\\', "/");
    let path = path.trim_start_matches("./");
    let path = path.trim_start_matches('/');
    if path.is_empty() {
        return Err("vfs path is empty".into());
    }
    let mut parts = Vec::new();
    for part in path.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            return Err(format!("vfs path escapes the bundle: {path}"));
        }
        parts.push(part);
    }
    if parts.is_empty() {
        return Err("vfs path is empty".into());
    }
    Ok(parts.join("/"))
}

pub fn put(path: &str, data: &[u8]) -> Result<(), String> {
    let key = normalize(path)?;
    FILES.with(|files| {
        files.borrow_mut().insert(key, data.to_vec());
    });
    Ok(())
}

pub fn read(path: &str) -> Result<Vec<u8>, String> {
    let key = normalize(path)?;
    FILES.with(|files| {
        files
            .borrow()
            .get(&key)
            .cloned()
            .ok_or_else(|| format!("vfs missing {key}"))
    })
}

/// Remove a file and return its bytes. Parsing and atlas decode use this so
/// the compressed copy does not stay beside the decoded one.
pub fn take(path: &str) -> Result<Vec<u8>, String> {
    let key = normalize(path)?;
    FILES.with(|files| {
        files
            .borrow_mut()
            .remove(&key)
            .ok_or_else(|| format!("vfs missing {key}"))
    })
}

/// Bytes still stored in the VFS. Decoded images are not included.
pub fn resident_bytes() -> usize {
    FILES.with(|files| files.borrow().values().map(|bytes| bytes.len()).sum())
}

/// Paths still stored, largest first. Decoded images are not included.
pub fn resident_files() -> Vec<(String, usize)> {
    FILES.with(|files| {
        let mut rows: Vec<_> = files
            .borrow()
            .iter()
            .map(|(path, bytes)| (path.clone(), bytes.len()))
            .collect();
        rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        rows
    })
}

pub fn contains(path: &str) -> bool {
    normalize(path)
        .ok()
        .is_some_and(|key| FILES.with(|files| files.borrow().contains_key(&key)))
}

pub fn clear() {
    FILES.with(|files| files.borrow_mut().clear());
}

pub fn len() -> usize {
    FILES.with(|files| files.borrow().len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_copies_bytes_and_rejects_parent_segments() {
        clear();
        put("display-trees/chr_hog_rider.json", b"{\"a\":1}").unwrap();
        assert_eq!(read("./display-trees/chr_hog_rider.json").unwrap(), b"{\"a\":1}");
        assert!(put("../assets.json", b"no").is_err());
        assert!(read("missing.json").is_err());
        assert_eq!(resident_bytes(), b"{\"a\":1}".len());
        assert_eq!(take("display-trees/chr_hog_rider.json").unwrap(), b"{\"a\":1}");
        assert_eq!(resident_bytes(), 0);
        assert!(take("display-trees/chr_hog_rider.json").is_err());
        clear();
        assert_eq!(len(), 0);
    }
}
