import { Box, Screen, Text, bold, createCliRenderer, t, underline } from "@bettertui/core";
import type { CliRenderer, KeyEvent } from "@bettertui/core";
import { APP_THEME } from "../constants/theme";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

// ── Module-level state ──────────────────────────────────────────────────────

let globalKeyboardHandler: ((key: KeyEvent) => void) | null = null;
let globalScreen: Screen | null = null;

// Animation state — initialise to -1 so the first frame always applies the
// correct phase rather than comparing against a stale "0".
let zIndexPhase = -1;
let animationSpeed = 2000;

// Current z-index values (mutated by animation).
let zIndexA = 100;
let zIndexB = 50;
let zIndexC = 20;

// ── run ─────────────────────────────────────────────────────────────────────

export function run(renderer: CliRenderer): void {
  renderer.start();

  const theme = APP_THEME.dark.saha;
  const tokens = theme.tokens;

  renderer.setBackgroundColor(tokens.background);

  // Screen provides a full-terminal layout manager.
  // body has position:"relative" and flexGrow:1, making it a proper containing
  // block for absolutely-positioned children.
  globalScreen = new Screen(renderer, {
    id: "nestedZindex-screen",
    backgroundColor: tokens.background,
    body: {
      id: "nestedZindex-body",
      backgroundColor: tokens.background,
    },
  });

  const body = globalScreen.body;

  // ── Title ────────────────────────────────────────────────────────────────
  // left:2 matches the footer baseline so all chrome is flush to the left edge.

  const title = new Text(renderer, {
    id: "main-title",
    content: t`${bold(underline("Nested Render Objects & Z-Index Demo"))}`,
    position: "absolute",
    left: 2,
    top: 1,
    fg: tokens.primary,
    zIndex: 1000,
  });
  body.add(title);

  // ── Parent groups ────────────────────────────────────────────────────────
  //
  // Three overlapping groups.  Each group's z-index controls which group
  // paints on top.  The animation cycles through four layering configurations.

  // Group A — primary green, starts highest (z=100)
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
    borderColor: tokens.primary,
    backgroundColor: tokens.secondary,
    title: "Group A  (z=100)",
    titleAlignment: "center",
  });
  body.add(parentGroupA);

  // Group B — info blue, starts middle (z=50)
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
    borderColor: tokens.info,
    backgroundColor: tokens.muted,
    title: "Group B  (z=50)",
    titleAlignment: "center",
  });
  body.add(parentGroupB);

  // Group C — warning amber, starts lowest (z=20)
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
    borderColor: tokens.warning,
    backgroundColor: tokens.accent,
    title: "Group C  (z=20)",
    titleAlignment: "center",
  });
  body.add(parentGroupC);

  // ── Children inside Group A ──────────────────────────────────────────────
  // Each child box uses flex centering so its label sits in the body centre.
  // The border title shows the child's own z-index for reference.

  const boxA1 = new Box(renderer, {
    id: "box-a1",
    position: "absolute",
    left: 2,
    top: 2,
    width: 22,
    height: 5,
    backgroundColor: tokens.accent,
    zIndex: 10,
    border: true,
    borderStyle: "single",
    borderColor: tokens.ring,
    title: "z=10",
    titleAlignment: "center",
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
  });
  parentGroupA.add(boxA1);

  const textA1 = new Text(renderer, {
    id: "text-a1",
    content: t`${bold("A1")}`,
    fg: tokens.primary,
  });
  boxA1.add(textA1);

  const boxA2 = new Box(renderer, {
    id: "box-a2",
    position: "absolute",
    left: 2,
    top: 9,
    width: 14,
    height: 3,
    backgroundColor: tokens.secondary,
    zIndex: 5,
    border: true,
    borderStyle: "single",
    borderColor: tokens.border,
    title: "z=5",
    titleAlignment: "center",
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
  });
  parentGroupA.add(boxA2);

  const textA2 = new Text(renderer, {
    id: "text-a2",
    content: t`${bold("A2")}`,
    fg: tokens.mutedForeground,
  });
  boxA2.add(textA2);

  // ── Children inside Group B ──────────────────────────────────────────────

  const boxB1 = new Box(renderer, {
    id: "box-b1",
    position: "absolute",
    left: 2,
    top: 2,
    width: 22,
    height: 5,
    backgroundColor: tokens.secondary,
    zIndex: 20,
    border: true,
    borderStyle: "double",
    borderColor: tokens.info,
    title: "z=20",
    titleAlignment: "center",
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
  });
  parentGroupB.add(boxB1);

  const textB1 = new Text(renderer, {
    id: "text-b1",
    content: t`${bold("B1")}`,
    fg: tokens.info,
  });
  boxB1.add(textB1);

  const boxB2 = new Box(renderer, {
    id: "box-b2",
    position: "absolute",
    left: 2,
    top: 9,
    width: 14,
    height: 3,
    backgroundColor: tokens.muted,
    zIndex: 15,
    border: true,
    borderStyle: "single",
    borderColor: tokens.border,
    title: "z=15",
    titleAlignment: "center",
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
  });
  parentGroupB.add(boxB2);

  const textB2 = new Text(renderer, {
    id: "text-b2",
    content: t`${bold("B2")}`,
    fg: tokens.mutedForeground,
  });
  boxB2.add(textB2);

  // ── Children inside Group C ──────────────────────────────────────────────

  const boxC1 = new Box(renderer, {
    id: "box-c1",
    position: "absolute",
    left: 2,
    top: 2,
    width: 22,
    height: 5,
    backgroundColor: tokens.secondary,
    zIndex: 30,
    border: true,
    borderStyle: "round",
    borderColor: tokens.warning,
    title: "z=30",
    titleAlignment: "center",
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
  });
  parentGroupC.add(boxC1);

  const textC1 = new Text(renderer, {
    id: "text-c1",
    content: t`${bold("C1")}`,
    fg: tokens.warning,
  });
  boxC1.add(textC1);

  const boxC2 = new Box(renderer, {
    id: "box-c2",
    position: "absolute",
    left: 2,
    top: 9,
    width: 14,
    height: 3,
    backgroundColor: tokens.muted,
    zIndex: 25,
    border: true,
    borderStyle: "single",
    borderColor: tokens.border,
    title: "z=25",
    titleAlignment: "center",
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
  });
  parentGroupC.add(boxC2);

  const textC2 = new Text(renderer, {
    id: "text-c2",
    content: t`${bold("C2")}`,
    fg: tokens.mutedForeground,
  });
  boxC2.add(textC2);

  // ── Footer — status & explanation ───────────────────────────────────────
  // All footer lines share left:2 — flush with the title above.

  const explanation1 = new Text(renderer, {
    id: "explanation1",
    content:
      "A → A1 (z=10)  A2 (z=5)   ·   B → B1 (z=20)  B2 (z=15)   ·   C → C1 (z=30)  C2 (z=25)",
    position: "absolute",
    left: 2,
    bottom: 4,
    fg: tokens.mutedForeground,
    zIndex: 1000,
  });
  body.add(explanation1);

  const explanation2 = new Text(renderer, {
    id: "explanation2",
    content:
      "Group z-index beats child z-index — C1 (z=30) is occluded by Group A (z=100) when Group C has z=20",
    position: "absolute",
    left: 2,
    bottom: 3,
    fg: tokens.mutedForeground,
    zIndex: 1000,
  });
  body.add(explanation2);

  const phaseIndicator = new Text(renderer, {
    id: "phase-indicator",
    content: t`${bold("Animation Phase: 1/4")}`,
    position: "absolute",
    left: 2,
    bottom: 1,
    fg: tokens.foreground,
    zIndex: 1000,
  });
  body.add(phaseIndicator);

  const zIndexDisplay = new Text(renderer, {
    id: "zindex-display",
    content: `Current Z-Indices — A:${zIndexA}  B:${zIndexB}  C:${zIndexC}`,
    position: "absolute",
    left: 44,
    bottom: 1,
    fg: tokens.secondaryForeground,
    zIndex: 1000,
  });
  body.add(zIndexDisplay);

  // ── Animation loop ──────────────────────────────────────────────────────

  const phases = [
    "Original Hierarchy",
    "C Group on Top",
    "B Group on Top",
    "Equal Parents — child z-index decides",
  ];

  renderer.setFrameCallback(async (_deltaMs) => {
    const time = Date.now();
    const newPhase = Math.floor((time % (animationSpeed * 4)) / animationSpeed);

    if (newPhase !== zIndexPhase) {
      zIndexPhase = newPhase;

      switch (zIndexPhase) {
        case 0: // A=100, B=50, C=20 — original
          zIndexA = 100;
          zIndexB = 50;
          zIndexC = 20;
          parentGroupA.title = "Group A  (z=100)";
          parentGroupB.title = "Group B  (z=50)";
          parentGroupC.title = "Group C  (z=20)";
          break;
        case 1: // C on top — A=50, B=20, C=100
          zIndexA = 50;
          zIndexB = 20;
          zIndexC = 100;
          parentGroupA.title = "Group A  (z=50)";
          parentGroupB.title = "Group B  (z=20)";
          parentGroupC.title = "Group C  (z=100)";
          break;
        case 2: // B on top — A=20, B=100, C=50
          zIndexA = 20;
          zIndexB = 100;
          zIndexC = 50;
          parentGroupA.title = "Group A  (z=20)";
          parentGroupB.title = "Group B  (z=100)";
          parentGroupC.title = "Group C  (z=50)";
          break;
        case 3: // All equal — A=B=C=60
          zIndexA = 60;
          zIndexB = 60;
          zIndexC = 60;
          parentGroupA.title = "Group A  (z=60)";
          parentGroupB.title = "Group B  (z=60)";
          parentGroupC.title = "Group C  (z=60)";
          break;
      }

      // Apply z-index changes — the fixed zIndex setter re-applies the full
      // layout so width/height/inset/border are never reset.
      parentGroupA.zIndex = zIndexA;
      parentGroupB.zIndex = zIndexB;
      parentGroupC.zIndex = zIndexC;

      phaseIndicator.content = `Animation Phase: ${zIndexPhase + 1}/4 — ${phases[zIndexPhase]}`;
      zIndexDisplay.content = `Current Z-Indices — A:${zIndexA}  B:${zIndexB}  C:${zIndexC}`;
    }
  });

  // ── Keyboard ─────────────────────────────────────────────────────────────

  globalKeyboardHandler = (key: KeyEvent) => {
    if (key.name === "+" || key.name === "=") {
      animationSpeed = Math.max(500, animationSpeed - 200);
    } else if (key.name === "-" || key.name === "_") {
      animationSpeed = Math.min(5000, animationSpeed + 200);
    }
  };

  renderer.keyInput.on("keypress", globalKeyboardHandler);
}

// ── destroy ─────────────────────────────────────────────────────────────────

export function destroy(renderer: CliRenderer): void {
  if (globalKeyboardHandler) {
    renderer.keyInput.off("keypress", globalKeyboardHandler);
    globalKeyboardHandler = null;
  }

  renderer.clearFrameCallbacks();

  if (globalScreen) {
    globalScreen.destroy();
    globalScreen = null;
  }

  // Reset animation state so the next run() starts cleanly.
  zIndexPhase = -1;
  animationSpeed = 2000;
  zIndexA = 100;
  zIndexB = 50;
  zIndexC = 20;
}

// ── Standalone entry-point ───────────────────────────────────────────────────

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
}
