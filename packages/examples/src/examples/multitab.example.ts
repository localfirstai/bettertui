import {
  type BorderSide,
  Box,
  RGBA,
  type RawKeyEvent,
  Text,
  createCliRenderer,
  parseColor,
} from "@bettertui/core";
import type { CliRenderer } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys.js";
import { TabController } from "../lib/tabController.js";

// ── Local stubs for missing @bettertui/core exports ────────────────────────

// biome-ignore lint/correctness/noUnusedVariables: stub for future API compatibility
interface BorderCharacters {
  topLeft: string;
  topRight: string;
  bottomLeft: string;
  bottomRight: string;
  horizontal: string;
  vertical: string;
  topT: string;
  bottomT: string;
  leftT: string;
  rightT: string;
  cross: string;
}

interface BorderSidesConfig {
  top: boolean;
  right: boolean;
  bottom: boolean;
  left: boolean;
}

function getBorderFromSides(sides: BorderSidesConfig): boolean | BorderSide[] {
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
// ────────────────────────────────────────────────────────────────────────────

let globalTabController: TabController | null = null;
let globalKeyboardHandler: ((key: RawKeyEvent) => void) | null = null;

export function run(renderer: CliRenderer): void {
  renderer.start();
  renderer.setBackgroundColor("#000028");

  const tabController = new TabController("main-tabController", renderer, {
    position: "absolute",
    left: 0,
    top: 0,
    width: renderer.terminalWidth,
    height: renderer.terminalHeight,
    zIndex: 0,
  } as never);
  globalTabController = tabController;
  renderer.root.add(tabController as unknown as Box);

  // Tab: Text & Attributes
  const wheelRadius = 7;
  const wheelCenterX = 70;
  const wheelCenterY = 15;
  let activeWheelPixels = new Set<string>();

  tabController.addTab({
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

      // Text attributes
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

      // Color gradient
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
      // Animate the rotating color wheel
      const time = Date.now() / 1000;
      const rotationSpeed = 45; // degrees per second
      const rotationAngle = (time * rotationSpeed) % 360;
      const rotationRadians = rotationAngle * (Math.PI / 180);

      // Track new wheel pixels for this frame
      const newWheelPixels = new Set<string>();

      for (let y = wheelCenterY - wheelRadius; y <= wheelCenterY + wheelRadius; y++) {
        for (let x = wheelCenterX - wheelRadius * 2; x <= wheelCenterX + wheelRadius * 2; x++) {
          const dx = (x - wheelCenterX) / 2; // Adjust for terminal character aspect ratio
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

      // Remove any wheel pixels that are no longer part of the wheel
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
  tabController.addTab({
    title: "Basics",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;
      const title = new Text(renderer, {
        id: "opentui-title",
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
      // Update cursor position (make it move in a small circle)
      const cursorTime = Date.now() / 1000;
      const cursorX = 15 + Math.floor(3 * Math.cos(cursorTime));
      const cursorY = 13 + Math.floor(2 * Math.sin(cursorTime));

      // Change cursor style every few seconds
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

      // setCursorStyle/setCursorPosition not available — just display info
      // Display cursor position and style info
      const cursorInfo = g.getRenderable("cursor-info") as Text;
      if (cursorInfo) {
        cursorInfo.content = `Cursor: (${cursorX},${cursorY}) - Style: ${cursorStyle}${cursorBlinking ? " (blinking)" : ""}`;
      }
    },
    show: () => {
      // setCursorPosition not available
    },
    hide: () => {
      // setCursorPosition not available
    },
  });

  // Tab: Borders
  let partialBorderPhase = 0;
  tabController.addTab({
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

      // Different border styles
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

      // Partial borders
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

      // Note: customBorderChars not available in BoxOptions — using default borders
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
      // Animate partial borders
      const time = Date.now() / 1000;
      const phase = Math.floor(time % 8);

      if (phase !== partialBorderPhase) {
        partialBorderPhase = phase;

        const borderSides: BorderSidesConfig = {
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
  tabController.addTab({
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
      // Animate moving elements
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

      // Animate color-changing box
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
  tabController.addTab({
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

      // Boxes with titles and different alignments
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

  tabController.addTab({
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
      // Update interactive border state
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

  tabController.focus();

  globalKeyboardHandler = (key: RawKeyEvent) => {
    // Interactive border controls (only active in Interactive tab)
    if (tabController.getCurrentTab().title === "Interactive") {
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

export function destroy(renderer: CliRenderer): void {
  renderer.clearFrameCallbacks();

  if (globalKeyboardHandler) {
    renderer.keyInput.off("keypress", globalKeyboardHandler);
    globalKeyboardHandler = null;
  }

  if (globalTabController) {
    renderer.root.remove(globalTabController as unknown as Box);
    globalTabController = null;
  }

  // setCursorPosition not available in this renderer implementation
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
}
