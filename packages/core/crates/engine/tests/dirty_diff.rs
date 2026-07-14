//! Tests for the dirty diff module.

use bettertui_engine::dirty_diff::{DirtyDiff, DirtyRegion};
use bettertui_engine::framebuffer::{Cell, FrameBuffer};

#[test]
fn dirty_region_new() {
    let r = DirtyRegion::new(5, 10, 20, 15);
    assert_eq!(r.x, 5);
    assert_eq!(r.y, 10);
    assert_eq!(r.width, 20);
    assert_eq!(r.height, 15);
}

#[test]
fn dirty_region_edges() {
    let r = DirtyRegion::new(5, 10, 20, 15);
    assert_eq!(r.right(), 25);
    assert_eq!(r.bottom(), 25);
    assert_eq!(r.area(), 300);
}

#[test]
fn dirty_region_contains() {
    let r = DirtyRegion::new(5, 5, 10, 10);
    assert!(r.contains(5, 5));
    assert!(r.contains(14, 14));
    assert!(!r.contains(4, 5));
    assert!(!r.contains(15, 15));
}

#[test]
fn dirty_region_intersects() {
    let a = DirtyRegion::new(0, 0, 10, 10);
    let b = DirtyRegion::new(5, 5, 10, 10);
    let c = DirtyRegion::new(20, 20, 5, 5);
    assert!(a.intersects(&b));
    assert!(!a.intersects(&c));
}

#[test]
fn dirty_region_merge() {
    let a = DirtyRegion::new(0, 0, 5, 5);
    let b = DirtyRegion::new(5, 0, 5, 5);
    let merged = a.merge(&b);
    assert_eq!(merged, DirtyRegion::new(0, 0, 10, 5));
}

#[test]
fn dirty_region_can_merge_horizontal() {
    let a = DirtyRegion::new(0, 0, 5, 5);
    let b = DirtyRegion::new(5, 0, 5, 5);
    let c = DirtyRegion::new(0, 5, 5, 5);
    assert!(a.can_merge_horizontal(&b));
    assert!(!a.can_merge_horizontal(&c));
}

#[test]
fn dirty_region_can_merge_vertical() {
    let a = DirtyRegion::new(0, 0, 5, 5);
    let b = DirtyRegion::new(0, 5, 5, 5);
    let c = DirtyRegion::new(5, 0, 5, 5);
    assert!(a.can_merge_vertical(&b));
    assert!(!a.can_merge_vertical(&c));
}

#[test]
fn dirty_diff_no_changes() {
    let mut a = FrameBuffer::new(5, 5);
    let mut b = FrameBuffer::new(5, 5);
    a.swap();
    b.swap();
    let mut diff = DirtyDiff::new();
    let regions = diff.compute(&a, &b, 1);
    assert!(regions.is_empty());
}

#[test]
fn dirty_diff_with_changes() {
    let mut a = FrameBuffer::new(5, 5);
    let mut b = FrameBuffer::new(5, 5);
    a.swap();
    b.swap();
    a.set(2, 2, Cell::new('X'));
    let mut diff = DirtyDiff::new();
    let regions = diff.compute(&a, &b, 1);
    assert!(!regions.is_empty());
}

#[test]
fn dirty_diff_full_repaint() {
    let mut diff = DirtyDiff::new();
    let regions = diff.compute_full_repaint(80, 24);
    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0], DirtyRegion::new(0, 0, 80, 24));
}

#[test]
fn dirty_diff_generation_caching() {
    let mut a = FrameBuffer::new(5, 5);
    let mut b = FrameBuffer::new(5, 5);
    a.swap();
    b.swap();
    a.set(0, 0, Cell::new('A'));
    let mut diff = DirtyDiff::new();
    let r1 = diff.compute(&a, &b, 1);
    let len1 = r1.len();
    let r2 = diff.compute(&a, &b, 1);
    assert_eq!(r2.len(), len1);
}

#[test]
fn dirty_diff_merge_regions() {
    let mut a = FrameBuffer::new(10, 10);
    let mut b = FrameBuffer::new(10, 10);
    a.swap();
    b.swap();
    a.set(2, 2, Cell::new('X'));
    a.set(3, 2, Cell::new('Y'));
    a.set(2, 3, Cell::new('Z'));
    a.set(3, 3, Cell::new('W'));
    let mut diff = DirtyDiff::new();
    let regions = diff.compute(&a, &b, 1);
    assert!(regions.len() <= 2);
}

#[test]
fn dirty_diff_total_area() {
    let mut diff = DirtyDiff::new();
    diff.compute_full_repaint(80, 24);
    assert_eq!(diff.total_area(), 80 * 24);
}

fn make_diff_with_cells(cells: &[(u16, u16)]) -> Vec<DirtyRegion> {
    let mut fb = FrameBuffer::new(20, 20);
    let empty = FrameBuffer::new(20, 20);
    for &(x, y) in cells {
        fb.set(x, y, Cell::new('X'));
    }
    let mut diff = DirtyDiff::new();
    diff.compute(&fb, &empty, 1).to_vec()
}

#[test]
fn merge_cells_horizontal_span() {
    let regions = make_diff_with_cells(&[(0, 0), (1, 0), (2, 0)]);
    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0].x, 0);
    assert_eq!(regions[0].width, 3);
    assert_eq!(regions[0].height, 1);
}

#[test]
fn merge_cells_vertical_stack() {
    let regions = make_diff_with_cells(&[(0, 0), (0, 1), (0, 2)]);
    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0].y, 0);
    assert_eq!(regions[0].height, 3);
}

#[test]
fn merge_cells_separate_regions() {
    let regions = make_diff_with_cells(&[(0, 0), (5, 0), (0, 5), (5, 5)]);
    assert_eq!(regions.len(), 4);
}

#[test]
fn merge_cells_rectangle() {
    let mut cells = Vec::new();
    for y in 0..3 {
        for x in 0..5 {
            cells.push((x, y));
        }
    }
    let regions = make_diff_with_cells(&cells);
    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0].width, 5);
    assert_eq!(regions[0].height, 3);
}

#[test]
fn merge_cells_non_contiguous_vertical() {
    let regions = make_diff_with_cells(&[(0, 0), (0, 1), (0, 3), (0, 4)]);
    assert!(regions.len() >= 2);
}

#[test]
fn merge_cells_no_changes() {
    let fb = FrameBuffer::new(10, 10);
    let empty = FrameBuffer::new(10, 10);
    let mut diff = DirtyDiff::new();
    let regions = diff.compute(&fb, &empty, 1);
    assert!(regions.is_empty());
}

#[test]
fn merge_adjacent_horizontal_regions() {
    let mut regions = vec![DirtyRegion::new(0, 0, 5, 1), DirtyRegion::new(5, 0, 5, 1)];
    DirtyDiff::merge_adjacent_regions(&mut regions);
    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0].width, 10);
}

#[test]
fn merge_adjacent_vertical_regions() {
    let mut regions = vec![DirtyRegion::new(0, 0, 5, 3), DirtyRegion::new(0, 3, 5, 3)];
    DirtyDiff::merge_adjacent_regions(&mut regions);
    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0].height, 6);
}

#[test]
fn merge_non_adjacent_regions_unchanged() {
    let mut regions = vec![DirtyRegion::new(0, 0, 3, 3), DirtyRegion::new(10, 10, 3, 3)];
    DirtyDiff::merge_adjacent_regions(&mut regions);
    assert_eq!(regions.len(), 2);
}

#[test]
fn compute_returns_merged_regions() {
    let mut current = FrameBuffer::new(20, 10);
    let mut previous = FrameBuffer::new(20, 10);
    current.swap();
    previous.swap();
    current.set(0, 0, Cell::new('X'));
    current.set(1, 0, Cell::new('Y'));
    current.set(0, 1, Cell::new('Z'));
    current.set(1, 1, Cell::new('W'));
    let mut diff = DirtyDiff::new();
    let regions = diff.compute(&current, &previous, 1);
    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0].width, 2);
    assert_eq!(regions[0].height, 2);
}
