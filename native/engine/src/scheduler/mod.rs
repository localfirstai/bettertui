use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameStatus {
    Idle,
    Pending,
    Due,
}

pub struct RenderScheduler {
    frame_interval: Duration,
    last_frame: Instant,
    pending: bool,
    frame_count: u64,
    dropped_frames: u64,
}

impl Default for RenderScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderScheduler {
    pub fn new() -> Self {
        Self {
            frame_interval: Duration::from_millis(16),
            last_frame: Instant::now(),
            pending: false,
            frame_count: 0,
            dropped_frames: 0,
        }
    }

    pub fn with_fps(fps: u32) -> Self {
        let interval = if fps > 0 {
            Duration::from_millis(1000 / fps as u64)
        } else {
            Duration::from_millis(16)
        };
        Self {
            frame_interval: interval,
            last_frame: Instant::now(),
            pending: false,
            frame_count: 0,
            dropped_frames: 0,
        }
    }

    pub fn request_frame(&mut self) {
        self.pending = true;
    }

    pub fn status(&self) -> FrameStatus {
        if !self.pending {
            return FrameStatus::Idle;
        }
        if self.last_frame.elapsed() >= self.frame_interval {
            FrameStatus::Due
        } else {
            FrameStatus::Pending
        }
    }

    pub fn begin_frame(&mut self) -> bool {
        if self.status() != FrameStatus::Due {
            return false;
        }
        self.pending = false;
        self.last_frame = Instant::now();
        self.frame_count += 1;
        true
    }

    pub fn skip_frame(&mut self) {
        if self.pending {
            self.dropped_frames += 1;
            self.pending = false;
        }
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn dropped_frames(&self) -> u64 {
        self.dropped_frames
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.frame_interval = if fps > 0 {
            Duration::from_millis(1000 / fps as u64)
        } else {
            Duration::from_millis(16)
        };
    }

    pub fn time_until_next_frame(&self) -> Duration {
        let elapsed = self.last_frame.elapsed();
        if elapsed >= self.frame_interval {
            Duration::ZERO
        } else {
            self.frame_interval - elapsed
        }
    }

    pub fn reset(&mut self) {
        self.last_frame = Instant::now();
        self.pending = false;
        self.frame_count = 0;
        self.dropped_frames = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_new() {
        let s = RenderScheduler::new();
        assert_eq!(s.status(), FrameStatus::Idle);
        assert_eq!(s.frame_count(), 0);
    }

    #[test]
    fn scheduler_request_frame() {
        let mut s = RenderScheduler::new();
        s.request_frame();
        assert_ne!(s.status(), FrameStatus::Idle);
    }

    #[test]
    fn scheduler_begin_frame() {
        let mut s = RenderScheduler::new();
        s.request_frame();
        std::thread::sleep(Duration::from_millis(20));
        assert!(s.begin_frame());
        assert_eq!(s.frame_count(), 1);
        assert_eq!(s.status(), FrameStatus::Idle);
    }

    #[test]
    fn scheduler_skip_frame() {
        let mut s = RenderScheduler::new();
        s.request_frame();
        s.skip_frame();
        assert_eq!(s.status(), FrameStatus::Idle);
        assert_eq!(s.dropped_frames(), 1);
    }

    #[test]
    fn scheduler_with_fps() {
        let s = RenderScheduler::with_fps(30);
        assert_eq!(s.frame_interval, Duration::from_millis(33));
    }

    #[test]
    fn scheduler_set_fps() {
        let mut s = RenderScheduler::new();
        s.set_fps(60);
        assert_eq!(s.frame_interval, Duration::from_millis(16));
    }

    #[test]
    fn scheduler_time_until_next_frame() {
        let s = RenderScheduler::new();
        let t = s.time_until_next_frame();
        assert!(t <= Duration::from_millis(16));
    }

    #[test]
    fn scheduler_reset() {
        let mut s = RenderScheduler::new();
        s.request_frame();
        s.reset();
        assert_eq!(s.status(), FrameStatus::Idle);
        assert_eq!(s.frame_count(), 0);
    }

    #[test]
    fn scheduler_multiple_frames() {
        let mut s = RenderScheduler::new();
        for _ in 0..5 {
            s.request_frame();
            std::thread::sleep(Duration::from_millis(20));
            s.begin_frame();
        }
        assert_eq!(s.frame_count(), 5);
    }
}
