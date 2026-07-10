use super::color::Color;

/// Visual styling for a node. Applied during rendering.
///
/// Uses `Option<bool>` instead of bitflags to allow style inheritance.
/// A child node can inherit its parent's `bold` value by having
/// `bold: None`. `Some(true)` or `Some(false)` overrides the parent.
///
/// Size: ~16 bytes. Stack-allocated.
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
        }
    }
}

/// Fully resolved style with no Option fields. Used during rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
    }
}
