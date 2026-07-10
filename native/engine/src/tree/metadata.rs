/// Optional metadata for a node.
///
/// Most nodes don't have metadata. `Option<Box<Metadata>>` means
/// zero overhead for nodes without metadata.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Metadata {
    /// React key for reconciliation.
    pub key: Option<Box<str>>,
    /// Test identifier.
    pub test_id: Option<Box<str>>,
    /// Accessibility label.
    pub aria_label: Option<Box<str>>,
    /// Tooltip text.
    pub tooltip: Option<Box<str>>,
}

impl Metadata {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Accessibility information for a node.
///
/// Screen readers need the full tree structure. Even non-interactive
/// nodes may have accessibility roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Accessibility {
    pub role: AriaRole,
    pub label: Option<AriaLabel>,
    pub description: Option<AriaLabel>,
    pub live: AriaLive,
    pub hidden: bool,
}

/// Aria label as a fixed-size string reference.
/// For Phase 1, we use a simple u32 index. In later phases,
/// this will reference a string table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AriaLabel(pub u32);

/// ARIA roles for accessibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AriaRole {
    #[default]
    Text,
    Button,
    Input,
    List,
    ListItem,
    Table,
    TableRow,
    TableCell,
    Tree,
    TreeItem,
    Tab,
    TabPanel,
    Dialog,
    Alert,
    Status,
    Custom(u16),
}

/// ARIA live region modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AriaLive {
    #[default]
    Off,
    Polite,
    Assertive,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_metadata() {
        let meta = Metadata::default();
        assert!(meta.key.is_none());
        assert!(meta.test_id.is_none());
        assert!(meta.aria_label.is_none());
        assert!(meta.tooltip.is_none());
    }

    #[test]
    fn default_accessibility() {
        let acc = Accessibility::default();
        assert_eq!(acc.role, AriaRole::Text);
        assert!(acc.label.is_none());
        assert!(acc.description.is_none());
        assert_eq!(acc.live, AriaLive::Off);
        assert!(!acc.hidden);
    }

    #[test]
    fn aria_role_custom() {
        let role = AriaRole::Custom(42);
        assert_eq!(role, AriaRole::Custom(42));
        assert_ne!(role, AriaRole::Custom(43));
    }
}
