//! Generic typed event emitter with listener registration and emission.
//!
//! Provides a type-safe pub/sub pattern for event handling with support for
//! priority-based dispatch and listener removal.

use std::any::Any;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;

/// Unique identifier for a registered listener.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ListenerId(pub u64);

impl ListenerId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// A boxed event listener function.
pub type BoxedListener<E> = Box<dyn Fn(&E) -> ListenerResult + Send + Sync>;

/// Result of a listener handling an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListenerResult {
    /// Event was consumed, stop propagation.
    Consumed,
    /// Event was not consumed, continue propagation.
    #[default]
    Continue,
}

/// A registered listener with metadata.
struct ListenerEntry<E> {
    id: ListenerId,
    listener: BoxedListener<E>,
    priority: i32,
    once: bool,
}

/// A typed event emitter that manages listeners for a specific event type.
///
/// Listeners are called in priority order (highest first). A listener can
/// consume an event to stop further propagation.
pub struct EventEmitter<E: 'static> {
    listeners: Vec<ListenerEntry<E>>,
    next_id: u64,
    max_listeners: usize,
}

impl<E: 'static> Default for EventEmitter<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: 'static> EventEmitter<E> {
    pub fn new() -> Self {
        Self { listeners: Vec::new(), next_id: 1, max_listeners: 1024 }
    }

    pub fn with_max_listeners(max: usize) -> Self {
        Self { listeners: Vec::new(), next_id: 1, max_listeners: max }
    }

    /// Register a listener with default priority (0).
    pub fn on<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&E) -> ListenerResult + Send + Sync + 'static,
    {
        self.on_with_priority(listener, 0)
    }

    /// Register a listener with a specific priority.
    ///
    /// Higher priority listeners are called first.
    pub fn on_with_priority<F>(&mut self, listener: F, priority: i32) -> ListenerId
    where
        F: Fn(&E) -> ListenerResult + Send + Sync + 'static,
    {
        if self.listeners.len() >= self.max_listeners {
            self.listeners.remove(0);
        }

        let id = ListenerId::new(self.next_id);
        self.next_id += 1;

        let entry = ListenerEntry { id, listener: Box::new(listener), priority, once: false };

        let pos = self.listeners.iter().position(|e| e.priority < priority).unwrap_or(self.listeners.len());
        self.listeners.insert(pos, entry);

        id
    }

    /// Register a one-time listener that removes itself after first invocation.
    pub fn once<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&E) -> ListenerResult + Send + Sync + 'static,
    {
        self.once_with_priority(listener, 0)
    }

    /// Register a one-time listener with a specific priority.
    pub fn once_with_priority<F>(&mut self, listener: F, priority: i32) -> ListenerId
    where
        F: Fn(&E) -> ListenerResult + Send + Sync + 'static,
    {
        if self.listeners.len() >= self.max_listeners {
            self.listeners.remove(0);
        }

        let id = ListenerId::new(self.next_id);
        self.next_id += 1;

        let entry = ListenerEntry { id, listener: Box::new(listener), priority, once: true };

        let pos = self.listeners.iter().position(|e| e.priority < priority).unwrap_or(self.listeners.len());
        self.listeners.insert(pos, entry);

        id
    }

    /// Remove a listener by its ID.
    pub fn off(&mut self, id: ListenerId) -> bool {
        let len = self.listeners.len();
        self.listeners.retain(|e| e.id != id);
        self.listeners.len() < len
    }

    /// Remove all listeners.
    pub fn clear(&mut self) {
        self.listeners.clear();
    }

    /// Emit an event to all listeners.
    ///
    /// Returns `true` if the event was consumed by any listener.
    pub fn emit(&mut self, event: &E) -> bool {
        let mut once_ids: Vec<ListenerId> = Vec::new();

        for entry in &self.listeners {
            let result = (entry.listener)(event);
            if entry.once {
                once_ids.push(entry.id);
            }
            if result == ListenerResult::Consumed {
                for id in once_ids {
                    self.off(id);
                }
                return true;
            }
        }

        for id in once_ids {
            self.off(id);
        }

        false
    }

    /// Get the number of registered listeners.
    pub fn len(&self) -> usize {
        self.listeners.len()
    }

    /// Check if there are no listeners.
    pub fn is_empty(&self) -> bool {
        self.listeners.is_empty()
    }

    /// Check if a listener ID is still registered.
    pub fn has(&self, id: ListenerId) -> bool {
        self.listeners.iter().any(|e| e.id == id)
    }
}

/// Event type enumeration for the multi-event emitter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    Key,
    Mouse,
    Paste,
    Focus,
    Blur,
    Resize,
    Lifecycle,
    Custom(u32),
}

/// A multi-event emitter that handles different event types.
///
/// This is the main event hub that TypeScript/FFFI code interacts with.
pub struct MultiEventEmitter {
    key_emitter: EventEmitter<crate::event_bus::KeyEvent>,
    mouse_emitter: EventEmitter<crate::event_bus::MouseEvent>,
    paste_emitter: EventEmitter<crate::event_bus::PasteEvent>,
    focus_emitter: EventEmitter<crate::event_bus::FocusEvent>,
    blur_emitter: EventEmitter<crate::event_bus::BlurEvent>,
    resize_emitter: EventEmitter<crate::event_bus::ResizeEvent>,
    lifecycle_emitter: EventEmitter<crate::event_bus::LifecycleEvent>,
    custom_emitters: HashMap<u32, EventEmitter<Arc<dyn Any + Send + Sync>>>,
}

impl Default for MultiEventEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiEventEmitter {
    pub fn new() -> Self {
        Self {
            key_emitter: EventEmitter::new(),
            mouse_emitter: EventEmitter::new(),
            paste_emitter: EventEmitter::new(),
            focus_emitter: EventEmitter::new(),
            blur_emitter: EventEmitter::new(),
            resize_emitter: EventEmitter::new(),
            lifecycle_emitter: EventEmitter::new(),
            custom_emitters: HashMap::new(),
        }
    }

    pub fn on_key<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::event_bus::KeyEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.key_emitter.on(listener)
    }

    pub fn on_mouse<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::event_bus::MouseEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.mouse_emitter.on(listener)
    }

    pub fn on_paste<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::event_bus::PasteEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.paste_emitter.on(listener)
    }

    pub fn on_focus<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::event_bus::FocusEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.focus_emitter.on(listener)
    }

    pub fn on_blur<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::event_bus::BlurEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.blur_emitter.on(listener)
    }

    pub fn on_resize<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::event_bus::ResizeEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.resize_emitter.on(listener)
    }

    pub fn on_lifecycle<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::event_bus::LifecycleEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.lifecycle_emitter.on(listener)
    }

    pub fn off(&mut self, event_type: EventType, id: ListenerId) -> bool {
        match event_type {
            EventType::Key => self.key_emitter.off(id),
            EventType::Mouse => self.mouse_emitter.off(id),
            EventType::Paste => self.paste_emitter.off(id),
            EventType::Focus => self.focus_emitter.off(id),
            EventType::Blur => self.blur_emitter.off(id),
            EventType::Resize => self.resize_emitter.off(id),
            EventType::Lifecycle => self.lifecycle_emitter.off(id),
            EventType::Custom(code) => {
                if let Some(emitter) = self.custom_emitters.get_mut(&code) {
                    emitter.off(id)
                } else {
                    false
                }
            }
        }
    }

    pub fn emit_key(&mut self, event: &crate::event_bus::KeyEvent) -> bool {
        self.key_emitter.emit(event)
    }

    pub fn emit_mouse(&mut self, event: &crate::event_bus::MouseEvent) -> bool {
        self.mouse_emitter.emit(event)
    }

    pub fn emit_paste(&mut self, event: &crate::event_bus::PasteEvent) -> bool {
        self.paste_emitter.emit(event)
    }

    pub fn emit_focus(&mut self, event: &crate::event_bus::FocusEvent) -> bool {
        self.focus_emitter.emit(event)
    }

    pub fn emit_blur(&mut self, event: &crate::event_bus::BlurEvent) -> bool {
        self.blur_emitter.emit(event)
    }

    pub fn emit_resize(&mut self, event: &crate::event_bus::ResizeEvent) -> bool {
        self.resize_emitter.emit(event)
    }

    pub fn emit_lifecycle(&mut self, event: &crate::event_bus::LifecycleEvent) -> bool {
        self.lifecycle_emitter.emit(event)
    }

    pub fn clear(&mut self) {
        self.key_emitter.clear();
        self.mouse_emitter.clear();
        self.paste_emitter.clear();
        self.focus_emitter.clear();
        self.blur_emitter.clear();
        self.resize_emitter.clear();
        self.lifecycle_emitter.clear();
        self.custom_emitters.clear();
    }

    pub fn listener_count(&self) -> usize {
        self.key_emitter.len()
            + self.mouse_emitter.len()
            + self.paste_emitter.len()
            + self.focus_emitter.len()
            + self.blur_emitter.len()
            + self.resize_emitter.len()
            + self.lifecycle_emitter.len()
            + self.custom_emitters.values().map(|e| e.len()).sum::<usize>()
    }
}

// === EventEmitterHub ===

use crate::event_bus::{
    BlurEvent, Event, EventPhase, EventQueue, EventResult, FocusEvent, KeyEvent, LifecycleEvent, MouseEvent,
    PasteEvent, ResizeEvent,
};
use crate::tree::NodeArena;

type EventHandler = Box<dyn FnMut(&mut Event) -> EventResult + Send>;
type HandlerVec = Vec<EventHandler>;

/// Central event dispatch hub — THE central event API.
///
/// Combines typed global listeners (priority-ordered), untyped global handlers,
/// per-node handlers, capture/target/bubble phases, and an optional FFI `EventSink`.
///
/// # Pipeline
///
/// ```text
/// EventQueue::process_all(&mut hub, arena)
///     │
///     ▼
/// EventEmitterHub::dispatch(event, arena)
///     │
///     ├─ 1. Typed global listeners (priority order)
///     ├─ 2. Global untyped handlers (fire before node handlers)
///     ├─ 3. Capture phase (root → target ancestors)
///     ├─ 4. Target phase (target node handlers)
///     └─ 5. Bubble phase (target → root ancestors)
///     │
///     ▼
/// EventSink (FFI callback to TypeScript)
/// ```
pub struct EventEmitterHub {
    key: EventEmitter<KeyEvent>,
    mouse: EventEmitter<MouseEvent>,
    paste: EventEmitter<PasteEvent>,
    focus: EventEmitter<FocusEvent>,
    blur: EventEmitter<BlurEvent>,
    resize: EventEmitter<ResizeEvent>,
    lifecycle: EventEmitter<LifecycleEvent>,
    global_handlers: HandlerVec,
    node_handlers: HashMap<crate::tree::NodeId, HandlerVec>,
    sink: Option<crate::event_bus::EventSink>,
}

impl Default for EventEmitterHub {
    fn default() -> Self {
        Self::new()
    }
}

impl EventEmitterHub {
    pub fn new() -> Self {
        Self {
            key: EventEmitter::new(),
            mouse: EventEmitter::new(),
            paste: EventEmitter::new(),
            focus: EventEmitter::new(),
            blur: EventEmitter::new(),
            resize: EventEmitter::new(),
            lifecycle: EventEmitter::new(),
            global_handlers: Vec::new(),
            node_handlers: HashMap::new(),
            sink: None,
        }
    }

    /// Set the FFI event sink for emitting events to TypeScript.
    pub fn set_sink(&mut self, sink: crate::event_bus::EventSink) {
        self.sink = Some(sink);
    }

    /// Clear the FFI event sink.
    pub fn clear_sink(&mut self) {
        self.sink = None;
    }

    // ─── Typed global listeners (priority-ordered) ───────────────────────

    /// Register a typed global key listener.
    pub fn on_key<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&KeyEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.key.on(listener)
    }

    /// Register a typed global mouse listener.
    pub fn on_mouse<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&MouseEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.mouse.on(listener)
    }

    /// Register a typed global paste listener.
    pub fn on_paste<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&PasteEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.paste.on(listener)
    }

    /// Register a typed global focus listener.
    pub fn on_focus<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&FocusEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.focus.on(listener)
    }

    /// Register a typed global blur listener.
    pub fn on_blur<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&BlurEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.blur.on(listener)
    }

    /// Register a typed global resize listener.
    pub fn on_resize<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&ResizeEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.resize.on(listener)
    }

    /// Register a typed global lifecycle listener.
    pub fn on_lifecycle<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&LifecycleEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.lifecycle.on(listener)
    }

    // ─── Untyped global handlers ────────────────────────────────────────

    /// Register a global (untyped) event handler.
    ///
    /// Global handlers fire **before** per-node capture/target/bubble.
    /// If any handler returns `EventResult::Consumed`, propagation stops.
    pub fn on_global(&mut self, handler: impl FnMut(&mut Event) -> EventResult + Send + 'static) {
        self.global_handlers.push(Box::new(handler));
    }

    /// Remove all global handlers.
    pub fn clear_global_handlers(&mut self) {
        self.global_handlers.clear();
    }

    // ─── Per-node handlers ──────────────────────────────────────────────

    /// Register a per-node event handler (capture/target/bubble).
    pub fn on_node(
        &mut self,
        node_id: crate::tree::NodeId,
        handler: impl FnMut(&mut Event) -> EventResult + Send + 'static,
    ) {
        self.node_handlers.entry(node_id).or_default().push(Box::new(handler));
    }

    /// Remove all handlers for a node.
    pub fn remove_node_handlers(&mut self, node_id: crate::tree::NodeId) {
        self.node_handlers.remove(&node_id);
    }

    /// Check if a node has handlers registered.
    pub fn has_node_handlers(&self, node_id: crate::tree::NodeId) -> bool {
        self.node_handlers.contains_key(&node_id)
    }

    // ─── Phase-aware emission ───────────────────────────────────────────

    /// Emit a typed event at a specific phase.
    ///
    /// This allows callers to manually control which phase the event is
    /// dispatched at, useful for synthetic events or replay.
    pub fn emit_with_phase(&mut self, event: &mut Event, phase: EventPhase) -> EventResult {
        event.set_phase(phase);
        match event {
            Event::Key(e) => {
                if self.key.emit(e) {
                    return EventResult::Consumed;
                }
            }
            Event::Mouse(e) => {
                if self.mouse.emit(e) {
                    return EventResult::Consumed;
                }
            }
            Event::Paste(e) => {
                if self.paste.emit(e) {
                    return EventResult::Consumed;
                }
            }
            Event::Focus(e) => {
                if self.focus.emit(e) {
                    return EventResult::Consumed;
                }
            }
            Event::Blur(e) => {
                if self.blur.emit(e) {
                    return EventResult::Consumed;
                }
            }
            Event::Resize(e) => {
                if self.resize.emit(e) {
                    return EventResult::Consumed;
                }
            }
            Event::Lifecycle(e) => {
                if self.lifecycle.emit(e) {
                    return EventResult::Consumed;
                }
            }
        }
        EventResult::Ignored
    }

    // ─── Full pipeline dispatch ─────────────────────────────────────────

    /// Dispatch a single event through the full pipeline:
    ///
    /// 1. Typed global listeners (priority order) — stops if consumed
    /// 2. Global untyped handlers — stops if consumed
    /// 3. Capture phase (root → target ancestors) — stops if consumed
    /// 4. Target phase (target node handlers) — stops if consumed
    /// 5. Bubble phase (target → root ancestors) — stops if consumed
    /// 6. Emit to FFI sink
    pub fn dispatch(&mut self, event: &mut Event, arena: &NodeArena) -> EventResult {
        // 1. Typed global listeners
        let consumed = match event {
            Event::Key(e) => self.key.emit(e),
            Event::Mouse(e) => self.mouse.emit(e),
            Event::Paste(e) => self.paste.emit(e),
            Event::Focus(e) => self.focus.emit(e),
            Event::Blur(e) => self.blur.emit(e),
            Event::Resize(e) => self.resize.emit(e),
            Event::Lifecycle(e) => self.lifecycle.emit(e),
        };
        if consumed {
            return EventResult::Consumed;
        }

        // 2. Global untyped handlers
        for handler in &mut self.global_handlers {
            let result = handler(event);
            if result == EventResult::Consumed {
                return EventResult::Consumed;
            }
        }

        // 3-5. Capture → Target → Bubble (per-node handlers)
        let target = match event.target() {
            Some(id) => id,
            None => {
                self.emit_to_sink(event);
                return EventResult::Ignored;
            }
        };

        let ancestors: Vec<crate::tree::NodeId> = arena.ancestors(target);

        // 3. Capture phase (root → target)
        for &ancestor in ancestors.iter().rev() {
            event.set_phase(EventPhase::Capture);
            if let Some(handlers) = self.node_handlers.get_mut(&ancestor) {
                for handler in handlers {
                    let result = handler(event);
                    if result == EventResult::Consumed || event.is_consumed() {
                        self.emit_to_sink(event);
                        return EventResult::Consumed;
                    }
                }
            }
        }

        // 4. Target phase
        event.set_phase(EventPhase::Target);
        if let Some(handlers) = self.node_handlers.get_mut(&target) {
            for handler in handlers {
                let result = handler(event);
                if result == EventResult::Consumed || event.is_consumed() {
                    self.emit_to_sink(event);
                    return EventResult::Consumed;
                }
            }
        }

        // 5. Bubble phase (target → root)
        for &ancestor in &ancestors {
            event.set_phase(EventPhase::Bubble);
            if let Some(handlers) = self.node_handlers.get_mut(&ancestor) {
                for handler in handlers {
                    let result = handler(event);
                    if result == EventResult::Consumed || event.is_consumed() {
                        self.emit_to_sink(event);
                        return EventResult::Consumed;
                    }
                }
            }
        }

        // 6. Emit to FFI sink
        self.emit_to_sink(event);
        EventResult::Ignored
    }

    // ─── FFI sink ───────────────────────────────────────────────────────

    /// Emit event to the FFI sink as JSON.
    fn emit_to_sink(&mut self, event: &Event) {
        if let Some(ref sink) = self.sink {
            let (name, json) = match event {
                Event::Key(e) => (
                    "key",
                    serde_json::json!({
                        "key": format!("{:?}", e.key),
                        "modifiers": serde_json::json!({
                            "ctrl": e.modifiers.ctrl,
                            "shift": e.modifiers.shift,
                            "alt": e.modifiers.alt,
                            "meta": e.modifiers.meta,
                        }),
                        "phase": format!("{:?}", e.phase),
                        "defaultPrevented": e.default_prevented,
                    }),
                ),
                Event::Mouse(e) => (
                    "mouse",
                    serde_json::json!({
                        "button": format!("{:?}", e.button),
                        "position": { "x": e.position.x, "y": e.position.y },
                        "phase": format!("{:?}", e.phase),
                    }),
                ),
                Event::Paste(e) => (
                    "paste",
                    serde_json::json!({
                        "text": e.text.as_ref(),
                        "phase": format!("{:?}", e.phase),
                    }),
                ),
                Event::Focus(e) => (
                    "focus",
                    serde_json::json!({
                        "phase": format!("{:?}", e.phase),
                    }),
                ),
                Event::Blur(e) => (
                    "blur",
                    serde_json::json!({
                        "phase": format!("{:?}", e.phase),
                    }),
                ),
                Event::Resize(e) => (
                    "resize",
                    serde_json::json!({
                        "width": e.width,
                        "height": e.height,
                        "previousWidth": e.previous_width,
                        "previousHeight": e.previous_height,
                    }),
                ),
                Event::Lifecycle(e) => (
                    "lifecycle",
                    serde_json::json!({
                        "type": format!("{:?}", e),
                    }),
                ),
            };
            let json_str = json.to_string();
            sink.emit_str(name, &json_str);
        }
    }

    // ─── Utilities ──────────────────────────────────────────────────────

    /// Get total listener count across all emitters + handlers.
    pub fn listener_count(&self) -> usize {
        self.key.len()
            + self.mouse.len()
            + self.paste.len()
            + self.focus.len()
            + self.blur.len()
            + self.resize.len()
            + self.lifecycle.len()
            + self.global_handlers.len()
            + self.node_handlers.values().map(|v| v.len()).sum::<usize>()
    }

    /// Get count of global handlers only.
    pub fn global_handler_count(&self) -> usize {
        self.global_handlers.len()
    }

    /// Clear all listeners and handlers.
    pub fn clear(&mut self) {
        self.key.clear();
        self.mouse.clear();
        self.paste.clear();
        self.focus.clear();
        self.blur.clear();
        self.resize.clear();
        self.lifecycle.clear();
        self.global_handlers.clear();
        self.node_handlers.clear();
    }
}

// === EventQueue dispatch methods ===

impl EventQueue {
    /// Drain all events and dispatch through the hub.
    pub fn process_all(&mut self, hub: &mut EventEmitterHub, arena: &NodeArena) {
        let events: VecDeque<Event> = self.drain();
        for mut event in events {
            hub.dispatch(&mut event, arena);
        }
    }

    /// Dispatch events until one is consumed.
    pub fn process_until_consumed(&mut self, hub: &mut EventEmitterHub, arena: &NodeArena) -> Option<EventResult> {
        while let Some(mut event) = self.pop_front() {
            let result = hub.dispatch(&mut event, arena);
            if result == EventResult::Consumed {
                return Some(result);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    use crate::event_bus::{
        Event, EventPhase, EventQueue, EventResult, Key, KeyEvent, Modifiers, MouseButton, MouseEvent,
    };
    use crate::tree::NodeId;

    #[test]
    fn emitter_on_emit() {
        let mut emitter = EventEmitter::<i32>::new();
        let called = Arc::new(Mutex::new(false));

        let called_clone = called.clone();
        emitter.on(move |_e| {
            *called_clone.lock().unwrap() = true;
            ListenerResult::Continue
        });

        emitter.emit(&42);
        assert!(*called.lock().unwrap());
    }

    #[test]
    fn emitter_priority_order() {
        let mut emitter = EventEmitter::<i32>::new();
        let order = Arc::new(Mutex::new(Vec::new()));

        let order_clone = order.clone();
        emitter.on_with_priority(
            move |_| {
                order_clone.lock().unwrap().push(1);
                ListenerResult::Continue
            },
            0,
        );

        let order_clone = order.clone();
        emitter.on_with_priority(
            move |_| {
                order_clone.lock().unwrap().push(2);
                ListenerResult::Continue
            },
            10,
        );

        let order_clone = order.clone();
        emitter.on_with_priority(
            move |_| {
                order_clone.lock().unwrap().push(3);
                ListenerResult::Continue
            },
            5,
        );

        emitter.emit(&0);

        let result = order.lock().unwrap();
        assert_eq!(*result, vec![2, 3, 1]);
    }

    #[test]
    fn emitter_consume_stops_propagation() {
        let mut emitter = EventEmitter::<i32>::new();
        let called = Arc::new(Mutex::new(false));

        emitter.on(|_| ListenerResult::Consumed);

        let called_clone = called.clone();
        emitter.on(move |_| {
            *called_clone.lock().unwrap() = true;
            ListenerResult::Continue
        });

        let consumed = emitter.emit(&42);
        assert!(consumed);
        assert!(!*called.lock().unwrap());
    }

    #[test]
    fn emitter_once() {
        let mut emitter = EventEmitter::<i32>::new();
        let count = Arc::new(Mutex::new(0));

        let count_clone = count.clone();
        emitter.once(move |_| {
            *count_clone.lock().unwrap() += 1;
            ListenerResult::Continue
        });

        emitter.emit(&1);
        emitter.emit(&2);

        assert_eq!(*count.lock().unwrap(), 1);
    }

    #[test]
    fn emitter_off() {
        let mut emitter = EventEmitter::<i32>::new();
        let called = Arc::new(Mutex::new(false));

        let called_clone = called.clone();
        let id = emitter.on(move |_| {
            *called_clone.lock().unwrap() = true;
            ListenerResult::Continue
        });

        emitter.off(id);
        emitter.emit(&42);

        assert!(!*called.lock().unwrap());
    }

    #[test]
    fn hub_dispatch_key_to_global() {
        let mut hub = EventEmitterHub::new();

        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();
        hub.key.on(move |ev: &KeyEvent| {
            if ev.key == Key::Enter {
                *called_clone.lock().unwrap() = true;
            }
            ListenerResult::Continue
        });

        let target = NodeId::default();
        let mut event = Event::Key(KeyEvent::new(Key::Enter, target));
        hub.dispatch(&mut event, &crate::tree::NodeArena::new());

        assert!(*called.lock().unwrap());
    }

    #[test]
    fn hub_global_typed_listener_priority() {
        let mut hub = EventEmitterHub::new();
        let order = Arc::new(Mutex::new(Vec::new()));

        let o1 = order.clone();
        hub.key.on_with_priority(
            move |_ev: &KeyEvent| {
                o1.lock().unwrap().push("low".to_string());
                ListenerResult::Continue
            },
            10,
        );

        let o2 = order.clone();
        hub.key.on_with_priority(
            move |_ev: &KeyEvent| {
                o2.lock().unwrap().push("high".to_string());
                ListenerResult::Continue
            },
            -100,
        );

        let mut event = Event::Key(KeyEvent::new(Key::Enter, NodeId::default()));
        hub.dispatch(&mut event, &crate::tree::NodeArena::new());

        let o = order.lock().unwrap();
        assert_eq!(o[0], "low");
        assert_eq!(o[1], "high");
    }

    #[test]
    fn hub_listener_count() {
        let mut hub = EventEmitterHub::new();
        assert_eq!(hub.listener_count(), 0);

        hub.key.on(|_| ListenerResult::Continue);
        hub.mouse.on(|_| ListenerResult::Continue);

        assert_eq!(hub.listener_count(), 2);
    }

    #[test]
    fn hub_dispatch_consumed() {
        let mut hub = EventEmitterHub::new();

        hub.key.on(|_| ListenerResult::Consumed);

        let mut event = Event::Key(KeyEvent::new(Key::Enter, NodeId::default()));
        let result = hub.dispatch(&mut event, &crate::tree::NodeArena::new());

        assert_eq!(result, EventResult::Consumed);
    }

    #[test]
    fn event_queue_process_all() {
        use crate::event_bus::EventQueue;

        let mut q = EventQueue::new();
        q.push(Event::Key(KeyEvent::new(Key::Enter, NodeId::default())));
        q.push(Event::Key(KeyEvent::new(Key::Escape, NodeId::default())));

        let mut hub = EventEmitterHub::new();
        let count = Arc::new(Mutex::new(0u32));
        let count_clone = count.clone();
        hub.key.on(move |_: &KeyEvent| {
            *count_clone.lock().unwrap() += 1;
            ListenerResult::Continue
        });

        let arena = crate::tree::NodeArena::new();
        q.process_all(&mut hub, &arena);

        assert_eq!(*count.lock().unwrap(), 2);
        assert!(q.is_empty());
    }

    #[test]
    fn hub_global_untyped_handler() {
        let mut hub = EventEmitterHub::new();
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();

        hub.on_global(move |event: &mut Event| {
            if matches!(event, Event::Key(_)) {
                *called_clone.lock().unwrap() = true;
            }
            EventResult::Ignored
        });

        let mut event = Event::Key(KeyEvent::new(Key::Enter, NodeId::default()));
        hub.dispatch(&mut event, &crate::tree::NodeArena::new());

        assert!(*called.lock().unwrap());
    }

    #[test]
    fn hub_global_handler_stops_node_propagation() {
        let mut hub = EventEmitterHub::new();
        let arena = crate::tree::NodeArena::new();
        let target = arena.root();

        // Global handler consumes — stops capture/target/bubble
        hub.on_global(|_event: &mut Event| EventResult::Consumed);

        // Node handler should NOT fire
        let typed_called = Arc::new(Mutex::new(false));
        let typed_called_clone = typed_called.clone();
        hub.on_node(target, move |event: &mut Event| {
            if event.phase() == EventPhase::Target {
                *typed_called_clone.lock().unwrap() = true;
            }
            EventResult::Ignored
        });

        let mut event = Event::Key(KeyEvent::new(Key::Enter, target));
        let result = hub.dispatch(&mut event, &arena);

        assert_eq!(result, EventResult::Consumed);
        assert!(!*typed_called.lock().unwrap());
    }

    #[test]
    fn hub_emit_with_phase() {
        let mut hub = EventEmitterHub::new();
        let phase_seen = Arc::new(Mutex::new(String::new()));
        let phase_seen_clone = phase_seen.clone();

        hub.key.on(move |ev: &KeyEvent| {
            *phase_seen_clone.lock().unwrap() = format!("{:?}", ev.phase);
            ListenerResult::Continue
        });

        let mut event = Event::Key(KeyEvent::new(Key::Enter, NodeId::default()));
        hub.emit_with_phase(&mut event, EventPhase::Capture);

        assert_eq!(*phase_seen.lock().unwrap(), "Capture");
    }

    #[test]
    fn hub_on_node_handler() {
        let mut hub = EventEmitterHub::new();
        let arena = crate::tree::NodeArena::new();
        let target = arena.root();

        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();
        hub.on_node(target, move |event: &mut Event| {
            if event.phase() == EventPhase::Target {
                *called_clone.lock().unwrap() = true;
            }
            EventResult::Ignored
        });

        let mut event = Event::Key(KeyEvent::new(Key::Enter, target));
        hub.dispatch(&mut event, &arena);

        assert!(*called.lock().unwrap());
    }

    #[test]
    fn hub_clear_global_handlers() {
        let mut hub = EventEmitterHub::new();
        hub.on_global(|_| EventResult::Ignored);
        hub.on_global(|_| EventResult::Ignored);
        assert_eq!(hub.global_handler_count(), 2);

        hub.clear_global_handlers();
        assert_eq!(hub.global_handler_count(), 0);
    }
}
