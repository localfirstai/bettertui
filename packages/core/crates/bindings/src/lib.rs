#![deny(clippy::all)]

use bettertui_engine::VERSION;
use bettertui_engine::engine::Engine;
use bettertui_engine::events::EventBus;
use bettertui_engine::focus::FocusManager;
use bettertui_engine::post_process::RenderPass;
use bettertui_engine::renderer::Renderer;
use bettertui_engine::scheduler::Scheduler;
use bettertui_engine::text::TextEngine;
use bettertui_engine::tree::{Color, LayoutProps, NodeKind, Style};
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
    SetAlignSelf { id: u64, value: String },
    SetWidth { id: u64, value: SizingJson },
    SetHeight { id: u64, value: SizingJson },
    SetMinWidth { id: u64, value: SizingJson },
    SetMinHeight { id: u64, value: SizingJson },
    SetMaxWidth { id: u64, value: SizingJson },
    SetMaxHeight { id: u64, value: SizingJson },
    SetFlexBasis { id: u64, value: SizingJson },
    SetPadding { id: u64, value: RectValuesJson },
    SetMargin { id: u64, value: RectValuesJson },
    SetGap { id: u64, value: GapJson },
    SetFlexGrow { id: u64, value: f32 },
    SetFlexShrink { id: u64, value: f32 },
    SetPosition { id: u64, value: String },
    SetInset { id: u64, value: RectValuesJson },
    SetDisplay { id: u64, value: String },
    SetOpacity { id: u64, value: f32 },
    SetClip { id: u64, value: bool },
    SetZIndex { id: u64, value: i32 },
    SetOverflow { id: u64, value: String },
    SetAttribute { id: u64, key: String, value: String },
    RemoveAttribute { id: u64, key: String },
    SetTranslateX { id: u64, value: i32 },
    SetTranslateY { id: u64, value: i32 },
    SetTabIndex { id: u64, value: i32 },
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
                _ => bettertui_engine::tree::color::Color::parse(&name)
                    .unwrap_or(Color::Named(bettertui_engine::tree::NamedColor::White)),
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
        "table" | "datatable" => NodeKind::Table,
        "tree" => NodeKind::Tree,
        "scrollarea" | "scroll" => NodeKind::Scroll,
        "tabs" | "tab" => NodeKind::Tab,
        "modal" => NodeKind::Modal,
        "code" => NodeKind::Code,
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
                "start" | "Start" | "flex_start" | "FlexStart" => {
                    bettertui_engine::tree::AlignItems::FlexStart
                }
                "end" | "End" | "flex_end" | "FlexEnd" => {
                    bettertui_engine::tree::AlignItems::FlexEnd
                }
                "center" | "Center" => bettertui_engine::tree::AlignItems::Center,
                "stretch" | "Stretch" => bettertui_engine::tree::AlignItems::Stretch,
                "baseline" | "Baseline" => bettertui_engine::tree::AlignItems::Baseline,
                _ => bettertui_engine::tree::AlignItems::Stretch,
            };
            Some(Command::SetAlignItems {
                id: u64_to_node_id(id),
                value: v,
            })
        }
        CommandJson::SetAlignSelf { id, value } => {
            let v = match value.as_str() {
                "start" | "Start" | "flex_start" | "FlexStart" => {
                    bettertui_engine::tree::AlignSelf::FlexStart
                }
                "end" | "End" | "flex_end" | "FlexEnd" => {
                    bettertui_engine::tree::AlignSelf::FlexEnd
                }
                "center" | "Center" => bettertui_engine::tree::AlignSelf::Center,
                "stretch" | "Stretch" => bettertui_engine::tree::AlignSelf::Stretch,
                "baseline" | "Baseline" => bettertui_engine::tree::AlignSelf::Baseline,
                _ => bettertui_engine::tree::AlignSelf::Stretch,
            };
            Some(Command::SetAlignSelf {
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
        CommandJson::SetMinWidth { id, value } => Some(Command::SetMinWidth {
            id: u64_to_node_id(id),
            value: value.into(),
        }),
        CommandJson::SetMinHeight { id, value } => Some(Command::SetMinHeight {
            id: u64_to_node_id(id),
            value: value.into(),
        }),
        CommandJson::SetMaxWidth { id, value } => Some(Command::SetMaxWidth {
            id: u64_to_node_id(id),
            value: value.into(),
        }),
        CommandJson::SetMaxHeight { id, value } => Some(Command::SetMaxHeight {
            id: u64_to_node_id(id),
            value: value.into(),
        }),
        CommandJson::SetFlexBasis { id, value } => Some(Command::SetFlexBasis {
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
        CommandJson::SetInset { id, value } => Some(Command::SetInset {
            id: u64_to_node_id(id),
            value: value.into(),
        }),
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
        CommandJson::SetAttribute { id, key, value } => Some(Command::SetAttribute {
            id: u64_to_node_id(id),
            key,
            value,
        }),
        CommandJson::RemoveAttribute { id, key } => Some(Command::RemoveAttribute {
            id: u64_to_node_id(id),
            key,
        }),
        CommandJson::SetTranslateX { id, value } => Some(Command::SetTranslateX {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetTranslateY { id, value } => Some(Command::SetTranslateY {
            id: u64_to_node_id(id),
            value,
        }),
        CommandJson::SetTabIndex { id, value } => Some(Command::SetTabIndex {
            id: u64_to_node_id(id),
            value,
        }),
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
                "bright_yellow" | "light_yellow" => {
                    bettertui_engine::tree::NamedColor::BrightYellow
                }
                "bright_blue" | "light_blue" => bettertui_engine::tree::NamedColor::BrightBlue,
                "bright_magenta" | "light_magenta" => {
                    bettertui_engine::tree::NamedColor::BrightMagenta
                }
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
                "bright_yellow" | "light_yellow" => {
                    bettertui_engine::tree::NamedColor::BrightYellow
                }
                "bright_blue" | "light_blue" => bettertui_engine::tree::NamedColor::BrightBlue,
                "bright_magenta" | "light_magenta" => {
                    bettertui_engine::tree::NamedColor::BrightMagenta
                }
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
        let arena = self.engine.arena_mut();
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
        let arena = self.engine.arena_mut();
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

    // ─── Post-Processing Pipeline ────────────────────────────────────

    /// Enable or disable the entire post-processing pipeline.
    #[napi]
    pub fn set_post_processing_enabled(&mut self, enabled: bool) {
        self.renderer.pipeline_mut().set_enabled(enabled);
    }

    /// Check if post-processing is enabled.
    #[napi]
    pub fn is_post_processing_enabled(&self) -> bool {
        self.renderer.pipeline().enabled()
    }

    /// Add a color matrix pass.
    /// matrix: 16 comma-separated f32 values (row-major 4x4 RGBA matrix)
    /// enabled: whether the pass starts enabled
    #[napi]
    pub fn add_color_matrix_pass(&mut self, matrix_str: String, enabled: Option<bool>) {
        let values: Vec<f32> = matrix_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if values.len() != 16 {
            return;
        }
        let mut m = [0.0f32; 16];
        m.copy_from_slice(&values);
        let mut pass = bettertui_engine::post_process::effects::ColorMatrixPass::new(m);
        if !enabled.unwrap_or(true) {
            pass.set_enabled(false);
        }
        self.renderer.pipeline_mut().add_pass(Box::new(pass));
    }

    /// Add a CRT effect pass.
    #[napi]
    pub fn add_crt_pass(&mut self, glow: Option<f64>, curvature: Option<f64>) {
        let mut pass = bettertui_engine::post_process::effects::CrtPass::new();
        if let Some(g) = glow {
            pass = pass.with_glow(g as f32);
        }
        if let Some(c) = curvature {
            pass = pass.with_curvature(c as f32);
        }
        self.renderer.pipeline_mut().add_pass(Box::new(pass));
    }

    /// Add a scanlines effect pass.
    #[napi]
    pub fn add_scanlines_pass(&mut self, intensity: Option<f64>, odd_rows: Option<bool>) {
        let mut pass = bettertui_engine::post_process::effects::ScanlinesPass::new();
        if let Some(i) = intensity {
            pass = pass.with_intensity(i as f32);
        }
        if let Some(odd) = odd_rows {
            pass = pass.with_mode(if odd {
                bettertui_engine::post_process::effects::ScanlineMode::OddRows
            } else {
                bettertui_engine::post_process::effects::ScanlineMode::EvenRows
            });
        }
        self.renderer.pipeline_mut().add_pass(Box::new(pass));
    }

    /// Add a vignette effect pass.
    #[napi]
    pub fn add_vignette_pass(
        &mut self,
        strength: Option<f64>,
        radius: Option<f64>,
        falloff: Option<f64>,
    ) {
        let mut pass = bettertui_engine::post_process::effects::VignettePass::new();
        if let Some(s) = strength {
            pass = pass.with_strength(s as f32);
        }
        if let Some(r) = radius {
            pass = pass.with_radius(r as f32);
        }
        if let Some(f) = falloff {
            pass = pass.with_falloff(f as f32);
        }
        self.renderer.pipeline_mut().add_pass(Box::new(pass));
    }

    /// Add a noise effect pass.
    #[napi]
    pub fn add_noise_pass(&mut self, intensity: Option<f64>, seed: Option<u32>) {
        let mut pass = bettertui_engine::post_process::effects::NoisePass::new();
        if let Some(i) = intensity {
            pass = pass.with_intensity(i as f32);
        }
        if let Some(s) = seed {
            pass = pass.with_seed(s);
        }
        self.renderer.pipeline_mut().add_pass(Box::new(pass));
    }

    /// Add a chromatic aberration effect pass.
    #[napi]
    pub fn add_chromatic_aberration_pass(&mut self, strength: Option<f64>) {
        let mut pass = bettertui_engine::post_process::effects::ChromaticAberrationPass::new();
        if let Some(s) = strength {
            pass = pass.with_strength(s as f32);
        }
        self.renderer.pipeline_mut().add_pass(Box::new(pass));
    }

    /// Add a bloom effect pass.
    #[napi]
    pub fn add_bloom_pass(
        &mut self,
        threshold: Option<f64>,
        strength: Option<f64>,
        radius: Option<u32>,
    ) {
        let mut pass = bettertui_engine::post_process::effects::BloomPass::new();
        if let Some(t) = threshold {
            pass = pass.with_threshold(t as f32);
        }
        if let Some(s) = strength {
            pass = pass.with_strength(s as f32);
        }
        if let Some(r) = radius {
            pass = pass.with_radius(r as u16);
        }
        self.renderer.pipeline_mut().add_pass(Box::new(pass));
    }

    /// Remove a render pass by name.
    #[napi]
    pub fn remove_render_pass(&mut self, name: String) {
        self.renderer.pipeline_mut().remove_pass(&name);
    }

    /// Enable or disable a render pass by name.
    #[napi]
    pub fn set_pass_enabled(&mut self, name: String, enabled: bool) {
        if let Some(pass) = self.renderer.pipeline_mut().get_pass_mut(&name) {
            pass.set_enabled(enabled);
        }
    }

    /// Check if a render pass is enabled.
    #[napi]
    pub fn is_pass_enabled(&self, name: String) -> Option<bool> {
        self.renderer
            .pipeline()
            .get_pass(&name)
            .map(|p| p.enabled())
    }

    /// Get the number of active render passes.
    #[napi]
    pub fn pass_count(&self) -> u32 {
        self.renderer.pipeline().len() as u32
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

// ─── NapiKeymap ─────────────────────────────────────────────────────────────

use bettertui_engine::keybinding::{KeyBinding, KeyParser, Keymap};

#[napi(object)]
pub struct BindingInfo {
    pub id: String,
    pub keys: String,
    pub command: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub layer: String,
}

fn format_key_combo(combo: &bettertui_engine::keybinding::KeyCombo) -> String {
    let mut parts = Vec::new();
    if combo.modifiers.ctrl {
        parts.push("ctrl".to_string());
    }
    if combo.modifiers.shift {
        parts.push("shift".to_string());
    }
    if combo.modifiers.alt {
        parts.push("alt".to_string());
    }
    if combo.modifiers.meta {
        parts.push("meta".to_string());
    }
    use bettertui_engine::events::types::Key;
    let key_display = match &combo.key {
        Key::Character(ch) => ch.to_string(),
        Key::Ctrl(ch) => format!("ctrl_{}", ch),
        Key::Alt(ch) => format!("alt_{}", ch),
        Key::Enter => "enter".to_string(),
        Key::Escape => "escape".to_string(),
        Key::Backspace => "backspace".to_string(),
        Key::Delete => "delete".to_string(),
        Key::Tab => "tab".to_string(),
        Key::Space => "space".to_string(),
        Key::ArrowUp => "up".to_string(),
        Key::ArrowDown => "down".to_string(),
        Key::ArrowLeft => "left".to_string(),
        Key::ArrowRight => "right".to_string(),
        Key::Home => "home".to_string(),
        Key::End => "end".to_string(),
        Key::PageUp => "page_up".to_string(),
        Key::PageDown => "page_down".to_string(),
        Key::F(n) => format!("f{}", n),
    };
    parts.push(key_display);
    if parts.len() == 1 {
        parts[0].clone()
    } else {
        parts.join("+")
    }
}

fn format_key_sequence(seq: &bettertui_engine::keybinding::KeySequence) -> String {
    seq.keys
        .iter()
        .map(format_key_combo)
        .collect::<Vec<_>>()
        .join(" ")
}

#[napi]
pub struct NapiKeymap {
    keymap: Keymap,
}

impl Default for NapiKeymap {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
impl NapiKeymap {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            keymap: Keymap::new(),
        }
    }

    /// Add a binding to a specific layer with full command name support
    #[napi]
    pub fn add_binding(
        &mut self,
        layer: String,
        id: String,
        keys: String,
        command: String,
        description: Option<String>,
        priority: i32,
    ) -> bool {
        match KeyParser::parse_sequence(&keys) {
            Ok(seq) => {
                let binding = KeyBinding {
                    id,
                    command,
                    sequence: seq,
                    description,
                    condition: None,
                    enabled: true,
                };
                self.keymap.add_binding_to_layer(&layer, binding, priority);
                true
            }
            Err(_) => false,
        }
    }

    /// Set the current mode for mode-conditional bindings
    #[napi]
    pub fn set_mode(&mut self, mode: String) {
        self.keymap.set_mode(mode);
    }

    /// Get the current mode
    #[napi]
    pub fn current_mode(&self) -> Option<String> {
        self.keymap.current_mode().map(String::from)
    }

    /// Clear the current mode
    #[napi]
    pub fn clear_mode(&mut self) {
        self.keymap.clear_mode();
    }

    /// Remove a layer by name
    #[napi]
    pub fn remove_layer(&mut self, name: String) -> bool {
        self.keymap.remove_layer(&name)
    }

    /// Set chord timeout in ms
    #[napi]
    pub fn set_chord_timeout(&mut self, ms: f64) {
        self.keymap.set_chord_timeout(ms as u64);
    }

    /// Get chord timeout in ms
    #[napi]
    pub fn chord_timeout(&self) -> f64 {
        self.keymap.chord_timeout_ms() as f64
    }

    /// Handle a key string (e.g. "ctrl+c") and return the command if matched
    #[napi]
    pub fn handle_key(&mut self, key_str: String) -> Option<String> {
        match KeyParser::parse_combo(&key_str) {
            Ok(combo) => {
                let event = bettertui_engine::events::types::KeyEvent {
                    key: combo.key,
                    modifiers: combo.modifiers,
                    target: bettertui_engine::tree::NodeId::default(),
                    default_prevented: false,
                    phase: bettertui_engine::events::types::EventPhase::Target,
                };
                self.keymap.handle_event(&event)
            }
            Err(_) => None,
        }
    }

    /// Check if there's a pending chord sequence
    #[napi]
    pub fn has_pending(&self) -> bool {
        self.keymap.has_pending_sequence()
    }

    /// Clear any pending chord sequence
    #[napi]
    pub fn clear_pending(&mut self) {
        self.keymap.clear_pending_sequence();
    }

    /// Get formatted pending keys (for UI display)
    #[napi]
    pub fn pending_keys(&self) -> Vec<String> {
        self.keymap
            .pending_keys()
            .iter()
            .map(format_key_combo)
            .collect()
    }

    /// Get active bindings for cheat sheet / UI display
    #[napi]
    pub fn active_bindings(&self) -> Vec<BindingInfo> {
        self.keymap
            .active_bindings()
            .into_iter()
            .map(|(b, layer_name)| BindingInfo {
                id: b.id.clone(),
                keys: format_key_sequence(&b.sequence),
                command: b.command.clone(),
                description: b.description.clone(),
                enabled: b.enabled,
                layer: layer_name.to_string(),
            })
            .collect()
    }

    /// Get all bindings (including disabled)
    #[napi]
    pub fn all_bindings(&self) -> Vec<BindingInfo> {
        self.keymap
            .all_bindings()
            .into_iter()
            .map(|(b, layer_name)| BindingInfo {
                id: b.id.clone(),
                keys: format_key_sequence(&b.sequence),
                command: b.command.clone(),
                description: b.description.clone(),
                enabled: b.enabled,
                layer: layer_name.to_string(),
            })
            .collect()
    }

    /// Get command history
    #[napi]
    pub fn command_history(&self) -> Vec<String> {
        self.keymap.command_history().to_vec()
    }

    /// Clear command history
    #[napi]
    pub fn clear_history(&mut self) {
        self.keymap.clear_history();
    }

    /// Parse a key string into its normalized form
    #[napi]
    pub fn parse_key(&self, key_str: String) -> Option<String> {
        KeyParser::parse_combo(&key_str)
            .ok()
            .map(|combo| format_key_combo(&combo))
    }

    /// Parse a key sequence string into normalized parts
    #[napi]
    pub fn parse_sequence(&self, key_str: String) -> Vec<String> {
        KeyParser::parse_sequence(&key_str)
            .ok()
            .map(|seq| seq.keys.iter().map(format_key_combo).collect())
            .unwrap_or_default()
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

    /// Push a mouse motion event.
    #[napi]
    pub fn push_mouse_motion(&mut self, x: u32, y: u32, target_id: u32) {
        use bettertui_engine::events::types::MouseButton;
        use bettertui_engine::tree::visual::Point;
        let target = u64_to_node_id(target_id as u64);
        self.bus
            .push_mouse(MouseButton::None, Point::new(x as u16, y as u16), target);
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

    /// Focus a node. Returns true if focus changed.
    #[napi]
    pub fn focus(&mut self, id: u32) -> bool {
        let node_id = u64_to_node_id(id as u64);
        self.manager.focus(node_id).is_some()
    }

    /// Blur the currently focused node.
    #[napi]
    pub fn blur_current(&mut self) -> bool {
        if let Some(focused) = self.manager.focused() {
            self.manager.blur(focused).is_some()
        } else {
            false
        }
    }

    /// Blur a specific node. Returns true if the node was blurred.
    #[napi]
    pub fn blur(&mut self, id: u32) -> bool {
        let node_id = u64_to_node_id(id as u64);
        self.manager.blur(node_id).is_some()
    }

    /// Get the currently focused node ID (0 if none).
    #[napi]
    pub fn focused(&self) -> u32 {
        self.manager
            .focused()
            .map(|id| node_id_to_u64(id) as u32)
            .unwrap_or(0)
    }

    /// Check if a specific node is focused.
    #[napi]
    pub fn is_focused(&self, id: u32) -> bool {
        let node_id = u64_to_node_id(id as u64);
        self.manager.is_focused(node_id)
    }

    /// Traverse focus to next/previous node. Returns the newly focused node ID (0 if none).
    #[napi]
    pub fn traverse(&mut self, direction: String) -> u32 {
        let dir = match direction.to_lowercase().as_str() {
            "forward" | "next" => bettertui_engine::focus::FocusDirection::Forward,
            "backward" | "previous" | "prev" => bettertui_engine::focus::FocusDirection::Backward,
            "first" => bettertui_engine::focus::FocusDirection::First,
            "last" => bettertui_engine::focus::FocusDirection::Last,
            "up" => bettertui_engine::focus::FocusDirection::Up,
            "down" => bettertui_engine::focus::FocusDirection::Down,
            "left" => bettertui_engine::focus::FocusDirection::Left,
            "right" => bettertui_engine::focus::FocusDirection::Right,
            _ => bettertui_engine::focus::FocusDirection::Forward,
        };
        let next = bettertui_engine::focus::FocusTraversal::traverse(&self.manager, dir);
        if let Some(id) = next {
            self.manager.focus(id);
            node_id_to_u64(id) as u32
        } else {
            0
        }
    }

    /// Get focus order list.
    #[napi]
    pub fn focus_order(&self) -> Vec<u32> {
        self.manager
            .tab_order()
            .iter()
            .map(|id| node_id_to_u64(*id) as u32)
            .collect()
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

    /// Insert text at the cursor position (alias for insertStr for compatibility).
    #[napi]
    pub fn insert_text(&mut self, text: String) {
        self.engine.insert_str(&text);
    }

    /// Delete the character before the cursor (backspace).
    #[napi]
    pub fn delete_char(&mut self) {
        self.engine.delete_char();
    }

    /// Delete character after the cursor (forward delete).
    #[napi]
    pub fn delete_char_forward(&mut self) {
        let pos = self.engine.cursor().position();
        let total = self.engine.char_count();
        if pos < total {
            self.engine.buffer_mut().delete_char(pos);
        }
    }

    /// Delete the word before the cursor.
    #[napi]
    pub fn delete_word_backward(&mut self) {
        let pos = self.engine.cursor().position();
        let boundary = self.engine.buffer().word_boundary_left(pos);
        if boundary < pos {
            self.engine.buffer_mut().delete_range(boundary, pos);
            self.engine.cursor_mut().set_position(boundary);
        }
    }

    /// Delete the word after the cursor.
    #[napi]
    pub fn delete_word_forward(&mut self) {
        let pos = self.engine.cursor().position();
        let boundary = self.engine.buffer().word_boundary_right(pos);
        if boundary > pos {
            self.engine.buffer_mut().delete_range(pos, boundary);
        }
    }

    /// Delete from cursor to start of line.
    #[napi]
    pub fn delete_line_backward(&mut self) {
        let pos = self.engine.cursor().position();
        let line = self.engine.buffer().char_to_line(pos);
        let line_start = self.engine.buffer().line_to_char(line);
        if line_start < pos {
            self.engine.buffer_mut().delete_range(line_start, pos);
            self.engine.cursor_mut().set_position(line_start);
        }
    }

    /// Delete from cursor to end of line.
    #[napi]
    pub fn delete_line_forward(&mut self) {
        let pos = self.engine.cursor().position();
        let line = self.engine.buffer().char_to_line(pos);
        let line_end = if line + 1 < self.engine.line_count() {
            self.engine.buffer().line_to_char(line + 1)
        } else {
            self.engine.char_count()
        };
        if pos < line_end {
            self.engine.buffer_mut().delete_range(pos, line_end);
        }
    }

    /// Move cursor left one character.
    #[napi]
    pub fn cursor_left(&mut self) {
        if self.engine.cursor().position() > 0 {
            self.engine.cursor_mut().move_left();
        }
    }

    /// Move cursor right one character.
    #[napi]
    pub fn cursor_right(&mut self) {
        if self.engine.cursor().position() < self.engine.char_count() {
            self.engine.cursor_mut().move_right();
        }
    }

    /// Move cursor up one line.
    #[napi]
    pub fn cursor_up(&mut self) {
        let line = self
            .engine
            .buffer()
            .char_to_line(self.engine.cursor().position());
        if line > 0 {
            let current_line_start = self.engine.buffer().line_to_char(line);
            let prev_line_start = self.engine.buffer().line_to_char(line - 1);
            let prev_line_len = current_line_start - prev_line_start - 1;
            let col = self.engine.cursor().position() - current_line_start;
            let new_col = col.min(prev_line_len);
            self.engine
                .cursor_mut()
                .set_position(prev_line_start + new_col);
        }
    }

    /// Move cursor down one line.
    #[napi]
    pub fn cursor_down(&mut self) {
        let line = self
            .engine
            .buffer()
            .char_to_line(self.engine.cursor().position());
        if line + 1 < self.engine.line_count() {
            let current_line_start = self.engine.buffer().line_to_char(line);
            let next_line_start = self.engine.buffer().line_to_char(line + 1);
            let next_line_len = if line + 2 < self.engine.line_count() {
                self.engine.buffer().line_to_char(line + 2) - next_line_start - 1
            } else {
                self.engine.char_count() - next_line_start
            };
            let col = self.engine.cursor().position() - current_line_start;
            let new_col = col.min(next_line_len);
            self.engine
                .cursor_mut()
                .set_position(next_line_start + new_col);
        }
    }

    /// Move cursor to start of the current line.
    #[napi]
    pub fn cursor_line_start(&mut self) {
        let line = self
            .engine
            .buffer()
            .char_to_line(self.engine.cursor().position());
        let line_start = self.engine.buffer().line_to_char(line);
        self.engine.cursor_mut().set_position(line_start);
    }

    /// Move cursor to end of the current line.
    #[napi]
    pub fn cursor_line_end(&mut self) {
        let line = self
            .engine
            .buffer()
            .char_to_line(self.engine.cursor().position());
        let line_end = if line + 1 < self.engine.line_count() {
            self.engine.buffer().line_to_char(line + 1) - 1
        } else {
            self.engine.char_count()
        };
        self.engine.cursor_mut().set_position(line_end);
    }

    /// Insert text at a specific position.
    #[napi]
    pub fn insert_at(&mut self, position: u32, text: String) {
        let pos = position as usize;
        if pos <= self.engine.char_count() {
            self.engine.buffer_mut().insert_str(pos, &text);
        }
    }

    /// Delete a range of characters. Returns the deleted text.
    #[napi]
    pub fn delete_at(&mut self, position: u32, length: u32) -> String {
        let pos = position as usize;
        let end = pos + length as usize;
        let max = self.engine.char_count();
        let actual_end = end.min(max);
        if pos < actual_end {
            let deleted = self.engine.buffer().substring(pos, actual_end);
            self.engine.buffer_mut().delete_range(pos, actual_end);
            deleted
        } else {
            String::new()
        }
    }

    /// Get the character at a position.
    #[napi]
    pub fn char_at(&self, position: u32) -> String {
        let result = self.engine.buffer().char_at(position as usize);
        if result == '\0' {
            String::new()
        } else {
            result.to_string()
        }
    }

    /// Get a substring of the text buffer.
    #[napi]
    pub fn substring(&self, start: u32, end: u32) -> String {
        self.engine.buffer().substring(start as usize, end as usize)
    }

    /// Replace all occurrences of a pattern with replacement. Returns number of replacements.
    #[napi]
    pub fn replace_all(
        &mut self,
        pattern: String,
        replacement: String,
        case_sensitive: bool,
    ) -> u32 {
        use bettertui_engine::text::SearchOptions;
        let options = SearchOptions {
            case_sensitive,
            ..Default::default()
        };
        self.engine.replace(&pattern, &replacement, options) as u32
    }

    /// Check if undo is available.
    #[napi]
    pub fn can_undo(&self) -> bool {
        self.engine.undo_manager().can_undo()
    }

    /// Check if redo is available.
    #[napi]
    pub fn can_redo(&self) -> bool {
        self.engine.undo_manager().can_redo()
    }

    /// Get the total length in characters.
    #[napi]
    pub fn length(&self) -> u32 {
        self.engine.char_count() as u32
    }

    /// Get all lines as a string array.
    #[napi]
    pub fn lines(&self) -> Vec<String> {
        let mut result = Vec::new();
        for i in 0..self.engine.line_count() {
            result.push(self.engine.line(i).unwrap_or_default());
        }
        result
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

    /// Get the current FPS.
    #[napi]
    pub fn fps(&self) -> String {
        let interval = self.scheduler.frame_budget().target_frame_time;
        if interval.is_zero() {
            "0".into()
        } else {
            (1000 / interval.as_millis()).to_string()
        }
    }

    /// Get the frame budget in milliseconds.
    #[napi]
    pub fn frame_budget_ms(&self) -> String {
        self.scheduler
            .frame_budget()
            .target_frame_time
            .as_millis()
            .to_string()
    }

    /// Check if the scheduler is idle.
    #[napi]
    pub fn is_idle(&self) -> bool {
        matches!(
            self.scheduler.status(),
            bettertui_engine::scheduler::FrameStatus::Idle
        )
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
    osc8: bool,
    synchronized_output: bool,
    underline_color: bool,
    strikethrough: bool,
    cursor_style: bool,
    alternate_scroll: bool,
    kitty_graphics: bool,
    sixel: bool,
    iterm_images: bool,
    focus_events: bool,
    csi_u: bool,
    term_width: u32,
    term_height: u32,
    pixel_width: u32,
    pixel_height: u32,
    has_pixel_size: bool,
}

#[napi]
impl NapiCapabilities {
    #[napi(factory)]
    pub fn detect() -> Self {
        let caps = bettertui_engine::capabilities::global_capabilities();
        let (w, h) = caps.terminal_size();
        let pixel = caps.pixel_size();
        Self {
            brand: format!("{:?}", caps.brand()),
            true_color: caps.supports_true_color(),
            kitty_keyboard: caps.supports_kitty_keyboard(),
            bracketed_paste: caps.supports_bracketed_paste(),
            mouse_support: caps.input.mouse_modes.normal_mouse,
            osc52_clipboard: caps.supports_osc52(),
            osc8: caps.supports_osc8(),
            synchronized_output: caps.features().synchronized_output,
            underline_color: caps.features().underline_color,
            strikethrough: caps.features().strikethrough,
            cursor_style: caps.features().cursor_style,
            alternate_scroll: caps.features().alternate_scroll,
            kitty_graphics: caps.supports_kitty_graphics(),
            sixel: caps.supports_sixel(),
            iterm_images: caps.supports_iterm_images(),
            focus_events: caps.supports_focus_events(),
            csi_u: caps.supports_csi_u(),
            term_width: w as u32,
            term_height: h as u32,
            pixel_width: pixel.map_or(0, |p| p.0),
            pixel_height: pixel.map_or(0, |p| p.1),
            has_pixel_size: pixel.is_some(),
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

    #[napi(getter)]
    pub fn get_pixel_size(&self) -> Vec<u32> {
        if self.has_pixel_size {
            vec![self.pixel_width, self.pixel_height]
        } else {
            vec![]
        }
    }

    #[napi(getter)]
    pub fn get_osc8(&self) -> bool {
        self.osc8
    }

    #[napi(getter)]
    pub fn get_synchronized_output(&self) -> bool {
        self.synchronized_output
    }

    #[napi(getter)]
    pub fn get_underline_color(&self) -> bool {
        self.underline_color
    }

    #[napi(getter)]
    pub fn get_strikethrough(&self) -> bool {
        self.strikethrough
    }

    #[napi(getter)]
    pub fn get_cursor_style(&self) -> bool {
        self.cursor_style
    }

    #[napi(getter)]
    pub fn get_alternate_scroll(&self) -> bool {
        self.alternate_scroll
    }

    #[napi(getter)]
    pub fn get_kitty_graphics(&self) -> bool {
        self.kitty_graphics
    }

    #[napi(getter)]
    pub fn get_sixel(&self) -> bool {
        self.sixel
    }

    #[napi(getter)]
    pub fn get_iterm_images(&self) -> bool {
        self.iterm_images
    }

    #[napi(getter)]
    pub fn get_focus_events(&self) -> bool {
        self.focus_events
    }

    #[napi(getter)]
    pub fn get_csi_u(&self) -> bool {
        self.csi_u
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
    let pixel = caps.pixel_size();
    let features = caps.features();
    serde_json::json!({
        "brand": format!("{:?}", caps.brand()),
        "trueColor": caps.supports_true_color(),
        "kittyKeyboard": caps.supports_kitty_keyboard(),
        "csi_u": caps.supports_csi_u(),
        "bracketedPaste": caps.supports_bracketed_paste(),
        "focusEvents": caps.supports_focus_events(),
        "mouse": caps.input.mouse_modes.normal_mouse,
        "osc52": caps.supports_osc52(),
        "osc8": caps.supports_osc8(),
        "sync": features.synchronized_output,
        "sgrPixel": caps.supports_kitty_graphics(),
        "underlineColor": features.underline_color,
        "strikethrough": features.strikethrough,
        "cursorStyle": features.cursor_style,
        "alternateScroll": features.alternate_scroll,
        "inlineImages": caps.supports_iterm_images(),
        "sixel": caps.supports_sixel(),
        "terminalSize": { "width": w, "height": h },
        "pixelSize": pixel.map(|(pw, ph)| { serde_json::json!({ "width": pw, "height": ph }) }),
    })
    .to_string()
}

/// Highlight source code using tree-sitter syntax highlighting.
///
/// Returns a JSON array of lines, each line being an array of segments.
/// Each segment has `text`, `fg`, `bg`, `bold`, `italic`, `underline`, and `dim` fields.
/// Colors are hex strings like `"#ff7b72"` or named strings like `"red"`.
#[napi]
pub fn highlight_code(code: String, language: String) -> String {
    let mut hl = bettertui_engine::syntax::global_highlighter()
        .lock()
        .unwrap();
    let lines = hl.highlight(&code, &language);
    match lines {
        Some(lines) => {
            let json_lines: Vec<serde_json::Value> = lines
                .iter()
                .map(|line| {
                    let segments: Vec<serde_json::Value> = line
                        .segments
                        .iter()
                        .map(|seg| {
                            let fg = seg.style.fg.map(color_to_hex);
                            let bg = seg.style.bg.map(color_to_hex);
                            serde_json::json!({
                                "text": seg.text,
                                "fg": fg,
                                "bg": bg,
                                "bold": seg.style.bold,
                                "italic": seg.style.italic,
                                "underline": seg.style.underline,
                                "dim": seg.style.dim,
                                "strikethrough": seg.style.strikethrough,
                            })
                        })
                        .collect();
                    serde_json::json!(segments)
                })
                .collect();
            serde_json::to_string(&json_lines).unwrap_or_default()
        }
        None => "[]".to_string(),
    }
}

fn color_to_hex(c: bettertui_engine::tree::color::Color) -> String {
    c.to_rgba(255).to_hex()
}
