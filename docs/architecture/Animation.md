# Animation

The animation engine provides tween, spring, keyframe, and color interpolation. Code: `native/engine/src/animation/` (single `mod.rs`, ~1200 lines, ~30 tests). It is decoupled from rendering and driven by the `Scheduler`.

## Types

```mermaid
classDiagram
    class AnimationEngine {
        +tween(from, to, dur, easing) Animation
        +spring(target, ...) Animation
        +keyframes(frames, easing) Animation
        +update(dt) 
        +active_count() usize
        +is_running() bool
        +cancel_all()
    }
    class Tween {
        +from: f32
        +to: f32
        +duration
        +easing
        +value_at(t) f32
        +is_complete() bool
    }
    class Spring {
        +target/stiffness/damping/mass/velocity
        +update(current, dt) (f32, bool)
    }
    class Keyframes {
        +Vec~Keyframe~ keyframes
        +value_at(t) f32
    }
    class AnimColor {
        +rgba
        +lerp(other, t)
    }
    class Timeline {
        +speed
        +loop
        +progress
    }
    AnimationEngine --> Tween
    AnimationEngine --> Spring
    AnimationEngine --> Keyframes
```

- `Easing`: Linear, EaseIn/Out/InOut, quadratic/cubic/expo/bounce/elastic, `CubicBezier(f32,f32,f32,f32)`, `Steps(u32, StepJump)`. `apply(t) -> f32`.
- `AnimationState`: Idle, Playing, Paused, Completed.
- `AnimatableValue`: `Float` | `Color`. `AnimatableProperty` enumerates which node property is animated.

## Integration

```mermaid
sequenceDiagram
    participant S as Scheduler
    participant A as AnimationEngine
    participant Arena as NodeArena
    S->>A: update(dt)
    A->>A: interpolate active animations
    A->>Arena: set animated property (sets dirty flags)
    Arena-->>S: dirty -> layout/render scheduled
```

## From TypeScript

`@bettertui/react` exposes `useAnimation()`; `@bettertui/native` exposes `NapiScheduler.schedule_animation()` through the bindings.

> Known issue (Phase 8 review): `schedule_animation()`/`cancel_animation()` exist but animation callbacks were not being executed at the time of the review — documented as a half-implemented path.
