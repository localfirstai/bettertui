# FrameBuffer

> The frame buffer is a cell-based grid that represents the terminal screen.
> It supports double buffering, dirty tracking, and efficient ANSI encoding.

## 1. Overview

The frame buffer is a 2D grid of cells, where each cell represents one character position on the terminal. It supports:

- **Double buffering:** Two buffers are maintained. The "front" buffer represents what's currently on screen. The "back" buffer is being written to. After rendering, the back buffer is diffed against the front, and only changed cells are written.
- **Dirty tracking:** Each cell has a dirty flag. Only dirty cells are encoded and written.
- **ANSI encoding:** Dirty cells are translated into ANSI escape sequences.

```
┌─────────────────────────────────────┐
│           Frame Buffer              │
│  ┌─────┬─────┬─────┬─────┬─────┐  │
│  │Cell │Cell │Cell │Cell │Cell │  │
│  │(0,0)│(1,0)│(2,0)│(3,0)│(4,0)│  │
│  ├─────┼─────┼─────┼─────┼─────┤  │
│  │Cell │Cell │Cell │Cell │Cell │  │
│  │(0,1)│(1,1)│(2,1)│(3,1)│(4,1)│  │
│  ├─────┼─────┼─────┼─────┼─────┤  │
│  │ ... │ ... │ ... │ ... │ ... │  │
│  └─────┴─────┴─────┴─────┴─────┘  │
└─────────────────────────────────────┘
```

## 2. Cell Structure

### 2.1 Cell

```rust
pub struct Cell {
    pub char: CellChar,
    pub fg: Color,
    pub bg: Color,
    pub underline_color: Color,
    pub attributes: CellAttributes,
}
```

### 2.2 CellChar

```rust
pub enum CellChar {
    /// A single character (most common case)
    Char(char),
    /// A wide character (CJK, emoji) that occupies 2 cells
    Wide(char),
    /// The second cell of a wide character (empty, but linked to the first)
    WideContinuation,
    /// A combining character (accent, diacritical mark)
    Combining(char),
    /// An empty cell (space)
    Empty,
}
```

**Why an enum, not just `char`:** Terminal characters have different width behaviors. A CJK character occupies 2 cells. A combining character attaches to the previous character. An empty cell is a space. Modeling these as separate variants allows correct width calculation and rendering.

### 2.3 CellAttributes

```rust
pub struct CellAttributes {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
    pub strikethrough: bool,
    pub inverse: bool,
    pub hidden: bool,
}
```

**Size:** 1 byte (8 bits, one per attribute). This is packed into a single byte for cache efficiency.

### 2.4 Color Representation

Colors in the frame buffer use the same `Color` enum as the node model:

```rust
pub enum Color {
    Named(NamedColor),
    Indexed(u8),
    Rgb { r: u8, g: u8, b: u8 },
    Default,
}
```

**Resolution:** Colors are resolved from style inheritance before being written to the frame buffer. The frame buffer stores resolved colors, not inherited ones.

## 3. Frame Buffer Structure

### 3.1 Buffer

```rust
pub struct FrameBuffer {
    width: u16,
    height: u16,
    cells: Vec<Cell>,
    dirty: Vec<bool>,
    dirty_count: u32,
}
```

**`cells`:** A flat `Vec<Cell>` of size `width × height`. Indexed as `cells[y * width + x]`.

**`dirty`:** A flat `Vec<bool>` of size `width × height`. Tracks which cells have changed since the last frame.

**`dirty_count`:** Number of dirty cells. Used for early exit (if 0, skip diffing).

### 3.2 Double Buffering

```rust
pub struct DoubleBuffer {
    front: FrameBuffer,
    back: FrameBuffer,
}
```

**Front buffer:** Represents what's currently on screen. Read-only during rendering.

**Back buffer:** Being written to during rendering. After rendering, diffed against front buffer.

**Swap:** After diffing, the back buffer becomes the front buffer. The old front buffer is cleared and becomes the new back buffer.

```
Before render:
  Front: [old content]
  Back:  [empty]

After render:
  Front: [old content]
  Back:  [new content]

After diff + swap:
  Front: [new content]
  Back:  [empty]
```

### 3.3 Operations

```rust
impl FrameBuffer {
    pub fn new(width: u16, height: u16) -> Self;
    pub fn resize(&mut self, width: u16, height: u16);
    pub fn clear(&mut self);
    pub fn get(&self, x: u16, y: u16) -> Option<&Cell>;
    pub fn set(&mut self, x: u16, y: u16, cell: Cell);
    pub fn fill_rect(&mut self, rect: Rect, cell: Cell);
    pub fn write_text(&mut self, x: u16, y: u16, text: &str, style: &ResolvedStyle);
    pub fn write_char(&mut self, x: u16, y: u16, ch: char, style: &ResolvedStyle);
    pub fn mark_dirty(&mut self, x: u16, y: u16);
    pub fn is_dirty(&self, x: u16, y: u16) -> bool;
    pub fn dirty_cells(&self) -> impl Iterator<Item = (u16, u16, &Cell)>;
    pub fn dirty_count(&self) -> u32;
}
```

### 3.4 Bounds Checking

All operations perform bounds checking. Out-of-bounds writes are silently ignored (not panicked). This is because:

- Terminal resizing can cause temporary size mismatches.
- Clipping should be handled by the renderer, not by the frame buffer.
- Panicking on out-of-bounds would crash the application.

## 4. Text Rendering

### 4.1 Character Width

Character width is determined using the `unicode-width` crate:

```rust
pub fn char_width(ch: char) -> u16 {
    unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0) as u16
}
```

**Width categories:**
- **Half-width (1 cell):** Latin characters, numbers, punctuation.
- **Full-width (2 cells):** CJK characters, some emoji.
- **Zero-width (0 cells):** Combining characters, control characters.

### 4.2 Wide Character Handling

Wide characters (width = 2) occupy two cells:

```
Cell (5, 3): '中' (wide, left half)
Cell (6, 3): WideContinuation (right half)
```

When writing a wide character:
1. Write the character to cell (x, y) with `CellChar::Wide(ch)`.
2. Write `CellChar::WideContinuation` to cell (x+1, y).
3. If x+1 is out of bounds, write a space to cell (x, y) instead (the wide character doesn't fit).

### 4.3 Combining Characters

Combining characters (accents, diacritical marks) attach to the previous character:

```
Cell (5, 3): 'e' (base character)
Cell (6, 3): Combining('́') (combining acute accent)
```

When writing a combining character:
1. Find the previous non-combining character in the same cell or the previous cell.
2. Apply the combining character to the base character.
3. The combining character does not occupy a separate cell.

**Simplified approach for v1:** In the initial implementation, combining characters are rendered as separate cells (they appear after the base character). Full combining character support can be added later.

### 4.4 Emoji

Emoji characters are treated as wide characters (2 cells). Some emoji are "modifier base" characters that can accept skin tone modifiers. The initial implementation treats all emoji as 2-cell characters.

## 5. Dirty Tracking

### 5.1 Cell-Level Dirty Tracking

Each cell has a dirty flag. When a cell is written to, its dirty flag is set:

```rust
pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
    let idx = y as usize * self.width as usize + x as usize;
    if idx < self.cells.len() {
        if self.cells[idx] != cell {
            self.cells[idx] = cell;
            self.dirty[idx] = true;
            self.dirty_count += 1;
        }
    }
}
```

**Why check equality:** If the new cell is identical to the old cell, don't mark it dirty. This prevents unnecessary work when rendering static content.

### 5.2 Region-Level Dirty Tracking

For efficiency, we also support region-level dirty tracking:

```rust
pub fn mark_rect_dirty(&mut self, rect: Rect) {
    for y in rect.y..rect.y + rect.height {
        for x in rect.x..rect.x + rect.width {
            self.mark_dirty(x, y);
        }
    }
}
```

This is used when a node's layout changes — the entire node region is marked dirty.

### 5.3 Dirty Cell Iteration

```rust
pub fn dirty_cells(&self) -> impl Iterator<Item = (u16, u16, &Cell)> {
    self.dirty.iter()
        .enumerate()
        .filter(|(_, &dirty)| dirty)
        .map(move |(i, _)| {
            let x = (i % self.width as usize) as u16;
            let y = (i / self.width as usize) as u16;
            (x, y, &self.cells[i])
        })
}
```

**Performance:** This iterates over all cells to find dirty ones. For large terminals with few dirty cells, this is O(w × h). We can optimize with a dirty cell list (see section 5.4).

### 5.4 Dirty Cell List Optimization

For better performance, maintain a separate list of dirty cell coordinates:

```rust
pub struct FrameBuffer {
    cells: Vec<Cell>,
    dirty_flags: Vec<bool>,
    dirty_list: Vec<(u16, u16)>,  // sorted by (y, x) for sequential access
    dirty_count: u32,
}
```

When a cell is marked dirty, its coordinate is added to `dirty_list`. When iterating dirty cells, iterate `dirty_list` instead of scanning all cells.

**Trade-off:** `dirty_list` requires insertion and deduplication. For most frames (< 1000 dirty cells), this is negligible.

## 6. Frame Diffing

### 6.1 Diff Algorithm

The diff compares current (back) and previous (front) frame buffers:

```rust
pub fn diff(&self, front: &FrameBuffer, back: &FrameBuffer) -> Vec<DirtyRegion> {
    let mut dirty_regions = Vec::new();

    for y in 0..back.height {
        let mut region_start: Option<u16> = None;

        for x in 0..back.width {
            let back_cell = back.get(x, y);
            let front_cell = front.get(x, y);

            if back_cell != front_cell {
                if region_start.is_none() {
                    region_start = Some(x);
                }
            } else {
                if let Some(start) = region_start {
                    dirty_regions.push(DirtyRegion {
                        x: start,
                        y,
                        width: x - start,
                        height: 1,
                    });
                    region_start = None;
                }
            }
        }

        if let Some(start) = region_start {
            dirty_regions.push(DirtyRegion {
                x: start,
                y,
                width: back.width - start,
                height: 1,
            });
        }
    }

    dirty_regions
}
```

### 6.2 Dirty Regions

```rust
pub struct DirtyRegion {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
```

Dirty regions are rectangular areas where the frame buffer has changed. They are used to:
- Minimize ANSI output (only write changed regions).
- Optimize cursor positioning (move cursor to region start, write cells, move to next region).
- Reduce terminal I/O (fewer escape sequences).

### 6.3 Region Merging

Adjacent dirty regions are merged to reduce the number of regions:

```
Before merging:
  [1, 0, 5, 1]  (row 0, cols 1-5)
  [1, 1, 5, 1]  (row 1, cols 1-5)
  [1, 2, 5, 1]  (row 2, cols 1-5)

After merging:
  [1, 0, 5, 3]  (rows 0-2, cols 1-5)
```

**When to merge:** Regions are merged if they are adjacent (touching edges) and have the same width.

### 6.4 Full-Frame Optimization

If dirty_count > width × height / 2, it's more efficient to do a full-frame repaint than to diff and encode individual regions. The renderer detects this and switches to full-frame mode.

## 7. ANSI Encoding

### 7.1 Encoding Pipeline

```
Dirty regions → Cursor positioning → Style application → Character output
```

### 7.2 Cursor Positioning

ANSI escape sequence for cursor positioning:

```
ESC[{row};{col}H
```

Where row and col are 1-indexed (ANSI convention).

### 7.3 Style Application

SGR (Select Graphic Rendition) sequences:

```
ESC[0m        — reset all styles
ESC[1m        — bold
ESC[2m        — dim
ESC[3m        — italic
ESC[4m        — underline
ESC[7m        — inverse
ESC[8m        — hidden
ESC[9m        — strikethrough
ESC[38;5;nM   — foreground color (256 palette)
ESC[48;5;nM   — background color (256 palette)
ESC[38;2;r;g;bm — foreground color (true color)
ESC[48;2;r;g;bm — background color (true color)
ESC[39m       — default foreground
ESC[49m       — default background
```

### 7.4 Style Coalescing

Adjacent cells with the same style share a single SGR sequence:

```
Without coalescing:
  ESC[1mESC[38;5;196mH ESC[1mESC[38;5;196me ESC[1mESC[38;5;196ml ESC[1mESC[38;5;196ml ESC[1mESC[38;5;196mo

With coalescing:
  ESC[1mESC[38;5;196mHello
```

**Savings:** For a 10-character bold red string, coalescing reduces output from ~60 bytes to ~15 bytes.

### 7.5 Move Optimization

Cursor movements are minimized:

1. Process cells in reading order (left-to-right, top-to-bottom).
2. Use relative movements (ESC[C = right, ESC[B = down) when moving short distances.
3. Use absolute positioning (ESC[{row};{col}H) when jumping to a new row or distant cell.

### 7.6 Full ANSI Output

```rust
pub fn encode_frame(
    front: &FrameBuffer,
    back: &FrameBuffer,
    dirty_regions: &[DirtyRegion],
) -> Vec<u8> {
    let mut output = Vec::with_capacity(4096);
    let mut current_style = ResolvedStyle::default();

    // Hide cursor during rendering
    output.extend_from_slice(b"\x1b[?25l");

    for region in dirty_regions {
        // Move cursor to region start
        write!(output, "\x1b[{};{}H", region.y + 1, region.x + 1).unwrap();

        for y in region.y..region.y + region.height {
            for x in region.x..region.x + region.width {
                let cell = back.get(x, y).unwrap();

                // Apply style changes
                let new_style = cell.to_style();
                let sgr = compute_sgr_diff(&current_style, &new_style);
                output.extend_from_slice(&sgr);
                current_style = new_style;

                // Write character
                match cell.char {
                    CellChar::Char(ch) | CellChar::Wide(ch) => {
                        output.extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes());
                    }
                    CellChar::Empty => {
                        output.push(b' ');
                    }
                    _ => {}
                }
            }
        }
    }

    // Show cursor
    output.extend_from_slice(b"\x1b[?25h");

    output
}
```

## 8. Cursor Rendering

### 8.1 Cursor Position

The cursor is rendered at the focused input node's cursor position:

```rust
pub fn render_cursor(
    &self,
    output: &mut Vec<u8>,
    cursor_pos: Option<Point>,
    cursor_style: CursorStyle,
) {
    if let Some(pos) = cursor_pos {
        // Move cursor to position
        write!(output, "\x1b[{};{}H", pos.y + 1, pos.x + 1).unwrap();

        // Set cursor style
        match cursor_style {
            CursorStyle::Block => output.extend_from_slice(b"\x1b[2 q"),
            CursorStyle::Underline => output.extend_from_slice(b"\x1b[4 q"),
            CursorStyle::Bar => output.extend_from_slice(b"\x1b[6 q"),
            CursorStyle::None => {}
        }

        // Show cursor
        output.extend_from_slice(b"\x1b[?25h");
    }
}
```

### 8.2 Cursor Blinking

Cursor blinking is handled by the terminal emulator, not by BetterTUI. We set the cursor style and position; the terminal handles blinking.

## 9. Selection Rendering

### 9.1 Selection Highlight

Text selection is rendered by inverting the foreground and background colors of selected cells:

```rust
pub fn render_selection(
    &mut self,
    selection: &Selection,
) {
    for pos in selection.cells() {
        if let Some(cell) = self.get_mut(pos.x, pos.y) {
            std::mem::swap(&mut cell.fg, &mut cell.bg);
        }
    }
}
```

### 9.2 Selection Coordinates

```rust
pub struct Selection {
    pub start: Point,
    pub end: Point,
    pub mode: SelectionMode,
}

pub enum SelectionMode {
    Char,
    Word,
    Line,
}
```

## 10. Memory

### 10.1 Cell Size

```rust
// Cell size breakdown:
// char: 4 bytes (enum discriminant + char)
// fg: 5 bytes (Color enum)
// bg: 5 bytes (Color enum)
// underline_color: 5 bytes (Color enum)
// attributes: 1 byte (bitflags)
// Total: ~20 bytes per cell
```

### 10.2 Buffer Size

For a 200×50 terminal:
- Single buffer: 200 × 50 × 20 = 200KB
- Double buffer: 400KB
- Dirty flags: 200 × 50 × 1 = 10KB
- Dirty list: ~1000 × 4 = 4KB
- **Total: ~414KB**

This is well within acceptable memory limits for a terminal application.

### 10.3 Optimization — Color Packing

Colors can be packed into 4 bytes instead of 5:

```rust
pub struct PackedColor {
    // Bits 0-3: color kind (Named/Indexed/Rgb/Default)
    // Bits 4-31: color data (varies by kind)
    data: u32,
}
```

This reduces cell size to ~16 bytes, saving 20% memory.

## 11. Future Considerations

### 11.1 Triple Buffering

Triple buffering allows the renderer to write to a third buffer while the second buffer is being diffed and the first buffer is being displayed. This is useful for high-frame-rate applications.

### 11.2 GPU Rendering

The frame buffer can be backed by a GPU texture instead of a CPU `Vec<Cell>`. The GPU handles diffing and ANSI encoding via compute shaders.

### 11.3 Partial Frame Buffers

For very large terminals (e.g., 400×100), we can split the frame buffer into tiles and only allocate tiles that contain content. This reduces memory usage for sparse layouts.
