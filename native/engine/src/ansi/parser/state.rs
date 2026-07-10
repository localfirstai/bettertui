use super::csi::CsiCommand;
use super::osc::OscCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserState {
    Ground,
    Escape,
    Csi,
    Osc,
    OscTerminator,
    Dcs,
    DcsTerminator,
    Pm,
    PmTerminator,
    Sos,
    SosTerminator,
    Apc,
    ApcTerminator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserEvent {
    Char(u8),
    Backspace,
    Tab,
    LineFeed,
    CarriageReturn,
    Bell,
    Csi(CsiCommand),
    Osc(OscCommand),
    Dcs(Vec<u8>),
    Pm(Vec<u8>),
    Sos(Vec<u8>),
    Apc(Vec<u8>),
    Index,
    ReverseIndex,
    NextLine,
    Reset,
}

impl ParserEvent {
    pub fn is_printable(&self) -> bool {
        matches!(
            self,
            Self::Char(_) | Self::Tab | Self::LineFeed | Self::CarriageReturn
        )
    }

    pub fn is_cursor_movement(&self) -> bool {
        if let Self::Csi(cmd) = self {
            matches!(
                cmd,
                CsiCommand::CursorMovement(_)
                    | CsiCommand::CursorPositionSave
                    | CsiCommand::CursorPositionRestore
            )
        } else {
            false
        }
    }

    pub fn is_sgr(&self) -> bool {
        matches!(self, Self::Csi(CsiCommand::Sgr(_)))
    }

    pub fn is_erase(&self) -> bool {
        matches!(self, Self::Csi(CsiCommand::Erase(_)))
    }

    pub fn is_scroll(&self) -> bool {
        matches!(self, Self::Csi(CsiCommand::Scroll(_, _)))
    }

    pub fn is_device_status(&self) -> bool {
        matches!(self, Self::Csi(CsiCommand::DeviceStatus(_)))
    }

    pub fn is_mode(&self) -> bool {
        matches!(self, Self::Csi(CsiCommand::Mode(_, _)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_state_ground() {
        assert_ne!(ParserState::Ground, ParserState::Escape);
    }

    #[test]
    fn parser_event_char() {
        let event = ParserEvent::Char(b'A');
        assert!(event.is_printable());
        assert!(!event.is_cursor_movement());
    }

    #[test]
    fn parser_event_csi() {
        let event = ParserEvent::Csi(CsiCommand::CursorMovement(
            super::super::csi::CursorMovement::Up(1),
        ));
        assert!(event.is_cursor_movement());
    }

    #[test]
    fn parser_event_sgr() {
        let event = ParserEvent::Csi(CsiCommand::Sgr(vec![super::super::csi::SgrAttribute::Bold]));
        assert!(event.is_sgr());
    }
}
