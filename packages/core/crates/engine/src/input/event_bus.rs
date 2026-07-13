use std::collections::VecDeque;

use super::event_dispatch::EventDispatcher;
use super::event_types::{Event, EventResult, MouseButton, MouseEvent};
use crate::tree::arena::NodeArena;
use crate::tree::visual::Point;

pub struct EventBus {
    queue: VecDeque<Event>,
    max_queue_size: usize,
    coalesce_mouse: bool,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            max_queue_size: 256,
            coalesce_mouse: true,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            max_queue_size: 256,
            coalesce_mouse: true,
        }
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

    pub fn push_key(
        &mut self,
        key: super::event_types::Key,
        modifiers: super::event_types::Modifiers,
        target: crate::tree::node_id::NodeId,
    ) {
        let mut event = Event::Key(super::event_types::KeyEvent::new(key, target));
        if let Event::Key(ref mut ke) = event {
            ke.modifiers = modifiers;
        }
        self.push(event);
    }

    pub fn push_mouse(
        &mut self,
        button: MouseButton,
        position: Point,
        target: crate::tree::node_id::NodeId,
    ) {
        self.push(Event::Mouse(MouseEvent::new(button, position, target)));
    }

    pub fn push_paste(
        &mut self,
        text: impl Into<std::sync::Arc<str>>,
        target: crate::tree::node_id::NodeId,
    ) {
        let text: std::sync::Arc<str> = text.into();
        let text_box: Box<str> = Box::from(text.as_ref());
        self.push(Event::Paste(super::event_types::PasteEvent::new(
            text_box, target,
        )));
    }

    pub fn push_resize(&mut self, width: u16, height: u16, prev_width: u16, prev_height: u16) {
        self.push(Event::Resize(super::event_types::ResizeEvent::new(
            width,
            height,
            prev_width,
            prev_height,
        )));
    }

    pub fn push_lifecycle(&mut self, event: super::event_types::LifecycleEvent) {
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

    pub fn process_all(&mut self, dispatcher: &mut EventDispatcher, arena: &NodeArena) {
        let events: VecDeque<Event> = self.drain();
        for mut event in events {
            let _ = dispatcher.dispatch(&mut event, arena);
        }
    }

    pub fn process_until_consumed(
        &mut self,
        dispatcher: &mut EventDispatcher,
        arena: &NodeArena,
    ) -> Option<EventResult> {
        while let Some(mut event) = self.queue.pop_front() {
            let result = dispatcher.dispatch(&mut event, arena);
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
    use crate::input::{Key, KeyEvent, LifecycleEvent, Modifiers};
    use crate::tree::arena::NodeArena;
    use crate::tree::node_id::NodeId;
    use crate::tree::node_kind::NodeKind;
    use crate::tree::render_node::RenderNode;

    fn make_target(arena: &mut NodeArena) -> NodeId {
        let id = arena.insert(RenderNode::new(NodeKind::Box));
        arena.append_child(arena.root(), id).unwrap();
        id
    }

    #[test]
    fn bus_new() {
        let bus = EventBus::new();
        assert!(bus.is_empty());
        assert_eq!(bus.len(), 0);
    }

    #[test]
    fn bus_push_and_drain() {
        let mut bus = EventBus::new();
        let mut arena = NodeArena::new();
        let target = make_target(&mut arena);

        bus.push(Event::Key(KeyEvent::new(Key::Enter, target)));
        assert_eq!(bus.len(), 1);

        let events = bus.drain();
        assert_eq!(events.len(), 1);
        assert!(bus.is_empty());
    }

    #[test]
    fn bus_push_key() {
        let mut bus = EventBus::new();
        let mut arena = NodeArena::new();
        let target = make_target(&mut arena);

        bus.push_key(Key::Enter, Modifiers::NONE, target);
        assert_eq!(bus.len(), 1);
    }

    #[test]
    fn bus_push_mouse() {
        let mut bus = EventBus::new();
        let mut arena = NodeArena::new();
        let target = make_target(&mut arena);

        bus.push_mouse(MouseButton::Left, Point::new(5, 10), target);
        assert_eq!(bus.len(), 1);
    }

    #[test]
    fn bus_push_resize() {
        let mut bus = EventBus::new();
        bus.push_resize(120, 40, 80, 24);
        assert_eq!(bus.len(), 1);
    }

    #[test]
    fn bus_push_lifecycle() {
        let mut bus = EventBus::new();
        bus.push_lifecycle(LifecycleEvent::Mount);
        assert_eq!(bus.len(), 1);
    }

    #[test]
    fn bus_coalesce_mouse() {
        let mut bus = EventBus::new();
        let mut arena = NodeArena::new();
        let target = make_target(&mut arena);

        bus.push_mouse(MouseButton::Left, Point::new(1, 1), target);
        bus.push_mouse(MouseButton::Left, Point::new(2, 2), target);
        bus.push_mouse(MouseButton::Left, Point::new(3, 3), target);

        assert_eq!(bus.len(), 1);
    }

    #[test]
    fn bus_no_coalesce_different_buttons() {
        let mut bus = EventBus::new();
        let mut arena = NodeArena::new();
        let target = make_target(&mut arena);

        bus.push_mouse(MouseButton::Left, Point::new(1, 1), target);
        bus.push_mouse(MouseButton::Right, Point::new(2, 2), target);

        assert_eq!(bus.len(), 2);
    }

    #[test]
    fn bus_max_queue_size() {
        let mut bus = EventBus::new();
        let mut arena = NodeArena::new();
        let target = make_target(&mut arena);

        bus.max_queue_size = 3;
        for _ in 0..5 {
            bus.push(Event::Key(KeyEvent::new(Key::Enter, target)));
        }

        assert_eq!(bus.len(), 3);
    }

    #[test]
    fn bus_clear() {
        let mut bus = EventBus::new();
        let mut arena = NodeArena::new();
        let target = make_target(&mut arena);

        bus.push(Event::Key(KeyEvent::new(Key::Enter, target)));
        bus.clear();
        assert!(bus.is_empty());
    }

    #[test]
    fn bus_process_all() {
        let mut bus = EventBus::new();
        let mut arena = NodeArena::new();
        let target = make_target(&mut arena);

        bus.push(Event::Key(KeyEvent::new(Key::Enter, target)));
        bus.push(Event::Key(KeyEvent::new(Key::Escape, target)));

        let mut dispatcher = EventDispatcher::new();
        bus.process_all(&mut dispatcher, &arena);
        assert!(bus.is_empty());
    }

    #[test]
    fn bus_disable_coalesce() {
        let mut bus = EventBus::new();
        bus.set_coalesce_mouse(false);

        let mut arena = NodeArena::new();
        let target = make_target(&mut arena);

        bus.push_mouse(MouseButton::Left, Point::new(1, 1), target);
        bus.push_mouse(MouseButton::Left, Point::new(2, 2), target);

        assert_eq!(bus.len(), 2);
    }
}
