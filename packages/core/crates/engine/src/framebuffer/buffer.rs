use super::cell::{Cell, CellAttributes};
use crate::text::grapheme_clusters;
use crate::tree::color::Color;

/// SoA (Struct of Arrays) storage for terminal cells.
///
/// Compared to AoS (Array of Structs), SoA allows:
/// - Cache-friendly access when iterating a single field (e.g., just chars)
/// - SIMD-friendly comparison (can compare 16 chars at once)
/// - Independent field updates without copying the entire Cell struct
/// - Future packed representations (e.g., 4-bit alpha, 8-bit palette index)
#[derive(Debug, Clone)]
struct CellArrays {
    chars: Vec<char>,
    fg: Vec<Color>,
    bg: Vec<Color>,
    underline_color: Vec<Color>,
    attrs: Vec<CellAttributes>,
}

impl CellArrays {
    fn new(size: usize) -> Self {
        Self {
            chars: vec![' '; size],
            fg: vec![Color::Default; size],
            bg: vec![Color::Default; size],
            underline_color: vec![Color::Default; size],
            attrs: vec![CellAttributes::empty(); size],
        }
    }

    fn resize(&mut self, size: usize) {
        self.chars.resize(size, ' ');
        self.fg.resize(size, Color::Default);
        self.bg.resize(size, Color::Default);
        self.underline_color.resize(size, Color::Default);
        self.attrs.resize(size, CellAttributes::empty());
    }
}

#[derive(Debug, Clone)]
pub struct FrameBuffer {
    width: u16,
    height: u16,
    cells: CellArrays,
    back: CellArrays,
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
            cells: CellArrays::new(size),
            back: CellArrays::new(size),
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
        self.cells.resize(size);
        self.back.resize(size);
        self.clear();
    }

    pub fn clear(&mut self) {
        self.cells.chars.fill(' ');
        self.cells.fg.fill(Color::Default);
        self.cells.bg.fill(Color::Default);
        self.cells.underline_color.fill(Color::Default);
        self.cells.attrs.fill(CellAttributes::empty());
    }

    /// Clear a rectangular region without affecting cells outside the rect.
    pub fn clear_rect(&mut self, x: u16, y: u16, width: u16, height: u16) {
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
                let idx = self.index(col, row);
                self.cells.chars[idx] = ' ';
                self.cells.fg[idx] = Color::Default;
                self.cells.bg[idx] = Color::Default;
                self.cells.underline_color[idx] = Color::Default;
                self.cells.attrs[idx] = CellAttributes::empty();
            }
        }
    }

    pub fn index(&self, x: u16, y: u16) -> usize {
        (y as usize) * (self.width as usize) + (x as usize)
    }

    pub fn in_bounds(&self, x: u16, y: u16) -> bool {
        x < self.width && y < self.height
    }

    pub fn get(&self, x: u16, y: u16) -> Cell {
        if self.in_bounds(x, y) {
            let idx = self.index(x, y);
            Cell {
                ch: self.cells.chars[idx],
                fg: self.cells.fg[idx],
                bg: self.cells.bg[idx],
                underline_color: self.cells.underline_color[idx],
                attributes: self.cells.attrs[idx],
            }
        } else {
            Cell::default()
        }
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if self.in_bounds(x, y) {
            let idx = self.index(x, y);
            self.cells.chars[idx] = cell.ch;
            self.cells.fg[idx] = cell.fg;
            self.cells.bg[idx] = cell.bg;
            self.cells.underline_color[idx] = cell.underline_color;
            self.cells.attrs[idx] = cell.attributes;
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

    pub fn write_str(&mut self, x: u16, y: u16, s: &str, fg: Color, bg: Color) {
        let mut col = x;
        for g in grapheme_clusters(s) {
            if col >= self.width {
                break;
            }
            let w = crate::text::grapheme_width(g) as u16;
            if let Some(ch) = g.chars().next() {
                self.set(col, y, Cell::new(ch).with_fg(fg).with_bg(bg));
                if w == 2 && col + 1 < self.width {
                    self.set(col + 1, y, Cell::new(' ').with_fg(fg).with_bg(bg));
                }
            }
            col += w;
        }
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.cells, &mut self.back);
    }

    pub fn copy_from(&mut self, other: &FrameBuffer) {
        let len = (self.width as usize) * (self.height as usize);
        let other_len = (other.width as usize) * (other.height as usize);
        let copy_len = len.min(other_len);
        self.cells.chars[..copy_len].copy_from_slice(&other.cells.chars[..copy_len]);
        self.cells.fg[..copy_len].copy_from_slice(&other.cells.fg[..copy_len]);
        self.cells.bg[..copy_len].copy_from_slice(&other.cells.bg[..copy_len]);
        self.cells.underline_color[..copy_len]
            .copy_from_slice(&other.cells.underline_color[..copy_len]);
        self.cells.attrs[..copy_len].copy_from_slice(&other.cells.attrs[..copy_len]);
    }

    pub fn diff(&self) -> Vec<(u16, u16)> {
        let mut dirty = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.index(x, y);
                if self.cells.chars[idx] != self.back.chars[idx]
                    || self.cells.fg[idx] != self.back.fg[idx]
                    || self.cells.bg[idx] != self.back.bg[idx]
                    || self.cells.underline_color[idx] != self.back.underline_color[idx]
                    || self.cells.attrs[idx] != self.back.attrs[idx]
                {
                    dirty.push((x, y));
                }
            }
        }
        dirty
    }

    pub fn cells(&self) -> Vec<Cell> {
        let len = (self.width as usize) * (self.height as usize);
        let mut result = Vec::with_capacity(len);
        for i in 0..len {
            result.push(Cell {
                ch: self.cells.chars[i],
                fg: self.cells.fg[i],
                bg: self.cells.bg[i],
                underline_color: self.cells.underline_color[i],
                attributes: self.cells.attrs[i],
            });
        }
        result
    }

    pub fn is_empty(&self) -> bool {
        self.cells.chars.iter().all(|&c| c == ' ')
    }

    /// Iterate over all cell positions and apply a mutable transformation to each cell.
    pub fn process_cells<F>(&mut self, mut f: F)
    where
        F: FnMut(u16, u16, &mut Cell),
    {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.index(x, y);
                let mut cell = Cell {
                    ch: self.cells.chars[idx],
                    fg: self.cells.fg[idx],
                    bg: self.cells.bg[idx],
                    underline_color: self.cells.underline_color[idx],
                    attributes: self.cells.attrs[idx],
                };
                f(x, y, &mut cell);
                self.cells.chars[idx] = cell.ch;
                self.cells.fg[idx] = cell.fg;
                self.cells.bg[idx] = cell.bg;
                self.cells.underline_color[idx] = cell.underline_color;
                self.cells.attrs[idx] = cell.attributes;
            }
        }
    }

    /// Apply a transformation to every cell's foreground color.
    pub fn process_fg_colors<F>(&mut self, mut f: F)
    where
        F: FnMut(u16, u16, Color) -> Color,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.index(x, y);
                self.cells.fg[idx] = f(x, y, self.cells.fg[idx]);
            }
        }
    }

    /// Apply a transformation to every cell's background color.
    pub fn process_bg_colors<F>(&mut self, mut f: F)
    where
        F: FnMut(u16, u16, Color) -> Color,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.index(x, y);
                self.cells.bg[idx] = f(x, y, self.cells.bg[idx]);
            }
        }
    }

    /// Apply a transformation to every cell in a rectangular region.
    pub fn process_region<F>(&mut self, x: u16, y: u16, w: u16, h: u16, mut f: F)
    where
        F: FnMut(u16, u16, &mut Cell),
    {
        for dy in 0..h {
            let row = y + dy;
            if row >= self.height {
                break;
            }
            for dx in 0..w {
                let col = x + dx;
                if col >= self.width {
                    break;
                }
                let idx = self.index(col, row);
                let mut cell = Cell {
                    ch: self.cells.chars[idx],
                    fg: self.cells.fg[idx],
                    bg: self.cells.bg[idx],
                    underline_color: self.cells.underline_color[idx],
                    attributes: self.cells.attrs[idx],
                };
                f(col, row, &mut cell);
                self.cells.chars[idx] = cell.ch;
                self.cells.fg[idx] = cell.fg;
                self.cells.bg[idx] = cell.bg;
                self.cells.underline_color[idx] = cell.underline_color;
                self.cells.attrs[idx] = cell.attributes;
            }
        }
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
