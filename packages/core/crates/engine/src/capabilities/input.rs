use std::env;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_capabilities_detect() {
        let caps = InputCapabilities::detect();
        assert!(caps.supports_bracketed_paste());
        assert!(caps.supports_focus_events());
    }

    #[test]
    fn mouse_modes_detect() {
        let modes = MouseModes::detect();
        assert!(modes.normal_mouse);
        assert!(modes.button_tracking);
    }
}
