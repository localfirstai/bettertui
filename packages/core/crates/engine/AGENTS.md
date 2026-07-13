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
- `PtyProcess::is_running(&mut self)` — takes `&mut self` (calls `child.try_wait()`). Cannot be called from `&self` context.
- `TerminalRuntime::is_running(&self)` works by checking its own `TerminalState`, NOT by calling `PtyProcess::is_running()`.

## Scheduler

- `RenderScheduler` replaced with enhanced `Scheduler` — update renderer imports.
- `Scheduler` supports priority queue, frame budgeting, animation frames, idle callbacks.

## Borrow Checker Patterns

- When iterating `&self.field` while calling `self.method()`, clone the collection first: `let items = self.field.clone(); for item in &items { self.method(item); }`.

## FFI Bridge (`ffi/mod.rs`)

- **Must match actual Engine API.** The FFI module was rewritten because it assumed methods (`width()`, `height()`, `resize()`, `EngineError`) that don't exist on `Engine`. Always verify the struct's actual public API before writing FFI functions.
- **`NodeId` (DefaultKey) field is private.** `NodeId` is `slotmap::DefaultKey` — field `0` is private. Cannot access `node.0` directly. Must use `NodeArena::insert()` to get valid IDs.
- **Test helpers cannot construct NodeId directly.** `mouse/mod.rs` and `selection/mod.rs` test helpers must use `NodeArena::insert()` instead of non-existent `crate::tree::Key` or `crate::tree::GenerationalIndex`.
- **Editor::set_content must reset engine state.** Calls `engine.clear()` not `buffer_mut().clear()` to properly reset cursor position.
- Struct field and method with the same name (e.g., `status: ProcessStatus` field + `fn status()` method): accessing `self.status` in the impl block goes to the FIELD. Call the field when you mean it, or rename if ambiguous.

## Local Font Bundling

- Use `include_bytes!("../../fonts/FontName.otf")` to embed fonts at compile time.
- Font files go in `packages/core/crates/engine/fonts/`.
- `LocalFontDetector` checks bundled font first, then system fonts.

## Error Chaining

- `terminal_process` error chain: `PtyError` → `TerminalError` → `NeovimError`. Each impls `From<T>` for the next level up, so `?` auto-converts. Adding new error types should follow this chain.

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
- **TextEngine doesn't derive Debug.** Types containing it (like Editor) need manual Debug impl or `#[derive(Default)]` won't work with `#[derive(Debug)]`.
- **NodeKind has no Inline or Image variants.** Variants are: Text, Box, Flex, Input, List, Table, Tree, Scroll, Tab, Modal, Spacer, Separator, Custom(u16).

## Testing Philosophy — TDD with 100% Coverage

BetterTUI enforces **test-driven development** with a goal of **100% test coverage** for all Rust engine code. Every implementation must be accompanied by proper tests before it is considered complete.

### Testing Stack

| Layer | Tool | Purpose |
|-------|------|---------|
| Unit tests | `cargo test` | Test individual functions and modules in isolation |
| Snapshot tests | `insta` | Capture and review complex output (FrameBuffer, Cell, Color, etc.) |
| E2E with PTY | `portable-pty` | Spawn real processes in a pseudo-terminal for true terminal integration tests |
| ANSI parsing | `vt100` | Parse raw ANSI output into a virtual `Screen` for readable assertions |

### E2E Testing Pattern (portable-pty + vt100)

```
Rust Engine → ANSI output → portable-pty → vt100 Parser → virtual Screen → assertions
```

Example flow:
```rust
let mut process = PtyProcess::spawn(config)?;
let output = read_pty_output(&mut process, timeout);
process.kill()?;

let mut parser = vt100::Parser::new(24, 80, 0);
parser.process(&output);

let screen = parser.screen();
assert!(screen.contents().contains("expected text"));
assert_eq!(screen.cell(0, 0).unwrap().fgcolor(), expected_color);
```

### TDD Rules

- **Test-first**: Write the test before the implementation. Red → Green → Refactor.
- **Every feature must have tests**: No exception. A feature without tests is incomplete.
- **E2E tests for all terminal interactions**: Any code path that produces ANSI output, handles input, or manages terminal state must have a PTY + vt100 e2e test.
- **Snapshots for complex data structures**: Use `insta::assert_debug_snapshot!` for FrameBuffer, Cell, Color, and any other structured data that is expensive to assert field-by-field.
- **No binary targets for testing**: E2E tests use `portable-pty` directly to spawn processes — do NOT add `trycmd` or binary-only testing harnesses.
- **Coverage gate**: All new code must maintain or improve line coverage. Do not merge code that drops coverage.
