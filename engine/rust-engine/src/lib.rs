//! Native Rust compilation runtime for source-bound recovered battle code.
//!
//! The original-engine reference runtime remains in `native-engine/` in this
//! checkout. The earlier source is also retained on the `slow-working` branch.
pub mod cpu;
pub mod generated;
pub mod heap;
pub mod memory;
pub mod rl;
pub mod runtime;
pub mod session;
pub mod snapshot;
/// Exact source/compiler identities embedded when this executable was built.
pub const BUILD_PROVENANCE: &str = include_str!(concat!(env!("OUT_DIR"), "/provenance.json"));
