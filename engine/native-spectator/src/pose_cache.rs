//! Disk cache of fitted original sprites.
//!
//! A pose is identified by its document, export, frame, playback variant and
//! draw scale. The first time it is composited, the PNG and its anchor offset
//! are stored. Later matches and later fights load that file instead of
//! rendering the SC clip again.

#[cfg(not(feature = "wasm"))]
use image::codecs::png::PngDecoder;
#[cfg(not(feature = "wasm"))]
use image::{DynamicImage, ImageFormat};
use image::RgbaImage;
#[cfg(not(feature = "wasm"))]
use std::fs;
#[cfg(not(feature = "wasm"))]
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct PoseCache {
    #[cfg_attr(feature = "wasm", allow(dead_code))]
    root: PathBuf,
}

#[cfg(feature = "wasm")]
impl PoseCache {
    pub fn open(_root: &Path) -> Result<Self, String> {
        Err("disk pose cache is disabled under wasm".into())
    }

    pub fn load(&self, _key: &str) -> Option<(RgbaImage, f64, f64)> {
        None
    }

    pub fn store(&self, _key: &str, _image: &RgbaImage, _dx: f64, _dy: f64) {}
}

#[cfg(not(feature = "wasm"))]
impl PoseCache {
    pub fn open(root: &Path) -> Result<Self, String> {
        fs::create_dir_all(root).map_err(|e| format!("create pose cache: {e}"))?;
        Ok(Self {
            root: root.to_owned(),
        })
    }

    pub fn load(&self, key: &str) -> Option<(RgbaImage, f64, f64)> {
        let png = self.path(key);
        let xy = fs::read(png.with_extension("xy")).ok()?;
        if xy.len() != 16 {
            return None;
        }
        let dx = f64::from_le_bytes(xy[0..8].try_into().ok()?);
        let dy = f64::from_le_bytes(xy[8..16].try_into().ok()?);
        let file = fs::File::open(&png).ok()?;
        let decoder = PngDecoder::new(BufReader::new(file)).ok()?;
        let image = DynamicImage::from_decoder(decoder).ok()?.into_rgba8();
        if image.width() == 0
            || image.height() == 0
            || image.width() > 4096
            || image.height() > 4096
        {
            return None;
        }
        Some((image, dx, dy))
    }

    pub fn store(&self, key: &str, image: &RgbaImage, dx: f64, dy: f64) {
        if image.width() == 0 || image.height() == 0 {
            return;
        }
        let png = self.path(key);
        let Some(dir) = png.parent() else {
            return;
        };
        if fs::create_dir_all(dir).is_err() {
            return;
        }
        let temp = dir.join(format!(".{key}.{}.tmp", std::process::id()));
        if image.save_with_format(&temp, ImageFormat::Png).is_err() {
            let _ = fs::remove_file(&temp);
            return;
        }
        if fs::rename(&temp, &png).is_err() {
            let _ = fs::remove_file(&temp);
            return;
        }
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&dx.to_le_bytes());
        bytes[8..].copy_from_slice(&dy.to_le_bytes());
        let _ = fs::write(png.with_extension("xy"), bytes);
    }

    fn path(&self, key: &str) -> PathBuf {
        let shard = key.get(..2).unwrap_or("00");
        self.root.join(shard).join(format!("{key}.png"))
    }
}

/// Stable identity for one fitted pose. Not a security hash.
pub fn key(parts: &[&str], nums: &[u64]) -> String {
    fn mix(seed: u64, parts: &[&str], nums: &[u64]) -> u64 {
        let mut hash = seed;
        for part in parts {
            for byte in part.as_bytes() {
                hash ^= u64::from(*byte);
                hash = hash.wrapping_mul(0x100000001b3);
            }
            hash ^= 0xff;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        for number in nums {
            for byte in number.to_le_bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
        hash
    }
    format!(
        "{:016x}{:016x}",
        mix(0xcbf29ce484222325, parts, nums),
        mix(0x84222325cbf29ce4, parts, nums)
    )
}

/// Drop older entries down to three quarters of `limit` instead of emptying
/// the map. A full wipe re-renders every visible troop on the next frame.
pub fn trim_map<K: Clone + Ord, V>(map: &mut std::collections::BTreeMap<K, V>, limit: usize) {
    if map.len() <= limit {
        return;
    }
    let keep = limit.saturating_mul(3) / 4;
    let remove = map.len().saturating_sub(keep);
    let keys: Vec<K> = map.keys().take(remove).cloned().collect();
    for key in keys {
        map.remove(&key);
    }
}

#[cfg(all(test, not(feature = "wasm")))]
mod tests {
    use super::*;
    use image::Rgba;

    #[test]
    fn a_stored_pose_loads_with_its_anchor() {
        let root = std::env::temp_dir().join(format!("pose-cache-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let cache = PoseCache::open(&root).unwrap();
        let mut image = RgbaImage::new(2, 1);
        image.put_pixel(0, 0, Rgba([1, 2, 3, 4]));
        let id = key(&["chr_pekka", "pekka_run"], &[3, 9, 1]);
        cache.store(&id, &image, -4.0, 7.0);
        let (loaded, dx, dy) = cache.load(&id).expect("cached pose");
        assert_eq!(loaded.dimensions(), (2, 1));
        assert_eq!(loaded.get_pixel(0, 0).0, [1, 2, 3, 4]);
        assert_eq!((dx, dy), (-4., 7.));
        let _ = fs::remove_dir_all(&root);
    }
}
