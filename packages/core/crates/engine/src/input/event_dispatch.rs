use std::collections::HashMap;

use super::event_types::{Event, EventPhase, EventResult};
use crate::tree::NodeArena;
use crate::tree::NodeId;

type EventHandlerFn = dyn FnMut(&mut Event) -> EventResult + Send;
type EventHandler = Box<EventHandlerFn>;
type HandlerVec = Vec<EventHandler>;

pub struct EventDispatcher {
    handlers: HashMap<NodeId, HandlerVec>,
    global_handlers: HandlerVec,
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            global_handlers: Vec::new(),
        }
    }

    pub fn on(
        &mut self,
        node_id: NodeId,
        handler: impl FnMut(&mut Event) -> EventResult + Send + 'static,
    ) {
        self.handlers
            .entry(node_id)
            .or_default()
            .push(Box::new(handler));
    }

    pub fn on_global(&mut self, handler: impl FnMut(&mut Event) -> EventResult + Send + 'static) {
        self.global_handlers.push(Box::new(handler));
    }

    pub fn remove_handlers(&mut self, node_id: NodeId) {
        self.handlers.remove(&node_id);
    }

    pub fn clear(&mut self) {
        self.handlers.clear();
        self.global_handlers.clear();
    }

    pub fn handler_count(&self) -> usize {
        self.handlers.values().map(|v| v.len()).sum::<usize>() + self.global_handlers.len()
    }

    pub fn has_handlers(&self, node_id: NodeId) -> bool {
        self.handlers.contains_key(&node_id)
    }

    pub fn dispatch(&mut self, event: &mut Event, arena: &NodeArena) -> EventResult {
        for handler in &mut self.global_handlers {
            let result = handler(event);
            if result == EventResult::Consumed {
                return EventResult::Consumed;
            }
        }

        let target = match event.target() {
            Some(id) => id,
            None => return EventResult::Ignored,
        };

        let ancestors = arena.ancestors(target);

        for &ancestor in ancestors.iter().rev() {
            event.set_phase(EventPhase::Capture);
            if let Some(handlers) = self.handlers.get_mut(&ancestor) {
                for handler in handlers {
                    let result = handler(event);
                    if result == EventResult::Consumed || event.is_consumed() {
                        return EventResult::Consumed;
                    }
                }
            }
        }

        event.set_phase(EventPhase::Target);
        if let Some(handlers) = self.handlers.get_mut(&target) {
            for handler in handlers {
                let result = handler(event);
                if result == EventResult::Consumed || event.is_consumed() {
                    return EventResult::Consumed;
                }
            }
        }

        for &ancestor in &ancestors {
            event.set_phase(EventPhase::Bubble);
            if let Some(handlers) = self.handlers.get_mut(&ancestor) {
                for handler in handlers {
                    let result = handler(event);
                    if result == EventResult::Consumed || event.is_consumed() {
                        return EventResult::Consumed;
                    }
                }
            }
        }

        EventResult::Ignored
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{Key, KeyEvent};
    use crate::tree::NodeArena;
    use crate::tree::NodeKind;
    use crate::tree::RenderNode;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn make_arena_with_child() -> (NodeArena, NodeId) {
        let mut arena = NodeArena::new();
        let child = arena.insert(RenderNode::new(NodeKind::Box));
        arena.append_child(arena.root(), child).unwrap();
        (arena, child)
    }

    #[test]
    fn dispatcher_new() {
        let dispatcher = EventDispatcher::new();
        assert_eq!(dispatcher.handler_count(), 0);
    }

    #[test]
    fn dispatcher_register_handler() {
        let mut dispatcher = EventDispatcher::new();
        let (_arena, child) = make_arena_with_child();
        dispatcher.on(child, |_: &mut Event| EventResult::Ignored);
        assert!(dispatcher.has_handlers(child));
        assert_eq!(dispatcher.handler_count(), 1);
    }

    #[test]
    fn dispatcher_remove_handlers() {
        let mut dispatcher = EventDispatcher::new();
        let (_arena, child) = make_arena_with_child();
        dispatcher.on(child, |_: &mut Event| EventResult::Ignored);
        dispatcher.remove_handlers(child);
        assert!(!dispatcher.has_handlers(child));
    }

    #[test]
    fn dispatcher_dispatch_to_target() {
        let mut dispatcher = EventDispatcher::new();
        let (arena, child) = make_arena_with_child();

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        dispatcher.on(child, move |_: &mut Event| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            EventResult::Consumed
        });

        let mut event = Event::Key(KeyEvent::new(Key::Enter, child));
        let result = dispatcher.dispatch(&mut event, &arena);
        assert_eq!(result, EventResult::Consumed);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn dispatcher_capture_phase() {
        let mut dispatcher = EventDispatcher::new();
        let (arena, child) = make_arena_with_child();
        let root = arena.root();

        let order = Arc::new(std::sync::Mutex::new(Vec::new()));
        let order_clone = order.clone();

        dispatcher.on(root, move |_: &mut Event| {
            order_clone.lock().unwrap().push("capture_root");
            EventResult::Ignored
        });

        let order_clone2 = order.clone();
        dispatcher.on(child, move |_: &mut Event| {
            order_clone2.lock().unwrap().push("target_child");
            EventResult::Consumed
        });

        let mut event = Event::Key(KeyEvent::new(Key::Enter, child));
        dispatcher.dispatch(&mut event, &arena);

        let captured = order.lock().unwrap();
        assert_eq!(captured.len(), 2);
        assert_eq!(captured[0], "capture_root");
        assert_eq!(captured[1], "target_child");
    }

    #[test]
    fn dispatcher_bubble_phase() {
        let mut dispatcher = EventDispatcher::new();
        let (arena, child) = make_arena_with_child();
        let root = arena.root();

        let order = Arc::new(std::sync::Mutex::new(Vec::new()));

        let order_clone = order.clone();
        dispatcher.on(root, move |_: &mut Event| {
            order_clone.lock().unwrap().push("root_handler");
            EventResult::Ignored
        });

        let order_clone2 = order.clone();
        dispatcher.on(child, move |event: &mut Event| {
            order_clone2.lock().unwrap().push("child_handler");
            event.set_phase(EventPhase::Bubble);
            EventResult::Ignored
        });

        let mut event = Event::Key(KeyEvent::new(Key::Enter, child));
        dispatcher.dispatch(&mut event, &arena);

        let captured = order.lock().unwrap();
        assert!(captured.contains(&"child_handler"));
        assert!(captured.contains(&"root_handler"));
    }

    #[test]
    fn dispatcher_global_handler() {
        let mut dispatcher = EventDispatcher::new();
        let (arena, child) = make_arena_with_child();

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        dispatcher.on_global(move |_: &mut Event| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            EventResult::Ignored
        });

        let mut event = Event::Key(KeyEvent::new(Key::Enter, child));
        dispatcher.dispatch(&mut event, &arena);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn dispatcher_consumed_stops_bubble() {
        let mut dispatcher = EventDispatcher::new();
        let (arena, child) = make_arena_with_child();
        let root = arena.root();

        let order = Arc::new(std::sync::Mutex::new(Vec::new()));

        let o = order.clone();
        dispatcher.on(root, move |event: &mut Event| {
            if event.phase() == EventPhase::Bubble {
                o.lock().unwrap().push("root_bubble");
            }
            EventResult::Ignored
        });

        let o = order.clone();
        dispatcher.on(child, move |_: &mut Event| {
            o.lock().unwrap().push("child_target");
            EventResult::Consumed
        });

        let mut event = Event::Key(KeyEvent::new(Key::Enter, child));
        dispatcher.dispatch(&mut event, &arena);

        let captured = order.lock().unwrap();
        assert!(captured.contains(&"child_target"));
        assert!(!captured.contains(&"root_bubble"));
    }

    #[test]
    fn dispatcher_clear() {
        let mut dispatcher = EventDispatcher::new();
        let (_arena, child) = make_arena_with_child();
        dispatcher.on(child, |_: &mut Event| EventResult::Ignored);
        dispatcher.on_global(|_: &mut Event| EventResult::Ignored);
        dispatcher.clear();
        assert_eq!(dispatcher.handler_count(), 0);
    }
}
