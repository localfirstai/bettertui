/**
 * Keyboard Input Example
 * Demonstrates keyboard event handling
 */

import { defineExample } from "../lib/types";
import type { CliRenderer, KeyEvent } from "../renderer";

let keyHandler: ((key: KeyEvent) => void) | null = null;
let keyCount = 0;

export const keyboardExample = defineExample("Input", {
  name: "Keyboard Input",
  slug: "keyboard",
  description: "Interactive keyboard event demonstration",
  run(renderer: CliRenderer) {
    renderer.clearTree();
    renderer.clearScreen();

    keyCount = 0;

    renderer.addKeyBinding("global", "quit", "escape", "quit", "Exit", 100);
    renderer.addKeyBinding("global", "quit_ctrlc", "ctrl+c", "quit", "Exit", 99);
    renderer.addKeyBinding("global", "clear", "c", "clear", "Clear screen", 50);

    process.stdout.write("\x1b[1;1H\x1b[1;36mKeyboard Input Demo\x1b[0m\n\n");
    process.stdout.write("Press any key to see its details.\n");
    process.stdout.write("Press \x1b[1;32mEscape\x1b[0m or \x1b[1;32mCtrl+C\x1b[0m to exit.\n");
    process.stdout.write("Press \x1b[1;33mc\x1b[0m to clear.\n\n");

    keyHandler = (key: KeyEvent) => {
      const cmd = renderer.handleKey(key.sequence);

      if (cmd === "quit") {
        renderer.stop();
        return;
      }

      if (cmd === "clear") {
        keyCount = 0;
        process.stdout.write("\x1b[2J\x1b[H\x1b[1;36mKeyboard Input Demo\x1b[0m\n\n");
        process.stdout.write("Screen cleared.\n\n");
        return;
      }

      keyCount++;
      let line = `#${keyCount}: key="${key.name}"`;
      const mods: string[] = [];
      if (key.ctrl) mods.push("Ctrl");
      if (key.alt) mods.push("Alt");
      if (key.shift) mods.push("Shift");
      if (mods.length > 0) line += ` [${mods.join("+")}]`;

      process.stdout.write(`${line}\n`);
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
