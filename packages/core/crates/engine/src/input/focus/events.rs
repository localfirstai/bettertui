use crate::tree::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusEventType {
    Focus,
    Blur,
    FocusIn,
    FocusOut,
}

#[derive(Debug, Clone)]
pub struct FocusEvent {
    pub node_id: NodeId,
    pub event_type: FocusEventType,
}

impl FocusEvent {
    pub fn new(node_id: NodeId, event_type: FocusEventType) -> Self {
        Self {
            node_id,
            event_type,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_event_new() {
        let event = FocusEvent::new(NodeId::default(), FocusEventType::Focus);
        assert!(event.is_focus());
    }

    #[test]
    fn focus_event_is_focus() {
        let event = FocusEvent::new(NodeId::default(), FocusEventType::Focus);
        assert!(event.is_focus());
        assert!(!event.is_blur());
    }

    #[test]
    fn focus_event_is_blur() {
        let event = FocusEvent::new(NodeId::default(), FocusEventType::Blur);
        assert!(event.is_blur());
        assert!(!event.is_focus());
    }

    #[test]
    fn focus_event_is_focus_in() {
        let event = FocusEvent::new(NodeId::default(), FocusEventType::FocusIn);
        assert!(event.is_focus_in());
    }

    #[test]
    fn focus_event_is_focus_out() {
        let event = FocusEvent::new(NodeId::default(), FocusEventType::FocusOut);
        assert!(event.is_focus_out());
    }
}
