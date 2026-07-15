//! Tests for the frame buffer module.
//!
//! Covers [`Cell`], [`CellAttributes`], and [`FrameBuffer`] behavior.

use bettertui_engine::framebuffer::{Cell, CellAttributes, FrameBuffer};
use bettertui_engine::tree::{Color, NamedColor};

// ---------------------------------------------------------------------------
// Cell tests
// ---------------------------------------------------------------------------

#[test]
fn cell_new() {
    let c = Cell::new('A');
    assert_eq!(c.ch, 'A');
    assert_eq!(c.fg, Color::Default);
    assert_eq!(c.bg, Color::Default);
    assert!(c.attributes.is_empty());
}

#[test]
fn cell_default() {
    let c = Cell::default();
    assert_eq!(c.ch, ' ');
    assert!(c.is_empty());
}

#[test]
fn cell_with_fg_bg() {
    let c = Cell::new('X').with_fg(Color::Named(NamedColor::Red)).with_bg(Color::Named(NamedColor::Blue));
    assert_eq!(c.fg, Color::Named(NamedColor::Red));
    assert_eq!(c.bg, Color::Named(NamedColor::Blue));
}

#[test]
fn cell_is_empty() {
    let mut c = Cell::new(' ');
    assert!(c.is_empty());
    c.ch = 'A';
    assert!(!c.is_empty());
}

#[test]
fn cell_clear() {
    let mut c = Cell::new('X').with_fg(Color::Named(NamedColor::Red)).with_bg(Color::Named(NamedColor::Blue));
    c.attributes |= CellAttributes::BOLD;
    c.clear();
    assert!(c.is_empty());
    assert_eq!(c.ch, ' ');
}

#[test]
fn cell_attributes_bitflags() {
    let a = CellAttributes::BOLD | CellAttributes::ITALIC;
    assert!(a.contains(CellAttributes::BOLD));
    assert!(a.contains(CellAttributes::ITALIC));
    assert!(!a.contains(CellAttributes::UNDERLINE));
}

#[test]
fn cell_from_char() {
    let c: Cell = 'Z'.into();
    assert_eq!(c.ch, 'Z');
}

#[test]
fn cell_equality() {
    let a = Cell::new('A');
    let b = Cell::new('A');
    let c = Cell::new('B');
    assert_eq!(a, b);
    assert_ne!(a, c);
}

// ---------------------------------------------------------------------------
// FrameBuffer tests
// ---------------------------------------------------------------------------

#[test]
fn framebuffer_new() {
    let fb = FrameBuffer::new(80, 24);
    assert_eq!(fb.width(), 80);
    assert_eq!(fb.height(), 24);
}

#[test]
fn framebuffer_in_bounds() {
    let fb = FrameBuffer::new(80, 24);
    assert!(fb.in_bounds(0, 0));
    assert!(fb.in_bounds(79, 23));
    assert!(!fb.in_bounds(80, 24));
    assert!(!fb.in_bounds(80, 0));
}

#[test]
fn framebuffer_set_get() {
    let mut fb = FrameBuffer::new(10, 5);
    let cell = Cell::new('X');
    fb.set(3, 2, cell);
    assert_eq!(fb.get(3, 2).ch, 'X');
}

#[test]
fn framebuffer_fill_rect() {
    let mut fb = FrameBuffer::new(10, 5);
    let cell = Cell::new('#');
    fb.fill_rect(2, 1, 3, 2, cell);
    assert_eq!(fb.get(2, 1).ch, '#');
    assert_eq!(fb.get(4, 1).ch, '#');
    assert_eq!(fb.get(2, 2).ch, '#');
    assert_eq!(fb.get(5, 1).ch, ' ');
}

#[test]
fn framebuffer_write_str() {
    let mut fb = FrameBuffer::new(10, 5);
    fb.write_str(1, 0, "Hello", Color::Default, Color::Default);
    assert_eq!(fb.get(1, 0).ch, 'H');
    assert_eq!(fb.get(5, 0).ch, 'o');
    assert_eq!(fb.get(6, 0).ch, ' ');
}

#[test]
fn framebuffer_clear() {
    let mut fb = FrameBuffer::new(5, 3);
    fb.set(2, 1, Cell::new('X'));
    fb.clear();
    assert!(fb.get(2, 1).is_empty());
}

#[test]
fn framebuffer_resize() {
    let mut fb = FrameBuffer::new(10, 5);
    fb.set(5, 3, Cell::new('X'));
    fb.resize(20, 10);
    assert_eq!(fb.width(), 20);
    assert_eq!(fb.height(), 10);
    assert!(fb.get(5, 3).is_empty());
}

#[test]
fn framebuffer_diff() {
    let mut fb = FrameBuffer::new(3, 3);
    fb.swap();
    fb.set(1, 1, Cell::new('X'));
    let dirty = fb.diff();
    assert_eq!(dirty.len(), 1);
    assert_eq!(dirty[0], (1, 1));
}

#[test]
fn framebuffer_swap() {
    let mut fb = FrameBuffer::new(3, 3);
    fb.swap();
    fb.set(1, 1, Cell::new('A'));
    let dirty = fb.diff();
    assert_eq!(dirty.len(), 1);
}
