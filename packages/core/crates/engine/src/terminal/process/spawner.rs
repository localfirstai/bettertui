use std::collections::HashMap;

use crate::pty::PtySize;
use crate::terminal::process::config::ProcessConfig;

pub struct SpawnResult {
    pub process_config: ProcessConfig,
    pub pid: u32,
    pub time: std::time::Instant,
}

impl SpawnResult {
    pub fn new(process_config: ProcessConfig, pid: u32) -> Self {
        Self {
            process_config,
            pid,
            time: std::time::Instant::now(),
        }
    }
}

pub struct ProcessSpawner {
    default_shell: String,
    default_size: PtySize,
    default_env: Vec<(String, String)>,
    search_paths: Vec<String>,
    spawn_history: Vec<SpawnResult>,
    max_history: usize,
}

impl Default for ProcessSpawner {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessSpawner {
    pub fn new() -> Self {
        let default_shell = std::env::var("SHELL")
            .or_else(|_| std::env::var("COMSPEC"))
            .unwrap_or_else(|_| "/bin/bash".to_string());

        Self {
            default_shell,
            default_size: PtySize::default(),
            default_env: Vec::new(),
            search_paths: Self::default_search_paths(),
            spawn_history: Vec::new(),
            max_history: 100,
        }
    }

    pub fn with_default_shell(mut self, shell: &str) -> Self {
        self.default_shell = shell.to_string();
        self
    }

    pub fn with_default_size(mut self, size: PtySize) -> Self {
        self.default_size = size;
        self
    }

    pub fn with_default_env(mut self, env: Vec<(String, String)>) -> Self {
        self.default_env = env;
        self
    }

    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    pub fn resolve_command(&self, command: &str) -> String {
        if command.contains('/') || cfg!(target_os = "windows") {
            return command.to_string();
        }
        for path in &self.search_paths {
            let full = format!("{}/{}", path, command);
            if std::path::Path::new(&full).exists() {
                return full;
            }
        }
        command.to_string()
    }

    pub fn build_config(&self, program: &str) -> ProcessConfig {
        ProcessConfig {
            program: self.resolve_command(program),
            args: Vec::new(),
            env: self.default_env.clone(),
            working_directory: None,
            size: self.default_size,
            auto_restart: false,
            restart_delay: std::time::Duration::from_millis(500),
        }
    }

    pub fn build_config_with_args(&self, program: &str, args: Vec<String>) -> ProcessConfig {
        ProcessConfig {
            program: self.resolve_command(program),
            args,
            env: self.default_env.clone(),
            working_directory: None,
            size: self.default_size,
            auto_restart: false,
            restart_delay: std::time::Duration::from_millis(500),
        }
    }

    pub fn build_config_from_parts(&self, parts: ProcessConfigBuilder) -> ProcessConfig {
        let program = if parts.program.is_empty() {
            self.default_shell.clone()
        } else {
            self.resolve_command(&parts.program)
        };

        let mut env = self.default_env.clone();
        env.extend(parts.env);

        ProcessConfig {
            program,
            args: parts.args,
            env,
            working_directory: parts.working_directory,
            size: parts.size.unwrap_or(self.default_size),
            auto_restart: parts.auto_restart,
            restart_delay: parts
                .restart_delay
                .unwrap_or(std::time::Duration::from_millis(500)),
        }
    }

    pub fn record_spawn(&mut self, result: SpawnResult) {
        self.spawn_history.push(result);
        if self.spawn_history.len() > self.max_history {
            self.spawn_history.remove(0);
        }
    }

    pub fn spawn_history(&self) -> &[SpawnResult] {
        &self.spawn_history
    }

    pub fn default_shell(&self) -> &str {
        &self.default_shell
    }

    pub fn default_size(&self) -> PtySize {
        self.default_size
    }

    fn default_search_paths() -> Vec<String> {
        let mut paths = Vec::new();
        if let Ok(path_var) = std::env::var("PATH") {
            paths.extend(std::env::split_paths(&path_var).map(|p| p.to_string_lossy().to_string()));
        }
        paths
    }
}

pub struct ProcessConfigBuilder {
    pub program: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub working_directory: Option<std::path::PathBuf>,
    pub size: Option<PtySize>,
    pub auto_restart: bool,
    pub restart_delay: Option<std::time::Duration>,
}

impl ProcessConfigBuilder {
    pub fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
            args: Vec::new(),
            env: Vec::new(),
            working_directory: None,
            size: None,
            auto_restart: false,
            restart_delay: None,
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

    pub fn with_env_map(mut self, env: HashMap<String, String>) -> Self {
        self.env = env.into_iter().collect();
        self
    }

    pub fn with_working_directory(mut self, dir: std::path::PathBuf) -> Self {
        self.working_directory = Some(dir);
        self
    }

    pub fn with_size(mut self, size: PtySize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_auto_restart(mut self, enabled: bool) -> Self {
        self.auto_restart = enabled;
        self
    }

    pub fn with_restart_delay(mut self, delay: std::time::Duration) -> Self {
        self.restart_delay = Some(delay);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawner_new() {
        let spawner = ProcessSpawner::new();
        assert!(!spawner.default_shell().is_empty());
        assert!(spawner.spawn_history().is_empty());
    }

    #[test]
    fn spawner_default() {
        let spawner = ProcessSpawner::default();
        assert!(!spawner.default_shell().is_empty());
    }

    #[test]
    fn spawner_build_config() {
        let spawner = ProcessSpawner::new();
        let config = spawner.build_config("bash");
        assert!(config.program.ends_with("bash"));
    }

    #[test]
    fn spawner_build_config_with_args() {
        let spawner = ProcessSpawner::new();
        let config = spawner.build_config_with_args("bash", vec!["--norc".to_string()]);
        assert!(config.program.ends_with("bash"));
        assert_eq!(config.args.len(), 1);
    }

    #[test]
    fn spawner_record_history() {
        let mut spawner = ProcessSpawner::new();
        let config = spawner.build_config("echo");
        let result = SpawnResult::new(config, 42);
        spawner.record_spawn(result);
        assert_eq!(spawner.spawn_history().len(), 1);
    }

    #[test]
    fn spawner_builder() {
        let builder = ProcessConfigBuilder::new("nvim")
            .with_args(vec!["file.txt".to_string()])
            .with_size(PtySize::new(120, 40));

        let spawner = ProcessSpawner::new();
        let config = spawner.build_config_from_parts(builder);
        assert!(config.program.ends_with("nvim"));
        assert_eq!(config.args.len(), 1);
        assert_eq!(config.size.cols, 120);
    }

    #[test]
    fn builder_env_map() {
        let mut map = HashMap::new();
        map.insert("TERM".to_string(), "xterm-256color".to_string());
        map.insert("COLORTERM".to_string(), "truecolor".to_string());

        let builder = ProcessConfigBuilder::new("test").with_env_map(map);
        assert_eq!(builder.env.len(), 2);
    }
}
