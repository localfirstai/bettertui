//! Rendering pipeline: Renderer, RenderFrame, and the RenderBackend trait for ANSI output.

pub mod backend;

use crate::dirty_diff::{DirtyDiff, DirtyRegion};
use crate::framebuffer::FrameBuffer;
use crate::layout::LayoutTreeSync;
use crate::painter::Painter;
use crate::render_object::{RenderTree, build_render_tree};
use crate::renderer::backend::RenderBackend;
use crate::renderer::backend::ansi::AnsiBackend;
use crate::scheduler::{FrameStatus, Scheduler};
use crate::tree::arena::NodeArena;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderFrame {
    pub output_data: Vec<u8>,
    pub dirty_regions: Vec<DirtyRegion>,
    pub width: u16,
    pub height: u16,
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
    needs_full_repaint: bool,
    generation: u64,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
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
            needs_full_repaint: true,
            generation: 0,
        }
    }

    pub fn with_backend(width: u16, height: u16, backend: Box<dyn RenderBackend>) -> Self {
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
            needs_full_repaint: true,
            generation: 0,
        }
    }

    pub fn with_fps(fps: u32) -> Self {
        Self {
            scheduler: Scheduler::with_fps(fps),
            ..Self::new(80, 24)
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.painter.resize(width, height);
        self.snapshot.resize(width, height);
        self.needs_full_repaint = true;
    }

    pub fn request_frame(&mut self) {
        self.scheduler.request_frame();
    }

    pub fn should_render(&self) -> FrameStatus {
        self.scheduler.status()
    }

    pub fn render(&mut self, arena: &NodeArena) -> RenderFrame {
        self.layout_sync.sync_full(arena);

        let root_id = arena.root();
        for (id, _node) in arena.iter() {
            let children = arena.children(id);
            if !children.is_empty() {
                self.layout_sync.sync_children(arena, id);
            }
        }
        let _ = self.layout_sync.compute(root_id, self.width, self.height);

        self.render_tree = build_render_tree(arena, self.layout_sync.results());

        let ctx = crate::render_object::PaintContext::new(self.width, self.height);
        self.painter.paint(&self.render_tree, &ctx);

        let _ = self.scheduler.begin_frame();
        self.generation += 1;
        let dirty_regions = if self.needs_full_repaint {
            let _ = self
                .dirty_diff
                .compute_full_repaint(self.width, self.height);
            self.needs_full_repaint = false;
            self.dirty_diff.regions().to_vec()
        } else {
            self.dirty_diff
                .compute(self.painter.buffer(), &self.snapshot, self.generation);
            self.dirty_diff.regions().to_vec()
        };

        self.backend.encode(self.painter.buffer(), &dirty_regions);

        self.snapshot.copy_from(self.painter.buffer());

        RenderFrame {
            output_data: self.backend.finish().to_vec(),
            dirty_regions,
            width: self.width,
            height: self.height,
        }
    }

    pub fn render_full(&mut self, arena: &NodeArena) -> RenderFrame {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::backend::ansi::AnsiBackend;

    fn make_arena() -> NodeArena {
        let mut arena = NodeArena::new();
        arena.insert(crate::tree::RenderNode::box_node());
        arena
    }

    #[test]
    fn renderer_new() {
        let r = Renderer::new(80, 24);
        assert_eq!(r.dimensions(), (80, 24));
    }

    #[test]
    fn renderer_default() {
        let r = Renderer::default();
        assert_eq!(r.dimensions(), (80, 24));
    }

    #[test]
    fn renderer_with_fps() {
        let r = Renderer::with_fps(60);
        assert_eq!(r.dimensions(), (80, 24));
    }

    #[test]
    fn renderer_resize() {
        let mut r = Renderer::new(80, 24);
        r.resize(120, 40);
        assert_eq!(r.dimensions(), (120, 40));
        assert!(r.needs_full_repaint);
    }

    #[test]
    fn renderer_render_empty_tree() {
        let mut r = Renderer::new(40, 10);
        let arena = make_arena();
        let frame = r.render(&arena);
        assert_eq!(frame.width, 40);
        assert_eq!(frame.height, 10);
        assert!(!frame.output_data.is_empty());
    }

    #[test]
    fn renderer_render_full() {
        let mut r = Renderer::new(40, 10);
        let arena = make_arena();
        let frame = r.render_full(&arena);
        assert_eq!(frame.width, 40);
        assert!(!frame.output_data.is_empty());
    }

    #[test]
    fn renderer_render_tree_empty() {
        let r = Renderer::new(40, 10);
        assert!(r.render_tree().is_empty());
    }

    #[test]
    fn renderer_framebuffer_dimensions() {
        let r = Renderer::new(120, 50);
        assert_eq!(r.framebuffer().width(), 120);
        assert_eq!(r.framebuffer().height(), 50);
    }

    #[test]
    fn renderer_with_backend() {
        let backend = Box::new(AnsiBackend::new());
        let r = Renderer::with_backend(80, 24, backend);
        assert_eq!(r.dimensions(), (80, 24));
    }

    #[test]
    fn renderer_set_backend() {
        let mut r = Renderer::new(80, 24);
        let backend = Box::new(AnsiBackend::new());
        r.set_backend(backend);
        assert_eq!(r.dimensions(), (80, 24));
    }
}
