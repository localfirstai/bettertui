//! Module-aware runtime filtering for logs.
//!
//! This module provides filtering based on module paths, allowing fine-grained
//! control over which modules produce logs. Filters can be changed at runtime.

use crate::logger::Level;

/// Module filter for runtime log filtering.
#[derive(Debug, Clone)]
pub struct ModuleFilter {
    /// Modules to include (whitelist). If empty, all modules are included.
    include: Vec<String>,
    /// Modules to exclude (blacklist). Takes precedence over include.
    exclude: Vec<String>,
    /// Default log level for modules not explicitly configured.
    default_level: Level,
}

impl ModuleFilter {
    /// Create a new module filter with the given default level.
    pub fn new(default_level: Level) -> Self {
        Self { include: Vec::new(), exclude: Vec::new(), default_level }
    }

    /// Add a module to the include list.
    pub fn include(mut self, module: impl Into<String>) -> Self {
        self.include.push(module.into());
        self
    }

    /// Add multiple modules to the include list.
    pub fn include_many(mut self, modules: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.include.extend(modules.into_iter().map(Into::into));
        self
    }

    /// Add a module to the exclude list.
    pub fn exclude(mut self, module: impl Into<String>) -> Self {
        self.exclude.push(module.into());
        self
    }

    /// Add multiple modules to the exclude list.
    pub fn exclude_many(mut self, modules: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclude.extend(modules.into_iter().map(Into::into));
        self
    }

    /// Set the default log level.
    pub fn with_default_level(mut self, level: Level) -> Self {
        self.default_level = level;
        self
    }

    /// Check if a module should be logged based on the filter.
    pub fn should_log(&self, module_path: &str) -> bool {
        // Check exclude list first (highest priority)
        for excluded in &self.exclude {
            if module_path.starts_with(excluded) {
                return false;
            }
        }

        // If include list is empty, allow everything not excluded
        if self.include.is_empty() {
            return true;
        }

        // Check include list
        for included in &self.include {
            if module_path.starts_with(included) {
                return true;
            }
        }

        false
    }

    /// Get the default level.
    pub fn default_level(&self) -> Level {
        self.default_level
    }
}

impl Default for ModuleFilter {
    fn default() -> Self {
        Self::new(Level::Info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_filter_allows_all() {
        let filter = ModuleFilter::default();

        assert!(filter.should_log("bettertui_engine::render"));
        assert!(filter.should_log("bettertui_engine::layout"));
        assert!(filter.should_log("some_other_crate"));
    }

    #[test]
    fn include_filter() {
        let filter = ModuleFilter::new(Level::Info).include("bettertui_engine::render");

        assert!(filter.should_log("bettertui_engine::render"));
        assert!(filter.should_log("bettertui_engine::render::painter"));
        assert!(!filter.should_log("bettertui_engine::layout"));
    }

    #[test]
    fn exclude_filter() {
        let filter = ModuleFilter::new(Level::Info).exclude("bettertui_engine::render");

        assert!(!filter.should_log("bettertui_engine::render"));
        assert!(!filter.should_log("bettertui_engine::render::painter"));
        assert!(filter.should_log("bettertui_engine::layout"));
    }

    #[test]
    fn exclude_takes_precedence() {
        let filter = ModuleFilter::new(Level::Info).include("bettertui_engine").exclude("bettertui_engine::render");

        assert!(!filter.should_log("bettertui_engine::render"));
        assert!(!filter.should_log("bettertui_engine::render::painter"));
        assert!(filter.should_log("bettertui_engine::layout"));
    }

    #[test]
    fn multiple_includes() {
        let filter =
            ModuleFilter::new(Level::Info).include_many(vec!["bettertui_engine::render", "bettertui_engine::layout"]);

        assert!(filter.should_log("bettertui_engine::render"));
        assert!(filter.should_log("bettertui_engine::layout"));
        assert!(!filter.should_log("bettertui_engine::event"));
    }

    #[test]
    fn multiple_excludes() {
        let filter =
            ModuleFilter::new(Level::Info).exclude_many(vec!["bettertui_engine::render", "bettertui_engine::layout"]);

        assert!(!filter.should_log("bettertui_engine::render"));
        assert!(!filter.should_log("bettertui_engine::layout"));
        assert!(filter.should_log("bettertui_engine::event"));
    }

    #[test]
    fn prefix_matching() {
        let filter = ModuleFilter::new(Level::Info).include("bettertui_engine::render");

        assert!(filter.should_log("bettertui_engine::render"));
        assert!(filter.should_log("bettertui_engine::render::painter"));
        assert!(filter.should_log("bettertui_engine::render::painter::core"));
        // "bettertui_engine::renderer" starts with "bettertui_engine::render" so it matches
        // This is expected behavior for prefix matching
        assert!(filter.should_log("bettertui_engine::renderer"));
    }

    #[test]
    fn default_level() {
        let filter = ModuleFilter::new(Level::Warn);
        assert_eq!(filter.default_level(), Level::Warn);

        let filter = filter.with_default_level(Level::Debug);
        assert_eq!(filter.default_level(), Level::Debug);
    }
}
