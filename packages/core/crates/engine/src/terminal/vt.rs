//! VT100/VTxxx terminal emulation state machine.

use crate::ansi::{
    BackgroundColor, CsiCommand, CursorMovement, EraseMode, ForegroundColor, KittyEventType,
    ModeAction, ModeType, OscCommand, ParserEvent, ScrollDirection, SgrAttribute, TabStopAction,
};
use crate::framebuffer::{Cell, CellAttributes, FrameBuffer};
use crate::input::{KeyAction, KeyModifiers, KeyboardInput};
use crate::tree::{Color, NamedColor};

const DEFAULT_SCROLLBACK_LINES: usize = 10000;

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

#[derive(Debug, Clone)]
pub struct Cursor {
    pub row: u16,
    pub col: u16,
    pub saved_row: u16,
    pub saved_col: u16,
    pub visible: bool,
    pub style: CursorStyle,
    pub shape: CursorShape,
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

    pub fn position(&self) -> (u16, u16) {
        (self.row, self.col)
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

// =============================================================================
// Screen
// =============================================================================

#[derive(Debug, Clone)]
pub struct ScrollbackBuffer {
    lines: Vec<Vec<Cell>>,
    max_lines: usize,
}

impl Default for ScrollbackBuffer {
    fn default() -> Self {
        Self::new(DEFAULT_SCROLLBACK_LINES)
    }
}

impl ScrollbackBuffer {
    pub fn new(max_lines: usize) -> Self {
        Self {
            lines: Vec::with_capacity(max_lines.min(1000)),
            max_lines,
        }
    }

    pub fn push_line(&mut self, line: Vec<Cell>) {
        if self.lines.len() >= self.max_lines {
            self.lines.remove(0);
        }
        self.lines.push(line);
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_line(&self, index: usize) -> Option<&[Cell]> {
        self.lines.get(index).map(|l| l.as_slice())
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }
}

#[derive(Debug, Clone)]
pub struct ScreenBuffer {
    buffer: FrameBuffer,
    tab_stops: Vec<u16>,
    scrollback: ScrollbackBuffer,
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
            scrollback: ScrollbackBuffer::default(),
            default_bg: Color::Default,
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer.resize(width, height);
        self.tab_stops.clear();
        for t in (8..width).step_by(8) {
            self.tab_stops.push(t);
        }
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

    pub fn scrollback(&self) -> &ScrollbackBuffer {
        &self.scrollback
    }

    pub fn set_cell(
        &mut self,
        row: u16,
        col: u16,
        ch: char,
        fg: Color,
        bg: Color,
        attrs: CellAttributes,
    ) {
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
        self.set_cell(
            row,
            col,
            ' ',
            Color::Default,
            pen.bg,
            CellAttributes::empty(),
        );
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
                    let end_col = if y == cursor_row {
                        cursor_col + 1
                    } else {
                        cols
                    };
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
            self.scrollback.push_line(line);
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
        Self {
            fg: Color::Default,
            bg: Color::Default,
            attrs: CellAttributes::empty(),
        }
    }
}

// =============================================================================
// VT Machine
// =============================================================================

#[derive(Debug, Clone)]
pub struct VtMachine {
    pub screen: ScreenBuffer,
    pub alt_screen: ScreenBuffer,
    pub cursor: Cursor,
    pub alt_cursor: Cursor,
    pub modes: TerminalMode,
    pub pen: Pen,
    pub title: String,
    pub icon_name: String,
    pub hyperlink: Option<(Option<String>, String)>,
    pub clipboard: Option<String>,
    pub device_attributes: Option<Vec<u32>>,
    pub secondary_device_attributes: Option<Vec<u32>>,
    pub tertiary_device_attributes: Option<String>,
    pub terminal_responses: Vec<TerminalResponse>,
    pub last_kitty_key: Option<KittyKeyEvent>,
    pub kitty_enhancement_levels: [u8; 5],
    pub kitty_keyboard_query_response: Option<Vec<u32>>,
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
        KeyboardInput {
            key,
            modifiers: mods,
            action,
        }
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

    fn screen(&self) -> &ScreenBuffer {
        if self.modes.alt_screen() {
            &self.alt_screen
        } else {
            &self.screen
        }
    }

    fn screen_mut(&mut self) -> &mut ScreenBuffer {
        if self.modes.alt_screen() {
            &mut self.alt_screen
        } else {
            &mut self.screen
        }
    }

    fn cursor(&self) -> &Cursor {
        if self.modes.alt_screen() {
            &self.alt_cursor
        } else {
            &self.cursor
        }
    }

    fn cursor_mut(&mut self) -> &mut Cursor {
        if self.modes.alt_screen() {
            &mut self.alt_cursor
        } else {
            &mut self.cursor
        }
    }

    pub fn framebuffer(&self) -> &crate::framebuffer::FrameBuffer {
        self.screen().buffer()
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
        let _ = screen;

        if cursor_col + 1 >= max_col {
            if auto_wrap {
                if cursor_row + 1 >= self.screen().height() {
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
        let height = self.screen().height();
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
        let height = self.screen().height();
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
        let tab_stops = self.screen().tab_stops().to_vec();
        self.cursor_mut().tab(&tab_stops);
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
            CsiCommand::KittyKeyEvent {
                keycode,
                modifiers,
                event_type,
                associated_text,
            } => {
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
        let screen_width = self.screen().width();
        let screen_height = self.screen().height();
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
            CursorMovement::ColumnAbsolute(n) => cursor.move_to_column(*n as u16),
            CursorMovement::Position(row, col) => {
                cursor.move_to(*row as u16, *col as u16);
                if origin {
                    cursor.row = cursor.row.min(screen_height.saturating_sub(1));
                }
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
                                self.alt_screen =
                                    ScreenBuffer::new(self.screen.width(), self.screen.height());
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
