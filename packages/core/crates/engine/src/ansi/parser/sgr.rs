pub use super::csi::{BackgroundColor, ForegroundColor, SgrAttribute, UnderlineColor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SgrState {
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub inverse: bool,
    pub hidden: bool,
    pub strikethrough: bool,
    pub foreground: ForegroundColor,
    pub background: BackgroundColor,
    pub underline_color: UnderlineColor,
}

impl Default for SgrState {
    fn default() -> Self {
        Self::new()
    }
}

impl SgrState {
    pub fn new() -> Self {
        Self {
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            inverse: false,
            hidden: false,
            strikethrough: false,
            foreground: ForegroundColor::Default,
            background: BackgroundColor::Default,
            underline_color: UnderlineColor::Default,
        }
    }

    pub fn apply(&mut self, attrs: &[SgrAttribute]) {
        for attr in attrs {
            match attr {
                SgrAttribute::Reset => *self = Self::new(),
                SgrAttribute::Bold => self.bold = true,
                SgrAttribute::Dim => self.dim = true,
                SgrAttribute::Italic => self.italic = true,
                SgrAttribute::Underline => self.underline = true,
                SgrAttribute::Blink => self.blink = true,
                SgrAttribute::Inverse => self.inverse = true,
                SgrAttribute::Hidden => self.hidden = true,
                SgrAttribute::Strikethrough => self.strikethrough = true,
                SgrAttribute::Foreground(color) => self.foreground = *color,
                SgrAttribute::Background(color) => self.background = *color,
                SgrAttribute::UnderlineColor(color) => self.underline_color = *color,
            }
        }
    }

    pub fn is_plain(&self) -> bool {
        !self.bold
            && !self.dim
            && !self.italic
            && !self.underline
            && !self.blink
            && !self.inverse
            && !self.hidden
            && !self.strikethrough
            && self.foreground == ForegroundColor::Default
            && self.background == BackgroundColor::Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sgr_state_new() {
        let state = SgrState::new();
        assert!(state.is_plain());
    }

    #[test]
    fn sgr_state_default() {
        let state = SgrState::default();
        assert!(state.is_plain());
    }

    #[test]
    fn sgr_state_apply_bold() {
        let mut state = SgrState::new();
        state.apply(&[SgrAttribute::Bold]);
        assert!(state.bold);
        assert!(!state.is_plain());
    }

    #[test]
    fn sgr_state_apply_reset() {
        let mut state = SgrState::new();
        state.apply(&[SgrAttribute::Bold, SgrAttribute::Italic]);
        assert!(!state.is_plain());
        state.apply(&[SgrAttribute::Reset]);
        assert!(state.is_plain());
    }

    #[test]
    fn sgr_state_apply_foreground() {
        let mut state = SgrState::new();
        state.apply(&[SgrAttribute::Foreground(ForegroundColor::Red)]);
        assert_eq!(state.foreground, ForegroundColor::Red);
    }

    #[test]
    fn sgr_state_apply_background() {
        let mut state = SgrState::new();
        state.apply(&[SgrAttribute::Background(BackgroundColor::Blue)]);
        assert_eq!(state.background, BackgroundColor::Blue);
    }
}
