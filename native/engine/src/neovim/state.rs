#[derive(Debug, Clone)]
pub struct NeovimState {
    running: bool,
    mode: NeovimMode,
    modified: bool,
    filename: Option<String>,
    line_count: usize,
    cursor_position: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeovimMode {
    Normal,
    Insert,
    Visual,
    Command,
    Replace,
    Terminal,
}

impl NeovimMode {
    pub fn mode_name(&self) -> &'static str {
        match self {
            NeovimMode::Normal => "NORMAL",
            NeovimMode::Insert => "INSERT",
            NeovimMode::Visual => "VISUAL",
            NeovimMode::Command => "COMMAND",
            NeovimMode::Replace => "REPLACE",
            NeovimMode::Terminal => "TERMINAL",
        }
    }
}

impl Default for NeovimState {
    fn default() -> Self {
        Self::new()
    }
}

impl NeovimState {
    pub fn new() -> Self {
        Self {
            running: false,
            mode: NeovimMode::Normal,
            modified: false,
            filename: None,
            line_count: 0,
            cursor_position: (1, 1),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    pub fn mode(&self) -> NeovimMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: NeovimMode) {
        self.mode = mode;
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }

    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }

    pub fn set_filename(&mut self, filename: Option<String>) {
        self.filename = filename;
    }

    pub fn line_count(&self) -> usize {
        self.line_count
    }

    pub fn set_line_count(&mut self, count: usize) {
        self.line_count = count;
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor_position
    }

    pub fn set_cursor_position(&mut self, row: usize, col: usize) {
        self.cursor_position = (row, col);
    }

    pub fn mode_name(&self) -> &'static str {
        self.mode.mode_name()
    }

    pub fn status_line(&self) -> String {
        let mode = self.mode_name();
        let file = self.filename().unwrap_or("[No Name]");
        let modified = if self.modified { " [+]" } else { "" };
        let position = format!("{}:{}", self.cursor_position.0, self.cursor_position.1);

        format!(" {} | {}{} | {}", mode, file, modified, position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_new() {
        let state = NeovimState::new();
        assert!(!state.is_running());
        assert_eq!(state.mode(), NeovimMode::Normal);
    }

    #[test]
    fn state_mode() {
        let mut state = NeovimState::new();
        state.set_mode(NeovimMode::Insert);
        assert_eq!(state.mode(), NeovimMode::Insert);
        assert_eq!(state.mode_name(), "INSERT");
    }

    #[test]
    fn state_filename() {
        let mut state = NeovimState::new();
        state.set_filename(Some("test.rs".to_string()));
        assert_eq!(state.filename(), Some("test.rs"));
    }

    #[test]
    fn state_modified() {
        let mut state = NeovimState::new();
        state.set_modified(true);
        assert!(state.is_modified());
    }

    #[test]
    fn state_cursor() {
        let mut state = NeovimState::new();
        state.set_cursor_position(10, 5);
        assert_eq!(state.cursor_position(), (10, 5));
    }

    #[test]
    fn state_status_line() {
        let mut state = NeovimState::new();
        state.set_filename(Some("test.rs".to_string()));
        state.set_modified(true);
        state.set_cursor_position(10, 5);

        let status = state.status_line();
        assert!(status.contains("NORMAL"));
        assert!(status.contains("test.rs"));
        assert!(status.contains("[+]"));
        assert!(status.contains("10:5"));
    }

    #[test]
    fn state_mode_names() {
        assert_eq!(NeovimMode::Normal.mode_name(), "NORMAL");
        assert_eq!(NeovimMode::Insert.mode_name(), "INSERT");
        assert_eq!(NeovimMode::Visual.mode_name(), "VISUAL");
        assert_eq!(NeovimMode::Command.mode_name(), "COMMAND");
        assert_eq!(NeovimMode::Replace.mode_name(), "REPLACE");
        assert_eq!(NeovimMode::Terminal.mode_name(), "TERMINAL");
    }
}
