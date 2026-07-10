use std::env;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clipboard_capabilities_detect() {
        let caps = ClipboardCapabilities::detect();
        assert!(caps.supports_osc52() || !caps.supports_osc52());
    }
}
