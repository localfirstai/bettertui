//! Unified event pipeline: raw bytes → AnsiParser → VtMachine → EventBus.
//!
//! This module provides a single entry point for terminal input processing,
//! wiring together the ANSI parser, VT state machine, and event bus.

use std::collections::VecDeque;

use crate::ansi::{AnsiParser, ParserEvent};
use crate::input::{Event, EventBus, Key, KeyEvent, Modifiers, MouseButton};
use crate::terminal::{KittyKeyEvent, VtMachine};
use crate::tree::{NodeId, Point};

/// Configuration for the event pipeline.
#[derive(Debug, Clone)]
pub struct EventPipelineConfig {
    /// Enable Kitty keyboard protocol parsing.
    pub kitty_keyboard: bool,
    /// Enable bracketed paste mode detection.
    pub bracketed_paste: bool,
    /// Enable focus event detection.
    pub focus_events: bool,
    /// Enable mouse event parsing.
    pub mouse_tracking: bool,
    /// Terminal width for VtMachine.
    pub width: u16,
    /// Terminal height for VtMachine.
    pub height: u16,
}

impl Default for EventPipelineConfig {
    fn default() -> Self {
        Self {
            kitty_keyboard: false,
            bracketed_paste: false,
            focus_events: false,
            mouse_tracking: false,
            width: 80,
            height: 24,
        }
    }
}

/// The unified event pipeline.
///
/// Processes raw terminal input through:
/// 1. `AnsiParser` - parses escape sequences
/// 2. `VtMachine` - maintains terminal state
/// 3. `EventBus` - queues events for dispatch
#[derive(Debug)]
pub struct EventPipeline {
    parser: AnsiParser,
    vt: VtMachine,
    bus: EventBus,
    config: EventPipelineConfig,
    pending_text: Vec<u8>,
}

impl Default for EventPipeline {
    fn default() -> Self {
        Self::new(EventPipelineConfig::default())
    }
}

impl EventPipeline {
    pub fn new(config: EventPipelineConfig) -> Self {
        Self {
            parser: AnsiParser::new(),
            vt: VtMachine::new(config.width, config.height),
            bus: EventBus::new(),
            config,
            pending_text: Vec::new(),
        }
    }

    /// Feed raw bytes into the pipeline.
    pub fn feed(&mut self, data: &[u8]) {
        self.parser.feed(data);
        self.process_parser_events();
    }

    /// Process all pending parser events.
    fn process_parser_events(&mut self) {
        while let Some(event) = self.parser.poll_event() {
            self.handle_parser_event(event);
        }
    }

    /// Handle a single parser event.
    fn handle_parser_event(&mut self, event: ParserEvent) {
        match &event {
            ParserEvent::Char(ch) => {
                self.pending_text.push(*ch);
            }
            ParserEvent::LineFeed | ParserEvent::CarriageReturn | ParserEvent::Tab | ParserEvent::Backspace => {
                self.flush_pending_text();
                self.vt.process(&event);
            }
            ParserEvent::Csi(cmd) => {
                self.flush_pending_text();
                self.vt.process(&event);
                self.handle_csi_event(cmd, &event);
            }
            ParserEvent::Osc(_) => {
                self.flush_pending_text();
                self.vt.process(&event);
            }
            _ => {
                self.flush_pending_text();
                self.vt.process(&event);
            }
        }
    }

    /// Flush pending text as a key event or paste.
    fn flush_pending_text(&mut self) {
        if self.pending_text.is_empty() {
            return;
        }

        let text = String::from_utf8_lossy(&self.pending_text);
        let text_str = text.as_ref();

        if self.config.bracketed_paste && text_str.len() > 1 {
            self.bus.push_paste(text_str, NodeId::default());
        } else {
            for ch in text_str.chars() {
                if !ch.is_control() {
                    let key = Key::Character(ch);
                    self.bus.push_key(key, Modifiers::NONE, NodeId::default());
                }
            }
        }

        self.pending_text.clear();
    }

    /// Handle CSI events, converting to EventBus events.
    fn handle_csi_event(&mut self, _cmd: &crate::ansi::CsiCommand, _event: &ParserEvent) {
        if let Some(kitty_key) = self.vt.last_kitty_key() {
            let key_event = self.kitty_key_to_event(kitty_key);
            self.bus.push(key_event);
            self.vt.last_kitty_key_mut().take();
        }
    }

    /// Convert a KittyKeyEvent to an Event.
    fn kitty_key_to_event(&self, kitty: &KittyKeyEvent) -> Event {
        let key = self.keycode_to_key(kitty.keycode);
        let modifiers = self.modifiers_from_kitty(kitty.modifiers);
        Event::Key(KeyEvent::new(key, NodeId::default()).with_modifiers(modifiers))
    }

    /// Convert a Kitty keycode to a Key.
    fn keycode_to_key(&self, keycode: u32) -> Key {
        match keycode {
            0x0d => Key::Enter,
            0x1b => Key::Escape,
            0x08 => Key::Backspace,
            0x7f => Key::Delete,
            0x09 => Key::Tab,
            0x20 => Key::Space,
            0x4001 => Key::ArrowUp,
            0x4002 => Key::ArrowDown,
            0x4003 => Key::ArrowLeft,
            0x4004 => Key::ArrowRight,
            0x4005 => Key::Home,
            0x4006 => Key::End,
            0x4007 => Key::PageUp,
            0x4008 => Key::PageDown,
            code if (0x400a..=0x400d).contains(&code) => Key::F((code - 0x400a + 1) as u8),
            code => char::from_u32(code).map(Key::Character).unwrap_or(Key::Character('\0')),
        }
    }

    /// Convert Kitty modifiers to our Modifiers struct.
    fn modifiers_from_kitty(&self, mods: u32) -> Modifiers {
        Modifiers { shift: mods & 1 != 0, alt: mods & 2 != 0, ctrl: mods & 4 != 0, meta: mods & 8 != 0 }
    }

    /// Push a raw key event directly.
    pub fn push_key(&mut self, key: Key, modifiers: Modifiers, target: NodeId) {
        self.bus.push_key(key, modifiers, target);
    }

    /// Push a raw mouse event directly.
    pub fn push_mouse(&mut self, button: MouseButton, position: Point, target: NodeId) {
        self.bus.push_mouse(button, position, target);
    }

    /// Push a paste event directly.
    pub fn push_paste(&mut self, text: impl Into<std::sync::Arc<str>>, target: NodeId) {
        self.bus.push_paste(text, target);
    }

    /// Push a resize event.
    pub fn push_resize(&mut self, width: u16, height: u16, prev_width: u16, prev_height: u16) {
        self.bus.push_resize(width, height, prev_width, prev_height);
    }

    /// Drain all events from the bus.
    pub fn drain(&mut self) -> VecDeque<Event> {
        self.flush_pending_text();
        self.bus.drain()
    }

    /// Get the number of pending events.
    pub fn len(&self) -> usize {
        self.bus.len()
    }

    /// Check if there are no pending events.
    pub fn is_empty(&self) -> bool {
        self.bus.is_empty()
    }

    /// Clear all pending events.
    pub fn clear(&mut self) {
        self.pending_text.clear();
        self.bus.clear();
    }

    /// Resize the terminal.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.config.width = width;
        self.config.height = height;
        self.vt.resize(width, height);
    }

    /// Get the underlying VtMachine.
    pub fn vt(&self) -> &VtMachine {
        &self.vt
    }

    /// Get mutable access to the VtMachine.
    pub fn vt_mut(&mut self) -> &mut VtMachine {
        &mut self.vt
    }

    /// Get the underlying EventBus.
    pub fn bus(&self) -> &EventBus {
        &self.bus
    }

    /// Get mutable access to the EventBus.
    pub fn bus_mut(&mut self) -> &mut EventBus {
        &mut self.bus
    }

    /// Get the underlying AnsiParser.
    pub fn parser(&self) -> &AnsiParser {
        &self.parser
    }

    /// Get mutable access to the AnsiParser.
    pub fn parser_mut(&mut self) -> &mut AnsiParser {
        &mut self.parser
    }

    /// Reset the pipeline state.
    pub fn reset(&mut self) {
        self.parser.reset();
        self.bus.clear();
        self.pending_text.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_new() {
        let pipeline = EventPipeline::default();
        assert!(pipeline.is_empty());
        assert_eq!(pipeline.len(), 0);
    }

    #[test]
    fn pipeline_feed_plain_text() {
        let mut pipeline = EventPipeline::new(EventPipelineConfig::default());
        pipeline.feed(b"hello");

        let events = pipeline.drain();
        assert_eq!(events.len(), 5);

        for (i, event) in events.iter().enumerate() {
            if let Event::Key(ke) = event {
                assert!(matches!(ke.key, Key::Character(c) if c == "hello".chars().nth(i).unwrap()));
            } else {
                panic!("Expected key event");
            }
        }
    }

    #[test]
    fn pipeline_feed_single_char() {
        let mut pipeline = EventPipeline::new(EventPipelineConfig::default());
        pipeline.feed(b"a");

        let events = pipeline.drain();
        assert_eq!(events.len(), 1);

        if let Event::Key(ke) = &events[0] {
            assert!(matches!(ke.key, Key::Character('a')));
        } else {
            panic!("Expected key event");
        }
    }

    #[test]
    fn pipeline_push_key() {
        let mut pipeline = EventPipeline::default();
        pipeline.push_key(Key::Enter, Modifiers::default(), NodeId::default());

        let events = pipeline.drain();
        assert_eq!(events.len(), 1);

        if let Event::Key(ke) = &events[0] {
            assert_eq!(ke.key, Key::Enter);
        } else {
            panic!("Expected key event");
        }
    }

    #[test]
    fn pipeline_push_mouse() {
        let mut pipeline = EventPipeline::default();
        pipeline.push_mouse(MouseButton::Left, Point::new(10, 5), NodeId::default());

        let events = pipeline.drain();
        assert_eq!(events.len(), 1);

        if let Event::Mouse(me) = &events[0] {
            assert_eq!(me.button, MouseButton::Left);
            assert_eq!(me.position.x, 10);
            assert_eq!(me.position.y, 5);
        } else {
            panic!("Expected mouse event");
        }
    }

    #[test]
    fn pipeline_resize() {
        let mut pipeline = EventPipeline::new(EventPipelineConfig { width: 80, height: 24, ..Default::default() });
        pipeline.resize(120, 40);

        assert_eq!(pipeline.vt().framebuffer().width(), 120);
        assert_eq!(pipeline.vt().framebuffer().height(), 40);
    }

    #[test]
    fn pipeline_clear() {
        let mut pipeline = EventPipeline::default();
        pipeline.push_key(Key::Enter, Modifiers::default(), NodeId::default());
        assert!(!pipeline.is_empty());

        pipeline.clear();
        assert!(pipeline.is_empty());
    }

    #[test]
    fn pipeline_reset() {
        let mut pipeline = EventPipeline::default();
        pipeline.push_key(Key::Enter, Modifiers::default(), NodeId::default());
        assert!(!pipeline.is_empty());

        pipeline.reset();
        assert!(pipeline.is_empty());
    }
}
