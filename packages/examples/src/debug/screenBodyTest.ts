/**
 * Simple test to verify Screen body fills the screen and accepts children
 */

import { Box, Screen, Text, bold, createCliRenderer, t } from "@bettertui/core";

const renderer = await createCliRenderer({
  exitOnCtrlC: true,
});

renderer.start();

console.log("Terminal size:", renderer.terminalWidth, "x", renderer.terminalHeight);

// Create Screen
const screen = new Screen(renderer, {
  id: "test-screen",
  body: {
    id: "body",
    backgroundColor: "#001133",
  },
});

console.log("Screen created. Body ID:", screen.body.id);

// Add a full-screen box inside body
const fillBox = new Box(renderer, {
  id: "fill",
  width: "100%",
  height: "100%",
  backgroundColor: "#002244",
  position: "relative",
});
screen.body.add(fillBox);

// Add a visible box with border
const box = new Box(renderer, {
  id: "box",
  position: "absolute",
  left: 5,
  top: 5,
  width: 30,
  height: 10,
  backgroundColor: "#440000",
  border: true,
  borderColor: "#ff0000",
  title: "Test Box",
});
fillBox.add(box);

// Add text label
const text = new Text(renderer, {
  id: "label",
  content: t`${bold("If you see this, body layout works!")}`,
  position: "absolute",
  left: 10,
  top: 7,
  fg: "#ffffff",
});
fillBox.add(text);

// Test log every 2 seconds
setInterval(() => {
  console.log("[Status] Frame running, body children:", screen.body.getChildren().length);
}, 2000);

// Exit on key press
renderer.keyInput.on("keypress", (key) => {
  if (key.name === "q") {
    renderer.destroy();
    process.exit(0);
  }
});

console.log("Test running. Press 'q' to exit.");
