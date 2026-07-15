#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

impl CursorPosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn zero() -> Self {
        Self { line: 0, column: 0 }
    }
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Debug, Clone)]
pub struct Cursor {
    position: usize,
    line: usize,
    column: usize,
    desired_column: usize,
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {
    pub fn new() -> Self {
        Self { position: 0, line: 0, column: 0, desired_column: 0 }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, pos: usize) {
        self.position = pos;
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn set_line(&mut self, line: usize) {
        self.line = line;
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn set_column(&mut self, column: usize) {
        self.column = column;
        self.desired_column = column;
    }

    pub fn cursor_position(&self) -> CursorPosition {
        CursorPosition::new(self.line, self.column)
    }

    pub fn move_left(&mut self) {
        if self.position > 0 {
            self.position -= 1;
            if self.column > 0 {
                self.column -= 1;
            }
        }
    }

    pub fn move_right(&mut self) {
        self.position += 1;
        self.column += 1;
    }

    pub fn move_left_by(&mut self, count: usize) {
        for _ in 0..count {
            self.move_left();
        }
    }

    pub fn move_right_by(&mut self, count: usize) {
        for _ in 0..count {
            self.move_right();
        }
    }

    pub fn move_up(&mut self, line_length: usize) {
        if self.line > 0 {
            self.line -= 1;
            self.column = self.desired_column;
            self.position -= line_length;
        }
    }

    pub fn move_down(&mut self, line_length: usize) {
        self.line += 1;
        self.column = self.desired_column;
        self.position += line_length;
    }

    pub fn move_to_start(&mut self) {
        self.position = 0;
        self.line = 0;
        self.column = 0;
        self.desired_column = 0;
    }

    pub fn move_to_end(&mut self, total_length: usize) {
        self.position = total_length;
        self.column = total_length;
        self.desired_column = total_length;
    }

    pub fn reset(&mut self) {
        self.position = 0;
        self.line = 0;
        self.column = 0;
        self.desired_column = 0;
    }

    pub fn is_at_start(&self) -> bool {
        self.position == 0
    }

    pub fn is_at_end(&self, total_length: usize) -> bool {
        self.position >= total_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_new() {
        let cursor = Cursor::new();
        assert_eq!(cursor.position(), 0);
    }

    #[test]
    fn cursor_default() {
        let cursor = Cursor::default();
        assert_eq!(cursor.position(), 0);
    }

    #[test]
    fn cursor_move_left() {
        let mut cursor = Cursor::new();
        cursor.move_right();
        cursor.move_left();
        assert_eq!(cursor.position(), 0);
    }

    #[test]
    fn cursor_move_right() {
        let mut cursor = Cursor::new();
        cursor.move_right();
        assert_eq!(cursor.position(), 1);
    }

    #[test]
    fn cursor_move_to_start() {
        let mut cursor = Cursor::new();
        cursor.move_right();
        cursor.move_right();
        cursor.move_to_start();
        assert_eq!(cursor.position(), 0);
    }

    #[test]
    fn cursor_move_to_end() {
        let mut cursor = Cursor::new();
        cursor.move_to_end(10);
        assert_eq!(cursor.position(), 10);
    }

    #[test]
    fn cursor_is_at_start() {
        let cursor = Cursor::new();
        assert!(cursor.is_at_start());
    }

    #[test]
    fn cursor_is_at_end() {
        let mut cursor = Cursor::new();
        cursor.move_to_end(10);
        assert!(cursor.is_at_end(10));
    }

    #[test]
    fn cursor_position_new() {
        let pos = CursorPosition::new(5, 10);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.column, 10);
    }

    #[test]
    fn cursor_position_zero() {
        let pos = CursorPosition::zero();
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 0);
    }
}
