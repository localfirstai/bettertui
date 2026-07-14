//! Consolidated end-to-end benchmarks for the BetterTUI Rust crates.
//!
//! All Rust benchmarking lives in this crate. Public API surface of the
//! `engine`, `widgets`, and `terminal` crates is exercised through
//! [`criterion`] benchmark targets under `benches/`.
//!
//! The `benchmark` module contains a lightweight timing/throughput harness
//! migrated from the engine crate; it is used for crate-internal micro
//! benchmarks that don't need the full criterion infrastructure.

pub mod benchmark;

pub use benchmark::{BenchmarkHarness, BenchmarkResult, OpCounter};
