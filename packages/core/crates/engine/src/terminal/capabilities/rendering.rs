use std::env;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_support_detect() {
        let support = ColorSupport::detect();
        let _ = support;
    }

    #[test]
    fn color_support_max_colors() {
        assert_eq!(ColorSupport::TrueColor.max_colors(), 16_777_216);
        assert_eq!(ColorSupport::Color256.max_colors(), 256);
        assert_eq!(ColorSupport::Color16.max_colors(), 16);
        assert_eq!(ColorSupport::Color8.max_colors(), 8);
        assert_eq!(ColorSupport::Monochrome.max_colors(), 0);
    }

    #[test]
    fn render_capabilities_detect() {
        let caps = RenderCapabilities::detect();
        let _ = caps;
    }
}
