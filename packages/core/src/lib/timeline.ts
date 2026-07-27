/**
 * Timeline animation system.
 * Provides a GSAP-like API for building animation sequences.
 */

export interface TweenConfig {
  [key: string]: unknown;
}

export interface TimelineOptions {
  looping?: boolean;
  speed?: number;
  onComplete?: () => void;
}

/**
 * A simple animation timeline.
 * Tracks progress from 0 to 1 over a duration, with looping support.
 */
export class Timeline {
  private readonly _duration: number;
  private readonly _looping: boolean;
  private _position = 0;
  private _isPlaying = false;
  private _speed: number;
  private _onComplete: (() => void) | undefined;
  private _tweens: Array<{
    targets: unknown;
    props: TweenConfig;
    offset: number;
    duration: number;
  }> = [];
  private _children: Timeline[] = [];

  constructor(duration = 1, options: TimelineOptions = {}) {
    this._duration = duration;
    this._looping = options.looping ?? false;
    this._speed = options.speed ?? 1;
    this._onComplete = options.onComplete;
  }

  get position(): number {
    return this._position;
  }

  set position(v: number) {
    this._position = Math.max(0, Math.min(1, v));
  }

  get isPlaying(): boolean {
    return this._isPlaying;
  }

  get duration(): number {
    return this._duration;
  }

  get speed(): number {
    return this._speed;
  }

  set speed(v: number) {
    this._speed = v;
    for (const child of this._children) {
      child.speed = v;
    }
  }

  get looping(): boolean {
    return this._looping;
  }

  /** Start or resume playback. */
  play(): void {
    this._isPlaying = true;
    for (const child of this._children) {
      child.play();
    }
  }

  /** Pause playback. */
  pause(): void {
    this._isPlaying = false;
    for (const child of this._children) {
      child.pause();
    }
  }

  /** Reset to beginning and stop. */
  stop(): void {
    this._isPlaying = false;
    this._position = 0;
    for (const child of this._children) {
      child.stop();
    }
  }

  /** Reset to beginning and play. */
  restart(): void {
    this._position = 0;
    this._isPlaying = true;
    for (const child of this._children) {
      child.restart();
    }
  }

  /** Toggle play/pause. */
  toggle(): void {
    if (this._isPlaying) {
      this.pause();
    } else {
      this.play();
    }
  }

  /**
   * Add a tween to the timeline (GSAP-like API).
   * @param targets - The target objects to animate
   * @param props - The properties to tween and their target values
   * @param offset - Time offset in seconds (or "+=N" for relative)
   */
  add(targets: unknown, props: TweenConfig, offset?: number): this {
    this._tweens.push({
      targets,
      props,
      offset: offset ?? 0,
      duration: (props.duration as number) ?? this._duration,
    });
    return this;
  }

  /** Add a child timeline. */
  addChild(child: Timeline): this {
    this._children.push(child);
    return this;
  }

  /**
   * Update the timeline by deltaTime milliseconds.
   * Returns whether the timeline is still active.
   */
  update(deltaTimeMs: number): boolean {
    if (!this._isPlaying) return this._isPlaying;

    const deltaProgress = ((deltaTimeMs / 1000) * this._speed) / this._duration;
    this._position += deltaProgress;

    if (this._position >= 1) {
      if (this._looping) {
        this._position %= 1;
      } else {
        this._position = 1;
        this._isPlaying = false;
        this._onComplete?.();
      }
    }

    for (const child of this._children) {
      child.update(deltaTimeMs);
    }

    return this._isPlaying;
  }

  /**
   * Get the current value of a property at the current position.
   * For simple linear interpolation between 0 and target value.
   */
  getValue<T = number>(prop: string): T {
    // Find tween for this prop
    for (const tween of this._tweens) {
      if (typeof tween.props === "object" && prop in (tween.props as object)) {
        const target = (tween.props as Record<string, unknown>)[prop] as number;
        const from = 0;
        const progress = Math.max(0, Math.min(1, this._position));
        return (from + (target - from) * progress) as T;
      }
    }
    return 0 as T;
  }

  /** Seek to a specific position (0-1). */
  seek(position: number): void {
    this._position = Math.max(0, Math.min(1, position));
  }

  /** Get the time in seconds at the current position. */
  get currentTime(): number {
    return this._position * this._duration;
  }
}

/** Create a new Timeline instance. */
export function createTimeline(duration?: number, options?: TimelineOptions): Timeline {
  return new Timeline(duration ?? 1, options ?? {});
}
