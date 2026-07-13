/// Layout properties for a node. Maps directly to CSS flexbox concepts.
///
/// Uses f32 because flex calculations require fractional values.
/// Taffy uses f32 internally. Final positions are rounded to integers
/// only at the last step.
///
/// Size: ~56 bytes. Stack-allocated.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutProps {
    pub display: Display,
    pub position: Position,
    pub direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify: JustifyContent,
    pub align: AlignItems,
    pub align_self: Option<AlignSelf>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Option<Sizing>,
    pub gap: Option<Gap>,
    pub padding: Option<RectValues>,
    pub margin: Option<RectValues>,
    pub border: Option<RectValues>,
    pub width: Option<Sizing>,
    pub height: Option<Sizing>,
    pub min_width: Option<Sizing>,
    pub min_height: Option<Sizing>,
    pub max_width: Option<Sizing>,
    pub max_height: Option<Sizing>,
    pub inset: Option<RectValues>,
}

impl Default for LayoutProps {
    fn default() -> Self {
        Self {
            display: Display::Flex,
            position: Position::Relative,
            direction: FlexDirection::Column,
            flex_wrap: FlexWrap::NoWrap,
            justify: JustifyContent::FlexStart,
            align: AlignItems::Stretch,
            align_self: None,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: None,
            gap: None,
            padding: None,
            margin: None,
            border: None,
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            inset: None,
        }
    }
}

impl LayoutProps {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Width/height values for sizing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sizing {
    /// Fixed size in terminal cells.
    Points(f32),
    /// Percentage of parent size.
    Percent(f32),
    /// Size determined by content.
    Auto,
}

/// Display mode for a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Display {
    /// Node is laid out and rendered.
    #[default]
    Flex,
    /// Node is removed from layout entirely (CSS `display: none`).
    None,
}

/// Flex wrap mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexWrap {
    /// All children in a single line (may overflow).
    #[default]
    NoWrap,
    /// Children wrap to next line when overflow.
    Wrap,
    /// Children wrap in reverse direction.
    WrapReverse,
}

/// Flex direction for child layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexDirection {
    /// Children laid out horizontally (left to right).
    Row,
    /// Children laid out vertically (top to bottom).
    #[default]
    Column,
    /// Children laid out horizontally (right to left).
    RowReverse,
    /// Children laid out vertically (bottom to top).
    ColumnReverse,
}

/// Alignment along the main axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JustifyContent {
    #[default]
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Alignment along the cross axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    #[default]
    Stretch,
    Baseline,
}

/// Per-child cross axis alignment override.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignSelf {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
}

/// Positioning mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    /// Positioned by flexbox flow.
    #[default]
    Relative,
    /// Removed from flow, positioned relative to parent.
    Absolute,
}

/// Gap between children.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gap {
    /// Gap between rows (main axis for column direction).
    pub row: f32,
    /// Gap between columns (main axis for row direction).
    pub column: f32,
}

impl Gap {
    pub fn new(row: f32, column: f32) -> Self {
        Self { row, column }
    }

    /// Create uniform gap.
    pub fn uniform(gap: f32) -> Self {
        Self {
            row: gap,
            column: gap,
        }
    }
}

/// Rectangular values for padding, margin, and inset.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RectValues {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

impl RectValues {
    /// Create uniform values on all sides.
    pub fn uniform(value: f32) -> Self {
        Self {
            top: Some(value),
            right: Some(value),
            bottom: Some(value),
            left: Some(value),
        }
    }

    /// Create values with horizontal/vertical separation.
    pub fn new(horizontal: f32, vertical: f32) -> Self {
        Self {
            top: Some(vertical),
            right: Some(horizontal),
            bottom: Some(vertical),
            left: Some(horizontal),
        }
    }

    /// Create with individual values.
    pub fn sides(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top: Some(top),
            right: Some(right),
            bottom: Some(bottom),
            left: Some(left),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_layout_props() {
        let props = LayoutProps::default();
        assert_eq!(props.display, Display::Flex);
        assert_eq!(props.position, Position::Relative);
        assert_eq!(props.direction, FlexDirection::Column);
        assert_eq!(props.justify, JustifyContent::FlexStart);
        assert_eq!(props.align, AlignItems::Stretch);
        assert_eq!(props.flex_grow, 0.0);
        assert_eq!(props.flex_shrink, 1.0);
    }

    #[test]
    fn sizing_variants() {
        let fixed = Sizing::Points(100.0);
        let percent = Sizing::Percent(50.0);
        let auto = Sizing::Auto;
        assert_eq!(fixed, Sizing::Points(100.0));
        assert_eq!(percent, Sizing::Percent(50.0));
        assert_eq!(auto, Sizing::Auto);
    }

    #[test]
    fn gap_uniform() {
        let gap = Gap::uniform(5.0);
        assert_eq!(gap.row, 5.0);
        assert_eq!(gap.column, 5.0);
    }

    #[test]
    fn rect_values_uniform() {
        let rect = RectValues::uniform(10.0);
        assert_eq!(rect.top, Some(10.0));
        assert_eq!(rect.right, Some(10.0));
        assert_eq!(rect.bottom, Some(10.0));
        assert_eq!(rect.left, Some(10.0));
    }

    #[test]
    fn rect_values_sides() {
        let rect = RectValues::sides(1.0, 2.0, 3.0, 4.0);
        assert_eq!(rect.top, Some(1.0));
        assert_eq!(rect.right, Some(2.0));
        assert_eq!(rect.bottom, Some(3.0));
        assert_eq!(rect.left, Some(4.0));
    }
}
