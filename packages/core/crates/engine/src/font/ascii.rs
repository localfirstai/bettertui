use crate::framebuffer::{Cell, FrameBuffer};
use crate::tree::Color;

pub type AsciiFontName = &'static str;

pub const FONT_NAMES: &[AsciiFontName] = &[
    "tiny",
    "block",
    "shade",
    "slick",
    "huge",
    "grid",
    "pallet",
    "console",
    "simple",
    "simple3d",
    "chrome",
    "simpleblock",
    "3d",
];

fn to_cfonts_font(name: &str) -> Option<cfonts::Fonts> {
    Some(match name {
        "tiny" => cfonts::Fonts::FontTiny,
        "block" => cfonts::Fonts::FontBlock,
        "shade" => cfonts::Fonts::FontShade,
        "slick" => cfonts::Fonts::FontSlick,
        "huge" => cfonts::Fonts::FontHuge,
        "grid" => cfonts::Fonts::FontGrid,
        "pallet" => cfonts::Fonts::FontPallet,
        "console" => cfonts::Fonts::FontConsole,
        "simple" => cfonts::Fonts::FontSimple,
        "simple3d" => cfonts::Fonts::FontSimple3d,
        "chrome" => cfonts::Fonts::FontChrome,
        "simpleblock" => cfonts::Fonts::FontSimpleBlock,
        "3d" => cfonts::Fonts::Font3d,
        _ => return None,
    })
}

fn cfonts_render(text: &str, font: &str) -> Option<cfonts::render::RenderedString> {
    let cfont = to_cfonts_font(font)?;
    let options = cfonts::Options {
        text: text.to_string(),
        font: cfont,
        colors: vec![],
        spaceless: true,
        raw_mode: true,
        ..Default::default()
    };
    Some(cfonts::render(options))
}

pub fn measure_text(text: &str, font_name: &str) -> Option<(usize, usize)> {
    let result = cfonts_render(text, font_name)?;
    let lines = result.vec.len();
    if text.is_empty() {
        return Some((0, lines));
    }
    let width = result.text.lines().map(|l| l.len()).max().unwrap_or(0);
    Some((width, lines))
}

pub fn render_text(text: &str, font_name: &str) -> Option<String> {
    let result = cfonts_render(text, font_name)?;
    Some(result.text)
}

pub fn get_character_positions(text: &str, font_name: &str) -> Option<Vec<usize>> {
    let cfont = to_cfonts_font(font_name)?;
    let mut positions = vec![0usize];
    let mut current = 0usize;

    for ch in text.chars() {
        let options = cfonts::Options {
            text: ch.to_uppercase().to_string(),
            font: cfont.clone(),
            colors: vec![],
            spaceless: true,
            raw_mode: true,
            ..Default::default()
        };
        let result = cfonts::render(options);
        let char_width = result.text.lines().map(|l| l.len()).max().unwrap_or(0);
        current += char_width;
        positions.push(current);
    }

    Some(positions)
}

pub struct AsciiFontSegment {
    pub x: u16,
    pub y: u16,
    pub text: String,
    pub color_index: usize,
}

pub struct AsciiFontLayout {
    pub segments: Vec<AsciiFontSegment>,
    pub width: usize,
    pub height: usize,
}

pub fn layout_text(text: &str, font_name: &str, start_x: u16, start_y: u16) -> Option<AsciiFontLayout> {
    let result = cfonts_render(text, font_name)?;
    let mut segments = Vec::new();
    let mut width = 0usize;

    for (y, line) in result.text.lines().enumerate() {
        let line_len = line.len();
        width = width.max(line_len);
        segments.push(AsciiFontSegment { x: start_x, y: start_y + y as u16, text: line.to_string(), color_index: 0 });
    }

    Some(AsciiFontLayout { segments, width, height: result.vec.len() })
}

pub fn coordinate_to_character_index(x: u16, text: &str, font_name: &str) -> Option<usize> {
    let positions = get_character_positions(text, font_name)?;

    if positions.is_empty() || x < positions[0] as u16 {
        return Some(0);
    }

    for i in 0..positions.len().saturating_sub(1) {
        let current = positions[i] as u16;
        let next = positions[i + 1] as u16;

        if x >= current && x < next {
            let midpoint = current + (next - current) / 2;
            return Some(if x < midpoint { i } else { i + 1 });
        }
    }

    if let Some(&last) = positions.last()
        && x >= last as u16
    {
        return Some(text.len());
    }

    Some(0)
}

pub fn render_font_to_frame_buffer(
    buffer: &mut FrameBuffer,
    text: &str,
    x: u16,
    y: u16,
    colors: &[Color],
    background_color: Color,
    font_name: &str,
) -> Option<(usize, usize)> {
    let result = cfonts_render(text, font_name)?;
    let buf_width = buffer.width();
    let buf_height = buffer.height();
    let fg = colors.first().copied().unwrap_or(Color::Default);

    let mut max_line_width = 0usize;

    for (line_idx, line) in result.text.lines().enumerate() {
        let render_y = y + line_idx as u16;
        if render_y >= buf_height {
            break;
        }
        max_line_width = max_line_width.max(line.len());

        for (char_idx, ch) in line.chars().enumerate() {
            let render_x = x + char_idx as u16;
            if render_x >= buf_width {
                break;
            }
            if ch != ' ' {
                buffer.set(render_x, render_y, Cell::new(ch).with_fg(fg).with_bg(background_color));
            }
        }
    }

    Some((max_line_width, result.vec.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_all_fonts() {
        for name in FONT_NAMES {
            let result = measure_text("ABC", name);
            assert!(result.is_some(), "Font '{}' failed to load", name);
            let (w, h) = result.unwrap();
            assert!(w > 0);
            assert!(h > 0);
        }
    }

    #[test]
    fn measure_tiny() {
        let (w, h) = measure_text("HI", "tiny").unwrap();
        assert_eq!(h, 2);
        assert!(w > 0);
    }

    #[test]
    fn measure_block() {
        let (w, h) = measure_text("A", "block").unwrap();
        assert_eq!(h, 6);
        assert!(w > 0);
    }

    #[test]
    fn measure_empty_string() {
        let (w, h) = measure_text("", "tiny").unwrap();
        assert_eq!(w, 0);
        assert_eq!(h, 2);
    }

    #[test]
    fn measure_unknown_font() {
        let result = measure_text("test", "nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn get_positions() {
        let positions = get_character_positions("AB", "tiny").unwrap();
        assert_eq!(positions.len(), 3);
        assert_eq!(positions[0], 0);
        assert!(positions[1] > 0);
    }

    #[test]
    fn get_positions_unknown_font() {
        let result = get_character_positions("test", "nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn layout_unknown_font() {
        let result = layout_text("test", "nonexistent", 0, 0);
        assert!(result.is_none());
    }

    #[test]
    fn layout_tiny_hi() {
        let layout = layout_text("HI", "tiny", 0, 0).unwrap();
        assert!(layout.width > 0);
        assert_eq!(layout.height, 2);
        assert!(!layout.segments.is_empty());
    }

    #[test]
    fn layout_with_offset() {
        let layout = layout_text("A", "tiny", 10, 5).unwrap();
        for seg in &layout.segments {
            assert!(seg.x >= 10);
            assert!(seg.y >= 5);
        }
    }

    #[test]
    fn layout_huge() {
        let layout = layout_text("A", "huge", 0, 0).unwrap();
        assert_eq!(layout.height, 11);
        assert!(layout.segments.len() > 5);
    }

    #[test]
    fn measure_all_font_names() {
        for name in &["tiny", "block", "shade", "slick", "huge", "grid", "pallet"] {
            let result = measure_text("Hello", name);
            assert!(result.is_some(), "Failed to measure font: {}", name);
        }
    }

    #[test]
    fn render_text_returns_string() {
        let output = render_text("HELLO", "tiny").unwrap();
        assert!(!output.is_empty());
        assert!(output.lines().count() >= 2);
    }

    #[test]
    fn render_text_unknown_font() {
        let result = render_text("test", "nonexistent");
        assert!(result.is_none());
    }
}
