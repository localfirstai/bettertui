//! Render tree construction: builds a sorted render tree from the arena + layout results.
//! Provides paint context, clip bounds, and viewport culling for the painting phase.

mod build;
pub mod culling;
mod object;
mod paint;
mod tree;

pub use build::build_render_tree;
pub use build::build_render_tree_with_viewport;
pub use culling::{PositionedChild, PrimaryAxis, get_objects_in_viewport};
pub use object::RenderObject;
pub use paint::{ClipBounds, PaintBounds, PaintContext, PaintFlags, Viewport};
pub use tree::RenderTree;
