//! Rendering pipeline: objects, painter, renderer, backend, post-process effects, compositor.

pub mod compositor;
pub mod effects;

mod ansi;
mod backend;
mod object;
mod painter;
mod pipeline;
mod renderer;

pub use ansi::AnsiBackend;
pub use backend::RenderBackend;
pub use object::{RenderObject, RenderTree};
pub use painter::Painter;
pub use pipeline::{PassPriority, RenderPass, RenderPassContext, RenderPipeline};
pub use renderer::{RenderFrame, Renderer};

/// Result of executing a render pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassResult {
    /// The pass did not change the buffer.
    Unchanged,
    /// The pass modified the buffer.
    Modified,
}
