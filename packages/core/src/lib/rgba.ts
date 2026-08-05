/**
 * RGBA color class with static factory methods.
 */

// Named CSS terminal colors
const NAMED_COLORS: Record<string, { r: number; g: number; b: number; a: number }> = {
  black: { r: 0, g: 0, b: 0, a: 255 },
  red: { r: 205, g: 49, b: 49, a: 255 },
  green: { r: 13, g: 188, b: 121, a: 255 },
  yellow: { r: 229, g: 229, b: 16, a: 255 },
  blue: { r: 36, g: 114, b: 200, a: 255 },
  magenta: { r: 188, g: 63, b: 188, a: 255 },
  cyan: { r: 17, g: 168, b: 205, a: 255 },
  white: { r: 229, g: 229, b: 229, a: 255 },
  brightblack: { r: 102, g: 102, b: 102, a: 255 },
  brightred: { r: 241, g: 76, b: 76, a: 255 },
  brightgreen: { r: 35, g: 209, b: 139, a: 255 },
  brightyellow: { r: 245, g: 245, b: 67, a: 255 },
  brightblue: { r: 59, g: 142, b: 234, a: 255 },
  brightmagenta: { r: 214, g: 112, b: 214, a: 255 },
  brightcyan: { r: 41, g: 184, b: 219, a: 255 },
  brightwhite: { r: 229, g: 229, b: 229, a: 255 },
  transparent: { r: 0, g: 0, b: 0, a: 0 },
  orange: { r: 255, g: 165, b: 0, a: 255 },
  gray: { r: 128, g: 128, b: 128, a: 255 },
  grey: { r: 128, g: 128, b: 128, a: 255 },
  darkgray: { r: 64, g: 64, b: 64, a: 255 },
  darkgrey: { r: 64, g: 64, b: 64, a: 255 },
  lightgray: { r: 192, g: 192, b: 192, a: 255 },
  lightgrey: { r: 192, g: 192, b: 192, a: 255 },
  pink: { r: 255, g: 192, b: 203, a: 255 },
  purple: { r: 128, g: 0, b: 128, a: 255 },
  violet: { r: 238, g: 130, b: 238, a: 255 },
  brown: { r: 165, g: 42, b: 42, a: 255 },
  gold: { r: 255, g: 215, b: 0, a: 255 },
  lime: { r: 0, g: 255, b: 0, a: 255 },
  navy: { r: 0, g: 0, b: 128, a: 255 },
  teal: { r: 0, g: 128, b: 128, a: 255 },
  silver: { r: 192, g: 192, b: 192, a: 255 },
  maroon: { r: 128, g: 0, b: 0, a: 255 },
  olive: { r: 128, g: 128, b: 0, a: 255 },
  aqua: { r: 0, g: 255, b: 255, a: 255 },
  fuchsia: { r: 255, g: 0, b: 255, a: 255 },
  coral: { r: 255, g: 127, b: 80, a: 255 },
  salmon: { r: 250, g: 128, b: 114, a: 255 },
  tomato: { r: 255, g: 99, b: 71, a: 255 },
  skyblue: { r: 135, g: 206, b: 235, a: 255 },
  turquoise: { r: 64, g: 224, b: 208, a: 255 },
  indigo: { r: 75, g: 0, b: 130, a: 255 },
  crimson: { r: 220, g: 20, b: 60, a: 255 },
  limegreen: { r: 50, g: 205, b: 50, a: 255 },
  forestgreen: { r: 34, g: 139, b: 34, a: 255 },
  darkorange: { r: 255, g: 140, b: 0, a: 255 },
};

/**
 * RGBA color type with static factory methods.
 * r, g, b, a are in 0-255 range.
 */
export type RGBA = {
  r: number;
  g: number;
  b: number;
  a: number;
};

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace RGBA {
  /** Create RGBA from 0-255 integer components. */
  export function fromInts(r: number, g: number, b: number, a = 255): RGBA {
    return {
      r: Math.max(0, Math.min(255, Math.round(r))),
      g: Math.max(0, Math.min(255, Math.round(g))),
      b: Math.max(0, Math.min(255, Math.round(b))),
      a: Math.max(0, Math.min(255, Math.round(a))),
    };
  }

  /** Create RGBA from 0.0–1.0 float components. */
  export function fromValues(r: number, g: number, b: number, a = 1): RGBA {
    return fromInts(r * 255, g * 255, b * 255, a * 255);
  }

  /** Parse a CSS hex color string (#rgb, #rrggbb, #rrggbbaa). */
  export function fromHex(hex: string): RGBA {
    const h = hex.replace("#", "");
    const p = (s: string) => Number.parseInt(s, 16) || 0;
    if (h.length === 3) {
      const r = h[0] ?? "0";
      const g = h[1] ?? "0";
      const b = h[2] ?? "0";
      return fromInts(p(r + r), p(g + g), p(b + b));
    }
    if (h.length === 6) {
      return fromInts(p(h.slice(0, 2)), p(h.slice(2, 4)), p(h.slice(4, 6)));
    }
    if (h.length === 8) {
      return fromInts(p(h.slice(0, 2)), p(h.slice(2, 4)), p(h.slice(4, 6)), p(h.slice(6, 8)));
    }
    return fromInts(0, 0, 0);
  }

  /** Transparent black. */
  export const transparent: RGBA = { r: 0, g: 0, b: 0, a: 0 };

  /** Convert to a #rrggbb hex string (ignores alpha). */
  export function toHex(rgba: RGBA): string {
    const hex = (n: number) =>
      Math.max(0, Math.min(255, Math.round(n)))
        .toString(16)
        .padStart(2, "0");
    return `#${hex(rgba.r)}${hex(rgba.g)}${hex(rgba.b)}`;
  }

  /** Convert to a #rrggbbaa hex string. */
  export function toHexAlpha(rgba: RGBA): string {
    const hex = (n: number) =>
      Math.max(0, Math.min(255, Math.round(n)))
        .toString(16)
        .padStart(2, "0");
    return `#${hex(rgba.r)}${hex(rgba.g)}${hex(rgba.b)}${hex(rgba.a)}`;
  }

  /** Convert to a CSS rgba() string. */
  export function toCSS(rgba: RGBA): string {
    const a = (rgba.a / 255).toFixed(3);
    return `rgba(${rgba.r},${rgba.g},${rgba.b},${a})`;
  }

  /** Get ANSI RGB components as "r;g;b" string. */
  export function toAnsiColor(rgba: RGBA): string {
    return `${rgba.r};${rgba.g};${rgba.b}`;
  }

  /** Check if RGBA is transparent (a === 0). */
  export function isTransparent(rgba: RGBA): boolean {
    return rgba.a === 0;
  }

  /** Check equality. */
  export function equals(a: RGBA, b: RGBA): boolean {
    return a.r === b.r && a.g === b.g && a.b === b.b && a.a === b.a;
  }

  /** Blend src over dst using normal alpha compositing. */
  export function blend(dst: RGBA, src: RGBA): RGBA {
    if (src.a === 255) return src;
    if (src.a === 0) return dst;
    const alpha = src.a / 255;
    const invAlpha = 1 - alpha;
    return fromInts(
      src.r * alpha + dst.r * invAlpha,
      src.g * alpha + dst.g * invAlpha,
      src.b * alpha + dst.b * invAlpha,
      255,
    );
  }
}

/** A color input: hex string, named color string, or RGBA object. */
export type ColorInput = string | RGBA | null | undefined;

/**
 * Parse any color input to RGBA.
 * Supports: hex strings (#rgb, #rrggbb, #rrggbbaa), named colors,
 * rgb()/rgba() strings, and RGBA objects.
 */
export function parseColor(input: ColorInput): RGBA {
  if (!input) return RGBA.transparent;
  if (typeof input === "object") return input;

  const s = input.trim();
  if (!s || s === "transparent") return RGBA.transparent;

  // Named color
  const named = NAMED_COLORS[s.toLowerCase()];
  if (named) return named;

  // Hex
  if (s.startsWith("#")) return RGBA.fromHex(s);

  // rgb(r,g,b) or rgba(r,g,b,a)
  const rgbMatch = s.match(/^rgba?\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)(?:\s*,\s*([\d.]+))?\s*\)$/i);
  if (rgbMatch) {
    const r = Number.parseInt(rgbMatch[1] ?? "0", 10);
    const g = Number.parseInt(rgbMatch[2] ?? "0", 10);
    const b = Number.parseInt(rgbMatch[3] ?? "0", 10);
    const a = rgbMatch[4] !== undefined ? Math.round(Number.parseFloat(rgbMatch[4]) * 255) : 255;
    return RGBA.fromInts(r, g, b, a);
  }

  // Fallback: white
  return RGBA.fromInts(255, 255, 255);
}

/** Convert RGBA to a CSS color string suitable for the engine. */
export function rgbaToEngineColor(rgba: RGBA): string {
  if (rgba.a === 0) return "transparent";
  if (rgba.a === 255) return RGBA.toHex(rgba);
  return RGBA.toHexAlpha(rgba);
}
