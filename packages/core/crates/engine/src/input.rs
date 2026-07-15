//! Input system: event model, bus, dispatch, focus management, keyboard, mouse, clipboard, keybindings.

use std::collections::VecDeque;

// === clipboard.rs ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardAction {
    Copy,
    Paste,
    Cut,
}

#[derive(Debug, Clone)]
pub struct ClipboardInput {
    pub data: String,
    pub action: ClipboardAction,
}

impl ClipboardInput {
    pub fn new(action: ClipboardAction, data: String) -> Self {
        Self { data, action }
    }

    pub fn copy(data: String) -> Self {
        Self::new(ClipboardAction::Copy, data)
    }

    pub fn paste(data: String) -> Self {
        Self::new(ClipboardAction::Paste, data)
    }

    pub fn cut(data: String) -> Self {
        Self::new(ClipboardAction::Cut, data)
    }

    pub fn is_copy(&self) -> bool {
        self.action == ClipboardAction::Copy
    }

    pub fn is_paste(&self) -> bool {
        self.action == ClipboardAction::Paste
    }

    pub fn is_cut(&self) -> bool {
        self.action == ClipboardAction::Cut
    }
}

impl Default for ClipboardInput {
    fn default() -> Self {
        Self::new(ClipboardAction::Copy, String::new())
    }
}

// === keyboard.rs ===

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0001;
        const CONTROL = 0b0010;
        const ALT = 0b0100;
        const SUPER = 0b1000;
    }
}

impl Default for KeyModifiers {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    Press,
    Release,
    Repeat,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Char(char),
    Enter,
    Esc,
    Backspace,
    Tab,
    Space,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    F(u8),
    Modifier(KeyModifiers),
    Media(MediaKey),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKey {
    PlayPause,
    NextTrack,
    PreviousTrack,
    Stop,
    VolumeUp,
    VolumeDown,
    Mute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardInput {
    pub key: char,
    pub modifiers: KeyModifiers,
    pub action: KeyAction,
}

impl KeyboardInput {
    pub fn new(key: char, modifiers: KeyModifiers) -> Self {
        Self { key, modifiers, action: KeyAction::Press }
    }

    pub fn with_action(mut self, action: KeyAction) -> Self {
        self.action = action;
        self
    }

    pub fn press(key: char, modifiers: KeyModifiers) -> Self {
        Self::new(key, modifiers).with_action(KeyAction::Press)
    }

    pub fn release(key: char, modifiers: KeyModifiers) -> Self {
        Self::new(key, modifiers).with_action(KeyAction::Release)
    }

    pub fn repeat(key: char, modifiers: KeyModifiers) -> Self {
        Self::new(key, modifiers).with_action(KeyAction::Repeat)
    }

    pub fn is_ctrl(&self) -> bool {
        self.modifiers.contains(KeyModifiers::CONTROL)
    }

    pub fn is_shift(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SHIFT)
    }

    pub fn is_alt(&self) -> bool {
        self.modifiers.contains(KeyModifiers::ALT)
    }

    pub fn is_super(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SUPER)
    }

    pub fn is_modifier_only(&self) -> bool {
        matches!(self.key, '\x00' | '\x1b' | '\x08' | '\x09' | '\x0d' | '\x0a')
    }

    pub fn to_display_string(&self) -> String {
        let mut result = String::new();

        if self.is_ctrl() {
            result.push_str("Ctrl+");
        }
        if self.is_shift() {
            result.push_str("Shift+");
        }
        if self.is_alt() {
            result.push_str("Alt+");
        }
        if self.is_super() {
            result.push_str("Super+");
        }

        match self.key {
            '\x00' => result.push_str("Null"),
            '\x1b' => result.push_str("Esc"),
            '\x08' => result.push_str("Backspace"),
            '\x09' => result.push_str("Tab"),
            '\x0d' => result.push_str("Enter"),
            '\x0a' => result.push_str("LineFeed"),
            ' ' => result.push_str("Space"),
            c => result.push(c),
        }

        result
    }
}

impl Default for KeyboardInput {
    fn default() -> Self {
        Self::new('\0', KeyModifiers::empty())
    }
}

// === mouse.rs ===

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MouseButtons: u8 {
        const LEFT = 0b0001;
        const RIGHT = 0b0010;
        const MIDDLE = 0b0100;
        const EXTRA1 = 0b1000;
        const EXTRA2 = 0b10000;
    }
}

impl Default for MouseButtons {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
    Press,
    Release,
    Move,
    Scroll,
    Drag,
    Drop,
}

#[derive(Debug, Clone)]
pub struct MouseInput {
    pub x: u16,
    pub y: u16,
    pub buttons: MouseButtons,
    pub modifiers: KeyModifiers,
    pub event_type: MouseEventType,
    pub scroll_delta: Option<(i16, i16)>,
}

impl MouseInput {
    pub fn new(x: u16, y: u16, buttons: MouseButtons) -> Self {
        Self { x, y, buttons, modifiers: KeyModifiers::empty(), event_type: MouseEventType::Press, scroll_delta: None }
    }

    pub fn with_modifiers(mut self, modifiers: KeyModifiers) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn with_event_type(mut self, event_type: MouseEventType) -> Self {
        self.event_type = event_type;
        self
    }

    pub fn with_scroll_delta(mut self, delta_x: i16, delta_y: i16) -> Self {
        self.scroll_delta = Some((delta_x, delta_y));
        self
    }

    pub fn press(x: u16, y: u16, buttons: MouseButtons) -> Self {
        Self::new(x, y, buttons).with_event_type(MouseEventType::Press)
    }

    pub fn release(x: u16, y: u16, buttons: MouseButtons) -> Self {
        Self::new(x, y, buttons).with_event_type(MouseEventType::Release)
    }

    pub fn move_to(x: u16, y: u16) -> Self {
        Self::new(x, y, MouseButtons::empty()).with_event_type(MouseEventType::Move)
    }

    pub fn scroll(x: u16, y: u16, delta_x: i16, delta_y: i16) -> Self {
        Self::new(x, y, MouseButtons::empty())
            .with_event_type(MouseEventType::Scroll)
            .with_scroll_delta(delta_x, delta_y)
    }

    pub fn drag(x: u16, y: u16, buttons: MouseButtons) -> Self {
        Self::new(x, y, buttons).with_event_type(MouseEventType::Drag)
    }

    pub fn drop(x: u16, y: u16, buttons: MouseButtons) -> Self {
        Self::new(x, y, buttons).with_event_type(MouseEventType::Drop)
    }

    pub fn is_left_button(&self) -> bool {
        self.buttons.contains(MouseButtons::LEFT)
    }

    pub fn is_right_button(&self) -> bool {
        self.buttons.contains(MouseButtons::RIGHT)
    }

    pub fn is_middle_button(&self) -> bool {
        self.buttons.contains(MouseButtons::MIDDLE)
    }

    pub fn is_ctrl(&self) -> bool {
        self.modifiers.contains(KeyModifiers::CONTROL)
    }

    pub fn is_shift(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SHIFT)
    }

    pub fn is_alt(&self) -> bool {
        self.modifiers.contains(KeyModifiers::ALT)
    }
}

impl Default for MouseInput {
    fn default() -> Self {
        Self::new(0, 0, MouseButtons::empty())
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MouseInputEvent {
    pub input: MouseInput,
    pub timestamp: u64,
}

impl MouseInputEvent {
    #[allow(dead_code)]
    pub fn new(input: MouseInput) -> Self {
        Self {
            input,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }
}

// === event.rs ===

#[derive(Debug, Clone)]
pub enum InputEventType {
    Keyboard(KeyboardInput),
    Mouse(MouseInput),
    Clipboard(ClipboardInput),
    Resize(u16, u16),
    Focus,
    Blur,
    Paste(String),
}

#[derive(Debug, Clone)]
pub struct InputEvent {
    pub event_type: InputEventType,
    pub timestamp: u64,
}

impl InputEvent {
    pub fn new(event_type: InputEventType) -> Self {
        Self {
            event_type,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    pub fn keyboard(input: KeyboardInput) -> Self {
        Self::new(InputEventType::Keyboard(input))
    }

    pub fn mouse(input: MouseInput) -> Self {
        Self::new(InputEventType::Mouse(input))
    }

    pub fn clipboard(input: ClipboardInput) -> Self {
        Self::new(InputEventType::Clipboard(input))
    }

    pub fn resize(width: u16, height: u16) -> Self {
        Self::new(InputEventType::Resize(width, height))
    }

    pub fn focus() -> Self {
        Self::new(InputEventType::Focus)
    }

    pub fn blur() -> Self {
        Self::new(InputEventType::Blur)
    }

    pub fn paste(data: String) -> Self {
        Self::new(InputEventType::Paste(data))
    }

    pub fn is_keyboard(&self) -> bool {
        matches!(self.event_type, InputEventType::Keyboard(_))
    }

    pub fn is_mouse(&self) -> bool {
        matches!(self.event_type, InputEventType::Mouse(_))
    }

    pub fn is_clipboard(&self) -> bool {
        matches!(self.event_type, InputEventType::Clipboard(_))
    }

    pub fn is_resize(&self) -> bool {
        matches!(self.event_type, InputEventType::Resize(_, _))
    }

    pub fn is_focus(&self) -> bool {
        matches!(self.event_type, InputEventType::Focus)
    }

    pub fn is_blur(&self) -> bool {
        matches!(self.event_type, InputEventType::Blur)
    }

    pub fn is_paste(&self) -> bool {
        matches!(self.event_type, InputEventType::Paste(_))
    }
}

impl Default for InputEvent {
    fn default() -> Self {
        Self::new(InputEventType::Focus)
    }
}

// === event_types.rs ===

use crate::tree::NodeId;
use crate::tree::Point;

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
    pub const NONE: Self = Self { ctrl: false, shift: false, alt: false, meta: false };

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
        Self { text: text.into(), target, phase: EventPhase::Target, default_prevented: false }
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
        Self { target, previous, phase: EventPhase::Target }
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
        Self { target, next, phase: EventPhase::Target }
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
        Self { width, height, previous_width, previous_height }
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

// === event_dispatch.rs ===

use std::collections::HashMap;

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
        Self { handlers: HashMap::new(), global_handlers: Vec::new() }
    }

    pub fn on(&mut self, node_id: NodeId, handler: impl FnMut(&mut Event) -> EventResult + Send + 'static) {
        self.handlers.entry(node_id).or_default().push(Box::new(handler));
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

    pub fn dispatch(&mut self, event: &mut Event, arena: &crate::tree::NodeArena) -> EventResult {
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

// === event_bus.rs ===

pub struct EventBus {
    queue: VecDeque<Event>,
    pub max_queue_size: usize,
    coalesce_mouse: bool,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
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

    pub fn process_all(&mut self, dispatcher: &mut EventDispatcher, arena: &crate::tree::NodeArena) {
        let events: VecDeque<Event> = self.drain();
        for mut event in events {
            let _ = dispatcher.dispatch(&mut event, arena);
        }
    }

    pub fn process_until_consumed(
        &mut self,
        dispatcher: &mut EventDispatcher,
        arena: &crate::tree::NodeArena,
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

// === focus/events.rs ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusEventType {
    Focus,
    Blur,
    FocusIn,
    FocusOut,
}

#[derive(Debug, Clone)]
pub struct FocusEvent_ {
    pub node_id: NodeId,
    pub event_type: FocusEventType,
}

impl FocusEvent_ {
    pub fn new(node_id: NodeId, event_type: FocusEventType) -> Self {
        Self { node_id, event_type }
    }

    pub fn is_focus(&self) -> bool {
        self.event_type == FocusEventType::Focus
    }

    pub fn is_blur(&self) -> bool {
        self.event_type == FocusEventType::Blur
    }

    pub fn is_focus_in(&self) -> bool {
        self.event_type == FocusEventType::FocusIn
    }

    pub fn is_focus_out(&self) -> bool {
        self.event_type == FocusEventType::FocusOut
    }
}

// === focus/scope.rs ===

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FocusScopeType {
    Window,
    Panel,
    Modal,
    Popup,
    Tooltip,
}

#[derive(Debug, Clone)]
pub struct FocusScope {
    pub id: NodeId,
    pub scope_type: FocusScopeType,
    pub modal: bool,
    pub trap_focus: bool,
}

impl FocusScope {
    pub fn new(id: NodeId, scope_type: FocusScopeType) -> Self {
        Self { id, scope_type, modal: false, trap_focus: false }
    }

    pub fn with_modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    pub fn with_trap_focus(mut self, trap_focus: bool) -> Self {
        self.trap_focus = trap_focus;
        self
    }

    pub fn is_modal(&self) -> bool {
        self.modal
    }

    pub fn traps_focus(&self) -> bool {
        self.trap_focus
    }
}

impl Default for FocusScope {
    fn default() -> Self {
        Self::new(NodeId::default(), FocusScopeType::Window)
    }
}

// === focus/traversal.rs ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusDirection {
    Forward,
    Backward,
    Up,
    Down,
    Left,
    Right,
    First,
    Last,
}

pub struct FocusTraversal;

impl FocusTraversal {
    pub fn next(manager: &FocusManager) -> Option<NodeId> {
        let focusable = manager.focusable_nodes();
        if focusable.is_empty() {
            return None;
        }

        let current = manager.focused();
        if let Some(current_id) = current {
            if let Some(pos) = focusable.iter().position(|&id| id == current_id) {
                let next_pos = (pos + 1) % focusable.len();
                Some(focusable[next_pos])
            } else {
                Some(focusable[0])
            }
        } else {
            Some(focusable[0])
        }
    }

    pub fn previous(manager: &FocusManager) -> Option<NodeId> {
        let focusable = manager.focusable_nodes();
        if focusable.is_empty() {
            return None;
        }

        let current = manager.focused();
        if let Some(current_id) = current {
            if let Some(pos) = focusable.iter().position(|&id| id == current_id) {
                let prev_pos = if pos == 0 { focusable.len() - 1 } else { pos - 1 };
                Some(focusable[prev_pos])
            } else {
                Some(focusable[focusable.len() - 1])
            }
        } else {
            Some(focusable[focusable.len() - 1])
        }
    }

    pub fn first(manager: &FocusManager) -> Option<NodeId> {
        let focusable = manager.focusable_nodes();
        focusable.into_iter().next()
    }

    pub fn last(manager: &FocusManager) -> Option<NodeId> {
        let focusable = manager.focusable_nodes();
        focusable.into_iter().last()
    }

    pub fn traverse(manager: &FocusManager, direction: FocusDirection) -> Option<NodeId> {
        match direction {
            FocusDirection::Forward => Self::next(manager),
            FocusDirection::Backward => Self::previous(manager),
            FocusDirection::First => Self::first(manager),
            FocusDirection::Last => Self::last(manager),
            FocusDirection::Up | FocusDirection::Down | FocusDirection::Left | FocusDirection::Right => {
                Self::next(manager)
            }
        }
    }
}

// === focus/manager.rs ===

use std::collections::HashMap as StdHashMap;

#[derive(Debug, Clone)]
pub struct FocusManager {
    nodes: StdHashMap<NodeId, FocusState>,
    focused: Option<FocusId>,
    previous: Option<FocusId>,
    scopes: Vec<FocusScope>,
    tab_order: Vec<NodeId>,
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FocusManager {
    pub fn new() -> Self {
        Self { nodes: StdHashMap::new(), focused: None, previous: None, scopes: Vec::new(), tab_order: Vec::new() }
    }

    pub fn register(&mut self, node_id: NodeId, state: FocusState) {
        self.nodes.insert(node_id, state);
        self.update_tab_order();
    }

    pub fn unregister(&mut self, node_id: NodeId) {
        self.nodes.remove(&node_id);
        if self.focused.map(|f| f.node_id()) == Some(node_id) {
            self.focused = None;
        }
        self.update_tab_order();
    }

    pub fn focus(&mut self, node_id: NodeId) -> Option<FocusEvent_> {
        if !self.is_focusable(node_id) {
            return None;
        }

        let old_focused = self.focused;
        let new_focused = Some(FocusId::new(node_id));

        if old_focused == new_focused {
            return None;
        }

        self.previous = old_focused;
        self.focused = new_focused;

        let mut events = Vec::new();

        if let Some(old_id) = old_focused
            && let Some(state) = self.nodes.get_mut(&old_id.node_id())
        {
            state.focused = None;
            events.push(FocusEvent_ { node_id: old_id.node_id(), event_type: FocusEventType::Blur });
        }

        if let Some(state) = self.nodes.get_mut(&node_id) {
            state.focused = Some(FocusId::new(node_id));
            events.push(FocusEvent_ { node_id, event_type: FocusEventType::Focus });
        }

        events.first().cloned()
    }

    pub fn blur(&mut self, node_id: NodeId) -> Option<FocusEvent_> {
        if self.focused.map(|f| f.node_id()) == Some(node_id) {
            self.focused = None;
            if let Some(state) = self.nodes.get_mut(&node_id) {
                state.focused = None;
            }
            Some(FocusEvent_ { node_id, event_type: FocusEventType::Blur })
        } else {
            None
        }
    }

    pub fn focused(&self) -> Option<NodeId> {
        self.focused.map(|f| f.node_id())
    }

    pub fn previous(&self) -> Option<NodeId> {
        self.previous.map(|f| f.node_id())
    }

    pub fn restore(&mut self) -> Option<FocusEvent_> {
        if let Some(prev) = self.previous { self.focus(prev.node_id()) } else { None }
    }

    pub fn is_focusable(&self, node_id: NodeId) -> bool {
        self.nodes.get(&node_id).is_some_and(|state| state.is_focusable())
    }

    pub fn is_focused(&self, node_id: NodeId) -> bool {
        self.focused.map(|f| f.node_id()) == Some(node_id)
    }

    pub fn focusable_nodes(&self) -> Vec<NodeId> {
        self.nodes.iter().filter(|(_, state)| state.is_focusable()).map(|(id, _)| *id).collect()
    }

    pub fn tab_order(&self) -> &[NodeId] {
        &self.tab_order
    }

    fn update_tab_order(&mut self) {
        self.tab_order = self
            .nodes
            .iter()
            .filter(|(_, state)| state.is_focusable())
            .map(|(id, state)| (*id, state.tab_index))
            .collect::<Vec<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        self.tab_order.sort_by_key(|id| self.nodes.get(id).map_or(0, |state| state.tab_index));
    }

    pub fn push_scope(&mut self, scope: FocusScope) {
        self.scopes.push(scope);
    }

    pub fn pop_scope(&mut self) -> Option<FocusScope> {
        self.scopes.pop()
    }

    pub fn current_scope(&self) -> Option<&FocusScope> {
        self.scopes.last()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.focused = None;
        self.previous = None;
        self.scopes.clear();
        self.tab_order.clear();
    }
}

// === focus/mod.rs (FocusId, FocusState) ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FocusId(pub NodeId);

impl FocusId {
    pub fn new(node_id: NodeId) -> Self {
        Self(node_id)
    }

    pub fn node_id(&self) -> NodeId {
        self.0
    }
}

#[allow(clippy::derivable_impls)]
impl Default for FocusId {
    fn default() -> Self {
        Self(NodeId::default())
    }
}

#[derive(Debug, Clone)]
pub struct FocusState {
    pub focused: Option<FocusId>,
    pub previous: Option<FocusId>,
    pub scope: FocusScope,
    pub tab_index: i32,
    pub focusable: bool,
}

impl Default for FocusState {
    fn default() -> Self {
        Self::new()
    }
}

impl FocusState {
    pub fn new() -> Self {
        Self { focused: None, previous: None, scope: FocusScope::default(), tab_index: 0, focusable: true }
    }

    pub fn with_focusable(focusable: bool) -> Self {
        Self { focusable, ..Self::new() }
    }

    pub fn with_tab_index(tab_index: i32) -> Self {
        Self { tab_index, ..Self::new() }
    }

    pub fn is_focusable(&self) -> bool {
        self.focusable
    }

    pub fn is_focused(&self) -> bool {
        self.focused.is_some()
    }
}

// === Input Runtime (from input/mod.rs) ===

#[derive(Debug, Clone)]
pub struct InputRuntime {
    events: VecDeque<InputEvent>,
    clipboard: ClipboardState,
    mouse_state: MouseState,
    keyboard_state: KeyboardState,
}

#[derive(Debug, Clone)]
pub struct ClipboardState {
    pub content: Option<String>,
    pub selection: Option<String>,
    pub primary: Option<String>,
}

impl Default for ClipboardState {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardState {
    pub fn new() -> Self {
        Self { content: None, selection: None, primary: None }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = Some(content);
    }

    pub fn get_content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    pub fn set_selection(&mut self, selection: String) {
        self.selection = Some(selection);
    }

    pub fn get_selection(&self) -> Option<&str> {
        self.selection.as_deref()
    }

    pub fn set_primary(&mut self, primary: String) {
        self.primary = Some(primary);
    }

    pub fn get_primary(&self) -> Option<&str> {
        self.primary.as_deref()
    }

    pub fn clear(&mut self) {
        self.content = None;
        self.selection = None;
        self.primary = None;
    }
}

#[derive(Debug, Clone)]
pub struct MouseState {
    pub position: (u16, u16),
    pub buttons: MouseButtons,
    pub modifiers: KeyModifiers,
    pub scroll_direction: Option<ScrollDirection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Default for MouseState {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            buttons: MouseButtons::empty(),
            modifiers: KeyModifiers::empty(),
            scroll_direction: None,
        }
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        self.position = (x, y);
    }

    pub fn set_buttons(&mut self, buttons: MouseButtons) {
        self.buttons = buttons;
    }

    pub fn set_modifiers(&mut self, modifiers: KeyModifiers) {
        self.modifiers = modifiers;
    }

    pub fn set_scroll(&mut self, direction: ScrollDirection) {
        self.scroll_direction = Some(direction);
    }

    pub fn clear_scroll(&mut self) {
        self.scroll_direction = None;
    }
}

#[derive(Debug, Clone)]
pub struct KeyboardState {
    pub modifiers: KeyModifiers,
    pub kitty_keyboard: bool,
    pub bracketed_paste: bool,
    pub focus_events: bool,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardState {
    pub fn new() -> Self {
        Self { modifiers: KeyModifiers::empty(), kitty_keyboard: false, bracketed_paste: false, focus_events: false }
    }

    pub fn with_kitty_keyboard(mut self, enabled: bool) -> Self {
        self.kitty_keyboard = enabled;
        self
    }

    pub fn with_bracketed_paste(mut self, enabled: bool) -> Self {
        self.bracketed_paste = enabled;
        self
    }

    pub fn with_focus_events(mut self, enabled: bool) -> Self {
        self.focus_events = enabled;
        self
    }

    pub fn set_modifiers(&mut self, modifiers: KeyModifiers) {
        self.modifiers = modifiers;
    }

    pub fn press_key(&mut self, key: char, modifiers: KeyModifiers) -> KeyboardInput {
        KeyboardInput { key, modifiers, action: KeyAction::Press }
    }

    pub fn release_key(&mut self, key: char, modifiers: KeyModifiers) -> KeyboardInput {
        KeyboardInput { key, modifiers, action: KeyAction::Release }
    }
}

impl Default for InputRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl InputRuntime {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            clipboard: ClipboardState::new(),
            mouse_state: MouseState::new(),
            keyboard_state: KeyboardState::new(),
        }
    }

    pub fn with_kitty_keyboard(mut self, enabled: bool) -> Self {
        self.keyboard_state = self.keyboard_state.with_kitty_keyboard(enabled);
        self
    }

    pub fn with_bracketed_paste(mut self, enabled: bool) -> Self {
        self.keyboard_state = self.keyboard_state.with_bracketed_paste(enabled);
        self
    }

    pub fn with_focus_events(mut self, enabled: bool) -> Self {
        self.keyboard_state = self.keyboard_state.with_focus_events(enabled);
        self
    }

    pub fn push_event(&mut self, event: InputEvent) {
        self.events.push_back(event);
    }

    pub fn poll_event(&mut self) -> Option<InputEvent> {
        self.events.pop_front()
    }

    pub fn events(&self) -> &VecDeque<InputEvent> {
        &self.events
    }

    pub fn clipboard(&self) -> &ClipboardState {
        &self.clipboard
    }

    pub fn clipboard_mut(&mut self) -> &mut ClipboardState {
        &mut self.clipboard
    }

    pub fn mouse_state(&self) -> &MouseState {
        &self.mouse_state
    }

    pub fn mouse_state_mut(&mut self) -> &mut MouseState {
        &mut self.mouse_state
    }

    pub fn keyboard_state(&self) -> &KeyboardState {
        &self.keyboard_state
    }

    pub fn keyboard_state_mut(&mut self) -> &mut KeyboardState {
        &mut self.keyboard_state
    }

    pub fn handle_keyboard_input(&mut self, input: KeyboardInput) {
        let event = InputEvent {
            event_type: InputEventType::Keyboard(input),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };
        self.push_event(event);
    }

    pub fn handle_mouse_input(&mut self, input: MouseInput) {
        let event = InputEvent {
            event_type: InputEventType::Mouse(input),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };
        self.push_event(event);
    }

    pub fn handle_clipboard_input(&mut self, input: ClipboardInput) {
        match input.action {
            ClipboardAction::Copy => {
                self.clipboard.set_content(input.data.clone());
            }
            ClipboardAction::Paste => {}
            ClipboardAction::Cut => {
                self.clipboard.set_content(input.data.clone());
            }
        }

        let event = InputEvent {
            event_type: InputEventType::Clipboard(input),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };
        self.push_event(event);
    }

    pub fn clear(&mut self) {
        self.events.clear();
        self.clipboard.clear();
        self.mouse_state = MouseState::new();
        self.keyboard_state = KeyboardState::new();
    }
}

// === keybinding/mod.rs ===

/// Represents a sequence of keys (for chord bindings like `dd`, `<leader>s`)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeySequence {
    pub keys: Vec<KeyCombo>,
}

/// A single key combination (key + modifiers)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    pub key: Key,
    pub modifiers: Modifiers,
}

impl KeyCombo {
    pub fn new(key: Key, modifiers: Modifiers) -> Self {
        Self { key, modifiers }
    }

    pub fn plain(key: Key) -> Self {
        Self { key, modifiers: Modifiers::NONE }
    }

    pub fn with_ctrl(key: Key) -> Self {
        Self { key, modifiers: Modifiers { ctrl: true, ..Modifiers::NONE } }
    }

    pub fn with_shift(key: Key) -> Self {
        Self { key, modifiers: Modifiers { shift: true, ..Modifiers::NONE } }
    }

    pub fn with_alt(key: Key) -> Self {
        Self { key, modifiers: Modifiers { alt: true, ..Modifiers::NONE } }
    }

    pub fn matches(&self, event: &KeyEvent) -> bool {
        self.key == event.key && self.modifiers == event.modifiers
    }
}

impl KeySequence {
    pub fn single(combo: KeyCombo) -> Self {
        Self { keys: vec![combo] }
    }

    pub fn chord(combos: Vec<KeyCombo>) -> Self {
        Self { keys: combos }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn starts_with(&self, combo: &KeyCombo) -> bool {
        self.keys.first() == Some(combo)
    }

    pub fn tail(&self) -> Self {
        Self { keys: self.keys[1..].to_vec() }
    }
}

/// Parses key strings into KeyCombo or KeySequence
pub struct KeyParser;

impl KeyParser {
    pub fn parse_combo(s: &str) -> Result<KeyCombo, ParseError> {
        let s = s.trim().to_lowercase();
        let mut modifiers = Modifiers::NONE;
        let mut key_str = s.as_str();

        loop {
            if key_str.starts_with("ctrl+") {
                modifiers.ctrl = true;
                key_str = &key_str[5..];
            } else if key_str.starts_with("alt+") {
                modifiers.alt = true;
                key_str = &key_str[4..];
            } else if key_str.starts_with("shift+") {
                modifiers.shift = true;
                key_str = &key_str[6..];
            } else if key_str.starts_with("meta+") {
                modifiers.meta = true;
                key_str = &key_str[5..];
            } else {
                break;
            }
        }

        let key = Self::parse_key(key_str)?;
        Ok(KeyCombo::new(key, modifiers))
    }

    pub fn parse_sequence(s: &str) -> Result<KeySequence, ParseError> {
        let s = s.trim();

        if s.len() == 2 && !s.contains('+') && !s.contains('<') {
            let chars: Vec<char> = s.chars().collect();
            if chars[0] == chars[1] {
                let combo = KeyCombo::plain(Key::Character(chars[0]));
                return Ok(KeySequence::chord(vec![combo.clone(), combo]));
            }
        }

        if !s.contains(',') {
            let combo = Self::parse_combo(s)?;
            return Ok(KeySequence::single(combo));
        }

        let combos: Result<Vec<_>, _> = s.split(',').map(Self::parse_combo).collect();
        Ok(KeySequence::chord(combos?))
    }

    fn parse_key(s: &str) -> Result<Key, ParseError> {
        match s {
            "enter" | "return" | "cr" => Ok(Key::Enter),
            "escape" | "esc" => Ok(Key::Escape),
            "backspace" | "bs" => Ok(Key::Backspace),
            "delete" | "del" => Ok(Key::Delete),
            "tab" => Ok(Key::Tab),
            "space" | "sp" => Ok(Key::Space),
            "up" | "arrow_up" => Ok(Key::ArrowUp),
            "down" | "arrow_down" => Ok(Key::ArrowDown),
            "left" | "arrow_left" => Ok(Key::ArrowLeft),
            "right" | "arrow_right" => Ok(Key::ArrowRight),
            "home" => Ok(Key::Home),
            "end" => Ok(Key::End),
            "page_up" | "pgup" => Ok(Key::PageUp),
            "page_down" | "pgdn" => Ok(Key::PageDown),
            s if s.starts_with('f') && s.len() <= 3 => {
                let num: u8 = s[1..].parse().map_err(|_| ParseError::InvalidKey(s.to_string()))?;
                Ok(Key::F(num))
            }
            s if s.len() == 1 => Ok(Key::Character(s.chars().next().unwrap())),
            _ => Err(ParseError::InvalidKey(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidKey(String),
    InvalidModifier(String),
    InvalidSequence(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidKey(s) => write!(f, "Invalid key: {}", s),
            ParseError::InvalidModifier(s) => write!(f, "Invalid modifier: {}", s),
            ParseError::InvalidSequence(s) => write!(f, "Invalid sequence: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}

/// A single key binding mapping a key sequence to a command
#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub id: String,
    pub sequence: KeySequence,
    pub command: String,
    pub description: Option<String>,
    pub condition: Option<BindingCondition>,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub enum BindingCondition {
    Mode(String),
    FocusScope(String),
    Custom(String),
}

impl KeyBinding {
    pub fn new(id: impl Into<String>, key_str: &str, description: impl Into<String>) -> Self {
        let id_str = id.into();
        let sequence = KeyParser::parse_sequence(key_str).expect("Invalid key sequence");
        Self {
            command: id_str.clone(),
            id: id_str,
            sequence,
            description: Some(description.into()),
            condition: None,
            enabled: true,
        }
    }

    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.command = command.into();
        self
    }

    pub fn in_mode(mut self, mode: impl Into<String>) -> Self {
        self.condition = Some(BindingCondition::Mode(mode.into()));
        self
    }

    pub fn in_focus_scope(mut self, scope: impl Into<String>) -> Self {
        self.condition = Some(BindingCondition::FocusScope(scope.into()));
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    pub fn matches(&self, event: &KeyEvent, current_mode: Option<&str>) -> bool {
        if !self.enabled {
            return false;
        }

        if let Some(ref condition) = self.condition {
            match condition {
                BindingCondition::Mode(mode) => {
                    if current_mode.is_none_or(|m| m != mode) {
                        return false;
                    }
                }
                BindingCondition::FocusScope(_) => {}
                BindingCondition::Custom(_) => {}
            }
        }

        if !self.sequence.is_empty() {
            return self.sequence.keys[0].matches(event);
        }

        false
    }
}

/// A collection of bindings with priority and optional conditions
#[derive(Debug, Clone)]
pub struct KeyLayer {
    pub name: String,
    pub priority: i32,
    pub enabled: bool,
    bindings: Vec<KeyBinding>,
}

impl KeyLayer {
    pub fn new(name: impl Into<String>, priority: i32) -> Self {
        Self { name: name.into(), priority, enabled: true, bindings: Vec::new() }
    }

    pub fn add_binding(&mut self, binding: KeyBinding) {
        self.bindings.push(binding);
    }

    pub fn remove_binding(&mut self, id: &str) -> bool {
        let len = self.bindings.len();
        self.bindings.retain(|b| b.id != id);
        self.bindings.len() < len
    }

    pub fn bindings(&self) -> &[KeyBinding] {
        &self.bindings
    }

    pub fn find_binding(&self, event: &KeyEvent, current_mode: Option<&str>) -> Option<&KeyBinding> {
        if !self.enabled {
            return None;
        }
        self.bindings.iter().find(|b| b.matches(event, current_mode))
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

/// The main keymap manager that resolves bindings across layers
#[derive(Debug)]
pub struct Keymap {
    layers: Vec<KeyLayer>,
    current_mode: Option<String>,
    pending_sequence: Vec<KeyCombo>,
    chord_timeout_ms: u64,
    command_history: Vec<String>,
}

impl Default for Keymap {
    fn default() -> Self {
        Self::new()
    }
}

impl Keymap {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            current_mode: None,
            pending_sequence: Vec::new(),
            chord_timeout_ms: 1000,
            command_history: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: KeyLayer) {
        self.layers.push(layer);
        self.layers.sort_by_key(|b| std::cmp::Reverse(b.priority));
    }

    pub fn remove_layer(&mut self, name: &str) -> bool {
        let len = self.layers.len();
        self.layers.retain(|l| l.name != name);
        self.layers.len() < len
    }

    pub fn get_layer(&self, name: &str) -> Option<&KeyLayer> {
        self.layers.iter().find(|l| l.name == name)
    }

    pub fn get_layer_mut(&mut self, name: &str) -> Option<&mut KeyLayer> {
        self.layers.iter_mut().find(|l| l.name == name)
    }

    pub fn add_binding_to_layer(&mut self, layer_name: &str, binding: KeyBinding, priority: i32) {
        if let Some(layer) = self.get_layer_mut(layer_name) {
            layer.add_binding(binding);
        } else {
            let mut layer = KeyLayer::new(layer_name, priority);
            layer.add_binding(binding);
            self.add_layer(layer);
        }
    }

    pub fn add_binding(&mut self, binding: KeyBinding) {
        self.add_binding_to_layer("default", binding, 0);
    }

    pub fn set_mode(&mut self, mode: impl Into<String>) {
        self.current_mode = Some(mode.into());
        self.pending_sequence.clear();
    }

    pub fn current_mode(&self) -> Option<&str> {
        self.current_mode.as_deref()
    }

    pub fn clear_mode(&mut self) {
        self.current_mode = None;
        self.pending_sequence.clear();
    }

    pub fn set_chord_timeout(&mut self, ms: u64) {
        self.chord_timeout_ms = ms;
    }

    pub fn chord_timeout_ms(&self) -> u64 {
        self.chord_timeout_ms
    }

    pub fn handle_event(&mut self, event: &KeyEvent) -> Option<String> {
        if !self.pending_sequence.is_empty() {
            let expected = &self.pending_sequence[0];
            if expected.matches(event) {
                self.pending_sequence.remove(0);
                if self.pending_sequence.is_empty() {
                    return self.last_binding_command();
                }
                return None;
            } else {
                self.pending_sequence.clear();
            }
        }

        for layer in &self.layers {
            if let Some(binding) = layer.find_binding(event, self.current_mode.as_deref()) {
                if binding.sequence.len() == 1 {
                    self.command_history.push(binding.command.clone());
                    return Some(binding.command.clone());
                } else {
                    self.pending_sequence = binding.sequence.keys[1..].to_vec();
                    self.command_history.push(binding.command.clone());
                    return None;
                }
            }
        }

        None
    }

    pub fn has_pending_sequence(&self) -> bool {
        !self.pending_sequence.is_empty()
    }

    pub fn clear_pending_sequence(&mut self) {
        self.pending_sequence.clear();
    }

    pub fn pending_keys(&self) -> &[KeyCombo] {
        &self.pending_sequence
    }

    pub fn command_history(&self) -> &[String] {
        &self.command_history
    }

    pub fn clear_history(&mut self) {
        self.command_history.clear();
    }

    pub fn active_bindings(&self) -> Vec<(&KeyBinding, &str)> {
        let mut result = Vec::new();
        for layer in &self.layers {
            if !layer.enabled {
                continue;
            }
            for binding in layer.bindings() {
                if binding.enabled {
                    result.push((binding, layer.name.as_str()));
                }
            }
        }
        result
    }

    pub fn all_bindings(&self) -> Vec<(&KeyBinding, &str)> {
        self.layers.iter().flat_map(|layer| layer.bindings().iter().map(move |b| (b, layer.name.as_str()))).collect()
    }

    fn last_binding_command(&self) -> Option<String> {
        self.command_history.last().cloned()
    }
}

/// Builder for creating keymaps with a fluent API
pub struct KeymapBuilder {
    keymap: Keymap,
}

impl KeymapBuilder {
    pub fn new() -> Self {
        Self { keymap: Keymap::new() }
    }

    pub fn binding(mut self, id: &str, keys: &str, desc: &str) -> Self {
        self.keymap.add_binding(KeyBinding::new(id, keys, desc));
        self
    }

    pub fn binding_in_layer(mut self, layer: &str, priority: i32, id: &str, keys: &str, desc: &str) -> Self {
        self.keymap.add_binding_to_layer(layer, KeyBinding::new(id, keys, desc), priority);
        self
    }

    pub fn mode(mut self, mode: &str) -> Self {
        self.keymap.set_mode(mode);
        self
    }

    pub fn build(self) -> Keymap {
        self.keymap
    }
}

impl Default for KeymapBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod clipboard {
        use super::*;

        #[test]
        fn new_default() {
            let c = ClipboardInput::default();
            assert!(c.data.is_empty());
            assert_eq!(c.action, ClipboardAction::Copy);
        }

        #[test]
        fn copy_constructor() {
            let c = ClipboardInput::copy("hello".to_string());
            assert_eq!(c.data, "hello");
            assert!(c.is_copy());
            assert!(!c.is_paste());
            assert!(!c.is_cut());
        }

        #[test]
        fn paste_constructor() {
            let c = ClipboardInput::paste("world".to_string());
            assert_eq!(c.data, "world");
            assert!(c.is_paste());
        }

        #[test]
        fn cut_constructor() {
            let c = ClipboardInput::cut("data".to_string());
            assert_eq!(c.data, "data");
            assert!(c.is_cut());
        }

        #[test]
        fn action_ordering() {
            assert_ne!(ClipboardAction::Copy, ClipboardAction::Paste);
            assert_ne!(ClipboardAction::Copy, ClipboardAction::Cut);
        }
    }

    mod keyboard {
        use super::*;

        #[test]
        fn new_default() {
            let k = KeyboardInput::default();
            assert_eq!(k.key, '\0');
            assert!(k.modifiers.is_empty());
            assert_eq!(k.action, KeyAction::Press);
        }

        #[test]
        fn new_with_modifiers() {
            let k = KeyboardInput::new('a', KeyModifiers::CONTROL);
            assert!(k.is_ctrl());
            assert!(!k.is_shift());
        }

        #[test]
        fn modifier_flags() {
            let ctrl_alt = KeyModifiers::CONTROL | KeyModifiers::ALT;
            let k = KeyboardInput::new('x', ctrl_alt);
            assert!(k.is_ctrl());
            assert!(k.is_alt());
            assert!(!k.is_shift());
            assert!(!k.is_super());
        }

        #[test]
        fn action_variants() {
            let p = KeyboardInput::press('a', KeyModifiers::empty());
            assert_eq!(p.action, KeyAction::Press);

            let r = KeyboardInput::release('a', KeyModifiers::empty());
            assert_eq!(r.action, KeyAction::Release);

            let rr = KeyboardInput::repeat('a', KeyModifiers::empty());
            assert_eq!(rr.action, KeyAction::Repeat);
        }

        #[test]
        fn with_action_overrides() {
            let k = KeyboardInput::new('a', KeyModifiers::empty()).with_action(KeyAction::Repeat);
            assert_eq!(k.action, KeyAction::Repeat);
        }

        #[test]
        fn modifier_only_keys() {
            let null = KeyboardInput::new('\0', KeyModifiers::empty());
            assert!(null.is_modifier_only());

            let esc = KeyboardInput::new('\x1b', KeyModifiers::empty());
            assert!(esc.is_modifier_only());

            let normal = KeyboardInput::new('a', KeyModifiers::empty());
            assert!(!normal.is_modifier_only());
        }

        #[test]
        fn display_string_ctrl() {
            let k = KeyboardInput::new('c', KeyModifiers::CONTROL);
            assert_eq!(k.to_display_string(), "Ctrl+c");
        }

        #[test]
        fn display_string_ctrl_shift() {
            let k = KeyboardInput::new('C', KeyModifiers::CONTROL | KeyModifiers::SHIFT);
            assert_eq!(k.to_display_string(), "Ctrl+Shift+C");
        }

        #[test]
        fn display_string_special() {
            let esc = KeyboardInput::new('\x1b', KeyModifiers::empty());
            assert_eq!(esc.to_display_string(), "Esc");

            let space = KeyboardInput::new(' ', KeyModifiers::empty());
            assert_eq!(space.to_display_string(), "Space");
        }

        #[test]
        fn key_modifiers_bitflags() {
            let empty = KeyModifiers::empty();
            assert!(empty.is_empty());

            let all = KeyModifiers::all();
            assert!(all.contains(KeyModifiers::SHIFT));
            assert!(all.contains(KeyModifiers::CONTROL));
            assert!(all.contains(KeyModifiers::ALT));
            assert!(all.contains(KeyModifiers::SUPER));
        }
    }

    mod mouse {
        use super::*;

        #[test]
        fn new_default() {
            let m = MouseInput::default();
            assert_eq!(m.x, 0);
            assert_eq!(m.y, 0);
            assert!(m.buttons.is_empty());
        }

        #[test]
        fn new_with_position() {
            let m = MouseInput::new(10, 20, MouseButtons::LEFT);
            assert_eq!(m.x, 10);
            assert_eq!(m.y, 20);
            assert!(m.is_left_button());
        }

        #[test]
        fn button_checks() {
            let left = MouseInput::new(0, 0, MouseButtons::LEFT);
            assert!(left.is_left_button());
            assert!(!left.is_right_button());
            assert!(!left.is_middle_button());

            let right = MouseInput::new(0, 0, MouseButtons::RIGHT);
            assert!(right.is_right_button());

            let middle = MouseInput::new(0, 0, MouseButtons::MIDDLE);
            assert!(middle.is_middle_button());
        }

        #[test]
        fn with_modifiers() {
            let m = MouseInput::new(5, 5, MouseButtons::LEFT).with_modifiers(KeyModifiers::CONTROL);
            assert!(m.is_ctrl());
        }

        #[test]
        fn event_type_variants() {
            let press = MouseInput::press(1, 2, MouseButtons::LEFT);
            assert_eq!(press.event_type, MouseEventType::Press);

            let release = MouseInput::release(1, 2, MouseButtons::LEFT);
            assert_eq!(release.event_type, MouseEventType::Release);

            let move_ = MouseInput::move_to(3, 4);
            assert_eq!(move_.event_type, MouseEventType::Move);

            let scroll = MouseInput::scroll(5, 6, 0, -3);
            assert_eq!(scroll.event_type, MouseEventType::Scroll);
            assert_eq!(scroll.scroll_delta, Some((0, -3)));

            let drag = MouseInput::drag(7, 8, MouseButtons::LEFT);
            assert_eq!(drag.event_type, MouseEventType::Drag);

            let drop = MouseInput::drop(9, 10, MouseButtons::LEFT);
            assert_eq!(drop.event_type, MouseEventType::Drop);
        }

        #[test]
        fn mouse_buttons_bitflags() {
            let both = MouseButtons::LEFT | MouseButtons::RIGHT;
            assert!(both.contains(MouseButtons::LEFT));
            assert!(both.contains(MouseButtons::RIGHT));
            assert!(!both.contains(MouseButtons::MIDDLE));
        }
    }

    mod input_event {
        use super::*;

        #[test]
        fn keyboard_event() {
            let input = KeyboardInput::new('q', KeyModifiers::empty());
            let ev = InputEvent::keyboard(input);
            assert!(ev.is_keyboard());
            assert!(!ev.is_mouse());
            assert!(!ev.is_clipboard());
            assert!(!ev.is_resize());
            assert!(!ev.is_focus());
            assert!(!ev.is_blur());
            assert!(!ev.is_paste());
        }

        #[test]
        fn focus_blur_events() {
            let focus = InputEvent::focus();
            assert!(focus.is_focus());

            let blur = InputEvent::blur();
            assert!(blur.is_blur());
        }

        #[test]
        fn resize_event() {
            let r = InputEvent::resize(120, 40);
            assert!(r.is_resize());
            assert!(!r.is_focus());
        }

        #[test]
        fn paste_event() {
            let p = InputEvent::paste("hello".to_string());
            assert!(p.is_paste());
        }

        #[test]
        fn clipboard_event() {
            let c = InputEvent::clipboard(ClipboardInput::copy("data".to_string()));
            assert!(c.is_clipboard());
        }

        #[test]
        fn mouse_event() {
            let m = InputEvent::mouse(MouseInput::new(10, 20, MouseButtons::RIGHT));
            assert!(m.is_mouse());
        }

        #[test]
        fn default_is_focus() {
            assert!(InputEvent::default().is_focus());
        }

        #[test]
        fn timestamps_advance() {
            let e1 = InputEvent::focus();
            let e2 = InputEvent::focus();
            assert!(e2.timestamp >= e1.timestamp);
        }
    }

    mod key_code_enum {
        use super::*;

        #[test]
        fn variant_distinct() {
            assert_ne!(KeyCode::Enter, KeyCode::Esc);
            assert_ne!(KeyCode::Tab, KeyCode::Space);
        }

        #[test]
        fn f_keys() {
            let f1 = KeyCode::F(1);
            let f12 = KeyCode::F(12);
            assert_ne!(f1, f12);
        }

        #[test]
        fn media_keys() {
            let pp = MediaKey::PlayPause;
            let next = MediaKey::NextTrack;
            assert_ne!(pp, next);
        }
    }

    mod event_enum {
        use super::*;

        #[test]
        fn key_event_new() {
            let ke = KeyEvent::new(Key::Enter, NodeId::default());
            assert_eq!(ke.key, Key::Enter);
            assert!(ke.modifiers.is_empty());
            assert_eq!(ke.phase, EventPhase::Target);
        }

        #[test]
        fn key_event_with_modifiers() {
            let ke = KeyEvent::new(Key::Character('c'), NodeId::default())
                .with_modifiers(Modifiers { ctrl: true, ..Default::default() });
            assert_eq!(ke.key, Key::Character('c'));
            assert!(ke.modifiers.ctrl);
        }

        #[test]
        fn key_event_prevent_default() {
            let mut ke = KeyEvent::new(Key::Escape, NodeId::default());
            assert!(!ke.default_prevented);
            ke.prevent_default();
            assert!(ke.default_prevented);
        }

        #[test]
        fn mouse_event_new() {
            let pt = Point::new(5, 10);
            let me = MouseEvent::new(MouseButton::Left, pt, NodeId::default());
            assert_eq!(me.button, MouseButton::Left);
            assert_eq!(me.position, pt);
        }

        #[test]
        fn paste_event_new() {
            let pe = PasteEvent::new("content", NodeId::default());
            assert_eq!(&*pe.text, "content");
        }

        #[test]
        fn event_types() {
            let ke = KeyEvent::new(Key::Enter, NodeId::default());
            assert!(matches!(Event::Key(ke), Event::Key(_)));

            let me = MouseEvent::new(MouseButton::Left, Point::new(0, 0), NodeId::default());
            assert!(matches!(Event::Mouse(me), Event::Mouse(_)));
        }

        #[test]
        fn event_phase_default() {
            let ke = KeyEvent::new(Key::Enter, NodeId::default());
            assert_eq!(ke.phase, EventPhase::Target);
        }

        #[test]
        fn event_phase_set_get() {
            let ke = KeyEvent::new(Key::Enter, NodeId::default());
            let event = Event::Key(ke);
            assert_eq!(event.phase(), EventPhase::Target);

            let mut event = event;
            event.set_phase(EventPhase::Capture);
            assert_eq!(event.phase(), EventPhase::Capture);
        }

        #[test]
        fn event_phase_resize() {
            let event = Event::Resize(ResizeEvent::new(80, 24, 80, 24));
            assert_eq!(event.phase(), EventPhase::Target);
            let mut event = event;
            event.set_phase(EventPhase::Bubble);
            assert_eq!(event.phase(), EventPhase::Target);
        }

        #[test]
        fn event_target_returns_some() {
            let target = NodeId::default();
            let ke = KeyEvent::new(Key::Enter, target);
            let event = Event::Key(ke);
            assert!(event.target().is_some());
        }

        #[test]
        fn event_target_resize_is_none() {
            let event = Event::Resize(ResizeEvent::new(80, 24, 80, 24));
            assert!(event.target().is_none());
        }

        #[test]
        fn event_is_consumed_false_by_default() {
            let ke = KeyEvent::new(Key::Enter, NodeId::default());
            let event = Event::Key(ke);
            assert!(!event.is_consumed());
        }

        #[test]
        fn event_modifiers() {
            let mut ke = KeyEvent::new(Key::Tab, NodeId::default());
            ke.modifiers.shift = true;
            assert!(ke.modifiers.shift);
            assert!(!ke.modifiers.ctrl);
        }
    }

    mod modifiers {
        use super::*;

        #[test]
        fn new_empty() {
            let m = Modifiers::default();
            assert!(!m.ctrl);
            assert!(!m.shift);
            assert!(!m.alt);
            assert!(!m.meta);
        }

        #[test]
        fn is_empty_true() {
            let m = Modifiers::default();
            assert!(m.is_empty());
        }

        #[test]
        fn is_empty_false() {
            let m = Modifiers { ctrl: true, ..Default::default() };
            assert!(!m.is_empty());
        }
    }

    mod mouse_button_enum {
        use super::*;

        #[test]
        fn variants() {
            assert_ne!(MouseButton::Left, MouseButton::Right);
            assert_ne!(MouseButton::Left, MouseButton::Middle);
        }
    }

    mod event_phase_enum {
        use super::*;

        #[test]
        fn order() {
            assert_ne!(EventPhase::Capture, EventPhase::Target);
            assert_ne!(EventPhase::Target, EventPhase::Bubble);
        }
    }
}
