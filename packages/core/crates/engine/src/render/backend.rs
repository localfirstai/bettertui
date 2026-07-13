use crate::dirty_diff::DirtyRegion;
use crate::framebuffer::FrameBuffer;

pub trait RenderBackend {
    fn encode(&mut self, buffer: &FrameBuffer, regions: &[DirtyRegion]);
    fn finish(&self) -> &[u8];
    fn reset(&mut self);
}
