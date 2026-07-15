/**
 * Terminal Capabilities Example
 * Demonstrates terminal capability detection
 */

import { detectCapabilities, getVersion } from "@bettertui/core";
import { defineExample } from "../lib/types.js";
import type { CliRenderer } from "../renderer/index.js";

let keyHandler: ((key: { name: string; ctrl: boolean }) => void) | null = null;

function render(renderer: CliRenderer): void {
  renderer.clearScreen();
  const caps = detectCapabilities();

  renderer.write("\x1b[1;36mTerminal Capabilities\x1b[0m\n\n");

  renderer.write(`\x1b[1;33mEngine Version:\x1b[0m ${getVersion()}\n\n`);

  renderer.write(`\x1b[1;33mTerminal:\x1b[0m ${caps.brand || "Unknown"}\n`);
  renderer.write(`\x1b[1;33mSize:\x1b[0m ${caps.columns} x ${caps.rows}\n\n`);

  renderer.write("\x1b[1;35mCapabilities:\x1b[0m\n\n");

  const capabilities: [string, boolean][] = [
    ["True Color (24-bit)", caps.true_color],
    ["Kitty Keyboard Protocol", caps.kitty_keyboard],
    ["CSI-u Key Encoding", caps.csi_u],
    ["Bracketed Paste", caps.bracketed_paste],
    ["Focus Events", caps.focus_events],
    ["Mouse Support", caps.mouse],
    ["OSC 52 (Clipboard)", caps.osc52],
    ["OSC 8 (Hyperlinks)", caps.osc8],
    ["Sync Output", caps.sync],
    ["SGR Pixel Mouse", caps.sgr_pixel],
    ["Underline Color", caps.underline_color],
    ["Strikethrough", caps.strikethrough],
    ["Cursor Styles", caps.cursor_style],
    ["Alternate Scroll", caps.alternate_scroll],
    ["Inline Images", caps.inline_images],
    ["Sixel Graphics", caps.sixel],
  ];

  const maxNameLen = Math.max(...capabilities.map(([name]) => name.length));

  for (const [name, supported] of capabilities) {
    const status = supported ? "\x1b[1;32m✓ Yes\x1b[0m" : "\x1b[1;31m✗ No\x1b[0m";
    const paddedName = name.padEnd(maxNameLen);
    renderer.write(`  ${paddedName}  ${status}\n`);
  }

  renderer.write("\n\n\x1b[2mPress Escape to return to menu\x1b[0m");
}

export const capabilitiesExample = defineExample("Core", {
  name: "Terminal Capabilities",
  slug: "capabilities",
  description: "Detect and display terminal capabilities",
  run(renderer: CliRenderer) {
    render(renderer);

    keyHandler = (key) => {
      if (key.name === "escape" || (key.ctrl && key.name === "c")) {
        return;
      }
    };
    renderer.keyInput.on("keypress", keyHandler);
  },

  destroy(renderer: CliRenderer) {
    if (keyHandler) {
      renderer.keyInput.off("keypress", keyHandler);
      keyHandler = null;
    }
  },
});
