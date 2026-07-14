//! Tree data structures: arena-backed node tree with layout, style, and visual properties.
//!
//! This module is the foundational data structure layer of the engine. It provides:
//!
//! - [`NodeId`] — A generational index (via `slotmap`) identifying a node in the arena.
//! - [`NodeArena`] — An arena allocator that owns all [`RenderNode`]s and manages parent-child links.
//! - [`RenderNode`] — A node in the UI tree with style, layout, text, focus, and metadata.
//! - [`Color`] / [`NamedColor`] — Terminal color representation (named, indexed, RGB).
//! - [`Style`] / [`ResolvedStyle`] — Cascading style properties with builder pattern.
//! - [`NodeKind`] — Enum of node types (Box, Text, Flex, etc.).
//!
//! # Architecture
//!
//! The tree uses a flat arena ([`NodeArena`]) with [`NodeId`] handles rather than
//! recursive pointer-based trees. This avoids ownership issues and enables O(1)
//! insertion, removal, and lookup. Each [`RenderNode`] stores a `parent` and
//! `children: SmallVec<[NodeId; 4]>` for tree traversal.
//!
//! # Example
//!
//! ```no_run
//! use bettertui_engine::tree::{NodeArena, NodeKind, RenderNode};
//!
//! let mut arena = NodeArena::new();
//! let root = arena.root();
//! let child = arena.insert(RenderNode::new(NodeKind::Text));
//! arena.append_child(root, child).unwrap();
//! ```

use std::collections::HashMap;
use std::fmt;

use bitflags::bitflags;
use slotmap::{DefaultKey, SlotMap};
use smallvec::SmallVec;

use crate::layout::LayoutProps;

// === node_id.rs ===

/// Uniquely identifies a node in the arena.
///
/// Uses generational indices via `slotmap::DefaultKey` to prevent use-after-free.
/// If a node is removed and a new node allocated at the same index,
/// the generation mismatch catches stale references.
///
/// Size: 8 bytes (two u32 values). Stack-allocated. O(1) comparison.
pub type NodeId = DefaultKey;

// === node_kind.rs ===
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

// === tree_error.rs ===

/// Errors that can occur during tree operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeError {
    /// Node with the given ID was not found in the arena.
    NodeNotFound(NodeId),
    /// A cycle would be created by this operation.
    CycleDetected { node: NodeId, ancestor: NodeId },
    /// The operation is invalid for some other reason.
    InvalidOperation(String),
}

impl fmt::Display for TreeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NodeNotFound(id) => write!(f, "Node not found: {id:?}"),
            Self::CycleDetected { node, ancestor } => {
                write!(
                    f,
                    "Cycle detected: node {node:?} is ancestor of {ancestor:?}"
                )
            }
            Self::InvalidOperation(msg) => write!(f, "Invalid operation: {msg}"),
        }
    }
}

impl std::error::Error for TreeError {}

// === color.rs ===
/// Represents a color with its intent.
///
/// Different terminals support different color modes. A color defined as
/// `Indexed(196)` should remain `Indexed(196)` even if the terminal
/// supports true color — this preserves theme portability. Only when
/// rendering do we resolve to the best available representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Color {
    Named(NamedColor),
    Indexed(u8),
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    #[default]
    Default,
}

/// Color intent preserves the original color space for rendering decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorIntent {
    /// Color is defined as RGB values
    Rgb,
    /// Color is an ANSI index (preserves palette slot)
    Indexed,
    /// Color is the terminal default
    Default,
}

/// RGBA color with alpha channel for compositing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    /// Create new RGBA color
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create RGB color (alpha = 255)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Parse from hex string (#RGB, #RGBA, #RRGGBB, #RRGGBBAA)
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
                Some(Self::rgb(r, g, b))
            }
            4 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
                let a = u8::from_str_radix(&hex[3..4], 16).ok()? * 17;
                Some(Self::new(r, g, b, a))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Self::new(r, g, b, a))
            }
            _ => None,
        }
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
        }
    }

    /// Linearly interpolate between two colors
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let inv_t = 1.0 - t;
        Self {
            r: (self.r as f32 * inv_t + other.r as f32 * t) as u8,
            g: (self.g as f32 * inv_t + other.g as f32 * t) as u8,
            b: (self.b as f32 * inv_t + other.b as f32 * t) as u8,
            a: (self.a as f32 * inv_t + other.a as f32 * t) as u8,
        }
    }

    /// Alpha blend this color over another
    pub fn blend_over(&self, background: &Self) -> Self {
        let alpha = self.a as f32 / 255.0;
        let inv_alpha = 1.0 - alpha;
        Self {
            r: (self.r as f32 * alpha + background.r as f32 * inv_alpha) as u8,
            g: (self.g as f32 * alpha + background.g as f32 * inv_alpha) as u8,
            b: (self.b as f32 * alpha + background.b as f32 * inv_alpha) as u8,
            a: 255,
        }
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Self::rgb(0, 0, 0)
    }
}

impl From<Rgba> for Color {
    fn from(rgba: Rgba) -> Self {
        Color::Rgb {
            r: rgba.r,
            g: rgba.g,
            b: rgba.b,
        }
    }
}

/// The 16 standard terminal colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    #[default]
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb { r, g, b }
    }

    /// Get the color intent (original color space)
    pub fn intent(&self) -> ColorIntent {
        match self {
            Self::Named(_) => ColorIntent::Rgb,
            Self::Indexed(_) => ColorIntent::Indexed,
            Self::Rgb { .. } => ColorIntent::Rgb,
            Self::Default => ColorIntent::Default,
        }
    }

    /// Parse color from string (hex, named, etc.)
    pub fn parse(s: &str) -> Option<Self> {
        // Try hex first
        if let Some(rgba) = Rgba::from_hex(s) {
            return Some(rgba.into());
        }

        // Try named colors
        match s.to_lowercase().as_str() {
            "black" => Some(Self::Named(NamedColor::Black)),
            "red" => Some(Self::Named(NamedColor::Red)),
            "green" => Some(Self::Named(NamedColor::Green)),
            "yellow" => Some(Self::Named(NamedColor::Yellow)),
            "blue" => Some(Self::Named(NamedColor::Blue)),
            "magenta" | "purple" => Some(Self::Named(NamedColor::Magenta)),
            "cyan" | "teal" => Some(Self::Named(NamedColor::Cyan)),
            "white" | "default" => Some(Self::Named(NamedColor::White)),
            "gray" | "grey" | "dark_gray" | "darkgrey" => {
                Some(Self::Named(NamedColor::BrightBlack))
            }
            "bright_red" | "light_red" => Some(Self::Named(NamedColor::BrightRed)),
            "bright_green" | "light_green" => Some(Self::Named(NamedColor::BrightGreen)),
            "bright_yellow" | "light_yellow" => Some(Self::Named(NamedColor::BrightYellow)),
            "bright_blue" | "light_blue" => Some(Self::Named(NamedColor::BrightBlue)),
            "bright_magenta" | "light_magenta" | "pink" => {
                Some(Self::Named(NamedColor::BrightMagenta))
            }
            "bright_cyan" | "light_cyan" => Some(Self::Named(NamedColor::BrightCyan)),
            "bright_white" | "light_gray" | "lightgrey" | "lightgray" => {
                Some(Self::Named(NamedColor::BrightWhite))
            }
            _ => None,
        }
    }

    /// Convert to RGBA (for compositing)
    pub fn to_rgba(&self, alpha: u8) -> Rgba {
        match self {
            Self::Named(named) => {
                let (r, g, b) = named.to_rgb();
                Rgba::new(r, g, b, alpha)
            }
            Self::Indexed(idx) => {
                let (r, g, b) = indexed_to_rgb(*idx);
                Rgba::new(r, g, b, alpha)
            }
            Self::Rgb { r, g, b } => Rgba::new(*r, *g, *b, alpha),
            Self::Default => Rgba::new(0, 0, 0, alpha),
        }
    }

    /// Linearly interpolate between two colors
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let c1 = self.to_rgba(255);
        let c2 = other.to_rgba(255);
        let blended = c1.lerp(&c2, t);
        Color::Rgb {
            r: blended.r,
            g: blended.g,
            b: blended.b,
        }
    }
}

impl NamedColor {
    /// Returns the ANSI color index (0-15) for this named color.
    pub fn ansi_index(&self) -> u8 {
        match self {
            Self::Black => 0,
            Self::Red => 1,
            Self::Green => 2,
            Self::Yellow => 3,
            Self::Blue => 4,
            Self::Magenta => 5,
            Self::Cyan => 6,
            Self::White => 7,
            Self::BrightBlack => 8,
            Self::BrightRed => 9,
            Self::BrightGreen => 10,
            Self::BrightYellow => 11,
            Self::BrightBlue => 12,
            Self::BrightMagenta => 13,
            Self::BrightCyan => 14,
            Self::BrightWhite => 15,
        }
    }

    /// Convert to RGB values
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Self::Black => (0, 0, 0),
            Self::Red => (170, 0, 0),
            Self::Green => (0, 170, 0),
            Self::Yellow => (170, 85, 0),
            Self::Blue => (0, 0, 170),
            Self::Magenta => (170, 0, 170),
            Self::Cyan => (0, 170, 170),
            Self::White => (170, 170, 170),
            Self::BrightBlack => (85, 85, 85),
            Self::BrightRed => (255, 85, 85),
            Self::BrightGreen => (85, 255, 85),
            Self::BrightYellow => (255, 255, 85),
            Self::BrightBlue => (85, 85, 255),
            Self::BrightMagenta => (255, 85, 255),
            Self::BrightCyan => (85, 255, 255),
            Self::BrightWhite => (255, 255, 255),
        }
    }

    /// Create from ANSI color index (0-15)
    pub fn from_ansi_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::Black),
            1 => Some(Self::Red),
            2 => Some(Self::Green),
            3 => Some(Self::Yellow),
            4 => Some(Self::Blue),
            5 => Some(Self::Magenta),
            6 => Some(Self::Cyan),
            7 => Some(Self::White),
            8 => Some(Self::BrightBlack),
            9 => Some(Self::BrightRed),
            10 => Some(Self::BrightGreen),
            11 => Some(Self::BrightYellow),
            12 => Some(Self::BrightBlue),
            13 => Some(Self::BrightMagenta),
            14 => Some(Self::BrightCyan),
            15 => Some(Self::BrightWhite),
            _ => None,
        }
    }
}

/// Convert ANSI 256 indexed color to RGB
#[doc(hidden)]
pub fn indexed_to_rgb(index: u8) -> (u8, u8, u8) {
    match index {
        0..=15 => NamedColor::from_ansi_index(index)
            .map(|c| c.to_rgb())
            .unwrap_or((0, 0, 0)),
        16..=231 => {
            // 6x6x6 color cube
            let idx = index - 16;
            let r = idx / 36;
            let g = (idx % 36) / 6;
            let b = idx % 6;
            let conv = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };
            (conv(r), conv(g), conv(b))
        }
        232..=255 => {
            // Grayscale ramp
            let gray = 8 + (index - 232) * 10;
            (gray, gray, gray)
        }
    }
}

// === style.rs ===

/// Border style for a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderStyle {
    #[default]
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
}

/// Visual styling for a node. Applied during rendering.
///
/// Uses `Option<bool>` instead of bitflags to allow style inheritance.
/// A child node can inherit its parent's `bold` value by having
/// `bold: None`. `Some(true)` or `Some(false)` overrides the parent.
///
/// Size: ~32 bytes. Stack-allocated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub underline_color: Option<Color>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub dim: Option<bool>,
    pub strikethrough: Option<bool>,
    pub inverse: Option<bool>,
    pub hidden: Option<bool>,
    pub grid_columns: Option<u16>,
    pub grid_rows: Option<u16>,
    pub border_style: Option<BorderStyle>,
    pub border_color: Option<Color>,
    pub border_width: Option<u16>,
    pub rounded_corners: Option<bool>,
    pub overflow: Option<Overflow>,
    pub opacity: Option<u8>,
}

impl Style {
    /// Returns a new style with all fields set to None (fully inheritable).
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if this style has no explicit values (all None).
    pub fn is_empty(&self) -> bool {
        self.fg.is_none()
            && self.bg.is_none()
            && self.underline_color.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.underline.is_none()
            && self.dim.is_none()
            && self.strikethrough.is_none()
            && self.inverse.is_none()
            && self.hidden.is_none()
            && self.grid_columns.is_none()
            && self.grid_rows.is_none()
            && self.border_style.is_none()
            && self.border_color.is_none()
            && self.border_width.is_none()
            && self.rounded_corners.is_none()
            && self.overflow.is_none()
            && self.opacity.is_none()
    }

    /// Merges this style with a parent style. Self values take precedence.
    pub fn resolve(&self, parent: &Style) -> ResolvedStyle {
        ResolvedStyle {
            fg: self.fg.or(parent.fg),
            bg: self.bg.or(parent.bg),
            underline_color: self.underline_color.or(parent.underline_color),
            bold: self.bold.or(parent.bold).unwrap_or(false),
            italic: self.italic.or(parent.italic).unwrap_or(false),
            underline: self.underline.or(parent.underline).unwrap_or(false),
            dim: self.dim.or(parent.dim).unwrap_or(false),
            strikethrough: self.strikethrough.or(parent.strikethrough).unwrap_or(false),
            inverse: self.inverse.or(parent.inverse).unwrap_or(false),
            hidden: self.hidden.or(parent.hidden).unwrap_or(false),
            border_style: self
                .border_style
                .or(parent.border_style)
                .unwrap_or(BorderStyle::None),
            border_color: self.border_color.or(parent.border_color),
            border_width: self.border_width.or(parent.border_width).unwrap_or(0),
            rounded_corners: self
                .rounded_corners
                .or(parent.rounded_corners)
                .unwrap_or(false),
            overflow: self
                .overflow
                .or(parent.overflow)
                .unwrap_or(Overflow::Visible),
            opacity: self.opacity.or(parent.opacity).unwrap_or(255),
        }
    }

    /// Set foreground color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set bold text.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    /// Set italic text.
    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    /// Set underline text.
    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = Some(underline);
        self
    }

    /// Set border style and color.
    pub fn border(mut self, style: BorderStyle, color: Color) -> Self {
        self.border_style = Some(style);
        self.border_color = Some(color);
        self.border_width = Some(1);
        self
    }

    /// Set border width.
    pub fn border_width(mut self, width: u16) -> Self {
        self.border_width = Some(width);
        self
    }

    /// Set rounded corners.
    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded_corners = Some(rounded);
        self
    }

    /// Set overflow handling.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    /// Set opacity (0-255).
    pub fn opacity(mut self, opacity: u8) -> Self {
        self.opacity = Some(opacity);
        self
    }
}

/// Fully resolved style with no Option fields. Used during rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedStyle {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub underline_color: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
    pub strikethrough: bool,
    pub inverse: bool,
    pub hidden: bool,
    pub border_style: BorderStyle,
    pub border_color: Option<Color>,
    pub border_width: u16,
    pub rounded_corners: bool,
    pub overflow: Overflow,
    pub opacity: u8,
}

impl Default for ResolvedStyle {
    fn default() -> Self {
        Self {
            fg: None,
            bg: None,
            underline_color: None,
            bold: false,
            italic: false,
            underline: false,
            dim: false,
            strikethrough: false,
            inverse: false,
            hidden: false,
            border_style: BorderStyle::None,
            border_color: None,
            border_width: 0,
            rounded_corners: false,
            overflow: Overflow::Visible,
            opacity: 255,
        }
    }
}

// === visual.rs ===
/// Controls whether a node is rendered and how it affects layout.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Visibility {
    /// Display mode. `None` = node removed from layout entirely.
    pub display: Display,
    /// Opacity multiplied with parent opacity during rendering.
    pub opacity: f32,
    /// Whether to clip children that overflow the node's bounds.
    pub clip: bool,
}

impl Default for Visibility {
    fn default() -> Self {
        Self {
            display: Display::Flex,
            opacity: 1.0,
            clip: false,
        }
    }
}

/// Display mode for visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Display {
    #[default]
    Flex,
    None,
}

/// Visual offset and layer ordering without affecting layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Transform {
    /// Horizontal offset in terminal cells.
    pub translate_x: i32,
    /// Vertical offset in terminal cells.
    pub translate_y: i32,
    /// Layer ordering. Higher z-index renders on top.
    /// Equal z-index renders in tree order (depth-first).
    pub z_index: i32,
}

/// How content overflows the node's bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Overflow {
    /// Children render outside bounds (may overlap siblings).
    #[default]
    Visible,
    /// Children are clipped at bounds.
    Hidden,
    /// Children are clipped, scrollbar rendered, scroll offsets tracked.
    Scroll,
}

/// Cursor appearance and position for input nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CursorProps {
    pub style: CursorStyle,
    pub blink: bool,
    pub position: Option<Point>,
}

/// Terminal cursor style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorStyle {
    #[default]
    Block,
    Underline,
    Bar,
    None,
}

/// A 2D point in terminal cell coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// A 2D size in terminal cell units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

/// A rectangle in terminal cell coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns the right edge (x + width).
    pub fn right(&self) -> u16 {
        self.x + self.width
    }

    /// Returns the bottom edge (y + height).
    pub fn bottom(&self) -> u16 {
        self.y + self.height
    }

    /// Checks if a point is contained within this rectangle.
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x && point.x < self.right() && point.y >= self.y && point.y < self.bottom()
    }

    /// Checks if this rectangle intersects another.
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }
}

// === metadata.rs ===
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

// === interaction.rs ===

/// Focus properties for a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FocusProps {
    /// Tab order. `None` = follow tree order. `Some(n)` = explicit order (lower first).
    pub tab_index: Option<i32>,
    /// Whether this node can receive focus.
    pub focusable: bool,
    /// Whether this node currently has focus.
    pub focused: bool,
}

/// Mutable state of a node, updated by the engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeState {
    /// Horizontal scroll offset.
    pub scroll_x: i32,
    /// Vertical scroll offset.
    pub scroll_y: i32,
    /// Measured content width in cells.
    pub content_width: u32,
    /// Measured content height in cells.
    pub content_height: u32,
    /// Generic dirty flag for any change.
    pub dirty: bool,
    /// Layout needs recalculation.
    pub layout_dirty: bool,
    /// Render needs redraw.
    pub render_dirty: bool,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            scroll_x: 0,
            scroll_y: 0,
            content_width: 0,
            content_height: 0,
            dirty: true,
            layout_dirty: true,
            render_dirty: true,
        }
    }
}

impl NodeState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark node and propagate dirty flags upward.
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
        self.layout_dirty = true;
        self.render_dirty = true;
    }

    /// Mark only layout as dirty.
    pub fn mark_layout_dirty(&mut self) {
        self.dirty = true;
        self.layout_dirty = true;
        self.render_dirty = true;
    }

    /// Mark only render as dirty.
    pub fn mark_render_dirty(&mut self) {
        self.dirty = true;
        self.render_dirty = true;
    }

    /// Clear all dirty flags.
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
        self.layout_dirty = false;
        self.render_dirty = false;
    }
}

bitflags! {
    /// Flags indicating what changed on a node. Used for efficient dirty propagation.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UpdateFlags: u32 {
        const STYLE = 0b0000_0001;
        const LAYOUT = 0b0000_0010;
        const TEXT = 0b0000_0100;
        const CHILDREN = 0b0000_1000;
        const VISIBILITY = 0b0001_0000;
        const TRANSFORM = 0b0010_0000;
        const FOCUS = 0b0100_0000;
        const METADATA = 0b1000_0000;
        const ALL = 0b1111_1111;
    }
}

impl Default for UpdateFlags {
    fn default() -> Self {
        Self::empty()
    }
}

/// Event handler placeholders. Actual handler implementation comes in later phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EventHandlers {
    /// Whether this node has any event handlers registered.
    pub has_handlers: bool,
}

// === render_node.rs ===

/// The complete data for a single UI node. Stored in the arena, accessed by `NodeId`.
///
/// **Ownership:** Owned by the arena. The arena is the sole owner of all nodes.
/// References are by `NodeId`, not by pointer.
///
/// **Memory:** Approximately 256-320 bytes per node. The `SmallVec<[NodeId; 4]>`
/// stores up to 4 children inline (32 bytes) without heap allocation.
/// Most nodes have fewer than 4 children.
pub struct RenderNode {
    /// Unique identifier for this node.
    pub id: NodeId,
    /// Type of node (Text, Box, Flex, etc.).
    pub kind: NodeKind,
    /// Parent node. None for root.
    pub parent: Option<NodeId>,
    /// Child nodes. SmallVec stores up to 4 inline.
    pub children: SmallVec<[NodeId; 4]>,
    /// Visual styling.
    pub style: Style,
    /// Layout properties.
    pub layout: LayoutProps,
    /// Text content (for Text nodes).
    pub text: Option<Box<str>>,
    /// Visibility control.
    pub visibility: Visibility,
    /// Visual offset and layer ordering.
    pub transform: Transform,
    /// How content overflows.
    pub overflow: Overflow,
    /// Cursor appearance and position.
    pub cursor: Option<CursorProps>,
    /// Text alignment.
    pub text_align: crate::text::TextAlign,
    /// Whether text should wrap.
    pub text_wrap: bool,
    /// Focus properties.
    pub focus: FocusProps,
    /// Event handler placeholders.
    pub events: EventHandlers,
    /// Mutable node state.
    pub state: NodeState,
    /// Optional metadata.
    pub metadata: Option<Box<Metadata>>,
    /// Optional accessibility data.
    pub accessibility: Option<Box<Accessibility>>,
    /// Generic key-value attributes.
    pub attributes: HashMap<String, String>,
}

impl Default for RenderNode {
    fn default() -> Self {
        Self {
            id: NodeId::default(),
            kind: NodeKind::default(),
            parent: None,
            children: SmallVec::new(),
            style: Style::default(),
            layout: LayoutProps::default(),
            text: None,
            visibility: Visibility::default(),
            transform: Transform::default(),
            overflow: Overflow::default(),
            cursor: None,
            text_align: crate::text::TextAlign::Left,
            text_wrap: false,
            focus: FocusProps::default(),
            events: EventHandlers::default(),
            state: NodeState::default(),
            metadata: None,
            accessibility: None,
            attributes: HashMap::new(),
        }
    }
}

impl RenderNode {
    /// Create a new node with the given kind. ID is set by the arena.
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            ..Default::default()
        }
    }

    /// Create a new text node.
    pub fn text(content: impl Into<Box<str>>) -> Self {
        Self {
            kind: NodeKind::Text,
            text: Some(content.into()),
            ..Default::default()
        }
    }

    /// Create a new box/container node.
    pub fn box_node() -> Self {
        Self::new(NodeKind::Box)
    }

    /// Create a new flex container node.
    pub fn flex() -> Self {
        Self::new(NodeKind::Flex)
    }

    /// Returns true if this node has children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Returns the number of children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Returns true if this node is the root (no parent).
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Set text content on this node.
    pub fn set_text(&mut self, content: impl Into<Box<str>>) {
        self.text = Some(content.into());
        self.state.mark_render_dirty();
    }

    /// Set style on this node.
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
        self.state.mark_render_dirty();
    }

    /// Set layout properties on this node.
    pub fn set_layout(&mut self, layout: LayoutProps) {
        self.layout = layout;
        self.state.mark_layout_dirty();
    }

    /// Mark this node as focused.
    pub fn focus(&mut self) {
        self.focus.focused = true;
        self.state.mark_render_dirty();
    }

    /// Mark this node as unfocused.
    pub fn blur(&mut self) {
        self.focus.focused = false;
        self.state.mark_render_dirty();
    }

    /// Mark this node as focusable.
    pub fn set_focusable(&mut self, focusable: bool) {
        self.focus.focusable = focusable;
    }
}

// === arena.rs ===

/// Arena-allocated node storage backed by `slotmap::SlotMap`.
///
/// Provides O(1) insertion, O(1) access, O(1) removal.
/// Generational indices prevent use-after-free.
///
/// The arena maintains a **tree invariant**: every node has exactly one parent
/// (except root, which has none). Violations are caught at operation time.
pub struct NodeArena {
    nodes: SlotMap<NodeId, RenderNode>,
    root: NodeId,
    /// Incremented on every structural change (insert, remove, tree ops)
    generation: u64,
    /// Incremented on every change including property mutations via CommandProcessor
    change_count: u64,
}

impl Default for NodeArena {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeArena {
    /// Create a new arena with a root node.
    pub fn new() -> Self {
        let mut nodes = SlotMap::with_key();
        let root = nodes.insert(RenderNode {
            kind: NodeKind::Box,
            ..Default::default()
        });
        Self {
            nodes,
            root,
            generation: 0,
            change_count: 0,
        }
    }

    /// Mark arena as changed (for property mutations from CommandProcessor).
    pub fn mark_changed(&mut self) {
        self.change_count += 1;
    }

    /// Get the total number of changes since creation.
    pub fn change_count(&self) -> u64 {
        self.change_count
    }

    /// Insert a node into the arena. Returns its NodeId.
    pub fn insert(&mut self, node: RenderNode) -> NodeId {
        let id = self.nodes.insert(node);
        self.nodes[id].id = id;
        self.generation += 1;
        self.mark_changed();
        id
    }

    /// Get a reference to a node by ID.
    pub fn get(&self, id: NodeId) -> Option<&RenderNode> {
        self.nodes.get(id)
    }

    /// Get a mutable reference to a node by ID.
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut RenderNode> {
        self.nodes.get_mut(id)
    }

    /// Remove a node from the arena, returning it.
    pub fn remove(&mut self, id: NodeId) -> Option<RenderNode> {
        if id == self.root {
            return None;
        }
        self.generation += 1;
        self.mark_changed();
        self.nodes.remove(id)
    }

    /// Check if a node exists in the arena.
    pub fn contains(&self, id: NodeId) -> bool {
        self.nodes.contains_key(id)
    }

    /// Number of nodes in the arena (including root).
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true if the arena has no nodes (should never happen since root always exists).
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Clear dirty flags on all nodes.
    /// Call this after render() completes to reset per-node dirty tracking.
    pub fn clear_dirty_flags(&mut self) {
        for (_, node) in &mut self.nodes {
            node.state.clear_dirty();
        }
    }

    /// Remove all nodes from the arena, keeping only root.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.root = self.nodes.insert(RenderNode {
            kind: NodeKind::Box,
            ..Default::default()
        });
        self.nodes[self.root].id = self.root;
        self.generation += 1;
        self.mark_changed();
    }

    /// Get the root node ID.
    pub fn root(&self) -> NodeId {
        self.root
    }

    /// Get the current generation ( incremented on every mutation).
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Iterate all nodes as (NodeId, &RenderNode).
    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &RenderNode)> {
        self.nodes.iter()
    }

    /// Iterate all nodes mutably as (NodeId, &mut RenderNode).
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (NodeId, &mut RenderNode)> {
        self.nodes.iter_mut()
    }

    /// Get direct children of a node.
    pub fn children(&self, id: NodeId) -> SmallVec<[NodeId; 4]> {
        self.nodes
            .get(id)
            .map(|n| n.children.clone())
            .unwrap_or_default()
    }

    /// Get descendants of a node in DFS order.
    pub fn descendants(&self, id: NodeId) -> Vec<NodeId> {
        let mut result = Vec::new();
        self.descendants_recursive(id, &mut result);
        result
    }

    fn descendants_recursive(&self, id: NodeId, result: &mut Vec<NodeId>) {
        if let Some(node) = self.nodes.get(id) {
            for &child in &node.children {
                result.push(child);
                self.descendants_recursive(child, result);
            }
        }
    }

    /// Get ancestors of a node from parent to root.
    pub fn ancestors(&self, id: NodeId) -> Vec<NodeId> {
        let mut result = Vec::new();
        let mut current = id;
        while let Some(node) = self.nodes.get(current) {
            if let Some(parent) = node.parent {
                result.push(parent);
                current = parent;
            } else {
                break;
            }
        }
        result
    }

    /// Count all descendants of a node (recursive).
    pub fn descendant_count(&self, id: NodeId) -> usize {
        let mut count = 0;
        if let Some(node) = self.nodes.get(id) {
            for &child in &node.children {
                count += 1;
                count += self.descendant_count(child);
            }
        }
        count
    }

    /// Compute the depth of a node (root = 0).
    pub fn depth(&self, id: NodeId) -> u32 {
        let mut depth = 0;
        let mut current = id;
        while let Some(node) = self.nodes.get(current) {
            if let Some(parent) = node.parent {
                depth += 1;
                current = parent;
            } else {
                break;
            }
        }
        depth
    }

    /// Check if `ancestor` is an ancestor of `descendant`.
    pub fn is_ancestor(&self, ancestor: NodeId, descendant: NodeId) -> bool {
        let mut current = descendant;
        while let Some(node) = self.nodes.get(current) {
            if let Some(parent) = node.parent {
                if parent == ancestor {
                    return true;
                }
                current = parent;
            } else {
                break;
            }
        }
        false
    }

    // ─── Tree Operations ───────────────────────────────────────────

    /// Append a child to a parent node.
    pub fn append_child(&mut self, parent: NodeId, child: NodeId) -> Result<(), TreeError> {
        if !self.contains(parent) {
            return Err(TreeError::NodeNotFound(parent));
        }
        if !self.contains(child) {
            return Err(TreeError::NodeNotFound(child));
        }
        if child == self.root {
            return Err(TreeError::InvalidOperation(
                "Cannot append root as child".into(),
            ));
        }
        if self.is_ancestor(child, parent) {
            return Err(TreeError::CycleDetected {
                node: child,
                ancestor: parent,
            });
        }

        // Detach child from current parent if any
        if let Some(_current_parent) = self.nodes[child].parent {
            self.detach(child);
        }

        self.nodes[child].parent = Some(parent);
        self.nodes[parent].children.push(child);
        self.generation += 1;
        self.mark_changed();
        Ok(())
    }

    /// Insert a child before a reference node.
    pub fn insert_before(&mut self, reference: NodeId, child: NodeId) -> Result<(), TreeError> {
        if !self.contains(reference) {
            return Err(TreeError::NodeNotFound(reference));
        }
        if !self.contains(child) {
            return Err(TreeError::NodeNotFound(child));
        }
        if child == self.root {
            return Err(TreeError::InvalidOperation(
                "Cannot insert root as child".into(),
            ));
        }
        if self.is_ancestor(child, reference) {
            return Err(TreeError::CycleDetected {
                node: child,
                ancestor: reference,
            });
        }

        let parent = self.nodes[reference]
            .parent
            .ok_or(TreeError::InvalidOperation(
                "Reference node has no parent".into(),
            ))?;

        // Detach child from current parent if any
        if let Some(_current_parent) = self.nodes[child].parent {
            self.detach(child);
        }

        // Find the index of the reference node in parent's children
        if let Some(parent_node) = self.nodes.get_mut(parent) {
            if let Some(idx) = parent_node.children.iter().position(|&id| id == reference) {
                parent_node.children.insert(idx, child);
            } else {
                return Err(TreeError::InvalidOperation(
                    "Reference node not found in parent's children".into(),
                ));
            }
        }

        self.nodes[child].parent = Some(parent);
        self.generation += 1;
        self.mark_changed();
        Ok(())
    }

    /// Move a node to a new parent.
    pub fn move_node(&mut self, node: NodeId, new_parent: NodeId) -> Result<(), TreeError> {
        if !self.contains(node) {
            return Err(TreeError::NodeNotFound(node));
        }
        if !self.contains(new_parent) {
            return Err(TreeError::NodeNotFound(new_parent));
        }
        if node == self.root {
            return Err(TreeError::InvalidOperation("Cannot move root".into()));
        }
        if self.is_ancestor(node, new_parent) {
            return Err(TreeError::CycleDetected {
                node,
                ancestor: new_parent,
            });
        }

        // Detach from current parent
        self.detach(node);

        // Append to new parent
        self.append_child(new_parent, node)
    }

    /// Replace one node with another.
    pub fn replace_node(&mut self, old: NodeId, new: NodeId) -> Result<(), TreeError> {
        if !self.contains(old) {
            return Err(TreeError::NodeNotFound(old));
        }
        if !self.contains(new) {
            return Err(TreeError::NodeNotFound(new));
        }
        if old == self.root {
            return Err(TreeError::InvalidOperation("Cannot replace root".into()));
        }
        if new == self.root {
            return Err(TreeError::InvalidOperation(
                "Cannot replace with root".into(),
            ));
        }

        let parent = self.nodes[old]
            .parent
            .ok_or(TreeError::InvalidOperation("Old node has no parent".into()))?;

        // Move all children from old to new
        let old_children: SmallVec<[NodeId; 4]> = self.nodes[old].children.clone();
        for &child in &old_children {
            self.nodes[child].parent = Some(new);
            self.nodes[new].children.push(child);
        }
        self.nodes[old].children.clear();

        // Replace old with new in parent's children
        if let Some(parent_node) = self.nodes.get_mut(parent)
            && let Some(idx) = parent_node.children.iter().position(|&id| id == old)
        {
            parent_node.children[idx] = new;
        }

        self.nodes[new].parent = Some(parent);
        self.nodes.remove(old);
        self.generation += 1;
        self.mark_changed();
        Ok(())
    }

    /// Remove a node and all its descendants from the arena.
    pub fn remove_subtree(&mut self, id: NodeId) {
        if id == self.root {
            // Don't remove root, just clear children
            let children: SmallVec<[NodeId; 4]> = self.nodes[self.root].children.clone();
            for child in children {
                self.remove_subtree_recursive(child);
            }
            self.nodes[self.root].children.clear();
            return;
        }
        self.remove_subtree_recursive(id);
        self.generation += 1;
        self.mark_changed();
    }

    fn remove_subtree_recursive(&mut self, id: NodeId) {
        let children: SmallVec<[NodeId; 4]> = self
            .nodes
            .get(id)
            .map(|n| n.children.clone())
            .unwrap_or_default();
        for child in children {
            self.remove_subtree_recursive(child);
        }
        self.nodes.remove(id);
    }

    /// Detach a node from its parent (but keep in arena).
    pub fn detach(&mut self, id: NodeId) {
        if id == self.root {
            return;
        }

        let parent = match self.nodes.get(id).and_then(|n| n.parent) {
            Some(p) => p,
            None => return,
        };

        // Remove from parent's children
        if let Some(parent_node) = self.nodes.get_mut(parent) {
            parent_node.children.retain(|c| *c != id);
        }

        // Clear parent reference
        if let Some(node) = self.nodes.get_mut(id) {
            node.parent = None;
        }

        self.generation += 1;
        self.mark_changed();
    }

    /// Validate tree invariants. Returns Ok(()) if valid.
    pub fn validate(&self) -> Result<(), TreeError> {
        // Check root exists and has no parent
        let root = self
            .nodes
            .get(self.root)
            .ok_or(TreeError::NodeNotFound(self.root))?;
        if root.parent.is_some() {
            return Err(TreeError::InvalidOperation("Root has parent".into()));
        }

        // Check all nodes have consistent parent-child relationships
        for (id, node) in &self.nodes {
            if id == self.root {
                continue;
            }

            // Check parent exists
            let parent_id = node.parent.ok_or(TreeError::InvalidOperation(format!(
                "Non-root node {id:?} has no parent"
            )))?;

            if !self.contains(parent_id) {
                return Err(TreeError::InvalidOperation(format!(
                    "Node {id:?} references non-existent parent {parent_id:?}"
                )));
            }

            // Check this node is in parent's children
            let parent_node = &self.nodes[parent_id];
            if !parent_node.children.contains(&id) {
                return Err(TreeError::InvalidOperation(format!(
                    "Node {id:?} claims parent {parent_id:?} but is not in parent's children"
                )));
            }
        }

        // Check all children exist in arena
        for (id, node) in &self.nodes {
            for &child in &node.children {
                if !self.contains(child) {
                    return Err(TreeError::InvalidOperation(format!(
                        "Node {id:?} references non-existent child {child:?}"
                    )));
                }
            }
        }

        Ok(())
    }

    /// Print the tree in a human-readable format for debugging.
    pub fn print_tree(&self) -> String {
        let mut output = String::new();
        self.print_node(self.root, &mut output, "", true);
        output
    }

    fn print_node(&self, id: NodeId, output: &mut String, prefix: &str, is_last: bool) {
        if let Some(node) = self.nodes.get(id) {
            let connector = if is_last { "└── " } else { "├── " };
            let kind_name = node.kind.name();
            let text_preview = node
                .text
                .as_ref()
                .map(|t| format!(" \"{}\"", t))
                .unwrap_or_default();
            output.push_str(&format!("{prefix}{connector}{kind_name}{text_preview}\n"));

            let child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            let child_count = node.children.len();
            for (i, &child) in node.children.iter().enumerate() {
                self.print_node(child, output, &child_prefix, i == child_count - 1);
            }
        }
    }
}

impl fmt::Debug for NodeArena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeArena")
            .field("len", &self.len())
            .field("generation", &self.generation)
            .field("root", &self.root)
            .finish()
    }
}

/// Re-export `Display` under an alias to avoid confusion with `std::fmt::Display`.
pub use Display as VisibilityDisplay;
