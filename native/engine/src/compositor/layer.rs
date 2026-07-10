use crate::framebuffer::{Cell, FrameBuffer};
use crate::tree::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayerId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerType {
    Background,
    Content,
    Overlay,
    Popup,
    Tooltip,
    Cursor,
    Selection,
}

impl LayerType {
    pub fn z_index(&self) -> i32 {
        match self {
            Self::Background => 0,
            Self::Content => 10,
            Self::Selection => 20,
            Self::Overlay => 30,
            Self::Popup => 40,
            Self::Tooltip => 50,
            Self::Cursor => 60,
        }
    }

    pub fn is_transparent(&self) -> bool {
        matches!(self, Self::Cursor | Self::Selection)
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub id: LayerId,
    pub layer_type: LayerType,
    pub visible: bool,
    pub opacity: f32,
    pub offset_x: i16,
    pub offset_y: i16,
    buffer: FrameBuffer,
}

impl Layer {
    pub fn new(id: LayerId, layer_type: LayerType, width: u16, height: u16) -> Self {
        Self {
            id,
            layer_type,
            visible: true,
            opacity: 1.0,
            offset_x: 0,
            offset_y: 0,
            buffer: FrameBuffer::new(width, height),
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer.resize(width, height);
    }

    pub fn set_cell(&mut self, x: u16, y: u16, cell: Cell) {
        self.buffer.set(x, y, cell);
    }

    pub fn get_cell(&self, x: u16, y: u16) -> Option<Cell> {
        if self.buffer.in_bounds(x, y) {
            Some(*self.buffer.get(x, y))
        } else {
            None
        }
    }

    pub fn set_char(&mut self, x: u16, y: u16, ch: char) {
        let cell = Cell::new(ch);
        self.set_cell(x, y, cell);
    }

    pub fn set_char_with_color(&mut self, x: u16, y: u16, ch: char, fg: Color, bg: Color) {
        let cell = Cell::new(ch).with_fg(fg).with_bg(bg);
        self.set_cell(x, y, cell);
    }

    pub fn fill(&mut self, ch: char) {
        for y in 0..self.buffer.height() {
            for x in 0..self.buffer.width() {
                self.set_char(x, y, ch);
            }
        }
    }

    pub fn fill_rect(&mut self, x: u16, y: u16, width: u16, height: u16, ch: char) {
        for dy in 0..height {
            for dx in 0..width {
                self.set_char(x + dx, y + dy, ch);
            }
        }
    }

    pub fn clear(&mut self) {
        for y in 0..self.buffer.height() {
            for x in 0..self.buffer.width() {
                self.set_char(x, y, ' ');
            }
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    pub fn set_offset(&mut self, x: i16, y: i16) {
        self.offset_x = x;
        self.offset_y = y;
    }

    pub fn buffer(&self) -> &FrameBuffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut FrameBuffer {
        &mut self.buffer
    }

    pub fn dimensions(&self) -> (u16, u16) {
        (self.buffer.width(), self.buffer.height())
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.cells().iter().all(|c| c.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_id_new() {
        let id = LayerId(0);
        assert_eq!(id.0, 0);
    }

    #[test]
    fn layer_type_z_index() {
        assert_eq!(LayerType::Background.z_index(), 0);
        assert_eq!(LayerType::Content.z_index(), 10);
        assert_eq!(LayerType::Selection.z_index(), 20);
        assert_eq!(LayerType::Overlay.z_index(), 30);
        assert_eq!(LayerType::Popup.z_index(), 40);
        assert_eq!(LayerType::Tooltip.z_index(), 50);
        assert_eq!(LayerType::Cursor.z_index(), 60);
    }

    #[test]
    fn layer_type_is_transparent() {
        assert!(!LayerType::Background.is_transparent());
        assert!(!LayerType::Content.is_transparent());
        assert!(LayerType::Selection.is_transparent());
        assert!(!LayerType::Overlay.is_transparent());
        assert!(LayerType::Cursor.is_transparent());
    }

    #[test]
    fn layer_new() {
        let layer = Layer::new(LayerId(0), LayerType::Background, 80, 24);
        assert_eq!(layer.id, LayerId(0));
        assert_eq!(layer.layer_type, LayerType::Background);
        assert!(layer.visible);
        assert_eq!(layer.opacity, 1.0);
    }

    #[test]
    fn layer_resize() {
        let mut layer = Layer::new(LayerId(0), LayerType::Background, 80, 24);
        layer.resize(120, 40);
        assert_eq!(layer.dimensions(), (120, 40));
    }

    #[test]
    fn layer_set_char() {
        let mut layer = Layer::new(LayerId(0), LayerType::Background, 10, 10);
        layer.set_char(5, 5, 'X');
        let cell = layer.get_cell(5, 5);
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().ch, 'X');
    }

    #[test]
    fn layer_set_char_with_color() {
        let mut layer = Layer::new(LayerId(0), LayerType::Background, 10, 10);
        layer.set_char_with_color(
            5,
            5,
            'X',
            Color::Named(crate::tree::color::NamedColor::Red),
            Color::Default,
        );
        let cell = layer.get_cell(5, 5);
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().ch, 'X');
    }

    #[test]
    fn layer_fill() {
        let mut layer = Layer::new(LayerId(0), LayerType::Background, 10, 10);
        layer.fill('X');
        let cell = layer.get_cell(0, 0);
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().ch, 'X');
    }

    #[test]
    fn layer_fill_rect() {
        let mut layer = Layer::new(LayerId(0), LayerType::Background, 10, 10);
        layer.fill_rect(2, 2, 3, 3, 'X');
        let cell = layer.get_cell(2, 2);
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().ch, 'X');
    }

    #[test]
    fn layer_clear() {
        let mut layer = Layer::new(LayerId(0), LayerType::Background, 10, 10);
        layer.fill('X');
        layer.clear();
        let cell = layer.get_cell(0, 0);
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().ch, ' ');
    }

    #[test]
    fn layer_set_visible() {
        let mut layer = Layer::new(LayerId(0), LayerType::Background, 10, 10);
        layer.set_visible(false);
        assert!(!layer.visible);
    }

    #[test]
    fn layer_set_opacity() {
        let mut layer = Layer::new(LayerId(0), LayerType::Background, 10, 10);
        layer.set_opacity(0.5);
        assert_eq!(layer.opacity, 0.5);
    }

    #[test]
    fn layer_set_offset() {
        let mut layer = Layer::new(LayerId(0), LayerType::Background, 10, 10);
        layer.set_offset(5, 10);
        assert_eq!(layer.offset_x, 5);
        assert_eq!(layer.offset_y, 10);
    }

    #[test]
    fn layer_dimensions() {
        let layer = Layer::new(LayerId(0), LayerType::Background, 80, 24);
        assert_eq!(layer.dimensions(), (80, 24));
    }

    #[test]
    fn layer_is_empty() {
        let layer = Layer::new(LayerId(0), LayerType::Background, 10, 10);
        assert!(layer.is_empty());
    }
}
