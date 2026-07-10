pub mod tree;

pub mod animation;
pub mod ansi;
pub mod benchmark;
pub mod capabilities;
pub mod clipboard;
pub mod dirty_diff;
pub mod editor;
pub mod engine;
pub mod events;
pub mod ffi;
pub mod framebuffer;
pub mod graphics;
pub mod keyboard;
pub mod layout;
pub mod mouse;
pub mod painter;
pub mod protocol;
pub mod render_object;
pub mod renderer;
pub mod scheduler;
pub mod screen;
pub mod selection;
pub mod terminal;
pub mod widgets;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
