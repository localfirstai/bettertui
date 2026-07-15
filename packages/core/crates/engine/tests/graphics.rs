//! Tests for the graphics module.

use bettertui_engine::framebuffer::{Cell, CellAttributes, FrameBuffer};
use bettertui_engine::graphics::{DrawStyle, GraphicsContext, Point, Rect};
use bettertui_engine::tree::Color;

#[test]
fn rect_contains() {
    let rect = Rect::new(5, 5, 10, 10);
    assert!(rect.contains(Point::new(5, 5)));
    assert!(rect.contains(Point::new(14, 14)));
    assert!(!rect.contains(Point::new(4, 5)));
    assert!(!rect.contains(Point::new(5, 15)));
}

#[test]
fn rect_edges() {
    let rect = Rect::new(2, 3, 10, 5);
    assert_eq!(rect.right(), 12);
    assert_eq!(rect.bottom(), 8);
}

#[test]
fn draw_style_chain() {
    let style = DrawStyle::new().fg(Color::rgb(255, 0, 0)).bg(Color::rgb(0, 0, 0)).bold().italic();
    assert!(style.fg.is_some());
    assert!(style.bg.is_some());
    assert!(style.attributes.contains(CellAttributes::BOLD));
    assert!(style.attributes.contains(CellAttributes::ITALIC));
}

#[test]
fn graphics_clear() {
    let mut fb = FrameBuffer::new(10, 10);
    fb.set(0, 0, Cell::new('x'));
    let mut gfx = GraphicsContext::new(&mut fb);
    gfx.clear();
    assert!(gfx.buffer().get(0, 0).is_empty());
}

#[test]
fn draw_char() {
    let mut fb = FrameBuffer::new(10, 10);
    let mut gfx = GraphicsContext::new(&mut fb);
    let style = DrawStyle::new().fg(Color::rgb(255, 0, 0));
    gfx.draw_char(0, 0, 'A', &style);
    assert_eq!(gfx.buffer().get(0, 0).ch, 'A');
}

#[test]
fn draw_str() {
    let mut fb = FrameBuffer::new(10, 10);
    let mut gfx = GraphicsContext::new(&mut fb);
    let style = DrawStyle::new();
    gfx.draw_str(0, 0, "hello", &style);
    assert_eq!(gfx.buffer().get(0, 0).ch, 'h');
    assert_eq!(gfx.buffer().get(4, 0).ch, 'o');
}

#[test]
fn draw_hline() {
    let mut fb = FrameBuffer::new(10, 10);
    let mut gfx = GraphicsContext::new(&mut fb);
    let style = DrawStyle::new();
    gfx.draw_hline(2, 0, 5, '-', &style);
    assert_eq!(gfx.buffer().get(2, 0).ch, '-');
    assert_eq!(gfx.buffer().get(6, 0).ch, '-');
    assert!(gfx.buffer().get(7, 0).is_empty());
}

#[test]
fn fill_rect() {
    let mut fb = FrameBuffer::new(10, 10);
    let mut gfx = GraphicsContext::new(&mut fb);
    let style = DrawStyle::new();
    gfx.fill_rect(Rect::new(1, 1, 3, 3), '#', &style);
    assert_eq!(gfx.buffer().get(1, 1).ch, '#');
    assert_eq!(gfx.buffer().get(3, 3).ch, '#');
    assert!(gfx.buffer().get(0, 0).is_empty());
}

#[test]
fn clear_rect() {
    let mut fb = FrameBuffer::new(10, 10);
    let mut gfx = GraphicsContext::new(&mut fb);
    let style = DrawStyle::new();
    gfx.fill_rect(Rect::new(0, 0, 10, 10), '#', &style);
    gfx.clear_rect(Rect::new(2, 2, 3, 3));
    assert_eq!(gfx.buffer().get(0, 0).ch, '#');
    assert!(gfx.buffer().get(2, 2).is_empty());
}
