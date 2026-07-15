#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    User,
    Assistant,
    System,
}

impl Role {
    pub fn label(&self) -> &str {
        match self {
            Role::User => "You",
            Role::Assistant => "Assistant",
            Role::System => "System",
        }
    }

    pub fn color_name(&self) -> &str {
        match self {
            Role::User => "blue",
            Role::Assistant => "green",
            Role::System => "gray",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub role: Role,
    pub content: Box<str>,
    pub timestamp: u64,
    pub thinking: Option<Box<str>>,
}

impl Message {
    pub fn user(content: impl Into<Box<str>>, timestamp: u64) -> Self {
        Self { role: Role::User, content: content.into(), timestamp, thinking: None }
    }

    pub fn assistant(content: impl Into<Box<str>>, timestamp: u64) -> Self {
        Self { role: Role::Assistant, content: content.into(), timestamp, thinking: None }
    }

    pub fn system(content: impl Into<Box<str>>, timestamp: u64) -> Self {
        Self { role: Role::System, content: content.into(), timestamp, thinking: None }
    }

    pub fn with_thinking(mut self, thinking: impl Into<Box<str>>) -> Self {
        self.thinking = Some(thinking.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ChatStatus {
    #[default]
    Idle,
    Thinking,
    Streaming,
    Error,
}

impl ChatStatus {
    pub fn label(&self) -> &str {
        match self {
            ChatStatus::Idle => "Ready",
            ChatStatus::Thinking => "Thinking...",
            ChatStatus::Streaming => "Streaming...",
            ChatStatus::Error => "Error",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChatState {
    pub messages: Vec<Message>,
    pub status: ChatStatus,
    pub scroll_offset: usize,
    pub input_text: Box<str>,
    pub cursor_position: usize,
}

impl ChatState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.scroll_offset = 0;
    }

    pub fn set_status(&mut self, status: ChatStatus) {
        self.status = status;
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        let max_scroll = self.messages.len().saturating_sub(1);
        if self.scroll_offset < max_scroll {
            self.scroll_offset += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_label() {
        assert_eq!(Role::User.label(), "You");
        assert_eq!(Role::Assistant.label(), "Assistant");
        assert_eq!(Role::System.label(), "System");
    }

    #[test]
    fn role_color() {
        assert_eq!(Role::User.color_name(), "blue");
        assert_eq!(Role::Assistant.color_name(), "green");
        assert_eq!(Role::System.color_name(), "gray");
    }

    #[test]
    fn message_user() {
        let msg = Message::user("Hello", 100);
        assert_eq!(msg.role, Role::User);
        assert_eq!(msg.content.as_ref(), "Hello");
        assert_eq!(msg.timestamp, 100);
        assert!(msg.thinking.is_none());
    }

    #[test]
    fn message_assistant() {
        let msg = Message::assistant("Hi there", 200);
        assert_eq!(msg.role, Role::Assistant);
        assert_eq!(msg.content.as_ref(), "Hi there");
    }

    #[test]
    fn message_with_thinking() {
        let msg = Message::assistant("Answer", 300).with_thinking("Let me think...");
        assert_eq!(msg.thinking.as_deref(), Some("Let me think..."));
    }

    #[test]
    fn chat_status_label() {
        assert_eq!(ChatStatus::Idle.label(), "Ready");
        assert_eq!(ChatStatus::Thinking.label(), "Thinking...");
        assert_eq!(ChatStatus::Streaming.label(), "Streaming...");
        assert_eq!(ChatStatus::Error.label(), "Error");
    }

    #[test]
    fn chat_state_new() {
        let state = ChatState::new();
        assert!(state.messages.is_empty());
        assert_eq!(state.status, ChatStatus::Idle);
    }

    #[test]
    fn chat_state_add_message() {
        let mut state = ChatState::new();
        state.add_message(Message::user("Hello", 100));
        assert_eq!(state.messages.len(), 1);
    }

    #[test]
    fn chat_state_clear() {
        let mut state = ChatState::new();
        state.add_message(Message::user("Hello", 100));
        state.clear();
        assert!(state.messages.is_empty());
    }

    #[test]
    fn chat_state_scroll() {
        let mut state = ChatState::new();
        for i in 0..5 {
            state.add_message(Message::user(format!("msg{}", i), i));
        }
        state.scroll_down();
        assert_eq!(state.scroll_offset, 1);
        state.scroll_up();
        assert_eq!(state.scroll_offset, 0);
    }
}
