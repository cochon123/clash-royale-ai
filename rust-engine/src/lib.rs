//! Native Rust compilation runtime for source-bound recovered battle code.
//!
//! This crate is under development. The slow, verified original-engine runtime
//! remains available on the `slow-working` Git branch and separate worktree.
pub mod cpu;
pub mod generated;
pub mod heap;
pub mod memory;
pub mod rl;
pub mod runtime;
pub mod session;
/// Exact source/compiler identities embedded when this executable was built.
pub const BUILD_PROVENANCE: &str = include_str!(concat!(env!("OUT_DIR"), "/provenance.json"));
