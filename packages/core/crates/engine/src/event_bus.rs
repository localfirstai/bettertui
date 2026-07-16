//! C-compatible event sink for FFI bindings.
//!
//! Provides a simple callback-based event sink that can be used across
//! the native FFI boundary with TypeScript/JavaScript.

use std::ffi::c_void;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SinkId(pub u64);

impl SinkId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl Default for SinkId {
    fn default() -> Self {
        Self(0)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicPtr, Ordering};

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
}
