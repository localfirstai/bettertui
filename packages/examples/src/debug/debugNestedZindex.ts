import { Box, Text, bold, createCliRenderer, t } from "@bettertui/core";
import type { CliRenderer } from "@bettertui/core";

let globalKeyboardHandler: ((key: import("@bettertui/core").KeyEvent) => void) | null = null;

export function run(renderer: CliRenderer): void {
  renderer.start();
  renderer.setBackgroundColor("#001122");

  console.log("=== NESTED Z-INDEX DEBUG ===");
  console.log("Terminal size:", renderer.terminalWidth, "x", renderer.terminalHeight);

  // Create a full-screen container
  const parentContainer = new Box(renderer, {
    id: "parent-container",
    width: "100%",
    height: "100%",
  });
  renderer.root.add(parentContainer);
  console.log("1. Added parent-container (full screen)");

  // Create a positioned parent group
  const parentGroupA = new Box(renderer, {
    id: "parent-group-a",
    position: "absolute",
    left: 0,
    top: 0,
    width: 80,
    height: 25,
    zIndex: 100,
    visible: true,
  });
  parentContainer.add(parentGroupA);
  console.log("2. Added parent-group-a at (0,0) size 80x25, zIndex 100");

  // Create a child box inside the parent group - should appear at (15, 8) relative to screen
  const boxA1 = new Box(renderer, {
    id: "box-a1",
    position: "absolute",
    left: 15,
    top: 8,
    width: 25,
    height: 6,
    backgroundColor: "#220044",
    zIndex: 10,
    borderStyle: "single",
    borderColor: "#FF44FF",
    title: "Box A1 (z=10)",
    titleAlignment: "center",
    border: true,
  });
  parentGroupA.add(boxA1);
  console.log("3. Added box-a1 at (15,8) relative to parent, size 25x6");

  // Add a child text
  const textA1 = new Text(renderer, {
    id: "text-a1",
    content: t`${bold("Child A1")}`,
    position: "absolute",
    left: 17,
    top: 10,
    fg: "#FF44FF",
    zIndex: 10,
  });
  parentGroupA.add(textA1);
  console.log("4. Added text-a1 at (17,10) relative to parent");

  // Create a second group at lower z-index
  const parentGroupB = new Box(renderer, {
    id: "parent-group-b",
    position: "absolute",
    left: 0,
    top: 0,
    width: 80,
    height: 25,
    zIndex: 50,
    visible: true,
  });
  parentContainer.add(parentGroupB);
  console.log("5. Added parent-group-b at (0,0) size 80x25, zIndex 50");

  const boxB1 = new Box(renderer, {
    id: "box-b1",
    position: "absolute",
    left: 30,
    top: 12,
    width: 25,
    height: 6,
    backgroundColor: "#004422",
    zIndex: 20,
    borderStyle: "double",
    borderColor: "#44FF44",
    title: "Box B1 (z=20)",
    titleAlignment: "center",
    border: true,
  });
  parentGroupB.add(boxB1);
  console.log("6. Added box-b1 at (30,12) relative to parent, size 25x6");

  // Add instructions
  const instructions = new Text(renderer, {
    id: "instructions",
    content:
      "DEBUG: Should see 2 boxes (purple A1 on top of green B1). Press F12 for debug overlay.",
    position: "absolute",
    left: 2,
    top: renderer.terminalHeight - 2,
    fg: "#FFFFFF",
  });
  parentContainer.add(instructions);
  console.log("7. Added instructions");

  // Debug: verify tree structure after a delay
  setTimeout(() => {
    console.log("=== RENDER TREE VERIFICATION ===");

    const verifyRenderable = (id: string) => {
      const r = renderer.root.getRenderable(id);
      if (r) {
        const box = r as Box;
        console.log(`${id}:`, {
          found: true,
          width: box.width,
          height: box.height,
          visible: box.visible,
          zIndex: box.zIndex,
        });
        return true;
      }
      console.log(`${id}: NOT FOUND`);
      return false;
    };

    verifyRenderable("parent-container");
    verifyRenderable("parent-group-a");
    verifyRenderable("box-a1");
    verifyRenderable("text-a1");
    verifyRenderable("parent-group-b");
    verifyRenderable("box-b1");
  }, 500);
}

export function destroy(renderer: CliRenderer): void {
  if (globalKeyboardHandler) {
    renderer.keyInput.off("keypress", globalKeyboardHandler);
    globalKeyboardHandler = null;
  }

  for (const id of [
    "parent-container",
    "parent-group-a",
    "box-a1",
    "text-a1",
    "parent-group-b",
    "box-b1",
    "instructions",
  ]) {
    const child = renderer.root.getRenderable(id);
    if (child) renderer.root.remove(child);
  }

  renderer.clearFrameCallbacks();
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
}
