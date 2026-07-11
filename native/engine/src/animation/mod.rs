/// Easing functions for animations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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
                    -2.0_f32.powf(10.0 * t - 10.0)
                        * ((t * 10.0 - 10.75) * std::f32::consts::TAU / 3.0).sin()
                }
            }
            Self::EaseOutElastic => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    2.0_f32.powf(-10.0 * t)
                        * ((t * 10.0 - 0.75) * std::f32::consts::TAU / 3.0).sin()
                        + 1.0
                }
            }
            Self::EaseInOutElastic => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    -(2.0_f32.powf(20.0 * t - 10.0)
                        * ((20.0 * t - 11.125) * std::f32::consts::TAU / 4.5).sin())
                        / 2.0
                } else {
                    (2.0_f32.powf(-20.0 * t + 10.0)
                        * ((20.0 * t - 11.125) * std::f32::consts::TAU / 4.5).sin())
                        / 2.0
                        + 1.0
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
        Self {
            from: 0.0,
            to: 1.0,
            duration: 1.0,
            easing: Easing::Linear,
            delay: 0.0,
        }
    }
}

impl Tween {
    pub fn new(from: f32, to: f32, duration: f32) -> Self {
        Self {
            from,
            to,
            duration,
            ..Default::default()
        }
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
        Self {
            target: 1.0,
            stiffness: 170.0,
            damping: 26.0,
            mass: 1.0,
            velocity: 0.0,
            precision: 0.01,
        }
    }
}

impl Spring {
    pub fn new(target: f32) -> Self {
        Self {
            target,
            ..Default::default()
        }
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

        let is_settled =
            displacement.abs() < self.precision && self.velocity.abs() < self.precision;

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
        Self {
            keyframes: Vec::new(),
            easing: Easing::Linear,
        }
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
        self.keyframes
            .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
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
    pub on_complete: Option<Box<dyn Fn() + Send + Sync>>,
    pub on_update: Option<Box<dyn Fn(f32) + Send + Sync>>,
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
        Self {
            id,
            tween: Some(tween),
            state: AnimationState::Idle,
            ..Default::default()
        }
    }

    pub fn from_spring(spring: Spring, id: u32) -> Self {
        let target = spring.target;
        Self {
            id,
            spring: Some(spring),
            state: AnimationState::Idle,
            current_value: target,
            ..Default::default()
        }
    }

    pub fn from_keyframes(keyframes: Keyframes, id: u32) -> Self {
        Self {
            id,
            keyframes: Some(keyframes),
            state: AnimationState::Idle,
            ..Default::default()
        }
    }

    pub fn with_on_complete(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_complete = Some(Box::new(handler));
        self
    }

    pub fn with_on_update(mut self, handler: impl Fn(f32) + Send + Sync + 'static) -> Self {
        self.on_update = Some(Box::new(handler));
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
        Self {
            animations: Vec::new(),
            next_id: 1,
        }
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
        self.animations
            .retain(|a| a.state != AnimationState::Completed);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easing_linear() {
        let easing = Easing::Linear;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.5), 0.5);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn easing_clamps_input() {
        let easing = Easing::Linear;
        assert_eq!(easing.apply(-0.5), 0.0);
        assert_eq!(easing.apply(1.5), 1.0);
    }

    #[test]
    fn easingEaseIn() {
        let easing = Easing::EaseIn;
        assert!((easing.apply(0.25) - 0.0625).abs() < 0.001);
        assert!((easing.apply(0.5) - 0.25).abs() < 0.001);
    }

    #[test]
    fn easingEaseOut() {
        let easing = Easing::EaseOut;
        assert!((easing.apply(0.25) - 0.4375).abs() < 0.001);
        assert!((easing.apply(0.5) - 0.75).abs() < 0.001);
    }

    #[test]
    fn tween_basic() {
        let tween = Tween::new(0.0, 100.0, 1.0);
        assert_eq!(tween.value_at(0.0), 0.0);
        assert_eq!(tween.value_at(0.5), 50.0);
        assert_eq!(tween.value_at(1.0), 100.0);
    }

    #[test]
    fn tween_with_delay() {
        let tween = Tween::new(0.0, 100.0, 1.0).with_delay(0.5);
        assert_eq!(tween.value_at(0.0), 0.0);
        assert_eq!(tween.value_at(0.25), 0.0);
        assert_eq!(tween.value_at(0.5), 0.0);
        assert_eq!(tween.value_at(0.75), 25.0);
        assert_eq!(tween.value_at(1.5), 100.0);
    }

    #[test]
    fn tween_with_easing() {
        let tween = Tween::new(0.0, 100.0, 1.0).with_easing(Easing::EaseIn);
        assert_eq!(tween.value_at(0.0), 0.0);
        assert!((tween.value_at(0.5) - 25.0).abs() < 0.001);
        assert_eq!(tween.value_at(1.0), 100.0);
    }

    #[test]
    fn tween_is_complete() {
        let tween = Tween::new(0.0, 100.0, 1.0);
        assert!(!tween.is_complete(0.5));
        assert!(tween.is_complete(1.0));
        assert!(tween.is_complete(1.5));
    }

    #[test]
    fn spring_basic() {
        let mut spring = Spring::new(100.0).with_stiffness(100.0).with_damping(10.0);
        let (value, _) = spring.update(0.0, 0.016);
        assert!(value > 0.0);
        assert!(value < 100.0);
    }

    #[test]
    fn spring_settles() {
        let mut spring = Spring::new(100.0).with_stiffness(100.0).with_damping(20.0);
        let mut value = 0.0;
        for _ in 0..1000 {
            let (new_value, is_settled) = spring.update(value, 0.016);
            value = new_value;
            if is_settled {
                break;
            }
        }
        assert!((value - 100.0).abs() < 0.1);
    }

    #[test]
    fn keyframes_basic() {
        let keyframes = Keyframes::new()
            .add_keyframe(0.0, 0.0)
            .add_keyframe(1.0, 100.0);
        assert_eq!(keyframes.value_at(0.0), 0.0);
        assert_eq!(keyframes.value_at(0.5), 50.0);
        assert_eq!(keyframes.value_at(1.0), 100.0);
    }

    #[test]
    fn keyframes_multiple() {
        let keyframes = Keyframes::new()
            .add_keyframe(0.0, 0.0)
            .add_keyframe(0.5, 100.0)
            .add_keyframe(1.0, 50.0);
        assert_eq!(keyframes.value_at(0.0), 0.0);
        assert_eq!(keyframes.value_at(0.25), 50.0);
        assert_eq!(keyframes.value_at(0.5), 100.0);
        assert_eq!(keyframes.value_at(0.75), 75.0);
        assert_eq!(keyframes.value_at(1.0), 50.0);
    }

    #[test]
    fn keyframes_empty() {
        let keyframes = Keyframes::new();
        assert_eq!(keyframes.value_at(0.0), 0.0);
    }

    #[test]
    fn animation_from_tween() {
        let mut animation = Animation::from_tween(Tween::new(0.0, 100.0, 1.0), 1);
        assert_eq!(animation.state, AnimationState::Idle);
        animation.play();
        assert_eq!(animation.state, AnimationState::Playing);
        animation.update(0.5);
        assert_eq!(animation.current_value, 50.0);
        animation.update(0.5);
        assert_eq!(animation.state, AnimationState::Completed);
        assert_eq!(animation.current_value, 100.0);
    }

    #[test]
    fn animation_pause_resume() {
        let mut animation = Animation::from_tween(Tween::new(0.0, 100.0, 1.0), 1);
        animation.play();
        animation.update(0.5);
        animation.pause();
        assert_eq!(animation.state, AnimationState::Paused);
        animation.update(0.5);
        assert_eq!(animation.current_value, 50.0);
        animation.resume();
        assert_eq!(animation.state, AnimationState::Playing);
        animation.update(0.5);
        assert_eq!(animation.state, AnimationState::Completed);
    }

    #[test]
    fn animation_reset() {
        let mut animation = Animation::from_tween(Tween::new(0.0, 100.0, 1.0), 1);
        animation.play();
        animation.update(0.5);
        animation.reset();
        assert_eq!(animation.state, AnimationState::Idle);
        assert_eq!(animation.elapsed, 0.0);
        assert_eq!(animation.current_value, 0.0);
    }

    #[test]
    fn engine_new() {
        let engine = AnimationEngine::new();
        assert_eq!(engine.active_count(), 0);
        assert!(!engine.is_running());
    }

    #[test]
    fn engine_tween() {
        let mut engine = AnimationEngine::new();
        engine.tween(0.0, 100.0, 1.0);
        assert_eq!(engine.active_count(), 1);
        assert!(engine.is_running());
    }

    #[test]
    fn engine_update() {
        let mut engine = AnimationEngine::new();
        engine.tween(0.0, 100.0, 1.0);
        engine.update(0.5);
        assert_eq!(engine.active_count(), 1);
        engine.update(0.5);
        assert_eq!(engine.active_count(), 0);
    }

    #[test]
    fn engine_cancel_all() {
        let mut engine = AnimationEngine::new();
        engine.tween(0.0, 100.0, 1.0);
        engine.tween(0.0, 50.0, 0.5);
        engine.cancel_all();
        assert_eq!(engine.active_count(), 0);
    }

    #[test]
    fn engine_spring() {
        let mut engine = AnimationEngine::new();
        engine.spring(100.0);
        assert_eq!(engine.active_count(), 1);
    }

    #[test]
    fn engine_keyframes() {
        let mut engine = AnimationEngine::new();
        let keyframes = Keyframes::new()
            .add_keyframe(0.0, 0.0)
            .add_keyframe(1.0, 100.0);
        engine.keyframes(keyframes);
        assert_eq!(engine.active_count(), 1);
    }
}
