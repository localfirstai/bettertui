/**
 * Input Demo Example
 * Demonstrates text input handling with the native engine
 */

import { type NapiTextEngine, createTextEngine } from "@bettertui/core";
import { defineExample } from "../lib/types.js";
import type { CliRenderer } from "../renderer/index.js";

let keyHandler:
  | ((key: { name: string; ctrl: boolean; shift: boolean; sequence: string }) => void)
  | null = null;
let textEngine: NapiTextEngine | null = null;
let inputValue = "";
let cursorPos = 0;

function render(renderer: CliRenderer): void {
  renderer.clearScreen();

  const w = renderer.terminalWidth;

  renderer.write("\x1b[1;36mInput Demo\x1b[0m\n\n");

  renderer.write("\x1b[2mType something below:\x1b[0m\n\n");

  const boxWidth = Math.min(60, w - 4);
  const border = "─".repeat(boxWidth);

  renderer.write(`┌${border}┐\n`);
  renderer.write("│ ");

  const displayValue = inputValue || "\x1b[2mEnter text...\x1b[0m";
  const paddedValue = displayValue.padEnd(boxWidth - 2);
  renderer.write(paddedValue.slice(0, boxWidth - 2));

  renderer.write(" │\n");
  renderer.write(`└${border}┘\n`);

  if (inputValue) {
    const cursorCol = 2 + cursorPos;
    renderer.write(`\x1b[4A\x1b[${cursorCol}G\x1b[5m \x1b[0m\x1b[4B`);
  }

  renderer.write(`\n\n\x1b[2mValue:\x1b[0m \x1b[1;33m"${inputValue}"\x1b[0m\n`);
  renderer.write(
    `\x1b[2mLength:\x1b[0m ${inputValue.length} | \x1b[2mCursor:\x1b[0m ${cursorPos}\n`,
  );

  renderer.write("\n\n\x1b[2mControls:\x1b[0m\n");
  renderer.write("  \x1b[36m←/→\x1b[0m Move cursor | \x1b[36mHome/End\x1b[0m Jump to start/end\n");
  renderer.write(
    "  \x1b[36mBackspace\x1b[0m Delete | \x1b[36mCtrl+U\x1b[0m Clear | \x1b[36mEscape\x1b[0m Return\n",
  );
}

export const inputDemoExample = defineExample("Input", {
  name: "Input Demo",
  slug: "input-demo",
  description: "Interactive text input demonstration",
  run(renderer: CliRenderer) {
    textEngine = createTextEngine();
    inputValue = "";
    cursorPos = 0;
    render(renderer);

    keyHandler = (key) => {
      if (key.name === "escape") {
        return;
      }

      if (key.ctrl && key.name === "u") {
        inputValue = "";
        cursorPos = 0;
        textEngine?.clear();
        render(renderer);
        return;
      }

      if (key.name === "left") {
        cursorPos = Math.max(0, cursorPos - 1);
        textEngine?.setCursorPosition(cursorPos);
        render(renderer);
        return;
      }

      if (key.name === "right") {
        cursorPos = Math.min(inputValue.length, cursorPos + 1);
        textEngine?.setCursorPosition(cursorPos);
        render(renderer);
        return;
      }

      if (key.name === "home") {
        cursorPos = 0;
        textEngine?.setCursorPosition(0);
        render(renderer);
        return;
      }

      if (key.name === "end") {
        cursorPos = inputValue.length;
        textEngine?.setCursorPosition(cursorPos);
        render(renderer);
        return;
      }

      if (key.name === "backspace") {
        if (cursorPos > 0) {
          inputValue = inputValue.slice(0, cursorPos - 1) + inputValue.slice(cursorPos);
          cursorPos--;
          textEngine?.setCursorPosition(cursorPos);
          render(renderer);
        }
        return;
      }

      if (key.name === "delete") {
        if (cursorPos < inputValue.length) {
          inputValue = inputValue.slice(0, cursorPos) + inputValue.slice(cursorPos + 1);
          render(renderer);
        }
        return;
      }

      if (key.sequence && key.sequence.length === 1 && !key.ctrl) {
        const char = key.sequence;
        if (char.charCodeAt(0) >= 32) {
          inputValue = inputValue.slice(0, cursorPos) + char + inputValue.slice(cursorPos);
          cursorPos++;
          textEngine?.insertChar(char);
          render(renderer);
        }
      }
    };

    renderer.keyInput.on("keypress", keyHandler);
  },

  destroy(renderer: CliRenderer) {
    if (keyHandler) {
      renderer.keyInput.off("keypress", keyHandler);
      keyHandler = null;
    }
    textEngine = null;
    inputValue = "";
    cursorPos = 0;
  },
});
