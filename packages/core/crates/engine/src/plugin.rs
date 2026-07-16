//! Plugin host for lifecycle management and capability negotiation.
//!
//! Provides a registry for plugins with lifecycle hooks (init, start, stop, destroy),
//! capability negotiation, and extension points for custom commands.

use std::collections::HashMap;

/// Plugin lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PluginState {
    /// Plugin is registered but not initialized.
    #[default]
    Registered,
    /// Plugin has been initialized.
    Initialized,
    /// Plugin is running.
    Running,
    /// Plugin has been stopped.
    Stopped,
    /// Plugin encountered an error.
    Error,
}

/// Capabilities a plugin can declare.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Can provide custom commands.
    Commands,
    /// Can provide custom widgets.
    Widgets,
    /// Can provide custom themes.
    Themes,
    /// Can intercept events.
    Events,
    /// Can provide file system access.
    FileSystem,
    /// Custom capability string.
    Custom(String),
}

/// Metadata about a plugin.
#[derive(Debug, Clone)]
pub struct PluginInfo {
    /// Plugin name.
    pub name: String,
    /// Plugin version.
    pub version: String,
    /// Plugin author.
    pub author: String,
    /// Capabilities this plugin provides.
    pub capabilities: Vec<Capability>,
    /// Additional metadata.
    pub metadata: HashMap<String, String>,
}

/// A registered plugin with its info and state.
#[derive(Debug)]
pub struct Plugin {
    /// Plugin metadata.
    pub info: PluginInfo,
    /// Current state.
    pub state: PluginState,
    /// Commands provided by this plugin.
    pub commands: Vec<String>,
}

/// Manages plugin registration, lifecycle, and capability queries.
#[derive(Debug)]
pub struct PluginHost {
    plugins: Vec<Plugin>,
    index: HashMap<String, usize>,
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHost {
    /// Creates a new plugin host.
    pub fn new() -> Self {
        Self { plugins: Vec::new(), index: HashMap::new() }
    }

    /// Registers a plugin.
    pub fn register(&mut self, info: PluginInfo) -> Result<(), String> {
        if self.index.contains_key(&info.name) {
            return Err(format!("Plugin '{}' is already registered", info.name));
        }
        let idx = self.plugins.len();
        self.plugins.push(Plugin { info: info.clone(), state: PluginState::Registered, commands: Vec::new() });
        self.index.insert(info.name, idx);
        Ok(())
    }

    /// Unregisters a plugin by name.
    pub fn unregister(&mut self, name: &str) -> Result<(), String> {
        let idx = self.index.remove(name).ok_or_else(|| format!("Plugin '{name}' is not registered"))?;
        self.plugins.remove(idx);
        self.index.clear();
        for (i, plugin) in self.plugins.iter().enumerate() {
            self.index.insert(plugin.info.name.clone(), i);
        }
        Ok(())
    }

    /// Transitions a plugin to a new state.
    pub fn set_state(&mut self, name: &str, state: PluginState) -> Result<(), String> {
        let idx = self.index.get(name).ok_or_else(|| format!("Plugin '{name}' is not registered"))?;
        self.plugins[*idx].state = state;
        Ok(())
    }

    /// Returns the state of a plugin.
    pub fn state(&self, name: &str) -> Option<PluginState> {
        self.index.get(name).map(|&idx| self.plugins[idx].state)
    }

    /// Returns a reference to a plugin.
    pub fn get(&self, name: &str) -> Option<&Plugin> {
        self.index.get(name).map(|&idx| &self.plugins[idx])
    }

    /// Returns the number of registered plugins.
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Returns `true` if no plugins are registered.
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Returns all plugins with a given capability.
    pub fn with_capability(&self, cap: &Capability) -> Vec<&Plugin> {
        self.plugins.iter().filter(|p| p.info.capabilities.contains(cap)).collect()
    }

    /// Returns all plugin names.
    pub fn names(&self) -> Vec<&str> {
        self.plugins.iter().map(|p| p.info.name.as_str()).collect()
    }

    /// Registers a command for a plugin.
    pub fn add_command(&mut self, plugin_name: &str, command: String) -> Result<(), String> {
        let idx = self.index.get(plugin_name).ok_or_else(|| format!("Plugin '{plugin_name}' is not registered"))?;
        self.plugins[*idx].commands.push(command);
        Ok(())
    }

    /// Returns all commands provided by all plugins.
    pub fn all_commands(&self) -> Vec<&str> {
        self.plugins.iter().flat_map(|p| p.commands.iter().map(|s| s.as_str())).collect()
    }

    /// Returns all running plugins.
    pub fn running(&self) -> Vec<&Plugin> {
        self.plugins.iter().filter(|p| p.state == PluginState::Running).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_info(name: &str) -> PluginInfo {
        PluginInfo {
            name: name.to_string(),
            version: "1.0".to_string(),
            author: "test".to_string(),
            capabilities: vec![Capability::Commands],
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn host_new_is_empty() {
        let host = PluginHost::new();
        assert!(host.is_empty());
        assert_eq!(host.len(), 0);
    }

    #[test]
    fn host_register_plugin() {
        let mut host = PluginHost::new();
        host.register(test_info("my-plugin")).unwrap();
        assert_eq!(host.len(), 1);
        assert!(!host.is_empty());
    }

    #[test]
    fn host_register_duplicate_fails() {
        let mut host = PluginHost::new();
        host.register(test_info("dup")).unwrap();
        let err = host.register(test_info("dup")).unwrap_err();
        assert!(err.contains("already registered"));
    }

    #[test]
    fn host_unregister_plugin() {
        let mut host = PluginHost::new();
        host.register(test_info("p1")).unwrap();
        host.register(test_info("p2")).unwrap();
        host.unregister("p1").unwrap();
        assert_eq!(host.len(), 1);
        assert!(host.get("p1").is_none());
        assert!(host.get("p2").is_some());
    }

    #[test]
    fn host_unregister_missing_fails() {
        let mut host = PluginHost::new();
        let err = host.unregister("nonexistent").unwrap_err();
        assert!(err.contains("not registered"));
    }

    #[test]
    fn host_set_state() {
        let mut host = PluginHost::new();
        host.register(test_info("p")).unwrap();
        host.set_state("p", PluginState::Running).unwrap();
        assert_eq!(host.state("p"), Some(PluginState::Running));
    }

    #[test]
    fn host_state_for_missing_returns_none() {
        let host = PluginHost::new();
        assert_eq!(host.state("nope"), None);
    }

    #[test]
    fn host_get_plugin() {
        let mut host = PluginHost::new();
        host.register(test_info("p")).unwrap();
        let plugin = host.get("p").unwrap();
        assert_eq!(plugin.info.name, "p");
        assert_eq!(plugin.state, PluginState::Registered);
    }

    #[test]
    fn host_get_missing_returns_none() {
        let host = PluginHost::new();
        assert!(host.get("nope").is_none());
    }

    #[test]
    fn host_with_capability() {
        let mut host = PluginHost::new();
        let mut info = test_info("a");
        info.capabilities = vec![Capability::Commands];
        host.register(info).unwrap();
        let mut info2 = test_info("b");
        info2.capabilities = vec![Capability::Events];
        host.register(info2).unwrap();
        let cmd_plugins = host.with_capability(&Capability::Commands);
        assert_eq!(cmd_plugins.len(), 1);
        assert_eq!(cmd_plugins[0].info.name, "a");
    }

    #[test]
    fn host_names() {
        let mut host = PluginHost::new();
        host.register(test_info("z")).unwrap();
        host.register(test_info("a")).unwrap();
        let mut names = host.names();
        names.sort();
        assert_eq!(names, vec!["a", "z"]);
    }

    #[test]
    fn host_add_command() {
        let mut host = PluginHost::new();
        host.register(test_info("p")).unwrap();
        host.add_command("p", "my:cmd".into()).unwrap();
        let plugin = host.get("p").unwrap();
        assert_eq!(plugin.commands, vec!["my:cmd"]);
    }

    #[test]
    fn host_add_command_to_missing_fails() {
        let mut host = PluginHost::new();
        let err = host.add_command("nope", "x".into()).unwrap_err();
        assert!(err.contains("not registered"));
    }

    #[test]
    fn host_all_commands() {
        let mut host = PluginHost::new();
        host.register(test_info("p1")).unwrap();
        host.register(test_info("p2")).unwrap();
        host.add_command("p1", "a".into()).unwrap();
        host.add_command("p2", "b".into()).unwrap();
        let cmds = host.all_commands();
        assert_eq!(cmds.len(), 2);
        assert!(cmds.contains(&"a"));
        assert!(cmds.contains(&"b"));
    }

    #[test]
    fn host_running_plugins() {
        let mut host = PluginHost::new();
        host.register(test_info("a")).unwrap();
        host.register(test_info("b")).unwrap();
        host.set_state("a", PluginState::Running).unwrap();
        let running = host.running();
        assert_eq!(running.len(), 1);
        assert_eq!(running[0].info.name, "a");
    }

    #[test]
    fn plugin_state_default_is_registered() {
        assert_eq!(PluginState::default(), PluginState::Registered);
    }

    #[test]
    fn capability_variants() {
        let caps = [
            Capability::Commands,
            Capability::Widgets,
            Capability::Themes,
            Capability::Events,
            Capability::FileSystem,
            Capability::Custom("x".into()),
        ];
        assert_eq!(caps.len(), 6);
    }

    #[test]
    fn custom_capability_equality() {
        let a = Capability::Custom("foo".into());
        let b = Capability::Custom("foo".into());
        let c = Capability::Custom("bar".into());
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
