/**
 * Debug script for nestedZindex body layout issues
 * This debugs the specific pattern used in nestedZindex.example.ts
 * to see why the body might disappear during animation.
 */

import { Box, Screen, Text, bold, createCliRenderer, t } from "@bettertui/core";

const debugInfo = {
  frameCount: 0,
  bodyChildren: 0 as number,
  groupsContainerChildren: 0 as number,
  phase: 0,
};

export async function run(): Promise<void> {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  renderer.start();
  renderer.setBackgroundColor("#001122");

  // Create Screen
  const screen = new Screen(renderer, {
    id: "main-screen",
    backgroundColor: "#001122",
    body: {
      id: "screen-body",
      backgroundColor: "#002233",
    },
  });

  const parentContainer = screen.body;

  console.log("Screen created:");
  console.log(`  body: ${screen.body.id} (nodeId: ${screen.body.nodeId})`);
  console.log(`  container: ${screen.container.id}`);

  // Add visible debug text in top left
  const debugText = new Text(renderer, {
    id: "debug-text",
    content: t`${bold("Frame: 0")} | Children: 0 | Phase: 0`,
    position: "absolute",
    left: 1,
    top: 1,
    fg: "#ffffff",
    zIndex: 10000,
  });
  parentContainer.add(debugText);

  // Container for all groups - this is critical
  const groupsContainer = new Box(renderer, {
    id: "groups-container",
    width: "100%",
    height: "100%",
    zIndex: 5,
  });
  parentContainer.add(groupsContainer);

  // Test 1: Just one simple visible box inside groups container
  const testBox = new Box(renderer, {
    id: "test-box",
    position: "absolute",
    left: 10,
    top: 5,
    width: 30,
    height: 8,
    backgroundColor: "#440000",
    border: true,
    borderStyle: "single",
    borderColor: "#ff0000",
    title: "Test Box",
    titleAlignment: "center",
    zIndex: 1,
  });
  groupsContainer.add(testBox);

  // Frame loop for debugging
  let frameCount = 0;
  let lastLog = 0;

  renderer.setFrameCallback(async (_deltaMs) => {
    frameCount++;
    const now = Date.now();

    debugInfo.frameCount = frameCount;
    debugInfo.bodyChildren = parentContainer.getChildren().length;
    debugInfo.groupsContainerChildren = groupsContainer.getChildren().length;

    // Update debug text
    debugText.content = t`${bold("Frame:")} ${frameCount} | ${bold("Body Children:")} ${parentContainer.getChildren().length} | ${bold("Groups Container:")} ${groupsContainer.getChildren().length}`;

    // Log every 2 seconds
    if (now - lastLog > 2000) {
      lastLog = now;
      console.log(`\n[Frame ${frameCount}]`);
      console.log(`  Body children: ${parentContainer.getChildren().length}`);
      console.log("  Body boxOptions:", screen.body.boxOptions);
      console.log(`  Groups container children: ${groupsContainer.getChildren().length}`);

      // Check if groups container itself has dimensions
      console.log("  Groups container width:", groupsContainer.width);
      console.log("  Groups container height:", groupsContainer.height);

      // Check test box
      console.log("  Test box visible:", testBox.visible);
      console.log("  Test box nodeId:", testBox.nodeId);
    }

    // Periodically try to trigger a layout issue by modifying z-index
    // This mimics what the animation does
    if (frameCount % 300 === 0) {
      // Every ~5 seconds at 60fps
      debugInfo.phase = (debugInfo.phase + 1) % 4;
      console.log(`\n[Phase Change] New phase: ${debugInfo.phase}`);

      // Try different z-index values like the original does
      const zValues = [1, 10, 5, 2];
      const newZ = zValues[debugInfo.phase];
      console.log(`  Changing testBox zIndex from ${testBox.boxOptions.zIndex} to ${newZ}`);
      testBox.zIndex = newZ;
    }
  });

  // Exit handler
  const keyHandler = (key: { name: string }) => {
    if (key.name === "q" || key.name === "Q") {
      console.log("\nExiting... Final stats:");
      console.log(`  Total frames: ${frameCount}`);
      console.log(`  Final phase: ${debugInfo.phase}`);
      renderer.destroy();
      process.exit(0);
    }
  };
  renderer.keyInput.on("keypress", keyHandler);

  console.log("\nRunning... Press 'q' to exit.");
  console.log("Watch for any layout changes or disappearing elements.");
}

if (import.meta.main) {
  run().catch(console.error);
}
