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
