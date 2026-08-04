#!/usr/bin/env tsx
// Directly test the engine's color parsing and rendering

import { createEngine } from "@bettertui/core";

const engine = createEngine(20, 5);
const root = engine.root();

// Set root bg to transparent
engine.setStyle(root, JSON.stringify({ bg: "transparent" }));
console.log("Root bg set to 'transparent'");

// Create a box
const box = engine.createNode("Box");
engine.appendChild(root, box);
engine.setStyle(box, JSON.stringify({ fg: "#ffffff" }));
engine.setLayout(box, JSON.stringify({ width: "100%", height: "100%" }));
engine.setText(box, "Hello");

// Render
engine.beginFrame();
const frame = engine.render();
engine.commitFrame();

const output = Buffer.from(frame.output_data, "base64").toString();
console.log("Raw output (escaped):", output.replaceAll("\x1b", "\\x1b"));

// Check for white bg codes
const has47 = output.includes("\x1b[47m") || output.includes("\x1b[;47m");
const has107 = output.includes("\x1b[107m");
const hasWhiteBg = /\[48;2;2?5[0-9];2?5[0-9];2?5[0-9]m/.test(output);
console.log("Has bg=47 (white):", has47);
console.log("Has bg=107 (bright white):", has107);
console.log("Has bg=48;2;~25x3 (near white):", hasWhiteBg);

// Now test with bg="default"
engine.setStyle(root, JSON.stringify({ bg: "default" }));
engine.beginFrame();
const frame2 = engine.renderFull();
engine.commitFrame();
const output2 = Buffer.from(frame2.output_data, "base64").toString();
console.log("\nWith bg='default':");
console.log("Raw output (escaped):", output2.replaceAll("\x1b", "\\x1b").slice(0, 500));

// Now test with NO bg set at all
engine.setStyle(root, JSON.stringify({}));
engine.beginFrame();
const frame3 = engine.renderFull();
engine.commitFrame();
const output3 = Buffer.from(frame3.output_data, "base64").toString();
console.log("\nWith no bg:");
console.log("Raw output (escaped):", output3.replaceAll("\x1b", "\\x1b").slice(0, 500));

engine.shutdown();
