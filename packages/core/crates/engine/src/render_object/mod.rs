//! Render tree data structures: RenderObject and RenderTree.
//!
//! These are the output types from the layout pipeline, consumed by the painter.
//! The layout pipeline itself (build, culling, paint primitives) lives in `crate::layout`.

mod object;
mod tree;

pub use object::RenderObject;
pub use tree::RenderTree;
