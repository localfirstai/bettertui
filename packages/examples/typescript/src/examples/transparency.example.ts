import {
  BoxRenderable,
  type CliRenderer,
  type KeyEvent,
  RGBA,
  TextRenderable,
  bold,
  createCliRenderer,
  fg,
  t,
  underline,
} from "@bettertui/core";
import type { ThemeMode } from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys.js";

let nextZIndex = 101;
let draggableBoxes: DraggableTransparentBox[] = [];
let keyListener: ((key: KeyEvent) => void) | null = null;
let themeModeListener: ((mode: ThemeMode) => void) | null = null;
let demoRunVersion = 0;

const DEFAULT_THEME_MODE: ThemeMode = "dark";

const THEMES = {
  dark: {
    backgroundColor: "#0A0E14",
    headerAccent: "#00D4AA",
    headerMuted: "#A8A8B2",
    textUnderAlpha: "#FFB84D",
    moreTextUnder: "#7B68EE",
    boxLabelColor: RGBA.fromInts(255, 255, 255, 220),
    label: "dark",
  },
  light: {
    backgroundColor: "#F6F1E5",
    headerAccent: "#0F766E",
    headerMuted: "#4B5563",
    textUnderAlpha: "#B45309",
    moreTextUnder: "#6D28D9",
    boxLabelColor: RGBA.fromInts(17, 24, 39, 220),
    label: "light",
  },
  transparent: {
    backgroundColor: "transparent",
    headerAccent: "#0284C7",
    headerMuted: "#64748B",
    textUnderAlpha: "#D97706",
    moreTextUnder: "#7C3AED",
    boxLabelColor: RGBA.fromInts(255, 255, 255, 220),
    label: "transparent",
  },
} as const;

type ThemeName = keyof typeof THEMES;
const THEME_ORDER: ThemeName[] = ["dark", "light", "transparent"];

function getTransparentFallbackBackgroundColor(themeMode: ThemeMode): RGBA {
  return themeMode === "light" ? RGBA.fromInts(255, 255, 255, 0) : RGBA.fromInts(0, 0, 0, 0);
}

function getThemeBackgroundColor(themeName: ThemeName, themeMode: ThemeMode): string | RGBA {
  if (themeName === "transparent") {
    return getTransparentFallbackBackgroundColor(themeMode);
  }

  return THEMES[themeName].backgroundColor;
}

function getHeaderText(themeName: ThemeName) {
  const theme = THEMES[themeName];

  return t`${bold(underline(fg(theme.headerAccent)("Interactive Alpha Transparency & Blending Demo - Drag the boxes!")))}
${fg(theme.headerMuted)(`Drag boxes with the mouse • Press B to cycle dark/light/transparent (current: ${theme.label})`)}`;
}

function colorToString(color: string | RGBA): string {
  if (typeof color === "string") return color;
  return RGBA.toHex(color);
}

class DraggableTransparentBox extends BoxRenderable {
  private isDragging = false;
  private dragOffsetX = 0;
  private dragOffsetY = 0;
  private alphaPercentage: number;
  private labelColor: RGBA;

  constructor(
    ctx: CliRenderer,
    id: string,
    x: number,
    y: number,
    width: number,
    height: number,
    bg: RGBA,
    zIndex: number,
  ) {
    super(ctx, {
      id,
      width,
      height,
      zIndex,
      backgroundColor: bg,
      position: "absolute",
      left: x,
      top: y,
    });
    this.alphaPercentage = Math.round(bg.a * 100);
    this.labelColor = THEMES.dark.boxLabelColor;

    // Alpha percentage label as a child TextRenderable
    const alphaText = `${this.alphaPercentage}%`;
    const w = typeof width === "number" ? width : 0;
    const h = typeof height === "number" ? height : 0;
    const label = new TextRenderable(ctx, {
      id: `${id}-label`,
      content: alphaText,
      position: "absolute",
      left: Math.max(0, Math.floor(w / 2) - Math.floor(alphaText.length / 2)),
      top: Math.floor(h / 2),
    });
    label.fg = this.labelColor;
    this.add(label);

    // Wire up mouse event handling via options after super() returns
    this._options.onMouseDown = (e: unknown) => this._onMouse(e, "down");
    this._options.onMouseDrag = (e: unknown) => this._onMouse(e, "drag");
    this._options.onMouseDragEnd = (e: unknown) => this._onMouse(e, "drag-end");
  }

  public setLabelColor(color: RGBA): void {
    this.labelColor = color;
  }

  private _onMouse(event: unknown, type: string): void {
    const e = event as { x?: number; y?: number; stopPropagation?: () => void };
    const ex = (e.x ?? 0) as number;
    const ey = (e.y ?? 0) as number;
    const w = typeof this._options.width === "number" ? this._options.width : 0;
    const h = typeof this._options.height === "number" ? this._options.height : 0;

    switch (type) {
      case "down":
        this.isDragging = true;
        this.dragOffsetX = ex - this.x;
        this.dragOffsetY = ey - this.y;
        this.zIndex = nextZIndex++;
        e.stopPropagation?.();
        break;
      case "drag-end":
        if (this.isDragging) {
          this.isDragging = false;
          e.stopPropagation?.();
        }
        break;
      case "drag":
        if (this.isDragging) {
          const newX = ex - this.dragOffsetX;
          const newY = ey - this.dragOffsetY;
          this.setPosition({
            left: Math.max(0, Math.min(newX, this._renderer.terminalWidth - w)),
            top: Math.max(4, Math.min(newY, this._renderer.terminalHeight - h)),
          });
          e.stopPropagation?.();
        }
        break;
    }
  }
}

export function run(renderer: CliRenderer): void {
  renderer.start();

  const currentRunVersion = ++demoRunVersion;
  let currentTheme: ThemeName = "dark";
  let currentThemeMode: ThemeMode = renderer.themeMode ?? DEFAULT_THEME_MODE;
  let transparentBackgroundColor = getTransparentFallbackBackgroundColor(currentThemeMode);
  let transparentPaletteRequestVersion = 0;
  renderer.setBackgroundColor(
    colorToString(getThemeBackgroundColor(currentTheme, currentThemeMode)),
  );

  const parentContainer = new BoxRenderable(renderer, {
    id: "parent-container",
    zIndex: 10,
  });
  renderer.root.add(parentContainer);

  const headerDisplay = new TextRenderable(renderer, {
    id: "header-text",
    content: getHeaderText(currentTheme),
    width: 85,
    height: 3,
    position: "absolute",
    left: 10,
    top: 2,
    zIndex: 1,
  });
  parentContainer.add(headerDisplay);

  const textUnderAlpha = new TextRenderable(renderer, {
    id: "text-under-alpha",
    content: t`${bold("This text should not be selectable")}`,
    position: "absolute",
    left: 10,
    top: 6,
    fg: THEMES[currentTheme].textUnderAlpha,
    zIndex: 4,
  });
  parentContainer.add(textUnderAlpha);

  const moreTextUnder = new TextRenderable(renderer, {
    id: "more-text-under",
    content: t`${bold("Selectable text to show character preservation")}`,
    position: "absolute",
    left: 15,
    top: 10,
    fg: THEMES[currentTheme].moreTextUnder,
    zIndex: 1,
  });
  parentContainer.add(moreTextUnder);

  const alphaBox50 = new DraggableTransparentBox(
    renderer,
    "alpha-box-50",
    15,
    5,
    25,
    8,
    RGBA.fromValues(64 / 255, 176 / 255, 255 / 255, 128 / 255),
    50,
  );
  parentContainer.add(alphaBox50);
  draggableBoxes.push(alphaBox50);

  const alphaBox75 = new DraggableTransparentBox(
    renderer,
    "alpha-box-75",
    30,
    7,
    25,
    8,
    RGBA.fromValues(255 / 255, 107 / 255, 129 / 255, 192 / 255),
    30,
  );
  parentContainer.add(alphaBox75);
  draggableBoxes.push(alphaBox75);

  const alphaBox25 = new DraggableTransparentBox(
    renderer,
    "alpha-box-25",
    45,
    9,
    25,
    8,
    RGBA.fromValues(139 / 255, 69 / 255, 193 / 255, 64 / 255),
    10,
  );
  parentContainer.add(alphaBox25);
  draggableBoxes.push(alphaBox25);

  const alphaGreen = new DraggableTransparentBox(
    renderer,
    "alpha-green",
    20,
    11,
    30,
    5,
    RGBA.fromValues(88 / 255, 214 / 255, 141 / 255, 96 / 255),
    20,
  );
  parentContainer.add(alphaGreen);
  draggableBoxes.push(alphaGreen);

  const alphaYellow = new DraggableTransparentBox(
    renderer,
    "alpha-yellow",
    25,
    13,
    20,
    6,
    RGBA.fromValues(255 / 255, 183 / 255, 77 / 255, 128 / 255),
    40,
  );
  parentContainer.add(alphaYellow);
  draggableBoxes.push(alphaYellow);

  const alphaOverlay = new DraggableTransparentBox(
    renderer,
    "alpha-overlay",
    10,
    17,
    65,
    4,
    RGBA.fromValues(200 / 255, 162 / 255, 255 / 255, 32 / 255),
    60,
  );
  parentContainer.add(alphaOverlay);
  draggableBoxes.push(alphaOverlay);

  const applyTheme = (themeName: ThemeName): void => {
    currentTheme = themeName;

    const theme = THEMES[themeName];
    renderer.setBackgroundColor(
      themeName === "transparent"
        ? colorToString(transparentBackgroundColor)
        : theme.backgroundColor,
    );
    headerDisplay.content = getHeaderText(themeName);
    textUnderAlpha.fg = theme.textUnderAlpha;
    moreTextUnder.fg = theme.moreTextUnder;

    for (const box of draggableBoxes) {
      box.setLabelColor(theme.boxLabelColor);
    }

    if (themeName === "transparent") {
      void updateTransparentBackgroundColor();
    }
  };

  const updateTransparentBackgroundColor = async (): Promise<void> => {
    const requestVersion = ++transparentPaletteRequestVersion;
    transparentBackgroundColor = getTransparentFallbackBackgroundColor(currentThemeMode);

    if (currentTheme === "transparent") {
      renderer.setBackgroundColor(colorToString(transparentBackgroundColor));
    }

    // Note: renderer.getPalette() is not available in the current API;
    // transparent background color is derived from the theme mode fallback.
    if (
      currentRunVersion !== demoRunVersion ||
      requestVersion !== transparentPaletteRequestVersion
    ) {
      return;
    }

    if (currentTheme === "transparent") {
      renderer.setBackgroundColor(colorToString(transparentBackgroundColor));
    }
  };

  applyTheme(currentTheme);

  if (keyListener) {
    renderer.keyInput.off("keypress", keyListener);
  }

  if (themeModeListener) {
    renderer.off("theme_mode", themeModeListener);
  }

  keyListener = (key: KeyEvent) => {
    if (key.name !== "b") {
      return;
    }

    const currentThemeIndex = THEME_ORDER.indexOf(currentTheme);
    const nextTheme = THEME_ORDER[(currentThemeIndex + 1) % THEME_ORDER.length];
    applyTheme(nextTheme);
  };

  renderer.keyInput.on("keypress", keyListener);

  themeModeListener = (mode: ThemeMode) => {
    currentThemeMode = mode;
    // renderer.clearPaletteCache() is not available in the current API

    if (currentTheme === "transparent") {
      void updateTransparentBackgroundColor();
    }
  };

  renderer.on("theme_mode", themeModeListener);
}

export function destroy(renderer: CliRenderer): void {
  demoRunVersion += 1;
  renderer.clearFrameCallbacks();

  if (keyListener) {
    renderer.keyInput.off("keypress", keyListener);
    keyListener = null;
  }

  if (themeModeListener) {
    renderer.off("theme_mode", themeModeListener);
    themeModeListener = null;
  }

  for (const box of draggableBoxes) {
    renderer.root.remove(box);
  }
  draggableBoxes = [];

  const parentContainer = renderer.root.getRenderable("parent-container");
  if (parentContainer) renderer.root.remove(parentContainer);
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });

  run(renderer);
  setupCommonDemoKeys(renderer);
  renderer.start();
}
