import type { CodeOptions } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { CodeOptions };

export class Code extends Renderable<CodeOptions> {
  private _content: string;

  constructor(content = "", options: CodeOptions = {}) {
    super(options);
    this._content = content;
  }

  get content(): string {
    return this._content;
  }

  set content(value: string) {
    this._content = value;
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Code" }];
    if (this._content) {
      cmds.push({ type: "SetText", id, text: this._content });
    }
    cmds.push({ type: "SetBackground", id, color: "bright_black" as never });
    return cmds;
  }
}
