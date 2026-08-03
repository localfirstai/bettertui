/**
 * Stub renderables for features that require complex implementation.
 * These provide type-correct APIs that compile, with simplified functionality.
 */

import { renderFontToText } from "../lib/asciiFont";
import { type ColorInput, RGBA, parseColor } from "../lib/rgba";
import type { StyledText, TextChunk } from "../lib/styledText";
import type { CliRenderer } from "../platform/cliRenderer";
import { type BorderStyleKind, type BoxOptions, BoxRenderable } from "./Box";
import { type TextOptions, TextRenderable } from "./Text";

// ── ASCIIFontRenderable ───────────────────────────────────────────────────────

export type ASCIIFont = "tiny" | "block" | "shade" | "slick" | string;

export interface ASCIIFontOptions extends BoxOptions {
  text?: string;
  font?: ASCIIFont;
  color?: ColorInput | ColorInput[];
  backgroundColor?: ColorInput;
  selectionBg?: ColorInput;
  selectionFg?: ColorInput;
}

let _asciiCounter = 0;

export class ASCIIFontRenderable extends BoxRenderable {
  private _text: string;
  private _font: ASCIIFont;
  private _color: RGBA[];
  private _contentNodeId: number;

  // ASCII art maps for tiny font (simple 3-char-wide)
  private static readonly TINY_CHARS: Record<string, string> = {
    " ": "   \n   \n   ",
    A: " A \n/ \\\nA_A",
    B: "B_ \nB_|\nB_/",
    C: " _C\n C \n_C/",
    "0": "_0_\n0 0\n0_0",
    "1": "_1 \n 1 \n_1_",
  };

  constructor(renderer: CliRenderer, options: ASCIIFontOptions = {}) {
    _asciiCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `asciifont-${_asciiCounter}`,
    });

    this._text = options.text ?? "";
    this._font = options.font ?? "block";
    const rawColor = options.color;
    const toRGBA = (c: ColorInput): RGBA =>
      c !== null && typeof c === "object" && "r" in c ? c : parseColor(c);
    this._color = Array.isArray(rawColor)
      ? rawColor.map(toRGBA)
      : rawColor
        ? [toRGBA(rawColor)]
        : [{ r: 255, g: 255, b: 255, a: 255 }];

    this._contentNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._contentNodeId);
    this._render();
  }

  get text(): string {
    return this._text;
  }

  set text(v: string) {
    this._text = v;
    this._render();
  }

  get font(): ASCIIFont {
    return this._font;
  }

  set font(v: ASCIIFont) {
    this._font = v;
    this._render();
  }

  get color(): RGBA[] {
    return this._color;
  }

  set color(v: RGBA | RGBA[]) {
    this._color = Array.isArray(v) ? v : [v];
    this._render();
  }

  private _render(): void {
    if (this._isDestroyed) return;
    const renderedText = renderFontToText(this._text, this._font, this._color);
    this._renderer.setText(this._contentNodeId, renderedText);
  }

  override destroy(): void {
    if (this._isDestroyed) return;
    try {
      this._renderer.removeNode(this._contentNodeId);
    } catch {
      /* ignore */
    }
    super.destroy();
  }

  /** Returns whether this renderable has an active selection. */
  hasSelection(): boolean {
    return false;
  }
}

// ── FrameBufferRenderable ─────────────────────────────────────────────────────

export interface FrameBufferOptions extends BoxOptions {
  drawFn?: (buffer: FrameBufferLike, deltaTime: number, renderable: FrameBufferRenderable) => void;
}

export interface FrameBufferLike {
  width: number;
  height: number;
  setCell(x: number, y: number, char: string, fg?: RGBA, bg?: RGBA): void;
  drawText(text: string, x: number, y: number, fg?: RGBA, bg?: RGBA): void;
  fillRect(x: number, y: number, w: number, h: number, color: RGBA): void;
  clear(color?: RGBA): void;
}

let _framebufferCounter = 0;

export class FrameBufferRenderable extends BoxRenderable {
  private _drawFn: FrameBufferOptions["drawFn"];
  private _buffer: SimpleFrameBuffer;
  private _contentNodeId: number;

  get frameBuffer(): FrameBufferLike {
    return this._buffer;
  }

  constructor(renderer: CliRenderer, options: FrameBufferOptions = {}) {
    _framebufferCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `framebuffer-${_framebufferCounter}`,
    });

    const w = typeof options.width === "number" ? options.width : 80;
    const h = typeof options.height === "number" ? options.height : 24;
    this._buffer = new SimpleFrameBuffer(w, h);
    this._drawFn = options.drawFn;
    this._contentNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._contentNodeId);
  }

  draw(deltaTime: number): void {
    if (this._isDestroyed || !this._drawFn) return;
    this._drawFn(this._buffer, deltaTime, this);
    this._flush();
  }

  private _flush(): void {
    this._renderer.setText(this._contentNodeId, this._buffer.toString());
  }

  override destroy(): void {
    if (this._isDestroyed) return;
    try {
      this._renderer.removeNode(this._contentNodeId);
    } catch {
      /* ignore */
    }
    super.destroy();
  }
}

class SimpleFrameBuffer implements FrameBufferLike {
  readonly width: number;
  readonly height: number;
  private cells: Array<{ char: string; fg?: RGBA; bg?: RGBA }>;

  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
    this.cells = Array.from({ length: width * height }, () => ({ char: " " }));
  }

  setCell(x: number, y: number, char: string, fg?: RGBA, bg?: RGBA): void {
    if (x < 0 || x >= this.width || y < 0 || y >= this.height) return;
    this.cells[y * this.width + x] = { char, fg, bg };
  }

  drawText(text: string, x: number, y: number, fg?: RGBA, _bg?: RGBA): void {
    for (let i = 0; i < text.length; i++) {
      this.setCell(x + i, y, text[i] ?? "", fg);
    }
  }

  fillRect(x: number, y: number, w: number, h: number, color: RGBA): void {
    for (let dy = 0; dy < h; dy++) {
      for (let dx = 0; dx < w; dx++) {
        this.setCell(x + dx, y + dy, " ", undefined, color);
      }
    }
  }

  clear(color?: RGBA): void {
    for (let i = 0; i < this.cells.length; i++) {
      this.cells[i] = { char: " ", bg: color };
    }
  }

  toString(): string {
    let result = "";
    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        const cell = this.cells[y * this.width + x];
        if (!cell) continue;
        if (cell.fg) {
          result += `\x1b[38;2;${cell.fg.r};${cell.fg.g};${cell.fg.b}m`;
        }
        if (cell.bg) {
          result += `\x1b[48;2;${cell.bg.r};${cell.bg.g};${cell.bg.b}m`;
        }
        result += cell.char;
        if (cell.fg || cell.bg) result += "\x1b[0m";
      }
      if (y < this.height - 1) result += "\n";
    }
    return result;
  }
}

// ── CodeRenderable ────────────────────────────────────────────────────────────

export interface CodeOptions extends TextOptions {
  language?: string;
  filetype?: string;
  showLineNumbers?: boolean;
  code?: string;
  selectionBg?: ColorInput;
  selectionFg?: ColorInput;
  syntaxStyle?: unknown;
}

let _codeCounter = 0;

export class CodeRenderable extends TextRenderable {
  private _language: string;
  private _showLineNumbers: boolean;
  private _code: string;
  filetype = "";
  selectionBg: ColorInput = undefined;
  selectionFg: ColorInput = undefined;
  syntaxStyle: unknown = null;
  virtualLineCount = 0;

  constructor(renderer: CliRenderer, options: CodeOptions = {}) {
    _codeCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `code-${_codeCounter}`,
      content: options.code ?? options.content ?? "",
    });
    this._language = options.language ?? options.filetype ?? "text";
    this._showLineNumbers = options.showLineNumbers !== false;
    this._code = options.code ?? "";
    this.filetype = options.filetype ?? this._language;
    if (this._code) this._renderCode();
  }

  get code(): string {
    return this._code;
  }
  set code(v: string) {
    this._code = v;
    this._renderCode();
  }
  get showLineNumbers(): boolean {
    return this._showLineNumbers;
  }
  set showLineNumbers(v: boolean) {
    this._showLineNumbers = v;
    this._renderCode();
  }
  set language(v: string) {
    this._language = v;
    this._renderCode();
  }

  conceal(_ranges: unknown): void {}

  private _renderCode(): void {
    const lines = this._code.split("\n");
    const lineNumWidth = String(lines.length).length;
    const rendered = lines.map((line, i) => {
      if (this._showLineNumbers) {
        const num = String(i + 1).padStart(lineNumWidth);
        return `\x1b[38;2;80;80;100m${num}\x1b[0m \x1b[38;2;200;200;200m${line}\x1b[0m`;
      }
      return `\x1b[38;2;200;200;200m${line}\x1b[0m`;
    });
    this.content = rendered.join("\n");
  }
}

// ── DiffRenderable ────────────────────────────────────────────────────────────

export interface DiffOptions extends TextOptions {
  oldText?: string;
  newText?: string;
  mode?: "unified" | "split";
}

let _diffCounter = 0;

export class DiffRenderable extends TextRenderable {
  constructor(renderer: CliRenderer, options: DiffOptions = {}) {
    _diffCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `diff-${_diffCounter}`,
    });
    if (options.oldText !== undefined || options.newText !== undefined) {
      this._setDiff(options.oldText ?? "", options.newText ?? "");
    }
  }

  setDiff(oldText: string, newText: string): void {
    this._setDiff(oldText, newText);
  }

  private _setDiff(oldText: string, newText: string): void {
    const oldLines = oldText.split("\n");
    const newLines = newText.split("\n");
    const lines: string[] = [];

    // Simple line-by-line diff display
    const maxLen = Math.max(oldLines.length, newLines.length);
    for (let i = 0; i < maxLen; i++) {
      const old = oldLines[i];
      const next = newLines[i];
      if (old === undefined) {
        lines.push(`\x1b[38;2;0;200;0m+ ${next ?? ""}\x1b[0m`);
      } else if (next === undefined) {
        lines.push(`\x1b[38;2;200;0;0m- ${old}\x1b[0m`);
      } else if (old === next) {
        lines.push(`  ${old}`);
      } else {
        lines.push(`\x1b[38;2;200;0;0m- ${old}\x1b[0m`);
        lines.push(`\x1b[38;2;0;200;0m+ ${next}\x1b[0m`);
      }
    }
    this.content = lines.join("\n");
  }
}

// ── MarkdownRenderable ────────────────────────────────────────────────────────

export interface MarkdownOptions extends TextOptions {
  content?: string | StyledText;
}

let _markdownCounter = 0;

export class MarkdownRenderable extends TextRenderable {
  constructor(renderer: CliRenderer, options: MarkdownOptions = {}) {
    _markdownCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `markdown-${_markdownCounter}`,
    });
  }

  set markdown(text: string) {
    // Basic markdown rendering
    const lines = text.split("\n").map((line) => {
      if (line.startsWith("# ")) {
        return `\x1b[1;38;2;255;200;0m${line.slice(2)}\x1b[0m`;
      }
      if (line.startsWith("## ")) {
        return `\x1b[1;38;2;200;160;0m${line.slice(3)}\x1b[0m`;
      }
      if (line.startsWith("### ")) {
        return `\x1b[1;38;2;160;120;0m${line.slice(4)}\x1b[0m`;
      }
      if (line.startsWith("- ") || line.startsWith("* ")) {
        return `  \x1b[38;2;100;200;100m•\x1b[0m ${line.slice(2)}`;
      }
      if (line.startsWith("> ")) {
        return `\x1b[38;2;100;100;180m│\x1b[0m ${line.slice(2)}`;
      }
      // Bold **text**
      return line
        .replace(/\*\*(.+?)\*\*/g, "\x1b[1m$1\x1b[0m")
        .replace(/\*(.+?)\*/g, "\x1b[3m$1\x1b[0m")
        .replace(/`(.+?)`/g, "\x1b[38;2;150;220;150m$1\x1b[0m");
    });
    this.content = lines.join("\n");
  }
}

// ── TextTableRenderable ───────────────────────────────────────────────────────

export interface TableColumn {
  header: string;
  key?: string;
  width?: number;
  align?: "left" | "center" | "right";
}

export type TextTableColumnWidthMode = "content" | "full";
export type TextTableColumnFitter = "proportional" | "balanced";
export type TextTableContent = Array<Array<TextChunk[] | Array<TextChunk>>>;

export interface TextTableOptions extends BoxOptions {
  columns?: TableColumn[];
  rows?: Record<string, unknown>[][];
  data?: string[][];
  showBorder?: boolean;
  headerColor?: ColorInput;
  rowColor?: ColorInput;
  alternateRowColor?: ColorInput;
  wrapMode?: "none" | "word" | "char";
  columnWidthMode?: TextTableColumnWidthMode;
  columnFitter?: TextTableColumnFitter;
  cellPadding?: number;
  border?: boolean;
  outerBorder?: boolean;
  showBorders?: boolean;
  borderStyle?: BorderStyleKind;
  borderColor?: ColorInput;
  fg?: ColorInput;
  bg?: ColorInput;
  content?: TextTableContent;
}

let _tableCounter = 0;

export class TextTableRenderable extends BoxRenderable {
  private _columns: TableColumn[];
  private _data: string[][];
  private _contentNodeId: number;
  private _headerColor: RGBA;
  private _rowColor: RGBA;
  private _wrapMode: "none" | "word" | "char";
  private _columnWidthMode: TextTableColumnWidthMode;
  private _columnFitter: TextTableColumnFitter;
  private _cellPadding: number;
  private _outerBorder: boolean;
  private _showBorders: boolean;
  private _content: TextTableContent | null = null;

  constructor(renderer: CliRenderer, options: TextTableOptions = {}) {
    _tableCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `table-${_tableCounter}`,
    });

    this._columns = options.columns ?? [];
    this._data = options.data ?? [];
    this._headerColor = parseColor(options.headerColor ?? "#0088ff");
    this._rowColor = parseColor(options.rowColor ?? "#dddddd");
    this._wrapMode = options.wrapMode ?? "none";
    this._columnWidthMode = options.columnWidthMode ?? "content";
    this._columnFitter = options.columnFitter ?? "proportional";
    this._cellPadding = options.cellPadding ?? 0;
    this._outerBorder = options.outerBorder !== false;
    this._showBorders = options.showBorders !== false;
    this._content = options.content ?? null;
    this._contentNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._contentNodeId);
    this._render();
  }

  get wrapMode(): "none" | "word" | "char" {
    return this._wrapMode;
  }
  set wrapMode(v: "none" | "word" | "char") {
    this._wrapMode = v;
    this._render();
  }

  get columnWidthMode(): TextTableColumnWidthMode {
    return this._columnWidthMode;
  }
  set columnWidthMode(v: TextTableColumnWidthMode) {
    this._columnWidthMode = v;
    this._render();
  }

  get columnFitter(): TextTableColumnFitter {
    return this._columnFitter;
  }
  set columnFitter(v: TextTableColumnFitter) {
    this._columnFitter = v;
    this._render();
  }

  get cellPadding(): number {
    return this._cellPadding;
  }
  set cellPadding(v: number) {
    this._cellPadding = v;
    this._render();
  }

  get outerBorder(): boolean {
    return this._outerBorder;
  }
  set outerBorder(v: boolean) {
    this._outerBorder = v;
    this._render();
  }

  get showBorders(): boolean {
    return this._showBorders;
  }
  set showBorders(v: boolean) {
    this._showBorders = v;
    this._render();
  }

  get content(): TextTableContent | null {
    return this._content;
  }
  set content(v: TextTableContent | null) {
    this._content = v;
    this._render();
  }

  setData(columns: TableColumn[], data: string[][]): void {
    this._columns = columns;
    this._data = data;
    this._render();
  }

  addRow(row: string[]): void {
    this._data.push(row);
    this._render();
  }

  private _render(): void {
    if (this._isDestroyed) return;
    const lines: string[] = [];

    // Header row
    if (this._columns.length > 0) {
      const hc = `${this._headerColor.r};${this._headerColor.g};${this._headerColor.b}`;
      const headers = this._columns.map((col) => {
        const w = col.width ?? 12;
        return `\x1b[1;38;2;${hc}m${col.header.slice(0, w).padEnd(w)}\x1b[0m`;
      });
      lines.push(headers.join(" │ "));
      lines.push(this._columns.map((col) => "─".repeat(col.width ?? 12)).join("─┼─"));
    }

    const rc = `${this._rowColor.r};${this._rowColor.g};${this._rowColor.b}`;
    for (const row of this._data) {
      const cells =
        this._columns.length > 0
          ? this._columns.map((col, i) => {
              const w = col.width ?? 12;
              const cell = (row[i] ?? "").slice(0, w).padEnd(w);
              return `\x1b[38;2;${rc}m${cell}\x1b[0m`;
            })
          : row.map((cell) => `\x1b[38;2;${rc}m${cell}\x1b[0m`);
      lines.push(cells.join(" │ "));
    }

    this._renderer.setText(this._contentNodeId, lines.join("\n"));
  }

  override destroy(): void {
    if (this._isDestroyed) return;
    try {
      this._renderer.removeNode(this._contentNodeId);
    } catch {
      /* ignore */
    }
    super.destroy();
  }
}

// ── LineNumberRenderable ──────────────────────────────────────────────────────

export interface LineNumberOptions extends BoxOptions {
  lineCount?: number;
  startLine?: number;
  color?: ColorInput;
  highlightColor?: ColorInput;
  highlightLine?: number;
  target?: unknown;
}

let _lineNumCounter = 0;

export class LineNumberRenderable extends BoxRenderable {
  private _lineCount: number;
  private _startLine: number;
  private _color: RGBA;
  private _highlightColor: RGBA;
  private _highlightLine: number;
  private _contentNodeId: number;
  fg: ColorInput = undefined;
  bg: ColorInput = undefined;

  constructor(renderer: CliRenderer, options: LineNumberOptions = {}) {
    _lineNumCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `linenum-${_lineNumCounter}`,
    });
    this._lineCount = options.lineCount ?? 0;
    this._startLine = options.startLine ?? 1;
    this._color = parseColor(options.color ?? "#555577");
    this._highlightColor = parseColor(options.highlightColor ?? "#8888aa");
    this._highlightLine = options.highlightLine ?? -1;
    this._contentNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._contentNodeId);
    this._render();
  }

  get lineCount(): number {
    return this._lineCount;
  }
  set lineCount(v: number) {
    this._lineCount = v;
    this._render();
  }
  get showLineNumbers(): boolean {
    return true;
  }
  set highlightLine(v: number) {
    this._highlightLine = v;
    this._render();
  }

  setLineColor(_line: number, _color: ColorInput): void {
    this._render();
  }
  clearAllLineColors(): void {
    this._render();
  }
  setLineSign(_line: number, _sign: string, _color?: ColorInput): void {
    this._render();
  }
  clearLineSign(_line: number): void {
    this._render();
  }
  getLineSigns(_line: number): string[] {
    return [];
  }

  private _render(): void {
    if (this._isDestroyed) return;
    const lines: string[] = [];
    const width = String(this._startLine + this._lineCount).length;
    const nc = `${this._color.r};${this._color.g};${this._color.b}`;
    const hc = `${this._highlightColor.r};${this._highlightColor.g};${this._highlightColor.b}`;
    for (let i = 0; i < this._lineCount; i++) {
      const num = this._startLine + i;
      const isHighlight = num === this._highlightLine;
      const numStr = String(num).padStart(width);
      lines.push(
        isHighlight ? `\x1b[38;2;${hc}m${numStr}\x1b[0m` : `\x1b[38;2;${nc}m${numStr}\x1b[0m`,
      );
    }
    this._renderer.setText(this._contentNodeId, lines.join("\n"));
  }
  override destroy(): void {
    if (this._isDestroyed) return;
    try {
      this._renderer.removeNode(this._contentNodeId);
    } catch {
      /* ignore */
    }
    super.destroy();
  }
}
// ── TimeToFirstDrawRenderable ─────────────────────────────────────────────────

export interface TimeToFirstDrawOptions extends TextOptions {
  fg?: ColorInput;
  color?: RGBA;
}

let _ttfdCounter = 0;

export class TimeToFirstDrawRenderable extends BoxRenderable {
  private _fg: RGBA;
  private _color: RGBA;
  private _contentNodeId: number;
  private _startTime: number;

  constructor(renderer: CliRenderer, options: TimeToFirstDrawOptions = {}) {
    _ttfdCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `ttfd-${_ttfdCounter}`,
    });
    this._fg = parseColor(options.fg ?? "#888888");
    this._color = options.color ?? this._fg;
    this._startTime = Date.now();
    this._contentNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._contentNodeId);
    // Apply the foreground color to the text node using setNodeStyle
    renderer.setNodeStyle(this._contentNodeId, { fg: RGBA.toHex(this._color) });
    this._render();
  }

  get fg(): RGBA {
    return this._fg;
  }

  set fg(color: ColorInput) {
    this._fg = parseColor(color);
    this._color = this._fg;
    this._renderer.setNodeStyle(this._contentNodeId, {
      fg: RGBA.toHex(this._color),
    });
    this._render();
  }

  get color(): RGBA {
    return this._color;
  }

  set color(v: RGBA) {
    this._color = v;
    this._renderer.setNodeStyle(this._contentNodeId, {
      fg: RGBA.toHex(this._color),
    });
    this._render();
  }

  private _render(): void {
    if (this._isDestroyed) return;
    const elapsed = Date.now() - this._startTime;
    // Use plain text - colors should be applied via style system, not ANSI codes
    this._renderer.setText(this._contentNodeId, `Time to first draw: ${elapsed}ms`);
  }

  override destroy(): void {
    if (this._isDestroyed) return;
    try {
      this._renderer.removeNode(this._contentNodeId);
    } catch {
      /* ignore */
    }
    super.destroy();
  }
}
