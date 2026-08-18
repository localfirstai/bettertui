/**
 * Minimal test for z-index visibility
 */

import { Box, Screen, Text, bold, createCliRenderer, t } from "@bettertui/core";

const renderer = await createCliRenderer({
  exitOnCtrlC: true,
  width: 80,
  height: 24,
});

renderer.start();

// Create screen
const screen = new Screen(renderer, {
  id: "main-screen",
  body: { backgroundColor: "#001122" },
});

// Create visible container (border + background)
const container = new Box(renderer, {
  id: "container",
  position: "relative",
  width: "100%",
  height: "100%",
  border: true,
  borderColor: "#ffffff",
  backgroundColor: "#002244",
});
screen.body.add(container);

// Create child box - this should be visible INSIDE container
const child1 = new Box(renderer, {
  id: "child1",
  position: "absolute",
  left: 5,
  top: 3,
  width: 20,
  height: 6,
  backgroundColor: "#440000",
  border: true,
  borderColor: "#ff0000",
  title: "Child 1",
  titleAlignment: "center",
});
container.add(child1);

// Add text inside child
const text = new Text(renderer, {
  id: "text",
  content: t`${bold("Hello")}`,
  position: "absolute",
  left: 8,
  top: 5,
  fg: "#ffffff",
});
container.add(text);

// Debug info
console.log("Container:");
console.log("  position:", container.boxOptions.position);
console.log("  width:", container.boxOptions.width);
console.log("  height:", container.boxOptions.height);
console.log("  children:", container.getChildren().length);

console.log("Child1:");
console.log("  position:", child1.boxOptions.position);
console.log("  left:", child1.boxOptions.left);
console.log("  top:", child1.boxOptions.top);

// Exit
renderer.keyInput.on("keypress", (key) => {
  if (key.name === "q") {
    screen.destroy();
    renderer.destroy();
    process.exit(0);
  }
});
