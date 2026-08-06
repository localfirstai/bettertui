#!/usr/bin/env bun

import {
  type CliRenderer,
  FrameBuffer,
  type FrameBufferLike,
  type KeyEvent,
  RGBA,
  createCliRenderer,
} from "@bettertui/core";
import {
  PATTERN_NAMES,
  drawGrayscaleBuffer,
  drawGrayscaleBufferSupersampled,
  getIntensity,
} from "../lib/grayscalePatterns";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let framebuffer: FrameBufferLike | null = null;
let keyListener: ((key: KeyEvent) => void) | null = null;
let resizeListener: ((width: number, height: number) => void) | null = null;
let leftBuffer: Float32Array | null = null;
let rightBuffer: Float32Array | null = null;

let patternMode = 0;

export async function run(renderer: CliRenderer): Promise<void> {
  renderer.start();

  let WIDTH = renderer.terminalWidth;
  let HEIGHT = renderer.terminalHeight;
  let time = 0;
  let paused = false;

  const framebufferRenderable = new FrameBuffer(renderer, {
    id: "grayscale-demo",
    width: WIDTH,
    height: HEIGHT,
    zIndex: 0,
  });
  renderer.root.add(framebufferRenderable);
  framebuffer = framebufferRenderable.frameBuffer;

  function renderDemo(): void {
    if (!framebuffer) return;

    const fb = framebuffer;
    const totalWidth = fb.width;
    const totalHeight = fb.height;

    const headerHeight = 3;
    const panelHeight = totalHeight - headerHeight;
    const panelWidth = Math.floor((totalWidth - 3) / 2);

    if (panelWidth < 10 || panelHeight < 5) return;

    const bgColor = RGBA.fromInts(20, 20, 30, 255);

    fb.fillRect(0, 0, totalWidth, totalHeight, bgColor);

    if (!leftBuffer || leftBuffer.length !== panelWidth * panelHeight) {
      leftBuffer = new Float32Array(panelWidth * panelHeight);
    }
    for (let y = 0; y < panelHeight; y++) {
      for (let x = 0; x < panelWidth; x++) {
        leftBuffer[y * panelWidth + x] = getIntensity(
          patternMode,
          x,
          y,
          panelWidth,
          panelHeight,
          time,
        );
      }
    }
    drawGrayscaleBuffer(fb, 0, headerHeight, leftBuffer, panelWidth, panelHeight);

    const rightX = panelWidth + 3;
    const ssWidth = panelWidth * 2;
    const ssHeight = panelHeight * 2;
    if (!rightBuffer || rightBuffer.length !== ssWidth * ssHeight) {
      rightBuffer = new Float32Array(ssWidth * ssHeight);
    }
    for (let y = 0; y < ssHeight; y++) {
      for (let x = 0; x < ssWidth; x++) {
        rightBuffer[y * ssWidth + x] = getIntensity(patternMode, x, y, ssWidth, ssHeight, time);
      }
    }
    drawGrayscaleBufferSupersampled(fb, rightX, headerHeight, rightBuffer, ssWidth, ssHeight);

    const dividerX = panelWidth + 1;
    for (let y = headerHeight; y < totalHeight; y++) {
      fb.setCell(dividerX, y, "|", RGBA.fromInts(60, 60, 80, 255), bgColor);
    }

    const headerBg = RGBA.fromInts(40, 40, 60, 255);
    const labelColor = RGBA.fromInts(200, 200, 220, 255);
    const highlightColor = RGBA.fromInts(100, 200, 255, 255);

    fb.fillRect(0, 0, totalWidth, headerHeight, headerBg);

    const leftLabel = "1:1 Standard";
    const leftLabelX = Math.floor(panelWidth / 2 - leftLabel.length / 2);
    for (let i = 0; i < leftLabel.length; i++) {
      fb.setCell(leftLabelX + i, 1, leftLabel[i], labelColor, headerBg);
    }

    const rightLabel = "2x Supersampled";
    const rightLabelX = rightX + Math.floor(panelWidth / 2 - rightLabel.length / 2);
    for (let i = 0; i < rightLabel.length; i++) {
      fb.setCell(rightLabelX + i, 1, rightLabel[i], highlightColor, headerBg);
    }

    const info = `[${PATTERN_NAMES[patternMode]}] SPACE:pause P:pattern`;
    const infoX = Math.floor(totalWidth / 2 - info.length / 2);
    for (let i = 0; i < info.length; i++) {
      fb.setCell(infoX + i, 0, info[i], RGBA.fromInts(150, 150, 170, 255), headerBg);
    }
  }

  keyListener = (key: KeyEvent) => {
    switch (key.name) {
      case "space":
        paused = !paused;
        break;
      case "p":
        patternMode = (patternMode + 1) % 6;
        break;
    }
  };
  renderer.keyInput.on("keypress", keyListener);

  resizeListener = (width: number, height: number) => {
    WIDTH = width;
    HEIGHT = height;
    // Note: FrameBufferLike.resize() not in current API; new content will render at next frame
  };
  renderer.on("resize", resizeListener);

  renderer.setFrameCallback(async (deltaTime) => {
    if (!paused) {
      time += (deltaTime / 1000) * 0.8;
    }
    renderDemo();
  });
}

export function destroy(renderer: CliRenderer): void {
  renderer.clearFrameCallbacks();

  if (resizeListener) {
    renderer.off("resize", resizeListener);
    resizeListener = null;
  }

  if (keyListener) {
    renderer.keyInput.off("keypress", keyListener);
    keyListener = null;
  }

  const grayscaleDemo = renderer.root.getRenderable("grayscale-demo");
  if (grayscaleDemo) renderer.root.remove(grayscaleDemo);
  framebuffer = null;
  leftBuffer = null;
  rightBuffer = null;
  patternMode = 0;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
    targetFps: 30,
  });
  await run(renderer);
  setupCommonDemoKeys(renderer);
}
