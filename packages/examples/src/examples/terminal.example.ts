#!/usr/bin/env bun

import {
  Box,
  type CliRenderer,
  FrameBuffer,
  Input,
  InputEvents,
  RGBA,
  Text,
  createCliRenderer,
  rgbaToEngineColor,
} from "@bettertui/core";
import { ScrollBox } from "@bettertui/core";
import { HexList } from "../lib/HexList";
import { PaletteGrid } from "../lib/PaletteGrid";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";
import { formatHex, inferModeFromHex, isColorOscResponse, visibleOsc } from "../lib/terminalOsc";

// Local stub for TerminalColors (not exported from @bettertui/core)
type TerminalColors = {
  palette: (string | null | undefined)[];
  background?: string | null;
  foreground?: string | null;
  defaultBackground?: string | null;
  defaultForeground?: string | null;
  cursorColor?: string | null;
  mouseForeground?: string | null;
  mouseBackground?: string | null;
  tekForeground?: string | null;
  tekBackground?: string | null;
  highlightBackground?: string | null;
  highlightForeground?: string | null;
};

/**
 * This demo showcases terminal palette detection.
 * Enter a palette size (1-256) in the input field and press Enter to fetch colors.
 */

let scrollBox: ScrollBox | null = null;
let contentContainer: Box | null = null;
let paletteGrid: PaletteGrid | null = null;
let statusText: Text | null = null;
let diagnosticsText: Text | null = null;
let rawOscText: Text | null = null;
let hexList: HexList | null = null;
let specialColorsBuffer: FrameBuffer | null = null;
let ansiComparisonBuffer: FrameBuffer | null = null;
let terminalColors: TerminalColors | null = null;
// biome-ignore lint/suspicious/noExplicitAny: key event type not narrowed in handler
let keyboardHandler: ((key: any) => void) | null = null;
let paletteSizeInput: Input | null = null;
let oscUnsubscribe: (() => void) | null = null;

const recentColorOscResponses: string[] = [];
const maxRecentColorOscResponses = 8;

export function run(renderer: CliRenderer): void {
  renderer.start();
  const backgroundColor = RGBA.fromInts(15, 23, 42); // Slate-900 inspired
  renderer.setBackgroundColor(rgbaToEngineColor(backgroundColor));

  const mainContainer = new Box(renderer, {
    id: "main-container",
    flexGrow: 1,
    flexDirection: "column",
  });
  renderer.root.add(mainContainer);

  scrollBox = new ScrollBox(renderer, {
    id: "terminal-scroll-box",
    stickyScroll: false,
    border: true,
    borderColor: "#8B5CF6",
    title: "Terminal Palette Demo (Ctrl+C to exit)",
    titleAlignment: "center",
    contentOptions: {
      paddingLeft: 2,
      paddingRight: 2,
      paddingTop: 1,
    },
  });
  mainContainer.add(scrollBox);

  contentContainer = new Box(renderer, {
    id: "terminal-palette-container",
    width: "auto",
    flexDirection: "column",
  });
  scrollBox.add(contentContainer);

  const subtitleText = new Text(renderer, {
    id: "terminal_subtitle",
    content:
      "Enter palette size (1-256) and press Enter to fetch | 'r' fresh fetch | 'c' clear cache | Watch OSC 10/11 diagnostics below",
    fg: RGBA.fromInts(148, 163, 184), // Slate-400 - softer contrast
  });
  contentContainer.add(subtitleText);

  // Add input field for palette size
  const inputContainer = new Box(renderer, {
    id: "input-container",
    flexDirection: "row",
    marginTop: 1,
  });
  contentContainer.add(inputContainer);

  const inputLabel = new Text(renderer, {
    id: "input-label",
    content: "Palette Size: ",
    fg: RGBA.fromInts(148, 163, 184),
  });
  inputContainer.add(inputLabel);

  paletteSizeInput = new Input(renderer, {
    id: "palette-size-input",
    width: 10,
    backgroundColor: RGBA.fromInts(30, 41, 59),
    textColor: RGBA.fromInts(255, 255, 255),
    placeholder: "16",
    placeholderColor: RGBA.fromInts(100, 116, 139),
    cursorColor: RGBA.fromInts(139, 92, 246), // Purple cursor
    value: "16",
    maxLength: 3,
  });
  inputContainer.add(paletteSizeInput);

  statusText = new Text(renderer, {
    id: "terminal_status",
    content: "Status: Ready to fetch palette",
    marginTop: 1,
    fg: RGBA.fromInts(56, 189, 248), // Sky blue - modern accent
  });
  contentContainer.add(statusText);

  diagnosticsText = new Text(renderer, {
    id: "terminal_diagnostics",
    content: "Diagnostics: fetching the default 16-color palette...",
    marginTop: 1,
    fg: RGBA.fromInts(203, 213, 225),
  });
  contentContainer.add(diagnosticsText);

  rawOscText = new Text(renderer, {
    id: "terminal_raw_osc",
    content: "Recent raw OSC color replies: none yet",
    marginTop: 1,
    fg: RGBA.fromInts(148, 163, 184),
  });
  contentContainer.add(rawOscText);

  const instructionsText = new Text(renderer, {
    id: "terminal_instructions",
    content: "Press Escape to return to menu",
    marginTop: 1,
    fg: RGBA.fromInts(100, 116, 139), // Slate-500 - muted but readable
  });
  contentContainer.add(instructionsText);

  // Create palette grid - will be populated when palette is fetched
  // biome-ignore lint/suspicious/noExplicitAny: PaletteGrid accepts extended renderer type
  paletteGrid = new PaletteGrid(renderer as any, {
    id: "palette-grid",
    colors: [],
    marginTop: 2,
  });
  contentContainer.add(paletteGrid);

  // Set up input submit handler
  paletteSizeInput.on(InputEvents.ENTER, async (value: string) => {
    const size = Number.parseInt(value, 10);
    if (Number.isNaN(size) || size < 1 || size > 256) {
      if (statusText) {
        statusText.content =
          "Status: Invalid palette size. Please enter a number between 1 and 256.";
        statusText.fg = RGBA.fromInts(239, 68, 68); // Red error
      }
      return;
    }
    await fetchAndDisplayPalette(renderer, size);
  });

  // Set up keyboard handler
  keyboardHandler = async (key) => {
    if (key.name === "c") {
      clearPaletteCache(renderer);
    }
    if (key.name === "r") {
      // biome-ignore lint/suspicious/noExplicitAny: accessing internal palette cache API
      // biome-ignore lint/suspicious/noExplicitAny: accessing internal palette cache API
      (renderer as any).clearPaletteCache?.();
      await fetchAndDisplayPalette(renderer, getRequestedPaletteSize());
    }
  };

  renderer.keyInput.on("keypress", keyboardHandler);

  oscUnsubscribe =
    // biome-ignore lint/suspicious/noExplicitAny: accessing internal OSC subscription API
    ((renderer as any).subscribeOsc?.((sequence: string) => {
      if (!isColorOscResponse(sequence)) return;
      recentColorOscResponses.push(sequence);
      if (recentColorOscResponses.length > maxRecentColorOscResponses) {
        recentColorOscResponses.splice(
          0,
          recentColorOscResponses.length - maxRecentColorOscResponses,
        );
      }
      updateRawOscText();
    }) as (() => void) | undefined) ?? (() => {});

  // Focus the input field on start
  paletteSizeInput.focus();

  void fetchAndDisplayPalette(renderer, 16);
}

async function fetchAndDisplayPalette(renderer: CliRenderer, size: number): Promise<void> {
  if (!statusText || !paletteGrid) return;

  try {
    // biome-ignore lint/suspicious/noExplicitAny: accessing internal renderer palette detection state
    const r = renderer as any;
    const wasAlreadyCached = r.paletteDetectionStatus === "cached";
    statusText.content = `Status: ${wasAlreadyCached ? "Using cached palette" : "Fetching palette..."}`;
    statusText.fg = RGBA.fromInts(250, 204, 21); // Amber - warm loading state
    recentColorOscResponses.length = 0;
    updateRawOscText();

    const startTime = performance.now();
    // biome-ignore lint/suspicious/noExplicitAny: accessing internal palette API
    terminalColors = (await (renderer as any).getPalette?.({ size })) as TerminalColors;
    const elapsed = Math.round(performance.now() - startTime);

    statusText.content = `Status: Palette (${size} colors) fetched in ${elapsed}ms (${wasAlreadyCached ? "from cache" : "from terminal"})`;
    statusText.fg = RGBA.fromInts(34, 197, 94); // Emerald - fresh success state

    drawPalette(renderer, terminalColors, size);
  } catch (error) {
    if (statusText) {
      statusText.content = `Status: Error - ${error instanceof Error ? error.message : String(error)}`;
      statusText.fg = RGBA.fromInts(239, 68, 68); // Red-500 - modern error state
    }
  }
}

function clearPaletteCache(renderer: CliRenderer): void {
  if (!statusText) return;

  // biome-ignore lint/suspicious/noExplicitAny: accessing internal palette cache API
  (renderer as any).clearPaletteCache?.();
  recentColorOscResponses.length = 0;
  statusText.content =
    "Status: Cache cleared. Enter a size and press Enter to fetch palette again.";
  statusText.fg = RGBA.fromInts(148, 163, 184); // Slate-400 - neutral info state
  if (diagnosticsText) {
    diagnosticsText.content =
      "Diagnostics: cache cleared. Fetch again to inspect OSC replies and background fallback.";
  }
  updateRawOscText();
}

function getRequestedPaletteSize(): number {
  const size = Number.parseInt(paletteSizeInput?.value ?? "16", 10);
  if (Number.isNaN(size) || size < 1 || size > 256) return 16;
  return size;
}

function drawPalette(renderer: CliRenderer, terminalColors: TerminalColors, size: number): void {
  const colors = terminalColors.palette.slice(0, size).map((color) => color ?? "#000000");

  updateDiagnostics(renderer, terminalColors);
  updateRawOscText();

  // Update the palette grid with new colors
  if (paletteGrid) {
    paletteGrid.colors = colors;
  }

  // Create special colors list with colored boxes
  const specialColors = [
    { label: "Default FG", value: terminalColors.defaultForeground },
    { label: "Default BG", value: terminalColors.defaultBackground },
    { label: "Cursor", value: terminalColors.cursorColor },
    { label: "Mouse FG", value: terminalColors.mouseForeground },
    { label: "Mouse BG", value: terminalColors.mouseBackground },
    { label: "Tek FG", value: terminalColors.tekForeground },
    { label: "Tek BG", value: terminalColors.tekBackground },
    { label: "Highlight BG", value: terminalColors.highlightBackground },
    { label: "Highlight FG", value: terminalColors.highlightForeground },
  ];

  // Create a framebuffer for special colors with colored boxes
  const specialBufferWidth = 30;
  const specialBufferHeight = specialColors.length * 2;

  if (!specialColorsBuffer) {
    specialColorsBuffer = new FrameBuffer(renderer, {
      id: "special-colors-buffer",
      width: specialBufferWidth,
      height: specialBufferHeight,
      marginTop: 2,
    });
    contentContainer?.add(specialColorsBuffer);
  }

  const specialBuffer = specialColorsBuffer.frameBuffer;
  specialBuffer.clear(RGBA.fromInts(30, 41, 59, 255)); // Slate-800 background

  specialColors.forEach(({ label, value }, index) => {
    const y = index * 2;
    const boxWidth = 4;

    if (value) {
      // Parse hex color
      const hex = value.replace("#", "");
      const r = Number.parseInt(hex.substring(0, 2), 16);
      const g = Number.parseInt(hex.substring(2, 4), 16);
      const b = Number.parseInt(hex.substring(4, 6), 16);
      const rgba = RGBA.fromInts(r, g, b);

      // Draw colored box (4x2 block)
      for (let dy = 0; dy < 2; dy++) {
        for (let dx = 0; dx < boxWidth; dx++) {
          specialBuffer.setCell(dx, y + dy, " ", RGBA.fromInts(255, 255, 255), rgba);
        }
      }

      // Draw label and hex value
      const text = `${label}: ${value.toUpperCase()}`;
      const textColor = RGBA.fromInts(148, 163, 184);
      const bgColor = RGBA.fromInts(30, 41, 59, 255);
      for (let i = 0; i < text.length; i++) {
        specialBuffer.drawText(text[i], boxWidth + 1 + i, y, textColor, bgColor);
      }
    } else {
      // Draw N/A
      const text = `${label}: N/A`;
      const textColor = RGBA.fromInts(100, 116, 139);
      const bgColor = RGBA.fromInts(30, 41, 59, 255);
      for (let i = 0; i < text.length; i++) {
        specialBuffer.drawText(text[i], boxWidth + 1 + i, y, textColor, bgColor);
      }
    }
  });

  // Update the hex list with new colors
  if (!hexList) {
    // biome-ignore lint/suspicious/noExplicitAny: HexList accepts extended renderer type
    hexList = new HexList(renderer as any, {
      id: "hex-list",
      colors: colors,
      marginTop: 2,
    });
    contentContainer?.add(hexList);
  } else {
    hexList.colors = colors;
  }

  drawAnsiComparison(renderer, terminalColors);
}

function drawAnsiComparison(renderer: CliRenderer, colors: TerminalColors): void {
  const width = 74;
  const height = 5;
  const labelWidth = 19;
  const swatchWidth = 3;
  const textColor = RGBA.fromInts(203, 213, 225);
  const mutedTextColor = RGBA.fromInts(148, 163, 184);
  const bgColor = RGBA.fromInts(30, 41, 59);

  if (!ansiComparisonBuffer) {
    ansiComparisonBuffer = new FrameBuffer(renderer, {
      id: "ansi-comparison-buffer",
      width,
      height,
      marginTop: 2,
    });
    contentContainer?.add(ansiComparisonBuffer);
  }

  const buffer = ansiComparisonBuffer.frameBuffer;
  buffer.clear(bgColor);

  buffer.drawText("ANSI Slot Comparison", 0, 0, textColor, bgColor);
  buffer.drawText("Indexed 0-15:", 0, 1, mutedTextColor, bgColor);
  buffer.drawText("RGB snapshots:", 0, 3, mutedTextColor, bgColor);

  for (let index = 0; index < 16; index++) {
    const x = labelWidth + index * swatchWidth;
    const detected = colors.palette[index];
    // RGBA.fromIndex not available; use fallback color based on index
    const indexedColor = detected
      ? RGBA.fromHex(detected)
      : RGBA.fromInts(index & 1 ? 128 : 0, index & 2 ? 128 : 0, index & 4 ? 128 : 0);
    const rgbColor = detected ? RGBA.fromHex(detected) : RGBA.fromInts(0, 0, 0);

    for (let dx = 0; dx < swatchWidth; dx++) {
      buffer.setCell(x + dx, 1, " ", textColor, indexedColor);
      buffer.setCell(x + dx, 3, " ", textColor, rgbColor);
    }
  }

  buffer.drawText(
    "If rows differ, RGB system-theme colors are not rendering as their source ANSI slots.",
    0,
    4,
    mutedTextColor,
    bgColor,
  );
}

function updateDiagnostics(renderer: CliRenderer, colors: TerminalColors): void {
  if (!diagnosticsText) return;

  const palette0 = colors.palette[0] ?? null;
  const defaultBackground = colors.defaultBackground;
  const effectiveBackground = defaultBackground ?? palette0;
  // biome-ignore lint/suspicious/noExplicitAny: accessing internal renderer theme mode
  const rendererMode = ((renderer as any).themeMode as string) ?? "unknown";
  const inferredMode = inferModeFromHex(effectiveBackground);
  const modeMismatch =
    rendererMode !== "unknown" && inferredMode !== "unknown" && rendererMode !== inferredMode;
  const rawRgbaSeen = recentColorOscResponses.some((sequence) => sequence.includes("rgba:"));
  const backgroundSource = defaultBackground
    ? "OSC 11 defaultBackground"
    : palette0
      ? "palette[0] fallback because OSC 11 was missing or unparsed"
      : "none";
  const verdict = defaultBackground
    ? modeMismatch
      ? "ERR: renderer.themeMode disagrees with the detected background; derived system colors will be wrong."
      : "OK : defaultBackground parsed; system theme can use the terminal background."
    : rawRgbaSeen
      ? "ERR : raw rgba response seen, but defaultBackground is N/A. The parser dropped OSC 11."
      : "WARN: defaultBackground is N/A. System themes will fall back to palette[0].";

  diagnosticsText.content = [
    "Diagnostics:",
    `  renderer.themeMode: ${rendererMode}`,
    `  bg-inferred mode: ${inferredMode}`,
    `  Default FG (OSC 10): ${formatHex(colors.defaultForeground ?? null)}`,
    `  Default BG (OSC 11): ${formatHex(defaultBackground ?? null)}`,
    `  palette[0]: ${formatHex(palette0)}`,
    `  system-theme background source: ${backgroundSource}`,
    `  effective background: ${formatHex(effectiveBackground)}`,
    `  ${verdict}`,
  ].join("\n");
}

function updateRawOscText(): void {
  if (!rawOscText) return;

  if (recentColorOscResponses.length === 0) {
    rawOscText.content = "Recent raw OSC color replies: none yet";
    return;
  }

  rawOscText.content = [
    "Recent raw OSC color replies:",
    ...recentColorOscResponses.map((sequence) => `  ${visibleOsc(sequence)}`),
  ].join("\n");
}

export function destroy(renderer: CliRenderer): void {
  if (oscUnsubscribe) {
    oscUnsubscribe();
    oscUnsubscribe = null;
  }

  if (keyboardHandler) {
    renderer.keyInput.off("keypress", keyboardHandler);
    keyboardHandler = null;
  }

  if (paletteSizeInput) {
    paletteSizeInput.destroy();
    paletteSizeInput = null;
  }

  if (scrollBox) {
    const mainContainer = renderer.root.getRenderable("main-container");
    if (mainContainer) renderer.root.remove(mainContainer);
    scrollBox = null;
  }

  contentContainer = null;
  paletteGrid = null;
  hexList = null;
  specialColorsBuffer = null;
  ansiComparisonBuffer = null;
  diagnosticsText = null;
  rawOscText = null;
  statusText = null;
  terminalColors = null;
  recentColorOscResponses.length = 0;
}

if (import.meta.main) {
  const renderer = await createCliRenderer({
    exitOnCtrlC: true,
  });
  run(renderer);
  setupCommonDemoKeys(renderer);
}
