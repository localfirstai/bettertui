//! Tests for the animation module (easing, tweens, springs, keyframes, timelines).

use bettertui_engine::animation::{
    AnimColor, Animation, AnimationEngine, AnimationState, Easing, Keyframes, Spring, Timeline,
    Tween,
};

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
fn easing_ease_in() {
    let easing = Easing::EaseIn;
    assert!((easing.apply(0.25) - 0.0625).abs() < 0.001);
    assert!((easing.apply(0.5) - 0.25).abs() < 0.001);
}

#[test]
fn easing_ease_out() {
    let easing = Easing::EaseOut;
    assert!((easing.apply(0.25) - 0.4375).abs() < 0.001);
    assert!((easing.apply(0.5) - 0.75).abs() < 0.001);
}

#[test]
fn easing_in_circ() {
    let easing = Easing::EaseInCirc;
    assert!((easing.apply(0.0) - 0.0).abs() < 0.001);
    assert!((easing.apply(0.5) - 0.13397).abs() < 0.01);
    assert!((easing.apply(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn easing_out_circ() {
    let easing = Easing::EaseOutCirc;
    assert!((easing.apply(0.0) - 0.0).abs() < 0.001);
    assert!((easing.apply(0.5) - 0.86603).abs() < 0.01);
    assert!((easing.apply(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn easing_in_back() {
    let easing = Easing::EaseInBack;
    assert!((easing.apply(0.0) - 0.0).abs() < 0.001);
    assert!(easing.apply(0.5) < 0.0);
    assert!((easing.apply(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn easing_out_back() {
    let easing = Easing::EaseOutBack;
    assert!((easing.apply(0.0) - 0.0).abs() < 0.001);
    assert!(easing.apply(0.5) > 1.0);
    assert!((easing.apply(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn easing_cubic_bezier() {
    let easing = Easing::CubicBezier(0.0, 0.0, 1.0, 1.0);
    assert!((easing.apply(0.0) - 0.0).abs() < 0.01);
    assert!((easing.apply(0.5) - 0.5).abs() < 0.05);
    assert!((easing.apply(1.0) - 1.0).abs() < 0.01);
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
fn anim_engine_new() {
    let engine = AnimationEngine::new();
    assert_eq!(engine.active_count(), 0);
    assert!(!engine.is_running());
}

#[test]
fn anim_engine_tween() {
    let mut engine = AnimationEngine::new();
    engine.tween(0.0, 100.0, 1.0);
    assert_eq!(engine.active_count(), 1);
    assert!(engine.is_running());
}

#[test]
fn anim_engine_update() {
    let mut engine = AnimationEngine::new();
    engine.tween(0.0, 100.0, 1.0);
    engine.update(0.5);
    assert_eq!(engine.active_count(), 1);
    engine.update(0.5);
    assert_eq!(engine.active_count(), 0);
}

#[test]
fn anim_engine_cancel_all() {
    let mut engine = AnimationEngine::new();
    engine.tween(0.0, 100.0, 1.0);
    engine.tween(0.0, 50.0, 0.5);
    engine.cancel_all();
    assert_eq!(engine.active_count(), 0);
}

#[test]
fn anim_engine_spring() {
    let mut engine = AnimationEngine::new();
    engine.spring(100.0);
    assert_eq!(engine.active_count(), 1);
}

#[test]
fn anim_engine_keyframes() {
    let mut engine = AnimationEngine::new();
    let keyframes = Keyframes::new()
        .add_keyframe(0.0, 0.0)
        .add_keyframe(1.0, 100.0);
    engine.keyframes(keyframes);
    assert_eq!(engine.active_count(), 1);
}

// --- Timeline Tests ---

#[test]
fn timeline_new() {
    let timeline = Timeline::new();
    assert_eq!(timeline.current_time(), 0.0);
    assert!(!timeline.is_playing());
    assert!(!timeline.is_complete());
}

#[test]
fn timeline_play_pause() {
    let mut timeline = Timeline::new();
    timeline.play();
    assert!(timeline.is_playing());
    timeline.pause();
    assert!(!timeline.is_playing());
}

#[test]
fn timeline_update() {
    let mut timeline = Timeline::new();
    timeline.play();
    timeline.update(0.5);
    assert_eq!(timeline.current_time(), 0.5);
}

#[test]
fn timeline_speed() {
    let mut timeline = Timeline::new();
    timeline.set_speed(2.0);
    timeline.play();
    timeline.update(0.5);
    assert_eq!(timeline.current_time(), 1.0);
}

#[test]
fn timeline_add_animation() {
    let mut timeline = Timeline::new();
    let animation = Animation::from_tween(Tween::new(0.0, 100.0, 1.0), 1);
    timeline.add_animation(animation, 0.0);
    timeline.play();
    timeline.update(0.5);
    assert_eq!(timeline.current_time(), 0.5);
}

#[test]
fn timeline_sequential_animations() {
    let mut timeline = Timeline::new();
    let anim1 = Animation::from_tween(Tween::new(0.0, 50.0, 1.0), 1);
    let anim2 = Animation::from_tween(Tween::new(50.0, 100.0, 1.0), 2);
    timeline.add_animation(anim1, 0.0);
    timeline.add_animation(anim2, 1.0);
    timeline.play();

    timeline.update(0.5);
    assert_eq!(timeline.current_time(), 0.5);

    timeline.update(0.5);
    assert_eq!(timeline.current_time(), 1.0);
}

#[test]
fn timeline_with_duration() {
    let mut timeline = Timeline::new().with_duration(2.0);
    timeline.play();
    timeline.update(3.0);
    assert_eq!(timeline.current_time(), 2.0);
    assert!(!timeline.is_playing());
    assert!(timeline.is_complete());
}

#[test]
fn timeline_with_looping() {
    let mut timeline = Timeline::new().with_duration(1.0).with_looping(true);
    timeline.play();
    timeline.update(1.5);
    assert!((timeline.current_time() - 0.5).abs() < 0.001);
    assert!(timeline.is_playing());
}

#[test]
fn timeline_restart() {
    let mut timeline = Timeline::new().with_duration(1.0);
    timeline.play();
    timeline.update(0.5);
    assert_eq!(timeline.current_time(), 0.5);
    timeline.restart();
    assert_eq!(timeline.current_time(), 0.0);
    assert!(timeline.is_playing());
}

#[test]
fn timeline_on_complete() {
    use std::sync::atomic::{AtomicBool, Ordering};
    let completed = std::sync::Arc::new(AtomicBool::new(false));
    let completed_clone = completed.clone();
    let mut timeline = Timeline::new()
        .with_duration(1.0)
        .with_on_complete(move || {
            completed_clone.store(true, Ordering::SeqCst);
        });
    timeline.play();
    timeline.update(1.5);
    assert!(completed.load(Ordering::SeqCst));
}

// --- Color Interpolation Tests ---

#[test]
fn color_lerp_rgb() {
    let c1 = AnimColor::rgb(0, 0, 0);
    let c2 = AnimColor::rgb(255, 255, 255);
    let blended = c1.lerp(&c2, 0.5);
    assert_eq!(blended.r, 127);
    assert_eq!(blended.g, 127);
    assert_eq!(blended.b, 127);
}

#[test]
fn color_lerp_alpha() {
    let c1 = AnimColor::rgba(255, 0, 0, 0);
    let c2 = AnimColor::rgba(0, 0, 255, 255);
    let blended = c1.lerp(&c2, 0.5);
    assert_eq!(blended.r, 127);
    assert_eq!(blended.g, 0);
    assert_eq!(blended.b, 127);
    assert_eq!(blended.a, 127);
}

#[test]
fn color_from_hex() {
    let c = AnimColor::from_hex("#FF0000").unwrap();
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 0);
    assert_eq!(c.b, 0);
    assert_eq!(c.a, 255);
}

#[test]
fn color_to_hex() {
    let c = AnimColor::rgb(255, 128, 0);
    assert_eq!(c.to_hex(), "#FF8000");
}
