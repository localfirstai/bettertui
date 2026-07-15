use bettertui_engine::framebuffer::{Cell, FrameBuffer};
use bettertui_engine::taffy::{
    BoxSizing, FlexDirection, LayoutOverflow, LayoutProps, LayoutResult, Position, RectValues, Sizing, Viewport,
};
use bettertui_engine::tree::{Color, NamedColor};

#[test]
fn test_framebuffer_snapshot() {
    let mut fb = FrameBuffer::new(10, 5);
    fb.write_str(0, 0, "Hello", Color::Named(NamedColor::White), Color::Named(NamedColor::Black));
    fb.write_str(0, 1, "World", Color::Named(NamedColor::Green), Color::Default);

    insta::assert_debug_snapshot!(fb);
}

#[test]
fn test_cell_snapshot() {
    let cell = Cell::new('A').with_fg(Color::Named(NamedColor::Red)).with_bg(Color::Named(NamedColor::Blue));
    insta::assert_debug_snapshot!(cell);
}

#[test]
fn test_color_snapshot() {
    let colors = vec![
        Color::Named(NamedColor::Red),
        Color::Named(NamedColor::Green),
        Color::Named(NamedColor::Blue),
        Color::rgb(255, 128, 0),
        Color::Default,
    ];
    insta::assert_debug_snapshot!(colors);
}

#[test]
fn test_named_color_snapshot() {
    let named = vec![
        NamedColor::Black,
        NamedColor::Red,
        NamedColor::Green,
        NamedColor::Yellow,
        NamedColor::Blue,
        NamedColor::Magenta,
        NamedColor::Cyan,
        NamedColor::White,
    ];
    insta::assert_debug_snapshot!(named);
}

#[test]
fn test_layout_result_snapshot() {
    let result = LayoutResult::new(5, 10, 80, 24);
    let with_border = LayoutResult {
        border_top: 1,
        border_right: 1,
        border_bottom: 1,
        border_left: 1,
        padding_top: 2,
        padding_right: 2,
        padding_bottom: 2,
        padding_left: 2,
        content_width: 74,
        content_height: 20,
        ..result
    };
    insta::assert_debug_snapshot!(with_border);
}

#[test]
fn test_layout_props_snapshot() {
    let props = LayoutProps {
        display: bettertui_engine::taffy::types::Display::Flex,
        position: Position::Relative,
        direction: FlexDirection::Column,
        width: Some(Sizing::Points(80.0)),
        height: Some(Sizing::Percent(50.0)),
        min_width: Some(Sizing::Points(20.0)),
        max_height: Some(Sizing::Points(100.0)),
        padding: Some(RectValues::uniform(2.0)),
        margin: Some(RectValues::new(1.0, 0.0)),
        gap: Some(bettertui_engine::taffy::Gap::uniform(1.0)),
        flex_grow: 1.0,
        aspect_ratio: Some(1.5),
        overflow: Some(LayoutOverflow::Hidden),
        box_sizing: Some(BoxSizing::BorderBox),
        ..Default::default()
    };
    insta::assert_debug_snapshot!(props);
}

#[test]
fn test_viewport_snapshot() {
    let vp = Viewport::new(0, 0, 80, 24);
    let padded = vp.with_padding(5);
    let offset = vp.offset(10, 5);
    insta::assert_debug_snapshot!((vp, padded, offset));
}
