//! Frame buffer: cell-based pixel buffer with diff computation and dirty region tracking.
//!
//! This module provides the foundational rendering surface for the terminal UI engine.
//! It uses a Struct of Arrays (SoA) layout for cache-friendly cell access and efficient
//! diff computation between frames.
//!
//! # Architecture
//!
//! - [`Cell`] — A single terminal character with foreground/background colors and attributes.
//! - [`CellAttributes`] — Bitflags for bold, italic, underline, etc.
//! - [`FrameBuffer`] — A 2D grid of cells with double-buffering for diff support.
//!
//! # Example
//!
//! ```no_run
//! use bettertui_engine::framebuffer::{Cell, FrameBuffer};
//! use bettertui_engine::tree::Color;
//!
//! let mut fb = FrameBuffer::new(80, 24);
//! fb.write_str(0, 0, "Hello", Color::Default, Color::Default);
//! assert_eq!(fb.get(0, 0).ch, 'H');
//! ```

use crate::text::grapheme_clusters;
use crate::text::grapheme_width;
use crate::tree::Color;

// ---------------------------------------------------------------------------
// Cell
// ---------------------------------------------------------------------------

/// A single terminal cell containing a character, colors, and text attributes.
///
/// Cells are the atomic unit of the frame buffer. Each cell stores:
/// - A character (`ch`)
/// - Foreground and background colors
/// - An underline color (separate from foreground)
/// - Text attributes (bold, italic, underline, etc.) via [`CellAttributes`]
///
/// `Cell` implements `Copy`, so it can be freely duplicated without ownership concerns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// The character displayed in this cell.
    pub ch: char,
    /// Foreground (text) color.
    pub fg: Color,
    /// Background color.
    pub bg: Color,
    /// Underline color (independent of foreground).
    pub underline_color: Color,
    /// Text attributes (bold, italic, etc.).
    pub attributes: CellAttributes,
    /// Hyperlink id referencing a URL in the [`LinkPool`]. `0` means no link.
    pub link_id: u16,
}

impl Cell {
    /// Creates a new cell with the given character and default colors/attributes.
    pub fn new(ch: char) -> Self {
        Self {
            ch,
            fg: Color::Default,
            bg: Color::Default,
            underline_color: Color::Default,
            attributes: CellAttributes::empty(),
            link_id: 0,
        }
    }

    /// Sets the foreground color (builder pattern).
    pub fn with_fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }

    /// Sets the background color (builder pattern).
    pub fn with_bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    /// Sets the text attributes (builder pattern).
    pub fn with_attrs(mut self, attrs: CellAttributes) -> Self {
        self.attributes = attrs;
        self
    }

    /// Sets the hyperlink id (builder pattern). `0` clears the link.
    pub fn with_link(mut self, link_id: u16) -> Self {
        self.link_id = link_id;
        self
    }

    /// Returns `true` if the cell is in its default state (space character, default colors, no attributes).
    pub fn is_empty(&self) -> bool {
        self.ch == ' '
            && self.fg == Color::Default
            && self.bg == Color::Default
            && self.attributes.is_empty()
            && self.link_id == 0
    }

    /// Resets the cell to its default state.
    pub fn clear(&mut self) {
        self.ch = ' ';
        self.fg = Color::Default;
        self.bg = Color::Default;
        self.underline_color = Color::Default;
        self.attributes = CellAttributes::empty();
        self.link_id = 0;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new(' ')
    }
}

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        Self::new(ch)
    }
}

// ---------------------------------------------------------------------------
// CellAttributes
// ---------------------------------------------------------------------------

bitflags::bitflags! {
    /// Text attributes that can be applied to a [`Cell`].
    ///
    /// These are stored as bitflags for efficient composition and comparison.
    /// Multiple attributes can be combined using the `|` operator.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bettertui_engine::framebuffer::CellAttributes;
    ///
    /// let attrs = CellAttributes::BOLD | CellAttributes::ITALIC;
    /// assert!(attrs.contains(CellAttributes::BOLD));
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CellAttributes: u8 {
        /// Bold text (increased intensity).
        const BOLD          = 0b0000_0001;
        /// Italic text.
        const ITALIC        = 0b0000_0010;
        /// Underlined text.
        const UNDERLINE     = 0b0000_0100;
        /// Dim/faint text (decreased intensity).
        const DIM           = 0b0000_1000;
        /// Strikethrough text.
        const STRIKETHROUGH = 0b0001_0000;
        /// Inverse/reverse video (swaps fg and bg).
        const INVERSE       = 0b0010_0000;
        /// Hidden/invisible text.
        const HIDDEN        = 0b0100_0000;
    }
}

impl Default for CellAttributes {
    fn default() -> Self {
        Self::empty()
    }
}

// ---------------------------------------------------------------------------
// LinkPool
// ---------------------------------------------------------------------------

/// Stores hyperlink URLs referenced by [`Cell::link_id`].
///
/// Cells carry a compact `u16` id instead of an owned `String`, keeping the SoA
/// cell arrays small and `Copy`. Id `0` is reserved to mean "no link"; real ids
/// start at `1`. Identical URLs registered with the same explicit OSC 8 `id`
/// (or the same URL with no id) are de-duplicated so a multi-cell link shares one
/// entry.
#[derive(Debug, Clone, Default)]
pub struct LinkPool {
    /// Parallel to ids: `urls[link_id - 1]` is the URL for `link_id`.
    urls: Vec<String>,
    /// De-dup key `(explicit_osc8_id, url)` -> assigned `link_id`.
    lookup: std::collections::HashMap<(String, String), u16>,
}

impl LinkPool {
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocates (or reuses) a link id for `url`, optionally keyed by an explicit
    /// OSC 8 `id=` parameter so that non-adjacent runs of the same logical link
    /// coalesce. Returns `0` when `url` is empty or the pool is exhausted
    /// (`u16::MAX` distinct links).
    pub fn alloc(&mut self, url: &str, explicit_id: Option<&str>) -> u16 {
        if url.is_empty() {
            return 0;
        }
        let key = (explicit_id.unwrap_or("").to_string(), url.to_string());
        if let Some(&id) = self.lookup.get(&key) {
            return id;
        }
        if self.urls.len() >= (u16::MAX as usize) {
            return 0;
        }
        self.urls.push(url.to_string());
        let id = self.urls.len() as u16; // ids are 1-based
        self.lookup.insert(key, id);
        id
    }

    /// Returns the URL for `link_id`, or `None` for id `0` / unknown ids.
    pub fn get(&self, link_id: u16) -> Option<&str> {
        if link_id == 0 {
            return None;
        }
        self.urls.get((link_id - 1) as usize).map(String::as_str)
    }

    /// Number of distinct links registered.
    pub fn len(&self) -> usize {
        self.urls.len()
    }

    /// Returns `true` if no links are registered.
    pub fn is_empty(&self) -> bool {
        self.urls.is_empty()
    }

    /// Removes all links.
    pub fn clear(&mut self) {
        self.urls.clear();
        self.lookup.clear();
    }
}

// ---------------------------------------------------------------------------
// FrameBuffer
// ---------------------------------------------------------------------------

/// Struct of Arrays (SoA) storage for terminal cells.
///
/// Compared to Array of Structs (AoS), SoA provides:
/// - Cache-friendly access when iterating a single field (e.g., just chars)
/// - SIMD-friendly comparison (can compare 16 chars at once)
/// - Independent field updates without copying the entire `Cell` struct
/// - Future packed representations (e.g., 4-bit alpha, 8-bit palette index)
#[derive(Debug, Clone)]
struct CellArrays {
    chars: Vec<char>,
    fg: Vec<Color>,
    bg: Vec<Color>,
    underline_color: Vec<Color>,
    attrs: Vec<CellAttributes>,
    link_id: Vec<u16>,
}

impl CellArrays {
    fn new(size: usize) -> Self {
        Self {
            chars: vec![' '; size],
            fg: vec![Color::Default; size],
            bg: vec![Color::Default; size],
            underline_color: vec![Color::Default; size],
            attrs: vec![CellAttributes::empty(); size],
            link_id: vec![0; size],
        }
    }

    fn resize(&mut self, size: usize) {
        self.chars.resize(size, ' ');
        self.fg.resize(size, Color::Default);
        self.bg.resize(size, Color::Default);
        self.underline_color.resize(size, Color::Default);
        self.attrs.resize(size, CellAttributes::empty());
        self.link_id.resize(size, 0);
    }
}

/// A 2D grid of [`Cell`]s with double-buffering for efficient diff computation.
///
/// The frame buffer is the primary rendering surface. It maintains two internal
/// cell arrays (`cells` and `back`) to support diff-based rendering: after rendering
/// a frame, [`diff`](Self::diff) returns only the cells that changed since the last
/// [`swap`](Self::swap).
///
/// # Layout
///
/// The buffer uses row-major ordering: cell `(x, y)` is at index `y * width + x`.
///
/// # Example
///
/// ```no_run
/// use bettertui_engine::framebuffer::{Cell, FrameBuffer};
///
/// let mut fb = FrameBuffer::new(10, 5);
/// fb.set(0, 0, Cell::new('A'));
/// assert_eq!(fb.get(0, 0).ch, 'A');
/// ```
#[derive(Debug, Clone)]
pub struct FrameBuffer {
    width: u16,
    height: u16,
    cells: CellArrays,
    back: CellArrays,
    links: LinkPool,
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl FrameBuffer {
    /// Creates a new frame buffer with the given dimensions.
    ///
    /// All cells are initialized to the default (space character, default colors).
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self { width, height, cells: CellArrays::new(size), back: CellArrays::new(size), links: LinkPool::new() }
    }

    /// Returns the width of the buffer in cells.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Returns the height of the buffer in cells.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Resizes the buffer to the new dimensions.
    ///
    /// **Warning:** This does NOT preserve existing cell content. All cells are reset.
    pub fn resize(&mut self, width: u16, height: u16) {
        let size = (width as usize) * (height as usize);
        self.width = width;
        self.height = height;
        self.cells.resize(size);
        self.back.resize(size);
        self.clear();
    }

    /// Clears all cells to their default state.
    pub fn clear(&mut self) {
        self.cells.chars.fill(' ');
        self.cells.fg.fill(Color::Default);
        self.cells.bg.fill(Color::Default);
        self.cells.underline_color.fill(Color::Default);
        self.cells.attrs.fill(CellAttributes::empty());
        self.cells.link_id.fill(0);
        self.links.clear();
    }

    /// Clears a rectangular region without affecting cells outside the rect.
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
                self.cells.link_id[idx] = 0;
            }
        }
    }

    /// Computes the flat array index for `(x, y)`.
    ///
    /// This does NOT check bounds. Use [`in_bounds`](Self::in_bounds) first if needed.
    pub fn index(&self, x: u16, y: u16) -> usize {
        (y as usize) * (self.width as usize) + (x as usize)
    }

    /// Returns `true` if `(x, y)` is within the buffer bounds.
    pub fn in_bounds(&self, x: u16, y: u16) -> bool {
        x < self.width && y < self.height
    }

    /// Returns the cell at `(x, y)`, or a default cell if out of bounds.
    pub fn get(&self, x: u16, y: u16) -> Cell {
        if self.in_bounds(x, y) {
            let idx = self.index(x, y);
            Cell {
                ch: self.cells.chars[idx],
                fg: self.cells.fg[idx],
                bg: self.cells.bg[idx],
                underline_color: self.cells.underline_color[idx],
                attributes: self.cells.attrs[idx],
                link_id: self.cells.link_id[idx],
            }
        } else {
            Cell::default()
        }
    }

    /// Sets the cell at `(x, y)`. No-op if out of bounds.
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if self.in_bounds(x, y) {
            let idx = self.index(x, y);
            self.cells.chars[idx] = cell.ch;
            self.cells.fg[idx] = cell.fg;
            if cell.bg != Color::Default || self.cells.bg[idx] == Color::Default {
                self.cells.bg[idx] = cell.bg;
            }
            self.cells.underline_color[idx] = cell.underline_color;
            self.cells.attrs[idx] = cell.attributes;
            self.cells.link_id[idx] = cell.link_id;
        }
    }

    /// Registers a hyperlink URL and returns its id for use in [`Cell::link_id`].
    ///
    /// `explicit_id` corresponds to the OSC 8 `id=` parameter and lets
    /// non-adjacent cells share one logical link. Returns `0` for an empty URL.
    pub fn alloc_link(&mut self, url: &str, explicit_id: Option<&str>) -> u16 {
        self.links.alloc(url, explicit_id)
    }

    /// Returns the URL registered for `link_id`, or `None`.
    pub fn link_url(&self, link_id: u16) -> Option<&str> {
        self.links.get(link_id)
    }

    /// Returns a reference to the buffer's [`LinkPool`].
    pub fn links(&self) -> &LinkPool {
        &self.links
    }

    /// Fills a rectangular region with the given cell.
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

    /// Writes a string at `(x, y)` with the given foreground and background colors.
    ///
    /// Handles grapheme clusters and wide characters (e.g., CJK, emoji).
    /// Characters that exceed the buffer width are truncated.
    pub fn write_str(&mut self, x: u16, y: u16, s: &str, fg: Color, bg: Color) {
        let mut col = x;
        for g in grapheme_clusters(s) {
            if col >= self.width {
                break;
            }
            let w = grapheme_width(g) as u16;
            if let Some(ch) = g.chars().next() {
                self.set(col, y, Cell::new(ch).with_fg(fg).with_bg(bg));
                if w == 2 && col + 1 < self.width {
                    self.set(col + 1, y, Cell::new(' ').with_fg(fg).with_bg(bg));
                }
            }
            col += w;
        }
    }

    /// Swaps the front and back buffers. After swapping, [`diff`](Self::diff) will
    /// report changes relative to the new back buffer.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.cells, &mut self.back);
    }

    /// Copies cell data from another buffer. Sizes may differ; only the overlapping
    /// region is copied.
    pub fn copy_from(&mut self, other: &FrameBuffer) {
        let len = (self.width as usize) * (self.height as usize);
        let other_len = (other.width as usize) * (other.height as usize);
        let copy_len = len.min(other_len);
        self.cells.chars[..copy_len].copy_from_slice(&other.cells.chars[..copy_len]);
        self.cells.fg[..copy_len].copy_from_slice(&other.cells.fg[..copy_len]);
        self.cells.bg[..copy_len].copy_from_slice(&other.cells.bg[..copy_len]);
        self.cells.underline_color[..copy_len].copy_from_slice(&other.cells.underline_color[..copy_len]);
        self.cells.attrs[..copy_len].copy_from_slice(&other.cells.attrs[..copy_len]);
        self.cells.link_id[..copy_len].copy_from_slice(&other.cells.link_id[..copy_len]);
        self.links = other.links.clone();
    }

    /// Returns the list of `(x, y)` coordinates that differ between the front and back buffers.
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
                    || self.cells.link_id[idx] != self.back.link_id[idx]
                {
                    dirty.push((x, y));
                }
            }
        }
        dirty
    }

    /// Returns a `Vec<Cell>` containing all cells in row-major order.
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
                link_id: self.cells.link_id[i],
            });
        }
        result
    }

    /// Returns `true` if all cells contain the space character.
    pub fn is_empty(&self) -> bool {
        self.cells.chars.iter().all(|&c| c == ' ')
    }

    /// Iterates over all cell positions and applies a mutable transformation to each cell.
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
                    link_id: self.cells.link_id[idx],
                };
                f(x, y, &mut cell);
                self.cells.chars[idx] = cell.ch;
                self.cells.fg[idx] = cell.fg;
                self.cells.bg[idx] = cell.bg;
                self.cells.underline_color[idx] = cell.underline_color;
                self.cells.attrs[idx] = cell.attributes;
                self.cells.link_id[idx] = cell.link_id;
            }
        }
    }

    /// Applies a transformation to every cell's foreground color.
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

    /// Applies a transformation to every cell's background color.
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

    /// Applies a transformation to every cell in a rectangular region.
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
                    link_id: self.cells.link_id[idx],
                };
                f(col, row, &mut cell);
                self.cells.chars[idx] = cell.ch;
                self.cells.fg[idx] = cell.fg;
                self.cells.bg[idx] = cell.bg;
                self.cells.underline_color[idx] = cell.underline_color;
                self.cells.attrs[idx] = cell.attributes;
                self.cells.link_id[idx] = cell.link_id;
            }
        }
    }
}

#[cfg(test)]
mod link_tests {
    use super::*;

    #[test]
    fn link_pool_alloc_and_get() {
        let mut pool = LinkPool::new();
        let id = pool.alloc("https://example.com", None);
        assert_eq!(id, 1);
        assert_eq!(pool.get(id), Some("https://example.com"));
        assert_eq!(pool.get(0), None);
        assert_eq!(pool.get(99), None);
    }

    #[test]
    fn link_pool_empty_url_is_zero() {
        let mut pool = LinkPool::new();
        assert_eq!(pool.alloc("", None), 0);
        assert!(pool.is_empty());
    }

    #[test]
    fn link_pool_dedups_same_url() {
        let mut pool = LinkPool::new();
        let a = pool.alloc("https://a.com", None);
        let b = pool.alloc("https://a.com", None);
        assert_eq!(a, b);
        assert_eq!(pool.len(), 1);
    }

    #[test]
    fn link_pool_explicit_id_distinguishes() {
        let mut pool = LinkPool::new();
        // Same URL but different explicit ids are distinct logical links.
        let a = pool.alloc("https://a.com", Some("one"));
        let b = pool.alloc("https://a.com", Some("two"));
        assert_ne!(a, b);
        assert_eq!(pool.len(), 2);
    }

    #[test]
    fn framebuffer_cell_carries_link_id() {
        let mut fb = FrameBuffer::new(4, 1);
        let id = fb.alloc_link("https://example.com", None);
        fb.set(0, 0, Cell::new('X').with_link(id));
        assert_eq!(fb.get(0, 0).link_id, id);
        assert_eq!(fb.link_url(id), Some("https://example.com"));
    }

    #[test]
    fn framebuffer_link_diff_detected() {
        let mut fb = FrameBuffer::new(2, 1);
        fb.swap(); // back is now the (empty) baseline
        let id = fb.alloc_link("https://example.com", None);
        fb.set(0, 0, Cell::new('X').with_link(id));
        let dirty = fb.diff();
        assert!(dirty.contains(&(0, 0)));
    }

    #[test]
    fn framebuffer_clear_resets_links() {
        let mut fb = FrameBuffer::new(2, 1);
        let id = fb.alloc_link("https://example.com", None);
        fb.set(0, 0, Cell::new('X').with_link(id));
        fb.clear();
        assert_eq!(fb.get(0, 0).link_id, 0);
        assert!(fb.links().is_empty());
    }
}
