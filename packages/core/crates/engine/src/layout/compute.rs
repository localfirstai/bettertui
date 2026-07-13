use std::cell::RefCell;
use std::collections::HashMap;

use crate::layout::types::{
    AlignItems, AlignSelf, FlexDirection, FlexWrap, JustifyContent, LayoutProps, Position,
    RectValues, Sizing,
};
use crate::tree::NodeId;

use super::result::LayoutResult;

#[derive(Debug)]
pub enum LayoutError {
    NodeNotRegistered(NodeId),
    TaffyError(taffy::TaffyError),
}

impl std::fmt::Display for LayoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutError::NodeNotRegistered(id) => {
                write!(f, "Node {id:?} not registered in layout engine")
            }
            LayoutError::TaffyError(e) => write!(f, "Taffy error: {e}"),
        }
    }
}

impl std::error::Error for LayoutError {}

impl From<taffy::TaffyError> for LayoutError {
    fn from(e: taffy::TaffyError) -> Self {
        LayoutError::TaffyError(e)
    }
}

pub struct LayoutEngine {
    taffy: taffy::TaffyTree<()>,
    node_map: HashMap<NodeId, taffy::NodeId>,
    reverse_map: HashMap<taffy::NodeId, NodeId>,
    /// Text content for text nodes. Used by the measure function to compute intrinsic size.
    text_map: RefCell<HashMap<NodeId, String>>,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            taffy: taffy::TaffyTree::new(),
            node_map: HashMap::new(),
            reverse_map: HashMap::new(),
            text_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn has_node(&self, id: NodeId) -> bool {
        self.node_map.contains_key(&id)
    }

    pub fn node_count(&self) -> usize {
        self.node_map.len()
    }

    pub fn register_node(&mut self, id: NodeId) {
        if self.node_map.contains_key(&id) {
            return;
        }
        let style = taffy::Style::default();
        let taffy_id = self.taffy.new_leaf(style).unwrap();
        self.node_map.insert(id, taffy_id);
        self.reverse_map.insert(taffy_id, id);
    }

    pub fn register_container(&mut self, id: NodeId, props: &LayoutProps) {
        if self.node_map.contains_key(&id) {
            self.update_style(id, props);
            return;
        }
        let style = layout_props_to_taffy(props);
        let taffy_id = self.taffy.new_leaf(style).unwrap();
        self.node_map.insert(id, taffy_id);
        self.reverse_map.insert(taffy_id, id);
    }

    pub fn remove_node(&mut self, id: NodeId) {
        if let Some(taffy_id) = self.node_map.remove(&id) {
            self.reverse_map.remove(&taffy_id);
            self.text_map.borrow_mut().remove(&id);
            let _ = self.taffy.remove(taffy_id);
        }
    }

    pub fn update_style(&mut self, id: NodeId, props: &LayoutProps) {
        if let Some(&taffy_id) = self.node_map.get(&id) {
            let style = layout_props_to_taffy(props);
            self.taffy.set_style(taffy_id, style).unwrap();
        }
    }

    /// Register node as a text node with content for intrinsic sizing.
    /// The measure function will compute the node's size based on text content.
    pub fn register_text_node(&mut self, id: NodeId, props: &LayoutProps, text: &str) {
        if self.node_map.contains_key(&id) {
            self.text_map.borrow_mut().insert(id, text.to_string());
            self.update_style(id, props);
            return;
        }
        let style = layout_props_to_taffy(props);
        let taffy_id = self.taffy.new_leaf(style).unwrap();
        self.node_map.insert(id, taffy_id);
        self.reverse_map.insert(taffy_id, id);
        self.text_map.borrow_mut().insert(id, text.to_string());
    }

    /// Update the text content for an existing text node (for re-measurement).
    pub fn update_text(&mut self, id: NodeId, text: &str) {
        self.text_map.borrow_mut().insert(id, text.to_string());
    }

    pub fn add_child(&mut self, parent: NodeId, child: NodeId) {
        let &p = self.node_map.get(&parent).expect("parent not registered");
        let &c = self.node_map.get(&child).expect("child not registered");
        self.taffy.add_child(p, c).unwrap();
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        let &p = self.node_map.get(&parent).expect("parent not registered");
        let &c = self.node_map.get(&child).expect("child not registered");
        self.taffy.remove_child(p, c).unwrap();
    }

    pub fn compute_layout(
        &mut self,
        root: NodeId,
        width: f32,
        height: f32,
    ) -> Result<(), LayoutError> {
        let &taffy_root = self
            .node_map
            .get(&root)
            .ok_or(LayoutError::NodeNotRegistered(root))?;
        let size = taffy::Size {
            width: taffy::AvailableSpace::Definite(width),
            height: taffy::AvailableSpace::Definite(height),
        };
        let reverse_map = &self.reverse_map;
        let text_map = &self.text_map;

        self.taffy.compute_layout_with_measure(
            taffy_root,
            size,
            |known_dimensions, available_space, node_id, _context, _style| {
                // Short-circuit if both dimensions are already known
                if let taffy::Size {
                    width: Some(w),
                    height: Some(h),
                } = known_dimensions
                {
                    return taffy::Size {
                        width: w,
                        height: h,
                    };
                }

                // Map Taffy's NodeId to our NodeId to look up text
                let our_id = match reverse_map.get(&node_id) {
                    Some(id) => *id,
                    None => return taffy::Size::ZERO,
                };

                let text = text_map.borrow();
                let content = match text.get(&our_id) {
                    Some(t) => t.as_str(),
                    None => return taffy::Size::ZERO,
                };

                let available_width = match available_space.width {
                    taffy::AvailableSpace::Definite(w) => w,
                    _ => f32::INFINITY,
                };

                // Measure text: compute how many lines fit in available width
                let char_width = 1.0_f32;
                let char_height = 1.0_f32;
                let max_chars_per_line = (available_width / char_width).floor().max(1.0) as usize;

                // If no wrap, intrinsic width is the max line width
                // We don't have wrap info here, so we measure for wrapping
                let text_str = content;
                let total_width = text_str
                    .lines()
                    .map(|line| unicode_width::UnicodeWidthStr::width(line) as f32 * char_width)
                    .fold(0.0_f32, f32::max);

                let wrapped_width = available_width.min(total_width);
                let chars_per_line = (wrapped_width / char_width).floor().max(1.0) as usize;
                let lines = if max_chars_per_line > 0 && total_width > available_width {
                    // Count wrapping lines
                    text_str
                        .lines()
                        .map(|line| {
                            let w = unicode_width::UnicodeWidthStr::width(line);
                            if w == 0 {
                                1
                            } else {
                                w.div_ceil(chars_per_line)
                            }
                        })
                        .sum::<usize>()
                } else {
                    text_str.lines().count().max(1)
                };

                taffy::Size {
                    width: known_dimensions.width.unwrap_or(wrapped_width),
                    height: known_dimensions
                        .height
                        .unwrap_or(lines as f32 * char_height),
                }
            },
        )?;
        Ok(())
    }

    pub fn collect_results(&self) -> HashMap<NodeId, LayoutResult> {
        let mut results = HashMap::new();
        for (&node_id, &taffy_id) in &self.node_map {
            if let Ok(layout) = self.taffy.layout(taffy_id) {
                results.insert(
                    node_id,
                    LayoutResult {
                        x: (layout.location.x.round() as i32).max(0) as u16,
                        y: (layout.location.y.round() as i32).max(0) as u16,
                        width: (layout.size.width.round() as i32).max(0) as u16,
                        height: (layout.size.height.round() as i32).max(0) as u16,
                        content_width: (layout.content_box_width().round() as i32).max(0) as u16,
                        content_height: (layout.content_box_height().round() as i32).max(0) as u16,
                        padding_top: (layout.padding.top.round() as i32).max(0) as u16,
                        padding_right: (layout.padding.right.round() as i32).max(0) as u16,
                        padding_bottom: (layout.padding.bottom.round() as i32).max(0) as u16,
                        padding_left: (layout.padding.left.round() as i32).max(0) as u16,
                        border_top: (layout.border.top.round() as i32).max(0) as u16,
                        border_right: (layout.border.right.round() as i32).max(0) as u16,
                        border_bottom: (layout.border.bottom.round() as i32).max(0) as u16,
                        border_left: (layout.border.left.round() as i32).max(0) as u16,
                    },
                );
            }
        }
        results
    }
}

fn sizing_to_taffy(sizing: Option<Sizing>) -> taffy::Dimension {
    match sizing {
        Some(Sizing::Points(p)) => taffy::Dimension::Length(p),
        Some(Sizing::Percent(p)) => taffy::Dimension::Percent(p.clamp(0.0, 1.0)),
        Some(Sizing::Auto) | None => taffy::Dimension::Auto,
    }
}

fn rect_values_to_taffy(r: &RectValues) -> taffy::Rect<taffy::LengthPercentage> {
    taffy::Rect {
        top: r
            .top
            .map(taffy::LengthPercentage::Length)
            .unwrap_or(taffy::LengthPercentage::Length(0.0)),
        right: r
            .right
            .map(taffy::LengthPercentage::Length)
            .unwrap_or(taffy::LengthPercentage::Length(0.0)),
        bottom: r
            .bottom
            .map(taffy::LengthPercentage::Length)
            .unwrap_or(taffy::LengthPercentage::Length(0.0)),
        left: r
            .left
            .map(taffy::LengthPercentage::Length)
            .unwrap_or(taffy::LengthPercentage::Length(0.0)),
    }
}

fn rect_values_to_taffy_auto(r: &RectValues) -> taffy::Rect<taffy::LengthPercentageAuto> {
    taffy::Rect {
        top: r
            .top
            .map(taffy::LengthPercentageAuto::Length)
            .unwrap_or(taffy::LengthPercentageAuto::Length(0.0)),
        right: r
            .right
            .map(taffy::LengthPercentageAuto::Length)
            .unwrap_or(taffy::LengthPercentageAuto::Length(0.0)),
        bottom: r
            .bottom
            .map(taffy::LengthPercentageAuto::Length)
            .unwrap_or(taffy::LengthPercentageAuto::Length(0.0)),
        left: r
            .left
            .map(taffy::LengthPercentageAuto::Length)
            .unwrap_or(taffy::LengthPercentageAuto::Length(0.0)),
    }
}

fn map_align_items(val: AlignItems) -> taffy::AlignItems {
    match val {
        AlignItems::FlexStart => taffy::AlignItems::FlexStart,
        AlignItems::FlexEnd => taffy::AlignItems::FlexEnd,
        AlignItems::Center => taffy::AlignItems::Center,
        AlignItems::Stretch => taffy::AlignItems::Stretch,
        AlignItems::Baseline => taffy::AlignItems::Baseline,
    }
}

fn map_justify_content(val: JustifyContent) -> taffy::JustifyContent {
    match val {
        JustifyContent::FlexStart => taffy::JustifyContent::FlexStart,
        JustifyContent::FlexEnd => taffy::JustifyContent::FlexEnd,
        JustifyContent::Center => taffy::JustifyContent::Center,
        JustifyContent::SpaceBetween => taffy::JustifyContent::SpaceBetween,
        JustifyContent::SpaceAround => taffy::JustifyContent::SpaceAround,
        JustifyContent::SpaceEvenly => taffy::JustifyContent::SpaceEvenly,
    }
}

fn map_flex_direction(val: FlexDirection) -> taffy::FlexDirection {
    match val {
        FlexDirection::Row => taffy::FlexDirection::Row,
        FlexDirection::Column => taffy::FlexDirection::Column,
        FlexDirection::RowReverse => taffy::FlexDirection::RowReverse,
        FlexDirection::ColumnReverse => taffy::FlexDirection::ColumnReverse,
    }
}

fn rect_values_to_inset(r: &RectValues) -> taffy::Rect<taffy::LengthPercentageAuto> {
    taffy::Rect {
        top: r
            .top
            .map(taffy::LengthPercentageAuto::Length)
            .unwrap_or(taffy::LengthPercentageAuto::Auto),
        right: r
            .right
            .map(taffy::LengthPercentageAuto::Length)
            .unwrap_or(taffy::LengthPercentageAuto::Auto),
        bottom: r
            .bottom
            .map(taffy::LengthPercentageAuto::Length)
            .unwrap_or(taffy::LengthPercentageAuto::Auto),
        left: r
            .left
            .map(taffy::LengthPercentageAuto::Length)
            .unwrap_or(taffy::LengthPercentageAuto::Auto),
    }
}

fn map_flex_wrap(val: FlexWrap) -> taffy::FlexWrap {
    match val {
        FlexWrap::NoWrap => taffy::FlexWrap::NoWrap,
        FlexWrap::Wrap => taffy::FlexWrap::Wrap,
        FlexWrap::WrapReverse => taffy::FlexWrap::WrapReverse,
    }
}

fn map_position(val: Position) -> taffy::Position {
    match val {
        Position::Relative => taffy::Position::Relative,
        Position::Absolute => taffy::Position::Absolute,
    }
}

fn map_align_self(val: AlignSelf) -> taffy::AlignSelf {
    match val {
        AlignSelf::FlexStart => taffy::AlignSelf::FlexStart,
        AlignSelf::FlexEnd => taffy::AlignSelf::FlexEnd,
        AlignSelf::Center => taffy::AlignSelf::Center,
        AlignSelf::Stretch => taffy::AlignSelf::Stretch,
        AlignSelf::Baseline => taffy::AlignSelf::Baseline,
    }
}

fn layout_props_to_taffy(props: &LayoutProps) -> taffy::Style {
    let padding = props
        .padding
        .map(|r| rect_values_to_taffy(&r))
        .unwrap_or(taffy::Rect {
            top: taffy::LengthPercentage::Length(0.0),
            right: taffy::LengthPercentage::Length(0.0),
            bottom: taffy::LengthPercentage::Length(0.0),
            left: taffy::LengthPercentage::Length(0.0),
        });
    let margin = props
        .margin
        .map(|r| rect_values_to_taffy_auto(&r))
        .unwrap_or(taffy::Rect {
            top: taffy::LengthPercentageAuto::Length(0.0),
            right: taffy::LengthPercentageAuto::Length(0.0),
            bottom: taffy::LengthPercentageAuto::Length(0.0),
            left: taffy::LengthPercentageAuto::Length(0.0),
        });
    let border = props
        .border
        .map(|r| rect_values_to_taffy(&r))
        .unwrap_or(taffy::Rect {
            top: taffy::LengthPercentage::Length(0.0),
            right: taffy::LengthPercentage::Length(0.0),
            bottom: taffy::LengthPercentage::Length(0.0),
            left: taffy::LengthPercentage::Length(0.0),
        });

    let gap = match props.gap {
        Some(g) => taffy::Size {
            width: taffy::LengthPercentage::Length(g.column),
            height: taffy::LengthPercentage::Length(g.row),
        },
        None => taffy::Size {
            width: taffy::LengthPercentage::Length(0.0),
            height: taffy::LengthPercentage::Length(0.0),
        },
    };

    let size = taffy::Size {
        width: sizing_to_taffy(props.width),
        height: sizing_to_taffy(props.height),
    };

    taffy::Style {
        display: match props.display {
            crate::layout::types::Display::Flex => taffy::Display::Flex,
            crate::layout::types::Display::None => taffy::Display::None,
        },
        position: map_position(props.position),
        flex_direction: map_flex_direction(props.direction),
        flex_wrap: map_flex_wrap(props.flex_wrap),
        align_items: Some(map_align_items(props.align)),
        align_self: props.align_self.map(map_align_self),
        justify_content: Some(map_justify_content(props.justify)),
        flex_grow: props.flex_grow,
        flex_shrink: props.flex_shrink,
        flex_basis: sizing_to_taffy(props.flex_basis),
        size,
        min_size: taffy::Size {
            width: sizing_to_taffy(props.min_width),
            height: sizing_to_taffy(props.min_height),
        },
        max_size: taffy::Size {
            width: sizing_to_taffy(props.max_width),
            height: sizing_to_taffy(props.max_height),
        },
        inset: props
            .inset
            .map(|r| rect_values_to_inset(&r))
            .unwrap_or(taffy::Rect {
                top: taffy::LengthPercentageAuto::Auto,
                right: taffy::LengthPercentageAuto::Auto,
                bottom: taffy::LengthPercentageAuto::Auto,
                left: taffy::LengthPercentageAuto::Auto,
            }),
        padding,
        margin,
        border,
        gap,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::arena::NodeArena;
    use crate::tree::node_kind::NodeKind;
    use crate::tree::render_node::RenderNode;

    fn make_ids(count: usize) -> Vec<NodeId> {
        let mut arena = NodeArena::new();
        let mut ids = Vec::new();
        for _ in 0..count {
            ids.push(arena.insert(RenderNode::new(NodeKind::Box)));
        }
        ids
    }

    #[test]
    fn engine_new() {
        let engine = LayoutEngine::new();
        assert_eq!(engine.node_map.len(), 0);
    }

    #[test]
    fn register_node_and_remove() {
        let mut engine = LayoutEngine::new();
        let ids = make_ids(1);
        let id = ids[0];
        engine.register_node(id);
        assert!(engine.node_map.contains_key(&id));
        engine.remove_node(id);
        assert!(!engine.node_map.contains_key(&id));
    }

    #[test]
    fn register_container_and_compute() {
        let mut engine = LayoutEngine::new();
        let ids = make_ids(1);
        let id = ids[0];
        let props = LayoutProps {
            width: Some(Sizing::Points(80.0)),
            height: Some(Sizing::Points(24.0)),
            ..Default::default()
        };
        engine.register_container(id, &props);
        engine.compute_layout(id, 80.0, 24.0).unwrap();
        let results = engine.collect_results();
        assert!(results.contains_key(&id));
        let result = results.get(&id).unwrap();
        assert_eq!(result.width, 80);
        assert_eq!(result.height, 24);
    }

    #[test]
    fn add_child_and_compute() {
        let mut engine = LayoutEngine::new();
        let ids = make_ids(2);
        let parent = ids[0];
        let child = ids[1];
        let parent_props = LayoutProps {
            width: Some(Sizing::Points(80.0)),
            height: Some(Sizing::Points(24.0)),
            ..Default::default()
        };
        let child_props = LayoutProps {
            width: Some(Sizing::Points(20.0)),
            height: Some(Sizing::Points(10.0)),
            ..Default::default()
        };
        engine.register_container(parent, &parent_props);
        engine.register_container(child, &child_props);
        engine.add_child(parent, child);
        engine.compute_layout(parent, 80.0, 24.0).unwrap();
        let results = engine.collect_results();
        assert!(results.contains_key(&parent));
        assert!(results.contains_key(&child));
    }

    #[test]
    fn multiple_children() {
        let mut engine = LayoutEngine::new();
        let ids = make_ids(3);
        let parent = ids[0];
        let child1 = ids[1];
        let child2 = ids[2];
        let parent_props = LayoutProps {
            width: Some(Sizing::Points(80.0)),
            height: Some(Sizing::Points(24.0)),
            ..Default::default()
        };
        let child_props = LayoutProps {
            width: Some(Sizing::Points(40.0)),
            height: Some(Sizing::Points(5.0)),
            ..Default::default()
        };
        engine.register_container(parent, &parent_props);
        engine.register_container(child1, &child_props);
        engine.register_container(child2, &child_props);
        engine.add_child(parent, child1);
        engine.add_child(parent, child2);
        engine.compute_layout(parent, 80.0, 24.0).unwrap();
        let results = engine.collect_results();
        assert!(results.len() >= 3);
    }

    #[test]
    fn child_positioning_column() {
        let mut engine = LayoutEngine::new();
        let ids = make_ids(2);
        let parent = ids[0];
        let child = ids[1];
        let parent_props = LayoutProps {
            width: Some(Sizing::Points(80.0)),
            height: Some(Sizing::Points(24.0)),
            direction: FlexDirection::Column,
            ..Default::default()
        };
        let child_props = LayoutProps {
            width: Some(Sizing::Points(20.0)),
            height: Some(Sizing::Points(10.0)),
            ..Default::default()
        };
        engine.register_container(parent, &parent_props);
        engine.register_container(child, &child_props);
        engine.add_child(parent, child);
        engine.compute_layout(parent, 80.0, 24.0).unwrap();
        let results = engine.collect_results();
        let parent_r = results.get(&parent).unwrap();
        let child_r = results.get(&child).unwrap();
        assert_eq!(parent_r.width, 80);
        assert_eq!(child_r.width, 20);
        assert_eq!(child_r.height, 10);
    }

    #[test]
    fn child_positioning_row() {
        let mut engine = LayoutEngine::new();
        let ids = make_ids(2);
        let parent = ids[0];
        let child = ids[1];
        let parent_props = LayoutProps {
            width: Some(Sizing::Points(80.0)),
            height: Some(Sizing::Points(24.0)),
            direction: FlexDirection::Row,
            ..Default::default()
        };
        let child_props = LayoutProps {
            width: Some(Sizing::Points(20.0)),
            height: Some(Sizing::Points(10.0)),
            ..Default::default()
        };
        engine.register_container(parent, &parent_props);
        engine.register_container(child, &child_props);
        engine.add_child(parent, child);
        engine.compute_layout(parent, 80.0, 24.0).unwrap();
        let results = engine.collect_results();
        let parent_r = results.get(&parent).unwrap();
        let child_r = results.get(&child).unwrap();
        assert_eq!(parent_r.width, 80);
        assert_eq!(child_r.width, 20);
    }
}
