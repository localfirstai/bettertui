/**
 * Terminal graphics utilities: pixel buffer, canvas, ANSI color helpers,
 * and gradient generation.
 *
 * @example
 * ```ts
 * import { Canvas, parseHex } from "@bettertui/core"
 *
 * const canvas = new Canvas(40, 20)
 * canvas.fill(parseHex("#1a1a2e"))
 * canvas.drawRect(5, 2, 10, 6, { r: 255, g: 64, b: 0 })
 * process.stdout.write(canvas.render())
 * ```
 */

// ── Color types ───────────────────────────────────────────────────────────────

export interface RGB {
  r: number;
  g: number;
  b: number;
}

export interface RGBA extends RGB {
  a: number;
}

/** Parse a CSS hex color string (`#rgb`, `#rrggbb`, `#rrggbbaa`) to RGBA. */
export function parseHex(hex: string): RGBA {
  const h = hex.replace("#", "");
  const p = (s: string) => Number.parseInt(s, 16) || 0;
  if (h.length === 3) {
    const r = h[0] ?? "0";
    const g = h[1] ?? "0";
    const b = h[2] ?? "0";
    return { r: p(r + r), g: p(g + g), b: p(b + b), a: 255 };
  }
  if (h.length === 6) {
    return { r: p(h.slice(0, 2)), g: p(h.slice(2, 4)), b: p(h.slice(4, 6)), a: 255 };
  }
  if (h.length === 8) {
    return {
      r: p(h.slice(0, 2)),
      g: p(h.slice(2, 4)),
      b: p(h.slice(4, 6)),
      a: p(h.slice(6, 8)),
    };
  }
  return { r: 0, g: 0, b: 0, a: 255 };
}

/** Convert RGB to a 24-bit ANSI truecolor foreground escape sequence. */
export function rgbFg(color: RGB): string {
  return `\x1b[38;2;${color.r};${color.g};${color.b}m`;
}

/** Convert RGB to a 24-bit ANSI truecolor background escape sequence. */
export function rgbBg(color: RGB): string {
  return `\x1b[48;2;${color.r};${color.g};${color.b}m`;
}

/** ANSI reset sequence. */
export const RESET = "\x1b[0m";

// ── PixelBuffer ───────────────────────────────────────────────────────────────

/** A mutable RGBA pixel buffer. Each pixel is 4 bytes: R, G, B, A. */
export class PixelBuffer {
  readonly width: number;
  readonly height: number;
  readonly data: Uint8ClampedArray;

  constructor(width: number, height: number, fill?: RGBA) {
    this.width = width;
    this.height = height;
    this.data = new Uint8ClampedArray(width * height * 4);
    if (fill) this.fill(fill);
  }

  private _offset(x: number, y: number): number {
    return (y * this.width + x) * 4;
  }

  getPixel(x: number, y: number): RGBA {
    const o = this._offset(x, y);
    return {
      r: this.data[o] ?? 0,
      g: this.data[o + 1] ?? 0,
      b: this.data[o + 2] ?? 0,
      a: this.data[o + 3] ?? 255,
    };
  }

  setPixel(x: number, y: number, color: RGB | RGBA): void {
    if (x < 0 || x >= this.width || y < 0 || y >= this.height) return;
    const o = this._offset(x, y);
    this.data[o] = color.r;
    this.data[o + 1] = color.g;
    this.data[o + 2] = color.b;
    this.data[o + 3] = "a" in color ? color.a : 255;
  }

  fill(color: RGB | RGBA): void {
    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        this.setPixel(x, y, color);
      }
    }
  }

  /** Convert this pixel buffer to a Node.js Buffer containing raw RGB bytes. */
  toRgbBuffer(): Buffer {
    const out = Buffer.allocUnsafe(this.width * this.height * 3);
    let oi = 0;
    for (let i = 0; i < this.data.length; i += 4) {
      out[oi++] = this.data[i] ?? 0;
      out[oi++] = this.data[i + 1] ?? 0;
      out[oi++] = this.data[i + 2] ?? 0;
    }
    return out;
  }

  /** Convert this pixel buffer to a Node.js Buffer containing raw RGBA bytes. */
  toRgbaBuffer(): Buffer {
    return Buffer.from(this.data.buffer);
  }
}

// ── Canvas ────────────────────────────────────────────────────────────────────

/**
 * A terminal "canvas" that renders RGBA pixels as Unicode half-block
 * characters (`▀`, `▄`), achieving 1×2 sub-cell pixel resolution.
 * Each character cell covers one column × 2 rows of pixels.
 *
 * @example
 * ```ts
 * const canvas = new Canvas(40, 20)
 * canvas.fill({ r: 0, g: 0, b: 0 })
 * canvas.setPixel(10, 5, { r: 255, g: 64, b: 0 })
 * process.stdout.write(canvas.render())
 * ```
 */
export class Canvas {
  readonly pixelWidth: number;
  readonly pixelHeight: number;
  private _buffer: PixelBuffer;

  /**
   * @param pixelWidth  Width in pixels (each char column = 1 pixel wide).
   * @param pixelHeight Height in pixels (each char row = 2 pixels tall).
   */
  constructor(pixelWidth: number, pixelHeight: number) {
    this.pixelWidth = pixelWidth;
    // Ensure even height for ▀ / ▄ encoding
    this.pixelHeight = pixelHeight % 2 === 0 ? pixelHeight : pixelHeight + 1;
    this._buffer = new PixelBuffer(this.pixelWidth, this.pixelHeight);
  }

  get buffer(): PixelBuffer {
    return this._buffer;
  }

  setPixel(x: number, y: number, color: RGB | RGBA): void {
    this._buffer.setPixel(x, y, color);
  }

  getPixel(x: number, y: number): RGBA {
    return this._buffer.getPixel(x, y);
  }

  fill(color: RGB | RGBA): void {
    this._buffer.fill(color);
  }

  /**
   * Render the canvas to a string using upper-half block `▀` characters.
   * Each character encodes two pixel rows: foreground = top pixel, background = bottom pixel.
   */
  render(): string {
    let out = "";
    for (let y = 0; y < this.pixelHeight; y += 2) {
      for (let x = 0; x < this.pixelWidth; x++) {
        const top = this._buffer.getPixel(x, y);
        const bottom = this._buffer.getPixel(x, y + 1);
        out += `${rgbFg(top)}${rgbBg(bottom)}▀`;
      }
      out += `${RESET}\n`;
    }
    return out;
  }

  /** Draw a filled rectangle. */
  drawRect(x: number, y: number, w: number, h: number, color: RGB | RGBA): void {
    for (let dy = 0; dy < h; dy++) {
      for (let dx = 0; dx < w; dx++) {
        this._buffer.setPixel(x + dx, y + dy, color);
      }
    }
  }

  /** Draw a 1-pixel-wide rectangle outline. */
  drawRectOutline(x: number, y: number, w: number, h: number, color: RGB | RGBA): void {
    for (let dx = 0; dx < w; dx++) {
      this._buffer.setPixel(x + dx, y, color);
      this._buffer.setPixel(x + dx, y + h - 1, color);
    }
    for (let dy = 1; dy < h - 1; dy++) {
      this._buffer.setPixel(x, y + dy, color);
      this._buffer.setPixel(x + w - 1, y + dy, color);
    }
  }

  /** Draw a line using Bresenham's algorithm. */
  drawLine(x0: number, y0: number, x1: number, y1: number, color: RGB | RGBA): void {
    const dx = Math.abs(x1 - x0);
    const dy = Math.abs(y1 - y0);
    const sx = x0 < x1 ? 1 : -1;
    const sy = y0 < y1 ? 1 : -1;
    let err = dx - dy;
    let cx = x0;
    let cy = y0;

    while (true) {
      this._buffer.setPixel(cx, cy, color);
      if (cx === x1 && cy === y1) break;
      const e2 = 2 * err;
      if (e2 > -dy) {
        err -= dy;
        cx += sx;
      }
      if (e2 < dx) {
        err += dx;
        cy += sy;
      }
    }
  }

  /** Draw a filled or outline circle using midpoint circle algorithm. */
  drawCircle(cx: number, cy: number, radius: number, color: RGB | RGBA, filled = true): void {
    const r2 = radius * radius;
    for (let y = -radius; y <= radius; y++) {
      for (let x = -radius; x <= radius; x++) {
        const dist2 = x * x + y * y;
        if (filled ? dist2 <= r2 : Math.abs(dist2 - r2) <= radius) {
          this._buffer.setPixel(cx + x, cy + y, color);
        }
      }
    }
  }

  /** Build a PixelBuffer image suitable for passing to the Image widget. */
  toPixelBuffer(): PixelBuffer {
    return this._buffer;
  }
}

// ── Gradient helpers ──────────────────────────────────────────────────────────

/** Generate a horizontal gradient between two RGB colors across `steps` stops. */
export function gradientH(from: RGB, to: RGB, steps: number): RGB[] {
  return Array.from({ length: steps }, (_, i) => {
    const t = steps <= 1 ? 0 : i / (steps - 1);
    return {
      r: Math.round(from.r + (to.r - from.r) * t),
      g: Math.round(from.g + (to.g - from.g) * t),
      b: Math.round(from.b + (to.b - from.b) * t),
    };
  });
}
