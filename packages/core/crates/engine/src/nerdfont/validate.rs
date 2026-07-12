use super::font::NerdFont;

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub glyph_count: usize,
    pub valid_glyphs: usize,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub code: ErrorCode,
    pub message: String,
    pub glyph: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub code: WarningCode,
    pub message: String,
    pub glyph: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    MissingGlyphs,
    InvalidCodepoint,
    DuplicateCodepoint,
    InvalidWidth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningCode {
    DeprecatedGlyph,
    UnusualWidth,
    MissingName,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        self.valid && self.errors.is_empty()
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    pub fn coverage_percentage(&self) -> f64 {
        if self.glyph_count == 0 {
            0.0
        } else {
            (self.valid_glyphs as f64 / self.glyph_count as f64) * 100.0
        }
    }
}

impl NerdFont {
    pub fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut seen_codepoints = std::collections::HashSet::new();
        let mut valid_glyphs = 0;

        for glyph in &self.glyphs {
            if seen_codepoints.contains(&glyph.codepoint) {
                errors.push(ValidationError {
                    code: ErrorCode::DuplicateCodepoint,
                    message: format!("Duplicate codepoint: U+{:04X}", glyph.codepoint),
                    glyph: Some(glyph.codepoint),
                });
            }
            seen_codepoints.insert(glyph.codepoint);

            if !is_valid_codepoint(glyph.codepoint) {
                errors.push(ValidationError {
                    code: ErrorCode::InvalidCodepoint,
                    message: format!("Invalid codepoint: U+{:04X}", glyph.codepoint),
                    glyph: Some(glyph.codepoint),
                });
            } else if glyph.width == 0 {
                errors.push(ValidationError {
                    code: ErrorCode::InvalidWidth,
                    message: format!("Zero width glyph: U+{:04X}", glyph.codepoint),
                    glyph: Some(glyph.codepoint),
                });
            } else {
                valid_glyphs += 1;
            }

            if glyph.name.is_empty() {
                warnings.push(ValidationWarning {
                    code: WarningCode::MissingName,
                    message: format!("Missing name for glyph: U+{:04X}", glyph.codepoint),
                    glyph: Some(glyph.codepoint),
                });
            }

            if glyph.width > 2 {
                warnings.push(ValidationWarning {
                    code: WarningCode::UnusualWidth,
                    message: format!(
                        "Unusual width {} for glyph: U+{:04X}",
                        glyph.width, glyph.codepoint
                    ),
                    glyph: Some(glyph.codepoint),
                });
            }
        }

        let glyph_count = self.glyphs.len();

        ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
            glyph_count,
            valid_glyphs,
        }
    }
}

fn is_valid_codepoint(cp: u32) -> bool {
    cp > 0 && cp <= 0x10FFFF && !is_surrogate(cp)
}

fn is_surrogate(cp: u32) -> bool {
    (0xD800..=0xDFFF).contains(&cp)
}

#[cfg(test)]
mod tests {
    use super::super::font::{GlyphCategory, NerdFontGlyph};
    use super::*;

    #[test]
    fn validate_empty_font() {
        let font = NerdFont::new("TestFont");
        let result = font.validate();
        assert!(result.is_valid());
        assert_eq!(result.glyph_count, 0);
    }

    #[test]
    fn validate_valid_glyphs() {
        let glyphs = vec![
            NerdFontGlyph::new(0xE0A0, "branch", GlyphCategory::Powerline),
            NerdFontGlyph::new(0xE0B0, "right-triangle", GlyphCategory::Powerline),
        ];
        let font = NerdFont::new("TestFont").with_glyphs(glyphs);

        let result = font.validate();
        assert!(result.is_valid());
        assert_eq!(result.valid_glyphs, 2);
    }

    #[test]
    fn validate_duplicate_codepoint() {
        let glyphs = vec![
            NerdFontGlyph::new(0xE0A0, "branch1", GlyphCategory::Powerline),
            NerdFontGlyph::new(0xE0A0, "branch2", GlyphCategory::Powerline),
        ];
        let font = NerdFont::new("TestFont").with_glyphs(glyphs);

        let result = font.validate();
        assert!(!result.is_valid());
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.code == ErrorCode::DuplicateCodepoint)
        );
    }

    #[test]
    fn validate_invalid_codepoint() {
        let glyphs = vec![NerdFontGlyph::new(
            0xD800,
            "surrogate",
            GlyphCategory::Powerline,
        )];
        let font = NerdFont::new("TestFont").with_glyphs(glyphs);

        let result = font.validate();
        assert!(!result.is_valid());
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.code == ErrorCode::InvalidCodepoint)
        );
    }

    #[test]
    fn validate_zero_width() {
        let glyphs =
            vec![NerdFontGlyph::new(0xE0A0, "branch", GlyphCategory::Powerline).with_width(0)];
        let font = NerdFont::new("TestFont").with_glyphs(glyphs);

        let result = font.validate();
        assert!(!result.is_valid());
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.code == ErrorCode::InvalidWidth)
        );
    }

    #[test]
    fn validate_missing_name_warning() {
        let glyphs = vec![NerdFontGlyph::new(0xE0A0, "", GlyphCategory::Powerline)];
        let font = NerdFont::new("TestFont").with_glyphs(glyphs);

        let result = font.validate();
        assert!(result.is_valid());
        assert!(
            result
                .warnings
                .iter()
                .any(|w| w.code == WarningCode::MissingName)
        );
    }

    #[test]
    fn validate_coverage_percentage() {
        let glyphs = vec![
            NerdFontGlyph::new(0xE0A0, "branch", GlyphCategory::Powerline),
            NerdFontGlyph::new(0xE0B0, "right-triangle", GlyphCategory::Powerline),
        ];
        let font = NerdFont::new("TestFont").with_glyphs(glyphs);

        let result = font.validate();
        assert_eq!(result.coverage_percentage(), 100.0);
    }
}
