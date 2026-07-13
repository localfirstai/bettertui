use crate::framebuffer::{Cell, CellAttributes, FrameBuffer};
use crate::layout::paint::{PaintBounds, PaintContext, PaintFlags};
use crate::render_object::{RenderObject, RenderTree};
use crate::text::{LayoutConfig, layout_text};
use crate::tree::color::Color;
use crate::tree::style::ResolvedStyle;

pub struct Painter {
    buffer: FrameBuffer,
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

    pub fn paint(&mut self, tree: &RenderTree, ctx: &PaintContext) {
        self.paint_with_clear(tree, ctx, true);
    }

    /// Paint with optional clear. When `full_clear` is false and no structural
    /// changes occurred, old content outside dirty regions persists in the buffer.
    /// The downstream DirtyDiff still correctly detects changed regions.
    ///
    /// Frame suppression (RenderFrame::is_empty) handles the no-change case
    /// instead of relying solely on incremental paint.
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

        let config = LayoutConfig {
            wrap: obj.text_wrap,
            align: obj.text_align,
            max_width: content.width,
            max_height: content.height,
            pad_left: content.x,
            pad_top: content.y,
            ..LayoutConfig::default()
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
                // Check content bounds BEFORE writing to avoid rendering when width=0
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LayoutEngine;
    use crate::layout::build_render_tree;
    use crate::layout::types::Sizing;
    use crate::tree::arena::NodeArena;
    use crate::tree::color::NamedColor;
    use crate::tree::node_kind::NodeKind;
    use crate::tree::render_node::RenderNode;
    use crate::tree::visual::Display;

    fn make_painter_with_tree() -> (Painter, RenderTree) {
        let mut arena = NodeArena::new();
        let child = arena.insert({
            let mut n = RenderNode::new(NodeKind::Text);
            n.text = Some("Hi".into());
            n.style.fg = Some(Color::Named(NamedColor::Red));
            n.style.bold = Some(true);
            n.layout.width = Some(Sizing::Points(4.0));
            n.layout.height = Some(Sizing::Points(1.0));
            n
        });
        arena.append_child(arena.root(), child).unwrap();

        {
            let root = arena.get_mut(arena.root()).unwrap();
            root.layout.width = Some(Sizing::Points(80.0));
            root.layout.height = Some(Sizing::Points(24.0));
        }

        let mut engine = LayoutEngine::new();
        let root = arena.root();
        let rn = arena.get(root).unwrap();
        engine.register_container(root, &rn.layout);
        let cn = arena.get(child).unwrap();
        engine.register_container(child, &cn.layout);
        engine.add_child(root, child);
        engine.compute_layout(root, 80.0, 24.0).unwrap();
        let results = engine.collect_results();

        let tree = build_render_tree(&arena, &results);
        let ctx = PaintContext::new(80, 24);
        let mut painter = Painter::new(80, 24);
        painter.paint(&tree, &ctx);
        (painter, tree)
    }

    #[test]
    fn painter_new() {
        let painter = Painter::new(80, 24);
        assert_eq!(painter.buffer.width(), 80);
        assert_eq!(painter.buffer.height(), 24);
    }

    #[test]
    fn painter_paint_background() {
        let mut arena = NodeArena::new();
        let child = arena.insert({
            let mut n = RenderNode::new(NodeKind::Box);
            n.style.bg = Some(Color::Named(NamedColor::Blue));
            n.layout.width = Some(Sizing::Points(5.0));
            n.layout.height = Some(Sizing::Points(1.0));
            n
        });
        arena.append_child(arena.root(), child).unwrap();

        {
            let root = arena.get_mut(arena.root()).unwrap();
            root.layout.width = Some(Sizing::Points(80.0));
            root.layout.height = Some(Sizing::Points(24.0));
        }

        let mut engine = LayoutEngine::new();
        let root = arena.root();
        let rn = arena.get(root).unwrap();
        engine.register_container(root, &rn.layout);
        let cn = arena.get(child).unwrap();
        engine.register_container(child, &cn.layout);
        engine.add_child(root, child);
        engine.compute_layout(root, 80.0, 24.0).unwrap();
        let results = engine.collect_results();

        let tree = build_render_tree(&arena, &results);
        let ctx = PaintContext::new(80, 24);
        let mut painter = Painter::new(80, 24);
        painter.paint(&tree, &ctx);

        let cell = painter.buffer().get(0, 0);
        assert_eq!(cell.bg, Color::Named(NamedColor::Blue));
    }

    #[test]
    fn painter_paint_text() {
        let (painter, _) = make_painter_with_tree();
        let cell = painter.buffer().get(0, 0);
        assert_eq!(cell.ch, 'H');
        assert_eq!(cell.fg, Color::Named(NamedColor::Red));
        assert!(cell.attributes.contains(CellAttributes::BOLD));
    }

    #[test]
    fn painter_excludes_hidden() {
        let mut arena = NodeArena::new();
        let child = arena.insert({
            let mut n = RenderNode::new(NodeKind::Text);
            n.text = Some("Hidden".into());
            n.visibility.display = Display::None;
            n
        });
        arena.append_child(arena.root(), child).unwrap();

        let mut engine = LayoutEngine::new();
        let root = arena.root();
        let rn = arena.get(root).unwrap();
        engine.register_container(root, &rn.layout);
        let cn = arena.get(child).unwrap();
        engine.register_container(child, &cn.layout);
        engine.add_child(root, child);
        engine.compute_layout(root, 80.0, 24.0).unwrap();
        let results = engine.collect_results();

        let tree = build_render_tree(&arena, &results);
        let ctx = PaintContext::new(80, 24);
        let mut painter = Painter::new(80, 24);
        painter.paint(&tree, &ctx);

        let cell = painter.buffer().get(0, 0);
        assert!(cell.is_empty());
    }

    #[test]
    fn painter_diff() {
        let (mut painter, tree) = make_painter_with_tree();
        let ctx = PaintContext::new(80, 24);
        painter.paint(&tree, &ctx);
        painter.swap();
        painter.paint(&tree, &ctx);
        let dirty = painter.diff();
        assert!(dirty.is_empty());
    }
}
