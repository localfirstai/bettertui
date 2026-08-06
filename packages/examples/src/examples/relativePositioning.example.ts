/**
 * Relative Positioning Demo — Clean 3-box animation with header/footer.
 *
 * Demonstrates:
 * - Screen/Canvas pattern with header, body, footer
 * - Theme integration from constants/theme.ts
 * - Three animated parent boxes in the body
 * - Interactive keyboard controls
 */
import type { CliRenderer, ThemeMode } from "@bettertui/core";
import { Box, type RawKeyEvent, Screen, Text, createCliRenderer, dim, t } from "@bettertui/core";
import { DEFAULT_THEME_MODE, getComponentTheme, getThemeTokens } from "../constants/theme";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let globalScreen: Screen | null = null;
let globalKeyboardHandler: ((key: RawKeyEvent) => void) | null = null;

let animationSpeed = 4000;
let animationTime = 0;

type DemoTheme = {
  bg: string;
  surface: string;
  headerBg: string;
  headerFg: string;
  footerBg: string;
  footerFg: string;
  borderHighlight: string;
  primary: string;
  success: string;
  warning: string;
  text: string;
  textMuted: string;
};

let theme: DemoTheme;

function buildTheme(mode: ThemeMode): DemoTheme {
  const tokens = getThemeTokens(mode);
  const _comp = getComponentTheme(mode);
  return {
    bg: tokens.background,
    surface: tokens.secondary,
    headerBg: tokens.secondary,
    headerFg: tokens.primary,
    footerBg: tokens.muted,
    footerFg: tokens.mutedForeground,
    borderHighlight: tokens.primary,
    primary: tokens.primary,
    success: tokens.success,
    warning: tokens.warning,
    text: tokens.foreground,
    textMuted: tokens.mutedForeground,
  };
}

export function run(renderer: CliRenderer): void {
  theme = buildTheme(renderer.themeMode ?? DEFAULT_THEME_MODE);

  renderer.start();
  renderer.setBackgroundColor(theme.bg);

  // Estimate the stage's inner dimensions so initial positions and animations
  // never escape the container.  The formula accounts for:
  //   body padding 2 each side → -4 cols / -4 rows
  //   stage border 1 each side → -2 cols / -2 rows
  //   header=3, footer=3, stage marginTop=1 → -7 rows total
  const initStageH = Math.max(16, renderer.viewportHeight - 13);

  globalScreen = new Screen(renderer, {
    id: "relative-position-screen",
    backgroundColor: theme.bg,
    header: {
      id: "screen-header",
      height: 3,
      backgroundColor: theme.headerBg,
      border: true,
      borderStyle: "double",
      borderColor: theme.borderHighlight,
      title: "",
      alignItems: "center",
      justifyContent: "center",
      padding: 0,
    },
    body: {
      id: "screen-body",
      flexDirection: "column",
      padding: 2,
      backgroundColor: theme.bg,
    },
    footer: {
      id: "screen-footer",
      height: 3,
      backgroundColor: theme.footerBg,
      border: true,
      borderStyle: "single",
      borderColor: theme.borderHighlight,
      alignItems: "center",
      justifyContent: "space-between",
      paddingX: 2,
    },
  });

  // Header title
  const headerTitle = new Text(renderer, {
    id: "header-title",
    content: "Relative Positioning Demo - Child positions are relative to parent",
    fg: theme.headerFg,
    bg: "transparent",
    zIndex: 1,
    flexGrow: 1,
    flexShrink: 1,
    textAlign: "center",
  });
  globalScreen.header?.add(headerTitle);

  // MAIN STAGE - holds the 3 parent boxes
  const stage = new Box(renderer, {
    id: "animation-stage",
    position: "relative",
    flexGrow: 1,
    flexShrink: 1,
    marginTop: 1,
    overflow: "hidden",
    border: true,
    borderStyle: "double",
    borderColor: theme.borderHighlight,
    backgroundColor: theme.bg,
  });
  globalScreen.body.add(stage);

  // FOOTER LEFT - Speed display
  const speedDisplay = new Text(renderer, {
    id: "speed-display",
    content: `Speed: ${animationSpeed}ms`,
    fg: theme.footerFg,
    bg: "transparent",
    zIndex: 1,
    flexGrow: 0,
    flexShrink: 0,
  });
  globalScreen.footer?.add(speedDisplay);

  // FOOTER RIGHT - Controls info
  const controlsDisplay = new Text(renderer, {
    id: "controls-display",
    content: t`Range: ${dim("(min: 500ms, max: 8000ms)")} | "+" slow down  "-" speed up  (step: 500ms)`,
    fg: theme.footerFg,
    bg: "transparent",
    zIndex: 1,
    flexGrow: 0,
    flexShrink: 0,
  });
  globalScreen.footer?.add(controlsDisplay);

  // ---------- PARENT A ----------
  // Sweeps the full stage using a Lissajous figure (2:3 ratio covers all corners)
  const parentA = new Box(renderer, {
    id: "parent-a",
    position: "absolute",
    left: 0,
    top: 0,
    zIndex: 50,
  });
  stage.add(parentA);

  const parentBoxA = new Box(renderer, {
    id: "parent-box-a",
    left: 0,
    top: 0,
    width: 32,
    height: 9,
    backgroundColor: theme.surface,
    zIndex: 1,
    borderStyle: "double",
    borderColor: theme.primary,
    title: "Parent A (moves in circle)",
    titleAlignment: "center",
    flexDirection: "row",
    alignItems: "stretch",
    justifyContent: "space-between",
    border: true,
  });
  parentA.add(parentBoxA);

  const childA1 = new Box(renderer, {
    id: "child-a1",
    flexGrow: 1,
    flexShrink: 1,
    minWidth: 8,
    backgroundColor: theme.success,
    borderStyle: "single",
    borderColor: theme.headerBg,
    title: "Child 1",
    titleAlignment: "center",
    border: true,
  });
  parentBoxA.add(childA1);

  const childA2 = new Box(renderer, {
    id: "child-a2",
    flexGrow: 1,
    flexShrink: 1,
    minWidth: 8,
    backgroundColor: theme.warning,
    borderStyle: "single",
    borderColor: theme.headerBg,
    title: "Child 2",
    titleAlignment: "center",
    border: true,
  });
  parentBoxA.add(childA2);

  const childA3 = new Box(renderer, {
    id: "child-a3",
    flexGrow: 1,
    flexShrink: 1,
    minWidth: 8,
    backgroundColor: theme.primary,
    borderStyle: "single",
    borderColor: theme.headerBg,
    title: "Child 3",
    titleAlignment: "center",
    border: true,
  });
  parentBoxA.add(childA3);

  // ---------- PARENT B ----------
  // Moving vertically
  const parentB = new Box(renderer, {
    id: "parent-b",
    position: "absolute",
    left: 0,
    top: 0,
    zIndex: 50,
  });
  stage.add(parentB);

  const parentBoxB = new Box(renderer, {
    id: "parent-box-b",
    left: 0,
    top: 0,
    width: 32,
    height: 8,
    backgroundColor: theme.surface,
    zIndex: 1,
    borderStyle: "round",
    borderColor: theme.success,
    title: "Parent B (moves vertically)",
    titleAlignment: "center",
    padding: 1,
    flexDirection: "column",
    justifyContent: "space-between",
    border: true,
  });
  parentB.add(parentBoxB);

  const parentLabelB = new Text(renderer, {
    id: "parent-label-b",
    content: "Parent B Position: (50, 8)",
    fg: theme.text,
    zIndex: 2,
  });
  parentBoxB.add(parentLabelB);

  parentBoxB.add(
    new Text(renderer, {
      id: "child-b1",
      content: "Child at relative (1,3)",
      fg: theme.textMuted,
      zIndex: 2,
    }),
  );

  // ---------- STATIC PARENT ----------
  // Never moves — pinned 1 col / 1 row from the bottom-left corner of the stage.
  const staticParent = new Box(renderer, {
    id: "static-parent",
    position: "absolute",
    left: 1,
    top: Math.max(1, initStageH - 8),
    zIndex: 50,
  });
  stage.add(staticParent);

  const staticBox = new Box(renderer, {
    id: "static-box",
    left: 0,
    top: 0,
    width: 32,
    height: 7,
    backgroundColor: theme.surface,
    zIndex: 1,
    borderStyle: "single",
    borderColor: theme.warning,
    title: "Static Parent (doesn't move)",
    titleAlignment: "center",
    padding: 1,
    flexDirection: "column",
    border: true,
    overflow: "hidden",
  });
  staticParent.add(staticBox);

  staticBox.add(
    new Text(renderer, {
      id: "static-text-1",
      content: "Static child at (2,2)",
      fg: theme.textMuted,
      zIndex: 2,
    }),
  );

  staticBox.add(
    new Text(renderer, {
      id: "static-text-2",
      content: "Never moves from here",
      fg: theme.textMuted,
      zIndex: 2,
    }),
  );

  // ANIMATION
  const BOX_A_W = 32;
  const BOX_A_H = 9;
  const BOX_B_W = 32;
  const BOX_B_H = 8;

  renderer.setFrameCallback(async (deltaMs) => {
    animationTime += deltaMs;

    // Recompute stage inner bounds each frame so the demo adapts to resize.
    const stageW = Math.max(40, renderer.terminalWidth - 6);
    const stageH = Math.max(16, renderer.viewportHeight - 13);

    // Parent A — Lissajous sweep across the entire stage (ratio 2:3).
    // (sin * 0.5 + 0.5) maps [-1,1] → [0,1], always within container bounds.
    const t = (animationTime / animationSpeed) * Math.PI * 2;
    parentA.setPosition({
      left: Math.round((Math.sin(t * 2 + Math.PI / 4) * 0.5 + 0.5) * (stageW - BOX_A_W)),
      top: Math.round((Math.sin(t * 3) * 0.5 + 0.5) * (stageH - BOX_A_H)),
    });

    // Parent B — full vertical travel, strictly inside the container.
    // (sin * 0.5 + 0.5) maps to [0, stageH - BOX_B_H] with no possibility of escape.
    const bX = Math.max(0, Math.min(stageW - BOX_B_W, Math.floor(stageW * 0.62)));
    const vertSpeed = (animationTime / (animationSpeed * 1.5)) * Math.PI * 2;
    const bY = Math.round((Math.sin(vertSpeed) * 0.5 + 0.5) * (stageH - BOX_B_H));

    parentB.setPosition({ left: bX, top: bY });
    parentLabelB.content = `Parent B Position: (${bX}, ${bY})`;
  });

  // KEYBOARD HANDLER
  globalKeyboardHandler = (key: RawKeyEvent) => {
    if (key.name === "+" || key.name === "=") {
      animationSpeed = Math.min(8000, animationSpeed + 500);
      speedDisplay.content = `Speed: ${animationSpeed}ms`;
    } else if (key.name === "-" || key.name === "_") {
      animationSpeed = Math.max(500, animationSpeed - 500);
      speedDisplay.content = `Speed: ${animationSpeed}ms`;
    }
  };

  renderer.keyInput.on("keypress", globalKeyboardHandler);
}

export function destroy(renderer: CliRenderer): void {
  renderer.clearFrameCallbacks();

  if (globalKeyboardHandler) {
    renderer.keyInput.off("keypress", globalKeyboardHandler);
    globalKeyboardHandler = null;
  }

  if (globalScreen) {
    globalScreen.destroy();
    globalScreen = null;
  }

  animationTime = 0;
  animationSpeed = 4000;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
}
