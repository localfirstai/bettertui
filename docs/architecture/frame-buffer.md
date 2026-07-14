# Frame Buffer

The frame buffer is the cell grid the renderer writes into and diffs against the previous frame. Code: `packages/core/crates/engine/src/framebuffer/`. Dirty-region diffing: `packages/core/crates/engine/src/dirty_diff/`.

## Structure

```mermaid
classDiagram
    class FrameBuffer {
        +u16 width
        +u16 height
        +Vec~Cell~ cells
        +Vec~Cell~ back
        +new(w, h) FrameBuffer
        +get(x, y) &Cell
        +set(x, y, cell)
        +fill_rect(rect, cell)
        +write_str(x, y, text, fg, bg)
        +swap()
        +copy_from(other)
        +diff() Vec~(u16,u16)~
        +clear()
    }
    class Cell {
        +char: char
        +fg: Color
        +bg: Color
        +underline_color: Color
        +attributes: CellAttributes
        +is_empty() bool
    }
    class CellAttributes {
        +BOLD
        +ITALIC
        +UNDERLINE
        +DIM
        +STRIKETHROUGH
        +INVERSE
        +HIDDEN
    }
    FrameBuffer "1" o-- "*" Cell
    Cell *-- CellAttributes
```

- `Cell` is `Copy`. `get()` returns a static `EMPTY` cell when out of bounds (never `Option`) — out-of-bounds writes are silently ignored.
- `CellAttributes` is a `bitflags` type (one byte).
- `Color` stores intent (`Named`/`Indexed`/`Rgb`/`Default`); resolved to the terminal's best representation only at encode time.

## Dirty-region diffing

```mermaid
flowchart TD
    A[RenderFrame: current] --> B[DirtyDiff.compute current, previous, generation]
    B --> C{generation unchanged?}
    C -- yes --> D[empty regions -> skip encode]
    C -- no --> E[compare cells -> dirty cells]
    E --> F[merge into DirtyRegion rectangles]
    F --> G[AnsiBackend encodes only regions]
```

- `DirtyRegion { x, y, width, height: u16 }` with `merge`, `intersects`, `contains`, `can_merge_horizontal/vertical`, `area`.
- `DirtyDiff { regions, last_generation }`; `compute_full_repaint(w, h)` for full repaints.

## ANSI encoding

The `AnsiBackend` (in `render/render.rs`) produces ANSI bytes:
- cursor moves (`ESC[{row};{col}H`, plus relative moves for short hops)
- SGR for colors/attributes, with style coalescing so adjacent same-style cells share one sequence
- character output, cursor hide/show around the write

> Width handling: CJK/emoji occupy 2 cells; `unicode-width` drives measurement. Combining characters are currently rendered as separate cells (full combining support is future work).
