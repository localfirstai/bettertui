# Compositor & Screen

Layered compositing builds the final frame buffer from stacked layers; the screen module manages viewport, alternate screen, scrollback, and selection. Compositing primitives (`Rgba`, `blend_over`, `PaintFlags`) live in the engine's `tree.rs` / `graphics.rs`; the screen state and `ScreenBuffer` live in the `bettertui-terminal` crate (`screen` and `vt` modules).

## Layers

```mermaid
graph TD
    A[Compositor] --> B[Layer: Background 0]
    A --> C[Layer: Content 10]
    A --> D[Layer: Selection 20]
    A --> E[Layer: Overlay 30]
    A --> F[Layer: Popup 40]
    A --> G[Layer: Tooltip 50]
    A --> H[Layer: Cursor 60]
    B & C & D & E & F & G & H --> I[composite -> FrameBuffer]
```

- `Compositor { layers: Vec<Layer>, width, height }`: `add_layer(LayerType) -> LayerId`, `remove_layer`, `get_layer`, `composite(&mut FrameBuffer)`, `resize`, `clear`.
- `Layer { id, layer_type, visible, opacity, offset_x/y, buffer: FrameBuffer }`. `set_cell`, `get_cell(x,y) -> Option<Cell>` (by value), `set_char`, `fill`, `fill_rect`, `set_opacity` (clamped 0..1), `set_offset`, `is_empty`.
- `enum LayerType { Background(0), Content(10), Selection(20), Overlay(30), Popup(40), Tooltip(50), Cursor(60) }` — `z_index()` follows this ordering; `Selection`/`Cursor` are transparent.

## Screen state

The terminal crate provides screen state (`screen.rs`):

```rust
pub struct ScreenState {
    viewport: TerminalViewport,
    alternate_screen: AlternateScreen,
    cursor: CursorState,
    scrollback: ScrollbackBuffer,
    selection_active: bool,
    selection_start: Option<Point>,
    selection_end: Option<Point>,
    dirty: bool,
}
```

- `enter/leave_alternate_screen`, `scroll_up/down/reset`, `set/clear_selection`, `resize`, `push_scrollback_line`.
- `enum AlternateScreen { Main, Alternate }`, `enum CursorStyle { Block, Underline, Bar, Hidden }`.

## CompositorRenderer

The terminal crate's `screen.rs` drives the composite step. The compositor is independent of the arena; the `render` pipeline composites after painting into the content layer.
eline composites after painting into the content layer.
