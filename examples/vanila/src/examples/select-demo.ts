/**
 * Select Demo Example
 * Demonstrates list selection with the native engine
 */

import { defineExample } from "../lib/types.js";
import type { CliRenderer } from "../renderer/index.js";

interface SelectOption {
  name: string;
  description: string;
  value: string;
}

let keyHandler: ((key: { name: string; ctrl: boolean; shift: boolean }) => void) | null = null;
let selectedIndex = 0;
let selectedOption: SelectOption | null = null;

const OPTIONS: SelectOption[] = [
  { name: "Home", description: "Navigate to the home page", value: "home" },
  { name: "Profile", description: "View and edit your user profile", value: "profile" },
  { name: "Settings", description: "Configure application preferences", value: "settings" },
  { name: "Dashboard", description: "View analytics and key metrics", value: "dashboard" },
  { name: "Projects", description: "Manage your active projects", value: "projects" },
  { name: "Reports", description: "Generate and view detailed reports", value: "reports" },
  { name: "Users", description: "Manage user accounts and permissions", value: "users" },
  { name: "Analytics", description: "Deep dive into usage analytics", value: "analytics" },
  { name: "Tools", description: "Access various utility tools", value: "tools" },
  { name: "Help Center", description: "Find answers to common questions", value: "help" },
];

function render(renderer: CliRenderer): void {
  renderer.clearScreen();
  const w = renderer.terminalWidth;

  renderer.write("\x1b[1;36mSelect Demo\x1b[0m\n\n");

  const boxWidth = Math.min(60, w - 4);
  const border = "─".repeat(boxWidth);

  renderer.write(`┌${border}┐\n`);

  for (let i = 0; i < OPTIONS.length; i++) {
    const opt = OPTIONS[i];
    if (!opt) continue;
    const isSelected = i === selectedIndex;
    const prefix = isSelected ? "\x1b[1;36m→\x1b[0m " : "  ";
    const nameStyle = isSelected ? "\x1b[1;36m" : "\x1b[0m";
    const descStyle = isSelected ? "\x1b[36m" : "\x1b[2m";

    const nameLine = `${prefix}${nameStyle}${opt.name}\x1b[0m`;
    const descLine = `    ${descStyle}${opt.description}\x1b[0m`;

    renderer.write(`│ ${nameLine.padEnd(boxWidth - 2)} │\n`);
    renderer.write(`│ ${descLine.padEnd(boxWidth - 2)} │\n`);

    if (i < OPTIONS.length - 1) {
      renderer.write(`│ ${"".padEnd(boxWidth - 2)} │\n`);
    }
  }

  renderer.write(`└${border}┘\n`);

  if (selectedOption) {
    renderer.write(
      `\n\x1b[1;32m✓ Selected:\x1b[0m ${selectedOption.name} (${selectedOption.value})\n`,
    );
  } else {
    renderer.write("\n\x1b[2mNavigate with j/k or arrows, Enter to select\x1b[0m\n");
  }

  renderer.write("\n\n\x1b[2mControls:\x1b[0m\n");
  renderer.write(
    "  \x1b[36mj/k or ↑/↓\x1b[0m Navigate | \x1b[36mEnter\x1b[0m Select | \x1b[36mEscape\x1b[0m Return\n",
  );
}

export const selectDemoExample = defineExample("Widgets", {
  name: "Select Demo",
  slug: "select-demo",
  description: "Interactive list selection component",
  run(renderer: CliRenderer) {
    selectedIndex = 0;
    selectedOption = null;
    render(renderer);

    keyHandler = (key) => {
      if (key.name === "escape") {
        return;
      }

      if (key.name === "up" || key.name === "k") {
        selectedIndex = Math.max(0, selectedIndex - 1);
        render(renderer);
        return;
      }

      if (key.name === "down" || key.name === "j") {
        selectedIndex = Math.min(OPTIONS.length - 1, selectedIndex + 1);
        render(renderer);
        return;
      }

      if (key.name === "enter") {
        selectedOption = OPTIONS[selectedIndex] ?? null;
        render(renderer);
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
    selectedIndex = 0;
    selectedOption = null;
  },
});
