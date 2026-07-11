use crate::tree::node_id::NodeId;
use crate::tree::visual::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventPhase {
    Capture,
    Target,
    Bubble,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

impl Modifiers {
    pub const NONE: Self = Self {
        ctrl: false,
        shift: false,
        alt: false,
        meta: false,
    };

    pub fn is_empty(&self) -> bool {
        !self.ctrl && !self.shift && !self.alt && !self.meta
    }
}

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
        Self {
            key,
            modifiers: Modifiers::NONE,
            target,
            phase: EventPhase::Target,
            default_prevented: false,
        }
    }

    pub fn with_modifiers(mut self, modifiers: Modifiers) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn prevent_default(&mut self) {
        self.default_prevented = true;
    }
}

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

#[derive(Debug, Clone)]
pub struct PasteEvent {
    pub text: Box<str>,
    pub target: NodeId,
    pub phase: EventPhase,
    pub default_prevented: bool,
}

impl PasteEvent {
    pub fn new(text: impl Into<Box<str>>, target: NodeId) -> Self {
        Self {
            text: text.into(),
            target,
            phase: EventPhase::Target,
            default_prevented: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FocusEvent {
    pub target: NodeId,
    pub previous: Option<NodeId>,
    pub phase: EventPhase,
}

impl FocusEvent {
    pub fn new(target: NodeId, previous: Option<NodeId>) -> Self {
        Self {
            target,
            previous,
            phase: EventPhase::Target,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlurEvent {
    pub target: NodeId,
    pub next: Option<NodeId>,
    pub phase: EventPhase,
}

impl BlurEvent {
    pub fn new(target: NodeId, next: Option<NodeId>) -> Self {
        Self {
            target,
            next,
            phase: EventPhase::Target,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ResizeEvent {
    pub width: u16,
    pub height: u16,
    pub previous_width: u16,
    pub previous_height: u16,
}

impl ResizeEvent {
    pub fn new(width: u16, height: u16, previous_width: u16, previous_height: u16) -> Self {
        Self {
            width,
            height,
            previous_width,
            previous_height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleEvent {
    Mount,
    Unmount,
    Suspend,
    Resume,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    Consumed,
    Ignored,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_id() -> NodeId {
        let mut arena = crate::tree::arena::NodeArena::new();
        arena.insert(crate::tree::render_node::RenderNode::new(
            crate::tree::node_kind::NodeKind::Box,
        ))
    }

    #[test]
    fn event_key_creation() {
        let id = make_id();
        let event = KeyEvent::new(Key::Character('a'), id);
        assert_eq!(event.key, Key::Character('a'));
        assert_eq!(event.target, id);
        assert_eq!(event.phase, EventPhase::Target);
        assert!(!event.default_prevented);
    }

    #[test]
    fn event_key_prevent_default() {
        let id = make_id();
        let mut event = KeyEvent::new(Key::Enter, id);
        event.prevent_default();
        assert!(event.default_prevented);
    }

    #[test]
    fn event_key_with_modifiers() {
        let id = make_id();
        let event = KeyEvent::new(Key::Character('c'), id).with_modifiers(Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        });
        assert!(event.modifiers.ctrl);
        assert!(!event.modifiers.shift);
    }

    #[test]
    fn event_mouse_creation() {
        let id = make_id();
        let event = MouseEvent::new(MouseButton::Left, Point::new(5, 10), id);
        assert_eq!(event.button, MouseButton::Left);
        assert_eq!(event.position, Point::new(5, 10));
    }

    #[test]
    fn event_paste_creation() {
        let id = make_id();
        let event = PasteEvent::new("hello world", id);
        assert_eq!(event.text.as_ref(), "hello world");
    }

    #[test]
    fn event_focus_creation() {
        let id = make_id();
        let event = FocusEvent::new(id, None);
        assert_eq!(event.target, id);
        assert!(event.previous.is_none());
    }

    #[test]
    fn event_blur_creation() {
        let id = make_id();
        let event = BlurEvent::new(id, None);
        assert_eq!(event.target, id);
        assert!(event.next.is_none());
    }

    #[test]
    fn event_resize_creation() {
        let event = ResizeEvent::new(120, 40, 80, 24);
        assert_eq!(event.width, 120);
        assert_eq!(event.height, 40);
        assert_eq!(event.previous_width, 80);
        assert_eq!(event.previous_height, 24);
    }

    #[test]
    fn event_lifecycle_variants() {
        assert_eq!(LifecycleEvent::Mount, LifecycleEvent::Mount);
        assert_eq!(LifecycleEvent::Unmount, LifecycleEvent::Unmount);
        assert_ne!(LifecycleEvent::Mount, LifecycleEvent::Unmount);
    }

    #[test]
    fn modifiers_none() {
        let m = Modifiers::NONE;
        assert!(m.is_empty());
    }

    #[test]
    fn modifiers_with_ctrl() {
        let m = Modifiers {
            ctrl: true,
            ..Modifiers::NONE
        };
        assert!(!m.is_empty());
    }

    #[test]
    fn event_phase_set() {
        let id = make_id();
        let mut event = Event::Key(KeyEvent::new(Key::Enter, id));
        event.set_phase(EventPhase::Capture);
        assert_eq!(event.phase(), EventPhase::Capture);
    }

    #[test]
    fn event_target() {
        let id = make_id();
        let event = Event::Key(KeyEvent::new(Key::Enter, id));
        assert_eq!(event.target(), Some(id));
    }

    #[test]
    fn event_resize_no_target() {
        let event = Event::Resize(ResizeEvent::new(80, 24, 80, 24));
        assert!(event.target().is_none());
    }

    #[test]
    fn key_variants() {
        assert_eq!(Key::Enter, Key::Enter);
        assert_eq!(Key::Escape, Key::Escape);
        assert_eq!(Key::Backspace, Key::Backspace);
        assert_eq!(Key::Delete, Key::Delete);
        assert_eq!(Key::Tab, Key::Tab);
        assert_eq!(Key::Space, Key::Space);
        assert_eq!(Key::ArrowUp, Key::ArrowUp);
        assert_eq!(Key::ArrowDown, Key::ArrowDown);
        assert_eq!(Key::ArrowLeft, Key::ArrowLeft);
        assert_eq!(Key::ArrowRight, Key::ArrowRight);
        assert_eq!(Key::Home, Key::Home);
        assert_eq!(Key::End, Key::End);
        assert_eq!(Key::PageUp, Key::PageUp);
        assert_eq!(Key::PageDown, Key::PageDown);
        assert_eq!(Key::F(1), Key::F(1));
        assert_eq!(Key::F(12), Key::F(12));
        assert_eq!(Key::Ctrl('c'), Key::Ctrl('c'));
        assert_eq!(Key::Alt('x'), Key::Alt('x'));
    }

    #[test]
    fn mouse_button_variants() {
        assert_eq!(MouseButton::Left, MouseButton::Left);
        assert_eq!(MouseButton::Right, MouseButton::Right);
        assert_eq!(MouseButton::Middle, MouseButton::Middle);
        assert_eq!(MouseButton::ScrollUp, MouseButton::ScrollUp);
        assert_eq!(MouseButton::ScrollDown, MouseButton::ScrollDown);
        assert_eq!(MouseButton::None, MouseButton::None);
    }
}
