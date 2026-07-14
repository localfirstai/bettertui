pub mod tree;

pub mod input;

pub mod animation;
pub mod ansi;
pub mod dirty_diff;
pub mod engine;
pub mod ffi;
pub mod framebuffer;
pub mod glyph;
pub mod graphics;
pub mod layout;
pub mod nerdfont;
pub mod plugin;
pub mod protocol;
pub mod pty;
pub mod render;
pub mod scheduler;
pub mod syntax;
pub mod text;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
