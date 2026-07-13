//! Post-processing pipeline for framebuffer effects.
//!
//! Render passes sit between the Painter and DirtyDiff in the rendering pipeline.
//! Each pass transforms the framebuffer before it is diffed and encoded to ANSI.

mod pipeline;

pub mod effects;

pub use pipeline::{PassPriority, RenderPass, RenderPassContext, RenderPipeline};

/// Result of processing a render pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassResult {
    /// Pass made no changes — skip dirty re-evaluation.
    Unchanged,
    /// Pass modified the buffer — dirty diff needs re-run.
    Modified,
}
