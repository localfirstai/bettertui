use super::Compositor;
use crate::dirty_diff::DirtyRegion;
use crate::framebuffer::FrameBuffer;
use crate::renderer::backend::RenderBackend;

pub struct CompositorRenderer {
    compositor: Compositor,
    backend: Box<dyn RenderBackend>,
    previous_buffer: Option<FrameBuffer>,
}

impl Default for CompositorRenderer {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

impl CompositorRenderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            compositor: Compositor::new(width, height),
            backend: Box::new(crate::renderer::backend::ansi::AnsiBackend::new()),
            previous_buffer: Some(FrameBuffer::new(width, height)),
        }
    }

    pub fn with_backend(width: u16, height: u16, backend: Box<dyn RenderBackend>) -> Self {
        Self {
            compositor: Compositor::new(width, height),
            backend,
            previous_buffer: Some(FrameBuffer::new(width, height)),
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.compositor.resize(width, height);
        if let Some(prev) = &mut self.previous_buffer {
            prev.resize(width, height);
        }
    }

    pub fn compositor(&self) -> &Compositor {
        &self.compositor
    }

    pub fn compositor_mut(&mut self) -> &mut Compositor {
        &mut self.compositor
    }

    pub fn render(&mut self) -> Vec<u8> {
        let current_buffer = self.compositor.composite_to_buffer();

        let dirty_regions = if let Some(prev) = &self.previous_buffer {
            self.compute_dirty_regions(&current_buffer, prev)
        } else {
            vec![DirtyRegion::new(
                0,
                0,
                current_buffer.width(),
                current_buffer.height(),
            )]
        };

        self.backend.encode(&current_buffer, &dirty_regions);
        let output = self.backend.finish().to_vec();

        if let Some(prev) = &mut self.previous_buffer {
            prev.copy_from(&current_buffer);
        }

        output
    }

    fn compute_dirty_regions(
        &self,
        current: &FrameBuffer,
        previous: &FrameBuffer,
    ) -> Vec<DirtyRegion> {
        let mut regions = Vec::new();
        let width = current.width();
        let height = current.height();

        // Simple implementation: check each cell
        // A more sophisticated implementation would use a dirty rect algorithm
        let mut dirty = false;
        let mut min_x = width;
        let mut min_y = height;
        let mut max_x = 0;
        let mut max_y = 0;

        for y in 0..height {
            for x in 0..width {
                let current_cell = current.get(x, y);
                let previous_cell = previous.get(x, y);
                if current_cell != previous_cell {
                    dirty = true;
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
            }
        }

        if dirty {
            regions.push(DirtyRegion::new(
                min_x,
                min_y,
                max_x - min_x + 1,
                max_y - min_y + 1,
            ));
        }

        regions
    }

    pub fn dimensions(&self) -> (u16, u16) {
        self.compositor.dimensions()
    }

    pub fn backend(&self) -> &dyn RenderBackend {
        self.backend.as_ref()
    }

    pub fn set_backend(&mut self, backend: Box<dyn RenderBackend>) {
        self.backend = backend;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compositor::LayerType;

    #[test]
    fn compositor_renderer_new() {
        let renderer = CompositorRenderer::new(80, 24);
        assert_eq!(renderer.dimensions(), (80, 24));
    }

    #[test]
    fn compositor_renderer_default() {
        let renderer = CompositorRenderer::default();
        assert_eq!(renderer.dimensions(), (80, 24));
    }

    #[test]
    fn compositor_renderer_resize() {
        let mut renderer = CompositorRenderer::new(80, 24);
        renderer.resize(120, 40);
        assert_eq!(renderer.dimensions(), (120, 40));
    }

    #[test]
    fn compositor_renderer_render() {
        let mut renderer = CompositorRenderer::new(10, 10);
        let layer_id = renderer.compositor_mut().add_layer(LayerType::Background);
        if let Some(layer) = renderer.compositor_mut().get_layer_mut(layer_id) {
            layer.set_char(0, 0, 'X');
        }
        let output = renderer.render();
        assert!(
            !output.is_empty(),
            "layer with content should produce output"
        );
    }

    #[test]
    fn compositor_renderer_dimensions() {
        let renderer = CompositorRenderer::new(80, 24);
        assert_eq!(renderer.dimensions(), (80, 24));
    }
}
