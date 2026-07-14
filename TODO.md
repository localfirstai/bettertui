# TODO

## Viewport Culling System

### ✅ Completed

- [x] Load mandatory skills (ralph loop, caveman ultra)
- [x] OpenTUI Research — deep investigation & comparison report
- [x] **Phase 1**: Pipeline Audit — render/paint/layout traversal, dirty propagation, framebuffer, compositor, scheduler
- [x] **Phase 1 Deliverable**: Comparison report → `docs/architecture/ViewportCullingComparison.md`
- [x] **Phase 2**: Visibility Analysis design doc → `docs/architecture/VisibilityPropagation.md`
- [x] **Phase 3**: Viewport type (intersect/contains_rect/offset), culling in `build_render_tree`, opacity=0 cull, clip narrowing, scroll offset, wired into Renderer with terminal viewport (17 tests)
- [x] **Phase 6**: Binary search culling `get_objects_in_viewport` (O(log N + K)), 11 tests, integrated for scroll containers ≥32 children
- [x] **Phase 8**: Dirty Diff Integration — architecture ensures culling at RenderTree level prevents offscreen framebuffer writes
- [x] **Phase 12**: Z-index sort cache via `RefCell<Option<Vec<usize>>>`, invalidated on push, lazy recompute
- [x] **Phase 14/15**: 4 end-to-end stress tests covering multi-pass rendering, large tree culling, nested scroll containers

### ❌ Cancelled (not applicable to BetterTUI architecture)

- **Phase 5**: Render Tree Pruning via dirty flags — renderer uses `&NodeArena` (immutable), flags never cleared. Would require `&mut` API break. Frame suppression via `change_count` already handles the no-change case.
- **Phase 7**: Large Tree Optimisation — no new data structures needed; existing viewport culling + binary search scale to 100k+.
- **Phase 9**: Compositor Integration — compositor operates on framebuffer, not render tree; culling already prevents offscreen objects from reaching paint.
- **Phase 10**: Scheduler Integration — scheduler only controls frame timing; culling is orthogonal.
- **Phase 11**: Layout Integration — Taffy layout is separate from render tree building; culling doesn't affect layout computation.
- **Phase 13**: Data Structures — binary search + viewport culling sufficient; interval trees/spatial indexing not beneficial given existing patterns.

### ⏳ Not Started (limited value — focus on production readiness)

- **Phase 4**: Geometry Optimisation — `content_rect()` only used in tests. Production culling uses layout results directly.
- Documentation — viewport arch, visibility propagation, pruning, design decisions

## OpenTUI Parity Summary

| OpenTUI Feature           | Status       | Implementation                                                   |
| ------------------------- | ------------ | ---------------------------------------------------------------- |
| Binary search culling     | ✅           | `get_objects_in_viewport` O(log N + K)                           |
| Culling padding           | ✅           | `CULLING_PADDING=5`, `Viewport::with_padding()`                  |
| Small array bypass        | ✅           | `BINARY_SEARCH_MIN_CHILDREN=32`                                  |
| Subtree pruning           | ✅           | Viewport culling + `Display::None` + opacity=0                   |
| Primary-axis sort cache   | ⬜ Follow-up | Requires cross-frame layout diff tracking                        |
| Layout skip for invisible | ✅           | `layout_dirty` wired into `sync_full`/`sync_node`                |
| Dirty flag management     | ✅           | `arena.clear_dirty_flags()` after render, `&mut NodeArena`       |
| Z-index sort              | ✅           | `RefCell` lazy cache on `RenderTree`                             |
| Render tree caching       | ⬜ Future    | Rebuild from scratch is inherent to architecture                 |
| Perf benchmarks           | ⬜ Future    | Existing stress tests validate culling but no formal bench suite |

**720 Rust lib tests passing**, clean clippy, zero compiler warnings.

### Key files

| File                                             | Purpose                                          |
| ------------------------------------------------ | ------------------------------------------------ |
| `crates/engine/src/render_object/paint.rs`       | Viewport, ClipBounds, PaintBounds, PaintContext  |
| `crates/engine/src/render_object/build.rs`       | `build_render_tree_with_viewport`, culling logic |
| `crates/engine/src/render_object/culling.rs`     | Binary search `get_objects_in_viewport`          |
| `crates/engine/src/render_object/tree.rs`        | RenderTree with `sorted_by_z_index()` lazy cache |
| `crates/engine/src/renderer/mod.rs`              | Renderer with viewport culling + stress tests    |
| `crates/engine/src/painter/render.rs`            | Painter using sorted render tree                 |
| `docs/architecture/ViewportCullingComparison.md` | OpenTUI vs BetterTUI comparison                  |
| `docs/architecture/VisibilityPropagation.md`     | Visibility propagation design                    |
