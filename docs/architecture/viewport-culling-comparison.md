# OpenTUI vs BetterTUI: Viewport Culling Comparison

## Rendering Architecture

| Aspect | OpenTUI | BetterTUI |
|--------|---------|-----------|
| Render tree | Built per-frame via `updateLayout()` recursion, cached via `canReuseRenderList` | Built per-frame from scratch via `build_render_tree()` — no caching |
| Layout engine | Yoga (JS FFI per node, deduplicated via `_lastLayoutFrame`) | Taffy (Rust native, full recompute every frame) |
| Paint | Zig native buffer with cell-level ops | Rust native framebuffer with SoA cells |
| Scheduler | JS timer + FPS cap + `requestLive()` ref-counting | Rust `Scheduler` struct with priority queue (but `begin_frame()` never called) |
| Command protocol | Direct Zig FFI calls per property | JSON serialization → napi-rs → Rust parse |

## Culling Mechanisms

| Mechanism | OpenTUI | BetterTUI |
|-----------|---------|-----------|
| Off-screen culling | `getObjectsInViewport()` — binary search + interval expansion | `get_objects_in_viewport()` — binary search + interval expansion on sorted children (`render_object/culling.rs`) |
| Subtree pruning | `_getVisibleChildren()` skips culled subtrees in `updateLayout()` | `build_render_tree_with_viewport()` prunes children outside the viewport (except `Display::None`) |
| Display:None skip | Yoga layout: display=none children excluded from layout | `build_node()` skips `Display::None` at line 47 |
| Opacity=0 skip | `_getVisibleChildren()` checks opacity < 1 for filtering | `Painter::is_visible()` checks opacity > 0 |
| Hidden skip | Not directly — relies on culling | `is_visible()` checks `HIDDEN` flag |
| Clipping | Scissor rect stack in buffer + parallel hit grid stack | `PaintContext` clip stack in painter |
| Z-index culling | None — z-sort in `_getVisibleChildren()` for visible-only | Full sort of all render objects `sorted_by_z_index()` |
| Layout skip for invisible | Children layouts still updated (`updateFromLayout()` always called) | Full layout sync every frame regardless of visibility |
| Frame suppression | `canReuseRenderList` skips entire `updateLayout` pass | `change_count` check skips full render if no changes |
| Render list caching | Cached across frames for static content | **None** — render tree rebuilt every frame |

## Key Algorithms

### Viewport Culling: OpenTUI
```
getObjectsInViewport(viewport, objects[], direction, padding, minTriggerSize)
  ├─ Early out: 0-size viewport, empty objects, <minTriggerSize
  ├─ Binary search for first overlapping object (O(log N))
  ├─ Left expansion (maxLookBehind=50) — catches spanning objects
  ├─ Right expansion — linear scan until past viewport
  └─ Cross-axis AABB filter + z-index sort
```
**Complexity:** O(log N + K) dense, O(log N + 50 + K) sparse.  
**Precondition:** Objects sorted by primary-axis start position.  
**Cache:** `childrenSortedByPrimaryAxis` lazy-recomputed on position change.

### Viewport Culling: BetterTUI
```
get_objects_in_viewport(viewport, objects[], direction, padding)
  ├─ Early out: 0-size viewport, empty objects
  ├─ Binary search for first overlapping object (O(log N), threshold BINARY_SEARCH_MIN_CHILDREN=32)
  ├─ Left/right expansion until past viewport
  └─ Cross-axis AABB filter
```
**Complexity:** O(log N + K). Precondition: children sorted by primary-axis start. `CULLING_PADDING = 5` rows of margin so partially-visible nodes are not clipped mid-scroll. Driven by `build_render_tree_with_viewport()` in `render_object/`.

## Clipping Systems

| Property | OpenTUI | BetterTUI |
|----------|---------|-----------|
| Scissor stack | Yes (native buffer + JS hit grid) | Yes (`PaintContext` clip stack, `push_clip()`/`pop_clip()`) |
| Nested clipping | Yes — stack-based, natural nesting | Yes — clip rect intersection |
| Clip to bounds | On `overflow:hidden/scroll` | On `NEEDS_CLIP` flag (overflow=Hidden/Scroll or clip prop) |
| Hit grid clipping | Parallel scissor stack in hit grid | **None** — hit grid not implemented |

## Dirty Tracking

| Mechanism | OpenTUI | BetterTUI |
|-----------|---------|-----------|
| Per-node dirty | `_dirty` flag on BaseRenderable | `NodeState { dirty, layout_dirty, render_dirty }` (flags defined but **unused** in render path) |
| Global change counter | None (tree revision bumped per structural change) | `Arena::change_count()` — single u64, checked for frame suppression |
| Layout generation | `RenderContext::bumpLayoutGeneration()` | `UpdateFlags` defined but never set or read |
| Render list revision | `RenderContext::bumpRenderListRevision()` | **None** |

## Wasted Work Comparison

| Waste | OpenTUI | BetterTUI |
|-------|---------|-----------|
| Off-screen content processed | **No** — culled before `updateLayout` recursion | **No** — `build_render_tree_with_viewport()` prunes off-screen nodes |
| Full buffer clear each frame | **No** — incremental painting with dirty regions | **Yes** — `paint_with_clear()` clears all cells |
| Cell-by-cell diff each frame | **No** — native Zig tracks dirty cells | **Yes** — `DirtyDiff::compute()` scans all cells |
| Full layout recompute | **No** — Yoga incremental, `_lastLayoutFrame` guard | **Yes** — Taffy `compute_layout()` from scratch |
| JSON serialization overhead | **No** — direct FFI calls | **Yes** — every command batch serialized to JSON |
| Layout for hidden nodes | **Yes** (intentional — `updateFromLayout()` always called to keep positions correct for culling) | **Yes** (unintentional — no visibility check) |
| Tree rebuild from scratch | Only when render list invalidated | **Yes** — every frame |

## Optimisations Present Only in OpenTUI

1. **Binary search viewport culling** — `getObjectsInViewport()` O(log N)
2. **Render list caching** — skip `updateLayout` for static content
3. **Layout FFI deduplication** — `_lastLayoutFrame` guard, one call per node per frame
4. **Primary-axis sort cache** — lazy sorted children array
5. **Parallel hit grid scissoring** — mouse event clipping matches viewport clipping
6. **Subtree pruning** — culled children's `updateLayout` + `render` skipped entirely
7. **Small array bypass** — `minTriggerSize=16` avoids binary search overhead
8. **Culling padding** — `padding=10` for smooth scrolling buffer
9. **`requestLive()` ref-counting** — continuous rendering only when animations active

## Optimisations Present Only in BetterTUI

1. **`NodeState` flags** — `layout_dirty` and `render_dirty` defined for incremental updates
2. **`Display::None` skip** — excluded from render tree build
3. **Frame suppression via `change_count`** — skip entire render when nothing changed
4. **Viewport culling** — `get_objects_in_viewport()` binary-search interval culling with `CULLING_PADDING = 5`

## Conclusion

Both engines implement viewport culling via binary-search interval search on primary-axis-sorted children. BetterTUI's `build_render_tree_with_viewport()` prunes off-screen subtrees; remaining work is full buffer diff, full tree rebuild when dirty, and JSON command serialization.

**Remaining low-hanging fruit:**
1. Wire existing `NodeState` flags into the render path (`layout_dirty`, `render_dirty`)
2. Incremental render tree (reuse unchanged nodes)
3. Dirty-region-only buffer clear instead of full clear
