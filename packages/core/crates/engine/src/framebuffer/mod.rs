//! Frame buffer: cell-based pixel buffer with diff computation and dirty region tracking.

mod buffer;
mod cell;

pub use buffer::FrameBuffer;
pub use cell::{Cell, CellAttributes};
