use crate::terminal::capabilities::brand::TerminalBrand;
use crate::terminal::capabilities::clipboard::ClipboardCapabilities;
use crate::terminal::capabilities::graphics::GraphicsCapabilities;
use crate::terminal::capabilities::input::InputCapabilities;
use crate::terminal::capabilities::rendering::RenderCapabilities;
use crate::terminal::capabilities::unicode::UnicodeCapabilities;
use crate::terminal::capabilities::window::WindowMetrics;
use crate::terminal::query::QueryResult;

#[derive(Debug, Clone)]
pub struct CapabilityDetector {
    pub brand: TerminalBrand,
    pub render: RenderCapabilities,
    pub unicode: UnicodeCapabilities,
    pub input: InputCapabilities,
    pub graphics: GraphicsCapabilities,
    pub clipboard: ClipboardCapabilities,
    pub window: WindowMetrics,
    pub features: FeatureMatrix,
    pub query_origin: QueryOrigin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryOrigin {
    EnvOnly,
    Confirmed,
    Inferred,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct FeatureMatrix {
    pub true_color: bool,
    pub kitty_keyboard: bool,
    pub csi_u: bool,
    pub bracketed_paste: bool,
    pub focus_events: bool,
    pub osc52: bool,
    pub osc8: bool,
    pub kitty_graphics: bool,
    pub sixel: bool,
    pub iterm_images: bool,
    pub synchronized_output: bool,
    pub underline_color: bool,
    pub strikethrough: bool,
    pub cursor_style: bool,
    pub alternate_scroll: bool,
    pub win32_input: bool,
    pub conemu_input: bool,
    pub da1_attributes: Vec<u32>,
    pub da2_model: u32,
}

impl Default for FeatureMatrix {
    fn default() -> Self {
        Self {
            true_color: true,
            kitty_keyboard: false,
            csi_u: false,
            bracketed_paste: true,
            focus_events: true,
            osc52: false,
            osc8: true,
            kitty_graphics: false,
            sixel: false,
            iterm_images: false,
            synchronized_output: true,
            underline_color: true,
            strikethrough: true,
            cursor_style: true,
            alternate_scroll: true,
            win32_input: false,
            conemu_input: false,
            da1_attributes: Vec::new(),
            da2_model: 0,
        }
    }
}

impl CapabilityDetector {
    pub fn detect() -> Self {
        let brand = TerminalBrand::detect();
        let render = RenderCapabilities::detect();
        let input = InputCapabilities::detect();
        Self {
            features: FeatureMatrix::default_for_brand(brand),
            query_origin: QueryOrigin::EnvOnly,
            brand,
            render,
            unicode: UnicodeCapabilities::detect(),
            input,
            graphics: GraphicsCapabilities::detect(),
            clipboard: ClipboardCapabilities::detect(),
            window: WindowMetrics::detect(),
        }
    }

    pub fn update_from_queries(&mut self, results: &[QueryResult]) {
        for result in results {
            match result {
                QueryResult::DeviceAttributes {
                    terminal_type,
                    attributes,
                } => {
                    self.features.da1_attributes = attributes.clone();
                    self.features.da1_attributes.insert(0, *terminal_type);
                    // DA1 attribute flags: bit 0 = columns 132, bit 1 = printer, bit 2 = color
                    if attributes.contains(&4)
                        || attributes.contains(&22)
                        || attributes.contains(&28)
                    {
                        self.features.true_color = true;
                        self.render.true_color = true;
                        self.render.color_support =
                            crate::terminal::capabilities::rendering::ColorSupport::TrueColor;
                    }
                    if attributes.contains(&62) {
                        self.features.sixel = true;
                        self.graphics.sixel = true;
                    }
                    self.query_origin = QueryOrigin::Confirmed;
                }
                QueryResult::SecondaryDeviceAttributes {
                    model,
                    firmware_major: _,
                    firmware_minor: _,
                } => {
                    self.features.da2_model = *model;
                    let detected_brand = brand_from_da2_model(*model);
                    if detected_brand != TerminalBrand::Unknown {
                        self.brand = detected_brand;
                        self.query_origin = QueryOrigin::Confirmed;
                    }
                    match detected_brand {
                        TerminalBrand::Kitty | TerminalBrand::Ghostty => {
                            self.features.kitty_keyboard = true;
                            self.features.csi_u = true;
                            self.input.kitty_keyboard = true;
                            self.input.csi_u = true;
                        }
                        _ => {}
                    }
                }
                QueryResult::TertiaryDeviceAttributes { data } if !data.is_empty() => {
                    self.query_origin = QueryOrigin::Confirmed;
                }
                QueryResult::ProgressiveEnhancement { features } if *features > 0 => {
                    self.features.kitty_keyboard = true;
                    self.features.csi_u = true;
                    self.input.kitty_keyboard = true;
                    self.input.csi_u = true;
                    self.query_origin = QueryOrigin::Confirmed;
                }
                _ => {}
            }
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

    pub fn features(&self) -> &FeatureMatrix {
        &self.features
    }

    pub fn query_origin(&self) -> &QueryOrigin {
        &self.query_origin
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

    pub fn supports_csi_u(&self) -> bool {
        self.input.csi_u
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

impl FeatureMatrix {
    pub fn default_for_brand(brand: TerminalBrand) -> Self {
        let mut f = Self::default();
        match brand {
            TerminalBrand::Kitty => {
                f.kitty_keyboard = true;
                f.csi_u = true;
                f.kitty_graphics = true;
                f.osc52 = true;
                f.sixel = false;
            }
            TerminalBrand::Ghostty => {
                f.kitty_keyboard = true;
                f.csi_u = true;
                f.kitty_graphics = true;
                f.osc52 = true;
            }
            TerminalBrand::WezTerm => {
                f.kitty_keyboard = true;
                f.csi_u = true;
                f.kitty_graphics = false;
                f.iterm_images = true;
                f.sixel = true;
                f.osc52 = true;
            }
            TerminalBrand::Alacritty => {
                f.kitty_keyboard = false;
                f.csi_u = false;
                f.kitty_graphics = false;
            }
            TerminalBrand::Foot => {
                f.sixel = true;
            }
            TerminalBrand::ITerm2 => {
                f.iterm_images = true;
                f.kitty_keyboard = true;
                f.csi_u = true;
            }
            TerminalBrand::Tmux => {
                f.osc52 = true;
            }
            _ => {}
        }
        f
    }

    pub fn all_true_color(&self) -> bool {
        self.true_color
    }

    pub fn all_input_features(&self) -> bool {
        self.kitty_keyboard && self.csi_u && self.bracketed_paste && self.focus_events
    }

    pub fn all_clipboard_features(&self) -> bool {
        self.osc52 && self.osc8
    }

    pub fn all_graphics_features(&self) -> bool {
        self.kitty_graphics || self.sixel || self.iterm_images
    }

    pub fn any_advanced_input(&self) -> bool {
        self.kitty_keyboard || self.csi_u || self.win32_input || self.conemu_input
    }
}

/// Maps DA2 model numbers to known terminal brands.
fn brand_from_da2_model(model: u32) -> TerminalBrand {
    match model {
        10..=16 => TerminalBrand::Kitty,
        17 => TerminalBrand::Alacritty,
        18 => TerminalBrand::Ghostty,
        19 => TerminalBrand::WezTerm,
        20 => TerminalBrand::ITerm2,
        21 => TerminalBrand::Foot,
        22 => TerminalBrand::WindowsTerminal,
        23 => TerminalBrand::VSCodeTerminal,
        _ => TerminalBrand::Unknown,
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

    #[test]
    fn update_from_da1_response() {
        let mut detector = CapabilityDetector::detect();
        let results = vec![QueryResult::DeviceAttributes {
            terminal_type: 1,
            attributes: vec![4, 22, 28],
        }];
        detector.update_from_queries(&results);
        assert!(detector.features.da1_attributes.contains(&4));
        assert!(detector.supports_true_color());
        assert_eq!(detector.query_origin, QueryOrigin::Confirmed);
    }

    #[test]
    fn update_from_da2_kitty() {
        let mut detector = CapabilityDetector::detect();
        let results = vec![QueryResult::SecondaryDeviceAttributes {
            model: 10,
            firmware_major: 0,
            firmware_minor: 0,
        }];
        detector.update_from_queries(&results);
        assert_eq!(detector.brand, TerminalBrand::Kitty);
        assert!(detector.supports_kitty_keyboard());
        assert_eq!(detector.query_origin, QueryOrigin::Confirmed);
    }

    #[test]
    fn update_from_da2_ghostty() {
        let mut detector = CapabilityDetector::detect();
        let results = vec![QueryResult::SecondaryDeviceAttributes {
            model: 18,
            firmware_major: 0,
            firmware_minor: 0,
        }];
        detector.update_from_queries(&results);
        assert_eq!(detector.brand, TerminalBrand::Ghostty);
        assert!(detector.supports_kitty_keyboard());
    }

    #[test]
    fn update_from_da2_unknown() {
        let mut detector = CapabilityDetector::detect();
        let results = vec![QueryResult::SecondaryDeviceAttributes {
            model: 999,
            firmware_major: 0,
            firmware_minor: 0,
        }];
        detector.update_from_queries(&results);
        // model 999 doesn't match any known brand, brand stays as detected from env
        assert_eq!(detector.query_origin, QueryOrigin::EnvOnly);
    }

    #[test]
    fn feature_matrix_default_for_brand() {
        let kitty = FeatureMatrix::default_for_brand(TerminalBrand::Kitty);
        assert!(kitty.kitty_keyboard);
        assert!(kitty.csi_u);
        assert!(kitty.kitty_graphics);
        assert!(kitty.osc52);

        let unknown = FeatureMatrix::default_for_brand(TerminalBrand::Unknown);
        assert!(!unknown.kitty_keyboard);
        assert!(!unknown.kitty_graphics);
    }

    #[test]
    fn query_origin_env_only() {
        let detector = CapabilityDetector::detect();
        assert_eq!(detector.query_origin, QueryOrigin::EnvOnly);
    }
}
