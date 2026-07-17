pub mod tree;

pub mod input;

pub mod animation;
pub mod ansi;
pub mod clock;
pub mod dirty_diff;
pub mod engine;
pub mod event_bus;
pub mod event_emitter;
pub mod event_pipeline;
pub mod ffi;
pub mod font;
pub mod framebuffer;
pub mod glyph;
pub mod graphics;
pub mod hit_grid;
pub mod logger;
pub mod plugin;
pub mod protocol;
pub mod pty;
pub mod render;
pub mod scheduler;
pub mod span_feed;
pub mod syntax;
pub mod taffy;
pub mod terminal;
pub mod text;
pub mod theme;

#[cfg(feature = "napi")]
pub mod napi;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
