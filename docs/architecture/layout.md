# Layout System

BetterTUI lays out nodes with **Taffy** (CSS flexbox) adapted to a terminal cell grid. Code: `packages/core/crates/engine/src/taffy.rs`.

## Mapping

- `LayoutEngine` wraps `taffy::TaffyTree<()>` plus a `HashMap<NodeId, taffy::NodeId>` mirror.
- `LayoutTreeSync` syncs arena nodes to the Taffy tree and caches results.
- `LayoutResult { x, y, width, height, content_width, content_height: u16 }`.

Taffy works in floats/pixels. BetterTUI configures Taffy so "pixel" means "terminal cell". `LayoutProps` uses `f32` internally; final positions are `u16`.

## Sizing & overflow

- `width/height` accept `Auto`, fixed, or percent.
- `overflow: Scroll` marks a scroll container; `ScrollExtent` tracks content vs viewport.
- `display: None` removes a node from layout (CSS `display: none`).

## Validation

`@bettertui/core` provides `validate(layout, style)` and `warnIfInvalid(layout, style, context)`.

## Edge cases

- Empty trees lay out root at terminal size.
- The tree invariant prevents cycles (`is_ancestor` guard).
- Zero-size nodes are valid (used for hit testing / focus).
