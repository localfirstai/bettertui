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

export interface NapiEngine {
  processCommands(commandsJson: string): string;
  render(): string;
  resize(width: number, height: number): void;
  beginFrame(): void;
  commitFrame(): void;
  nodeCount(): number;
  treeSummary(): string;
  printTree(): string;
  validate(): string;
  root(): string;
  generation(): string;
}

export interface NapiEventBus {
  pushKey(key: string, ctrl: boolean, shift: boolean, alt: boolean, targetId: number): void;
  pushMouse(button: string, x: number, y: number, targetId: number): void;
  pushMouseMotion(x: number, y: number, targetId: number): void;
  pushPaste(text: string, targetId: number): void;
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
  focusedInScope(): number | null;
  traverse(direction: string): number;
  setScope(scopeId: number): void;
  clearScope(): void;
  scopeId(): number | null;
  focusOrder(): number[];
  isFocused(nodeId: number): boolean;
}

export interface NapiTextEngine {
  insertText(text: string): void;
  deleteCharBackward(): void;
  deleteCharForward(): void;
  deleteWordBackward(): void;
  deleteWordForward(): void;
  deleteLineBackward(): void;
  deleteLineForward(): void;
  cursorLeft(): void;
  cursorRight(): void;
  cursorUp(): void;
  cursorDown(): void;
  cursorLineStart(): void;
  cursorLineEnd(): void;
  cursorPosition(): number;
  setCursorPosition(pos: number): void;
  text(): string;
  insertAt(position: number, text: string): void;
  deleteAt(position: number, length: number): string;
  charAt(position: number): string;
  substring(start: number, end: number): string;
  find(pattern: string, caseSensitive: boolean): Array<{ start: number; end: number }>;
  replaceAll(pattern: string, replacement: string, caseSensitive: boolean): number;
  undo(): boolean;
  redo(): boolean;
  canUndo(): boolean;
  canRedo(): boolean;
  clear(): void;
  length(): number;
  isEmpty(): boolean;
  lines(): string[];
  lineCount(): number;
}

export interface NapiScheduler {
  beginFrame(): boolean;
  endFrame(): void;
  requestFrame(): void;
  frameCount(): string;
  droppedFrames(): string;
  fps(): string;
  frameBudgetMs(): string;
  isIdle(): boolean;
}
