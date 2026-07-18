//! ANSI escape sequence encoding and parsing (CSI, OSC, SGR).

use std::collections::VecDeque;

use crate::dirty_diff::DirtyRegion;
use crate::framebuffer::{Cell, CellAttributes, FrameBuffer};
use crate::tree::{Color, NamedColor};
use tracing::trace;

// === encoder.rs ===

pub struct AnsiEncoder {
    buffer: Vec<u8>,
}

impl Default for AnsiEncoder {
    fn default() -> Self {
        Self::new()
    }
}

pub const SYNC_SET: &[u8] = b"\x1b[?2026h";
pub const SYNC_RESET: &[u8] = b"\x1b[?2026l";

impl AnsiEncoder {
    pub fn new() -> Self {
        Self { buffer: Vec::with_capacity(4096) }
    }

    pub fn encode(&mut self, buffer: &FrameBuffer, regions: &[DirtyRegion]) {
        trace!(
            region_count = regions.len(),
            buffer_width = buffer.width(),
            buffer_height = buffer.height(),
            "AnsiEncoder::encode() - encoding frame with DECSET 2026 sync mode"
        );
        self.buffer.clear();
        self.buffer.extend_from_slice(SYNC_SET);
        self.hide_cursor();

        for region in regions {
            self.encode_region(buffer, region);
        }

        self.show_cursor();
        self.buffer.extend_from_slice(SYNC_RESET);
    }

    pub fn encode_region(&mut self, buffer: &FrameBuffer, region: &DirtyRegion) {
        for y in region.y..region.y + region.height {
            self.move_to(region.x, y);
            let mut last_fg: Option<Color> = None;
            let mut last_bg: Option<Color> = None;
            let mut last_attrs: Option<CellAttributes> = None;

            for x in region.x..region.x + region.width {
                let cell = buffer.get(x, y);
                self.encode_cell(&cell, &mut last_fg, &mut last_bg, &mut last_attrs);
            }
        }
    }

    pub fn encode_cell(
        &mut self,
        cell: &Cell,
        last_fg: &mut Option<Color>,
        last_bg: &mut Option<Color>,
        last_attrs: &mut Option<CellAttributes>,
    ) {
        let fg_changed = *last_fg != Some(cell.fg);
        let bg_changed = *last_bg != Some(cell.bg);
        let attrs_changed = *last_attrs != Some(cell.attributes);

        if fg_changed || bg_changed || attrs_changed {
            self.begin_sgr();
            if fg_changed {
                self.push_fg_sgr(cell.fg);
            }
            if bg_changed {
                self.push_bg_sgr(cell.bg);
            }
            if attrs_changed {
                self.push_attrs_sgr(cell.attributes);
            }
            self.end_sgr();
        }

        self.push_char(cell.ch);

        *last_fg = Some(cell.fg);
        *last_bg = Some(cell.bg);
        *last_attrs = Some(cell.attributes);
    }

    pub fn begin_sgr(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[");
    }

    pub fn end_sgr(&mut self) {
        self.buffer.push(b'm');
    }

    fn push_fg_sgr(&mut self, color: Color) {
        match color {
            Color::Default => self.push_param(39),
            Color::Named(named) => {
                let code = match named {
                    NamedColor::Black => 30,
                    NamedColor::Red => 31,
                    NamedColor::Green => 32,
                    NamedColor::Yellow => 33,
                    NamedColor::Blue => 34,
                    NamedColor::Magenta => 35,
                    NamedColor::Cyan => 36,
                    NamedColor::White => 37,
                    NamedColor::BrightBlack => 90,
                    NamedColor::BrightRed => 91,
                    NamedColor::BrightGreen => 92,
                    NamedColor::BrightYellow => 93,
                    NamedColor::BrightBlue => 94,
                    NamedColor::BrightMagenta => 95,
                    NamedColor::BrightCyan => 96,
                    NamedColor::BrightWhite => 97,
                };
                self.push_param(code);
            }
            Color::Rgb { r, g, b } => {
                self.push_param(38);
                self.push_param(2);
                self.push_param(r as u32);
                self.push_param(g as u32);
                self.push_param(b as u32);
            }
            Color::Indexed(i) => {
                self.push_param(38);
                self.push_param(5);
                self.push_param(i as u32);
            }
        }
    }

    fn push_bg_sgr(&mut self, color: Color) {
        match color {
            Color::Default => self.push_param(49),
            Color::Named(named) => {
                let code = match named {
                    NamedColor::Black => 40,
                    NamedColor::Red => 41,
                    NamedColor::Green => 42,
                    NamedColor::Yellow => 43,
                    NamedColor::Blue => 44,
                    NamedColor::Magenta => 45,
                    NamedColor::Cyan => 46,
                    NamedColor::White => 47,
                    NamedColor::BrightBlack => 100,
                    NamedColor::BrightRed => 101,
                    NamedColor::BrightGreen => 102,
                    NamedColor::BrightYellow => 103,
                    NamedColor::BrightBlue => 104,
                    NamedColor::BrightMagenta => 105,
                    NamedColor::BrightCyan => 106,
                    NamedColor::BrightWhite => 107,
                };
                self.push_param(code);
            }
            Color::Rgb { r, g, b } => {
                self.push_param(48);
                self.push_param(2);
                self.push_param(r as u32);
                self.push_param(g as u32);
                self.push_param(b as u32);
            }
            Color::Indexed(i) => {
                self.push_param(48);
                self.push_param(5);
                self.push_param(i as u32);
            }
        }
    }

    fn push_attrs_sgr(&mut self, attrs: CellAttributes) {
        if attrs.contains(CellAttributes::BOLD) {
            self.push_param(1);
        }
        if attrs.contains(CellAttributes::DIM) {
            self.push_param(2);
        }
        if attrs.contains(CellAttributes::ITALIC) {
            self.push_param(3);
        }
        if attrs.contains(CellAttributes::UNDERLINE) {
            self.push_param(4);
        }
        if attrs.contains(CellAttributes::STRIKETHROUGH) {
            self.push_param(9);
        }
        if attrs.contains(CellAttributes::INVERSE) {
            self.push_param(7);
        }
        if attrs.contains(CellAttributes::HIDDEN) {
            self.push_param(8);
        }
    }

    pub fn push_param(&mut self, n: u32) {
        if !self.buffer.ends_with(b"[") && !self.buffer.ends_with(b";") {
            self.buffer.push(b';');
        }
        let mut buf = [0u8; 10];
        let mut i = buf.len();
        let mut val = n;
        if val == 0 {
            i -= 1;
            buf[i] = b'0';
        } else {
            while val > 0 {
                i -= 1;
                buf[i] = b'0' + (val % 10) as u8;
                val /= 10;
            }
        }
        self.buffer.extend_from_slice(&buf[i..]);
    }

    pub fn push_char(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        let s = ch.encode_utf8(&mut buf);
        self.buffer.extend_from_slice(s.as_bytes());
    }

    pub fn move_to(&mut self, x: u16, y: u16) {
        self.buffer.extend_from_slice(b"\x1b[");
        let mut buf = [0u8; 10];
        let mut i = buf.len();
        let mut val = (y + 1) as u32;
        if val == 0 {
            i -= 1;
            buf[i] = b'0';
        } else {
            while val > 0 {
                i -= 1;
                buf[i] = b'0' + (val % 10) as u8;
                val /= 10;
            }
        }
        self.buffer.extend_from_slice(&buf[i..]);
        self.buffer.push(b';');
        let mut i = buf.len();
        let mut val = (x + 1) as u32;
        if val == 0 {
            i -= 1;
            buf[i] = b'0';
        } else {
            while val > 0 {
                i -= 1;
                buf[i] = b'0' + (val % 10) as u8;
                val /= 10;
            }
        }
        self.buffer.extend_from_slice(&buf[i..]);
        self.buffer.push(b'H');
    }

    pub fn hide_cursor(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?25l");
    }

    pub fn show_cursor(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?25h");
    }

    pub fn finish(&self) -> &[u8] {
        &self.buffer
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }

    pub fn reset_sgr(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[0m");
    }
}

// === palette.rs ===

/// A command that can appear in the palette.
#[derive(Debug, Clone)]
pub struct PaletteCommand {
    /// The command label (displayed to user).
    pub label: String,
    /// Optional description.
    pub description: String,
    /// Command category for filtering.
    pub category: String,
    /// Keyboard shortcut hint.
    pub shortcut: Option<String>,
    /// Whether this command is currently enabled.
    pub enabled: bool,
}

impl PaletteCommand {
    /// Creates a new palette command.
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), description: String::new(), category: String::new(), shortcut: None, enabled: true }
    }

    /// Adds a description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Adds a category.
    pub fn with_category(mut self, cat: impl Into<String>) -> Self {
        self.category = cat.into();
        self
    }

    /// Adds a shortcut.
    pub fn with_shortcut(mut self, sc: impl Into<String>) -> Self {
        self.shortcut = Some(sc.into());
        self
    }
}

/// A scored search result.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// The matched command.
    pub command: PaletteCommand,
    /// Match score (higher is better).
    pub score: i64,
    /// Matched character indices in the label.
    pub matches: Vec<usize>,
}

/// Fuzzy matching score calculation.
pub fn fuzzy_score(query: &str, target: &str) -> Option<(i64, Vec<usize>)> {
    if query.is_empty() {
        return Some((0, vec![]));
    }

    let query_lower: Vec<char> = query.chars().map(|c| c.to_ascii_lowercase()).collect();
    let target_lower: Vec<char> = target.chars().map(|c| c.to_ascii_lowercase()).collect();

    let mut matches = Vec::new();
    let mut qi = 0;

    for (ti, &tc) in target_lower.iter().enumerate() {
        if qi < query_lower.len() && tc == query_lower[qi] {
            matches.push(ti);
            qi += 1;
        }
    }

    if qi < query_lower.len() {
        return None; // Not all query chars matched
    }

    // Score: prefer matches at start, consecutive matches, shorter targets
    let mut score = 0i64;

    // Bonus for matches at word boundaries
    for &mi in &matches {
        if mi == 0 || target_lower[mi - 1] == ' ' || target_lower[mi - 1] == '_' || target_lower[mi - 1] == '-' {
            score += 10;
        }
    }

    // Bonus for consecutive matches
    for window in matches.windows(2) {
        if window[1] == window[0] + 1 {
            score += 5;
        }
    }

    // Bonus for exact match
    if matches.len() == target_lower.len() {
        score += 20;
    }

    // Penalty for longer targets
    score -= target_lower.len() as i64 / 2;

    // Bonus for query length match ratio
    score += (matches.len() as i64 * 10) / query_lower.len().max(1) as i64;

    Some((score, matches))
}

/// The command palette providing fuzzy search and navigation.
#[derive(Debug)]
pub struct CommandPalette {
    /// All available commands.
    commands: Vec<PaletteCommand>,
    /// Current search query.
    query: String,
    /// Current selection index in filtered results.
    selected: usize,
    /// Cached search results.
    results: Vec<SearchResult>,
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandPalette {
    /// Creates a new empty palette.
    pub fn new() -> Self {
        Self { commands: Vec::new(), query: String::new(), selected: 0, results: Vec::new() }
    }

    /// Adds a command to the palette.
    pub fn add(&mut self, command: PaletteCommand) {
        self.commands.push(command);
        self.update_results();
    }

    /// Removes all commands.
    pub fn clear(&mut self) {
        self.commands.clear();
        self.results.clear();
        self.query.clear();
        self.selected = 0;
    }

    /// Sets the search query and updates results.
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.selected = 0;
        self.update_results();
    }

    /// Returns the current query.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Returns the current search results.
    pub fn results(&self) -> &[SearchResult] {
        &self.results
    }

    /// Returns mutable access to commands.
    pub fn commands_mut(&mut self) -> &mut Vec<PaletteCommand> {
        &mut self.commands
    }

    /// Returns the currently selected result.
    pub fn selected(&self) -> Option<&SearchResult> {
        self.results.get(self.selected)
    }

    /// Returns the selected index.
    pub fn selected_index(&self) -> usize {
        self.selected
    }

    /// Moves selection up.
    pub fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Moves selection down.
    pub fn select_next(&mut self) {
        if self.selected + 1 < self.results.len() {
            self.selected += 1;
        }
    }

    /// Selects the first result.
    pub fn select_first(&mut self) {
        self.selected = 0;
    }

    /// Selects the last result.
    pub fn select_last(&mut self) {
        if !self.results.is_empty() {
            self.selected = self.results.len() - 1;
        }
    }

    /// Returns the number of available commands.
    pub fn command_count(&self) -> usize {
        self.commands.len()
    }

    /// Returns the number of search results.
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    fn update_results(&mut self) {
        if self.query.is_empty() {
            self.results = self
                .commands
                .iter()
                .filter(|c| c.enabled)
                .cloned()
                .map(|c| SearchResult { command: c, score: 0, matches: vec![] })
                .collect();
        } else {
            let mut scored: Vec<SearchResult> = self
                .commands
                .iter()
                .filter(|c| c.enabled)
                .filter_map(|c| {
                    fuzzy_score(&self.query, &c.label).map(|(score, matches)| SearchResult {
                        command: c.clone(),
                        score,
                        matches,
                    })
                })
                .collect();
            scored.sort_by_key(|r| std::cmp::Reverse(r.score));
            self.results = scored;
        }
        if self.selected >= self.results.len() {
            self.selected = self.results.len().saturating_sub(1);
        }
    }
}

// === parser/mod.rs ===

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
                if byte == b';' || byte == b':' {
                    // Both semicolon (parameter) and colon (sub-parameter, e.g.
                    // Kitty's `modifiers:event_type`) separators start a new param
                    // slot. Colon is flattened into the param list; handlers that
                    // care about sub-params (CSI-u) read the extra slot positionally.
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
                    self.events.push_back(ParserEvent::Dcs(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else {
                    self.buffer.push(0x1b);
                    self.buffer.push(byte);
                    self.state = ParserState::Dcs;
                }
            }
            ParserState::Pm => {
                if byte == 0x07 {
                    self.events.push_back(ParserEvent::Pm(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else if byte == 0x1b {
                    self.state = ParserState::PmTerminator;
                } else {
                    self.buffer.push(byte);
                }
            }
            ParserState::PmTerminator => {
                if byte == b'\\' {
                    self.events.push_back(ParserEvent::Pm(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else {
                    self.buffer.push(0x1b);
                    self.buffer.push(byte);
                    self.state = ParserState::Pm;
                }
            }
            ParserState::Sos => {
                if byte == 0x07 {
                    self.events.push_back(ParserEvent::Sos(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else if byte == 0x1b {
                    self.state = ParserState::SosTerminator;
                } else {
                    self.buffer.push(byte);
                }
            }
            ParserState::SosTerminator => {
                if byte == b'\\' {
                    self.events.push_back(ParserEvent::Sos(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else {
                    self.buffer.push(0x1b);
                    self.buffer.push(byte);
                    self.state = ParserState::Sos;
                }
            }
            ParserState::Apc => {
                if byte == 0x07 {
                    self.events.push_back(ParserEvent::Apc(std::mem::take(&mut self.buffer)));
                    self.state = ParserState::Ground;
                } else if byte == 0x1b {
                    self.state = ParserState::ApcTerminator;
                } else {
                    self.buffer.push(byte);
                }
            }
            ParserState::ApcTerminator => {
                if byte == b'\\' {
                    self.events.push_back(ParserEvent::Apc(std::mem::take(&mut self.buffer)));
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

// === parser/state.rs ===

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
        matches!(self, Self::Char(_) | Self::Tab | Self::LineFeed | Self::CarriageReturn)
    }

    pub fn is_cursor_movement(&self) -> bool {
        if let Self::Csi(cmd) = self {
            matches!(
                cmd,
                CsiCommand::CursorMovement(_) | CsiCommand::CursorPositionSave | CsiCommand::CursorPositionRestore
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

// === parser/csi.rs ===

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CsiCommand {
    CursorMovement(CursorMovement),
    Erase(EraseMode),
    Scroll(ScrollDirection, u32),
    Sgr(Vec<SgrAttribute>),
    DeviceStatus(DeviceStatus),
    Mode(ModeAction, ModeType),
    TabStop(TabStopAction),
    DeleteLine(u32),
    InsertLine(u32),
    DeleteChar(u32),
    InsertChar(u32),
    EraseChar(u32),
    CursorPositionSave,
    CursorPositionRestore,
    AttributeReset,
    DeviceAttributes(Vec<u32>),
    SecondaryDeviceAttributes(Vec<u32>),
    TertiaryDeviceAttributes(String),
    /// Cursor Position Report response: `ESC[<row>;<col>R`.
    CursorPositionReport {
        row: u32,
        col: u32,
    },
    KittyKeyEvent {
        keycode: u32,
        modifiers: u32,
        event_type: KittyEventType,
        associated_text: Option<String>,
    },
    KittyEnhancementLevel {
        level: u8,
        action: ModeAction,
    },
    KittyKeyboardQuery(Vec<u32>),
    Unknown(u8, Vec<u32>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KittyEventType {
    Press,
    Repeat,
    Release,
    Unknown,
}

impl KittyEventType {
    pub fn from_flag(flag: u32) -> Self {
        match flag {
            1 => Self::Press,
            2 => Self::Repeat,
            3 => Self::Release,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorMovement {
    Up(u32),
    Down(u32),
    Forward(u32),
    Backward(u32),
    NextLine(u32),
    PreviousLine(u32),
    ColumnAbsolute(u32),
    Position(u32, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EraseMode {
    CursorToEnd,
    CursorToBeginning,
    Entire,
    CursorToEndLines,
    CursorToBeginningLines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceStatus {
    ReportCursorPosition,
    DeviceAttributes,
    DeviceAttributes2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeAction {
    Set,
    Reset,
    Save,
    Restore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeType {
    Normal(u32),
    Private(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabStopAction {
    Set,
    Clear,
    ClearAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SgrAttribute {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Inverse,
    Hidden,
    Strikethrough,
    Foreground(ForegroundColor),
    Background(BackgroundColor),
    UnderlineColor(UnderlineColor),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForegroundColor {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Extended(u8),
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundColor {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Extended(u8),
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnderlineColor {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Extended(u8),
    Rgb(u8, u8, u8),
}

impl CsiCommand {
    pub fn parse(final_byte: u8, params: &[u32], intermediate: &[u8]) -> Option<Self> {
        match final_byte {
            b'A' => Some(Self::CursorMovement(CursorMovement::Up(params.first().copied().unwrap_or(1)))),
            b'B' => Some(Self::CursorMovement(CursorMovement::Down(params.first().copied().unwrap_or(1)))),
            b'C' => Some(Self::CursorMovement(CursorMovement::Forward(params.first().copied().unwrap_or(1)))),
            b'D' => Some(Self::CursorMovement(CursorMovement::Backward(params.first().copied().unwrap_or(1)))),
            b'E' => Some(Self::CursorMovement(CursorMovement::NextLine(params.first().copied().unwrap_or(1)))),
            b'F' => Some(Self::CursorMovement(CursorMovement::PreviousLine(params.first().copied().unwrap_or(1)))),
            b'G' => Some(Self::CursorMovement(CursorMovement::ColumnAbsolute(params.first().copied().unwrap_or(1)))),
            b'H' | b'f' => {
                let row = params.first().copied().unwrap_or(1);
                let col = params.get(1).copied().unwrap_or(1);
                Some(Self::CursorMovement(CursorMovement::Position(row, col)))
            }
            b'R' => {
                // Cursor Position Report (CPR) response: `ESC[<row>;<col>R`.
                let row = params.first().copied().unwrap_or(1);
                let col = params.get(1).copied().unwrap_or(1);
                Some(Self::CursorPositionReport { row, col })
            }
            b'J' => {
                let mode = match params.first().copied().unwrap_or(0) {
                    0 => EraseMode::CursorToEnd,
                    1 => EraseMode::CursorToBeginning,
                    2 => EraseMode::Entire,
                    3 => EraseMode::CursorToEndLines,
                    _ => EraseMode::CursorToEnd,
                };
                Some(Self::Erase(mode))
            }
            b'K' => {
                let mode = match params.first().copied().unwrap_or(0) {
                    0 => EraseMode::CursorToEnd,
                    1 => EraseMode::CursorToBeginning,
                    2 => EraseMode::Entire,
                    _ => EraseMode::CursorToEnd,
                };
                Some(Self::Erase(mode))
            }
            b'L' => Some(Self::InsertLine(params.first().copied().unwrap_or(1))),
            b'M' => Some(Self::DeleteLine(params.first().copied().unwrap_or(1))),
            b'P' => Some(Self::DeleteChar(params.first().copied().unwrap_or(1))),
            b'@' => Some(Self::InsertChar(params.first().copied().unwrap_or(1))),
            b'X' => Some(Self::EraseChar(params.first().copied().unwrap_or(1))),
            b'S' => Some(Self::Scroll(ScrollDirection::Up, params.first().copied().unwrap_or(1))),
            b'T' => Some(Self::Scroll(ScrollDirection::Down, params.first().copied().unwrap_or(1))),
            b'n' => {
                if params.first() == Some(&6) {
                    Some(Self::DeviceStatus(DeviceStatus::ReportCursorPosition))
                } else if params.first() == Some(&0) {
                    Some(Self::DeviceStatus(DeviceStatus::DeviceAttributes))
                } else {
                    None
                }
            }
            b'c' => {
                if intermediate.first() == Some(&b'?') {
                    Some(Self::DeviceAttributes(params.to_vec()))
                } else if intermediate.first() == Some(&b'>') {
                    Some(Self::SecondaryDeviceAttributes(params.to_vec()))
                } else {
                    Some(Self::DeviceStatus(DeviceStatus::DeviceAttributes))
                }
            }
            b'}' | b'|' => {
                if intermediate.first() == Some(&b'!') {
                    Some(Self::TertiaryDeviceAttributes(String::new()))
                } else {
                    None
                }
            }
            b'h' => {
                if intermediate.first() == Some(&b'?') {
                    let mode = params.first().copied().unwrap_or(0);
                    if mode == 27127 {
                        let level = params.get(1).copied().unwrap_or(1) as u8;
                        Some(Self::KittyEnhancementLevel { level, action: ModeAction::Set })
                    } else {
                        Some(Self::Mode(ModeAction::Set, ModeType::Private(mode)))
                    }
                } else {
                    let mode = params.first().copied().unwrap_or(0);
                    Some(Self::Mode(ModeAction::Set, ModeType::Normal(mode)))
                }
            }
            b'l' => {
                if intermediate.first() == Some(&b'?') {
                    let mode = params.first().copied().unwrap_or(0);
                    if mode == 27127 {
                        let level = params.get(1).copied().unwrap_or(1) as u8;
                        Some(Self::KittyEnhancementLevel { level, action: ModeAction::Reset })
                    } else {
                        Some(Self::Mode(ModeAction::Reset, ModeType::Private(mode)))
                    }
                } else {
                    let mode = params.first().copied().unwrap_or(0);
                    Some(Self::Mode(ModeAction::Reset, ModeType::Normal(mode)))
                }
            }
            b's' => Some(Self::CursorPositionSave),
            b'u' => {
                if intermediate.first() == Some(&b'?') {
                    Some(Self::KittyKeyboardQuery(params.to_vec()))
                } else if params.is_empty() || params.first() == Some(&0) || params.first() == Some(&1) {
                    Some(Self::CursorPositionRestore)
                } else {
                    let keycode = params[0];
                    let modifiers = params.get(1).copied().unwrap_or(0);
                    let event_type_value = params.get(2).copied().unwrap_or(1);
                    let event_type = KittyEventType::from_flag(event_type_value);
                    let associated_text = None;
                    Some(Self::KittyKeyEvent { keycode, modifiers, event_type, associated_text })
                }
            }
            b'm' => Some(Self::Sgr(parse_sgr(params))),
            b'g' => {
                let action = match params.first().copied().unwrap_or(0) {
                    0 => TabStopAction::Set,
                    2 => TabStopAction::Clear,
                    3 => TabStopAction::ClearAll,
                    _ => TabStopAction::Set,
                };
                Some(Self::TabStop(action))
            }
            _ => Some(Self::Unknown(final_byte, params.to_vec())),
        }
    }
}

fn parse_sgr(params: &[u32]) -> Vec<SgrAttribute> {
    let mut attrs = Vec::new();
    let mut i = 0;

    while i < params.len() {
        match params[i] {
            0 => attrs.push(SgrAttribute::Reset),
            1 => attrs.push(SgrAttribute::Bold),
            2 => attrs.push(SgrAttribute::Dim),
            3 => attrs.push(SgrAttribute::Italic),
            4 => attrs.push(SgrAttribute::Underline),
            5 => attrs.push(SgrAttribute::Blink),
            7 => attrs.push(SgrAttribute::Inverse),
            8 => attrs.push(SgrAttribute::Hidden),
            9 => attrs.push(SgrAttribute::Strikethrough),
            30..=37 => {
                let color = match params[i] - 30 {
                    0 => ForegroundColor::Black,
                    1 => ForegroundColor::Red,
                    2 => ForegroundColor::Green,
                    3 => ForegroundColor::Yellow,
                    4 => ForegroundColor::Blue,
                    5 => ForegroundColor::Magenta,
                    6 => ForegroundColor::Cyan,
                    7 => ForegroundColor::White,
                    _ => ForegroundColor::Black,
                };
                attrs.push(SgrAttribute::Foreground(color));
            }
            38 => {
                if let Some(color) = parse_extended_color(params, &mut i) {
                    attrs.push(SgrAttribute::Foreground(color));
                }
            }
            39 => attrs.push(SgrAttribute::Foreground(ForegroundColor::Default)),
            40..=47 => {
                let color = match params[i] - 40 {
                    0 => BackgroundColor::Black,
                    1 => BackgroundColor::Red,
                    2 => BackgroundColor::Green,
                    3 => BackgroundColor::Yellow,
                    4 => BackgroundColor::Blue,
                    5 => BackgroundColor::Magenta,
                    6 => BackgroundColor::Cyan,
                    7 => BackgroundColor::White,
                    _ => BackgroundColor::Black,
                };
                attrs.push(SgrAttribute::Background(color));
            }
            48 => {
                if let Some(color) = parse_extended_bg_color(params, &mut i) {
                    attrs.push(SgrAttribute::Background(color));
                }
            }
            49 => attrs.push(SgrAttribute::Background(BackgroundColor::Default)),
            90..=97 => {
                let color = match params[i] - 90 {
                    0 => ForegroundColor::BrightBlack,
                    1 => ForegroundColor::BrightRed,
                    2 => ForegroundColor::BrightGreen,
                    3 => ForegroundColor::BrightYellow,
                    4 => ForegroundColor::BrightBlue,
                    5 => ForegroundColor::BrightMagenta,
                    6 => ForegroundColor::BrightCyan,
                    7 => ForegroundColor::BrightWhite,
                    _ => ForegroundColor::BrightBlack,
                };
                attrs.push(SgrAttribute::Foreground(color));
            }
            100..=107 => {
                let color = match params[i] - 100 {
                    0 => BackgroundColor::BrightBlack,
                    1 => BackgroundColor::BrightRed,
                    2 => BackgroundColor::BrightGreen,
                    3 => BackgroundColor::BrightYellow,
                    4 => BackgroundColor::BrightBlue,
                    5 => BackgroundColor::BrightMagenta,
                    6 => BackgroundColor::BrightCyan,
                    7 => BackgroundColor::BrightWhite,
                    _ => BackgroundColor::BrightBlack,
                };
                attrs.push(SgrAttribute::Background(color));
            }
            _ => {}
        }
        i += 1;
    }

    attrs
}

fn parse_extended_color(params: &[u32], i: &mut usize) -> Option<ForegroundColor> {
    if *i + 1 >= params.len() {
        return None;
    }

    match params[*i + 1] {
        5 => {
            if *i + 2 < params.len() {
                *i += 2;
                Some(ForegroundColor::Extended(params[*i] as u8))
            } else {
                None
            }
        }
        2 => {
            if *i + 4 < params.len() {
                *i += 4;
                Some(ForegroundColor::Rgb(params[*i - 2] as u8, params[*i - 1] as u8, params[*i] as u8))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_extended_bg_color(params: &[u32], i: &mut usize) -> Option<BackgroundColor> {
    if *i + 1 >= params.len() {
        return None;
    }

    match params[*i + 1] {
        5 => {
            if *i + 2 < params.len() {
                *i += 2;
                Some(BackgroundColor::Extended(params[*i] as u8))
            } else {
                None
            }
        }
        2 => {
            if *i + 4 < params.len() {
                *i += 4;
                Some(BackgroundColor::Rgb(params[*i - 2] as u8, params[*i - 1] as u8, params[*i] as u8))
            } else {
                None
            }
        }
        _ => None,
    }
}

// === parser/sgr.rs ===

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

// === parser/osc.rs ===

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OscCommand {
    SetClipboard(ClipboardData),
    SetHyperlink(Hyperlink),
    SetIconName(String),
    SetTitle(String),
    /// OSC 4 palette entry: `OSC 4 ; index ; spec ST`. `spec` is either an
    /// `rgb:RR/GG/BB` color (a set, or a query response) or `?` (a query).
    SetPaletteColor {
        index: u32,
        spec: String,
    },
    SetBackgroundColor(String),
    SetForegroundColor(String),
    SetCursorColor(String),
    SetMouseCursorShape(String),
    SetWorkingDirectory(String),
    InvalidUrl(String),
    Unknown(u32, Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardData {
    pub data: String,
    pub selection: ClipboardSelection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardSelection {
    Clipboard,
    Primary,
    Secondary,
    Tertiary,
}

impl ClipboardSelection {
    /// The OSC 52 selection parameter character (`c`/`p`/`s`/`q`).
    pub fn param(self) -> char {
        match self {
            Self::Clipboard => 'c',
            Self::Primary => 'p',
            Self::Secondary => 's',
            Self::Tertiary => 'q',
        }
    }
}

impl ClipboardData {
    /// Builds an OSC 52 sequence that sets the terminal clipboard to `text`.
    ///
    /// The payload is base64-encoded per the OSC 52 spec. The returned bytes
    /// include the `ESC ] 52 ; <sel> ; <base64> ESC \` framing (ST terminator).
    pub fn set_sequence(selection: ClipboardSelection, text: &str) -> Vec<u8> {
        use base64::Engine as _;
        let encoded = base64::engine::general_purpose::STANDARD.encode(text.as_bytes());
        format!("\x1b]52;{};{}\x1b\\", selection.param(), encoded).into_bytes()
    }

    /// Builds an OSC 52 *query* sequence (`... ; ? ...`) asking the terminal to
    /// report the current clipboard contents. The response arrives as an inbound
    /// OSC 52 which [`OscCommand::parse`] decodes into a [`ClipboardData`].
    pub fn query_sequence(selection: ClipboardSelection) -> Vec<u8> {
        format!("\x1b]52;{};?\x1b\\", selection.param()).into_bytes()
    }

    /// Decodes the base64 `data` field into the clipboard text.
    ///
    /// Returns `None` when the payload is the query marker `?` or is not valid
    /// base64/UTF-8.
    pub fn decoded(&self) -> Option<String> {
        use base64::Engine as _;
        if self.data == "?" {
            return None;
        }
        let bytes = base64::engine::general_purpose::STANDARD.decode(self.data.as_bytes()).ok()?;
        String::from_utf8(bytes).ok()
    }

    /// Returns `true` if this is a clipboard *query* (data is the `?` marker).
    pub fn is_query(&self) -> bool {
        self.data == "?"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hyperlink {
    pub id: Option<String>,
    pub uri: String,
}

impl OscCommand {
    pub fn parse(data: &[u8]) -> Option<Self> {
        let s = String::from_utf8_lossy(data);
        let parts: Vec<&str> = s.splitn(2, ';').collect();

        if parts.len() < 2 {
            return None;
        }

        let code: u32 = parts[0].parse().ok()?;
        let value = parts[1];

        match code {
            52 => {
                let clipboard_parts: Vec<&str> = value.splitn(2, ';').collect();
                if clipboard_parts.len() < 2 {
                    return None;
                }

                let selection = match clipboard_parts[0] {
                    "c" | "Clipboard" => ClipboardSelection::Clipboard,
                    "p" | "Primary" => ClipboardSelection::Primary,
                    "s" | "Secondary" => ClipboardSelection::Secondary,
                    "q" | "Tertiary" => ClipboardSelection::Tertiary,
                    "0" => ClipboardSelection::Clipboard,
                    "1" => ClipboardSelection::Primary,
                    "2" => ClipboardSelection::Secondary,
                    "3" => ClipboardSelection::Tertiary,
                    _ => ClipboardSelection::Clipboard,
                };

                let data = clipboard_parts[1].to_string();
                Some(Self::SetClipboard(ClipboardData { data, selection }))
            }
            8 => {
                let link_parts: Vec<&str> = value.splitn(2, ';').collect();
                let id = if link_parts[0].is_empty() { None } else { Some(link_parts[0].to_string()) };
                let uri = link_parts.get(1).unwrap_or(&"").to_string();
                Some(Self::SetHyperlink(Hyperlink { id, uri }))
            }
            4 => {
                // OSC 4 ; index ; spec  — palette set or query response.
                let palette_parts: Vec<&str> = value.splitn(2, ';').collect();
                let index: u32 = palette_parts[0].parse().ok()?;
                let spec = palette_parts.get(1).unwrap_or(&"").to_string();
                Some(Self::SetPaletteColor { index, spec })
            }
            0 | 2 => Some(Self::SetTitle(value.to_string())),
            1 => Some(Self::SetIconName(value.to_string())),
            10 => Some(Self::SetForegroundColor(value.to_string())),
            11 => Some(Self::SetBackgroundColor(value.to_string())),
            12 => Some(Self::SetCursorColor(value.to_string())),
            13 => Some(Self::SetMouseCursorShape(value.to_string())),
            7 => Some(Self::SetWorkingDirectory(value.to_string())),
            _ => Some(Self::Unknown(code, data.to_vec())),
        }
    }

    /// Builds an OSC 4 query for palette entry `index`: `OSC 4 ; index ; ? ST`.
    /// The terminal replies with an OSC 4 carrying an `rgb:RR/GG/BB` spec, which
    /// [`OscCommand::parse`] decodes into `SetPaletteColor` and
    /// [`OscCommand::palette_rgb`] turns into `(r, g, b)`.
    pub fn palette_query(index: u32) -> Vec<u8> {
        format!("\x1b]4;{};?\x1b\\", index).into_bytes()
    }

    /// Builds an OSC 4 set sequence assigning `rgb` to palette entry `index`.
    pub fn palette_set(index: u32, r: u8, g: u8, b: u8) -> Vec<u8> {
        // OSC 4 uses 16-bit color components (`rgb:RRRR/GGGG/BBBB`); duplicate
        // each 8-bit byte to the high and low nibble pairs.
        format!("\x1b]4;{};rgb:{:02x}{:02x}/{:02x}{:02x}/{:02x}{:02x}\x1b\\", index, r, r, g, g, b, b).into_bytes()
    }

    /// If this is a `SetPaletteColor` whose spec is an `rgb:R/G/B` color, returns
    /// the decoded 8-bit `(r, g, b)`. Returns `None` for query markers (`?`) or
    /// unrecognized specs. Handles both 8-bit (`rgb:ff/00/00`) and 16-bit
    /// (`rgb:ffff/0000/0000`) component widths, using the high byte of each.
    pub fn palette_rgb(&self) -> Option<(u8, u8, u8)> {
        let Self::SetPaletteColor { spec, .. } = self else {
            return None;
        };
        let body = spec.strip_prefix("rgb:")?;
        let mut parts = body.split('/');
        let r = parse_osc_color_component(parts.next()?)?;
        let g = parse_osc_color_component(parts.next()?)?;
        let b = parse_osc_color_component(parts.next()?)?;
        if parts.next().is_some() {
            return None;
        }
        Some((r, g, b))
    }
}

/// Parses one `rgb:` color component (1–4 hex digits) into an 8-bit value,
/// scaling by taking the most-significant byte for 16-bit components.
fn parse_osc_color_component(s: &str) -> Option<u8> {
    if s.is_empty() || s.len() > 4 {
        return None;
    }
    let v = u32::from_str_radix(s, 16).ok()?;
    // Normalize to 8 bits based on the number of hex digits (bits = 4 * len).
    let value = match s.len() {
        1 => v * 0x11, // 4-bit -> 8-bit (0xF -> 0xFF)
        2 => v,        // already 8-bit
        3 => v >> 4,   // 12-bit -> 8-bit
        4 => v >> 8,   // 16-bit -> 8-bit
        _ => return None,
    };
    Some(value as u8)
}
