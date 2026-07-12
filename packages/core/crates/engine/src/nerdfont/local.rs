use super::font::NerdFontVariant;
use std::path::{Path, PathBuf};

const BUNDLED_FONT_DATA: &[u8] = include_bytes!("../../fonts/DroidSansMNerdFont-Regular.otf");

#[derive(Debug, Clone)]
pub struct LocalFont {
    pub path: PathBuf,
    pub name: String,
    pub family: String,
    pub variant: NerdFontVariant,
    pub is_bundled: bool,
}

impl LocalFont {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        Self {
            path,
            name: name.clone(),
            family: name,
            variant: NerdFontVariant::Complete,
            is_bundled: false,
        }
    }

    pub fn bundled() -> Self {
        Self {
            path: PathBuf::from("bundled://DroidSansMNerdFont-Regular.otf"),
            name: "DroidSansMNerdFont".to_string(),
            family: "Droid Sans Mono".to_string(),
            variant: NerdFontVariant::Complete,
            is_bundled: true,
        }
    }

    pub fn load_bytes(&self) -> std::io::Result<Vec<u8>> {
        if self.is_bundled {
            Ok(BUNDLED_FONT_DATA.to_vec())
        } else {
            std::fs::read(&self.path)
        }
    }

    pub fn exists(&self) -> bool {
        if self.is_bundled {
            true
        } else {
            self.path.exists()
        }
    }
}

pub struct LocalFontDetector {
    bundled_font: LocalFont,
    system_fonts: Vec<LocalFont>,
    search_paths: Vec<PathBuf>,
}

impl Default for LocalFontDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalFontDetector {
    pub fn new() -> Self {
        let search_paths = Self::default_search_paths();
        Self {
            bundled_font: LocalFont::bundled(),
            system_fonts: Vec::new(),
            search_paths,
        }
    }

    pub fn with_search_paths(paths: Vec<PathBuf>) -> Self {
        Self {
            bundled_font: LocalFont::bundled(),
            system_fonts: Vec::new(),
            search_paths: paths,
        }
    }

    fn default_search_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();

        if let Some(home) = dirs_or_home() {
            paths.push(home.join(".local/share/fonts"));
            paths.push(home.join(".fonts"));
            paths.push(home.join("Library/Fonts"));
        }

        paths.push(PathBuf::from("/usr/local/share/fonts"));
        paths.push(PathBuf::from("/usr/share/fonts"));
        paths.push(PathBuf::from("/System/Library/Fonts"));

        paths
    }

    pub fn detect(&mut self) -> Vec<LocalFont> {
        self.system_fonts.clear();

        let paths = self.search_paths.clone();
        for path in &paths {
            if path.exists() {
                self.scan_directory(path);
            }
        }

        self.system_fonts.clone()
    }

    fn scan_directory(&mut self, dir: &Path) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file()
                    && let Some(ext) = path.extension()
                {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if matches!(ext_str.as_str(), "otf" | "ttf" | "woff" | "woff2") {
                        let name = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_string();

                        if name.contains("NerdFont") || name.contains("nerd-font") {
                            let font = LocalFont::new(path);
                            self.system_fonts.push(font);
                        }
                    }
                }
            }
        }
    }

    pub fn has_bundled_font(&self) -> bool {
        true
    }

    pub fn bundled_font(&self) -> &LocalFont {
        &self.bundled_font
    }

    pub fn system_fonts(&self) -> &[LocalFont] {
        &self.system_fonts
    }

    pub fn find_font(&self, name: &str) -> Option<&LocalFont> {
        if self.bundled_font.name.contains(name) {
            return Some(&self.bundled_font);
        }
        self.system_fonts.iter().find(|f| f.name.contains(name))
    }

    pub fn any_font_available(&self) -> bool {
        self.bundled_font.exists() || !self.system_fonts.is_empty()
    }

    pub fn best_font(&self) -> &LocalFont {
        if self.bundled_font.exists() {
            &self.bundled_font
        } else if let Some(first) = self.system_fonts.first() {
            first
        } else {
            &self.bundled_font
        }
    }
}

fn dirs_or_home() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| std::env::var("USERPROFILE").ok().map(PathBuf::from))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_font_exists() {
        let font = LocalFont::bundled();
        assert!(font.exists());
        assert!(font.is_bundled);
    }

    #[test]
    fn bundled_font_load() {
        let font = LocalFont::bundled();
        let data = font.load_bytes().unwrap();
        assert!(data.len() > 1000);
        assert_eq!(&data[0..4], b"OTTO");
    }

    #[test]
    fn local_font_new() {
        let font = LocalFont::new(PathBuf::from("/tmp/TestFont.otf"));
        assert_eq!(font.name, "TestFont");
        assert!(!font.is_bundled);
    }

    #[test]
    fn detector_new() {
        let detector = LocalFontDetector::new();
        assert!(detector.has_bundled_font());
        assert!(detector.any_font_available());
    }

    #[test]
    fn detector_best_font() {
        let detector = LocalFontDetector::new();
        let best = detector.best_font();
        assert!(best.exists());
    }

    #[test]
    fn detector_find_font() {
        let detector = LocalFontDetector::new();
        assert!(detector.find_font("DroidSans").is_some());
        assert!(detector.find_font("NonExistent").is_none());
    }
}
