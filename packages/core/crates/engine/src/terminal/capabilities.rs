//! Terminal capability detection: brand, color, unicode, input, rendering, and clipboard support.

use std::env;
use std::sync::OnceLock;

use crate::terminal::query::QueryResult;

static GLOBAL_CAPABILITIES: OnceLock<CapabilityDetector> = OnceLock::new();

pub fn global_capabilities() -> &'static CapabilityDetector {
    GLOBAL_CAPABILITIES.get_or_init(CapabilityDetector::detect)
}

pub fn init_capabilities() -> &'static CapabilityDetector {
    GLOBAL_CAPABILITIES.get_or_init(CapabilityDetector::detect)
}

// === brand.rs ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TerminalBrand {
    Ghostty,
    Kitty,
    WezTerm,
    Alacritty,
    Foot,
    ITerm2,
    WindowsTerminal,
    VSCodeTerminal,
    Tmux,
    GnuScreen,
    Warp,
    #[default]
    Unknown,
}

impl TerminalBrand {
    pub fn detect() -> Self {
        if env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
            return Self::Ghostty;
        }

        if env::var("KITTY_WINDOW_ID").is_ok() {
            return Self::Kitty;
        }

        if env::var("WEZTERM_PANE").is_ok()
            || env::var("TERM_PROGRAM").is_ok_and(|v| v == "WezTerm")
        {
            return Self::WezTerm;
        }

        if env::var("TERM_PROGRAM").is_ok_and(|v| v == "Alacritty") {
            return Self::Alacritty;
        }

        if env::var("FOOT_PID").is_ok() {
            return Self::Foot;
        }

        if env::var("TERM_PROGRAM").is_ok_and(|v| v == "iTerm.app") {
            return Self::ITerm2;
        }

        if env::var("WT_SESSION").is_ok() {
            return Self::WindowsTerminal;
        }

        if env::var("TERM_PROGRAM").is_ok_and(|v| v == "vscode") {
            return Self::VSCodeTerminal;
        }

        if env::var("TMUX").is_ok() {
            return Self::Tmux;
        }

        if env::var("STY").is_ok() {
            return Self::GnuScreen;
        }

        if env::var("TERM_PROGRAM").is_ok_and(|v| v == "Warp") {
            return Self::Warp;
        }

        Self::Unknown
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::Ghostty => "Ghostty",
            Self::Kitty => "Kitty",
            Self::WezTerm => "WezTerm",
            Self::Alacritty => "Alacritty",
            Self::Foot => "Foot",
            Self::ITerm2 => "iTerm2",
            Self::WindowsTerminal => "Windows Terminal",
            Self::VSCodeTerminal => "VSCode Terminal",
            Self::Tmux => "tmux",
            Self::GnuScreen => "GNU Screen",
            Self::Warp => "Warp",
            Self::Unknown => "Unknown",
        }
    }

    pub fn is_known(&self) -> bool {
        *self != Self::Unknown
    }
}

impl std::fmt::Display for TerminalBrand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// === clipboard.rs ===

#[derive(Debug, Clone)]
pub struct ClipboardCapabilities {
    pub osc52: bool,
    pub osc8: bool,
}

impl ClipboardCapabilities {
    pub fn detect() -> Self {
        let is_kitty = env::var("KITTY_WINDOW_ID").is_ok();
        let is_ghostty = env::var("GHOSTTY_RESOURCES_DIR").is_ok();
        let is_wezterm = env::var("WEZTERM_PANE").is_ok();
        let is_tmux = env::var("TMUX").is_ok();

        Self {
            osc52: is_kitty || is_ghostty || is_wezterm || is_tmux,
            osc8: is_kitty || is_ghostty || is_wezterm,
        }
    }

    pub fn supports_osc52(&self) -> bool {
        self.osc52
    }

    pub fn supports_osc8(&self) -> bool {
        self.osc8
    }
}

impl Default for ClipboardCapabilities {
    fn default() -> Self {
        Self::detect()
    }
}

// === detection.rs ===

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
                    if attributes.contains(&4)
                        || attributes.contains(&22)
                        || attributes.contains(&28)
                    {
                        self.features.true_color = true;
                        self.render.true_color = true;
                        self.render.color_support = ColorSupport::TrueColor;
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

// === graphics.rs ===

#[derive(Debug, Clone)]
pub struct GraphicsCapabilities {
    pub kitty_graphics: bool,
    pub sixel: bool,
    pub iterm_images: bool,
}

impl GraphicsCapabilities {
    pub fn detect() -> Self {
        let is_kitty = env::var("KITTY_WINDOW_ID").is_ok();
        let is_ghostty = env::var("GHOSTTY_RESOURCES_DIR").is_ok();
        let is_iterm = env::var("TERM_PROGRAM").is_ok_and(|v| v == "iTerm.app");
        let is_wezterm = env::var("WEZTERM_PANE").is_ok();

        Self {
            kitty_graphics: is_kitty || is_ghostty,
            sixel: Self::detect_sixel(),
            iterm_images: is_iterm || is_wezterm,
        }
    }

    fn detect_sixel() -> bool {
        if let Ok(val) = env::var("TERM")
            && val.contains("sixel")
        {
            return true;
        }

        if let Ok(val) = env::var("TERM_PROGRAM")
            && matches!(val.as_str(), "WezTerm" | "foot")
        {
            return true;
        }

        false
    }

    pub fn supports_kitty_graphics(&self) -> bool {
        self.kitty_graphics
    }

    pub fn supports_sixel(&self) -> bool {
        self.sixel
    }

    pub fn supports_iterm_images(&self) -> bool {
        self.iterm_images
    }

    pub fn has_any_graphics(&self) -> bool {
        self.kitty_graphics || self.sixel || self.iterm_images
    }
}

impl Default for GraphicsCapabilities {
    fn default() -> Self {
        Self::detect()
    }
}

// === input.rs ===

#[derive(Debug, Clone)]
pub struct InputCapabilities {
    pub kitty_keyboard: bool,
    pub csi_u: bool,
    pub bracketed_paste: bool,
    pub focus_events: bool,
    pub mouse_modes: MouseModes,
}

#[derive(Debug, Clone)]
pub struct MouseModes {
    pub normal_mouse: bool,
    pub button_tracking: bool,
    pub any_event_tracking: bool,
    pub sgr_extended: bool,
    pub urxvt: bool,
    pub kitty_mouse: bool,
}

impl MouseModes {
    pub fn detect() -> Self {
        let is_kitty = env::var("KITTY_WINDOW_ID").is_ok();
        let is_ghostty = env::var("GHOSTTY_RESOURCES_DIR").is_ok();

        Self {
            normal_mouse: true,
            button_tracking: true,
            any_event_tracking: true,
            sgr_extended: true,
            urxvt: !is_kitty && !is_ghostty,
            kitty_mouse: is_kitty,
        }
    }
}

impl Default for MouseModes {
    fn default() -> Self {
        Self::detect()
    }
}

impl InputCapabilities {
    pub fn detect() -> Self {
        let is_kitty = env::var("KITTY_WINDOW_ID").is_ok();
        let is_ghostty = env::var("GHOSTTY_RESOURCES_DIR").is_ok();

        Self {
            kitty_keyboard: is_kitty || is_ghostty,
            csi_u: is_kitty || is_ghostty,
            bracketed_paste: true,
            focus_events: true,
            mouse_modes: MouseModes::detect(),
        }
    }

    pub fn supports_kitty_keyboard(&self) -> bool {
        self.kitty_keyboard
    }

    pub fn supports_csi_u(&self) -> bool {
        self.csi_u
    }

    pub fn supports_bracketed_paste(&self) -> bool {
        self.bracketed_paste
    }

    pub fn supports_focus_events(&self) -> bool {
        self.focus_events
    }
}

impl Default for InputCapabilities {
    fn default() -> Self {
        Self::detect()
    }
}

// === rendering.rs ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorSupport {
    TrueColor,
    Color256,
    Color16,
    Color8,
    Monochrome,
}

impl ColorSupport {
    pub fn detect() -> Self {
        if Self::supports_true_color() {
            Self::TrueColor
        } else if Self::supports_256_colors() {
            Self::Color256
        } else if Self::supports_16_colors() {
            Self::Color16
        } else if Self::supports_8_colors() {
            Self::Color8
        } else {
            Self::Monochrome
        }
    }

    fn supports_true_color() -> bool {
        if let Ok(val) = env::var("COLORTERM")
            && (val == "truecolor" || val == "24bit")
        {
            return true;
        }

        if let Ok(val) = env::var("TERM_PROGRAM") {
            match val.as_str() {
                "iTerm.app" | "WezTerm" | "Ghostty" | "kitty" => return true,
                _ => {}
            }
        }

        if env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
            return true;
        }

        if env::var("KITTY_WINDOW_ID").is_ok() {
            return true;
        }

        false
    }

    fn supports_256_colors() -> bool {
        if let Ok(val) = env::var("TERM")
            && val.contains("256color")
        {
            return true;
        }
        false
    }

    fn supports_16_colors() -> bool {
        if let Ok(val) = env::var("TERM")
            && !val.is_empty()
            && val != "dumb"
        {
            return true;
        }
        false
    }

    fn supports_8_colors() -> bool {
        if let Ok(val) = env::var("TERM")
            && !val.is_empty()
            && val != "dumb"
        {
            return true;
        }
        false
    }

    pub fn max_colors(&self) -> u32 {
        match self {
            Self::TrueColor => 16_777_216,
            Self::Color256 => 256,
            Self::Color16 => 16,
            Self::Color8 => 8,
            Self::Monochrome => 0,
        }
    }

    pub fn supports_rgb(&self) -> bool {
        *self == Self::TrueColor
    }
}

impl Default for ColorSupport {
    fn default() -> Self {
        Self::detect()
    }
}

#[derive(Debug, Clone)]
pub struct RenderCapabilities {
    pub color_support: ColorSupport,
    pub true_color: bool,
    pub rgb: bool,
    pub palette: bool,
}

impl RenderCapabilities {
    pub fn detect() -> Self {
        let color_support = ColorSupport::detect();
        Self {
            color_support,
            true_color: color_support == ColorSupport::TrueColor,
            rgb: color_support.supports_rgb(),
            palette: color_support != ColorSupport::Monochrome,
        }
    }

    pub fn supports_color(&self, color_count: u32) -> bool {
        self.color_support.max_colors() >= color_count
    }
}

impl Default for RenderCapabilities {
    fn default() -> Self {
        Self::detect()
    }
}

// === unicode.rs ===

#[derive(Debug, Clone)]
pub struct UnicodeCapabilities {
    pub unicode_version: UnicodeVersion,
    pub emoji_support: bool,
    pub emoji_width: EmojiWidth,
    pub nerd_font_available: bool,
    pub private_use_area: bool,
    pub cjk_width: CjkWidth,
    pub combining_characters: bool,
    pub zero_width_joiners: bool,
    pub ligatures: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnicodeVersion {
    Unicode8,
    Unicode9,
    Unicode10,
    Unicode11,
    Unicode12,
    Unicode13,
    Unicode14,
    Unicode15,
    Unicode16,
    Unknown,
}

impl UnicodeVersion {
    pub fn detect() -> Self {
        if let Ok(val) = env::var("UNICODE_VERSION") {
            match val.as_str() {
                "8.0.0" | "8" => return Self::Unicode8,
                "9.0.0" | "9" => return Self::Unicode9,
                "10.0.0" | "10" => return Self::Unicode10,
                "11.0.0" | "11" => return Self::Unicode11,
                "12.0.0" | "12" => return Self::Unicode12,
                "13.0.0" | "13" => return Self::Unicode13,
                "14.0.0" | "14" => return Self::Unicode14,
                "15.0.0" | "15" => return Self::Unicode15,
                "16.0.0" | "16" => return Self::Unicode16,
                _ => {}
            }
        }

        if env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
            return Self::Unicode15;
        }

        if env::var("KITTY_WINDOW_ID").is_ok() {
            return Self::Unicode15;
        }

        Self::Unknown
    }

    pub fn version_number(&self) -> f32 {
        match self {
            Self::Unicode8 => 8.0,
            Self::Unicode9 => 9.0,
            Self::Unicode10 => 10.0,
            Self::Unicode11 => 11.0,
            Self::Unicode12 => 12.0,
            Self::Unicode13 => 13.0,
            Self::Unicode14 => 14.0,
            Self::Unicode15 => 15.0,
            Self::Unicode16 => 16.0,
            Self::Unknown => 0.0,
        }
    }
}

impl Default for UnicodeVersion {
    fn default() -> Self {
        Self::detect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EmojiWidth {
    SingleWidth,
    #[default]
    DoubleWidth,
    Variant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CjkWidth {
    #[default]
    FullWidth,
    HalfWidth,
    Ambiguous,
}

impl UnicodeCapabilities {
    pub fn detect() -> Self {
        let unicode_version = UnicodeVersion::detect();
        let emoji_support = Self::detect_emoji_support();
        let nerd_font_available = Self::detect_nerd_font();

        Self {
            unicode_version,
            emoji_support,
            emoji_width: if emoji_support {
                EmojiWidth::DoubleWidth
            } else {
                EmojiWidth::SingleWidth
            },
            nerd_font_available,
            private_use_area: nerd_font_available,
            cjk_width: CjkWidth::FullWidth,
            combining_characters: true,
            zero_width_joiners: true,
            ligatures: Self::detect_ligatures(),
        }
    }

    fn detect_emoji_support() -> bool {
        if env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
            return true;
        }
        if env::var("KITTY_WINDOW_ID").is_ok() {
            return true;
        }
        if let Ok(val) = env::var("TERM_PROGRAM") {
            match val.as_str() {
                "iTerm.app" | "WezTerm" => return true,
                _ => {}
            }
        }
        true
    }

    fn detect_nerd_font() -> bool {
        if let Ok(val) = env::var("NERD_FONT") {
            return val == "1" || val == "true" || val == "yes";
        }

        if let Ok(val) = env::var("FONT")
            && val.to_lowercase().contains("nerd")
        {
            return true;
        }

        if let Ok(val) = env::var("TERM_FONT")
            && val.to_lowercase().contains("nerd")
        {
            return true;
        }

        false
    }

    fn detect_ligatures() -> bool {
        if env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
            return true;
        }
        if env::var("KITTY_WINDOW_ID").is_ok() {
            return true;
        }
        true
    }
}

impl Default for UnicodeCapabilities {
    fn default() -> Self {
        Self::detect()
    }
}

// === window.rs ===

#[derive(Debug, Clone)]
pub struct WindowMetrics {
    pub terminal_width: u16,
    pub terminal_height: u16,
    pub pixel_width: Option<u32>,
    pub pixel_height: Option<u32>,
    pub cell_width: Option<u32>,
    pub cell_height: Option<u32>,
    pub dpi: Option<f64>,
}

impl WindowMetrics {
    pub fn detect() -> Self {
        let (terminal_width, terminal_height) = Self::detect_terminal_size();
        let (pixel_width, pixel_height) = Self::detect_pixel_size();
        let (cell_width, cell_height) = Self::detect_cell_size();
        let dpi = Self::detect_dpi();

        Self {
            terminal_width,
            terminal_height,
            pixel_width,
            pixel_height,
            cell_width,
            cell_height,
            dpi,
        }
    }

    fn detect_terminal_size() -> (u16, u16) {
        if let Ok((w, h)) = crossterm::terminal::size() {
            (w, h)
        } else {
            (80, 24)
        }
    }

    fn detect_pixel_size() -> (Option<u32>, Option<u32>) {
        if let Ok(val) = env::var("WINDOW像素宽度")
            && let Ok(w) = val.parse()
            && let Ok(val) = env::var("WINDOW像素高度")
            && let Ok(h) = val.parse()
        {
            return (Some(w), Some(h));
        }

        if let Ok(val) = env::var("GHOSTTY窗口宽度")
            && let Ok(w) = val.parse()
            && let Ok(val) = env::var("GHOSTTY窗口高度")
            && let Ok(h) = val.parse()
        {
            return (Some(w), Some(h));
        }

        (None, None)
    }

    fn detect_cell_size() -> (Option<u32>, Option<u32>) {
        if let Ok(val) = env::var("GHOSTTY单元宽度")
            && let Ok(w) = val.parse()
            && let Ok(val) = env::var("GHOSTTY单元高度")
            && let Ok(h) = val.parse()
        {
            return (Some(w), Some(h));
        }

        (None, None)
    }

    fn detect_dpi() -> Option<f64> {
        if let Ok(val) = env::var("GHOSTTY DPI")
            && let Ok(dpi) = val.parse()
        {
            return Some(dpi);
        }

        None
    }

    pub fn cell_aspect_ratio(&self) -> Option<f64> {
        if let (Some(w), Some(h)) = (self.cell_width, self.cell_height)
            && h > 0
        {
            return Some(w as f64 / h as f64);
        }
        None
    }

    pub fn pixels_per_cell(&self) -> Option<(u32, u32)> {
        if let (Some(pw), Some(ph)) = (self.pixel_width, self.pixel_height)
            && self.terminal_width > 0
            && self.terminal_height > 0
        {
            let cell_w = pw / self.terminal_width as u32;
            let cell_h = ph / self.terminal_height as u32;
            return Some((cell_w, cell_h));
        }
        None
    }
}

impl Default for WindowMetrics {
    fn default() -> Self {
        Self::detect()
    }
}
