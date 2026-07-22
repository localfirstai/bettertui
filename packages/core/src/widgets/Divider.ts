import type { DividerOptions } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { DividerOptions };

export class Divider extends Renderable<DividerOptions> {
  constructor(options: DividerOptions = {}) {
    super(options);
  }

  renderCommands(id: string): Command[] {
    const isVertical = this.opts.orientation === "vertical";
    const char = this.opts.char ?? (isVertical ? "│" : "─");
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];

    if (isVertical) {
      cmds.push({ type: "SetWidth", id, value: 1 });
      cmds.push({ type: "SetHeight", id, value: "100%" as never });
      cmds.push({ type: "SetFlexDirection", id, direction: "column" as never });
    } else {
      cmds.push({ type: "SetWidth", id, value: "100%" as never });
      cmds.push({ type: "SetHeight", id, value: 1 });
      cmds.push({ type: "SetFlexDirection", id, direction: "row" as never });
    }

    if (this.opts.label && !isVertical) {
      // left line
      const leftId = `${id}-left`;
      cmds.push({ type: "CreateNode", id: leftId, kind: "Text" });
      cmds.push({ type: "SetText", id: leftId, text: `${char.repeat(2)} ` });
      if (this.opts.color) {
        cmds.push({ type: "SetForeground", id: leftId, color: this.opts.color as never });
      }
      cmds.push({ type: "AppendChild", parent: id, child: leftId });

      // label
      const labelId = `${id}-label`;
      cmds.push({ type: "CreateNode", id: labelId, kind: "Text" });
      cmds.push({ type: "SetText", id: labelId, text: this.opts.label });
      cmds.push({ type: "AppendChild", parent: id, child: labelId });

      // right line
      const rightId = `${id}-right`;
      cmds.push({ type: "CreateNode", id: rightId, kind: "Text" });
      cmds.push({ type: "SetText", id: rightId, text: ` ${char.repeat(2)}` });
      if (this.opts.color) {
        cmds.push({ type: "SetForeground", id: rightId, color: this.opts.color as never });
      }
      cmds.push({ type: "AppendChild", parent: id, child: rightId });
    } else {
      // Simple line — fill with repeated char via SetText
      const lineId = `${id}-line`;
      cmds.push({ type: "CreateNode", id: lineId, kind: "Text" });
      // Let the engine fill to full width; use a long run that gets clipped
      cmds.push({ type: "SetText", id: lineId, text: char.repeat(200) });
      if (this.opts.color) {
        cmds.push({ type: "SetForeground", id: lineId, color: this.opts.color as never });
      }
      cmds.push({ type: "AppendChild", parent: id, child: lineId });
    }

    return cmds;
  }
}
