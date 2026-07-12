use crate::ansi::parser::{
    BackgroundColor, CsiCommand, CursorMovement, EraseMode, ForegroundColor, KittyEventType,
    ModeAction, ModeType, OscCommand, ParserEvent, ScrollDirection, SgrAttribute, TabStopAction,
};
use crate::framebuffer::CellAttributes;
use crate::input::{KeyAction, KeyModifiers, KeyboardInput};
use crate::tree::color::{Color, NamedColor};

use super::cursor::Cursor;
use super::modes::PrivateMode;
use super::modes::TerminalMode;
use super::screen::{Pen, ScreenBuffer};

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
    /// Convert this Kitty keyboard event to a `KeyboardInput` for the input system.
    /// Kitty modifier bits: Shift=1, Alt=2, Ctrl=4, Super=8
    /// App KeyModifiers:    Shift=1, Ctrl=2, Alt=4, Super=8
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ansi::parser::AnsiParser;

    #[test]
    fn machine_new() {
        let m = VtMachine::new(80, 24);
        assert_eq!(m.screen.width(), 80);
        assert_eq!(m.screen.height(), 24);
        assert!(m.modes.auto_wrap());
    }

    #[test]
    fn machine_write_text() {
        let mut m = VtMachine::new(10, 5);
        let mut p = AnsiParser::new();
        p.feed(b"Hello");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        assert_eq!(m.framebuffer().get(0, 0).ch, 'H');
        assert_eq!(m.framebuffer().get(4, 0).ch, 'o');
    }

    #[test]
    fn machine_newline() {
        let mut m = VtMachine::new(10, 5);
        let mut p = AnsiParser::new();
        p.feed(b"AB\nCD");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        assert_eq!(m.framebuffer().get(0, 0).ch, 'A');
        assert_eq!(m.framebuffer().get(1, 0).ch, 'B');
        assert_eq!(m.framebuffer().get(0, 1).ch, 'C');
        assert_eq!(m.framebuffer().get(1, 1).ch, 'D');
    }

    #[test]
    fn machine_cursor_movement() {
        let mut m = VtMachine::new(10, 5);
        let mut p = AnsiParser::new();
        p.feed(b"\x1b[3;4HX");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        assert_eq!(m.framebuffer().get(3, 2).ch, 'X');
    }

    #[test]
    fn machine_sgr_colors() {
        let mut m = VtMachine::new(10, 5);
        let mut p = AnsiParser::new();
        p.feed(b"\x1b[31mR\x1b[0m");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        assert_eq!(m.framebuffer().get(0, 0).ch, 'R');
        assert_eq!(m.framebuffer().get(0, 0).fg, Color::Named(NamedColor::Red));
    }

    #[test]
    fn machine_sgr_bold() {
        let mut m = VtMachine::new(10, 5);
        let mut p = AnsiParser::new();
        p.feed(b"\x1b[1mB");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        let cell = m.framebuffer().get(0, 0);
        assert_eq!(cell.ch, 'B');
        assert!(cell.attributes.contains(CellAttributes::BOLD));
    }

    #[test]
    fn machine_erase_display() {
        let mut m = VtMachine::new(10, 5);
        let mut p = AnsiParser::new();
        p.feed(b"AB\x1b[2J");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        assert!(m.framebuffer().get(0, 0).is_empty());
        assert!(m.framebuffer().get(1, 0).is_empty());
    }

    #[test]
    fn machine_scroll() {
        let mut m = VtMachine::new(10, 3);
        let mut p = AnsiParser::new();
        p.feed(b"Line1\nLine2\nLine3\nLine4");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        assert_eq!(m.framebuffer().get(0, 0).ch, 'L');
        assert_eq!(m.framebuffer().get(4, 0).ch, '2');
        assert_eq!(m.framebuffer().get(0, 1).ch, 'L');
        assert_eq!(m.framebuffer().get(4, 1).ch, '3');
        assert_eq!(m.framebuffer().get(0, 2).ch, 'L');
        assert_eq!(m.framebuffer().get(4, 2).ch, '4');
    }

    #[test]
    fn machine_reset() {
        let mut m = VtMachine::new(80, 24);
        let mut p = AnsiParser::new();
        p.feed(b"\x1b[31mX");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        p.feed(b"\x1bc");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        let cell = m.framebuffer().get(0, 0);
        assert_eq!(cell.fg, Color::Default);
    }

    #[test]
    fn machine_osc_title() {
        let mut m = VtMachine::new(80, 24);
        let mut p = AnsiParser::new();
        p.feed(b"\x1b]2;My Terminal\x07");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        assert_eq!(m.title, "My Terminal");
    }

    #[test]
    fn machine_carriage_return() {
        let mut m = VtMachine::new(10, 5);
        let mut p = AnsiParser::new();
        p.feed(b"Hello\rX");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        assert_eq!(m.framebuffer().get(0, 0).ch, 'X');
        assert_eq!(m.framebuffer().get(1, 0).ch, 'e');
    }

    #[test]
    fn machine_tab() {
        let mut m = VtMachine::new(20, 5);
        let mut p = AnsiParser::new();
        p.feed(b"\tX");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        assert_eq!(m.framebuffer().get(8, 0).ch, 'X');
    }

    #[test]
    fn machine_alternate_screen() {
        let mut m = VtMachine::new(10, 5);
        let mut p = AnsiParser::new();
        p.feed(b"Main");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        assert_eq!(m.framebuffer().get(0, 0).ch, 'M');

        let mut p2 = AnsiParser::new();
        p2.feed(b"\x1b[?1049hAlt");
        while let Some(event) = p2.poll_event() {
            m.process(&event);
        }
        assert!(m.modes.alt_screen());
        assert_eq!(m.framebuffer().get(0, 0).ch, 'A');

        let mut p3 = AnsiParser::new();
        p3.feed(b"\x1b[?1049l");
        while let Some(event) = p3.poll_event() {
            m.process(&event);
        }
        assert!(!m.modes.alt_screen());
        assert_eq!(m.framebuffer().get(0, 0).ch, 'M');
    }

    #[test]
    fn kitty_key_event_to_keyboard_input_press() {
        let ev = KittyKeyEvent {
            keycode: 97,
            modifiers: 0,
            event_type: KittyEventType::Press,
            associated_text: None,
        };
        let ki = ev.to_keyboard_input();
        assert_eq!(ki.key, 'a');
        assert_eq!(ki.modifiers, KeyModifiers::empty());
        assert_eq!(ki.action, KeyAction::Press);
    }

    #[test]
    fn kitty_key_event_to_keyboard_input_modifiers() {
        let ev = KittyKeyEvent {
            keycode: 65,
            modifiers: 1 | 4, // Shift=1, Ctrl=4
            event_type: KittyEventType::Repeat,
            associated_text: None,
        };
        let ki = ev.to_keyboard_input();
        assert_eq!(ki.key, 'A');
        assert!(ki.modifiers.contains(KeyModifiers::SHIFT));
        assert!(ki.modifiers.contains(KeyModifiers::CONTROL));
        assert!(!ki.modifiers.contains(KeyModifiers::ALT));
        assert!(!ki.modifiers.contains(KeyModifiers::SUPER));
        assert_eq!(ki.action, KeyAction::Repeat);
    }

    #[test]
    fn kitty_key_event_to_keyboard_input_alt_super() {
        let ev = KittyKeyEvent {
            keycode: 98,
            modifiers: 2 | 8, // Alt=2, Super=8
            event_type: KittyEventType::Release,
            associated_text: None,
        };
        let ki = ev.to_keyboard_input();
        assert_eq!(ki.key, 'b');
        assert!(!ki.modifiers.contains(KeyModifiers::SHIFT));
        assert!(!ki.modifiers.contains(KeyModifiers::CONTROL));
        assert!(ki.modifiers.contains(KeyModifiers::ALT));
        assert!(ki.modifiers.contains(KeyModifiers::SUPER));
        assert_eq!(ki.action, KeyAction::Release);
    }

    #[test]
    fn kitty_key_event_to_keyboard_input_unknown_type() {
        let ev = KittyKeyEvent {
            keycode: 99,
            modifiers: 0,
            event_type: KittyEventType::Unknown,
            associated_text: None,
        };
        let ki = ev.to_keyboard_input();
        assert_eq!(ki.key, 'c');
        assert_eq!(ki.action, KeyAction::Press);
    }

    #[test]
    fn kitty_key_event_to_keyboard_input_invalid_keycode() {
        let ev = KittyKeyEvent {
            keycode: 0x110000, // beyond valid Unicode
            modifiers: 0,
            event_type: KittyEventType::Press,
            associated_text: None,
        };
        let ki = ev.to_keyboard_input();
        assert_eq!(ki.key, '\0');
    }

    #[test]
    fn machine_true_color_sgr() {
        let mut m = VtMachine::new(10, 5);
        let mut p = AnsiParser::new();
        p.feed(b"\x1b[38;2;255;128;64mC");
        while let Some(event) = p.poll_event() {
            m.process(&event);
        }
        assert_eq!(
            m.framebuffer().get(0, 0).fg,
            Color::Rgb {
                r: 255,
                g: 128,
                b: 64
            }
        );
    }
}
