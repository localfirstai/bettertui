use std::env;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brand_detect_returns_value() {
        let brand = TerminalBrand::detect();
        assert!(brand.name().len() > 0);
    }

    #[test]
    fn brand_default_is_unknown() {
        assert_eq!(TerminalBrand::default(), TerminalBrand::Unknown);
    }

    #[test]
    fn brand_name_consistent() {
        for brand in [
            TerminalBrand::Ghostty,
            TerminalBrand::Kitty,
            TerminalBrand::WezTerm,
            TerminalBrand::Alacritty,
            TerminalBrand::Foot,
            TerminalBrand::ITerm2,
            TerminalBrand::WindowsTerminal,
            TerminalBrand::VSCodeTerminal,
            TerminalBrand::Tmux,
            TerminalBrand::GnuScreen,
            TerminalBrand::Warp,
            TerminalBrand::Unknown,
        ] {
            assert!(!brand.name().is_empty());
        }
    }

    #[test]
    fn brand_is_known() {
        assert!(!TerminalBrand::Unknown.is_known());
        assert!(TerminalBrand::Ghostty.is_known());
    }
}
