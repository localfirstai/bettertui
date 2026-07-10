# Plugin API

> The plugin system allows extending BetterTUI with custom widgets, themes, commands, and more.
> Plugins are first-class citizens, not afterthoughts.

## 1. Overview

The plugin system provides extension points at every level of the framework:

```
Application
    ↓
Plugin Registry
    ↓
┌──────────┬──────────┬──────────┬──────────┐
│ Widgets  │ Themes   │ Commands │ Animations│
├──────────┼──────────┼──────────┼──────────┤
│ Layouts  │ Input    │ Render   │ Events   │
│          │ Handlers │ Backends │          │
└──────────┴──────────┴──────────┴──────────┘
```

### 1.1 Why a Plugin System?

BetterTUI cannot anticipate every use case. A plugin system allows:

- **Community contributions:** Third-party widgets, themes, and extensions.
- **Composability:** Applications combine plugins to create rich UIs.
- **Extensibility:** New features can be added without modifying the core.
- **Separation of concern:** Core stays lean; features are opt-in.

## 2. Plugin Architecture

### 2.1 Plugin Trait (Rust)

```rust
pub trait Plugin: Send + Sync {
    /// Plugin name.
    fn name(&self) -> &str;

    /// Plugin version.
    fn version(&self) -> &str;

    /// Initialize the plugin.
    fn init(&self, host: &mut PluginHost) -> Result<(), PluginError>;

    /// Called when the plugin is registered.
    fn on_register(&self, host: &PluginHost) -> Result<(), PluginError> {}

    /// Called when the plugin is unregistered.
    fn on_unregister(&self, host: &PluginHost) -> Result<(), PluginError> {}

    /// Called on each frame tick.
    fn on_tick(&self, host: &mut PluginHost, delta: Duration) {}

    /// Called when the terminal is resized.
    fn on_resize(&self, host: &mut PluginHost, width: u16, height: u16) {}
}
```

### 2.2 Plugin Host

```rust
pub struct PluginHost {
    pub arena: &mut NodeArena,
    pub event_system: &mut EventSystem,
    pub animation_engine: &mut AnimationEngine,
    pub focus_manager: &mut FocusManager,
    pub renderer: &mut Renderer,
    pub registry: &PluginRegistry,
}
```

### 2.3 Plugin Registry

```rust
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
    extensions: HashMap<ExtensionPoint, Vec<Box<dyn Extension>>>,
}
```

## 3. Extension Points

### 3.1 Widget Extension

```rust
pub trait WidgetExtension: Send + Sync {
    fn create_widget(&self, name: &str, props: &HashMap<String, Value>) -> Result<Box<dyn Widget>, PluginError>;
    fn list_widgets(&self) -> Vec<&str>;
}
```

**Usage:**

```rust
pub struct MyWidgetPlugin;

impl WidgetExtension for MyWidgetPlugin {
    fn create_widget(&self, name: &str, props: &HashMap<String, Value>) -> Result<Box<dyn Widget>, PluginError> {
        match name {
            "my-button" => Ok(Box::new(MyButtonWidget::from_props(props)?)),
            "my-modal" => Ok(Box::new(MyModalWidget::from_props(props)?)),
            _ => Err(PluginError::UnknownWidget(name.into())),
        }
    }

    fn list_widgets(&self) -> Vec<&str> {
        vec!["my-button", "my-modal"]
    }
}
```

### 3.2 Theme Extension

```rust
pub trait ThemeExtension: Send + Sync {
    fn create_theme(&self, name: &str, overrides: &HashMap<String, Value>) -> Result<Theme, PluginError>;
    fn list_themes(&self) -> Vec<&str>;
}
```

### 3.3 Command Extension

```rust
pub trait CommandExtension: Send + Sync {
    fn execute(&self, command: &str, args: &HashMap<String, Value>, host: &mut PluginHost) -> Result<Value, PluginError>;
    fn list_commands(&self) -> Vec<&str>;
}
```

**Usage:**

```rust
pub struct GitPlugin;

impl CommandExtension for GitPlugin {
    fn execute(&self, command: &str, args: &HashMap<String, Value>, host: &mut PluginHost) -> Result<Value, PluginError> {
        match command {
            "git.status" => {
                let status = std::process::Command::new("git").arg("status").output()?;
                Ok(Value::String(String::from_utf8(status.stdout)?))
            }
            "git.commit" => {
                let message = args.get("message").and_then(|v| v.as_str()).unwrap_or("");
                std::process::Command::new("git").args(["commit", "-m", message]).output()?;
                Ok(Value::Bool(true))
            }
            _ => Err(PluginError::UnknownCommand(command.into())),
        }
    }

    fn list_commands(&self) -> Vec<&str> {
        vec!["git.status", "git.commit", "git.diff"]
    }
}
```

### 3.4 Animation Extension

```rust
pub trait AnimationExtension: Send + Sync {
    fn create_animation(&self, name: &str, params: &HashMap<String, Value>) -> Result<Animation, PluginError>;
    fn list_animations(&self) -> Vec<&str>;
}
```

### 3.5 Layout Extension

```rust
pub trait LayoutExtension: Send + Sync {
    fn compute_layout(&self, node: &RenderNode, constraints: &Constraints) -> Result<Size, PluginError>;
    fn supported_layouts(&self) -> Vec<&str>;
}
```

### 3.6 Input Handler Extension

```rust
pub trait InputHandlerExtension: Send + Sync {
    fn handle_input(&self, event: &Event, host: &mut PluginHost) -> EventResult;
    fn priority(&self) -> i32;
}
```

### 3.7 Renderer Extension

```rust
pub trait RendererExtension: Send + Sync {
    fn render_node(&self, node: &RenderNode, buffer: &mut FrameBuffer) -> Result<(), PluginError>;
    fn supported_kinds(&self) -> Vec<NodeKind>;
}
```

### 3.8 Event Extension

```rust
pub trait EventExtension: Send + Sync {
    fn handle_event(&self, event: &Event, host: &mut PluginHost) -> EventResult;
    fn event_types(&self) -> Vec<EventType>;
}
```

## 4. Plugin Registration

### 4.1 Rust Side

```rust
impl PluginRegistry {
    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        plugin.on_register(&self.host)?;
        self.plugins.push(plugin);
        Ok(())
    }

    pub fn unregister(&mut self, name: &str) -> Result<(), PluginError> {
        if let Some(pos) = self.plugins.iter().position(|p| p.name() == name) {
            let plugin = self.plugins.remove(pos);
            plugin.on_unregister(&self.host)?;
        }
        Ok(())
    }

    pub fn get_extension<T: 'static>(&self, point: ExtensionPoint) -> Vec<&T> {
        self.extensions.get(&point)
            .map(|exts| exts.iter().filter_map(|e| e.downcast_ref::<T>()).collect())
            .unwrap_or_default()
    }
}
```

### 4.2 TypeScript Side

```typescript
interface Plugin {
  name: string;
  version: string;
  init(host: PluginHost): void;
  onRegister?(host: PluginHost): void;
  onUnregister?(host: PluginHost): void;
  onTick?(host: PluginHost, delta: number): void;
  onResize?(host: PluginHost, width: number, height: number): void;
}

interface PluginHost {
  registerWidget(name: string, factory: WidgetFactory): void;
  registerTheme(name: string, theme: Theme): void;
  registerCommand(name: string, handler: CommandHandler): void;
  registerAnimation(name: string, factory: AnimationFactory): void;
  registerInputHandler(handler: InputHandler, priority: number): void;
  registerRenderer(kind: string, renderer: NodeRenderer): void;
}
```

### 4.3 Plugin Loading

Plugins are loaded at startup:

```rust
pub fn load_plugins(registry: &mut PluginRegistry) -> Result<(), PluginError> {
    // Built-in plugins
    registry.register(Box::new(BuiltinWidgetsPlugin))?;
    registry.register(Box::new(BuiltinThemesPlugin))?;

    // User plugins (from config)
    for plugin_config in load_plugin_config()? {
        let plugin = load_plugin(&plugin_config)?;
        registry.register(plugin)?;
    }

    Ok(())
}
```

## 5. Plugin API Boundaries

### 5.1 What Plugins CAN Do

- Register custom widgets, themes, commands, animations.
- Handle events (with permission).
- Access the node arena (with permission).
- Modify node properties (with permission).
- Trigger re-renders.
- Access the clipboard.
- Access the terminal.

### 5.2 What Plugins CANNOT Do

- Access other plugins' internal state.
- Modify the core rendering pipeline.
- Bypass security restrictions.
- Access the file system (without explicit permission).
- Access the network (without explicit permission).

### 5.3 Security Model

Plugins run in a sandboxed environment:

1. **Capability-based:** Plugins declare required capabilities at registration.
2. **Permission-based:** The host grants or denies capabilities.
3. **Audit trail:** All plugin actions are logged for debugging.

```rust
pub struct PluginCapabilities {
    pub read_arena: bool,
    pub write_arena: bool,
    pub handle_events: bool,
    pub access_clipboard: bool,
    pub access_terminal: bool,
    pub trigger_render: bool,
}
```

## 6. Plugin Communication

### 6.1 Direct Communication

Plugins can communicate via the shared arena:

```rust
// Plugin A writes to a node
arena.get_mut(node_a).unwrap().text = Some("Hello".into());

// Plugin B reads from the same node
let text = arena.get(node_a).unwrap().text.as_ref();
```

### 6.2 Event-Based Communication

Plugins can communicate via custom events:

```rust
// Plugin A emits a custom event
event_system.emit_custom(type_id: 1, payload: data);

// Plugin B handles the custom event
event_system.on_custom(1, |event| { ... });
```

### 6.3 Shared State

Plugins can share state via a common context:

```rust
pub struct SharedState {
    pub data: HashMap<String, Box<dyn Any>>,
}

// Plugin A writes
state.data.insert("key".into(), Box::new(value));

// Plugin B reads
let value = state.data.get("key");
```

## 7. Plugin Lifecycle

### 7.1 Registration

```
1. Plugin is instantiated
2. Plugin.init() is called
3. Plugin registers extensions
4. Plugin.on_register() is called
5. Plugin is ready
```

### 7.2 Active

```
1. Plugin.on_tick() called on each frame
2. Plugin handles events
3. Plugin modifies nodes
4. Plugin triggers re-renders as needed
```

### 7.3 Unregistration

```
1. Plugin.on_unregister() is called
2. Plugin extensions are removed
3. Plugin resources are cleaned up
4. Plugin is dropped
```

## 8. Plugin Discovery

### 8.1 NPM Packages

Plugins are distributed as npm packages:

```
@bettertui/plugin-markdown
@bettertui/plugin-syntax-highlight
@bettertui/plugin-git-status
```

### 8.2 Plugin Manifest

```json
{
  "name": "@bettertui/plugin-markdown",
  "version": "1.0.0",
  "bettertui": {
    "type": "plugin",
    "entry": "dist/index.js",
    "capabilities": ["read_arena", "handle_events"],
    "extensions": {
      "widgets": ["markdown"],
      "commands": ["markdown.render"]
    }
  }
}
```

### 8.3 Auto-Discovery

Plugins are discovered via:

1. **Config file:** `bettertui.config.json` lists plugins.
2. **Package.json:** `bettertui.plugins` field.
3. **Directory scan:** `node_modules/@bettertui/plugin-*`.
4. **Programmatic:** `registry.register(plugin)`.

## 9. Error Handling

### 9.1 Plugin Errors

```rust
pub enum PluginError {
    InitializationError(String),
    RegistrationError(String),
    RuntimeError(String),
    CapabilityError(String),
    UnknownWidget(String),
    UnknownCommand(String),
    UnknownTheme(String),
    UnknownAnimation(String),
}
```

### 9.2 Error Recovery

- **Initialization error:** Plugin is not registered. Error is logged.
- **Runtime error:** Plugin is disabled. Error is logged.
- **Capability error:** Action is denied. Error is logged.

Plugins should never crash the host application. All errors are caught and logged.

## 10. Performance

### 10.1 Plugin Overhead

- Registration: ~10μs per plugin.
- Tick: ~1μs per plugin.
- Event handling: ~0.1μs per event.

### 10.2 Extension Lookup

Extension lookup is O(1) via HashMap.

### 10.3 Plugin Communication

Direct arena access is O(1). Event-based communication is O(n) where n is the number of listeners.

## 11. Future Considerations

### 11.1 Plugin Sandboxing

Run plugins in separate threads or WebAssembly modules for isolation.

### 11.2 Plugin Marketplace

A marketplace for discovering and installing plugins.

### 11.3 Plugin Versioning

Support multiple versions of the same plugin side by side.

### 11.4 Plugin Testing

Provide test utilities for plugin authors:

```rust
pub fn test_plugin(plugin: Box<dyn Plugin>, test: impl FnOnce(&mut PluginHost)) {
    let mut host = create_test_host();
    plugin.init(&mut host).unwrap();
    test(&mut host);
}
```
