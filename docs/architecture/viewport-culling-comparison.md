# Viewport Culling: OpenTUI vs BetterTUI Comparison

## Overview

Both systems implement binary-search-based viewport culling for scrollable
terminal UI. The pattern is adapted from OpenTUI's `getObjectsInViewport`.

## Architecture Comparison

| Aspect | OpenTUI | BetterTUI |
|--------|---------|-----------|
| Language | TypeScript (objects-in-viewport.ts) | Rust (culling.rs + build.rs) |
| Pipeline position | Framework-level | Render tree construction |
| Input type | Generic `ViewportObject[]` | `Vec<PositionedChild>` |
| Output type | Filtered + z-sorted array | `Vec<NodeId>` |
| Binary search | For 16+ items | For 32+ children in scroll containers |
| Linear scan bypass | `minTriggerSize` param | < 16 children |
| Padding | 10px (configurable) | 5px (CULLING_PADDING constant) |
| Look-behind limit | 50 | 50 |
| Cross-axis check | In result filter loop | In build.rs `contains_rect` |
| zIndex sorting | In culling result | Handled in painter phase |
| Negative coords | Supported (f64) | Not supported (u16) |

## Algorithm Comparison

### OpenTUI
```typescript
getObjectsInViewport(viewport, objects, direction, padding, minTriggerSize)
  1. Check empty / zero-size viewport → return []
  2. If objects.length < minTriggerSize → return all objects
  3. Apply padding to viewport (all 4 sides)
  4. Binary search for first overlapping child
  5. If no candidate → start from lo position
  6. Expand left with bounded look-behind (50 max gaps)
  7. Expand right (stop when start >= padded_end)
  8. Filter: check primary AND cross-axis intersection
  9. Sort by zIndex
```

### BetterTUI (Rust)
```rust
get_objects_in_viewport(viewport, &[PositionedChild], primary_axis)
  1. Check empty / zero-size viewport → return []
  2. If children.len() < 16 → linear scan filter
  3. Apply CULLING_PADDING (5) to viewport
  4. Binary search for first overlapping child
  5. If no candidate → expand_from(lo) with bounded look-behind
  6. Expand left with bounded look-behind (50 max gaps)
  7. Expand right (stop when start >= padded_end)
  8. Filter: check primary-axis overlap
  9. Cross-axis checks done by build.rs per-node
```

## Test Coverage Comparison

| Category | OpenTUI (TS) | BetterTUI (Rust) |
|----------|-------------|------------------|
| Empty input | ✅ | ✅ |
| Zero-size viewport | ✅ (3 tests) | ✅ (1 test) |
| All visible | ✅ | ✅ |
| Some visible | ✅ | ✅ |
| None visible | ✅ | ✅ |
| Row axis | ✅ | ✅ |
| Column axis | ✅ | ✅ |
| Small array bypass | ✅ (4 tests) | ✅ (1 test) |
| Padding behavior | ✅ (2 tests) | ✅ (1 test) |
| Boundary conditions | ✅ (3 tests) | ✅ (2 tests) |
| Cross-axis filtering | ✅ (2 tests) | ✅ (via build.rs) |
| Large objects spanning | ✅ (5 tests) | ✅ (1 test) |
| Sparse objects | ✅ (2 tests) | ✅ (1 test) |
| Clustered objects | ✅ (1 test) | ❌ |
| zIndex sorting | ✅ (3 tests) | ❌ (painter handles) |
| Negative coordinates | ✅ (2 tests) | ❌ (u16 limitation) |
| Overlapping objects | ✅ (2 tests) | ❌ |
| Realistic scroll | ✅ (4 tests) | ✅ (via build.rs) |
| Stress tests | ✅ (2 tests) | ✅ (2 benchmarks) |
| CLI/chat scenario | ✅ (1 test) | ❌ |
| Grid layout | ✅ (1 test) | ❌ |
| Single-pixel gaps | ✅ (1 test) | ❌ |

**Total unit tests:** OpenTUI ~50+, BetterTUI 25

## Implementation Differences

### 1. Coordinate types
OpenTUI uses `number` (f64) allowing negative coordinates. BetterTUI uses
`u16` for coordinates, which is appropriate for terminal rendering but
cannot represent offscreen-negative positions.

### 2. Padding application
OpenTUI applies padding to all 4 sides of the viewport (x - padding, x + width + padding,
y - padding, y + height + padding). BetterTUI only applies padding along the primary
axis direction. This means:
- OpenTUI catches more cross-axis near-misses
- BetterTUI may occasionally miss a child that's just outside the
  viewport on the cross-axis during diagonal scroll

Fix: Apply padding to the viewport in both axes before passing to
`get_objects_in_viewport`.

### 3. zIndex sorting
OpenTUI sorts results by zIndex. BetterTUI relies on tree-order + painter
z-ordering. Both approaches are valid; OpenTUI's is an optimization that
reduces painter work.

### 4. Binary search threshold
OpenTUI triggers at 16+ objects (configurable). BetterTUI's
`get_objects_in_viewport` triggers at 16+ but the call in `build.rs`
only happens for scroll containers with 32+ children. This means
flat trees with 16-31 children skip binary search.

### 5. `contains_rect` naming
BetterTUI's `Viewport::contains_rect` is actually an intersection check
(`r > self.x && px < self.right() && b > self.y && py < self.bottom()`).
OpenTUI correctly names this concept as overlap/intersection. The function
works correctly but the name is misleading.

## Gaps to Address

1. **TypeScript-side culling**: No `getObjectsInViewport` equivalent in our
   `@bettertui/core` TypeScript package. Useful for framework-level
   optimizations.

2. **Cross-axis padding**: BetterTUI applies padding only along primary
   axis. Should apply to both axes for consistent behavior.

3. **Negative coordinate handling**: `u16` prevents representing objects
   partially offscreen above/left of the terminal. Consider `i16` for
   scroll containers (already done: `Viewport::offset` uses `i32` and
   clamps to 0).

4. **Test coverage gaps**: Add tests for overlapping objects, clustered
   objects, single-pixel gaps, and realistic scrolling scenarios
   (chat, grid layout).

5. **Missing zIndex check in result**: OpenTUI sorts by zIndex. BetterTUI
   relies on painter z-ordering. Consider adding zIndex sort during
   culling for scroll containers with mixed z-order children.
