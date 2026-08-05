// Curated re-export of shared types
export type {
  AlignItems,
  AlignSelf,
  BorderStyle,
  ColorValue,
  FlexDirection,
  Gap,
  Inset,
  JustifyContent,
  KeyEventSource,
  KeyEventType,
  LayoutConstraints,
  Margin,
  MouseButton,
  MouseEvent,
  Overflow,
  Padding,
  Position,
  Sizing,
  Style,
  Theme,
  ThemeColors,
  ThemeSpacing,
} from "@bettertui/shared";

// Geometry types (core-only: not needed by framework adapters)
export type { Point, Rect, Size } from "./geometry.types";

// Command protocol, buffer, and tree operations
export * from "./command";

// Reconciler (wraps tree ops with command emission)
export { createReconciler } from "./reconciler";

// Command runtime (frame loop over CommandBuffer)
export { CommandRuntime } from "./runtime";
export type { CommandRuntimeOptions } from "./runtime";

export { Renderable } from "./renderable";
export type {
  WidgetContext,
  WidgetLifecycle,
  ImperativeContext,
} from "./renderable";

// Keymap, clock, and validation utilities (includes styled text, RGBA, events, etc.)
export * from "./lib";

// Platform (native engine bridge, events, runtime)
export * from "./platform";

// Testing utilities (explicit re-exports to avoid conflicts)
export {
  createTestRenderer,
  createTestRendererSync,
  createMockKeys,
  KeyCodes,
  createMockMouse,
  MouseButtons,
  createTestStdin,
  createTestStdout,
  TestReadStream,
  TestWriteStream,
  createSpy,
  createTerminalCapabilities,
  createMinimalTerminalCapabilities,
  createFullTerminalCapabilities,
  createKittyTerminalCapabilities,
  createITerm2TerminalCapabilities,
  createMockNativeKeymap,
  createTestKeymap,
} from "./testing";
export type {
  TestRendererOptions,
  TestRenderer,
  MockInput,
  MockMouse,
  TestRendererSetup,
  TestKeyInput,
  MockKeysOptions,
  KeyModifiers,
  MousePosition,
  MouseModifiers,
  MouseEventType,
  MouseEventOptions,
  TestStdin,
  TestStdout,
  Spy,
  TerminalCapabilitiesOptions,
  TestBinding,
} from "./testing";

// Animation utilities: easing, Tween, Spring, lerp helpers
export * from "./animations";

// Terminal graphics utilities: PixelBuffer, Canvas, color helpers
// Note: Export specific items to avoid RGBA interface conflict (RGBA class exported from ./lib)
export {
  type RGB,
  parseHex,
  rgbFg,
  rgbBg,
  RESET,
  PixelBuffer,
  Canvas,
  gradientH,
} from "./graphics";

// In-core debug tooling (moved from the retired @bettertui/devtools package).
export * from "./devtools";

// Audio API (stub implementations; native playback requires native engine)
export { Audio, AudioStream, AudioStreamError } from "./audio";
export type {
  AudioGroup,
  AudioStreamAction,
  AudioStreamErrorContext,
  AudioStreamState,
  AudioStreamStats,
  AudioStreamMetadata,
  AudioStreamMetadataFormat,
  AudioStreamReconnectEvent,
  AudioStreamUrlOptions,
  AudioPlaybackDevice,
  AudioSound,
  AudioVoice,
  AudioSetupOptions,
  AudioStartOptions,
  AudioPlayOptions,
  AudioStats,
  AudioTapResult,
} from "./audio";

// ── Renderable widgets (high-level CliRenderer-backed UI components) ──────────

export {
  // Core renderables (standard naming)
  Box,
  Root,
  Text,
  Input,
  Select,
  ScrollBox,
  ScrollBar,
  Textarea,
  ExtmarksControllerStub,
  TabSelect,
  Slider,
  TextNode,
  RootTextNode,
  Screen,
  ScreenEvents,
  ASCIIFont,
  FrameBuffer,
  Code,
  Diff,
  Markdown,
  TextTable,
  LineNumber,
  TimeToFirstDraw,
} from "./renderables";

export type {
  BoxOptions,
  BorderSide,
  BorderStyleKind,
  TextOptions,
  InputOptions,
  InputRenderableOptions,
  SelectOption,
  SelectOptions,
  SelectRenderableOptions,
  ScrollBoxOptions,
  ScrollBarOptions,
  TextareaOptions,
  ExtmarksController,
  TabOption,
  TabSelectOptions,
  TabSelectRenderableOptions,
  SliderOptions,
  SliderRenderableOptions,
  TextNodeOptions,
  ASCIIFontKind,
  ASCIIFontOptions,
  FrameBufferOptions,
  FrameBufferLike,
  CodeOptions,
  DiffOptions,
  MarkdownOptions,
  TableColumn,
  TextTableOptions,
  TextTableColumnFitter,
  TextTableColumnWidthMode,
  TextTableContent,
  LineNumberOptions,
  TimeToFirstDrawOptions,
  CanvasOptions,
  CanvasHeaderOptions,
  CanvasFooterOptions,
  CanvasBodyOptions,
  ScreenResizeEvent,
} from "./renderables";

// ── Additional utility exports ─────────────────────────────────────────────────

import { measureFontText } from "./lib/asciiFont";

/** Measure the display width and height of text. */
export function measureText(opts: { text: string; font?: string }): {
  width: number;
  height: number;
} {
  const { text, font } = opts;
  if (font) {
    // biome-ignore lint/suspicious/noControlCharactersInRegex: ANSI escape sequences require ESC character
    const stripped = text.replace(/\x1b\[[^m]*m/g, "");
    return measureFontText(stripped, font);
  }
  // biome-ignore lint/suspicious/noControlCharactersInRegex: ANSI escape sequences require ESC character
  const stripped = text.replace(/\x1b\[[^m]*m|\x1b\][^\x07\x1b]*[\x07\x1b\\]/g, "");
  return { width: stripped.length, height: 1 };
}

export function decodePasteBytes(bytes: Uint8Array): string {
  return Buffer.from(bytes).toString("utf8");
}

export function resolveRenderLib(): { getArenaAllocatedBytes: () => number } {
  return { getArenaAllocatedBytes: () => 0 };
}

/** Strip ANSI escape sequences from a string. */
export function stripAnsiSequences(str: string): string {
  // biome-ignore lint/suspicious/noControlCharactersInRegex: ANSI escape sequences require ESC character
  return str.replace(/\x1b\[[^m]*m|\x1b\][^\x07\x1b]*[\x07\x1b\\]/g, "");
}

export type Selection = {
  start: { line: number; col: number };
  end: { line: number; col: number };
  text: string;
  getSelectedText(): string;
  isDragging: boolean;
};

/** HAST (Hypertext Abstract Syntax Tree) element type. */
export type HASTElement = {
  type: string;
  tagName?: string;
  value?: string;
  properties?: Record<string, unknown>;
  children?: HASTElement[];
};

export class SyntaxStyle {
  fg?: string;
  bg?: string;
  bold?: boolean;
  italic?: boolean;
  private _styles: Map<string, Record<string, unknown>> = new Map();
  private _cache: Map<string, unknown> = new Map();

  constructor(opts?: {
    fg?: string;
    bg?: string;
    bold?: boolean;
    italic?: boolean;
  }) {
    Object.assign(this, opts ?? {});
  }

  /** Create a new SyntaxStyle instance. */
  static create(): SyntaxStyle {
    return new SyntaxStyle();
  }

  /** Create a SyntaxStyle from a record of style definitions. */
  static fromStyles(
    styles: Record<string, { fg?: unknown; bg?: unknown; bold?: boolean; italic?: boolean }>,
  ): SyntaxStyle {
    const s = new SyntaxStyle();
    for (const [name, style] of Object.entries(styles)) {
      s._styles.set(name, style as Record<string, unknown>);
    }
    return s;
  }

  /** Register a named style and return its numeric ID. */
  registerStyle(name: string, style: Record<string, unknown>): number {
    this._styles.set(name, style);
    return this._styles.size - 1;
  }

  /** Get the number of cached entries. */
  getCacheSize(): number {
    return this._cache.size;
  }

  /** Clear the style cache. */
  clearCache(): void {
    this._cache.clear();
  }

  destroy(): void {
    this._styles.clear();
    this._cache.clear();
  }
}

/** Convert a HAST tree to a StyledText string using the given SyntaxStyle. */
export function hastToStyledText(node: HASTElement, _style: SyntaxStyle): string {
  function traverse(n: HASTElement): string {
    if (n.type === "text") return n.value ?? "";
    if (n.children) return n.children.map(traverse).join("");
    return "";
  }
  return traverse(node);
}

export type RenderContext = {
  width: number;
  height: number;
  requestRender(): void;
};

export type OptimizedBuffer = {
  width: number;
  height: number;
  buffers: {
    bg: Uint16Array;
    fg: Uint16Array;
    char: Uint32Array;
    attributes: Uint32Array;
  };
  setCell(x: number, y: number, char: string, fg?: unknown, bg?: unknown): void;
  drawText(
    text: string,
    x: number,
    y: number,
    fg?: unknown,
    bg?: unknown,
    attributes?: number,
  ): void;
  fillRect(x: number, y: number, w: number, h: number, color: unknown): void;
};
