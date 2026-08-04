#!/usr/bin/env tsx
// Diagnostic tool to test layout calculations (Taffy flexbox integration, dimensions, positioning).

import { createEngine } from "@bettertui/core";

console.log("=== DEBUG: TESTING ENGINE LAYOUT ENGINE ===");

const engine = createEngine(80, 24);
const root = engine.root();

// Set root layout
engine.setLayout(
  root,
  JSON.stringify({
    width: "100%",
    height: "100%",
    flexDirection: "column",
    padding: 1,
  }),
);

// Header box
const header = engine.createNode("Box");
engine.appendChild(root, header);
engine.setLayout(
  header,
  JSON.stringify({
    width: "100%",
    height: 3,
  }),
);
engine.setStyle(header, JSON.stringify({ bg: "#1e293b", fg: "#ffffff" }));
engine.setText(header, "Header Title");

// Content box with two columns
const main = engine.createNode("Box");
engine.appendChild(root, main);
engine.setLayout(
  main,
  JSON.stringify({
    width: "100%",
    flexGrow: 1,
    flexDirection: "row",
    gap: 1,
  }),
);

const leftCol = engine.createNode("Box");
engine.appendChild(main, leftCol);
engine.setLayout(leftCol, JSON.stringify({ width: "30%", height: "100%" }));
engine.setStyle(leftCol, JSON.stringify({ bg: "#0f172a" }));
engine.setText(leftCol, "Sidebar");

const rightCol = engine.createNode("Box");
engine.appendChild(main, rightCol);
engine.setLayout(rightCol, JSON.stringify({ flexGrow: 1, height: "100%" }));
engine.setStyle(rightCol, JSON.stringify({ bg: "#1e1e1e" }));
engine.setText(rightCol, "Main Body Content");

// Render
engine.beginFrame();
const frame = engine.renderFull();
engine.commitFrame();

const output = Buffer.from(frame.output_data, "base64").toString();
console.log(`Rendered frame output byte size: ${frame.output_data.length}`);
console.log("Render output preview (first 1000 chars):");
console.log(output.replaceAll("\x1b", "\\x1b").slice(0, 1000));

engine.shutdown();
console.log("\n=== LAYOUT DEBUG COMPLETE ===");
