/**
 * Flex Layout Example
 * Demonstrates flexbox layout with the native engine
 */

import type { CliRenderer } from "@bettertui/core";
import { defineExample } from "../lib/types.js";

let keyHandler: ((key: { name: string; ctrl: boolean }) => void) | null = null;
let currentLayout = 0;

const LAYOUTS = [
  { name: "Horizontal", direction: "row" },
  { name: "Vertical", direction: "column" },
  { name: "Grid", direction: "grid" },
];

function renderLayout(renderer: CliRenderer): void {
  renderer.clearScreen();
  const layout = LAYOUTS[currentLayout];
  if (!layout) return;

  const w = renderer.terminalWidth;
  const h = renderer.terminalHeight;

  renderer.write(`\x1b[1;36mFlex Layout: ${layout.name}\x1b[0m\n\n`);

  if (layout.direction === "row") {
    const boxWidth = Math.floor((w - 4) / 3);
    for (let i = 0; i < 3; i++) {
      const colors = ["41", "42", "44"];
      renderer.write(`\x1b[${colors[i]}m${" ".repeat(boxWidth)}\x1b[0m `);
    }
    renderer.write("\n\n");
    for (let i = 0; i < 3; i++) {
      const colors = ["41", "42", "44"];
      renderer.write(`\x1b[${colors[i]}m \x1b[1;37mBox ${i + 1}\x1b[0m          `);
    }
  } else if (layout.direction === "column") {
    const boxHeight = Math.floor((h - 6) / 3);
    for (let i = 0; i < 3; i++) {
      const colors = ["44", "45", "46"];
      renderer.write(`\x1b[${colors[i]}m${" ".repeat(w - 2)}\x1b[0m\n`);
      for (let j = 0; j < boxHeight - 1; j++) {
        renderer.write(`\x1b[${colors[i]}m${" ".repeat(w - 2)}\x1b[0m\n`);
      }
    }
  } else {
    const cellW = Math.floor((w - 4) / 3);
    const cellH = Math.floor((h - 6) / 3);
    const colors = ["41", "42", "43", "44", "45", "46", "47", "100", "41"];
    for (let row = 0; row < 3; row++) {
      for (let col = 0; col < 3; col++) {
        const idx = row * 3 + col;
        renderer.write(`\x1b[${colors[idx]}m${" ".repeat(cellW)}\x1b[0m `);
      }
      renderer.write("\n");
      for (let h = 0; h < cellH - 1; h++) {
        for (let col = 0; col < 3; col++) {
          renderer.write(`\x1b[${colors[row * 3 + col]}m${" ".repeat(cellW)}\x1b[0m `);
        }
        renderer.write("\n");
      }
    }
  }

  renderer.write("\n\n\x1b[2mPress Space to cycle layouts | Escape to return\x1b[0m");
}

export const flexLayoutExample = defineExample("Layout", {
  name: "Flex Layout",
  slug: "flex-layout",
  description: "Demonstrates flexbox layout patterns",
  run(renderer: CliRenderer) {
    renderLayout(renderer);

    keyHandler = (key) => {
      if (key.name === "escape" || (key.ctrl && key.name === "c")) {
        return;
      }
      if (key.name === "space") {
        currentLayout = (currentLayout + 1) % LAYOUTS.length;
        renderLayout(renderer);
      }
    };
    renderer.keyInput.on("keypress", keyHandler);
  },

  destroy(renderer: CliRenderer) {
    if (keyHandler) {
      renderer.keyInput.off("keypress", keyHandler);
      keyHandler = null;
    }
    currentLayout = 0;
  },
});
