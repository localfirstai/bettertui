//! Terminal query/response protocol implementation.
//! Generates DCS/CSI query sequences and parses terminal responses.

use crate::vt::VtMachine;

/// Query types that can be sent to the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalQuery {
    /// Primary Device Attributes (DA1): `ESC[c` or `ESC[0c`
    /// Response: `ESC[?...c`
    DeviceAttributes,
    /// Secondary Device Attributes (DA2): `ESC[>c` or `ESC[>0c`
    /// Response: `ESC[>...c`
    SecondaryDeviceAttributes,
    /// Tertiary Device Attributes (DA3): `ESC[!{}`
    TertiaryDeviceAttributes,
    /// Report Cursor Position (DSR): `ESC[6n`
    /// Response: `ESC[...;...R`
    CursorPosition,
    /// Request Terminal Name (DECID): `ESC[Z`
    /// Response: terminal identification string
    TerminalId,
    /// Request Terminal Version (XTGETTEXT or similar)
    XTVersion,
    /// Kitty Keyboard Protocol Query: `ESC[?u`
    /// Response: `ESC[?64;Nu` where N is a bitmask of supported features
    ProgressiveEnhancement,
}

impl TerminalQuery {
    /// Returns the raw bytes to send to the terminal for this query.
    pub fn query_bytes(&self) -> &'static [u8] {
        match self {
            Self::DeviceAttributes => b"\x1b[c",
            Self::SecondaryDeviceAttributes => b"\x1b[>c",
            Self::TertiaryDeviceAttributes => b"\x1b[!}",
            Self::CursorPosition => b"\x1b[6n",
            Self::TerminalId => b"\x1b[Z",
            Self::XTVersion => b"\x1b[>0c",
            Self::ProgressiveEnhancement => b"\x1b[?u",
        }
    }

    /// Returns a human-readable name for this query.
    pub fn name(&self) -> &'static str {
        match self {
            Self::DeviceAttributes => "DA1",
            Self::SecondaryDeviceAttributes => "DA2",
            Self::TertiaryDeviceAttributes => "DA3",
            Self::CursorPosition => "DSR-CPR",
            Self::TerminalId => "DECID",
            Self::XTVersion => "XTVersion",
            Self::ProgressiveEnhancement => "KittyProgressive",
        }
    }
}

/// Parsed result from a terminal query response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryResult {
    DeviceAttributes {
        terminal_type: u32,
        attributes: Vec<u32>,
    },
    SecondaryDeviceAttributes {
        model: u32,
        firmware_major: u32,
        firmware_minor: u32,
    },
    TertiaryDeviceAttributes {
        data: String,
    },
    CursorPosition {
        row: u32,
        col: u32,
    },
    ProgressiveEnhancement {
        features: u32,
    },
    Unknown,
}

/// Checks if a VtMachine has received any terminal responses and
/// returns parsed results.
pub fn check_responses(machine: &VtMachine) -> Vec<(TerminalQuery, QueryResult)> {
    let mut results = Vec::new();

    if let Some(params) = &machine.device_attributes {
        let terminal_type = params.first().copied().unwrap_or(0);
        let attributes = params[1..].to_vec();
        results.push((
            TerminalQuery::DeviceAttributes,
            QueryResult::DeviceAttributes {
                terminal_type,
                attributes,
            },
        ));
    }

    if let Some(params) = &machine.secondary_device_attributes {
        let model = params.first().copied().unwrap_or(0);
        let fw_major = params.get(1).copied().unwrap_or(0);
        let fw_minor = params.get(2).copied().unwrap_or(0);
        results.push((
            TerminalQuery::SecondaryDeviceAttributes,
            QueryResult::SecondaryDeviceAttributes {
                model,
                firmware_major: fw_major,
                firmware_minor: fw_minor,
            },
        ));
    }

    if let Some(data) = &machine.tertiary_device_attributes {
        results.push((
            TerminalQuery::TertiaryDeviceAttributes,
            QueryResult::TertiaryDeviceAttributes { data: data.clone() },
        ));
    }

    if let Some(params) = &machine.kitty_keyboard_query_response {
        let features = params.get(1).copied().unwrap_or(0);
        results.push((
            TerminalQuery::ProgressiveEnhancement,
            QueryResult::ProgressiveEnhancement { features },
        ));
    }

    results
}

/// Clears all stored terminal responses on the machine.
pub fn clear_responses(machine: &mut VtMachine) {
    machine.device_attributes = None;
    machine.secondary_device_attributes = None;
    machine.tertiary_device_attributes = None;
    machine.kitty_keyboard_query_response = None;
    machine.last_kitty_key = None;
    machine.terminal_responses.clear();
}

/// Returns all query strings to send for a full capability probe.
pub fn full_probe_queries() -> Vec<TerminalQuery> {
    vec![
        TerminalQuery::DeviceAttributes,
        TerminalQuery::SecondaryDeviceAttributes,
        TerminalQuery::TertiaryDeviceAttributes,
        TerminalQuery::ProgressiveEnhancement,
    ]
}

/// Returns query bytes to enable Kitty keyboard protocol at a given level (1-5).
pub fn kitty_enable_level_bytes(level: u8) -> Vec<u8> {
    let level = level.clamp(1, 5);
    format!("\x1b[?27127;{}h", level).into_bytes()
}

/// Returns query bytes to disable Kitty keyboard protocol at a given level (1-5).
pub fn kitty_disable_level_bytes(level: u8) -> Vec<u8> {
    let level = level.clamp(1, 5);
    format!("\x1b[?27127;{}l", level).into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_names() {
        assert_eq!(TerminalQuery::DeviceAttributes.name(), "DA1");
        assert_eq!(TerminalQuery::SecondaryDeviceAttributes.name(), "DA2");
        assert_eq!(TerminalQuery::TertiaryDeviceAttributes.name(), "DA3");
        assert_eq!(TerminalQuery::CursorPosition.name(), "DSR-CPR");
        assert_eq!(TerminalQuery::TerminalId.name(), "DECID");
        assert_eq!(TerminalQuery::XTVersion.name(), "XTVersion");
        assert_eq!(
            TerminalQuery::ProgressiveEnhancement.name(),
            "KittyProgressive"
        );
    }

    #[test]
    fn query_bytes_non_empty() {
        for q in &[
            TerminalQuery::DeviceAttributes,
            TerminalQuery::SecondaryDeviceAttributes,
            TerminalQuery::TertiaryDeviceAttributes,
            TerminalQuery::CursorPosition,
            TerminalQuery::TerminalId,
            TerminalQuery::XTVersion,
        ] {
            assert!(!q.query_bytes().is_empty(), "{} query is empty", q.name());
        }
    }

    #[test]
    fn full_probe_contains_expected() {
        let probes = full_probe_queries();
        assert!(probes.contains(&TerminalQuery::DeviceAttributes));
        assert!(probes.contains(&TerminalQuery::SecondaryDeviceAttributes));
        assert!(probes.contains(&TerminalQuery::TertiaryDeviceAttributes));
        assert!(probes.contains(&TerminalQuery::ProgressiveEnhancement));
        assert_eq!(probes.len(), 4);
    }

    #[test]
    fn parse_device_attributes_response() {
        let mut machine = VtMachine::new(80, 24);
        let mut parser = bettertui_engine::ansi::AnsiParser::new();
        parser.feed(b"\x1b[?1;2c");
        while let Some(event) = parser.poll_event() {
            machine.process(&event);
        }
        let results = check_responses(&machine);
        assert!(!results.is_empty());
        if let QueryResult::DeviceAttributes {
            terminal_type,
            attributes,
        } = &results[0].1
        {
            assert_eq!(*terminal_type, 1);
            assert_eq!(attributes, &[2]);
        } else {
            panic!("Expected DeviceAttributes result");
        }
    }

    #[test]
    fn parse_secondary_device_attributes_response() {
        let mut machine = VtMachine::new(80, 24);
        let mut parser = bettertui_engine::ansi::AnsiParser::new();
        parser.feed(b"\x1b[>1;10;0c");
        while let Some(event) = parser.poll_event() {
            machine.process(&event);
        }
        let results = check_responses(&machine);
        assert!(!results.is_empty());
        if let QueryResult::SecondaryDeviceAttributes {
            model,
            firmware_major,
            firmware_minor,
        } = &results[0].1
        {
            assert_eq!(*model, 1);
            assert_eq!(*firmware_major, 10);
            assert_eq!(*firmware_minor, 0);
        } else {
            panic!("Expected SecondaryDeviceAttributes result");
        }
    }

    #[test]
    fn no_responses_initially() {
        let machine = VtMachine::new(80, 24);
        let results = check_responses(&machine);
        assert!(results.is_empty());
    }

    #[test]
    fn clear_responses_works() {
        let mut machine = VtMachine::new(80, 24);
        let mut parser = bettertui_engine::ansi::AnsiParser::new();
        parser.feed(b"\x1b[?1;2c");
        while let Some(event) = parser.poll_event() {
            machine.process(&event);
        }
        assert!(!check_responses(&machine).is_empty());
        clear_responses(&mut machine);
        assert!(check_responses(&machine).is_empty());
    }
}
