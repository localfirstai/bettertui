import type { TextOptions } from "./widget.types";
import type { Command } from "../command/command.types";
import { Renderable } from "../renderable";

export type { TextOptions };

export class Text extends Renderable<TextOptions> {
  private _content: string;

  constructor(content = "", options: TextOptions = {}) {
    super(options);
    this._content = content;
  }

  get content(): string {
    return this._content;
  }

  set content(value: string) {
    this._content = value;
  }

  override update(options: Partial<TextOptions & { content?: string }>): void {
    if (options.content !== undefined) {
      this._content = options.content;
    }
    super.update(options as Partial<TextOptions>);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Text" }];
    if (this._content) {
      cmds.push({ type: "SetText", id, text: this._content });
    }
    cmds.push(...this.styleCommands(id, this.opts as unknown as Record<string, unknown>));
    return cmds;
  }
}
