//! Rendering pipeline: objects, painter, renderer, backend, post-process effects.

pub mod effects;

#[allow(clippy::module_inception)]
mod render;

pub use render::{
    AnsiBackend, CursorState, Painter, PassPriority, PassResult, RenderBackend, RenderFrame,
    RenderObject, RenderPass, RenderPassContext, RenderPipeline, RenderTree, Renderer,
};
