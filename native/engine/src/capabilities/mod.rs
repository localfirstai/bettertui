mod brand;
mod clipboard;
mod detection;
mod graphics;
mod input;
mod rendering;
mod unicode;
mod window;

pub use brand::TerminalBrand;
pub use clipboard::ClipboardCapabilities;
pub use detection::CapabilityDetector;
pub use graphics::GraphicsCapabilities;
pub use input::InputCapabilities;
pub use rendering::{ColorSupport, RenderCapabilities};
pub use unicode::{EmojiWidth, UnicodeCapabilities, UnicodeVersion};
pub use window::WindowMetrics;

use std::sync::OnceLock;

static GLOBAL_CAPABILITIES: OnceLock<CapabilityDetector> = OnceLock::new();

pub fn global_capabilities() -> &'static CapabilityDetector {
    GLOBAL_CAPABILITIES.get_or_init(CapabilityDetector::detect)
}

pub fn init_capabilities() -> &'static CapabilityDetector {
    GLOBAL_CAPABILITIES.get_or_init(CapabilityDetector::detect)
}
