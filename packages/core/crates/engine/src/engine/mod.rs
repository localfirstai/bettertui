//! High-level engine API integrating the renderer, event system, and runtime.
//! The main entry point for framework users.

pub mod core;
pub mod inspector;

pub use core::Engine;
pub use inspector::Inspector;
