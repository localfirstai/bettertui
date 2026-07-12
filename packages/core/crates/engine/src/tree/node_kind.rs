/// Identifies the type of a UI node.
///
/// The Rust engine uses this to determine rendering behavior,
/// input handling, and layout strategy.
///
/// `Custom(u16)` allows plugins and widgets to register custom node types
/// without modifying the core enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NodeKind {
    Text,
    #[default]
    Box,
    Flex,
    Input,
    List,
    Table,
    Tree,
    Scroll,
    Tab,
    Modal,
    Code,
    Spacer,
    Separator,
    Custom(u16),
}

impl NodeKind {
    /// Returns a human-readable name for the node kind.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Text => "Text",
            Self::Box => "Box",
            Self::Flex => "Flex",
            Self::Input => "Input",
            Self::List => "List",
            Self::Table => "Table",
            Self::Tree => "Tree",
            Self::Scroll => "Scroll",
            Self::Tab => "Tab",
            Self::Modal => "Modal",
            Self::Code => "Code",
            Self::Spacer => "Spacer",
            Self::Separator => "Separator",
            Self::Custom(_) => "Custom",
        }
    }

    /// Returns true if this node kind is a container (can have children).
    pub fn is_container(&self) -> bool {
        !matches!(self, Self::Text | Self::Spacer | Self::Separator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_kind_is_box() {
        assert_eq!(NodeKind::default(), NodeKind::Box);
    }

    #[test]
    fn kind_names() {
        assert_eq!(NodeKind::Text.name(), "Text");
        assert_eq!(NodeKind::Box.name(), "Box");
        assert_eq!(NodeKind::Custom(42).name(), "Custom");
    }

    #[test]
    fn container_detection() {
        assert!(NodeKind::Box.is_container());
        assert!(NodeKind::Flex.is_container());
        assert!(NodeKind::Scroll.is_container());
        assert!(!NodeKind::Text.is_container());
        assert!(!NodeKind::Spacer.is_container());
        assert!(!NodeKind::Separator.is_container());
    }

    #[test]
    fn custom_kind_equality() {
        assert_eq!(NodeKind::Custom(1), NodeKind::Custom(1));
        assert_ne!(NodeKind::Custom(1), NodeKind::Custom(2));
    }
}
