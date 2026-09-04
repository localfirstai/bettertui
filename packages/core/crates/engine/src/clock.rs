use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

pub trait Clock: Send {
    fn now_ms(&self) -> u64;
    fn now(&self) -> Duration;
    fn is_monotonic(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct SystemClock {
    epoch: Instant,
}

impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemClock {
    pub fn new() -> Self {
        Self { epoch: Instant::now() }
    }
}

impl Clock for SystemClock {
    fn now_ms(&self) -> u64 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
    }
    fn now(&self) -> Duration {
        self.epoch.elapsed()
    }
}

#[derive(Debug)]
struct ManualClockInner {
    time: Duration,
    timers: Vec<ScheduledTimer>,
    next_id: u64,
}

#[derive(Debug)]
pub struct ManualClock {
    inner: Arc<std::sync::Mutex<ManualClockInner>>,
}

#[derive(Debug)]
struct ScheduledTimer {
    id: u64,
    fire_at: Duration,
    delay: Duration,
    repeat: bool,
    callback_fn: TimerFn,
}

type TimerFn = fn();

impl ManualClock {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(std::sync::Mutex::new(ManualClockInner {
                time: Duration::ZERO,
                timers: Vec::new(),
                next_id: 1,
            })),
        }
    }

    pub fn advance(&mut self, ms: u64) {
        let mut inner = self.inner.lock().unwrap();
        let target = inner.time + Duration::from_millis(ms);
        Self::advance_to_inner(&mut inner, target);
    }

    pub fn advance_duration(&mut self, duration: Duration) {
        let mut inner = self.inner.lock().unwrap();
        let target = inner.time + duration;
        Self::advance_to_inner(&mut inner, target);
    }

    pub fn set_time(&mut self, time: Duration) {
        let mut inner = self.inner.lock().unwrap();
        if time > inner.time {
            Self::advance_to_inner(&mut inner, time);
        } else {
            inner.time = time;
        }
    }

    pub fn run_all(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        loop {
            let next = Self::next_fire_time_inner(&inner);
            match next {
                Some(t) if t <= inner.time => Self::fire_due_timers_inner(&mut inner),
                Some(t) => Self::advance_to_inner(&mut inner, t),
                None => break,
            }
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.inner.lock().unwrap().time
    }
    pub fn elapsed_ms(&self) -> u64 {
        self.inner.lock().unwrap().time.as_millis() as u64
    }

    pub fn set_timeout(&mut self, callback: TimerFn, delay_ms: u64) -> u64 {
        let mut inner = self.inner.lock().unwrap();
        let id = inner.next_id;
        inner.next_id += 1;
        let now = inner.time;
        inner.timers.push(ScheduledTimer {
            id,
            fire_at: now + Duration::from_millis(delay_ms),
            delay: Duration::from_millis(delay_ms),
            repeat: false,
            callback_fn: callback,
        });
        id
    }

    pub fn set_interval(&mut self, callback: TimerFn, interval_ms: u64) -> u64 {
        let mut inner = self.inner.lock().unwrap();
        let id = inner.next_id;
        inner.next_id += 1;
        let now = inner.time;
        inner.timers.push(ScheduledTimer {
            id,
            fire_at: now + Duration::from_millis(interval_ms),
            delay: Duration::from_millis(interval_ms),
            repeat: true,
            callback_fn: callback,
        });
        id
    }

    pub fn clear_timeout(&mut self, id: u64) {
        let mut inner = self.inner.lock().unwrap();
        inner.timers.retain(|t| t.id != id);
    }

    fn advance_to_inner(inner: &mut ManualClockInner, target: Duration) {
        loop {
            let next = Self::next_fire_time_inner(inner);
            match next {
                Some(t) if t <= target => {
                    inner.time = t;
                    Self::fire_due_timers_inner(inner);
                }
                _ => {
                    inner.time = target;
                    break;
                }
            }
        }
    }

    fn next_fire_time_inner(inner: &ManualClockInner) -> Option<Duration> {
        inner.timers.iter().map(|t| t.fire_at).min()
    }

    fn fire_due_timers_inner(inner: &mut ManualClockInner) {
        let due: Vec<usize> =
            inner.timers.iter().enumerate().filter(|(_, t)| t.fire_at <= inner.time).map(|(i, _)| i).collect();

        for idx in due.into_iter().rev() {
            let timer = inner.timers.remove(idx);
            (timer.callback_fn)();
            if timer.repeat {
                inner.timers.push(ScheduledTimer { fire_at: inner.time + timer.delay, ..timer });
            }
        }
    }

    pub fn clear_interval(&mut self, id: u64) {
        self.clear_timeout(id);
    }
}

impl Clone for ManualClock {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

impl Clock for ManualClock {
    fn now_ms(&self) -> u64 {
        self.inner.lock().unwrap().time.as_millis() as u64
    }
    fn now(&self) -> Duration {
        self.inner.lock().unwrap().time
    }
}

impl Default for ManualClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

    #[test]
    fn manual_clock_starts_at_zero() {
        let clock = ManualClock::new();
        assert_eq!(clock.now_ms(), 0);
    }

    #[test]
    fn manual_clock_advance() {
        let mut clock = ManualClock::new();
        clock.advance(100);
        assert_eq!(clock.now_ms(), 100);
    }

    #[test]
    fn manual_clock_set_timeout() {
        static FIRED: AtomicBool = AtomicBool::new(false);
        FIRED.store(false, Ordering::SeqCst);
        let mut clock = ManualClock::new();
        clock.set_timeout(
            || {
                FIRED.store(true, Ordering::SeqCst);
            },
            50,
        );
        clock.advance(49);
        assert!(!FIRED.load(Ordering::SeqCst));
        clock.advance(2);
        assert!(FIRED.load(Ordering::SeqCst));
    }

    #[test]
    fn manual_clock_run_all() {
        static COUNT: AtomicU64 = AtomicU64::new(0);
        COUNT.store(0, Ordering::SeqCst);
        let mut clock = ManualClock::new();
        clock.set_timeout(
            || {
                COUNT.fetch_add(1, Ordering::SeqCst);
            },
            10,
        );
        clock.set_timeout(
            || {
                COUNT.fetch_add(1, Ordering::SeqCst);
            },
            20,
        );
        clock.run_all();
        assert_eq!(COUNT.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn clock_trait_object() {
        let sys: Box<dyn Clock> = Box::new(SystemClock::new());
        assert!(sys.is_monotonic());
        let manual: Box<dyn Clock> = Box::new(ManualClock::new());
        assert_eq!(manual.now_ms(), 0);
    }
}
