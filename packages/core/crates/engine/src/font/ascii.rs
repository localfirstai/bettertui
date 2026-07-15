use std::collections::HashMap;

pub type AsciiFontName = &'static str;

pub const FONT_NAMES: &[AsciiFontName] = &["tiny", "block", "shade", "slick", "huge", "grid", "pallet"];

#[derive(Debug, Clone)]
struct FontSegment {
    text: String,
    color_index: usize,
}

#[derive(Debug, Clone)]
struct ParsedCharDef {
    lines: Vec<Vec<FontSegment>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ParsedFont {
    name: String,
    lines: usize,
    letterspace_size: usize,
    letterspace: Vec<String>,
    colors: usize,
    chars: HashMap<String, ParsedCharDef>,
}

fn parse_color_tags(text: &str) -> Vec<FontSegment> {
    let mut segments = Vec::new();
    let mut pos = 0;

    while pos < text.len() {
        if text[pos..].starts_with("<c") {
            let after_open = &text[pos + 2..];
            if let Some(end_bracket) = after_open.find('>') {
                let tag_content = &after_open[..end_bracket];
                if let Ok(idx) = tag_content.parse::<usize>() {
                    let after_tag = &after_open[end_bracket + 1..];
                    let close_tag = format!("</c{}>", idx);
                    if let Some(close_pos) = after_tag.find(&close_tag) {
                        let content = &after_tag[..close_pos];
                        if !content.is_empty() {
                            segments
                                .push(FontSegment { text: content.to_string(), color_index: idx.saturating_sub(1) });
                        }
                        pos = pos + 2 + end_bracket + 1 + close_pos + close_tag.len();
                        continue;
                    }
                }
            }
            pos += 2;
            continue;
        }

        let remaining = &text[pos..];
        let next_tag = remaining.find("<c");
        let end = if let Some(tag_start) = next_tag { pos + tag_start } else { text.len() };

        if end > pos {
            segments.push(FontSegment { text: text[pos..end].to_string(), color_index: 0 });
        }
        pos = end;
    }

    segments
}

fn parse_font_json(json_str: &str) -> ParsedFont {
    let value: serde_json::Value = serde_json::from_str(json_str).unwrap();
    let obj = value.as_object().unwrap();

    let name = obj["name"].as_str().unwrap().to_string();
    let lines = obj["lines"].as_u64().unwrap() as usize;
    let letterspace_size = obj["letterspace_size"].as_u64().unwrap_or(0) as usize;
    let colors = obj.get("colors").and_then(|c| c.as_u64()).unwrap_or(1) as usize;

    let letterspace: Vec<String> = obj["letterspace"]
        .as_array()
        .map(|arr| arr.iter().map(|v| v.as_str().unwrap_or(" ").to_string()).collect())
        .unwrap_or_else(|| vec![" ".repeat(lines); lines]);

    let chars_obj = obj["chars"].as_object().unwrap();
    let mut chars = HashMap::new();

    for (ch, lines_arr) in chars_obj {
        let raw_lines: Vec<Vec<FontSegment>> =
            lines_arr.as_array().unwrap().iter().map(|line| parse_color_tags(line.as_str().unwrap_or(""))).collect();

        chars.insert(ch.clone(), ParsedCharDef { lines: raw_lines });
    }

    ParsedFont { name, lines, letterspace_size, letterspace, colors, chars }
}

macro_rules! include_font {
    ($name:expr) => {
        include_str!(concat!("../../fonts/ascii/", $name, ".json"))
    };
}

fn load_font(name: &str) -> Option<ParsedFont> {
    let json = match name {
        "tiny" => include_font!("tiny"),
        "block" => include_font!("block"),
        "shade" => include_font!("shade"),
        "slick" => include_font!("slick"),
        "huge" => include_font!("huge"),
        "grid" => include_font!("grid"),
        "pallet" => include_font!("pallet"),
        _ => return None,
    };
    Some(parse_font_json(json))
}

fn get_char_width(char_def: &ParsedCharDef) -> usize {
    char_def.lines.first().map(|segments| segments.iter().map(|s| s.text.len()).sum()).unwrap_or(0)
}

pub fn measure_text(text: &str, font_name: &str) -> Option<(usize, usize)> {
    let font = load_font(font_name)?;
    let mut current_x = 0;

    for (i, ch) in text.chars().enumerate() {
        let upper = ch.to_uppercase().to_string();
        let char_def = font.chars.get(&upper).or_else(|| font.chars.get(" "));

        if let Some(def) = char_def {
            current_x += get_char_width(def);
        } else {
            current_x += 1;
        }

        if i < text.len() - 1 {
            current_x += font.letterspace_size;
        }
    }

    Some((current_x, font.lines))
}

pub fn get_character_positions(text: &str, font_name: &str) -> Option<Vec<usize>> {
    let font = load_font(font_name)?;
    let mut positions = vec![0usize];
    let mut current_x = 0;

    for (i, ch) in text.chars().enumerate() {
        let upper = ch.to_uppercase().to_string();
        let char_def = font.chars.get(&upper).or_else(|| font.chars.get(" "));

        if let Some(def) = char_def {
            current_x += get_char_width(def);
        } else {
            current_x += 1;
        }

        if i < text.len() - 1 {
            current_x += font.letterspace_size;
        }
        positions.push(current_x);
    }

    Some(positions)
}

pub struct AsciiFontRenderOutput {
    pub width: usize,
    pub height: usize,
}

pub struct AsciiFontSegment {
    pub x: u16,
    pub y: u16,
    pub text: String,
    pub color_index: usize,
}

pub fn layout_text(text: &str, font_name: &str, start_x: u16, start_y: u16) -> Option<AsciiFontLayout> {
    let font = load_font(font_name)?;
    let mut segments = Vec::new();
    let mut current_x = start_x as usize;

    for (i, ch) in text.chars().enumerate() {
        let upper = ch.to_uppercase().to_string();
        let char_def = font.chars.get(&upper).or_else(|| font.chars.get(" "));

        if let Some(def) = char_def {
            for (line_idx, line_segments) in def.lines.iter().enumerate() {
                let mut seg_x = current_x;
                for seg in line_segments {
                    segments.push(AsciiFontSegment {
                        x: seg_x as u16,
                        y: start_y + line_idx as u16,
                        text: seg.text.clone(),
                        color_index: seg.color_index,
                    });
                    seg_x += seg.text.len();
                }
            }
            current_x += get_char_width(def);
        } else {
            current_x += 1;
        }

        if i < text.len() - 1 {
            current_x += font.letterspace_size;
        }
    }

    Some(AsciiFontLayout { segments, width: current_x - start_x as usize, height: font.lines })
}

pub struct AsciiFontLayout {
    pub segments: Vec<AsciiFontSegment>,
    pub width: usize,
    pub height: usize,
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
    fn parse_color_tags_single() {
        let segments = parse_color_tags("hello");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text, "hello");
        assert_eq!(segments[0].color_index, 0);
    }

    #[test]
    fn parse_color_tags_with_color() {
        let segments = parse_color_tags("<c1>red</c1>");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text, "red");
        assert_eq!(segments[0].color_index, 0);
    }

    #[test]
    fn parse_color_tags_mixed() {
        let segments = parse_color_tags("plain<c1>colored</c1>end");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].text, "plain");
        assert_eq!(segments[1].text, "colored");
        assert_eq!(segments[1].color_index, 0);
        assert_eq!(segments[2].text, "end");
    }

    #[test]
    fn parse_color_tags_color2() {
        let segments = parse_color_tags("<c2>secondary</c2>");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text, "secondary");
        assert_eq!(segments[0].color_index, 1);
    }

    #[test]
    fn layout_huge() {
        let layout = layout_text("A", "huge", 0, 0).unwrap();
        assert_eq!(layout.height, 11);
        assert!(layout.segments.len() > 10);
    }

    #[test]
    fn measure_all_font_names() {
        for name in &["tiny", "block", "shade", "slick", "huge", "grid", "pallet"] {
            let result = measure_text("Hello", name);
            assert!(result.is_some(), "Failed to measure font: {}", name);
        }
    }
}
