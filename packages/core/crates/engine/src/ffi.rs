//! C ABI bindings for node:ffi.
//! Every function is `extern "C"` with `#[unsafe(no_mangle)]`.
//! Handles are opaque u64 values into global type-safe stores.
//! String returns are `*mut c_char` (caller must free with `ffi_free_string`).

use base64::Engine as _;
use std::collections::HashMap;
use std::ffi::CString;
use std::sync::LazyLock;
use std::sync::Mutex;

use crate::VERSION;
use crate::engine::Engine;
use crate::event_bus::{EventPhase, EventQueue, Key, KeyEvent, Modifiers, MouseButton};
use crate::input::{
    FocusDirection, FocusManager, FocusScope, FocusScopeType, FocusTraversal, KeyBinding, KeyParser, Keymap,
};
use crate::protocol::Command;
use crate::render::Renderer;
use crate::scheduler::{FrameStatus, Scheduler};
use crate::syntax::global_highlighter;
use crate::taffy::types::{
    AlignItems, AlignSelf, FlexDirection, Gap, JustifyContent, LayoutProps, Position, RectValues, Sizing,
};
use crate::text::TextEngine;
use crate::theme::Theme;
use crate::tree::{Color, NamedColor, NodeId, NodeKind, Overflow, Point, Style, VisibilityDisplay};

// ─── Handle Store ────────────────────────────────────────────────────────────

struct HandleStore<T> {
    next_id: u64,
    objects: HashMap<u64, T>,
}

impl<T> Default for HandleStore<T> {
    fn default() -> Self {
        Self { next_id: 1, objects: HashMap::new() }
    }
}

impl<T> HandleStore<T> {
    fn insert(&mut self, obj: T) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.objects.insert(id, obj);
        id
    }
    fn get(&self, id: u64) -> Option<&T> {
        self.objects.get(&id)
    }
    fn get_mut(&mut self, id: u64) -> Option<&mut T> {
        self.objects.get_mut(&id)
    }
    fn remove(&mut self, id: u64) -> Option<T> {
        self.objects.remove(&id)
    }
}

struct EngineState {
    engine: Engine,
    renderer: Renderer,
    id_map: HashMap<u64, u64>,
}

static ENGINES: LazyLock<Mutex<HandleStore<EngineState>>> = LazyLock::new(Default::default);
static EVENT_BUSES: LazyLock<Mutex<HandleStore<EventQueue>>> = LazyLock::new(Default::default);
static FOCUS_MANAGERS: LazyLock<Mutex<HandleStore<FocusManager>>> = LazyLock::new(Default::default);
static TEXT_ENGINES: LazyLock<Mutex<HandleStore<TextEngine>>> = LazyLock::new(Default::default);
static SCHEDULERS: LazyLock<Mutex<HandleStore<Scheduler>>> = LazyLock::new(Default::default);
static KEYMAPS: LazyLock<Mutex<HandleStore<Keymap>>> = LazyLock::new(Default::default);

// ─── String Helpers ──────────────────────────────────────────────────────────

fn json_string(value: &serde_json::Value) -> *mut std::ffi::c_char {
    let s = serde_json::to_string(value).unwrap_or_default();
    CString::new(s).unwrap_or_default().into_raw()
}

fn str_from_parts<'a>(ptr: *const u8, len: u32) -> &'a str {
    if ptr.is_null() || len == 0 {
        return "";
    }
    unsafe { std::str::from_utf8(std::slice::from_raw_parts(ptr, len as usize)).unwrap_or("") }
}

fn node_id(val: u64) -> NodeId {
    unsafe { std::mem::transmute(val) }
}

fn node_id_u64(id: NodeId) -> u64 {
    unsafe { std::mem::transmute(id) }
}

fn theme_color_hex(c: Color) -> String {
    c.to_rgba(255).to_hex()
}

// ─── JSON Command Types ──────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
#[serde(tag = "type")]
enum CommandJson {
    CreateNode {
        id: u64,
        kind: String,
    },
    RemoveNode {
        id: u64,
    },
    AppendChild {
        parent: u64,
        child: u64,
    },
    InsertBefore {
        reference: u64,
        child: u64,
    },
    MoveNode {
        node: u64,
        #[serde(rename = "newParent")]
        new_parent: u64,
    },
    ReplaceNode {
        old: u64,
        new: u64,
    },
    DetachNode {
        id: u64,
    },
    SetText {
        id: u64,
        text: String,
    },
    SetStyle {
        id: u64,
        style: StyleJson,
    },
    SetForeground {
        id: u64,
        color: ColorJson,
    },
    SetBackground {
        id: u64,
        color: ColorJson,
    },
    SetBold {
        id: u64,
        value: bool,
    },
    SetItalic {
        id: u64,
        value: bool,
    },
    SetUnderline {
        id: u64,
        value: bool,
    },
    SetStrikethrough {
        id: u64,
        value: bool,
    },
    SetDim {
        id: u64,
        value: bool,
    },
    SetInverse {
        id: u64,
        value: bool,
    },
    SetHidden {
        id: u64,
        value: bool,
    },
    SetLayout {
        id: u64,
        layout: LayoutJson,
    },
    SetFlexDirection {
        id: u64,
        direction: String,
    },
    SetJustifyContent {
        id: u64,
        value: String,
    },
    SetAlignItems {
        id: u64,
        value: String,
    },
    SetAlignSelf {
        id: u64,
        value: String,
    },
    SetWidth {
        id: u64,
        value: SizingJson,
    },
    SetHeight {
        id: u64,
        value: SizingJson,
    },
    SetMinWidth {
        id: u64,
        value: SizingJson,
    },
    SetMinHeight {
        id: u64,
        value: SizingJson,
    },
    SetMaxWidth {
        id: u64,
        value: SizingJson,
    },
    SetMaxHeight {
        id: u64,
        value: SizingJson,
    },
    SetFlexBasis {
        id: u64,
        value: SizingJson,
    },
    SetPadding {
        id: u64,
        value: RectValuesJson,
    },
    SetMargin {
        id: u64,
        value: RectValuesJson,
    },
    SetGap {
        id: u64,
        value: GapJson,
    },
    SetFlexGrow {
        id: u64,
        value: f32,
    },
    SetFlexShrink {
        id: u64,
        value: f32,
    },
    SetPosition {
        id: u64,
        value: String,
    },
    SetInset {
        id: u64,
        value: RectValuesJson,
    },
    SetDisplay {
        id: u64,
        value: String,
    },
    SetOpacity {
        id: u64,
        value: f32,
    },
    SetClip {
        id: u64,
        value: bool,
    },
    SetZIndex {
        id: u64,
        value: i32,
    },
    SetOverflow {
        id: u64,
        value: String,
    },
    SetAttribute {
        id: u64,
        key: String,
        value: String,
    },
    RemoveAttribute {
        id: u64,
        key: String,
    },
    SetTranslateX {
        id: u64,
        value: i32,
    },
    SetTranslateY {
        id: u64,
        value: i32,
    },
    SetTabIndex {
        id: u64,
        value: i32,
    },
    FocusNode {
        id: u64,
    },
    BlurNode {
        id: u64,
    },
    BeginFrame {
        #[serde(rename = "frameId")]
        frame_id: u64,
    },
    CommitFrame {
        #[serde(rename = "frameId")]
        frame_id: u64,
    },
    Invalidate {
        id: u64,
    },
    Shutdown,
}

#[derive(serde::Deserialize)]
struct StyleJson {
    #[serde(default)]
    fg: Option<ColorJson>,
    #[serde(default)]
    bg: Option<ColorJson>,
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
    #[serde(default, alias = "textAlign")]
    text_align: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum ColorJson {
    Named(String),
    Rgb { r: u8, g: u8, b: u8 },
}

#[derive(serde::Deserialize)]
struct LayoutJson {
    #[serde(default)]
    direction: Option<String>,
    #[serde(default)]
    justify: Option<String>,
    #[serde(default)]
    align: Option<String>,
    #[serde(default)]
    padding: Option<RectValuesJson>,
    #[serde(default)]
    margin: Option<RectValuesJson>,
    #[serde(default)]
    gap: Option<GapJson>,
    #[serde(default)]
    flex_grow: Option<f32>,
    #[serde(default)]
    flex_shrink: Option<f32>,
}

#[derive(serde::Deserialize)]
struct RectValuesJson {
    #[serde(default)]
    top: Option<f32>,
    #[serde(default)]
    right: Option<f32>,
    #[serde(default)]
    bottom: Option<f32>,
    #[serde(default)]
    left: Option<f32>,
    #[serde(default)]
    horizontal: Option<f32>,
    #[serde(default)]
    vertical: Option<f32>,
    #[serde(default)]
    all: Option<f32>,
}

#[derive(serde::Deserialize)]
struct GapJson {
    #[serde(default)]
    width: Option<f32>,
    #[serde(default)]
    height: Option<f32>,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum SizingJson {
    Points(f32),
    Percent(f32),
    Auto,
}

// ─── Memory ──────────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_free_string(s: *mut std::ffi::c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_free_bytes(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe {
            drop(Box::from_raw(ptr));
        }
    }
}

// ─── Engine ──────────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_create(width: u32, height: u32) -> u64 {
    let state = EngineState {
        engine: Engine::new(),
        renderer: Renderer::new(width.max(1).min(9999) as u16, height.max(1).min(9999) as u16),
        id_map: HashMap::new(),
    };
    ENGINES.lock().unwrap().insert(state)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_destroy(handle: u64) {
    ENGINES.lock().unwrap().remove(handle);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_process_commands(
    handle: u64,
    json_ptr: *const u8,
    json_len: u32,
) -> *mut std::ffi::c_char {
    let mut store = ENGINES.lock().unwrap();
    let state = match store.get_mut(handle) {
        Some(s) => s,
        None => {
            return json_string(&serde_json::json!({
                "success": 0,
                "errors": ["invalid engine handle"],
                "idMappings": []
            }));
        }
    };
    let json_str = str_from_parts(json_ptr, json_len);
    let commands: Vec<CommandJson> = match serde_json::from_str(json_str) {
        Ok(c) => c,
        Err(e) => {
            return json_string(&serde_json::json!({
                "success": 0,
                "errors": [format!("JSON parse error: {}", e)],
                "idMappings": []
            }));
        }
    };

    let mut id_mappings = Vec::new();
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
                if let Some(rust_cmd) = convert_command(other) {
                    deferred.push(rust_cmd);
                }
            }
        }
    }

    let resolve = |id: NodeId| -> NodeId {
        let raw = node_id_u64(id);
        state.id_map.get(&raw).map_or(id, |&real| node_id(real))
    };
    let resolved: Vec<Command> = deferred.into_iter().map(|cmd| resolve_command_ids(cmd, &resolve)).collect();
    let result = state.engine.process_commands(resolved);

    json_string(&serde_json::json!({
        "success": result.processed,
        "errors": result.errors.iter().map(|e| format!("{}", e)).collect::<Vec<_>>(),
        "idMappings": id_mappings
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_begin_frame(handle: u64) {
    if let Some(state) = ENGINES.lock().unwrap().get_mut(handle) {
        state.engine.begin_frame();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_commit_frame(handle: u64) {
    if let Some(state) = ENGINES.lock().unwrap().get_mut(handle) {
        state.engine.commit_frame();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_render(handle: u64) -> *mut std::ffi::c_char {
    let mut store = ENGINES.lock().unwrap();
    let state = match store.get_mut(handle) {
        Some(s) => s,
        None => return CString::new("{}").unwrap().into_raw(),
    };
    let frame = state.renderer.render(state.engine.arena_mut());
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(&frame.output_data);
    json_string(&serde_json::json!({
        "outputData": data_b64,
        "width": frame.width,
        "height": frame.height,
        "dirtyRegionCount": frame.dirty_regions.len(),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_render_full(handle: u64) -> *mut std::ffi::c_char {
    let mut store = ENGINES.lock().unwrap();
    let state = match store.get_mut(handle) {
        Some(s) => s,
        None => return CString::new("{}").unwrap().into_raw(),
    };
    let frame = state.renderer.render_full(state.engine.arena_mut());
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(&frame.output_data);
    json_string(&serde_json::json!({
        "outputData": data_b64,
        "width": frame.width,
        "height": frame.height,
        "dirtyRegionCount": frame.dirty_regions.len(),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_node_count(handle: u64) -> u32 {
    ENGINES.lock().unwrap().get(handle).map_or(0, |s| s.engine.node_count() as u32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_frame_count(handle: u64) -> u64 {
    ENGINES.lock().unwrap().get(handle).map_or(0, |s| s.engine.frame_count())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_create_node(handle: u64, kind_ptr: *const u8, kind_len: u32) -> u64 {
    ENGINES.lock().unwrap().get_mut(handle).map_or(0, |state| {
        let id = state.engine.create_node(parse_node_kind(str_from_parts(kind_ptr, kind_len)));
        node_id_u64(id)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_append_child(handle: u64, parent: u64, child: u64) -> i32 {
    ENGINES.lock().unwrap().get_mut(handle).map_or(-1, |state| {
        match state.engine.append_child(node_id(parent), node_id(child)) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_remove_node(handle: u64, id: u64) {
    if let Some(state) = ENGINES.lock().unwrap().get_mut(handle) {
        state.engine.remove_node(node_id(id));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_set_text(handle: u64, id: u64, text_ptr: *const u8, text_len: u32) {
    if let Some(state) = ENGINES.lock().unwrap().get_mut(handle) {
        state.engine.set_text(node_id(id), str_from_parts(text_ptr, text_len).to_string());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_resize(handle: u64, width: u32, height: u32) {
    if let Some(state) = ENGINES.lock().unwrap().get_mut(handle) {
        state.renderer.resize(width.max(1).min(9999) as u16, height.max(1).min(9999) as u16);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_shutdown(handle: u64) {
    if let Some(state) = ENGINES.lock().unwrap().get_mut(handle) {
        let _ = state.engine.process_command(Command::Shutdown);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_print_tree(handle: u64) -> *mut std::ffi::c_char {
    ENGINES.lock().unwrap().get(handle).map_or(CString::new("").unwrap().into_raw(), |s| {
        CString::new(s.engine.print_tree()).unwrap_or_default().into_raw()
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_root(handle: u64) -> u64 {
    ENGINES.lock().unwrap().get(handle).map_or(0, |s| node_id_u64(s.engine.arena().root()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_validate(handle: u64) -> i32 {
    ENGINES.lock().unwrap().get(handle).map_or(-1, |s| match s.engine.validate() {
        Ok(()) => 0,
        Err(_) => 1,
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_generation(handle: u64) -> u64 {
    ENGINES.lock().unwrap().get(handle).map_or(0, |s| s.engine.arena().generation())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_dimensions(handle: u64, width: *mut u32, height: *mut u32) -> i32 {
    ENGINES.lock().unwrap().get(handle).map_or(-1, |s| {
        let (w, h) = s.renderer.dimensions();
        unsafe {
            *width = w as u32;
        }
        unsafe {
            *height = h as u32;
        }
        0
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_request_frame(handle: u64) {
    if let Some(state) = ENGINES.lock().unwrap().get_mut(handle) {
        state.renderer.request_frame();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_tree_summary(handle: u64) -> *mut std::ffi::c_char {
    ENGINES.lock().unwrap().get(handle).map_or(CString::new("").unwrap().into_raw(), |s| {
        CString::new(s.engine.tree_summary()).unwrap_or_default().into_raw()
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_engine_should_render(handle: u64) -> *mut std::ffi::c_char {
    ENGINES.lock().unwrap().get(handle).map_or(CString::new("idle").unwrap().into_raw(), |s| {
        let status = match s.renderer.should_render() {
            FrameStatus::Idle => "idle",
            FrameStatus::Pending => "pending",
            FrameStatus::Due => "due",
            FrameStatus::Overdue => "overdue",
        };
        CString::new(status).unwrap().into_raw()
    })
}

// ─── EventBus ────────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_event_bus_create() -> u64 {
    EVENT_BUSES.lock().unwrap().insert(EventQueue::new())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_event_bus_destroy(handle: u64) {
    EVENT_BUSES.lock().unwrap().remove(handle);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_event_bus_push_key(
    handle: u64,
    key_ptr: *const u8,
    key_len: u32,
    ctrl: i32,
    shift: i32,
    alt: i32,
    target: u64,
) {
    if let Some(bus) = EVENT_BUSES.lock().unwrap().get_mut(handle) {
        bus.push_key(
            parse_key_str(str_from_parts(key_ptr, key_len)),
            Modifiers { ctrl: ctrl != 0, shift: shift != 0, alt: alt != 0, meta: false },
            node_id(target),
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_event_bus_push_mouse(
    handle: u64,
    button_ptr: *const u8,
    button_len: u32,
    x: u32,
    y: u32,
    target: u64,
) {
    if let Some(bus) = EVENT_BUSES.lock().unwrap().get_mut(handle) {
        let btn = match str_from_parts(button_ptr, button_len) {
            "left" => MouseButton::Left,
            "right" => MouseButton::Right,
            "middle" => MouseButton::Middle,
            "scroll_up" => MouseButton::ScrollUp,
            "scroll_down" => MouseButton::ScrollDown,
            _ => MouseButton::None,
        };
        bus.push_mouse(btn, Point::new(x as u16, y as u16), node_id(target));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_event_bus_push_mouse_motion(handle: u64, x: u32, y: u32, target: u64) {
    if let Some(bus) = EVENT_BUSES.lock().unwrap().get_mut(handle) {
        bus.push_mouse(MouseButton::None, Point::new(x as u16, y as u16), node_id(target));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_event_bus_push_resize(
    handle: u64,
    width: u32,
    height: u32,
    prev_width: u32,
    prev_height: u32,
) {
    if let Some(bus) = EVENT_BUSES.lock().unwrap().get_mut(handle) {
        bus.push_resize(width as u16, height as u16, prev_width as u16, prev_height as u16);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_event_bus_drain(handle: u64) -> *mut std::ffi::c_char {
    EVENT_BUSES.lock().unwrap().get_mut(handle).map_or(CString::new("[]").unwrap().into_raw(), |bus| {
        let events: Vec<serde_json::Value> = bus.drain().iter().map(event_to_json).collect();
        json_string(&serde_json::Value::Array(events))
    })
}

fn event_to_json(e: &crate::event_bus::Event) -> serde_json::Value {
    match e {
        crate::event_bus::Event::Key(ke) => serde_json::json!({
            "type": "key",
            "key": format!("{:?}", ke.key).to_lowercase(),
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_event_bus_len(handle: u64) -> u32 {
    EVENT_BUSES.lock().unwrap().get(handle).map_or(0, |bus| bus.len() as u32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_event_bus_push_paste(handle: u64, text_ptr: *const u8, text_len: u32, target: u64) {
    if let Some(bus) = EVENT_BUSES.lock().unwrap().get_mut(handle) {
        bus.push_paste(str_from_parts(text_ptr, text_len).to_string(), node_id(target));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_event_bus_is_empty(handle: u64) -> i32 {
    EVENT_BUSES.lock().unwrap().get(handle).map_or(1, |bus| bus.is_empty() as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_event_bus_clear(handle: u64) {
    if let Some(bus) = EVENT_BUSES.lock().unwrap().get_mut(handle) {
        bus.clear();
    }
}

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

// ─── FocusManager ────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_create() -> u64 {
    FOCUS_MANAGERS.lock().unwrap().insert(FocusManager::new())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_destroy(handle: u64) {
    FOCUS_MANAGERS.lock().unwrap().remove(handle);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_focus(handle: u64, id: u64) -> i32 {
    FOCUS_MANAGERS.lock().unwrap().get_mut(handle).map_or(-1, |m| m.focus(node_id(id)).is_some() as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_blur(handle: u64, id: u64) -> i32 {
    FOCUS_MANAGERS.lock().unwrap().get_mut(handle).map_or(-1, |m| m.blur(node_id(id)).is_some() as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_blur_current(handle: u64) -> i32 {
    FOCUS_MANAGERS
        .lock()
        .unwrap()
        .get_mut(handle)
        .map_or(-1, |m| m.focused().map_or(0, |focused| m.blur(focused).is_some() as i32))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_focused(handle: u64) -> u64 {
    FOCUS_MANAGERS.lock().unwrap().get(handle).map_or(0, |m| m.focused().map(node_id_u64).unwrap_or(0))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_is_focused(handle: u64, id: u64) -> i32 {
    FOCUS_MANAGERS.lock().unwrap().get(handle).map_or(0, |m| m.is_focused(node_id(id)) as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_traverse(handle: u64, dir_ptr: *const u8, dir_len: u32) -> u64 {
    FOCUS_MANAGERS.lock().unwrap().get_mut(handle).map_or(0, |m| {
        let dir = match str_from_parts(dir_ptr, dir_len) {
            "forward" | "next" => FocusDirection::Forward,
            "backward" | "previous" | "prev" => FocusDirection::Backward,
            "first" => FocusDirection::First,
            "last" => FocusDirection::Last,
            "up" => FocusDirection::Up,
            "down" => FocusDirection::Down,
            "left" => FocusDirection::Left,
            "right" => FocusDirection::Right,
            _ => FocusDirection::Forward,
        };
        FocusTraversal::traverse(m, dir)
            .map(|id| {
                m.focus(id);
                node_id_u64(id)
            })
            .unwrap_or(0)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_focus_order(handle: u64) -> *mut std::ffi::c_char {
    FOCUS_MANAGERS.lock().unwrap().get(handle).map_or(CString::new("[]").unwrap().into_raw(), |m| {
        let order: Vec<u64> = m.tab_order().iter().copied().map(node_id_u64).collect();
        json_string(&serde_json::json!(order))
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_set_scope(handle: u64, scope_id: u64) {
    if let Some(m) = FOCUS_MANAGERS.lock().unwrap().get_mut(handle) {
        m.push_scope(FocusScope::new(node_id(scope_id), FocusScopeType::Window));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_clear_scope(handle: u64) {
    if let Some(m) = FOCUS_MANAGERS.lock().unwrap().get_mut(handle) {
        m.pop_scope();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_scope_id(handle: u64) -> u64 {
    FOCUS_MANAGERS.lock().unwrap().get(handle).map_or(0, |m| m.current_scope().map(|s| node_id_u64(s.id)).unwrap_or(0))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_focus_manager_focused_in_scope(handle: u64) -> u64 {
    FOCUS_MANAGERS.lock().unwrap().get(handle).map_or(0, |m| m.focused().map(node_id_u64).unwrap_or(0))
}

// ─── TextEngine ──────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_create(text_ptr: *const u8, text_len: u32) -> u64 {
    let text = str_from_parts(text_ptr, text_len);
    let engine = if text.is_empty() { TextEngine::default() } else { TextEngine::with_text(text) };
    TEXT_ENGINES.lock().unwrap().insert(engine)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_destroy(handle: u64) {
    TEXT_ENGINES.lock().unwrap().remove(handle);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_insert_char(handle: u64, ch: u32) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        if let Some(c) = char::from_u32(ch) {
            te.insert_char(c);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_insert_str(handle: u64, text_ptr: *const u8, text_len: u32) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        te.insert_str(str_from_parts(text_ptr, text_len));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_delete_char(handle: u64) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        te.delete_char();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_delete_char_forward(handle: u64) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        let pos = te.cursor().position();
        if pos < te.char_count() {
            te.buffer_mut().delete_char(pos);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_cursor_left(handle: u64) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        if te.cursor().position() > 0 {
            te.cursor_mut().move_left();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_cursor_right(handle: u64) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        if te.cursor().position() < te.char_count() {
            te.cursor_mut().move_right();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_length(handle: u64) -> u32 {
    TEXT_ENGINES.lock().unwrap().get(handle).map_or(0, |te| te.char_count() as u32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_get_text(handle: u64) -> *mut std::ffi::c_char {
    TEXT_ENGINES.lock().unwrap().get(handle).map_or(CString::new("").unwrap().into_raw(), |te| {
        let lines: Vec<String> = (0..te.line_count()).filter_map(|i| te.line(i)).collect();
        CString::new(lines.join("\n")).unwrap_or_default().into_raw()
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_cursor_position(handle: u64) -> u32 {
    TEXT_ENGINES.lock().unwrap().get(handle).map_or(0, |te| te.cursor().position() as u32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_can_undo(handle: u64) -> i32 {
    TEXT_ENGINES.lock().unwrap().get(handle).map_or(0, |te| te.undo_manager().can_undo() as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_can_redo(handle: u64) -> i32 {
    TEXT_ENGINES.lock().unwrap().get(handle).map_or(0, |te| te.undo_manager().can_redo() as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_line_count(handle: u64) -> u32 {
    TEXT_ENGINES.lock().unwrap().get(handle).map_or(0, |te| te.line_count() as u32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_is_empty(handle: u64) -> i32 {
    TEXT_ENGINES.lock().unwrap().get(handle).map_or(1, |te| if te.char_count() == 0 { 1 } else { 0 })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_clear(handle: u64) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        te.clear();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_undo(handle: u64) -> i32 {
    TEXT_ENGINES.lock().unwrap().get_mut(handle).map_or(0, |te| te.undo() as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_redo(handle: u64) -> i32 {
    TEXT_ENGINES.lock().unwrap().get_mut(handle).map_or(0, |te| te.redo() as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_set_cursor_position(handle: u64, pos: u32) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        let max = te.char_count();
        te.cursor_mut().set_position((pos as usize).min(max));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_insert_at(handle: u64, pos: u32, text_ptr: *const u8, text_len: u32) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        let text = str_from_parts(text_ptr, text_len);
        te.buffer_mut().insert_str(pos as usize, text);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_delete_at(handle: u64, pos: u32, len: u32) -> i32 {
    TEXT_ENGINES.lock().unwrap().get_mut(handle).map_or(0, |te| {
        let end = pos as usize + len as usize;
        if end > te.char_count() {
            0
        } else {
            te.buffer_mut().delete_range(pos as usize, end);
            1
        }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_cursor_up(handle: u64) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        let line_len = te.line_length(te.cursor().line()).unwrap_or(0);
        te.cursor_mut().move_up(line_len);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_cursor_down(handle: u64) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        let line_len = te.line_length(te.cursor().line()).unwrap_or(0);
        te.cursor_mut().move_down(line_len);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_cursor_line_start(handle: u64) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        let pos = te.line_to_char(te.cursor().line());
        te.cursor_mut().set_position(pos);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_cursor_line_end(handle: u64) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        let line = te.cursor().line();
        let pos = te.line_to_char(line) + te.line_length(line).unwrap_or(0);
        te.cursor_mut().set_position(pos);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_delete_word_backward(handle: u64) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        let pos = te.cursor().position();
        let new_pos = te.buffer().word_boundary_left(pos);
        if new_pos < pos {
            te.buffer_mut().delete_range(new_pos, pos);
            te.cursor_mut().set_position(new_pos);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_text_engine_delete_word_forward(handle: u64) {
    if let Some(te) = TEXT_ENGINES.lock().unwrap().get_mut(handle) {
        let pos = te.cursor().position();
        let new_pos = te.buffer().word_boundary_right(pos);
        if new_pos > pos {
            te.buffer_mut().delete_range(pos, new_pos);
        }
    }
}

// ─── Scheduler ───────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_create(fps: u32) -> u64 {
    let s = if fps > 0 { Scheduler::with_fps(fps) } else { Scheduler::default() };
    SCHEDULERS.lock().unwrap().insert(s)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_destroy(handle: u64) {
    SCHEDULERS.lock().unwrap().remove(handle);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_request_frame(handle: u64) {
    if let Some(s) = SCHEDULERS.lock().unwrap().get_mut(handle) {
        s.request_frame();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_begin_frame(handle: u64) -> i32 {
    SCHEDULERS.lock().unwrap().get_mut(handle).map_or(0, |s| s.begin_frame() as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_end_frame(handle: u64) {
    if let Some(s) = SCHEDULERS.lock().unwrap().get_mut(handle) {
        s.end_frame();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_should_render(handle: u64) -> *mut std::ffi::c_char {
    SCHEDULERS.lock().unwrap().get(handle).map_or(CString::new("idle").unwrap().into_raw(), |s| {
        let status = match s.status() {
            FrameStatus::Idle => "idle",
            FrameStatus::Pending => "pending",
            FrameStatus::Due => "due",
            FrameStatus::Overdue => "overdue",
        };
        CString::new(status).unwrap().into_raw()
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_is_idle(handle: u64) -> i32 {
    SCHEDULERS.lock().unwrap().get(handle).map_or(1, |s| matches!(s.status(), FrameStatus::Idle) as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_request_render_coalesced(handle: u64) {
    if let Some(s) = SCHEDULERS.lock().unwrap().get_mut(handle) {
        s.request_render_coalesced();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_request_render_immediate(handle: u64) {
    if let Some(s) = SCHEDULERS.lock().unwrap().get_mut(handle) {
        s.request_render_immediate();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_has_scheduled_frame(handle: u64) -> i32 {
    SCHEDULERS.lock().unwrap().get(handle).map_or(0, |s| s.has_scheduled_frame() as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_is_rendering(handle: u64) -> i32 {
    SCHEDULERS.lock().unwrap().get(handle).map_or(0, |s| s.is_rendering() as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_begin_render(handle: u64) {
    if let Some(s) = SCHEDULERS.lock().unwrap().get_mut(handle) {
        s.begin_render();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_end_render(handle: u64) -> i32 {
    SCHEDULERS.lock().unwrap().get_mut(handle).map_or(0, |s| s.end_render() as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_frame_count(handle: u64) -> u64 {
    SCHEDULERS.lock().unwrap().get(handle).map_or(0, |s| s.frame_count())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_dropped_frames(handle: u64) -> u64 {
    SCHEDULERS.lock().unwrap().get(handle).map_or(0, |s| s.dropped_frames())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_fps(handle: u64) -> f64 {
    SCHEDULERS.lock().unwrap().get(handle).map_or(0.0, |s| {
        let interval = s.frame_budget().target_frame_time;
        if interval.is_zero() { 0.0 } else { 1000.0 / interval.as_millis() as f64 }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_scheduler_frame_budget_ms(handle: u64) -> f64 {
    SCHEDULERS.lock().unwrap().get(handle).map_or(0.0, |s| s.frame_budget().target_frame_time.as_millis() as f64)
}

// ─── Keymap ──────────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_create() -> u64 {
    KEYMAPS.lock().unwrap().insert(Keymap::new())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_destroy(handle: u64) {
    KEYMAPS.lock().unwrap().remove(handle);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_add_binding(
    handle: u64,
    layer_ptr: *const u8,
    layer_len: u32,
    id_ptr: *const u8,
    id_len: u32,
    keys_ptr: *const u8,
    keys_len: u32,
    cmd_ptr: *const u8,
    cmd_len: u32,
    desc_ptr: *const u8,
    desc_len: u32,
    priority: i32,
) -> i32 {
    KEYMAPS.lock().unwrap().get_mut(handle).map_or(-1, |km| {
        let keys_str = str_from_parts(keys_ptr, keys_len);
        match KeyParser::parse_sequence(keys_str) {
            Ok(seq) => {
                let binding = KeyBinding {
                    id: str_from_parts(id_ptr, id_len).to_string(),
                    command: str_from_parts(cmd_ptr, cmd_len).to_string(),
                    sequence: seq,
                    description: if desc_len > 0 { Some(str_from_parts(desc_ptr, desc_len).to_string()) } else { None },
                    condition: None,
                    enabled: true,
                };
                km.add_binding_to_layer(str_from_parts(layer_ptr, layer_len), binding, priority);
                0
            }
            Err(_) => 1,
        }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_set_mode(handle: u64, mode_ptr: *const u8, mode_len: u32) {
    if let Some(km) = KEYMAPS.lock().unwrap().get_mut(handle) {
        km.set_mode(str_from_parts(mode_ptr, mode_len).to_string());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_current_mode(handle: u64) -> *mut std::ffi::c_char {
    KEYMAPS.lock().unwrap().get(handle).map_or(CString::new("").unwrap().into_raw(), |km| {
        CString::new(km.current_mode().unwrap_or("")).unwrap().into_raw()
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_handle_key(handle: u64, key_ptr: *const u8, key_len: u32) -> *mut std::ffi::c_char {
    KEYMAPS.lock().unwrap().get_mut(handle).map_or(CString::new("").unwrap().into_raw(), |km| {
        let key_str = str_from_parts(key_ptr, key_len);
        match KeyParser::parse_combo(key_str) {
            Ok(combo) => {
                let event = KeyEvent {
                    key: combo.key,
                    modifiers: combo.modifiers,
                    target: NodeId::default(),
                    default_prevented: false,
                    phase: EventPhase::Target,
                };
                match km.handle_event(&event) {
                    Some(cmd) => CString::new(cmd).unwrap().into_raw(),
                    None => CString::new("").unwrap().into_raw(),
                }
            }
            Err(_) => CString::new("").unwrap().into_raw(),
        }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_has_pending(handle: u64) -> i32 {
    KEYMAPS.lock().unwrap().get(handle).map_or(0, |km| km.has_pending_sequence() as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_clear_pending(handle: u64) {
    if let Some(km) = KEYMAPS.lock().unwrap().get_mut(handle) {
        km.clear_pending_sequence();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_remove_layer(handle: u64, name_ptr: *const u8, name_len: u32) -> i32 {
    KEYMAPS.lock().unwrap().get_mut(handle).map_or(-1, |km| km.remove_layer(str_from_parts(name_ptr, name_len)) as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_clear_mode(handle: u64) {
    if let Some(km) = KEYMAPS.lock().unwrap().get_mut(handle) {
        km.clear_mode();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_set_chord_timeout(handle: u64, ms: u64) {
    if let Some(km) = KEYMAPS.lock().unwrap().get_mut(handle) {
        km.set_chord_timeout(ms);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_chord_timeout_ms(handle: u64) -> u64 {
    KEYMAPS.lock().unwrap().get(handle).map_or(0, |km| km.chord_timeout_ms())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_command_history(handle: u64) -> *mut std::ffi::c_char {
    KEYMAPS.lock().unwrap().get(handle).map_or(CString::new("[]").unwrap().into_raw(), |km| {
        let history: Vec<String> = km.command_history().to_vec();
        json_string(&serde_json::json!(history))
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_clear_history(handle: u64) {
    if let Some(km) = KEYMAPS.lock().unwrap().get_mut(handle) {
        km.clear_history();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_active_bindings(handle: u64) -> *mut std::ffi::c_char {
    KEYMAPS.lock().unwrap().get(handle).map_or(CString::new("[]").unwrap().into_raw(), |km| {
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
        json_string(&serde_json::Value::Array(bindings))
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_parse_key(_handle: u64, key_ptr: *const u8, key_len: u32) -> *mut std::ffi::c_char {
    let key_str = str_from_parts(key_ptr, key_len);
    match crate::input::KeyParser::parse_combo(key_str) {
        Ok(_) => CString::new(key_str).unwrap().into_raw(),
        Err(_) => CString::new("").unwrap().into_raw(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_keymap_parse_sequence(
    _handle: u64,
    seq_ptr: *const u8,
    seq_len: u32,
) -> *mut std::ffi::c_char {
    let seq_str = str_from_parts(seq_ptr, seq_len);
    match crate::input::KeyParser::parse_sequence(seq_str) {
        Ok(seq) => {
            let parts: Vec<String> = seq.keys.iter().map(|chord| format!("{:?}", chord)).collect();
            json_string(&serde_json::json!(parts))
        }
        Err(_) => CString::new("[]").unwrap().into_raw(),
    }
}

// ─── Global Functions ────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_get_version() -> *mut std::ffi::c_char {
    CString::new(VERSION).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_detect_capabilities() -> *mut std::ffi::c_char {
    let caps = crate::terminal::global_capabilities();
    let (w, h) = caps.terminal_size();
    let pixel = caps.pixel_size();
    let features = caps.features();
    json_string(&serde_json::json!({
        "brand": format!("{:?}", caps.brand()),
        "trueColor": caps.supports_true_color(),
        "kittyKeyboard": caps.supports_kitty_keyboard(),
        "csi_u": caps.supports_csi_u(),
        "bracketedPaste": caps.supports_bracketed_paste(),
        "focusEvents": caps.supports_focus_events(),
        "mouse": caps.input.mouse_modes.normal_mouse,
        "osc52": caps.supports_osc52(),
        "osc8": caps.supports_osc8(),
        "sync": features.synchronized_output,
        "sgrPixel": caps.supports_kitty_graphics(),
        "underlineColor": features.underline_color,
        "strikethrough": features.strikethrough,
        "cursorStyle": features.cursor_style,
        "alternateScroll": features.alternate_scroll,
        "inlineImages": caps.supports_iterm_images(),
        "sixel": caps.supports_sixel(),
        "terminalSize": { "columns": w, "rows": h },
        "pixelSize": pixel.map(|(pw, ph)| serde_json::json!({ "width": pw, "height": ph })),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_highlight_code(
    code_ptr: *const u8,
    code_len: u32,
    lang_ptr: *const u8,
    lang_len: u32,
) -> *mut std::ffi::c_char {
    let code = str_from_parts(code_ptr, code_len);
    let lang = str_from_parts(lang_ptr, lang_len);
    let mut hl = global_highlighter().lock().unwrap();
    let lines = hl.highlight(code, lang);
    let result = match lines {
        Some(lines) => {
            let json_lines: Vec<serde_json::Value> = lines
                .iter()
                .map(|line| {
                    let segments: Vec<serde_json::Value> = line
                        .segments
                        .iter()
                        .map(|seg| {
                            serde_json::json!({
                                "text": seg.text,
                                "fg": seg.style.fg.map(|c| c.to_rgba(255).to_hex()),
                                "bg": seg.style.bg.map(|c| c.to_rgba(255).to_hex()),
                                "bold": seg.style.bold,
                                "italic": seg.style.italic,
                                "underline": seg.style.underline,
                                "dim": seg.style.dim,
                                "strikethrough": seg.style.strikethrough,
                            })
                        })
                        .collect();
                    serde_json::json!(segments)
                })
                .collect();
            serde_json::to_string(&json_lines).unwrap_or_default()
        }
        None => "[]".to_string(),
    };
    CString::new(result).unwrap_or_default().into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_create_dark_theme() -> *mut std::ffi::c_char {
    let t = Theme::dark();
    json_string(&theme_to_json(&t))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_create_light_theme() -> *mut std::ffi::c_char {
    let t = Theme::light();
    json_string(&theme_to_json(&t))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffi_create_default_theme() -> *mut std::ffi::c_char {
    let t = Theme::default();
    json_string(&theme_to_json(&t))
}

fn theme_to_json(t: &Theme) -> serde_json::Value {
    serde_json::json!({
        "name": t.name.clone(),
        "colors": {
            "background": theme_color_hex(t.colors.background),
            "surface": theme_color_hex(t.colors.surface),
            "surfaceHigh": theme_color_hex(t.colors.surface_high),
            "surfaceLow": theme_color_hex(t.colors.surface_low),
            "primary": theme_color_hex(t.colors.primary),
            "primaryForeground": theme_color_hex(t.colors.primary_foreground),
            "secondary": theme_color_hex(t.colors.secondary),
            "secondaryForeground": theme_color_hex(t.colors.secondary_foreground),
            "text": theme_color_hex(t.colors.text),
            "textMuted": theme_color_hex(t.colors.text_muted),
            "textDim": theme_color_hex(t.colors.text_dim),
            "border": theme_color_hex(t.colors.border),
            "borderFocused": theme_color_hex(t.colors.border_focused),
            "accent": theme_color_hex(t.colors.accent),
            "accentForeground": theme_color_hex(t.colors.accent_foreground),
            "error": theme_color_hex(t.colors.error),
            "warning": theme_color_hex(t.colors.warning),
            "success": theme_color_hex(t.colors.success),
            "info": theme_color_hex(t.colors.info),
            "scrollbar": theme_color_hex(t.colors.scrollbar),
            "scrollbarThumb": theme_color_hex(t.colors.scrollbar_thumb),
        },
        "spacing": {
            "none": t.spacing.none as u32,
            "xxs": t.spacing.xxs as u32,
            "xs": t.spacing.xs as u32,
            "sm": t.spacing.sm as u32,
            "md": t.spacing.md as u32,
            "lg": t.spacing.lg as u32,
            "xl": t.spacing.xl as u32,
            "xxl": t.spacing.xxl as u32,
        },
        "borders": {
            "style": format!("{:?}", t.borders.style),
            "fg": theme_color_hex(t.borders.fg),
        },
    })
}

// ─── Command Conversion ──────────────────────────────────────────────────────

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

fn convert_command(cj: CommandJson) -> Option<Command> {
    use CommandJson as C;
    match cj {
        C::RemoveNode { id } => Some(Command::RemoveNode { id: node_id(id) }),
        C::AppendChild { parent, child } => {
            Some(Command::AppendChild { parent: node_id(parent), child: node_id(child) })
        }
        C::InsertBefore { reference, child } => {
            Some(Command::InsertBefore { reference: node_id(reference), child: node_id(child) })
        }
        C::MoveNode { node, new_parent } => {
            Some(Command::MoveNode { node: node_id(node), new_parent: node_id(new_parent) })
        }
        C::ReplaceNode { old, new } => Some(Command::ReplaceNode { old: node_id(old), new: node_id(new) }),
        C::DetachNode { id } => Some(Command::DetachNode { id: node_id(id) }),
        C::SetText { id, text } => Some(Command::SetText { id: node_id(id), text }),
        C::SetStyle { id, style } => {
            let mut s = Style::default();
            if let Some(fg) = style.fg {
                s.fg = Some(fg.into());
            }
            if let Some(bg) = style.bg {
                s.bg = Some(bg.into());
            }
            if let Some(v) = style.bold {
                s.bold = Some(v);
            }
            if let Some(v) = style.italic {
                s.italic = Some(v);
            }
            if let Some(v) = style.underline {
                s.underline = Some(v);
            }
            if let Some(v) = style.strikethrough {
                s.strikethrough = Some(v);
            }
            if let Some(v) = style.dim {
                s.dim = Some(v);
            }
            if let Some(v) = style.inverse {
                s.inverse = Some(v);
            }
            if let Some(v) = style.text_align {
                s.text_align = Some(match v.as_str() {
                    "center" | "Center" => crate::text::TextAlign::Center,
                    "right" | "Right" => crate::text::TextAlign::Right,
                    "justify" | "Justify" => crate::text::TextAlign::Justify,
                    _ => crate::text::TextAlign::Left,
                });
            }
            Some(Command::SetStyle { id: node_id(id), style: s })
        }
        C::SetForeground { id, color } => Some(Command::SetForeground { id: node_id(id), color: color.into() }),
        C::SetBackground { id, color } => Some(Command::SetBackground { id: node_id(id), color: color.into() }),
        C::SetBold { id, value } => Some(Command::SetBold { id: node_id(id), value }),
        C::SetItalic { id, value } => Some(Command::SetItalic { id: node_id(id), value }),
        C::SetUnderline { id, value } => Some(Command::SetUnderline { id: node_id(id), value }),
        C::SetStrikethrough { id, value } => Some(Command::SetStrikethrough { id: node_id(id), value }),
        C::SetDim { id, value } => Some(Command::SetDim { id: node_id(id), value }),
        C::SetInverse { id, value } => Some(Command::SetInverse { id: node_id(id), value }),
        C::SetHidden { id, value } => Some(Command::SetHidden { id: node_id(id), value }),
        C::SetLayout { id, layout } => Some(Command::SetLayout { id: node_id(id), layout: layout.into() }),
        C::SetFlexDirection { id, direction } => {
            let dir = match direction.as_str() {
                "row" | "Row" => FlexDirection::Row,
                "column" | "Column" => FlexDirection::Column,
                "row_reverse" | "RowReverse" => FlexDirection::RowReverse,
                "column_reverse" | "ColumnReverse" => FlexDirection::ColumnReverse,
                _ => FlexDirection::Column,
            };
            Some(Command::SetFlexDirection { id: node_id(id), direction: dir })
        }
        C::SetJustifyContent { id, value } => {
            let v = match value.as_str() {
                "flex_start" | "FlexStart" => JustifyContent::FlexStart,
                "center" | "Center" => JustifyContent::Center,
                "flex_end" | "FlexEnd" => JustifyContent::FlexEnd,
                "space_between" | "SpaceBetween" => JustifyContent::SpaceBetween,
                "space_around" | "SpaceAround" => JustifyContent::SpaceAround,
                "space_evenly" | "SpaceEvenly" => JustifyContent::SpaceEvenly,
                _ => JustifyContent::FlexStart,
            };
            Some(Command::SetJustifyContent { id: node_id(id), value: v })
        }
        C::SetAlignItems { id, value } => {
            let v = match value.as_str() {
                "start" | "Start" | "flex_start" | "FlexStart" => AlignItems::FlexStart,
                "end" | "End" | "flex_end" | "FlexEnd" => AlignItems::FlexEnd,
                "center" | "Center" => AlignItems::Center,
                "stretch" | "Stretch" => AlignItems::Stretch,
                "baseline" | "Baseline" => AlignItems::Baseline,
                _ => AlignItems::Stretch,
            };
            Some(Command::SetAlignItems { id: node_id(id), value: v })
        }
        C::SetAlignSelf { id, value } => {
            let v = match value.as_str() {
                "start" | "Start" | "flex_start" | "FlexStart" => AlignSelf::FlexStart,
                "end" | "End" | "flex_end" | "FlexEnd" => AlignSelf::FlexEnd,
                "center" | "Center" => AlignSelf::Center,
                "stretch" | "Stretch" => AlignSelf::Stretch,
                "baseline" | "Baseline" => AlignSelf::Baseline,
                _ => AlignSelf::Stretch,
            };
            Some(Command::SetAlignSelf { id: node_id(id), value: v })
        }
        C::SetWidth { id, value } => Some(Command::SetWidth { id: node_id(id), value: value.into() }),
        C::SetHeight { id, value } => Some(Command::SetHeight { id: node_id(id), value: value.into() }),
        C::SetMinWidth { id, value } => Some(Command::SetMinWidth { id: node_id(id), value: value.into() }),
        C::SetMinHeight { id, value } => Some(Command::SetMinHeight { id: node_id(id), value: value.into() }),
        C::SetMaxWidth { id, value } => Some(Command::SetMaxWidth { id: node_id(id), value: value.into() }),
        C::SetMaxHeight { id, value } => Some(Command::SetMaxHeight { id: node_id(id), value: value.into() }),
        C::SetFlexBasis { id, value } => Some(Command::SetFlexBasis { id: node_id(id), value: value.into() }),
        C::SetPadding { id, value } => Some(Command::SetPadding { id: node_id(id), value: value.into() }),
        C::SetMargin { id, value } => Some(Command::SetMargin { id: node_id(id), value: value.into() }),
        C::SetGap { id, value } => Some(Command::SetGap { id: node_id(id), value: value.into() }),
        C::SetFlexGrow { id, value } => Some(Command::SetFlexGrow { id: node_id(id), value }),
        C::SetFlexShrink { id, value } => Some(Command::SetFlexShrink { id: node_id(id), value }),
        C::SetPosition { id, value } => {
            let v = match value.as_str() {
                "relative" | "Relative" => Position::Relative,
                "absolute" | "Absolute" => Position::Absolute,
                _ => Position::Relative,
            };
            Some(Command::SetPosition { id: node_id(id), value: v })
        }
        C::SetInset { id, value } => Some(Command::SetInset { id: node_id(id), value: value.into() }),
        C::SetDisplay { id, value } => {
            let v = match value.as_str() {
                "none" | "None" => VisibilityDisplay::None,
                _ => VisibilityDisplay::Flex,
            };
            Some(Command::SetDisplay { id: node_id(id), value: v })
        }
        C::SetOpacity { id, value } => Some(Command::SetOpacity { id: node_id(id), value }),
        C::SetClip { id, value } => Some(Command::SetClip { id: node_id(id), value }),
        C::SetZIndex { id, value } => Some(Command::SetZIndex { id: node_id(id), value }),
        C::SetOverflow { id, value } => {
            let v = match value.as_str() {
                "hidden" | "Hidden" => Overflow::Hidden,
                "scroll" | "Scroll" => Overflow::Scroll,
                _ => Overflow::Visible,
            };
            Some(Command::SetOverflow { id: node_id(id), value: v })
        }
        C::SetAttribute { id, key, value } => Some(Command::SetAttribute { id: node_id(id), key, value }),
        C::RemoveAttribute { id, key } => Some(Command::RemoveAttribute { id: node_id(id), key }),
        C::SetTranslateX { id, value } => Some(Command::SetTranslateX { id: node_id(id), value }),
        C::SetTranslateY { id, value } => Some(Command::SetTranslateY { id: node_id(id), value }),
        C::SetTabIndex { id, value } => Some(Command::SetTabIndex { id: node_id(id), value }),
        C::FocusNode { id } => Some(Command::FocusNode { id: node_id(id) }),
        C::BlurNode { id } => Some(Command::BlurNode { id: node_id(id) }),
        C::BeginFrame { frame_id } => Some(Command::BeginFrame { frame_id }),
        C::CommitFrame { frame_id } => Some(Command::CommitFrame { frame_id }),
        C::Invalidate { id } => Some(Command::Invalidate { id: node_id(id) }),
        C::Shutdown => Some(Command::Shutdown),
        C::CreateNode { .. } => None,
    }
}

fn resolve_command_ids<F>(cmd: Command, resolve: &F) -> Command
where
    F: Fn(NodeId) -> NodeId,
{
    use Command as Cmd;
    match cmd {
        Cmd::RemoveNode { id } => Cmd::RemoveNode { id: resolve(id) },
        Cmd::AppendChild { parent, child } => Cmd::AppendChild { parent: resolve(parent), child: resolve(child) },
        Cmd::InsertBefore { reference, child } => {
            Cmd::InsertBefore { reference: resolve(reference), child: resolve(child) }
        }
        Cmd::MoveNode { node, new_parent } => Cmd::MoveNode { node: resolve(node), new_parent: resolve(new_parent) },
        Cmd::ReplaceNode { old, new } => Cmd::ReplaceNode { old: resolve(old), new: resolve(new) },
        Cmd::DetachNode { id } => Cmd::DetachNode { id: resolve(id) },
        Cmd::SetStyle { id, style } => Cmd::SetStyle { id: resolve(id), style },
        Cmd::SetForeground { id, color } => Cmd::SetForeground { id: resolve(id), color },
        Cmd::SetBackground { id, color } => Cmd::SetBackground { id: resolve(id), color },
        Cmd::SetBold { id, value } => Cmd::SetBold { id: resolve(id), value },
        Cmd::SetItalic { id, value } => Cmd::SetItalic { id: resolve(id), value },
        Cmd::SetUnderline { id, value } => Cmd::SetUnderline { id: resolve(id), value },
        Cmd::SetStrikethrough { id, value } => Cmd::SetStrikethrough { id: resolve(id), value },
        Cmd::SetDim { id, value } => Cmd::SetDim { id: resolve(id), value },
        Cmd::SetInverse { id, value } => Cmd::SetInverse { id: resolve(id), value },
        Cmd::SetHidden { id, value } => Cmd::SetHidden { id: resolve(id), value },
        Cmd::SetLayout { id, layout } => Cmd::SetLayout { id: resolve(id), layout },
        Cmd::SetFlexDirection { id, direction } => Cmd::SetFlexDirection { id: resolve(id), direction },
        Cmd::SetJustifyContent { id, value } => Cmd::SetJustifyContent { id: resolve(id), value },
        Cmd::SetAlignItems { id, value } => Cmd::SetAlignItems { id: resolve(id), value },
        Cmd::SetAlignSelf { id, value } => Cmd::SetAlignSelf { id: resolve(id), value },
        Cmd::SetWidth { id, value } => Cmd::SetWidth { id: resolve(id), value },
        Cmd::SetHeight { id, value } => Cmd::SetHeight { id: resolve(id), value },
        Cmd::SetMinWidth { id, value } => Cmd::SetMinWidth { id: resolve(id), value },
        Cmd::SetMinHeight { id, value } => Cmd::SetMinHeight { id: resolve(id), value },
        Cmd::SetMaxWidth { id, value } => Cmd::SetMaxWidth { id: resolve(id), value },
        Cmd::SetMaxHeight { id, value } => Cmd::SetMaxHeight { id: resolve(id), value },
        Cmd::SetPadding { id, value } => Cmd::SetPadding { id: resolve(id), value },
        Cmd::SetMargin { id, value } => Cmd::SetMargin { id: resolve(id), value },
        Cmd::SetGap { id, value } => Cmd::SetGap { id: resolve(id), value },
        Cmd::SetFlexGrow { id, value } => Cmd::SetFlexGrow { id: resolve(id), value },
        Cmd::SetFlexShrink { id, value } => Cmd::SetFlexShrink { id: resolve(id), value },
        Cmd::SetFlexBasis { id, value } => Cmd::SetFlexBasis { id: resolve(id), value },
        Cmd::SetPosition { id, value } => Cmd::SetPosition { id: resolve(id), value },
        Cmd::SetInset { id, value } => Cmd::SetInset { id: resolve(id), value },
        Cmd::SetText { id, text } => Cmd::SetText { id: resolve(id), text },
        Cmd::SetAttribute { id, key, value } => Cmd::SetAttribute { id: resolve(id), key, value },
        Cmd::RemoveAttribute { id, key } => Cmd::RemoveAttribute { id: resolve(id), key },
        Cmd::SetDisplay { id, value } => Cmd::SetDisplay { id: resolve(id), value },
        Cmd::SetOpacity { id, value } => Cmd::SetOpacity { id: resolve(id), value },
        Cmd::SetClip { id, value } => Cmd::SetClip { id: resolve(id), value },
        Cmd::SetTranslateX { id, value } => Cmd::SetTranslateX { id: resolve(id), value },
        Cmd::SetTranslateY { id, value } => Cmd::SetTranslateY { id: resolve(id), value },
        Cmd::SetZIndex { id, value } => Cmd::SetZIndex { id: resolve(id), value },
        Cmd::SetOverflow { id, value } => Cmd::SetOverflow { id: resolve(id), value },
        Cmd::FocusNode { id } => Cmd::FocusNode { id: resolve(id) },
        Cmd::BlurNode { id } => Cmd::BlurNode { id: resolve(id) },
        Cmd::SetTabIndex { id, value } => Cmd::SetTabIndex { id: resolve(id), value },
        Cmd::Invalidate { id } => Cmd::Invalidate { id: resolve(id) },
        other => other,
    }
}

// ─── Conversions ─────────────────────────────────────────────────────────────

impl From<ColorJson> for Color {
    fn from(c: ColorJson) -> Self {
        match c {
            ColorJson::Named(name) => match name.to_lowercase().as_str() {
                "red" => Color::Named(NamedColor::Red),
                "green" => Color::Named(NamedColor::Green),
                "blue" => Color::Named(NamedColor::Blue),
                "yellow" => Color::Named(NamedColor::Yellow),
                "cyan" => Color::Named(NamedColor::Cyan),
                "magenta" => Color::Named(NamedColor::Magenta),
                "white" => Color::Named(NamedColor::White),
                "black" => Color::Named(NamedColor::Black),
                "dark_gray" | "darkgray" => Color::Named(NamedColor::BrightBlack),
                "light_gray" | "lightgray" => Color::Named(NamedColor::BrightWhite),
                "light_red" | "lightred" => Color::Named(NamedColor::BrightRed),
                "light_green" | "lightgreen" => Color::Named(NamedColor::BrightGreen),
                "light_blue" | "lightblue" => Color::Named(NamedColor::BrightBlue),
                "light_yellow" | "lightyellow" => Color::Named(NamedColor::BrightYellow),
                "light_cyan" | "lightcyan" => Color::Named(NamedColor::BrightCyan),
                "light_magenta" | "lightmagenta" => Color::Named(NamedColor::BrightMagenta),
                _ => Color::parse(&name).unwrap_or(Color::Named(NamedColor::White)),
            },
            ColorJson::Rgb { r, g, b } => Color::Rgb { r, g, b },
        }
    }
}

impl From<LayoutJson> for LayoutProps {
    fn from(l: LayoutJson) -> Self {
        let mut props = LayoutProps::default();
        if let Some(dir) = l.direction {
            props.direction = match dir.as_str() {
                "row" | "Row" => FlexDirection::Row,
                "column" | "Column" => FlexDirection::Column,
                "row_reverse" | "RowReverse" => FlexDirection::RowReverse,
                "column_reverse" | "ColumnReverse" => FlexDirection::ColumnReverse,
                _ => FlexDirection::Column,
            };
        }
        if let Some(j) = l.justify {
            props.justify = match j.as_str() {
                "flex_start" | "FlexStart" => JustifyContent::FlexStart,
                "center" | "Center" => JustifyContent::Center,
                "flex_end" | "FlexEnd" => JustifyContent::FlexEnd,
                "space_between" | "SpaceBetween" => JustifyContent::SpaceBetween,
                "space_around" | "SpaceAround" => JustifyContent::SpaceAround,
                "space_evenly" | "SpaceEvenly" => JustifyContent::SpaceEvenly,
                _ => JustifyContent::FlexStart,
            };
        }
        if let Some(a) = l.align {
            props.align = match a.as_str() {
                "flex_start" | "FlexStart" => AlignItems::FlexStart,
                "center" | "Center" => AlignItems::Center,
                "flex_end" | "FlexEnd" => AlignItems::FlexEnd,
                "stretch" | "Stretch" => AlignItems::Stretch,
                _ => AlignItems::Stretch,
            };
        }
        if let Some(p) = l.padding {
            props.padding = Some(p.into());
        }
        if let Some(m) = l.margin {
            props.margin = Some(m.into());
        }
        if let Some(g) = l.gap {
            props.gap = Some(g.into());
        }
        if let Some(grow) = l.flex_grow {
            props.flex_grow = grow;
        }
        if let Some(shrink) = l.flex_shrink {
            props.flex_shrink = shrink;
        }
        props
    }
}

impl From<RectValuesJson> for RectValues {
    fn from(r: RectValuesJson) -> Self {
        let mut rv = RectValues::default();
        if let Some(all) = r.all {
            rv.top = Some(all);
            rv.right = Some(all);
            rv.bottom = Some(all);
            rv.left = Some(all);
        }
        if let Some(h) = r.horizontal {
            rv.left = Some(h);
            rv.right = Some(h);
        }
        if let Some(v) = r.vertical {
            rv.top = Some(v);
            rv.bottom = Some(v);
        }
        if let Some(t) = r.top {
            rv.top = Some(t);
        }
        if let Some(ri) = r.right {
            rv.right = Some(ri);
        }
        if let Some(b) = r.bottom {
            rv.bottom = Some(b);
        }
        if let Some(l) = r.left {
            rv.left = Some(l);
        }
        rv
    }
}

impl From<GapJson> for Gap {
    fn from(g: GapJson) -> Self {
        Gap { row: g.width.unwrap_or(0.0), column: g.height.unwrap_or(0.0) }
    }
}

impl From<SizingJson> for Sizing {
    fn from(s: SizingJson) -> Self {
        match s {
            SizingJson::Points(v) => Sizing::Points(v),
            SizingJson::Percent(v) => Sizing::Percent(v),
            SizingJson::Auto => Sizing::Auto,
        }
    }
}
