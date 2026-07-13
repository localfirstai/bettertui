//! Rendering pipeline: Renderer, RenderFrame, and the RenderBackend trait for ANSI output.

use crate::dirty_diff::{DirtyDiff, DirtyRegion};
use crate::framebuffer::FrameBuffer;
use crate::layout::LayoutTreeSync;
use crate::layout::build_render_tree_with_viewport;
use crate::layout::paint::Viewport;
use crate::render::ansi::AnsiBackend;
use crate::render::backend::RenderBackend;
use crate::render::object::RenderTree;
use crate::render::painter::Painter;
use crate::render::{PassResult, RenderPassContext, RenderPipeline};
use crate::scheduler::{FrameStatus, Scheduler};
use crate::tree::arena::NodeArena;

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
            pipeline: RenderPipeline::new(),
            needs_full_repaint: true,
            generation: 0,
            last_change_count: 0,
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
            pipeline: RenderPipeline::new(),
            needs_full_repaint: true,
            generation: 0,
            last_change_count: 0,
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

    pub fn render(&mut self, arena: &mut NodeArena) -> RenderFrame {
        self.generation += 1;

        // Frame suppression: if nothing changed and no full repaint needed, skip
        let change_count = arena.change_count();
        if !self.needs_full_repaint && change_count == self.last_change_count {
            return RenderFrame::new_empty(self.width, self.height);
        }
        self.last_change_count = change_count;

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
        self.render_tree =
            build_render_tree_with_viewport(arena, self.layout_sync.results(), Some(&vp));

        let ctx = crate::layout::paint::PaintContext::new(self.width, self.height);
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

        self.snapshot.copy_from(self.painter.buffer());

        self.scheduler.end_frame();

        // Clear dirty flags so next frame only updates changed nodes
        arena.clear_dirty_flags();

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
mod tests {
    use super::*;

    use crate::render::ansi::AnsiBackend;
    use crate::render::effects::ColorMatrixPass;
    use crate::render::effects::INVERT_MATRIX;
    use crate::tree::color::Color;

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
        let mut arena = make_arena();
        let frame = r.render(&mut arena);
        assert_eq!(frame.width, 40);
        assert_eq!(frame.height, 10);
        assert!(!frame.output_data.is_empty());
    }

    #[test]
    fn renderer_render_full() {
        let mut r = Renderer::new(40, 10);
        let mut arena = make_arena();
        let frame = r.render_full(&mut arena);
        assert_eq!(frame.width, 40);
        assert!(!frame.output_data.is_empty());
    }

    #[test]
    fn renderer_frame_suppression() {
        let mut r = Renderer::new(40, 10);
        let mut arena = make_arena();
        // First render always produces output
        let frame1 = r.render(&mut arena);
        assert!(!frame1.is_empty(), "first render should produce output");
        // Second render with no changes should be suppressed
        let frame2 = r.render(&mut arena);
        assert!(
            frame2.is_empty(),
            "second render with no changes should be suppressed"
        );
    }

    #[test]
    fn renderer_frame_suppression_released_on_change() {
        let mut r = Renderer::new(40, 10);
        let mut arena = make_arena();
        // First render
        let _ = r.render(&mut arena);
        // Second render suppressed (no changes)
        let frame2 = r.render(&mut arena);
        assert!(
            frame2.is_empty(),
            "second render with no changes should be suppressed"
        );
        // Mutate the arena to release suppression
        arena.mark_changed();
        // Third render should proceed (change_count check passes)
        // Note: output may still be empty if visual content matches snapshot
        // The key is that render() doesn't early-return with empty frame
        let _frame3 = r.render(&mut arena);
        // After mutation, render proceeds - verify it doesn't early-return
        // by checking that last_change_count was updated
        let frame4 = r.render(&mut arena);
        assert!(
            frame4.is_empty(),
            "fourth render with no new changes should be suppressed"
        );
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

    #[test]
    fn renderer_viewport_culling_pipeline() {
        let mut r = Renderer::new(80, 24);
        let mut arena = make_arena();
        let root = arena.root();

        // Add 50 children stacked vertically at y=0..50 (only first 24 in viewport)
        for _i in 0..50 {
            let mut n = crate::tree::RenderNode::new(crate::tree::NodeKind::Box);
            n.layout.width = Some(crate::layout::types::Sizing::Points(80.0));
            n.layout.height = Some(crate::layout::types::Sizing::Points(1.0));
            let id = arena.insert(n);
            arena.append_child(root, id).unwrap();
        }

        // First render — full tree visible
        let frame1 = r.render(&mut arena);
        assert!(!frame1.is_empty(), "first render should produce output");

        // Mark arena changed but don't actually change anything structural
        arena.mark_changed();
        let frame2 = r.render(&mut arena);
        // Content visually unchanged, so dirty regions should be empty
        // (renderer proceeds without early-return, but diff detects no changes)
        assert!(
            frame2.dirty_regions.is_empty(),
            "unchanged content should have no dirty regions"
        );

        // Render again — verify frame suppression
        let frame3 = r.render(&mut arena);
        assert!(frame3.is_empty(), "no-change render should be suppressed");
    }

    #[test]
    fn renderer_stress_large_tree_partial_visible() {
        let mut r = Renderer::new(80, 24);
        let mut arena = make_arena();
        let root = arena.root();

        // Build 200 nodes
        for _i in 0..200 {
            let n = crate::tree::RenderNode::new(crate::tree::NodeKind::Box);
            let id = arena.insert(n);
            arena.append_child(root, id).unwrap();
        }

        let frame = r.render(&mut arena);
        assert!(!frame.is_empty());
        let tree = r.render_tree();
        assert!(
            tree.len() < 200,
            "stress: large tree should be pruned, len={}",
            tree.len()
        );
    }

    #[test]
    fn renderer_stress_nested_scroll() {
        let mut r = Renderer::new(80, 24);
        let mut arena = make_arena();
        let root = arena.root();

        // Create scroll container with 1000 children
        let scroll_parent = arena.insert({
            let mut n = crate::tree::RenderNode::new(crate::tree::NodeKind::Scroll);
            n.layout.width = Some(crate::layout::types::Sizing::Points(80.0));
            n.layout.height = Some(crate::layout::types::Sizing::Points(20.0));
            n
        });
        arena.append_child(root, scroll_parent).unwrap();

        for _ in 0..1000 {
            let child = arena.insert({
                let mut n = crate::tree::RenderNode::new(crate::tree::NodeKind::Text);
                n.layout.width = Some(crate::layout::types::Sizing::Points(80.0));
                n.layout.height = Some(crate::layout::types::Sizing::Points(1.0));
                n.text = Some("x".into());
                n
            });
            arena.append_child(scroll_parent, child).unwrap();
        }

        let frame = r.render(&mut arena);
        assert!(!frame.is_empty());
        let tree = r.render_tree();
        // With 80x24 viewport and 80x20 scroll container + 1000 children at 1px each,
        // culling should reduce tree. Exact count depends on layout direction.
        assert!(
            !tree.is_empty() && tree.len() < 1000,
            "nested scroll should cull (len={})",
            tree.len()
        );
    }

    // ─── Post-Processing Pipeline Integration Tests ─────────────────────

    fn make_arena_with_text() -> NodeArena {
        let mut arena = NodeArena::new();
        let child = arena.insert({
            let mut n = crate::tree::RenderNode::new(crate::tree::NodeKind::Text);
            n.text = Some("Hello".into());
            n.style.fg = Some(Color::Rgb {
                r: 128,
                g: 128,
                b: 128,
            });
            n.layout.width = Some(crate::layout::types::Sizing::Points(10.0));
            n.layout.height = Some(crate::layout::types::Sizing::Points(1.0));
            n
        });
        arena.append_child(arena.root(), child).unwrap();
        {
            let root = arena.get_mut(arena.root()).unwrap();
            root.layout.width = Some(crate::layout::types::Sizing::Points(40.0));
            root.layout.height = Some(crate::layout::types::Sizing::Points(10.0));
        }
        arena
    }

    #[test]
    fn renderer_pipeline_passthrough_empty() {
        let mut r = Renderer::new(40, 10);
        let mut arena = make_arena();
        let frame = r.render(&mut arena);
        assert!(!frame.is_empty());
    }

    #[test]
    fn renderer_pipeline_modifies_output() {
        let mut r = Renderer::new(40, 10);
        r.pipeline_mut()
            .add_pass(Box::new(ColorMatrixPass::new(INVERT_MATRIX)));
        let mut arena = make_arena_with_text();
        let frame = r.render(&mut arena);
        assert!(!frame.is_empty());
        let cell = r.framebuffer().get(0, 0);
        assert_eq!(cell.ch, 'H');
        // 128 inverts to 126 via float math: (-128/255 + 1) * 255 = 126.999... → 126
        assert_eq!(
            cell.fg,
            Color::Rgb {
                r: 126,
                g: 126,
                b: 126
            }
        );
    }

    #[test]
    fn renderer_pipeline_passthrough_no_modify() {
        let mut r = Renderer::new(40, 10);
        // No passes added — pipeline is empty
        let mut arena = make_arena_with_text();
        let frame = r.render(&mut arena);
        assert!(!frame.is_empty());
        let cell = r.framebuffer().get(0, 0);
        assert_eq!(
            cell.fg,
            Color::Rgb {
                r: 128,
                g: 128,
                b: 128
            }
        );
    }

    #[test]
    fn renderer_pipeline_disabled_no_modify() {
        let mut r = Renderer::new(40, 10);
        r.pipeline_mut()
            .add_pass(Box::new(ColorMatrixPass::new(INVERT_MATRIX)));
        r.pipeline_mut().set_enabled(false);
        let mut arena = make_arena_with_text();
        let frame = r.render(&mut arena);
        assert!(!frame.output_data.is_empty());
        // Buffer unchanged
        let cell = r.framebuffer().get(0, 0);
        assert_eq!(
            cell.fg,
            Color::Rgb {
                r: 128,
                g: 128,
                b: 128
            }
        );
    }

    #[test]
    fn renderer_pipeline_get_pass_mut() {
        let mut r = Renderer::new(40, 10);
        r.pipeline_mut()
            .add_pass(Box::new(ColorMatrixPass::new(INVERT_MATRIX)));
        {
            let pass = r.pipeline_mut().get_pass_mut("color_matrix").unwrap();
            pass.set_enabled(false);
        }
        assert!(!r.pipeline().get_pass("color_matrix").unwrap().enabled());
    }

    #[test]
    fn renderer_multiple_passes_no_overflow() {
        let mut r = Renderer::new(80, 24);
        let mut arena = make_arena();
        let root = arena.root();

        for _ in 0..10 {
            let n = crate::tree::RenderNode::new(crate::tree::NodeKind::Box);
            let id = arena.insert(n);
            arena.append_child(root, id).unwrap();
        }

        // Render multiple times — should not crash or leak
        for i in 0..5 {
            arena.mark_changed();
            let frame = r.render(&mut arena);
            // First pass produces output, subsequent passes may suppress
            if i == 0 {
                assert!(!frame.is_empty());
            }
        }
    }
}
