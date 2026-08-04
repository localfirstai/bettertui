#!/usr/bin/env tsx
// Diagnostic tool to verify style updates, style merging, borders, and color parsing.

import { createEngine } from "@bettertui/core";

console.log("=== DEBUG: TESTING ENGINE STYLE MERGING AND PARSING ===");

const engine = createEngine(40, 10);
const root = engine.root();

// 1. Test style merging: sequential setStyle calls should merge fields, not overwrite
const box = engine.createNode("Box");
engine.appendChild(root, box);

console.log("Setting initial style: fg=#ff0000...");
engine.setStyle(box, JSON.stringify({ fg: "#ff0000" }));

console.log("Applying second style update: bg=#0000ff (should merge with fg)...");
engine.setStyle(box, JSON.stringify({ bg: "#0000ff" }));

console.log("Applying border style update: border=single, border_color=#00ff00...");
engine.setStyle(box, JSON.stringify({ border: "single", border_color: "#00ff00" }));

engine.setLayout(box, JSON.stringify({ width: "100%", height: "100%" }));
engine.setText(box, "Style Test");

engine.beginFrame();
const frame = engine.renderFull();
engine.commitFrame();

const output = Buffer.from(frame.output_data, "base64").toString();
console.log("\nRender output (escaped):");
console.log(output.replaceAll("\x1b", "\\x1b"));

// Verify SGR codes in output (may be combined in compound SGR e.g. 38;2;R;G;B;48;2;R;G;B)
const fgMatch = output.includes("38;2;255;0;0");
const bgMatch = output.includes("48;2;0;0;255");
console.log("\nChecks:");
console.log("- Contains FG #ff0000 SGR:", fgMatch ? "PASS" : "FAIL");
console.log("- Contains BG #0000ff SGR:", bgMatch ? "PASS" : "FAIL");

engine.shutdown();
console.log("\n=== STYLE DEBUG COMPLETE ===");
