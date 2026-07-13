use std::path::PathBuf;
use std::time::Duration;

use crate::pty::PtySize;

#[derive(Debug, Clone)]
pub struct ProcessConfig {
    pub program: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub working_directory: Option<PathBuf>,
    pub size: PtySize,
    pub auto_restart: bool,
    pub restart_delay: Duration,
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            program: String::new(),
            args: Vec::new(),
            env: Vec::new(),
            working_directory: None,
            size: PtySize::default(),
            auto_restart: false,
            restart_delay: Duration::from_millis(500),
        }
    }
}

impl ProcessConfig {
    pub fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
            ..Default::default()
        }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn with_env(mut self, env: Vec<(String, String)>) -> Self {
        self.env = env;
        self
    }

    pub fn with_working_directory(mut self, dir: PathBuf) -> Self {
        self.working_directory = Some(dir);
        self
    }

    pub fn with_size(mut self, size: PtySize) -> Self {
        self.size = size;
        self
    }

    pub fn with_auto_restart(mut self, enabled: bool) -> Self {
        self.auto_restart = enabled;
        self
    }

    pub fn with_restart_delay(mut self, delay: Duration) -> Self {
        self.restart_delay = delay;
        self
    }

    pub fn is_valid(&self) -> bool {
        !self.program.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new() {
        let config = ProcessConfig::new("bash");
        assert_eq!(config.program, "bash");
        assert!(config.is_valid());
    }

    #[test]
    fn config_default() {
        let config = ProcessConfig::default();
        assert!(!config.is_valid());
        assert_eq!(config.args.len(), 0);
    }

    #[test]
    fn config_builder() {
        let config = ProcessConfig::new("nvim")
            .with_args(vec!["--clean".to_string()])
            .with_env(vec![("TERM".to_string(), "xterm-256color".to_string())])
            .with_working_directory(PathBuf::from("/tmp"))
            .with_auto_restart(true)
            .with_restart_delay(Duration::from_secs(1));

        assert_eq!(config.program, "nvim");
        assert_eq!(config.args.len(), 1);
        assert!(!config.env.is_empty());
        assert!(config.auto_restart);
    }

    #[test]
    fn config_empty_program_invalid() {
        let config = ProcessConfig::default();
        assert!(!config.is_valid());
    }
}
