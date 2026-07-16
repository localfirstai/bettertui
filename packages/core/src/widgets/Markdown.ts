import type { MarkdownOptions } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { MarkdownOptions };

export class Markdown extends Renderable<MarkdownOptions> {
  private _content = "";

  constructor(options: MarkdownOptions = {}) {
    super(options);
    this._content = options.content ?? "";
  }

  get content(): string {
    return this._content;
  }

  set content(value: string) {
    this._content = value;
  }

  override update(options: Partial<MarkdownOptions>): void {
    if (options.content !== undefined) this._content = options.content;
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Markdown" }];
    if (this._content) {
      cmds.push({ type: "SetText", id, text: this._content });
    }
    return cmds;
  }
}
