/**
 * Debug script: Test if Screen body is visible at all
 *
 * This is a minimal reproduction to see if the body from Screen
 * shows up correctly when we add content to it.
 */

import { Box, Screen, bold, createCliRenderer, t } from "@bettertui/core";

const renderer = await createCliRenderer({
  exitOnCtrlC: true,
  width: 100,
  height: 30,
});

renderer.start();
renderer.setBackgroundColor("#001122");

console.log("=== Screen Body Debug ===");
console.log(`Terminal size: ${renderer.terminalWidth}x${renderer.terminalHeight}`);

// Create Screen
const screen = new Screen(renderer, {
  id: "main-screen",
  backgroundColor: "#112233",
  body: {
    id: "screen-body",
    backgroundColor: "#223344",
  },
});

console.log("Screen created:");
console.log(`  Container: ${screen.container.id}`);
console.log(`  Body: ${screen.body.id}`);
console.log(`  terminalWidth: ${screen.terminalWidth}`);
console.log(`  terminalHeight: ${screen.terminalHeight}`);

// Get the body
const body = screen.body;

// Create a box that fills the body
const fillBox = new Box(renderer, {
  id: "fill-box",
  width: "100%",
  height: "100%",
  backgroundColor: "#334455",
  position: "relative",
});
body.add(fillBox);
console.log("Added fill-box (100% x 100%) to body");

// Create a sub-box at known position
const subBox = new Box(renderer, {
  id: "sub-box",
  width: 30,
  height: 10,
  left: 5,
  top: 5,
  position: "absolute",
  backgroundColor: "#556677",
  border: true,
  borderColor: "#ffffff",
});
fillBox.add(subBox);
console.log("Added sub-box (30x10) at (5,5) to fill-box");

// Add text labels
const label1 = new (await import("@bettertui/core")).Text(renderer, {
  id: "label1",
  content: t`${bold("If you see this text,")}`,
  position: "absolute",
  left: 7,
  top: 6,
  fg: "#ffffff",
});
fillBox.add(label1);

const label2 = new (await import("@bettertui/core")).Text(renderer, {
  id: "label2",
  content: "the body is working!",
  position: "absolute",
  left: 7,
  top: 7,
  fg: "#ffffff",
});
fillBox.add(label2);

// Status every 2 seconds
let frame = 0;
setInterval(() => {
  frame++;
  console.log(
    `[Frame ${frame}] body children: ${body.getChildren().length}, fillBox children: ${fillBox.getChildren().length}`,
  );
}, 2000);

// Exit handler
renderer.keyInput.on("keypress", (key) => {
  if (key.name === "q" || key.name === "Q") {
    console.log("\nExiting...");
    screen.destroy();
    renderer.destroy();
    process.exit(0);
  }
});

console.log("\nRunning... Press 'q' to exit.");
