use crate::tree::Style;

/// A single highlighted segment of code with its visual style.
#[derive(Debug, Clone)]
pub struct StyledSegment {
    pub text: String,
    pub style: Style,
}

impl StyledSegment {
    pub fn new(text: impl Into<String>, style: Style) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }
}

/// A line of code composed of potentially multiple styled segments.
#[derive(Debug, Clone)]
pub struct HighlightedLine {
    pub segments: Vec<StyledSegment>,
}

impl HighlightedLine {
    pub fn new(segments: Vec<StyledSegment>) -> Self {
        Self { segments }
    }

    pub fn plain(text: impl Into<String>, style: Style) -> Self {
        Self {
            segments: vec![StyledSegment::new(text, style)],
        }
    }

    pub fn text(&self) -> String {
        self.segments.iter().map(|s| s.text.as_str()).collect()
    }
}
