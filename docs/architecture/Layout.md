# Layout System

BetterTUI lays out nodes with **Taffy** (CSS flexbox) adapted to a terminal cell grid. Code lives in `native/engine/src/layout/`.

## Mapping

```mermaid
graph LR
    A[NodeArena: LayoutProps] -->|LayoutTreeSync| B[TaffyTree]
    B -->|compute_layout root, w, h| C[LayoutResult per node]
    C --> D[RenderNode positioning]
```

- `LayoutEngine` wraps `taffy::TaffyTree<()>` plus a `HashMap<NodeId, taffy::NodeId>` mirror.
- `LayoutTreeSync` owns the engine and a results cache; `sync_full(arena)` takes only the arena (no separate engine ref).
- `LayoutResult { x, y, width, height, content_width, content_height: u16 }`.

## Why cells are pixels

Taffy works in floats/pixels. BetterTUI configures Taffy so its "pixel" unit means "terminal cell": styles use cell counts as pixel values, and Taffy's output is rounded to integers only at the end. `LayoutProps` uses `f32` internally (flex math needs fractions); final positions are `u16`.

```mermaid
flowchart TD
    A[arena dirty: layout_dirty] --> B[LayoutTreeSync.sync_full]
    B --> C[translate LayoutProps -> taffy::Style]
    C --> D[taffy.compute_layout root, w, h]
    D --> E[collect_results: HashMap~NodeId, LayoutResult~]
    E --> F[renderer reads positions]
```

## Sizing & overflow

- `width/height` accept `Auto`, fixed, or percent.
- `overflow: Scroll` marks a scroll container; `ScrollExtent` tracks content vs viewport and clamps offsets.
- `display: None` removes a node from layout entirely (CSS `display: none`).

## Edge cases

- Empty trees still lay out the root at terminal size.
- The tree invariant prevents cycles (`is_ancestor` guard).
- Zero-size nodes are valid (used for hit testing / focus).
