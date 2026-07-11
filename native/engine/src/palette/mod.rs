//! Command palette for fuzzy search and command discovery.
//!
//! Provides fuzzy matching, scored search results, and keyboard-navigable
//! command discovery for editor-like interfaces.

/// A command that can appear in the palette.
#[derive(Debug, Clone)]
pub struct PaletteCommand {
    /// The command label (displayed to user).
    pub label: String,
    /// Optional description.
    pub description: String,
    /// Command category for filtering.
    pub category: String,
    /// Keyboard shortcut hint.
    pub shortcut: Option<String>,
    /// Whether this command is currently enabled.
    pub enabled: bool,
}

impl PaletteCommand {
    /// Creates a new palette command.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: String::new(),
            category: String::new(),
            shortcut: None,
            enabled: true,
        }
    }

    /// Adds a description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Adds a category.
    pub fn with_category(mut self, cat: impl Into<String>) -> Self {
        self.category = cat.into();
        self
    }

    /// Adds a shortcut.
    pub fn with_shortcut(mut self, sc: impl Into<String>) -> Self {
        self.shortcut = Some(sc.into());
        self
    }
}

/// A scored search result.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// The matched command.
    pub command: PaletteCommand,
    /// Match score (higher is better).
    pub score: i64,
    /// Matched character indices in the label.
    pub matches: Vec<usize>,
}

/// Fuzzy matching score calculation.
fn fuzzy_score(query: &str, target: &str) -> Option<(i64, Vec<usize>)> {
    if query.is_empty() {
        return Some((0, vec![]));
    }

    let query_lower: Vec<char> = query.chars().map(|c| c.to_ascii_lowercase()).collect();
    let target_lower: Vec<char> = target.chars().map(|c| c.to_ascii_lowercase()).collect();

    let mut matches = Vec::new();
    let mut qi = 0;

    for (ti, &tc) in target_lower.iter().enumerate() {
        if qi < query_lower.len() && tc == query_lower[qi] {
            matches.push(ti);
            qi += 1;
        }
    }

    if qi < query_lower.len() {
        return None; // Not all query chars matched
    }

    // Score: prefer matches at start, consecutive matches, shorter targets
    let mut score = 0i64;

    // Bonus for matches at word boundaries
    for &mi in &matches {
        if mi == 0
            || target_lower[mi - 1] == ' '
            || target_lower[mi - 1] == '_'
            || target_lower[mi - 1] == '-'
        {
            score += 10;
        }
    }

    // Bonus for consecutive matches
    for window in matches.windows(2) {
        if window[1] == window[0] + 1 {
            score += 5;
        }
    }

    // Bonus for exact match
    if matches.len() == target_lower.len() {
        score += 20;
    }

    // Penalty for longer targets
    score -= target_lower.len() as i64 / 2;

    // Bonus for query length match ratio
    score += (matches.len() as i64 * 10) / query_lower.len().max(1) as i64;

    Some((score, matches))
}

/// The command palette providing fuzzy search and navigation.
#[derive(Debug)]
pub struct CommandPalette {
    /// All available commands.
    commands: Vec<PaletteCommand>,
    /// Current search query.
    query: String,
    /// Current selection index in filtered results.
    selected: usize,
    /// Cached search results.
    results: Vec<SearchResult>,
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandPalette {
    /// Creates a new empty palette.
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            query: String::new(),
            selected: 0,
            results: Vec::new(),
        }
    }

    /// Adds a command to the palette.
    pub fn add(&mut self, command: PaletteCommand) {
        self.commands.push(command);
        self.update_results();
    }

    /// Removes all commands.
    pub fn clear(&mut self) {
        self.commands.clear();
        self.results.clear();
        self.query.clear();
        self.selected = 0;
    }

    /// Sets the search query and updates results.
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.selected = 0;
        self.update_results();
    }

    /// Returns the current query.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Returns the current search results.
    pub fn results(&self) -> &[SearchResult] {
        &self.results
    }

    /// Returns the currently selected result.
    pub fn selected(&self) -> Option<&SearchResult> {
        self.results.get(self.selected)
    }

    /// Returns the selected index.
    pub fn selected_index(&self) -> usize {
        self.selected
    }

    /// Moves selection up.
    pub fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Moves selection down.
    pub fn select_next(&mut self) {
        if self.selected + 1 < self.results.len() {
            self.selected += 1;
        }
    }

    /// Selects the first result.
    pub fn select_first(&mut self) {
        self.selected = 0;
    }

    /// Selects the last result.
    pub fn select_last(&mut self) {
        if !self.results.is_empty() {
            self.selected = self.results.len() - 1;
        }
    }

    /// Returns the number of available commands.
    pub fn command_count(&self) -> usize {
        self.commands.len()
    }

    /// Returns the number of search results.
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    fn update_results(&mut self) {
        if self.query.is_empty() {
            self.results = self
                .commands
                .iter()
                .filter(|c| c.enabled)
                .cloned()
                .map(|c| SearchResult {
                    command: c,
                    score: 0,
                    matches: vec![],
                })
                .collect();
        } else {
            let mut scored: Vec<SearchResult> = self
                .commands
                .iter()
                .filter(|c| c.enabled)
                .filter_map(|c| {
                    fuzzy_score(&self.query, &c.label).map(|(score, matches)| SearchResult {
                        command: c.clone(),
                        score,
                        matches,
                    })
                })
                .collect();
            scored.sort_by_key(|r| std::cmp::Reverse(r.score));
            self.results = scored;
        }
        if self.selected >= self.results.len() {
            self.selected = self.results.len().saturating_sub(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_commands() -> Vec<PaletteCommand> {
        vec![
            PaletteCommand::new("Save File").with_category("File"),
            PaletteCommand::new("Open File").with_category("File"),
            PaletteCommand::new("Find and Replace").with_category("Edit"),
            PaletteCommand::new("Toggle Terminal").with_category("View"),
        ]
    }

    #[test]
    fn fuzzy_match_exact() {
        let (score, matches) = fuzzy_score("save", "Save File").unwrap();
        assert!(score > 0);
        assert_eq!(matches.len(), 4);
    }

    #[test]
    fn fuzzy_match_partial() {
        let result = fuzzy_score("sv", "Save File");
        assert!(result.is_some());
    }

    #[test]
    fn fuzzy_no_match() {
        let result = fuzzy_score("xyz", "Save File");
        assert!(result.is_none());
    }

    #[test]
    fn fuzzy_empty_query() {
        let result = fuzzy_score("", "anything");
        assert!(result.is_some());
    }

    #[test]
    fn palette_add_and_query() {
        let mut palette = CommandPalette::new();
        for cmd in test_commands() {
            palette.add(cmd);
        }
        assert_eq!(palette.command_count(), 4);
        palette.set_query("save");
        assert_eq!(palette.result_count(), 1);
        assert_eq!(palette.selected().unwrap().command.label, "Save File");
    }

    #[test]
    fn palette_navigation() {
        let mut palette = CommandPalette::new();
        for cmd in test_commands() {
            palette.add(cmd);
        }
        palette.set_query("file");
        assert!(palette.result_count() >= 2);
        palette.select_next();
        assert!(palette.selected_index() > 0);
        palette.select_previous();
        assert_eq!(palette.selected_index(), 0);
    }

    #[test]
    fn palette_first_last() {
        let mut palette = CommandPalette::new();
        for cmd in test_commands() {
            palette.add(cmd);
        }
        palette.set_query("");
        palette.select_last();
        assert_eq!(palette.selected_index(), palette.result_count() - 1);
        palette.select_first();
        assert_eq!(palette.selected_index(), 0);
    }

    #[test]
    fn palette_empty_query_shows_all() {
        let mut palette = CommandPalette::new();
        for cmd in test_commands() {
            palette.add(cmd);
        }
        palette.set_query("");
        assert_eq!(palette.result_count(), 4);
    }

    #[test]
    fn palette_disabled_commands_filtered() {
        let mut palette = CommandPalette::new();
        palette.add(PaletteCommand::new("Enabled"));
        palette.add(PaletteCommand::new("Disabled").with_category("test"));
        palette.commands[1].enabled = false;
        palette.set_query("");
        assert_eq!(palette.result_count(), 1);
    }

    #[test]
    fn palette_clear() {
        let mut palette = CommandPalette::new();
        palette.add(PaletteCommand::new("test"));
        palette.clear();
        assert_eq!(palette.command_count(), 0);
        assert_eq!(palette.result_count(), 0);
    }

    #[test]
    fn palette_shortcut() {
        let cmd = PaletteCommand::new("Save").with_shortcut("Ctrl+S");
        assert_eq!(cmd.shortcut.as_deref(), Some("Ctrl+S"));
    }
}
