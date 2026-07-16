pub mod ascii;
pub mod loader;
pub mod metrics;
pub mod provider;
pub mod registry;

mod builtin {
    include!(concat!(env!("OUT_DIR"), "/builtin.rs"));
}

pub use ascii::AsciiFontLayout;
pub use ascii::AsciiFontSegment;
pub use ascii::FONT_NAMES;
pub use ascii::{
    coordinate_to_character_index, get_character_positions, layout_text, measure_text, render_font_to_frame_buffer,
    render_text,
};
pub use loader::BundledFont;
pub use loader::FontMetadata;
pub use metrics::FontMetrics;
pub use metrics::FontMetricsCache;
pub use provider::FontProvider;
pub use registry::IconCategory;
pub use registry::IconGlyph;
pub use registry::IconRegistry;
