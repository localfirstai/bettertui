use super::font::{GlyphCategory, NerdFont, NerdFontGlyph, NerdFontVariant};
use super::local::{LocalFont, LocalFontDetector};
use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detector_new() {
        let detector = NerdFontDetector::new();
        assert!(detector.available_fonts().is_empty());
        assert!(detector.local_detector.has_bundled_font());
    }

    #[test]
    fn detector_create_font() {
        let detector = NerdFontDetector::new();
        let font = detector.create_font("TestFont");
        assert_eq!(font.name, "TestFont");
        assert!(font.glyph_count() > 0);
    }

    #[test]
    fn detector_has_font() {
        let mut detector = NerdFontDetector::new();
        let font = detector.create_font("TestFont");
        detector.fonts.insert("TestFont".to_string(), font);

        assert!(detector.has_font("TestFont"));
        assert!(!detector.has_font("OtherFont"));
    }

    #[test]
    fn detector_has_local_font() {
        let detector = NerdFontDetector::new();
        assert!(detector.has_local_font("DroidSans"));
        assert!(!detector.has_local_font("NonExistentFont"));
    }

    #[test]
    fn detector_font_count() {
        let mut detector = NerdFontDetector::new();
        let font = detector.create_font("TestFont");
        detector.fonts.insert("TestFont".to_string(), font);

        assert_eq!(detector.font_count(), 1);
    }
}
