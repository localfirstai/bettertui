use crate::dirty_diff::DirtyRegion;
use crate::framebuffer::{Cell, CellAttributes, FrameBuffer};
use crate::renderer::backend::RenderBackend;
use crate::tree::color::{Color, NamedColor};

pub struct AnsiBackend {
    buffer: Vec<u8>,
}

impl Default for AnsiBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl AnsiBackend {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(4096),
        }
    }

    fn encode_region(&mut self, buffer: &FrameBuffer, region: &DirtyRegion) {
        for y in region.y..region.y + region.height {
            self.move_to(region.x, y);
            let mut last_fg: Option<Color> = None;
            let mut last_bg: Option<Color> = None;
            let mut last_attrs: Option<CellAttributes> = None;

            for x in region.x..region.x + region.width {
                let cell = buffer.get(x, y);
                self.encode_cell(cell, &mut last_fg, &mut last_bg, &mut last_attrs);
            }
        }
    }

    fn encode_cell(
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

    fn begin_sgr(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[");
    }

    fn end_sgr(&mut self) {
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

    fn push_param(&mut self, n: u32) {
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

    fn push_char(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        let s = ch.encode_utf8(&mut buf);
        self.buffer.extend_from_slice(s.as_bytes());
    }

    fn move_to(&mut self, x: u16, y: u16) {
        self.buffer.extend_from_slice(b"\x1b[");
        let y_str = format!("{}", y + 1);
        let x_str = format!("{}", x + 1);
        self.buffer.extend_from_slice(y_str.as_bytes());
        self.buffer.push(b';');
        self.buffer.extend_from_slice(x_str.as_bytes());
        self.buffer.push(b'H');
    }

    fn hide_cursor(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?25l");
    }

    fn show_cursor(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?25h");
    }

    pub fn reset_sgr(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[0m");
    }
}

impl RenderBackend for AnsiBackend {
    fn encode(&mut self, buffer: &FrameBuffer, regions: &[DirtyRegion]) {
        self.buffer.clear();
        self.hide_cursor();

        for region in regions {
            self.encode_region(buffer, region);
        }

        self.show_cursor();
    }

    fn finish(&self) -> &[u8] {
        &self.buffer
    }

    fn reset(&mut self) {
        self.buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ansi_backend_new() {
        let backend = AnsiBackend::new();
        assert!(backend.finish().is_empty());
    }

    #[test]
    fn ansi_backend_encode_empty() {
        let mut backend = AnsiBackend::new();
        let fb = FrameBuffer::new(5, 5);
        backend.encode(&fb, &[]);
        let out = backend.finish();
        let s = String::from_utf8_lossy(out);
        assert!(s.contains("\x1b[?25l"));
        assert!(s.contains("\x1b[?25h"));
    }

    #[test]
    fn ansi_backend_move_to() {
        let mut backend = AnsiBackend::new();
        backend.move_to(0, 0);
        let out = backend.finish();
        assert!(out.windows(4).any(|w| w == b"1;1H"));
    }

    #[test]
    fn ansi_backend_move_to_offset() {
        let mut backend = AnsiBackend::new();
        backend.move_to(9, 4);
        let out = backend.finish();
        let s = String::from_utf8_lossy(out);
        assert!(s.contains("5;10H"));
    }

    #[test]
    fn ansi_backend_push_char() {
        let mut backend = AnsiBackend::new();
        backend.push_char('A');
        let out = backend.finish();
        assert_eq!(out, b"A");
    }

    #[test]
    fn ansi_backend_sgr_bold() {
        let mut backend = AnsiBackend::new();
        backend.begin_sgr();
        backend.push_param(1);
        backend.end_sgr();
        backend.push_char('X');
        let out = backend.finish();
        let s = String::from_utf8_lossy(out);
        assert!(s.starts_with("\x1b["));
        assert!(s.contains('1'));
        assert!(s.ends_with('X'));
    }

    #[test]
    fn ansi_backend_full_cell() {
        use crate::tree::color::NamedColor;
        let mut backend = AnsiBackend::new();
        let cell = Cell::new('Z')
            .with_fg(Color::Named(NamedColor::Red))
            .with_bg(Color::Named(NamedColor::Blue))
            .with_attrs(CellAttributes::BOLD);
        let mut last_fg = None;
        let mut last_bg = None;
        let mut last_attrs = None;
        backend.encode_cell(&cell, &mut last_fg, &mut last_bg, &mut last_attrs);
        let out = backend.finish();
        let s = String::from_utf8_lossy(out);
        assert!(s.contains("31"));
        assert!(s.contains("44"));
        assert!(s.contains("1"));
        assert!(s.ends_with('Z'));
    }

    #[test]
    fn ansi_backend_style_coalescing() {
        let mut backend = AnsiBackend::new();
        let cell = Cell::new('A').with_fg(Color::Named(NamedColor::Red));
        let mut last_fg = Some(Color::Named(NamedColor::Red));
        let mut last_bg = None;
        let mut last_attrs = None;
        backend.encode_cell(&cell, &mut last_fg, &mut last_bg, &mut last_attrs);
        let len1 = backend.finish().len();
        let mut backend2 = AnsiBackend::new();
        let cell2 = Cell::new('B').with_fg(Color::Named(NamedColor::Red));
        let mut last_fg2 = Some(Color::Named(NamedColor::Red));
        backend2.encode_cell(&cell2, &mut last_fg2, &mut last_bg, &mut last_attrs);
        let len2 = backend2.finish().len();
        assert!(len1 > len2);
    }

    #[test]
    fn ansi_backend_hide_show_cursor() {
        let mut backend = AnsiBackend::new();
        backend.hide_cursor();
        backend.show_cursor();
        let out = backend.finish();
        let s = String::from_utf8_lossy(out);
        assert!(s.contains("\x1b[?25l"));
        assert!(s.contains("\x1b[?25h"));
    }

    #[test]
    fn ansi_backend_reset_sgr() {
        let mut backend = AnsiBackend::new();
        backend.reset_sgr();
        let out = backend.finish();
        assert_eq!(out, b"\x1b[0m");
    }

    #[test]
    fn ansi_backend_reset() {
        let mut backend = AnsiBackend::new();
        backend.push_char('X');
        backend.reset();
        assert!(backend.finish().is_empty());
    }

    #[test]
    fn ansi_backend_region() {
        let mut backend = AnsiBackend::new();
        let mut fb = FrameBuffer::new(5, 3);
        fb.set(1, 1, Cell::new('H'));
        let region = DirtyRegion::new(0, 0, 5, 3);
        backend.encode_region(&fb, &region);
        let out = backend.finish();
        let s = String::from_utf8_lossy(out);
        assert!(s.contains('H'));
    }
}
