/**
 * Layout System — interactive layout demos.
 *
 * Demonstrates flexbox-style layout primitives in BetterTUI (`@bettertui/core`):
 * row/column direction, grow/shrink/basis, alignment, justification, gaps,
 * padding/margin, absolute positioning, zIndex, and Box titles with borders.
 *
 * Uses `Screen` from `@bettertui/core` as the root container — it handles
 * full-terminal sizing, header/body/footer slots, and resize automatically.
 *
 * Controls:
 *   SPACE  next demo       R      restart from first demo
 *   P      toggle autoplay
 */

import {
  Box,
  type CliRenderer,
  type KeyEvent,
  Screen,
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

/** Bright, saturated colours derived from the Saha theme's semantic tokens. */
interface DemoColors {
  // Panel accent colors
  blue: string;
  green: string;
  amber: string;
  red: string;
  brand: string;
  slate: string;
  textOnBright: string;
  textOnDark: string;
  // App shell
  appBg: string;
  // Header — elevated secondary surface with brand ring border
  headerBg: string;
  headerFg: string;
  headerBorder: string;
  // Footer — muted surface with quiet hints text
  footerBg: string;
  footerFg: string;
  footerBorder: string;
  // Absolute-positioned button (destructive red bg, white border + text)
  absoluteBg: string;
  absoluteFg: string;
  absoluteBorder: string;
}

// ── Module-level state ─────────────────────────────────────────────────────────

let renderer: CliRenderer | null = null;
let screen: Screen | null = null;
let headerText: Text | null = null;
let sidebar: Box | null = null;
let sidebarText: Text | null = null;
let mainContent: Box | null = null;
let mainContentText: Text | null = null;
let rightSidebar: Box | null = null;
let rightSidebarText: Text | null = null;
let footerText: Text | null = null;
let absolutePositionedBox: Box | null = null;
let absolutePositionedText: Text | null = null;
let currentDemoIndex = 0;
let autoAdvanceTimeout: ReturnType<typeof setTimeout> | null = null;
let autoplayEnabled = true;
let colors: DemoColors;

// ── Color tokens ───────────────────────────────────────────────────────────────

function buildColors(mode: ThemeMode): DemoColors {
  const tokens = getThemeTokens(mode);
  const comp = getComponentTheme(mode);
  return {
    blue: tokens.info,
    green: tokens.success,
    amber: tokens.warning,
    red: tokens.destructive,
    brand: tokens.primary,
    slate: tokens.muted,
    textOnBright: tokens.primaryForeground,
    textOnDark: tokens.foreground,
    appBg: tokens.background,
    headerBg: tokens.secondary,
    headerFg: tokens.foreground,
    headerBorder: tokens.ring,
    footerBg: tokens.muted,
    footerFg: tokens.mutedForeground,
    footerBorder: comp.border,
    absoluteBg: tokens.destructive,
    absoluteFg: tokens.destructiveForeground,
    absoluteBorder: tokens.destructiveForeground,
  };
}

// ── Layout demos ───────────────────────────────────────────────────────────────
function setupHorizontalLayout(): void {
  if (!screen || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = false;

  screen.setBodyLayout({ flexDirection: "row", alignItems: "stretch" });

  const sidebarWidth = Math.max(15, Math.floor(screen.terminalWidth * 0.2));
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
  if (!screen || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = false;

  screen.setBodyLayout({ flexDirection: "column", alignItems: "stretch" });

  const contentHeight = screen.terminalHeight - 6;
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
  if (!screen || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = false;
  mainContent.visible = true;
  rightSidebar.visible = false;

  screen.setBodyLayout({
    flexDirection: "row",
    alignItems: "stretch",
    justifyContent: "center",
  });

  const tw = screen.terminalWidth;
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
  if (!screen || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = true;

  screen.setBodyLayout({ flexDirection: "row", alignItems: "stretch" });

  const sidebarWidth = Math.max(12, Math.floor(screen.terminalWidth * 0.15));

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
  if (!screen || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = true;

  screen.setBodyLayout({ flexDirection: "row", alignItems: "stretch" });

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
  if (!screen || !sidebar || !mainContent || !rightSidebar) return;

  sidebar.visible = true;
  mainContent.visible = true;
  rightSidebar.visible = true;

  screen.setBodyLayout({
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
  });

  const sidebarWidth = Math.max(12, Math.floor(screen.terminalWidth * 0.12));

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
    description: "Three equal-width columns",
    setup: setupEqualColumnsLayout,
  },
  {
    name: "Justify Space-Between",
    description: "justifyContent: space-between",
    setup: setupJustifyLayout,
  },
];

// ── Color application ──────────────────────────────────────────────────────────

function applyColors(): void {
  if (!renderer || !screen) return;
  renderer.setBackgroundColor(colors.appBg);

  screen.applyHeaderOptions({
    backgroundColor: colors.headerBg,
    borderColor: colors.headerBorder,
  });
  screen.applyFooterOptions({
    backgroundColor: colors.footerBg,
    borderColor: colors.footerBorder,
  });

  if (headerText) headerText.fg = colors.headerFg;
  if (footerText) footerText.fg = colors.footerFg;

  if (absolutePositionedBox) {
    absolutePositionedBox.backgroundColor = colors.absoluteBg;
    absolutePositionedBox.borderColor = colors.absoluteBorder;
  }
  if (absolutePositionedText) absolutePositionedText.fg = colors.absoluteFg;
}

// ── Setup ──────────────────────────────────────────────────────────────────────

function createLayoutElements(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  const mode: ThemeMode = renderer.themeMode ?? DEFAULT_THEME_MODE;
  colors = buildColors(mode);
  renderer.setBackgroundColor(colors.appBg);

  screen = new Screen(renderer, {
    id: "layout-demo",
    backgroundColor: colors.appBg,
    header: {
      id: "layout-header",
      height: 3,
      backgroundColor: colors.headerBg,
      borderColor: colors.headerBorder,
      border: true,
      borderStyle: "single",
      alignItems: "center",
      justifyContent: "center",
    },
    body: {
      id: "layout-body",
      flexDirection: "row",
    },
    footer: {
      id: "layout-footer",
      height: 3,
      backgroundColor: colors.footerBg,
      borderColor: colors.footerBorder,
      border: true,
      borderStyle: "single",
      alignItems: "center",
      justifyContent: "center",
    },
  });

  // Header text
  headerText = new Text(renderer, {
    id: "header-text",
    content: "LAYOUT DEMO",
    fg: colors.headerFg,
    bg: "transparent",
    zIndex: 1,
    flexGrow: 1,
    flexShrink: 1,
    textAlign: "center",
  });
  screen.header?.add(headerText);

  // Footer text
  footerText = new Text(renderer, {
    id: "footer-text",
    content: "",
    fg: colors.footerFg,
    bg: "transparent",
    zIndex: 1,
    flexGrow: 1,
    flexShrink: 1,
    textAlign: "center",
  });
  screen.footer?.add(footerText);

  // Panel: left sidebar
  sidebar = new Box(renderer, {
    id: "sidebar",
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
  });
  sidebar.add(sidebarText);

  // Panel: main content
  mainContent = new Box(renderer, {
    id: "main-content",
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
  });
  mainContent.add(mainContentText);

  // Panel: right sidebar
  rightSidebar = new Box(renderer, {
    id: "right-sidebar",
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
  });
  rightSidebar.add(rightSidebarText);

  rightSidebar.visible = false;
  screen.body.add(sidebar);
  screen.body.add(mainContent);
  screen.body.add(rightSidebar);

  // Button-style box: transparent background, visible border, centered text
  absolutePositionedBox = new Box(renderer, {
    id: "absolute-positioned-box",
    zIndex: 150,
    width: 16,
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

  // Outline-button text: transparent bg, ring-colored fg, fills inner area
  absolutePositionedText = new Text(renderer, {
    id: "absolute-positioned-text",
    content: "BOTTOM RIGHT",
    fg: colors.absoluteFg,
    bg: "transparent",
    flexGrow: 1,
    flexShrink: 1,
    textAlign: "center",
  });

  absolutePositionedBox.add(absolutePositionedText);
  renderer.root.add(absolutePositionedBox);

  updateFooterText();
}

// ── Event handlers ─────────────────────────────────────────────────────────────

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

function updateFooterText(): void {
  if (!footerText) return;
  const autoplayStatus = autoplayEnabled ? "ON" : "OFF";
  footerText.content = `SPACE: next | R: restart | P: autoplay (${autoplayStatus})`;
}

function applyCurrentDemo(): void {
  const demo = layoutDemos[currentDemoIndex];
  if (!headerText) return;

  const autoplayStatus = autoplayEnabled ? "AUTO" : "MANUAL";
  headerText.content = t`${bold(demo.name)}  ${italic(`(${currentDemoIndex + 1}/${layoutDemos.length})`)}  —  ${demo.description}  [${autoplayStatus}]`;
  demo.setup();
  applyColors();
  updateFooterText();

  if (autoAdvanceTimeout) clearTimeout(autoAdvanceTimeout);
  if (autoplayEnabled) {
    autoAdvanceTimeout = setTimeout(() => nextDemo(), 4000);
  }
}

function handleThemeMode(mode: ThemeMode): void {
  colors = buildColors(mode);
  applyCurrentDemo();
}

// ── Public lifecycle ───────────────────────────────────────────────────────────

export function run(rendererInstance: CliRenderer): void {
  rendererInstance.start();
  createLayoutElements(rendererInstance);
  rendererInstance.keyInput.on("keypress", handleKeyPress);
  rendererInstance.on("theme_mode", handleThemeMode);
  currentDemoIndex = 0;
  applyCurrentDemo();
  rendererInstance.renderFull();
}

export function destroy(rendererInstance: CliRenderer): void {
  if (autoAdvanceTimeout) {
    clearTimeout(autoAdvanceTimeout);
    autoAdvanceTimeout = null;
  }

  rendererInstance.keyInput.off("keypress", handleKeyPress);
  rendererInstance.off("theme_mode", handleThemeMode);

  screen?.destroy();

  if (absolutePositionedBox) rendererInstance.root.remove(absolutePositionedBox);

  screen = null;
  renderer = null;
  headerText = null;
  sidebar = null;
  sidebarText = null;
  mainContent = null;
  mainContentText = null;
  rightSidebar = null;
  rightSidebarText = null;
  footerText = null;
  absolutePositionedBox = null;
  absolutePositionedText = null;
  currentDemoIndex = 0;
  autoplayEnabled = true;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
    targetFps: 30,
  });
  run(renderer);
  setupCommonDemoKeys(renderer);
}
