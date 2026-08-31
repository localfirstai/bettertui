//! C-compatible event sink for FFI bindings.
//!
//! Provides a simple callback-based event sink that can be used across
//! the native FFI boundary with TypeScript/JavaScript.

use std::collections::VecDeque;
use std::ffi::c_void;

use crate::tree::{NodeId, Point};

/// C-compatible callback type for event emission.
///
/// Parameters:
/// - `name_ptr`: Pointer to event name bytes
/// - `name_len`: Length of event name
/// - `data_ptr`: Pointer to event data bytes (JSON-encoded)
/// - `data_len`: Length of event data
/// - `user_data`: User-provided context pointer
pub type EventCallback =
    extern "C" fn(name_ptr: *const u8, name_len: u32, data_ptr: *const u8, data_len: u32, user_data: *mut c_void);

/// A C-compatible event sink that holds a callback and user data.
#[derive(Debug)]
#[repr(C)]
pub struct EventSink {
    callback: Option<EventCallback>,
    user_data: *mut c_void,
}

unsafe impl Send for EventSink {}
unsafe impl Sync for EventSink {}

impl Default for EventSink {
    fn default() -> Self {
        Self::new()
    }
}

impl EventSink {
    pub fn new() -> Self {
        Self { callback: None, user_data: std::ptr::null_mut() }
    }

    pub fn with_callback(callback: EventCallback, user_data: *mut c_void) -> Self {
        Self { callback: Some(callback), user_data }
    }

    pub fn set_callback(&mut self, callback: EventCallback, user_data: *mut c_void) {
        self.callback = Some(callback);
        self.user_data = user_data;
    }

    pub fn clear_callback(&mut self) {
        self.callback = None;
        self.user_data = std::ptr::null_mut();
    }

    pub fn emit(&self, name: &[u8], data: &[u8]) -> bool {
        if let Some(callback) = self.callback {
            if name.len() > u32::MAX as usize || data.len() > u32::MAX as usize {
                return false;
            }
            callback(name.as_ptr(), name.len() as u32, data.as_ptr(), data.len() as u32, self.user_data);
            true
        } else {
            false
        }
    }

    pub fn emit_str(&self, name: &str, data: &str) -> bool {
        self.emit(name.as_bytes(), data.as_bytes())
    }

    pub fn has_callback(&self) -> bool {
        self.callback.is_some()
    }
}

impl Drop for EventSink {
    fn drop(&mut self) {
        self.clear_callback();
    }
}

pub fn create_event_sink(callback: EventCallback, user_data: *mut c_void) -> Box<EventSink> {
    Box::new(EventSink::with_callback(callback, user_data))
}

pub fn destroy_event_sink(sink: Box<EventSink>) {
    drop(sink);
}

pub fn emit_to_sink(sink: &EventSink, name: &[u8], data: &[u8]) -> bool {
    sink.emit(name, data)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SinkId(pub u64);

impl SinkId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

pub struct EventBus {
    sinks: Vec<(SinkId, EventSink)>,
    next_id: u64,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self { sinks: Vec::new(), next_id: 1 }
    }

    pub fn register(&mut self, callback: EventCallback, user_data: *mut c_void) -> SinkId {
        let id = SinkId::new(self.next_id);
        self.next_id += 1;
        let sink = EventSink::with_callback(callback, user_data);
        self.sinks.push((id, sink));
        id
    }

    pub fn unregister(&mut self, id: SinkId) -> bool {
        let len = self.sinks.len();
        self.sinks.retain(|(sink_id, _)| *sink_id != id);
        self.sinks.len() < len
    }

    pub fn emit(&self, name: &[u8], data: &[u8]) -> usize {
        self.sinks.iter().filter(|(_, sink)| sink.emit(name, data)).count()
    }

    pub fn emit_str(&self, name: &str, data: &str) -> usize {
        self.emit(name.as_bytes(), data.as_bytes())
    }

    pub fn len(&self) -> usize {
        self.sinks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sinks.is_empty()
    }

    pub fn clear(&mut self) {
        self.sinks.clear();
    }
}

#[derive(Debug, Clone)]
pub struct NativeEvent {
    pub name: Box<str>,
    pub data: Box<[u8]>,
}

impl NativeEvent {
    pub fn new(name: impl Into<Box<str>>, data: impl Into<Box<[u8]>>) -> Self {
        Self { name: name.into(), data: data.into() }
    }

    pub fn json(name: impl Into<Box<str>>, json: &str) -> Self {
        Self { name: name.into(), data: json.as_bytes().into() }
    }
}

#[derive(Debug)]
pub struct NativeEventBus {
    queue: std::collections::VecDeque<NativeEvent>,
    sink: EventSink,
    max_queue_size: usize,
}

impl Default for NativeEventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl NativeEventBus {
    pub fn new() -> Self {
        Self { queue: std::collections::VecDeque::new(), sink: EventSink::new(), max_queue_size: 256 }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { queue: std::collections::VecDeque::with_capacity(capacity), sink: EventSink::new(), max_queue_size: 256 }
    }

    pub fn set_callback(&mut self, callback: EventCallback, user_data: *mut c_void) {
        self.sink.set_callback(callback, user_data);
    }

    pub fn clear_callback(&mut self) {
        self.sink.clear_callback();
    }

    pub fn push(&mut self, event: NativeEvent) {
        if self.queue.len() >= self.max_queue_size {
            self.queue.pop_front();
        }
        self.queue.push_back(event);
    }

    pub fn push_json(&mut self, name: impl Into<Box<str>>, json: &str) {
        self.push(NativeEvent::json(name, json));
    }

    pub fn drain(&mut self) -> usize {
        let mut count = 0;
        while let Some(event) = self.queue.pop_front() {
            if self.sink.emit(event.name.as_bytes(), &event.data) {
                count += 1;
            }
        }
        count
    }

    pub fn flush(&mut self) -> usize {
        self.drain()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn sink(&self) -> &EventSink {
        &self.sink
    }

    pub fn sink_mut(&mut self) -> &mut EventSink {
        &mut self.sink
    }
}

// === Event Types (moved from input.rs) ===

/// Phase of event propagation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventPhase {
    Capture,
    Target,
    Bubble,
}

/// The main event enum covering all input event types.
#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(PasteEvent),
    Focus(FocusEvent),
    Blur(BlurEvent),
    Resize(ResizeEvent),
    Lifecycle(LifecycleEvent),
}

impl Event {
    pub fn phase(&self) -> EventPhase {
        match self {
            Event::Key(e) => e.phase,
            Event::Mouse(e) => e.phase,
            Event::Paste(e) => e.phase,
            Event::Focus(e) => e.phase,
            Event::Blur(e) => e.phase,
            Event::Resize(_) => EventPhase::Target,
            Event::Lifecycle(_) => EventPhase::Target,
        }
    }

    pub fn set_phase(&mut self, phase: EventPhase) {
        match self {
            Event::Key(e) => e.phase = phase,
            Event::Mouse(e) => e.phase = phase,
            Event::Paste(e) => e.phase = phase,
            Event::Focus(e) => e.phase = phase,
            Event::Blur(e) => e.phase = phase,
            Event::Resize(_) | Event::Lifecycle(_) => {}
        }
    }

    pub fn target(&self) -> Option<NodeId> {
        match self {
            Event::Key(e) => Some(e.target),
            Event::Mouse(e) => Some(e.target),
            Event::Paste(e) => Some(e.target),
            Event::Focus(e) => Some(e.target),
            Event::Blur(e) => Some(e.target),
            Event::Resize(_) | Event::Lifecycle(_) => None,
        }
    }

    pub fn is_consumed(&self) -> bool {
        match self {
            Event::Key(e) => e.default_prevented,
            Event::Mouse(e) => e.default_prevented,
            Event::Paste(e) => e.default_prevented,
            Event::Focus(_) | Event::Blur(_) | Event::Resize(_) | Event::Lifecycle(_) => false,
        }
    }
}

/// A key identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Character(char),
    Enter,
    Escape,
    Backspace,
    Delete,
    Tab,
    Space,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,
    F(u8),
    Ctrl(char),
    Alt(char),
}

/// Keyboard modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

impl Modifiers {
    pub const NONE: Self = Self { ctrl: false, shift: false, alt: false, meta: false };

    pub fn is_empty(&self) -> bool {
        !self.ctrl && !self.shift && !self.alt && !self.meta
    }
}

/// A key event with modifiers and target.
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: Key,
    pub modifiers: Modifiers,
    pub target: NodeId,
    pub phase: EventPhase,
    pub default_prevented: bool,
}

impl KeyEvent {
    pub fn new(key: Key, target: NodeId) -> Self {
        Self { key, modifiers: Modifiers::NONE, target, phase: EventPhase::Target, default_prevented: false }
    }

    pub fn with_modifiers(mut self, modifiers: Modifiers) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn prevent_default(&mut self) {
        self.default_prevented = true;
    }
}

/// Mouse button identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    None,
}

/// A mouse event.
#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub button: MouseButton,
    pub position: Point,
    pub modifiers: Modifiers,
    pub target: NodeId,
    pub phase: EventPhase,
    pub default_prevented: bool,
}

impl MouseEvent {
    pub fn new(button: MouseButton, position: Point, target: NodeId) -> Self {
        Self {
            button,
            position,
            modifiers: Modifiers::NONE,
            target,
            phase: EventPhase::Target,
            default_prevented: false,
        }
    }

    pub fn prevent_default(&mut self) {
        self.default_prevented = true;
    }
}

/// A paste event.
#[derive(Debug, Clone)]
pub struct PasteEvent {
    pub text: Box<str>,
    pub target: NodeId,
    pub phase: EventPhase,
    pub default_prevented: bool,
}

impl PasteEvent {
    pub fn new(text: impl Into<Box<str>>, target: NodeId) -> Self {
        Self { text: text.into(), target, phase: EventPhase::Target, default_prevented: false }
    }
}

/// A focus event.
#[derive(Debug, Clone)]
pub struct FocusEvent {
    pub target: NodeId,
    pub previous: Option<NodeId>,
    pub phase: EventPhase,
}

impl FocusEvent {
    pub fn new(target: NodeId, previous: Option<NodeId>) -> Self {
        Self { target, previous, phase: EventPhase::Target }
    }
}

/// A blur event.
#[derive(Debug, Clone)]
pub struct BlurEvent {
    pub target: NodeId,
    pub next: Option<NodeId>,
    pub phase: EventPhase,
}

impl BlurEvent {
    pub fn new(target: NodeId, next: Option<NodeId>) -> Self {
        Self { target, next, phase: EventPhase::Target }
    }
}

/// A resize event.
#[derive(Debug, Clone, Copy)]
pub struct ResizeEvent {
    pub width: u16,
    pub height: u16,
    pub previous_width: u16,
    pub previous_height: u16,
}

impl ResizeEvent {
    pub fn new(width: u16, height: u16, previous_width: u16, previous_height: u16) -> Self {
        Self { width, height, previous_width, previous_height }
    }
}

/// A lifecycle event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleEvent {
    Mount,
    Unmount,
    Suspend,
    Resume,
}

/// Result of event dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    Consumed,
    Ignored,
}

// === EventQueue (absorbed from input.rs::EventBus) ===

/// Priority-aware event queue with mouse coalescing.
///
/// Processes events through an `EventEmitterHub` which handles global listeners,
/// capture/target/bubble phases, and FFI sink emission.
#[derive(Debug)]
pub struct EventQueue {
    queue: VecDeque<Event>,
    max_queue_size: usize,
    coalesce_mouse: bool,
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl EventQueue {
    pub fn new() -> Self {
        Self { queue: VecDeque::new(), max_queue_size: 256, coalesce_mouse: true }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { queue: VecDeque::with_capacity(capacity), max_queue_size: 256, coalesce_mouse: true }
    }

    pub fn push(&mut self, event: Event) {
        if self.coalesce_mouse && matches!(event, Event::Mouse(_)) {
            self.coalesce_mouse_event(event);
            return;
        }

        if self.queue.len() >= self.max_queue_size {
            self.queue.pop_front();
        }
        self.queue.push_back(event);
    }

    fn coalesce_mouse_event(&mut self, event: Event) {
        if let Some(Event::Mouse(last)) = self.queue.back()
            && let Event::Mouse(ref new) = event
            && last.button == new.button
            && last.modifiers == new.modifiers
        {
            self.queue.pop_back();
        }
        self.queue.push_back(event);
    }

    pub fn push_key(&mut self, key: Key, modifiers: Modifiers, target: NodeId) {
        let mut event = Event::Key(KeyEvent::new(key, target));
        if let Event::Key(ref mut ke) = event {
            ke.modifiers = modifiers;
        }
        self.push(event);
    }

    pub fn push_mouse(&mut self, button: MouseButton, position: Point, target: NodeId) {
        self.push(Event::Mouse(MouseEvent::new(button, position, target)));
    }

    pub fn push_paste(&mut self, text: impl Into<std::sync::Arc<str>>, target: NodeId) {
        let text: std::sync::Arc<str> = text.into();
        let text_box: Box<str> = Box::from(text.as_ref());
        self.push(Event::Paste(PasteEvent::new(text_box, target)));
    }

    pub fn push_resize(&mut self, width: u16, height: u16, prev_width: u16, prev_height: u16) {
        self.push(Event::Resize(ResizeEvent::new(width, height, prev_width, prev_height)));
    }

    pub fn push_lifecycle(&mut self, event: LifecycleEvent) {
        self.push(Event::Lifecycle(event));
    }

    pub fn drain(&mut self) -> VecDeque<Event> {
        std::mem::take(&mut self.queue)
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn set_coalesce_mouse(&mut self, coalesce: bool) {
        self.coalesce_mouse = coalesce;
    }

    pub fn set_max_queue_size(&mut self, max: usize) {
        self.max_queue_size = max;
    }

    pub fn pop_front(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::NodeId;

    thread_local! {
        static LAST_EVENT_NAME: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
        static LAST_EVENT_DATA: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
    }

    extern "C" fn test_callback(
        name_ptr: *const u8,
        name_len: u32,
        data_ptr: *const u8,
        data_len: u32,
        _user_data: *mut c_void,
    ) {
        let name: Vec<u8> = if name_ptr.is_null() || name_len == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(name_ptr, name_len as usize).to_vec() }
        };
        let data: Vec<u8> = if data_ptr.is_null() || data_len == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(data_ptr, data_len as usize).to_vec() }
        };
        LAST_EVENT_NAME.with(|n| *n.borrow_mut() = String::from_utf8_lossy(&name).to_string());
        LAST_EVENT_DATA.with(|d| *d.borrow_mut() = String::from_utf8_lossy(&data).to_string());
    }

    #[test]
    fn event_sink_emit() {
        let sink = EventSink::with_callback(test_callback, std::ptr::null_mut());

        let result = sink.emit_str("test_event", "{\"key\":\"value\"}");

        assert!(result);

        LAST_EVENT_NAME.with(|n| assert_eq!(n.borrow().as_str(), "test_event"));
        LAST_EVENT_DATA.with(|d| assert_eq!(d.borrow().as_str(), "{\"key\":\"value\"}"));
    }

    #[test]
    fn event_sink_no_callback() {
        let sink = EventSink::new();
        assert!(!sink.has_callback());

        let result = sink.emit_str("test", "{}");
        assert!(!result);
    }

    #[test]
    fn event_bus_register_emit() {
        let mut bus = EventBus::new();

        let _id = bus.register(test_callback, std::ptr::null_mut());

        let count = bus.emit_str("event", "data");

        assert_eq!(count, 1);
    }

    #[test]
    fn event_bus_unregister() {
        let mut bus = EventBus::new();

        let id = bus.register(test_callback, std::ptr::null_mut());
        bus.unregister(id);

        let count = bus.emit_str("event", "data");

        assert_eq!(count, 0);
    }

    #[test]
    fn native_event_bus_queue() {
        let mut bus = NativeEventBus::new();

        bus.push_json("key", "{\"name\":\"enter\"}");
        bus.push_json("mouse", "{\"x\":10,\"y\":20}");

        assert_eq!(bus.len(), 2);

        bus.clear_callback();

        let drained = bus.drain();
        assert_eq!(drained, 0);
        assert!(bus.is_empty());
    }

    #[test]
    fn event_queue_new() {
        let q = EventQueue::new();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn event_queue_push_and_drain() {
        let mut q = EventQueue::new();
        q.push(Event::Key(KeyEvent::new(Key::Enter, NodeId::default())));
        q.push(Event::Key(KeyEvent::new(Key::Escape, NodeId::default())));
        assert_eq!(q.len(), 2);

        let drained = q.drain();
        assert_eq!(drained.len(), 2);
        assert!(q.is_empty());
    }

    #[test]
    fn event_queue_coalesce_mouse() {
        let mut q = EventQueue::new();
        let pos1 = crate::tree::Point::new(5, 10);
        let pos2 = crate::tree::Point::new(15, 20);

        q.push(Event::Mouse(MouseEvent::new(MouseButton::Left, pos1, NodeId::default())));
        q.push(Event::Mouse(MouseEvent::new(MouseButton::Left, pos2, NodeId::default())));

        assert_eq!(q.len(), 1);
        let drained = q.drain();
        if let Event::Mouse(me) = &drained[0] {
            assert_eq!(me.position, pos2);
        } else {
            panic!("expected mouse event");
        }
    }

    #[test]
    fn event_queue_no_coalesce_different_button() {
        let mut q = EventQueue::new();
        let pos = crate::tree::Point::new(5, 10);

        q.push(Event::Mouse(MouseEvent::new(MouseButton::Left, pos, NodeId::default())));
        q.push(Event::Mouse(MouseEvent::new(MouseButton::Right, pos, NodeId::default())));

        assert_eq!(q.len(), 2);
    }

    #[test]
    fn event_queue_max_size() {
        let mut q = EventQueue::new();
        q.set_max_queue_size(3);

        for i in 0..5u8 {
            q.push(Event::Key(KeyEvent::new(Key::Character((b'a' + i) as char), NodeId::default())));
        }

        assert_eq!(q.len(), 3);
    }

    #[test]
    fn event_queue_pop_front() {
        let mut q = EventQueue::new();
        q.push(Event::Key(KeyEvent::new(Key::Enter, NodeId::default())));
        q.push(Event::Key(KeyEvent::new(Key::Escape, NodeId::default())));

        let first = q.pop_front();
        assert!(first.is_some());
        assert_eq!(q.len(), 1);

        let second = q.pop_front();
        assert!(second.is_some());
        assert!(q.is_empty());

        assert!(q.pop_front().is_none());
    }

    #[test]
    fn event_queue_push_key() {
        let mut q = EventQueue::new();
        q.push_key(Key::Enter, Modifiers::NONE, NodeId::default());
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn event_queue_push_paste() {
        let mut q = EventQueue::new();
        q.push_paste("hello world", NodeId::default());
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn event_queue_push_resize() {
        let mut q = EventQueue::new();
        q.push_resize(80, 24, 120, 40);
        assert_eq!(q.len(), 1);
        let drained = q.drain();
        if let Event::Resize(re) = &drained[0] {
            assert_eq!(re.width, 80);
            assert_eq!(re.height, 24);
        } else {
            panic!("expected resize event");
        }
    }
}
