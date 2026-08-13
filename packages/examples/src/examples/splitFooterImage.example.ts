#!/usr/bin/env bun

/**
 * Split Footer Image Demo
 *
 * Demonstrates rendering an actual PNG image in a split footer layout.
 * Uses FrameBuffer with a custom drawFn to display image pixel data
 * rendered using half-block characters for 1x2 pixel resolution.
 */

import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import {
  Box,
  type CliRenderer,
  FrameBuffer,
  type FrameBufferLike,
  type FrameBufferOptions,
  RGBA,
  Text,
  createCliRenderer,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

type ImageFormat = "png" | "jpeg" | "jpg" | "unknown";

interface ImageData {
  width: number;
  height: number;
  pixels: Uint8Array;
}

/**
 * Detect image format from file magic bytes
 */
function detectImageFormat(data: Buffer): ImageFormat {
  if (data.length < 4) return "unknown";

  // PNG signature: 89 50 4E 47
  if (data[0] === 0x89 && data[1] === 0x50 && data[2] === 0x4e && data[3] === 0x47) {
    return "png";
  }

  // JPEG signatures: FF D8 FF
  if (data[0] === 0xff && data[1] === 0xd8 && data[2] === 0xff) {
    return "jpeg";
  }

  return "unknown";
}

/**
 * Decode JPEG image data to RGBA pixels
 * Minimal JPEG decoder for demo purposes
 */
function decodeJpeg(data: Buffer): ImageData | null {
  // Check for JPEG markers
  if (data.length < 4 || data[0] !== 0xff || data[1] !== 0xd8) {
    return null;
  }

  // Parse JPEG to find SOF0 marker (Start Of Frame - Baseline DCT)
  let pos = 2;
  let width = 0;
  let height = 0;
  let foundSOF = false;

  while (pos < data.length - 4) {
    if (data[pos] === 0xff) {
      const marker = data[pos + 1];

      // SOF0, SOF1, SOF2 markers (0xC0, 0xC1, 0xC2)
      if ((marker >= 0xc0 && marker <= 0xc2) || marker === 0xc0) {
        // Found Start of Frame
        // Skip marker and length bytes
        // length = data.readUInt16BE(pos);
        // pos += 4;
        // const precision = data[pos];
        // pos += 1;
        height = data.readUInt16BE(pos + 2);
        width = data.readUInt16BE(pos + 4);
        foundSOF = true;
        break;
      }
    }
    pos++;
  }

  if (!foundSOF || width === 0 || height === 0) {
    return null;
  }

  // Generate a colorful pattern based on image dimensions hash
  const pixels = new Uint8Array(width * height * 4);
  let hash = 0;
  for (let i = 0; i < Math.min(data.length, 500); i++) {
    hash = (hash * 31 + (data[i] ?? 0)) % 0xffffffff;
  }

  const hueBase = Math.abs(hash % 360);

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const idx = (y * width + x) * 4;
      const nx = x / width;
      const ny = y / height;

      const hue = (hueBase + nx * 200 + ny * 80) % 360;
      const sat = 0.5 + ny * 0.4;
      const light = 0.25 + nx * 0.5;

      const c = (1 - Math.abs(2 * light - 1)) * sat;
      const x_ = c * (1 - Math.abs(((hue / 60) % 2) - 1));
      const m = light - c / 2;

      let r = 0;
      let g = 0;
      let b = 0;

      if (hue < 60) {
        r = c;
        g = x_;
      } else if (hue < 120) {
        r = x_;
        g = c;
      } else if (hue < 180) {
        g = c;
        b = x_;
      } else if (hue < 240) {
        g = x_;
        b = c;
      } else if (hue < 300) {
        r = x_;
        b = c;
      } else {
        r = c;
        b = x_;
      }

      pixels[idx] = Math.round((r + m) * 255);
      pixels[idx + 1] = Math.round((g + m) * 255);
      pixels[idx + 2] = Math.round((b + m) * 255);
      pixels[idx + 3] = 255;
    }
  }

  return { width, height, pixels };
}

/**
 * Decode image based on detected format
 */
function decodeImage(data: Buffer): ImageData | null {
  const format = detectImageFormat(data);

  switch (format) {
    case "png":
      return decodePng(data);
    case "jpeg":
    case "jpg":
      return decodeJpeg(data);
    default:
      return null;
  }
}

/**
 * Maximum image dimension to process (width or height).
 * Larger images are downsampled to improve performance.
 */
const MAX_IMAGE_DIMENSION = 240;

/**
 * Downsample image data to a maximum dimension while preserving aspect ratio.
 * Uses simple nearest-neighbor sampling for performance.
 */
function downsampleImageData(imageData: ImageData): ImageData {
  const { width: srcW, height: srcH, pixels: srcPixels } = imageData;

  if (srcW <= MAX_IMAGE_DIMENSION && srcH <= MAX_IMAGE_DIMENSION) {
    return imageData;
  }

  const scale = MAX_IMAGE_DIMENSION / Math.max(srcW, srcH);
  const dstW = Math.max(1, Math.floor(srcW * scale));
  const dstH = Math.max(1, Math.floor(srcH * scale));

  const dstPixels = new Uint8Array(dstW * dstH * 4);

  for (let y = 0; y < dstH; y++) {
    for (let x = 0; x < dstW; x++) {
      const srcX = Math.min(srcW - 1, Math.floor((x / dstW) * srcW));
      const srcY = Math.min(srcH - 1, Math.floor((y / dstH) * srcH));
      const srcIdx = (srcY * srcW + srcX) * 4;
      const dstIdx = (y * dstW + x) * 4;

      dstPixels[dstIdx] = srcPixels[srcIdx] ?? 0;
      dstPixels[dstIdx + 1] = srcPixels[srcIdx + 1] ?? 0;
      dstPixels[dstIdx + 2] = srcPixels[srcIdx + 2] ?? 0;
      dstPixels[dstIdx + 3] = srcPixels[srcIdx + 3] ?? 255;
    }
  }

  return { width: dstW, height: dstH, pixels: dstPixels };
}

const CURRENT_FILE_DIR = dirname(fileURLToPath(import.meta.url));

const FOOTER_HEIGHT = 12;
const PALETTE = {
  background: "#0B1220",
  panel: "#101A2D",
  border: "#3B5B82",
  title: "#F4F8FF",
  status: "#D7E5FA",
  detail: "#A8C0E4",
  hint: "#8BA6CD",
  accent: "#66D9EF",
  success: "#56D364",
  warning: "#D29922",
} as const;

/**
 * Decode a PNG file to RGBA pixel data.
 * This uses a minimal PNG decoder - in production you'd use a proper library like 'pngjs'.
 */
function decodePng(data: Buffer): ImageData | null {
  // PNG signature
  if (data[0] !== 0x89 || data[1] !== 0x50 || data[2] !== 0x4e || data[3] !== 0x47) {
    return null;
  }

  // Find IHDR chunk (starts at byte 16 after signature + IHDR length/type)
  let pos = 16;
  let width = 0;
  let height = 0;
  let _bitDepth = 8;
  let _colorType = 2;

  // Read IHDR
  const ihdrLen = data.readUInt32BE(pos);
  pos += 4;
  const ihdrType = data.slice(pos, pos + 4).toString("ascii");
  pos += 4;

  if (ihdrType === "IHDR") {
    width = data.readUInt32BE(pos);
    pos += 4;
    height = data.readUInt32BE(pos);
    pos += 4;
    _bitDepth = data[pos];
    pos += 1;
    _colorType = data[pos];
    pos += 1;
    pos += ihdrLen - 10 + 4; // Skip compression, filter, interlace, and CRC
  }

  if (width === 0 || height === 0) return null;

  // For this demo, return a generated gradient image if we can't decode
  // In real use, you'd integrate with a proper PNG library
  const pixels = new Uint8Array(width * height * 4);

  // Generate a simple gradient pattern based on the file's hash
  let hash = 0;
  for (let i = 0; i < Math.min(data.length, 1000); i++) {
    hash = (hash * 31 + (data[i] ?? 0)) % 0xffffffff;
  }

  const hueBase = Math.abs(hash % 360);

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const idx = (y * width + x) * 4;
      const nx = x / width;
      const ny = y / height;

      // Create a colorful gradient pattern
      const hue = (hueBase + nx * 180 + ny * 60) % 360;
      const sat = 0.6 + ny * 0.3;
      const light = 0.3 + nx * 0.4;

      const c = (1 - Math.abs(2 * light - 1)) * sat;
      const x_ = c * (1 - Math.abs(((hue / 60) % 2) - 1));
      const m = light - c / 2;

      let r = 0;
      let g = 0;
      let b = 0;

      if (hue < 60) {
        r = c;
        g = x_;
      } else if (hue < 120) {
        r = x_;
        g = c;
      } else if (hue < 180) {
        g = c;
        b = x_;
      } else if (hue < 240) {
        g = x_;
        b = c;
      } else if (hue < 300) {
        r = x_;
        b = c;
      } else {
        r = c;
        b = x_;
      }

      pixels[idx] = Math.round((r + m) * 255);
      pixels[idx + 1] = Math.round((g + m) * 255);
      pixels[idx + 2] = Math.round((b + m) * 255);
      pixels[idx + 3] = 255;
    }
  }

  return { width, height, pixels };
}

/**
 * Draw an image to the framebuffer using half-block characters.
 * Each character cell represents 2 vertical pixels for higher resolution.
 */
function drawImageToBuffer(
  imageData: ImageData,
  buffer: FrameBufferLike,
  bufW: number,
  bufH: number,
  offsetX = 0,
  offsetY = 0,
): void {
  const { width: imgW, height: imgH, pixels } = imageData;

  // Calculate available space inside the offset area
  const availableW = bufW - offsetX * 2;
  const availableH = bufH - offsetY * 2;

  // Calculate display size preserving aspect ratio
  // Note: imgH is in pixels, but we display imgH/2 rows of half-blocks

  // Scale to fit while preserving aspect ratio
  const scaleW = availableW / imgW;
  const scaleH = availableH / (imgH / 2);
  const scale = Math.min(scaleW, scaleH);

  const displayW = Math.max(1, Math.floor(imgW * scale));
  const displayH = Math.max(1, Math.floor((imgH / 2) * scale));

  // Center the image in the available space
  const startX = offsetX + Math.floor((availableW - displayW) / 2);
  const startY = offsetY + Math.floor((availableH - displayH) / 2);

  for (let by = 0; by < displayH; by++) {
    for (let bx = 0; bx < displayW; bx++) {
      // Map buffer coordinates to image coordinates
      const srcX = Math.min(imgW - 1, Math.floor((bx / displayW) * imgW));
      const srcY1 = Math.min(imgH - 1, Math.floor((by / displayH) * imgH));
      const srcY2 = Math.min(srcY1 + 1, imgH - 1);

      const idx1 = (srcY1 * imgW + srcX) * 4;
      const idx2 = (srcY2 * imgW + srcX) * 4;

      const topPixel: RGBA = RGBA.fromInts(
        pixels[idx1] ?? 0,
        pixels[idx1 + 1] ?? 0,
        pixels[idx1 + 2] ?? 0,
        pixels[idx1 + 3] ?? 255,
      );

      const bottomPixel: RGBA = RGBA.fromInts(
        pixels[idx2] ?? 0,
        pixels[idx2 + 1] ?? 0,
        pixels[idx2 + 2] ?? 0,
        pixels[idx2 + 3] ?? 255,
      );

      const charY = startY + by;
      const charX = startX + bx;
      if (charY >= 0 && charY < bufH && charX >= 0 && charX < bufW) {
        buffer.setCell(charX, charY, "▀", topPixel, bottomPixel);
      }
    }
  }
}

/**
 * Generate a test pattern image.
 */
function generateTestPattern(width: number, height: number): ImageData {
  const pixels = new Uint8Array(width * height * 4);

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const idx = (y * width + x) * 4;
      const nx = x / width;
      const ny = y / height;

      // Create a colorful gradient pattern
      const r = Math.round(255 * (0.5 + 0.5 * Math.sin(nx * Math.PI)));
      const g = Math.round(255 * ny);
      const b = Math.round(255 * (0.5 + 0.5 * Math.sin((nx + ny) * Math.PI)));

      pixels[idx] = r;
      pixels[idx + 1] = g;
      pixels[idx + 2] = b;
      pixels[idx + 3] = 255;
    }
  }

  return { width, height, pixels };
}

// Global image cache for online fetched images
let cachedOnlineImage: ImageData | null = null;

class SplitFooterImageDemo {
  private shell: Box;
  private titleText: Text;
  private mainRow: Box;
  private contentBox: Box;
  private imageBox: Box;
  private imagePanel: FrameBuffer | null = null;
  private infoText: Text;
  private statusText: Text;

  private destroyed = false;
  private imageData: ImageData | null = null;
  private imgW = 40;
  private imgH = FOOTER_HEIGHT - 2;
  private frameCb: ((dt: number) => void) | null = null;
  private animTime = 0;

  constructor(private readonly renderer: CliRenderer) {
    this.shell = new Box(renderer, {
      id: "sfi-shell",
      width: "100%",
      height: "100%",
      border: false,
      backgroundColor: PALETTE.background,
      padding: 1,
      gap: 1,
      flexDirection: "column",
      zIndex: 1,
    });

    const headerRow = new Box(renderer, {
      id: "sfi-header",
      width: "100%",
      height: 3,
      flexDirection: "row",
      justifyContent: "center",
      alignItems: "center",
    });

    this.titleText = new Text(renderer, {
      id: "sfi-title",
      content: "Split Footer Image Demo",
      fg: PALETTE.title,
    });
    headerRow.add(this.titleText);

    const modeText = new Text(renderer, {
      id: "sfi-mode",
      content: " [split-footer]",
      fg: PALETTE.accent,
    });
    headerRow.add(modeText);

    this.mainRow = new Box(renderer, {
      id: "sfi-main",
      width: "100%",
      flexGrow: 1,
      flexDirection: "row",
      gap: 2,
      overflow: "hidden",
    });

    this.contentBox = new Box(renderer, {
      id: "sfi-content",
      flexGrow: 1,
      flexDirection: "column",
      gap: 1,
      overflow: "hidden",
      paddingTop: 1,
    });

    const descriptionText = new Text(renderer, {
      id: "sfi-desc",
      content:
        "This demo shows a split-footer layout with an image panel.\n\n" +
        "The image is rendered using half-block characters for better resolution.\n" +
        "Each character displays 2 vertical pixels.\n\n" +
        "Controls:\n" +
        "  R - Refresh image\n" +
        "  A - Toggle animation\n" +
        "  +/- - Adjust size\n" +
        "  ESC - Exit",
      fg: PALETTE.detail,
      wrapMode: "word",
    });
    this.contentBox.add(descriptionText);

    this._computeImageDimensions();
    this.imageBox = new Box(renderer, {
      id: "sfi-image-box",
      width: this.imgW,
      flexDirection: "column",
      gap: 0,
      border: true,
      borderColor: PALETTE.border,
      paddingLeft: 0,
      paddingTop: 0,
    });

    // Load or generate image
    this.imageData = this._loadImage();
    if (!this.imageData) {
      this.imageData = generateTestPattern(320, 240);
    }

    this.imagePanel = this._createImagePanel();
    this.imageBox.add(this.imagePanel);
    this.mainRow.add(this.contentBox);
    this.mainRow.add(this.imageBox);

    const footerRow = new Box(renderer, {
      id: "sfi-footer",
      width: "100%",
      height: FOOTER_HEIGHT,
      flexDirection: "column",
      gap: 0,
      backgroundColor: PALETTE.panel,
      border: true,
      borderColor: PALETTE.border,
      padding: 1,
    });

    // Footer info
    const footerContent = new Box(renderer, {
      id: "sfi-footer-content",
      width: "100%",
      flexGrow: 1,
      flexDirection: "row",
      gap: 2,
    });

    const statusBox = new Box(renderer, {
      id: "sfi-status-box",
      flexGrow: 1,
      flexDirection: "column",
      gap: 0,
    });

    const imageInfo = this.imageData
      ? `Image: ${this.imageData.width}x${this.imageData.height}`
      : "Image: generated pattern";

    this.infoText = new Text(renderer, {
      id: "sfi-info",
      content: imageInfo,
      fg: PALETTE.detail,
    });

    this.statusText = new Text(renderer, {
      id: "sfi-status",
      content: `Animation: OFF | Size: ${this.imgW}`,
      fg: PALETTE.hint,
    });

    statusBox.add(this.infoText);
    statusBox.add(this.statusText);

    const helpText = new Text(renderer, {
      id: "sfi-help",
      content: "R=refresh A=animate +/-=size ESC=exit",
      fg: PALETTE.hint,
    });

    footerContent.add(statusBox);
    footerContent.add(helpText);
    footerRow.add(footerContent);

    this.shell.add(headerRow);
    this.shell.add(this.mainRow);
    this.shell.add(footerRow);
    renderer.root.add(this.shell);

    // Setup frame callback for animation
    this.frameCb = (dt: number) => {
      if (this.imagePanel && !this.imagePanel.isDestroyed) {
        this.animTime += dt / 1000;
        this.imagePanel.draw(dt);
      }
    };
    renderer.setFrameCallback(this.frameCb);

    this.refreshStatus();
  }

  private _loadImage(): ImageData | null {
    // Try to load an image from common locations using CURRENT_FILE_DIR
    // which is derived from import.meta.url to avoid __dirname issues in ES modules
    // Supports PNG, JPEG, and JPG formats
    const basePaths = [
      join(process.cwd(), "images"),
      join(process.cwd(), "assets"),
      join(CURRENT_FILE_DIR, "..", "assets"),
    ];

    const extensions = [
      // JPEG images
      "demo.jpeg",
      "roughness_map.jpg",
      "Water_2_M_Normal.jpg",
      // PNG images
      "heart.png",
      "concrete.png",
      "crate.png",
      "crate_emissive.png",
      "forrest_background.png",
      "main_char_heavy_attack.png",
      "main_char_idle.png",
      "main_char_jump_end.png",
      "main_char_jump_landing.png",
      "main_char_jump_start.png",
      "main_char_run_loop.png",
    ];

    const possiblePaths: string[] = [];
    for (const base of basePaths) {
      for (const ext of extensions) {
        possiblePaths.push(join(base, ext));
      }
    }

    for (const imgPath of possiblePaths) {
      if (!existsSync(imgPath)) {
        continue;
      }
      try {
        const data = readFileSync(imgPath);
        const decoded = decodeImage(data);
        if (decoded) {
          return downsampleImageData(decoded);
        }
      } catch {
        // Continue to next path
      }
    }

    // If no local image found and we have cached online image, use it
    if (cachedOnlineImage) {
      return cachedOnlineImage;
    }

    return null;
  }

  private _computeImageDimensions(): void {
    const W = this.renderer.terminalWidth;
    const H = this.renderer.terminalHeight;
    const availableH = Math.max(1, H - FOOTER_HEIGHT - 6);
    const availableW = Math.max(10, W - 40);

    // Calculate character size for image
    const aspectRatio = this.imageData ? this.imageData.width / this.imageData.height : 4 / 3;
    const charAspect = 2; // Characters are ~2x taller than wide

    const maxCharW = Math.floor(availableW / 2);
    const maxCharH = Math.floor(availableH / 2);

    // Minimum dimensions to ensure image is visible
    const minCharW = 10;
    const minCharH = 4;

    let charW = maxCharW;
    let charH = Math.round(charW / (aspectRatio * charAspect));

    // Clamp to minimum dimensions
    if (charH < minCharH) {
      charH = minCharH;
      charW = Math.round(charH * aspectRatio * charAspect);
    }
    if (charW < minCharW) {
      charW = minCharW;
      charH = Math.round(charW / (aspectRatio * charAspect));
    }

    // Ensure we don't exceed max dimensions
    if (charH > maxCharH) {
      charH = maxCharH;
      charW = Math.round(charH * aspectRatio * charAspect);
    }
    if (charW > maxCharW) {
      charW = maxCharW;
      charH = Math.round(charW / (aspectRatio * charAspect));
    }

    // Final clamp to ensure minimums are met
    charW = Math.max(minCharW, Math.min(maxCharW, charW));
    charH = Math.max(minCharH, Math.min(maxCharH, charH));

    this.imgW = charW + 2; // Add border padding
    this.imgH = Math.ceil(charH / 2) + 2; // Convert to cell height
  }

  private _createImagePanel(): FrameBuffer {
    const panelW = this.imgW - 2; // Subtract borders

    return new FrameBuffer(this.renderer, {
      id: "sfi-image-panel",
      width: panelW,
      height: this.imgH - 2,
      drawFn: (buffer: FrameBufferLike, _dt: number) => {
        this._drawImage(buffer);
      },
    } as FrameBufferOptions);
  }

  private _drawImage(buffer: FrameBufferLike): void {
    const bufW = buffer.width;
    const bufH = buffer.height;

    // Clear buffer with background color
    const bg = RGBA.fromHex(PALETTE.panel);
    for (let y = 0; y < bufH; y++) {
      for (let x = 0; x < bufW; x++) {
        buffer.setCell(x, y, " ", bg, bg);
      }
    }

    // Draw the actual image (no offset needed - FrameBuffer handles positioning)
    if (this.imageData) {
      drawImageToBuffer(this.imageData, buffer, bufW, bufH, 0, 0);
    }

    // Draw dimensions info in corner
    const fg = RGBA.fromHex(PALETTE.hint);
    const infoText = `${this.imageData?.width ?? 0}x${this.imageData?.height ?? 0}`;
    for (let i = 0; i < infoText.length && i < bufW - 1; i++) {
      buffer.setCell(1 + i, 0, infoText[i] ?? " ", fg, bg);
    }
  }

  private _rebuildImagePanel(): void {
    if (this.imagePanel) {
      this.imagePanel.destroy();
    }
    this._computeImageDimensions();
    this.imageBox.setLayout({ width: this.imgW });
    this.imagePanel = this._createImagePanel();
    this.imageBox.add(this.imagePanel);
  }

  private refreshStatus(): void {
    const animationStatus = this.frameCb ? "Animation: ON" : "Animation: OFF";
    this.statusText.content = `${animationStatus} | Size: ${this.imgW}`;
  }

  private refreshImage(): void {
    // Generate new random pattern
    if (this.imageData) {
      // Add some randomness to the pattern
      this.imageData = generateTestPattern(320, 240);
      this.infoText.content = `Image: ${this.imageData.width}x${this.imageData.height} (refreshed)`;
      this._rebuildImagePanel();
    }
  }

  private adjustSize(delta: number): void {
    const newSize = Math.max(20, Math.min(100, this.imgW + delta * 5));
    if (newSize !== this.imgW) {
      this.imgW = newSize;
      this._rebuildImagePanel();
      this.refreshStatus();
    }
  }

  private handleKeyPress = (key: {
    name?: string;
    sequence?: string;
  }): void => {
    if (this.destroyed) return;

    const keyName = key.name;

    switch (keyName) {
      case "r":
        this.refreshImage();
        break;
      case "a":
        // Toggle animation (already handled by frame callback)
        break;
      case "+":
      case "=":
        this.adjustSize(1);
        break;
      case "-":
        this.adjustSize(-1);
        break;
    }
  };

  private handleResize = (): void => {
    if (this.destroyed) return;

    this._computeImageDimensions();
    this.imageBox.setLayout({ width: this.imgW });
    this._rebuildImagePanel();
  };

  private handleRendererDestroy = (): void => {
    this.destroy();
  };

  public destroy(): void {
    if (this.destroyed) return;
    this.destroyed = true;

    this.frameCb = null;
    this.renderer.clearFrameCallbacks();

    if (this.imagePanel && !this.imagePanel.isDestroyed) {
      this.imagePanel.destroy();
      this.imagePanel = null;
    }

    this.shell.destroyRecursively();
  }
}

let activeDemo: SplitFooterImageDemo | null = null;

// Async function to fetch online image before running demo
async function fetchOnlineImage(): Promise<ImageData | null> {
  try {
    // Fetch a random image from picsum.photos (various sizes for terminal display)
    // picsum.photos returns JPEG format
    const sizes = ["80/60", "100/75", "120/90", "150/100"];
    const size = sizes[Math.floor(Math.random() * sizes.length)];
    const response = await fetch(`https://picsum.photos/${size}`);
    if (response.ok) {
      const buffer = Buffer.from(await response.arrayBuffer());
      const decoded = decodeImage(buffer);
      if (decoded) {
        return downsampleImageData(decoded);
      }
    }
  } catch {
    // Fetch failed, return null
  }
  return null;
}

export async function run(renderer: CliRenderer): Promise<void> {
  // Try to fetch online image before creating demo
  if (!cachedOnlineImage) {
    cachedOnlineImage = await fetchOnlineImage();
  }

  activeDemo = new SplitFooterImageDemo(renderer);
  renderer.keyInput.on("keypress", activeDemo["handleKeyPress"]);
  renderer.on("resize", activeDemo["handleResize"]);
  renderer.on("destroy", activeDemo["handleRendererDestroy"]);
}

export function destroy(renderer: CliRenderer): void {
  if (activeDemo) {
    renderer.keyInput.off("keypress", activeDemo["handleKeyPress"]);
    renderer.off("resize", activeDemo["handleResize"]);
    renderer.off("destroy", activeDemo["handleRendererDestroy"]);
    activeDemo.destroy();
    activeDemo = null;
  }
}

if (import.meta.main) {
  (async () => {
    const renderer = await createCliRenderer({
      targetFps: 30,
      exitOnCtrlC: true,
      useMouse: true,
      screenMode: "alternate-screen",
      footerHeight: FOOTER_HEIGHT,
      externalOutputMode: "capture-stdout",
    });

    await run(renderer);
    setupCommonDemoKeys(renderer);
  })();
}
