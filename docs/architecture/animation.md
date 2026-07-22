# Animation

The animation engine provides tween, spring, keyframe, and color interpolation. Code: `packages/core/crates/engine/src/animation.rs`.

## Types

- `Easing`: Linear, EaseIn/Out/InOut, quadratic/cubic/expo/bounce/elastic, `CubicBezier(f32,f32,f32,f32)`, `Steps(u32, StepJump)`.
- `AnimationState`: Idle, Playing, Paused, Completed.
- `AnimatableValue`: `Float` | `Color`.
- `AnimatableProperty` enumerates which node property is animated.

## Integration

The scheduler ticks the animation engine, which interpolates values and sets dirty flags on the arena, triggering layout/render.

## TypeScript surface

The native bridge exposes `NapiScheduler.schedule_animation()` and scheduler frame API (`beginFrame`, `endFrame`, `requestFrame`, `shouldRender`, `fps`, `frameBudgetMs`, `isIdle`, `frameCount`, `droppedFrames`).

> Known gap: `schedule_animation()`/`cancel_animation()` exist but animation callbacks were not executing at time of review — documented as a half-implemented path. There is no `useAnimation()` React hook.
