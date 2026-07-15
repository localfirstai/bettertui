//! napi-rs bindings for Node.js.
//! Simplified API focused on the command-buffer pattern.

use std::collections::HashMap;
use std::sync::Mutex;

use napi_derive::napi;

use crate::VERSION;
use crate::engine::Engine;
use crate::input::{
    EventBus, FocusDirection, FocusManager, FocusTraversal, Key, KeyBinding, KeyEvent, KeyParser, Keymap, Modifiers,
    MouseButton,
};
use crate::protocol::Command;
use crate::render::Renderer;
use crate::scheduler::{FrameStatus, Scheduler};
use crate::text::TextEngine;
use crate::tree::{NodeId, NodeKind, Point};

struct HandleStore<T> {
    next_id: u32,
    objects: HashMap<u32, T>,
}

impl<T> Default for HandleStore<T> {
    fn default() -> Self {
        Self { next_id: 1, objects: HashMap::new() }
    }
}

impl<T> HandleStore<T> {
    fn insert(&mut self, obj: T) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1).max(1);
        self.objects.insert(id, obj);
        id
    }
    fn get(&self, id: u32) -> Option<&T> {
        self.objects.get(&id)
    }
    fn get_mut(&mut self, id: u32) -> Option<&mut T> {
        self.objects.get_mut(&id)
    }
    fn remove(&mut self, id: u32) -> Option<T> {
        self.objects.remove(&id)
    }
}

struct EngineState {
    engine: Engine,
    renderer: Renderer,
    id_map: HashMap<u32, u64>,
}

static ENGINES: std::sync::OnceLock<Mutex<HandleStore<EngineState>>> = std::sync::OnceLock::new();
static EVENT_BUSES: std::sync::OnceLock<Mutex<HandleStore<EventBus>>> = std::sync::OnceLock::new();
static FOCUS_MANAGERS: std::sync::OnceLock<Mutex<HandleStore<FocusManager>>> = std::sync::OnceLock::new();
static TEXT_ENGINES: std::sync::OnceLock<Mutex<HandleStore<TextEngine>>> = std::sync::OnceLock::new();
static SCHEDULERS: std::sync::OnceLock<Mutex<HandleStore<Scheduler>>> = std::sync::OnceLock::new();
static KEYMAPS: std::sync::OnceLock<Mutex<HandleStore<Keymap>>> = std::sync::OnceLock::new();

fn engines() -> &'static Mutex<HandleStore<EngineState>> {
    ENGINES.get_or_init(Default::default)
}
fn event_buses() -> &'static Mutex<HandleStore<EventBus>> {
    EVENT_BUSES.get_or_init(Default::default)
}
fn focus_managers() -> &'static Mutex<HandleStore<FocusManager>> {
    FOCUS_MANAGERS.get_or_init(Default::default)
}
fn text_engines() -> &'static Mutex<HandleStore<TextEngine>> {
    TEXT_ENGINES.get_or_init(Default::default)
}
fn schedulers() -> &'static Mutex<HandleStore<Scheduler>> {
    SCHEDULERS.get_or_init(Default::default)
}
fn keymaps() -> &'static Mutex<HandleStore<Keymap>> {
    KEYMAPS.get_or_init(Default::default)
}

fn node_id(val: u64) -> NodeId {
    unsafe { std::mem::transmute(val) }
}

fn node_id_u64(id: NodeId) -> u64 {
    unsafe { std::mem::transmute(id) }
}

// ─── Global Functions ────────────────────────────────────────────────────────────

#[napi]
pub fn get_version() -> String {
    VERSION.to_string()
}

#[napi(object)]
pub struct TerminalCapabilities {
    pub brand: String,
    pub true_color: bool,
    pub kitty_keyboard: bool,
    pub csi_u: bool,
    pub bracketed_paste: bool,
    pub focus_events: bool,
    pub mouse: bool,
    pub osc52: bool,
    pub osc8: bool,
    pub sync: bool,
    pub sgr_pixel: bool,
    pub underline_color: bool,
    pub strikethrough: bool,
    pub cursor_style: bool,
    pub alternate_scroll: bool,
    pub inline_images: bool,
    pub sixel: bool,
    pub columns: u32,
    pub rows: u32,
}

#[napi]
pub fn detect_capabilities() -> TerminalCapabilities {
    let caps = crate::terminal::global_capabilities();
    let (w, h) = caps.terminal_size();
    let features = caps.features();
    TerminalCapabilities {
        brand: format!("{:?}", caps.brand()),
        true_color: caps.supports_true_color(),
        kitty_keyboard: caps.supports_kitty_keyboard(),
        csi_u: caps.supports_csi_u(),
        bracketed_paste: caps.supports_bracketed_paste(),
        focus_events: caps.supports_focus_events(),
        mouse: caps.input.mouse_modes.normal_mouse,
        osc52: caps.supports_osc52(),
        osc8: caps.supports_osc8(),
        sync: features.synchronized_output,
        sgr_pixel: caps.supports_kitty_graphics(),
        underline_color: features.underline_color,
        strikethrough: features.strikethrough,
        cursor_style: features.cursor_style,
        alternate_scroll: features.alternate_scroll,
        inline_images: caps.supports_iterm_images(),
        sixel: caps.supports_sixel(),
        columns: w as u32,
        rows: h as u32,
    }
}

// ─── Engine Class ────────────────────────────────────────────────────────────────

#[napi]
pub struct NativeEngine {
    handle: u32,
}

#[napi]
impl NativeEngine {
    #[napi(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        let state = EngineState {
            engine: Engine::new(),
            renderer: Renderer::new(width.max(1).min(9999) as u16, height.max(1).min(9999) as u16),
            id_map: HashMap::new(),
        };
        let handle = engines().lock().unwrap().insert(state);
        Self { handle }
    }

    #[napi]
    pub fn process_commands(&self, commands_json: String) -> String {
        let mut store = engines().lock().unwrap();
        let state = match store.get_mut(self.handle) {
            Some(s) => s,
            None => {
                return serde_json::to_string(&serde_json::json!({
                    "success": 0,
                    "errors": ["invalid engine handle"],
                    "id_mappings": []
                }))
                .unwrap_or_default();
            }
        };

        let commands: Vec<CommandJson> = match serde_json::from_str(&commands_json) {
            Ok(c) => c,
            Err(e) => {
                return serde_json::to_string(&serde_json::json!({
                    "success": 0,
                    "errors": [format!("JSON parse error: {}", e)],
                    "id_mappings": []
                }))
                .unwrap_or_default();
            }
        };

        let mut id_mappings: Vec<serde_json::Value> = Vec::new();
        let mut deferred = Vec::new();

        for cmd in commands {
            match cmd {
                CommandJson::CreateNode { id, kind } => {
                    let real_id = state.engine.create_node(parse_node_kind(&kind));
                    let real_u64 = node_id_u64(real_id);
                    state.id_map.insert(id, real_u64);
                    id_mappings.push(serde_json::json!({"temp": id, "real": real_u64}));
                }
                other => {
                    if let Some(rust_cmd) = convert_command(other, &state.id_map) {
                        deferred.push(rust_cmd);
                    }
                }
            }
        }

        let result = state.engine.process_commands(deferred);

        serde_json::to_string(&serde_json::json!({
            "success": result.processed,
            "errors": result.errors.iter().map(|e| format!("{}", e)).collect::<Vec<_>>(),
            "id_mappings": id_mappings
        }))
        .unwrap_or_default()
    }

    #[napi]
    pub fn begin_frame(&self) {
        if let Some(state) = engines().lock().unwrap().get_mut(self.handle) {
            state.engine.begin_frame();
        }
    }

    #[napi]
    pub fn commit_frame(&self) {
        if let Some(state) = engines().lock().unwrap().get_mut(self.handle) {
            state.engine.commit_frame();
        }
    }

    #[napi]
    pub fn render(&self) -> String {
        let mut store = engines().lock().unwrap();
        let state = match store.get_mut(self.handle) {
            Some(s) => s,
            None => return "{}".to_string(),
        };
        let frame = state.renderer.render(state.engine.arena_mut());
        use base64::Engine;
        let data_b64 = base64::engine::general_purpose::STANDARD.encode(&frame.output_data);
        serde_json::to_string(&serde_json::json!({
            "output_data": data_b64,
            "width": frame.width,
            "height": frame.height,
            "dirty_region_count": frame.dirty_regions.len(),
        }))
        .unwrap_or_default()
    }

    #[napi]
    pub fn render_full(&self) -> String {
        let mut store = engines().lock().unwrap();
        let state = match store.get_mut(self.handle) {
            Some(s) => s,
            None => return "{}".to_string(),
        };
        let frame = state.renderer.render_full(state.engine.arena_mut());
        use base64::Engine;
        let data_b64 = base64::engine::general_purpose::STANDARD.encode(&frame.output_data);
        serde_json::to_string(&serde_json::json!({
            "output_data": data_b64,
            "width": frame.width,
            "height": frame.height,
            "dirty_region_count": frame.dirty_regions.len(),
        }))
        .unwrap_or_default()
    }

    #[napi]
    pub fn resize(&self, width: u32, height: u32) {
        if let Some(state) = engines().lock().unwrap().get_mut(self.handle) {
            state.renderer.resize(width.max(1).min(9999) as u16, height.max(1).min(9999) as u16);
        }
    }

    #[napi]
    pub fn node_count(&self) -> u32 {
        engines().lock().unwrap().get(self.handle).map_or(0, |s| s.engine.node_count() as u32)
    }

    #[napi]
    pub fn frame_count(&self) -> u32 {
        engines().lock().unwrap().get(self.handle).map_or(0, |s| s.engine.frame_count() as u32)
    }

    #[napi]
    pub fn print_tree(&self) -> String {
        engines().lock().unwrap().get(self.handle).map_or(String::new(), |s| s.engine.print_tree())
    }

    #[napi]
    pub fn validate(&self) -> bool {
        engines().lock().unwrap().get(self.handle).map_or(false, |s| s.engine.validate().is_ok())
    }

    #[napi]
    pub fn shutdown(&self) {
        if let Some(state) = engines().lock().unwrap().get_mut(self.handle) {
            let _ = state.engine.process_command(Command::Shutdown);
        }
    }
}

impl Drop for NativeEngine {
    fn drop(&mut self) {
        engines().lock().unwrap().remove(self.handle);
    }
}

// ─── EventBus Class ──────────────────────────────────────────────────────────────

#[napi]
pub struct NativeEventBus {
    handle: u32,
}

#[napi]
impl NativeEventBus {
    #[napi(constructor)]
    pub fn new() -> Self {
        let handle = event_buses().lock().unwrap().insert(EventBus::new());
        Self { handle }
    }

    #[napi]
    pub fn push_key(&self, key: String, ctrl: bool, shift: bool, alt: bool) {
        if let Some(bus) = event_buses().lock().unwrap().get_mut(self.handle) {
            bus.push_key(parse_key_str(&key), Modifiers { ctrl, shift, alt, meta: false }, NodeId::default());
        }
    }

    #[napi]
    pub fn push_mouse(&self, button: String, x: u32, y: u32) {
        if let Some(bus) = event_buses().lock().unwrap().get_mut(self.handle) {
            let btn = match button.as_str() {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                "scroll_up" => MouseButton::ScrollUp,
                "scroll_down" => MouseButton::ScrollDown,
                _ => MouseButton::None,
            };
            bus.push_mouse(btn, Point::new(x as u16, y as u16), NodeId::default());
        }
    }

    #[napi]
    pub fn push_resize(&self, width: u32, height: u32, prev_width: u32, prev_height: u32) {
        if let Some(bus) = event_buses().lock().unwrap().get_mut(self.handle) {
            bus.push_resize(width as u16, height as u16, prev_width as u16, prev_height as u16);
        }
    }

    #[napi]
    pub fn drain(&self) -> String {
        event_buses().lock().unwrap().get_mut(self.handle).map_or("[]".to_string(), |bus| {
            let events: Vec<serde_json::Value> = bus.drain().iter().map(event_to_json).collect();
            serde_json::to_string(&events).unwrap_or_default()
        })
    }

    #[napi]
    pub fn len(&self) -> u32 {
        event_buses().lock().unwrap().get(self.handle).map_or(0, |bus| bus.len() as u32)
    }

    #[napi]
    pub fn is_empty(&self) -> bool {
        event_buses().lock().unwrap().get(self.handle).map_or(true, |bus| bus.is_empty())
    }

    #[napi]
    pub fn clear(&self) {
        if let Some(bus) = event_buses().lock().unwrap().get_mut(self.handle) {
            bus.clear();
        }
    }
}

impl Drop for NativeEventBus {
    fn drop(&mut self) {
        event_buses().lock().unwrap().remove(self.handle);
    }
}

// ─── FocusManager Class ──────────────────────────────────────────────────────────

#[napi]
pub struct NativeFocusManager {
    handle: u32,
}

#[napi]
impl NativeFocusManager {
    #[napi(constructor)]
    pub fn new() -> Self {
        let handle = focus_managers().lock().unwrap().insert(FocusManager::new());
        Self { handle }
    }

    #[napi]
    pub fn traverse(&self, direction: String) -> String {
        focus_managers().lock().unwrap().get_mut(self.handle).map_or("null".to_string(), |m| {
            let dir = match direction.as_str() {
                "forward" | "next" => FocusDirection::Forward,
                "backward" | "previous" | "prev" => FocusDirection::Backward,
                "first" => FocusDirection::First,
                "last" => FocusDirection::Last,
                _ => FocusDirection::Forward,
            };
            FocusTraversal::traverse(m, dir).map(|id| node_id_u64(id).to_string()).unwrap_or("null".to_string())
        })
    }
}

impl Drop for NativeFocusManager {
    fn drop(&mut self) {
        focus_managers().lock().unwrap().remove(self.handle);
    }
}

// ─── TextEngine Class ────────────────────────────────────────────────────────────

#[napi]
pub struct NativeTextEngine {
    handle: u32,
}

#[napi]
impl NativeTextEngine {
    #[napi(constructor)]
    pub fn new(text: Option<String>) -> Self {
        let engine = match text {
            Some(t) if !t.is_empty() => TextEngine::with_text(&t),
            _ => TextEngine::default(),
        };
        let handle = text_engines().lock().unwrap().insert(engine);
        Self { handle }
    }

    #[napi]
    pub fn insert_char(&self, ch: String) {
        if let Some(te) = text_engines().lock().unwrap().get_mut(self.handle) {
            if let Some(c) = ch.chars().next() {
                te.insert_char(c);
            }
        }
    }

    #[napi]
    pub fn insert_str(&self, text: String) {
        if let Some(te) = text_engines().lock().unwrap().get_mut(self.handle) {
            te.insert_str(&text);
        }
    }

    #[napi]
    pub fn delete_char(&self) {
        if let Some(te) = text_engines().lock().unwrap().get_mut(self.handle) {
            te.delete_char();
        }
    }

    #[napi]
    pub fn get_text(&self) -> String {
        text_engines().lock().unwrap().get(self.handle).map_or(String::new(), |te| {
            let lines: Vec<String> = (0..te.line_count()).filter_map(|i| te.line(i)).collect();
            lines.join("\n")
        })
    }

    #[napi]
    pub fn clear(&self) {
        if let Some(te) = text_engines().lock().unwrap().get_mut(self.handle) {
            te.clear();
        }
    }

    #[napi]
    pub fn can_undo(&self) -> bool {
        text_engines().lock().unwrap().get(self.handle).map_or(false, |te| te.undo_manager().can_undo())
    }

    #[napi]
    pub fn can_redo(&self) -> bool {
        text_engines().lock().unwrap().get(self.handle).map_or(false, |te| te.undo_manager().can_redo())
    }

    #[napi]
    pub fn undo(&self) -> bool {
        text_engines().lock().unwrap().get_mut(self.handle).map_or(false, |te| te.undo())
    }

    #[napi]
    pub fn redo(&self) -> bool {
        text_engines().lock().unwrap().get_mut(self.handle).map_or(false, |te| te.redo())
    }
}

impl Drop for NativeTextEngine {
    fn drop(&mut self) {
        text_engines().lock().unwrap().remove(self.handle);
    }
}

// ─── Scheduler Class ─────────────────────────────────────────────────────────────

#[napi]
pub struct NativeScheduler {
    handle: u32,
}

#[napi]
impl NativeScheduler {
    #[napi(constructor)]
    pub fn new(fps: Option<u32>) -> Self {
        let s = match fps {
            Some(f) if f > 0 => Scheduler::with_fps(f),
            _ => Scheduler::default(),
        };
        let handle = schedulers().lock().unwrap().insert(s);
        Self { handle }
    }

    #[napi]
    pub fn request_frame(&self) {
        if let Some(s) = schedulers().lock().unwrap().get_mut(self.handle) {
            s.request_frame();
        }
    }

    #[napi]
    pub fn begin_frame(&self) -> bool {
        schedulers().lock().unwrap().get_mut(self.handle).map_or(false, |s| s.begin_frame())
    }

    #[napi]
    pub fn end_frame(&self) {
        if let Some(s) = schedulers().lock().unwrap().get_mut(self.handle) {
            s.end_frame();
        }
    }

    #[napi]
    pub fn is_idle(&self) -> bool {
        schedulers().lock().unwrap().get(self.handle).map_or(true, |s| matches!(s.status(), FrameStatus::Idle))
    }

    #[napi]
    pub fn frame_count(&self) -> u32 {
        schedulers().lock().unwrap().get(self.handle).map_or(0, |s| s.frame_count() as u32)
    }
}

impl Drop for NativeScheduler {
    fn drop(&mut self) {
        schedulers().lock().unwrap().remove(self.handle);
    }
}

// ─── Keymap Class ────────────────────────────────────────────────────────────────

#[napi]
pub struct NativeKeymap {
    handle: u32,
}

#[napi]
impl NativeKeymap {
    #[napi(constructor)]
    pub fn new() -> Self {
        let handle = keymaps().lock().unwrap().insert(Keymap::new());
        Self { handle }
    }

    #[napi]
    pub fn add_binding(
        &self,
        layer: String,
        id: String,
        keys: String,
        command: String,
        description: Option<String>,
        priority: i32,
    ) -> bool {
        keymaps().lock().unwrap().get_mut(self.handle).map_or(false, |km| match KeyParser::parse_sequence(&keys) {
            Ok(seq) => {
                let binding = KeyBinding { id, command, sequence: seq, description, condition: None, enabled: true };
                km.add_binding_to_layer(&layer, binding, priority);
                true
            }
            Err(_) => false,
        })
    }

    #[napi]
    pub fn handle_key(&self, key: String) -> String {
        keymaps().lock().unwrap().get_mut(self.handle).map_or(String::new(), |km| match KeyParser::parse_combo(&key) {
            Ok(combo) => {
                let event = KeyEvent {
                    key: combo.key,
                    modifiers: combo.modifiers,
                    target: NodeId::default(),
                    default_prevented: false,
                    phase: crate::input::EventPhase::Target,
                };
                match km.handle_event(&event) {
                    Some(cmd) => cmd,
                    None => String::new(),
                }
            }
            Err(_) => String::new(),
        })
    }

    #[napi]
    pub fn has_pending(&self) -> bool {
        keymaps().lock().unwrap().get(self.handle).map_or(false, |km| km.has_pending_sequence())
    }

    #[napi]
    pub fn clear_pending(&self) {
        if let Some(km) = keymaps().lock().unwrap().get_mut(self.handle) {
            km.clear_pending_sequence();
        }
    }
}

impl Drop for NativeKeymap {
    fn drop(&mut self) {
        keymaps().lock().unwrap().remove(self.handle);
    }
}

// ─── Helper Functions ────────────────────────────────────────────────────────────

fn parse_key_str(s: &str) -> Key {
    match s {
        "enter" => Key::Enter,
        "escape" | "esc" => Key::Escape,
        "backspace" => Key::Backspace,
        "delete" => Key::Delete,
        "tab" => Key::Tab,
        "space" => Key::Space,
        "arrow_up" | "up" => Key::ArrowUp,
        "arrow_down" | "down" => Key::ArrowDown,
        "arrow_left" | "left" => Key::ArrowLeft,
        "arrow_right" | "right" => Key::ArrowRight,
        "home" => Key::Home,
        "end" => Key::End,
        "page_up" => Key::PageUp,
        "page_down" => Key::PageDown,
        _ if s.starts_with('f') && s.len() <= 3 => s[1..].parse::<u8>().map(Key::F).unwrap_or(Key::Character(' ')),
        _ if s.starts_with("ctrl_") => Key::Ctrl(s.chars().nth(5).unwrap_or(' ')),
        _ if s.starts_with("alt_") => Key::Alt(s.chars().nth(4).unwrap_or(' ')),
        _ => Key::Character(s.chars().next().unwrap_or(' ')),
    }
}

fn event_to_json(e: &crate::input::Event) -> serde_json::Value {
    match e {
        crate::input::Event::Key(ke) => serde_json::json!({
            "type": "key",
            "key": format!("{:?}", ke.key).to_lowercase(),
            "ctrl": ke.modifiers.ctrl,
            "shift": ke.modifiers.shift,
            "alt": ke.modifiers.alt,
        }),
        crate::input::Event::Mouse(me) => serde_json::json!({
            "type": "mouse",
            "button": format!("{:?}", me.button).to_lowercase(),
            "x": me.position.x,
            "y": me.position.y,
        }),
        crate::input::Event::Resize(re) => serde_json::json!({
            "type": "resize",
            "width": re.width,
            "height": re.height,
            "prev_width": re.previous_width,
            "prev_height": re.previous_height,
        }),
        _ => serde_json::json!({"type": "unknown"}),
    }
}

fn parse_node_kind(kind: &str) -> NodeKind {
    match kind.to_lowercase().as_str() {
        "text" => NodeKind::Text,
        "flex" => NodeKind::Flex,
        "input" => NodeKind::Input,
        "list" => NodeKind::List,
        "table" | "datatable" => NodeKind::Table,
        "tree" => NodeKind::Tree,
        "scrollarea" | "scroll" => NodeKind::Scroll,
        "tabs" | "tab" => NodeKind::Tab,
        "modal" => NodeKind::Modal,
        "code" => NodeKind::Code,
        "spacer" => NodeKind::Spacer,
        "separator" => NodeKind::Separator,
        _ => NodeKind::Box,
    }
}

// ─── JSON Command Types ──────────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
#[serde(tag = "type")]
enum CommandJson {
    CreateNode { id: u32, kind: String },
    RemoveNode { id: String },
    AppendChild { parent: String, child: String },
    SetText { id: String, text: String },
    SetBold { id: String, value: bool },
    SetItalic { id: String, value: bool },
    Shutdown,
}

fn convert_command(cj: CommandJson, id_map: &HashMap<u32, u64>) -> Option<Command> {
    let resolve = |s: &str| -> NodeId {
        s.parse::<u32>()
            .ok()
            .and_then(|temp| id_map.get(&temp).copied())
            .map(node_id)
            .unwrap_or_else(|| node_id(s.parse::<u64>().unwrap_or(0)))
    };

    match cj {
        CommandJson::CreateNode { .. } => None, // Handled separately in process_commands
        CommandJson::RemoveNode { id } => Some(Command::RemoveNode { id: resolve(&id) }),
        CommandJson::AppendChild { parent, child } => {
            Some(Command::AppendChild { parent: resolve(&parent), child: resolve(&child) })
        }
        CommandJson::SetText { id, text } => Some(Command::SetText { id: resolve(&id), text }),
        CommandJson::SetBold { id, value } => Some(Command::SetBold { id: resolve(&id), value }),
        CommandJson::SetItalic { id, value } => Some(Command::SetItalic { id: resolve(&id), value }),
        CommandJson::Shutdown => Some(Command::Shutdown),
    }
}
