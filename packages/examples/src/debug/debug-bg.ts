#!/usr/bin/env tsx
// Diagnostic: capture the raw ANSI output from a single render frame.

import { Box, Text, createCliRenderer } from "@bettertui/core";

// Intercept stdout to capture ANSI output
const captured: string[] = [];
const origWrite = process.stdout.write.bind(process.stdout);
(process.stdout as { write: (s: string) => boolean }).write = (chunk: string | Uint8Array) => {
  const str = typeof chunk === "string" ? chunk : Buffer.from(chunk).toString("utf8");
  captured.push(str);
  // Don't actually write to terminal
  return true;
};

const renderer = await createCliRenderer({
  exitOnCtrlC: false,
  targetFps: 60,
});

renderer.setBackgroundColor("transparent");

const container = new Box(renderer, {
  id: "test-container",
  flexDirection: "column",
  width: "100%",
  height: "100%",
  backgroundColor: "transparent",
});
renderer.root.add(container);

const label = new Text(renderer, {
  id: "test-label",
  content: "Hello World",
  fg: "#ffffff",
});
container.add(label);

// Wait a moment for frame loop
await new Promise((r) => setTimeout(r, 500));

// Restore stdout
(process.stdout as { write: typeof origWrite }).write = origWrite;

// Analyze captured output
const allOutput = captured.join("");
console.log("=== CAPTURED OUTPUT LENGTH:", allOutput.length, "===\n");

// Look for SGR sequences that set bg to white (47) or bright white (107)
// biome-ignore lint/suspicious/noControlCharactersInRegex: needed for ANSI sequence regex
const sgrRegex = /\x1B\[([0-9;]*)m/g;
let match: RegExpExecArray | null = sgrRegex.exec(allOutput);
const sgrCodes: string[] = [];
while (match !== null) {
  sgrCodes.push(match[1] || "0");
  match = sgrRegex.exec(allOutput);
}

console.log("=== SGR sequences found:", sgrCodes.length, "===");
// Show unique SGR codes
const unique = [...new Set(sgrCodes)].sort();
console.log("Unique SGR codes:", unique);

// Check for bg=47 (white bg), bg=107 (bright white bg), bg=48;2;...;255;255;255 (rgb white)
const whiteBg = sgrCodes.filter((c) => {
  const parts = c.split(";").map(Number);
  if (parts.includes(47) || parts.includes(107)) return true;
  // Check for RGB white: 48;2;255;255;255 or near-white
  const bgIdx = parts.indexOf(48);
  if (bgIdx >= 0 && parts[bgIdx + 1] === 2) {
    const r = parts[bgIdx + 2] || 0;
    const g = parts[bgIdx + 3] || 0;
    const b = parts[bgIdx + 4] || 0;
    if (r > 200 && g > 200 && b > 200) return true;
  }
  return false;
});
console.log("\n=== WHITE BG SGR codes:", whiteBg.length, "===");
if (whiteBg.length > 0) {
  console.log("White bg codes:", whiteBg.slice(0, 20));
}

// Show the raw output as escaped string (first 2000 chars)
console.log("\n=== RAW OUTPUT (first 2000 chars, escaped) ===");
const escaped = allOutput.replaceAll("\x1b", "\\x1b").slice(0, 2000);
console.log(escaped);

// Show frame info
console.log("\n=== RENDERER INFO ===");
console.log("Width:", renderer.terminalWidth, "Height:", renderer.terminalHeight);
console.log("Theme mode:", renderer.themeMode);

renderer.destroy();
process.exit(0);
