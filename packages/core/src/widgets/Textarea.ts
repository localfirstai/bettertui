import type { KeyEvent, TextareaOptions } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { TextareaOptions };

export class Textarea extends Renderable<TextareaOptions> {
  private _value = "";

  constructor(options: TextareaOptions = {}) {
    super(options);
    this._value = options.value ?? "";
  }

  get value(): string {
    return this._value;
  }

  override update(options: Partial<TextareaOptions>): void {
    if (options.value !== undefined) {
      this._value = options.value;
    }
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];
    const rows = this.opts.rows ?? 3;
    cmds.push({ type: "SetHeight", id, value: rows });
    const text = this._value || this.opts.placeholder || "";
    cmds.push({ type: "CreateNode", id: `${id}-text`, kind: "Text" });
    cmds.push({ type: "SetText", id: `${id}-text`, text });
    cmds.push({ type: "AppendChild", parent: id, child: `${id}-text` });
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
      this._value += "\n";
      this.opts.onChange?.(this._value);
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
