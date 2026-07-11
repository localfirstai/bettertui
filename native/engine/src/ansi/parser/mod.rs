mod csi;
mod osc;
mod sgr;
mod state;

pub use csi::{
    BackgroundColor, CsiCommand, CursorMovement, DeviceStatus, EraseMode, ForegroundColor,
    KittyEventType, ModeAction, ModeType, ScrollDirection, TabStopAction,
};
pub use osc::{ClipboardData, Hyperlink, OscCommand};
pub use sgr::{SgrAttribute, SgrState};
pub use state::{ParserEvent, ParserState};

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct AnsiParser {
    state: ParserState,
    params: Vec<u32>,
    intermediate: Vec<u8>,
    buffer: Vec<u8>,
    events: VecDeque<ParserEvent>,
}

impl Default for AnsiParser {
    fn default() -> Self {
        Self::new()
    }
}

impl AnsiParser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Ground,
            params: Vec::new(),
            intermediate: Vec::new(),
            buffer: Vec::with_capacity(256),
            events: VecDeque::new(),
        }
    }

    pub fn feed(&mut self, data: &[u8]) {
        for &byte in data {
            self.process_byte(byte);
        }
    }

    pub fn poll_event(&mut self) -> Option<ParserEvent> {
        self.events.pop_front()
    }

    pub fn events(&self) -> &VecDeque<ParserEvent> {
        &self.events
    }

    fn process_byte(&mut self, byte: u8) {
        match self.state {
            ParserState::Ground => {
                if byte == 0x1b {
                    self.state = ParserState::Escape;
                } else if byte == 0x08 {
                    self.events.push_back(ParserEvent::Backspace);
                } else if byte == 0x09 {
                    self.events.push_back(ParserEvent::Tab);
                } else if byte == 0x0a {
                    self.events.push_back(ParserEvent::LineFeed);
                } else if byte == 0x0d {
                    self.events.push_back(ParserEvent::CarriageReturn);
                } else if byte == 0x07 {
                    self.events.push_back(ParserEvent::Bell);
                } else {
                    self.events.push_back(ParserEvent::Char(byte));
                }
            }
            ParserState::Escape => {
                if byte == b'[' {
                    self.state = ParserState::Csi;
                    self.params.clear();
                    self.intermediate.clear();
                } else if byte == b']' {
                    self.state = ParserState::Osc;
                    self.buffer.clear();
                } else if byte == b'P' {
                    self.state = ParserState::Dcs;
                    self.buffer.clear();
                } else if byte == b'^' {
                    self.state = ParserState::Pm;
                    self.buffer.clear();
                } else if byte == b'_' {
                    self.state = ParserState::Sos;
                    self.buffer.clear();
                } else if byte == b'M' {
                    self.events.push_back(ParserEvent::ReverseIndex);
                    self.state = ParserState::Ground;
                } else if byte == b'D' {
                    self.events.push_back(ParserEvent::Index);
                    self.state = ParserState::Ground;
                } else if byte == b'E' {
                    self.events.push_back(ParserEvent::NextLine);
                    self.state = ParserState::Ground;
                } else if byte == b'c' {
                    self.events.push_back(ParserEvent::Reset);
                    self.state = ParserState::Ground;
                } else {
                    self.state = ParserState::Ground;
                }
            }
            ParserState::Csi => {
                if byte == b';' {
                    self.params.push(0);
                } else if (0x30..=0x39).contains(&byte) {
                    if let Some(last) = self.params.last_mut() {
                        *last = *last * 10 + (byte - 0x30) as u32;
                    } else {
                        self.params.push((byte - 0x30) as u32);
                    }
                } else if (0x3c..=0x3f).contains(&byte) || (0x20..=0x2f).contains(&byte) {
                    self.intermediate.push(byte);
                } else if (0x40..=0x7e).contains(&byte) {
                    self.process_csi(byte);
                    self.state = ParserState::Ground;
                }
            }
            ParserState::Osc => {
                if byte == 0x07 {
                    self.process_osc();
                    self.state = ParserState::Ground;
                } else if byte == 0x1b {
                    self.state = ParserState::OscTerminator;
                } else {
                    self.buffer.push(byte);
                }
            }
            ParserState::OscTerminator => {
                if byte == b'\\' {
                    self.process_osc();
                    self.state = ParserState::Ground;
                } else {
                    self.buffer.push(0x1b);
                    self.buffer.push(byte);
                    self.state = ParserState::Osc;
                }
            }
            ParserState::Dcs => {
                if byte == 0x1b {
                    self.state = ParserState::DcsTerminator;
                } else {
                    self.buffer.push(byte);
                }
            }
            ParserState::DcsTerminator => {
                if byte == b'\\' {
                    self.events
                        .push_back(ParserEvent::Dcs(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else {
                    self.buffer.push(0x1b);
                    self.buffer.push(byte);
                    self.state = ParserState::Dcs;
                }
            }
            ParserState::Pm => {
                if byte == 0x07 {
                    self.events
                        .push_back(ParserEvent::Pm(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else if byte == 0x1b {
                    self.state = ParserState::PmTerminator;
                } else {
                    self.buffer.push(byte);
                }
            }
            ParserState::PmTerminator => {
                if byte == b'\\' {
                    self.events
                        .push_back(ParserEvent::Pm(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else {
                    self.buffer.push(0x1b);
                    self.buffer.push(byte);
                    self.state = ParserState::Pm;
                }
            }
            ParserState::Sos => {
                if byte == 0x07 {
                    self.events
                        .push_back(ParserEvent::Sos(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else if byte == 0x1b {
                    self.state = ParserState::SosTerminator;
                } else {
                    self.buffer.push(byte);
                }
            }
            ParserState::SosTerminator => {
                if byte == b'\\' {
                    self.events
                        .push_back(ParserEvent::Sos(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else {
                    self.buffer.push(0x1b);
                    self.buffer.push(byte);
                    self.state = ParserState::Sos;
                }
            }
            ParserState::Apc => {
                if byte == 0x07 {
                    self.events
                        .push_back(ParserEvent::Apc(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else if byte == 0x1b {
                    self.state = ParserState::ApcTerminator;
                } else {
                    self.buffer.push(byte);
                }
            }
            ParserState::ApcTerminator => {
                if byte == b'\\' {
                    self.events
                        .push_back(ParserEvent::Apc(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else {
                    self.buffer.push(0x1b);
                    self.buffer.push(byte);
                    self.state = ParserState::Apc;
                }
            }
        }
    }

    fn process_csi(&mut self, final_byte: u8) {
        let command = CsiCommand::parse(final_byte, &self.params, &self.intermediate);
        if let Some(cmd) = command {
            self.events.push_back(ParserEvent::Csi(cmd));
        }
    }

    fn process_osc(&mut self) {
        let command = OscCommand::parse(&self.buffer);
        if let Some(cmd) = command {
            self.events.push_back(ParserEvent::Osc(cmd));
        }
    }

    pub fn reset(&mut self) {
        self.state = ParserState::Ground;
        self.params.clear();
        self.intermediate.clear();
        self.buffer.clear();
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_new() {
        let parser = AnsiParser::new();
        assert!(parser.events().is_empty());
    }

    #[test]
    fn parser_default() {
        let parser = AnsiParser::default();
        assert!(parser.events().is_empty());
    }

    #[test]
    fn parser_feed_text() {
        let mut parser = AnsiParser::new();
        parser.feed(b"hello");
        assert_eq!(parser.events().len(), 5);
    }

    #[test]
    fn parser_feed_escape() {
        let mut parser = AnsiParser::new();
        parser.feed(b"\x1b[A");
        assert_eq!(parser.events().len(), 1);
    }

    #[test]
    fn parser_feed_csi() {
        let mut parser = AnsiParser::new();
        parser.feed(b"\x1b[1;2H");
        assert_eq!(parser.events().len(), 1);
    }

    #[test]
    fn parser_reset() {
        let mut parser = AnsiParser::new();
        parser.feed(b"hello");
        parser.reset();
        assert!(parser.events().is_empty());
    }

    #[test]
    fn parser_poll_event() {
        let mut parser = AnsiParser::new();
        parser.feed(b"ab");
        assert!(parser.poll_event().is_some());
        assert!(parser.poll_event().is_some());
        assert!(parser.poll_event().is_none());
    }
}
