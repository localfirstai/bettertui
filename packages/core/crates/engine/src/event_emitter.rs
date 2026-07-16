//! Generic typed event emitter with listener registration and emission.
//!
//! Provides a type-safe pub/sub pattern for event handling with support for
//! priority-based dispatch and listener removal.

use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

/// Unique identifier for a registered listener.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListenerId(pub u64);

impl ListenerId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl Default for ListenerId {
    fn default() -> Self {
        Self(0)
    }
}

/// A boxed event listener function.
pub type BoxedListener<E> = Box<dyn Fn(&E) -> ListenerResult + Send + Sync>;

/// Result of a listener handling an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListenerResult {
    /// Event was consumed, stop propagation.
    Consumed,
    /// Event was not consumed, continue propagation.
    Continue,
}

impl Default for ListenerResult {
    fn default() -> Self {
        Self::Continue
    }
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
    key_emitter: EventEmitter<crate::input::KeyEvent>,
    mouse_emitter: EventEmitter<crate::input::MouseEvent>,
    paste_emitter: EventEmitter<crate::input::PasteEvent>,
    focus_emitter: EventEmitter<crate::input::FocusEvent>,
    blur_emitter: EventEmitter<crate::input::BlurEvent>,
    resize_emitter: EventEmitter<crate::input::ResizeEvent>,
    lifecycle_emitter: EventEmitter<crate::input::LifecycleEvent>,
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
        F: Fn(&crate::input::KeyEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.key_emitter.on(listener)
    }

    pub fn on_mouse<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::input::MouseEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.mouse_emitter.on(listener)
    }

    pub fn on_paste<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::input::PasteEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.paste_emitter.on(listener)
    }

    pub fn on_focus<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::input::FocusEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.focus_emitter.on(listener)
    }

    pub fn on_blur<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::input::BlurEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.blur_emitter.on(listener)
    }

    pub fn on_resize<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::input::ResizeEvent) -> ListenerResult + Send + Sync + 'static,
    {
        self.resize_emitter.on(listener)
    }

    pub fn on_lifecycle<F>(&mut self, listener: F) -> ListenerId
    where
        F: Fn(&crate::input::LifecycleEvent) -> ListenerResult + Send + Sync + 'static,
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

    pub fn emit_key(&mut self, event: &crate::input::KeyEvent) -> bool {
        self.key_emitter.emit(event)
    }

    pub fn emit_mouse(&mut self, event: &crate::input::MouseEvent) -> bool {
        self.mouse_emitter.emit(event)
    }

    pub fn emit_paste(&mut self, event: &crate::input::PasteEvent) -> bool {
        self.paste_emitter.emit(event)
    }

    pub fn emit_focus(&mut self, event: &crate::input::FocusEvent) -> bool {
        self.focus_emitter.emit(event)
    }

    pub fn emit_blur(&mut self, event: &crate::input::BlurEvent) -> bool {
        self.blur_emitter.emit(event)
    }

    pub fn emit_resize(&mut self, event: &crate::input::ResizeEvent) -> bool {
        self.resize_emitter.emit(event)
    }

    pub fn emit_lifecycle(&mut self, event: &crate::input::LifecycleEvent) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

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
}
