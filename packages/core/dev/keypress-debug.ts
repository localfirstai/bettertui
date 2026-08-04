#!/usr/bin/env tsx

/**
 * Keypress Debug Tool for BetterTUI.
 *
 * Puts stdin in raw mode and prints detailed parsed keypress information
 * including raw escape sequences, modifiers, codes, and Kitty protocol fields.
 *
 * Usage:
 *   pnpm dev:keypress
 */

import { parseKeypress } from "../src/lib/parseKeypress";

console.log("\x1b[1;36mBetterTUI Keypress Debug Tool\x1b[0m");
console.log("Press keys to see their parsed output. Press \x1b[1;33mCtrl+C\x1b[0m to exit.\n");

if (!process.stdin.isTTY) {
  console.error("Error: stdin is not a TTY terminal.");
  process.exit(1);
}

process.stdin.setRawMode(true);
process.stdin.resume();

process.stdin.on("data", (data: Buffer) => {
  // Check for Ctrl+C to exit
  if (data.toString() === "\x03") {
    console.log("\nExiting keypress debug tool...");
    process.stdin.setRawMode(false);
    process.exit(0);
  }

  const parsed = parseKeypress(data, { useKittyKeyboard: true });

  console.log("\x1b[32mInput data:\x1b[0m", JSON.stringify(data.toString()));
  console.log("\x1b[32mRaw buffer:\x1b[0m", data);
  console.log("\x1b[32mParsed:\x1b[0m", {
    name: parsed?.name,
    ctrl: parsed?.ctrl,
    meta: parsed?.meta,
    shift: parsed?.shift,
    option: parsed?.option,
    number: parsed?.number,
    sequence: parsed?.sequence,
    code: parsed?.code,
    eventType: parsed?.eventType,
    source: parsed?.source,
  });
  console.log("\x1b[90m--------------------------------------------------\x1b[0m");
});

process.on("SIGINT", () => {
  console.log("\nExiting keypress debug tool...");
  process.stdin.setRawMode(false);
  process.exit(0);
});
