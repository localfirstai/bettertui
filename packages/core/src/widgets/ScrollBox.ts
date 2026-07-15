import type { ScrollBoxOptions } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { ScrollBoxOptions };

export class ScrollBox extends Renderable<ScrollBoxOptions> {
  constructor(options: ScrollBoxOptions = {}) {
    super(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "ScrollBox" }];
    if (this.opts.width) cmds.push({ type: "SetWidth", id, value: this.opts.width as never });
    if (this.opts.height) cmds.push({ type: "SetHeight", id, value: this.opts.height as never });
    return cmds;
  }

  override handleKey(): boolean {
    return false;
  }
}
