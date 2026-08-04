/**
 * Textarea — a multi-line text editor widget.
 */

import { InputEvents, RenderableEvents } from "../lib/renderableEvents";
import { type ColorInput, type RGBA, parseColor, rgbaToEngineColor } from "../lib/rgba";
import type { CliRenderer } from "../platform/cliRenderer";
import type { RawKeyEvent } from "../platform/cliRenderer";
import { Box, type BoxOptions } from "./Box";

/** Minimal extmarks controller stub for type compatibility. */
export interface ExtmarksController {
  create(opts: {
    start: number;
    end: number;
    virtual?: boolean;
    styleId?: number;
    data?: unknown;
  }): number;
  getAtOffset(
    offset: number,
  ): Array<{ start: number; end: number; styleId?: number; data?: unknown }>;
  getVirtual(): Array<{ start: number; end: number; styleId?: number; data?: unknown }>;
  destroy(): void;
}

export class ExtmarksControllerStub implements ExtmarksController {
  create(_opts: {
    start: number;
    end: number;
    virtual?: boolean;
    styleId?: number;
    data?: unknown;
  }): number {
    return 0;
  }
  getAtOffset(
    _offset: number,
  ): Array<{ start: number; end: number; styleId?: number; data?: unknown }> {
    return [];
  }
  getVirtual(): Array<{ start: number; end: number; styleId?: number; data?: unknown }> {
    return [];
  }
  destroy(): void {}
}

export interface TextareaOptions extends BoxOptions {
  initialValue?: string;
  placeholder?: string;
  placeholderColor?: ColorInput;
  textColor?: ColorInput;
  focusedTextColor?: ColorInput;
  cursorColor?: ColorInput;
  backgroundColor?: ColorInput;
  focusedBackgroundColor?: ColorInput;
  wrapMode?: "none" | "char" | "word";
  showCursor?: boolean;
  readonly?: boolean;
  selectionBg?: ColorInput;
  selectionFg?: ColorInput;
  syntaxStyle?: unknown;
}

let _textareaCounter = 0;

export class Textarea extends Box {
  protected _text: string;
  protected _cursorLine = 0;
  protected _cursorCol = 0;
  private _placeholder: string;
  private _placeholderColor: RGBA;
  private _textColor: RGBA;
  private _focusedTextColor: RGBA;
  private _cursorColor: RGBA;
  private _focusedBgColor: RGBA | null = null;
  private _wrapMode: "none" | "char" | "word";
  private _showCursor: boolean;
  private _readonly: boolean;
  private _textNodeId: number;
  private _scrollOffset = 0;
  private readonly _keyHandler: (key: RawKeyEvent) => void;

  /** Extmarks controller stub — override or replace in subclasses for full functionality. */
  public extmarks: ExtmarksControllerStub = new ExtmarksControllerStub();

  /** Logical cursor position (line/col). */
  get logicalCursor(): { row: number; col: number } {
    return { row: this._cursorLine, col: this._cursorCol };
  }

  constructor(renderer: CliRenderer, options: TextareaOptions = {}) {
    _textareaCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `textarea-${_textareaCounter}`,
      focusable: true,
    });

    this._text = options.initialValue ?? "";
    this._placeholder = options.placeholder ?? "";
    this._placeholderColor = parseColor(options.placeholderColor ?? "#666666");
    this._textColor = parseColor(options.textColor ?? "#ffffff");
    this._focusedTextColor = parseColor(options.focusedTextColor ?? "#ffffff");
    this._cursorColor = parseColor(options.cursorColor ?? "#ffff00");
    this._wrapMode = options.wrapMode ?? "char";
    this._showCursor = options.showCursor !== false;
    this._readonly = options.readonly ?? false;

    if (options.focusedBackgroundColor) {
      this._focusedBgColor = parseColor(options.focusedBackgroundColor);
    }

    this._textNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._textNodeId);

    this._keyHandler = this._handleKey.bind(this);
    this._render();
  }

  get plainText(): string {
    return this._text;
  }

  set plainText(v: string) {
    this._text = v;
    this._render();
  }

  get cursorOffset(): number {
    // Linear offset from start
    const lines = this._text.split("\n");
    let offset = 0;
    for (let i = 0; i < this._cursorLine; i++) {
      offset += (lines[i]?.length ?? 0) + 1; // +1 for newline
    }
    return offset + this._cursorCol;
  }

  setText(text: string): void {
    this._text = text;
    this._cursorLine = 0;
    this._cursorCol = 0;
    this._render();
  }

  insertText(text: string): void {
    if (this._readonly) return;
    const lines = this._text.split("\n");
    const line = lines[this._cursorLine] ?? "";
    lines[this._cursorLine] = line.slice(0, this._cursorCol) + text + line.slice(this._cursorCol);
    this._text = lines.join("\n");
    this._cursorCol += text.length;
    this._render();
    this.emit(InputEvents.INPUT, this._text);
  }

  newLine(): boolean {
    if (this._readonly) return false;
    const lines = this._text.split("\n");
    const line = lines[this._cursorLine] ?? "";
    const before = line.slice(0, this._cursorCol);
    const after = line.slice(this._cursorCol);
    lines.splice(this._cursorLine, 1, before, after);
    this._text = lines.join("\n");
    this._cursorLine++;
    this._cursorCol = 0;
    this._render();
    this.emit(InputEvents.INPUT, this._text);
    return true;
  }

  submit(): boolean {
    const current = this._text;
    this.emit(InputEvents.CHANGE, current);
    this.emit(InputEvents.ENTER, current);
    return true;
  }

  // ── Focus ─────────────────────────────────────────────────────────────────────

  override focus(): void {
    if (this._isDestroyed) return;
    this._focused = true;
    if (this._focusedBgColor) {
      this._renderer.setNodeStyle(this._nodeId, {
        bg: rgbaToEngineColor(this._focusedBgColor),
      });
    }
    this._render();
    this.emit(RenderableEvents.FOCUSED, this);
    this._renderer.keyInput.on("keypress", this._keyHandler);
  }

  override blur(): void {
    if (this._isDestroyed) return;
    this._renderer.keyInput.off("keypress", this._keyHandler);
    const current = this._text;
    this._focused = false;
    if (this._focusedBgColor && this._backgroundColor) {
      this._renderer.setNodeStyle(this._nodeId, {
        bg: rgbaToEngineColor(this._backgroundColor),
      });
    }
    this._render();
    this.emit(InputEvents.CHANGE, current);
    this.emit(RenderableEvents.BLURRED, this);
  }

  // ── Key handling ──────────────────────────────────────────────────────────────

  protected _handleKey(key: RawKeyEvent): void {
    if (!this._focused || this._isDestroyed) return;

    const lines = this._text.split("\n");

    if (key.name === "up") {
      this._cursorLine = Math.max(0, this._cursorLine - 1);
      this._cursorCol = Math.min(this._cursorCol, lines[this._cursorLine]?.length ?? 0);
      this._render();
      return;
    }
    if (key.name === "down") {
      this._cursorLine = Math.min(lines.length - 1, this._cursorLine + 1);
      this._cursorCol = Math.min(this._cursorCol, lines[this._cursorLine]?.length ?? 0);
      this._render();
      return;
    }
    if (key.name === "left") {
      if (this._cursorCol > 0) {
        this._cursorCol--;
      } else if (this._cursorLine > 0) {
        this._cursorLine--;
        this._cursorCol = lines[this._cursorLine]?.length ?? 0;
      }
      this._render();
      return;
    }
    if (key.name === "right") {
      const lineLen = lines[this._cursorLine]?.length ?? 0;
      if (this._cursorCol < lineLen) {
        this._cursorCol++;
      } else if (this._cursorLine < lines.length - 1) {
        this._cursorLine++;
        this._cursorCol = 0;
      }
      this._render();
      return;
    }

    if (key.name === "return" || key.name === "linefeed") {
      if (!this._readonly) this.newLine();
      return;
    }

    if (key.name === "backspace") {
      if (this._readonly) return;
      if (this._cursorCol > 0) {
        const line = lines[this._cursorLine] ?? "";
        lines[this._cursorLine] = line.slice(0, this._cursorCol - 1) + line.slice(this._cursorCol);
        this._text = lines.join("\n");
        this._cursorCol--;
        this._render();
        this.emit(InputEvents.INPUT, this._text);
      } else if (this._cursorLine > 0) {
        const prevLine = lines[this._cursorLine - 1] ?? "";
        const curLine = lines[this._cursorLine] ?? "";
        const newCol = prevLine.length;
        lines.splice(this._cursorLine - 1, 2, prevLine + curLine);
        this._text = lines.join("\n");
        this._cursorLine--;
        this._cursorCol = newCol;
        this._render();
        this.emit(InputEvents.INPUT, this._text);
      }
      return;
    }

    // Regular character
    if (key.sequence && !key.ctrl && !key.alt && !key.meta && !this._readonly) {
      const char = key.sequence;
      if (char.length === 1 && char.charCodeAt(0) >= 32) {
        this.insertText(char);
      }
    }
  }

  protected _render(): void {
    if (this._isDestroyed) return;

    const lines = this._text.split("\n");
    const textColor = this._focused ? this._focusedTextColor : this._textColor;
    const tc = `${textColor.r};${textColor.g};${textColor.b}`;

    const rendered: string[] = [];
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i] ?? "";
      if (this._focused && this._showCursor && i === this._cursorLine) {
        const before = line.slice(0, this._cursorCol);
        const cursorChar = line[this._cursorCol] ?? " ";
        const after = line.slice(this._cursorCol + 1);
        const cc = `${this._cursorColor.r};${this._cursorColor.g};${this._cursorColor.b}`;
        rendered.push(
          `\x1b[38;2;${tc}m${before}\x1b[38;2;${cc}m\x1b[7m${cursorChar}\x1b[0m\x1b[38;2;${tc}m${after}\x1b[0m`,
        );
      } else {
        rendered.push(`\x1b[38;2;${tc}m${line}\x1b[0m`);
      }
    }

    this._renderer.setText(this._textNodeId, rendered.join("\n"));
  }

  override destroy(): void {
    if (this._isDestroyed) return;
    this._renderer.keyInput.off("keypress", this._keyHandler);
    try {
      this._renderer.removeNode(this._textNodeId);
    } catch {
      // ignore
    }
    super.destroy();
  }
}

export { InputEvents };
