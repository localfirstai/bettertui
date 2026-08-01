import { EventEmitter } from "node:events";
import type { LayoutConstraints, Style } from "@bettertui/shared";
import type { DevTools, DevToolsOptions } from "../devtools";
import { createDevTools } from "../devtools";
import { OverlayHost } from "../devtools/overlay/overlayHost";
import { DebugPanel } from "../devtools/overlay/panel.types";
import { KeyInput } from "../lib/keyInput";
import { CliRenderEvents } from "../lib/renderableEvents";
import type { NapiEngine, NapiKeymap, TerminalCapabilities } from "./binding";
import {
  createEngine,
  createKeymap,
  detectCapabilities,
  getVersion,
  loggerGetDiagnostics,
  loggerInit,
} from "./binding";
import type { DiagnosticSnapshot, LoggerConfig } from "./logger";
import type { ExternalOutputMode, ScreenMode } from "./platform.types";

export { CliRenderEvents };

export interface RawKeyEvent {
  name: string;
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
  meta: boolean;
  sequence: string;
  preventDefault(): void;
}

export interface CliRendererOptions {
  width?: number;
  height?: number;
  exitOnCtrlC?: boolean;
  targetFps?: number;
  screenMode?: ScreenMode;
  footerHeight?: number;
  externalOutputMode?: ExternalOutputMode;
  logger?: LoggerConfig;
  debug?: boolean | DevToolsOptions;
  onDestroy?: () => void;
  enableMouseMovement?: boolean;
  useMouse?: boolean;
  autoFocus?: boolean;
  backgroundColor?: string;
}

/** Minimal console overlay stub. */
export class TerminalConsole {
  private _visible = false;
  keyBindings: Record<string, unknown> = {};
  onCopySelection?: () => void;

  show(): void {
    this._visible = true;
  }
  hide(): void {
    this._visible = false;
  }
  toggle(): void {
    this._visible = !this._visible;
  }
  get visible(): boolean {
    return this._visible;
  }
}

export type ThemeMode = "light" | "dark";

type FrameCallback = (deltaTime: number) => void | Promise<void>;

// Lazy import to avoid circular at module level
let _RootRenderable: typeof import("../renderables/Box").RootRenderable | undefined;

function getRootRenderable() {
  if (!_RootRenderable) {
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    _RootRenderable = require("../renderables/Box").RootRenderable;
  }
  if (!_RootRenderable) throw new Error("RootRenderable could not be loaded");
  return _RootRenderable;
}

export class CliRenderer extends EventEmitter {
  private engine: NapiEngine;
  private keymap: NapiKeymap;
  private _keyInput: KeyInput;
  private _capabilities: TerminalCapabilities;
  private width: number;
  private height: number;
  private renderOffset: number;
  private _screenMode: ScreenMode;
  private _externalOutputMode: ExternalOutputMode;
  private externalOutputBuffer: string[] = [];
  private nodes: Map<number, { parent: number | null; children: number[] }> = new Map();
  private running = false;
  private paused = false;
  private _devtools: DevTools;
  private overlay: OverlayHost | null = null;
  private lastFrameTime = 0;
  private _frameInterval: ReturnType<typeof setTimeout> | null = null;
  private _frameCallbacks: Set<FrameCallback> = new Set();
  private _root: import("../renderables/Box").RootRenderable | null = null;
  private _console: TerminalConsole = new TerminalConsole();
  private _themeMode: ThemeMode = "dark";
  private _targetFps: number;
  private _onDestroy: (() => void) | undefined;
  private _pendingRender = false;
  private _resizeHandler: (() => void) | null = null;
  private _liveCount = 0;

  constructor(options: CliRendererOptions = {}) {
    super();
    if (options.logger) {
      loggerInit({
        dev: process.env.NODE_ENV !== "production",
        ...options.logger,
      });
    }
    this._capabilities = detectCapabilities();
    this.width = options.width ?? this._capabilities.columns;
    this.height = options.height ?? this._capabilities.rows;
    this._targetFps = options.targetFps ?? 30;

    this._screenMode = options.screenMode ?? "alternate-screen";
    this._externalOutputMode =
      options.externalOutputMode ??
      (this._screenMode === "split-footer" ? "capture-stdout" : "passthrough");
    const footerHeight = options.footerHeight ?? 0;
    this.renderOffset =
      this._screenMode === "split-footer" ? Math.max(0, this.height - footerHeight) : 0;

    this.engine = createEngine(this.width, this.height);
    this.keymap = createKeymap();
    this._keyInput = new KeyInput();
    this._onDestroy = options.onDestroy;

    const rootId = this.engine.root();
    this.nodes.set(rootId, { parent: null, children: [] });

    // Debug tooling
    const envDebug =
      process.env.BTUI_DEBUG === "1" ||
      process.env.BTUI_DEBUG === "true" ||
      process.env.BTUI_SHOW_STATS === "1" ||
      process.env.BTUI_SHOW_STATS === "true";
    const debugOption = options.debug;

    if (debugOption || envDebug) {
      const devToolsOptions: DevToolsOptions =
        typeof debugOption === "object" ? { ...debugOption, enabled: true } : { enabled: true };
      this._devtools = createDevTools(devToolsOptions);
      this._devtools.updateCapabilities(mapCapabilities(this._capabilities));
      this.overlay = new OverlayHost(this, this._devtools);
      if (envDebug || debugOption === true) {
        this._devtools.show(DebugPanel.Performance);
      }
    } else {
      this._devtools = createDevTools();
    }

    // Ctrl+C exit handler
    if (options.exitOnCtrlC !== false) {
      this._keyInput.on("keypress", (key) => {
        if (key.ctrl && key.name === "c") {
          this.destroy();
          process.exit(0);
        }
      });
    }

    // Resize handling
    this._resizeHandler = () => {
      const cols = process.stdout.columns || 80;
      const rows = process.stdout.rows || 24;
      if (cols !== this.width || rows !== this.height) {
        this.resize(cols, rows);
        this.emit(CliRenderEvents.RESIZE, cols, rows);
      }
    };
    process.stdout.on("resize", this._resizeHandler);
  }

  // ── Getters ─────────────────────────────────────────────────────────────────

  get terminalWidth(): number {
    return this.width;
  }

  get terminalHeight(): number {
    return this.height;
  }

  get viewportHeight(): number {
    return this._screenMode === "split-footer" ? this.renderOffset : this.height;
  }

  get screenMode(): ScreenMode {
    return this._screenMode;
  }

  get externalOutputMode(): ExternalOutputMode {
    return this._externalOutputMode;
  }

  get keyInput(): KeyInput {
    return this._keyInput;
  }

  /** Alias for keyInput. */
  get keyHandler(): KeyInput {
    return this._keyInput;
  }

  get version(): string {
    return getVersion();
  }

  get isRunning(): boolean {
    return this.running;
  }

  /** The scene root renderable. All top-level renderables should be added here. */
  get root(): import("../renderables/Box").RootRenderable {
    if (!this._root) {
      const Ctor = getRootRenderable();
      this._root = new Ctor(this);
    }
    return this._root;
  }

  /** The terminal console overlay. */
  get console(): TerminalConsole {
    return this._console;
  }

  /** Current terminal theme mode (light/dark). */
  get themeMode(): ThemeMode {
    return this._themeMode;
  }

  /** Terminal capabilities detected at startup. */
  get capabilities(): TerminalCapabilities {
    return this._capabilities;
  }

  getDiagnostics(): DiagnosticSnapshot {
    return loggerGetDiagnostics();
  }

  get devtools(): DevTools {
    return this._devtools;
  }

  get debugEnabled(): boolean {
    return this.overlay !== null;
  }

  // ── Lifecycle ─────────────────────────────────────────────────────────────────

  /** Start the render loop and keyboard input. */
  start(): void {
    if (this.running) return;
    this.running = true;
    this.paused = false;

    if (this._screenMode === "alternate-screen") {
      this.enterAlternateScreen();
    }
    this._keyInput.start();
    this._startFrameLoop();
  }

  /** Stop the render loop and exit alternate screen. */
  stop(): void {
    if (!this.running) return;
    this.running = false;
    this._stopFrameLoop();
    this._keyInput.stop();

    if (this._screenMode === "split-footer") {
      this.flushExternalOutput();
    } else {
      this.exitAlternateScreen();
    }
  }

  /**
   * Auto-start / toggle mode.
   * Starts if stopped, or pauses/resumes the loop if running.
   */
  auto(): void {
    if (!this.running) {
      this.start();
    } else if (this.paused) {
      this.resume();
    }
  }

  /** Pause the frame loop without stopping input. */
  pause(): void {
    this.paused = true;
  }

  /** Resume a paused frame loop. */
  resume(): void {
    this.paused = false;
  }

  /** Full suspend: stop input and frame loop. */
  suspend(): void {
    this.paused = true;
    this._keyInput.stop();
  }

  /** Full cleanup: stop everything and destroy engine. */
  destroy(): void {
    this._onDestroy?.();
    this._stopFrameLoop();
    try {
      this._keyInput.stop();
    } catch {
      /* ignore */
    }
    if (this._screenMode === "split-footer") {
      try {
        this.flushExternalOutput();
      } catch {
        /* ignore */
      }
    } else {
      try {
        this.exitAlternateScreen();
      } catch {
        /* ignore */
      }
    }
    if (this._resizeHandler) {
      process.stdout.off("resize", this._resizeHandler);
      this._resizeHandler = null;
    }
    this.running = false;
    this.emit(CliRenderEvents.DESTROY);
    try {
      this.engine.shutdown();
    } catch {
      /* ignore */
    }
  }

  // ── Frame loop ────────────────────────────────────────────────────────────────

  private _startFrameLoop(): void {
    const msPerFrame = 1000 / this._targetFps;
    let lastTime = performance.now();

    const loop = () => {
      if (!this.running) return;

      const now = performance.now();
      const dt = now - lastTime;
      lastTime = now;

      if (!this.paused) {
        // Run frame callbacks
        for (const cb of this._frameCallbacks) {
          try {
            cb(dt);
          } catch (err) {
            console.error("Frame callback error:", err);
          }
        }

        // Emit frame event
        this.emit(CliRenderEvents.FRAME, { frameId: this.lastFrameTime });

        // Render
        try {
          this.render();
        } catch {
          /* ignore render errors */
        }
      }

      this._frameInterval = setTimeout(loop, msPerFrame);
    };

    this._frameInterval = setTimeout(loop, msPerFrame);
  }

  private _stopFrameLoop(): void {
    if (this._frameInterval !== null) {
      clearTimeout(this._frameInterval);
      this._frameInterval = null;
    }
  }

  /** Register a frame callback (called every frame before render). */
  setFrameCallback(cb: FrameCallback): void {
    this._frameCallbacks.add(cb);
  }

  /** Remove a previously registered frame callback. */
  removeFrameCallback(cb: FrameCallback): void {
    this._frameCallbacks.delete(cb);
  }

  /** Clear all frame callbacks. */
  clearFrameCallbacks(): void {
    this._frameCallbacks.clear();
  }

  /** Request an immediate render (useful outside the frame loop). */
  requestRender(): void {
    if (!this._pendingRender) {
      this._pendingRender = true;
      setImmediate(() => {
        this._pendingRender = false;
        try {
          this.render();
        } catch {
          /* ignore */
        }
      });
    }
  }

  // ── Visual API ────────────────────────────────────────────────────────────────

  /** Set the terminal window title via OSC 0 sequence. */
  setTerminalTitle(title: string): void {
    process.stdout.write(`\x1b]0;${title}\x07`);
  }

  setBackgroundColor(color: string): void {
    try {
      this.engine.setStyle(this.engine.root(), JSON.stringify({ bg: color }));
    } catch {
      /* ignore */
    }
  }

  dumpHitGrid(): void {
    try {
      this.engine.hitGridDump?.();
    } catch {
      /* ignore */
    }
  }

  copyToClipboardOSC52(text: string): void {
    const encoded = Buffer.from(text).toString("base64");
    process.stdout.write(`\x1b]52;c;${encoded}\x07`);
  }

  clearClipboardOSC52(): void {
    process.stdout.write("\x1b]52;c;!\x07");
  }

  /** Increment the live render counter; starts the renderer if not running. */
  requestLive(): void {
    this._liveCount++;
    if (!this.running) {
      this.start();
    }
  }

  /** Decrement the live render counter. */
  dropLive(): void {
    this._liveCount = Math.max(0, this._liveCount - 1);
  }

  clearSelection(): void {}
  getSelectionContainer(): null {
    return null;
  }
  get hasSelection(): boolean {
    return false;
  }
  setCursorPosition(_x: number, _y: number, _visible?: boolean): void {}

  toggleDebugOverlay(panel: DebugPanel = DebugPanel.Performance): void {
    if (!this.overlay) return;
    const nowVisible = this._devtools.toggle(panel);
    if (!nowVisible && !this.overlay.visible) {
      this.overlay.clear();
      this.renderFull();
    }
  }

  configureDebugOverlay(options: Parameters<OverlayHost["configure"]>[0]): void {
    this.overlay?.configure(options);
  }

  // ── Node management ───────────────────────────────────────────────────────────

  get rootNodeId(): number {
    return this.engine.root();
  }

  getChildrenOf(id: number): number[] {
    return this.nodes.get(id)?.children ?? [];
  }

  setNodeStyle(id: number, style: Style): void {
    this.engine.setStyle(id, JSON.stringify(style));
  }

  setNodeLayout(id: number, layout: LayoutConstraints): void {
    const layoutJson = layoutToEngineJson(layout);
    this.engine.setLayout(id, JSON.stringify(layoutJson));
  }

  insertNodeBefore(parentId: number, childId: number, beforeId: number): void {
    this.engine.insertBefore(beforeId, childId);
    const parentNode = this.nodes.get(parentId);
    if (parentNode) {
      const beforeIdx = parentNode.children.indexOf(beforeId);
      const childIdx = parentNode.children.indexOf(childId);
      if (childIdx !== -1) parentNode.children.splice(childIdx, 1);
      const insertAt = beforeIdx === -1 ? parentNode.children.length : beforeIdx;
      parentNode.children.splice(insertAt, 0, childId);
      const childNode = this.nodes.get(childId);
      if (childNode) childNode.parent = parentId;
    }
  }

  createNode(kind: string): number {
    const id = this.engine.createNode(kind);
    this.nodes.set(id, { parent: null, children: [] });
    return id;
  }

  appendChild(parent: number, child: number): boolean {
    const result = this.engine.appendChild(parent, child);
    if (result) {
      const parentNode = this.nodes.get(parent);
      const childNode = this.nodes.get(child);
      if (parentNode && childNode) {
        parentNode.children.push(child);
        childNode.parent = parent;
      }
    }
    return result;
  }

  removeNode(id: number): void {
    const node = this.nodes.get(id);
    if (node) {
      if (node.parent !== null) {
        const parent = this.nodes.get(node.parent);
        if (parent) {
          parent.children = parent.children.filter((c) => c !== id);
        }
      }
      for (const child of node.children) {
        this.removeNode(child);
      }
      this.nodes.delete(id);
    }
    try {
      this.engine.removeNode(id);
    } catch {
      /* ignore */
    }
  }

  setText(id: number, text: string): void {
    this.engine.setText(id, text);
  }

  clearTree(): void {
    const rootId = this.engine.root();
    const rootNode = this.nodes.get(rootId);
    if (rootNode) {
      for (const child of [...rootNode.children]) {
        this.removeNode(child);
      }
    }
  }

  // ── Screen modes ──────────────────────────────────────────────────────────────

  setScreenMode(mode: ScreenMode, footerHeight?: number): void {
    const oldMode = this._screenMode;
    this._screenMode = mode;

    if (mode === "split-footer") {
      this._externalOutputMode = "capture-stdout";
      this.renderOffset = Math.max(0, this.height - (footerHeight ?? 0));
      this.engine.setScreenMode("split-footer", footerHeight ?? 0);
      if (oldMode === "alternate-screen") {
        this.exitAlternateScreen();
      }
    } else if (mode === "alternate-screen") {
      this._externalOutputMode = "passthrough";
      this.renderOffset = 0;
      this.engine.setScreenMode("alternate-screen");
      this.enterAlternateScreen();
    } else {
      this._externalOutputMode = "passthrough";
      this.renderOffset = 0;
      this.engine.setScreenMode("main-screen");
      if (oldMode === "alternate-screen") {
        this.exitAlternateScreen();
      }
    }
  }

  // ── Rendering ─────────────────────────────────────────────────────────────────

  render(): void {
    const start = performance.now();
    this.engine.beginFrame();
    const frame = this.engine.render();
    this.engine.commitFrame();
    this.writeFrame(frame, performance.now() - start);
  }

  renderFull(): void {
    const start = performance.now();
    this.engine.beginFrame();
    const frame = this.engine.renderFull();
    this.engine.commitFrame();
    this.writeFrame(frame, performance.now() - start);
  }

  private writeFrame(
    frame: { output_data: string; dirty_region_count?: number },
    renderDuration: number,
  ): void {
    if (frame.output_data) {
      const decoded = Buffer.from(frame.output_data, "base64");
      if (this._screenMode === "split-footer") {
        process.stdout.write(`\x1b[1;1H${decoded.toString()}`);
      } else {
        process.stdout.write(decoded);
      }
    }

    if (this._screenMode === "split-footer" && this.externalOutputBuffer.length > 0) {
      this.flushExternalOutput();
    }

    if (this._devtools.enabled) {
      const now = performance.now();
      const dirtyRegionCount = frame.dirty_region_count ?? 0;
      this._devtools.recordFrame({
        duration: this.lastFrameTime > 0 ? now - this.lastFrameTime : renderDuration,
        renderDuration,
        dirtyRegionCount,
      });
      this.lastFrameTime = now;

      if (this.overlay) {
        this.overlay.setDirtyRegionCount(dirtyRegionCount);
        if (this.overlay.visible) {
          this.overlay.paint();
        }
      }
    }
  }

  clearScreen(): void {
    process.stdout.write("\x1b[2J\x1b[H");
  }

  write(text: string): void {
    process.stdout.write(text);
  }

  resize(width: number, height: number): void {
    this.width = width;
    this.height = height;
    if (this._screenMode === "split-footer") {
      this.engine.resize(width, this.viewportHeight);
    } else {
      this.engine.resize(width, height);
    }
  }

  // ── Key bindings ──────────────────────────────────────────────────────────────

  handleKey(sequence: string): string | null {
    return this.keymap.handleKey(sequence);
  }

  addKeyBinding(
    layer: string,
    id: string,
    keys: string,
    command: string,
    description?: string,
    priority = 0,
  ): boolean {
    return this.keymap.addBinding(layer, id, keys, command, description ?? null, priority);
  }

  // ── Private helpers ───────────────────────────────────────────────────────────

  private flushExternalOutput(): void {
    if (this.externalOutputBuffer.length === 0) return;
    const output = this.externalOutputBuffer.join("");
    this.externalOutputBuffer = [];
    process.stdout.write(`\x1b[${this.renderOffset + 1};1H${output}`);
  }

  interceptStdoutWrite = (chunk: string | Uint8Array): boolean => {
    if (this._externalOutputMode === "capture-stdout") {
      this.externalOutputBuffer.push(
        typeof chunk === "string" ? chunk : Buffer.from(chunk).toString("utf8"),
      );
      return true;
    }
    return false;
  };

  private enterAlternateScreen(): void {
    process.stdout.write("\x1b[?1049h\x1b[?25l");
  }

  private exitAlternateScreen(): void {
    process.stdout.write("\x1b[?25h\x1b[?1049l");
  }
}

export async function createCliRenderer(options: CliRendererOptions = {}): Promise<CliRenderer> {
  const renderer = new CliRenderer(options);
  renderer.start();
  return renderer;
}

// ── Utility ───────────────────────────────────────────────────────────────────

function mapCapabilities(caps: TerminalCapabilities): {
  trueColor: boolean;
  kittyKeyboard: boolean;
  mouseSupport: boolean;
  osc52: boolean;
  osc8: boolean;
  pixelSupport: boolean;
  terminalBrand: string;
  terminalSize: { columns: number; rows: number };
  syncUpdate: boolean;
  bracketedPaste: boolean;
  focusEvents: boolean;
  strikethrough: boolean;
  underlineColor: boolean;
  cursorStyle: boolean;
  sixel: boolean;
  inlineImages: boolean;
} {
  return {
    trueColor: caps.true_color,
    kittyKeyboard: caps.kitty_keyboard,
    mouseSupport: caps.mouse,
    osc52: caps.osc52,
    osc8: caps.osc8,
    pixelSupport: caps.sgr_pixel,
    terminalBrand: caps.brand,
    terminalSize: { columns: caps.columns, rows: caps.rows },
    syncUpdate: caps.sync,
    bracketedPaste: caps.bracketed_paste,
    focusEvents: caps.focus_events,
    strikethrough: caps.strikethrough,
    underlineColor: caps.underline_color,
    cursorStyle: caps.cursor_style,
    sixel: caps.sixel,
    inlineImages: caps.inline_images,
  };
}

export { getVersion, detectCapabilities };

/**
 * Convert a TypeScript {@link LayoutConstraints} object to the snake_case JSON
 * shape expected by the Rust engine's `setLayout` command.
 */
function layoutToEngineJson(layout: LayoutConstraints): Record<string, unknown> {
  const j: Record<string, unknown> = {};

  if (layout.flexDirection !== undefined) j.direction = layout.flexDirection;
  if (layout.flexWrap !== undefined) j.flex_wrap = layout.flexWrap;
  if (layout.justifyContent !== undefined) j.justify = layout.justifyContent;
  if (layout.alignItems !== undefined) j.align = layout.alignItems;
  if (layout.alignSelf !== undefined) j.align_self = layout.alignSelf;
  if (layout.flexGrow !== undefined) j.flex_grow = layout.flexGrow;
  if (layout.flexShrink !== undefined) j.flex_shrink = layout.flexShrink;
  if (layout.flexBasis !== undefined) j.flex_basis = String(layout.flexBasis);
  if (layout.display !== undefined) j.display = layout.display;

  if (layout.width !== undefined) j.width = String(layout.width);
  if (layout.height !== undefined) j.height = String(layout.height);
  if (layout.minWidth !== undefined) j.min_width = String(layout.minWidth);
  if (layout.minHeight !== undefined) j.min_height = String(layout.minHeight);
  if (layout.maxWidth !== undefined) j.max_width = String(layout.maxWidth);
  if (layout.maxHeight !== undefined) j.max_height = String(layout.maxHeight);

  // Position and insets
  if (layout.position !== undefined) j.position = layout.position;
  if (layout.top !== undefined) j.top = layout.top;
  if (layout.right !== undefined) j.right = layout.right;
  if (layout.bottom !== undefined) j.bottom = layout.bottom;
  if (layout.left !== undefined) j.left = layout.left;
  if (layout.zIndex !== undefined) j.z_index = layout.zIndex;
  if (layout.overflow !== undefined) j.overflow = layout.overflow;

  // Inset shorthand
  if (layout.inset !== undefined) {
    if (layout.inset.top !== undefined) j.top = layout.inset.top;
    if (layout.inset.right !== undefined) j.right = layout.inset.right;
    if (layout.inset.bottom !== undefined) j.bottom = layout.inset.bottom;
    if (layout.inset.left !== undefined) j.left = layout.inset.left;
  }

  // Padding
  const pt =
    layout.paddingTop ??
    (typeof layout.padding === "number" ? layout.padding : layout.padding?.top);
  const pr =
    layout.paddingRight ??
    (typeof layout.padding === "number" ? layout.padding : layout.padding?.right);
  const pb =
    layout.paddingBottom ??
    (typeof layout.padding === "number" ? layout.padding : layout.padding?.bottom);
  const pl =
    layout.paddingLeft ??
    (typeof layout.padding === "number" ? layout.padding : layout.padding?.left);
  if (pt !== undefined) j.padding_top = pt;
  if (pr !== undefined) j.padding_right = pr;
  if (pb !== undefined) j.padding_bottom = pb;
  if (pl !== undefined) j.padding_left = pl;

  // Margin
  const mt =
    layout.marginTop ?? (typeof layout.margin === "number" ? layout.margin : layout.margin?.top);
  const mr =
    layout.marginRight ??
    (typeof layout.margin === "number" ? layout.margin : layout.margin?.right);
  const mb =
    layout.marginBottom ??
    (typeof layout.margin === "number" ? layout.margin : layout.margin?.bottom);
  const ml =
    layout.marginLeft ?? (typeof layout.margin === "number" ? layout.margin : layout.margin?.left);
  if (mt !== undefined) j.margin_top = mt;
  if (mr !== undefined) j.margin_right = mr;
  if (mb !== undefined) j.margin_bottom = mb;
  if (ml !== undefined) j.margin_left = ml;

  // Gap
  const gapVal = layout.gap;
  if (gapVal !== undefined) {
    if (typeof gapVal === "number") {
      j.gap_row = gapVal;
      j.gap_column = gapVal;
    } else {
      if (gapVal.row !== undefined) j.gap_row = gapVal.row;
      if (gapVal.column !== undefined) j.gap_column = gapVal.column;
    }
  }

  return j;
}
