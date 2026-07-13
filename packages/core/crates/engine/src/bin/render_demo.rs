use bettertui_engine::framebuffer::FrameBuffer;
use bettertui_engine::tree::{Color, NamedColor};

fn main() {
    let mut fb = FrameBuffer::new(40, 5);
    fb.write_str(
        0,
        0,
        "Hello from BetterTUI Engine!",
        Color::Named(NamedColor::Green),
        Color::Default,
    );
    fb.write_str(
        0,
        2,
        "Width: 40, Height: 5",
        Color::Named(NamedColor::White),
        Color::Default,
    );

    for y in 0..5 {
        for x in 0..40 {
            let cell = fb.get(x, y);
            print!("{}", cell.ch);
        }
        println!();
    }
}
