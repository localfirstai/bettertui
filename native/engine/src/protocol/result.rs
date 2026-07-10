use super::error::{CommandError, CommandWarning};

/// Result of processing a batch of commands.
#[derive(Debug, Clone, Default)]
pub struct CommandResult {
    /// Number of commands successfully processed.
    pub processed: usize,
    /// Number of commands that failed.
    pub failed: usize,
    /// Errors from failed commands.
    pub errors: Vec<CommandError>,
    /// Non-fatal warnings.
    pub warnings: Vec<CommandWarning>,
}

impl CommandResult {
    /// Create an empty result.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a result for a single successful command.
    pub fn success() -> Self {
        Self {
            processed: 1,
            ..Default::default()
        }
    }

    /// Create a result for a single failed command.
    pub fn error(err: CommandError) -> Self {
        Self {
            failed: 1,
            errors: vec![err],
            ..Default::default()
        }
    }

    /// Add a success to the result.
    pub fn push_success(&mut self) {
        self.processed += 1;
    }

    /// Add a failure to the result.
    pub fn push_error(&mut self, err: CommandError) {
        self.failed += 1;
        self.errors.push(err);
    }

    /// Add a warning to the result.
    pub fn push_warning(&mut self, warn: CommandWarning) {
        self.warnings.push(warn);
    }

    /// Merge another result into this one.
    pub fn merge(&mut self, other: CommandResult) {
        self.processed += other.processed;
        self.failed += other.failed;
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }

    /// Returns true if all commands succeeded.
    pub fn is_success(&self) -> bool {
        self.failed == 0
    }

    /// Returns true if any commands failed.
    pub fn has_errors(&self) -> bool {
        self.failed > 0
    }

    /// Total commands processed (success + failed).
    pub fn total(&self) -> usize {
        self.processed + self.failed
    }
}

impl std::fmt::Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CommandResult(processed={}, failed={}, warnings={})",
            self.processed,
            self.failed,
            self.warnings.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_default() {
        let result = CommandResult::new();
        assert_eq!(result.processed, 0);
        assert_eq!(result.failed, 0);
        assert!(result.is_success());
    }

    #[test]
    fn result_success() {
        let result = CommandResult::success();
        assert_eq!(result.processed, 1);
        assert!(result.is_success());
    }

    #[test]
    fn result_error() {
        let result = CommandResult::error(CommandError::InvalidOperation("test".into()));
        assert_eq!(result.failed, 1);
        assert!(result.has_errors());
    }

    #[test]
    fn result_merge() {
        let mut r1 = CommandResult::success();
        let r2 = CommandResult::error(CommandError::InvalidOperation("test".into()));
        r1.merge(r2);

        assert_eq!(r1.processed, 1);
        assert_eq!(r1.failed, 1);
        assert_eq!(r1.total(), 2);
    }

    #[test]
    fn result_display() {
        let result = CommandResult::new();
        let display = format!("{result}");
        assert!(display.contains("CommandResult"));
    }
}
