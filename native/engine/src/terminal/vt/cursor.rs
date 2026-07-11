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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_new() {
        let c = Cursor::new();
        assert_eq!(c.position(), (0, 0));
        assert!(c.visible);
    }

    #[test]
    fn cursor_move_up() {
        let mut c = Cursor::new();
        c.set_position(5, 5);
        c.move_up(2);
        assert_eq!(c.row, 3);
    }

    #[test]
    fn cursor_move_up_saturating() {
        let mut c = Cursor::new();
        c.move_up(5);
        assert_eq!(c.row, 0);
    }

    #[test]
    fn cursor_move_down() {
        let mut c = Cursor::new();
        c.move_down(3, 24);
        assert_eq!(c.row, 3);
    }

    #[test]
    fn cursor_move_down_clamp() {
        let mut c = Cursor::new();
        c.set_position(20, 0);
        c.move_down(10, 24);
        assert_eq!(c.row, 23);
    }

    #[test]
    fn cursor_move_left_right() {
        let mut c = Cursor::new();
        c.set_position(0, 10);
        c.move_left(3);
        assert_eq!(c.col, 7);
        c.move_right(5, 80);
        assert_eq!(c.col, 12);
    }

    #[test]
    fn cursor_move_to_column() {
        let mut c = Cursor::new();
        c.move_to_column(5);
        assert_eq!(c.col, 4);
    }

    #[test]
    fn cursor_move_to() {
        let mut c = Cursor::new();
        c.move_to(10, 20);
        assert_eq!(c.row, 9);
        assert_eq!(c.col, 19);
    }

    #[test]
    fn cursor_save_restore() {
        let mut c = Cursor::new();
        c.set_position(5, 10);
        c.save_position();
        c.set_position(0, 0);
        c.restore_position();
        assert_eq!(c.position(), (5, 10));
    }

    #[test]
    fn cursor_carriage_return() {
        let mut c = Cursor::new();
        c.set_position(5, 20);
        c.carriage_return();
        assert_eq!(c.col, 0);
        assert_eq!(c.row, 5);
    }

    #[test]
    fn cursor_tab() {
        let mut c = Cursor::new();
        c.set_position(0, 3);
        c.tab(&[8, 16, 24]);
        assert_eq!(c.col, 8);
    }

    #[test]
    fn cursor_tab_next_stop() {
        let mut c = Cursor::new();
        c.set_position(0, 12);
        c.tab(&[8, 16, 24]);
        assert_eq!(c.col, 16);
    }

    #[test]
    fn cursor_tab_default() {
        let mut c = Cursor::new();
        c.set_position(0, 5);
        c.tab(&[]);
        assert_eq!(c.col, 8);
    }

    #[test]
    fn cursor_backspace() {
        let mut c = Cursor::new();
        c.set_position(0, 5);
        c.backspace();
        assert_eq!(c.col, 4);
    }

    #[test]
    fn cursor_backspace_saturating() {
        let mut c = Cursor::new();
        c.backspace();
        assert_eq!(c.col, 0);
    }
}
