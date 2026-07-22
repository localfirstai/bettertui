import { describe, expect, it } from "vitest";
import { Canvas, PixelBuffer, RESET, gradientH, parseHex, rgbBg, rgbFg } from "../graphics";

// ── parseHex ─────────────────────────────────────────────────────────────────

describe("parseHex", () => {
  it("parses #rgb shorthand", () => {
    const c = parseHex("#f80");
    expect(c.r).toBe(0xff);
    expect(c.g).toBe(0x88);
    expect(c.b).toBe(0x00);
    expect(c.a).toBe(255);
  });

  it("parses #rrggbb", () => {
    const c = parseHex("#ff8000");
    expect(c.r).toBe(255);
    expect(c.g).toBe(128);
    expect(c.b).toBe(0);
    expect(c.a).toBe(255);
  });

  it("parses #rrggbbaa", () => {
    const c = parseHex("#ff8000cc");
    expect(c.r).toBe(255);
    expect(c.g).toBe(128);
    expect(c.b).toBe(0);
    expect(c.a).toBe(0xcc);
  });

  it("returns black for unknown input", () => {
    const c = parseHex("#xyz");
    expect(c.r).toBe(0);
    expect(c.g).toBe(0);
    expect(c.b).toBe(0);
  });
});

// ── ANSI sequences ────────────────────────────────────────────────────────────

describe("rgbFg / rgbBg / RESET", () => {
  it("rgbFg contains the colour values", () => {
    const seq = rgbFg({ r: 255, g: 128, b: 0 });
    expect(seq).toContain("255;128;0");
    expect(seq).toContain("\x1b[38;2;");
  });

  it("rgbBg contains the colour values", () => {
    const seq = rgbBg({ r: 10, g: 20, b: 30 });
    expect(seq).toContain("10;20;30");
    expect(seq).toContain("\x1b[48;2;");
  });

  it("RESET is the standard ANSI reset sequence", () => {
    expect(RESET).toBe("\x1b[0m");
  });
});

// ── PixelBuffer ───────────────────────────────────────────────────────────────

describe("PixelBuffer", () => {
  it("creates a buffer with correct dimensions", () => {
    const buf = new PixelBuffer(10, 5);
    expect(buf.width).toBe(10);
    expect(buf.height).toBe(5);
  });

  it("fills with given color", () => {
    const buf = new PixelBuffer(2, 2, { r: 255, g: 0, b: 0, a: 255 });
    const px = buf.getPixel(0, 0);
    expect(px.r).toBe(255);
    expect(px.g).toBe(0);
    expect(px.b).toBe(0);
  });

  it("setPixel / getPixel roundtrip", () => {
    const buf = new PixelBuffer(10, 10);
    buf.setPixel(3, 7, { r: 1, g: 2, b: 3, a: 200 });
    const px = buf.getPixel(3, 7);
    expect(px.r).toBe(1);
    expect(px.g).toBe(2);
    expect(px.b).toBe(3);
    expect(px.a).toBe(200);
  });

  it("toRgbBuffer has correct byte count", () => {
    const buf = new PixelBuffer(4, 4);
    const rgb = buf.toRgbBuffer();
    expect(rgb.length).toBe(4 * 4 * 3);
  });

  it("toRgbaBuffer has correct byte count", () => {
    const buf = new PixelBuffer(4, 4);
    const rgba = buf.toRgbaBuffer();
    expect(rgba.length).toBe(4 * 4 * 4);
  });

  it("ignores out-of-bounds setPixel", () => {
    const buf = new PixelBuffer(4, 4);
    expect(() => buf.setPixel(-1, 0, { r: 255, g: 0, b: 0 })).not.toThrow();
    expect(() => buf.setPixel(0, 100, { r: 255, g: 0, b: 0 })).not.toThrow();
  });
});

// ── Canvas ────────────────────────────────────────────────────────────────────

describe("Canvas", () => {
  it("normalises odd height to even", () => {
    const c = new Canvas(10, 5);
    expect(c.pixelHeight).toBe(6);
  });

  it("render returns a non-empty string", () => {
    const c = new Canvas(4, 4);
    c.fill({ r: 30, g: 30, b: 30 });
    const out = c.render();
    expect(typeof out).toBe("string");
    expect(out.length).toBeGreaterThan(0);
  });

  it("setPixel writes through to buffer", () => {
    const c = new Canvas(10, 10);
    c.setPixel(2, 3, { r: 99, g: 88, b: 77 });
    const px = c.getPixel(2, 3);
    expect(px.r).toBe(99);
  });

  it("drawRect fills a rectangle", () => {
    const c = new Canvas(10, 10);
    c.drawRect(1, 1, 3, 3, { r: 255, g: 0, b: 0 });
    expect(c.getPixel(2, 2).r).toBe(255);
    expect(c.getPixel(0, 0).r).toBe(0);
  });
});

// ── gradientH ─────────────────────────────────────────────────────────────────

describe("gradientH", () => {
  it("returns the correct number of stops", () => {
    const g = gradientH({ r: 0, g: 0, b: 0 }, { r: 255, g: 255, b: 255 }, 5);
    expect(g).toHaveLength(5);
  });

  it("first stop is the from color", () => {
    const g = gradientH({ r: 255, g: 0, b: 0 }, { r: 0, g: 0, b: 255 }, 3);
    expect(g[0]).toEqual({ r: 255, g: 0, b: 0 });
  });

  it("last stop is the to color", () => {
    const g = gradientH({ r: 255, g: 0, b: 0 }, { r: 0, g: 0, b: 255 }, 3);
    expect(g[2]).toEqual({ r: 0, g: 0, b: 255 });
  });
});
