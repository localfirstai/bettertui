use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct NeovimConfig {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub session_file: Option<PathBuf>,
    pub init_lua: Option<PathBuf>,
    pub preserve_user_config: bool,
}

impl Default for NeovimConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl NeovimConfig {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let home_path = PathBuf::from(home);
        let config_dir = home_path.join(".config").join("nvim");
        let data_dir = home_path.join(".local").join("share").join("nvim");
        let cache_dir = home_path.join(".cache").join("nvim");

        Self {
            config_dir,
            data_dir,
            cache_dir,
            session_file: None,
            init_lua: None,
            preserve_user_config: true,
        }
    }

    pub fn with_config_dir(mut self, dir: PathBuf) -> Self {
        self.config_dir = dir;
        self
    }

    pub fn with_data_dir(mut self, dir: PathBuf) -> Self {
        self.data_dir = dir;
        self
    }

    pub fn with_cache_dir(mut self, dir: PathBuf) -> Self {
        self.cache_dir = dir;
        self
    }

    pub fn with_session_file(mut self, file: PathBuf) -> Self {
        self.session_file = Some(file);
        self
    }

    pub fn with_init_lua(mut self, file: PathBuf) -> Self {
        self.init_lua = Some(file);
        self
    }

    pub fn with_preserve_user_config(mut self, preserve: bool) -> Self {
        self.preserve_user_config = preserve;
        self
    }

    pub fn user_config_exists(&self) -> bool {
        self.config_dir.exists() && self.config_dir.is_dir()
    }

    pub fn user_init_lua_exists(&self) -> bool {
        let init_lua = self.config_dir.join("init.lua");
        init_lua.exists()
    }

    pub fn user_init_vim_exists(&self) -> bool {
        let init_vim = self.config_dir.join("init.vim");
        init_vim.exists()
    }

    pub fn session_file_path(&self) -> Option<&Path> {
        self.session_file.as_deref()
    }

    pub fn build_nvim_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        if self.preserve_user_config && self.user_config_exists() {
            args.push("--cmd".to_string());
            args.push(format!("set rtp^={}", self.config_dir.display()));
        }

        if let Some(ref init_lua) = self.init_lua {
            args.push("-u".to_string());
            args.push(init_lua.display().to_string());
        }

        if let Some(ref session) = self.session_file {
            args.push("-S".to_string());
            args.push(session.display().to_string());
        }

        args.push("--clean".to_string());
        args
    }

    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.config_dir)?;
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new() {
        let config = NeovimConfig::new();
        assert!(config.preserve_user_config);
        assert!(config.config_dir.ends_with("nvim"));
    }

    #[test]
    fn config_builder() {
        let config = NeovimConfig::new()
            .with_config_dir(PathBuf::from("/tmp/test-config"))
            .with_data_dir(PathBuf::from("/tmp/test-data"))
            .with_cache_dir(PathBuf::from("/tmp/test-cache"))
            .with_preserve_user_config(false);

        assert_eq!(config.config_dir, PathBuf::from("/tmp/test-config"));
        assert_eq!(config.data_dir, PathBuf::from("/tmp/test-data"));
        assert_eq!(config.cache_dir, PathBuf::from("/tmp/test-cache"));
        assert!(!config.preserve_user_config);
    }

    #[test]
    fn config_build_args() {
        let config = NeovimConfig::new()
            .with_preserve_user_config(false)
            .with_session_file(PathBuf::from("/tmp/session.vim"));

        let args = config.build_nvim_args();
        assert!(args.contains(&"--clean".to_string()));
        assert!(args.contains(&"-S".to_string()));
    }
}
