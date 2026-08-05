/**
 * Layout System — interactive set of layout demos.
 *
 * Demonstrates flexbox-style layout primitives in BetterTUI (`@bettertui/core`):
 * row/column direction, grow/shrink/basis, alignment, justification, gaps,
 * padding/margin, absolute positioning, zIndex, and Box titles with borders.
 *
 * The whole demo is theme-aware: every colour is derived from the Saha theme
 * tokens (`src/constants/theme.ts`) for the current `renderer.themeMode`, and
 * reacts live to `theme_mode` changes (like the menu does).
 *
 * Controls:
 *   SPACE  next demo
 *   R      restart from first demo
 *   P      toggle autoplay
 *   V      toggle the moveable overlay
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
import { DEFAULT_THEME_MODE, getComponentTheme, getThemeTokens } from "../constants/theme";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

interface LayoutDemo {
  name: string;
  description: string;
  setup: () => void;
}

interface LayoutPalette {
  background: string;
  headerBg: string;
  headerBorder: string;
  headerTitle: string;
  headerFg: string;
  panelFg: string;
  panelMuted: string;
  sidebarBg: string;
  sidebarBorder: string;
  sidebarTitle: string;
  mainBg: string;
  mainBorder: string;
  mainTitle: string;
  rightBg: string;
  rightBorder: string;
  rightTitle: string;
  primary: string;
  primaryFg: string;
  moveableBorder: string;
  absoluteBg: string;
  absoluteBorder: string;
  footerBg: string;
  footerFg: string;
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
let palette: LayoutPalette;

/** Build the semantic colour set for the given theme mode. */
function buildPalette(mode: ThemeMode): LayoutPalette {
  const tokens = getThemeTokens(mode);
  const components = getComponentTheme(mode);
  return {
    background: components.appBackground,
    headerBg: tokens.secondary,
    headerBorder: tokens.border,
    headerTitle: tokens.foreground,
    headerFg: tokens.secondaryForeground,
    panelFg: tokens.foreground,
    panelMuted: tokens.mutedForeground,
    sidebarBg: tokens.secondary,
    sidebarBorder: tokens.info,
    sidebarTitle: tokens.info,
    mainBg: tokens.muted,
    mainBorder: tokens.success,
    mainTitle: tokens.success,
    rightBg: tokens.accent,
    rightBorder: tokens.warning,
    rightTitle: tokens.warning,
    primary: tokens.primary,
    primaryFg: tokens.primaryForeground,
    moveableBorder: tokens.ring,
    absoluteBg: tokens.info,
    absoluteBorder: tokens.primary,
    footerBg: tokens.secondary,
    footerFg: tokens.mutedForeground,
  };
}

function resetElementLayout(element: Box): void {
  element.flexBasis = "auto";
  element.flexGrow = 0;
  element.width = "auto";
  element.height = "auto";
  element.setLayout({
    flexShrink: 0,
    minWidth: undefined,
    maxWidth: undefined,
    minHeight: undefined,
    maxHeight: undefined,
  });
}

function setupHorizontalLayout(): void {
  if (!contentArea || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = false;

  resetElementLayout(sidebar);
  resetElementLayout(mainContent);

  contentArea.flexDirection = "row";
  contentArea.setLayout({ alignItems: "stretch", gap: 1 });

  const sidebarWidth = Math.max(15, Math.floor((renderer?.terminalWidth ?? 80) * 0.2));
  sidebar.flexBasis = sidebarWidth;
  sidebar.flexGrow = 0;
  sidebar.setLayout({ flexShrink: 0, minWidth: 15 });
  sidebar.width = sidebarWidth;
  sidebar.height = "auto";
  sidebar.title = "SIDEBAR";
  if (sidebarText) sidebarText.content = "LEFT SIDEBAR";

  mainContent.flexBasis = "auto";
  mainContent.flexGrow = 1;
  mainContent.setLayout({ flexShrink: 1, minWidth: 20 });
  mainContent.width = "auto";
  mainContent.height = "auto";
  mainContent.title = "CONTENT";
  if (mainContentText) mainContentText.content = "MAIN CONTENT";
}

function setupVerticalLayout(): void {
  if (!contentArea || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = false;

  resetElementLayout(sidebar);
  resetElementLayout(mainContent);

  contentArea.flexDirection = "column";
  contentArea.setLayout({ alignItems: "stretch", gap: 1 });

  const contentHeight = (renderer?.terminalHeight ?? 24) - 6;
  const topBarHeight = Math.max(3, Math.floor(contentHeight * 0.2));
  sidebar.flexBasis = topBarHeight;
  sidebar.flexGrow = 0;
  sidebar.setLayout({ flexShrink: 0, minHeight: 3 });
  sidebar.height = topBarHeight;
  sidebar.width = "auto";
  sidebar.title = "TOP BAR";
  if (sidebarText) sidebarText.content = "TOP BAR";

  mainContent.flexBasis = "auto";
  mainContent.flexGrow = 1;
  mainContent.setLayout({ flexShrink: 1, minHeight: 5 });
  mainContent.height = "auto";
  mainContent.width = "auto";
  mainContent.title = "CONTENT";
  if (mainContentText) mainContentText.content = "MAIN CONTENT";
}

function setupCenteredLayout(): void {
  if (!contentArea || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = false;
  mainContent.visible = true;
  rightSidebar.visible = false;

  resetElementLayout(mainContent);

  contentArea.flexDirection = "row";
  contentArea.setLayout({ alignItems: "stretch", justifyContent: "center" });

  const centerWidth = Math.max(30, Math.floor((renderer?.terminalWidth ?? 80) * 0.6));
  mainContent.flexBasis = centerWidth;
  mainContent.flexGrow = 0;
  mainContent.setLayout({
    flexShrink: 0,
    minWidth: 30,
    maxWidth: Math.floor((renderer?.terminalWidth ?? 80) * 0.8),
  });
  mainContent.width = centerWidth;
  mainContent.height = "auto";
  mainContent.title = "CENTERED";
  if (mainContentText) mainContentText.content = "CENTERED CONTENT";
}

function setupThreeColumnLayout(): void {
  if (!contentArea || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = true;

  resetElementLayout(sidebar);
  resetElementLayout(mainContent);
  resetElementLayout(rightSidebar);

  contentArea.flexDirection = "row";
  contentArea.setLayout({ alignItems: "stretch", gap: 1 });

  const terminalWidth = renderer?.terminalWidth ?? 80;
  const sidebarWidth = Math.max(12, Math.floor(terminalWidth * 0.15));

  sidebar.flexBasis = sidebarWidth;
  sidebar.flexGrow = 0;
  sidebar.setLayout({ flexShrink: 0, minWidth: 12 });
  sidebar.width = sidebarWidth;
  sidebar.height = "auto";
  sidebar.title = "LEFT";
  if (sidebarText) sidebarText.content = "LEFT";

  mainContent.flexBasis = "auto";
  mainContent.flexGrow = 1;
  mainContent.setLayout({ flexShrink: 1, minWidth: 20 });
  mainContent.width = "auto";
  mainContent.height = "auto";
  mainContent.title = "CENTER";
  if (mainContentText) mainContentText.content = "CENTER";

  rightSidebar.flexBasis = sidebarWidth;
  rightSidebar.flexGrow = 0;
  rightSidebar.setLayout({ flexShrink: 0, minWidth: 12 });
  rightSidebar.width = sidebarWidth;
  rightSidebar.height = "auto";
  rightSidebar.title = "RIGHT";
  if (rightSidebarText) rightSidebarText.content = "RIGHT";
}

function setupJustifyLayout(): void {
  if (!contentArea || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = true;

  resetElementLayout(sidebar);
  resetElementLayout(mainContent);
  resetElementLayout(rightSidebar);

  contentArea.flexDirection = "row";
  contentArea.setLayout({
    alignItems: "center",
    justifyContent: "space-between",
    gap: 1,
  });

  const sidebarWidth = Math.max(12, Math.floor((renderer?.terminalWidth ?? 80) * 0.12));

  sidebar.flexBasis = sidebarWidth;
  sidebar.flexGrow = 0;
  sidebar.setLayout({ flexShrink: 0, minWidth: 12 });
  sidebar.width = sidebarWidth;
  sidebar.height = "auto";
  sidebar.title = "SIDE";
  if (sidebarText) sidebarText.content = "LEFT";

  mainContent.flexBasis = "auto";
  mainContent.flexGrow = 1;
  mainContent.setLayout({ flexShrink: 1, minWidth: 20 });
  mainContent.width = "auto";
  mainContent.height = "auto";
  mainContent.title = "MAIN";
  if (mainContentText) mainContentText.content = "CENTER";

  rightSidebar.flexBasis = sidebarWidth;
  rightSidebar.flexGrow = 0;
  rightSidebar.setLayout({ flexShrink: 0, minWidth: 12 });
  rightSidebar.width = sidebarWidth;
  rightSidebar.height = "auto";
  rightSidebar.title = "SIDE";
  if (rightSidebarText) rightSidebarText.content = "RIGHT";
}

const layoutDemos: LayoutDemo[] = [
  {
    name: "Horizontal Layout",
    description: "Sidebar on left, main content on right",
    setup: setupHorizontalLayout,
  },
  {
    name: "Vertical Layout",
    description: "Sidebar on top, main content below",
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
    name: "Justify Space-Between",
    description: "justifyContent: space-between with a gap between columns",
    setup: setupJustifyLayout,
  },
];

/** Apply the semantic colour set to every static element. */
function applyColors(): void {
  if (!renderer) return;
  renderer.setBackgroundColor(palette.background);

  if (header) {
    header.backgroundColor = palette.headerBg;
    header.borderColor = palette.headerBorder;
  }
  if (headerText) headerText.fg = palette.headerFg;

  if (sidebar) {
    sidebar.backgroundColor = palette.sidebarBg;
    sidebar.borderColor = palette.sidebarBorder;
  }
  if (sidebarText) sidebarText.fg = palette.panelFg;

  if (mainContent) {
    mainContent.backgroundColor = palette.mainBg;
    mainContent.borderColor = palette.mainBorder;
  }
  if (mainContentText) mainContentText.fg = palette.panelMuted;

  if (rightSidebar) {
    rightSidebar.backgroundColor = palette.rightBg;
    rightSidebar.borderColor = palette.rightBorder;
  }
  if (rightSidebarText) rightSidebarText.fg = palette.panelFg;

  if (footer) {
    footer.backgroundColor = palette.footerBg;
    footer.borderColor = palette.headerBorder;
  }
  if (footerText) footerText.fg = palette.footerFg;

  if (moveableElement) moveableElement.borderColor = palette.moveableBorder;
  if (absolutePositionedBox) {
    absolutePositionedBox.backgroundColor = palette.absoluteBg;
    absolutePositionedBox.borderColor = palette.absoluteBorder;
  }
}

function createLayoutElements(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  const mode: ThemeMode = renderer.themeMode ?? DEFAULT_THEME_MODE;
  palette = buildPalette(mode);

  header = new Box(renderer, {
    id: "header",
    zIndex: 0,
    width: "auto",
    height: 4,
    backgroundColor: palette.headerBg,
    borderStyle: "thick",
    alignItems: "center",
    justifyContent: "center",
    border: true,
    title: "LAYOUT SYSTEM",
    titleAlignment: "center",
    titleColor: palette.headerTitle,
  });

  headerText = new Text(renderer, {
    id: "header-text",
    content: "",
    fg: palette.headerFg,
    bg: "transparent",
    zIndex: 1,
  });
  header.add(headerText);

  contentArea = new Box(renderer, {
    id: "content-area",
    zIndex: 0,
    width: "auto",
    height: "auto",
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    gap: 1,
  });

  sidebar = new Box(renderer, {
    id: "sidebar",
    zIndex: 0,
    width: "auto",
    height: "auto",
    backgroundColor: palette.sidebarBg,
    borderStyle: "round",
    flexGrow: 0,
    flexShrink: 0,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    border: true,
    title: "SIDEBAR",
    titleAlignment: "center",
    titleColor: palette.sidebarTitle,
  });

  sidebarText = new Text(renderer, {
    id: "sidebar-text",
    content: "SIDEBAR",
    fg: palette.panelFg,
    bg: "transparent",
    zIndex: 1,
  });
  sidebar.add(sidebarText);

  mainContent = new Box(renderer, {
    id: "main-content",
    zIndex: 0,
    width: "auto",
    height: "auto",
    backgroundColor: palette.mainBg,
    borderStyle: "round",
    flexGrow: 1,
    flexShrink: 1,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    border: true,
    title: "CONTENT",
    titleAlignment: "center",
    titleColor: palette.mainTitle,
  });

  mainContentText = new Text(renderer, {
    id: "main-content-text",
    content: "MAIN CONTENT",
    fg: palette.panelMuted,
    bg: "transparent",
    zIndex: 1,
  });
  mainContent.add(mainContentText);

  rightSidebar = new Box(renderer, {
    id: "right-sidebar",
    zIndex: 0,
    width: "auto",
    height: "auto",
    backgroundColor: palette.rightBg,
    borderStyle: "round",
    flexGrow: 0,
    flexShrink: 0,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    border: true,
    title: "RIGHT",
    titleAlignment: "center",
    titleColor: palette.rightTitle,
  });

  rightSidebarText = new Text(renderer, {
    id: "right-sidebar-text",
    content: "RIGHT",
    fg: palette.panelFg,
    bg: "transparent",
    zIndex: 1,
  });
  rightSidebar.add(rightSidebarText);

  footer = new Box(renderer, {
    id: "footer",
    zIndex: 0,
    width: "auto",
    height: 3,
    backgroundColor: palette.footerBg,
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
    fg: palette.footerFg,
    bg: "transparent",
    zIndex: 1,
  });
  footer.add(footerText);

  moveableElement = new Box(renderer, {
    id: "moveable",
    zIndex: 100,
    width: 8,
    height: 3,
    backgroundColor: palette.primary,
    borderStyle: "single",
    borderColor: palette.moveableBorder,
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
    fg: palette.primaryFg,
    bg: "transparent",
    zIndex: 101,
  });
  moveableElement.add(moveableText);

  absolutePositionedBox = new Box(renderer, {
    id: "absolute-positioned-box",
    zIndex: 150,
    width: 20,
    height: 3,
    backgroundColor: palette.absoluteBg,
    borderStyle: "single",
    borderColor: palette.absoluteBorder,
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
    fg: palette.panelFg,
    bg: "transparent",
    zIndex: 151,
  });
  absolutePositionedBox.add(absolutePositionedText);

  // Add all children to contentArea: left, center, right
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
  footerText.content = t`${bold(`SPACE: next | R: restart | P: autoplay (${autoplayStatus}) | V: overlay (${moveableStatus}) | WASD: move`)}`;
}

function applyCurrentDemo(): void {
  const demo = layoutDemos[currentDemoIndex];
  if (!headerText) return;

  const autoplayStatus = autoplayEnabled ? "AUTO" : "MANUAL";
  headerText.content = t`${bold(`${demo.name} (${currentDemoIndex + 1}/${layoutDemos.length})`)}  ${italic(`— ${demo.description}`)}  ${autoplayStatus}`;
  demo.setup();
  applyColors();

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

/** Rebuild the palette and re-paint when the host switches light/dark theme. */
function handleThemeMode(mode: ThemeMode): void {
  palette = buildPalette(mode);
  applyColors();
  if (headerText) headerText.fg = palette.headerFg;
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
