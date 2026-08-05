import {
  type BorderSide,
  Box,
  RGBA,
  type RawKeyEvent,
  Screen,
  Text,
  createCliRenderer,
  parseColor,
} from "@bettertui/core";
import type { CliRenderer } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";
import { TabController } from "../lib/tabController";

// ── Helpers ────────────────────────────────────────────────────────────────────

function getBorderFromSides(sides: {
  top: boolean;
  right: boolean;
  bottom: boolean;
  left: boolean;
}): boolean | BorderSide[] {
  const result: BorderSide[] = [];
  if (sides.top) result.push("top");
  if (sides.right) result.push("right");
  if (sides.bottom) result.push("bottom");
  if (sides.left) result.push("left");
  if (result.length === 4) return true;
  if (result.length === 0) return false;
  return result;
}

function hsvToRgb(h: number, s: number, v: number): RGBA {
  const c = v * s;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = v - c;
  let r = 0;
  let g = 0;
  let b = 0;
  if (h < 60) {
    r = c;
    g = x;
  } else if (h < 120) {
    r = x;
    g = c;
  } else if (h < 180) {
    g = c;
    b = x;
  } else if (h < 240) {
    g = x;
    b = c;
  } else if (h < 300) {
    r = x;
    b = c;
  } else {
    r = c;
    b = x;
  }
  return RGBA.fromValues(r + m, g + m, b + m, 1);
}

function rgbToHex(color: RGBA): string {
  const r = Math.round(color.r * 255)
    .toString(16)
    .padStart(2, "0");
  const g = Math.round(color.g * 255)
    .toString(16)
    .padStart(2, "0");
  const b = Math.round(color.b * 255)
    .toString(16)
    .padStart(2, "0");
  return `#${r}${g}${b}`;
}

// ── Module-level state ─────────────────────────────────────────────────────────

let globalScreen: Screen | null = null;
let globalTabController: TabController | null = null;
let globalKeyboardHandler: ((key: RawKeyEvent) => void) | null = null;

// ── Colors ──────────────────────────────────────────────────────────────────────

const theme = {
  bg: "#000028",
  headerBg: "#1a1a3e",
  headerFg: "#FFFF00",
  footerBg: "#1a1a3e",
  footerFg: "#888888",
  cardBg: "#1e1e3e",
  cardBorder: "#666688",
  accent1: "#FFFF00",
  accent2: "#00FF00",
  accent3: "#FF6464",
  accent4: "#8888FF",
  muted: "#CCCCCC",
  textWhite: "#FFFFFF",
};

// ── Tab init functions ─────────────────────────────────────────────────────────
// These standalone functions are superseded by the inline tab definitions in
// run() and kept only as reference implementations.

// biome-ignore lint/correctness/noUnusedVariables: legacy reference implementation
function initTextAttributesTab(tabGroup: Box, _renderer: CliRenderer): void {
  const wheelRadius = 7;
  const wheelCenterX = 70;
  const wheelCenterY = 15;
  let activeWheelPixels = new Set<string>();

  const headerRow = new Box(_renderer, {
    id: "text-tab-header",
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    paddingX: 1,
    height: 1,
  });
  tabGroup.add(headerRow);

  const title = new Text(_renderer, {
    id: "text-title",
    content: "Text Styling & Color Gradients",
    fg: theme.accent1,
    zIndex: 10,
  });
  headerRow.add(title);

  const contentRow = new Box(_renderer, {
    id: "text-tab-content",
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    paddingX: 1,
    paddingY: 1,
    gap: 1,
  });
  tabGroup.add(contentRow);

  // Left column: text attributes
  const leftCol = new Box(_renderer, {
    id: "text-tab-left-col",
    flexDirection: "column",
    flexGrow: 0,
    flexShrink: 0,
    width: 25,
    gap: 1,
  });
  contentRow.add(leftCol);

  const attrSection = new Box(_renderer, {
    id: "attr-section",
    flexDirection: "column",
    border: true,
    borderStyle: "single",
    borderColor: theme.cardBorder,
    backgroundColor: theme.cardBg,
    paddingX: 1,
    paddingY: 1,
    gap: 1,
  });
  leftCol.add(attrSection);

  const attrTitle = new Text(_renderer, {
    content: "Text Attributes",
    fg: theme.accent1,
    zIndex: 10,
  });
  attrSection.add(attrTitle);

  const attrBold = new Text(_renderer, {
    content: "Bold Text",
    fg: theme.textWhite,
    zIndex: 10,
  });
  attrSection.add(attrBold);

  const attrItalic = new Text(_renderer, {
    content: "Italic Text",
    fg: theme.textWhite,
    zIndex: 10,
  });
  attrSection.add(attrItalic);

  const attrUnderline = new Text(_renderer, {
    content: "Underlined Text",
    fg: theme.textWhite,
    zIndex: 10,
  });
  attrSection.add(attrUnderline);

  const attrDim = new Text(_renderer, {
    content: "Dim Text",
    fg: theme.textWhite,
    zIndex: 10,
  });
  attrSection.add(attrDim);

  const attrCombined = new Text(_renderer, {
    content: "Bold + Italic + Underline",
    fg: theme.accent3,
    zIndex: 10,
  });
  attrSection.add(attrCombined);

  // Right column: gradient
  const rightCol = new Box(_renderer, {
    id: "text-tab-right-col",
    flexDirection: "column",
    flexGrow: 1,
    flexShrink: 1,
    gap: 1,
  });
  contentRow.add(rightCol);

  const gradientCard = new Box(_renderer, {
    id: "gradient-card",
    flexDirection: "column",
    border: true,
    borderStyle: "single",
    borderColor: theme.cardBorder,
    backgroundColor: theme.cardBg,
    paddingX: 1,
    paddingY: 1,
    gap: 1,
  });
  rightCol.add(gradientCard);

  const gradientTitle = new Text(_renderer, {
    content: "Rainbow Gradient:",
    fg: theme.muted,
    zIndex: 10,
  });
  gradientCard.add(gradientTitle);

  const gradientRow = new Box(_renderer, {
    id: "gradient-row",
    flexDirection: "row",
    gap: 0,
  });
  gradientCard.add(gradientRow);

  for (let i = 0; i < 40; i++) {
    const hue = (i / 40) * 360;
    const color = hsvToRgb(hue, 1, 1);
    const hexColor = rgbToHex(color);

    const gradientPixel = new Text(_renderer, {
      id: `gradient-${i}`,
      content: "█",
      fg: hexColor,
      zIndex: 10,
    });
    gradientRow.add(gradientPixel);
  }

  const updateWheel = (_deltaMs: number) => {
    const time = Date.now() / 1000;
    const rotationSpeed = 45;
    const rotationAngle = (time * rotationSpeed) % 360;
    const rotationRadians = rotationAngle * (Math.PI / 180);

    const newWheelPixels = new Set<string>();

    for (let y = wheelCenterY - wheelRadius; y <= wheelCenterY + wheelRadius; y++) {
      for (let x = wheelCenterX - wheelRadius * 2; x <= wheelCenterX + wheelRadius * 2; x++) {
        const dx = (x - wheelCenterX) / 2;
        const dy = y - wheelCenterY;
        const distance = Math.sqrt(dx * dx + dy * dy);

        if (distance <= wheelRadius) {
          const angle = Math.atan2(dy, dx);
          const rotatedAngle = angle + rotationRadians;
          const hue = ((rotatedAngle / Math.PI) * 180 + 180) % 360;
          const saturation = distance / wheelRadius;
          const color = hsvToRgb(hue, saturation, 1);

          const pixelId = `wheel-${x}-${y}`;
          newWheelPixels.add(pixelId);

          const existingPixel = tabGroup.getRenderable(pixelId) as Text;
          if (existingPixel) {
            existingPixel.setPosition({ left: x, top: y });
            existingPixel.fg = color;
          } else {
            const wheelPixel = new Text(_renderer, {
              id: pixelId,
              content: "█",
              position: "absolute",
              left: x,
              top: y,
              fg: color,
              zIndex: 10,
            });
            tabGroup.add(wheelPixel);
            activeWheelPixels.add(pixelId);
          }
        }
      }
    }

    for (const pixelId of activeWheelPixels) {
      if (!newWheelPixels.has(pixelId)) {
        const pixel = tabGroup.getRenderable(pixelId);
        if (pixel) tabGroup.remove(pixel);
        activeWheelPixels.delete(pixelId);
      }
    }

    activeWheelPixels = newWheelPixels;
  };

  // biome-ignore lint/correctness/noVoidTypeReturn: returns frame callback for legacy callers
  return updateWheel as unknown as undefined;
}

// biome-ignore lint/correctness/noUnusedVariables: legacy reference implementation
function initBasicsTab(tabGroup: Box, renderer: CliRenderer): void {
  const contentRow = new Box(renderer, {
    id: "basics-content",
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    paddingX: 1,
    paddingY: 1,
    gap: 1,
  });
  tabGroup.add(contentRow);

  // Title card
  const titleCard = new Box(renderer, {
    id: "basics-title-card",
    flexDirection: "column",
    flexGrow: 0,
    flexShrink: 0,
    width: 30,
    border: true,
    borderStyle: "single",
    borderColor: theme.cardBorder,
    backgroundColor: theme.cardBg,
    paddingX: 1,
    paddingY: 1,
    gap: 1,
  });
  contentRow.add(titleCard);

  const titleText = new Text(renderer, {
    content: "Basic CLI Renderer Demo",
    fg: theme.accent1,
    zIndex: 10,
  });
  titleCard.add(titleText);

  const box1 = new Box(renderer, {
    id: "box1",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: 5,
    backgroundColor: "#333366",
    borderStyle: "single",
    borderColor: theme.textWhite,
    border: true,
  });
  titleCard.add(box1);

  const box1Title = new Text(renderer, {
    content: "Simple Box",
    fg: theme.textWhite,
    zIndex: 10,
  });
  box1.add(box1Title);

  // Info card
  const infoCard = new Box(renderer, {
    id: "basics-info-card",
    flexDirection: "column",
    flexGrow: 1,
    flexShrink: 1,
    border: true,
    borderStyle: "double",
    borderColor: theme.accent1,
    backgroundColor: "#663333",
    paddingX: 1,
    paddingY: 1,
    gap: 1,
  });
  contentRow.add(infoCard);

  const infoTitle = new Text(renderer, {
    content: "Double Border Box",
    fg: theme.textWhite,
    zIndex: 10,
  });
  infoCard.add(infoTitle);

  const description = new Text(renderer, {
    id: "description",
    content: "This tab demonstrates basic box and text rendering with different border styles.",
    fg: theme.muted,
    zIndex: 10,
  });
  infoCard.add(description);

  const cursorInfo = new Text(renderer, {
    id: "cursor-info",
    content: "Cursor: (0,0) - Style: block",
    fg: theme.textWhite,
    zIndex: 10,
  });
  infoCard.add(cursorInfo);

  // biome-ignore lint/correctness/noVoidTypeReturn: returns frame callback for legacy callers
  return (() => {
    const cursorTime = Date.now() / 1000;
    const cursorX = 15 + Math.floor(3 * Math.cos(cursorTime));
    const cursorY = 13 + Math.floor(2 * Math.sin(cursorTime));

    const cursorStyleIndex = Math.floor(cursorTime / 2) % 6;
    let cursorStyle: "block" | "line" | "underline" = "block";
    let cursorBlinking = false;

    switch (cursorStyleIndex) {
      case 0:
        cursorStyle = "block";
        cursorBlinking = false;
        break;
      case 1:
        cursorStyle = "block";
        cursorBlinking = true;
        break;
      case 2:
        cursorStyle = "line";
        cursorBlinking = false;
        break;
      case 3:
        cursorStyle = "line";
        cursorBlinking = true;
        break;
      case 4:
        cursorStyle = "underline";
        cursorBlinking = false;
        break;
      case 5:
        cursorStyle = "underline";
        cursorBlinking = true;
        break;
    }

    const cursorInfoEl = tabGroup.getRenderable("cursor-info") as Text;
    if (cursorInfoEl) {
      cursorInfoEl.content = `Cursor: (${cursorX},${cursorY}) - Style: ${cursorStyle}${cursorBlinking ? " (blinking)" : ""}`;
    }
  }) as unknown as undefined;
}

// biome-ignore lint/correctness/noUnusedVariables: legacy reference implementation
function initBordersTab(tabGroup: Box, _renderer: CliRenderer): void {
  let partialBorderPhase = 0;

  const contentRow = new Box(_renderer, {
    id: "borders-content",
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    paddingX: 1,
    paddingY: 1,
    gap: 1,
  });
  tabGroup.add(contentRow);

  // Left column: border styles
  const leftCol = new Box(_renderer, {
    id: "borders-left-col",
    flexDirection: "column",
    flexGrow: 0,
    flexShrink: 0,
    width: 25,
    gap: 1,
  });
  contentRow.add(leftCol);

  const borderTitle = new Text(_renderer, {
    content: "Border Styles",
    fg: theme.accent1,
    zIndex: 10,
  });
  leftCol.add(borderTitle);

  const singleBox = new Box(_renderer, {
    id: "single-box",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: 3,
    backgroundColor: "#222244",
    borderStyle: "single",
    borderColor: theme.textWhite,
    border: true,
  });
  leftCol.add(singleBox);
  const singleLabel = new Text(_renderer, {
    content: "Single",
    fg: theme.textWhite,
    zIndex: 10,
  });
  singleBox.add(singleLabel);

  const doubleBox = new Box(_renderer, {
    id: "double-box",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: 3,
    backgroundColor: "#442222",
    borderStyle: "double",
    borderColor: theme.textWhite,
    border: true,
  });
  leftCol.add(doubleBox);
  const doubleLabel = new Text(_renderer, {
    content: "Double",
    fg: theme.textWhite,
    zIndex: 10,
  });
  doubleBox.add(doubleLabel);

  const roundedBox = new Box(_renderer, {
    id: "rounded-box",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: 3,
    backgroundColor: "#224422",
    borderStyle: "round",
    borderColor: theme.textWhite,
    border: true,
  });
  leftCol.add(roundedBox);
  const roundedLabel = new Text(_renderer, {
    content: "Rounded",
    fg: theme.textWhite,
    zIndex: 10,
  });
  roundedBox.add(roundedLabel);

  // Right column: partial borders and animated borders
  const rightCol = new Box(_renderer, {
    id: "borders-right-col",
    flexDirection: "column",
    flexGrow: 1,
    flexShrink: 1,
    gap: 1,
  });
  contentRow.add(rightCol);

  const partialTitle = new Text(_renderer, {
    content: "Partial Borders:",
    fg: theme.muted,
    zIndex: 10,
  });
  rightCol.add(partialTitle);

  const partialRow = new Box(_renderer, {
    id: "borders-partial-row",
    flexDirection: "row",
    gap: 1,
  });
  rightCol.add(partialRow);

  const partialLeft = new Box(_renderer, {
    id: "partial-left",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: 3,
    backgroundColor: "#222244",
    borderStyle: "single",
    borderColor: theme.textWhite,
    border: ["left"],
  });
  partialRow.add(partialLeft);
  const partialLeftLabel = new Text(_renderer, {
    content: "Left Only",
    fg: theme.textWhite,
    zIndex: 10,
  });
  partialLeft.add(partialLeftLabel);

  const partialAnimated = new Box(_renderer, {
    id: "partial-animated",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    flexGrow: 1,
    height: 3,
    backgroundColor: "#334455",
    borderStyle: "single",
    borderColor: theme.textWhite,
    border: true,
  });
  partialRow.add(partialAnimated);
  const partialAnimatedLabel = new Text(_renderer, {
    content: "Animated Borders",
    fg: theme.textWhite,
    zIndex: 10,
  });
  partialAnimated.add(partialAnimatedLabel);

  const partialPhase = new Text(_renderer, {
    id: "partial-phase",
    content: "Phase: 1/8",
    fg: "#AAAAAA",
    zIndex: 10,
  });
  rightCol.add(partialPhase);

  const customBorderTitle = new Text(_renderer, {
    content: "Custom Border Characters:",
    fg: theme.muted,
    zIndex: 10,
  });
  rightCol.add(customBorderTitle);

  const customRow = new Box(_renderer, {
    id: "borders-custom-row",
    flexDirection: "row",
    gap: 1,
  });
  rightCol.add(customRow);

  const asciiBox = new Box(_renderer, {
    id: "ascii-box",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: 3,
    backgroundColor: "#222244",
    borderStyle: "ascii",
    borderColor: theme.textWhite,
    border: true,
  });
  customRow.add(asciiBox);
  const asciiLabel = new Text(_renderer, {
    content: "ASCII",
    fg: theme.textWhite,
    zIndex: 10,
  });
  asciiBox.add(asciiLabel);

  const blockBox = new Box(_renderer, {
    id: "block-box",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: 3,
    backgroundColor: "#442222",
    borderStyle: "thick",
    borderColor: theme.textWhite,
    border: true,
  });
  customRow.add(blockBox);
  const blockLabel = new Text(_renderer, {
    content: "Block",
    fg: theme.textWhite,
    zIndex: 10,
  });
  blockBox.add(blockLabel);

  const dashedBox = new Box(_renderer, {
    id: "dashed-box",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: 3,
    backgroundColor: "#224422",
    borderStyle: "dashed",
    borderColor: theme.textWhite,
    border: true,
  });
  customRow.add(dashedBox);
  const dashedLabel = new Text(_renderer, {
    content: "Dashed",
    fg: theme.textWhite,
    zIndex: 10,
  });
  dashedBox.add(dashedLabel);

  // biome-ignore lint/correctness/noVoidTypeReturn: returns frame callback for legacy callers
  return (() => {
    const time = Date.now() / 1000;
    const phase = Math.floor(time % 8);

    if (phase !== partialBorderPhase) {
      partialBorderPhase = phase;

      const borderSides = {
        top: [0, 3, 5, 7].includes(phase),
        right: [1, 3, 6, 7].includes(phase),
        bottom: [2, 3, 5, 7].includes(phase),
        left: [4, 5, 6, 7].includes(phase),
      };

      const partialAnimatedBox = tabGroup.getRenderable("partial-animated") as Box;
      if (partialAnimatedBox) {
        partialAnimatedBox.border = getBorderFromSides(borderSides);
        partialAnimatedBox.borderStyle = "single";
      }

      const partialPhaseText = tabGroup.getRenderable("partial-phase") as Text;
      if (partialPhaseText) {
        partialPhaseText.content = `Phase: ${phase + 1}/8`;
      }
    }
  }) as unknown as undefined;
}

// biome-ignore lint/correctness/noUnusedVariables: legacy reference implementation
function initAnimationTab(tabGroup: Box, _renderer: CliRenderer): void {
  let animPosition = 5;
  let animDirection = 1;
  const animSpeed = 15;

  const contentRow = new Box(_renderer, {
    id: "anim-content",
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    paddingX: 1,
    paddingY: 1,
    gap: 1,
  });
  tabGroup.add(contentRow);

  // Animation area
  const animArea = new Box(_renderer, {
    id: "anim-area",
    flexDirection: "column",
    flexGrow: 0,
    flexShrink: 0,
    width: 40,
    gap: 1,
  });
  contentRow.add(animArea);

  const animTitle = new Text(_renderer, {
    content: "Animation Demonstrations",
    fg: theme.accent1,
    zIndex: 10,
  });
  animArea.add(animTitle);

  const movingText = new Text(_renderer, {
    id: "moving-text",
    content: "Moving Text",
    position: "absolute",
    left: animPosition,
    top: 3,
    fg: theme.accent2,
    zIndex: 10,
  });
  tabGroup.add(movingText);

  const animatedBox = new Box(_renderer, {
    id: "animated-box",
    position: "absolute",
    left: animPosition,
    top: 5,
    width: 10,
    height: 3,
    backgroundColor: "#550055",
    borderStyle: "round",
    borderColor: "#FF00FF",
    border: true,
  });
  tabGroup.add(animatedBox);

  // Color box
  const colorCard = new Box(_renderer, {
    id: "color-card",
    flexDirection: "column",
    flexGrow: 1,
    flexShrink: 1,
    border: true,
    borderStyle: "double",
    borderColor: theme.textWhite,
    backgroundColor: "#550055",
    paddingX: 1,
    paddingY: 1,
    gap: 1,
  });
  contentRow.add(colorCard);

  const colorBoxTitle = new Text(_renderer, {
    content: "Animated Color",
    fg: theme.textWhite,
    zIndex: 10,
  });
  colorCard.add(colorBoxTitle);

  // biome-ignore lint/correctness/noVoidTypeReturn: returns frame callback for legacy callers
  return (() => {
    const deltaTime = Math.min((Date.now() % 1000) / 1000, 0.1);
    animPosition += animSpeed * animDirection * deltaTime;

    if (animPosition > 40) {
      animPosition = 40;
      animDirection = -1;
    } else if (animPosition < 5) {
      animPosition = 5;
      animDirection = 1;
    }

    const x = Math.round(animPosition);

    const movingTextEl = tabGroup.getRenderable("moving-text") as Text;
    if (movingTextEl) {
      movingTextEl.setPosition({ left: x, top: 3 });
    }

    const animatedBoxEl = tabGroup.getRenderable("animated-box") as Box;
    if (animatedBoxEl) {
      animatedBoxEl.setPosition({ left: x, top: 5 });
    }

    const time = Date.now() / 1000;
    const hue = (time * 30) % 360;
    const color = hsvToRgb(hue, 1, 0.7);
    const hexColor = rgbToHex(color);

    const colorCard = tabGroup.getRenderable("color-card") as Box;
    if (colorCard) {
      colorCard.backgroundColor = parseColor(hexColor);
    }
  }) as unknown as undefined;
}

// biome-ignore lint/correctness/noUnusedVariables: legacy reference implementation
function initTitlesTab(tabGroup: Box, _renderer: CliRenderer): void {
  const contentRow = new Box(_renderer, {
    id: "titles-content",
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    paddingX: 1,
    paddingY: 1,
    gap: 1,
    alignItems: "flex-start",
  });
  tabGroup.add(contentRow);

  const layoutTitle = new Text(_renderer, {
    content: "Box Titles",
    fg: theme.accent1,
    zIndex: 10,
  });
  contentRow.add(layoutTitle);

  const titledLeft = new Box(_renderer, {
    id: "titled-left",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    width: 20,
    height: 5,
    backgroundColor: "#222244",
    borderStyle: "single",
    borderColor: theme.textWhite,
    title: "Left Aligned",
    titleAlignment: "left",
    border: true,
  });
  contentRow.add(titledLeft);

  const titledCenter = new Box(_renderer, {
    id: "titled-center",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    width: 20,
    height: 5,
    backgroundColor: "#442222",
    borderStyle: "double",
    borderColor: theme.textWhite,
    title: "Centered Title",
    titleAlignment: "center",
    border: true,
  });
  contentRow.add(titledCenter);

  const titledRight = new Box(_renderer, {
    id: "titled-right",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    width: 20,
    height: 5,
    backgroundColor: "#224422",
    borderStyle: "round",
    borderColor: theme.textWhite,
    title: "Right Aligned",
    titleAlignment: "right",
    border: true,
  });
  contentRow.add(titledRight);
}

// biome-ignore lint/correctness/noUnusedVariables: legacy reference implementation
function initInteractiveTab(tabGroup: Box, _renderer: CliRenderer): void {
  const interactiveBorderSides = {
    top: true,
    right: true,
    bottom: true,
    left: true,
  };

  const contentRow = new Box(_renderer, {
    id: "interactive-content",
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    paddingX: 1,
    paddingY: 1,
    gap: 1,
  });
  tabGroup.add(contentRow);

  // Left: interactive border
  const leftCol = new Box(_renderer, {
    id: "interactive-left-col",
    flexDirection: "column",
    flexGrow: 0,
    flexShrink: 0,
    width: 30,
    gap: 1,
  });
  contentRow.add(leftCol);

  const interactiveTitle = new Text(_renderer, {
    content: "Interactive Controls",
    fg: theme.accent1,
    zIndex: 10,
  });
  leftCol.add(interactiveTitle);

  const interactiveBorder = new Box(_renderer, {
    id: "interactive-border",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    flexGrow: 0,
    flexShrink: 0,
    height: 8,
    backgroundColor: "#333344",
    borderStyle: "double",
    borderColor: theme.textWhite,
    border: true,
  });
  leftCol.add(interactiveBorder);

  const interactiveLabel = new Text(_renderer, {
    content: "Press keys to toggle borders",
    fg: theme.textWhite,
    zIndex: 10,
  });
  interactiveBorder.add(interactiveLabel);

  // Right: instructions
  const rightCol = new Box(_renderer, {
    id: "interactive-right-col",
    flexDirection: "column",
    flexGrow: 1,
    flexShrink: 1,
    gap: 1,
  });
  contentRow.add(rightCol);

  const interactiveInstructions = new Text(_renderer, {
    content: "Keyboard Controls:",
    fg: theme.textWhite,
    zIndex: 10,
  });
  rightCol.add(interactiveInstructions);

  const keyT = new Text(_renderer, {
    content: "T - Toggle top border",
    fg: theme.muted,
    zIndex: 10,
  });
  rightCol.add(keyT);

  const keyR = new Text(_renderer, {
    content: "R - Toggle right border",
    fg: theme.muted,
    zIndex: 10,
  });
  rightCol.add(keyR);

  const keyB = new Text(_renderer, {
    content: "B - Toggle bottom border",
    fg: theme.muted,
    zIndex: 10,
  });
  rightCol.add(keyB);

  const keyL = new Text(_renderer, {
    content: "L - Toggle left border",
    fg: theme.muted,
    zIndex: 10,
  });
  rightCol.add(keyL);

  const borderState = new Text(_renderer, {
    id: "border-state",
    content: "Active borders: All",
    fg: "#AAAAAA",
    zIndex: 10,
  });
  rightCol.add(borderState);

  // biome-ignore lint/correctness/noVoidTypeReturn: returns frame callback for legacy callers
  return (() => {
    const interactiveBorderEl = tabGroup.getRenderable("interactive-border") as Box;
    if (interactiveBorderEl) {
      interactiveBorderEl.border = getBorderFromSides(interactiveBorderSides);
    }

    let borderDesc = "";
    if (interactiveBorderSides.top) borderDesc += "Top ";
    if (interactiveBorderSides.right) borderDesc += "Right ";
    if (interactiveBorderSides.bottom) borderDesc += "Bottom ";
    if (interactiveBorderSides.left) borderDesc += "Left ";
    if (!borderDesc) borderDesc = "None";

    const borderStateEl = tabGroup.getRenderable("border-state") as Text;
    if (borderStateEl) {
      borderStateEl.content = `Active borders: ${borderDesc}`;
    }
  }) as unknown as undefined;
}

// ── Run ────────────────────────────────────────────────────────────────────────

export function run(renderer: CliRenderer): void {
  renderer.start();
  renderer.setBackgroundColor(theme.bg);

  globalScreen = new Screen(renderer, {
    id: "multitab-screen",
    backgroundColor: theme.bg,
    header: {
      id: "multitab-header",
      height: 1,
      backgroundColor: theme.headerBg,
      border: true,
      borderStyle: "single",
      borderColor: theme.cardBorder,
      alignItems: "center",
      justifyContent: "center",
    },
    body: {
      id: "multitab-body",
      flexDirection: "column",
    },
    footer: {
      id: "multitab-footer",
      height: 1,
      backgroundColor: theme.footerBg,
      border: true,
      borderStyle: "single",
      borderColor: theme.cardBorder,
      alignItems: "center",
      justifyContent: "center",
    },
  });

  const headerText = new Text(renderer, {
    id: "multitab-header-text",
    content: "BetterTUI Multi-Tab Demo — Use Left/Right arrows to navigate tabs",
    fg: theme.headerFg,
    bg: "transparent",
    zIndex: 1,
    flexGrow: 1,
    flexShrink: 1,
    textAlign: "center",
  });
  globalScreen.header?.add(headerText);

  const footerText = new Text(renderer, {
    id: "multitab-footer-text",
    content: "Left/Right: switch tabs | T/R/B/L: toggle borders (Interactive tab) | Ctrl+C: quit",
    fg: theme.footerFg,
    bg: "transparent",
    zIndex: 1,
    flexGrow: 1,
    flexShrink: 1,
    textAlign: "center",
  });
  globalScreen.footer?.add(footerText);

  globalTabController = new TabController("main-tabController", renderer, {
    id: "multitab-tab-controller",
    flexGrow: 1,
    flexShrink: 1,
  });
  globalScreen.body.add(globalTabController as unknown as Box);

  // Tab: Text & Attributes
  let activeWheelPixels = new Set<string>();

  globalTabController.addTab({
    title: "Text & Attributes",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;
      const textTitle = new Text(renderer, {
        id: "text-title",
        content: "Text Styling & Color Gradients",
        position: "absolute",
        left: 10,
        top: 5,
        fg: "#FFFF00",
        zIndex: 10,
      });
      g.add(textTitle);

      const attrBold = new Text(renderer, {
        id: "attr-bold",
        content: "Bold Text",
        position: "absolute",
        left: 10,
        top: 8,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(attrBold);

      const attrItalic = new Text(renderer, {
        id: "attr-italic",
        content: "Italic Text",
        position: "absolute",
        left: 10,
        top: 9,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(attrItalic);

      const attrUnderline = new Text(renderer, {
        id: "attr-underline",
        content: "Underlined Text",
        position: "absolute",
        left: 10,
        top: 10,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(attrUnderline);

      const attrDim = new Text(renderer, {
        id: "attr-dim",
        content: "Dim Text",
        position: "absolute",
        left: 10,
        top: 11,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(attrDim);

      const attrCombined = new Text(renderer, {
        id: "attr-combined",
        content: "Bold + Italic + Underline",
        position: "absolute",
        left: 10,
        top: 12,
        fg: "#FF6464",
        zIndex: 10,
      });
      g.add(attrCombined);

      const gradientTitle = new Text(renderer, {
        id: "gradient-title",
        content: "Rainbow Gradient:",
        position: "absolute",
        left: 10,
        top: 15,
        fg: "#CCCCCC",
        zIndex: 10,
      });
      g.add(gradientTitle);

      for (let i = 0; i < 40; i++) {
        const hue = (i / 40) * 360;
        const color = hsvToRgb(hue, 1, 1);
        const hexColor = rgbToHex(color);

        const gradientPixel = new Text(renderer, {
          id: `gradient-${i}`,
          content: "█",
          position: "absolute",
          left: 10 + i,
          top: 17,
          fg: hexColor,
          zIndex: 10,
        });
        g.add(gradientPixel);
      }
    },
    update: (_deltaMs: number, tabGroup: unknown) => {
      const g = tabGroup as Box;
      const time = Date.now() / 1000;
      const rotationSpeed = 45;
      const rotationAngle = (time * rotationSpeed) % 360;
      const rotationRadians = rotationAngle * (Math.PI / 180);

      const newWheelPixels = new Set<string>();

      for (let y = 15 - 7; y <= 15 + 7; y++) {
        for (let x = 70 - 14; x <= 70 + 14; x++) {
          const dx = (x - 70) / 2;
          const dy = y - 15;
          const distance = Math.sqrt(dx * dx + dy * dy);

          if (distance <= 7) {
            const angle = Math.atan2(dy, dx);
            const rotatedAngle = angle + rotationRadians;
            const hue = ((rotatedAngle / Math.PI) * 180 + 180) % 360;
            const saturation = distance / 7;
            const color = hsvToRgb(hue, saturation, 1);

            const pixelId = `wheel-${x}-${y}`;
            newWheelPixels.add(pixelId);

            const existingPixel = g.getRenderable(pixelId) as Text;
            if (existingPixel) {
              existingPixel.setPosition({ left: x, top: y });
              existingPixel.fg = color;
            } else {
              const wheelPixel = new Text(renderer, {
                id: pixelId,
                content: "█",
                position: "absolute",
                left: x,
                top: y,
                fg: color,
                zIndex: 10,
              });
              g.add(wheelPixel);
              activeWheelPixels.add(pixelId);
            }
          }
        }
      }

      for (const pixelId of activeWheelPixels) {
        if (!newWheelPixels.has(pixelId)) {
          const pixel = g.getRenderable(pixelId);
          if (pixel) g.remove(pixel);
          activeWheelPixels.delete(pixelId);
        }
      }

      activeWheelPixels = newWheelPixels;
    },
    show: () => {
      activeWheelPixels.clear();
    },
    hide: () => {
      for (const pixelId of activeWheelPixels) {
        const pixel = renderer.root.getRenderable(pixelId);
        if (pixel) renderer.root.remove(pixel);
      }
      activeWheelPixels.clear();
    },
  });

  // Tab: Basics
  globalTabController.addTab({
    title: "Basics",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;
      const title = new Text(renderer, {
        id: "bettertui-title",
        content: "Basic CLI Renderer Demo",
        position: "absolute",
        left: 10,
        top: 5,
        fg: "#FFFF00",
        zIndex: 10,
      });
      g.add(title);

      const box1 = new Box(renderer, {
        id: "box1",
        position: "absolute",
        left: 10,
        top: 8,
        width: 20,
        height: 8,
        backgroundColor: "#333366",
        zIndex: 0,
        borderStyle: "single",
        borderColor: "#FFFFFF",
        border: true,
      });
      g.add(box1);

      const box1Title = new Text(renderer, {
        id: "box1-title",
        content: "Simple Box",
        position: "absolute",
        left: 12,
        top: 10,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(box1Title);

      const box2 = new Box(renderer, {
        id: "box2",
        position: "absolute",
        left: 35,
        top: 10,
        width: 25,
        height: 6,
        backgroundColor: "#663333",
        zIndex: 1,
        borderStyle: "double",
        borderColor: "#FFFF00",
        border: true,
      });
      g.add(box2);

      const box2Title = new Text(renderer, {
        id: "box2-title",
        content: "Double Border Box",
        position: "absolute",
        left: 37,
        top: 12,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(box2Title);

      const description = new Text(renderer, {
        id: "description",
        content: "This tab demonstrates basic box and text rendering with different border styles.",
        position: "absolute",
        left: 10,
        top: 18,
        fg: "#CCCCCC",
        zIndex: 10,
      });
      g.add(description);

      const cursorInfo = new Text(renderer, {
        id: "cursor-info",
        content: "Cursor: (0,0) - Style: block",
        position: "absolute",
        left: 10,
        top: 20,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(cursorInfo);
    },
    update: (_deltaMs: number, tabGroup: unknown) => {
      const g = tabGroup as Box;
      const cursorTime = Date.now() / 1000;
      const cursorX = 15 + Math.floor(3 * Math.cos(cursorTime));
      const cursorY = 13 + Math.floor(2 * Math.sin(cursorTime));

      const cursorStyleIndex = Math.floor(cursorTime / 2) % 6;
      let cursorStyle: "block" | "line" | "underline" = "block";
      let cursorBlinking = false;

      switch (cursorStyleIndex) {
        case 0:
          cursorStyle = "block";
          cursorBlinking = false;
          break;
        case 1:
          cursorStyle = "block";
          cursorBlinking = true;
          break;
        case 2:
          cursorStyle = "line";
          cursorBlinking = false;
          break;
        case 3:
          cursorStyle = "line";
          cursorBlinking = true;
          break;
        case 4:
          cursorStyle = "underline";
          cursorBlinking = false;
          break;
        case 5:
          cursorStyle = "underline";
          cursorBlinking = true;
          break;
      }

      const cursorInfo = g.getRenderable("cursor-info") as Text;
      if (cursorInfo) {
        cursorInfo.content = `Cursor: (${cursorX},${cursorY}) - Style: ${cursorStyle}${cursorBlinking ? " (blinking)" : ""}`;
      }
    },
    show: () => {},
    hide: () => {},
  });

  // Tab: Borders
  let partialBorderPhase = 0;
  globalTabController.addTab({
    title: "Borders",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;
      const borderTitle = new Text(renderer, {
        id: "border-title",
        content: "Border Styles & Partial Borders",
        position: "absolute",
        left: 10,
        top: 5,
        fg: "#FFFF00",
        zIndex: 10,
      });
      g.add(borderTitle);

      const singleBox = new Box(renderer, {
        id: "single-box",
        position: "absolute",
        left: 10,
        top: 8,
        width: 15,
        height: 5,
        backgroundColor: "#222244",
        zIndex: 0,
        borderStyle: "single",
        borderColor: "#FFFFFF",
        border: true,
      });
      g.add(singleBox);
      const singleLabel = new Text(renderer, {
        id: "single-label",
        content: "Single",
        position: "absolute",
        left: 12,
        top: 10,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(singleLabel);

      const doubleBox = new Box(renderer, {
        id: "double-box",
        position: "absolute",
        left: 30,
        top: 8,
        width: 15,
        height: 5,
        backgroundColor: "#442222",
        zIndex: 0,
        borderStyle: "double",
        borderColor: "#FFFFFF",
        border: true,
      });
      g.add(doubleBox);
      const doubleLabel = new Text(renderer, {
        id: "double-label",
        content: "Double",
        position: "absolute",
        left: 32,
        top: 10,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(doubleLabel);

      const roundedBox = new Box(renderer, {
        id: "rounded-box",
        position: "absolute",
        left: 50,
        top: 8,
        width: 15,
        height: 5,
        backgroundColor: "#224422",
        zIndex: 0,
        borderStyle: "round",
        borderColor: "#FFFFFF",
        border: true,
      });
      g.add(roundedBox);
      const roundedLabel = new Text(renderer, {
        id: "rounded-label",
        content: "Rounded",
        position: "absolute",
        left: 52,
        top: 10,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(roundedLabel);

      const partialTitle = new Text(renderer, {
        id: "partial-title",
        content: "Partial Borders:",
        position: "absolute",
        left: 10,
        top: 15,
        fg: "#CCCCCC",
        zIndex: 10,
      });
      g.add(partialTitle);

      const partialLeft = new Box(renderer, {
        id: "partial-left",
        position: "absolute",
        left: 10,
        top: 17,
        width: 12,
        height: 4,
        backgroundColor: "#222244",
        zIndex: 0,
        borderStyle: "single",
        borderColor: "#FFFFFF",
        border: ["left"],
      });
      g.add(partialLeft);
      const partialLeftLabel = new Text(renderer, {
        id: "partial-left-label",
        content: "Left Only",
        position: "absolute",
        left: 12,
        top: 18,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(partialLeftLabel);

      const partialAnimated = new Box(renderer, {
        id: "partial-animated",
        position: "absolute",
        left: 30,
        top: 17,
        width: 20,
        height: 4,
        backgroundColor: "#334455",
        zIndex: 0,
        borderStyle: "single",
        borderColor: "#FFFFFF",
        border: true,
      });
      g.add(partialAnimated);
      const partialAnimatedLabel = new Text(renderer, {
        id: "partial-animated-label",
        content: "Animated Borders",
        position: "absolute",
        left: 32,
        top: 18,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(partialAnimatedLabel);

      const partialPhase = new Text(renderer, {
        id: "partial-phase",
        content: "Phase: 1/8",
        position: "absolute",
        left: 30,
        top: 22,
        fg: "#AAAAAA",
        zIndex: 10,
      });
      g.add(partialPhase);

      const customBorderTitle = new Text(renderer, {
        id: "custom-border-title",
        content: "Custom Border Characters:",
        position: "absolute",
        left: 10,
        top: 25,
        fg: "#CCCCCC",
        zIndex: 10,
      });
      g.add(customBorderTitle);

      const asciiBox = new Box(renderer, {
        id: "ascii-box",
        position: "absolute",
        left: 10,
        top: 27,
        width: 15,
        height: 5,
        backgroundColor: "#222244",
        zIndex: 0,
        borderStyle: "ascii",
        borderColor: "#FFFFFF",
        border: true,
      });
      g.add(asciiBox);
      const asciiLabel = new Text(renderer, {
        id: "ascii-label",
        content: "ASCII Border",
        position: "absolute",
        left: 12,
        top: 29,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(asciiLabel);

      const blockBox = new Box(renderer, {
        id: "block-box",
        position: "absolute",
        left: 30,
        top: 27,
        width: 15,
        height: 5,
        backgroundColor: "#442222",
        zIndex: 0,
        borderStyle: "thick",
        borderColor: "#FFFFFF",
        border: true,
      });
      g.add(blockBox);
      const blockLabel = new Text(renderer, {
        id: "block-label",
        content: "Block Border",
        position: "absolute",
        left: 32,
        top: 29,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(blockLabel);

      const starBox = new Box(renderer, {
        id: "star-box",
        position: "absolute",
        left: 50,
        top: 27,
        width: 15,
        height: 5,
        backgroundColor: "#224422",
        zIndex: 0,
        borderStyle: "dashed",
        borderColor: "#FFFFFF",
        border: true,
      });
      g.add(starBox);
      const starLabel = new Text(renderer, {
        id: "star-label",
        content: "Star Border",
        position: "absolute",
        left: 52,
        top: 29,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(starLabel);
    },
    update: (_deltaMs: number, tabGroup: unknown) => {
      const g = tabGroup as Box;
      const time = Date.now() / 1000;
      const phase = Math.floor(time % 8);

      if (phase !== partialBorderPhase) {
        partialBorderPhase = phase;

        const borderSides = {
          top: [0, 3, 5, 7].includes(phase),
          right: [1, 3, 6, 7].includes(phase),
          bottom: [2, 3, 5, 7].includes(phase),
          left: [4, 5, 6, 7].includes(phase),
        };

        const partialAnimatedBox = g.getRenderable("partial-animated") as Box;
        if (partialAnimatedBox) {
          partialAnimatedBox.border = getBorderFromSides(borderSides);
          partialAnimatedBox.borderStyle = "single";
        }

        const partialPhaseText = g.getRenderable("partial-phase") as Text;
        if (partialPhaseText) {
          partialPhaseText.content = `Phase: ${phase + 1}/8`;
        }
      }
    },
  });

  // Tab: Animation
  let animPosition = 5;
  let animDirection = 1;
  const animSpeed = 15;
  globalTabController.addTab({
    title: "Animation",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;
      const animTitle = new Text(renderer, {
        id: "anim-title",
        content: "Animation Demonstrations",
        position: "absolute",
        left: 10,
        top: 5,
        fg: "#FFFF00",
        zIndex: 10,
      });
      g.add(animTitle);

      const movingText = new Text(renderer, {
        id: "moving-text",
        content: "Moving Text",
        position: "absolute",
        left: animPosition,
        top: 8,
        fg: "#00FF00",
        zIndex: 10,
      });
      g.add(movingText);

      const animatedBox = new Box(renderer, {
        id: "animated-box",
        position: "absolute",
        left: animPosition,
        top: 10,
        width: 10,
        height: 3,
        backgroundColor: "#550055",
        zIndex: 0,
        borderStyle: "round",
        borderColor: "#FF00FF",
        border: true,
      });
      g.add(animatedBox);

      const colorBox = new Box(renderer, {
        id: "color-box",
        position: "absolute",
        left: 50,
        top: 12,
        width: 18,
        height: 5,
        backgroundColor: "#550055",
        zIndex: 0,
        borderStyle: "double",
        borderColor: "#FFFFFF",
        border: true,
      });
      g.add(colorBox);

      const colorBoxTitle = new Text(renderer, {
        id: "color-box-title",
        content: "Animated Color",
        position: "absolute",
        left: 52,
        top: 14,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(colorBoxTitle);
    },
    update: (deltaMs: number, tabGroup: unknown) => {
      const g = tabGroup as Box;
      const deltaTime = Math.min(deltaMs / 1000, 0.1);
      animPosition += animSpeed * animDirection * deltaTime;

      if (animPosition > 40) {
        animPosition = 40;
        animDirection = -1;
      } else if (animPosition < 5) {
        animPosition = 5;
        animDirection = 1;
      }

      const x = Math.round(animPosition);

      const movingText = g.getRenderable("moving-text") as Text;
      if (movingText) {
        movingText.setPosition({ left: x, top: 8 });
      }

      const animatedBox = g.getRenderable("animated-box") as Box;
      if (animatedBox) {
        animatedBox.setPosition({ left: x, top: 10 });
      }

      const time = Date.now() / 1000;
      const hue = (time * 30) % 360;
      const color = hsvToRgb(hue, 1, 0.7);
      const hexColor = rgbToHex(color);

      const colorBox = g.getRenderable("color-box") as Box;
      if (colorBox) {
        colorBox.backgroundColor = parseColor(hexColor);
      }
    },
  });

  // Tab: Titles
  globalTabController.addTab({
    title: "Titles",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;
      const layoutTitle = new Text(renderer, {
        id: "layout-title",
        content: "Box Titles",
        position: "absolute",
        left: 10,
        top: 5,
        fg: "#FFFF00",
        zIndex: 10,
      });
      g.add(layoutTitle);

      const titledLeft = new Box(renderer, {
        id: "titled-left",
        position: "absolute",
        left: 10,
        top: 8,
        width: 20,
        height: 5,
        backgroundColor: "#222244",
        zIndex: 0,
        borderStyle: "single",
        borderColor: "#FFFFFF",
        title: "Left Aligned",
        titleAlignment: "left",
        border: true,
      });
      g.add(titledLeft);

      const titledCenter = new Box(renderer, {
        id: "titled-center",
        position: "absolute",
        left: 35,
        top: 8,
        width: 20,
        height: 5,
        backgroundColor: "#442222",
        zIndex: 0,
        borderStyle: "double",
        borderColor: "#FFFFFF",
        title: "Centered Title",
        titleAlignment: "center",
        border: true,
      });
      g.add(titledCenter);

      const titledRight = new Box(renderer, {
        id: "titled-right",
        position: "absolute",
        left: 60,
        top: 8,
        width: 20,
        height: 5,
        backgroundColor: "#224422",
        zIndex: 0,
        borderStyle: "round",
        borderColor: "#FFFFFF",
        title: "Right Aligned",
        titleAlignment: "right",
        border: true,
      });
      g.add(titledRight);
    },
  });

  // Tab: Interactive
  const interactiveBorderSides = {
    top: true,
    right: true,
    bottom: true,
    left: true,
  };

  globalTabController.addTab({
    title: "Interactive",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;
      const interactiveTitle = new Text(renderer, {
        id: "interactive-title",
        content: "Interactive Controls",
        position: "absolute",
        left: 10,
        top: 5,
        fg: "#FFFF00",
        zIndex: 10,
      });
      g.add(interactiveTitle);

      const interactiveBorder = new Box(renderer, {
        id: "interactive-border",
        position: "absolute",
        left: 15,
        top: 8,
        width: 40,
        height: 8,
        backgroundColor: "#333344",
        zIndex: 0,
        borderStyle: "double",
        borderColor: "#FFFFFF",
        border: true,
      });
      g.add(interactiveBorder);

      const interactiveLabel = new Text(renderer, {
        id: "interactive-label",
        content: "Press keys to toggle borders",
        position: "absolute",
        left: 22,
        top: 12,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(interactiveLabel);

      const interactiveInstructions = new Text(renderer, {
        id: "interactive-instructions",
        content: "Keyboard Controls:",
        position: "absolute",
        left: 10,
        top: 18,
        fg: "#FFFFFF",
        zIndex: 10,
      });
      g.add(interactiveInstructions);

      const keyT = new Text(renderer, {
        id: "key-t",
        content: "T - Toggle top border",
        position: "absolute",
        left: 10,
        top: 19,
        fg: "#CCCCCC",
        zIndex: 10,
      });
      g.add(keyT);

      const keyR = new Text(renderer, {
        id: "key-r",
        content: "R - Toggle right border",
        position: "absolute",
        left: 10,
        top: 20,
        fg: "#CCCCCC",
        zIndex: 10,
      });
      g.add(keyR);

      const keyB = new Text(renderer, {
        id: "key-b",
        content: "B - Toggle bottom border",
        position: "absolute",
        left: 10,
        top: 21,
        fg: "#CCCCCC",
        zIndex: 10,
      });
      g.add(keyB);

      const keyL = new Text(renderer, {
        id: "key-l",
        content: "L - Toggle left border",
        position: "absolute",
        left: 10,
        top: 22,
        fg: "#CCCCCC",
        zIndex: 10,
      });
      g.add(keyL);

      const borderState = new Text(renderer, {
        id: "border-state",
        content: "Active borders: All",
        position: "absolute",
        left: 10,
        top: 24,
        fg: "#AAAAAA",
        zIndex: 10,
      });
      g.add(borderState);
    },
    update: (_deltaMs: number, tabGroup: unknown) => {
      const g = tabGroup as Box;
      const interactiveBorder = g.getRenderable("interactive-border") as Box;
      if (interactiveBorder) {
        interactiveBorder.border = getBorderFromSides(interactiveBorderSides);
      }

      let borderDesc = "";
      if (interactiveBorderSides.top) borderDesc += "Top ";
      if (interactiveBorderSides.right) borderDesc += "Right ";
      if (interactiveBorderSides.bottom) borderDesc += "Bottom ";
      if (interactiveBorderSides.left) borderDesc += "Left ";
      if (!borderDesc) borderDesc = "None";

      const borderState = g.getRenderable("border-state") as Text;
      if (borderState) {
        borderState.content = `Active borders: ${borderDesc}`;
      }
    },
  });

  globalTabController.focus();

  globalKeyboardHandler = (key: RawKeyEvent) => {
    if (globalTabController?.getCurrentTab().title === "Interactive") {
      if (key.name === "t" || key.name === "T") {
        interactiveBorderSides.top = !interactiveBorderSides.top;
      } else if (key.name === "r" || key.name === "R") {
        interactiveBorderSides.right = !interactiveBorderSides.right;
      } else if (key.name === "b" || key.name === "B") {
        interactiveBorderSides.bottom = !interactiveBorderSides.bottom;
      } else if (key.name === "l" || key.name === "L") {
        interactiveBorderSides.left = !interactiveBorderSides.left;
      }
    }
  };

  renderer.keyInput.on("keypress", globalKeyboardHandler);
}

// ── Cleanup ────────────────────────────────────────────────────────────────────

export function destroy(renderer: CliRenderer): void {
  renderer.clearFrameCallbacks();

  if (globalKeyboardHandler) {
    renderer.keyInput.off("keypress", globalKeyboardHandler);
    globalKeyboardHandler = null;
  }

  if (globalTabController) {
    globalScreen?.body.remove(globalTabController as unknown as Box);
    globalTabController = null;
  }

  if (globalScreen) {
    globalScreen.destroy();
    globalScreen = null;
  }
}

// ── Standalone entry ───────────────────────────────────────────────────────────

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
}
