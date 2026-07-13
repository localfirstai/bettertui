//! Binary search viewport culling for large scrollable lists.
//!
//! Pattern adapted from OpenTUI's `getObjectsInViewport`.
//! Uses binary search + interval expansion to find visible children
//! in O(log N + K) time where K is the number of visible objects.

use super::paint::Viewport;
use crate::tree::NodeId;

/// A positioned child in a sorted array for binary search culling.
#[derive(Debug, Clone, Copy)]
pub struct PositionedChild {
    pub id: NodeId,
    /// Primary-axis start position (y for column layout, x for row layout).
    pub start: u16,
    /// Primary-axis size (height for column, width for row).
    pub size: u16,
}

/// Padding to apply when culling — keeps a buffer of visible objects
/// just outside the viewport for smooth scrolling. Matches OpenTUI's padding.
pub const CULLING_PADDING: u16 = 5;

/// Returns children that intersect the given viewport along the primary axis.
///
/// `children` must be pre-sorted by `start` ascending.
/// Uses binary search for O(log N) lookup, then expands left/right.
///
/// This is specifically for scroll containers with many children.
/// The viewport should already be offset by scroll position.
///
/// A `CULLING_PADDING` buffer is applied so objects just outside the
/// viewport are still included (prevents pop-in during smooth scrolling).
pub fn get_objects_in_viewport(
    viewport: &Viewport,
    children: &[PositionedChild],
    primary_axis: PrimaryAxis,
) -> Vec<NodeId> {
    if children.is_empty() || viewport.width == 0 || viewport.height == 0 {
        return Vec::new();
    }

    // Apply culling padding to prevent pop-in during scrolling
    let vp_padded = viewport.with_padding(CULLING_PADDING);

    // Small arrays: skip binary search overhead
    if children.len() < 16 {
        return children
            .iter()
            .filter(|c| {
                let end = c.start.saturating_add(c.size);
                end > viewport_start(&vp_padded, primary_axis)
                    && c.start < viewport_end(&vp_padded, primary_axis)
            })
            .map(|c| c.id)
            .collect();
    }

    let vp_start = viewport_start(&vp_padded, primary_axis);
    let vp_end = viewport_end(&vp_padded, primary_axis);

    // Binary search for first overlapping child
    let mut lo = 0i32;
    let mut hi = children.len() as i32 - 1;
    let mut candidate: Option<usize> = None;

    while lo <= hi {
        let mid = ((lo + hi) >> 1) as usize;
        let c = &children[mid];
        let end = c.start.saturating_add(c.size);

        if end <= vp_start {
            lo = mid as i32 + 1;
        } else if c.start >= vp_end {
            hi = mid as i32 - 1;
        } else {
            candidate = Some(mid);
            break;
        }
    }

    let Some(center) = candidate else {
        // Viewport is in a gap — start from where search ended.
        // Clamp to last valid index since `lo` can be children.len()
        // when all children are before the viewport.
        let start_idx = (lo.max(0) as usize).min(children.len().saturating_sub(1));
        return expand_from(children, start_idx, vp_start, vp_end, primary_axis);
    };

    // Expand left with bounded look-behind for spanning objects
    let max_look_behind = 50;
    let mut left = center;
    let mut gap_count = 0;

    while left > 0 {
        let prev = &children[left - 1];
        let prev_end = prev.start.saturating_add(prev.size);

        if prev_end <= vp_start {
            gap_count += 1;
            if gap_count >= max_look_behind {
                break;
            }
        } else {
            gap_count = 0;
        }
        left -= 1;
    }

    // Expand right
    let mut right = center + 1;
    while right < children.len() {
        let next = &children[right];
        if next.start >= vp_end {
            break;
        }
        right += 1;
    }

    // Collect visible children
    children[left..right]
        .iter()
        .filter(|c| {
            let end = c.start.saturating_add(c.size);
            end > vp_start && c.start < vp_end
        })
        .map(|c| c.id)
        .collect()
}

fn expand_from(
    children: &[PositionedChild],
    start_idx: usize,
    vp_start: u16,
    vp_end: u16,
    _axis: PrimaryAxis,
) -> Vec<NodeId> {
    let mut result = Vec::new();
    // Scan backward
    let mut i = start_idx as i32;
    let mut look_behind = 0;
    while i >= 0 {
        let c = &children[i as usize];
        let end = c.start.saturating_add(c.size);
        if end > vp_start.saturating_sub(10) && c.start < vp_end {
            result.push(c.id);
            look_behind = 0;
        } else {
            look_behind += 1;
            if look_behind >= 50 {
                break;
            }
        }
        i -= 1;
    }
    result.reverse();
    // Scan forward with early termination
    let mut i = start_idx;
    while i < children.len() {
        let c = &children[i];
        if c.start >= vp_end {
            break;
        }
        let end = c.start.saturating_add(c.size);
        if end > vp_start {
            result.push(c.id);
        }
        i += 1;
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryAxis {
    Column, // Sort by y (vertical layout)
    Row,    // Sort by x (horizontal layout)
}

fn viewport_start(vp: &Viewport, axis: PrimaryAxis) -> u16 {
    match axis {
        PrimaryAxis::Column => vp.y,
        PrimaryAxis::Row => vp.x,
    }
}

fn viewport_end(vp: &Viewport, axis: PrimaryAxis) -> u16 {
    match axis {
        PrimaryAxis::Column => vp.bottom(),
        PrimaryAxis::Row => vp.right(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sorted_children(
        start: u16,
        count: usize,
        step: u16,
        size: u16,
    ) -> Vec<PositionedChild> {
        let mut arena = crate::tree::NodeArena::new();
        (0..count)
            .map(|i| {
                let id = arena.insert(crate::tree::RenderNode::new(crate::tree::NodeKind::Box));
                PositionedChild {
                    id,
                    start: start + i as u16 * step,
                    size,
                }
            })
            .collect()
    }

    #[test]
    fn empty_children() {
        let vp = Viewport::new(0, 0, 80, 24);
        let result = get_objects_in_viewport(&vp, &[], PrimaryAxis::Column);
        assert!(result.is_empty());
    }

    #[test]
    fn zero_size_viewport() {
        let children = make_sorted_children(0, 5, 5, 3);
        let vp = Viewport::new(0, 0, 0, 0);
        let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
        assert!(result.is_empty());
    }

    #[test]
    fn all_children_visible() {
        let children = make_sorted_children(0, 5, 2, 1);
        let vp = Viewport::new(0, 0, 10, 10);
        let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn some_children_visible() {
        let children = make_sorted_children(0, 20, 2, 1);
        let vp = Viewport::new(0, 0, 10, 5);
        let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
        assert!(result.len() < 20);
        assert!(!result.is_empty());
        // With padding of 5, viewport expands to y=0..15 → first 8 children (y=0..14)
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn no_children_visible() {
        let children = make_sorted_children(100, 5, 5, 3);
        let vp = Viewport::new(0, 0, 10, 10);
        let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
        assert!(result.is_empty());
    }

    #[test]
    fn spanning_object_caught() {
        let mut arena = crate::tree::NodeArena::new();
        let tall_id = arena.insert(crate::tree::RenderNode::new(crate::tree::NodeKind::Box));
        let children = vec![PositionedChild {
            id: tall_id,
            start: 0,
            size: 50,
        }];
        let vp = Viewport::new(0, 30, 10, 10);
        let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], tall_id);
    }

    #[test]
    fn row_axis() {
        let children = make_sorted_children(0, 10, 5, 3);
        let vp = Viewport::new(5, 0, 10, 10);
        let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Row);
        // Children at x=0..3 (idx 0), x=5..8 (idx 1), x=10..13 (idx 2), x=15..18 (idx 3)
        // Padded viewport x=0..30 → catches all 4 children in range
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn small_array_bypasses_binary_search() {
        let children = make_sorted_children(0, 15, 2, 1);
        let vp = Viewport::new(0, 5, 10, 5);
        let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
        assert!(!result.is_empty());
        assert!(result.len() < 15);
    }

    #[test]
    fn gap_between_children() {
        let children = make_sorted_children(0, 3, 100, 5);
        let vp = Viewport::new(0, 50, 10, 10);
        let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
        assert!(result.is_empty());
    }

    #[test]
    fn partial_overlap_at_edge() {
        let children = make_sorted_children(0, 5, 5, 10);
        let vp = Viewport::new(0, 8, 10, 5);
        let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
        assert_eq!(
            result.len(),
            4,
            "with padding should catch 4 overlapping children"
        );
    }

    #[test]
    fn padding_includes_nearby_objects() {
        let children = make_sorted_children(10, 3, 10, 5);
        let vp = Viewport::new(0, 0, 80, 10);
        let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
        assert_eq!(
            result.len(),
            1,
            "padding should include child just below viewport"
        );
    }

    #[test]
    fn large_sparse_list() {
        let children = make_sorted_children(0, 1000, 10, 5);
        let vp = Viewport::new(0, 5000, 80, 24);
        let result = get_objects_in_viewport(&vp, &children, PrimaryAxis::Column);
        assert!(
            result.len() <= 3,
            "should find only overlapping children from large sparse list"
        );
    }
}
