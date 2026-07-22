/**
 * Animation utilities: easing functions, Tween, Spring, and interpolation helpers.
 *
 * @example
 * ```ts
 * import { easing, Tween, Spring, lerp } from "@bettertui/core"
 *
 * const tw = new Tween({ from: 0, to: 100, duration: 1, onUpdate: v => setX(v) })
 * tw.play()
 * tw.tick(deltaSeconds)
 * ```
 */

// ── Easing functions ──────────────────────────────────────────────────────────

/**
 * Standard easing functions operating on [0, 1].
 * Each function takes a normalised time `t` (0 = start, 1 = end) and returns
 * the eased value.
 */
export const easing = {
  linear: (t: number) => t,

  // Quadratic
  easeInQuad: (t: number) => t * t,
  easeOutQuad: (t: number) => t * (2 - t),
  easeInOutQuad: (t: number) => (t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t),

  // Cubic
  easeInCubic: (t: number) => t * t * t,
  easeOutCubic: (t: number) => {
    const tt = t - 1;
    return tt * tt * tt + 1;
  },
  easeInOutCubic: (t: number) =>
    t < 0.5 ? 4 * t * t * t : (t - 1) * (2 * t - 2) * (2 * t - 2) + 1,

  // Quartic
  easeInQuart: (t: number) => t * t * t * t,
  easeOutQuart: (t: number) => {
    const tt = t - 1;
    return 1 - tt * tt * tt * tt;
  },
  easeInOutQuart: (t: number) => {
    if (t < 0.5) return 8 * t * t * t * t;
    const tt = t - 1;
    return 1 - 8 * tt * tt * tt * tt;
  },

  // Sine
  easeInSine: (t: number) => 1 - Math.cos((t * Math.PI) / 2),
  easeOutSine: (t: number) => Math.sin((t * Math.PI) / 2),
  easeInOutSine: (t: number) => -(Math.cos(Math.PI * t) - 1) / 2,

  // Exponential
  easeInExpo: (t: number) => (t === 0 ? 0 : 2 ** (10 * t - 10)),
  easeOutExpo: (t: number) => (t === 1 ? 1 : 1 - 2 ** (-10 * t)),
  easeInOutExpo: (t: number) => {
    if (t === 0) return 0;
    if (t === 1) return 1;
    return t < 0.5 ? 2 ** (20 * t - 10) / 2 : (2 - 2 ** (-20 * t + 10)) / 2;
  },

  // Circular
  easeInCirc: (t: number) => 1 - Math.sqrt(1 - t * t),
  easeOutCirc: (t: number) => Math.sqrt(1 - (t - 1) ** 2),
  easeInOutCirc: (t: number) =>
    t < 0.5 ? (1 - Math.sqrt(1 - 4 * t * t)) / 2 : (Math.sqrt(1 - (-2 * t + 2) ** 2) + 1) / 2,

  // Back (overshoot)
  easeInBack: (t: number, s = 1.70158) => t * t * ((s + 1) * t - s),
  easeOutBack: (t: number, s = 1.70158) => {
    const tt = t - 1;
    return tt * tt * ((s + 1) * tt + s) + 1;
  },

  // Elastic
  easeInElastic: (t: number) => {
    const c4 = (2 * Math.PI) / 3;
    return t === 0 ? 0 : t === 1 ? 1 : -(2 ** (10 * t - 10)) * Math.sin((t * 10 - 10.75) * c4);
  },
  easeOutElastic: (t: number) => {
    const c4 = (2 * Math.PI) / 3;
    return t === 0 ? 0 : t === 1 ? 1 : 2 ** (-10 * t) * Math.sin((t * 10 - 0.75) * c4) + 1;
  },

  // Bounce
  easeOutBounce: (t: number): number => {
    const n1 = 7.5625;
    const d1 = 2.75;
    if (t < 1 / d1) return n1 * t * t;
    if (t < 2 / d1) {
      const t2 = t - 1.5 / d1;
      return n1 * t2 * t2 + 0.75;
    }
    if (t < 2.5 / d1) {
      const t2 = t - 2.25 / d1;
      return n1 * t2 * t2 + 0.9375;
    }
    const t2 = t - 2.625 / d1;
    return n1 * t2 * t2 + 0.984375;
  },
  easeInBounce: (t: number): number => 1 - easing.easeOutBounce(1 - t),
} as const;

export type EasingName = keyof typeof easing;

// ── Interpolation helpers ─────────────────────────────────────────────────────

/** Linear interpolation between `a` and `b` by factor `t` (clamped to [0,1]). */
export function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * Math.max(0, Math.min(1, t));
}

/** Inverse lerp: returns how far `value` is between `a` and `b` (0–1). */
export function inverseLerp(a: number, b: number, value: number): number {
  if (a === b) return 0;
  return Math.max(0, Math.min(1, (value - a) / (b - a)));
}

/** Smoothly interpolate between `a` and `b` using Hermite smoothstep. */
export function smoothstep(a: number, b: number, t: number): number {
  const x = Math.max(0, Math.min(1, (t - a) / (b - a)));
  return x * x * (3 - 2 * x);
}

/** Clamp `value` to [min, max]. */
export function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

// ── Imperative tween ──────────────────────────────────────────────────────────

export interface TweenOptions {
  from: number;
  to: number;
  duration: number;
  easing?: EasingName;
  onUpdate?: (value: number) => void;
  onComplete?: () => void;
}

/**
 * A simple imperative tween driven manually with `tick(dt)`.
 * Does NOT require a Timeline — useful for one-shot or procedural animations.
 *
 * @example
 * ```ts
 * const tw = new Tween({ from: 0, to: 255, duration: 1, onUpdate: v => setAlpha(v) })
 * tw.play()
 * tw.tick(deltaSeconds)
 * ```
 */
export class Tween {
  private _time = 0;
  private _playing = false;
  private readonly _options: TweenOptions;

  constructor(options: TweenOptions) {
    this._options = { ...options };
  }

  get value(): number {
    const { from, to, duration, easing: easingName = "linear" } = this._options;
    if (duration <= 0) return to;
    const t = clamp(this._time / duration, 0, 1);
    const easeFn = easing[easingName];
    return lerp(from, to, easeFn(t));
  }

  get progress(): number {
    return clamp(this._time / (this._options.duration || 1), 0, 1);
  }

  get isComplete(): boolean {
    return this._time >= this._options.duration;
  }

  play(): this {
    this._playing = true;
    return this;
  }

  pause(): this {
    this._playing = false;
    return this;
  }

  reset(): this {
    this._time = 0;
    this._playing = false;
    return this;
  }

  tick(dt: number): void {
    if (!this._playing) return;
    this._time = Math.min(this._time + dt, this._options.duration);
    this._options.onUpdate?.(this.value);
    if (this.isComplete) {
      this._playing = false;
      this._options.onComplete?.();
    }
  }
}

// ── Spring simulation ─────────────────────────────────────────────────────────

export interface SpringOptions {
  /** Natural frequency (stiffness). Higher = faster. Default: 10. */
  frequency?: number;
  /** Damping ratio. 1.0 = critically damped. Default: 0.8. */
  damping?: number;
  /** Initial position. Default: 0. */
  initial?: number;
  /** Target position. */
  target: number;
}

/**
 * Simple critically-damped spring for smooth follow animations.
 *
 * @example
 * ```ts
 * const spring = new Spring({ target: 100, frequency: 8, damping: 0.75 })
 * spring.tick(dt)
 * const x = spring.position
 * ```
 */
export class Spring {
  private _pos: number;
  private _vel = 0;
  private _target: number;
  private readonly _frequency: number;
  private readonly _damping: number;

  constructor(options: SpringOptions) {
    this._target = options.target;
    this._pos = options.initial ?? 0;
    this._frequency = options.frequency ?? 10;
    this._damping = options.damping ?? 0.8;
  }

  get position(): number {
    return this._pos;
  }

  get velocity(): number {
    return this._vel;
  }

  set target(t: number) {
    this._target = t;
  }

  /** Advance the spring simulation by `dt` seconds. */
  tick(dt: number): void {
    const omega = 2 * Math.PI * this._frequency;
    const zeta = this._damping;
    const x0 = this._pos - this._target;
    const v0 = this._vel;

    if (Math.abs(zeta - 1) < 1e-6) {
      // Critically damped
      const e = Math.exp(-omega * dt);
      const c2 = v0 + omega * x0;
      const newX = (x0 + c2 * dt) * e;
      const newV = c2 * e + (x0 + c2 * dt) * -omega * e;
      this._pos = newX + this._target;
      this._vel = newV;
    } else if (zeta < 1) {
      // Under-damped
      const omegaD = omega * Math.sqrt(1 - zeta * zeta);
      const e = Math.exp(-zeta * omega * dt);
      const cosD = Math.cos(omegaD * dt);
      const sinD = Math.sin(omegaD * dt);
      const newX = e * (x0 * cosD + ((v0 + zeta * omega * x0) / omegaD) * sinD);
      const newV =
        e *
          ((v0 + zeta * omega * x0) * cosD -
            (x0 * omegaD + (v0 + zeta * omega * x0) * ((zeta * omega) / omegaD)) * sinD) -
        zeta * omega * newX;
      this._pos = newX + this._target;
      this._vel = newV;
    } else {
      // Over-damped
      const alpha = omega * (zeta - Math.sqrt(zeta * zeta - 1));
      const beta = omega * (zeta + Math.sqrt(zeta * zeta - 1));
      const denom = beta - alpha;
      const c1 = (v0 + beta * x0) / denom;
      const c2 = -(v0 + alpha * x0) / denom;
      this._pos = c1 * Math.exp(-alpha * dt) + c2 * Math.exp(-beta * dt) + this._target;
      this._vel = -c1 * alpha * Math.exp(-alpha * dt) - c2 * beta * Math.exp(-beta * dt);
    }
  }

  /** Instantly snap to the target. */
  snap(): void {
    this._pos = this._target;
    this._vel = 0;
  }

  /** Returns true when the spring has essentially settled. */
  isSettled(tolerance = 0.01): boolean {
    return Math.abs(this._pos - this._target) < tolerance && Math.abs(this._vel) < tolerance;
  }
}
