# Animation

> The animation system provides smooth, hardware-accelerated animations.
> It runs at 60fps and integrates with the rendering pipeline.

## 1. Overview

The animation system provides tween-based animations with keyframe support. It runs independently of the main render loop and updates node properties over time.

```
Animation System
    ↓ (updates node properties)
Node Arena (dirty flags set)
    ↓ (triggers render)
Rendering Pipeline
```

### 1.1 Why a Separate Animation System?

Animations require:
- **Time-based updates:** Properties change over time, not in response to user input.
- **Smooth interpolation:** Between start and end values.
- **Frame-accurate timing:** Updates must align with frame boundaries.
- **Cancellation:** Animations can be interrupted or cancelled.

These requirements are different from the event-driven model used for user input. A separate system avoids polluting the event system with time-based concerns.

## 2. Animation Model

### 2.1 Animation

```rust
pub struct Animation {
    pub id: AnimationId,
    pub target: NodeId,
    pub property: AnimatableProperty,
    pub keyframes: Vec<Keyframe>,
    pub timing: TimingFunction,
    pub duration: Duration,
    pub delay: Duration,
    pub iterations: IterationCount,
    pub direction: AnimationDirection,
    pub fill_mode: FillMode,
    pub state: AnimationState,
    pub current_time: Duration,
}
```

### 2.2 Keyframe

```rust
pub struct Keyframe {
    pub offset: f32,  // 0.0 to 1.0
    pub value: AnimatableValue,
    pub easing: EasingFunction,
}
```

### 2.3 AnimatableProperty

```rust
pub enum AnimatableProperty {
    // Style properties
    Foreground,
    Background,
    Opacity,

    // Layout properties
    Width,
    Height,
    PaddingTop,
    PaddingRight,
    PaddingBottom,
    PaddingLeft,
    MarginTop,
    MarginRight,
    MarginBottom,
    MarginLeft,
    FlexGrow,
    FlexShrink,

    // Transform properties
    TranslateX,
    TranslateY,

    // Scroll properties
    ScrollX,
    ScrollY,

    // Custom property
    Custom(u16),
}
```

### 2.4 AnimatableValue

```rust
pub enum AnimatableValue {
    Float(f32),
    Color(Color),
    Point(Point),
}
```

### 2.5 Timing Functions

```rust
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
    Steps(u32, StepPosition),
}

pub enum StepPosition {
    Start,
    End,
}
```

### 2.6 Timing

```rust
pub struct TimingFunction {
    pub easing: EasingFunction,
    pub duration: Duration,
    pub delay: Duration,
}
```

### 2.7 Iteration

```rust
pub enum IterationCount {
    Finite(u32),
    Infinite,
}

pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

pub enum FillMode {
    None,
    Forwards,
    Backwards,
    Both,
}
```

### 2.8 Animation State

```rust
pub enum AnimationState {
    Pending,
    Running,
    Paused,
    Completed,
    Cancelled,
}
```

## 3. Animation Engine

### 3.1 Engine Structure

```rust
pub struct AnimationEngine {
    animations: Vec<Animation>,
    timeline: Timeline,
    last_tick: Instant,
}
```

### 3.2 Timeline

```rust
pub struct Timeline {
    current_time: Duration,
    start_time: Instant,
    time_scale: f32,
}
```

### 3.3 Tick Processing

```rust
impl AnimationEngine {
    pub fn tick(&mut self, arena: &mut NodeArena) {
        let now = Instant::now();
        let delta = now - self.last_tick;
        self.last_tick = now;

        self.timeline.current_time += delta;

        for animation in &mut self.animations {
            if animation.state != AnimationState::Running {
                continue;
            }

            animation.current_time += delta;

            // Check if animation is complete
            if animation.current_time >= animation.duration + animation.delay {
                match animation.iterations {
                    IterationCount::Finite(n) => {
                        // Check if we've completed all iterations
                        // If yes, apply fill mode and mark complete
                    }
                    IterationCount::Infinite => {
                        // Reset to beginning
                        animation.current_time -= animation.duration;
                    }
                }
            }

            // Calculate interpolated value
            let progress = self.calculate_progress(animation);
            let value = self.interpolate(animation, progress);

            // Apply value to node
            self.apply_value(arena, animation.target, animation.property, value);
        }

        // Remove completed/cancelled animations
        self.animations.retain(|a| {
            a.state == AnimationState::Running || a.state == AnimationState::Paused
        });
    }
}
```

### 3.4 Progress Calculation

```rust
fn calculate_progress(&self, animation: &Animation) -> f32 {
    let elapsed = animation.current_time.as_secs_f32() - animation.delay.as_secs_f32();

    if elapsed < 0.0 {
        return 0.0; // In delay period
    }

    let raw_progress = elapsed / animation.duration.as_secs_f32();

    match animation.direction {
        AnimationDirection::Normal => raw_progress % 1.0,
        AnimationDirection::Reverse => 1.0 - (raw_progress % 1.0),
        AnimationDirection::Alternate => {
            let iteration = (raw_progress / 1.0) as u32;
            let t = raw_progress % 1.0;
            if iteration % 2 == 0 { t } else { 1.0 - t }
        }
        AnimationDirection::AlternateReverse => {
            let iteration = (raw_progress / 1.0) as u32;
            let t = raw_progress % 1.0;
            if iteration % 2 == 0 { 1.0 - t } else { t }
        }
    }
}
```

### 3.5 Interpolation

```rust
fn interpolate(&self, animation: &Animation, progress: f32) -> AnimatableValue {
    let easing = animation.timing.easing.apply(progress);

    match (&animation.keyframes[0].value, &animation.keyframes[1].value) {
        (AnimatableValue::Float(a), AnimatableValue::Float(b)) => {
            AnimatableValue::Float(a + (b - a) * easing)
        }
        (AnimatableValue::Color(a), AnimatableValue::Color(b)) => {
            AnimatableValue::Color(interpolate_color(a, b, easing))
        }
        (AnimatableValue::Point(a), AnimatableValue::Point(b)) => {
            AnimatableValue::Point(Point {
                x: a.x + ((b.x - a.x) as f32 * easing) as i32,
                y: a.y + ((b.y - a.y) as f32 * easing) as i32,
            })
        }
        _ => animation.keyframes[1].value.clone(),
    }
}
```

### 3.6 Color Interpolation

```rust
fn interpolate_color(a: &Color, b: &Color, t: f32) -> Color {
    match (a, b) {
        (Color::Rgb { r: r1, g: g1, b: b1 }, Color::Rgb { r: r2, g: g2, b: b2 }) => {
            Color::Rgb {
                r: (r1 as f32 + (*r2 as f32 - *r1 as f32) * t) as u8,
                g: (g1 as f32 + (*g2 as f32 - *g1 as f32) * t) as u8,
                b: (b1 as f32 + (*b2 as f32 - *b1 as f32) * t) as u8,
            }
        }
        // Handle other color types by converting to RGB first
        _ => {
            let rgb1 = a.to_rgb();
            let rgb2 = b.to_rgb();
            interpolate_color(&rgb1, &rgb2, t)
        }
    }
}
```

## 4. Easing Functions

### 4.1 Standard Easings

```rust
impl EasingFunction {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t).powi(3),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            EasingFunction::CubicBezier(x1, y1, x2, y2) => {
                // Newton-Raphson iteration for cubic bezier
                cubic_bezier(*x1, *y1, *x2, *y2, t)
            }
            EasingFunction::Steps(n, position) => {
                let step = match position {
                    StepPosition::Start => (t * *n as f32).ceil() / *n as f32,
                    StepPosition::End => (t * *n as f32).floor() / *n as f32,
                };
                step.clamp(0.0, 1.0)
            }
        }
    }
}
```

## 5. Animation API

### 5.1 Creating Animations

```rust
pub fn animate(
    engine: &mut AnimationEngine,
    target: NodeId,
    property: AnimatableProperty,
    from: AnimatableValue,
    to: AnimatableValue,
    duration: Duration,
    easing: EasingFunction,
) -> AnimationId {
    let animation = Animation {
        id: AnimationId::new(),
        target,
        property,
        keyframes: vec![
            Keyframe { offset: 0.0, value: from, easing: EasingFunction::Linear },
            Keyframe { offset: 1.0, value: to, easing },
        ],
        timing: TimingFunction {
            easing: EasingFunction::Linear,
            duration,
            delay: Duration::ZERO,
        },
        duration,
        delay: Duration::ZERO,
        iterations: IterationCount::Finite(1),
        direction: AnimationDirection::Normal,
        fill_mode: FillMode::Forwards,
        state: AnimationState::Running,
        current_time: Duration::ZERO,
    };

    engine.add(animation)
}
```

### 5.2 Chained Animations

```rust
pub fn chain_animations(
    engine: &mut AnimationEngine,
    animations: Vec<Animation>,
) -> AnimationId {
    // Set each animation's delay to the sum of previous durations
    let mut delay = Duration::ZERO;
    for mut anim in animations {
        anim.delay = delay;
        delay += anim.duration;
        engine.add(anim);
    }
}
```

### 5.3 Parallel Animations

```rust
pub fn parallel_animations(
    engine: &mut AnimationEngine,
    animations: Vec<Animation>,
) -> Vec<AnimationId> {
    animations.into_iter().map(|anim| engine.add(anim)).collect()
}
```

## 6. Spring Animations

### 6.1 Spring Model

```rust
pub struct SpringAnimation {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
    pub initial_velocity: f32,
    pub target: f32,
    pub current: f32,
    pub velocity: f32,
}
```

### 6.2 Spring Physics

```rust
impl SpringAnimation {
    pub fn tick(&mut self, dt: f32) -> bool {
        let spring_force = -self.stiffness * (self.current - self.target);
        let damping_force = -self.damping * self.velocity;
        let acceleration = (spring_force + damping_force) / self.mass;

        self.velocity += acceleration * dt;
        self.current += self.velocity * dt;

        // Check if spring has settled
        self.velocity.abs() < 0.01 && (self.current - self.target).abs() < 0.01
    }
}
```

## 7. Integration with Rendering

### 7.1 Frame-Accurate Updates

The animation engine ticks on every frame:

```
1. AnimationEngine.tick() updates all running animations
2. Changed properties set dirty flags on affected nodes
3. Layout is recalculated (if layout properties changed)
4. Frame is rendered
```

### 7.2 Batch Updates

Multiple property changes from animations are batched:

```
Animation 1: opacity 0 → 1
Animation 2: translate_x 0 → 10
    ↓
Batch: setOpacity(1), setTranslate(10, 0)
    ↓
Single layout + render pass
```

### 7.3 Performance

Animations should not cause layout thrashing:

1. **Batch property updates:** All animation updates are collected before triggering layout.
2. **Avoid layout-triggering animations:** Prefer opacity and transform (which don't affect layout).
3. **Throttle expensive animations:** Limit to 60fps even if the display supports higher refresh rates.

## 8. Future Considerations

### 8.1 Scroll-Linked Animations

Animations tied to scroll position:

```rust
pub fn scroll_linked_animation(
    scroll_node: NodeId,
    target_node: NodeId,
    property: AnimatableProperty,
    scroll_range: Range<i32>,
    value_range: Range<AnimatableValue>,
) -> AnimationId {
    // Update animation based on scroll position
}
```

### 8.2 Gesture-Driven Animations

Animations triggered by gestures:

```rust
pub fn gesture_animation(
    gesture: Gesture,
    animation: Animation,
) -> AnimationId {
    // Start animation on gesture detection
}
```

### 8.3 Physics-Based Animations

More physics models:

- **Damped harmonic oscillator:** For natural-feeling animations.
- **Gravity simulation:** For particle effects.
- **Fluid dynamics:** For liquid-like animations.

### 8.4 Animation Composition

Combining multiple animations:

- **Add:** Sum of two animations.
- **Multiply:** Product of two animations.
- **Override:** Later animation takes precedence.
- **Merge:** Blend two animations (like CSS animation composition).
