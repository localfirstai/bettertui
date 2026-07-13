use std::cell::RefCell;
use std::collections::HashMap;

use crate::layout::{ClipBounds, PaintBounds, PaintFlags};
use crate::tree::Overflow;
use crate::tree::ResolvedStyle;
use crate::tree::{NodeId, Rect};

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
    pub text_align: crate::text::TextAlign,
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
            text_align: crate::text::TextAlign::Left,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::NodeId;
    use crate::tree::{Color, NamedColor};

    fn make_id() -> NodeId {
        let mut arena = crate::tree::NodeArena::new();
        arena.insert(crate::tree::RenderNode::new(crate::tree::NodeKind::Box))
    }

    #[test]
    fn render_object_new() {
        let id = make_id();
        let obj = RenderObject::new(id);
        assert_eq!(obj.id, id);
        assert_eq!(obj.opacity, 1.0);
        assert_eq!(obj.z_index, 0);
        assert!(obj.text.is_none());
        assert!(obj.clip.is_none());
    }

    #[test]
    fn render_object_has_background() {
        let mut obj = RenderObject::new(make_id());
        assert!(!obj.has_background());
        obj.style.bg = Some(Color::Named(NamedColor::Blue));
        assert!(obj.has_background());
    }

    #[test]
    fn render_object_has_text() {
        let mut obj = RenderObject::new(make_id());
        assert!(!obj.has_text());
        obj.text = Some("hello".into());
        assert!(obj.has_text());
    }

    #[test]
    fn render_object_is_visible() {
        let mut obj = RenderObject::new(make_id());
        assert!(obj.is_visible());
        obj.opacity = 0.0;
        assert!(!obj.is_visible());
    }

    #[test]
    fn render_object_content_rect() {
        let mut obj = RenderObject::new(make_id());
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
}

#[cfg(test)]
mod tree_tests {
    use super::*;
    use crate::tree::NodeArena;
    use crate::tree::NodeKind;
    use crate::tree::RenderNode;

    fn make_ids(count: usize) -> Vec<NodeId> {
        let mut arena = NodeArena::new();
        let mut ids = Vec::new();
        for _ in 0..count {
            ids.push(arena.insert(RenderNode::new(NodeKind::Box)));
        }
        ids
    }

    #[test]
    fn render_tree_new() {
        let tree = RenderTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn render_tree_push_and_get() {
        let ids = make_ids(2);
        let mut tree = RenderTree::new();
        let mut obj = RenderObject::new(ids[0]);
        obj.z_index = 1;
        tree.push(obj);
        tree.push(RenderObject::new(ids[1]));
        assert_eq!(tree.len(), 2);
        assert!(tree.get(ids[0]).is_some());
        assert_eq!(tree.get(ids[0]).unwrap().z_index, 1);
    }

    #[test]
    fn render_tree_root() {
        let ids = make_ids(1);
        let mut tree = RenderTree::new();
        assert!(tree.root().is_none());
        tree.push(RenderObject::new(ids[0]));
        assert_eq!(tree.root(), Some(ids[0]));
    }

    #[test]
    fn render_tree_sorted_by_z_index() {
        let ids = make_ids(3);
        let mut tree = RenderTree::new();
        let mut obj0 = RenderObject::new(ids[0]);
        obj0.z_index = 10;
        let mut obj1 = RenderObject::new(ids[1]);
        obj1.z_index = 0;
        let mut obj2 = RenderObject::new(ids[2]);
        obj2.z_index = 5;
        tree.push(obj0);
        tree.push(obj1);
        tree.push(obj2);
        let sorted = tree.sorted_by_z_index();
        assert_eq!(sorted[0], 1);
        assert_eq!(sorted[1], 2);
        assert_eq!(sorted[2], 0);
        // Second call should return cached result
        let sorted2 = tree.sorted_by_z_index();
        assert_eq!(sorted2, sorted);
    }

    #[test]
    fn render_tree_clear() {
        let ids = make_ids(2);
        let mut tree = RenderTree::new();
        tree.push(RenderObject::new(ids[0]));
        tree.push(RenderObject::new(ids[1]));
        tree.clear();
        assert!(tree.is_empty());
        assert!(tree.root().is_none());
    }

    #[test]
    fn render_tree_iter_mut() {
        let ids = make_ids(2);
        let mut tree = RenderTree::new();
        tree.push(RenderObject::new(ids[0]));
        tree.push(RenderObject::new(ids[1]));
        for obj in tree.iter_mut() {
            obj.opacity = 0.5;
        }
        for obj in tree.iter() {
            assert_eq!(obj.opacity, 0.5);
        }
    }
}
