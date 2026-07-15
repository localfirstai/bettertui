import type { DiffOptions } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { DiffOptions };

export class Diff extends Renderable<DiffOptions> {
  private _content = "";

  constructor(options: DiffOptions = {}) {
    super(options);
    this._content = options.content ?? "";
  }

  get content(): string {
    return this._content;
  }

  set content(value: string) {
    this._content = value;
  }

  override update(options: Partial<DiffOptions>): void {
    if (options.content !== undefined) this._content = options.content;
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Diff" }];
    if (this._content) {
      cmds.push({ type: "SetText", id, text: this._content });
    }
    return cmds;
  }
}
