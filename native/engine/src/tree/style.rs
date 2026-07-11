use super::color::Color;

/// Border style for a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderStyle {
    #[default]
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
}

/// Overflow handling for a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
    Scroll,
}

/// Visual styling for a node. Applied during rendering.
///
/// Uses `Option<bool>` instead of bitflags to allow style inheritance.
/// A child node can inherit its parent's `bold` value by having
/// `bold: None`. `Some(true)` or `Some(false)` overrides the parent.
///
/// Size: ~32 bytes. Stack-allocated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub underline_color: Option<Color>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub dim: Option<bool>,
    pub strikethrough: Option<bool>,
    pub inverse: Option<bool>,
    pub hidden: Option<bool>,
    pub grid_columns: Option<u16>,
    pub grid_rows: Option<u16>,
    pub border_style: Option<BorderStyle>,
    pub border_color: Option<Color>,
    pub border_width: Option<u16>,
    pub rounded_corners: Option<bool>,
    pub overflow: Option<Overflow>,
    pub opacity: Option<u8>,
}

impl Style {
    /// Returns a new style with all fields set to None (fully inheritable).
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if this style has no explicit values (all None).
    pub fn is_empty(&self) -> bool {
        self.fg.is_none()
            && self.bg.is_none()
            && self.underline_color.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.underline.is_none()
            && self.dim.is_none()
            && self.strikethrough.is_none()
            && self.inverse.is_none()
            && self.hidden.is_none()
            && self.grid_columns.is_none()
            && self.grid_rows.is_none()
            && self.border_style.is_none()
            && self.border_color.is_none()
            && self.border_width.is_none()
            && self.rounded_corners.is_none()
            && self.overflow.is_none()
            && self.opacity.is_none()
    }

    /// Merges this style with a parent style. Self values take precedence.
    pub fn resolve(&self, parent: &Style) -> ResolvedStyle {
        ResolvedStyle {
            fg: self.fg.or(parent.fg),
            bg: self.bg.or(parent.bg),
            underline_color: self.underline_color.or(parent.underline_color),
            bold: self.bold.or(parent.bold).unwrap_or(false),
            italic: self.italic.or(parent.italic).unwrap_or(false),
            underline: self.underline.or(parent.underline).unwrap_or(false),
            dim: self.dim.or(parent.dim).unwrap_or(false),
            strikethrough: self.strikethrough.or(parent.strikethrough).unwrap_or(false),
            inverse: self.inverse.or(parent.inverse).unwrap_or(false),
            hidden: self.hidden.or(parent.hidden).unwrap_or(false),
            border_style: self
                .border_style
                .or(parent.border_style)
                .unwrap_or(BorderStyle::None),
            border_color: self.border_color.or(parent.border_color),
            border_width: self.border_width.or(parent.border_width).unwrap_or(0),
            rounded_corners: self
                .rounded_corners
                .or(parent.rounded_corners)
                .unwrap_or(false),
            overflow: self
                .overflow
                .or(parent.overflow)
                .unwrap_or(Overflow::Visible),
            opacity: self.opacity.or(parent.opacity).unwrap_or(255),
        }
    }

    /// Set foreground color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set bold text.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    /// Set italic text.
    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    /// Set underline text.
    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = Some(underline);
        self
    }

    /// Set border style and color.
    pub fn border(mut self, style: BorderStyle, color: Color) -> Self {
        self.border_style = Some(style);
        self.border_color = Some(color);
        self.border_width = Some(1);
        self
    }

    /// Set border width.
    pub fn border_width(mut self, width: u16) -> Self {
        self.border_width = Some(width);
        self
    }

    /// Set rounded corners.
    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded_corners = Some(rounded);
        self
    }

    /// Set overflow handling.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    /// Set opacity (0-255).
    pub fn opacity(mut self, opacity: u8) -> Self {
        self.opacity = Some(opacity);
        self
    }
}

/// Fully resolved style with no Option fields. Used during rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedStyle {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub underline_color: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
    pub strikethrough: bool,
    pub inverse: bool,
    pub hidden: bool,
    pub border_style: BorderStyle,
    pub border_color: Option<Color>,
    pub border_width: u16,
    pub rounded_corners: bool,
    pub overflow: Overflow,
    pub opacity: u8,
}

impl Default for ResolvedStyle {
    fn default() -> Self {
        Self {
            fg: None,
            bg: None,
            underline_color: None,
            bold: false,
            italic: false,
            underline: false,
            dim: false,
            strikethrough: false,
            inverse: false,
            hidden: false,
            border_style: BorderStyle::None,
            border_color: None,
            border_width: 0,
            rounded_corners: false,
            overflow: Overflow::Visible,
            opacity: 255,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::color::NamedColor;

    #[test]
    fn default_style_is_empty() {
        assert!(Style::default().is_empty());
    }

    #[test]
    fn style_resolve_inherits_parent() {
        let parent = Style {
            bold: Some(true),
            fg: Some(Color::Named(NamedColor::Red)),
            ..Default::default()
        };
        let child = Style {
            italic: Some(true),
            ..Default::default()
        };

        let resolved = child.resolve(&parent);
        assert!(resolved.bold);
        assert!(resolved.italic);
        assert_eq!(resolved.fg, Some(Color::Named(NamedColor::Red)));
    }

    #[test]
    fn style_resolve_child_overrides_parent() {
        let parent = Style {
            bold: Some(true),
            fg: Some(Color::Named(NamedColor::Red)),
            ..Default::default()
        };
        let child = Style {
            bold: Some(false),
            fg: Some(Color::Named(NamedColor::Blue)),
            ..Default::default()
        };

        let resolved = child.resolve(&parent);
        assert!(!resolved.bold);
        assert_eq!(resolved.fg, Some(Color::Named(NamedColor::Blue)));
    }

    #[test]
    fn resolved_style_defaults() {
        let resolved = ResolvedStyle::default();
        assert!(!resolved.bold);
        assert!(!resolved.italic);
        assert!(!resolved.underline);
        assert!(!resolved.dim);
        assert!(!resolved.strikethrough);
        assert!(!resolved.inverse);
        assert!(!resolved.hidden);
        assert_eq!(resolved.border_style, BorderStyle::None);
        assert_eq!(resolved.border_width, 0);
        assert!(!resolved.rounded_corners);
        assert_eq!(resolved.overflow, Overflow::Visible);
        assert_eq!(resolved.opacity, 255);
    }

    #[test]
    fn style_builder_fg() {
        let style = Style::new().fg(Color::Named(NamedColor::Red));
        assert_eq!(style.fg, Some(Color::Named(NamedColor::Red)));
    }

    #[test]
    fn style_builder_bg() {
        let style = Style::new().bg(Color::Named(NamedColor::Blue));
        assert_eq!(style.bg, Some(Color::Named(NamedColor::Blue)));
    }

    #[test]
    fn style_builder_bold() {
        let style = Style::new().bold(true);
        assert_eq!(style.bold, Some(true));
    }

    #[test]
    fn style_builder_border() {
        let style = Style::new().border(BorderStyle::Solid, Color::Named(NamedColor::White));
        assert_eq!(style.border_style, Some(BorderStyle::Solid));
        assert_eq!(style.border_color, Some(Color::Named(NamedColor::White)));
        assert_eq!(style.border_width, Some(1));
    }

    #[test]
    fn style_builder_rounded() {
        let style = Style::new().rounded(true);
        assert_eq!(style.rounded_corners, Some(true));
    }

    #[test]
    fn style_builder_opacity() {
        let style = Style::new().opacity(128);
        assert_eq!(style.opacity, Some(128));
    }

    #[test]
    fn style_resolve_border() {
        let parent = Style {
            border_style: Some(BorderStyle::Solid),
            border_color: Some(Color::Named(NamedColor::White)),
            border_width: Some(2),
            ..Default::default()
        };
        let child = Style::default();
        let resolved = child.resolve(&parent);
        assert_eq!(resolved.border_style, BorderStyle::Solid);
        assert_eq!(resolved.border_color, Some(Color::Named(NamedColor::White)));
        assert_eq!(resolved.border_width, 2);
    }

    #[test]
    fn style_resolve_opacity() {
        let parent = Style {
            opacity: Some(128),
            ..Default::default()
        };
        let child = Style::default();
        let resolved = child.resolve(&parent);
        assert_eq!(resolved.opacity, 128);
    }

    #[test]
    fn border_style_variants() {
        assert_eq!(BorderStyle::None, BorderStyle::None);
        assert_eq!(BorderStyle::Solid, BorderStyle::Solid);
        assert_eq!(BorderStyle::Dashed, BorderStyle::Dashed);
        assert_eq!(BorderStyle::Dotted, BorderStyle::Dotted);
        assert_eq!(BorderStyle::Double, BorderStyle::Double);
    }

    #[test]
    fn overflow_variants() {
        assert_eq!(Overflow::Visible, Overflow::Visible);
        assert_eq!(Overflow::Hidden, Overflow::Hidden);
        assert_eq!(Overflow::Scroll, Overflow::Scroll);
    }
}
