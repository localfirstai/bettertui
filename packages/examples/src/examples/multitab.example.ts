/**
 * Multi-Tab Demo — showcases Screen (Canvas.ts), TabController, flexbox layout,
 * text styling, border styles, and animations.
 *
 * All tab content uses the flexbox layout system (flexDirection / flexGrow /
 * alignItems / justifyContent / gap / padding) through the Screen body.
 * `position: "absolute"` is reserved for animated overlays that intentionally
 * escape the flex flow (the color wheel pixels and the moving animation elements).
 */
import {
  Box,
  type RawKeyEvent,
  Screen,
  Text,
  bold,
  createCliRenderer,
  dim,
  fg,
  italic,
  parseColor,
  t,
  underline,
} from "@bettertui/core";
import type { CliRenderer, ThemeMode } from "@bettertui/core";
import { DEFAULT_THEME_MODE, getComponentTheme, getThemeTokens } from "../constants/theme";
import { getBorderFromSides } from "../lib/borderSides";
import { hsvToRgb, rgbaToHex } from "../lib/colorUtils";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";
import { TabController } from "../lib/tabController";

// ── Module-level state ─────────────────────────────────────────────────────────

let globalScreen: Screen | null = null;
let globalTabController: TabController | null = null;
let globalKeyboardHandler: ((key: RawKeyEvent) => void) | null = null;

// ── Colors ──────────────────────────────────────────────────────────────────────

/** Semantic demo colors derived from the Saha theme tokens. */
interface DemoThemeColors {
  bg: string;
  headerBg: string;
  headerFg: string;
  footerBg: string;
  footerFg: string;
  cardBg: string;
  cardBorder: string;
  accent1: string;
  accent2: string;
  accent3: string;
  accent4: string;
  muted: string;
  textWhite: string;
}

/** Module-level colors, rebuilt from the active theme mode inside run(). */
let theme: DemoThemeColors = buildTheme(DEFAULT_THEME_MODE);

/** Map the Saha theme tokens onto this demo's semantic color roles. */
function buildTheme(mode: ThemeMode): DemoThemeColors {
  const tokens = getThemeTokens(mode);
  const comp = getComponentTheme(mode);
  return {
    bg: tokens.background,
    headerBg: tokens.secondary,
    headerFg: tokens.foreground,
    footerBg: tokens.muted,
    footerFg: tokens.mutedForeground,
    cardBg: tokens.secondary,
    cardBorder: comp.border,
    accent1: tokens.primary,
    accent2: tokens.success,
    accent3: tokens.destructive,
    accent4: tokens.info,
    muted: tokens.mutedForeground,
    textWhite: tokens.foreground,
  };
}

// ── Run ────────────────────────────────────────────────────────────────────────

export function run(renderer: CliRenderer): void {
  theme = buildTheme(renderer.themeMode ?? DEFAULT_THEME_MODE);

  renderer.start();
  renderer.setBackgroundColor(theme.bg);

  globalScreen = new Screen(renderer, {
    id: "multitab-screen",
    backgroundColor: theme.bg,
    header: {
      id: "multitab-header",
      height: 3,
      backgroundColor: theme.headerBg,
      border: true,
      borderStyle: "single",
      borderColor: theme.cardBorder,
      title: "BetterTUI Multi-Tab Demo",
      titleAlignment: "center",
      alignItems: "center",
      justifyContent: "center",
    },
    body: {
      id: "multitab-body",
      flexDirection: "column",
    },
    footer: {
      id: "multitab-footer",
      height: 3,
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
    content: "Use Left/Right arrows to navigate tabs",
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
    content: "Left/Right: switch tabs | T/R/B/L: toggle borders (Interactive) | Ctrl+C: quit",
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

  // ── Tab: Text & Attributes ──────────────────────────────────────────────────
  let activeWheelPixels = new Set<string>();

  globalTabController.addTab({
    title: "Text & Attrs",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;

      // Content row: two columns side by side
      const contentRow = new Box(renderer, {
        id: "text-content-row",
        flexDirection: "row",
        flexGrow: 1,
        flexShrink: 1,
        padding: 1,
        gap: 1,
      });
      g.add(contentRow);

      // Left column: text attributes card
      const attrCard = new Box(renderer, {
        id: "attr-card",
        flexDirection: "column",
        flexGrow: 0,
        flexShrink: 0,
        width: 28,
        border: true,
        borderStyle: "round",
        borderColor: theme.cardBorder,
        backgroundColor: theme.cardBg,
        title: "Attributes",
        titleAlignment: "center",
        padding: 1,
        gap: 1,
      });
      contentRow.add(attrCard);

      const attrBold = new Text(renderer, {
        id: "attr-bold",
        content: t`${bold("Bold Text")}`,
        fg: theme.textWhite,
        zIndex: 10,
      });
      attrCard.add(attrBold);

      const attrItalic = new Text(renderer, {
        id: "attr-italic",
        content: t`${italic("Italic Text")}`,
        fg: theme.textWhite,
        zIndex: 10,
      });
      attrCard.add(attrItalic);

      const attrUnderline = new Text(renderer, {
        id: "attr-underline",
        content: t`${underline("Underlined Text")}`,
        fg: theme.textWhite,
        zIndex: 10,
      });
      attrCard.add(attrUnderline);

      const attrDim = new Text(renderer, {
        id: "attr-dim",
        content: t`${dim("Dim Text")}`,
        fg: theme.textWhite,
        zIndex: 10,
      });
      attrCard.add(attrDim);

      const attrCombined = new Text(renderer, {
        id: "attr-combined",
        content: t`${bold(italic(underline("Bold + Italic + Underline")))}`,
        fg: theme.accent3,
        zIndex: 10,
      });
      attrCard.add(attrCombined);

      const attrColor = new Text(renderer, {
        id: "attr-color",
        content: t`${fg(theme.accent1)("Coloured Text")} ${fg(theme.accent2)("Green")} ${fg(theme.accent4)("Blue")}`,
        zIndex: 10,
      });
      attrCard.add(attrColor);

      // Right column: gradient + color wheel
      const rightCol = new Box(renderer, {
        id: "text-right-col",
        flexDirection: "column",
        flexGrow: 1,
        flexShrink: 1,
        gap: 1,
      });
      contentRow.add(rightCol);

      // Gradient card
      const gradientCard = new Box(renderer, {
        id: "gradient-card",
        flexDirection: "column",
        flexGrow: 0,
        flexShrink: 0,
        border: true,
        borderStyle: "single",
        borderColor: theme.cardBorder,
        backgroundColor: theme.cardBg,
        title: "Rainbow Gradient",
        titleAlignment: "center",
        padding: 1,
      });
      rightCol.add(gradientCard);

      const gradientRow = new Box(renderer, {
        id: "gradient-row",
        flexDirection: "row",
        height: 1,
      });
      gradientCard.add(gradientRow);

      for (let i = 0; i < 40; i++) {
        const hue = (i / 40) * 360;
        const hexColor = rgbaToHex(hsvToRgb(hue, 1, 1));
        const pixel = new Text(renderer, {
          id: `gradient-${i}`,
          content: "█",
          fg: hexColor,
          zIndex: 10,
        });
        gradientRow.add(pixel);
      }

      // Color wheel area (relative container for absolute-positioned pixels)
      const wheelArea = new Box(renderer, {
        id: "wheel-area",
        position: "relative",
        flexGrow: 1,
        flexShrink: 1,
        border: true,
        borderStyle: "single",
        borderColor: theme.cardBorder,
        backgroundColor: theme.cardBg,
        title: "Color Wheel",
        titleAlignment: "center",
      });
      rightCol.add(wheelArea);
    },
    update: (_deltaMs: number, tabGroup: unknown) => {
      const g = tabGroup as Box;
      const wheelArea = g.getRenderable("wheel-area") as Box;
      if (!wheelArea) return;

      const time = Date.now() / 1000;
      const rotationSpeed = 45;
      const rotationAngle = (time * rotationSpeed) % 360;
      const rotationRadians = rotationAngle * (Math.PI / 180);

      // Centre the wheel inside the wheel-area
      const wheelCenterX = 20;
      const wheelCenterY = 7;
      const wheelRadius = 6;

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

            const existingPixel = wheelArea.getRenderable(pixelId) as Text;
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
              wheelArea.add(wheelPixel);
              activeWheelPixels.add(pixelId);
            }
          }
        }
      }

      for (const pixelId of activeWheelPixels) {
        if (!newWheelPixels.has(pixelId)) {
          const pixel = wheelArea.getRenderable(pixelId);
          if (pixel) wheelArea.remove(pixel);
          activeWheelPixels.delete(pixelId);
        }
      }

      activeWheelPixels = newWheelPixels;
    },
    show: () => {
      activeWheelPixels.clear();
    },
    hide: () => {
      const wheelArea = globalTabController?.getCurrentTabGroup().getRenderable("wheel-area");
      for (const pixelId of activeWheelPixels) {
        const pixel = wheelArea?.getRenderable(pixelId);
        if (pixel) wheelArea?.remove(pixel);
      }
      activeWheelPixels.clear();
    },
  });

  // ── Tab: Basics ─────────────────────────────────────────────────────────────

  globalTabController.addTab({
    title: "Basics",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;

      const contentRow = new Box(renderer, {
        id: "basics-content-row",
        flexDirection: "row",
        flexGrow: 1,
        flexShrink: 1,
        padding: 1,
        gap: 1,
        alignItems: "stretch",
      });
      g.add(contentRow);

      // Simple box with single border
      const box1 = new Box(renderer, {
        id: "box1",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        flexGrow: 0,
        flexShrink: 0,
        width: 22,
        backgroundColor: theme.cardBg,
        borderStyle: "single",
        borderColor: theme.textWhite,
        title: "Single",
        titleAlignment: "center",
        border: true,
      });
      contentRow.add(box1);
      const box1Label = new Text(renderer, {
        content: "Simple Box",
        fg: theme.textWhite,
        zIndex: 10,
      });
      box1.add(box1Label);

      // Double border box
      const box2 = new Box(renderer, {
        id: "box2",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        flexGrow: 0,
        flexShrink: 0,
        width: 22,
        backgroundColor: theme.cardBg,
        borderStyle: "double",
        borderColor: theme.accent1,
        title: "Double",
        titleAlignment: "center",
        border: true,
      });
      contentRow.add(box2);
      const box2Label = new Text(renderer, {
        content: "Double Border",
        fg: theme.textWhite,
        zIndex: 10,
      });
      box2.add(box2Label);

      // Info card (fills remaining space)
      const infoCard = new Box(renderer, {
        id: "info-card",
        flexDirection: "column",
        flexGrow: 1,
        flexShrink: 1,
        border: true,
        borderStyle: "round",
        borderColor: theme.cardBorder,
        backgroundColor: theme.cardBg,
        title: "Info",
        titleAlignment: "center",
        padding: 1,
        gap: 1,
      });
      contentRow.add(infoCard);

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
    },
    update: (_deltaMs: number, tabGroup: unknown) => {
      const g = tabGroup as Box;
      const cursorTime = Date.now() / 1000;
      const cursorX = 15 + Math.floor(3 * Math.cos(cursorTime));
      const cursorY = 13 + Math.floor(2 * Math.sin(cursorTime));

      const cursorStyleIndex = Math.floor(cursorTime / 2) % 6;
      const cursorStyles: Array<"block" | "line" | "underline"> = [
        "block",
        "block",
        "line",
        "line",
        "underline",
        "underline",
      ];
      const cursorStyle = cursorStyles[cursorStyleIndex];
      const cursorBlinking = cursorStyleIndex % 2 === 1;

      const cursorInfo = g.getRenderable("cursor-info") as Text;
      if (cursorInfo) {
        cursorInfo.content = `Cursor: (${cursorX},${cursorY}) - Style: ${cursorStyle}${cursorBlinking ? " (blinking)" : ""}`;
      }
    },
  });

  // ── Tab: Borders ────────────────────────────────────────────────────────────

  let partialBorderPhase = 0;

  globalTabController.addTab({
    title: "Borders",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;

      const contentCol = new Box(renderer, {
        id: "borders-content-col",
        flexDirection: "column",
        flexGrow: 1,
        flexShrink: 1,
        padding: 1,
        gap: 1,
      });
      g.add(contentCol);

      // Row 1: standard border styles
      const stylesRow = new Box(renderer, {
        id: "borders-styles-row",
        flexDirection: "row",
        gap: 1,
      });
      contentCol.add(stylesRow);

      const makeBorderBox = (
        id: string,
        label: string,
        style: "single" | "double" | "round",
      ): Box => {
        const box = new Box(renderer, {
          id,
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          flexGrow: 1,
          flexShrink: 1,
          height: 5,
          backgroundColor: theme.cardBg,
          borderStyle: style,
          borderColor: theme.textWhite,
          title: label,
          titleAlignment: "center",
          border: true,
        });
        return box;
      };

      stylesRow.add(makeBorderBox("single-box", "Single", "single"));
      stylesRow.add(makeBorderBox("double-box", "Double", "double"));
      stylesRow.add(makeBorderBox("rounded-box", "Rounded", "round"));

      // Row 2: partial borders
      const partialRow = new Box(renderer, {
        id: "borders-partial-row",
        flexDirection: "row",
        gap: 1,
      });
      contentCol.add(partialRow);

      const partialLeft = new Box(renderer, {
        id: "partial-left",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        flexGrow: 1,
        flexShrink: 1,
        height: 5,
        backgroundColor: theme.cardBg,
        borderStyle: "single",
        borderColor: theme.textWhite,
        border: ["left"],
      });
      partialRow.add(partialLeft);
      const partialLeftLabel = new Text(renderer, {
        content: "Left Only",
        fg: theme.textWhite,
        zIndex: 10,
      });
      partialLeft.add(partialLeftLabel);

      const partialAnimated = new Box(renderer, {
        id: "partial-animated",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        flexGrow: 1,
        flexShrink: 1,
        height: 5,
        backgroundColor: theme.cardBg,
        borderStyle: "single",
        borderColor: theme.textWhite,
        title: "Animated",
        titleAlignment: "center",
        border: true,
      });
      partialRow.add(partialAnimated);
      const partialAnimatedLabel = new Text(renderer, {
        content: "Cycling Borders",
        fg: theme.textWhite,
        zIndex: 10,
      });
      partialAnimated.add(partialAnimatedLabel);

      const partialPhase = new Text(renderer, {
        id: "partial-phase",
        content: "Phase: 1/8",
        fg: theme.muted,
        zIndex: 10,
      });
      contentCol.add(partialPhase);

      // Row 3: custom border characters
      const customRow = new Box(renderer, {
        id: "borders-custom-row",
        flexDirection: "row",
        gap: 1,
      });
      contentCol.add(customRow);

      const makeCustomBox = (
        id: string,
        label: string,
        style: "ascii" | "thick" | "dashed",
      ): Box => {
        const box = new Box(renderer, {
          id,
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          flexGrow: 1,
          flexShrink: 1,
          height: 5,
          backgroundColor: theme.cardBg,
          borderStyle: style,
          borderColor: theme.textWhite,
          title: label,
          titleAlignment: "center",
          border: true,
        });
        return box;
      };

      customRow.add(makeCustomBox("ascii-box", "ASCII", "ascii"));
      customRow.add(makeCustomBox("thick-box", "Thick", "thick"));
      customRow.add(makeCustomBox("dashed-box", "Dashed", "dashed"));
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

  // ── Tab: Animation ──────────────────────────────────────────────────────────
  // Animated elements use position: "absolute" inside a position: "relative"
  // container — the valid overlay pattern for elements that move freely.

  let animPosition = 5;
  let animDirection = 1;
  const animSpeed = 15;

  globalTabController.addTab({
    title: "Animation",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;

      // Relative container — absolute children are positioned within it
      const animArea = new Box(renderer, {
        id: "anim-area",
        position: "relative",
        flexGrow: 1,
        flexShrink: 1,
        border: true,
        borderStyle: "round",
        borderColor: theme.cardBorder,
        backgroundColor: theme.cardBg,
        title: "Animation Playground",
        titleAlignment: "center",
      });
      g.add(animArea);

      const movingText = new Text(renderer, {
        id: "moving-text",
        content: "Moving Text",
        position: "absolute",
        left: animPosition,
        top: 3,
        fg: theme.accent2,
        zIndex: 10,
      });
      animArea.add(movingText);

      const animatedBox = new Box(renderer, {
        id: "animated-box",
        position: "absolute",
        left: animPosition,
        top: 6,
        width: 12,
        height: 3,
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        backgroundColor: theme.cardBg,
        borderStyle: "round",
        borderColor: theme.accent4,
        border: true,
      });
      animArea.add(animatedBox);
      const animatedBoxLabel = new Text(renderer, {
        content: "Box",
        fg: theme.textWhite,
        zIndex: 10,
      });
      animatedBox.add(animatedBoxLabel);

      const colorBox = new Box(renderer, {
        id: "color-box",
        position: "absolute",
        right: 2,
        top: 2,
        width: 20,
        height: 5,
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        backgroundColor: theme.cardBg,
        borderStyle: "double",
        borderColor: theme.textWhite,
        title: "Animated Color",
        titleAlignment: "center",
        border: true,
      });
      animArea.add(colorBox);
    },
    update: (deltaMs: number, tabGroup: unknown) => {
      const g = tabGroup as Box;
      const animArea = g.getRenderable("anim-area") as Box;
      if (!animArea) return;

      const deltaTime = Math.min(deltaMs / 1000, 0.1);
      animPosition += animSpeed * animDirection * deltaTime;

      const maxX = Math.max(20, (animArea.width as number) - 14);
      if (animPosition > maxX) {
        animPosition = maxX;
        animDirection = -1;
      } else if (animPosition < 2) {
        animPosition = 2;
        animDirection = 1;
      }

      const x = Math.round(animPosition);

      const movingText = animArea.getRenderable("moving-text") as Text;
      if (movingText) {
        movingText.setPosition({ left: x, top: 3 });
      }

      const animatedBox = animArea.getRenderable("animated-box") as Box;
      if (animatedBox) {
        animatedBox.setPosition({ left: x, top: 6 });
      }

      const time = Date.now() / 1000;
      const hue = (time * 30) % 360;
      const hexColor = rgbaToHex(hsvToRgb(hue, 1, 0.7));

      const colorBox = animArea.getRenderable("color-box") as Box;
      if (colorBox) {
        colorBox.backgroundColor = parseColor(hexColor);
      }
    },
  });

  // ── Tab: Titles ─────────────────────────────────────────────────────────────

  globalTabController.addTab({
    title: "Titles",
    init: (tabGroup: unknown) => {
      const g = tabGroup as Box;

      const contentCol = new Box(renderer, {
        id: "titles-content-col",
        flexDirection: "column",
        flexGrow: 1,
        flexShrink: 1,
        padding: 2,
        gap: 1,
        alignItems: "center",
        justifyContent: "center",
      });
      g.add(contentCol);

      const heading = new Text(renderer, {
        id: "titles-heading",
        content: "Box titles can be aligned left, centre, or right.",
        fg: theme.muted,
        zIndex: 10,
      });
      contentCol.add(heading);

      const row = new Box(renderer, {
        id: "titles-row",
        flexDirection: "row",
        gap: 2,
        alignItems: "stretch",
        justifyContent: "center",
      });
      contentCol.add(row);

      const makeTitledBox = (
        id: string,
        borderStyle: "single" | "double" | "round",
        title: string,
        titleAlignment: "left" | "center" | "right",
      ): Box => {
        const box = new Box(renderer, {
          id,
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          width: 22,
          height: 5,
          backgroundColor: theme.cardBg,
          borderStyle,
          borderColor: theme.textWhite,
          title,
          titleAlignment,
          border: true,
        });
        return box;
      };

      const left = makeTitledBox("titled-left", "single", "Left Aligned", "left");
      row.add(left);
      const leftLabel = new Text(renderer, {
        content: "Single",
        fg: theme.textWhite,
        zIndex: 10,
      });
      left.add(leftLabel);

      const center = makeTitledBox("titled-center", "double", "Centered Title", "center");
      row.add(center);
      const centerLabel = new Text(renderer, {
        content: "Double",
        fg: theme.textWhite,
        zIndex: 10,
      });
      center.add(centerLabel);

      const right = makeTitledBox("titled-right", "round", "Right Aligned", "right");
      row.add(right);
      const rightLabel = new Text(renderer, {
        content: "Rounded",
        fg: theme.textWhite,
        zIndex: 10,
      });
      right.add(rightLabel);
    },
  });

  // ── Tab: Interactive ────────────────────────────────────────────────────────

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

      const contentRow = new Box(renderer, {
        id: "interactive-content-row",
        flexDirection: "row",
        flexGrow: 1,
        flexShrink: 1,
        padding: 1,
        gap: 1,
        alignItems: "flex-start",
      });
      g.add(contentRow);

      // Left: the interactive border box
      const leftCol = new Box(renderer, {
        id: "interactive-left-col",
        flexDirection: "column",
        flexGrow: 1,
        flexShrink: 1,
        gap: 1,
      });
      contentRow.add(leftCol);

      const interactiveBorder = new Box(renderer, {
        id: "interactive-border",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        flexGrow: 1,
        backgroundColor: theme.cardBg,
        borderStyle: "double",
        borderColor: theme.textWhite,
        border: true,
      });
      leftCol.add(interactiveBorder);

      const interactiveLabel = new Text(renderer, {
        content: "Press T / R / B / L to toggle borders",
        fg: theme.textWhite,
        zIndex: 10,
        textAlign: "center",
      });
      interactiveBorder.add(interactiveLabel);

      // Right: keyboard instructions
      const rightCol = new Box(renderer, {
        id: "interactive-right-col",
        flexDirection: "column",
        flexGrow: 0,
        flexShrink: 0,
        width: 32,
        gap: 1,
        padding: 1,
        border: true,
        borderStyle: "round",
        borderColor: theme.cardBorder,
        backgroundColor: theme.cardBg,
        title: "Controls",
        titleAlignment: "center",
      });
      contentRow.add(rightCol);

      const instructions: Array<[string, string]> = [
        ["T", "Toggle top border"],
        ["R", "Toggle right border"],
        ["B", "Toggle bottom border"],
        ["L", "Toggle left border"],
      ];

      for (const [key, desc] of instructions) {
        const line = new Text(renderer, {
          id: `key-${key.toLowerCase()}`,
          content: t`${bold(fg(theme.accent1)(key))} — ${desc}`,
          zIndex: 10,
        });
        rightCol.add(line);
      }

      const borderState = new Text(renderer, {
        id: "border-state",
        content: "Active borders: All",
        fg: theme.muted,
        zIndex: 10,
      });
      rightCol.add(borderState);
    },
    update: (_deltaMs: number, tabGroup: unknown) => {
      const g = tabGroup as Box;
      const interactiveBorderEl = g.getRenderable("interactive-border") as Box;
      if (interactiveBorderEl) {
        interactiveBorderEl.border = getBorderFromSides(interactiveBorderSides);
      }

      let borderDesc = "";
      if (interactiveBorderSides.top) borderDesc += "Top ";
      if (interactiveBorderSides.right) borderDesc += "Right ";
      if (interactiveBorderSides.bottom) borderDesc += "Bottom ";
      if (interactiveBorderSides.left) borderDesc += "Left ";
      if (!borderDesc) borderDesc = "None";

      const borderStateEl = g.getRenderable("border-state") as Text;
      if (borderStateEl) {
        borderStateEl.content = `Active borders: ${borderDesc}`;
      }
    },
  });

  globalTabController.focus();

  globalKeyboardHandler = (key: RawKeyEvent) => {
    if (globalTabController?.getCurrentTab().title === "Interactive") {
      if (key.name === "t") {
        interactiveBorderSides.top = !interactiveBorderSides.top;
      } else if (key.name === "r") {
        interactiveBorderSides.right = !interactiveBorderSides.right;
      } else if (key.name === "b") {
        interactiveBorderSides.bottom = !interactiveBorderSides.bottom;
      } else if (key.name === "l") {
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
