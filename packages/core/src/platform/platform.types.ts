export interface ProcessResult {
  success: number;
  errors: string[];
  idMappings: Array<{ temp: number; real: number }>;
}

export interface TerminalCapabilities {
  trueColor: boolean;
  mouse: boolean;
  bracketedPaste: boolean;
  sync: boolean;
  sgrPixel: boolean;
  kittyKeyboard: boolean;
  csi_u: boolean;
  focusEvents: boolean;
  osc8: boolean;
  underlineColor: boolean;
  strikethrough: boolean;
  cursorStyle: boolean;
  alternateScroll: boolean;
  hyperlinks: boolean;
  inlineImages: boolean;
  sixel: boolean;
  terminalSize: { columns: number; rows: number };
  pixelSize: { width: number; height: number } | null;
  brand: string;
}

export interface SchedulerStats {
  frameCount: string;
  droppedFrames: string;
  fps: string;
  frameBudgetMs: string;
  isIdle: boolean;
}

export interface NapiRenderResult {
  outputData: Uint8Array;
  width: number;
  height: number;
  dirtyRegionCount: number;
}

export interface NapiEngine {
  processCommands(commandsJson: string): string;
  render(): NapiRenderResult;
  renderFull(): NapiRenderResult;
  resize(width: number, height: number): void;
  setScreenMode(mode: string, footerHeight?: number | null): void;
  beginFrame(): void;
  commitFrame(): void;
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
  focus(nodeId: number): boolean;
  blur(nodeId: number): boolean;
  blurCurrent(): boolean;
  focused(): number;
  isFocused(nodeId: number): boolean;
  traverse(direction: string): number;
  focusOrder(): number[];
  clear(): void;
}

export interface NapiTextEngine {
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

export interface NapiScheduler {
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

export type ScreenMode = "alternate-screen" | "main-screen" | "split-footer";
export type ExternalOutputMode = "capture-stdout" | "passthrough";

export interface BindingInfo {
  id: string;
  keys: string;
  command: string;
  description: string | null;
  enabled: boolean;
  layer: string;
}

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

export interface NapiWidgetHost {
  widgetCount(): number;
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
  setMode(mode: string): void;
  currentMode(): string | null;
  clearMode(): void;
  removeLayer(name: string): boolean;
  setChordTimeout(ms: number): void;
  chordTimeout(): number;
  handleKey(keyStr: string): string | null;
  hasPending(): boolean;
  clearPending(): void;
  pendingKeys(): string[];
  activeBindings(): BindingInfo[];
  allBindings(): BindingInfo[];
  commandHistory(): string[];
  clearHistory(): void;
  parseKey(keyStr: string): string | null;
  parseSequence(keyStr: string): string[];
}
