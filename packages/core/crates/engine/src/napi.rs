//! napi-rs bindings for Node.js.
//! Uses the recommended wrapper pattern with direct #[napi] on impl methods.

use std::collections::HashMap;
use std::sync::Mutex;

use napi_derive::napi;

use crate::VERSION;
use crate::animation::{Animation, Easing, Timeline, Tween};
use crate::engine::Engine;
use crate::hit_grid::HitGrid;
// use crate::span_feed::SpanFeed;
use crate::event_bus::{EventQueue, Key, KeyEvent, Modifiers, MouseButton};
use crate::input::{FocusDirection, FocusManager, FocusTraversal, KeyBinding, KeyParser, Keymap};
use crate::logger::{Level, Logger, LoggerConfig, ModuleFilter};
use crate::plugin::{Capability, PluginHost, PluginInfo, PluginState, SlotMode, SlotRegistry};
use crate::protocol::{Command, ScreenMode};
use crate::render::Renderer;
use crate::scheduler::{FrameStatus, Scheduler};
use crate::text::TextEngine;
use crate::theme::Theme;
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

/// Maps a selection name (`clipboard`/`primary`/`secondary`/`tertiary`, or the
/// short `c`/`p`/`s`/`q`) to a [`ClipboardSelection`]. Defaults to `Clipboard`.
fn clipboard_selection_from_str(sel: &str) -> crate::ansi::ClipboardSelection {
    use crate::ansi::ClipboardSelection;
    match sel {
        "primary" | "p" => ClipboardSelection::Primary,
        "secondary" | "s" => ClipboardSelection::Secondary,
        "tertiary" | "q" => ClipboardSelection::Tertiary,
        _ => ClipboardSelection::Clipboard,
    }
}

/// Builds the OSC 52 escape sequence that sets the terminal clipboard to `text`.
///
/// `selection` is one of `clipboard`/`primary`/`secondary`/`tertiary`. The
/// caller writes the returned bytes to the terminal (stdout).
#[napi]
pub fn clipboard_set_sequence(selection: String, text: String) -> Vec<u8> {
    crate::ansi::ClipboardData::set_sequence(clipboard_selection_from_str(&selection), &text)
}

/// Builds the OSC 52 *query* sequence asking the terminal to report the current
/// clipboard contents. The response returns as an inbound OSC 52 that
/// [`clipboard_decode`] can decode.
#[napi]
pub fn clipboard_query_sequence(selection: String) -> Vec<u8> {
    crate::ansi::ClipboardData::query_sequence(clipboard_selection_from_str(&selection))
}

/// Decodes a base64 OSC 52 clipboard payload into UTF-8 text. Returns `null`
/// for the `?` query marker or invalid base64/UTF-8.
#[napi]
pub fn clipboard_decode(payload: String) -> Option<String> {
    use crate::ansi::{ClipboardData, ClipboardSelection};
    let data = ClipboardData { data: payload, selection: ClipboardSelection::Clipboard };
    data.decoded()
}

// ─── Graphics protocols (Kitty / Sixel / iTerm2) ─────────────────────────────

/// Maps a format string (`rgb`/`rgba`/`png`) to an [`ImageFormat`].
fn image_format_from_str(fmt: &str) -> crate::graphics_protocol::ImageFormat {
    use crate::graphics_protocol::ImageFormat;
    match fmt {
        "rgba" => ImageFormat::Rgba,
        "png" => ImageFormat::Png,
        _ => ImageFormat::Rgb,
    }
}

/// Builds a Kitty graphics-protocol sequence transmitting+displaying `data`
/// (raw `rgb`/`rgba` pixels or `png` bytes) with the given numeric `id`.
#[napi]
pub fn graphics_kitty_write(
    format: String,
    width: u32,
    height: u32,
    data: napi::bindgen_prelude::Buffer,
    id: u32,
) -> napi::bindgen_prelude::Buffer {
    let image = crate::graphics_protocol::GraphicsImage {
        format: image_format_from_str(&format),
        width,
        height,
        data: data.as_ref(),
    };
    crate::graphics_protocol::kitty_write(&image, id).into()
}

/// Builds the Kitty sequence deleting image `id`.
#[napi]
pub fn graphics_kitty_delete(id: u32) -> napi::bindgen_prelude::Buffer {
    crate::graphics_protocol::kitty_delete(id).into()
}

/// Builds the Kitty sequence deleting all transmitted images.
#[napi]
pub fn graphics_kitty_delete_all() -> napi::bindgen_prelude::Buffer {
    crate::graphics_protocol::kitty_delete_all().into()
}

/// Builds a Sixel sequence for a raw `rgb`/`rgba` image (empty for `png`).
#[napi]
pub fn graphics_sixel_write(
    format: String,
    width: u32,
    height: u32,
    data: napi::bindgen_prelude::Buffer,
) -> napi::bindgen_prelude::Buffer {
    let image = crate::graphics_protocol::GraphicsImage {
        format: image_format_from_str(&format),
        width,
        height,
        data: data.as_ref(),
    };
    crate::graphics_protocol::sixel_write(&image).into()
}

/// Builds an iTerm2 inline-image sequence for `file_bytes` (e.g. PNG file data).
#[napi]
pub fn graphics_iterm_write(
    file_bytes: napi::bindgen_prelude::Buffer,
    name: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
) -> napi::bindgen_prelude::Buffer {
    crate::graphics_protocol::iterm_write(file_bytes.as_ref(), name.as_deref(), width, height).into()
}

/// Builds the probe sequence(s) detecting which graphics protocols the terminal
/// supports (Kitty query + DA1 for Sixel).
#[napi]
pub fn graphics_query() -> napi::bindgen_prelude::Buffer {
    crate::graphics_protocol::graphics_query().into()
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
        let start = std::time::Instant::now();
        let frame = renderer.render(engine.arena_mut());
        #[cfg(feature = "diagnostics")]
        if let Some(d) = Logger::diagnostics() {
            d.inc_render_calls();
            d.add_render_bytes(frame.output_data.len() as u64);
            d.record_frame_time(start.elapsed());
        }
        #[cfg(not(feature = "diagnostics"))]
        let _ = start;
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
        let start = std::time::Instant::now();
        let frame = renderer.render_full(engine.arena_mut());
        #[cfg(feature = "diagnostics")]
        if let Some(d) = Logger::diagnostics() {
            d.inc_render_calls();
            d.add_render_bytes(frame.output_data.len() as u64);
            d.record_frame_time(start.elapsed());
        }
        #[cfg(not(feature = "diagnostics"))]
        let _ = start;
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
    pub fn set_style(&self, id: i64, style_json: String) {
        if let Ok(mut state) = self.state.lock() {
            if let Ok(style) = serde_json::from_str::<StyleJson>(&style_json) {
                state.engine.set_style(node_id(id as u64), style.into_style());
            }
        }
    }

    #[napi]
    pub fn set_layout(&self, id: i64, layout_json: String) {
        if let Ok(mut state) = self.state.lock() {
            if let Ok(layout) = serde_json::from_str::<LayoutJson>(&layout_json) {
                state.engine.set_layout(node_id(id as u64), layout.into_layout());
            }
        }
    }

    #[napi]
    pub fn get_node(&self, id: i64) -> String {
        self.state.lock().map_or("{}".to_string(), |s| {
            s.engine.get_node(node_id(id as u64)).map_or("null".to_string(), |node| {
                serde_json::to_string(&serde_json::json!({
                    "kind": node.kind.name(),
                    "text": node.text,
                    "has_children": !node.children.is_empty(),
                    "child_count": node.children.len(),
                }))
                .unwrap_or_default()
            })
        })
    }

    #[napi]
    pub fn tree_summary(&self) -> String {
        self.state.lock().map_or(String::new(), |s| s.engine.tree_summary())
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

    /// Inserts `child` immediately before `before` in the tree.
    /// The parent is inferred from `before`'s current parent — this is the fast
    /// path used by the React reconciler to avoid the JSON command round-trip.
    #[napi]
    pub fn insert_before(&self, before: i64, child: i64) -> bool {
        let mut state = match self.state.lock() {
            Ok(s) => s,
            Err(_) => return false,
        };
        state.engine.insert_before(node_id(before as u64), node_id(child as u64)).is_ok()
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

// ─── EventQueue Class (Wrapper Pattern) ─────────────────────────────────────────────

#[napi]
pub struct NativeEventBus {
    bus: Mutex<EventQueue>,
}

#[napi]
impl NativeEventBus {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self { bus: Mutex::new(EventQueue::new()) }
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
            bus.push_mouse(btn, Point::new(x as u16, y as u16), NodeId::default());
        }
    }

    #[napi]
    pub fn push_mouse_motion(&self, x: u32, y: u32) {
        if let Ok(mut bus) = self.bus.lock() {
            bus.push_mouse(MouseButton::None, Point::new(x as u16, y as u16), NodeId::default());
        }
    }

    #[napi]
    pub fn push_paste(&self, text: String) {
        if let Ok(mut bus) = self.bus.lock() {
            bus.push_paste(text, NodeId::default());
        }
    }

    #[napi]
    pub fn push_resize(&self, width: u32, height: u32, prev_width: u32, prev_height: u32) {
        if let Ok(mut bus) = self.bus.lock() {
            bus.push_resize(width as u16, height as u16, prev_width as u16, prev_height as u16);
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
    pub fn focus(&self, id: i64) -> bool {
        self.manager.lock().map_or(false, |mut m| m.focus(node_id(id as u64)).is_some())
    }

    #[napi]
    pub fn blur(&self, id: i64) -> bool {
        self.manager.lock().map_or(false, |mut m| m.blur(node_id(id as u64)).is_some())
    }

    #[napi]
    pub fn blur_current(&self) -> bool {
        self.manager
            .lock()
            .map_or(false, |mut m| if let Some(focused) = m.focused() { m.blur(focused).is_some() } else { false })
    }

    #[napi]
    pub fn focused(&self) -> i64 {
        self.manager.lock().map_or(0, |m| m.focused().map_or(0, |id| node_id_u64(id) as i64))
    }

    #[napi]
    pub fn is_focused(&self, id: i64) -> bool {
        self.manager.lock().map_or(false, |m| m.is_focused(node_id(id as u64)))
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

    #[napi]
    pub fn focus_order(&self) -> Vec<i64> {
        self.manager
            .lock()
            .map_or(Vec::new(), |m| m.focusable_nodes().iter().map(|id| node_id_u64(*id) as i64).collect())
    }

    #[napi]
    pub fn clear(&self) {
        if let Ok(mut m) = self.manager.lock() {
            m.clear();
        }
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
        self.engine.lock().map_or(String::new(), |te| te.text())
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

    #[napi]
    pub fn cursor_left(&self) {
        if let Ok(mut te) = self.engine.lock() {
            te.cursor_mut().move_left();
        }
    }

    #[napi]
    pub fn cursor_right(&self) {
        if let Ok(mut te) = self.engine.lock() {
            te.cursor_mut().move_right();
        }
    }

    #[napi]
    pub fn cursor_position(&self) -> u32 {
        self.engine.lock().map_or(0, |te| te.cursor().position() as u32)
    }

    #[napi]
    pub fn set_cursor_position(&self, pos: u32) {
        if let Ok(mut te) = self.engine.lock() {
            te.cursor_mut().set_position(pos as usize);
        }
    }

    #[napi]
    pub fn length(&self) -> u32 {
        self.engine.lock().map_or(0, |te| te.char_count() as u32)
    }

    #[napi]
    pub fn line_count(&self) -> u32 {
        self.engine.lock().map_or(0, |te| te.line_count() as u32)
    }

    #[napi]
    pub fn is_empty(&self) -> bool {
        self.engine.lock().map_or(true, |te| te.is_empty())
    }

    #[napi]
    pub fn word_count(&self) -> u32 {
        self.engine.lock().map_or(0, |te| te.word_count() as u32)
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

    #[napi]
    pub fn fps(&self) -> f64 {
        self.scheduler.lock().map_or(0.0, |s| {
            let interval = s.frame_interval;
            if interval.is_zero() { 0.0 } else { 1000.0 / interval.as_millis() as f64 }
        })
    }

    #[napi]
    pub fn should_render(&self) -> bool {
        self.scheduler.lock().map_or(false, |s| matches!(s.status(), FrameStatus::Due | FrameStatus::Overdue))
    }

    #[napi]
    pub fn request_render_coalesced(&self) {
        if let Ok(mut s) = self.scheduler.lock() {
            s.request_render_coalesced();
        }
    }

    #[napi]
    pub fn request_render_immediate(&self) {
        if let Ok(mut s) = self.scheduler.lock() {
            s.request_render_immediate();
        }
    }

    #[napi]
    pub fn has_scheduled_frame(&self) -> bool {
        self.scheduler.lock().map_or(false, |s| s.has_scheduled_frame())
    }

    #[napi]
    pub fn is_rendering(&self) -> bool {
        self.scheduler.lock().map_or(false, |s| s.is_rendering())
    }

    #[napi]
    pub fn begin_render(&self) {
        if let Ok(mut s) = self.scheduler.lock() {
            s.begin_render();
        }
    }

    #[napi]
    pub fn end_render(&self) -> bool {
        self.scheduler.lock().map_or(false, |mut s| s.end_render())
    }
}

// ─── Animation Timeline (Wrapper Pattern) ───────────────────────────────────────

/// Maps an easing name to an [`Easing`]. Unknown names fall back to `Linear`.
fn easing_from_str(name: &str) -> Easing {
    match name {
        "ease-in" | "easeIn" => Easing::EaseIn,
        "ease-out" | "easeOut" => Easing::EaseOut,
        "ease-in-out" | "easeInOut" => Easing::EaseInOut,
        "ease-in-cubic" | "easeInCubic" => Easing::EaseInCubic,
        "ease-out-cubic" | "easeOutCubic" => Easing::EaseOutCubic,
        "ease-in-out-cubic" | "easeInOutCubic" => Easing::EaseInOutCubic,
        "ease-out-bounce" | "easeOutBounce" => Easing::EaseOutBounce,
        "ease-out-elastic" | "easeOutElastic" => Easing::EaseOutElastic,
        _ => Easing::Linear,
    }
}

/// napi wrapper exposing the Rust animation [`Timeline`] to TypeScript.
/// Schedule scalar tweens at offsets, drive the timeline each frame with `update(dt)`, and read each
/// tween's interpolated value with `animationValue(index)`.
#[napi]
pub struct NativeTimeline {
    timeline: Mutex<Timeline>,
    next_id: Mutex<u32>,
}

#[napi]
impl NativeTimeline {
    #[napi(constructor)]
    pub fn new(duration: Option<f64>, looping: Option<bool>) -> Self {
        let mut tl = Timeline::new();
        if let Some(d) = duration {
            tl = tl.with_duration(d as f32);
        }
        if looping.unwrap_or(false) {
            tl = tl.with_looping(true);
        }
        Self { timeline: Mutex::new(tl), next_id: Mutex::new(1) }
    }

    /// Schedule a scalar tween (`from`→`to` over `duration` seconds, with the
    /// named easing) to begin at `start_time`. Returns the animation index used
    /// by [`animation_value`](Self::animation_value).
    #[napi]
    pub fn add_tween(&self, from: f64, to: f64, duration: f64, start_time: f64, easing: Option<String>) -> u32 {
        let ease = easing.as_deref().map(easing_from_str).unwrap_or(Easing::Linear);
        let mut tween = Tween::new(from as f32, to as f32, duration as f32);
        tween.easing = ease;
        let id = {
            let mut n = self.next_id.lock().unwrap();
            let cur = *n;
            *n += 1;
            cur
        };
        if let Ok(mut tl) = self.timeline.lock() {
            let index = tl.animation_count() as u32;
            tl.add_animation(Animation::from_tween(tween, id), start_time as f32);
            index
        } else {
            0
        }
    }

    #[napi]
    pub fn play(&self) {
        if let Ok(mut tl) = self.timeline.lock() {
            tl.play();
        }
    }

    #[napi]
    pub fn pause(&self) {
        if let Ok(mut tl) = self.timeline.lock() {
            tl.pause();
        }
    }

    #[napi]
    pub fn restart(&self) {
        if let Ok(mut tl) = self.timeline.lock() {
            tl.restart();
        }
    }

    /// Advance the timeline by `dt` seconds (typically the frame delta).
    #[napi]
    pub fn update(&self, dt: f64) {
        if let Ok(mut tl) = self.timeline.lock() {
            tl.update(dt as f32);
        }
    }

    /// Current interpolated value of the tween scheduled at `index`.
    #[napi]
    pub fn animation_value(&self, index: u32) -> Option<f64> {
        self.timeline.lock().ok().and_then(|tl| tl.animation_value(index as usize)).map(|v| v as f64)
    }

    #[napi]
    pub fn current_time(&self) -> f64 {
        self.timeline.lock().map_or(0.0, |tl| tl.current_time() as f64)
    }

    #[napi]
    pub fn is_complete(&self) -> bool {
        self.timeline.lock().map_or(false, |tl| tl.is_complete())
    }

    #[napi]
    pub fn is_playing(&self) -> bool {
        self.timeline.lock().map_or(false, |tl| tl.is_playing())
    }

    #[napi]
    pub fn set_speed(&self, speed: f64) {
        if let Ok(mut tl) = self.timeline.lock() {
            tl.set_speed(speed as f32);
        }
    }

    /// Progress 0.0–1.0 if the timeline has a duration, else `None`.
    #[napi]
    pub fn progress(&self) -> Option<f64> {
        self.timeline.lock().ok().and_then(|tl| tl.progress()).map(|p| p as f64)
    }
}

// ─── Plugin host + slot composition (Wrapper Pattern) ────────────────────────

/// Parses a slot mode name (`append`/`single-winner`/`replace`).
fn slot_mode_from_str(mode: &str) -> SlotMode {
    match mode {
        "single-winner" | "singleWinner" => SlotMode::SingleWinner,
        "replace" => SlotMode::Replace,
        _ => SlotMode::Append,
    }
}

/// Parses a capability name into a [`Capability`]. Unknown names become
/// `Capability::Custom`.
fn capability_from_str(name: &str) -> Capability {
    match name {
        "commands" => Capability::Commands,
        "widgets" => Capability::Widgets,
        "themes" => Capability::Themes,
        "events" => Capability::Events,
        "filesystem" | "fileSystem" => Capability::FileSystem,
        other => Capability::Custom(other.to_string()),
    }
}

fn plugin_state_name(state: PluginState) -> &'static str {
    match state {
        PluginState::Registered => "registered",
        PluginState::Initialized => "initialized",
        PluginState::Running => "running",
        PluginState::Stopped => "stopped",
        PluginState::Error => "error",
    }
}

/// napi wrapper exposing the plugin [`PluginHost`] and named string-valued
/// [`SlotRegistry`]s to TypeScript. Slot values are strings (typically a node id or
/// a serialized descriptor); the TS side owns their meaning.
#[napi]
pub struct NativePluginHost {
    host: Mutex<PluginHost>,
    slots: Mutex<std::collections::HashMap<String, SlotRegistry<String>>>,
}

#[napi]
impl NativePluginHost {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self { host: Mutex::new(PluginHost::new()), slots: Mutex::new(std::collections::HashMap::new()) }
    }

    /// Registers a plugin. `capabilities` is a list of capability names. Returns
    /// an error string on duplicate registration, else `None`.
    #[napi]
    pub fn register(&self, name: String, version: String, author: String, capabilities: Vec<String>) -> Option<String> {
        let info = PluginInfo {
            name,
            version,
            author,
            capabilities: capabilities.iter().map(|c| capability_from_str(c)).collect(),
            metadata: std::collections::HashMap::new(),
        };
        self.host.lock().ok().and_then(|mut h| h.register(info).err())
    }

    /// Unregisters a plugin and removes any slot contributions it made.
    #[napi]
    pub fn unregister(&self, name: String) -> Option<String> {
        if let Ok(mut slots) = self.slots.lock() {
            for reg in slots.values_mut() {
                reg.remove_plugin(&name);
            }
        }
        self.host.lock().ok().and_then(|mut h| h.unregister(&name).err())
    }

    /// Lifecycle hook: `Registered`/`Stopped` → `Initialized`.
    #[napi]
    pub fn initialize(&self, name: String) -> Option<String> {
        self.host.lock().ok().and_then(|mut h| h.initialize(&name).err())
    }

    /// Lifecycle hook: `Initialized`/`Stopped` → `Running`.
    #[napi]
    pub fn start(&self, name: String) -> Option<String> {
        self.host.lock().ok().and_then(|mut h| h.start(&name).err())
    }

    /// Lifecycle hook: `Running` → `Stopped`.
    #[napi]
    pub fn stop(&self, name: String) -> Option<String> {
        self.host.lock().ok().and_then(|mut h| h.stop(&name).err())
    }

    /// Lifecycle hook: mark a plugin as errored.
    #[napi]
    pub fn mark_error(&self, name: String) -> Option<String> {
        self.host.lock().ok().and_then(|mut h| h.mark_error(&name).err())
    }

    /// Current lifecycle state of a plugin, or `None` if unregistered.
    #[napi]
    pub fn state(&self, name: String) -> Option<String> {
        self.host.lock().ok().and_then(|h| h.state(&name)).map(|s| plugin_state_name(s).to_string())
    }

    /// Names of all registered plugins.
    #[napi]
    pub fn plugin_names(&self) -> Vec<String> {
        self.host.lock().map_or_else(|_| Vec::new(), |h| h.names().into_iter().map(String::from).collect())
    }

    /// Ensures a named slot exists with the given resolution mode. No-op if the
    /// slot already exists (mode is not changed).
    #[napi]
    pub fn ensure_slot(&self, slot: String, mode: String) {
        if let Ok(mut slots) = self.slots.lock() {
            slots.entry(slot).or_insert_with(|| SlotRegistry::new(slot_mode_from_str(&mode)));
        }
    }

    /// Registers a contribution to `slot` and returns its token (0 on failure).
    #[napi]
    pub fn slot_register(&self, slot: String, plugin_id: String, priority: i32, value: String) -> i64 {
        if let Ok(mut slots) = self.slots.lock() {
            let reg = slots.entry(slot).or_insert_with(|| SlotRegistry::new(SlotMode::Append));
            reg.register(plugin_id, priority, value) as i64
        } else {
            0
        }
    }

    /// Removes a slot contribution by token. Returns `true` if one was removed.
    #[napi]
    pub fn slot_remove(&self, slot: String, token: i64) -> bool {
        self.slots.lock().map_or(false, |mut slots| slots.get_mut(&slot).map_or(false, |reg| reg.remove(token as u64)))
    }

    /// Resolves a slot to its effective contributions per its mode.
    #[napi]
    pub fn slot_resolve(&self, slot: String) -> Vec<String> {
        self.slots
            .lock()
            .map_or_else(|_| Vec::new(), |slots| slots.get(&slot).map(|reg| reg.resolve()).unwrap_or_default())
    }

    /// Returns and clears a slot's dirty flag (for coalesced recomputation).
    #[napi]
    pub fn slot_take_dirty(&self, slot: String) -> bool {
        self.slots.lock().map_or(false, |mut slots| slots.get_mut(&slot).map_or(false, |reg| reg.take_dirty()))
    }
}

impl Default for NativePluginHost {
    fn default() -> Self {
        Self::new()
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
                    phase: crate::event_bus::EventPhase::Target,
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

    #[napi]
    pub fn set_mode(&self, mode: String) {
        if let Ok(mut km) = self.keymap.lock() {
            km.set_mode(mode);
        }
    }

    #[napi]
    pub fn current_mode(&self) -> String {
        self.keymap.lock().map_or(String::new(), |km| km.current_mode().unwrap_or("").to_string())
    }

    #[napi]
    pub fn clear_mode(&self) {
        if let Ok(mut km) = self.keymap.lock() {
            km.clear_mode();
        }
    }

    #[napi]
    pub fn remove_layer(&self, name: String) -> bool {
        self.keymap.lock().map_or(false, |mut km| km.remove_layer(&name))
    }

    #[napi]
    pub fn set_chord_timeout(&self, ms: i64) {
        if let Ok(mut km) = self.keymap.lock() {
            km.set_chord_timeout(ms as u64);
        }
    }

    #[napi]
    pub fn chord_timeout(&self) -> i64 {
        self.keymap.lock().map_or(0, |km| km.chord_timeout_ms() as i64)
    }

    #[napi]
    pub fn pending_keys(&self) -> Vec<String> {
        self.keymap.lock().map_or(Vec::new(), |km| {
            km.pending_keys().iter().map(|combo| format!("{:?}", combo.key).to_lowercase()).collect()
        })
    }

    #[napi]
    pub fn active_bindings(&self) -> String {
        self.keymap.lock().map_or("[]".to_string(), |km| {
            let bindings: Vec<serde_json::Value> = km
                .active_bindings()
                .iter()
                .map(|(b, layer)| {
                    serde_json::json!({
                        "id": b.id,
                        "keys": format!("{:?}", b.sequence),
                        "command": b.command,
                        "description": b.description,
                        "enabled": b.enabled,
                        "layer": layer,
                    })
                })
                .collect();
            serde_json::to_string(&bindings).unwrap_or_default()
        })
    }

    #[napi]
    pub fn all_bindings(&self) -> String {
        self.keymap.lock().map_or("[]".to_string(), |km| {
            let bindings: Vec<serde_json::Value> = km
                .all_bindings()
                .iter()
                .map(|(b, layer)| {
                    serde_json::json!({
                        "id": b.id,
                        "keys": format!("{:?}", b.sequence),
                        "command": b.command,
                        "description": b.description,
                        "enabled": b.enabled,
                        "layer": layer,
                    })
                })
                .collect();
            serde_json::to_string(&bindings).unwrap_or_default()
        })
    }

    #[napi]
    pub fn command_history(&self) -> Vec<String> {
        self.keymap.lock().map_or(Vec::new(), |km| km.command_history().to_vec())
    }

    #[napi]
    pub fn clear_history(&self) {
        if let Ok(mut km) = self.keymap.lock() {
            km.clear_history();
        }
    }

    #[napi]
    pub fn parse_key(&self, key_str: String) -> String {
        match KeyParser::parse_combo(&key_str) {
            Ok(combo) => serde_json::to_string(&serde_json::json!({
                "key": format!("{:?}", combo.key).to_lowercase(),
                "ctrl": combo.modifiers.ctrl,
                "shift": combo.modifiers.shift,
                "alt": combo.modifiers.alt,
            }))
            .unwrap_or_default(),
            Err(_) => "{}".to_string(),
        }
    }

    #[napi]
    pub fn parse_sequence(&self, key_str: String) -> Vec<String> {
        match KeyParser::parse_sequence(&key_str) {
            Ok(seq) => seq.keys.iter().map(|combo| format!("{:?}", combo.key).to_lowercase()).collect(),
            Err(_) => Vec::new(),
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

fn event_to_json(e: &crate::event_bus::Event) -> serde_json::Value {
    match e {
        crate::event_bus::Event::Key(ke) => serde_json::json!({
            "type": "key",
            "key": format!("{:?}", ke.key),
            "ctrl": ke.modifiers.ctrl,
            "shift": ke.modifiers.shift,
            "alt": ke.modifiers.alt,
            "target": node_id_u64(ke.target),
        }),
        crate::event_bus::Event::Mouse(me) => serde_json::json!({
            "type": "mouse",
            "button": format!("{:?}", me.button).to_lowercase(),
            "x": me.position.x,
            "y": me.position.y,
            "target": node_id_u64(me.target),
        }),
        crate::event_bus::Event::Resize(re) => serde_json::json!({
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
    SetGap { id: String, row: f32, column: f32 },
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

fn parse_color(s: &str) -> crate::tree::Color {
    if let Some(rgba) = crate::tree::Rgba::from_hex(s) {
        return rgba.into();
    }
    match s.to_lowercase().as_str() {
        "black" => crate::tree::Color::Named(crate::tree::NamedColor::Black),
        "red" => crate::tree::Color::Named(crate::tree::NamedColor::Red),
        "green" => crate::tree::Color::Named(crate::tree::NamedColor::Green),
        "yellow" => crate::tree::Color::Named(crate::tree::NamedColor::Yellow),
        "blue" => crate::tree::Color::Named(crate::tree::NamedColor::Blue),
        "magenta" | "purple" => crate::tree::Color::Named(crate::tree::NamedColor::Magenta),
        "cyan" | "teal" => crate::tree::Color::Named(crate::tree::NamedColor::Cyan),
        "white" | "default" => crate::tree::Color::Named(crate::tree::NamedColor::White),
        _ => crate::tree::Color::Default,
    }
}

fn parse_sizing(s: &str) -> crate::taffy::Sizing {
    if let Ok(px) = s.trim_end_matches("px").parse::<f32>() {
        return crate::taffy::Sizing::Points(px);
    }
    if let Some(pct) = s.strip_suffix('%') {
        if let Ok(val) = pct.parse::<f32>() {
            return crate::taffy::Sizing::Percent(val / 100.0);
        }
    }
    match s {
        "auto" => crate::taffy::Sizing::Auto,
        _ => crate::taffy::Sizing::Auto,
    }
}

fn parse_rect_values(top: f32, right: f32, bottom: f32, left: f32) -> crate::taffy::RectValues {
    crate::taffy::RectValues { top: Some(top), right: Some(right), bottom: Some(bottom), left: Some(left) }
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
        // Tree commands
        CommandJson::CreateNode { .. } => None,
        CommandJson::RemoveNode { id } => Some(Command::RemoveNode { id: resolve(&id) }),
        CommandJson::AppendChild { parent, child } => {
            Some(Command::AppendChild { parent: resolve(&parent), child: resolve(&child) })
        }
        CommandJson::InsertBefore { reference, child } => {
            Some(Command::InsertBefore { reference: resolve(&reference), child: resolve(&child) })
        }
        CommandJson::MoveNode { node, new_parent } => {
            Some(Command::MoveNode { node: resolve(&node), new_parent: resolve(&new_parent) })
        }
        CommandJson::ReplaceNode { old, new } => Some(Command::ReplaceNode { old: resolve(&old), new: resolve(&new) }),
        CommandJson::DetachNode { id } => Some(Command::DetachNode { id: resolve(&id) }),
        // Style commands
        CommandJson::SetText { id, text } => Some(Command::SetText { id: resolve(&id), text }),
        CommandJson::SetStyle { id, style_json } => {
            if let Ok(style) = serde_json::from_str::<StyleJson>(&style_json) {
                Some(Command::SetStyle { id: resolve(&id), style: style.into_style() })
            } else {
                None
            }
        }
        CommandJson::SetForeground { id, color } => {
            Some(Command::SetForeground { id: resolve(&id), color: parse_color(&color) })
        }
        CommandJson::SetBackground { id, color } => {
            Some(Command::SetBackground { id: resolve(&id), color: parse_color(&color) })
        }
        CommandJson::SetBold { id, value } => Some(Command::SetBold { id: resolve(&id), value }),
        CommandJson::SetItalic { id, value } => Some(Command::SetItalic { id: resolve(&id), value }),
        CommandJson::SetUnderline { id, value } => Some(Command::SetUnderline { id: resolve(&id), value }),
        CommandJson::SetStrikethrough { id, value } => Some(Command::SetStrikethrough { id: resolve(&id), value }),
        CommandJson::SetDim { id, value } => Some(Command::SetDim { id: resolve(&id), value }),
        CommandJson::SetInverse { id, value } => Some(Command::SetInverse { id: resolve(&id), value }),
        CommandJson::SetHidden { id, value } => Some(Command::SetHidden { id: resolve(&id), value }),
        // Layout commands
        CommandJson::SetFlexDirection { id, value } => {
            let dir = match value.as_str() {
                "column" => crate::taffy::FlexDirection::Column,
                "column-reverse" => crate::taffy::FlexDirection::ColumnReverse,
                "row-reverse" => crate::taffy::FlexDirection::RowReverse,
                _ => crate::taffy::FlexDirection::Row,
            };
            Some(Command::SetFlexDirection { id: resolve(&id), direction: dir })
        }
        CommandJson::SetFlexWrap { id, value } => {
            let wrap = match value.as_str() {
                "wrap" => crate::taffy::FlexWrap::Wrap,
                "wrap-reverse" => crate::taffy::FlexWrap::WrapReverse,
                _ => crate::taffy::FlexWrap::NoWrap,
            };
            Some(Command::SetFlexWrap { id: resolve(&id), value: wrap })
        }
        CommandJson::SetJustifyContent { id, value } => {
            let jc = match value.as_str() {
                "flex-start" => crate::taffy::JustifyContent::FlexStart,
                "flex-end" => crate::taffy::JustifyContent::FlexEnd,
                "center" => crate::taffy::JustifyContent::Center,
                "space-between" => crate::taffy::JustifyContent::SpaceBetween,
                "space-around" => crate::taffy::JustifyContent::SpaceAround,
                "space-evenly" => crate::taffy::JustifyContent::SpaceEvenly,
                _ => crate::taffy::JustifyContent::FlexStart,
            };
            Some(Command::SetJustifyContent { id: resolve(&id), value: jc })
        }
        CommandJson::SetAlignItems { id, value } => {
            let ai = match value.as_str() {
                "flex-start" => crate::taffy::AlignItems::FlexStart,
                "flex-end" => crate::taffy::AlignItems::FlexEnd,
                "center" => crate::taffy::AlignItems::Center,
                "stretch" => crate::taffy::AlignItems::Stretch,
                "baseline" => crate::taffy::AlignItems::Baseline,
                _ => crate::taffy::AlignItems::Stretch,
            };
            Some(Command::SetAlignItems { id: resolve(&id), value: ai })
        }
        CommandJson::SetAlignSelf { id, value } => {
            let as_ = match value.as_str() {
                "flex-start" => crate::taffy::AlignSelf::FlexStart,
                "flex-end" => crate::taffy::AlignSelf::FlexEnd,
                "center" => crate::taffy::AlignSelf::Center,
                "stretch" => crate::taffy::AlignSelf::Stretch,
                "baseline" => crate::taffy::AlignSelf::Baseline,
                _ => crate::taffy::AlignSelf::Stretch,
            };
            Some(Command::SetAlignSelf { id: resolve(&id), value: as_ })
        }
        CommandJson::SetWidth { id, value } => {
            Some(Command::SetWidth { id: resolve(&id), value: parse_sizing(&value) })
        }
        CommandJson::SetHeight { id, value } => {
            Some(Command::SetHeight { id: resolve(&id), value: parse_sizing(&value) })
        }
        CommandJson::SetMinWidth { id, value } => {
            Some(Command::SetMinWidth { id: resolve(&id), value: parse_sizing(&value) })
        }
        CommandJson::SetMinHeight { id, value } => {
            Some(Command::SetMinHeight { id: resolve(&id), value: parse_sizing(&value) })
        }
        CommandJson::SetMaxWidth { id, value } => {
            Some(Command::SetMaxWidth { id: resolve(&id), value: parse_sizing(&value) })
        }
        CommandJson::SetMaxHeight { id, value } => {
            Some(Command::SetMaxHeight { id: resolve(&id), value: parse_sizing(&value) })
        }
        CommandJson::SetPadding { id, top, right, bottom, left } => {
            Some(Command::SetPadding { id: resolve(&id), value: parse_rect_values(top, right, bottom, left) })
        }
        CommandJson::SetMargin { id, top, right, bottom, left } => {
            Some(Command::SetMargin { id: resolve(&id), value: parse_rect_values(top, right, bottom, left) })
        }
        CommandJson::SetGap { id, row, column } => {
            Some(Command::SetGap { id: resolve(&id), value: crate::taffy::Gap { row, column } })
        }
        CommandJson::SetFlexGrow { id, value } => Some(Command::SetFlexGrow { id: resolve(&id), value }),
        CommandJson::SetFlexShrink { id, value } => Some(Command::SetFlexShrink { id: resolve(&id), value }),
        CommandJson::SetFlexBasis { id, value } => {
            Some(Command::SetFlexBasis { id: resolve(&id), value: parse_sizing(&value) })
        }
        CommandJson::SetPosition { id, value } => {
            let pos = match value.as_str() {
                "relative" => crate::taffy::Position::Relative,
                _ => crate::taffy::Position::Absolute,
            };
            Some(Command::SetPosition { id: resolve(&id), value: pos })
        }
        CommandJson::SetInset { id, top, right, bottom, left } => {
            Some(Command::SetInset { id: resolve(&id), value: parse_rect_values(top, right, bottom, left) })
        }
        // Content commands
        CommandJson::SetAttribute { id, key, value } => Some(Command::SetAttribute { id: resolve(&id), key, value }),
        CommandJson::RemoveAttribute { id, key } => Some(Command::RemoveAttribute { id: resolve(&id), key }),
        // Visibility commands
        CommandJson::SetDisplay { id, value } => {
            let display = match value.as_str() {
                "none" => crate::tree::VisibilityDisplay::None,
                _ => crate::tree::VisibilityDisplay::Flex,
            };
            Some(Command::SetDisplay { id: resolve(&id), value: display })
        }
        CommandJson::SetOpacity { id, value } => Some(Command::SetOpacity { id: resolve(&id), value }),
        CommandJson::SetClip { id, value } => Some(Command::SetClip { id: resolve(&id), value }),
        // Transform commands
        CommandJson::SetTranslateX { id, value } => Some(Command::SetTranslateX { id: resolve(&id), value }),
        CommandJson::SetTranslateY { id, value } => Some(Command::SetTranslateY { id: resolve(&id), value }),
        CommandJson::SetZIndex { id, value } => Some(Command::SetZIndex { id: resolve(&id), value }),
        // Overflow commands
        CommandJson::SetOverflow { id, value } => {
            let overflow = match value.as_str() {
                "hidden" => crate::tree::Overflow::Hidden,
                "scroll" => crate::tree::Overflow::Scroll,
                _ => crate::tree::Overflow::Visible,
            };
            Some(Command::SetOverflow { id: resolve(&id), value: overflow })
        }
        // Focus commands
        CommandJson::FocusNode { id } => Some(Command::FocusNode { id: resolve(&id) }),
        CommandJson::BlurNode { id } => Some(Command::BlurNode { id: resolve(&id) }),
        CommandJson::SetTabIndex { id, value } => Some(Command::SetTabIndex { id: resolve(&id), value }),
        // Frame commands
        CommandJson::BeginFrame { frame_id } => Some(Command::BeginFrame { frame_id }),
        CommandJson::CommitFrame { frame_id } => Some(Command::CommitFrame { frame_id }),
        CommandJson::Invalidate { id } => Some(Command::Invalidate { id: resolve(&id) }),
        // Screen commands
        CommandJson::SetScreenMode { mode, footer_height } => {
            let screen_mode = match mode.as_str() {
                "split-footer" => {
                    crate::protocol::ScreenMode::SplitFooter { height: footer_height.unwrap_or(0) as u16 }
                }
                "main-screen" => crate::protocol::ScreenMode::MainScreen,
                _ => crate::protocol::ScreenMode::AlternateScreen,
            };
            Some(Command::SetScreenMode { mode: screen_mode })
        }
        // Lifecycle
        CommandJson::Shutdown => Some(Command::Shutdown),
    }
}

#[derive(serde::Deserialize)]
struct StyleJson {
    #[serde(default)]
    fg: Option<String>,
    #[serde(default)]
    bg: Option<String>,
    #[serde(default)]
    bold: Option<bool>,
    #[serde(default)]
    italic: Option<bool>,
    #[serde(default)]
    underline: Option<bool>,
    #[serde(default)]
    strikethrough: Option<bool>,
    #[serde(default)]
    dim: Option<bool>,
    #[serde(default)]
    inverse: Option<bool>,
    #[serde(default)]
    hidden: Option<bool>,
}

impl StyleJson {
    fn into_style(self) -> crate::tree::Style {
        crate::tree::Style {
            fg: self.fg.map(|c| parse_color(&c)),
            bg: self.bg.map(|c| parse_color(&c)),
            bold: self.bold,
            italic: self.italic,
            underline: self.underline,
            strikethrough: self.strikethrough,
            dim: self.dim,
            inverse: self.inverse,
            hidden: self.hidden,
            ..Default::default()
        }
    }
}

/// Layout payload sent from TypeScript via `engine.setLayout(id, json)`.
///
/// This MUST round-trip every field of `taffy::LayoutProps` — anything omitted
/// here is silently dropped because `serde(default)` ignores unknown keys and
/// `into_layout` fills in defaults. Historical bug: only ~12 of ~25 fields were
/// decoded, so `position: absolute`, `min/maxWidth/Height`, `flexBasis`,
/// `alignSelf`, `overflow`, `display:none`, `aspectRatio`, `boxSizing`, and
/// `border` were all non-functional from TypeScript.
#[derive(serde::Deserialize)]
struct LayoutJson {
    #[serde(default)]
    direction: Option<String>,
    #[serde(default)]
    flex_wrap: Option<String>,
    #[serde(default)]
    justify: Option<String>,
    #[serde(default)]
    align: Option<String>,
    #[serde(default)]
    align_content: Option<String>,
    #[serde(default)]
    align_self: Option<String>,
    #[serde(default)]
    display: Option<String>,
    #[serde(default)]
    position: Option<String>,
    #[serde(default)]
    overflow: Option<String>,
    #[serde(default)]
    box_sizing: Option<String>,
    #[serde(default)]
    width: Option<String>,
    #[serde(default)]
    height: Option<String>,
    #[serde(default)]
    min_width: Option<String>,
    #[serde(default)]
    min_height: Option<String>,
    #[serde(default)]
    max_width: Option<String>,
    #[serde(default)]
    max_height: Option<String>,
    #[serde(default)]
    flex_basis: Option<String>,
    #[serde(default)]
    flex_grow: Option<f32>,
    #[serde(default)]
    flex_shrink: Option<f32>,
    #[serde(default)]
    aspect_ratio: Option<f32>,
    #[serde(default)]
    padding_top: Option<f32>,
    #[serde(default)]
    padding_right: Option<f32>,
    #[serde(default)]
    padding_bottom: Option<f32>,
    #[serde(default)]
    padding_left: Option<f32>,
    #[serde(default)]
    margin_top: Option<f32>,
    #[serde(default)]
    margin_right: Option<f32>,
    #[serde(default)]
    margin_bottom: Option<f32>,
    #[serde(default)]
    margin_left: Option<f32>,
    #[serde(default)]
    border_top: Option<f32>,
    #[serde(default)]
    border_right: Option<f32>,
    #[serde(default)]
    border_bottom: Option<f32>,
    #[serde(default)]
    border_left: Option<f32>,
    #[serde(default)]
    top: Option<f32>,
    #[serde(default)]
    right: Option<f32>,
    #[serde(default)]
    bottom: Option<f32>,
    #[serde(default)]
    left: Option<f32>,
    #[serde(default)]
    gap_row: Option<f32>,
    #[serde(default)]
    gap_column: Option<f32>,
}

impl LayoutJson {
    fn into_layout(self) -> crate::taffy::LayoutProps {
        let direction = match self.direction.as_deref() {
            Some("column") => crate::taffy::FlexDirection::Column,
            Some("column-reverse") => crate::taffy::FlexDirection::ColumnReverse,
            Some("row-reverse") => crate::taffy::FlexDirection::RowReverse,
            Some("row") => crate::taffy::FlexDirection::Row,
            _ => crate::taffy::FlexDirection::default(),
        };

        let flex_wrap = match self.flex_wrap.as_deref() {
            Some("wrap") => crate::taffy::FlexWrap::Wrap,
            Some("wrap-reverse") => crate::taffy::FlexWrap::WrapReverse,
            Some("nowrap") | Some("no-wrap") => crate::taffy::FlexWrap::NoWrap,
            _ => crate::taffy::FlexWrap::default(),
        };

        let justify = match self.justify.as_deref() {
            Some("flex-end") => crate::taffy::JustifyContent::FlexEnd,
            Some("center") => crate::taffy::JustifyContent::Center,
            Some("space-between") => crate::taffy::JustifyContent::SpaceBetween,
            Some("space-around") => crate::taffy::JustifyContent::SpaceAround,
            Some("space-evenly") => crate::taffy::JustifyContent::SpaceEvenly,
            _ => crate::taffy::JustifyContent::default(),
        };

        let align = match self.align.as_deref() {
            Some("flex-start") => crate::taffy::AlignItems::FlexStart,
            Some("flex-end") => crate::taffy::AlignItems::FlexEnd,
            Some("center") => crate::taffy::AlignItems::Center,
            Some("stretch") => crate::taffy::AlignItems::Stretch,
            Some("baseline") => crate::taffy::AlignItems::Baseline,
            _ => crate::taffy::AlignItems::default(),
        };

        let align_content = self.align_content.as_deref().map(|s| match s {
            "flex-start" => crate::taffy::AlignContent::FlexStart,
            "flex-end" => crate::taffy::AlignContent::FlexEnd,
            "center" => crate::taffy::AlignContent::Center,
            "stretch" => crate::taffy::AlignContent::Stretch,
            "space-between" => crate::taffy::AlignContent::SpaceBetween,
            "space-around" => crate::taffy::AlignContent::SpaceAround,
            _ => crate::taffy::AlignContent::default(),
        });

        let align_self = self.align_self.as_deref().map(|s| match s {
            "flex-start" => crate::taffy::AlignSelf::FlexStart,
            "flex-end" => crate::taffy::AlignSelf::FlexEnd,
            "center" => crate::taffy::AlignSelf::Center,
            "stretch" => crate::taffy::AlignSelf::Stretch,
            "baseline" => crate::taffy::AlignSelf::Baseline,
            "auto" => crate::taffy::AlignSelf::Stretch,
            _ => crate::taffy::AlignSelf::Stretch,
        });

        // NOTE: `taffy::types::Display` currently has only `Flex` and `None`.
        // CSS `contents`/`block` are accepted for forward-compat but map to Flex.
        let display = match self.display.as_deref() {
            Some("none") => crate::taffy::types::Display::None,
            Some("flex") | Some("block") | Some("contents") | None => crate::taffy::types::Display::Flex,
            _ => crate::taffy::types::Display::Flex,
        };

        let position = match self.position.as_deref() {
            Some("absolute") => crate::taffy::Position::Absolute,
            Some("static") => crate::taffy::Position::Static,
            Some("relative") => crate::taffy::Position::Relative,
            _ => crate::taffy::Position::default(),
        };

        let overflow = self.overflow.as_deref().map(|s| match s {
            "hidden" => crate::taffy::LayoutOverflow::Hidden,
            "scroll" => crate::taffy::LayoutOverflow::Scroll,
            "visible" => crate::taffy::LayoutOverflow::Visible,
            _ => crate::taffy::LayoutOverflow::default(),
        });

        let box_sizing = self.box_sizing.as_deref().map(|s| match s {
            "border-box" => crate::taffy::BoxSizing::BorderBox,
            "content-box" => crate::taffy::BoxSizing::ContentBox,
            _ => crate::taffy::BoxSizing::default(),
        });

        // Inset: explicit `top/right/bottom/left` win over `inset` shorthand;
        // `layoutToEngineJson` in cliRenderer.ts already explodes `inset` into
        // the per-edge fields, so we only need to read the per-edge fields here.
        let inset_present = self.top.is_some() || self.right.is_some() || self.bottom.is_some() || self.left.is_some();
        let inset = if inset_present {
            Some(crate::taffy::RectValues { top: self.top, right: self.right, bottom: self.bottom, left: self.left })
        } else {
            None
        };

        // Border: a per-edge value (typically 0 or 1 in terminal contexts).
        // A single `border: true` from BoxOptions is translated to 1-per-edge
        // on the TS side; here we accept any numeric per-edge width.
        let border_present = self.border_top.is_some()
            || self.border_right.is_some()
            || self.border_bottom.is_some()
            || self.border_left.is_some();
        let border = if border_present {
            Some(crate::taffy::RectValues {
                top: self.border_top,
                right: self.border_right,
                bottom: self.border_bottom,
                left: self.border_left,
            })
        } else {
            None
        };

        crate::taffy::LayoutProps {
            display,
            position,
            direction,
            flex_wrap,
            justify,
            align,
            align_content,
            align_self,
            flex_grow: self.flex_grow.unwrap_or(0.0),
            flex_shrink: self.flex_shrink.unwrap_or(1.0),
            flex_basis: self.flex_basis.as_deref().map(parse_sizing),
            gap: Some(crate::taffy::Gap { row: self.gap_row.unwrap_or(0.0), column: self.gap_column.unwrap_or(0.0) }),
            padding: Some(parse_rect_values(
                self.padding_top.unwrap_or(0.0),
                self.padding_right.unwrap_or(0.0),
                self.padding_bottom.unwrap_or(0.0),
                self.padding_left.unwrap_or(0.0),
            )),
            margin: Some(parse_rect_values(
                self.margin_top.unwrap_or(0.0),
                self.margin_right.unwrap_or(0.0),
                self.margin_bottom.unwrap_or(0.0),
                self.margin_left.unwrap_or(0.0),
            )),
            border,
            width: self.width.as_deref().map(parse_sizing),
            height: self.height.as_deref().map(parse_sizing),
            min_width: self.min_width.as_deref().map(parse_sizing),
            min_height: self.min_height.as_deref().map(parse_sizing),
            max_width: self.max_width.as_deref().map(parse_sizing),
            max_height: self.max_height.as_deref().map(parse_sizing),
            inset,
            aspect_ratio: self.aspect_ratio,
            overflow,
            box_sizing,
        }
    }
}

// ─── Theme Napi Bindings ─────────────────────────────────────────────────────────

fn color_to_hex(c: crate::tree::Color) -> String {
    match c {
        crate::tree::Color::Rgb { r, g, b } => format!("#{:02x}{:02x}{:02x}", r, g, b),
        crate::tree::Color::Named(n) => {
            let (r, g, b) = n.to_rgb();
            format!("#{:02x}{:02x}{:02x}", r, g, b)
        }
        crate::tree::Color::Indexed(i) => {
            let (r, g, b) = crate::tree::indexed_to_rgb(i);
            format!("#{:02x}{:02x}{:02x}", r, g, b)
        }
        crate::tree::Color::Default => "#000000".to_string(),
    }
}

#[napi(object)]
pub struct NapiThemeColors {
    pub background: String,
    pub surface: String,
    pub surface_high: String,
    pub surface_low: String,
    pub primary: String,
    pub primary_foreground: String,
    pub secondary: String,
    pub secondary_foreground: String,
    pub text: String,
    pub text_muted: String,
    pub text_dim: String,
    pub border: String,
    pub border_focused: String,
    pub accent: String,
    pub accent_foreground: String,
    pub error: String,
    pub warning: String,
    pub success: String,
    pub info: String,
    pub scrollbar: String,
    pub scrollbar_thumb: String,
}

#[napi(object)]
pub struct NapiThemeSpacing {
    pub none: u32,
    pub xxs: u32,
    pub xs: u32,
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
    pub xxl: u32,
}

#[napi(object)]
pub struct NapiThemeBorders {
    pub style: String,
    pub fg: String,
}

#[napi(object)]
pub struct NapiTheme {
    pub name: String,
    pub colors: NapiThemeColors,
    pub spacing: NapiThemeSpacing,
    pub borders: NapiThemeBorders,
}

impl From<Theme> for NapiTheme {
    fn from(t: Theme) -> Self {
        NapiTheme {
            name: t.name.to_string(),
            colors: NapiThemeColors {
                background: color_to_hex(t.colors.background),
                surface: color_to_hex(t.colors.surface),
                surface_high: color_to_hex(t.colors.surface_high),
                surface_low: color_to_hex(t.colors.surface_low),
                primary: color_to_hex(t.colors.primary),
                primary_foreground: color_to_hex(t.colors.primary_foreground),
                secondary: color_to_hex(t.colors.secondary),
                secondary_foreground: color_to_hex(t.colors.secondary_foreground),
                text: color_to_hex(t.colors.text),
                text_muted: color_to_hex(t.colors.text_muted),
                text_dim: color_to_hex(t.colors.text_dim),
                border: color_to_hex(t.colors.border),
                border_focused: color_to_hex(t.colors.border_focused),
                accent: color_to_hex(t.colors.accent),
                accent_foreground: color_to_hex(t.colors.accent_foreground),
                error: color_to_hex(t.colors.error),
                warning: color_to_hex(t.colors.warning),
                success: color_to_hex(t.colors.success),
                info: color_to_hex(t.colors.info),
                scrollbar: color_to_hex(t.colors.scrollbar),
                scrollbar_thumb: color_to_hex(t.colors.scrollbar_thumb),
            },
            spacing: NapiThemeSpacing {
                none: t.spacing.none as u32,
                xxs: t.spacing.xxs as u32,
                xs: t.spacing.xs as u32,
                sm: t.spacing.sm as u32,
                md: t.spacing.md as u32,
                lg: t.spacing.lg as u32,
                xl: t.spacing.xl as u32,
                xxl: t.spacing.xxl as u32,
            },
            borders: NapiThemeBorders { style: format!("{:?}", t.borders.style), fg: color_to_hex(t.borders.fg) },
        }
    }
}

#[napi]
pub fn create_dark_theme() -> NapiTheme {
    Theme::dark().into()
}

#[napi]
pub fn create_light_theme() -> NapiTheme {
    Theme::light().into()
}

// ─── EventPipeline Class (Wrapper Pattern) ───────────────────────────────────────

#[napi(object)]
pub struct NativeEventPipelineConfig {
    pub kitty_keyboard: Option<bool>,
    pub bracketed_paste: Option<bool>,
    pub focus_events: Option<bool>,
    pub mouse_tracking: Option<bool>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl Default for NativeEventPipelineConfig {
    fn default() -> Self {
        Self {
            kitty_keyboard: Some(false),
            bracketed_paste: Some(false),
            focus_events: Some(false),
            mouse_tracking: Some(false),
            width: Some(80),
            height: Some(24),
        }
    }
}

impl From<NativeEventPipelineConfig> for crate::event_pipeline::EventPipelineConfig {
    fn from(c: NativeEventPipelineConfig) -> Self {
        Self {
            kitty_keyboard: c.kitty_keyboard.unwrap_or(false),
            bracketed_paste: c.bracketed_paste.unwrap_or(false),
            focus_events: c.focus_events.unwrap_or(false),
            mouse_tracking: c.mouse_tracking.unwrap_or(false),
            width: c.width.unwrap_or(80) as u16,
            height: c.height.unwrap_or(24) as u16,
        }
    }
}

#[napi]
pub struct NativeEventPipeline {
    pipeline: Mutex<crate::event_pipeline::EventPipeline>,
}

#[napi]
impl NativeEventPipeline {
    #[napi(constructor)]
    pub fn new(config: Option<NativeEventPipelineConfig>) -> Self {
        let cfg = config.unwrap_or_default();
        Self { pipeline: Mutex::new(crate::event_pipeline::EventPipeline::new(cfg.into())) }
    }

    #[napi]
    pub fn feed(&self, data: napi::bindgen_prelude::Buffer) {
        if let Ok(mut pipeline) = self.pipeline.lock() {
            pipeline.feed(&data);
        }
    }

    #[napi]
    pub fn push_key(&self, key: String, ctrl: bool, shift: bool, alt: bool) {
        if let Ok(mut pipeline) = self.pipeline.lock() {
            pipeline.push_key(parse_key_str(&key), Modifiers { ctrl, shift, alt, meta: false }, NodeId::default());
        }
    }

    #[napi]
    pub fn push_mouse(&self, button: String, x: u32, y: u32) {
        if let Ok(mut pipeline) = self.pipeline.lock() {
            let btn = match button.as_str() {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                "scroll_up" => MouseButton::ScrollUp,
                "scroll_down" => MouseButton::ScrollDown,
                _ => MouseButton::None,
            };
            pipeline.push_mouse(btn, Point::new(x as u16, y as u16), NodeId::default());
        }
    }

    #[napi]
    pub fn push_paste(&self, text: String) {
        if let Ok(mut pipeline) = self.pipeline.lock() {
            pipeline.push_paste(text, NodeId::default());
        }
    }

    #[napi]
    pub fn push_resize(&self, width: u32, height: u32, prev_width: u32, prev_height: u32) {
        if let Ok(mut pipeline) = self.pipeline.lock() {
            pipeline.push_resize(width as u16, height as u16, prev_width as u16, prev_height as u16);
        }
    }

    #[napi]
    pub fn drain(&self) -> String {
        self.pipeline.lock().map_or("[]".to_string(), |mut pipeline| {
            let events: Vec<serde_json::Value> = pipeline.drain().iter().map(event_to_json).collect();
            serde_json::to_string(&events).unwrap_or_default()
        })
    }

    #[napi]
    pub fn len(&self) -> u32 {
        self.pipeline.lock().map_or(0, |pipeline| pipeline.len() as u32)
    }

    #[napi]
    pub fn is_empty(&self) -> bool {
        self.pipeline.lock().map_or(true, |pipeline| pipeline.is_empty())
    }

    #[napi]
    pub fn clear(&self) {
        if let Ok(mut pipeline) = self.pipeline.lock() {
            pipeline.clear();
        }
    }

    #[napi]
    pub fn resize(&self, width: u32, height: u32) {
        if let Ok(mut pipeline) = self.pipeline.lock() {
            pipeline.resize(width as u16, height as u16);
        }
    }

    #[napi]
    pub fn reset(&self) {
        if let Ok(mut pipeline) = self.pipeline.lock() {
            pipeline.reset();
        }
    }
}

// ─── Logger Bindings ─────────────────────────────────────────────────────────────

/// Logger configuration for TypeScript
#[napi(object)]
pub struct NapiLoggerConfig {
    pub level: Option<String>,
    pub color: Option<bool>,
    pub timestamp: Option<bool>,
    pub module: Option<bool>,
    pub thread: Option<bool>,
    pub file: Option<String>,
    pub max_file_size: Option<i64>,
    pub max_files: Option<u32>,
    pub dev: Option<bool>,
}

impl From<NapiLoggerConfig> for LoggerConfig {
    fn from(config: NapiLoggerConfig) -> Self {
        let mut logger_config = LoggerConfig::default();

        if let Some(level_str) = config.level {
            if let Some(level) = Level::from_str(&level_str) {
                logger_config.level = level;
            }
        }

        if let Some(color) = config.color {
            logger_config.color = color;
        }

        if let Some(timestamp) = config.timestamp {
            logger_config.timestamp = timestamp;
        }

        if let Some(module) = config.module {
            logger_config.module = module;
        }

        if let Some(thread) = config.thread {
            logger_config.thread = thread;
        }

        if let Some(file) = config.file {
            logger_config.file = Some(file);
        }

        if let Some(max_file_size) = config.max_file_size {
            logger_config.max_file_size = Some(max_file_size as u64);
        }

        if let Some(max_files) = config.max_files {
            logger_config.max_files = Some(max_files as usize);
        }

        if let Some(dev) = config.dev {
            logger_config.dev = dev;
        }

        logger_config
    }
}

/// Diagnostic snapshot for TypeScript
#[napi(object)]
pub struct NapiDiagnosticSnapshot {
    pub render_calls: f64,
    pub render_bytes: f64,
    pub event_dispatches: f64,
    pub layout_computations: f64,
    pub cache_hits: f64,
    pub cache_misses: f64,
    pub allocations: f64,
    pub average_frame_time: f64,
    pub fps: f64,
}

/// Initialize the logger with the given configuration
#[napi]
pub fn logger_init(config: NapiLoggerConfig) -> Result<(), napi::Error> {
    let logger_config = LoggerConfig::from(config);
    Logger::init(logger_config).map_err(|e| napi::Error::from_reason(format!("Logger initialization failed: {}", e)))
}

/// Set the global log level
#[napi]
pub fn logger_set_level(level: String) -> Result<(), napi::Error> {
    Level::from_str(&level)
        .map(|l| {
            Logger::set_level(l);
            Ok(())
        })
        .unwrap_or_else(|| Err(napi::Error::from_reason(format!("Invalid log level: {}", level))))
}

/// Get the current log level
#[napi]
pub fn logger_get_level() -> String {
    Logger::get_level().as_str().to_string()
}

/// Set the module filter
#[napi]
pub fn logger_set_module_filter(include: Option<Vec<String>>, exclude: Option<Vec<String>>) {
    let mut filter = ModuleFilter::default();

    if let Some(inc) = include {
        filter = filter.include_many(inc);
    }

    if let Some(exc) = exclude {
        filter = filter.exclude_many(exc);
    }

    Logger::set_module_filter(filter);
}

/// Get a snapshot of diagnostic counters
#[napi]
pub fn logger_get_diagnostics() -> NapiDiagnosticSnapshot {
    let snapshot = Logger::snapshot_diagnostics();
    NapiDiagnosticSnapshot {
        render_calls: snapshot.render_calls as f64,
        render_bytes: snapshot.render_bytes as f64,
        event_dispatches: snapshot.event_dispatches as f64,
        layout_computations: snapshot.layout_computations as f64,
        cache_hits: snapshot.cache_hits as f64,
        cache_misses: snapshot.cache_misses as f64,
        allocations: snapshot.allocations as f64,
        average_frame_time: snapshot.average_frame_time,
        fps: snapshot.fps,
    }
}

/// Flush the logger
#[napi]
pub fn logger_flush() {
    Logger::flush();
}

// ─── Syntax Highlighting ─────────────────────────────────────────────────────

/// A single styled segment returned by [`highlight_code`].
#[napi(object)]
pub struct HighlightSegment {
    pub text: String,
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub dim: Option<bool>,
    pub strikethrough: Option<bool>,
}

/// A single highlighted line, composed of [`HighlightSegment`]s.
#[napi(object)]
pub struct HighlightedLine {
    pub segments: Vec<HighlightSegment>,
}

/// Highlight `code` written in `language` using the built-in tree-sitter engine.
///
/// Returns one `HighlightedLine` per source line (empty array if the language
/// is unknown or the input is empty).
#[napi]
pub fn highlight_code(code: String, language: String) -> Vec<HighlightedLine> {
    use crate::syntax::global_highlighter;

    let highlighter = global_highlighter();
    let result = highlighter.lock().ok().and_then(|mut h| h.highlight(&code, &language));

    match result {
        None => Vec::new(),
        Some(lines) => lines
            .into_iter()
            .map(|line| HighlightedLine {
                segments: line
                    .segments
                    .into_iter()
                    .map(|seg| {
                        let style = &seg.style;
                        HighlightSegment {
                            text: seg.text,
                            fg: style.fg.map(color_to_hex),
                            bg: style.bg.map(color_to_hex),
                            bold: style.bold,
                            italic: style.italic,
                            underline: style.underline,
                            dim: style.dim,
                            strikethrough: style.strikethrough,
                        }
                    })
                    .collect(),
            })
            .collect(),
    }
}

#[cfg(test)]
mod layout_json_tests {
    use super::*;

    /// Regression: previously `LayoutJson` only deserialized ~12 fields and
    /// `into_layout` hardcoded `position: Relative`, `inset: None`, etc.,
    /// silently dropping absolute positioning, min/max sizing, flex-basis,
    /// alignSelf, overflow, display:none, aspectRatio, boxSizing, and border.
    #[test]
    fn layout_json_round_trips_all_layout_props() {
        let json = r#"{
            "direction": "row",
            "flex_wrap": "wrap-reverse",
            "justify": "space-evenly",
            "align": "baseline",
            "align_content": "space-between",
            "align_self": "flex-end",
            "display": "none",
            "position": "absolute",
            "overflow": "hidden",
            "box_sizing": "border-box",
            "width": "50%",
            "height": "20",
            "min_width": "5",
            "min_height": "3",
            "max_width": "100",
            "max_height": "40",
            "flex_basis": "50%",
            "flex_grow": 2.0,
            "flex_shrink": 0.0,
            "aspect_ratio": 1.6,
            "padding_top": 1.0,
            "padding_right": 2.0,
            "padding_bottom": 3.0,
            "padding_left": 4.0,
            "margin_top": 0.5,
            "margin_right": 0.0,
            "margin_bottom": 0.0,
            "margin_left": 0.0,
            "border_top": 1.0,
            "border_right": 1.0,
            "border_bottom": 1.0,
            "border_left": 1.0,
            "top": 10.0,
            "right": 20.0,
            "bottom": 30.0,
            "left": 40.0,
            "gap_row": 1.0,
            "gap_column": 2.0
        }"#;
        let parsed: LayoutJson = serde_json::from_str(json).expect("JSON must parse");
        let layout = parsed.into_layout();

        assert_eq!(layout.display, crate::taffy::types::Display::None);
        assert_eq!(layout.position, crate::taffy::Position::Absolute);
        assert_eq!(layout.direction, crate::taffy::FlexDirection::Row);
        assert_eq!(layout.flex_wrap, crate::taffy::FlexWrap::WrapReverse);
        assert_eq!(layout.justify, crate::taffy::JustifyContent::SpaceEvenly);
        assert_eq!(layout.align, crate::taffy::AlignItems::Baseline);
        assert_eq!(layout.align_content, Some(crate::taffy::AlignContent::SpaceBetween));
        assert_eq!(layout.align_self, Some(crate::taffy::AlignSelf::FlexEnd));
        assert_eq!(layout.flex_grow, 2.0);
        assert_eq!(layout.flex_shrink, 0.0);
        assert_eq!(layout.flex_basis, Some(crate::taffy::Sizing::Percent(0.5)));
        assert_eq!(layout.width, Some(crate::taffy::Sizing::Percent(0.5)));
        assert_eq!(layout.height, Some(crate::taffy::Sizing::Points(20.0)));
        assert_eq!(layout.min_width, Some(crate::taffy::Sizing::Points(5.0)));
        assert_eq!(layout.min_height, Some(crate::taffy::Sizing::Points(3.0)));
        assert_eq!(layout.max_width, Some(crate::taffy::Sizing::Points(100.0)));
        assert_eq!(layout.max_height, Some(crate::taffy::Sizing::Points(40.0)));
        assert_eq!(layout.aspect_ratio, Some(1.6));
        assert_eq!(layout.overflow, Some(crate::taffy::LayoutOverflow::Hidden));
        assert_eq!(layout.box_sizing, Some(crate::taffy::BoxSizing::BorderBox));
        assert_eq!(
            layout.inset,
            Some(crate::taffy::RectValues { top: Some(10.0), right: Some(20.0), bottom: Some(30.0), left: Some(40.0) })
        );
        assert_eq!(
            layout.border,
            Some(crate::taffy::RectValues { top: Some(1.0), right: Some(1.0), bottom: Some(1.0), left: Some(1.0) })
        );
        assert_eq!(layout.gap, Some(crate::taffy::Gap { row: 1.0, column: 2.0 }));
        assert_eq!(
            layout.padding,
            Some(crate::taffy::RectValues { top: Some(1.0), right: Some(2.0), bottom: Some(3.0), left: Some(4.0) })
        );
        assert_eq!(
            layout.margin,
            Some(crate::taffy::RectValues { top: Some(0.5), right: Some(0.0), bottom: Some(0.0), left: Some(0.0) })
        );
    }

    /// Regression: previously `position: "absolute"` was silently dropped,
    /// always becoming `Relative` in the engine. Verify the round-trip.
    #[test]
    fn layout_json_position_absolute_round_trips() {
        let json = r#"{ "position": "absolute", "top": 5.0, "left": 10.0 }"#;
        let layout: LayoutJson = serde_json::from_str(json).expect("JSON must parse");
        let props = layout.into_layout();
        assert_eq!(props.position, crate::taffy::Position::Absolute);
        assert_eq!(
            props.inset,
            Some(crate::taffy::RectValues { top: Some(5.0), left: Some(10.0), right: None, bottom: None })
        );
    }

    /// Regression: previously `display: "none"` was dropped, always becoming
    /// `Flex`. Verify the round-trip.
    #[test]
    fn layout_json_display_none_round_trips() {
        let json = r#"{ "display": "none" }"#;
        let layout: LayoutJson = serde_json::from_str(json).expect("JSON must parse");
        let props = layout.into_layout();
        assert_eq!(props.display, crate::taffy::types::Display::None);
    }

    /// Defaults: omitting fields should yield `LayoutProps::default()` shapes
    /// (no `None` where the old code hard-coded `Relative`, etc.).
    #[test]
    fn layout_json_empty_payload_yields_defaults() {
        let json = r#"{}"#;
        let layout: LayoutJson = serde_json::from_str(json).expect("JSON must parse");
        let props = layout.into_layout();
        assert_eq!(props.display, crate::taffy::types::Display::Flex);
        assert_eq!(props.position, crate::taffy::Position::Relative);
        assert_eq!(props.direction, crate::taffy::FlexDirection::default());
        assert_eq!(props.inset, None);
        assert_eq!(props.min_width, None);
        assert_eq!(props.max_width, None);
        assert_eq!(props.flex_basis, None);
        assert_eq!(props.overflow, None);
        assert_eq!(props.box_sizing, None);
    }

    /// Regression: `flex_basis` previously dropped. Verify it parses both
    /// points and percentages.
    #[test]
    fn layout_json_flex_basis_round_trips() {
        for (json, expected) in [
            (r#"{ "flex_basis": "100" }"#, crate::taffy::Sizing::Points(100.0)),
            (r#"{ "flex_basis": "50%" }"#, crate::taffy::Sizing::Percent(0.5)),
            (r#"{ "flex_basis": "auto" }"#, crate::taffy::Sizing::Auto),
        ] {
            let layout: LayoutJson = serde_json::from_str(json).expect("JSON must parse");
            let props = layout.into_layout();
            assert_eq!(props.flex_basis, Some(expected));
        }
    }
}
