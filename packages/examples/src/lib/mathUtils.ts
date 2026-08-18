/** Clamp a number to the inclusive [min, max] range. */
export function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

/** Clamp a value to the inclusive [0, 1] unit range. */
export function clamp01(value: number): number {
  return clamp(value, 0, 1);
}

/** Linearly interpolate between left and right by an amount clamped to [0, 1]. */
export function lerpNumber(left: number, right: number, amount: number): number {
  return left + (right - left) * clamp01(amount);
}

/** Apply a quadratic ease-out curve to a value clamped to [0, 1]. */
export function easeOut(value: number): number {
  const t = clamp01(value);
  return 1 - (1 - t) * (1 - t);
}
