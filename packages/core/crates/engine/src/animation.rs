//! Animation engine: easing functions, springs, tweens, keyframes, and animation state management.

use std::sync::Arc;

/// Easing functions for animations.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Easing {
    #[default]
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    /// Cubic bezier with two control points (like CSS cubic-bezier)
    CubicBezier(f32, f32, f32, f32),
    /// Steps with step count and jump mode
    Steps(u32, StepJump),
}

/// Jump mode for step animations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepJump {
    JumpNone,
    JumpStart,
    #[default]
    JumpEnd,
    JumpBoth,
}

/// Solve cubic bezier for a given t using Newton-Raphson iteration.
fn cubic_bezier(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    // Find the x value at parameter t using Newton-Raphson, then evaluate y
    let cx = 3.0 * x1;
    let bx = 3.0 * (x2 - x1) - cx;
    let ax = 1.0 - cx - bx;

    let cy = 3.0 * y1;
    let by = 3.0 * (y2 - y1) - cy;
    let ay = 1.0 - cy - by;

    // Solve for parameter u where x(u) = t
    let u = sample_curve_x(t, ax, bx, cx);

    // Evaluate y at that parameter
    sample_curve_y(u, ay, by, cy)
}

fn sample_curve_x(t: f32, ax: f32, bx: f32, cx: f32) -> f32 {
    // Newton-Raphson iteration
    let mut u = t;
    for _ in 0..8 {
        let x = ((ax * u + bx) * u + cx) * u - t;
        if x.abs() < 1e-6 {
            return u;
        }
        let dx = (3.0 * ax * u + 2.0 * bx) * u + cx;
        if dx.abs() < 1e-6 {
            break;
        }
        u -= x / dx;
    }
    // Fall back to bisection if Newton-Raphson fails
    let mut a = 0.0f32;
    let mut b = 1.0f32;
    u = t;
    for _ in 0..20 {
        let x = ((ax * u + bx) * u + cx) * u;
        if (x - t).abs() < 1e-6 {
            return u;
        }
        if x < t {
            a = u;
        } else {
            b = u;
        }
        u = (a + b) / 2.0;
    }
    u
}

fn sample_curve_y(t: f32, ay: f32, by: f32, cy: f32) -> f32 {
    ((ay * t + by) * t + cy) * t
}

impl Easing {
    /// Apply easing function to a progress value (0.0 to 1.0).
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Self::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Self::EaseInQuad => t * t,
            Self::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Self::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Self::EaseInCubic => t * t * t,
            Self::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Self::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Self::EaseInExpo => {
                if t == 0.0 {
                    0.0
                } else {
                    2.0_f32.powf(10.0 * t - 10.0)
                }
            }
            Self::EaseOutExpo => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - 2.0_f32.powf(-10.0 * t)
                }
            }
            Self::EaseInOutExpo => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    2.0_f32.powf(20.0 * t - 10.0) / 2.0
                } else {
                    (2.0 - 2.0_f32.powf(-20.0 * t + 10.0)) / 2.0
                }
            }
            Self::EaseInBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                let t = 1.0 - t;
                if t < 1.0 / d1 {
                    1.0 - n1 * t * t
                } else if t < 2.0 / d1 {
                    1.0 - n1 * (t - 1.5 / d1).powi(2) - 0.75
                } else if t < 2.5 / d1 {
                    1.0 - n1 * (t - 2.25 / d1).powi(2) - 0.9375
                } else {
                    1.0 - n1 * (t - 2.625 / d1).powi(2) - 0.984375
                }
            }
            Self::EaseOutBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    n1 * (t - 1.5 / d1).powi(2) + 0.75
                } else if t < 2.5 / d1 {
                    n1 * (t - 2.25 / d1).powi(2) + 0.9375
                } else {
                    n1 * (t - 2.625 / d1).powi(2) + 0.984375
                }
            }
            Self::EaseInOutBounce => {
                if t < 0.5 {
                    (1.0 - Self::EaseOutBounce.apply(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + Self::EaseOutBounce.apply(2.0 * t - 1.0)) / 2.0
                }
            }
            Self::EaseInElastic => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    -2.0_f32.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * std::f32::consts::TAU / 3.0).sin()
                }
            }
            Self::EaseOutElastic => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * std::f32::consts::TAU / 3.0).sin() + 1.0
                }
            }
            Self::EaseInOutElastic => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    -(2.0_f32.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * std::f32::consts::TAU / 4.5).sin()) / 2.0
                } else {
                    (2.0_f32.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * std::f32::consts::TAU / 4.5).sin()) / 2.0
                        + 1.0
                }
            }
            Self::EaseInCirc => 1.0 - (1.0 - t * t).sqrt(),
            Self::EaseOutCirc => (1.0 - (t - 1.0).powi(2)).sqrt(),
            Self::EaseInOutCirc => {
                if t < 0.5 {
                    -0.5 * ((1.0 - (2.0 * t).powi(2)).sqrt() - 1.0)
                } else {
                    0.5 * ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0)
                }
            }
            Self::EaseInBack => {
                let s = 1.70158;
                t * t * ((s + 1.0) * t - s)
            }
            Self::EaseOutBack => {
                let s = 1.70158;
                let t = t - 1.0;
                t * t * ((s + 1.0) * t + s) + 1.0
            }
            Self::EaseInOutBack => {
                let s = 1.70158 * 1.525;
                let t2 = t * 2.0;
                if t2 < 1.0 {
                    0.5 * (t2 * t2 * ((s + 1.0) * t2 - s))
                } else {
                    let t2 = t2 - 2.0;
                    0.5 * (t2 * t2 * ((s + 1.0) * t2 + s) + 2.0)
                }
            }
            Self::CubicBezier(x1, y1, x2, y2) => cubic_bezier(t, *x1, *y1, *x2, *y2),
            Self::Steps(steps, jump) => {
                let steps = (*steps).max(1) as f32;
                match jump {
                    StepJump::JumpNone => (t * steps).floor() / steps,
                    StepJump::JumpStart => (t * steps).ceil() / steps,
                    StepJump::JumpEnd => {
                        if t >= 1.0 {
                            1.0
                        } else {
                            (t * steps).floor() / steps
                        }
                    }
                    StepJump::JumpBoth => {
                        if t >= 1.0 {
                            1.0
                        } else {
                            (t * steps).ceil() / steps
                        }
                    }
                }
            }
        }
    }
}

/// Tween animation for interpolating between values.
#[derive(Debug, Clone)]
pub struct Tween {
    pub from: f32,
    pub to: f32,
    pub duration: f32,
    pub easing: Easing,
    pub delay: f32,
}

impl Default for Tween {
    fn default() -> Self {
        Self { from: 0.0, to: 1.0, duration: 1.0, easing: Easing::Linear, delay: 0.0 }
    }
}

impl Tween {
    pub fn new(from: f32, to: f32, duration: f32) -> Self {
        Self { from, to, duration, ..Default::default() }
    }

    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    pub fn with_delay(mut self, delay: f32) -> Self {
        self.delay = delay;
        self
    }

    /// Get the interpolated value at the given time.
    pub fn value_at(&self, time: f32) -> f32 {
        let t = (time - self.delay) / self.duration;
        let t = t.clamp(0.0, 1.0);
        let eased = self.easing.apply(t);
        self.from + (self.to - self.from) * eased
    }

    /// Check if the animation is complete at the given time.
    pub fn is_complete(&self, time: f32) -> bool {
        time >= self.delay + self.duration
    }
}

/// Spring physics animation.
#[derive(Debug, Clone)]
pub struct Spring {
    pub target: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
    pub velocity: f32,
    pub precision: f32,
}

impl Default for Spring {
    fn default() -> Self {
        Self { target: 1.0, stiffness: 170.0, damping: 26.0, mass: 1.0, velocity: 0.0, precision: 0.01 }
    }
}

impl Spring {
    pub fn new(target: f32) -> Self {
        Self { target, ..Default::default() }
    }

    pub fn with_stiffness(mut self, stiffness: f32) -> Self {
        self.stiffness = stiffness;
        self
    }

    pub fn with_damping(mut self, damping: f32) -> Self {
        self.damping = damping;
        self
    }

    pub fn with_mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    pub fn with_velocity(mut self, velocity: f32) -> Self {
        self.velocity = velocity;
        self
    }

    /// Update the spring by the given time step.
    /// Returns the new value and whether the spring has settled.
    pub fn update(&mut self, current: f32, dt: f32) -> (f32, bool) {
        let displacement = current - self.target;
        let spring_force = -self.stiffness * displacement;
        let damping_force = -self.damping * self.velocity;
        let acceleration = (spring_force + damping_force) / self.mass;

        self.velocity += acceleration * dt;
        let new_value = current + self.velocity * dt;

        let is_settled = displacement.abs() < self.precision && self.velocity.abs() < self.precision;

        (new_value, is_settled)
    }
}

/// Keyframe animation with multiple waypoints.
#[derive(Debug, Clone)]
pub struct Keyframes {
    pub keyframes: Vec<Keyframe>,
    pub easing: Easing,
}

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
}

impl Default for Keyframes {
    fn default() -> Self {
        Self { keyframes: Vec::new(), easing: Easing::Linear }
    }
}

impl Keyframes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    pub fn add_keyframe(mut self, time: f32, value: f32) -> Self {
        self.keyframes.push(Keyframe { time, value });
        self.keyframes.sort_by(|a, b| a.time.total_cmp(&b.time));
        self
    }

    /// Get the interpolated value at the given time.
    pub fn value_at(&self, time: f32) -> f32 {
        if self.keyframes.is_empty() {
            return 0.0;
        }

        if time <= self.keyframes[0].time {
            return self.keyframes[0].value;
        }

        if time >= self.keyframes.last().unwrap().time {
            return self.keyframes.last().unwrap().value;
        }

        for i in 0..self.keyframes.len() - 1 {
            let kf1 = &self.keyframes[i];
            let kf2 = &self.keyframes[i + 1];

            if time >= kf1.time && time <= kf2.time {
                let t = (time - kf1.time) / (kf2.time - kf1.time);
                let eased = self.easing.apply(t);
                return kf1.value + (kf2.value - kf1.value) * eased;
            }
        }

        self.keyframes.last().unwrap().value
    }
}

/// Animation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Playing,
    Paused,
    Completed,
}

/// A running animation instance.
pub struct Animation {
    pub id: u32,
    pub tween: Option<Tween>,
    pub spring: Option<Spring>,
    pub keyframes: Option<Keyframes>,
    pub state: AnimationState,
    pub elapsed: f32,
    pub current_value: f32,
    pub on_complete: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_update: Option<Arc<dyn Fn(f32) + Send + Sync>>,
}

impl Clone for Animation {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            tween: self.tween.clone(),
            spring: self.spring.clone(),
            keyframes: self.keyframes.clone(),
            state: self.state,
            elapsed: self.elapsed,
            current_value: self.current_value,
            on_complete: self.on_complete.clone(),
            on_update: self.on_update.clone(),
        }
    }
}

impl std::fmt::Debug for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Animation")
            .field("id", &self.id)
            .field("tween", &self.tween)
            .field("spring", &self.spring)
            .field("keyframes", &self.keyframes)
            .field("state", &self.state)
            .field("elapsed", &self.elapsed)
            .field("current_value", &self.current_value)
            .field("on_complete", &self.on_complete.is_some())
            .field("on_update", &self.on_update.is_some())
            .finish()
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            id: 0,
            tween: None,
            spring: None,
            keyframes: None,
            state: AnimationState::Idle,
            elapsed: 0.0,
            current_value: 0.0,
            on_complete: None,
            on_update: None,
        }
    }
}

impl Animation {
    pub fn from_tween(tween: Tween, id: u32) -> Self {
        Self { id, tween: Some(tween), state: AnimationState::Idle, ..Default::default() }
    }

    pub fn from_spring(spring: Spring, id: u32) -> Self {
        let target = spring.target;
        Self { id, spring: Some(spring), state: AnimationState::Idle, current_value: target, ..Default::default() }
    }

    pub fn from_keyframes(keyframes: Keyframes, id: u32) -> Self {
        Self { id, keyframes: Some(keyframes), state: AnimationState::Idle, ..Default::default() }
    }

    pub fn with_on_complete(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_complete = Some(Arc::new(handler));
        self
    }

    pub fn with_on_update(mut self, handler: impl Fn(f32) + Send + Sync + 'static) -> Self {
        self.on_update = Some(Arc::new(handler));
        self
    }

    pub fn play(&mut self) {
        self.state = AnimationState::Playing;
        self.elapsed = 0.0;
    }

    pub fn pause(&mut self) {
        self.state = AnimationState::Paused;
    }

    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            self.state = AnimationState::Playing;
        }
    }

    pub fn reset(&mut self) {
        self.state = AnimationState::Idle;
        self.elapsed = 0.0;
        self.current_value = 0.0;
    }

    /// Update the animation by the given time step.
    pub fn update(&mut self, dt: f32) {
        if self.state != AnimationState::Playing {
            return;
        }

        self.elapsed += dt;

        if let Some(ref tween) = self.tween {
            self.current_value = tween.value_at(self.elapsed);
            if tween.is_complete(self.elapsed) {
                self.state = AnimationState::Completed;
                if let Some(ref handler) = self.on_complete {
                    handler();
                }
            }
        } else if let Some(ref mut spring) = self.spring {
            let (new_value, is_settled) = spring.update(self.current_value, dt);
            self.current_value = new_value;
            if is_settled {
                self.state = AnimationState::Completed;
                if let Some(ref handler) = self.on_complete {
                    handler();
                }
            }
        } else if let Some(ref keyframes) = self.keyframes {
            self.current_value = keyframes.value_at(self.elapsed);
            if let Some(last_kf) = keyframes.keyframes.last()
                && self.elapsed >= last_kf.time
            {
                self.state = AnimationState::Completed;
                if let Some(ref handler) = self.on_complete {
                    handler();
                }
            }
        }

        if let Some(ref handler) = self.on_update {
            handler(self.current_value);
        }
    }
}

/// Animation engine that manages multiple animations.
pub struct AnimationEngine {
    animations: Vec<Animation>,
    next_id: u32,
}

impl Default for AnimationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationEngine {
    pub fn new() -> Self {
        Self { animations: Vec::new(), next_id: 1 }
    }

    /// Create a new tween animation.
    pub fn tween(&mut self, from: f32, to: f32, duration: f32) -> &mut Animation {
        let id = self.next_id;
        self.next_id += 1;
        let mut animation = Animation::from_tween(Tween::new(from, to, duration), id);
        animation.play();
        self.animations.push(animation);
        self.animations.last_mut().unwrap()
    }

    /// Create a new spring animation.
    pub fn spring(&mut self, target: f32) -> &mut Animation {
        let id = self.next_id;
        self.next_id += 1;
        let mut animation = Animation::from_spring(Spring::new(target), id);
        animation.play();
        self.animations.push(animation);
        self.animations.last_mut().unwrap()
    }

    /// Create a new keyframes animation.
    pub fn keyframes(&mut self, keyframes: Keyframes) -> &mut Animation {
        let id = self.next_id;
        self.next_id += 1;
        let mut animation = Animation::from_keyframes(keyframes, id);
        animation.play();
        self.animations.push(animation);
        self.animations.last_mut().unwrap()
    }

    /// Update all animations by the given time step.
    pub fn update(&mut self, dt: f32) {
        for animation in &mut self.animations {
            animation.update(dt);
        }

        // Remove completed animations
        self.animations.retain(|a| a.state != AnimationState::Completed);
    }

    /// Get the number of active animations.
    pub fn active_count(&self) -> usize {
        self.animations.len()
    }

    /// Check if there are any active animations.
    pub fn is_running(&self) -> bool {
        !self.animations.is_empty()
    }

    /// Cancel all animations.
    pub fn cancel_all(&mut self) {
        for animation in &mut self.animations {
            animation.reset();
        }
        self.animations.clear();
    }
}

// ─── Timeline ────────────────────────────────────────────────────────────────

/// A timeline item type
#[derive(Debug, Clone)]
pub enum TimelineItemType {
    /// An animation item with start time
    Animation(TimelineAnimationItem),
    /// A callback to execute at a specific time
    Callback(TimelineCallbackItem),
}

/// An animation scheduled on a timeline
#[derive(Debug, Clone)]
pub struct TimelineAnimationItem {
    /// Start time in seconds
    pub start_time: f32,
    /// The animation to run
    pub animation: Animation,
    /// Whether this item has been started
    pub started: bool,
    /// Whether this item has completed
    pub completed: bool,
}

/// A callback scheduled on a timeline
#[derive(Debug, Clone)]
pub struct TimelineCallbackItem {
    /// When to execute
    pub start_time: f32,
    /// The callback to execute
    pub executed: bool,
}

/// A timeline for managing animation playback with time scaling and sequencing
pub struct Timeline {
    /// Current time in seconds
    current_time: f32,
    /// Playback speed (1.0 = normal, 2.0 = 2x, etc.)
    speed: f32,
    /// Whether the timeline is playing
    playing: bool,
    /// Duration limit (None = unlimited)
    duration: Option<f32>,
    /// Whether to loop
    looping: bool,
    /// Whether the timeline has completed
    complete: bool,
    /// Scheduled items
    items: Vec<TimelineItemType>,
    /// Sub-timelines synced to this timeline
    sub_timelines: Vec<SubTimeline>,
    /// Callback when timeline completes
    on_complete: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl std::fmt::Debug for Timeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timeline")
            .field("current_time", &self.current_time)
            .field("speed", &self.speed)
            .field("playing", &self.playing)
            .field("duration", &self.duration)
            .field("looping", &self.looping)
            .field("complete", &self.complete)
            .field("items", &self.items)
            .field("sub_timelines", &self.sub_timelines)
            .field("on_complete", &self.on_complete.is_some())
            .finish()
    }
}

/// A sub-timeline synced to a parent
#[derive(Debug, Clone)]
#[expect(dead_code, reason = "sub-timeline infrastructure for future use")]
struct SubTimeline {
    /// Start time in parent timeline
    start_time: f32,
    /// Whether this sub-timeline has been started
    started: bool,
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            current_time: 0.0,
            speed: 1.0,
            playing: false,
            duration: None,
            looping: false,
            complete: false,
            items: Vec::new(),
            sub_timelines: Vec::new(),
            on_complete: None,
        }
    }

    /// Set the duration limit
    pub fn with_duration(mut self, duration: f32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set looping
    pub fn with_looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    /// Set callback for when timeline completes
    pub fn with_on_complete(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_complete = Some(Arc::new(handler));
        self
    }

    /// Set playback speed
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    /// Get playback speed
    pub fn speed(&self) -> f32 {
        self.speed
    }

    /// Add an animation at a specific start time (in seconds)
    pub fn add_animation(&mut self, animation: Animation, start_time: f32) {
        self.items.push(TimelineItemType::Animation(TimelineAnimationItem {
            start_time,
            animation,
            started: false,
            completed: false,
        }));
    }

    /// Add a callback at a specific start time (in seconds)
    pub fn add_callback(&mut self, start_time: f32) {
        self.items.push(TimelineItemType::Callback(TimelineCallbackItem { start_time, executed: false }));
    }

    /// Check if the timeline has completed
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Play the timeline
    pub fn play(&mut self) {
        if self.complete {
            self.restart();
            return;
        }
        self.playing = true;
    }

    /// Pause the timeline
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Restart the timeline from the beginning
    pub fn restart(&mut self) {
        self.current_time = 0.0;
        self.complete = false;
        self.playing = true;
        self.reset_items();
    }

    /// Reset all items to their initial state
    fn reset_items(&mut self) {
        for item in &mut self.items {
            match item {
                TimelineItemType::Animation(a) => {
                    a.started = false;
                    a.completed = false;
                    a.animation.reset();
                }
                TimelineItemType::Callback(c) => {
                    c.executed = false;
                }
            }
        }
    }

    /// Check if playing
    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// Get current time
    pub fn current_time(&self) -> f32 {
        self.current_time
    }

    /// Set current time
    pub fn set_time(&mut self, time: f32) {
        self.current_time = time;
    }

    /// Reset to start
    pub fn reset(&mut self) {
        self.current_time = 0.0;
        self.complete = false;
        self.reset_items();
    }

    /// Update the timeline by dt
    pub fn update(&mut self, dt: f32) {
        if !self.playing || self.complete {
            return;
        }

        self.current_time += dt * self.speed;

        // Evaluate scheduled items
        for item in &mut self.items {
            match item {
                TimelineItemType::Animation(a) => {
                    if self.current_time >= a.start_time && !a.completed {
                        if !a.started {
                            a.started = true;
                            a.animation.play();
                        }
                        a.animation.update(dt * self.speed);
                        if a.animation.state == AnimationState::Completed {
                            a.completed = true;
                        }
                    }
                }
                TimelineItemType::Callback(c) => {
                    if self.current_time >= c.start_time && !c.executed {
                        c.executed = true;
                    }
                }
            }
        }

        // Check if duration limit reached
        if let Some(duration) = self.duration
            && self.current_time >= duration
        {
            if self.looping {
                self.current_time %= duration;
                self.reset_items();
            } else {
                self.current_time = duration;
                self.playing = false;
                self.complete = true;
                if let Some(ref handler) = self.on_complete {
                    handler();
                }
            }
        }
    }

    /// Get progress (0.0 to 1.0) if duration is set
    pub fn progress(&self) -> Option<f32> {
        self.duration.map(|d| if d > 0.0 { self.current_time / d } else { 0.0 })
    }
}

// ─── Color Interpolation ─────────────────────────────────────────────────────

/// RGBA color for animation interpolation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnimColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl AnimColor {
    /// Create RGB color (alpha = 255)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create RGBA color
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Parse from hex string (#RGB, #RGBA, #RRGGBB, #RRGGBBAA)
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
                Some(Self::rgb(r, g, b))
            }
            4 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
                let a = u8::from_str_radix(&hex[3..4], 16).ok()? * 17;
                Some(Self::rgba(r, g, b, a))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Self::rgba(r, g, b, a))
            }
            _ => None,
        }
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
        }
    }

    /// Linearly interpolate between two colors
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let inv_t = 1.0 - t;
        Self {
            r: (self.r as f32 * inv_t + other.r as f32 * t) as u8,
            g: (self.g as f32 * inv_t + other.g as f32 * t) as u8,
            b: (self.b as f32 * inv_t + other.b as f32 * t) as u8,
            a: (self.a as f32 * inv_t + other.a as f32 * t) as u8,
        }
    }
}

impl Default for AnimColor {
    fn default() -> Self {
        Self::rgb(0, 0, 0)
    }
}

// ─── Animatable Property ─────────────────────────────────────────────────────

/// Properties that can be animated
#[derive(Debug, Clone, PartialEq)]
pub enum AnimatableProperty {
    /// Opacity (0.0 to 1.0)
    Opacity,
    /// Width in cells
    Width,
    /// Height in cells
    Height,
    /// X position
    TranslateX,
    /// Y position
    TranslateY,
    /// Padding top
    PaddingTop,
    /// Padding right
    PaddingRight,
    /// Padding bottom
    PaddingBottom,
    /// Padding left
    PaddingLeft,
    /// Margin top
    MarginTop,
    /// Margin right
    MarginRight,
    /// Margin bottom
    MarginBottom,
    /// Margin left
    MarginLeft,
    /// Foreground color
    Foreground,
    /// Background color
    Background,
    /// Custom property (by name)
    Custom(String),
}

/// Animated value that can be numeric or color
#[derive(Debug, Clone)]
pub enum AnimatableValue {
    /// Numeric value
    Float(f32),
    /// Color value
    Color(AnimColor),
}

impl AnimatableValue {
    /// Interpolate between two values
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (AnimatableValue::Float(a), AnimatableValue::Float(b)) => AnimatableValue::Float(a + (b - a) * t),
            (AnimatableValue::Color(a), AnimatableValue::Color(b)) => AnimatableValue::Color(a.lerp(b, t)),
            _ => other.clone(),
        }
    }
}

// ─── Property Animation ──────────────────────────────────────────────────────

/// An animation bound to a specific property
pub struct PropertyAnimation {
    /// The property to animate
    pub property: AnimatableProperty,
    /// The animation type
    pub animation: Animation,
    /// From value
    pub from: AnimatableValue,
    /// To value
    pub to: AnimatableValue,
}

impl PropertyAnimation {
    /// Create a new property animation
    pub fn new(property: AnimatableProperty, from: AnimatableValue, to: AnimatableValue, duration: f32) -> Self {
        let id = 0; // Will be assigned by engine
        let tween = Tween::new(0.0, 1.0, duration);
        Self { property, animation: Animation::from_tween(tween, id), from, to }
    }

    /// Get the current interpolated value
    pub fn current_value(&self) -> AnimatableValue {
        self.from.lerp(&self.to, self.animation.current_value)
    }

    /// Play the animation
    pub fn play(&mut self) {
        self.animation.play();
    }

    /// Update the animation
    pub fn update(&mut self, dt: f32) {
        self.animation.update(dt);
    }
}
