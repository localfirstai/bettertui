//! Nerd Font integration: detection, glyph lookup, local font bundling, and metrics caching.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ============================================================================
// Font Types
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NerdFont {
    pub name: String,
    pub family: String,
    pub variant: NerdFontVariant,
    pub glyphs: Vec<NerdFontGlyph>,
    pub is_monospace: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NerdFontVariant {
    #[default]
    Complete,
    Mono,
    Propo,
    SeparatedMono,
    SeparatedPropo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NerdFontGlyph {
    pub codepoint: u32,
    pub name: &'static str,
    pub category: GlyphCategory,
    pub width: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlyphCategory {
    Powerline,
    Devicons,
    FontLogos,
    Octicons,
    Material,
    Weather,
    Pomicons,
    Clock,
    Hashes,
    FileType,
    Indicators,
    PowerSymbols,
    Custom,
}

impl NerdFont {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            family: name.to_string(),
            variant: NerdFontVariant::Complete,
            glyphs: Vec::new(),
            is_monospace: true,
        }
    }

    pub fn with_family(mut self, family: &str) -> Self {
        self.family = family.to_string();
        self
    }

    pub fn with_variant(mut self, variant: NerdFontVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_glyphs(mut self, glyphs: Vec<NerdFontGlyph>) -> Self {
        self.glyphs = glyphs;
        self
    }

    pub fn with_monospace(mut self, is_monospace: bool) -> Self {
        self.is_monospace = is_monospace;
        self
    }

    pub fn glyph_count(&self) -> usize {
        self.glyphs.len()
    }

    pub fn has_glyph(&self, codepoint: u32) -> bool {
        self.glyphs.iter().any(|g| g.codepoint == codepoint)
    }

    pub fn get_glyph(&self, codepoint: u32) -> Option<&NerdFontGlyph> {
        self.glyphs.iter().find(|g| g.codepoint == codepoint)
    }

    pub fn glyphs_by_category(&self, category: GlyphCategory) -> Vec<&NerdFontGlyph> {
        self.glyphs
            .iter()
            .filter(|g| g.category == category)
            .collect()
    }

    pub fn categories(&self) -> Vec<GlyphCategory> {
        let mut categories: Vec<GlyphCategory> = self.glyphs.iter().map(|g| g.category).collect();
        categories.sort_by_key(|c| format!("{:?}", c));
        categories.dedup();
        categories
    }

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

impl NerdFontGlyph {
    pub fn new(codepoint: u32, name: &'static str, category: GlyphCategory) -> Self {
        Self {
            codepoint,
            name,
            category,
            width: 1,
        }
    }

    pub fn with_width(mut self, width: u8) -> Self {
        self.width = width;
        self
    }

    pub fn is_wide(&self) -> bool {
        self.width > 1
    }

    pub fn is_powerline(&self) -> bool {
        self.category == GlyphCategory::Powerline
    }

    pub fn is_devicon(&self) -> bool {
        self.category == GlyphCategory::Devicons
    }
}

impl std::fmt::Display for NerdFontVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NerdFontVariant::Complete => write!(f, "Complete"),
            NerdFontVariant::Mono => write!(f, "Mono"),
            NerdFontVariant::Propo => write!(f, "Propo"),
            NerdFontVariant::SeparatedMono => write!(f, "SeparatedMono"),
            NerdFontVariant::SeparatedPropo => write!(f, "SeparatedPropo"),
        }
    }
}

impl std::fmt::Display for GlyphCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlyphCategory::Powerline => write!(f, "Powerline"),
            GlyphCategory::Devicons => write!(f, "Devicons"),
            GlyphCategory::FontLogos => write!(f, "FontLogos"),
            GlyphCategory::Octicons => write!(f, "Octicons"),
            GlyphCategory::Material => write!(f, "Material"),
            GlyphCategory::Weather => write!(f, "Weather"),
            GlyphCategory::Pomicons => write!(f, "Pomicons"),
            GlyphCategory::Clock => write!(f, "Clock"),
            GlyphCategory::Hashes => write!(f, "Hashes"),
            GlyphCategory::FileType => write!(f, "FileType"),
            GlyphCategory::Indicators => write!(f, "Indicators"),
            GlyphCategory::PowerSymbols => write!(f, "PowerSymbols"),
            GlyphCategory::Custom => write!(f, "Custom"),
        }
    }
}

fn is_valid_codepoint(cp: u32) -> bool {
    cp > 0 && cp <= 0x10FFFF && !is_surrogate(cp)
}

fn is_surrogate(cp: u32) -> bool {
    (0xD800..=0xDFFF).contains(&cp)
}

// ============================================================================
// Validation Types
// ============================================================================

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

// ============================================================================
// Local Font Handling
// ============================================================================

const BUNDLED_FONT_DATA: &[u8] = include_bytes!("../fonts/DroidSansMNerdFont-Regular.otf");

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

// ============================================================================
// Glyph Metrics
// ============================================================================

#[derive(Debug, Clone)]
pub struct GlyphMetrics {
    pub width: u16,
    pub height: u16,
    pub bearing_x: i16,
    pub bearing_y: i16,
    pub advance_x: u16,
    pub advance_y: u16,
    pub is_monospace: bool,
}

impl Default for GlyphMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl GlyphMetrics {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            bearing_x: 0,
            bearing_y: 0,
            advance_x: 0,
            advance_y: 0,
            is_monospace: true,
        }
    }

    pub fn with_dimensions(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_bearing(mut self, x: i16, y: i16) -> Self {
        self.bearing_x = x;
        self.bearing_y = y;
        self
    }

    pub fn with_advance(mut self, x: u16, y: u16) -> Self {
        self.advance_x = x;
        self.advance_y = y;
        self
    }

    pub fn with_monospace(mut self, is_monospace: bool) -> Self {
        self.is_monospace = is_monospace;
        self
    }

    pub fn cell_width(&self, cell_width: u16) -> u16 {
        if self.is_monospace {
            cell_width
        } else {
            self.advance_x
        }
    }

    pub fn is_wide(&self) -> bool {
        self.advance_x > 1
    }
}

#[derive(Debug, Clone, Default)]
pub struct MetricsCache {
    metrics: HashMap<u32, GlyphMetrics>,
    cell_width: u16,
    cell_height: u16,
}

impl MetricsCache {
    pub fn new(cell_width: u16, cell_height: u16) -> Self {
        Self {
            metrics: HashMap::new(),
            cell_width,
            cell_height,
        }
    }

    pub fn get(&self, codepoint: u32) -> Option<&GlyphMetrics> {
        self.metrics.get(&codepoint)
    }

    pub fn get_or_create(&mut self, codepoint: u32, glyph: &NerdFontGlyph) -> &GlyphMetrics {
        if !self.metrics.contains_key(&codepoint) {
            let metrics = self.measure_glyph(glyph);
            self.metrics.insert(codepoint, metrics);
        }
        self.metrics.get(&codepoint).unwrap()
    }

    pub fn insert(&mut self, codepoint: u32, metrics: GlyphMetrics) {
        self.metrics.insert(codepoint, metrics);
    }

    pub fn contains(&self, codepoint: u32) -> bool {
        self.metrics.contains_key(&codepoint)
    }

    pub fn len(&self) -> usize {
        self.metrics.len()
    }

    pub fn is_empty(&self) -> bool {
        self.metrics.is_empty()
    }

    pub fn clear(&mut self) {
        self.metrics.clear();
    }

    pub fn cell_width(&self) -> u16 {
        self.cell_width
    }

    pub fn cell_height(&self) -> u16 {
        self.cell_height
    }

    pub fn set_cell_size(&mut self, width: u16, height: u16) {
        self.cell_width = width;
        self.cell_height = height;
    }

    fn measure_glyph(&self, glyph: &NerdFontGlyph) -> GlyphMetrics {
        let width = glyph.width as u16 * self.cell_width;
        let height = self.cell_height;

        GlyphMetrics::new()
            .with_dimensions(width, height)
            .with_advance(glyph.width as u16, 1)
            .with_monospace(true)
    }

    pub fn measure_all(&mut self, font: &NerdFont) {
        for glyph in &font.glyphs {
            let metrics = self.measure_glyph(glyph);
            self.metrics.insert(glyph.codepoint, metrics);
        }
    }

    pub fn total_memory(&self) -> usize {
        self.metrics.len() * std::mem::size_of::<GlyphMetrics>()
    }
}

// ============================================================================
// Font Detection
// ============================================================================

const NERD_FONT_NAMES: &[&str] = &[
    "3270NerdFont",
    "AgaveNerdFont",
    "AnonymiceProNerdFont",
    "ArimoNerdFont",
    "BlexMonoNerdFont",
    "CaskaydiaCoveNerdFont",
    "CodeNewRomanNerdFont",
    "CousineNerdFont",
    "DaddyTimeMonoNerdFont",
    "DejaVuSansMonoNerdFont",
    "DroidSansMonoNerdFont",
    "FiraCodeNerdFont",
    "FiraMonoNerdFont",
    "GeistMonoNerdFont",
    "GoMonoNerdFont",
    "GohuNerdFont",
    "HackNerdFont",
    "HaskligNerdFont",
    "HeavyDataMonoNerdFont",
    "HermitNerdFont",
    "iAWriterMonoNerdFont",
    "iAWriterQuattroNerdFont",
    "IBMPlexMonoNerdFont",
    "InconsolataGoNerdFont",
    "InconsolataLGCNerdFont",
    "JetBrainsMonoNerdFont",
    "LektonNerdFont",
    "LiberationMonoNerdFont",
    "MesloLGDNerdFont",
    "MesloLGLNerdFont",
    "MesloLGMNerdFont",
    "MesloLGSNerdFont",
    "MonaspiceNerdFont",
    "MonaspaceArgonNerdFont",
    "MonaspiceKryptonNerdFont",
    "MonaspiceNeonNerdFont",
    "MonaspiceRadonNerdFont",
    "MonaspiceXenonNerdFont",
    "MonofurNerdFont",
    "MonoidNerdFont",
    "MononokiNerdFont",
    "MPLUS1CodeNerdFont",
    "NotoNerdFont",
    "OpenDyslexicNerdFont",
    "OverpassNerdFont",
    "ProFontNerdFont",
    "ProggyCleanNerdFont",
    "RobotoMonoNerdFont",
    "ShareTechMonoNerdFont",
    "SpaceMonoNerdFont",
    "TerminessNerdFont",
    "TinosNerdFont",
    "UbuntuMonoNerdFont",
    "VictorMonoNerdFont",
    "ZapFleetNerdFont",
    "ZapFinoNerdFont",
];

const POWERLINE_GLYPHS: &[(u32, &str)] = &[
    (0xE0A0, "powerline-branch"),
    (0xE0A1, "powerline-line"),
    (0xE0A2, "powerline-pipe"),
    (0xE0B0, "right-pointing-triangle"),
    (0xE0B1, "right-pointing-triangle-thin"),
    (0xE0B2, "left-pointing-triangle"),
    (0xE0B3, "left-pointing-triangle-thin"),
];

const DEVICON_GLYPHS: &[(u32, &str)] = &[
    (0xE700, "devicon-file-type-js"),
    (0xE701, "devicon-file-type-ts"),
    (0xE702, "devicon-file-type-jsx"),
    (0xE703, "devicon-file-type-tsx"),
    (0xE704, "devicon-file-type-vue"),
    (0xE705, "devicon-file-type-python"),
    (0xE706, "devicon-file-type-rust"),
    (0xE707, "devicon-file-type-go"),
    (0xE708, "devicon-file-type-ruby"),
    (0xE709, "devicon-file-type-java"),
    (0xE70A, "devicon-file-type-c"),
    (0xE70B, "devicon-file-type-cpp"),
    (0xE70C, "devicon-file-type-csharp"),
    (0xE70D, "devicon-file-type-php"),
    (0xE70E, "devicon-file-type-swift"),
    (0xE70F, "devicon-file-type-kotlin"),
    (0xE710, "devicon-file-type-dart"),
    (0xE711, "devicon-file-type-html"),
    (0xE712, "devicon-file-type-css"),
    (0xE713, "devicon-file-type-sass"),
    (0xE714, "devicon-file-type-less"),
    (0xE715, "devicon-file-type-markdown"),
    (0xE716, "devicon-file-type-docker"),
    (0xE717, "devicon-file-type-shell"),
    (0xE718, "devicon-file-type-vim"),
    (0xE719, "devicon-file-type-lua"),
    (0xE71A, "devicon-file-type-perl"),
    (0xE71B, "devicon-file-type-r"),
    (0xE71C, "devicon-file-type-haskell"),
    (0xE71D, "devicon-file-type-elixir"),
    (0xE71E, "devicon-file-type-clojure"),
    (0xE71F, "devicon-file-type-erlang"),
    (0xE720, "devicon-file-type-coffee"),
    (0xE721, "devicon-file-type-elm"),
    (0xE722, "devicon-file-type-purescript"),
    (0xE723, "devicon-file-type-scala"),
    (0xE724, "devicon-file-type-scheme"),
    (0xE725, "devicon-file-type-lisp"),
    (0xE726, "devicon-file-type-julia"),
    (0xE727, "devicon-file-type-matlab"),
    (0xE728, "devicon-file-type-photoshop"),
    (0xE729, "devicon-file-type-illustrator"),
    (0xE72A, "devicon-file-type-sketch"),
    (0xE72B, "devicon-file-type-figma"),
    (0xE72C, "devicon-file-type-blender"),
    (0xE72D, "devicon-file-type-unity"),
    (0xE72E, "devicon-file-type-aws"),
    (0xE72F, "devicon-file-type-azure"),
    (0xE730, "devicon-file-type-gcp"),
    (0xE731, "devicon-file-type-graphql"),
    (0xE732, "devicon-file-type-mongodb"),
    (0xE733, "devicon-file-type-postgresql"),
    (0xE734, "devicon-file-type-mysql"),
    (0xE735, "devicon-file-type-redis"),
    (0xE736, "devicon-file-type-elasticsearch"),
    (0xE737, "devicon-file-type-kubernetes"),
    (0xE738, "devicon-file-type-terraform"),
    (0xE739, "devicon-file-type-ansible"),
    (0xE73A, "devicon-file-type-jenkins"),
    (0xE73B, "devicon-file-type-git"),
    (0xE73C, "devicon-file-type-github"),
    (0xE73D, "devicon-file-type-gitlab"),
    (0xE73E, "devicon-file-type-bitbucket"),
    (0xE73F, "devicon-file-type-linux"),
    (0xE740, "devicon-file-type-apple"),
    (0xE741, "devicon-file-type-windows"),
    (0xE742, "devicon-file-type-android"),
    (0xE743, "devicon-file-type-chrome"),
    (0xE744, "devicon-file-type-firefox"),
    (0xE745, "devicon-file-type-edge"),
    (0xE746, "devicon-file-type-safari"),
    (0xE747, "devicon-file-type-opera"),
    (0xE748, "devicon-file-type-vscode"),
    (0xE749, "devicon-file-type-vim"),
    (0xE74A, "devicon-file-type-emacs"),
    (0xE74B, "devicon-file-type-sublime"),
    (0xE74C, "devicon-file-type-atom"),
    (0xE74D, "devicon-file-type-notepadpp"),
    (0xE74E, "devicon-file-type-intellij"),
    (0xE74F, "devicon-file-type-pycharm"),
    (0xE750, "devicon-file-type-webstorm"),
    (0xE751, "devicon-file-type-clion"),
    (0xE752, "devicon-file-type-rider"),
    (0xE753, "devicon-file-type-datagrip"),
    (0xE754, "devicon-file-type-appcode"),
    (0xE755, "devicon-file-type-phpstorm"),
    (0xE756, "devicon-file-type-rubymine"),
    (0xE757, "devicon-file-type-goland"),
    (0xE758, "devicon-file-type-studio3"),
    (0xE759, "devicon-file-type-androidstudio"),
    (0xE75A, "devicon-file-type-xcode"),
    (0xE75B, "devicon-file-type-visualstudio"),
    (0xE75C, "devicon-file-type-visualstudiocode"),
];

pub struct NerdFontDetector {
    fonts: HashMap<String, NerdFont>,
    available_fonts: Vec<String>,
    local_detector: LocalFontDetector,
    local_fonts: Vec<LocalFont>,
}

impl Default for NerdFontDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl NerdFontDetector {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            available_fonts: Vec::new(),
            local_detector: LocalFontDetector::new(),
            local_fonts: Vec::new(),
        }
    }

    pub fn detect(&mut self) -> Vec<String> {
        self.available_fonts.clear();
        self.local_fonts.clear();

        self.local_detector.detect();
        self.local_fonts = self.local_detector.system_fonts().to_vec();

        for name in NERD_FONT_NAMES {
            if self.is_font_available(name) || self.has_local_font(name) {
                self.available_fonts.push(name.to_string());
                let font = self.create_font(name);
                self.fonts.insert(name.to_string(), font);
            }
        }

        self.available_fonts.clone()
    }

    pub fn has_local_font(&self, name: &str) -> bool {
        self.local_detector.find_font(name).is_some()
    }

    pub fn local_detector(&self) -> &LocalFontDetector {
        &self.local_detector
    }

    pub fn local_fonts(&self) -> &[LocalFont] {
        &self.local_fonts
    }

    pub fn is_font_available(&self, name: &str) -> bool {
        cfg!(target_os = "macos") && self.check_font_macos(name)
            || cfg!(target_os = "linux") && self.check_font_linux(name)
            || cfg!(target_os = "windows") && self.check_font_windows(name)
    }

    fn check_font_macos(&self, name: &str) -> bool {
        std::process::Command::new("fc-list")
            .arg(":family")
            .arg(format!(":family={}", name))
            .output()
            .map(|output| {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.contains(name)
            })
            .unwrap_or(false)
    }

    fn check_font_linux(&self, name: &str) -> bool {
        std::process::Command::new("fc-list")
            .arg(":family")
            .arg(format!(":family={}", name))
            .output()
            .map(|output| {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.contains(name)
            })
            .unwrap_or(false)
    }

    fn check_font_windows(&self, name: &str) -> bool {
        std::process::Command::new("powershell")
            .arg("-Command")
            .arg(format!("Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts' | Select-Object -ExpandProperty '*' | Select-String '{}'", name))
            .output()
            .map(|output| {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.contains(name)
            })
            .unwrap_or(false)
    }

    fn create_font(&self, name: &str) -> NerdFont {
        let mut glyphs = Vec::new();

        for &(codepoint, name_str) in POWERLINE_GLYPHS {
            glyphs.push(NerdFontGlyph::new(
                codepoint,
                name_str,
                GlyphCategory::Powerline,
            ));
        }

        for &(codepoint, name_str) in DEVICON_GLYPHS {
            glyphs.push(NerdFontGlyph::new(
                codepoint,
                name_str,
                GlyphCategory::Devicons,
            ));
        }

        NerdFont::new(name)
            .with_variant(NerdFontVariant::Complete)
            .with_glyphs(glyphs)
    }

    pub fn available_fonts(&self) -> &[String] {
        &self.available_fonts
    }

    pub fn get_font(&self, name: &str) -> Option<&NerdFont> {
        self.fonts.get(name)
    }

    pub fn has_font(&self, name: &str) -> bool {
        self.fonts.contains_key(name)
    }

    pub fn font_count(&self) -> usize {
        self.fonts.len()
    }
}
