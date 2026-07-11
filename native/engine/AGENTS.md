# AGENTS.md

## Rendering Pipeline

- **LayoutTreeSync owns its own LayoutEngine.** `sync_full(arena)` takes only the arena — do NOT pass an engine reference. The engine is internal.
- **DirtyDiff::compute needs two FrameBuffers.** Signature: `compute(current, previous, generation)`. The Renderer must maintain a snapshot buffer from the previous frame.
- **Scheduler::begin_frame returns bool, not u64.** DirtyDiff needs a u64 generation counter. The Renderer maintains its own `generation: u64` field, not the scheduler's return value.
- **Painter owns its own FrameBuffer.** Can't paint into an external buffer. Use the snapshot pattern: `painter.buffer()` → diff → `snapshot.copy_from(painter.buffer())`.
- **FrameBuffer::write_str takes 5 args** — `(x, y, &str, fg: Color, bg: Color)`. Colors are required, not optional.
- **DirtyRegion::merge returns a new DirtyRegion.** It does NOT mutate self. `let merged = r1.merge(&r2);`
- **Arena children: use `append_child`, not `add_child`.** Common API mistake.
- **RenderNode::box_node() takes no arguments.** No name/label parameter — use `box_node()`, not `box_node("name")`.

## Layout Integration

- After `sync_full(arena)`, iterate arena parents and call `sync_children(arena, parent_id)` for each with children.
- Then call `compute(root_id, width, height)` to actually run Taffy layout.
- `LayoutTreeSync::results()` returns `&HashMap<NodeId, LayoutResult>` — feed this to `build_render_tree`.

## Clippy Patterns

- **Module inception:** `painter/painter.rs` triggers `clippy::module-inception` — rename inner file to `render.rs`.
- **Redundant closures:** `.map(|v| Type::Variant(v))` → `.map(Type::Variant)`.
- **Derive Default:** For structs with all-zero/default fields, use `#[derive(Default)]` instead of manual impl.
- **Too many arguments:** Use `#[allow(clippy::too_many_arguments)]` for builder functions with >7 params.

## FrameBuffer

- `copy_from(&mut self, other)` was added — not in original API. Needed for the double-buffer snapshot pattern.
- `resize()` does NOT preserve existing cell content. Tests should not assert old content survives resize.
- `get(x, y)` returns `&Cell` directly (not Option). Check `in_bounds()` first or get static EMPTY cell.
- Cell is `Copy` type — use `*cell` to dereference, not `.cloned()`.

## Compositor

- Layer z-index ordering: Background(0) < Content(10) < Selection(20) < Overlay(30) < Popup(40) < Tooltip(50) < Cursor(60).
- `get_cell()` returns `Option<Cell>` (not Option<&Cell>) since Cell is Copy.
- Use `cell.is_empty()` not `cell.is_transparent()` — Cell has no is_transparent method.

## Glyph Cache

- **Module inception:** `glyph/glyph.rs` triggers clippy — rename to `character.rs`.
- Emoji ranges: `0x2600..=0x26FF` overlaps with Symbol detection. Remove from emoji check to avoid false positives.
- Box drawing: Clippy catches `0x2500..=0x257F | 0x2580..=0x259F | 0x25A0..=0x25FF` — simplify to `0x2500..=0x25FF`.

## PTY API

- `PtyConfig.working_directory` (not `working_dir`).
- `PtyConfig.env` is `Vec<(String, String)>`, not HashMap.
- `PtyProcess::spawn(config)` takes only config — size is in config.size.

## Scheduler

- `RenderScheduler` replaced with enhanced `Scheduler` — update renderer imports.
- `Scheduler` supports priority queue, frame budgeting, animation frames, idle callbacks.

## Borrow Checker Patterns

- When iterating `&self.field` while calling `self.method()`, clone the collection first: `let items = self.field.clone(); for item in &items { self.method(item); }`.

## Local Font Bundling

- Use `include_bytes!("../../fonts/FontName.otf")` to embed fonts at compile time.
- Font files go in `native/engine/fonts/`.
- `LocalFontDetector` checks bundled font first, then system fonts.

## Widget Framework

- **Test modules need explicit imports of sibling types.** `super::*` in `pipeline.rs` tests only brings in `pipeline.rs` imports, not `WidgetId` from `widgets/mod.rs`. Add `use crate::widgets::WidgetId;` explicitly.
- **WidgetId is a tuple struct.** `WidgetId(pub NodeId)` — construct with `WidgetId(node_id)`, not field syntax.
- **WidgetContext requires lifetime annotation** in return types: `WidgetContext<'_>` not `WidgetContext`.
- **NodeArena::append_child returns Result.** Handle with `let _ = ctx.append_child(parent, child);` or propagate error.
- **Widget::create returns WidgetId.** The trait signature is `fn create(&self, ctx: &mut WidgetContext) -> WidgetId`.
- **FlexDirection, not Direction.** FlexWidget uses `FlexDirection::Column` (not `Direction::Vertical`).
- **Theme has no is_dark() method.** Check `theme.colors.is_empty()` or store the mode separately.
- **Key::Character(char) is the variant name.** Not `Key::Char` — it's `Key::Character(c)` in the enum.
- **BoxWidget and ContainerWidget need #[derive(Default)].** Clippy `derivable_impls` fires if manual impl is identical to derived.
