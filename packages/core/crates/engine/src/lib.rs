pub mod tree;

pub mod animation;
pub mod ansi;
pub mod benchmark;
pub mod dirty_diff;
pub mod engine;
pub mod events;
pub mod ffi;
pub mod focus;
pub mod framebuffer;
pub mod glyph;
pub mod graphics;
pub mod input;
pub mod keybinding;
pub mod layout;
pub mod neovim;
pub mod nerdfont;
pub mod plugin;
pub mod protocol;
pub mod pty;
pub mod render;
pub mod scheduler;
pub mod syntax;
pub mod terminal;
pub mod text;
pub mod widgets;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
