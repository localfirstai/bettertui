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
    /// Registered plugins.
    plugins: Vec<Plugin>,
    /// Plugin names for quick lookup.
    index: HashMap<String, usize>,
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHost {
    /// Creates a new PluginHost.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            index: HashMap::new(),
        }
    }

    /// Registers a plugin.
    pub fn register(&mut self, info: PluginInfo) -> Result<(), String> {
        if self.index.contains_key(&info.name) {
            return Err(format!("Plugin '{}' is already registered", info.name));
        }
        let idx = self.plugins.len();
        self.plugins.push(Plugin {
            info: info.clone(),
            state: PluginState::Registered,
            commands: Vec::new(),
        });
        self.index.insert(info.name, idx);
        Ok(())
    }

    /// Unregisters a plugin by name.
    pub fn unregister(&mut self, name: &str) -> Result<(), String> {
        let idx = self
            .index
            .remove(name)
            .ok_or_else(|| format!("Plugin '{name}' is not registered"))?;
        self.plugins.remove(idx);
        // Rebuild index
        self.index.clear();
        for (i, plugin) in self.plugins.iter().enumerate() {
            self.index.insert(plugin.info.name.clone(), i);
        }
        Ok(())
    }

    /// Transitions a plugin to a new state.
    pub fn set_state(&mut self, name: &str, state: PluginState) -> Result<(), String> {
        let idx = self
            .index
            .get(name)
            .ok_or_else(|| format!("Plugin '{name}' is not registered"))?;
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

    /// Returns true if no plugins are registered.
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Returns all plugins with a given capability.
    pub fn with_capability(&self, cap: &Capability) -> Vec<&Plugin> {
        self.plugins
            .iter()
            .filter(|p| p.info.capabilities.contains(cap))
            .collect()
    }

    /// Returns all plugin names.
    pub fn names(&self) -> Vec<&str> {
        self.plugins.iter().map(|p| p.info.name.as_str()).collect()
    }

    /// Registers a command for a plugin.
    pub fn add_command(&mut self, plugin_name: &str, command: String) -> Result<(), String> {
        let idx = self
            .index
            .get(plugin_name)
            .ok_or_else(|| format!("Plugin '{plugin_name}' is not registered"))?;
        self.plugins[*idx].commands.push(command);
        Ok(())
    }

    /// Returns all commands provided by all plugins.
    pub fn all_commands(&self) -> Vec<&str> {
        self.plugins
            .iter()
            .flat_map(|p| p.commands.iter().map(|s| s.as_str()))
            .collect()
    }

    /// Returns running plugins.
    pub fn running(&self) -> Vec<&Plugin> {
        self.plugins
            .iter()
            .filter(|p| p.state == PluginState::Running)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_info(name: &str) -> PluginInfo {
        PluginInfo {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            author: "test".to_string(),
            capabilities: vec![Capability::Commands],
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn register_plugin() {
        let mut host = PluginHost::new();
        assert!(host.register(test_info("p1")).is_ok());
        assert_eq!(host.len(), 1);
    }

    #[test]
    fn register_duplicate() {
        let mut host = PluginHost::new();
        host.register(test_info("p1")).unwrap();
        assert!(host.register(test_info("p1")).is_err());
    }

    #[test]
    fn unregister_plugin() {
        let mut host = PluginHost::new();
        host.register(test_info("p1")).unwrap();
        assert!(host.unregister("p1").is_ok());
        assert!(host.is_empty());
    }

    #[test]
    fn unregister_nonexistent() {
        let mut host = PluginHost::new();
        assert!(host.unregister("p1").is_err());
    }

    #[test]
    fn state_transitions() {
        let mut host = PluginHost::new();
        host.register(test_info("p1")).unwrap();
        assert_eq!(host.state("p1"), Some(PluginState::Registered));
        host.set_state("p1", PluginState::Initialized).unwrap();
        assert_eq!(host.state("p1"), Some(PluginState::Initialized));
        host.set_state("p1", PluginState::Running).unwrap();
        assert_eq!(host.state("p1"), Some(PluginState::Running));
    }

    #[test]
    fn capability_filter() {
        let mut host = PluginHost::new();
        let mut info = test_info("p1");
        info.capabilities.push(Capability::Widgets);
        host.register(info).unwrap();
        host.register(test_info("p2")).unwrap();
        let with_cmds = host.with_capability(&Capability::Commands);
        assert_eq!(with_cmds.len(), 2);
        let with_widgets = host.with_capability(&Capability::Widgets);
        assert_eq!(with_widgets.len(), 1);
        assert_eq!(with_widgets[0].info.name, "p1");
    }

    #[test]
    fn plugin_commands() {
        let mut host = PluginHost::new();
        host.register(test_info("p1")).unwrap();
        host.add_command("p1", "cmd1".into()).unwrap();
        host.add_command("p1", "cmd2".into()).unwrap();
        assert_eq!(host.all_commands().len(), 2);
    }

    #[test]
    fn running_plugins() {
        let mut host = PluginHost::new();
        host.register(test_info("p1")).unwrap();
        host.register(test_info("p2")).unwrap();
        host.set_state("p1", PluginState::Running).unwrap();
        assert_eq!(host.running().len(), 1);
    }

    #[test]
    fn names() {
        let mut host = PluginHost::new();
        host.register(test_info("a")).unwrap();
        host.register(test_info("b")).unwrap();
        let mut names = host.names();
        names.sort();
        assert_eq!(names, vec!["a", "b"]);
    }
}
