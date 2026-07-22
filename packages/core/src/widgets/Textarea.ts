import type { KeyEvent, TextareaOptions } from "@bettertui/shared";
import type { Command } from "../command/types";
import type { NapiTextEngine } from "../platform/binding";
import { createTextEngine } from "../platform/binding";
import { Renderable } from "../renderable";

export type { TextareaOptions };

export class Textarea extends Renderable<TextareaOptions> {
  private _engine: NapiTextEngine;

  constructor(options: TextareaOptions = {}) {
    super(options);
    this._engine = createTextEngine(options.value ?? "");
    // Position cursor at end of initial value
    if (options.value && options.value.length > 0) {
      this._engine.setCursorPosition(this._engine.length());
    }
  }

  get value(): string {
    return this._engine.getText();
  }

  get cursorPosition(): number {
    return this._engine.cursorPosition();
  }

  override update(options: Partial<TextareaOptions>): void {
    if (options.value !== undefined && options.value !== this._engine.getText()) {
      this._engine.clear();
      if (options.value.length > 0) {
        this._engine.insertStr(options.value);
        this._engine.setCursorPosition(this._engine.length());
      }
    }
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];
    const rows = this.opts.rows ?? 3;
    cmds.push({ type: "SetHeight", id, value: rows });

    const text = this._engine.getText() || this.opts.placeholder || "";
    const textId = `${id}-text`;
    cmds.push({ type: "CreateNode", id: textId, kind: "Text" });
    cmds.push({ type: "SetText", id: textId, text });
    cmds.push({ type: "AppendChild", parent: id, child: textId });

    // Cursor position for the engine
    cmds.push({
      type: "SetAttribute",
      id,
      key: "cursorPos",
      value: String(this._engine.cursorPosition()),
    });

    return cmds;
  }

  override handleKey(key: KeyEvent): boolean {
    if (this.opts.disabled) return false;

    // Undo / redo
    if ((key.ctrl || key.meta) && key.key === "z") {
      if (key.shift) {
        this._engine.redo();
      } else {
        this._engine.undo();
      }
      this.opts.onChange?.(this._engine.getText());
      return true;
    }

    if ((key.ctrl || key.meta) && key.key === "y") {
      this._engine.redo();
      this.opts.onChange?.(this._engine.getText());
      return true;
    }

    // Navigation
    if (key.key === "left") {
      if (key.ctrl || key.meta) {
        this._moveCursorWordLeft();
      } else {
        this._engine.cursorLeft();
      }
      return true;
    }

    if (key.key === "right") {
      if (key.ctrl || key.meta) {
        this._moveCursorWordRight();
      } else {
        this._engine.cursorRight();
      }
      return true;
    }

    if (key.key === "home" || (key.ctrl && key.key === "a")) {
      // Move to start of current line
      const text = this._engine.getText();
      let pos = this._engine.cursorPosition();
      while (pos > 0 && text[pos - 1] !== "\n") pos--;
      this._engine.setCursorPosition(pos);
      return true;
    }

    if (key.key === "end" || (key.ctrl && key.key === "e")) {
      // Move to end of current line
      const text = this._engine.getText();
      let pos = this._engine.cursorPosition();
      const len = text.length;
      while (pos < len && text[pos] !== "\n") pos++;
      this._engine.setCursorPosition(pos);
      return true;
    }

    if (key.key === "up") {
      this._moveCursorVertically(-1);
      return true;
    }

    if (key.key === "down") {
      this._moveCursorVertically(1);
      return true;
    }

    // Deletion
    if (key.key === "backspace") {
      this._engine.deleteChar();
      this.opts.onChange?.(this._engine.getText());
      return true;
    }

    if (key.key === "delete") {
      const pos = this._engine.cursorPosition();
      if (pos < this._engine.length()) {
        this._engine.cursorRight();
        this._engine.deleteChar();
        this._engine.setCursorPosition(pos);
        this.opts.onChange?.(this._engine.getText());
      }
      return true;
    }

    // Newline
    if (key.key === "return") {
      this._engine.insertChar("\n");
      this.opts.onChange?.(this._engine.getText());
      return true;
    }

    // Kill line (line-scoped, unlike Input which kills whole buffer)
    if (key.ctrl && key.key === "k") {
      const text = this._engine.getText();
      const pos = this._engine.cursorPosition();
      let lineEnd = pos;
      while (lineEnd < text.length && text[lineEnd] !== "\n") lineEnd++;
      const before = text.slice(0, pos);
      const after = text.slice(lineEnd);
      this._engine.clear();
      if (before.length > 0) this._engine.insertStr(before);
      if (after.length > 0) this._engine.insertStr(after);
      this._engine.setCursorPosition(pos);
      this.opts.onChange?.(this._engine.getText());
      return true;
    }

    // Kill to line start
    if (key.ctrl && key.key === "u") {
      const text = this._engine.getText();
      const pos = this._engine.cursorPosition();
      let lineStart = pos;
      while (lineStart > 0 && text[lineStart - 1] !== "\n") lineStart--;
      const before = text.slice(0, lineStart);
      const after = text.slice(pos);
      this._engine.clear();
      if (before.length > 0) this._engine.insertStr(before);
      if (after.length > 0) this._engine.insertStr(after);
      this._engine.setCursorPosition(lineStart);
      this.opts.onChange?.(this._engine.getText());
      return true;
    }

    // Skip modifier combos
    if (key.ctrl || key.meta || key.alt) return false;

    // Printable characters
    if (key.key && key.key.length === 1) {
      this._engine.insertChar(key.key);
      this.opts.onChange?.(this._engine.getText());
      return true;
    }

    return false;
  }

  private _moveCursorWordLeft(): void {
    const text = this._engine.getText();
    let pos = this._engine.cursorPosition();
    while (pos > 0 && text[pos - 1] === " ") pos--;
    while (pos > 0 && text[pos - 1] !== " " && text[pos - 1] !== "\n") pos--;
    this._engine.setCursorPosition(pos);
  }

  private _moveCursorWordRight(): void {
    const text = this._engine.getText();
    let pos = this._engine.cursorPosition();
    const len = text.length;
    while (pos < len && text[pos] !== " " && text[pos] !== "\n") pos++;
    while (pos < len && text[pos] === " ") pos++;
    this._engine.setCursorPosition(pos);
  }

  private _moveCursorVertically(direction: -1 | 1): void {
    const text = this._engine.getText();
    const pos = this._engine.cursorPosition();

    // Find current line start
    let lineStart = pos;
    while (lineStart > 0 && text[lineStart - 1] !== "\n") lineStart--;

    // Current column offset
    const col = pos - lineStart;

    if (direction === -1) {
      // Move up: find previous line
      if (lineStart === 0) return;
      const prevLineEnd = lineStart - 1; // newline char
      let prevLineStart = prevLineEnd;
      while (prevLineStart > 0 && text[prevLineStart - 1] !== "\n") prevLineStart--;
      const prevLineLen = prevLineEnd - prevLineStart;
      this._engine.setCursorPosition(prevLineStart + Math.min(col, prevLineLen));
    } else {
      // Move down: find next line
      let lineEnd = pos;
      while (lineEnd < text.length && text[lineEnd] !== "\n") lineEnd++;
      if (lineEnd >= text.length) return;
      const nextLineStart = lineEnd + 1;
      let nextLineEnd = nextLineStart;
      while (nextLineEnd < text.length && text[nextLineEnd] !== "\n") nextLineEnd++;
      const nextLineLen = nextLineEnd - nextLineStart;
      this._engine.setCursorPosition(nextLineStart + Math.min(col, nextLineLen));
    }
  }
}
