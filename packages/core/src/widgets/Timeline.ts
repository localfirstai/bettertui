import type { TimelineOptions, TweenConfig } from "@bettertui/shared";
import type { Command } from "../command/command.types";
import { type NapiTimeline, createTimeline } from "../platform/binding";
import { Renderable } from "../renderable";

/**
 * Timeline widget — wraps the native tween/spring animation engine
 * (`NativeTimeline`). Use `addTween()` to schedule scalar animations, call
 * `update(dt)` once per frame, and read `animationValue(index)` to get the
 * current interpolated value.
 *
 * The widget is invisible (renders as a zero-size box); it exists solely to
 * hold animation state and to be driven from a render/update loop.
 */
export class Timeline extends Renderable<TimelineOptions> {
  private _tl: NapiTimeline;
  private _tweens: TweenConfig[] = [];

  constructor(options: TimelineOptions = {}) {
    super(options);
    this._tl = createTimeline(options.duration, options.looping);
    if (options.autoPlay !== false) {
      this._tl.play();
    }
  }

  // ── Tween API ──────────────────────────────────────────────────────────────

  /**
   * Schedule a scalar tween from `from` → `to`.
   * Returns the animation index used by {@link animationValue}.
   */
  addTween(config: TweenConfig): number {
    this._tweens.push(config);
    return this._tl.addTween(
      config.from,
      config.to,
      config.duration,
      config.startTime ?? 0,
      config.easing,
    );
  }

  /** Advance the timeline by `dt` seconds. Call once per frame. */
  tick(dt: number): void {
    this._tl.update(dt);
    if (this._tl.isComplete()) {
      this.opts.onComplete?.();
    }
  }

  /**
   * Override Renderable.update to update timeline options.
   * To advance the animation use `tick(dt)` instead.
   */
  override update(options: Partial<import("@bettertui/shared").TimelineOptions>): void {
    super.update(options);
  }

  /** Current interpolated value of tween at `index`. */
  animationValue(index: number): number | null {
    return this._tl.animationValue(index);
  }

  play(): void {
    this._tl.play();
  }
  pause(): void {
    this._tl.pause();
  }
  restart(): void {
    this._tl.restart();
  }

  currentTime(): number {
    return this._tl.currentTime();
  }
  isComplete(): boolean {
    return this._tl.isComplete();
  }
  isPlaying(): boolean {
    return this._tl.isPlaying();
  }
  setSpeed(speed: number): void {
    this._tl.setSpeed(speed);
  }

  /** Progress 0.0–1.0 if timeline has a fixed duration, else `null`. */
  progress(): number | null {
    return this._tl.progress();
  }

  // ── Renderable ─────────────────────────────────────────────────────────────

  renderCommands(id: string): Command[] {
    // Timeline is invisible — zero-size box that holds no visual children
    return [
      { type: "CreateNode", id, kind: "Box" },
      { type: "SetWidth", id, value: 0 },
      { type: "SetHeight", id, value: 0 },
    ];
  }
}
