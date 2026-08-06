import { RGBA } from "@bettertui/core";
import { lerpNumber } from "./mathUtils";

/** Parse a hex color string ("#rrggbb" or "rrggbb") into integer RGB components. */
export function hexToRgb(color: string): [number, number, number] {
  const normalized = color.startsWith("#") ? color.slice(1) : color;
  const value = Number.parseInt(normalized, 16);
  if (!Number.isFinite(value)) {
    return [255, 255, 255];
  }

  return [(value >> 16) & 0xff, (value >> 8) & 0xff, value & 0xff];
}

/** Format integer RGB components (0-255) as a "#rrggbb" hex string. */
export function rgbToHex(red: number, green: number, blue: number): string {
  const value =
    ((Math.round(red) & 0xff) << 16) |
    ((Math.round(green) & 0xff) << 8) |
    (Math.round(blue) & 0xff);
  return `#${value.toString(16).padStart(6, "0")}`;
}

/** Convert an RGBA color into a "#rrggbb" hex string (ignores alpha). */
export function rgbaToHex(color: RGBA): string {
  const r = Math.round(color.r * 255)
    .toString(16)
    .padStart(2, "0");
  const g = Math.round(color.g * 255)
    .toString(16)
    .padStart(2, "0");
  const b = Math.round(color.b * 255)
    .toString(16)
    .padStart(2, "0");
  return `#${r}${g}${b}`;
}

/** Interpolate between two hex colors by a factor clamped to [0, 1]. */
export function mixColor(left: string, right: string, amount: number): string {
  const [lr, lg, lb] = hexToRgb(left);
  const [rr, rg, rb] = hexToRgb(right);
  return rgbToHex(
    lerpNumber(lr, rr, amount),
    lerpNumber(lg, rg, amount),
    lerpNumber(lb, rb, amount),
  );
}

/** Convert HSV (h: 0-360, s/v: 0-1) into an RGBA color. */
export function hsvToRgb(h: number, s: number, v: number): RGBA {
  const c = v * s;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = v - c;
  let r = 0;
  let g = 0;
  let b = 0;
  if (h < 60) {
    r = c;
    g = x;
  } else if (h < 120) {
    r = x;
    g = c;
  } else if (h < 180) {
    g = c;
    b = x;
  } else if (h < 240) {
    g = x;
    b = c;
  } else if (h < 300) {
    r = x;
    b = c;
  } else {
    r = c;
    b = x;
  }
  return RGBA.fromValues(r + m, g + m, b + m, 1);
}

/** Serialize a color that is either a hex string or an RGBA value into a hex string. */
export function colorToString(color: string | RGBA): string {
  if (typeof color === "string") return color;
  return RGBA.toHex(color);
}

/** Compute a perceptual brightness in [0, 255] for RGB components (0-255). */
export function brightness(red: number, green: number, blue: number): number {
  return (red * 299 + green * 587 + blue * 114) / 1000;
}
