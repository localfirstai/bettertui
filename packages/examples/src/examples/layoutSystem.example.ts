/**
 * Layout System — interactive layout demos.
 *
 * Demonstrates flexbox-style layout primitives in BetterTUI (`@bettertui/core`):
 * row/column direction, grow/shrink/basis, alignment, justification, gaps,
 * padding/margin, absolute positioning, zIndex, and Box titles with borders.
 *
 * Panel fills use the Saha theme's bright semantic tokens (info, success,
 * warning, destructive, primary) — assigned per-layout so each demo has a
 * distinct, vibrant look.  The whole demo reacts live to `theme_mode` changes.
 *
 * Controls:
 *   SPACE  next demo       R      restart from first demo
 *   P      toggle autoplay V      toggle the moveable overlay
 *   WASD   move the moveable overlay
 */

import {
  Box,
  type CliRenderer,
  type KeyEvent,
  Text,
  bold,
  createCliRenderer,
  italic,
  t,
} from "@bettertui/core";
import type { ThemeMode } from "@bettertui/core";
import { DEFAULT_THEME_MODE, getThemeTokens } from "../constants/theme";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

interface LayoutDemo {
  name: string;
  description: string;
  setup: () => void;
}

/** Bright, saturated colours derived from the Saha theme's semantic tokens. */
interface DemoColors {
  blue: string;
  green: string;
  amber: string;
  red: string;
  brand: string;
  slate: string;

  textOnBright: string;
  textOnDark: string;

  appBg: string;
  footerBg: string;
  footerFg: string;
  border: string;

  moveableBg: string;
  moveableFg: string;
  moveableBorder: string;
  absoluteBg: string;
  absoluteFg: string;
  absoluteBorder: string;
}

let renderer: CliRenderer | null = null;
let header: Box | null = null;
let headerText: Text | null = null;
let contentArea: Box | null = null;
let sidebar: Box | null = null;
let sidebarText: Text | null = null;
let mainContent: Box | null = null;
let mainContentText: Text | null = null;
let rightSidebar: Box | null = null;
let rightSidebarText: Text | null = null;
let footer: Box | null = null;
let footerText: Text | null = null;
let moveableElement: Box | null = null;
let moveableText: Text | null = null;
let absolutePositionedBox: Box | null = null;
let absolutePositionedText: Text | null = null;
let currentDemoIndex = 0;
let autoAdvanceTimeout: ReturnType<typeof setTimeout> | null = null;
let autoplayEnabled = true;
let moveableElementVisible = true;
let moveableElementX = 0;
let moveableElementY = 0;
let colors: DemoColors;

function buildColors(mode: ThemeMode): DemoColors {
  const tokens = getThemeTokens(mode);
  return {
    blue: tokens.info,
    green: tokens.success,
    amber: tokens.warning,
    red: tokens.destructive,
    brand: tokens.primary,
    slate: tokens.border,

    textOnBright: tokens.background,
    textOnDark: tokens.foreground,

    appBg: tokens.background,
    footerBg: tokens.info,
    footerFg: tokens.background,
    border: tokens.border,

    moveableBg: tokens.destructive,
    moveableFg: tokens.destructiveForeground,
    moveableBorder: tokens.destructive,
    absoluteBg: tokens.info,
    absoluteFg: tokens.background,
    absoluteBorder: tokens.primary,
  };
}

function setupHorizontalLayout(): void {
  if (!contentArea || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = false;

  contentArea.setLayout({
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    alignItems: "stretch",
  });

  const sidebarWidth = Math.max(15, Math.floor((renderer?.terminalWidth ?? 80) * 0.2));
  sidebar.setLayout({
    width: sidebarWidth,
    flexGrow: 0,
    flexShrink: 0,
    minWidth: 15,
  });
  sidebar.backgroundColor = colors.slate;
  sidebar.title = "SIDEBAR";
  if (sidebarText) {
    sidebarText.content = "LEFT SIDEBAR";
    sidebarText.fg = colors.textOnDark;
  }

  mainContent.setLayout({ flexGrow: 1, flexShrink: 1, minWidth: 20 });
  mainContent.backgroundColor = colors.amber;
  mainContent.title = "CONTENT";
  if (mainContentText) {
    mainContentText.content = "MAIN CONTENT";
    mainContentText.fg = colors.textOnBright;
  }
}

function setupVerticalLayout(): void {
  if (!contentArea || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = false;

  contentArea.setLayout({
    flexDirection: "column",
    flexGrow: 1,
    flexShrink: 1,
    alignItems: "stretch",
  });

  const contentHeight = (renderer?.terminalHeight ?? 24) - 6;
  const topBarHeight = Math.max(3, Math.floor(contentHeight * 0.2));
  sidebar.setLayout({
    height: topBarHeight,
    flexGrow: 0,
    flexShrink: 0,
    minHeight: 3,
  });
  sidebar.backgroundColor = colors.green;
  sidebar.title = "TOP BAR";
  if (sidebarText) {
    sidebarText.content = "TOP BAR";
    sidebarText.fg = colors.textOnDark;
  }

  mainContent.setLayout({ flexGrow: 1, flexShrink: 1, minHeight: 5 });
  mainContent.backgroundColor = colors.amber;
  mainContent.title = "CONTENT";
  if (mainContentText) {
    mainContentText.content = "MAIN CONTENT";
    mainContentText.fg = colors.textOnBright;
  }
}

function setupCenteredLayout(): void {
  if (!contentArea || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = false;
  mainContent.visible = true;
  rightSidebar.visible = false;

  contentArea.setLayout({
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    alignItems: "stretch",
    justifyContent: "center",
  });

  const tw = renderer?.terminalWidth ?? 80;
  const centerWidth = Math.max(30, Math.floor(tw * 0.6));
  mainContent.setLayout({
    width: centerWidth,
    flexGrow: 0,
    flexShrink: 0,
    minWidth: 30,
    maxWidth: Math.floor(tw * 0.8),
  });
  mainContent.backgroundColor = colors.brand;
  mainContent.title = "CENTERED";
  if (mainContentText) {
    mainContentText.content = "CENTERED CONTENT";
    mainContentText.fg = colors.textOnDark;
  }
}

function setupThreeColumnLayout(): void {
  if (!contentArea || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = true;

  contentArea.setLayout({
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    alignItems: "stretch",
  });

  const sidebarWidth = Math.max(12, Math.floor((renderer?.terminalWidth ?? 80) * 0.15));

  sidebar.setLayout({
    width: sidebarWidth,
    flexGrow: 0,
    flexShrink: 0,
    minWidth: 12,
  });
  sidebar.backgroundColor = colors.red;
  sidebar.title = "LEFT";
  if (sidebarText) {
    sidebarText.content = "LEFT";
    sidebarText.fg = colors.textOnDark;
  }

  mainContent.setLayout({ flexGrow: 1, flexShrink: 1, minWidth: 20 });
  mainContent.backgroundColor = colors.green;
  mainContent.title = "CENTER";
  if (mainContentText) {
    mainContentText.content = "CENTER";
    mainContentText.fg = colors.textOnDark;
  }

  rightSidebar.setLayout({
    width: sidebarWidth,
    flexGrow: 0,
    flexShrink: 0,
    minWidth: 12,
  });
  rightSidebar.backgroundColor = colors.blue;
  rightSidebar.title = "RIGHT";
  if (rightSidebarText) {
    rightSidebarText.content = "RIGHT";
    rightSidebarText.fg = colors.textOnDark;
  }
}

function setupEqualColumnsLayout(): void {
  if (!contentArea || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = true;

  contentArea.setLayout({
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    alignItems: "stretch",
  });

  sidebar.setLayout({ flexGrow: 1, flexShrink: 1 });
  sidebar.backgroundColor = colors.blue;
  sidebar.title = "LEFT";
  if (sidebarText) {
    sidebarText.content = "LEFT";
    sidebarText.fg = colors.textOnDark;
  }

  mainContent.setLayout({ flexGrow: 1, flexShrink: 1 });
  mainContent.backgroundColor = colors.amber;
  mainContent.title = "CENTER";
  if (mainContentText) {
    mainContentText.content = "CENTER";
    mainContentText.fg = colors.textOnBright;
  }

  rightSidebar.setLayout({ flexGrow: 1, flexShrink: 1 });
  rightSidebar.backgroundColor = colors.green;
  rightSidebar.title = "RIGHT";
  if (rightSidebarText) {
    rightSidebarText.content = "RIGHT";
    rightSidebarText.fg = colors.textOnDark;
  }
}

function setupJustifyLayout(): void {
  if (!contentArea || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = true;

  contentArea.setLayout({
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    alignItems: "center",
    justifyContent: "space-between",
  });

  const sidebarWidth = Math.max(12, Math.floor((renderer?.terminalWidth ?? 80) * 0.12));

  sidebar.setLayout({
    width: sidebarWidth,
    flexGrow: 0,
    flexShrink: 0,
    minWidth: 12,
  });
  sidebar.backgroundColor = colors.brand;
  sidebar.title = "SIDE";
  if (sidebarText) {
    sidebarText.content = "LEFT";
    sidebarText.fg = colors.textOnDark;
  }

  mainContent.setLayout({ flexGrow: 1, flexShrink: 1, minWidth: 20 });
  mainContent.backgroundColor = colors.amber;
  mainContent.title = "MAIN";
  if (mainContentText) {
    mainContentText.content = "CENTER";
    mainContentText.fg = colors.textOnBright;
  }

  rightSidebar.setLayout({
    width: sidebarWidth,
    flexGrow: 0,
    flexShrink: 0,
    minWidth: 12,
  });
  rightSidebar.backgroundColor = colors.blue;
  rightSidebar.title = "SIDE";
  if (rightSidebarText) {
    rightSidebarText.content = "RIGHT";
    rightSidebarText.fg = colors.textOnDark;
  }
}

const layoutDemos: LayoutDemo[] = [
  {
    name: "Horizontal Layout",
    description: "Sidebar on left, main content on right",
    setup: setupHorizontalLayout,
  },
  {
    name: "Vertical Layout",
    description: "Top bar, main content fills the rest",
    setup: setupVerticalLayout,
  },
  {
    name: "Centered Layout",
    description: "Content centered with margins",
    setup: setupCenteredLayout,
  },
  {
    name: "Three Column",
    description: "Left sidebar, center content, right sidebar",
    setup: setupThreeColumnLayout,
  },
  {
    name: "Equal Columns",
    description: "Three equal-width columns (flexBasis: 0)",
    setup: setupEqualColumnsLayout,
  },
  {
    name: "Justify Space-Between",
    description: "justifyContent: space-between",
    setup: setupJustifyLayout,
  },
];

function applyColors(): void {
  if (!renderer) return;
  renderer.setBackgroundColor(colors.appBg);

  if (header) header.backgroundColor = colors.blue;
  if (headerText) headerText.fg = colors.textOnBright;

  if (footer) {
    footer.backgroundColor = colors.footerBg;
    footer.borderColor = colors.border;
  }
  if (footerText) footerText.fg = colors.footerFg;

  if (moveableElement) {
    moveableElement.backgroundColor = colors.moveableBg;
    moveableElement.borderColor = colors.moveableBorder;
  }
  if (moveableText) moveableText.fg = colors.moveableFg;

  if (absolutePositionedBox) {
    absolutePositionedBox.backgroundColor = colors.absoluteBg;
    absolutePositionedBox.borderColor = colors.absoluteBorder;
  }
  if (absolutePositionedText) absolutePositionedText.fg = colors.absoluteFg;
}

function createLayoutElements(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  const mode: ThemeMode = renderer.themeMode ?? DEFAULT_THEME_MODE;
  colors = buildColors(mode);
  renderer.setBackgroundColor(colors.appBg);

  header = new Box(renderer, {
    id: "header",
    zIndex: 0,
    width: "auto",
    height: 3,
    backgroundColor: colors.blue,
    borderStyle: "single",
    flexGrow: 0,
    flexShrink: 0,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    border: true,
  });

  headerText = new Text(renderer, {
    id: "header-text",
    content: "LAYOUT DEMO",
    fg: colors.textOnBright,
    bg: "transparent",
    zIndex: 1,
  });
  header.add(headerText);

  contentArea = new Box(renderer, {
    id: "content-area",
    zIndex: 0,
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
  });

  sidebar = new Box(renderer, {
    id: "sidebar",
    zIndex: 0,
    width: "auto",
    height: "auto",
    backgroundColor: colors.slate,
    borderStyle: "single",
    flexGrow: 0,
    flexShrink: 0,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    border: true,
    titleAlignment: "center",
  });

  sidebarText = new Text(renderer, {
    id: "sidebar-text",
    content: "SIDEBAR",
    fg: colors.textOnDark,
    bg: "transparent",
    zIndex: 1,
  });
  sidebar.add(sidebarText);

  mainContent = new Box(renderer, {
    id: "main-content",
    zIndex: 0,
    width: "auto",
    height: "auto",
    backgroundColor: colors.slate,
    borderStyle: "single",
    flexGrow: 1,
    flexShrink: 1,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    border: true,
    titleAlignment: "center",
  });

  mainContentText = new Text(renderer, {
    id: "main-content-text",
    content: "MAIN CONTENT",
    fg: colors.textOnDark,
    bg: "transparent",
    zIndex: 1,
  });
  mainContent.add(mainContentText);

  rightSidebar = new Box(renderer, {
    id: "right-sidebar",
    zIndex: 0,
    width: "auto",
    height: "auto",
    backgroundColor: colors.slate,
    borderStyle: "single",
    flexGrow: 0,
    flexShrink: 0,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    border: true,
    titleAlignment: "center",
  });

  rightSidebarText = new Text(renderer, {
    id: "right-sidebar-text",
    content: "RIGHT",
    fg: colors.textOnDark,
    bg: "transparent",
    zIndex: 1,
  });
  rightSidebar.add(rightSidebarText);

  footer = new Box(renderer, {
    id: "footer",
    zIndex: 0,
    width: "100%",
    height: 3,
    backgroundColor: colors.footerBg,
    borderStyle: "single",
    flexGrow: 0,
    flexShrink: 0,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    border: true,
  });

  footerText = new Text(renderer, {
    id: "footer-text",
    content: "",
    fg: colors.footerFg,
    bg: "transparent",
    zIndex: 1,
  });
  footer.add(footerText);

  moveableElement = new Box(renderer, {
    id: "moveable",
    zIndex: 100,
    width: 8,
    height: 3,
    backgroundColor: colors.moveableBg,
    borderStyle: "single",
    borderColor: colors.moveableBorder,
    position: "absolute",
    left: 0,
    top: 0,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    border: true,
  });

  moveableText = new Text(renderer, {
    id: "moveable-text",
    content: "MOVE",
    fg: colors.moveableFg,
    bg: "transparent",
    zIndex: 101,
  });
  moveableElement.add(moveableText);

  absolutePositionedBox = new Box(renderer, {
    id: "absolute-positioned-box",
    zIndex: 150,
    width: 20,
    height: 3,
    backgroundColor: colors.absoluteBg,
    borderStyle: "single",
    borderColor: colors.absoluteBorder,
    position: "absolute",
    bottom: 1,
    right: 1,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    border: true,
  });

  absolutePositionedText = new Text(renderer, {
    id: "absolute-positioned-text",
    content: "BOTTOM RIGHT",
    fg: colors.absoluteFg,
    bg: "transparent",
    zIndex: 151,
  });
  absolutePositionedBox.add(absolutePositionedText);

  contentArea.add(sidebar);
  contentArea.add(mainContent);
  contentArea.add(rightSidebar);
  rightSidebar.visible = false;

  renderer.root.add(header);
  renderer.root.add(contentArea);
  renderer.root.add(footer);
  renderer.root.add(moveableElement);
  renderer.root.add(absolutePositionedBox);

  centerMoveableElement();
  updateFooterText();
  renderer.on("resize", handleResize);
}

function handleResize(_width: number, _height: number): void {
  centerMoveableElement();
}

function handleKeyPress(key: KeyEvent): void {
  switch (key.name) {
    case "space":
      nextDemo();
      break;
    case "r":
      currentDemoIndex = 0;
      applyCurrentDemo();
      break;
    case "p":
      toggleAutoplay();
      break;
    case "v":
      toggleMoveableElement();
      break;
    case "w":
      moveMoveableElement(0, -1);
      break;
    case "a":
      moveMoveableElement(-1, 0);
      break;
    case "s":
      moveMoveableElement(0, 1);
      break;
    case "d":
      moveMoveableElement(1, 0);
      break;
  }
}

function nextDemo(): void {
  currentDemoIndex = (currentDemoIndex + 1) % layoutDemos.length;
  applyCurrentDemo();
}

function toggleAutoplay(): void {
  autoplayEnabled = !autoplayEnabled;

  if (autoplayEnabled) {
    if (autoAdvanceTimeout) clearTimeout(autoAdvanceTimeout);
    autoAdvanceTimeout = setTimeout(() => nextDemo(), 4000);
  } else {
    if (autoAdvanceTimeout) {
      clearTimeout(autoAdvanceTimeout);
      autoAdvanceTimeout = null;
    }
  }

  updateFooterText();
}

function toggleMoveableElement(): void {
  if (!moveableElement) return;

  moveableElementVisible = !moveableElementVisible;
  moveableElement.visible = moveableElementVisible;
  updateFooterText();
}

function moveMoveableElement(deltaX: number, deltaY: number): void {
  if (!moveableElement || !renderer) return;

  moveableElementX += deltaX;
  moveableElementY += deltaY;

  moveableElementX = Math.max(0, Math.min(renderer.terminalWidth - 8, moveableElementX));
  moveableElementY = Math.max(0, Math.min(renderer.terminalHeight - 3, moveableElementY));

  moveableElement.setPosition({
    left: moveableElementX,
    top: moveableElementY,
  });
}

function centerMoveableElement(): void {
  if (!renderer || !moveableElement) return;

  moveableElementX = Math.floor((renderer.terminalWidth - 8) / 2);
  moveableElementY = Math.floor((renderer.terminalHeight - 3) / 2);

  moveableElement.setPosition({
    left: moveableElementX,
    top: moveableElementY,
  });
}

function updateFooterText(): void {
  if (!footerText) return;
  const autoplayStatus = autoplayEnabled ? "ON" : "OFF";
  const moveableStatus = moveableElementVisible ? "ON" : "OFF";
  footerText.content = `SPACE: next | R: restart | P: autoplay (${autoplayStatus}) | V: overlay (${moveableStatus}) | WASD: move`;
}

function applyCurrentDemo(): void {
  const demo = layoutDemos[currentDemoIndex];
  if (!headerText) return;

  const autoplayStatus = autoplayEnabled ? "AUTO" : "MANUAL";
  headerText.content = t`${bold(`${demo.name}`)}  ${italic(`(${currentDemoIndex + 1}/${layoutDemos.length})`)}  —  ${demo.description}  [${autoplayStatus}]`;
  demo.setup();
  applyColors();
  updateFooterText();

  if (autoAdvanceTimeout) clearTimeout(autoAdvanceTimeout);

  if (autoplayEnabled) {
    autoAdvanceTimeout = setTimeout(() => nextDemo(), 4000);
  }
}

export function run(rendererInstance: CliRenderer): void {
  createLayoutElements(rendererInstance);
  rendererInstance.keyInput.on("keypress", handleKeyPress);
  rendererInstance.on("theme_mode", handleThemeMode);
  currentDemoIndex = 0;
  applyCurrentDemo();
}

function handleThemeMode(mode: ThemeMode): void {
  colors = buildColors(mode);
  applyCurrentDemo();
}

export function destroy(rendererInstance: CliRenderer): void {
  if (autoAdvanceTimeout) {
    clearTimeout(autoAdvanceTimeout);
    autoAdvanceTimeout = null;
  }

  rendererInstance.keyInput.off("keypress", handleKeyPress);
  rendererInstance.off("theme_mode", handleThemeMode);

  if (renderer) {
    renderer.off("resize", handleResize);
  }

  if (header) rendererInstance.root.remove(header);
  if (contentArea) rendererInstance.root.remove(contentArea);
  if (footer) rendererInstance.root.remove(footer);
  if (moveableElement) rendererInstance.root.remove(moveableElement);
  if (absolutePositionedBox) rendererInstance.root.remove(absolutePositionedBox);

  header = null;
  headerText = null;
  contentArea = null;
  sidebar = null;
  sidebarText = null;
  mainContent = null;
  mainContentText = null;
  rightSidebar = null;
  rightSidebarText = null;
  footer = null;
  footerText = null;
  moveableElement = null;
  moveableText = null;
  absolutePositionedBox = null;
  absolutePositionedText = null;
  renderer = null;
  currentDemoIndex = 0;
  moveableElementVisible = true;
  moveableElementX = 0;
  moveableElementY = 0;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
    targetFps: 30,
  });
  run(renderer);
  setupCommonDemoKeys(renderer);
}
