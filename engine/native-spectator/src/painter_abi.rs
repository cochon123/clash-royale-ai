//! Raw C ABI for the browser painter. No wasm-bindgen.
//!
//! Strings are UTF-8 in linear memory. `0` is success. A negative
//! `painter_step` or a nonzero return from the other calls stores a message
//! for `painter_last_error`.

use crate::export_session::ExportSession;
use std::alloc::{alloc, dealloc, Layout};
use std::cell::RefCell;
use std::ptr;

struct Host {
    session: Option<ExportSession>,
    lag_ticks: f64,
    assets: String,
    result: Vec<u8>,
    error: String,
    /// RGBA returned by `painter_texture_ptr`, stable until the next call.
    texture: Vec<u8>,
    peak_memory: usize,
    init_memory: usize,
}

impl Host {
    fn new() -> Self {
        Self {
            session: None,
            lag_ticks: 3.0,
            assets: "vfs".into(),
            result: Vec::new(),
            error: String::new(),
            texture: Vec::new(),
            peak_memory: 0,
            init_memory: 0,
        }
    }

    fn note_memory(&mut self) {
        self.peak_memory = self.peak_memory.max(linear_memory());
    }
}

thread_local! {
    static HOST: RefCell<Host> = RefCell::new(Host::new());
}

fn linear_memory() -> usize {
    #[cfg(target_arch = "wasm32")]
    {
        core::arch::wasm32::memory_size(0) * 65536
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

fn bytes<'a>(ptr: *const u8, len: u32) -> Result<&'a [u8], String> {
    if len == 0 {
        return Ok(&[]);
    }
    if ptr.is_null() {
        return Err("null pointer".into());
    }
    Ok(unsafe { std::slice::from_raw_parts(ptr, len as usize) })
}

fn store_result(host: &mut Host, bytes: &[u8]) {
    host.result.clear();
    host.result.extend_from_slice(bytes);
}

fn fail(host: &mut Host, message: impl Into<String>) -> i32 {
    host.error = message.into();
    1
}

#[no_mangle]
pub extern "C" fn painter_alloc(len: u32) -> *mut u8 {
    if len == 0 {
        return ptr::null_mut();
    }
    let Ok(layout) = Layout::from_size_align(len as usize, 8) else {
        return ptr::null_mut();
    };
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        return ptr::null_mut();
    }
    unsafe { ptr::write_bytes(ptr, 0, len as usize) };
    ptr
}

#[no_mangle]
pub extern "C" fn painter_free(ptr: *mut u8, len: u32) {
    if ptr.is_null() || len == 0 {
        return;
    }
    let Ok(layout) = Layout::from_size_align(len as usize, 8) else {
        return;
    };
    unsafe { dealloc(ptr, layout) };
}

#[no_mangle]
pub extern "C" fn painter_vfs_put(
    path_ptr: *const u8,
    path_len: u32,
    data_ptr: *const u8,
    data_len: u32,
) -> i32 {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        let path = match bytes(path_ptr, path_len).and_then(|b| {
            std::str::from_utf8(b)
                .map(|s| s.to_owned())
                .map_err(|e| e.to_string())
        }) {
            Ok(path) => path,
            Err(error) => return fail(&mut host, error),
        };
        let data = match bytes(data_ptr, data_len) {
            Ok(data) => data,
            Err(error) => return fail(&mut host, error),
        };
        match clash_native_ui_viewer::vfs::put(&path, data) {
            Ok(()) => {
                host.error.clear();
                0
            }
            Err(error) => fail(&mut host, error),
        }
    })
}

#[no_mangle]
pub extern "C" fn painter_init(
    cards_ptr: *const u8,
    cards_len: u32,
    width: u32,
    height: u32,
) -> i32 {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        let cards = match bytes(cards_ptr, cards_len).and_then(parse_cards) {
            Ok(cards) => cards,
            Err(error) => return fail(&mut host, error),
        };
        let width = if width == 0 { 540 } else { width };
        let height = if height == 0 { 900 } else { height };
        let lag = host.lag_ticks;
        let root = host.assets.clone();
        match ExportSession::open(std::path::Path::new(&root), &cards, width, height, lag) {
            Ok(session) => {
                host.session = Some(session);
                host.error.clear();
                host.init_memory = linear_memory();
                host.note_memory();
                0
            }
            Err(error) => fail(&mut host, error),
        }
    })
}

#[no_mangle]
pub extern "C" fn painter_set_lag_ticks(lag_ticks: f64) {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        if lag_ticks.is_finite() {
            let lag = lag_ticks.max(0.);
            host.lag_ticks = lag;
            if let Some(session) = host.session.as_mut() {
                session.set_lag_ticks(lag);
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn painter_observe(snapshot_ptr: *const u8, snapshot_len: u32) -> i32 {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        let snapshot = match bytes(snapshot_ptr, snapshot_len) {
            Ok(bytes) => bytes.to_vec(),
            Err(error) => return fail(&mut host, error),
        };
        if host.session.is_none() {
            return fail(&mut host, "painter_init was not called");
        }
        match host.session.as_mut().unwrap().observe_json(&snapshot) {
            Ok(()) => {
                host.error.clear();
                host.note_memory();
                0
            }
            Err(error) => {
                host.error = error;
                1
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn painter_step(dt_seconds: f64) -> i32 {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        if host.session.is_none() {
            host.error = "painter_init was not called".into();
            return -1;
        }
        let code = host.session.as_mut().unwrap().step(dt_seconds);
        match code {
            Ok(code) => {
                if code > 0 {
                    let packet = host.session.as_ref().unwrap().packet().to_vec();
                    host.result = packet;
                }
                host.error.clear();
                host.note_memory();
                code
            }
            Err(error) => {
                host.error = error;
                -1
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn painter_result_ptr() -> *const u8 {
    HOST.with(|host| {
        let host = host.borrow();
        if host.result.is_empty() {
            ptr::null()
        } else {
            host.result.as_ptr()
        }
    })
}

#[no_mangle]
pub extern "C" fn painter_result_len() -> u32 {
    HOST.with(|host| host.borrow().result.len() as u32)
}

#[no_mangle]
pub extern "C" fn painter_texture_ptr(id_ptr: *const u8, id_len: u32) -> *const u8 {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        host.texture.clear();
        let Ok(id) = bytes(id_ptr, id_len).and_then(|b| {
            std::str::from_utf8(b)
                .map(|s| s.to_owned())
                .map_err(|e| e.to_string())
        }) else {
            return ptr::null();
        };
        let Some(pixels) = host.session.as_ref().and_then(|session| session.copy_rgba(&id)) else {
            return ptr::null();
        };
        host.texture = pixels;
        host.note_memory();
        host.texture.as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn painter_texture_len(id_ptr: *const u8, id_len: u32) -> u32 {
    HOST.with(|host| {
        let host = host.borrow();
        let Ok(id) = bytes(id_ptr, id_len).and_then(|b| {
            std::str::from_utf8(b)
                .map(|s| s.to_owned())
                .map_err(|e| e.to_string())
        }) else {
            return 0;
        };
        host.session
            .as_ref()
            .and_then(|session| session.copy_rgba(&id))
            .map(|pixels| pixels.len() as u32)
            .unwrap_or(0)
    })
}

#[no_mangle]
pub extern "C" fn painter_texture_release(id_ptr: *const u8, id_len: u32) {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        let Ok(id) = bytes(id_ptr, id_len).and_then(|b| {
            std::str::from_utf8(b)
                .map(|s| s.to_owned())
                .map_err(|e| e.to_string())
        }) else {
            return;
        };
        if let Some(session) = host.session.as_mut() {
            session.release_texture(&id);
        }
        host.texture.clear();
        host.note_memory();
    });
}

/// Record the host's measurement of the last `painter_step`, in milliseconds.
/// `Instant` panics on `wasm32-unknown-unknown` and the module has no imports,
/// so the page times the call and passes the duration in.
#[no_mangle]
pub extern "C" fn painter_set_draw_ms(ms: f64) {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        if let Some(session) = host.session.as_mut() {
            session.set_draw_ms(ms);
        }
    });
}

#[no_mangle]
pub extern "C" fn painter_reset() {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        if let Some(session) = host.session.as_mut() {
            session.reset();
        }
        host.result.clear();
        host.texture.clear();
        host.error.clear();
    });
}

#[no_mangle]
pub extern "C" fn painter_stats_json() -> i32 {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        host.note_memory();
        if host.session.is_none() {
            return fail(&mut host, "painter_init was not called");
        }
        let mut stats = host.session.as_ref().unwrap().stats_value();
        if let Some(object) = stats.as_object_mut() {
            object.insert(
                "wasm_memory".into(),
                serde_json::json!(linear_memory()),
            );
            object.insert(
                "wasm_memory_peak".into(),
                serde_json::json!(host.peak_memory),
            );
            object.insert(
                "wasm_memory_after_init".into(),
                serde_json::json!(host.init_memory),
            );
        }
        match serde_json::to_vec(&stats) {
            Ok(bytes) => {
                store_result(&mut host, &bytes);
                host.error.clear();
                0
            }
            Err(error) => fail(&mut host, error.to_string()),
        }
    })
}

#[no_mangle]
pub extern "C" fn painter_last_error() -> i32 {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        let message = host.error.clone();
        store_result(&mut host, message.as_bytes());
        0
    })
}

fn parse_cards(bytes: &[u8]) -> Result<Vec<String>, String> {
    if bytes.iter().all(|byte| byte.is_ascii_whitespace()) {
        return Ok(Vec::new());
    }
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|e| format!("cards json: {e}"))?;
    let list = value
        .as_array()
        .ok_or("cards json must be an array of native card names")?;
    list.iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| "card name must be a string".to_string())
        })
        .collect()
}
