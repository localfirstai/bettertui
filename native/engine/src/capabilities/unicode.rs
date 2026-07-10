use std::env;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicode_version_detect() {
        let version = UnicodeVersion::detect();
        assert!(version.version_number() >= 0.0);
    }

    #[test]
    fn unicode_capabilities_detect() {
        let caps = UnicodeCapabilities::detect();
        assert!(caps.unicode_version.version_number() >= 0.0);
    }

    #[test]
    fn emoji_width_default() {
        assert_eq!(EmojiWidth::default(), EmojiWidth::DoubleWidth);
    }

    #[test]
    fn cjk_width_default() {
        assert_eq!(CjkWidth::default(), CjkWidth::FullWidth);
    }
}
