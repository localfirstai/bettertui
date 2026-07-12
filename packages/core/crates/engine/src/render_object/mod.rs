//! Render tree construction: builds a sorted render tree from the arena + layout results.
//! Provides paint context and clip bounds for the painting phase.

mod build;
mod object;
mod paint;
mod tree;

pub use build::build_render_tree;
pub use object::RenderObject;
pub use paint::{ClipBounds, PaintBounds, PaintContext, PaintFlags};
pub use tree::RenderTree;
