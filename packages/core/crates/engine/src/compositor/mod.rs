//! Compositor: layer-based rendering with z-ordering (background, content, overlay, cursor).

mod layer;
mod pane;
mod renderer;

pub use layer::{Layer, LayerId, LayerType};
pub use pane::{FocusDirection, Pane, PaneError, PaneId, PaneManager, SplitDirection};
pub use renderer::CompositorRenderer;

use crate::framebuffer::FrameBuffer;

#[derive(Debug, Clone)]
pub struct Compositor {
    layers: Vec<Layer>,
    width: u16,
    height: u16,
}

impl Default for Compositor {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

impl Compositor {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            layers: Vec::new(),
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        for layer in &mut self.layers {
            layer.resize(width, height);
        }
    }

    pub fn add_layer(&mut self, layer_type: LayerType) -> LayerId {
        let id = LayerId(self.layers.len());
        let layer = Layer::new(id, layer_type, self.width, self.height);
        self.layers.push(layer);
        id
    }

    pub fn remove_layer(&mut self, id: LayerId) -> bool {
        if let Some(pos) = self.layers.iter().position(|l| l.id == id) {
            self.layers.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_layer(&self, id: LayerId) -> Option<&Layer> {
        self.layers.iter().find(|l| l.id == id)
    }

    pub fn get_layer_mut(&mut self, id: LayerId) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.id == id)
    }

    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }

    pub fn layers_mut(&mut self) -> &mut [Layer] {
        &mut self.layers
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn clear(&mut self) {
        self.layers.clear();
    }

    pub fn composite(&self, target: &mut FrameBuffer) {
        // Clear target
        for y in 0..self.height {
            for x in 0..self.width {
                target.set(x, y, crate::framebuffer::Cell::new(' '));
            }
        }

        // Composite layers in order
        for layer in &self.layers {
            if layer.visible {
                self.composite_layer(target, layer);
            }
        }
    }

    fn composite_layer(&self, target: &mut FrameBuffer, layer: &Layer) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = layer.get_cell(x, y)
                    && !cell.is_empty()
                {
                    target.set(x, y, cell);
                }
            }
        }
    }

    pub fn composite_to_buffer(&self) -> FrameBuffer {
        let mut buffer = FrameBuffer::new(self.width, self.height);
        self.composite(&mut buffer);
        buffer
    }

    pub fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compositor_new() {
        let compositor = Compositor::new(80, 24);
        assert_eq!(compositor.dimensions(), (80, 24));
        assert_eq!(compositor.layer_count(), 0);
    }

    #[test]
    fn compositor_default() {
        let compositor = Compositor::default();
        assert_eq!(compositor.dimensions(), (80, 24));
    }

    #[test]
    fn compositor_resize() {
        let mut compositor = Compositor::new(80, 24);
        compositor.resize(120, 40);
        assert_eq!(compositor.dimensions(), (120, 40));
    }

    #[test]
    fn compositor_add_layer() {
        let mut compositor = Compositor::new(80, 24);
        let id = compositor.add_layer(LayerType::Background);
        assert_eq!(compositor.layer_count(), 1);
        assert!(compositor.get_layer(id).is_some());
    }

    #[test]
    fn compositor_remove_layer() {
        let mut compositor = Compositor::new(80, 24);
        let id = compositor.add_layer(LayerType::Background);
        assert!(compositor.remove_layer(id));
        assert_eq!(compositor.layer_count(), 0);
    }

    #[test]
    fn compositor_get_layer() {
        let mut compositor = Compositor::new(80, 24);
        let id = compositor.add_layer(LayerType::Background);
        let layer = compositor.get_layer(id);
        assert!(layer.is_some());
        assert_eq!(layer.unwrap().layer_type, LayerType::Background);
    }

    #[test]
    fn compositor_get_layer_mut() {
        let mut compositor = Compositor::new(80, 24);
        let id = compositor.add_layer(LayerType::Background);
        let layer = compositor.get_layer_mut(id);
        assert!(layer.is_some());
    }

    #[test]
    fn compositor_clear() {
        let mut compositor = Compositor::new(80, 24);
        compositor.add_layer(LayerType::Background);
        compositor.add_layer(LayerType::Content);
        compositor.clear();
        assert_eq!(compositor.layer_count(), 0);
    }

    #[test]
    fn compositor_composite() {
        let mut compositor = Compositor::new(10, 10);
        compositor.add_layer(LayerType::Background);
        let buffer = compositor.composite_to_buffer();
        assert_eq!(buffer.width(), 10);
        assert_eq!(buffer.height(), 10);
    }

    #[test]
    fn compositor_dimensions() {
        let compositor = Compositor::new(80, 24);
        assert_eq!(compositor.dimensions(), (80, 24));
    }
}
