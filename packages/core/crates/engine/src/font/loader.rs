const BUNDLED_FONT_DATA: &[u8] = include_bytes!("../../fonts/DroidSansMNerdFont-Regular.otf");

const BUNDLED_FONT_NAME: &str = "DroidSansMNerdFont";
const BUNDLED_FONT_FAMILY: &str = "Droid Sans Mono";

#[derive(Debug, Clone)]
pub enum NerdFontVariant {
    Complete,
    Mono,
    Propo,
    SeparatedMono,
    SeparatedPropo,
}

impl Default for NerdFontVariant {
    fn default() -> Self {
        Self::Complete
    }
}

#[derive(Debug, Clone)]
pub struct FontMetadata {
    pub name: String,
    pub family: String,
    pub variant: NerdFontVariant,
    pub is_monospace: bool,
}

impl Default for FontMetadata {
    fn default() -> Self {
        Self {
            name: BUNDLED_FONT_NAME.to_string(),
            family: BUNDLED_FONT_FAMILY.to_string(),
            variant: NerdFontVariant::Complete,
            is_monospace: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BundledFont {
    metadata: FontMetadata,
}

impl Default for BundledFont {
    fn default() -> Self {
        Self::new()
    }
}

impl BundledFont {
    pub fn new() -> Self {
        Self {
            metadata: FontMetadata::default(),
        }
    }

    pub fn bytes(&self) -> &'static [u8] {
        BUNDLED_FONT_DATA
    }

    pub fn metadata(&self) -> &FontMetadata {
        &self.metadata
    }

    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    pub fn family(&self) -> &str {
        &self.metadata.family
    }

    pub fn size(&self) -> usize {
        BUNDLED_FONT_DATA.len()
    }

    pub fn exists(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_font_exists() {
        assert!(BundledFont::new().exists());
    }

    #[test]
    fn bundled_font_bytes() {
        let font = BundledFont::new();
        let data = font.bytes();
        assert!(data.len() > 1000);
        assert_eq!(&data[..4], b"OTTO");
    }

    #[test]
    fn bundled_font_metadata() {
        let font = BundledFont::new();
        let meta = font.metadata();
        assert!(meta.name.contains("NerdFont"));
        assert!(meta.is_monospace);
    }

    #[test]
    fn bundled_font_size() {
        let font = BundledFont::new();
        assert!(font.size() > 100_000);
    }
}
