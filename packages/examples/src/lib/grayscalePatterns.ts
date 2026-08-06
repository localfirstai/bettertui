import { type FrameBufferLike, RGBA } from "@bettertui/core";

/** Names of the grayscale demo patterns, indexed by pattern id. */
export const PATTERN_NAMES = ["Plasma", "Ripples", "Waves", "Starburst", "Dots", "Checkers"];

/** Plasma noise intensity in [0, 1] at a pixel. */
function generatePlasma(x: number, y: number, w: number, h: number, t: number): number {
  const nx = x / w;
  const ny = y / h;
  const v1 = Math.sin(nx * 10 + t);
  const v2 = Math.sin(ny * 10 + t * 0.7);
  const v3 = Math.sin((nx + ny) * 8 + t * 1.3);
  const v4 = Math.sin(Math.sqrt((nx - 0.5) ** 2 + (ny - 0.5) ** 2) * 12 - t * 2);
  return (v1 + v2 + v3 + v4 + 4) / 8;
}

/** Concentric ripple intensity [0,1] at a pixel. */
function generateRipples(x: number, y: number, w: number, h: number, t: number): number {
  const cx = w / 2;
  const cy = h / 2;
  const dist = Math.sqrt((x - cx) ** 2 + (y - cy) ** 2);
  const wave = Math.sin(dist * 0.5 - t * 3) * 0.5 + 0.5;
  const fade = 1 - Math.min(dist / Math.max(w, h), 1);
  return wave * fade;
}

/** Diagonal and cross waves intensity [0,1] at a pixel. */
function generateWaves(x: number, y: number, w: number, h: number, t: number): number {
  const nx = x / w;
  const ny = y / h;
  const diagonal = (nx + ny) * 6 - t * 2;
  const cross = Math.sin(nx * 8 + t) * Math.sin(ny * 8 + t * 0.8);
  return (Math.sin(diagonal) * 0.5 + 0.5) * 0.6 + (cross * 0.5 + 0.5) * 0.4;
}

/** Rotating starburst rays intensity [0,1] at a pixel. */
function generateStarburst(x: number, y: number, w: number, h: number, t: number): number {
  const cx = w / 2;
  const cy = h / 2;
  const dx = x - cx;
  const dy = y - cy;
  const angle = Math.atan2(dy, dx) + t * 0.5;
  const numRays = 12;
  const rayAngle = (angle * numRays) / (2 * Math.PI);
  const rayIntensity = Math.abs(Math.sin(rayAngle * Math.PI));
  return rayIntensity > 0.7 ? 1.0 : 0.0;
}

/** Drifting dot grid intensity [0,1] at a pixel. */
function generateDots(x: number, y: number, w: number, h: number, t: number): number {
  const gridSize = Math.min(w, h) / 6;
  const offsetX = t * 3;
  const offsetY = t * 2;
  const gx = ((((x + offsetX) % gridSize) + gridSize) % gridSize) - gridSize / 2;
  const gy = ((((y + offsetY) % gridSize) + gridSize) % gridSize) - gridSize / 2;
  const dist = Math.sqrt(gx * gx + gy * gy);
  const radius = gridSize * 0.35;
  return dist < radius ? 1.0 : 0.0;
}

/** Rotating checkerboard intensity [0,1] at a pixel. */
function generateCheckers(x: number, y: number, w: number, h: number, t: number): number {
  const cx = w / 2;
  const cy = h / 2;
  const dx = x - cx;
  const dy = y - cy;
  const cos = Math.cos(t * 0.3);
  const sin = Math.sin(t * 0.3);
  const rx = dx * cos - dy * sin;
  const ry = dx * sin + dy * cos;
  const size = Math.min(w, h) / 8;
  const checkX = Math.floor(rx / size);
  const checkY = Math.floor(ry / size);
  return (checkX + checkY) % 2 === 0 ? 1.0 : 0.0;
}

/** Compute intensity [0,1] for a pattern id at a pixel. */
export function getIntensity(
  patternIndex: number,
  x: number,
  y: number,
  w: number,
  h: number,
  t: number,
): number {
  switch (patternIndex) {
    case 0:
      return generatePlasma(x, y, w, h, t);
    case 1:
      return generateRipples(x, y, w, h, t);
    case 2:
      return generateWaves(x, y, w, h, t);
    case 3:
      return generateStarburst(x, y, w, h, t);
    case 4:
      return generateDots(x, y, w, h, t);
    case 5:
      return generateCheckers(x, y, w, h, t);
    default:
      return generatePlasma(x, y, w, h, t);
  }
}

/** Render a Float32Array of intensity values [0,1] as grayscale cells. */
export function drawGrayscaleBuffer(
  fb: FrameBufferLike,
  x: number,
  y: number,
  buffer: Float32Array,
  width: number,
  height: number,
): void {
  for (let gy = 0; gy < height; gy++) {
    for (let gx = 0; gx < width; gx++) {
      const intensity = buffer[gy * width + gx] ?? 0;
      const v = Math.floor(Math.max(0, Math.min(1, intensity)) * 255);
      fb.setCell(x + gx, y + gy, " ", undefined, RGBA.fromInts(v, v, v, 255));
    }
  }
}

/** Render a Float32Array with 2x2 supersampling, averaging input pixels into output cells. */
export function drawGrayscaleBufferSupersampled(
  fb: FrameBufferLike,
  x: number,
  y: number,
  buffer: Float32Array,
  width: number,
  height: number,
): void {
  const outW = Math.floor(width / 2);
  const outH = Math.floor(height / 2);
  for (let gy = 0; gy < outH; gy++) {
    for (let gx = 0; gx < outW; gx++) {
      const i00 = buffer[gy * 2 * width + gx * 2] ?? 0;
      const i01 = buffer[gy * 2 * width + (gx * 2 + 1)] ?? 0;
      const i10 = buffer[(gy * 2 + 1) * width + gx * 2] ?? 0;
      const i11 = buffer[(gy * 2 + 1) * width + (gx * 2 + 1)] ?? 0;
      const avg = (i00 + i01 + i10 + i11) / 4;
      const v = Math.floor(Math.max(0, Math.min(1, avg)) * 255);
      fb.setCell(x + gx, y + gy, " ", undefined, RGBA.fromInts(v, v, v, 255));
    }
  }
}
