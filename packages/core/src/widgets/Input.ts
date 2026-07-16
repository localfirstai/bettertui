import type { InputOptions, KeyEvent } from "@bettertui/shared";
import { generateId } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { InputOptions };

export class Input extends Renderable<InputOptions> {
  private _textId = generateId();
  private _value = "";

  constructor(options: InputOptions = {}) {
    super(options);
    this._value = options.value ?? "";
  }

  get value(): string {
    return this._value;
  }

  override update(options: Partial<InputOptions>): void {
    if (options.value !== undefined) {
      this._value = options.value;
    }
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Input" }];
    const displayText = this.opts.password
      ? "*".repeat(this._value.length)
      : this._value || this.opts.placeholder || "";
    cmds.push({ type: "CreateNode", id: this._textId, kind: "Text" });
    cmds.push({ type: "SetText", id: this._textId, text: displayText });
    cmds.push({ type: "AppendChild", parent: id, child: this._textId });
    if (this.opts.disabled) {
      cmds.push({ type: "SetDim", id, value: true });
    }
    return cmds;
  }

  override handleKey(key: KeyEvent): boolean {
    if (this.opts.disabled) return false;
    if (key.key === "backspace") {
      this._value = this._value.slice(0, -1);
      this.opts.onChange?.(this._value);
      return true;
    }
    if (key.key === "return") {
      this.opts.onSubmit?.(this._value);
      return true;
    }
    if (key.ctrl || key.meta || key.alt) return false;
    if (key.key && key.key.length === 1) {
      this._value += key.key;
      this.opts.onChange?.(this._value);
      return true;
    }
    return false;
  }
}
