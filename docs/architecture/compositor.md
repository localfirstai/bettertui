# Compositor

Compositing produces the final frame buffer by combining layers. Primitive types (`Rgba`, `blend_over`, `PaintFlags`) live in `packages/core/crates/engine/src/tree.rs` and `graphics.rs`. Screen state — viewport, alternate screen, scrollback, selection — lives in `terminal/screen.rs`.

## Compositing primitives

The engine defines compositing primitives in `tree.rs` / `graphics.rs`:

- `Rgba` color type with `blend_over` compositing
- `PaintFlags` for paint control
- `LayerType` enum: `Background(0)`, `Content(10)`, `Selection(20)`, `Overlay(30)`, `Popup(40)`, `Tooltip(50)`, `Cursor(60)` — `z_index()` follows this ordering
- Painting operations: `set_cell`, `get_cell(x,y)`, `set_char`, `fill`, `fill_rect`, `set_opacity`

There is no standalone `Compositor { layers: Vec<Layer> }` struct. The compositor pipeline composites after painting into the content layer using the rendering pipeline.

## Screen state (`terminal/screen.rs`)

```
ScreenState {
    viewport: TerminalViewport,
    alternate_screen: AlternateScreen,
    cursor: CursorState,
    scrollback: ScrollbackBuffer,
    selection_active: bool,
    selection_start/end: Option<Point>,
    dirty: bool,
}
```

- `enter/leave_alternate_screen`, `scroll_up/down/reset`, `set/clear_selection`, `resize`
- `AlternateScreen { Main, Alternate }`, `CursorStyle { Block, Underline, Bar, Hidden }`

The `render` pipeline composites after painting into the content layer.
