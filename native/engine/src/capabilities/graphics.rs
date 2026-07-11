use std::env;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graphics_capabilities_detect() {
        let caps = GraphicsCapabilities::detect();
        let _ = caps;
    }

    #[test]
    fn graphics_has_any() {
        let caps = GraphicsCapabilities::detect();
        let _ = caps.has_any_graphics();
    }
}
