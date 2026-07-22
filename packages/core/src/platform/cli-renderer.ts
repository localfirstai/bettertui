import { EventEmitter } from "node:events";
import type { LayoutConstraints, Style } from "@bettertui/shared";
import type { DevTools, DevToolsOptions } from "../devtools";
import { createDevTools } from "../devtools";
import { OverlayHost } from "../devtools/overlay/overlayHost";
import { DebugPanel } from "../devtools/overlay/panel.types";
import { StdinParser } from "../lib/stdin-parser";
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
import type { ExternalOutputMode, ScreenMode } from "./types";

export interface KeyEvent {
  name: string;
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
  meta: boolean;
  sequence: string;
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
  /**
   * Enable the in-core debug overlay/tooling (§6.4). Pass `true` for defaults or
   * a {@link DevToolsOptions} object to configure it. When omitted/false, a
   * zero-cost no-op DevTools is used (Principle #2). Env vars `BTUI_DEBUG` and
   * `BTUI_SHOW_STATS` force it on regardless.
   */
  debug?: boolean | DevToolsOptions;
}

type KeyInputEvents = {
  keypress: [KeyEvent];
};

export class KeyInput extends EventEmitter<KeyInputEvents> {
  private stdinParser: StdinParser;
  private rawMode = false;
  private readonly onDataBound: (data: Buffer) => void;

  constructor() {
    super();
    this.stdinParser = new StdinParser({
      useKittyKeyboard: false, // Keep disabled to match old behavior
      onTimeoutFlush: () => {
        this.drain();
      },
    });
    this.onDataBound = this.onData.bind(this);
  }

  start(): void {
    const stdin = process.stdin;
    if (stdin.setRawMode && !this.rawMode) {
      stdin.setRawMode(true);
      this.rawMode = true;
    }
    stdin.resume();
    stdin.on("data", this.onDataBound);
  }

  stop(): void {
    process.stdin.off("data", this.onDataBound);
    if (this.rawMode && process.stdin.setRawMode) {
      process.stdin.setRawMode(false);
      this.rawMode = false;
    }
    process.stdin.pause();
    this.stdinParser.reset();
  }

  private onData(data: Buffer): void {
    this.stdinParser.push(new Uint8Array(data.buffer, data.byteOffset, data.byteLength));
    this.drain();
  }

  private drain(): void {
    this.stdinParser.drain((event) => {
      if (event.type === "key") {
        this.emit("keypress", {
          name: event.key.name,
          ctrl: event.key.ctrl,
          shift: event.key.shift,
          alt: event.key.option, // option acts as alt
          meta: event.key.meta,
          sequence: event.key.sequence,
        });
      }
    });
  }
}

export class CliRenderer {
  private engine: NapiEngine;
  private keymap: NapiKeymap;
  private _keyInput: KeyInput;
  private capabilities: TerminalCapabilities;
  private width: number;
  private height: number;
  private renderOffset: number;
  private _screenMode: ScreenMode;
  private _externalOutputMode: ExternalOutputMode;
  private externalOutputBuffer: string[] = [];
  private nodes: Map<number, { parent: number | null; children: number[] }> = new Map();
  private running = false;
  private _devtools: DevTools;
  private overlay: OverlayHost | null = null;
  private lastFrameTime = 0;

  constructor(options: CliRendererOptions = {}) {
    if (options.logger) {
      // Default `dev` from NODE_ENV so file logging lands in the repo-root
      // `logs/` dir during development but stays off in production unless the
      // caller passes an explicit `file` path. An explicit `dev` still wins.
      loggerInit({ dev: process.env.NODE_ENV !== "production", ...options.logger });
    }
    this.capabilities = detectCapabilities();
    this.width = options.width ?? this.capabilities.columns;
    this.height = options.height ?? this.capabilities.rows;

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

    const rootId = this.engine.root();
    this.nodes.set(rootId, { parent: null, children: [] });

    // ─── Debug tooling (§6.4) ────────────────────────────────────────────────
    // Env vars mirror OpenTUI's OTUI_SHOW_STATS: BTUI_DEBUG / BTUI_SHOW_STATS
    // force the overlay on regardless of the option.
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
      this._devtools.updateCapabilities(mapCapabilities(this.capabilities));
      this.overlay = new OverlayHost(this, this._devtools);
      // Env-forced or bare `true` starts with the performance panel visible so
      // the overlay is discoverable out of the box (like OTUI_SHOW_STATS).
      if (envDebug || debugOption === true) {
        this._devtools.show(DebugPanel.Performance);
      }
    } else {
      // Zero-cost no-op keeps `renderer.devtools` always defined (Principle #2).
      this._devtools = createDevTools();
    }
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

  get keyHandler(): KeyInput {
    return this._keyInput;
  }

  get version(): string {
    return getVersion();
  }

  get isRunning(): boolean {
    return this.running;
  }

  /**
   * Snapshot the engine's diagnostic counters (render calls, frame times, cache
   * hit/miss, etc.). Returns zeroed counters if the logger was never initialized.
   */
  getDiagnostics(): DiagnosticSnapshot {
    return loggerGetDiagnostics();
  }

  /**
   * The DevTools facade. Always defined — a zero-cost no-op instance unless the
   * `debug` option (or `BTUI_DEBUG`/`BTUI_SHOW_STATS`) enabled it.
   */
  get devtools(): DevTools {
    return this._devtools;
  }

  /** Whether the debug overlay is active (enabled via option/env). */
  get debugEnabled(): boolean {
    return this.overlay !== null;
  }

  /**
   * Toggle a debug panel (defaults to the performance panel). No-op unless the
   * overlay was enabled. When toggling the last panel off, forces a full redraw
   * so the vacated region is repainted cleanly.
   */
  toggleDebugOverlay(panel: DebugPanel = DebugPanel.Performance): void {
    if (!this.overlay) return;
    const nowVisible = this._devtools.toggle(panel);
    if (!nowVisible && !this.overlay.visible) {
      // Last panel hidden: wipe the overlay region and repaint the full frame.
      this.overlay.clear();
      this.renderFull();
    }
  }

  /** Configure the debug overlay (corner, panel width, body rows). No-op when disabled. */
  configureDebugOverlay(options: Parameters<OverlayHost["configure"]>[0]): void {
    this.overlay?.configure(options);
  }

  /**
   * Returns the native ID of the engine's root node.
   * Used by the React adapter to append top-level elements to the engine root.
   */
  get rootNodeId(): number {
    return this.engine.root();
  }

  /**
   * Returns the native IDs of the direct children of a node.
   * Used by the React adapter to iterate and clear children.
   */
  getChildrenOf(id: number): number[] {
    return this.nodes.get(id)?.children ?? [];
  }

  /**
   * Set visual style properties on a native node.
   * Called by the React reconciler after prop commits.
   */
  setNodeStyle(id: number, style: Style): void {
    this.engine.setStyle(id, JSON.stringify(style));
  }

  /**
   * Set layout properties on a native node.
   * Called by the React reconciler after prop commits.
   */
  setNodeLayout(id: number, layout: LayoutConstraints): void {
    const layoutJson = layoutToEngineJson(layout);
    this.engine.setLayout(id, JSON.stringify(layoutJson));
  }

  /**
   * Insert `childId` immediately before `beforeId` in `parentId`'s children.
   * Uses processCommands with the raw u64 node IDs as decimal strings.
   */
  insertNodeBefore(parentId: number, childId: number, beforeId: number): void {
    const cmds = [{ type: "InsertBefore", reference: String(beforeId), child: String(childId) }];
    this.engine.processCommands(JSON.stringify(cmds));
    // Keep our TS node tracking in sync
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

  start(): void {
    if (this.running) return;
    this.running = true;

    if (this._screenMode === "alternate-screen") {
      this.enterAlternateScreen();
    }
    this._keyInput.start();
  }

  stop(): void {
    if (!this.running) return;
    this.running = false;
    this._keyInput.stop();

    if (this._screenMode === "split-footer") {
      this.flushExternalOutput();
    } else {
      this.exitAlternateScreen();
    }
    this.engine.shutdown();
  }

  /** Flush captured external output above the footer region. */
  private flushExternalOutput(): void {
    if (this.externalOutputBuffer.length === 0) return;
    // Move cursor above footer, replay output, restore footer position
    const output = this.externalOutputBuffer.join("");
    this.externalOutputBuffer = [];
    process.stdout.write(`\x1b[${this.renderOffset + 1};1H${output}`);
  }

  /** Capture or passthrough stdout writes depending on mode. */
  interceptStdoutWrite = (chunk: string | Uint8Array): boolean => {
    if (this._externalOutputMode === "capture-stdout") {
      this.externalOutputBuffer.push(
        typeof chunk === "string" ? chunk : Buffer.from(chunk).toString("utf8"),
      );
      return true;
    }
    return false;
  };

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
    this.engine.removeNode(id);
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

  /** Switch screen mode at runtime. */
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

  /**
   * Shared frame write path for {@link render} and {@link renderFull}. Writes
   * the engine's ANSI output, flushes captured external output, then (if the
   * debug overlay is enabled) records the frame and composites the overlay on
   * top. The overlay is written *after* engine output and outside the engine's
   * dirty-diff, so it self-clears vacated rows (see {@link OverlayHost.paint}).
   */
  private writeFrame(
    frame: { output_data: string; dirty_region_count?: number },
    renderDuration: number,
  ): void {
    if (frame.output_data) {
      const decoded = Buffer.from(frame.output_data, "base64");
      if (this._screenMode === "split-footer") {
        // Paint only in the viewport region (above footer)
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
      // Keep footer height stable, adjust viewport
      this.engine.resize(width, this.viewportHeight);
    } else {
      this.engine.resize(width, height);
    }
  }

  private enterAlternateScreen(): void {
    process.stdout.write("\x1b[?1049h\x1b[?25l");
  }

  private exitAlternateScreen(): void {
    process.stdout.write("\x1b[?25h\x1b[?1049l");
  }

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
}

export async function createCliRenderer(options: CliRendererOptions = {}): Promise<CliRenderer> {
  return new CliRenderer(options);
}

/**
 * Map the engine's snake_case {@link TerminalCapabilities} to the camelCase
 * shape the DevTools CapabilityInspector expects.
 */
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
  if (layout.flexGrow !== undefined) j.flex_grow = layout.flexGrow;
  if (layout.flexShrink !== undefined) j.flex_shrink = layout.flexShrink;

  if (layout.width !== undefined) j.width = String(layout.width);
  if (layout.height !== undefined) j.height = String(layout.height);
  if (layout.minWidth !== undefined) j.min_width = String(layout.minWidth);
  if (layout.minHeight !== undefined) j.min_height = String(layout.minHeight);
  if (layout.maxWidth !== undefined) j.max_width = String(layout.maxWidth);
  if (layout.maxHeight !== undefined) j.max_height = String(layout.maxHeight);

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
