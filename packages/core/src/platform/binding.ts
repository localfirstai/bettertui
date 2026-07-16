import { createRequire } from "node:module";
import type { BindingInfo } from "./types.js";

const require = createRequire(import.meta.url);

interface NativeModule {
  NativeEngine: new (width: number, height: number) => NativeEngine;
  NativeEventBus: new () => NativeEventBus;
  NativeFocusManager: new () => NativeFocusManager;
  NativeKeymap: new () => NativeKeymap;
  NativeScheduler: new (fps?: number | null) => NativeScheduler;
  NativeTextEngine: new (text?: string | null) => NativeTextEngine;
  NativeSpanFeed: new (options?: NativeSpanFeedOptions | null) => NativeSpanFeed;
  NativeHitGrid: new (width: number, height: number) => NativeHitGrid;
  TerminalCapabilities: TerminalCapabilities;
  detectCapabilities: () => TerminalCapabilities;
  getVersion: () => string;
  createDarkTheme: () => NapiTheme;
  createLightTheme: () => NapiTheme;
}

interface NativeEngine {
  processCommands(commandsJson: string): string;
  beginFrame(): void;
  commitFrame(): void;
  render(): string;
  renderFull(): string;
  setScreenMode(mode: string, footerHeight?: number | null): void;
  resize(width: number, height: number): void;
  nodeCount(): number;
  frameCount(): number;
  printTree(): string;
  validate(): boolean;
  shutdown(): void;
  setStyle(id: number, styleJson: string): void;
  setLayout(id: number, layoutJson: string): void;
  getNode(id: number): string;
  treeSummary(): string;
  root(): number;
  createNode(kind: string): number;
  appendChild(parent: number, child: number): boolean;
  removeNode(id: number): void;
  setText(id: number, text: string): void;
  hitGridCheck(x: number, y: number): number;
  hitGridIsDirty(): boolean;
  hitGridClearCurrent(): void;
  hitGridPushScissor(x: number, y: number, width: number, height: number): void;
  hitGridPopScissor(): void;
  hitGridAddCurrentClipped(x: number, y: number, width: number, height: number, id: number): void;
  hitGridDump(): string;
}

interface NativeEventBus {
  pushKey(key: string, ctrl: boolean, shift: boolean, alt: boolean): void;
  pushMouse(button: string, x: number, y: number): void;
  pushMouseMotion(x: number, y: number): void;
  pushPaste(text: string): void;
  pushResize(width: number, height: number, prevWidth: number, prevHeight: number): void;
  drain(): string;
  len(): number;
  isEmpty(): boolean;
  clear(): void;
}

interface NativeFocusManager {
  focus(id: number): boolean;
  blur(id: number): boolean;
  blurCurrent(): boolean;
  focused(): number;
  isFocused(id: number): boolean;
  traverse(direction: string): string;
  focusOrder(): number[];
  clear(): void;
}

interface NativeTextEngine {
  insertChar(ch: string): void;
  insertStr(text: string): void;
  deleteChar(): void;
  getText(): string;
  clear(): void;
  canUndo(): boolean;
  canRedo(): boolean;
  undo(): boolean;
  redo(): boolean;
  cursorLeft(): void;
  cursorRight(): void;
  cursorPosition(): number;
  setCursorPosition(pos: number): void;
  length(): number;
  lineCount(): number;
  isEmpty(): boolean;
  wordCount(): number;
}

interface NativeScheduler {
  requestFrame(): void;
  beginFrame(): boolean;
  endFrame(): void;
  isIdle(): boolean;
  frameCount(): number;
  fps(): number;
  shouldRender(): boolean;
  requestRenderCoalesced(): void;
  requestRenderImmediate(): void;
  hasScheduledFrame(): boolean;
  isRendering(): boolean;
  beginRender(): void;
  endRender(): boolean;
}

interface NativeKeymap {
  addBinding(
    layer: string,
    id: string,
    keys: string,
    command: string,
    description: string | null,
    priority: number,
  ): boolean;
  handleKey(key: string): string;
  hasPending(): boolean;
  clearPending(): void;
  setMode(mode: string): void;
  currentMode(): string;
  clearMode(): void;
  removeLayer(name: string): boolean;
  setChordTimeout(ms: number): void;
  chordTimeout(): number;
  pendingKeys(): string[];
  activeBindings(): string;
  allBindings(): string;
  commandHistory(): string[];
  clearHistory(): void;
  parseKey(keyStr: string): string;
  parseSequence(keyStr: string): string[];
}

export interface TerminalCapabilities {
  brand: string;
  true_color: boolean;
  kitty_keyboard: boolean;
  csi_u: boolean;
  bracketed_paste: boolean;
  focus_events: boolean;
  mouse: boolean;
  osc52: boolean;
  osc8: boolean;
  sync: boolean;
  sgr_pixel: boolean;
  underline_color: boolean;
  strikethrough: boolean;
  cursor_style: boolean;
  alternate_scroll: boolean;
  inline_images: boolean;
  sixel: boolean;
  columns: number;
  rows: number;
}

let native: NativeModule;

try {
  native = require("./bettertui_engine.node");
} catch {
  native = require("../../dist/bettertui_engine.node");
}

export interface CommandResult {
  success: number;
  errors: string[];
  id_mappings: Array<{ temp: number; real: number }>;
}

export interface RenderResult {
  output_data: string;
  width: number;
  height: number;
  dirty_region_count: number;
}

export interface NapiEngine {
  processCommands(commandsJson: string): CommandResult;
  beginFrame(): void;
  commitFrame(): void;
  render(): RenderResult;
  renderFull(): RenderResult;
  resize(width: number, height: number): void;
  setScreenMode(mode: string, footerHeight?: number | null): void;
  setStyle(id: number, styleJson: string): void;
  setLayout(id: number, layoutJson: string): void;
  getNode(id: number): string;
  treeSummary(): string;
  nodeCount(): number;
  frameCount(): number;
  createNode(kind: string): number;
  appendChild(parent: number, child: number): boolean;
  removeNode(id: number): void;
  setText(id: number, text: string): void;
  root(): number;
  validate(): boolean;
  printTree(): string;
  shutdown(): void;
  hitGridCheck(x: number, y: number): number;
  hitGridIsDirty(): boolean;
  hitGridClearCurrent(): void;
  hitGridPushScissor(x: number, y: number, width: number, height: number): void;
  hitGridPopScissor(): void;
  hitGridAddCurrentClipped(x: number, y: number, width: number, height: number, id: number): void;
  hitGridDump(): string;
}

export interface NapiEventBus {
  pushKey(key: string, ctrl: boolean, shift: boolean, alt: boolean): void;
  pushMouse(button: string, x: number, y: number): void;
  pushMouseMotion(x: number, y: number): void;
  pushPaste(text: string): void;
  pushResize(width: number, height: number, prevWidth: number, prevHeight: number): void;
  drain(): string;
  len(): number;
  isEmpty(): boolean;
  clear(): void;
}

export interface NapiFocusManager {
  focus(id: number): boolean;
  blur(id: number): boolean;
  blurCurrent(): boolean;
  focused(): number;
  isFocused(id: number): boolean;
  traverse(direction: string): number;
  focusOrder(): number[];
  clear(): void;
}

export interface NapiTextEngine {
  insertChar(ch: string): void;
  insertStr(text: string): void;
  deleteChar(): void;
  cursorLeft(): void;
  cursorRight(): void;
  getText(): string;
  cursorPosition(): number;
  setCursorPosition(pos: number): void;
  length(): number;
  lineCount(): number;
  isEmpty(): boolean;
  wordCount(): number;
  canUndo(): boolean;
  canRedo(): boolean;
  undo(): boolean;
  redo(): boolean;
  clear(): void;
}

export interface NapiScheduler {
  requestFrame(): void;
  beginFrame(): boolean;
  endFrame(): void;
  shouldRender(): boolean;
  isIdle(): boolean;
  frameCount(): number;
  fps(): number;
  requestRenderCoalesced(): void;
  requestRenderImmediate(): void;
  hasScheduledFrame(): boolean;
  isRendering(): boolean;
  beginRender(): void;
  endRender(): boolean;
}

export interface NapiKeymap {
  addBinding(
    layer: string,
    id: string,
    keys: string,
    command: string,
    description: string | null,
    priority: number,
  ): boolean;
  handleKey(key: string): string;
  hasPending(): boolean;
  clearPending(): void;
  setMode(mode: string): void;
  currentMode(): string;
  clearMode(): void;
  removeLayer(name: string): boolean;
  setChordTimeout(ms: number): void;
  chordTimeout(): number;
  pendingKeys(): string[];
  activeBindings(): BindingInfo[];
  allBindings(): BindingInfo[];
  commandHistory(): string[];
  clearHistory(): void;
  parseKey(keyStr: string): string;
  parseSequence(keyStr: string): string[];
}

class EngineWrapper implements NapiEngine {
  constructor(private engine: NativeEngine) {}
  processCommands(commandsJson: string): CommandResult {
    return JSON.parse(this.engine.processCommands(commandsJson));
  }
  beginFrame(): void {
    this.engine.beginFrame();
  }
  commitFrame(): void {
    this.engine.commitFrame();
  }
  render(): RenderResult {
    return JSON.parse(this.engine.render());
  }
  renderFull(): RenderResult {
    return JSON.parse(this.engine.renderFull());
  }
  resize(width: number, height: number): void {
    this.engine.resize(width, height);
  }
  setScreenMode(mode: string, footerHeight?: number | null): void {
    this.engine.setScreenMode(mode, footerHeight ?? null);
  }
  setStyle(id: number, styleJson: string): void {
    this.engine.setStyle(id, styleJson);
  }
  setLayout(id: number, layoutJson: string): void {
    this.engine.setLayout(id, layoutJson);
  }
  getNode(id: number): string {
    return this.engine.getNode(id);
  }
  treeSummary(): string {
    return this.engine.treeSummary();
  }
  nodeCount(): number {
    return this.engine.nodeCount();
  }
  frameCount(): number {
    return this.engine.frameCount();
  }
  createNode(kind: string): number {
    return this.engine.createNode(kind);
  }
  appendChild(parent: number, child: number): boolean {
    return this.engine.appendChild(parent, child);
  }
  removeNode(id: number): void {
    this.engine.removeNode(id);
  }
  setText(id: number, text: string): void {
    this.engine.setText(id, text);
  }
  root(): number {
    return this.engine.root();
  }
  validate(): boolean {
    return this.engine.validate();
  }
  printTree(): string {
    return this.engine.printTree();
  }
  shutdown(): void {
    this.engine.shutdown();
  }
  hitGridCheck(x: number, y: number): number {
    return this.engine.hitGridCheck(x, y);
  }
  hitGridIsDirty(): boolean {
    return this.engine.hitGridIsDirty();
  }
  hitGridClearCurrent(): void {
    this.engine.hitGridClearCurrent();
  }
  hitGridPushScissor(x: number, y: number, width: number, height: number): void {
    this.engine.hitGridPushScissor(x, y, width, height);
  }
  hitGridPopScissor(): void {
    this.engine.hitGridPopScissor();
  }
  hitGridAddCurrentClipped(x: number, y: number, width: number, height: number, id: number): void {
    this.engine.hitGridAddCurrentClipped(x, y, width, height, id);
  }
  hitGridDump(): string {
    return this.engine.hitGridDump();
  }
}

export function createEngine(width = 80, height = 24): NapiEngine {
  return new EngineWrapper(new native.NativeEngine(width, height));
}

export function createEventBus(): NapiEventBus {
  const bus = new native.NativeEventBus();
  return {
    pushKey: (key, ctrl, shift, alt) => bus.pushKey(key, ctrl, shift, alt),
    pushMouse: (button, x, y) => bus.pushMouse(button, x, y),
    pushMouseMotion: (x, y) => bus.pushMouseMotion(x, y),
    pushPaste: (text) => bus.pushPaste(text),
    pushResize: (w, h, pw, ph) => bus.pushResize(w, h, pw, ph),
    drain: () => bus.drain(),
    len: () => bus.len(),
    isEmpty: () => bus.isEmpty(),
    clear: () => bus.clear(),
  };
}

export function createFocusManager(): NapiFocusManager {
  const fm = new native.NativeFocusManager();
  return {
    focus: (id) => fm.focus(id),
    blur: (id) => fm.blur(id),
    blurCurrent: () => fm.blurCurrent(),
    focused: () => fm.focused(),
    isFocused: (id) => fm.isFocused(id),
    traverse: (dir) => {
      const result = fm.traverse(dir);
      return result === "null" ? 0 : Number.parseInt(result, 10);
    },
    focusOrder: () => fm.focusOrder(),
    clear: () => fm.clear(),
  };
}

export function createTextEngine(text?: string): NapiTextEngine {
  const te = new native.NativeTextEngine(text ?? null);
  return {
    insertChar: (ch) => te.insertChar(ch),
    insertStr: (t) => te.insertStr(t),
    deleteChar: () => te.deleteChar(),
    getText: () => te.getText(),
    clear: () => te.clear(),
    canUndo: () => te.canUndo(),
    canRedo: () => te.canRedo(),
    undo: () => te.undo(),
    redo: () => te.redo(),
    cursorLeft: () => te.cursorLeft(),
    cursorRight: () => te.cursorRight(),
    cursorPosition: () => te.cursorPosition(),
    setCursorPosition: (pos) => te.setCursorPosition(pos),
    length: () => te.length(),
    lineCount: () => te.lineCount(),
    isEmpty: () => te.isEmpty(),
    wordCount: () => te.wordCount(),
  };
}

export function createScheduler(fps?: number): NapiScheduler {
  const sched = new native.NativeScheduler(fps ?? null);
  return {
    requestFrame: () => sched.requestFrame(),
    beginFrame: () => sched.beginFrame(),
    endFrame: () => sched.endFrame(),
    isIdle: () => sched.isIdle(),
    frameCount: () => sched.frameCount(),
    fps: () => sched.fps(),
    shouldRender: () => sched.shouldRender(),
    requestRenderCoalesced: () => sched.requestRenderCoalesced(),
    requestRenderImmediate: () => sched.requestRenderImmediate(),
    hasScheduledFrame: () => sched.hasScheduledFrame(),
    isRendering: () => sched.isRendering(),
    beginRender: () => sched.beginRender(),
    endRender: () => sched.endRender(),
  };
}

export function createKeymap(): NapiKeymap {
  const km = new native.NativeKeymap();
  return {
    addBinding: (layer, id, keys, command, desc, priority) =>
      km.addBinding(layer, id, keys, command, desc, priority),
    handleKey: (key) => km.handleKey(key),
    hasPending: () => km.hasPending(),
    clearPending: () => km.clearPending(),
    setMode: (mode) => km.setMode(mode),
    currentMode: () => km.currentMode(),
    clearMode: () => km.clearMode(),
    removeLayer: (name) => km.removeLayer(name),
    setChordTimeout: (ms) => km.setChordTimeout(ms),
    chordTimeout: () => km.chordTimeout(),
    pendingKeys: () => km.pendingKeys(),
    activeBindings: () => JSON.parse(km.activeBindings()),
    allBindings: () => JSON.parse(km.allBindings()),
    commandHistory: () => km.commandHistory(),
    clearHistory: () => km.clearHistory(),
    parseKey: (keyStr) => km.parseKey(keyStr),
    parseSequence: (keyStr) => km.parseSequence(keyStr),
  };
}

export function detectCapabilities(): TerminalCapabilities {
  return native.detectCapabilities();
}

export function getVersion(): string {
  return native.getVersion();
}

export function getNativePackageName(): string {
  return "bettertui_engine";
}

export interface HighlightSegment {
  text: string;
  fg: string | null;
  bg: string | null;
  bold: boolean | null;
  italic: boolean | null;
  underline: boolean | null;
  dim: boolean | null;
  strikethrough: boolean | null;
}

export function highlightCode(_code: string, _language: string): HighlightSegment[][] {
  return [];
}

export interface NapiWidgetHost {
  widgetCount(): number;
}

export function createWidgetHost(): NapiWidgetHost {
  return {
    widgetCount: () => 0,
  };
}

// ─── NativeSpanFeed ─────────────────────────────────────────────────────────────

export interface NativeSpanFeedOptions {
  chunkSize?: number;
  initialChunks?: number;
  maxBytes?: number;
  /** 0 = grow, 1 = block */
  growthPolicy?: number;
  autoCommitOnFull?: boolean;
  spanQueueCapacity?: number;
}

interface NativeSpanFeed {
  write(data: Buffer): number;
  drainSpans(out: Buffer): number;
  close(): void;
  reset(): void;
  pendingSpans(): number;
  pendingBytes(): number;
  isClosed(): boolean;
  isBackpressured(): boolean;
  stats(): NapiSpanFeedStats;
  markConsumed(chunkIndex: number): void;
}

interface NativeHitGrid {
  resize(width: number, height: number): void;
  add(x: number, y: number, width: number, height: number, id: number): void;
  check(x: number, y: number): number;
  clearNext(): void;
  clearCurrent(): void;
  swap(): boolean;
  isDirty(): boolean;
  dimensions(): string;
  pushScissor(x: number, y: number, width: number, height: number): void;
  popScissor(): void;
  clearScissors(): void;
}

export interface NapiSpanFeedStats {
  bytesWritten: number;
  spansCommitted: number;
  chunks: number;
  pendingSpans: number;
}

export class NapiSpanFeed {
  constructor(private feed: NativeSpanFeed) {}

  write(data: Buffer): number {
    return this.feed.write(data);
  }

  drainSpans(out: Buffer): number {
    return this.feed.drainSpans(out);
  }

  close(): void {
    this.feed.close();
  }

  reset(): void {
    this.feed.reset();
  }

  get pendingSpans(): number {
    return this.feed.pendingSpans();
  }

  get pendingBytes(): number {
    return this.feed.pendingBytes();
  }

  get isClosed(): boolean {
    return this.feed.isClosed();
  }

  get isBackpressured(): boolean {
    return this.feed.isBackpressured();
  }

  stats(): NapiSpanFeedStats {
    return this.feed.stats();
  }

  markConsumed(chunkIndex: number): void {
    this.feed.markConsumed(chunkIndex);
  }
}

export function createSpanFeed(options?: NativeSpanFeedOptions): NapiSpanFeed {
  const nativeOptions = options
    ? {
        chunkSize: options.chunkSize ?? 65536,
        initialChunks: options.initialChunks ?? 2,
        maxBytes: options.maxBytes ?? 0,
        growthPolicy: options.growthPolicy ?? 0,
        autoCommitOnFull: options.autoCommitOnFull ?? true,
        spanQueueCapacity: options.spanQueueCapacity ?? 4096,
      }
    : null;
  return new NapiSpanFeed(new native.NativeSpanFeed(nativeOptions));
}

export class NapiHitGrid {
  constructor(private grid: NativeHitGrid) {}

  resize(width: number, height: number): void {
    this.grid.resize(width, height);
  }

  add(x: number, y: number, width: number, height: number, id: number): void {
    this.grid.add(x, y, width, height, id);
  }

  check(x: number, y: number): number {
    return this.grid.check(x, y);
  }

  clearNext(): void {
    this.grid.clearNext();
  }

  clearCurrent(): void {
    this.grid.clearCurrent();
  }

  swap(): boolean {
    return this.grid.swap();
  }

  get isDirty(): boolean {
    return this.grid.isDirty();
  }

  pushScissor(x: number, y: number, width: number, height: number): void {
    this.grid.pushScissor(x, y, width, height);
  }

  popScissor(): void {
    this.grid.popScissor();
  }

  clearScissors(): void {
    this.grid.clearScissors();
  }
}

export function createHitGrid(width: number, height: number): NapiHitGrid {
  return new NapiHitGrid(new native.NativeHitGrid(width, height));
}

// ─── Theme Functions ─────────────────────────────────────────────────────────────

export interface NapiThemeColors {
  background: string;
  surface: string;
  surfaceHigh: string;
  surfaceLow: string;
  primary: string;
  primaryForeground: string;
  secondary: string;
  secondaryForeground: string;
  text: string;
  textMuted: string;
  textDim: string;
  border: string;
  borderFocused: string;
  accent: string;
  accentForeground: string;
  error: string;
  warning: string;
  success: string;
  info: string;
  scrollbar: string;
  scrollbarThumb: string;
}

export interface NapiThemeSpacing {
  none: number;
  xxs: number;
  xs: number;
  sm: number;
  md: number;
  lg: number;
  xl: number;
  xxl: number;
}

export interface NapiThemeBorders {
  style: string;
  fg: string;
}

export interface NapiTheme {
  name: string;
  colors: NapiThemeColors;
  spacing: NapiThemeSpacing;
  borders: NapiThemeBorders;
}

export function createDarkTheme(): NapiTheme {
  return native.createDarkTheme();
}

export function createLightTheme(): NapiTheme {
  return native.createLightTheme();
}
