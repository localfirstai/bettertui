use crate::capabilities::brand::TerminalBrand;
use crate::capabilities::clipboard::ClipboardCapabilities;
use crate::capabilities::graphics::GraphicsCapabilities;
use crate::capabilities::input::InputCapabilities;
use crate::capabilities::rendering::RenderCapabilities;
use crate::capabilities::unicode::UnicodeCapabilities;
use crate::capabilities::window::WindowMetrics;

#[derive(Debug, Clone)]
pub struct CapabilityDetector {
    pub brand: TerminalBrand,
    pub render: RenderCapabilities,
    pub unicode: UnicodeCapabilities,
    pub input: InputCapabilities,
    pub graphics: GraphicsCapabilities,
    pub clipboard: ClipboardCapabilities,
    pub window: WindowMetrics,
}

impl CapabilityDetector {
    pub fn detect() -> Self {
        Self {
            brand: TerminalBrand::detect(),
            render: RenderCapabilities::detect(),
            unicode: UnicodeCapabilities::detect(),
            input: InputCapabilities::detect(),
            graphics: GraphicsCapabilities::detect(),
            clipboard: ClipboardCapabilities::detect(),
            window: WindowMetrics::detect(),
        }
    }

    pub fn brand(&self) -> &TerminalBrand {
        &self.brand
    }

    pub fn render(&self) -> &RenderCapabilities {
        &self.render
    }

    pub fn unicode(&self) -> &UnicodeCapabilities {
        &self.unicode
    }

    pub fn input(&self) -> &InputCapabilities {
        &self.input
    }

    pub fn graphics(&self) -> &GraphicsCapabilities {
        &self.graphics
    }

    pub fn clipboard(&self) -> &ClipboardCapabilities {
        &self.clipboard
    }

    pub fn window(&self) -> &WindowMetrics {
        &self.window
    }

    pub fn is_known_terminal(&self) -> bool {
        self.brand.is_known()
    }

    pub fn supports_true_color(&self) -> bool {
        self.render.true_color
    }

    pub fn supports_kitty_keyboard(&self) -> bool {
        self.input.kitty_keyboard
    }

    pub fn supports_bracketed_paste(&self) -> bool {
        self.input.bracketed_paste
    }

    pub fn supports_focus_events(&self) -> bool {
        self.input.focus_events
    }

    pub fn supports_osc52(&self) -> bool {
        self.clipboard.osc52
    }

    pub fn supports_osc8(&self) -> bool {
        self.clipboard.osc8
    }

    pub fn supports_kitty_graphics(&self) -> bool {
        self.graphics.kitty_graphics
    }

    pub fn supports_sixel(&self) -> bool {
        self.graphics.sixel
    }

    pub fn supports_iterm_images(&self) -> bool {
        self.graphics.iterm_images
    }

    pub fn terminal_size(&self) -> (u16, u16) {
        (self.window.terminal_width, self.window.terminal_height)
    }

    pub fn pixel_size(&self) -> Option<(u32, u32)> {
        self.window.pixel_width.zip(self.window.pixel_height)
    }

    pub fn cell_size(&self) -> Option<(u32, u32)> {
        self.window.cell_width.zip(self.window.cell_height)
    }
}

impl Default for CapabilityDetector {
    fn default() -> Self {
        Self::detect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detector_detect() {
        let detector = CapabilityDetector::detect();
        assert!(detector.is_known_terminal() || !detector.is_known_terminal());
    }

    #[test]
    fn detector_default() {
        let detector = CapabilityDetector::default();
        assert!(detector.terminal_size().0 > 0);
        assert!(detector.terminal_size().1 > 0);
    }

    #[test]
    fn detector_capabilities() {
        let detector = CapabilityDetector::detect();
        assert!(detector.supports_true_color() || !detector.supports_true_color());
        assert!(detector.supports_kitty_keyboard() || !detector.supports_kitty_keyboard());
        assert!(detector.supports_bracketed_paste());
        assert!(detector.supports_focus_events());
    }
}
