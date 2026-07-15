pub mod tree;

pub mod input;

pub mod animation;
pub mod ansi;
pub mod dirty_diff;
pub mod engine;
pub mod ffi;
pub mod font;
pub mod framebuffer;
pub mod glyph;
pub mod graphics;
pub mod plugin;
pub mod protocol;
pub mod pty;
pub mod render;
pub mod scheduler;
pub mod syntax;
pub mod taffy;
pub mod text;
pub mod theme;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
