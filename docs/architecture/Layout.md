# Layout

> The layout system calculates positions and sizes for every node.
> It uses Taffy (CSS flexbox engine) adapted for terminal grids.

## 1. Overview

BetterTUI uses **Taffy** for layout calculation. Taffy implements CSS flexbox and grid, adapted for terminal cells instead of pixels.

```
Node Arena (with dirty flags)
    ↓
Taffy Node Tree (mirrored)
    ↓
Layout Calculation
    ↓
Layout Results (position + size per node)
```

### 1.1 Why Taffy?

Taffy is the same layout engine used by Leptos, Dioxus, and other Rust UI frameworks. It implements CSS flexbox correctly, has built-in layout caching, and is actively maintained.

**Alternative considered:** Writing a custom layout engine. Rejected because flexbox is complex (the CSS spec is 200+ pages) and Taffy already solves this correctly.

**Trade-off:** Taffy is designed for pixel-based layouts, not terminal cells. We must adapt its output (pixel values) to terminal grid positions. This is a well-understood mapping: 1 cell = 1 character width × 1 line height.

## 2. Taffy Integration

### 2.1 Node Mapping

Each `RenderNode` in the arena has a corresponding `taffy::Node` in Taffy's tree:

```
BetterTUI Arena          Taffy Tree
    NodeA            →    TaffyNodeA
    ├── NodeB        →    ├── TaffyNodeB
    └── NodeC        →    └── TaffyNodeC
```

**Why a mirror tree, not direct integration:** Taffy owns its node tree. We cannot store Taffy nodes inside our arena (different ownership model). The mirror tree is kept in sync via the command processing pipeline.

### 2.2 Style Mapping

BetterTUI `LayoutProps` maps to Taffy's `Style`:

```rust
impl From<&LayoutProps> for taffy::Style {
    fn from(props: &LayoutProps) -> Self {
        taffy::Style {
            display: match props.display {
                Display::Flex => taffy::Display::Flex,
                Display::None => taffy::Display::None,
            },
            flex_direction: match props.direction {
                FlexDirection::Row => taffy::FlexDirection::Row,
                FlexDirection::Column => taffy::FlexDirection::Column,
                FlexDirection::RowReverse => taffy::FlexDirection::RowReverse,
                FlexDirection::ColumnReverse => taffy::FlexDirection::ColumnReverse,
            },
            justify_content: map_justify(props.justify),
            align_items: map_align(props.align),
            flex_grow: props.flex_grow,
            flex_shrink: props.flex_shrink,
            flex_basis: map_sizing(props.flex_basis),
            gap: map_gap(props.gap),
            padding: map_rect(props.padding),
            margin: map_rect(props.margin),
            size: map_size(props.width, props.height),
            min_size: map_size(props.min_width, props.min_height),
            max_size: map_size(props.max_width, props.max_height),
            position: map_position(props.position, props.inset),
            ..Default::default()
        }
    }
}
```

### 2.3 Sizing Adaptation

Taffy works in pixels. We need to convert to terminal cells:

```rust
/// Convert Taffy's pixel output to terminal cell count.
pub fn pixels_to_cells(pixels: f32) -> u16 {
    // In terminal UI, 1 cell = 1 character width.
    // Taffy's pixel values map 1:1 to cell counts.
    // This is because we set Taffy's "pixel" unit to mean "cells".
    (pixels.round() as i32).max(0) as u16
}
```

**Why this works:** We configure Taffy to treat its "pixel" unit as "terminal cells." When we create Taffy styles, we use cell counts as pixel values. Taffy then performs flexbox calculations in cell units. The output is directly usable as terminal positions.

### 2.4 Terminal Size as Root Constraints

The root node receives the terminal dimensions as its constraints:

```rust
let root_size = taffy::Size {
    width: terminal_width as f32,   // e.g., 120 cells
    height: terminal_height as f32,  // e.g., 40 cells
};

taffy.compute_layout(root_node, taffy::Constraints {
    min_width: Some(terminal_width as f32),
    max_width: Some(terminal_width as f32),
    min_height: Some(terminal_height as f32),
    max_height: Some(terminal_height as f32),
})?;
```

## 3. Layout Algorithm

### 3.1 Incremental Layout

When nodes change, we don't recalculate the entire tree. Only dirty subtrees are recalculated:

```
1. Identify dirty nodes (layout_dirty = true)
2. For each dirty node:
   a. Mark ancestors as needing recalculation
   b. Update Taffy style for the changed node
3. Recalculate layout starting from the lowest common ancestor of all dirty nodes
4. Store results on each node
```

**Why incremental matters:** A full layout of 1000 nodes takes ~1ms. An incremental layout of 10 dirty nodes takes ~0.01ms. For interactive applications with frequent small updates, this is a 100x improvement.

### 3.2 Layout Propagation

Layout propagates top-down (constraints) and bottom-up (sizes):

```
Top-Down (Constraints):
  Root: { width: 120, height: 40 }
    → BoxA: { width: 120, height: 40 }
      → TextB: { width: 50, height: 1 }  (text content width)
      → BoxC: { width: 70, height: 40 }

Bottom-Up (Sizes):
  TextB: measured size = { width: 50, height: 1 }
  BoxC: measured size = { width: 70, height: 40 }
  BoxA: computed size = { width: 120, height: 40 }
```

### 3.3 Flexbox Algorithm

Taffy implements the CSS flexbox algorithm:

1. **Determine flex direction:** Row (horizontal) or column (vertical).
2. **Collect flex items:** Children with flex-grow > 0 or flex-shrink > 0.
3. **Calculate free space:** Parent size minus fixed-size children.
4. **Distribute positive free space:** Grows items proportionally by flex-grow.
5. **Distribute negative free space:** Shrinks items proportionally by flex-shrink.
6. **Align items:** Apply justify-content and align-items.
7. **Position items:** Compute final positions from sizes and alignment.

### 3.4 Auto-Sizing

Nodes with `width: Auto` or `height: Auto` are sized based on their content:

- **Text nodes:** Width = text length (in cells), height = 1.
- **Container nodes:** Width/height = sum of children's sizes (in flex direction).
- **Leaf nodes with no content:** Width = 0, height = 0.

**Why f32:** Auto-sizing requires measuring text, which produces fractional values in some edge cases (e.g., percentage-based sizing). We use f32 throughout and only round to integers at the final step.

## 4. Layout Caching

### 4.1 Cache Invalidation

Taffy maintains a layout cache for each node. The cache is invalidated when:

- The node's layout properties change.
- The node's children change (add/remove/reorder).
- The node's parent changes.
- The terminal is resized.

### 4.2 Cache Strategy

```
Node Cache Entry:
  - style_hash: u64      (hash of layout properties)
  - children_hash: u64   (hash of children list)
  - known_size: Size     (cached computed size)
  - known_layout: Rect   (cached final position)
  - valid: bool          (whether cache is still valid)
```

**Cache hit:** If style_hash and children_hash match, return known_size and known_layout.

**Cache miss:** Recalculate layout and update cache.

### 4.3 Cache Performance

For a tree with 1000 nodes where 5 nodes change:
- Without cache: 1000 node calculations.
- With cache: 5 node calculations + 995 cache hits.
- **Speedup: ~200x.**

## 5. Position and Size

### 5.1 Layout Result

```rust
pub struct LayoutResult {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub content_width: u16,
    pub content_height: u16,
    pub scroll_x: i32,
    pub scroll_y: i32,
}
```

- `x, y` — absolute position in terminal cells (from top-left).
- `width, height` — outer size including padding and border.
- `content_width, content_height` — inner size (excluding padding and border).
- `scroll_x, scroll_y` — scroll offset for scrollable containers.

### 5.2 Coordinate System

```
(0, 0)─────────────────────(width, 0)
  │                              │
  │         Terminal Grid        │
  │                              │
(0, height)──────────────(width, height)
```

Coordinates are in terminal cells. (0, 0) is the top-left corner. x increases to the right. y increases downward.

### 5.3 Absolute vs Relative Positioning

**Relative positioning (default):** Nodes are positioned by the flexbox algorithm. Position is determined by parent's layout, siblings, and flex properties.

**Absolute positioning:** Nodes are removed from the flex flow and positioned relative to their parent's content box. Use `inset` (top, right, bottom, left) for offsets.

```rust
// Relative: positioned by flexbox
LayoutProps { position: Position::Relative, .. }

// Absolute: positioned by inset values
LayoutProps {
    position: Position::Absolute,
    inset: Some(RectValues {
        top: Some(0),
        left: Some(10),
        ..Default::default()
    }),
    ..Default::default()
}
```

## 6. Scroll Containers

### 6.1 Scroll Behavior

Nodes with `overflow: Scroll` become scroll containers:

1. Content is measured (may exceed container size).
2. Container size is fixed by layout constraints.
3. Content is offset by `scroll_x` and `scroll_y`.
4. A scrollbar is rendered (optional).

### 6.2 Scroll Extent

```rust
pub struct ScrollExtent {
    pub content_width: u32,
    pub content_height: u32,
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub scroll_x: i32,
    pub scroll_y: i32,
    pub max_scroll_x: i32,
    pub max_scroll_y: i32,
}
```

- `content_width/height` — total size of scrollable content.
- `viewport_width/height` — visible area size.
- `scroll_x/y` — current scroll offset.
- `max_scroll_x/y` — maximum scroll offset (content - viewport).

### 6.3 Scroll Clamping

Scroll offsets are clamped to valid ranges:

```rust
scroll_x = scroll_x.clamp(0, max_scroll_x);
scroll_y = scroll_y.clamp(0, max_scroll_y);
```

This prevents scrolling beyond the content boundaries.

## 7. Responsive Behavior

### 7.1 Terminal Resize

When the terminal is resized:

1. Root constraints are updated.
2. Layout is recalculated from the root.
3. All nodes are repositioned.
4. Frame is re-rendered.

### 7.2 Adaptive Layouts

Nodes can respond to size changes:

```rust
// Use flex-grow to fill available space
LayoutProps { flex_grow: 1.0, .. }

// Use percentage sizing for proportional layouts
LayoutProps { width: Some(Sizing::Percent(0.5)), .. }

// Use auto sizing for content-adaptive layouts
LayoutProps { width: Some(Sizing::Auto), .. }
```

## 8. Edge Cases

### 8.1 Empty Trees

An empty tree (no children) produces no layout output. The root node is still laid out with terminal dimensions.

### 8.2 Circular References

The tree invariant prevents circular parent-child references. The `is_ancestor` check prevents operations that would create cycles.

### 8.3 Overflow Content

Content that exceeds its container is handled by the `overflow` property:
- `Visible` — renders outside bounds (may overlap siblings).
- `Hidden` — clipped at bounds.
- `Scroll` — clipped, but scrollable.

### 8.4 Zero-Size Nodes

Nodes with zero width and/or height are valid. They occupy no space but are still part of the tree (for event handling, focus, etc.).

## 9. Performance

### 9.1 Layout Complexity

- **Full layout:** O(n) where n is the number of nodes.
- **Incremental layout:** O(k × d) where k is the number of dirty nodes and d is the average tree depth.
- **Cache lookup:** O(1) per node.

### 9.2 Memory

- Taffy allocates one `Style` and one `Layout` per node.
- Cache entries are ~64 bytes each.
- For 1000 nodes: ~128KB of Taffy data.

### 9.3 Benchmarks

| Scenario | Nodes | Time |
|----------|-------|------|
| Static tree (no changes) | 1000 | 0ms (cached) |
| Single node change | 1000 | 0.01ms |
| 10 node changes | 1000 | 0.05ms |
| Full tree rebuild | 1000 | 1ms |
| Terminal resize | 1000 | 1ms |
