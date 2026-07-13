//! Layout engine: Taffy-based layout computation with tree synchronization and result mapping.
//!
//! This module contains everything related to layout:
//! - `types` — Input types: LayoutProps, Sizing, enums (FlexDirection, etc.)
//! - `engine` — LayoutEngine, LayoutTreeSync, LayoutResult: Taffy wrapping + arena bridging + result mapping
//! - `build` — Render tree builder: consumes LayoutResults → produces RenderTree
//! - `culling` — Binary search viewport culling for large scrollable lists
//! - `paint` — Paint primitives: PaintBounds, Viewport, ClipBounds, PaintContext, PaintFlags

mod build;
pub mod culling;
mod engine;
pub mod paint;
pub mod types;

pub use build::{build_render_tree, build_render_tree_with_viewport};
pub use engine::{LayoutEngine, LayoutResult, LayoutTreeSync};
pub use types::*;
