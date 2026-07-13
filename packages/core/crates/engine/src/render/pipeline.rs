use crate::framebuffer::FrameBuffer;
use crate::render::PassResult;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::Cell;
    use crate::tree::color::Color;

    struct TestPass {
        name: &'static str,
        enabled: bool,
        priority: PassPriority,
        modify: bool,
    }

    impl RenderPass for TestPass {
        fn name(&self) -> &str {
            self.name
        }

        fn execute(&mut self, buffer: &mut FrameBuffer, _ctx: &RenderPassContext) -> PassResult {
            if self.modify {
                let cell = Cell::new('T').with_fg(Color::Rgb { r: 255, g: 0, b: 0 });
                buffer.set(0, 0, cell);
                PassResult::Modified
            } else {
                PassResult::Unchanged
            }
        }

        fn enabled(&self) -> bool {
            self.enabled
        }

        fn set_enabled(&mut self, enabled: bool) {
            self.enabled = enabled;
        }

        fn priority(&self) -> PassPriority {
            self.priority
        }
    }

    #[test]
    fn pipeline_new() {
        let p = RenderPipeline::new();
        assert!(p.is_empty());
        assert!(p.enabled());
    }

    #[test]
    fn pipeline_add_pass() {
        let mut p = RenderPipeline::new();
        p.add_pass(Box::new(TestPass {
            name: "test",
            enabled: true,
            priority: PassPriority::Normal,
            modify: false,
        }));
        assert_eq!(p.len(), 1);
    }

    #[test]
    fn pipeline_get_pass() {
        let mut p = RenderPipeline::new();
        p.add_pass(Box::new(TestPass {
            name: "test",
            enabled: true,
            priority: PassPriority::Normal,
            modify: false,
        }));
        assert!(p.get_pass("test").is_some());
        assert!(p.get_pass("nonexistent").is_none());
    }

    #[test]
    fn pipeline_execute_unmodified() {
        let mut p = RenderPipeline::new();
        p.add_pass(Box::new(TestPass {
            name: "test",
            enabled: true,
            priority: PassPriority::Normal,
            modify: false,
        }));
        let mut fb = FrameBuffer::new(10, 10);
        let ctx = RenderPassContext::new(10, 10);
        assert_eq!(p.execute(&mut fb, &ctx), PassResult::Unchanged);
    }

    #[test]
    fn pipeline_execute_modified() {
        let mut p = RenderPipeline::new();
        p.add_pass(Box::new(TestPass {
            name: "test",
            enabled: true,
            priority: PassPriority::Normal,
            modify: true,
        }));
        let mut fb = FrameBuffer::new(10, 10);
        let ctx = RenderPassContext::new(10, 10);
        assert_eq!(p.execute(&mut fb, &ctx), PassResult::Modified);
    }

    #[test]
    fn pipeline_disabled() {
        let mut p = RenderPipeline::new();
        p.add_pass(Box::new(TestPass {
            name: "test",
            enabled: true,
            priority: PassPriority::Normal,
            modify: true,
        }));
        p.set_enabled(false);
        let mut fb = FrameBuffer::new(10, 10);
        let ctx = RenderPassContext::new(10, 10);
        assert_eq!(p.execute(&mut fb, &ctx), PassResult::Unchanged);
    }

    #[test]
    fn pipeline_pass_disabled() {
        let mut p = RenderPipeline::new();
        p.add_pass(Box::new(TestPass {
            name: "test",
            enabled: false,
            priority: PassPriority::Normal,
            modify: true,
        }));
        let mut fb = FrameBuffer::new(10, 10);
        let ctx = RenderPassContext::new(10, 10);
        assert_eq!(p.execute(&mut fb, &ctx), PassResult::Unchanged);
    }

    #[test]
    fn pipeline_priority_ordering() {
        let mut p = RenderPipeline::new();
        let last = TestPass {
            name: "last",
            enabled: true,
            priority: PassPriority::Last,
            modify: false,
        };
        let first = TestPass {
            name: "first",
            enabled: true,
            priority: PassPriority::First,
            modify: false,
        };
        p.add_pass(Box::new(last));
        // Should still be ordered by priority after resort
        assert_eq!(p.passes[0].priority(), PassPriority::Last);
        p.add_pass(Box::new(first));
        // After add, resort runs — first should now be at index 0
        assert_eq!(p.passes[0].priority(), PassPriority::First);
        assert_eq!(p.passes[1].priority(), PassPriority::Last);
    }

    #[test]
    fn pipeline_remove_pass() {
        let mut p = RenderPipeline::new();
        p.add_pass(Box::new(TestPass {
            name: "test",
            enabled: true,
            priority: PassPriority::Normal,
            modify: false,
        }));
        assert_eq!(p.len(), 1);
        p.remove_pass("test");
        assert!(p.is_empty());
    }

    #[test]
    fn pipeline_priority_get_pass_mut() {
        let mut p = RenderPipeline::new();
        p.add_pass(Box::new(TestPass {
            name: "test",
            enabled: true,
            priority: PassPriority::Normal,
            modify: false,
        }));
        {
            let pass = p.get_pass_mut("test").unwrap();
            pass.set_enabled(false);
        }
        assert!(!p.get_pass("test").unwrap().enabled());
    }
}
