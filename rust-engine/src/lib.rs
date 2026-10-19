//! Native Rust compilation runtime for source-bound recovered battle code.
//!
//! This crate is under development. The slow, verified original-engine runtime
//! remains available on the `slow-working` Git branch and separate worktree.
pub mod cpu;
pub mod heap;
pub mod memory;
pub mod runtime;
