/**
 * Performance Stress Test Example
 * Demonstrates rendering performance with the native engine
 */

import type { CliRenderer } from "@bettertui/core";
import { defineExample } from "../lib/types.js";

let keyHandler: ((key: { name: string; ctrl: boolean }) => void) | null = null;
let animationInterval: ReturnType<typeof setInterval> | null = null;
let frameCount = 0;
let startTime = 0;
let isRunning = false;

function render(renderer: CliRenderer): void {
  renderer.clearScreen();
  const w = renderer.terminalWidth;
  const h = renderer.terminalHeight;

  const elapsed = startTime > 0 ? (performance.now() - startTime) / 1000 : 0;
  const fps = elapsed > 0 ? frameCount / elapsed : 0;

  renderer.write("\x1b[1;36mPerformance Stress Test\x1b[0m\n\n");

  renderer.write(`\x1b[1;33mFrames:\x1b[0m ${frameCount}\n`);
  renderer.write(`\x1b[1;33mTime:\x1b[0m ${elapsed.toFixed(2)}s\n`);
  renderer.write(`\x1b[1;33mFPS:\x1b[0m ${fps.toFixed(1)}\n\n`);

  const boxCount = Math.min(100, Math.floor((w - 2) / 8) * Math.floor((h - 10) / 4));

  renderer.write(`\x1b[1;35mDrawing ${boxCount} boxes:\x1b[0m\n\n`);

  const colors = ["41", "42", "43", "44", "45", "46", "47", "101", "102", "103", "104", "105"];
  let drawn = 0;

  for (let row = 0; row < Math.floor((h - 10) / 4) && drawn < boxCount; row++) {
    let line1 = "";
    let line2 = "";

    for (let col = 0; col < Math.floor((w - 2) / 8) && drawn < boxCount; col++) {
      const colorIndex = (drawn + frameCount) % colors.length;
      const color = colors[colorIndex];
      if (!color) continue;

      line1 += `\x1b[${color}m████████\x1b[0m`;
      line2 += `\x1b[${color}m ${(drawn + 1).toString().padStart(3, " ")}   \x1b[0m`;
      drawn++;
    }

    renderer.write(`${line1}\n${line2}\n\n`);
  }

  if (isRunning) {
    renderer.write("\n\x1b[1;32m● Running\x1b[0m | Press \x1b[36mSpace\x1b[0m to pause\n");
  } else {
    renderer.write("\n\x1b[1;33m● Paused\x1b[0m | Press \x1b[36mSpace\x1b[0m to resume\n");
  }

  renderer.write("\x1b[36mR\x1b[0m Reset | \x1b[36mEscape\x1b[0m Return to menu\n");
}

export const performanceExample = defineExample("Performance", {
  name: "Performance Stress Test",
  slug: "performance",
  description: "Test rendering performance with animated boxes",
  run(renderer: CliRenderer) {
    frameCount = 0;
    startTime = performance.now();
    isRunning = true;
    render(renderer);

    animationInterval = setInterval(() => {
      if (isRunning) {
        frameCount++;
        render(renderer);
      }
    }, 16);

    keyHandler = (key) => {
      if (key.name === "escape") {
        if (animationInterval) {
          clearInterval(animationInterval);
          animationInterval = null;
        }
        return;
      }

      if (key.name === "space") {
        isRunning = !isRunning;
        if (!isRunning) {
          render(renderer);
        }
        return;
      }

      if (key.name === "r") {
        frameCount = 0;
        startTime = performance.now();
        isRunning = true;
        render(renderer);
        return;
      }
    };

    renderer.keyInput.on("keypress", keyHandler);
  },

  destroy(renderer: CliRenderer) {
    if (animationInterval) {
      clearInterval(animationInterval);
      animationInterval = null;
    }
    if (keyHandler) {
      renderer.keyInput.off("keypress", keyHandler);
      keyHandler = null;
    }
    frameCount = 0;
    startTime = 0;
    isRunning = false;
  },
});
