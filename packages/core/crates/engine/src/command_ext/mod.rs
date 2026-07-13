//! Command platform extension for undo/redo and command history.
//!
//! Provides a command registry with execution, undo/redo support,
//! and history tracking for platform-specific command extensions.

use std::collections::VecDeque;

/// A command that can be executed and potentially undone.
#[derive(Debug, Clone)]
pub struct CommandEntry {
    /// The command name/identifier.
    pub name: String,
    /// Platform-specific command data.
    pub data: String,
    /// Whether this command supports undo.
    pub undoable: bool,
    /// Timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

impl CommandEntry {
    /// Creates a new command entry.
    pub fn new(name: impl Into<String>, data: impl Into<String>, undoable: bool) -> Self {
        Self {
            name: name.into(),
            data: data.into(),
            undoable,
            timestamp: 0,
        }
    }

    /// Sets the timestamp.
    pub fn with_timestamp(mut self, ts: u64) -> Self {
        self.timestamp = ts;
        self
    }
}

/// Result of executing a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandResult {
    /// Command executed successfully.
    Success,
    /// Command failed with a message.
    Failure(String),
    /// Command was not found.
    NotFound,
}

/// Manages command execution, undo/redo, and history.
#[derive(Debug)]
pub struct CommandRegistry {
    /// Command history (most recent last).
    history: VecDeque<CommandEntry>,
    /// Undo stack (commands that can be undone).
    undo_stack: Vec<CommandEntry>,
    /// Redo stack (commands that were undone).
    redo_stack: Vec<CommandEntry>,
    /// Maximum history size.
    max_history: usize,
    /// Maximum undo stack size.
    max_undo: usize,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRegistry {
    /// Creates a new CommandRegistry.
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history: 1000,
            max_undo: 100,
        }
    }

    /// Sets the maximum history size.
    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    /// Sets the maximum undo stack size.
    pub fn with_max_undo(mut self, max: usize) -> Self {
        self.max_undo = max;
        self
    }

    /// Executes a command and records it in history.
    pub fn execute(&mut self, entry: CommandEntry) -> CommandResult {
        let undoable = entry.undoable;
        self.history.push_back(entry.clone());
        while self.history.len() > self.max_history {
            self.history.pop_front();
        }
        if undoable {
            self.undo_stack.push(entry);
            if self.undo_stack.len() > self.max_undo {
                self.undo_stack.remove(0);
            }
            // Clear redo stack on new command
            self.redo_stack.clear();
        }
        CommandResult::Success
    }

    /// Undoes the last undoable command.
    pub fn undo(&mut self) -> Option<CommandEntry> {
        let entry = self.undo_stack.pop()?;
        self.redo_stack.push(entry.clone());
        Some(entry)
    }

    /// Redoes the last undone command.
    pub fn redo(&mut self) -> Option<CommandEntry> {
        let entry = self.redo_stack.pop()?;
        self.undo_stack.push(entry.clone());
        Some(entry)
    }

    /// Returns whether undo is possible.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Returns whether redo is possible.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Returns the command history.
    pub fn history(&self) -> impl Iterator<Item = &CommandEntry> {
        self.history.iter()
    }

    /// Returns the number of commands in history.
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Returns the undo stack depth.
    pub fn undo_depth(&self) -> usize {
        self.undo_stack.len()
    }

    /// Returns the redo stack depth.
    pub fn redo_depth(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clears all history and stacks.
    pub fn clear(&mut self) {
        self.history.clear();
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Finds commands by name.
    pub fn find(&self, name: &str) -> Vec<&CommandEntry> {
        self.history.iter().filter(|e| e.name == name).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_records_history() {
        let mut reg = CommandRegistry::new();
        let entry = CommandEntry::new("test", "data", false);
        reg.execute(entry);
        assert_eq!(reg.history_len(), 1);
    }

    #[test]
    fn undo_redo() {
        let mut reg = CommandRegistry::new();
        reg.execute(CommandEntry::new("cmd", "data", true));
        assert!(reg.can_undo());
        let undone = reg.undo();
        assert!(undone.is_some());
        assert!(!reg.can_undo());
        assert!(reg.can_redo());
        let redone = reg.redo();
        assert!(redone.is_some());
        assert!(!reg.can_redo());
    }

    #[test]
    fn undo_empty() {
        let mut reg = CommandRegistry::new();
        assert!(reg.undo().is_none());
    }

    #[test]
    fn redo_empty() {
        let mut reg = CommandRegistry::new();
        assert!(reg.redo().is_none());
    }

    #[test]
    fn non_undoable_not_in_undo_stack() {
        let mut reg = CommandRegistry::new();
        reg.execute(CommandEntry::new("cmd", "data", false));
        assert!(!reg.can_undo());
    }

    #[test]
    fn new_command_clears_redo() {
        let mut reg = CommandRegistry::new();
        reg.execute(CommandEntry::new("cmd", "data", true));
        reg.undo();
        assert!(reg.can_redo());
        reg.execute(CommandEntry::new("cmd2", "data2", true));
        assert!(!reg.can_redo());
    }

    #[test]
    fn max_history() {
        let mut reg = CommandRegistry::new().with_max_history(3);
        for i in 0..5 {
            reg.execute(CommandEntry::new("cmd", i.to_string(), false));
        }
        assert_eq!(reg.history_len(), 3);
    }

    #[test]
    fn max_undo() {
        let mut reg = CommandRegistry::new().with_max_undo(2);
        for i in 0..5 {
            reg.execute(CommandEntry::new("cmd", i.to_string(), true));
        }
        assert_eq!(reg.undo_depth(), 2);
    }

    #[test]
    fn find_commands() {
        let mut reg = CommandRegistry::new();
        reg.execute(CommandEntry::new("save", "f1", false));
        reg.execute(CommandEntry::new("open", "f2", false));
        reg.execute(CommandEntry::new("save", "f3", false));
        let saves = reg.find("save");
        assert_eq!(saves.len(), 2);
    }

    #[test]
    fn clear() {
        let mut reg = CommandRegistry::new();
        reg.execute(CommandEntry::new("cmd", "data", true));
        reg.clear();
        assert_eq!(reg.history_len(), 0);
        assert!(!reg.can_undo());
    }
}
