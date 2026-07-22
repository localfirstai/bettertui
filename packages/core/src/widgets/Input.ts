import type { InputOptions, KeyEvent } from "@bettertui/shared";
import type { Command } from "../command/types";
import type { NapiTextEngine } from "../platform/binding";
import { createTextEngine } from "../platform/binding";
import { Renderable } from "../renderable";

export type { InputOptions };

export class Input extends Renderable<InputOptions> {
  private _engine: NapiTextEngine;

  constructor(options: InputOptions = {}) {
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

  override update(options: Partial<InputOptions>): void {
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
    const value = this._engine.getText();
    const displayText = this.opts.password
      ? "*".repeat(value.length)
      : value || this.opts.placeholder || "";

    const textId = `${id}-text`;
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Input" }];
    cmds.push({ type: "CreateNode", id: textId, kind: "Text" });
    cmds.push({ type: "SetText", id: textId, text: displayText });
    cmds.push({ type: "AppendChild", parent: id, child: textId });

    if (this.opts.disabled) {
      cmds.push({ type: "SetDim", id, value: true });
    }

    // Emit cursor position as attribute for the engine to render cursor
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

    // Navigation
    if (key.key === "left") {
      if (key.ctrl || key.meta) {
        // Jump word left
        this._moveCursorWordLeft();
      } else {
        this._engine.cursorLeft();
      }
      return true;
    }

    if (key.key === "right") {
      if (key.ctrl || key.meta) {
        // Jump word right
        this._moveCursorWordRight();
      } else {
        this._engine.cursorRight();
      }
      return true;
    }

    if (key.key === "home" || (key.ctrl && key.key === "a")) {
      this._engine.setCursorPosition(0);
      return true;
    }

    if (key.key === "end" || (key.ctrl && key.key === "e")) {
      this._engine.setCursorPosition(this._engine.length());
      return true;
    }

    // Deletion
    if (key.key === "backspace") {
      if (key.ctrl) {
        // Delete word before cursor
        this._deleteWordBefore();
      } else {
        this._engine.deleteChar();
      }
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

    // Kill line
    if (key.ctrl && key.key === "k") {
      const pos = this._engine.cursorPosition();
      const text = this._engine.getText();
      const before = text.slice(0, pos);
      this._engine.clear();
      if (before.length > 0) {
        this._engine.insertStr(before);
        this._engine.setCursorPosition(pos);
      }
      this.opts.onChange?.(this._engine.getText());
      return true;
    }

    // Kill line from start
    if (key.ctrl && key.key === "u") {
      const pos = this._engine.cursorPosition();
      const text = this._engine.getText();
      const after = text.slice(pos);
      this._engine.clear();
      if (after.length > 0) {
        this._engine.insertStr(after);
        this._engine.setCursorPosition(0);
      }
      this.opts.onChange?.(this._engine.getText());
      return true;
    }

    // Submit
    if (key.key === "return") {
      this.opts.onSubmit?.(this._engine.getText());
      return true;
    }

    // Skip modifier-only keys
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
    // Skip whitespace
    while (pos > 0 && text[pos - 1] === " ") pos--;
    // Skip word
    while (pos > 0 && text[pos - 1] !== " ") pos--;
    this._engine.setCursorPosition(pos);
  }

  private _moveCursorWordRight(): void {
    const text = this._engine.getText();
    let pos = this._engine.cursorPosition();
    const len = text.length;
    // Skip word
    while (pos < len && text[pos] !== " ") pos++;
    // Skip whitespace
    while (pos < len && text[pos] === " ") pos++;
    this._engine.setCursorPosition(pos);
  }

  private _deleteWordBefore(): void {
    const text = this._engine.getText();
    let pos = this._engine.cursorPosition();
    const start = pos;
    // Skip whitespace
    while (pos > 0 && text[pos - 1] === " ") pos--;
    // Skip word
    while (pos > 0 && text[pos - 1] !== " ") pos--;
    const after = text.slice(start);
    const before = text.slice(0, pos);
    this._engine.clear();
    if (before.length > 0) this._engine.insertStr(before);
    if (after.length > 0) this._engine.insertStr(after);
    this._engine.setCursorPosition(pos);
  }
}
