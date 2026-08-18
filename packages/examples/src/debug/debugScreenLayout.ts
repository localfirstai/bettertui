/**
 * Debug script to verify Screen layout and body sizing
 * This script creates a Screen with a debug Box filling the entire body
 * to ensure body dimensions are correct.
 */

import { Box, Screen, Text, bold, createCliRenderer, t } from "@bettertui/core";

export async function run(): Promise<void> {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
    width: 100,
    height: 30,
  });

  renderer.start();

  // Track if we should exit
  let shouldExit = false;

  // Set up key handler
  const keyHandler = (key: { name: string }) => {
    if (key.name === "q" || key.name === "Q") {
      shouldExit = true;
    }
  };
  renderer.keyInput.on("keypress", keyHandler);

  console.log("Creating Screen...");

  // Create Screen with explicit body options
  const screen = new Screen(renderer, {
    id: "main-screen",
    backgroundColor: "#001122",
    body: {
      id: "screen-body",
      backgroundColor: "#002244",
      flexDirection: "column",
      alignItems: "stretch",
    },
  });

  console.log("Screen created:");
  console.log(`  - container id: ${screen.container.id}`);
  console.log(`  - body id: ${screen.body.id}`);
  console.log(`  - terminalWidth: ${screen.terminalWidth}`);
  console.log(`  - terminalHeight: ${screen.terminalHeight}`);

  // Explore the body
  const body = screen.body;
  console.log("\nBody options:");
  console.log(`  - width from boxOptions: ${body.boxOptions.width}`);
  console.log(`  - height from boxOptions: ${body.boxOptions.height}`);
  console.log(`  - flexGrow: ${body.boxOptions.flexGrow}`);
  console.log(`  - flexShrink: ${body.boxOptions.flexShrink}`);
  console.log(`  - flexDirection: ${body.boxOptions.flexDirection}`);

  // Create a debug box that fills the entire body
  const debugFillBox = new Box(renderer, {
    id: "debug-fill-box",
    width: "100%",
    height: "100%",
    backgroundColor: "#004400",
    border: true,
    borderStyle: "single",
    borderColor: "#00ff00",
  });
  body.add(debugFillBox);
  console.log("\nAdded debug-fill-box (100% x 100%) to screen.body");

  // Add a text label
  const labelText = new Text(renderer, {
    id: "debug-label",
    content: t`${bold("This should be visible in the body")}`,
    position: "absolute",
    left: 5,
    top: 5,
    fg: "#00ff00",
    zIndex: 100,
  });
  body.add(labelText);
  console.log("Added debug label");

  // Wait a moment for initial render
  await new Promise((r) => setTimeout(r, 500));

  console.log("\n--- After 500ms, checking layout (will print to stdout after exit) ---\n");

  // Frame loop - log body dimensions
  renderer.setFrameCallback(async (_deltaMs) => {
    if (shouldExit) {
      renderer.destroy();
      process.exit(0);
    }
  });

  // Keep running for 5 seconds then log layout info
  setTimeout(() => {
    console.log("\n=== Layout Information ===");
    console.log("Screen:");
    console.log(`  terminalWidth: ${screen.terminalWidth}`);
    console.log(`  terminalHeight: ${screen.terminalHeight}`);

    console.log("\nBody (screen.body):");
    console.log(`  id: ${body.id}`);
    console.log(`  nodeId: ${body.nodeId}`);
    console.log(`  width (from boxOptions): ${body.boxOptions.width}`);
    console.log(`  height (from boxOptions): ${body.boxOptions.height}`);

    // Check if we can get computed dimensions from engine
    try {
      // @ts-ignore - accessing internal renderer method
      const nodeInfo = renderer.engine.getNode(body.nodeId);
      console.log("\nEngine node info for body:");
      console.log(nodeInfo);
    } catch (e) {
      console.log(`\nCould not get engine node info: ${e}`);
    }

    console.log("\nPress 'q' to exit...");
  }, 2000);
}

if (import.meta.main) {
  run().catch(console.error);
}
