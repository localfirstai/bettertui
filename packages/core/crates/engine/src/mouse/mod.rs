//! Mouse handler for processing mouse events and hit-testing.
//!
//! Translates raw mouse input into widget-aware interactions,
//! manages hover state, and supports double-click detection.

use crate::events::types::MouseButton;
use crate::tree::NodeId;

/// The type of mouse interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseAction {
    /// A mouse button was pressed.
    Press,
    /// A mouse button was released.
    Release,
    /// The mouse was moved.
    Move,
    /// A scroll event occurred.
    Scroll,
    /// A double-click was detected.
    DoubleClick,
}

/// A processed mouse event with widget context.
#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// The type of mouse action.
    pub action: MouseAction,
    /// Which button was involved.
    pub button: MouseButton,
    /// X position in terminal cells.
    pub x: u16,
    /// Y position in terminal cells.
    pub y: u16,
    /// The widget node under the cursor, if any.
    pub target: Option<NodeId>,
    /// Whether Ctrl was held.
    pub ctrl: bool,
    /// Whether Shift was held.
    pub shift: bool,
    /// Whether Alt was held.
    pub alt: bool,
    /// Scroll delta (positive = down, negative = up).
    pub scroll_delta: i16,
}

impl MouseEvent {
    /// Creates a new mouse event.
    pub fn new(action: MouseAction, button: MouseButton, x: u16, y: u16) -> Self {
        Self {
            action,
            button,
            x,
            y,
            target: None,
            ctrl: false,
            shift: false,
            alt: false,
            scroll_delta: 0,
        }
    }

    /// Sets the target node.
    pub fn with_target(mut self, target: NodeId) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets modifier state.
    pub fn with_modifiers(mut self, ctrl: bool, shift: bool, alt: bool) -> Self {
        self.ctrl = ctrl;
        self.shift = shift;
        self.alt = alt;
        self
    }
}

/// Tracks mouse state and detects interactions like double-clicks.
#[derive(Debug)]
pub struct MouseHandler {
    /// Last mouse position.
    last_x: u16,
    last_y: u16,
    /// Last click position and time for double-click detection.
    last_click_x: u16,
    last_click_y: u16,
    last_click_time: u64,
    /// Current button state (which buttons are held).
    held_buttons: u8,
    /// Double-click threshold in milliseconds.
    double_click_threshold_ms: u64,
    /// Whether mouse tracking is enabled.
    tracking_enabled: bool,
    /// Whether button-event tracking is enabled (reports button on release).
    button_tracking: bool,
    /// Whether SGR mouse mode is enabled (reports coordinates > 223).
    sgr_mode: bool,
}

impl Default for MouseHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseHandler {
    /// Creates a new MouseHandler.
    pub fn new() -> Self {
        Self {
            last_x: 0,
            last_y: 0,
            last_click_x: 0,
            last_click_y: 0,
            last_click_time: 0,
            held_buttons: 0,
            double_click_threshold_ms: 500,
            tracking_enabled: false,
            button_tracking: false,
            sgr_mode: false,
        }
    }

    /// Sets the double-click threshold.
    pub fn with_double_click_threshold(mut self, ms: u64) -> Self {
        self.double_click_threshold_ms = ms;
        self
    }

    /// Enables or disables mouse tracking.
    pub fn set_tracking(&mut self, enabled: bool) {
        self.tracking_enabled = enabled;
    }

    /// Returns whether mouse tracking is enabled.
    pub fn is_tracking(&self) -> bool {
        self.tracking_enabled
    }

    /// Enables or disables button-event tracking.
    pub fn set_button_tracking(&mut self, enabled: bool) {
        self.button_tracking = enabled;
    }

    /// Enables or disables SGR mouse mode.
    pub fn set_sgr_mode(&mut self, enabled: bool) {
        self.sgr_mode = enabled;
    }

    /// Returns whether SGR mouse mode is enabled.
    pub fn sgr_mode(&self) -> bool {
        self.sgr_mode
    }

    /// Processes a mouse press event.
    pub fn press(&mut self, button: MouseButton, x: u16, y: u16, time_ms: u64) -> MouseEvent {
        self.last_x = x;
        self.last_y = y;

        let button_bit = mouse_button_bit(button);
        self.held_buttons |= button_bit;

        let action = if self.is_double_click(x, y, time_ms) {
            MouseAction::DoubleClick
        } else {
            MouseAction::Press
        };

        if action == MouseAction::DoubleClick || action == MouseAction::Press {
            self.last_click_x = x;
            self.last_click_y = y;
            self.last_click_time = time_ms;
        }

        MouseEvent::new(action, button, x, y)
    }

    /// Processes a mouse release event.
    pub fn release(&mut self, button: MouseButton, x: u16, y: u16) -> MouseEvent {
        self.last_x = x;
        self.last_y = y;

        let button_bit = mouse_button_bit(button);
        self.held_buttons &= !button_bit;

        MouseEvent::new(MouseAction::Release, button, x, y)
    }

    /// Processes a mouse move event.
    pub fn move_to(&mut self, x: u16, y: u16) -> MouseEvent {
        self.last_x = x;
        self.last_y = y;

        let button = if self.held_buttons & mouse_button_bit(MouseButton::Left) != 0 {
            MouseButton::Left
        } else if self.held_buttons & mouse_button_bit(MouseButton::Right) != 0 {
            MouseButton::Right
        } else if self.held_buttons & mouse_button_bit(MouseButton::Middle) != 0 {
            MouseButton::Middle
        } else {
            MouseButton::None
        };

        MouseEvent::new(MouseAction::Move, button, x, y)
    }

    /// Processes a scroll event.
    pub fn scroll(&mut self, delta: i16, x: u16, y: u16) -> MouseEvent {
        self.last_x = x;
        self.last_y = y;

        let mut event = MouseEvent::new(MouseAction::Scroll, MouseButton::None, x, y);
        event.scroll_delta = delta;
        event
    }

    /// Returns the last known cursor position.
    pub fn last_position(&self) -> (u16, u16) {
        (self.last_x, self.last_y)
    }

    /// Returns which buttons are currently held.
    pub fn held_buttons(&self) -> u8 {
        self.held_buttons
    }

    /// Returns whether any button is currently held.
    pub fn is_dragging(&self) -> bool {
        self.held_buttons != 0
    }

    fn is_double_click(&self, x: u16, y: u16, time_ms: u64) -> bool {
        let same_pos = x == self.last_click_x && y == self.last_click_y;
        let within_threshold =
            time_ms.saturating_sub(self.last_click_time) <= self.double_click_threshold_ms;
        same_pos && within_threshold
    }
}

fn mouse_button_bit(button: MouseButton) -> u8 {
    match button {
        MouseButton::Left => 1,
        MouseButton::Right => 2,
        MouseButton::Middle => 4,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn press_and_release() {
        let mut handler = MouseHandler::new();
        let press = handler.press(MouseButton::Left, 10, 5, 0);
        assert_eq!(press.action, MouseAction::Press);
        assert_eq!(press.x, 10);
        assert_eq!(press.y, 5);

        let release = handler.release(MouseButton::Left, 10, 5);
        assert_eq!(release.action, MouseAction::Release);
    }

    #[test]
    fn move_tracking() {
        let mut handler = MouseHandler::new();
        handler.press(MouseButton::Left, 0, 0, 0);
        let mv = handler.move_to(5, 3);
        assert_eq!(mv.action, MouseAction::Move);
        assert_eq!(mv.button, MouseButton::Left);
    }

    #[test]
    fn scroll() {
        let mut handler = MouseHandler::new();
        let scroll = handler.scroll(-1, 10, 10);
        assert_eq!(scroll.action, MouseAction::Scroll);
        assert_eq!(scroll.scroll_delta, -1);
    }

    #[test]
    fn double_click() {
        let mut handler = MouseHandler::new();
        handler.press(MouseButton::Left, 5, 5, 0);
        let dc = handler.press(MouseButton::Left, 5, 5, 100);
        assert_eq!(dc.action, MouseAction::DoubleClick);
    }

    #[test]
    fn double_click_different_position() {
        let mut handler = MouseHandler::new();
        handler.press(MouseButton::Left, 5, 5, 0);
        let press = handler.press(MouseButton::Left, 6, 5, 100);
        assert_eq!(press.action, MouseAction::Press);
    }

    #[test]
    fn double_click_timeout() {
        let mut handler = MouseHandler::new();
        handler.press(MouseButton::Left, 5, 5, 0);
        let press = handler.press(MouseButton::Left, 5, 5, 1000);
        assert_eq!(press.action, MouseAction::Press);
    }

    #[test]
    fn drag_detection() {
        let mut handler = MouseHandler::new();
        assert!(!handler.is_dragging());
        handler.press(MouseButton::Left, 0, 0, 0);
        assert!(handler.is_dragging());
        handler.release(MouseButton::Left, 0, 0);
        assert!(!handler.is_dragging());
    }

    #[test]
    fn tracking_toggle() {
        let mut handler = MouseHandler::new();
        assert!(!handler.is_tracking());
        handler.set_tracking(true);
        assert!(handler.is_tracking());
    }

    #[test]
    fn sgr_mode() {
        let mut handler = MouseHandler::new();
        assert!(!handler.sgr_mode());
        handler.set_sgr_mode(true);
        assert!(handler.sgr_mode());
    }

    #[test]
    fn event_with_target() {
        let mut arena = crate::tree::NodeArena::new();
        let id = arena.insert(crate::tree::RenderNode::new(crate::tree::NodeKind::Box));
        let event = MouseEvent::new(MouseAction::Press, MouseButton::Left, 0, 0).with_target(id);
        assert_eq!(event.target, Some(id));
    }

    #[test]
    fn last_position() {
        let mut handler = MouseHandler::new();
        handler.press(MouseButton::Left, 15, 25, 0);
        assert_eq!(handler.last_position(), (15, 25));
    }
}
