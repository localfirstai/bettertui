//! Layout engine: Taffy-based layout computation with tree synchronization and result mapping.
//!
//! This module contains everything related to layout:
//! - Types — Input types: LayoutProps, Sizing, enums (FlexDirection, etc.)
//! - Paint — Paint primitives: PaintBounds, Viewport, ClipBounds, PaintContext, PaintFlags
//! - Culling — Binary search viewport culling for large scrollable lists
//! - Engine — LayoutEngine, LayoutTreeSync, LayoutResult: Taffy wrapping + arena bridging + result mapping
//! - Build — Render tree builder: consumes LayoutResults → produces RenderTree

use std::cell::RefCell;
use std::collections::HashMap;

use bitflags::bitflags;
use unicode_segmentation::UnicodeSegmentation;

use crate::render::RenderObject;
use crate::render::RenderTree;
use crate::text;
use crate::tree::NodeArena;
use crate::tree::NodeId;
use crate::tree::Overflow;
use crate::tree::Rect;

// ============================================================================
// BACKWARD COMPATIBILITY: Re-export submodules
// ============================================================================

/// Types submodule for backward compatibility with `crate::taffy::types::*`
pub mod types {
    pub use super::{
        AlignItems, AlignSelf, BoxSizing, FlexDirection, FlexWrap, Gap, JustifyContent, LayoutOverflow, LayoutProps,
        Position, RectValues, Sizing,
    };
    /// Display mode for a node (re-exported for backward compatibility)
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum Display {
        /// Node is laid out and rendered.
        #[default]
        Flex,
        /// Node is removed from layout entirely (CSS `display: none`).
        None,
    }
}

/// Paint submodule for backward compatibility with `crate::taffy::paint::*`
pub mod paint {
    pub use super::{ClipBounds, PaintBounds, PaintContext, PaintFlags, Viewport};
}

// Re-export types at top level for backward compatibility
pub use types::Display;

// ============================================================================
// TYPES
// ============================================================================

/// Layout properties for a node. Maps directly to CSS flexbox concepts.
///
/// Uses f32 because flex calculations require fractional values.
/// Taffy uses f32 internally. Final positions are rounded to integers
/// only at the last step.
///
/// Size: ~56 bytes. Stack-allocated.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutProps {
    pub display: types::Display,
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
    /// Aspect ratio (width / height).
    pub aspect_ratio: Option<f32>,
    /// Overflow behavior for this container.
    pub overflow: Option<LayoutOverflow>,
    /// Box sizing model (`border-box` vs `content-box`).
    pub box_sizing: Option<BoxSizing>,
}

impl Default for LayoutProps {
    fn default() -> Self {
        Self {
            display: types::Display::Flex,
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
            aspect_ratio: None,
            overflow: None,
            box_sizing: None,
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
    /// Positioned according to normal flow.
    Static,
}

/// Layout-level overflow behavior for flex items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutOverflow {
    /// Content is not clipped (may overflow the container).
    #[default]
    Visible,
    /// Content is clipped to the container bounds.
    Hidden,
    /// Content is clipped and scrollable.
    Scroll,
}

/// Box sizing model, mirroring CSS `box-sizing`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoxSizing {
    /// `border-box`: width/height includes padding and border.
    BorderBox,
    /// `content-box`: width/height excludes padding and border (CSS default).
    #[default]
    ContentBox,
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
        Self { row: gap, column: gap }
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
        Self { top: Some(value), right: Some(value), bottom: Some(value), left: Some(value) }
    }

    /// Create values with horizontal/vertical separation.
    pub fn new(horizontal: f32, vertical: f32) -> Self {
        Self { top: Some(vertical), right: Some(horizontal), bottom: Some(vertical), left: Some(horizontal) }
    }

    /// Create with individual values.
    pub fn sides(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top: Some(top), right: Some(right), bottom: Some(bottom), left: Some(left) }
    }
}

// ============================================================================
// PAINT
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PaintBounds {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub padding_left: u16,
    pub padding_right: u16,
    pub padding_top: u16,
    pub padding_bottom: u16,
    pub border_top: u16,
    pub border_right: u16,
    pub border_bottom: u16,
    pub border_left: u16,
}

impl PaintBounds {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height, ..Default::default() }
    }

    pub fn with_padding(mut self, left: u16, right: u16, top: u16, bottom: u16) -> Self {
        self.padding_left = left;
        self.padding_right = right;
        self.padding_top = top;
        self.padding_bottom = bottom;
        self
    }

    pub fn with_border(mut self, top: u16, right: u16, bottom: u16, left: u16) -> Self {
        self.border_top = top;
        self.border_right = right;
        self.border_bottom = bottom;
        self.border_left = left;
        self
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    pub fn border_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    pub fn content_rect(&self) -> Rect {
        Rect::new(
            self.x + self.border_left + self.padding_left,
            self.y + self.border_top + self.padding_top,
            self.width.saturating_sub(self.border_left + self.border_right + self.padding_left + self.padding_right),
            self.height.saturating_sub(self.border_top + self.border_bottom + self.padding_top + self.padding_bottom),
        )
    }

    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    pub fn intersects(&self, other: &PaintBounds) -> bool {
        self.x < other.right() && self.right() > other.x && self.y < other.bottom() && self.bottom() > other.y
    }

    pub fn intersect(&self, other: &PaintBounds) -> Option<PaintBounds> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());
        if right > x && bottom > y { Some(PaintBounds::new(x, y, right - x, bottom - y)) } else { None }
    }
}

/// Viewport defines the visible region of the terminal.
/// Used for culling: nodes outside the viewport are skipped during render tree building.
/// Unlike ClipBounds (which clips rendering), Viewport is a culling-only concept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Viewport {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Viewport {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub fn contains_rect(&self, px: u16, py: u16, pw: u16, ph: u16) -> bool {
        let r = px.saturating_add(pw);
        let b = py.saturating_add(ph);
        r > self.x && px < self.right() && b > self.y && py < self.bottom()
    }

    pub fn intersect(&self, other: &Viewport) -> Option<Viewport> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let r = self.right().min(other.right());
        let b = self.bottom().min(other.bottom());
        if r > x && b > y { Some(Viewport::new(x, y, r - x, b - y)) } else { None }
    }

    pub fn offset(&self, dx: i32, dy: i32) -> Viewport {
        Viewport::new((self.x as i32 + dx).max(0) as u16, (self.y as i32 + dy).max(0) as u16, self.width, self.height)
    }

    /// Expand viewport by `padding` cells on all sides.
    /// Prevents objects from popping in/out at viewport edges during scroll.
    pub fn with_padding(&self, padding: u16) -> Viewport {
        Viewport::new(
            self.x.saturating_sub(padding),
            self.y.saturating_sub(padding),
            self.width.saturating_add(padding * 2),
            self.height.saturating_add(padding * 2),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClipBounds {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl ClipBounds {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    pub fn from_rect(rect: &Rect) -> Self {
        Self { x: rect.x, y: rect.y, width: rect.width, height: rect.height }
    }

    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    pub fn intersect(&self, other: &ClipBounds) -> Option<ClipBounds> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());
        if right > x && bottom > y { Some(ClipBounds::new(x, y, right - x, bottom - y)) } else { None }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PaintFlags: u8 {
        const EMPTY       = 0b0000_0000;
        const BACKGROUND   = 0b0000_0001;
        const BORDER       = 0b0000_0010;
        const TEXT         = 0b0000_0100;
        const SCROLLBAR    = 0b0000_1000;
        const CURSOR       = 0b0001_0000;
        const OVERLAY      = 0b0010_0000;
        const HIDDEN       = 0b0100_0000;
        const NEEDS_CLIP   = 0b1000_0000;
    }
}

use smallvec::SmallVec;

#[cfg(test)]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(test)]
use std::sync::{Arc, Mutex};

// ============================================================================
// CALLBACK TYPES
// ============================================================================

/// Callback invoked when a node's layout becomes dirty.
/// Useful for invalidating cached layout data or triggering re-renders.
type DirtiedCallback = Box<dyn Fn(NodeId) + Send + 'static>;

/// Result of a custom measure function.
pub struct MeasureResult {
    pub width: f32,
    pub height: f32,
}

/// Callback for measuring a node's intrinsic size.
/// Allows custom measurement strategies (text content, rendered widgets, etc.)
/// instead of using the default text-based heuristic.
///
/// Parameters: (known_width, known_height, available_width, available_height)
/// Returns: (measured_width, measured_height)
type MeasureCallback = Box<dyn Fn(Option<f32>, Option<f32>, f32, f32) -> MeasureResult + Send + 'static>;

#[derive(Clone)]
pub struct PaintContext {
    pub terminal_width: u16,
    pub terminal_height: u16,
    pub clip_stack: SmallVec<[ClipBounds; 8]>,
}

impl PaintContext {
    pub fn new(width: u16, height: u16) -> Self {
        Self { terminal_width: width, terminal_height: height, clip_stack: SmallVec::new() }
    }

    pub fn push_clip(&mut self, clip: ClipBounds) {
        let effective = if let Some(parent) = self.clip_stack.last() {
            parent.intersect(&clip).unwrap_or(ClipBounds::new(0, 0, 0, 0))
        } else {
            clip
        };
        self.clip_stack.push(effective);
    }

    pub fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }

    pub fn current_clip(&self) -> Option<&ClipBounds> {
        self.clip_stack.last()
    }

    pub fn is_visible(&self, bounds: &PaintBounds) -> bool {
        if let Some(clip) = self.clip_stack.last() {
            clip.intersect(&ClipBounds::new(bounds.x, bounds.y, bounds.width, bounds.height)).is_some()
        } else {
            true
        }
    }

    pub fn clipped_bounds(&self, bounds: &PaintBounds) -> Option<PaintBounds> {
        if let Some(clip) = self.clip_stack.last() {
            let cb = ClipBounds::new(bounds.x, bounds.y, bounds.width, bounds.height);
            cb.intersect(clip).map(|c| PaintBounds::new(c.x, c.y, c.width, c.height))
        } else {
            Some(*bounds)
        }
    }
}

// ============================================================================
// CULLING
// ============================================================================

/// Binary search viewport culling for large scrollable lists.
///
/// Pattern adapted from OpenTUI's `getObjectsInViewport`.
/// Uses binary search + interval expansion to find visible children
/// in O(log N + K) time where K is the number of visible objects.
///
/// A positioned child in a sorted array for binary search culling.
#[derive(Debug, Clone, Copy)]
pub struct PositionedChild {
    pub id: NodeId,
    /// Primary-axis start position (y for column layout, x for row layout).
    pub start: u16,
    /// Primary-axis size (height for column, width for row).
    pub size: u16,
}

/// Padding to apply when culling — keeps a buffer of visible objects
/// just outside the viewport for smooth scrolling. Matches OpenTUI's padding.
pub const CULLING_PADDING: u16 = 5;

/// Returns children that intersect the given viewport along the primary axis.
///
/// `children` must be pre-sorted by `start` ascending.
/// Uses binary search for O(log N) lookup, then expands left/right.
///
/// This is specifically for scroll containers with many children.
/// The viewport should already be offset by scroll position.
///
/// A `CULLING_PADDING` buffer is applied so objects just outside the
/// viewport are still included (prevents pop-in during smooth scrolling).
pub fn get_objects_in_viewport(
    viewport: &Viewport,
    children: &[PositionedChild],
    primary_axis: PrimaryAxis,
) -> Vec<NodeId> {
    if children.is_empty() || viewport.width == 0 || viewport.height == 0 {
        return Vec::new();
    }

    // Apply culling padding to prevent pop-in during scrolling
    let vp_padded = viewport.with_padding(CULLING_PADDING);

    // Small arrays: skip binary search overhead
    if children.len() < 16 {
        return children
            .iter()
            .filter(|c| {
                let end = c.start.saturating_add(c.size);
                end > viewport_start(&vp_padded, primary_axis) && c.start < viewport_end(&vp_padded, primary_axis)
            })
            .map(|c| c.id)
            .collect();
    }

    let vp_start = viewport_start(&vp_padded, primary_axis);
    let vp_end = viewport_end(&vp_padded, primary_axis);

    // Binary search for first overlapping child
    let mut lo = 0i32;
    let mut hi = children.len() as i32 - 1;
    let mut candidate: Option<usize> = None;

    while lo <= hi {
        let mid = ((lo + hi) >> 1) as usize;
        let c = &children[mid];
        let end = c.start.saturating_add(c.size);

        if end <= vp_start {
            lo = mid as i32 + 1;
        } else if c.start >= vp_end {
            hi = mid as i32 - 1;
        } else {
            candidate = Some(mid);
            break;
        }
    }

    let Some(center) = candidate else {
        // Viewport is in a gap — start from where search ended.
        // Clamp to last valid index since `lo` can be children.len()
        // when all children are before the viewport.
        let start_idx = (lo.max(0) as usize).min(children.len().saturating_sub(1));
        return expand_from(children, start_idx, vp_start, vp_end, primary_axis);
    };

    // Expand left with bounded look-behind for spanning objects
    let max_look_behind = 50;
    let mut left = center;
    let mut gap_count = 0;

    while left > 0 {
        let prev = &children[left - 1];
        let prev_end = prev.start.saturating_add(prev.size);

        if prev_end <= vp_start {
            gap_count += 1;
            if gap_count >= max_look_behind {
                break;
            }
        } else {
            gap_count = 0;
        }
        left -= 1;
    }

    // Expand right
    let mut right = center + 1;
    while right < children.len() {
        let next = &children[right];
        if next.start >= vp_end {
            break;
        }
        right += 1;
    }

    // Collect visible children
    children[left..right]
        .iter()
        .filter(|c| {
            let end = c.start.saturating_add(c.size);
            end > vp_start && c.start < vp_end
        })
        .map(|c| c.id)
        .collect()
}

fn expand_from(
    children: &[PositionedChild],
    start_idx: usize,
    vp_start: u16,
    vp_end: u16,
    _axis: PrimaryAxis,
) -> Vec<NodeId> {
    let mut result = Vec::new();
    // Scan backward
    let mut i = start_idx as i32;
    let mut look_behind = 0;
    while i >= 0 {
        let c = &children[i as usize];
        let end = c.start.saturating_add(c.size);
        if end > vp_start.saturating_sub(10) && c.start < vp_end {
            result.push(c.id);
            look_behind = 0;
        } else {
            look_behind += 1;
            if look_behind >= 50 {
                break;
            }
        }
        i -= 1;
    }
    result.reverse();
    // Scan forward with early termination
    let mut i = start_idx;
    while i < children.len() {
        let c = &children[i];
        if c.start >= vp_end {
            break;
        }
        let end = c.start.saturating_add(c.size);
        if end > vp_start {
            result.push(c.id);
        }
        i += 1;
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryAxis {
    Column, // Sort by y (vertical layout)
    Row,    // Sort by x (horizontal layout)
}

fn viewport_start(vp: &Viewport, axis: PrimaryAxis) -> u16 {
    match axis {
        PrimaryAxis::Column => vp.y,
        PrimaryAxis::Row => vp.x,
    }
}

fn viewport_end(vp: &Viewport, axis: PrimaryAxis) -> u16 {
    match axis {
        PrimaryAxis::Column => vp.bottom(),
        PrimaryAxis::Row => vp.right(),
    }
}

// ============================================================================
// CONFIG (matching OpenTUI's YGConfig)
// ============================================================================

/// Layout configuration controlling global layout behavior.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutConfig {
    /// Scale factor for rounding layout positions.
    /// Terminal cells are 1:1, so this is typically 1.0.
    pub point_scale_factor: f32,
    /// Whether to use web-like defaults for flexbox.
    /// When `true`, nodes default to flex row direction.
    /// When `false` (default), nodes default to column direction.
    pub use_web_defaults: bool,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self { point_scale_factor: 1.0, use_web_defaults: false }
    }
}

// ============================================================================
// ENGINE
// ============================================================================

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

/// Measure text content for intrinsic layout sizing.
///
/// Uses grapheme-aware width calculation to properly handle:
/// - Wide characters (CJK, emoji)
/// - Zero-width joiners and combining marks
/// - Multi-codepoint graphemes (flag emojis, skin tone modifiers)
///
/// Returns (intrinsic_width, line_count).
///
/// # Arguments
/// * `text` - The text content to measure
/// * `available_width` - Available width for wrapping (f32::INFINITY for no wrap)
///
/// # Returns
/// * `intrinsic_width` - The computed width (either max line width or constrained width)
/// * `line_count` - Number of lines after wrapping
fn measure_text(text: &str, available_width: f32) -> (f32, usize) {
    if text.is_empty() {
        return (1.0, 1);
    }

    let line_widths: Vec<usize> = text.lines().map(text::display_width).collect();

    let max_width = line_widths.iter().copied().max().unwrap_or(0);

    if available_width.is_infinite() || available_width <= 0.0 {
        return (max_width.max(1) as f32, line_widths.len().max(1));
    }

    let wrap_width = available_width.floor() as usize;

    if max_width <= wrap_width {
        return (max_width.max(1) as f32, line_widths.len().max(1));
    }

    let mut total_lines = 0usize;
    for line in text.lines() {
        total_lines += count_wrapped_lines(line, wrap_width);
    }

    let intrinsic_width = if wrap_width > 0 { wrap_width.min(max_width) as f32 } else { max_width.max(1) as f32 };

    (intrinsic_width, total_lines.max(1))
}

fn count_wrapped_lines(line: &str, wrap_width: usize) -> usize {
    if line.is_empty() || wrap_width == 0 {
        return 1;
    }

    let line_width = text::display_width(line);
    if line_width <= wrap_width {
        return 1;
    }

    let mut current_line_width = 0usize;
    let mut line_count = 1usize;
    let mut word_width = 0usize;

    for grapheme in line.graphemes(true) {
        let g_width = text::grapheme_width(grapheme);

        if current_line_width + word_width + g_width > wrap_width {
            if current_line_width > 0 {
                line_count += 1;
                current_line_width = 0;
            } else if word_width > 0 {
                line_count += 1;
                current_line_width = g_width;
                word_width = 0;
                continue;
            }
        }

        let is_whitespace = grapheme.chars().all(|c| c.is_whitespace());

        if is_whitespace {
            current_line_width += word_width + g_width;
            word_width = 0;
        } else {
            word_width += g_width;
        }
    }

    if word_width > 0 && current_line_width + word_width > wrap_width && current_line_width > 0 {
        line_count += 1;
    }

    line_count
}

pub struct LayoutEngine {
    taffy: taffy::TaffyTree<()>,
    node_map: HashMap<NodeId, taffy::NodeId>,
    reverse_map: HashMap<taffy::NodeId, NodeId>,
    /// Text content for text nodes. Used by the default measure function to compute intrinsic size.
    text_map: RefCell<HashMap<NodeId, String>>,
    /// Callback invoked when a node's layout becomes dirty.
    dirtied_handler: Option<DirtiedCallback>,
    /// Per-node measure callbacks for custom measurement.
    measure_callbacks: HashMap<NodeId, MeasureCallback>,
    /// Layout configuration.
    config: LayoutConfig,
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
            dirtied_handler: None,
            measure_callbacks: HashMap::new(),
            config: LayoutConfig::default(),
        }
    }

    /// Set a callback invoked when any node's layout becomes dirty.
    pub fn set_dirtied_handler<F>(&mut self, handler: F)
    where
        F: Fn(NodeId) + Send + 'static,
    {
        self.dirtied_handler = Some(Box::new(handler));
    }

    /// Remove the dirtied callback.
    pub fn clear_dirtied_handler(&mut self) {
        self.dirtied_handler = None;
    }

    /// Set a custom measure callback for a specific node.
    /// When set, this callback is invoked during layout computation to
    /// determine the node's intrinsic size, instead of using text heuristics.
    pub fn set_measure_callback<F>(&mut self, id: NodeId, callback: F)
    where
        F: Fn(Option<f32>, Option<f32>, f32, f32) -> MeasureResult + Send + 'static,
    {
        self.measure_callbacks.insert(id, Box::new(callback));
    }

    /// Remove the measure callback for a node, reverting to default text-based measurement.
    pub fn remove_measure_callback(&mut self, id: NodeId) {
        self.measure_callbacks.remove(&id);
    }

    /// Check if a node has a custom measure callback registered.
    pub fn has_measure_callback(&self, id: NodeId) -> bool {
        self.measure_callbacks.contains_key(&id)
    }

    /// Get the current layout config.
    pub fn config(&self) -> &LayoutConfig {
        &self.config
    }

    /// Set the layout config.
    pub fn set_config(&mut self, config: LayoutConfig) {
        self.config = config;
    }

    /// Fire the dirtied callback if one is registered.
    fn fire_dirtied(&self, id: NodeId) {
        if let Some(ref handler) = self.dirtied_handler {
            handler(id);
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
            self.measure_callbacks.remove(&id);
            let _ = self.taffy.remove(taffy_id);
            self.fire_dirtied(id);
        }
    }

    pub fn update_style(&mut self, id: NodeId, props: &LayoutProps) {
        if let Some(&taffy_id) = self.node_map.get(&id) {
            let style = layout_props_to_taffy(props);
            self.taffy.set_style(taffy_id, style).unwrap();
            let _ = self.taffy.mark_dirty(taffy_id);
            self.fire_dirtied(id);
        }
    }

    /// Check if a node's layout needs recomputation.
    pub fn is_dirty(&self, id: NodeId) -> bool {
        if let Some(&taffy_id) = self.node_map.get(&id) { self.taffy.dirty(taffy_id).unwrap_or(false) } else { false }
    }

    /// Mark a node as needing layout recomputation.
    /// This also marks all ancestors as dirty.
    pub fn mark_dirty(&mut self, id: NodeId) {
        if let Some(&taffy_id) = self.node_map.get(&id) {
            let _ = self.taffy.mark_dirty(taffy_id);
            self.fire_dirtied(id);
        }
    }

    /// Check if a node has a freshly computed layout.
    /// Returns `true` if the node's layout was computed and is not dirty.
    pub fn has_new_layout(&self, id: NodeId) -> bool {
        if let Some(&taffy_id) = self.node_map.get(&id) {
            // After compute_layout, Taffy resets dirty flags.
            // A node that is not dirty and has a layout has "new" layout.
            !self.taffy.dirty(taffy_id).unwrap_or(true) && self.taffy.layout(taffy_id).is_ok()
        } else {
            false
        }
    }

    /// Reset a node to its default layout state, clearing any custom style, text, or callbacks.
    pub fn reset_node(&mut self, id: NodeId) {
        if let Some(&taffy_id) = self.node_map.get(&id) {
            self.taffy.set_style(taffy_id, taffy::Style::default()).unwrap();
            let _ = self.taffy.mark_dirty(taffy_id);
            self.text_map.borrow_mut().remove(&id);
            self.measure_callbacks.remove(&id);
            self.fire_dirtied(id);
        }
    }

    /// Copy the layout style from one node to another.
    /// Leaves the source node unchanged; only the target node is updated.
    pub fn copy_style(&mut self, from: NodeId, to: NodeId) {
        let from_taffy_id = match self.node_map.get(&from).copied() {
            Some(id) => id,
            None => return,
        };
        let style = match self.taffy.style(from_taffy_id) {
            Ok(s) => s.clone(),
            Err(_) => return,
        };
        if let Some(&to_taffy_id) = self.node_map.get(&to) {
            let _ = self.taffy.set_style(to_taffy_id, style);
            let _ = self.taffy.mark_dirty(to_taffy_id);
            self.fire_dirtied(to);
        }
    }

    /// Get the computed left edge position for a node.
    pub fn get_computed_left(&self, id: NodeId) -> f32 {
        if let Some(&taffy_id) = self.node_map.get(&id)
            && let Ok(layout) = self.taffy.layout(taffy_id)
        {
            return layout.location.x;
        }
        0.0
    }

    /// Get the computed top edge position for a node.
    pub fn get_computed_top(&self, id: NodeId) -> f32 {
        if let Some(&taffy_id) = self.node_map.get(&id)
            && let Ok(layout) = self.taffy.layout(taffy_id)
        {
            return layout.location.y;
        }
        0.0
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
        if let Some(&taffy_id) = self.node_map.get(&id) {
            let _ = self.taffy.mark_dirty(taffy_id);
            self.fire_dirtied(id);
        }
    }

    pub fn add_child(&mut self, parent: NodeId, child: NodeId) {
        let Some(&p) = self.node_map.get(&parent) else {
            return;
        };
        let Some(&c) = self.node_map.get(&child) else {
            return;
        };
        let _ = self.taffy.add_child(p, c);
    }

    pub fn set_children(&mut self, parent: NodeId, children: &[NodeId]) {
        let Some(&p) = self.node_map.get(&parent) else {
            return;
        };
        let taffy_children: Vec<_> = children.iter().filter_map(|c| self.node_map.get(c).copied()).collect();
        let _ = self.taffy.set_children(p, &taffy_children);
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        let Some(&p) = self.node_map.get(&parent) else {
            return;
        };
        let Some(&c) = self.node_map.get(&child) else {
            return;
        };
        let _ = self.taffy.remove_child(p, c);
    }

    pub fn compute_layout(&mut self, root: NodeId, width: f32, height: f32) -> Result<(), LayoutError> {
        let &taffy_root = self.node_map.get(&root).ok_or(LayoutError::NodeNotRegistered(root))?;

        if !self.taffy.dirty(taffy_root).unwrap_or(false) {
            return Ok(());
        }

        let size = taffy::Size {
            width: taffy::AvailableSpace::Definite(width),
            height: taffy::AvailableSpace::Definite(height),
        };
        let reverse_map = &self.reverse_map;
        let text_map = &self.text_map;
        let measure_callbacks = &self.measure_callbacks;

        self.taffy.compute_layout_with_measure(
            taffy_root,
            size,
            |known_dimensions, available_space, node_id, _context, _style| {
                if let taffy::Size { width: Some(w), height: Some(h) } = known_dimensions {
                    return taffy::Size { width: w, height: h };
                }

                let our_id = match reverse_map.get(&node_id) {
                    Some(id) => *id,
                    None => return taffy::Size::ZERO,
                };

                let available_width = match available_space.width {
                    taffy::AvailableSpace::Definite(w) => w,
                    taffy::AvailableSpace::MaxContent => f32::INFINITY,
                    taffy::AvailableSpace::MinContent => 0.0,
                };
                let available_height = match available_space.height {
                    taffy::AvailableSpace::Definite(h) => h,
                    taffy::AvailableSpace::MaxContent => f32::INFINITY,
                    taffy::AvailableSpace::MinContent => 0.0,
                };

                // Check for per-node measure callback first (OpenTUI native measure path)
                if let Some(callback) = measure_callbacks.get(&our_id) {
                    let result =
                        callback(known_dimensions.width, known_dimensions.height, available_width, available_height);
                    return taffy::Size { width: result.width, height: result.height };
                }

                // Fall back to text measurement (default behavior)
                let text = text_map.borrow();
                let content = match text.get(&our_id) {
                    Some(t) => t.as_str(),
                    None => return taffy::Size::ZERO,
                };

                let (intrinsic_width, line_count) = measure_text(content, available_width);

                taffy::Size {
                    width: known_dimensions.width.unwrap_or(intrinsic_width),
                    height: known_dimensions.height.unwrap_or(line_count as f32),
                }
            },
        )?;
        Ok(())
    }

    /// Force layout computation regardless of dirty state.
    /// Use this when terminal size changes.
    pub fn compute_layout_forced(&mut self, root: NodeId, width: f32, height: f32) -> Result<(), LayoutError> {
        let &taffy_root = self.node_map.get(&root).ok_or(LayoutError::NodeNotRegistered(root))?;
        let _ = self.taffy.mark_dirty(taffy_root);
        self.compute_layout(root, width, height)
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
        top: r.top.map(taffy::LengthPercentage::Length).unwrap_or(taffy::LengthPercentage::Length(0.0)),
        right: r.right.map(taffy::LengthPercentage::Length).unwrap_or(taffy::LengthPercentage::Length(0.0)),
        bottom: r.bottom.map(taffy::LengthPercentage::Length).unwrap_or(taffy::LengthPercentage::Length(0.0)),
        left: r.left.map(taffy::LengthPercentage::Length).unwrap_or(taffy::LengthPercentage::Length(0.0)),
    }
}

fn rect_values_to_taffy_auto(r: &RectValues) -> taffy::Rect<taffy::LengthPercentageAuto> {
    taffy::Rect {
        top: r.top.map(taffy::LengthPercentageAuto::Length).unwrap_or(taffy::LengthPercentageAuto::Length(0.0)),
        right: r.right.map(taffy::LengthPercentageAuto::Length).unwrap_or(taffy::LengthPercentageAuto::Length(0.0)),
        bottom: r.bottom.map(taffy::LengthPercentageAuto::Length).unwrap_or(taffy::LengthPercentageAuto::Length(0.0)),
        left: r.left.map(taffy::LengthPercentageAuto::Length).unwrap_or(taffy::LengthPercentageAuto::Length(0.0)),
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
        top: r.top.map(taffy::LengthPercentageAuto::Length).unwrap_or(taffy::LengthPercentageAuto::Auto),
        right: r.right.map(taffy::LengthPercentageAuto::Length).unwrap_or(taffy::LengthPercentageAuto::Auto),
        bottom: r.bottom.map(taffy::LengthPercentageAuto::Length).unwrap_or(taffy::LengthPercentageAuto::Auto),
        left: r.left.map(taffy::LengthPercentageAuto::Length).unwrap_or(taffy::LengthPercentageAuto::Auto),
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
        Position::Static => taffy::Position::Relative, // Taffy has no Static; Relative is closest
    }
}

fn map_layout_overflow(val: LayoutOverflow) -> taffy::Overflow {
    match val {
        LayoutOverflow::Visible => taffy::Overflow::Visible,
        LayoutOverflow::Hidden => taffy::Overflow::Hidden,
        LayoutOverflow::Scroll => taffy::Overflow::Scroll,
    }
}

fn map_box_sizing(val: BoxSizing) -> taffy::BoxSizing {
    match val {
        BoxSizing::BorderBox => taffy::BoxSizing::BorderBox,
        BoxSizing::ContentBox => taffy::BoxSizing::ContentBox,
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
    let padding = props.padding.map(|r| rect_values_to_taffy(&r)).unwrap_or(taffy::Rect {
        top: taffy::LengthPercentage::Length(0.0),
        right: taffy::LengthPercentage::Length(0.0),
        bottom: taffy::LengthPercentage::Length(0.0),
        left: taffy::LengthPercentage::Length(0.0),
    });
    let margin = props.margin.map(|r| rect_values_to_taffy_auto(&r)).unwrap_or(taffy::Rect {
        top: taffy::LengthPercentageAuto::Length(0.0),
        right: taffy::LengthPercentageAuto::Length(0.0),
        bottom: taffy::LengthPercentageAuto::Length(0.0),
        left: taffy::LengthPercentageAuto::Length(0.0),
    });
    let border = props.border.map(|r| rect_values_to_taffy(&r)).unwrap_or(taffy::Rect {
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
        None => {
            taffy::Size { width: taffy::LengthPercentage::Length(0.0), height: taffy::LengthPercentage::Length(0.0) }
        }
    };

    let size = taffy::Size { width: sizing_to_taffy(props.width), height: sizing_to_taffy(props.height) };

    taffy::Style {
        display: match props.display {
            types::Display::Flex => taffy::Display::Flex,
            types::Display::None => taffy::Display::None,
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
        min_size: taffy::Size { width: sizing_to_taffy(props.min_width), height: sizing_to_taffy(props.min_height) },
        max_size: taffy::Size { width: sizing_to_taffy(props.max_width), height: sizing_to_taffy(props.max_height) },
        inset: props.inset.map(|r| rect_values_to_inset(&r)).unwrap_or(taffy::Rect {
            top: taffy::LengthPercentageAuto::Auto,
            right: taffy::LengthPercentageAuto::Auto,
            bottom: taffy::LengthPercentageAuto::Auto,
            left: taffy::LengthPercentageAuto::Auto,
        }),
        padding,
        margin,
        border,
        gap,
        aspect_ratio: props.aspect_ratio,
        overflow: taffy::Point {
            x: props.overflow.map(map_layout_overflow).unwrap_or(taffy::Overflow::Visible),
            y: props.overflow.map(map_layout_overflow).unwrap_or(taffy::Overflow::Visible),
        },
        box_sizing: props.box_sizing.map(map_box_sizing).unwrap_or(taffy::BoxSizing::BorderBox),
        ..Default::default()
    }
}

pub struct LayoutTreeSync {
    layout: LayoutEngine,
    results: HashMap<NodeId, LayoutResult>,
    /// Generation counter incremented on each layout computation.
    generation: u64,
    /// Revision counter for structural changes (child add/remove, visibility, etc.)
    revision: u64,
}

impl Default for LayoutTreeSync {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutTreeSync {
    pub fn new() -> Self {
        Self { layout: LayoutEngine::new(), results: HashMap::new(), generation: 0, revision: 0 }
    }

    /// Get current layout generation.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Get current render list revision.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Bump revision for structural changes (child add/remove, visibility change, etc.)
    pub fn bump_revision(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn sync_full(&mut self, arena: &NodeArena) {
        for (id, node) in arena.iter() {
            if !self.layout.has_node(id) {
                if let Some(text) = &node.text {
                    self.layout.register_text_node(id, &node.layout, text);
                } else {
                    self.layout.register_container(id, &node.layout);
                }
            } else if node.state.layout_dirty {
                if let Some(text) = &node.text {
                    self.layout.update_text(id, text);
                }
                self.layout.update_style(id, &node.layout);
            }
        }
    }

    pub fn sync_node(&mut self, arena: &NodeArena, id: NodeId) {
        if let Some(node) = arena.get(id) {
            if !self.layout.has_node(id) {
                if let Some(text) = &node.text {
                    self.layout.register_text_node(id, &node.layout, text);
                } else {
                    self.layout.register_container(id, &node.layout);
                }
            } else if node.state.layout_dirty {
                if let Some(text) = &node.text {
                    self.layout.update_text(id, text);
                }
                self.layout.update_style(id, &node.layout);
            }
        }
    }

    pub fn remove_node(&mut self, id: NodeId) {
        self.layout.remove_node(id);
        self.bump_revision();
    }

    pub fn sync_children(&mut self, arena: &NodeArena, parent: NodeId) {
        let children = arena.children(parent);
        let had_changes = !children.is_empty();
        for child in &children {
            self.sync_node(arena, *child);
        }
        self.layout.set_children(parent, &children);
        if had_changes {
            self.bump_revision();
        }
    }

    pub fn compute(&mut self, root: NodeId, width: u16, height: u16) -> Result<(), LayoutError> {
        if !self.layout.is_dirty(root) {
            return Ok(());
        }
        self.layout.compute_layout(root, width as f32, height as f32)?;
        self.results = self.layout.collect_results();
        self.generation = self.generation.wrapping_add(1);
        Ok(())
    }

    /// Force layout computation regardless of dirty state.
    pub fn compute_forced(&mut self, root: NodeId, width: u16, height: u16) -> Result<(), LayoutError> {
        self.layout.mark_dirty(root);
        self.compute(root, width, height)
    }

    pub fn results(&self) -> &HashMap<NodeId, LayoutResult> {
        &self.results
    }

    pub fn node_count(&self) -> usize {
        self.layout.node_count()
    }
}

/// Resolved layout for a single node after layout computation.
///
/// Stores the final position and size in terminal cell coordinates.
/// All values are integers — fractional Taffy output is rounded at the last step.
///
/// **Memory:** 32 bytes per node. Stack-allocated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LayoutResult {
    /// Absolute X position in terminal cells (from left edge).
    pub x: u16,
    /// Absolute Y position in terminal cells (from top edge).
    pub y: u16,
    /// Outer width including padding and border (in cells).
    /// Clamped to minimum 1 for terminal rendering.
    pub width: u16,
    /// Outer height including padding and border (in cells).
    /// Clamped to minimum 1 for terminal rendering.
    pub height: u16,
    /// Inner width excluding padding and border (in cells).
    pub content_width: u16,
    /// Inner height excluding padding and border (in cells).
    pub content_height: u16,
    /// Computed padding from Taffy (resolved from percentages to concrete values).
    pub padding_top: u16,
    pub padding_right: u16,
    pub padding_bottom: u16,
    pub padding_left: u16,
    /// Computed border from Taffy.
    pub border_top: u16,
    pub border_right: u16,
    pub border_bottom: u16,
    pub border_left: u16,
}

impl LayoutResult {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height, content_width: width, content_height: height, ..Default::default() }
    }

    /// Convert Taffy's f32 pixel output to terminal cell count.
    /// In BetterTUI, 1 "pixel" = 1 terminal cell.
    pub fn pixels_to_cells(pixels: f32) -> u16 {
        (pixels.round() as i32).max(0) as u16
    }

    /// Returns the bounding rectangle for this layout.
    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    /// Returns the content rectangle (excluding padding/border).
    pub fn content_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.content_width, self.content_height)
    }

    /// Returns the right edge (x + width).
    pub fn right(&self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the bottom edge (y + height).
    pub fn bottom(&self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Check if a point is within this layout's bounds.
    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }
}

// ============================================================================
// BUILD
// ============================================================================

/// Minimum children to trigger binary search culling for scroll containers.
const BINARY_SEARCH_MIN_CHILDREN: usize = 32;

pub fn build_render_tree(arena: &NodeArena, layout_results: &HashMap<NodeId, LayoutResult>, tree: &mut RenderTree) {
    build_render_tree_with_viewport(arena, layout_results, None, tree)
}

pub fn build_render_tree_with_viewport(
    arena: &NodeArena,
    layout_results: &HashMap<NodeId, LayoutResult>,
    viewport: Option<&Viewport>,
    tree: &mut RenderTree,
) {
    tree.clear();
    let root = arena.root();
    build_node(arena, layout_results, root, &crate::tree::Style::default(), 0, 0, 1.0, 0, 0, viewport, tree);
}

#[allow(clippy::too_many_arguments)]
fn build_node(
    arena: &NodeArena,
    layout_results: &HashMap<NodeId, LayoutResult>,
    id: NodeId,
    parent_style: &crate::tree::Style,
    clip_x: u16,
    clip_y: u16,
    parent_opacity: f32,
    accum_tx: i32,
    accum_ty: i32,
    viewport: Option<&Viewport>,
    tree: &mut RenderTree,
) {
    let node = match arena.get(id) {
        Some(n) => n,
        None => return,
    };

    if node.visibility.display == crate::tree::Display::None {
        return;
    }

    let opacity = parent_opacity * node.visibility.opacity;

    if opacity == 0.0 {
        return;
    }

    let layout = layout_results.get(&id).cloned().unwrap_or_default();

    let (_current_viewport, child_viewport) = match viewport {
        None => (None, None),
        Some(vp) => {
            let narrowed = if flags_need_clip(node) {
                vp.intersect(&Viewport::new(layout.x, layout.y, layout.width, layout.height))
            } else {
                Some(*vp)
            };

            match narrowed {
                None => return, // outside clip → cull entire subtree
                Some(nv) => {
                    if !nv.contains_rect(layout.x, layout.y, layout.width, layout.height) {
                        return; // outside viewport → cull subtree
                    }
                    let cv = if node.overflow == Overflow::Scroll {
                        nv.offset(node.state.scroll_x, node.state.scroll_y)
                    } else {
                        nv
                    };
                    (Some(nv), Some(cv))
                }
            }
        }
    };

    let resolved_style = node.style.resolve(parent_style);

    let mut flags = PaintFlags::empty();
    if resolved_style.bg.is_some() {
        flags |= PaintFlags::BACKGROUND;
    }
    if resolved_style.border_style != crate::tree::BorderStyle::None {
        flags |= PaintFlags::BORDER;
    }
    if node.text.is_some() {
        flags |= PaintFlags::TEXT;
    }
    if node.overflow == Overflow::Hidden || node.overflow == Overflow::Scroll {
        flags |= PaintFlags::NEEDS_CLIP;
    }
    if !node.visibility.clip && node.overflow == Overflow::Visible {
        // no clip needed
    } else if node.visibility.clip {
        flags |= PaintFlags::NEEDS_CLIP;
    }

    let mut bounds = PaintBounds::new(layout.x, layout.y, layout.width.max(1), layout.height.max(1));
    bounds = bounds.with_padding(layout.padding_left, layout.padding_right, layout.padding_top, layout.padding_bottom);
    bounds = bounds.with_border(layout.border_top, layout.border_right, layout.border_bottom, layout.border_left);

    let clip = if flags.contains(PaintFlags::NEEDS_CLIP) {
        Some(ClipBounds::new(layout.x, layout.y, layout.width, layout.height))
    } else {
        None
    };

    let mut obj = RenderObject::new(id);
    obj.bounds = bounds;
    obj.clip = clip;
    obj.style = resolved_style;
    obj.opacity = opacity;
    obj.z_index = node.transform.z_index;
    obj.translate_x = accum_tx + node.transform.translate_x;
    obj.translate_y = accum_ty + node.transform.translate_y;
    obj.text = node.text.clone();
    obj.text_align = resolved_style.text_align;
    obj.text_wrap = node.text_wrap;
    obj.overflow = node.overflow;
    obj.flags = flags;

    tree.push(obj);

    let child_clip_x = if flags.contains(PaintFlags::NEEDS_CLIP) { layout.x } else { clip_x };
    let child_clip_y = if flags.contains(PaintFlags::NEEDS_CLIP) { layout.y } else { clip_y };

    let child_ids: Vec<NodeId> = match child_viewport {
        Some(ref vp) if node.overflow == Overflow::Scroll && node.children.len() >= BINARY_SEARCH_MIN_CHILDREN => {
            let primary = determine_primary_axis(&node.layout);
            let mut positioned: Vec<PositionedChild> = node
                .children
                .iter()
                .filter_map(|&cid| {
                    let layout = layout_results.get(&cid)?;
                    let (start, size) = match primary {
                        PrimaryAxis::Column => (layout.y, layout.height),
                        PrimaryAxis::Row => (layout.x, layout.width),
                    };
                    Some(PositionedChild { id: cid, start, size })
                })
                .collect();
            positioned.sort_by_key(|c| c.start);
            get_objects_in_viewport(vp, &positioned, primary)
        }
        _ => {
            let mut children: Vec<NodeId> = node.children.iter().copied().collect();
            children.sort_by_key(|&cid| arena.get(cid).map(|n| n.transform.z_index).unwrap_or(0));
            children
        }
    };

    let child_accum_tx = accum_tx + layout.x as i32 - node.state.scroll_x + node.transform.translate_x;
    let child_accum_ty = accum_ty + layout.y as i32 - node.state.scroll_y + node.transform.translate_y;

    for &child_id in &child_ids {
        build_node(
            arena,
            layout_results,
            child_id,
            &node.style,
            child_clip_x,
            child_clip_y,
            opacity,
            child_accum_tx,
            child_accum_ty,
            child_viewport.as_ref(),
            tree,
        );
    }
}

fn flags_need_clip(node: &crate::tree::RenderNode) -> bool {
    node.overflow == Overflow::Hidden || node.overflow == Overflow::Scroll || node.visibility.clip
}

fn determine_primary_axis(layout: &LayoutProps) -> PrimaryAxis {
    match layout.direction {
        FlexDirection::Row | FlexDirection::RowReverse => PrimaryAxis::Row,
        FlexDirection::Column | FlexDirection::ColumnReverse => PrimaryAxis::Column,
    }
}

#[cfg(test)]
mod measure_tests {
    use super::*;

    #[test]
    fn measure_empty_text() {
        let (width, lines) = measure_text("", f32::INFINITY);
        assert_eq!(width, 1.0);
        assert_eq!(lines, 1);
    }

    #[test]
    fn measure_ascii_text() {
        let (width, lines) = measure_text("hello", f32::INFINITY);
        assert_eq!(width, 5.0);
        assert_eq!(lines, 1);
    }

    #[test]
    fn measure_multiline_text() {
        let (width, lines) = measure_text("hello\nworld", f32::INFINITY);
        assert_eq!(width, 5.0);
        assert_eq!(lines, 2);
    }

    #[test]
    fn measure_cjk_text() {
        let (width, lines) = measure_text("\u{4e2d}\u{6587}", f32::INFINITY);
        assert_eq!(width, 4.0);
        assert_eq!(lines, 1);
    }

    #[test]
    fn measure_emoji_text() {
        let (width, lines) = measure_text("\u{1F600}", f32::INFINITY);
        assert_eq!(width, 2.0);
        assert_eq!(lines, 1);
    }

    #[test]
    fn measure_with_wrap() {
        let (width, lines) = measure_text("hello world", 6.0);
        assert_eq!(width, 6.0);
        assert_eq!(lines, 2);
    }

    #[test]
    fn count_lines_no_wrap() {
        assert_eq!(count_wrapped_lines("hello", 10), 1);
    }

    #[test]
    fn count_lines_simple_wrap() {
        assert_eq!(count_wrapped_lines("hello world", 5), 3);
    }

    #[test]
    fn count_lines_cjk_wrap() {
        let line = "\u{4e2d}\u{6587}\u{4e2d}\u{6587}\u{4e2d}\u{6587}";
        assert_eq!(count_wrapped_lines(line, 4), 4);
    }

    #[test]
    fn measure_text_long_word() {
        let (width, lines) = measure_text("supercalifragilisticexpialidocious", 10.0);
        assert_eq!(width, 10.0);
        assert!(lines >= 4);
    }

    #[test]
    fn measure_respects_available_width() {
        let (width, _) = measure_text("short", 100.0);
        assert_eq!(width, 5.0);
    }

    #[test]
    fn measure_max_content() {
        let (width, lines) = measure_text("hello\nworld\ntest", f32::INFINITY);
        assert_eq!(width, 5.0);
        assert_eq!(lines, 3);
    }
}

#[cfg(test)]
mod generation_tests {
    use super::*;

    #[test]
    fn layout_tree_sync_generation() {
        let sync = LayoutTreeSync::new();
        assert_eq!(sync.generation(), 0);
        assert_eq!(sync.revision(), 0);
    }
}

#[cfg(test)]
mod dirtied_callback_tests {
    use super::*;
    use crate::tree::NodeArena;

    #[test]
    fn dirtied_handler_fires_on_update_style() {
        let mut engine = LayoutEngine::new();
        let mut arena = NodeArena::new();
        let id = arena.insert(crate::tree::RenderNode::new(crate::tree::NodeKind::Box));
        engine.register_container(id, &LayoutProps::default());

        let fired = Arc::new(AtomicBool::new(false));
        let fired_clone = fired.clone();
        let fired_id = Arc::new(Mutex::new(NodeId::default()));
        let fired_id_clone = fired_id.clone();
        engine.set_dirtied_handler(move |nid| {
            fired_clone.store(true, Ordering::SeqCst);
            *fired_id_clone.lock().unwrap() = nid;
        });

        engine.update_style(id, &LayoutProps { flex_grow: 2.0, ..Default::default() });
        assert!(fired.load(Ordering::SeqCst));
        assert_eq!(*fired_id.lock().unwrap(), id);
    }

    #[test]
    fn dirtied_handler_fires_on_mark_dirty() {
        let mut engine = LayoutEngine::new();
        let mut arena = NodeArena::new();
        let id = arena.insert(crate::tree::RenderNode::new(crate::tree::NodeKind::Box));
        engine.register_container(id, &LayoutProps::default());

        let fired = Arc::new(AtomicBool::new(false));
        let f = fired.clone();
        engine.set_dirtied_handler(move |_| {
            f.store(true, Ordering::SeqCst);
        });
        engine.mark_dirty(id);
        assert!(fired.load(Ordering::SeqCst));
    }

    #[test]
    fn dirtied_handler_fires_on_remove_node() {
        let mut engine = LayoutEngine::new();
        let mut arena = NodeArena::new();
        let id = arena.insert(crate::tree::RenderNode::new(crate::tree::NodeKind::Box));
        engine.register_container(id, &LayoutProps::default());

        let fired = Arc::new(AtomicBool::new(false));
        let f = fired.clone();
        engine.set_dirtied_handler(move |_| {
            f.store(true, Ordering::SeqCst);
        });
        engine.remove_node(id);
        assert!(fired.load(Ordering::SeqCst));
    }

    #[test]
    fn dirtied_handler_fires_on_update_text() {
        let mut engine = LayoutEngine::new();
        engine.register_text_node(NodeId::default(), &LayoutProps::default(), "hello");

        let fired = Arc::new(AtomicBool::new(false));
        let f = fired.clone();
        engine.set_dirtied_handler(move |_| {
            f.store(true, Ordering::SeqCst);
        });
        engine.update_text(NodeId::default(), "world");
        assert!(fired.load(Ordering::SeqCst));
    }

    #[test]
    fn clear_dirtied_handler_stops_firing() {
        let mut engine = LayoutEngine::new();
        let mut arena = NodeArena::new();
        let id = arena.insert(crate::tree::RenderNode::new(crate::tree::NodeKind::Box));
        engine.register_container(id, &LayoutProps::default());

        let fired = Arc::new(AtomicBool::new(false));
        let f = fired.clone();
        engine.set_dirtied_handler(move |_| {
            f.store(true, Ordering::SeqCst);
        });
        engine.clear_dirtied_handler();
        engine.mark_dirty(id);
        assert!(!fired.load(Ordering::SeqCst));
    }

    #[test]
    fn dirtied_handler_fires_on_reset_node() {
        let mut engine = LayoutEngine::new();
        let mut arena = NodeArena::new();
        let id = arena.insert(crate::tree::RenderNode::new(crate::tree::NodeKind::Box));
        engine.register_container(id, &LayoutProps::default());

        let fired = Arc::new(AtomicBool::new(false));
        let f = fired.clone();
        engine.set_dirtied_handler(move |_| {
            f.store(true, Ordering::SeqCst);
        });
        engine.reset_node(id);
        assert!(fired.load(Ordering::SeqCst));
    }
}

#[cfg(test)]
mod measure_callback_tests {
    use super::*;

    #[test]
    fn set_and_has_measure_callback() {
        let mut engine = LayoutEngine::new();
        let id = NodeId::default();
        assert!(!engine.has_measure_callback(id));

        engine.set_measure_callback(id, |_, _, _, _| MeasureResult { width: 10.0, height: 5.0 });
        assert!(engine.has_measure_callback(id));
    }

    #[test]
    fn remove_measure_callback() {
        let mut engine = LayoutEngine::new();
        let id = NodeId::default();
        engine.set_measure_callback(id, |_, _, _, _| MeasureResult { width: 10.0, height: 5.0 });
        assert!(engine.has_measure_callback(id));

        engine.remove_measure_callback(id);
        assert!(!engine.has_measure_callback(id));
    }

    #[test]
    fn measure_callback_invoked_during_compute() {
        let mut engine = LayoutEngine::new();
        let id = NodeId::default();
        engine.register_text_node(
            id,
            &LayoutProps { width: Some(Sizing::Points(50.0)), height: Some(Sizing::Auto), ..Default::default() },
            "hello",
        );

        engine.set_measure_callback(id, |known_w, known_h, _avail_w, _avail_h| MeasureResult {
            width: known_w.unwrap_or(20.0),
            height: known_h.unwrap_or(10.0),
        });

        // Force dirty and compute
        engine.mark_dirty(id);
        engine.compute_layout(id, 80.0, 24.0).unwrap();

        // Callback returned width=50 (known) and height=10 (default)
        let results = engine.collect_results();
        let result = results.get(&id).unwrap();
        assert_eq!(result.width, 50);
        assert_eq!(result.height, 10);
    }

    #[test]
    fn measure_callback_overrides_text_measurement() {
        let mut engine = LayoutEngine::new();
        let id = NodeId::default();

        // Register with text, but override with callback.
        // Use Auto sizing so the callback fully controls the result.
        engine.register_text_node(
            id,
            &LayoutProps { width: Some(Sizing::Auto), height: Some(Sizing::Auto), ..Default::default() },
            "this is long text that would measure wide",
        );

        engine.set_measure_callback(id, |_, _, _, _| MeasureResult { width: 5.0, height: 1.0 });
        engine.mark_dirty(id);
        engine.compute_layout(id, 80.0, 24.0).unwrap();

        let results = engine.collect_results();
        let result = results.get(&id).unwrap();
        // Callback returned (5, 1), so width should be 5, height 1
        assert_eq!(result.width, 5);
        assert_eq!(result.height, 1);
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = LayoutConfig::default();
        assert_eq!(config.point_scale_factor, 1.0);
        assert!(!config.use_web_defaults);
    }

    #[test]
    fn set_and_get_config() {
        let mut engine = LayoutEngine::new();
        let config = LayoutConfig { point_scale_factor: 2.0, use_web_defaults: true };
        engine.set_config(config);
        assert_eq!(engine.config().point_scale_factor, 2.0);
        assert!(engine.config().use_web_defaults);
    }
}

#[cfg(test)]
mod new_style_props_tests {
    use super::*;

    #[test]
    fn layout_props_default_aspect_ratio() {
        let props = LayoutProps::default();
        assert_eq!(props.aspect_ratio, None);
    }

    #[test]
    fn layout_props_aspect_ratio_set() {
        let props = LayoutProps { aspect_ratio: Some(2.0), ..Default::default() };
        assert_eq!(props.aspect_ratio, Some(2.0));
    }

    #[test]
    fn position_static_variant() {
        let pos = Position::Static;
        match pos {
            Position::Static => {}
            _ => panic!("expected Static"),
        }
    }

    #[test]
    fn layout_overflow_default() {
        assert_eq!(LayoutOverflow::default(), LayoutOverflow::Visible);
    }

    #[test]
    fn box_sizing_default() {
        assert_eq!(BoxSizing::default(), BoxSizing::ContentBox);
    }

    #[test]
    fn map_layout_overflow_roundtrip() {
        let _ = map_layout_overflow(LayoutOverflow::Visible);
        let _ = map_layout_overflow(LayoutOverflow::Hidden);
        let _ = map_layout_overflow(LayoutOverflow::Scroll);
    }

    #[test]
    fn map_box_sizing_roundtrip() {
        let _ = map_box_sizing(BoxSizing::BorderBox);
        let _ = map_box_sizing(BoxSizing::ContentBox);
    }
}

#[cfg(test)]
mod node_operation_tests {
    use super::*;
    use crate::tree::NodeArena;

    #[test]
    fn has_new_layout_after_compute() {
        let mut engine = LayoutEngine::new();
        let id = NodeId::default();
        engine.register_container(
            id,
            &LayoutProps {
                width: Some(Sizing::Points(100.0)),
                height: Some(Sizing::Points(50.0)),
                ..Default::default()
            },
        );

        assert!(!engine.has_new_layout(id));
        engine.mark_dirty(id);
        engine.compute_layout(id, 80.0, 24.0).unwrap();
        assert!(engine.has_new_layout(id));
    }

    #[test]
    fn reset_node_clears_style() {
        let mut engine = LayoutEngine::new();
        let id = NodeId::default();
        engine.register_container(id, &LayoutProps { flex_grow: 2.0, ..Default::default() });
        engine.reset_node(id);

        engine.mark_dirty(id);
        engine.compute_layout(id, 80.0, 24.0).unwrap();

        // After reset, style should be default
        let results = engine.collect_results();
        let result = results.get(&id).unwrap();
        // Default style has flex_grow=0, so width/height should be 0 in 80x24
        assert!(result.width <= 80);
    }

    #[test]
    fn copy_style_transfers_props() {
        let mut engine = LayoutEngine::new();
        let id_from = NodeId::default();
        let id_to = {
            let mut arena = NodeArena::new();
            arena.insert(crate::tree::RenderNode::new(crate::tree::NodeKind::Box))
        };

        engine.register_container(
            id_from,
            &LayoutProps {
                flex_grow: 3.0,
                width: Some(Sizing::Points(200.0)),
                height: Some(Sizing::Points(100.0)),
                ..Default::default()
            },
        );
        engine.register_container(id_to, &LayoutProps::default());

        engine.copy_style(id_from, id_to);

        // Both should now have same layout positions
        engine.mark_dirty(id_from);
        engine.mark_dirty(id_to);
        engine.compute_layout(id_from, 80.0, 24.0).unwrap();
        engine.compute_layout(id_to, 80.0, 24.0).unwrap();

        let results = engine.collect_results();
        let from_result = results.get(&id_from).unwrap();
        let to_result = results.get(&id_to).unwrap();
        assert_eq!(from_result.width, to_result.width);
        assert_eq!(from_result.height, to_result.height);
    }

    #[test]
    fn get_computed_left_and_top() {
        let mut engine = LayoutEngine::new();
        let id = NodeId::default();
        engine.register_container(
            id,
            &LayoutProps {
                width: Some(Sizing::Points(100.0)),
                height: Some(Sizing::Points(50.0)),
                ..Default::default()
            },
        );
        engine.mark_dirty(id);
        engine.compute_layout(id, 80.0, 24.0).unwrap();

        // Left and top should be 0 for root
        assert_eq!(engine.get_computed_left(id), 0.0);
        assert_eq!(engine.get_computed_top(id), 0.0);
    }

    #[test]
    fn get_computed_edge_for_unregistered_node() {
        let engine = LayoutEngine::new();
        assert_eq!(engine.get_computed_left(NodeId::default()), 0.0);
        assert_eq!(engine.get_computed_top(NodeId::default()), 0.0);
    }
}
