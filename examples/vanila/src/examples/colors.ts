/**
 * Colors Example
 * Demonstrates terminal color capabilities
 */

import { detectCapabilities } from "@bettertui/core";
import type { CliRenderer } from "@bettertui/core";
import { defineExample } from "../lib/types";

let keyHandler: ((key: { name: string; ctrl: boolean }) => void) | null = null;

const COLORS = [
  { name: "Black", fg: "30", bg: "40" },
  { name: "Red", fg: "31", bg: "41" },
  { name: "Green", fg: "32", bg: "42" },
  { name: "Yellow", fg: "33", bg: "43" },
  { name: "Blue", fg: "34", bg: "44" },
  { name: "Magenta", fg: "35", bg: "45" },
  { name: "Cyan", fg: "36", bg: "46" },
  { name: "White", fg: "37", bg: "47" },
];

export const colorsExample = defineExample("Core", {
  name: "Colors Demo",
  slug: "colors",
  description: "Terminal color palette demonstration",
  run(renderer: CliRenderer) {
    renderer.clearTree();
    renderer.clearScreen();

    const caps = detectCapabilities();

    process.stdout.write("\x1b[1;1H\x1b[1;36mBetterTUI Colors Demo\x1b[0m\n\n");
    process.stdout.write(`Terminal: ${caps.brand}\n`);
    process.stdout.write(`True Color: ${caps.true_color ? "Yes" : "No"}\n\n`);

    process.stdout.write("Standard Colors:\n");
    for (const color of COLORS) {
      process.stdout.write(`  \x1b[${color.fg}m████\x1b[0m ${color.name}\n`);
    }

    process.stdout.write("\nBright Colors:\n");
    for (const color of COLORS) {
      const brightFg = String(Number(color.fg) + 60);
      process.stdout.write(`  \x1b[${brightFg}m████\x1b[0m Bright ${color.name}\n`);
    }

    process.stdout.write("\n\nPress Escape to return to menu");

    keyHandler = (key) => {
      if (key.name === "escape" || (key.ctrl && key.name === "c")) {
        renderer.stop();
      }
    };
    renderer.keyHandler.on("keypress", keyHandler);
  },

  destroy(renderer: CliRenderer) {
    if (keyHandler) {
      renderer.keyHandler.off("keypress", keyHandler);
      keyHandler = null;
    }
  },
});
