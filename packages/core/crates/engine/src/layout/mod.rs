//! Layout engine: Taffy-based layout computation with tree synchronization and result mapping.
//!
//! This module contains everything related to layout:
//! - `types` — Input types: LayoutProps, Sizing, enums (FlexDirection, etc.)
//! - `compute` — LayoutEngine wrapping Taffy: style mapping, layout computation, measure functions
//! - `sync` — LayoutTreeSync: bridges NodeArena ↔ Taffy
//! - `result` — LayoutResult: resolved position/size/padding/border
//! - `build` — Render tree builder: consumes LayoutResults → produces RenderTree
//! - `culling` — Binary search viewport culling for large scrollable lists
//! - `paint` — Paint primitives: PaintBounds, Viewport, ClipBounds, PaintContext, PaintFlags

mod build;
mod compute;
pub mod culling;
pub mod paint;
mod result;
mod sync;
pub mod types;

pub use build::{build_render_tree, build_render_tree_with_viewport};
pub use compute::LayoutEngine;
pub use result::LayoutResult;
pub use sync::LayoutTreeSync;
pub use types::*;
