# Visibility Propagation Design

## Current State

| Property | Build skip | Paint skip | Subtree culled |
|----------|-----------|------------|----------------|
| `Display::None` | Yes (build.rs:47) | N/A | Yes |
| `opacity: 0` | No | Yes (render.rs:57) | **No** — subtree still built |
| `width=0 \|\| height=0` | No | No (no-op paint) | **No** |
| Off-screen | No | Partial (clip check) | **No** |
| Scroll-hidden | No | No | **No** (scroll offset not applied) |
| `style.hidden` | No | Partial (text hidden, bg paints) | No |

## Design Decisions

### 1. Visibility Propagation Rules

```
parent visibility → child visibility
```

| Parent state | Child render? | Child layout? | Notes |
|---|---|---|---|
| `display: none` | No | No | Already implemented |
| `opacity: 0` | **No** — subtree culled | Yes — layout needed for siblings | **New**: skip subtree in build |
| `width=0 \|\| height=0` | **No** — subtree culled | Yes — layout for siblings | **New**: skip when final size is 0 |
| Off-screen (outside viewport) | **No** — subtree culled | Yes — positions needed for scroll | **New**: skip in build |
| Clipped (outside clip rect) | **No** — not painted | Yes | Already paint-skipped; can also build-skip |

### 2. Opacity=0 Subtree Culling

**Change:** In `build_render_tree`, when `opacity == 0.0`, skip recursing into children.

**Rationale:** If parent is fully transparent, no child can be visible. Children still need layout updates (siblings depend on their size), but render tree building can skip them.

**Edge case:** `opacity: 0` with `overflow: visible` — children may overflow outside parent bounds but parent's opacity=0 still makes everything invisible. Safe to cull.

### 3. Zero-Size Culling

**Change:** In `build_render_tree`, when `layout.width == 0 || layout.height == 0`, skip node and its subtree.

**Rationale:** Zero-sized node renders nothing. No child can be visible through it. Children still need layout.

**Edge case:** Overflowing children — zero-sized parent with `overflow: visible` and children outside bounds. The children occupy space in layout. They can be culled from render because there's no visible area to show through.

### 4. Viewport Culling (Off-Screen)

**Change:** Introduce viewport bounds. In `build_render_tree`, compute effective viewport. If node's bounds don't intersect viewport, skip subtree.

**Viewport propagation:**
- Root viewport = terminal dimensions (0, 0, width, height)
- Clipped nodes narrow the viewport to their clip rect intersection
- Scrollable nodes offset the viewport by scroll position
- Children inherit the narrowed viewport

**Algorithm:**
```
fn build_node(arena, id, parent_opacity, viewport, clip_x, clip_y):
  if display == None: return
  if opacity == 0: return  // cull subtree
  if width == 0 || height == 0: return  // cull subtree
  if !intersects(layout.bounds, viewport): return  // cull subtree

  child_viewport = viewport
  if NEEDS_CLIP:
    child_viewport = intersect(viewport, ClipBounds(layout.x, layout.y, layout.width, layout.height))

  if SCROLLABLE:
    child_viewport = offset(child_viewport, -scroll_x, -scroll_y)

  recurse into children with child_viewport
```

### 5. Scroll Offset Integration

**Change:** Apply `NodeState.scroll_x`/`scroll_y` during render tree building.

- Node with `overflow: scroll` gets `NEEDS_CLIP`
- `PaintBounds` includes `translate_x`/`translate_y` offset
- Viewport is offset by scroll position so children are tested against the visible scroll window
- Children whose bounds fall outside the scroll viewport are culled

### 6. Dirty Flag Wiring

**Change:** Use `NodeState.layout_dirty` and `NodeState.render_dirty` to skip work:

- `layout_dirty == false` → skip Taffy layout sync for this node
- `render_dirty == false` → skip subtree in render tree build
- Dirty flags propagate: marking a child dirty marks ancestors as needing render

This avoids full tree rebuild when only a small subtree changed.

## Implementation Order

1. Wire `NodeState` dirty flags into render path (skip unchanged subtrees)
2. Add opacity=0 subtree culling to `build_render_tree`
3. Add zero-size subtree culling to `build_render_tree`
4. Add viewport abstraction and off-screen culling
5. Integrate scroll offset into viewport
6. Add nested viewport support (clip → viewport narrowing)
7. Optimise: binary search on sorted children for large scrollable lists
