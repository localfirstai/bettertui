# Viewport Culling Architecture

## Overview

BetterTUI's viewport culling system optimizes rendering by skipping nodes outside the visible terminal area. It operates at the render tree construction phase — before any painting occurs.

## Pipeline Integration

```
Arena (node tree)
  → LayoutEngine (Taffy layout)
    → layout_results: HashMap<NodeId, LayoutResult>
      → build_render_tree_with_viewport(arena, results, Some(&viewport))
        → RenderTree (contains only visible + padding-buffer nodes)
          → Painter.paint(&render_tree, &ctx)
            → FrameBuffer
              → DirtyDiff (compared against snapshot)
                → RenderBackend.encode(buffer, dirty_regions)
```

## Core Data Structures

### `Viewport` (paint.rs)
The visible terminal region. Properties:
- `x`, `y`, `width`, `height` — terminal pixel coordinates
- `contains_rect(px, py, pw, ph)` — intersection test (misleading name)
- `intersect(other)` — narrows by another Viewport
- `offset(dx, dy)` — applies scroll offset
- `with_padding(n)` — expands by CULLING_PADDING (5) for smooth scroll

### `ClipBounds` (paint.rs)
Enforced by `overflow: hidden` or `overflow: scroll`. Uses the same
intersection logic as Viewport but is applied at the **clip** level,
preventing children from rendering outside their parent's bounds even
if they would otherwise be in the viewport.

### `PaintBounds` (paint.rs)
Stores node position + padding. Provides `content_rect()` for the
inner area after subtracting padding.

### `PositionedChild` (culling.rs)
```rust
pub struct PositionedChild {
    pub id: NodeId,
    pub start: u16,   // y (column) or x (row)
    pub size: u16,    // height or width
}
```

## Culling Algorithm

### 1. Per-Node Viewport Test (build.rs)

Each `build_node()` call:

1. **Display:none check** — skip immediately
2. **Opacity check** — skip if parent_opacity * opacity == 0.0
3. **Clip narrowing** — if node needs clip (overflow hidden/scroll), intersect
   viewport with node's layout rect. `None` intersection → cull entire subtree.
4. **Viewport intersection** — check if node intersects current viewport.
   No intersection → cull subtree.
5. **Scroll offset** — if `overflow: scroll`, offset viewport by
   `node.state.scroll_x` / `scroll_y` for children.
6. **Binary search culling** (for scroll containers with ≥32 children) —
   see section below.
7. **Recurse** into visible children with narrowed viewport.

### 2. Binary Search Culling (culling.rs)

For scroll containers with 32+ children and a known primary axis:

```
Input:  viewport, children[0..N] sorted by start position
Output: Vec<NodeId> — children intersecting expanded viewport

1. Expand viewport by CULLING_PADDING (5px)
2. Binary search for first overlapping child (O(log N))
3. Expand left:
   - Walk backward from candidate
   - Track consecutive non-overlapping children ("gaps")
   - Stop after 50 consecutive gaps (bounded look-behind)
4. Expand right:
   - Walk forward until child.start >= padded_viewport_end
5. Filter: check both primary and cross-axis intersection
6. Sort by zIndex
```

### 3. Small Array Bypass

Arrays with fewer than 16 children skip the binary search overhead and use
a linear scan instead.

## Key Constants

| Constant | Value | Purpose |
|----------|-------|---------|
| `CULLING_PADDING` | 5 | Extra buffer around viewport to prevent pop-in during scrolling |
| `BINARY_SEARCH_MIN_CHILDREN` | 32 | Minimum children to trigger binary search optimization |
| `SMALL_ARRAY_THRESHOLD` | 16 | Maximum children for linear scan bypass |
| `MAX_LOOK_BEHIND` | 50 | Maximum gap children to scan backward for spanning objects |

## Comparison with OpenTUI

### Similarities
- Same binary search + interval expansion pattern
- Same padding concept (OpenTUI uses 10, BetterTUI uses 5)
- Same bounded look-behind (50 in both)
- Same small array bypass (16 in both)
- Same primary-axis sorting requirement

### Differences
- **OpenTUI** has `getObjectsInViewport` in TypeScript (objects-in-viewport.ts)
  that operates on the JS side. The Rust engine also calls a similar function
  but only for scroll containers with 32+ children.
- **OpenTUI** applies padding to ALL 4 sides. BetterTUI applies it only along
  the primary axis direction in the culling function.
- **OpenTUI** does cross-axis filtering in the result loop. BetterTUI relies on
  the viewport intersection in `build_node()` for cross-axis culling.
- **OpenTUI** includes zIndex sorting in the result. BetterTUI handles
  z-ordering in the painter/rendering phase, not in culling.
- **OpenTUI's** padding is 10px vs BetterTUI's 5px.

## Test Coverage

25 passing tests across 3 modules:

### culling.rs unit tests (12)
- empty_children, zero_size_viewport, all_children_visible
- some_children_visible, no_children_visible, spanning_object_caught
- row_axis, small_array_bypasses_binary_search, gap_between_children
- partial_overlap_at_edge, padding_includes_nearby_objects, large_sparse_list

### build.rs integration tests (12)
- viewport_culling_inside, viewport_culling_outside
- viewport_culling_opacity_zero, viewport_culling_clip_narrows
- viewport_culling_scroll_offset, viewport_culling_partial_overlap
- viewport_culling_deep_tree, viewport_culling_nested_clip_narrows_viewport
- viewport_culling_outside_clip_skips_deep
- viewport_culling_multiple_children_some_visible
- viewport_culling_benchmark_large_tree (100 iterations)
- viewport_culling_benchmark_mostly_offscreen

### renderer integration tests (1)
- renderer_viewport_culling_pipeline

## Missing / Future Work

1. **TypeScript-side culling** — OpenTUI has `objects-in-viewport.ts` in JS.
   Currently BetterTUI only has Rust-side culling. A TS-side utility would
   be useful for framework-level optimizations.
2. **Cross-axis padding** — OpenTUI applies padding on all 4 sides. BetterTUI
   only applies it along the primary axis. This could cause pop-in during
   diagonal scrolling.
3. **Benchmarks against OpenTUI** — No performance comparison data exists yet.
4. **Dynamic viewport resize** — Currently viewport is fixed at terminal
   dimensions. Scroll containers could benefit from dynamic viewport updates.
