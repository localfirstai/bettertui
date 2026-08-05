#!/usr/bin/env tsx
// Diagnostic tool to test rendering pipeline, sync mode, screen modes, and ANSI buffer encoding.

import { createEngine } from "@bettertui/core";

console.log("=== DEBUG: TESTING RENDER PIPELINE & ANSI BUFFER ENCODING ===");

const engine = createEngine(60, 10);
const root = engine.root();

engine.setLayout(root, JSON.stringify({ width: "100%", height: "100%" }));
engine.setStyle(root, JSON.stringify({ bg: "transparent" }));

const textNode = engine.createNode("Text");
engine.appendChild(root, textNode);
engine.setStyle(textNode, JSON.stringify({ fg: "#38bdf8", bold: true }));
engine.setText(textNode, "Testing Render Pipeline");

// 1. Begin Frame
engine.beginFrame();
const frame1 = engine.render();
engine.commitFrame();

const output1 = Buffer.from(frame1.output_data, "base64").toString();

console.log("\n1. First Frame output length:", output1.length);
console.log(
  "Contains Sync Mode Start (\\x1b[?2026h):",
  output1.includes("\x1b[?2026h") ? "PASS" : "FAIL",
);
console.log(
  "Contains Sync Mode End (\\x1b[?2026l):",
  output1.includes("\x1b[?2026l") ? "PASS" : "FAIL",
);

// 2. Dirty region check on frame 2 (no changes -> empty or minimal output)
engine.beginFrame();
const frame2 = engine.render();
engine.commitFrame();

const output2 = Buffer.from(frame2.output_data, "base64").toString();
console.log("\n2. Second Frame (unchanged) output length:", output2.length);

// 3. Full render check
engine.beginFrame();
const frame3 = engine.renderFull();
engine.commitFrame();

const output3 = Buffer.from(frame3.output_data, "base64").toString();
console.log("\n3. Full Render Frame output length:", output3.length);

engine.shutdown();
console.log("\n=== RENDER PIPELINE DEBUG COMPLETE ===");
