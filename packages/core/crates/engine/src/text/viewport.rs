use super::unicode;
use super::wrap::{WrapMode, WrappedLine, wrap_text};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewportLine {
    pub text: String,
    pub x: u16,
    pub y: u16,
    pub width: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextViewport {
    pub lines: Vec<ViewportLine>,
    pub total_height: u16,
    pub max_line_width: u16,
}

#[derive(Debug, Clone)]
pub struct ViewportConfig {
    pub align: TextAlign,
    pub wrap: bool,
    pub max_width: u16,
    pub max_height: u16,
    pub pad_left: u16,
    pub pad_top: u16,
    pub ellipsis: bool,
}

impl Default for ViewportConfig {
    fn default() -> Self {
        Self {
            align: TextAlign::Left,
            wrap: false,
            max_width: 80,
            max_height: u16::MAX,
            pad_left: 0,
            pad_top: 0,
            ellipsis: false,
        }
    }
}

pub fn layout_text(text: &str, config: &ViewportConfig) -> TextViewport {
    let wrapped = if config.wrap {
        wrap_text(text, config.max_width, WrapMode::WordOrChar)
    } else if text.contains('\n') {
        let mut lines = Vec::new();
        for hard_line in text.split('\n') {
            lines.push(WrappedLine {
                byte_offset: 0,
                byte_len: hard_line.len(),
                visual_width: unicode::display_width(hard_line) as u16,
            });
        }
        lines
    } else {
        vec![WrappedLine {
            byte_offset: 0,
            byte_len: text.len(),
            visual_width: unicode::display_width(text) as u16,
        }]
    };

    let mut lines = Vec::new();
    let mut total_height: u16 = 0;
    let mut max_line_width: u16 = 0;

    let max_y = config.max_height.saturating_sub(1);

    let any_content = wrapped.iter().any(|wl| wl.byte_len > 0 || !text.is_empty());

    for wl in &wrapped {
        if !any_content && wl.byte_len == 0 {
            continue;
        }
        if total_height > max_y {
            break;
        }
        let fragment = &text[wl.byte_offset..][..wl.byte_len];
        let line_width = if config.ellipsis && wl.visual_width > config.max_width {
            config.max_width
        } else {
            wl.visual_width.min(config.max_width)
        };
        let line_text = if config.ellipsis && wl.visual_width > config.max_width {
            unicode::truncate_with_ellipsis(fragment, config.max_width as usize)
        } else {
            fragment.to_string()
        };
        let x_offset = match config.align {
            TextAlign::Left => config.pad_left,
            TextAlign::Center => {
                let available = config.max_width.saturating_sub(line_width);
                config.pad_left + available / 2
            }
            TextAlign::Right => {
                let available = config.max_width.saturating_sub(line_width);
                config.pad_left + available
            }
            TextAlign::Justify => config.pad_left,
        };
        let display_w = if config.ellipsis && wl.visual_width > config.max_width {
            config.max_width
        } else {
            unicode::display_width(&line_text) as u16
        };
        lines.push(ViewportLine {
            text: line_text,
            x: x_offset,
            y: config.pad_top + total_height,
            width: display_w,
        });
        max_line_width = max_line_width.max(x_offset + display_w);
        total_height += 1;
    }

    TextViewport {
        lines,
        total_height,
        max_line_width,
    }
}

#[allow(dead_code)]
pub fn layout_styled(spans: &[(String, u16)], config: &ViewportConfig) -> TextViewport {
    let capacity = spans.iter().map(|(t, _)| t.len()).sum();
    let mut combined = String::with_capacity(capacity);
    for (text, _) in spans {
        combined.push_str(text);
    }
    layout_text(&combined, config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_left_aligned() {
        let config = ViewportConfig {
            align: TextAlign::Left,
            max_width: 80,
            ..ViewportConfig::default()
        };
        let layout = layout_text("hello world", &config);
        assert_eq!(layout.lines.len(), 1);
        assert_eq!(layout.lines[0].x, 0);
        assert_eq!(layout.lines[0].text, "hello world");
    }

    #[test]
    fn layout_center_aligned() {
        let config = ViewportConfig {
            align: TextAlign::Center,
            max_width: 20,
            ..ViewportConfig::default()
        };
        let layout = layout_text("hello", &config);
        assert_eq!(layout.lines[0].x, 7);
    }

    #[test]
    fn layout_right_aligned() {
        let config = ViewportConfig {
            align: TextAlign::Right,
            max_width: 20,
            ..ViewportConfig::default()
        };
        let layout = layout_text("hello", &config);
        assert_eq!(layout.lines[0].x, 15);
    }

    #[test]
    fn layout_with_wrap() {
        let config = ViewportConfig {
            wrap: true,
            max_width: 10,
            ..ViewportConfig::default()
        };
        let layout = layout_text("hello world foo bar", &config);
        assert!(layout.lines.len() > 1);
    }

    #[test]
    fn layout_with_ellipsis() {
        let config = ViewportConfig {
            ellipsis: true,
            max_width: 5,
            ..ViewportConfig::default()
        };
        let layout = layout_text("hello world", &config);
        assert_eq!(layout.lines.len(), 1);
        assert!(layout.lines[0].text.contains('\u{2026}'));
    }

    #[test]
    fn layout_max_height() {
        let config = ViewportConfig {
            wrap: true,
            max_width: 5,
            max_height: 2,
            ..ViewportConfig::default()
        };
        let layout = layout_text("abcdefghijklmnop", &config);
        assert!(layout.lines.len() <= 2);
    }

    #[test]
    fn layout_padding() {
        let config = ViewportConfig {
            pad_left: 2,
            pad_top: 1,
            max_width: 80,
            ..ViewportConfig::default()
        };
        let layout = layout_text("hello\nworld", &config);
        assert_eq!(layout.lines[0].x, 2);
        assert_eq!(layout.lines[0].y, 1);
        assert_eq!(layout.lines[1].y, 2);
    }

    #[test]
    fn layout_newlines_no_wrap() {
        let config = ViewportConfig {
            max_width: 80,
            ..ViewportConfig::default()
        };
        let layout = layout_text("hello\nworld", &config);
        assert_eq!(layout.lines.len(), 2);
    }

    #[test]
    fn layout_empty() {
        let config = ViewportConfig::default();
        let layout = layout_text("", &config);
        assert_eq!(layout.lines.len(), 0, "got {layout:#?}");
    }

    #[test]
    fn layout_max_width_respected() {
        let config = ViewportConfig {
            wrap: true,
            max_width: 8,
            ..ViewportConfig::default()
        };
        let layout = layout_text("hello world foo bar baz qux", &config);
        for line in &layout.lines {
            assert!(unicode::display_width(&line.text) <= 8);
        }
    }

    #[test]
    fn layout_styled_combines_text() {
        let spans = vec![("hello ".to_string(), 6), ("world".to_string(), 5)];
        let config = ViewportConfig::default();
        let layout = layout_styled(&spans, &config);
        assert_eq!(layout.lines.len(), 1);
        assert_eq!(layout.lines[0].text, "hello world");
    }
}
