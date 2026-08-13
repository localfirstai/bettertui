import { EventEmitter } from "node:events";
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import type { LayoutConstraints, Style } from "@bettertui/shared";
import type { DevTools, DevToolsOptions } from "../devtools";
import { createDevTools } from "../devtools";
import { type ConsoleLogEntry, terminalConsoleCache } from "../devtools/consoleCapture";
import { OverlayHost } from "../devtools/overlay/overlayHost";
import { DebugPanel } from "../devtools/overlay/panel.types";
import { env } from "../lib/env";
import { InternalKeyHandler } from "../lib/keyHandler";
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
import { layoutToEngineJson } from "./layoutSerializer";
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
  autoStart?: boolean;
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

/** Interactive terminal console overlay for capturing and inspecting console log output. */
export class TerminalConsole extends EventEmitter {
  private _visible = false;
  private _renderer: CliRenderer | null = null;
  keyBindings: Record<string, unknown> = {};
  onCopySelection?: () => void;

  constructor(renderer?: CliRenderer) {
    super();
    this._renderer = renderer ?? null;
    if (env.BTUI_USE_CONSOLE) {
      terminalConsoleCache.activate();
    }
  }

  attachRenderer(renderer: CliRenderer): void {
    this._renderer = renderer;
  }

  show(): void {
    this._visible = true;
    terminalConsoleCache.activate();
    this.emit("show");
  }

  hide(): void {
    this._visible = false;
    this.emit("hide");
  }

  toggle(): void {
    this._visible = !this._visible;
    if (this._visible) {
      this.show();
    } else {
      this.hide();
    }
  }

  get visible(): boolean {
    return this._visible;
  }

  clear(): void {
    terminalConsoleCache.clearConsole();
  }

  entries(): readonly ConsoleLogEntry[] {
    return terminalConsoleCache.cachedLogs;
  }

  saveLogsToFile(filepath?: string): string | null {
    try {
      const timestamp = Date.now();
      const targetPath = filepath || join(process.cwd(), `_console_${timestamp}.log`);
      const formatArg = (arg: unknown) =>
        typeof arg === "object" && arg !== null ? JSON.stringify(arg) : String(arg);
      const logs = terminalConsoleCache.cachedLogs
        .map(
          ([date, level, args]) =>
            `[${date.toISOString()}] [${level}] ${args.map(formatArg).join(" ")}`,
        )
        .join("\n");
      writeFileSync(targetPath, logs, "utf8");
      return targetPath;
    } catch {
      return null;
    }
  }
}

export type ThemeMode = "light" | "dark";

type FrameCallback = (deltaTime: number) => void | Promise<void>;

// Lazy import to avoid circular at module level
let _Root: typeof import("../renderables/Box").Root | undefined;

function getRoot() {
  if (!_Root) {
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    _Root = require("../renderables/Box").Root;
  }
  if (!_Root) throw new Error("Root could not be loaded");
  return _Root;
}

export class CliRenderer extends EventEmitter {
  private engine: NapiEngine;
  private keymap: NapiKeymap;
  private _keyInput: KeyInput;
  private _keyDispatch: InternalKeyHandler;
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
  private _frameId = 0;
  private _frameInterval: ReturnType<typeof setTimeout> | null = null;
  private _frameCallbacks: Set<FrameCallback> = new Set();
  private _lifecyclePasses: Set<() => void> = new Set();
  private _root: import("../renderables/Box").Root | null = null;
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
    this._targetFps = options.targetFps ?? 60;

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
    this._keyDispatch = new InternalKeyHandler();
    this._onDestroy = options.onDestroy;

    // Bridge raw key/paste events from _keyInput through the priority dispatcher.
    // Global handlers registered via renderer.keyHandler.on() (tier-1) fire first
    // and can call key.preventDefault() / key.stopPropagation() before the focused
    // widget's handler (tier-2, registered via onInternal()) sees the event.
    this._keyInput.on("keypress", (key) => this._keyDispatch.processParsedKey(key));
    this._keyInput.on("keyrelease", (key) => this._keyDispatch.processParsedKey(key));
    this._keyInput.on("paste", (event) =>
      this._keyDispatch.processPaste(event.bytes, event.metadata),
    );

    const rootId = this.engine.root();
    this.nodes.set(rootId, { parent: null, children: [] });
    this.setNodeLayout(rootId, { width: "100%", height: "100%" });

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

    this._console.attachRenderer(this);

    // Ctrl+C exit handler & Debug shortcut handlers (tier-1 global listener)
    this._keyDispatch.on("keypress", (key) => {
      if (options.exitOnCtrlC !== false && key.ctrl && key.name === "c") {
        this.destroy();
        process.exit(0);
      }

      // Backtick (` ` `) or Ctrl+F12 toggles console overlay
      if (key.name === "`" || (key.ctrl && key.name === "f12")) {
        this._console.toggle();
      }

      // F12 or Ctrl+Shift+D toggles performance debug overlay
      if ((key.name === "f12" && !key.ctrl) || (key.ctrl && key.shift && key.name === "d")) {
        this.toggleDebugOverlay();
      }
    });

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

    if (options.autoStart !== false) {
      this.start();
    }
  }

  // ── Getters ─────────────────────────────────────────────────────────────────

  get frameId(): number {
    return this._frameId;
  }

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

  /**
   * Two-tier priority key dispatcher.
   *
   * - `.on("keypress", fn)` → tier-1 global handler (fires before any focused
   *   widget).  Can call `key.preventDefault()` / `key.stopPropagation()`.
   * - `.onInternal("keypress", fn)` → tier-2 renderable handler (used by
   *   focusable widgets; only fires when no global handler stopped propagation).
   *
   * Example code that needs to intercept keys before the focused widget should
   * use `renderer.keyHandler.on(...)`.  Widgets must use
   * `renderer.keyHandler.onInternal(...)` inside `focus()`.
   */
  get keyHandler(): InternalKeyHandler {
    return this._keyDispatch;
  }

  get version(): string {
    return getVersion();
  }

  get isRunning(): boolean {
    return this.running;
  }

  /** The scene root renderable. All top-level renderables should be added here. */
  get root(): import("../renderables/Box").Root {
    if (!this._root) {
      const Ctor = getRoot();
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

      const frameStart = performance.now();
      const dt = frameStart - lastTime;
      lastTime = frameStart;

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
        this._frameId++;
        this.emit(CliRenderEvents.FRAME, { frameId: this._frameId });

        // Run lifecycle passes (e.g. Text syncing its node tree to the engine)
        for (const pass of this._lifecyclePasses) {
          try {
            pass();
          } catch (err) {
            console.error("Lifecycle pass error:", err);
          }
        }

        // Render
        try {
          this.render();
        } catch {
          /* ignore render errors */
        }
      }

      const processingTime = performance.now() - frameStart;
      const delay = Math.max(0, msPerFrame - processingTime);
      if (this.running) {
        this._frameInterval = setTimeout(loop, Math.round(delay));
      }
    };

    this._frameInterval = setTimeout(loop, 0);
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

  /** Register a function to be called once per frame before render (lifecycle pass). */
  registerLifecyclePass(fn: () => void): void {
    this._lifecyclePasses.add(fn);
  }

  /** Unregister a previously registered lifecycle pass function. */
  unregisterLifecyclePass(fn: () => void): void {
    this._lifecyclePasses.delete(fn);
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
      this.engine.setBackgroundColor?.(color);
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
      if (oldMode === "split-footer") {
        process.stdout.write("\x1b[2J\x1b[H");
      }
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
