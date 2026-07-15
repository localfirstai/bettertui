//! Terminal interaction: crossterm-based terminal event handling, queries, and VT emulation.

mod capabilities;
pub mod neovim;
pub mod process;

pub use capabilities::{
    CapabilityDetector, CjkWidth, ClipboardCapabilities, ColorSupport, EmojiWidth, FeatureMatrix, GraphicsCapabilities,
    InputCapabilities, MouseModes, QueryOrigin, RenderCapabilities, TerminalBrand, UnicodeCapabilities, UnicodeVersion,
    WindowMetrics, global_capabilities,
};
pub use process::{
    ProcessConfig, ProcessConfigBuilder, ProcessSpawner, ProcessStatus, ScrollMode, SpawnResult, TerminalError,
    TerminalRuntime, TerminalState, TerminalViewport,
};
pub mod query;
pub mod screen;
pub mod scrollback;
mod vt;

pub use screen::*;
pub use scrollback::*;
pub use vt::{
    Cursor, CursorShape, CursorStyle, KittyKeyEvent, Pen, PrivateMode, ResponseKind, ScreenBuffer, TerminalMode,
    TerminalResponse, VtMachine,
};

use std::io::{self, IsTerminal, Write, stdout};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, ClearType},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tracing::{debug, info, trace, warn};

pub struct Terminal {
    width: u16,
    height: u16,
    raw_mode: bool,
    alternate_screen: bool,
    is_tty: bool,
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

impl Terminal {
    pub fn new() -> Self {
        let (w, h) = terminal::size().unwrap_or((80, 24));
        let is_tty = std::io::stdin().is_terminal();
        info!(width = w, height = h, is_tty, "Terminal::new() - creating terminal");
        Self { width: w, height: h, raw_mode: false, alternate_screen: false, is_tty }
    }

    pub fn is_tty(&self) -> bool {
        self.is_tty
    }

    pub fn size(&self) -> (u16, u16) {
        trace!(width = self.width, height = self.height, "Terminal::size() - returning cached size");
        (self.width, self.height)
    }

    pub fn refresh_size(&mut self) -> io::Result<(u16, u16)> {
        let old_w = self.width;
        let old_h = self.height;
        let (w, h) = terminal::size()?;
        self.width = w;
        self.height = h;
        debug!(
            old_width = old_w,
            old_height = old_h,
            new_width = w,
            new_height = h,
            "Terminal::refresh_size() - refreshed terminal size"
        );
        Ok((w, h))
    }

    pub fn update_size(&mut self, width: u16, height: u16) {
        let old_w = self.width;
        let old_h = self.height;
        self.width = width;
        self.height = height;
        debug!(
            old_width = old_w,
            old_height = old_h,
            new_width = width,
            new_height = height,
            "Terminal::update_size() - updated from resize event"
        );
    }

    pub fn enter_raw_mode(&mut self) -> io::Result<()> {
        if !self.is_tty {
            warn!("Terminal::enter_raw_mode() - skipping raw mode (not a TTY)");
            return Ok(());
        }
        if !self.raw_mode {
            info!("Terminal::enter_raw_mode() - entering raw mode");
            enable_raw_mode()?;
            self.raw_mode = true;
        }
        Ok(())
    }

    pub fn leave_raw_mode(&mut self) -> io::Result<()> {
        if !self.is_tty {
            return Ok(());
        }
        if self.raw_mode {
            info!("Terminal::leave_raw_mode() - leaving raw mode");
            disable_raw_mode()?;
            self.raw_mode = false;
        }
        Ok(())
    }

    pub fn enter_alternate_screen(&mut self) -> io::Result<()> {
        if !self.alternate_screen {
            info!("Terminal::enter_alternate_screen() - entering alternate screen");
            execute!(stdout(), terminal::EnterAlternateScreen)?;
            self.alternate_screen = true;
        }
        Ok(())
    }

    pub fn leave_alternate_screen(&mut self) -> io::Result<()> {
        if self.alternate_screen {
            info!("Terminal::leave_alternate_screen() - leaving alternate screen");
            execute!(stdout(), terminal::LeaveAlternateScreen)?;
            self.alternate_screen = false;
        }
        Ok(())
    }

    pub fn clear(&self) -> io::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        Ok(())
    }

    pub fn hide_cursor(&self) -> io::Result<()> {
        execute!(stdout(), cursor::Hide)?;
        Ok(())
    }

    pub fn show_cursor(&self) -> io::Result<()> {
        execute!(stdout(), cursor::Show)?;
        Ok(())
    }

    pub fn move_cursor(&self, x: u16, y: u16) -> io::Result<()> {
        execute!(stdout(), cursor::MoveTo(x, y))?;
        Ok(())
    }

    pub fn write_bytes(&self, data: &[u8]) -> io::Result<()> {
        let mut out = stdout();
        out.write_all(data)?;
        out.flush()?;
        Ok(())
    }

    pub fn flush(&self) -> io::Result<()> {
        stdout().flush()?;
        Ok(())
    }

    pub fn poll_event(&self, timeout: std::time::Duration) -> io::Result<Option<TerminalEvent>> {
        if !self.is_tty {
            std::thread::sleep(timeout);
            return Ok(None);
        }
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key_event) => {
                    let key_input = KeyInput {
                        code: match key_event.code {
                            KeyCode::Char(c) => Key::Char(c),
                            KeyCode::Enter => Key::Enter,
                            KeyCode::Esc => Key::Esc,
                            KeyCode::Backspace => Key::Backspace,
                            KeyCode::Tab => Key::Tab,
                            KeyCode::Up => Key::Up,
                            KeyCode::Down => Key::Down,
                            KeyCode::Left => Key::Left,
                            KeyCode::Right => Key::Right,
                            KeyCode::Home => Key::Home,
                            KeyCode::End => Key::End,
                            KeyCode::PageUp => Key::PageUp,
                            KeyCode::PageDown => Key::PageDown,
                            KeyCode::F(n) => Key::F(n),
                            _ => Key::Other,
                        },
                        modifiers: KeyModifiers::from_bits_truncate(key_event.modifiers.bits()),
                    };
                    debug!(?key_input, "Terminal::poll_event() - key event received");
                    Ok(Some(TerminalEvent::Key(key_input)))
                }
                Event::Mouse(mouse) => {
                    debug!(?mouse, "Terminal::poll_event() - mouse event received");
                    Ok(Some(TerminalEvent::Mouse(mouse)))
                }
                Event::Resize(w, h) => {
                    debug!(width = w, height = h, "Terminal::poll_event() - resize event received");
                    Ok(Some(TerminalEvent::Resize(w, h)))
                }
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        info!("Terminal::drop() - cleaning up terminal state");
        let _ = self.leave_alternate_screen();
        let _ = self.leave_raw_mode();
        let _ = self.show_cursor();
    }
}

#[derive(Debug, Clone)]
pub enum TerminalEvent {
    Key(KeyInput),
    Mouse(crossterm::event::MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug, Clone)]
pub struct KeyInput {
    pub code: Key,
    pub modifiers: KeyModifiers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Esc,
    Backspace,
    Tab,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    F(u8),
    Other,
}

impl Key {
    pub fn is_char(&self) -> bool {
        matches!(self, Key::Char(_))
    }

    pub fn as_char(&self) -> Option<char> {
        if let Key::Char(c) = self { Some(*c) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_new() {
        let t = Terminal::new();
        assert!(t.size().0 > 0);
        assert!(t.size().1 > 0);
        assert!(!t.raw_mode);
        assert!(!t.alternate_screen);
    }

    #[test]
    fn terminal_size() {
        let t = Terminal::new();
        let (w, h) = t.size();
        assert!(w > 0);
        assert!(h > 0);
    }

    #[test]
    fn key_is_char() {
        assert!(Key::Char('a').is_char());
        assert!(!Key::Enter.is_char());
    }

    #[test]
    fn key_as_char() {
        assert_eq!(Key::Char('x').as_char(), Some('x'));
        assert_eq!(Key::Enter.as_char(), None);
    }

    #[test]
    fn key_equality() {
        assert_eq!(Key::Char('a'), Key::Char('a'));
        assert_ne!(Key::Char('a'), Key::Char('b'));
        assert_eq!(Key::Enter, Key::Enter);
    }

    #[test]
    fn terminal_event_clone() {
        let ev = TerminalEvent::Key(KeyInput { code: Key::Char('x'), modifiers: KeyModifiers::empty() });
        let ev2 = ev.clone();
        match ev2 {
            TerminalEvent::Key(k) => assert_eq!(k.code, Key::Char('x')),
            _ => panic!("wrong variant"),
        }
    }
}
