import type { ScrollBarOptions } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { ScrollBarOptions };

export class ScrollBar extends Renderable<ScrollBarOptions> {
  constructor(options: ScrollBarOptions = {}) {
    super(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "ScrollBar" }];
    if (this.opts.orientation === "horizontal") {
      cmds.push({ type: "SetHeight", id, value: 1 });
      cmds.push({ type: "SetWidth", id, value: "100%" as never });
    } else {
      cmds.push({ type: "SetWidth", id, value: 1 });
      cmds.push({ type: "SetHeight", id, value: "100%" as never });
    }
    return cmds;
  }

  override handleKey(): boolean {
    return false;
  }
}
