# Animations

Animation is driven by the Rust `animation` module and surfaced through the native bridge's scheduler API.

## Engine model

- `AnimationEngine` supports `tween(from, to, duration, easing)`, `spring(...)`, `keyframes(frames, easing)`
- `Easing`: Linear, EaseIn/Out/InOut, quad/cubic/expo/bounce/elastic, `CubicBezier`, `Steps`
- `AnimatableValue`: `Float` | `Color`

## Native usage

`@bettertui/core` exposes `NapiScheduler.schedule_animation()` and scheduler frame API.

> Known gap: `schedule_animation()/cancel_animation()` exist but callbacks were not executing at review time. There is no `useAnimation()` React hook.
