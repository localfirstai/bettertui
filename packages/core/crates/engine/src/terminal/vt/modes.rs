bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TerminalMode: u32 {
        const NONE                = 0;
        const INSERT              = 1 << 0;
        const ORIGIN              = 1 << 1;
        const AUTO_WRAP           = 1 << 2;
        const REVERSE_VIDEO       = 1 << 3;
        const APPLICATION_CURSOR  = 1 << 4;
        const APPLICATION_KEYPAD  = 1 << 5;
        const MOUSE_TRACKING      = 1 << 6;
        const MOUSE_BUTTON        = 1 << 7;
        const MOUSE_MOTION        = 1 << 8;
        const MOUSE_SGR           = 1 << 9;
        const MOUSE_URXVT         = 1 << 10;
        const BRACKETED_PASTE     = 1 << 11;
        const FOCUS_EVENTS        = 1 << 12;
        const ALT_SCREEN          = 1 << 13;
        const SAVE_CURSOR         = 1 << 14;
        const BLINKING_CURSOR     = 1 << 15;
        const VISIBLE_CURSOR      = 1 << 16;
        const COLUMN_132          = 1 << 17;
        const SMOOTH_SCROLL       = 1 << 18;
        const EIGHT_BIT           = 1 << 19;
        const DECCOLM             = 1 << 20;
        const KEYBOARD_PROTOCOL   = 1 << 21;
    }
}

impl Default for TerminalMode {
    fn default() -> Self {
        Self::AUTO_WRAP | Self::VISIBLE_CURSOR | Self::BLINKING_CURSOR
    }
}

impl TerminalMode {
    pub fn is_insert(&self) -> bool {
        self.contains(Self::INSERT)
    }

    pub fn origin(&self) -> bool {
        self.contains(Self::ORIGIN)
    }

    pub fn auto_wrap(&self) -> bool {
        self.contains(Self::AUTO_WRAP)
    }

    pub fn alt_screen(&self) -> bool {
        self.contains(Self::ALT_SCREEN)
    }

    pub fn bracketed_paste(&self) -> bool {
        self.contains(Self::BRACKETED_PASTE)
    }

    pub fn focus_events(&self) -> bool {
        self.contains(Self::FOCUS_EVENTS)
    }

    pub fn cursor_visible(&self) -> bool {
        self.contains(Self::VISIBLE_CURSOR)
    }

    pub fn cursor_blinking(&self) -> bool {
        self.contains(Self::BLINKING_CURSOR)
    }

    pub fn mouse_tracking(&self) -> bool {
        self.contains(Self::MOUSE_TRACKING)
    }

    pub fn keyboard_protocol(&self) -> bool {
        self.contains(Self::KEYBOARD_PROTOCOL)
    }

    pub fn application_cursor(&self) -> bool {
        self.contains(Self::APPLICATION_CURSOR)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivateMode {
    CursorVisible = 25,
    BlinkingCursor = 12,
    Origin = 6,
    AutoWrap = 7,
    ReverseVideo = 5,
    ApplicationCursor = 1,
    BracketedPaste = 2004,
    FocusEvents = 1004,
    MouseTracking = 1000,
    MouseButton = 1002,
    MouseMotion = 1003,
    MouseSgr = 1006,
    MouseUrxvt = 1015,
    AltScreen = 1049,
    SaveCursor = 1048,
    Column132 = 3,
    SmoothScroll = 4,
    KeyboardProtocol = 27127,
}

impl PrivateMode {
    pub fn from_code(code: u32) -> Option<Self> {
        match code {
            1 => Some(Self::ApplicationCursor),
            3 => Some(Self::Column132),
            4 => Some(Self::SmoothScroll),
            5 => Some(Self::ReverseVideo),
            6 => Some(Self::Origin),
            7 => Some(Self::AutoWrap),
            12 => Some(Self::BlinkingCursor),
            25 => Some(Self::CursorVisible),
            1000 => Some(Self::MouseTracking),
            1002 => Some(Self::MouseButton),
            1003 => Some(Self::MouseMotion),
            1004 => Some(Self::FocusEvents),
            1006 => Some(Self::MouseSgr),
            1015 => Some(Self::MouseUrxvt),
            1048 => Some(Self::SaveCursor),
            1049 => Some(Self::AltScreen),
            2004 => Some(Self::BracketedPaste),
            27127 => Some(Self::KeyboardProtocol),
            _ => None,
        }
    }

    pub fn to_terminal_mode(self) -> TerminalMode {
        match self {
            Self::CursorVisible => TerminalMode::VISIBLE_CURSOR,
            Self::BlinkingCursor => TerminalMode::BLINKING_CURSOR,
            Self::Origin => TerminalMode::ORIGIN,
            Self::AutoWrap => TerminalMode::AUTO_WRAP,
            Self::ReverseVideo => TerminalMode::REVERSE_VIDEO,
            Self::ApplicationCursor => TerminalMode::APPLICATION_CURSOR,
            Self::BracketedPaste => TerminalMode::BRACKETED_PASTE,
            Self::FocusEvents => TerminalMode::FOCUS_EVENTS,
            Self::MouseTracking => TerminalMode::MOUSE_TRACKING,
            Self::MouseButton => TerminalMode::MOUSE_BUTTON,
            Self::MouseMotion => TerminalMode::MOUSE_MOTION,
            Self::MouseSgr => TerminalMode::MOUSE_SGR,
            Self::MouseUrxvt => TerminalMode::MOUSE_URXVT,
            Self::AltScreen => TerminalMode::ALT_SCREEN,
            Self::SaveCursor => TerminalMode::SAVE_CURSOR,
            Self::Column132 => TerminalMode::COLUMN_132,
            Self::SmoothScroll => TerminalMode::SMOOTH_SCROLL,
            Self::KeyboardProtocol => TerminalMode::KEYBOARD_PROTOCOL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_modes() {
        let m = TerminalMode::default();
        assert!(m.auto_wrap());
        assert!(m.cursor_visible());
        assert!(m.cursor_blinking());
        assert!(!m.alt_screen());
        assert!(!m.bracketed_paste());
    }

    #[test]
    fn mode_set_reset() {
        let mut m = TerminalMode::default();
        m.insert(TerminalMode::BRACKETED_PASTE);
        assert!(m.bracketed_paste());
        m.remove(TerminalMode::BRACKETED_PASTE);
        assert!(!m.bracketed_paste());
    }

    #[test]
    fn mode_toggle() {
        let mut m = TerminalMode::default();
        m.insert(TerminalMode::INSERT);
        assert!(m.is_insert());
        m.remove(TerminalMode::INSERT);
        assert!(!m.is_insert());
    }

    #[test]
    fn private_mode_from_code() {
        assert_eq!(PrivateMode::from_code(25), Some(PrivateMode::CursorVisible));
        assert_eq!(
            PrivateMode::from_code(2004),
            Some(PrivateMode::BracketedPaste)
        );
        assert_eq!(PrivateMode::from_code(9999), None);
    }

    #[test]
    fn private_mode_to_terminal() {
        let tm = PrivateMode::CursorVisible.to_terminal_mode();
        assert_eq!(tm, TerminalMode::VISIBLE_CURSOR);
    }
}
