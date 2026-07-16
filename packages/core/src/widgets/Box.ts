import type { BoxOptions } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { BoxOptions };

export class Box extends Renderable<BoxOptions> {
  constructor(options: BoxOptions = {}) {
    super(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];
    cmds.push(...this.layoutCommands(id, this.opts as unknown as Record<string, unknown>));
    const style: Record<string, unknown> = {};
    if (this.opts.border) style["border"] = this.opts.border;
    if (this.opts.borderStyle) style["borderStyle"] = this.opts.borderStyle;
    cmds.push(...this.styleCommands(id, style));
    return cmds;
  }
}
