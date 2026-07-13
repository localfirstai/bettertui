use bettertui_engine::framebuffer::{Cell, FrameBuffer};
use bettertui_engine::tree::{Color, NamedColor};

#[test]
fn test_framebuffer_snapshot() {
    let mut fb = FrameBuffer::new(10, 5);
    fb.write_str(
        0,
        0,
        "Hello",
        Color::Named(NamedColor::White),
        Color::Named(NamedColor::Black),
    );
    fb.write_str(
        0,
        1,
        "World",
        Color::Named(NamedColor::Green),
        Color::Default,
    );

    insta::assert_debug_snapshot!(fb);
}

#[test]
fn test_cell_snapshot() {
    let cell = Cell::new('A')
        .with_fg(Color::Named(NamedColor::Red))
        .with_bg(Color::Named(NamedColor::Blue));
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
