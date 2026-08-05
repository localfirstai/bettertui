#!/usr/bin/env tsx
import fs from "node:fs";
import { Box, Text, createCliRenderer } from "@bettertui/core";

// Capture stdout
let captured = "";

const origWrite = process.stdout.write.bind(process.stdout);
(process.stdout as { write: (chunk: string | Uint8Array) => boolean }).write = (
  chunk: string | Uint8Array,
) => {
  const str = typeof chunk === "string" ? chunk : Buffer.from(chunk).toString("utf8");
  captured += str;
  // Don't actually write to stdout
  return true;
};

const renderer = await createCliRenderer({
  exitOnCtrlC: false,
  targetFps: 30,
});

renderer.setBackgroundColor("transparent");

// Create a simple transparent box with text
const container = new Box(renderer, {
  id: "test-container",
  width: "100%",
  height: "100%",
  backgroundColor: "transparent",
});

const childBox = new Box(renderer, {
  id: "child-box",
  width: 20,
  height: 5,
  backgroundColor: "transparent",
  border: true,
});

const text = new Text(renderer, {
  content: "Hello",
  fg: "#ffffff",
  bg: "transparent",
});

childBox.add(text);
container.add(childBox);
renderer.root.add(container);

// Wait a few frames
await new Promise((r) => setTimeout(r, 500));

// Restore stdout write
(process.stdout as { write: typeof origWrite }).write = origWrite;

// Write captured output to file for analysis
fs.writeFileSync("/tmp/captured_bg_output.bin", captured);

// Print summary
console.error("=== CAPTURED OUTPUT (first 2000 chars) ===");
console.error(JSON.stringify(captured.slice(0, 2000)));
console.error("=== Total bytes:", captured.length, "===");

renderer.destroy();
process.exit(0);
