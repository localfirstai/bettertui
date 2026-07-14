//! Rendering pipeline: Renderer, RenderFrame, RenderBackend, AnsiBackend, Painter, RenderPipeline, RenderTree.

use std::cell::RefCell;
use std::collections::HashMap;

use tracing::{debug, info};

use crate::dirty_diff::{DirtyDiff, DirtyRegion};
use crate::framebuffer::{Cell, CellAttributes, FrameBuffer};
use crate::layout::build_render_tree_with_viewport;
use crate::layout::{ClipBounds, LayoutTreeSync, PaintBounds, PaintContext, PaintFlags, Viewport};
use crate::scheduler::{FrameStatus, Scheduler};
use crate::text::{TextAlign, ViewportConfig, layout_text};
use crate::tree::NodeArena;
use crate::tree::{Color, NamedColor, NodeId, Overflow, Rect, ResolvedStyle};

// ═══════════════════════════════════════════════════════════════════════════════
// === backend.rs ===
// ═══════════════════════════════════════════════════════════════════════════════

pub trait RenderBackend {
    fn encode(&mut self, buffer: &FrameBuffer, regions: &[DirtyRegion]);
    fn finish(&self) -> &[u8];
    fn reset(&mut self);
    fn set_cursor_position(&mut self, x: u16, y: u16, visible: bool);
}

// ═══════════════════════════════════════════════════════════════════════════════
// === ansi.rs ===
// ═══════════════════════════════════════════════════════════════════════════════

pub struct AnsiBackend {
    buffer: Vec<u8>,
    cursor_x: u16,
    cursor_y: u16,
}

impl Default for AnsiBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl AnsiBackend {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(4096),
            cursor_x: u16::MAX,
            cursor_y: u16::MAX,
        }
    }

    fn encode_region(&mut self, buffer: &FrameBuffer, region: &DirtyRegion) {
        for y in region.y..region.y + region.height {
            self.move_to(region.x, y);

            // Run-length coalescing: batch consecutive same-styled cells
            let mut x = region.x;
            while x < region.x + region.width {
                let cell = buffer.get(x, y);
                let run_start = x;
                x += 1;
                while x < region.x + region.width {
                    let next = buffer.get(x, y);
                    if next.fg == cell.fg
                        && next.bg == cell.bg
                        && next.attributes == cell.attributes
                    {
                        x += 1;
                    } else {
                        break;
                    }
                }
                let run_len = x - run_start;

                // Emit SGR once for entire run
                self.encode_cell(&cell);

                // Emit all characters in the run
                for cx in run_start..run_start + run_len {
                    self.push_char(buffer.get(cx, y).ch);
                }
                self.cursor_x += run_len;
            }
        }
    }

    fn encode_cell(&mut self, cell: &Cell) {
        self.begin_sgr();
        self.push_fg_sgr(cell.fg);
        self.push_bg_sgr(cell.bg);
        self.push_attrs_sgr(cell.attributes);
        self.end_sgr();
    }

    fn begin_sgr(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[");
    }

    fn end_sgr(&mut self) {
        self.buffer.push(b'm');
    }

    fn push_fg_sgr(&mut self, color: Color) {
        match color {
            Color::Default => self.push_param(39),
            Color::Named(named) => {
                let code = match named {
                    NamedColor::Black => 30,
                    NamedColor::Red => 31,
                    NamedColor::Green => 32,
                    NamedColor::Yellow => 33,
                    NamedColor::Blue => 34,
                    NamedColor::Magenta => 35,
                    NamedColor::Cyan => 36,
                    NamedColor::White => 37,
                    NamedColor::BrightBlack => 90,
                    NamedColor::BrightRed => 91,
                    NamedColor::BrightGreen => 92,
                    NamedColor::BrightYellow => 93,
                    NamedColor::BrightBlue => 94,
                    NamedColor::BrightMagenta => 95,
                    NamedColor::BrightCyan => 96,
                    NamedColor::BrightWhite => 97,
                };
                self.push_param(code);
            }
            Color::Rgb { r, g, b } => {
                self.push_param(38);
                self.push_param(2);
                self.push_param(r as u32);
                self.push_param(g as u32);
                self.push_param(b as u32);
            }
            Color::Indexed(i) => {
                self.push_param(38);
                self.push_param(5);
                self.push_param(i as u32);
            }
        }
    }

    fn push_bg_sgr(&mut self, color: Color) {
        match color {
            Color::Default => self.push_param(49),
            Color::Named(named) => {
                let code = match named {
                    NamedColor::Black => 40,
                    NamedColor::Red => 41,
                    NamedColor::Green => 42,
                    NamedColor::Yellow => 43,
                    NamedColor::Blue => 44,
                    NamedColor::Magenta => 45,
                    NamedColor::Cyan => 46,
                    NamedColor::White => 47,
                    NamedColor::BrightBlack => 100,
                    NamedColor::BrightRed => 101,
                    NamedColor::BrightGreen => 102,
                    NamedColor::BrightYellow => 103,
                    NamedColor::BrightBlue => 104,
                    NamedColor::BrightMagenta => 105,
                    NamedColor::BrightCyan => 106,
                    NamedColor::BrightWhite => 107,
                };
                self.push_param(code);
            }
            Color::Rgb { r, g, b } => {
                self.push_param(48);
                self.push_param(2);
                self.push_param(r as u32);
                self.push_param(g as u32);
                self.push_param(b as u32);
            }
            Color::Indexed(i) => {
                self.push_param(48);
                self.push_param(5);
                self.push_param(i as u32);
            }
        }
    }

    fn push_attrs_sgr(&mut self, attrs: CellAttributes) {
        if attrs.contains(CellAttributes::BOLD) {
            self.push_param(1);
        }
        if attrs.contains(CellAttributes::DIM) {
            self.push_param(2);
        }
        if attrs.contains(CellAttributes::ITALIC) {
            self.push_param(3);
        }
        if attrs.contains(CellAttributes::UNDERLINE) {
            self.push_param(4);
        }
        if attrs.contains(CellAttributes::STRIKETHROUGH) {
            self.push_param(9);
        }
        if attrs.contains(CellAttributes::INVERSE) {
            self.push_param(7);
        }
        if attrs.contains(CellAttributes::HIDDEN) {
            self.push_param(8);
        }
    }

    fn push_param(&mut self, n: u32) {
        if !self.buffer.ends_with(b"[") && !self.buffer.ends_with(b";") {
            self.buffer.push(b';');
        }
        let mut buf = [0u8; 10];
        let mut i = buf.len();
        let mut val = n;
        if val == 0 {
            i -= 1;
            buf[i] = b'0';
        } else {
            while val > 0 {
                i -= 1;
                buf[i] = b'0' + (val % 10) as u8;
                val /= 10;
            }
        }
        self.buffer.extend_from_slice(&buf[i..]);
    }

    fn push_char(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        let s = ch.encode_utf8(&mut buf);
        self.buffer.extend_from_slice(s.as_bytes());
    }

    fn move_to(&mut self, x: u16, y: u16) {
        if x == self.cursor_x && y == self.cursor_y {
            return;
        }
        self.buffer.extend_from_slice(b"\x1b[");
        self.push_u16(y + 1);
        self.buffer.push(b';');
        self.push_u16(x + 1);
        self.buffer.push(b'H');
        self.cursor_x = x;
        self.cursor_y = y;
    }

    fn push_u16(&mut self, n: u16) {
        if n == 0 {
            self.buffer.push(b'0');
            return;
        }
        let mut buf = [0u8; 5];
        let mut i = buf.len();
        let mut val = n;
        while val > 0 {
            i -= 1;
            buf[i] = b'0' + (val % 10) as u8;
            val /= 10;
        }
        self.buffer.extend_from_slice(&buf[i..]);
    }

    fn hide_cursor(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?25l");
    }

    fn show_cursor(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?25h");
    }

    pub fn reset_sgr(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[0m");
    }
}

impl RenderBackend for AnsiBackend {
    fn encode(&mut self, buffer: &FrameBuffer, regions: &[DirtyRegion]) {
        self.buffer.clear();
        self.cursor_x = u16::MAX;
        self.cursor_y = u16::MAX;

        if regions.is_empty() {
            return;
        }

        self.hide_cursor();

        for region in regions {
            self.encode_region(buffer, region);
        }
    }

    fn finish(&self) -> &[u8] {
        &self.buffer
    }

    fn reset(&mut self) {
        self.buffer.clear();
    }

    fn set_cursor_position(&mut self, x: u16, y: u16, visible: bool) {
        self.move_to(x, y);
        if visible {
            self.show_cursor();
        } else {
            self.hide_cursor();
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// === object.rs ===
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct RenderObject {
    pub id: NodeId,
    pub bounds: PaintBounds,
    pub clip: Option<ClipBounds>,
    pub style: ResolvedStyle,
    pub opacity: f32,
    pub z_index: i32,
    pub translate_x: i32,
    pub translate_y: i32,
    pub text: Option<Box<str>>,
    pub text_align: TextAlign,
    pub text_wrap: bool,
    pub overflow: Overflow,
    pub flags: PaintFlags,
}

impl RenderObject {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            bounds: PaintBounds::default(),
            clip: None,
            style: ResolvedStyle::default(),
            opacity: 1.0,
            z_index: 0,
            translate_x: 0,
            translate_y: 0,
            text: None,
            text_align: TextAlign::Left,
            text_wrap: false,
            overflow: Overflow::Visible,
            flags: PaintFlags::empty(),
        }
    }

    pub fn has_background(&self) -> bool {
        self.style.bg.is_some()
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    pub fn is_visible(&self) -> bool {
        self.opacity > 0.0 && !self.flags.contains(PaintFlags::HIDDEN)
    }

    pub fn translated_bounds(&self) -> PaintBounds {
        let mut b = self.bounds;
        b.x = (b.x as i32 + self.translate_x).max(0) as u16;
        b.y = (b.y as i32 + self.translate_y).max(0) as u16;
        b
    }

    pub fn content_rect(&self) -> Rect {
        let b = &self.bounds;
        Rect::new(
            b.x + b.padding_left,
            b.y + b.padding_top,
            b.width.saturating_sub(b.padding_left + b.padding_right),
            b.height.saturating_sub(b.padding_top + b.padding_bottom),
        )
    }
}

/// Render commands for structured rendering pipeline.
///
/// This enum matches OpenTUI's render command pattern, allowing
/// proper stacking of scissor rects and opacity values.
#[derive(Debug, Clone)]
pub enum RenderCommand {
    /// Render a renderable object
    Render { object: RenderObject },
    /// Push a scissor rect for clipping
    PushScissorRect {
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    },
    /// Pop the top scissor rect
    PopScissorRect,
    /// Push an opacity value (multiplied with current)
    PushOpacity { opacity: f32 },
    /// Pop the top opacity value
    PopOpacity,
}

impl RenderCommand {
    pub fn render(obj: RenderObject) -> Self {
        Self::Render { object: obj }
    }

    pub fn push_scissor(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self::PushScissorRect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn pop_scissor() -> Self {
        Self::PopScissorRect
    }

    pub fn push_opacity(opacity: f32) -> Self {
        Self::PushOpacity { opacity }
    }

    pub fn pop_opacity() -> Self {
        Self::PopOpacity
    }
}

#[derive(Debug, Clone)]
pub struct RenderTree {
    objects: Vec<RenderObject>,
    index: HashMap<NodeId, usize>,
    root: Option<NodeId>,
    sorted_cache: RefCell<Option<Vec<usize>>>,
}

impl Default for RenderTree {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderTree {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            index: HashMap::new(),
            root: None,
            sorted_cache: RefCell::new(None),
        }
    }

    pub fn push(&mut self, obj: RenderObject) {
        let idx = self.objects.len();
        if self.root.is_none() {
            self.root = Some(obj.id);
        }
        self.index.insert(obj.id, idx);
        self.objects.push(obj);
        *self.sorted_cache.borrow_mut() = None;
    }

    pub fn get(&self, id: NodeId) -> Option<&RenderObject> {
        self.index.get(&id).and_then(|&idx| self.objects.get(idx))
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut RenderObject> {
        self.index
            .get(&id)
            .copied()
            .and_then(|idx| self.objects.get_mut(idx))
    }

    pub fn root(&self) -> Option<NodeId> {
        self.root
    }

    pub fn objects(&self) -> &[RenderObject] {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut [RenderObject] {
        &mut self.objects
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &RenderObject> {
        self.objects.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut RenderObject> {
        self.objects.iter_mut()
    }

    pub fn sorted_by_z_index(&self) -> Vec<usize> {
        let mut cache = self.sorted_cache.borrow_mut();
        if let Some(ref cached) = *cache {
            return cached.clone();
        }
        let mut indices: Vec<usize> = (0..self.objects.len()).collect();
        indices.sort_by_key(|&i| self.objects[i].z_index);
        *cache = Some(indices.clone());
        indices
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.index.clear();
        self.root = None;
        *self.sorted_cache.borrow_mut() = None;
    }

    /// Collect render commands with proper scissor/opacity stacking.
    ///
    /// This follows OpenTUI's pattern:
    /// 1. Push opacity if < 1.0
    /// 2. Push render command
    /// 3. Push scissor rect if overflow !== visible
    /// 4. Process children
    /// 5. Pop scissor rect
    /// 6. Pop opacity
    pub fn collect_commands(&self) -> Vec<RenderCommand> {
        let mut commands = Vec::with_capacity(self.objects.len() * 2);
        let sorted = self.sorted_by_z_index();

        for &idx in &sorted {
            let obj = &self.objects[idx];
            if !obj.is_visible() {
                continue;
            }

            let needs_opacity = obj.opacity < 1.0;
            let needs_scissor = obj.clip.is_some();

            if needs_opacity {
                commands.push(RenderCommand::push_opacity(obj.opacity));
            }

            commands.push(RenderCommand::render(obj.clone()));

            if let Some(clip) = &obj.clip {
                commands.push(RenderCommand::push_scissor(
                    clip.x,
                    clip.y,
                    clip.width,
                    clip.height,
                ));
            }

            if needs_scissor {
                commands.push(RenderCommand::pop_scissor());
            }

            if needs_opacity {
                commands.push(RenderCommand::pop_opacity());
            }
        }

        commands
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// === painter.rs ===
// ═══════════════════════════════════════════════════════════════════════════════

pub struct Painter {
    buffer: FrameBuffer,
    opacity_stack: Vec<f32>,
    scissor_stack: Vec<ClipBounds>,
}

impl Default for Painter {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Painter {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buffer: FrameBuffer::new(width, height),
            opacity_stack: Vec::with_capacity(8),
            scissor_stack: Vec::with_capacity(8),
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer.resize(width, height);
    }

    pub fn buffer(&self) -> &FrameBuffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut FrameBuffer {
        &mut self.buffer
    }

    /// Get current effective opacity (product of all stacked values).
    pub fn current_opacity(&self) -> f32 {
        self.opacity_stack.last().copied().unwrap_or(1.0)
    }

    /// Push an opacity value onto the stack.
    pub fn push_opacity(&mut self, opacity: f32) {
        let current = self.current_opacity();
        let effective = (current * opacity).clamp(0.0, 1.0);
        self.opacity_stack.push(effective);
    }

    /// Pop an opacity value from the stack.
    pub fn pop_opacity(&mut self) {
        self.opacity_stack.pop();
    }

    /// Push a scissor rect onto the stack.
    /// Intersects with current scissor if any.
    pub fn push_scissor(&mut self, x: u16, y: u16, width: u16, height: u16) {
        let new_clip = ClipBounds::new(x, y, width, height);
        if let Some(current) = self.scissor_stack.last() {
            if let Some(intersected) = current.intersect(&new_clip) {
                self.scissor_stack.push(intersected);
            } else {
                self.scissor_stack.push(ClipBounds::new(0, 0, 0, 0));
            }
        } else {
            self.scissor_stack.push(new_clip);
        }
    }

    /// Pop a scissor rect from the stack.
    pub fn pop_scissor(&mut self) {
        self.scissor_stack.pop();
    }

    /// Get current scissor rect if any.
    pub fn current_scissor(&self) -> Option<&ClipBounds> {
        self.scissor_stack.last()
    }

    pub fn paint(&mut self, tree: &RenderTree, ctx: &PaintContext) {
        self.paint_with_clear(tree, ctx, true);
    }

    pub fn paint_with_clear(&mut self, tree: &RenderTree, ctx: &PaintContext, full_clear: bool) {
        if full_clear {
            self.buffer.clear();
        }
        let sorted = tree.sorted_by_z_index();
        for idx in &sorted {
            let obj = &tree.objects()[*idx];
            self.paint_object(obj, ctx);
        }
    }

    /// Paint from render commands with proper stacking.
    pub fn paint_commands(&mut self, commands: &[RenderCommand], ctx: &PaintContext) {
        self.buffer.clear();
        self.opacity_stack.clear();
        self.scissor_stack.clear();

        for command in commands {
            match command {
                RenderCommand::Render { object } => {
                    self.paint_object_with_scissor(object, ctx);
                }
                RenderCommand::PushScissorRect {
                    x,
                    y,
                    width,
                    height,
                } => {
                    self.push_scissor(*x, *y, *width, *height);
                }
                RenderCommand::PopScissorRect => {
                    self.pop_scissor();
                }
                RenderCommand::PushOpacity { opacity } => {
                    self.push_opacity(*opacity);
                }
                RenderCommand::PopOpacity => {
                    self.pop_opacity();
                }
            }
        }
    }

    fn paint_object_with_scissor(&mut self, obj: &RenderObject, ctx: &PaintContext) {
        if !obj.is_visible() {
            return;
        }

        let translated = obj.translated_bounds();
        let bounds = &translated;

        let effective_bounds = if let Some(scissor) = self.current_scissor() {
            if let Some(intersected) = scissor.intersect(&ClipBounds::new(
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
            )) {
                PaintBounds::new(
                    intersected.x,
                    intersected.y,
                    intersected.width,
                    intersected.height,
                )
                .with_padding(
                    bounds.padding_left,
                    bounds.padding_right,
                    bounds.padding_top,
                    bounds.padding_bottom,
                )
                .with_border(
                    bounds.border_top,
                    bounds.border_right,
                    bounds.border_bottom,
                    bounds.border_left,
                )
            } else {
                return;
            }
        } else {
            translated
        };

        if !ctx.is_visible(&effective_bounds) {
            return;
        }

        if let Some(clipped) = ctx.clipped_bounds(&effective_bounds) {
            self.paint_background(obj, &clipped);
            self.paint_border(obj, &clipped);
            self.paint_text(obj, &clipped);
        }
    }

    fn paint_object(&mut self, obj: &RenderObject, ctx: &PaintContext) {
        if !obj.is_visible() {
            return;
        }

        if let Some(clip) = &obj.clip {
            let mut child_ctx = ctx.clone();
            child_ctx.push_clip(*clip);
            self.paint_with_clip(obj, &child_ctx);
        } else {
            self.paint_with_clip(obj, ctx);
        }
    }

    fn paint_with_clip(&mut self, obj: &RenderObject, ctx: &PaintContext) {
        let translated = obj.translated_bounds();
        let bounds = &translated;

        if !ctx.is_visible(bounds) {
            return;
        }

        if let Some(clipped) = ctx.clipped_bounds(bounds) {
            self.paint_background(obj, &clipped);
            self.paint_border(obj, &clipped);
            self.paint_text(obj, &clipped);
        }
    }

    fn paint_background(&mut self, obj: &RenderObject, bounds: &PaintBounds) {
        if !obj.flags.contains(PaintFlags::BACKGROUND) {
            return;
        }

        let bg = obj.style.bg.unwrap_or(Color::Default);
        if bg == Color::Default {
            return;
        }

        let cell = Cell::new(' ').with_bg(bg);
        self.buffer
            .fill_rect(bounds.x, bounds.y, bounds.width, bounds.height, cell);
    }

    fn paint_text(&mut self, obj: &RenderObject, bounds: &PaintBounds) {
        // If node has a border, the text is used as the border title, so don't draw it inside
        let has_border = bounds.border_top > 0
            || bounds.border_right > 0
            || bounds.border_bottom > 0
            || bounds.border_left > 0;

        if has_border && obj.style.border_style != crate::tree::BorderStyle::None {
            return;
        }

        let text = match &obj.text {
            Some(t) => t.as_ref(),
            None => return,
        };

        let content = bounds.content_rect();
        let fg = obj.style.fg.unwrap_or(Color::Default);
        let bg = obj.style.bg.unwrap_or(Color::Default);
        let attrs = style_to_attrs(&obj.style);

        if text.is_empty() {
            return;
        }

        let config = ViewportConfig {
            wrap: obj.text_wrap,
            align: obj.text_align,
            max_width: content.width,
            max_height: content.height,
            pad_left: content.x,
            pad_top: content.y,
            ..ViewportConfig::default()
        };

        let layout = layout_text(text, &config);

        for line in &layout.lines {
            let y = line.y;
            if y >= self.buffer.height() {
                break;
            }
            let mut col = line.x;
            for g in unicode_segmentation::UnicodeSegmentation::graphemes(line.text.as_str(), true)
            {
                if col >= content.x + content.width {
                    break;
                }
                if col >= self.buffer.width() {
                    break;
                }
                let w = unicode_width::UnicodeWidthStr::width(g) as u16;
                if let Some(ch) = g.chars().next() {
                    let cell = Cell::new(ch).with_fg(fg).with_bg(bg).with_attrs(attrs);
                    self.buffer.set(col, y, cell);
                    if w == 2 && col + 1 < self.buffer.width() {
                        let space = Cell::new(' ').with_fg(fg).with_bg(bg).with_attrs(attrs);
                        self.buffer.set(col + 1, y, space);
                    }
                }
                col += w;
            }
        }
    }

    fn paint_border(&mut self, obj: &RenderObject, bounds: &PaintBounds) {
        let border_style = obj.style.border_style;
        if border_style == crate::tree::BorderStyle::None {
            return;
        }

        let has_border = bounds.border_top > 0
            || bounds.border_right > 0
            || bounds.border_bottom > 0
            || bounds.border_left > 0;
        if !has_border {
            return;
        }

        let fg = obj
            .style
            .border_color
            .unwrap_or(obj.style.fg.unwrap_or(Color::Default));
        let bg = obj.style.bg.unwrap_or(Color::Default);
        let attrs = style_to_attrs(&obj.style);

        let (tl, tr, bl, br, horiz, vert) = match border_style {
            crate::tree::BorderStyle::Solid => ('┌', '┐', '└', '┘', '─', '│'),
            crate::tree::BorderStyle::Double => ('╔', '╗', '╚', '╝', '═', '║'),
            crate::tree::BorderStyle::Dashed => ('┌', '┐', '└', '┘', '╌', '╎'),
            crate::tree::BorderStyle::Dotted => ('╭', '╮', '╰', '╯', '─', '│'),
            crate::tree::BorderStyle::None => return,
        };

        let x = bounds.x;
        let y = bounds.y;
        let w = bounds.width;
        let h = bounds.height;

        // Draw top border
        if bounds.border_top > 0 {
            for row in 0..bounds.border_top {
                for col in 0..w {
                    let ch = if row == 0 {
                        if col == 0 && bounds.border_left > 0 {
                            tl
                        } else if col == w - 1 && bounds.border_right > 0 {
                            tr
                        } else if bounds.border_top > 1 && row < bounds.border_top - 1 {
                            ' '
                        } else {
                            horiz
                        }
                    } else {
                        ' '
                    };

                    if col < self.buffer.width() && y + row < self.buffer.height() {
                        let cell = Cell::new(ch).with_fg(fg).with_bg(bg).with_attrs(attrs);
                        self.buffer.set(x + col, y + row, cell);
                    }
                }
            }
        }

        // Draw bottom border
        if bounds.border_bottom > 0 {
            for row in 0..bounds.border_bottom {
                for col in 0..w {
                    let ch = if row == bounds.border_bottom - 1 {
                        if col == 0 && bounds.border_left > 0 {
                            bl
                        } else if col == w - 1 && bounds.border_right > 0 {
                            br
                        } else {
                            horiz
                        }
                    } else {
                        ' '
                    };
                    let draw_y = y + h - 1 - (bounds.border_bottom - 1 - row);
                    if col < self.buffer.width() && draw_y < self.buffer.height() {
                        let cell = Cell::new(ch).with_fg(fg).with_bg(bg).with_attrs(attrs);
                        self.buffer.set(x + col, draw_y, cell);
                    }
                }
            }
        }

        // Draw left border
        if bounds.border_left > 0 {
            for row in 0..h {
                for col in 0..bounds.border_left {
                    let ch = if col == 0 {
                        if row == 0 && bounds.border_top > 0 {
                            tl
                        } else if row == h - 1 && bounds.border_bottom > 0 {
                            bl
                        } else {
                            vert
                        }
                    } else {
                        ' '
                    };
                    if x + col < self.buffer.width() && y + row < self.buffer.height() {
                        let cell = Cell::new(ch).with_fg(fg).with_bg(bg).with_attrs(attrs);
                        self.buffer.set(x + col, y + row, cell);
                    }
                }
            }
        }

        // Draw right border
        if bounds.border_right > 0 {
            for row in 0..h {
                for col in 0..bounds.border_right {
                    let ch = if col == bounds.border_right - 1 {
                        if row == 0 && bounds.border_top > 0 {
                            tr
                        } else if row == h - 1 && bounds.border_bottom > 0 {
                            br
                        } else {
                            vert
                        }
                    } else {
                        ' '
                    };
                    let draw_x = x + w - 1 - (bounds.border_right - 1 - col);
                    if draw_x < self.buffer.width() && y + row < self.buffer.height() {
                        let cell = Cell::new(ch).with_fg(fg).with_bg(bg).with_attrs(attrs);
                        self.buffer.set(draw_x, y + row, cell);
                    }
                }
            }
        }
    }

    pub fn swap(&mut self) {
        self.buffer.swap();
    }

    pub fn diff(&self) -> Vec<(u16, u16)> {
        self.buffer.diff()
    }
}

fn style_to_attrs(style: &ResolvedStyle) -> CellAttributes {
    let mut attrs = CellAttributes::empty();
    if style.bold {
        attrs |= CellAttributes::BOLD;
    }
    if style.italic {
        attrs |= CellAttributes::ITALIC;
    }
    if style.underline {
        attrs |= CellAttributes::UNDERLINE;
    }
    if style.dim {
        attrs |= CellAttributes::DIM;
    }
    if style.strikethrough {
        attrs |= CellAttributes::STRIKETHROUGH;
    }
    if style.inverse {
        attrs |= CellAttributes::INVERSE;
    }
    if style.hidden {
        attrs |= CellAttributes::HIDDEN;
    }
    attrs
}

// ═══════════════════════════════════════════════════════════════════════════════
// === pipeline.rs ===
// ═══════════════════════════════════════════════════════════════════════════════

/// Priority level for render pass ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PassPriority {
    /// Runs first — ideal for pre-processing (e.g. color adjustments).
    First = 0,
    /// Runs early in the pipeline.
    Early = 1,
    /// Default priority for most effects.
    Normal = 2,
    /// Runs late — ideal for overlay effects.
    Late = 3,
    /// Runs last — ideal for debug overlays, accessibility filters.
    Last = 4,
}

/// Context provided to each render pass on execution.
#[derive(Debug, Clone)]
pub struct RenderPassContext {
    pub width: u16,
    pub height: u16,
    pub delta_time: f32,
    pub frame_count: u64,
    pub generation: u64,
}

impl RenderPassContext {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            delta_time: 0.0,
            frame_count: 0,
            generation: 0,
        }
    }
}

/// A single render pass that transforms a framebuffer.
pub trait RenderPass: Send {
    fn name(&self) -> &str;

    fn execute(&mut self, buffer: &mut FrameBuffer, ctx: &RenderPassContext) -> PassResult;

    fn enabled(&self) -> bool;

    fn set_enabled(&mut self, enabled: bool);

    fn priority(&self) -> PassPriority;
}

/// An ordered pipeline of render passes.
pub struct RenderPipeline {
    passes: Vec<Box<dyn RenderPass>>,
    enabled: bool,
}

impl Default for RenderPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPipeline {
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            enabled: true,
        }
    }

    pub fn add_pass(&mut self, pass: Box<dyn RenderPass>) {
        self.passes.push(pass);
        self.resort();
    }

    pub fn remove_pass(&mut self, name: &str) {
        self.passes.retain(|p| p.name() != name);
    }

    pub fn get_pass(&self, name: &str) -> Option<&dyn RenderPass> {
        self.passes
            .iter()
            .find(|p| p.name() == name)
            .map(|p| p.as_ref())
    }

    pub fn get_pass_mut(&mut self, name: &str) -> Option<&mut dyn RenderPass> {
        self.passes
            .iter_mut()
            .find(|p| p.name() == name)
            .map(|p| p.as_mut() as &mut dyn RenderPass)
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn len(&self) -> usize {
        self.passes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.passes.is_empty()
    }

    pub fn passes(&self) -> &[Box<dyn RenderPass>] {
        &self.passes
    }

    /// Execute all enabled passes in priority order.
    ///
    /// Returns `PassResult::Modified` if ANY pass modified the buffer.
    /// Short-circuits only if a pass name matches the `stop_after` parameter.
    pub fn execute(&mut self, buffer: &mut FrameBuffer, ctx: &RenderPassContext) -> PassResult {
        if !self.enabled || self.passes.is_empty() {
            return PassResult::Unchanged;
        }

        let mut any_modified = false;
        for pass in &mut self.passes {
            if !pass.enabled() {
                continue;
            }
            let result = pass.execute(buffer, ctx);
            if result == PassResult::Modified {
                any_modified = true;
            }
        }

        if any_modified {
            PassResult::Modified
        } else {
            PassResult::Unchanged
        }
    }

    fn resort(&mut self) {
        self.passes.sort_by_key(|p| p.priority());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// === renderer.rs ===
// ═══════════════════════════════════════════════════════════════════════════════

/// Result of executing a render pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassResult {
    /// The pass did not change the buffer.
    Unchanged,
    /// The pass modified the buffer.
    Modified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderFrame {
    pub output_data: Vec<u8>,
    pub dirty_regions: Vec<DirtyRegion>,
    pub width: u16,
    pub height: u16,
}

impl RenderFrame {
    pub fn new_empty(width: u16, height: u16) -> Self {
        Self {
            output_data: Vec::new(),
            dirty_regions: Vec::new(),
            width,
            height,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.output_data.is_empty() && self.dirty_regions.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CursorState {
    pub x: u16,
    pub y: u16,
    pub visible: bool,
}

pub struct Renderer {
    width: u16,
    height: u16,
    layout_sync: LayoutTreeSync,
    render_tree: RenderTree,
    painter: Painter,
    snapshot: FrameBuffer,
    dirty_diff: DirtyDiff,
    backend: Box<dyn RenderBackend>,
    scheduler: Scheduler,
    pipeline: RenderPipeline,
    needs_full_repaint: bool,
    generation: u64,
    last_change_count: u64,
    cursor_state: CursorState,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        info!(width, height, "Renderer::new() - creating renderer");
        Self {
            width,
            height,
            layout_sync: LayoutTreeSync::new(),
            render_tree: RenderTree::new(),
            painter: Painter::new(width, height),
            snapshot: FrameBuffer::new(width, height),
            dirty_diff: DirtyDiff::new(),
            backend: Box::new(AnsiBackend::new()),
            scheduler: Scheduler::default(),
            pipeline: RenderPipeline::new(),
            needs_full_repaint: true,
            generation: 0,
            last_change_count: 0,
            cursor_state: CursorState::default(),
        }
    }

    pub fn with_backend(width: u16, height: u16, backend: Box<dyn RenderBackend>) -> Self {
        info!(
            width,
            height, "Renderer::with_backend() - creating renderer with custom backend"
        );
        Self {
            width,
            height,
            layout_sync: LayoutTreeSync::new(),
            render_tree: RenderTree::new(),
            painter: Painter::new(width, height),
            snapshot: FrameBuffer::new(width, height),
            dirty_diff: DirtyDiff::new(),
            backend,
            scheduler: Scheduler::default(),
            pipeline: RenderPipeline::new(),
            needs_full_repaint: true,
            generation: 0,
            last_change_count: 0,
            cursor_state: CursorState::default(),
        }
    }

    pub fn with_fps(fps: u32) -> Self {
        Self {
            scheduler: Scheduler::with_fps(fps),
            ..Self::new(80, 24)
        }
    }

    pub fn set_cursor_position(&mut self, x: u16, y: u16, visible: bool) {
        self.cursor_state = CursorState { x, y, visible };
    }

    pub fn cursor_state(&self) -> &CursorState {
        &self.cursor_state
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        let old_w = self.width;
        let old_h = self.height;
        self.width = width;
        self.height = height;
        self.painter.resize(width, height);
        self.snapshot.resize(width, height);
        self.needs_full_repaint = true;
        debug!(
            old_width = old_w,
            old_height = old_h,
            new_width = width,
            new_height = height,
            "Renderer::resize() - framebuffer resized"
        );
    }

    pub fn request_frame(&mut self) {
        self.scheduler.request_frame();
    }

    pub fn should_render(&self) -> FrameStatus {
        self.scheduler.status()
    }

    pub fn render(&mut self, arena: &mut NodeArena) -> RenderFrame {
        self.generation += 1;

        let change_count = arena.change_count();
        if !self.needs_full_repaint && change_count == self.last_change_count {
            debug!(
                generation = self.generation,
                "Renderer::render() - skipping frame (no changes)"
            );
            return RenderFrame::new_empty(self.width, self.height);
        }
        self.last_change_count = change_count;

        debug!(
            generation = self.generation,
            node_count = arena.len(),
            needs_full_repaint = self.needs_full_repaint,
            "Renderer::render() - rendering frame"
        );

        self.layout_sync.sync_full(arena);

        let root_id = arena.root();
        for (id, _node) in arena.iter() {
            let children = arena.children(id);
            if !children.is_empty() {
                self.layout_sync.sync_children(arena, id);
            }
        }
        let _ = self.layout_sync.compute(root_id, self.width, self.height);

        let vp = Viewport::new(0, 0, self.width, self.height);
        build_render_tree_with_viewport(
            arena,
            self.layout_sync.results(),
            Some(&vp),
            &mut self.render_tree,
        );

        let ctx = crate::layout::PaintContext::new(self.width, self.height);
        self.painter.paint(&self.render_tree, &ctx);

        // Post-processing: execute render passes on the painter's framebuffer
        let pp_ctx = RenderPassContext {
            width: self.width,
            height: self.height,
            delta_time: (1.0 / 60.0),
            frame_count: self.generation,
            generation: self.generation,
        };
        let pp_result = self.pipeline.execute(self.painter.buffer_mut(), &pp_ctx);

        let dirty_regions = if pp_result == PassResult::Modified {
            // Post-processing modified the buffer — re-diff from snapshot
            self.dirty_diff
                .compute(self.painter.buffer(), &self.snapshot, self.generation);
            self.dirty_diff.regions().to_vec()
        } else if self.needs_full_repaint {
            self.dirty_diff
                .compute_full_repaint(self.width, self.height);
            self.needs_full_repaint = false;
            self.dirty_diff.regions().to_vec()
        } else {
            self.dirty_diff
                .compute(self.painter.buffer(), &self.snapshot, self.generation);
            self.dirty_diff.regions().to_vec()
        };

        self.backend.encode(self.painter.buffer(), &dirty_regions);

        if self.cursor_state.visible {
            self.backend
                .set_cursor_position(self.cursor_state.x, self.cursor_state.y, true);
        }

        self.snapshot.copy_from(self.painter.buffer());

        self.scheduler.end_frame();

        arena.clear_dirty_flags();

        debug!(
            generation = self.generation,
            dirty_region_count = dirty_regions.len(),
            output_bytes = self.backend.finish().len(),
            "Renderer::render() - frame complete"
        );

        RenderFrame {
            output_data: self.backend.finish().to_vec(),
            dirty_regions,
            width: self.width,
            height: self.height,
        }
    }

    pub fn render_full(&mut self, arena: &mut NodeArena) -> RenderFrame {
        self.needs_full_repaint = true;
        self.render(arena)
    }

    pub fn set_backend(&mut self, backend: Box<dyn RenderBackend>) {
        self.backend = backend;
    }

    pub fn backend(&self) -> &dyn RenderBackend {
        self.backend.as_ref()
    }

    pub fn layout_sync(&self) -> &LayoutTreeSync {
        &self.layout_sync
    }

    pub fn render_tree(&self) -> &RenderTree {
        &self.render_tree
    }

    pub fn framebuffer(&self) -> &FrameBuffer {
        self.painter.buffer()
    }

    pub fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    pub fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }

    pub fn pipeline_mut(&mut self) -> &mut RenderPipeline {
        &mut self.pipeline
    }
}

#[cfg(test)]
mod render_command_tests {
    use super::*;

    #[test]
    fn render_command_variants() {
        let obj = RenderObject::new(NodeId::default());
        let cmd1 = RenderCommand::render(obj);
        assert!(matches!(cmd1, RenderCommand::Render { .. }));

        let cmd2 = RenderCommand::push_scissor(0, 0, 10, 10);
        assert!(matches!(cmd2, RenderCommand::PushScissorRect { .. }));

        let cmd3 = RenderCommand::pop_scissor();
        assert!(matches!(cmd3, RenderCommand::PopScissorRect));

        let cmd4 = RenderCommand::push_opacity(0.5);
        assert!(matches!(cmd4, RenderCommand::PushOpacity { .. }));

        let cmd5 = RenderCommand::pop_opacity();
        assert!(matches!(cmd5, RenderCommand::PopOpacity));
    }

    #[test]
    fn painter_opacity_stack() {
        let mut painter = Painter::new(10, 10);

        assert_eq!(painter.current_opacity(), 1.0);

        painter.push_opacity(0.5);
        assert_eq!(painter.current_opacity(), 0.5);

        painter.push_opacity(0.5);
        assert_eq!(painter.current_opacity(), 0.25);

        painter.pop_opacity();
        assert_eq!(painter.current_opacity(), 0.5);

        painter.pop_opacity();
        assert_eq!(painter.current_opacity(), 1.0);
    }

    #[test]
    fn painter_scissor_stack() {
        let mut painter = Painter::new(20, 20);

        assert!(painter.current_scissor().is_none());

        painter.push_scissor(0, 0, 10, 10);
        let clip = painter.current_scissor().unwrap();
        assert_eq!(clip.x, 0);
        assert_eq!(clip.width, 10);

        painter.push_scissor(5, 5, 10, 10);
        let clip = painter.current_scissor().unwrap();
        assert_eq!(clip.x, 5);
        assert_eq!(clip.width, 5);

        painter.pop_scissor();
        let clip = painter.current_scissor().unwrap();
        assert_eq!(clip.x, 0);

        painter.pop_scissor();
        assert!(painter.current_scissor().is_none());
    }

    #[test]
    fn render_tree_collect_commands() {
        let mut tree = RenderTree::new();

        let mut obj1 = RenderObject::new(NodeId::default());
        obj1.opacity = 0.8;
        obj1.z_index = 1;
        tree.push(obj1);

        let mut obj2 = RenderObject::new(NodeId::default());
        obj2.opacity = 1.0;
        obj2.clip = Some(ClipBounds::new(0, 0, 10, 10));
        obj2.z_index = 0;
        tree.push(obj2);

        let commands = tree.collect_commands();
        assert!(!commands.is_empty());
    }
}
