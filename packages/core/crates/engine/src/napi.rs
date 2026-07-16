//! napi-rs bindings for Node.js.
//! Uses the recommended wrapper pattern with direct #[napi] on impl methods.

use std::collections::HashMap;
use std::sync::Mutex;

use napi_derive::napi;

use crate::VERSION;
use crate::engine::Engine;
use crate::hit_grid::HitGrid;
use crate::input::{
    EventBus, FocusDirection, FocusManager, FocusTraversal, Key, KeyBinding, KeyEvent, KeyParser, Keymap, Modifiers,
    MouseButton,
};
use crate::protocol::{Command, ScreenMode};
use crate::render::Renderer;
use crate::scheduler::{FrameStatus, Scheduler};
use crate::span_feed::SpanFeed;
use crate::text::TextEngine;
use crate::tree::{NodeId, NodeKind, Point};

fn node_id(val: u64) -> NodeId {
    // SAFETY: NodeId is slotmap::DefaultKey, which is a transparent newtype
    // around a 64-bit value (generational index). The transmute is safe because
    // both types have the same size and layout.
    unsafe { std::mem::transmute(val) }
}

fn node_id_u64(id: NodeId) -> u64 {
    // SAFETY: NodeId is slotmap::DefaultKey, which is a transparent newtype
    // around a 64-bit value. The transmute is safe because both types have
    // the same size and layout.
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

// ─── Engine Class (Wrapper Pattern) ──────────────────────────────────────────────

struct EngineState {
    engine: Engine,
    renderer: Renderer,
    id_map: HashMap<u32, u64>,
}

#[napi]
pub struct NativeEngine {
    state: Mutex<EngineState>,
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
        Self { state: Mutex::new(state) }
    }

    #[napi]
    pub fn process_commands(&self, commands_json: String) -> String {
        let mut state = match self.state.lock() {
            Ok(s) => s,
            Err(_) => {
                return serde_json::to_string(&serde_json::json!({
                    "success": 0,
                    "errors": ["engine lock poisoned"],
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
        if let Ok(mut state) = self.state.lock() {
            state.engine.begin_frame();
        }
    }

    #[napi]
    pub fn commit_frame(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.engine.commit_frame();
        }
    }

    #[napi]
    pub fn render(&self) -> String {
        let mut state = match self.state.lock() {
            Ok(s) => s,
            Err(_) => return "{}".to_string(),
        };
        let EngineState { engine, renderer, id_map: _ } = &mut *state;
        let frame = renderer.render(engine.arena_mut());
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
        let mut state = match self.state.lock() {
            Ok(s) => s,
            Err(_) => return "{}".to_string(),
        };
        let EngineState { engine, renderer, id_map: _ } = &mut *state;
        let frame = renderer.render_full(engine.arena_mut());
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
    pub fn set_screen_mode(&self, mode: String, footer_height: Option<u32>) {
        if let Ok(mut state) = self.state.lock() {
            let screen_mode = match mode.as_str() {
                "split-footer" => {
                    ScreenMode::SplitFooter { height: footer_height.unwrap_or(0).min(u16::MAX as u32) as u16 }
                }
                "main-screen" => ScreenMode::MainScreen,
                _ => ScreenMode::AlternateScreen,
            };
            state.renderer.set_screen_mode(screen_mode);
        }
    }

    #[napi]
    pub fn resize(&self, width: u32, height: u32) {
        if let Ok(mut state) = self.state.lock() {
            state.renderer.resize(width.max(1).min(9999) as u16, height.max(1).min(9999) as u16);
        }
    }

    #[napi]
    pub fn node_count(&self) -> u32 {
        self.state.lock().map_or(0, |s| s.engine.node_count() as u32)
    }

    #[napi]
    pub fn frame_count(&self) -> u32 {
        self.state.lock().map_or(0, |s| s.engine.frame_count() as u32)
    }

    #[napi]
    pub fn print_tree(&self) -> String {
        self.state.lock().map_or(String::new(), |s| s.engine.print_tree())
    }

    #[napi]
    pub fn validate(&self) -> bool {
        self.state.lock().map_or(false, |s| s.engine.validate().is_ok())
    }

    #[napi]
    pub fn shutdown(&self) {
        if let Ok(mut state) = self.state.lock() {
            let _ = state.engine.process_command(Command::Shutdown);
        }
    }

    #[napi]
    pub fn root(&self) -> i64 {
        self.state.lock().map_or(0, |s| node_id_u64(s.engine.arena().root()) as i64)
    }

    #[napi]
    pub fn create_node(&self, kind: String) -> i64 {
        let mut state = match self.state.lock() {
            Ok(s) => s,
            Err(_) => return 0,
        };
        let node_kind = parse_node_kind(&kind);
        let id = state.engine.create_node(node_kind);
        node_id_u64(id) as i64
    }

    #[napi]
    pub fn append_child(&self, parent: i64, child: i64) -> bool {
        let mut state = match self.state.lock() {
            Ok(s) => s,
            Err(_) => return false,
        };
        state.engine.append_child(node_id(parent as u64), node_id(child as u64)).is_ok()
    }

    #[napi]
    pub fn remove_node(&self, id: i64) {
        if let Ok(mut state) = self.state.lock() {
            state.engine.remove_node(node_id(id as u64));
        }
    }

    #[napi]
    pub fn set_text(&self, id: i64, text: String) {
        if let Ok(mut state) = self.state.lock() {
            state.engine.set_text(node_id(id as u64), text);
        }
    }

    // ─── Hit Grid Methods ───────────────────────────────────────────

    #[napi]
    pub fn hit_grid_check(&self, x: u32, y: u32) -> i64 {
        self.state.lock().map_or(0, |s| s.renderer.hit_grid_check(x, y) as i64)
    }

    #[napi]
    pub fn hit_grid_is_dirty(&self) -> bool {
        self.state.lock().map_or(false, |s| s.renderer.hit_grid_dirty())
    }

    #[napi]
    pub fn hit_grid_clear_current(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.renderer.hit_grid_clear_current();
        }
    }

    #[napi]
    pub fn hit_grid_push_scissor(&self, x: i32, y: i32, width: u32, height: u32) {
        if let Ok(mut state) = self.state.lock() {
            state.renderer.hit_grid_push_scissor(x, y, width, height);
        }
    }

    #[napi]
    pub fn hit_grid_pop_scissor(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.renderer.hit_grid_pop_scissor();
        }
    }

    #[napi]
    pub fn hit_grid_add_current_clipped(&self, x: u32, y: u32, width: u32, height: u32, id: i64) {
        if let Ok(mut state) = self.state.lock() {
            let x = x.min(u16::MAX as u32) as u16;
            let y = y.min(u16::MAX as u32) as u16;
            let width = width.min(u16::MAX as u32) as u16;
            let height = height.min(u16::MAX as u32) as u16;
            state.renderer.hit_grid_add_current_clipped(x, y, width, height, id as u64);
        }
    }

    #[napi]
    pub fn hit_grid_dump(&self) -> String {
        self.state.lock().map_or(String::new(), |s| s.renderer.hit_grid_dump())
    }
}

// ─── EventBus Class (Wrapper Pattern) ─────────────────────────────────────────────

#[napi]
pub struct NativeEventBus {
    bus: Mutex<EventBus>,
}

#[napi]
impl NativeEventBus {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self { bus: Mutex::new(EventBus::new()) }
    }

    #[napi]
    pub fn push_key(&self, key: String, ctrl: bool, shift: bool, alt: bool) {
        if let Ok(mut bus) = self.bus.lock() {
            bus.push_key(parse_key_str(&key), Modifiers { ctrl, shift, alt, meta: false }, NodeId::default());
        }
    }

    #[napi]
    pub fn push_mouse(&self, button: String, x: u32, y: u32) {
        if let Ok(mut bus) = self.bus.lock() {
            let btn = match button.as_str() {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                "scroll_up" => MouseButton::ScrollUp,
                "scroll_down" => MouseButton::ScrollDown,
                _ => MouseButton::None,
            };
            bus.push_mouse(
                btn,
                Point::new(x.min(u16::MAX as u32) as u16, y.min(u16::MAX as u32) as u16),
                NodeId::default(),
            );
        }
    }

    #[napi]
    pub fn push_resize(&self, width: u32, height: u32, prev_width: u32, prev_height: u32) {
        if let Ok(mut bus) = self.bus.lock() {
            bus.push_resize(
                width.min(u16::MAX as u32) as u16,
                height.min(u16::MAX as u32) as u16,
                prev_width.min(u16::MAX as u32) as u16,
                prev_height.min(u16::MAX as u32) as u16,
            );
        }
    }

    #[napi]
    pub fn drain(&self) -> String {
        self.bus.lock().map_or("[]".to_string(), |mut bus| {
            let events: Vec<serde_json::Value> = bus.drain().iter().map(event_to_json).collect();
            serde_json::to_string(&events).unwrap_or_default()
        })
    }

    #[napi]
    pub fn len(&self) -> u32 {
        self.bus.lock().map_or(0, |bus| bus.len() as u32)
    }

    #[napi]
    pub fn is_empty(&self) -> bool {
        self.bus.lock().map_or(true, |bus| bus.is_empty())
    }

    #[napi]
    pub fn clear(&self) {
        if let Ok(mut bus) = self.bus.lock() {
            bus.clear();
        }
    }
}

// ─── FocusManager Class (Wrapper Pattern) ─────────────────────────────────────────

#[napi]
pub struct NativeFocusManager {
    manager: Mutex<FocusManager>,
}

#[napi]
impl NativeFocusManager {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self { manager: Mutex::new(FocusManager::new()) }
    }

    #[napi]
    pub fn traverse(&self, direction: String) -> String {
        self.manager.lock().map_or("null".to_string(), |mut m| {
            let dir = match direction.as_str() {
                "forward" | "next" => FocusDirection::Forward,
                "backward" | "previous" | "prev" => FocusDirection::Backward,
                "first" => FocusDirection::First,
                "last" => FocusDirection::Last,
                _ => FocusDirection::Forward,
            };
            FocusTraversal::traverse(&mut m, dir).map(|id| node_id_u64(id).to_string()).unwrap_or("null".to_string())
        })
    }
}

// ─── TextEngine Class (Wrapper Pattern) ───────────────────────────────────────────

#[napi]
pub struct NativeTextEngine {
    engine: Mutex<TextEngine>,
}

#[napi]
impl NativeTextEngine {
    #[napi(constructor)]
    pub fn new(text: Option<String>) -> Self {
        let engine = match text {
            Some(t) if !t.is_empty() => TextEngine::with_text(&t),
            _ => TextEngine::default(),
        };
        Self { engine: Mutex::new(engine) }
    }

    #[napi]
    pub fn insert_char(&self, ch: String) {
        if let Ok(mut te) = self.engine.lock() {
            if let Some(c) = ch.chars().next() {
                te.insert_char(c);
            }
        }
    }

    #[napi]
    pub fn insert_str(&self, text: String) {
        if let Ok(mut te) = self.engine.lock() {
            te.insert_str(&text);
        }
    }

    #[napi]
    pub fn delete_char(&self) {
        if let Ok(mut te) = self.engine.lock() {
            te.delete_char();
        }
    }

    #[napi]
    pub fn get_text(&self) -> String {
        self.engine.lock().map_or(String::new(), |te| {
            let lines: Vec<String> = (0..te.line_count()).filter_map(|i| te.line(i)).collect();
            lines.join("\n")
        })
    }

    #[napi]
    pub fn clear(&self) {
        if let Ok(mut te) = self.engine.lock() {
            te.clear();
        }
    }

    #[napi]
    pub fn can_undo(&self) -> bool {
        self.engine.lock().map_or(false, |te| te.undo_manager().can_undo())
    }

    #[napi]
    pub fn can_redo(&self) -> bool {
        self.engine.lock().map_or(false, |te| te.undo_manager().can_redo())
    }

    #[napi]
    pub fn undo(&self) -> bool {
        self.engine.lock().map_or(false, |mut te| te.undo())
    }

    #[napi]
    pub fn redo(&self) -> bool {
        self.engine.lock().map_or(false, |mut te| te.redo())
    }
}

// ─── Scheduler Class (Wrapper Pattern) ────────────────────────────────────────────

#[napi]
pub struct NativeScheduler {
    scheduler: Mutex<Scheduler>,
}

#[napi]
impl NativeScheduler {
    #[napi(constructor)]
    pub fn new(fps: Option<u32>) -> Self {
        let s = match fps {
            Some(f) if f > 0 => Scheduler::with_fps(f),
            _ => Scheduler::default(),
        };
        Self { scheduler: Mutex::new(s) }
    }

    #[napi]
    pub fn request_frame(&self) {
        if let Ok(mut s) = self.scheduler.lock() {
            s.request_frame();
        }
    }

    #[napi]
    pub fn begin_frame(&self) -> bool {
        self.scheduler.lock().map_or(false, |mut s| s.begin_frame())
    }

    #[napi]
    pub fn end_frame(&self) {
        if let Ok(mut s) = self.scheduler.lock() {
            s.end_frame();
        }
    }

    #[napi]
    pub fn is_idle(&self) -> bool {
        self.scheduler.lock().map_or(true, |s| matches!(s.status(), FrameStatus::Idle))
    }

    #[napi]
    pub fn frame_count(&self) -> u32 {
        self.scheduler.lock().map_or(0, |s| s.frame_count() as u32)
    }
}

// ─── SpanFeed Class (Wrapper Pattern) ──────────────────────────────────────────────

// ─── NativeSpanFeed ───────────────────────────────────────────────────────────────

// ─── HitGrid Class (Wrapper Pattern) ───────────────────────────────────────────────

#[napi]
pub struct NativeHitGrid {
    grid: Mutex<HitGrid>,
}

#[napi]
impl NativeHitGrid {
    #[napi(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        Self { grid: Mutex::new(HitGrid::new(width, height)) }
    }

    #[napi]
    pub fn resize(&self, width: u32, height: u32) {
        if let Ok(mut g) = self.grid.lock() {
            g.resize(width, height);
        }
    }

    #[napi]
    pub fn add(&self, x: i32, y: i32, width: u32, height: u32, id: f64) {
        if let Ok(mut g) = self.grid.lock() {
            g.add(x, y, width, height, id as u64);
        }
    }

    #[napi]
    pub fn check(&self, x: u32, y: u32) -> f64 {
        self.grid.lock().map_or(0.0, |g| g.check(x, y) as f64)
    }

    #[napi]
    pub fn clear_next(&self) {
        if let Ok(mut g) = self.grid.lock() {
            g.clear_next();
        }
    }

    #[napi]
    pub fn clear_current(&self) {
        if let Ok(mut g) = self.grid.lock() {
            g.clear_current();
        }
    }

    #[napi]
    pub fn swap(&self) -> bool {
        self.grid.lock().map_or(false, |mut g| g.swap())
    }

    #[napi]
    pub fn is_dirty(&self) -> bool {
        self.grid.lock().map_or(false, |g| g.is_dirty())
    }

    #[napi]
    pub fn dimensions(&self) -> String {
        self.grid.lock().map_or("[]".to_string(), |g| {
            let (w, h) = g.dimensions();
            format!("[{}, {}]", w, h)
        })
    }

    #[napi]
    pub fn push_scissor(&self, x: i32, y: i32, width: u32, height: u32) {
        if let Ok(mut g) = self.grid.lock() {
            g.push_scissor(x, y, width, height);
        }
    }

    #[napi]
    pub fn pop_scissor(&self) {
        if let Ok(mut g) = self.grid.lock() {
            g.pop_scissor();
        }
    }

    #[napi]
    pub fn clear_scissors(&self) {
        if let Ok(mut g) = self.grid.lock() {
            g.clear_scissors();
        }
    }
}

// ─── Keymap Class (Wrapper Pattern) ───────────────────────────────────────────────

#[napi]
pub struct NativeKeymap {
    keymap: Mutex<Keymap>,
}

#[napi]
impl NativeKeymap {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self { keymap: Mutex::new(Keymap::new()) }
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
        self.keymap.lock().map_or(false, |mut km| match KeyParser::parse_sequence(&keys) {
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
        self.keymap.lock().map_or(String::new(), |mut km| match KeyParser::parse_combo(&key) {
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
        self.keymap.lock().map_or(false, |km| km.has_pending_sequence())
    }

    #[napi]
    pub fn clear_pending(&self) {
        if let Ok(mut km) = self.keymap.lock() {
            km.clear_pending_sequence();
        }
    }
}

// ─── NativeSpanFeed Class (Wrapper Pattern) ─────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone)]
pub struct NativeSpanFeedOptions {
    pub chunk_size: u32,
    pub initial_chunks: u32,
    pub max_bytes: f64,
    pub growth_policy: u8,
    pub auto_commit_on_full: u8,
    pub span_queue_capacity: u32,
}

impl Default for NativeSpanFeedOptions {
    fn default() -> Self {
        Self {
            chunk_size: 65536,
            initial_chunks: 2,
            max_bytes: 0.0,
            growth_policy: 0,
            auto_commit_on_full: 1,
            span_queue_capacity: 4096,
        }
    }
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct NativeSpanFeedStats {
    pub bytes_written: f64,
    pub spans_committed: f64,
    pub chunks: u32,
    pub pending_spans: u32,
}

#[napi]
pub struct NativeSpanFeed {
    inner: std::sync::Mutex<crate::span_feed::SpanFeed>,
}

#[napi]
impl NativeSpanFeed {
    #[napi(constructor)]
    pub fn new(options: Option<NativeSpanFeedOptions>) -> Self {
        match options {
            Some(opts) => Self {
                inner: std::sync::Mutex::new(crate::span_feed::SpanFeed::with_options(
                    crate::span_feed::SpanFeedOptions {
                        chunk_size: opts.chunk_size,
                        initial_chunks: opts.initial_chunks,
                        max_bytes: opts.max_bytes as u64,
                        growth_policy: if opts.growth_policy == 0 {
                            crate::span_feed::GrowthPolicy::Grow
                        } else {
                            crate::span_feed::GrowthPolicy::Block
                        },
                        auto_commit_on_full: opts.auto_commit_on_full != 0,
                        span_queue_capacity: opts.span_queue_capacity,
                    },
                )),
            },
            None => Self { inner: std::sync::Mutex::new(crate::span_feed::SpanFeed::new()) },
        }
    }

    #[napi]
    pub fn write(&self, data: napi::bindgen_prelude::Buffer) -> u32 {
        if let Ok(mut feed) = self.inner.lock() { feed.write(&data) as u32 } else { 0 }
    }

    #[napi]
    pub fn drain_spans(&self, out: napi::bindgen_prelude::Buffer) -> u32 {
        if let Ok(mut feed) = self.inner.lock() {
            // Safety: the buffer must be big enough for SpanInfo structs
            let capacity = out.len() / std::mem::size_of::<crate::span_feed::SpanInfo>();
            let mut buf: Vec<crate::span_feed::SpanInfo> = Vec::with_capacity(capacity);
            let count = feed.drain_spans(&mut buf);
            // Copy span info into the output buffer as raw bytes
            let dst = out.as_ptr() as *mut crate::span_feed::SpanInfo;
            // SAFETY: dst is a mutable pointer to the JS-allocated buffer. The buffer
            // capacity was checked above (capacity >= count). Both src and dst are
            // properly aligned for SpanInfo. count items are copied, which is within bounds.
            unsafe {
                std::ptr::copy_nonoverlapping(buf.as_ptr(), dst, count as usize);
            }
            count
        } else {
            0
        }
    }

    #[napi]
    pub fn close(&self) {
        if let Ok(mut feed) = self.inner.lock() {
            feed.close();
        }
    }

    #[napi]
    pub fn reset(&self) {
        if let Ok(mut feed) = self.inner.lock() {
            feed.reset();
        }
    }

    #[napi]
    pub fn pending_spans(&self) -> u32 {
        self.inner.lock().map_or(0, |f| f.pending_spans())
    }

    #[napi]
    pub fn pending_bytes(&self) -> u32 {
        self.inner.lock().map_or(0, |f| f.pending_bytes())
    }

    #[napi]
    pub fn is_closed(&self) -> bool {
        self.inner.lock().map_or(true, |f| f.is_closed())
    }

    #[napi]
    pub fn is_backpressured(&self) -> bool {
        self.inner.lock().map_or(true, |f| f.is_backpressured())
    }

    #[napi]
    pub fn stats(&self) -> NativeSpanFeedStats {
        self.inner.lock().map_or(
            NativeSpanFeedStats { bytes_written: 0.0, spans_committed: 0.0, chunks: 0, pending_spans: 0 },
            |f| NativeSpanFeedStats {
                bytes_written: f.bytes_written() as f64,
                spans_committed: f.spans_committed() as f64,
                chunks: f.chunk_count(),
                pending_spans: f.pending_spans(),
            },
        )
    }

    #[napi]
    pub fn mark_consumed(&self, chunk_index: u32) {
        if let Ok(mut feed) = self.inner.lock() {
            feed.mark_consumed(chunk_index);
        }
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
    // Tree commands
    CreateNode { id: u32, kind: String },
    RemoveNode { id: String },
    AppendChild { parent: String, child: String },
    InsertBefore { reference: String, child: String },
    MoveNode { node: String, new_parent: String },
    ReplaceNode { old: String, new: String },
    DetachNode { id: String },
    // Style commands
    SetText { id: String, text: String },
    SetStyle { id: String, style_json: String },
    SetForeground { id: String, color: String },
    SetBackground { id: String, color: String },
    SetBold { id: String, value: bool },
    SetItalic { id: String, value: bool },
    SetUnderline { id: String, value: bool },
    SetStrikethrough { id: String, value: bool },
    SetDim { id: String, value: bool },
    SetInverse { id: String, value: bool },
    SetHidden { id: String, value: bool },
    // Layout commands
    SetFlexDirection { id: String, value: String },
    SetFlexWrap { id: String, value: String },
    SetJustifyContent { id: String, value: String },
    SetAlignItems { id: String, value: String },
    SetAlignSelf { id: String, value: String },
    SetWidth { id: String, value: String },
    SetHeight { id: String, value: String },
    SetMinWidth { id: String, value: String },
    SetMinHeight { id: String, value: String },
    SetMaxWidth { id: String, value: String },
    SetMaxHeight { id: String, value: String },
    SetPadding { id: String, top: f32, right: f32, bottom: f32, left: f32 },
    SetMargin { id: String, top: f32, right: f32, bottom: f32, left: f32 },
    SetGap { id: String, width: f32, height: f32 },
    SetFlexGrow { id: String, value: f32 },
    SetFlexShrink { id: String, value: f32 },
    SetFlexBasis { id: String, value: String },
    SetPosition { id: String, value: String },
    SetInset { id: String, top: f32, right: f32, bottom: f32, left: f32 },
    // Content commands
    SetAttribute { id: String, key: String, value: String },
    RemoveAttribute { id: String, key: String },
    // Visibility commands
    SetDisplay { id: String, value: String },
    SetOpacity { id: String, value: f32 },
    SetClip { id: String, value: bool },
    // Transform commands
    SetTranslateX { id: String, value: i32 },
    SetTranslateY { id: String, value: i32 },
    SetZIndex { id: String, value: i32 },
    // Overflow commands
    SetOverflow { id: String, value: String },
    // Focus commands
    FocusNode { id: String },
    BlurNode { id: String },
    SetTabIndex { id: String, value: i32 },
    // Frame commands
    BeginFrame { frame_id: u64 },
    CommitFrame { frame_id: u64 },
    Invalidate { id: String },
    // Screen commands
    SetScreenMode { mode: String, footer_height: Option<u32> },
    // Lifecycle
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
        CommandJson::CreateNode { .. } => None,
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
