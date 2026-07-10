use super::cell::Cell;

#[derive(Debug, Clone)]
pub struct FrameBuffer {
    width: u16,
    height: u16,
    cells: Vec<Cell>,
    back: Vec<Cell>,
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl FrameBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            cells: vec![Cell::default(); size],
            back: vec![Cell::default(); size],
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        let size = (width as usize) * (height as usize);
        self.width = width;
        self.height = height;
        self.cells.resize(size, Cell::default());
        self.back.resize(size, Cell::default());
        self.clear();
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }
    }

    pub fn index(&self, x: u16, y: u16) -> usize {
        (y as usize) * (self.width as usize) + (x as usize)
    }

    pub fn in_bounds(&self, x: u16, y: u16) -> bool {
        x < self.width && y < self.height
    }

    pub fn get(&self, x: u16, y: u16) -> &Cell {
        if self.in_bounds(x, y) {
            &self.cells[self.index(x, y)]
        } else {
            static EMPTY: Cell = Cell {
                ch: ' ',
                fg: crate::tree::color::Color::Default,
                bg: crate::tree::color::Color::Default,
                underline_color: crate::tree::color::Color::Default,
                attributes: super::cell::CellAttributes::empty(),
            };
            &EMPTY
        }
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if self.in_bounds(x, y) {
            let idx = self.index(x, y);
            self.cells[idx] = cell;
        }
    }

    pub fn fill_rect(&mut self, x: u16, y: u16, width: u16, height: u16, cell: Cell) {
        for dy in 0..height {
            let row = y + dy;
            if row >= self.height {
                break;
            }
            for dx in 0..width {
                let col = x + dx;
                if col >= self.width {
                    break;
                }
                self.set(col, row, cell);
            }
        }
    }

    pub fn write_str(
        &mut self,
        x: u16,
        y: u16,
        s: &str,
        fg: crate::tree::color::Color,
        bg: crate::tree::color::Color,
    ) {
        for (i, ch) in s.chars().enumerate() {
            let col = x + i as u16;
            if col >= self.width {
                break;
            }
            self.set(col, y, Cell::new(ch).with_fg(fg).with_bg(bg));
        }
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.cells, &mut self.back);
    }

    pub fn copy_from(&mut self, other: &FrameBuffer) {
        let len = (self.width as usize) * (self.height as usize);
        let other_len = (other.width as usize) * (other.height as usize);
        let copy_len = len.min(other_len);
        self.cells[..copy_len].copy_from_slice(&other.cells[..copy_len]);
    }

    pub fn diff(&self) -> Vec<(u16, u16)> {
        let mut dirty = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.index(x, y);
                if self.cells[idx] != self.back[idx] {
                    dirty.push((x, y));
                }
            }
        }
        dirty
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn framebuffer_new() {
        let fb = FrameBuffer::new(80, 24);
        assert_eq!(fb.width(), 80);
        assert_eq!(fb.height(), 24);
    }

    #[test]
    fn framebuffer_in_bounds() {
        let fb = FrameBuffer::new(80, 24);
        assert!(fb.in_bounds(0, 0));
        assert!(fb.in_bounds(79, 23));
        assert!(!fb.in_bounds(80, 24));
        assert!(!fb.in_bounds(80, 0));
    }

    #[test]
    fn framebuffer_set_get() {
        let mut fb = FrameBuffer::new(10, 5);
        let cell = Cell::new('X');
        fb.set(3, 2, cell);
        assert_eq!(fb.get(3, 2).ch, 'X');
    }

    #[test]
    fn framebuffer_fill_rect() {
        let mut fb = FrameBuffer::new(10, 5);
        let cell = Cell::new('#');
        fb.fill_rect(2, 1, 3, 2, cell);
        assert_eq!(fb.get(2, 1).ch, '#');
        assert_eq!(fb.get(4, 1).ch, '#');
        assert_eq!(fb.get(2, 2).ch, '#');
        assert_eq!(fb.get(5, 1).ch, ' ');
    }

    #[test]
    fn framebuffer_write_str() {
        let mut fb = FrameBuffer::new(10, 5);
        use crate::tree::color::Color;
        fb.write_str(1, 0, "Hello", Color::Default, Color::Default);
        assert_eq!(fb.get(1, 0).ch, 'H');
        assert_eq!(fb.get(5, 0).ch, 'o');
        assert_eq!(fb.get(6, 0).ch, ' ');
    }

    #[test]
    fn framebuffer_clear() {
        let mut fb = FrameBuffer::new(5, 3);
        fb.set(2, 1, Cell::new('X'));
        fb.clear();
        assert!(fb.get(2, 1).is_empty());
    }

    #[test]
    fn framebuffer_resize() {
        let mut fb = FrameBuffer::new(10, 5);
        fb.set(5, 3, Cell::new('X'));
        fb.resize(20, 10);
        assert_eq!(fb.width(), 20);
        assert_eq!(fb.height(), 10);
        assert!(fb.get(5, 3).is_empty());
    }

    #[test]
    fn framebuffer_diff() {
        let mut fb = FrameBuffer::new(3, 3);
        fb.swap();
        fb.set(1, 1, Cell::new('X'));
        let dirty = fb.diff();
        assert_eq!(dirty.len(), 1);
        assert_eq!(dirty[0], (1, 1));
    }

    #[test]
    fn framebuffer_swap() {
        let mut fb = FrameBuffer::new(3, 3);
        fb.swap();
        fb.set(1, 1, Cell::new('A'));
        let dirty = fb.diff();
        assert_eq!(dirty.len(), 1);
    }
}
