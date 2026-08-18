#!/usr/bin/env bun

/**
 * Split Footer Image Demo
 *
 * Renders actual PNG images inside a split-footer layout using FrameBuffer
 * and Unicode half-block characters (▀) for 1×2 sub-cell pixel resolution.
 *
 * PNG files are decoded via Node.js built-in `zlib` — IDAT chunks are inflated
 * and the five PNG filter types (None/Sub/Up/Average/Paeth) are reconstructed
 * to recover the original RGBA pixel data. JPEG assets show a gradient fallback.
 */

import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { inflateSync } from "node:zlib";
import {
  Box,
  type CliRenderer,
  FrameBuffer,
  type FrameBufferLike,
  RGBA,
  Screen,
  Text,
  bold,
  createCliRenderer,
  dim,
  fg,
  t,
} from "@bettertui/core";
import type { KeyEvent } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

// ── Types ─────────────────────────────────────────────────────────────────────

interface RgbaImage {
  width: number;
  height: number;
  pixels: Uint8Array;
}

// ── Constants ─────────────────────────────────────────────────────────────────

const PALETTE = {
  background: "#0B1220",
  panel: "#101A2D",
  imagePanel: "#0D1830",
  border: "#3B5B82",
  borderHighlight: "#56B4D3",
  title: "#F4F8FF",
  text: "#D7E5FA",
  detail: "#A8C0E4",
  hint: "#8BA6CD",
  accent: "#66D9EF",
} as const;

const CURRENT_FILE_DIR = dirname(fileURLToPath(import.meta.url));
const HEADER_HEIGHT = 3;
const FOOTER_HEIGHT = 3;
const IMAGE_PANEL_WIDTH = 44;

const ASSET_NAMES = [
  "heart.png",
  "concrete.png",
  "crate.png",
  "crate_emissive.png",
  "forrest_background.png",
  "main_char_idle.png",
  "main_char_run_loop.png",
  "main_char_heavy_attack.png",
  "main_char_jump_start.png",
];

// ── Module-level state ────────────────────────────────────────────────────────

let globalScreen: Screen | null = null;
let globalImageContainer: Box | null = null;
let globalImagePanel: FrameBuffer | null = null;
let globalInfoText: Text | null = null;
let globalStatusText: Text | null = null;
let globalImage: RgbaImage | null = null;
let imageIndex = 0;
let imageDirty = false;

let keyHandler: ((key: KeyEvent) => void) | null = null;
let resizeHandler: (() => void) | null = null;

// ── PNG Decoder ───────────────────────────────────────────────────────────────

/**
 * Decodes an 8-bit PNG (grayscale, RGB, grayscale+alpha, or RGBA) into raw RGBA pixels.
 *
 * Reads the IHDR for dimensions and color type, concatenates all IDAT chunks,
 * decompresses the zlib stream via `inflateSync`, then reconstructs every row
 * by applying the declared filter (0=None, 1=Sub, 2=Up, 3=Average, 4=Paeth).
 * Returns `null` for indexed-color (type 3), 16-bit-depth, or malformed files.
 */
function decodePng(data: Buffer): RgbaImage | null {
  if (data.length < 8) return null;
  if (data.readUInt32BE(0) !== 0x89504e47 || data.readUInt32BE(4) !== 0x0d0a1a0a) return null;

  let pos = 8;
  let width = 0;
  let height = 0;
  let bitDepth = 0;
  let colorType = -1;
  const idatChunks: Buffer[] = [];

  while (pos + 12 <= data.length) {
    const chunkLen = data.readUInt32BE(pos);
    const type = data.slice(pos + 4, pos + 8).toString("ascii");
    const chunk = data.slice(pos + 8, pos + 8 + chunkLen);

    if (type === "IHDR") {
      width = chunk.readUInt32BE(0);
      height = chunk.readUInt32BE(4);
      bitDepth = chunk[8] ?? 0;
      colorType = chunk[9] ?? -1;
    } else if (type === "IDAT") {
      idatChunks.push(chunk);
    } else if (type === "IEND") {
      break;
    }

    pos += 12 + chunkLen;
  }

  if (width === 0 || height === 0 || idatChunks.length === 0) return null;
  if (bitDepth !== 8 || colorType === 3) return null;

  let raw: Buffer;
  try {
    raw = inflateSync(Buffer.concat(idatChunks));
  } catch {
    return null;
  }

  const channels = colorType === 6 ? 4 : colorType === 4 ? 2 : colorType === 0 ? 1 : 3;
  const stride = width * channels;
  const pixels = new Uint8Array(width * height * 4);
  let rawPos = 0;
  const prev = new Uint8Array(stride);

  for (let y = 0; y < height; y++) {
    const filter = raw[rawPos++] ?? 0;
    const row = new Uint8Array(stride);
    for (let i = 0; i < stride; i++) row[i] = raw[rawPos++] ?? 0;

    switch (filter) {
      case 1:
        for (let i = channels; i < stride; i++)
          row[i] = ((row[i] ?? 0) + (row[i - channels] ?? 0)) & 0xff;
        break;
      case 2:
        for (let i = 0; i < stride; i++) row[i] = ((row[i] ?? 0) + (prev[i] ?? 0)) & 0xff;
        break;
      case 3:
        for (let i = 0; i < stride; i++) {
          const a = i >= channels ? (row[i - channels] ?? 0) : 0;
          row[i] = ((row[i] ?? 0) + Math.floor((a + (prev[i] ?? 0)) / 2)) & 0xff;
        }
        break;
      case 4:
        for (let i = 0; i < stride; i++) {
          const a = i >= channels ? (row[i - channels] ?? 0) : 0;
          const b = prev[i] ?? 0;
          const c = i >= channels ? (prev[i - channels] ?? 0) : 0;
          const p = a + b - c;
          const pa = Math.abs(p - a);
          const pb = Math.abs(p - b);
          const pc = Math.abs(p - c);
          row[i] = ((row[i] ?? 0) + (pa <= pb && pa <= pc ? a : pb <= pc ? b : c)) & 0xff;
        }
        break;
    }

    for (let x = 0; x < width; x++) {
      const dst = (y * width + x) * 4;
      const src = x * channels;
      if (colorType === 0 || colorType === 4) {
        const gray = row[src] ?? 0;
        pixels[dst] = gray;
        pixels[dst + 1] = gray;
        pixels[dst + 2] = gray;
        pixels[dst + 3] = colorType === 4 ? (row[src + 1] ?? 255) : 255;
      } else {
        pixels[dst] = row[src] ?? 0;
        pixels[dst + 1] = row[src + 1] ?? 0;
        pixels[dst + 2] = row[src + 2] ?? 0;
        pixels[dst + 3] = colorType === 6 ? (row[src + 3] ?? 255) : 255;
      }
    }

    prev.set(row);
  }

  return { width, height, pixels };
}

// ── Image utilities ───────────────────────────────────────────────────────────

/** Downsamples an image to a maximum dimension while preserving aspect ratio (nearest-neighbour). */
function downsample(img: RgbaImage, maxDim: number): RgbaImage {
  const { width: srcW, height: srcH, pixels: src } = img;
  if (srcW <= maxDim && srcH <= maxDim) return img;

  const scale = maxDim / Math.max(srcW, srcH);
  const dstW = Math.max(1, Math.floor(srcW * scale));
  const dstH = Math.max(1, Math.floor(srcH * scale));
  const dst = new Uint8Array(dstW * dstH * 4);

  for (let y = 0; y < dstH; y++) {
    for (let x = 0; x < dstW; x++) {
      const sx = Math.min(srcW - 1, Math.floor((x / dstW) * srcW));
      const sy = Math.min(srcH - 1, Math.floor((y / dstH) * srcH));
      const si = (sy * srcW + sx) * 4;
      const di = (y * dstW + x) * 4;
      dst[di] = src[si] ?? 0;
      dst[di + 1] = src[si + 1] ?? 0;
      dst[di + 2] = src[si + 2] ?? 0;
      dst[di + 3] = src[si + 3] ?? 255;
    }
  }

  return { width: dstW, height: dstH, pixels: dst };
}

/** Generates an HSL gradient test pattern used as a fallback when no image can be decoded. */
function testPattern(width: number, height: number): RgbaImage {
  const pixels = new Uint8Array(width * height * 4);
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const i = (y * width + x) * 4;
      const nx = x / width;
      const ny = y / height;
      pixels[i] = Math.round(255 * (0.5 + 0.5 * Math.sin(nx * Math.PI * 2)));
      pixels[i + 1] = Math.round(255 * ny);
      pixels[i + 2] = Math.round(255 * (0.5 + 0.5 * Math.cos((nx + ny) * Math.PI)));
      pixels[i + 3] = 255;
    }
  }
  return { width, height, pixels };
}

// ── Rendering ─────────────────────────────────────────────────────────────────

/**
 * Draws an RgbaImage into a FrameBuffer using Unicode half-block characters.
 *
 * The `▀` glyph (U+2580) occupies one terminal cell and carries two pixel rows:
 * the foreground color maps to the upper half-block and the background color maps
 * to the lower half-block, yielding 1×2 pixel resolution per cell.
 *
 * The image is scaled to fill the buffer while preserving its aspect ratio and
 * centered in any remaining space.
 */
function renderImageToBuffer(img: RgbaImage, buf: FrameBufferLike): void {
  const { width: imgW, height: imgH, pixels } = img;
  const { width: bufW, height: bufH } = buf;

  const scaleW = bufW / imgW;
  const scaleH = bufH / (imgH / 2);
  const scale = Math.min(scaleW, scaleH);

  const displayW = Math.max(1, Math.floor(imgW * scale));
  const displayH = Math.max(1, Math.floor((imgH / 2) * scale));

  const startX = Math.floor((bufW - displayW) / 2);
  const startY = Math.floor((bufH - displayH) / 2);

  for (let by = 0; by < displayH; by++) {
    for (let bx = 0; bx < displayW; bx++) {
      const srcX = Math.min(imgW - 1, Math.floor((bx / displayW) * imgW));
      const srcY1 = Math.min(imgH - 1, Math.floor((by / displayH) * imgH));
      const srcY2 = Math.min(srcY1 + 1, imgH - 1);

      const i1 = (srcY1 * imgW + srcX) * 4;
      const i2 = (srcY2 * imgW + srcX) * 4;

      const top = RGBA.fromInts(
        pixels[i1] ?? 0,
        pixels[i1 + 1] ?? 0,
        pixels[i1 + 2] ?? 0,
        pixels[i1 + 3] ?? 255,
      );
      const bot = RGBA.fromInts(
        pixels[i2] ?? 0,
        pixels[i2 + 1] ?? 0,
        pixels[i2 + 2] ?? 0,
        pixels[i2 + 3] ?? 255,
      );

      buf.setCell(startX + bx, startY + by, "▀", top, bot);
    }
  }
}

// ── Image loading ─────────────────────────────────────────────────────────────

function loadAsset(name: string): RgbaImage | null {
  const assetPath = join(CURRENT_FILE_DIR, "..", "assets", name);
  if (!existsSync(assetPath)) return null;
  try {
    const data = readFileSync(assetPath);
    if (!name.toLowerCase().endsWith(".png")) return null;
    const img = decodePng(data);
    return img ? downsample(img, 320) : null;
  } catch {
    return null;
  }
}

function findFirstAvailableImage(): { img: RgbaImage; name: string; idx: number } | null {
  for (let i = 0; i < ASSET_NAMES.length; i++) {
    const name = ASSET_NAMES[i];
    if (!name) continue;
    const img = loadAsset(name);
    if (img) return { img, name, idx: i };
  }
  return null;
}

// ── Panel helpers ─────────────────────────────────────────────────────────────

function panelDims(renderer: CliRenderer): { w: number; h: number } {
  const w = Math.max(4, IMAGE_PANEL_WIDTH - 4);
  const h = Math.max(4, renderer.terminalHeight - HEADER_HEIGHT - FOOTER_HEIGHT - 4);
  return { w, h };
}

function buildImagePanel(renderer: CliRenderer): void {
  if (!globalImageContainer) return;

  if (globalImagePanel) {
    globalImageContainer.remove(globalImagePanel);
    globalImagePanel.destroy();
    globalImagePanel = null;
  }

  const { w, h } = panelDims(renderer);

  globalImagePanel = new FrameBuffer(renderer, {
    id: "sfi-framebuffer",
    width: w,
    height: h,
    drawFn: (buf: FrameBufferLike) => {
      buf.fillRect(0, 0, buf.width, buf.height, RGBA.fromHex(PALETTE.imagePanel));
      if (globalImage) renderImageToBuffer(globalImage, buf);
    },
  });

  globalImageContainer.add(globalImagePanel);
  imageDirty = true;
}

function updateLabels(name: string, img: RgbaImage | null): void {
  if (globalInfoText) {
    globalInfoText.content = img
      ? t`${fg(PALETTE.accent)(name)}  ${fg(PALETTE.detail)(`${img.width}×${img.height}`)}`
      : t`${fg(PALETTE.hint)("test pattern")}`;
  }
  if (globalStatusText) {
    globalStatusText.content = t`${fg(PALETTE.hint)(`${imageIndex + 1} / ${ASSET_NAMES.length}`)}  ${dim("N=next  P=prev  R=reload")}`;
  }
}

function setImage(img: RgbaImage | null, name: string): void {
  globalImage = img ?? testPattern(160, 80);
  updateLabels(name, img);
  imageDirty = true;
}

function cycleImage(delta: number): void {
  imageIndex =
    (((imageIndex + delta) % ASSET_NAMES.length) + ASSET_NAMES.length) % ASSET_NAMES.length;
  const name = ASSET_NAMES[imageIndex] ?? "unknown";
  setImage(loadAsset(name), name);
}

// ── run / destroy ─────────────────────────────────────────────────────────────

export function run(renderer: CliRenderer): void {
  renderer.start();
  renderer.setBackgroundColor(PALETTE.background);

  globalScreen = new Screen(renderer, {
    id: "sfi-screen",
    backgroundColor: PALETTE.background,
    header: {
      id: "sfi-header",
      height: HEADER_HEIGHT,
      backgroundColor: PALETTE.panel,
      border: true,
      borderStyle: "single",
      borderColor: PALETTE.borderHighlight,
      title: " Split Footer — Image Demo ",
      titleAlignment: "center",
      alignItems: "center",
    },
    body: {
      id: "sfi-body",
      flexDirection: "row",
      padding: 0,
    },
    footer: {
      id: "sfi-footer",
      height: FOOTER_HEIGHT,
      backgroundColor: PALETTE.panel,
      border: true,
      borderStyle: "single",
      borderColor: PALETTE.border,
      alignItems: "center",
      justifyContent: "center",
    },
  });

  const footerText = new Text(renderer, {
    id: "sfi-footer-text",
    content: t`${dim("N / →")} next image  ${dim("P / ←")} prev  ${dim("R")} reload  ${dim("Ctrl+C")} exit`,
    fg: PALETTE.hint,
    zIndex: 1,
  });
  globalScreen.footer?.add(footerText);

  const contentPanel = new Box(renderer, {
    id: "sfi-content-panel",
    flexGrow: 1,
    flexShrink: 1,
    flexDirection: "column",
    padding: 2,
    gap: 1,
    overflow: "hidden",
  });
  globalScreen.body.add(contentPanel);

  const descText = new Text(renderer, {
    id: "sfi-desc",
    content: t`${bold("PNG Decoding via Node.js zlib")}\n\nEach PNG is decoded by inflating its IDAT chunks with\n${fg(PALETTE.accent)("inflateSync")} then applying PNG filter types 0–4 to\nreconstruct the original pixel data.\n\nThe ${fg(PALETTE.accent)("▀")} half-block character (U+2580) maps two rows\nof pixels into one terminal cell:\n  · ${fg(PALETTE.detail)("foreground")} → upper pixel\n  · ${fg(PALETTE.detail)("background")} → lower pixel`,
    fg: PALETTE.detail,
    wrapMode: "word",
    flexGrow: 1,
    flexShrink: 1,
  });
  contentPanel.add(descText);

  globalInfoText = new Text(renderer, {
    id: "sfi-info",
    content: "",
    fg: PALETTE.text,
    flexGrow: 0,
    flexShrink: 0,
  });
  contentPanel.add(globalInfoText);

  globalStatusText = new Text(renderer, {
    id: "sfi-status",
    content: "",
    fg: PALETTE.hint,
    flexGrow: 0,
    flexShrink: 0,
  });
  contentPanel.add(globalStatusText);

  globalImageContainer = new Box(renderer, {
    id: "sfi-image-container",
    width: IMAGE_PANEL_WIDTH,
    flexShrink: 0,
    flexGrow: 0,
    border: true,
    borderStyle: "round",
    borderColor: PALETTE.borderHighlight,
    title: " Image ",
    titleAlignment: "center",
    backgroundColor: PALETTE.imagePanel,
    padding: 1,
    overflow: "hidden",
  });
  globalScreen.body.add(globalImageContainer);

  buildImagePanel(renderer);

  const found = findFirstAvailableImage();
  if (found) {
    imageIndex = found.idx;
    setImage(found.img, found.name);
  } else {
    setImage(null, "test-pattern");
  }

  renderer.setFrameCallback((_dt) => {
    if (imageDirty && globalImagePanel) {
      globalImagePanel.draw(0);
      imageDirty = false;
    }
  });

  keyHandler = (key: KeyEvent) => {
    if (key.name === "n" || key.name === "right") {
      cycleImage(1);
    } else if (key.name === "p" || key.name === "left") {
      cycleImage(-1);
    } else if (key.name === "r") {
      const name = ASSET_NAMES[imageIndex] ?? "test-pattern";
      setImage(loadAsset(name), name);
    }
  };
  renderer.keyInput.on("keypress", keyHandler);

  resizeHandler = () => {
    buildImagePanel(renderer);
  };
  renderer.on("resize", resizeHandler);
}

export function destroy(renderer: CliRenderer): void {
  renderer.clearFrameCallbacks();

  if (keyHandler) {
    renderer.keyInput.off("keypress", keyHandler);
    keyHandler = null;
  }
  if (resizeHandler) {
    renderer.off("resize", resizeHandler);
    resizeHandler = null;
  }

  globalImagePanel = null;
  globalImageContainer = null;
  globalInfoText = null;
  globalStatusText = null;
  globalImage = null;
  imageIndex = 0;
  imageDirty = false;

  if (globalScreen) {
    globalScreen.destroy();
    globalScreen = null;
  }
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
    targetFps: 30,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
}
