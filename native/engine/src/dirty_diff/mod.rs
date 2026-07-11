//! Dirty region computation: diffs two framebuffers to find changed areas for incremental rendering.

mod diff;

pub use diff::{DirtyDiff, DirtyRegion};
