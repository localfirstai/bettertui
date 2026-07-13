use crate::framebuffer::{Cell, CellAttributes, FrameBuffer};
use crate::tree::Color;

const DEFAULT_SCROLLBACK_LINES: usize = 10000;

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
                // Cursor to end of screen
                for y in cursor_row..rows {
                    let start_col = if y == cursor_row { cursor_col } else { 0 };
                    for x in start_col..cols {
                        self.erase_char(y, x, pen);
                    }
                }
            }
            1 => {
                // Beginning to cursor
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
                // Entire display (3 also clears scrollback)
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

        // Push scrolled-out rows to scrollback
        for y in 0..count {
            let mut line = Vec::with_capacity(cols as usize);
            for x in 0..cols {
                let cell = self.buffer.get(x, y);
                line.push(cell);
            }
            self.scrollback.push_line(line);
        }

        // Shift rows up
        for y in count..rows {
            for x in 0..cols {
                let src = self.buffer.get(x, y);
                self.buffer.set(x, y - count, src);
            }
        }

        // Clear bottom rows
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

        // Shift rows down
        for y in (count..rows).rev() {
            for x in 0..cols {
                let src = self.buffer.get(x, y - count);
                self.buffer.set(x, y, src);
            }
        }

        // Clear top rows
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

        // Shift rows down from bottom
        for y in ((row + count)..rows).rev() {
            for x in 0..cols {
                let src = self.buffer.get(x, y - count);
                self.buffer.set(x, y, src);
            }
        }

        // Clear inserted rows
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

        // Shift rows up
        for y in (row + count)..rows {
            for x in 0..cols {
                let src = self.buffer.get(x, y);
                self.buffer.set(x, y - count, src);
            }
        }

        // Clear bottom rows
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

        // Shift chars right
        for x in ((col + count)..cols).rev() {
            let src = self.buffer.get(x - count, row);
            self.buffer.set(x, row, src);
        }

        // Clear inserted chars
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

        // Shift chars left
        for x in (col + count)..cols {
            let src = self.buffer.get(x, row);
            self.buffer.set(x - count, row, src);
        }

        // Clear rightmost chars
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pen() -> Pen {
        Pen::default()
    }

    #[test]
    fn screen_buffer_new() {
        let sb = ScreenBuffer::new(80, 24);
        assert_eq!(sb.width(), 80);
        assert_eq!(sb.height(), 24);
    }

    #[test]
    fn screen_write_char() {
        let mut sb = ScreenBuffer::new(10, 5);
        let pen = make_pen();
        sb.write_char(2, 3, 'X', &pen);
        assert_eq!(sb.buffer().get(3, 2).ch, 'X');
    }

    #[test]
    fn screen_scroll_up() {
        let mut sb = ScreenBuffer::new(10, 5);
        let pen = make_pen();
        sb.write_char(0, 0, 'A', &pen);
        sb.write_char(1, 0, 'B', &pen);
        sb.scroll_up(1, &pen);
        assert_eq!(sb.buffer().get(0, 0).ch, 'B');
        assert_eq!(sb.buffer().get(0, 4).ch, ' ');
    }

    #[test]
    fn screen_scroll_down() {
        let mut sb = ScreenBuffer::new(10, 5);
        let pen = make_pen();
        sb.write_char(3, 0, 'X', &pen);
        sb.scroll_down(2, &pen);
        assert_eq!(sb.buffer().get(0, 0).ch, ' ');
        assert_eq!(sb.buffer().get(0, 0).ch, ' ');
    }

    #[test]
    fn screen_erase_in_display_cursor_to_end() {
        let mut sb = ScreenBuffer::new(10, 5);
        let pen = make_pen();
        sb.write_char(2, 0, 'X', &pen);
        sb.write_char(3, 0, 'Y', &pen);
        sb.erase_in_display(0, 2, 0, &pen);
        assert_eq!(sb.buffer().get(0, 2).ch, ' ');
        assert_eq!(sb.buffer().get(0, 3).ch, ' ');
    }

    #[test]
    fn screen_erase_in_display_beginning_to_cursor() {
        let mut sb = ScreenBuffer::new(10, 5);
        let pen = make_pen();
        sb.write_char(0, 0, 'A', &pen);
        sb.write_char(1, 0, 'B', &pen);
        sb.erase_in_display(1, 1, 5, &pen);
        assert_eq!(sb.buffer().get(0, 0).ch, ' ');
        assert_eq!(sb.buffer().get(5, 1).ch, ' ');
    }

    #[test]
    fn screen_erase_in_display_entire() {
        let mut sb = ScreenBuffer::new(10, 5);
        let pen = make_pen();
        sb.write_char(0, 0, 'A', &pen);
        sb.write_char(4, 9, 'Z', &pen);
        sb.erase_in_display(2, 0, 0, &pen);
        assert!(sb.buffer().get(0, 0).is_empty());
        assert!(sb.buffer().get(9, 4).is_empty());
    }

    #[test]
    fn screen_insert_lines() {
        let mut sb = ScreenBuffer::new(10, 5);
        let pen = make_pen();
        sb.write_char(1, 0, 'A', &pen);
        sb.write_char(2, 0, 'B', &pen);
        sb.insert_lines(1, 2, &pen);
        assert_eq!(sb.buffer().get(0, 1).ch, ' ');
        assert_eq!(sb.buffer().get(0, 3).ch, 'A');
        assert_eq!(sb.buffer().get(0, 4).ch, 'B');
    }

    #[test]
    fn screen_delete_lines() {
        let mut sb = ScreenBuffer::new(10, 5);
        let pen = make_pen();
        sb.write_char(0, 0, 'A', &pen);
        sb.write_char(1, 0, 'B', &pen);
        sb.write_char(2, 0, 'C', &pen);
        sb.delete_lines(0, 2, &pen);
        assert_eq!(sb.buffer().get(0, 0).ch, 'C');
    }

    #[test]
    fn screen_insert_chars() {
        let mut sb = ScreenBuffer::new(10, 5);
        let pen = make_pen();
        sb.write_char(0, 5, 'A', &pen);
        sb.insert_chars(0, 2, 3, &pen);
        assert_eq!(sb.buffer().get(2, 0).ch, ' ');
        assert_eq!(sb.buffer().get(8, 0).ch, 'A');
    }

    #[test]
    fn screen_delete_chars() {
        let mut sb = ScreenBuffer::new(10, 5);
        let pen = make_pen();
        sb.write_char(0, 2, 'D', &pen);
        sb.write_char(0, 5, 'E', &pen);
        sb.delete_chars(0, 2, 3, &pen);
        assert_eq!(sb.buffer().get(2, 0).ch, 'E');
    }

    #[test]
    fn screen_tab_stops() {
        let mut sb = ScreenBuffer::new(80, 24);
        assert!(sb.tab_stops().contains(&8));
        assert!(sb.tab_stops().contains(&16));
        sb.set_tab_stop(5);
        assert!(sb.tab_stops().contains(&5));
        sb.clear_tab_stop(8);
        assert!(!sb.tab_stops().contains(&8));
    }

    #[test]
    fn screen_scrollback() {
        let mut sb = ScreenBuffer::new(10, 3);
        let pen = make_pen();
        sb.write_char(0, 0, 'A', &pen);
        sb.write_char(0, 1, 'B', &pen);
        sb.write_char(0, 2, 'C', &pen);
        sb.scroll_up(2, &pen);
        assert!(sb.scrollback().line_count() == 2);
    }
}
