import { Box, type KeyEvent, Text, bold, createCliRenderer, t, underline } from "@bettertui/core";
import type { CliRenderer } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let globalKeyboardHandler: ((key: KeyEvent) => void) | null = null;
let zIndexPhase = 0;
let animationSpeed = 2000;

// Track current z-index values for display
let zIndexA = 100;
let zIndexB = 50;
let zIndexC = 20;

export function run(renderer: CliRenderer): void {
  renderer.start();
  renderer.setBackgroundColor("#001122");
  const rootContainer = new Box(renderer, {
    id: "root-container",
    width: "100%",
    height: "100%",
    zIndex: 1,
  });
  renderer.root.add(rootContainer);

  // Title
  const title = new Text(renderer, {
    id: "main-title",
    content: t`${bold(underline("Nested Render Objects & Z-Index Demo"))}`,
    position: "absolute",
    left: 10,
    top: 1,
    fg: "#FFFF00",
    zIndex: 1000,
  });
  rootContainer.add(title);

  zIndexA = 100;
  zIndexB = 50;
  zIndexC = 20;

  // ── Parent groups ───────────────────────────────────────────────────────────
  // Group A — top-left, starts with highest z-index (z=100)
  const parentGroupA = new Box(renderer, {
    id: "parent-group-a",
    position: "absolute",
    left: 4,
    top: 4,
    width: 50,
    height: 16,
    zIndex: zIndexA,
    border: true,
    borderStyle: "single",
    borderColor: "#9944FF",
    backgroundColor: "#1a0a2e",
  });
  rootContainer.add(parentGroupA);

  // Group B — offset right+down so it overlaps A
  const parentGroupB = new Box(renderer, {
    id: "parent-group-b",
    position: "absolute",
    left: 16,
    top: 7,
    width: 50,
    height: 16,
    zIndex: zIndexB,
    border: true,
    borderStyle: "single",
    borderColor: "#44FF44",
    backgroundColor: "#0a2e1a",
  });
  rootContainer.add(parentGroupB);

  // Group C — offset further so it overlaps both A and B
  const parentGroupC = new Box(renderer, {
    id: "parent-group-c",
    position: "absolute",
    left: 28,
    top: 10,
    width: 50,
    height: 16,
    zIndex: zIndexC,
    border: true,
    borderStyle: "single",
    borderColor: "#FFFF44",
    backgroundColor: "#2e2a0a",
  });
  rootContainer.add(parentGroupC);

  // ── Children inside Group A ──────────────────────────────────────────────────
  const boxA1 = new Box(renderer, {
    id: "box-a1",
    position: "absolute",
    left: 2,
    top: 2,
    width: 22,
    height: 5,
    backgroundColor: "#441155",
    zIndex: 10,
    border: true,
    borderStyle: "single",
    borderColor: "#FF88FF",
    title: "A (z=100)",
    titleAlignment: "center",
  });
  parentGroupA.add(boxA1);

  const textA1 = new Text(renderer, {
    id: "text-a1",
    content: t`${bold("Child A1")}`,
    position: "absolute",
    left: 4,
    top: 4,
    fg: "#FFFFFF",
    zIndex: 10,
  });
  parentGroupA.add(textA1);

  const boxA2 = new Box(renderer, {
    id: "box-a2",
    position: "absolute",
    left: 2,
    top: 9,
    width: 14,
    height: 3,
    backgroundColor: "#552244",
    zIndex: 5,
    border: true,
    borderStyle: "single",
    borderColor: "#FFB8FF",
  });
  parentGroupA.add(boxA2);

  const textA2 = new Text(renderer, {
    id: "text-a2",
    content: "A2",
    position: "absolute",
    left: 4,
    top: 10,
    fg: "#FFFFFF",
    zIndex: 5,
  });
  parentGroupA.add(textA2);

  // ── Children inside Group B ──────────────────────────────────────────────────
  const boxB1 = new Box(renderer, {
    id: "box-b1",
    position: "absolute",
    left: 2,
    top: 2,
    width: 22,
    height: 5,
    backgroundColor: "#115522",
    zIndex: 20,
    border: true,
    borderStyle: "double",
    borderColor: "#88FF88",
    title: "B (z=50)",
    titleAlignment: "center",
  });
  parentGroupB.add(boxB1);

  const textB1 = new Text(renderer, {
    id: "text-b1",
    content: t`${bold("Child B1")}`,
    position: "absolute",
    left: 4,
    top: 4,
    fg: "#FFFFFF",
    zIndex: 20,
  });
  parentGroupB.add(textB1);

  const boxB2 = new Box(renderer, {
    id: "box-b2",
    position: "absolute",
    left: 2,
    top: 9,
    width: 14,
    height: 3,
    backgroundColor: "#226622",
    zIndex: 15,
    border: true,
    borderStyle: "single",
    borderColor: "#AAFFAA",
  });
  parentGroupB.add(boxB2);

  const textB2 = new Text(renderer, {
    id: "text-b2",
    content: "B2",
    position: "absolute",
    left: 4,
    top: 10,
    fg: "#FFFFFF",
    zIndex: 15,
  });
  parentGroupB.add(textB2);

  // ── Children inside Group C ──────────────────────────────────────────────────
  const boxC1 = new Box(renderer, {
    id: "box-c1",
    position: "absolute",
    left: 2,
    top: 2,
    width: 22,
    height: 5,
    backgroundColor: "#554411",
    zIndex: 30,
    border: true,
    borderStyle: "round",
    borderColor: "#FFFF88",
    title: "C (z=20)",
    titleAlignment: "center",
  });
  parentGroupC.add(boxC1);

  const textC1 = new Text(renderer, {
    id: "text-c1",
    content: t`${bold("Child C1")}`,
    position: "absolute",
    left: 4,
    top: 4,
    fg: "#FFFFFF",
    zIndex: 30,
  });
  parentGroupC.add(textC1);

  const boxC2 = new Box(renderer, {
    id: "box-c2",
    position: "absolute",
    left: 2,
    top: 9,
    width: 14,
    height: 3,
    backgroundColor: "#444422",
    zIndex: 25,
    border: true,
    borderStyle: "single",
    borderColor: "#FFFFAA",
  });
  parentGroupC.add(boxC2);

  const textC2 = new Text(renderer, {
    id: "text-c2",
    content: "C2",
    position: "absolute",
    left: 4,
    top: 10,
    fg: "#FFFFFF",
    zIndex: 25,
  });
  parentGroupC.add(textC2);

  // ── Explanation / status text at bottom ────────────────────────────────────
  const termH = renderer.terminalHeight;

  const explanation1 = new Text(renderer, {
    id: "explanation1",
    content:
      "Key Concept: Parent z-index determines group layering, child z-index determines order within group",
    position: "absolute",
    left: 2,
    top: termH - 5,
    fg: "#AAAAAA",
    zIndex: 1000,
  });
  rootContainer.add(explanation1);

  const explanation2 = new Text(renderer, {
    id: "explanation2",
    content: "Even if Child C1 has z=30, it renders behind Parent A & B because Parent C has z=20",
    position: "absolute",
    left: 2,
    top: termH - 4,
    fg: "#AAAAAA",
    zIndex: 1000,
  });
  rootContainer.add(explanation2);

  const phaseIndicator = new Text(renderer, {
    id: "phase-indicator",
    content: t`${bold("Animation Phase: 1/4")}`,
    position: "absolute",
    left: 2,
    top: termH - 2,
    fg: "#FFFFFF",
    zIndex: 1000,
  });
  rootContainer.add(phaseIndicator);

  const zIndexDisplay = new Text(renderer, {
    id: "zindex-display",
    content: `Current Z-Indices - A:${zIndexA}, B:${zIndexB}, C:${zIndexC}`,
    position: "absolute",
    left: 40,
    top: termH - 2,
    fg: "#FFFFFF",
    zIndex: 1000,
  });
  rootContainer.add(zIndexDisplay);

  // ── Animation loop ─────────────────────────────────────────────────────────
  renderer.setFrameCallback(async (_deltaMs) => {
    const time = Date.now();
    const newPhase = Math.floor((time % (animationSpeed * 4)) / animationSpeed);

    if (newPhase !== zIndexPhase) {
      zIndexPhase = newPhase;

      switch (zIndexPhase) {
        case 0: // Original: A=100, B=50, C=20
          zIndexA = 100;
          zIndexB = 50;
          zIndexC = 20;
          parentGroupA.zIndex = zIndexA;
          parentGroupB.zIndex = zIndexB;
          parentGroupC.zIndex = zIndexC;
          boxA1.title = "Parent A (z=100)";
          boxB1.title = "Parent B (z=50)";
          boxC1.title = "Parent C (z=20)";
          break;
        case 1: // C becomes highest: A=50, B=20, C=100
          zIndexA = 50;
          zIndexB = 20;
          zIndexC = 100;
          parentGroupA.zIndex = zIndexA;
          parentGroupB.zIndex = zIndexB;
          parentGroupC.zIndex = zIndexC;
          boxA1.title = "Parent A (z=50)";
          boxB1.title = "Parent B (z=20)";
          boxC1.title = "Parent C (z=100)";
          break;
        case 2: // B becomes highest: A=20, B=100, C=50
          zIndexA = 20;
          zIndexB = 100;
          zIndexC = 50;
          parentGroupA.zIndex = zIndexA;
          parentGroupB.zIndex = zIndexB;
          parentGroupC.zIndex = zIndexC;
          boxA1.title = "Parent A (z=20)";
          boxB1.title = "Parent B (z=100)";
          boxC1.title = "Parent C (z=50)";
          break;
        case 3: // All equal — child z-index matters: A=B=C=60
          zIndexA = 60;
          zIndexB = 60;
          zIndexC = 60;
          parentGroupA.zIndex = zIndexA;
          parentGroupB.zIndex = zIndexB;
          parentGroupC.zIndex = zIndexC;
          boxA1.title = "Parent A (z=60)";
          boxB1.title = "Parent B (z=60)";
          boxC1.title = "Parent C (z=60)";
          break;
      }

      const phases = [
        "Original Hierarchy",
        "C Group on Top",
        "B Group on Top",
        "Equal Parents (Child z-index matters)",
      ];
      phaseIndicator.content = `Animation Phase: ${zIndexPhase + 1}/4 - ${phases[zIndexPhase]}`;
      zIndexDisplay.content = `Current Z-Indices - A:${zIndexA}, B:${zIndexB}, C:${zIndexC}`;
    }
  });

  globalKeyboardHandler = (key: KeyEvent) => {
    if (key.name === "+" || key.name === "=") {
      animationSpeed = Math.max(500, animationSpeed - 200);
    } else if (key.name === "-" || key.name === "_") {
      animationSpeed = Math.min(5000, animationSpeed + 200);
    }
  };

  renderer.keyInput.on("keypress", globalKeyboardHandler);
}

export function destroy(renderer: CliRenderer): void {
  if (globalKeyboardHandler) {
    renderer.keyInput.off("keypress", globalKeyboardHandler);
    globalKeyboardHandler = null;
  }

  const rootContainer = renderer.root.getRenderable("root-container");
  if (rootContainer) {
    rootContainer.destroyRecursively();
    renderer.root.remove(rootContainer);
  }

  renderer.clearFrameCallbacks();
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
}
