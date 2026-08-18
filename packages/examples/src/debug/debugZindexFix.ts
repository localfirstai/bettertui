/**
 * Debug script: Test z-index fix
 * Creates a box with position:absolute and z-index, then changes z-index
 * to verify position is preserved.
 */

import { Box, Screen, createCliRenderer } from "@bettertui/core";

const renderer = await createCliRenderer({
  exitOnCtrlC: true,
});

renderer.start();

const screen = new Screen(renderer, {
  body: { backgroundColor: "#001122" },
});

// Create absolutely positioned box
const box = new Box(renderer, {
  id: "test-box",
  position: "absolute",
  left: 5,
  top: 5,
  width: 20,
  height: 6,
  backgroundColor: "#330000",
  zIndex: 10,
  border: true,
});

console.log("Created box:");
console.log("  position:", box.boxOptions.position);
console.log("  zIndex:", box.boxOptions.zIndex);
console.log("  left:", box.boxOptions.left);
console.log("  top:", box.boxOptions.top);

// Add to screen body
screen.body.add(box);

// Wait 2 seconds then change z-index
setTimeout(() => {
  console.log("\nChanging zIndex to 50...");
  box.zIndex = 50;

  console.log("After zIndex change:");
  console.log("  position:", box.boxOptions.position);
  console.log("  zIndex:", box.boxOptions.zIndex);
  console.log("  left:", box.boxOptions.left);
  console.log("  top:", box.boxOptions.top);

  // If position stayed as "absolute", the box should still be at (5,5)
  // If position reset to "relative", the box would be laid out by flex
}, 2000);

renderer.keyInput.on("keypress", (key) => {
  if (key.name === "q") {
    screen.destroy();
    renderer.destroy();
    process.exit(0);
  }
});

console.log("\nRunning. Press 'q' to exit.");
