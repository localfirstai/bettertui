//! VT100/VTxxx terminal emulation state machine.

use crate::ansi::{
    BackgroundColor, CsiCommand, CursorMovement, EraseMode, ForegroundColor, KittyEventType, ModeAction, ModeType,
    OscCommand, ParserEvent, ScrollDirection, SgrAttribute, TabStopAction,
};
use crate::framebuffer::{Cell, CellAttributes, FrameBuffer};
use crate::input::{KeyAction, KeyModifiers, KeyboardInput};
use crate::tree::{Color, NamedColor};

// =============================================================================
// Cursor
// =============================================================================

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CursorStyle {
    #[default]
    Block,
    Underline,
    Bar,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CursorShape {
    Static,
    #[default]
    Blinking,
    Block,
    Underline,
    VerticalLine,
}

/// Terminal cursor position, visibility, and rendering style.
#[derive(Debug, Clone)]
pub struct Cursor {
    row: u16,
    col: u16,
    saved_row: u16,
    saved_col: u16,
    visible: bool,
    style: CursorStyle,
    shape: CursorShape,
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            saved_row: 0,
            saved_col: 0,
            visible: true,
            style: CursorStyle::default(),
            shape: CursorShape::default(),
        }
    }

    pub fn row(&self) -> u16 {
        self.row
    }

    pub fn col(&self) -> u16 {
        self.col
    }

    pub fn position(&self) -> (u16, u16) {
        (self.row, self.col)
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn style(&self) -> CursorStyle {
        self.style
    }

    pub fn shape(&self) -> CursorShape {
        self.shape
    }

    pub fn set_position(&mut self, row: u16, col: u16) {
        self.row = row;
        self.col = col;
    }

    pub fn move_up(&mut self, n: u16) {
        self.row = self.row.saturating_sub(n);
    }

    pub fn move_down(&mut self, n: u16, max_row: u16) {
        self.row = (self.row + n).min(max_row.saturating_sub(1));
    }

    pub fn move_left(&mut self, n: u16) {
        self.col = self.col.saturating_sub(n);
    }

    pub fn move_right(&mut self, n: u16, max_col: u16) {
        self.col = (self.col + n).min(max_col.saturating_sub(1));
    }

    pub fn move_to_column(&mut self, col: u16) {
        self.col = col.saturating_sub(1);
    }

    pub fn move_to(&mut self, row: u16, col: u16) {
        self.row = row.saturating_sub(1);
        self.col = col.saturating_sub(1);
    }

    pub fn save_position(&mut self) {
        self.saved_row = self.row;
        self.saved_col = self.col;
    }

    pub fn restore_position(&mut self) {
        self.row = self.saved_row;
        self.col = self.saved_col;
    }

    pub fn carriage_return(&mut self) {
        self.col = 0;
    }

    pub fn newline(&mut self) {
        self.row += 1;
    }

    pub fn tab(&mut self, tab_stops: &[u16]) {
        for &stop in tab_stops {
            if stop > self.col {
                self.col = stop;
                return;
            }
        }
        self.col = (self.col + 8) & !7;
    }

    pub fn backspace(&mut self) {
        self.col = self.col.saturating_sub(1);
    }
}

// =============================================================================
// Modes
// =============================================================================

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

// =============================================================================
// Screen
// =============================================================================

/// A scrollable terminal screen buffer backed by a [`FrameBuffer`] with
/// tab-stop management and a scrollback history.
#[derive(Debug, Clone)]
pub struct ScreenBuffer {
    buffer: FrameBuffer,
    tab_stops: Vec<u16>,
    scrollback: crate::terminal::scrollback::ScrollbackBuffer,
    default_bg: Color,
}

impl ScreenBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        let mut tab_stops = Vec::new();
        for t in (8..width).step_by(8) {
            tab_stops.push(t);
        }

        Self {
            buffer: FrameBuffer::new(width, height),
            tab_stops,
            scrollback: crate::terminal::scrollback::ScrollbackBuffer::with_width(width),
            default_bg: Color::Default,
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer.resize(width, height);
        self.tab_stops.clear();
        for t in (8..width).step_by(8) {
            self.tab_stops.push(t);
        }
        self.scrollback.resize(width);
    }

    pub fn buffer(&self) -> &FrameBuffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut FrameBuffer {
        &mut self.buffer
    }

    pub fn width(&self) -> u16 {
        self.buffer.width()
    }

    pub fn height(&self) -> u16 {
        self.buffer.height()
    }

    pub fn tab_stops(&self) -> &[u16] {
        &self.tab_stops
    }

    pub fn set_tab_stop(&mut self, col: u16) {
        if !self.tab_stops.contains(&col) {
            self.tab_stops.push(col);
            self.tab_stops.sort();
        }
    }

    pub fn clear_tab_stop(&mut self, col: u16) {
        self.tab_stops.retain(|&t| t != col);
    }

    pub fn clear_all_tab_stops(&mut self) {
        self.tab_stops.clear();
    }

    pub fn scrollback(&self) -> &crate::terminal::scrollback::ScrollbackBuffer {
        &self.scrollback
    }

    /// Returns the number of lines stored in the scrollback buffer.
    pub fn scrollback_len(&self) -> usize {
        self.scrollback.len()
    }

    pub fn set_cell(&mut self, row: u16, col: u16, ch: char, fg: Color, bg: Color, attrs: CellAttributes) {
        if row < self.buffer.height() && col < self.buffer.width() {
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            cell.attributes = attrs;
            self.buffer.set(col, row, cell);
        }
    }

    pub fn write_char(&mut self, row: u16, col: u16, ch: char, pen: &Pen) {
        self.set_cell(row, col, ch, pen.fg, pen.bg, pen.attrs);
    }

    pub fn erase_char(&mut self, row: u16, col: u16, pen: &Pen) {
        self.set_cell(row, col, ' ', Color::Default, pen.bg, CellAttributes::empty());
    }

    pub fn erase_in_display(&mut self, mode: u32, cursor_row: u16, cursor_col: u16, pen: &Pen) {
        let rows = self.buffer.height();
        let cols = self.buffer.width();

        match mode {
            0 => {
                for y in cursor_row..rows {
                    let start_col = if y == cursor_row { cursor_col } else { 0 };
                    for x in start_col..cols {
                        self.erase_char(y, x, pen);
                    }
                }
            }
            1 => {
                for y in 0..=cursor_row {
                    let end_col = if y == cursor_row { cursor_col + 1 } else { cols };
                    for x in 0..end_col {
                        self.erase_char(y, x, pen);
                    }
                }
            }
            2 | 3 => {
                self.clear_lines(0, rows, pen);
                if mode == 3 {
                    self.scrollback.clear();
                }
            }
            _ => {}
        }
    }

    pub fn erase_in_line(&mut self, mode: u32, row: u16, cursor_col: u16, pen: &Pen) {
        if row >= self.buffer.height() {
            return;
        }
        let cols = self.buffer.width();

        match mode {
            0 => {
                for x in cursor_col..cols {
                    self.erase_char(row, x, pen);
                }
            }
            1 => {
                for x in 0..=cursor_col {
                    self.erase_char(row, x, pen);
                }
            }
            2 => {
                for x in 0..cols {
                    self.erase_char(row, x, pen);
                }
            }
            _ => {}
        }
    }

    pub fn scroll_up(&mut self, count: u16, pen: &Pen) {
        let rows = self.buffer.height();
        let cols = self.buffer.width();
        let count = count.min(rows);

        for y in 0..count {
            let mut line = Vec::with_capacity(cols as usize);
            for x in 0..cols {
                let cell = self.buffer.get(x, y);
                line.push(cell);
            }
            self.scrollback.push_line(line, cols, false);
        }

        for y in count..rows {
            for x in 0..cols {
                let src = self.buffer.get(x, y);
                self.buffer.set(x, y - count, src);
            }
        }

        for y in (rows - count)..rows {
            for x in 0..cols {
                self.erase_char(y, x, pen);
            }
        }
    }

    pub fn scroll_down(&mut self, count: u16, pen: &Pen) {
        let rows = self.buffer.height();
        let cols = self.buffer.width();
        let count = count.min(rows);

        for y in (count..rows).rev() {
            for x in 0..cols {
                let src = self.buffer.get(x, y - count);
                self.buffer.set(x, y, src);
            }
        }

        for y in 0..count {
            for x in 0..cols {
                self.erase_char(y, x, pen);
            }
        }
    }

    pub fn insert_lines(&mut self, row: u16, count: u16, pen: &Pen) {
        let rows = self.buffer.height();
        let cols = self.buffer.width();
        let count = count.min(rows.saturating_sub(row));

        if count == 0 {
            return;
        }

        for y in ((row + count)..rows).rev() {
            for x in 0..cols {
                let src = self.buffer.get(x, y - count);
                self.buffer.set(x, y, src);
            }
        }

        for y in row..(row + count).min(rows) {
            for x in 0..cols {
                self.erase_char(y, x, pen);
            }
        }
    }

    pub fn delete_lines(&mut self, row: u16, count: u16, pen: &Pen) {
        let rows = self.buffer.height();
        let cols = self.buffer.width();
        let count = count.min(rows.saturating_sub(row));

        if count == 0 {
            return;
        }

        for y in (row + count)..rows {
            for x in 0..cols {
                let src = self.buffer.get(x, y);
                self.buffer.set(x, y - count, src);
            }
        }

        for y in (rows - count)..rows {
            for x in 0..cols {
                self.erase_char(y, x, pen);
            }
        }
    }

    pub fn insert_chars(&mut self, row: u16, col: u16, count: u16, pen: &Pen) {
        let cols = self.buffer.width();
        let count = count.min(cols.saturating_sub(col));

        if count == 0 {
            return;
        }

        for x in ((col + count)..cols).rev() {
            let src = self.buffer.get(x - count, row);
            self.buffer.set(x, row, src);
        }

        for x in col..(col + count) {
            self.erase_char(row, x, pen);
        }
    }

    pub fn delete_chars(&mut self, row: u16, col: u16, count: u16, pen: &Pen) {
        let cols = self.buffer.width();
        let count = count.min(cols.saturating_sub(col));

        if count == 0 {
            return;
        }

        for x in (col + count)..cols {
            let src = self.buffer.get(x, row);
            self.buffer.set(x - count, row, src);
        }

        for x in (cols - count)..cols {
            self.erase_char(row, x, pen);
        }
    }

    pub fn erase_chars(&mut self, row: u16, col: u16, count: u16, pen: &Pen) {
        let cols = self.buffer.width();
        let count = count.min(cols.saturating_sub(col));
        for x in col..(col + count) {
            self.erase_char(row, x, pen);
        }
    }

    pub fn set_default_bg(&mut self, bg: Color) {
        self.default_bg = bg;
    }

    pub fn default_bg(&self) -> Color {
        self.default_bg
    }

    fn clear_lines(&mut self, start: u16, end: u16, pen: &Pen) {
        let cols = self.buffer.width();
        for y in start..end.min(self.buffer.height()) {
            for x in 0..cols {
                self.erase_char(y, x, pen);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pen {
    pub fg: Color,
    pub bg: Color,
    pub attrs: CellAttributes,
}

impl Default for Pen {
    fn default() -> Self {
        Self { fg: Color::Default, bg: Color::Default, attrs: CellAttributes::empty() }
    }
}

// =============================================================================
// VT Machine
// =============================================================================

/// The primary VT100/VTxxx terminal emulation state machine.
///
/// Processes [`ParserEvent`]s from the ANSI parser and maintains:
/// - Primary and alternate screen buffers
/// - Cursor state (position, visibility, style)
/// - Terminal modes (wrapping, insert, reverse video, etc.)
/// - Rendering attributes (pen/ink state)
/// - Terminal responses (DA1/DA2/DA3, Kitty keyboard protocol)
#[derive(Debug, Clone)]
pub struct VtMachine {
    screen: ScreenBuffer,
    alt_screen: ScreenBuffer,
    cursor: Cursor,
    alt_cursor: Cursor,
    modes: TerminalMode,
    pen: Pen,
    title: String,
    icon_name: String,
    hyperlink: Option<(Option<String>, String)>,
    clipboard: Option<String>,
    device_attributes: Option<Vec<u32>>,
    secondary_device_attributes: Option<Vec<u32>>,
    tertiary_device_attributes: Option<String>,
    terminal_responses: Vec<TerminalResponse>,
    last_kitty_key: Option<KittyKeyEvent>,
    kitty_enhancement_levels: [u8; 5],
    kitty_keyboard_query_response: Option<Vec<u32>>,
}

#[derive(Debug, Clone)]
pub struct KittyKeyEvent {
    pub keycode: u32,
    pub modifiers: u32,
    pub event_type: KittyEventType,
    pub associated_text: Option<String>,
}

impl KittyKeyEvent {
    pub fn to_keyboard_input(&self) -> KeyboardInput {
        let mut mods = KeyModifiers::empty();
        if self.modifiers & 1 != 0 {
            mods.insert(KeyModifiers::SHIFT);
        }
        if self.modifiers & 4 != 0 {
            mods.insert(KeyModifiers::CONTROL);
        }
        if self.modifiers & 2 != 0 {
            mods.insert(KeyModifiers::ALT);
        }
        if self.modifiers & 8 != 0 {
            mods.insert(KeyModifiers::SUPER);
        }
        let action = match self.event_type {
            KittyEventType::Press => KeyAction::Press,
            KittyEventType::Repeat => KeyAction::Repeat,
            KittyEventType::Release => KeyAction::Release,
            KittyEventType::Unknown => KeyAction::Press,
        };
        let key = char::from_u32(self.keycode).unwrap_or('\0');
        KeyboardInput { key, modifiers: mods, action }
    }
}

#[derive(Debug, Clone)]
pub struct TerminalResponse {
    pub query: String,
    pub params: Vec<u32>,
    pub intermediate: Option<u8>,
    pub kind: ResponseKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseKind {
    DeviceAttributes,
    SecondaryDeviceAttributes,
    TertiaryDeviceAttributes,
    CursorPosition,
    Unknown,
}

impl VtMachine {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            screen: ScreenBuffer::new(width, height),
            alt_screen: ScreenBuffer::new(width, height),
            cursor: Cursor::new(),
            alt_cursor: Cursor::new(),
            modes: TerminalMode::default(),
            pen: Pen::default(),
            title: String::new(),
            icon_name: String::new(),
            hyperlink: None,
            clipboard: None,
            device_attributes: None,
            secondary_device_attributes: None,
            tertiary_device_attributes: None,
            terminal_responses: Vec::new(),
            last_kitty_key: None,
            kitty_enhancement_levels: [0; 5],
            kitty_keyboard_query_response: None,
        }
    }

    // ── Public accessors ──

    pub fn current_screen(&self) -> &ScreenBuffer {
        if self.modes.alt_screen() { &self.alt_screen } else { &self.screen }
    }

    pub fn current_cursor(&self) -> &Cursor {
        if self.modes.alt_screen() { &self.alt_cursor } else { &self.cursor }
    }

    pub fn current_modes(&self) -> TerminalMode {
        self.modes
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn icon_name(&self) -> &str {
        &self.icon_name
    }

    pub fn device_attributes(&self) -> Option<&[u32]> {
        self.device_attributes.as_deref()
    }

    pub fn secondary_device_attributes(&self) -> Option<&[u32]> {
        self.secondary_device_attributes.as_deref()
    }

    pub fn tertiary_device_attributes(&self) -> Option<&str> {
        self.tertiary_device_attributes.as_deref()
    }

    pub fn terminal_responses(&self) -> &[TerminalResponse] {
        &self.terminal_responses
    }

    pub fn clipboard(&self) -> Option<&str> {
        self.clipboard.as_deref()
    }

    pub fn hyperlink(&self) -> Option<&(Option<String>, String)> {
        self.hyperlink.as_ref()
    }

    pub fn last_kitty_key(&self) -> Option<&KittyKeyEvent> {
        self.last_kitty_key.as_ref()
    }

    pub fn kitty_keyboard_query_response(&self) -> Option<&[u32]> {
        self.kitty_keyboard_query_response.as_deref()
    }

    // ── pub(crate) accessors (for query.rs) ──

    pub(crate) fn device_attributes_mut(&mut self) -> &mut Option<Vec<u32>> {
        &mut self.device_attributes
    }

    pub(crate) fn secondary_device_attributes_mut(&mut self) -> &mut Option<Vec<u32>> {
        &mut self.secondary_device_attributes
    }

    pub(crate) fn tertiary_device_attributes_mut(&mut self) -> &mut Option<String> {
        &mut self.tertiary_device_attributes
    }

    pub(crate) fn kitty_keyboard_query_response_mut(&mut self) -> &mut Option<Vec<u32>> {
        &mut self.kitty_keyboard_query_response
    }

    pub(crate) fn last_kitty_key_mut(&mut self) -> &mut Option<KittyKeyEvent> {
        &mut self.last_kitty_key
    }

    pub(crate) fn terminal_responses_mut(&mut self) -> &mut Vec<TerminalResponse> {
        &mut self.terminal_responses
    }

    // ── Public mutators ──

    pub fn resize(&mut self, width: u16, height: u16) {
        self.screen.resize(width, height);
        self.alt_screen.resize(width, height);
    }

    pub fn process(&mut self, event: &ParserEvent) {
        match event {
            ParserEvent::Char(ch) => self.handle_char(*ch),
            ParserEvent::LineFeed => self.handle_line_feed(),
            ParserEvent::CarriageReturn => {
                self.cursor_mut().carriage_return();
            }
            ParserEvent::Tab => self.handle_tab(),
            ParserEvent::Backspace => self.cursor_mut().backspace(),
            ParserEvent::Bell => {}
            ParserEvent::Index => self.handle_index(),
            ParserEvent::ReverseIndex => self.handle_reverse_index(),
            ParserEvent::NextLine => self.handle_next_line(),
            ParserEvent::Reset => self.reset(),
            ParserEvent::Csi(cmd) => self.handle_csi(cmd),
            ParserEvent::Osc(cmd) => self.handle_osc(cmd),
            ParserEvent::Dcs(_) => {}
            ParserEvent::Pm(_) => {}
            ParserEvent::Sos(_) => {}
            ParserEvent::Apc(_) => {}
        }
    }

    pub fn framebuffer(&self) -> &FrameBuffer {
        self.current_screen().buffer()
    }

    // ── Private helpers ──

    fn screen_mut(&mut self) -> &mut ScreenBuffer {
        if self.modes.alt_screen() { &mut self.alt_screen } else { &mut self.screen }
    }

    fn cursor(&self) -> &Cursor {
        if self.modes.alt_screen() { &self.alt_cursor } else { &self.cursor }
    }

    fn cursor_mut(&mut self) -> &mut Cursor {
        if self.modes.alt_screen() { &mut self.alt_cursor } else { &mut self.cursor }
    }

    fn handle_char(&mut self, byte: u8) {
        let ch = byte as char;
        if ch.is_ascii_control() && ch != ' ' {
            return;
        }

        let auto_wrap = self.modes.auto_wrap();
        let cursor_row = self.cursor().row;
        let cursor_col = self.cursor().col;
        let pen = self.pen;
        let screen = self.screen_mut();

        screen.write_char(cursor_row, cursor_col, ch, &pen);

        let max_col = screen.width();

        if cursor_col + 1 >= max_col {
            if auto_wrap {
                if cursor_row + 1 >= self.current_screen().height() {
                    self.screen_mut().scroll_up(1, &pen);
                } else {
                    self.cursor_mut().row += 1;
                }
                self.cursor_mut().col = 0;
            }
        } else {
            self.cursor_mut().col += 1;
        }
    }

    fn handle_line_feed(&mut self) {
        let pen = self.pen;
        let height = self.current_screen().height();
        let row = self.cursor().row;
        if row + 1 >= height {
            self.screen_mut().scroll_up(1, &pen);
            self.cursor_mut().row = row;
            self.cursor_mut().col = 0;
        } else {
            self.cursor_mut().row = row + 1;
            self.cursor_mut().col = 0;
        }
    }

    fn handle_next_line(&mut self) {
        self.cursor_mut().carriage_return();
        self.handle_line_feed();
    }

    fn handle_index(&mut self) {
        let pen = self.pen;
        let height = self.current_screen().height();
        let row = self.cursor().row;
        if row + 1 >= height {
            self.screen_mut().scroll_up(1, &pen);
            self.cursor_mut().row = row;
        } else {
            self.cursor_mut().row = row + 1;
        }
    }

    fn handle_reverse_index(&mut self) {
        let row = self.cursor().row;
        if row == 0 {
            let pen = self.pen;
            self.screen_mut().scroll_down(1, &pen);
        } else {
            self.cursor_mut().row = row - 1;
        }
    }

    fn handle_tab(&mut self) {
        let tab_stops = self.current_screen().tab_stops().to_vec();
        let max_col = self.current_screen().width().saturating_sub(1);
        let cursor = self.cursor_mut();
        cursor.tab(&tab_stops);
        cursor.col = cursor.col.min(max_col);
    }

    fn reset(&mut self) {
        let w = self.screen.width();
        let h = self.screen.height();
        *self = Self::new(w, h);
    }

    fn handle_csi(&mut self, cmd: &CsiCommand) {
        match cmd {
            CsiCommand::CursorMovement(movement) => {
                self.handle_cursor_movement(movement);
            }
            CsiCommand::Sgr(attrs) => self.handle_sgr(attrs),
            CsiCommand::Erase(mode) => self.handle_erase(mode),
            CsiCommand::Scroll(dir, count) => self.handle_scroll(*dir, *count),
            CsiCommand::DeleteLine(count) => {
                let cnt = (*count as u16).max(1);
                let row = self.cursor().row;
                let pen = self.pen;
                self.screen_mut().delete_lines(row, cnt, &pen);
            }
            CsiCommand::InsertLine(count) => {
                let cnt = (*count as u16).max(1);
                let row = self.cursor().row;
                let pen = self.pen;
                self.screen_mut().insert_lines(row, cnt, &pen);
            }
            CsiCommand::DeleteChar(count) => {
                let cnt = (*count as u16).max(1);
                let (row, col) = self.cursor().position();
                let pen = self.pen;
                self.screen_mut().delete_chars(row, col, cnt, &pen);
            }
            CsiCommand::InsertChar(count) => {
                let cnt = (*count as u16).max(1);
                let (row, col) = self.cursor().position();
                let pen = self.pen;
                self.screen_mut().insert_chars(row, col, cnt, &pen);
            }
            CsiCommand::EraseChar(count) => {
                let cnt = (*count as u16).max(1);
                let (row, col) = self.cursor().position();
                let pen = self.pen;
                self.screen_mut().erase_chars(row, col, cnt, &pen);
            }
            CsiCommand::CursorPositionSave => {
                self.cursor_mut().save_position();
            }
            CsiCommand::CursorPositionRestore => {
                self.cursor_mut().restore_position();
            }
            CsiCommand::DeviceStatus(_) => {}
            CsiCommand::DeviceAttributes(params) => {
                self.device_attributes = Some(params.clone());
                self.terminal_responses.push(TerminalResponse {
                    query: String::new(),
                    params: params.clone(),
                    intermediate: Some(b'?'),
                    kind: ResponseKind::DeviceAttributes,
                });
            }
            CsiCommand::SecondaryDeviceAttributes(params) => {
                self.secondary_device_attributes = Some(params.clone());
                self.terminal_responses.push(TerminalResponse {
                    query: String::new(),
                    params: params.clone(),
                    intermediate: Some(b'>'),
                    kind: ResponseKind::SecondaryDeviceAttributes,
                });
            }
            CsiCommand::TertiaryDeviceAttributes(data) => {
                self.tertiary_device_attributes = Some(data.clone());
                self.terminal_responses.push(TerminalResponse {
                    query: String::new(),
                    params: Vec::new(),
                    intermediate: None,
                    kind: ResponseKind::TertiaryDeviceAttributes,
                });
            }
            CsiCommand::Mode(action, mode_type) => self.handle_mode_action(*action, mode_type),
            CsiCommand::TabStop(action) => self.handle_tab_stop(action),
            CsiCommand::AttributeReset => {
                self.pen = Pen::default();
            }
            CsiCommand::KittyKeyEvent { keycode, modifiers, event_type, associated_text } => {
                self.last_kitty_key = Some(KittyKeyEvent {
                    keycode: *keycode,
                    modifiers: *modifiers,
                    event_type: *event_type,
                    associated_text: associated_text.clone(),
                });
            }
            CsiCommand::KittyEnhancementLevel { level, action } => {
                let idx = (*level as usize).saturating_sub(1);
                if idx < 5 {
                    let enabled = *action == ModeAction::Set;
                    self.kitty_enhancement_levels[idx] = if enabled { *level } else { 0 };
                }
                let tm = TerminalMode::KEYBOARD_PROTOCOL;
                match action {
                    ModeAction::Set => self.modes.insert(tm),
                    _ => self.modes.remove(tm),
                }
            }
            CsiCommand::KittyKeyboardQuery(params) => {
                self.kitty_keyboard_query_response = Some(params.clone());
                self.terminal_responses.push(TerminalResponse {
                    query: String::new(),
                    params: params.clone(),
                    intermediate: Some(b'?'),
                    kind: ResponseKind::Unknown,
                });
            }
            CsiCommand::Unknown(_, _) => {}
        }
    }

    fn handle_cursor_movement(&mut self, movement: &CursorMovement) {
        let screen_width = self.current_screen().width();
        let screen_height = self.current_screen().height();
        let origin = self.modes.origin();
        let cursor = self.cursor_mut();

        match movement {
            CursorMovement::Up(n) => cursor.move_up((*n).max(1) as u16),
            CursorMovement::Down(n) => {
                cursor.move_down((*n).max(1) as u16, screen_height);
            }
            CursorMovement::Forward(n) => {
                cursor.move_right((*n).max(1) as u16, screen_width);
            }
            CursorMovement::Backward(n) => cursor.move_left((*n).max(1) as u16),
            CursorMovement::NextLine(n) => {
                cursor.carriage_return();
                cursor.move_down((*n).max(1) as u16, screen_height);
            }
            CursorMovement::PreviousLine(n) => {
                cursor.carriage_return();
                cursor.move_up((*n).max(1) as u16);
            }
            CursorMovement::ColumnAbsolute(n) => {
                cursor.move_to_column(*n as u16);
                let max_col = screen_width.saturating_sub(1);
                cursor.col = cursor.col.min(max_col);
            }
            CursorMovement::Position(row, col) => {
                cursor.move_to(*row as u16, *col as u16);
                let max_row = screen_height.saturating_sub(1);
                let max_col = screen_width.saturating_sub(1);
                if origin {
                    cursor.row = cursor.row.min(max_row);
                }
                cursor.row = cursor.row.min(max_row);
                cursor.col = cursor.col.min(max_col);
            }
        }
    }

    fn handle_sgr(&mut self, attrs: &[SgrAttribute]) {
        for attr in attrs {
            match attr {
                SgrAttribute::Reset => {
                    self.pen = Pen::default();
                }
                SgrAttribute::Bold => self.pen.attrs |= CellAttributes::BOLD,
                SgrAttribute::Dim => self.pen.attrs |= CellAttributes::DIM,
                SgrAttribute::Italic => self.pen.attrs |= CellAttributes::ITALIC,
                SgrAttribute::Underline => self.pen.attrs |= CellAttributes::UNDERLINE,
                SgrAttribute::Blink => {}
                SgrAttribute::Inverse => self.pen.attrs |= CellAttributes::INVERSE,
                SgrAttribute::Hidden => self.pen.attrs |= CellAttributes::HIDDEN,
                SgrAttribute::Strikethrough => self.pen.attrs |= CellAttributes::STRIKETHROUGH,
                SgrAttribute::Foreground(fg) => {
                    self.pen.fg = convert_fg(*fg);
                }
                SgrAttribute::Background(bg) => {
                    self.pen.bg = convert_bg(*bg);
                }
                SgrAttribute::UnderlineColor(_) => {}
            }
        }
    }

    fn handle_erase(&mut self, mode: &EraseMode) {
        let (row, col) = self.cursor().position();
        let pen = self.pen;

        let (code, is_line) = match mode {
            EraseMode::CursorToEnd => (0, false),
            EraseMode::CursorToBeginning => (1, false),
            EraseMode::Entire => (2, false),
            EraseMode::CursorToEndLines => (0, true),
            EraseMode::CursorToBeginningLines => (1, true),
        };

        if is_line {
            self.screen_mut().erase_in_line(code, row, col, &pen);
        } else {
            self.screen_mut().erase_in_display(code, row, col, &pen);
        }
    }

    fn handle_scroll(&mut self, dir: ScrollDirection, count: u32) {
        let cnt = (count as u16).max(1);
        let pen = self.pen;
        match dir {
            ScrollDirection::Up => self.screen_mut().scroll_up(cnt, &pen),
            ScrollDirection::Down => self.screen_mut().scroll_down(cnt, &pen),
        }
    }

    fn handle_mode_action(&mut self, action: ModeAction, mode_type: &ModeType) {
        match mode_type {
            ModeType::Normal(code) if *code == 4 => {
                let enabled = action == ModeAction::Set;
                if enabled {
                    self.modes.insert(TerminalMode::INSERT);
                } else {
                    self.modes.remove(TerminalMode::INSERT);
                }
            }
            ModeType::Private(code) => {
                if let Some(private_mode) = PrivateMode::from_code(*code) {
                    let enabled = action == ModeAction::Set;
                    let m = private_mode.to_terminal_mode();

                    match private_mode {
                        PrivateMode::AltScreen => {
                            if enabled {
                                self.cursor.save_position();
                                self.alt_screen = ScreenBuffer::new(self.screen.width(), self.screen.height());
                                self.modes.insert(m);
                                self.alt_cursor = Cursor::new();
                            } else {
                                self.modes.remove(m);
                                self.cursor.restore_position();
                            }
                        }
                        PrivateMode::SaveCursor => {
                            if enabled {
                                self.cursor_mut().save_position();
                            } else {
                                self.cursor_mut().restore_position();
                            }
                        }
                        _ => {
                            if enabled {
                                self.modes.insert(m);
                            } else {
                                self.modes.remove(m);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_tab_stop(&mut self, action: &TabStopAction) {
        match action {
            TabStopAction::Set => {
                let col = self.cursor().col;
                self.screen_mut().set_tab_stop(col);
            }
            TabStopAction::Clear => {
                let col = self.cursor().col;
                self.screen_mut().clear_tab_stop(col);
            }
            TabStopAction::ClearAll => self.screen_mut().clear_all_tab_stops(),
        }
    }

    fn handle_osc(&mut self, cmd: &OscCommand) {
        match cmd {
            OscCommand::SetClipboard(data) => {
                self.clipboard = Some(data.data.clone());
            }
            OscCommand::SetHyperlink(link) => {
                if link.uri.is_empty() {
                    self.hyperlink = None;
                } else {
                    self.hyperlink = Some((link.id.clone(), link.uri.clone()));
                }
            }
            OscCommand::SetTitle(title) => {
                self.title = title.clone();
            }
            OscCommand::SetIconName(name) => {
                self.icon_name = name.clone();
            }
            OscCommand::SetBackgroundColor(color) => {
                if let Some(c) = parse_color_string(color) {
                    self.pen.bg = c;
                    self.screen_mut().set_default_bg(c);
                }
            }
            OscCommand::SetForegroundColor(color) => {
                if let Some(c) = parse_color_string(color) {
                    self.pen.fg = c;
                }
            }
            OscCommand::SetCursorColor(_) => {}
            OscCommand::SetMouseCursorShape(_) => {}
            OscCommand::SetWorkingDirectory(_) => {}
            OscCommand::InvalidUrl(_) => {}
            OscCommand::Unknown(_, _) => {}
        }
    }
}

fn convert_fg(fg: ForegroundColor) -> Color {
    match fg {
        ForegroundColor::Default => Color::Default,
        ForegroundColor::Black => Color::Named(NamedColor::Black),
        ForegroundColor::Red => Color::Named(NamedColor::Red),
        ForegroundColor::Green => Color::Named(NamedColor::Green),
        ForegroundColor::Yellow => Color::Named(NamedColor::Yellow),
        ForegroundColor::Blue => Color::Named(NamedColor::Blue),
        ForegroundColor::Magenta => Color::Named(NamedColor::Magenta),
        ForegroundColor::Cyan => Color::Named(NamedColor::Cyan),
        ForegroundColor::White => Color::Named(NamedColor::White),
        ForegroundColor::BrightBlack => Color::Named(NamedColor::BrightBlack),
        ForegroundColor::BrightRed => Color::Named(NamedColor::BrightRed),
        ForegroundColor::BrightGreen => Color::Named(NamedColor::BrightGreen),
        ForegroundColor::BrightYellow => Color::Named(NamedColor::BrightYellow),
        ForegroundColor::BrightBlue => Color::Named(NamedColor::BrightBlue),
        ForegroundColor::BrightMagenta => Color::Named(NamedColor::BrightMagenta),
        ForegroundColor::BrightCyan => Color::Named(NamedColor::BrightCyan),
        ForegroundColor::BrightWhite => Color::Named(NamedColor::BrightWhite),
        ForegroundColor::Extended(i) => Color::Indexed(i),
        ForegroundColor::Rgb(r, g, b) => Color::Rgb { r, g, b },
    }
}

fn convert_bg(bg: BackgroundColor) -> Color {
    match bg {
        BackgroundColor::Default => Color::Default,
        BackgroundColor::Black => Color::Named(NamedColor::Black),
        BackgroundColor::Red => Color::Named(NamedColor::Red),
        BackgroundColor::Green => Color::Named(NamedColor::Green),
        BackgroundColor::Yellow => Color::Named(NamedColor::Yellow),
        BackgroundColor::Blue => Color::Named(NamedColor::Blue),
        BackgroundColor::Magenta => Color::Named(NamedColor::Magenta),
        BackgroundColor::Cyan => Color::Named(NamedColor::Cyan),
        BackgroundColor::White => Color::Named(NamedColor::White),
        BackgroundColor::BrightBlack => Color::Named(NamedColor::BrightBlack),
        BackgroundColor::BrightRed => Color::Named(NamedColor::BrightRed),
        BackgroundColor::BrightGreen => Color::Named(NamedColor::BrightGreen),
        BackgroundColor::BrightYellow => Color::Named(NamedColor::BrightYellow),
        BackgroundColor::BrightBlue => Color::Named(NamedColor::BrightBlue),
        BackgroundColor::BrightMagenta => Color::Named(NamedColor::BrightMagenta),
        BackgroundColor::BrightCyan => Color::Named(NamedColor::BrightCyan),
        BackgroundColor::BrightWhite => Color::Named(NamedColor::BrightWhite),
        BackgroundColor::Extended(i) => Color::Indexed(i),
        BackgroundColor::Rgb(r, g, b) => Color::Rgb { r, g, b },
    }
}

fn parse_color_string(s: &str) -> Option<Color> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix('#')
        && hex.len() == 6
    {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        return Some(Color::Rgb { r, g, b });
    }
    if let Some(rgb) = s.strip_prefix("rgb:") {
        let parts: Vec<&str> = rgb.split('/').collect();
        if parts.len() == 3 {
            let r = u8::from_str_radix(parts[0], 16).ok()?;
            let g = u8::from_str_radix(parts[1], 16).ok()?;
            let b = u8::from_str_radix(parts[2], 16).ok()?;
            return Some(Color::Rgb { r, g, b });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Cursor ──

    #[test]
    fn cursor_new_defaults() {
        let c = Cursor::new();
        assert_eq!(c.row(), 0);
        assert_eq!(c.col(), 0);
        assert!(c.visible());
    }

    #[test]
    fn cursor_position() {
        let c = Cursor::new();
        assert_eq!(c.position(), (0, 0));
    }

    #[test]
    fn cursor_set_position() {
        let mut c = Cursor::new();
        c.set_position(5, 10);
        assert_eq!(c.row(), 5);
        assert_eq!(c.col(), 10);
    }

    #[test]
    fn cursor_move_up() {
        let mut c = Cursor::new();
        c.set_position(5, 0);
        c.move_up(3);
        assert_eq!(c.row(), 2);
    }

    #[test]
    fn cursor_move_up_saturate() {
        let mut c = Cursor::new();
        c.move_up(10);
        assert_eq!(c.row(), 0);
    }

    #[test]
    fn cursor_move_down() {
        let mut c = Cursor::new();
        c.move_down(3, 10);
        assert_eq!(c.row(), 3);
    }

    #[test]
    fn cursor_move_down_saturate() {
        let mut c = Cursor::new();
        c.set_position(8, 0);
        c.move_down(5, 10);
        assert_eq!(c.row(), 9);
    }

    #[test]
    fn cursor_move_left() {
        let mut c = Cursor::new();
        c.set_position(0, 10);
        c.move_left(3);
        assert_eq!(c.col(), 7);
    }

    #[test]
    fn cursor_move_left_saturate() {
        let mut c = Cursor::new();
        c.set_position(0, 2);
        c.move_left(10);
        assert_eq!(c.col(), 0);
    }

    #[test]
    fn cursor_move_right() {
        let mut c = Cursor::new();
        c.set_position(0, 3);
        c.move_right(5, 20);
        assert_eq!(c.col(), 8);
    }

    #[test]
    fn cursor_move_right_saturate() {
        let mut c = Cursor::new();
        c.set_position(0, 18);
        c.move_right(5, 20);
        assert_eq!(c.col(), 19);
    }

    #[test]
    fn cursor_move_to_column() {
        let mut c = Cursor::new();
        c.move_to_column(10);
        assert_eq!(c.col(), 9);
    }

    #[test]
    fn cursor_move_to() {
        let mut c = Cursor::new();
        c.move_to(5, 10);
        assert_eq!(c.row(), 4);
        assert_eq!(c.col(), 9);
    }

    #[test]
    fn cursor_save_and_restore() {
        let mut c = Cursor::new();
        c.set_position(10, 20);
        c.save_position();
        c.set_position(5, 5);
        c.restore_position();
        assert_eq!(c.row(), 10);
        assert_eq!(c.col(), 20);
    }

    #[test]
    fn cursor_carriage_return() {
        let mut c = Cursor::new();
        c.set_position(5, 15);
        c.carriage_return();
        assert_eq!(c.col(), 0);
        assert_eq!(c.row(), 5);
    }

    #[test]
    fn cursor_newline() {
        let mut c = Cursor::new();
        c.set_position(3, 0);
        c.newline();
        assert_eq!(c.row(), 4);
    }

    #[test]
    fn cursor_tab_to_next_stop() {
        let mut c = Cursor::new();
        c.set_position(0, 3);
        c.tab(&[5, 10, 20]);
        assert_eq!(c.col(), 5);
    }

    #[test]
    fn cursor_tab_beyond_last_stop() {
        let mut c = Cursor::new();
        c.set_position(0, 25);
        c.tab(&[5, 10, 20]);
        assert_eq!(c.col(), 32);
    }

    #[test]
    fn cursor_backspace() {
        let mut c = Cursor::new();
        c.set_position(0, 10);
        c.backspace();
        assert_eq!(c.col(), 9);
    }

    #[test]
    fn cursor_backspace_saturate() {
        let mut c = Cursor::new();
        c.backspace();
        assert_eq!(c.col(), 0);
    }

    #[test]
    fn cursor_style_default() {
        assert_eq!(CursorStyle::default(), CursorStyle::Block);
    }

    #[test]
    fn cursor_shape_default() {
        assert_eq!(CursorShape::default(), CursorShape::Blinking);
    }

    #[test]
    fn cursor_visible_style_shape() {
        let c = Cursor::new();
        assert!(c.visible());
        assert_eq!(c.style(), CursorStyle::Block);
        assert_eq!(c.shape(), CursorShape::Blinking);
    }

    // ── TerminalMode ──

    #[test]
    fn terminal_mode_default() {
        let m = TerminalMode::default();
        assert!(m.auto_wrap());
        assert!(m.contains(TerminalMode::VISIBLE_CURSOR));
        assert!(!m.alt_screen());
    }

    #[test]
    fn terminal_mode_custom() {
        let m = TerminalMode::INSERT | TerminalMode::ALT_SCREEN;
        assert!(m.is_insert());
        assert!(m.alt_screen());
        assert!(!m.auto_wrap());
    }

    #[test]
    fn terminal_mode_toggle() {
        let mut m = TerminalMode::default();
        assert!(!m.alt_screen());
        m.toggle(TerminalMode::ALT_SCREEN);
        assert!(m.alt_screen());
        m.toggle(TerminalMode::ALT_SCREEN);
        assert!(!m.alt_screen());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Proptest property-based tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
mod proptests {
    use super::*;
    use crate::ansi::{AnsiParser, CursorMovement, EraseMode, KittyEventType, SgrAttribute};
    use proptest::prelude::*;

    // ── Strategies ──

    fn arb_cursor_movement() -> impl Strategy<Value = CursorMovement> {
        prop_oneof![
            (1u32..100).prop_map(CursorMovement::Up),
            (1u32..100).prop_map(CursorMovement::Down),
            (1u32..100).prop_map(CursorMovement::Forward),
            (1u32..100).prop_map(CursorMovement::Backward),
            (1u32..100).prop_map(CursorMovement::NextLine),
            (1u32..100).prop_map(CursorMovement::PreviousLine),
            (1u32..160).prop_map(CursorMovement::ColumnAbsolute),
            (1u32..100, 1u32..160).prop_map(|(r, c)| CursorMovement::Position(r, c)),
        ]
    }

    fn arb_erase_mode() -> impl Strategy<Value = EraseMode> {
        prop_oneof![
            Just(EraseMode::CursorToEnd),
            Just(EraseMode::CursorToBeginning),
            Just(EraseMode::Entire),
            Just(EraseMode::CursorToEndLines),
            Just(EraseMode::CursorToBeginningLines),
        ]
    }

    fn arb_sgr_attribute() -> impl Strategy<Value = SgrAttribute> {
        prop_oneof![
            Just(SgrAttribute::Reset),
            Just(SgrAttribute::Bold),
            Just(SgrAttribute::Dim),
            Just(SgrAttribute::Italic),
            Just(SgrAttribute::Underline),
            Just(SgrAttribute::Blink),
            Just(SgrAttribute::Inverse),
            Just(SgrAttribute::Hidden),
            Just(SgrAttribute::Strikethrough),
            Just(SgrAttribute::Foreground(bettertui_engine::ansi::ForegroundColor::Default,)),
            Just(SgrAttribute::Background(bettertui_engine::ansi::BackgroundColor::Default,)),
        ]
    }

    fn arb_parser_event() -> impl Strategy<Value = ParserEvent> {
        prop_oneof![
            (0u8..0x7f).prop_map(ParserEvent::Char),
            Just(ParserEvent::Backspace),
            Just(ParserEvent::Tab),
            Just(ParserEvent::LineFeed),
            Just(ParserEvent::CarriageReturn),
            Just(ParserEvent::Bell),
            arb_csi(),
            Just(ParserEvent::Index),
            Just(ParserEvent::ReverseIndex),
            Just(ParserEvent::NextLine),
            Just(ParserEvent::Reset),
        ]
    }

    fn arb_csi() -> impl Strategy<Value = ParserEvent> {
        prop_oneof![
            arb_cursor_movement().prop_map(|m| ParserEvent::Csi(bettertui_engine::ansi::CsiCommand::CursorMovement(m))),
            arb_erase_mode().prop_map(|e| ParserEvent::Csi(bettertui_engine::ansi::CsiCommand::Erase(e))),
            (0u32..5).prop_map(|n| ParserEvent::Csi(bettertui_engine::ansi::CsiCommand::DeleteLine(n))),
            (0u32..5).prop_map(|n| ParserEvent::Csi(bettertui_engine::ansi::CsiCommand::InsertLine(n))),
            (0u32..5).prop_map(|n| ParserEvent::Csi(bettertui_engine::ansi::CsiCommand::DeleteChar(n))),
            (0u32..5).prop_map(|n| ParserEvent::Csi(bettertui_engine::ansi::CsiCommand::InsertChar(n))),
            (0u32..5).prop_map(|n| ParserEvent::Csi(bettertui_engine::ansi::CsiCommand::EraseChar(n))),
            Just(ParserEvent::Csi(bettertui_engine::ansi::CsiCommand::CursorPositionSave)),
            Just(ParserEvent::Csi(bettertui_engine::ansi::CsiCommand::CursorPositionRestore)),
            Just(ParserEvent::Csi(bettertui_engine::ansi::CsiCommand::AttributeReset)),
            (prop::collection::vec(arb_sgr_attribute(), 0..10))
                .prop_map(|attrs| ParserEvent::Csi(bettertui_engine::ansi::CsiCommand::Sgr(attrs))),
        ]
    }

    // ── Cursor bounds properties ──

    proptest! {
        #[test]
        fn cursor_stays_in_bounds_after_any_movement(
            start_row in 0u16..50,
            start_col in 0u16..100,
            max_row in 2u16..50,
            max_col in 2u16..100,
            moves in prop::collection::vec(arb_cursor_movement(), 0..50),
        ) {
            let mut c = Cursor::new();
            c.set_position(start_row.min(max_row.saturating_sub(1)), start_col.min(max_col.saturating_sub(1)));

            for m in &moves {
                match m {
                    CursorMovement::Up(n) => c.move_up(*n as u16),
                    CursorMovement::Down(n) => c.move_down(*n as u16, max_row),
                    CursorMovement::Forward(n) => c.move_right(*n as u16, max_col),
                    CursorMovement::Backward(n) => c.move_left(*n as u16),
                    CursorMovement::ColumnAbsolute(n) => {
                        c.move_to_column((*n as u16).min(max_col));
                    }
                    CursorMovement::Position(r, col) => {
                        c.move_to((*r as u16).min(max_row), (*col as u16).min(max_col));
                    }
                    CursorMovement::NextLine(n) => {
                        c.carriage_return();
                        c.move_down(*n as u16, max_row);
                    }
                    CursorMovement::PreviousLine(n) => {
                        c.carriage_return();
                        c.move_up(*n as u16);
                    }
                }

                assert!(
                    c.row() < max_row,
                    "row {} should be < max_row {} after movement {:?}",
                    c.row(), max_row, m
                );
                assert!(
                    c.col() < max_col,
                    "col {} should be < max_col {} after movement {:?}",
                    c.col(), max_col, m
                );
            }
        }
    }

    proptest! {
        #[test]
        fn cursor_save_restore_invariant(
            moves in prop::collection::vec(arb_cursor_movement(), 0..20),
        ) {
            let mut c = Cursor::new();
            c.set_position(10, 20);
            c.save_position();

            for m in &moves {
                match m {
                    CursorMovement::Up(n) => c.move_up(*n as u16),
                    CursorMovement::Down(n) => c.move_down(*n as u16, 50),
                    CursorMovement::Forward(n) => c.move_right(*n as u16, 80),
                    CursorMovement::Backward(n) => c.move_left(*n as u16),
                    _ => {}
                }
            }

            c.restore_position();
            assert_eq!(c.row(), 10, "saved row should be restored");
            assert_eq!(c.col(), 20, "saved col should be restored");
        }
    }

    proptest! {
        #[test]
        fn cursor_tab_in_bounds(
            start_col in 0u16..150,
            max_col in 8u16..160,
        ) {
            let mut c = Cursor::new();
            let pos = start_col.min(max_col.saturating_sub(1));
            c.set_position(0, pos);
            let tab_stops: Vec<u16> = (8..max_col).step_by(8).collect();

            c.tab(&tab_stops);

            assert!(
                c.col() >= pos,
                "tab moved backward: col={} < pos={}",
                c.col(), pos
            );
            assert!(
                tab_stops.contains(&c.col()) || c.col().is_multiple_of(8),
                "tab landed on invalid column: col={}, stops={:?}",
                c.col(), tab_stops
            );
        }
    }

    // ── TerminalMode properties ──

    proptest! {
        #[test]
        fn terminal_mode_insert_remove_is_idempotent(
            bits: u32
        ) {
            let mut m = TerminalMode::default();
            let mode = TerminalMode::from_bits_truncate(bits);

            m.insert(mode);
            let after_insert = m;
            m.insert(mode);
            assert_eq!(m, after_insert, "inserting same mode twice is idempotent");

            m.remove(mode);
            let after_remove = m;
            m.remove(mode);
            assert_eq!(m, after_remove, "removing same mode twice is idempotent");
        }
    }

    proptest! {
        #[test]
        fn terminal_mode_toggle_twice_restores(
            bits: u32
        ) {
            let mut m = TerminalMode::default();
            let before = m;
            let mode = TerminalMode::from_bits_truncate(bits);

            m.toggle(mode);
            m.toggle(mode);
            assert_eq!(m, before, "toggle twice should restore original");
        }
    }

    proptest! {
        #[test]
        fn terminal_mode_contains_implies_no_remove(
            bits: u32
        ) {
            let mode = TerminalMode::from_bits_truncate(bits);
            let mut m = mode;

            assert!(m.contains(mode));
            m.remove(mode);
            assert!(!m.contains(mode), "remove should clear the flag");
        }
    }

    // ── VtMachine panic-free properties ──

    proptest! {
        #[test]
        fn vt_machine_never_panics_on_any_parser_event(
            events in prop::collection::vec(arb_parser_event(), 0..100),
        ) {
            let mut m = VtMachine::new(80, 24);
            for event in &events {
                m.process(event);
            }
            let fb = m.framebuffer();
            assert_eq!(fb.width(), 80);
            assert_eq!(fb.height(), 24);
        }
    }

    proptest! {
        #[test]
        fn vt_machine_never_panics_on_ansi_bytes(
            bytes in prop::collection::vec(any::<u8>(), 0..200),
        ) {
            let mut m = VtMachine::new(80, 24);
            let mut parser = AnsiParser::new();
            parser.feed(&bytes);
            while let Some(event) = parser.poll_event() {
                m.process(&event);
            }
            let fb = m.framebuffer();
            assert_eq!(fb.width(), 80);
            assert_eq!(fb.height(), 24);
        }
    }

    proptest! {
        #[test]
        fn vt_machine_resize_never_panics(
            events in prop::collection::vec(arb_parser_event(), 0..50),
            new_w in 1u16..200,
            new_h in 1u16..100,
        ) {
            let mut m = VtMachine::new(80, 24);
            for event in &events {
                m.process(event);
            }
            m.resize(new_w, new_h);
            let fb = m.framebuffer();
            assert_eq!(fb.width(), new_w);
            assert_eq!(fb.height(), new_h);
        }
    }

    // ── KittyKeyEvent properties ──

    proptest! {
        #[test]
        fn kitty_key_event_modifier_bit_mapping(
            keycode in 0u32..0x110000,
            modifiers in 0u32..16u32,
        ) {
            let ev = KittyKeyEvent {
                keycode,
                modifiers,
                event_type: KittyEventType::Press,
                associated_text: None,
            };
            let ki = ev.to_keyboard_input();

            let has_shift = modifiers & 1 != 0;
            let has_alt = modifiers & 2 != 0;
            let has_ctrl = modifiers & 4 != 0;
            let has_super = modifiers & 8 != 0;

            assert_eq!(ki.modifiers.contains(KeyModifiers::SHIFT), has_shift);
            assert_eq!(ki.modifiers.contains(KeyModifiers::ALT), has_alt);
            assert_eq!(ki.modifiers.contains(KeyModifiers::CONTROL), has_ctrl);
            assert_eq!(ki.modifiers.contains(KeyModifiers::SUPER), has_super);
        }
    }

    proptest! {
        #[test]
        fn kitty_key_event_event_type_mapping(
            event_type in prop_oneof![
                Just(KittyEventType::Press),
                Just(KittyEventType::Repeat),
                Just(KittyEventType::Release),
                Just(KittyEventType::Unknown),
            ],
        ) {
            let ev = KittyKeyEvent {
                keycode: 65,
                modifiers: 0,
                event_type,
                associated_text: None,
            };
            let ki = ev.to_keyboard_input();

            match event_type {
                KittyEventType::Press | KittyEventType::Unknown => {
                    assert_eq!(ki.action, KeyAction::Press);
                }
                KittyEventType::Repeat => {
                    assert_eq!(ki.action, KeyAction::Repeat);
                }
                KittyEventType::Release => {
                    assert_eq!(ki.action, KeyAction::Release);
                }
            }
        }
    }

    proptest! {
        #[test]
        fn kitty_key_event_invalid_keycode_returns_null(
            keycode in 0x110001u32..0x200000u32,
        ) {
            let ev = KittyKeyEvent {
                keycode,
                modifiers: 0,
                event_type: KittyEventType::Press,
                associated_text: None,
            };
            let ki = ev.to_keyboard_input();
            assert_eq!(ki.key, '\0');
        }
    }
}
