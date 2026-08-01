# Dirty Diff Engineering Audit — Phase 1

## Current Architecture: BetterTUI

### Rendering Pipeline

1. **Layout Sync:** `LayoutTreeSync.sync_full(arena)` → register all nodes
2. **Layout Compute:** `layout_sync.compute(root, width, height)` → Taffy layout
3. **Build RenderTree:** `build_render_tree(arena, results)` → flat RenderObject list
4. **Paint:** `painter.paint(&render_tree, &ctx)` → FULL clear + repaint
5. **Scheduler:** `scheduler.begin_frame()` → marks frame timing (_**called AFTER paint**_)
6. **Diff:** `dirty_diff.compute(painter.buffer(), &snapshot, generation)` → dirty regions
7. **Encode:** `backend.encode(painter.buffer(), &dirty_regions)` → ANSI bytes
8. **Snapshot:** `snapshot.copy_from(painter.buffer())` → save for next frame

### DirtyDiff Implementation (`src/dirty_diff/diff.rs`)

- **`find_dirty_cells`:** O(W*H) scan, allocates `Vec<(u16,u16)>` per frame
- **`merge_cells_to_regions`:** Creates boolean grid (W*H bytes), greedy horizontal+vertical merge
- **Generation caching:** Returns cached regions if generation unchanged
- **`compute_full_repaint`:** Single region covering full terminal

### Issues Found

1. **Full clear on every paint:** `Painter::paint()` calls `buffer.clear()` — O(n) per frame
2. **Full scan always:** DirtyDiff scans all cells even when few changes
3. **Per-node dirty flags unused:** `NodeState` has dirty/layout_dirty/render_dirty but render pipeline never reads them
4. **Generation granularity:** Single `u64` for entire arena — any mutation invalidates ALL cached regions
5. **Scheduler begin_frame timing:** Called at END of `render()`, not start — means timing includes layout+render work from previous conceptual frame
6. **Format! in move_to:** `AnsiBackend::move_to()` uses `format!()` per row — heap allocation
7. **No run-length optimization:** Each cell in dirty region emits individual SGR sequence
8. **Region merging limitations:** Greedy horizontal+vertical extension only — fragmented regions not merged
9. **CompositorRenderer simpler:** Single bounding-box region, no DirtyDiff reuse

### Memory/Allocation Profile

- `find_dirty_cells`: `Vec<(u16, u16)>` = up to W*H tuples per frame
- `merge_cells_to_regions`: `Vec<bool>` grid = W*H bytes, `Vec<bool>` visited = W*H bytes
- `merge_cells_to_regions`: `Vec<DirtyRegion>` output
- `Painter::paint()`: Full clear iterates all cells (O(W*H) writes)
- `AnsiBackend`: `Vec<u8>` buffer with 4096 initial capacity, grows as needed

---

## Comparison: BetterTUI vs Reference

### Diff Strategy

| Aspect          | Reference                   | BetterTUI                  | Impact                                                                |
| --------------- | --------------------------- | -------------------------- | --------------------------------------------------------------------- |
| Scan type       | Full-screen cell scan       | Full-screen → region merge | Reference omits region merge step                                     |
| Region tracking | None (cell-level only)      | DirtyRegion merging        | BetterTUI has more complex but potentially more efficient ANSI output |
| Cache strategy  | No cache                    | Generation-based           | BetterTUI can skip frame if no changes                                |
| Lazy frames     | Only emits if cells changed | Always emits               | Reference produces 0 bytes for unchanged frames                       |
| Run-length      | Yes (style runs coalesced)  | Per-cell SGR               | Reference significantly less ANSI bytes                               |
| Cursor diff     | Yes (cached state)          | No                         | Reference avoids redundant cursor sequences                           |

### Performance Considerations

1. **Reference's approach scales with screen size, not change count:** It scans ALL cells every frame but uses SoA layout (4 parallel arrays) for cache-friendly access.
2. **BetterTUI's approach has overhead from region merging:** It does the same full scan, then allocates grid + visited arrays to merge cells into rectangles.
3. **Run-length encoding in Reference** reduces ANSI output significantly for same-styled runs.
4. **No-op frames in Reference** emit 0 bytes vs BetterTUI always emits hide/show cursor.

### Key Advantage: Reference

- **Frame suppression:** No ANSI emitted for identical frames
- **Style coalescing:** Run-length encoding across adjacent cells
- **SoA buffer layout:** Better cache locality during diff
- **Lazy sync envelopes:** No cursor hide/show for blank frames

### Key Advantage: BetterTUI

- **Region merging:** Fewer cursor move operations for contiguous dirty areas
- **Generation caching:** Can skip diff entirely if no mutations
- **Compositor layer system:** More sophisticated compositing pipeline
- **Per-node dirty flags:** Potential for subtree-level invalidation (currently unused)

---

## Completed Improvements (v1 Dirty Diff)

| #   | Improvement                          | Status | Impact                                          |
| --- | ------------------------------------ | ------ | ----------------------------------------------- |
| 1   | Row-scan diff (no intermediate Vec)  | ✅     | Eliminated up to W*H tuple allocation per frame |
| 2   | Grid-free merge algorithm            | ✅     | Eliminated 2x `Vec<bool>` (W*H each) allocation |
| 3   | Post-merge region adjacency pass     | ✅     | Fewer, larger regions → fewer cursor moves      |
| 4   | Stack buffer for cursor positioning  | ✅     | No `format!()` heap allocation per dirty row    |
| 5   | Frame lifecycle: generation at start | ✅     | Correct DirtyDiff generation tracking           |
| 6   | end_frame() for budget tracking      | ✅     | Frame timing properly recorded                  |
| 7   | Pre-allocated region Vec capacity    | ✅     | Reduced reallocations during merge              |
| 8   | 38 Rust integration tests (Phase 2)  | ✅     | Behavior verification for all scenarios         |
| 9   | 21 TypeScript behavior tests         | ✅     | Command flow & reconciler verification          |
| 10  | 10 new DirtyDiff unit tests          | ✅     | Merge algorithm, regions, edge cases            |
| 11  | Complete engineering audit           | ✅     | docs/architecture/DirtyDiffAudit.md             |
| 12  | Reference comparison report          | ✅     | Embedded in audit                               |

## Remaining Gaps (Post-v1)

1. **Frame suppression:** Emit 0 bytes for identical frames
2. **Per-node dirty tracking:** Use NodeState flags to skip unchanged subtrees
3. **Incremental paint:** Paint only changed nodes instead of full clear
4. **SoA buffer layout:** FrameBuffer Cell as parallel arrays for cache locality
5. **Run-length ANSI coalescing** within dirty regions
6. **Cursor diffing:** Cache and diff cursor state
7. **Pre-allocated ANSI buffer** with spike reclamation
