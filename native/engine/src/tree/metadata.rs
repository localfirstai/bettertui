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
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Accessibility {
    pub role: AriaRole,
    pub label: Option<AriaLabel>,
    pub description: Option<AriaLabel>,
    pub live: AriaLive,
    pub hidden: bool,
    pub properties: AriaProperties,
}

/// ARIA properties for a node.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AriaProperties {
    pub expanded: Option<bool>,
    pub selected: Option<bool>,
    pub checked: Option<AriaChecked>,
    pub disabled: Option<bool>,
    pub pressed: Option<AriaPressed>,
    pub current: Option<AriaCurrent>,
    pub relevant: Option<AriaRelevant>,
    pub atomic: Option<bool>,
    pub busy: Option<bool>,
    pub level: Option<u32>,
    pub value_min: Option<f64>,
    pub value_max: Option<f64>,
    pub value_now: Option<f64>,
    pub value_text: Option<Box<str>>,
}

/// ARIA checked state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AriaChecked {
    #[default]
    False,
    True,
    Mixed,
}

/// ARIA pressed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AriaPressed {
    #[default]
    False,
    True,
    Mixed,
}

/// ARIA current state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AriaCurrent {
    #[default]
    False,
    Page,
    Step,
    Location,
    Date,
    Time,
}

/// ARIA relevant states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AriaRelevant {
    pub additions: bool,
    pub removals: bool,
    pub text: bool,
    pub all: bool,
}

impl Default for AriaRelevant {
    fn default() -> Self {
        Self {
            additions: true,
            removals: false,
            text: true,
            all: false,
        }
    }
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
    Link,
    Checkbox,
    Radio,
    Switch,
    Slider,
    Tab,
    TabPanel,
    Menu,
    Menuitem,
    Menuitemcheckbox,
    Menuitemradio,
    List,
    Listbox,
    ListItem,
    Option,
    Table,
    Grid,
    TableRow,
    TableCell,
    Columnheader,
    Rowheader,
    Tree,
    TreeItem,
    Treegrid,
    Dialog,
    Alertdialog,
    Alert,
    Status,
    Log,
    Marquee,
    Timer,
    Progressbar,
    Toolbar,
    Menubar,
    Tablist,
    Group,
    Region,
    Heading,
    Form,
    Img,
    Complementary,
    Contentinfo,
    Definition,
    Directory,
    Document,
    Feed,
    Figure,
    Footer,
    Header,
    Landmark,
    Main,
    Navigation,
    None,
    Note,
    Presentation,
    Search,
    Separator,
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

/// Focus information for a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FocusInfo {
    pub focusable: bool,
    pub tabindex: Option<i32>,
    pub focused: bool,
}

/// Keyboard navigation support.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KeyboardInfo {
    pub keybindings: Vec<Keybinding>,
    pub roledescription: Option<Box<str>>,
    pub describedby: Option<Box<str>>,
    pub flowto: Option<Box<str>>,
    pub labelledby: Option<Box<str>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keybinding {
    pub key: Box<str>,
    pub description: Box<str>,
}

impl Keybinding {
    pub fn new(key: impl Into<Box<str>>, description: impl Into<Box<str>>) -> Self {
        Self {
            key: key.into(),
            description: description.into(),
        }
    }
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

    #[test]
    fn aria_properties_default() {
        let props = AriaProperties::default();
        assert!(props.expanded.is_none());
        assert!(props.selected.is_none());
        assert!(props.checked.is_none());
        assert!(props.disabled.is_none());
    }

    #[test]
    fn aria_checked_variants() {
        assert_eq!(AriaChecked::False, AriaChecked::False);
        assert_eq!(AriaChecked::True, AriaChecked::True);
        assert_eq!(AriaChecked::Mixed, AriaChecked::Mixed);
    }

    #[test]
    fn aria_pressed_variants() {
        assert_eq!(AriaPressed::False, AriaPressed::False);
        assert_eq!(AriaPressed::True, AriaPressed::True);
        assert_eq!(AriaPressed::Mixed, AriaPressed::Mixed);
    }

    #[test]
    fn aria_current_variants() {
        assert_eq!(AriaCurrent::False, AriaCurrent::False);
        assert_eq!(AriaCurrent::Page, AriaCurrent::Page);
        assert_eq!(AriaCurrent::Step, AriaCurrent::Step);
    }

    #[test]
    fn aria_relevant_default() {
        let rel = AriaRelevant::default();
        assert!(rel.additions);
        assert!(!rel.removals);
        assert!(rel.text);
        assert!(!rel.all);
    }

    #[test]
    fn focus_info_default() {
        let info = FocusInfo::default();
        assert!(!info.focusable);
        assert!(info.tabindex.is_none());
        assert!(!info.focused);
    }

    #[test]
    fn keybinding_new() {
        let kb = Keybinding::new("Enter", "Activate");
        assert_eq!(kb.key.as_ref(), "Enter");
        assert_eq!(kb.description.as_ref(), "Activate");
    }

    #[test]
    fn keyboard_info_default() {
        let info = KeyboardInfo::default();
        assert!(info.keybindings.is_empty());
        assert!(info.roledescription.is_none());
    }

    #[test]
    fn accessibility_with_properties() {
        let acc = Accessibility {
            role: AriaRole::Button,
            properties: AriaProperties {
                expanded: Some(true),
                pressed: Some(AriaPressed::True),
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(acc.role, AriaRole::Button);
        assert_eq!(acc.properties.expanded, Some(true));
        assert_eq!(acc.properties.pressed, Some(AriaPressed::True));
    }
}
