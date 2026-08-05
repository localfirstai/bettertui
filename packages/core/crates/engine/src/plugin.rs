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

    /// Lifecycle hook: initialize a `Registered` plugin (→ `Initialized`).
    ///
    /// Unlike raw [`set_state`](Self::set_state), the lifecycle hooks enforce the
    /// legal transition order `Registered → Initialized → Running → Stopped`
    /// (with `Stopped`/`Initialized` re-initializable and `start` allowed from
    /// `Initialized`/`Stopped`). Illegal transitions return an error and leave
    /// the plugin unchanged.
    pub fn initialize(&mut self, name: &str) -> Result<(), String> {
        self.transition(name, &[PluginState::Registered, PluginState::Stopped], PluginState::Initialized, "initialize")
    }

    /// Lifecycle hook: start an `Initialized`/`Stopped` plugin (→ `Running`).
    pub fn start(&mut self, name: &str) -> Result<(), String> {
        self.transition(name, &[PluginState::Initialized, PluginState::Stopped], PluginState::Running, "start")
    }

    /// Lifecycle hook: stop a `Running` plugin (→ `Stopped`).
    pub fn stop(&mut self, name: &str) -> Result<(), String> {
        self.transition(name, &[PluginState::Running], PluginState::Stopped, "stop")
    }

    /// Lifecycle hook: mark a plugin as errored from any state.
    pub fn mark_error(&mut self, name: &str) -> Result<(), String> {
        let idx = *self.index.get(name).ok_or_else(|| format!("Plugin '{name}' is not registered"))?;
        self.plugins[idx].state = PluginState::Error;
        Ok(())
    }

    /// Validates and applies a lifecycle transition.
    fn transition(
        &mut self,
        name: &str,
        allowed_from: &[PluginState],
        to: PluginState,
        action: &str,
    ) -> Result<(), String> {
        let idx = *self.index.get(name).ok_or_else(|| format!("Plugin '{name}' is not registered"))?;
        let current = self.plugins[idx].state;
        if !allowed_from.contains(&current) {
            return Err(format!("cannot {action} plugin '{name}' from state {current:?}"));
        }
        self.plugins[idx].state = to;
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

// ─── Slot-based composition ──────────────────────────────────────────────────

/// How a [`SlotRegistry`] resolves multiple contributions to the same slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SlotMode {
    /// All contributions are kept, ordered by registration (then priority).
    #[default]
    Append,
    /// Only the highest-priority contribution wins (ties: first registered).
    SingleWinner,
    /// The most recent contribution replaces all previous ones.
    Replace,
}

/// One contribution to a slot, owned by a plugin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotEntry<T> {
    /// The plugin that registered this entry.
    pub plugin_id: String,
    /// Higher priority wins under `SingleWinner` and sorts first under `Append`.
    pub priority: i32,
    /// The contributed value (e.g. a node id, widget descriptor, string).
    pub value: T,
}

/// A named composition point that plugins contribute to.
/// Used to compose core UI regions
/// (status bar, header, footer) from multiple plugins.
///
/// Registration returns a token; dropping it is not automatic (call
/// [`remove`](Self::remove)). Mutations set a dirty flag so a host can coalesce
/// recomputation via [`take_dirty`](Self::take_dirty).
#[derive(Debug, Clone)]
pub struct SlotRegistry<T> {
    mode: SlotMode,
    entries: Vec<SlotEntry<T>>,
    next_token: u64,
    tokens: Vec<u64>,
    dirty: bool,
}

impl<T: Clone> Default for SlotRegistry<T> {
    fn default() -> Self {
        Self::new(SlotMode::default())
    }
}

impl<T: Clone> SlotRegistry<T> {
    pub fn new(mode: SlotMode) -> Self {
        Self { mode, entries: Vec::new(), next_token: 1, tokens: Vec::new(), dirty: false }
    }

    /// Registers a contribution and returns a token identifying it. Under
    /// `Replace`, this clears any previous contributions first.
    pub fn register(&mut self, plugin_id: impl Into<String>, priority: i32, value: T) -> u64 {
        if self.mode == SlotMode::Replace {
            self.entries.clear();
            self.tokens.clear();
        }
        let token = self.next_token;
        self.next_token += 1;
        self.entries.push(SlotEntry { plugin_id: plugin_id.into(), priority, value });
        self.tokens.push(token);
        self.dirty = true;
        token
    }

    /// Removes a contribution by token. Returns `true` if one was removed.
    pub fn remove(&mut self, token: u64) -> bool {
        if let Some(pos) = self.tokens.iter().position(|&t| t == token) {
            self.tokens.remove(pos);
            self.entries.remove(pos);
            self.dirty = true;
            true
        } else {
            false
        }
    }

    /// Removes all contributions registered by a plugin. Returns how many.
    pub fn remove_plugin(&mut self, plugin_id: &str) -> usize {
        let before = self.entries.len();
        let mut i = 0;
        while i < self.entries.len() {
            if self.entries[i].plugin_id == plugin_id {
                self.entries.remove(i);
                self.tokens.remove(i);
            } else {
                i += 1;
            }
        }
        let removed = before - self.entries.len();
        if removed > 0 {
            self.dirty = true;
        }
        removed
    }

    /// Resolves the slot to the effective contributions per the [`SlotMode`].
    ///
    /// - `Append`: all entries, sorted by descending priority (stable, so equal
    ///   priorities keep registration order).
    /// - `SingleWinner`: the single highest-priority entry (or empty).
    /// - `Replace`: whatever remains (at most the last registration).
    pub fn resolve(&self) -> Vec<T> {
        match self.mode {
            SlotMode::Append => {
                let mut ordered: Vec<&SlotEntry<T>> = self.entries.iter().collect();
                ordered.sort_by_key(|b| std::cmp::Reverse(b.priority));
                ordered.into_iter().map(|e| e.value.clone()).collect()
            }
            SlotMode::SingleWinner => self
                .entries
                .iter()
                .enumerate()
                .max_by(|(ai, a), (bi, b)| a.priority.cmp(&b.priority).then(bi.cmp(ai)))
                .map(|(_, e)| vec![e.value.clone()])
                .unwrap_or_default(),
            SlotMode::Replace => self.entries.last().map(|e| vec![e.value.clone()]).into_iter().flatten().collect(),
        }
    }

    /// Number of registered contributions.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the slot has no contributions.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns and clears the dirty flag (for coalesced recomputation).
    pub fn take_dirty(&mut self) -> bool {
        std::mem::replace(&mut self.dirty, false)
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

    // ── Lifecycle hooks ──────────────────────────────────────────────────────

    #[test]
    fn lifecycle_happy_path() {
        let mut host = PluginHost::new();
        host.register(test_info("p")).unwrap();
        assert_eq!(host.state("p"), Some(PluginState::Registered));
        host.initialize("p").unwrap();
        assert_eq!(host.state("p"), Some(PluginState::Initialized));
        host.start("p").unwrap();
        assert_eq!(host.state("p"), Some(PluginState::Running));
        host.stop("p").unwrap();
        assert_eq!(host.state("p"), Some(PluginState::Stopped));
        // Stopped plugins can be restarted.
        host.start("p").unwrap();
        assert_eq!(host.state("p"), Some(PluginState::Running));
    }

    #[test]
    fn lifecycle_rejects_illegal_transition() {
        let mut host = PluginHost::new();
        host.register(test_info("p")).unwrap();
        // Cannot start before initializing.
        let err = host.start("p").unwrap_err();
        assert!(err.contains("cannot start"));
        assert_eq!(host.state("p"), Some(PluginState::Registered));
        // Cannot stop something that is not running.
        assert!(host.stop("p").is_err());
    }

    #[test]
    fn lifecycle_mark_error_from_any_state() {
        let mut host = PluginHost::new();
        host.register(test_info("p")).unwrap();
        host.mark_error("p").unwrap();
        assert_eq!(host.state("p"), Some(PluginState::Error));
    }

    // ── Slot registry ────────────────────────────────────────────────────────

    #[test]
    fn slot_append_orders_by_priority() {
        let mut slot: SlotRegistry<&str> = SlotRegistry::new(SlotMode::Append);
        slot.register("a", 0, "low");
        slot.register("b", 10, "high");
        slot.register("c", 5, "mid");
        assert_eq!(slot.resolve(), vec!["high", "mid", "low"]);
        assert_eq!(slot.len(), 3);
    }

    #[test]
    fn slot_single_winner_picks_highest_priority() {
        let mut slot: SlotRegistry<&str> = SlotRegistry::new(SlotMode::SingleWinner);
        slot.register("a", 1, "a");
        slot.register("b", 9, "b");
        slot.register("c", 3, "c");
        assert_eq!(slot.resolve(), vec!["b"]);
    }

    #[test]
    fn slot_single_winner_ties_keep_first() {
        let mut slot: SlotRegistry<&str> = SlotRegistry::new(SlotMode::SingleWinner);
        slot.register("a", 5, "first");
        slot.register("b", 5, "second");
        assert_eq!(slot.resolve(), vec!["first"]);
    }

    #[test]
    fn slot_replace_keeps_last() {
        let mut slot: SlotRegistry<&str> = SlotRegistry::new(SlotMode::Replace);
        slot.register("a", 0, "old");
        slot.register("b", 0, "new");
        assert_eq!(slot.len(), 1);
        assert_eq!(slot.resolve(), vec!["new"]);
    }

    #[test]
    fn slot_remove_by_token_and_plugin() {
        let mut slot: SlotRegistry<&str> = SlotRegistry::new(SlotMode::Append);
        let t = slot.register("a", 0, "x");
        slot.register("a", 0, "y");
        slot.register("b", 0, "z");
        assert!(slot.remove(t));
        assert!(!slot.remove(9999));
        assert_eq!(slot.remove_plugin("a"), 1); // only "y" remains under plugin a
        assert_eq!(slot.resolve(), vec!["z"]);
    }

    #[test]
    fn slot_dirty_flag_coalesces() {
        let mut slot: SlotRegistry<&str> = SlotRegistry::new(SlotMode::Append);
        assert!(!slot.take_dirty());
        slot.register("a", 0, "x");
        slot.register("a", 0, "y");
        assert!(slot.take_dirty(), "registration sets dirty");
        assert!(!slot.take_dirty(), "dirty cleared after take");
    }
}
