# Animations

Animation is driven by the Rust `animation` module and surfaced to React via `useAnimation()` and to the engine via the scheduler.

## Engine model

- `AnimationEngine` supports `tween(from, to, duration, easing)`, `spring(...)`, `keyframes(frames, easing)`, `update(dt)`, `cancel_all()`, `active_count()`, `is_running()`.
- `Easing`: Linear, EaseIn/Out/InOut, quad/cubic/expo/bounce/elastic, `CubicBezier`, `Steps`.
- `AnimatableValue`: `Float` | `Color`. `AnimatableProperty` picks the target property.
- The engine is decoupled from rendering; the `Scheduler` ticks it and sets dirty flags on the arena.

## React usage

```tsx
import { useAnimation } from "@bettertui/react";

function Spinner() {
  const anim = useAnimation();
  // returns controls to register/run animations against the native scheduler
  return <Spinner />;
}
```

## Native usage

`@bettertui/core`'s native bridge exposes `NapiScheduler.schedule_animation()` and the scheduler frame API (`beginFrame`, `endFrame`, `requestFrame`, `shouldRender`, `fps`, `frameBudgetMs`, `isIdle`, `frameCount`, `droppedFrames`).

> Known gap (Phase 8 review): `schedule_animation`/`cancel_animation` exist but animation callbacks were not being executed at review time. Use the lower-level scheduler frame API for now.
