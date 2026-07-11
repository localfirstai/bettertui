#![deny(clippy::all)]

use bettertui_engine::engine::Engine;
use bettertui_engine::events::EventBus;
use bettertui_engine::focus::FocusManager;
use bettertui_engine::renderer::Renderer;
use bettertui_engine::scheduler::Scheduler;
use bettertui_engine::text::TextEngine;
use bettertui_engine::tree::{Color, LayoutProps, NodeKind, Style};
use bettertui_engine::VERSION;
use napi_derive::napi;
use std::collections::HashMap;

// ─── NodeId Conversion ───────────────────────────────────────────────────────
// NodeId = slotmap::DefaultKey (8 bytes wrapping NonZeroU64)
// We encode as u64 for FFI via transmute (safe: both are 8-byte #[repr(transparent)])

fn node_id_to_u64(id: bettertui_engine::tree::NodeId) -> u64 {
    unsafe { std::mem::transmute(id) }
}

fn u64_to_node_id(val: u64) -> bettertui_engine::tree::NodeId {
    unsafe { std::mem::transmute(val) }
}

// ─── JSON Command Deserialization ─────────────────────────────────────────────

#[derive(serde::Deserialize)]
#[serde(tag = "type")]
enum CommandJson {
    CreateNode { id: u64, kind: String },
    RemoveNode { id: u64 },
    AppendChild { parent: u64, child: u64 },
    InsertBefore { reference: u64, child: u64 },
    MoveNode { node: u64, new_parent: u64 },
    ReplaceNode { old: u64, new: u64 },
    DetachNode { id: u64 },
    SetText { id: u64, text: String },
    SetStyle { id: u64, style: StyleJson },
    SetForeground { id: u64, color: ColorJson },
    SetBackground { id: u64, color: ColorJson },
    SetBold { id: u64, value: bool },
    SetItalic { id: u64, value: bool },
    SetUnderline { id: u64, value: bool },
    SetStrikethrough { id: u64, value: bool },
    SetDim { id: u64, value: bool },
    SetInverse { id: u64, value: bool },
    SetHidden { id: u64, value: bool },
    SetLayout { id: u64, layout: LayoutJson },
    SetFlexDirection { id: u64, direction: String },
    SetJustifyContent { id: u64, value: String },
    SetAlignItems { id: u64, value: String },
    SetWidth { id: u64, value: SizingJson },
    SetHeight { id: u64, value: SizingJson },
    SetPadding { id: u64, value: RectValuesJson },
    SetMargin { id: u64, value: RectValuesJson },
    SetGap { id: u64, value: GapJson },
    SetFlexGrow { id: u64, value: f32 },
    SetFlexShrink { id: u64, value: f32 },
    SetPosition { id: u64, value: String },
    SetDisplay { id: u64, value: String },
    SetOpacity { id: u64, value: f32 },
    SetClip { id: u64, value: bool },
    SetZIndex { id: u64, value: i32 },
    SetOverflow { id: u64, value: String },
    FocusNode { id: u64 },
    BlurNode { id: u64 },
    BeginFrame { frame_id: u64 },
    CommitFrame { frame_id: u64 },
    Invalidate { id: u64 },
    Shutdown,
}

#[derive(serde::Deserialize)]
struct StyleJson {
    #[serde(default)]
    fg: Option<ColorJson>,
    #[serde(default)]
    bg: Option<ColorJson>,
    #[serde(default)]
    bold: Option<bool>,
    #[serde(default)]
    italic: Option<bool>,
    #[serde(default)]
    underline: Option<bool>,
    #[serde(default)]
    strikethrough: Option<bool>,
    #[serde(default)]
    dim: Option<bool>,
    #[serde(default)]
    inverse: Option<bool>,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum ColorJson {
    Named(String),
    Rgb { r: u8, g: u8, b: u8 },
}

impl From<ColorJson> for Color {
    fn from(c: ColorJson) -> Self {
        match c {
            ColorJson::Named(name) => match name.to_lowercase().as_str() {
                "red" => Color::Named(bettertui_engine::tree::NamedColor::Red),
                "green" => Color::Named(bettertui_engine::tree::NamedColor::Green),
                "blue" => Color::Named(bettertui_engine::tree::NamedColor::Blue),
                "yellow" => Color::Named(bettertui_engine::tree::NamedColor::Yellow),
                "cyan" => Color::Named(bettertui_engine::tree::NamedColor::Cyan),
                "magenta" => Color::Named(bettertui_engine::tree::NamedColor::Magenta),
                "white" => Color::Named(bettertui_engine::tree::NamedColor::White),
                "black" => Color::Named(bettertui_engine::tree::NamedColor::Black),
                "dark_gray" | "darkgray" => {
                    Color::Named(bettertui_engine::tree::NamedColor::BrightBlack)
                }
                "light_gray" | "lightgray" => {
                    Color::Named(bettertui_engine::tree::NamedColor::BrightWhite)
                }
                "light_red" | "lightred" => {
                    Color::Named(bettertui_engine::tree::NamedColor::BrightRed)
                }
                "light_green" | "lightgreen" => {
                    Color::Named(bettertui_engine::tree::NamedColor::BrightGreen)
                }
                "light_blue" | "lightblue" => {
                    Color::Named(bettertui_engine::tree::NamedColor::BrightBlue)
                }
                "light_yellow" | "lightyellow" => {
                    Color::Named(bettertui_engine::tree::NamedColor::BrightYellow)
                }
                "light_cyan" | "lightcyan" => {
                    Color::Named(bettertui_engine::tree::NamedColor::BrightCyan)
                }
                "light_magenta" | "lightmagenta" => {
                    Color::Named(bettertui_engine::tree::NamedColor::BrightMagenta)
                }
                _ => Color::Named(bettertui_engine::tree::NamedColor::White),
            },
            ColorJson::Rgb { r, g, b } => Color::Rgb { r, g, b },
        }
    }
}

#[derive(serde::Deserialize)]
struct LayoutJson {
    #[serde(default)]
    direction: Option<String>,
    #[serde(default)]
    justify: Option<String>,
    #[serde(default)]
    align: Option<String>,
    #[serde(default)]
    padding: Option<RectValuesJson>,
    #[serde(default)]
    margin: Option<RectValuesJson>,
    #[serde(default)]
    gap: Option<GapJson>,
    #[serde(default)]
    flex_grow: Option<f32>,
    #[serde(default)]
    flex_shrink: Option<f32>,
}

impl From<LayoutJson> for LayoutProps {
    fn from(l: LayoutJson) -> Self {
        let mut props = LayoutProps::default();
        if let Some(dir) = l.direction {
            props.direction = match dir.as_str() {
                "row" | "Row" => bettertui_engine::tree::FlexDirection::Row,
                "column" | "Column" => bettertui_engine::tree::FlexDirection::Column,
                "row_reverse" | "RowReverse" => bettertui_engine::tree::FlexDirection::RowReverse,
                "column_reverse" | "ColumnReverse" => {
                    bettertui_engine::tree::FlexDirection::ColumnReverse
                }
                _ => bettertui_engine::tree::FlexDirection::Column,
            };
        }
        if let Some(j) = l.justify {
            props.justify = match j.as_str() {
                "flex_start" | "FlexStart" => bettertui_engine::tree::JustifyContent::FlexStart,
                "center" | "Center" => bettertui_engine::tree::JustifyContent::Center,
                "flex_end" | "FlexEnd" => bettertui_engine::tree::JustifyContent::FlexEnd,
                "space_between" | "SpaceBetween" => {
                    bettertui_engine::tree::JustifyContent::SpaceBetween
                }
                "space_around" | "SpaceAround" => {
                    bettertui_engine::tree::JustifyContent::SpaceAround
                }
                "space_evenly" | "SpaceEvenly" => {
                    bettertui_engine::tree::JustifyContent::SpaceEvenly
                }
                _ => bettertui_engine::tree::JustifyContent::FlexStart,
            };
        }
        if let Some(a) = l.align {
            props.align = match a.as_str() {
                "flex_start" | "FlexStart" => bettertui_engine::tree::AlignItems::FlexStart,
                "center" | "Center" => bettertui_engine::tree::AlignItems::Center,
                "flex_end" | "FlexEnd" => bettertui_engine::tree::AlignItems::FlexEnd,
                "stretch" | "Stretch" => bettertui_engine::tree::AlignItems::Stretch,
                _ => bettertui_engine::tree::AlignItems::Stretch,
            };
        }
        if let Some(p) = l.padding {
            props.padding = Some(p.into());
        }
        if let Some(m) = l.margin {
            props.margin = Some(m.into());
        }
        if let Some(g) = l.gap {
            props.gap = Some(g.into());
        }
        if let Some(grow) = l.flex_grow {
            props.flex_grow = grow;
        }
        if let Some(shrink) = l.flex_shrink {
            props.flex_shrink = shrink;
        }
        props
    }
}

#[derive(serde::Deserialize)]
struct RectValuesJson {
    #[serde(default)]
    top: Option<f32>,
    #[serde(default)]
    right: Option<f32>,
    #[serde(default)]
    bottom: Option<f32>,
    #[serde(default)]
    left: Option<f32>,
    #[serde(default)]
    horizontal: Option<f32>,
    #[serde(default)]
    vertical: Option<f32>,
    #[serde(default)]
    all: Option<f32>,
}

impl From<RectValuesJson> for bettertui_engine::tree::RectValues {
    fn from(r: RectValuesJson) -> Self {
        let mut rv = bettertui_engine::tree::RectValues::default();
        if let Some(all) = r.all {
            rv.top = Some(all);
            rv.right = Some(all);
            rv.bottom = Some(all);
            rv.left = Some(all);
        }
        if let Some(h) = r.horizontal {
            rv.left = Some(h);
            rv.right = Some(h);
        }
        if let Some(v) = r.vertical {
            rv.top = Some(v);
            rv.bottom = Some(v);
        }
        if let Some(t) = r.top {
            rv.top = Some(t);
        }
        if let Some(ri) = r.right {
            rv.right = Some(ri);
        }
        if let Some(b) = r.bottom {
            rv.bottom = Some(b);
        }
        if let Some(l) = r.left {
            rv.left = Some(l);
        }
        rv
    }
}

#[derive(serde::Deserialize)]
struct GapJson {
    #[serde(default)]
    width: Option<f32>,
    #[serde(default)]
    height: Option<f32>,
}

impl From<GapJson> for bettertui_engine::tree::Gap {
    fn from(g: GapJson) -> Self {
        bettertui_engine::tree::Gap {
            row: g.width.unwrap_or(0.0),
            column: g.height.unwrap_or(0.0),
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum SizingJson {
    Points(f32),
    Percent(f32),
    Auto,
}

impl From<SizingJson> for bettertui_engine::tree::Sizing {
    fn from(s: SizingJson) -> Self {
        match s {
            SizingJson::Points(v) => bettertui_engine::tree::Sizing::Points(v),
            SizingJson::Percent(v) => bettertui_engine::tree::Sizing::Percent(v),
            SizingJson::Auto => bettertui_engine::tree::Sizing::Auto,
        }
    }
}

fn parse_node_kind(kind: &str) -> NodeKind {
    match kind.to_lowercase().as_str() {
        "text" => NodeKind::Text,
        "flex" => NodeKind::Flex,
        "input" => NodeKind::Input,
        "list" => NodeKind::List,
        "table" => NodeKind::Table,
        "tree" => NodeKind::Tree,
        "scroll" => NodeKind::Scroll,
        "tab" => NodeKind::Tab,
        "modal" => NodeKind::Modal,
        "spacer" => NodeKind::Spacer,
        "separator" => NodeKind::Separator,
        _ => NodeKind::Box,
    }
}

fn convert_command(cj: CommandJson) -> Option<bettertui_engine::protocol::Command> {
    use bettertui_engine::protocol::Command;
    match cj {
        CommandJson::CreateNode { id, kind } => Some(Command::CreateNode {
            id: u64_to_node_id(id),
            kind: parse_node_kind(&kind),
        }),
        CommandJson::RemoveNode { id } => Some(Command::RemoveNode {
            id: u64_to_node_id(id),
        }),
        CommandJson::AppendChild { parent, child } => Some(Command::AppendChild {
            parent: u64_to_node_id(parent),
            child: u64_to_node_id(child),
        }),
        CommandJson::InsertBefore { reference, child } => Some(Command::InsertBefore {
            reference: u64_to_node_id(reference),
            child: u64_to_node_id(child),
        }),
        CommandJson::MoveNode { node, new_parent } => Some(Command::MoveNode {
            node: u64_to_node_id(node),
            new_parent: u64_to_node_id(new_parent),
        }),
        CommandJson::ReplaceNode { old, new } => Some(Command::ReplaceNode {
            old: u64_to_node_id(old),
            new: u64_to_node_id(new),
        }),
        CommandJson::DetachNode { id } => Some(Command::DetachNode {
            id: u64_to_node_id(id),
        }),
        CommandJson::SetText { id, text } => Some(Command::SetText {
            id: u64_to_node_id(id),
            text,
        }),
        CommandJson::SetStyle { id, style } => {
            let mut s = Style::default();
            if let Some(fg) = style.fg {
                s.fg = Some(fg.into());
            }
            if let Some(bg) = style.bg {
                s.bg = Some(bg.into());
            }
            if let Some(v) = style.bold {
                s.bold = Some(v);
            }
            if let Some(v) = style.italic {
                s.italic = Some(v);
            }
            if let Some(v) = style.underline {
                s.underline = Some(v);
            }
            if let Some(v) = style.strikethrough {
                s.strikethrough = Some(v);
            }
            if let Some(v) = style.dim {
                s.dim = Some(v);
            }
            if let Some(v) = style.inverse {
                s.inverse = Some(v);
            }
            Some(Command::SetStyle {
                id: u64_to_node_id(id),
                style: s,
            })
        }
        CommandJson::SetForeground { id, color } => Some(Command::SetForeground {
            id: u64_to_node_id(id),
            color: color.into(),
        }),
        CommandJson::SetBackground { id, color } => Some(Command::SetBackground {
            id: u64_to_node_id(id),
            color: color.into(),
        }),
        CommandJson::SetBold { id, value } => Some(Command::SetBold {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetItalic { id, value } => Some(Command::SetItalic {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetUnderline { id, value } => Some(Command::SetUnderline {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetStrikethrough { id, value } => Some(Command::SetStrikethrough {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetDim { id, value } => Some(Command::SetDim {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetInverse { id, value } => Some(Command::SetInverse {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetHidden { id, value } => Some(Command::SetHidden {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetLayout { id, layout } => Some(Command::SetLayout {
            id: u64_to_node_id(id),
            layout: layout.into(),
        }),
        CommandJson::SetFlexDirection { id, direction } => {
            let dir = match direction.as_str() {
                "row" | "Row" => bettertui_engine::tree::FlexDirection::Row,
                "column" | "Column" => bettertui_engine::tree::FlexDirection::Column,
                "row_reverse" | "RowReverse" => bettertui_engine::tree::FlexDirection::RowReverse,
                "column_reverse" | "ColumnReverse" => {
                    bettertui_engine::tree::FlexDirection::ColumnReverse
                }
                _ => bettertui_engine::tree::FlexDirection::Column,
            };
            Some(Command::SetFlexDirection {
                id: u64_to_node_id(id),
                direction: dir,
            })
        }
        CommandJson::SetJustifyContent { id, value } => {
            let v = match value.as_str() {
                "flex_start" | "FlexStart" => bettertui_engine::tree::JustifyContent::FlexStart,
                "center" | "Center" => bettertui_engine::tree::JustifyContent::Center,
                "flex_end" | "FlexEnd" => bettertui_engine::tree::JustifyContent::FlexEnd,
                "space_between" | "SpaceBetween" => {
                    bettertui_engine::tree::JustifyContent::SpaceBetween
                }
                "space_around" | "SpaceAround" => {
                    bettertui_engine::tree::JustifyContent::SpaceAround
                }
                "space_evenly" | "SpaceEvenly" => {
                    bettertui_engine::tree::JustifyContent::SpaceEvenly
                }
                _ => bettertui_engine::tree::JustifyContent::FlexStart,
            };
            Some(Command::SetJustifyContent {
                id: u64_to_node_id(id),
                value: v,
            })
        }
        CommandJson::SetAlignItems { id, value } => {
            let v = match value.as_str() {
                "flex_start" | "FlexStart" => bettertui_engine::tree::AlignItems::FlexStart,
                "center" | "Center" => bettertui_engine::tree::AlignItems::Center,
                "flex_end" | "FlexEnd" => bettertui_engine::tree::AlignItems::FlexEnd,
                "stretch" | "Stretch" => bettertui_engine::tree::AlignItems::Stretch,
                _ => bettertui_engine::tree::AlignItems::Stretch,
            };
            Some(Command::SetAlignItems {
                id: u64_to_node_id(id),
                value: v,
            })
        }
        CommandJson::SetWidth { id, value } => Some(Command::SetWidth {
            id: u64_to_node_id(id),
            value: value.into(),
        }),
        CommandJson::SetHeight { id, value } => Some(Command::SetHeight {
            id: u64_to_node_id(id),
            value: value.into(),
        }),
        CommandJson::SetPadding { id, value } => Some(Command::SetPadding {
            id: u64_to_node_id(id),
            value: value.into(),
        }),
        CommandJson::SetMargin { id, value } => Some(Command::SetMargin {
            id: u64_to_node_id(id),
            value: value.into(),
        }),
        CommandJson::SetGap { id, value } => Some(Command::SetGap {
            id: u64_to_node_id(id),
            value: value.into(),
        }),
        CommandJson::SetFlexGrow { id, value } => Some(Command::SetFlexGrow {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetFlexShrink { id, value } => Some(Command::SetFlexShrink {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetPosition { id, value } => {
            let v = match value.as_str() {
                "relative" | "Relative" => bettertui_engine::tree::Position::Relative,
                "absolute" | "Absolute" => bettertui_engine::tree::Position::Absolute,
                _ => bettertui_engine::tree::Position::Relative,
            };
            Some(Command::SetPosition {
                id: u64_to_node_id(id),
                value: v,
            })
        }
        CommandJson::SetDisplay { id, value } => {
            let v = match value.as_str() {
                "none" | "None" => bettertui_engine::tree::VisibilityDisplay::None,
                _ => bettertui_engine::tree::VisibilityDisplay::Flex,
            };
            Some(Command::SetDisplay {
                id: u64_to_node_id(id),
                value: v,
            })
        }
        CommandJson::SetOpacity { id, value } => Some(Command::SetOpacity {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetClip { id, value } => Some(Command::SetClip {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetZIndex { id, value } => Some(Command::SetZIndex {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetOverflow { id, value } => {
            let v = match value.as_str() {
                "hidden" | "Hidden" => bettertui_engine::tree::Overflow::Hidden,
                "scroll" | "Scroll" => bettertui_engine::tree::Overflow::Scroll,
                _ => bettertui_engine::tree::Overflow::Visible,
            };
            Some(Command::SetOverflow {
                id: u64_to_node_id(id),
                value: v,
            })
        }
        CommandJson::FocusNode { id } => Some(Command::FocusNode {
            id: u64_to_node_id(id),
        }),
        CommandJson::BlurNode { id } => Some(Command::BlurNode {
            id: u64_to_node_id(id),
        }),
        CommandJson::BeginFrame { frame_id } => Some(Command::BeginFrame { frame_id }),
        CommandJson::CommitFrame { frame_id } => Some(Command::CommitFrame { frame_id }),
        CommandJson::Invalidate { id } => Some(Command::Invalidate {
            id: u64_to_node_id(id),
        }),
        CommandJson::Shutdown => Some(Command::Shutdown),
    }
}

// ─── NapiEngine ──────────────────────────────────────────────────────────────

#[napi]
pub struct NapiEngine {
    engine: Engine,
    renderer: Renderer,
    id_map: HashMap<u64, u64>,
}

#[napi]
impl NapiEngine {
    #[napi(constructor)]
    pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
        Self {
            engine: Engine::new(),
            renderer: Renderer::new(width.unwrap_or(80) as u16, height.unwrap_or(24) as u16),
            id_map: HashMap::new(),
        }
    }

    /// Process a batch of commands from JSON.
    /// Returns a JSON string with { success: number, errors: string[], idMappings: {temp: real}[] }.
    #[napi]
    pub fn process_commands(&mut self, commands_json: String) -> String {
        let commands: Vec<CommandJson> = match serde_json::from_str(&commands_json) {
            Ok(c) => c,
            Err(e) => {
                return serde_json::json!({
                    "success": 0,
                    "errors": [format!("JSON parse error: {}", e)],
                    "idMappings": []
                })
                .to_string();
            }
        };

        let mut rust_commands = Vec::new();
        let mut id_mappings = Vec::new();

        for cmd in commands {
            match cmd {
                CommandJson::CreateNode { id, kind } => {
                    let temp_id = id;
                    let real_id = self.engine.create_node(parse_node_kind(&kind));
                    let real_u64 = node_id_to_u64(real_id);
                    self.id_map.insert(temp_id, real_u64);
                    id_mappings.push(serde_json::json!({ "temp": temp_id, "real": real_u64 }));
                }
                other => {
                    if let Some(rust_cmd) = convert_command(other) {
                        rust_commands.push(rust_cmd);
                    }
                }
            }
        }

        // Resolve temporary IDs in remaining commands using the mapping
        let resolved_commands: Vec<bettertui_engine::protocol::Command> = rust_commands
            .into_iter()
            .map(|cmd| resolve_command_ids(cmd, &self.id_map))
            .collect();

        let result = self.engine.process_commands(resolved_commands);

        serde_json::json!({
            "success": result.processed,
            "errors": result.errors.iter().map(|e| format!("{}", e)).collect::<Vec<_>>(),
            "idMappings": id_mappings
        })
        .to_string()
    }

    /// Begin a new frame.
    #[napi]
    pub fn begin_frame(&mut self) {
        self.engine.begin_frame();
    }

    /// Commit the current frame.
    #[napi]
    pub fn commit_frame(&mut self) {
        self.engine.commit_frame();
    }

    /// Get the number of nodes in the tree.
    #[napi]
    pub fn node_count(&self) -> u32 {
        self.engine.node_count() as u32
    }

    /// Get the current frame count.
    #[napi]
    pub fn frame_count(&self) -> String {
        self.engine.frame_count().to_string()
    }

    /// Print the tree for debugging.
    #[napi]
    pub fn print_tree(&self) -> String {
        self.engine.print_tree()
    }

    /// Get a summary of the tree.
    #[napi]
    pub fn tree_summary(&self) -> String {
        self.engine.tree_summary()
    }

    /// Validate tree invariants.
    #[napi]
    pub fn validate(&self) -> bool {
        self.engine.validate().is_ok()
    }

    /// Get the root node ID as a string.
    #[napi]
    pub fn root(&self) -> String {
        node_id_to_u64(self.engine.arena().root()).to_string()
    }

    /// Get the arena generation (changes when nodes are allocated/freed).
    #[napi]
    pub fn generation(&self) -> String {
        self.engine.arena().generation().to_string()
    }

    /// Set text content on a node.
    #[napi]
    pub fn set_text(&mut self, id: u32, text: String) {
        let node_id = u64_to_node_id(id as u64);
        self.engine.set_text(node_id, text);
    }

    /// Set style on a node.
    #[napi]
    pub fn set_style(
        &mut self,
        id: u32,
        fg: Option<String>,
        bg: Option<String>,
        bold: Option<bool>,
        italic: Option<bool>,
        underline: Option<bool>,
    ) {
        let node_id = u64_to_node_id(id as u64);
        let mut style = Style::default();
        if let Some(fg) = fg {
            style.fg = Some(Color::Named(match fg.as_str() {
                "red" => bettertui_engine::tree::NamedColor::Red,
                "green" => bettertui_engine::tree::NamedColor::Green,
                "blue" => bettertui_engine::tree::NamedColor::Blue,
                "yellow" => bettertui_engine::tree::NamedColor::Yellow,
                "cyan" => bettertui_engine::tree::NamedColor::Cyan,
                "magenta" => bettertui_engine::tree::NamedColor::Magenta,
                "white" => bettertui_engine::tree::NamedColor::White,
                "black" => bettertui_engine::tree::NamedColor::Black,
                "bright_black" | "dark_gray" => bettertui_engine::tree::NamedColor::BrightBlack,
                "bright_red" | "light_red" => bettertui_engine::tree::NamedColor::BrightRed,
                "bright_green" | "light_green" => bettertui_engine::tree::NamedColor::BrightGreen,
                "bright_yellow" | "light_yellow" => bettertui_engine::tree::NamedColor::BrightYellow,
                "bright_blue" | "light_blue" => bettertui_engine::tree::NamedColor::BrightBlue,
                "bright_magenta" | "light_magenta" => bettertui_engine::tree::NamedColor::BrightMagenta,
                "bright_cyan" | "light_cyan" => bettertui_engine::tree::NamedColor::BrightCyan,
                "bright_white" | "light_gray" => bettertui_engine::tree::NamedColor::BrightWhite,
                _ => bettertui_engine::tree::NamedColor::White,
            }));
        }
        if let Some(bg) = bg {
            style.bg = Some(Color::Named(match bg.as_str() {
                "red" => bettertui_engine::tree::NamedColor::Red,
                "green" => bettertui_engine::tree::NamedColor::Green,
                "blue" => bettertui_engine::tree::NamedColor::Blue,
                "yellow" => bettertui_engine::tree::NamedColor::Yellow,
                "cyan" => bettertui_engine::tree::NamedColor::Cyan,
                "magenta" => bettertui_engine::tree::NamedColor::Magenta,
                "white" => bettertui_engine::tree::NamedColor::White,
                "black" => bettertui_engine::tree::NamedColor::Black,
                "bright_black" | "dark_gray" => bettertui_engine::tree::NamedColor::BrightBlack,
                "bright_red" | "light_red" => bettertui_engine::tree::NamedColor::BrightRed,
                "bright_green" | "light_green" => bettertui_engine::tree::NamedColor::BrightGreen,
                "bright_yellow" | "light_yellow" => bettertui_engine::tree::NamedColor::BrightYellow,
                "bright_blue" | "light_blue" => bettertui_engine::tree::NamedColor::BrightBlue,
                "bright_magenta" | "light_magenta" => bettertui_engine::tree::NamedColor::BrightMagenta,
                "bright_cyan" | "light_cyan" => bettertui_engine::tree::NamedColor::BrightCyan,
                "bright_white" | "light_gray" => bettertui_engine::tree::NamedColor::BrightWhite,
                _ => bettertui_engine::tree::NamedColor::White,
            }));
        }
        if let Some(v) = bold {
            style.bold = Some(v);
        }
        if let Some(v) = italic {
            style.italic = Some(v);
        }
        if let Some(v) = underline {
            style.underline = Some(v);
        }
        self.engine.set_style(node_id, style);
    }

    /// Create a node and return its ID.
    #[napi]
    pub fn create_node(&mut self, kind: String) -> u32 {
        let node_id = self.engine.create_node(parse_node_kind(&kind));
        node_id_to_u64(node_id) as u32
    }

    /// Append a child to a parent.
    #[napi]
    pub fn append_child(&mut self, parent: u32, child: u32) -> bool {
        let p = u64_to_node_id(parent as u64);
        let c = u64_to_node_id(child as u64);
        self.engine.append_child(p, c).is_ok()
    }

    /// Remove a node and its descendants.
    #[napi]
    pub fn remove_node(&mut self, id: u32) {
        let node_id = u64_to_node_id(id as u64);
        self.engine.remove_node(node_id);
    }

    /// Shutdown the engine.
    #[napi]
    pub fn shutdown(&mut self) {
        let _ = self
            .engine
            .process_command(bettertui_engine::protocol::Command::Shutdown);
    }

    /// Resize the renderer.
    #[napi]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width as u16, height as u16);
    }

    /// Render the current tree. Returns ANSI output as a buffer.
    #[napi]
    pub fn render(&mut self) -> RenderResult {
        let arena = self.engine.arena();
        let frame = self.renderer.render(arena);
        RenderResult {
            output_data: frame.output_data,
            width: frame.width as u32,
            height: frame.height as u32,
            dirty_region_count: frame.dirty_regions.len() as u32,
        }
    }

    /// Render with full repaint.
    #[napi]
    pub fn render_full(&mut self) -> RenderResult {
        let arena = self.engine.arena();
        let frame = self.renderer.render_full(arena);
        RenderResult {
            output_data: frame.output_data,
            width: frame.width as u32,
            height: frame.height as u32,
            dirty_region_count: frame.dirty_regions.len() as u32,
        }
    }

    /// Request a frame from the scheduler.
    #[napi]
    pub fn request_frame(&mut self) {
        self.renderer.request_frame();
    }

    /// Check if a frame should be rendered.
    #[napi]
    pub fn should_render(&self) -> String {
        match self.renderer.should_render() {
            bettertui_engine::scheduler::FrameStatus::Idle => "idle".into(),
            bettertui_engine::scheduler::FrameStatus::Pending => "pending".into(),
            bettertui_engine::scheduler::FrameStatus::Due => "due".into(),
            bettertui_engine::scheduler::FrameStatus::Overdue => "overdue".into(),
        }
    }

    /// Get renderer dimensions.
    #[napi]
    pub fn dimensions(&self) -> Vec<u32> {
        let (w, h) = self.renderer.dimensions();
        vec![w as u32, h as u32]
    }
}

/// Resolve temporary IDs in a command using the mapping.
fn resolve_command_ids(
    cmd: bettertui_engine::protocol::Command,
    id_map: &HashMap<u64, u64>,
) -> bettertui_engine::protocol::Command {
    use bettertui_engine::protocol::Command;

    let resolve = |id: bettertui_engine::tree::NodeId| -> bettertui_engine::tree::NodeId {
        let raw = node_id_to_u64(id);
        id_map.get(&raw).map_or(id, |&real| u64_to_node_id(real))
    };

    match cmd {
        Command::RemoveNode { id } => Command::RemoveNode { id: resolve(id) },
        Command::AppendChild { parent, child } => Command::AppendChild {
            parent: resolve(parent),
            child: resolve(child),
        },
        Command::InsertBefore { reference, child } => Command::InsertBefore {
            reference: resolve(reference),
            child: resolve(child),
        },
        Command::MoveNode { node, new_parent } => Command::MoveNode {
            node: resolve(node),
            new_parent: resolve(new_parent),
        },
        Command::ReplaceNode { old, new } => Command::ReplaceNode {
            old: resolve(old),
            new: resolve(new),
        },
        Command::DetachNode { id } => Command::DetachNode { id: resolve(id) },
        Command::SetStyle { id, style } => Command::SetStyle {
            id: resolve(id),
            style,
        },
        Command::SetForeground { id, color } => Command::SetForeground {
            id: resolve(id),
            color,
        },
        Command::SetBackground { id, color } => Command::SetBackground {
            id: resolve(id),
            color,
        },
        Command::SetBold { id, value } => Command::SetBold {
            id: resolve(id),
            value,
        },
        Command::SetItalic { id, value } => Command::SetItalic {
            id: resolve(id),
            value,
        },
        Command::SetUnderline { id, value } => Command::SetUnderline {
            id: resolve(id),
            value,
        },
        Command::SetStrikethrough { id, value } => Command::SetStrikethrough {
            id: resolve(id),
            value,
        },
        Command::SetDim { id, value } => Command::SetDim {
            id: resolve(id),
            value,
        },
        Command::SetInverse { id, value } => Command::SetInverse {
            id: resolve(id),
            value,
        },
        Command::SetHidden { id, value } => Command::SetHidden {
            id: resolve(id),
            value,
        },
        Command::SetLayout { id, layout } => Command::SetLayout {
            id: resolve(id),
            layout,
        },
        Command::SetFlexDirection { id, direction } => Command::SetFlexDirection {
            id: resolve(id),
            direction,
        },
        Command::SetJustifyContent { id, value } => Command::SetJustifyContent {
            id: resolve(id),
            value,
        },
        Command::SetAlignItems { id, value } => Command::SetAlignItems {
            id: resolve(id),
            value,
        },
        Command::SetAlignSelf { id, value } => Command::SetAlignSelf {
            id: resolve(id),
            value,
        },
        Command::SetWidth { id, value } => Command::SetWidth {
            id: resolve(id),
            value,
        },
        Command::SetHeight { id, value } => Command::SetHeight {
            id: resolve(id),
            value,
        },
        Command::SetMinWidth { id, value } => Command::SetMinWidth {
            id: resolve(id),
            value,
        },
        Command::SetMinHeight { id, value } => Command::SetMinHeight {
            id: resolve(id),
            value,
        },
        Command::SetMaxWidth { id, value } => Command::SetMaxWidth {
            id: resolve(id),
            value,
        },
        Command::SetMaxHeight { id, value } => Command::SetMaxHeight {
            id: resolve(id),
            value,
        },
        Command::SetPadding { id, value } => Command::SetPadding {
            id: resolve(id),
            value,
        },
        Command::SetMargin { id, value } => Command::SetMargin {
            id: resolve(id),
            value,
        },
        Command::SetGap { id, value } => Command::SetGap {
            id: resolve(id),
            value,
        },
        Command::SetFlexGrow { id, value } => Command::SetFlexGrow {
            id: resolve(id),
            value,
        },
        Command::SetFlexShrink { id, value } => Command::SetFlexShrink {
            id: resolve(id),
            value,
        },
        Command::SetFlexBasis { id, value } => Command::SetFlexBasis {
            id: resolve(id),
            value,
        },
        Command::SetPosition { id, value } => Command::SetPosition {
            id: resolve(id),
            value,
        },
        Command::SetInset { id, value } => Command::SetInset {
            id: resolve(id),
            value,
        },
        Command::SetText { id, text } => Command::SetText {
            id: resolve(id),
            text,
        },
        Command::SetAttribute { id, key, value } => Command::SetAttribute {
            id: resolve(id),
            key,
            value,
        },
        Command::RemoveAttribute { id, key } => Command::RemoveAttribute {
            id: resolve(id),
            key,
        },
        Command::SetDisplay { id, value } => Command::SetDisplay {
            id: resolve(id),
            value,
        },
        Command::SetOpacity { id, value } => Command::SetOpacity {
            id: resolve(id),
            value,
        },
        Command::SetClip { id, value } => Command::SetClip {
            id: resolve(id),
            value,
        },
        Command::SetTranslateX { id, value } => Command::SetTranslateX {
            id: resolve(id),
            value,
        },
        Command::SetTranslateY { id, value } => Command::SetTranslateY {
            id: resolve(id),
            value,
        },
        Command::SetZIndex { id, value } => Command::SetZIndex {
            id: resolve(id),
            value,
        },
        Command::SetOverflow { id, value } => Command::SetOverflow {
            id: resolve(id),
            value,
        },
        Command::FocusNode { id } => Command::FocusNode { id: resolve(id) },
        Command::BlurNode { id } => Command::BlurNode { id: resolve(id) },
        Command::SetTabIndex { id, value } => Command::SetTabIndex {
            id: resolve(id),
            value,
        },
        Command::Invalidate { id } => Command::Invalidate { id: resolve(id) },
        other => other,
    }
}

// ─── RenderResult ────────────────────────────────────────────────────────────

#[napi(object)]
pub struct RenderResult {
    pub output_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub dirty_region_count: u32,
}

// ─── NapiEventBus ────────────────────────────────────────────────────────────

#[napi]
pub struct NapiEventBus {
    bus: EventBus,
}

impl Default for NapiEventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
impl NapiEventBus {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            bus: bettertui_engine::events::EventBus::new(),
        }
    }

    /// Push a key event.
    #[napi]
    pub fn push_key(&mut self, key: String, ctrl: bool, shift: bool, alt: bool, target_id: u32) {
        use bettertui_engine::events::types::{Key, Modifiers};
        let target = u64_to_node_id(target_id as u64);
        let key = match key.as_str() {
            "enter" => Key::Enter,
            "escape" | "esc" => Key::Escape,
            "backspace" => Key::Backspace,
            "delete" => Key::Delete,
            "tab" => Key::Tab,
            "space" => Key::Space,
            "arrow_up" | "up" => Key::ArrowUp,
            "arrow_down" | "down" => Key::ArrowDown,
            "arrow_left" | "left" => Key::ArrowLeft,
            "arrow_right" | "right" => Key::ArrowRight,
            "home" => Key::Home,
            "end" => Key::End,
            "page_up" => Key::PageUp,
            "page_down" => Key::PageDown,
            s if s.starts_with('f') && s.len() <= 3 => {
                if let Ok(n) = s[1..].parse::<u8>() {
                    Key::F(n)
                } else {
                    Key::Character(key.chars().next().unwrap_or(' '))
                }
            }
            s if s.starts_with("ctrl_") => Key::Ctrl(s.chars().nth(5).unwrap_or(' ')),
            s if s.starts_with("alt_") => Key::Alt(s.chars().nth(4).unwrap_or(' ')),
            s if s.len() == 1 => Key::Character(s.chars().next().unwrap()),
            _ => Key::Character(key.chars().next().unwrap_or(' ')),
        };
        self.bus.push_key(
            key,
            Modifiers {
                ctrl,
                shift,
                alt,
                meta: false,
            },
            target,
        );
    }

    /// Push a mouse event.
    #[napi]
    pub fn push_mouse(&mut self, button: String, x: u32, y: u32, target_id: u32) {
        use bettertui_engine::events::types::MouseButton;
        use bettertui_engine::tree::visual::Point;
        let target = u64_to_node_id(target_id as u64);
        let btn = match button.as_str() {
            "left" => MouseButton::Left,
            "right" => MouseButton::Right,
            "middle" => MouseButton::Middle,
            "scroll_up" => MouseButton::ScrollUp,
            "scroll_down" => MouseButton::ScrollDown,
            _ => MouseButton::None,
        };
        self.bus
            .push_mouse(btn, Point::new(x as u16, y as u16), target);
    }

    /// Push a paste event.
    #[napi]
    pub fn push_paste(&mut self, text: String, target_id: u32) {
        let target = u64_to_node_id(target_id as u64);
        self.bus.push_paste(text, target);
    }

    /// Push a resize event.
    #[napi]
    pub fn push_resize(&mut self, width: u32, height: u32, prev_width: u32, prev_height: u32) {
        self.bus.push_resize(
            width as u16,
            height as u16,
            prev_width as u16,
            prev_height as u16,
        );
    }

    /// Get the number of pending events.
    #[napi]
    pub fn len(&self) -> u32 {
        self.bus.len() as u32
    }

    /// Check if the event bus is empty.
    #[napi]
    pub fn is_empty(&self) -> bool {
        self.bus.is_empty()
    }

    /// Clear all events.
    #[napi]
    pub fn clear(&mut self) {
        self.bus.clear();
    }

    /// Drain all events as JSON.
    #[napi]
    pub fn drain(&mut self) -> String {
        let events = self.bus.drain();
        let event_jsons: Vec<serde_json::Value> = events
            .iter()
            .map(|e| match e {
                bettertui_engine::events::Event::Key(ke) => {
                    let key_str = format!("{:?}", ke.key);
                    serde_json::json!({
                        "type": "key",
                        "key": key_str.to_lowercase(),
                        "ctrl": ke.modifiers.ctrl,
                        "shift": ke.modifiers.shift,
                        "alt": ke.modifiers.alt,
                        "target": node_id_to_u64(ke.target),
                    })
                }
                bettertui_engine::events::Event::Mouse(me) => {
                    serde_json::json!({
                        "type": "mouse",
                        "button": format!("{:?}", me.button).to_lowercase(),
                        "x": me.position.x,
                        "y": me.position.y,
                        "target": node_id_to_u64(me.target),
                    })
                }
                bettertui_engine::events::Event::Resize(re) => {
                    serde_json::json!({
                        "type": "resize",
                        "width": re.width,
                        "height": re.height,
                        "prev_width": re.previous_width,
                        "prev_height": re.previous_height,
                    })
                }
                _ => serde_json::json!({ "type": "unknown" }),
            })
            .collect();
        serde_json::to_string(&event_jsons).unwrap_or_else(|_| "[]".into())
    }
}

// ─── NapiFocusManager ────────────────────────────────────────────────────────

#[napi]
pub struct NapiFocusManager {
    manager: FocusManager,
}

impl Default for NapiFocusManager {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
impl NapiFocusManager {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            manager: FocusManager::new(),
        }
    }

    /// Focus a node.
    #[napi]
    pub fn focus(&mut self, id: u32) {
        let node_id = u64_to_node_id(id as u64);
        self.manager.focus(node_id);
    }

    /// Blur the currently focused node.
    #[napi]
    pub fn blur_current(&mut self) {
        if let Some(focused) = self.manager.focused() {
            let _ = self.manager.blur(focused);
        }
    }

    /// Blur a specific node.
    #[napi]
    pub fn blur(&mut self, id: u32) {
        let node_id = u64_to_node_id(id as u64);
        let _ = self.manager.blur(node_id);
    }

    /// Get the currently focused node ID (0 if none).
    #[napi]
    pub fn focused(&self) -> u32 {
        self.manager
            .focused()
            .map(|id| node_id_to_u64(id) as u32)
            .unwrap_or(0)
    }

    /// Traverse focus to next/previous node.
    #[napi]
    pub fn traverse(&mut self, forward: bool) {
        let next = if forward {
            bettertui_engine::focus::FocusTraversal::next(&self.manager)
        } else {
            bettertui_engine::focus::FocusTraversal::previous(&self.manager)
        };
        if let Some(id) = next {
            self.manager.focus(id);
        }
    }
}

// ─── NapiTextEngine ──────────────────────────────────────────────────────────

#[napi]
pub struct NapiTextEngine {
    engine: TextEngine,
}

#[napi]
impl NapiTextEngine {
    #[napi(constructor)]
    pub fn new(text: Option<String>) -> Self {
        Self {
            engine: text.map(|t| TextEngine::with_text(&t)).unwrap_or_default(),
        }
    }

    /// Insert a character at the cursor position.
    #[napi]
    pub fn insert_char(&mut self, ch: String) {
        if let Some(c) = ch.chars().next() {
            self.engine.insert_char(c);
        }
    }

    /// Insert a string at the cursor position.
    #[napi]
    pub fn insert_str(&mut self, text: String) {
        self.engine.insert_str(&text);
    }

    /// Delete the character before the cursor.
    #[napi]
    pub fn delete_char(&mut self) {
        self.engine.delete_char();
    }

    /// Undo the last action.
    #[napi]
    pub fn undo(&mut self) -> bool {
        self.engine.undo()
    }

    /// Redo the last undone action.
    #[napi]
    pub fn redo(&mut self) -> bool {
        self.engine.redo()
    }

    /// Get the full text content.
    #[napi]
    pub fn text(&self) -> String {
        self.engine.text()
    }

    /// Get the character count.
    #[napi]
    pub fn char_count(&self) -> u32 {
        self.engine.char_count() as u32
    }

    /// Get the line count.
    #[napi]
    pub fn line_count(&self) -> u32 {
        self.engine.line_count() as u32
    }

    /// Get the word count.
    #[napi]
    pub fn word_count(&self) -> u32 {
        self.engine.word_count() as u32
    }

    /// Check if the buffer is empty.
    #[napi]
    pub fn is_empty(&self) -> bool {
        self.engine.is_empty()
    }

    /// Clear the buffer.
    #[napi]
    pub fn clear(&mut self) {
        self.engine.clear();
    }

    /// Get cursor position.
    #[napi]
    pub fn cursor_position(&self) -> u32 {
        self.engine.cursor().position() as u32
    }

    /// Set cursor position.
    #[napi]
    pub fn set_cursor_position(&mut self, pos: u32) {
        self.engine.cursor_mut().set_position(pos as usize);
    }

    /// Search for a pattern.
    #[napi]
    pub fn search(&mut self, pattern: String) -> String {
        let results = self.engine.search(&pattern, Default::default());
        let json_results: Vec<serde_json::Value> = results
            .iter()
            .map(|r| {
                serde_json::json!({
                    "start": r.range.start,
                    "end": r.range.end,
                    "line": r.line,
                    "column": r.column,
                })
            })
            .collect();
        serde_json::to_string(&json_results).unwrap_or_else(|_| "[]".into())
    }
}

// ─── NapiScheduler ───────────────────────────────────────────────────────────

#[napi]
pub struct NapiScheduler {
    scheduler: Scheduler,
}

#[napi]
impl NapiScheduler {
    #[napi(constructor)]
    pub fn new(fps: Option<u32>) -> Self {
        Self {
            scheduler: fps.map(Scheduler::with_fps).unwrap_or_default(),
        }
    }

    /// Request a frame.
    #[napi]
    pub fn request_frame(&mut self) {
        self.scheduler.request_frame();
    }

    /// Check if a frame is due.
    #[napi]
    pub fn should_render(&self) -> String {
        match self.scheduler.status() {
            bettertui_engine::scheduler::FrameStatus::Idle => "idle".into(),
            bettertui_engine::scheduler::FrameStatus::Pending => "pending".into(),
            bettertui_engine::scheduler::FrameStatus::Due => "due".into(),
            bettertui_engine::scheduler::FrameStatus::Overdue => "overdue".into(),
        }
    }

    /// Begin a frame.
    #[napi]
    pub fn begin_frame(&mut self) -> bool {
        self.scheduler.begin_frame()
    }

    /// End a frame.
    #[napi]
    pub fn end_frame(&mut self) {
        self.scheduler.end_frame();
    }

    /// Get frame count.
    #[napi]
    pub fn frame_count(&self) -> String {
        self.scheduler.frame_count().to_string()
    }

    /// Get dropped frame count.
    #[napi]
    pub fn dropped_frames(&self) -> String {
        self.scheduler.dropped_frames().to_string()
    }
}

// ─── NapiCapabilities ────────────────────────────────────────────────────────

#[napi]
pub struct NapiCapabilities {
    brand: String,
    true_color: bool,
    kitty_keyboard: bool,
    bracketed_paste: bool,
    mouse_support: bool,
    osc52_clipboard: bool,
    term_width: u32,
    term_height: u32,
}

#[napi]
impl NapiCapabilities {
    #[napi(factory)]
    pub fn detect() -> Self {
        let caps = bettertui_engine::capabilities::global_capabilities();
        let (w, h) = caps.terminal_size();
        Self {
            brand: format!("{:?}", caps.brand()),
            true_color: caps.supports_true_color(),
            kitty_keyboard: caps.supports_kitty_keyboard(),
            bracketed_paste: caps.supports_bracketed_paste(),
            mouse_support: caps.input.mouse_modes.normal_mouse,
            osc52_clipboard: caps.supports_osc52(),
            term_width: w as u32,
            term_height: h as u32,
        }
    }

    #[napi(getter)]
    pub fn get_brand(&self) -> String {
        self.brand.clone()
    }

    #[napi(getter)]
    pub fn get_true_color(&self) -> bool {
        self.true_color
    }

    #[napi(getter)]
    pub fn get_kitty_keyboard(&self) -> bool {
        self.kitty_keyboard
    }

    #[napi(getter)]
    pub fn get_bracketed_paste(&self) -> bool {
        self.bracketed_paste
    }

    #[napi(getter)]
    pub fn get_mouse_support(&self) -> bool {
        self.mouse_support
    }

    #[napi(getter)]
    pub fn get_osc52_clipboard(&self) -> bool {
        self.osc52_clipboard
    }

    #[napi(getter)]
    pub fn get_terminal_size(&self) -> Vec<u32> {
        vec![self.term_width, self.term_height]
    }
}

// ─── Global Functions ────────────────────────────────────────────────────────

/// Get the library version.
#[napi]
pub fn get_version() -> String {
    VERSION.to_string()
}

/// Detect terminal capabilities.
#[napi]
pub fn detect_capabilities() -> String {
    let caps = bettertui_engine::capabilities::global_capabilities();
    let (w, h) = caps.terminal_size();
    serde_json::json!({
        "brand": format!("{:?}", caps.brand()),
        "trueColor": caps.supports_true_color(),
        "kittyKeyboard": caps.supports_kitty_keyboard(),
        "bracketedPaste": caps.supports_bracketed_paste(),
        "mouse": caps.input.mouse_modes.normal_mouse,
        "osc52": caps.supports_osc52(),
        "terminalSize": { "width": w, "height": h },
    })
    .to_string()
}
