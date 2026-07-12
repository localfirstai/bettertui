use crate::tree::style::ResolvedStyle;
use crate::tree::visual::Overflow;
use crate::tree::{NodeId, Rect};

use super::paint::{ClipBounds, PaintBounds, PaintFlags};

#[derive(Debug, Clone)]
pub struct RenderObject {
    pub id: NodeId,
    pub bounds: PaintBounds,
    pub clip: Option<ClipBounds>,
    pub style: ResolvedStyle,
    pub opacity: f32,
    pub z_index: i32,
    pub text: Option<Box<str>>,
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
            text: None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::NodeId;
    use crate::tree::color::{Color, NamedColor};

    fn make_id(_n: u32) -> NodeId {
        let mut arena = crate::tree::arena::NodeArena::new();
        arena.insert(crate::tree::render_node::RenderNode::new(
            crate::tree::node_kind::NodeKind::Box,
        ))
    }

    #[test]
    fn render_object_new() {
        let id = make_id(0);
        let obj = RenderObject::new(id);
        assert_eq!(obj.id, id);
        assert_eq!(obj.opacity, 1.0);
        assert_eq!(obj.z_index, 0);
        assert!(obj.text.is_none());
        assert!(obj.clip.is_none());
    }

    #[test]
    fn render_object_has_background() {
        let mut obj = RenderObject::new(make_id(0));
        assert!(!obj.has_background());
        obj.style.bg = Some(Color::Named(NamedColor::Blue));
        assert!(obj.has_background());
    }

    #[test]
    fn render_object_has_text() {
        let mut obj = RenderObject::new(make_id(0));
        assert!(!obj.has_text());
        obj.text = Some("hello".into());
        assert!(obj.has_text());
    }

    #[test]
    fn render_object_is_visible() {
        let mut obj = RenderObject::new(make_id(0));
        assert!(obj.is_visible());
        obj.opacity = 0.0;
        assert!(!obj.is_visible());
    }

    #[test]
    fn render_object_content_rect() {
        let mut obj = RenderObject::new(make_id(0));
        obj.bounds.width = 20;
        obj.bounds.height = 10;
        obj.bounds.padding_left = 2;
        obj.bounds.padding_right = 2;
        obj.bounds.padding_top = 1;
        obj.bounds.padding_bottom = 1;
        let cr = obj.content_rect();
        assert_eq!(cr.x, 2);
        assert_eq!(cr.y, 1);
        assert_eq!(cr.width, 16);
        assert_eq!(cr.height, 8);
    }
}
