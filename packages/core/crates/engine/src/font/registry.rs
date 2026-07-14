use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconCategory {
    Cod,
    Custom,
    Dev,
    Extra,
    Fa,
    Fae,
    Iec,
    Indent,
    Linux,
    Md,
    Oct,
    Pl,
    Ple,
    Pom,
    Seti,
    Weather,
    Unknown,
}

impl IconCategory {
    pub fn from_prefix(prefix: &str) -> Self {
        match prefix {
            "cod" => Self::Cod,
            "custom" => Self::Custom,
            "dev" => Self::Dev,
            "extra" => Self::Extra,
            "fa" => Self::Fa,
            "fae" => Self::Fae,
            "iec" => Self::Iec,
            "indent" | "indentation" => Self::Indent,
            "linux" => Self::Linux,
            "md" => Self::Md,
            "oct" => Self::Oct,
            "pl" => Self::Pl,
            "ple" => Self::Ple,
            "pom" | "pomicons" => Self::Pom,
            "seti" | "seti-ui" => Self::Seti,
            "weather" => Self::Weather,
            _ => Self::Unknown,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Cod => "cod",
            Self::Custom => "custom",
            Self::Dev => "dev",
            Self::Extra => "extra",
            Self::Fa => "fa",
            Self::Fae => "fae",
            Self::Iec => "iec",
            Self::Indent => "indent",
            Self::Linux => "linux",
            Self::Md => "md",
            Self::Oct => "oct",
            Self::Pl => "pl",
            Self::Ple => "ple",
            Self::Pom => "pom",
            Self::Seti => "seti",
            Self::Weather => "weather",
            Self::Unknown => "unknown",
        }
    }

    pub fn all() -> &'static [IconCategory] {
        &[
            Self::Cod,
            Self::Custom,
            Self::Dev,
            Self::Extra,
            Self::Fa,
            Self::Fae,
            Self::Iec,
            Self::Indent,
            Self::Linux,
            Self::Md,
            Self::Oct,
            Self::Pl,
            Self::Ple,
            Self::Pom,
            Self::Seti,
            Self::Weather,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct IconGlyph {
    pub codepoint: u32,
    pub name: String,
    pub category: IconCategory,
    pub width: u8,
}

impl IconGlyph {
    pub fn new(codepoint: u32, name: &str, category: IconCategory) -> Self {
        Self {
            codepoint,
            name: name.to_string(),
            category,
            width: 1,
        }
    }

    pub fn to_char(&self) -> Option<char> {
        char::from_u32(self.codepoint)
    }

    pub fn is_powerline(&self) -> bool {
        matches!(self.category, IconCategory::Pl | IconCategory::Ple)
    }
}

pub struct IconRegistry {
    by_codepoint: HashMap<u32, IconGlyph>,
    by_name: HashMap<String, u32>,
    by_category: HashMap<IconCategory, Vec<u32>>,
}

impl Default for IconRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl IconRegistry {
    pub fn new() -> Self {
        Self {
            by_codepoint: HashMap::new(),
            by_name: HashMap::new(),
            by_category: HashMap::new(),
        }
    }

    pub fn with_builtin() -> Self {
        let mut reg = Self::new();
        reg.load_builtin();
        reg
    }

    fn load_builtin(&mut self) {
        let data = super::builtin::BUILTIN_ICONS;
        for entry in data {
            let category = IconCategory::from_prefix(entry.category);
            let glyph = IconGlyph {
                codepoint: entry.codepoint,
                name: entry.name.to_string(),
                category,
                width: 1,
            };

            self.by_codepoint.insert(entry.codepoint, glyph);

            self.by_name.insert(entry.name.to_string(), entry.codepoint);
            self.by_name
                .insert(format!("nf-{}", entry.name), entry.codepoint);

            self.by_category
                .entry(category)
                .or_default()
                .push(entry.codepoint);
        }
    }

    pub fn lookup_codepoint(&self, codepoint: u32) -> Option<&IconGlyph> {
        self.by_codepoint.get(&codepoint)
    }

    pub fn lookup_name(&self, name: &str) -> Option<&IconGlyph> {
        let codepoint = self.by_name.get(name).copied()?;
        self.by_codepoint.get(&codepoint)
    }

    pub fn resolve_char(&self, name: &str) -> Option<char> {
        self.lookup_name(name).and_then(|g| g.to_char())
    }

    pub fn name_for_codepoint(&self, codepoint: u32) -> Option<&str> {
        self.by_codepoint.get(&codepoint).map(|g| g.name.as_str())
    }

    pub fn codepoint_for_name(&self, name: &str) -> Option<u32> {
        self.by_name.get(name).copied()
    }

    pub fn icons_by_category(&self, category: IconCategory) -> Vec<&IconGlyph> {
        self.by_category
            .get(&category)
            .map(|cps| {
                cps.iter()
                    .filter_map(|cp| self.by_codepoint.get(cp))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn categories(&self) -> Vec<IconCategory> {
        let mut cats: Vec<IconCategory> = self.by_category.keys().copied().collect();
        cats.sort_by_key(|c| c.name());
        cats
    }

    pub fn total_count(&self) -> usize {
        self.by_codepoint.len()
    }

    pub fn contains_codepoint(&self, codepoint: u32) -> bool {
        self.by_codepoint.contains_key(&codepoint)
    }

    pub fn contains_name(&self, name: &str) -> bool {
        self.by_name.contains_key(name)
    }

    pub fn codepoints(&self) -> Vec<u32> {
        let mut cps: Vec<u32> = self.by_codepoint.keys().copied().collect();
        cps.sort_unstable();
        cps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_new_empty() {
        let reg = IconRegistry::new();
        assert_eq!(reg.total_count(), 0);
    }

    #[test]
    fn registry_with_builtin() {
        let reg = IconRegistry::with_builtin();
        assert!(reg.total_count() > 1000);
    }

    #[test]
    fn registry_lookup_codepoint() {
        let reg = IconRegistry::with_builtin();
        assert!(reg.lookup_codepoint(0xE0A0).is_some());
        assert!(reg.lookup_codepoint(0xE0A1).is_some());
    }

    #[test]
    fn registry_lookup_name() {
        let reg = IconRegistry::with_builtin();
        let glyph = reg.lookup_name("dev-rust");
        assert!(glyph.is_some());
        assert_eq!(glyph.unwrap().codepoint, 0xE7A8);
    }

    #[test]
    fn registry_lookup_name_with_aliases() {
        let reg = IconRegistry::with_builtin();
        let glyph = reg.lookup_name("dev-aarch64");
        assert!(glyph.is_some());
        assert_eq!(glyph.unwrap().codepoint, 0xE700);
    }

    #[test]
    fn registry_lookup_nf_prefixed() {
        let reg = IconRegistry::with_builtin();
        let glyph = reg.lookup_name("nf-dev-rust");
        assert!(glyph.is_some());
        assert_eq!(glyph.unwrap().codepoint, 0xE7A8);
    }

    #[test]
    fn registry_codepoint_for_name() {
        let reg = IconRegistry::with_builtin();
        assert_eq!(reg.codepoint_for_name("dev-rust"), Some(0xE7A8));
        assert_eq!(reg.codepoint_for_name("pl-branch"), Some(0xE0A0));
    }

    #[test]
    fn registry_name_for_codepoint() {
        let reg = IconRegistry::with_builtin();
        let name = reg.name_for_codepoint(0xE0A0);
        assert_eq!(name, Some("pl-branch"));
    }

    #[test]
    fn registry_resolve_char() {
        let reg = IconRegistry::with_builtin();
        let ch = reg.resolve_char("dev-rust");
        assert!(ch.is_some());
        assert_eq!(ch.unwrap() as u32, 0xE7A8);
    }

    #[test]
    fn registry_total_count_is_unique_codepoints() {
        let reg = IconRegistry::with_builtin();
        assert!(reg.total_count() >= 10000);
        assert!(reg.total_count() <= 10764);
    }

    #[test]
    fn registry_all_names_in_by_name() {
        let reg = IconRegistry::with_builtin();
        assert!(reg.contains_name("fa-glass"));
        assert!(reg.contains_name("fa-martini_glass_empty"));
        assert_eq!(
            reg.codepoint_for_name("fa-glass"),
            reg.codepoint_for_name("fa-martini_glass_empty")
        );
    }
}
